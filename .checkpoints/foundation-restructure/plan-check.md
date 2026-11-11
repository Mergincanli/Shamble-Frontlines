# Plan check: foundation 2.1b (plan revision 3, steps 6-11) against today's main 76b4f7a

Written 2026-11-11 by the main session (Opus 5.5) of session_019YNMUWdGJZoQFUuuvvBP1Y, before any edit. Plan: `plan.md`
in this folder (blob d0b00ec, byte-identical to c8f4bb382d80d2248a3aef75f864753428bfac8b). OLD = 44446a5 (the plan's
base and the independent judge); A = 76b4f7a (main with 2.1a, PR #50). Method: every line the plan cites for a 2.1b
edit re-found on main with grep and read; the plan's inventory grep re-run on main and diffed against 44446a5 (per-file
counts); the folder-argument commands (`preview.js .`, `pageshot.js .`, `battery.js .`) re-grepped; the files #49 and
#50 changed diffed against OLD (`git diff --stat 44446a5 76b4f7a`).

## 1. Line numbers of every 2.1b edit (plan vs main)
| Plan target (plan line) | On main | Content as the plan describes? | 2.1b does |
|---|---|---|---|
| CLAUDE.md :19 ("It moves to `legacy/` in phase 1") | :19 | yes | kit line 1 (his chat yes first) |
| CLAUDE.md :41 (Layout, the three engine files) | :43 | yes (+2: #49's lines above) | kit line 2 |
| CLAUDE.md :42 (Layout, the two pages) | :44 | yes (+2) | kit line 3 |
| CLAUDE.md :46 (hard constraint, "double-clicking index.html") | :48 | yes (+2) | untouched (SCOPE) |
| CLAUDE.md :214-219 (`<dir>` commands) | :239-242 | yes, `<dir>` placeholders (NAME) | none |
| `.claude/agents/verifier.md:28` (`preview.js . eval tools/sim.js`) | :28 | yes | kit line 4 |
| `tools/README.md` :21 ("outside the repo") | :21 | yes (still "outside the repo"; already stale since 2.1a's smoke says "outside the build folder") | edit |
| `tools/README.md` :22-28, :56, :58 (`.` folder arguments) | same | yes | `.` → `legacy` |
| `tools/README.md` :35 (gate-globals wording) | :35 | done in 2.1a (dd12aeb) | none |
| `tools/village.py` :2 (doc line) | :2 | yes (2.1a changed :10-11 only) | edit |
| header comments: preview.js:2, pageshot.js:8, sim.js:1, reach.js:1, perfbots.js:2, tuning-dump.js:3, aabbs.js:1 | same | yes | `.` → `legacy` |
| `README.md` :12, :13, :19-22 | same | yes | `legacy/…` |
| `docs/REPO_MAP.md` :14-18 (five root rows), :6 (date) | same | yes | one `legacy/` row; date |
| `docs/START_HERE.md` :12 | :12 | yes | names `legacy/index.html` |
| `docs/START_HERE.md` :21-22 ("sit at the repo root") | :22-23 | yes (+1: #49 split :20 in two) | "in `legacy/`" |
| `docs/START_HERE.md` :104, :114-116 (`.` commands) | :105, :115-117 | yes (+1) | `.` → `legacy` |
| `docs/MODULE_MAP.md` (one line above the file table) | table at :17-23 | yes | one line |
| `docs/TECHNICAL.md` §9 :238-239 | :238-239 | yes (#49 changed :240 only) | `legacy/` |
| `docs/STATUS.md` :987 risk ("line numbers go stale") | split by STATUS-DIET: `docs/STATUS.md:106` and `docs/STATUS_ARCHIVE.md:843` | yes; the plan foresaw the split (Part 4: "found again in the split files") | corrected in both |
| `docs/ROADMAP.md` (2.1's entry; banner :3 optional) | :3 | yes | entry; banner optional |
| ADR 0002 OQ-0002-2 entry (0002:998-1007) | :998-1007 | yes (2.1a's +4 shift starts after :1425) | the answer |
| ADR 0002 Status line | :3 ("**Status:** Proposed") | yes | his SCOPE (a), his go |
| the register: "the register's OQ-0002-2 row" (plan 1.4 and step 10) | **no such row** | **no: D1 below** | see D1 |
| plan rationale citations in advisor.md (:63-64, :90-91, :149-150, :153-156, :271-274, :285-290, :313) | moved by #46 (e.g. Revert at :269-300, the Never line at :323) | same content (Revert step 2: "ask Mustafa before `git revert` runs ... The main session runs `git revert`, never a subagent") | none (citations only) |
| settings.json:74 (`ln -s ../../../tools/node_modules .gate/*/tools/node_modules`) | :74 | yes | none (E2's deeper link prompts once) |
| editor.html:1058, :1248 (1.8: Export, no import) | same (game files unchanged) | yes | none |

## 2. New legacy-path hits since OLD
The plan's grep at 44446a5: 63 files, 728 hits. On main: 64 files, 747 hits. Every changed count, read in context:
- `docs/STATUS.md` 7 → 0 and `docs/STATUS_ARCHIVE.md` 0 → 10: STATUS-DIET's split. The archive's ten are merged rows,
  follow-ups and old ADR lines (HISTORY / NAME); none is an edit.
- `docs/reports/2026-11-11-foundation-2-1a.md` 0 → 10: HISTORY.
- `docs/ROADMAP.md` 1 → 2: :3 the banner (NAME, optional edit, as the plan says) and :110 2.1a's entry (HISTORY).
- `tools/smoke.mjs` 4 → 7, `tools/village.py` 2 → 3: 2.1a's own edits (gate file: untouched; village.py :2 is planned).
- `.claude/hooks/routine-guard.test.mjs` 15 → 16: #46's row "a game file" with `prFiles: 'shamble-game.js'` (TESTDATA;
  kit file; untouched).
- `docs/DECISIONS.md` holds no legacy file name (0 hits).
Folder-argument commands: the same set as at OLD, only shifted (START_HERE +1, CLAUDE.md :215-218 → :239-242, the
guard test :207 → :219, STATUS :987 → the split files). Nothing new needs an edit.

## 3. Differences beyond a line number
- **D1 (the only one): the register has no OQ-0002-2 row**, at OLD or on main (`git grep OQ-0002-2` finds it only in
  ADR 0002). The register lists only questions routed to Mustafa or the Advisor ("Open questions decided by a spike
  alone, or by an agent, are in the drafts and in SPIKE_PLAN, not here", `docs/adr/README.md:41-43`), and OQ-0002-2's
  decider is "the phase 1 restructure plan (the main session with docs-keeper)" (0002:1003). So the plan's words are a
  slip present since OLD, not drift from #49 or #50. The session stopped on it (his rule: "If the plan and today's main
  disagree on more than a line number, stop and tell me in three lines.") and asked him. **His answer, 2026-11-11 (his
  message in this session, word for word):** "D1: (a). Record OQ-0002-2's answer as your plan check's section 3
  proposes: the register's ADR 0002 row gets the new open-question count and my SCOPE (a) in its status column,
  PM-0002-36's reading goes into the register's loose-ends tables as one row, and ADR 0002's two lines that say "open:
  OQ-0002-2" read "answered". No new register row, and nothing wider." So, exactly: the register's ADR 0002 row (:17):
  "8 open" → "7 open", with OQ-0002-2 named as answered by the restructure plan, and his SCOPE (a) in its status column;
  PM-0002-36's reading (Q-A1) as one row of the register's loose-ends tables; ADR 0002 :1596 and :1626, "open:
  OQ-0002-2" → "answered: OQ-0002-2". Plus what the plan already names in ADR 0002 itself: the OQ-0002-2 entry's answer
  (:998-1007) and the Status line (his SCOPE (a)). No other register edit.

## 3b. The open pull requests' files (plan Part 4: re-read before each build step; read 2026-11-11 21:45 Amsterdam)
- **#40** (netcode proof test, `claude/netcode-proof-test-plan-hcky5g` at 8c681cf, merge-base bb2d817; 600 files):
  outside `spikes/` and `.checkpoints/` it changes `docs/DECISIONS.md`, `docs/REPO_MAP.md`, `docs/STATUS.md`,
  `docs/STATUS_ARCHIVE.md`, `docs/design/netcode-proof-messages.md` (new) and `docs/design/ws-reconnect.md`. 2.1b's
  docs step edits the first four too. The plan routes this case itself (Part 4, #40, items 1, 2 and 6: either order;
  whichever merges second merges main and keeps both sides; #40's `check.mjs` reads none of 2.1b's edits). Part 4's
  general line "a file another open PR edits waits until that PR merges" was written for #46 and his queue (the guard,
  CLAUDE.md); read here as not stopping 2.1b's docs step for #40, since the same plan sets #40's route. For the
  advisor agent to confirm.
- **#48** (the toolchain, `claude/laughing-pasteur-fwya24` at d8186fd, merge-base 2d22b69; 54 files): the root pins,
  `crates/`, `packages/`, `.github/workflows/toolchain.yml`, `.gitignore`. No file in common with 2.1b. It waits for
  2.1b's merge (his order).
- No 2.1b edit touches `spikes/` (#40's `referenceProblems`) or a PM row of ADR 0001, 0004 or 0006 (#40's marks).
- Foreseen by the plan, not differences: the STATUS split (Part 4); 2.1a's done parts (tools/README.md :35, village.py
  :10-11); the advisor.md citations (same content). New since the plan, from his task message: ADR 0002's Status line
  (SCOPE (a)), his render-budget note in DECISIONS.md "Queued" (no other file), the four kit lines asked in one
  message. From his D1 message: his Windows run of the new gate on main 76b4f7a (Windows PowerShell 5.1, Node
  v24.17.0, base 44446a5, "files at the root" twice, every check PASS, smoke 8 of 8 and 5 of 5, GATE PASS) goes into the
  report; his finding for 2.3 (the pinned Playwright's installer hangs after the download reaches 100% under Node
  24.16.0 and later, Playwright before 1.60.0; finished under Node 24.15.0) becomes a 2.3 follow-up; if this session's
  own browser install stops after 100%, it is recorded as that bug and told to him, not waited on.
- His words of this session (the task message and the D1 message) are captured by script from the session's events
  (`list_events`, byte for byte, as 2.1a's `capture-message.mjs` did) into this folder, then into the report's Appendix
  A; DECISIONS.md gets his render-budget note under "Queued" word for word, and one entry per message at the top of
  "Decisions, newest first", quoted whole, copied by script from the appendix and checked byte for byte (2.1a's A3.3 to
  A3.5 precedent). The note's text goes in no other file (the report's appendix records his whole message).

## 4. The follow-ups from 2.1a, folded in
- **IE-2** (all five in one pushed commit): step 6 is one commit holding only the five renames, pushed as the head of
  its own push (so the push run's base `BEFORE` is the root-layout tip 5c35c29 and runs the move check); the draft PR
  opens on it before any `[skip ci]` save, so the pull-request run also tests the move commit alone. Proof: the push and
  pull-request runs' gate headers read back.
- **IE-3 and SA-L1** (tools/README.md lacks the build-folder rule and the letter-case note): step 7 adds one short
  paragraph to tools/README.md: the legacy build lives in `legacy/`; the rule (`findBuild`: `legacy/` when it holds any
  `shamble-*.js`, else the root; an engine file in both places exits 2); a base is read from the first of `legacy/`, the
  root and `shamble-frontlines-repo/` holding all five; on a folder move the old folder must keep no page or engine file
  in any letter case (exit 2); letter case: Linux (CI) treats `Legacy/` or `legacy/SHAMBLE-core.js` as other names and
  judges the root, Windows and macOS open them as the same files and exit 2 ("two places"); both safe. Not a gate file.
- **IE-4** (the pinned Chromium 1194 never ran `legacy/`): the proof is CI's smoke job on the move commit and on the
  final head (run numbers in the report). Locally, first the settings' own allowed command
  `node tools/node_modules/playwright/cli.js install chromium` (settings.json:12) for the pinned build; if the network
  refuses it, the 2.1a alias (1243 named 1194), recorded as a refusal and a caveat.
- **IE-7** (merge on a current pull-request run): before asking his go, the PR's pull-request run must be on the
  current main (if main moved, main is brought in by a merge in a worktree under `.gate/`, pushed, CI re-run), and the
  report quotes that run's gate header: "(files at the root) | head = working tree (files in legacy/)".

## 5. The exact kit lines (asked in one chat message; written only after his yes)
1. CLAUDE.md:19: "… frozen: critical fixes only. It moves to `legacy/` in phase 1." → "… frozen: critical fixes only.
   It lives in `legacy/`."
2. CLAUDE.md:43: "`shamble-core.js` (engine) ← …" → "`legacy/`: `shamble-core.js` (engine) ← …" (rest unchanged)
3. CLAUDE.md:44: "`index.html` (player build) · `editor.html` (authoring build) · …" → "`legacy/index.html` (player
   build) · `legacy/editor.html` (authoring build) · …" (rest unchanged)
4. `.claude/agents/verifier.md:28`: "`node tools/preview.js . eval tools/sim.js`" → "`node tools/preview.js legacy eval
   tools/sim.js`"
Plus E8's one ask (plan 1.5; advisor.md, Revert, step 2): in a throwaway worktree of A under `.gate/`, `git merge --no-ff`
of 2.1b's head then `git revert -m 1 --no-edit` of that merge; both write CLAUDE.md and verifier.md there, and the guard
does not read `git revert`. Run by the main session only; nothing leaves the worktree.

## 6. Evidence in this container (plan 1.5, adapted)
- Toolkit: `npm ci --prefix tools --ignore-scripts --no-audit --no-fund`; the browser as in IE-4; `xvfb-run -a` for
  preview.js (present); Node 22.23.4.
- **E1:** `git diff --find-renames=100% --name-status 76b4f7a <move>`: exactly five `R100`; for each file
  `git rev-parse 76b4f7a:<f>` = `git rev-parse <move>:legacy/<f>` = `git rev-parse 44446a5:<f>`.
- **E2:** OLD's five gate files (`git show 44446a5:tools/<x>`) written into `legacy/tools/` of a throwaway copy of the
  head under `.gate/` (a worktree; `protectedRel` is false for `legacy/tools/…`); `node_modules` linked;
  `node <copy>/legacy/tools/gate.mjs 44446a5` must PASS, symbols +0 −0, smoke both pages.
- **E3:** two worktrees, OLD (its own gate, root) and the move commit (the head's gate, `legacy/`), the 31 entries of
  `e3-mutations.json` (copied from c8f4bb3) planted in both by a copy of `e3-run-f1.mjs`, failing check ids and exit
  codes compared per entry; then compared with the recorded 2026-11-11 run (`e3-results-f1.json`). Plus the head's gate
  against three bases: OLD (root), 8e58937 (`shamble-frontlines-repo/`) and its own head (`legacy/`).
- **E4:** the tuning dump (`--pre=tools/tuning-dump.js`, both maps) at A with folder `.` and at the head with folder
  `legacy`, twice each, A's `preview.js` and `tuning-dump.js` for all runs; head runs byte-identical to A's; then
  `build-tuning.py` at a715be2 (`.checkpoints/r7-legacy-inventory/build-tuning.py`) on the head dumps: `cmp`-equal to
  `docs/legacy-tuning.json` and `docs/LEGACY_TUNING.md`.
- **E5:** headless Chromium (the container's Playwright), one persistent profile, `file://` pages: `index.html` writes a
  KVStore key (`KVStore.backend === 'localStorage'` asserted); relaunch; read back (control); then `editor.html`,
  `legacy/index.html`, `legacy/editor.html` and an unrelated folder read it. Result: shared, per folder or per file.
- **E6:** full gate with smoke on the head; every changed command run once as written (`preview.js legacy shot`,
  `preview.js legacy eval tools/sim.js`, `pageshot.js legacy index.html`, `village.py` with no arguments after the
  aabbs export, `battery.js legacy`); `repo-map-check.mjs` exit 0; the kit tests with main's counts.
- **E8:** after his yes, as in 5; on the reverted tree `node tools/gate.mjs <M>` and `node tools/gate.mjs --smoke-only`
  must both PASS.
- C7: every refusal recorded (checkpoint "Refusals"), never worked around silently.

## 7. Order, docs and reviews
- Steps, in the plan's order: after the advisor agent's APPROVE, the four kit lines and E8's ask go to him in one
  message, before the build (his D1 message: "Then continue with your next steps: the advisor agent's review of the plan
  check, then the four kit lines and E8's ask to me in one message."). Then 6 the move (alone, pushed, the draft PR
  opened on it) → 7 the commands in text → 8 the kit lines (his yes) → 9 E1-E6 and E8 (E3 in the background) → 10 the
  docs (docs-keeper; xhigh for the ADR and register edits) → 11 the reviews on the final diff (security-auditor,
  test-engineer, infra-engineer; 3 at a time), the verifier until PASS, the advisor agent on every pushed head, the
  report, the checkpoint deleted, ready, stop for his go. Only if his yes is late does step 8 wait while 9 and 10 run
  (an order change in time only: the gate reads neither kit file; E8 always runs after step 8).
- The docs (step 10, today's lines): REPO_MAP :14-18 and :6; START_HERE :12, :22-23, :105, :115-117; MODULE_MAP one line
  above the table (:17); TECHNICAL :238-239; STATUS (2.1b's row, #50 merged, the risk at :106) and STATUS_ARCHIVE (#50's
  row; the risk at :843); ROADMAP (2.1b's entry); ADR 0002 (Status line: his SCOPE (a) word for word; the OQ-0002-2
  answer; :1596, :1626); the register (D1); DECISIONS.md "Queued": his render-budget note word for word, no other file.
- Not touched: gate files, settings, hooks, allow-lists, `spikes/`, `docs/legacy-tuning.json`, `docs/LEGACY_TUNING.md`,
  any PM row, reports, research files, the inventories.
