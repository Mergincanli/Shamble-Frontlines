# rust-reviewer: U3C-HELD-1 review (round 1, final diff)

Branch claude/netcode-proof-test-plan-hcky5g, PR #40 (draft). Read only; no builds, no cargo. No Edit tool here:
this file is rewritten whole with Write.

## Plan
1. Read the diff (`git diff -- spikes/netcode-proof`) and the u3c-plan.md correction (`git log -p -3`).
2. Read the plan draft, the advisor review (C1-C10, HELD-Q1-Q12) in full, the design and evidence notes.
3. Read delay.rs and delay_tests.rs around the changed items in full context: cfgs of every new item and statement,
   Life's new fields and Default, the check's signature change and every caller, lock and borrow order at the two
   growth sites, integer arithmetic, names and doc comments.
4. Read build.md and the logs: light.log (fmt, clippy, doc), l1 / l1-1-94, e3a / e3b / e3c, p1-p5, e5.
5. Determinism and build-config differences (measured vs bites, wasm32).
6. Findings with file:line, severity, fix; verdict.

## Progress (all steps done)
- Tree under review = tested tree: `git diff HEAD -- .` in the spike hashes a7199f7a… (as the light.log, e3a, e3b,
  e3c and e5 headers); files 3dbbccf7… / 8f8c0f9a… / 1dd67e2f… = build.md:52-54's fixed hashes. HEAD moved
  801d284 -> 43a6f1f by checkpoint commits only (`git diff --stat 801d284 HEAD -- spikes/` empty). D-1 gone
  (`grep -rn "over_waiting\|over_at_write" crates/` exit 1).
- cfg audit: every changed non-doc line of delay.rs sits under #[cfg(test)]: Life (:377), held_now's call (:523-524),
  check_held_bound (:537), check_held_falls (:557), model_write's call (:634-635). Non-test token changes: doc
  comments only (module doc :46-53, model_write's doc :617-618). delay_tests.rs is `#[cfg(test)] mod tests`
  (delay.rs:1915-1917). No cfg(feature) in delay.rs; delay_tests.rs's only one (:558) is an unrelated wt test.
- Life: `#[derive(Debug, Clone, Copy, Default)]` keeps Copy (two [u64; 2] more); Default gives held_at_write [0, 0],
  the right start (no bytes held before a write); built only by Life::default (:1073); life() copies under the lock
  (:1297-1299). Conn grows 32 B in test builds only; ws_conn_fixed_bytes counts buffers only (:1077-1078), so its
  pinned 309,583 (delay_tests.rs:2709) is untouched (E3c's whole binary passed).
- Signature: check_held_bound(&mut self) -> (&mut self, d: usize); its only caller is model_write (:635), whose two
  callers pass DOWN (:1615) and UP (:830): no index can leave [_; 2]. check_held_falls's only caller is held_now (:524).
- Growth sites and locks: down poll_write :1597 extend -> :1615 model_write under one guard `c` (current(), :1580;
  Stepped's wakes fire after the guard drops); up move_up :829 extend -> :830 model_write on `&mut Conn` under the
  pump's guard (:1827, scan_up :1838 after step_to :1837 with the same t). No held_now between either pair, no early
  return between them; one mutex per connection, no nested lock added; check_held_bound touches no pool or atomics.
  The only other `bytes[` writes are the two drains (:1526, :1771) and a test's in-place byte flip
  (delay_tests.rs:1796): held_grew can count only a real growth outside model_write, never a timing.
- Arithmetic: `model_end[d] - outstanding(d)` and `r.model_end - size` moved unchanged (plain sub, safe: write_seq ==
  model_end, model_end cumulative), now run once per write instead of twice per sample; `budget + prefix` <=
  65,536 + 16,406; `saturating_sub` kept for the excess; counters are u64 `+= 1`. TcpConfig::default() is a
  Copy struct of constants (tcp_model.rs:224-235).
- held_beyond_budget now updates at writes only; since held only falls between writes, its maximum is the same as
  over all samples: e3a prints 14,326 B (588 runs) or 14,348 B (12 runs), l1-1-94.log:188 printed 14,326 B. So
  G-NC2's loop exit (`beyond > 0`, delay_tests.rs:2285-2288) is not made harder: no new hang-guard risk.
- Stalled-client test: post-hold write of RECORD_MAX (delay_tests.rs:780) is Pending by arithmetic, not timing
  (8 records, 65,360 + 8 x 22 = 65,536 model bytes released, S took 1 KB, so down_free = 0 under quiet P2). The bound
  asserts run before the Pending assert, so a P1-type overflow names itself. owed = 16,384 after it is read by
  nothing in this test (owed's readers: :760, :1231, :1242). `&[0x82; RECORD_MAX]` is a promoted constant, never held
  across an await.
- Determinism/builds: pure observations, no feedback into control flow, no HashMap, no clock read added. Measured and
  bites builds compile the same code (no feature gate on the path; npt-l1's bites touches only L1Net's model). wasm32:
  npt-net is not a wasm32 crate (check.mjs:36: the wasm32 crates are the leaf crates, free of transport crates and
  async runtimes; delay.rs needs tokio), so the doc step's wasm32 leg does not cover it and nothing here can differ there.
- Logs read: light.log:4-8 PASS (fmt; clippy both builds, --all-targets, -D warnings; doc host+wasm32) on diff
  a7199f7a; l1.log:202 run 94 exit 101 and l1-1-94.log:188-193 (D-1 [1, 0] / [0, 0], panicked at :2302 = HEAD's :2300);
  e3a 600/600, e3b 600/600, e3c 20/20 (0 non-zero exits each, "end … exit 0"); p1-p5 as build.md:73-95 (p1-1-1.log
  :189-192 and :239-242; p2 :190-193 [121, 0]; p3 :189-192 [134, 0], :239-242 [1144, 0]; p4 :189-192 [137, 129], :238-239
  hang guard; p5 :189-192 [0, 172]); e5 98c3967e… equal. The binary name ed8673 stays while its sha256 changes with
  each source (04c76dc4…, 2cf1e962…, 6cb02f1e…, …): Cargo's -C metadata hash follows package, features and profile,
  not source, as test-engineer F2 says.
- Widths: the longest added line is 120 columns, the files' doc convention; rustfmt has no rustfmt.toml (defaults;
  comments not wrapped) and its check passed.
- Later steps: C5's 552 / 632 is right (no test added or removed; e3c's 169 + 12 ignored equals L1's). E3 looped the
  bites binary only; the measured binary first runs the changed tests in E6 and CI. No timing dependence remains by
  construction, so that is no gap.

## Findings (a line after each)
- RR-UH1-1 Low: delay_tests.rs:788 `assert!(matches!(r, Poll::Pending));` has no message. If a later change admits a
  write that stays inside the bound (for example 1,000 B free after the hold), the failure shows neither r nor how
  much was admitted. Fix (optional; with the next touch of the file, never a push alone):
  `assert!(matches!(r, Poll::Pending), "the post-hold whole record was admitted: {r:?}");`.
- RR-UH1-2 Low: marks/SERIES.md:642 (and the same up phrase in :613): the down part carries the bound to the row's own
  reading ("and only falling between writes, so below 64 KB + 16,406 B"), the up part gives only "at most, at each
  write, …" with no bridge to the second's-end reading it describes, and its "at each write" also covers the
  scanner's buffer, which is bounded at all times. The module doc covers it ("Up likewise", delay.rs:51). Fix
  (optional, a STATUS follow-up or the next touch; the text is the design notes' as C1 and HELD-Q5 require): "up: …
  at most, at each write, 64 KB plus the acknowledged prefix of the then front unreleased record (below 549 B, or the
  request's 8,214 B), and only falling between writes, plus one frame (527 B) or the 8,192 B request cap".

## Verdict
APPROVE WITH FINDINGS: two Lows, neither a push on its own. The Rust is sound: test-only items and statements,
non-test changes doc text only, one lock at both growth sites, no new arithmetic risk, fmt/clippy/doc PASS on the
exact tree, evidence holds.
