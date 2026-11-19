Task: CI-WASM, the facts and a draft of the exact CI text (infra-engineer, part "ci-plan"), started 2026-11-19.

## Plan (blocks, in order)
1. Today's CI and the old draft (toolchain.yml, gate.yml, #60 report, cs2-plan s18 + s7 5a, tool-route.sh,
   security-auditor-tool.md, build.mjs + build-checks.mjs + wasm-bindgen-tool.mjs, playwright config + 5 specs,
   cs2 report Evidence + Not verified). What the old draft gets wrong.
2. `gh release verify-asset` from official sources (command, proves / does not, gh version vs image, token,
   permissions, hosts, output and exit codes, `gh release verify`, immutability of 0.2.130, first-run list, lines to quote).
3. Browser on the runner: (a) Playwright 1.65.0 download, (b) preinstalled Chrome via SHAMBLE_CHROMIUM, (c) other;
   spec needs; recommendation with exact download list; FIREFOX-TESTS options.
4. The shape: inside the cut / own job / both; Chromium under unshare --net + no-new-privs; order; time; least privilege.
5. The exact text: proposed workflow files under .gate/ci-wasm/proposed/.github/workflows/, diff at
   .checkpoints/ci-wasm/ci-edit.diff, table (line -> why -> tried here).
6. Dry run here (build, boundary tests, type checks, browser tests once, member list), outputs and times.
7. First runner run: stop rule, planted changes.
8. CI side of follow-ups: RR-F5, SA-DIST, E2E-TYPES, TE-NT6 CI settings.
End: planned size, questions for Mustafa, Not verified, DONE.

## Notes (one line per finding)
- Memory: no `.claude/agent-memory-local/infra-engineer/` exists; nothing to load.
- Context read: .checkpoints/ci-wasm/brief-context.md (whole).

## Block 1. Today's CI and the old draft (DONE)
Read 2026-11-19: `.github/workflows/toolchain.yml` (101 lines), `.github/workflows/gate.yml` (93 lines),
`docs/reports/2026-11-18-kit-hold-ci.md`, `.gate/ci-wasm/hist/cs2-plan.md` s18 (:960-1004) and s7 item 5a
(:349-383), `.gate/ci-wasm/hist/tool-route.sh`, `wasm-bindgen-cli-check.md`, `security-auditor-tool.md`,
`packages/client/scripts/{build,build-checks,wasm-bindgen-tool}.mjs`, `packages/e2e/playwright.config.ts`, the five
specs + `report.ts` + `render-counts.json`, cs2 report "Evidence" (:208-281) and "Not verified" (:411-439).

**Today's CI, the facts the new text must keep:**
- toolchain.yml: one job `toolchain`, ubuntu-24.04, `timeout-minutes: 15`, `cache-mode: none` (:33; #60's run log
  printed "Cache mode: none"), `permissions: contents: read` at workflow level (:13-14), `persist-credentials: false`
  (:37), actions pinned by SHA: checkout 3d3c42e5... v7.0.1, setup-node 82076278... v7.0.0 (node 22). Steps: rustup
  install from rust-toolchain.toml; pin check; "Fetch every dependency (the last step with the network)" =
  `cargo fetch --locked` + `npm ci --ignore-scripts --no-audit --no-fund` (:57-60); the cut (:62-100): control probe
  before, `sudo -n unshare --net` + `ip link set lo up` + `setpriv --reuid=<runner uid> --regid=nogroup --clear-groups
  --no-new-privs` + `env -i` with an explicit list (HOME PATH USER CI LANG PROBE_IP CARGO_HOME RUSTUP_HOME
  RUSTUP_AUTO_INSTALL npm_config_update_notifier), probe inside must give curl exit 7 or 28; then cargo tests, write
  types again + compare, release tests, wasm32 build of 5 crates (no tool), `npm ci --offline`, `npm test
  --workspaces`, tsc of packages/types (shipped with lib check; shipped+tests with --skipLibCheck).
- gate.yml: `gate` (tools/gate.mjs --no-smoke) and `smoke` (tools/ Playwright 1.56.1 Chromium, cached by
  actions/cache caa29612... v5.1.0 under `~/.cache/ms-playwright`, key = playwright-core version; `npx --no --
  playwright install --with-deps chromium` on a miss, `install-deps chromium` on a hit: apt with sudo).
- How #60 put CI text to him: the exact text as a diff, his yes on it (with changes he names), the files written by
  copy from tested scratch copies and checked hunk for hunk; stop rule "none had run on a runner before; if the clean
  run failed on one, or the log did not show 'Cache mode: none': stop and tell him"; one planted change (a generated
  file committed with the executable bit) pushed, both runs failed at the compare step with the expected log lines,
  taken back in the next commit; the report quotes job-log lines and lists what the runs prove and do not prove.
- infra-engineer's own lines from #60 on the gate v2 list: the job that runs the cut should have no `uses:` step at all
  (CI-CUT part 2: no post step runs after dependency code with the network and token). Today toolchain.yml has two
  `uses:` steps (checkout, setup-node) before the cut; neither has a post step that matters here (checkout's post step
  removes credentials, which are not persisted). Keep in mind for the shape (Block 4).

**The build as it exists (what CI must call):**
- `build.mjs` (:1-150): needs `SHAMBLE_WASM_BINDGEN` absolute; `checkedTool()` compares size+sha256 with
  `WASM_BINDGEN_TOOL.program` before EVERY run (twice per build: `--version`, then the binding run, :55, :65, :77);
  refuses env overrides matching `CARGO_OVERRIDES` (RUSTFLAGS, CARGO_BUILD_*, CARGO_PROFILE_*, CARGO_TARGET_*,
  RUSTC*, RUSTUP_TOOLCHAIN, WASM_BINDGEN_*, RUST_LOG; :44-47, :68-69); runs `cargo build --locked --release --target
  wasm32-unknown-unknown -p sim-wasm --target-dir <root>/target`; writes `packages/sim-worker/pkg/`; Vite build into
  `packages/client/dist` or `--out-dir` (name must be `dist` or `dist-*`, `checkOutDir`); `--wasm-only`,
  `--production`; the page checks: 25 MiB per file, `cspProblems`, `bundleProblems(production)` (dev markers absent
  in production, present in development = the control); `commitName()` runs `git status`/`git rev-parse` (git works
  inside the cut: #60). Development build copies `serve.mjs` into the out dir (the e2e webServer needs it).
- `wasm-bindgen-tool.mjs` (:13-27): `WASM_BINDGEN_TOOL = {version "0.2.130", archive {url, bytes 11860397, sha256
  9687f3bb...227f}, program {member "wasm-bindgen-0.2.130-x86_64-unknown-linux-musl/wasm-bindgen", bytes 8170784,
  sha256 ca4df773...6931}}`, frozen; the comment already says "the CI edit that brings the tool to CI ... checks the
  archive's". Not in the file: the expected member list (six names + folder), the 12000000 download cap.
- Boundary tests: `packages/sim-worker/boundary/sim-wasm.test.mjs` (7 tests; needs `pkg/` from the build: imports
  `../pkg/sim_wasm.js` and reads `../pkg/sim_wasm_bg.wasm`). cs2 ran them as `cd packages/sim-worker && node
  ../../node_modules/vitest/vitest.mjs run --dir boundary` (plants.py:23, at 29ab052^).
- Worker's src imports `../pkg/sim_wasm.js` (worker.ts:4): its type check needs the build first. Client's
  `app/boot.ts:1` imports `@shamble/sim-worker/worker?worker`: needs `--types vite/client`.
- Type-check flags that passed in cs2 (old checkpoint at 29ab052^ :280, :301-305; report :218-220, :280, :319): the
  Worker with libs `es2023,webworker,esnext.disposable` and WITHOUT `--types vite/client` (Vite's client types need
  `SharedWorker`, absent from the webworker lib, so they fail with the lib check on); the page with
  `es2023,dom,dom.iterable,esnext.disposable` and `--types vite/client`; tests with `--skipLibCheck`. Exact lines to
  be re-derived in Block 6.

**The e2e side:**
- `playwright.config.ts`: projects `node` (tests/node, run by `npm test --workspaces` already) and `chromium`
  (tests/ minus node/**; `executablePath` = `SHAMBLE_CHROMIUM` when set, else Playwright's own browser);
  `webServer` only when `SHAMBLE_DIST` is set: `node serve.mjs` in the dist folder, url http://127.0.0.1:4173,
  `reuseExistingServer: false`; `retries: 0`; `forbidOnly` when `CI` is set; `trace: "off"`; no `workers`, no
  `timeout`, no `reporter` set (defaults: workers = half the CPU cores, test timeout 30 s, reporter "list" locally,
  "dot" on CI per Playwright's defaults; to confirm in Block 3/8).
- 12 tests: skeleton.spec.ts 5 (two backends: default and `?backend=webgl`, render counts must equal
  render-counts.json for "WebGL 2 fallback", WebGPU null; hidden tab; two broken-Worker tests through
  `page.route`), allocation.spec.ts 1 (CDP `HeapProfiler.startSampling`: CHROMIUM ONLY; speed-dependent bounds:
  frames > 20, blocks > 200, after.blocks < 420 inside an 8 s window), clock.spec.ts 1 (`page.clock`), file-url
  1 (file:// page), isolation.spec.ts 4 (Vite dev server + preview server inside the test process, 127.0.0.1, ports
  chosen by Vite: preview defaults to 4173, which serve.mjs holds, so Vite moves to the next free port).
- Waits: `running()` waits up to 60 s for 90 blocks (30 for isolation); skeleton waits 2 s; hidden-tab 4.5 s;
  allocation 8 s; clock 2 s; file-url 3 s. Default test timeout (30 s) applies to each test unless set.

**What the old draft (cs2-plan s18) gets wrong now that the build exists:**
1. `wb_sha256=<sha256>` placeholder: the values exist now, in `wasm-bindgen-tool.mjs`. Writing them a second time
   in the workflow would let the two drift; read them from the file with node (Block 5).
2. Its download lacks the route's limits: no `--proto-redir '=https'`, no `--max-redirs`, no `--max-filesize`; it
   checks sha256 but not the size first.
3. It unpacks the whole archive (`tar -xzf ... --strip-components=1`): all six files, no member list, no
   one-member extraction, no `--no-same-owner --no-same-permissions`; the program's own size and sha256 are not
   checked before the first run (build.mjs now does that itself before every run, but the workflow should stop
   before the cut with a clear line).
4. GitHub's own check (`gh release verify-asset`) is only a sentence, not text; it needs `GH_TOKEN` in that step's
   env and must sit before the tool's first run (his CI-WASM (a)).
5. The type-check lines are wrong: the Worker's line has `--types vite/client` (fails with the lib check on) and
   lacks `esnext.disposable`; the page's lacks `esnext.disposable`. No test-file check, no e2e check.
6. The boundary line `npx --no -- vitest run --root packages/sim-worker --dir boundary`: equivalent to cs2's
   working route (cwd packages/sim-worker), to be re-run in Block 6.
7. It builds only the development page; no production build (what players get) and no browser tests at all.
8. Its tool folder `$HOME/.local/wasm-bindgen-0.2.130` is writable by the same user the cut runs as; build.mjs
   re-checks before each run, but the window between check and run remains (security-auditor's Low). On the
   runner the cut drops to the runner's uid with no_new_privs, so a root-owned 0555 folder would close that window.
9. Its open points are now answered: the archive has one top folder (member list in wasm-bindgen-cli-check.md);
   `vite/client` resolves from node_modules inside the cut (cs2 ran offline-equivalent here; to confirm in Block 6).
- Block 1 done; next: Block 2 (gh release verify-asset from official sources).
## Block 2 sources (saved as read; Block 2 NOT yet done)
- S2.1 cli.github.com/manual/gh_release_verify-asset (read 2026-11-19): usage "gh release verify-asset [<tag>]
  <file-path> [flags]"; "Verify that a given asset file originated from a specific GitHub Release using
  cryptographically signed attestations."; "checks that the asset you provide matches a valid attestation for the
  specified release (or the latest release, if no tag is given)"; "validating that the asset's digest matches the
  subject in the attestation and that the attestation is associated with the release." Flags: --format json, -q/--jq,
  -t/--template; inherited -R/--repo [HOST/]OWNER/REPO. Page says nothing on exit codes, auth or printed text; no
  version number on the page. Sure: high (official manual).
- S2.2 cli.github.com/manual/gh_release_verify (read 2026-11-19): usage "gh release verify [<tag>] [flags]";
  "Verify that a GitHub Release is accompanied by a valid cryptographically signed attestation."; prints metadata
  about every asset the attestation references, including digests. Same flags. Nothing on exit codes/auth.
- S2.3 docs.github.com ".../verify-release-integrity" (read 2026-11-19): prerequisite "install the GitHub CLI"; no
  version, auth or permission stated; `gh release verify RELEASE-TAG` = "verify a release exists and is immutable";
  `gh release verify-asset RELEASE-TAG ARTIFACT-PATH` = "verify a local artifact is an exact match for a release
  asset"; "This command cannot be used to verify the source code zip file or tarball for a release"; no example
  output on the page; no date shown. Sure: high.
- S2.4 docs.github.com ".../immutable-releases" (read 2026-11-19): "Immutable releases are releases where the assets
  and associated Git tag cannot be changed after publication."; tag "locked to a specific commit"; assets
  "protected from modification or deletion"; "creating an immutable release automatically generates a release
  attestation, which is a cryptographically verifiable record of a release containing the release tag, commit SHA,
  and release assets."; title, notes, pre-release/latest status still editable; a deleted immutable release's tag
  cannot be reused. Nothing on signer, format or Sigstore instance (security-auditor's S9/S20 read it from the
  bundle: GitHub's own Sigstore instance, cert "GitHub, Inc./Attester", SAN https://dotcom.releases.github.com).
