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
- M10 (release up to 1,100 B before delivery) -> replay, split, loopback FAIL; order_and_integrity PASSES: its "no
  release before its stamp" half (delay_tests.rs:469-492) is by construction (release stamped at the step time <= now) -
  nit, the replay is the real proof.
- F-K (medium): (c)'s S12 assertion (delay_tests.rs:1791-1792 `expect("a byte after the return")`) needs the first
  down byte after the 2nd return (~start+5.5 s) inside the fixed 230-tick run (7.63 s): margin ~0.25 s against observed
  first-byte times 1,729-1,869 ms (24 runs under 3-way load: 0 fail, max 1,868.7 ms; cloop.py). Probe M12 (writes 5 ms
  later in each tick, i.e. another tick/burst phase): 1 of 4 runs failed at that expect (copy line 1795:46). The
  outcome depends on the wall-clock tick/burst phase: a duration-coupled assertion (contrary to C16). Fix: run until
  both returns' first byte and first message are recorded (model state), with a hang guard, not a fixed tick count.
- F-L (low): (c)'s gate inputs are read in four separate locked calls (delay_tests.rs:1714-1723), each stepping the
  model; an ACK between the srtt and outstanding reads can let a correct gate leave outstanding one SNAP above N(srtt at
  the return): a rare false failure (W2's pattern); U6's WsServer needs one atomic read of the four inputs on L2 anyway.
  Probe M11 (20 ms blocking gap, loop falls behind) failed the x2 BITE assertion instead (1802): bites need the writer
  to keep pace with the tick.
- F-M (low): DelayStream's release clock slack (1 ns written and read back, plan 1.2; commit c5d7260) is never asserted
  (no "slack" in delay_tests.rs); ImpairSocket's precedent asserts it (socket_tests.rs:294-333).
- F-N (low): the non-Linux stub path ("the workspace still builds anywhere", C12 (c)) is not in check.mjs: clippy-wasm32
  covers LEAF only (check.mjs:22, :448); tcp_kernel.rs:39-42's not(unix) handle is never compiled; one manual wasm32
  clippy in build B's notes. Critic B's point, still open.
- Cargo.lock diff: + npt-tcpinfo, + tokio-rustls; npt-net gains futures-util (dev), npt-tcpinfo, tokio-rustls. As plan §5
  + the advisor's futures-util ruling. Test counts 242/256 = 233+9 / 247+9 (build A fix + build B's tcpinfo 8 + tcp_kernel 1).
- Kernel check verdict.json: problems [], cubic/cubic all runs, delivered 3000/3000, retrans 61/56/61; model 100 KCMODEL lines.
- F-F confirmed by probe (limit_case read loop instrumented in the copy): in all 9 limit cases the first read after the
  close is the InvalidData error although 170 B (request) or 500 B (request + fragment) had entered the model: records
  before a bad frame are never handed over.
- wt tests (all features) 3 runs: 9/9 pass each; P2 up 193-195, down 198-199, frames sent 200, frames == received.
- check.mjs mutations (checkbites.py, scratch copy, restored): 8/8 caught by check.test.mjs (libc check, dev-as-shipped,
  any-allow placed, inner-as-outer, no path check, string mentions, only-npt-net, count check). Self-tests bite.
- Nits: build B's "15 of 15 mutations" script (scratchpad u3cbfix/bites_check.py) is not kept in .checkpoints (only the
  first 11-mutation u3c-build-b-bites.py); SHUTDOWN_WAIT_NS 10 s default never asserted (tests use 300 ms);
  wt_tests ">= 180" band level not stated.

## Verdict
REVISE (small): F-K (C16: duration-coupled S12 assertion in (c), failure path shown), F-D (deps step overclaims: root
lint / rustflags unchecked, shown), F-B (C5 burst-axis conversion caught only by test order, shown). Lows/nits recorded
above may be deferred with a line each. Every other rev 2 item has a test that bites (M1, M4, M6, M10, the builders'
bites, check.mjs 8/8). Evidence scripts' pass rules predate their runs.
DONE
