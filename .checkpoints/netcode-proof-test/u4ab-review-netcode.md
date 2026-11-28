# U4a + U4b review, round 1 (netcode-architect)

Started 2026-11-10. Read only: no edit, build or run in either worktree.

## Plan
1. Read the approved plan u4-build-plan.md §5 U4a/U4b and §4.1, §4.3, §4.4-§4.6, §4.10, §4.11, §4.14.
2. Read advisor-u4.md (U4 verdict, Rulings, Ruling: U4s, Ruling: F1 and F2) and u4s-carries.md (1-5, 34, 6, 35).
3. Read docs/design/netcode-proof-messages.md and .claude/rules/netcode.md.
4. Read the builders' notes: u4a-build.md, u4b-build.md.
5. U4a: git diff/status in its worktree; read crates/proto src (bits, control, error, hash, input, limits, msg,
   quant, snapshot, wrap, lib), tests, check.mjs. Check each message's fields, channels, directions, limits; decode
   order and refusals; Wrap compares; snapshot header and group split; EntityRecord mapping; -57/-17/-21/-58 bites.
6. S1: read the skeptic note at 6652be4, seam.rs:114-137; decide cheapest correct fix and when it must land.
7. U4b: git diff in .gate/u4b-wt; read schedule.rs, lib.rs, tests/schedule.rs; ADR 0004 :207-231; marks.json
   PM-0004-67's "what"; check due-tick rule, cap, drop and rebase, counters, three bites.
8. Findings: one line each below as found. Then verdicts.

## Progress
- 1-4 read: plan §4.1-§4.6, §4.10-§4.14, §5 U4a/U4b; advisor-u4.md (verdict, U4s ruling, F1/F2); carries 1-5, 34,
  6, 35; catalogue; rules/netcode.md; u4a-build.md (both copies identical). Skeptic S1 read at 6652be4 (I ran a
  `git fetch` in the main folder to get that object: a ref update only, no switch, no tree change).
- 5 U4a src read: lib, msg, limits, error, input, quant, wrap, snapshot, hash, bits, control; sim seam.rs at base.
- U4a-N1 (candidate, Medium): msg.rs:219-231 content_limit uses SNAPSHOT_RECEIVE (1,150) for Snapshot on BOTH
  encode (:343) and decode; Limits::SNAPSHOT (1,000, the send cap, limits.rs:24, TYPES :65) is never enforced, so
  `encode` returns Ok for a 1,001-1,150 B snapshot. Check tests for a 1,001 B encode refusal.
- U4a-N2 (candidate, Low/Medium): snapshot.rs:16-18, :392-395: "newest received" and "last run" are 16-bit command
  SEQUENCES, not the tick offsets §4.4 (:269-270) names; -58/lt-tick-offset (§4.14) site/run setting need restating.
  Better design (ticks repeat/skip under OQ-0001-6 (b)), but a wire deviation not listed in the report items.
- U4a-N3 (Low): snapshot.rs:331-358 Hit/Kill always carry shooter/killer and the shooter's command seq; catalogue
  :141 "no shooter id when the shooter is not relevant". No absent form; a 255 sentinel would do (doc it).
- U4a-N1 CONFIRMED: no test encodes a 1,001-1,150 B Snapshot (wire.rs:375-387 only a 200-soldier one far above
  1,150; common/mod.rs:486-497 framed_limit and catalogue.rs:728-733 use SNAPSHOT_RECEIVE). Medium, before merge
  (public `encode` behaviour U4e's splitter builds on). Fix: content_limit(send) = SNAPSHOT for encode; test 1,000 Ok
  / 1,001 TooLarge.
- Read: catalogue.rs audit_u4 (items 1, 2, 3 size half, 6; 9 broken copies), check.mjs diff (twins, link grep,
  rename refusal, carry 34's four), proto clippy.toml/lib.rs diff, Cargo.toml (bites comment stale: entry 41).
  Decode order msg.rs:365-461 matches §4.4/catalogue §1: size(channel) > type > size(type) > prefix > direction >
  channel > body > finish(strict). Every catalogue byte count re-derived: Ping 10, Pong 16, Join 69, JoinAccepted
  80, RoomEnded 8, RecordStep 504, SessionStatus 16, Resync 3: all match.
- U4a-N4 (Low): input.rs:153-154 a repeat's tick step >255 is EncodeError::OutOfRange for the whole Input; a client
  whose unacked commands span a >8.5 s jump cannot send at all unless it drops those repeats. Doc the rule for U4e/U4f.
- U4a-N5 (Low): EntityDelta::between (snapshot.rs:79-107) codes a generation change as one Add; ADR 0001 part 6
  says "a removal plus an addition". Fine if U4e/U4g read an Add at a held index as replace-and-place: state it.
- marks.json read: PM-0004-67's three bites' "what" (timer-period; uncapped >7 per wake; drop not in the
  dropped-time counter), notes: `dropped_ns` = the counter's increase. -41, -60 (own-state-missing) read for S1.
- 7 U4b read: schedule.rs (263 lines), lib.rs diff, clippy.toml (no workspace clippy.toml to shadow), tests
  (389 lines), u4b-build.md, ADR 0004:207-231 and :1193. Due rule, last_due closed form (re-derived: k < (e+1)hz/1e9),
  cap, drop+rebase (base' = old grid point of last+1: phase kept, ≤1 ns per drop), counters, 200 ms → 6/12, 1 s →
  7+23 (766,666,667 ns) / 15+45: all hold. Carries 6 and 35 met. Bites: timer-period (1 per wake, base=now: the
  classic sleep-then-tick drift), uncapped (all owed), uncounted (drop+rebase, no counter, no report): each a
  faithful form of its marks.json "what"; site due_ticks; per instance (with_bite; two-thread test).
- U4b-L1 (Low): due_ticks takes ns "since the loop started" and Schedule::new fixes tick 0 at 0 ns; a U6 loop that
  passes raw monotonic ns gets one huge counted drop at its first wake (PM-0004-67's and PM-0006-23's counters read a
  startup stall). Doc/test it in U6's brief (or a start arg). Follow-up.
- U4b-L2 (Low): R2's jitter log must take "scheduled start" from an independent reference (loop start +
  ⌊(k_run + dropped_ticks)·1e9/hz⌋), never next_due_ns(): under timer-period next_due_ns() tracks the drift, so the
  jitter and "no ticks owed" checks would pass and only the 1 s stall's counter check would catch it. U6/R2 brief.
- U4a tests read: bites.rs (7 ids; -57 x3 through encode/decode; tick16 65,537 -> 1, hashes apart; no-trailing;
  no-channel), wire.rs a1 mapping (every kind, None/same/other-generation baselines, area/raw never sent), a3.
  Panic paths checked by reading: frame 2/4 index after get(frame); groups=0 caught before `groups - 1`
  (snapshot.rs:460 before :476); Field::integer asserts min<max so bits>=1 (grid.rs:60); RoomControl len-2 after u16.
- U4a-N6 (Low): input.rs:198-200 ack_groups 0 with ack_tick != 0 decodes (two encodings of one Input), unlike
  Snapshot's known() rule (snapshot.rs:501-509). Refuse as UnknownField. Follow-up.
- U4a-N7 (Low, design): one (tick, mask) ack per Input, of the newest tick decoded (input.rs:41-44): under reorder a
  tick the client holds complete may never be acked complete; B falls back older (correct, larger). U4e's client
  should ack the newest COMPLETE tick when the newest is partial, or U4r logs how often. Follow-up.
- U4a-N8 (Low, before merge by plan text): the catalogue edits are "in U4a's change-set" (§5 U4a: item 3's note) and
  rules/netcode.md: the builder lists them for the main session (u4a-build.md last bullet); add the header's
  sequences (N2), the event wire form and the send cap (N1).
- S1 view: cheapest correct fix = an owner-only record kind in npt-sim (velocity + ammo, fire timer, reload,
  active reload; ≤ 12), velocity out of SOLDIER (ADR 0001 part 6's remote row has none); U4a: the kind joins the two
  "own state" rows (2 lines; KIND_BITS stays 3 with 8 kinds; hash and generators follow ALL) + a test that owner-only
  kinds sit only in Owner groups and the remote soldier's width equals the catalogue row's. Not a per-(group,kind)
  field mask: a second home of layout positions (carry 1), group-dependent masks and hash, and it cannot fit (pos 3
  + yaw + pitch + vel 3 + flags + health + weapon + ammo/fire/reload/active = 15 > FIELDS 12). When: the main
  session's commit right after U4d merges (condition 3 freezes EntityKind and SOLDIER in wave 1), before U4e builds
  (its brief rev 2 at 0dccbf0 must carry it) and before U4g; tell U4d's lane now to keep weapon state out of SOLDIER.
  Not a U4a merge blocker.
- Verdicts: U4a APPROVE WITH FINDINGS (before merge: N1 fix + test; N2 and N8 text). U4b APPROVE WITH FINDINGS
  (L1, L2 follow-ups for U6/R2).
- Correction: N2's plan cite is u4-build-plan.md:273-274 (§4.4; plan sha256 0d69332c, as the builder's), and
  §4.14's row :454. Review done; returned to the workflow.
