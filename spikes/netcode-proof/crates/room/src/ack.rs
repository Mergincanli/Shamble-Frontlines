//! The AckRing (u4-build-plan.md §4.5; plan §4.7): per client, every snapshot sent in the last
//! `npt_sim::ACK_RING_TICKS` ticks with what its groups acknowledged, and the client's view after each, the baseline a
//! later snapshot is coded against.

use npt_proto::Wrap;
use npt_sim::{EntityKind, EntityRecord};

/// A client's view at one tick as the room means it to hold it after decoding that tick: the entity set and the
/// own-state section, each sorted by (kind, index), every record as the wire carries it (`area` and `raw` 0).
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub(crate) struct Frame {
    pub(crate) entities: Vec<EntityRecord>,
    pub(crate) own: Vec<EntityRecord>,
}

/// The sort and lookup key of a frame's records.
pub(crate) fn key(r: &EntityRecord) -> (EntityKind, u16) {
    (r.kind, r.index)
}

/// The record of (kind, index) in a sorted list.
pub(crate) fn find(list: &[EntityRecord], k: (EntityKind, u16)) -> Option<&EntityRecord> {
    list.binary_search_by_key(&k, key).ok().map(|i| &list[i])
}

/// One sent tick: its group count, the groups acknowledged, the view it gives, and each group's gameplay event numbers.
#[derive(Clone, Debug, Default)]
pub(crate) struct Sent {
    pub(crate) tick: u32,
    pub(crate) valid: bool,
    pub(crate) groups: u8,
    pub(crate) acked: u8,
    pub(crate) frame: Frame,
    /// (group, event number) for each gameplay event the tick carried.
    pub(crate) events: Vec<(u8, u32)>,
}

impl Sent {
    fn full_mask(&self) -> u8 {
        ((1u16 << self.groups) - 1) as u8
    }

    fn complete(&self) -> bool {
        self.valid && self.groups > 0 && self.acked == self.full_mask()
    }
}

// The group masks (`Sent::acked`, an acknowledgement's, `full_mask`) are a u8: they rely on at most 8 groups a tick
// (RR-3; the wire's 3-bit group).
const _: () = assert!(
    npt_proto::Limits::GROUPS <= 8,
    "a u8 group mask holds at most 8 groups"
);

/// What an acknowledgement named (the room's check "the acknowledgement names a sent snapshot": catalogue :87).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum Acked {
    /// A snapshot (or full state) the ring holds; its groups were marked.
    Held,
    /// A tick inside the span the room has sent, older than the ring: sent, and too old to be a baseline.
    Old,
    /// No snapshot sent to this client has that tick.
    NeverSent,
}

/// Per client, the ticks sent in the last `npt_sim::ACK_RING_TICKS` ticks (u4-build-plan.md §4.5): ages 0 to 31 kept
/// at 30 Hz, age 32 evicted (age = the tick being coded less the sent tick). Baseline B is the newest sent tick whose
/// every group the client acknowledged; a partly acknowledged tick is never a baseline, and a B older than the ring
/// means coding against the per-type defaults.
#[derive(Clone, Debug)]
pub struct AckRing {
    entries: Vec<Sent>,
    newest: Option<u32>,
    first: Option<u32>,
    last_acked: Option<u32>,
    /// Under PM-0001-18/no-fallback (bites builds only): the last acknowledged view, kept when its entry is evicted.
    keep_stale: bool,
    stale: Option<Sent>,
}

impl AckRing {
    /// The ring's length in ticks, `npt_sim::ACK_RING_TICKS` by path (32 at 30 Hz).
    pub(crate) fn len() -> u32 {
        npt_sim::ACK_RING_TICKS
    }

    pub(crate) fn new() -> AckRing {
        AckRing {
            entries: vec![Sent::default(); AckRing::len() as usize],
            newest: None,
            first: None,
            last_acked: None,
            keep_stale: false,
            stale: None,
        }
    }

    /// Keep the last acknowledged view past its eviction (the -18 bite's encoder codes against it).
    pub(crate) fn keep_stale(&mut self) {
        self.keep_stale = true;
    }

    fn slot(tick: u32) -> usize {
        (tick % AckRing::len()) as usize
    }

    /// Records a sent tick (a snapshot's groups, or a full state as one group): its group count, the view it gives
    /// and its gameplay events.
    pub(crate) fn record(&mut self, tick: u32, groups: u8, frame: Frame, events: Vec<(u8, u32)>) {
        let i = AckRing::slot(tick);
        let old = &mut self.entries[i];
        if self.keep_stale && old.valid && Some(old.tick) == self.last_acked {
            self.stale = Some(core::mem::take(old));
        }
        self.entries[i] = Sent {
            tick,
            valid: true,
            groups,
            acked: 0,
            frame,
            events,
        };
        let w = Wrap::default();
        if self.newest.is_none_or(|n| w.after32("tick", n, tick)) {
            self.newest = Some(tick);
        }
        if self.first.is_none() {
            self.first = Some(tick);
        }
    }

    /// The newest tick sent, if any.
    pub(crate) fn newest(&self) -> Option<u32> {
        self.newest
    }

    /// The entry the ring holds for `tick`, if any.
    fn held(&self, tick: u32) -> Option<&Sent> {
        let e = &self.entries[AckRing::slot(tick)];
        let newest = self.newest?;
        (e.valid && e.tick == tick && newest.wrapping_sub(tick) < AckRing::len()).then_some(e)
    }

    /// Marks `mask`'s groups of `tick` acknowledged, pushing the gameplay event numbers they carried onto `events`.
    pub(crate) fn ack(&mut self, tick: u32, mask: u8, events: &mut Vec<u32>) -> Acked {
        let w = Wrap::default();
        if self.held(tick).is_some() {
            let e = &mut self.entries[AckRing::slot(tick)];
            let new = mask & e.full_mask() & !e.acked;
            e.acked |= new;
            for &(g, n) in &e.events {
                if new & (1 << g) != 0 {
                    events.push(n);
                }
            }
            if e.complete() && self.last_acked.is_none_or(|l| w.after32("tick", l, tick)) {
                self.last_acked = Some(tick);
            }
            return Acked::Held;
        }
        match (self.first, self.newest) {
            (Some(f), Some(n)) if !w.after32("tick", tick, f) && !w.after32("tick", n, tick) => {
                // Inside the sent span: older than the ring (a tick in the ring's span it lacks was never sent).
                if n.wrapping_sub(tick) >= AckRing::len() {
                    Acked::Old
                } else {
                    Acked::NeverSent
                }
            }
            _ => Acked::NeverSent,
        }
    }

    /// Whether the ring holds `tick` with every group acknowledged.
    pub(crate) fn acked(&self, tick: u32) -> bool {
        self.held(tick).is_some_and(Sent::complete)
    }

    /// Baseline B for coding tick `now`: the newest sent tick whose every group was acknowledged, at age 1 to
    /// `len() - 1` (age `len()` is evicted). `None`: code against the per-type defaults.
    pub(crate) fn baseline(&self, now: u32) -> Option<&Sent> {
        let mut best: Option<&Sent> = None;
        for e in &self.entries {
            let age = now.wrapping_sub(e.tick);
            if !e.complete() || age == 0 || age >= AckRing::len() {
                continue;
            }
            if best.is_none_or(|b| now.wrapping_sub(b.tick) > age) {
                best = Some(e);
            }
        }
        best
    }

    /// The newest tick ever fully acknowledged (the last baseline the client confirmed), held or evicted.
    pub(crate) fn last_acked(&self) -> Option<u32> {
        self.last_acked
    }

    /// Whether the last acknowledged baseline has left the ring at tick `now` (u4-build-plan.md §4.5's
    /// `event.burst_eviction`, read at the first tick after a burst).
    pub(crate) fn evicted(&self, now: u32) -> bool {
        self.last_acked
            .is_some_and(|l| now.wrapping_sub(l) >= AckRing::len())
    }

    /// The last acknowledged view past its eviction (kept only under the -18 bite).
    pub(crate) fn stale(&self) -> Option<&Sent> {
        self.stale
            .as_ref()
            .filter(|s| Some(s.tick) == self.last_acked)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sent(ring: &mut AckRing, tick: u32, groups: u8) {
        ring.record(tick, groups, Frame::default(), vec![(0, tick)]);
    }

    /// e4: the baseline rule and the ring's age edges (T15; u4-build-plan.md §4.5): age 31 kept, age 32 evicted; a
    /// partly acknowledged tick is never a baseline.
    #[test]
    fn e4_the_baseline_is_the_newest_fully_acked_tick_age_31_kept_age_32_evicted() {
        let len = AckRing::len();
        assert_eq!(len, npt_sim::ack_ring_ticks_at(npt_sim::TICK_HZ));
        let start = 1_000u32;
        let mut ring = AckRing::new();
        let mut events = Vec::new();
        sent(&mut ring, start, 2);
        // Group 0 alone: partly acknowledged, never a baseline.
        assert_eq!(ring.ack(start, 0b01, &mut events), Acked::Held);
        assert!(ring.baseline(start + 1).is_none());
        assert_eq!(ring.ack(start, 0b10, &mut events), Acked::Held);
        assert_eq!(ring.baseline(start + 1).map(|s| s.tick), Some(start));
        // Age 31 kept, age 32 evicted.
        assert_eq!(ring.baseline(start + len - 1).map(|s| s.tick), Some(start));
        assert!(!ring.evicted(start + len - 1));
        assert!(ring.baseline(start + len).is_none());
        assert!(ring.evicted(start + len));
        // A newer partly acknowledged tick never displaces the older complete one.
        sent(&mut ring, start + 3, 3);
        ring.ack(start + 3, 0b011, &mut events);
        assert_eq!(ring.baseline(start + 4).map(|s| s.tick), Some(start));
        ring.ack(start + 3, 0b100, &mut events);
        assert_eq!(ring.baseline(start + 4).map(|s| s.tick), Some(start + 3));
        // Each group's events come back once, when that group is first acknowledged.
        assert_eq!(events, [start, start + 3]);
    }

    #[test]
    fn an_ack_names_a_held_tick_an_old_sent_one_or_one_never_sent() {
        let len = AckRing::len();
        let mut ring = AckRing::new();
        let mut events = Vec::new();
        assert_eq!(ring.ack(5, 1, &mut events), Acked::NeverSent);
        for t in 100..100 + 2 * len {
            if t != 100 + 2 * len - 3 {
                sent(&mut ring, t, 1);
            }
        }
        let newest = 100 + 2 * len - 1;
        assert_eq!(ring.ack(newest, 1, &mut events), Acked::Held);
        assert_eq!(
            ring.ack(newest - 2, 1, &mut events),
            Acked::NeverSent,
            "a skipped tick in the ring"
        );
        assert_eq!(ring.ack(100, 1, &mut events), Acked::Old);
        assert_eq!(
            ring.ack(99, 1, &mut events),
            Acked::NeverSent,
            "before the first"
        );
        assert_eq!(
            ring.ack(newest + 1, 1, &mut events),
            Acked::NeverSent,
            "ahead of the newest"
        );
    }

    #[test]
    fn the_ring_works_across_the_32_bit_tick_wrap() {
        let mut ring = AckRing::new();
        let mut events = Vec::new();
        let start = u32::MAX - 2;
        for k in 0..6u32 {
            sent(&mut ring, start.wrapping_add(k), 1);
        }
        assert_eq!(ring.ack(start.wrapping_add(4), 1, &mut events), Acked::Held);
        assert_eq!(
            ring.baseline(start.wrapping_add(6)).map(|s| s.tick),
            Some(start.wrapping_add(4))
        );
        assert_eq!(ring.newest(), Some(start.wrapping_add(5)));
    }
}
