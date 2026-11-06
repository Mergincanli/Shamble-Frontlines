# Advisor review of B.2 plan rev1 (b2-community-servers), 2026-11-06

**Verdict: APPROVE WITH CONDITIONS** (9 conditions below). Docs only, fits the queue (STATUS:33-36), smallest correct
scope, his words check out byte for byte. One premise is wrong (DK-41) and one pointer line overstates the drafts.

## Rulings on the six decisions (plan §7)
- **PLACE: (a), approved.** §5 "Accounts and business" holds progression and revenue; §6 would make it an engine need,
  against his "keep the door open, don't design it" and CLAUDE.md's trace rule.
- **TAG: (a), approved.** He names it "the trust rule" and states it as a rule in both 2026-11-03 messages; under a
  `[LATER]` note it reads "when community servers come"; (b) would decide by label that they come. He confirms marks
  at merge (plan §8).
- **HANDOVER: (b), approved with conditions 3-4.** His 2026-11-04 Factions line answers pillar 5's question at the level
  of principle ("the differences grow with tech"), not unit by unit (B3-Q1 (c) defers that to his unit list; his
  2026-11-03 "Still open: faction asymmetry (Mustafa works on the unit list first)", register :120). The draft's
  parenthetical keeps that residual, and §7:86 stays the open question the register names as owner. The old quote
  "infantry shared, tanks per faction" is false today. Consistent with LOOP_V2 §5, which carries no `[OPEN]` for it.
- **REG: (a), approved** (condition 8). One clause helps whoever decides OQ-0006-21; B3-REG is the precedent.
- **DK-41: REVISE the premise; rule (b), narrowed** (condition 5). PR #40 does not record #41's merge: its STATUS says
  "its readings PR #41 (needs-mustafa: waiting for his go)" and "Open: his go to merge PR #41"; no d8c485a anywhere
  in it (checked at its head 1b5136e). Main's STATUS names neither #40 nor #41. B.2 edits STATUS:3's merge list anyway
  for #39, so one clause for #41 adds no new conflict line.
- **HEADER: (a), approved.** Same form as LOOP_V2:5 and EDITOR_V2:6.

## Conditions (each checkable on the final diff)
1. **The note's pointer line** must not say the drafts already keep anything possible: ADR 0006 part 12 (:514-515)
   says "No field marks a match as official or community ... adding one is design for later". Word it as the open
   question, e.g. "Its open question in the drafts: ADR 0006, OQ-0006-21 (COMMUNITY-SERVERS, his words of 2026-11-03:
   "keep the door open, don't design it")." infra-engineer checks it against part 12 and OQ-0006-21.
2. **The trust rule answers nothing else.** Neither the note, the register clause nor the report may say or imply
   whether private matches (R2-ROOM-OQ7, register :183), bot and private matches in rates (R6-TEL-OQ2), or rented
   "official private servers" count for progression. game-designer checks this explicitly, in addition to LOOP_V2 §8.
3. **Pillar 5's new line:** E1 checks byte for byte only his sentence ("Factions: the counter triangle ... vehicles are
   unique).", wrapped lines joined, tag and markup stripped); the attribution parenthetical differs from LOOP_V2:75 and
   the report lists that as the one deviation. The parenthetical keeps the residual (unit by unit in his unit list,
   with the two fixed rules), as drafted; "(Mustafa, 2026-11-04" stays in it.
4. **The report's "For Mustafa"** says in plain words: pillar 5's `[OPEN]` is closed at the level of principle only;
   which units are shared stays open (VISION_V2 §7, register OQ-0005-CD5, his unit list); he may pick (a), pointers
   only, at his go. The old sub-bullet is in the appendix word for word.
5. **DK-41:** fix the plan's sentence about #40; add #41's merge as one clause in STATUS:3's merge list (d8c485a,
   by merge commit, time from the GitHub API's `merged_at`; the commit time is 2026-11-06 13:35:36 UTC). Everything
   else about #41 (its rows, the ADR answers it carried) stays with the netcode session; the report names #40's two
   stale lines ("waiting for his go") for that session.
6. **STATUS:446:** cite `docs/adr/README.md:294-298` (the section heading through the paragraph that names EDITOR_V2),
   not :293-298 (:293 is blank). The other shifts are right: :99 and :447 -> :119; :465 -> :202; :466 -> :192-193.
7. **ROADMAP:99** (B3's entry: "VISION_V2 lines 36-37 ... 59 ... and 86 ..., for B2") gets the same "taken by B.2
   (PR #42)" as STATUS:102, or the report says why not.
8. **The register clause** is appended to OQ-0006-21's row (:160) only; "No option is weighed and no timing is set."
   stays; a line diff of README shows one changed line. STATUS gains a follow-up for the drafts' next revision: ADR
   0006's OQ-0006-21 entry may point at VISION_V2 §5 (no ADR edited in B.2).
9. **E4 adds line-number citations into VISION_V2:** `docs/research/r1-simulation-core.md:3301` and `:3535` cite
   `docs/VISION_V2.md:54`, already stale (the 100 ms line is :55 today, :56 after the header line). Report it as a
   pre-existing stale citation in a dated research record and leave the research file unedited; E4 confirms no other
   line-number citation into VISION_V2 outside STATUS and ROADMAP (which B.2 rewrites).

## For Mustafa (at his go, already in plan §8; nothing to decide now)
No escalation. At merge he confirms: (1) the note's words and its marks (`[LATER]` note, `[DECIDED]` trust rule);
(2) pillar 5: his Factions line replacing the `[OPEN]` (b), or pointers only (a); (3) that the pointers read his
2026-11-04 words as he meant them. No Brave check (nothing a player sees).

## Evidence, reviewers, budget
- E1-E7 fit a docs change-set; E3's expected counts (`[DECIDED]` +2, `[OPEN]` -1, `[LATER]` +1) match the drafted lines
  (the pointers and the header carry no tag). E6 `--no-smoke` is fine for docs (CI runs smoke).
- Three reviewers at Opus 5.5 high, one round on the final diff, one fix pass, skeptics only for High or above: enough
  and not more. The advisor's final-head review covers the fix pass.

## What I checked (file:line, at the worktree head 8ee1b5b = main ee736ec + notes)
- His words: `git show 4f268da:.checkpoints/kit-v3-3/queue/mustafa-2026-11-03-list.md` :6, :10, :12 match
  mustafa-words.md byte for byte; COMMUNITY-SERVERS matches appendix :2218 and ADR 0006 :1137-1141; B3 notes at c5d86cb
  (plan :34, :82; his words :17, :29-30) match. STATUS:498-506 holds the same B.2 text.
- VISION_V2 (88 lines): :36-37, :59, :86 as quoted; §6.10 `[LATER]` Accounts is the only §6 progression item.
- LOOP_V2 :74-75, :82-84, :136-138, :146 as cited; :5 and EDITOR_V2 :6 header lines as cited.
- ADR 0006 OQ-0006-21 at :1135-1148 (:1149 starts OQ-0006-22; plan says 1135-1149, harmless); part 12 :513-515.
- Register: CD5 :120, OQ-0006-21 :160, R2-ROOM-OQ7 :183, "Needs outside VISION_V2 §6" heading :294 (af8393e: :293);
  CD4 :119 (af8393e :118), R6-TEL-OQ2 :202 (:201), R4-LOS-OQ1/2 :192-193 (:191-192). #41 adds one net line (@@ -13,18
  +13,19).
- STATUS :99 and :447 cite README:118, :446 :292-297, :465 :201, :466 :191-192; B.1 report :25 cites README:118.
- ROADMAP :73 (also stale: "A3, in flight as PR #38, a draft"; the plan rewrites :73), :93, :99.
- Citations of "VISION_V2 §5" (ADR 0001:993, 0002:1028, 0005:521, 0006:42, LEGACY_INVENTORY:84/:366, r1, r2) and
  "VISION_V2 §7" (README:120, ADR 0005:686/:794, r1:2806, r6:834) all still resolve: §5 keeps guest play and team
  colours, §7 keeps faction asymmetry and the snowball. No doc cites pillar 5's `[OPEN]` except STATUS:35/:102 and
  ROADMAP:73/:99. "infantry shared, tanks per faction" appears only in VISION_V2:36 and those queue lines.
- #40: `git diff --stat origin/main...FETCH_HEAD -- docs/` = REPO_MAP, STATUS, docs/design/netcode-proof-messages.md;
  head 1b5136e has merged main at d8c485a but its STATUS still calls #41 waiting for his go.
- Commits: ee736ec parents d8c485a and 4f5976b, committed 17:09:29 UTC; d8c485a 13:35:36 UTC.
- Not checked: the GitHub API's `merged_at` values (no API call made; the build reads them); the gate (nothing built yet).
