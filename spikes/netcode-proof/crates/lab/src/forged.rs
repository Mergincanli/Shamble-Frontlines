//! -21's attribution (u4f-brief.md 2.4 item 6; TE3-2, SA-4): each forger's items, in 2.4's unit, counted rejected only
//! by its kind's rule, against every tick's records and `TickInput` (`event.forged_kind`, one record per kind at the
//! run's end, SERIES.md:158-159):
//! - kinds 1, 2, 4, 5, 6 and 10: a room record of that kind names (slot, seq), and no command of (slot, seq) runs;
//! - kind 3: a room record of kind 3 names (slot, seq), the forged copy's content never runs, and (slot, seq) runs
//!   once (its honest original);
//! - kind 7: a sim record (slot, seq, Cooldown or Reload) names it (the command itself runs); an item is left out of
//!   `sent_count` and `unrejected_count` when its soldier was dead, by the server's own World before the command phase
//!   of the tick that ran its command or the one before it (`npt_bots::kind_7_left_out`; FP-2, F-B-COUNT (a), Mustafa
//!   2026-11-21), and those items are reported beside the check (`dead_left_out_count`, 0 for every other kind);
//! - kinds 8, 9 and 11: by count on the forger's session (the room's `ack_refused` datagrams; the pump's trailing-bytes
//!   and unknown-field drops; its wrong-channel drops), read at the cutoff.
//!
//! The room's kinds and the sim's stay apart (C-U4F2-10): a kind-5 item with only a sim Cooldown record is unrejected.
//! A record naming a sequence the forger did not forge changes nothing (U4F2-Q4). Only items forged before the cutoff
//! count, the measured end less the room's lead bound in ticks, so every one's verdict falls inside the run; the room's
//! dropped records are logged beside (C-U4F2-SEC-1), and kinds 8, 9 and 11's counter beside their count (FP-8,
//! SA-U4F2-3). Items are keyed by their 16-bit sequence, so a run is judged within one sequence lap (65,536 commands a
//! forger, about 36 minutes at 30 Hz): a sequence forged again counts unrejected (FP-8, RR-U4F2-2), so a longer run
//! fails, never passes. Memory: one entry per forged sequence, at most 65,536 a forger.

use std::collections::{BTreeMap, BTreeSet, VecDeque};

use npt_log::{EventRecord, Phase};
use npt_room::RefusalRecord;
use npt_sim::{Command, RefusalKind, Rewind};

/// One forger's tally.
#[derive(Clone, Debug, Default)]
pub(crate) struct Forger {
    slot: u8,
    kind: u8,
    /// Each forged item by sequence: kind 3's copy as sent.
    items: BTreeMap<u16, Option<Command>>,
    measured: u64,
    /// The items with a record of the forger's own kind (room kinds by number; kind 7 the sim's Cooldown or Reload).
    refused: BTreeSet<u16>,
    /// How many times each forged sequence ran, and those whose run carried a kind-3 copy's content.
    ran: BTreeMap<u16, u32>,
    copy_ran: BTreeSet<u16>,
    /// Kinds 8, 9 and 11: their counter at the cutoff.
    counted: u64,
    /// Sequences forged again (a second lap), each counted unrejected.
    reused: u64,
    /// Kind 7 (FP-2): its last runs' sequences, each with whether its soldier was dead before that tick's command
    /// phase (the first run of each), and its items left out by the dead window.
    recent: VecDeque<(u16, bool)>,
    left_out: BTreeSet<u16>,
}

/// Kind 7's first runs kept for the dead window: the item's command is the next first run after the one before it
/// (the forger's sequences run in order), so the ring needs only the latest few; a property of run order, not of time.
const RECENT_RUNS: usize = 4;

/// Every forger's tally of one run.
#[derive(Clone, Debug, Default)]
pub(crate) struct Attribution {
    forgers: Vec<Forger>,
}

impl Attribution {
    /// The tallies for `forgers`, each (slot, kind).
    pub(crate) fn new(forgers: impl IntoIterator<Item = (u8, u8)>) -> Attribution {
        let forgers = forgers
            .into_iter()
            .map(|(slot, kind)| Forger {
                slot,
                kind,
                ..Forger::default()
            })
            .collect();
        Attribution { forgers }
    }

    fn of(&mut self, slot: u8) -> Option<&mut Forger> {
        self.forgers.iter_mut().find(|f| f.slot == slot)
    }

    /// `slot`'s items forged at one client step (`seqs`; kind 3's `copies` beside them), in the measured phase or not.
    pub(crate) fn forged(&mut self, slot: u8, seqs: &[u16], copies: &[Command], measured: bool) {
        let Some(f) = self.of(slot) else { return };
        for (i, &seq) in seqs.iter().enumerate() {
            if f.items.insert(seq, copies.get(i).copied()).is_some() {
                f.reused += 1;
            }
            f.measured += u64::from(measured);
        }
    }

    /// One room tick: the room's records of the receives before it, the sim's records of its step, the commands it ran
    /// (each with its slot set), and the slots whose soldier was dead before its command phase (the server's World
    /// after the tick before: FP-2).
    pub(crate) fn tick(
        &mut self,
        room: &[RefusalRecord],
        sim: &[npt_sim::Refusal],
        ran: &[(Command, Option<Rewind>)],
        dead: &[u8],
    ) {
        for f in &mut self.forgers {
            let is_dead = dead.contains(&f.slot);
            for (c, _) in ran.iter().filter(|(c, _)| f.kind == 7 && c.slot == f.slot) {
                if f.recent.iter().any(|r| r.0 == c.seq) {
                    continue;
                }
                let before = f.recent.iter().find(|r| r.0 == c.seq.wrapping_sub(1));
                let out = npt_bots::kind_7_left_out(is_dead, before.map(|r| r.1));
                if out && f.items.contains_key(&c.seq) {
                    f.left_out.insert(c.seq);
                }
                f.recent.push_back((c.seq, is_dead));
                if f.recent.len() > RECENT_RUNS {
                    f.recent.pop_front();
                }
            }
            for r in room.iter().filter(|r| r.slot == f.slot) {
                if f.kind != 7 && r.kind.kind() == f.kind && f.items.contains_key(&r.seq) {
                    f.refused.insert(r.seq);
                }
            }
            for r in sim.iter().filter(|r| r.slot == f.slot) {
                let own = matches!(r.kind, RefusalKind::Cooldown | RefusalKind::Reload);
                if f.kind == 7 && own && f.items.contains_key(&r.seq) {
                    f.refused.insert(r.seq);
                }
            }
            for (c, _) in ran.iter().filter(|(c, _)| c.slot == f.slot) {
                let Some(copy) = f.items.get(&c.seq) else {
                    continue;
                };
                *f.ran.entry(c.seq).or_default() += 1;
                if copy.is_some_and(|x| Command { slot: c.slot, ..x } == *c) {
                    f.copy_ran.insert(c.seq);
                }
            }
        }
    }

    /// Kinds 8, 9 and 11's counters of `slot`'s session at the cutoff (ack_refused, trailing + unknown field, wrong
    /// channel: whichever is its kind's).
    pub(crate) fn counted(&mut self, slot: u8, counted: u64) {
        if let Some(f) = self.of(slot) {
            f.counted = counted;
        }
    }

    /// Each forger's `unrejected_count` by its kind's rule (the module text), never negative, plus its sequences forged
    /// again.
    fn unrejected(f: &Forger) -> u64 {
        let sent = f.items.len() as u64;
        let by_kind = match f.kind {
            8 | 9 | 11 => sent.saturating_sub(f.counted),
            7 => {
                let counted = |s: &&u16| !f.refused.contains(s) && !f.left_out.contains(s);
                f.items.keys().filter(counted).count() as u64
            }
            k => f
                .items
                .keys()
                .filter(|s| {
                    let runs = f.ran.get(s).copied().unwrap_or(0);
                    let ran = if k == 3 {
                        runs > 1 || f.copy_ran.contains(s)
                    } else {
                        runs > 0
                    };
                    !f.refused.contains(s) || ran
                })
                .count() as u64,
        };
        by_kind + f.reused
    }

    /// The `forged_kind` records, one per forger (fields `slot`, `kind`, `sent_count` less kind 7's left-out items,
    /// `measured_sent_count`, `unrejected_count`, the room's dropped records `refusals_dropped_count`, kind 7's
    /// left-out items `dead_left_out_count` (0 for the others); kinds 8, 9 and 11 their counter at the cutoff,
    /// `counted_count`).
    pub(crate) fn records(&self, dropped: u64) -> Vec<EventRecord> {
        self.forgers
            .iter()
            .map(|f| {
                let r = EventRecord::new("forged_kind", Phase::After)
                    .with("slot", f.slot)
                    .with("kind", f.kind)
                    .with("sent_count", f.items.len() - f.left_out.len())
                    .with("measured_sent_count", f.measured)
                    .with("unrejected_count", Attribution::unrejected(f))
                    .with("refusals_dropped_count", dropped)
                    .with("dead_left_out_count", f.left_out.len());
                if matches!(f.kind, 8 | 9 | 11) {
                    r.with("counted_count", f.counted)
                } else {
                    r
                }
            })
            .collect()
    }
}

/// f33 (TE3-2, TE-23, SA-4) and its two cases beyond the 13 (U4F2-Q4, C-U4F2-10), on hand-made records (line 15).
#[cfg(test)]
mod tests {
    use super::*;
    use npt_room::CommandRefusal;

    const SLOT: u8 = 4;

    fn cmd(seq: u16) -> Command {
        Command {
            slot: SLOT,
            seq,
            tick: 100 + u32::from(seq),
            ..Command::default()
        }
    }

    fn room(seq: u16, kind: CommandRefusal) -> RefusalRecord {
        RefusalRecord {
            slot: SLOT,
            seq,
            kind,
        }
    }

    fn sim(seq: u16, kind: RefusalKind) -> npt_sim::Refusal {
        npt_sim::Refusal {
            slot: SLOT,
            seq,
            kind,
        }
    }

    /// One forger of `kind` on `SLOT` with items 1 and 2 forged (kind 3: their copies with a changed move), then one
    /// tick of `room`, `sim` and `ran`: its record's (sent, unrejected).
    fn one(kind: u8, room: &[RefusalRecord], sim: &[npt_sim::Refusal], ran: &[u16]) -> (u64, u64) {
        let mut a = Attribution::new([(SLOT, kind), (SLOT + 1, kind)]);
        let copies: Vec<Command> = [1, 2]
            .map(|s| Command {
                move_xy: [9, 9],
                ..cmd(s)
            })
            .to_vec();
        a.forged(SLOT, &[1, 2], if kind == 3 { &copies } else { &[] }, true);
        let ran: Vec<(Command, Option<Rewind>)> = ran.iter().map(|&s| (cmd(s), None)).collect();
        a.tick(room, sim, &ran, &[]);
        let r = &a.records(0)[0];
        let n = |k: &str| r.f.get(k).and_then(|v| v.as_u64()).expect(k);
        (n("sent_count"), n("unrejected_count"))
    }

    /// The room kind of number `k`.
    fn kind(k: u8) -> CommandRefusal {
        use CommandRefusal::*;
        [
            LeadBound,
            NotNewer,
            SeqNotForward,
            MoveLength,
            PressEdge,
            AimBounds,
            AckNeverSent,
            Catchup,
        ]
        .into_iter()
        .find(|r| r.kind() == k)
        .expect("a room kind")
    }

    /// f33, one room-whole kind (1, 2, 4, 5, 6, 10): item 1 refused under its own kind and never run counts rejected;
    /// item 2, with no record of its own, counts unrejected.
    fn room_whole(k: u8) {
        assert_eq!(one(k, &[room(1, kind(k))], &[], &[]), (2, 1), "{k}");
    }

    #[test]
    fn f33_kind_1_counts_rejected_only_under_its_own_kind() {
        room_whole(1);
    }

    #[test]
    fn f33_kind_2_counts_rejected_only_under_its_own_kind() {
        room_whole(2);
    }

    #[test]
    fn f33_kind_4_counts_rejected_only_under_its_own_kind() {
        room_whole(4);
    }

    #[test]
    fn f33_kind_5_counts_rejected_only_under_its_own_kind() {
        room_whole(5);
    }

    #[test]
    fn f33_kind_6_counts_rejected_only_under_its_own_kind() {
        room_whole(6);
    }

    #[test]
    fn f33_kind_10_counts_rejected_only_under_its_own_kind() {
        room_whole(10);
    }

    /// f33, kind 3 (TE3-2): both copies refused under kind 3, the honest originals run once each: rejected; then item
    /// 2's copy content runs in its original's place: unrejected 1.
    #[test]
    fn f33_kind_3_counts_its_copy_rejected_with_the_honest_original_run() {
        let k3 = CommandRefusal::SeqNotForward;
        let both = [room(1, k3), room(2, k3)];
        assert_eq!(one(3, &both, &[], &[1, 2]), (2, 0));
        let mut a = Attribution::new([(SLOT, 3)]);
        let copy = Command {
            move_xy: [9, 9],
            ..cmd(2)
        };
        a.forged(SLOT, &[2], &[copy], false);
        a.tick(&[room(2, k3)], &[], &[(copy, None)], &[]);
        let r = &a.records(0)[0];
        assert_eq!(r.f.get("unrejected_count"), Some(&1.into()));
        assert_eq!(r.f.get("measured_sent_count"), Some(&0.into()));
    }

    /// f33, kind 7 (TE3-2): the commands run, item 1's press refused by the sim's reload check and item 2's by its
    /// cooldown: both rejected; a run with no sim record leaves both unrejected.
    #[test]
    fn f33_kind_7_counts_its_press_rejected_with_the_command_run() {
        let s = [sim(1, RefusalKind::Reload), sim(2, RefusalKind::Cooldown)];
        assert_eq!(one(7, &[], &s, &[1, 2]), (2, 0));
        assert_eq!(one(7, &[], &[], &[1, 2]), (2, 2));
    }

    /// f33, kinds 8, 9 and 11 (by count on the session): 2 forged, 2 counted at the cutoff: none unrejected; 1 counted:
    /// one; 3 counted (another drop on the session) is never negative.
    fn by_count(k: u8) {
        for (counted, want) in [(2, 0), (1, 1), (3, 0)] {
            let mut a = Attribution::new([(SLOT, k)]);
            a.forged(SLOT, &[1, 2], &[], true);
            a.counted(SLOT, counted);
            let r = &a.records(0)[0];
            assert_eq!(r.f.get("unrejected_count"), Some(&want.into()), "{k}");
        }
    }

    #[test]
    fn f33_kind_8_counts_by_the_rooms_refused_datagrams() {
        by_count(8);
    }

    #[test]
    fn f33_kind_9_counts_by_the_pumps_trailing_and_unknown_field_drops() {
        by_count(9);
    }

    #[test]
    fn f33_kind_11_counts_by_the_pumps_wrong_channel_drops() {
        by_count(11);
    }

    /// f33 (neg; SA-4): a forged command refused under another kind counts unrejected (kind 1 under kind 2, kind 2
    /// under kind 1; kind 7 with a room record and no sim one).
    #[test]
    fn f33_a_forged_command_refused_under_another_kind_counts_unrejected() {
        assert_eq!(
            one(1, &[room(1, kind(2)), room(2, kind(2))], &[], &[]),
            (2, 2)
        );
        assert_eq!(
            one(2, &[room(1, kind(1)), room(2, kind(1))], &[], &[]),
            (2, 2)
        );
        assert_eq!(
            one(7, &[room(1, kind(5)), room(2, kind(5))], &[], &[]),
            (2, 2)
        );
    }

    /// f33 (neg; TE-23): a planted copy that ran counts unrejected 1: kind 1's item 1 refused yet one command of (slot,
    /// 1) ran; kind 3's item 1 refused and run twice (the original and the copy).
    #[test]
    fn f33_a_planted_copy_that_ran_counts_unrejected_1() {
        let k1 = [room(1, kind(1)), room(2, kind(1))];
        assert_eq!(one(1, &k1, &[], &[1]), (2, 1));
        let k3 = [room(1, kind(3)), room(2, kind(3))];
        assert_eq!(one(3, &k3, &[], &[1, 1, 2]), (2, 1));
    }

    /// U4F2-Q4 (neg, beyond the 13): kind-10 records naming sequences the forger did not forge, on its own slot and on
    /// an honest one (an honest hold's cap refusals), change neither its sent nor its unrejected count.
    #[test]
    fn q4_an_honest_kind_10_refusal_changes_neither_count() {
        let base = one(10, &[room(1, kind(10)), room(2, kind(10))], &[], &[]);
        let mut other = vec![room(1, kind(10)), room(2, kind(10)), room(7, kind(10))];
        other.push(RefusalRecord {
            slot: SLOT + 1,
            ..room(3, kind(10))
        });
        assert_eq!(base, (2, 0));
        assert_eq!(one(10, &other, &[], &[]), base);
    }

    /// FP-6 (TE-U4F2-3, neg): an own-kind record on another slot that names a forged sequence leaves the item
    /// unrejected: room kind 10 on SLOT + 1 for item 2, and the sim's Cooldown on SLOT + 1 for kind 7's item 2.
    #[test]
    fn fp6_an_own_kind_record_on_another_slot_rejects_nothing() {
        let rooms = [
            room(1, kind(10)),
            RefusalRecord {
                slot: SLOT + 1,
                ..room(2, kind(10))
            },
        ];
        assert_eq!(one(10, &rooms, &[], &[]), (2, 1), "the room's");
        let mut sims = [sim(1, RefusalKind::Cooldown), sim(2, RefusalKind::Cooldown)];
        sims[1].slot = SLOT + 1;
        assert_eq!(one(7, &[], &sims, &[1, 2]), (2, 1), "the sim's");
    }

    /// FP-8 (RR-U4F2-2, neg): a sequence forged again (a second lap) counts unrejected whatever its records: kind 1's
    /// item 1 forged twice, refused under its kind and never run, reads (sent 1, unrejected 1).
    #[test]
    fn fp8_a_sequence_forged_again_counts_unrejected() {
        let mut a = Attribution::new([(SLOT, 1)]);
        a.forged(SLOT, &[1], &[], true);
        a.forged(SLOT, &[1], &[], true);
        a.tick(&[room(1, kind(1))], &[], &[], &[]);
        let r = &a.records(0)[0];
        let n = |k: &str| r.f.get(k).and_then(|v| v.as_u64());
        assert_eq!((n("sent_count"), n("unrejected_count")), (Some(1), Some(1)));
    }

    /// FP-8 (TE-U4F2-9, SA-U4F2-3): the room's dropped records reach every record (3 here), and kinds 8, 9 and 11's
    /// records carry their counter at the cutoff (`counted_count`), the other kinds' none.
    #[test]
    fn fp8_every_record_carries_the_dropped_count_and_kinds_8_9_11_their_counter() {
        let mut a = Attribution::new([(SLOT, 8), (SLOT + 1, 1)]);
        a.counted(SLOT, 5);
        for r in a.records(3) {
            let k = r.f.get("kind").and_then(|v| v.as_u64());
            let counted = r.f.get("counted_count").and_then(|v| v.as_u64());
            assert_eq!(r.f.get("refusals_dropped_count"), Some(&3.into()), "{k:?}");
            assert_eq!(counted, (k == Some(8)).then_some(5), "{k:?}");
        }
    }

    /// FP-2 (F-B-COUNT (a)), on hand-made inputs: kind 7's items 2, 4, 6, 8 and 10, one command a tick from 1 to 10
    /// but 9 (never run), with the soldier dead before the command phase of the ticks running 2 and 3. Item 2 (dead at
    /// d) and item 4 (dead at d - 1, alive at d) are left out; item 6 (alive at both, no record) counts unrejected; item
    /// 8 (alive, a Cooldown record) rejected; item 10 (d - 1 never ran, alive at d, a record) is not left out:
    /// `sent_count` 3, `unrejected_count` 1, `dead_left_out_count` 2.
    #[test]
    fn fp2_kind_7_items_run_dead_at_d_or_d_minus_1_are_left_out_and_counted_apart() {
        let mut a = Attribution::new([(SLOT, 7)]);
        a.forged(SLOT, &[2, 4, 6, 8, 10], &[], true);
        for seq in [1, 2, 3, 4, 5, 6, 7, 8, 10] {
            let dead: &[u8] = if [2, 3].contains(&seq) { &[SLOT] } else { &[] };
            let record = [sim(seq, RefusalKind::Cooldown)];
            let sims: &[npt_sim::Refusal] = if [8, 10].contains(&seq) { &record } else { &[] };
            a.tick(&[], sims, &[(cmd(seq), None)], dead);
        }
        let r = &a.records(0)[0];
        let n = |k: &str| r.f.get(k).and_then(|v| v.as_u64());
        let got = ["sent_count", "unrejected_count", "dead_left_out_count"].map(n);
        assert_eq!(got, [Some(3), Some(1), Some(2)], "sent, unrejected, out");
    }

    /// C-U4F2-10 (neg, beyond the 13): under PM-0001-21/no-press-edge the room takes kind 5's forged commands and the
    /// sim refuses their extra edges (Cooldown records of those sequences): they still count unrejected (room and sim
    /// kinds kept apart). Item 1 ran, as under the bite; item 2's record stands alone, so its count rests on the kinds'
    /// separation only (a rule matching any kind would count it rejected).
    #[test]
    fn c_u4f2_10_kind_5_items_with_only_sim_cooldown_records_count_unrejected() {
        let s = [sim(1, RefusalKind::Cooldown), sim(2, RefusalKind::Cooldown)];
        assert_eq!(one(5, &[], &s, &[1]), (2, 2));
    }
}
