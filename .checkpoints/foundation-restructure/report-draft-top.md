# Advisor report: foundation-restructure, step 2.1a (2026-11-11; PR #50 replacing #47; branch claude/pensive-heisenberg-aazidf for cs/foundation-restructure; base main bb2d817)

## Goal
Teach the legacy gate and tools to read the five legacy files from `legacy/` as well as from the root, before anything
moves, so 2.1b's move and a revert of it both stay green in CI. Nothing a player sees changes.

## Outcome
DONE, pending his go: game files untouched, symbols +0 −0; risk tier 2 (gate files, decision docs): needs-mustafa, merged
from a day session on his go; no Brave check. The final head and its CI runs are in the PR body (a file cannot name its
own commit's runs); the latest head with CI before this commit, 0590d8c: gate and smoke green (push 43070000525,
pull_request 43070010305). The body runs past the skill's ~60 lines because his words of 2026-11-11 added the Windows
step, the prompt tally and the refusal table.

## Plan and approval
Plan revision 3 (`plan.md`, at LAST_CKPT_SHA), from the advisor agent's APPROVE WITH CONDITIONS of revision 2 (8bbebfa);
his FREEZE-FDN (a), SPLIT (a), SCOPE (a) and build go (A2.1). C1-C8 was never committed (only C7 is labelled): the advisor
agent rebuilt it from its findings F-A to F-O, rulings Q-A1 to Q-A6 and the 8bbebfa..b03f4d8 plan diff, without
renumbering: met or for 2.1b/2.2, except L-3 and L-4 below. Amendment: the security review's F1 fix, the advisor agent's
APPROVE with conditions, all met; written on his yes (A3.3).

## What changed
- `tools/smoke.mjs` (dd12aeb; gate file, 18058e4): `findBuild`, one build-folder rule: `legacy/` when it holds any of the
  three `shamble-*.js` files, else the root; an engine file in both places: exit 2; under `legacy/` a page reads only
  inside it. As built: the three engine files, not plan 1.3's "all five files" (L-1).
- `tools/gate.mjs` (dd12aeb, 3f29cb8; gate file, 7a8480c): the head from that folder; each base from the first of
  `legacy/`, the root and `shamble-frontlines-repo/` holding all five; and (3f29cb8, F1) when the folder differs between
  base and head, the base's folder keeps no `index.html`, `editor.html` or `shamble-*.js` in any letter case, else exit 2.
  Only CI's gate job catches a decoy (`--smoke-only` has no base). Exactly `f1-gate.patch` applied to blob 2bfab53.
- `tools/village.py`, `tools/README.md` (dd12aeb). Docs: DECISIONS.md (his words of 2026-11-10 and 2026-11-11, by
  script), STATUS (#50's row, #49 merged, follow-ups, CI risk), STATUS_ARCHIVE (#49's row, follow-ups), ROADMAP, pointer
  lines in ADR 0002 and SPIKE_PLAN (decision docs). Symbols +0 −0, as planned.

## Evidence
- **Provenance:** #47's 41 commits merged onto bb2d817 as d6a3fee: its diff byte-identical to #47's; `.claude/`,
  `.github/`, CLAUDE.md identical to main's; the game files identical at d6a3fee, dd12aeb and main.
- **E3, the mutation proof:** 31 of 31 on dd12aeb's gate (2026-11-10) and again on the fixed gate (2026-11-11), identical
  per entry and column (93 runs each); the `legacy/` column is a move, so the new check ran on every one of its entries.
- **F1 red then green:** `rows-test-f1.mjs` (13 + 2 rows): the unfixed gate lets 4 folder-move rows through (exit 0); the
  fixed gate stops them (exit 2) and passes the no-move and clean-revert rows. The decoy: main's gate exit 2, the unfixed
  gate exit 0, the fixed gate exit 2, its `--smoke-only` exit 0.
- **Both move directions, raw with exit codes** (closes TE-1): 2.1b's PR exit 0; its revert exit 0 (base prefix legacy/,
  on the reproducible base 89286ed and on a local move commit); main's gate on that base exit 2. IE-6 settled: exit 0.
- **Gate:** with smoke PASS, +0 −0, 8/8, 5/5. **His words:** Appendix A, 12 blocks, byte-identical to their sources;
  DECISIONS.md's check 13 quoted runs, each its source; security-auditor matched A3.3-A3.5 to the session events.
- **CI:** gate green on every head; the smoke job never reached its test on 3f29cb8 (push run both attempts; pull_request
  run once, then cancelled at the job timeout) and on 8d8c422 (both runs): the runner could not resolve the Ubuntu
  mirror. Green from 0607b45 on (the same `tools` tree and game blobs as 3f29cb8): 0607b45, 76544aa, 0590d8c.
- **Docs commits:** 2dd8827 (Appendix A), 1023870 (the docs pass), 3c98ad5 (touch-ups), 8d8c422 and 0607b45 (his words,
  L-2, follow-ups), 76544aa (ROADMAP), 0590d8c (round 2's follow-ups), and this report's commit.
- **Verifier:** VERIFIER.
- **Not verified:** Windows (the step below); the pinned Chromium 1194 on the `legacy/` layout (local smoke ran 1243
  under its name: L-4); A1.1-A1.3, A2.1, A2.2 and A4 against their chat records (not readable from this account).

## Reviews
- Round 1 (3c98ad5): infra-engineer APPROVE (4 Lows); test-engineer APPROVE (TE-1 Medium, closed; 5 Lows); security-auditor
  REVISE (F1 High: a decoy or a stray page in the old folder passed on a move); the advisor agent REVISE (F1).
- F1 fixed in 3f29cb8 on his yes; round 2 (76544aa): all three APPROVE, F1 closed. Every Low and the one Medium (IE-5,
  CI's live mirrors) are follow-ups in the archive's group "From foundation 2.1a" and STATUS's bullet.

## Decisions and deviations
- BRANCH-47 (a): #50 replaces #47 by one merge (no rebase). **The exception (his condition, one line):** the main folder
  fast-forwarded once, bb2d817 to d6a3fee, on his yes, its `.claude/`, CLAUDE.md and `.github/` byte-identical to main's.
- F1-FIX (a), FREEZE-FDN (a) for this one check (A3.3). #49's merge recorded here, as its merge commit asks.
- L-3: docs-keeper ran at `high` for one ADR 0002 pointer line (CLAUDE.md: `xhigh` for ADR work; the line is byte-checked).
- V-2: `BUILD_PREFIX` in `tools/gate.mjs` is not named in plan revision 3 (disclosed in dd12aeb's message); `readdirSync`
  is inside the approved patch. TE-4: a broken `tools/smoke.mjs` now stops the whole gate with exit 2, even with
  `--no-smoke` (it fails closed). TE-5: the DECISIONS check compares against the appendix, the appendix against the notes;
  A2.3 was byte-compared to the old chat's record, and A3.1-A3.5 were extracted from this session's events.
- The advisor tool: N_CONSULT consultations, each logged with what it changed, readable at LAST_CKPT_SHA (the checkpoint's
  "Advisor-tool consultations"); the count runs to the report's writing, and any later one is named in chat.
- security-auditor asked for its threat model in agent memory: not done here (out of scope).

## Risks
1. F-L: a revert of #50 also removes #49's record and his words of 2026-11-10 and 2026-11-11 from DECISIONS.md.
2. The edited gate has never run on Windows. 3. The v2 root page against pre-move bases (a 2.3 known limit).
4. CI's smoke depends on live mirrors (IE-5). 5. R2-1, A-1, TE-2: narrow gaps, all failing closed or unreachable today.

## Proposed next step
2.1b, the move, in a fresh chat (this one is about CHAT_FULL full); it reads `plan.md` and `prompt-2.2.md` at LAST_CKPT_SHA.

## Findings for 2.3: what asked him, and every refusal (his words of 2026-11-11, A3.4 and A3.5)
- **What asked him** (the session's record, 09:59-16:37 UTC, 499 tool calls): 14 prompts, all from the main session: git
  worktree and branch operations in `.gate/` worktrees 6 (worktree add 3, switch --detach 2, the one fast-forward 1);
  shell writes the guard cannot read 8 (variables, loops, redirects; one was the approved gate write, answered after his
  chat yes). Auto-allowed: classifier 318, settings rules 99, permission mode 40. Subagents never prompt: their writes
  are refused. A snapshot to 16:37 UTC.
- **Every refusal, with the route taken:** REFUSAL_COUNT, in the table at the end, before Appendix A.

## The Windows step (security-auditor's ask, before 2.1b)
After #50 merges, on his Windows machine in the repo folder: `git pull`, `cd tools`, `npm ci`,
`npx --no -- playwright install chromium`, `cd ..`, then `node tools/gate.mjs 44446a5`: GATE PASS, the header naming
"files at the root" twice (the folder rule and base reading on Windows; nothing moves, so not the move check). On 2.1b's
branch before 2.1b merges: `node tools/gate.mjs origin/main`: GATE PASS, the header "(files at the root) | head = working
tree (files in legacy/)" (the move check on Windows).

## Decisions needed
None.

## For Mustafa
1. Confirm that the texts not checked against a chat record are your words: A1.1-A1.3, A2.1 and A2.2 (the foundation
   chat's notes; A2.2's item 1 is the authority for dd12aeb's two gate-file writes) and A4 (your go for #49 as merge
   commit bb2d817's message records it).
2. Your go for #50 in a day session's chat, "merge #50", once the advisor approves its final head (risk tier 2:
   needs-mustafa; no Brave check). The head and its CI runs are in the PR body.
3. Before 2.1b: the Windows run above.

## The refusals of 2026-11-11, each with the route taken (the next call the same agent made)
REFUSAL_TABLE

