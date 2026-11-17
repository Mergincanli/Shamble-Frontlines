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
