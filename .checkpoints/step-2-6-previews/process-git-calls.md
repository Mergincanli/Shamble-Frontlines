# Git and gh calls from 2026-11-25T16:41:51 to 2026-11-26T00:10:00 UTC that did not start their own call (this chat only, not its agents; git run inside scripts not counted)

Breaches: 53. A, something other than git or gh starts the call (cd, a variable assignment, another program): 31. B, git starts the call but a later git or gh command in the same call follows other commands: 22. Not counted, C, git alone first with other commands after it: 3.

- A: cd starts the call: 9
- A: python3 starts the call: 5
- B: git starts the call; a later git follows head: 5
- A: cat starts the call: 4
- B: git starts the call; a later git follows cut: 4
- B: git starts the call; a later git follows tail: 3
- B: git starts the call; a later git follows echo: 3
- B: git starts the call; a later git follows another git: 3
- A: node starts the call: 3
- B: git starts the call; a later git follows grep: 2
- A: ls starts the call: 2
- A: a variable assignment starts the call: 2
- A: sed starts the call: 1
- A: grep starts the call: 1
- B: git starts the call; a later git follows python3: 1
- A: echo starts the call: 1
- A: ln starts the call: 1
- A: rm starts the call: 1
- B: git starts the call; a later git follows node: 1
- A: timeout starts the call: 1

| Time (UTC) | Shape | The call's first line (cut at 100 characters) |
|---|---|---|
| 2026-11-25 16:42:53 | A: cd starts the call | `cd /home/user/Shamble-Frontlines && git fetch origin main 2>&1 / tail -1 && git log -1 --format='%H ` |
| 2026-11-25 16:43:02 | A: cd starts the call | `cd /home/user/Shamble-Frontlines/.gate/merge-69 && git merge --no-ff --no-commit origin/main 2>&1 / ` |
| 2026-11-25 16:43:33 | A: sed starts the call | `sed -n 114p docs/STATUS.md; echo ====; sed -n 195p docs/ROADMAP.md; echo ====; sed -n 255,256p docs/` |
| 2026-11-25 16:45:16 | A: cat starts the call | `cat > /tmp/claude-0/-home-user/a7e7f150-3195-52bd-a549-73b27426198b/scratchpad/m68/msg69.txt <<'EOF'` |
| 2026-11-25 16:45:58 | A: cd starts the call | `cd /home/user/Shamble-Frontlines && grep -n "guard" .claude/settings.json / head -8 / cut -c1-200; g` |
| 2026-11-25 17:13:12 | A: cd starts the call | `cd /home/user/Shamble-Frontlines && git fetch origin main 2>&1 / tail -1 && git merge --ff-only orig` |
| 2026-11-25 18:29:48 | A: python3 starts the call | `python3 - <<'EOF'` |
| 2026-11-25 19:45:55 | A: ls starts the call | `ls -la .gate/preview-hosting-check/old-tools/tools/ .gate/preview-hosting-check/old-tools/site 2>/de` |
| 2026-11-25 20:21:00 | A: python3 starts the call | `python3 - <<'EOF'` |
| 2026-11-25 20:21:25 | A: a variable assignment starts the call | `D=/tmp/claude-0/-home-user/a7e7f150-3195-52bd-a549-73b27426198b/scratchpad/npm26/lock && cp "$D/pack` |
| 2026-11-25 20:23:49 | A: python3 starts the call | `python3 /tmp/claude-0/-home-user/a7e7f150-3195-52bd-a549-73b27426198b/scratchpad/dec/add_three.py --` |
| 2026-11-25 20:23:59 | A: grep starts the call | `grep -n "2fc39fb\/KEPT-LOGS\/120 lines" docs/reports/2026-11-25-adr0006-header-row.md / cut -c1-260;` |
| 2026-11-25 20:24:48 | A: node starts the call | `node .claude/scripts/repo-map-check.mjs 2>&1 / tail -1 && git add docs/DECISIONS.md docs/reports/202` |
| 2026-11-25 20:25:05 | A: ls starts the call | `ls docs/runbooks 2>/dev/null; git ls-tree -r --name-only origin/main / grep -i runbook / head` |
| 2026-11-25 20:35:03 | A: cd starts the call | `cd /home/user/Shamble-Frontlines && sha256sum hosting/package.json hosting/package-lock.json && git ` |
| 2026-11-25 20:35:17 | A: node starts the call | `node .claude/scripts/repo-map-check.mjs 2>&1 / tail -1 && git add hosting/preview.test.mjs docs/runb` |
| 2026-11-25 20:35:32 | A: python3 starts the call | `python3 - <<'EOF'` |
| 2026-11-25 20:37:48 | A: node starts the call | `node .claude/scripts/checkpoint.mjs save step-2-6-previews -m "The question file rewritten with the ` |
| 2026-11-25 20:41:06 | A: python3 starts the call | `python3 - <<'EOF'` |
| 2026-11-25 20:46:28 | A: cat starts the call | `cat >> .checkpoints/step-2-6-previews/build-notes.md <<'EOF'` |
| 2026-11-25 20:56:56 | A: echo starts the call | `echo "== main's side since the merge base:"; git diff --name-only HEAD...origin/main; echo "== this ` |
| 2026-11-25 20:58:09 | A: cd starts the call | `cd /home/user/Shamble-Frontlines/.gate/merge-71 && git merge --no-ff --no-commit origin/main 2>&1 / ` |
| 2026-11-25 20:59:40 | A: ln starts the call | `ln -s /home/user/Shamble-Frontlines/tools/node_modules tools/node_modules && timeout 600 node tools/` |
| 2026-11-25 21:00:05 | A: cd starts the call | `cd /home/user/Shamble-Frontlines && git status --short --branch / head -2 && git merge --ff-only ori` |
| 2026-11-25 22:58:58 | A: cat starts the call | `cat >> .checkpoints/step-2-6-previews/build-notes.md <<'EOF'` |
| 2026-11-25 23:01:46 | A: cd starts the call | `cd /home/user/Shamble-Frontlines && python3 - <<'EOF'` |
| 2026-11-25 23:18:25 | A: rm starts the call | `rm -rf .gate/mut-suite && timeout 600 node tools/gate.mjs --no-smoke 2>&1 / tail -1 && node .claude/` |
| 2026-11-25 23:35:47 | A: cat starts the call | `cat >> .checkpoints/step-2-6-previews/build-notes.md <<'EOF'` |
| 2026-11-25 23:36:51 | A: timeout starts the call | `timeout 600 node tools/gate.mjs --no-smoke 2>&1 / tail -1 && node .claude/scripts/repo-map-check.mjs` |
| 2026-11-25 23:53:45 | A: cd starts the call | `cd /home/user/Shamble-Frontlines && git status --short / head -20; git log --oneline -5; grep -n "fu` |
| 2026-11-26 00:09:57 | A: a variable assignment starts the call | `R=/home/user/Shamble-Frontlines; T=$R/.gate/step26-r2-test/te; P=$R/.gate/step26-r2-probe; grep -c "` |
| 2026-11-25 16:44:02 | B: git starts the call; a later git follows grep | `git show origin/main:docs/ROADMAP.md / grep -n "^\*\*The foundation, 2.3: the first docs pass\/^- \*` |
| 2026-11-25 16:45:01 | B: git starts the call; a later git follows tail | `git diff --stat origin/main -- .claude CLAUDE.md .github tools / tail -1; git diff --name-only origi` |
| 2026-11-25 16:45:25 | B: git starts the call; a later git follows echo | `git merge-base --is-ancestor 216fb21ede873d75666c2850b49d8f58f3db12fb HEAD && echo ff-ok && git push` |
| 2026-11-25 16:52:09 | B: git starts the call; a later git follows cut | `git ls-remote origin refs/heads/claude/optimistic-albattani-mdoonz / cut -c1-40 && git worktree remo` |
| 2026-11-25 17:13:18 | B: git starts the call; a later git follows tail | `git diff --stat origin/main -- .claude / tail -1; git diff --quiet origin/main -- .claude/hooks && e` |
| 2026-11-25 19:43:38 | B: git starts the call; a later git follows head | `git status --short --branch / head -20; git log --oneline -5; date -u +%H:%M:%S; tail -5 /tmp/claude` |
| 2026-11-25 20:20:51 | B: git starts the call; a later git follows another git | `git status --short && git add hosting/preview.mjs hosting/preview.test.mjs hosting/_headers docs/REP` |
| 2026-11-25 20:23:07 | B: git starts the call; a later git follows cut | `git log --oneline -1 216fb21e 2>&1 / cut -c1-150; git log --oneline -1 35adfbeb04d0326aa69a06356ff12` |
| 2026-11-25 20:24:20 | B: git starts the call; a later git follows cut | `git log --oneline -3 -S'**Kit: KEPT-LOGS (2026-11-15):**' origin/main -- docs/ROADMAP.md / cut -c1-1` |
| 2026-11-25 20:24:28 | B: git starts the call; a later git follows echo | `git merge-base --is-ancestor 08bb20af 311c0484 && echo "08bb20a is in 311c0484"; git merge-base --is` |
| 2026-11-25 20:54:10 | B: git starts the call; a later git follows python3 | `git add .github/workflows/preview.yml .github/workflows/preview-close.yml .github/workflows/wall-tes` |
| 2026-11-25 20:55:28 | B: git starts the call; a later git follows another git | `git fetch -q origin main && git log --oneline -5 origin/main / cut -c1-120` |
| 2026-11-25 20:55:38 | B: git starts the call; a later git follows grep | `git show origin/main:.github/workflows/toolchain.yml / grep -n "preview-site\/upload-artifact" / hea` |
| 2026-11-25 20:58:03 | B: git starts the call; a later git follows head | `git status --short / head -3; git rev-parse HEAD; git merge-base HEAD origin/main; git worktree add ` |
| 2026-11-25 20:59:24 | B: git starts the call; a later git follows echo | `git add docs/DECISIONS.md docs/REPO_MAP.md && echo "main-only files differing from main: $(git diff ` |
| 2026-11-25 20:59:35 | B: git starts the call; a later git follows another git | `git commit -q -F - <<'EOF'` |
| 2026-11-25 20:59:50 | B: git starts the call; a later git follows cut | `git ls-remote origin refs/heads/claude/optimistic-albattani-mdoonz / cut -c1-40 && git merge-base --` |
| 2026-11-25 21:00:17 | B: git starts the call; a later git follows tail | `git worktree remove .gate/merge-71 2>&1 / tail -1; git worktree list / head -3; python3 - <<'EOF'` |
| 2026-11-25 23:14:22 | B: git starts the call; a later git follows head | `git status --short --branch / head -8 && node .claude/scripts/checkpoint.mjs save step-2-6-previews ` |
| 2026-11-25 23:19:04 | B: git starts the call; a later git follows node | `git add docs/runbooks/previews.md hosting/preview.mjs hosting/preview.test.mjs && git commit -q -F -` |
| 2026-11-25 23:19:12 | B: git starts the call; a later git follows head | `git status -sb / head -1 && for i in 1 2 3 4; do git push -u origin claude/optimistic-albattani-mdoo` |
| 2026-11-25 23:19:50 | B: git starts the call; a later git follows head | `git status --short --branch / head -6 && node .claude/scripts/checkpoint.mjs save step-2-6-previews ` |
| 2026-11-25 18:05:24 | C: git alone first, other commands after | `git status --short / head && node .claude/scripts/checkpoint.mjs save step-2-6-previews -m "Step 4: ` |
| 2026-11-25 19:06:24 | C: git alone first, other commands after | `git status -sb / head -3; tail -3 /tmp/claude-0/-home-user/a7e7f150-3195-52bd-a549-73b27426198b/task` |
| 2026-11-25 21:33:10 | C: git alone first, other commands after | `git status --short --branch / head -3; date -u +%H:%M:%S; cat /tmp/claude-0/-home-user/a7e7f150-3195` |
