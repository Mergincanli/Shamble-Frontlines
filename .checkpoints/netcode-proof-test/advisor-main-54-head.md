# Advisor: MAIN-54 pushed-head check (head 20048a5)

Started 2026-11-17 (after 07:31 UTC). Read-only. main's advisor.md read (git show origin/main:.claude/agents/advisor.md, 335 lines).

## Plan
1. Read the plan's "As approved" section (main-54-plan.md) and my conditions C1-C7 (advisor-main-54-full.json) in full.
2. Read the card's Base line and the "**MAIN-54 BUILT (07:28-07:31 UTC):**" block; gate.log; light.log; main54_counts.py.
3. Git checks: merge parents; diff --stat 3f2fa21^1 3f2fa21; kit equality origin/main vs 20048a5; spike tree ddd38ba vs 20048a5;
   diff --stat 3f2fa21 20048a5; commit message of 20048a5; later commits on the branch (checkpoint-only?).
4. Each condition C1-C7 against record and git.
5. CI: the four runs on 20048a5 (gate push/PR, toolchain push/PR), jobs incl. smoke; re-read until done.
6. Record errors (times, counts, words). Verdict.

## Log
- Read: advisor-main-54-full.json whole (C1-C7, findings ADV-M54-1..9, docs54); main-54-plan.md whole (83 lines; "As approved" :3-10).
- git: 3f2fa21 parents 3c65d14 + 82c605e (cat-file); message git's own "Merge remote-tracking branch 'origin/main' into claude/netcode-proof-test-plan-hcky5g"; signed.
- git diff --stat 3f2fa21^1 3f2fa21: `.claude/NIGHT_SHIFT.md | 3 +++`, 1 file. OK.
- 20048a5^1 = 3f2fa21. git diff --stat 3f2fa21 20048a5: card +17/-3 (wait: 17 lines changed), main54_counts.py +18 (new). Only .checkpoints/. OK.
- Kit equality origin/main vs 20048a5 (C3 list): exit 0. Extra protected globs (shrinkwrap, **/CLAUDE.md, AGENTS.md, .npmrc, .mcp.json): exit 0. 3-dot protected list: empty. merge-base 20048a5 origin/main = 82c605e = origin/main.
- Spike tree ddd38ba vs 20048a5 (excl results): exit 0.
- 20048a5 message: MAIN-52 record (his words :2055-2057, reading :2082-2086), base 53272a1, no conflict, nothing else changed, why git's message; both attribution lines; no [skip ci]. OK. Times: 3c65d14 07:28:05, 3f2fa21 07:28:47, 20048a5 07:31:31 UTC.
- history.md:2054-2057 his item 4 (the sentence at :2056-2057); :2082-2086 the reading (MAIN-52). Citations hold.
- Card Base line (card:6-7 at 20048a5): "main, last merged in at 3f2fa21 (main 82c605e, PR #54; MAIN-52); before that at 2bd44a5 ..." = C6.3's words plus history. OK.
- Card "Last update" (card:9-10 at 20048a5) still reads 07:06 UTC, "next MAIN-52 ...": stale (Low).
- gate.log: GATE PASS (5a skipped --no-smoke), base 82c605e, symbols +0 -0; repo map OK 131/110. C5 met (saved by Write, per its own header).
- light.log every line vs C4: lock line; pwd/free 4951 MiB (>= 4096)/HEAD 3f2fa21; diff+untracked both e3b0c442...b855 (full); refs PASS "1350 tracked files: none outside spikes/ references it (§4.9's one exception: ...)" exact; marks exact; self "78 of 78 tests passed" exact; redact "10 of 10 tests passed; redact-check: 305 item(s) checked, 0 finding(s)" exact; "check: all 4 steps pass"; "exit 0"; provenance empty, other cargo empty, "end 07:30:36 UTC, exit 0". No FAIL/other check line. C4 met.
- Counts: git ls-tree 3f2fa21 = 1350 (N), spikes/netcode-proof = 305 (M); 20048a5 = 1351 (+main54_counts.py); 3c65d14 = 1350. Status --ignored spike folder now empty. main54_counts.py read: read-only (git ls-files, status, rev-parse; prints only).
- Later commits on top: a62d64a 07:32:37, ad971ac 07:33:01, e6925c1 07:33:32, all [skip ci]; git diff --name-only 20048a5 e6925c1: only .checkpoints/ (card, my notes). OK.
- C1 saver: checkpoint.mjs:192-200 on SIGTERM saves (background) then logs "watch stopped: terminated"; save logs "saved and pushed <sha>" (:185). Output "[killed]" = no last line. But reflogs: 3c65d14 "autosave (terminated)" committed 07:28:05 (HEAD reflog), origin ref "update by push" at 07:28:23; merge 3f2fa21 at 07:28:47 with ^1 = 3c65d14; no other commit between. So the save finished and was pushed before the pre-checks; nothing moved HEAD after. Met in substance; letter not (record says it: card e6925c1:268-270). Low: the BUILT block (card 20048a5:248) says "its last save 3c65d14, pushed" without noting the missing last line (fixed later in the Saver line).
- Push times: 20048a5 pushed 07:31:33 (origin reflog) = record. Saver b2f2ov1so started 07:31:41 (card e6925c1:268), before this check (07:33). C6.4 timing met; its command form not visible in git (not verifiable by me).
