# Advisor: the check of the night plan's fix pass, 5afc772..7812127 (workflow wf_6df5140c-eeb), 2026-11-15

Written by script, word for word, from the workflow's journal. Nothing below is the main session's. Its notes: advisor-night-plan-fixes.md.

- **Verdict:** APPROVE

## Summary

The fix pass at 7812127 (full SHA 781212774986118183a7c6521c34cca31b5fe2a0) meets conditions 1-5 and folds ADV-NP-7 to -10 the way I meant them. I checked every other change against the files at 11fd190 and each one is right: the verifier's five Lows, two more corrections taken from the verifier's spot checks (U4E-Q11, the Kick rule), the "Corrections to the map" sentence, the "Checked before posting" line, the header time and the rewrapped lines. A word-level diff shows that the rewraps change no words. Rules 1-7 are unchanged apart from rule 4's cite, which is now R6. No command was added. The only new paths are reads inside the worktree; c-u4r-runs.mjs is read, not run, so rule 3 still holds. No new agent, step or decision reaches the night. Mustafa's 2026-11-09 words in the header (:5-8) match the card's record exactly at 11fd190 (:3362-3366) and still do at 7812127. I compared them by script after joining the lines. The commit is pushed, and the plan file is the same at the branch head (2ca6ebe): blob 0f024533, sha256 d1330964. I found six Lows and none of them needs a change. Post item 7 to issue #16 and link the plan at 781212774986118183a7c6521c34cca31b5fe2a0. Not checked: GitHub (issue #16 itself). Notes: /home/user/Shamble-Frontlines/.checkpoints/netcode-proof-test/advisor-night-plan-fixes.md

## Conditions

- **C1 (ADV-NP-1)** · **met:** yes · Plan :139-144, shape item 12. Grep finds rust-reviewer 1, engine-architect 1 and "at launch" 1. At 11fd190: u4-build-plan.md:816 names netcode-architect, security-auditor, test-engineer and rust-reviewer; :930-931 is §7.2's Public items (A19) line, where engine-architect checks; :925-927 is the builder's seat; advisor-u4.md:147 is condition 10 (c), xhigh and the 3-agent limit; u4r-brief.md:1845 is the §12 heading 'Left for the main session at launch'. The W1-TARGET lock is in the card's U4 line (:3070-3072).
- **C2 (ADV-NP-2)** · **met:** yes · Plan :90-94 cite §4.14 (:453-476, with -21/not-older-than :462, -21/no-catchup-cap :463 and -22/unbounded :475, all verified), Q16 (a) (:1042-1045, verified) and advisor-u4.md:78-89 (condition 1), amended at :477-479 (the 2026-11-11 amendment, verified). The tests use counts as deltas (§5 preamble :507-512, verified). Carry 46 is -21's press twins (u4s-carries.md:506).
- **C3 (ADV-NP-3)** · **met:** yes · Plan :128-130 route each item by its id and its home's file:line range, put word-for-word text in as @@WORDS-<file>:<a>-<b>@@, and say no agent copies a ruling's words or Mustafa's by hand. Grep: "home's words" 0 hits, "@@WORDS-" 1 hit.
- **C4 (ADV-NP-4)** · **met:** yes · Plan :71-73 cite N/advisor-te-m3.md:126-127 for TE-Q2's after-clause and say the stop clause is asked, not decided. Shape item 10 (:136-137) asks whether the U4d-F ruling's TE2-5 stop clause still applies. (Low: my own cite is one line early; the after-clause is at :127-128.)
- **C5 (ADV-NP-5, -6)** · **met:** yes · Line 3 reads "about 15:21 UTC" (grep finds 1). Lines 156-157 read "If 06:45 / Amsterdam (05:45 UTC) comes first". Because of the line wrap, a plain line grep finds 0 and a grep on joined text finds 1. NIGHT_SHIFT.md on main uses Amsterdam time, and `date` gives 2026-11-16 06:45 CET = 05:45 UTC.
- **ADV-NP-7** · **met:** yes · Folded at :104-106: §4.11 (u4-build-plan.md:392-407, verified: the HashMap and HashSet bans and the clock ban in server, lab and bots) binding ClockSync, the 1.5x clock and the lag switch to simulated time (plan.md:617-628, SimClock; rate 1.5 at :625).
- **ADV-NP-8** · **met:** yes · Folded at :126-127 as "every code or file item the brief cites". Low: the builder list in brackets (U4s, U4d, U4d2, U4j, U4e, U4r, BRAWL-1) leaves out U4a, U4b, U4c and earlier units, but "every" governs.
- **ADV-NP-9** · **met:** yes · Folded at :147-148: both checkers read each carry's text at its N/u4s-carries.md line range, since the draft holds only placeholders.
- **ADV-NP-10** · **met:** yes · Folded at :12-14: the item reads 11fd190 only, and anything that lands later (one of Mustafa's answers, a card refresh) is for the morning recheck (shape item 8).

## Other changes

- **Verifier Low 1: §7 build rules (plan :102-104)** · **right:** yes · §7 now lists only what it holds (:911-915: the shared target, touch, counts as deltas). The rest comes from U4r's brief §4: CARGO_NET_OFFLINE at u4r-brief.md:940, the disk floors at :968-971 (4 GB / 2 GB) and FLOOR = 4 GiB at c-u4r-runs.mjs:131-132. All verified at 11fd190.
- **Verifier Low 2: carries met, by number (plan :99-101)** · **right:** yes · advisor-u4.md:334-335 is the U4s ruling's condition 4 (reports list carries met, by number). u4s-carries.md:9-10 is the word-for-word rule. u4r-brief.md:695-696 gives U4r's wording (met / not met (why)).
- **Verifier Low 3: the Write/Edit sentence now cites R6 (plan :39-42)** · **right:** yes · advisor-u4df.md R6 heading is at :301 and its ruling at :317-318. T4 item 1 is at :462-464 (:461 is the bullet header). The quoted sentence matches T4 and the card's Rules in force word for word; only its line breaks changed.
- **Verifier Low 4: the marks.json/judge read vs SERIES.md (plan :119-122)** · **right:** yes · history.md:2127 names only marks.json and the judge. advisor-c62h-review-full.md:89 (U4f's brief says whether marks.json changes) and :93 (SERIES.md is recommended, 'though his words do not name them') are right. The advisor's recommendation is no longer recorded as his rule. That is a narrowing toward his exact words, not a loosening.
- **Verifier Low 5 and the 'Corrections to the map' sentence (plan :60-65)** · **right:** yes · NC-U4R-5 is at advisor-u4r-r1-full.md:78-81 and EA-U4R-7 at :200-204; :61 falls under NC-U4R-1 (heading at :58). Also right: U4E-Q11 at advisor-u4-wave1.md:492-495, and the Kick rule at u4-build-plan.md:383-384 ('refused and counted, never kicked, so no forged kind closes the session'). Low: the quoted phrase "never a Kick for a forged kind" is not the exact wording of either file (the map's :60 says 'never for a forged kind').
- **'Checked before posting' line (plan :158-161)** · **right:** yes · It is information only and adds no step. Both notes files exist at 7812127. 'Five Lows folded' is true, and 'conditions 1-5 met' is confirmed by this check.
- **Header time 15:21 UTC and 'Done when' time zone** · **right:** yes · These are condition 5. 06:45 Amsterdam is 05:45 UTC on 2026-11-16 (CET).
- **Rewrapped lines** · **right:** yes · A word-level diff (5afc772 to 7812127) shows no changed word outside the listed hunks. Rules 1-7 are unchanged except rule 4's cite. The file is 161 lines, the longest 120 characters (line 102).
- **Mustafa's 2026-11-09 words in the header (:5-8)** · **right:** yes · Untouched by the fix pass. After joining the lines, they match the card's record at 11fd190 (:3362-3366) and at 7812127 (:3390-3394) exactly. My ruling's ':3376-3380 at 11fd190' was the wrong line. The plan cites the card by heading, so the plan is unaffected.

## Findings

### ADV-FX-1 (Low)

- **Where:** night-2026-11-16.md:156-157 (Done when); card at 7812127, the fix-pass line
- **What:** Condition 5's string "06:45 Amsterdam (05:45 UTC)" is split across a line break. A plain line grep finds nothing; a grep on joined text finds it. The card says each check was 'run by grep'.
- **Fix:** None needed. If the file changes for another reason, keep the string on one line, or have the card record that the check ran on joined text.

### ADV-FX-2 (Low)

- **Where:** night-2026-11-16.md:126-127 (shape item 8)
- **What:** The builder list in brackets (U4s, U4d, U4d2, U4j, U4e, U4r, BRAWL-1) leaves out U4a, U4b, U4c and earlier units, so a reader could take the list as a limit. "Every code or file item the brief cites" governs.
- **Fix:** None needed. If the file changes anyway: "(whichever unit built it: its name, ...)".

### ADV-FX-3 (Low)

- **Where:** night-2026-11-16.md:93-94 (shape item 2, -21's twins)
- **What:** The twin list (clamp, press, fate-aligned, ack-aligned) does not name §4.10's other sim-checked kinds: cooldown, ammunition and reload edges (u4-build-plan.md:367-368). The cited range :360-390 and U4 condition 1 (advisor-u4.md:78-89) both cover them.
- **Fix:** None needed. The test-engineer check covers every twin.

### ADV-FX-4 (Low)

- **Where:** night-2026-11-16.md:64
- **What:** "never a Kick for a forged kind" is in quotation marks but is not the exact wording of u4-build-plan.md:383-384 or of the map (:60). The cited lines say the same thing in other words.
- **Fix:** None needed.

### ADV-FX-5 (Low)

- **Where:** My ruling advisor-night-plan-full.md (ADV-NP-1, ADV-NP-4, (b)), carried into the plan at :72 and :142
- **What:** Three of my own cites are off. TE-Q2's after-clause is at advisor-te-m3.md:127-128, not :126-127. The card's U4 line names W1-TARGET's flock lock but not 'the lane runner'; the runner is in u4r-brief §12, which item 12 also cites. And his 2026-11-09 words are at card :3362-3366 at 11fd190, not :3376-3380. The plan cites the card by heading, so nothing in the plan is affected.
- **Fix:** None needed in the plan. Record the corrected lines with this check.

### ADV-FX-6 (Low)

- **Where:** night-2026-11-16.md:131-132 (shape item 9)
- **What:** Since 2156e49 (his 16:35 answers, history.md:2131-2146: KL-YES with one change, C62H-KEEP (a), C62H-BOUND (a)), "open with Mustafa" is out of date. The plan's own rule at :12-14 sends later answers to the morning recheck. Its "designs no lost-tick bound" still fits his C62H-BOUND (a), which brings the wording and size to him before anything is built.
- **Fix:** No change to the plan, so its SHA and this check stand. Optional: one line in the #16 entry citing history.md:2131-2146 by range only (no retyping), to be folded in the morning.

## To Mustafa

Tonight's night item 7 is cleared: a draft of U4f's build brief, with no building or testing. The day session made all six fixes I asked for, plus the verifier's corrections. I checked every changed line against the files, and nothing else in the plan changed. Your 2026-11-09 words in its header still match the card's record exactly. It can go on issue #16 now, linked to commit 781212774986118183a7c6521c34cca31b5fe2a0. Six small notes (wording and line numbers), none needing a change. One of them: the plan still calls your three answers of 16:35 "open". It was written before them, and it says the morning fold picks up anything you decide later. The draft also designs no lost-tick limit, which fits your C62H-BOUND (a): its wording and size come to you first.

Decisions needed: none from this check.

For Mustafa: nothing for this item.
