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
- **PM-0006-10 (does GitHub show the preview link on the pull request's page?): yes, logged out.** The session's
  picture of pull request #3's public page (headless Chromium, no login, 2026-11-18 18:29:33 UTC, HTTP 200;
  `.gate/preview-hosting-check/day1/pr-page/pr3.png`, 2560x2284, kept out of the repository; its text in pr3.json),
  transcribed: in the timeline, "Mergincanli deployed to preview now — with GitHub Actions" and a "View deployment"
  button; below, "This branch was successfully deployed", "1 active deployment", "Show environments". The page's links
  "preview" and "View deployment" both point to https://cs-pd-one.shamble-preview-check.pages.dev/ (the branch link,
  the deploy job's environment url). His logged-in page: asked of him (one line). (The deploy job holds no
  `deployments: write`: the link shows without it, as the security-auditor's L-finding asked to learn.)
- The six first runs, all conclusion success (verify exit 0: every link of the upload passed; list read 18:30:40 UTC):
  #7 (a) 18:28:02-18:29:24, #8 (b) push 1 18:28:05-18:29:30, #9 (c) 18:28:08-18:29:32, #10 (d) 18:28:11-18:29:14,
  #11 (a') 18:28:15-18:29:36, #12 (b') push 1 18:28:17-18:29:57 (created to last update, UTC). What was read of the
  ten test runs: their conclusions (all success, so verify passed every link) and, after the closes, their upload
  records through the probe: each cleanup's "Delete <id>" lines matched against probe-2's record rows per branch with
  jq (no new download): (a) 1b3c905e; (a') 672b3d0e, tombstone 2b214ae6 kept; (b) 03630e1d, f1e112f2, f516ed27; (b')
  6b6415e5, 5ad91df8, ed9c2421, tombstone 5ff46e6d kept: the deleted ids equal the recorded uploads, branch by branch.
  The ten headers files themselves were not downloaded.
- Push 2 of (b) and (b'), after each branch's first verify had finished (headers-44370024556 at 18:29:27,
  headers-44370062021 at 18:29:54): `git push` 18:30:19 UTC, cs/pd-three d78dfb4..2244b55, cs/pdt-three
  9ebeeb6..51130dd (both commits hashed before: the approved texts). Runs #13 44370443574 (b) and #14 44370444666
  (b'), created 18:30:25, no wait.
- Push 2 results: run #13 (b) verify done 18:31:23 UTC (headers-44370443574), run #14 (b') 18:31:33
  (headers-44370444666). Push 3, each after its branch's push-2 verify: cs/pd-three 2244b55..12f71d3 at 18:31:44 UTC,
  cs/pdt-three 51130dd..6188812 at 18:31:55 UTC (both hashed before: the approved texts).
- Push 3 results: run #15 (b) 44370706631 success (18:31:50-18:33:08 UTC), run #16 (b') 44370744826 success
  (18:32:02-18:33:21). **All ten test uploads passed verify** (ten runs, conclusion success).
- **The closes (the 24-hour clocks start here)**, by the session with the GitHub MCP tool (closed_at from each pull
  request's data): (a) #3 closed 2026-11-18T18:32:51Z; (a') #7 18:32:54Z; (d) #6 18:32:56Z; none merged, branches
  kept. (a)'s and (a')'s pushes had finished verify before; (b)'s and (b')'s push-3 runs were still running on other
  branches.
- Cleanup runs: close #2 44370905082 for (a) (cs/pd-one), created 18:32:54, success at 18:33:08; close #3 44370914405
  for (a') (cs/pdt-one), created 18:32:57, running at 18:33:30. **No cleanup run for (d)** (cs/pd-off): close.yml
  deleted on its branch, as planned (Risk 10 did not happen so far; the close.yml run list holds no cs/pd-off run).
- His message (2026-11-18 18:52:12 UTC, 8cc8eb72-82ae-48ed-ac2a-baaea512ac7a), word for word: "1. I think I do" /
  "2. merged" / "3. Merged"; then, queued while I worked, a screenshot of pull request #8 (18:52:36 UTC, attachment
  f154b837-2e59-4a7e-b81f-27873b4c7c97), transcribed: "Merged",
  "Mergincanli merged in 347d5e0 1 minute ago. Delete the cs/pdt-three branch to tidy up." with the buttons "Delete
  branch" and "Revert"; the sidebar "Deployments": "preview", "1 minute ago", with an open-link icon. **PM-0006-10,
  logged in: the page shows the deployment ("Deployments: preview" with a link)**; his words "I think I do".
- The merges (from the pull requests' data): (b) #4 merged 2026-11-18T18:50:33Z (merge commit 756d33f), (b') #8 merged
  18:50:43Z (347d5e0), both by Mergincanli. **The branches were not deleted** (git ls-remote at 18:52:58 UTC still
  lists cs/pd-three 12f71d3 and cs/pdt-three 6188812): his "Delete branch" asked again.
- `main` re-hashed after the merges (347d5e0): deploy 29593af18d34, close 3afdf389d636, probe f24ad6e81daf, browsers
  6791076916ae (unchanged); `git diff --name-status 2162db5 347d5e0`: only "A test-prs/pd-three.txt", "A
  test-prs/pdt-three.txt".
- The cleanups, word for word from their logs:
  - (a) close #2 44370905082: "Pull request closed at 2026-11-18T18:32:51Z (merged: false); this job started at
    2026-11-18T18:33:03Z." / "Uploads of cs/pd-one found: 1" / "Delete 1b3c905e-aa4c-4832-bb02-a0337ea78361
    (https://1b3c905e.shamble-preview-check.pages.dev): HTTP 200 {"success":true,"errors":[]}" / "This job ended at
    2026-11-18T18:33:05Z."
  - (a') close #3 44370914405: the tombstone "✨ Success! Uploaded 1 files (0.90 sec)", deployment
    2b214ae6-2566-4331-b348-c89e074b3764, alias https://cs-pdt-one.shamble-preview-check.pages.dev, made
    18:33:22.294Z; "Pull request closed at 2026-11-18T18:32:54Z (merged: false); this job started at
    2026-11-18T18:33:22Z." / "Uploads of cs/pdt-one found: 2" / "Delete 672b3d0e-d806-45ea-a286-5c2b8229e095
    (https://672b3d0e.shamble-preview-check.pages.dev): HTTP 200 {"success":true,"errors":[]}" / "Kept (the
    tombstone): 2b214ae6-2566-4331-b348-c89e074b3764" / "This job ended at 2026-11-18T18:33:23Z."
  - (b) close #4 44374077151: "Pull request closed at 2026-11-18T18:50:33Z (merged: true); this job started at
    2026-11-18T18:50:46Z." / "Uploads of cs/pd-three found: 3" / "Delete 03630e1d-0ce2-4de1-a230-0a752eb81165 ...:
    HTTP 200 ..." / "Delete f1e112f2-eefd-441c-b2c3-ad6d0201a2be ...: HTTP 200 ..." / "Delete
    f516ed27-3513-4737-a5b1-6a60e51ebd88 ...: HTTP 200 ..." / "This job ended at 2026-11-18T18:50:47Z."
  - (b') close #5 44374107847: the tombstone "✨ Success! Uploaded 0 files (1 already uploaded) (0.30 sec)",
    deployment 5ff46e6d-ea01-4c0e-b4e3-c0d46e50dc1c, alias https://cs-pdt-three.shamble-preview-check.pages.dev, made
    18:51:05.446Z; "Pull request closed at 2026-11-18T18:50:43Z (merged: true); this job started at
    2026-11-18T18:51:05Z." / "Uploads of cs/pdt-three found: 4" / three deletes, HTTP 200 each (6b6415e5..., 5ad91df8...,
    ed9c2421...) / "Kept (the tombstone): 5ff46e6d-ea01-4c0e-b4e3-c0d46e50dc1c" / "This job ended at
    2026-11-18T18:51:08Z."
  - "Uploads found" against the records (F3/TE-7): (a) 1 = 1 upload; (a') 2 = 1 upload + the tombstone; (b) 3 = 3
    uploads; (b') 4 = 3 uploads + the tombstone. From close to deletion: (a) 14 s, (a') 29 s, (b) 14 s, (b') 25 s.
  - No cleanup run for (d). Every cleanup ran at once (no reviewer).
- A probe started by hand right after the last cleanup (run_workflow, `purpose: deletion`).
- **The probe right after the cleanups**: probe run #2, id 44374666257 (by hand, `purpose: deletion`, head 347d5e0),
  created 18:53:54 UTC, success 18:54:34; probe-results-44374666257 (artifact 14461031992, zip sha256 db8b83a018fe...,
  equal to its digest), probed_at 2026-11-18T18:54:22.157Z, read with jq from `.gate/preview-hosting-check/day1/probe-2/`.
  By case: (a) 2 rows, gone 2; (a') tombstone 3 rows, gone 3; (b) 4 rows, gone 4; (b') tombstone 5 rows, gone 5; (c)
  control 2 rows, serves 2; (d) bite 2 rows, serves 2; the by-hand previews 19 rows, serves 19; the rehearsal 2 rows,
  gone 2; production 4 rows, serves 4 (its own address as expected; the one-off links of the three production uploads
  0354374e, e9a37459 (the noindex bite) and de1c3e5e still serve, "expected" null: no row judges them). Every judged
  row as expected. So 22 to 3 minutes after their closes, every link of (a), (a'), (b) and (b') is already gone; the
  mark itself is read at 24 hours.
- The hourly schedule: the 18:23 UTC slot passed without a scheduled run as well (four slots missed: 15:23, 16:23,
  17:23, 18:23). Stand-ins: a by-hand probe about an hour later (check-in at 19:55 UTC) and the 24-hour probe (check-in
  at 2026-11-19 18:55 UTC, after the last close's 24-hour mark, 18:50:43Z).
- **Sitting 2's runs against its list** (all runs of the repository, total_count 25 at about 18:56 UTC): the ten
  deploy runs #7-#16 (six first pushes, two second, two third), the four cleanup runs #2-#5 ((a), (a'), (b), (b')),
  the by-hand probe #2; with the ten runs before sitting 2, 25. No cleanup run for (d), no scheduled probe, nothing
  else. All by Mergincanli.
- His message "I have deleted both branches 4 and 8," / "You can do sitting 3" (2026-11-18 18:58:58 UTC,
  49c5e112-f261-48e8-9b31-ee3d56bf008b):
  git ls-remote at 18:59:29 UTC lists no cs/pd-three and no cs/pdt-three (deleted); cs/pd-one, cs/pd-off, cs/pd-open,
  cs/pdt-one, check-token, setup and main (347d5e0) remain. Case (b) and (b'): merged, branch deleted, as planned.

### Day 1, sitting 3 (the token tests, PM-0006-32; consultation 20's order)
- His list sent at about 19:06 UTC (the order: off, on; "say "plan order" if you prefer"); no reply before the go; the
  go by this session's scheduled check-in (fired 19:10:59 UTC).
- Step 1 (the reviewer off since his untick, 18:08 UTC): cs/b9-edit 7f3a684 pushed at 19:11:17 UTC (deploy.yml = the
  approved variant fd3dc4c6029b; the other three workflows unchanged); pull request #9 "Token test: cs/b9-edit, the
  deploy job rewritten" opened right after.
- **Step 1's outcome row hit: "the off run waits".** Run #17 (44377870587): plan green (19:11:37-19:11:54 UTC), build
  green (19:11:58-19:12:20), then the deploy job (135062245511) "waiting" from 19:12:20 UTC, although the reviewer was
  off at his untick (18:08 UTC) and every run since then ran without waiting (the last: close #5 for (b'), 18:50:47 to
  18:51:11). So `preview` changed after 18:51 UTC; the likely reading (not yet shown by a record) is that he ticked
  "Required reviewers" after my list of about 19:06 UTC (its step 2). Per the row: stop, his preview page read (a
  screenshot asked). If it shows the reviewer on, this run is the control (approve; "secret: present") and the
  reviewer-off reading is taken in the plan's order (untick, re-run, re-tick).
- Check-in (fired 19:55:21 UTC, read 19:55:47): run #17's deploy job still "waiting" (no message from him since my
  question of about 19:13 UTC); the 19:23 UTC slot passed without a scheduled probe run (five slots missed:
  15:23-19:23; 26 runs in all, the 26th being run #17). A probe started by hand (`purpose: deletion`) as the hourly
  stand-in; it holds no secret and names no environment, so it does not touch the waiting run.
- **The hourly stand-in probe**: probe run #3, id 44385645676 (by hand, head 347d5e0), created 19:56:06 UTC, success
  19:56:56; probe-results-44385645676 (artifact 14466161509, zip sha256 5136260483b7..., equal to its digest),
  probed_at 2026-11-18T19:56:45.076Z, read with jq from `.gate/preview-hosting-check/day1/probe-3/`. By case: (a) 2
  rows gone (hours_since_close 1.4); (a') tombstone 3 rows gone (1.4); (b) 4 rows gone (1.1); (b') tombstone 5 rows
  gone (1.1); (c) control 2 rows serves; (d) bite 2 rows serves (1.4); the by-hand previews 19 rows serves;
  production 4 rows serves (its own address as expected; three one-off links unjudged); the rehearsal 2 rows gone
  (4.99). No row "not as expected" (as_expected false: 0). Nothing to tell him (the check-in's rule).
- His screenshot of `preview` with "approved the run also: https://github.com/Mergincanli/shamble-preview-check/actions/runs/44377870587"
  (2026-11-18 23:15:46 UTC, e6d8a826-ad3a-412f-be18-8ec09bc13788), transcribed: "Environments / Configure preview";
  "Required reviewers" ticked, "Add up to 5 more reviewers", Mergincanli listed; "Prevent self-review" unticked; "Wait
  timer" unticked; "Allow administrators to bypass configured protection rules" unticked; "No restriction"; secrets
  CLOUDFLARE_ACCOUNT_ID and CLOUDFLARE_API_TOKEN, "yesterday"; "This environment has no variables." So the reviewer
  was on when run #17 started (he ticked it after my list of about 19:06 UTC; when exactly, no record shows). The
  reviewer-off window (his untick 18:08 UTC) closed at that tick, at some time between 18:51:11 (the last run that ran
  without waiting) and 19:12:20 UTC (run #17's wait).
- **The control (PM-0006-32's first row): run #17's deploy job waited on `preview` from 19:12:20 UTC, he approved it,
  and it ran at 23:15:23 UTC**, word for word: "B9-MARKER: the rewritten deploy job ran at 2026-11-18T23:15:23Z" /
  "secret: present" (the env block shows "TOKEN: ***": the value masked). So with the reviewer on, a rewritten job
  waits, and after the approval it reaches the token. Sitting 3 now goes in the plan's order (untick, re-run without a
  wait, re-tick; SA-R4's re-run must wait and he rejects it; then cs/b9-prod).
- His message (2026-11-19 00:10:53 UTC, 5682ca4d-c2ca-4cca-924f-3d1539190ac9), word for word in part: "I have unticked
  Required Reviewers and saved it" / ""Repository secrets" list says  "This repository has no secrets"." (his words; no
  screenshot of that list came with it: SA-R8 asks for a screenshot, asked for with the re-tick) / "Open
  https://claude.ai/customize/connectors. I have opened it and the connection to shamble frontlines is working"; his
  screenshot is of claude.ai's GitHub plugin page ("GitHub account connected"; "Claude GitHub App", "Cloud sessions
  only", Mergincanli "Installed"; "Check repository status": "Claude Code cloud sessions can work in
  Mergincanli/Shamble-Frontlines"; "Connectors: This plugin doesn't include any connectors."): the main GitHub
  connection, not a way past the proxy's 403.
- **Window opened: the reviewer off on `preview`** from his "unticked" (00:10:53 UTC). Step 2 (the reviewer bite):
  run #17 re-run by the session with rerun_workflow_run (201 Created, about 00:11 UTC): attempt 2; its deploy job must
  run without waiting and print "secret: present".
- **The reviewer bite (PM-0006-32's second row): run #17 attempt 2** (re-run 00:11:09 UTC): plan green
  (00:11:13-00:11:32), build green (00:11:34-00:11:48); the deploy job 135234549298 created 00:12:06, status "queued"
  (never "waiting"), started 00:12:13, completed 00:12:15, success; its log, word for word: "B9-MARKER: the rewritten
  deploy job ran at 2026-11-19T00:12:14Z" / "secret: present" ("TOKEN: ***" in the env block). So with the reviewer
  off, a rewritten job reaches the token without anyone's click. Next: his re-tick (the window closes at its
  screenshot), then SA-R4's re-run, which must wait and which he rejects.
- **His re-tick**: screenshot (2026-11-19 00:13:42 UTC, ed9f3bac-f2e0-4cf1-a61b-05a1fd4c4dc0), transcribed: banner
  "Environment "preview" updated."; "Required reviewers" ticked, Mergincanli listed; "Prevent self-review" unticked;
  "Wait timer" unticked; "Allow administrators to bypass configured protection rules" unticked; "No restriction";
  the two secrets, "yesterday". **Window closed: the reviewer was off from 00:10:53 to about 00:13:42 UTC** (his two
  messages), and only run #17's attempt 2 ran in it (the session pushed nothing else).
- SA-R4 (the re-tick proven by a wait): run #17 re-run again by the session (rerun_workflow_run, 201 Created, about
  00:13:54 UTC): attempt 3; its deploy job must wait on `preview`, and he rejects it.
- **SA-R8**: his screenshot of "Actions secrets and variables" (queued 2026-11-19 00:14:02 UTC, attachment
  7f9bc63e-b2cf-4669-9886-807ca00a3403), transcribed: "Environment secrets": CANARY (production, "3 days ago"),
  CLOUDFLARE_ACCOUNT_ID (preview, "yesterday"), CLOUDFLARE_API_TOKEN (preview, "yesterday"); "Repository secrets":
  "This repository has no secrets." Met: no secret outside the two environments.
- **SA-R4 measured: run #17 attempt 3** (re-run 00:13:52 UTC): plan green (00:13:56-00:14:11), build green
  (00:14:13-00:14:37); the deploy job 135235739456 created 00:14:38, read "queued" once in the jobs list, then
  "status":"waiting" (get_workflow_job, about 00:14:45 UTC). So after his re-tick a rewritten job waits again. His
  rejection asked next.
- His "rejected" (2026-11-19 00:15:52 UTC, 193acd50-d5b0-4df0-9bec-25a5e4c23951): attempt 3's deploy job completed
  00:15:49 UTC, conclusion "failure", runner_id 0 (it never reached a runner). SA-R4 done.
- Step 4 (the refusal): cs/b9-prod 9965630 pushed 00:16:15 UTC (deploy.yml = the approved variant 8b745631a278; the
  other three workflows unchanged); pull request #10 "Token test: cs/b9-prod, a pull-request job that names
  production" opened right after. Its job `b9-production` must be refused at once (production admits `main` only).
- **The refusal (PM-0006-32's third row): run #18** (44427671656, created 00:16:27 UTC): the job `b9-production`
  (135236608357) completed 00:16:31 UTC, conclusion "failure", no runner (refused before any step; no log); its check
  run's output empty. The public run page, logged out (headless Chromium, 2026-11-19 00:17:28 UTC, HTTP 200; picture
  `.gate/preview-hosting-check/day1/sitting3/run18.png`), "Annotations: 2 errors", word for word: "b9-production" /
  "Branch "refs/pull/10/merge" is not allowed to deploy to production due to environment protection rules." /
  "b9-production" / "The deployment was rejected or didn't satisfy other protection rules." So a pull-request job
  that names `production` is refused at once; nothing waited for him.
- His "open" with a screenshot (2026-11-19 00:19:35 UTC, 66a03064-486c-4491-9c5e-8bf4639d2ac9): banner "Environment changes successfully saved: all
  branches can deploy."; "Configure production": "Required reviewers" unticked, "Wait timer" unticked, "Allow
  administrators to bypass configured protection rules" unticked; "Deployment branches and tags": "No restriction";
  secret CANARY, "3 days ago". **Window opened: production's branch rule off.** Step 5: run #18's failed job re-run by
  the session (rerun_failed_jobs): "secret: present" expected.
- **The branch-rule bite (PM-0002-66's row, in PM-0006-32): run #18 attempt 2** (rerun_failed_jobs, about 00:19:45
  UTC): `b9-production` (135238126711) started 00:19:48, completed 00:19:51, success; its log, word for word:
  "B9-MARKER: the production job ran at 2026-11-19T00:19:49Z" / "secret: present" ("CANARY: ***"). So with
  production's branch rule set to "No restriction", a pull-request job reaches production's secret. His restore asked
  now (the window stays open until his screenshot shows `main`).
- **His restore**: screenshot (2026-11-19 00:21:51 UTC, c31773fa-3a4e-4172-b55f-1d158e4557f1), transcribed: banner
  "Deployment branch rule "main" saved successfully."; "Configure production": "Required reviewers", "Wait timer" and
  "Allow administrators to bypass configured protection rules" unticked; "Deployment branches and tags": "Selected
  branches and tags", "1 branch and 0 tags allowed", rule `main`, "Currently applies to 1 branch"; secret CANARY.
  **Window closed: production's branch rule was off from 00:19:35 to about 00:21:51 UTC**; only run #18's attempt 2
  ran in it. The measurement: run #18 re-run (rerun_workflow_run, 201, about 00:22:00 UTC): attempt 3, its
  `b9-production` must be refused again.
- **The measurement after the restore: run #18 attempt 3** (re-run about 00:22:00 UTC): `b9-production` (135239185962)
  created 00:22:02, completed 00:22:06, conclusion "failure", no runner; the public run page for attempt 3 (logged
  out, 00:22:39 UTC; picture `.gate/preview-hosting-check/day1/sitting3/run18-attempt3.png`), "Annotations: 2 errors",
  word for word: "Branch "refs/pull/10/merge" is not allowed to deploy to production due to environment protection
  rules." / "The deployment was rejected or didn't satisfy other protection rules." So with `main` back, the
  pull-request job is refused again.
- **Sitting 3 in short (PM-0006-32 and SA-R4, SA-R8):** reviewer on: the rewritten job waited, approved, "secret:
  present" (#17 attempt 1); reviewer off: no wait, "secret: present" (attempt 2); reviewer on again: it waited and
  was rejected (attempt 3, no runner); a pull-request job naming production: refused at once (#18 attempt 1); the
  branch rule off: "secret: present" (attempt 2); `main` back: refused again (attempt 3); no repository secrets. Two
  windows, each closed by his screenshot: the reviewer off 00:10:53-00:13:42 UTC, production's rule off
  00:19:35-00:21:51 UTC. State left: `preview` with the reviewer (Mergincanli) on, `production` with `main` only;
  pull requests #5 (c), #9 (cs/b9-edit) and #10 (cs/b9-prod) open, for Day 2.
- **The schedule fired once**: probe run #4, id 44413340706, event "schedule", created 2026-11-18 22:37:26 UTC (the
  22:23 slot, 14 minutes late), success 22:37:53; probe-results-44413340706 (artifact 14478546185, zip sha256
  f497337be4af..., equal to its digest), probed_at 2026-11-18T22:37:46.669Z, read with jq from
  `.gate/preview-hosting-check/day1/probe-4/`: (a) 2 rows gone (hours_since_close 4.08); (a') 3 gone (4.08); (b) 4
  gone (3.79); (b') 5 gone (3.78); (c) 2 serves; (d) 2 serves (4.08); the by-hand previews 19 serves; production 4
  serves; the rehearsal 2 gone (7.68); no row not as expected. The slots 15:23 to 21:23 UTC (seven) had no run; the
  23:23 slot had none by 2026-11-19 00:23 UTC (29 runs in all).
- **Sitting 3's runs against its list** (29 runs at about 00:23 UTC): run #17 (cs/b9-edit, three attempts: the
  control, the reviewer bite, SA-R4's rejected one), run #18 (cs/b9-prod, three attempts: refused, the branch-rule
  bite, refused), the by-hand probe #3 and the scheduled probe #4; nothing else.
- **Direct records (logged-out public pages, 2026-11-19 00:26-00:27 UTC, pictures in
  `.gate/preview-hosting-check/day1/sitting3/`):** run #17 attempt 1: "Deployment protection rules" / "Mergincanli" /
  "approved" / "preview"; attempt 2: no "Deployment protection rules" section at all (no approval event: nobody
  clicked); attempt 3: "Mergincanli" / "rejected" / "preview", annotation "The deployment was rejected or didn't
  satisfy other protection rules." **The environments' deployment history for the token tests**, as the pull request
  pages show it (the repository's deployments pages answer 404 when logged out): #9 (cs/b9-edit): "Mergincanli
  deployed to preview — with GitHub Actions Active" (twice), then "Mergincanli had a problem deploying to preview —
  with GitHub Actions Failure"; #10 (cs/b9-prod): "Mergincanli had a problem deploying to production — with GitHub
  Actions Failure", "Mergincanli deployed to production — with GitHub Actions Active", "Mergincanli had a problem
  deploying to production — with GitHub Actions Failure" (attempts 1, 2, 3 in that order).
- **PM-0006-32 clause by clause** (ADR 0006 :1379; plan.md:103 and :130):
  - "a test pull request ... whose deploy job is edited to print a marker gets no secret until the `preview`
    environment's required reviewer approves it (the job prints only whether the secret is set, never its value)":
    met (#17 attempt 1 waited 19:12:20 to 23:15, approved, "secret: present"; nothing printed but presence).
  - "a job in a pull request that names the `production` environment is refused by its `main`-only deployment-branch
    rule": met (#18 attempt 1, and again attempt 3 after the restore).
  - "2 of 2": both met. Control (after approval, "secret set"): met. Bite (reviewer removed, "secret set"): met
    (attempt 2, no approval event). The plan's added PM-0002-66 bite (the rule removed, "present"): met (#18 attempt 2).
  - Reported, not a pass condition: a production deploy with the `preview` environment's token is NOT refused: round-a's
    production leg (deploy job in `preview`) made production deployment 0354374e (environment "production") on
    2026-11-18 17:02:25 UTC, as part 13 expected; for OQ-0006-13.
  - Recorded: how the rule treats a pull request's merge ref: it judges the ref "refs/pull/10/merge" and refuses it
    ("Branch "refs/pull/10/merge" is not allowed to deploy to production ...").
  - Evidence asked: each run's log (in this file) and the environments' deployment history (the pull request pages
    above). Still open for Day 2: "Mustafa's revocation of the test token when the check ends" (the revoke and the
    `revoked` run).

### Day 2 (plan Part 3 D)
- **The schedule fired twice more overnight** (probe.yml's runs listed with the GitHub MCP tool, 2026-11-19 about
  13:38 UTC, total_count 6; zips downloaded 13:39 UTC, read with jq):
  - probe run #5, id 44457125198, event "schedule", head 347d5e0, created 2026-11-19 04:15:59 UTC, success 04:16:29;
    probe-results-44457125198 (artifact 14498371957, zip sha256 356c906a5d62..., equal to its digest), probed_at
    2026-11-19T04:16:20.864Z, `.gate/preview-hosting-check/day2/probe5/`: 43 rows. (a) 2 rows, gone 2
    (hours_since_close 9.72); (a') tombstone 3, gone 3 (9.72); (b) 4, gone 4 (9.43); (b') tombstone 5, gone 5 (9.43);
    (c) control 2, serves 2; (d) bite 2, serves 2 (9.72); the by-hand previews 19, serves 19; production 4, serves 4
    (its own address as expected; the one-off links of 0354374e, e9a37459 and de1c3e5e not judged); the rehearsal 2,
    gone 2 (13.32). Unexpected 0, no reading 0.
  - probe run #6, id 44533901175, event "schedule", head 347d5e0, created 13:29:10 UTC, success 13:29:43;
    probe-results-44533901175 (artifact 14534309293, zip sha256 00e8244c50ad..., equal to its digest), probed_at
    2026-11-19T13:29:36.470Z, `.gate/preview-hosting-check/day2/probe6/`: the same 43 rows with the same outcomes;
    hours_since_close (a) 18.95, (a') 18.95, (b) 18.65, (b') 18.65, (d) 18.94, the rehearsal 22.54. Unexpected 0, no
    reading 0.
  - The hourly schedule so far: 3 scheduled runs (created 2026-11-18 22:37:26, 2026-11-19 04:15:59 and 13:29:10 UTC)
    in the 23 slots from 2026-11-18 15:23 to 2026-11-19 13:23 UTC. The run data names no slot, so which slot each run
    stands for is not recorded.
- **PM-0006-1: the header checked again before the 24-hour reading** (2026-11-19 about 13:45 UTC):
  - The test project: its production link serves (probes #5 and #6: its own address "serves", expected "serves").
  - The test repository: `main` is 347d5e04487189be05dd2bc3a2c397bed8415e4b (git ls-remote, 2026-11-19 about 13:43 UTC):
    the merges of (b) #4 and (b') #8 on 2162db5, which differ from it only by `test-prs/pd-three.txt` and
    `test-prs/pdt-three.txt` (git diff --name-status 2162db5 347d5e0). The four workflows at 347d5e0, re-hashed:
    deploy.yml 29593af18d34, close.yml 3afdf389d636, probe.yml f24ad6e81daf, browsers.yml 6791076916ae, equal to the
    header. Remote branches: check-token, cs/b9-edit 7f3a684, cs/b9-prod 9965630, cs/pd-off, cs/pd-one, cs/pd-open,
    cs/pdt-one, main, setup.
  - The build-marker rule: unchanged (deploy.yml unchanged).
  - The machines and versions: ubuntu-24.04 for the probes (the 24-hour probe's image version to be read from its log);
    the browser rows of sitting 1: windows-2025 (image win25-vs2026 20261103.298.1), Chrome 156.0.8078.80, Edge
    156.0.4314.58, Firefox 159.0.1, WebdriverIO 10.1.0; Safari left the check by his decision (2026-11-18 18:21:30 UTC).
    **Missing: his Brave version** (the header's "every browser's version"): asked on Day 2's list, before the wall.
  - The token: last measured 2026-11-18 14:27 UTC (Pages Read and Pages Write, not_before null, expires_on
    2026-11-25T22:59:59Z); not measured again (no run uses it before (c)'s cleanup).
  - Settings changed during the check: `preview`'s reviewer off from his untick (2026-11-18 18:08 UTC) until his tick
    (between 18:51:11 and 19:12:20 UTC by the run records; seen on in his screenshot of 23:15:46 UTC), and again
    2026-11-19 00:10:53-00:13:42 UTC (closed by his screenshot); `production`'s branch rule off 00:19:35-00:21:51 UTC
    (closed by his screenshot). State by his last screenshots: `preview` with reviewer
    Mergincanli (self-review and bypass off) since 00:13:42 UTC; `production` with `main` only since 00:21:51 UTC.
