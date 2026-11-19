# Plan: the CI-WASM pull request (CI builds the WebAssembly and runs the tests that need it), with the docs pass for #61's merge

_Revision 1, written by the main session, 2026-11-19, 17:53 UTC (from the clock), from infra-engineer's notes
(`infra-engineer-ci-plan.md`), test-engineer's notes (`test-engineer-followups.md`) and the card (`../ci-wasm.md`).
Nothing outside `.checkpoints/` and `.gate/` is edited before the advisor agent's APPROVE; no line under `.github/`
is written before Mustafa's yes on its exact text, with the chat in Accept edits._

_Revision 2, 18:14 UTC (from the clock): the advisor agent's verdict on revision 1 is **APPROVE with conditions**
(`advisor-plan.md`; no Critical or High finding; steps 1 to 3 may be built). Its eight conditions are worked in:
two questions go to him instead of six, and its rulings on the other four stand in section 16; the chat's mode is
read before each of the three CI writes; the dated note and the quoted question are copied by script from the
chat's message; two lead texts corrected; the stop rule widened; the docs get a size and a stop line; the tier-2
evidence added; the record of the compiled Python file completed._

## 0. In one screen
- **What:** `.github/workflows/toolchain.yml` gains 90 lines and loses none. Before the network cut the job fetches
  the one wasm-bindgen archive Mustafa named, checks its size and sha256, runs GitHub's own check of it
  (`gh release verify-asset`), and installs the one program root-owned. Inside the cut it builds the page twice
  (production, development), runs the boundary tests, type-checks the Worker, the page and the browser specs, and
  runs the 12 browser tests in the Google Chrome that GitHub's runner image already holds. **Nothing is downloaded
  for the browser.**
- **With it, small:** four follow-ups the archive placed here (a test of the build's refused variables; five browser
  checks made robust for a slower machine without loosening what they prove; a 15-line declaration file so that the
  specs can be type-checked with no new dependency; one `.gitignore` line), and the docs pass that records #61's
  merge (his words of 2026-11-19, 16:08:04 UTC: it "stays with the CI pull request").
- **Not in it:** Firefox in CI (question FIREFOX-TESTS), any manifest or lockfile change, any new dependency, any
  edit to `gate.yml`, the kit or the gate, anything a player sees.
- **Risk tier 2** (a CI file). Reviews: the advisor agent and security-auditor, with infra-engineer and
  test-engineer. Merge: his go, in this day session.
- **Planned size:** about 155 added lines outside the docs (90 of them the workflow); stop line 310.
- **Questions for Mustafa (section 16):** CHROME-SOURCE, then CI-TEXT. The advisor agent ruled on four more
  (PERM-ATT, PLANTS, FIREFOX-TESTS, PYCACHE-NOW); he is told each ruling and can object.

## 1. Goal
After this pull request, a push or a pull request cannot go green unless the Wasm builds with the checked tool, the
boundary tests pass on that Wasm, the Worker, the page and the specs type-check, and the 12 browser tests pass, all
inside the network cut. His words (CI-WASM (a), 2026-11-18): "It runs GitHub's own check on the tool's file before
the tool's first run there, and its report quotes that check's output." And of 2026-11-19, 14:28:25 UTC: "Until the
CI-WASM pull request is merged, those tests ran in your container only."

## 2. What Mustafa can see after it
Nothing in the game. On GitHub: the `toolchain` check of every pull request now lists the tool's check, GitHub's
own check, the two builds and "12 passed". The report quotes GitHub's check's output from the job's log.

## 3. Before any edit
1. The advisor agent's APPROVE of this plan (its notes: `advisor-plan.md`); security-auditor reads the CI text once
   before it goes to him (`security-auditor-ci-text.md`).
2. His yes on the exact text (`ci-edit.diff`, section 5) and his answers to section 16. The chat's mode is read
   from the session record **right before each of the three CI writes** (the file; P-GH's two lines; their
   take-back), and the main session stops unless it is Accept edits (his WRITE-MODE: "right before each kit
   write"). After each write the file's sha256 is compared with its scratch copy: the proposed file, then the
   proposed file with the plant, then the proposed file again. Until his yes only the non-CI steps are built
   (section 8, steps 1 to 3). One of them depends on his answer: if he refuses CI-TEXT or changes it so that the
   specs' type-check line goes, `packages/e2e/node-names.d.ts` is removed again, or kept with its reason in the
   report.
3. No manifest, lockfile or dependency changes: `.claude/CHECKPOINTS.md` section 3's hold is not touched.

## 4. Files, with their lane
All are lane 1's (Part 15: `packages/e2e/tests/skeleton/` is lane 1's folder; lane 2's `render/` is not touched).
| File | Change | Protected |
|---|---|---|
| `.github/workflows/toolchain.yml` | +90, -0 (section 5) | yes: CI; his yes on the text |
| `.gitignore` | +1 `dist-*/` (SA-DIST); the `__pycache__/` line stays with change-set 3 (PYCACHE-NOW (b)) | no |
| `packages/client/scripts/build-checks.mjs` | `CARGO_OVERRIDES` moved in; new `refusedBuildSettings(env)` | no |
| `packages/client/scripts/build.mjs` | the pattern out; one call changed | no |
| `packages/client/scripts/build-checks.test.mjs` | one test | no |
| `packages/e2e/playwright.config.ts` | chromium project: `workers: 1`, `timeout: 60_000` | no |
| `packages/e2e/tests/skeleton/skeleton.spec.ts`, `clock.spec.ts`, `allocation.spec.ts` | TE-NT6 (section 6) | no |
| `packages/e2e/node-names.d.ts` | new, 15 lines (E2E-TYPES) | no |
| `docs/` | the docs pass (section 7) and the report | no; `docs/VISION_V2.md` and `docs/DECISIONS.md` carry his words |
`gate.yml`, `tools/`, `.claude/`, every `CLAUDE.md`, every manifest and lockfile: unchanged.

## 5. The CI text (kept apart from everything else, as his words of 16:08:04 UTC ask)
- **The exact text:** `ci-edit.diff` in this folder (125 lines; 90 added, 0 removed; sha256
  282890b716f767d01e910a5a18dd2f454a9787abe4ecdee61e8521ce2df3290e), made by `diff -u` of main's file against the
  scratch copy `.gate/ci-wasm/proposed/.github/workflows/toolchain.yml` (190 lines, sha256
  76bfed6e345e77bb8d5799c05994df27c5edbd721352f10bf9466a59ddff96d8; it parses as YAML: one job, nine steps,
  `permissions: contents: read`, `cache-mode: none`, 15 minutes). It is infra-engineer's draft (88 lines) plus two
  lines and two changed lines from the main session for the specs' type check (test-engineer's block 4).
- **What it does, in order:**
  1. The job gets seven values (`WB_*`): the tool's version, the archive's name, size and sha256, the program's size
     and sha256, all his of 2026-11-18, and the install path `/opt/wasm-bindgen-0.2.130/wasm-bindgen`.
  2. Step "the archive": each value must stand as an exact line in `packages/client/scripts/wasm-bindgen-tool.mjs`
     (read as text, never imported: no repository code runs with the network); then the download (https only, at
     most two redirects, at most 12,000,000 bytes), its size, then its sha256, before any tar call.
  3. Step "GitHub's own check": the only step that holds the job's token. It prints `gh --version`, refuses a `gh`
     older than 2.93.0 (which stopped sending the token to the TUF mirrors, CVE-2026-48501; the image lists
     2.103.3), then runs `gh release verify-asset 0.2.130 <archive> --repo wasm-bindgen/wasm-bindgen`. It proves
     the file is byte for byte an asset GitHub recorded and signed for that immutable release; it does not prove how
     the program was built (security-auditor's reading of 2026-11-18 stands).
  4. Step "the one program": the member list must be the folder and its six files; no link, no name with a space,
     nothing above 64 MiB; only `wasm-bindgen` is taken out, installed owned by root with mode 0555 (so nothing
     inside the cut can swap it), then its size and sha256 are checked.
  5. "Fetch every dependency" stays the last step with the network, unchanged.
  6. Inside the cut the tool's path enters the bare environment (`SHAMBLE_WASM_BINDGEN`), and after today's lines:
     the production build and the development build (into `.gate/dist-production` and `.gate/dist-development`),
     the Wasm's sha256 and the builder path it carries (printed, not judged: RR-F5), the boundary tests, five
     type-check lines (the Worker and the page, each with the lib check and again with their tests; the browser
     specs), Chrome's version, and the browser tests (`--workers=1 --retries=0 --reporter=list`, stopped by
     Playwright after 5 minutes).
- **Tried here** (infra-engineer's block 6, with the container's Chromium 153 and `node <path>` in place of `npx`):
  the value, size, sha256 and member checks pass and each fails on its broken copy; both builds under the cut's
  variable list; boundary tests 7 passed; the four type-check lines exit 0; browser tests 12 passed in 68 s with
  one worker. The fifth type-check line: exit 0 in 1.2 s (the main session, 17:37 UTC, with the scratch declaration
  file).
- **Never run anywhere yet** (read on the first runner run, under the stop rule of section 9): GitHub's check
  itself (its output, exit code, time, and whether `contents: read` is enough: PERM-ATT); the download through
  GitHub's redirect on a runner; `sudo -n install`; Chrome 157 of the image inside `unshare --net` with
  no_new_privs; the job's time; the `npx --no --` route of the new lines.
- **One job, not two:** the build that makes what players get stays inside the cut, and the browser tests need only
  127.0.0.1, so they run there too; a second job would need its own toolchain install and its own tool route.
  Expected added time: 2 to 2.5 minutes; the job's limit stays at 15.

## 6. Code: the follow-ups taken here, with their exact symbols
From `docs/STATUS_ARCHIVE.md`, "For the CI pull request (CI-WASM)". Design and line counts: test-engineer's notes.
- **RR-N4 with VF-L5** (the build refuses cargo's settings from the environment, untested):
  `packages/client/scripts/build-checks.mjs` gets `export const CARGO_OVERRIDES` (moved from `build.mjs:39-47` with
  its comment) and `export function refusedBuildSettings(env)` (the names in `env` the Wasm build refuses; only
  names are read); `build.mjs` imports it and line 68 becomes `const overrides = refusedBuildSettings(process.env);`.
  `build-checks.test.mjs`: one test, 15 refused names (one per family of the pattern, and two build-folder names with
  more after them) and 15 allowed (the two build-folder names, every name CI's cut sets, `SHAMBLE_WASM_BINDGEN`,
  two names that only contain a refused one). CI already runs it (`npm test --workspaces`). About +35, -9 (moved).
- **TE-NT6** (browser checks tuned on this container only). Fifteen of the twenty timing points stay; five become
  robust without loosening what they prove:
  1. `playwright.config.ts`, chromium project: `timeout: 60_000` (today the 30 s default cuts off the 60 s wait in
     `report.ts:38`) and `workers: 1` (Playwright's own advice for CI; the tests were tuned beside a second browser).
  2. `skeleton.spec.ts`, hidden tab: wait for 4 new page frames after hiding and after showing before reading the
     Worker's numbers (the page asks the Worker only from its frame loop, so a slow page reads a stale report).
  3. Same test, line 91: the fixed bound of 60 ticks becomes `Math.floor(30 * seconds) + 2`, from the seconds
     measured around the window: tighter on a fast machine, not false on a slow one, and it still catches a Worker
     that runs the hidden gap on return.
  4. `skeleton.spec.ts:105`, the broken-Worker notice: `{ timeout: 20_000 }` on that one expect.
  5. `clock.spec.ts:11-13`: wait for the first block (tick at least 1) before reading the start.
  And readings: one `console.log` line per test before its first expect on those numbers (frames, posts, the
  window's real milliseconds, wakes, ticks, lateness, drops), so a red run on a runner comes with its numbers.
  About +15, 4 changed. No bound is lowered: the allocation floors (more than 20 frames; the byte windows) stay.
- **E2E-TYPES** (the specs are type-checked by no tool): new `packages/e2e/node-names.d.ts`, 15 lines declaring the
  four Node names the specs use (`readFileSync`, `path.join`, `pathToFileURL` and `fileURLToPath`, `process.env`).
  With it the specs' type check passes today with 0 errors and catches 6 of 6 planted type errors, one of which
  Playwright itself lets pass. The other way, `@types/node`, is two new packages from the internet: not here.
- **SA-DIST:** `.gitignore`: `dist-*/` after `dist/`.
- **RR-F5** (the builder's path inside the Wasm): CI prints the Wasm's sha256 and the path; nothing is asserted.
  The fix (a path remap from the repository's own settings) is change-set 3's, with rust-reviewer.
- **SA-TREE** (a test that every runtime crate has its overflow override): moves to change-set 3, which adds the
  random-number crate and so touches that list anyway (test-engineer's draft: about 52 lines, in its notes).
The netcode proof test's rules: no new manifest or config file; no root `.cargo/config.toml`; no `"type"` in the root
`package.json`; a search of the changed files for the spike's folder name is part of step 3.

## 7. The docs pass (his words: it rides this pull request; the report keeps the workflow text apart)
1. `docs/DECISIONS.md`, by `decisions-3.mjs` (always rebuilt from main's copy; tried into a scratch file, every
   check "yes"): his go of 14:28:25 UTC with BROWSER-SET (a); his "a, yes" of 14:31:30 UTC (VISION-NOTE (a)); a
   pointer (marked as a pointer, not a decision) to his browser check; his two notes of 16:08:04 UTC; and one dated
   pointer under the entry of 13:36:37 UTC (its header says "accepted as the report writes it down"; his words are
   "is accepted as you wrote it down, with your lesson"): by REC-OLD a line that records what he decided is not
   reworded. Every later message of his in this pull request (his yes on the text, his answers) is one more entry.
2. `docs/reports/2026-11-19-cs2-browser-check.md`, new, by `browser-check-record.mjs`: the reading of his eight
   reports against the zip's README, and his four report messages whole (609 lines, about 500 of them his quoted
   words). His go: "My browser reports are recorded with the next docs pass". The picture he sent is not in the
   repository (it shows a folder path of his computer).
3. `docs/VISION_V2.md`, by `vision-note.mjs`: the dated note he said yes to (VISION-NOTE (a)), copied by script
   from the chat's own message of 14:29:51 UTC in the session's transcript (uuid
   699736ec-f365-4458-aceb-592bba842d59, saved whole as `chat-2026-11-19T142951-vision-note.md`: 2433 bytes, sha256
   6bf347a2...c423), never typed from the card; directly under the decided browser line of section 4, which stays
   unchanged; and the sentence appended to the "Updated on" lines. The question and its two options quoted in the
   DECISIONS entry come from the same message by the same script. Compared at 18:12 UTC: the card's copy of the
   note equals the chat's byte for byte (309 bytes, sha256 7bf61530...4f92), and so does the sentence (61 bytes);
   the report says so.
4. docs-keeper, after the build: `docs/STATUS.md` (at most 120 lines: #61 merged at f5030a3; this pull request in
   flight; "Waiting on Mustafa"), `docs/ROADMAP.md`, `docs/REPO_MAP.md` (CI's new parts; the new files),
   `docs/STATUS_ARCHIVE.md` (the block of #61: the group "For the CI pull request" marked item by item as done here
   or moved to change-set 3; a new group "From his browser check" with BR-GAP, FPS-LINE, NEG-ZERO, FF-WAKES,
   BR-LATE, ADAPTER, TQ-FF, NOT-SHOWN; a dated pointer under the FIREFOX line of 13:36:37 UTC saying what his
   Firefox check found; the advisor agent's nits on head 4cf30ce).
5. The report `docs/reports/2026-11-19-ci-wasm.md` (`/advisor-report`): section "The workflow text" holds the
   merged diff of `.github/workflows/toolchain.yml` whole, with the sha256 of the text he said yes to and a line
   saying whether the two are byte-equal; P-GH's two lines, the commit that wrote them and the commit that took
   them out, and the take-back's tree shown equal to step 4's tree; and a before/after table of every rule
   sentence of the workflow file (each new or stricter; a looser one would need his exact words). The docs pass has
   its own section. GitHub's check's output is quoted from the job log with the run and job ids.

## 8. Build steps, in order
1. (No CI file.) RR-N4: move the pattern, add the function and the test. Client unit tests: 49 passed.
2. (No CI file.) TE-NT6's five edits and the readings lines; `node-names.d.ts`; `.gitignore`. The build, then the
   browser tests three times with one worker on the container's Chromium; change-set 2's plants that touch the
   edited specs run again (each must still fail).
3. The docs pass, items 1 to 3 of section 7. Gate; commit by path (code and docs in separate commits); push; the
   pull request opens as a draft. CI runs today's workflow on it (the new unit test included).
4. After his yes: read the chat's mode (stop unless Accept edits); write `.github/workflows/toolchain.yml` from
   the scratch copy; compare its sha256 with the proposed file's; commit that one file; push. **The first runner
   run, under the stop rule.**
5. The two planted changes, one per push, each taken back in the next commit. P-GH is a CI write: the mode is read
   before its two lines are written and again before they are taken out, with the sha256 compared each time.
   P-BROWSER is one value in `render-counts.json`. After the last take-back the tree must equal step 4's tree.
6. Review round 1 on the final diff (security-auditor, infra-engineer, test-engineer; at most two rounds), fixes;
   the verifier; the advisor agent on every pushed head; docs-keeper; the report; the checkpoint folded and deleted;
   the pull request marked ready; stop for his go.

## 9. Evidence
- **Legacy gate:** `node tools/gate.mjs --no-smoke`, unchanged and green (nothing in `legacy/` or `tools/` changes).
- **Logic (RR-N4):** the new unit test; shown failing on a broken copy of the pattern (test-engineer's 7 mutants:
  7 caught); end to end, `RUSTFLAGS=-Coverflow-checks=off ... build.mjs --wasm-only` must exit 1 with "build
  settings from the environment are refused: unset RUSTFLAGS".
- **Tests (TE-NT6):** three green browser runs here with one worker; the plants of change-set 2 on the edited
  specs still fail as they must; the readings lines appear in the output.
- **CI, on the runner:** `gate`, `smoke` and `toolchain` green on the head, push and pull request, read by hand.
  From the first clean run the report quotes: GitHub's check's whole output; the tool's size and sha256 lines; the
  image's, gh's and Chrome's versions; "Tests 7 passed"; "Running 12 tests using 1 worker" with each test's
  duration and "12 passed"; the Wasm's size and sha256 on both runners against this container's (28,987 bytes
  here, its sha256 printed by the same line); the job's time.
- **Risk tier 2's own evidence:** the kit's three test files (`.claude/hooks/routine-guard.test.mjs`,
  `.claude/hooks/settings.test.mjs`, `.claude/scripts/checkpoint.test.mjs`) pass with the same counts on main and
  on the head; the before/after table of the workflow's rule sentences (section 7, item 5); security-auditor's
  review with each finding addressed or deferred in writing.
- **The stop rule for the new lines** (as #60's: none has run on a runner). It holds on **every** run of the new
  lines until the report is in, not only the first. The main session stops and tells him, with the log's lines and
  before any fix, push or re-run, if: a new step or line fails; or the log lacks a final address on GitHub's
  release-asset host, the archive's "size: 11860397" and ": OK", a gh of 2.93.0 or later and its success line naming
  release 0.2.130, the seven members, "mode 555, owner root:root, 8170784 bytes" and the program's ": OK", "the
  network is cut", the two size tables, "Tests 7 passed", Chrome's version line, "Running 12 tests using 1 worker"
  and "12 passed"; or a browser test fails once (nothing is re-run into green); or the job takes more than 10 of
  its 15 minutes; or a planted run fails anywhere but at its own step, or without its expected line (P-GH: gh's
  line that the file is not a subject of the release's statement, not a permission line and not "no attestations
  found"; P-BROWSER: the test's own difference of 4 against 3, with no retry line). Not a stop, only recorded: the
  Wasm's bytes differing from here (expected: RR-F5); which backend the runner's Chrome starts.
- **Planted changes** (PLANTS): P-GH, GitHub's check is given a file that is not the release's asset (two lines of
  CI text before the real check, so his yes covers them and their removal): the step must fail with gh's own line
  and nothing after it runs; it also answers PERM-ATT. P-BROWSER, one committed render count changed from 3 to 4:
  a browser test must fail inside the cut, with no retry. Shown here instead of on a runner, with the same
  commands: a boundary test with a wrong value, and a spec with a type error that Playwright lets pass.
- **What this cannot show:** Firefox and WebKit; WebGPU and a real GPU; Brave; any timed mark on a named machine;
  how the tool's program was built.

## 10. Risks
- **The image's Chrome moves with the image** (157.0.8103.50 in image 20261111.370.1). A later image can bring a
  Chrome that Playwright 1.65.0 does not drive, or whose V8 moves the allocation windows: a visible red, fixed by
  a Playwright bump (a dependency change with its security answer) or a reading-based retune. The job prints the
  version in every run. CHROME-SOURCE (c) is the pinned alternative.
- **GitHub's check may need a permission the job's token lacks** (PERM-ATT): then the first run stops at that step,
  the stop rule fires, and one line comes back to him.
- **Chrome inside the cut** has never been tried (`unshare --net`, no_new_privs): if it cannot start, the browser
  tests move to a job of their own without the cut, which is a new text for his yes.
- **A slower runner** can fail the allocation test's floor of 20 frames (27 to 30 measured here beside a second
  browser; 36 with one worker). The floor is not lowered blind: a red is read and brought to him.
- **Two checks, one truth:** the workflow writes the tool's values once and compares them, as text, with
  `wasm-bindgen-tool.mjs`; a change to either without the other fails before the download.
- **`toolchain` is not a required check** (REQUIRED-CHECK, an open follow-up for the quality gate step): until it
  is, a red `toolchain` does not block a merge by itself; this lane reads it by hand before every merge.

## 11. Change type and risk tier
CI and tests; no game rule, no page text, nothing a player sees. Risk tier 2 by `.github/workflows/toolchain.yml`.

## 12. Reviewers
The advisor agent (this plan; every pushed head). security-auditor (the CI text before it goes to him; the final
diff). infra-engineer and test-engineer (the final diff; each drafted a part, so each reads the other's part
first). The verifier until PASS. docs-keeper for section 7, item 4.

## 13. What Mustafa checks in his browser
Nothing: no build of the game changes. His part: the answers of section 16, the chat's mode, and the go.

## 14. Planned size
| Part | Added lines |
|---|---|
| `.github/workflows/toolchain.yml` | 90 |
| `build-checks.mjs`, `build.mjs`, `build-checks.test.mjs` | about 35 (9 of them moved) |
| `playwright.config.ts` and the three specs | about 15 |
| `packages/e2e/node-names.d.ts` | 15 |
| `.gitignore` | 1 |
| **Outside the docs** | **about 155; stop line 310** |
**The docs, counted apart** (his words copied by script and the quoted workflow diff do not count: his SIZE-0,
"copied lines do not count"): about 440 written lines, stop line 880. By file: the DECISIONS headers about 45; the
browser-check record's own text about 95 (of its 609 lines, about 510 are his quoted words); the report about 250;
STATUS (within its 120 lines), ROADMAP, the map and the archive about 50; the vision's note 4.
What would make it grow: PERM-ATT (b) +3; CHROME-SOURCE (c) +10 to +15 and a second ask; FIREFOX-TESTS (a) +6 to
+12 CI lines and a config edit; a fix after a stop.

## 15. Not in it
Firefox or WebKit in CI; a required-check change; a cache; a second job; `@types/node`; SA-TREE; the path remap for
RR-F5; any change to `gate.yml` or the `smoke` job; change-set 3 (its plan is drafted beside this: `cs3-context.md`,
`rust-reviewer-cs3-rng.md`).

## 16. Open questions for Mustafa (two), and the advisor agent's rulings on four more
1. **CHROME-SOURCE: which browser runs the browser tests in CI?** Why: the browser's version decides what the tests
   prove. Options: (a) the Google Chrome already in GitHub's runner image (157.0.8103.50 in today's image). It
   brings nothing in from the internet, so his rule on such code is untouched; what he accepts is a browser version
   that moves with GitHub's image, as gh, curl, tar and Node in this job already do, and that is printed in every
   run. (c) Playwright 1.65.0's Chrome for Testing 158.0.8127.18, fetched by curl and checked against a size and
   sha256 he approves after one run prints them: pinned bytes, about 170 MB per run, and a second text for his yes.
   Recommendation: (a). (Not offered, by the advisor agent's condition: Playwright's own installer, which would be
   the first package code to run with the network in this job and whose download nothing checks.)
2. **CI-TEXT: one yes for the CI text, covering five things.** Why: no CI file is edited without his yes on its
   exact text, and #60 put its stop rule and its plant to him too.
   - `ci-edit.diff`: 125 lines, 90 added, none removed, sha256
     282890b716f767d01e910a5a18dd2f454a9787abe4ecdee61e8521ce2df3290e.
   - `ci-plant-p-gh.diff`: the two lines of the planted change for GitHub's check, sha256
     fe2004ac3d0f5b39086a096c077d5b707fcddae29a2a9df8d554a4856fb290a1, written in one commit and taken out in the
     next.
   - The second planted change: one committed render count in
     `packages/e2e/tests/skeleton/render-counts.json` changed from 3 to 4, and taken back.
   - The stop rule of section 9.
   - That the one tool file he named now also comes by a second route: the runner's curl line, with the limits of
     the route he approved (https only, the size limit, size and sha256 before any unpacking).
   Options: (a) yes to all five as shown; (b) yes with changes he names; (c) no. Recommendation: (a).

**Ruled by the advisor agent** (plan review, `advisor-plan.md`; he can overrule any of them):
- **PERM-ATT: (a).** The job's token keeps `contents: read` only. If GitHub's check is refused for lack of a
  permission, the run stops there and the one line comes back to him.
- **PLANTS: both run.** P-BROWSER is evidence, the advisor agent's call; P-GH's lines are part of CI-TEXT.
- **FIREFOX-TESTS: (c).** Not in this pull request (he called it small; one new unknown per first run) and not
  inside change-set 3: a small pull request of its own after this one, with its download and its exact text for
  his yes. He is told, so that he can object: the archive's line of 13:36:37 UTC promised him this question with
  the CI text.
- **PYCACHE-NOW: (b).** The `.gitignore` line stays with change-set 3, where his words put it; the local ignore and
  `-B` guard until then.

## 17. Records due
His words of 14:28:25, 14:31:30 and 16:08:04 UTC and his browser reports (section 7); his answers to section 16
when they come (by script, never retyped); the refused commands and their routes, if any; and what went wrong,
with times from records. **The compiled Python file** (`.checkpoints/ci-wasm/__pycache__/rr-chacha-ref.cpython-313.pyc`,
7,356 bytes): made at 16:34 UTC (the file's own time, read by `ls -la` at 17:33:58 UTC); added by the saver's
autosave b7325df (committed 16:39:03 UTC, pushed 16:39:06 UTC by the saver's log); found at 17:33:58 UTC and removed
from the folder at 17:35:06 UTC (the clock lines of the main session's two commands); the removal committed by the
autosave 395a370 (17:39:12 UTC). It stays in the branch's history and enters main's history with the merge, so his
go can accept it, as his go on #61 accepted change-set 2's two. He is told in the same message as the questions.
