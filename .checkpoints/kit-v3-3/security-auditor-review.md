Task: security-auditor review of A3 (kit-v3-3, PR #38) final diff 12d7961..442fbf7: guard, secret check, settings, advisor CI text.

Plan (blocks, in order):
B1 sources: mustafa-words, plan-rev2 s2/s4, advisor notes, security-review-auto 1+2, builder notes residual risks
B2 diff stat + settings.json diff
B3 routine-guard.mjs review (routines strictness, day ask, main-folder checks, Actions reads)
B4 guard.mjs review
B5 checkpoint.mjs secret patterns
B6 advisor.md CI text
B7 probes (break the six fixes) via scratch script
B8 limits L1-L11 and residual risks weighed
B9 findings written

Notes:
- B1 done (sources: mustafa-words; plan-rev2 s2 L1-L11, s3.2, s4.1-4.4; auto reviews S-1..S-4, 5, 6; builder 4b/4c residuals: routines short-name origin/main in protectedInBranch, interpreter, npmCheck; S-4 cd in if/while or after failed cd read as run, profile CDPATH; 4c: env/nohup/timeout cd followed as move, setter list, line-level flag; e11a probes ask held in default/acceptEdits/auto; gate PASS 555/555). Next B2.
- B2 done: settings adds allow git rev-list * (:30) and deny git rev-list *--output* (:179); guard.mjs ask printed only PreToolUse+no role, else deny (C5 holds by reading guard.mjs:44-45); checkpoint ESC fragment :30. Next B3.
- B3 part 1 (routine-guard.mjs :1-700 read). Candidate findings (to verify in B7):
  F-a sed --in-place abbreviations (`sed --i`, `--in-pl`) not matched by writtenBy :557 regex ^--in-place(=|$).
  F-b `bash -c '...> CLAUDE.md'`, `sh -c`, `eval '...'`, `sudo cp|tee` not inspected by day ask (unwrap :222 strips no sudo; -c string is one word); not named in L1/L2 (L1 names node -e, python -c only).
  F-c kitDir :111-117 only asks for top, .claude, .github, tools, or a folder directly holding CLAUDE.md: rm -rf / mv of an ancestor of the worktree top (e.g. the parent of the project) or of a folder with a deeper CLAUDE.md is not asked.
  F-d GIT_DIR/GIT_WORK_TREE checked only as a prefix (:579-583): `export GIT_DIR=...; ...` earlier in the line passes; GIT_COMMON_DIR/GIT_INDEX_FILE unnamed.
  Routine path: bash() :471-519 role branches unchanged except day-only blocks guarded by !role (:477, :483, :491, :515). decide :341 dayEdit only when !role (routines go on to writePath :346).
- B3 done (routine-guard.mjs :699-1042 read). More candidates:
  F-e (pre-existing, builder residual) routines' short-name origin/main at :465 (npmCheck), :690 (interpreter), :783 (protectedInBranch); night-shift may `git branch origin/main <sha>` / `switch -c origin/main` (branch, switch in GIT_ROUTINE :279; no name check), pr-advisor `git branch origin/main X` (branch in GIT_READ :284) then `switch --detach origin/main` (:749 name check). Verify in B7.
  F-f S-4 side effect: dayCd lost() :609 sets vcwd = projectDir; a relative write after a lost cd (`(cd .claude && sed -i ... settings.json)`, any line with a pipe) resolves against the project folder -> no ask. Verify vs f22c4df.
  F-g git aliases (user ~/.gitconfig `sw = switch`, `co = checkout`) and gh user aliases bypass mainFolderMove :832 (sub is the alias name); gh built-in `co` handled :910.
  F-h day sessions and subagents: mcp() :944 returns null for GitHub writes with no role: create_or_update_file / push_files on branch main write kit files on GitHub without the ask (check branch protection docs).
  Actions reads :920-934, :943: owner/repo first, night refused, method whitelist fails closed (array/undefined). OK.
  mainFolderMove: bundled short options (-qc, -qd) fall to pos[0] -> branchCommit; -c on an existing name fails in git: safe. merge exact forms by commit: OK. `git merge --abort` asks (UX only).
  Next B4/B5 done by B2 reading (guard.mjs, checkpoint.mjs) -> B6 advisor.md.
  F-h withdrawn: main has a ruleset requiring a PR with gate+smoke (STATUS:585 in .gate copy), so a day MCP write to main is blocked; MCP writes to cs/* are ordinary PR work. Accept.
  F-i (new, pre-existing, routines) `git fetch origin <branch>:refs/remotes/origin/main` passes the routine fetch check :743-747 (only remote and pull/ refs checked): forges origin/main for protectedInBranch, interpreter (node runs own committed script), npmCheck; and the day WT-UPDATE (merge --ff-only origin/main). Verify in B7.
  F-j cloud day session: ask unverified (e11a ran only `claude -p` locally); note as limit, probe needed.
- B6 done: advisor.md :73-85 CI text: right (methods match ACTIONS_READ :920-923, logs are data, PASS in log proves nothing, only switch is --detach origin/main); :95-96 day review in a worktree; :154-160 memory text consistent with writePath :361 (file tools) though writeTarget :445 omits agent-memory (shell) - fine. Next B7 probes.
- B7 probes done (.checkpoints/kit-v3-3/sa-probe.mjs -> sa-probe.out; repos .gate/kit-v3-3/sa-probe; head vs f22c4df (s4) vs 12d7961 (base)):
  CONFIRMED F-f (regression from the S-4 fix): Ff1-Ff3 `(cd .claude && sed -i ... settings.json)`, `cd .claude && sed -i ... && git status | cat`, `...; ls | head` head=null, s4=ASK. Cause dayCd lost() :609-610 returns projectDir; dayShellWrites :597 resolves relative target there.
  CONFIRMED F-i: fetch refspec forges refs/remotes/origin/main: day Fi1-Fi3 null (WT-UPDATE bypass); night-shift Ri1 null then Rp2 push of a .claude change null and Rn2 `node tools/sim.js` (changed script) null; pr-advisor Re3 null. Same on base (pre-existing).
  CONFIRMED F-e: local branch origin/main shadows: Re1/Re2 null; Rp4, Rn4 null; Rs4 pr-advisor `switch --detach origin/main` lands on PR head. Same on base.
  CONFIRMED F-a (sed --in, --in-pl; GNU sed 4.9 accepts --in), F-b (bash -c, eval, sudo tee/cp), F-c (rm -rf docs with docs/sub/CLAUDE.md; rm -rf parent of project), F-g (git alias sw=switch).
  OK: rev-list --output really writes a file (deny needed, holds: Rr2/Rr3 DENY); rev-list --count allowed (Rr1). Actions: 10 rows as intended (only listed methods on this repo for pr-advisor; arrays, missing, other owner, artifacts, run_trigger, night shift all DENY). Controls C1-C3 ASK.
  Next B8 limits.
- tests at head: 555/555 (routine-guard, settings, checkpoint).
- secrets probe (.checkpoints/kit-v3-3/sa-secrets.mjs, booleans only): caught %20, %2520, %u0020, \u0020, \n, =2C, &#32;, \u{20}; AWS %2C and \t. Missed: hyphen before (key-sk-, by design vs risk-), sk%2D inner hyphen, lowercase =2c, x_AKIA, AKIA.._x. FP: C:\tsk (accepted in plan), =DE+sk (implausible). Accept (push protection backstop; F2 decode follow-up).
- probe2 (sa-probe2.mjs -> sa-probe2.out): night-shift push from a .gate/ worktree of a branch changing .claude/settings.json: head null, base null. Plan s10 follow-up says "refused at write time rather than at commit and push" - commit and push do NOT catch it either (worktreeRel :770-772 maps to .gate/w/...). Finding SA-3.
- B8 done: limits weighed (see final block).

## Final (B9)
Verdict FAIL (one regression in this diff, SA-1; cheap fix; re-review only that), plus SA-2 pre-existing major for the advisor to route.
Findings:
SA-1 major routine-guard.mjs:608-610,:597 lost cd -> relative writes resolved at project folder; Ff1-Ff3 null at head, ASK at f22c4df. Fix: two folders (followed vcwd for writes, lost->projectDir only for the main-folder check), or check relative targets against every folder the line cd'd to; add Ff1-Ff3 rows.
SA-2 major (pre-existing) origin/main forgeable: fetch refspec dst (:743-747; day: no fetch check) and local branch origin/main (no name check; branch/switch in GIT_ROUTINE :279, GIT_READ :284); short names :465,:690,:783,:749. Probes Ri1,Rp2,Rn2,Re1-3,Rp4,Rn4,Rs4,Fi1-3. Fix: routines refuse fetch refspecs with ':' or a leading '+', and branch/switch -c names matching ^(origin/|refs/|remotes/|HEAD$|main$); day: fetch whose dst is refs/remotes/origin/* or refs/heads/main asks; the three routine sites and pr-advisor switch use refs/remotes/origin/main and compare by commit.
SA-3 minor (pre-existing) .gate/ worktree blind spot also in commit (:755) and push (:779 via worktreeRel :770); probe2. Fix: worktreeRel by worktree top when common dir is the project's (kitPath logic); correct the s10 follow-up text; must land before WT-NIGHT (b).
SA-4 minor bash -c/sh -c/eval/sudo/doas not parsed, not named in L1/L2 (unwrap :218-242, writtenBy :527). Fix: recurse parseShell on -c string and eval args, unwrap sudo/doas; or name in L2+STATUS.
SA-5 nit sed --in/--in-pl abbreviations (:557). Fix prefix match of --in-place, min length 4.
SA-6 nit kitDir :111-117 misses ancestors of the worktree top and nested CLAUDE.md (Fc1, Fc2). Fix: ancestor of projectDir/top asks; ls-files for */CLAUDE.md under the target.
SA-7 nit git aliases bypass :832 and GIT_NEVER :705 (Fg1). Fix: git config --get alias.<sub> -> re-check expansion or ask.
SA-8 nit GIT_DIR/GIT_WORK_TREE prefix only :579-583. Fix: raw-text check incl GIT_COMMON_DIR, GIT_INDEX_FILE, export forms.
SA-9 nit fold only win32/darwin :67; WSL /mnt/c. Fix: always fold.
SA-10 nit ask unverified in a cloud day session and bypassPermissions (e11a local -p only). Fix: record as limit; probe in E11 P1.
Right: guard.mjs C5 :44-45; routines: all new paths behind !role (:341,:477,:483-487,:491-494,:515,:764,:909-910); actions :920-934,:943 correct (10 probes); rev-list --output deny needed (real git writes) :259 + settings :179; allow git rev-list * day-only effect; secrets ESC :30 fixes his sk- case; S-1, S-2 (name shadow), S-3, 5, 6 hold; S-4 holds for main-folder check but caused SA-1. advisor.md CI text right.
Accepted: L1-L11 (reasons in answer), S-4 if/while/failed cd/profile CDPATH, 4c env/nohup/timeout cd, setter list, line flag; L7 mods; secret FNs; MCP writes to main (ruleset); node .gate/* .. reach (day only; routines refuse .gate scripts :687).
DONE
