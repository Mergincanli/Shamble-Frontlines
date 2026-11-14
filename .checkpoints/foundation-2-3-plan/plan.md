# Plan: foundation step 2.3, the gate v2 and CI, and the end of the kit's freeze (revision 2)

- **The task:** Mustafa's words of 2026-11-13 (`docs/DECISIONS.md`, the entry "Taken by Mustafa, 2026-11-13, 18:47:55
  UTC", from "Then, after the docs pass is merged: plan step 2.3." to its end: items 1 to 9), and his message of
  2026-11-14 that starts this plan (three additions to the lists; this plan gets the advisor agent's review and the
  advisor tool at its five moments, no review rounds and no verifier). His two rules of 2026-11-14 (merge commit
  4460ef5's message, items 2 and 3) apply.
- **A plan only.** Nothing here is built, and nothing outside this checkpoint is edited. Every kit, gate and CI edit
  below is a proposal. Each is asked of him in chat with its exact text and written only after his yes, with the chat
  in Accept edits (his words of 2026-11-13).
- **Base:** main 4460ef5 (PR #52's merge, 2026-11-14 09:06:30 UTC). **Branch:** `claude/sweet-mayer-grsukw`, which
  stands in for `cs/foundation-2-3-plan`. It was moved to main by a fast-forward and pushed.
- **Revision 2** folds in the advisor agent's review of revision 1 (03d05f5: REVISE, one High, seven Medium, five Low,
  and its four rulings). Part 11 lists what changed.
- **Sources in this folder:**
  - `reading.md`: blocks B1 to B13, each source with its file and line;
  - `approach.md`: the sketch the advisor tool checked;
  - `kit-texts.py` and its output, `kit-texts.md`: every exact "before" read from main by line number, and every quote
    of his checked against `docs/DECISIONS.md`. The output is Appendix K.
- **Codes:**
  - D1 to D7 are the done-when lines of ADR 0002's migration step 2.3.
  - PM-0002-nn are ADR 0002's pass marks.
  - G0 to G13 are the gate v2's checks (ADR 0002 part 9).
  - K-nn are kit edits (Part 5), and G-n are guard changes (Part 6).
  - Other capitalised names are follow-up ids, explained where they first appear in Part 4.

## In one screen
- **The cut.** Step 2.3 splits in two. Its first part is seven change-sets. They are built after his go on this plan,
  before the proof tests report, one pull request at a time, in this order:
  1. **CS-A, docs** (decision docs): STATUS and ROADMAP made true for main 4460ef5; his words of 2026-11-14 copied into
     `docs/DECISIONS.md` by script; his go on this plan, with his answers.
  2. **CS-B, rules text** (CLAUDE.md, three agent files, CHECKPOINTS.md): his model rule of 2026-11-12, his
     advisor-tool words of 2026-11-11, the three lines that name 2.1 and 2.2 as the only exception, and six small
     fixes (K-6 to K-10, K-14).
  3. **CS-C1, the legacy gate:** six findings in `tools/gate.mjs` and `tools/smoke.mjs`.
  4. **CS-D, the guard:**
     - it protects the gate v2's folder and the pins he chooses (PIN-SCOPE);
     - it refuses kit and gate edits in Auto mode, where today it only asks (AUTO-ASK).
  5. **CS-C, the gate v2's first part:**
     - checks G0, G1, G2, G3, G5, G6 and G9, each in part;
     - `--prove`, the run that plants defects to show each check catches its own.
  6. **CS-E, CI:**
     - one workflow, whose final job `required` is the one check he requires;
     - it holds the legacy gate and smoke jobs too;
     - the network cut stays around the tests, hardened.
  7. **CS-F, the switch:** the guard and the rules name `required`. He adds it to his required checks between CS-E and
     CS-F, and remove `gate` and `smoke` after CS-F.
- **The rest of step 2.3 joins later,** each piece when its result is in, with its pass marks unchanged:
  - **2.3-B** rides step 2.4, after the boundary spike: G4, the boundary layer of G9, G7 and the budget file.
  - **2.3-S**, its own change-set after the sim core spike: G2's float ban and G5's same-feature check.
  - **2.3-E** rides step 2.5, after the netcode proof test and the sim core spike: G6's Vite build, G8, G10, G11, G12.
  - **G13** rides slice 2, as ADR 0002 already places it.
- **Marks, read strictly** (every clause holds):
  - By CS-E, the first part fully meets PM-0002-40 (with his one Windows run), PM-0002-54, and D4, D6 and D7.
    PM-0002-57 and D2 are met at its closing proof, after his second ruleset step (PROOF-57).
  - Every other mark is met when its last clause joins, at a named change-set. Its built clauses run from CS-C on.
  - No mark is dropped or loosened (Part 2).
  - Several marks name things that arrive only after the foundation: `server` and `navbake` (slices 3 and 2), the
    editor (slice 5), legacy goldens (slice 1), ADR 0003's count baseline. Those clauses join there, so PM-0002-41,
    -42, -44, -46, -47, -51 and -52 are met only then.
- **The waiting items** (Part 4, 107 rows in one table, one per item and source, counted by script):
  - 50 are carried by the first part.
  - 48 come later, each with its time.
  - 5 are dropped, each with its reason.
  - 2 are not a session's edit, and 2 are rules for this chat.
  - Leaning to the smallest step, as he asked: the old kit follow-ups go to a kit queue after 2.3. GUARD-SEC comes
    first, with PX-1 (the merge check that trusts CI results by their name alone) and W1 to W3 (gaps in what the
    guard lets routines and agents write).
- **His part in the first part** (Part 9):
  - his go on this plan;
  - seven exact-text asks (CS-B, C1, D, C, F, and two for CS-E: its workflow, and its two planted workflow failures
    with their reverts). Each is a patch at a pushed checkpoint commit, approved by naming the commit's SHA and
    written in Accept edits, with about 2 to 21 guard prompts each, about 45 in all;
  - seven merge gos;
  - two ruleset edits on GitHub;
  - three scratch branches, for PROOF-57's pull requests;
  - one Windows run.
  Each change-set still gets its own plan, the advisor agent's plan review, review rounds within the budget, the
  verifier, and the advisor agent's review of its final head.
- **Decisions needed:**
  - GATE-PATH: where the gate v2's script lives (item 7);
  - PIN-SCOPE: which v2 pins and configs the guard protects;
  - FREEZE-END: when KIT-FREEZE-2 ends;
  - MODELS-HOW: effort max set in the agent files now, or after PR #40;
  - ALLOW-GATE2: an allow rule for the gate v2's command;
  - ROLLBACK-SCOPE: whether a red gate v2 job on main counts as "main breaks" (may wait until CS-F).
- **#52's leftovers are not on main 4460ef5:**
  - STATUS :5, :13, :29, :36 and :43, and ROADMAP :117, still show #52 open;
  - `docs/DECISIONS.md` has no entry of 2026-11-14;
  - they are CS-A's, unless a docs chat on main carries them first.
- **PR #40** has nothing to do now. It merges main before its own merge, and before his ruleset's `required` matters
  to it (Part 8).

## Part 0. Method and base
- **Today's main against the old plan's part 3** (26e0d82, `.checkpoints/foundation-restructure/plan.md:442-452`):
  - **Done since:**
    - "2.2 merged": PR #48, 4d21c1a;
    - "PR #46 merged": 2d22b69;
    - the gate v2's path is decided in this plan (item 7).
  - **Still waiting:**
    - the netcode proof test (PR #40: a draft, head 4bd5ac3, 2026-11-14 08:31:05 UTC, in its U4r fix pass, behind and
      conflicting with main);
    - the sim core spike and the boundary spike, neither started (`docs/SPIKE_PLAN.md`: one heavy spike at a time);
    - his branch-protection setting.
  - **New:** his decision of 2026-11-13 builds the parts that need no result now.
  - **Nothing of the gate v2 exists on main.** CI runs the legacy gate (`gate.yml`: jobs `gate` and `smoke`) and 2.2's
    `toolchain.yml`. The latter runs the pin check, `cargo test`, the release overflow tests, the wasm32 build,
    `npm test --workspaces` and `tsc --version`, inside a network cut, and no clippy, rustfmt, ESLint or `tsc -b`.
- **How "needs a result" is read** (his rule: "Every gate check that needs a result joins later, when that result is
  in"):
  - **Rule 1, the spike's content.** A check needs a result when a spike that has not reported decides its content: a
    tool, a list or a number.
  - **Rule 2, a re-proof.** A check also needs a result when it re-proves a spike's mark. SPIKE_PLAN's foundation entry
    (`docs/SPIKE_PLAN.md:465-467`) names these marks: R5-LAYOUT-REC4 and R5-TEST-REC2's tools from the sim core spike,
    and R5-TEST-REC4's L1 from the netcode proof test.
  - **Rule 3, this plan's choice under his "may".** His decision says the parts that need no result "may be built"
    now; it does not say they must. A check whose subject arrives only with a later step waits for that step, even
    when its result is in earlier, because a check with nothing to check would be a feature "for later", which
    CLAUDE.md rules out. The checks this choice holds back:
    - G4 and the generated-union mutation (step 2.4's generators);
    - G9's boundary layer and Wasm tests, G6's `.wasm` file, G7 and the `wasm-opt` check (step 2.4's bindings);
    - G6's Vite build, G8 and G12 (step 2.5's client and server), and the decoder panic lints (step 2.5's protocol);
    - G10 (step 2.5's first recording, after the sim core spike);
    - G11 with PM-0002-49 and -50 (step 2.5's loopback). G11 is the one that may wait past its result: if the netcode
      proof test reports before step 2.5, L1 still has nothing to run against until then.
  - **The three facts this reading rests on** (`reading.md` B13):
    - F-FLOAT: the sim core spike decides R1-DET-OQ10, whether Wasm's `sqrt` is exact, and `sqrt` is on ADR 0004 part
      7's float allow-list. So the float ban waits. The hash-map and clock bans rest on no open question.
    - F-HOOK: the hook input carries `permission_mode`, values `default`, `plan`, `acceptEdits`, `auto`, `dontAsk` and
      `bypassPermissions`.
    - F-PR: a `pull_request` run uses the workflow of the merge commit, and none runs while a PR conflicts with its
      base.
    - F-HOOK and F-PR are two read-only docs agents' readings of the official pages, quoted in the notes and not
      re-fetched by this session.
- **"Met"** means every clause in the mark's row holds: its controls, its machines and its run counts. A mark whose
  clauses are partly built is "met at" the change-set where its last clause joins.
- **The advisor tool:** consultation 1 at 2026-11-14 11:00:19 UTC (the transcript) changed this plan's shape. Its line
  is in the checkpoint, and the later consultations will be there too.

## Part 1. Check by check, G0 to G13 (item 1)
On main today, no check below exists as a gate v2 check. The second column says what CI already runs of it.

| Check | On main 4460ef5 | First part (built after his go) | Waits for, and joins in |
|---|---|---|---|
| G0 setup | `toolchain.yml`'s pin check (exit 1, not 2) and fetch steps | CS-C: the pinned toolchain, with the source checks: `RUSTUP_TOOLCHAIN` unset, the active toolchain named by the file, `RUSTUP_AUTO_INSTALL=0`; Node equals the pin; `cargo --locked` and `npm ci`; exit 2 on any setup failure; the G0 finding's mutations | the missing-browser mutation joins with G12 (2.3-E): no built check uses a browser before then |
| G1 format | none | CS-C: `cargo fmt --all --check` with `rustfmt.toml` (stable options, part 7); `prettier --check` on `packages/**` with `.prettierrc.json` and `.prettierignore` | nothing |
| G2 Rust lint | none (2.2 ran clippy locally only) | CS-C: clippy `-D warnings` on the host and on wasm32; every crate opts into the workspace lints; `unsafe_code` deny; part 4's build-flag check; the hash-map ban with `iter_over_hash_type`, and the clock ban, in each `crates/*/clippy.toml` where part 7 puts them; pedantic on `sim` and `protocol` through the gate's command line (LINT-TABLE-PEDANTIC) | three parts: the float ban, and the map types from sim's dependencies, after the sim core spike (2.3-S: R1-DET-OQ10 and PM-0004-22 decide the list); the decoder panic lints with the first decoder (2.3-E, step 2.5's protocol messages); the `wasm-opt` flag check with the `wasm-opt` step (2.3-B) |
| G3 TypeScript | `tsc --version` only | CS-C: `tsconfig.base.json` with part 7's strict flags; per-package `tsconfig.json` as project references, built with `tsc -b`; ESLint 10 typed lint, `--max-warnings 0`, with part 7's four project rules and `no-eval`/`no-new-func` (TSCONFIG) | the generated-union switch rule's mutation with generated types (2.3-B); the editor import rules with the editor package (slice 5) |
| G4 boundary | none | none | the boundary spike's item 4 and step 2.4's generators: 2.3-B |
| G5 dependency rules | none | CS-C: cargo-deny's bans for part 3's rules, its `sources` at deny (crates.io only), `wildcards` and `multiple-versions` at deny with a reviewed skip list; registry crates named like ours banned (CRATE-NAMES); exact npm versions (CI-ENFORCED-LATER) | the same-feature check (and FEATURE-UNIFY) after the sim core spike: it re-proves R5-LAYOUT-REC4 (2.3-S) |
| G6 build | `toolchain.yml` builds five crates for wasm32 | CS-C: the native release build of `devtools`; the wasm32 library builds | the `.wasm` file itself after `sim-wasm`'s bindings (WASM-ARTIFACT, 2.3-B); the Vite build of the player entry (2.3-E); the editor entry (slice 5) |
| G7 budgets | none | none | the boundary spike's size measurements, which give the starting values ("not guesses", part 9), and the first artifact: 2.3-B; the JavaScript bytes per entry from 2.5 |
| G8 player build | none | none | the Vite build (2.3-E); its editor clauses (slice 5) |
| G9 tests | `cargo test --workspace`, `npm test --workspaces` | CS-C: `cargo test --workspace`; Vitest in every package, run by its path, never through a package script | proptest's fixed seed with the first property test; legacy goldens (slice 1); insta with the first snapshot; Wasm tests in Node and the boundary layer (2.3-B) |
| G10 determinism | none | none | the sim core spike's replay runner and state hash, and step 2.5's first recording: 2.3-E |
| G11 network | none | none | the netcode proof test's L1, and step 2.5's loopback, which puts L1 behind it: 2.3-E |
| G12 end-to-end | `e2e`'s stub spec (no browser) | none | step 2.5's built client and local server: 2.3-E; the draw-call clause with ADR 0003's committed count baseline |
| G13 maps | none | none | slice 2's first committed map, as ADR 0002 places it |
| `--prove` | none | CS-C: at least one mutation and one control per built check, each in a throwaway copy under `.gate/` | grows as each check joins |
| the budget file | none | none | with G7 (2.3-B): today it would hold no row with a value |
| the workflow and its one required check | `gate.yml` (`gate`, `smoke`) and `toolchain.yml`; the ruleset requires `gate` and `smoke` | CS-E: `required`, the aggregate job; his ruleset change; CS-F | nothing |

## Part 2. Every done-when line and pass mark of step 2.3 (item 2)
The later change-sets named below:

| Name | Holds | Trigger | Kind |
|---|---|---|---|
| **2.3-B** | the boundary checks | the boundary spike's report merged (its items 4 and 8) | rides step 2.4's change-set (one PR, or a PR right before it, as 2.4's plan says), because G4 checks what 2.4 generates |
| **2.3-S** | the sim core spike's checks | the sim core spike's report merged (ADR 0004 part 7's final list, sim's dependency set, R5-LAYOUT-REC4's result) | step 2.3's own change-set, built right after that report, and merged before any f32 or f64 arithmetic enters `sim` (the advisor agent's ruling (b)) |
| **2.3-E** | the skeleton checks | the netcode proof test's and the sim core spike's reports merged, and step 2.5's code | rides step 2.5's change-set (2.5's plan may put it in a PR just before) |
| **PROOF-57** | the merge-box proofs: PM-0002-57's "cannot merge" and D2's "single required check" | his second ruleset step, after CS-F | an evidence step with no files: scratch pull requests on branches he allows, read in GitHub's merge box, recorded by the next change-set's docs pass |

Clauses that name a crate or a package that enters later join with the change-set that brings it, as the marks say ("from
slice 1", "each when that crate enters") or as their nouns require: `server` at slice 3, `navbake` at slice 2, the
editor at slice 5. Run counts and machines are part of each mark.

| Line or mark | Its clauses | First part | Joins later | Met at |
|---|---|---|---|---|
| D1 | `--prove` passes; every mutation fails exactly its named checks; every control passes | CS-C, for the built checks | each check's mutations as it joins | holds on every `--prove` run from CS-C on, for the mutations then built; met in full at slice 5, with PM-0002-52 |
| D2 | a TypeScript-only pull request runs and passes the single required check | CS-E shows such a pull request passing `required` | "the single required check" holds only after his second ruleset step | **PROOF-57** |
| D3 | warm pull-request wall time at most 10 min at p50 over 5 runs; at most 20 min cold | CS-E measures and reports it | the full workflow | 2.3-E, as PM-0002-56 |
| D4 | a failing and a cancelled job each turn the required check red | CS-E | | **CS-E** |
| D5 | a fork pull request runs every job with no secret | | step 2.6, where the first secret enters (PM-0002-58 covers part 10 too) | step 2.6 |
| D6 | the repository's Rust cache at or below 8 GB | CS-E (3 runs) | re-measured at 2.3-E | **CS-E** |
| D7 | Mustafa names the required check in his branch protection | his ruleset change after CS-E | | **CS-E** (his setting) |
| PM-0002-38 (every tool pinned; part 2; outside his range -39 to -62, claimed by no change-set: the old plan gave it to 2.2, whose report does not name it) | `rustc`, clippy, rustfmt, Node, TypeScript and the wasm-bindgen CLI print versions equal to their pins in every job's log; the same commit gives the same lint result on two CI runs a week apart | CS-E: each job writes its version lines and the aggregate compares them with the pins; the week-apart pair read a week after CS-E's merge, recorded by the next docs pass | the wasm-bindgen line (2.3-B) | 2.3-B |
| PM-0002-39 (G0) | toolchain file deleted: G0 exit 2, no other Rust; `Cargo.toml` edited without `Cargo.lock` fails at the first cargo step; missing Playwright browser: exit 2 with the setup command; control: a consistent toolchain bump passes | CS-C: the first, second and fourth, plus the G0 finding's source-check mutations | the missing-browser clause (2.3-E) | 2.3-E |
| PM-0002-40 (G1) | misformatted `.rs`, CRLF `.rs`, misformatted `.ts` each fail G1 only; control: long string literals pass; after `prettier --write`, ESLint reports 0; the formatted tree is byte-identical on Windows and Linux (once, his PC) | CS-C, and his Windows run (Part 9) | | **CS-C** + his run |
| PM-0002-41 (G2) | the planted defects ADR 0002 lists at :1307, each failing G2 only with its reason; controls in `server`, in a `#[test]`, and sim's min/max/clamp helpers with `abs` and `fract` | CS-C: HashMap field and `map.values().sum()` in sim; `Instant::now()` in a tick function, in room and in client-endpoint; a correctness bug; a style warning; `unsafe`; a crate without `[lints] workspace = true`; `+fma`, `-C target-cpu=native`, `relaxed-simd` in the build flags; control `.unwrap()` in a `#[test]` | `sin`, `powi`, `to_radians`, `min`, and the helper controls (2.3-S); `.unwrap()` and `buf[i]` in a decoder (2.3-E); `-ffm` in `wasm-opt` (2.3-B); the `server` control (slice 3) | slice 3 |
| PM-0002-42 (G3) | one violation per strict flag; `eval` and `new Function` in client; an unawaited promise; a switch over a generated union missing one kind; an import of editor from client; a bare `event`; DOM code imported by sim-worker or net; an import from `legacy/` or `spikes/`; controls: `import type` from types in editor passes; the clean tree is 0 and 0 | CS-C: all but the generated union and the two editor clauses; the switch rule is on from CS-C, with an interim plant on a hand-written union that does not count to the mark (the advisor agent's ruling (d)) | generated union (2.3-B); the editor clauses (slice 5) | slice 5 |
| PM-0002-43 (G4) | all | | 2.3-B | 2.3-B |
| PM-0002-44 (G5) | transport crate or `tokio`/`quinn` in sim; `tokio` in room; `navbake` or `server` reachable from sim-wasm (each when that crate enters); wasm-bindgen or a transport crate in client-endpoint; a git dependency; a wildcard; a second version off the skip list; a server-only feature on a shared maths dependency (feature lists differ across server, devtools, navbake, sim-wasm); controls: proptest as sim's dev-dependency, a dev-only feature on devtools | CS-C: all but the feature comparison and the two crates that enter later; how each dependency mutation keeps `Cargo.lock` consistent offline is CS-C's design (local stand-in crates by name, or the network in CI's `--prove` job) | the feature comparison (2.3-S); navbake (slice 2) and server (slice 3), as the mark says | 2.3-S, and its own per-crate clauses as those crates enter |
| PM-0002-45 (G7) | all | | 2.3-B | 2.3-B |
| PM-0002-46 (G8) | all | | 2.3-E; the editor's clauses (slice 5) | slice 5 |
| PM-0002-47 (G9 layers) | a broken property; a golden beyond tolerance (from slice 1); a changed insta snapshot; a byte swap in one encoder; a page error in deploy; control: an equal-output refactor | CS-C: the control, for `cargo test` and Vitest | the boundary layer (2.3-B); the end-to-end clause (2.3-E); the property, insta and golden clauses with slice 1's change-set (the first sim rules, v2 outputs and legacy goldens; a property test or snapshot added earlier joins from its own change-set) | slice 1's change-set |
| PM-0002-48 (G10) | three planted divergences reported at the first tick; a control; G10 under 3 min over 5 runs | | 2.3-E | 2.3-E |
| PM-0002-49 (L1) | the same seed gives an identical fate log and hashes twice; over 1,000,000 packets per profile, loss, delay and jitter within 5 %; G11 under 2 min | | 2.3-E (L1 enters behind the loopback at 2.5) | 2.3-E |
| PM-0002-50 (G11) | all | | 2.3-E | 2.3-E |
| PM-0002-51 (G12) | throw in deploy; console error; outbound request; WebSocket broken (from slice 3); +20 % draw calls against ADR 0003's baseline; control; the `forceWebGL` run and the backend named | | 2.3-E; WebSocket with slice 3's change-set, as the mark says; the draw-call clause with the change-set that commits ADR 0003's count baseline (PM-0003-36, its benchmark scene in CI) | the later of slice 3's change-set and PM-0003-36's |
| PM-0002-52 (`--prove`) | every mutation of PM-0002-39 to -51; one mutation and one control per id G0-G12; under 30 min | CS-C, for the built ids | as each check joins | slice 5's change-set: the last mutations of PM-0002-39 to -51 (the editor's, in -42 and -46) join there; one mutation and one control per id G0-G12 from 2.3-E |
| PM-0002-53 (overflow, 4 builds) | a planted counter panics in dev and release, native and Wasm; `wrapping_add` agrees in all four; the cost reported | | moved by the advisor agent's ruling (a) of this review: the native half rides the first change-set that adds any dependency on `sim`; the Wasm half needs a Wasm runner (wasm-bindgen, at 2.4) | the first change-set with both a crate depending on `sim` and a Wasm runner (expected 2.3-B) |
| PM-0002-54 (lint and type speed) | `tsc -b` under 30 s, typed lint under 60 s, clippy under 30 s warm and 3 min cold; 5 runs each | CS-C and CS-E (5 runs each, from the job logs) | | **CS-E** |
| PM-0002-55 (gate speed) | `--quick` under 60 s warm; the full gate under 10 min warm, on Linux and on Windows; 5 runs per system | measured and reported at CS-C (not the full gate yet) | the full gate (2.3-E) and his Windows runs | 2.3-E |
| PM-0002-56 (CI speed) | as D3, per job and in total | measured and reported at CS-E | the full workflow | 2.3-E |
| PM-0002-57 (one required check) | a TypeScript-only pull request passes; a failing, a cancelled and a skipped job each turn it red and block the merge; removing a check fails the aggregate | CS-E, on its own pull request: a failing job (a failing test, no workflow edit) and a cancelled job (cancelled through the API) each turn `required` red; a skipped job and a removed check (each a workflow edit, asked of him with its exact revert) each fail the aggregate | "block the merge", read in GitHub's merge box once `required` is in the ruleset, and "passes the single required check" once it is the only one: scratch pull requests after his second ruleset step | **PROOF-57** |
| PM-0002-58 (forks) | a fork pull request runs every job with no secret; its deploy job skipped | | step 2.6 (and a test account, his) | step 2.6 |
| PM-0002-59 (caches) | warm: no dependency crate recompiled; total Actions cache at or below 8 GB with every v2 job cached; warm against cold reported; 3 runs each | measured at CS-E | every v2 job (2.3-E) | 2.3-E |
| PM-0002-60 (wasm-bindgen install) | exactly the lock's version, under 30 s, 3 runs | | 2.3-B | 2.3-B |
| PM-0002-61 (no flake) | 20 identical runs of the whole suite; a planted timing test caught; a retry-only test fails Playwright | CS-E: the 20 runs of what is built, and the timing plant (Vitest) | the full suite and the Playwright clause (2.3-E) | 2.3-E |
| PM-0002-62 (per-edit hook) | the hook reports a type error within 10 s at p95 over 20 edits, on his PC and on Linux | | ADR 0002 places it in its own kit change-set in phase 1, "not in the foundation" (unchanged) | that kit change-set |

## Part 3. The cut into change-sets (item 3)

### The rules every change-set follows
- **Sequence.** Each change-set merges on its own, in this order, one pull request at a time from this branch. Each
  starts only when the one before has merged, because CLAUDE.md's protocol step 1 says one change-set at a time.
- **The protocol, every time.** Each one gets:
  - its own plan, and the advisor agent's plan review of it;
  - the build, and the gate after every step;
  - review rounds within the budget;
  - the verifier until PASS;
  - the advisor agent's review of its final head;
  - ready, then his go.
- **Rule and guard edits stay apart from the gate v2's code** (his item 3): CS-B and CS-F hold rules text, CS-D holds
  guard code, CS-C holds gate code, CS-E holds CI.
- **Prompt counts are estimates.** Each count assumes one Write per protected file.
- **`toolchain` read by hand until `required` counts.** Until his first ruleset step makes `required` a required check,
  the ruleset and the guard check only `gate` and `smoke`, while `toolchain` alone tests the root manifests, the
  lockfiles and the crates. So every change-set before then merges only with `toolchain` green on its head, push and
  pull_request, read by the session, as his gos for #48 and #52 asked.

### CS-A: the docs after #52, and this plan's record (risk tier 1: decision docs; needs-mustafa)
- **Files:**
  - `docs/STATUS.md`, `docs/ROADMAP.md`, `docs/STATUS_ARCHIVE.md` ("In flight") and `docs/DECISIONS.md`;
  - a report, `docs/reports/2026-11-<dd>-foundation-2-3-plan.md`. It folds in this plan, so the plan reaches main
    rather than living only at a checkpoint commit.
- **Content:**
  - STATUS and ROADMAP made true for main 4460ef5: #52 merged, step 2.3's plan given.
  - His words of 2026-11-14, copied into DECISIONS.md by script from this session's transcript, each byte for byte:
    the go for #52 with his rules 2 and 3 (they now stand only in the merge message), today's message starting the plan,
    his go on this plan, and his answers to its decisions.
- **If a docs chat on main carries #52's leftovers first** (his note), CS-A keeps only the plan's record.
- **Reviews:** one round of two reviewers on the final diff, the verifier (his words byte for byte), and the advisor
  agent.
- **His asks:** none (no kit file). **His go:** the merge.

### CS-B: the rules text (risk tier 2; CLAUDE.md, three agent files, `.claude/CHECKPOINTS.md`; no code)
- **Edits** (exact texts in Appendix K):
  - K-1, K-2, K-3: the three exception lines, now carrying his decision of 2026-11-13 with its limits.
  - K-4: "Models and effort", with his words of 2026-11-12 and of 2026-11-11; its effort line exempts the netcode
    chat.
  - K-6: MANUAL-MODE. K-7: GIT-C. K-8: L-4. K-9: SD-N2.
  - K-10a to K-10c: status-diet's citations.
  - K-14: the agents' paste block names the shell forms the guard refuses (PROMPTS-2.3's route).
- **Not here:**
  - The lines naming `gate` and `smoke` as the required checks: they change with the switch (CS-F), so the rules stay
    true at every merge.
  - SD-N1: it changes what counts as his go, so it is its own ask (Part 4).
  - The agent files' effort lines: by MODELS-HOW, not before PR #40 merges.
- **Evidence:**
  - `kit-texts.py`, re-run against the written files: each equals the text he approved, byte for byte, and each quote of
    his is checked.
  - A grep: no line still names 2.1 and 2.2 as the only exception.
  - The guard's tests, unchanged and equal on the branch and on main.
- **Reviews:** security-auditor and the verifier, then the advisor agent.
- **His asks:** one exact-text ask. **Protected writes:** 4 files (CLAUDE.md, advisor.md, netcode-architect.md,
  CHECKPOINTS.md), so about 4 guard prompts. **His go:** the merge.

### CS-C1: the legacy gate's findings (risk tier 2: gate files `tools/gate.mjs`, `tools/smoke.mjs`)
- **Items:**
  - **F4** (2.1b's security-auditor): refuse page and engine names, in any letter case, on every run. It checks only the
    root and the old `shamble-frontlines-repo/` folder, so pages in `spikes/` and v2's `packages/` stay legal. The CS-C1
    plan first lists PR #40's `.html` and `shamble-*.js` names.
  - **IE-1:** `smoke.mjs:31`'s wording.
  - **TE-2:** a base holding both full sets exits 2, as a working tree in that state already does.
  - **TE-7:** test rows for `editor.html` alone, and for a letter-case variant of an engine file.
  - **R2-1:** names with trailing dots or spaces.
  - **TOOLKIT-RESOLVES-UP:** `smoke.mjs` uses Playwright only from `tools/node_modules`.
  - **A-1, N2, and the v2-root-page limit:** recorded as known limits in `tools/README.md`, which is not a gate file.
- **Evidence:**
  - Phase 0's mutation proofs re-run (31 of 31, as 2.1b's E3).
  - Each fix shown failing before it and passing after it.
  - The gate on a local merge with PR #40's head, never pushed: no refusal for `spikes/`.
  - CI's `gate` and `smoke` green.
- **Precondition: a local smoke run.** CS-C1 edits `smoke.mjs`, and the advisor agent's day-session seat runs the full
  gate, smoke included (`advisor.md`, step 2 and its Never line). So before CS-C1's review the session must run the
  smoke locally: with the toolkit's Chromium build (the allowed `node tools/node_modules/playwright/cli.js install
  chromium`), or by 2.1a's route of the container's Chromium under that build's name. Its plan settles which (row 94).
- **Reviews:** security-auditor and test-engineer, then the advisor agent.
- **His asks:** one. **Protected writes:** 2, so about 2 prompts. **His go:** the merge.

### CS-D: the guard (risk tier 2: `.claude/hooks/routine-guard.mjs` and its tests; CLAUDE.md and advisor.md for the lists)
- **Guard changes** (three lines each in Part 6):
  - G-1, PIN-GUARD, by PIN-SCOPE;
  - G-2, the gate v2's folder, by GATE-PATH;
  - G-3, AUTO-ASK;
  - the kit lists that name the protected files: K-6b, and K-11a to K-11c.
- **Why it comes before CS-C:** the gate v2's files are then protected from their first commit. Without it, a routine
  could write them on a branch, a day session would get no ask, and the PR Advisor could merge them, since `tools/` is
  one of its merge places (`routine-guard.mjs:84`).
- **Evidence:**
  - The guard's suite (1107 tests today), `settings.test.mjs` (127) and `checkpoint.test.mjs` (27).
  - New rows for each protected path:
    - it asks in a day session;
    - it is refused for a routine and for an agent;
    - its case variants are caught on win32 and darwin folding;
    - PR #40's `spikes/...` manifests stay free;
    - `mergeAllowed` refuses the gate v2's folder.
  - New rows for G-3:
    - `auto`, `dontAsk` and `bypassPermissions` refuse, with the message to switch to Accept edits;
    - `default`, `acceptEdits`, `plan` and a missing field ask, as today.
  - Before building, CS-D's own plan re-reads the hooks page, since the agent's reading (F-HOOK) is not yet the
    session's.
- **Reviews:** security-auditor and test-engineer, then the advisor agent.
- **His asks:** one. **Protected writes:** 4 (the guard, its tests, CLAUDE.md, advisor.md), so about 4 prompts.
  **His go:** the merge.

### CS-C: the gate v2's first part (risk tier 2: gate files; new dev dependencies)
- **Files:**
  - the gate v2's folder (by GATE-PATH (a): `tools/gate-v2/`, entry `tools/gate-v2/gate.mjs`): its checks, the
    `--prove` suite and its fixtures;
  - root configs: `tsconfig.base.json`, `tsconfig.json` (references only), `eslint.config.mjs`, `.prettierrc.json`,
    `.prettierignore`, `deny.toml`, `rustfmt.toml`;
  - `clippy.toml` in `crates/sim`, `room`, `protocol`, `client-endpoint` and `sim-wasm`;
  - `tsconfig.json` in each package;
  - the root `package.json` and `package-lock.json`: ESLint 10, typescript-eslint and Prettier, exact versions;
  - the root `Cargo.toml`: `exclude = [".gate", ".checkpoints"]` (ROOT-WORKSPACE);
  - the docs: REPO_MAP, STATUS and ROADMAP.
- **The command:** today's conventions (the base-ref rule; exits 0, 1 and 2; `--json`; `--allow-removed`; `--quick`;
  `--prove`), plus `--only <ids>` and `--list` for CI's jobs.
- **Checks:** as Part 1's first-part column.
- **Tools run by their path, never through `npm run`, `npm test` or `npx`.** The settings deny those
  (`.claude/settings.json:237-240`), and a package script could otherwise put a command into the gate.
- **Evidence:**
  - the `--prove` table: each mutation fails exactly its named checks, each control passes;
  - the gate v2 passes on main's tree, and on a local merge with PR #40's head, never pushed (as 2.2's E4);
  - the new npm packages reviewed: registry, integrity, install scripts;
  - timings: PM-0002-54 over 5 runs, and PM-0002-55 measured and reported;
  - the legacy gate unchanged and green.
- **Reviews:** test-engineer, security-auditor and rust-reviewer, then engine-architect for part 3's rules. That is one
  round in two waves of three at most. The advisor agent last.
- **His asks:** one, the patch covering every file of the change-set, protected or not.
  - **Protected writes**, by PIN-SCOPE (b): about 8 files in the folder, 7 root configs, 5 `clippy.toml` files and
    `Cargo.toml`, so about 21 prompts.
  - Under PIN-SCOPE (a), add `package.json` and `package-lock.json`.
- **His go:** the merge.

### CS-E: CI (risk tier 2: `.github/workflows/`)
- **Files:** a new `ci.yml` (proposed name) replaces `gate.yml` and `toolchain.yml`. Its jobs:
  - `gate` and `smoke`: the legacy jobs, moved as they are, names kept so today's ruleset and guard keep working;
  - `rust`: G0's Rust half, G1's rustfmt, G2, G5, G6 and G9's cargo, inside the network cut;
  - `typescript`: G0's Node half, G1's Prettier, G3 and G9's Vitest, inside the cut;
  - `required`: the aggregate, which needs every job, runs `if: always()`, fails unless each job succeeded (a skipped one
    fails too), and compares the check ids that ran with `node tools/gate-v2/gate.mjs --list`.
- **Items:**
  - REQUIRED-CHECK;
  - CUT-CARRY: keep the cut;
  - SETPRIV-HARDEN: `--inh-caps=-all --bounding-set=-all`, the uid not 0, capability sets 0, NoNewPrivs 1, groups
    65534, only `lo` up;
  - CUT-ESCAPE: the tests run as another uid on their own copy of the workspace, so nothing a later step runs or reads
    is theirs. CS-E's plan settles the design with security-auditor and infra-engineer;
  - SETUP-NODE-CACHE: `package-manager-cache: false` on every setup-node step;
  - CANCEL-ON-MAIN: main's runs are never cancelled;
  - IE-5: a step timeout and one retry on the apt install, and `cache: npm` for `tools/`;
  - Swatinem/rust-cache, saved from main only;
  - per-job timeouts, SHA pins, `contents: read`, `persist-credentials: false`.
- **Evidence:**
  - PM-0002-57's and D4's checks on CS-E's own pull request: a failing job made by a failing test (no workflow edit)
    and a cancelled job (cancelled through the API), each turning `required` red; a skipped job and a removed check,
    each a workflow edit asked of him with its exact revert, each failing the aggregate. The merge-box clauses and
    the TypeScript-only pull request wait for PROOF-57;
  - PM-0002-38: each job writes its tools' version lines for the aggregate to compare with the pins; the same
    commit's lint result read again a week later;
  - the name: the aggregate's job id and job name are both exactly `required`, in code font in every kit text (the
    advisor agent's ruling (c));
  - a red run with `--net` removed, ending in the probe's "still reachable" line (CUT-CARRY);
  - SETPRIV-HARDEN's assertions in the log;
  - timings for D3 and PM-0002-56 (5 warm, 3 cold), reported;
  - the cache size for D6;
  - the legacy jobs diffed against `gate.yml`'s: equal but for the named fixes.
- **Reviews:** security-auditor, infra-engineer and test-engineer, then the advisor agent.
- **His asks:** two: the workflow (3 protected writes: `ci.yml` written, `gate.yml` and `toolchain.yml` removed), and
  the two planted workflow failures with their exact reverts (4 writes). So about 7 prompts. **His go:** the merge.
- **Then his ruleset step:** add `required` once its first push run on main is green (Part 8).

### CS-F: the switch (risk tier 2: the guard's merge check; rules text; settings)
- **Edits:**
  - G-4: the guard's merge check requires `required`, `gate` and `smoke`, plus its comment at `routine-guard.mjs:6`, its
    message at `:990`, and its tests;
  - K-12a to K-12h: the lines naming `gate` and `smoke` as the required checks (K-12a, f and g widen "main breaks",
    so they need his words: ROLLBACK-SCOPE);
  - K-13a to K-13d: the lines naming the gate's command learn the gate v2 for v2 code;
  - S-1, by ALLOW-GATE2: the allow rule;
  - the docs: STATUS (REQUIRED-CHECK closed), and DECISIONS.md's "Queued" (KIT-FREEZE-2 leaves, by FREEZE-END (a)).
- **Evidence:**
  - the guard's tests: no `required` refuses; `required` pending or red refuses; two runs named `required`, one of
    them red, refuse (the advisor agent's ruling (c)); all three green allows;
  - a grep: no kit line still names `gate` and `smoke` alone as the required checks;
  - the settings test.
- **Reviews:** security-auditor and test-engineer, then the advisor agent.
- **His asks:** one. **Protected writes:** 7 (the guard, its tests, CLAUDE.md, advisor.md, verifier.md, the gate skill,
  settings.json), so about 7 prompts. **His go:** the merge.
- **Then his ruleset step:** remove `gate` and `smoke` from the required checks. They still run inside `required`.
- **Then PROOF-57,** the first part's closing proof: three scratch pull requests, one TypeScript-only that passes the
  single required check, and one with a failing job and one with a cancelled job, each blocked in GitHub's merge box.
  Its record goes into the next change-set's docs pass.

### First part, in sum
- 7 change-sets.
- 7 exact-text asks (CS-E has two).
- About 45 guard prompts by PIN-SCOPE (b): 4 + 2 + 4 + 21 + 7 + 7, and at most one more per change-set for making
  its scratch copy (5.1).
- 7 merge gos.
- Then PROOF-57.

### The later parts
- **2.3-B** rides step 2.4: G4, G9's boundary layer and Wasm tests, G7 with the budget file, PM-0002-60, PM-0002-38's
  wasm-bindgen line, the `wasm-opt` flag clause, PM-0002-53 (its native half rides the first change-set that adds a
  dependency on `sim`, by the advisor agent's ruling (a)), and WASM-ARTIFACT.
- **2.3-S**, its own change-set after the sim core spike, merged before any f32 or f64 arithmetic enters `sim`: the
  float ban, the map types from sim's dependencies, the same-feature check and FEATURE-UNIFY.
- **2.3-E** rides step 2.5: G6's Vite build, G8, G10, G11 with L1, G12, the decoder panic lints, the missing-browser
  clause, TWO-PLAYWRIGHTS, and the full-gate runs for PM-0002-52, -55, -56, -59 and -61.
- **G13** rides slice 2.
- **Each is planned when its trigger is met.** None is built before its result is in. Each keeps its marks unchanged.

## Part 4. Every item that waits for this step (item 4)

### How to read the table
- **The lists:** "Queued" in `docs/DECISIONS.md`; the "Findings for 2.3" of the 2.1a, 2.1b and 2.2 reports; STATUS's
  "Follow-ups" for the gate step; the three CI security items he accepted with #48; and his two additions: the 2.3-docs
  report's five findings, and #52's leftovers.
- **The tags,** counted for the summary:
  - [1st]: carried by the first part, with its change-set;
  - [later]: with when;
  - [dropped]: with why;
  - [not ours]: not a session's edit;
  - [chat]: a rule for this chat.
- **A row carried by several change-sets** is tagged by its first part.
- **The later kit queue names:**
  - **GUARD-SEC**: the guard's security pass, its own change-set right after CS-F;
  - **GUARD-DAY**: the guard's day-reader pass, after GUARD-SEC;
  - **KIT-CHECKPOINTS**: a CHECKPOINTS.md pass;
  - **KIT-EFFORT**: the agent files' effort lines, right after PR #40 merges, by MODELS-HOW (c).
  - Their order after 2.3 is his to set.

| # | Item | Source | Mark |
|---|---|---|---|
| 1 | KIT-FREEZE-2, with what he meant (FREEZE-FDN) | DECISIONS "Queued" :27-40 | [1st] ends with this step: leaves "Queued" with CS-F's docs pass by FREEZE-END (a), or with the last later part by (b) |
| 2 | his unit walk (UNITS.md §0, LOOP_V2) | "Queued" :41-72 | [later] his order (ORDER (b)): its own docs change-set in a fresh chat, not this step |
| 3 | the Slice 1 note (camera tuning panel) | "Queued" :73-75 | [later] slice 1 |
| 4 | the CLAUDE.md clarification of the advisor tool | "Queued" :76-80 | [1st] CS-B (K-4); leaves "Queued" with CS-B |
| 5 | the render budget bench note | "Queued" :81-85 | [later] ADR 0003's render budget bench |
| 6 | the CLAUDE.md and agent-file change for models | "Queued" :86-91 | [1st] CS-B (K-4) for CLAUDE.md; the agent files by MODELS-HOW ((c): KIT-EFFORT); leaves "Queued" when both are in |
| 7 | what asked him most (14 prompts: worktree and branch operations under `.gate/`; shell writes the guard cannot read) | 2.1a report :81-85 | [1st] weighed in Part 6 (no guard change); CS-B's K-14 tells agents the forms |
| 8 | every refusal (37, each with its route) | 2.1a report :86, :107-146 | [1st] as row 7 |
| 9 | PROMPTS-2.3: the prompt tally; his PROMPTS-2.3 (a): "Step 2.3's plan weighs those guard rules as findings" | 2.1b :169-170; DECISIONS :283 | [1st] weighed in Part 6: every rule kept; CS-B's K-14 |
| 10 | the guard refused `git -C` twice | 2.1b :171-172 | [1st] CS-B (K-7); the guard's rule stays (Part 6) |
| 11 | Node 24.16 or later with Playwright before 1.60 hangs after the download (his Windows run) | 2.1b :173-175; DECISIONS :339 | [later] with NODE-26: the toolkit's Playwright moves to 1.60 or later before `tools/` leaves Node 22; CS-E keeps Node 22 |
| 12 | F4: the stray and decoy page check stopped running after the move | 2.1b :176-178 | [1st] CS-C1 (root and old folder only) |
| 13 | the Bash background limit stopped the saver once | 2.1b :179-180 | [dropped] CHECKPOINTS §5 already sets the 2-hour limit, which this session's saver uses |
| 14 | AUTO-ASK: Auto mode can pass the guard's ask | 2.2 report :333 | [1st] CS-D (G-3) |
| 15 | MANUAL-MODE | 2.2 :334 | [1st] CS-B (K-6; K-6b with G-3) |
| 16 | GIT-C (`CHECKPOINTS.md:221`) | 2.2 :335 | [1st] CS-B (K-7) |
| 17 | ADVISOR-TOOL-WORDS | 2.2 :336 | [1st] CS-B (K-4) |
| 18 | MODELS-2026-11-12 | 2.2 :337 | [1st] CS-B (K-4); the agent files by MODELS-HOW |
| 19 | REFUSED-PUSH | 2.2 :338 | [dropped] the session's permission check refused a compound line, no kit rule did, and the guard allows the push; a plain `git push` line is the route |
| 20 | PIN-GUARD | 2.2 :339 | [1st] CS-D (G-1), by PIN-SCOPE |
| 21 | G0: exit 2, the source checks, the mutations | 2.2 :340 | [1st] CS-C |
| 22 | OVERFLOW-XCRATE-CI (Medium) | 2.2 :341 | [later] the advisor agent's ruling (a) of this review moved the anchor, as its text allows: the native half rides the first change-set that adds any dependency on `sim`; PM-0002-53 is met at the first change-set with both a crate depending on `sim` and a Wasm runner (expected 2.3-B). CS-A rewrites STATUS :95 and the archive's anchor text |
| 23 | TSCONFIG | 2.2 :342 | [1st] CS-C (G3) |
| 24 | REQUIRED-CHECK | 2.2 :343 | [1st] CS-E, his ruleset, CS-F |
| 25 | SETUP-NODE-CACHE (security, accepted with #48) | 2.2 :344 | [1st] CS-E, a CI edit asked of him |
| 26 | SAVER-SKIP-CI | 2.2 :345 | [dropped] it fails closed (a `[skip ci]` head has no `required` run, so it cannot merge), and CHECKPOINTS §9 already makes the last commit before ready by hand |
| 27 | CRATE-NAMES | 2.2 :346 | [1st] CS-C (G5's bans) |
| 28 | CI-ENFORCED-LATER | 2.2 :347 | [1st] CS-C: the lint-table opt-in (G2) and exact npm versions (G5). Its "has a test" part is dropped: no ADR 0002 mark asks it, and review covers it |
| 29 | WASM-ARTIFACT | 2.2 :348 | [later] 2.3-B: `sim-wasm`'s crate type comes with its bindings |
| 30 | TWO-PLAYWRIGHTS | 2.2 :349 | [later] 2.3-E, with G12's browser cache |
| 31 | TOOLKIT-RESOLVES-UP | 2.2 :350 | [1st] CS-C1 |
| 32 | CARGO-NET-OFFLINE | 2.2 :351 | [later] with the first third-party crate, a CI edit asked then |
| 33 | CRATE-LICENCE | 2.2 :352 | [later] when a licence check is adopted: G5's row has none, and ADR 0006's baseline may add one |
| 34 | CUT-CARRY | 2.2 :353 | [1st] CS-E: keep the cut |
| 35 | CUT-ESCAPE (security, accepted with #48) | 2.2 :354 | [1st] CS-E, a CI edit asked of him |
| 36 | SETPRIV-HARDEN (security, accepted with #48) | 2.2 :355 | [1st] CS-E, a CI edit asked of him |
| 37 | CANCEL-ON-MAIN | 2.2 :356 | [1st] CS-E |
| 38 | LINT-TABLE-PEDANTIC | 2.2 :357 | [1st] CS-C: the gate's command line. The ADR's wording waits for the drafts' next revision, with his go then |
| 39 | FEATURE-UNIFY | 2.2 :358 | [later] 2.3-S, with the same-feature check |
| 40 | ROOT-WORKSPACE | 2.2 :359 | [1st] CS-C (`exclude`, tested locally by its skeptic) |
| 41 | CLIENT-NET-RULE | 2.2 :360 | [later] the drafts' next revision (part 3's wording, his go), then a G3 entry |
| 42 | NODE-26 | 2.2 :361 | [later] its own toolchain change-set after 2.3's first part, before 2027-03-31 |
| 43 | OVERFLOW-PER-CRATE | 2.2 :364 | [later] the first third-party crate |
| 44 | OVERFLOW-PANIC-RULE (security, accepted with #48) | 2.2 :375; his go for #48, item 5 | [later] the first code in `room` or a server crate (2.5) |
| 45 | PACKAGE-ENTRIES | 2.2 :382 | [later] the step that gives a package code (2.5) |
| 46 | EMPTY-SLOTS (OQ-0002-7, the wasm-bindgen pin) | 2.2 :383 | [later] step 2.4, after its reading by day |
| 47 | OQ-0002-8-TEST | 2.2 :388 | [later] PM-0002-64, at step 2.5, is its test (the advisor agent's reading in this review); recorded by CS-A |
| 48 | BRIEF-STALE | 2.2 :393 | [not ours] his and the Advisor's brief |
| 49 | PX-1: the merge check trusts `gate` and `smoke` by name alone | STATUS :70; archive :457-464 | [later] GUARD-SEC, by his PX1-ROUTE (a): "its own change-set after the foundation's gate step" |
| 50 | PX-1's two test gaps | archive :465-468 | [later] GUARD-SEC (they ride PX-1) |
| 51 | W1 (Medium): a nested `<dir>/.claude/` file written without an ask | archive :1011-1013 | [later] GUARD-SEC |
| 52 | W2 (Medium): a gitignored path written without an ask | archive :1011-1013 | [later] GUARD-SEC |
| 53 | W3 (Low): `kitDir` checks for `CLAUDE.md` only | archive :1013 | [later] GUARD-SEC |
| 54 | the recorded Windows-name limits | STATUS :71-72; archive (kit-v3-5) | [later] GUARD-SEC: recorded again or closed |
| 55 | F2: CI and a pure revert's dropped symbols (High, fixed in text) | archive :435-440 | [later] its own change-set after 2.3's first part, as its text says; CS-C builds `--allow-removed` with the same pure-revert shape in mind |
| 56 | F5: the third "main breaks" case in the revert check | archive :449-454 | [later] with his words (a loosening or a narrowing) |
| 57 | F-1: CHECKPOINTS §4's trust rule | archive :235-239 | [later] KIT-CHECKPOINTS |
| 58 | L12: a day fetch into `refs/remotes/origin/*` or `refs/heads/main` | archive :275 | [later] GUARD-DAY |
| 59 | L14: day git aliases | archive :276-277 | [later] GUARD-DAY |
| 60 | L9 / SA-9: fold case always | archive :278 | [later] GUARD-DAY |
| 61 | L13: the ask in a cloud day session and in bypassPermissions | archive :279 | [1st] CS-D's G-3 refuses kit edits in `bypassPermissions` and `dontAsk` (F-HOOK). The rest, the ask in a cloud day session, goes to GUARD-DAY |
| 62 | L1's runners (sudo, doas, `sh -c`, eval) | archive :280-282 | [later] GUARD-DAY |
| 63 | A-4: the day bootstrap passes when `routine-guard.mjs` is missing | archive :284-285 | [later] GUARD-DAY |
| 64 | FU-P3a: the day reader against real bash | archive :288-290 | [later] GUARD-DAY |
| 65 | FU-P4a (Medium): here-document bodies in D-ASK | archive :294-298 | [later] GUARD-DAY, first |
| 66 | FU-P4b: D-ASK's prompt cost | archive :299-305 | [later] GUARD-DAY; his call |
| 67 | FU-P4c (Medium): the clone folder after unknown options | archive :306-311 | [later] GUARD-DAY, first |
| 68 | FU-P4d: R-PROG's reason drops the hints | archive :314-318 | [later] GUARD-DAY |
| 69 | A3's plan readers | archive :257-263 | [later] GUARD-SEC, weighed by security-auditor |
| 70 | A3's cuts | archive :264-267 | [later] GUARD-SEC |
| 71 | CHECKPOINTS §7's cap line | archive :250-252 | [later] KIT-CHECKPOINTS |
| 72 | E9's untested cases | archive :253-254 | [later] KIT-CHECKPOINTS |
| 73 | KIT-ALIGN | archive :470-480 | [later] a kit pass after 2.3's first part: stale rules lines, not the gate or CI |
| 74 | the kit-v3-1 Windows test | STATUS :75 | [later] its own change-set, on his Windows machine |
| 75 | STATUS citations in `advisor.md` :17 and :286, and `netcode-architect.md` :24 | archive :490-494 | [1st] CS-B (K-10a to K-10c) |
| 76 | the same citations in the ADRs, the register, LOOP_V2 and ENGINE_OVERVIEW | archive :490-494 | [later] the drafts' next revision |
| 77 | SD-N1: a pasted block is his word, against `advisor.md:204-207` | archive :495-497 | [later] its own ask: it changes what counts as his go |
| 78 | SD-N2: "the advisor" in Standing permission | archive :498-500 | [1st] CS-B (K-9). `advisor.md` step 6 says "your APPROVE", so it needs no edit |
| 79 | SD-N3: the Lanes rule is enforced by nothing | archive :501-502 | [dropped] nothing can enforce it while both accounts push as one GitHub user, and a merge's expected head still fails a stale go |
| 80 | SD-N6: the archive's copies of his words merge by path | archive :503-505 | [later] his call, asked with GUARD-SEC (the guard's decision-doc list) |
| 81 | SD-L3: the index misses wrapped dated lines | archive :506-508 | [dropped] the verifier found nothing lost: each decision is in DECISIONS.md or indexed by its code |
| 82 | IE-1: `smoke.mjs:31`'s wording | archive :523-524; his F1-FIX words | [1st] CS-C1 |
| 83 | TE-2: a base holding both full sets resolves silently | archive :525-526 | [1st] CS-C1 |
| 84 | A-1: a base in the old folder and a head at the root (accepted for 2.3 with #50) | archive :527-528 | [1st] CS-C1: recorded in `tools/README.md` as a known limit, since it reaches old bases only |
| 85 | N2: containment compares paths as text (accepted for 2.3 with #50) | archive :529-531 | [1st] CS-C1: recorded as a known limit, as on main |
| 86 | L-4: CLAUDE.md's setup line uses the `npx` the settings deny | archive :532-533 | [1st] CS-B (K-8) |
| 87 | the v2 root page against pre-move bases | archive :534-535 | [1st] CS-C1: recorded as a known limit; v2's pages live in `packages/` |
| 88 | IE-5 (Medium): CI depends on live mirrors (accepted for 2.3 with #50) | archive :536-540 | [1st] CS-E |
| 89 | TE-7: rows for `editor.html` alone and a case variant | archive :541-543 | [1st] CS-C1 |
| 90 | R2-1: trailing dots or spaces (accepted for 2.3 with #50) | archive :544-546 | [1st] CS-C1 |
| 91 | five "before the foundation" lines with no pointer to his decisions | archive :547-554 | [later] the drafts' next revision, by their owners |
| 92 | the three exception lines (2.3-docs finding 1) | 2.3-docs report :288-289 | [1st] CS-B (K-1 to K-3) |
| 93 | BRIEF-STALE and those lines also lack the 2026-11-13 decision (finding 2) | 2.3-docs :290 | [not ours] the brief is his and the Advisor's; the drafts' lines go with row 91 |
| 94 | no `tools/` install in this container, so the local smoke cannot start (finding 3) | 2.3-docs :291-292 | [1st] a precondition of CS-C1, which edits `smoke.mjs`: a local smoke run before its review (Part 3). For the other change-sets, CI's smoke job stands, as in 2.1b and 2.2 |
| 95 | the advisor-tool moment-5 line (finding 4) | 2.3-docs :293-294 | [1st] CS-B: K-4's optional line B, wider than his words, for him to accept or refuse |
| 96 | read the advisor tool's model from the records (finding 5) | 2.3-docs :295-296 | [1st] CS-B: K-4's optional line A |
| 97 | STATUS still lists #52 as open (:5, :13, :29, :36, :43) | main 4460ef5 | [1st] CS-A (not in on main) |
| 98 | ROADMAP says "PR #52: in review" (:117) | main 4460ef5 | [1st] CS-A |
| 99 | his words of 2026-11-14 stand only in the merge message | main 4460ef5 | [1st] CS-A |
| 100 | his rule of 2026-11-14, item 2: consult the advisor tool before a step is saved as done | merge 4460ef5 | [chat] followed here; recorded by CS-A |
| 101 | his rule of 2026-11-14, item 3: about my own process, only what a record shows, with its time | merge 4460ef5 | [chat] followed here; recorded by CS-A |
| 102 | MERGE-PLACES: whether the v2 folders become PR-Advisor merge places | archive :1005-1008 | [later] his decision, after 2.3's first part |
| 103 | the netcode chat does not take main 4460ef5 now (his addition 3) | his message of 2026-11-14 | [1st] Part 8 says when it must |
| 104 | the old plan's "PR #46 merged (the guard must learn the gate v2's files)" | old plan part 3 | [1st] CS-D (G-2) |
| 105 | the old plan's "the gate v2's path (OQ-0002-2's third part)" | old plan part 3; ADR 0002 :1019-1021 | [1st] GATE-PATH (Part 7); CS-C builds it; the register's OQ-0002-2 row reads "answered" from CS-C's docs pass |
| 106 | the old plan's "Mustafa's branch-protection setting" | old plan part 3 | [1st] his ruleset steps around CS-E and CS-F (Parts 8 and 9) |
| 107 | the per-edit hook (PM-0002-62) | ADR 0002 :1328 | [later] ADR 0002's own place: its kit change-set in phase 1 |

## Part 5. Every kit, gate and CI edit foreseen (item 5)

### 5.1 How each edit is asked and written (the F1-FIX way of 2026-11-11, at scale)
1. **Prepared away from the main folder.**
   - Rules text (CS-B, and the kit lines of CS-D and CS-F): the patch is generated by script from main's text and the
     after-texts, as `kit-texts.py` does, so no kit file is written before his yes.
   - Code (CS-C1, CS-C, CS-D's guard, CS-E): developed and tested in a plain copy of the tree made with `git
     archive`, never a worktree of this repository and never committed. For builds the copy sits under
     `.gate/<change-set>/` (CHECKPOINTS §4); for the guard's own files it sits in the session's scratchpad, outside
     the project, so no step leans on the guard's nested-folder gap (W1). The 2.1a precedent: `.gate/fdn-scratch`,
     and F1-FIX's patch.
   - The patch (the copy's diff against main) goes into the change-set's notes folder and is pushed with a checkpoint
     save. Before his yes there is at most one guard prompt per change-set, for making the copy.
2. **Asked in chat.** The ask carries:
   - the patch's path;
   - the checkpoint commit's full SHA, and the patch's sha256;
   - the files, with line counts;
   - for rules text, each changed sentence before and after, in plain words, marked looser or stricter, with his words.
3. **His yes names that SHA.** A guard prompt alone is not his yes (his words).
4. **Written in Accept edits.** He switches the chat to Accept edits. The main session then writes each file with the
   Write tool, from the scratch copy's file, never retyped: one guard prompt per protected file.
   - Only the main session writes these files; agents and teammates never do (CLAUDE.md step 3).
   - `git apply` is not used for them: the guard does not read patch-applying commands (the archive's limit L4).
5. **Checked by script.** The `git diff` of the written files must equal the patch byte for byte. If one byte differs,
   the session stops and tells him.
6. **Back to Auto.** He switches back once the session says the writes are committed and pushed.

### 5.2 Rules text whose exact before and after can be written now (Appendix K, generated from main 4460ef5)

| Id | File and lines on 4460ef5 | Change-set | What | Looser or stricter |
|---|---|---|---|---|
| K-1 | CLAUDE.md :20-23 | CS-B | the Phase 0 bullet's exception names his decision of 2026-11-13, quoting its sentence whole | looser, by his exception, inside its limits |
| K-2 | CLAUDE.md :54-57 | CS-B | "Phase 0 writes no v2 code, except ...", the same | looser, as K-1 |
| K-3 | advisor.md :107-108 | CS-B | the reviewer's line, with both of his limiting sentences | looser, as K-1 |
| K-4 | CLAUDE.md :170-185 | CS-B | "Models and effort", bullets 1 to 4 (Routines unchanged): his words of 2026-11-12 (four bullets) and of 2026-11-11 (his bullets, without the replaced model sentence and the request to record them), with two optional lines (A, B) for him to accept or refuse; its effort line exempts the netcode chat, and is also given under MODELS-HOW (a) and (b) | neither: it matches his words. Optional B is wider than his words. One clause is marked to verify (a workflow agent's model option) |
| K-6 | CLAUDE.md :71-73 | CS-B | MANUAL-MODE: "Accept edits or Manual mode" made true for a cloud session | wording |
| K-6b | CLAUDE.md :71-73 | CS-D | the same clause says the guard refuses kit edits in Auto mode (only with G-3) | stricter |
| K-7 | CHECKPOINTS.md :219-224 | CS-B | GIT-C: `git status` run inside the copy | wording |
| K-8 | CLAUDE.md :236-237 | CS-B | L-4: the setup line uses the allowed `node tools/node_modules/playwright/cli.js install chromium` | wording |
| K-9 | CLAUDE.md :127-130 | CS-B | SD-N2: "the advisor agent" approved, twice | wording |
| K-10a to c | advisor.md :17, :286-287; netcode-architect.md :23-24 | CS-B | status-diet's citations point to the archive and DECISIONS.md | wording |
| K-11a to c | CLAUDE.md :68-69; advisor.md :164-166 and :39-43 | CS-D | the lists of protected and gate files, and escalation category 3's, name the gate v2's folder and PIN-SCOPE's paths | stricter |
| K-12a to h | CLAUDE.md :119, :130; advisor.md :157, :163, :224, :273, :294, :323 | CS-F | `gate` and `smoke` as the required checks become `required`, which needs both and every v2 job | same strength, except K-12a, f and g, which widen "main breaks" (ROLLBACK-SCOPE) |
| K-13a to d | CLAUDE.md :74-75, :238-239; verifier.md :21; the gate skill :6 | CS-F | the gate's command lines learn the gate v2 for v2 code | stricter |
| K-14 | CHECKPOINTS.md :128 | CS-B | the agents' paste block names the shell forms the guard refuses | guidance |

### 5.3 Kit edits that wait: KIT-EFFORT
Line 6 of each agent file below changes, after PR #40 merges, by MODELS-HOW (c), or in CS-B by (a):
- `effort: high` becomes `effort: max` in ten files: advisor, art-director, game-designer, graphics-engineer,
  infra-engineer, perf-auditor, rust-reviewer, security-auditor, test-engineer, verifier.
- `effort: xhigh` becomes `effort: max` in two: engine-architect, netcode-architect.
- `docs-keeper.md` stays `high`: it writes docs and reviews no game or engine code.
- **The reading:** "the agents that review, verify or design game and engine code always run at max". game-designer is
  counted, because it designs the rules that code implements. infra-engineer is counted too: his words allow max for
  every agent.

### 5.4 Guard code (CS-D, CS-F)
- **The changes:** G-1 to G-4, described in Part 6, with their test rows.
- **The exact code** is each change-set's patch, asked as in 5.1.

### 5.5 Gate files
- **CS-C1, the legacy gate:** `tools/gate.mjs` and `tools/smoke.mjs`, the six items of Part 3. The exact diff is in
  its ask.
- **CS-C, the gate v2:**
  - new files in the gate v2's folder;
  - the root configs, `clippy.toml` files and package `tsconfig.json` files listed in Part 3;
  - `package.json`, `package-lock.json` and `Cargo.toml` changes;
  - the exact text in its ask. Each config's settings come from ADR 0002 part 7, named there line by line.

### 5.6 CI (CS-E)
- **The workflow:** `.github/workflows/ci.yml`, with the jobs and items of Part 3. `gate.yml` and `toolchain.yml` are
  removed.
- **The legacy jobs** are moved byte for byte except IE-5's two lines and the concurrency rule, shown as a diff in the
  ask.
- **The exact YAML** is its ask.

### 5.7 Settings (CS-F, only by ALLOW-GATE2 (a))
After `.claude/settings.json:7` (`"Bash(node tools/gate.mjs *)",`), two lines:
```
      "Bash(node tools/gate-v2/gate.mjs)",
      "Bash(node tools/gate-v2/gate.mjs *)",
```

### 5.8 Lines checked that need no edit
- **`engine-architect.md:52` and `graphics-engineer.md:41`** ("write engine code in phase 0"): agents never write engine
  code, and the gate v2 is not engine code.
- **`advisor.md:123`, `NIGHT_SHIFT.md:56`, the changeset skill :31:** they defer to CLAUDE.md "Models and effort",
  which K-4 changes.
- **ADR 0002 :642 and PM-0002-57:** they name "one required check", not `gate` and `smoke`.
- **`docs/START_HERE.md:87`:** it names `gate.yml`'s jobs. It is a doc, not kit: CS-E's docs pass updates it.

## Part 6. Every guard change, in three lines (item 6), and PROMPTS-2.3
**G-1, PIN-GUARD (CS-D)**
- What it changes: `protectedRel` and `mergeAllowed` learn PIN-SCOPE's list:
  - by (b): `rust-toolchain.toml`, `Cargo.toml`, `deny.toml`, `rustfmt.toml`, `tsconfig.base.json`, `eslint.config.mjs`,
    `.prettierrc.json` and `.prettierignore` at the root, and `crates/*/clippy.toml`;
  - each anchored at the root, with its letter-case folds for Windows and macOS.
- Stricter or looser: stricter. These paths ask in a day session, are refused for routines and agents, and never merge
  through the PR Advisor.
- What could go wrong:
  - **Friction.** By (b), each change to the root `Cargo.toml` asks him; that file also holds the shared dependency
    versions. By (a), every dependency change asks too, and no builder agent may make one.
  - **A pattern matching any depth** would catch PR #40's `spikes/netcode-proof/Cargo.toml` and stall that chat. Root
    anchoring and a test row prevent it.
  - **A missed case fold.**

**G-2, the gate v2's folder (CS-D)**
- What it changes: every path under `tools/gate-v2/` (by GATE-PATH (a)) is protected and refused by `mergeAllowed`. It
  is one folder rule, not a file list, so files added later are covered.
- Stricter or looser: stricter.
- What could go wrong: a prefix bug would leave the gate unprotected. Test rows cover `tools/gate-v2x/`, case variants
  and `tools/./gate-v2/`. With GATE-PATH (b) or (c), the rule names those paths instead.

**G-3, AUTO-ASK (CS-D)**
- What it changes: for the edits that ask today (kit and gate files), the guard reads the hook input's
  `permission_mode`:
  - `auto`, `dontAsk` or `bypassPermissions`: it refuses, saying "switch the chat to Accept edits";
  - `default`, `acceptEdits`, `plan` or no field: it asks, as today.
- Stricter or looser: stricter. A refusal replaces an ask that Auto mode might pass (K1, 2026-11-10: answered in six
  seconds, by whom the log does not say). The hooks page says Auto's classifier normally does not decide a hook's ask
  (F-HOOK); G-3 makes it certain.
- What could go wrong:
  - If a Claude Code version stops sending the field, the guard falls back to today's ask, never to an edit without a
    prompt.
  - A session in Auto must switch modes before each kit write, as his rule already says.
  - Routines are unaffected: kit edits are refused to them already.

**G-4, the required-check switch (CS-F)**
- What it changes: `mergeCheck` requires check runs named `required`, `gate` and `smoke`, each completed and green on
  the approved SHA. Today it requires `gate` and `smoke` (`routine-guard.mjs:1486`).
- Stricter or looser: stricter, by one check that itself needs every job.
- What could go wrong:
  - A pull request whose head predates CS-E (PR #40, until it merges main) cannot merge. That fails closed.
  - Renaming the aggregate job later blocks merges until the guard learns the new name.
  - The by-name weakness (PX-1) carries over to `required` until GUARD-SEC.

**S-1, the allow rule (CS-F, by ALLOW-GATE2).** It is not a guard change, but it gets the same three lines:
- What it changes: two allow rules for the gate v2's command, as `node tools/gate.mjs` has (`settings.json:6-7`).
- Stricter or looser: looser. That command no longer prompts, also for routines, which otherwise cannot run it
  unattended.
- What could go wrong: the command runs the gate's pinned tools without a prompt. If the gate v2 ever ran package
  scripts, a pull request could put a command into it. CS-C's rule (tools by path, never package scripts) and its review
  stand against that. His words of 2026-11-11, "Do not edit any settings file, hook or allow-list to get fewer prompts",
  are why it is his decision.

**PROMPTS-2.3** (his PROMPTS-2.3 (a): "Step 2.3's plan weighs those guard rules as findings"). Each rule that asked or
refused in 2.1a and 2.1b:
1. **Worktree add and switch under `.gate/` ask** (6 of 2.1a's 14 prompts). **Keep.** A worktree is how a PR's head is
   read safely, and his correction of 2026-11-11 already makes his answer on such a prompt his answer.
2. **Shell lines the guard cannot read ask** (main session) or are refused (agents): variables, loops and redirects. They
   caused 8 prompts and most of the 37 refusals. **Keep.** His rule of 2026-11-05 ("any line the guard can't fully read
   is an ask") is what stops an unseen kit write. The routes cost little: a literal path, the Write tool, one plain
   command. K-14 tells agents.
3. **`git -C` refused.** **Keep.** The guard reads git's subcommand only when no option comes before it, and `cd` first
   works. K-7 fixes the one rule line that taught `git -C`.
4. **A relative write after `cd` refused for agents.** **Keep.** The guard cannot follow `cd`; absolute paths work.
5. **`git checkout`, `restore`, `reset --keep` and `worktree remove --force` refused.** **Keep.** They move a HEAD or
   delete work (the Never list; CHECKPOINTS §10 forbids `--force`). `git switch` and a plain `git worktree remove` are
   the routes.
6. **The permission system's denials** ("has been denied", no list named) are not the guard's; there is nothing to change
   in the kit.

So no guard rule changes for PROMPTS-2.3: one rules line (K-14) and one rule fix (K-7).

## Part 7. The gate v2's script path: OQ-0002-2's third part (item 7)
- **The question:** where the gate v2's script lives. ADR 0002 left it to this plan (:1019-1021), as "answered in part"
  by his REG-TRUE (b) of 2026-11-11.
- **The options:**
  - **(a) Its own folder beside the legacy gate: `tools/gate-v2/`**, run as `node tools/gate-v2/gate.mjs`. Its checks,
    its `--prove` suite and, later, the budget file live inside.
    - Cost: one guard folder rule (G-2), the allow rule if ALLOW-GATE2, and K-13's command lines. Nothing about the
      legacy gate moves.
    - It is OQ-0002-2's option (a) ("both gates and the legacy tools in `tools/`"), and it fits ADR 0002 part 1 as he
      accepted it: no new root folder.
  - **(b) The gate v2 takes `tools/gate.mjs`, and the legacy gate moves** (to `tools/gate-legacy.mjs`, say). That gives
    one name in the long run, R5's working name. But it renames a gate file that all of these name:
    - CI;
    - the guard's file list and its night-shift checks;
    - `settings.json`'s allow rules;
    - CLAUDE.md, START_HERE, the gate skill and `verifier.md`;
    - PR #40's own commands.
    2.1b counted about 8 kit files for a like move. The rename lands in the middle of #40's work, and every v2 change
    would then edit the file that guards `legacy/`.
  - **(c) A new root folder, `gate/`.** It is clean, and outside the PR Advisor's merge places by default. But it adds a
    root folder to the layout he accepted in ADR 0002 part 1 (SCOPE (a)), so it needs his word on that layout too.
- **Recommendation: (a).** The fewest paths change. The legacy gate stays untouched while it guards `legacy/`. The guard
  learns one folder. When legacy retires, a later change-set can give the gate v2 the short name.
- **Afterwards:** CS-C's docs pass marks OQ-0002-2 answered in the register and in ADR 0002. These are decision docs, in
  a tier-2 pull request that merges on his go anyway.

## Part 8. PR #40 after each merge, and when he changes the required check (item 8)
**Facts:**
- #40 is a draft, head 4bd5ac3. GitHub lists its base as 4d21c1a, and it conflicts with main 4460ef5.
- While it conflicts, GitHub runs no `pull_request` workflow on it (F-PR). Its push runs use its own branch's workflow
  files.
- A `pull_request` run takes the workflow of the PR head merged into main (F-PR).
- No session pushes to #40's branch. What it must do reaches its chat through him, or through STATUS.
- His words of 2026-11-14: #40 does not take main 4460ef5 now, because #52 is docs only.

| After | What #40 must do |
|---|---|
| CS-A | nothing now. When it next merges main (in its worktree), it keeps both sides in STATUS, ROADMAP and DECISIONS.md, where #52 already conflicts with it |
| CS-B | nothing. Nothing in `spikes/` changes. The new CLAUDE.md keeps the netcode chat's models and efforts: K-4 exempts it from the callers' `max`, and MODELS-HOW (c) leaves the agent files' effort lines alone until it merges |
| CS-C1 | nothing. Its gate command `node tools/gate.mjs --no-smoke` stays. The stray-page check looks at the root and the old folder only; CS-C1's evidence runs on a local merge with its head |
| CS-D | nothing to change. After its next main merge it runs the new guard: the root pins and gate configs are protected, its `spikes/` manifests are not, and in Auto mode a kit edit is refused (its planned `.github/` Safari workflow needs Accept edits and his yes anyway) |
| CS-C | nothing. The gate v2 ignores `spikes/`: the spikes keep their own workspace, G1 and G3 read `packages/**`, and G2 and G5 read the root workspace. CS-C's evidence runs the gate v2 on a local merge with its head |
| CS-E | it must merge main, resolving its conflicts, **before its next ready**, so its push and `pull_request` runs carry `required`. Until then its push runs use its old `gate.yml` and `toolchain.yml` |
| his ruleset change (add `required`) | from then on #40 merges only with `required` green on a head that has taken main. It is a draft, so nothing breaks now |
| CS-F | its merge, on his go from its own day session, is checked against `required` by the guard |
| #40's own merge | KIT-EFFORT follows (the agent files' effort lines, by MODELS-HOW (c)) |

**When he changes the required check** (ruleset 25069670):
1. **After CS-E merges and its first push run on main shows `required` green:** add `required` to the required checks,
   and keep `gate` and `smoke`. If the ruleset offers a source for the check, pick GitHub Actions. That choice is
   documented for branch protection rules; this session could not confirm it for rulesets.
2. **After CS-F merges:** remove `gate` and `smoke`. They still run inside `required`.

Why this order: every merge needs `gate` and `smoke` green from both the ruleset and the guard, throughout. From step 1
to CS-F the ruleset also requires `required`, so it is stricter than the guard, which is safe. From CS-F on, both
require `required`. Reordering instead, step 2 before CS-F, would leave the rules text naming `gate` and `smoke` as the
required checks while the ruleset no longer did.

**Then PROOF-57** (after step 2): the merge-box proofs of PM-0002-57 and D2, on scratch pull requests.

## Part 9. What he does himself (item 9)
1. **Now:**
   - answer GATE-PATH, PIN-SCOPE, FREEZE-END and MODELS-HOW, and ALLOW-GATE2 and ROLLBACK-SCOPE now or at CS-F;
   - give or refuse his go on this plan.
2. **For each change-set:**
   - **The exact-text asks** (CS-B, C1, D, C, F, and two for CS-E: seven): say yes in chat, naming the checkpoint
     commit's SHA.
   - **Modes:** switch the chat to Accept edits before the writes, and back to Auto after. The session says when.
   - **Guard prompts:** answer each, about 4, 2, 4, 21, 7 and 7, and at most one per change-set for its scratch copy.
   - **The merge** (CS-A to CS-F, seven): give his go in the chat, after the advisor agent's APPROVE of the final head.
3. **GitHub:** ruleset 25069670 (Settings → Rules → Rulesets):
   - add `required` after CS-E, choosing GitHub Actions as its source if offered;
   - remove `gate` and `smoke` after CS-F.
4. **Three scratch branches for PROOF-57,** after his second ruleset step: let the session push
   `claude/sweet-mayer-grsukw-ts-only`, `claude/sweet-mayer-grsukw-fail` and `claude/sweet-mayer-grsukw-cancel`, each
   opened as a pull request, read in GitHub's merge box and closed unmerged. Or open them himself.
5. **His Windows laptop, after CS-C:** PM-0002-40's two-system check. Exact steps come in CS-C's report, in plain words:
   1. install the v2 toolchain: rustup takes the pin from `rust-toolchain.toml`, Node 22, `npm ci --ignore-scripts` at
      the root;
   2. run the gate v2's format step;
   3. paste the hash line it prints into the chat.
   With 2.3-E later: PM-0002-55's Windows timing, 5 runs.
6. **Later, not in the first part:**
   - a test GitHub account for the fork pull request (PM-0002-58, step 2.6);
   - his calls on SD-N6, F5, FU-P4b and MERGE-PLACES;
   - the order of the kit queue;
   - telling the netcode chat what Part 8 says, if it does not read STATUS.

## Part 10. Risks
1. **The first part's checks enforce rules from drafts not yet accepted** (ADR 0002 parts 2 to 9; ADR 0004's bans). His
   decision of 2026-11-13 allows building them. If a spike changes a draft, a check changes with it: mostly a config
   line.
2. **More asks.** The guard's new protections (PIN-SCOPE) add prompts, and asks for later game chats. That is why
   PIN-SCOPE is his.
3. **CS-E moves the legacy jobs into a new workflow.** A slip there would weaken the legacy game's guard. Against it:
   the moved jobs are diffed against `gate.yml`'s, equal but for the named fixes, and each job gets a planted failure.
4. **The switch order.** A wrong order could block every merge, which fails closed, or loosen the merge check. In Part
   8's order, `gate` and `smoke` stay required by both the ruleset and the guard at every step, the ruleset is at
   least as strict as the guard from step 1 on, and the rules text stays true at every merge.
5. **#40's merge burden** grows with each docs change-set. It must merge main before its own merge, and after CS-E it
   needs the new workflow.
6. **G-3 rests on the hook input's `permission_mode`.** Its fallback is today's ask.
7. **The gate v2 runs pinned tools inside one `node` process.** For those tools, that steps around the settings' deny of
   `npx` and `npm test`. It never runs package scripts. security-auditor reviews this in CS-C and S-1.
8. **`--prove`'s dependency mutations need a consistent lockfile in a throwaway copy.** CS-C's plan settles how: local
   stand-in crates by name offline, or the network in CI's `--prove` job.
9. **Usage and time.** Seven change-sets, each with its own plan, reviews and verifier. This chat's fullness is in the
   end message; a fresh chat per change-set is possible, each with its own checkpoint.
10. **Readings by agents.** F-HOOK and F-PR rest on two docs agents' quotes of official pages. CS-D's and CS-E's plans
    re-read the pages before building on them.

## Part 11. This plan's review
- **His message of 2026-11-14:** the advisor agent's review, and the advisor tool at its five moments. No review rounds
  and no verifier.
- **The advisor agent** (its file last changed in b478380, before this session started) reviews this plan at a pushed
  commit. Its verdict is recorded in the checkpoint against that SHA. REVISE means a new revision and a new review.
- **The advisor tool:**
  - consultation 1 (moment 1) is done;
  - any design call with more than one defensible answer gets moment 4;
  - moment 5 comes before the plan is saved as done, after the advisor agent's verdict, by his rule 2 of 2026-11-14.
- **Revision 2** (the advisor agent's review of revision 1 at 03d05f5: REVISE), what changed:
  - H1: PM-0002-57's "cannot merge" and D2's "single required check" move to PROOF-57, after his second ruleset
    step; Part 8's and Part 10's sentences on the order restated (the ruleset is the stricter one from step 1 to CS-F).
  - M2: CS-E's planted skipped job and removed check, with their reverts, are a second ask (4 writes); its failing
    and cancelled jobs use a failing test and an API cancel; the totals become 7 asks and about 45 prompts.
  - M3: Part 2's cells: D1 and PM-0002-52 met at slice 5; PM-0002-47, -51 and -53 name their change-sets;
    PM-0002-38 placed (CS-E, its wasm-bindgen line at 2.3-B); the summary adds -44, -51 and -52.
  - M4: Rule 3 is the plan's choice under his "may", with its list; G11 may wait past its result.
  - M5: K-4's effort line exempts the netcode chat; its effort line under MODELS-HOW (a) and (b) added; [to verify]
    covers `model: "fable"`, "on his personal account" added.
  - M6: K-11c (escalation category 3's gate files); K-12a, f and g labelled a widening (ROLLBACK-SCOPE, a new
    decision, at CS-F).
  - M7: until `required` counts, each change-set reads `toolchain`'s runs by hand.
  - M8: row 94 is a precondition of CS-C1 (a local smoke run), not dropped.
  - Lows: row 47 (PM-0002-64 is OQ-0002-8's test, its reading); `routine-guard.mjs:84`; `npm ci --ignore-scripts`;
    5.1 says how the scratch copy is made and counts its prompt; the checkpoint's consultation count.
  - Rulings folded: (a) OVERFLOW-XCRATE-CI's anchor moved (row 22, PM-0002-53); (b) the float ban, with its condition
    on 2.3-S; (c) `required`, with its three conditions (CS-E, CS-F); (d) the interim union plant (PM-0002-42).
  - Decisions: GATE-PATH (a)'s import cost; PIN-SCOPE (b)'s unprotected pins; MODELS-HOW quotes DECISIONS.md :91.

## Decisions needed
- **GATE-PATH: where does the gate v2's script live?**
  - Why it matters: the guard, the allow rules and the kit's command lines all name it, and CS-C builds it there.
  - Options:
    - (a) its own folder beside the legacy gate, `tools/gate-v2/` (run as `node tools/gate-v2/gate.mjs`). One more cost:
      a script there that imports a package by its bare name finds `tools/node_modules` first (acorn, eslint-scope,
      Playwright 1.56.1), not the root's, so the gate v2 imports no package by bare name and runs the root's tools by
      their path;
    - (b) it takes `tools/gate.mjs`, and the legacy gate moves;
    - (c) a new root folder `gate/`, which adds to the layout he accepted in ADR 0002 part 1.
  - Recommendation: (a). The fewest changes, and the legacy gate stays untouched (Part 7).
- **PIN-SCOPE: which v2 pins and configs should the guard protect (asks in a day session; routines and agents
  refused)?**
  - Why it matters: it decides whether every dependency change in later game chats asks him.
  - Options:
    - (a) everything that pins: (b)'s list plus the root `package.json`, `package-lock.json` and `Cargo.lock`. Every new
      dependency or version asks him, and no builder agent may make one;
    - (b) the gate's own rules and the toolchain pin: `rust-toolchain.toml`, the root `Cargo.toml` (its lint table,
      overflow setting and shared versions), `deny.toml`, `rustfmt.toml`, each crate's `clippy.toml`,
      `tsconfig.base.json`, `eslint.config.mjs`, `.prettierrc.json` and `.prettierignore`. Dependency changes stay with
      CI (`--locked`, G5's bans and sources, exact versions), with review and with his go at merge. It leaves
      unprotected the gate v2's own npm tool pins (the root `package.json` and lockfile, which hold ESLint, Prettier,
      TypeScript and Vitest) and `Cargo.lock`, while the legacy gate's `tools/package*.json` are gate files today;
    - (c) none beyond the gate v2's folder.
  - Recommendation: (b). Nobody can quietly weaken the gate's rules, and dependency work stays possible for builder
    agents, while his go at merge still sees every dependency change.
- **FREEZE-END: when does KIT-FREEZE-2 end?**
  - Why it matters: the kit queue (GUARD-SEC with PX-1 and W1 to W3, GUARD-DAY, and the rest) is frozen until then.
  - Options:
    - (a) when 2.3's first part has merged (CS-F);
    - (b) when the last later part of 2.3 joins, after the spikes, perhaps months away.
  - Recommendation: (a). The later parts only add gate checks, and the security follow-ups should not wait for the
    spikes.
- **MODELS-HOW: how should "the agents that review, verify or design game and engine code always run at max" be
  written?**
  - Why it matters: his words also say "The netcode chat on my university account keeps its models and efforts as they
    are", and it takes main's agent files when it merges main. And they say "Step 2.3 changes CLAUDE.md's "Models and
    effort" section and the agent files to match" (DECISIONS.md :91), which (c) meets for the agent files only after
    PR #40 merges.
  - Options:
    - (a) `effort: max` in those 12 agent files now, in CS-B. The netcode chat's agents then run at max after its next
      main merge;
    - (b) the agent files stay as they are, and CLAUDE.md tells every caller to pass `effort: "max"` to those agents;
    - (c) CLAUDE.md says (b) now, and the 12 agent files say `max` right after PR #40 merges (KIT-EFFORT).
  - Recommendation: (c). It keeps the netcode chat's efforts exactly as his words say, at the cost of the agent
    files' part of step 2.3 coming after #40. (a) keeps step 2.3 whole, but leaves the netcode chat to pass its old
    efforts itself.
- **ALLOW-GATE2: may `.claude/settings.json` allow the gate v2's command?** That is `node tools/gate-v2/gate.mjs`, with
  and without arguments, as it allows `node tools/gate.mjs` today.
  - Why it matters: without the rule, routines cannot run the gate v2 unattended, and day sessions are asked each time.
    But his words of 2026-11-11 were "Do not edit any settings file, hook or allow-list to get fewer prompts".
  - Options:
    - (a) yes, in CS-F, with the gate v2 never running package scripts;
    - (b) no: sessions answer the prompts, and routines do no v2 gate runs.
  - Recommendation: (a). The legacy gate has the same two rules, and the night shift will need the gate v2.
  - He can answer it now or at CS-F.
- **ROLLBACK-SCOPE: should a red gate v2 job on main count as "main breaks"?** "Main breaks" starts a revert with no
  plan step (CLAUDE.md :119-122; `advisor.md`, "Revert"). Today it means `gate` or `smoke` red on main.
  - Why it matters: K-12a, f and g, as drafted, make it "`required` red on main", which counts every gate v2 job. That
    is more cases where a day session reverts first: a widening his words must allow (Part 4 row 56, F5, has the same
    shape).
  - Options:
    - (a) yes: `required` red on main, after one re-run and pinned to one merge, is "main breaks";
    - (b) no: "main breaks" stays `gate` or `smoke` red, and a red gate v2 job on main gets a fix forward, plan first.
  - Recommendation: (a). After CS-F, `required` is the whole gate, and a red gate on main is what the rule is for.
  - He can answer it now or at CS-F.

## For Mustafa
1. Answer GATE-PATH, PIN-SCOPE, FREEZE-END and MODELS-HOW, and ALLOW-GATE2 and ROLLBACK-SCOPE if he wishes now.
2. Give or refuse his go on this plan, after the advisor agent's verdict, which the end message names.
3. If a docs chat on main should carry #52's leftovers before CS-A, say so. CS-A then keeps only this plan's record.

## Appendix K. The exact kit texts (generated by `kit-texts.py` from main 4460ef5)
Each "Before" is read from main 4460ef5 by line number, with its opening checked. Each quote of his in an "After" is
copied from `docs/DECISIONS.md` and checked against it. Lines marked "re-wraps when written" keep their words; the
line breaks are set when the change-set writes them. The texts are proposals: each is asked of him again, as a patch,
in its change-set (5.1).
Quotes of his words checked: 19, each found in DECISIONS.md once whitespace is joined.

### K-1 (CS-B): `CLAUDE.md` lines 20-23 on 4460ef5

Before:
```
- **Phase 0** (`docs/V2_PHASE0_BRIEF.md`): process upgrades (kit-v3-0), then research (`docs/research/`), then decision
  drafts (`docs/ENGINE_OVERVIEW.md`, `docs/adr/0001`–`0006`). No engine code: throwaway spikes only, and only after
  Mustafa and the Advisor have approved the drafts; until then, stop. Exception: foundation steps 2.1 and 2.2
  (Mustafa's decision of 2026-11-10 in `docs/DECISIONS.md`).
```

After:
```
- **Phase 0** (`docs/V2_PHASE0_BRIEF.md`): process upgrades (kit-v3-0), then research (`docs/research/`), then decision
  drafts (`docs/ENGINE_OVERVIEW.md`, `docs/adr/0001`–`0006`). No engine code: throwaway spikes only, and only after
  Mustafa and the Advisor have approved the drafts; until then, stop. Exception: foundation steps 2.1 and 2.2 (Mustafa's
  decision of 2026-11-10 in `docs/DECISIONS.md`), and for step 2.3 his decision of 2026-11-13 there: "The parts of step
  2.3 that need no proof test's result may be built before the proof tests report, after my go on the plan."
```

Looser by his exception, inside its own limits (his sentence quoted whole).

### K-2 (CS-B): `CLAUDE.md` lines 54-57 on 4460ef5

Before:
```
Decisions pending that may change these: the v2 decision drafts `docs/adr/0001`–`0006` (V2_PHASE0_BRIEF §4; they
replace ADR-001..003). Phase 0 writes no v2 code, except foundation steps 2.1 and 2.2 (Mustafa's decision of
2026-11-10 in `docs/DECISIONS.md`). Constraints for v2 code come only from those drafts once Mustafa accepts them.
Until then, the constraints above stand.
```

After:
```
Decisions pending that may change these: the v2 decision drafts `docs/adr/0001`–`0006` (V2_PHASE0_BRIEF §4; they replace
ADR-001..003). Phase 0 writes no v2 code, except foundation steps 2.1 and 2.2 (Mustafa's decision of 2026-11-10 in
`docs/DECISIONS.md`) and, for step 2.3, what his decision of 2026-11-13 there allows: "The parts of step 2.3 that need
no proof test's result may be built before the proof tests report, after my go on the plan." Constraints for v2 code
come only from those drafts once Mustafa accepts them. Until then, the constraints above stand.
```

Looser by his exception, inside its own limits (his sentence quoted whole).

### K-3 (CS-B): `.claude/agents/advisor.md` lines 107-108 on 4460ef5

Before:
```
- No engine code. Throwaway spikes only, and only after Mustafa and you have approved the decision drafts. Exception:
  foundation steps 2.1 and 2.2 (Mustafa's decision of 2026-11-10 in `docs/DECISIONS.md`).
```

After:
```
- No engine code. Throwaway spikes only, and only after Mustafa and you have approved the decision drafts. Exception:
  foundation steps 2.1 and 2.2 (Mustafa's decision of 2026-11-10 in `docs/DECISIONS.md`), and step 2.3 within his
  decision of 2026-11-13 there: "The parts of step 2.3 that need no proof test's result may be built before the proof
  tests report, after my go on the plan. Every gate check that needs a result joins later, when that result is in, with
  its pass marks unchanged."
```

Looser by his exception; the reviewer's line carries both his limits (built only without a result, after his go; checks that need a result join later with marks unchanged).

### K-4 (CS-B): `CLAUDE.md` lines 170-185 on 4460ef5

Before:
```
- **Opus 5.5 for everything that thinks:** architecture, code, reviews, research, plans and reports, every subagent
  included. Agent files say `model: opus`, which resolves to Opus 5.5 today (`claude-opus-5-5`; an `opus` agent reported
  it on 2026-10-25). When a newer Opus appears, Mustafa decides whether to move. Built-in agents (Explore, Plan,
  general-purpose) get `model: "opus"` passed explicitly; workflow agents inherit the session's model.
- **Effort is always stated** (Opus 5.5 defaults to medium): `high` by default; `xhigh` for architecture decisions
  (ADRs, `docs/ENGINE_OVERVIEW.md`, the synthesis) and netcode. Agent files set it in their frontmatter. The caller
  passes it for everything else: the Agent tool's `effort` for built-in agents and for a `high` agent working on an ADR
  or on netcode (`xhigh`); `effort: 'high'` or `'xhigh'` on every workflow `agent()` call (without it, a workflow agent
  takes the session's effort).
- **Sonnet 5.5 only for purely mechanical chores, and only when genuinely necessary:** file moves, formatting, link
  checks, summarizing logs. Opus remains the preference even there. No other model, except Fable 5.2 as the session's
  advisor through Claude Code's advisor tool, on Mustafa's personal account (his words below).
- **The advisor tool** (Mustafa's LANE-RULES, 2026-11-10: its Models line and his addition, word for word): "on my
  personal account, Fable 5.2 may be the session's advisor through Claude Code's advisor tool. The main model and every
  agent stay Opus 5.5." His addition: "because the advisor tool's advice cannot be read back, it is an extra. The
  recorded plan review by the advisor agent stays required."
```

After:
```
- **Who runs on which model** (Mustafa's words of 2026-11-12, 16:25, and what he means by them; word for word in
  `docs/DECISIONS.md`, the entry of 2026-11-12, 15:35:17 UTC; they replace LANE-RULES' "The main model and every agent
  stay Opus 5.5"):
  > - On my personal account, Fable 5.2 is the main model of every chat that writes game or engine code, and of every
  >   builder agent that chat starts to write that code.
  > - Every other agent runs on Opus 5.5 and may run at effort max. The agents that review, verify or design game and
  >   engine code always run at max.
  > - The guard, the gate and CI work stays with Opus 5.5.
  > - The netcode chat on my university account keeps its models and efforts as they are.
- **How:** agent files say `model: opus`, which resolves to Opus 5.5 today (`claude-opus-5-5`; an `opus` agent reported
  it on 2026-10-25). When a newer Opus appears, Mustafa decides whether to move. Built-in agents (Explore, Plan,
  general-purpose) get their model passed explicitly: `model: "fable"` for the builder agents of a chat on his personal
  account that writes game or engine code, `model: "opus"` for every other. Workflow agents inherit the session's model,
  so a chat on Fable 5.2 sets every workflow agent that is not a builder to Opus 5.5 [to verify before this text is
  asked: the Agent tool's model value for Fable 5.2, and the workflow API's model option].
- **Effort is always stated** (Opus 5.5 defaults to medium): `max` for every agent that reviews, verifies or designs
  game or engine code (his words above); until PR #40, the netcode chat, has merged, the caller passes `effort: "max"`
  to those agents, except in the netcode chat, which keeps its efforts; their agent files say `max` after that.
  Otherwise `high` by default; `xhigh` for architecture decisions (ADRs, `docs/ENGINE_OVERVIEW.md`, the synthesis) and
  netcode. Agent files set it in their frontmatter. The caller passes it for everything else: the Agent tool's `effort`
  for built-in agents and for a `high` agent working on an ADR or on netcode (`xhigh`); `effort: 'high'`, `'xhigh'` or
  `'max'` on every workflow `agent()` call (without it, a workflow agent takes the session's effort).
- **Sonnet 5.5 only for purely mechanical chores, and only when genuinely necessary:** file moves, formatting, link
  checks, summarizing logs. Opus remains the preference even there. No other model, except Fable 5.2 on Mustafa's
  personal account: the main model of a chat that writes game or engine code and of its builder agents (above), and the
  session's advisor through Claude Code's advisor tool (below).
- **The advisor tool** (Mustafa's LANE-RULES, 2026-11-10: the first sentence of its Models line, whose second sentence
  his words of 2026-11-12 replace, and his addition, word for word): "on my personal account, Fable 5.2 may be the
  session's advisor through Claude Code's advisor tool." His addition: "because the advisor tool's advice cannot be read
  back, it is an extra. The recorded plan review by the advisor agent stays required." His words of 2026-11-11 on it
  (`docs/DECISIONS.md`, the entry of 2026-11-11, 11:56:44 UTC; his bullets on the replaced model sentence and on
  recording them are left out), word for word:
  > They sharpen "it is an extra"; nothing else changes.
  > - "Extra" means on top of the recorded plan review by the advisor agent, never instead of it. It does not mean
  >   optional.
  > - When the advisor tool is on in this session, consult it every time at these moments:
  >   1. before you commit to a plan or an approach;
  >   2. when an error or a failed approach comes back a second time;
  >   3. before any change to rules, the gate, CI or security-relevant code;
  >   4. on a design call with more than one defensible answer;
  >   5. before you declare a step or the task done.
  > - Usage is not a reason to skip a consultation. Quality comes first.
  > - Its advice cannot be quoted into the repo, so record each consultation in the checkpoint in one line: when, your
  >   question, and what you changed because of it. Count them in your report.
  > - If the advisor tool is not on in this session, say so in one line and carry on; I will decide.
  [OPTIONAL A, not his words (the 2.3-docs report, finding 5): "Read which model answers it from the session's records
  (the transcript's advisor-tool entries), not from the `/advisor` command's output."]
  [OPTIONAL B, wider than his words, which say "From now on in this chat" (2026-11-14, item 2): "Consult it before
  you save a step as done, never after."]
```

Rules text to match his words of 2026-11-12 and 2026-11-11; nothing looser than his words (the optional lines are marked and his to refuse).

### K-4 (a) (CS-B): `CLAUDE.md` lines 174-178 on 4460ef5

Before:
```
- **Effort is always stated** (Opus 5.5 defaults to medium): `high` by default; `xhigh` for architecture decisions
  (ADRs, `docs/ENGINE_OVERVIEW.md`, the synthesis) and netcode. Agent files set it in their frontmatter. The caller
  passes it for everything else: the Agent tool's `effort` for built-in agents and for a `high` agent working on an ADR
  or on netcode (`xhigh`); `effort: 'high'` or `'xhigh'` on every workflow `agent()` call (without it, a workflow agent
  takes the session's effort).
```

After:
```
- **Effort is always stated** (Opus 5.5 defaults to medium): `max` for every agent that reviews, verifies or designs
  game or engine code (his words above), set in their agent files (K-5 in this change-set); the netcode chat keeps its
  efforts by passing each agent its former effort (`high`, or `xhigh` for engine-architect and netcode-architect) until
  PR #40 has merged. Otherwise `high` by default; `xhigh` for architecture decisions (ADRs, `docs/ENGINE_OVERVIEW.md`,
  the synthesis) and netcode. Agent files set it in their frontmatter. The caller passes it for everything else: the
  Agent tool's `effort` for built-in agents and for a `high` agent working on an ADR or on netcode (`xhigh`); `effort:
  'high'`, `'xhigh'` or `'max'` on every workflow `agent()` call (without it, a workflow agent takes the session's
  effort).
```

MODELS-HOW (a) only: the effort bullet; the agent files change in CS-B.

### K-4 (b) (CS-B): `CLAUDE.md` lines 174-178 on 4460ef5

Before:
```
- **Effort is always stated** (Opus 5.5 defaults to medium): `high` by default; `xhigh` for architecture decisions
  (ADRs, `docs/ENGINE_OVERVIEW.md`, the synthesis) and netcode. Agent files set it in their frontmatter. The caller
  passes it for everything else: the Agent tool's `effort` for built-in agents and for a `high` agent working on an ADR
  or on netcode (`xhigh`); `effort: 'high'` or `'xhigh'` on every workflow `agent()` call (without it, a workflow agent
  takes the session's effort).
```

After:
```
- **Effort is always stated** (Opus 5.5 defaults to medium): `max` for every agent that reviews, verifies or designs
  game or engine code (his words above): the caller passes `effort: "max"` to those agents, except in the netcode chat,
  which keeps its efforts. Otherwise `high` by default; `xhigh` for architecture decisions (ADRs,
  `docs/ENGINE_OVERVIEW.md`, the synthesis) and netcode. Agent files set it in their frontmatter. The caller passes it
  for everything else: the Agent tool's `effort` for built-in agents and for a `high` agent working on an ADR or on
  netcode (`xhigh`); `effort: 'high'`, `'xhigh'` or `'max'` on every workflow `agent()` call (without it, a workflow
  agent takes the session's effort).
```

MODELS-HOW (b) only: the effort bullet; the agent files keep their effort lines.

### K-6 (CS-B): `CLAUDE.md` lines 71-73 on 4460ef5

Before:
```
   (in auto mode, Claude Code's own classifier may refuse a `.claude/` edit before the prompt; make that edit in Accept
   edits or Manual mode); agents and teammates may not edit them. The ask is a seatbelt, not the wall: the PR review
   and Mustafa's go are the wall.
```

After:
```
   (in auto mode, Claude Code's own classifier may refuse a `.claude/` edit before the prompt; make that edit in Accept
   edits mode, the mode a cloud session offers besides Plan and Auto, or in a CLI's Manual mode, which hooks see as
   `default`); agents and teammates may not edit them. The ask is a seatbelt, not the wall: the PR review and Mustafa's
   go are the wall.
```

Wording only: the cloud app has no Manual mode (the environment's permissions page, 2026-11-14).

### K-7 (CS-B): `.claude/CHECKPOINTS.md` lines 219-224 on 4460ef5

Before:
```
- When a change-set is merged, remove its agent copies with `git worktree remove <path>`, never with `--force`. Git
  refuses a copy with changed or untracked files, but deletes ignored files (`.gate/`, builds, logs) and drops commits
  on a detached HEAD. Before removing: nothing to keep in `git -C <path> status --short --ignored`; its HEAD is on a
  pushed branch or inside main (`git merge-base --is-ancestor <HEAD> origin/main`); copies nested under it removed
  first; then `git worktree prune`. A copy with work to keep stays until that work is saved. The night shift's
  removals make the same checks.
```

After:
```
- When a change-set is merged, remove its agent copies with `git worktree remove <path>`, never with `--force`. Git
  refuses a copy with changed or untracked files, but deletes ignored files (`.gate/`, builds, logs) and drops commits
  on a detached HEAD. Before removing: nothing to keep in `git status --short --ignored`, run inside the copy (`cd
  <path>` first: the guard refuses `git -C`); its HEAD is on a pushed branch or inside main (`git merge-base
  --is-ancestor <HEAD> origin/main`); copies nested under it removed first; then `git worktree prune`. A copy with work
  to keep stays until that work is saved. The night shift's removals make the same checks.
```

Wording only: the command the guard accepts.

### K-8 (CS-B): `CLAUDE.md` lines 236-237 on 4460ef5

Before:
```
- Setup (once per machine, Node 22.13+): `cd tools && npm ci`, then inside tools/: `npx --no -- playwright install chromium`
  (the gate's smoke test 5a and pageshot need it)
```

After:
```
- Setup (once per machine, Node 22.13+): `cd tools && npm ci`, then from the root: `node
  tools/node_modules/playwright/cli.js install chromium` (the gate's smoke test 5a and pageshot need it; a session's
  settings deny `npx`, and allow this line)
```

Wording only: the form settings.json:12 allows (L-4).

### K-9 (CS-B): `CLAUDE.md` lines 127-130 on 4460ef5

Before:
```
before edits (a pure revert aside: Rollback), the advisor's APPROVE before anything merges to main, and Mustafa's Brave
check for anything a player sees, hears or feels. A session merges only on Mustafa's own go in its chat, about that PR
and for the PR's current head, given after the advisor approved that head (its verdict in this session, or its PR review
comment posted by the `Mergincanli` account and naming that full SHA), with the gate and smoke checks green, and passes
```

After:
```
before edits (a pure revert aside: Rollback), the advisor agent's APPROVE before anything merges to main, and Mustafa's Brave
check for anything a player sees, hears or feels. A session merges only on Mustafa's own go in its chat, about that PR
and for the PR's current head, given after the advisor agent approved that head (its verdict in this session, or its PR review
comment posted by the `Mergincanli` account and naming that full SHA), with the gate and smoke checks green, and passes
```

Wording only (SD-N2): the advisor agent, never the advisor tool, whose advice cannot be read back. The lines re-wrap when written.

### K-10a (CS-B): `.claude/agents/advisor.md` lines 17-17 on 4460ef5

Before:
```
- `docs/STATUS.md`: where things stand.
```

After:
```
- `docs/STATUS.md`: where things stand; the rest in `docs/STATUS_ARCHIVE.md`, his decisions word for word in
  `docs/DECISIONS.md`.
```

Wording only (status-diet's citations).

### K-10b (CS-B): `.claude/agents/advisor.md` lines 286-287 on 4460ef5

Before:
```
  2. If the merge touched kit or gate files, ask Mustafa before `git revert` runs (CLAUDE.md step 3): the guard does not
     read `git revert` (STATUS, limit L4). The main session runs `git revert`, never a subagent.
```

After:
```
  2. If the merge touched kit or gate files, ask Mustafa before `git revert` runs (CLAUDE.md step 3): the guard does not
     read `git revert` (`docs/STATUS_ARCHIVE.md`, "Open bugs and risks", kit-v3-3, limit L4). The main session runs `git revert`, never a subagent.
```

Wording only (status-diet's citations); re-wraps when written.

### K-10c (CS-B): `.claude/agents/netcode-architect.md` lines 23-24 on 4460ef5

Before:
```
game state inside THREE objects and DOM updates in `update(dt)`. docs/LEGACY_INVENTORY.md (R7, once it lands) lists what
it proves and the determinism debts STATUS records (wall-clock reads, shared `Math.random`).
```

After:
```
game state inside THREE objects and DOM updates in `update(dt)`. docs/LEGACY_INVENTORY.md (R7, once it lands) lists what
it proves and the determinism debts `docs/STATUS_ARCHIVE.md` records ("Open bugs and risks") (wall-clock reads, shared `Math.random`).
```

Wording only (status-diet's citations); re-wraps when written.

### K-12a (CS-F): `CLAUDE.md` lines 119-119 on 4460ef5

Before:
```
- **Rollback** (A4): if main breaks (`gate` or `smoke` red on main after one re-run, pinned to one merge; Mustafa
```

After:
```
- **Rollback** (A4): if main breaks (the required check `required` red on main after one re-run, pinned to one merge; Mustafa
```

A widening of "main breaks" (Rollback): a red gate v2 job on main then also starts a revert with no plan step; his words needed (ROLLBACK-SCOPE, asked at CS-F); re-wraps when written.

### K-12b (CS-F): `CLAUDE.md` lines 130-130 on 4460ef5

Before:
```
comment posted by the `Mergincanli` account and naming that full SHA), with the gate and smoke checks green, and passes
```

After:
```
comment posted by the `Mergincanli` account and naming that full SHA), with the required check `required` green (it needs the `gate` and `smoke` jobs and every gate v2 job), and passes
```

Same strength: `required` fails unless gate, smoke and every gate v2 job succeeded; re-wraps when written.

### K-12c (CS-F): `.claude/agents/advisor.md` lines 157-157 on 4460ef5

Before:
```
   only when `gate` and `smoke` are green on the approved SHA. How you establish that depends on the seat:
```

After:
```
   only when the required check `required` is green on the approved SHA (it needs `gate`, `smoke` and every gate v2 job). How you establish that depends on the seat:
```

Same strength: `required` fails unless gate, smoke and every gate v2 job succeeded; re-wraps when written.

### K-12d (CS-F): `.claude/agents/advisor.md` lines 163-163 on 4460ef5

Before:
```
   - **As the PR Advisor routine** (no interpreter, no sim, no render): read `gate` and `smoke` from CI's check runs on
```

After:
```
   - **As the PR Advisor routine** (no interpreter, no sim, no render): read `required` from CI's check runs on
```

Same strength: `required` fails unless gate, smoke and every gate v2 job succeeded; re-wraps when written.

### K-12e (CS-F): `.claude/agents/advisor.md` lines 224-224 on 4460ef5

Before:
```
  establishes it in your seat (a day session runs it itself; the routine reads CI's `gate` and `smoke`, and only where
```

After:
```
  establishes it in your seat (a day session runs it itself; the routine reads CI's `required`, and only where
```

Same strength: `required` fails unless gate, smoke and every gate v2 job succeeded; re-wraps when written.

### K-12f (CS-F): `.claude/agents/advisor.md` lines 273-273 on 4460ef5

Before:
```
- **Main breaks** when one of these holds: `gate` or `smoke` is red on a main commit, still red after one re-run, and
```

After:
```
- **Main breaks** when one of these holds: `required` is red on a main commit, still red after one re-run, and
```

A widening of "main breaks" (Rollback): a red gate v2 job on main then also starts a revert with no plan step; his words needed (ROLLBACK-SCOPE, asked at CS-F); re-wraps when written.

### K-12g (CS-F): `.claude/agents/advisor.md` lines 294-294 on 4460ef5

Before:
```
    holds: the red `gate` or `smoke` run on main names that commit and its re-run is red too, or the report is
```

After:
```
    holds: the red `required` run on main names that commit and its re-run is red too, or the report is
```

A widening of "main breaks" (Rollback): a red gate v2 job on main then also starts a revert with no plan step; his words needed (ROLLBACK-SCOPE, asked at CS-F); re-wraps when written.

### K-12h (CS-F): `.claude/agents/advisor.md` lines 323-323 on 4460ef5

Before:
```
- Merge with failing or pending checks, or unless the `gate` and `smoke` jobs both passed on the approved SHA.
```

After:
```
- Merge with failing or pending checks, or unless the required check `required` passed on the approved SHA.
```

Same strength: `required` fails unless gate, smoke and every gate v2 job succeeded; re-wraps when written.

### K-14 (CS-B): `.claude/CHECKPOINTS.md` lines 128-128 on 4460ef5

Before:
```
  from your notes alone, opening no other file. Never commit or push.
```

After:
```
  from your notes alone, opening no other file. Never commit or push. Write files with the Write or Edit tool, or
  with one plain command whose target is a literal path: the guard refuses `git -C`, a write after `cd`, and a
  write to a path held in a variable or built in a loop.
```

Stricter-neutral guidance (fewer refusals); the paste block re-wraps when written.

### K-6b (CS-D): `CLAUDE.md` lines 71-73 on 4460ef5

Before:
```
   (in auto mode, Claude Code's own classifier may refuse a `.claude/` edit before the prompt; make that edit in Accept
   edits or Manual mode); agents and teammates may not edit them. The ask is a seatbelt, not the wall: the PR review
   and Mustafa's go are the wall.
```

After:
```
   (in Auto mode the guard refuses a kit or gate edit outright, and Claude Code's own classifier may refuse a `.claude/`
   edit before the prompt: switch the chat to Accept edits first, the mode a cloud session offers besides Plan and Auto,
   or use a CLI's Manual mode, which hooks see as `default`); agents and teammates may not edit them. The ask is a
   seatbelt, not the wall: the PR review and Mustafa's go are the wall.
```

Wording for G-3 (CS-D); replaces K-6 once G-3 is built. Stricter (the guard refuses where it asked).

### K-11a (CS-D): `CLAUDE.md` lines 68-69 on 4460ef5

Before:
```
   (the guard's protected files: `.claude/` except agent memory, every `CLAUDE.md`, `CLAUDE.local.md` and `AGENTS.md`,
   `.github/`, `tools/gate.mjs` and the other gate files) made with a file tool or a shell write asks Mustafa first, in
```

After:
```
   (the guard's protected files: `.claude/` except agent memory, every `CLAUDE.md`, `CLAUDE.local.md` and `AGENTS.md`,
   `.github/`, `tools/gate.mjs` and the other gate files, the gate v2's folder `tools/gate-v2/`, and the v2 pins and
   gate configs at the root: `rust-toolchain.toml`, `Cargo.toml`, `deny.toml`, `rustfmt.toml`, `tsconfig.base.json`,
   `eslint.config.mjs`, `.prettierrc.json`, `.prettierignore`, and each crate's `clippy.toml`) made with a file tool or
   a shell write asks Mustafa first, in
```

Stricter (more protected paths); the list follows GATE-PATH (a) and PIN-SCOPE (b); re-wraps when written.

### K-11b (CS-D): `.claude/agents/advisor.md` lines 164-166 on 4460ef5

Before:
```
     the approved SHA. That green stands only when the PR touches neither `.github/` nor the gate's own files
     (`tools/gate.mjs`, `smoke.mjs`, `battery.js`, `gate-globals.json`, `package*.json`, `npm-shrinkwrap.json`, any
     `.npmrc`) — otherwise CI
```

After:
```
     the approved SHA. That green stands only when the PR touches neither `.github/` nor the gate's own files
     (`tools/gate.mjs`, `smoke.mjs`, `battery.js`, `gate-globals.json`, `package*.json`, `npm-shrinkwrap.json`, any
     `.npmrc`; the gate v2's folder `tools/gate-v2/`; the v2 pins and gate configs the guard protects) — otherwise CI
```

Stricter (more files make the PR Advisor's CI green insufficient); re-wraps when written.

### K-11c (CS-D): `.claude/agents/advisor.md` lines 39-43 on 4460ef5

Before:
```
3. **The rules themselves:** changes to hard constraints, ADRs, `CLAUDE.md`, `.claude/` (this file included), CI
   (`.github/`), security rules, or the gate's own files: every file `tools/gate.mjs` or `.github/workflows/` run as
   code or read as settings, not the game files they check (today `tools/gate.mjs`, `tools/smoke.mjs`,
   `tools/battery.js`, `tools/gate-globals.json`, `tools/package.json`, `tools/package-lock.json`,
   `tools/npm-shrinkwrap.json`) and any `.npmrc`.
```

After:
```
3. **The rules themselves:** changes to hard constraints, ADRs, `CLAUDE.md`, `.claude/` (this file included), CI
   (`.github/`), security rules, or the gate's own files: every file `tools/gate.mjs`, the gate v2 (`tools/gate-v2/`) or
   `.github/workflows/` run as code or read as settings, not the game files they check (today `tools/gate.mjs`,
   `tools/smoke.mjs`, `tools/battery.js`, `tools/gate-globals.json`, `tools/package.json`, `tools/package-lock.json`,
   `tools/npm-shrinkwrap.json`, the gate v2's folder and the v2 pins and gate configs the guard protects) and any
   `.npmrc`.
```

Stricter (escalation category 3 names the gate v2 and the protected pins); follows GATE-PATH (a) and PIN-SCOPE (b).

### K-13a (CS-F): `CLAUDE.md` lines 74-75 on 4460ef5

Before:
```
4. Gate: `/gate` must PASS (syntax, concatenation, symbol conservation, player-build resolvability, and check 5: smoke,
   names, layering, script order), plus evidence:
```

After:
```
4. Gate: `/gate` must PASS (syntax, concatenation, symbol conservation, player-build resolvability, and check 5: smoke,
   names, layering, script order), and for v2 code (`crates/`, `packages/`, the root pins) the gate v2, `node
   tools/gate-v2/gate.mjs`, must PASS too, plus evidence:
```

Stricter (a second gate for v2 code).

### K-13b (CS-F): `CLAUDE.md` lines 238-239 on 4460ef5

Before:
```
- Gate: `node tools/gate.mjs [baseRef] [--allow-drop a,b] [--no-smoke]` (checks 1–4 and 5a–5d vs the base ref, default:
  merge-base with main or origin/main, whichever is newer; CI runs it)
```

After:
```
- Gate: `node tools/gate.mjs [baseRef] [--allow-drop a,b] [--no-smoke]` (checks 1–4 and 5a–5d vs the base ref, default:
  merge-base with main or origin/main, whichever is newer; CI runs it)
- Gate v2 (v2 code): `node tools/gate-v2/gate.mjs [baseRef] [--allow-removed a,b] [--quick] [--prove] [--only G1,G2]
  [--json]` (checks G0-G13 as built; exits 0, 1 or 2; CI runs it inside the required check `required`)
```

A new command line beside the legacy gate's.

### K-13c (CS-F): `.claude/agents/verifier.md` lines 21-21 on 4460ef5

Before:
```
   (This is the legacy gate; the gate v2 arrives in phase 1 and replaces it for v2 code.)
```

After:
```
   (This is the legacy gate. For v2 code also run the gate v2, `node tools/gate-v2/gate.mjs <baseRef>`, and its
   `--prove` when the change touches the gate, its checks or its configs.)
```

Stricter (the verifier runs the gate v2 for v2 code).

### K-13d (CS-F): `.claude/skills/gate/SKILL.md` lines 6-6 on 4460ef5

Before:
```
Run `node tools/gate.mjs $ARGUMENTS` and read the whole output.
```

After:
```
Run `node tools/gate.mjs $ARGUMENTS` and read the whole output. When the change touches v2 code (`crates/`,
`packages/`, the root pins), also run `node tools/gate-v2/gate.mjs $ARGUMENTS` and read its whole output.
```

Stricter (the skill runs both gates for v2 code).

