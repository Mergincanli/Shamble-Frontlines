# game-designer review: vfx-xp-notes at e967196 (base 4c6db5b), 2026-11-07

Role read from `origin/main:.claude/agents/game-designer.md` and `origin/main:CLAUDE.md`. Writes only this file.

## Review plan (written first)
1. His words (mustafa-words.md) against the XP block in VISION_V2 §5 and the VFX §7 in EDITOR_V2.
2. XP: no mapping to Service points, account level or LOOP_V2:242's weapon XP; the XP-TERM follow-up's list of what
   "XP" names (LOOP_V2:242, README:69, DESIGN_NOTES:23, legacy career XP) checked against the files; fit with LOOP_V2
   §8 (account progression, trust rule) and §10 (bots).
3. Register clauses R2-ROOM-OQ7 (README:183) and OQ-0006-21 (:160): "bears on it", not "answered"; telemetry open.
4. VFX principles vs feel kit (LOOP_V2 §7, Feel Lab, README:122 OQ-0005-CD7): not "the VFX editor is the Feel Lab";
   hit-stop / camera shake vs LOOP_V2 §7's split feedback: note tension only.
5. Contradictions in LOOP_V2, VISION_V2, UNITS left unhandled.

## Findings (added as confirmed)

### What I checked and found right
- His words: VISION_V2:71-75 equal mustafa-words.md item 2 (header words "on progression in private (rented) matches",
  both lines whole after their tag, "Open," kept beside `[OPEN]`). EDITOR_V2:115-134 (§7) hold his ten lines whole,
  in his order. The worktree is e967196, base 4c6db5b; LOOP_V2, UNITS, LOOK_V2 and DESIGN_NOTES are unchanged
  (`git diff --quiet`).
- No mapping of "XP": VISION_V2:71-75, README:183 and STATUS:492-497 and :505-510 map his "XP" to nothing. README:183
  ends "Which progression "XP" means, and telemetry, stay open". README:160 adds the XP rule to what the note "gives",
  not as an answer. README:122 ends "Whether it and the Feel Lab are one tool is not said."
- The XP-TERM list, against the files: LOOP_V2:242 (`[DECIDED]` levelling XP) yes; LOOP_V2:237-238 (tracks reset each
  match) yes; README:69 (OQ-0005-AB3 quotes it) yes; DESIGN_NOTES:23 ("account XP", marked superseded at :24-25) yes;
  LEGACY_INVENTORY:302 (GE15, career XP) yes; LOOP_V2:140 ("the existing career rank") yes.
- Scope: the `[DECIDED]` rule is headed "private (rented) matches", so public matches with bot fillers (VISION_V2:52,
  LOOP_V2:22, §10) are untouched; only the `[OPEN]` suggestion (1) speaks of them.
- Hit-stop already lives outside the sim: ADR 0004:222-223 ("hit-stop never stop[s] the shared tick online; hit-stop
  is presentation time") and ADR 0003:459.

### Rules table
| Rule | Source | Recorded as | Status |
|---|---|---|---|
| Private (rented) match with bots: 25 % of normal XP | mustafa-words.md item 2 | VISION_V2:72 | `[DECIDED]`, his tag |
| Private (rented) match, humans only: 100 % | same | VISION_V2:72 | `[DECIDED]`, his tag |
| Per-action weighting; minimum humans per team; daily cap | the Advisor's suggestions | VISION_V2:73-75 | `[OPEN]`, his "Open" |
| The VFX editor, after the map features | mustafa-words.md item 1 | EDITOR_V2:115-117 | `[LATER]` |
| Its ten principles | same | EDITOR_V2:118-134 | `[DECIDED]` each |
Nothing is built and no number enters a tuning table, so the balance-math and counter checks have nothing to run on;
the rule's math is in finding 2.

## Findings
1. **Medium. The XP-TERM question lacks the fact that decides between its options.** Service points come only from a
   share of the team's tech (LOOP_V2:131-132, `[DECIDED]`), and tech comes only from uplinks (LOOP_V2:45-50,
   `[DECIDED]`; :52 `[DEFAULT]` "one tech source"), never from kills. So the Advisor's open suggestion (1), "weigh XP
   per action (kills on bots give little)" (VISION_V2:73), has nothing to weigh if "XP" means Service points; it
   fits a per-action XP: the weapon tracks' (LOOP_V2:242) or an account-level XP, which v2 has not defined
   (LEGACY_INVENTORY:302: "LOOP_V2 does not say how account level is earned"; legacy: 50 XP per kill). The XP-TERM
   follow-up (STATUS:505-510) and the plan's report entry (advisor condition 3) list what "XP" names but not this.
   Fix: one sentence in the follow-up and in the report's XP-TERM entry: "Service points come from a share of the
   team's uplink tech, not per kill (LOOP_V2 §4, §8); suggestion (1) assumes a per-action XP, which only the weapon
   tracks have today; LOOP_V2 does not say how the account level is earned." It maps nothing and decides nothing.
2. **Low. The rule as written has a cliff and edge cases, none recorded.** "With bots in the match: 25 %" reads as
   any bot at any time. Math: in a private 6v6 with 11 humans and 1 bot, each human gets 0.25 of normal XP; with 12
   humans, 1.0. One bot costs 75 % of the XP of all 11 humans. Since bots fill empty slots by default (VISION_V2:52
   `[DEFAULT]`, LOOP_V2:22 `[DECIDED]`), a player who leaves a humans-only match and is replaced by a bot would cut
   everyone's XP from 100 % to 25 %, if a bot fills the slot (R2-ROOM-OQ4, README:180, is open). The Advisor's
   suggestion (2) (a minimum number of humans per team) is the open fix for part of it. Not this pass's to decide.
   Fix: optional, one line in the "Not decided by the note" follow-up (STATUS:492-497) for the community-servers
   design session: when "with bots" is judged (start, any moment, end), and whether a host may turn bot fill off
   (R2-ROOM-OQ5, README:182).
3. **Low. Scope: "private (rented)" against other private matches.** VISION_V2:50-51 (`[DECIDED]`, clan private
   matches) and :82 (§6.1, "rooms and private matches") name private matches that may not be rented. His rule covers
   rented ones only; README:183's clause says "for private (rented) matches", correctly. Also: under XP-TERM options
   (a) to (c), his rule implies rented private matches do give account progression, which in practice answers
   STATUS:492's question ("whether rented 'official private servers' count as official for progression"). Fix: the
   report's XP-TERM entry can say that answering (a), (b) or (c) also closes that follow-up; and the follow-up can
   note that private matches that are not rented are not covered.
4. **Low. His "Whole impact moments" against LOOP_V2 §7's split feedback, note only.** The sim side is settled
   (hit-stop is presentation time, ADR 0004:222-223, ADR 0003:459). The tension that remains is timing: LOOP_V2:119
   (`[DECIDED]`) puts the shot's feedback at once and hit feedback on confirmation, about one round trip later
   (VISION_V2:58, "about 100 ms of ping"). One asset that bundles "particles, sound, camera shake and hit-stop, timed
   together" (EDITOR_V2:127-128) must split into a shot-time part and a confirm-time part, or its hit-stop arrives up
   to about 100 ms after the shot it belongs to. Two more for the same design session: the feel kit (ADR 0005 CD-14,
   :528-531: muzzle flash, impact, sound per weapon) and his effect assets now both name muzzle flash, impact and
   sound, so which one references the other is open; and his camera shake, impact frames and halftone flashes add
   sources to OQ-0003-18 (reduced motion, README:105), which his words do not address. Fix: optional, one STATUS
   follow-up line for the VFX tool's design session and OQ-0005-CD7; no doc edit, nothing decided.
5. **Low. Quality tiers and effects that hide players.** "Low drops extras but stays readable" (EDITOR_V2:130-131)
   is about readability. The planned Shinobi has a smoke bomb (UNITS:91) and Sydney Mobius "+ smoke" (UNITS:134): if
   a smoke hides players only on screen, a tier that thins it gives Low players a sight advantage. Fix: optional
   follow-up for the tool's design session and OQ-0003-16 (README:104): an effect that hides players hides them the
   same on every tier, or the sim's line of sight decides.
6. **Low. Two small completeness gaps.** (a) The XP-TERM list omits LOOP_V2:235 (the Advisor's `[DEFAULT]` reading,
   "guns still level from XP earned while using them"): the same weapon XP, so cosmetic. (b) VISION_V2's header
   (:6-7) dates §5 to his note of 2026-11-03 and does not mention the 2026-11-06 XP block (the advisor made this
   optional; the block carries its own date at :71). Fix: add :235 to the list; optionally a header clause.

No contradiction with LOOP_V2, VISION_V2 or UNITS is left unhandled beyond findings 1 to 5: the rule touches no
`[DECIDED]` line (LOOP_V2 §8's share of tech and "breadth only, never power" stand; §10's bots are unaffected in
public matches), and the VFX principles add no rule to LOOP_V2 §7.

## Verdict: PASS WITH NOTES
His words are faithful; nothing he left open is decided; "XP" is mapped to nothing; the register clauses say "bears
on it", not "answered", and telemetry stays open; the CD7 clause does not make the VFX editor the Feel Lab. Finding 1
(Medium) should be fixed in the follow-up and the report before Mustafa's XP-TERM answer; 2 to 6 are optional.
