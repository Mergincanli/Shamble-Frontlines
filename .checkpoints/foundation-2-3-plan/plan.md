# Plan: foundation step 2.3, the gate v2 and CI, and the end of the kit's freeze (revision 1)

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
- **The cut.** Step 2.3 splits in two. Its first part is seven change-sets. They are built after your go on this plan,
  before the proof tests report, one pull request at a time, in this order:
  1. **CS-A, docs** (decision docs): STATUS and ROADMAP made true for main 4460ef5; your words of 2026-11-14 copied into
     `docs/DECISIONS.md` by script; your go on this plan, with your answers.
  2. **CS-B, rules text** (CLAUDE.md, three agent files, CHECKPOINTS.md): your model rule of 2026-11-12, your
     advisor-tool words of 2026-11-11, the three lines that name 2.1 and 2.2 as the only exception, and seven small
     fixes.
  3. **CS-C1, the legacy gate:** six findings in `tools/gate.mjs` and `tools/smoke.mjs`.
  4. **CS-D, the guard:**
     - it protects the gate v2's folder and the pins you choose (PIN-SCOPE);
     - it refuses kit and gate edits in Auto mode, where today it only asks (AUTO-ASK).
  5. **CS-C, the gate v2's first part:**
     - checks G0, G1, G2, G3, G5, G6 and G9, each in part;
     - `--prove`, the run that plants defects to show each check catches its own.
  6. **CS-E, CI:**
     - one workflow, whose final job `required` is the one check you require;
     - it holds the legacy gate and smoke jobs too;
     - the network cut stays around the tests, hardened.
  7. **CS-F, the switch:** the guard and the rules name `required`. You add it to your required checks between CS-E and
     CS-F, and remove `gate` and `smoke` after CS-F.
- **The rest of step 2.3 joins later,** each piece when its result is in, with its pass marks unchanged:
  - **2.3-B** rides step 2.4, after the boundary spike: G4, the boundary layer of G9, G7 and the budget file.
  - **2.3-S**, its own change-set after the sim core spike: G2's float ban and G5's same-feature check.
  - **2.3-E** rides step 2.5, after the netcode proof test and the sim core spike: G6's Vite build, G8, G10, G11, G12.
  - **G13** rides slice 2, as ADR 0002 already places it.
- **Marks, read strictly** (every clause holds):
  - The first part fully meets only PM-0002-40 (with your one Windows run), PM-0002-54, PM-0002-57, and D2, D4, D6 and
    D7.
  - Every other mark is met when its last clause joins, at a named change-set. Its built clauses run from CS-C on.
  - No mark is dropped or loosened (Part 2).
  - Several marks name things that arrive only after the foundation: `server` (slice 3), the editor (slice 5),
    legacy goldens (slice 1). Those clauses join there, so PM-0002-41, -42, -46 and -47 are met only then.
- **The waiting items** (Part 4, 107 entries in one table, one row per item and source):
  - 54 are carried by the first part.
  - 39 come later, each with its time.
  - 8 are dropped, each with its reason.
  - 4 are not a session's edit, and 2 are rules for this chat.
  - Leaning to the smallest step, as you asked: the old kit follow-ups go to a kit queue after 2.3. GUARD-SEC comes
    first, with PX-1 (the merge check that trusts CI results by their name alone) and W1 to W3 (gaps in what the guard
    lets routines and agents write).
- **Your part in the first part** (Part 9):
  - your go on this plan;
  - six exact-text asks (CS-B, C1, D, C, E, F). Each is a patch at a pushed checkpoint commit, approved by naming the
    commit's SHA and written in Accept edits, with about 2 to 21 guard prompts each, about 41 in all;
  - seven merge gos;
  - two ruleset edits on GitHub;
  - one scratch branch;
  - one Windows run.
  Each change-set still gets its own plan, the advisor agent's plan review, review rounds within the budget, the
  verifier, and the advisor agent's review of its final head.
- **Decisions needed:**
  - GATE-PATH: where the gate v2's script lives (item 7);
  - PIN-SCOPE: which v2 pins and configs the guard protects;
  - FREEZE-END: when KIT-FREEZE-2 ends;
  - MODELS-HOW: effort max set in the agent files now, or after PR #40;
  - ALLOW-GATE2: an allow rule for the gate v2's command.
- **#52's leftovers are not on main 4460ef5:**
  - STATUS :5, :13, :29, :36 and :43, and ROADMAP :117, still show #52 open;
  - `docs/DECISIONS.md` has no entry of 2026-11-14;
  - they are CS-A's, unless a docs chat on main carries them first.
- **PR #40** has nothing to do now. It merges main before its own merge, and before your ruleset's `required` matters
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
  - **Rule 3, code from a later step.** A check whose subject arrives only with a later step (2.4, 2.5) joins with that
    step. A check with nothing to check would be a feature "for later", which CLAUDE.md rules out.
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
| **2.3-S** | the sim core spike's checks | the sim core spike's report merged (ADR 0004 part 7's final list, sim's dependency set, R5-LAYOUT-REC4's result) | step 2.3's own change-set, built right after that report |
| **2.3-E** | the skeleton checks | the netcode proof test's and the sim core spike's reports merged, and step 2.5's code | rides step 2.5's change-set (2.5's plan may put it in a PR just before) |

Clauses that name a crate or a package that enters later join with the change-set that brings it, as the marks say ("from
slice 1", "each when that crate enters") or as their nouns require: `server` at slice 3, `navbake` at slice 2, the
editor at slice 5. Run counts and machines are part of each mark.

| Line or mark | Its clauses | First part | Joins later | Met at |
|---|---|---|---|---|
| D1 | `--prove` passes; every mutation fails exactly its named checks; every control passes | CS-C, for the built checks | each check's mutations as it joins | 2.3-E (the last of G0-G12 joins), as PM-0002-52 |
| D2 | a TypeScript-only pull request runs and passes the single required check | CS-E (a scratch pull request) | | **CS-E** |
| D3 | warm pull-request wall time at most 10 min at p50 over 5 runs; at most 20 min cold | CS-E measures and reports it | the full workflow | 2.3-E, as PM-0002-56 |
| D4 | a failing and a cancelled job each turn the required check red | CS-E | | **CS-E** |
| D5 | a fork pull request runs every job with no secret | | step 2.6, where the first secret enters (PM-0002-58 covers part 10 too) | step 2.6 |
| D6 | the repository's Rust cache at or below 8 GB | CS-E (3 runs) | re-measured at 2.3-E | **CS-E** |
| D7 | Mustafa names the required check in his branch protection | his ruleset change after CS-E | | **CS-E** (his setting) |
| PM-0002-39 (G0) | toolchain file deleted: G0 exit 2, no other Rust; `Cargo.toml` edited without `Cargo.lock` fails at the first cargo step; missing Playwright browser: exit 2 with the setup command; control: a consistent toolchain bump passes | CS-C: the first, second and fourth, plus the G0 finding's source-check mutations | the missing-browser clause (2.3-E) | 2.3-E |
| PM-0002-40 (G1) | misformatted `.rs`, CRLF `.rs`, misformatted `.ts` each fail G1 only; control: long string literals pass; after `prettier --write`, ESLint reports 0; the formatted tree is byte-identical on Windows and Linux (once, his PC) | CS-C, and his Windows run (Part 9) | | **CS-C** + his run |
| PM-0002-41 (G2) | the planted defects ADR 0002 lists at :1307, each failing G2 only with its reason; controls in `server`, in a `#[test]`, and sim's min/max/clamp helpers with `abs` and `fract` | CS-C: HashMap field and `map.values().sum()` in sim; `Instant::now()` in a tick function, in room and in client-endpoint; a correctness bug; a style warning; `unsafe`; a crate without `[lints] workspace = true`; `+fma`, `-C target-cpu=native`, `relaxed-simd` in the build flags; control `.unwrap()` in a `#[test]` | `sin`, `powi`, `to_radians`, `min`, and the helper controls (2.3-S); `.unwrap()` and `buf[i]` in a decoder (2.3-E); `-ffm` in `wasm-opt` (2.3-B); the `server` control (slice 3) | slice 3 |
| PM-0002-42 (G3) | one violation per strict flag; `eval` and `new Function` in client; an unawaited promise; a switch over a generated union missing one kind; an import of editor from client; a bare `event`; DOM code imported by sim-worker or net; an import from `legacy/` or `spikes/`; controls: `import type` from types in editor passes; the clean tree is 0 and 0 | CS-C: all but the generated union and the two editor clauses | generated union (2.3-B); the editor clauses (slice 5) | slice 5 |
| PM-0002-43 (G4) | all | | 2.3-B | 2.3-B |
| PM-0002-44 (G5) | transport crate or `tokio`/`quinn` in sim; `tokio` in room; `navbake` or `server` reachable from sim-wasm (each when that crate enters); wasm-bindgen or a transport crate in client-endpoint; a git dependency; a wildcard; a second version off the skip list; a server-only feature on a shared maths dependency (feature lists differ across server, devtools, navbake, sim-wasm); controls: proptest as sim's dev-dependency, a dev-only feature on devtools | CS-C: all but the feature comparison and the two crates that enter later; how each dependency mutation keeps `Cargo.lock` consistent offline is CS-C's design (local stand-in crates by name, or the network in CI's `--prove` job) | the feature comparison (2.3-S); navbake (slice 2) and server (slice 3), as the mark says | 2.3-S, and its own per-crate clauses as those crates enter |
| PM-0002-45 (G7) | all | | 2.3-B | 2.3-B |
| PM-0002-46 (G8) | all | | 2.3-E; the editor's clauses (slice 5) | slice 5 |
| PM-0002-47 (G9 layers) | a broken property; a golden beyond tolerance (from slice 1); a changed insta snapshot; a byte swap in one encoder; a page error in deploy; control: an equal-output refactor | CS-C: the control, for `cargo test` and Vitest | the boundary layer (2.3-B), the end-to-end clause (2.3-E), the property and insta clauses with the first such test, the golden (slice 1) | slice 1 |
| PM-0002-48 (G10) | three planted divergences reported at the first tick; a control; G10 under 3 min over 5 runs | | 2.3-E | 2.3-E |
| PM-0002-49 (L1) | the same seed gives an identical fate log and hashes twice; over 1,000,000 packets per profile, loss, delay and jitter within 5 %; G11 under 2 min | | 2.3-E (L1 enters behind the loopback at 2.5) | 2.3-E |
| PM-0002-50 (G11) | all | | 2.3-E | 2.3-E |
| PM-0002-51 (G12) | throw in deploy; console error; outbound request; WebSocket broken (from slice 3); +20 % draw calls against ADR 0003's baseline; control; the `forceWebGL` run and the backend named | | 2.3-E; WebSocket (slice 3, as the mark says); the draw-call clause when ADR 0003's count baseline is committed | slice 3, and that baseline |
| PM-0002-52 (`--prove`) | every mutation of PM-0002-39 to -51; one mutation and one control per id G0-G12; under 30 min | CS-C, for the built ids | as each check joins | 2.3-E (all of G0-G12); later mutations join it as their crates enter |
| PM-0002-53 (overflow, 4 builds) | a planted counter panics in dev and release, native and Wasm; `wrapping_add` agrees in all four; the cost reported | | the anchor of OVERFLOW-XCRATE-CI (Part 4): its Wasm half needs a Wasm runner, which comes with wasm-bindgen at 2.4 | 2.3-B at the earliest |
| PM-0002-54 (lint and type speed) | `tsc -b` under 30 s, typed lint under 60 s, clippy under 30 s warm and 3 min cold; 5 runs each | CS-C and CS-E (5 runs each, from the job logs) | | **CS-E** |
| PM-0002-55 (gate speed) | `--quick` under 60 s warm; the full gate under 10 min warm, on Linux and on Windows; 5 runs per system | measured and reported at CS-C (not the full gate yet) | the full gate (2.3-E) and his Windows runs | 2.3-E |
| PM-0002-56 (CI speed) | as D3, per job and in total | measured and reported at CS-E | the full workflow | 2.3-E |
| PM-0002-57 (one required check) | a TypeScript-only pull request passes; a failing, a cancelled and a skipped job each turn it red and block the merge; removing a check fails the aggregate | CS-E (scratch pull requests, planted failures on CS-E's own pull request) | | **CS-E** |
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
  - K-4: "Models and effort", with his words of 2026-11-12 and of 2026-11-11.
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
- **Reviews:** security-auditor and test-engineer, then the advisor agent.
- **His asks:** one. **Protected writes:** 2, so about 2 prompts. **His go:** the merge.

### CS-D: the guard (risk tier 2: `.claude/hooks/routine-guard.mjs` and its tests; CLAUDE.md and advisor.md for the lists)
- **Guard changes** (three lines each in Part 6):
  - G-1, PIN-GUARD, by PIN-SCOPE;
  - G-2, the gate v2's folder, by GATE-PATH;
  - G-3, AUTO-ASK;
  - the kit lists that name the protected files: K-6b, K-11a and K-11b.
- **Why it comes before CS-C:** the gate v2's files are then protected from their first commit. Without it, a routine
  could write them on a branch, a day session would get no ask, and the PR Advisor could merge them, since `tools/` is
  one of its merge places (`routine-guard.mjs:87`).
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
  - Before building, this plan re-reads the hooks page itself, since the agent's reading (F-HOOK) is not yet the
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
  - PM-0002-57's checks: a TypeScript-only scratch pull request; planted failing, cancelled and skipped jobs and a
    removed check on CS-E's own pull request, each red and blocking, then reverted exactly;
  - a red run with `--net` removed, ending in the probe's "still reachable" line (CUT-CARRY);
  - SETPRIV-HARDEN's assertions in the log;
  - timings for D3 and PM-0002-56 (5 warm, 3 cold), reported;
  - the cache size for D6;
  - the legacy jobs diffed against `gate.yml`'s: equal but for the named fixes.
- **Reviews:** security-auditor, infra-engineer and test-engineer, then the advisor agent.
- **His asks:** one. **Protected writes:** 3 (`ci.yml` written; `gate.yml` and `toolchain.yml` removed), so about 3
  prompts. **His go:** the merge.
- **Then his ruleset step:** add `required` once its first push run on main is green (Part 8).

### CS-F: the switch (risk tier 2: the guard's merge check; rules text; settings)
- **Edits:**
  - G-4: the guard's merge check requires `required`, `gate` and `smoke`, plus its comment at `routine-guard.mjs:6`, its
    message at `:990`, and its tests;
  - K-12a to K-12h: the lines naming `gate` and `smoke` as the required checks;
  - K-13a to K-13d: the lines naming the gate's command learn the gate v2 for v2 code;
  - S-1, by ALLOW-GATE2: the allow rule;
  - the docs: STATUS (REQUIRED-CHECK closed), and DECISIONS.md's "Queued" (KIT-FREEZE-2 leaves, by FREEZE-END (a)).
- **Evidence:**
  - the guard's tests: no `required` refuses; `required` pending or red refuses; all three green allows;
  - a grep: no kit line still names `gate` and `smoke` alone as the required checks;
  - the settings test.
- **Reviews:** security-auditor and test-engineer, then the advisor agent.
- **His asks:** one. **Protected writes:** 7 (the guard, its tests, CLAUDE.md, advisor.md, verifier.md, the gate skill,
  settings.json), so about 7 prompts. **His go:** the merge.
- **Then his ruleset step:** remove `gate` and `smoke` from the required checks. They still run inside `required`.

### First part, in sum
- 7 change-sets.
- 6 exact-text asks.
- About 41 guard prompts by PIN-SCOPE (b): 4 + 2 + 4 + 21 + 3 + 7.
- 7 merge gos.

### The later parts
- **2.3-B** rides step 2.4: G4, G9's boundary layer and Wasm tests, G7 with the budget file, PM-0002-60, the
  `wasm-opt` flag clause, the Wasm half of PM-0002-53, and WASM-ARTIFACT.
- **2.3-S**, its own change-set after the sim core spike: the float ban, the map types from sim's dependencies, the
  same-feature check and FEATURE-UNIFY.
- **2.3-E** rides step 2.5: G6's Vite build, G8, G10, G11 with L1, G12, the decoder panic lints, the missing-browser
  clause, TWO-PLAYWRIGHTS, and the full-gate runs for PM-0002-52, -55, -56, -59 and -61.
- **G13** rides slice 2.
- **Each is planned when its trigger is met.** None is built before its result is in. Each keeps its marks unchanged.
