# Trials on main 311c048 (3b #68 and the guard fix #69 taken in; merge 77d28ee), 2026-11-25
All in the worktree `.gate/ci3`; logs in `.gate/trial/*-b.log` (not committed). Playwright started through
`node node_modules/@playwright/test/cli.js` (his PW-RUNS (a)); CI keeps `npx --no --` like the file's other lines.
- a. `cargo test --release -p sim -p protocol -p room -p client-endpoint -p sim-wasm -p devtools --locked`: exit 0, 31 s,
  21 test binaries ok, 113 tests passed.
- c. `python3 -I -B crates/devtools/vectors/make_chacha8_vectors.py --check`: exit 0, 197 ms;
  `make_f32_vectors.py --check`: exit 0, 1,604 ms ("38 operations, 10072 cases": 3b changed the file and its generator).
  Plants (on 28254a7): a changed `pin` line and one changed byte each fail their check (exit 1).
- d. `cargo build --locked --target wasm32-unknown-unknown -p devtools --lib`: exit 0, 5 s.
- The development page built with the approved wasm-bindgen (both sha256 checked).
- Test lists with the draft config: Chromium 16 (3b added `loop.spec.ts` and `presses.spec.ts`); Firefox 15 (the same
  less `allocation.spec.ts`, the only spec with Chromium's heap profiler, lines 45-58). The `node` project: 1 passed.
- Chromium (Chrome for Testing 158.0.8127.36 here; the runner uses its image Chrome): 16 passed, 3.3 min.
- Firefox 161.0 (his TRIAL-DOWNLOAD (a)): the 2 rule tests pass (662 ms, 686 ms); clock, file-url and two isolation
  tests fail on the page's own line "This page could not start drawing: can't access property "getSupportedExtensions",
  this.gl is null"; 9 did not run before Playwright's 5-minute stop. Earlier, with his two EGL files (EGL-TRIAL (a)),
  the same; a blank page gives the same refusal (FIREFOX-GL question). His FIREFOX-GL (a): the first GitHub run is the
  drawing tests' trial; the same line there means the Firefox entry goes to `rules/` only.
- The e2e type check (workflow line 191's command, with the draft config): passes.
- The draft workflow (`draft-toolchain.diff`, sha256 6784c2d5ad3c279a6795540f5bc8ddb3006c68afa9c72ded4cd988c3b0787271;
  the file after it: sha256 35c8d11b73b041c3dd3f692b6652f78df3204f256d56adf911c430e40cd8f3d3; the file today: 56d37fc6...9811,
  unchanged on main since 28254a7): YAML parses; the inner script passes `bash -n` and holds no apostrophe (it sits in
  single quotes). Its new lines run here as written where they can: the libEGL line prints "libEGL.so.1: not on this
  runner (printed, not judged)" in this container; the plain-file check and `--version` pass on the trial Firefox's path.
- Job time estimate: main's run 2 min 7 s before 3b; 3b's Chromium run 3.4 of 5 min on CI; the Firefox run up to its
  5-minute stop; the new Rust and Python lines about 40 s: about 11 min against the job's 15-minute limit.
- The legacy gate (`node tools/gate.mjs origin/main --no-smoke`): PASS (smoke left to CI).
- Not tried here: the network cut itself (`unshare --net`), the runner's Python 3.12, the runner's Firefox 160 at
  `/usr/lib/firefox/firefox`, Firefox inside a network namespace.
