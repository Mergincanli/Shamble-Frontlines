# docs-keeper (ci-preview-handover): old and new text of every corrected or moved line

## Moved by script (docs-keeper-moves.py, byte for byte; sha256 before -> after printed by the script)
- docs/STATUS.md 50269c58...aaeed -> 2260b284...bde18; docs/STATUS_ARCHIVE.md 98a25c48...35977 -> de65e66a...e264b;
  docs/ROADMAP.md d2a17314...5b7 -> 1e7dcf98...b10.
- STATUS line 25 (#70's In-flight row) -> STATUS_ARCHIVE line 265, under a new header at 261 ("In flight", before
  "## Targets"). Old text = new text (byte for byte).
- STATUS lines 104-105 (Follow-ups: "From guard-env-fix (#69)", "From ci-after-3a") -> STATUS_ARCHIVE lines 1867-1868,
  under a new header bullet at 1862-1866 (end of "Follow-ups"). Old text = new text.
- ROADMAP lines 389-398 (a blank line and the entry "Part 15 of the build plan, the second lane, to main") -> ROADMAP
  209-218, at the end of "Build history"'s paragraphs (the oldest "Recent work" entry folded). Old text = new text.

## Corrected by Edit (old -> new)

(The lines below are `git diff -U0 d12e3a8 -- <file>` for the four docs, so every old (-) and new (+) line
is kept as it stood; the moved lines appear as - in one place and + in the other.)

### docs/STATUS.md
```diff
--- a/docs/STATUS.md
+++ b/docs/STATUS.md
@@ -5 +5 @@ next, waiting on Mustafa, the routines' state, pointers. His decisions, word for
-_Last updated: 2026-11-25 (change-set 4a's docs pass, with 3b's owed one, on main 07d673b: #70 in flight; no change-set 3c (his ORDER-3C (c)); the merged #71's row and waiting bullet and 3a's follow-ups bullet moved to the archive by script; lane 2's lines left as they are)._
+_Last updated: 2026-11-26 (ci-preview-handover's docs pass, by the CI Builder's docs-keeper, on main 2483e9a: the preview hand-over in flight; the merged #70's row and the follow-ups bullets of the merged #69 and #71 moved to the archive by script; in lane 1's lines only #70's merge and FIREFOX-REPLAY's answer; lane 2's lines left as they are)._
@@ -13 +13 @@ _Last updated: 2026-11-25 (change-set 4a's docs pass, with 3b's owed one, on mai
-- **Last merged** (2026-11-25, read with git): #71, ci-after-3a (07d673b); #67 (c1da1bd); #69 (311c048); #68, 3b (3ab9eae).
+- **Last merged** (read with git, 2026-11-26): #70, 4a (2483e9a, 2026-11-26); #71, ci-after-3a (07d673b); #67 (c1da1bd); #69 (311c048); #68, 3b (3ab9eae).
@@ -19 +19 @@ _Last updated: 2026-11-25 (change-set 4a's docs pass, with 3b's owed one, on mai
-  07d673b (read 2026-11-25, 22:12 UTC). Approved unattended work: [Night queue, issue #16](https://github.com/Mergincanli/Shamble-Frontlines/issues/16).
+  2483e9a (runs 45818982040 and 45818982052, read with gh's REST route 2026-11-26). Approved unattended work: [Night queue, issue #16](https://github.com/Mergincanli/Shamble-Frontlines/issues/16).
@@ -25 +25 @@ _Last updated: 2026-11-25 (change-set 4a's docs pass, with 3b's owed one, on mai
-| #70 | change-set 4a, "The recording" (the first of Walk's four parts, his WALK-CUT (a)), by lane 1 (the chat he calls Builder V4): a recorded match replays to the same hashes, tick by tick, natively (one process, two processes, two matches in turn) and in WebAssembly on the players' file (Node and Chromium), and each planted divergence is caught at its tick with the field that differs; the one new number on the players' file is the shared state's hash (`Sim::shared_hash`), read only by a replay | `claude/cool-pasteur-oiqk1r` (stands in for `cs/4a-recording`) | risk tier 1; no browser check (nothing a player sees changes); five reviewers in two rounds (netcode-architect, rust-reviewer, test-engineer, engine-architect, security-auditor), all APPROVE in round 2, nothing Critical, High or Medium open; source final at fb5d0ff; the local checks, CI (on 21c1046, six of six) and the verifier (PASS) on it; 3,300 written lines against the stop line of 3,400 (3,050 before his STOP-4A (a), 2026-11-25, 20:48:11 UTC); report `docs/reports/2026-11-25-cs4a-recording.md`; ready for review once the advisor agent has approved its last head | the advisor agent's review of the last head; his Advisor's merge check; his go for that head |
+| none yet (title and text in the notes) | ci-preview-handover, by the chat he calls CI Builder (his block of 2026-11-26, 13:20 Amsterdam; lane 2's request on issue #59): at the end of the `toolchain` job, on pull requests from this repository only, a check that the tested player build holds only plain files and folders with no name that begins with a dot, then the hand-over of that build to lane 2's preview workflow as the artifact `preview-site`, kept 7 days (`actions/upload-artifact` v7.0.2 at cf430e0, his UPLOAD-SOURCE (a)); TOKEN-WORDS and two comments made true; the recording's replay test joins the rule tests in Firefox (FIREFOX-REPLAY) | `claude/keen-tesla-v807cf` (stands in for `cs/preview-handover`) | risk tier 2; no browser check; written at d12e3a8 on his CI-WRITE-3 (a), GUARD-STEP (a) and HIDDEN-FILES (b) (2026-11-26, 15:05 Amsterdam), both files at his sha256 values; one review round before his yes (security-auditor: nothing stops the change, its SA-1 Medium is the accepted CUT-ESCAPE; infra-engineer: APPROVE); in the container Firefox 161 3 passed (the replay test included), Chromium 17 passed, the check on the real build and on seven planted faults; the upload step has never run, and no preview starts until lane 2's `preview.yml` is on main; CI: to be read; report `docs/reports/2026-11-26-ci-preview-handover.md` | his Advisor opens the draft and watches the first run; the advisor agent's review of the final head; his go for that head |
@@ -30 +30 @@ _Last updated: 2026-11-25 (change-set 4a's docs pass, with 3b's owed one, on mai
-2. v2: the netcode proof test (#40); step 2.3's chat stays stopped. 3b (#68) and the small CI pull request (#71) merged.
+2. v2: the netcode proof test (#40); step 2.3's chat stays stopped. 3b (#68), the small CI pull request (#71) and 4a (#70) merged.
@@ -41 +41,2 @@ _Last updated: 2026-11-25 (change-set 4a's docs pass, with 3b's owed one, on mai
-- #70 (4a): his go for the head the advisor agent approves, after his Advisor's merge check (no browser check). From
+- ci-preview-handover (CI Builder): his go for the head the advisor agent approves, after his Advisor has opened the draft and watched the first run (CI: to be read); a red on the new steps or on Firefox's replay test comes to him first; if the run fails at its start because the action is not allowed, the chat tells him which setting to change (his 15:05 points 5 and 7).
+- #70 (4a), merged 2026-11-26 (2483e9a). From
@@ -43 +44 @@ _Last updated: 2026-11-25 (change-set 4a's docs pass, with 3b's owed one, on mai
-  DEPART-4; FIREFOX-REPLAY (it widens the Firefox entry he cut to the rule tests); LINKS-CHECK (4A-S3-READ). STEP-60
+  DEPART-4; FIREFOX-REPLAY (answered by his block of 2026-11-26, 13:20 Amsterdam; carried by ci-preview-handover); LINKS-CHECK (4A-S3-READ). STEP-60
@@ -91 +92 @@ _Last updated: 2026-11-25 (change-set 4a's docs pass, with 3b's owed one, on mai
-- **From #60, #61, #64 and change-set 3a (#65)** (moved by script, OLD-LINES): the archive's "Moved from STATUS (2026-11-25...)".
+- **From #60, #61, #64, change-set 3a (#65), guard-env-fix (#69) and ci-after-3a (#71)** (moved by script, OLD-LINES): the archive's "Moved from STATUS (2026-11-25...)" and "(2026-11-26, ci-preview-handover's docs pass)".
@@ -100 +101 @@ _Last updated: 2026-11-25 (change-set 4a's docs pass, with 3b's owed one, on mai
-  talks to a real server); for 8 N4, C2; Slice 3 C3; the gate step 4A-S3-READ; his word first 4A-FIREFOX-REPLAY; 4A-HASH-REVISIT.
+  talks to a real server); for 8 N4, C2; Slice 3 C3; the gate step 4A-S3-READ; 4A-FIREFOX-REPLAY (his block of 2026-11-26, 13:20 Amsterdam; done on ci-preview-handover's branch); 4A-HASH-REVISIT.
@@ -104,2 +105 @@ _Last updated: 2026-11-25 (change-set 4a's docs pass, with 3b's owed one, on mai
-- **From guard-env-fix (#69):** GEF-SAME (the same kind of gap in other wrappers and keywords the day guard strips; the forms are with Mustafa, kept out of the repository), SA-R1-3 (the wrapper loop's turn cap), SA-R2-2 (very long lines against the hook timeout), V-2 (the folder tracking after a split-string value), the reason-text nit.
-- **From ci-after-3a** (the small CI pull request; the archive's block "From ci-after-3a"; each closed at its merge): done on its branch E1, 3A-GEN-CHECK, 3A-RULES-STEP's CI half (its target-gating rule stays open), the Firefox half of FIREFOX and 3A-OTHER-BROWSERS in CI on Linux (the rule tests only; WebKit and Safari open); still open 3B-OTHER-BROWSERS (its mark is a drawing test); new: FF-DRAW (his question first), FF-SPEC-COMMENTS, FF-FILEURL-PREF, FF-STOP, FF-SANDBOX, FF-ORDER; CUT-PATH's new evidence; TOKEN-WORDS deferred again.
+- **From ci-preview-handover** (the archive's block "From ci-preview-handover"; the ids are that docs pass's, not his): done on its branch, closed when it merges, TOKEN-WORDS and 4A-FIREFOX-REPLAY (his FIREFOX-REPLAY); CUT-ESCAPE stays on record as accepted (his 15:05 point 6) and its later fix is CUT-ESCAPE-FIX ("the cut's code as its own throwaway user"); PH-SA-2 (CUT-ESCAPE's acceptance now also rests on lane 2's preview treating the build as data); PH-IE-4 (a re-run and the artifact's name, read on the first re-run); for the gate step 2.3 PH-KEEP-NAMES; for lane 2 PH-MERGE-REF; on the first run PH-TIME and PH-ACTIONS-SETTING; at the merge PH-DELETE (the action's read-only copy and the Firefox folder).
@@ -112 +112 @@ _Last updated: 2026-11-25 (change-set 4a's docs pass, with 3b's owed one, on mai
-  attempts and was cancelled in 1 (a runner DNS outage). #64's browser tests use a Chrome that moves with the image.
+  attempts and was cancelled in 1 (a runner DNS outage). #64's browser tests use a Chrome that moves with the image. ci-preview-handover's upload runs after the cut on what the cut wrote (CUT-ESCAPE, accepted; PH-SA-2), and its artifact is readable by signed-in users for 7 days.
```

### docs/STATUS_ARCHIVE.md
```diff
--- a/docs/STATUS_ARCHIVE.md
+++ b/docs/STATUS_ARCHIVE.md
@@ -260,0 +261,6 @@ _And STATUS's "Waiting on Mustafa" bullet for ci-after-3a (#71), moved by the sa
+_Moved from STATUS (2026-11-26, ci-preview-handover's docs pass, under his OLD-LINES of 2026-11-18): STATUS's row for the merged change-set 4a (PR #70, 2483e9a, merged 2026-11-26 12:22:01 UTC by `merged_at`, read with gh's REST route), moved by script, byte for byte, so its cells read as they stood when it left STATUS:_
+
+| PR | Work | Branch | State | Waiting on |
+|---|---|---|---|---|
+| #70 | change-set 4a, "The recording" (the first of Walk's four parts, his WALK-CUT (a)), by lane 1 (the chat he calls Builder V4): a recorded match replays to the same hashes, tick by tick, natively (one process, two processes, two matches in turn) and in WebAssembly on the players' file (Node and Chromium), and each planted divergence is caught at its tick with the field that differs; the one new number on the players' file is the shared state's hash (`Sim::shared_hash`), read only by a replay | `claude/cool-pasteur-oiqk1r` (stands in for `cs/4a-recording`) | risk tier 1; no browser check (nothing a player sees changes); five reviewers in two rounds (netcode-architect, rust-reviewer, test-engineer, engine-architect, security-auditor), all APPROVE in round 2, nothing Critical, High or Medium open; source final at fb5d0ff; the local checks, CI (on 21c1046, six of six) and the verifier (PASS) on it; 3,300 written lines against the stop line of 3,400 (3,050 before his STOP-4A (a), 2026-11-25, 20:48:11 UTC); report `docs/reports/2026-11-25-cs4a-recording.md`; ready for review once the advisor agent has approved its last head | the advisor agent's review of the last head; his Advisor's merge check; his go for that head |
+
@@ -818,0 +825,6 @@ _And STATUS's "Waiting on Mustafa" bullet for ci-after-3a (#71), moved by the sa
+      _Dated pointer, 2026-11-26 (ci-preview-handover's docs pass; his words of 2026-11-26, 15:05 Amsterdam, point 6,
+      `docs/DECISIONS.md`: "CUT-ESCAPE stays on record as accepted."): on that pull request's branch a step after the
+      cut uploads the build the cut's code wrote, for lane 2's preview workflow, which publishes it with a Cloudflare
+      token (security-auditor's read on lane 2's branch at 9ded5a8); the acceptance now also rests on that workflow
+      treating the build as data (PH-SA-2), and the later fix is
+      CUT-ESCAPE-FIX (both in "From ci-preview-handover")._
@@ -1278,0 +1291,3 @@ _And STATUS's "Waiting on Mustafa" bullet for ci-after-3a (#71), moved by the sa
+      _Dated pointer, 2026-11-26 (ci-preview-handover's docs pass): on that pull request's branch, closed when it
+      merges, the `firefox` project also runs the recording's replay test (`tests/replay/`), his FIREFOX-REPLAY
+      (FIREFOX-REPLAY in "From ci-preview-handover")._
@@ -1348,0 +1364,3 @@ _And STATUS's "Waiting on Mustafa" bullet for ci-after-3a (#71), moved by the sa
+    _Dated pointer, 2026-11-26 (ci-preview-handover's docs pass): **done** on that pull request's branch, closed when
+    it merges: `.github/workflows/toolchain.yml:94` reads "The only run step with the token" [verified 2026-11-26 at
+    d12e3a8]._
@@ -1373,0 +1392,2 @@ _And STATUS's "Waiting on Mustafa" bullet for ci-after-3a (#71), moved by the sa
+    _Dated pointer, 2026-11-26: the replay test joins them on ci-preview-handover's branch, closed when it merges
+    (FIREFOX-REPLAY in "From ci-preview-handover")._
@@ -1489,0 +1510,3 @@ _And STATUS's "Waiting on Mustafa" bullet for ci-after-3a (#71), moved by the sa
+    _Dated pointer, 2026-11-26 (ci-preview-handover's docs pass): on that pull request's branch, closed when it
+    merges, the `firefox` project also runs the recording's replay test (FIREFOX-REPLAY in "From
+    ci-preview-handover")._
@@ -1733,0 +1757,2 @@ _And STATUS's "Waiting on Mustafa" bullet for ci-after-3a (#71), moved by the sa
+    _Dated pointer, 2026-11-26 (ci-preview-handover's docs pass): **done** there, on that pull request's branch, closed
+    when it merges (`toolchain.yml:94`, "The only run step with the token" [verified 2026-11-26 at d12e3a8])._
@@ -1797,0 +1823,3 @@ _And STATUS's "Waiting on Mustafa" bullet for ci-after-3a (#71), moved by the sa
+    _Dated pointer, 2026-11-26 (ci-preview-handover's docs pass): his word came as FIREFOX-REPLAY in his block of
+    2026-11-26, 13:20 Amsterdam (`docs/DECISIONS.md`); **done** on that pull request's branch, closed when it merges
+    (FIREFOX-REPLAY in "From ci-preview-handover")._
@@ -1803,0 +1832,64 @@ _And STATUS's "Waiting on Mustafa" bullet for ci-after-3a (#71), moved by the sa
+- **From ci-preview-handover, the preview hand-over step** (the chat he calls CI Builder,
+  session_019YNMUUemXge56777MiUWFz, on his block of 2026-11-26, 13:20 Amsterdam; lane 2's request: issue #59, its
+  comment of 2026-11-25, 19:39 UTC; no pull request number yet, its title and text in the chat's notes; local branch
+  `cs/preview-handover` pushed to `claude/keen-tesla-v807cf`, code head d12e3a8; risk tier 2; the advisor agent's plan
+  review `advisor-plan.md` and one review round by security-auditor and infra-engineer on the tried texts before his
+  yes, `security-auditor-ci.md` and `infra-engineer-ci.md`, all in `.checkpoints/ci-preview-handover/`, readable at
+  d12e3a8 until the notes are deleted; report `docs/reports/2026-11-26-ci-preview-handover.md`; CI on d12e3a8: to be
+  read; the ids are this docs pass's, the reviewers' own ids in brackets, not his):
+  - The hand-over (his UPLOAD-SOURCE (a), GUARD-STEP (a) and HIDDEN-FILES (b)): at the end of job `toolchain`, on pull
+    requests from this repository only, a check with absolute program paths that `.gate/dist-production` holds only
+    plain files and folders, none named with a leading dot, and is not empty (it prints the file count and size), then
+    `actions/upload-artifact` at cf430e030ddbb5b0abf93d22962f4752f3646cd9 (v7.0.2) uploads it as `preview-site`, kept 7
+    days, an empty upload an error, hidden files off (`.github/workflows/toolchain.yml:215-243` [verified 2026-11-26 at
+    d12e3a8]). In the container the check passed on the real production build (5 files, 892,209 bytes) and failed on
+    each of seven planted faults (`trials.md`, `question-2026-11-26-ask2.md`). The upload step has never run; its proof
+    is the pull request run's log: its file count against the check's (counts, not bytes: infra-engineer's IE-5), the
+    digest and the artifact id. CI: to be read. No consumer yet: lane 2's `preview.yml` is not on main, and a
+    `workflow_run` starts only from main's copy, so no preview starts until it is (infra-engineer; "Open bugs and
+    risks", the ci-preview-handover entry).
+  - TOKEN-WORDS (the #64 block above): **done** on the branch, closed when it merges (`toolchain.yml:94`), with two
+    comments the new steps would have made false (`:4`, `:130`) [verified 2026-11-26 at d12e3a8].
+  - FIREFOX-REPLAY (his block of 2026-11-26, point 2; 4A-FIREFOX-REPLAY in the 4a block above): **done** on the
+    branch, closed when it merges: the `firefox` project's `testDir` is `./tests` with `testMatch`
+    `["rules/**/*.spec.ts", "replay/**/*.spec.ts"]` (`packages/e2e/playwright.config.ts:38-40` [verified 2026-11-26 at
+    d12e3a8]). In the container's Firefox 161.0, 3 passed, the replay test "first-loop.rec: 1800 steps, every shared
+    hash as in the file" (`trials.md`); in the runner image's Firefox: CI: to be read. A red, a time-out or a hash
+    difference there comes to him before any change and is never re-run for a green (his point 2; plan R2-4).
+  - CUT-ESCAPE-FIX (his words of 2026-11-26, 15:05 Amsterdam, point 6: "CUT-ESCAPE stays on record as accepted. Put
+    its later fix (the cut's code as its own throwaway user) into STATUS as a follow-up."; when: not named by him;
+    CUT-ESCAPE's own home is step 2.3, by his go for #48): the cut's code as its own throwaway user.
+    security-auditor's fuller form of the fix (its SA-1, Medium, CUT-ESCAPE itself, not new with this change): its own
+    uid on its own copy of the workspace, the output copied out by root and checked, and `unshare --pid --fork` so that
+    no process of the cut outlives the step. Why: the cut's code keeps the runner's uid and can change what later steps
+    run (the action's code in the work folder and the step's `GITHUB_ENV` and `GITHUB_PATH` files, inferred from the
+    runner's layout; `BASH_ENV` set that way reaches the check's absolute shell, shown locally), and the upload holds
+    the runner's upload key with the network, as checkout's and setup-node's post steps already do (from the runner's
+    source, read through a fetched summary: inferred).
+  - PH-SA-2 (security-auditor's SA-2, Low): CUT-ESCAPE's acceptance rested on "contents: read and no secrets"; after
+    this change a workflow holding the Cloudflare token publishes what the cut wrote, so the acceptance now also rests
+    on lane 2's preview treating the build as data. security-auditor's read at lane 2's 9ded5a8 (its branch, not main;
+    [assumed] here, not read by docs-keeper): `hosting/preview.mjs` walks with lstat (no link followed), refuses dot
+    names, names outside a safe set, host-acted files and non-plain items, caps size and count, needs `index.html`,
+    and never runs the build; `preview.yml`'s job `check` needs the run's workflow name `toolchain`, its path, the
+    event `pull_request`, success and a run from this repository. The sentence is on CUT-ESCAPE's record above.
+  - PH-IE-4 (infra-engineer's IE-4, Low, likely benign): the upload keeps `overwrite` false; if artifact names are
+    scoped to the run, not the attempt, a re-run of a pull request run whose first attempt uploaded `preview-site`
+    fails the step, and lane 2's runbook prescribes that re-run. GitHub's docs are silent on it; the action's own client
+    expects several artifacts of one name in a run (consistent with names per attempt, not shown). Read it on the first
+    re-run; `overwrite: true` would be a CI text change (his yes first).
+  - PH-KEEP-NAMES (for the gate step 2.3; lane 2's note 1 in its request; plan R2-10): step 2.3 replaces
+    `toolchain.yml`; keep the workflow name `toolchain`, the path `.github/workflows/toolchain.yml` and the artifact
+    name `preview-site`, or change lane 2's preview with them: its `check` job matches the name and the path, and
+    `preview.mjs` finds the hand-over on main by the line `name: preview-site` (infra-engineer's read at lane 2's
+    dc00c79).
+  - PH-MERGE-REF (for lane 2; the advisor agent's F9, plan R2-10): a pull request run builds the pull request merged
+    into main, so `preview-site` is the build of that merge, not of the pull request's head commit [assumed: from
+    the plan and the review, not read in GitHub's docs by docs-keeper].
+  - PH-TIME (plan R2-10): the job's time on the first run, against its 15-minute limit (`toolchain.yml:38`); main's
+    last run took 6.0 minutes (infra-engineer). CI: to be read.
+  - PH-ACTIONS-SETTING (his 15:05 point 5; infra-engineer's one unknown): if the repository's Actions settings allow
+    only chosen actions without `actions/upload-artifact`, every run fails at its start; the first run on the draft is
+    the test, and such a failure goes to him with the setting to change.
+  - PH-DELETE (at this pull request's merge; his FF-REFETCH (a), the advisor agent's N2): the Firefox 161 folder under
+    `.gate/` and the read-only copy of the action's code in the scratchpad (a download not used) are deleted.
@@ -1855,0 +1948,7 @@ _And STATUS's "Waiting on Mustafa" bullet for ci-after-3a (#71), moved by the sa
+- **Moved from STATUS (2026-11-26, ci-preview-handover's docs pass)** (under his OLD-LINES: lines about merged pull
+  requests, moved by script, byte for byte): STATUS's Follow-ups bullets for guard-env-fix (PR #69, merged 2026-11-25
+  as 311c048) and ci-after-3a (PR #71, merged 2026-11-25 as 07d673b) are the two bullets below. The second one's
+  pointer leads to the block "From ci-after-3a" above; its "TOKEN-WORDS deferred again" is done on ci-preview-handover's
+  branch (the block "From ci-preview-handover" above).
+- **From guard-env-fix (#69):** GEF-SAME (the same kind of gap in other wrappers and keywords the day guard strips; the forms are with Mustafa, kept out of the repository), SA-R1-3 (the wrapper loop's turn cap), SA-R2-2 (very long lines against the hook timeout), V-2 (the folder tracking after a split-string value), the reason-text nit.
+- **From ci-after-3a** (the small CI pull request; the archive's block "From ci-after-3a"; each closed at its merge): done on its branch E1, 3A-GEN-CHECK, 3A-RULES-STEP's CI half (its target-gating rule stays open), the Firefox half of FIREFOX and 3A-OTHER-BROWSERS in CI on Linux (the rule tests only; WebKit and Safari open); still open 3B-OTHER-BROWSERS (its mark is a drawing test); new: FF-DRAW (his question first), FF-SPEC-COMMENTS, FF-FILEURL-PREF, FF-STOP, FF-SANDBOX, FF-ORDER; CUT-PATH's new evidence; TOKEN-WORDS deferred again.
@@ -2161,0 +2261,11 @@ _From STATUS's "Routines", moved here on 2026-11-23 by docs-keeper's script, byt
+- **ci-preview-handover, the preview hand-over step** (2026-11-26, no pull request number yet; the evidence: `plan.md`,
+  section 6, and `security-auditor-ci.md` in `.checkpoints/ci-preview-handover/`; the follow-ups: the block "From
+  ci-preview-handover" in "Follow-ups"):
+  1. The upload runs after the cut on the build the cut's code wrote: CUT-ESCAPE, accepted (his words of 2026-11-26,
+     15:05 Amsterdam, point 6), its acceptance now also resting on lane 2's preview treating the build as data
+     (PH-SA-2); its fix is CUT-ESCAPE-FIX.
+  2. The artifact `preview-site` can be downloaded by any signed-in GitHub user for 7 days: the production page only,
+     which the preview publishes anyway (the plan's risks; security-auditor: no finding).
+  3. No consumer yet: lane 2's `.github/workflows/preview.yml` is on its own branch, not in this branch's
+     `.github/workflows/` (gate.yml and toolchain.yml only [verified 2026-11-26]); a `workflow_run` starts only from
+     main's copy, so no preview starts until lane 2's file is on main (infra-engineer).
@@ -2255 +2365,2 @@ _From STATUS's "Routines", moved here on 2026-11-23 by docs-keeper's script, byt
-| CI on main, latest (the rows below are older) | `gate`, `smoke` and `toolchain` success on 07d673b, PR #71's merge commit (push; gate and smoke completed 20:49 UTC, toolchain 20:54 UTC) | 2026-11-25 | GitHub's REST API, the commit's check runs (read 22:12 UTC) |
+| CI on main, latest (the rows below are older) | `gate`, `smoke` and `toolchain` success on 2483e9a, PR #70's merge commit (push; gate run 45818982040 with its gate and smoke jobs, completed 12:23 UTC; toolchain run 45818982052, 12:28 UTC) | 2026-11-26 | GitHub's REST API, the commit's workflow runs and the gate run's jobs (read by ci-preview-handover's docs-keeper) |
+| CI on main (2026-11-25) | `gate`, `smoke` and `toolchain` success on 07d673b, PR #71's merge commit (push; gate and smoke completed 20:49 UTC, toolchain 20:54 UTC) | 2026-11-25 | GitHub's REST API, the commit's check runs (read 22:12 UTC) |
```

### docs/REPO_MAP.md
```diff
--- a/docs/REPO_MAP.md
+++ b/docs/REPO_MAP.md
@@ -91 +91 @@ engine files, and `START_HERE.md` explains how they work and why. docs-keeper ke
-| `packages/e2e/playwright.config.ts` | Three projects: `node` (the browser-free tests in `tests/node/`, what the package's `test` script and CI run), `chromium` (the browser tests in the rest of `tests/` on a built page that `SHAMBLE_DIST` names, one worker, a 90 s limit per test; CI runs it since PR #64, his CI-WASM (a), in the runner image's Google Chrome) and `firefox` (since ci-after-3a: `tests/rules/` only, one worker, a 90 s limit per test, in Mozilla's own Firefox through Playwright's `moz-firefox` channel over WebDriver BiDi, the program `SHAMBLE_FIREFOX` names; CI runs it in the runner image's Mozilla Firefox, his FIREFOX-SOURCE (c); the drawing tests wait for a graphics route, FF-DRAW). Its report goes to `packages/e2e/playwright-report/`, ignored by git |
+| `packages/e2e/playwright.config.ts` | Three projects: `node` (the browser-free tests in `tests/node/`, what the package's `test` script and CI run), `chromium` (the browser tests in the rest of `tests/` on a built page that `SHAMBLE_DIST` names, one worker, a 90 s limit per test; CI runs it since PR #64, his CI-WASM (a), in the runner image's Google Chrome) and `firefox` (since ci-after-3a: the rule tests in `tests/rules/`; since ci-preview-handover also the recording's replay test in `tests/replay/`, his FIREFOX-REPLAY, matched as `rules/**/*.spec.ts` and `replay/**/*.spec.ts` under `tests/` [verified 2026-11-26]; one worker, a 90 s limit per test, in Mozilla's own Firefox through Playwright's `moz-firefox` channel over WebDriver BiDi, the program `SHAMBLE_FIREFOX` names; CI runs it in the runner image's Mozilla Firefox, his FIREFOX-SOURCE (c); the drawing tests wait for a graphics route, FF-DRAW). Its report goes to `packages/e2e/playwright-report/`, ignored by git |
@@ -95 +95 @@ engine files, and `START_HERE.md` explains how they work and why. docs-keeper ke
-| `packages/e2e/tests/replay/determinism.spec.ts` | The recorded matches in the browser's own WebAssembly engine (change-set 4a; PM-0004-19): in a Worker made for this spec alone, every recording replays on the players' file to its hashes file's shared column; the file under test is first shown to be byte for byte the built page's, and the hashes file's digest is checked against the recording. Runs in the `chromium` project |
+| `packages/e2e/tests/replay/determinism.spec.ts` | The recorded matches in the browser's own WebAssembly engine (change-set 4a; PM-0004-19): in a Worker made for this spec alone, every recording replays on the players' file to its hashes file's shared column; the file under test is first shown to be byte for byte the built page's, and the hashes file's digest is checked against the recording. Runs in the `chromium` project and, since ci-preview-handover, the `firefox` project (his FIREFOX-REPLAY) [verified 2026-11-26] |
@@ -171,0 +172 @@ engine files, and `START_HERE.md` explains how they work and why. docs-keeper ke
+| `docs/reports/2026-11-26-ci-preview-handover.md` | The report of the preview hand-over pull request (the chat he calls CI Builder): the check and the upload that hand the tested player build to lane 2's preview workflow as `preview-site`, TOKEN-WORDS, the recording's replay test in Firefox (FIREFOX-REPLAY); the plan and its approval, his answers, the trials, the two reviews, what changed and what is not verified |
@@ -244 +245 @@ engine files, and `START_HERE.md` explains how they work and why. docs-keeper ke
-| `.github/workflows/toolchain.yml` | Job `toolchain` (foundation step 2.2) on every push and pull request, with no cache restored or saved (`cache-mode: none`, PR #60): installs the pinned Rust toolchain with rustup, checks `rustc` against the pin; then (PR #64) fetches the wasm-bindgen tool's archive (https only; its size and sha256 against Mustafa's TOOL-SOURCE values, which the job's `env` holds and compares line for line with `packages/client/scripts/wasm-bindgen-tool.mjs`), runs GitHub's own check on it (`gh release verify-asset`, the only run step given the job's read-only token) and installs its one program root-owned at `/wasm-bindgen-0.2.130/wasm-bindgen` (the archive's member list checked; the program and every folder above it up to `/` root-owned and closed to others; its size and sha256 checked); runs `cargo fetch --locked` and `npm ci` (the last steps with the network), then builds and tests with the network cut off (its own network namespace; a TCP probe must fail inside): `cargo test --workspace --locked`; the generated types written again (`SHAMBLE_WRITE_TYPES=1`) and compared with git, failing on any change in `packages/types/src/generated`; every crate's release tests (all six crates since ci-after-3a, E1; before it `sim` and `protocol`), the two vector generators' `--check` (`python3 -I -B`, writing nothing; since ci-after-3a, 3A-GEN-CHECK), the `wasm32-unknown-unknown` build of five crates and, on a line of its own, of `devtools` (`--lib`; since ci-after-3a, 3A-RULES-STEP's CI half), `npm ci --offline`, `npm test --workspaces`; then `packages/types` type-checked with strict flags (`tsc --ignoreConfig`: the shipped files, then the shipped files and their tests with `--skipLibCheck`); then (PR #64) the page built twice with `build.mjs` on that tool (`SHAMBLE_WASM_BINDGEN`: the production page, then the development page the browser tests need, into `.gate/dist-*`), the Wasm's sha256 and the builder paths it carries printed (RR-F5: printed, not judged), the boundary tests on the built Wasm, five more type checks (the Worker's and the page's shipped files with the lib check, then with their tests; then the browser specs and `playwright.config.ts` with `packages/e2e/node-names.d.ts`), and the browser tests (Playwright's `chromium` project, one worker, no retry, stopped after 5 minutes) in the runner image's Google Chrome (`/opt/google/chrome/chrome`); then (since ci-after-3a) a read-only line that prints whether the runner has `libEGL.so.1`, a check that `/usr/lib/firefox/firefox` is a plain file, its version, and the `firefox` project's tests (the rule tests only) with the same limits in the runner image's Mozilla Firefox (his FIREFOX-SOURCE (c)); nothing is downloaded for either browser. CI pins no test count: a skipped or removed test keeps it green. Step 2.3 replaces it with the gate v2's workflow |
+| `.github/workflows/toolchain.yml` | Job `toolchain` (foundation step 2.2) on every push and pull request, with no cache restored or saved (`cache-mode: none`, PR #60): installs the pinned Rust toolchain with rustup, checks `rustc` against the pin; then (PR #64) fetches the wasm-bindgen tool's archive (https only; its size and sha256 against Mustafa's TOOL-SOURCE values, which the job's `env` holds and compares line for line with `packages/client/scripts/wasm-bindgen-tool.mjs`), runs GitHub's own check on it (`gh release verify-asset`, the only run step given the job's read-only token) and installs its one program root-owned at `/wasm-bindgen-0.2.130/wasm-bindgen` (the archive's member list checked; the program and every folder above it up to `/` root-owned and closed to others; its size and sha256 checked); runs `cargo fetch --locked` and `npm ci` (the last steps with the network before the cut), then builds and tests with the network cut off (its own network namespace; a TCP probe must fail inside): `cargo test --workspace --locked`; the generated types written again (`SHAMBLE_WRITE_TYPES=1`) and compared with git, failing on any change in `packages/types/src/generated`; every crate's release tests (all six crates since ci-after-3a, E1; before it `sim` and `protocol`), the two vector generators' `--check` (`python3 -I -B`, writing nothing; since ci-after-3a, 3A-GEN-CHECK), the `wasm32-unknown-unknown` build of five crates and, on a line of its own, of `devtools` (`--lib`; since ci-after-3a, 3A-RULES-STEP's CI half), `npm ci --offline`, `npm test --workspaces`; then `packages/types` type-checked with strict flags (`tsc --ignoreConfig`: the shipped files, then the shipped files and their tests with `--skipLibCheck`); then (PR #64) the page built twice with `build.mjs` on that tool (`SHAMBLE_WASM_BINDGEN`: the production page, then the development page the browser tests need, into `.gate/dist-*`), the Wasm's sha256 and the builder paths it carries printed (RR-F5: printed, not judged), the boundary tests on the built Wasm, five more type checks (the Worker's and the page's shipped files with the lib check, then with their tests; then the browser specs and `playwright.config.ts` with `packages/e2e/node-names.d.ts`), and the browser tests (Playwright's `chromium` project, one worker, no retry, stopped after 5 minutes) in the runner image's Google Chrome (`/opt/google/chrome/chrome`); then (since ci-after-3a) a read-only line that prints whether the runner has `libEGL.so.1`, a check that `/usr/lib/firefox/firefox` is a plain file, its version, and the `firefox` project's tests (the rule tests and, since ci-preview-handover, the replay test) with the same limits in the runner image's Mozilla Firefox (his FIREFOX-SOURCE (c)); nothing is downloaded for either browser; then (since ci-preview-handover), after the cut and on pull requests from this repository only, a check with absolute program paths that `.gate/dist-production` holds only plain files and folders, none named with a leading dot, and is not empty, and the upload of that build as the artifact `preview-site`, kept 7 days, for lane 2's preview workflow, by `actions/upload-artifact` pinned at cf430e0 (v7.0.2, his UPLOAD-SOURCE (a)) [verified 2026-11-26 at d12e3a8]; the action uses the runner's own upload key, not the job's token (security-auditor's read of the action's code, `security-auditor-ci.md` in the chat's notes) [assumed]; no preview starts until lane 2's `preview.yml` is on main. CI pins no test count: a skipped or removed test keeps it green. Step 2.3 replaces it with the gate v2's workflow |
```

### docs/ROADMAP.md
```diff
--- a/docs/ROADMAP.md
+++ b/docs/ROADMAP.md
@@ -209,0 +210,10 @@ stands as written (a fail with today's rifle prototype, not waived) and the chec
+**Part 15 of the build plan, the second lane, to main [2026-11-17 to 11-18, MERGED]** — part-15-second-lane, by
+his go of 2026-11-17 for #56 (docs only): Part 15 as he read it, with his six answers on top (all (a), with additions),
+in `docs/reports/2026-11-17-build-plan-slice-1-part-15.md`; his two messages of 2026-11-17 in `docs/DECISIONS.md` by
+script; dated pointers in ADR 0002 (a seventh package, VFX-ROUTE) and in the build plan report; the rules' line on gun
+files (GUN-FILES); STATUS, ROADMAP, the archive and REPO_MAP after #56's merge. The verifier: PASS. Report
+`docs/reports/2026-11-17-part-15-second-lane.md`. Merged: PR #57, 22c6afa, 2026-11-18 11:09:31 UTC, by merge commit,
+on his go after the advisor agent's APPROVE of f26ac5b; CI green on main (gate run 866 with smoke, toolchain run 165).
+With the merge he answered OLD-LINES, who may fold old lines in the shared record files (`docs/DECISIONS.md`,
+2026-11-18, 11:08:25 UTC).
+
@@ -259,0 +270,9 @@ stands as written (a fail with today's rifle prototype, not waived) and the chec
+**The preview hand-over step [2026-11-26, in flight]** — ci-preview-handover, by the chat he calls CI Builder, on his
+block of 2026-11-26 (risk tier 2; CI and the test configuration only): at the end of the `toolchain` job, on pull
+requests from this repository only, a check that the tested player build holds only plain files and folders with no
+dot names, then its upload as `preview-site` (7 days) for lane 2's preview workflow, by `actions/upload-artifact`
+v7.0.2 pinned by commit (his UPLOAD-SOURCE (a), GUARD-STEP (a), HIDDEN-FILES (b)); TOKEN-WORDS; the recording's replay
+test joins the rule tests in Firefox (FIREFOX-REPLAY: 3 passed in the container's Firefox 161). CUT-ESCAPE stays
+accepted, its fix a follow-up (his words). Written at d12e3a8 on his CI-WRITE-3 (a); CI: to be read. Report
+`docs/reports/2026-11-26-ci-preview-handover.md`; waits for the advisor agent's review of the final head and his go.
+
@@ -271,0 +291,3 @@ Report `docs/reports/2026-11-25-cs4a-recording.md`. PR #70; it merges on his go
+_Pointer (2026-11-26, ci-preview-handover's docs pass): merged as PR #70, 2483e9a, 2026-11-26 12:22:01 UTC
+(`merged_at`), by merge commit, with the head 420b16b; gate, smoke and toolchain green on main (runs 45818982040 and
+45818982052)._
@@ -280,0 +303,3 @@ drawing tests are a follow-up (FF-DRAW). Green on code head 832268e (Chromium 16
+_Pointer (2026-11-26, ci-preview-handover's docs pass): merged as PR #71, 07d673b, 2026-11-25 20:49:09 UTC
+(`merged_at`), by merge commit, by his tap with the head c9460ca pinned (his words of 2026-11-25, 21:50 Amsterdam,
+`docs/DECISIONS.md`)._
@@ -389,10 +413,0 @@ hold and the two CI edits (PR #60), the "Lane requests" issue (#59), no watch (`
-
-**Part 15 of the build plan, the second lane, to main [2026-11-17 to 11-18, MERGED]** — part-15-second-lane, by
-his go of 2026-11-17 for #56 (docs only): Part 15 as he read it, with his six answers on top (all (a), with additions),
-in `docs/reports/2026-11-17-build-plan-slice-1-part-15.md`; his two messages of 2026-11-17 in `docs/DECISIONS.md` by
-script; dated pointers in ADR 0002 (a seventh package, VFX-ROUTE) and in the build plan report; the rules' line on gun
-files (GUN-FILES); STATUS, ROADMAP, the archive and REPO_MAP after #56's merge. The verifier: PASS. Report
-`docs/reports/2026-11-17-part-15-second-lane.md`. Merged: PR #57, 22c6afa, 2026-11-18 11:09:31 UTC, by merge commit,
-on his go after the advisor agent's APPROVE of f26ac5b; CI green on main (gate run 866 with smoke, toolchain run 165).
-With the merge he answered OLD-LINES, who may fold old lines in the shared record files (`docs/DECISIONS.md`,
-2026-11-18, 11:08:25 UTC).
```
