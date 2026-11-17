# Advisor: MAIN-54 plan review (take main 82c605e into PR #40's branch) + DOCS-54 ruling
Started 2026-11-17. Read only. Plan file sha256 read: 3b446794b3f0c976235a887cf4510ad84c097d4697c37f7bf73a0a4870dad328 (matches the brief).

## My plan
1. Main's advisor.md read (git show origin/main:...): done.
2. Plan file read and hashed: done.
3. Facts: rev-parse origin/main; merge-base; diff --stat 53272a1 origin/main; diff --quiet NIGHT_SHIFT; diff --quiet spike tree vs ddd38ba; status --short; HEAD vs origin branch; 82c605e's parents and tree.
4. MAIN-52 words (history.md 2082-2086), his go on #54 (2199-2202), NS-STATUS (2182).
5. Precedent: card lines ~276-320, merge_main_53.py, 2bd44a5 message, counted-merge-2.log header and steps.
6. CLAUDE.md Never: switch rule vs `git merge --no-ff --no-commit origin/main` in main folder; what 2bd44a5 actually ran.
7. CHECKPOINTS.md §5, §9; save pushes.
8. U3C-HELD-1 ruling (c62h-bound-build.md 426-455), card Open findings U3C-HELD-1 and CHECK-NFF-1.
9. main-run.sh header/usage; step 4c command and pass bar; is check.mjs whole needed.
10. DOCS-54: ADV-NS-L5, branch rule, main's STATUS/DECISIONS KIT-FREEZE-2 record.
11. Verdict.

## Facts checked (one line each)
- F1 `git rev-parse origin/main` = 82c605e2e8a7f284e6925718368d3fe569f89ae9; parents 53272a1 20f9d59; tree c2b5dec = 20f9d59's tree. Matches plan.
- F2 `git merge-base HEAD origin/main` = 53272a11024dcec8349e67bd8c70a8db337e85bb. Matches.
- F3 `git diff --stat 53272a1 origin/main` = `.claude/NIGHT_SHIFT.md | 3 +++`, 1 file. Matches.
- F4 `git diff --quiet 53272a1 HEAD -- .claude/NIGHT_SHIFT.md` exit 0. Matches.
- F5 `git diff --quiet ddd38ba HEAD -- spikes/netcode-proof ':(exclude)spikes/netcode-proof/results'` exit 0. Matches.
- F6 `git status --short`: only `?? .checkpoints/netcode-proof-test/advisor-main-54.md` (my own notes file). Tree otherwise clean.
- F7 HEAD = origin/claude/netcode-proof-test-plan-hcky5g = 2a32497 (07:10:43 UTC, checkpoint "MAIN-54 plan written...; saver beoc3blfv"), NOT 4db26b2 as the plan's facts say; 4db26b2..2a32497 changes only the card and main-54-plan.md. The saver is running (beoc3blfv), as the plan foresaw ("the saver starts with this review"). Plan's HEAD fact stale but harmless; step 1 re-checks HEAD = origin branch.
- F8 MAIN-52, his verbatim words: history.md:2055-2057 ("4. Main is now 4460ef5...: PR #52 is merged (...). Do not merge it in now. Take main in when main changes code, CI or rules, and once before PR #40 is marked ready."). history.md:2082-2086 is the main session's "Read as" (label MAIN-52, adds "(the kit, gate or CI files, a CLAUDE.md, .claude/ or .github/)"). The plan (line 4-5) quotes the reading in quote marks as "Mustafa's rule". Meaning same; attribution loose -> finding Low (ADV-M54-L1).
- F9 .claude/NIGHT_SHIFT.md is a rules (kit) file -> MAIN-52 requires taking main in. Merge is due.
- F10 His go on #54: history.md:2199-2202, "Merge #54" (07:00:54 UTC), merge 82c605e 07:03:02 UTC, expected head 20f9d59; CI on main green (gate run 44035715175, toolchain 44035715349). NS-STATUS (history.md:2182) asked for "its own pull request".
