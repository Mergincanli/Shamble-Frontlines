# Plan: the small CI pull request after change-set 3a (CI Builder), revision 2

_Revision 2 answers the advisor agent's REVISE of revision 1 (`advisor-plan.md`, required changes 1 to 7); each
change is marked "(R2-n)"._

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
     folder (`.gate/target-rules`), the registry remap and both profiles (`build.mjs:118-127`). No plant: a plant
     would mean editing `crates/`. Ask 2 names the other reading in one line (R2 ruling 5): a second workflow step with
     its own network cut.
   - c. two lines plus an echo, inside the cut (Python's standard library only; reads files, writes nothing; `-I -B`):
     `echo "== the two vector generators' checks: the committed files equal what each generator makes (nothing written)"`,
     `python3 --version`,
     `python3 -I -B crates/devtools/vectors/make_chacha8_vectors.py --check`,
     `python3 -I -B crates/devtools/vectors/make_f32_vectors.py --check`.
     Placed after the release tests, before the wasm32 builds.
   - b. the Firefox run, after the Chromium run, same shape (one worker, no retry, Playwright's 5-minute stop), the
     browser's version printed and a missing browser failing by name; the exact lines depend on his FIREFOX-SOURCE
     answer (section 3). Route (b) also adds a fetch, check and install step before the cut, like wasm-bindgen's.
     Route (a)'s path (R2-7): `/usr/lib/firefox/firefox`, read from the runner image's install script
     (`images/ubuntu/scripts/build/install-firefox.sh` in actions/runner-images: `apt-get install ... firefox` from the
     Mozilla team's archive, then a preference written to `/usr/lib/firefox/browser/defaults/preferences/syspref.js`);
     the CI line checks it is a plain file before the run and fails by name if not, and the trial checks the same
     layout in the package.
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
  default Linux path `/snap/bin/firefox`, so CI passes `executablePath`). It is a different route from the one
  Playwright uses with its own Firefox build; how many of our 13 tests it runs is unknown until tried (R2-4: the word
  "experimental" is dropped, no source in the installed package says it).
- The runner image `ubuntu-24.04` (readme of image 20261116.376.1, which lists Google Chrome 157.0.8103.78, the version
  run 45503161333 printed on main) carries Mozilla Firefox 160.0 from the Mozilla team's PPA (install script:
  `apt-get install ... firefox`, prefs in `/usr/lib/firefox/`): the same major version as his Windows Firefox 160. The
  PPA's index already offers 161.0 (`firefox_161.0+build1-0ubuntu0.24.04.1~mt1_amd64.deb`, 90,437,572 bytes, sha256
  bd304b0b...cf1ed1), so it moves with the image, as Chrome does (IMAGE-CHROME).
- Playwright's own Firefox for 1.65.0: Firefox 159.0, build 1572 (`playwright-core/browsers.json`), address
  `https://cdn.playwright.dev/dbazure/download/playwright/builds/firefox/1572/firefox-ubuntu-24.04.zip`, which answers
  307 to `https://playwright.download.prss.microsoft.com/dbazure/download/playwright/builds/firefox/1572/firefox-ubuntu-24.04.zip`,
  118,965,377 bytes. No publisher checksum exists, and Playwright checks none on its downloads (the advisor agent's
  read). Measured under the advisor agent's ruling (below), 2026-11-24: 118,965,377 bytes, sha256
  189ce3e09c61776010ab8a7cbffaa016b8b1914d3c9b50cb5ab6d0c5db4c368f, final address the Microsoft one above; it lies in
  `.gate/ci3/.gate/pw-firefox-1572/` (gitignored), mode 644, never unpacked or run. CI would pin these values so every
  run gets the same bytes.
Options: (a) the runner image's Firefox, through `moz-firefox`: no download in CI, the browser players run, a version
that moves with the image, a driver route our tests have never met; (b) Playwright's Firefox 159.0 build 1572:
Playwright's usual route, a patched Firefox players do not run, a 119 MB download pinned by our own sha256, plus a
fetch-check-install step before the cut (about 25 lines). Recommendation: (a), with his yes in advance to fall back to
(b) if (a) fails for a driver reason (a failure Playwright's BiDi route causes, not the page).
The fallback (R2-2): on such a failure the chat tells him in a message with the failing lines and carries on with (b)
under his yes in advance (the trial unpacks the measured zip only if its size and sha256 still equal the values
above); if (b) fails too, the chat stops and tells him in three lines with options.
The trial download for (a) (rule 3; R2-4), stated whole in ask 1:
- The trial installs Firefox **161.0** (the PPA's current package), while the runner image of 2026-11-16 carries
  **160.0**: the trial is not the same version as CI.
- The source: the Mozilla team's package archive (Launchpad PPA `~mozillateam/ubuntu/ppa`, "Firefox ESR and
  Thunderbird stable builds"), the same archive the runner image installs from; its apt source line would be
  `deb https://ppa.launchpadcontent.net/mozillateam/ppa/ubuntu noble main`, but the trial adds no apt source.
- The chain, checked here 2026-11-24: the archive's signing key, fingerprint
  738BEB9321D1AAEC13EA9391AEBDF4819BE21867 (Launchpad's own record of the archive, api.launchpad.net), fetched from
  keyserver.ubuntu.com (key file sha256 26b00a44534deecf0148a16599d03692e668c6ae24a8fe60a64143253db84e92; a new download,
  data, not a program); `InRelease` for noble: "Good signature from "Launchpad PPA for Mozilla Team"" with that key
  (gpg 2 in this container); its SHA256 line for `main/binary-amd64/Packages.gz` (69,487 bytes,
  638157d5e83da3e0bcc7a01b7e371252c3fd93c647882f2c567bb58ce892a37d) equals the index read; the index's `firefox` entry:
  version 161.0+build1-0ubuntu0.24.04.1~mt1, `pool/main/f/firefox/firefox_161.0+build1-0ubuntu0.24.04.1~mt1_amd64.deb`,
  90,437,572 bytes, sha256 bd304b0b181c4fed449279e25d309075c3a2e85345e63d1118c5085cefcf1ed1.
- How: curl over https into a new folder under `.gate/`, size and sha256 checked against the index, then
  `dpkg-deb -x` into `.gate/` (no apt, no root, none of the package's install scripts run). Its 30 dependencies are all
  installed in this container already (`dpkg -s` on each, 2026-11-24), so no other package comes in. The trial checks
  that the package's layout holds `usr/lib/firefox/firefox`, the path CI passes (section 2, b).
The ruling on fetching (b) only to measure (the advisor agent, `advisor-plan.md`, recorded here): fetching and hashing
is not "used" under his rule 3, since the rule asks for a checksum and none is published; conditions: plain https curl
with redirects limited, never `playwright install`, a new empty folder under `.gate/`, only `stat` and `sha256sum`, the
size must equal 118,965,377 or stop, ask 1 says it was fetched and where and that this ruling allowed it (his to
overrule), deleted at once if he picks (a) with no fallback, says no, or says the fetch counted as use; his main Advisor
is asked to measure it over another route.

## 4. The two asks (his rules 2 and 3)
- Ask 1, right after the advisor agent's APPROVE: FIREFOX-SOURCE plus the trial download, with address, version, size
  and checksum, and the fallback yes in advance.
- Then try everything here: the Firefox project on the development page (`.gate/dist-development`, built with the
  wasm-bindgen file he approved, its two sha256s checked), the Chromium project unchanged on the new config, the e2e type
  check (workflow line 191's command) with the new config, and every new CI line run as in the cut where the container
  allows (`unshare --net` if allowed; otherwise said).
- Before ask 2 (R2-3): security-auditor and infra-engineer read the tried texts together (two agents, his rule 7);
  that is the change-set's one review round, and its fixes go into the texts before he sees them. Ask 2 lists their
  findings and what was done with each.
- Ask 2, in one message: the full diff of `toolchain.yml` (with the file's sha256 after the write) and of
  `playwright.config.ts`, each line marked tried and how. Written only after his yes, mode read right before the write
  under his rule of 2026-11-23, 17:15:07 UTC (write on `acceptEdits`, or on `default` naming it; any other value: stop).
  The config file is tier 1 and inside the fence, but it ships in the same ask so he sees all the text at once.

## 5. Risks and stop rules
- The first GitHub run is the only proof of the cut, the runner's Python 3.12 (tried here on 3.13.18; the scripts use
  `fractions`, `math` and the standard library only, no `math.fma`), the runner's Firefox path and Firefox inside a
  network namespace with no network. Stop rule (R2-1): a red run is read, never re-run into green; every change to
  the CI file, a typo fix included, goes to him first with its exact text and the file's new sha256, and is written
  only after his yes, the mode read right before. On the final head the CI file's sha256 must equal the one he
  approved.
- Firefox failure classes: a driver or preference fix that lives in `playwright.config.ts` → in ask 2; a spec or
  `render-counts.json` change → I ask him; a game-code cause → the three-line stop with options, no fix.
- Likely Firefox weak spots: the clock spec (Playwright's clock through BiDi), the rules spec's made-up address
  (request routing through BiDi), the Worker reads. Only the trial shows it.
- Job time: main's run took 2 min 7 s (run 45503161333; the cut step 1 min 47 s, the Chromium run 54.8 s, under the
  15-minute job limit). Added here: release tests ~11 s, checks ~2.4 s, module build ~10 s, a Firefox run of 13 tests
  ~1 min; route (b) adds a 119 MB fetch.
- Size: well under the 300-line stop (about 15 workflow lines for (a), about 40 for (b); about 15 config lines).
- Checkpoint saver: `checkpoint.mjs save` and `watch` push to the local branch's name (SAVER, STATUS_ARCHIVE), which
  here is `cs/ci-after-3a`. Departure, as Builder V3 had: `checkpoint.mjs save` and `watch` are never run (R2-6); the
  card and notes are committed by path and pushed by hand with `git push origin
  cs/ci-after-3a:claude/keen-tesla-v807cf`, never with force, the remote head read after each push. The head sent for
  approval carries no `[skip ci]` (so toolchain runs on it); nothing is pushed after the final approval; the report
  names the commit where his main Advisor reads `pr-title.txt` and `pr-body.md` before the last commit deletes the notes.

## 6. Evidence
- Tried here already (logs in `.gate/trial/`): a. the six crates' release tests pass, 11.1 s, every test binary green;
  c. both checks exit 0 (0.23 s and 2.12 s), and two plants fail them (a `pin` line changed in `f32-expected.txt`: exit
  1, "differs or is missing: f32-expected.txt"; one byte of `chacha8-words.bin`: exit 1), the files restored and `git
  status` clean; d. `cargo build --locked --target wasm32-unknown-unknown -p devtools --lib` builds (9.9 s); the
  development page builds with the approved tool.
- After the write: the workflow's runs on GitHub (LONG-RUNS), push and pull request: the Firefox version line, the
  Firefox list of tests, both checks' lines, the module build line, the release tests' binaries; gate and smoke green.
- Not planted in CI: no plant for c or d in CI (a plant there is another CI write); the plants above are local.
- Also (R2-7): the legacy gate (`node tools/gate.mjs`) after each step; `npm test -w packages/e2e` on the new config;
  the test lists (`playwright test --list`): Chromium unchanged at 14, Firefox exactly the 13 named (the 14 less the
  allocation spec's one); the permission mode read right before each of the two writes (CI file, config file).

## 7. Reviews, report, docs
- The advisor agent on this plan, then on the final head (full SHA). security-auditor and infra-engineer read the
  tried texts before ask 2 (R2-3; he named both); on the final head security-auditor checks only that the written
  file equals the approved text (its sha256) and anything changed since. One round; a second only on what changed. At most 2
  agents at a time, Opus 5.5 at effort high.
- Docs pass (me, or docs-keeper): STATUS (in flight, next, follow-ups: 3A-GEN-CHECK done; 3A-RULES-STEP only its CI
  half done, the rule that `devtools`' first native-only dependency is target-gated stays open; E1 done; FIREFOX done
  for CI on Linux; 3A-OTHER-BROWSERS: Firefox done, WebKit and Safari stay open (R2-5); a new follow-up for the stale
  comment in `isolation.spec.ts:2-3`, "Firefox and WebKit run in CI later", which is a spec and outside the fence), REPO_MAP's rows for
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
Playwright download server; his opening block saved by an adapted copy of `chat-words.mjs`. After the advisor agent's
review of revision 1: Launchpad's record of the archive (api.launchpad.net), the archive's key (keyserver.ubuntu.com)
and its signed `InRelease`, checked with gpg into a fresh keyring in the scratchpad; Playwright's Firefox zip fetched
and measured under the ruling (section 3), not unpacked.
