# U4a + U4b review, round 2 (netcode-architect), on what changed only

Started 2026-11-10. Read only: no edit, build or run in any copy (diff and reading only).

## Plan
1. Re-read my round-1 notes (u4ab-review-netcode.md): N1 (send cap), N2 (header sequences, record), N6 (ack_groups 0
   with ack_tick != 0), N8 (catalogue/rules edits), U4b L1, L2.
2. U4a diff prefix -> fixed worktree (crates/proto). Check N1: encode refuses 1,001 B Snapshot, decode takes up to
   1,150 B, channel_max; size check after framing, nothing appended on error; wire.rs a2 `as_sent`.
3. N6: decoder refuses (strict), encoder refuses NotCanonical; generator; test case.
4. N2 record wording in u4a-build.md "Fix pass, round 1" (and any doc comments in snapshot.rs/limits.rs).
5. Debug redaction (control.rs), never-0 guard unit test (hash.rs), nit; check none adds a defect.
6. U4b diff: tests/schedule.rs T-B1/T-B2/T-B3.
7. Verdicts.

## Progress
- 1 done. Diffs made (scratchpad u4a.diff 629 lines, u4b.diff 50 lines; `diff -ru` only).
- 2 N1 read (msg.rs fixed copy :219-388): encode = encode_into (uncapped, framing+length prefix written) then
  `out.len() - start - framing(row(t).1) > send_limit(t)` -> TooLarge, truncate to start. framing(row(t).1) = the
  `frame` encode_into used (same row, same fn), so the old check moved intact. send_limit = row limit (Snapshot 1,000,
  Limits::TYPES), receive_limit = 1,150 for Snapshot only; decode's per-type check and channel_max use receive_limit
  (RoomToClient datagram stays 1,150). Matches catalogue §2 Snapshot cell (netcode-proof-messages.md:88: 1,000 B at
  the send side until PM-0001-36; npt-client refuses above 1,150). Control 2 B length subtracted by after_length for
  both. A control message > 65,535 B uncapped would write a truncated u16 prefix, but the size check then refuses and
  truncates: never escapes. Allocation unchanged (old check also ran after the body). Only other caller of
  encode_into: the test helper `uncapped`. No crate outside proto in any copy uses Snapshot encode, SNAPSHOT_RECEIVE
  or ack_groups (grep of every worktree and .gate copy): the cap breaks no other lane. N1 CLOSED.
  - Test a3 snapshot: exact-length builds by bisection (removals 21 bits, shots as fillers); 1,001 and 1,150 decode,
    1,151 TooLarge on decode; 1,000 encodes to 1,000; 1,001 TooLarge, nothing appended ([7] kept). Fixer's before/after
    (step2-today FAIL at the 1,001 encode) is the evidence the test bites N1.
  - a2's as_sent: a decoded Snapshot of 1,001-1,150 B must give encoded() = None. Costs a2 the byte-identity check for
    those lengths only (canonical form is length-independent; flipped cases come from <=1,000 B encodings). Not a gap.
