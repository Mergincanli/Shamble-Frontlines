# Plan revision 2: folds the advisor agent's review of revision 1 (03d05f5) into plan.md. Each replacement must match
# exactly once; Appendix K is replaced by the regenerated kit-texts.md. Usage: python3 -I rev2.py
import re, sys
BASE = '/home/user/Shamble-Frontlines/.checkpoints/foundation-2-3-plan/'
P = BASE + 'plan.md'
s = open(P, encoding='utf-8').read()

def rep(a, b):
    global s
    n = s.count(a)
    if n != 1:
        sys.exit(f'{n} matches for: {a[:90]!r}')
    s = s.replace(a, b)

def row_mark(num, new_mark):
    global s
    m = [l for l in s.split('\n') if l.startswith(f'| {num} |')]
    if len(m) != 1:
        sys.exit(f'row {num}: {len(m)} lines')
    old = m[0]
    prefix = old[:-2].rsplit(' | ', 1)[0]
    rep(old, f'{prefix} | {new_mark} |')

def part2_row(start, new_row):
    global s
    m = [l for l in s.split('\n') if l.startswith(start)]
    if len(m) != 1:
        sys.exit(f'part 2 row {start!r}: {len(m)} lines')
    rep(m[0], new_row)

# --- header
rep('# Plan: foundation step 2.3, the gate v2 and CI, and the end of the kit\'s freeze (revision 1)',
    '# Plan: foundation step 2.3, the gate v2 and CI, and the end of the kit\'s freeze (revision 2)')
rep('- **Sources in this folder:**',
    '- **Revision 2** folds in the advisor agent\'s review of revision 1 (03d05f5: REVISE, one High, seven Medium, five Low,\n'
    '  and its four rulings). Part 11 lists what changed.\n'
    '- **Sources in this folder:**')

# --- summary (H1, M2, M3)
rep('  - The first part fully meets only PM-0002-40 (with his one Windows run), PM-0002-54, PM-0002-57, and D2, D4, D6 and\n'
    '    D7.\n',
    '  - By CS-E, the first part fully meets PM-0002-40 (with his one Windows run), PM-0002-54, and D4, D6 and D7.\n'
    '    PM-0002-57 and D2 are met at its closing proof, after his second ruleset step (PROOF-57).\n')
rep('  - Several marks name things that arrive only after the foundation: `server` (slice 3), the editor (slice 5),\n'
    '    legacy goldens (slice 1). Those clauses join there, so PM-0002-41, -42, -46 and -47 are met only then.\n',
    '  - Several marks name things that arrive only after the foundation: `server` and `navbake` (slices 3 and 2), the\n'
    '    editor (slice 5), legacy goldens (slice 1), ADR 0003\'s count baseline. Those clauses join there, so PM-0002-41,\n'
    '    -42, -44, -46, -47, -51 and -52 are met only then.\n')
rep('  - six exact-text asks (CS-B, C1, D, C, E, F). Each is a patch at a pushed checkpoint commit, approved by naming the\n'
    '    commit\'s SHA and written in Accept edits, with about 2 to 21 guard prompts each, about 41 in all;\n',
    '  - seven exact-text asks (CS-B, C1, D, C, F, and two for CS-E: its workflow, and its two planted workflow failures\n'
    '    with their reverts). Each is a patch at a pushed checkpoint commit, approved by naming the commit\'s SHA and\n'
    '    written in Accept edits, with about 2 to 21 guard prompts each, about 45 in all;\n')
rep('  - one scratch branch;\n', '  - three scratch branches, for PROOF-57\'s pull requests;\n')
rep('  - ALLOW-GATE2: an allow rule for the gate v2\'s command.\n',
    '  - ALLOW-GATE2: an allow rule for the gate v2\'s command;\n'
    '  - ROLLBACK-SCOPE: whether a red gate v2 job on main counts as "main breaks" (may wait until CS-F).\n')

# --- Part 0, Rule 3 (M4)
rep('  - **Rule 3, code from a later step.** A check whose subject arrives only with a later step (2.4, 2.5) joins with that\n'
    '    step. A check with nothing to check would be a feature "for later", which CLAUDE.md rules out.\n',
    '  - **Rule 3, this plan\'s choice under his "may".** His decision says the parts that need no result "may be built"\n'
    '    now; it does not say they must. A check whose subject arrives only with a later step waits for that step, even\n'
    '    when its result is in earlier, because a check with nothing to check would be a feature "for later", which\n'
    '    CLAUDE.md rules out. The checks this choice holds back:\n'
    '    - G4 and the generated-union mutation (step 2.4\'s generators);\n'
    '    - G9\'s boundary layer and Wasm tests, G6\'s `.wasm` file, G7 and the `wasm-opt` check (step 2.4\'s bindings);\n'
    '    - G6\'s Vite build, G8 and G12 (step 2.5\'s client and server), and the decoder panic lints (step 2.5\'s protocol);\n'
    '    - G10 (step 2.5\'s first recording, after the sim core spike);\n'
    '    - G11 with PM-0002-49 and -50 (step 2.5\'s loopback). G11 is the one that may wait past its result: if the netcode\n'
    '      proof test reports before step 2.5, L1 still has nothing to run against until then.\n')

# --- Part 2: the later change-sets table (H1, ruling b)
rep('| **2.3-S** | the sim core spike\'s checks | the sim core spike\'s report merged (ADR 0004 part 7\'s final list, sim\'s dependency set, R5-LAYOUT-REC4\'s result) | step 2.3\'s own change-set, built right after that report |',
    '| **2.3-S** | the sim core spike\'s checks | the sim core spike\'s report merged (ADR 0004 part 7\'s final list, sim\'s dependency set, R5-LAYOUT-REC4\'s result) | step 2.3\'s own change-set, built right after that report, and merged before any f32 or f64 arithmetic enters `sim` (the advisor agent\'s ruling (b)) |')
rep('| **2.3-E** | the skeleton checks | the netcode proof test\'s and the sim core spike\'s reports merged, and step 2.5\'s code | rides step 2.5\'s change-set (2.5\'s plan may put it in a PR just before) |',
    '| **2.3-E** | the skeleton checks | the netcode proof test\'s and the sim core spike\'s reports merged, and step 2.5\'s code | rides step 2.5\'s change-set (2.5\'s plan may put it in a PR just before) |\n'
    '| **PROOF-57** | the merge-box proofs: PM-0002-57\'s "cannot merge" and D2\'s "single required check" | his second ruleset step, after CS-F | an evidence step with no files: scratch pull requests on branches he allows, read in GitHub\'s merge box, recorded by the next change-set\'s docs pass |')

# --- Part 2 rows (H1, M3)
part2_row('| D1 |', '| D1 | `--prove` passes; every mutation fails exactly its named checks; every control passes | CS-C, for the built checks | each check\'s mutations as it joins | holds on every `--prove` run from CS-C on, for the mutations then built; met in full at slice 5, with PM-0002-52 |')
part2_row('| D2 |', '| D2 | a TypeScript-only pull request runs and passes the single required check | CS-E shows such a pull request passing `required` | "the single required check" holds only after his second ruleset step | **PROOF-57** |')
part2_row('| D7 |', '| D7 | Mustafa names the required check in his branch protection | his ruleset change after CS-E | | **CS-E** (his setting) |\n'
          '| PM-0002-38 (every tool pinned; part 2; outside his range -39 to -62, claimed by no change-set: the old plan gave it to 2.2, whose report does not name it) | `rustc`, clippy, rustfmt, Node, TypeScript and the wasm-bindgen CLI print versions equal to their pins in every job\'s log; the same commit gives the same lint result on two CI runs a week apart | CS-E: each job writes its version lines and the aggregate compares them with the pins; the week-apart pair read a week after CS-E\'s merge, recorded by the next docs pass | the wasm-bindgen line (2.3-B) | 2.3-B |')
part2_row('| PM-0002-42 (G3) |', [l for l in s.split('\n') if l.startswith('| PM-0002-42 (G3) |')][0].replace(
    '| CS-C: all but the generated union and the two editor clauses |',
    '| CS-C: all but the generated union and the two editor clauses; the switch rule is on from CS-C, with an interim plant on a hand-written union that does not count to the mark (the advisor agent\'s ruling (d)) |'))
part2_row('| PM-0002-47 (G9 layers) |', '| PM-0002-47 (G9 layers) | a broken property; a golden beyond tolerance (from slice 1); a changed insta snapshot; a byte swap in one encoder; a page error in deploy; control: an equal-output refactor | CS-C: the control, for `cargo test` and Vitest | the boundary layer (2.3-B); the end-to-end clause (2.3-E); the property, insta and golden clauses with slice 1\'s change-set (the first sim rules, v2 outputs and legacy goldens; a property test or snapshot added earlier joins from its own change-set) | slice 1\'s change-set |')
part2_row('| PM-0002-51 (G12) |', '| PM-0002-51 (G12) | throw in deploy; console error; outbound request; WebSocket broken (from slice 3); +20 % draw calls against ADR 0003\'s baseline; control; the `forceWebGL` run and the backend named | | 2.3-E; WebSocket with slice 3\'s change-set, as the mark says; the draw-call clause with the change-set that commits ADR 0003\'s count baseline (PM-0003-36, its benchmark scene in CI) | the later of slice 3\'s change-set and PM-0003-36\'s |')
part2_row('| PM-0002-52 (`--prove`) |', '| PM-0002-52 (`--prove`) | every mutation of PM-0002-39 to -51; one mutation and one control per id G0-G12; under 30 min | CS-C, for the built ids | as each check joins | slice 5\'s change-set: the last mutations of PM-0002-39 to -51 (the editor\'s, in -42 and -46) join there; one mutation and one control per id G0-G12 from 2.3-E |')
part2_row('| PM-0002-53 (overflow, 4 builds) |', '| PM-0002-53 (overflow, 4 builds) | a planted counter panics in dev and release, native and Wasm; `wrapping_add` agrees in all four; the cost reported | | moved by the advisor agent\'s ruling (a) of this review: the native half rides the first change-set that adds any dependency on `sim`; the Wasm half needs a Wasm runner (wasm-bindgen, at 2.4) | the first change-set with both a crate depending on `sim` and a Wasm runner (expected 2.3-B) |')
part2_row('| PM-0002-57 (one required check) |', '| PM-0002-57 (one required check) | a TypeScript-only pull request passes; a failing, a cancelled and a skipped job each turn it red and block the merge; removing a check fails the aggregate | CS-E, on its own pull request: a failing job (a failing test, no workflow edit) and a cancelled job (cancelled through the API) each turn `required` red; a skipped job and a removed check (each a workflow edit, asked of him with its exact revert) each fail the aggregate | "block the merge", read in GitHub\'s merge box once `required` is in the ruleset, and "passes the single required check" once it is the only one: scratch pull requests after his second ruleset step | **PROOF-57** |')

# --- Part 3 (M7, M8, M2, H1, ruling c)
rep('- **Prompt counts are estimates.** Each count assumes one Write per protected file.\n',
    '- **Prompt counts are estimates.** Each count assumes one Write per protected file.\n'
    '- **`toolchain` read by hand until `required` counts.** Until his first ruleset step makes `required` a required check,\n'
    '  the ruleset and the guard check only `gate` and `smoke`, while `toolchain` alone tests the root manifests, the\n'
    '  lockfiles and the crates. So every change-set before then merges only with `toolchain` green on its head, push and\n'
    '  pull_request, read by the session, as his gos for #48 and #52 asked.\n')
rep('  - K-4: "Models and effort", with his words of 2026-11-12 and of 2026-11-11.\n',
    '  - K-4: "Models and effort", with his words of 2026-11-12 and of 2026-11-11; its effort line exempts the netcode\n'
    '    chat.\n')
rep('  - CI\'s `gate` and `smoke` green.\n',
    '  - CI\'s `gate` and `smoke` green.\n'
    '- **Precondition: a local smoke run.** CS-C1 edits `smoke.mjs`, and the advisor agent\'s day-session seat runs the full\n'
    '  gate, smoke included (`advisor.md`, step 2 and its Never line). So before CS-C1\'s review the session must run the\n'
    '  smoke locally: with the toolkit\'s Chromium build (the allowed `node tools/node_modules/playwright/cli.js install\n'
    '  chromium`), or by 2.1a\'s route of the container\'s Chromium under that build\'s name. Its plan settles which (row 94).\n')
rep('  - the kit lists that name the protected files: K-6b, K-11a and K-11b.\n',
    '  - the kit lists that name the protected files: K-6b, and K-11a to K-11c.\n')
rep('  one of its merge places (`routine-guard.mjs:87`).', '  one of its merge places (`routine-guard.mjs:84`).')
rep('  - PM-0002-57\'s checks: a TypeScript-only scratch pull request; planted failing, cancelled and skipped jobs and a\n'
    '    removed check on CS-E\'s own pull request, each red and blocking, then reverted exactly;\n',
    '  - PM-0002-57\'s and D4\'s checks on CS-E\'s own pull request: a failing job made by a failing test (no workflow edit)\n'
    '    and a cancelled job (cancelled through the API), each turning `required` red; a skipped job and a removed check,\n'
    '    each a workflow edit asked of him with its exact revert, each failing the aggregate. The merge-box clauses and\n'
    '    the TypeScript-only pull request wait for PROOF-57;\n'
    '  - PM-0002-38: each job writes its tools\' version lines for the aggregate to compare with the pins; the same\n'
    '    commit\'s lint result read again a week later;\n'
    '  - the name: the aggregate\'s job id and job name are both exactly `required`, in code font in every kit text (the\n'
    '    advisor agent\'s ruling (c));\n')
rep('- **His asks:** one. **Protected writes:** 3 (`ci.yml` written; `gate.yml` and `toolchain.yml` removed), so about 3\n'
    '  prompts. **His go:** the merge.\n',
    '- **His asks:** two: the workflow (3 protected writes: `ci.yml` written, `gate.yml` and `toolchain.yml` removed), and\n'
    '  the two planted workflow failures with their exact reverts (4 writes). So about 7 prompts. **His go:** the merge.\n')
rep('  - K-12a to K-12h: the lines naming `gate` and `smoke` as the required checks;\n',
    '  - K-12a to K-12h: the lines naming `gate` and `smoke` as the required checks (K-12a, f and g widen "main breaks",\n'
    '    so they need his words: ROLLBACK-SCOPE);\n')
rep('  - the guard\'s tests: no `required` refuses; `required` pending or red refuses; all three green allows;\n',
    '  - the guard\'s tests: no `required` refuses; `required` pending or red refuses; two runs named `required`, one of\n'
    '    them red, refuse (the advisor agent\'s ruling (c)); all three green allows;\n')
rep('- **Then his ruleset step:** remove `gate` and `smoke` from the required checks. They still run inside `required`.\n',
    '- **Then his ruleset step:** remove `gate` and `smoke` from the required checks. They still run inside `required`.\n'
    '- **Then PROOF-57,** the first part\'s closing proof: three scratch pull requests, one TypeScript-only that passes the\n'
    '  single required check, and one with a failing job and one with a cancelled job, each blocked in GitHub\'s merge box.\n'
    '  Its record goes into the next change-set\'s docs pass.\n')
rep('- 6 exact-text asks.\n- About 41 guard prompts by PIN-SCOPE (b): 4 + 2 + 4 + 21 + 3 + 7.\n- 7 merge gos.\n',
    '- 7 exact-text asks (CS-E has two).\n'
    '- About 45 guard prompts by PIN-SCOPE (b): 4 + 2 + 4 + 21 + 7 + 7, and at most one more per change-set for making\n'
    '  its scratch copy (5.1).\n'
    '- 7 merge gos.\n- Then PROOF-57.\n')
rep('- **2.3-B** rides step 2.4: G4, G9\'s boundary layer and Wasm tests, G7 with the budget file, PM-0002-60, the\n'
    '  `wasm-opt` flag clause, the Wasm half of PM-0002-53, and WASM-ARTIFACT.\n',
    '- **2.3-B** rides step 2.4: G4, G9\'s boundary layer and Wasm tests, G7 with the budget file, PM-0002-60, PM-0002-38\'s\n'
    '  wasm-bindgen line, the `wasm-opt` flag clause, PM-0002-53 (its native half rides the first change-set that adds a\n'
    '  dependency on `sim`, by the advisor agent\'s ruling (a)), and WASM-ARTIFACT.\n')
rep('- **2.3-S**, its own change-set after the sim core spike: the float ban, the map types from sim\'s dependencies, the\n'
    '  same-feature check and FEATURE-UNIFY.\n',
    '- **2.3-S**, its own change-set after the sim core spike, merged before any f32 or f64 arithmetic enters `sim`: the\n'
    '  float ban, the map types from sim\'s dependencies, the same-feature check and FEATURE-UNIFY.\n')

# --- Part 4 rows (ruling a, M8, Low)
row_mark(22, '[later] the advisor agent\'s ruling (a) of this review moved the anchor, as its text allows: the native half rides the first change-set that adds any dependency on `sim`; PM-0002-53 is met at the first change-set with both a crate depending on `sim` and a Wasm runner (expected 2.3-B). CS-A rewrites STATUS :95 and the archive\'s anchor text')
row_mark(47, '[later] PM-0002-64, at step 2.5, is its test (the advisor agent\'s reading in this review); recorded by CS-A')
row_mark(94, '[1st] a precondition of CS-C1, which edits `smoke.mjs`: a local smoke run before its review (Part 3). For the other change-sets, CI\'s smoke job stands, as in 2.1b and 2.2')

# --- Part 5 (Low: the scratch copy; M6 table rows)
rep('1. **Prepared away from the main folder.** The session makes the change in a scratch copy under `.gate/`, never in the\n'
    '   main folder. It saves the copy\'s `git diff` as a patch in the change-set\'s notes folder, and pushes it with a\n'
    '   checkpoint save.\n',
    '1. **Prepared away from the main folder.**\n'
    '   - Rules text (CS-B, and the kit lines of CS-D and CS-F): the patch is generated by script from main\'s text and the\n'
    '     after-texts, as `kit-texts.py` does, so no kit file is written before his yes.\n'
    '   - Code (CS-C1, CS-C, CS-D\'s guard, CS-E): developed and tested in a plain copy of the tree made with `git\n'
    '     archive`, never a worktree of this repository and never committed. For builds the copy sits under\n'
    '     `.gate/<change-set>/` (CHECKPOINTS §4); for the guard\'s own files it sits in the session\'s scratchpad, outside\n'
    '     the project, so no step leans on the guard\'s nested-folder gap (W1). The 2.1a precedent: `.gate/fdn-scratch`,\n'
    '     and F1-FIX\'s patch.\n'
    '   - The patch (the copy\'s diff against main) goes into the change-set\'s notes folder and is pushed with a checkpoint\n'
    '     save. Before his yes there is at most one guard prompt per change-set, for making the copy.\n')
m = [l for l in s.split('\n') if l.startswith('| K-11a, b |')]
assert len(m) == 1
rep(m[0], '| K-11a to c | CLAUDE.md :68-69; advisor.md :164-166 and :39-43 | CS-D | the lists of protected and gate files, and escalation category 3\'s, name the gate v2\'s folder and PIN-SCOPE\'s paths | stricter |')
m = [l for l in s.split('\n') if l.startswith('| K-12a to h |')]
assert len(m) == 1
rep(m[0], m[0].replace('| same strength |', '| same strength, except K-12a, f and g, which widen "main breaks" (ROLLBACK-SCOPE) |'))
m = [l for l in s.split('\n') if l.startswith('| K-4 |')]
assert len(m) == 1
rep(m[0], m[0].replace('with two optional lines (A, B) for him to accept or refuse',
                       'with two optional lines (A, B) for him to accept or refuse; its effort line exempts the netcode chat, and is also given under MODELS-HOW (a) and (b)'))

# --- Part 8 (H1, M5)
rep('| CS-B | nothing. Nothing in `spikes/` changes. The new CLAUDE.md keeps the netcode chat\'s models and efforts, by his words and by MODELS-HOW (c), which leaves the agent files\' effort lines alone until it merges |',
    '| CS-B | nothing. Nothing in `spikes/` changes. The new CLAUDE.md keeps the netcode chat\'s models and efforts: K-4 exempts it from the callers\' `max`, and MODELS-HOW (c) leaves the agent files\' effort lines alone until it merges |')
rep('Why this order: at every moment the guard and the ruleset require the same checks, or the guard requires more.\n',
    'Why this order: every merge needs `gate` and `smoke` green from both the ruleset and the guard, throughout. From step 1\n'
    'to CS-F the ruleset also requires `required`, so it is stricter than the guard, which is safe. From CS-F on, both\n'
    'require `required`. Reordering instead, step 2 before CS-F, would leave the rules text naming `gate` and `smoke` as the\n'
    'required checks while the ruleset no longer did.\n\n'
    '**Then PROOF-57** (after step 2): the merge-box proofs of PM-0002-57 and D2, on scratch pull requests.\n')

# --- Part 9 (M2, H1, Low)
rep('   - answer GATE-PATH, PIN-SCOPE, FREEZE-END, MODELS-HOW and ALLOW-GATE2 (ALLOW-GATE2 may wait until CS-F);\n',
    '   - answer GATE-PATH, PIN-SCOPE, FREEZE-END and MODELS-HOW, and ALLOW-GATE2 and ROLLBACK-SCOPE now or at CS-F;\n')
rep('   - **The exact-text ask** (CS-B, C1, D, C, E, F: six): say yes in chat, naming the checkpoint commit\'s SHA.\n',
    '   - **The exact-text asks** (CS-B, C1, D, C, F, and two for CS-E: seven): say yes in chat, naming the checkpoint\n'
    '     commit\'s SHA.\n')
rep('   - **Guard prompts:** answer each, about 4, 2, 4, 21, 3 and 7.\n',
    '   - **Guard prompts:** answer each, about 4, 2, 4, 21, 7 and 7, and at most one per change-set for its scratch copy.\n')
rep('4. **A scratch branch for PM-0002-57\'s TypeScript-only pull request:** let the session push one branch,\n'
    '   `claude/sweet-mayer-grsukw-ts-only`, closed unmerged afterwards. Or open that pull request himself.\n',
    '4. **Three scratch branches for PROOF-57,** after his second ruleset step: let the session push\n'
    '   `claude/sweet-mayer-grsukw-ts-only`, `claude/sweet-mayer-grsukw-fail` and `claude/sweet-mayer-grsukw-cancel`, each\n'
    '   opened as a pull request, read in GitHub\'s merge box and closed unmerged. Or open them himself.\n')
rep('   1. install the v2 toolchain: rustup takes the pin from `rust-toolchain.toml`, Node 22, `npm ci` at the root;\n',
    '   1. install the v2 toolchain: rustup takes the pin from `rust-toolchain.toml`, Node 22, `npm ci --ignore-scripts` at\n'
    '      the root;\n')

# --- Part 10 (H1)
rep('4. **The switch order.** A wrong order could block every merge, which fails closed, or loosen the merge check. Part 8\'s\n'
    '   order keeps the guard equal to or stricter than the ruleset at every step.\n',
    '4. **The switch order.** A wrong order could block every merge, which fails closed, or loosen the merge check. In Part\n'
    '   8\'s order, `gate` and `smoke` stay required by both the ruleset and the guard at every step, the ruleset is at\n'
    '   least as strict as the guard from step 1 on, and the rules text stays true at every merge.\n')

# --- Part 11: the revision log
rep('  - moment 5 comes before the plan is saved as done, after the advisor agent\'s verdict, by his rule 2 of 2026-11-14.\n',
    '  - moment 5 comes before the plan is saved as done, after the advisor agent\'s verdict, by his rule 2 of 2026-11-14.\n'
    '- **Revision 2** (the advisor agent\'s review of revision 1 at 03d05f5: REVISE), what changed:\n'
    '  - H1: PM-0002-57\'s "cannot merge" and D2\'s "single required check" move to PROOF-57, after his second ruleset\n'
    '    step; Part 8\'s and Part 10\'s sentences on the order restated (the ruleset is the stricter one from step 1 to CS-F).\n'
    '  - M2: CS-E\'s planted skipped job and removed check, with their reverts, are a second ask (4 writes); its failing\n'
    '    and cancelled jobs use a failing test and an API cancel; the totals become 7 asks and about 45 prompts.\n'
    '  - M3: Part 2\'s cells: D1 and PM-0002-52 met at slice 5; PM-0002-47, -51 and -53 name their change-sets;\n'
    '    PM-0002-38 placed (CS-E, its wasm-bindgen line at 2.3-B); the summary adds -44, -51 and -52.\n'
    '  - M4: Rule 3 is the plan\'s choice under his "may", with its list; G11 may wait past its result.\n'
    '  - M5: K-4\'s effort line exempts the netcode chat; its effort line under MODELS-HOW (a) and (b) added; [to verify]\n'
    '    covers `model: "fable"`, "on his personal account" added.\n'
    '  - M6: K-11c (escalation category 3\'s gate files); K-12a, f and g labelled a widening (ROLLBACK-SCOPE, a new\n'
    '    decision, at CS-F).\n'
    '  - M7: until `required` counts, each change-set reads `toolchain`\'s runs by hand.\n'
    '  - M8: row 94 is a precondition of CS-C1 (a local smoke run), not dropped.\n'
    '  - Lows: row 47 (PM-0002-64 is OQ-0002-8\'s test, its reading); `routine-guard.mjs:84`; `npm ci --ignore-scripts`;\n'
    '    5.1 says how the scratch copy is made and counts its prompt; the checkpoint\'s consultation count.\n'
    '  - Rulings folded: (a) OVERFLOW-XCRATE-CI\'s anchor moved (row 22, PM-0002-53); (b) the float ban, with its condition\n'
    '    on 2.3-S; (c) `required`, with its three conditions (CS-E, CS-F); (d) the interim union plant (PM-0002-42).\n'
    '  - Decisions: GATE-PATH (a)\'s import cost; PIN-SCOPE (b)\'s unprotected pins; MODELS-HOW quotes DECISIONS.md :91.\n')

# --- Decisions (rulings on the five; ROLLBACK-SCOPE)
rep('    - (a) its own folder beside the legacy gate, `tools/gate-v2/` (run as `node tools/gate-v2/gate.mjs`);\n',
    '    - (a) its own folder beside the legacy gate, `tools/gate-v2/` (run as `node tools/gate-v2/gate.mjs`). One more cost:\n'
    '      a script there that imports a package by its bare name finds `tools/node_modules` first (acorn, eslint-scope,\n'
    '      Playwright 1.56.1), not the root\'s, so the gate v2 imports no package by bare name and runs the root\'s tools by\n'
    '      their path;\n')
rep('      CI (`--locked`, G5\'s bans and sources, exact versions), with review and with his go at merge;\n',
    '      CI (`--locked`, G5\'s bans and sources, exact versions), with review and with his go at merge. It leaves\n'
    '      unprotected the gate v2\'s own npm tool pins (the root `package.json` and lockfile, which hold ESLint, Prettier,\n'
    '      TypeScript and Vitest) and `Cargo.lock`, while the legacy gate\'s `tools/package*.json` are gate files today;\n')
rep('  - Why it matters: his words also say "The netcode chat on my university account keeps its models and efforts as they\n'
    '    are", and it takes main\'s agent files when it merges main.\n',
    '  - Why it matters: his words also say "The netcode chat on my university account keeps its models and efforts as they\n'
    '    are", and it takes main\'s agent files when it merges main. And they say "Step 2.3 changes CLAUDE.md\'s "Models and\n'
    '    effort" section and the agent files to match" (DECISIONS.md :91), which (c) meets for the agent files only after\n'
    '    PR #40 merges.\n')
rep('  - Recommendation: (c). It keeps both of his sentences true.\n',
    '  - Recommendation: (c). It keeps the netcode chat\'s efforts exactly as his words say, at the cost of the agent\n'
    '    files\' part of step 2.3 coming after #40. (a) keeps step 2.3 whole, but leaves the netcode chat to pass its old\n'
    '    efforts itself.\n')
rep('  - He can answer it now or at CS-F.\n',
    '  - He can answer it now or at CS-F.\n'
    '- **ROLLBACK-SCOPE: should a red gate v2 job on main count as "main breaks"?** "Main breaks" starts a revert with no\n'
    '  plan step (CLAUDE.md :119-122; `advisor.md`, "Revert"). Today it means `gate` or `smoke` red on main.\n'
    '  - Why it matters: K-12a, f and g, as drafted, make it "`required` red on main", which counts every gate v2 job. That\n'
    '    is more cases where a day session reverts first: a widening his words must allow (Part 4 row 56, F5, has the same\n'
    '    shape).\n'
    '  - Options:\n'
    '    - (a) yes: `required` red on main, after one re-run and pinned to one merge, is "main breaks";\n'
    '    - (b) no: "main breaks" stays `gate` or `smoke` red, and a red gate v2 job on main gets a fix forward, plan first.\n'
    '  - Recommendation: (a). After CS-F, `required` is the whole gate, and a red gate on main is what the rule is for.\n'
    '  - He can answer it now or at CS-F.\n')
rep('1. Answer GATE-PATH, PIN-SCOPE, FREEZE-END and MODELS-HOW, and ALLOW-GATE2 if he wishes now.\n',
    '1. Answer GATE-PATH, PIN-SCOPE, FREEZE-END and MODELS-HOW, and ALLOW-GATE2 and ROLLBACK-SCOPE if he wishes now.\n')

# --- Appendix K: replace with the regenerated kit texts
i = s.index('\n## Appendix K. The exact kit texts')
head_end = s.index('in its change-set (5.1).', i) + len('in its change-set (5.1).')
head = s[i:head_end]
k = open(BASE + 'kit-texts.md', encoding='utf-8').read().split('\n')
assert k[0].startswith('# Kit texts for the step-2.3 plan')
body = [('### ' + l[3:]) if l.startswith('## K-') else l for l in k[1:]]
s = s[:i] + head + '\n'.join(body)

open(P, 'w', encoding='utf-8').write(s)
rows = [l for l in s[s.index('## Part 4.'):s.index('## Part 5.')].split('\n') if re.match(r'\| \d+ \|', l)]
tags = {}
for r in rows:
    t = re.search(r'\| \[(1st|later|dropped|not ours|chat|ruling)\]', r)
    t = t.group(1) if t else 'NONE'
    tags[t] = tags.get(t, 0) + 1
print('revision 2 written;', len(rows), 'rows;', tags)
