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
