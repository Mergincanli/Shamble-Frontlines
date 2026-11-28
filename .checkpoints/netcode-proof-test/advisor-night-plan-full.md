# Advisor: the check of night-2026-11-16.md (workflow wf_6e5acbfd-5c3), 2026-11-15

Written by script, word for word, from the workflow's journal. Nothing below is the main session's. Its notes: advisor-night-plan.md.

- **Verdict:** APPROVE WITH CONDITIONS

## Summary

Read-only review of night-2026-11-16.md (item 7: draft U4f's build brief), checked against main's advisor.md, NIGHT_SHIFT.md, CLAUDE.md, main's routine guard (routine-guard.mjs), N/night-u4f-scout-full.md, u4-build-plan.md:805-816 and advisor-u4.md.

(a) Fit with the night's rules: the night can run the whole item with no step the plan leaves out. Every command is one the guard allows as written: a plain `git fetch origin <branch>`; `git worktree add .gate/npt-spike-1116 <sha>` (the target is inside the project and is no kit path); `git switch -c cs/npt-u4f-brief origin/main` (the switch check passes because origin/main is main); git add, commit -m and push origin <current branch>, each alone. The checkpoint saver is main's tracked checkpoint.mjs. Comments go only on #16. It runs no PR, web, build, test or script from the spike, and at most 2 agents at once. The three agent types have Write, and the required Write/Edit sentence covers the missing Edit tool. Two earlier night items (cs/npt-u4e-brief, about 30 minutes; cs/npt-u4r-brief, about 65 minutes) ran this same pattern end to end. If a run is cut off, NIGHT_SHIFT.md step 4 resumes it, and rule 1's "use it as it is" covers the worktree.

(b) Scope: nothing of Mustafa's or the advisor's is decided at night. C62H-KEEP, C62H-BOUND and KL-YES stay open ("designs no lost-tick bound"). Nothing is packed or deleted, and the only push is to cs/npt-u4f-brief. The header's copy of his 2026-11-09 words matches the card's record (.checkpoints/netcode-proof-test.md:3376-3380 at 11fd190) word for word, and it was copied by script.

(c) Completeness: I found no scope creep. Three Medium gaps in the brief's shape (ADV-NP-1 to -3), plus Lows.

(d) Approval: the item falls in the class of work his 2026-11-09 rule approves (plan drafts for the next unit; no build or test), with precedent in items 2 and 4 of 2026-11-10. Nothing in it needs his word first.

(e) Risks: carries copied by hand are handled by the @@CARRY@@ placeholders, but item 9 reopens hand copying (ADV-NP-3). The card's moving line numbers are handled by "find card items by their text". Time looks fine. The plan states one time wrong (written 15:21 UTC, not 15:35) and gives 06:45 with no time zone.

I confirmed that between the scout's commit 0ad41a6 and 11fd190 only the card changed (+21/-5) and new scout files were added, so the plan's "line numbers hold at 11fd190 for every file but the card" is true.

Not checked: GitHub (issue #16 and the night-shift comments; the GitHub MCP failed to connect). The verifier checks every reference separately.

Process note: I saved main's routine-guard.mjs to my scratchpad with one shell redirect before re-reading the Write/Edit-only rule. Nothing in the repository changed, and I made no other shell writes. Notes: /home/user/Shamble-Frontlines/.checkpoints/netcode-proof-test/advisor-night-plan.md

## Findings

### ADV-NP-1 (Medium)

- **Where:** night-2026-11-16.md:65-115 (the brief's shape, items 6-7)
- **What:** The shape never names U4f's reviewers or how the build is launched. The plan names netcode-architect, security-auditor, test-engineer and rust-reviewer (u4-build-plan.md:816), plus engine-architect on the pub items (A19, §7.2 :930-931), at xhigh and counted toward the 3-agent limit (advisor-u4.md:147, condition 10 (c)). Also missing: the builder's seat (§7.2 :925-927), W1-TARGET's flock lock and the lane runner (the card's Rules in force, "U4" line), and a "Left for the main session at launch" list. U4r's brief, which the plan calls the model, has all of these (u4r-brief.md:1896-1899 and §12 :1845-1909; advisor-u4r-brief.md L1-L6).
- **Fix:** Add a shape item "Review and launch" that names the four reviewers per :816 plus engine-architect for A19, at xhigh and counted toward the 3-agent limit (advisor-u4.md:147); the builder's seat (§7.2 :925-927); the W1-TARGET lock and the lane runner; and a launch list on the model of u4r-brief.md §12 (<base> and its counts, the runner's sha256, the placeholder fill).

### ADV-NP-2 (Medium)

- **Where:** night-2026-11-16.md:70-81 (shape item 2, What to build)
- **What:** Item 2 lists the components to build, but no tests or negative cases with counts as deltas, no bites with their planted sites (§4.14, u4-build-plan.md:453-476), and no -21 twins. The twins are set by §4.10 (:360-390), Q16 (a) (:1042-1045) and U4 condition 1 (advisor-u4.md:78-89, amended :477-479). U4r's model has 2.4 tests and 2.5 bites. The checks would likely catch the gap, but these are U4f's core proof, and catching them late costs a revision.
- **Fix:** Add to item 2: the tests and negative cases (counts as deltas); each bite with its §4.14 site and plant; -21's twins per §4.10, Q16 (a), condition 1 (advisor-u4.md:78-89, :477-479) and carry 46.

### ADV-NP-3 (Medium)

- **Where:** night-2026-11-16.md:108-110 (shape item 9)
- **What:** "Routed into the brief with its home's words" asks the night agent to copy text by hand: advisor ruling text and, where an Open item's home is history.md, Mustafa's own words (KL-YES: history.md:2088-2111). That goes against the plan's rule 6, his rule that his words move only by script and line ranges (card :3072; history.md:353-356), and the plan's own placeholder route for carries.
- **Fix:** Reword item 9: each Open item is routed by its id and its home's file:line range. Any text needed word for word goes in as a placeholder @@WORDS-<file>:<a>-<b>@@ for the day session's script. No agent copies a ruling's or Mustafa's words by hand.

### ADV-NP-4 (Low)

- **Where:** night-2026-11-16.md:62-64 (Read first, rules in force)
- **What:** "TE2-5's rule ended at U4r's merge" is correct (card :1567 M6; advisor-te-m3.md:119-127). But two things are left unsaid. TE-Q2's after-clause still applies (:126-127: FX-10's saved output stays, and a failing test is a defect until its cause is known). And whether the U4d-F ruling's TE2-5 stop clause (card :3098-3101) still applies is the advisor's call, not the drafter's. The card's Rules in force also still says "until carry 65 is met" (:3088), which is out of date.
- **Fix:** State that the brief carries TE-Q2's after-clause (advisor-te-m3.md:126-127), and that the stop clause's status is asked as a U4F-Q question, not decided. Flag the out-of-date card line for the day session.

### ADV-NP-5 (Low)

- **Where:** night-2026-11-16.md:3
- **What:** The header says "about 15:35 UTC", but the file was written at 15:21:28 UTC (file mtime; the card's :46-47 already notes this).
- **Fix:** Change it to "about 15:21 UTC".

### ADV-NP-6 (Low)

- **Where:** night-2026-11-16.md:126-127 (Done when)
- **What:** "If 06:45 comes first" gives no time zone. NIGHT_SHIFT.md's 06:45 is Amsterdam time (05:45 UTC in November).
- **Fix:** Write "06:45 Amsterdam (05:45 UTC)".

### ADV-NP-7 (Low)

- **Where:** night-2026-11-16.md:87-89 (shape item 4, Build rules)
- **What:** The build rules leave out §4.11's bans (u4-build-plan.md:392-407: HashMap/HashSet bans; clock bans in server, lab and bots). Those bans bind ClockSync, the 1.5x host clock and the lag switch, which must run on simulated time (plan.md:617-628).
- **Fix:** Add §4.11 (:392-407) to item 4.

### ADV-NP-8 (Low)

- **Where:** night-2026-11-16.md:106-107 (shape item 8, Recheck)
- **What:** The recheck covers only items that U4e, U4r or BRAWL-1 built. The brief also cites items from U4s, U4d, U4d2 and U4j (CMD_CATCHUP with carry 27's const assert, carry 18's route, check.mjs's 11 steps, README §4).
- **Fix:** Widen it to every code or file item the brief cites at 11fd190.

### ADV-NP-9 (Low)

- **Where:** night-2026-11-16.md:118-124 (the two checks)
- **What:** Both checks are asked to find each carry "present and testable as worded", but the draft holds only @@CARRY@@ placeholders, not the carries' text.
- **Fix:** Tell both checkers to read each carry's text at N/u4s-carries.md's line ranges.

### ADV-NP-10 (Low)

- **Where:** night-2026-11-16.md:12
- **What:** "Nothing the item reads changed after 11fd190" holds only at the time of writing. Evening decisions (KL-YES, C62H-*) or card refreshes may land before the night.
- **Fix:** Rephrase: the item reads 11fd190 only, and later changes are the morning recheck's job.

## Conditions

1. 1 (ADV-NP-1): the shape gains a "Review and launch" item that names U4f's reviewers per u4-build-plan.md:816 plus engine-architect for A19 (§7.2 :930-931), at xhigh and counted toward the 3-agent limit (advisor-u4.md:147); the builder's seat (§7.2 :925-927); the W1-TARGET lock and the lane runner; and a launch list on the model of u4r-brief.md §12 (:1845-1909). Check: the plan text contains "rust-reviewer", "engine-architect" and "at launch".
2. 2 (ADV-NP-2): shape item 2 gains the tests and negative cases (counts as deltas), each bite with its §4.14 site and plant (u4-build-plan.md:453-476), and -21's twins per §4.10, Q16 (a) (:1042-1045) and U4 condition 1 (advisor-u4.md:78-89, :477-479). Check: item 2 cites §4.14, Q16 and advisor-u4.md:78-89.
3. 3 (ADV-NP-3): item 9's "with its home's words" is replaced by routing by id and file:line range, with any word-for-word text as a placeholder @@WORDS-<file>:<a>-<b>@@ for the day session's script. Check: the phrase "home's words" no longer appears, and "@@WORDS-" does.
4. 4 (ADV-NP-4): the Read-first paragraph says the brief carries TE-Q2's after-clause (advisor-te-m3.md:126-127), and that the U4d-F TE2-5 stop clause's status is asked as a U4F-Q question, not decided. Check: advisor-te-m3.md is cited, and the item-10 questions list includes the stop clause.
5. 5 (ADV-NP-5, ADV-NP-6): the header reads "about 15:21 UTC", and "Done when" reads "06:45 Amsterdam (05:45 UTC)". Check: grep the plan for both strings.
6. 6: ADV-NP-7 to ADV-NP-10 are folded at the author's discretion. Any other change beyond conditions 1-5 comes back to the advisor before item 7 is posted to issue #16, and issue #16 links the plan by the SHA of the fixed file.

## Approved under his rule of 2026-11-09?

Mustafa's rule of 2026-11-09 (the adjusted version), held word for word on the card .checkpoints/netcode-proof-test.md:3376-3380 at 11fd190 and copied by script into the plan's :5-8. It makes scouting and plan drafts for the next unit, posted each evening to issue #16, approved Night queue entries, and it keeps building and testing for the day. U4f is the next sub-unit (u4-build-plan.md:882-883), and its brief goes to the advisor before its build (advisor-u4.md:157; Q21 :1054-1056). Item 7 drafts that brief, with no build, test or decision. This plan becomes the item's approved plan once conditions 1-5 are met; CLAUDE.md "Night shift" and NIGHT_SHIFT.md §7 ask for that approval. Precedent: night items 2 and 4 of 2026-11-10.

## To Mustafa

Nothing to decide. Tonight's night item 7 (a draft of U4f's build brief, no building or testing) fits your 2026-11-09 night rule and the night shift's rules. It is approved once the day session makes six small, checkable fixes, three of them substantive: name U4f's reviewers and launch steps, list its tests, bites and -21 twins, and make sure no agent copies ruling text or your words by hand. Your three open answers (KL-YES, C62H-KEEP, C62H-BOUND) stay open, and the draft only marks where they apply.

Decisions needed: none.

For Mustafa: nothing.
