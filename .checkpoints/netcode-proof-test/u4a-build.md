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
- For the main session (outside my files): (1) the catalogue's §6 item 3 note (plan §5 U4a, nit e), proposed words:
  "(The allocation half, refused before anything is allocated, is checked with PM-0001-34's counting allocator in U5:
  the advisor's Q8 ruling; U4 proves the size-first refusal.)"; (2) the catalogue's Input row: the codec's maxima are
  header 5 B, whole command 23 B, repeat at most 19 B, 161 B at the cap (input.rs's field table), in place of "about
  6 B" and "at most 20 B"; (3) proposals the catalogue may record: Leave's reasons (quit, error), ClientSettings'
  delay in 256ths of an interval, the peer-set encoding, SessionStatus's 16 B layout; (4) crates/proto/Cargo.toml's
  bites comment (entry 41) is still stale (no manifest edit in my lane).
