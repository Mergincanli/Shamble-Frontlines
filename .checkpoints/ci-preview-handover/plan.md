# Plan: the preview hand-over step, TOKEN-WORDS and FIREFOX-REPLAY (CI Builder), revision 1

Branch `claude/keen-tesla-v807cf` (this chat's one push branch; its last pull request, #71, is merged and main
2483e9a contains its head c9460ca, so a commit on main is a fast-forward there, no force). Worktree `.gate/ci4`, local
branch `cs/preview-handover` from main 2483e9a (#70 merged). Risk tier 2 (a `.github/` file). His word: the saved file
`mustafa-2026-11-26T1320-preview-handover.md` (1086 bytes, sha256 d7291959...0f18). The request: issue #59, comment
7172595904 of 2026-11-25, 19:39 UTC, posted by the `Mergincanli` account as lane 2 (Builder V2); information, not his
word.

## 1. Goal (three parts, all in his block)
a. One step at the end of job `toolchain`, after "Build and test with the network cut off", that hands the tested
   player build (`.gate/dist-production/`) to lane 2's preview workflow as the artifact `preview-site`, on pull
   requests from this repository only (lane 2's step 2.6, its plan revision 2).
b. TOKEN-WORDS: the comment "The only step with the token" (`toolchain.yml:94`) becomes "The only run step with the
   token" (checkout and setup-node also receive the job's token as their default input).
c. FIREFOX-REPLAY: the replay test (`packages/e2e/tests/replay/determinism.spec.ts`, from #70) joins the firefox
   entry beside the rule tests. If red in Firefox: stop and bring it to him before any change; a hash difference is a
   finding, not a flake.

## 2. Files (the fence) and exact changes
1. `.github/workflows/toolchain.yml` (his yes on exact text, mode read right before the write):
   - b: line 94, one word.
   - a: lane 2's proposed step, reviewed and tried, at the job's end, with:
     - the action `actions/upload-artifact`, pinned by full commit SHA. The request names
       cf430e030ddbb5b0abf93d22962f4752f3646cd9 for v7.0.2; read from the action's own repository today with
       `git ls-remote https://github.com/actions/upload-artifact 'refs/tags/v7*'`: v7.0.2 and v7 both at that commit (a
       lightweight tag). It is a new third-party action in this repository (gate.yml and toolchain.yml use only
       checkout, setup-node and cache), so by his opening rule 3 ("No other new package or tool") its source comes to
       him as its own question, with its address, version and pin (ask 2 carries it).
     - `if: github.event_name == 'pull_request' && github.event.pull_request.head.repo.full_name == github.repository`.
     - a guard step before it, proposed by me for security-auditor to weigh: the build folder is written by the
       repository's code inside the cut and read by the upload outside it; the upload follows symbolic links, so a
       planted link (to `/proc/self/environ`, which would hold the upload's runtime token, or to any runner file)
       would land in the artifact. The guard fails the job if `.gate/dist-production` holds anything but plain files
       and folders, or is empty, and prints the file count and total size (read-only).
     - `retention-days: 7`, `if-no-files-found: error`, `include-hidden-files: true` as proposed.
   - No other change: the workflow name `toolchain`, the file path and the artifact name stay (lane 2's note 1).
2. `packages/e2e/playwright.config.ts`: the firefox project's `testDir: "./tests/rules"` becomes `testDir: "./tests"`
   with `testMatch: ["rules/**", "replay/**"]`, and its comment names both. The workflow's Firefox line already runs
   what the project names, so no workflow change is needed for c.
3. Notes under `.checkpoints/ci-preview-handover/`; the docs pass (DECISIONS by script, STATUS, the archive's follow-ups:
   TOKEN-WORDS done, a block for this pull request; REPO_MAP rows; ROADMAP); the report
   `docs/reports/2026-11-26-ci-preview-handover.md`.
Nothing under `crates/`, no package source, no spec file, no kit or gate file, no `preview.yml` (lane 2's).

## 3. Asks to him
- Ask 1 (small, now, after this plan's APPROVE): FF-REFETCH: the Firefox trial needs Firefox again; the file he allowed
  under TRIAL-DOWNLOAD (a) was deleted at #71's merge as his rule said. May the chat fetch the same file again
  (`firefox_161.0+build1-0ubuntu0.24.04.1~mt1_amd64.deb`, 90,437,572 bytes, sha256
  bd304b0b181c4fed449279e25d309075c3a2e85345e63d1118c5085cefcf1ed1, the same route: https into a new `.gate/` folder,
  size and sha256 checked first, `dpkg-deb -x`, no apt, no root), deleted again at this merge? If the archive no longer
  serves that exact file, stop and tell him.
- Ask 2 (one message): the full diffs of both files with the workflow's sha256 after the write; UPLOAD-SOURCE (the
  action, its address, version and pin, as above); each line marked tried and how; the two reviewers' findings.

## 4. Trials (in the container)
- Firefox 161 (after FF-REFETCH): the firefox project with the replay test: list (rules 2 + replay 1 = 3), run. If the
  replay test is red: stop, read it, bring it to him with the lines and options (his point 2); no change.
- Chromium unchanged (16 + replay = 17 now; listed); the e2e type check (workflow line 191's command) with the new
  config; `npm test -w`'s equivalent through node for the `node` project.
- The upload step cannot run here (it needs GitHub's runtime): tried are the guard step's shell line (on the real
  production build, plus a planted symbolic link and a planted fifo that must fail it, and an empty folder that must
  fail it) and the `if:` expression by reading; YAML parse and `bash -n` of the inner script unchanged.
- The legacy gate (no smoke) after each step.
- On GitHub after the write (LONG-RUNS): the push run skips the upload (not a pull request); the pull request run,
  once his Advisor opens the draft, shows the artifact `preview-site` in the run's artifacts list. Red run rule (his
  words of 2026-11-25, 20:35 Amsterdam): a red his words send to him comes to him before any change.

## 5. Reviews, size, process
- The advisor agent on this plan, then on the final head (full SHA). security-auditor and infra-engineer read the
  tried texts together before ask 2 (the one round). At most 2 agents at a time, Opus 5.5, effort high.
- Size: about 25 workflow lines and 4 config lines (stop line 300).
- No checkpoint script (it pushes to the local branch name); notes committed by path, pushed with
  `git push origin cs/preview-handover:claude/keen-tesla-v807cf`, never force, remote head read after each push.
  The head sent for approval carries no `[skip ci]`; the PR text named at the notes' last commit.
- git and gh always start their own command (his interim rule).
- Board: the add-on's thread for this pull request; a question waits as a pushed notes file named in the post.

## 6. Risks
- The replay test's request routing (`page.route` serving a module Worker on a made-up origin) under Playwright's BiDi
  route in Firefox may not work; only the trial shows it (a cause in Playwright's route is still his question first).
- The `workflow_run` pattern: the preview workflow (lane 2's) consumes an artifact from a pull request's run; this
  side only uploads. What the preview workflow does with it is lane 2's and its reviewers'.
- The artifact is public to signed-in users of a public repository for 7 days: the production page only, which the
  preview publishes anyway.

## 7. What Mustafa checks
No browser check: nothing a player sees changes. He answers FF-REFETCH, then ask 2, then gives his go.
