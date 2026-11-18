# U4f brief rev 3.3 fold: notes (netcode-architect, opus, xhigh), 2026-11-18

Task: fold the advisor's ruling (advisor-u4f-brief.md, condition L1, :370-378) into u4f-brief.md rev 3.2
(sha256 e9f33e61cffc5b229774000d9f6ac8ab1acc9567078caca14e3b2f14167a33ba, 1,734 lines) as rev 3.3, by the Edit tool only.
Rules: each old phrase found once (line break + next line's indent read as one space, adv_phrases.py's method); a phrase
that fails, or a fold that would change a word the ruling does not name: listed here, sent back to the advisor. His words
only as `<<Pn>>`. Add nothing the ruling does not ask (TEST-FIRST rule 3, history.md:2264-2267).

## Plan
1. Check the brief's sha256 and line count equal rev 3.2's (r31_check.py's first line, at the end only; here: git hash
   not needed: read the file; the advisor's sha is the start state).
2. Read the brief whole, in parts; note each fold site's line.
3. Fold A1 to A7 exactly as the ruling writes them (:65-172), one Edit per named phrase.
4. Fold the 18 rulings (:173-198) into each "by U4F-Qn" place: section 10's questions get a "Ruled:" line each; the
   conditionals the ruling makes firm (f38, f39; Q6 (b)'s +4 and A2's +1; Q9 (a)'s +1; Q11 (iii) (c)'s +1); p15 to p20
   kept; KL-U4F's trigger at A6's place; section 4's lane and Q1 (a) as A2 words them.
5. Restate counts: at least +118 measured, +133 bites, 67 negative cases (plus T-6's +5 only if contract.rs is edited;
   P33's moved test -1 +1); 21 pre-runs; disk about 3,727 MiB. SIZE-CHECK table: a row for this ruling (about +300),
   total about 5,150 (4,200 to 6,200).
6. Section 10: "Ruled: ... (advisor-u4f-brief.md)" under each question.
7. First change-list block "Change list (rev 3.2 to rev 3.3: the advisor's ruling)", one row per fold item; title says
   rev 3.3.
8. Run once each, read-only: r31_check.py and adv_words_check.py; quote output here.
9. Hand back: items folded, sent back, counts, sha256, lines.

10. Added mid-run (the coordinator, from N/advisor-u4f-lead-bound.md E4's second bullet): U4F-Q11 (iii) (c)'s fixed
    inputs walk, no sprint or crouch (the brawl bots hold fire, so never sprint: bots/src/lib.rs:292), in section 10 and
    where 2.6 or 2.10 state the fast clock's fixed inputs; "top-speed movement" for the bound's 240 cm reads "full-speed
    walking (600 cm/s)" (sprint 1.5x the walk: world.rs:484-486). Cites read: bots/src/lib.rs:292 `buttons:
    Command::FIRE`; world.rs:484-486 sprint off when fire held, 3/2 speed. Grep: "top-speed" once (Q8's why, :1401);
    2.6 states no fixed inputs; 2.10's +1 by Q11 (iii) (c) names "the fixed-input episode".

## Site map (read whole at rev 3.2; Grep counts)
- A7: netcode.md:15 once (:268); netcode.md:13-14 twice (:322 2.1 Bounds, :371 2.4 item 2).
- A3 carry 74: "the commands that arrived after their stamped tick ran" twice: :340 (2.2's pass-marks row, NOT named by
  the ruling) and :967 (5.3's closing paragraph, named). D15 (:1607, named) does not hold the phrase. So: fold :967
  only; :340 and D15 go back to the advisor.
- "at a 2 s episode" twice (:605 scale rule, :636 f19): f19's edit matched with its row's text.
- f38/f39 "only by" and "1 (conditional)" at :655-656; p15-p18 :961, p19 :962, p20 :963.
- KL-U4F's trigger: 9.1 :1234 ("after 5.5 items 1 and 2"), 12 item 6 :1678-1679, 5.5's room paragraph :1024-1025
  ("read at launch (D2 after items 1 and 2)").
- "(only if a ruling opens npt-server)" once (D9 :1601); its When "after the build, only by the ruling" once.

## Decisions on placement (for the advisor's eye)
- A1 Q11 (iii) (c)'s add: appended to option (iii) (c)'s text in section 10 (after "so the row's words hold as written").
- A2's sentence "The fix moves or re-gates loopback.rs's use ...": placed at the end of section 6 item 4 (where the
  E0425 stop is described), as the ruling's bullet names item 4 and D9 and gives no other place.
- A3 option A cell: a ";" between "f34" and the appended text (punctuation only, so the cell parses).
- A4 2.1 cell: appended with "; "; f31: after "ClockSync's tick period in ns" (the advisor's anchor ends there).
- A5: B2's gain labelled "rev 3.3 (A5):" like the list's rev 3.1 and rev 3.2 labels.
- Q5: P25's substitution (section 4 and 12 item 3) reads the ruling's words: "(and npt-proto, npt-server and npt-sim,
  which U4F-Q1 opens)".
- Q6: f1 gains "on a trace that carries P2's 2 % loss" (the ruling names f1).
- Section 10's Ruled lines: the ruling's one-line rulings (:174-197), the advisor's words, with "(advisor-u4f-brief.md)".

## Progress (one line per item as done)
- A7 done: section 1 item 5 netcode.md:15 -> :17; 2.1 Bounds and 2.4 item 2 netcode.md:13-14 -> :14-15.
- A4 done: 2.1's Ping rate and phase cell gains the interval text ("; " join); f31 gains "; the honest ping interval (A4)"
  after ClockSync's tick period.
- A3 done in part: 2.3's Missing row, option A's cell (";" after f34) and option B's cell; f34's row; p12's row; 5.3's
  closing paragraph's option-A reading (line rewrapped). SENT BACK: D15 holds no "the commands that arrived after their
  stamped tick ran" (it reads "1 - `tick.s.cmd_on_time`'s share" only: nothing to replace); 2.2's pass-marks row
  (:340) holds the same phrase but the ruling does not name 2.2: left as rev 3.2 wrote it, for the advisor to rule.
- A1 done: the Movement paragraph before "### 2.7" (closing 2.6); f19 after "at a 2 s episode" (matched with its row);
  f20's replacement; Q11 (iii) (c)'s add (appended to option (iii) (c)); 5.6's B8 row after B7.
- E4 (the coordinator's added item) done: Q11 (iii) (c) gains "the fixed inputs walk, no sprint or crouch (the brawl bots
  hold fire, so never sprint: bots/src/lib.rs:292; rev 3.3, advisor-u4f-lead-bound.md E4)"; Q8's why "12 ticks of
  top-speed movement per episode" -> "12 ticks of full-speed walking (600 cm/s) per episode" (once; line rewrapped);
  2.10's Q11 (iii) (c) test gets the walk words in the counts restatement (below). 2.6 states no fixed inputs: none.
- A2 done: Q1 (a)'s replacement (rewrapped); section 4's lane replacement; 2.5's replacement; section 6 item 4's
  heading replaced and A2's fix sentence ("The fix moves or re-gates ...") appended to item 4; D9's claim and its When
  ("after the build"); 2.13's npt-server row and a new npt-proto row after it. 2.10's +1 goes in the counts step.
- A5 done: B2's list gains "rev 3.3 (A5): U4F-Q3 (c)'s ticks ...; the lead bound's header param (U4F-Q8 (b)); A1's
  dead and respawn ticks apart".
- A6 done: 5.5 item 2's heading (capital "After" kept at the item's start; item rewrapped); D2's after-item-2 reading
  appended to D2's "Left" cell, with "(A6)". KL-U4F's trigger at A6's place: 9.1's KL-U4F row and 12 item 6's second
  draft ("after 5.5 item 2 (A6's place)"), and 5.5's room paragraph ("read by D2 after item 2 (A6's place)", which said
  "read at launch (D2 after items 1 and 2)"). Not changed: Q2's own text (its Ruled line carries the trigger).
- Conditionals made firm: f38 and f39 ("**by U4F-Q18 (a) (ruled)**", "**by U4F-Q18 (ii) (d) (ruled)**", Δ "1"); p15-p18,
  p19, p20 kept ("by ... (ruled)"); 5.5's pre-runs row "21 (...ruled) | about 315" and Total "65 + 21 + 9", "about 3,727".
- Counts restated: 2.10's paragraph (+118, +133, 67; made firm: f38, f39, Q6 (b) +4, A2 +1, Q9 (a) +1 neg, Q11 (iii)
  (c) +1 with E4's walk words; still conditional T-6 +5, P33 -1 +1; struck Q13 (a)'s +1); section 6 item 1 (+118,
  +133); section 7 items 3 (+118, +133, 67) and 16 (5,150).
- SIZE-CHECK done: a row "the advisor's ruling (rev 3.3) | about 300 | ..." and the Total "about 5,150 (4,200 to
  6,200)", "about 1.7 times M's 3,000" (5,150 / 3,000 = 1.72; was 1.6).
- Q5 done: P25's substitution in section 4's stop clause and in 12 item 3 now reads "(and npt-proto, npt-server and
  npt-sim, which U4F-Q1 opens)" (the ruling's Q5 reasoning, :222-223); both paragraphs rewrapped (whitespace only).
- Q6 done: f1 gains ", on a trace that carries P2's 2 % loss (rev 3.3: U4F-Q6's ruling)".
- Section 10 Ruled lines: Q1-Q18 done, 18 lines (Grep count 18), the ruling's lines :174-197 with
  "(advisor-u4f-brief.md)" after each; one word changed: Q12's "come to me" reads "come to the advisor" (the brief is
  not the advisor's voice).
- Title "(rev 3.3, day draft)"; a rev 3.3 record paragraph above rev 3.2's (states every U4F-Q ruled and that rev 3.2's
  "Next" and "Nothing here is ruled" are its own state); the first change-list block "Change list (rev 3.2 to rev 3.3:
  the advisor's ruling)", rows R21-R33, and "Counts at rev 3.3". `git diff --stat 0ad9661 HEAD -- spikes/netcode-proof`
  printed nothing at HEAD 5347170eddccac6c5798adf90f82f9af3288ba88.
- Line-length scan (Grep `^[^|].{120,}$`): three lines of mine past 120 (5.3's closing paragraph, Q8's why, 12 item
  6's KL-U4F); fixed by a short line each so rev 3.2's following lines stay byte-identical. The other long lines are
  rev 3.2's own.

## The checks, run once each after the fold (read-only)

`python3 -I .checkpoints/netcode-proof-test/u4f-r3/r31_check.py`:
```
brief sha256 609bfd169bacb08c024c1788e03537586f00ba4b596b44cc56d3bf9dd35bf0d0 lines 1827
P24 key count: 1 bare words count: 2
placeholder uses: {'P1': 8, 'P2': 2, 'P3': 3, 'P4': 2, 'P5': 1, 'P6': 2, 'P7': 2, 'P8': 3, 'P9': 2, 'P10': 1, 'P11': 2, 'P12': 1, 'P13': 2, 'P14': 3, 'P15': 12, 'P16': 7, 'P17': 5, 'P18': 4, 'P19': 4, 'P20': 3, 'P21': 8, 'P22': 6, 'P23': 5, 'P24': 2, 'P25': 3, 'P26': 2, 'P27': 5, 'P28': 2, 'P29': 2, 'P30': 3, 'P31': 4, 'P32': 8, 'P33': 3, 'P34': 3, 'P35': 2, 'P36': 4}
unknown <<P ids: []
PV1 6 base 34 @@ 4 r3 markers 0
section 12 item 3: lines 1709 to 1753
7-word runs outside section 12 item 3: 0
```

`python3 -I .checkpoints/netcode-proof-test/u4f-r3/adv_words_check.py`:
```
history line 414 <- brief lines [818] |     `CARGO_TARGET_DIR=/home/user/Shamble-Frontlines/.gate/shared-target`, one folder for all copies, created b
history line 1913 <- brief lines [23] |    merge folded (u4e_fold2.py: 3.1 with carry 61; R27, R28, R38 at d50396a). **The advisor's check of rev 3.2
history line 2212 <- brief lines [1776, 1777] |       - (a) Yes. I may pack one or two closed, older log folders by the kept-logs rule, never the P0 folders, 
history line 2309 <- brief lines [14] |   brief rev 2 `git show 34a2e55:.checkpoints/npt-u4f-brief/u4f-brief-draft.md` (1,117 lines; rev 1 at 828e2f4)
history line 2363 <- brief lines [1304] |   negative snap_incomplete's phrase; NA-CBB-2 no npt-lab log judged before the P2 set (its pre-flight call goe
history line 2405 <- brief lines [1045] |   against c1935a5b… with a stop on any difference, --commit the full HEAD after the spike-tree check against d
history line 2487 <- brief lines [1099] |   both P0 folders, fc411116c1f1, fd4612050cd6 or the c-u2 folders without his word. **The report went to Musta
history line 2495 <- brief lines [393] |   words by script, one review round, his go to merge); -58's two checks go with U4g (the wording yes first; AD
history line 2536 <- brief lines [1684] |   the parent, diff and kit-equality checks; C4 `node check.mjs --only refs,marks,self,redact` through main-run
history line 2551 <- brief lines [1684] |   line, marks 81, self 78 of 78, redact 10 of 10 and 305 items, 0 findings; "check: all 4 steps pass"; exit 0:
history line 2578 <- brief lines [231] |   08:36-08:51 UTC; saved word for word by u3c_held_1_review_full.py: u3c-held-1/advisor-plan-review-full.json,
history line 2612 <- brief lines [832, 1684] |   test 552 / 632 passed, 0 failed, 0 skipped; refs 1382; marks 81; self 78 of 78; redact 10 of 10, 306 items, 
history line 2613 <- brief lines [1684] |   findings; deps 137 crate files; wasm 27000 ticks, 0 mismatching both ways; free 5,552 → 5,514 MiB. **Step 8:
history line 2628 <- brief lines [147] |   u3c-held-1/u3c-held-1-report.md; Open findings updated. **DOCS-54 BUILD STARTED (12:41 UTC):** the
history line 2702 <- brief lines [94] |   new **U4F-Q18** (option A's past window W = CMD_CATCHUP), the lab tests' scale rule, the build phase in the 
history line 2703 <- brief lines [99] |   budget, packing only u4r-P3 (a U4f set's own folder only after his yes to KL-U4F). Now: tests +107 / +122, 6
history line 2707 <- brief lines [1293, 1770] |   them; **KL-U4F** only if D2 shows the room short after the launch pack and part-cleans. **D2 read** at 18:54
history line 2708 <- brief lines [70, 1685] |   `df -BM` 5,468 MiB free (1,372 above the floor); runs/: u4r-P3-0f066338dd8d 519, u4r-P3-d66e23cd78f2 298,
history line 2709 <- brief lines [1685] |   u4r-P3-fd4612050cd6 291, u4r-P2-8ee453c22882 741, u4r-P2-fc411116c1f1 623, u4r-P0-051995db9cea 3,249,
history line 2712 <- brief lines [1079] |   target's part-clean (unmeasured; the last dev part-clean gave back 6,125 MiB).
history line 2728 <- brief lines [1424] |   estimators passes -47's p99 of 5 ms (best, the mean: 6.06 ms with random phases, 8.69 ms with one phase per
history line 2730 <- brief lines [1427] |   stamp (U4F-Q6 (b)'s reference) the mean passes (2.32 ms; 2.34 with P2's 2 % loss) and the tick-quantised bit
history line 2741 <- brief lines [58] |   u4f-brief-r3-notes.md, "Revision for round 2"); then Advisor, general-purpose reading main's advisor.md, che
history line 2947 <- brief lines [603] | where carry 80 (b) and (d) touch how the client files lost ticks. Push it, then add item 7 to issue #16's body
history line 2977 <- brief lines [11, 12, 1781] | (rev 2 draft, the test-engineer's and the security-auditor's checks), run the recheck at HEAD with his 16:35 a
history line 3614 <- brief lines [1132, 1193, 1693] |               Recommended too: marks/SERIES.md and marks/README.md. The test at a unit's close: `git diff --st
history line 3615 <- brief lines [1132, 1193, 1693] |               <base>..<head> -- spikes/netcode-proof/marks/marks.json spikes/netcode-proof/crates/judge`.
history line 4093 <- brief lines [861] |   - **(3) The one runner change**, c-u4r-runs.mjs and c-u4r-r2.mjs, plan first with the advisor's plan review.
history line 4873 <- brief lines [1787] |   1 point 1); xhigh; at most three agents at a time, so two batches; **the advisor confirms the panel first**.
history line 4909 <- brief lines [1036] |   only after the record run of c-u4r-r2.mjs shows 0 on every clean short L1 P0 run; then `npt-judge --only
history line 5411 <- brief lines [378] | `npt_sim::ticks_for_ms(rtt_ms + jitter_ms).max(1) + 1`, P0 2, P1 3, P2 and B1 5, P3 7, within CMD_CATCHUP 8; c
history line 5431 <- brief lines [1702] | D6, D8, D10-D12 filled, placeholders P1-P15, section 12 "Left for the main session at launch"; its own slip (a
history line 6259 <- brief lines [924] |   tell him in three lines (what grew, why, the cheaper option). /compact: ask as before, never wait; if he has
history line 6315 <- brief lines [805] |   3. RM (carry 67) for any change that moves ES's synthetic recording or the World's outcome on it.
history line 6398 <- brief lines [1306] |     u3c-held-1/u3c-held-1-report.md). **Its deferred follow-ups (Low; taker: the next change to delay.rs,
history line 6422 <- brief lines [1302] |     change, U4f, U4g or JDG-ONLY-1; to STATUS with the report):** TE-CBH-3, TE-CBH-4, TE-CBH-6, TE-CBH-7, RR-CB-1 (the
history lines hit: 36
```

**Reading of the 36 hits (for the advisor):** the advisor saw 2 at rev 3.2 against 2,232 history lines. history.md
grew after its check: the old card moved in whole at 22:45 UTC (commit 8a39dc7, history.md:2284-6789), and that text
is the sessions' own. Mapped against `git diff -U0 8a39dc7 -- u4f-brief.md` (rev 3.2 as committed): only three hit lines
are lines rev 3.3 touched. :23 ("The advisor's check of rev 3.2", my new block) hits history :1913, the session's own
record of U4e's brief check (read). :1293 (9.1's KL-U4F row) and :1685 (D2's row) hit on words those rows already had at
rev 3.2 ("only if D2 shows the room short after", history :2707; D2's df and du figures, :2708-2709). Every other hit
is on a line rev 3.3 did not change. Spot-read and found to be session text: history :1913, :2947 (the card's night
narrative, not a paste) and :3614-3615 (the card's c62h-review record). The others (:2309-:6422) are not read one by
one here; L4 item 10's re-run on rev 3.3 should read them against the larger history.

## Sent back to the advisor (not folded, or folded with a choice to confirm)
1. **A3, carry 74's option-A reading:** the ruling names "5.3's closing paragraph and D15". Folded in 5.3's closing
   paragraph. **D15 holds no such phrase** (it reads "1 - `tick.s.cmd_on_time`'s share"): nothing replaced there.
   **2.2's pass-marks row** (now :378) holds the same old phrase ("the commands that arrived after their stamped tick
   ran") but is not named: left as rev 3.2 wrote it. So the phrase stood twice in the brief, not once.
2. **Not restated, outside the places L1 names:** "3,607 to 3,727 MiB" in 5.5's room paragraph (with "about 1,855 MiB
   short of the sets' 3,607"), in Q2's question, in 12 item 6's KL-U4F draft ("Why it matters") and in NA-5; 9.1's
   "npt-server `--all-features` compile" row ("if a ruling opens npt-server; else U4h"), now answered by A2; 9.1's
   readings-PR row, which A1's new reading does not join in the brief (L3 is the main session's commit).
3. **Placement choices:** A2's "The fix moves or re-gates ..." at the end of section 6 item 4; A1's Q11 (iii) (c) add
   and E4's walk rule appended to option (iii) (c); A3's ";" after "f34"; A4's "; " join in 2.1; B2's "rev 3.3 (A5):"
   label; Q12's Ruled line "come to me" read as "come to the advisor"; 5.5's room sentence ("read by D2 after item 2
   (A6's place)") taken as KL-U4F's trigger; Q5's P25 substitution in section 4 and 12 item 3 and Q6's f1 loss, both as
   the rulings' words in the places they name.

## Done
All of A1-A7, the 18 Ruled lines, the conditionals, the counts, SIZE-CHECK, the title, the record and the change list
are in; E4 (the coordinator's added item) is in. Brief: sha256 609bfd169bacb08c024c1788e03537586f00ba4b596b44cc56d3bf9dd35bf0d0,
1,827 lines (r31_check.py). No commit (the saver commits this folder).
