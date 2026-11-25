# Checkpoint: ci-after-3a (CI Builder, session_019YNMUUemXge56777MiUWFz): the resume card
- Goal: the small CI pull request after change-set 3a: a. every crate's release tests (E1); b. Firefox in CI
  (FIREFOX-TESTS (b)); c. both vector generators' --check (3A-GEN-CHECK); d. the test-only module's own build line
  (3A-RULES-STEP, CI half). Plan: `ci-after-3a/plan.md` revision 2 (7951759), the advisor agent's APPROVE in
  `ci-after-3a/advisor-plan.md` ("Re-read of revision 2", with conditions: no checkpoint.mjs; a config change after ask
  2 goes back to him; the zip deleted if (b) is not used).
- Where: worktree `/home/user/Shamble-Frontlines/.gate/ci3`, local branch `cs/ci-after-3a`, pushed by hand with
  `git push origin cs/ci-after-3a:claude/keen-tesla-v807cf` (never force; read the remote head after). Base: main
  28254a73ab226379373e380eeee64336e58ef916. Never run checkpoint.mjs save or watch here (SAVER).
- His words, saved by script (`ci-after-3a/chat-words.mjs`, adapted from 6491b839): opening block; ask-1 answers
  (FIREFOX-SOURCE (c): GitHub's own Mozilla Firefox, fallback to Playwright's Firefox only for a failure Playwright's
  route causes, named with failing lines in ask 2; TRIAL-DOWNLOAD (a); the zip unopened unless the fallback happens,
  deleted at merge if unused; rule 5: a question waits as a pushed notes file named in the board post); PW-RUNS (a)
  and its route (Playwright through `node node_modules/@playwright/test/cli.js`, never npx or npm test, which the
  repository's deny list refuses; no install or install-deps; if refused, stop in three lines; ask 2 says CI keeps
  `npx --no --` and the trial ran the same program through node); EGL-TRIAL (a) (the two files for the container only;
  ask 2 says which Firefox tests passed with them, adds one read-only CI line printing whether the runner has
  libEGL.so.1, no install line for the runner; a runner without it is its own question; a failure caused by the game's
  page is the three-line stop).
- Done and tried here: a. 6 crates' release tests pass (11.1 s); c. both checks pass (0.23 s, 2.12 s), two plants
  fail them; d. `cargo build --locked --target wasm32-unknown-unknown -p devtools --lib` builds (9.9 s); the
  development page built with the approved wasm-bindgen (scratchpad `wb/x`, both sha256 checked). Firefox 161.0
  (`.gate/moz-firefox-161/x/usr/lib/firefox/firefox`) and EGL (`.gate/egl/x`) fetched, checked, unpacked; Playwright's
  zip measured only (`.gate/pw-firefox-1572/`, sha256 189ce3e0...368f, his Advisor measured the same). Test lists:
  Chromium 14 unchanged, Firefox 13. In Firefox: the 2 rule tests pass; the 11 drawing tests fail on "this.gl is null"
  with or without EGL; a blank-page probe (scratchpad `probe-webgl.mjs`) shows Firefox's reasons ("AllowWebgl2:false",
  forced: "Exhausted GL driver options"); no `/dev/dri` here. Logs: `.gate/trial/` (not committed).
- The draft Firefox entry of the test configuration: `ci-after-3a/draft-playwright-config.diff` (applied only for
  trials; the file stays as on main between them).
- Current step: FIREFOX-GL answered (a) (his words of 14:55 Amsterdam, saved by script: a red with the same "no WebGL" line on GitHub means the Firefox entry goes to rules only, the drawing tests a follow-up, three lines to him; any other red comes back with options; no install line or graphics override without his word; no env, stdbuf or nice with options before git or gh). Waiting for #68 (3b) to merge, then take main in, run lists and timings again; if not merged by Thu 26 Nov 12:00 Amsterdam, tell him in one line and go on from that main. .gate/egl deleted.
- Next three: 1. per his answer, the final Firefox entry and the CI text (the libEGL line included; route (a): the
  Firefox lines inside the cut after Chromium, `SHAMBLE_FIREFOX=/usr/lib/firefox/firefox`, version printed, missing
  file fails by name; header comment updated); 2. security-auditor and infra-engineer read the tried texts together
  (the one review round); 3. ask 2 in one message (full diffs, the file's sha256 after the write, each line marked
  tried, their findings, part d's other reading in one line), then write after his yes with the mode read first.
- Later: GitHub runs (LONG-RUNS), the advisor agent on the final head (no [skip ci] on it), docs pass (STATUS,
  REPO_MAP rows, archive marks, ROADMAP, DECISIONS by script, report `docs/reports/2026-11-25-ci-after-3a.md`),
  `pr-title.txt` and `pr-body.md` with the commit his Advisor reads them at; delete `.gate/egl`, `.gate/moz-firefox-161`
  and the zip at merge.
- Running: no agent, workflow or background shell.
- Key numbers: main's toolchain run 2 min 7 s (cut step 1 min 47 s, Chromium 54.8 s).
- The board add-on is active.
