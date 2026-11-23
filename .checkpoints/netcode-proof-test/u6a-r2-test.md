# U6a round 2, test seat (NPT-TRIM: what the fix pass changed only), 2026-11-23

Inputs: u6a-fix-pass.md (FP-1 to FP-12), u6a-build.md (FP-n lines), wip/u6a-build.patch (sha256 022ddb84...98e1,
checked; 19 files; 5,843 added), .gate/u6a/fp*.log, check4.log, fp-restored.log. Tree read only:
W = /home/user/Shamble-Frontlines/.claude/worktrees/agent-a95ebf14368f277ff/spikes/netcode-proof
Round 1: u6a-review-test.md (TR-U6A-1 to -12). Nothing run: no cargo, no plant, no runner call.

## Plan
1. Check the patch hash; read u6a-fix-pass.md and the FP lines in u6a-build.md. DONE.
2. (1) TR-U6A-1: super's new header test and the reworked double-load bite vs -16's check and the judge's rules. DONE.
3. (2) Each new or changed test's assertions (FP-2, -3, -4, -6, -7, -8, -9, -10, -11, -12). Mostly DONE.
4. (3) Plants: P7/P7b on the final tree and option A; P8, P9 (+FP-4), P12-P14; fp-restored.log's hashes. DONE.
5. (4) check4's counts against section 3's bars. TODO.
6. (5) Anything the fixes broke in round 1's scope. In progress.
7. Verdict, table, not verified, DONE.

## Notes
- Patch sha256 022ddb84...98e1 matches.
- (1) FP-1: npt-super takes --bite (optional), --scene and --tick-hz (both required; 1-1,000) (super main.rs:97-107,
  :136-137) and log() writes them (:201-208); features stay [] (npt-super has no bites feature). New test
  h_the_log_header_carries_the_runs_bite_scene_and_tick_rate (:560-593) runs the real time_starts and log() against a
  fake room opening after 2.1 s and asserts on npt-super's own super.jsonl: header (Some(bite), "SS", 30) and max
  spawn_to_open_ns >= 2e9, the check named in the message. P12 (log() back to None/""/0) fails it (fp-plant-p12.log:
  main.rs:573, left (None, "", 0)). binary.rs's double-load bite (binary.rs:222-240) drops the hand-built log and
  asserts the room's OPEN >= 2 s, the room header's bite and features ["bites"]. So -16's check now fails by name on
  the log npt-super itself writes, and that header can sit in the room's run (the judge's eight consistency fields:
  set, role, layer, profile, date by the old flags; bite, tick_hz, scene by the new). TR-U6A-1 closed. Left, by design:
  the judge never runs on it (an inline copy of the check), and no test runs npt-super against the real bites room (two
  links each tested: room OPEN >= 2 s; npt-super's timer and header). --bite and --scene are free text, not checked
  against the room's (a mismatch would make the run INVALID, never pass it): Low note.
- (2) FP-2 (main.rs:431-446, test :785-807): sibling_refusal on a fixed 2-thread map: io [1,7] refused with the line,
  [1,2] and no pinning not. P13 bites it. The call in run() (:444-446, exit 2 before minting) is not reached by any test:
  this container's map is 1 thread a core, so no binary test can trigger it; a removed call passes -> TR2-U6A-3 (Low,
  not-verified line).
- FP-3 (proc_read.rs:166-193, fixtures :418-447): empty walk, walk without tid 1, walk without the tick thread -> Err;
  whole walk Ok(true); empty SMT map and a None list -> Err. P14 bites it. Not fixtured: the TASKS_MAX cap branch
  (:167), and affinity()'s map of Err to 0 plus its stderr line (tick_loop.rs:1326-1342) is read, not tested -> Low,
  folded into TR2-U6A-3.
- FP-4 (tick_loop.rs:945-1097, test tests.rs:868-897): fed "" and a text without cpu3: no steal and no rss key, flags
  [true, true, false], CPU shares 500 and 100; then a good text: steal 100 (the StealReader kept its baseline across
  the failures), rss 4,096, CPU None -> no CPU keys and flag set. Asserts what FP-4 says. P9 also fails it (:889).
- FP-4 SIDE EFFECT (new): StealReader::read returns None also when no jiffy passed between two reads (proc_read.rs:
  "no time passed"), and values() now treats that None as a failed read: key left out and a "failed" stderr line. The
  pinned test still asserts steal_permille on EVERY second record (tests.rs:843-851). The second-0 record's read
  happens when the writer thread handles tick 30's row and the last record's read at close (stop); a writer that lags
  the tick thread by about 300 ms (rows handled in one burst at the stop) puts both reads within one jiffy (10 ms),
  so the last record lacks the key and the test fails. Round 1 wrote 0 there, so this flake is new (the builder's own
  note, u6a-build.md:253-254, saw the case and judged only a stop within 10 ms can meet it; it missed writer lag). On RS
  the same case gives a false "failed" stderr line and a gap in the last record -> TR2-U6A-1 (Medium).
- FP-6 (tests.rs:1033-1062, :1118-1149): counted() computes everyone by the lab's rule (every other soldier, ES
  vehicles; no own-soldier term) over c.relevant(), the override gone, the whole SceneRow compared on all 90 ticks,
  plus seen > 0. Proves the server's `own &&` rule equals the lab's rule on the chosen set (they differ only for a slot
  with no soldier in a room with no other soldier). Still untested, by construction: the lab reads the delivered frame,
  here relevant() stands for it (the test's own comment, :1033-1034). Not-verified line.
- FP-7 (carry.rs:99-228): four legs (A, B x down, up), B now explicit (:111-114), all run before the asserts.
- (3) P7 on the final tree (fp-plant-p7.log, 03:07): cap removed -> FAILS at carry.rs:224, "B, its ping slow (up)" 6
  refused; A down, A up, B down 0 refused at lead 9. Why A never bites: under option A a command runs at the first
  tick after it arrives and kind 10 counts commands buffered for the coming tick on arrival (room input.rs:42,
  :316-326, :336); the stamp, which is all the controller's cap moves, plays no part in A's kind 10. So "the cap holds
  under option A" is true by A's rule, not by the cap: no plant of the cap can bite the A legs, and they are a
  non-regression run of the honest client under A. What closes C-U6-17's A half: the report states it as proven by
  reading (input.rs:316-326, :336) with the netcode seat's yes, and names the A legs as non-regression, not proof; the
  test's name still says "under_option_b" -> TR2-U6A-2 (Low).
- P8 (fp-plant-p8.log): FAILS proc_read s4_alone... at :390 (4 passed, 1 failed). P9 (fp-plant-p9.log): FAILS
  s4_the_steal_reader... (:335, Some(27) vs Some(100)) and s4_a_failed_per_second_read... (tests.rs:889). P12, P13,
  P14 above. Each the named test(s), exit 101, no compile error; the runner's diff or untracked hash off the clean
  tree's while planted. fp-restored.log: the five files at client 1613a3f0, proc_read b231a3f6, super 0e2a4269, server
  main ef115163, tick_loop 7dd0a9bf, tree diff 0fc8865b / untracked 421bff63 = check4's; the five hash the same now
  (2026-11-23, my sha256sum). The fix-pass plant calls ran `cargo test` without `--locked` (round 1's had it); the
  next call's diff hash (which covers Cargo.lock) is the clean tree's, so the lock did not move: Low note.
