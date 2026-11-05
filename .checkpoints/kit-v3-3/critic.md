Task: kit-v3-3 (A3) critic of plan-rev1 before the advisor; notes only, no edits elsewhere.
Plan (blocks):
B1 read mustafa-words, checkpoint, plan-rev1
B2 read reader-askfirst / reader-routines / reader-secrets
B3 check ask-first holes vs code (settings.json, hooks, guard files)
B4 secret patterns: FP/FN test (scratch script)
B5 conflicts with CLAUDE.md / CHECKPOINTS.md / NIGHT_SHIFT.md; routine rules; read tools that act
B6 item 4 worktree rule: grep every place that says to switch main folder; enforcement of checkout/switch/reset
B7 item 5 queue docs-pass-items durable homes
B8 write findings section
Notes:
- B1 done (plan-rev1 is the uncommitted working copy, 396 lines). Next B2.
- B2 part: reader-secrets, reader-askfirst read.
- B2 done (all three readers + probe read). Next B3 code check.
- B3 partial notes: settings deny already blocks MCP push_files/create_or_update_file/delete_file (settings.json:256-258) -> not a hole. sh -c/bash -c/eval denied statically (:242-245).
  Finding: `node .gate/*.mjs|*.js` pre-allowed (settings.json:49-50) + CLAUDE.md code rule "Multi-site mechanical edits: an AST script" => sanctioned script path writes kit files with no ask and no prompt; plan lists scripts only as limit.
  Finding: dir-level targets (rm -r tools, cp -r x/. ., mv tools) not protectedRel (protectedRel('tools')=false, '.'=false) routine-guard.mjs:27-38.
  Finding: day git writers missing from §2 limits: git rm/mv, revert, merge in a worktree, config --file, format-patch -o, archive -o.
  Finding: WT-DAY exempts `git switch main` by name; local main can be old (an old commit) -> use protected-tree check vs origin/main like WT-NIGHT.
  Finding: coverage level never put to Mustafa as a decision (only §2) -> add AF-COVER.
  CRITICAL: protectedRel('.gate/x/.claude/settings.json')=false, '.gate/x/tools/gate.mjs'=false (node probe); kitPath "projectRel first" misses the .gate/<slug> worktrees the plan itself prescribes (CLAUDE.md Never bullet, CHECKPOINTS §8, advisor.md day seat).
  MAJOR: local main = b6effde, behind origin/main by 101, has no guard.mjs (git ls-tree) -> WT-DAY exemption of `git switch main` loads guardless rules.
  Finding: START_HERE.md:22 "cut from an up-to-date main" not in plan's edit list (same as SKILL.md:11).
  Finding: WT-DAY dropped reader's cherry-pick/am/apply/read-tree/revert/stash apply in main folder (they load a PR's hook code too).
  Finding: WT-NIGHT target 'cs/x' with no local branch (fresh container, switch DWIM) -> diff fails -> "fetch first" deny on every resume; resolve origin/<name>.
  Finding: gh() has no vcwd param (routine-guard.mjs:562) -> plan's gh pr checkout main-folder test needs it.
- B3 done. Next B4 secrets.
- B4 done: critic-esc.mjs: A misses JSON \b \f (and \0 \v), QP =2C, %u002C, \u0040 in URL creds; FP Windows \tsk- path (nit). Next B5.
- B5 notes: ACTIONS_READ method names match the tool schema (ToolSearch actions_get/actions_list enums). get_job_logs schema: return_content "Returns actual log content instead of URLs" -> default returns URLs, so excluding get_workflow_run_logs_url "because signed URL" is inconsistent (minor).
  Conflicts: CLAUDE.md:213-217 (agents build kit in worktrees) vs AF-AGENT deny; CHECKPOINTS.md:84-85 "main checkout's .checkpoints/" vs session in a worktree; CLAUDE.md:141-143 + guard.mjs:8-9 "session keeps hook settings" vs plan's Never-bullet "loads settings"; plan 4.6 "the only switch a routine makes" false (night switches); CLAUDE.md Never bullet lists no night exception vs NIGHT_SHIFT:26 new text (NIGHT_SHIFT:2 "Follow CLAUDE.md in full").
- B5 done. B6 (item 4) findings already noted in B3. Next B7 queue.
- B6 done (item 4): places that switch: SKILL.md:11, START_HERE.md:22 (missed by plan), NIGHT_SHIFT.md:26, advisor.md:72-74 (main only, fine), advisor.md:91 day seat (place unstated), CHECKPOINTS §8:189; already right: render-ab SKILL:8, verifier.md:20. tools/ and CI: no switch (grep). Enforcement today: checkout/restore/reset --hard refused by guard every session (routine-guard.mjs:202-205, :502-508) and settings deny (:178-196); git switch allowed (:25); gh pr checkout no rule; all only in sessions whose main folder has the guard (this session's e4d8b22 has none). git --outp real-git test was refused by the permission system (my scratch run): E4's "--outp writes" remains a prediction.
- B7 done (item 5): every docs-pass-items point maps to a plan home or a closed proof (PORTAL-EMBED in adr/README:17,21,91; trigger STATUS:48,468; TEST-RUN STATUS:50). Gaps: plan never says which "three deferred points" it reads (likely PR #37's F-1, R-1, R-2: docs-pass-items:123-127); closed list omits the Routines rows (B1/B2 passed) though STATUS:50 has them; mustafa-words.md itself (A3 scope + additions) gets no named durable home after the notes folder is deleted.
- B8: writing findings below.

## Findings (B8)
C-1 CRITICAL §4.1 kitPath/§5: projectRel-first misses kit files in worktrees under the project's .gate/ (the place the plan's own Never bullet, CHECKPOINTS §8 and advisor day seat send work): protectedRel('.gate/x/.claude/settings.json') and ('.gate/x/tools/gate.mjs') = false (node probe on routine-guard.mjs:27-38). Fix: always resolve the path's own git top first (same common dir), protectedRel relative to that top; tests A9b/A9c; note routines' writePath/commit check (:287-299, :532-538) have the same blind spot for .gate worktrees.
C-2 MAJOR WT-DAY/WT-UPDATE §4.1: `git switch main` exempt by name; local main here is b6effde, behind origin/main by 101, with no guard.mjs (git branch -vv; git ls-tree). Fix: exempt a target only when protectedHit(origin/main vs target, two-dot) is empty (the WT-NIGHT check, both sessions); test.
C-3 MAJOR §2/§4.1: scripts are the sanctioned multi-site edit route (CLAUDE.md:190, START_HERE.md:75-76) and `node .gate/*.mjs|*.js` run with no prompt (settings.json:49-50): kit edits by script get neither ask nor prompt. Fix: day ask for node/python on a script outside main's tracked files (or -e/-c) whose text names a kit path; or a PostToolUse tripwire on protected files' status; at least a decision.
C-4 MAJOR §2/§3: coverage limits never go to Mustafa as a decision, and the list misses git rm/mv (silently removes the guard: day no-decision, guard.mjs:41-42), revert, merge/pull in a worktree, config --file, format-patch -o, archive -o, find -exec/-delete, xargs, install, dd of=, patch, tar/unzip, perl -pi. Fix: day ask for git rm/mv on kit paths (logic at :540-545); decision AF-COVER listing covered/not.
C-5 MAJOR §8/§6 E10/E11: doc claims under AF-MECH/WT-DAY verified at step 7, after steps 3-6 build on them; no pre-merge proof the CLI honours the hook ask. Fix: E10 before decisions go to Mustafa; pre-merge `claude -p` in the worktree (own project dir) per mode, if the nested CLI authenticates.
C-6 MAJOR §4.7/WT-NIGHT: CLAUDE.md Never bullet has no night exception, NIGHT_SHIFT.md:26 new text switches; NIGHT_SHIFT.md:2 "Follow CLAUDE.md in full". (a) is an exception to his literal words; it refuses only when main moved (night can't push kit changes, :554-560), i.e. after every kit merge. Fix: name the exception in the bullet and the decision, or pick (b).
C-7 MAJOR WT-NIGHT §4.1: `git switch cs/x` on a fresh container (only origin/cs/x; DWIM) -> diff vs 'cs/x' fails -> "fetch first" deny on every handoff. Fix: target = local ref, else origin/<name>; test.
C-8 minor WT-DAY: dropped cherry-pick/am/apply/revert/read-tree -u/stash apply|pop in the main folder (reader-askfirst B7); GIT_DIR/GIT_WORK_TREE prefixes stripped by unwrap (:161) make vcwd lie. Fix: add; ask on those prefixes.
C-9 minor dayShellWrites: directory targets (protectedRel('tools')=false, '.'=false): rm -r tools, mv tools, cp -r x/. . miss. Fix: a target that contains a protected path asks.
C-10 minor §4.3: ESC misses JSON \b \f (and \0 \v), QP =2C, %u002C; @ URL creds (critic-esc.mjs); pctDecode not applied to ENV_SECRETS (checkpoint.mjs:110). Fix: \\(?:[bfnrtv0]|x..|u....); env check on decoded; 2 cases. FP Windows `\tsk-` path: accept.
C-11 minor docs: CLAUDE.md:213-217 vs AF-AGENT; CHECKPOINTS.md:84-85 "main checkout's .checkpoints/"; CLAUDE.md:141-143 + guard.mjs:8-9 "keeps the hook settings" vs bullet "loads". Fix: amend all three.
C-12 minor §4.6: "the only switch a routine makes" false (night switches, NIGHT_SHIFT.md:26, :42). Fix: "the PR Advisor".
C-13 minor §4.7/E7: START_HERE.md:22 "cut from an up-to-date main" missing.
C-14 minor §4.6: memory: local may auto-enable Write/Edit (unverified); resolve at plan stage with E10.
C-15 minor §4.1: get_job_logs returns URLs unless return_content (schema) -> logs_url exclusion reason inconsistent; require return_content true for routines or drop the reason.
C-16 minor READ-SCOPE: search_* and gh pr view --repo (gh() :587) still read other repos; reword or close.
C-17 minor item 5: name the "three deferred points" (likely F-1, R-1, R-2, docs-pass-items:123-127); durable home for mustafa-words.md (the report, quoted whole); E9 table must include the Routines rows (STATUS:50).
C-18 minor §5: missing tests (C-1, C-2, C-7, deny-beats-ask in git, perl -pi, sed -i.bak/--in-place=, git rm guard.mjs, gh pr checkout from /w with gh() given vcwd :562); E4 "--outp writes" is a prediction (my run refused).
C-19 nit: define the allowed pull/merge forms (git pull --ff-only origin main).
C-20 nit: CHECKPOINTS §8 worktree resume: `git worktree prune` first (lost scratchpad worktrees, git worktree list).
C-21 nit: ~/.claude/settings*.json (disableAllHooks) outside kitPath.
C-22 nit: Windows: case-insensitive paths and Git Bash /c/ paths miss the ask; add to E11 P4 and limits.
C-23 nit: header says "PR: none yet"; the session's task list names "PR #38 draft": check.
Right: AF-MECH (b); MCP file writes already denied (settings.json:256-258); sh -c/bash -c/eval denied (:242-245); checkout/restore/reset --hard refused (:202-205, :502-508); --output gap real; ACTIONS_READ = schema enums; exclusions; secret fix A covers %20 %2C %3A; NODE-ALLOW (a); QUEUE-HOME (a); routine paths untouched with A26; F1-FIT with E8.
DONE
