# Advisor review: B3 plan, revision 1

## Review plan
1. Read main's advisor.md (plan-review section) and CLAUDE.md (protocol, reporting rule, Models and effort). Done.
2. Read the spec (his words) and the plan rev1. Done.
3. Check (a): every one of his 17 items against the plan's new text, byte for byte, and each marker.
4. Check (b): read LOOP_V2, UNITS, DESIGN_NOTES, adr/README, STATUS in the worktree; verify each "old" anchor exists;
   look for conflicts the plan missed; settle the open questions the docs can settle.
5. Check (c): B3-TAG, B3-HOME, B3-SCOPE, B3-SUPERSEDE against the files (section numbers, citations, STATUS lines).
6. Check (d) and (e): missing pieces; reviewers and effort.
7. Verdict, conditions, Decisions needed, For Mustafa.

## Notes as I go
- Main's CLAUDE.md differs from the session copy only in the night window, routine effort and web rules; nothing here
  depends on those. Worktree HEAD 76c35bf = main 868e165 + PR #35; #35 touches README, STATUS, REPO_MAP, not LOOP_V2,
  UNITS or DESIGN_NOTES (git diff --name-only origin/main...HEAD).
- (a) All 17 items compared with the plan's new text: verbatim, markers right. Every "old" anchor in LOOP_V2 (lines 6-7,
  23, 59-60, 64-68, 73-79, 82, 94-95, 105, 113, 134, 163-167) and UNITS (5, 10, 137, 149, 151) exists as quoted.
- 2026-11-03 words checked against the appendix (docs/reports/2026-11-01-v2-decision-drafts-appendix.md lines 195-202):
  "Upgrade visibility: enemies can see a player's build." and the name line are his words; STATUS's "the advisor's
  reading" is about placing it at AB15 only. Carried 2026-10-31 lines match STATUS 236-240.
- Q-POWER duplicates OQ-0005-AB7 (README row 119; ADR 0005 line 1297, whose "Why it matters" already weighs "Tiers are
  sophistication, not damage" and finds it silent on picks). Q-CARDS duplicates OQ-0005-AB8 (ADR 0005 lines 1308-1318:
  "what happens when a track has fewer left than the offer's size (3)"). Both are routed, not new.
- Q-LBD: settleable as a reading. The 2026-11-03 levelling answer is [DECIDED] and not plainly replaced; the same walk
  says "when a gun levels up". "Learn-by-doing" appears in no doc.
- Q-UNIQUE (vs [DECIDED] Juggernaut "shared", LOOP 82) and Q-UNLOCK (vs [DECIDED] "more classes", LOOP 105, with his
  2026-11-03 lean "class variants") are real conflicts between his own decisions: his. Q-TECHBLD and Q-DOWNED are new
  rules for a [LATER] unit: his, no hurry. "Tech building" and "downed" appear in no v2 doc.
- Register convention (README line 43): readings are marked "(the advisor's reading, <date>)", not "subject to".
- The 10-31 line "no weapon choice when buying" carried into §13 sits beside §8's third gate: OQ-0005-AB5 needs a pointer.
- Sprint exists (GAME_DESIGN line 62), so "sprint-to-fire" has a base. Engine-architect is xhigh by its file: oversized
  for pointer lines the verifier already checks.
- The spec lives in the scratchpad: the PR Advisor cannot reproduce E1 unless the report carries his message.

---

## Verdict: APPROVE, with conditions

The plan records his 17 items word for word, under the right markers. Every conflict with older text is either
plainly replaced and listed, or marked `[OPEN]` beside the text. The four plan-level choices are right. Three of the
seven open questions do not need a new question: two are already in the register, and the docs settle the third as a
reading. Apply conditions 1-11 while building; no revision 2 is needed.

**What I checked:** every new line in §4 and §5 against his message (`b3-spec-2026-11-04.md`), byte for byte. Every
old anchor in LOOP_V2, UNITS and DESIGN_NOTES against the worktree (76c35bf): all present as quoted. The carried
2026-10-31 and 2026-11-03 lines against STATUS 185-199 and 236-240 and the appendix's word-for-word record (lines
195-202). The 11 register rows and OQ-0005-AB4, AB7, AB8 and AB14 in ADR 0005. That no doc cites a LOOP_V2 section
above §12 or a UNITS section number, so §13 and UNITS §0 renumber nothing. The `[LATER]` count (6). The agent files'
effort on main. **Not checked:** E1-E10 themselves (nothing is built yet), and the 22 ids in R0's "bears on" list one
by one (condition 7).

### Rulings on (a)-(e)
- **(a) His words and markers:** right. Items 1-12 are `[DECIDED]`, 13-16 `[IDEA]`, the Open item `[OPEN]`. "Parked"
  and "first sketch, to iterate" stay his words with no tag. The `[LATER]` that moves from the Spy to the infiltrator
  line decides nothing new: infiltrators enter buildings, which §9 marks `[LATER]`, and §11's v0 list already leaves
  the Spy out. One small slip: in L14 the "(§13)" pointer sits inside his sentence (condition 9).
- **(b) Conflicts:** all handled. Of the seven questions:
  - **Q-LBD:** I settle it as a reading (condition 1). His [DECIDED] answer of 2026-11-03 says guns level from "XP ...
    while using that unit or weapon". Nothing in the walk plainly replaces that, and the walk itself says "when a gun
    levels up". So "no learn-by-doing layer" rules out a separate layer, not gun levelling.
  - **Q-POWER:** routed, not new (condition 2). It is OQ-0005-AB7, whose decider is Mustafa. The ADR already weighed
    "Tiers are sophistication, not damage" and found that it doesn't name in-match picks.
  - **Q-CARDS:** routed, not new (condition 3). It is OQ-0005-AB8's "what happens when a track has fewer left than the
    offer's size (3)".
  - **Q-UNIQUE and Q-UNLOCK:** truly his. Each sets two of his own decisions against each other: Juggernaut "shared"
    against "vehicles are unique", and "more classes" against "class variants, cosmetics" (with his 2026-11-03 lean
    towards class variants).
  - **Q-TECHBLD and Q-DOWNED:** truly his (new rules), but they concern a `[LATER]` unit and block nothing. They stay
    `[OPEN]` in the docs and go to him as one low-urgency entry.
- **(c) Plan-level choices:** all four approved.
  - **B3-TAG:** a new `[IDEA]` tag. His message says "marked decided or idea".
  - **B3-HOME (a):** §13 appended and UNITS §0 inserted, which renumbers nothing.
  - **B3-SCOPE (c):** the carried lines are his decided words, dated. Without them, §13 doesn't read on its own. One
    pointer is missing (condition 4).
  - **B3-SUPERSEDE (a):** one list in §12, in the history-keeping style DESIGN_NOTES already uses.
  - **The smaller choices:** B3-MARKS, B3-DN (a) and B3-VISION are approved. B3-REG (a) is approved with conditions 6
    and 7.
- **(d) Missing:** the AB5 pointer (condition 4); his message in the repository, so the PR Advisor can rerun E1
  (condition 5); the register's own label convention (condition 6); the night queue entry and the agents' note paths
  (condition 10).
- **(e) Reviewers:** drop engine-architect (condition 8). Its file runs it at xhigh, which is oversized for twelve
  pointer lines that the verifier already checks against the ADR questions. Keep advisor, verifier, game-designer and
  docs-keeper, all Opus 5.5 at high, as stated.

### Conditions
1. **Q-LBD becomes a reading.** In §13 (L17), replace the Q-LBD `[OPEN]` sub-bullet with:
   `  - [DEFAULT] The Advisor's reading (2026-11-04): "no learn-by-doing layer" rules out a separate layer that grows a
   class or a player from use; guns still level from XP earned while using them (the levelling line below), as the same
   walk says "when a gun levels up". Mustafa may veto it.`
   Drop Q-LBD from §8, with the tag in backticks as elsewhere.
2. **Q-POWER is routed to OQ-0005-AB7.**
   - In UNITS §0, replace its `[OPEN]` sub-bullet with an untagged pointer: `  - Whether amps may add raw power is
     OQ-0005-AB7 in docs/adr/README.md (Mustafa's); these ladders bear on it: faster reloads, a drum mag, and hollow
     points and slugs, whose damage gain comes with a cost.`
   - N4 becomes: `→ *Whether this covers in-match amps is OQ-0005-AB7 (docs/adr/README.md); Mustafa's first amp
     ladders bear on it (UNITS.md §0, 2026-11-04).*`
   - R8 adds "B3's power question is this one".
   - Move Q-POWER to §8.4's routed table.
3. **Q-CARDS is routed to OQ-0005-AB8.**
   - In §13, replace its `[OPEN]` sub-bullet with an untagged pointer: `  - What fills the other cards when a ladder
     level names fewer than three amps (UNITS.md §0) is OQ-0005-AB8's question in docs/adr/README.md: whether the
     candidates always differ, and what a track with fewer candidates than the offer's size does.`
   - UNITS §0's card line points to that pointer.
   - R9 names it.
   - Move Q-CARDS to §8.4.
4. **The AB5 pointer.** In §13, after "A class always comes with its own weapon: no weapon choice when buying.
   (2026-10-31)", add: `How an account-unlocked weapon then reaches a match (§8's third gate) is open: OQ-0005-AB5
   in docs/adr/README.md.`
5. **His message in the repository.** The report (`docs/reports/…`) carries his 2026-11-04 message word for word as
   an appendix. E1 runs against that copy, so the PR Advisor can reproduce it from git alone.
6. **The register's labels follow its own convention** (README line 43). Replace "subject to the Advisor's reading"
   with "(the advisor's reading, 2026-11-04)". My readings for these rows:
   - **AB1:** its "Assumed, to confirm" half is confirmed by "Looting keeps the class ability" (no longer "in part").
   - **AB14:** what ends it early is now his words, and the Advisor's pencil is his decision. What it covers, and
     whether an ability press counts as firing, stay open.
   - **AB2, AB4, AB8, AB10 and OQ-0004-12:** answered in part.
   - **AB3, AB7, CD5 and OQ-0004-10:** "bears on".
   No row loses text.
7. **R0's "bears on" list is verified one id at a time.** The verifier reads each of the 22 ids' question and keeps
   only those where his words plainly touch it. It lists the dropped ones in the report.
8. **Reviewers:** remove engine-architect. The verifier (high) also confirms that no ADR decision text moved and no
   register row lost words.
9. **L14:** put the pointer after the attribution, so his sentence stays whole: `Bots can loot weapons and pick
   amps. (Mustafa, 2026-11-04; §13)`.
10. **Process:**
    - Agents write their notes to `.checkpoints/b3-design-walk/<agent>-<part>.md`.
    - Once these conditions are folded into the plan file, the work goes into the Night queue (issue #16) with a link
      to this verdict. It is gated on PR #35 being merged; a night run never merges.
11. **Evidence updates:**
    - E4: "Open since 2026-11-04" is 4 in LOOP_V2 (Q-UNIQUE, Q-TECHBLD, Q-DOWNED, Q-UNLOCK), 0 in UNITS, 4 in all.
    - E5: add one new `[DEFAULT]` in LOOP_V2 (condition 1), and no other new `[DEFAULT]`.
    - The §8 and Counts sections now say 4 open questions, 12 gaps routed and 3 tensions.

The PR will likely be labelled needs-mustafa: it rewrites rule text in LOOP_V2 and adds one Advisor reading. That
makes it his reconfirmation, not a Brave check. The PR Advisor decides.

## Decisions needed
- **B3-Q1: Which of today's shared units become faction-specific?**
  - *The question:* On 2026-11-04 you decided that tier-two specialists and vehicles are unique to each faction.
    Today the Javelin (rockets) and the Officer are shared tier-two units, and you decided earlier that the Juggernaut
    "stays as the shared heavy walker".
  - *Why it matters:* the Javelin is the only listed answer to vehicles. The rules promise every counter role from
    level 1, so whatever changes, each faction still needs an anti-vehicle unit.
  - *Options:*
    - (a) All three become per-faction (each faction gets its own version).
    - (b) All three stay shared, and "unique" means each faction's own extra units.
    - (c) Decide unit by unit in the unit list you are writing.
  - *Recommendation:* (c), with one rule kept: every faction keeps an anti-vehicle answer from level 1. Until you
    answer, the docs keep them shared and mark the question open.
- **B3-Q2: Are new classes still account unlocks?**
  - *The question:* Your 2026-11-04 line says account progression gives "class variants, cosmetics". Your older line
    says unlocks give "more classes, sidegrade variants, cosmetics".
  - *Why it matters:* it decides whether a new class is earned on your account or only reached through the team's
    tech in a match.
  - *Options:*
    - (a) Only class variants and cosmetics; new classes come through tech in the match.
    - (b) Your list was examples; new classes can still be unlocked, as sidegrades.
  - *Recommendation:* (a). It is your newest and narrowest wording, and it matches your 2026-11-03 lean towards
    "class variants". No hurry: the unlock catalogue is not built yet.
- **B3-Q3: Two infiltrator rules (no hurry: infiltrators come after the prototype).**
  - *(i) The tech building:* the base has a barracks, a refinery, a power plant and an HQ, and tech comes from uplinks.
    Which building is the "tech building": a new one, or an existing one such as the HQ? And does "cuts their power"
    work like a destroyed power plant (prices rise) for a set time?
  - *(ii) "Downed":* no rule has a downed state; a soldier dies and respawns. What is "a downed enemy" for the KGB
    agent: a short downed state before death (a big change that invites revives), or a short window on a fresh body?
    And does the reveal show the downed enemy's team only, or both enemy teams?
  - *Why it matters:* one of the three targets and the Soviet verb don't exist in the rules yet.
  - *Recommendation:* a new tech building, with a power cut that works like a destroyed plant for a set time; and a
    short window on a fresh body, revealing that enemy's team only. These are the smallest rules that keep your words.

## For Mustafa
1. Nothing to check in Brave: B3 changes docs only.
2. Answer B3-Q1 to B3-Q3 whenever you like. B3 doesn't wait for them, and the docs mark them open until you answer.
3. Later: when B3's pull request is ready and approved, you'll likely be asked to reconfirm its merge (it rewrites
   rule text in the core loop). Say "merge #<n>" in a session's chat if the record matches your walk.
