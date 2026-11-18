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

**The test repository (GitHub).**
- Mergincanli/shamble-preview-check, public; `main` 4a67b83b9ed672c81e1a6e3067f916841c02b4a9 (his README).
- The reviewed commit of `setup`: 1619164900553ee3e32339447b30865347711156 (the advisor agent's APPROVE, 2026-11-16,
  hand-back 09:02:47). After the merge, `main`'s merge commit is added here; it carries the same files and is what every
  later run uses.
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
