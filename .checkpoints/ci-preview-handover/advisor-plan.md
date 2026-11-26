# Advisor agent: plan review of ci-preview-handover, revision 1 (eb62201), from main 2483e9a's advisor.md

## Review plan
1. Read his block (saved file, 1086 bytes), his opening block (DECISIONS 2026-11-24 22:50:34 UTC, rules 1-8), TRIAL-DOWNLOAD,
   RULES-ONLY-READ / TOKEN-WORDS (a), the request (issue #59 comment 7172595904, information only).
2. Read toolchain.yml, playwright.config.ts, determinism.spec.ts.
3. Check pins, push route, visibility, ancestry with git/gh myself.
4. Rule on the six questions; verdict.

## Checked myself
- Remote claude/keen-tesla-v807cf = eb62201 (this plan's commit); c9460ca is an ancestor of main 2483e9a; worktree branch cs/preview-handover.
- git ls-remote actions/upload-artifact: v7 and v7.0.2 -> cf430e030ddbb5b0abf93d22962f4752f3646cd9 (v7.0.1 043fb46, v7.0.0 bbbca2d). Matches the plan.
- Repository is public (REST: visibility public): fork pull requests can exist.
- TOKEN-WORDS text matches STATUS_ARCHIVE.md:1344-1346 ("the only run step with the token").
- Request comment: posted by Mergincanli 2026-11-25T19:39:29Z; proposal step text matches the plan's description.
- toolchain.yml: permissions contents: read; upload-artifact uses the runtime token, not the job token (no extra permission).
- firefox project today: testDir ./tests/rules; tests/rules holds one spec, tests/replay one spec.

## Findings (as found)
F1 (Medium) Risk 6 understates the workflow_run pattern: the `if:` is hygiene, not a boundary. The repository is public; a
   pull request's run uses the pull request's own toolchain.yml, so a fork PR can drop the `if:` and upload `preview-site`.
   preview.yml must itself check the run's event and head repository. Plan must say so, and carry it to lane 2 (#59) and to him.
F2 (Medium) The guard step (two steps, not "one step") is inside part a's purpose and the fence, but departs from "one step"
   in his block and lane 2's ask: show it in ask 2 as its own marked choice with and without, security-auditor's read with it.
   Its limits must be stated: a process left running from the cut step (same uid, same pid namespace) can change the folder
   after the check (time-of-check gap); the folder itself and .gate must be checked as non-links (find on a path without a
   trailing slash).
F3 (Medium) Rule 3 reads "No other new package or tool": a prohibition, not a procedure. UPLOAD-SOURCE must be its own
   question in ask 2 that names this conflict (his rule 3 vs his block's request), with options; the write waits for its own
   yes. Ask 2 is the right time (nothing of it is used before the write; nothing runs in the container).
F4 (Medium) Stop rule wording: name that a red, a time-out or a hash difference of the replay test in GitHub's Firefox
   (Firefox 160 on the last run; the container has 161) also stops before any change, and that a red replay is never re-run
   to look for a green.
F5 (Medium) Checkpoint departure: skipping checkpoint.mjs is acceptable (it pushes to the local name), but the resume card
   `.checkpoints/ci-preview-handover.md` is still owed (CLAUDE.md Checkpoints 1), updated after each step and at least every
   30 minutes, committed by path and pushed by the same route; the departure named in the report.
F6 (Low) testMatch ["rules/**","replay/**"] replaces the default spec pattern: any helper file later put there becomes a
   "test file". Use "rules/**/*.spec.ts", "replay/**/*.spec.ts".
F7 (Low) Trial runs without the EGL files (not part of FF-REFETCH); if Firefox needs them, that is a stop and a question.
F8 (Low) Point 3 of his block (the one-line answer) is not in the plan; it is a chat answer, not PR scope: note it answered.
F9 (Low) Docs: STATUS follow-up for step 2.3 keeping the name `toolchain`, the path and `preview-site` (lane 2's note 1);
   the artifact is the build of the pull request's merge ref, not its head (information for lane 2); job time vs the 15-minute
   limit read on the first run.

## Rulings
1. Fence: nothing beyond his parts except the guard step (F2): inside part a, but shown as its own choice. 
2. Upload-artifact: correctly a new tool; ask 2 right, as its own question naming rule 3 (F3).
3. FF-REFETCH: yes, ask 1 is right (his yes was for that job's trial; rule 3 "any new download ... before it is used").
4. Trials: sound; F4, F6, F7.
5. Push route: sound (fast-forward verified). Saver: F5.
6. Risks/evidence: F1, F9.
Verdict: REVISE (F1-F5 into the plan; F6-F9 may ride).

## Second pass (after the advisor tool's read)
- Main's advisor.md read from origin/main (335 lines): plan-review list and tier-2 checklist as applied above.
- Report 2026-11-25-ci-after-3a.md read: #71 kept a card (SAVER departure: card and notes committed by path, pushed
  with the same refspec). `ls .checkpoints/` here: only the folder ci-preview-handover/; no `ci-preview-handover.md` card. F5 stands.
- Lane 2's preview.yml read at 266f42e (branch claude/optimistic-albattani-mdoonz): job `check` requires
  workflow_run.name == 'toolchain', path == '.github/workflows/toolchain.yml', event == 'pull_request', conclusion
  'success', head_repository.full_name == github.repository; no secret in that job. So the boundary exists on lane 2's
  side. F1 becomes Low: the plan must say the `if:` is hygiene and the boundary is preview.yml's check (cite it), in its
  risks and in ask 2; posting on #59 is not needed. preview.yml also uses actions/upload-artifact at the same pin and
  download-artifact 9000827 (v8.0.2): the tool enters the repository through either lane.
- Could not read the action's text at the pin: gh api on actions/upload-artifact refused (repository not enabled for
  this session); I did not fetch its code. Condition on the builder (F11).
- FF-SPEC-COMMENTS check: determinism.spec.ts names no browser; no comment turns false by FIREFOX-REPLAY.

F10 (High) Steps after the cut for the first time. (i) Comments that turn false: toolchain.yml:4 "the last step runs in
   its own network namespace", :130 "the last step with the network"; the plan's "No other change" contradicts it. Fix
   them in the asked text as part of a, named. (ii) Code after the cut that the cut's user (same uid as the runner,
   setpriv --reuid=$(id -u)) can reach: the guard step's `bash` and `find` resolve through PATH, whose first folders
   that user can write (CUT-PATH, STATUS_ARCHIVE.md:1340-1342; CUT-PATH-N1 :1724); the upload action's code is fetched
   at "Set up job" into the runner's work folder, owned by the runner's user (inferred, not measured), so code in the cut
   could change it before it runs with the runtime token and the network. The same holds for checkout's post step today
   (pre-existing, inferred). Put both to security-auditor as explicit questions (absolute program paths and shell; no run
   step; what the upload's exposure means), in the plan's risks; a High from it goes to him before or in ask 2.
F11 (Medium) Upload trial: cite the action's own text at cf430e0 for (a) symbolic links followed, (b) whether `.gate/`
   counts as hidden, (c) no token input (decides whether "the only run step with the token" stays true and whether the
   upload is a token step); name the read route for GitHub's proof (the step log via gh; my call to an Actions path was
   refused by the proxy: confirm the route works before relying on it); compare the upload's printed file count with the
   guard's; a red upload step stops and goes to him.
F12 Ask 1 may go now (independent of the revisions). UPLOAD-SOURCE may ride in ask 1 (optional): an early no saves the round.

Verdict: REVISE. Blocking: F10, F2, F3, F4, F5, F11. Ride: F1, F6-F9. Ask 1 may go to him now.
