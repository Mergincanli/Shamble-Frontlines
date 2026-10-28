# R5 helper draft: CI with Rust caching (R5-CI) and preview links per PR (R5-PREV)

Author: infra-engineer (helper to the R5 owner, test-engineer), 2026-10-28, branch cs/v2-research. Status: draft, not
verified. The owner folds both sections into `docs/research/r5-toolchain-quality.md` unchanged. Every source was read
2026-10-28 with WebFetch. Tiers: "official" means the specification, the platform's own docs, or the tool's own README
and release pages; "other" means anything else, and is marked. No spike was run, no code was written, and no CI file
was changed. Pass marks are proposed, not decided.

---

## Section 1. R5 helper: CI with Rust caching; preview links per PR (part 1: CI with Rust caching)

The brief's R5 row (docs/V2_PHASE0_BRIEF.md:125): "Repository layout (a Cargo workspace plus TypeScript with Vite);
strict TypeScript and lint rules; clippy and rustfmt; tests (cargo test, property tests, Vitest, Playwright, golden
tests against legacy); CI with Rust caching; **preview links per PR**; the gate v2 that replaces today's gate". This
section answers "CI with Rust caching", plus how CI runs the gate v2. What the gate v2 checks is the owner's part.

### Findings

**Rust build caching**

- **R5-CI-F1.** Swatinem/rust-cache caches the Cargo home and the dependencies' build output in `target`, not the
  unpacked registry sources. Latest release v2.9.3 ("20 Oct", no year on the page). Quotes: it caches "`~/.cargo`
  (installed binaries, the cargo registry, cache, and git dependencies)" and "`./target` (build artifacts of
  dependencies)"; "The `~/.cargo/registry/src` directory is not cached." · Source: https://github.com/Swatinem/rust-cache
  (README) and https://github.com/Swatinem/rust-cache/releases · official (the tool's own docs) · read 2026-10-28 ·
  confirmed in the source.
- **R5-CI-F2.** rust-cache keys each cache on the toolchain and the lockfile, so a toolchain bump or a lockfile change
  starts a new cache. Quotes: the key holds "the rustc release / host / hash (for all installed toolchains when
  available)" and "a hash of the workspace's `Cargo.lock`". The inputs include `shared-key` ("stable over multiple
  jobs"), `save-if` ("Determines whether the cache should be saved."), `cache-on-failure`, `workspaces` ("`$workspace ->
  $target`"), `cache-targets`, `cache-all-crates` and `cache-bin`. · Source: https://github.com/Swatinem/rust-cache
  (README, inputs and cache key) · official (tool docs) · read 2026-10-28 · confirmed in the source.
- **R5-CI-F3.** rust-cache caches only dependencies and turns incremental builds off. Quotes: before saving it removes
  "Incremental build artifacts."; "the workspace crates themselves are not cached since doing so is generally not
  effective."; "this action automatically sets `CARGO_INCREMENTAL=0` to disable incremental compilation." · Source:
  https://github.com/Swatinem/rust-cache (README) · official (tool docs) · read 2026-10-28 · confirmed in the source.
- **R5-CI-F4.** GitHub scopes caches by branch. A pull request can restore its base branch's and the default branch's
  caches, but a cache that a pull request writes serves only re-runs of that same pull request. Quotes: "Workflow runs
  can restore caches created in either the current branch or the default branch (usually `main`)."; "If a workflow run
  is triggered for a pull request, it can also restore caches created in the base branch,"; a PR's cache "is created
  for the merge ref (`refs/pull/.../merge`)" and "can only be restored by re-runs of the pull request." "It cannot be
  restored by the base branch or other pull requests". rust-cache says the same: "Caches from base branches are
  available to PRs, but not across unrelated branches." · Sources:
  https://docs.github.com/en/actions/reference/workflows-and-actions/dependency-caching ("Restrictions for accessing a
  cache"; no page date) and https://github.com/Swatinem/rust-cache · official · read 2026-10-28 · confirmed in the
  source.
- **R5-CI-F5.** The free cache is 10 GB per repository, and entries unused for 7 days are evicted. Going above 10 GB is
  opt-in, paid, and needs a Pro, Team or Enterprise account. Over the limit, the least recently used entries go first.
  Quotes: "By default, the limit is 10 GB per repository"; "GitHub will remove any cache entries that have not been
  accessed in over 7 days."; the default 10 GB and seven-day limits come "at no additional cost."; exceeding 10 GB
  "requires a valid Pro, Team, or Enterprise account."; "Enterprise, Organization, and Repository admins can increase
  these limits."; "the least recently used cache entries are automatically evicted to stay within the configured
  size." rust-cache: "exceeding that limit will cause eviction of older caches." · Sources:
  https://docs.github.com/en/actions/reference/workflows-and-actions/dependency-caching and
  https://github.blog/changelog/2025-11-20-github-actions-cache-size-can-now-exceed-10-gb-per-repository/ (November 20,
  2025) · official · read 2026-10-28 · confirmed in the source.
- **R5-CI-F6.** Caches can be read and poisoned through pull requests, forks included, so a cache must never hold a
  secret. Quotes: "We recommend that you don't store any sensitive information, such as access tokens or login
  credentials,"; "Anyone with read access can create a pull request on a repository and access the contents of a
  cache."; "forks of a repository can create pull requests on the base branch and access caches on the base branch.";
  "This class of attack is known as *cache poisoning*." · Source:
  https://docs.github.com/en/actions/reference/workflows-and-actions/dependency-caching · official · read 2026-10-28 ·
  confirmed in the source.
- **R5-CI-F7.** GitHub rate-limits cache traffic per repository. Quotes: "up to 200 uploads per minute per repository,"
  and "1500 downloads per minute per repository." · Source: same page · official · read 2026-10-28 · confirmed in the
  source.
- **R5-CI-F8.** sccache can use the GitHub Actions cache as its storage. The current action is
  mozilla-actions/sccache-action v0.0.11 and the current sccache is v0.19.0 (24 Oct 2026, per the asset timestamps).
  Quotes: "For Rust code, the following environment variables should be set:" `SCCACHE_GHA_ENABLED: "true"` and
  `RUSTC_WRAPPER: "sccache"`; the README pins `mozilla-actions/sccache-action@v0.0.11`, and its version example is
  `version: "v0.16.0"`; "Versions prior to sccache v0.11.0 probably will not work." · Sources:
  https://github.com/Mozilla-Actions/sccache-action (README) and https://github.com/mozilla/sccache/releases · official
  (tool docs) · read 2026-10-28 · confirmed in the source.
- **R5-CI-F9.** sccache cannot cache the crates that matter most to us: our Wasm crate (a `cdylib`, inferred from R1's
  wasm-bindgen design), the server binary, proc-macro crates, and every incrementally compiled crate. Quotes: "Crates
  that invoke the system linker cannot be cached." "This includes `bin`, `dylib`, `cdylib`, and `proc-macro` crates.";
  "Incrementally compiled crates cannot be cached."; "By default, in the debug profile Cargo will use incremental
  compilation for workspace members and path dependencies."; "By default, absolute paths to files must match to get a
  cache hit." · Source: https://github.com/mozilla/sccache (README, "Known Caveats") · official (tool docs) · read
  2026-10-28 · confirmed in the source (the limits); inferred (which of our crates they hit).
- **R5-CI-F10.** Cargo's dev profile is incremental by default, and an environment variable overrides that. Quotes:
  dev `incremental = true`, release `incremental = false`; "The incremental value can be overridden globally with the
  `CARGO_INCREMENTAL` [environment variable]"; `"line-tables-only"` "Generates the minimal amount of debug info for
  backtraces with filename/line number info". · Source: https://doc.rust-lang.org/cargo/reference/profiles.html · official
  · read 2026-10-28 · confirmed in the source.

**Pinning the toolchain, the lockfile and the wasm-bindgen CLI**

- **R5-CI-F11.** `cargo --locked` fails the build when Cargo.lock is missing or would change, and the Cargo book
  suggests it for CI. Quotes: it "Asserts that the exact same dependencies and versions are used as when the existing
  `Cargo.lock` file was originally generated."; "It may be used in environments where deterministic builds are desired,
  such as in CI pipelines."; `--frozen` is "Equivalent to specifying both `--locked` and `--offline`."; `--timings`
  writes "`cargo-timing.html`" to "`target/cargo-timings`" and does "not provide machine-readable timing data." ·
  Source: https://doc.rust-lang.org/cargo/commands/cargo-build.html · official · read 2026-10-28 · confirmed in the
  source.
- **R5-CI-F12.** One `rust-toolchain.toml` pins the channel, the components and the extra targets, wasm32 included.
  Quotes: the example holds `channel = "nightly-2020-07-10"`, `components = [ "rustfmt", "rustc-dev" ]`, `targets = [
  "wasm32-unknown-unknown", "thumbv2-none-eabi" ]` and `profile = "minimal"`; "The `targets` setting contains a list of
  platforms to install for [cross-compilation]."; "The host platform is automatically included". The channel grammar
  allows "<major.minor>|<major.minor.patch>". Precedence, highest first: `cargo +toolchain`, `RUSTUP_TOOLCHAIN`,
  `rustup override`, then `rust-toolchain.toml`, then the default. For pinned projects, "`Cargo.lock` should probably be
  tracked too". · Source: https://rust-lang.github.io/rustup/overrides.html · official · read 2026-10-28 · confirmed in
  the source.
- **R5-CI-F13.** Since rustup 1.28.0 (2 Mar 2025), a missing pinned toolchain is no longer installed automatically, so
  CI must install it explicitly. Quotes: "rustup will no longer automatically install the active toolchain if it is not
  installed."; "run `rustup toolchain install` with no arguments"; `rustup show active-toolchain || rustup toolchain
  install` "installs the active toolchain both before and after this change." 1.28.1 added `RUSTUP_AUTO_INSTALL`, and
  1.28.2 (5 May 2025) added "`rustup set auto-install disable`". The default after 1.28.1 is not stated on the pages
  read. · Sources: https://blog.rust-lang.org/2025/03/02/Rustup-1.28.0/ and
  https://blog.rust-lang.org/2025/05/05/Rustup-1.28.2/ · official · read 2026-10-28 · confirmed in the source.
- **R5-CI-F14.** The latest wasm-bindgen is 0.2.129 (25 Sep 2026, per the asset timestamps). Each release ships prebuilt
  CLI archives per platform with `.sha256sum` files and a release attestation. Quote (0.2.129): "The CLI's schema
  version mismatch error now names the input Wasm file and the path of the running `wasm-bindgen` binary." The rule
  that the CLI and library versions must match is R1-BND-F2 (wave 1); not repeated here. · Source:
  https://github.com/wasm-bindgen/wasm-bindgen/releases · official · read 2026-10-28 · confirmed in the source. The
  x86_64 Linux archive sat behind "Show all 17 assets" and was not seen: inferred present.
- **R5-CI-F15.** taiki-e/install-action installs wasm-bindgen (also wasm-pack, cargo-nextest, cargo-deny and
  cargo-audit) at an exact version from the GitHub release and checks its SHA256. wasm-opt (binaryen) is not on its
  list. Quotes: "To install a specific version, use `@version` syntax:"; "this action will also verify SHA256 checksums
  for downloaded files for all tools covered in this section."; TOOLS.md lists "**wasm-bindgen** (alias:
  `wasm-bindgen-cli`)" from "GitHub Releases"; tools not on the list fall back to cargo-binstall, which `fallback: none`
  turns off; "consider using the `@v<major>.<minor>.<patch>` tag or their hash to pin the version."; "Pinning
  `@<tool_name>` tags by hash is strongly discouraged." · Sources: https://github.com/taiki-e/install-action (README; no
  release shown) and https://raw.githubusercontent.com/taiki-e/install-action/main/TOOLS.md · official (tool docs) ·
  read 2026-10-28 · confirmed in the source.

**Node dependencies, runners and browsers**

- **R5-CI-F16.** actions/setup-node (v7 in its examples) caches npm's global package cache, not `node_modules`, keyed
  by a lockfile it can be pointed at. It advises turning automatic caching off in privileged workflows. Quotes: `cache`
  is "Used to specify a package manager for caching in the default directory. Supported values: npm, yarn, pnpm.";
  `cache-dependency-path` is "Used to specify the path to a dependency file: package-lock.json, yarn.lock, etc.";
  `package-manager-cache` "Controls automatic caching for npm."; "For workflows with elevated privileges or access to
  sensitive information, we recommend disabling automatic caching". · Source: https://github.com/actions/setup-node
  (README) · official · read 2026-10-28 · confirmed in the source for the quotes. That automatic caching turns on when
  package.json has a `packageManager` field is inferred (WebFetch summary only).
- **R5-CI-F17.** Standard GitHub-hosted runners are free and unlimited for public repositories. ubuntu-24.04 has 4 CPUs,
  16 GB of RAM and a 14 GB SSD, an arm64 runner exists, and standard runners have no GPU. Quotes: "Use of the standard
  GitHub-hosted runners is free and unlimited on public repositories."; public `ubuntu-24.04`: CPU "4", RAM "16 GB", SSD
  "14 GB" (private: "2", "8 GB", "14 GB"); "ubuntu-24.04-arm" (arm64, 4 CPUs, 16 GB, 14 GB); GPUs appear only under
  larger runners ("GPU-powered runners"). · Source: https://docs.github.com/en/actions/reference/runners/github-hosted-runners
  (no page date) · official · read 2026-10-28 · confirmed in the source.
- **R5-CI-F18.** Playwright advises against caching browsers in CI and recommends one worker there. Its docs reference
  v1.64.1 (Docker tag), while the legacy toolkit pins 1.56.1 (TL17). Quotes: "Caching browser binaries is not
  recommended, since the amount of time it takes to restore the cache is comparable to the time it takes to download the
  binaries."; "operating system dependencies need to be installed, which are not cacheable."; "We recommend setting
  workers to "1" in CI environments to prioritize stability and reproducibility."; "headed execution requires Xvfb to
  be installed." No GPU or WebGPU guidance. · Source: https://playwright.dev/docs/ci · official · read 2026-10-28 ·
  confirmed in the source.
- **R5-CI-F19.** The Cargo book's CI guide shows a toolchain matrix and a separate, non-blocking job that tests the
  newest dependency versions. It warns that scheduled jobs can be switched off on quiet repositories. Quotes: the
  latest-deps job sets `continue-on-error: true` and `CARGO_RESOLVER_INCOMPATIBLE_RUST_VERSIONS: allow`, and runs
  `cargo update`; "A hosted CI service may disable scheduled jobs for repositories that haven’t been touched in a
  while". · Source: https://doc.rust-lang.org/cargo/guide/continuous-integration.html · official · read 2026-10-28 ·
  confirmed in the source.

**Running the gate in CI: required checks and supply chain**

- **R5-CI-F20.** A workflow skipped by a path filter blocks a required check forever, while a job skipped by a
  condition counts as passed. Quotes: skipped "by path filtering, branch filtering, or a commit message": "Associated
  checks stay in a "Pending" state and block merging"; skipped by a conditional: "The job reports "Success""; a job that
  depends on a failed job "is skipped and may not block merging." · Source:
  https://docs.github.com/en/pull-requests/collaborating-with-pull-requests/collaborating-on-repositories-with-code-quality-features/troubleshooting-required-status-checks
  · official · read 2026-10-28 · confirmed in the source.
- **R5-CI-F21.** GitHub's hardening guide asks for SHA-pinned actions, a minimal `GITHUB_TOKEN`, and untrusted input
  passed through environment variables. Quotes: "Pin references to actions to a commit SHA in your workflow files.";
  "When selecting a SHA, you should verify it is from the action's repository and not a repository fork."; "make sure
  that the `GITHUB_TOKEN` is granted the minimum required permissions."; the intermediate environment variable is the
  preferred handling of untrusted input (WebFetch summary, inferred). · Source:
  https://docs.github.com/en/actions/reference/security/secure-use · official · read 2026-10-28 · confirmed in the source
  (quotes).
- **R5-CI-F22.** GitHub now offers workflow dependency lockfiles (public preview since 14 Oct 2026). A lockfile records
  the commit of every action, transitive ones included, and GitHub calls it stronger than SHA pinning. Quotes: "The best
  measure is to lock your workflow dependencies with a lockfile."; "This records the exact commit for every action your
  workflows depend on, including transitive dependencies,"; "supersedes SHA pinning for repositories that adopt it.";
  the path is `.github/workflows/actions.lock`, made with "the `gh actions-lock` extension"; plans: "GitHub Enterprise
  Cloud, GitHub Enterprise Cloud with data residency, Team, and Free plans." (Pro is not listed); Dependabot "also
  regenerates the matching lockfile entry in the same pull request."; "Locking does not determine whether the
  dependency itself is safe". · Sources: https://docs.github.com/en/actions/reference/security/secure-use,
  https://github.blog/changelog/2026-10-14-github-actions-workflow-lockfiles-in-public-preview/ (October 14, 2026) and
  https://docs.github.com/en/actions/concepts/security/workflow-dependency-lockfiles · official · read 2026-10-28 ·
  confirmed in the source.
- **R5-CI-F23.** A repository setting can require full-length SHA pins, and in a personal account's repository the
  default token is read-only. Quotes: the option "Require actions to be pinned to a full-length commit SHA"; in
  personal-account repositories the "GITHUB_TOKEN only has read access for the contents and packages scopes." That the
  pin rule also covers GitHub's own actions is inferred (summary). · Source:
  https://docs.github.com/en/repositories/managing-your-repositorys-settings-and-features/enabling-features-for-your-repository/managing-github-actions-settings-for-a-repository
  · official · read 2026-10-28 · confirmed in the source (quotes).
- **R5-CI-F24.** Dependabot can keep action references current. Quote: "You can use Dependabot to keep the actions you
  use updated to the latest versions." The page does not say how Dependabot treats SHA-pinned references. · Source:
  https://docs.github.com/en/code-security/how-tos/secure-your-supply-chain/secure-your-dependencies/keeping-your-actions-up-to-date-with-dependabot
  · official · read 2026-10-28 · confirmed in the source (the SHA-pin behaviour: unverified).

### Legacy facts used

- **KT8 (gate.yml).** Today's CI hygiene carries over unchanged: `permissions: contents: read`, actions pinned by SHA
  with a version comment (checkout v7.0.1, setup-node v7.0.0, cache v5.1.0), `persist-credentials: false`,
  `--ignore-scripts`, environment-only inputs, concurrency with cancel-in-progress, and per-job timeouts of 10 and
  15 min (.github/workflows/gate.yml:12-17, :22-29, :34, :54). The legacy numbers set the time scale v2 is judged
  against: gate job 10-17 s; smoke job 57-59 s with Chromium cached and 63-67 s without, about 20 s of it apt. Required
  checks have been on main since 2026-10-17. For v2: the same hygiene plus Rust and TypeScript jobs.
- **KT8 plus R5-CI-F18.** Caching the browser saved about 4-8 s of a roughly 60 s job, close to what Playwright
  predicts (R5-CI-F18). Dropping that cache in v2 costs little.
- **gate.yml:67-72.** Today's CI reads a tool version from the lockfile, checks that it looks like `x.y.z`, and keys
  the cache on it. The same pattern pins the wasm-bindgen CLI to the library version in Cargo.lock (R5-CI-REC3).
- **TL1.** The gate's rules carry over to the gate v2 in CI: an explicit base ref, exit 2 for a setup problem and never
  a silent skip, `--json`, `--allow-drop` tied to the plan, and mutation proofs after every gate change. A required
  check that hangs as "Pending" (R5-CI-F20) would break the "never a silent skip" rule.
- **TL9.** The smoke harness's shape carries over to v2's Playwright job. Its network rule flips: allow the local test
  server's WebTransport and WebSocket, block everything else.
- **TL17.** Exact pins with a lockfile, `npm ci`, optional native dependencies kept out of CI, and the dependency audit
  all carry over. The open legacy recommendation applies: commit the mutation suite so the gate v2 proves itself in one
  command.
- **KT19.** `.gitignore` must add `target/`, the Wasm package output and the TypeScript caches. CI caches never hold
  secrets (R5-CI-F6).
- **Repository visibility.** The repository is public (`gh api repos/Mergincanli/Shamble-Frontlines`:
  `"visibility":"public"`, 2026-10-28). So standard runners cost nothing (R5-CI-F17), forks can open pull requests and
  read base-branch caches (R5-CI-F6), and environment reviewers are available on the Free plan (R5-PREV-F6).

### Options

**Rust build cache**

| Option | Complexity | Cost | Speed | Risk | Fit |
|---|---|---|---|---|---|
| A. No cache | none | $0 | every run builds all dependencies from scratch (time not measured) | none from caches | serves no §6 item; slow feedback |
| B. Swatinem/rust-cache v2.9.3, one shared key per build kind (native, wasm32), saved only from `main` | low: one step per job | $0 within 10 GB (F5) | restores dependency builds; our own crates always rebuild (F3) | poisoning via PRs (F6), limited by saving only on `main`; eviction past 10 GB (F5) | serves fast feedback for every §6 item; adds no engine feature |
| C. sccache v0.19.0 with the GHA backend | medium: a wrapper and environment variables | $0 within 10 GB | per-crate hits, but none for cdylib, bin, proc-macro or incremental crates (F9) | cache entries count against F7's rate limits (inferred: a cache entry per compiled crate) | as B |
| D. B and C together | higher | $0 within 10 GB | possibly more hits on dependency changes (unmeasured) | two caches inside one 10 GB budget | only if measurement shows B is not enough |

**Toolchain and CLI pinning**

| Option | Trade-off |
|---|---|
| T1. `rust-toolchain.toml` with an exact `x.y.z` channel, `targets = ["wasm32-unknown-unknown"]`, components clippy and rustfmt, `profile = "minimal"`; CI runs `rustup toolchain install` (F12, F13) | one file for local, CI and every agent; bumps are explicit PRs; the cache rolls over on each bump (F2) |
| T2. `stable` channel | no pin: a new Rust release changes the build silently; breaks "identical results everywhere" checks (R1-DET-REC6) between runs on different days |
| W1. wasm-bindgen CLI through taiki-e/install-action at the Cargo.lock version, `fallback: none` (F14, F15) | prebuilt and checksum-checked; one more third-party action to pin |
| W2. `cargo install wasm-bindgen-cli --version <lock> --locked`, cached in `~/.cargo/bin` (rust-cache `cache-bin`, F1, F2) | no extra action; builds from source on every cache miss (time not measured; `cargo install` docs not read) |

**How CI runs the gate v2**

| Option | Trade-off |
|---|---|
| G1. One workflow on push and pull_request, parallel jobs (Rust native, Rust wasm32 plus generated types, TypeScript, Playwright end-to-end, determinism), plus one final job that is the only required check | one stable required check name; jobs skip by condition, never by path filter (F20); parallel jobs make wall time the longest job, not the sum |
| G2. Several workflows with path filters, each required | path-filtered workflows leave required checks "Pending" forever (F20): rejected |
| G3. One serial job | simplest; wall time is the sum of every step |

### Recommendations

- **R5-CI-REC1. Cache the Rust dependencies with Swatinem/rust-cache v2.9.3, pinned by SHA. Use one `shared-key` per
  build kind (native, wasm32), save only from pushes to `main` (`save-if`), and let pull requests restore from
  `main`. No sccache at the start.** Reason: rust-cache covers the dependency builds that dominate a cold build and
  turns incremental builds off, which the CI cache cannot reuse anyway (F1-F3, F10). Pull requests already read
  `main`'s caches, and a pull request's own cache serves only its re-runs (F4), so saving from PRs mostly spends the
  10 GB (F5). Saving only from `main` also keeps fork-written caches out of what other runs restore (F6). sccache cannot
  cache our cdylib, our binaries, proc macros or incremental crates (F9), so it adds a second cache to one 10 GB budget
  for an unmeasured gain. Rests on: R5-CI-F1, F2, F3, F4, F5, F6, F7, F9, F10. Serves: none directly (CI speed). Every
  §6 item is built and tested faster; no engine feature is added. Spike must prove (proposed): (1) on a warm cache, no
  dependency crate is recompiled in a PR run (checked in `cargo build` output or `--timings`, F11); (2) the
  repository's total cache stays at or below 8 GB with every v2 job cached (read from the repository's Actions cache
  list), leaving room under 10 GB; (3) warm against cold time per Rust job, reported over 3 runs each.
- **R5-CI-REC2. Pin the Rust toolchain exactly and build with `--locked`.** `rust-toolchain.toml` holds an exact
  `x.y.z` channel, `targets = ["wasm32-unknown-unknown"]`, components `clippy` and `rustfmt`, and `profile = "minimal"`.
  CI installs it with `rustup toolchain install` (no arguments) before any cargo call. Cargo.lock is committed, and
  every CI cargo command takes `--locked`. A weekly scheduled job tests the latest dependency versions without blocking
  (`continue-on-error`). Toolchain bumps are their own PR. Reason: one file pins local, CI and agent builds (F12). Since
  rustup 1.28 the pinned toolchain must be installed explicitly (F13). `--locked` fails rather than silently
  re-resolving (F11). A toolchain change rolls the cache over anyway (F2). R1-DET-REC6 compares native and Wasm results,
  which only means something when both come from one compiler version. The Cargo book's latest-deps job finds
  upstream breakage without turning PRs red (F19). Rests on: R5-CI-F2, F11, F12, F13, F19; R1-DET-REC6 (wave 1).
  Serves: §6.6 and §3 through R1-DET-REC6 (reproducible runs); otherwise none. Spike must prove (proposed): (1) a fresh
  runner builds native and wasm32 from `rust-toolchain.toml` alone (no other toolchain step); (2) a PR that edits
  Cargo.toml without updating Cargo.lock fails CI at the first cargo step; (3) `rustc --version` printed in every Rust
  job's log matches the file.
- **R5-CI-REC3. Pin the wasm-bindgen CLI to the library version that Cargo.lock records, and fail CI on any
  mismatch.** As gate.yml:67-72 does for Playwright, a step reads `wasm-bindgen`'s version from Cargo.lock and checks
  that it looks like `x.y.z`. It then installs exactly that CLI through taiki-e/install-action (pinned by SHA, with
  `fallback: none`), and compares `wasm-bindgen --version` with the lock before generating bindings. Reason:
  R1-BND-F2 requires matching versions, and R1-BND-REC4's pass mark 4 asks CI to fail on a mismatch. The action
  downloads the release's prebuilt CLI and checks its SHA256 (F14, F15), which avoids a source build on a cache miss
  (W2). `fallback: none` stops a silent switch to cargo-binstall (F15). Rests on: R5-CI-F14, F15; legacy gate.yml:67-72;
  R1-BND-F2, R1-BND-REC4 (wave 1). Serves: §6.3 and §6.7 through R1-BND-REC4 (generated types for data-driven
  content). Spike must prove (proposed): (1) R1-BND-REC4's pass marks 1 and 4 in CI: regenerating the types gives zero
  diff, and a deliberate CLI/library mismatch fails the job; (2) the install step takes under 30 s (proposed, measured
  over 3 runs).
- **R5-CI-REC4. Node: setup-node v7 with `cache: npm` and `cache-dependency-path` set to the client lockfile, keeping
  `npm ci --ignore-scripts`. Do not cache Playwright's browsers; install them in each run. In any job that holds a
  secret, turn automatic package caching off.** Reason: setup-node caches the global npm cache keyed by the lockfile
  (F16). Playwright says a browser cache restores about as fast as a download (F18), and the legacy numbers agree (KT8:
  about 4-8 s saved), so the browser cache would spend part of the 10 GB (F5) for little. setup-node itself advises
  against automatic caching in privileged workflows (F16), and caches are writable from pull requests (F6). Rests on:
  R5-CI-F5, F6, F16, F18; legacy KT8, TL17. Serves: none (CI hygiene). Spike must prove (proposed): (1) a warm `npm ci`
  is faster than a cold one, reported over 3 runs each; (2) the Playwright job's wall time, browser install included,
  is reported against the legacy smoke job's 57-67 s.
- **R5-CI-REC5. Run the gate v2 in CI as option G1. One workflow on push and pull_request runs parallel jobs, and one
  final job is the only required check. Jobs skip by condition, never by path filter. Carry over the legacy hygiene:
  SHA pins, `contents: read`, `persist-credentials: false`, concurrency cancel, per-job timeouts. Long checks run on a
  schedule.** The jobs: Rust native (rustfmt, clippy, cargo test, R1-ECS-REC2's no-HashMap-iteration check), Rust
  wasm32 (build, wasm-bindgen, generated-type diff), TypeScript (tsc, lint, Vitest), Playwright end-to-end on the built
  client (TL9's harness shape), and determinism (R1-DET-REC6: two native processes, Wasm in Node; browser targets
  inside the Playwright job). The scheduled jobs: R2-TRN-REC4's 10-million-input fuzz run per decoder, and the
  latest-deps job (REC2); the fuzz run's cost and time are unknown. Add Dependabot for `github-actions` to keep the pins
  moving (F24). Asking Mustafa to turn on the "require SHA pinning" setting (F23) is a later step; lockfiles wait until
  they are generally available and confirmed for his plan (R5-CI-OQ3). Reason: one aggregate required check survives
  jobs being added or renamed, and conditional skips count as success, while path-filtered workflows hang required
  checks (F20). Parallel jobs on free 4-CPU runners cost nothing in a public repository (F17). The hygiene is GitHub's
  own advice (F21) and is already proven here (KT8). Rests on: R5-CI-F17, F20, F21, F22, F23, F24; legacy KT8, TL1, TL9;
  R1-DET-REC6, R1-ECS-REC2, R2-TRN-REC4 (wave 1). Serves: the CI side of every §6 item; the determinism job serves §6.6
  and §3 (R1-DET-REC6). Spike must prove (proposed): (1) a PR that touches only TypeScript runs, and passes, the single
  required check; (2) with warm caches, the PR wall time is at most 10 min at p50 over 5 runs and at most 20 min cold,
  per job and in total (proposed budget; VISION_V2 sets no CI target; legacy is about 1 min); (3) the mutation suite
  (TL17) makes each job fail on the fault it guards against; (4) a fork PR runs every job with no secret available
  (R5-PREV-F1).

### Open questions

- **R5-CI-OQ1. How big are our caches?** No source gives sizes for a Cargo workspace like ours (native plus wasm32,
  bevy_ecs or hecs, Rapier, quinn and wtransport). Decided by the spike's measurement against R5-CI-REC1's 8 GB pass
  mark. If it is exceeded: fewer cache keys, or Mustafa's paid limit (F5, needs Pro).
- **R5-CI-OQ2. Is sccache worth adding later?** It would help only with dependency changes and rlib crates (F9).
  Decided by measuring dependency rebuild time once the workspace exists, against REC1's pass mark 3.
- **R5-CI-OQ3. Workflow lockfiles.** They are in public preview, Pro is not among the listed plans (F22), and the
  account type of Mustafa's personal repository is not known here. Decided by the feature's general availability and
  by Mustafa's plan. Until then: SHA pins and Dependabot.
- **R5-CI-OQ4. The wasm-opt (binaryen) pin.** It is not on install-action's list (F15). Decided by whether R1 or the
  owner's toolchain uses wasm-opt or wasm-pack, then by its release page (not read).
- **R5-CI-OQ5. GPU for WebGPU tests.** Standard runners have no GPU (F17), and Playwright's CI docs say nothing about
  WebGPU (F18). Whether the end-to-end job exercises WebGPU or only the WebGL 2 fallback is decided by the owner's gate
  checks and R3. A GPU runner is a larger, paid runner (price not read).
- **R5-CI-OQ6. Dependabot and SHA pins.** The page read does not say that Dependabot updates SHA-pinned references
  (F24). Decided by a trial PR on one action.
- **R5-CI-OQ7. An arm64 determinism leg.** `ubuntu-24.04-arm` is free for public repositories (F17). Whether the
  determinism job also runs on arm64 depends on R1-DET-OQ7 (the server's CPU architecture, R6).

### Sources read (part 1)

| URL | Title | Version or page date | Tier | Read |
|---|---|---|---|---|
| https://github.com/Swatinem/rust-cache | Swatinem/rust-cache README | v2.9.3 ("20 Oct") | official (tool) | 2026-10-28 |
| https://github.com/Swatinem/rust-cache/releases | rust-cache releases | v2.9.3, v2.9.2, v2.9.1 | official (tool) | 2026-10-28 |
| https://docs.github.com/en/actions/reference/workflows-and-actions/dependency-caching | Dependency caching reference | no date | official | 2026-10-28 |
| https://github.blog/changelog/2025-11-20-github-actions-cache-size-can-now-exceed-10-gb-per-repository/ | Actions cache size can now exceed 10 GB | 2025-11-20 | official | 2026-10-28 |
| https://github.com/Mozilla-Actions/sccache-action | sccache-action README | v0.0.11 | official (tool) | 2026-10-28 |
| https://github.com/mozilla/sccache | sccache README | release not shown | official (tool) | 2026-10-28 |
| https://github.com/mozilla/sccache/releases | sccache releases | v0.19.0 (2026-10-24) | official (tool) | 2026-10-28 |
| https://doc.rust-lang.org/cargo/reference/profiles.html | Cargo Book: Profiles | no version | official | 2026-10-28 |
| https://doc.rust-lang.org/cargo/commands/cargo-build.html | Cargo Book: cargo build | no version | official | 2026-10-28 |
| https://doc.rust-lang.org/cargo/guide/continuous-integration.html | Cargo Book: Continuous Integration | no version | official | 2026-10-28 |
| https://rust-lang.github.io/rustup/overrides.html | rustup book: Overrides | no version | official | 2026-10-28 |
| https://blog.rust-lang.org/2025/03/02/Rustup-1.28.0/ | Announcing Rustup 1.28.0 | 2025-03-02 | official | 2026-10-28 |
| https://blog.rust-lang.org/2025/05/05/Rustup-1.28.2/ | Announcing rustup 1.28.2 | 2025-05-05 | official | 2026-10-28 |
| https://github.com/wasm-bindgen/wasm-bindgen/releases | wasm-bindgen releases | 0.2.129 (2026-09-25) | official | 2026-10-28 |
| https://github.com/taiki-e/install-action | install-action README | release not shown | official (tool) | 2026-10-28 |
| https://raw.githubusercontent.com/taiki-e/install-action/main/TOOLS.md | install-action TOOLS.md | main | official (tool) | 2026-10-28 |
| https://github.com/actions/setup-node | setup-node README | v7 | official | 2026-10-28 |
| https://docs.github.com/en/actions/reference/runners/github-hosted-runners | GitHub-hosted runners reference | no date | official | 2026-10-28 |
| https://playwright.dev/docs/ci | Playwright: Continuous Integration | v1.64.1 (Docker tag) | official | 2026-10-28 |
| https://docs.github.com/en/pull-requests/collaborating-with-pull-requests/collaborating-on-repositories-with-code-quality-features/troubleshooting-required-status-checks | Troubleshooting required status checks | no date | official | 2026-10-28 |
| https://docs.github.com/en/actions/reference/security/secure-use | Secure use reference | no date | official | 2026-10-28 |
| https://github.blog/changelog/2026-10-14-github-actions-workflow-lockfiles-in-public-preview/ | Workflow lockfiles in public preview | 2026-10-14 | official | 2026-10-28 |
| https://docs.github.com/en/actions/concepts/security/workflow-dependency-lockfiles | Workflow dependency lockfiles | no date | official | 2026-10-28 |
| https://docs.github.com/en/repositories/managing-your-repositorys-settings-and-features/enabling-features-for-your-repository/managing-github-actions-settings-for-a-repository | Managing GitHub Actions settings for a repository | no date | official | 2026-10-28 |
| https://docs.github.com/en/code-security/how-tos/secure-your-supply-chain/secure-your-dependencies/keeping-your-actions-up-to-date-with-dependabot | Keeping your actions up to date with Dependabot | no date | official | 2026-10-28 |
| https://docs.github.com/en/actions/how-tos/manage-runners/github-hosted-runners/customize-runners | Customizing GitHub-hosted runners (title only; nothing used) | no date | official | 2026-10-28 |

### Sources not read (part 1)

| URL | Reason |
|---|---|
| https://internals.rust-lang.org/t/seeking-beta-testers-for-rustup-v1-28-0/22060 | outside the allow list (search lead) |
| https://sia.hackernoon.com/whats-new-in-rustup-1280 | outside the allow list (search lead) |
| https://daleseo.com/rustup/ ; https://openmamba.org/en/rpms/base/rustup | outside the allow list (search leads) |
| https://azukiazusa.dev/en/blog/github-actions-workflow-lockfiles ; https://horadecodar.com.br/?p=47732 ; https://techenclave.com/news/items/github-actions-previews-lockfiles-for-workflow-dependencies | outside the allow list (search leads) |
| https://www.mexc.com/news/181586 ; https://fast.io/resources/github-storage-limit/ ; https://github.internet2.edu/... ; https://gitea.psi.ch/... | outside the allow list (search leads) |
| https://redirect.github.com/rust-lang/rustup/pull/3985 (and other redirect.github.com leads) | outside the allow list (redirect host) |
| https://docs.github.com/en/actions/how-tos/secure-your-work/lock-workflow-dependencies | on the allow list; not needed (concept page and changelog read) |
| https://docs.github.com/en/actions/reference/workflows-and-actions/workflow-syntax (characters 200000 onward) | not read: the needed sections were found earlier |
| wasm-bindgen guide (wasm-bindgen.github.io) | outside the allow list; R1-BND-F2 covers the version rule |
| doc.rust-lang.org cargo install page | on the allow list; not read in this run (option W2's build time stays unmeasured) |

---

## Section 2. R5 helper: CI with Rust caching; preview links per PR (part 2: preview links per PR)

The brief's R5 row (docs/V2_PHASE0_BRIEF.md:125), quoted in full in section 1; this section answers "**preview links
per PR**". Mustafa opens each link in Brave on Windows. Which host serves the previews, and its limits and prices, is
R6's: this section names the hosts considered and what the workflow needs from them, and leaves prices to R6.

### Findings

**Pull requests from forks and the secrets**

- **R5-PREV-F1.** A pull request from a fork gets no secrets and a read-only token. Quotes: "secrets are not passed to
  the runner when a workflow is triggered from a forked repository."; "The `GITHUB_TOKEN` has read-only permissions in
  pull requests from forked repositories." · Source:
  https://docs.github.com/en/actions/reference/workflows-and-actions/events-that-trigger-workflows (`pull_request`; no
  page date) · official · read 2026-10-28 · confirmed in the source.
- **R5-PREV-F2.** `pull_request_target` runs the base repository's workflow with its trust, and GitHub says not to use
  it to build pull-request code. Quotes: "This event runs in the context of the default branch of the base repository,
  rather than in the context of the merge commit."; "Running untrusted code on the `pull_request_target` trigger may
  lead to security vulnerabilities."; "Avoid using this event if you need to build or run code from the pull request."
  · Source: same page (`pull_request_target`) · official · read 2026-10-28 · confirmed in the source.
- **R5-PREV-F3.** A `workflow_run` workflow gets secrets and a write token even when the run that triggered it had
  neither, so it must treat that run's artifacts as untrusted and never check out its code. Quotes: it "is able to
  access secrets and write tokens, even if the previous workflow was not."; "Running untrusted code on the
  `workflow_run` trigger may lead to security vulnerabilities."; "Workflows triggered on `workflow_run` should treat
  artifacts uploaded from other workflows with caution."; such workflows "must not explicitly check out untrusted
  code." · Sources: the same events page (`workflow_run`) and
  https://docs.github.com/en/actions/reference/security/secure-use · official · read 2026-10-28 · confirmed in the
  source.
- **R5-PREV-F4.** A workflow can run when a pull request closes or merges. By default `pull_request` runs only on
  opened, synchronize and reopened. Quotes: "To run a workflow when a pull request merges, use the `pull_request`
  `closed` event type along with a conditional that checks the `merged` value of the event."; "By default, a workflow
  only runs when a `pull_request` event's activity type is `opened`, `synchronize`, or `reopened`." · Source: the same
  events page · official · read 2026-10-28 · confirmed in the source.
- **R5-PREV-F5.** Workflow runs from first-time contributors' forks wait for a maintainer's approval by default, and
  stricter settings exist. Quotes: "By default, all first-time contributors require approval to run workflows."; the
  options are "Require approval for first-time contributors who are new to GitHub", "Require approval for first-time
  contributors" and "Require approval for all external contributors"; approvers should be "especially alert to any
  proposed changes in the `.github/workflows/` directory". · Sources:
  https://docs.github.com/en/repositories/managing-your-repositorys-settings-and-features/enabling-features-for-your-repository/managing-github-actions-settings-for-a-repository
  and https://docs.github.com/en/actions/how-tos/manage-workflow-runs/approve-runs-from-forks · official · read
  2026-10-28 · confirmed in the source.
- **R5-PREV-F6.** A GitHub environment can hold the deploy token behind rules, and the reviewer rule is available to
  public repositories on the Free plan. Quotes: "required reviewers are only available for public repositories" (on
  Free, Pro and Team); "a job cannot access environment secrets until one of the required reviewers approves it";
  "Deployment branches and tags are available for all public repositories."; "Running a workflow that references an
  environment that does not exist will create an environment with the referenced name."; "only repository admins can
  configure the environment". · Sources:
  https://docs.github.com/en/actions/reference/workflows-and-actions/deployments-and-environments and
  https://docs.github.com/en/actions/how-tos/deploy/configure-and-manage-deployments/manage-environments · official ·
  read 2026-10-28 · confirmed in the source.
- **R5-PREV-F7.** A job's environment can carry a URL, which becomes the deployment's `environment_url`. Quotes: "as an
  environment object with the `name` and `url`."; "The URL maps to `environment_url` in the deployments API." That
  GitHub shows this link on the pull request page is not stated on the pages read. · Sources:
  https://docs.github.com/en/actions/reference/workflows-and-actions/workflow-syntax (`jobs.<job_id>.environment`) and
  the manage-environments page · official · read 2026-10-28 · confirmed in the source (the API field); the PR-page link
  is unverified.
- **R5-PREV-F8.** Writing a deployment or commenting on a pull request needs a named token scope. Quotes: "`deployments:
  write` permits an action to create a new deployment."; "`pull-requests: write` permits an action to add a label to a
  pull request." (the comment scope is inferred from the same row). · Source: the workflow-syntax page (`permissions`) ·
  official · read 2026-10-28 · confirmed in the source (quotes).
- **R5-PREV-F9.** A later job or run downloads the build through actions/download-artifact (v8). Artifacts are kept
  90 days by default; public repositories can set 1 to 90. Quotes: "This is required when downloading artifacts from a
  different repository or from a different workflow run." (token "with actions:read permissions"); artifacts and logs
  are "retained for 90 days before they are automatically deleted." · Sources: https://github.com/actions/download-artifact
  (README) and the repository Actions-settings page · official · read 2026-10-28 · confirmed in the source.

**Hosts considered and how they deploy a preview**

- **R5-PREV-F10.** Cloudflare Pages takes a direct upload from GitHub Actions through cloudflare/wrangler-action v4,
  with an account-scoped "Pages Edit" API token stored as a repository secret. With the GitHub token it can also
  create a GitHub Deployment. Quotes: the example command is `"pages deploy YOUR_DIRECTORY_OF_STATIC_ASSETS
  --project-name=YOUR_PROJECT_NAME"`; it "automatically triggers on the current git branch, unless you add a `branch`
  option to the `with` section."; for the token, "select *Account*, *Cloudflare Pages* and *Edit*"; with `deployments:
  write` the action can "create a Deployment on your behalf." · Source:
  https://developers.cloudflare.com/pages/how-to/use-direct-upload-with-continuous-integration/ (Last updated Sep 18,
  2026) · official · read 2026-10-28 · confirmed in the source.
- **R5-PREV-F11.** wrangler-action returns the preview's URLs as outputs and can pin Wrangler's version. Quotes:
  "utilize the `deployment-url` output variable to get the URL of the deployment"; "Pages deployments will also provide
  their alias URL (since Wrangler v3.78.0)"; "The action exposes `preview-url`, `preview-deployment-url`,
  `preview-name`, `preview-id`,"; "You can pass `wranglerVersion` to install a specific version of Wrangler from
  NPM."; "Your API token is encrypted by GitHub, and the action won't print it into logs." Nothing on forks. · Source:
  https://github.com/cloudflare/wrangler-action (README; examples `@v4`, "defaults to **Wrangler v4**") · official (tool)
  · read 2026-10-28 · confirmed in the source.
- **R5-PREV-F12.** Cloudflare Pages previews are public by default, but can be limited to the Cloudflare account. They
  send `noindex`, and get a stable alias per branch. Quotes: "By default, these deployment URLs are public."; "Enabling
  the access policy will restrict viewing project deployments to your Cloudflare account."; "By default, every preview
  deployment generated by Cloudflare Pages includes the X-Robots-Tag: noindex HTTP response header."; "Branch name
  aliases are lowercased and non-alphanumeric characters are replaced with a hyphen." (`fix/api` becomes
  `fix-api.<project>.pages.dev`). · Source: https://developers.cloudflare.com/pages/configuration/preview-deployments/
  (Last updated Oct 12, 2026) · official · read 2026-10-28 · confirmed in the source.
- **R5-PREV-F13.** Cloudflare Pages deployments are deleted with Wrangler, but a branch's latest deployment cannot be.
  Quotes: `npx wrangler pages deployment delete <DEPLOYMENT_ID> --project-name <PROJECT_NAME>`; "Use the `--force` (or
  `-f`) flag to skip the confirmation prompt, and to force deletion of aliased deployments."; "The latest deployment for
  a branch cannot be deleted." · Source: same page · official · read 2026-10-28 · confirmed in the source. How the two
  sentences fit together is not explained (R5-PREV-OQ2).
- **R5-PREV-F14.** Cloudflare Workers make preview aliases at upload. Those are public when version URLs are on, can be
  put behind Cloudflare Access, and only the 1000 most recent are kept. Quotes: `npx wrangler versions upload
  --preview-alias staging`; URL "<alias>-<worker-name>.<subdomain>.workers.dev"; "If Version URLs are enabled, the URL
  is public and available after version creation."; "To require visitors to sign in, use Cloudflare Access."; "The
  alias and Worker name combined with a dash must not exceed 63 characters due to DNS label limits."; "the 1000 most
  recently deployed aliases are retained."; "When a new alias is created beyond this limit, the least recently deployed
  alias is deleted." The page says nothing about static assets or `noindex`. · Source:
  https://developers.cloudflare.com/workers/configuration/previews/ (Last updated Sep 22, 2026) · official · read
  2026-10-28 · confirmed in the source.
- **R5-PREV-F15.** Netlify builds a Deploy Preview for each pull request on its own servers, at a numbered URL, with a
  status on the PR, a password option and `noindex`. Quotes: "When you open a pull / merge request in a connected site
  repository from GitHub, ... Netlify automatically builds a Deploy Preview for you."; "`deploy-preview-42--mysitename.netlify.app`";
  "You can control who can access your site's Deploy Previews by requiring a password"; "Netlify adds an `X-Robots-Tag:
  noindex` header to Deploy Previews."; "a Deploy Preview status will appear on your pull / merge request."; a comment
  with the link comes only "If you have [deploy notifications] enabled". · Source:
  https://docs.netlify.com/site-deploys/deploy-previews/ (Last updated: Oct 8, 2026) · official · read 2026-10-28 ·
  confirmed in the source.
- **R5-PREV-F16.** For public repositories, Netlify by default keeps sensitive variables out of untrusted (fork)
  builds. Quote: "our [sensitive variable policy] default setting for public repositories prevents untrusted pull/merge
  requests from building with sensitive variable values. When there are potentially sensitive environment variables in
  your project configuration, untrusted deploys require approval." · Source:
  https://docs.netlify.com/platform/security-checklist/ (Last updated: Sep 30, 2026) · official · read 2026-10-28 ·
  confirmed in the source.
- **R5-PREV-F17.** Vercel can deploy a preview that Actions built, using three repository secrets, with its own Git
  deployments switched off. Quotes: "vercel pull --yes --environment=preview --token=${{ secrets.VERCEL_TOKEN }}",
  "vercel build --token=...", "vercel deploy --prebuilt --token=..."; secrets `VERCEL_TOKEN`, `VERCEL_ORG_ID`,
  `VERCEL_PROJECT_ID`; "Set `"git": { "deploymentEnabled": false }` in `vercel.json` to hand deployment to Actions
  cleanly." · Source: https://vercel.com/kb/guide/how-can-i-use-github-actions-with-vercel (published 2026-07-23,
  updated 2026-10-16) · official · read 2026-10-28 · confirmed in the source. Vercel's Hobby plan terms are R6's.

**Headers that previews need**

- **R5-PREV-F18.** Cloudflare Pages reads response headers from a `_headers` file in the build output, and its rules
  can match one hostname pattern, such as preview subdomains. Quotes: "a plain text file called `_headers` without a
  file extension, in the static asset directory of your project."; the example rules `https://:project.pages.dev/*` and
  `https://:version.:project.pages.dev/*` (the latter with `X-Robots-Tag: noindex`); "You may define up to 100 header
  rules."; "Each line in the `_headers` file has a 2,000 character limit."; the headers do not apply to Pages Functions
  responses. · Source: https://developers.cloudflare.com/pages/configuration/headers/ (Last updated Aug 25, 2026) ·
  official · read 2026-10-28 · confirmed in the source. Whether the branch-alias hostnames match
  `:version.:project.pages.dev` is inferred (one label before the project, F12).
- **R5-PREV-F19.** Vite 8 (v8.3.4) sets response headers for both its dev server and its local preview server, and says
  the preview server is not for production. Quotes: `server.headers` and `preview.headers` are typed
  `OutgoingHttpHeaders`, "Specify server response headers."; "`vite preview` is intended for previewing the build
  locally and not meant as a production server."; "By default, the build output will be placed at `dist`." · Sources:
  https://vite.dev/config/server-options, https://vite.dev/config/preview-options and
  https://vite.dev/guide/static-deploy · official · read 2026-10-28 · confirmed in the source.
- **R5-PREV-F20.** The fast way to load Wasm needs the `.wasm` file served as `application/wasm`; otherwise it fails.
  Quotes: "If the response has an incorrect MIME type (instead of `application/wasm`), the promise rejects with a
  [`TypeError`]"; "`.wasm` files should be returned with an `application/wasm` MIME type by the server." · Source:
  https://developer.mozilla.org/en-US/docs/WebAssembly/Reference/JavaScript_interface/instantiateStreaming_static (Last
  modified Sep 22, 2026) · official · read 2026-10-28 · confirmed in the source.

### Legacy facts used

- **KT8 (gate.yml).** Today's CI has no preview, no deploy step and no secret (`permissions: contents: read`,
  gate.yml:12-13). A preview workflow is the first CI job to hold a secret, so the legacy rule "no
  `pull_request_target`" (security-auditor, phase 0, cited in KT8) carries over and extends to `workflow_run`
  (R5-PREV-F3).
- **TL9.** The smoke harness fails on any outside network request. A preview link adds a check after deployment that
  the deployed build is the one CI tested (R5-PREV-REC4).
- **CLAUDE.md, "People".** Mustafa plays in Brave on Windows, and his screenshots are the final visual gate. The
  preview link is how that check reaches him for v2. Isolation's `crossOriginIsolated` must therefore be checked in
  Brave too (R1-BND-REC5 pass mark).
- **Repository visibility.** Public (section 1): anyone can fork and open a pull request (R5-PREV-F1, F5), and
  environment reviewers are on the Free plan (R5-PREV-F6).

### Options

**Workflow shape (whatever the host)**

| Option | Complexity | Risk | Effort | Fit |
|---|---|---|---|---|
| P1. Build in a `pull_request` job with no secret, upload `dist` as an artifact; a second job (`needs: build`) in an environment `preview` holds the host token, downloads the artifact, uploads it to the host and posts the link. Only same-repository PRs deploy; forks get CI but no preview | low | PR code never runs where the token is; the deploy job runs only our pinned tools on static files (F1, F6) | small | Mustafa's own branches get links; forks rarely matter today |
| P2. Like P1, but forks get a preview after a reviewer's approval of the `preview` environment | low-medium | the approver must read the diff first (F5); static files from a stranger then sit on our preview domain (phishing risk, inferred) | small | only once outside contributors appear |
| P3. `workflow_run` deploys every PR's artifact, forks included | medium | gets secrets and write tokens (F3); a mistake lets PR code run with secrets | medium | not needed: rejected for now |
| P4. `pull_request_target` | low | GitHub warns against building PR code with it (F2) | small | rejected |
| P5. The host builds from Git itself (Netlify, Vercel, Cloudflare Git integration) | low on our side | the host builds fork code on its servers (Netlify guards secrets by default, F16); the host's build image needs our pinned Rust, wasm-bindgen and Node versions, and CI and the host build separately, so the preview is not the artifact CI tested (inferred) | medium | breaks "the preview is the tested build" |

**Hosts considered (technical fit only; limits and prices are R6's)**

| Host | Deploy from CI | Per-PR link | Private option | `noindex` by default | Custom headers for COOP/COEP | Cleanup |
|---|---|---|---|---|---|---|
| Cloudflare Pages | wrangler-action v4, `pages deploy` (F10, F11) | branch alias (F12) | access policy, Cloudflare account (F12) | yes (F12) | `_headers`, host-scoped (F18) | delete by id; a branch's latest deployment cannot be deleted (F13) |
| Cloudflare Workers static assets | `versions upload --preview-alias` (F14) | alias URL (F14) | Cloudflare Access (F14) | not stated | `_headers` (R6's notes) | the 1000 most recent kept (F14) |
| Netlify | Git integration (F15); CLI deploy not read | `deploy-preview-N` (F15) | password (F15) | yes (F15) | `_headers` (R6's notes: global, not per context) | not read |
| Vercel | `vercel deploy --prebuilt` (F17) | preview URL | Deployment Protection (R6's notes) | not read | `vercel.json` headers (R6's notes) | not read |
| GitHub Pages | Actions (Vite guide) | no per-PR site natively (inferred) | none read | not read | none found (R6's notes) | not read |

### Recommendations

- **R5-PREV-REC1. Use workflow shape P1. Build in the ordinary `pull_request` workflow with no secret, and deploy
  from a separate job that runs in an environment named `preview`, holds the only host token, never checks out or runs
  PR code, and runs only for PRs from this repository. Fork PRs get the full CI but no preview (P2 later, if outside
  contributors appear). No `pull_request_target`, no `workflow_run`.** The deploy job's permissions: `contents: read`,
  `actions: read` (artifact), `deployments: write`, `pull-requests: write`. It sets the environment's `url` to the
  preview link, posts or updates one PR comment with that link, and turns automatic package caching off (R5-CI-F16).
  Reason: fork runs get no secrets anyway (F1). `pull_request_target` and `workflow_run` hand secrets and write tokens
  to runs triggered by pull requests (F2, F3), which today's CI has never done (KT8). Splitting build and deploy keeps
  the token away from every build script and npm or cargo dependency. The environment gives one place for the secret
  and, when wanted, a reviewer gate (F6). Deploying the artifact CI built means the link shows the build that was
  tested (F9). Rests on: R5-PREV-F1, F2, F3, F5, F6, F7, F8, F9; R5-CI-F6, R5-CI-F16; legacy KT8. Serves: none directly
  (process). It carries Mustafa's Brave check for every player-facing §6 item (§6.2, §6.4, §6.9). Spike must prove
  (proposed): (1) for a same-repository PR, the link exists within 5 min after the build job ends (median of 5 runs);
  (2) a fork PR runs the build and test jobs, the deploy job is skipped, and its log shows no secret; (3) the deploy
  job contains no `actions/checkout` of the PR head and no `npm ci` or `cargo` step (checked by reading the workflow, and
  by a test PR that changes package.json scripts: the change never runs in the deploy job).
- **R5-PREV-REC2. Choose a host that meets these needs: deploy a prebuilt directory from Actions with a scoped
  token; a stable link per PR branch; `noindex`; custom headers per hostname; an access option; deletion through a
  CLI or API. On the docs read, Cloudflare Pages meets every need. Cloudflare Workers static assets, Netlify and Vercel
  meet most. GitHub Pages lacks per-PR sites and custom headers. The choice and its price are R6's.** Reason: F10-F13
  and F18 cover every need for Cloudflare Pages; F14 shows Workers previews without a stated `noindex`; Netlify's
  natural path is P5 (F15), which builds outside our tested CI; Vercel's prebuilt path works (F17), but its plan terms
  are R6's. Rests on: R5-PREV-F10, F11, F12, F13, F14, F15, F17, F18. Serves: none directly (process, as REC1). Spike
  must prove (proposed): on the chosen host, every row of REC4's header check passes on a real preview link.
- **R5-PREV-REC3. Clean up when the PR closes, and sweep weekly.** A `pull_request` `closed` job (same-repository
  only, environment `preview`) deletes the PR branch's preview deployments (on Pages: `wrangler pages deployment
  delete … --force`) and edits the PR comment to say the link is gone. A weekly scheduled job lists and deletes
  previews of branches with no open PR. Build artifacts keep 7 days (`retention-days`), not the default 90. Reason:
  closing is an event workflows can catch (F4). Pages deletes by id but keeps a branch's latest deployment (F13), so a
  sweep backs up the close job, and what stays reachable must be known (R5-PREV-OQ2). Short artifact retention limits
  what a public repository keeps lying around (F9). Rests on: R5-PREV-F4, F9, F13, F14. Serves: none. Spike must prove
  (proposed): 24 h after a PR closes, its preview URL no longer serves the build (HTTP 404, or the access wall), or the
  remaining case is written down with the reason; the sweep leaves 0 previews of closed PRs older than 7 days.
- **R5-PREV-REC4. Preview headers: a preview-only header set, scoped by hostname, generated by the build. The deploy
  job checks the headers.** The set: COOP `same-origin` and COEP `require-corp` on preview hostnames only (R1-BND-REC5
  G3; production stays non-isolated until REC5's test decides), `X-Robots-Tag: noindex` (the host default, F12, F15),
  `.wasm` served as `application/wasm`, and R6's content security policy once written. Use the same values in Vite's
  `server.headers` and `preview.headers`, so local development matches the preview (F19). After deploying, the job
  fetches the preview's HTML and `.wasm` and fails if a header is missing. Reason: R1-BND-REC5 allows isolation in
  development and previews and asks that `crossOriginIsolated` be proven on a preview link. Cloudflare's `_headers`
  rules can target one hostname pattern (F18), and Vite sets the same headers locally (F19). The wrong `.wasm` type
  breaks streaming instantiation (F20). Rests on: R5-PREV-F12, F15, F18, F19, F20; R1-BND-REC5 (wave 1). Serves: §4 (four
  desktop browsers, including Safari) through R1-BND-REC5; no §6 item names isolation (as R1-BND-REC5 says). Spike must
  prove (proposed): (1) the deploy job's header check passes, and fails when a header is removed (mutation); (2)
  `crossOriginIsolated` is true on a preview link in Chrome, Edge, Firefox, Safari and Brave (R1-BND-REC5's pass mark,
  plus Brave for Mustafa); (3) the production build's headers do not include COOP/COEP.
- **R5-PREV-REC5. Keep previews public but unlisted at the start (`noindex`, branch-name links), and offer Mustafa
  the access wall as a choice, not a default.** Reason: Mustafa opens links in Brave (CLAUDE.md), and an access policy
  limits viewing to the Cloudflare account (F12) or Cloudflare Access (F14), which puts a login in front of every
  check. A preview contains only what the public repository already contains (inferred), so the main exposure is
  search indexing, which `noindex` handles (F12, F15). This changes once a preview talks to a real server or carries
  unreleased art: then the access wall is on. Rests on: R5-PREV-F12, F14, F15. Serves: none (process). Spike must
  prove (proposed): a preview link opens in Brave on Windows without a login. If Mustafa picks the access wall, he
  logs in once per session and the link works after that.

### Open questions

- **R5-PREV-OQ1. Access wall or public-unlisted previews?** It is Mustafa's call (R5-PREV-REC5). What decides it: does
  he accept a Cloudflare login per Brave session, and will previews carry anything not already public?
- **R5-PREV-OQ2. Does a closed PR's preview really disappear on Cloudflare Pages?** "The latest deployment for a branch
  cannot be deleted" and "`--force` ... to force deletion of aliased deployments" (F13) leave the last link's fate
  unclear. Decided by a spike on a test project, or by R6 reading the Pages API docs.
- **R5-PREV-OQ3. Do previews need a game server?** At the start a preview can run the sim locally in the worker
  (R1-BND-REC6: offline play). Multiplayer previews need an EU server and certificates (R2-TRN-REC5: self-signed for at
  most 14 days, pinned via `serverCertificateHashes`, in dev and previews). Decided by phase 1's slice plan and R6's
  hosting.
- **R5-PREV-OQ4. Which host, at what price?** R6 decides from its price and limit research. This draft gives only the
  technical needs (R5-PREV-REC2).
- **R5-PREV-OQ5. Does GitHub show the environment URL on the PR page?** It is not stated in the pages read (F7). If
  not, the PR comment (REC1) is the link Mustafa uses. Decided by the spike.
- **R5-PREV-OQ6. Do branch aliases match `https://:version.:project.pages.dev/*`?** It is inferred only (F18). Decided
  by REC4's header check on a real preview.

### Sources read (part 2)

| URL | Title | Version or page date | Tier | Read |
|---|---|---|---|---|
| https://docs.github.com/en/actions/reference/workflows-and-actions/events-that-trigger-workflows | Events that trigger workflows | no date | official | 2026-10-28 |
| https://docs.github.com/en/actions/reference/security/secure-use | Secure use reference | no date | official | 2026-10-28 |
| https://docs.github.com/en/actions/how-tos/manage-workflow-runs/approve-runs-from-forks | Approving workflow runs from forks | no date | official | 2026-10-28 |
| https://docs.github.com/en/repositories/managing-your-repositorys-settings-and-features/enabling-features-for-your-repository/managing-github-actions-settings-for-a-repository | Managing GitHub Actions settings for a repository | no date | official | 2026-10-28 |
| https://docs.github.com/en/actions/reference/workflows-and-actions/deployments-and-environments | Deployments and environments | no date | official | 2026-10-28 |
| https://docs.github.com/en/actions/how-tos/deploy/configure-and-manage-deployments/manage-environments | Managing environments for deployment | no date | official | 2026-10-28 |
| https://docs.github.com/en/actions/reference/workflows-and-actions/workflow-syntax | Workflow syntax (characters 0-200000) | no date | official | 2026-10-28 |
| https://github.com/actions/download-artifact | download-artifact README | v8 | official | 2026-10-28 |
| https://developers.cloudflare.com/pages/how-to/use-direct-upload-with-continuous-integration/ | Pages: Direct Upload with CI | Last updated Sep 18, 2026 | official | 2026-10-28 |
| https://github.com/cloudflare/wrangler-action | wrangler-action README | v4 | official (tool) | 2026-10-28 |
| https://developers.cloudflare.com/pages/configuration/preview-deployments/ | Pages: Preview deployments | Last updated Oct 12, 2026 | official | 2026-10-28 |
| https://developers.cloudflare.com/pages/configuration/headers/ | Pages: Headers | Last updated Aug 25, 2026 | official | 2026-10-28 |
| https://developers.cloudflare.com/workers/configuration/previews/ | Workers: Preview URLs | Last updated Sep 22, 2026 | official | 2026-10-28 |
| https://docs.netlify.com/site-deploys/deploy-previews/ | Netlify: Deploy Previews | Last updated Oct 8, 2026 | official | 2026-10-28 |
| https://docs.netlify.com/platform/security-checklist/ | Netlify: Security checklist | Last updated Sep 30, 2026 | official | 2026-10-28 |
| https://vercel.com/kb/guide/how-can-i-use-github-actions-with-vercel | Vercel: GitHub Actions with Vercel | updated 2026-10-16 | official | 2026-10-28 |
| https://vite.dev/config/server-options | Vite: Server options | v8.3.4 | official | 2026-10-28 |
| https://vite.dev/config/preview-options | Vite: Preview options | v8.3.4 | official | 2026-10-28 |
| https://vite.dev/guide/static-deploy | Vite: Deploying a static site | v8.3.4 | official | 2026-10-28 |
| https://developer.mozilla.org/en-US/docs/WebAssembly/Reference/JavaScript_interface/instantiateStreaming_static | MDN: WebAssembly.instantiateStreaming() | Last modified Sep 22, 2026 | official | 2026-10-28 |

### Sources not read (part 2)

| URL | Reason |
|---|---|
| https://www.netlify.com/blog/2020/05/19/environment-variables-and-preview-deploys-get-a-boost/ | outside the allow list (www.netlify.com; search lead) |
| https://answers.netlify.com/t/how-is-determined-if-environment-variables-are-sensitive-or-not/41608 ; https://community.netlify.com/... | outside the allow list (search leads) |
| https://docs.netlify.com/build/environment-variables/overview/ | on the allow list; not needed (the security checklist gave the policy's default) |
| Netlify CLI deploy, Vercel and Netlify cleanup, Cloudflare Pages REST API | on the allow list; not read in this run (R5-PREV-OQ2, R6) |

---

## Cross-topic notes

- **For the R5 owner (test-engineer):** R5-CI-REC5's job list is a frame for your gate v2 checks; the check names and
  pass marks are yours. R1-ECS-REC2 pass mark 2 (no HashMap iteration in the sim) needs a tool from you; CI only runs
  it. The Playwright job's WebGPU coverage is R5-CI-OQ5 (no GPU on standard runners). Commit the mutation suite (TL17)
  so each CI job's guard is proven.
- **For R6 (infra-engineer, hosting and costs):** previews need the R5-PREV-REC2 needs list. Please settle
  R5-PREV-OQ2 (Pages deletion of a branch's latest deployment) and R5-PREV-OQ4 (host and price), and whether a GPU
  runner (R5-CI-OQ5) or more than 10 GB of cache (R5-CI-F5, Pro account) would ever be paid. The server CPU
  architecture (R1-DET-OQ7) decides R5-CI-OQ7's arm64 leg.
- **For R6's security baseline (security-auditor):** the preview deploy job is the first CI job with a secret
  (R5-PREV-REC1). Dependency audits (cargo-deny or cargo-audit; both installable through install-action, R5-CI-F15) and
  the CSP that REC4's header set waits for are yours.
- **For Mustafa (settings he owns, never changed by agents):** the required check name for the gate v2, the
  `preview` environment and its secret, the fork-approval setting, and, later, the SHA-pinning requirement and
  workflow lockfiles.
