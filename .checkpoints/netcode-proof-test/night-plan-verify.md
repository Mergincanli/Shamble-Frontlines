# Verifier (references): night-2026-11-16.md, item 7, pinned 11fd190

Plan: check every reference in N/night-2026-11-16.md at 11fd190 (file:line, sections, carries, rulings, Qs), the SHA on origin, every file to read exists, then spot-check 15 scout-map entries; a line per finding. (No Edit tool here: this file is rewritten whole with Write.)

## Findings
- Setup: HEAD moved during the check (5afc772, ab5cfc1 commit the plan and the card); vs 11fd190 only the card, the plan, advisor-night-plan.md and these notes differ, so every other file is read from the working tree; the card from `git show 11fd190:`. The card's change after 11fd190 is only its LATEST status lines: the plan's "nothing the item reads changed after 11fd190" holds.
- SHA: 11fd190e95... is a commit, on origin/claude/netcode-proof-test-plan-hcky5g (git branch -r --contains). OK.
- Files: all 47 paths named in the plan or the scout exist at 11fd190 (git cat-file -e; line counts taken). OK.
- Header: card at 11fd190 :3362-3366 "Mustafa, 2026-11-09 (verbatim, the adjusted version)": words equal the plan's quote (card :25 names it too). night_1116_rule_words.py exists at 11fd190. OK.
- NIGHT_SHIFT.md step 4: "a step it does not cover ... stops the item under Decisions needed" (:30-33). OK.
- CHECKPOINTS.md §8 :190 "without moving the main folder" via a worktree; §4 :65-85 plan first, save as you go, absolute path :85. OK.
- CLAUDE.md "Models and effort": xhigh for netcode (:174-177). Agent types netcode-architect, test-engineer, security-auditor exist in .claude/agents/. OK.
- u4-build-plan.md:882-883 "Then U4r ... then U4f, U4g and U4h in order". OK. U4r merged (card :1550), BRAWL-1 closed (card :17). OK.
- u4-build-plan.md:1054-1056 Q21 U4-BRIEFS. OK.
- u4-build-plan.md:805-816 "### U4f, clock and commands" (size M; Builds :806-809, Marks moved :810-815, Reviewers :816). OK.
- advisor-u4.md:143-157 conditions 10-12; :157 "The five briefs (Q21) come to me before their builds". OK.
- advisor-u4.md:112-124 conditions 4 (:112-118) and 5 (:119-124). OK.
- ADR 0001 :1610 -20, :1611 -21, :1612 -22, :1644 -47. OK. Time paragraph :239-250/251. OK. OQ-0001-6 :1156-1167. OK.
- marks.json :4618-4778 -20, :4779-5119 -21 (8 parts, 13 bites), :5120-5202 -22, :9046-9109 -47 (two bites). OK.
- plan.md is rev 3.3 (:1); components :456-481, §6.2 rows :721-722. OK.
- u4-build-plan.md:338-341 §4.8. OK. "§4.10's 11 forged kinds" is :808's phrase. OK. :422-424, :921-922, §7.2 A19 :930-931, §9 :958-968 counts at :962-963. OK.
- FINDING 1 (Low): shape item 4 cites `u4-build-plan.md:909-932` for "the shared target, offline, touch, counts as deltas, the df floor". §7 has shared target (:911-912), touch (:913), counts (:914-915), but no "offline" and no df floor. Those are U4r's brief §4: CARGO_NET_OFFLINE `u4r-brief.md:940`, Disk floors `u4r-brief.md:968-971` (4 GB / 2 GB; runner FLOOR c-u4r-runs.mjs:131-132).
- check.mjs STEPS (:2601-2636) = 11 steps. OK.
- advisor-u4r-brief.md:289-359 U4R-Q14. OK.
- FINDING 2 (Low): shape item 3 cites `u4s-carries.md:9-14` for "reported met by number, or not met with why". :9-14 is the word-for-word carry rule; the report-by-number words are the advisor's U4s ruling condition 4, `advisor-u4.md:334-335` (U4r's form `u4r-brief.md:695-696`; card Rules in force U4 bullet 11fd190 :3070-3071). The scout's Carries "header" entry has the same mis-cite.
- u4s-carries.md entries: 46 :506-509, 80 :510-534 ((a) :513-518 ... (e) :529-534), 86 :535-537, 57 :223-228, 74 :343-354, 21 :148-149, 27 :166-168, 18 :129-132, 39 :169-179, 50 :180-183, 61 :229-247, 58 :269-273, 76 :364-371, 77 :372-381, 63 :322-327, 64 :448-473, 67 :476-493. All present; texts match.
- Card (11fd190) Rules in force :3013-3088 (U4j r2 :3073-3079, U4d-F :3080-3088); M6 line :1553 (TE-Q2) in U4r's merge block :1550; DEPTH :1930-1933; option-A note :1960-1964; JDG-ONLY-1 interim rule :1473; C62H-KEEP/BOUND/KL-YES open (:17-19, :492, :509; no Decision in history.md, which ends :2129). OK.
- FINDING 3 (Low): rule 4 attributes the Write/Edit sentence to "the advisor's U4d-F ruling, R1". advisor-u4df.md R1 (:28) is NF-M1 (the ES level); the sentence is R6's ruling (:301-320, :317-318), text T4 item 1 (:461-464); card :3080-3083 item 1. Words right. Correct: "the advisor's U4d-F ruling, R6 (T4 item 1)".
- advisor-u4r-build2-full.md:28 point 31. OK. history.md:2113-2129 (:2127). OK. advisor-c62h-review-full.md:89. OK.
- FINDING 4 (Low, wording): shape item 5 puts SERIES.md under "the reviewer's read that his rule of 2026-11-15 asks for". His words (:2127) name marks.json and the judge only; SERIES.md is the advisor's recommendation "though his words do not name them" (advisor-c62h-review-full.md:93). Cite :93 for SERIES.md.
- history.md:2039-2087 and :398-399 SIZE-CHECK. OK. marks/README.md:285-290. OK. u4-scout-netcode.md:122 F104 (only definition). OK.

## Scout spot-checks (25 entries)
- Sections: u4-build-plan §4.12 :409-425 OK; §4.14 :453-476 (:462, :463, :467, :475) OK; plan.md :617-628 (:625 rate 1.5) OK; catalogue :87 Input (2 x TICK_HZ) OK, :99 Kick (reasons replaced/malformed/overflow/stopped; "never for a forged kind" is inferred, explicit at u4-build-plan.md:383-384) OK; SERIES.md :21, :93, :128, :442 OK; ADR :226-255 (lead bound :231-233, catch-up :238-239) OK.
- Carries: 80 (a)-(e) OK; 74 OK; 76 OK; 57 OK; header :9-14 WRONG in part (Finding 2).
- Rulings: JDG-ONLY-1 c62h-full :8-20, :44-45 OK; EVQ-LOAD p3-full :156-158 OK; F2 advisor-u4 :436-465 (:463-465) OK; U4E-Q12 u4e-brief :217-244, :340-341 OK; U4E-Q11 wave1 is :492-495 (scout :491, one early) OK; U4R-Q4 :198-205 OK; NC-U4R-5/EA-U4R-7 ":61, :80": :80 right, :61 is NC-U4R-1's Why; correct NC-U4R-5 :78-81, EA-U4R-7 :200-204 (Low).
- Open items: TE-B1-R2-1 report :1137-1141, card :163-171, bots :130-132, :404 OK; RR-B1-7 (1) logs.rs:495 room_tick OK (report :1162 points to stale :402-405); C62H-BOUND c62h-full :59, :63, :75, :132 OK.
- Code lane: input.rs :1-5, :61, :64, :80, :129 OK; client lib.rs :214, :267, :298, :414, :429, :450, :492, :528, :583, :645, :810 OK; pump.rs :112, :204, :372, :500-517 OK; room.rs :361-369, :460, :1402 OK; scenario.rs :18-19, :126, :848, :851, :1405 OK; c-u4r-r2.mjs SUITES :77-92 (-21 :82-85) OK; events.rs :26, :40 OK.

## Not verified
- Issue #16's night-shift comments (2026-11-14 00:11 and 23:11 UTC) and PR #40: GitHub, no web allowed.

## Verdict
PASS: 4 Low wrong or partial cites (corrections above) and 2 Low scout mis-cites; none sends the night to the wrong text for its work (the sentence is given whole; the other cites reach the right text for their main claim). Refs checked: about 80 in the plan + 25 scout entries + 47 paths.
