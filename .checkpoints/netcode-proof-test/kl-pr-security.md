# security-auditor: PR #53 (KEPT-LOGS) review, head a637c7a

## Plan
1. Read Mustafa's words (history.md 2088-2111, 2131-2161), approved texts, PR diff.
2. Threat-model kept-logs.mjs: scope of removal, symlinks, TOCTOU, path traversal; who may run it; env steering git; archive writer/reader; .part handling.
3. Section 11 text vs his words; guard unchanged.
4. Run test suite in worktree; spot-check a planted bug.
5. Findings + verdict.

## Notes (block 1: reading and checks, complete)
- Read history.md 2088-2161, the diff (4 files; no settings.json, hook, .github or gate file: 0 files), script whole, tests whole.
- Tests in worktree, Node v22.23.4: 9 pass, 1 skip (dry run, expected), 0 fail.
- Re-ran planted bugs: M5 (realDir link check removed) fails tests 7 and 8; M3 (routine refusal removed) fails test 6. Bites hold.
- compare-head.log: the 4 files at a637c7a equal approved text + kl-change-v2.diff (compare: PASS).
- Guard: routines cannot prefix SHAMBLE_ROLE= (ROLE_ENV_OK only, routine-guard.mjs:469-480), cannot use env -u
  (CODE_RUNNERS); day-session lines naming GIT_DIR/WORK_TREE/COMMON_DIR/INDEX_FILE ask (routine-guard.mjs:693-698,772);
  GIT_CONFIG_* not covered. kept-logs.mjs is not on settings.json's allow list (so a day session's run prompts or goes
  to the classifier); Bash(rm -rf *) deny at settings.json:247 unchanged.
- Scratch proof (.gate/kept-logs-rule/review/rm-link.mjs): Node 22 rmSync recursive unlinks a link at the path and a
  link inside, never follows them; it DOES follow a link in a parent component.
- PR body: ToolSearch / GitHub MCP tools not available to this agent: skipped.

## Findings (block 2, complete)
- SA-KL-1 Low: parent-component TOCTOU. realDir checked at checkSet (kept-logs.mjs:132) and cache (289); rmSync at
  273 / 303 after minutes of packing. A link swapped into .gate, .gate/netcode-proof-test or runs between them makes
  rmSync remove <target>/<set> (scratch case 3). Needs a concurrent same-user process. Fix: re-check
  realpathSync(dir) === dir and lstat isDirectory right before each rmSync (follow-up).
- SA-KL-2 Low: git() inherits the whole environment (kept-logs.mjs:49-55); GIT_CONFIG_COUNT/KEY_n/VALUE_n
  (core.worktree etc.) is not caught by the day guard's ask; the "committed and pushed" checks could read another
  repository. Archive read-back still checks against the on-disk list, so no unarchived loss. Tests' sh()/run()
  (test:36-45) also inherit: a test run with GIT_DIR exported writes user.name/email/gpgsign config into that repo.
  Fix: env with every GIT_* removed in the script's git() and the test's sh()/run().
- SA-KL-3 Low: cache identifies the old cache by path + mtime only (kept-logs.mjs:283-299); mtime is settable
  (touch -d), ctime is not. Deliberate misuse only. Fix: also bound ctimeMs, or require cargo's .rustc_info.json;
  text change needs his yes: follow-up.
- SA-KL-4 Low: archive commit step: fsync of the .part file (250-251) but not of the kept-logs folder after
  renameSync (264) before rmSync (273); renameSync also replaces an archive that appeared after the exists check (224).
  Fix: linkSync(part, archive) (EEXIST refuses) or a re-check, then fsync the folder before removing the set.
- Checked, no finding: SET_OK/PATH_OK (no traversal, ASCII, <=99 chars, depth bounded); walk lstat-only; second walk
  throws Mismatch (exit 1); header fields and checksum; readBack loop always consumes or breaks, memory bounded per
  chunk, sha compared so a crafted archive cannot pass; .part opened wx (O_EXCL refuses a planted link), kept on every
  failure; execFileSync no shell, 64 MiB maxBuffer; section 11 is narrower than his words, quotes them verbatim,
  keeps "rm -rf stays denied" (CHECKPOINTS.md:229 and the new paragraph).
- Pre-existing, outside this PR: settings.json:50-51 allow `node .gate/*.mjs|js` without a prompt in a day session,
  so a session-written .gate script could rmSync anything; that is a wider route around "rm -rf stays denied" than
  this PR. Note for the main session / advisor, not a finding on #53.

## Verdict: APPROVE WITH FINDINGS (4 Low, all follow-ups).
