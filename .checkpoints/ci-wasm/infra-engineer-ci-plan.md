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
- S2.5 runner-images `images/ubuntu/Ubuntu2404-Readme.md` on main (raw.githubusercontent.com, read 2026-11-19 via
  WebFetch, fetch-model quotes): "Image Version: 20261111.370.1"; "GitHub CLI 2.103.3"; "Google Chrome
  157.0.8103.50"; "ChromeDriver 157.0.8103.50"; "Chromium 157.0.8103.0"; "Mozilla Firefox 160.0"; "Geckodriver
  0.37.1"; "Microsoft Edge 157.0.4341.46"; "Node.js 22.23.4"; "Rust 1.99.1", "Rustup 1.29.3"; curl
  8.5.0-2ubuntu10.16; tar 1.35+dfsg-3ubuntu0.4; env CHROMEWEBDRIVER /usr/local/share/chromedriver-linux64,
  GECKOWEBDRIVER /usr/local/share/gecko_driver; no CHROME_BIN, no Playwright listed. Announcement: "`ubuntu-latest`
  label will use Ubuntu 26.04 in November 2026" (we pin `ubuntu-24.04`: no effect). Caveat: the readme on main is
  the NEXT/CURRENT image description; the image a run gets is printed in the job log's "Runner Image" group (to be
  read on the first runner run). Sure: medium-high (official file, fetch-model transcription).
- S2.6 github.com/cli/cli/releases/tag/v2.81.0 (read 2026-11-19): "GitHub CLI 2.81.0", "01 Oct 20:22" (year not
  shown; 2025 by the release order); "This release introduces the `release verify` and `release verify-asset`
  commands for verifying GitHub Release attestations."; "a release attestation provides a signed, timestamped binding
  between a release, its git tag, and any associated assets."; usage "gh release verify-asset v1.2.3 my-asset.zip".
  So the command exists from gh 2.81.0; the image ships 2.103.3 (S2.5). Sure: high.
- S2.7 github.com/cli/cli/security/advisories/GHSA-8xvp-7hj6-mcj9 (read 2026-11-19; CVE-2026-48501), published
  2026-05-27: `gh attestation`, `gh release verify`, `gh release verify-asset` sent the Authorization header to TUF
  mirrors; affected "<= v2.92.0", patched v2.93.0. Hosts these commands contact in normal operation: TUF metadata
  from tuf-repo.github.com and tuf-repo-cdn.sigstore.dev, artifact bundles from Azure Blob Storage
  (tmaproduction.blob.core.windows.net); plus the GitHub API itself. github.com users' token went to
  tuf-repo.github.com (GitHub Pages). => On the runner: gh must be >= 2.93.0 (image: 2.103.3), and the token it holds
  should be the job's short-lived `contents: read` token only. Sure: high (primary advisory).
- S2.8 cli/cli trunk `pkg/cmd/release/verify-asset/verify_asset.go` (raw.githubusercontent.com, read 2026-11-19,
  fetch-model quotes; trunk, not necessarily = 2.103.3): stdout on success: "Calculated digest for %s: %s",
  "Resolved tag %s to %s", "Loaded attestation from GitHub API", then green "%s Verification succeeded! %s is
  present in release %s" (with the success icon). Nothing to stderr directly; errors are returned to the top level:
  "you must specify an asset filepath"; "failed to determine base repository: %w"; "no attestations found for tag %s
  (%s)" (GetByDigest failure, underlying error dropped); "error parsing attestations for tag %s: %w"; "no
  attestations found for release %s in %s/%s"; "error parsing attestations for digest %s: %w"; "attestation for %s
  does not contain subject %s" (= digest mismatch); "failed to verify attestation for tag %s: %w". Digest: sha256
  of the file (`artifact.NewDigestedArtifact(..., "sha256")`). Flow: tag -> commit SHA (`shared.FetchRefSHA`) ->
  attestations by that commit digest with PredicateType "release", Initiator "github", Limit 100 -> filter by tag ->
  the file's sha256 must be a subject -> `AttVerifier.VerifyAttestation` (signature against the trusted root). No
  tag given = latest release (`FetchLatestRelease`): so the workflow MUST pass the tag. `--format json` replaces the
  text output. Sure: medium-high (source on trunk via fetch model; exact strings to be read on the first run).
- S2.9 cli.github.com/manual/gh_help_exit-codes (read 2026-11-19): 0 success; 1 failed; 2 cancelled; 4 requires
  authentication; commands may define more. So verify-asset: 0 on "Verification succeeded!", 1 on any returned
  error (no attestation, digest mismatch, bad signature, API error), 4 when no token is found. Sure: high for the
  general table; per-case codes to be read on the first run (the planted run gives exit 1).
- S2.10 docs.github.com "use GitHub CLI in workflows" (read 2026-11-19): "GitHub CLI is preinstalled on all
  GitHub-hosted runners."; each step that calls it "must set an environment variable called `GH_TOKEN`"; examples
  use `GH_TOKEN: ${{ secrets.GITHUB_TOKEN }}`; no permission list. Sure: high.
- S2.11 cli/cli trunk `pkg/cmd/release/shared/attestation.go` (read 2026-11-19, fetch-model quotes): policy SAN must
  match `^https://dotcom\.releases\.github\.com$` (trust domain "dotcom" by default); issuer: any ("No issuer
  extension"); `NoPublicGood: true` (only GitHub's own Sigstore instance is trusted); trusted root from the
  attestation client's trust domain. Matches security-auditor's S9 (cert SAN https://dotcom.releases.github.com,
  issuer "GitHub, Inc., Fulcio Intermediate l1") and S20 (GitHub TSA). Sure: medium-high.
- S2.12 docs.github.com/en/rest/repos/attestations (read 2026-11-19): list attestations by subject digest: "when
  using a fine-grained access token the attestations:read permission is required"; caller needs read access to the
  repository; `predicate_type` accepts provenance, sbom, release or free text; page silent on unauthenticated access
  for public repos and on GITHUB_TOKEN. Sure: high on what it says; the GITHUB_TOKEN case is OPEN.
- S2.13 docs.github.com workflow syntax, `permissions` (read 2026-11-19, first 100000 chars): keys include
  "attestations: read|write|none" ("Work with artifact attestations."), "contents: read|write|none"; "If you specify
  the access for any of these permissions, all of those that are not specified are set to `none`." So today's
  `contents: read` leaves `attestations: none` on our token. Top-level `cache-mode` entry exists (read, write,
  write-only, none). Sure: high.
- S2.14 RE-READ 2026-11-19 github.com/wasm-bindgen/wasm-bindgen/releases/tag/0.2.130 (WebFetch): "released this 30
  Oct 06:16" by github-actions, commit c93b8ae848d4d9e6beaeaf9516e18a7cadd818a2, badge text "Immutable release. Only
  release title and notes can be modified.", "Assets 17". And .../releases/expanded_assets/0.2.130: entry 13
  wasm-bindgen-0.2.130-x86_64-unknown-linux-musl.tar.gz, 11.3 MB, 2026-10-30T06:11:06Z, sha256
  9687f3bb551b9f66f37ed32a40d7913a00f7962db653054e56857bc306c3227f (= his value; GitHub's server-side digest,
  fetch-model transcription); entry 17 "Release attestation (json)" -> /wasm-bindgen/wasm-bindgen/attestations/
  61631161/download, 2026-10-30T05:45:20Z. => CONFIRMED from public pages today: immutable, with a release
  attestation (security-auditor's S1/S2 of 2026-11-18 stand). Sure: high.
- Finding: gh's permissions on the job token are not documented for another public repository's attestations
  (S2.12 says attestations:read "is required" for fine-grained tokens; the job token is a GitHub App installation
  token). Open; see Block 2's write-up for the choice.
- S2.15 cli/cli trunk `pkg/cmd/release/verify/verify.go` (read 2026-11-19, fetch-model quotes): prints "Resolved tag
  %s to %s", "Loaded attestation from GitHub API", "%s Release %s verified!", then an "Assets" table (Name, Digest)
  of every subject; errors include "no attestations for tag %s (%s)", "no attestations found for release %s in %s",
  "duplicate attestations found for release %s in %s", "failed to verify attestations for tag %s: %w". It does NOT
  check immutability itself (no such field read); it relies on the release attestation existing.

## Block 2. GitHub's own check of the tool's file (DONE)
**The command** (for the downloaded archive `$dl/wasm-bindgen-0.2.130-x86_64-unknown-linux-musl.tar.gz`):
    GH_TOKEN: ${{ github.token }}   (in that step's env only)
    gh release verify-asset 0.2.130 "$dl/wasm-bindgen-0.2.130-x86_64-unknown-linux-musl.tar.gz" --repo wasm-bindgen/wasm-bindgen
The tag must be given: without it gh checks against the LATEST release (S2.1, S2.8). `--repo` is required on the
runner (the checkout is our repository).

**What it proves** (S2.1, S2.3, S2.4, S2.8, S2.11; security-auditor S8-S10, S20): the file's sha256 is a subject of
the release attestation of tag 0.2.130 in wasm-bindgen/wasm-bindgen, whose tag resolves to commit c93b8ae...; the
attestation's signature verifies against GitHub's own Sigstore trusted root (public-good refused, `NoPublicGood:
true`), signer certificate SAN `https://dotcom.releases.github.com`. In plain words: the file is byte for byte an
asset GitHub recorded and signed when that immutable release was published. This is the step that this container
could not do (security-auditor: the chain to GitHub's root and the timestamp were "not proved here").
**What it does not prove:** how the program was built (no build provenance is published: security-auditor S11, S18;
the release jobs build without `--locked`, with actions pinned by tag); that the source at c93b8ae is benign; that
the file equals HIS values (our own size + sha256 lines do that; they are the binding check, gh is the origin
anchor); anything about the program inside the archive beyond the archive's bytes (the member list and the
program's own size + sha256 do that). It trusts GitHub (its API, its signing service, its TUF root).

**What it needs on a runner:**
- gh >= 2.81.0 for the command (S2.6); >= 2.93.0 so the token is not sent to TUF mirrors (S2.7, CVE-2026-48501).
  The ubuntu-24.04 image lists gh 2.103.3 (S2.5, image 20261111.370.1). Preinstalled on every GitHub-hosted runner
  (S2.10). The text prints `gh --version` and refuses a gh older than 2.93.0 (one line; cheap belt).
- `GH_TOKEN` set to the job's token on that one step (S2.10); with no token gh exits 4 (S2.9). Not passed into the
  cut (`env -i` drops it) and not set on any other step.
- `permissions:` today's `contents: read` (workflow level). The attestations REST page says fine-grained tokens need
  `attestations:read` (S2.12), and today's block sets every unnamed permission to none (S2.13). For ANOTHER public
  repository the job token's need is undocumented; the release's attestation file itself is public (WebFetch read it
  unauthenticated, security-auditor S7). Recommendation: keep `contents: read` only (least privilege) and read the
  answer on the first runner run; if it fails for lack of permission, the stop rule fires and the one-line fix
  (`attestations: read` on the job, harmless: our repository has no attestations) comes to him as exact text. This
  is question PERM-ATT below, so he can choose to include it up front and save a round.
- Hosts (S2.7): the GitHub API (api.github.com: the tag ref, the attestations by digest), TUF metadata from
  tuf-repo.github.com (GitHub's trusted root; and tuf-repo-cdn.sigstore.dev per the advisory), attestation bundles
  possibly from tmaproduction.blob.core.windows.net. All before the cut; GitHub-hosted runners have open egress.

**Output and exit codes** (S2.8, S2.9; trunk source, exact text to be read on the first run):
- success, exit 0: "Calculated digest for <file>: sha256:9687f3bb...227f", "Resolved tag 0.2.130 to
  sha1:c93b8ae8...", "Loaded attestation from GitHub API", "✓ Verification succeeded! <file> is present in release
  0.2.130".
- no attestation / no release attestation for the tag: "no attestations found for tag 0.2.130 (<digest>)" or "no
  attestations found for release 0.2.130 in wasm-bindgen/wasm-bindgen"; exit 1. (The first form drops the API's own
  error, so an authorization refusal may look the same: PERM-ATT.)
- digest mismatch (a file that is not an asset): "attestation for <file> does not contain subject sha256:<hex>"
  (wording per source: "attestation for %s does not contain subject %s"); exit 1.
- bad signature / wrong signer: "failed to verify attestation for tag 0.2.130: <reason>"; exit 1.
- no token: gh's "gh auth login ... populate the GH_TOKEN environment variable" message; exit 4 (S2.9).
- The step runs under `bash -e` (defaults.run.shell: bash = `bash --noprofile --norc -eo pipefail {0}`), so any
  non-zero exit stops the job there, before anything is unpacked.

**`gh release verify 0.2.130 -R wasm-bindgen/wasm-bindgen`:** not a separate proof. It checks the same attestation
(signature + tag) and prints every subject's digest (S2.15); it does not check immutability itself. verify-asset
already verifies the attestation AND that our file is a subject. Not proposed (one more API round, 15 lines of
output, nothing new proved). Could be added for the report's quote if he wants the attested digest table.

**Immutable / attested:** YES, re-read today from public pages (S2.14): badge "Immutable release. Only release title
and notes can be modified."; asset "Release attestation (json)" (id 61631161); the musl archive's GitHub digest =
his sha256. security-auditor's reading stands.

**The lines the report quotes** (his "its report quotes that check's output"): from the job log of the first clean
run, the step's output as printed: `gh --version`'s first line, the four lines above (digest, tag, attestation
loaded, "Verification succeeded!"), with the run id, job id and the log timestamps; from the planted run, gh's
failure line and "Process completed with exit code 1."

**To be read on the first runner run (Block 2's part):** the image version and gh version actually used; whether
`contents: read` suffices (PERM-ATT); the exact printed lines and the file-name form; exit code on success; time
taken; that no other step sees GH_TOKEN (the log shows env per step: GH_TOKEN appears masked only on that step).
- Block 2 done; next: Block 3 (browser on the runner; FIREFOX-TESTS).
## Block 3 sources (saved as read; Block 3 NOT yet done)
- S3.1 local `node_modules/playwright-core/browsers.json` (1.65.0, read 2026-11-19): chromium revision 1254 =
  "Chrome for Testing" 158.0.8127.18; chromium-headless-shell 1254 = 158.0.8127.18; firefox 1572 = Firefox 159.0;
  webkit 2388 = 27.2; ffmpeg 1013 (all `installByDefault: true`). The container's /opt/pw-browsers holds
  chromium-1243 (Chromium 153 per cs2 report), chromium_headless_shell-1243, ffmpeg-1011: an older Playwright's set,
  not 1.65.0's.
- S3.2 local `node_modules/playwright-core/lib/coreBundle.js` (1.65.0): DOWNLOAD_PATHS for ubuntu24.04-x64:
  chromium = `https://cdn.playwright.dev/builds/cft/158.0.8127.18/linux64/chrome-linux64.zip` (cftUrl: only mirror
  cdn.playwright.dev); chromium-headless-shell = `https://cdn.playwright.dev/builds/cft/158.0.8127.18/linux64/
  chrome-headless-shell-linux64.zip`; ffmpeg = `builds/ffmpeg/1013/ffmpeg-linux.zip` and firefox =
  `builds/firefox/1572/firefox-ubuntu-24.04.zip`, each tried on PLAYWRIGHT_CDN_MIRRORS in turn:
  https://cdn.playwright.dev/dbazure/download/playwright, https://playwright.download.prss.microsoft.com/dbazure/
  download/playwright, https://cdn.playwright.dev; up to 5 attempts. Cache dir default `~/.cache/ms-playwright`.
- S3.3 same bundle, browserFetcher.ts + oopDownloadBrowserMain.ts: NO hash check of a browser download (no sha/
  integrity/checksum/createHash in either); it checks HTTP 200 and bytes = Content-Length (when not chunked), then
  unzips and chmods the executable. => Option (a)'s only integrity is TLS to the CDN (as `smoke` today).
- S3.4 same bundle, chromium `_innerDefaultArgs`: headless => plain `--headless` (+ --hide-scrollbars, --mute-audio,
  --blink-settings...); `--no-sandbox` unless `chromiumSandbox: true` (default false); chromiumSwitches include
  `--enable-unsafe-swiftshader` (WebGL on SwiftShader without a GPU), `--disable-dev-shm-usage`,
  `--disable-background-networking`, `--disable-component-update`, features off: OptimizationHints,
  NetworkTimeServiceQuerying, AimEnabled, PreconnectToSearch. With `executablePath` the given binary runs with
  these args; `--headless` on a full chrome binary (Chrome >= 132) is the NEW headless mode, on
  chrome-headless-shell it is the old headless implementation. The container's runs (SHAMBLE_CHROMIUM=
  /opt/pw-browsers/chromium -> chromium-1243/chrome-linux64/chrome) were the FULL binary in new headless.
- S3.5 playwright.dev/docs/browsers (read 2026-11-19): "Playwright ships a regular Chromium build for headed
  operations and a separate chromium headless shell for headless mode."; `--only-shell` avoids the full Chromium for
  headless-only CI; "You can opt into the new headless mode by using `'chromium'` channel."; Playwright "can operate
  against the branded Google Chrome and Microsoft Edge browsers available on the machine"; "the current Playwright
  version will support Stable and Beta channels of these browsers."; new headless "can behave differently ... expect
  different behavior in some cases"; "By default, Playwright downloads browsers from Microsoft's CDN."; Linux cache
  `~/.cache/ms-playwright`; `PLAYWRIGHT_BROWSERS_PATH` (0 = hermetic in node_modules/playwright-core/.local-browsers);
  `install-deps` installs system dependencies (tries to become root); Firefox: "Playwright's Firefox version matches
  the recent Firefox Stable build." and "Playwright doesn't work with the branded version of Firefox since it relies
  on patches."; sizes "a few hundred megabytes" (example 281M Chromium, 187M Firefox, 180M WebKit, one machine).
- S3.6 playwright.dev/docs/ci (read 2026-11-19): "Caching browser binaries is not recommended" (restore time about
  equal to download time); "We recommend setting workers to "1" in CI environments" (example `workers:
  process.env.CI ? 1 : undefined`); GitHub Actions example `npx playwright install --with-deps`; a job timeout, if
  set, "comfortably above `globalTimeout`, so Playwright stops first"; browsers launch headless by default.
- S3.7 local bundle: Playwright's `chrome` channel on Linux = `/opt/google/chrome/chrome` (the binary, not the
  `/usr/bin/google-chrome` wrapper script).
- S3.8 runner-images `images/ubuntu/scripts/build/install-google-chrome.sh` on main (read 2026-11-19, fetch-model
  quotes): Chrome from `https://dl.google.com/linux/direct/google-chrome-stable_current_amd64.deb` by
  `download_with_retry`, `apt-get install "$chrome_deb_path" -f` (a local file: no apt signature check by the
  script; integrity = HTTPS to dl.google.com at image build time); `CHROME_BIN=/usr/bin/google-chrome` set in
  /etc/environment; the Google apt source and the daily cron are removed (no update at run time); ChromeDriver from
  the Chrome for Testing JSON into /usr/local/share/chromedriver-linux64; Chromium = a SNAPSHOT build from
  chromium-browser-snapshots into /usr/local/share/chromium (/usr/bin/chromium): a developer build, not a release.
  The deb installs into /opt/google/chrome/ (the package's own layout). Sure: medium-high.
- S3.9 docs.github.com ".../runners/github-hosted-runners" (read 2026-11-19): public repositories: "Linux | 4 | 16 GB
  | 14 GB | x64 | ubuntu-latest, ubuntu-24.04, ..."; private: 2 CPU, 8 GB; "Use of the standard GitHub-hosted
  runners is free and unlimited on public repositories."; "The Linux and macOS virtual machines both run using
  passwordless `sudo`." This container: 4 CPUs, 17 GB (close to the public runner). Cost per run: 0 (public repo).

## Block 3. Which browser runs the browser tests on the runner (DONE)
**What the specs need from the browser** (Block 1): WebGL 2 without a GPU (SwiftShader; Playwright passes
`--enable-unsafe-swiftshader`, S3.4); a module Worker and Wasm in it; CDP `HeapProfiler` (allocation.spec.ts: Chromium
only); `page.clock` (clock.spec.ts); `page.route` on the Worker's script (skeleton.spec.ts); a file:// page
(file-url.spec.ts); COOP/COEP isolation from Vite's servers on 127.0.0.1 (isolation.spec.ts); real timers and enough
speed for the speed-bound assertions (allocation: > 20 frames and > 200 blocks in 8 s, ends before block 420;
TE-NT6). No external address: everything on 127.0.0.1, so the browser can run inside the network cut.

**(a) Playwright 1.65.0's own Chromium** (S3.1-S3.3, S3.5, S3.6):
- `playwright install chromium` at 1.65.0 fetches three archives: Chrome for Testing 158.0.8127.18 (revision 1254)
  `https://cdn.playwright.dev/builds/cft/158.0.8127.18/linux64/chrome-linux64.zip`; Chrome Headless Shell
  158.0.8127.18 `.../linux64/chrome-headless-shell-linux64.zip` (same host); ffmpeg revision 1013
  `builds/ffmpeg/1013/ffmpeg-linux.zip` from the first answering of cdn.playwright.dev/dbazure/download/playwright,
  playwright.download.prss.microsoft.com/dbazure/download/playwright, cdn.playwright.dev. `--only-shell` drops the
  full Chromium (headless shell + ffmpeg only). Sizes: not read (no download here; the docs' example: 281 MB
  installed for Chromium); to be read on a run if chosen.
- No hash check of any archive (S3.3): HTTP 200 + Content-Length only; the version is pinned by the lockfile
  (1.65.0 -> revision 1254), the bytes are not.
- It needs Playwright's installer (npm code) to run WITH the network before the cut: the first dependency code that
  would run with the network in the `toolchain` job (today only `npm ci --ignore-scripts` and `cargo fetch` run
  there, neither runs package code). Against the job's design.
- `install-deps` = apt with sudo (S3.5); likely unnecessary on the image (Chrome's libraries are installed with the
  Chrome deb) but unproven.
- Cache: `toolchain` has `cache-mode: none` on purpose (no channel between runs); Playwright itself says caching
  browsers is not recommended (S3.6). So a fresh download every run.
- Without `SHAMBLE_CHROMIUM`, headless runs use the headless SHELL (old headless): a different binary and mode from
  the container's runs (full chrome, new headless, S3.4). To match, `SHAMBLE_CHROMIUM` would point at the downloaded
  `chrome-linux64/chrome` (the full archive is then needed, not `--only-shell`).

**(b) The image's Google Chrome through `SHAMBLE_CHROMIUM=/opt/google/chrome/chrome`** (S2.5, S3.4, S3.7, S3.8):
- No download in the job; no package code with the network; nothing new from the internet beyond the runner image
  we already run (GitHub builds it; Chrome comes from Google's stable deb on dl.google.com at image-build time;
  update cron and apt source removed, so it does not change during a run).
- Version: Google Chrome 157.0.8103.50 in image 20261111.370.1; it MOVES with the image (weekly images). Playwright
  1.65.0 "will support Stable and Beta channels" of branded Chrome at its release (S3.5); 157 is one behind its
  bundled 158. A later image with a much newer Chrome could break Playwright 1.65.0: a visible failure, fixed by a
  Playwright bump (a dependency change, with its security answer). The job prints the version every run.
- Headless mode: Playwright passes `--headless` to the given binary = Chrome's NEW headless mode, the real browser
  (S3.4, S3.5); the same mode as every container run of change-set 2 (full CfT binary through SHAMBLE_CHROMIUM).
- CDP profiler: available (Playwright drives Chrome over CDP through `--remote-debugging-pipe`). WebGL 2 fallback:
  SwiftShader through `--enable-unsafe-swiftshader`, as here. WebGPU: unknown on the runner (here "No available
  adapters"); either backend passes skeleton's first test (WebGPU's counts are null, not compared).
- It is branded Chrome: the browser his words name first ("Chrome, the Chromium-based browsers and Firefox").
- Sandbox: Playwright passes `--no-sandbox` (S3.4), so no user namespace or setuid helper is needed under
  `setpriv --no-new-privs` inside `unshare --net`.

**(c) Other options considered:**
- (c1) Playwright's CfT archive fetched by curl with a recorded size + sha256 (like the tool): pinned bytes, no
  package code with the network; but the values are unknown until a run prints them, then his yes on the values,
  then a second CI edit; ~170 MB download every run with no cache. Better for reproducibility, more steps.
- (c2) Playwright's Docker image pinned by digest: ~2 GB, Docker inside the cut (docker group dropped): no.
- (c3) The image's Chromium snapshot (/usr/bin/chromium): a developer snapshot, not a release: no.

**Recommendation: (b), the image's Google Chrome**, with the job printing `/opt/google/chrome/chrome --version` before
the cut. Exact list for his yes: "nothing is downloaded for the browser tests; they run in the Google Chrome that
GitHub's ubuntu-24.04 image already holds at /opt/google/chrome/chrome (157.0.8103.50 in image 20261111.370.1;
the version follows the image and is printed in every run)". If he wants pinned bytes instead: (c1), in two steps.

**FIREFOX-TESTS** (his words of 13:36:37: "The game has to work in Chrome, the Chromium-based browsers and Firefox"):
- What it takes: Playwright's own patched Firefox (S3.5: branded Firefox does not work, so the image's Firefox 160 is
  no use): Firefox 159.0, revision 1572, `builds/firefox/1572/firefox-ubuntu-24.04.zip` from the three Microsoft
  mirrors above (S3.2), no hash check (S3.3), installed by Playwright's installer with the network (or by curl with
  recorded values, as c1); about 187 MB installed (docs example); its system libraries through `install-deps
  firefox` (apt with sudo) unless the image already has them (likely, since branded Firefox is installed; unproven).
- A `firefox` project in `packages/e2e/playwright.config.ts` (a source edit, the test side, not CI text), ignoring
  allocation.spec.ts (CDP is Chromium only). Specs that can run there: skeleton 5, clock 1, file-url 1 (records
  either outcome), isolation 4 (its own comment: "Firefox and WebKit run in CI later") = 11 of 12. Unknowns only a
  run answers: WebGL 2 in headless Firefox without a GPU; `page.route` on a module Worker's script in Firefox;
  render counts equal to the committed ones; speed.
- Added time: about 1 minute of tests with one worker, plus the download (seconds) and maybe install-deps (30-60 s).
- Cannot be dry-run here: no Firefox in this container, none may be fetched.
- Options: (i) now, in this pull request (one more download for his yes, a config edit, a second unknown on the first
  runner run; a Firefox failure would hold CI-WASM, which must merge before change-set 3's build starts); (ii) with
  change-set 3 (mixes a CI and test-config change into a feature change-set); (iii) later: its own small pull request
  right after this one merges, built beside change-set 3's plan (one new unknown per first run).
- Recommendation: (iii), right after this one. His Brave/Firefox checks stay the gate for Firefox until then.
- Block 3 done; next: Block 4 (the shape).
- Finding: `.gitignore` ignores `.gate/`, `dist/`, `test-results/`, `packages/sim-worker/pkg/`, not `dist-*/` (SA-DIST).
  A CI build into `packages/client/dist-*` would show as untracked under `packages/` and make build.mjs's
  `commitName()` (git status over crates, packages, ...) print "+changes" on the second build. CI builds into
  `.gate/dist-production` and `.gate/dist-development` instead (ignored, outside that pathspec).

## Block 4. The shape (DONE; the timeout value is revisited after Block 6's timings)
**Recommendation: everything in the one `toolchain` job; the downloads and GitHub's check before the cut, every new
line that runs package or repository code inside the existing cut, in this order:** build (production, then
development), boundary tests, type checks, browser tests. No second job.

Why one job and not two:
- **What must stay inside the cut:** the build that makes what players get (cargo build scripts, Vite and its plugins
  are dependency code), and with it everything else that runs package code: vitest, tsc, Playwright and the browser.
  Today's rule of this job is exactly that (only `rustup`, `cargo fetch`, `npm ci --ignore-scripts` run with the
  network; none runs package code). The browser tests need nothing outside 127.0.0.1, so they fit inside it.
- **A second job would need the build again** (a second download, a second GitHub check, a second Rust compile) or an
  artifact hand-off (actions/upload-artifact + download-artifact: two more `uses:` steps, a store that keeps the
  build for days, and a second cut to keep Playwright off the network). Both cost more and add surface. The browser
  job could not start before the build anyway (`needs:`), so no time is saved.
- **The cost of one job:** a red `toolchain` check no longer says by its name which part failed; the log does (each
  part starts with an `echo "== ..."` line), and the cut is one step. Accepted for a small pull request; the gate
  v2's workflow (step 2.3) can split it.

**Chromium inside `unshare --net` + `setpriv --no-new-privs`, loopback only** (S3.4): Playwright starts Chromium with
`--no-sandbox` (no user namespace, no setuid helper needed: no_new_privs would block the setuid helper anyway),
`--disable-dev-shm-usage` (no /dev/shm size issue), `--headless` (no display). Its profile goes to a temporary folder
under /tmp (TMPDIR is not passed by `env -i`). The page is served by `serve.mjs` on 127.0.0.1:4173 (the config's
`webServer`), the isolation spec's Vite servers bind 127.0.0.1 (vite.config.mjs `server.host`/`preview.host`), and
loopback is up inside the cut (toolchain.yml:70). The skeleton spec asserts that no request leaves 127.0.0.1, so the
cut changes nothing for a passing page. D-Bus and XDG runtime folders are absent: Chromium logs warnings and goes on
(to be read on the first runner run). Sure: medium-high; this is the main new unknown, and it cannot be tried here
(no `unshare` in this container); Block 6 runs the tests under `env -i` with the cut's variable list instead, which
covers the environment half.

**Where the tool's download and GitHub's check sit:** in new steps between "Fetch every dependency" and the cut, so
the order is: fetch (cargo, npm: unchanged) -> the tool's archive (download, size, sha256) -> GitHub's own check
(`gh release verify-asset`, the only step with the token) -> the member list, the one program taken out and installed
root-owned, its size and sha256 -> the cut. The tool's first run is build.mjs's `--version` inside the cut, after
GitHub's check (his CI-WASM (a)). Three steps rather than one, so the token sits on the one step that needs it and the
log shows GitHub's check as a step of its own (easy to quote).

**Root-owned tool** (new, closes security-auditor's Low "a process left behind by a build script could swap the tool
between the check and the run"): the program is copied with `sudo -n install -D -o root -g root -m 0555` to
`/opt/wasm-bindgen-0.2.130/wasm-bindgen` (folder root-owned 0755, under root-owned /opt). Inside the cut the runner's
uid has no groups and no_new_privs, so no process there can replace, rename or chmod the file or its folder. build.mjs
still checks it before each run. One line; the job already uses `sudo -n` for the cut.

**Least privilege:** `permissions: contents: read` unchanged (PERM-ATT is the only open point); `GH_TOKEN` on one step
only; `persist-credentials: false` unchanged; no new `uses:` step (no cache action, no artifact actions), so CI-CUT
part 2 gets no worse; `cache-mode: none` unchanged (no planted file crosses runs); actions stay pinned by full SHA
(none added). The values of the tool are literals in the workflow (his values, changeable only by a CI edit, which
needs his yes) and are compared line for line with `wasm-bindgen-tool.mjs` before the download (Block 5).
Not run before the cut: any repository or package code (the values are read as text, never by importing the module).

**Runtime** (estimate, refined in Block 6): new work = download + checks (about 5-10 s), GitHub's check (about 3-10 s:
API + TUF), a cold release build of sim-wasm for wasm32 (measured in Block 6), wasm-bindgen twice (about 1 s each),
two Vite builds (about 5-15 s each), boundary tests (about 2-5 s), five or six tsc runs (about 10-20 s together), the
browser tests with one worker (49.4 s here for 12 tests; slower on a busy runner). About +3 to +4 minutes per run.
Cost per run: 0 (standard runner, public repository: S3.9). The job's timeout (15 min today) is revisited with the
measured numbers; the browser tests get a global timeout below it so Playwright stops first (S3.6).

**Provider down / rollback:** if GitHub's release download, its API or its TUF host is down, the job fails at that
step, visibly, before anything runs; a re-run later fixes it (the download and the check live on the same GitHub that
runs CI). Google is not contacted at run time (Chrome is in the image). Rollback = revert the pull request: nothing
outside the repository is created (no cache, no artifact, no secret, no setting).
- Block 4 done; next: Block 5 (the exact text: proposed files + diff + table).
- Type-check lines tried 2026-11-19 (TypeScript 6.0.3, warm, this container; `common` = toolchain.yml:93's `strict`
  minus ` --lib es2023`): Worker shipped `--lib es2023,webworker,esnext.disposable $worker pkg/sim_wasm.d.ts`: exit 0
  (1.1 s); control without esnext.disposable: exit 2, "sim_wasm.d.ts(9,13): error TS2550: Property 'dispose' does
  not exist on type 'SymbolConstructor'"; Worker shipped + 6 tests `--skipLibCheck`: exit 0 (1.3 s); page shipped
  (13 files) `--lib es2023,dom,dom.iterable,esnext.disposable --types vite/client`: exit 0 (7.4 s); page shipped + 4
  tests `--skipLibCheck`: exit 0 (2.0 s).
- E2E-TYPES tried: the 8 e2e .ts files with `--lib es2023,dom,dom.iterable --skipLibCheck`: exit 2, 11 errors, all
  TS2591 "Cannot find name 'process'" / "'node:fs'|'node:path'|'node:url'. Do you need to install type definitions
  for node?" (playwright.config.ts:9,10,15; file-url.spec.ts:4,5; isolation.spec.ts:4,5,18,19; report.ts:44;
  skeleton.spec.ts:2). `node_modules/@types/` has chai, deep-eql, estree, stats.js, three, webxr: no node. So NO
  flag makes it pass: it needs Node's type declarations (`@types/node`, a new dependency: the hold, security-auditor,
  his yes) or a hand-written declaration file in packages/e2e (test side). Not in this CI text; the line to add later:
  `npx --no -- tsc --ignoreConfig $common --lib es2023,dom,dom.iterable --types node --skipLibCheck $e2e`.

## Block 5. The exact text (DONE; re-done after Block 6 only if a run shows a wrong line)
- Proposed file (whole): `.gate/ci-wasm/proposed/.github/workflows/toolchain.yml` (sha256 9a58a3ac...fc4 at
  writing). gate.yml: unchanged (not copied).
- Diff: `.checkpoints/ci-wasm/ci-edit.diff` (`diff -u`, labels a/.github/workflows/toolchain.yml and b/..., 125
  lines, sha256 d4defe3d...9cf at writing); `git apply --check` passes on today's file (sha256 8b581adf...016).
  Size: 87 added, 1 removed (`git diff --no-index --numstat`).
- Checks on the proposed file: PyYAML parses it; each step's `run:` and the cut's inner single-quoted script pass
  `bash -n`; no apostrophe inside the inner script; the unchanged steps compare equal to today's (checkout,
  setup-node, rustup, pin check, fetch); three steps NEW; the cut step CHANGED (one env line + 25 lines at the end).
- Order inside the job: checkout, setup-node, rustup, pin check, [NEW] the archive, [NEW] GitHub's check, [NEW] the
  one program, fetch ("the last step with the network": still true, so its name is kept), the cut (+ the build,
  boundary, type checks, browser tests at its end).
- Build flavours: both. `--production` first: the page players get, with the build's own checks for it (no dev
  marker, CSP, 25 MiB); then the development page, which the browser tests need (`__shambleReport`, `serve.mjs`) and
  whose build is the control of the marker list. The second build's cargo step is a no-op; wasm-bindgen runs again
  (checked again). Out dirs `.gate/dist-production`, `.gate/dist-development` (ignored by git; outside build.mjs's
  `git status` pathspec, so no "+changes" commit name).
- Drift between the workflow and wasm-bindgen-tool.mjs: the values are written once in the workflow (his values; a
  change is a CI edit, which needs his yes) and the module must contain each value's exact code line (`grep -qxF`),
  checked before the download; build.mjs checks the program against the module before every run. Why not read the
  values from the module with node: importing it would run repository code WITH the network, before the cut, for the
  first time in this job; and the values could then change with no CI edit. A plain `grep -F` of the values would not
  do either: the module's comment quotes his words with the same numbers, so it would pass even if a code value
  changed. The line match fails closed on a reformatting (visible, fixed by a CI edit).

**Table: each new line or group -> why -> tried here** (R-numbers are Block 6's runs)
| # | New line(s) | Why | Tried here |
|---|---|---|---|
| 1 | header comment (4 lines) | says what the CI-WASM lines do, in the file's style | n/a (comment) |
| 2 | `timeout-minutes: 25` (was 15) + comment | the job gains about 3-4 min; Playwright's global limit (10 min) must end before the job's (playwright.dev/docs/ci) | no: runner only; timings in R6-R10 |
| 3 | job `env:` WB_* (7 values) + comment | his TOOL-SOURCE values, once, in a file only his yes changes; one place for the three steps | R1 (values vs the module) |
| 4 | the 7-line drift loop (`grep -qxF`) | the workflow and the module must name the same tool | R1 (+ negative control) |
| 5 | `mkdir` + `curl --fail -L --max-redirs 2 --proto '=https' --proto-redir '=https' --max-filesize 12000000` + final address with the query cut | the route of tool-route.sh: https only, at most 2 redirects, a size cap; the address recorded | no: downloads are refused here (the route's own run of 2026-11-18 23:09 is the record: 1 redirect to release-assets.githubusercontent.com, 200) |
| 6 | size, then `sha256sum -c` | his values, before any tar call | R2 (on the archive kept from the route) |
| 7 | `gh --version` + floor 2.93.0 + `gh release verify-asset 0.2.130 <archive> --repo wasm-bindgen/wasm-bindgen`, `GH_TOKEN` on this step only | his CI-WASM (a): GitHub's own check before the tool's first run; the floor keeps the token off TUF mirrors | the floor's `sort -V -C` only (R11); the check itself no: network + this session's credentials + this container's gh 2.89.0 is in the CVE range |
| 8 | member list: `tar -tzf` sorted vs the expected seven names (`diff`) | the route's step 3 | R3, R4 (+ negative controls) |
| 9 | `tar -tvzf` + awk: plain files and one folder, 6 fields (no link, no space), each <= 64 MiB | the route's step 3 | R3, R4 (+ negative controls) |
| 10 | only `$WB_NAME/wasm-bindgen` extracted, `--no-same-owner --no-same-permissions --strip-components=1`; the folder holds exactly it | the route's step 4 | R5 |
| 11 | `sudo -n install -D -o root -g root -m 0555` to `/opt/wasm-bindgen-0.2.130/wasm-bindgen` | closes the swap between check and run for code inside the cut (security-auditor's Low) | R5 without sudo (this container runs as root), into a folder under .gate/ci-wasm/ |
| 12 | plain file, `stat`, size, `sha256sum -c` of the installed program | his program values, on the file that will run | R5 |
| 13 | env line `SHAMBLE_WASM_BINDGEN="$WB_TOOL"` | build.mjs needs the absolute path inside `env -i` | R6 (under `env -i` with the cut's list) |
| 14 | `build.mjs --production --out-dir .gate/dist-production` then `build.mjs --out-dir .gate/dist-development` | the page players get (its checks), then the page the browser tests need | R6 (+ R7: a cold release Wasm build's time) |
| 15 | boundary tests: `vitest run --root packages/sim-worker --dir packages/sim-worker/boundary` | the 7 boundary tests on the built Wasm; the old draft's `--dir boundary` with `--root` finds no file and exits 1 | R8 (+ the old form: exit 1, "No test files found") |
| 16 | `common=...`, four file lists, empty-list loop | the strict flags without `--lib`; a message when a list is empty | R9 |
| 17 | 4 tsc lines (Worker shipped / + tests; page shipped / + tests) | the type checks of the two new packages (CI-TYPES style) | R9 |
| 18 | `/opt/google/chrome/chrome --version` | the browser's version in every log (it moves with the image) | no: no Google Chrome here (/opt/pw-browsers/chromium is used instead) |
| 19 | `SHAMBLE_DIST=... SHAMBLE_CHROMIUM=/opt/google/chrome/chrome npx --no -- playwright test -c ... --project=chromium --workers=1 --retries=0 --reporter=list --global-timeout=600000` | the 12 browser tests; one worker (Playwright's CI advice, speed-bound tests), never retried, each test named in the log, a global limit | R10 (with the container's Chromium 153, under `env -i` with the cut's list) |
- Block 5 done; next: Block 6 (dry run here: R1-R11).

## Block 6 runs (saved as made; Block 6 NOT yet done)
- Calibration (a `toolchain` job log of 2026-11-18 12:04:41 UTC left in this session's scratch folder, before #60 and
  change-set 2): image ubuntu-24.04 20261111.370.1 (= S2.5), "Cache mode: write" then; inside the cut "uid=1001(runner)
  gid=65534(nogroup) groups=65534(nogroup)" and "the network is cut ... (curl exit 7)"; rustup install 9.5 s, fetch
  2 s, the cut 12 s (test profile compiled in 3.64 s, release in 3.75 s); the whole job 26 s. Today's job (with
  change-set 2's crates) is longer; its time is to be read on the first runner run.
- R1+R2 (16:40:03 UTC; step 1's text verbatim from the proposed file, the 3 curl lines replaced by `cp` of the archive
  kept from the route's run of 2026-11-18 23:09; `bash --noprofile --norc -eo pipefail` as GitHub runs `shell: bash`;
  the WB_* values exported from the proposed job env): output "size: 11860397", "<rt>/wasm-bindgen/wasm-bindgen-
  0.2.130-x86_64-unknown-linux-musl.tar.gz: OK"; exit 0; 0.70 s. The drift loop passed silently (all 7 lines found).
- R1-neg: the module copy with `    bytes: 8170785,` (scratch root): "packages/client/scripts/wasm-bindgen-tool.mjs lacks
  this line of the workflow:     bytes: 8170784,", exit 1. R2-neg size (one byte short): "size: 11860396", "the
  archive is 11860396 bytes, not 11860397", exit 1. R2-neg sha256 (last byte flipped, same size): "...: FAILED",
  "sha256sum: WARNING: 1 computed checksum did NOT match", exit 1.
- R3-R5 (16:41:12 UTC; step 3's text verbatim, only `sudo -n ` dropped (this container runs as root), WB_TOOL moved to
  .gate/ci-wasm/infra-engineer-dry/opt/wasm-bindgen-0.2.130/wasm-bindgen, RUNNER_TEMP = R1's folder): the member diff
  silent (equal), the `tar -tvzf` listing printed (7 lines, as wasm-bindgen-cli-check.md: folder + LICENSE-APACHE
  10847, LICENSE-MIT 1057, wasm2es6js 5795136, wasm-bindgen 8170784, README.md 5549, wasm-bindgen-test-runner
  19316480; owner runner/runner, 2026-10-30 06:10), then "mode 555, owner root:root, 8170784 bytes, .../wasm-bindgen",
  "mode 755, owner root:root, 4096 bytes, .../wasm-bindgen-0.2.130", ".../wasm-bindgen: OK"; exit 0; 1.12 s.
- R3-neg (crafted archive: the 7 names + `evil.sh`): diff "> wasm-bindgen-0.2.130-x86_64-unknown-linux-musl/evil.sh",
  "the archive's member list is not the expected one", exit 1. R4-neg (crafted: the 7 names, `wasm-bindgen` a link to
  /bin/sh): the listing shows "lrw-r--r-- ... wasm-bindgen -> /bin/sh", then "a member is not a plain file or folder,
  has a name with a space, or is above 64 MiB", exit 1. Neither extracted anything (no opt folder made).
- R7 (16:42:21 UTC): a COLD release build of sim-wasm for wasm32 into a fresh target folder, under `env -i` with the
  cut's list, `--offline`: compiles wasm-bindgen-shared, proc-macro2, quote, rustversion, bumpalo, cfg-if,
  wasm-bindgen, sim, syn, client-endpoint, wasm-bindgen-macro(-support), sim-wasm; "Finished `release` profile
  [optimized] target(s) in 34.38s" (real 34.4 s, user 15.3 s). This is the new compile work a runner with no cache
  pays once per run (the cut's existing wasm32 build is the debug profile).
- R6-R10 (16:43:06 to 16:44:31 UTC, exit 0, real 1 m 24.9 s): the cut's new lines verbatim (from the proposed file:
  the `strict` line + the 21 new lines) under `env -i` with exactly the cut's variable list + SHAMBLE_WASM_BINDGEN =
  R5's root-owned copy + (dry run only) CARGO_NET_OFFLINE=true. Substitutions, all listed: `npx --no -- vitest|tsc|
  playwright` -> `node node_modules/vitest/vitest.mjs | typescript/bin/tsc | @playwright/test/cli.js` (npx is
  refused here); out dirs -> `.gate/ci-wasm/infra-engineer-out/dist-production` (parent did NOT exist: Vite made it, so
  CI's `.gate/dist-production` on a fresh checkout needs no mkdir) and `.gate/ci-wasm/dist-infra-engineer-development`;
  `/opt/google/chrome/chrome` -> `/opt/pw-browsers/chromium` (no Google Chrome here); a `date` before each echo.
  - R6 builds (7 s, warm cargo): "Finished `release` profile [optimized] target(s) in 0.09s"; pkg/sim_wasm.js 9177
    raw, pkg/sim_wasm_bg.wasm 28987 raw (11655 gzip -9, 9955 brotli 11); production page: index-*.js 822196, css 495,
    wasm 28987, worker-*.js 8226, index.html 570 (5 files; no "Copy report" marker, no serve.mjs); development page:
    index-*.js 825204, overlay 617, report 2325 + the same css/wasm/worker/html, serve.mjs copied. Vite warns "Some
    chunks are larger than 500 kB after minification" (three.js; printed, not an error). The built page's commit name
    is "b7325dfb3" with no "+changes" (building into .gate/ keeps it clean).
  - R8 boundary: "boundary/sim-wasm.test.mjs (7 tests) 18ms", "Test Files 1 passed (1)", "Tests 7 passed (7)" (about
    1 s). The old draft's form (`--root packages/sim-worker --dir boundary`, run at 16:34): "No test files found,
    exiting with code 1".
  - R9 type checks: the empty-list loop silent; the four tsc lines silent (exit 0); 9 s together.
  - R10 browser tests: "Google Chrome for Testing 153.0.8010.12" (the container's Chromium, not Chrome 157); "Running
    12 tests using 1 worker"; all 12 "✓" with times: allocation 13.8 s ("64 bytes per frame ..., 472 bytes per block
    received, 14859 bytes a second (36 frames, blocks 104 to 351, ...)"), clock 3.0 s ("no: the Worker kept ticking on
    real time ... (ticks 0, 59, 78)"), file-url 3.3 s ("the page did not run from a file; the browser said: Access to
    CSS stylesheet at 'file:///...' from origin ..."), isolation 3.8/2.9/2.1/2.9 s, skeleton default backend 8.9 s,
    WebGL 2 fallback 9.5 s, hidden tab 9.7 s, Worker 404 2.5 s, Worker throws 2.2 s; "12 passed (1.1m)" (68 s).
    Margins of the speed-bound test with one worker: 36 frames (floor 20), 247 blocks (floor 200), last block 351
    (ceiling 420).
- Block 5 RE-DONE after R6-R10 (16:47 UTC): the measured new work is about 2 to 2.5 min on a cold runner (R7 34 s
  cold build here + builds 7 s + boundary 1 s + tsc 9 s + browser 68 s + tool steps about 2 s + GitHub's check), so
  today's `timeout-minutes: 15` STAYS (the change to 25 and its comment are gone) and the browser run gets
  `--global-timeout=300000` (5 min, 4.4 x the measured 68 s), so Playwright stops first and names what hung; the
  echo line says so. Proposed file now sha256 f18aae3f...19ee; `.checkpoints/ci-wasm/ci-edit.diff` now 120 lines,
  sha256 d93d823d...23b3, `git apply --check` passes; size **85 added, 0 removed**: every line that stands today
  stays byte for byte (the cut gains one env line and 21 lines at its end). Validated again: YAML parses; every
  `run:` and the inner cut script pass `bash -n`; no apostrophe in the inner script. Against what R6-R10 ran, the
  inner lines differ only in that echo text and 600000 -> 300000. Table rows changed: row 2 (no timeout change now:
  the job limit stays 15; the browser limit is row 19's `--global-timeout=300000`), row 19 (300000).

## Block 6. Dry run here (DONE)
| Run | What (the proposed text, substitutions listed above) | Result | Time |
|---|---|---|---|
| R1 | drift loop: the 7 workflow lines found in wasm-bindgen-tool.mjs | pass; neg (bytes 8170785): "lacks this line ... bytes: 8170784," exit 1 | <0.1 s |
| R2 | archive size, then sha256 (archive kept from 2026-11-18) | "size: 11860397", ": OK"; neg short: exit 1; neg flipped byte: "FAILED", exit 1 | 0.7 s |
| R3/R4 | member list (`diff`) and `tar -tvzf` awk check | pass; neg extra `evil.sh`: exit 1; neg link: exit 1 (nothing extracted) | with R5 |
| R5 | one member out, `install -D -o root -g root -m 0555` (no sudo: root here), size + sha256 | "mode 555, owner root:root, 8170784 bytes", ": OK" | 1.1 s |
| R6 | build.mjs --production, then development, under `env -i` + the cut's list | pass; 5 and 7+serve.mjs files; Wasm 28987 B; commit name clean | 7 s warm |
| R7 | cold release Wasm build (fresh target folder, offline) | "Finished `release` ... in 34.38s" | 34.4 s |
| R8 | boundary tests (`--root ... --dir packages/sim-worker/boundary`) | 7 passed; old form: "No test files found", exit 1 | 1 s |
| R9 | 4 tsc lines + empty-list loop | all exit 0 (control without esnext.disposable: exit 2) | 9 s |
| R10 | browser tests, `--workers=1 --retries=0 --reporter=list` (container Chromium 153) | "12 passed (1.1m)" | 68 s |
| R11 | gh floor `sort -V -C` | 2.103.3 -> 0, 2.92.0 -> 1, 2.93.0 -> 0 | <0.1 s |
Not run here, for the first runner run: the download (refused here); `gh release verify-asset` (network, this
session's credentials, and this container's gh is 2.89.0, inside the CVE range); `sudo -n install` (no sudo here; the
same `install` ran as root); the cut itself (`unshare`, refused here: the browser under the network namespace and
no_new_privs is unproven); `/opt/google/chrome/chrome` (absent here: the container's Chromium 153 stood in, so Chrome
157's behaviour, WebGPU adapter and speed are unproven); `npx --no --` (refused here; `node <bin>` ran the same
programs); the 15-minute limit with the new lines (estimate only).
- Block 6 done; next: Block 7 (stop rule, planted changes).

## Block 7. The first runner run (DONE)
**Stop rule for the new lines** (in #60's form; none of them has run on a runner): on the first clean run (push and
pull request), the main session stops and tells Mustafa, with the job's log lines and before any fix, push or re-run,
if:
1. any new step or new line fails; or
2. the log lacks any of these: in the archive step "size: 11860397" and the archive's ": OK" (and a final address on
   GitHub's release-asset host); in GitHub's check a gh version of 2.93.0 or later and gh's success line naming release
   0.2.130 ("Verification succeeded! ... is present in release 0.2.130"); in the program step the seven members, "mode
   555, owner root:root, 8170784 bytes" and the program's ": OK"; inside the cut, after "the network is cut", the two
   builds' size tables, "Tests 7 passed (7)", the Chrome version line, "Running 12 tests using 1 worker" and "12
   passed"; or
3. a browser test fails once (nothing is re-run to get a green check: retries stay 0, and a red-then-green run is a
   flake to report, not a pass); or
4. the job takes more than 10 of its 15 minutes (the estimate is about 4 to 7).
What is NOT a stop, only recorded: the Wasm's size or sha256 differing from this container's 28987 bytes (expected:
RR-F5, the builder's paths inside the Wasm differ), WebGPU or WebGL 2 as the default backend, Vite's chunk-size warning.

**Planted changes** (each pushed once after the clean runs end, its runs read, then taken back in the next commit; the
take-back commit's tree must equal the clean head's tree, as #60 checked):
- **P1, GitHub's check refuses a file that is not the release's asset.** Two lines put in the GitHub-check step,
  before the real check (CI text, so its exact text goes to him with the rest):
      printf 'not an asset of the release\n' > "$RUNNER_TEMP/planted.tar.gz"
      gh release verify-asset "$WB_VERSION" "$RUNNER_TEMP/planted.tar.gz" --repo wasm-bindgen/wasm-bindgen
  Expected: that step fails with gh's line "attestation for planted.tar.gz does not contain subject sha256:..." (by the
  source; exact wording read from the log) and "Process completed with exit code 1."; no later step runs (the tool is
  never unpacked or run). Proves: the check is live and fails visibly at its own step; and since gh reaches the subject
  comparison only after fetching and verifying the release's attestation, it also shows the job token as proposed
  (`contents: read`) is enough (answers PERM-ATT). Our own size and sha256 lines cannot let such a file through first,
  which is why the plant goes into the GitHub-check step itself and not into the archive.
- **P2, a browser test fails visibly inside the cut, and nothing retries it.** One value in
  `packages/e2e/tests/skeleton/render-counts.json`: "WebGL 2 fallback" "drawCalls" 3 -> 4 (source, tier 1; no CI
  text). Expected: "the first page on the forced WebGL 2 fallback" fails with the toEqual diff (drawCalls 4 expected, 3
  received), and "the first page on the default backend" too if the runner's default is the fallback; "1 failed" or
  "2 failed"; no "retry #1" line; exit 1 at the browser tests, after the builds, boundary tests and type checks passed.
  Proves: the browser tests run on the runner inside the cut and judge the page, a wrong committed value fails the job,
  and no retry hides it. By the spec's code (skeleton.spec.ts:63) the failure is certain; to be tried first in a
  worktree of the main session's, as #60 did.
- Not proposed: a plant of the drift, size, sha256 or member checks on a runner (each shown failing here: R1-neg,
  R2-neg, R3-neg, R4-neg; on a runner they would add no new fact).
- What the plants do not prove: that every browser assertion can fail (one is planted); anything about a real GPU or
  WebGPU; that GitHub's check would catch a changed archive with the same sha256 (not possible to plant).
- Block 7 done; next: Block 8 (CI side of RR-F5, SA-DIST, E2E-TYPES, TE-NT6).
