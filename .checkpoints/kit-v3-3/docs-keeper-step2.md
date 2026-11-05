Task: docs-keeper, kit-v3-3 (A3) step 2 of plan rev2 (section 8, C9): STATUS Follow-ups bullet + "Queued, in Mustafa's words" block + PR #37 merged / A3 in flight (PR #38 draft).
Plan (blocks, in order):
B1 read sources: mustafa-words, checkpoint, plan-rev2 sec 0/4.8/8, advisor C9, queue files (docs-pass-items, a4-spec, 2026-11-03 list)
B2 read STATUS.md (lines ~1-30, ~270-290, ~500-520) and ROADMAP lines on PR #37
B3 classify docs-pass-items: open vs closed (with evidence)
B4 edit STATUS: PR #37 merged lines (:3, :22-24, :512)
B5 edit STATUS: Follow-ups bullet (~:277)
B6 edit STATUS: Queued, in Mustafa's words block
B7 ROADMAP check (false lines only)
B8 E9 word-for-word diff of quoted blocks vs sources
Notes:
- B1 done: sources read. Task bullets name Follow-ups, Queued block and the PR #37/A3 lines; plan step 2 also says "stated limits" (L1-L11 in A3's Open-bugs entry): not in the task's bullets and A3 is unbuilt, so left for step 8's pass (to flag in reply).
- B2 done: STATUS Follow-ups at :277-281; PR #37 "in flight/ready" at :3, :22-24, :89 (In flight row), :147 ("land with PR #37"), :278, :377, :512. Facts verified: `gh api repos/Mergincanli/Shamble-Frontlines/pulls/37` -> closed, merged_at 2026-11-04T23:06:24Z, merge_commit_sha 12d7961c3e8d..., head 342ff49e5958...; `git log -1 --format='%P' 12d7961` -> 8bcc65e 342ff49 (merge commit). PR #38: open, draft true, head claude/sleepy-tesla-ofsxsr, title "kit-v3-3 (A3): the guard's follow-ups (draft: plan in progress)". Mustafa's go: [assumed] from the task text and mustafa-words.md ("after PR #37's merge"); advisor APPROVE of 342ff49: docs-pass-items.md:123 (comment 6643739289).
Next: B3.
- B3 done: docs-pass-items.md classified (line refs into the queue file "q:"; STATUS/CLAUDE lines at fc67f61).
  CLOSED (no STATUS line; evidence):
  - q:6 `date` in routine shell list: CLAUDE.md:135 names "`date`".
  - q:7-9 trigger rule: CLAUDE.md:140 "never by pull-request events"; STATUS:47-50, :468-470.
  - q:11 #33/#30 merged: STATUS:11 (e4d8b22), :16 (e2b5799).
  - q:12 the advisor's nit "a draft while A1 runs": [assumed] closed: no doc line matches (grep "while A1|draft while" in docs/, .claude/, CLAUDE.md, git log: no hits); it belonged to the docs pass that became PR #35 (STATUS:91, merged), and PR #30 (A1's PR) merged (STATUS:16), so the state it described is past. Flag in reply.
  - q:13 KT14 nit: STATUS:91 ("LEGACY_INVENTORY KT14").
  - q:15-37 role environments, Part A, B1, B2 rows, old #26 session: STATUS:46-51, :485 (B2 row).
  - q:38-41 actions_get / rev-list: A3's own scope (STATUS:527-540, A3 entry; mustafa-words item 3); carried as one Follow-ups pointer line to A3's entry.
  - q:39-41 label-by-number tool error: STATUS:543.
  - q:42-43 B2 report-times nit: docs/reports/2026-10-27-kit-v3-1.md:169 gives both runs (09:30:25–41 and 09:30:23–38).
  - q:44-52 PROBE-FILE: STATUS:180, :505-509.
  - q:53-56 ROUTINE-EFFORT: STATUS:185, :485, :503.
  - q:57-62 PR #26 follow-ups: A3 itself (STATUS:527-534).
  - q:63 Guard test B1: deleted, STATUS:177, :492.
  - q:64-72 PORTAL-EMBED: STATUS:189 (OQ-0002-9, OQ-0006-23), :91.
  - q:75-80, :86-87 #35 items 1-3, 6: "Taken by B3 (PR #36)" (q itself; STATUS:91 "taken by B3"); item 4 done 2026-11-04 (q:81); item 7 "No change needed" (q:89).
  - q:105-108 #36 items 3-4: taken by A2 (PR #37) (q itself; STATUS:90 B3 row records merge commit).
  - q:121-122 A2 item 4 REPO_MAP row: fixed in A2's last commit (q itself).
  OPEN (each gets a STATUS Follow-ups line): q:82-85 #35 item 5 remainder; q:90-95 night and PR Advisor runs (STATUS:471-474 "scheduled-start proof is still open"); q:98-104 #36 items 1-2 (already in STATUS:278-279, kept); q:109-111 #36 item 5 Playwright 1194 (no hit for 1194 in tools/README.md or STATUS); q:114-115 A2 item 1 E9 untested (STATUS:515); q:116-118 A2 item 2 (CHECKPOINTS.md §7 at :173); q:119-120 A2 item 3 toy-box (STATUS:377-379); q:123-128 F-1, R-1, R-2. Plus plan §4.8 items 8-10 (readers' points: settings.json:49-50 `node .gate/*.mjs|*.js` [verified]; WebFetch list has no code.claude.com/docs.anthropic.com [verified: grep settings.json no hit]; routine-guard.mjs:297 `insideDir(p, input.scratchpad_dir…) return null` before any protectedRel check for outside-project paths [verified]; F2-F4 cuts; Windows follow-up stays at STATUS:520-523).
  CHECKPOINTS.md trust-rule sites :77-78, :110-111, :119 [verified: each holds the "trust complete blocks / never reread" text].
Next: B4.
- B4 done (STATUS line numbers after the edit): :3 Last updated -> 2026-11-05, PR #37 added to the merged list (12d7961, 23:06 UTC, merge commit, go after APPROVE of 342ff49), In flight -> kit-v3-3 PR #38 draft; :22-28 Phase: kit-v3-2 moved to done, In flight -> kit-v3-3, A3 dropped from the Queue list (queue now points at "Queued, in Mustafa's words"); :91 new In-flight row kit-v3-3; :92 kit-v3-2 row -> merged / done / none; :380 and :515 "ready for review" -> "merged 12d7961". :150 "All four land with PR #37" left (still true). Next: B5.
- B5 done: Follow-ups bullet rewritten at STATUS.md:280-333 (built by .checkpoints/kit-v3-3/dk-step2-build.py, then two hand fixes: A3-items wording "and the points its plan weighs"; the Playwright line "the cloud container has 1243" [verified: `ls /opt/pw-browsers` -> chromium-1243]). Kept: LATER line, B3 report line (now with its full source text); TOY-BOX marked done. New: A3 pointer, R-1/R-2, F-1, B3-meaning, Playwright, routine proofs, CHECKPOINTS §7, E9 untested, toy-box tone, readers' points, A3 cuts (C3), Windows pointer.
- B6 done: "Queued, in Mustafa's words" at STATUS.md:334-369 directly under Follow-ups: A4 (quote, :338-345), A4's addition (:347-351), a 4-line "For A4's plan (the session's notes, not his words)" summary of a4-spec's notes (:352-355; added so they are not lost with the notes folder), B.1 and B.2 with his list's header line (:358-371). Lines 337 (122) and 345 (123) are quoted lines over 120: the source's own wraps + 4-space indent, kept to stay identical.
Next: B7.
- Correction to B5/B6 line numbers (checked with grep after the edits): Follow-ups STATUS.md:280-327; Queued block :328-367 (A4 :330-340, addition :341-347, A4 plan notes :348-351, B.1/B.2 :352-367).
- B7 done: ROADMAP had two now-false lines: :72 "kit-v3-2 (A2), in flight as PR #37; the guard's follow-ups (A3)" -> "merged as PR #37; ... (A3), in flight as PR #38, a draft"; :92 "[2026-11-04, PR #37, ready]" -> "[2026-11-04, MERGED] — kit-v3-2 (PR #37, 12d7961, 23:06 UTC, by merge commit, on Mustafa's go after the advisor's APPROVE of 342ff49; ...". No new Recent-work entry (A3 has not landed). Next: B8.
- B8 finding: my first diff run had two range bugs in the check commands (A4 range started one line early; the B awk printed the A section too); the content was not wrong. Rerun with corrected ranges:
- B8 done (E9 word-for-word for item 5; scratch copies in .gate/kit-v3-3/):
  - A4 quote, STATUS.md:332-340 with the 4-space indent removed vs a4-spec-2026-11-04.md lines 5-13 (the ">" lines before "### Addition"): `diff` -> identical (byte for byte per line).
  - A4's addition, STATUS.md:342-347 vs a4-spec lines 16-21: `diff` -> identical (byte for byte per line).
  - B list, STATUS.md:354-367 vs the code block of mustafa-2026-11-03-list.md (its line 1 "Small change-sets…" and "B. Docs:" to the end): after removing the "    > " quote prefix and joining wraps (whitespace collapsed): `diff` -> identical, 204 words each. Differences are whitespace only: B.1 and B.2 are re-wrapped at ~120 (source: one line each) and the source's leading space before "1." and "2." is dropped (markdown list inside the quote). Lines A. to A.2 of his list are left out on purpose (done: PR #30, PR #37), said in the block's header.
  - Mustafa's sentence in the Follow-ups header ("Record them in the repo, in STATUS's follow-ups, alongside A3's plan.") matches mustafa-words.md (grep: 1 hit each).
  - Not word for word, by design and labelled: the "For A4's plan (the session's notes, not his words)" lines (STATUS.md:348-351) summarise a4-spec's "For A4's plan to work out" section.
DONE
