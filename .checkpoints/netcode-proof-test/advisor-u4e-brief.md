# The advisor's check of the U4e builder brief (rev 3.2), 2026-11-11

Seat: a day session's advisor agent (general-purpose, opus, xhigh), on main's `.claude/agents/advisor.md`. Read only:
no build, no run, no commit. Branch `claude/netcode-proof-test-plan-hcky5g` at c5190b0.

## Plan
1. Read the brief whole (N/u4e-brief.md rev 3.2) and the check brief (N/advisor-u4e-brief-brief.md).
2. Read the plan's §5 U4e and the parts of §4 it leans on (N/u4-build-plan.md), the Lanes, §6 and §7.
3. Read my rulings: W1-U4E, W1-S1, W1-N2, W1-TARGET (N/advisor-u4-wave1.md), N/advisor-u4d-close.md,
   N/advisor-u4j-r2.md B2.1 (carry 61), N/advisor-u4.md where the brief cites it.
4. Read N/u4s-carries.md §U4e (carries 21-27, 39, 50, 51, 53-57, 61) and compare with the brief's section 3.1 copy.
5. Read the check (N/u4e-brief-r3-check.md): F1 to F3 and the Mediums and Lows; confirm each fix in rev 3.1/3.2 text.
6. Check rev 3.2's fold (N/u4e_fold2.py): carry 61's note, R27, R28, R38, section 0's note on line numbers, the
   "U4j unmerged" lines.
7. Spot-check the brief's claims about merged code at the branch head (crates sim, proto, server, room, client, log,
   judge; marks/) and the message catalogue; .claude/rules/netcode.md.
8. Coverage table: every test, negative case, bite and carry of §5 U4e against the brief.
9. Rule: verdict (APPROVE or REVISE with exact words), U4E-Q12, Q13, Q14, and the F2 route.
10. End: Decisions needed (Mustafa), For Mustafa.

## Blocks (written as each finishes)

### Block 1. What I read, and the state it rests on (done)
- The brief whole (930 lines, rev 3.2); the check brief; plan §5 U4e (:704-736 as the file stands), §4.1-§4.15, Lanes,
  §6, §7; W1-U4E, W1-S1, W1-N2, W1-PUB, W1-TARGET (advisor-u4-wave1.md); advisor-u4.md conditions 6, 10, 12 and the F2
  ruling (:436-466); advisor-u4d-close.md B2 and its U4d-F part; advisor-u4j-r2.md B2.1 and B5; u4s-carries.md §U4e
  and §U4r (58, 62) and 52, 65; the check (u4e-brief-r3-check.md); u4c-review-netcode.md; u4c-lc8-rerun.md;
  u4c-round2-netcode-architect.md; the catalogue §1-§3 and §6; .claude/rules/netcode.md.
- Code at the branch head c5190b0. `git diff --stat 39f4290 d50396a` touches none of crates/{room, server, client, sim,
  proto, link}, check.mjs or docs/; d50396a..HEAD adds only results/c-u2-pm-0001-3/65869918809f/. So every rev-3 cite
  into U4e's crates, npt-sim, npt-proto, npt-link and check.mjs still lands on the same text.
- `Scene::effect_rows()` is not at the head yet (scene.rs:119 is still `pub(crate) effect_rows: u8`): expected, the main
  session adds it at launch (7.2 item 2).

### Block 2. Coverage of §5 U4e, testable as worded (done)
| Plan item (u4-build-plan.md :704-736) | Brief | Verdict |
|---|---|---|
| Builds, npt-room (all 13 items and the clippy bans) | 2.1 | present, testable |
| Builds, npt-server (pump, ConnLimits, Counters, buckets, deadline; no AddrTable) | 2.2 | present, testable |
| Builds, npt-client (frames, complete-tick rule, sentinel, acks, ClientEndpoint) | 2.3 | present, testable |
| catalogue item 4 | e1 | present, testable (12 and 6 rows: snapshot.rs:488, :644; catalogue :132-157) |
| forced split, each group alone | e2 | present, testable |
| lost group unlogged and counted; refused gives the sentinel | e3 | present; its resolution timing is carry 61 (c)'s and is not tied to e3 (finding A1) |
| baseline rule; ring age edges | e4 | present, testable |
| 1 s gap evicting B | e5a, e5b | present; as worded it stops on Loopback (U4E-Q14, ruled below) |
| window counting the pump's bytes | e6 | present, testable |
| mirror = client hash, Loopback, 1,000 ticks | e7 | present; its split clause may be unreachable for a legitimate encoder (finding A2) |
| Send asserts | e8 | present, testable |
| the nine negative cases | e9 | all nine present, each testable as worded |
| -18 end to end | 2.5 | present; the gap as U4E-Q14 |
| Bites line (12 names) | 2.6 | all 12 present with a site and a proof; owner-to-all's route needs one limit (finding A3) |
| -22/unbounded (§4.14 :475, not on the Bites line) | 2.6's last bullet | open: U4E-Q12, ruled below |
| Marks moved: none | 2 | present |
| carries 21-27, 39, 50, 51, 53-57, 61 (sixteen) | 3.1 word for word; e12; return item 6 | 3.1 equal to carries :147-247 (my own `diff` of the two ranges: no output); e12 has a line for each but carry 61 (finding A1) |
- Beyond the plan: nothing built that the plan, a carry or a ruling does not name. e10 is the F2 ruling's pins; e11 is
  §4.11's own test; e7's non-vacuity and e5's alignments are test strength, not builds.
- W1-TARGET in the build rules: item 1 (lock) is rule 2, item 2 (provenance) rule 3, item 3 (disk) rule 6; item 4 is
  the main session's. Carried as ruled.

### Block 3. The check's three Highs, and rev 3.2's fold (done)
- **F1 (the fold carried into the body): fixed.** e12 has lines for carries 53, 54, 55, 56 and 57; carry 53's test
  asserts a generation change happened (e12; 3.1's note); the count reads "Sixteen carries"; return item 6 names 21-27,
  39, 50, 51, 53-57 and 61; no "pending" or "launch as is" is left outside the change history (grep).
- **F2 (bite proof against check.mjs green): fixed.** 2.6 states wave 1's two parts (a bites-only twin that passes; an
  in-place scratch switch shown failing by name, restored and sha256-checked), and e5b, 2.5 and return item 4 use
  them. check.mjs's `testBothBuilds` (check.mjs:1493-1501) then stays green in both builds.
- **F3 (owner-to-all's proof): fixed.** The proof is carry 53's per-record identity test; the group-level class check
  and the entity-section route are named as not counting.
- **Rev 3.2's fold:**
  - 3.1 equals u4s-carries.md :147-247 (checked above), carry 61 included, and 3.1's note on carry 61 matches B2.1
    (the decode lean, the drop count with a SERIES.md row) and marks/README.md §1 at d50396a (:50-71: the
    `server_hash` and `decoded_hash` items, with `replay_hash` at :70-71).
  - R27 holds at d50396a: marks.json spells every id U4e plants but owner-to-all's; U4j's one new id,
    `PM-0001-18/ws-budget-x2` (marks.json:4431), is U6's.
  - R28 holds: `TickRecord.hs` (log/src/lib.rs:121), `set_hashes` (:158), `hashes` (:168); SERIES.md :319 and :320.
  - R38 holds: 396, 443, 52 of 52, 10 steps (u4j-build.md:354-355, :388-389).
  - Section 0's line-number note is right: the diff of 30528c7's plan against today's shows one row added after :199,
    §10's preamble three lines longer and §11 two; the brief cites nothing in the preamble (old :954-:963), so every
    cite from :200 to :953 is one lower and from :964 on four lower. I spot-checked 14 plan cites against the file:
    each lands.
  - The "U4j unmerged" lines are updated (carry 54's note, section 4's lane list, U4E-Q13 (c)).
  - Two stale lines the fold missed (finding A6): section 1.4 still cites carries "§U4e (:147-228)" and carry 40 at
    ":231-239" (now :147-247 and :250-258), and the provenance paragraph says "rev 3.1, this file".

### Block 4. My findings on rev 3.2, each with its exact words (done)
Severity on CLAUDE.md's scale. None is High; none is a security finding. Every "old" phrase below is the brief's text
read with its line breaks as spaces; the fold matches it that way, once, and rewraps.

**A1, Medium: carry 61's unit test has no line in e12.** 3.1's note on carry 61 says "its unit test ... is one of your
tests (2.4)", but e12 lists every carry that names a behaviour except 61, the one the fold added. Carry 61 also states
two rules no test reaches: (a) no server hash for a snapshot the room did not send, and (c) a tick is resolved lost only
once it has left the frame ring. That is the check's F1 again, for one carry; the carry's own words keep it from a
High.
- In e12, end carry 57's bullet with ";" in place of its final ".", and add after it:
  "  - carry 61: `ClientEndpoint` fed tick t+1's snapshot before tick t's (as L1's reorder step delivers them) files t
    under t (its hash if the client decodes the stale group, or lost if it drops it, the drop counted; never the
    sentinel), and one datagram received twice files one hash (the carry's unit test); a tick with a group missing is
    not resolved lost while it is younger than `npt_sim::ACK_RING_TICKS` ticks (by the newest tick the client holds)
    and is resolved lost (`snap_incomplete` 1, no hash) once it is that old (carry 61 (c)); and in carry 54's test the
    room exposes no server hash for client k at the skipped tick (carry 61 (a))."

**A2, Medium: e7's split clause may be out of reach for a legitimate encoder.** One group carries at most 1,000 B of
payload (`Limits::SNAPSHOT`, limits.rs:25), and one client's datagrams in one interval stay within about 1,067 B,
counting 28 B each (catalogue :88). Two groups therefore carry at most about 1,010 B, 10 B more than one. An encoder
that never splits for that gain is correct, and then no tick of e7's run splits: e7 fails as worded, and the builder
stops or bends the encoder to pass a test. The mirror's split case still needs a comparison.
- In e7, replace "at least one tick was sent as 2 or more groups," with: "at least one tick was sent as 2 or more groups
  (if the encoder never splits a tick at the window's budget, which is likely: one group carries up to 1,000 B of
  payload and two at most about 1,010 B of the about 1,067 B a tick allows, counting 28 B each: catalogue :88,
  limits.rs:25; then e2's forced split tick has the mirror compared with the client's hash the same way, and your
  report says which),".

**A3, Medium: owner-to-all's route needs the section's limits, and carry 53's test needs a snapshot on every tick.**
Writing every other slot's own-state records into each client's section (48 slots × 10 records) passes the own-state
section's 8-bit count (snapshot.rs:26; `TooMany` at :526-530) and the 1,000 B datagram. The encoder then refuses, carry
54 skips the snapshot, and carry 53's check ("in every `Snapshot` ... sent to it") reads nothing and passes: the bite
cannot show its defect. Catalogue item 5 asks for "one `owner` group to every client" (catalogue :254), so one slot is
enough.
- In 2.6's owner-to-all row, replace "which writes other slots' own-state records into every client's own-state section
  (carry 53's route)" with: "which writes one other slot's own-state records (its Soldier, OwnState and Effect rows)
  into every client's own-state section (carry 53's route; catalogue :254's "one `owner` group to every client"),
  within the section's 8-bit count (snapshot.rs:26, :526-530) and each group's 1,000 B, so that every bitten snapshot
  still encodes: a refused one is skipped under carry 54 and would leave carry 53's check nothing to fail on".
- In e12's carry 53 bullet, after "the rows whose owner is that slot (`Scene::effect_rows()`);" insert: " each client
  receives a `Snapshot` on each compared tick, and the test fails if one has none (a snapshot skipped under carry 54
  would make the check vacuous);".

**A4, Medium: the room must handle every encoder error, not only `OutOfRange`.** Carry 54 names `OutOfRange`; the
encoder also returns `TooMany`, `TooLarge` and `NotCanonical` (error.rs:42-58). The nearest trap is the event section's
8-bit count: after a 1 s gap the stress scene queues about 350 events (§4.7, u4-build-plan.md :336 as the file stands),
more than one snapshot can carry by count or by bytes. An `unwrap` there panics the room on the first burst. Carry
54's "without a panic, `unwrap` or `expect`" is the rule for the encoder's refusals; this extends it to the other
kinds, and narrows nothing.
- In 2.1's Encoder bullet, after "and counted per client; the next tick encodes afresh." add: "The other `EncodeError` kinds (`TooMany`,
  `TooLarge`, `NotCanonical`: error.rs:42-58) take the same path, counted under their kind. A correct encoder meets
  none: it keeps each section within its count (own-state 8 bits, entities 16, events 8: snapshot.rs:26, :526-530)
  and each group within `Limits::SNAPSHOT` (1,000 B: limits.rs:25), so an event backlog (after a gap, up to the
  queue's cap) goes out over as many snapshots as the remainder needs, carry 50 still bounding its age."

**A5, Low: 2.8 does not name the bites-only constructors.** §4.11 names `Room::with_bite` (:400 as the file stands;
the brief's numbering :398-400); the pump and the endpoint need a per-instance bite choice too (2.2, 2.3). The words
below use the brief's numbering.
- In 2.8, after "and, bites builds only, `BITES` (:194; U4E-Q3)" add: "; the bites-only constructors that choose a bite
  per instance (`Room::with_bite`, which §4.11 :398-400 names, and the pump's and the endpoint's, where they take the id
  rather than a `Quant` or grid config passed in) are expected: list them as beyond §4.1's rows, for
  engine-architect".

**A6, Low: three stale cites.**
- The event's wire text is snapshot.rs:31-32 (at 39f4290 and at the head), not :29-30: replace "snapshot.rs:29-30"
  with "snapshot.rs:31-32" in all three places (2.1 twice, 2.3 once).
- Section 1.4: replace "carries §U4e (:147-228), quoted in section 3; carry 40 (§U4r, :231-239)" with "carries §U4e
  (:147-247), quoted in section 3; carry 40 (§U4r, :250-258)".
- The provenance paragraph: replace "and revised once for that check (rev 3.1, this file)" with "and revised once for
  that check (rev 3.1); rev 3.2 is the main session's fold at U4j's merge, and rev 3.3, this file, its fold of the
  advisor's check (advisor-u4e-brief.md)".

**A7, Low: the base counts' "otherwise" branch is dead.** `<base>` holds U4j's merge and the carry-51 commit, so it
differs from 39f4290 under spikes/ whatever happens.
- Section 4 rule 5: replace "If `<base>` differs from 39f4290 under spikes/, the main session gives `<base>`'s counts at
  launch (7.2 item 1)." with "At d50396a (U4j's merge): measured 396, bites 443, self 52 of 52 (R38). `<base>` also
  holds the carry-51 commit, so it differs from both under spikes/, and the main session gives `<base>`'s counts at
  launch (7.2 item 1)."
- 7.2 item 1: replace "Otherwise the counts are 365, 412 and 52 of 52, with 10 steps." with "(`<base>` holds U4j's
  merge and the carry-51 commit, so this run is always needed; the last counts known are d50396a's: 396, 443, 52 of
  52, 10 steps.)"
