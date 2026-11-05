Task: advisor ruling on the A3 (kit-v3-3, PR #38) fix round after step 8 reviews of head 442fbf7.

Plan (blocks):
B1 read reviews (security-auditor-review, verifier-review, skeptics, auto-reviews, mustafa-words)
B2 read plan-rev2 §2 limits + §10 follow-ups; prior rulings (skim)
B3 read the dayCd code at 442fbf7 (the cd tracking) to judge SA-1/7/8 fix shape
B4 write the ruling

Notes:
- B1 done: reviews read. SA-1 confirmed by skeptic (7 forked forms head=null, f22c4df=ASK). SA-2 routine half real, day half subsumed by L1. SA-3 .gate/ worktree blind at write+commit+push; STATUS:340 / plan :428 text wrong. SA-4: bash -c/eval exact forms denied by settings.json:243-246; sudo/doas and -c variants real gap. F-a refuted. F-b: git worktree add / clone into kit folders null for main AND agents; worktree add allow-listed settings.json:28. Auto 7,8 open; auto note: prefer fail-toward-ask by construction. Next B2.
- B2 done: plan §2 L1-L11 (L2 enumerated list; L4 patch cmds; L11 other worktrees), WT-NIGHT (a)/(b) text §3.1 (b) needs its own tested change-set; §8 step 9 waits; §10 follow-up "refused at write time rather than at commit and push" (wrong per SA-3). Next B3 dayCd code.
- B3 done (routine-guard.mjs at 442fbf7 :470-640, :180-245, :831): one vcwd serves both checks; lost() = projectDir (:609) fails toward ask for mainFolderMove, away from it for relative writes (:597). dayCd follows bare names unless CDPATH flag (lineSetsCdpath, VAR_SETTERS: a feature list). `||`/`&&` are endCmd, no fork (:192-193): skipped cds (a && cd X; b, if, ||) followed as run. unwrap strips command/builtin/env/timeout/nice/... and VAR= prefixes before the cd test (:220-245): `timeout 5 cd X`, `/usr/bin/cd X`, `HOME=.. cd` followed as moves although bash does not (or does differently). `command -p cd` leaves '-p' as prog.
- finding (mine, A-1): the cd is also followed through program wrappers and path spellings that do not move the shell (env/timeout/nohup/nice/stdbuf/xvfb-run/xargs, /usr/bin/cd) and through VAR= prefixes (HOME=... cd); same class as 7/8 (guard follows a folder bash did not go to).
- design chosen: "unknown means ask" + "follow only what the text fixes" (rules R-a..R-e), not two folders / every folder (both guess). Next B4 write ruling.

## Ruling (B4)

**Verdict: PROCEED with the fix list below.** The plan stands; this is the fix round of step 8. After the fixes: gate,
E1 (with a mutant for each new rule), the automated review on the fix commit, then security-auditor and the verifier on
only the fix diff. A3 still cannot merge before Mustafa answers WT-NIGHT and step 9 is built and reviewed (N-3 below).

What I read: the two reviews, the six skeptic notes, the three automated reviews, Mustafa's words, plan-rev2 §2, §3.1,
§4.1, §7-§10, and routine-guard.mjs at 442fbf7 (:180-245 parser and unwrap, :470-640 bash(), dayShellWrites, dayCd,
lineSetsCdpath, :831 mainFolderMove). What I did not do: run the guard or the gate myself (the reviews' probe outputs,
sa-probe.out and skeptic-sa1-probe.out, are consistent with the code I read).

### The cd tracking (SA-1, automated 7 and 8, and one more of the same kind)
**Neither offered option.** "Two folders" and "every folder the line moved into" both still guess the folder, and 7
and 8 show the guess can be wrong without the guard noticing: bash goes somewhere the guard never followed. Any rule
that resolves a relative write against a guessed folder fails away from the ask when the guess is wrong. One more case
of the same kind (A-1, mine, from reading :220-245 and :490-493): the guard follows a cd that bash does not perform, or
performs differently: `timeout 5 cd X`, `env cd X`, `/usr/bin/cd X` (a program in a child process, the shell does not
move), `HOME=/x cd` (bash goes to /x), and `a && cd X; b` / `true || cd X` / `if false; then cd X; fi` (cd skipped;
the parser joins `&&` and `||` without marking a fork, :192-193).

**The design: the folder is known only where the text fixes it; an unknown folder makes both checks ask.**
- R-a (SA-1). An unknown ("lost") folder is a state, not a folder. The main-folder check keeps treating it as the
  project folder (asks, as today). A relative write target while the folder is unknown asks like a `$` target (an
  agent is refused); absolute targets are checked as today. The reason says how to avoid it: an absolute path, or the
  cd as the first command of the line.
- R-b (7, 8, profile CDPATH, failed cd). A cd, pushd or popd target is followed only when it is absolute or starts with
  `./` or `../` (or is `.` or `..`), and names a folder that exists and can be entered now. Every other target is
  unknown: bare names (CDPATH, cdable_vars, a profile's CDPATH, all by construction), `~` forms and no target (HOME is
  a variable the line can set), `-`, `$`, globs, any option other than `-L` and `--`. Delete lineSetsCdpath, VAR_SETTERS
  and the `io.env.CDPATH` branch: dead under R-b; their tests become "unknown" tests.
- R-c (A-1, skipped cds). A cd is followed only in the line's leading run of cd/pushd/popd (before any other command),
  spelled as the bare word, reached through nothing but shell keywords, `{`, `!`, `time`, `builtin` and `command` (with
  `command`'s `-p`, `-v`, `-V`, `--` stripped in unwrap, the first half of 7's fix). Any later cd, a cd behind a program
  wrapper (env, timeout, nice, nohup, stdbuf, xvfb-run, xargs) or a `VAR=` prefix, or spelled as a path: unknown.
- R-d. A line with a subshell, pipe, background job or substitution: every cd is unknown (today's rule, kept).
- R-e. After a command that can move the folder without the word cd, the folder is unknown: `source`, `.`, `eval`,
  `exec`, and a program word that unwrap leaves starting with `-` or holding `$` or a backtick. (Only shell builtins
  move the shell's folder; this is bash's closed set, not a feature list.)
- Tests: Ff1-Ff3 and the skeptic's 7 forked forms ask; 7's `builtin export CD''PATH=<main> ; cd tools && git switch x`
  and 8's `shopt -s cdable_vars; x=<main>; cd x && git switch y` ask; `cd /nonexistent; git switch x`, `true || cd
  <wt>; git switch x`, `if false; then cd <wt>; fi; git switch x`, `timeout 5 cd <wt> && git switch x`, `command -p cd
  <main> && git switch x` ask (cwd the main folder, or the worktree for the last). Quiet controls stay null:
  `cd <abs wt> && git switch cs/x`, `cd ./.gate/wt && git switch cs/x`, `cd <abs wt> && echo x > notes.md`,
  `cd <abs>/tools && npm test 2>&1 | tail`. One mutant per rule R-a..R-e, each killed.
- Prompt cost: the builder lists every existing case that moves from null to ask, by name, so the cost is visible.
- What stays (new stated limit L15 below): a cd function or alias from the user's shell profile, and a profile that
  sets physical mode (`set -P`) with a symlinked folder.

### Finding by finding
| Finding | Ruling | Shape / exact line |
|---|---|---|
| SA-1 (major, A3 regression) | Fix in A3 now | R-a, with R-b..R-e above |
| Auto 7 (quoted CDPATH behind builtin / command -p) | Fix in A3 now | R-b (bare names unknown) and R-c (command's options stripped) |
| Auto 8 (cdable_vars) | Fix in A3 now | R-b (existing folder, never a bare name) |
| A-1 (mine: cd behind a program wrapper, a path spelling, a VAR= prefix, or skipped) | Fix in A3 now | R-c |
| SA-2 (major, pre-existing; origin/main forgeable) | Fix the routine half in A3 now, as a bug fix | See below. Day half: stated limit L12 and a follow-up |
| SA-3 (minor, pre-existing; `.gate/` worktrees blind at write, commit and push) | Text fix in A3 now; code: follow-up | See below |
| SA-4 (minor; sudo, doas, shell -c variants, eval not parsed) | Stated limit (L1 widened) | See below |
| F-b (minor; `git worktree add` / `git clone` into a kit folder, agents not refused) | Fix in A3 now | writtenBy returns the path of `git worktree add [opts] <path>` (skip -b/-B/--reason values) and `git clone [opts] <url> [<dir>]` (no dir: the URL's last part without .git); kitPath/kitDir then ask, agents refused; a DAY_CASES row each, main and agent |
| SA-5 (nit; sed --in, --in-pl) | Fix in A3 now | any prefix of `--in-place` from `--i` on, with or without `=suffix` (the only sed long option starting with i); rows --i, --in, --in-pl, --in-place=.bak |
| SA-6 (nit; kitDir misses ancestors and nested CLAUDE.md) | Fix in A3 now | by construction: a folder target asks when it is the worktree top or an ancestor of it, or when `git ls-files -z -- <dir>` names any kit path (protectedRel); git failing asks; today's checks stay as the fallback; rows Fc1, Fc2 |
| SA-7 (nit; git aliases) | Stated limit L14 and a follow-up | L14 below; routines unaffected (an alias cannot shadow a built-in git command, and routines run only listed ones) |
| SA-8 (nit; GIT_DIR export forms, GIT_COMMON_DIR, GIT_INDEX_FILE) | Fix in A3 now | a day line whose raw text or any word names GIT_DIR, GIT_WORK_TREE, GIT_COMMON_DIR or GIT_INDEX_FILE asks (replaces the prefix-only test :579-583); rows: export form, `GIT''_DIR=` form, a prefix form |
| SA-9 (nit; case folded only on win32/darwin; WSL /mnt/c) | Stated limit (L9 widened) and a follow-up after PC-SETUP | L9 below; PC-SETUP gains question (4) |
| SA-10 (nit; ask unverified in a cloud day session and in bypassPermissions) | Stated limit L13; E11 P1/P1b plus a bypassPermissions run after merge | L13 below |
| F-a (refuted) | No fix needed | When the checkpoint is deleted (step 10), docs-keeper adds the last commit that holds the notes to the four lines (STATUS.md:92, :142, :565, :678), as UNREAD_SOURCES.md:6 does: four words each |
| F-c (nit; git symbolic-ref HEAD) | Stated limit (L4 widened) | L4 below; it moves HEAD, not files, so no other branch's settings load |
| N-1 (E11a record partial) | Report line | The report says: E11a used Write, not Edit; the `.claude/` target ran only in default mode, so L5's auto-mode classifier path was not shown live; and the default `.claude/rules/probe.md` run also carried the guard's own ask |
| N-2 (C6 naming) | Report line | Beside R1, the report names the two import edits (routine-guard.test.mjs:12, :18), the comment line (:131) and the added fakeIo line (:60-61) every case now sees |
| N-3 (NIGHT_SHIFT.md:27 vs the Never rule) | Confirmed | See below |

**SA-2: in A3 now, routine half.** Why now: it is a real escape in a routine that runs today (the PR Advisor can be
led to `git fetch origin <x>:refs/remotes/origin/main` or `git branch origin/main <x>`, then `git switch --detach
origin/main` lands on that commit and its hook code runs from the next call, Rs4); A3 already touches every site
(protectedHit, ORIGIN_MAIN, isOriginMain); security-auditor re-reviews the fix diff anyway; and the next kit
change-set has no date (the night queue has no APPROVED item). "Fix bugs before features" applies. Shape:
1. Every guard site that means main uses the full ref `refs/remotes/origin/main` and compares by commit: :465
   (npmCheck), :690 (interpreter), :749 (pr-advisor's switch: the target must resolve to origin/main's commit), :783
   (protectedInBranch). This defeats a local branch or a tag named origin/main by construction (rev-parse looks at
   refs/tags and refs/heads before refs/remotes).
2. Routines refuse a `git fetch` or `git pull` refspec holding `:` or starting with `+`.
3. Routines refuse new branch names starting with `origin/`, `refs/`, `remotes/`, or equal to `HEAD` or `main`
   (`git branch`, `switch -c/-C`, `checkout -b/-B`, `worktree add -b/-B`): a second layer after 1.
4. The builder confirms by test that no routine-allowed command writes git config (`remote.origin.fetch`,
   `remote.origin.url`); if one does, refuse it.
5. Rows: the auditor's Ri1, Rp2, Rn2, Re1-Re3, Rp4, Rn4, Rs4 refused or correct; Fi1-Fi3 recorded under L12; a mutant per
   site.
6. The report lists it under "changes to his words" (his A3 list does not name it), with this reason.

**SA-3: text now, code as a follow-up.** Now: correct the three texts that say `.gate/` worktrees are caught at commit
and push (STATUS.md:340, plan §4.8 item 8 at :428, plan §10 at :637-638) to: "Scratchpad worktrees: a routine's kit
write is caught only at commit and push. Worktrees under `.gate/`: a routine's write, commit and push checks all miss
kit files there (follow-up)." Code (worktreeRel by the worktree's own top when its common dir is the project's, the
kitPath logic): first in the next kit change-set, or first in WT-NIGHT (b)'s own change-set if Mustafa picks (b),
whichever comes first; it must land before any night item works in a worktree. Why not now: pre-existing, minor, it
changes routine push and save behaviour that C6 keeps byte for byte, and no night item runs today.

**SA-3 and WT-NIGHT: yes, the question must say so.** It changes (b)'s cost. And SA-2's fix bears on (a): (a)'s check
compares the target with origin/main, which this fix round makes unforgeable. The updated question is under
"Decisions needed". My recommendation stays (a).

**SA-4: a stated limit, not a fix.** `bash -c`, `sh -c`, `zsh -c` and `eval` at the start of a command are already
refused by settings.json:243-246; the cloud container runs as root, so sudo is needless there; and L1 already leaves
`node -e` and `python -c`, far more common. Asking on these runners while `node -e` passes would add little to a
seatbelt.

**N-3: confirmed.** A3 cannot merge before Mustafa answers WT-NIGHT and step 9 is built and reviewed (plan §8 step 9,
condition C2: never ship the Never bullet with an exception he has not accepted, nor beside a NIGHT_SHIFT.md:27 that
still says "Check out that branch"). Until then the PR stays a draft. Nothing conflicts on main today: neither text is
there, and the night queue has no APPROVED item.

### Stated limits (exact lines for STATUS's A3 entry and the report)
Widened:
- **L1, scripts and other runners:** a `node`/`python` script, inline code (`node -e`, `python -c`), a shell given a
  command string (`bash -lc`, `bash -e -c`, `/bin/sh -c`, `dash -c`, `ksh -c`), `eval` after other words, and
  `sudo`/`doas` are not inspected: a kit edit made through them gets no ask. (`bash -c`, `sh -c`, `zsh -c` and `eval`
  at the start of a command are refused by settings.json.)
- **L4, patch-applying commands and HEAD moves:** `git cherry-pick`, `am`, `apply`, `revert`, `read-tree -u`, `stash
  apply|pop`, in any folder, the main folder included; and `git symbolic-ref HEAD <ref>` (moves HEAD, not files).
- **L9, case and Windows spellings:** Windows Git Bash `/c/…` spellings of a path; and, under WSL, Windows drives
  (`/mnt/c/…`) are case-insensitive while the guard folds case only on Windows and macOS.
New:
- **L12, a forged origin/main in a day session:** a day session's `git fetch origin <x>:refs/remotes/origin/main`
  repoints origin/main, so the main-folder checks compare with the wrong commit. (Routines refuse it.)
- **L13, where the ask is untested:** a hook's ask is not yet shown to prompt in a cloud day session or in
  bypassPermissions mode (E11 P1/P1b after merge, plus a bypassPermissions run).
- **L14, git aliases in day sessions:** an alias (`sw = switch`, `co = checkout`, a `!` alias) runs past the
  main-folder checks and the git write checks.
- **L15, the shell profile:** a cd function or alias defined in the user's shell profile, or a profile that sets
  physical mode (`set -P`) with a symlinked folder, can move the folder where the guard believes it knows it.
Removed (closed by R-a..R-e): S-4's accepted cd in if/while and after a failed cd, the profile CDPATH, and step 4c's
env/nohup/timeout cd followed as a move. The docs-keeper takes them out of STATUS.

### Follow-ups (STATUS's follow-ups, with where they go)
- SA-3's code fix: first in the next kit change-set, or first in WT-NIGHT (b)'s change-set, whichever comes first;
  before any night item works in a worktree.
- L12, a day fetch whose destination is `refs/remotes/origin/*` or `refs/heads/main` asks: next kit change-set.
- L14, day git aliases (`git config --get alias.<sub>`: re-check a plain expansion, ask for a `!` one): next kit
  change-set.
- L9 / SA-9, always fold case (or detect it at run time): after PC-SETUP (4) and E11 P4.
- L13, the ask in a cloud day session and in bypassPermissions: E11 after merge.
- L1's runners (sudo, doas, shell -c variants, eval): join the existing "L1-L4, if E11 or use shows a need" follow-up.

### Evidence and reviews for the fix round
Gate PASS (smoke included); all test suites with the new rows; E1's mutation harness extended with a mutant per new
rule (R-a..R-e, SA-2's four sites and two refusals, F-b, SA-5, SA-6, SA-8), each killed; the null-to-ask list (cd);
the automated review on the fix commit; then security-auditor and the verifier on only the fix diff (Opus 5.5, high).
Checkpoint saved after each step.

### Decisions needed
- **WT-NIGHT (updated; still Mustafa's): may the night shift switch its folder to an item's branch?** Your worktree
  rule says never switch a session's main folder, but the night shift's handoff does that today (NIGHT_SHIFT.md:26-27).
  Why it matters: without an answer the night either breaks your rule or loses its handoff, and A3 cannot merge.
  Options: **(a)** an exception: the night may switch only when the branch's rules, CI and gate files equal main's; the
  guard's view of main is now unforgeable (this fix round, SA-2). Cost: after each kit merge, older branches stop until
  a day session updates them. **(b)** no exception: the night works in a separate worktree. Cost: its own change-set,
  which must first close a gap the review found: the night's write, commit and push checks do not see kit files in
  worktrees under `.gate/` (SA-3); until then night handoffs stop. Recommendation: **(a)**.
- **PC-SETUP (updated; information only): how is your Windows PC set up?** (1) your permission mode (Manual, Accept
  edits or Auto); (2) any Claude Code mods installed; (3) a Team or Enterprise login, or a personal one; and new: (4)
  does Claude Code run in Windows itself (PowerShell or Git Bash) or inside WSL (Linux on Windows)? Why (4) matters: under
  WSL the guard does not treat upper and lower case as the same on Windows drives, so a `CLAUDE.MD` spelling could
  slip past the ask. Options: just tell us. Recommendation: if it runs in Windows itself, nothing changes for (4).

### For Mustafa
1. Answer WT-NIGHT (a) or (b).
2. Answer PC-SETUP (1)-(4).
Nothing to merge yet.

DONE
