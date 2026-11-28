//! The room-side frame mirror (u4r-brief.md 2.1, 2.4 r4; catalogue §6 item 5): each client's snapshots and full
//! states as the room handed them out (`take_datagrams`, `take_control`), decoded with npt-proto and the `Quant` that
//! client decodes with, and rebuilt into the frame the room meant that client to hold at each tick (its baseline's
//! frame plus the tick's deltas, every group of it, before the link). npt-client exposes no decoded record (R16), so
//! the lab reads what it needs here:
//! - **the client's set** at each tick: whether it holds every other soldier (ES: and every vehicle), PM-0001-2's
//!   `tick.s.everyone_in_view`; whether it holds all of the very large units (every part of each); how many
//!   projectiles (carry 49);
//! - **-57's pairing** (§4.6 :327-328; u4r-brief.md 2.1): per (client, tick, entity), only the fields that tick's
//!   snapshot carried (an `Add`'s whole record, an `Update`'s changed fields), each grid integer's value against the
//!   room's pre-grid value at that tick (`EntityRecord::raw`, carry 9); the scenario keeps a tick's errors only once
//!   that client filed a hash for the tick that is not the sentinel, so only decoded fields count;
//! - **r4's class audit**: every record of the own-state section is the client's own (its soldier, its `OwnState`, its
//!   effect rows); no other record names it; every spatial record lies inside that client's interest radius at that
//!   tick (the room's rule, its radius only: the flood is npt-room's, crate-private); no cosmetic shot of the client's
//!   own reaches it. Snapshots and full states have no team or public section in the spike (catalogue :144-145,
//!   :153-154), so those two rules cannot fire: reported as not exercised.

use std::collections::{BTreeMap, VecDeque};

use npt_link::Channel;
use npt_proto::{Direction, EntityDelta, Message, Quant};
use npt_sim::grid::{Config, Field, PITCH, POSITION_XY, POSITION_Z, YAW};
use npt_sim::{EntityKind, EntityRecord, SimEvent};

/// The room's interest radius, cm: a copy of npt-room's private `RADIUS_CM` (room/src/room.rs:29), which r4's spatial
/// rule checks (the report names the copy).
pub(crate) const INTEREST_RADIUS_CM: i64 = 6_400;
/// A very large unit's parts, at least (PM-0001-2: "4 very large units of at least 20 parts").
pub(crate) const LARGE_UNIT_PARTS: usize = 20;
/// Frames kept per client: the ack ring's ticks and two more (a baseline is at most the ring old). Beyond them each
/// client also keeps the frame of the room's `ClientSlot::last_acked()` (room.rs:238), as the room's own stale copy
/// does: under PM-0001-18/no-fallback the room codes against that tick's frame after it left the ring (ack.rs `stale`,
/// taken only while its tick is `last_acked`), at ages 33 and more (the advisor's U4r build ruling 3, point 8 (A)).
const FRAMES_KEPT: usize = npt_sim::ACK_RING_TICKS as usize + 2;

type Frame = BTreeMap<(EntityKind, u16), EntityRecord>;

/// One (client, tick)'s -57 errors: positions in µm, yaw and pitch in µdeg, one per carried field group.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub(crate) struct Errors {
    pub(crate) pos_um: Vec<i64>,
    pub(crate) yaw_microdeg: Vec<i64>,
    pub(crate) pitch_microdeg: Vec<i64>,
}

/// What one client's set held at one tick (from the frame the room meant it to hold).
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub(crate) struct SetStats {
    /// Every other soldier (ES: and every vehicle) in the set.
    pub(crate) everyone: bool,
    /// Every very large unit's hull and parts in the set.
    pub(crate) large_units: bool,
    pub(crate) projectiles: u32,
}

/// r4's counts: records audited per class, and the first breaches (the advisor's U4r build ruling, point 2).
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub(crate) struct Audit {
    pub(crate) owner: u64,
    /// Entity-section records checked against the room's radius (soldiers, hulls, parts, projectiles).
    pub(crate) spatial: u64,
    /// Entity-section records not checked against it: doors, the wall, and any record that tick's room records lack.
    pub(crate) spatial_unchecked: u64,
    pub(crate) public: u64,
    pub(crate) events: u64,
    /// The first 16 breaches' texts, and every breach counted.
    pub(crate) breaches: Vec<String>,
    pub(crate) breaches_count: u64,
}

#[derive(Clone, Debug, Default)]
struct ClientMirror {
    frames: VecDeque<(u32, Frame)>,
    latest: SetStats,
}

impl ClientMirror {
    fn frame(&self, tick: u32) -> Option<&Frame> {
        self.frames.iter().find(|(t, _)| *t == tick).map(|(_, f)| f)
    }

    /// Keeps `frame` as the room's frame of `tick`: the newest [`FRAMES_KEPT`] frames, and beyond them the frame of
    /// `pin`, the room's `last_acked()` for this client (the constant's text). In a clean run the baseline is always
    /// inside the ring, so the frames looked up are the same as with no pin.
    fn keep(&mut self, tick: u32, frame: Frame, pin: Option<u32>) {
        self.frames.retain(|(t, _)| *t != tick);
        self.frames.push_back((tick, frame));
        let unpinned = |t: u32| Some(t) != pin;
        while self.frames.iter().filter(|(t, _)| unpinned(*t)).count() > FRAMES_KEPT {
            let Some(i) = self.frames.iter().position(|(t, _)| unpinned(*t)) else {
                break;
            };
            self.frames.remove(i);
        }
    }
}

/// The room-side records of one tick, indexed for the mirror.
pub(crate) struct Records<'a> {
    by_key: BTreeMap<(EntityKind, u16), &'a EntityRecord>,
    /// Each very large unit's hull index and its part indices.
    large: Vec<(u16, Vec<u16>)>,
    /// Soldier slots and vehicle hulls, for the everyone-in-view flag.
    soldiers: Vec<u16>,
    vehicles: Vec<u16>,
}

impl<'a> Records<'a> {
    pub(crate) fn new(records: &'a [EntityRecord]) -> Records<'a> {
        let mut by_key = BTreeMap::new();
        let mut parts: BTreeMap<u16, Vec<u16>> = BTreeMap::new();
        let mut soldiers = Vec::new();
        let mut vehicles = Vec::new();
        for r in records {
            by_key.insert((r.kind, r.index), r);
            match r.kind {
                EntityKind::Soldier => soldiers.push(r.index),
                EntityKind::Vehicle => vehicles.push(r.index),
                EntityKind::VehiclePart => {
                    if let Ok(hull) = u16::try_from(r.grid[5]) {
                        parts.entry(hull).or_default().push(r.index);
                    }
                }
                _ => {}
            }
        }
        let large = parts
            .into_iter()
            .filter(|(_, p)| p.len() >= LARGE_UNIT_PARTS)
            .collect();
        Records {
            by_key,
            large,
            soldiers,
            vehicles,
        }
    }

    /// The very large units (hulls of at least 20 parts) this tick.
    pub(crate) fn large_units(&self) -> usize {
        self.large.len()
    }

    fn get(&self, kind: EntityKind, index: u16) -> Option<&EntityRecord> {
        self.by_key.get(&(kind, index)).copied()
    }
}

/// The value grid integer `k` of `field` stands for (the wire grid's, as the client holds it).
fn value(field: Field, k: i32) -> f64 {
    f64::from(Config::default().from_grid(field, k))
}

/// The room's pre-grid value of field `i` of `r` (`raw` holds it by its f32 bits).
fn raw(r: &EntityRecord, i: usize) -> f64 {
    f64::from(f32::from_bits(r.raw[i]))
}

/// An angle's difference brought into [-π, π].
fn turn(d: f64) -> f64 {
    let t = std::f64::consts::TAU;
    let d = d.rem_euclid(t);
    if d > t / 2.0 { d - t } else { d }
}

/// -57's errors for one carried record `sent` (the grid integers the client decodes), `changed` its carried fields'
/// mask, against the room's record `room` of the same entity at that tick.
fn errors(sent: &EntityRecord, changed: u16, room: &EntityRecord, out: &mut Errors) {
    let layout = sent.kind.layout();
    let carried = |i: usize| i < layout.len() && changed & (1 << i) != 0;
    let mut pos2 = 0.0f64;
    let mut any_pos = false;
    for (i, f) in layout.iter().enumerate() {
        if !carried(i) {
            continue;
        }
        if *f == POSITION_XY || *f == POSITION_Z {
            let e = value(*f, sent.grid[i]) - raw(room, i);
            pos2 += e * e;
            any_pos = true;
        } else if *f == YAW || *f == PITCH {
            let e = turn(value(*f, sent.grid[i]) - raw(room, i)).abs();
            let microdeg = (e.to_degrees() * 1e6).ceil() as i64;
            if *f == YAW {
                out.yaw_microdeg.push(microdeg);
            } else {
                out.pitch_microdeg.push(microdeg);
            }
        }
    }
    if any_pos {
        out.pos_um.push((pos2.sqrt() * 1e6).ceil() as i64);
    }
}

/// Every client's mirror (the module text).
pub(crate) struct Mirror {
    clients: Vec<ClientMirror>,
    slots: u8,
    effect_rows: u8,
    /// ES's flag also needs every vehicle (PM-0001-2's ES row, carry 59).
    with_vehicles: bool,
    pub(crate) audit: Audit,
}

impl Mirror {
    /// The mirror of `slots` clients of a scene with `effect_rows` per soldier; `with_vehicles` for ES.
    pub(crate) fn new(slots: u8, effect_rows: u8, with_vehicles: bool) -> Mirror {
        Mirror {
            clients: vec![ClientMirror::default(); usize::from(slots)],
            slots,
            effect_rows,
            with_vehicles,
            audit: Audit::default(),
        }
    }

    /// The latest set stats of `slot` (from its newest frame at or before the last tick seen).
    pub(crate) fn latest(&self, slot: u8) -> SetStats {
        self.clients
            .get(usize::from(slot))
            .map(|c| c.latest)
            .unwrap_or_default()
    }

    /// The soldiers `slot`'s frame of `tick` holds, with their positions (the bots' view: npt-bots' `Behaviour`).
    pub(crate) fn view(&self, slot: u8, tick: u32, into: &mut Vec<(u8, [i32; 3])>) {
        into.clear();
        let Some(f) = self
            .clients
            .get(usize::from(slot))
            .and_then(|c| c.frame(tick))
        else {
            return;
        };
        for ((kind, index), r) in f {
            if *kind == EntityKind::Soldier
                && let Ok(s) = u8::try_from(*index)
            {
                into.push((s, [r.grid[0], r.grid[1], r.grid[2]]));
            }
        }
    }

    fn own(&self, slot: u8, r: &EntityRecord) -> bool {
        let rows = u16::from(self.effect_rows.max(1));
        match r.kind {
            EntityKind::Soldier | EntityKind::OwnState => r.index == u16::from(slot),
            EntityKind::Effect => self.effect_rows > 0 && r.index / rows == u16::from(slot),
            _ => false,
        }
    }

    fn breach(&mut self, text: String) {
        self.audit.breaches_count += 1;
        if self.audit.breaches.len() < 16 {
            self.audit.breaches.push(text);
        }
    }

    /// The own-state section's rule (owner: only the client's own records) and the entity section's (no record of
    /// the client's own; every spatial record inside the interest radius of the client's soldier at that tick).
    fn audit_records(
        &mut self,
        slot: u8,
        tick: u32,
        own: &[EntityRecord],
        entities: &[EntityRecord],
        room: &Records<'_>,
    ) {
        for r in own {
            self.audit.owner += 1;
            if !self.own(slot, r) {
                self.breach(format!(
                    "tick {tick}: client {slot}'s own-state section holds {:?} {} (owner: only its own slot's)",
                    r.kind, r.index
                ));
            }
        }
        let me = room.get(EntityKind::Soldier, u16::from(slot));
        for r in entities {
            if self.own(slot, r) || matches!(r.kind, EntityKind::OwnState | EntityKind::Effect) {
                self.audit.spatial_unchecked += 1;
                self.breach(format!(
                    "tick {tick}: client {slot}'s entity section holds {:?} {} (an owner-only record)",
                    r.kind, r.index
                ));
                continue;
            }
            let at = match r.kind {
                EntityKind::Soldier | EntityKind::Vehicle => {
                    room.get(r.kind, r.index).map(|x| [x.grid[0], x.grid[1]])
                }
                EntityKind::Projectile => room.get(r.kind, r.index).map(|x| [x.grid[1], x.grid[2]]),
                EntityKind::VehiclePart => room
                    .get(r.kind, r.index)
                    .and_then(|p| u16::try_from(p.grid[5]).ok())
                    .and_then(|h| room.get(EntityKind::Vehicle, h))
                    .map(|x| [x.grid[0], x.grid[1]]),
                _ => None,
            };
            let (Some(me), Some(at)) = (me, at) else {
                // Doors, the wall, or a record this tick's room records lack: no radius test.
                self.audit.spatial_unchecked += 1;
                continue;
            };
            self.audit.spatial += 1;
            {
                let (dx, dy) = (
                    i64::from(at[0]) - i64::from(me.grid[0]),
                    i64::from(at[1]) - i64::from(me.grid[1]),
                );
                if dx * dx + dy * dy > INTEREST_RADIUS_CM * INTEREST_RADIUS_CM {
                    self.breach(format!(
                        "tick {tick}: client {slot}'s entity section holds {:?} {} beyond the interest radius",
                        r.kind, r.index
                    ));
                }
            }
        }
    }

    fn stats(&self, frame: &Frame, room: &Records<'_>, slot: u8) -> SetStats {
        let has = |k: EntityKind, i: u16| frame.contains_key(&(k, i));
        let others = room
            .soldiers
            .iter()
            .filter(|s| **s != u16::from(slot))
            .all(|s| has(EntityKind::Soldier, *s));
        let vehicles =
            !self.with_vehicles || room.vehicles.iter().all(|v| has(EntityKind::Vehicle, *v));
        let large = !room.large.is_empty()
            && room.large.iter().all(|(h, parts)| {
                has(EntityKind::Vehicle, *h)
                    && parts.iter().all(|p| has(EntityKind::VehiclePart, *p))
            });
        let projectiles = frame
            .keys()
            .filter(|(k, _)| *k == EntityKind::Projectile)
            .count() as u32;
        SetStats {
            everyone: others && vehicles,
            large_units: large,
            projectiles,
        }
    }

    /// One tick's messages for `slot` (the room's datagrams and reliable messages, as it handed them out), decoded with
    /// `quant`: the frame rebuilt, the audit run, and -57's errors of what the snapshot carried returned. `last_acked`
    /// is the room's `ClientSlot::last_acked()` for `slot` after the tick, whose frame is kept beyond the ring's
    /// ([`FRAMES_KEPT`]'s text).
    #[allow(
        clippy::too_many_arguments,
        reason = "the room's tick output for one client, and the one room value the frames' keep needs"
    )]
    pub(crate) fn tick(
        &mut self,
        slot: u8,
        tick: u32,
        datagrams: &[Vec<u8>],
        control: &[Vec<u8>],
        quant: Quant,
        room: &Records<'_>,
        last_acked: Option<u32>,
    ) -> Errors {
        let mut errs = Errors::default();
        if slot >= self.slots {
            return errs;
        }
        for b in control {
            let Ok(channel) = crate::scenario::stream_of(b, quant) else {
                self.breach(format!(
                    "tick {tick}: client {slot}: a reliable message on no known channel"
                ));
                continue;
            };
            let Ok(Message::FullState(f)) = quant.decode(Direction::RoomToClient, channel, b)
            else {
                continue;
            };
            self.audit.public += 1;
            self.audit_records(slot, f.tick, &f.own, &f.entities, room);
            let frame: Frame = f
                .entities
                .iter()
                .chain(&f.own)
                .map(|r| ((r.kind, r.index), *r))
                .collect();
            if f.tick == tick {
                for r in f.entities.iter().chain(&f.own) {
                    if let Some(x) = room.get(r.kind, r.index) {
                        errors(r, u16::MAX, x, &mut errs);
                    }
                }
            }
            let stats = self.stats(&frame, room, slot);
            let c = &mut self.clients[usize::from(slot)];
            c.latest = stats;
            // Kept under the room's tick: the room builds a full state at the tick it takes it (the wire's tick is
            // the same in a clean run, and a bitten header's under tick16).
            c.keep(tick, frame, last_acked);
        }
        let mut frame: Option<Frame> = None;
        for d in datagrams {
            let Ok(Message::Snapshot(s)) =
                quant.decode(Direction::RoomToClient, Channel::Datagram, d)
            else {
                self.breach(format!(
                    "tick {tick}: client {slot}: a datagram the mirror cannot decode"
                ));
                continue;
            };
            self.audit.public += 1;
            if frame.is_none() {
                let c = &self.clients[usize::from(slot)];
                // The baseline by the room's tick, never the wire's: the datagrams are the ones the room took at
                // `tick` (equal in a clean run; under PM-0001-17/tick16 the wire's is 16 bits, carry 73).
                frame = if s.baseline_age == 0 {
                    Some(Frame::new())
                } else {
                    c.frame(tick.wrapping_sub(u32::from(s.baseline_age)))
                        .cloned()
                };
                if frame.is_none() {
                    self.breach(format!(
                        "tick {tick}: client {slot}: its baseline frame (age {}) is not mirrored",
                        s.baseline_age
                    ));
                    return errs;
                }
            }
            let f = frame.as_mut().expect("set above");
            let mut own = Vec::new();
            let mut entities = Vec::new();
            for (section, deltas) in [(true, &s.own), (false, &s.entities)] {
                for delta in deltas {
                    let (kind, index, changed) = match *delta {
                        EntityDelta::Add(r) => (r.kind, r.index, u16::MAX),
                        EntityDelta::Update {
                            kind,
                            index,
                            changed,
                            ..
                        } => (kind, index, changed),
                        EntityDelta::Remove { kind, index } => {
                            f.remove(&(kind, index));
                            continue;
                        }
                    };
                    // Both sections are coded against the one baseline frame, which holds both.
                    let base = f.get(&(kind, index)).copied();
                    let Ok(Some(rebuilt)) = delta.apply(base.as_ref()) else {
                        self.breach(format!(
                            "tick {tick}: client {slot}: {kind:?} {index}'s update has no baseline record"
                        ));
                        continue;
                    };
                    if let Some(x) = room.get(kind, index) {
                        errors(&rebuilt, changed, x, &mut errs);
                    }
                    f.insert((kind, index), rebuilt);
                    if section {
                        own.push(rebuilt);
                    } else {
                        entities.push(rebuilt);
                    }
                }
            }
            for (_, _, e) in &s.events {
                self.audit.events += 1;
                if let SimEvent::Shot { slot: shooter, .. } = e
                    && *shooter == slot
                {
                    self.breach(format!(
                        "tick {tick}: client {slot} was sent its own shot (cosmetic)"
                    ));
                }
            }
            self.audit_records(slot, tick, &own, &entities, room);
        }
        if let Some(f) = frame {
            let stats = self.stats(&f, room, slot);
            let c = &mut self.clients[usize::from(slot)];
            c.latest = stats;
            c.keep(tick, f, last_acked);
        }
        errs
    }
}
