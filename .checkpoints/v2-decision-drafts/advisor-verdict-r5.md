# Advisor verdict: plan revision 5 (Mustafa's design answers), v2-decision-drafts

Status: DONE (verdict below the notes)

## Notes
- started 2026-11-03: read the words file, plan r5, verdict r4 (conditions 7-9). Words file = the relayed user request (same 12 bullets + 2 lines). Next: README rows 49-53, 58, 102, 111, 114, 115, 136, lines 23-25, "Design questions"; STATUS; draft entries.
- read README 1-186: lines 23-25 (98 open), rows 49-53, 58, 102 (CD5), 111, 114, 115, 136; "Design questions" (143-184) is a table of 38 DESIGN-owned research ids, none of his answers' ids is in it. So "the README's design questions" can only land as the plan reads it: the rows in "Decisions the drafts ask for" + one sentence in the section. Next: drafts' entries, the name "picks", STATUS.
- (run 2, after a cut) plan r5 on disk is the rewritten one (uncommitted; covers both messages). ANSWERS-2 = this run's relayed message, byte for byte (diff, scratchpad). advisor.md = main's.
- routing checked: e2 F2 (0006 key "loss on each") is test-engineer's text (stop-test-engineer-fixes.md:413), so the plan's routing is right and the verifier's label wrong; e2 F1 key S (0003:1122-1123) test-engineer's (:284-287), part 8 (0003:404-405) graphics-engineer's. e3 F1 (six map rows) is not in the plan, but stop-session-notes.md already holds the six NOTEs: fold into the report.
- gaps found: (1) e2b F1 has a second half (PM-0005-4's controls "tuning table" -> "rule table"), not in the plan; (2) e2b F5's fix ("where blockouts ... are enough", skeptic upheld) leaves the same phrase at SPIKE_PLAN:312-313 (e3b judged it faithful: the two verifiers disagree; the upheld one wins); (3) the plan's gloss "they measure client CPU, not frames" is wrong: PM-0001-43 times presented frames on ML and RD (input-to-screen; feeds OQ-0001-15); (4) SPIKE_PLAN Depends on: item 1 :129, boundary :234, sim core :269 all need ML (and RD, MAC) named, yet he names them "before the look proof test" (4th): the plan only touches the proof test; (5) MP "stands in for none" text at 0001:1366-1369, 0003:821-826, README:53, SPIKE_PLAN:37 must change consistently.
- COMMUNITY-SERVERS vs drafts: 0001 part 11 (room per process, mode id + overrides, Join carries protocol version + content hash, directory records build/version/hash, room writes the record; tokens per environment key id) and 0002 crate table (`server` = the room process with "the supervisor hook"; supervisor runs rooms as programs) and 0005 (record carries hash + mode id) and 0006 part 12 (record shape, no official/community field). Nothing locks it out; the plan's checkers miss 0002 and 0005 (engine-architect a). Related open DESIGN ids: R2-ROOM-OQ7, R6-TEL-OQ2 (not answered by it).
- counts: OQ-0003-1 stays "answered in part" by r4's COUNTS rule (the record needs the named models, like OQ-0001-5's value) -> 115 numbered, 97 open (0006: 21 numbered, 18 open). PM 341 unchanged.
- more checks: SPIKE_PLAN items 1 (:129), 3 boundary (:234) and 4 sim core (:269) "Depends on" ML/RD/MAC named; 5 look (:307) too. ADR 0001: 23 of 63 PM rows name ML, RD or MAC; spike claim 1 "runs end to end on four browsers" and PM-0001-28 (decides R2-TRN-OQ1) need Safari = MAC. So his timing ("before the look proof test") leaves three earlier spikes with rows on machines not yet named: escalation MACHINES-TIMING (non-blocking).
- AB14 options: what it covers (a)/(b) and what ends it (i)/(ii): his "3 s, against spawn killing" picks neither (a) nor (b): answered in part. AB11 second half (Peacekeeper before Tech 1) left to "numbers tuned later". AB15 options (a)/(b)/(c): "enemies can see" = everyone in substance; how (scoreboard or cue) not said.
- OQ-0006-15 labels "answered in part" at 0006:291, :756 (reading), :976, :1346, README:136, SPIKE_PLAN:140, :496; OQ-0006-20 at 0006:1057, README:141, SPIKE_PLAN:143, :496. PM-0006-18 "under the cap OQ-0006-20 sets" stays true.
- e2b F1 second half confirmed: PM-0005-4 says "a tuning table" twice, CD-2 says "the rule tables".
- composing the verdict now.

---

## VERDICT (verbatim): APPROVE WITH CONDITIONS

Plan revision 5 (Mustafa's design answers, his answers to the four escalations, COMMUNITY-SERVERS, the last fix
round), v2-decision-drafts, PR #33 (draft), branch tip 6115116, base main 9e7db28. Advisor agent, 2026-11-03.

**Why, in one line:** the plan keeps his scopes, routes every fixcheck-e finding to the text's real author and honours
"no further re-check rounds". But it misses four things. It over-reads two answers: the reference machines and spawn
protection. Its gloss on "doesn't measure client frames" is wrong for one row. And it leaves three spikes depending on
machines that, by his timing, won't be named yet. The conditions below fix all four, without a new round.

**TL;DR**
- DRAFT-ENTRIES: (a). The design answers go in README and STATUS only. README says the counts follow the drafts.
- OQ-0003-1 stays "answered in part": the classes are chosen, but the actual machines are not named yet. So the count
  is 115 numbered, 97 open (not 96). Pass marks stay at 341.
- "MP is never a measured result" stands. His laptop on its integrated graphics is a separate stand-in for ML, for rough
  checks only. No pass mark is judged on it, and nothing stands in for RD or MAC.
- COMMUNITY-SERVERS: nothing in ADR 0001 part 11 or ADR 0006 locks it out. It gets one open question, OQ-0006-21, and
  one README row. No field, part, pass mark or crate.
- One new decision for the report: three spikes before the look test measure on the laptop, desktop or Mac.

### What I checked myself
- `ANSWERS-2-mustafa-2026-11-03.md` against this run's relayed message: byte for byte the same (diff). My earlier
  run (cut) checked `DESIGN-ANSWERS-…` against the message relayed then: the same. `.claude/agents/advisor.md` here =
  main's.
- The plan on disk (rewritten, uncommitted) against both inputs, verdict r4 (conditions 7 to 9, COUNTS, SPIKE-MACHINES
  (d)), fixcheck e1, e2, e3, e2b, e3b and skeptic-8 (e1-1, e2b-1, e2b-2).
- Authorship, from the stop-*-fixes files:
  - e2 F2's "loss on each" (0006 key) is test-engineer's (stop-test-engineer-fixes.md:413), so the plan's routing is
    right and the verifier's label was wrong.
  - e2 F1's key place (0003:1122–1123) is test-engineer's (:284–287); its part 8 place (0003:404–405) is
    graphics-engineer's.
- README: lines 1–75, 76–141 (AB11, AB14, AB15, OQ-0006-1, -15 to -20) and "Design questions" (143–184): 38
  DESIGN-owned ids, and none of his answers' ids among them. So "the README's design questions" means the rows in
  "Decisions the drafts ask for", plus one sentence in that section. The plan reads it the same way.
- ADR 0005:
  - OQ-0005-AB11 (1351–1361): two questions. His answer covers the first; the second is left to "numbers tuned later".
  - AB14 (1387–1397): options (a)/(b) for what it covers. "Against spawn killing" picks neither.
  - AB15 (1405–1413): options (a)/(b)/(c).
  - CD-2 (319–329): "the rule tables". PM-0005-4 (2273) says "a tuning table" twice.
- ADR 0001:
  - Part 11 (473–531): one process per room; a mode id plus overrides; `Join` carries the protocol version and
    content hash, and the room checks both. The directory records build, version and hash. The room writes the
    record.
  - The message catalogue (663–664, 753).
  - The machine key (1355–1369).
  - 23 of its 63 pass-mark rows name ML, RD or MAC (script over column 4). PM-0001-43 times input-to-screen latency
    through "presented frames on a 60 Hz display" on ML and RD. PM-0001-28 (it decides R2-TRN-OQ1) and spike claim 1
    ("four browsers") need Safari, so they need MAC.
- ADR 0002's crate table (189–193): `server` = "the room process … the supervisor hook"; the supervisor "runs room
  processes as programs".
- ADR 0006:
  - Part 12: "Every match writes one record inside the room".
  - OQ-0006-15 (976–991) and OQ-0006-20 (1057–1075).
  - Every "answered in part" label for OQ-0006-15 and -20 (0006:291, 756, 1346; README; SPIKE_PLAN:140, 143, 496).
  - PM-0006-18 ("under the cap OQ-0006-20 sets") stays true.
- SPIKE_PLAN:
  - Item 1 (:129), item 3, the boundary spike (:234), and item 4, the sim core spike (:269): each "Depends on" ML (and
    RD, MAC) named.
  - Item 5, the look test (:307): also depends on them.
  - "Blockouts": the same phrase at :312–313 as ADR 0005:1498–1500.
  - "Not a session's" (:326–327, :520).
- The MP sentence "stands in for none" sits at 0001:1366–1369, 0003:821–826, README:53 and SPIKE_PLAN:37.
- e3 F1: `stop-session-notes.md` already holds NOTEs for the six map rows (N7, O23, L14, T2, T4, LA4).
- Not run: the gate. This is a plan review and nothing is edited yet. The gate runs on the final head at the PR
  review.

### Conditions
1. **His words and their labels (README rows, STATUS).**
   - Quote word for word from the two inputs files. Every label is no stronger than his words:
     - **AB1:** answered. "Assumed, to confirm: the class's own abilities stay." is kept as his words, and no one is
       named to confirm it.
     - **AB2:** still open, his three options in his words. "Limited but generous, refilled at the barracks and supply
       points" is marked as the Advisor's lean, never as an answer. The row keeps its LATER marker.
     - **AB3:** answered.
     - **AB5:** "leaning towards class variants, with more variants added over time. Not final." It stays open.
     - **AB11:** answered for the Shield and the blade, in his words. The Peacekeeper's strength before Tech 1 is left to
       his "numbers tuned later" (the advisor's reading, 2026-11-03).
     - **AB14:** answered in part.
       - Mustafa's: 3 s, "against spawn killing".
       - The Advisor's pencil, which Mustafa may veto: "Firing ends it early".
       - Still open: what it covers (ADR 0005's options (a) damage only, (b) every effect).
     - **AB15:** "enemies can see a player's build".
       - Placing it at AB15 is the advisor's reading (2026-11-03).
       - How they see it is not said: the scoreboard or a cue; a cue is ADR 0003's to draw, and the look is his.
     - **Fragments:** "visual only for now", plus his sentence.
   - The name "amplifications" ("amps") goes only in README's one sentence and in STATUS. No GLOSSARY entry, no
     renaming in the drafts.
   - "Still open" items:
     - Faction asymmetry ("Mustafa works on the unit list first") goes beside OQ-0005-CD5.
     - "The war around the matches" is quoted as his words and mapped to no id.
   - Rows answered now drop their "LATER" marker.
   - DOMAIN and REF-MACHINES quote his second message (about 17:31). Nothing in the docs says he "dropped" the
     fallback. The appendix carries both messages byte for byte.
2. **DRAFT-ENTRIES (a), said once in README.**
   - One sentence beside the counts says the open counts follow the drafts' own entries.
   - README records Mustafa's design answers of 2026-11-03 for AB1, AB3, AB11 and AB15, AB14 in part, and a lean for
     AB5. ADR 0005 (and ADR 0001 part 12 for AB15's visibility class, ADR 0003 for the fragments) take them up at
     their next revision.
3. **REF-MACHINES (OQ-0003-1), one wording in every place.**
   - The places:
     - ADR 0001's key (test-engineer);
     - ADR 0003's key (test-engineer);
     - ADR 0003's OQ-0003-1 entry and its migration line (graphics-engineer);
     - README's row and the ADR 0003 count (engine-architect b);
     - SPIKE_PLAN's machines paragraph (engine-architect b).
   - The wording:
     - Label: "Answered in part by Mustafa (2026-11-03)", with his classes quoted. ADR 0003's own class text stays
       beside his words, not rewritten.
     - His laptop: "Until then, my laptop's integrated graphics (Brave set to "Power saving") stands in for the
       minimum, for rough checks only."
     - The reading: "A rough check is reported, never judged against a pass mark. MP stays what he judges, never a
       measured result, and nothing stands in for RD or MAC (the advisor's reading, 2026-11-03)."
     - Still open: the actual machines, named by Mustafa before the look proof test.
   - Every "stands in for none of ML, RD and MAC" sentence goes. Call it "the stand-in for ML", with no new machine
     code.
4. **"The netcode proof test doesn't measure client frames."**
   - It is quoted once, in ADR 0001's key (test-engineer), with this reading: no client frame-time budget is judged in
     the proof test. PM-0001-43 times input-to-screen latency through presented frames on ML and RD. Like every row
     on ML, RD or MAC, it is judged only on a named machine (the advisor's reading, 2026-11-03).
   - Don't write the plan's gloss "they measure client CPU, not frames". No pass-mark row changes; marks stay at 341.
   - SPIKE_PLAN items 1, 3 and 4 (engine-architect b) replace "ML, RD and MAC named (OQ-0003-1, still open)" (and the
     items' equivalents) with one clause: "the machines its rows name (OQ-0003-1, answered in part: Mustafa names them
     before the look proof test; until then the stand-in for ML, for rough checks only); this spike's own plan lists
     its rows that run before they are named, and none of them is judged on the stand-in (the advisor's reading,
     2026-11-03)".
   - Item 5 keeps "named and on hand".
   - SPIKE_PLAN:495 ("still to be named") gets the same label.
5. **OQ-0006-15 and OQ-0006-20 answered.**
   - Every "answered in part" label for them changes: ADR 0006 (infra-engineer), README and SPIKE_PLAN
     (engine-architect b).
   - OQ-0006-15: domain and registrar in his words. The registration falls before the netcode proof test (r4's
     reading stays).
     - The DNS host and prices stay "not researched". No new claim about Cloudflare without a source.
     - "Production uses the same domain" may be written only as the advisor's reading, 2026-11-03 (it is the game's
       name, and part 6 needs one registrable domain).
   - OQ-0006-20: his rule in his words. PM-0006-18 is unchanged.
6. **COMMUNITY-SERVERS.**
   - **OQ-0006-21** (infra-engineer) quotes his whole bullet, "possibly with revenue such as official private servers
     they rent" included, uninterpreted. Its fields:
     - Options: none weighed ("keep the door open, don't design it").
     - Decider: Mustafa. No timing set.
     - Related open design questions it does not answer: R2-ROOM-OQ7 and R6-TEL-OQ2. Progression lives under VISION_V2
       §6.10 [LATER].
   - It adds no engine feature: no part, field, pass mark, crate or "Needs outside §6" entry.
   - **README** gets one row in "Decisions the drafts ask for", pointing to OQ-0006-21.
   - **Each check gives a disposition** ("no change: <reason>" or one pointer):
     - netcode-architect: ADR 0001 part 11 and the message catalogue.
     - infra-engineer: ADR 0006 parts 4 to 6 and 12. The one pointer expected is part 12's match record.
     - engine-architect (a): ADR 0002's `server` and `supervisor` rows (the room process must not need the supervisor
       or the lobby to start), and ADR 0005's mode, content-hash and record text.
   - A pointer outside ADR 0006 goes only where a sentence would otherwise read as closing the door.
   - ADR 0003 and 0004: the session greps "room process|match record|supervisor" and writes one note line. No author
     needed.
7. **Fix-round gaps.**
   - (a) **e2b F1, both halves** (test-engineer, PM-0005-4): the kinds become "units, vehicles, weapons and factions",
     and both "tuning table" become "rule table", as CD-2 has it.
   - (b) **e2b F5's twin** (engine-architect b, SPIKE_PLAN:312–313): the same fix as ADR 0005:1498–1500 ("…, where
     blockouts with a skin, named colour slots and LOOK_V2's proportions are enough"). Skeptic-8 upheld it; e3b's
     "faithful" note on the same phrase is overruled.
   - (c) **e3b 3:** I don't confirm "not a session's" ("with AI help" may be a session's help). Drop it at
     SPIKE_PLAN:326–327 and :520. "No spike's size carries it" may stay as the plan's own sizing.
   - (d) **e2b F4** (game-designer): the pointer form, after "once it can be read": "; those claims are the GAS findings
     AB-1 to AB-3 cite, F4's summed multipliers and F17's cooldown quote among them". No re-listed id ranges.
   - (e) **e3 F1:** the six NOTEs in `stop-session-notes.md` go into the report's "At the stop" table. No author round.
8. **No re-check rounds (his words), with these limits.**
   - No fixcheck-f and no skeptics.
   - The session runs: the write guard, dispositions, glossary, tables and cell counts, the open-question and pass-mark
     counts, the gate. Two more scripts:
     - every quote attributed to Mustafa 2026-11-03 is found verbatim in one of the three inputs files;
     - a label grep: no "answered in part" left on OQ-0006-15 or -20; OQ-0003-1 "answered in part" everywhere; no
       "stands in for none"; "tuning table" gone from PM-0005-4.
   - What is left goes into the report as notes.
   - My review of the final head (gate, CI, these scripts' output, the fix round's blocks) is the merge rule's review,
     not a round. If it finds a misquote, a misattribution of his words, or a failing gate or script, that gets a
     one-phrase fix, listed in the report. Anything else becomes a note.
9. **Close as r4 conditions 7 to 9, unchanged.**
   - The appendix also carries, byte for byte:
     - `DESIGN-ANSWERS-…` and `ANSWERS-2-…`;
     - this verdict;
     - the six NOTEs.
   - The report's "Decisions needed" holds MACHINES-TIMING (below). Its "For Mustafa" is the list below.
   - STATUS "Next" records two later steps:
     - LOOK_V2 §8 reads "Spline or Blender" once the asset research's export part reports. His words fix the text; the
       edit belongs to that part's change-set and goes to him for the merge.
     - The drafts take up the design answers at their next revision.
10. **Models and effort:** Opus for every agent. The authors' blocks at xhigh; docs-keeper-4 at high.
END CONDITIONS

### Rulings
- **DRAFT-ENTRIES:**
  - (a), on condition 2. It is his scope ("Record them in the README's design questions and in STATUS"), and it means
    fewer unverified edits after his stop.
  - The escalation answers differ: they close questions the drafts' own entries carry, so the drafts take them, as the
    plan says.
- **OQ-0003-1:**
  - Answered in part, not answered. The record needs the named models ("each is recorded with GPU, CPU, RAM …"), as
    OQ-0001-5 needs its value. Under r4's COUNTS rule it stays open.
  - It no longer blocks the netcode proof test (his words).
- **"Doesn't measure client frames":**
  - Recorded as his words, with condition 4's reading. It removes the named machines from the proof test's start, not
    from its rows.
  - The plan's gloss is wrong for PM-0001-43, which times presented frames.
- **MP and the stand-in:** MP's "never a measured result" is unchanged. The stand-in is a different setup with its own
  limit ("for rough checks only"), so no key's rule changes. Nothing judged on the stand-in counts as a result.
- **Counts after the round (by script):**
  - 115 numbered, 97 open.
  - ADR 0003: 20 numbered, 17 open (-1 answered in part).
  - ADR 0006: 21 numbered, 18 open (-15, -17, -20 answered; -5 answered in part; -21 new).
  - ADR 0005: still 33 (condition 2). Pass marks: 341.
- **COMMUNITY-SERVERS against ADR 0001 part 11 and ADR 0006:** no lock-out found.
  - (1) "One self-contained program with a config file": a room is already one process per match, started with a mode
    id plus overrides. A file holding them is a second way to start it. The only thing to watch is ADR 0002's
    "supervisor hook", which must not be required to start (condition 6).
  - (2) "Checks the game's version and content hash": `Join` already carries both, and the room checks them.
  - (3) "Marks each match as official or community": part 12's record has no such field. A pointer keeps the door open;
    adding the field is design, for later.
  - Token keys per environment and official-only storage keys fit "progression … on official servers only".
- **Verification stop:** honoured (condition 8). The scripts and the gate are not re-check rounds, and neither is my
  final-head review.
- **Routing:** right for every finding, including e2 F2 (test-engineer's text, not infra-engineer's). It misses only
  condition 7's gaps.
- **LOOK_V2-§8:** LOOK_V2 is not edited in this PR. ADR 0005's Settled inputs and SPIKE_PLAN's research section quote
  his answer. "Blender for the test models only" agrees with the drafts' readings (the export part researches both
  tools' export).
- **His scopes:** kept. The design answers go in README and STATUS. COMMUNITY-SERVERS goes in ADR 0006 and README, with
  at most a pointer elsewhere.
END RULINGS

### Findings (ranked)
1. Three spikes before the look test (the netcode proof test for Safari; the sim core and boundary spikes for the
   laptop and desktop) depend on machines that, by his timing, are named only before the look test (escalation).
2. OQ-0003-1 marked "answered" would claim the machines are named. It is "answered in part" (count 97, not 96).
3. AB14 "answered" over-reads "against spawn killing" as a choice of what protection covers (condition 1).
4. "They measure client CPU, not frames" is wrong for PM-0001-43 (condition 4).
5. Missed in the routing: e2b F1's "tuning table" half, the SPIKE_PLAN blockout twin, and e3 F1's NOTEs (condition 7).
6. COMMUNITY-SERVERS' check left out ADR 0002's `server` and `supervisor` rows and ADR 0005 (condition 6).
7. docs-keeper-4's effort is not stated (condition 10).
END FINDINGS

### Decisions needed (for the report; none blocks this plan or the netcode proof test's start)
- **MACHINES-TIMING**
  - The question: you name the laptop, desktop and Mac before the look proof test, but three earlier spikes measure on
    them. Should they wait for the machines, or run first and be judged later?
    - The netcode proof test needs the Mac for Safari: its four-browser claim and the WebTransport library choice.
    - The sim core spike and the boundary spike measure on the laptop and the desktop.
  - Why it matters: until those machines exist, those rows can only be rough checks. A row judged later costs a short
    re-run, with VM hours and a session.
  - Options:
    - (a) Borrow the Mac before the netcode proof test, and the laptop and desktop before the sim core spike.
    - (b) Keep your timing. Those rows run as rough checks first (the laptop rows on your stand-in, the Safari rows held),
      and a short re-run judges them once the machines are named. The library choice waits for Safari.
    - (c) A mix: the Mac early, the laptop and desktop on your timing.
  - Recommendation: (c). Safari is the one browser where WebTransport is new, so the library choice should see it. The
    laptop and desktop rows can wait for one re-run.

### For Mustafa
1. Check in your Hetzner console that a CCX23 and a CX33 can be ordered in an EU location (OQ-0006-1, still open).
2. Register shamblefrontlines.com at Cloudflare before the netcode proof test.
3. Borrow and name the laptop, desktop and Mac: at the latest before the look proof test, earlier as MACHINES-TIMING
   says.
4. OK each run's cost cap when its plan reaches you.
5. Once the advisor has approved PR #33's final head and gate and smoke are green on it, say "merge #33" in this
   session's chat. It merges by squash only.

Nothing in Brave: nothing a player sees changes.
END VERDICT

Status: DONE
