# U7 review round: the netcode seat (netcode-architect, 2026-11-26)
Packet: u7-round-packet.md. Diff: wip/u7-build-rs.patch (42 files, base 9177fb5). Brief u7-brief.md rev 1.1 + C1-C14
(advisor-u7-brief.md:110-247; C6 as reworded in advisor-main-66-head.md section 1). Read only, except this file.
No Edit tool in this seat: this file is rewritten whole by Write. File lines below are the applied tree's (the
worktree's), computed from the patch's hunk heads (new files: file line = patch line - the hunk head's line).

## VERDICT: APPROVE WITH FINDINGS
0 Critical, 0 High, 1 Medium, 7 Low. C-U7-3, C-U7-4 and C2, C3, C5, C6 and C9 are met in code. The one Medium is
missing evidence: no test shows the browser path's C-U7-3 wiring, and no test runs the Wasm core with any delay.

| Id | Sev | File:line | What is wrong | Smallest fix |
|---|---|---|---|---|
| U7R-NC-1 | Medium | crates/client-wasm/src/engine.rs:207-209 (the `sent(prev_send_ns)` call); :690, :698 (RoomHost's wake) | C-U7-3 reaches the endpoint (clock.rs and client/tests/sent.rs test it with an 80 ms path and a 5 ms send lag), but no test shows the Wasm `Core` passes the host's send time on. RoomHost passes `prev_send_ns` = the last wake's `now`, which equals that wake's ping wire stamp, so `t0 = max(at, wire)` is unchanged. Deleting engine.rs:207-209 fails no test (the P4 plant was clock-level only). Also, no U7 Rust test runs the Core with any latency: the Loopback has no delay (link/src/lib.rs:175), and T5 (ii) ran at P0 only. CLAUDE.md's netcode evidence asks for injected latency. | In RoomHost: stamp every send with `now + 40 ms` (the Loopback delivers at the send stamp: link/src/lib.rs:175-176, :210-214), and give the wake's sends a 5 ms lag (`prev_send_ns = now + 5 ms`). Then assert Playing, no refusal, and the newest clock sample's RTT = 80 ms exactly. Plant: engine.rs:207-209 removed must fail that test (it reads 85 ms). Jitter and loss stay with the endpoint's lab suites, since the Core adds no logic that depends on them; say so in the report. |
| U7R-NC-2 | Low | crates/client-wasm/src/script.rs:38-42; engine.rs:219 | The press counters come from the Core's step count. After a Worker stall, catch-up runs `LOOP_CATCHUP` (7) steps a wake at one `now` (client/src/lib.rs:790-792: the step grid starts at the first input), so steps advance about 58 per 33 ms (4 ms nested-timer clamp) while command ticks advance about 1. Room kind 5 allows at most one edge per tick covered (room/src/input.rs:286-297). So after a stall of 2 s or more, consecutive commands one tick apart can carry two fire edges: a PressEdge refusal against an honest page. That breaks U7's C-U6-19 line ("fires and reloads honestly"). It would not show in R1, since the room log has no per-slot refusals (T5ii-A). No test has a stall. | Count presses from the commands issued (`script::command(self.commands, newest)`, periods in commands; there are about 30 a second): at most one edge per command, and each command covers at least one tick. Add one RoomHost case with a 3 s gap between wakes: no refusal. |
| U7R-NC-3 | Low | web/e2e/t5ii.mjs:127-142, :241-243, :281 (mark 8b); T5ii-B | T5ii-B is accurate as far as it goes, but it misses a condition. Under the bite, the fallback happens at Chromium's own QUIC idle timeout ("Timeout:4s", run 3: 4.06 s). That is past -29's 3 s, so -29's judge catches the bite only while (a) blocked UDP is silent (DROP, like T5's bound socket) and (b) the browser's own handshake timeout is above 3 s. With a REJECT (ICMP port unreachable), Chrome fails at once: the measured leg and the bite both fall back fast, and the bite cannot bite. Firefox's and Safari's timeouts were not seen. | Restate mark 8b as "a fallback later than 3 s, or none in 10 s" (the builder's lean). Add to R8's bite plan, and to whoever writes `npt-ctl quic off` (U8's fallback.mjs calls it, lib.mjs:400-403 in wip/u8-build.patch), that the block must DROP, never REJECT. Record each browser's own timeout when R8 runs the bite. |
| U7R-NC-4 | Low | web/e2e/t5ii.mjs:335-340 (`fail`), :177 (npt-host spawned with stdin ignored); web/e2e/t5i.mjs:144, :205 | The known-open item, weighed. On an unhandled rejection, `fail` exits without the finallys. Rooms still end, because npt-server exits when stdin closes (server/src/main.rs:934-943) and the parent's pipe closes. But npt-host keeps 127.0.0.1:31000, and the dead room's tokens file stays under .gate/. t5i.mjs prints `e.message` raw, but its only fragment is the synthetic non-token (t5i.mjs:127), so no real token is at risk. No worse than written. | `fail`: SIGKILL every child the driver started (keep a set; `kill` sends synchronously) and `rmSync` the current load's tokens file, then exit. t5i.mjs's two lines go through the same `clean()`. |
| U7R-NC-5 | Low | web/src/main.ts:58-67 | The join token stays in `location.hash` (address bar, session history) after it is parsed. A reload or a restored tab sends a second `Join` with the same token. The roster refuses it (`BindMismatch`, room/src/roster.rs:111-114: the page makes a fresh bind per load), so nothing rejoins (C-U6-19 holds). But the room logs a refused join for that slot, and the token lingers in local history. | After a good parse: `history.replaceState(null, "", location.pathname + location.search)` before the Worker starts. A reload then shows "Bad link: no #t= fragment" with no network contact. Add a Node test on a pure helper. For Stop B's security item (C12: the token's handling). |
| U7R-NC-6 | Low | web/src/lib.ts:358-368 (`Recorder.datagram`, `second`); web/src/worker.ts:328-332, :335 (`end`) | -14's browser check is `sum == 0` over `second.v.recv_over_max_count` (marks.json:3583-3597). A datagram counted in the session's last partial second is never written: no `second` record at `end()` or before U8's read. That gap allows a false pass. Separately, the 1 s timer re-arms after each fire, so it drifts, and every few hundred seconds one `sec` is skipped. Its count merges into the next (harmless for a sum). | Write the partial second at `end()`, and arm `second` on the Worker clock's grid (delay = 1,000 - elapsed % 1,000). One Node test for each. |
| U7R-NC-7 | Low | web/src/worker.ts (no `securitypolicyviolation` listener); T5ii-D | T5ii-D is accurate. Both transports run in the Worker, and Chromium's CSP report line from a Worker never reaches Playwright's console. Every load reaching Playing shows that every connect the page needs was allowed, but an extra refused connect would go unseen. Not worse than written. | The Worker counts `securitypolicyviolation` events into a `page_csp_violations` counter (about 5 lines and one test), so mark 7's Worker half becomes readable. That counter would also cover any later "0 console errors" reading. |
| U7R-NC-8 | Low | web/e2e/t5ii.mjs:245-247 (the summary's `refused`), :192-193 (C8's pages with `ignore: true`) | Two wording traps for U7's report. (a) The `refused` column under mark 9 is the client endpoint's decode refusals (client/src/lib.rs:480-482), not the room's. Read as mark 9, it would claim something nobody measured. (b) C8's WebTransport leg ran under `ignoreHTTPSErrors` (the page origin uses the test certificate), so it shows the real host, its CSP and the room accepting. It does not show the pin admitting the certificate. Marks 1 and 4 (no ignore; the XOR-0xff hash refused) carry the pin. | Label the column "client decode refusals". Say in the report that the pin's evidence is marks 1 and 4, not C8. |

## The known-open items, weighed
- **T5ii-A (mark 9 not readable):** accurate (tick_loop.rs has no per-slot refusal or forged count: grep finds none).
  I agree with NOT-RUN from the log. The evidence that the press script draws no refusal is R3's in-process test
  (npt-room behind the real Decoder, kinds 1-6, 8 and 10 all 0, and every slot refusal counter 0 over 3,000 steps;
  r3-room-2.log 11 of 11, plants room-a and room-b caught). The live counters show the room ran 97-99 % of the page's
  commands (148/144 and so on: the in-flight tail). See NC-8 (a) for the `refused` label.
- **T5ii-B (8b):** accurate, plus NC-3's missing condition (DROP, never REJECT; the browser's own timeout above 3 s).
- **T5ii-C (outbound attempt):** accurate, and not worse. The fix is in place on both browsers (t5ii.mjs:188-189,
  :271), and run 3 shows every C8 response from 127.0.0.1:31000. One addition: the CONNECT named the project's real
  host to the session's egress proxy, which CLAUDE.md's privacy rule asks us to avoid. It was inadvertent and has been
  recorded; the flag prevents it. t5i.mjs's browser lacks the flag but loads only 127.0.0.1, which Chromium never
  proxies.
- **T5ii-D (mark 7's Worker half):** accurate; NC-7.
- **T5i-1 (an extra failed WebSocket handshake):** the builder's notes put it in C8's room log too. No mark reads
  `ws_accept.handshake_failed_count` or the wt one (marks.json: 0 hits; defined at SERIES.md:887, :894), so it is
  harmless to R1. The cause is still unknown. **T5i-2** (Join-deadline closes not in `deadline_closed_count`) is a
  server observability note for U6b's owner, not U7's. **T5i-3** is superseded by T5ii-D.
- **The driver's token print:** fixed (`clean()` on every error and console text). The `fail` path is NC-4.
- **C2's mode-hash half:** accurate. The room's defaults and MODE_HASH are all zero, so only the content-hash half
  can fail. No client reads `mode_hash` (advisor-u7-brief.md N2), so nothing more belongs in U7. The binary's
  content-hash equality is shown end to end by T5 (ii): its Decoder would reject a Join with another hash.
- **T5 (ii) not rerun on 9177fb5:** checked from the main folder. `git diff --stat 7e2d6f3 9177fb5 -- spikes/` shows
  only Cargo.lock +2 and crates/client-wasm/Cargo.toml +2 (dev edges). So npt-server's and npt-host's code is
  identical, and the transport legs' evidence carries. Whether the dist built at 19:35 equals the reviewed tree's
  non-test code (run 3's untracked hash 08a7f9f3 against end-1's 89aa34ae) is the test seat's question.
- **The shell-loop slip:** a process matter, read only, with no effect on any evidence. Not weighed further here.

## Checked and sound (no finding)
- C-U7-4 (clock.rs): the offset clamped at both ends in `pong` and `mean`; the next ping in closed form, saturating;
  `period_ns` `.max(1)`; `new`, `period_ns`, `unknown_pongs`, `next_tick_at` and `one_way_hi_ns` made `pub(crate)`;
  `ping_every_ns` removed.
- C-U7-3 (clock.rs `sent`): each unstamped ping is stamped once, `t0 = max(at, wire)`, and matching still reads the
  wire stamp. The Core's order is take(inbox), then `sent`, then the steps that read the pongs (engine.rs:205-237), so
  the send time is in place before the pong it times. The Worker stamps after each wake's sends (worker.ts:283-299,
  tested at the page level).
- C3: `HONEST_MAX_LEAD` replaces every literal cap, including bots/src/load.rs:45 and server/tests/carry.rs:129,
  which are outside the lane's file list (a departure for the report). Lab's assert names the constant; P17 caught.
- C2 and C5: version, content hash and tags come from their one homes through `npt_client::wire`. That module's 9
  re-exports are a new pub surface not in the brief's list, recorded by the builder (a departure for the report; it
  avoids a manifest change). The inbox and outbox layouts match on both sides (engine.rs `record` and `push_record`;
  lib.ts `packInbox` and `unpackOutbox`), and the golden file is read by both.
- C6: the room tests host `Room<World>` behind npt-server's real `Decoder`, with no hand-written pump.
- C9: the Join goes on one transport only. An abandoned WebTransport is closed, and closed again if it turns ready
  later (lib.ts `connect`; the worker test at 1.6 s).
- The CSP: serve.mjs's CSP equals plan.md:1123-1126 word for word, plus the two 127.0.0.1 sources. C8 under
  npt-host's compiled CSP and the real name reached Playing on both transports.
- The framing caps: control 256 B with its length. The full state's 65,541 follows C5, one above the decoder's
  65,540; that is harmless, because the room never sends more.
- Every Worker queue is bounded (inbox 512, Core pending 512, 64 writes in flight, 4 uni streams, 1 MiB WebSocket
  buffer, 4,096 page records). No clock and no hash map in the Rust (clippy.toml).
- A FullState that arrives before JoinAccepted is taken without a slot (client/src/lib.rs:826), so the independent
  order of QUIC streams cannot stall Playing.
- The hash file's `validity_ns` (13 days in ns, server/src/cert.rs:25) is a safe integer for `parseHashFile`.

## What I did not read
build.mjs in full; check.test.mjs; the Node tests other than worker.test.mjs's C-U7-3 and C9 parts and connect.test.mjs's
C9 line; fakes.mjs and fake-core.mjs (fidelity is the test seat's); contact and probe pages; the T5 (ii) run folder's
result.json files (I read the run log, the builder's summary and r3-room-2.log only); end-1.log and the plants' logs
(the test seat's); U8's current snapshot beyond fallback.mjs, `quic()` and mds36's wait. No build, test or browser run
here (read only).

DONE
