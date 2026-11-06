# game-designer review, B.2 (b2-community-servers), final diff ee736ec..26a5455

## Plan
1. Pillar 5's new [DECIDED] line vs LOOP_V2 §5 (:67-105), UNITS.md §0, the walk (c5d86cb); §7 and register OQ-0005-CD5 (README:120) consistency.
2. Pointers on §5 account line and §7 faction question vs LOOP_V2 §8 (:128-148) and §5.
3. The note's trust rule vs LOOP_V2 §8; C2: private matches (R2-ROOM-OQ7, README:183), rates (R6-TEL-OQ2, README:202), rented official private servers.
4. Other VISION_V2 lines his 2026-11-04 words now contradict (§1 weapon unlock vs AB5; pillar 7; §6.10; §3 private match line).

## Findings (as confirmed)

### Checked, no finding
- **Q1 pillar 5 (VISION_V2:38-40).** His sentence matches LOOP_V2:74-75 and his walk (c5d86cb mustafa-2026-11-04.md:17)
  byte for byte (wrapped lines joined: 1 hit in each). The parenthetical keeps the residual in his words of B3-Q1 (c)
  (walk :29; LOOP_V2:82-84; UNITS.md:24-25): "decided unit by unit in the unit list, with two fixed rules". It decides
  nothing about any unit: the Javelin, the Officer and the Juggernaut stay where LOOP_V2:84-87 and :108-109 put them, and
  VISION_V2 names none of them as shared or unique. Its `[DECIDED]` tag is his (LOOP_V2:74). §7 (VISION_V2:99-101) keeps
  the open question's words, so the register's OQ-0005-CD5 owner cell (README:120, "VISION_V2 §7, faction asymmetry")
  still resolves, and CD5's own text ("Still open: faction asymmetry (Mustafa works on the unit list first)") agrees
  with §7's new "waits on his unit list". Tags: `[DECIDED]` 10->12, `[OPEN]` 4->3, `[LATER]` 4->5, others unchanged
  (matches E3's expectation).
- **Q2 pointers.** §5 (VISION_V2:62-65): the quote matches LOOP_V2:137-138 byte for byte; "the unlock catalogue itself
  is still for later" = LOOP_V2:146 `[LATER]`. The original line's words and tags are untouched. §7 (VISION_V2:100-101):
  "Which units are shared or per faction waits on his unit list" = B3-Q1 (c) (LOOP_V2:82). Neither adds a rule; neither
  mentions power, gates or prices beyond LOOP_V2 §8.
- **Q3 trust rule vs LOOP_V2 §8.** "Progression counts on official servers only" restricts where account progression
  (Service points, LOOP_V2:132-135; the account level, :140) accrues; it grants nothing and changes no share. LOOP_V2:132's
  "After the match, the winning team keeps a share..." holds for every match there is today (all official). No
  contradiction. C2: no line of the note (VISION_V2:67-73) or the register clause (README:160) says private matches,
  bot matches, balance rates or rented official private servers count or don't count. "Only" is a necessary condition,
  never a sufficient one. The ADR entry the note points at (0006:1146-1148) already lists R2-ROOM-OQ7 and R6-TEL-OQ2 as
  "related open design questions it does not answer". C2 met. See finding 1 for the one reading risk.
- **Q4 §1 "Finally unlocked that weapon" (VISION_V2:24).** I agree with leaving it to OQ-0005-AB5 (README:66): it is a
  player quote in his words, the AB5 row cites both §1 and his 2026-11-04 line, and it is reconcilable (a class variant
  can carry another weapon; his 2026-11-03 lean was "class variants"). Pillar 7 (:42), §6.7 (:90) and §6.10 (:94) stay
  true under "class variants and cosmetics only"; pillar 3 (:33-34) agrees with "nobody is ever missing a counter".

### Findings
1. **Low. The note does not flag its residual where the wording invites a reading.** VISION_V2:69 ("renting out
   official private servers") sits two lines above :70 ("progression counts on official servers only"); a reader can
   take a rented official private server as an "official server" and so as counting. Nothing says it (C2 is met), and
   the pointer at :72-73 leads to ADR 0006:1146-1148, which names R2-ROOM-OQ7 and R6-TEL-OQ2. Pillar 5 got a residual
   parenthetical; the note did not. *Fix (optional, B.2's own words only, in the pointer line):* "...don't design it");
   it does not say whether private matches, bot matches or a rented official private server count: R2-ROOM-OQ7,
   R6-TEL-OQ2." Or defer: the report's "For Mustafa" names it when he confirms the note's marks.
2. **Low. VISION_V2 §7 does not list his other 2026-11-04 open question.** "Open: the war around the matches, in its own
   design session" (walk :24; LOOP_V2:147 `[OPEN]`) is in LOOP_V2 but not in VISION_V2 §7 (:96-103), which lists open
   design questions "for later sessions". Not a contradiction, and outside B3's three handed-over lines. *Fix:* a STATUS
   follow-up for VISION_V2's next update (Mustafa and the Advisor own the doc), not a B.2 edit.

## Verdict: PASS WITH NOTES (2 Low; no Critical, High or Medium)
