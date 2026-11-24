# Plan: the small CI pull request after change-set 3a (CI Builder), revision 1

Branch: `claude/keen-tesla-v807cf` (this cloud chat may push only there; it stands in for `cs/ci-after-3a`). Work in
the worktree `.gate/ci3` (local branch `cs/ci-after-3a`, from main 28254a73ab226379373e380eeee64336e58ef916). Risk tier
2 (a `.github/` file). Change type: CI (tier-2 config) plus test configuration.
His opening block, saved by script: `mustafa-2026-11-24-opening-block.md` (3857 bytes, sha256 93ca1c04...201fa).

## 1. Goal (his four parts, all on main)
a. E1: the release test line runs every crate (cs3-plan.md section 6 at 6491b839).
b. Firefox in CI (FIREFOX-TESTS (b), DECISIONS 2026-11-23, 21:24:59 UTC; with 3A-OTHER-BROWSERS: the vectors in Firefox).
c. CI runs both vector generators' `--check` (3A-GEN-CHECK, STATUS_ARCHIVE:1371).
d. The test-only WebAssembly module gets a build step of its own (3A-RULES-STEP, STATUS_ARCHIVE:1375; engine-architect's
   round 1 F5: "a separate rules build step when CI's lines are next edited (E1)").

## 2. Files touched (the fence) and exact changes
1. `.github/workflows/toolchain.yml` (tier 2; written only after his yes on the exact text, mode read right before):
   - a. line 159 `cargo test --release -p sim -p protocol --locked` becomes
     `cargo test --release -p sim -p protocol -p room -p client-endpoint -p sim-wasm -p devtools --locked`.
   - d. a new line right after line 160 (the five-crate wasm32 build), inside the cut:
     `cargo build --locked --target wasm32-unknown-unknown -p devtools --lib` (dev profile, like line 160; its own line,
     so a native-only dependency of `devtools` that breaks wasm32 fails here by name). Reading of "a build step of its
     own": a line of its own inside the cut step, not a separate workflow step (a separate step would sit outside the
     cut or need a second cut). Shown to him as such. What it proves: `devtools` compiles for wasm32 with plain cargo
     flags. What it does not prove: the module the tests load, which `build.mjs` still builds with its own target
     folder (`.gate/target-rules`), the registry remap and both profiles. No plant: a plant would mean editing `crates/`.
   - c. two lines plus an echo, inside the cut (Python's standard library only; reads files, writes nothing; `-I -B`):
     `echo "== the two vector generators' checks: the committed files equal what each generator makes (nothing written)"`,
     `python3 --version`,
     `python3 -I -B crates/devtools/vectors/make_chacha8_vectors.py --check`,
     `python3 -I -B crates/devtools/vectors/make_f32_vectors.py --check`.
     Placed after the release tests, before the wasm32 builds.
   - b. the Firefox run, after the Chromium run, same shape (one worker, no retry, Playwright's 5-minute stop), the
     browser's version printed and a missing browser failing by name; the exact lines depend on his FIREFOX-SOURCE
     answer (section 3). Route (b) also adds a fetch, check and install step before the cut, like wasm-bindgen's.
   - Comments: the header's "(in the runner image's Google Chrome: nothing is downloaded for them)" and the line
     before the browser run are updated to name Firefox and its source. No other comment changes (TOKEN-WORDS and
     TE-CPU stay out and are named still open).
2. `packages/e2e/playwright.config.ts`: a third project `firefox`: `testDir: "./tests"`, `testIgnore: ["node/**",
   "skeleton/allocation.spec.ts"]` (that spec uses Chromium's own heap profiler through CDP:
   `allocation.spec.ts:31-44`), `workers: 1`, `timeout: 90_000`, `use: { browserName: "firefox", baseURL: ADDRESS,
   launchOptions: ... }` from a new variable `SHAMBLE_FIREFOX` (route (a) also sets `channel: "moz-firefox"`). The
   `node` and `chromium` projects are not changed. The header comment names the new project. Anything else a Firefox
   failure would need (a spec, `render-counts.json`) is outside the fence: I ask him, I do not fix.
3. Notes under `.checkpoints/ci-after-3a/` (this plan, the card, his words by script, trial logs, `pr-title.txt`,
   `pr-body.md`), and the docs pass (section 7).
Nothing under `crates/`, nothing in a package's source, nothing in `spikes/`, no kit or gate file.

## 3. The Firefox source: his decision FIREFOX-SOURCE (ask 1)
Facts (read 2026-11-24):
- Playwright 1.65.0 (the locked version) drives its own patched Firefox build by default; that build is not in the
  runner image. It also has a mode for the ordinary Mozilla Firefox over WebDriver BiDi: `channel: "moz-firefox"`
  selects `_bidiFirefox.launch` (`node_modules/playwright-core/lib/coreBundle.js:47743-47746`; channel table :34982-34986,
  default Linux path `/snap/bin/firefox`, so CI passes `executablePath`). Playwright calls the BiDi route experimental.
- The runner image `ubuntu-24.04` (readme of image 20261116.376.1, which lists Google Chrome 157.0.8103.78, the version
  run 45503161333 printed on main) carries Mozilla Firefox 160.0 from the Mozilla team's PPA (install script:
  `apt-get install ... firefox`, prefs in `/usr/lib/firefox/`): the same major version as his Windows Firefox 160. The
  PPA's index already offers 161.0 (`firefox_161.0+build1-0ubuntu0.24.04.1~mt1_amd64.deb`, 90,437,572 bytes, sha256
  bd304b0b...cf1ed1), so it moves with the image, as Chrome does (IMAGE-CHROME).
- Playwright's own Firefox for 1.65.0: Firefox 159.0, build 1572 (`playwright-core/browsers.json`), address
  `https://cdn.playwright.dev/dbazure/download/playwright/builds/firefox/1572/firefox-ubuntu-24.04.zip`, which answers
  307 to `https://playwright.download.prss.microsoft.com/dbazure/download/playwright/builds/firefox/1572/firefox-ubuntu-24.04.zip`,
  118,965,377 bytes (Content-Length, a HEAD request; not downloaded). No publisher checksum exists: the sha256 would be
  the one we measure, then pinned in CI so every run gets the same bytes.
Options: (a) the runner image's Firefox, through `moz-firefox`: no download in CI, the browser players run, a moving
version, an experimental driver; (b) Playwright's Firefox 159.0 build 1572: a stable driver, a patched Firefox players do
not run, a 119 MB download pinned by our own sha256, plus a fetch-check-install step (about 25 lines). Recommendation:
(a), with his yes in advance to fall back to (b) if (a) fails for a driver reason (a failure Playwright's BiDi route
causes, not the page), shown to him with the failing lines before any (b) download is used.
The trial download (rule 3: nothing new used before his yes): neither route can be tried here without one.
- For (a): the PPA's `firefox` package for noble (version, address, size, sha256 from the signed index above) installed
  in this container with apt, which checks the index's signature; I list for him the other packages apt would pull in
  (`apt-get install --simulate`) before he answers, since he allowed no other new package.
- For (b): fetching the zip only to measure its sha256 (not unpacked or run) before his yes: the advisor agent rules
  whether that counts as "used"; its ruling is recorded here.

## 4. The two asks (his rules 2 and 3)
- Ask 1, right after the advisor agent's APPROVE: FIREFOX-SOURCE plus the trial download, with address, version, size
  and checksum, and the fallback yes in advance.
- Then try everything here: the Firefox project on the development page (`.gate/dist-development`, built with the
  wasm-bindgen file he approved, its two sha256s checked), the Chromium project unchanged on the new config, the e2e type
  check (workflow line 191's command) with the new config, and every new CI line run as in the cut where the container
  allows (`unshare --net` if allowed; otherwise said).
- Ask 2, in one message: the full diff of `toolchain.yml` (with the file's sha256 after the write) and of
  `playwright.config.ts`, each line marked tried and how. Written only after his yes, mode read right before the write
  under his rule of 2026-11-23, 17:15:07 UTC (write on `acceptEdits`, or on `default` naming it; any other value: stop).
  The config file is tier 1 and inside the fence, but it ships in the same ask so he sees all the text at once.

## 5. Risks and stop rules
- The first GitHub run is the only proof of the cut, the runner's Python 3.12 (tried here on 3.13.18; the scripts use
  `fractions`, `math` and the standard library only, no `math.fma`), the runner's Firefox path and Firefox inside a
  network namespace with no network. Stop rule: a red run is read, never re-run into green; any failure other than a
  plain typo in my own lines comes to him with the lines before any write.
- Firefox failure classes: a driver or preference fix that lives in `playwright.config.ts` → in ask 2; a spec or
  `render-counts.json` change → I ask him; a game-code cause → the three-line stop with options, no fix.
- Likely Firefox weak spots: the clock spec (Playwright's clock through BiDi), the rules spec's made-up address
  (request routing through BiDi), the Worker reads. Only the trial shows it.
- Job time: main's run took 2 min 7 s (run 45503161333; the cut step 1 min 47 s, the Chromium run 54.8 s, under the
  15-minute job limit). Added here: release tests ~11 s, checks ~2.4 s, module build ~10 s, a Firefox run of 13 tests
  ~1 min; route (b) adds a 119 MB fetch.
- Size: well under the 300-line stop (about 15 workflow lines for (a), about 40 for (b); about 15 config lines).
- Checkpoint saver: `checkpoint.mjs save` and `watch` push to the local branch's name (SAVER, STATUS_ARCHIVE), which
  here is `cs/ci-after-3a`. Departure, as Builder V3 had: the card and notes are committed by path and pushed by hand
  with `git push origin cs/ci-after-3a:claude/keen-tesla-v807cf` after each step.

## 6. Evidence
- Tried here already (logs in `.gate/trial/`): a. the six crates' release tests pass, 11.1 s, every test binary green;
  c. both checks exit 0 (0.23 s and 2.12 s), and two plants fail them (a `pin` line changed in `f32-expected.txt`: exit
  1, "differs or is missing: f32-expected.txt"; one byte of `chacha8-words.bin`: exit 1), the files restored and `git
  status` clean; d. `cargo build --locked --target wasm32-unknown-unknown -p devtools --lib` builds (9.9 s); the
  development page builds with the approved tool.
- After the write: the workflow's runs on GitHub (LONG-RUNS), push and pull request: the Firefox version line, the
  Firefox list of tests, both checks' lines, the module build line, the release tests' binaries; gate and smoke green.
- Not planted in CI: no plant for c or d in CI (a plant there is another CI write); the plants above are local.

## 7. Reviews, report, docs
- The advisor agent on this plan, then on the final head (full SHA). security-auditor on the final diff (tier 2), and
  infra-engineer reads the CI text before ask 2 (he named both). One round; a second only on what changed. At most 2
  agents at a time, Opus 5.5 at effort high.
- Docs pass (me, or docs-keeper): STATUS (in flight, next, follow-ups: 3A-GEN-CHECK and 3A-RULES-STEP done, E1 done,
  FIREFOX and 3A-OTHER-BROWSERS: Firefox in CI on Linux, Safari and WebKit later), REPO_MAP's rows for
  `toolchain.yml` and `playwright.config.ts`, the archive's marks, ROADMAP, DECISIONS (his words by script), the report
  `docs/reports/2026-11-25-ci-after-3a.md`.
- PR: no PR by me; `pr-title.txt` and `pr-body.md` in the notes; his main Advisor opens the draft and merges on his
  tap after the advisor agent approves the full SHA with gate, smoke and toolchain green on it.

## 8. What Mustafa checks
Nothing in Brave: nothing a player sees changes. He answers ask 1 and ask 2, then taps merge.

## 9. Already done in this container (reported, not hidden)
rustup installed the pinned toolchain 1.99.1 from `rust-toolchain.toml`; `npm ci --ignore-scripts` in the worktree; the
wasm-bindgen archive he approved fetched into the scratchpad, both sha256s checked; web reads: the runner-image readme
and install script (raw.githubusercontent.com), the PPA's index (moved to the scratchpad), two HEAD requests to the
Playwright download server; his opening block saved by an adapted copy of `chat-words.mjs`.
