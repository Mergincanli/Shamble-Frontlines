# security-auditor: dependency check, 4b's first manifest change (DEP-HOLD, CHECKPOINTS section 3)

Worktree .gate/lane1-takeover, branch cs/4b-content, base main 2483e9a, uncommitted. Read-only; no build, no test, no
fetch. What I ran: `sha256sum` on the files, `git status --short`, `git diff -- Cargo.lock crates/devtools/Cargo.toml`,
`cargo tree --locked --offline -p devtools -e normal,build,dev`, `cargo metadata --locked --offline --no-deps`
(neither compiles anything, and `--offline` rules out any fetch), a grep of crates/devtools, and reads of the two new
files and of protocol's manifest for comparison.

- Digests (mine equal deps-4b1.sha256):
  0ea3ed50699e37398df809cd6308178f2a0b7a701d32d513e208712002d3bbb0  Cargo.lock
  1c20979e0ae907ad818068f3be19060d24d637028e6916deaa0c0f82409474f0  crates/content/Cargo.toml
  c3b458d55796508790abc9fa7bb454e866fadc99df31b09e4262ed5a01946ec6  crates/devtools/Cargo.toml
  (read, not part of the hold: 73a63f43976c10c6eb9b1b73766a479ac7370a768849a9039ae78821a0ca22ee crates/content/src/lib.rs)
- Scope: `git status` lists only Cargo.lock and crates/devtools/Cargo.toml as modified, plus crates/content/ (just
  Cargo.toml and src/lib.rs, by find) and notes. No root Cargo.toml, toolchain, .cargo, package.json or npm lockfile change.
- Q1 lockfile: one hunk. It adds `[[package]] content 0.0.0` with no `source` and no `checksum` (a path crate), and
  changes devtools' list (+content, -client-endpoint, -room). No registry, git or other source is added, no checksum
  line, and no locked version moves: the registry crates in devtools' tree are unchanged (wasm-bindgen 0.2.130,
  cfg-if 1.0.5, etc.). Answer: no.
- Q2 manifests: content has no dependencies, features, links or build script. Metadata gives one target, a lib; there
  is no build.rs on disk and no `build` key. Its lints come from the workspace; lib.rs adds forbid(unsafe_code) and a
  deny list that includes arithmetic_side_effects, and holds no code. Every devtools path is inside the repository
  (crates/content, protocol, sim, sim-wasm); wasm-bindgen comes from the workspace, unchanged. No feature, patch,
  registry setting or path outside the repository. Answer: no.
- Q3 dropping the two edges: safe. devtools never ships: players get `-p sim-wasm` only (build.mjs:149-153), and a page
  holding any other Wasm is refused (build-checks.mjs:189-197). The test module still links client-endpoint and room
  through sim-wasm (cargo tree, depth 2). The removed lines named no features, so the feature set does not change. No
  devtools source names either crate: the grep finds only `.room()` method calls on sim-wasm's LocalMatch, which need
  no direct edge. Both crates stay workspace members, built and tested by `cargo test --workspace`. Answer: yes, safe.
- Low notes, not blocking: (1) `protocol` is kept as a normal dependency for an integration test
  (tests/aim_reach.rs); `[dev-dependencies]` would be the narrower edge. (2) The comment in content/Cargo.toml names
  crates/devtools/tests/dependency_rules.rs, which does not exist at this head.
- Verdict: APPROVE. This exact content may be committed, pushed and built: Cargo.lock 0ea3ed50...d3bbb0,
  crates/content/Cargo.toml 1c20979e...9474f0, crates/devtools/Cargo.toml c3b458d5...946ec6 (full digests above). Any
  later byte change to these files, a dependency or build.rs in crates/content, or the collision library's batch needs
  its own answer.
DONE
