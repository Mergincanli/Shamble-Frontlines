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
- Next: line-length scan of the new lines; then r31_check.py and adv_words_check.py once each.
