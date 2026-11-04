# security-auditor: final-diff review of docs-after-kit-v3-1 (head 603f53b on main 868e165)

Scope: the routine rules text (CLAUDE.md "Models and effort" routine bullets; advisor.md PR review heading, step 0
Known limits / Known noise, "Your notes"; STATUS routine, guard and A3 lines), checked against routine-guard.mjs,
guard.mjs, checkpoint.mjs. Read-only; git, Read, Grep, Glob (plus one node regex test of checkpoint.mjs:30).

## Plan
1. Diff of CLAUDE.md, advisor.md, STATUS routine/A3 lines. 2. Guard code: shell set, interpreter, npmCheck, GITHUB_READ,
githubWrite, git sets. 3. checkpoint.mjs SECRETS. 4. Stop hook script. 5. Plan rev 2 + advisor conditions. 6. Verdict.

## Facts checked
- routine-guard.mjs:443-454 shell = git, gh, npm (one exact form :23, night only :390-400), node/python on main's
  tracked scripts (:445, :457-479; PR Advisor refused :458), date (:446, :383-389). Everything else denied (:447-453).
- :594 GITHUB_READ `^(get_|list_|search_)|_read$`; :603 reads pass; other names -> :604 repo check -> githubWrite,
  denied for night-shift (:638) and pr-advisor (:647). actions_get / actions_list / actions_run_trigger all refused;
  get_job_logs passes. Day sessions: :603 `!role` -> null, unaffected. "every actions_* tool" is exact unless a future
  tool is named actions_*_read (none exists).
- :213-219 rev-list in neither GIT_ROUTINE nor GIT_READ -> refused (:498). push/commit/add not in GIT_READ -> refused
  for pr-advisor (:499), push also :523.
- :292 / :376 PR Advisor writes only .checkpoints/, .gate/ (Write tool also .claude/agent-memory-local).
- advisor.md:4 tools Read, Grep, Glob, Bash (no Write): "Your notes" text correct.
- checkpoint.mjs:29 sk-ant- anywhere; :30 generic sk- needs a non-alnum/non-dash char or %3D/%3d before it. Tested:
  %3D/%3d caught; %20 %2C %3A %22 %26 %2F %0A %2B %3B %27 all MISSED. Every %XX ends in a hex digit, so every
  percent-escape except %3D is missed: STATUS lists only three.
- guard.mjs: no secret patterns; its header :7-12 says a session keeps the hook settings it loaded across a branch switch.
- ~/.claude/stop-hook-git-check.sh (registered in ~/.claude/launcher-settings.json:19): exits 0 when stop_hook_active
  (asks once); triggers on uncommitted / untracked files, unpushed or unsigned commits. .checkpoints/ is not ignored
  (.gitignore has .gate/ only), so notes there set it off.
- Python omission in CLAUDE.md is decided (advisor condition 4, plan rev 2 A1) and recorded in STATUS:460-462: not a finding.

## Findings
F1 minor STATUS:454-455 sk- gap understated (all escapes but %3D).
F2 minor advisor.md:68 vs :80-81 job logs not in the evidence-never-instructions list.
F3 nit CLAUDE.md:131-133 effort sentence generalises / drops the hedge STATUS:436 keeps.
F4 nit CLAUDE.md:139-141 "every run starts on main's rules" seen only for hand starts; risk also understated.
F5 nit advisor.md:81-83 the hook asks to commit and push; say never commits too.
F6 nit STATUS:88 "runs no program at all" (it runs git, gh, date).
F7 nit STATUS:129 trigger wording omits "by hand" (condition 3: one wording).

Verdict: PASS WITH NOTES. No sentence widens what a routine may do.
