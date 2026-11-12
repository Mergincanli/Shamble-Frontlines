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

## Findings
(none yet)
