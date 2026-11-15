# BRAWL-1 review, round 1, netcode-architect seat

Snapshot: wip/brawl1.patch sha256 ee2973aa8ad8c453b5720099c27b37cb9980820da74f33ad9c78ddd7ad591dc7 (verified 2026-11-15)
Packet: brawl1-packet.md sha256 16eab462c4e0fcbcd8360aad7a951207275927c7f0f6813b794df54c12f4b1a4 (verified)
Read copy: .claude/worktrees/brawl1 (352a245 + patch uncommitted)

## Plan
1. Read the packet: approved plan, conditions 1-18, rulings copied.
2. Read the patch whole.
3. Focus items:
   a. client read item: newest complete Hash-filed tick by endpoint wrap rule; dead bit.
   b. leash under a lead's delay and under B1 bursts (conditions 1-3; pattern (iii)).
   c. room log in-building figure (condition 10).
   d. Part 1 refactors (wake step, run_leg SimClock, shared constructor): behaviour change?
4. Run tests via u4r-run.sh if useful; logs to .gate/u4r/brawl1-review-netcode/.
5. Findings, one line each as found.

## Progress
- Steps 1-2 done: packet read; diffs read.
- (d) checked: SimClock default 0, advance_to sets exactly t (assert t >= now holds: next.max(now+1)); wake() reorders
  due_ticks before the admit/gone sinks: pure on (now, schedule), so same; Server::build == run's old path; run_leg/lab_room
  gain with_event_start(0): Room::new next_event 0 (room.rs:459), with_event_start sets only next_event (:488-491): same;
  Bites::default().quant() == Quant::default() (decoder false), room() identity, world() Sim::new. No behaviour change found.
- (c) checked: room_tick called on every tick when s.logs (scenario.rs:1022-1026), any phase; Counts measured only;
  summary via logged(); no end-event field; SERIES row; TickRecord.s doc allows pairs (log lib.rs:111); contract (b) only.
- (a) checked: wrap.after32("tick") on self.wrap == Wrap::default() for "tick" in every bite (plain counters are
  tick_offset/event_seq only); own section always holds (Soldier, owner) (room.rs encode_all); full_state files Hash,
  groups 1 got 1. Test has no 32-bit wrap case (max_by_key mutation survives).
- (b) evidence: final-regen-tests.log (final head) full test: EveryTick max 1575; Skips max 1720 (seed 2, slot 36 at
  -1354,-1061); Bursts max 1832 all 3 seeds (past 1800 door-entry radius), in-building 0.

## Findings (one line each)
- F1 Low: endpoint.rs brawl1 test has no 32-bit wrap case; a max_by_key(tick) mutation passes it (client lib.rs:352-362).
- F2 Low: barred Skips pattern reaches 1720 cm (80 cm from 1800 door entry) at the minimum radii condition 1 allows;
  LEASH_OUT_CM doc (bots lib.rs:38-43) states the 4 m geometric margin only; bursts reach 1832 (past it).
- F3 Low: dead bit/flags index copied literals (client lib.rs:245-248; bots test :334) with no tie to npt-sim.
