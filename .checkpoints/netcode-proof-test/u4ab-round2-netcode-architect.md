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
- 3 N6 read: input.rs:205 decoder `ack_groups == 0 && ack_tick != 0 && q.strict()` -> UnknownField, right after the
  two header reads, before count; encoder :134 NotCanonical after TooMany. strict() is false only under the -57
  no-trailing bite (quant.rs:116-122), the same gate as Snapshot's known() flags rule (snapshot.rs:500-508): one rule,
  one bite. groups != 0 with tick 0 stays valid (tick 65,536k). Generator: same RNG draws (u16 then u8) so seeds keep
  their streams; never makes the refused form. a2's random buffers now reach the refusal. Catalogue has no Input field
  table (U4a states it: plan §4.4); input.rs module text and error.rs docs state the rule. N6 CLOSED.
- 5 Debug: Join (token, bind) and all 4 SessionControl variants hand-written via pub(crate) Redacted "[16 B]";
  content_hash, version, profile, step, result, note still print (none secret). JoinAccepted has no secret; RoomControl
  has none (MintToken/MintCapability carry only slot/kind; replies are U6's ControlSocket). Test checks decimal and hex
  of 3 distinct secret bytes in 10 texts (bare and inside Message): a false match can only fail it. Closed.
  (Observation only, not reopened: RecordStep's note is client text and still prints; catalogue :120-121's "never the
  data" is U4e's log discipline, not a Debug rule.)
- hash.rs finish(): same function, now testable; test feeds 0. Nit `for _`: fine. No defect added.
- Scope check (diff -rq of the whole spike, target excluded): U4a changed exactly the 8 named proto files; U4b exactly
  crates/server/tests/schedule.rs. npt_link sends datagram bytes as given (link/src/lib.rs:154-173), so the encoded
  Snapshot is the WebTransport payload the catalogue's 1,000 B counts. Plan :133-134 "a datagram cap of 1,000 B until
  -36 measures" = the fix.
- 4 N2 record (worktree u4a-build.md:221-236) checked against code and the advisor's W1-N2 ruling
  (advisor-u4-wave1.md:305-345, which already rules the plan, marks.json and catalogue text): plan cites hold (plan
  sha256 0d69332c; §4.4 :273-274; §4.14 :454), snapshot.rs:9-17 and :392-395 hold, input.rs's tick step 0-255 holds,
  the "why" matches the ruling. Consistent; the ruling's text supersedes the record's suggested row wording.
  - R2-N2a (Low, wording): the record says "`Command::seq`'s low 16 bits"; Command::seq is u16 (sim/src/seam.rs:28),
    so the header carries the whole sequence, nothing truncated. Harmless, but a reader may look for a widen step that
    does not exist for the sequence itself. No push on its own.
- 6 U4b: last_due = ((e+1)hz - 1)/1e9 (schedule.rs:139-141). T-B1 test: at(3) whole at 30 and 60 Hz (asserted);
  at at(3)-1 with "- 1" dropped, floor(at(3)*hz/1e9) = 3 -> to_run 3, test expects 2: fails, as the plant showed; at
  every non-whole due time the two forms agree, so no other test can see it (gap real, now closed). T-B3: 1 s stall
  from tick 0 at 0: last_due(1e9) = TICK_HZ owed, run LOOP_CATCHUP, drop TICK_HZ - LOOP_CATCHUP (30: 7 + 23, as round
  1); plus LOOP_CATCHUP == loop_catchup_at(TICK_HZ). Acts on the cap, not only stores it; the 60/15 scenarios catch a
  literal 7. T-B2: comment only, assert kept, label honest. All three closed; no defect added. L1, L2 stay U6/R2
  follow-ups (not reopened).
- 7 Verdicts: U4a APPROVE WITH FINDINGS (N1, N6, SEC-3/-6/-7 closed; one Low wording nit R2-N2a; N8's catalogue edits
  stay the main session's at U4a's merge per W1-N2). U4b APPROVE. Review done.
