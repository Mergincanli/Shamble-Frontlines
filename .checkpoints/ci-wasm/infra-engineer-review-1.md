# infra-engineer: review round 1 of PR #64 (CI-WASM), final diff

Head f54bd1c (code and CI state of ce5a7e78a7048f966460fdd346adf19250e009a3). Diff: `git diff origin/main HEAD -- .github .gitignore packages`.
Scratch: `.gate/ci-wasm/infra-review-1/`. I review; I fix nothing. Memory: none on disk (no agent-memory folder exists).

## Plan (blocks, in order; a line after each)
- B0. Read the plan (revision 4, sections 5, 7, 9, 12), the card's "Current step", my own notes (ci-plan, opt-path) for what I drafted.
- B1. test-engineer's part FIRST (plan section 12): playwright.config.ts, tests/skeleton/*.ts, node-names.d.ts, build-checks*.mjs, build.mjs, its followups notes. Task 4.
- B2. The workflow file whole (final vs main): shell, triggers, concurrency, timeout, cache-mode, image drift. Task 3.
- B3. The six clean job logs end to end (push/pr 1379740..., takeback-browser x2, takeback-gh x2): warnings, deprecations, step times, never-run list, Chrome version, the chrome_main_linux WARNING line, Wasm size and sha256 vs containers. Task 1.
- B4. stop-rule-check.py against plan section 9, point for point. Task 2.
- B5. THE TABLE: every rule sentence of the final workflow vs main's. Task 5.
- B6. Follow-ups for STATUS. Task 6.
- B7. Findings with severity, file:line, exact replacement text; verdict.

## Notes (a line after each block)
- B0 (read): plan.md rev 4 (s5 never-run list :138-141, s7 item 5 table :206-211, s9 stop rule :246-261, s12 :292-295);
  card "Current step" (:275-334: runs and ids); my notes ci-plan (Blocks 1-8, End) and opt-path (B1-B7). Memory: none.
- Note: HEAD moved to faae0b8 during this review (the session's docs commits e98b927, faae0b8: DECISIONS.md and
  decisions-3.mjs only); `git diff --stat f54bd1c HEAD -- .github .gitignore packages` is empty, so the reviewed text is
  the same. Merge-base with origin/main = origin/main = f5030a3.
- B1 (test-engineer's part, read first): diff of 9 files outside .github (+73 -17 with .gitignore). From the CI side:
  (a) playwright.config.ts:25-26 `workers: 1`, `timeout: 90_000`; the plan says `timeout: 60_000` (plan.md:87, :157;
  test-engineer-followups.md:49, :325); the commit 3cddb55 message and the config's comment say 90 s ("above report.ts's
  60 s wait"): the code is right (a 60 s test limit would still cut the 60 s wait), the plan's record is stale -> the
  report must say 90 s. 12 x 90 s = 18 min > the job's 15, but CI's `--global-timeout=300000` ends Playwright first.
  (b) report.ts (+9, `drawn()`) is changed but not in the plan's file table (plan.md:80-91) -> the report lists it.
  (c) `drawn()` (report.ts:47-49) and clock.spec.ts:12 have no own timeout: bounded by the 90 s test limit; fine.
  (d) Ports: serve.mjs on 127.0.0.1:4173 with reuseExistingServer false; inside `unshare --net` the loopback is new, so
  no other process can hold 4173 (the cut isolates the port too); isolation.spec.ts's Vite servers bind 127.0.0.1.
  (e) refusedBuildSettings: the test's allowed list (build-checks.test.mjs:219-222) is a hand copy of the cut's env -i
  names + SHAMBLE_WASM_BINDGEN; a later CI name that matches the pattern fails loudly at build.mjs:59-60 anyway.
  (f) Readings lines: one console.log per test before its first expect on those numbers; to be read in the logs (B3).
- B3 part 1 (push-137974047720.log read end to end, 932 lines; data only): image ubuntu-24.04 20261116.133.1, us-west-2,
  runner 2.338.2; "GITHUB_TOKEN Permissions: Contents: read, Metadata: read"; "Cache mode: none" (:27). Steps: checkout
  6.3 s, setup-node 5.7 s ("Found in cache" node 22.23.5, `package-manager-cache: true` echoed at :162, no cache line
  after it), rustup 10.8 s, pin 0.1 s, archive 0.25 s (:249-251), GitHub's check 5.0 s (gh --version alone 2.4 s:
  cold first exec), program 0.6 s (:310-320), fetch 3.7 s, the cut 110.6 s (20:01:40.12 to 20:03:30.75). Job 20:01:05
  to 20:03:31 = 2 min 26 s of 15. Inside the cut: tests 6.3 s compile; Wasm release build 5.09 s (:834); type checks
  7.0 s (:884-885); `chrome --version` 6.5 s (:885-887: cold first read of Chrome, same pattern as gh and node); the
  12 tests 20:02:27.6 to 20:03:30.7 = 63 s. Only warnings: git's defaultBranch hint (:74-86, checkout's own), Vite's
  500 kB chunk notice x2 (:840-844, :857-861, known since R6), Chrome's `[1123/200224.714432:WARNING:chrome/app/
  chrome_main_linux.cc:84] Read channel stable from /opt/google/chrome/CHROME_VERSION_EXTRA` (:886, on stderr before
  "Google Chrome 157.0.8103.78 " :887). No ##[warning], no deprecation, no error line. Client unit tests "18 tests"
  in build-checks.test.mjs, "49 passed" (:753, :760). Readings lines present for all 12 tests (:891-913).
  Wasm 28991 / gzip 11668 / brotli 9900 (:837), sha256 22dad2eb...018b (:871), one registry path under
  /home/runner/.cargo (:872). No post step of setup-node in the log (only checkout's "Post job cleanup", :916-931).
- Source S1 (read 2026-11-23, chromium.googlesource.com/chromium/src/+/main/chrome/app/chrome_main_linux.cc, through
  the fetch model; main branch, not the 157 tag; data only): `PossiblyDetermineFallbackChromeChannel(...)` does
  `LOG(WARNING) << "Read channel " << channel << " from " << path;`, compiled only with GOOGLE_CHROME_BRANDING, when
  the CHROME_VERSION_EXTRA variable is not set ("we may have been launched via the "chrome" binary directly, instead
  of the "google-chrome" wrapper script"). So the line is expected: the job runs /opt/google/chrome/chrome itself and
  `env -i` leaves CHROME_VERSION_EXTRA out. Harmless; Chrome reads its channel from the file beside it. Sure: high
  for the meaning, medium-high that 157 has the same line (the log names :84 of that file). (raw.githubusercontent.com
  answered 429 first.)
- B3 done (all six clean logs; push read whole, the other five by a timestamp-free diff against their twin plus the
  extraction script `.gate/ci-wasm/infra-review-1/log-facts.py`, output `facts-clean.txt`): all six on image
  20261116.133.1 (us-west-2 x4... regions us-west-2 and us-east-1), runner 2.338.2, gh 2.104.0, Chrome 157.0.8103.78,
  backend "WebGL 2 fallback" (SwiftShader via ANGLE Vulkan), webgpuOffered true in all 12 readings. Job 134.8 to
  145.8 s (2.2 to 2.4 min of 15; the stop's 10). Inside the cut: 22-24 s to the builds, builds 8.1-8.5 s, boundary
  0.5 s, type checks 6.8-7.0 s, `chrome --version` 3.8-6.5 s, tests 60-65 s ("12 passed (1.0m)" or "(1.1m)"). Per
  test max 14.8 s (allocation; limit 90 s). No ##[warning], no deprecation, no retry/flaky, no orphan process; the
  only ##[error] lines are in the planted/refused runs. Margins on runners (6 runs): allocation frames 65-76 (floor
  20), blocks 244 (floor 200), last block 340-343 (<420), per frame 50-61 (25..170), per block 487-523 (430..610);
  hidden tab 42-51 ticks since showing against a bound of 57-60; clock start tick 3-14 (>=1); notices 130-542 ms (20 s).
  Wasm: 28991 raw / 11668 gzip / 9900 brotli, sha256 22dad2eb...018b in all 6 clean + 2 plant-browser runs (8 runners,
  2 regions, one image), one registry path /home/runner/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/
  wasm-bindgen-0.2.130/src/externref.rs. Container (my notes R6, 2026-11-19): 28987 / 11655 / 9955, sha256
  c4935aca...2c31, path under /root/.cargo. "/home/runner" is 7 characters longer than "/root", the file only 4 bytes:
  consistent with RR-F5 if 3 bytes of alignment padding after the string are absorbed (a string ending at 1 mod 4,
  next item 4-aligned), but NOT shown: no same-path build exists, this container has no registry, node_modules or
  target to build one (checked: /root/.cargo has bin/ and env only), and the job prints only the sha256 and the
  registry-path grep (no second Wasm, no other path). The PR run builds the merge commit f97ff78 and gives the same
  Wasm bytes (Rust unchanged by the merge).
- (B3 wording fix: regions were us-west-2 four times and us-east-1 twice.)
- B4 done (stop-rule-check.py against plan.md:246-261, point for point). Ran it on the six clean logs: ALL (17/17)
  each. Fair on: final address host; "size: 11860397"; the archive ": OK"; gh >= 2.93.0 (first version line); gh's
  success line naming 0.2.130; seven members (the tar -tvzf listing, exactly 7); WB_TOOL env line (any step's env
  block: WB_TOOL is job-level, so equivalent); the four install lines adjacent with `/`'s byte count not judged;
  "exists already" outside the cyan script echo (ESC bytes are in the logs: 115 cyan prefixes in the push log, so
  the exclusion works); "the network is cut"; Chrome's line; "Running 12 tests using 1 worker"; "12 passed"; no
  failed/flaky/retry; no ##[error]. GAPS (each shown on a mutant copy of the push log in
  `.gate/ci-wasm/infra-review-1/`): (G1) 'Tests 7 passed' (:25) is a search over the whole log, and packages/types'
  unit tests print "Tests  7 passed (7)" too (push log :816, inside the cut via `npm test --workspaces`) besides the
  boundary tests (:880): mutant-a (boundary "5 passed | 2 skipped (7)") still reads ALL. (G2) the rule's "the job
  takes more than 10 of its 15 minutes" is not checked at all: mutant-b (last line +10 min, 746 s) reads ALL. Minor:
  "the two size tables" is ">= 2 of the 4 tables"; the "then" order (members, env line, then the four lines) is not
  checked; plants (rule point 6) are read by hand (card :300-326), not by the script. The proposed copy
  `stop-rule-check-proposed.py` (G1: the boundary block between its echo and "== the type checks"; G2: first to
  last timestamp <= 600 s) reads ALL on the six clean logs (19/19; 135-146 s) and NOT ALL on both mutants. The
  evidence of the six runs holds (boundary line read directly at :877-880 in each; times 134.8-145.8 s).
- Source S2 (read 2026-11-23, github.blog/changelog/2026-09-10-control-github-actions-cache-access-with-cache-mode,
  through the fetch model; data only): values read, write, write-only, none; "`none` prevents all cache access";
  workflow or job level, "Job-level settings override workflow-level settings"; "The selected mode is enforced by
  the cache service and carries through reusable workflows". So the job's `cache-mode: none` (toolchain.yml:37) also
  stops setup-node's automatic package cache (echoed `package-manager-cache: true` at push log :162; off today anyway:
  the root package.json has no packageManager field). Sure: high (GitHub's own post; the post does not name setup-node).
- B5 done: THE TABLE below (built from a script listing of every comment, step name and echo/exit line of main's
  file 8b581adf...e016 and the final file 56d37fc6...9811; `git diff --numstat`: 97 added, 0 removed; every rule
  line of main's stands byte for byte in the final file; no rule sentence is changed, so none is looser).

### THE TABLE (for the report, section "The workflow text"; pasteable)
Main's `.github/workflows/toolchain.yml` (sha256 8b581adf...e016, 100 lines) against the final file (sha256
56d37fc6...9811, 197 lines). A rule sentence: a comment, step name, or echo/exit line that says what is required or
refused. "Unchanged" means byte for byte (only its line number moved). Nothing of main's is removed or reworded.

| # | Final line | Rule sentence (its words) | Main's line | Status | Stricter or looser |
|---|---|---|---|---|---|
| 1 | 1-3 | "the pinned Rust toolchain from rust-toolchain.toml, then the Rust and npm workspaces' builds and tests with the network cut off. One job, `toolchain`; the gate and smoke jobs stay in gate.yml." | 1-3 | unchanged | same |
| 2 | 4-6 | "The cut: the last step runs in its own network namespace (sudo unshare --net), with loopback up, back as the runner's user with no docker group and no_new_privs; a TCP probe to an address resolved before the cut must connect outside and fail inside (only curl exit 7 or 28 counts as cut)." | 4-6 | unchanged | same |
| 3 | 7-10 | "CI-WASM: before the cut, the wasm-bindgen tool's archive is fetched and checked against Mustafa's values and by GitHub's own check of the release; its one program is installed root-owned. Inside the cut the page is built with it; then the boundary tests, the type checks of the Worker, the page and the browser specs, and the browser tests run (in the runner image's Google Chrome: nothing is downloaded for them)." | none | new | stricter (adds checks; the one new download is his TOOL-SOURCE file, by CI-TEXT (a)) |
| 4 | 25 | "Only the explicit install below brings a toolchain: rustup turned automatic install back on by default in 1.28.1." | 21 | unchanged | same |
| 5 | 36 | "No cache in this job, restored or saved: code planted for a later step cannot reach another run through it." | 32 | unchanged | same |
| 6 | 39-41 | "The one wasm-bindgen tool the build may run: Mustafa's values of 2026-11-18 (TOOL-SOURCE (a)); any other file, version or checksum comes back to him. The first tool step compares them line for line with packages/client/scripts/wasm-bindgen-tool.mjs, the copy build.mjs checks the program against before each run." | none | new | stricter |
| 7 | 42 | "WB_TOOL and each folder above it are root-owned and closed to others, so no process inside the cut can swap it." | none | new | stricter |
| 8 | 59 | step "Install the pinned Rust toolchain from rust-toolchain.toml (before any cargo call)" | 43 | unchanged | same |
| 9 | 65 | "rust-toolchain.toml: no single exact x.y.z channel" (exit 1) | 49 | unchanged | same |
| 10 | 66 | "RUSTUP_TOOLCHAIN is set: it would beat rust-toolchain.toml" (exit 1) | 50 | unchanged | same |
| 11 | 68 | "the active toolchain does not come from rust-toolchain.toml" (exit 1) | 52 | unchanged | same |
| 12 | 70 | "rustc equals the pin $pin" / "rustc is not the pin $pin" (exit 1) | 54 | unchanged | same |
| 13 | 73 | "wasm-bindgen-tool.mjs is read as text, never imported: no code of the repository runs with the network." | none | new | stricter |
| 14 | 74 | step "The wasm-bindgen tool's archive (https only, then its size and sha256, before any tar call)" | none | new | stricter |
| 15 | 81 | "packages/client/scripts/wasm-bindgen-tool.mjs lacks this line of the workflow: $line" (exit 1) | none | new | stricter |
| 16 | 88 | "the archive is $bytes bytes, not $WB_ARCHIVE_BYTES" (exit 1); then `sha256sum -c` (:89, no words of its own) | none | new | stricter |
| 17 | 91-92 | "Before the tool's first run (Mustafa's CI-WASM (a)). The only step with the token; gh 2.93.0 keeps it off the TUF mirrors (CVE-2026-48501)." | none | new | stricter (see finding F4 on "the only step") |
| 18 | 93 | step "GitHub's own check of the archive (gh release verify-asset)" | none | new | stricter |
| 19 | 99 | "gh printed no version of the form x.y.z: $gh_version" (exit 1) | none | new | stricter |
| 20 | 100 | "gh is older than 2.93.0" (exit 1) | none | new | stricter |
| 21 | 103 | step "The tool's one program (the member list, then only wasm-bindgen taken out, installed root-owned, checked)" | none | new | stricter |
| 22 | 109 | "the archive's member list is not the expected one" (exit 1) | none | new | stricter |
| 23 | 111 | "a member is not a plain file or folder, has a name with a space, or is above 64 MiB" (exit 1) | none | new | stricter |
| 24 | 114 | "more than the one program came out" (exit 1) | none | new | stricter |
| 25 | 115 | "WB_TOOL is not an absolute path: $WB_TOOL" (exit 1) | none | new | stricter |
| 26 | 117 | "${WB_TOOL%/*} exists already or could not be made: the tool needs a fresh folder made here as root" (exit 1) | none | new | stricter |
| 27 | 119 | "$WB_TOOL is not a plain file" (exit 1) | none | new | stricter |
| 28 | 123 | "$WB_TOOL or a folder above it is missing or a link, is not owned by root, or others than root may write in it" (exit 1) | none | new | stricter |
| 29 | 124 | "the program is not $WB_PROGRAM_BYTES bytes" (exit 1); then `sha256sum -c` (:125) | none | new | stricter |
| 30 | 127 | step "Fetch every dependency (the last step with the network)" | 57 | unchanged | same (still true: the three new network steps come before it) |
| 31 | 132 | step "Build and test with the network cut off" | 62 | unchanged | same |
| 32 | 136 | "control failed: github.com has no IPv4 address before the cut" (exit 1) | 66 | unchanged | same |
| 33 | 138 | "control failed: no connection before the cut, so the probe below would prove nothing" (exit 1) | 68 | unchanged | same |
| 34 | 148 | "getent: no answer (printed, not judged)" | 77 | unchanged | same |
| 35 | 151-153 | "the network is still reachable inside the cut" (exit 1); "the network is cut: no TCP connection to $PROBE_IP:443 (curl exit $rc)"; "probe inconclusive: curl exit $rc" (exit 1) | 80-82 | unchanged | same |
| 36 | 158 | "writing the generated types again changed:" (exit 1) | 87 | unchanged | same |
| 37 | 167-168 | `[ -n "$shipped" ]`, `[ -n "$tests" ]` (exit on an empty list; no words) | 96-97 | unchanged | same |
| 38 | 171 | "== the page as players get it, then the development page the browser tests need" (both builds; build.mjs's own refusals apply) | none | new | stricter |
| 39 | 174, 176 | "== the Wasm built here, and the builder paths it carries (RR-F5: printed, not judged)"; "no registry path in the Wasm" | none | new | neither: printed, not judged (by design) |
| 40 | 177 | "== the boundary tests, on the built Wasm" | none | new | stricter |
| 41 | 179 | "== the type checks: the shipped files of the Worker and the page with the lib check, then with their tests; then the browser specs" | none | new | stricter |
| 42 | 186 | "a file list is empty" (exit 1) | none | new | stricter |
| 43 | 192 | "== the browser tests, one at a time, never retried, stopped by Playwright after 5 minutes (so before the job)" | none | new | stricter |

Rule-bearing settings (no sentence of their own):
| Setting | Main | Final | Status |
|---|---|---|---|
| `on: push`, `pull_request` | :9-11 | :13-15 | unchanged |
| `permissions: contents: read` (log: "Contents: read, Metadata: read") | :13-14 | :17-18 | unchanged |
| `concurrency` group `toolchain-${{ github.workflow }}-${{ github.ref }}`, `cancel-in-progress: true` | :16-18 | :20-22 | unchanged |
| `RUSTUP_AUTO_INSTALL: "0"`; `defaults.run.shell: bash`; `runs-on: ubuntu-24.04` | :20-30 | :24-34 | unchanged |
| `timeout-minutes: 15` | :31 | :35 | unchanged (the job now takes 2.2-2.4 min of it) |
| `cache-mode: none` | :33 | :37 | unchanged |
| actions by full SHA (checkout v7.0.1, setup-node v7.0.0); `persist-credentials: false`; no new `uses:` | :35-41 | :51-57 | unchanged |
| job `env:` WB_* (seven values, his of 2026-11-18, and WB_TOOL) | none | :38-49 | new |
| `GH_TOKEN: ${{ github.token }}` on one run step | none | :94-95 | new exposure, by his CI-WASM (a): "It runs GitHub's own check on the tool's file before the tool's first run there" |
| `sudo -n` for mkdir, chown, install before the cut | none (sudo only for unshare, :70) | :116-118 | new, root-owned install by plan s5 item 4 (his CI-TEXT (a), FIX-WRITE (a)) |
| the cut's `env -i` list | :72-74 | :142-145 | one name more: `SHAMBLE_WASM_BINDGEN="$WB_TOOL"` (plan s5 item 6; build.mjs refuses a relative path) |
| Playwright flags `--workers=1 --retries=0 --reporter=list --global-timeout=300000` | none | :195-196 | new |
No looser rule: nothing needs new words of his. The three new exposures (the token in one run step, sudo for the
install, one more name in the cut's environment) are each inside the text he said yes to (CI-TEXT (a) on
240c7db7...2673, FIX-WRITE (a) on b4081e48...03ec).
- B2 done (the workflow file whole, final 56d37fc6...9811 against main 8b581adf...e016; +97 -0; gate.yml unchanged).
  Shell: every step runs `/usr/bin/bash --noprofile --norc -e -o pipefail {0}` (push log :181, :238, :258, :299, :420);
  the new lines fail closed under it: curl|sed and tar|sort/awk pipelines carry curl's/tar's failure (pipefail);
  `gh --version | awk` reads to EOF (no SIGPIPE); the judge's `$(find ...|wc -l)` lowers the count on a missing path
  (no reliance on -e inside a substitution); the inner script has no apostrophe and parses as one argument. awk's
  juxtaposed rules (:110) are accepted by mawk 1.3.4 (tried here) and ran on the image. Triggers unchanged (push of
  any branch or tag, pull_request default types). Concurrency unchanged: push runs group on refs/heads/<branch>, PR
  runs on refs/pull/64/merge, so the two never cancel each other; a new push cancels both in-progress runs of the
  previous head; a [skip ci] head creates no run and cancels nothing. Plants: their runs were not cancelled (each
  ends in its own step's "##[error]Process completed with exit code 1.", the take-back pushed after). Merge: the
  final head must not be a [skip ci] commit (CHECKPOINTS.md:214 already says so: the last commit runs CI); after the
  merge main's push run is cancelled by a second merge within about 2.5 min now (CANCEL-ON-MAIN, known). Limit:
  2.2-2.4 min of 15; Playwright's own 5 min bound ends first on a browser hang; other hangs end at 15 min.
  cache-mode none: enforced by the cache service (S2); setup-node's `package-manager-cache: true` default (log :162)
  stays off (no packageManager field) and would be refused anyway. Image drift: Chrome path hardcoded twice
  (:193-194), version moved .50 -> .78 within 5 days (image 20261111.370.1 -> 20261116.133.1), V8-sensitive bounds
  (per block 487-523 against 610); gh floor passes newer versions, refuses an unreadable one (fail closed); Node 22
  from the toolcache until it leaves the image (NODE-26: EOL 2027-04-30); `/` 755 root:root and passwordless sudo
  documented (S3.9 of my ci-plan notes); a new redirect hop beyond 2 fails curl visibly. Cost: 0 per run (public
  repository, standard runner: "free and unlimited on public repositories", read 2026-11-19; the runs report
  hardwareConcurrency 4, the public runner's size); about 4.7 runner-minutes per push (two events). Provider down:
  GitHub releases, GitHub's attestation API, crates.io, npm or rustup's host fail their own step before the cut,
  visibly; Google is not contacted. Exposes: public logs only (signed query cut by sed); Chrome runs with
  Playwright's --no-sandbox on our own page on 127.0.0.1 inside the cut. Rollback: revert; nothing outside the
  repository exists (no cache, artifact, secret or setting). Wording: :91 "The only step with the token" is not so
  in the letter: checkout and setup-node get the job token as their default `token` input (push log :38 "token: ***",
  :161 "token: ***"); it is the only run step with it (finding F3). PATH inside the cut is the runner's
  (`env -i PATH="$PATH"`, :142), whose first folders are writable by the cut's own uid on this image (S1-S3 of my
  opt-path notes): a build script can shadow node, npx, git or sha256sum for every later line; the root-owned tool
  closes only its own file (follow-up CUT-PATH).
- Plant logs re-read for B4 point 6: plant-browser x2 end at the cut step with the toEqual difference
  ('-   "drawCalls": 4,' / '+   "drawCalls": 3,' twice), "2 failed", no retry line, then "##[error]Process completed
  with exit code 1." (:972-976, :984-988); plant-gh x2 end at "Run gh --version" with gh's "attestation for 0.2.130
  does not contain subject sha256:27c24e42...6e10" and exit 1 (:274-275, :286-287), no later step group. None was
  cancelled. Known codes found in STATUS/archive: REQUIRED-CHECK, CANCEL-ON-MAIN (my IE-5), NODE-26, TWO-PLAYWRIGHTS,
  WASM-ARTIFACT, CI-CUT part 2 (STATUS.md:22, :95; STATUS_ARCHIVE.md:703-768, :939-944, :974-976).
- B6 done (follow-ups for STATUS, one line each, with why not here):
  1. REQUIRED-CHECK (known, for 2.3): `toolchain` now holds the build players get and the 12 browser tests, still not
     a required check, so each merge reads it by hand. Not here: branch protection is a repository setting (his), and
     plan s15 leaves out a required-check change.
  2. IMAGE-CHROME (new): the tests run the image's Chrome, which moved 157.0.8103.50 -> .78 in five days (images
     20261111.370.1 -> 20261116.133.1); 158 is Playwright 1.65.0's own; 159 may outrun it; V8 moves the allocation
     windows (per block 487-523 on runners against the 610 ceiling). Watch the printed line; a red is a Playwright
     bump (a dependency, its security answer) or CHROME-SOURCE (c). Not here: his CHROME-SOURCE (a) accepted it.
  3. CUT-PATH (new, with CI-CUT part 2; security, Medium, pre-existing class): inside the cut PATH is the runner's
     (toolchain.yml:142), and on this image its early folders (setup-node's bin in /opt/hostedtoolcache, ~/.cargo/bin,
     ~/.local/bin, /usr/local/bin: `chmod -R 777 /opt` and `/usr/local/bin` by the image's scripts) and /opt/google/
     chrome are writable by the cut's own uid, so a build script can shadow node, npx, git, sha256sum or Chrome for
     every later line; the root-owned tool closes only its own file. Not here: the fix (root-owned copies of node and
     cargo like the tool, a PATH of root-owned folders only, and no JavaScript action with a post step) is new CI
     text for the gate v2, beside CI-CUT part 2; first a run that prints `stat` of each PATH folder and of Chrome.
  4. FIREFOX-TESTS (c) (his answer): its own pull request next: Playwright's patched Firefox 159.0 (about 187 MB, no
     published checksum), 11 of the 12 tests; its download and exact text come to him first. Not here: his (c).
  5. CANCEL-ON-MAIN (known, for 2.3): weightier now that the job takes 2.2-2.4 min: two merges inside that window
     leave the first without a toolchain result on main. Not here: 2.3 owns the concurrency rule.
  6. RR-F5 (known, change-set 3 with rust-reviewer): after the remap CI can judge the Wasm's sha256 against a
     container build of one commit; until then 28991 against 28987 bytes fits the path but is not shown to be only it.
  7. TOKEN-WORDS (new, Low): toolchain.yml:91 "The only step with the token" -> "The only run step with the token" at
     the next CI edit. Not here: CI text needs his yes, and a Low triggers no push.
  8. NODE-26 (known): this job's `node-version: 22` moves with it (Node 22's end of life 2027-04-30).
  For this PR's own docs pass (docs-keeper), not follow-ups: TWO-PLAYWRIGHTS' clause "no real browser has run inside
  the cut's privilege drop" is now false (Chrome 157.0.8103.78 ran 12 tests there, six clean runs); WASM-ARTIFACT:
  CI now builds the release Wasm through build.mjs (the cut's own wasm32 line stays dev-profile).
- Source S3 (read 2026-11-23, github.com/actions/runner-images/blob/main/images/ubuntu/templates/build.ubuntu-24_04.pkr.hcl,
  through the fetch model; main branch, not the image's tag; data only): provisioner order lists install-google-chrome.sh
  (34th), install-nodejs.sh (44th), install-rust.sh (51st), Install-Toolset.ps1 (64th), then configure-system.sh (73rd).
  With configure-system.sh:12-15 `chmod -R 777 /opt` (my opt-path S1, tag ubuntu24/20261116.376), /opt/google/chrome and
  the Node toolcache are opened to all after their install. Sure: medium-high (main's template; modes of those files
  were never printed by a run).
- Correction to follow-up 3 (CUT-PATH): severity Low, not Medium. Inside the job a hostile dependency in the cut can
  already fake the verdict by other means (edit the specs, the built page, rustc under ~/.rustup, owned by the
  runner's uid); PATH shadowing adds no reach beyond the job (the cache is none; the post-step reach is CI-CUT part 2).
  What it changes: the root-owned tool's guarantee is narrow (its own file), and the report should not read wider.
- B7 done. VERDICT: APPROVE (the diff needs no change). Findings, ranked:
  F1 Medium: `.gate/ci-wasm/stop-rule-check.py:25` (Tests 7 passed is also the types package's line) and no check of
     the 10-minute stop; fix before the final head's runs are read (replacement text = the tested lines in
     `.gate/ci-wasm/infra-review-1/stop-rule-check-proposed.py`). Evidence of the six runs holds (read directly).
  F2 Low: record drift: plan.md:87, :157 say `timeout: 60_000`, the code is `90_000` (playwright.config.ts:26, commit
     3cddb55); report.ts (+9, drawn()) is missing from plan.md:80-91 and :301-308 -> the report's file list says so.
  F3 Low: toolchain.yml:91 "The only step with the token" and plan.md:110 "the only step that holds the job's token":
     checkout and setup-node get it as their default input (push log :38, :161) -> "the only run step"; the comment at
     the next CI edit (TOKEN-WORDS), the report now.
  F4 Low: the card's "(28,987 bytes in the containers: RR-F5)" (ci-wasm.md:296-297) reads as shown; the report says
     "consistent with RR-F5, not shown" with the numbers.
  Not verified: a same-path Wasm build (no registry, node_modules or target here); the runner's modes of PATH folders
  and of /opt/google/chrome (never printed); Chromium's line on the 157 branch (read on main); run attempts (API not
  queried); GitHub's own job times for the two take-back-browser runs (log stamps used); the PR's current head has no
  run (faae0b8 is [skip ci]); the docs.
DONE
