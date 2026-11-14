# Reading notes: foundation-2-3-plan (main session; main 4460ef5)
Plan (blocks, in order): B1 STATUS · B2 DECISIONS (Queued; 2026-11-10 to -13) · B3 merge 4d21c1a · B4 the 2.2 report ·
B5 ADR 0002 parts 7-9, migration step 2.3, PM-0002-39..62 · B6 old plan part 3 (26e0d82) · B7 the 2.1a and 2.1b reports'
findings for 2.3 · B8 the 2.3-docs report's five findings · B9 the archive's follow-ups and risks for the gate step ·
B10 the kit lines (models, advisor tool, the 2.1/2.2 exception, the required checks) on main · B11 the guard and CI on
main · B12 PR #40's state.

## B1 STATUS (main 4460ef5) — done
- :5 "Last updated: 2026-11-14 (... PR #52; main 4d21c1a)"; :13 "Last merged: foundation 2.2 (PR #48 ...)"; :29 #52's
  row in "In flight"; :36 "step 2.3: its first docs pass (#52), then its plan after #52 merges"; :43 "His go for #52"
  in "Waiting on Mustafa". All stale since 4460ef5 (#52 merged 2026-11-14 09:06:30 UTC).
- :21-23 main protected: `gate` and `smoke` required; `toolchain` not required (REQUIRED-CHECK).
- Follow-ups for the gate step, :69-100: Security (PX-1 + 2 test gaps, W1, W2, W3, Windows-name limits); Kit after the
  gate step (F2, F5, F-1, L12, L14, L9/SA-9, L13, L1's runners, A-4, FU-P3a, FU-P4a..d, A3's plan readers, A3's cuts,
  CHECKPOINTS §7 cap line, E9's untested cases, KIT-ALIGN, the kit-v3-1 Windows test); status-diet after the gate step
  (advisor.md :17 :286, netcode-architect.md :24 citations; SD-N1, SD-N2, SD-N3, SD-N6, SD-L3); 2.1a for 2.3 (IE-1,
  IE-5, TE-2, TE-7, R2-1, A-1, N2, L-4, v2 root page vs pre-move bases; five "before the foundation" lines for the
  drafts); 2.1b for 2.3 (Node 24 hang; stray-page check F4 Low; PROMPTS-2.3; `git -C` refusals); 2.2 for 2.3 (AUTO-ASK,
  MANUAL-MODE, GIT-C, ADVISOR-TOOL-WORDS, MODELS-2026-11-12, REFUSED-PUSH, PIN-GUARD, G0, OVERFLOW-XCRATE-CI, TSCONFIG,
  REQUIRED-CHECK, SAVER-SKIP-CI, CRATE-NAMES, CI-ENFORCED-LATER, WASM-ARTIFACT, TWO-PLAYWRIGHTS, TOOLKIT-RESOLVES-UP,
  CARGO-NET-OFFLINE, CRATE-LICENCE, CUT-CARRY, CANCEL-ON-MAIN, LINT-TABLE-PEDANTIC, FEATURE-UNIFY, ROOT-WORKSPACE,
  CLIENT-NET-RULE, NODE-26; later: OVERFLOW-PER-CRATE, PACKAGE-ENTRIES, EMPTY-SLOTS, OQ-0002-8-TEST, BRIEF-STALE;
  accepted open: SETPRIV-HARDEN, SETUP-NODE-CACHE, CUT-ESCAPE (2.3, CI edits that ask him), OVERFLOW-PANIC-RULE).

## B2 DECISIONS — done
- Queued items that bind 2.3: KIT-FREEZE-2 + FREEZE-FDN (:27-40; ends with "the foundation's gate step"); the CLAUDE.md
  clarification of the advisor tool (:76-80, "this clarification goes into it then"); the CLAUDE.md and agent-file change
  for models (:86-91). Not 2.3: the unit walk (:41-72), the Slice 1 note (:73-75), the render budget bench note (:81-85).
- His words on models, 2026-11-12 15:35:17 UTC (:236-254): Fable 5.2 main model of every chat that writes game or engine
  code (personal account) and of its builder agents; every other agent Opus 5.5, may run at max; reviewers/verifiers/
  designers of game and engine code always at max; guard, gate and CI work stays with Opus 5.5; the netcode chat keeps
  its models; replaces LANE-RULES' "The main model and every agent stay Opus 5.5"; step 2.3 changes CLAUDE.md "Models and
  effort" and the agent files, each edit asked in chat with exact text.
- His advisor-tool words, 2026-11-11 11:56:44 UTC (:470-482): extra = on top of the advisor agent's plan review, never
  instead; five moments; usage no reason to skip; one checkpoint line per consultation, counted in the report; if off,
  say so; ("main model and every agent stay Opus 5.5": replaced); CLAUDE.md takes the clarification at 2.3.
- PROMPTS-2.3 (a) (go for #51, :283): "Step 2.3's plan weighs those guard rules as findings. ... each rule change is
  asked of me in chat at 2.3."
- KIT-ASK-SEEN (b) (:531-534): "auto mode can pass the guard's ask" a kit finding for 2.3; no guard change before.
- Permission prompts (:404-424): guard prompt counts as not seen only for kit/gate edits and main-folder moves; report
  the commands that asked most and every refusal (findings for 2.3); do not edit settings/hooks/allow-lists for fewer
  prompts.
- Go for #46 (:588): PX-1, W1, W2, W3 and the Windows-name limits stay in STATUS until the foundation's gate step.
- Go for #48 (:181-197): item 5 accepts SETPRIV-HARDEN, SETUP-NODE-CACHE, CUT-ESCAPE "each for step 2.3, as a CI edit
  that asks me"; OVERFLOW-PANIC-RULE with the first room/server code.
- 2.1b's Node 24 finding (:339): Node >= 24.16.0 with Playwright < 1.60.0 hangs after 100%.
- 2.2's task answer (:579): guard checks for the new pins wait for the gate step (2.3).
- His decision of 2026-11-10 (:634): "Steps 2.3 to 2.6 keep their order and wait for their results" (changed for 2.3
  only by 2026-11-13).

## B3 merge 4d21c1a — done (his go for #48; same text as DECISIONS :181-197)

## B4 the 2.2 report — done
- Findings for 2.3 table :330-361 (full text in the archive, "From foundation 2.2"); later :363-397.
- Risks :295-301: at 2.3 the required check moves to the gate v2's aggregate check, with his ruleset setting (ADR 0002
  :1421) and every kit line naming gate and smoke as required, each asked. REQUIRED-CHECK's evidence list (:343):
  CLAUDE.md:119 and :130; advisor.md:157, :163, :224, :273, :294, :323; routine-guard.mjs:1486 + comments/tests; ADR 0002
  :642, PM-0002-57 (line numbers at 2.2's time: re-find on 4460ef5).
- G0 (:340): exit 2 for the missing toolchain file under `--prove`; source checks; mutations listed.
- CUT-CARRY (:353): 2.3 keeps the cut around build+test or drops it by a recorded decision; a kept cut takes
  SETPRIV-HARDEN and CUT-ESCAPE, plus a red run with `--net` removed.
- Kit edits K1..K4 table :112-119 (the exact K3/K4/K2b texts now on main).

## B5 ADR 0002 — done
- Part 9 (:562-654): one Node command (working name `node tools/gate.mjs`, v2 edition; path = OQ-0002-2), conventions:
  base-ref rule, exits 0/1/2, `--json`, `--allow-removed`, `--quick` (G0-G5 + changed crates' tests), `--prove`.
  G0 setup (exit 2) · G1 format · G2 Rust lint (+determinism/panic sets, workspace lints opt-in, part 4 build-flag check)
  · G3 TS (tsc -b, ESLint typed) · G4 boundary (generated types/layout/schema zero diff; wasm-bindgen CLI = lock) · G5
  deps (cargo-deny bans, cargo tree, multiple-versions/wildcards deny, same-feature check across server/devtools/navbake/
  sim-wasm, sources crates.io only) · G6 build (native release devtools/server; sim-wasm to Wasm; Vite both entries) · G7
  budgets (Wasm + JS bytes vs budget file; sim-wasm size recorded not gated until its first limit) · G8 player build
  (Vite manifest walk; CSP-ready) · G9 tests (cargo test, proptest fixed seed, goldens, insta, Wasm tests in Node,
  Vitest) · G10 determinism (short recordings, native x2, Wasm in Node and 3 browsers) · G11 L1 network · G12 Playwright
  e2e (built client + local server) · G13 maps (slice 2).
  CI option G1: one workflow, parallel jobs, ONE final aggregate job = the only required check (`if: always()`, fails
  unless every needed job succeeded; a skipped job fails; compares the set of check ids that ran with the gate's list).
  No path filters. Caching: Swatinem/rust-cache v2.9.3 by SHA, shared-key per kind, saved only from main pushes;
  setup-node `cache: npm` on the root lockfile, `npm ci --ignore-scripts`; Playwright browsers not cached; no package
  cache in a job with a secret. Hygiene: SHA pins, contents read, persist-credentials false, concurrency cancel, per-job
  timeouts; Dependabot for github-actions (tried on one action in the foundation). Scheduled jobs (soak, fuzz, ...) not
  in the PR gate. WebGPU in CI: record per engine the backend it gets.
- Migration step 2.3 (:1463-1470) done-when: (D1) `--prove` passes, every mutation fails exactly its named checks, every
  control passes; (D2) a TS-only PR runs and passes the single required check; (D3) warm PR wall time <= 10 min p50
  over 5 runs, <= 20 min cold; (D4) a failing and a cancelled job each turn the required check red; (D5) a fork PR runs
  every job with no secret; (D6) the repo's Rust cache <= 8 GB; (D7) Mustafa names the required check in his branch
  protection. Scope line: "G0-G12 (G13 joins with the first committed map, slice 2), `--prove`, the budget file, the
  workflow with its one required check".
- Pass marks PM-0002-39..62 (:1305-1328): 39 G0 (incl. missing Playwright browser -> exit 2; control: consistent bump);
  40 G1 (CRLF, misformatted .rs/.ts; Windows/Linux identical tree once); 41 G2 (many planted defects in sim/room/
  client-endpoint/decoders/build flags; controls in server); 42 G3 (one per strict flag, no-eval, no-new-func,
  unawaited promise, generated-union switch, editor import, bare `event`, DOM in sim-worker/net, import from legacy/
  spikes/); 43 G4; 44 G5 (tokio/quinn in sim, git dep, wildcard, 2nd version, feature diff across 4 builds); 45 G7; 46
  G8; 47 G9 layers; 48 G10 (<3 min); 49 L1 exact (1M packets/profile); 50 G11; 51 G12; 52 `--prove` passes, at least
  one mutation and one control per check id G0-G12, one run < 30 min; 53 overflow rule 4 builds (native+Wasm, dev+
  release) + wrapping_add + cost; 54 lint/type speed (tsc -b < 30 s, typed lint < 60 s, clippy < 30 s warm, < 3 min
  cold); 55 `--quick` < 60 s, full gate < 10 min warm Linux AND Windows (MP); 56 CI <= 10 min p50 warm, <= 20 cold;
  57 one required check that cannot be fooled (TS-only PR passes; failing/cancelled/skipped job red; removing a check
  fails aggregate); 58 fork PR, no secrets (test account, proposed); 59 caches (warm no recompile; <= 8 GB); 60
  wasm-bindgen CLI install exact, < 30 s; 61 no flaky test (20 runs identical; planted timing test caught; retry-only
  test fails Playwright); 62 per-edit hook (lands in its kit change-set, phase 1, not the foundation).
- OQ-0002-2 (:998-1021): parts 1-2 answered by 2.1b (legacy gate + tools stay in `tools/`; legacy docs stay in
  `docs/`); part 3 (the gate v2's script path) "left to step 2.3's plan, which edits the guard anyway".

## B6 old plan part 3 (26e0d82, .checkpoints/foundation-restructure/plan.md:442-452) — done
- 2.3 waits for: 2.2 merged; the netcode proof test's L1 (G11); the sim core spike's replay runner and state hash (G10;
  the sim's dependencies set G5's feature comparison); the boundary spike (G4, G9's boundary layer, part 9's WebGPU-in-CI
  route); PR #46 merged (the guard must learn the gate v2's files); the gate v2's path (decided in 2.3's plan); his
  branch-protection setting. Also SPIKE_PLAN's foundation entry: "the drafts accepted" (lifted for 2.1/2.2, and by
  2026-11-13 for 2.3's parts that need no result).
- Today: 2.2 merged (4d21c1a); #46 merged (2d22b69); netcode proof test (#40) still running (U4); sim core spike and
  boundary spike not started (SPIKE_PLAN order); no G-check code on main.
- Old plan part 4 (PR #40 after each merge): merge main in its worktree; expect conflicts in docs/STATUS.md,
  REPO_MAP.md; nothing in spikes/ changes; its check.mjs reads non-Markdown files outside spikes/, docs/, .checkpoints/
  (referenceProblems: no `spikes/` text there) and the PM rows of ADR 0001/0004/0006 (marks.json hashes).

## B7 2.1a and 2.1b reports' findings for 2.3 — done
- 2.1a :81-88: "what asked him" (14 prompts: worktree/branch ops in .gate/ 6; shell writes the guard cannot read 8);
  37 refusals (:107-146): git -C; writes after `cd` (agents); write targets it cannot read (variables, loops, sed -i);
  git checkout / restore / reset --keep / worktree remove --force; permission system denials.
- 2.1b :168-180: PROMPTS-2.3 (prompt-tally.json at its last checkpoint commit); `git -C` refused twice; Node >= 24.16 +
  Playwright < 1.60 install hang (for 2.3's toolchain pins); the stray/decoy page check no longer runs after the move
  (security-auditor F4: refuse page and engine names, any case, outside the build folder on every run); the Bash
  background limit stopped the saver once. Appendix B :200-215: OQ-0002-2's part 3 "left to step 2.3's plan ... the
  gate v2 gets its own name beside `tools/gate.mjs`, or takes `tools/gate.mjs` and the legacy gate moves then".

## B8 the 2.3-docs report's five findings (:287-296) — done
1 the 3 kit lines naming 2.1/2.2 as the only exception (CLAUDE.md :22, :55; advisor.md :108); 2 BRIEF-STALE and the
"before the foundation" lines lack the 2026-11-13 decision too; 3 no `tools/` install in this container, local smoke
cannot start (Chromium build mismatch); 4 the advisor-tool moment 5 is easy to miss when the checkpoint is saved first
(a line in CLAUDE.md's advisor-tool wording); 5 read the advisor tool's model from the records, not `/advisor` output.

## B9 the archive's follow-ups for the gate step — done (texts read: :217-321, :428-509, :509-556, :586-655)
- kit-v3-3 (A3) items, "after the kit freeze": F-1 (CHECKPOINTS §4 trust rule; ~15 words at :77-78, :110-111, :119 +
  3-run dry run); CHECKPOINTS §7 cap line; E9's untested cases; A3's plan readers (WebFetch list; `node .gate/*.mjs`
  matches `..`); A3's cuts (secret check decoding; encoded `@`; routine reads on other repos); L12 (day fetch to
  refs/remotes/origin/* or refs/heads/main asks); L14 (git aliases); L9/SA-9 (fold case); L13 (ask in cloud day session
  and bypassPermissions); L1's runners (sudo, doas, sh -c, eval); A-4 (day bootstrap passes when routine-guard.mjs is
  missing: ask instead); FU-P3a (day reader vs bash corpus); FU-P4a (Medium: here-doc bodies unseen by D-ASK); FU-P4b
  (D-ASK prompt cost; his call); FU-P4c (Medium: clone folder after unknown options); FU-P4d (R-PROG drops hints).
- kit-v3-4: F2 (High, fixed in text: CI passes no --allow-drop, so a pure revert of a symbol-adding merge fails CI's
  gate; a way for CI to accept a pure revert's drops, only for a first-parent merge named in "This reverts commit";
  until then reverts go to Mustafa); F5 (revert check's third "main breaks" case; a loosening needs his words).
- kit-v3-5: PX-1 (merge check reads `gate`/`smoke` by name only; bind to the gate workflow's run on the head SHA and the
  Actions app; proof test first; his PX1-ROUTE (a): "its own change-set after the foundation's gate step"); two test
  gaps ride PX-1; W1, W2, W3 + Windows-name limits (in "Open bugs and risks").
- KIT-ALIGN (:470-480): CLAUDE.md :7-8, :208 and agent-file lines stale vs the 2026-10-31 answers.
- status-diet: STATUS citations in advisor.md :17, :286, netcode-architect.md :24; SD-N1 (paste block vs advisor.md
  :204-207), SD-N2 ("the advisor" -> "the advisor agent's verdict" in Standing permission and advisor.md step 6), SD-N3
  (lanes unenforceable), SD-N6 (archive copies of his words merge by path), SD-L3 (index misses wrapped lines).
- 2.1a for 2.3: IE-1 (smoke.mjs:31 wording), TE-2, A-1, N2, L-4 (CLAUDE.md's `npx --no -- playwright install chromium`
  refused in a cloud day session by the settings' npx deny), v2 root page vs pre-move bases, IE-5 (Medium: live
  mirrors; step timeout + retry, `cache: npm`, or an image), TE-7, R2-1 (trailing dot/space; pattern given).
- 2.2 for 2.3: as the report's table (B4); G0's source checks and mutations (:611-630).

## B10 kit lines on main 4460ef5 — done (exact texts extracted by script into kit-before.md when the plan is written)
- The 2.1/2.2 exception: CLAUDE.md:22 (K3), CLAUDE.md:55 (K2b), advisor.md:108 (K4).
- Required checks named: CLAUDE.md:119 (Rollback), :130 (merge rule); advisor.md :157, :163, :224, :273, :294, :303,
  :323; routine-guard.mjs:6 (comment), :1486 (`for (const nm of ['gate', 'smoke'])` in mergeCheck), :990 (pr-advisor
  message); routine-guard.test.mjs :45, :144, :145, :364 (+ more rows to find).
- Agent files: all 13 `model: opus`; effort high except engine-architect and netcode-architect (xhigh); advisor.md:123
  "model and effort follow `CLAUDE.md` "Models and effort"".

## B11 the guard and CI on main — done
- routine-guard.mjs:28-29 GATE_FILES = tools/gate.mjs, smoke.mjs, battery.js, gate-globals.json, package.json,
  package-lock.json, npm-shrinkwrap.json; protectedRel (:35-48): CLAUDE.md/.npmrc/.mcp.json/CLAUDE.local.md/AGENTS.md
  at any depth, .github/, .git/, .claude/ (but agent-memory-local), GATE_FILES. mergeAllowed (:87-100): places docs/,
  tools/, README.md, FIRST_PROMPTS.md; refuses rules names, gate names under tools/, docs/adr/, decision docs. So a gate
  v2 file under tools/ that is not in GATE_FILES would be routine-writable, unasked in day sessions, and PR-Advisor
  mergeable: the guard must learn it before the gate v2's code lands.
- mergeCheck (:1473-1510): expectedHeadSha; same repo; the check list read whole; `gate` and `smoke` by name (PX-1).
- No guard code reads the hook's permission mode (grep: none) -> AUTO-ASK as a guard change needs the hook input's
  `permission_mode` field verified first.
- settings.json allow: `node tools/gate.mjs`, `node tools/gate.mjs *`, smoke, battery, preview, pageshot,
  `node tools/node_modules/playwright/cli.js install chromium` (:12), `node .gate/*.mjs`; deny: `npx *`, `npm exec *`,
  `npm run *`, `npm test*` (:237-240) -> the gate v2 should call tool binaries by path, never package scripts.
- gate.yml: jobs `gate` (legacy checks 1-4, 5b-5d) and `smoke` (5a), concurrency cancel-in-progress on every ref
  (CANCEL-ON-MAIN), setup-node without cache (IE-5), apt install with no timeout/retry (IE-5).
- toolchain.yml: one job `toolchain`; RUSTUP_AUTO_INSTALL=0; pin check :44-53 (source checks); fetch outside the cut;
  the cut :60-88 (unshare --net, setpriv --reuid --regid=nogroup --clear-groups --no-new-privs, env -i, probe).

## B12 PR #40 — done (read 2026-11-14 ~11:10 UTC with the GitHub API)
- Draft, head 4bd5ac3 (2026-11-14 08:31:05 UTC: "U4r's round-1 ruling in ... fix pass FP-1 to FP-11"), base shown
  4d21c1a, mergeable_state "dirty" (behind 4460ef5 and conflicting), 820 files, 684 commits. Its result (L1) is not
  near. Its own rules: never push to its branch, never touch spikes/. Its Safari workflow is planned as a separate
  `.github/` PR (W-PR) asking Mustafa.
