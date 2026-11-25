# Ask 2 for Mustafa, 2026-11-25: every CI and test-configuration text in one message

Branch `claude/keen-tesla-v807cf`, worktree head after this file's commit; main 311c048 (3b #68 and the guard fix #69)
taken in at 77d28ee. Nothing below is written yet: `.github/workflows/toolchain.yml` and
`packages/e2e/playwright.config.ts` stand as on main.

## The two texts (exact; the diffs are in this folder)
1. `draft-toolchain.diff` (sha256 55bdbd69a9c4e06609a2ef3e6547e0a1562d7217f3bb4feb80a2a8b4362e424f): 18 lines added,
   2 removed. `.github/workflows/toolchain.yml` today: sha256 56d37fc64ccfce2a21765613401a746633c6a9c661f3af52a9054bf650df9811
   (unchanged on main since 28254a7); after the write: sha256
   01898e4b3833812e945882106baa2bb3590866aff3c8d2dfa487a1fe38ce5742 (the diff applied with `patch` gives it).
2. `draft-playwright-config.diff` (sha256 7f79116888311c52e29b14839ff3fa72573e02bde96e4fa0becba8170d433dbe): 20 lines
   added, 1 removed. `packages/e2e/playwright.config.ts` after the write: sha256
   fed4b79d38cbbbf15cd69b0225e576dbfce880f9a69292096e74b1619b9e43ed.
Size: 41 changed lines outside notes and docs (stop line 300).

## Each line, tried and how (in this container, on main 311c048; `trials-main-311c048.md`)
- a. Every crate's release tests: tried, exit 0, 31 s, 113 tests.
- c. The two generator checks: tried, exit 0 (0.2 s, 1.6 s); two planted faults each fail their check; Python 3.12.3 on
  infra-engineer's side also exit 0.
- d. `cargo build --locked --target wasm32-unknown-unknown -p devtools --lib`: tried, exit 0, 5 s. The other reading of
  "a build step of its own", a second workflow step with its own network cut, was not chosen: one line inside the
  existing cut does the same with nothing new outside it. It proves `devtools` builds for wasm32 on its own named line;
  it proves nothing new about the module the tests load, which the build script still makes in both profiles.
- The libEGL line (his EGL-TRIAL point 3): tried under `bash -euo pipefail`: here it prints "libEGL.so.1: not in the
  loader cache of this runner (printed, not judged)" and the script goes on; it never stops the job.
- The Firefox file check and `--version`: tried on the trial Firefox's path (161.0 here; the runner has 160.0 at
  `/usr/lib/firefox/firefox`, read from GitHub's install script).
- The Firefox run: tried through node (CI keeps `npx --no --` like the file's other lines; the trial ran the same
  program as `node node_modules/@playwright/test/cli.js`, his PW-RUNS (a)): the 2 rule tests pass; the drawing tests fail
  here on "This page could not start drawing: ... this.gl is null" (no WebGL in this container; his FIREFOX-GL (a): the
  first GitHub run is their trial; the same line there means the Firefox entry goes to the rule tests only, which his
  answer already allows, three lines to him).
- The Chromium run with the new configuration: 16 passed, 3.3 min. Test lists: Chromium 16, Firefox 15, node 1.
- The e2e type check (workflow line 191's command) with the new configuration: passes. YAML parses; the script inside
  single quotes holds no apostrophe and passes `bash -n`. Legacy gate (no smoke): PASS.
- Not tried here: the network cut itself, the runner's Firefox inside it, the runner's Python 3.12 inside it.

## The review round (security-auditor and infra-engineer, together; their notes in this folder)
- security-auditor: containment passes (nothing weakens the cut, the root-owned tool, the token or the no-cache rule).
  - F1 Medium: the config comment said "the build players run", but Playwright starts Firefox with its own test
    preferences, one of which (`security.fileuri.strict_origin_policy` false) lets a page opened as a file run its
    scripts in Firefox where Chromium blocks them. Fixed: the comment now says so (taken word for word). The option to
    leave `file-url.spec.ts` out of Firefox is his question FF-FILEURL below.
  - F2 Low: the libEGL line printed "not on this runner" also when `ldconfig` itself failed. Fixed: its tested line.
  - Notes: CUT-PATH gets new evidence for its follow-up (GitHub's image build makes `/opt` and `/usr/share` writable to
    all, so Chrome's folder is probably writable inside the cut; Firefox's `/usr/lib/firefox` is root-owned); a stronger
    check on Firefox's file would not be a security boundary (no approved bytes); TOKEN-WORDS was due "at the next CI
    edit": deferred again, to keep this pull request to his four parts (named in the report).
- infra-engineer: nothing makes the first run fail except the known WebGL question.
  - F1 Medium: the header and the Firefox echo line said "the browser tests"; after his FIREFOX-GL rule could cut
    Firefox to the rule tests, they would be false. Fixed: both now say "the tests the firefox project names" (taken
    word for word), so the rule-tests-only change touches only the configuration.
  - F2 Low: as security-auditor's F2 (fixed by its line).
  - F6 Medium: Firefox's 5-minute stop may be tight even with WebGL working (question FF-STOP below).
  - F3 Low: the Firefox run starts only if the Chromium run passes, so a red Chromium run gives no Firefox evidence.
  - F4 Low: the job about 10.5 to 11.5 min (13 if both browsers hit their stops) against its 15-minute limit; public
    repository, so hosted minutes are free.
  - F5 Low, not verified: Firefox's own sandbox inside the cut; if it fails to start, the 2 rule tests fail too, which
    tells it apart from WebGL.

## Questions
- CI-WRITE-2: may I write the two texts above exactly? (a) yes; (b) yes with changes he names; (c) no.
  Recommended (a). The CI file is written only after his yes, with my permission mode read right before (his rule of
  2026-11-23, 17:15:07 UTC); the configuration file in the same step.
- FF-STOP: Firefox's run keeps Playwright's 5-minute stop for the first run? (a) keep 5 minutes (a red on "Timed out
  waiting 300s" with no WebGL line is easy to tell apart and comes back to him); (b) 7 minutes; (c) 7 minutes and a
  20-minute job limit. Recommended (a): (b) or (c) is another text change he would see first.
- FF-FILEURL: does `file-url.spec.ts` run in Firefox? (a) yes, with the comment that says Playwright's preference makes
  its Firefox reading differ from a player's (as drafted); (b) leave it out of Firefox. Recommended (a): the spec only
  records which way the page behaves, and both ways pass.
- GUARD-LOAD (from his block of 18:05 Amsterdam): this session's hooks run from its main folder, still at 28254a7; its
  `routine-guard.mjs` differs from main's, so the fixed guard is not loaded. May I bring the main folder to main with
  `git merge --ff-only origin/main` (a form CLAUDE.md allows there)? (a) yes; (b) no. Recommended (a).
