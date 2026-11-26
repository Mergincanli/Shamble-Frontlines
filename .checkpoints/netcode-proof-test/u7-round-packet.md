# U7's review round: the packet (2026-11-26 04:5x UTC, the main session)
One round (NPT-TRIM): netcode-architect and test-engineer, at once; no security seat (C12 of advisor-u7-brief.md).

## What U7 is
The browser client for the netcode proof test: a TypeScript page and Worker (spikes/netcode-proof/web/) over a Rust core
compiled to Wasm (crates/client, crates/client-wasm). Its brief: u7-brief.md rev 1.1 with the advisor's conditions C1-C14
(advisor-u7-brief.md, binding); the Rust half's build brief: launch-0850.md section 3 (brief 3). C6 was reworded by the
advisor at 23:5x UTC (advisor-main-66-head.md section 1: two more in-repo dev-dependencies, npt-link and npt-proto).

## What to read
- The diff: .checkpoints/netcode-proof-test/wip/u7-build-rs.patch (243,353 B, 42 files; base 9177fb5, the lane's head
  when it was taken). The same tree, applied: .claude/worktrees/agent-a9873cdba07a3219e (read only; never cd into it,
  never run git there).
- The builder's notes: wip/u7-build-rs-notes-0030.md (the Rust half) and wip/u7-t-build-notes-2050.md (the TypeScript
  half); the brief and conditions above.
- The evidence: .gate/u7/rs/end-1.log (`node check.mjs`, 00:05-00:26 UTC on 9177fb5: all 12 steps pass; test 881 and
  978, +20 each against 861 and 958, C14's floor; web 56 of 56; self 80 of 80; wasm 27,000 ticks, 0 mismatching);
  .gate/u7/rs/t5ii-run-3.log and the run folder .claude/worktrees/agent-a9873cdba07a3219e/.gate/u7/t5ii/2026-11-25T19-53-08-358Z
  (T5 (ii) run 3 on 7e2d6f3: marks 1-6 and 8a met); r3-room-2.log (R3's two room tests, 11 of 11; plants room-a and
  room-b caught).

## Known open (weigh each; say if it is wrong, missing or worse than written)
- T5ii-A (mark 9 not readable), T5ii-B (8b not as written), T5ii-C (one outbound attempt went to the session's proxy, no
  token sent; fixed by --no-proxy-server), T5ii-D (mark 7's Worker half not readable); T5i-1 to -3 (the T-half's notes).
- The T5 (ii) driver: a run-1 error printed a dead join token into a local log (blanked); the driver now cuts every
  printed line at '#'; its `fail` path exits without the finallys (a host process or a tokens file could stay behind).
  t5i.mjs prints e.message raw (lines 144, 205; a synthetic fragment only).
- C2's mode-hash half cannot fail today (the room's defaults and MODE_HASH are all zero): only the content-hash half is
  evidence.
- T5 (ii) was not rerun on 9177fb5 (run 3 used 7e2d6f3's binaries and dist; the builder infers equivalence from the
  fast-forward's file list).
- The builder waited on logs with shell loops from 19:4x to 00:16 UTC (read only), against brief 3's rule.

## Each seat
- netcode-architect: the client core and its protocol use (crates/client, client-wasm engine.rs and script.rs), the
  Worker and page boundary, R3's two room tests against npt-room and npt-server's real Decoder, T5 (ii)'s transport legs
  (WebTransport, the WebSocket fallback), the URL and join-token handling, and the known-open items above.
- test-engineer: is each claim proven: the +20 tests against C14's floor, the plants, T5 (ii)'s marks and what "not
  readable" leaves unproven, the web tests, the e2e drivers' honesty, and whether the evidence's tree is the reviewed one.

## Rules and output
Read only, except your one findings file. Severity: CLAUDE.md's one scale (Critical, High, Medium, Low). Write
.checkpoints/netcode-proof-test/u7-review-netcode.md or u7-review-test.md: a verdict line (APPROVE, APPROVE WITH
FINDINGS or REVISE), then one row per finding (id U7R-NC-n or U7R-TE-n, severity, file:line, what is wrong, the smallest
fix), then what you did not read. Return a summary of at most 10 lines.
