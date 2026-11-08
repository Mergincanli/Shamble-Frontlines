# Verifier: kit-v3-4 (A4, the review policy), head ad1f9bd vs origin/main 626df67

## Plan
1. E1: his words byte for byte (kit files, VISION_V2 + register for XP-TERM (a), STATUS for LOOK (a),
   CHECKPOINTS.md §10 for disk hygiene, STATUS queue removal with pointer). C12: not yet checkable (report comes later).
2. E2: rule-diff table (kit-draft.md §8) vs every changed/added sentence in the five kit files; verdict per row.
3. E3: three kit test files on head and on main (temp checkout under .gate/).
4. E4: gate.mjs origin/main --no-smoke; repo-map-check.
5. E5: STATUS, ROADMAP, START_HERE, REPO_MAP facts (merge times, SHAs, run ids, line numbers); docs parts.
6. E6: scope (no hooks/settings/CI/gate changed; CLAUDE.md growth <= ~25 lines).
7. C1-C13 and the addendum's two text changes.
8. Lines > 120 chars outside table rows, STATUS:3, ROADMAP entries.

## Results
(filled as they come)

### E1 (his words), done
- Script `.gate/verifier-a4/e1.py` (sources: origin/main STATUS's A4 block, joined; the three answer files' text
  blocks): 17/17 PASS, FAILS 0. advisor.md quotes items 1, 2, 3 and 5 whole, byte for byte (joined); SPIKE_PLAN item 4
  and "next to the fast checks on save"; ADR 0006 and README item-6 fragments; XP-TERM (a) in VISION_V2 (whole),
  README:183 (first clause), STATUS (whole, with "XP-TERM: (a), "); "LOOK: (a)." in STATUS; disk hygiene in
  CHECKPOINTS.md §10 (whole, from "clean finished ..." to "... stays denied.").
- STATUS queue: A4's quote, A4's addition, A4-FREEZE and the session's notes all removed; pointer to
  `docs/reports/2026-11-08-kit-v3-4.md`'s appendix present (STATUS:563-568).
- Added quotes not found in a source (`.gate/verifier-a4/quotes.py`): none of his; only the docs' own phrases.
- C12 (the report's appendix): not yet checkable; the report does not exist at ad1f9bd.

### E3 (kit tests), done
- Head ad1f9bd: routine-guard 1034/1034, settings 127/127, checkpoint 27/27 (fail 0 each).
- Main 626df67 in a detached git worktree under `.gate/verifier-a4/main-wt` (removed after): 1034/1034, 127/127,
  27/27. Equal counts. (A `git archive` copy without `.git` failed settings S4 "git switch in the main folder asks":
  the test needs a git repository; environmental, not a defect.) `.claude/hooks`, `.claude/scripts` and
  `.claude/settings.json` are byte-identical to main (`git diff --quiet`).

### E4 and E6, done
- `node tools/gate.mjs origin/main --no-smoke`: GATE PASS, exit 0; [1] 5/5 parse, [2] PASS, [3] +0 −0 (core 138 / game
  714 / editor 353), [4] 51 guarded 0 flagged, [5b]-[5d] PASS, 5a skipped (`.gate/verifier-a4/gate.txt`).
- `repo-map-check.mjs`: OK, 114 entries cover 101 required paths.
- E6: `git diff --name-only origin/main...HEAD` holds no `.claude/hooks/`, `.claude/settings*`, `.github/`, `tools/`,
  `.npmrc`, game file or page. Files outside `.checkpoints/`: 5 kit files + 9 docs. CHECKPOINTS.md (addendum, his
  DISK-WHEN (a)), START_HERE and REPO_MAP (docs-keeper drift) are beyond plan §3's list but each is approved or a
  docs-keeper duty. Line counts: CLAUDE.md 252 → 277 (+25; numstat +41 −16), advisor.md 172 → 301, SKILL.md 32 → 36,
  security-auditor.md 54 → 56, CHECKPOINTS.md 208 → 220. C13's "about 25 at most": met (+25).

### E2 (the rule-diff table, kit-draft.md §8, 55 rows), done
- Every changed or added rule sentence of CLAUDE.md, advisor.md, SKILL.md and security-auditor.md in the actual diff
  maps to a row (CLAUDE.md rows 1-21; advisor.md 22-49; SKILL.md 50-53; security-auditor.md 54-55). The applied text
  equals the draft with the markers removed (TIER0-SCOPE (a), REVERT-EXCEPTION (a) variants).
- Verdicts checked: right. Every "looser" row has his A4 words or his 2026-11-08 answers behind it (rows 17, 19, 21,
  45: REVERT-EXCEPTION (a), "day sessions only: open the pure revert at once; the advisor checks it's an exact undo,
  and it merges by today's rules", matched by CLAUDE.md:112-115 and advisor.md:260-274). Row 11 could read "same" (a
  follow-up is an explicit deferral); "looser" is the cautious label. No loosening found beyond his words.
- Missed: (1) CHECKPOINTS.md §10, the fifth kit file, has no row (the table predates it; the addendum asked for one
  reading "guidance"). My rows: heading "guidance, no rule change" same; his quote same; "Who runs it" same (states
  the guard: checked per seat, below); "cargo clean" bullet same (guidance); "git worktree remove, never --force" same
  (settings deny :214-216, guard deny); "To consider" same; "rm -rf stays denied" same (settings :247). (2) advisor.md
  tier 1's "Check" line (:220-222) has no row; verdict same (restates step 4, step 2 and category 1).
- §10's guard claims, run per seat (`.gate/verifier-a4/seat.mjs`, guard only decides): day: cargo clean, worktree
  remove, rm -rf, git revert: no guard decision (rm -rf denied by settings :247; worktree allowed by settings :28);
  `--force` deny. night-shift: cargo deny, worktree remove no decision (allowed), --force deny, rm -rf deny, git revert
  deny. pr-advisor: all five deny. Matches §10:214-215 and CLAUDE.md's Rollback "the night shift and the PR Advisor
  only report it".

### E5 (docs true), done
- GitHub API (read-only `gh api`): #43 merged_at 2026-11-07T11:28:59Z, merge 69ea88e (parents 4c6db5b, 4e03bb5),
  head 4e03bb5; comment 6701970962 on #43 by Mergincanli, "APPROVE WITH NOTES, head 4e03bb50..."; run 42368356165
  gate, push, main, 69ea88e, success. #45 merged_at 2026-11-07T20:52:18Z, 626df67, head 6ae3571; run 42413120827
  gate, push, main, 626df67, success; the merge message records his go after the advisor's APPROVE of 6ae3571...
  (STATUS:3's #45 clause holds; the run id itself is cited nowhere, "CI green on main" is true).
- Commits cited (d8147cd, ebbdaa2, 21d2b56, b85b2cf, 202951f; 4e03bb5 = #43's notes deletion) exist with those
  contents. ROADMAP Recent work: 12 entries; R1-R6 folded into Build history.
- Line numbers checked true at head: ADR 0006:482-483 ("private flag", "humans and bots per team"); VISION_V2 block
  :71-78, pointer :72-75, :85 "Modes as data"; ADR 0005 ranges (§2 27-43, §3 45-52, §4 54-58, §6 83-102 from the
  headings at :27, :45, :54, :83, :104); SPIKE_PLAN note :438-442; ADR 0006 notes :431-433, :461-465, :1026-1029,
  :1564-1565 (bullet starts :1562); 0002:472-474 and :570 (G1 format); 0003:1245 (slice 6); ADR 0006 :1019 "not read".
  At 69ea88e: STATUS:681 holds the EDITOR_V2 :94/:95 citations, STATUS:107 and ROADMAP:94 said "four register rows".
- WRONG at head: register citations written before main 626df67 was merged in. #45 added a register row at
  README:88, so every row after it moved +1 (R2-ROOM-OQ7 is :184, OQ-0003-15 :104, R6-TEL-OQ2 :203, I-18 :356).
  This change-set's own new text: STATUS:528 "`docs/adr/README.md:183`" (kit-v3-4's record: should be :184); vfx
  report :88 "README:183" (same); vfx report :73 and :83 "README:103" and :78 "README:202" (right at 69ea88e, the
  report's frame, but not anchored there). Rewritten paragraph, older text: STATUS:539 "README:103" (:104). Main's
  own drift from #45 in lines this PR did not touch: STATUS:116 (:160), :117 (:119), :465 (:294), :466 (:119), :484
  (:202), :485 (README:192), :492 (:160), :495 and :545 (:408; J3 is :409).
- Docs parts (C10, C11): no new register row (README +0 rows); ADR 0002, 0003, LOOK_V2 untouched; VISION_V2 tags
  unchanged (DECIDED 13, OPEN 4, LATER 5, DEFAULT 3); no slice mapped to "first playtests".

### Conditions C1-C13 and the addendum
- C1 met: CLAUDE.md:93; advisor.md:55-57 ("Tiers add, never remove"), :81-92 (step 0, 2, 4 triggers, categories 1-4).
- C2 met: advisor.md:58-75 (tier 0 file set; decision docs tier 1 always needs-mustafa; tools/ tier 1 except gate files;
  kit Markdown tier 2). His TIER0-SCOPE (a) matched (advisor's list is wider: stricter).
- C3 met: advisor.md:76-79 (no session or agent carries out the act; his explicit decision + his go; Never lines
  absolute), :43-44 (money = any act that creates a cost; infra-engineer.md:40).
- C4 met: SKILL.md:25-27 ("Fix and re-run until PASS" kept, no "within the budget"; outside the budget; FAIL = High);
  CLAUDE.md:105-106, :110-111.
- C5 met: CLAUDE.md:107-111 (floors; major/minor/nit mapped); security-auditor.md:52-54; SKILL.md:33 (labels).
- C6 met: CLAUDE.md:102-106, :83; advisor.md:240-252, :198-199.
- C7 met: CLAUDE.md:114; advisor.md:273-274.
- C8 met: advisor.md:257-275 (day only; re-run once, pinned; worktree from origin/main; ask before git revert on kit or
  gate files, L4; exact inverse + later-merge check; re-landing a new change-set); advisor.md Never :298.
- C9 met: advisor.md:205-235 (every tier: confirm the tier, step 2 per seat, "never take CI's green alone as enough in
  a day session", no .checkpoints/, STATUS/ROADMAP; tier 2 "As the routine" :225-226; "ignore nothing that touches
  security" :231).
- C10 met (E5 above). C11 met: README:184 R2-ROOM-OQ7; STATUS:506-511; Low note 2 at STATUS:116 and ROADMAP:97;
  vfx report :71-90.
- C12 NOT YET CHECKABLE: `docs/reports/2026-11-08-kit-v3-4.md` does not exist; advisor.md:47 and STATUS:567 point at
  its appendix.
- C13 met: E3 three files equal counts; E6 clean; CLAUDE.md +25; E2 complete for the four kit files (not for §10).
- Addendum change 1 met: CHECKPOINTS.md:214-215 (first two sentences; the optional third left out, as allowed).
  Change 2 met: :217-218 "Git refuses a copy that still has unsaved work; that copy stays until its work is saved."

### Line lengths
- Added lines over 120 characters outside table rows, STATUS:3 and ROADMAP entries: one, security-auditor.md:3, the
  one-line YAML `description:` (337 characters; 270-odd on main already). Nothing else.

## Verdict: PASS WITH NOTES
Nothing Critical or High. His words byte for byte (17/17), kit tests equal to main, gate PASS +0 −0, repo map OK, scope
clean, C1-C11 and C13 met, the addendum's two changes in. Findings:
- **Medium M1** (E2 incomplete): `.checkpoints/kit-v3-4/kit-draft.md` §8 has no row for `.claude/CHECKPOINTS.md`
  §10 (:209-220), the fifth kit file. Fix: add the rows above (all "same"; guidance, guard and settings unchanged) to
  the table the report carries.
- **Low L1** (stale line numbers after #45): `docs/STATUS.md:528` "`docs/adr/README.md:183`" → `:184`;
  `docs/reports/2026-11-07-vfx-xp-notes.md:88` "README:183" → ":184"; same report :73, :78, :83 (README:103, :202):
  add "at 69ea88e" or +1; `docs/STATUS.md:539` README:103 → :104. Main's own #45 drift (STATUS :116, :117, :465,
  :466, :484, :485, :492, :495, :545): a STATUS follow-up for docs-keeper.
- **Low L2** (E2 gap): advisor.md tier 1 "Check" line (:220-222) has no row. Fix: add it, "same".
- **Low L3** (CLAUDE.md's short table vs advisor.md, all in the stricter direction or restated there):
  `CLAUDE.md:97` row 0 "docs with no word or decision of Mustafa's" reads as the doc holding none (every STATUS edit
  would be tier 1), where advisor.md:58-59 says the PR "adds or changes no word or decision"; row 0's "CI + the PR
  Advisor" omits the change type's evidence and reviewers that advisor.md:56-57 applies at every tier (a research
  doc's verifier spot-check, CLAUDE.md step 4); `CLAUDE.md:98` row 1 "`ready`: the PR Advisor" does not say decision
  docs are always his go (advisor.md:63-64). No rule is loosened (advisor.md governs, CLAUDE.md:91 points there). Fix,
  at a later kit pass or now if a push is needed anyway: "that adds or changes no word ..." in row 0 and one clause
  "steps 4 and 5 apply at every tier; decision docs always need his go".
- Not verified: C12 (the report does not exist yet); the smoke check (`--no-smoke`; CI runs it); the live branch
  protection and routine settings (no change touches them). `.checkpoints/` files exist (expected in a draft; delete
  before ready).
