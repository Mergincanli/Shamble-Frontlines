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
- The brief whole (930 lines, rev 3.2); the check brief; plan §5 U4e (:704-735 as the file stands), §4.1-§4.15, Lanes,
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
| Plan item (u4-build-plan.md :704-735) | Brief | Verdict |
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

### Block 5. The brief's open questions (done)
Plan cites inside the quoted brief words use the brief's numbering (the plan at 7375cdcf…, section 0).

**U4E-Q14 (the 1 s gap on Loopback): (b), with the gap tied to the ring, not to 30 Hz.**
- Why the 1 s gap cannot work on Loopback. Call L the last snapshot delivered before the gap and F the first tick
  after it. B is never newer than L, so B's age at F is at least F − L, which is the number of snapshots lost plus 1.
  The server's sends are ⌊k × 10⁹ / 30⌋ ns apart (U4b's `due_ticks`), so s(k + 30) − s(k) is exactly 10⁹ ns, and a
  half-open 1 s gap holds exactly 30 sends: F − L = 31. If the client's first input after the gap (which acknowledges
  L) arrives before F's encode, B = L at age 31, inside the ring (ages 0 to 31 kept: §4.5): no eviction. So e5a would
  fail at one alignment or both, and the builder would stop. The brief's own reading (31 or 32) is right.
- The fix. A gap of `npt_sim::ACK_RING_TICKS` tick periods holds 31 or 32 sends at 30 Hz, whatever the alignment and
  whichever way the builder rounds the ns (32 × 10⁹ / 30 is not whole), so F − L ≥ 32 and B leaves the ring. Tying it
  to the constant, not "1 s plus two tick periods", keeps it right at 60 Hz too (the ring is 64 ticks there, and
  1 s + 2 periods would be 62 sends, B at 63, kept).
- Not looser: -18's judged runs are L1 B1, whose 1 s bursts add the RTT to B's age (the brief's U4E-Q14 text); this
  changes only the Loopback unit proof, which exists to show the bite can bite (§4.5 :298-299). No ADR or mark word
  changes. The gap is a test's input, not a constant the build needs, so the F2 ruling's point (3) does not apply; it
  stays a `let` computed from the constant by path and the rate the test passes in (carry 39: no const or static).
- **Words, the brief:**
  - e5's lead: replace "e5 a 1 s gap evicting B (:718; §4.14 :460's unit proof, the server half), as two tests," with
    "e5 a gap evicting B (:718's "a 1 s gap evicting B": on Loopback the gap is `npt_sim::ACK_RING_TICKS` tick periods,
    U4E-Q14 (b); §4.14 :460's unit proof, the server half), as two tests,".
  - e5a: replace "The ring holds 32 ticks = 1.067 s at 30 Hz, so a 1 s gap (30 ticks) evicts B only with B's age at
    the gap's start counted in." with "The gap lasts `npt_sim::ACK_RING_TICKS` tick periods (1.067 s at 30 Hz: 1 s plus
    two tick periods; U4E-Q14 (b)), its length in ns on the injected clock computed in the test from that constant by
    path and the rate the test passes in (a test's input, never a const or static: carry 39). Why: every snapshot whose
    send falls in the gap is lost, at least 31 at 30 Hz, so B's age at the first tick after the gap is at least 32 at
    either alignment, while a 1 s gap loses exactly 30 and leaves B at 31 or 32 on Loopback, at the ring's edge."
  - e5a: replace "the precondition holds only if both give at least 32. If either leaves B at age 31 or less, report
    both (§4.14 :460's "a 1 s gap evicts B" then does not hold on Loopback), and do not lengthen the gap or choose the
    alignment on your own (U4E-Q14)." with "the precondition holds only if both give at least 32
    (`npt_sim::ACK_RING_TICKS`); if either gives less, stop that part and report both, and change neither the gap nor
    the alignment on your own. e5a also runs a 1 s gap at both alignments and reports B's tick and age there (reported,
    not asserted)."
  - 2.5: replace "through a 1 s gap in delivery:" with "through a gap in delivery (e5a's: `npt_sim::ACK_RING_TICKS`
    tick periods, U4E-Q14 (b)):".
  - 2.5's gap bullet: replace "that drops every datagram both ways for 1 s on the injected clock is inside the lane;
    its length in ns, never a `*_TICKS` constant (carry 27)." with "that drops every datagram both ways for e5a's gap
    on the injected clock is inside the lane; its length in ns, from `npt_sim::ACK_RING_TICKS` by path and the rate
    passed in, never a const or static (carries 27 and 39)."
- **Words, the plan** (the main session's commit; condition 12's list of allowed edits gains them by this ruling):
  - §4.14's -18/no-fallback row: replace "a 1 s gap evicts B; `burst_eviction` = 1" with "a gap evicts B (on Loopback
    `npt_sim::ACK_RING_TICKS` tick periods, 1 s plus two tick periods at 30 Hz, since there a 1 s gap leaves B at the
    ring's edge: the advisor's U4E-Q14 ruling); `burst_eviction` = 1".
  - §5 U4e, Tests: replace "a 1 s gap evicting B;" with "a gap evicting B (on Loopback `npt_sim::ACK_RING_TICKS` tick
    periods: U4E-Q14);".
  - §5 U4e, -18's bite end to end: replace "through a 1 s gap in delivery:" with "through a gap in delivery (on
    Loopback `npt_sim::ACK_RING_TICKS` tick periods: U4E-Q14):".
  Each phrase occurs once in the plan (checked).

**U4E-Q12 (-22/unbounded's planting unit): (c).** U4e plants and unit-shows it; U4f runs it judged; the plan's Bites line
gains it.
- Why: §4.14's own rule is "planted where its code lives, shown to change state by a unit test in that sub-unit, and
  run judged in the sub-unit that moves its mark" (:453-454 in the brief's numbering). U4e builds the queue and its cap
  (:707; U4E-Q6); U4f builds no queue (:792-795) and moves -22 (:801). The Bites line missed it, as it missed lt-cmd-seq
  and lt-press (U4E-Q1, the same slip). (b) would have U4f edit a site U4e wrote and unit-shows nothing of its own.
- What the bite needs from the room: -22's first check reads `tick.v.ack_starved_headroom_count` (marks.json,
  PM-0001-22: "it reaches the cap and never passes it", min == 0), whose writer SERIES.md:21 names as npt-room ("cap
  minus queue length ... negative = over the cap"). So the room exposes each client's queue length beside the cap.
- **Words, the brief:**
  - 2.1's EventQueue bullet, after its last sentence (carry 50's), add: "By U4E-Q12 (c), the cap is also
    PM-0001-22/unbounded's site (§4.14 :474; 2.6), so the room exposes, per client and tick, its queue's length beside
    the cap (the headroom, cap less length, signed: below 0 only under the bite), which U4f's -22 runs write as
    `tick.v.ack_starved_headroom_count` for the never-acking client (SERIES.md:21)."
  - 2.1's `npt_room::BITES` bullet: replace "-62/server-echo, and owner-to-all's: U4E-Q13)" with "-62/server-echo,
    -22/unbounded (U4E-Q12), and `catalogue-5/owner-to-all` (U4E-Q13))".
  - 2.6's table: after the PM-0001-62/server-echo row, add: "| PM-0001-22/unbounded | the event queue's cap: under the
    bite the queue keeps every event past its cap, with no overflow resync and no `Kick::Overflow` (§4.14 :474;
    U4E-Q12 (c)) | a never-acking client: in the clean build its headroom (cap less queue length) falls to 0, never
    below, and one resync follows (carry 57's test); under the bite it goes below 0 with no resync and no kick (§4.14
    :474's "headroom below 0"); the test may give the room a smaller cap, as data, to stay short. U4f runs it judged
    over a full S30 (plan :801) |".
  - 2.6's bullet: replace "-22/unbounded's site is U4e's `EventQueue` cap, and the plan's U4e Bites line omits it:
    U4E-Q12." with "-22/unbounded's site is U4e's `EventQueue` cap: U4e plants and unit-shows it, and U4f runs it judged
    (U4E-Q12 (c); the plan's Bites line gains it)."
- **Words, the plan** (the main session's commit): §5 U4e's Bites line: replace "server-echo (-62, the room's routing),
  owner-to-all (catalogue item 5's)," with "server-echo (-62, the room's routing), unbounded (-22, the event queue's
  cap; U4f runs it judged: U4E-Q12), owner-to-all (catalogue item 5's),".

**U4E-Q13 (the owner-to-all bite's id): (a), the id `catalogue-5/owner-to-all`.**
- Why: no mark holds catalogue item 5 (the visibility classes are proven at slice 3: ADR 0001:1687-1688, the brief's
  cite), so (b) would invent a PM row that judges nothing and (c) would put a bite under a mark that does not judge
  it; both misstate what a run proves. A name outside `PM-<ADR>-<row>/<name>` cannot be mistaken for a mark's bite:
  check.mjs's `BITE_ID` (check.mjs:689) does not match it, and npt-log takes a header's bite as any string
  (log/src/lib.rs:71-72). The cost is that check.mjs's one-home grep cannot see it, so a test holds its one home.
- **Words, the brief:**
  - 2.1: as under Q12 above (the `npt_room::BITES` bullet).
  - 2.6's owner-to-all row: replace "Its id: U4E-Q13 |" with "Its id: `catalogue-5/owner-to-all` (U4E-Q13 (a)) |".
  - 2.6's bullet: replace "Every id U4e plants is marks.json's spelling, but owner-to-all's, which marks.json does
    not have (R27; U4E-Q13)." with "Every id U4e plants is marks.json's spelling but one: `catalogue-5/owner-to-all`
    (U4E-Q13 (a)), which marks.json does not have and check.mjs's bite-home grep does not read (it reads only
    `PM-<ADR>-<row>/<name>`: check.mjs:689). So npt-room's bites test pins `npt_room::BITES` and checks each of its
    PM-form ids against marks.json, as server/tests/schedule.rs:345-353 does, exempting that one id by name; and a
    bites-only test in npt-server's tests (which reach every crate's list under `--workspace --all-features`) asserts
    that no other list holds it (`npt_proto::BITES`, npt-sim's `World::BITES` and `OWN_COPY_SNAP_BITES`,
    `npt_server::BITES`, `npt_client::BITES`). U4r's `--bite` dispatch selects it by that string."
- For U4r's brief, later: the class audit's runs (plan :750) select that id, and their headers list no mark, so the
  judge never reads it.

**Section 8 and 7.2, for all three:**
- Section 8's ruled table: after the U4E-Q11 row add "| U4E-Q12 | (c): U4e plants and unit-shows -22/unbounded at the
  event queue's cap; U4f runs it judged; the plan's Bites line gains it | 2.1, 2.6 |", "| U4E-Q13 | (a): the id
  `catalogue-5/owner-to-all`, in `npt_room::BITES` only, its home held by tests | 2.1, 2.6 |" and "| U4E-Q14 | (b), the
  gap tied to the ring: `npt_sim::ACK_RING_TICKS` tick periods on Loopback; the 1 s ages reported | 2.4 (e5), 2.5 |".
- Section 8's "**Open**" heading: replace "**Open** (new in rev 3, and U4E-Q14 in rev 3.1; only where no ruling gives
  the answer; each self-contained):" with "**Ruled at the advisor's check** (advisor-u4e-brief.md; the questions as
  asked, kept for the record; the rulings are in the table above):".
- 7.2 item 4: replace "the advisor's answers, written into 2.6, 2.1 and 2.4 (e5a)." with "answered by the advisor's
  check (advisor-u4e-brief.md) and written into 2.1, 2.4 (e5), 2.5 and 2.6 (done)."

### Block 6. F2-ROUTE: U4c's netcode F2 (Medium, open) (done)
**Ruling: a carry for U4d2, the next free entry (66 at this writing), in u4s-carries.md's "U4d2 or the main session"
section; U4r's brief takes it if U4d2's is final first. F2 keeps its Medium label and stays open until the carry is
met. L-c8's judged half PASS stands, and its hash half holds today on the evidence below.**
- What the evidence shows (u4c-lc8-rerun.md; u4c-build.md:79-84; u4c-review-test.md:95-99): the merged build's npt-lab
  runs of P2, B1 and P4 seed 1 print whole-v2-log hashes (43745683ab36d67b, 722f20713e18352e, 432693efb9698c05) equal
  to those of the builder's scratch logs of the same runs; on those logs' bytes the builder's scratch harness gave
  SHA-256(v1 tag + datagram entries) equal to U2's 913db8e9…, feabf2f0… and b8a04f2f…, in the builder's build and
  again in the test-engineer's own build. Equal whole-log hashes mean equal logs, so the merged build's datagram
  entries are U2's.
- What it lacks: the datagram-entry hash comes from an uncommitted scratch harness, so no later run of -3's set can show
  it without rebuilding that scratch. That is F2's point, and it still holds. It is a reproducibility gap, not a doubt
  about U4c: no Critical or High, no security finding.
- Why U4d2: the change is in crates/lab (probe.rs and main.rs), which is U4d2's alone in wave 2 (u4-build-plan.md :842,
  :862 as the file stands); U4d2 already edits npt-lab (its `hash` subcommand), so no new lane is opened. It needs no
  npt-l1 change: `FateLog::for_each_canonical` is public (l1/src/lib.rs:351), and U4c's committed test reads the
  same bytes through it (l1/tests/l1.rs:481-532). Not U4e: crates/lab is never U4e's (condition 10 (b)).
- Not a "for later" feature: it is the evidence form an approved test (L-c8) names; -3's L1 set was re-run because U4c
  changed npt-l1's model, and the next model change would need the same re-run.
- **The carry's words** (copy as they are):
```
66. **U4c's netcode F2, Medium (u4c-review-netcode.md:53-54; L-c8's judged re-run, u4c-lc8-rerun.md; the advisor's U4e
    brief ruling, F2-ROUTE):** "npt-lab's probe also writes the SHA-256 of (v1's tag + the v2 fate log's datagram
    entries), the hash §5 U4c's L-c8 compares with U2's committed 913db8e9…, feabf2f0… and b8a04f2f… for P2, B1 and P4
    seed 1, beside the whole log's SHA-256: in the run log's header and in its summary line, so that -3's script
    (ops/runs/c-u2-pm-0001-3.mjs) records it unchanged. It reads the log through npt-l1's public
    `FateLog::for_each_canonical` and edits no crates/l1 file; v1's tag and the datagram entry's form are those U4c's
    test `v1s_tag_and_the_v2_logs_datagram_entries_hash_as_v1s_log_did` reads (l1/tests/l1.rs:481-532). U4d2's
    report shows the three values from probe runs of those three runs (the -3 set's own arguments), each equal to
    U2's, with the commands it ran. Until then L-c8's hash half rests on U4c's scratch harness through equal
    whole-log hashes (u4c-lc8-rerun.md). If U4d2's brief is final without this entry, U4r's brief takes it word for
    word, U4r in place of U4d2."
```
- PR #40's report lists F2 as open (Medium, netcode-architect's, U4c), with carry 66 as its route.

### Block 7. The verdict (done)
**APPROVE, on conditions L1 to L4.** The build may start once `<base>` is filled and L1 to L3 are done.
- Why not REVISE: rev 3.1 fixed the check's three Highs as claimed, rev 3.2's fold is right, every test, negative case,
  bite and carry of §5 U4e is in the brief and testable, W1-TARGET's rule is in the build rules, and nothing goes
  beyond the plan. My findings are four Mediums and three Lows, each with exact words a script can fold, and the three
  open questions are ruled with exact words. Nothing needs a second check unless the fold goes beyond those words.
- Risk tier 1 (spike code inside PR #40, when U4e builds); this ruling is a notes-folder text. Reviewers for U4e's build
  stay the plan's (netcode-architect, security-auditor, engine-architect, rust-reviewer, test-engineer: plan :734-735
  as the file stands), at xhigh, counted toward the three-agent limit (condition 10 (c)).

**Conditions (the main session's):**
- **L1. Fold this ruling into the brief by script** (rev 3.3): A1 to A7 (block 4) and the U4E-Q12, Q13 and Q14 words
  (block 5), each old phrase matched with its line breaks read as spaces. I checked each against rev 3.2: each occurs
  once, but "snapshot.rs:29-30" (three times, all replaced). 3.1 stays untouched (its `diff` against carries :147-247
  still empty), and the fold is recorded under the brief's title. If a phrase fails to match, or the fold must change
  any other word, it comes back to me; otherwise there is no second check.
- **L2. The plan and carries text, in the main session's own commit:** u4-build-plan.md's four edits (U4E-Q12's Bites
  line; U4E-Q14's §4.14 row, §5 U4e Tests and -18 lines; each phrase occurs once), allowed by condition 12 through this
  ruling; u4s-carries.md gains entry 66 (block 6) in its "U4d2 or the main session" section, and U4d2's brief names it.
- **L3. The carry-51 commit before launch** (7.2 item 2): `Scene::effect_rows()` and its §4.1 line, then 7.2 item 1's
  counted run gives `<base>`'s counts, every check.mjs step passing. U4e's review round reads its diff from the commit
  before the carry-51 commit, so engine-architect (a new pub item: A19) and rust-reviewer see the accessor with the
  code that uses it.
- **L4. Launch as section 1 says:** one builder (general-purpose, opus, xhigh) in a worktree, under W1-TARGET's lock,
  beside U4d2's lane, with at most three agents at a time counting reviewers.

**For later briefs (no action now):**
- U4r: the class audit selects `catalogue-5/owner-to-all` by that string, and its runs' headers list no mark (U4E-Q13).
- U4f: -22's judged runs write the room's per-client queue length and cap as `tick.v.ack_starved_headroom_count` for
  the never-acker, and select -22/unbounded (U4E-Q12).
- The main session, at U4e's report: if U4e's client drops stale groups, the readings list gains B5's entry
  (advisor-u4j-r2.md:264-268), and carry 62 (g)'s drop count applies.

## What I checked, and what I did not
- **Checked, by reading:** the files in block 1; 3.1 against u4s-carries.md :147-247 by my own `diff` of the two line
  ranges (no output); every phrase this ruling replaces, counted in the flattened brief or plan (each once, but the
  three planned repeats); 14 plan cites (old numbering against today's file, by section 0's note), about 45 code cites
  at the head (seam.rs, snapshot.rs, quant.rs, msg.rs, wrap.rs, hash.rs, input.rs, limits.rs, error.rs, ticks.rs,
  history.rs, world.rs, scene.rs, greybox.rs, link/src/lib.rs, server and client manifests and lib.rs, check.mjs's
  bite and test steps), and R27, R28 and R38 at d50396a; marks.json's -18 and -22 entries; SERIES.md :21, :128, :150,
  :319-320; the catalogue's rows and §6 item 5; the ADR row the brief cites for Q13 (0001:1687-1688).
- **Arithmetic I did:** Q14's send counts (s(k + 30) − s(k) = 10⁹ ns exactly; 32 × 10⁹ / 30 not whole, so 31 or 32
  sends); A2's split gain (1,066.67 − 56 = about 1,010 B against 1,000 B); A3's own-state count (48 × 10 records
  against 255).
- **Not checked:** I built, ran and planted nothing (the main session holds the shared target's exclusive lock now).
  Q14's arithmetic assumes the server sends at U4b's schedule on the injected clock and Loopback delivers at the send
  instant (link/src/lib.rs:128-131), both as merged; the builder's e5a is what shows it. A2 rests on the catalogue's
  budget and `Limits::SNAPSHOT`, not on a built encoder. Whether L1 B1's bursts evict B is U4r's (`burst_eviction`
  logged). I did not re-derive U2's datagram hashes for F2: I read the builder's and the test-engineer's records.

## Decisions needed (Mustafa)
None. Every point here is a technical or test choice inside the documented vision, or a route between lanes: mine to
decide. No word or decision of Mustafa's changes, and nothing he sees, hears or feels is touched.

## For Mustafa
Nothing. (PR #40 stays needs-mustafa for its existing reasons; this ruling adds no action for you.)

