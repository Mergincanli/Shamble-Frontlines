# Skeptic, round 2: R2-D2 (Medium; merges SA2-1)

Plan: read the deduped table and the reviewers' texts; read the report lines named (Risks bullet :213-215, OVERFLOW-ALL
entry :299-302); read each cited ADR line (0001 part 11 :493-503, failure table :768-769, OQ-0001-18, PM-0001-26,
PM-0001-59; 0004 part 20; 0002 part 4 :247-278) and check their status (Proposed vs accepted); reproduce the
silent-wrap example (100 - 150 as u32 = 4,294,967,246) in a scratch crate under the old profile blocks in release, in
/home/user/Shamble-Frontlines/.gate/skeptic-r2-R2-D2/; judge holds/severity; write the exact replacement texts.

## Checks
- (a1) Status lines read: ADR 0001 :3 "Proposed" (:7 signed off only "as the basis for the spikes ... not yet accepted");
  ADR 0004 :3 "Proposed" (same :7); ADR 0002 :3 Proposed, part 1 accepted only (part 4 is not accepted).
- (a2) ADR 0001 part 11 :493-503 read: unwind, connection work isolated from the tick; bounded channels, no lock the tick
  unwraps; "A panic on the tick thread is fatal to the room" (catch_unwind at the loop top, report, RoomEnded, match
  record crashed, exit non-zero); OQ-0001-18 answered by the Advisor 2026-11-03 ("end the match, as proposed, and keep
  the crash and match records"). Matches the finding.
- (a3) Failure table :768 (connection panic: that connection closes, tick and other connections go on; R2-ROOM-REC2
  mark 6) and :769 (tick panic: the match ends; PM-0001-59). Also :771 (a trap in the client's Wasm sim: the Worker
  restarts and asks for FullState with Resync). Matches.
- (a4) OQ-0001-18 :1254-1262: "Answered by the Advisor (2026-11-03) ... So (a) is decided: the match ends" (inside a
  Proposed draft). PM-0001-26 :1616 (one connection's panic ends only it; 47 stay; tick goes on) and PM-0001-59 :1617
  (tick panic: process exits non-zero within 1 s, every client told or closed). Both are pass marks (proposed), not
  built. Matches.
- (a5) ADR 0004 part 20 :562-590 read: "No panic is reachable from network input or content data"; named sources:
  unwrap, unchecked index, std clamp "inside the sim" (overflow not named); server: a tick panic ends that room's match
  and process; unwind so a connection's panic closes only it; browser: -Cpanic=abort ends the sim's instance; "Online,
  after a trap, the Worker's host starts a fresh instance and rejoins through ADR 0001's resync path"; "In local play
  the trapped instance was the authority; recovery there is not researched (OQ-0004-7)". The finding's browser outcome
  omits local play. PM-0004-60 :1181: fuzzed commands into a running sim (nightly CI, proposed); room's control
  messages fuzzed "once room exists ... since a panic there ends the match too (part 20)".
- (a6) ADR 0002 part 4 :247-272 read: panic rules = unwind, connection work isolated from the tick, decoders never
  panic; Why: "one remote panic must not end a 48-player match". So "by ADR 0002 part 4's panic rules that would likely
  end that match" cites part 4 for the opposite of what its panic rules are for; the match-ending rule is ADR 0001
  part 11 / ADR 0004 part 20 (tick thread only).
- (a7) Where room's code runs: ADR 0002 :189 `room` = "the room's match logic with no I/O, no async runtime and no
  clock (its timers count ticks)": slot table, bots, per-client command queues, snapshot/event encoding, so it runs on
  the tick; :192 `server` = transport, async runtime, supervisor hook, metrics (the I/O side, per-connection work).
  PM-0004-60 and PM-0001-34 treat a panic in room's control-message handling as match-ending ("a panic there ends the
  match too"). So: room's overflow -> match ends (by the drafts); server's per-connection overflow -> that connection
  only; server's tick-side code -> match ends. The report's "room's or the future server's ... would likely end that
  match" is right for room and tick-side code, wrong for connection-side code.
- (a8) The claim appears only at the report :213-215 and :299-302 (git grep of 4bccf55 outside .checkpoints/, ADRs
  0001/0004 and research excluded). Reviews :166 still says round 2 "(pending)", so :215's "Round 2's security review
  reads it against part 4 and R2-ROOM-REC2" is a forward IOU; the review's full text lives only in the notes, deleted
  before ready (CLAUDE.md, Checkpoints 6).
- (b) Silent-wrap example reproduced, rustc/cargo 1.99.1, in /home/user/Shamble-Frontlines/.gate/skeptic-r2-R2-D2/
  (old/ = git archive 4d78ad6, part 4's two blocks; routeA/ = git archive 4bccf55; identical probe added to both: room
  bin src/bin/wrap_r2d2.rs + test tests/wrap_r2d2.rs, room-own `credits - price` u32 and `id + 1` u16 (#[inline(never)],
  black_box, catch_unwind), plus sim helpers spend_inline/spend_never called from room (room got `sim` as a path dep;
  the scratch Cargo.lock updated, no network)). Results, `cargo run --release` and `cargo test --release`, the same:
  OLD release: control 150-100 Ok(50); room 100-150 Ok(4294967246); room u16::MAX+1 Ok(0); sim inline helper from room
  Ok(4294967246); sim never-inline helper panicked "attempt to subtract with overflow" (crates/sim/src/lib.rs:25).
  ROUTE A release: control Ok(50); all four panicked ("attempt to subtract with overflow" / "attempt to add with
  overflow"). OLD dev profile (cargo run, no --release): all four panicked ("as test builds already do").
  2^32 - 50 = 4,294,967,246. The example holds as stated (release, old blocks, room's own code: 4,294,967,246; route A:
  panic). Nuance: credits are a sim value (ADR 0004 part 17, economy as a sim phase); under the old blocks a credits
  helper in sim wrapped the same way when room called it directly (the RR-1 gap), so "credits" is a fair illustration
  either way, but the example should say whose code it ran in.
