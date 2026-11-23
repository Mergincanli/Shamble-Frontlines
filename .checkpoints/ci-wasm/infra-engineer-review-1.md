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
