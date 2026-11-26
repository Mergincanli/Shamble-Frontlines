# U7 review round, test seat (test-engineer, 2026-11-26)
Packet: u7-round-packet.md. Seat: is each claim proven (C14's +20, plants, T5 (ii)'s marks, web tests, e2e drivers'
honesty, evidence tree = reviewed tree).

## Plan
1. Read the brief (u7-brief.md rev 1.1), the conditions C1-C14 (advisor-u7-brief.md), both builders' notes. DONE.
2. Evidence: end-1.log, t5ii-run-3.log + run folder summary.json, r3-room-2.log, plant logs. DONE.
3. Tie the evidence tree to the patch (diff/untracked hashes vs the patch). DONE.
4. Read the diff: Rust tests (clock.rs, sent.rs, engine.rs, script.rs), check.mjs web step, web tests, fakes, e2e. DONE.
5. Claim / proof / gap table; findings with file:line; what I did not read. (below, after the notes)

## Notes (one line per finding, as I go)
- N1 Tree tie, end-1: patch head lines 1-750 (the 10 tracked files) sha256 = ffdba3ef...d8c4 = end-1's `diff`; the 32
  new files read from the worktree (patch's list, C-sorted, `sha256sum` lines, relative paths) hash to 89aa34ae...5409
  = end-1's `untracked`; each of the 32 new files' content in the patch equals the worktree file (32 of 32 same). So
  end-1 ran on exactly base 9177fb5 + this patch. Patch size 243,393 B (packet says 243,353 B: a packet typo).
- N2 Tree tie, T5 (ii) run 3: HEAD 7e2d6f3, diff ffdba3ef (same tracked), untracked 08a7f9f3 (not 89aa34ae); the dist
  was built at 19:35 on untracked 8b27b164 (t5ii-dist.log:3). `git diff --stat 7e2d6f3 9177fb5 -- spikes/`: only
  Cargo.lock +2 and client-wasm/Cargo.toml +2. Between dist and end-1 the builder's own untracked files changed too
  (engine.rs's room tests, t5ii.mjs's `fail`, probe-worker.js), which the builder's equivalence (notes :110) omits.
  The room tests sit inside engine.rs's `#[cfg(test)] mod tests` (patch :1047-1583), so the Wasm is likely unchanged:
  likely, not shown.
- N3 C14: 20 `#[test]` added, 0 removed in the patch (clock.rs 7, sent.rs 2, engine.rs 10, script.rs 1); 881-861 =
  978-958 = 20; the base's counts are 7e2d6f3's and 9177fb5 changes no test (N2). Test step 1,053.9 s vs base
  1,055.5-1,082.5 s (u7-0b before.log:5, step5-after.log:181): no 1.5x stop.
- N4 Plants (R): each log's tree hash differs from the clean run's and names exactly the brief's test(s): P1, P2, P3,
  P4 (+ sent.rs), P5, P6, P7 (panic), P18, P17 (E0080 at scenario.rs:63), room-a (3 tests), room-b (3 tests). T-half
  plants P8-P16, P19: counts in notes only (no kept log); web sources' sha256s today equal the plant-time "before"
  hashes (lib.ts fdf3b39c, worker.ts 8906558a, main.ts 74b7dda4, hooks.ts 1856f8e2), so they cover the reviewed code.
- N5 FINDING (Medium) C-U7-3's two browser links are untested. (a) engine.rs `Core::step` (patch :959-961) calls
  `self.endpoint.sent(prev_send_ns)`; no test fails if the call is deleted or given `now_ns`: the only callers with a
  non-zero prev_send are r3_the_first_step (asserts an empty outbox) and RoomHost::wake (prev = the last wake's time =
  the ping's wire stamp, so t0 = max(at, wire) = wire either way). (b) worker.test.mjs:84 asserts prevSendNs[i] =
  nowNs[i-1] under a FakeClock that never moves inside a wake, so "stamped after the sends" (worker.ts:289-291) cannot
  be told from "stamped before". Only the endpoint's `sent` is proven (clock.rs, sent.rs, P4).
- N6 FINDING (Medium) Mark 9: NOT-RUN is right (room.jsonl keys: event/second/header/footer only, no per-slot refusal),
  but the stand-in table is weaker than it reads: `refused` is the client's own decoder count (client/src/lib.rs:480-484),
  not the room's kinds 1-6, 8, 10; `last_run_command` near `commands` shows commands ran, not that none were refused.
  The in-process room legs (engine.rs RoomHost::run, patch :1460-1480) wake the Core exactly at next_due_ns over a
  zero-latency Loopback: no late wake, no catch-up burst, which is the browser's risk for kind 10 (buffer_full) and kind
  1. So "no refusal but the sim's own" is proven only under exact wakes.
- N7 FINDING (Medium) Mark 7's CSP half: the probe shows the Worker fires `securitypolicyviolation`
  (summary.json console_probe.status "refused in 3 ms; violation connect-src"; probe-worker.js:5-7), so a counter is
  at hand; the page reads none, so the Worker half (where both transports run) stays unread, and the page half has no
  positive control (the probe refuses only in the Worker). Same gap in C8 (summary c8.csp_lines from page console only).
- N8 FINDING (Medium) T5i-1 is systematic, not one-off: run 3's ws_accept shows accepted 1 + handshake_failed 1 in
  every WebSocket load (2-smoke-ws, 5-fallback, 8b, C8) and 0 in every WebTransport-only load; T5 (i) B the same. Root
  cause unknown; the known-open list carries it as a one-load note.
- N9 FINDING (Medium) The web step passes on an empty selection: check.mjs webStep runs `node --test <web>/test/*.test.mjs`
  and fails only on a non-zero exit; Node 22.23.4 here, `node --test '/nonexistent-dir-xyz/test/*.test.mjs'`: "# tests
  0", exit 0. A moved or renamed test folder gives "0 of 0 tests passed" and PASS: C7's "fails, never skips" is not
  held for that case; check.test.mjs's pin has no zero-count case.
- N10 Mark 8b (T5ii-B): agree with the builder; not worse than written. The contrast 1.60 s (mark 5, page timer) vs
  4.06 s (8b, Chromium's QUIC idle timeout) shows the bite removes the page's timer; dist.test.mjs:93-108 proves the
  page half (no fallback in 10 s). For R8: -29 catches this bite only when QUIC is silently dropped (an ICMP-refused
  port fails WebTransport at once, so the page falls back fast with or without the bite). Low note.
- N11 Lows seen: dist.test.mjs:19-28 scans stand-in wasm-bindgen files, build.mjs scans the real Wasm for home paths
  only (I grepped the 19:35 real dist: no PM-0001-, no innerHTML); clock.rs's u64::MAX case (patch :569-572) would hang,
  not fail, if the loop came back (libtest has no timeout); no test of "joined, no clock sample: no attempt.ok 1";
  PENDING_MAX, UNI_MAX, WS_BUFFER_MAX (worker.ts:53-55, :213, :261-271) untested and the fakes cannot backpressure
  (fakes.mjs:106, :54); the createWritable branch (worker.ts:182-183) runs nowhere (Chromium 153 lacks it, the fake
  too); t5ii.mjs:35 defaults `commit` to 7e2d6f3 (summary.json's commit is a constant, not read); C8's WebTransport page
  runs with ignoreHTTPSErrors (t5ii.mjs:192), so it does not isolate the pin (loads 1, 3, 4 do); C2's JoinAccepted
  equalities (patch :1517-1525) echo RoomHost::new's own with_hashes (patch :1373-1374), both halves: the evidence is the
  Decoder's acceptance (room-b bites) and T5 (ii)'s accepted joins against the binary; the npm ci branch of webStep is
  recorder-tested only; client-wasm's clippy.toml equals client's below its header but is not in CLIPPY_TWINS
  (check.mjs:995); r3_the_limits_are_the_wires pins 65,541 as FULL_STATE + 5, not U6b's FULL_STATE_MAX; notes :104 says
  "13 steps", end-1.log says 12.
