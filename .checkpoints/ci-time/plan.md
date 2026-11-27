# Plan: CI-TIME (CI Builder), revision 1
- His word: `mustafa-2026-11-27T1250-ci-time.md` (CI-TIME (a), the exact text approved in advance: "That one line becomes
  `timeout-minutes: 25`."). Risk tier 2 (a `.github/` file): the advisor agent and security-auditor.
- Branch: local `cs/ci-time` in worktree `.gate/ci5` from main b9aad01; pushed to `claude/keen-tesla-v807cf` (its last
  pull request, #74, is merged and main holds its head e8340eb, so a fast-forward; no force). The main folder is
  fast-forwarded to b9aad01 (his point 3), so the newest guard loads.
- The change, one line: `.github/workflows/toolchain.yml:38` `    timeout-minutes: 15` becomes `    timeout-minutes: 25`.
  Nothing else in the file; the two Playwright `--global-timeout=300000` stops (lines 205 and 212) stay. No comment or
  REPO_MAP row names the 15-minute limit (grep). The diff: `draft-toolchain.diff` (sha256
  cf98135b9a7f7dfba31c2bc06a332ffbec9027a60327c7766a4b585ef21d26bb); the file today sha256
  d017b83b82d412befb1cbb983a93215440e2b0e1608d2c8c749445db53e87fd8, after d1dd16d7eb850a8b27118eab51008b2e8ad97fcded27c468fa1d310f080ff11c
  (YAML parses: the job's timeout 25, 11 steps).
- Write: mode read right before (his rule of 2026-11-23); committed without `[skip ci]`; gate, smoke and toolchain read
  on the push run and on the pull request's run.
- Docs: DECISIONS (his two messages since #74 by script), STATUS (in flight, then merged), a short report
  `docs/reports/2026-11-27-ci-time.md`, REPO_MAP's row for `toolchain.yml` (adds "a 25-minute job limit") and the report's
  row. The notes are deleted in the last commit before his go; the PR text named at the notes' last commit.
- Pull request: opened as a draft by this chat if the GitHub tools allow it (his point 4), else title and text in the
  notes. On his go for the full head id: mark it ready and merge with that id as the expected head; a refusal is told
  to him, nothing worked around.
- Risk: none to security (a longer limit only lets a stuck run use more runner minutes; public repository, so hosted
  minutes are free); the evidence is the next long run passing where 15 would cut it.
- No checkpoint script (it pushes to the local branch's name). git and gh start their own command.
