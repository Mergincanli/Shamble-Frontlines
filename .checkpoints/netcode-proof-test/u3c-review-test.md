# U3c skeptic round: test-engineer notes

Scope: git diff 2c68bf9..989118e -- spikes/ ; plan u3c-plan.md rev 2 [C1]-[C16]; plan.md rev 3.3 U3c text.
Focus: proof per item/condition (bites, mutations), model-state vs wall clock judgement, real-time flake risk,
wt_tests FT1-FT4 fix, check.mjs deps checks + self-tests, evidence scripts' pass rules, commit-message claims.
Build dir: CARGO_TARGET_DIR=/home/user/Shamble-Frontlines/.gate/u3c-review-target (no network changes).

## Plan
1. Read u3c-plan rev 2, advisor notes, build/critic notes.
2. Diff stat, read each file in diff.
3. Map conditions to tests; check bites.
4. Run tests (cargo test) where cheap; check.mjs self-tests.
5. Findings.

## Notes
- Read: u3c-plan rev 2, advisor-u3c-plan (C10 ruled (a) accept: not re-ruled), build/critic notes a/b, commit msgs.
- Scripts fixed before runs: run.sh/compare.py/probe.py committed c5d7260 00:35:47Z, run started 00:47:23Z;
  siocoutq-root-check.py 2a93038 00:05Z, run 00:52:58Z. OK.
- ws_gate_tests: (a) pinned literals + x2 bite; (b) sweep iff restated independently, boundary cases in
  may_write_needs_all_three_conditions (233-238) catch off-by-one; queueing + x2 bites. Met.
- fixes.rs: adaptive_limit + room step call ws_gate (one definition); (e) grid test; pins unchanged. Met.
- delay_tests (c): faithful outstanding == N x SNAP tight at returns (meaningful); bites x2/unsent caught; owed-after-write
  real. Robust: srtt cannot change between last write and return without an ACK that also lowers outstanding.
- F-A (low): profile-burst conversion in wrap (delay.rs:591-608: clock->model axis, clip at origin, burst_ends,
  MAX_BURSTS refusal) has no test; no test uses cfg.profile.bursts_ns (grep).
- F-B (low): add_burst's clock->model conversion (delay.rs:704) only indirectly tested: burst test judges on the
  logged (already converted) start; (c) never asserts Return.at_ns == start + b + SEC (delay_tests.rs:1695-1696).
- F-C (nit): TLS test's "ring" check is self-referential (delay_tests.rs:1374-1376 compares against
  ws_config::provider()); the measured build's feature set is the real guard.
- F-D (medium): check.mjs deps step never reads the workspace root's [workspace.lints.rust] unsafe_code = "deny"
  (Cargo.toml:76) nor .cargo/config.toml rustflags; crateFiles() (check.mjs:351-356) scans crates/ only. A one-line
  root change relaxes unsafe_code everywhere while DEPS_CHECKED (check.mjs:473-478) still says it holds.
- F-E (low): a crate manifest's [lib]/[[bin]]/[[example]] path or build= pointing into crates/tcpinfo/ is another
  borrow route pathProblems (check.mjs:238-266) does not cover; cargo metadata's target src_path would.
- F-F (low): records before a bad frame enter the model but are never released once the pump ends (close ->
  Pump Ready -> pump_done -> current() stops stepping, delay.rs:746, 1281-1283); doc delay.rs:451-452 implies order kept;
  no test checks preceding frames reach tungstenite (each_limit test only asserts InvalidData, delay_tests.rs:666).
- F-G (low): up EOF "after its last delivered byte" (plan 1.2 C3; delay.rs:1075) has no test (no test closes the
  client's write half and reads DelayStream to EOF).
- F-H (low): a_full_budget test (delay_tests.rs:573-590) asserts only upper bounds; it passes vacuously if the flood
  never reached the bound (stall); no assertion that max[UP]/max[DOWN] got near 64 KB.
- F-I (nit): one_record test (delay_tests.rs:503-537) assumes no ACK within the 2 s RTT: a >= ~2 s stall breaks it.
- F-J (nit): root-run provenance "(with uncommitted changes)" is always printed: git status covers the spike folder
  incl. the log being written under results/ (siocoutq-root-check.py:186-188).
- Ran (scratch copy scratchpad/npt, CARGO_TARGET_DIR .gate/u3c-review-target): l2::delay baseline 22 pass, 1 ignored, 18.4 s;
  check.mjs --only deps,self on the real tree: PASS (78 crate files), self 21/21.
- Mutations (scratch copy, restored each time): M1 up_room without released bytes -> a_full_budget FAILS (held up 152,548):
  C2 bound bites. M2 read floor removed (delay.rs:1286) -> all 22 PASS: the 549 B floor rule is untested (low).
  M3 add_burst without the origin (delay.rs:704) -> caught only in the module run (burst test's refusal check at
  delay_tests.rs:948-953, needs origin > 1 ms); burst test alone 3/3 PASS, (c) alone PASS: order-dependent detection
  of C5's "bursts on DelayStream's clock" (F-B upgraded to medium). M4 release stamped at the step target -> replay +
  base-RTT FAIL. M6 record ignores 22 B -> one_record + 2 more FAIL (debug_assert delay.rs:317 and pinned numbers).
- F-D confirmed (rootlint.py): root unsafe_code = "allow" + an unsafe block in npt-room: deps step PASS, npt-room builds
  with --allow=unsafe_code; control with deny: build error. Medium.
- wt_tests FT1-FT4: FT1 sent==200 and received==frame_rx asserted (wt_tests.rs:129-141); FT2 missing<=real and up drops 0
  (:120-126, :142-150); FT3 in checkpoint :333-356; FT4 comment :40-48. Met. Bite: at 5 ms the 178 case had 22 missing vs
  7 real, so FT2 bites the old failure mode. Residual wall-clock asserts (rtt 75-160 ms :86-91, setup < 10 s :70-74,
  clean rtt < 20 ms :261/:445) pre-existing; ">= 180" band level not stated (Bin(200,0.02) tail ~1e-9) - nit.
