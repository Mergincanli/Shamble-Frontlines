# Skeptic r1, D11 (= RR-3, Low): default-features override in edition 2024 on 1.99.1, and feature unification across members

Plan: (1) read RR-3 in full and ADR 0002 part 3 (:212-217) plus the plan's 2.3/G5 text it leans on; (2) check the edition
guide's two quoted sentences against the source (web if reachable, else whatever the toolchain ships); (3) re-run the
repro offline in my own copy (.gate/skeptic-D11/, git archive 4d78ad6): a path crate depx with default = ["std"] in
[workspace.dependencies], sim takes { workspace = true, default-features = false }, then room takes { workspace = true };
cargo tree -e features per package and together, plus build output for warnings; (4) controls: the opposite direction,
member `features` keys, edition 2021, and a workspace entry with default-features = false; (5) judge the claim, the
severity, and the fix (completeness, what it breaks, a better one).

## Checks
- C0 (read): RR-3 in review-r1-findings.json (rust-reviewer's return, findings[2]) read in full. Reviewed head 4d78ad6; the
  worktree is at 27cdda1, and `git diff --stat 4d78ad6 HEAD -- . ':!.checkpoints'` is empty, so the Rust files are the
  reviewed ones. Copy: `git archive 4d78ad6 Cargo.toml Cargo.lock rust-toolchain.toml crates` into
  .gate/skeptic-D11/ws/; the stand-in maths crate at .gate/skeptic-D11/ext/depx (outside the workspace folder, so not a
  member: default = ["std"], std = [], libm = []). At head no crate has a [dependencies] table and
  [workspace.dependencies] is empty (Cargo.toml:11-12), so nothing in this PR is affected today.
- C1 (read): ADR 0002 part 3 (docs/adr/0002-code-structure-build-toolchain.md:212-217) says sim "is built with one feature
  set on every target", the maths-relevant features of the dependencies it shares with server "are set once in
  [workspace.dependencies]", AND "the gate compares the resolved feature list of every sim dependency (cargo tree -e
  features) across every build that runs the sim (server, devtools, navbake and sim-wasm), each made by the exact command
  CI and the release use, since Cargo unifies features over everything one command builds (R5-LAYOUT-F4)". So the ADR
  already names cross-member unification and plans the comparison for it; RR-3's second half is not new to the ADR.
- C2 (read): the research already knew the 1.99 change: docs/research/r5-toolchain-quality.md:92-95 (R5-LAYOUT-F5) quotes
  the Cargo workspaces page: "`default-features` declared on a package dependency overrides the value set here in Edition
  2024 packages (requires Rust 1.99+)". R5-LAYOUT-REC4 (:1206-1213) states the "set once in [workspace.dependencies]"
  rule but draws no enforcement consequence from F5. So RR-3's new part is the consequence (no direct check of where the
  features are set), not the fact.
- C3 (run, 1.99.1): workspace entry `depx = { path = "../ext/depx" }` (defaults on), sim takes
  `{ workspace = true, default-features = false }`. `cargo tree -e features -p sim -f "{p} [{f}]"`: depx `[]` (no
  features); `cargo build -p sim`: compiles, exit 0, no warning. Holds: on 1.99.1 the member's key overrides, silently.
- C4 (run, 1.97.0 = the installed `stable`, inside 1.85-1.98): the same manifests, `cargo +stable tree -e features -p sim`:
  exit 101, "error inheriting `depx` from workspace root manifest's `workspace.dependencies.depx`" / "`default-features =
  false` cannot override workspace's `default-features`". Holds: before 1.99 the combination was a hard error.
- C5 (run, 1.99.1): room adds `depx = { workspace = true }`. `cargo tree -e features -f "{p} [{f}]"`: `-p sim` depx [];
  `-p room` depx [default,std]; `-p sim -p room` depx [default,std] under BOTH sim and room; `--workspace` the same. Holds:
  one command unifies features across the members it selects, and sim's opt-out is undone in the joint build.
- C6 (run, both toolchains; driver .gate/skeptic-D11/variant.py, one fresh copy per variant): the REVERSE lever, workspace
  `default-features = false` and sim `{ workspace = true, default-features = true }`: depx [default,std] on 1.99.1 AND on
  1.97.0. A member could always turn the defaults back on; 1.99 opened only the "off over an implicit on" direction (the
  combination RR-3 names, correctly).
- C7 (run, both toolchains): workspace `default-features = false`, sim `{ workspace = true }`, room `{ workspace = true,
  features = ["std"] }`: allowed on 1.97.0 and 1.99.1; `-p sim -p room` gives sim's depx [std]. So "set once in
  [workspace.dependencies]" was never enforced by Cargo, before 1.99 either; RR-3's "not enforced" holds, and more widely
  than its 1.99 framing suggests.
- C8 (run, 1.99.1): room's entry is a bare `depx = { workspace = true }` (no features or default-features key) plus
  `[features] default = ["native"]`, `native = ["depx/std"]`: `-p sim -p room` gives sim's depx [std]. A check on the
  dependency entry's keys misses this route.
- C9 (run, 1.99.1): room declares `depx = { path = "../../../ext/depx" }` directly (not workspace-inherited): allowed, no
  warning; `-p sim -p room` gives sim's depx [default,std]. A check scoped to "workspace-inherited dependencies" misses it
  unless it also requires every member to inherit.
- C10 (run, 1.99.1): room -> depy (stand-in third-party crate taking depx with its defaults): `-p sim -p room` gives
  sim's depx [default,std] with no member manifest naming depx's features at all. No manifest check can catch this; only
  the cross-build comparison does.
- C11 (run, 1.99.1): edition-2021 control: sim with `edition = "2021"` and the opt-out over a defaults-on entry: depx
  [default,std] on both 1.99.1 and 1.97.0 (still ignored). The override is edition-2024-only, as RR-3 says.
- C12 (run, 1.99.1, model of ADR 0002 part 3's comparison; sim-wasm and room each depend on sim, room stands in for the
  server): case A, room also takes `depx = { workspace = true }`: `cargo tree -e features -p sim-wasm --target
  wasm32-unknown-unknown` shows depx [] and `-p room` shows depx [default,std]: the planned comparison flags it, in the
  same CI run a manifest check would. Case B, only sim takes depx: [] in both builds: the comparison passes, the features
  are identical everywhere (no determinism effect), but the choice lives in sim's manifest; the comparison fails only
  later, on whoever adds the second dependent. That is the one sense in which RR-3's "only notices that after the fact"
  holds. Scope: this is a two-crate model (room stands in for server) of a check that does not exist until 2.3;
  in that model the comparison flags case A in the same run, which is all this check shows.
- C13 (source): the edition guide page's source (raw.githubusercontent.com/rust-lang/edition-guide/master/src/rust-2024/
  cargo-inherited-default-features.md, HTTP 200, 2256 bytes, saved at .gate/skeptic-D11/fetched/) has both of RR-3's
  quotes verbatim: "Starting with Rust 1.99, `default-features` on an inherited dependency in a 2024 Edition package
  overrides the value from `[workspace.dependencies]`." and "The original 2024 Edition behavior in Rust 1.85 through 1.98
  rejected this combination." The same page warns: "if you build multiple workspace members at the same time, the
  features will be unified so that if one member sets `default-features = true` ... the default-features will be
  enabled for all members using that dependency."
- C14 (run, 1.99.1): "quietly" holds for edition 2024 only: `cargo build -p sim` in the edition-2021 variant prints
  "warning: crates/sim/Cargo.toml: `default-features` is ignored for depx, ... overriding workspace `default-features` to
  false requires Rust 1.99+ and the 2024 edition"; the edition-2024 build (C3) prints nothing.
- C15 (run, 1.99.1): `cargo build -p sim -p room --config 'resolver.feature-unification="package"' -v` prints "warning:
  ignoring `resolver.feature-unification` without `-Zfeature-unification`" (`cargo -Z help` lists it as unstable), and
  the verbose log shows ONE rustc call for depx with `--cfg 'feature="std"'`, linked by both sim and room
  (libdepx-7cab05e7b721e520). On the pinned stable, one command always unifies across the members it selects; and
  per-package unification would not help anyway, since one binary's own graph is always unified.

## Verdict
- Holds: yes. Severity: Low (the same as RR-3). Discriminating checks: C3 (silent override on 1.99.1), C4 (hard error on
  1.97.0), C13 (both quotes verbatim at the source), C5 and C15 (unification across selected members, unconditional on
  the pinned stable), C14 (silent only in edition 2024).
- Novelty: the facts were already on record (R5-LAYOUT-F5 for the 1.99 override; ADR 0002 part 3 and R5-LAYOUT-F4 for
  unification). What is new is the consequence: part 3's "set once in [workspace.dependencies]" clause has no direct
  check, and 1.99 adds one more silent way to break it. Cargo never enforced it before 1.99 either (C6, C7).
- Why not higher: no crate has a dependency at head (Cargo.toml:11-12 empty), and in my two-crate model the planned
  comparison (ADR 0002 line 574, PM-0002-44) flags the divergence (C12 case A). A note for step 2.3.
- Fix: the right direction, additive, and nothing to change in this PR. Incomplete as worded: "then the feature set really
  lives only in [workspace.dependencies]" fails three ways I ran: forwarding through a member's [features] table (C8), a
  direct, non-inherited declaration (C9), and a third-party crate taking the defaults (C10). Not run: target-specific,
  dev and build tables, and `dep?/feat`; R5-LAYOUT-F4's quoted resolver exclusions say features on target-specific and
  dev dependencies count when that target or kind is built, so a manifest check must scan those tables too. The
  comparison stays the guarantee; a manifest check only adds earlier, cause-level detection (C12 case B) and a clearer
  error.
- What it would break, and the route: (1) the optional line-11 comment is an edit in this PR; a Low never triggers a push
  on its own, so it can only ride a push that happens anyway, and it should read as a rule ("set only here"), not a fact,
  since nothing enforces it before 2.3. (2) The manifest check widens G5 beyond ADR 0002 line 574 and PM-0002-44, so it
  enters through 2.3's plan and the ADR's next revision, not silently. (3) Refusing all member `features` keys on
  sim-shared crates forces any per-member feature into the workspace entry, and so into every member and the Wasm build:
  no stricter than the comparison for builds that run the sim, stricter for members never built with it.
- Better: keep the comparison as the guarantee. If a lint is added, then for every crate in sim's resolved tree refuse
  (a) member-level `default-features` and `features` keys in every dependency table, (b) non-inherited declarations of
  that crate, and (c) `name/feat` and `name?/feat` entries naming it in members' [features] tables. Write those workspace
  entries with an explicit `default-features = false` and an explicit feature list, and add a planted PM-0002-44 case for
  the 1.99 opt-out.
- Not verified: only 1.97.0 was run from the 1.85-1.98 range; the edition guide was read from master, not pinned to
  1.99.1; depx is a local path crate standing in for a registry crate; the comparison was modelled with two crates, not
  run (it does not exist until 2.3); the target-specific, dev, build and `dep?/feat` routes were not run.
