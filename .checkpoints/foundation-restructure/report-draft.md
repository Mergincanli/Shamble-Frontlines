# Advisor report: foundation-restructure, step 2.1b (2026-11-12; PR #51; branch claude/clever-fermi-oudsfq for cs/foundation-restructure; base main 76b4f7a)

## Goal
Move the legacy game's five files into `legacy/` with their bytes unchanged, so the top of the repository is free for
v2 (ADR 0002 part 1), with every command, kit line and doc pointing at the new place, and the move provably revertible.

## Outcome
DONE: the move, the commands, the four kit lines (on his yes), evidence E1 to E6 and E8 (all PASS) and the docs; the
reviews and the verifier below. The final head, its CI runs (gate and smoke) and E1 on it are in the PR body: written
here they would change the head. Risk tier 2: needs-mustafa.

## Plan and approval
Plan revision 3, steps 6-11 (the old chat's notes at c8f4bb3), checked against main 76b4f7a: line numbers only, except
D1 (the plan named a register row that does not exist), which stopped the build and went to him: D1 (a). The advisor
agent's review of the plan check: APPROVE WITH CONDITIONS, C1-C17, each met: one ask for the kit lines, E8 and REG-TRUE
(C1: (a), (a), (b)); the move alone, pushed with the saver stopped, the draft PR opened on it, both runs read (C2: #674,
#675); E1 on the move commit and on the final head (C3); the README paragraph built from the code (C4); the kit commit
equal to the approved text (C5, 14/14); every changed command run (C6, 14/14); the console lines run on the real pages
and given to security-auditor (C7, 10/10); E8 by the main session in `.gate/fr-e8`, removed without --force (C8); the
browser route (C9: the settings' own install command, no hang under Node 22); his SCOPE (a) quoted whole by script
(C10); OQ-0002-2's answer with three parts, "answered in part" (C11, REG-TRUE (b)); the PM-0002-36 row names the
advisor agent's reading (C12); one DECISIONS.md entry per message, by script (C13); #40 re-read at 063af69, its lines
left alone (C14); the reviews after step 8, docs-keeper at xhigh (C15); the last commit without a skip marker and the
pull-request run on the current main (C16); this report's items (C17).

## What changed
- `legacy/`: the five files moved (`git mv`, five R100 renames), bytes unchanged; game symbols +0 -0 (the gate).
- `tools/README.md` (every `.` folder argument → `legacy`; the build-folder paragraph), the header comments of seven
  tools, `tools/village.py`:2, `README.md`: the commands name `legacy`.
- CLAUDE.md :19, :43, :44 and `.claude/agents/verifier.md:28`: the four kit lines, as approved.
- `docs/`: DECISIONS.md (six entries and the Queued note, by script; his render-budget note also sits inside his whole
  task message, A2, in that entry and in Appendix A: C13); ADR 0002 (Status line by script; :7, OQ-0002-2's answer, the
  two rows); the register (the ADR 0002 row, the status sentence, the Q-A1 row); REPO_MAP, START_HERE, MODULE_MAP,
  TECHNICAL, STATUS, STATUS_ARCHIVE, ROADMAP (docs-keeper); this report.
- Not touched: the gate files, settings, hooks, CI, `spikes/`, the tuning oracle files, every PM row.

## Evidence (notes, scripts and outputs: this branch's last checkpoint commit, named in the PR body)
| Check | Result |
|---|---|
| Gate, move commit, local | vs 76b4f7a and vs 2f4b974: PASS, "(files at the root) \| head = working tree (files in legacy/)", +0 -0, smoke 8/8 and 5/5 |
| CI, move commit c090843 | push #674 (base 2f4b974) and pull request #675 (base origin/main = 76b4f7a): gate PASS with that header; smoke 8/8 and 5/5 on the pinned Chromium |
| E1, the bytes | `git diff --find-renames=100% --name-status c090843^ c090843`: five R100 rows, nothing else; blobs equal at 76b4f7a, 44446a5 and `legacy/` (e528dca, 24e86d5, fce6f37, 008c66f, 5ba72df); final head: in the PR body |
| E2, OLD's gate | 44446a5's untouched gate files run on the moved bytes: GATE PASS, +0 -0, smoke 8/8 and 5/5 |
| E3, the gate bites | 31 of 31: each mutation fails the same checks in OLD's gate (files at the root) and the head's gate (files in `legacy/`); controls pass in both; M19 and M20 differ from the 2026-10-14 report's loose "5b + 5d" wording only (as in 2.1a). The head's gate vs 44446a5, 8e58937 (+34 -0), 76b4f7a, c090843, b6b3b81: PASS |
| E4, the oracle | tuning dumps at A (`.`) and head (`legacy`), both maps, twice: byte-identical (35792 and 37063 characters); R7's builder reproduces `legacy-tuning.json` (78862 bytes) and `LEGACY_TUNING.md` (22018 bytes); the head's edited tools give the same dump (`cmp`) |
| E5, the saves | headless Chromium 1194 on Linux: SHARED (every `file://` page, any folder, one storage); control and reverse hold. Both console lines on the real `legacy/` pages: 10/10. Brave on Windows: not verified |
| E6, the commands | 14 of 14 changed command lines run as written; kit tests 1261/1261; `repo-map-check.mjs` exit 0 after the docs pass (112 entries, 99 paths) and on the final head (PR body) |
| E8, the revert | main + `git merge --no-ff` of the PR head (c33ac99) + `git revert -m 1` (5f91112): tree = 76b4f7a; `node tools/gate.mjs c33ac99` PASS ("(files in legacy/) \| head = working tree (files at the root)"), `--smoke-only` PASS |
| Kit lines | b6b3b81: the four removed and added lines equal the approved text and main's before lines, by script (14/14) |
| Letter case | Linux: `Legacy/` and `legacy/SHAMBLE-*.js` are not counted by `findBuild` (`case-test.mjs`); the move check catches `Index.html` (2.1a's row j). Windows and macOS: by reading the code, not run |
| CI, final head | in the PR body: the push and pull-request runs, gate and smoke |

## Reviews
Round 1 on the final diff (security-auditor, test-engineer, infra-engineer), the verifier and the advisor agent's
review of the pushed head: filled in before the PR is marked ready.

## Decisions and deviations
- His answers: D1 (a); KIT-2.1B (a); E8-RUN (a); REG-TRUE (b): OQ-0002-2 stays open, "answered in part"; 8 open; the
  register's Total line unchanged (Appendix A3, A5).
- E3's head column ran at b6b3b81, not the move commit: the same gate files and the same five blobs.
- E5 gained run 4 (the console lines on the real pages, C7); E8 merged the PR head of the moment (f0322d2).
- The step 10 self-check, not a review round: `docs-check.mjs` (20 checks: the changed files, the ADR and register
  hunks, the acceptance words, quotes, START_HERE's commands, the render note's one home, no notes pointers,
  repo-map-check) and one blind reader of the ADR and register edits against his words: one Medium (the register's ADR
  0002 row did not mark -2 "answered in part"; fixed as REG-CELL (a), below), two Lows (the status sentence's "His
  words:", left as approved, stale list; the ADR's pointer to E2-E4 here, this report).
- OQ-0002-2's answer states its counts exactly: 2 gate files in 2.1a, 2 kit files in 2.1b; 15 `tools/` allow rules
  today (the plan counted 17); the docs counts labelled as the plan's of 2026-11-10.
- The worker restarted between kit edits 2 and 3; `git diff` showed which were made; the commit holds all four.

## Risks
1. Saves in Brave on Windows: E5 is headless Chromium on Linux; his backup (For Mustafa, step 1) comes first.
2. Reverting 2.1a while 2.1b is on main breaks the gate (2.1a reads `legacy/`): revert 2.1b first (plan 1.9).
3. PR #40 shares DECISIONS.md, REPO_MAP.md, STATUS.md and STATUS_ARCHIVE.md: whichever merges second keeps both sides.
4. Windows and macOS letter case: by reading the code only.
5. PM-0002-36's own wording still says the dump reproduces the docs file: fixed at ADR 0002's next revision (Q-A1).

## Proposed next step
Step 2.2, the toolchain (PR #48's lane), then 2.3, the gate step, with the findings for 2.3 below.

## Decisions needed
- **REG-CELL · Is the register's ADR 0002 row right to mark OQ-0002-2 "answered in part"?** Why it matters: your two
  answers read two ways. As built: your D1 (a) took the plan check's section 3 as sent (119b71d), which put "-2
  answered by the restructure plan" in that cell; your REG-TRUE (b) made it "answered in part" and kept the count at 8;
  the register's own rule marks such a question. The cell: "8 open: -8 answered at the stop (2026-11-03); -2 answered in
  part by the restructure plan (2026-11-11)". The other reading: the cell stays as on main, since REG-TRUE (b) names
  only the count. Options: (a) as built; (b) the cell as on main (one line, reverted before the merge).
  Recommendation: (a).
- **PROMPTS-2.3 · Should step 2.3's plan take this session's prompt findings?** Why it matters: you asked twice (A4,
  A6) for the session to accept prompts itself; a session cannot answer a prompt, and your rule bars it from editing
  settings or allow-lists to get fewer. From the session's own log (`prompt-tally.mjs`, 388 tool calls with a recorded
  decision): 14 reached you as prompts: 8 were the session's own `sed -i` lines on its checkpoint (stopped mid-session:
  the Edit tool asks nothing), 3 the kit edits (by design: your yes in chat came first), 1 E8's merge inside its
  throwaway worktree, 1 a worktree clean-up line, 1 the session's first command; the guard refused 2 lines (`git -C`).
  Options: (a) 2.3's plan weighs allow rules or guard readings for the safe forms (git merge, revert and worktree
  removal inside a `.gate/` worktree; a scripted edit of the session's own checkpoint), each asked of you as a kit edit;
  (b) no rule change: sessions avoid those forms. Recommendation: (a), as findings for 2.3's plan (the kit stays frozen
  until 2.3).

## For Mustafa
1. Confirm you have this version: download https://github.com/Mergincanli/Shamble-Frontlines/archive/refs/heads/claude/clever-fermi-oudsfq.zip and unzip it into a new folder · on screen, the folder holds
   `legacy/` with the game in it, and no `index.html` at its top.
2. Back up your saves first (nothing deletes them; this is the safety copy). In Brave, open your current
   `index.html` (your usual copy, not the new folder), press F12, open Console (type `allow pasting` if Brave asks),
   paste this line and press Enter:
   ```js
   (()=>{const o={};for(const k of Object.keys(localStorage).sort())o[k]=localStorage.getItem(k);copy(JSON.stringify(o));return Object.keys(o).length+' keys copied: paste them into a text file now'})()
   ```
   Expected: the console answers "<n> keys copied: paste them into a text file now". Paste into a new text file outside
   the repository and save it. Do the same on your current `editor.html`. Each file starts with `{` and holds your
   keys (`shamble_prog`, `vmap:…` for each map).
3. Double-click `legacy/index.html` in the new folder. Expected: the menu opens; Deploy starts a match; you can move,
   shoot and pause as before; your rank and progress show as before (in Chromium every `file://` page shares one
   storage, E5; Brave on Windows was not tested).
4. Double-click `legacy/editor.html`. Expected: the editor's menu opens; Map editor and Maps open; your saved maps are
   listed.
5. Only if a save is missing: on that page, F12, Console, type this line without pressing Enter, replace `PASTE_HERE`
   with the whole text of your backup file, then press Enter:
   ```js
   ((o)=>{let n=0,s=0;for(const k of Object.keys(o)){if(localStorage.getItem(k)===null){localStorage.setItem(k,String(o[k]));n++}else s++}return n+' keys restored, '+s+' already there (left as they are)'})(PASTE_HERE)
   ```
   Expected: "<n> keys restored, <m> already there (left as they are)"; reload the page and the saves are back. It
   only adds missing keys and never overwrites one.
6. On Windows, the gate on this branch (PowerShell, in your repository folder; a separate folder, your own is not
   changed): `git fetch origin`, then `git worktree add ..\shamble-2-1b origin/claude/clever-fermi-oudsfq`, then
   `cd ..\shamble-2-1b\tools`, `npm ci`, `cd ..`, `node tools/gate.mjs origin/main`. Expected: the header
   "base origin/main = 76b4f7a (files at the root) | head = working tree (files in legacy/)", every check PASS, smoke
   8/8 and 5/5, GATE PASS. Afterwards delete the folder `..\shamble-2-1b` and run `git worktree prune`.
7. If every step passes, say "merge #51" in a session's chat (risk tier 2, needs your go; the advisor approves the final
   head first).

## Findings for 2.3
- The prompt tally above (PROMPTS-2.3), with every prompt and refusal listed in `prompt-tally.json` at this branch's
  last checkpoint commit.
- The guard refused `git -C <folder> <command>` twice ("git: no option before the subcommand (run git from the folder,
  without -C, -c or --git-dir)"); the route both times: `cd` into the worktree, run git there, `cd` back.
- Your finding from the Windows run (A3): the pinned Playwright's installer (before 1.60.0) hangs after the download
  reaches 100% under Node 24.16.0 and later; it finished under 24.15.0. This container's install (Node 22.23.4) did not
  hang. For 2.3's toolchain pins.
- The Bash tool's background limit (30 minutes by default) stopped the checkpoint saver once; restarted with the
  2-hour limit.

## Stale lines left for the drafts' next revision (found by docs-keeper, unedited: "nothing wider")
- ENGINE_OVERVIEW.md:4 and SPIKE_PLAN.md:5: "not yet accepted", without the exception for ADR 0002's part 1.
- V2_PHASE0_BRIEF.md:10 and ADR 0001:1769: the legacy game "moves to `legacy/` in phase 1" (now done).
- ADR 0002:150: where the legacy gate and tools live "is the phase 1 restructure plan's call (OQ-0002-2)" (reads as
  pending; now answered in part).
- The register's status sentence: "His words:" now follows the SCOPE (a) clause, but quotes SPIKE-ORDER (b) (the
  inserted words are exactly the approved ones; a reader could take the quote as SCOPE (a)'s).
- ROADMAP: 2.1a's entry still reads "PR #50: in review"; the banner (:3) names the legacy files without `legacy/`;
  Recent work has 13 entries, not folded (its history is not edited).
- STATUS_ARCHIVE: the kit-v2-1 item "on main once #50 merges" (now past); :51-53, like ROADMAP :29 and :67-69, history
  text saying the drafts are not yet accepted.
- LEGACY_TUNING.md:9 keeps the oracle's `preview.js .` command: regenerated by R7's builder, never hand-edited.
- His go for #50 has no DECISIONS.md entry (it is in merge commit 76b4f7a's message; 2.1a recorded #49's the same way).
- CLAUDE.md:47-52, the hard constraints' scope to `legacy/` under SCOPE (a): step 2.2's kit line, not 2.1b's.

## The advisor tool
Fable 5.2 since his /compact at 21:36:59 UTC (the session log's `advisorModel`); Opus 5.5 before. The session's consultations: TBD-COUNT, each in the checkpoint (when, the question, what changed). docs-keeper's own
agent consulted it once in its fix pass (its notes).

## Appendix B. OQ-0002-2's three parts (plan revision 3, 1.4; the reasons ADR 0002's answer points to)
OQ-0002-2 (ADR 0002): "Where do the legacy gate and tools live after the move, and where does the gate v2's script
live?", decided by the phase 1 restructure plan, by which layout keeps R5-LAYOUT-REC6's marks with the fewest path
changes, and where the legacy docs go (row KT21). The marks hold in either layout, so the path counts decide (the
plan's counts of 2026-11-10). The
answer is the plan's; its status, "answered in part", is Mustafa's REG-TRUE (b) of 2026-11-11 (Appendix A5).

| Part | Answer | Cost of the other option |
|---|---|---|
| The legacy gate and tools | **stay in `tools/`** (option (a)), learning `legacy/`: 2 gate files edited (2.1a, PR #50) and 2 kit files (2.1b) | (b) inside `legacy/`: the guard's gate-file lists and its npm check, the `tools/` allow rules (the plan counted 17; 15 on 2026-11-12) in `.claude/settings.json`, `gate.yml`'s `--prefix tools` and both gate steps, and kit lines in about 8 more files |
| The legacy docs (row KT21: START_HERE, MODULE_MAP, TECHNICAL, GAME_DESIGN, ART_DIRECTION, ANIMATION_PLAN, AUDIT_BUGS_PERF, `ui_theme_mockup.html`) | **stay in `docs/`**, marked legacy: no path changes | to `legacy/docs/`: 27 lines in 12 files, 8 of them protected, 2 relative links broken (GAME_DESIGN.md:69, :90), about 25 references in history and ADRs left dangling. KT21's mark is "redesign": START_HERE gets rewritten for v2, not archived. The R7 files (LEGACY_INVENTORY, LEGACY_TUNING, `legacy-tuning.json`, EDITOR_V1_INVENTORY) stay in `docs/` either way |
| The gate v2's script | **left to step 2.3's plan**, which edits the guard anyway: the gate v2 gets its own name beside `tools/gate.mjs`, or takes `tools/gate.mjs` and the legacy gate moves then | deciding now would edit the guard in 2.1 |

R5-LAYOUT-REC6's marks after the move: today's gate passes on `legacy/` (CI #674, #675; E2), the Phase 0 mutation proofs
fail on the right checks (E3, 31 of 31), `docs/legacy-tuning.json` is reproduced byte for byte (E4).
