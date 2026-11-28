//! The per-client encoder (u4-build-plan.md §4.4-§4.6; catalogue §3): the 256 kbps window and its remainder after the
//! pump's bytes, groups with index and count that each decode alone against one baseline, the own-state section's
//! filter, the mirror of what the client will hold (§4.6's server side), and the exact wire size of each item, so a
//! group is filled to its limit before it is encoded. Every record goes out through npt-proto's `EntityDelta` and a
//! `Quant` passed in; the field-group layout is npt-proto's `Snapshot::GROUPS` (the encoder builds none of its own).

use npt_proto::{EntityDelta, Limits, Quant};
use npt_sim::{EntityKind, EntityRecord, SimEvent};

use crate::ack::{Frame, find, key};

/// The window: 256 kbps of snapshot datagrams per client (catalogue :88: "about 1,067 B" an interval at 30 Hz),
/// counted as UDP payload plus 28 B each.
pub(crate) const WINDOW_BYTES_PER_SECOND: u64 = 256_000 / 8;
/// What each datagram costs the window beyond its payload (catalogue :88; u4-build-plan.md §4.5).
pub(crate) const DATAGRAM_OVERHEAD: u32 = 28;
/// A snapshot group's bytes before its body: the type byte, then the header and the body's three counts (tick 32,
/// baseline age 16, two sequences 16 each, flags 8, group 8; counts 8, 16 and 8: snapshot.rs's wire text).
pub(crate) const GROUP_HEAD_BYTES: u32 = 1 + (32 + 16 + 16 + 16 + 8 + 8 + 8 + 16 + 8) / 8;
/// An entity's kind on the wire: enough bits for every position in `EntityKind::ALL` (snapshot.rs's `KIND_BITS`).
pub(crate) const KIND_BITS: u32 = usize::BITS - (EntityKind::ALL.len() - 1).leading_zeros();

/// The per-tick budget of tick `k` of a window at `tick_hz` ticks a second: the bytes from second-fraction `k` to
/// `k + 1`, so the fractional remainders carry and one second's budgets sum to exactly 32,000 B at any rate (the F2
/// ruling: the rate is an argument; production callers pass `npt_sim::TICK_HZ`).
pub(crate) fn tick_budget(tick_hz: u32, k: u64) -> u32 {
    let hz = u64::from(tick_hz.max(1));
    let at = |k: u64| (k % hz) * WINDOW_BYTES_PER_SECOND / hz;
    let whole = |k: u64| (k / hz) * WINDOW_BYTES_PER_SECOND;
    ((whole(k + 1) + at(k + 1)) - (whole(k) + at(k))) as u32
}

/// The bits of `kind`'s whole layout.
fn layout_bits(kind: EntityKind) -> u32 {
    kind.layout().iter().map(|f| f.bits()).sum()
}

/// The bits one delta takes on the wire (snapshot.rs's wire text): kind, index 16, operation 2, then an add's
/// generation 16 and every field, an update's mask and each changed field, a removal nothing.
pub(crate) fn delta_bits(d: &EntityDelta) -> u32 {
    KIND_BITS
        + 16
        + 2
        + match d {
            EntityDelta::Add(r) => 16 + layout_bits(r.kind),
            EntityDelta::Update { kind, changed, .. } => {
                let layout = kind.layout();
                layout.len() as u32
                    + layout
                        .iter()
                        .enumerate()
                        .filter(|(i, _)| changed & (1 << i) != 0)
                        .map(|(_, f)| f.bits())
                        .sum::<u32>()
            }
            EntityDelta::Remove { .. } => 0,
        }
}

/// The bits one event takes: number 16, age 8, kind 2, then its fields.
pub(crate) fn event_bits(e: &SimEvent) -> u32 {
    16 + 8
        + 2
        + match e {
            SimEvent::Shot { .. } => 8 + 16,
            SimEvent::Hit { .. } => 8 + 16 + 8 + 16,
            SimEvent::Kill { .. } => 8 + 8,
        }
}

/// A group's bytes for a body of `bits`.
pub(crate) fn group_bytes(bits: u32) -> u32 {
    GROUP_HEAD_BYTES + bits.div_ceil(8)
}

/// One group being filled.
#[derive(Clone, Debug, Default)]
pub(crate) struct Group {
    pub(crate) own: Vec<EntityDelta>,
    pub(crate) entities: Vec<EntityDelta>,
    pub(crate) events: Vec<(u16, u8, SimEvent)>,
    /// Each event's 32-bit number, in `events`' order.
    pub(crate) numbers: Vec<u32>,
    /// The 32-bit numbers of the gameplay events it carries (acknowledged with the group).
    pub(crate) gameplay: Vec<u32>,
    pub(crate) bits: u32,
}

/// Which section an item goes to.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum Section {
    Own,
    Entities,
    Events,
}

/// Fills groups in order, each at most `limit` bytes, within the window's remainder (each group costing its bytes plus
/// 28 B), at most `Limits::GROUPS` of them; the own-state section goes into group 0 alone.
#[derive(Clone, Debug)]
pub(crate) struct Packer {
    pub(crate) groups: Vec<Group>,
    limit: u32,
    left: u32,
}

impl Packer {
    /// Group 0, if the remainder `budget` holds an empty group's bytes and overhead within `limit`.
    pub(crate) fn new(limit: u32, budget: u32) -> Option<Packer> {
        let cost = DATAGRAM_OVERHEAD + group_bytes(0);
        if group_bytes(0) > limit || cost > budget {
            return None;
        }
        Some(Packer {
            groups: vec![Group::default()],
            limit,
            left: budget - cost,
        })
    }

    fn count_ok(g: &Group, section: Section) -> bool {
        match section {
            Section::Own => g.own.len() < usize::from(u8::MAX),
            Section::Entities => g.entities.len() < usize::from(u16::MAX),
            Section::Events => g.events.len() < usize::from(u8::MAX),
        }
    }

    /// Finds room for an item of `bits` in `section`: the last group if it fits there, else a new group (never for
    /// the own-state section); the group's index, or `None` (skipped).
    pub(crate) fn place(&mut self, bits: u32, section: Section) -> Option<usize> {
        let last = self.groups.len() - 1;
        let g = &self.groups[last];
        let now = group_bytes(g.bits);
        let after = group_bytes(g.bits + bits);
        let own_ok = section != Section::Own || last == 0;
        if own_ok && Packer::count_ok(g, section) && after <= self.limit && after - now <= self.left
        {
            self.left -= after - now;
            self.groups[last].bits += bits;
            return Some(last);
        }
        let fresh = group_bytes(bits);
        if section == Section::Own
            || self.groups.len() >= Limits::GROUPS
            || fresh > self.limit
            || DATAGRAM_OVERHEAD + fresh > self.left
        {
            return None;
        }
        self.left -= DATAGRAM_OVERHEAD + fresh;
        self.groups.push(Group {
            bits,
            ..Group::default()
        });
        Some(self.groups.len() - 1)
    }
}

/// Inserts or replaces `r` in a sorted list.
fn upsert(list: &mut Vec<EntityRecord>, r: EntityRecord) {
    match list.binary_search_by_key(&key(&r), key) {
        Ok(i) => list[i] = r,
        Err(i) => list.insert(i, r),
    }
}

/// `list` after `d`, each update read against `base`'s record (every group decodes alone against one baseline).
pub(crate) fn apply(list: &mut Vec<EntityRecord>, base: &[EntityRecord], d: &EntityDelta) {
    match d {
        EntityDelta::Add(r) => {
            if let Ok(Some(r)) = d.apply(None) {
                upsert(list, r);
            } else {
                upsert(list, *r);
            }
        }
        EntityDelta::Update { kind, index, .. } => {
            if let Ok(Some(r)) = d.apply(find(base, (*kind, *index))) {
                upsert(list, r);
            }
        }
        EntityDelta::Remove { kind, index } => {
            if let Ok(i) = list.binary_search_by_key(&(*kind, *index), key) {
                list.remove(i);
            }
        }
    }
}

/// The view a client holds after decoding every group: the baseline frame with each group's deltas.
pub(crate) fn view(base: &Frame, groups: &[Group]) -> Frame {
    let mut f = base.clone();
    for g in groups {
        for d in &g.own {
            apply(&mut f.own, &base.own, d);
        }
        for d in &g.entities {
            apply(&mut f.entities, &base.entities, d);
        }
    }
    f
}

/// The encoder's per-client state: the window's tick count and the `Quant` it codes with (npt-proto's, chosen per
/// instance through `Quant::with_bite` in a bites build: U4E-Q4).
#[derive(Clone, Debug)]
pub struct Encoder {
    tick_hz: u32,
    k: u64,
    pub(crate) quant: Quant,
}

impl Encoder {
    pub(crate) fn new(tick_hz: u32, quant: Quant) -> Encoder {
        Encoder {
            tick_hz,
            k: 0,
            quant,
        }
    }

    /// This tick's budget less the pump's bytes since the last tick (u4-build-plan.md §4.5: "the window counts them,
    /// and the encoder's budget is what remains"); the window moves on one tick.
    pub(crate) fn remainder(&mut self, server_bytes: u32) -> (u32, u32) {
        let budget = tick_budget(self.tick_hz, self.k);
        self.k += 1;
        (budget, budget.saturating_sub(server_bytes))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use npt_proto::{Direction, Message, Snapshot, decode};

    /// e10's window pin (the F2 ruling, advisor :446-447): one second's per-tick budgets sum to 32,000 B at 30 and at
    /// 60 Hz (32,000 / 30 is not a whole number of bytes: the remainders carry).
    #[test]
    fn e10_the_window_sums_to_32_000_b_a_second_at_30_and_60_hz() {
        for hz in [30u32, 60] {
            for second in [0u64, 1, 7] {
                let sum: u64 = (0..u64::from(hz))
                    .map(|i| u64::from(tick_budget(hz, second * u64::from(hz) + i)))
                    .sum();
                assert_eq!(sum, 32_000, "{hz} Hz, second {second}");
            }
        }
        assert_eq!((tick_budget(30, 0), tick_budget(30, 2)), (1_066, 1_067));
        assert_eq!((tick_budget(60, 0), tick_budget(60, 2)), (533, 534));
        assert_eq!(tick_budget(npt_sim::TICK_HZ, 0), tick_budget(30, 0));
    }

    /// The encoder's size arithmetic equals npt-proto's encoding, so a group filled to its limit encodes at it.
    #[test]
    fn the_size_arithmetic_equals_the_encoding_for_every_kind_and_delta_form() {
        let mut own = Vec::new();
        let mut entities = Vec::new();
        let mut bits = 0;
        for (i, kind) in EntityKind::ALL.into_iter().enumerate() {
            let mut r = EntityRecord {
                kind,
                index: i as u16,
                generation: 3,
                area: 0,
                grid: [0; EntityRecord::FIELDS],
                raw: [0; EntityRecord::FIELDS],
            };
            for (g, f) in r.grid.iter_mut().zip(kind.layout()) {
                *g = f.max();
            }
            let mut base = r;
            base.grid[0] = kind.layout()[0].min();
            let add = EntityDelta::Add(r);
            let update = EntityDelta::between(&r, Some(&base)).expect("one field changed");
            let remove = EntityDelta::Remove {
                kind,
                index: i as u16,
            };
            let to = if matches!(kind, EntityKind::Effect | EntityKind::OwnState) {
                &mut own
            } else {
                &mut entities
            };
            for d in [add, update, remove] {
                bits += delta_bits(&d);
                to.push(d);
            }
        }
        let events = vec![
            (1, 0, SimEvent::Shot { slot: 1, seq: 2 }),
            (
                2,
                3,
                SimEvent::Hit {
                    shooter: 1,
                    seq: 2,
                    victim: 3,
                    damage: 4,
                },
            ),
            (
                3,
                255,
                SimEvent::Kill {
                    killer: 1,
                    victim: 2,
                },
            ),
        ];
        bits += events.iter().map(|e| event_bits(&e.2)).sum::<u32>();
        let s = Snapshot {
            tick: 70_000,
            baseline_age: 2,
            own,
            entities,
            events,
            ..Snapshot::default()
        };
        let msg = Message::Snapshot(s);
        let mut out = Vec::new();
        Quant::default().encode(&msg, &mut out).expect("encodes");
        assert_eq!(out.len() as u32, group_bytes(bits));
        // npt-room does not depend on npt-link: the snapshot's channel is read from its row of `Limits::TYPES`.
        let (name, datagram, _, _) = Limits::TYPES[1];
        assert_eq!(name, "Snapshot");
        assert_eq!(decode(Direction::RoomToClient, datagram, &out), Ok(msg));
    }

    #[test]
    fn the_packer_keeps_the_own_section_in_group_0_and_opens_groups_to_the_limit() {
        let mut p = Packer::new(100, 1_000).expect("room for group 0");
        assert_eq!(p.place(8 * 50, Section::Own), Some(0));
        assert_eq!(p.place(8 * 20, Section::Entities), Some(0));
        // 17 + 70 + 40 = 127 > 100: a new group.
        assert_eq!(p.place(8 * 40, Section::Entities), Some(1));
        assert_eq!(
            p.place(8 * 10, Section::Own),
            None,
            "own never leaves group 0"
        );
        // 1,000 - (28 + 17 + 50 + 20) - (28 + 17 + 40) = 800 left; a group of 90 B costs 118: six more, then the cap
        // of 8 groups.
        let mut placed = 0;
        while p.place(8 * 73, Section::Entities).is_some() {
            placed += 1;
        }
        assert_eq!(p.groups.len(), Limits::GROUPS.min(2 + placed));
        assert!(Packer::new(10, 1_000).is_none() && Packer::new(1_000, 44).is_none());
    }
}
