# Advisor report: foundation-restructure, step 2.1b (2026-11-12; PR #51; branch claude/clever-fermi-oudsfq for cs/foundation-restructure; base main 76b4f7a)

## Goal
Move the legacy game's five files into `legacy/` with their bytes unchanged, so the top of the repository is free for
v2 (ADR 0002 part 1), with every command, kit line and doc pointing at the new place, and the move provably revertible.

## Outcome
DONE: the move, the commands, the four kit lines (on his yes), evidence E1 to E6 and E8 (all PASS), the docs, the
reviews. Final head TBD-HEAD; CI gate and smoke green on it (push run TBD, pull-request run TBD). Risk tier 2:
needs-mustafa.

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
- `docs/`: DECISIONS.md (six entries and the Queued note, by script), ADR 0002 (Status line by script; :7, OQ-0002-2's
  answer, two rows), the register, TBD-DOCS; this report.
- Not touched: the gate files, settings, hooks, CI, `spikes/`, the tuning oracle files, every PM row.

## Evidence (notes, scripts and outputs: this branch's last checkpoint commit, TBD-CKPT)
| Check | Result |
|---|---|
| Gate, move commit, local | vs 76b4f7a and vs 2f4b974: PASS, "(files at the root) \| head = working tree (files in legacy/)", +0 -0, smoke 8/8 and 5/5 |
| CI, move commit c090843 | push #674 (base 2f4b974) and pull request #675 (base origin/main = 76b4f7a): gate PASS with that header; smoke 8/8 and 5/5 on the pinned Chromium |
| E1, the bytes | `git diff --find-renames=100% --name-status c090843^ c090843`: five R100 rows, nothing else; blobs equal at 76b4f7a, 44446a5 and `legacy/` (e528dca, 24e86d5, fce6f37, 008c66f, 5ba72df); final head: TBD-E1 |
| E2, OLD's gate | 44446a5's untouched gate files run on the moved bytes: GATE PASS, +0 -0, smoke 8/8 and 5/5 |
| E3, the gate bites | 31 of 31: each mutation fails the same checks in OLD's gate (files at the root) and the head's gate (files in `legacy/`); controls pass in both; M19 and M20 differ from the 2026-10-14 report's loose "5b + 5d" wording only (as in 2.1a). The head's gate vs 44446a5, 8e58937 (+34 -0), 76b4f7a, c090843, b6b3b81: PASS |
| E4, the oracle | tuning dumps at A (`.`) and head (`legacy`), both maps, twice: byte-identical (35792 and 37063 characters); R7's builder reproduces `legacy-tuning.json` (78862 bytes) and `LEGACY_TUNING.md` (22018 bytes); the head's edited tools give the same dump (`cmp`) |
| E5, the saves | headless Chromium 1194 on Linux: SHARED (every `file://` page, any folder, one storage); control and reverse hold. Both console lines on the real `legacy/` pages: 10/10. Brave on Windows: not verified |
| E6, the commands | 14 of 14 changed command lines run as written; kit tests 1261/1261; `repo-map-check.mjs` TBD-RMC |
| E8, the revert | main + `git merge --no-ff` of the PR head (c33ac99) + `git revert -m 1` (5f91112): tree = 76b4f7a; `node tools/gate.mjs c33ac99` PASS ("(files in legacy/) \| head = working tree (files at the root)"), `--smoke-only` PASS |
| Kit lines | b6b3b81: the four removed and added lines equal the approved text and main's before lines, by script (14/14) |
| Letter case | Linux: `Legacy/` and `legacy/SHAMBLE-*.js` are not counted by `findBuild` (`case-test.mjs`); the move check catches `Index.html` (2.1a's row j). Windows and macOS: by reading the code, not run |
| CI, final head | TBD-CI |

## Reviews
TBD-REVIEWS

## Decisions and deviations
- His answers: D1 (a); KIT-2.1B (a); E8-RUN (a); REG-TRUE (b): OQ-0002-2 stays open, "answered in part"; 8 open; the
  register's Total line unchanged (Appendix A3, A5).
- E3's head column ran at b6b3b81, not the move commit: the same gate files and the same five blobs.
- E5 gained run 4 (the console lines on the real pages, C7); E8 merged the PR head of the moment (f0322d2).
- The step 10 self-check (a script and one blind reader of the ADR and register edits) ran before the reviews; not a
  review round.
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
TBD-DECISIONS (PROMPTS-2.3)

## For Mustafa
TBD-FOR-MUSTAFA (backup with both console lines; Brave check of legacy/index.html and legacy/editor.html; the Windows
command; his go)

## Findings for 2.3
- TBD: the prompt tally; the two guard refusals (`git -C`) with routes; his Node 24 finding.

## The advisor tool
Fable 5.2 since his /compact at 21:36:59 UTC (the session log's `advisorModel`); Opus 5.5 before. Consultations: TBD.

## Appendix B. OQ-0002-2's three parts (plan revision 3, 1.4; the reasons ADR 0002's answer points to)
OQ-0002-2 (ADR 0002): "Where do the legacy gate and tools live after the move, and where does the gate v2's script
live?", decided by the phase 1 restructure plan, by which layout keeps R5-LAYOUT-REC6's marks with the fewest path
changes, and where the legacy docs go (row KT21). The marks hold in either layout, so the path counts decide. The
answer is the plan's; its status, "answered in part", is Mustafa's REG-TRUE (b) of 2026-11-11 (Appendix A5).

| Part | Answer | Cost of the other option |
|---|---|---|
| The legacy gate and tools | **stay in `tools/`** (option (a)), learning `legacy/`: 2 gate files and 2 kit files edited (2.1a, PR #50) | (b) inside `legacy/`: the guard's gate-file lists and its npm check, 17 `tools/` allow rules in `.claude/settings.json`, `gate.yml`'s `--prefix tools` and both gate steps, and kit lines in about 8 more files |
| The legacy docs (row KT21: START_HERE, MODULE_MAP, TECHNICAL, GAME_DESIGN, ART_DIRECTION, ANIMATION_PLAN, AUDIT_BUGS_PERF, `ui_theme_mockup.html`) | **stay in `docs/`**, marked legacy: no path changes | to `legacy/docs/`: 27 lines in 12 files, 8 of them protected, 2 relative links broken (GAME_DESIGN.md:69, :90), about 25 references in history and ADRs left dangling. KT21's mark is "redesign": START_HERE gets rewritten for v2, not archived. The R7 files (LEGACY_INVENTORY, LEGACY_TUNING, `legacy-tuning.json`, EDITOR_V1_INVENTORY) stay in `docs/` either way |
| The gate v2's script | **left to step 2.3's plan**, which edits the guard anyway: the gate v2 gets its own name beside `tools/gate.mjs`, or takes `tools/gate.mjs` and the legacy gate moves then | deciding now would edit the guard in 2.1 |

R5-LAYOUT-REC6's marks after the move: today's gate passes on `legacy/` (CI #674, #675; E2), the Phase 0 mutation proofs
fail on the right checks (E3, 31 of 31), `docs/legacy-tuning.json` is reproduced byte for byte (E4).
