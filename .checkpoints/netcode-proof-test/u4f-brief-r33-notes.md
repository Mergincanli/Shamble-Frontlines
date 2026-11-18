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

## Progress (one line per item as done)
