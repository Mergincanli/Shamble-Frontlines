//! `client_hash`: what both ends of PM-0001-17 hash for one client and one tick (u4-build-plan.md §4.6; the advisor's
//! Q11 ruling). The room's encoder hashes what it intends the client to hold after decoding that tick, with the
//! server's own 32-bit tick and event numbers; the client hashes what it holds, turned back into grid integers through
//! `npt_sim::grid`, with the tick and event numbers as it rebuilt them. One function, so the two agree bit for bit
//! when the client holds what was sent.
//!
//! **What it covers, in this order:** the full 32-bit tick; the client's entity set in entity-id order (kind, then
//! index, then generation), each entity's kind, index, generation and its layout's grid integers
//! (`EntityKind::layout()`); the own-state section's records, the same way; the event section's events as (32-bit
//! event number, kind), in number order. The authority-only streams stay out (plan.md:531), and so do a record's
//! `area` and `raw`, which the wire never carries.
//!
//! **The hash:** FNV-1a, 64 bits, over that canonical little-endian byte string (spike-local, as npt-sim's state
//! hash is). It is never 0: 0 is the client's sentinel for a tick with a group it could not decode, which always
//! mismatches (§4.6), so a computed 0 becomes 1.

use npt_sim::{EntityKind, EntityRecord, SimEvent};

const FNV_OFFSET: u64 = 0xcbf2_9ce4_8422_2325;
const FNV_PRIME: u64 = 0x0000_0100_0000_01b3;

struct Fnv(u64);

impl Fnv {
    fn bytes(&mut self, b: &[u8]) {
        for &x in b {
            self.0 ^= u64::from(x);
            self.0 = self.0.wrapping_mul(FNV_PRIME);
        }
    }
}

fn kind_code(kind: EntityKind) -> u8 {
    EntityKind::ALL.iter().position(|k| *k == kind).unwrap_or(0) as u8
}

fn event_kind(e: &SimEvent) -> u8 {
    match e {
        SimEvent::Shot { .. } => 0,
        SimEvent::Hit { .. } => 1,
        SimEvent::Kill { .. } => 2,
    }
}

fn id(r: &EntityRecord) -> (u8, u16, u16) {
    (kind_code(r.kind), r.index, r.generation)
}

fn records(h: &mut Fnv, list: &[EntityRecord]) {
    h.bytes(&(list.len() as u32).to_le_bytes());
    let mut one = |r: &EntityRecord| {
        let (k, index, generation) = id(r);
        h.bytes(&[k]);
        h.bytes(&index.to_le_bytes());
        h.bytes(&generation.to_le_bytes());
        for g in &r.grid[..r.kind.layout().len()] {
            h.bytes(&g.to_le_bytes());
        }
    };
    if list.is_sorted_by_key(id) {
        list.iter().for_each(&mut one);
    } else {
        let mut sorted: Vec<&EntityRecord> = list.iter().collect();
        sorted.sort_by_key(|r| id(r));
        sorted.into_iter().for_each(one);
    }
}

/// The client-held hash of one client's view at `tick` (the module text): `entities` its entity set, `own` its
/// own-state records, `events` its events as (32-bit event number, event). Either side may pass the records and the
/// events in any order. Never 0.
pub fn client_hash(
    tick: u32,
    entities: &[EntityRecord],
    own: &[EntityRecord],
    events: &[(u32, SimEvent)],
) -> u64 {
    let mut h = Fnv(FNV_OFFSET);
    h.bytes(&tick.to_le_bytes());
    records(&mut h, entities);
    records(&mut h, own);
    h.bytes(&(events.len() as u32).to_le_bytes());
    let key = |e: &(u32, SimEvent)| (e.0, event_kind(&e.1));
    let mut one = |e: &(u32, SimEvent)| {
        let (number, kind) = key(e);
        h.bytes(&number.to_le_bytes());
        h.bytes(&[kind]);
    };
    if events.is_sorted_by_key(key) {
        events.iter().for_each(&mut one);
    } else {
        let mut sorted: Vec<&(u32, SimEvent)> = events.iter().collect();
        sorted.sort_by_key(|e| key(e));
        sorted.into_iter().for_each(one);
    }
    finish(h.0)
}

/// The hash's last step: a computed 0, the client's sentinel (the module text), becomes 1; every other value stays.
fn finish(h: u64) -> u64 {
    if h == 0 { 1 } else { h }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a8_the_finishing_step_turns_0_into_1_and_keeps_every_other_value() {
        // No view's FNV-1a is known to be 0 (a8's 100,000 random views reach it with p about 5e-15), so the guard is
        // fed 0 here.
        assert_eq!(finish(0), 1);
        for h in [1, 2, FNV_OFFSET, u64::MAX] {
            assert_eq!(finish(h), h);
        }
    }
}
