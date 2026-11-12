# engine-architect: review round 1 of foundation step 2.2 (PR #48)

Head under review: 4d78ad61e652f2e35ddf51c49557e725420248c8 (worktree HEAD 4c0dfd5 differs only in .checkpoints/,
checked: `git diff --name-only 4d78ad6 HEAD` lists only .checkpoints files). Base: origin/main 2aa5e9c.
Memory: none found (no agent-memory folder anywhere on disk); started fresh.

## Plan
1. Read the report draft (docs/reports/2026-11-12-foundation-2-2.md), then ADR 0002 parts 2, 3, 4, 7, 9 and the
   migration plan step 2.2; the plan's crate/package sections and the empty slots it names (OQ-0002-7, -3, -4, -1,
   boundary spike items 4 and 8).
2. List the diff (`git diff origin/main...4d78ad6 --stat -- . ':!.checkpoints'`), read every Cargo.toml, lib.rs,
   package.json, tsconfig, workspace file.
3. Check against ADR 0002 part 3: crate and package names, what each holds (doc comments), the dependency direction
   each manifest allows (no upward edge: sim depends on nothing above it; client/server depend on sim; tools on all).
4. Check the empty slots: each named slot exists, is empty, and says which OQ/spike item fills it; nothing filled
   early (no generic-engine feature for later).
5. Check the Rust and TypeScript boundary the skeleton leaves (wasm crate, generated types path, worker, shared
   memory): what is fixed now, what is left open, any hidden coupling or decision taken early.
6. Mechanical checks: cargo metadata dependency graph (offline), npm workspace graph, a mutation to see whether any
   check catches an upward dependency (in my own folder under .gate/).
7. Record findings below, one line each as found; then the verdict.

## Done checks
- C1 report read (docs/reports/2026-11-12-foundation-2-2.md, 387 lines); ADR 0002 parts 1-5, 7, 9, OQs, spike items, migration plan 2.2-2.5 read; plan Parts 1-3 and 6 read.
- C2 diff listed: 42 files outside .checkpoints; every crate manifest, lib.rs, package.json, stub source, toolchain.yml, .gitignore, REPO_MAP, adr/README diff read at 4d78ad6.
- C3 cargo metadata on a copy of the head (/home/user/Shamble-Frontlines/.gate/engine-architect-r1, git archive of 4d78ad6): 6 members, 0 dependencies, all crate type lib. Names match ADR part 3 and migration plan 2.2 exactly; packages match part 3 (client, sim-worker, net, types, e2e).
- C4 experiment: a lone crate under spikes/ with no [workspace] table fails: "current package believes it is in a workspace when it is not" (cargo 1.99.1). PR #40 head 8eb5d09 (API tree): spikes/netcode-proof has its own [workspace]; its two .checkpoints scratch crates carry [workspace] too.
- C5 experiment: [lints] workspace = true plus [lints.clippy] pedantic in sim: cargo 1.99.1 refuses: "cannot override workspace.lints in lints, either remove the overrides or lints.workspace = true and manually specify the lints". Restored; scratch manifest identical to head.
- C6 ADR 0004 migration step 2 (0004:1252): the foundation gives sim code (hash-map ban, replay runner, first recording); ADR 0002 migration 2.5 gives protocol/room/client-endpoint code (Join..Snapshot over the loopback).

- C7 CI on the head 4d78ad6: toolchain push 43312017234 (job 131510223737) and pull_request 43312027194, gate 43312016939 and 43312026745: all success. Job log: uid 1001 group nogroup, curl exit 7 "the network is cut", 6 unit tests ok, release sim + protocol ok, wasm32 dev build of 5 crates, vitest 4x1 + playwright 1, tsc 6.0.3.
- C8 local gate in the worktree: node tools/gate.mjs origin/main --no-smoke: GATE PASS, symbols +0 -0 (working tree = head outside .checkpoints).
- C9 lone crate under a nested .gate/ fails the same way; with exclude = [".gate", ".checkpoints"] in the copy root: the lone crate builds, members stay 6, cargo test --workspace --locked passes (lock unchanged). Copy restored, identical to head.
- C10 other reviewers notes skimmed for overlap: RR-2 (rust-reviewer) already has the lints + pedantic refusal (my C5): not re-reported. RR-1 (overflow in callers) not duplicated.
- C11 ADR 0002 static import rules (0002:207-209, :424, PM-0002-42, -46): no rule that client never imports net or the Wasm glue; page-side bans are runtime only (spike item 1, PM-0002-22).
- C12 OQ-0002-8 (a) (register :74, 0002:1100-1102): "the first foundation milestone that builds them is their test"; the report and docs are silent on whether 2.2 counts.

- C13 package-lock.json (node): 5 workspace links; every workspace dependency list empty; root devDependencies exactly typescript 6.0.3, vitest 5.1.0, @playwright/test 1.65.0; 64 registry packages; no three.js, no direct vite, no eslint/prettier/typescript-eslint, no wasm tooling. vite 8.3.7 + rolldown only as vitest peer (^6.4.0||^7.0.0||^8.0.0).
- C14 upward-edge mutation sim -> room in the copy: --locked fails on the old lock; lock regenerated: cargo test --workspace --locked 12 ok lines, wasm32 build ok, release tests ok. Nothing in 2.2 enforces part 3 direction (G5 at 2.3, as planned). Restored identical.
- C15 root .cargo/config.toml leak test in the copy: [build] target-dir and [env] TS_RS_EXPORT_DIR reach a nested spikes/ws-spike with its own [workspace]: metadata target_directory = leak-target; test sees TS_RS_EXPORT_DIR=.../packages/types/src/generated. Config moved aside (cargo-config-leaktest).
- C16 PR #40 compare 1efd46f...8eb5d09: ahead 8, 23 files, all under .checkpoints/: E4b/E4d inputs outside .checkpoints unchanged; spike .cargo/config.toml is target-dir only. check.mjs not run (unreviewed code). infra-engineer noted the same as "not verified".
- C17 register (docs/adr/README.md) lists OQ-0002-1, -4, -5, -6, -8, -9; no OQ-0002-7 row: its by-day decider lives only in ADR 0002:1082-1084.
- C18 advisor consultation 1 (after orientation): tightened EA-3 to an omission (only sim-wasm lib.rs:2 misattributes), EA-4 as my reading, EA-5 as proposed for 2.3 without the Vite mechanism; added C13-C16; fixes mind report :116-119 (unchanged since 566363f) and plan Part 6 "no exclude".

## Findings (final texts)
- EA-1 Low Cargo.toml:2-4: root [workspace] reaches every Cargo package below the root, unrecorded. (a) now: lone packages (future spikes before 2.3; .gate/ and .checkpoints/ scratch crates) fail without their own [workspace] (C4, C9). (b) for 2.4: ADR 0002:345-346 puts TS_RS_EXPORT_DIR in .cargo/config.toml; a root one reaches nested workspaces (C15) and fails the spike cargoConfigFiles check plan Part 6 cites; no follow-up carries it. Fix: follow-up lines now (report :181-191, archive From foundation 2.2); exclude [".gate", ".checkpoints"] as a 2.3 option (tested C9) or now with CI and :116-119 reworded.
- EA-2 Low crates/room/src/lib.rs:2 (+ sim:2, client-endpoint:1-2, REPO_MAP:18): "Empty until its first slice" vs ADR 0002:1467-1472 and ADR 0004:1252 (foundation); client-endpoint doc and REPO_MAP omit part 3 constraints (0002:190) and the block layout table (0002:304-305). Fix comment-only with RR-1 fix pass; :116-119 names the commit.
- EA-3 Low crates/sim-wasm/src/lib.rs:2 (+ report :185-186, archive :634-637): OQ-0002-7 by-day decider (0002:1082-1084) dropped (omission in EMPTY-SLOTS; lib.rs ties it to the spike); register has no row (C17). Fix: wording in both EMPTY-SLOTS and the doc line.
- EA-4 Low report :185: OQ-0002-8 (a) (0002:1100-1102; register :74) "the first foundation milestone that builds them is their test": 2.2 builds room and client-endpoint empty; report silent; my reading. Fix: one sentence: their test is 2.5 under G5.
- EA-5 Low docs/adr/0002:207-209 (proposed, for 2.3): no static rule that client never imports net or the Wasm glue; page-side bans runtime only (spike item 1, PM-0002-22). Fix: 2.3 plan weighs a no-restricted-imports entry + PM-0002-42 mutation; ADR owner words part 3 when the Advisor agrees.

- C19 git status --short in the worktree after the gate run: only " M .checkpoints/foundation-toolchain/engine-architect-review-r1.md" and the test-engineer notes; ignored: node_modules/ and tools/node_modules/ (pre-existing); no .gate/ folder; HEAD 26f4867 (saver), 0 files outside .checkpoints differ from 4d78ad6.
- C20 advisor consultation 2 (before done): EA-1 (b) "if 2.4 places it at the root", "per plan Part 6" (check.mjs not read), grep scope named; EA-2/EA-3 fixes ride any fix-pass push touching crates/, else a STATUS follow-up; RR-1 note: callers per part 3 once code arrives, fix behaviour per rust-reviewer C6/C7; verdict from my angle, concurring RR-1 stands.

VERDICT (my angle): APPROVE with five Lows; I concur that rust-reviewer RR-1 (High) stands (read against part 3, not re-run by me).
