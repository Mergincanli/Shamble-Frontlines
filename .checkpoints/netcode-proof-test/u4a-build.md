# U4a build: the wire (npt-proto), notes

Builder: general-purpose agent, opus, worktree /home/user/Shamble-Frontlines/.claude/worktrees/agent-a9297644e7c0354f3,
base dffef0e (branch claude/netcode-proof-test-plan-hcky5g). Plan: u4-build-plan.md rev 2 (sha256 0d69332c…) §5 U4a,
§4.1, §4.4-§4.6, §4.11, §4.14; advisor-u4.md (U4 verdict, U4s ruling, F1/F2 ruling); the catalogue; netcode.md;
u4s-carries.md entries 1-5 and 34. Files: crates/proto, check.mjs, check.test.mjs only. Commit nothing.

## Plan
1. npt-proto modules: bits (pub(crate) bit and byte readers and writers), quant (`Quant`: per-field codes from each
   `Field`'s min, max and bits, signed two's complement / unsigned / offset; -57's integer coarsening, -17/-58 tick16,
   -21 no-trailing and no-channel as its bites; `BITES`), wrap (`Wrap`: 8, 16, 32-bit compares by counter name,
   16-bit rebuild; bites-only plain-< defect for one named counter, no ids: their sites are U4e's), limits (`Limits`:
   every catalogue size, the redundancy cap, group cap, the per-type table), msg (the 15 types, `Message`,
   `Direction`, framing per channel, `encode`, `decode`), input (whole + delta repeats), snapshot (header with group
   index and count, `EntityDelta`, the layout tables from `EntityKind::layout()`, the record mapping), control
   (`SessionControl`, `RoomControl`), hash (`client_hash`, FNV-1a-64, never 0), error (`DecodeError`, `EncodeError`).
2. Tests a1-a9 + carry 4 (pitch -512) + catalogue items 1, 2, 3 (size half), 6 in tests/catalogue.rs with broken-copy
   bites.
3. check.mjs: sim/proto clippy.toml comparison (a9); npt_link-only-Channel grep for proto/src and client/src (carry 3);
   the four nits (carry 34): pin STEPS.doc and -D warnings (and fmt, clippy, test), doc the wasm32 build (tcpinfo's
   stub), lintBothBuilds summary on a bites-only failure, a measured-build failure's summary line names the first error.
   Self-tests with plants in check.test.mjs; the deps fixture gains a plant per new check.
4. Bites: mark bites in the bites build (unit-shown); scratch plants per test, in place, one at a time, sha256-restored.
5. Final: check.mjs twice, clippy host and wasm32 both builds, fmt.

## Progress
- (cut at ~18:40 UTC; resumed 21:40 UTC) Files written, never compiled yet: src/bits.rs (BitWriter/BitReader, LSB
  first, 2 unit tests: done), error.rs (DecodeError 10 kinds, EncodeError 4: done), limits.rs (Limits consts +
  TYPES table: done), quant.rs (Quant code/value per Field coding signed/unsigned/offset; -57 coarsening, tick16,
  no-trailing, no-channel; BITES 7 ids; tests a4 x2, pitch -512 (carry 4), BITES list: done), wrap.rs (after8/16/32,
  widen16, plain_lt defect; a5 + its bite test: done), msg.rs (15 types, Message, Direction, framing, encode/decode in
  the size-first order; type-table/enum test, channel_max test: done), input.rs (whole 23 B / repeat 19 B, canonical
  decode; field-table test: done), snapshot.rs (Snapshot, FullState, EntityDelta between/apply, Class, GROUPS
  tables; layout-kinds test: done), control.rs (SessionControl, SessionStatus, ProfileId, StepResult, RoomControl,
  TokenKind; note Cc/Cf test: done), hash.rs (client_hash FNV-1a-64, never 0: done, vectors not yet pinned), lib.rs
  (re-exports: done). Not started: first compile; integration tests tests/wire.rs (a1, a2, a3, a6, a8, mapping),
  tests/bites.rs (mark bites' unit proofs); catalogue.rs items 1, 2, 3, 6; check.mjs + check.test.mjs; plants; runs.
- 21:50 first compile: both builds compile; lib tests measured 10 pass, bites 13 pass (one bits test's own byte count
  fixed: 12 not 13); clippy -D warnings host both builds and wasm32 bites clean after one needless_range_loop fix in
  EntityDelta::between. Next: tests/common/mod.rs generators, tests/wire.rs.
- 22:10 tests/common/mod.rs (fixed-seed generators from layout(); snapshot drops entities until it fits) and
  tests/wire.rs: a1 (15 types + RoomControl x 10,000), a1 mapping (every kind x 2,000, through Snapshot and FullState),
  a2 prefixes (1,000 per type, 200 for Snapshot/FullState, every cut), a2 random (100,000 per type, length capped at
  1,200 B, + 10,000 bit-flipped valid encodings per type; Ok must re-encode to the same bytes), a3, a6, a8 (pinned
  0x88201fb960ff6465 empty and 0x8d9e8ddaafef41e8 events, both from an independent Python FNV-1a-64; records via an
  in-test reference byte string, so U4d's layout completion does not move a pin). All 7 pass; a2 random 9.4 s (was
  49.5 s with 65 KB random full states: capped). Coverage seen: every DecodeError kind reached by flips.
- 22:25 tests/bites.rs (bites build, 5 tests: BITES list; -57 x3 coarse through encode/decode at a tie and the top
  end, VELOCITY untouched; tick16 x2 ids: 65,537 decodes as 1 and hashes apart, below the wrap no effect;
  no-trailing; no-channel) all pass. tests/catalogue.rs gains audit_u4 (items 1, 2, 3 size half, 6 vs the doc) with
  9 broken copies, item 2/3 code half (each type on every other channel WrongChannel, wrong direction, doc limit + 1
  TooLarge), item 6 (largest control, 1,025 B, 10,000 random): all 7 catalogue tests pass. fmt (cargo fmt -p
  npt-proto) and clippy -D warnings both builds clean. Next: check.mjs (a9 clippy.toml compare, carry 3 npt_link
  grep, carry 34 nits) + check.test.mjs.
- 22:45 check.mjs: clippyTwinProblems + CLIPPY_TWINS (a9), linkUseProblems + CHANNEL_ONLY (carry 3) and npt-link
  rename refusal in dependencyProblems, wired into depsStep and DEPS_CHECKED; carry 34's nits: STEPS.fmt/clippy/
  clippy-wasm32 take a runner, lintBothBuilds/docBothBuilds/testBothBuilds exported with a runner (doc also a base
  env), doc adds the wasm32 doc of WASM_CRATES both builds (tcpinfo's stub), lintBothBuilds names a bites-only failure,
  failedRun puts the first `error` line in every failed build's summary. check.test.mjs: fixture gains the two
  clippy.toml files; deps-step plants +3 lines; 4 new tests (clippy twins, link use + rename, steps pinned with a
  recorder, failure summaries). self 52 of 52. Expected counts: measured 275 + 21 = 296, bites 292 + 29 = 321.
- 22:55 first full check.mjs: 10 of 10 PASS but test 282 / 304 (deltas +7 / +12, not +21 / +29): another copy's
  npt-proto artifacts ran (shared target, same hashes); proto alone then counted lib 10, catalogue 7, wire 7 (+21).
  That run is not reported. A bites clippy also met another copy's stub npt-proto rlib once (unresolved imports);
  re-run clean. Pub-surface trim: EntityDelta::id removed; SessionControl::variant and RoomControl::name now pinned
  by catalogue item 6's test; proto clippy.toml header names the check. Baseline sha256 (first 16 hex) before plants:
  bits 4a390bdd0679dd1c control 390c1fa059c7d336 error ce66d7b58375f789 hash 5a24907f63b6340f input 75b167fd93d08bf0
  lib a8603264db0cf0c3 limits 08e80997cab3809e msg 63677d4f00e3e151 quant 0d41e906a41f1c4d snapshot ef9e601309e0a88d
  wrap 9f2613c747b35c9c tests/bites 9d7cb2f874485439 catalogue 14313b7ba5bb1f38 wire 8675723a2b32b636 common
  17bee55e5d501f55 clippy.toml 30ab9b2103d6b4e0 check.mjs 7eddc285abef4580 check.test.mjs da63983fc622b546.
  Next: plants P1-P17, one at a time.
- Plants (each in place, touch, the named test FAILED, restored, sha256 = baseline):
  P1 input.rs whole-command seq ^ 1: wire a1_every_message_type… FAILED, a2_random… FAILED (5 pass, 2 fail);
     restored 75b167fd93d08bf0.
  P2 msg.rs `bytes[frame]` unchecked index: a2_every_proper_prefix… FAILED "Input: the decoder panicked on " (the
     empty prefix); restored 63677d4f00e3e151 (checked after P3).
  P3 msg.rs the type-limit size check disabled: a3… FAILED (Resync case 0, a padded encoding: TrailingBytes not
     TooLarge) and catalogue u4_items_2_and_3… FAILED; restored 63677d4f00e3e151.
  P4 quant.rs value() clamps in place of refusing: lib the_pitch_code_minus_512… FAILED (carry 4), and a2_random…
     FAILED; restored 0d41e906a41f1c4d.
  P5 wrap.rs after16's rule replaced by a plain `<`: lib a5_each_width… FAILED (wrap.rs:116, the 16-bit wrap);
     restored 9f2613c747b35c9c.
  P6 bits.rs finish() lets trailing bytes through: wire a6… FAILED (Ok(Input) where TrailingBytes); restored
     4a390bdd0679dd1c.
  P7 msg.rs the channel check disabled: catalogue u4_items_2_and_3… FAILED ("Input on Control"); restored
     63677d4f00e3e151.
  P8 hash.rs FNV prime ..01b5: wire a8… FAILED (left 4129966446014373093, pinned 9808874869469701221); restored
     5a24907f63b6340f.
  P9 (a9's self-test bite) proto clippy.toml `"f32::sin", "f64::sin",` → `"f32::sin",`: check.mjs deps FAIL
     "crates/proto/clippy.toml:31 differs from crates/sim/clippy.toml:35 below their headers…", self 51 of 52 (not ok
     49, the clippy-twins test's real-tree assertion); restored 30ab9b2103d6b4e0.
  P10 (carry 3) `pub use npt_link::Link;` in proto lib.rs: deps FAIL "crates/proto/src/lib.rs:46 names
     `npt_link::Link` from npt_link…", self 51 of 52 (not ok 50); restored a8603264db0cf0c3.
  P11 check.mjs depsStep without clippyTwinProblems: self not ok 38 (the deps-step plant test). P12 without
     linkUseProblems: not ok 38. P13 docBothBuilds without its two wasm32 runs: not ok 51 (steps pinned) and 52.
     P14 RUSTDOCFLAGS "" in place of "-D warnings": not ok 51. P15 lintBothBuilds's old "both linted" on a bites
     failure: not ok 52. P16 failedRun taking cargo's last line: not ok 52. Restored after each; check.mjs
     7eddc285abef4580 = baseline, self 52 of 52.
  P17 quant.rs pos-4cm's step 4 → 1 (the defect a no-op): bites lib a4_pm_0001_57s_three_bites… FAILED, tests/bites
     pm_0001_57s_bites_send_coarse… FAILED. P18 tick16 a no-op: tests/bites tick16_writes_the_header_tick… FAILED;
     quant.rs restored 0d41e906a41f1c4d. P19 wrap.rs plain_lt a no-op: bites lib a5_the_plain_lt_defect… FAILED;
     restored 9f2613c747b35c9c. P20 snapshot.rs apply() off by one on layout field 8: wire
     a1_the_entity_record_mapping… FAILED (wire.rs:115); restored ef9e601309e0a88d.
  After P20: every file's sha256 = the baseline above (18 files).
- Next: touch, two full check.mjs runs (expect measured 296, bites 321, self 52).
- 22:13 run A: 9 PASS, test FAIL after the measured build counted 296 (as expected): npt-net's doc-test "can't find
  crate for npt_l1" (another lane rebuilt npt-l1 in the shared target mid-run). 22:14 run B: clippy bites met another
  copy's stub npt_proto (unresolved imports), test counted 298 and "extern location for npt_link does not exist": the
  race again (pgrep showed U4d's and U4c's cargo test runs and a workspace test in flight). Neither run is reported.
  Waiting in the background for 30 s with no other cargo/rustc, then the two runs.
- 22:30 the other lanes ran workspace tests back to back (never 30 s idle by 22:27). Final runs now by a scratchpad
  script (u4a-final-runs.sh, logs in the scratchpad's u4a-runs/): each run waits for 15 s with no cargo/rustc/rustdoc/
  clippy-driver, touches, runs check.mjs; a pair needs exit 0, all 10 steps, 296/321 and self 52; else it restarts.
- 22:29 SAVE (coordinator: usage limit close). State: ALL BUILD WORK DONE (src 11 files, tests wire/bites/catalogue/
  common, check.mjs + check.test.mjs; plants P1-P20 done and restored, sha256 = the baseline list above). Half-done:
  only the two final check.mjs runs: the script was started in the background at 22:28 (task b70332tdr), no run
  logged yet (other lanes busy). EXACT NEXT STEP if resumed: re-run
  `bash /tmp/claude-0/-home-user-Shamble-Frontlines/077a208e-2c05-50f2-8333-d6ac6effb7f2/scratchpad/u4a-final-runs.sh`
  (or, if the scratchpad is gone, touch crates/ and run `node check.mjs` twice when no other cargo runs; expect 10 of
  10, measured 296, bites 321, self 52), check sha256 against the baseline, then `cargo +1.99.1 fmt --all --check`,
  then write the report (items listed under "For the main session" and the carries below).
- Carries for the report: 1 met (layout()-built tables and mapping, sizes from each Field, VELOCITY != POSITION_XY
  tested); 2 met with one reading (literal multiples; at a non-wrapping end whose nearest multiple is outside the
  range the error is < one step, 3 integers; within half the step everywhere else, worst = half the step); 3 met
  (grep + self-test + rename refusal; P10); 4 met (unit test; P4); 5 met; 34 met by 4 listed fixes (STEPS pinned via a
  runner recorder; doc adds wasm32 for WASM_CRATES; bites-only clippy failure named; failedRun's first-error summary),
  seen live in run B's clippy line. Freeze: no seam change needed.
- FINAL RUNS (completed before the 22:35 cut; read back 06:32 UTC 2026-11-10): run 1 22:32:46-22:35:33, run 2
  22:35:43-22:37:52, each exit 0, "check: all 10 steps pass", test "measured build: 296 tests passed, 0 failed, 0
  reported a skip; bites build: 321 tests passed, 0 failed, 0 reported a skip", self 52 of 52, deps PASS, doc "host
  and wasm32", df 12G free. Logs: scratchpad u4a-runs/run1.txt, run2.txt. 06:32 all 18 files' sha256 = the baseline
  (unchanged since); `cargo +1.99.1 fmt --all --check` exit 0. S1 (U4s skeptic, coordinator 06:30): my tables do
  not separate owner-only fields (the soldier's whole layout goes in both "own state" and "remote soldiers"); what
  U4a would need is in the report. DONE (report next).
- For the main session (outside my files): (1) the catalogue's §6 item 3 note (plan §5 U4a, nit e), proposed words:
  "(The allocation half, refused before anything is allocated, is checked with PM-0001-34's counting allocator in U5:
  the advisor's Q8 ruling; U4 proves the size-first refusal.)"; (2) the catalogue's Input row: the codec's maxima are
  header 5 B, whole command 23 B, repeat at most 19 B, 161 B at the cap (input.rs's field table), in place of "about
  6 B" and "at most 20 B"; (3) proposals the catalogue may record: Leave's reasons (quit, error), ClientSettings'
  delay in 256ths of an interval, the peer-set encoding, SessionStatus's 16 B layout; (4) crates/proto/Cargo.toml's
  bites comment (entry 41) is still stale (no manifest edit in my lane).

## Fix pass, round 1
Fixer: general-purpose agent, opus, same worktree, 2026-11-10. Round 1's notes: u4ab-review-netcode.md (N1, N2, N6),
u4ab-review-security.md (SEC-U4a-3, -6, -7), u4ab-review-rust.md (F-A3, F-A6, F-A8), u4ab-review-test.md (T-A1, T-A6,
T-A7). Files: crates/proto src and tests only (no manifest, no check.mjs). Commit nothing. Every other finding the
reviewers listed is a follow-up or waits for U4d (S1): not touched.

### Plan
1. Baseline sha256 of the 15 proto src/test files (= the builder's baseline above).
2. Write the new tests against today's limits (for fix 1 the size check moves from `encode_into` into `Quant::encode`,
   same limit, so a test can build an exact-length snapshot uncapped; behaviour unchanged), run them: fix 1's, 2's and
   3's tests FAIL on today's code; fix 4's passes (the guard is there; its evidence is a plant, step 5).
   - Fix 1: msg.rs unit tests: a Snapshot of exactly 1,000 B encodes, 1,001 B TooLarge; a received 1,150 B Snapshot
     decodes, a 1,151 B one TooLarge; FullState (the only other type whose encoder can reach its limit) at 65,536 B
     encodes, 65,537 TooLarge, received at 65,536 + 4 decodes, + 1 TooLarge. Every other type's send and receive
     limits checked by reading against Limits::TYPES and the catalogue (recorded below).
   - Fix 2: an a6 case: Input with ack_groups 0 and ack_tick 5 decodes as UnknownField; the encoder refuses that value
     (NotCanonical); {0, 0} still round-trips.
   - Fix 3: tests/wire.rs: Debug of Join, Message::Join, every SessionControl variant and Message::SessionControl holds
     none of the token's, bind's or capability's bytes (decimal or hex), only their length.
   - Fix 4: hash.rs unit test: the finishing step fed 0 gives 1, any other value itself.
3. Apply fixes 1-5 (msg.rs send/receive limits; input.rs ack rule + encoder refusal + generator; Debug by hand for Join
   and SessionControl; hash.rs `finish`; input.rs `let _ = i;`), run the proto tests both builds: PASS.
4. Plant for fix 4: the guard removed from `finish` -> the new test FAILS; restored, sha256 checked.
5. fmt, clippy both builds (host and wasm32 for proto) as check.mjs runs them; then check.mjs twice in a row, all 10
   steps (expect measured 296 + new, bites 321 + new, self 52).
6. Record each file's new sha256 and each fix's test with its failing-then-passing evidence here. N2's text below.

### Progress
- 1 done: the 15 proto files + clippy.toml = the builder's baseline (bits 4a390bdd, control 390c1fa0, error ce66d7b5,
  hash 5a24907f, input 75b167fd, lib a8603264, limits 08e80997, msg 63677d4f, quant 0d41e906, snapshot ef9e6013, wrap
  9f2613c7, tests/bites 9d7cb2f8, catalogue 14313b7b, wire 8675723a, common 17bee55e, clippy.toml 30ab9b21).
- 2 written (not yet run): msg.rs size check moved from encode_into into Quant::encode, still content_limit (today's
  limits); msg.rs tests a3_a_snapshot_is_taken_up_to_1_150_b_and_sent_up_to_1_000_b and
  a3_a_full_state_is_sent_and_taken_up_to_65_536_b_after_its_length (uncapped() + exactly() search helpers); wire.rs
  a6 gains the ack case at its end; wire.rs debug_output_shows_a_secrets_length_never_its_bytes; hash.rs finish() +
  a8_the_finishing_step_turns_0_into_1_and_keeps_every_other_value. Runner: scratchpad u4a-fix-run.sh.
- 2 run 10:58 UTC on today's limits (`cargo test --locked -p npt-proto --no-fail-fast`, measured build; log scratchpad
  u4a-fix/step2-today.log; idle 156 s first, df 12 G): lib 12 pass 1 FAIL, catalogue 7 pass, wire 6 pass 2 FAIL.
  - fix 1 FAILED at msg.rs:603 `encode(1,001 B snapshot)`: left Ok(()), right Err(TooLarge); the lines before it held
    (1,001 B and 1,150 B decode, 1,151 B TooLarge, 1,000 B encodes). The full-state test passed (no defect there).
  - fix 2 FAILED at wire.rs:548: Input { ack_tick: 5, ack_groups: 0, .. } decoded Ok where UnknownField.
  - fix 3 FAILED at wire.rs:600: `Join { token: [167, 167, ...], bind: [179, ...], ..}`.
  - fix 4's test passed (the guard is in today's code; its evidence is the plant at step 4).
- 3 fixes applied: msg.rs content_limit split into send_limit (encode; the row's limit, Snapshot 1,000) and
  receive_limit (decode and channel_max; Snapshot 1,150), after_length for the control stream's 2 B, module text and
  error.rs/limits.rs docs; input.rs: decoder refuses ack_groups 0 with ack_tick != 0 as UnknownField (strict, as
  Snapshot's known()), encoder refuses it as NotCanonical, `for _ in 0..count` and the `let _ = i;` gone; common's
  input() generator gives tick 0 when no group (same RNG draws); Join and SessionControl Debug by hand through
  control.rs's pub(crate) Redacted ("[16 B]"); wire.rs a2's re-encode rule through as_sent() (a decoded Snapshot above
  1,000 B must be refused by encode, else the same bytes: the rule stays true under the asymmetry). fmt clean.
- 3 run 11:02 UTC (log u4a-fix/step3-fixed.log; idle 107 s, df 12 G): measured lib 13, catalogue 7, wire 8; bites
  lib 16, bites 5, catalogue 7, wire 8; 0 failed. Proto +4 tests per build (3 lib, 1 wire): expect check.mjs 300/325.
- 4 plants (in place, one at a time, restored, sha256 checked):
  - PF4 hash.rs finish() returns h (the guard gone): lib a8_the_finishing_step... FAILED at hash.rs:112 (left 0,
    right 1), while wire's 8 tests, a8 among them, passed (the gap F-A6/T-A7 named); restored, hash.rs d3a9f95b =
    before. Log u4a-fix/plant-hash-guard.log.
  - PF2 input.rs the encoder's NotCanonical check removed: wire a6 FAILED at wire.rs:556 (left Ok(()), right
    Err(NotCanonical)); restored, input.rs a94e669a = before. Log u4a-fix/plant-ack-encoder.log.
