# Results log: the preview hosting check (ADR 0006 spike 1, SPIKE_PLAN entry 6)
Kept while the check runs (plan Part 2, "The results log"); folded into the report at the end. Every time is UTC
unless marked; every time about this chat's own process is a transcript time. Written 2026-11-17, before the setup
pull request opens (plan Part 3 B; consultation 3).

## Setup header (PM-0006-1)
To be checked again before the 24-hour reading.

**The test project (Cloudflare Pages).**
- Name `shamble-preview-check`; production link https://shamble-preview-check.pages.dev (his A4 answer, 2026-11-15
  20:42:20). Made by his drag-and-drop upload of a placeholder page (plan A4), so production's first version is his.
- His Cloudflare account holds nothing besides the domain shamblefrontlines.com and this project (his words,
  2026-11-15 20:42:20: "this is the first time that I've been using cloudflare").
- The token: an Account API token `shamble-preview-check-deploy`, "Pages Write" only (the form: "Developer Platform"
  1/62, "Pages", "Edit" ticked, "Read" unticked), scope "Entire Account", client IP filtering empty, token expiration
  "Nov 27 GMT+1" as chosen in the form (his screenshot, 2026-11-17 20:13:53). No screen after saving shows the end
  date; the "Account API tokens" list (20:29:51) shows the token, "Pages Write", "Last used" empty, created 2 minutes
  earlier. The token is meant to end ten days after it was made (his SA-R3 acceptance, 2026-11-16) and to be deleted
  by him on Day 2; Day 2 stays on or before 2026-11-26 (Amsterdam).
  Correction, 2026-11-18: the check measured the token's window as not_before 2026-11-26T23:00:00Z, expires_on
  2026-11-27T22:59:59Z (the token works only on 27 November); "Day 2 by the 26th" is withdrawn until the window is
  measured again after his fix.
  Update, 2026-11-18 (the second check, 14:27 UTC): the token holds "Pages Read" and "Pages Write" (Read added by him on
  2026-11-17, TOKEN-READ (a)); its window, measured: not_before null, expires_on 2026-11-25T22:59:59Z; the Cloudflare
  account is renamed "Shamble Frontlines" (his screenshot, 2026-11-18). Day 2's last day: 2026-11-25 (Amsterdam).

**The test repository (GitHub).**
- Mergincanli/shamble-preview-check, public; `main` 4a67b83b9ed672c81e1a6e3067f916841c02b4a9 (his README).
- The reviewed commit of `setup`: 1619164900553ee3e32339447b30865347711156 (the advisor agent's APPROVE, 2026-11-16,
  hand-back 09:02:47). After the merge, `main`'s merge commit is added here; it carries the same files and is what every
  later run uses. Added 2026-11-18: `main`'s merge commit 2162db55d93f59cf2ed7e371a3915409f0b5a587 (pull request #1
  merged by him at 14:57:11 UTC).
- The workflows at that commit, sha256 prefixes (equal to his yes of 2026-11-15 and 2026-11-16): deploy.yml
  29593af18d34, close.yml 3afdf389d636, probe.yml f24ad6e81daf, browsers.yml 6791076916ae. The two token-test variants
  (placed only in sitting 3): fd3dc4c6029b and 8b745631a278.
- Pins: actions/checkout v7.0.1 3d3c42e5aac5ba805825da76410c181273ba90b1, actions/setup-node v7.0.0
  820762786026740c76f36085b0efc47a31fe5020, actions/upload-artifact v7.0.2 cf430e030ddbb5b0abf93d22962f4752f3646cd9,
  actions/download-artifact v8.0.2 9000827ccba6bdab643e8b6fd33ac0654aef8333; Node 22 (setup-node); Wrangler 4.156.0;
  WebdriverIO 10.1.0 (tools/wdio/package-lock.json: 163 packages, all with integrity; installed with
  `npm ci --ignore-scripts`).
- Settings, from his screenshots (2026-11-16 11:34:31 and 12:34:49; 2026-11-17 19:17:53 and 20:29:51):
  - `preview`: required reviewer Mergincanli; "Prevent self-review" unticked; "Wait timer" unticked; "Allow
    administrators to bypass configured protection rules" unticked (found ticked on 2026-11-16, his fix on 2026-11-17);
    "Deployment branches and tags": "No restriction"; secrets CLOUDFLARE_API_TOKEN (pasted again 2026-11-17, "1 minute
    ago" at 20:29:51) and CLOUDFLARE_ACCOUNT_ID ("2 days ago" in that screenshot; from Cloudflare's Account Details
    and pasted again on 2026-11-17 by his words of 20:35:32, not by a screenshot).
  - `production`: no reviewer; "Wait timer" unticked; administrator bypass unticked; "Selected branches and tags",
    one rule, branch `main`; secret CANARY.
  - "Repository secrets": "This repository has no secrets."
  - Actions, General: "Allow all actions and reusable workflows"; "Require actions to be pinned to a full-length commit
    SHA" ticked; retention 90 days; "Require approval for all external contributors"; workflow permissions "Read
    repository contents and packages permissions"; "Allow GitHub Actions to create and approve pull requests"
    unticked.
- The Claude app (SR-READ, his screenshot of 2026-11-15 20:53:10): "Read access to Dependabot alerts, administration,
  commit statuses, merge queues, and metadata"; "Read and write access to actions, checks, code, discussions, issues,
  pull requests, repository creation, repository hooks, and workflows"; "All repositories". No write on administration
  or deployments.

**The build-marker rule.** Every build writes the commit it was built from into the page (`<meta name="build-marker">`
and the visible marker) and into manifest.json: `--marker "$MARKER"` with `MARKER: ${{ github.sha }}` (deploy.yml,
build job). For a pull request that is the merge commit GitHub makes for the run, not the branch's head; each record
names its marker, and the check scripts compare the served marker with the record's.

**The machines.** GitHub's hosted runners stand in for the draft's machines (MACHINES (a)): ubuntu-24.04 for every job
but the browser rows; windows-2025 (Chrome, Edge, Firefox) and macos-26 (Safari) for browsers.yml. Each run's image name
and version is read from its log (and recorded by run.mjs as `runner.image` and `runner.image_version`) and written next
to that run below. Browser versions: recorded per row by the browser run (to be added when it runs). Brave: his own,
on his PC (Day 2, steps 5 to 8).

**The rehearsal.** The setup pull request's two runs (its deploy, then its merge's cleanup) are a rehearsal outside the
marks (plan Part 3 B): kept and reported, counted toward no mark. The marks start with sitting 1.

## Lines carried in for later readings
- F-C (the advisor agent, 2026-11-16): the lockfile's edgedriver 8.0.0 and geckodriver 8.0.0 declare install scripts;
  `npm ci --ignore-scripts` skips their driver downloads, and run.mjs passes the runner's drivers through
  CHROMEWEBDRIVER, EDGEWEBDRIVER and GECKOWEBDRIVER. That this is harmless is inferred until the browser run: a missing
  driver shows as "no reading", never as a pass.
- L3 (round 2): index.html and 404.html are not fetched by check-headers, since Pages answers their .html paths with a
  308 (finding F1/TE-1); their headers are read through `/` and a missing path.
- L5 (round 2): a verify of an earlier run reads a branch link that has had more pushes since as stale: an expected red.
- Procedures (round 1): each close log's "Uploads found" against the records (F3, TE-7); each SIZES line and the
  tombstone's size (TE-11); the page's resource list against the manifest (TE-12); on bite-coep, Chromium refuses the
  worker's script, the observed form of "false" (TE-3); a non-zero second visit is root-caused (TE-13).
- SA-R8 (round 1): at sitting 3, the empty "Repository secrets" list is recorded again.
- After the setup pull request's deploy, Cloudflare's "Last used" for the token should show a time (a cross-check).

## Runs, readings and approvals
### The rehearsal: the setup pull request (outside the marks)
- Pull request https://github.com/Mergincanli/shamble-preview-check/pull/1 ("Setup: the test site and the throwaway
  workflows"), opened by this session at 2026-11-17 20:41 UTC (the run below was created 20:41:39); head `setup`
  1619164900553ee3e32339447b30865347711156, base `main` 4a67b83.
- Run 1: `deploy` #1, id 44171977539, event pull_request, actor and triggering actor Mergincanli, created 20:41:39
  (https://github.com/Mergincanli/shamble-preview-check/actions/runs/44171977539). Job `plan` (id 134353201815,
  runner label ubuntu-24.04) started 20:41:41, success at 20:41:49. Job `build (pr, preview, setup, preview, 1)` (id
  134353302216) 20:41:51 to 20:42:07, success. Job `deploy (pr, preview, setup, preview, 1)` (id 134353491166):
  "waiting" on `preview` from 20:42:07 (read 2026-11-17 with the GitHub MCP tool). His list sent after that reading.
- Approval: Mergincanli approved `preview` (the run page's "Deployment protection rules", his screenshot sent
  20:50:58: "approved 1 minute ago"); the deploy job's log starts 20:49:39. Runner image ubuntu-24.04 version
  20261111.127.1 (Ubuntu 24.04.5 LTS). The build marker (`github.sha`, the merge commit): 64dcf68505f503df3f32ab258ca4363f240088ed.
- The deploy job failed (exit 1) at its first call to Cloudflare. Wrangler 4.156.0 (installed: "added 38 packages in
  5s"; Node v22.23.4): "A request to the Cloudflare API (/accounts/***/pages/projects/shamble-preview-check) failed.
  Authentication error [code: 10000]", then, on its own follow-up lookup, "A request to the Cloudflare API (/accounts)
  failed. Invalid access token [code: 9109]". No record was written ("No files were found with the provided path:
  records/"). The verify job then exited 3 (no records: "no reading"). Run status: failure, 8 min 28 s; artifacts 2
  (one `headers-44171977539`, 311 bytes). Reading: the value in CLOUDFLARE_API_TOKEN (or the account ID beside it) is
  not what Cloudflare expects; which of the two, and why, is not known from the log (both values are masked).
  A rehearsal finding (plan Part 3 B: a fix found here is not a re-run of a failed mark).
- Run data (read 2026-11-17 after the run): the deploy job was created 20:42:07 ("waiting") and started 20:49:39, so
  his approval came between those times (the approvals endpoint is not among this session's tools); steps: download
  20:49:42-45, "Refuse server code in the upload" passed, "Install Wrangler (this step holds no secret)" 20:49:46-55,
  "Upload to Cloudflare Pages" 20:49:55-57 failure; job done 20:49:58. Verify 20:50:00-07; its artifact
  `headers-44171977539` (id 14369645074, digest sha256:0073aac72dce..., equal to the downloaded zip) holds
  `{"checked_at": "2026-11-17T20:50:04.604Z", ..., "records": [], "summary": {"records": 0, "pass": 0, "fail": 0,
  "no_reading": 0}}` (saved here before its deletion for the re-run).
- The fix (consultation 10), by his screenshots and words (user message 21:09:07, 53e93cb6-2a07-4ea3-b488-0ee4dc515901): before any change,
  `preview`'s secrets showed CLOUDFLARE_ACCOUNT_ID "26 minutes ago" and CLOUDFLARE_API_TOKEN "34 minutes ago" (so the
  token box was not overwritten by the account ID), and Cloudflare's list showed the token with "Pages Write", "Last
  used" empty, created and modified "36m ago". Then (his words) the token rolled and pasted, the account ID pasted from
  Account Details, `headers-44171977539` deleted; after: both secrets "now" ("Environment "preview" updated.").
- Attempt 2 (his "Re-run failed jobs"): deploy job id 134368200375, created 21:06:06, started 21:06:47 (his approval
  between), "Upload to Cloudflare Pages" 21:06:56-58 failure with the same two errors (Authentication error [code:
  10000] on /accounts/***/pages/projects/shamble-preview-check; Invalid access token [code: 9109] on /accounts; his
  screenshot of the log). Verify job id 134368776685, 21:07:03-11, failure; its artifact upload succeeded (21:07:07-09).
  By the stop rule told to him: no third paste; the cause is looked for elsewhere.
- Attempt 2's verify artifact `headers-44171977539` (id 14369802505, digest sha256:9a38c73cece4..., equal to the
  downloaded zip): `{"checked_at": "2026-11-17T21:07:07.678Z", ..., "records": [], "summary": {"records": 0, "pass": 0,
  "fail": 0, "no_reading": 0}}`. The same name uploaded again in attempt 2 after attempt 1's copy was deleted.
- His words (22:12:28, ab54f723-7d7b-438b-bef8-02ccbfe60f9b): "I have also added read to the pages, and rerun the failed tasks, and its still
  not working." (TOKEN-READ (a), done by him; no screenshot of the token's page.) Run data (get_workflow_run:
  run_attempt 5, conclusion failure): attempt 3, deploy job 134390099209 created 21:41:38, started 21:42:05, upload
  failed 21:42:18-20; attempt 4, deploy job 134400842206 created 21:59:30, cancelled 21:59:45 (never started);
  attempt 5, deploy job 134402686180 created 22:02:34, started 22:08:53, upload failed 22:09:13-15, run with debug
  logging (his paste of the log: the secrets evaluate to '***', `github.sha` 64dcf68505f5..., the same two errors,
  Wrangler's log file named 2026-11-17_22-09-14). Verify failed in attempts 3, 4 and 5; each uploaded its artifact.
  So the permission is ruled out as the cause if Read was saved (only his words show it; no screen does): the test told
  to him (consultation 11). Left: the account ID or the token itself. (Two workers-sdk issue comments describe this
  pair of errors as a value Cloudflare does not recognise, and the second error is on a path without the account ID:
  a lean toward the token's value, not a finding.)
- Debug logging (his message 22:26:24, 145c9f63-a900-411c-9e2b-80fc93926e9e): turned on through the "Re-run failed jobs" dialog's "Enable
  debug logging" box (his screenshot of the dialog, not run); GitHub's re-run page: it applies to "the re-run". No
  repository secret or variable ACTIONS_STEP_DEBUG (his words: "I have not done such thing"), so the empty "Repository
  secrets" condition stands on his screenshot of 2026-11-16 and these words.

### The token check (his yes of 2026-11-18; one run; outside the marks)
- Branch `check-token` made from `main` 4a67b83; one commit 863b639d128a994a47752fece88632d54344188e adding
  .github/workflows/check-token.yml (80 lines; `git show HEAD:` sha256 0ad66aa0f2b36f7469bc3bdd57a4a05b7945e0037f96b19059000e0ca9b5aa71,
  equal to his yes); pushed 2026-11-18 about 11:10 UTC (the push's own time is not in a record; the run below was
  created 11:10:59). Pull request https://github.com/Mergincanli/shamble-preview-check/pull/2.
- Run: `check-token` #1, id 44292323566, event pull_request, head 863b639d128a..., actor and triggering actor
  Mergincanli, created 11:10:59; job `check` (id 134760377971) "waiting" on `preview` from 11:11:00. No other run
  started (the run list holds only this one and the setup pull request's deploy run).
- His screenshots (user message 2026-11-18 13:49:17 UTC, 7ec06823-d86e-4e1f-ace6-22bde57d6101): Cloudflare's Configurations page, "Account
  name": "Shamble Frontlines" (the dashboard's top-left shows the same): the rename is done, so his gate "No deploy run
  of any kind starts before you have my screenshot of the renamed Cloudflare account" is met. The check's run page:
  "The deployments have been approved."; status Success; job "check" 15 s; "Mergincanli approved ... preview".
- The check's output, word for word (job 134760377971; log lines 2026-11-18 13:48:45 to 13:48:59 UTC; runner image
  ubuntu-24.04 version 20261111.127.1; "added 38 packages in 5s"):
  ```
  token: length 53; starts with cfat_: yes; whitespace in or around it: no
  account id: length 32; exactly 32 hex digits: yes; whitespace: no; first and last 4: d17d...d8e7
  == A. the token, for this account: HTTP 200
  {
    "success": true,
    "errors": [],
    "result": {
      "status": "active",
      "expires_on": "2026-11-27T22:59:59Z",
      "not_before": "2026-11-26T23:00:00Z"
    }
  }
  == B. the token, without an account: HTTP 401
  {
    "success": false,
    "errors": [
      {
        "code": 1000,
        "message": "Invalid API Token"
      }
    ],
    "result": null
  }
  == C. the project the upload asks for: HTTP 401
  {
    "success": false,
    "errors": [
      {
        "code": 10000,
        "message": "Authentication error"
      }
    ],
    "result": null
  }
  == D: Wrangler exit 1
  D: shamble-preview-check named in its output: no
        1 [code: 10000]
        1 [code: 9109]
  ```
- Reading: both values have the right shape (53 = "cfat_" + 40 + an 8-character checksum, by the token-formats page's
  "cfat_[40 characters][checksum]"), and A answers 200 for this account, so the token is recognised and belongs to the
  account in the ID box. But Cloudflare reports the token's window as not_before 2026-11-26T23:00:00Z and expires_on
  2026-11-27T22:59:59Z: it may be used only on 27 November (Amsterdam time). Every call before then is refused: C and D
  give 10000, D's lookup 9109, B 1000. That explains attempts 2 to 5 (the rolled token), the empty "Last used", and why
  adding Read did not help; attempt 1 (the token before the roll, whose window was never read) is consistent with it. The form's chip "Nov 27 GMT+1" (his screenshot of
  2026-11-17 20:13:53) set both ends of the window to that one day; the session read that screenshot as "end date 27
  November" and called the choices right: a miss of the session's.
- After reading (his words of 2026-11-18): pull request #2 closed unmerged at 13:51:27 UTC (list_pull_requests:
  state closed, merged false); the run's logs deleted (delete_workflow_run_logs: "204 No Content"; get_job_logs for job
  134760377971 then answered 404). No run started on the close (the run list holds the same two runs).

### The token check, second run (TOKEN-WINDOW and CHECK-RERUN done by him; ID-ENDS compared by him)
- His words (2026-11-18 14:30:11 UTC, 8b31c945-c07d-4b1b-8b62-185f409e0a0e): "Step 1 done, the api token is still the same, I have also set it to 7 days expiration / The rerun went well tool  / ran the check a 2nd time, which also ran properly. / Yes, cloudflare account id begins with that and ends with d8e7". His screenshots: Cloudflare's
  "Review token" page, NAME shamble-preview-check-deploy, EXPIRATION "November 25, 2026 (7 days)", "Entire Shamble
  Frontlines account" with "Pages Read" and "Pages Write", "All IP addresses allowed", buttons "Back to edit" and
  "Update token"; the check's run page, attempt "Latest #2", status Success, job "check" 12 s, "Mergincanli approved".
- Run 44292323566 attempt 2: job 134871714885 created 14:27:17, started 14:27:28 (his approval between), done 14:27:40.
  Its output, word for word (log lines 14:27:36 to 14:27:38 UTC):
  ```
  token: length 53; starts with cfat_: yes; whitespace in or around it: no
  account id: length 32; exactly 32 hex digits: yes; whitespace: no; first and last 4: d17d...d8e7
  == A. the token, for this account: HTTP 200
  {
    "success": true,
    "errors": [],
    "result": {
      "status": "active",
      "expires_on": "2026-11-25T22:59:59Z",
      "not_before": null
    }
  }
  == B. the token, without an account: HTTP 401
  {
    "success": false,
    "errors": [
      {
        "code": 1000,
        "message": "Invalid API Token"
      }
    ],
    "result": null
  }
  == C. the project the upload asks for: HTTP 200
  {
    "success": true,
    "errors": [],
    "result": {
      "name": "shamble-preview-check"
    }
  }
  == D: Wrangler exit 0
  D: shamble-preview-check named in its output: yes
  D: no error code in its output
  ```
- Reading (outcome row 3): the token works now (not_before null) until 2026-11-25T22:59:59Z, the first record of its
  end date after saving; C finds the project; the pinned Wrangler answers. B's 1000 stays as before (the user verify
  method refuses an account token, consistent with both runs). The ID's ends d17d...d8e7 match Account Details (his
  words). Day 2's last day: 2026-11-25 (Amsterdam), from expires_on. The deploy's "Re-run failed jobs" must come before
  `site-pr` expires (2026-11-24T20:42:04Z).
- After reading: the run's logs deleted again (delete_workflow_run_logs, "204 No Content"); the run list unchanged (the
  deploy run still at attempt 5).

### The rehearsal, completed (setup pull request #1)
- His message (2026-11-18 14:58:02 UTC, 2e16e511-3a92-408e-b805-574dfb6c61e4): "It finally worked, nice." / "I have merged the pull request."
  His screenshot: run 44171977539 "Latest #6", status Success, 53 s, artifacts 6, "Mergincanli approved ... preview".
- Attempt 6 (his "Re-run failed jobs", started 14:55:09): deploy job 134890100614 created 14:55:13, started 14:55:29
  (his approval between), "Upload to Cloudflare Pages" 14:55:38-46 success; its record, word for word:
  `{"deployment_id":"1f6487e4-0a85-43d1-8b1f-cd80c0e1d61c","url":"https://1f6487e4.shamble-preview-check.pages.dev","alias":"https://setup.shamble-preview-check.pages.dev","environment":"preview","production_branch":"main","branch":"setup","marker":"64dcf68505f503df3f32ab258ca4363f240088ed","pr":"1","run":"44171977539","leg":"pr","upload":"1","mode":"preview","expect":"preview","made":"2026-11-18T14:55:46.818Z"}`
  (artifact deployment-record-44171977539-pr, id 14438791220, 410 bytes; the environment URL set to the alias).
- Verify job 134890518875, 14:55:52 to 14:56:02, success; headers.json (artifact 14438806696, digest
  sha256:49e1998212c9..., equal to the downloaded zip): summary records 1, pass 2, fail 0, no_reading 0. Both links pass
  (stable https://setup.shamble-preview-check.pages.dev/ and one-off https://1f6487e4.shamble-preview-check.pages.dev/):
  every file 200 on the first try; `/tiny.wasm` as `application/wasm`; `X-Robots-Tag: noindex` on every response; no
  COOP or COEP (the preview mode without isolation, as expected); `content.ef463692.bin` with "public,
  max-age=31536000, immutable", the rest "public, max-age=0, must-revalidate"; `X-Check-Rule: version-host` on both
  links, the stable branch link included (a rehearsal observation on R5-PREV-OQ6, not a mark). index.html and 404.html
  not fetched, by design (L3).
- The merge: pull request #1 merged 14:57:11 UTC; `main` now 2162db55d93f...; the branch `setup` still exists (so
  "Automatically delete head branches" is off, as plan A2 step 4 needs). The merge's cleanup: run `close` #1, id
  44329986855, created 14:57:15, "waiting" on `preview` (read after his message). The question of plan step 4, whether
  a merged pull request's cleanup run starts at all: it does, and it names the branch `setup` (head_branch setup).
- The cleanup (his message "done", 2026-11-18 15:19:18 UTC, 00e898e1-427c-4966-8be1-11ca0bce6610; his screenshot: run `close` #1 Success, job
  "cleanup" 7 s, "Mergincanli approved ... preview"): job 134891480469 created 14:57:17, started 15:04:33 (his approval
  between); the tombstone steps skipped (not a cs/pdt-* branch); "Delete every other upload of this branch (force),
  oldest first" 15:04:36-38 success; its log, word for word: "Pull request closed at 2026-11-18T14:57:11Z (merged:
  true); this job started at 2026-11-18T15:04:36Z." / "Uploads of setup found: 1" / "Delete
  1f6487e4-0a85-43d1-8b1f-cd80c0e1d61c (https://1f6487e4.shamble-preview-check.pages.dev): HTTP 200
  {"success":true,"errors":[]}" / "This job ended at 2026-11-18T15:04:38Z." "Uploads found: 1" equals the records (one
  upload, 1f6487e4...: procedure F3/TE-7). The merged pull request's event carried the branch (BRANCH: setup, MERGED:
  true, CLOSED_AT set). From the close (14:57:11) to the deletion (15:04:38): 7 min 27 s, most of it the wait for his
  approval.
- The rehearsal is complete: deploy, verify and cleanup ran end to end. Still to read: the first `probe` run on `main`
  (schedule ":23"; none had run at 15:04:41 UTC, the cleanup run's last update).
- The first probe: none yet at 2026-11-18 15:39 UTC (list of probe.yml runs: total_count 0; the merge put probe.yml
  on `main` at 14:57:11, so the 15:23 slot was the first it could fill). Read again by a check-in at about 16:00 UTC.
- Still no probe run at 2026-11-18 16:02 UTC (total_count 0). The workflow is registered: list_workflows shows
  "probe", .github/workflows/probe.yml, state "active", created 2026-11-18T15:57:13+01:00 (14:57:13 UTC, the merge).
  A last check-in is set for 16:31 UTC, after the 16:23 slot.
- **Finding: the hourly schedule has not fired in its first two slots.** At 2026-11-18 16:31:34 UTC (check-in 3, the
  last), the list of probe.yml runs still gives total_count 0. The slots it could fill since the merge put probe.yml on
  `main` (14:57:11 UTC; workflow created 14:57:13 UTC): 15:23 and 16:23 UTC, both without a run. Why is not known
  (GitHub's page on the schedule event is not read in this session). Next (consultation 17): one probe run started by
  hand (`purpose: deletion`) after
  `round-a`'s verify, so the script has met Cloudflare before sitting 2's 24-hour clocks start. No further check-in.

### Day 1, sitting 1 (the marks start here)
- `round-a`: deploy run #2, id 44347351522, started by the session (run_workflow, ref `main`, set `round-a`; created
  2026-11-18 16:26:57 UTC), head 2162db55d93f59cf2ed7e371a3915409f0b5a587. plan green (16:27:00-16:27:07); builds green:
  production (16:27:09-16:27:16), iso-off (16:27:09-16:27:34), iso-on (16:27:09-16:27:20); the three deploy legs
  (production, iso-off, iso-on) "waiting" on `preview` from 16:27:34 UTC. His list sent after that; still waiting at
  16:31:34 UTC.
- His approval (his message "Done:" with a screenshot of run #2's summary, 2026-11-18 17:06:23 UTC,
  3f82e55a-75de-4181-898f-d30fb10e81b9): the run page shows one event,
  "Mergincanli approved", Environments "preview and 2 more". The three deploy legs started at 17:01:51 (iso-on),
  17:01:53 (production) and 17:01:53 (iso-off) UTC, within 3 seconds: consistent with one approval releasing all three
  (his count of clicks asked). From waiting (16:27:34) to the release: 34 min 17 s (his list went out after 16:28).
- The uploads (each leg's log, word for word): production: "The project's production branch: main" / "✨ Success!
  Uploaded 1 files (7 already uploaded) (0.69 sec)" / deployment 0354374e-8cf6-4b8f-9735-15472d1529d9,
  https://0354374e.shamble-preview-check.pages.dev, alias null, environment "production", made 17:02:25.115Z. iso-off:
  "✨ Success! Uploaded 2 files (6 already uploaded) (2.17 sec)" / 6e61a4c4-37e0-443a-87ee-22ab90961be0, alias
  https://iso-off.shamble-preview-check.pages.dev, environment "preview", made 17:02:13.827Z. iso-on: "✨ Success!
  Uploaded 2 files (6 already uploaded) (2.71 sec)" / e48ee07b-2734-4be9-9e71-e46f217f6a9f, alias
  https://iso-on.shamble-preview-check.pages.dev, environment "preview", made 17:02:13.221Z. Every record's marker
  2162db55d93f59cf2ed7e371a3915409f0b5a587.
- PM-0006-5 (each build's SIZES line, word for word, the same for all three; node v22.23.4): "largest:
  content.ef463692.bin, 65536 bytes; limit 26214400 bytes; flagged: 0" / "SIZES
  {"largest":{"path":"content.ef463692.bin","size":65536},"limit":26214400,"flagged":[],"count":9}".
- PM-0006-2, verify (job 134977500343, 17:02:33-17:02:49; headers-44347351522, artifact 14448883620, zip sha256
  bbb324aadcea..., equal to the log's digest; read with jq from `.gate/preview-hosting-check/day1/round-a/`):
  "summary: {"records":3,"pass":5,"fail":1,"no_reading":0}", exit 1. Per link: iso-off stable and one-off pass;
  iso-on stable and one-off pass; production stable (https://shamble-preview-check.pages.dev/) pass; **production
  one-off (https://0354374e.shamble-preview-check.pages.dev/) fail on all 7 paths: "production carries X-Robots-Tag
  noindex"** (value "noindex"; production's build writes no noindex, so the host added it). Every response 200 on the
  first try. All other headers as expected: `.wasm` application/wasm on all 6 links; the content file "public,
  max-age=31536000, immutable", the rest "public, max-age=0, must-revalidate"; noindex on all 4 preview links;
  iso-on's COOP "same-origin" and COEP "require-corp" on all 7 paths of both links; no COOP/COEP elsewhere.
  X-Check-Rule: "project-host" on production's own address; "version-host" on the 5 other links, the branch links
  included (R5-PREV-OQ6, recorded, judged nowhere).
- **Risk 13 (plan.md:1358) has happened:** production's one-off link carries noindex, so PM-0006-2's row is not met as
  written. Recorded "not met as written", never a pass; it goes to the draft's author with the report (ADR 0006
  :1409-1414) and holds the check's end until the draft is revised (:1401). The sitting continues; `restore-production`
  reads it again.
- **The first probe run** (started by hand by the session, `purpose: deletion`, after `round-a`'s verify): probe run
  #1, id 44355175073, created 17:08:00 UTC, completed 17:08:26, success; probe-results-44355175073 (artifact
  14452700323, zip sha256 4f37fba6fad4..., equal to its digest), probed_at 2026-11-18T17:08:20.713Z, read with jq from
  `.gate/preview-hosting-check/day1/probe-1/`. Rows, word for word in substance: setup (case "rehearsal", pull request
  1 closed and merged 14:57:11Z, hours_since_close 2.19) stable https://setup.shamble-preview-check.pages.dev/ status
  404, outcome "gone", expected "gone"; one-off https://1f6487e4.shamble-preview-check.pages.dev/ 404 "gone", expected
  "gone"; production (main) stable 200 "serves", expected "serves"; production one-off (0354374e) 200 "serves",
  expected null; iso-off and iso-on, both links, 200 "serves", expected "serves". Every row 1 try. So the script works
  against Cloudflare, and the setup upload is gone from both links (step 22's first outcome: nothing to do).
- **The hosting checklist, steps 1 to 3 (Brave, his PC)**: his message "all three opened and screenshotted for you"
  (2026-11-18 17:12:23 UTC, 8fce8e5a-1ebc-4993-ae7b-0aef9f8339c8), three screenshots, transcribed (the images stay
  out of the repository):
  - iso-off.shamble-preview-check.pages.dev: Build marker 2162db55d93f59cf2ed7e371a3915409f0b5a587; Build mode
    preview; crossOriginIsolated (page) false; crossOriginIsolated (worker) false; WebAssembly "ran (2 + 3 = 5)";
    Content file "content.ef463692.bin (65536 bytes)"; Status done. Resources: page "/", "/page.css", "/page.js",
    "/manifest.json", "/favicon.ico", "/content.ef463692.bin", "/worker.js"; worker "/worker.js", "/tiny.wasm".
  - iso-on.shamble-preview-check.pages.dev: the same marker; Build mode preview-isolated; page true; worker true;
    WebAssembly "ran (2 + 3 = 5)"; the same content file and size; Status done. Resources: page "/", "/page.css",
    "/page.js", "/manifest.json", "/content.ef463692.bin", "/favicon.ico", "/worker.js"; worker "/worker.js",
    "/tiny.wasm".
  - shamble-preview-check.pages.dev: the same marker; Build mode production; page false; worker false; WebAssembly
    "ran (2 + 3 = 5)"; the same content file and size; Status done. Resources: the same seven page paths as iso-on;
    worker "/worker.js", "/tiny.wasm".
  - No login screen appears in any screenshot; each shows the page itself. Every value is the expected one (ADR 0006
    :1444-1447). His "achieved"/"not yet" words: asked of him (the checklist has him record them).
  - TE-12 (the page's resource list against the manifest): the build's files the page and worker loaded are all
    in the manifest; "/favicon.ico" is the one extra entry, Brave's own icon request (no such file in the build; not
    fetched by check-headers, which reads the manifest's files).
- **The build count, first reading (checklist step 4, PM-0006-7)**: his screenshot (no words; 2026-11-18 17:19:10 UTC,
  e98551a5-b3fa-47c1-973b-4f89cd4e5544) of Cloudflare, "Workers &
  Pages > shamble-preview-check", tab "Deployments" (tabs: Deployments, Metrics, Custom domains, Settings), transcribed:
  "Production", "Domains: shamble-preview-check.pages.dev": Production, main, "preview-check run 443473515...",
  https://0354374e.shamble-preview-check.pages.dev, "16 minutes ago". "All deployments": Production main
  (0354374e, "16 minutes ago"); Preview iso-off ("preview-check run 443473515...", 6e61a4c4, "16 minutes ago"); Preview
  iso-on (e48ee07b, "16 minutes ago"); Production main (no message), https://77605f36.shamble-preview-check.pages.dev,
  "3 days ago" (his first upload when he made the project, plan A4). The setup upload (1f6487e4) is not listed (deleted
  15:04:38 UTC). **No count of builds is shown on this page.** The overview and the Metrics tab: asked of him before
  `uploads` starts.
- His message "achieved" with a screenshot (2026-11-18 17:21:40 UTC, 001061fb-f067-456c-b4f2-bfad995c4127):
  - **Hosting checklist steps 1, 2 and 3: "achieved"** (his word, in answer to "Reply "achieved" for steps 1 to 3, or
    name any that are "not yet"").
  - The Metrics tab, transcribed: selector "Production", range "Last 24 hours"; Requests "Success 0", "Errors 0", "No
    data is available for this time range"; Subrequests 0; Errors: Internal 0, Script Threw Exception 0, Exceeded CPU
    Time Limits 0, Exceeded Memory 0, Client Disconnected 0; Median CPU Time and Request duration: every percentile 0.
    **No count of builds is shown on this tab.** (Requests 0 although production was fetched many times today: what
    this tab counts is not documented here; recorded, not explained.)
  - The Workers & Pages overview: not yet seen; asked of him before `uploads` starts (plan Part 3 C step 3 names it).
- His screenshot of the Workers & Pages overview (no words; 2026-11-18 17:23:36 UTC,
  33658e3d-8f06-4af5-ad77-4e4fae9bcbb4), transcribed: account "Shamble Frontlines"; "Workers & Pages", one application,
  shamble-preview-check (shamble-preview-check.pages.dev), "21m ago", "No Git connection", "Showing 1 - 1 of 1"; panel
  "Usage" (Today): "Requests today 0 / 100,000", "Observability events today 0 / 200,000"; "Account details": the
  account ID (shown in full on his screen, not copied here; its first and last 4 are d17d...d8e7, as the token check
  printed) and the workers.dev subdomain (not copied here); "Cloudflare Access": "Set up Zero Trust before requiring
  Access sign-in for Workers on this account." **No count of builds is shown.**
- **The first reading of the build count: "no count shown"** on all three places plan Part 3 C step 3 names (the
  project's Deployments page, its Metrics tab, the Workers & Pages overview).
- `uploads` started by the session right after (run_workflow, ref `main`, set `uploads`; 204 at about 17:23:55 UTC).
- `uploads` (PM-0006-7's 10 deploys): deploy run #3, id 44358217862 (created 17:23:54 UTC). plan green (17:23:58-17:24:16),
  build green (17:24:18-17:24:26; SIZES line identical to round-a's: "flagged":[], "count":9); the deploy job "waiting"
  on `preview` from 17:24:26; his message "approved and deployed" (2026-11-18 17:29:32 UTC,
  49b34555-967c-4301-89ab-9481638129cf); the deploy job's log runs 17:27:42-17:28:54 UTC (his approval just before
  17:27:42; one approval, one job).
  - The 10 uploads to branch `uploads`, each "✨ Success! Uploaded 0 files (8 already uploaded)" (0.22 to 0.40 sec),
    alias https://uploads.shamble-preview-check.pages.dev, environment "preview", marker 2162db55d93f...: 1 c28fc449
    (made 17:28:10.728Z), 2 deb45872 (17:28:14.759Z), 3 5b57fa5a (17:28:19.971Z), 4 f512e84d (17:28:24.211Z), 5
    283bba70 (17:28:30.563Z), 6 a845d2e3 (17:28:34.429Z), 7 37ac53a6 (17:28:38.498Z), 8 51e10edc (17:28:42.226Z), 9
    ff3ec055 (17:28:46.114Z), 10 44dd439a (17:28:52.521Z) (full IDs in deployment-record-44358217862-uploads, artifact
    14451668742).
  - verify (17:28:59-17:29:23; headers-44358217862, artifact 14453018448, zip sha256 1f38f9088428..., equal to its
    digest): "summary: {"records":10,"pass":20,"fail":0,"no_reading":0}", exit 0: every upload passes on its one-off
    link and on the branch link (the branch link read once per record), X-Check-Rule "version-host" on all.
  - Run conclusion: success. The second build-count reading asked of him now.
- **The build count, second reading**: his screenshot of the project's Deployments page (2026-11-18 17:33:15 UTC,
  7442b908-6d1c-48ba-a65b-36944fd3247f; his words "When are we done with these hideous tasks"), transcribed:
  Production main 0354374e "30 minutes ago"; "All deployments": ten Preview rows, branch `uploads`, "preview-check run
  443582178...", 44dd439a, ff3ec055, 51e10edc, 37ac53a6, a845d2e3, 283bba70, f512e84d, 5b57fa5a ("4 minutes ago"),
  deb45872, c28fc449 ("5 minutes ago"); then Production main 0354374e, Preview iso-off 6e61a4c4 ("31 minutes ago"), the
  rest below the screenshot's edge. The ten uploads are listed, matching the records one for one. **No count of builds
  is shown.** The overview was not read again: at the first reading it had no count field at all (only "Requests
  today" and "Observability events today"), and he was not asked twice. So PM-0006-7 reads "no count shown" before
  and after, on the places plan Part 3 C step 3 names; the deployments list shows the ten uploads.
- `bites` started by the session right after (run_workflow, ref `main`, set `bites`; 204 at about 17:33:50 UTC).
- `bites` (PM-0006-2's COEP bite, PM-0006-4's header bite, PM-0006-5's size bite): deploy run #4, id 44360058220
  (created 17:33:49 UTC). plan green; builds green (17:34:15-17:34:40). SIZES lines: bite-oversize "largest:
  oversize.bin, 27262976 bytes; limit 26214400 bytes; flagged: 1" / "SIZES {"largest":{"path":"oversize.bin","size":
  27262976},"limit":26214400,"flagged":[{"path":"oversize.bin","size":27262976}],"count":10}" (the size bite caught by
  the size check). The three deploy legs "waiting" on `preview` from 17:34:40; they started at 17:50:20 (oversize),
  17:50:21 (nocache) and 17:50:22 (coep) UTC: one approval (his), just before his message of 17:50:30.
  - bite-oversize: Wrangler refused it, word for word: " ⛅️ wrangler 4.156.0" / "✘ [ERROR] Error: Pages only supports
    files up to 25 MiB in size" / "  oversize.bin is 26 MiB in size" / "##[error]Process completed with exit code 1."
    No upload, no record (if-no-files-found: warn). The refusal comes before any call to Cloudflare: the log shows no
    account name, no account ID and no error code (the account-printout question: nothing printed on this path).
  - bite-nocache: "✨ Success! Uploaded 1 files (7 already uploaded) (0.99 sec)"; deployment
    29622b3b-8d8c-4d66-b9d5-2a67e5207523, alias https://bite-nocache.shamble-preview-check.pages.dev, "preview".
  - bite-coep: "✨ Success! Uploaded 1 files (7 already uploaded) (0.86 sec)"; deployment
    0b844b60-0458-44ac-a22e-d31ed505691d, alias https://bite-coep.shamble-preview-check.pages.dev, "preview".
  - verify (17:51:00-17:51:23; headers-44360058220, artifact 14456636722, zip sha256 c3bcd2713ff6..., equal to its
    digest): "summary: {"records":2,"pass":0,"fail":4,"no_reading":0}", exit 1. bite-coep, both links: only "/worker.js:
    COEP "", expected "require-corp"" (the COEP bite caught); bite-nocache, both links: only "/content.ef463692.bin:
    content file Cache-Control "no-cache", expected "immutable"" (the cache bite caught, header half). Nothing else
    failed. Run conclusion: failure, as planned: all three bites caught.
- `browsers` started by the session (run_workflow, ref `main`; 204 at about 17:55:39 UTC), after the records showed
  bite-coep and bite-nocache uploaded; production is still round-a's good build (0354374e).
- **`browsers`** (PM-0006-3 and PM-0006-4): browsers run #1, id 44364041726, started by the session 17:55:38 UTC,
  completed 17:59:13, conclusion failure (the macOS job). Artifacts read with jq from
  `.gate/preview-hosting-check/day1/browsers/` (zip sha256: windows f52bfb919353..., macos 4ca00fa3669e..., each equal
  to its digest).
  - windows-2025 (image win25-vs2026 20261103.298.1, WebdriverIO 10.1.0, ran_at 17:59:08.392Z): "summary":
    {"rows":51,"unexpected":0,"no_reading":0}. Chrome 156.0.8078.80, Edge 156.0.4314.58, Firefox 159.0.1, each link 3
    loads: production false/false, iso-off false/false, iso-on true/true, bite-nocache false/false ("as expected");
    bite-coep page true, worker "not reported (worker error: its script did not load)" ("bite caught", all 3 browsers,
    all 3 loads). PM-0006-4 in Chrome: production's content file on the second visit transferSize 0 (3 of 3, "as
    expected"); bite-nocache 300 (3 of 3, "bite caught"). Every page row: marker 2162db55d93f..., status done, "ran (2 +
    3 = 5)"; the resource lists hold the build's files plus "/favicon.ico" (as in Brave).
  - macos-26 (image macos26 20261102.0473.1, ran_at 17:56:16.077Z): "summary": {"rows":15,"unexpected":0,
    "no_reading":14}, exit 3. Safari 27.2, production load 1: false/false, "as expected". The other 14 loads: "no
    reading", error "There is already a Safaridriver instance running on port 49178!". Cause (read in tools/wdio/run.mjs
    at 2162db5, lines 44-50): withSession() starts a new WebDriver session with remote() for every load and deletes it
    afterwards; the second Safari session could not start while the first safaridriver still ran. A script fault, not a
    Cloudflare reading: PM-0006-3's Safari row has 1 reading of 15 and stays open ("no reading" never passes).
- His untick (REVIEWER (b)): his message "done" with a screenshot (2026-11-18 18:08:10 UTC,
  a23e7d93-5eb1-4093-a99d-e64ebb1df1b8), transcribed: banner "Environment "preview" updated."; "Environments /
  Configure preview": "Required reviewers" unticked; "Wait timer" unticked; "Allow administrators to bypass configured
  protection rules" unticked; "Deployment branches and tags": "No restriction"; secrets CLOUDFLARE_ACCOUNT_ID and
  CLOUDFLARE_API_TOKEN, both "yesterday"; "This environment has no variables." Only the reviewer changed.
- `bite-production` started by the session after that (run_workflow, ref `main`, set `bite-production`); it does not
  wait (no reviewer).
- **`bite-production`** (PM-0006-2's noindex bite on production): deploy run #5, id 44366750977 (created 18:10:20 UTC).
  plan green (18:10:23-18:10:28), build green (18:10:31-18:10:51), the deploy job ran at once, no wait (18:10:54-
  18:11:16: the reviewer is off), deployment e9a37459-a342-4d33-ba73-32f5f3549ad7 (environment "production"). verify
  (18:11:18-18:11:41; headers-44366750977, artifact 14458320188, zip sha256 d1ddcad0f8f2..., equal to the log's
  digest): "summary: {"records":1,"pass":0,"fail":2,"no_reading":0}"; both production links
  (https://shamble-preview-check.pages.dev/ and https://e9a37459.shamble-preview-check.pages.dev/) fail on all 7 paths
  with "production carries X-Robots-Tag noindex", and on nothing else: the bite caught on production's own address.
- `restore-production` started by the session right after (run_workflow, ref `main`, set `restore-production`).
- **`restore-production`**: deploy run #6, id 44367096948 (created 18:12:13 UTC); plan, build and deploy green, no
  wait; deployment de1c3e5e-1abb-4c8c-af5f-fb49165914d7 (record artifact created 18:13:27 UTC). verify (headers-44367096948,
  artifact 14458520412, zip sha256 67308e8e565f..., equal to its digest; checked_at 18:13:45.151Z): "summary":
  {"records":1,"pass":1,"fail":1,"no_reading":0}: production's own address passes (no noindex: the bite is gone);
  the one-off link https://de1c3e5e.shamble-preview-check.pages.dev/ fails on all 7 paths with "production carries
  X-Robots-Tag noindex": Risk 13 again, as round-a showed. Production carried the planted noindex from the bite's
  upload (between 18:11:02 and 18:11:13 UTC) to the restore's (by 18:13:27 UTC): about 2 minutes.
- **Sitting 1's readings in short**: Brave steps 1-3 achieved (his word); PM-0006-2: every header as expected on 15
  preview links (24 readings: round-a 4, uploads 20; counted with jq from the headers files) and on production's own
  address (round-a and restore-production), the two planted mistakes caught (COEP on the worker; noindex on
  production), Risk 13 on production's one-off link (twice); PM-0006-3: Chrome, Edge and Firefox as expected on all
  five links (bite caught), Safari 1 reading of 15 (script fault, open); PM-0006-4: Chrome 0 bytes on production's
  second visit, 300 on the no-cache bite (caught), and the header half caught on both links; PM-0006-5: the 26 MiB file
  flagged by the size check and refused by Wrangler; PM-0006-7: "no count shown" before and after, ten uploads listed.
  Approvals clicked in sitting 1: 3 (round-a, uploads, bites), one per run whatever its legs; then REVIEWER (b).
- The hourly probe: the 17:23 UTC slot passed without a scheduled run as well (list of probe.yml runs at about 18:14
  UTC: only run #1, started by hand). Three slots missed since the merge: 15:23, 16:23, 17:23.
- **Sitting 1's runs against its list** (all runs of the repository, read at about 18:16 UTC: total_count 10): the
  five listed deploy runs #2 round-a (44347351522), #3 uploads (44358217862), #4 bites (44360058220), #5
  bite-production (44366750977), #6 restore-production (44367096948); the two that start without him, browsers #1
  (44364041726) and probe #1 (44355175073, started by hand, told him); all on `main` 2162db55d93f, actor and
  triggering actor Mergincanli. The other three runs are the rehearsal's and the token check's (before sitting 1).
  Nothing ran that was not on the list.

### Day 1, sitting 2 (PREV-DELETE's cases; REVIEWER (b): no approvals)
- His list sent at about 18:25 UTC (runs that will start; his two merges; the PR-page look); the go by this session's
  scheduled check-in (fired 18:27:22 UTC).
- The six branches pushed at 18:27:40 UTC (one `git push`, new branches): cs/pd-one a2cfef9, cs/pd-three d78dfb4
  (push 1), cs/pd-open 236a6d7, cs/pd-off 683e8c0, cs/pdt-one 7370d4b, cs/pdt-three 9ebeeb6 (push 1). Each commit's
  workflows hashed before the push: deploy 29593af18d34, close 3afdf389d636 (absent on 683e8c0, case (d), by design),
  probe f24ad6e81daf, browsers 6791076916ae; each adds only `test-prs/<branch>.txt`.
- The six pull requests opened right after with the GitHub MCP tool: #3 (a) cs/pd-one, #4 (b) cs/pd-three, #5 (c)
  cs/pd-open, #6 (d) cs/pd-off, #7 (a') cs/pdt-one, #8 (b') cs/pdt-three.
- Six deploy runs started on the pull_request event, none waiting (list read 18:28:28 UTC): #7 44370016948 (a) cs/pd-one
  (head a2cfef9, PR 3, created 18:28:02), #8 44370024556 (b) cs/pd-three push 1 (d78dfb4, PR 4, 18:28:05), #9
  44370034770 (c) cs/pd-open (236a6d7, PR 5, 18:28:08), #10 44370043893 (d) cs/pd-off (683e8c0, PR 6, 18:28:11), #11
  44370054922 (a') cs/pdt-one (7370d4b, PR 7, 18:28:15), #12 44370062021 (b') cs/pdt-three push 1 (9ebeeb6, PR 8,
  18:28:17). No other workflow started (no close run).
