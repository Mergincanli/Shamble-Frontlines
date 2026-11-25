# infra-engineer: CI review of the draft texts (ci-after-3a), 2026-11-25

Head f876c4b (main 311c048 merged in). Read-only in `.gate/ci3`. One review round (with security-auditor in parallel).

## Plan
1. Read plan.md (rev 2), trials-main-311c048.md, his answers (FIREFOX-SOURCE (c), FIREFOX-GL (a)), both drafts, the
   current toolchain.yml. (done)
2. Shell correctness of the new lines inside the single-quoted `bash -euo pipefail -c '...'`: quoting, apostrophes,
   `ldconfig | grep || echo` under pipefail/-e, `[ -f ] && [ ! -L ] || { }` precedence, PATH for ldconfig in `env -i`.
3. Firefox path: the runner image's install-firefox.sh (actions/runner-images), PPA pinning vs the snap transitional
   package; the layout in the trial deb (`usr/lib/firefox/firefox` plain file).
4. Playwright 1.65.0 `moz-firefox`: coreBundle.js channel table (~34982) and launch dispatch (~47743); executablePath
   handling; headless args; profile location; env passed; what fails without DISPLAY / in a netns.
5. Firefox inside `unshare --net` + `setpriv --no-new-privs` + `env -i`: HOME ownership, TMPDIR, sandbox (user
   namespaces, AppArmor on 24.04), loopback for BiDi, network-off startup.
6. Job time vs 15 min; cost per run (push + pull_request both trigger); a failing Firefox run's cost.
7. Release test line (every workspace member?), generator checks (-I -B, Python 3.12 compatibility, writes nothing),
   devtools wasm32 line (placement, `--lib`).
8. Header comment and echo text accuracy.
9. Verdict and ranked findings with exact replacement text.

## Findings (written as found)
Rebuilt the drafted file from the diff: sha256 35c8d11b...3d3 (equals trials-main-311c048.md); today's file 56d37fc6...811.

- F1 Medium (draft-toolchain.diff:8-10 and :34). The header and the new echo say "the browser tests" run in Firefox. His
  FIREFOX-GL item 2 (mustafa-2026-11-25T1455-firefox-gl.md) already decides that on the expected WebGL failure the
  Firefox entry in playwright.config.ts shrinks to the rule tests only. Then both texts in toolchain.yml become false,
  and fixing them is a CI-file change: another ask, plus another sha256 under plan.md section 5. His item 2 does not
  cover that. Fix: word them so they stay true in both outcomes ("the tests the firefox project names"). Also, "the
  test-only module's own wasm32 build" claims more than the line proves (plan 2.1d: build.mjs builds the module the
  tests load, not this line). Replacement texts in the summary.
- F2 Low (draft :35). If ldconfig is not on PATH, the fallback still says "libEGL.so.1: not on this runner". Tested
  with three PATHs under -euo pipefail: the script goes on in every case (right), but the text mixes up a missing tool
  with a missing library. The runner's PATH has /usr/sbin, and stderr would show "command not found", so the reach is
  small. Fix the text: "not in the loader cache".
- F3 Low (no text change). The Firefox run happens only when the Chromium run passes (set -e). The first GitHub run is
  the drawing tests' trial, and red runs are never re-run into green. So a red Chromium run there gives no Firefox
  evidence. Chromium was green on main's last 5 toolchain runs. Accept this and name it in ask 2.
- F4 Low (time and cost, no change). Main today (run 45662377271, c1da1bd): 4 min 58 s (31 s before the cut, 4 min 26 s
  in the cut step, Chromium about 3.4 min of that). Added: Firefox up to its 5-minute stop (the expected WebGL path
  uses all of it: trial "2 passed (5.0m)"), release builds of 4 more crates (about 0.5 to 1.5 min on the runner, not
  measured), about 10 s more. Estimate about 10.5 to 11.5 min. Worst case, with Chromium also at its stop: about 13
  min, 2 min under the 15-minute limit. Playwright's teardown shares the run's deadline (playwright/lib/runner/
  index.js:6413-6469), so the trial's "Timed out waiting 300s for the teardown" line is not a second 300 s. Cost: the
  repository is public (gh api visibility), so hosted Linux minutes cost $0. push and pull_request both run on a PR
  branch (separate concurrency groups, by ref): about 22 runner-minutes per push. A failing Firefox run costs its 5
  minutes. Follow-up if the drawing tests stay and the suite grows: raise timeout-minutes or lower Firefox's stop
  (his call).
- F5 Low (residual risk, no change). In CI, Firefox runs as the runner user with no_new_privs inside the network
  namespace. The trial ran it as root (uid 0) with no namespace. Its content sandbox wants unprivileged user
  namespaces. The image's configure-environment.sh (read 2026-11-25) sets no userns sysctl, and I could not check
  Ubuntu 24.04's AppArmor profile for /usr/lib/firefox/firefox here. Firefox usually weakens the sandbox rather than
  failing; not verified, medium-low confidence. A sandbox or launch error instead of the WebGL line counts as "any
  other cause" under FIREFOX-GL item 2: back to him, and no MOZ_* sandbox switch without his word.

Checked and right:
- Quoting: 0 apostrophes in the inner script; bash -n passes.
- `[ -f ] && [ ! -L ] || {}` is `(A && B) || C`: tested plain file (passes), link, folder and missing (each exits 1).
- The ldconfig line sits in a || list, so -e and pipefail do not end the script. grep -F without -q reads all its
  input, so there is no SIGPIPE.
- The path: install-firefox.sh (main, read 2026-11-25) runs `apt-get install --target-release 'o=LP-PPA-mozillateam'
  -y firefox`, then appends to /usr/lib/firefox/browser/defaults/preferences/syspref.js under bash -e, so the image
  build needs that layout. syspref only sets intl.locale.requested. In the trial deb, usr/lib/firefox/firefox is a
  plain ELF file and usr/bin/firefox is a link to firefox.sh, so checking the /usr/lib path is correct.
- Playwright 1.65.0: coreBundle.js:47743-47746 sends moz-* to _bidiFirefox. :34982-34986 has the default
  /snap/bin/firefox, which executablePath overrides (:41593-41601: a custom executablePath skips the registry lookup, so the
  channel path at :35167 is never consulted). :45740-45752 passes --remote-debugging-port=0, --headless and --profile <temp>, so no DISPLAY is
  needed. :45690-45703 needs an absolute HOME (HOME=/home/runner, owned by the setpriv uid), sets MOZ_CRASHREPORTER*
  and removes SNAP_*. Readiness comes from the "WebDriver BiDi listening on ws://" line, which needs loopback (the cut
  brings it up).
- Release line: the workspace members are crates/*, exactly these six crates. No Rust test reads .gate/ output, so the
  trial's warm worktree changes nothing.
- Generators under Python 3.12.3 (this container's /usr/bin/python3.12): both --check exit 0 (0.16 s, 1.99 s). git
  status is unchanged. They parse with ast feature_version=(3,12), and every stdlib call they make exists in 3.8 or
  older.
- devtools: crate-type is cdylib+rlib, so --lib builds both. It sits right after the five-crate wasm32 build, before
  npm.
- Config: the channel survives the fixture (the trial ran 2 rule tests through it). The workflow's line-191 type check
  passes. No lint or line-length rule applies.

- F6 Medium (draft :38-40, the Firefox run's `--global-timeout=300000`; no text change recommended for run 1). Even with
  WebGL working, Firefox has little room under its 5-minute stop. The Chromium trial (.gate/trial/chromium-b.log): the
  16 tests add up to 199.5 s; without the allocation spec (12.6 s), 187 s, about 3.1 of the 5 minutes. Two tests take
  2.4 minutes by themselves: the one-minute local match (1.1 m) and 10,000 presses (1.3 m). If Firefox runs the presses
  test about 1.6 times slower, or starts each test noticeably slower, it reaches the stop and the run goes red with
  "Timed out waiting 300s ... did not run" and no WebGL line. That is a red with a cause other than WebGL, and the stop
  lives in toolchain.yml (another ask). Options: (a) keep 300 s for the first run; the red would be easy to tell apart
  (tests passing, no "could not start drawing" line) and goes back to him under FIREFOX-GL item 2. (b) Raise Firefox's
  stop to 420 s now: the realistic job is about 13 min, but with Chromium also at its stop, about 15 min, at the job
  limit. (c) (b) plus timeout-minutes 20. My lean is (a), for the first run only; it is his call. Ask 2 should name it.

The first run is readable: the trial's failure detail shows the page's line ("This page could not start drawing:
... this.gl is null", firefox-b.log:40) even after the global stop, with the list reporter and trace off.

F5's signature in the log: Firefox's launch fails at the fixture (`browserType.launch` with "Browser logs", no
"WebDriver BiDi listening" line), so the 2 rule tests fail too. If the 2 rule tests pass, Firefox launched fine and F5
did not happen.

## Replacement texts (applied to a scratch copy and tried: YAML parses, 0 apostrophes inside the single-quoted script,
## bash -n passes, header width 119 = the file's widest; the file after them: sha256 4f3f451f...82ef6)
R1 (F1), toolchain.yml after the draft, lines 10-12 (draft-toolchain.diff:8-10), replaced by four lines:
# run in the runner image's Google Chrome, then the tests the firefox project of playwright.config.ts names run in its
# Mozilla Firefox (Mustafa's FIREFOX-SOURCE (c); Playwright drives that Firefox over WebDriver BiDi): nothing is
# downloaded for them. Inside the cut the job also runs every crate's release tests, the two vector generators' checks
# and a wasm32 build of the test-only crate (devtools) on its own line.
R2 (F1), line 205 (draft :34):
              echo "== the tests the firefox project names, in the runner image Mozilla Firefox, the same limits (never the allocation spec)"
R3 (F2), line 206 (draft :35):
              ldconfig -p | grep -F "libEGL.so.1 " || echo "libEGL.so.1: not in the loader cache (printed, not judged)"
Any replacement accepted changes both recorded sha256 values (the diff's 6784c2d5... and the file's 35c8d11b...). The
main session recomputes them and runs the tries again (YAML, bash -n, the libEGL line) before ask 2. playwright.config.ts
needs no change.

## Verdict: REVISE (text only: R1 to R3 before ask 2)
Nothing found makes the first GitHub run fail for a reason other than the known WebGL question. The quoting, the file
check, the Firefox path, Playwright's moz-firefox launch with executablePath, the release line, the generator checks
under 3.12 and the devtools line all hold. The non-WebGL candidates that remain are F6 (the healthy path at the
5-minute stop; his call, lean (a)) and F5 (the sandbox in the namespace; low likelihood; its signature is above).
