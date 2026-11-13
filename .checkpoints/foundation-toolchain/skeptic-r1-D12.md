# Skeptic r1, D12 (= EA-1, Low): the root [workspace] captures every Cargo package below the root

Plan: (1) confirm the worktree head vs 4d78ad6 differs only in .checkpoints/ and whether origin/main already has a root
Cargo.toml (does this PR introduce the capture?); (2) re-read the root Cargo.toml at 4d78ad6; (3) in a git-archive copy
of 4d78ad6 under .gate/skeptic-D12/, reproduce (a): a lone crate in spikes/, .gate/ and .checkpoints/ fails, one with
its own [workspace] builds; read Cargo's workspace-root discovery to confirm the mechanism; (4) re-run (b): a root
.cargo/config.toml reaches a nested workspace (target-dir and [env]); read ADR 0002:345-346 and plan Part 6 (no
exclude; the netcode spike's cargoConfigFiles check, PR #40); (5) re-grep the lasting docs at 4d78ad6 for any record;
(6) test the proposed exclude option and judge the fix lines (right, complete, what they break); (7) verdict.

## Checks
1. Head and base. `git merge-base --is-ancestor 4d78ad6 HEAD`: yes; `git diff --name-only 4d78ad6 HEAD`: 23 files, all
   under .checkpoints/. `git ls-tree origin/main` (2aa5e9c): no Cargo.toml, no .cargo/ at the top level. So the root
   [workspace] is new in this PR: before it, nothing above a crate in the repository captured it.
2. Root Cargo.toml at 4d78ad6: `[workspace] members = ["crates/*"]`, resolver 3, no `exclude`, no `default-members`.
   The only manifests in the tree are the root and crates/{client-endpoint,devtools,protocol,room,sim,sim-wasm}; no
   .cargo/ folder anywhere; .gitignore ignores `.gate/` and `/target/` (not `.checkpoints/`).
3. Reproduced (a) on a git-archive copy of 4d78ad6 (.gate/skeptic-D12/head, cargo 1.99.1, offline). Baseline
   `cargo metadata --locked`: 6 members, root = the copy. Lone crates (no [workspace] table): spikes/one-crate,
   .gate/lone and .checkpoints/foundation-toolchain/scratch-crate each fail `cargo build` with exit 101: "current
   package believes it's in a workspace when it's not: ... workspace: <copy>/Cargo.toml ... add the package to the
   `workspace.exclude` array, or add an empty `[workspace]` table to the package's manifest." spikes/own-ws, the same
   crate with an empty `[workspace]`, builds (exit 0). Control: the same lone crate at .gate/skeptic-D12/lone-control,
   i.e. under today's main folder (no root Cargo.toml), builds (exit 0). Reach: with the lone crates present, the
   copy's own workspace is untouched (`cargo metadata`: 6 members; `cargo test --workspace --locked`: all ok, exit 0).
   So only commands run on such a lone crate fail; CI and the 6 members do not. Holds as stated.
4. Reach of (a), refined. In the copy, a scratch workspace (.gate/star/ws, own [workspace]) whose member depends on a
   lone crate beside it (.gate/star/ext, path dependency outside ws) builds (exit 0); the same lone crate built
   directly fails (exit 101). So (a)'s "any lone package ... will fail to build" is a little broad: it fails when cargo
   runs with it as the current package (a spike or scratch crate built in its own folder), not when it is only a path
   dependency. Today's practice in the main folder's .gate/ (275 manifests scanned, the branch worktree, target/ and
   node_modules/ left out): every agent copy is a git-archive copy that carries its own root [workspace]; the only lone
   packages are path-dependency stubs (rust-reviewer-r1-star/ext, skeptic-D11/ext/depx and depy) and my control, so
   nothing there breaks after the merge. The failure is loud and self-explaining: the error names both fixes.
5. Scope of (a): SPIKE_PLAN.md:12 at 4d78ad6 "A spike is throwaway, lives in `spikes/`"; :84-95 order: the netcode
   proof test, then the sim core spike (then the 60 Hz run), then the boundary spike, and Mustafa's words quoted there
   (:95, :133): "Steps 2.3 to 2.6 keep their order and wait for their results." So spikes in spikes/ do come before
   2.3, as the finding says. PR #40 (open, draft, head now 811a476, moved since EA read 8eb5d09) is a workspace of its
   own (report E4b on 1efd46f). Holds.
6. No record (a). `git grep -F '[workspace]' / 'own workspace'` at 4d78ad6 outside .checkpoints/: only Cargo.toml:2,
   report :105 (E4b, "the netcode spike keeps its own workspace": a check that the spike complies, not a rule for
   later packages) and r5 research :78 (Cargo's docs on virtual manifests). Broader grep (workspace.exclude, nested
   workspace, lone crate, "believes it"): nothing on point. Report "Findings for 2.3" (:160-179) and "Follow-ups for
   later steps" (:181-191), REPO_MAP:16 (the root Cargo.toml's row), CHECKPOINTS §4 and §10: none says it. Holds.
7. (b) anchors. ADR 0002 :345-346 at 4d78ad6: "ts-rs writes the protocol and message types into
   `packages/types/src/generated/` on `cargo test`, through `TS_RS_EXPORT_DIR` in `.cargo/config.toml` (option D3)":
   no folder named, as the finding says. ADR 0002's migration plan, step 2.4 "Generated types. `packages/types` with
   the three generators" (:1468-1469 region), so "for 2.4" is the right step.
8. (b) reproduced in the copy. Root .cargo/config.toml with `[build] target-dir = ".gate/root-target"` and `[env]
   TS_RS_EXPORT_DIR = { value = "packages/types/src/generated", relative = true }`:
   - spikes/ws-spike (own [workspace], no config): `cargo metadata` target = <copy>/.gate/root-target; its test sees
     TS_RS_EXPORT_DIR = "<copy>/packages/types/src/generated" at run time and at compile time.
   - spikes/ws-own-cfg (own [workspace] and its own .cargo/config.toml with target-dir, the netcode spike's shape): its
     own target-dir wins (<spike>/.gate/own-target), but the root's TS_RS_EXPORT_DIR still reaches its test.
   - Nuance: cargo finds config files from the current folder, not the manifest: from outside the copy with
     --manifest-path the root config is not read (target = <spike>/target); from the copy's root with --manifest-path
     into ws-own-cfg, the root config is read and the spike's own is not (target = <copy>/.gate/root-target).
   So "a root .cargo/config.toml reaches every nested workspace" holds whenever cargo runs inside the tree (the normal
   case), [env] keys included. Root config removed again afterwards.
9. (b)'s "per plan Part 6, it would also trip the netcode spike's cargoConfigFiles check": does NOT hold. Plan Part 6
   (plan.md:446-447) says only that `cargoConfigFiles` "reads `.cargo/config.toml` in every folder above the spike",
   and chose no root config as a precaution. PR #40's check.mjs, read as text through the contents API (not run: it
   is unreviewed code) at 811a476 (its head today), 8eb5d09 (the head EA read) and 411eeee (the plan's): cargoConfigFiles
   is only the reader (:1316); the problems come from cargoConfigProblems (811a476, byte-identical to 8eb5d09 by `cmp`:
   doc :1228-1249, function :1250-1284; 411eeee :1160), which flags in a config file
   only rustflags with a lint-lowering flag, `clippy`/`fmt` aliases, `rustc`/`rustc-workspace-wrapper`, `include` and
   `runner` (tomlValues matches those exact key names), and says "Not read: a config's [env] table" (811a476 :1246;
   411eeee :1156). A root config holding TS_RS_EXPORT_DIR (and a target-dir) has none of those keys: the deps step
   would only count "2 cargo config file(s) read" (report E4d today: "1 cargo config file read"). So that clause, and
   the same words in fix line 2, would put a wrong statement into the lasting record.
10. PR #40 today (head 811a476, git trees API, not truncated): 19 Cargo.toml files; outside its spike's crates/ there
    are three, and each carries [workspace]: .checkpoints/netcode-proof-test/u4c-build/lc8-scratch (:10),
    .checkpoints/netcode-proof-test/u4s-rust-xtarget (:18), spikes/netcode-proof (:4). So the finding's "PR #40
    complies today" holds at its newest head too, and agents do put scratch crates in .checkpoints/ in practice.
11. The exclude option, re-run in the copy: root `[workspace]` plus `exclude = [".gate", ".checkpoints"]`. .gate/lone
    and .checkpoints/.../scratch-crate now build (exit 0); spikes/one-crate still fails (exit 101), so spikes keep
    needing their own [workspace]; `cargo metadata --locked`: 6 members; `cargo test --workspace --locked`: all ok;
    Cargo.lock sha256 18ec17b6... before and after (unchanged). The text names no `spikes/`, so PR #40's
    referenceProblems (which looks for that string in tracked non-Markdown files outside spikes/, docs/,
    .checkpoints/) would not see it. Nothing in the tree outside docs reads the root Cargo.toml (`git grep` exclude /
    Cargo.toml: only unrelated hits). But plan Part 6 (:443-445) says "no `exclude`" and report :116-119 says the
    pins and crates are unchanged since 566363f, so it is a CI-touching change with a rewording, as the finding says.
    Copy restored (Cargo.toml sha256 08835e1f..., as at the head).
12. Fix line 1's wording, tested: a lone crate at crates/sim/fixture-crate (inside crates/, not crates/*) also fails
    (exit 101). So "not in crates/" is a little loose: the rule is "not directly in crates/ (members crates/*)";
    the error itself names the other way out (workspace.exclude). Minor.
13. Fix line 2's "2.4's plan places it elsewhere", tested: a config in crates/protocol/.cargo/ is NOT read by `cargo
    test --workspace` run from the root (the test sees TS_RS_EXPORT_DIR NotPresent; run from crates/protocol it sees
    it). So "elsewhere" cannot be a sub-folder for the root-run test; the working alternative is the command line:
    `--config 'env.TS_RS_EXPORT_DIR.value="packages/types/src/generated"' --config 'env.TS_RS_EXPORT_DIR.relative=true'`
    from the root reached the test (plan Part 6's own "flags and folders stay on command lines"), and touches no spike.
    Copy restored (crates identical to the head's by diff -r).
14. Fix targets. Report :181-191 is "Follow-ups for later steps" (OVERFLOW-PER-CRATE, PACKAGE-ENTRIES, EMPTY-SLOTS,
    BRIEF-STALE, stale lines); STATUS_ARCHIVE.md:551 "From foundation 2.2" exists. CHECKPOINTS §9 item 3: a ready PR
    has no .checkpoints/ files, so the plan (and its Part 6 "why") does go. KIT-FREEZE-2 (DECISIONS.md:29-37): "the
    kit freezes again until the foundation's gate step" and "The freeze stops new rules", so a CHECKPOINTS rule waits
    for 2.3, as the fix says. CHECKPOINTS §10 is "Disk hygiene (Mustafa, 2026-11-08; guidance, no rule change)"; §4 is
    where scratch builds are placed, a closer home. REPO_MAP:16 (the root Cargo.toml's row) is where sessions look
    first and is tier-0 docs.

## Verdict
Holds, Low. (a) holds as stated, reproduced: once the root [workspace] (new in this PR; main has no Cargo.toml) is in
a folder, any Cargo package below it with no [workspace] of its own fails cargo with exit 101 when run on directly
(spikes/, .gate/, .checkpoints/, and also a crate nested inside crates/); no lasting doc records it. Reach is small:
the error is loud and names both fixes, path-dependency stubs and today's whole-copy practice are unaffected, CI and
the 6 members are untouched; the 60 Hz run reuses the netcode spike's harness, so it likely adds no package. (b) holds
in its core (a root .cargo/config.toml, [env] included, reaches every nested workspace when cargo runs inside the
tree; reproduced), but its clause "per plan Part 6, it would also trip the netcode spike's cargoConfigFiles check"
does not hold: PR #40's check reads those files but flags only rustflags/aliases/rustc wrappers/include/runner and
does not read [env]. Not High or Medium: a documentation gap for standard Cargo behaviour with a self-explaining error.
Fix: line 1 right (say "not directly in crates/", or "not a member"); also one clause on REPO_MAP:16. Line 2 must drop
the cargoConfigFiles clause and "waits for the spike's removal"; say instead that a root config's [env] reaches nested
spikes (a spike using ts-rs without its own TS_RS_EXPORT_DIR would write into packages/types/src/generated), and that
a sub-folder config is not read by a root-run `cargo test --workspace`, so the alternative is the command line
(`--config env.TS_RS_EXPORT_DIR...`, tested). The exclude option works as described and breaks nothing found; it
needs a CI run and the report :116-119 rewording, so 2.3. Docs-only lines break nothing.
