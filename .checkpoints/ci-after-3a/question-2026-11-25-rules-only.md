# Question for Mustafa, 2026-11-25: RULES-ONLY-READ and TOKEN-WORDS (before any merge of ci-after-3a)

The advisor agent's read of 80359fc: REVISE on the docs only (the workflow equals his approved text, CI green). Its two Highs are fixed in f7bd1f2: the red run's evidence stated truly, and these two answers placed before any merge. The question, from the report `docs/reports/2026-11-25-ci-after-3a.md`:

## Deviation: the rules-only change was made on the chat's reading
His point 6 of 2026-11-25, 19:00 Amsterdam: "if the Firefox drawing tests fail on the page's "no WebGL" line, the
firefox entry runs the rule tests only, by a change to the test configuration alone ... Any other red, a time-out
included, comes to me first." The first run's Firefox part showed the "no WebGL" line (file-url) and also Playwright's
"Timed out waiting 300s for the test suite to run": the clock and isolation tests waited 60 to 90 s each for blocks
from a page that had already stopped, and the suite hit its 5-minute stop with 9 tests not run. The container trial
had shown the same shape (the page's line on the tests that read its notice, time-outs on the rest;
`trials-main-311c048.md` and `.gate/trial/firefox-b.log`); the three time-outs show no line of their own, so their
cause is inferred (the Evidence above). The chat read the
time-out as a result of the "no WebGL" failure, not another red, and made the change without asking. His rule names a
time-out, so that reading is the chat's, not his; it goes to him as RULES-ONLY-READ below. The DECISIONS record says
the same (docs-keeper).
- Not planted in CI: no plant for the checks or the devtools line in CI (a plant there is another CI write).

## Reviews (Opus 5.5, effort high; one round on the tried texts, before ask 2)
- security-auditor: containment passes (nothing weakens the cut, the root-owned tool, the token or the no-cache rule).
  F1 Medium (the comment called Playwright's Firefox "the build players run"; Playwright's preference
  `security.fileuri.strict_origin_policy` false differs from a player's): fixed in the text he approved. F2 Low (the
  libEGL line could misreport a failed `ldconfig`): fixed. Notes: CUT-PATH evidence; TOKEN-WORDS deferred.
- infra-engineer: nothing beyond the WebGL question would fail the first run (it did not). F1 Medium (comments that
  would turn false under the rules-only change): fixed. F6 Medium (Firefox's 5-minute stop): his FF-STOP (a), kept.
  F2 to F5 Low: named in ask 2; F5 (Firefox's sandbox inside the cut) shows in the log as "Sandbox:
  CanCreateUserNamespace() unshare(CLONE_NEWPID): EPERM", and Firefox ran.
- The advisor agent: the plan (above); its read of the final head comes with the head's full id.

## Decisions needed
- **RULES-ONLY-READ: does he accept the rules-only change as made?** His point 6 sent "a time-out included" to him
  first, and the first run showed a time-out beside the "no WebGL" line (the deviation above). Options: (a) accept it
  as made, since the time-out followed from the same WebGL failure; (b) put the 15-test Firefox entry back (another
  run, red the same way) and decide again. Recommendation: (a).
- **TOKEN-WORDS: defer it again, or fold it in?** A Low security finding: the workflow's comment "The only step with
  the token" is true of run steps only (checkout and setup-node also get the token). It was due "at the next CI edit",
  which this was; the chat deferred it without his word. Options: (a) defer to the gate step 2.3; (b) fold it in now
  (a one-word CI text change: "the only run step with the token", its exact text and the file's new sha256 asked of
  him first, one more push and review). Recommendation: (a), to keep this pull request to his four parts.
- **FF-DRAW-WHEN: when does a graphics route for Firefox's drawing tests come back to him?** The runner's Firefox gets
  no WebGL, so the drawing tests run in Chromium only. Options: (a) with the quality gate step (2.3), as its own
  question with the exact install line or setting; (b) in the next CI pull request; (c) not until a WebGPU-capable
  runner exists. Recommendation: (a). Nothing is needed for this merge.

