# R5 verifier spot-check: the report, verbatim

VERDICT: PASS WITH NOTES. All 60 claims I checked against their sources are confirmed (0 wrong, 0 unsupported). The critical read found more than nits: items 1 and 2 below need a fix by the owner.

Gate: skipped as instructed (this is a research check; the main session runs the gate).
Symbol diff: not applicable.
Evidence: every page was opened with WebFetch on 2026-10-29, on allow-listed domains only. My notes, with every exact quote, are in /home/user/Shamble-Frontlines/.checkpoints/v2-research/verifier-r5.md (ends with DONE). The file checked is /home/user/Shamble-Frontlines/docs/research/r5-toolchain-quality.md.

**Research spot-check (claim · source · verdict · location and exact words)**

Question 1, repository layout (R5-LAYOUT):
- **R5-LAYOUT-F5**, Rust 1.99.1 on 15 Oct 2026 · blog.rust-lang.org/releases · confirmed · "Releases in 2026": "Announcing Rust 1.99.1" (Oct. 15, 2026); "Announcing Rust 1.99.0" (Oct. 1, 2026)
- **R5-LAYOUT-F1, F2**, one lockfile, one target folder, key inheritance · cargo/reference/workspaces.html · confirmed · "All packages share a common `Cargo.lock` file which resides in the _workspace root_."; "...a directory named `target` in the _workspace root_."; "`{key}.workspace = true`"; "(requires Rust 1.99+)"
- **R5-LAYOUT-F4**, feature unification · cargo/reference/features.html, "Feature unification" · confirmed · "Cargo will use the union of all features enabled on that dependency when building it."; "features should be _additive_"
- **R5-LAYOUT-F7**, ts-rs 12.0.1 and its export folder · docs.rs/ts-rs · confirmed · "ts-rs 12.0.1"; "When running `cargo test` or `cargo test export_bindings`, the following TypeScript type will be exported"; "Base directory into which bindings will be exported"
- **R5-LAYOUT-F12**, preview headers · vite.dev/config/preview-options · confirmed · v8.3.4; "Specify server response headers."; "Unless noted, the options in this section are only applied to preview."
- **R5-LAYOUT-F14** (and R5-GATE-F4), cargo-deny bans · cargo-deny bans/cfg.html · confirmed · `deny` "Determines specific crates that are denied."; `wrappers` "allows specific crates to have a direct dependency on the banned crate" "but denies all transitive dependencies on it."
- **R5-LAYOUT-F15**, Vite manifest fields · vite.dev/guide/backend-integration · confirmed · "The input file name of this chunk / asset if known"; "Whether this chunk or asset is an entry point"; "The list of statically imported chunks by this chunk"; "The list of dynamically imported chunks by this chunk"

Question 2, strict TypeScript and lint (R5-TSLINT):
- **R5-TSLINT-F1**, TypeScript releases · github.com/microsoft/TypeScript/releases · confirmed · "TypeScript 7.0.2" 20 Aug (Latest); "TypeScript 7.1 Beta" 20 Oct; "TypeScript 6.0.3" 16 Apr; "TypeScript 6.0" 23 Mar
- **R5-TSLINT-F4**, supported TypeScript range · typescript-eslint.io dependency-versions (v8.72.1) · confirmed · "The version range of TypeScript currently supported is `>=4.8.4 <6.1.0`."; "...ESLint currently supported is `^8.57.0 || ^9.0.0 || ^10.0.0`."
- **R5-TSLINT-F5**, no TypeScript 7 support yet · typescript-eslint issue #12518 · confirmed · "opened on Jul 8, 2026", "closed this as not planned"; "typescript-eslint isn't compatible with TS 7 at this time, because there is no TS 7 API at this time."; marked blocked on Aug 10, 2026, pointing to #10940
- **R5-TSLINT-F13**, `strict` configs are not semver-stable · typescript-eslint.io/users/configs · confirmed · "With the exception of `all`, `strict`, and `strict-type-checked`, all configurations are considered "stable"."; "not considered "stable" under Semantic Versioning (semver)."
  - The proficiency caveat the file marks "inferred" is in the source: "only if a nontrivial percentage" … "of its developers are highly proficient in TypeScript."
- **R5-TSLINT-F15**, typed lint costs a project build · typescript-eslint typed-linting page · confirmed · "you incur the performance penalty of asking TypeScript to do a build of your project before ESLint can do its linting."; "a few seconds or less"
- **R5-TSLINT-F16**, `no-floating-promises` · its rule page · confirmed · "Extending "plugin:@typescript-eslint/recommended-type-checked" in an ESLint configuration enables this rule."
  - The file states this config membership without a quote; the owner can add this one.
- **R5-TSLINT-F21**, ESLint recommends a formatter · eslint.org blog · confirmed · "Published 26 Oct, 2023"; "We recommend using a source code formatter instead of ESLint for formatting your code."; "will not be removed until at least ESLint v10.0.0"

Question 3, clippy and rustfmt (R5-RUSTLINT):
- **R5-RUSTLINT-F1, F2**, lint groups · clippy/lints.html · confirmed · "The `clippy::correctness` group is the only lint group in Clippy whose lints are deny-by-default"; "It is **not** recommended to enable the whole group"; "contains lints which are buggy or need more work."
- **R5-RUSTLINT-F7, F8, F15**, banned methods and types, lint versions · raw clippy disallowed_items.rs · confirmed · DISALLOWED_FIELDS "1.95.0", DISALLOWED_METHODS "1.49.0", DISALLOWED_TYPES "1.55.0"; `{ path = "std::time::Instant::now" },`
- **R5-RUSTLINT-F9**, `iter_over_hash_type` covers `for` loops only · raw iter_over_hash_type.rs · confirmed · "...(i.e., `HashSet` and `HashMap`) in for loops."; version "1.76.0"; `if let Some(for_loop) = ForLoop::hir(expr)`
- **R5-RUSTLINT-F11**, tests may unwrap if allowed · clippy/lint_configuration.html · confirmed · "Whether `unwrap` should be allowed in test functions or `#[cfg(test)]`", Default `false` (the same for expect and indexing_slicing)
- **R5-RUSTLINT-F13** (and R5-CI-F10), overflow and incremental defaults · cargo/reference/profiles.html · confirmed · `overflow-checks = true` (dev) and `false` (release); "Profile settings can be overridden for specific packages and build-time crates."; "Overrides cannot specify the `panic`, `lto`, or `rpath` settings."; `incremental = true` / `false`
- **R5-RUSTLINT-F14**, rustfmt options · raw rustfmt Configurations.md · confirmed · style_edition and edition default "2015", stable "Yes"; max_width `100`; newline_style "Auto"; imports_granularity "No (tracking issue: #4991)"; group_imports "No (… #5083)"

Question 4, tests (R5-TEST):
- **R5-TEST-F2**, proptest defaults · docs.rs proptest Config · confirmed · "proptest 1.12.0"; "The default is 256, which can be overridden by setting the `PROPTEST_CASES` environment variable."; "Set by using the PROPTEST_RNG_SEED environment variable"
  - The part marked "inferred" is in the source: "If the environment variable is undefined, a random seed is generated"
- **R5-TEST-F3**, insta snapshots · docs.rs/insta · confirmed · "insta 1.49.0"; "`no` for CI environments or `new` otherwise"; "`no`: does not write to snapshot files at all; just runs tests"
- **R5-TEST-F5**, wasm-bindgen-test in browsers · rustwasm.github.io …/browsers.html · confirmed · "Use Safari for headless browser testing, and `safaridriver` as its WebDriver."; "Note that this will ignore any environment variable set."
- **R5-TEST-F6**, Vitest Browser Mode providers · vitest.dev/guide/browser · confirmed · "v5.0.5"; "webdriverio supports these browsers:" firefox, chrome, edge, safari
- **R5-TEST-F8**, Playwright's WebKit is not Safari · playwright.dev/docs/browsers · confirmed for the quoted parts · "Playwright doesn't work with the branded version of Safari since it relies on patches."
  - The sentence the file completes by inference actually ends "…for the closest-to-Safari experience you should run WebKit on mac". It points to WebKit on macOS, not to real Safari. This matters for critical-read item 1.
- **R5-TEST-F9**, Playwright flake settings · playwright.dev TestConfig · confirmed · failOnFlakyTests "Added in: v1.52", "Whether to exit with an error if any tests are marked as flaky. Useful on CI."; "By default failing tests are not retried."
- **R5-TEST-F12**, GameNetworkingSockets fault injection · raw steamnetworkingtypes.h · confirmed · "[global float, 0--100] Randomly discard N pct of packets instead of sending/recv"; "Delay all outbound/inbound packets by N ms"; "Simulated jitter/clumping."; "Globally duplicate some percentage of packets."; "Global UDP token bucket rate limits."; separate _Send and _Recv values
- **R5-TEST-F13**, quinn on a custom socket · docs.rs quinn Endpoint · confirmed · "quinn 0.11.12"; `new_with_abstract_socket(… socket: Arc<dyn AsyncUdpSocket>, …)`; "Construct an endpoint with arbitrary configuration and pre-constructed abstract socket"
- **R5-TEST-F14**, Fiedler on network testing · gafferongames.com · confirmed · "Sep 6, 2016"; "99% packet loss, 10 seconds of latency"; "Your soak test passes when it runs overnight and doesn't hang or assert."

Question 5, CI with Rust caching (R5-CI, the helper's part):
- **R5-CI-F1, F2, F3**, what rust-cache saves and keys on · github.com/Swatinem/rust-cache · confirmed · "./target (build artifacts of dependencies)"; "the workspace crates themselves are not cached since doing so is generally not effective."; "sets CARGO_INCREMENTAL=0"; key: "the rustc release / host / hash" and "a hash of the workspace's Cargo.lock and of the Cargo.toml of the workspace root and every workspace member"
  - Nit: F2 leaves out the Cargo.toml part of the cache key.
- **R5-CI-F5, F7**, cache limits · docs.github.com dependency-caching and the github.blog changelog of 2025-11-20 · confirmed · "By default, the limit is 10 GB per repository"; "not been accessed in over 7 days."; "up to 200 uploads per minute", "1500 downloads per minute"; changelog: "going beyond the 10 GB per repo limit requires a valid Pro, Team, or Enterprise account."
- **R5-CI-F9**, what sccache cannot cache · github.com/mozilla/sccache, Known Caveats · confirmed · "This includes `bin`, `dylib`, `cdylib`, and `proc-macro` crates."; "Incrementally compiled crates cannot be cached."
- **R5-CI-F13**, rustup 1.28 stops installing toolchains automatically · blog.rust-lang.org Rustup-1.28.0 · confirmed · "rustup will no longer automatically install the active toolchain if it is not installed."; "rustup show active-toolchain || rustup toolchain install"
- **R5-CI-F14**, latest wasm-bindgen · wasm-bindgen releases · confirmed · "0.2.129", "released this 25 Sep 07:13"; "Show all 17 assets"
- **R5-CI-F15**, install-action · github.com/taiki-e/install-action · confirmed · "this action will also verify SHA256 checksums for downloaded files"; "none: disable all fallback options."
- **R5-CI-F17**, hosted runners · docs.github.com github-hosted-runners · confirmed · "free and unlimited on public repositories."; ubuntu-24.04 public "4" | "16 GB" | "14 GB"; GPU only under larger runners
- **R5-CI-F18**, Playwright in CI · playwright.dev/docs/ci · confirmed · "Caching browser binaries is not recommended…"; "setting workers to '1' in CI"; `mcr.microsoft.com/playwright:v1.64.1-noble`
- **R5-CI-F20**, skipped jobs and required checks · docs.github.com troubleshooting-required-status-checks · confirmed · "Associated checks stay in a "Pending" state and block merging."; a job skipped by a condition "reports "Success""; a job that depends on a failed job "is skipped and may not block merging."
- **R5-CI-F22**, workflow lockfiles · github.blog changelog of 2026-10-14 · confirmed (date, plans, path) · "GitHub Enterprise Cloud with data residency, Team, and Free plans."; ".github/workflows/actions.lock"

Question 6, preview links per PR (R5-PREV, the helper's part):
- **R5-PREV-F1 to F4**, fork PRs and trigger events · docs.github.com events-that-trigger-workflows · confirmed · "secrets are not passed to the runner when a workflow is triggered from a forked repository."; "Avoid using this event if you need to build or run code from the pull request."; workflow_run "is able to access secrets and write tokens, even if the previous workflow was not."; default activity types `opened`, `synchronize`, `reopened`
- **R5-PREV-F6**, environment reviewers · docs.github.com deployments-and-environments · confirmed · "If you are on a GitHub Free, GitHub Pro, or GitHub Team plan, required reviewers are only available for public repositories."
- **R5-PREV-F12, F13**, Pages preview deployments · developers.cloudflare.com pages/preview-deployments (updated Oct 12, 2026) · confirmed · "By default, these deployment URLs are public."; "…includes the X-Robots-Tag: noindex HTTP response header."; "The latest deployment for a branch cannot be deleted."
- **R5-PREV-F14**, Workers previews · developers.cloudflare.com workers/previews (updated Sep 22, 2026) · confirmed · "Only the 1000 most recently deployed aliases are retained."; "must not exceed 63 characters"; "If Version URLs are enabled, the URL is public and available after version creation."
- **R5-PREV-F18**, Pages `_headers` file · developers.cloudflare.com pages/headers (updated Aug 25, 2026) · confirmed · "You may define up to 100 header rules."; "Each line in the `_headers` file has a 2,000 character limit."; `https://:version.:project.pages.dev/*`
- **R5-PREV-F20**, Wasm MIME type · MDN instantiateStreaming (modified Sep 22, 2026) · confirmed · "If the response has an incorrect MIME type (instead of `application/wasm`), the promise rejects with a TypeError."

Question 7, the gate v2 (R5-GATE):
- **R5-GATE-F1**, cargo-mutants · github.com/sourcefrog/cargo-mutants · confirmed · "Inject bugs and see if your tests catch them!"
- **R5-GATE-F2**, Vite size limit only warns · vite.dev/config/build-options · confirmed · "Limit for chunk size warnings (in kB)." Default "500"; reportCompressedSize Default "true"
- **R5-GATE-F4**, cargo-deny defaults · bans/cfg.html · confirmed · `multiple-versions` warn (default) "Prints a warning for each crate with duplicates, but does not fail the check."; the same for `wildcards`

R5-GATE-REC2 names no R5 finding, so I checked its legacy basis in the repo instead (not counted in N). It is all confirmed:
- tools/gate.mjs:71-89 picks the newer merge-base.
- :37 exits with 2.
- :194 holds `ORDER` (the owner's notes say :202; the file's :194 is right).
- :213 sets exit codes 0, 1 and 2.
- :372 runs "[5a] smoke (slowest, so last)".
- tools/README.md:19 gives "About 20 s … 2.5 s".
- .github/workflows/gate.yml:67-72 reads the tool version from the lockfile.

**Counts:** 60 confirmed, 0 wrong, 0 unsupported.

**Wrong or unsupported basis claims per recommendation:** none, for all 36 recommendations. Every recommendation had at least one basis claim checked, except R5-GATE-REC2, which lists no R5 finding.

**Critical read, ranked**
1. **Real Safari does not follow from F8.** R5-TEST-REC1's Safari row says "the e2e suite on macOS with real Safari", and R5-TEST-REC2 says "real Safari joins on macOS", both citing F8. But the e2e suite is Playwright Test, and F8 says "Playwright doesn't work with the branded version of Safari"; its macOS advice is to run Playwright's WebKit there. Only F5 (wasm-bindgen-test with safaridriver) and F6 (Vitest with WebdriverIO) reach real Safari, and neither recommendation names a runner for an e2e suite in real Safari. R5-XQ-OQ1 inherits the same framing.
2. **The single required check can be skipped.** R5-CI-REC5 makes "one final job … the only required check" but never says that job needs `if: always()` and must fail when any job it depends on did not succeed. Its own basis, F20, says a job that depends on a failed job "is skipped and may not block merging", so as written a failing PR could merge. None of the spike marks tests a failing job. This is in the helper's part.
3. **"At warn" means "fails the gate" here.** R5-RUSTLINT-REC3 says "The sim itself keeps them [unwrap_used, expect_used, indexing_slicing] at warn". Under REC1's `-D warnings`, warn fails the gate, so every index read in `sim` would fail. These are also restriction lints, not pedantic ones as the sentence implies.
4. **R5-GATE-REC2's case against option B (`cargo xtask`) applies to its own option A.** It rejects B because every check would need the Rust toolchain, yet A's G0 checks the Rust toolchain on every run, and `--quick` runs G0–G5, which include cargo fmt and clippy. A TypeScript-only run without Rust is not actually provided.
5. **§6.2 is mis-traced.** R5-LAYOUT-REC4, LAYOUT-REC5, RUSTLINT-REC2, RUSTLINT-REC4 and TEST-REC2 cite §6.2 for "identical sim on client and server". VISION_V2 §6.2 is "A true 3D simulation" (floors, bots moving between levels, aiming at height). R1-ECS-REC2 traces the same determinism need to §4 and §6.6/§6.8. No feature lacks a need; the citation is wrong.
6. **Previews would not match production.** R5-PREV-REC4 isolates every preview (COOP and COEP headers), while production stays non-isolated (R1-BND-REC5 only "allow[s] it in development and previews for profiling"). Mustafa's Brave check would then run a header set production does not have. R5-XQ-OQ3 raises this only for local development.
7. **Missing options a reader would expect:**
   - cargo-nextest (a process per test; it bears on TEST-REC6's shared-state rule and F1). Its own site is off the allow list, but GitHub and docs.rs are on it.
   - Code coverage (cargo-llvm-cov, Vitest coverage). LAYOUT-REC6 even ignores `coverage/`.
   - A benchmark layer in TEST-REC1's table, although RUSTLINT-REC4 and TL15 rely on a tick bench.
8. **"Rests on:" lists miss what the reasons or bodies use:**
   - LAYOUT-REC1: R1-BND-F2.
   - LAYOUT-REC2: R1-DATA-REC2, R2-TRN-REC4 and R5-TEST-F4.
   - TEST-REC1: F7 (its reason says "F4–F9").
   - TEST-REC3: R1-DET-OQ3, plus CD4, CD6 and CD10–CD14.
   - TEST-REC4: R2-TRN-OQ8.
   - PREV-REC2: its GitHub Pages, Netlify-headers and Vercel claims rest on an inference and "R6's notes", not on any R5 finding.
   - GATE-REC2: names no R5 finding at all.
9. **Wave 1 drift.** R5-TEST-REC4's spike says app RTT "agrees with the profile within ±5 ms at P2 (R2-TRN-REC4 mark 2's form)". R2's mark 2 compares against quinn's server-side `rtt()` at 100 ms injected RTT, without P2's ±20 ms jitter and 2 % loss. R2's profiles themselves match r2-netcode.md:1983-1984 exactly, and the other R1 ids are consistent.
10. **Nits:**
    - R5-TSLINT-F22's language list is not quoted.
    - The "F2" and "F4" in GATE-REC1's table are ambiguous (GATE-F4 or LAYOUT-F4).
    - TEST-REC6's seed is both "per run from the commit" and "a fixed seed per test file".
    - The 200 KB mutation for check G7 depends on a budget that is not set yet.
    - PREV-REC2's "Cloudflare Pages meets every need" overstates the cleanup point, given R5-PREV-OQ2.
    - ESLint 10 is now current, so the formatting-rule removal window has opened; the file does not say whether the rules are gone.
    - Two stale draft cross-references remain: file:758 "section 1" and file:1854 "(part 8)".
    - Every pass-mark number is marked "proposed" or sourced; none is unlabelled.

**The helper's part (R5-CI and R5-PREV):** every line of /home/user/Shamble-Frontlines/.checkpoints/v2-research/infra-engineer-r5-ci-draft.md that is not a heading appears verbatim in the file. The only differences are the dropped draft title, the author and status paragraph, and the section headings. Ids and words match.

On "research only": nothing changes today's gate, tools or CI. Two passages could be misread:
- R5-GATE-REC2 calls the v2 gate `node tools/gate.mjs` (v2 edition), the same path as today's gate, which it says keeps running for legacy, while R5-GATE-OQ3 leaves the location open.
- R5-LAYOUT-REC6 says `tools/` "moves with them or learns the new path". That is phase 1 plan input touching frozen legacy tools.

**The label:** M = 115 (every finding except R5-CI-F8 and R5-PREV-F16), N = 60, so the status label becomes "spot-checked (60 of 115 key claims)".

**Only nits?** No. The source checks are clean, but critical-read items 1 and 2 are substantive, and items 3 to 5 are wording fixes beyond nits. A short re-check of only the fixed passages would be enough; whether that counts as the plan's second pass is the plan's call.

Not verified:
- Parts of findings I did not re-open: LAYOUT-F3's resolver page, CI-F4's GitHub-docs part, CI-F16, F19, F21, F23 and F24, the PREV-F3 quotes from the secure-use page, TOOLS.md, and the 1.28.2 post.
- R5-CI-F8 and R5-PREV-F16 are not in any "Rests on:" list, so they were not checked.
