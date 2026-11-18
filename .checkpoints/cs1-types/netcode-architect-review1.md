# netcode-architect, round 1 review of cs1-types (code at 82142fa, base main 22c6afa)

Read-only review. No source edits. Never `spikes/`, never /root/.claude/uploads/.

## Plan
1. Read crates/protocol/src/messages.rs (+ lib.rs, Cargo.toml) and 2-3 generated TS files.
2. Read plan-draft.md section 4 + O1, plan.md "After the advisor agent's review" item 6, rust-reviewer-review1.md "For later steps".
3. Read ADR 0001 part 5 (input, time, aim point), part 6 (snapshots), message table (~line 680), OQ-0001-6/7,
   PM-0001-17, PM-0001-58; parts 4 and 7 for the layout-hash question. ADR 0004 tick type; ADR 0005 content ids/hashes;
   R2 R2-XU-OQ4.
4. Check each type field by field (missing / extra / misleading); provisional list; pre-emption; decoder caps; layout q.
5. Verdict + ranked findings with file:line and the draft's line.

## Findings (one line each, as found)
- (read) messages.rs (116 lines), lib.rs, Command.ts, Snapshot.ts, Join.ts; plan-draft s4 + O1; plan.md item 6;
  rust-reviewer F2; A1 parts 4-8, 10-11, catalogue 675-702, budgets, failure modes, OQ-0001-5/6/7, PM-0001-13/17/20/
  21/51/57/58, test plan; A4 parts 2 and 4 (lines 153-196); A5 CD-1, CD-8; A2 part 5 + test table; r2 R2-SYNC-REC2,
  REC8 marks, R2-XU-OQ4, worked estimate rows 1738-1762; BP card 1, card 3, seam table 900-916, row 4 of 762-772.
- Check 1 result: every field the catalogue names (A1:682, 683, 685, 686) is present; none is extra. No missing type.
- N1 Low messages.rs:83,87,90: AimPoint doc credits "centimetres" to part 5; part 5 gives only "at most 80 bits"
  (A1:227); the cm step and 16-bit x/z are E3's for entity positions (A1:280), applied by the plan (plan-draft:65-66),
  with PM-0002-77's "proposed: the protocol's position step" (A2:1286) and PM-0001-51's 5 cm (A1:1648). Mark step and
  widths provisional, cite E3; note E3's height is 12 bits (0-40.95 m) while y: u16 holds 655.35 m (encoder clamps).
- N2 Low messages.rs:69-70: Command.sequence not marked provisional; drafts say "16-bit or narrower" (PM-0001-58,
  A1:1607; plan-draft:62 says so itself). Its wrap point is its width.
- N3 Low messages.rs:69,77,110-113: wrap modulus = the wire's width, not the Rust type's (R2 "small press counters",
  r2:2030; PM-0001-58's bite "a plain < instead of its wrap-aware comparison"). Doc line: counters compared only through
  the codec's wrap-aware helper, or the decoder unwraps them; presses doc lacks "wraps".
- N4 Low messages.rs:38-41: tick_rate_hz is a build-time constant, "never a server option" (A4:185-189): client checks
  it equals its build, never sets its clock from it; snapshot_rate_hz is nominal, the room falls to 20 Hz under
  pressure (A1:450-451): intervals come from header ticks. Doc lines.
- N5 Low messages.rs:48-50: order and contiguity of Input.commands unstated; U2 codes each repeat "against the newer
  command beside them" (A1:221-222): say newest-first (or oldest-first) and consecutive sequences now.
- N6 Low messages.rs:24-25,28-29: slot = the soldier's sim index (A4:155, GC2), so its width follows the wire's
  entity-index width (A4:156 -> A1); squad per team or room-wide unstated. Doc lines.
- N7 Low (optional) messages.rs:75-78: presses' 16 and buttons' 16 bits are two literals; one const ties them.
- (verified) settled and typed as settled: ticks u32 (A4:185), header tick 32-bit (A1:318-319), SHA-256 (A5 CD-8),
  ids strings (A5:275). Nothing marked provisional is settled. No `Eq`/`Clone` problem (integers only).
- (note) A1 tension for cs3: part 6 "deltas against the same baseline, acknowledged per group" (A1:293-294) vs Q11 P1
  "per-group baselines" (A1:892); types carry a baseline per datagram and an ack per group, so both fit.
- (note) Input names no 32-bit anchor; "16-bit offsets elsewhere" (A1:776): cs3 picks the anchor (room's own tick
  window, or the acked snapshot's tick); PM-0001-58 starts the server tick 30 below a multiple of 65,536.
- (note) A2 part 5 page post: "press and release edge counters ... (their exact fields are ADR 0001's)"; A1 names a
  press counter per button only: type follows A1; cs3 confirms no release counters.
- (note) OQ-0001-6 (b) "repeating the last on a miss" (A1:1161): replay then keys on ticks (commands stamped at or
  before the snapshot tick never run), under (a) on the last run sequence: the two acks' form rightly provisional.
- (note) Eq on Snapshot holds while all fields are integers; part 4's own-state section may be sent unquantised
  (A1:208), which would end Eq: decide when it enters.
- Q5 (layout check): nothing more now; part 6 keeps the wire off the block (A1:321-322); when cs2 wires it, run it on
  the first block of every Worker start, restarts after a trap included (A1:771).
- (weigh rust-reviewer F2) caps: agree; a count's own bit width caps commands and acks, token and mode_id need explicit
  length caps. Redacting Debug: due in cs3, where the join exchange first exists (the first code that could log a
  Join). Derived ==: harmless unless used to authenticate; the room verifies a signature and keys the replay cache on
  the unique id (A1:509-513), so the rule is "never == for auth", the derive can stay for tests.
- Verdict: APPROVE (no finding at Medium or above; Lows N1-N7, doc lines in messages.rs + one optional const; a doc
  edit needs one TS regeneration). Provisional list and decoder checks: in the handback.
- Not checked: spikes/ (not read: the proof test's codec, sizes, OQ-0001-6/-7 outcomes); no cargo run; 3 of 8 TS files.
DONE 2026-11-18
