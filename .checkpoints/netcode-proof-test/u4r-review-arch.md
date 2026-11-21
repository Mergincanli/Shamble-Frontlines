# U4r review, round 1: engine-architect seat (batch 2)

Snapshot under review: .checkpoints/netcode-proof-test/wip/u4r.patch, sha256 572c4f69435da2ad6f62d54d4bcbc47b04d6329b5493e0faf78fcecfdc4a09c2 (checked by sha256sum at start: equal).
Packet u4r-packet.md sha256 cbd82485... (equal); u4r-packet-tests.md sha256 d12555c8... (equal).
Read copy: .gate/u4r-snap1 (2fbec77 + patch). 26 entries in git status (13 M, 13 ??).
No engine-architect memory on disk (.claude/agent-memory-local/ holds only security-auditor).
No builder report on disk (u4r-build.md :1541 "Next: the report"): counted from the diff.
Nothing run (no cargo, no check.mjs): every finding below is from reading the read copy; no log under .gate/u4r-review-arch/.

## Plan (seat list: advisor-u4r-panel-full.md section 4)
1. Public items: every pub item the diff adds, by crate; compare to brief 2.7 allowed list + rulings (with_bite, contract/precheck subcommands). Report's list if on disk.
2. check.mjs changes counted as d2-r2 counted them (EA2-1, ruling 1 point 1): wasmStep's two options + defaults; carries 83 (b)-(d), 84, 85; new exports; det-u4.mjs imports wasmStep.
3. The seam (2.6): only merged pub items of npt-room/client/server/sim; no source change beyond lane; nothing depends on npt-lab; no other crate's bite id spelled in npt-lab non-test sources.
4. U4R-Q6 (a) server items: with_bite shape, BITES at four + pin in tests/schedule.rs, loopback.rs one-home test.
5. My earlier findings: carry 82 (EA-8), 83 (a)(d) (EA-3, EA-6, EA2-3), 75 (A3), 76 A6 half, 59 (A2-L3), 3.3 carries 6 and 20 (arch F6, arch N2).
6. Report's statements of missing read-only items as follow-ups (r4 spatial half, U6 bots' view, HISTORY_MS at U4h); ruling 4 point 2's bite-id choice.

## The seat's own public-items count (for the report's list, when it arrives)
- npt-bots: `Behaviour` (struct, every field private) and 6 methods: `brawl`, `script`, `every_step`, `given`, `skipped`, `step` (bots/src/lib.rs:76, 138, 144, 157, 163, 168, 176). Allowed: §4.1 :197.
- npt-server, bites builds only: `Decoder::with_bite(self, &str) -> Option<Decoder>` (pump.rs, new; U4R-Q6 (a)); `BITES` changed from `[&str; 3]` to `[&str; 4]` (schedule.rs; U4R-Q6 (a)). `WALLCLOCK_BITE` is pub(crate), `wallclock_aims` private.
- npt-log: none (`control_char`, `NoDuplicateKey` private).
- npt-lab library: none (lib.rs:8-9 doc text only, carry 83 (a)).
- npt-lab binary: subcommands `scenario` (with `--record`) and `replay` (2.7), `precheck` and `contract` (ruling 3 point 6), main.rs:225-313. `SimClock`, `ReplayX86` pub(crate) in the binary; no item named `L1Scenario` exists. No bare `pub` in any binary module.
- npt-hash-wasm loader.mjs: header comment only; exports equal to base.
- check.mjs: 67 -> 69 exports: `cargoConfigDirs` (carry 85 (d)) and `run` (carry 84 (b)-(c)), both imported by check.test.mjs:37, :62; `wasmStep` gains one options parameter `{ manifest = RECORDINGS_MANIFEST, bitesHalf = true }` (check.mjs:2321) whose defaults keep STEPS.wasm's call (:2594); det-u4.mjs:21 imports `FAILED_STEP_DIR`, `main`, `wasmStep` (no copy). check.test.mjs 71 -> 76 tests.

## Progress (facts checked; a line each)
- (1) as above; brief 2.7 met but for the L1Scenario naming (F1).
- (2) as above; with defaults the only printing changes are carries 83 (b) and 84's own (exit-0 problems, the shared deadline's lines).
- (3) seam: no diff under crates sim/proto/room/client/l1/link/net/judge, Cargo.toml, Cargo.lock; nothing depends on npt-lab (grep Cargo.toml and npt_lab:: outside crates/lab). Bite literals in lab non-test code: only scenario.rs:197 LAB_BITES (lab's own) and :200 (cfg(all(test,bites))); the rest in test modules.
- (4) with_bite shape as ruled, cfg(bites); site wallclock_aims with the allow on that fn alone; BITES at four incl. crate::pump::WALLCLOCK_BITE; tests/schedule.rs:350 pin updated and WALLCLOCK in Schedule::with_bite's None list; loopback.rs one-home test reads npt_server::BITES.to_vec() whole (:337-348).
- (5) carry 82 met: hash.rs:201-321 (Kind, name_ok, ticks <= bytes/12 before generating), regenerate_all :402-436 checks generators first; carry 83 (a) met: lib.rs:8-9; precheck.rs:383-387 one recording name on both legs, HashCompare Debug line :738; carry 83 (d) met: hash.rs:1655; carry 75 met: scenario.rs:719, :752-753, bound :849-853 (logged runs only: ruling 1 point 9 (d)); carry 76 A6 half met in code: Server::io/drop_slot/send scenario.rs:405-545, r8/r9 :1943, :1981 (F6, F7); carry 59: c-u4r-r2.mjs:61 ES suite; carry 6 met: scenario.rs:317, :321, precheck.rs:303; carry 20 met: no grid::Config hashed/recorded/compared (mirror.rs:152 default by value; replay via ClientEndpoint::with_bite, replay.rs:176).
- (6) no report in hand: its follow-up statements (r4's spatial half, U6 bots' view, HISTORY_MS at U4h) and carry 59's 7.250 line cannot be checked; ruling 4 point 2: lookup contract.rs:248-256, assertion :579-590 present.

## Findings (a line each, as found)
- F1 Low: RQ8's order has two homes and no `L1Scenario` item: scenario::run (scenario.rs:702-1158, SimClock :819, :1062) and precheck::run_leg (precheck.rs:266-479, its own loop with a bare u64 `now` at :350, while its doc :3 says "on one SimClock each"); lib.rs:3-5 says the library holds SimClock, L1Scenario and ReplayX86.
- F2 Low: npt-lab names npt_sim::World in three of U4r's modules (scenario.rs:40, logs.rs:24/:97/:455/:578, precheck.rs:48/:277), two construction sites (scenario.rs:704-726, precheck.rs:277-299), against brief 2.1 :356 / plan U4-SEAM :162-163 "names npt_sim::World once".
- F3 Low: Bites::of sets `schedule` from npt_server::BITES (scenario.rs:214), which now also holds the pump's id; schedule() then silently falls back to a clean Schedule (:311-322); world() and net() fall back silently too while room() and client() expect.
- F4 Low: PLACEMENT_Z_BOUND_CM (scenario.rs:61) copies npt-sim's private CHEST_CM (world.rs:79) as a literal 140; s1 (:1721-1725) pins the copy to a literal 3,955, not to npt-sim; a third copy beside rulings 1 points 2 and 5's, with no follow-up.
- F5 Low: Behaviour::step's `received` route cannot give a view of the newest tick; its only host bypasses it (scenario.rs:964-970) while bots/tests/behaviour.rs:92-165 test the other route.
- F6 Low (carry 76's evidence): r8 never reads the old link for Kick(Replaced) (scenario.rs:1943-1979); deleting drop_slot's send (:409-414) keeps r8 green. On L1 the Kick gets one transmission (close stops resends: l1/src/lib.rs:1683-1695).
- F7 Low: a replacement marks the live slot gone in the logs: io pushes (slot, "replaced") after admitting the new session (scenario.rs:444, :472, :405-419), and run calls logs.gone for every ended entry (:857-859; logs.rs:263-268). No U4r run replaces a session. `owner` also stays stale on "closed" (:488-495).
- F8 Low: scenario.rs:9-12 says each reliable message goes on the channel its Limits::TYPES row names; the code trial-decodes (stream_of :133-144, as :24-25 say), never reading TYPES (brief 2.1 (2) prescribed the row).
- F9 Low: crates/server/tests/schedule.rs:348 is still named the_three_bites_are_planted_here_under_their_marks_json_ids while it pins four ids.

## Verdict
APPROVE WITH FINDINGS: nine Lows, none Medium or above; no security finding from this seat.
