# Advisor: check of the night plan's fix pass (5afc772 -> 7812127)

Plan: read my ruling and the verifier's report, read the diff and the fixed file at 7812127, then check conditions 1-5, ADV-NP-7..10, every other change against 11fd190, and anything broken; one line per check below.

- Word diff (5afc772 -> 7812127, word regex): every changed word is in the hunks listed; the rewrapped lines change no word (the Write/Edit sentence only rewrapped; header lines 5-8 untouched).
- C1 met: plan :139-144 item 12 names netcode-architect, security-auditor, test-engineer, rust-reviewer (u4-build-plan.md:816 at 11fd190 holds those four), engine-architect on pub items (:930-931 = §7.2 Public items A19), xhigh + 3-agent limit (advisor-u4.md:147 = condition 10 (c)), builder's seat (:925-927), W1-TARGET lock, "Left for the main session at launch" (u4r-brief.md:1845 heading). grep: rust-reviewer 1, engine-architect 1, "at launch" 1.
- C1 Low (my own cite, carried): the card's U4 line (11fd190 :3070-3072) names W1-TARGET's flock lock but not "the lane runner"; the runner is in u4r-brief §12 (runner sha256), which item 12 also cites. No change needed.
- C2 met: plan :90-94; §4.14 (:453-476; :462, :463, :475 verified), Q16 (a) :1042-1045 verified, advisor-u4.md:78-89 (condition 1) and :477-479 (the 2026-11-11 amendment) verified; §5 preamble :507-512 holds "counts as deltas"; carry 46 = -21's press twins (u4s-carries.md:506).
- C2 Low: the twin list names "clamp, press, fate-aligned and ack-aligned" but not §4.10's sim-checked kinds (cooldown, ammunition, reload edges, :367-368); the cite range and condition 1 cover them. No change needed.
- C3 met: plan :128-130; "home's words" 0 hits, "@@WORDS-" 1 hit; routing by id and file:line range; no hand copying of rulings or his words.
- C4 met: plan :71-73 cite advisor-te-m3.md:126-127 and say the stop clause is asked, not decided; item 10 (:136-137) lists the stop clause question. Low (my own cite): the after-clause is at :127-128, one line later.
- C5 met: :3 "about 15:21 UTC"; :156-157 "06:45 / Amsterdam (05:45 UTC)" (one string across a wrap: a line grep gives 0, joined gives 1). NIGHT_SHIFT.md on main: Amsterdam time; 2026-11-16 06:45 CET = 05:45 UTC (date -d).
- ADV-NP-7 folded: :104-106, §4.11 (:392-407 verified: HashMap/HashSet bans for room, client, server, lab, bots; clock ban for server, lab, bots), plan.md:617-628 SimClock with rate 1.5 at :625.
- ADV-NP-8 folded: :126-127 "every code or file item the brief cites". Low: the parenthetical builder list omits U4a, U4b, U4c and earlier units; "every" governs. Optional: "whichever unit built it".
- ADV-NP-9 folded: :147-148 both checkers read each carry's text at its u4s-carries.md range.
- ADV-NP-10 folded: :12-14 "reads 11fd190 only; later landings are the morning recheck's job".
- Verifier Low 1 folded right: u4r-brief.md:940 CARGO_NET_OFFLINE, :968-971 disk floors, c-u4r-runs.mjs:131-132 FLOOR = 4 GiB; §7 now lists only what it holds (:911-915).
- Verifier Low 2 folded right: advisor-u4.md:334-335 (condition 4: reports list carries met by number), u4s-carries.md:9-10 (word for word), u4r-brief.md:695-696 (met / not met (why)).
- Verifier Low 3 folded right: advisor-u4df.md R6 heading :301, ruling :317-318, T4 item 1 :461-464 (461 is the bullet header); sentence equal to T4's and the card's Rules in force.
- Verifier Low 4 folded right: history.md:2127 names marks.json and the judge only; advisor-c62h-review-full.md:89 (U4f's brief says whether marks.json changes) and :93 (SERIES.md: recommended, "though his words do not name them"). Now nothing of the advisor's is recorded as his.
- Verifier Low 5 folded right: advisor-u4r-r1-full.md NC-U4R-5 :78-81, EA-U4R-7 :200-204, :61 under NC-U4R-1 (heading :58).
- Extra corrections: U4E-Q11 at advisor-u4-wave1.md:492-495 (right); "never a Kick for a forged kind" at u4-build-plan.md:383-384 (right in substance: "refused and counted, never kicked, so no forged kind closes the session"); Low: the quoted phrase is neither file's exact words (the map's :60 says "never for a forged kind").
- "Checked before posting" line: both notes files exist at 7812127; "five Lows folded" true; "conditions 1-5 met" true by this check; informational, no new step.
- His 2026-11-09 words: plan :5-8 equal the card's record at 11fd190 (:3362-3366) and at 7812127 (:3390-) after whitespace join: EQUAL. My ruling's ":3376-3380 at 11fd190" was wrong (that was a later commit's position); the plan cites the card by heading, not line, so nothing to fix in the plan.
- Broken? No command added or changed (rules 1-2 untouched); the new paths are reads under the worktree (c-u4r-runs.mjs is read, not run: rule 3 holds); no new agent, step or decision for the night; no line over 120 characters (line 102: 120); 161 lines.
