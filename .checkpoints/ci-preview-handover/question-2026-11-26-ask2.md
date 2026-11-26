# Ask 2 for Mustafa, 2026-11-26: every text of the preview hand-over pull request in one message

Branch `claude/keen-tesla-v807cf`, worktree `.gate/ci4`, base main 2483e9a. Nothing below is written yet. The diffs
are in this folder; each rebuilt with `patch` against main's file gives the sha256 named.

## The texts
1. The workflow, recommended form (GUARD-STEP yes, HIDDEN-FILES no): `draft-toolchain.diff` (sha256
   e66b55753c629938d06a44cb7f4f98c9f857b583470dc7a81746e8330f39aefc; 33 lines added, 5 removed).
   `.github/workflows/toolchain.yml` today sha256 01898e4b3833812e945882106baa2bb3590866aff3c8d2dfa487a1fe38ce5742;
   after: d017b83b82d412befb1cbb983a93215440e2b0e1608d2c8c749445db53e87fd8 (security-auditor rebuilt the same bytes).
   - If GUARD-STEP no: `draft-toolchain-no-guard.diff` (sha256
     dd722183b2d8f95e7cc6d6e7701811d04a4424c4b03bfa26bdb06673d44fc1dc); the file after:
     ef66275bbe65a54675931d1c3ee4047f2ed060799ee4f2a36a2ea0657363ff2d (the comment block keeps its first sentence; the
     upload's comment ends "the build holds none today").
   - If HIDDEN-FILES yes (with the guard): `draft-toolchain-hidden-on.diff` (sha256
     2515034851b26dc34a246cd8dd70e987407a111fa4264059685384265e7e6605); the file after:
     8cabac930444c76fd379da1bac86e45ab937a3984656ae1196a7a87303a1588b (one line, `include-hidden-files: true`).
2. The test configuration: `draft-playwright-config.diff` (sha256
   51465471d508dc16ac04d870305c039b4479dc6823d9ba6f3a81da2544f82778; 6 lines added, 4 removed).
   `packages/e2e/playwright.config.ts` today ef2136a14096fd12d5b9cce7b7fe28d611279850412a3269e5868139b19c9b0e; after:
   35abfd54fddb43d7bf8077b281c153c8625c22d863d04922afd0b7b1ecfe80c8.

## The tie of v7.0.2 to the commit (security-auditor's own run of `git ls-remote https://github.com/actions/upload-artifact 'refs/tags/v7*'`, 2026-11-26)
    cf430e030ddbb5b0abf93d22962f4752f3646cd9	refs/tags/v7.0.2
A lightweight tag (no `^{}` line), `refs/tags/v7` at the same commit; the read-only copy of its text at that commit:
no token input (`action.yml:4-53`, `src/upload/input-helper.ts:8-58`), symbolic links followed (`src/shared/search.ts:16`),
broken links skipped (`:18`), dot names dropped unless asked (`:19`, tested on each item from the build folder down,
`dist/upload/index.js:130173`), the runner's upload key read from its environment (`dist/upload/index.js:82477`).

## security-auditor on the steps after the network cut (its notes: `security-auditor-ci.md`)
- Absolute paths in the guard: keep them; they close the PATH route (follow-up CUT-PATH) and not the others
  (`BASH_ENV`, `LD_PRELOAD` set through the runner's per-step files), which belong to the next point.
- No extra script step: also safe; a run step gets neither the upload key nor the job's token. The guard adds a check
  and a count, and changes no risk.
- The upload's own code can be changed from inside the cut: Medium, and it is the existing accepted risk CUT-ESCAPE
  (`docs/STATUS_ARCHIVE.md:803-818`), not new with this change: the cut's code runs as the runner's user and can reach
  what later steps run; checkout's and setup-node's clean-up steps hold the same upload key today. The fix is a
  follow-up, not this text: the cut as a throwaway user on its own copy of the workspace, with its processes ended with
  the step. A High would need a secret or write scope in this job, or something that runs or trusts the build's
  content, or a text claiming the steps after the cut are protected: none here. Lane 2's `preview.mjs` (its branch,
  9ded5a8) treats the build as data (no links followed, dot names and non-plain files refused, nothing run); the docs
  pass adds that sentence to CUT-ESCAPE's record (SA-2).
- Nothing here needs to stop the change.

## infra-engineer: APPROVE; nothing would fail the first run, with one thing it could not read
The repository's Actions permission setting (the proxy refused the read): if Settings allows only a chosen list of
actions without `actions/upload-artifact`, every run fails at set-up, push runs included. Its other Lows overlap
security-auditor's and are in the text (IE-1 to IE-3); IE-4 (a re-run of a pull request run with the same artifact
name, likely fine) is a follow-up; IE-5: compare file counts, not bytes.

## Each line, tried and how
- TOKEN-WORDS and the two comment fixes: read against the file; YAML parses (11 steps; 10 without the guard).
- The guard: extracted and run by `plant-test.sh` on the real production build (exit 0, "5 files, 892209 bytes") and
  on seven plants (a symbolic link, a fifo, an empty build, the build folder as a link, `.gate` as a link, a dot-named
  file, a dot-named folder): each exit 1 with its own line; `bash -n` clean; its programs root-owned here.
- The upload step: cannot run outside GitHub; its inputs read from the action's own text (above). Its proof on GitHub:
  the push run shows both new steps skipped and "Set up job" downloading the action at the pin; the pull request run
  (once the draft is open) prints the guard's count and the upload's "there will be 5 files uploaded", its digest and
  artifact id.
- The test configuration: Firefox 161 (his FF-REFETCH): 3 passed, the replay test "first-loop.rec: 1800 steps, every
  shared hash as in the file"; Chromium 17 passed (3.2 min); lists Chromium 17, Firefox 3, node 1; the e2e type check
  passes.

## Questions
- CI-WRITE-3: may the chat write the workflow and the test configuration as above (the recommended form unless he
  picks otherwise below)? (a) yes; (b) yes with changes he names; (c) no. Recommended (a).
- GUARD-STEP: the check before the upload, its own yes or no. It catches a link or dot name left in the build and
  prints a count to compare with the upload's; it is no boundary. Recommended: yes.
- HIDDEN-FILES: `include-hidden-files: true`, its own yes or no. With the guard it never decides anything (a dot name
  fails the guard first); without it, "off" keeps a stray `.env` or `.git` from ever leaving the runner. Recommended: no.
