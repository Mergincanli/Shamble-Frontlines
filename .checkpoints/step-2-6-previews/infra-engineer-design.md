# infra-engineer: step 2.6 design research (preview link per pull request)

Task from the main session (lane 2), 2026-11-25. Design and cite; build nothing. Write only this file.
Memory: none found for infra-engineer in this checkout (no `.claude/agent-memory*` folder), 2026-11-25.

## Plan
1. Repo reads (file:line): build plan :1021-1027; Part 15 :103, :170-180; toolchain.yml; gate.yml; ADR 0006 parts 1-3,
   13, OQ-0006-13, PM-0006-2, PM-0006-7, Workers paragraph; ADR 0002 deploy job, S3, PM-0002-66; #66 report (App. B,
   PHC-*, "What the check answers"); #67 report (ADR6-*, PHC-PREVIEW-NOINDEX, STEP26-WORKERS); STATUS_ARCHIVE blocks;
   DECISIONS (his words). Save a notes line per source.
2. Official docs (URL, words, date read 2026-11-25): GitHub Actions (pull_request, pull_request_target, workflow_run,
   environments + deployment branch rules, concurrency, download-artifact run-id/github-token, artifact retention,
   GITHUB_TOKEN permissions, security hardening "pwn request"); Cloudflare (API token permission groups and resource
   scoping for Pages, Pages limits 30-day rule, wrangler pages deploy / deployment delete, Pages headers, Pages
   preview noindex, Workers static assets + preview URLs, wrangler-action).
3. Findings D1..D8, one block each, saved as written.
4. Advisor call before final; then handback (< 70 lines).

## Notes per source (appended as read)
- REPO build plan `docs/reports/2026-11-17-build-plan-slice-1.md:1021-1027` (read 2026-11-25): step 2.6 runs when (1)
  the hosting check ended, (2) change-set 2 merged (a page to deploy), (3) the 2.3 chat's CI change-set merged ("the
  build job the deploy job hangs on"), (4) the deploy token sits in the `preview` environment with B9's interim
  position (production only from main, a reviewer on previews) and OQ-0006-13 answered. "an Opus chat builds step 2.6".
- REPO Part 15 `...-part-15.md:103` (lane 2 unit row): owns "the workflow file it adds for previews (a new file), the
  host's config files it adds, its runbook in `docs/`"; needs "the tested build handed over by CI, which is an edit to
  `toolchain.yml` that lane 1 asks of you"; must not touch "`gate.yml`, `toolchain.yml`, the gate's files, `crates/`,
  `packages/`" (note: `packages/` too, so a PHC-PREVIEW-NOINDEX build-output test inside packages/ is lane 1's).
  `:170-180` (15.4): gate.yml/toolchain.yml owner lane 1 (GATE-FOLLOW: "You ask me for each gate and CI edit with its
  exact text"); the preview workflow lane 2. `:164-168`: cross-lane asks go through one issue "Lane requests"; only
  `Mergincanli` comments count.
- REPO `.github/workflows/toolchain.yml` (main c1da1bd): `on: push, pull_request` (:13-15); `permissions: contents:
  read` (:17-18); concurrency `toolchain-${{ github.workflow }}-${{ github.ref }}` cancel-in-progress (:20-22); one job
  `toolchain` (:33), ubuntu-24.04, 15 min, `cache-mode: none` (:37); checkout pinned v7.0.1 SHA 3d3c42e5...
  persist-credentials false (:51-53); setup-node v7.0.0 SHA 82076278... (:55); the build runs inside the network cut
  (:132-197): `node packages/client/scripts/build.mjs --production --out-dir "$PWD/.gate/dist-production"` (:172) and
  the development build `--out-dir "$PWD/.gate/dist-development"` (:173), then boundary tests, tsc, Playwright on the
  development build (:194-196). NO upload-artifact step exists today: nothing leaves the job. Both dist folders exist
  only after the last step passes, so an upload step placed after :197 runs only if every test passed (default
  `if: success()`).
- REPO `.github/workflows/gate.yml` (main c1da1bd): `on: push, pull_request`; `contents: read`; jobs `gate` (gate.mjs
  checks 1-4, 5b-5d) and `smoke` (5a, legacy pages). No build of v2, no artifacts. Not the preview's source.
- REPO ADR 0006 `docs/adr/0006-infrastructure-operations.md` (main c1da1bd, read 2026-11-25):
  - part 1 (:128-173): host Cloudflare Pages, Workers static assets the alternative "which takes the same `_headers`
    file"; generated `_headers` carries the production header set on production, the preview header set on previews;
    COOP/COEP only on a preview built with the isolation flag on; "the preview Mustafa checks is built with it off";
    25 MiB per file; 500 builds/month Free; direct-upload count unanswered (PM-0006-7); STEP4-COUNT (b): "only the
    newest push of each pull request deploys (ADR 0002's concurrency cancel) and the month's deploys are counted"
    (:144-146). Pages' limits page quote of 2026-11-25 "Preview deployments older than 30 days are deleted, unless they
    are one of the project's 100 most recent deployments" -> follow-up ADR6-PAGES-RETENTION (:149-151). Workers
    paragraph (:159-173): workers.dev Preview URLs get noindex "but not to custom domain Preview URLs"; up to 100
    Previews per Worker on Free, oldest deleted; "Start new projects with Workers"; "a move would mean running the check
    again on Workers".
  - part 2 (:174-196): offline previews public but unlisted (noindex, branch-name links) unless unreleased art -> the
    access wall; every multiplayer preview behind the wall from the first (PREVIEW-WALL (a)); Pages' policy restricts
    all project deployments; covers previews not production.
  - part 3 (:197-214): 24 h after close the link no longer serves the build; weekly sweep 0 previews of closed PRs
    older than 7 days; build artifacts kept 7 days; CI failure artefacts 30 days; close and sweep jobs are ADR 0002's
    workflows (seam S3).
  - part 13 B9 (:569, :574-603): deploy token "select Account, Cloudflare Pages and Edit" ("whether it can be narrowed
    to one project was not read"), held only by environments `preview` and `production`; a `pull_request` run takes
    its workflow from the merge commit, so any branch can rewrite the deploy job and use the token; interim: (a)
    production only by its own job on pushes to main, env `production`, main-only deployment-branch rule, own token;
    (b) `preview` has a required reviewer "unless OQ-0006-13 puts production where the preview token cannot reach it";
    (c) the check's token revoked. B10 (:570): "Fork pull requests get no preview (R5-PREV-REC1), but any branch of this
    repository can change its own workflow (B9), so that code is treated as untrusted".
  - OQ-0006-13 (:1010-1027): options (a) reviewer on preview; (b) production in a second Cloudflare account with own
    token, preview without reviewer (not researched); (c) "a token limited to the preview project, if Cloudflare offers
    one (not read)". Decider: the Advisor with security-auditor; Mustafa for a second account. "What decides it: a read
    of GitHub's rules for reviewing pending deployments and of Cloudflare's token scopes".
  - OQ-0006-18 (:1120-1131): environment reviewers "only available for public repositories" on Free/Pro/Team.
  - PM-0006-2 revised (:1393): previews `X-Robots-Tag: noindex`; production none on its own addresses; production's
    one-off (hash) addresses: recorded, not judged (the check read noindex there on all 7 paths, written by no rule).
  - PM-0006-6 (:1397): close workflow `wrangler pages deployment delete ... --force` per deployment; cookie-less hourly
    probe 24 h; tombstone workaround.
  - PM-0006-7 revised (:1398): count unanswered, STEP4-COUNT (b) holds anyway.
  - PM-0006-32 (:1402): B9 marks (reviewer gate; production env refused by main-only rule; production deploy with the
    preview token "reported, not a pass condition").
  - Migration step 2 (:1515-1518): foundation previews "with B9's interim position: production deployed only from main,
    a reviewer on preview". Rollback (:1581-1583): move to Workers static assets, "only ADR 0002's deploy target
    changes"; Netlify the named fallback.
- REPO ADR 0002 part 10 (`docs/adr/0002-code-structure-build-toolchain.md:660-707`, Proposed; read 2026-11-25): shape
  P1: the `pull_request` workflow builds with no secret and uploads `dist` as an artifact kept 7 days; a separate deploy
  job (`needs: build`) in environment `preview` holds the token, never checks out or runs PR code, same-repo PRs only,
  permissions `contents: read`, `actions: read`, `deployments: write`, `pull-requests: write`, automatic package
  caching off, environment URL = preview link, one comment updated. "No `pull_request_target`, no `workflow_run`."
  Gap stated: a `pull_request` run takes its workflow from the PR's merge commit, so any branch can rewrite the deploy
  job and use its token; token account-scoped; `branch` option lets one deploy name any branch -> reviewer on
  `preview` until the baseline "unless OQ-0006-13 puts production where the preview token cannot reach it".
  Production: own job on pushes to main, from main's tested artifact, env `production`, own token, main-only rule.
  "The build generates the `_headers` file": production set, preview set (`X-Robots-Tag: noindex`), COOP/COEP only
  with the isolation flag; "after deploying, the job fetches the preview's HTML and `.wasm` and fails if a header is
  missing". Close job and weekly sweep are ADR 0002's workflows. Options table (:887-891): P3 "`workflow_run` deploys
  every pull request's artifact ... gets secrets and write tokens (R5-PREV-F3) ... rejected for now"; P4
  `pull_request_target` rejected (GitHub warns, R5-PREV-F2). PM-0002-66 (:1340): deploy job has no checkout of the PR
  head, no npm ci/cargo; an edited job gets no secret while the reviewer has not approved; a PR job naming `production`
  is refused by its branch rule. PM-0002-67 (:1341): header check passes, and fails when a header is removed.
  Migration step 6 (:1507-1512): "Done when: a same-repository pull request has its link within 5 minutes after the
  build job ends ... (median of 5 runs, R5-PREV-REC1 mark 1); the header check passes and fails when a header is
  removed (R5-PREV-REC4 mark 1); Mustafa opens the link in Brave and sees the empty scene".
  => Recommending `workflow_run` departs from part 10's P1 and its "no workflow_run" line: a decision for Mustafa (the
  ADR is Proposed; he signed it off only as the basis for the spikes; part 10 not accepted).
- REPO #66 report `docs/reports/2026-11-24-preview-hosting-check.md` (read 2026-11-25): "What the check answers"
  (:87-115): PREV-DELETE met through the cleanup job (links gone 3-22 min after close; still gone at 24.1-24.4 h and
  144 h); without the job the links still served at 24 h ("the host itself does not remove a closed pull request's
  build"); R5-PREV-OQ5 yes (View deployment on the PR page); R5-PREV-OQ6 yes (`https://:version.:project.pages.dev/*`
  matches branch aliases); approvals counted (sitting 1: 3, would have been 5; sitting 2 would have been 14); "A
  session cannot approve a deployment with its tools"; "A production deploy with the `preview` environment's token is
  not refused"; "Measured for step 2.6 (sitting 3): with no required reviewer, a pull request's job gets the token; a
  `main`-only branch rule refuses a pull request's job that names production, judging the ref `refs/pull/10/merge`."
  Follow-ups (:251-261) = STATUS_ARCHIVE :1563-1577 ids PHC-CLICKLESS, -LOCKFILE, -TOKEN-SETUP (verify answer, not a
  screenshot; "the form's single-date chip sets a one-day window"), -PROBES (named User-Agent), -DELETE ("delete with
  `--force` or the API; a weekly sweep on `main`"), -ACTIONS-READ ("`actions: read` reconsidered (SA-R2)"), -ERROR-PATH
  ("the deploy step's error path and the account printout ... the ID is masked when it equals the secret"),
  PHC-TR-END, PHC-SCRIPT-LOWS.
- REPO #67 report `docs/reports/2026-11-25-adr0006-header-row.md` and STATUS_ARCHIVE :1578-1617: PHC-PREVIEW-NOINDEX
  (Medium, test-engineer F1): "step 2.6's plan takes a build-output test that each preview mode's `_headers` carries
  the `noindex` rule and production's does not, with a bite; and, before any preview is served where the host adds no
  `noindex`, the preview expectation is read there" (no bite today "because the host adds one anyway"). STEP26-WORKERS
  = his WORKERS-NEXT (a) point. ADR6-PAGES-RETENTION: 30 days + 100 newest. Risk (:183-186): retention "could expire
  old previews of open pull requests after 30 days, unless they are among the 100 newest. Step 2.6's plan weighs it".
- REPO DECISIONS (read 2026-11-25): :1350 "For step 2.6's plan: I do not want to click for each preview. Design the
  previews without a click from me and show what keeps the token safe then. I decide on it with that plan." :449
  STEP4-COUNT (b) "Until someone shows a count, step 2.6 works as if uploads count: only the newest push of a pull
  request is deployed, and it counts its deploys per month." :152 WORKERS-NEXT (a) text. :461 "The CI edit that hands
  the tested build to the preview job is the CI Builder chat's: write the exact request for it in step 2.6's plan."
  :1758 GATE-FOLLOW "You ask me for each gate and CI edit with its exact text".
- REPO STATUS (:16-18): main protected (PR with `gate` and `smoke` green); "`toolchain` runs on every push and pull
  request but is not a required check (REQUIRED-CHECK, a finding for step 2.3)". Both ADRs Proposed (:9-11).
- REPO GLOSSARY §5 (:562-672): preview header set = production set + `X-Robots-Tag: noindex`; production set =
  `.wasm` as `application/wasm`, CSP later, no COOP/COEP; isolation flag off by default everywhere, on only for
  profiling builds; "The preview Mustafa checks in Brave carries the preview header set with the isolation flag off".
- REPO `packages/client/scripts/build.mjs` (main c1da1bd): flags `--wasm-only`, `--production`, `--out-dir` only
  (:229-233); grep finds no `_headers`, `noindex`, `X-Robots` or isolation flag anywhere in packages/client/scripts.
  => TODAY THE BUILD WRITES NO `_headers`. ADR 0002's generated header sets do not exist yet; `packages/` is lane 1's.
- REPO #66 Appendix B deploy.yml (at TR 347d5e0, :1399-1587): `on: pull_request [opened, synchronize, reopened]` +
  workflow_dispatch on main; same-repo guard `github.event.pull_request.head.repo.full_name == github.repository`;
  jobs plan (no secret) -> build (no secret; upload-artifact v7.0.2 SHA cf430e03..., retention 7) -> deploy (env
  `preview`, permissions contents read + actions read; download-artifact v8.0.2 SHA 9000827c...; "Refuse server code
  in the upload": `_worker.js`, `_routes.json`, `functions`; `npm install --prefix "$RUNNER_TEMP/wrangler" wrangler@4.156.0`
  (NO lockfile: PHC-LOCKFILE); `wrangler pages deploy dist --project-name "$PROJECT" --branch "$BRANCH" --commit-hash
  ... --commit-dirty=true`; WRANGLER_OUTPUT_FILE_PATH ndjson, `pages-deploy-detailed` record gives deployment_id, url,
  alias, environment) -> verify (no secret; tools/check-headers.mjs on both links).
- REPO #66 Appendix B close.yml (:1589-1691): `on: pull_request: types: [closed]`, same-repo guard, env `preview`;
  lists deployments by Cloudflare API `GET .../pages/projects/$PROJECT/deployments?env=preview&page=N&per_page=25`,
  selects by `deployment_trigger.metadata.branch` (raw or slug), `DELETE .../deployments/$id?force=true` oldest first;
  tombstone only for cs/pdt-*. NOTE: its trigger `pull_request` takes the workflow from the PR's merge commit, the
  same gap as the deploy job.
- WEB GitHub Docs, "Events that trigger workflows"
  https://docs.github.com/en/actions/reference/workflows-and-actions/events-that-trigger-workflows (read 2026-11-25):
  workflow_run: GITHUB_SHA "Last commit on default branch", GITHUB_REF "Default branch"; "This event will only trigger
  a workflow run if the workflow file exists on the default branch."; "The workflow started by the `workflow_run`
  event is able to access secrets and write tokens, even if the previous workflow was not."; "Running untrusted code
  on the `workflow_run` trigger may lead to security vulnerabilities." pull_request_target: GITHUB_SHA "Last commit on
  default branch", GITHUB_REF "Default branch"; "This event runs in the context of the default branch of the base
  repository"; same untrusted-code warning. pull_request: GITHUB_REF "PR merge branch
  `refs/pull/PULL_REQUEST_NUMBER/merge`"; "secrets are not passed to the runner when a workflow is triggered from a
  forked repository"; default types opened, synchronize, reopened; `closed` with a `merged` conditional.
  Sure: high (official page; the fetch tool quotes, a summariser in between).
- WEB GitHub changelog 2025-11-07 "Actions pull_request_target and environment branch protections changes"
  https://github.blog/changelog/2025-11-07-actions-pull_request_target-and-environment-branch-protections-changes/
  (read 2026-11-25): "The workflow file and checkout commit will always be taken from the repository's default branch,
  regardless of the pull request's base branch."; "environment branch protection rules will evaluate against the
  execution reference (`GITHUB_REF`), not the pull request head (`HEAD_REF`)."; "For `pull_request`, ... environment
  rules evaluate against `refs/pull/number/merge`."; "For `pull_request_target`, environment rules evaluate against the
  default branch."; effective "12/8/2025". Matches the check's sitting-3 reading (`refs/pull/10/merge` judged and
  refused). Sure: high.
- WEB GitHub changelog 2026-11-02 "pull_request_target blocked by default December 2"
  https://github.blog/changelog/2026-11-02-pull_request_target-blocked-by-default-december-2 (read 2026-11-25):
  "block `pull_request_target` events by default in all public repositories, unless an applicable Actions event policy
  explicitly allows them"; "Starting December 2, 2026"; "Private and internal repositories aren't affected"; to allow:
  "Explicitly allow `pull_request_target` in an applicable Actions event policy" / "Use workflow file targeting to
  allow specific workflows". workflow_run not mentioned. Sure: high that pull_request_target needs a repo setting
  (his click) from 2026-12-02; medium that workflow_run is untouched (not named = not covered, inferred).
- WEB GitHub changelog 2026-09-17 "Workflow execution protections in GitHub Actions generally available"
  https://github.blog/changelog/2026-09-17-workflow-execution-protections-in-github-actions-generally-available/
  (read 2026-11-25): "Execution protections let you define an allowlist that controls who can trigger an Actions
  workflow and what events can start it"; "For public repositories that do not already have an applicable event policy,
  GitHub is introducing a default rule that disables `pull_request_target`"; names only pull_request_target. Docs:
  docs.github.com/actions/how-tos/administer/control-workflow-execution. Possible use: an ACTOR rule limiting who can
  trigger workflows (not needed for the design; noted).
- WEB Cloudflare, "API token permissions" https://developers.cloudflare.com/fundamentals/api/reference/permissions/
  (read 2026-11-25; page metadata dateModified 2026-11-15): "Permissions are segmented into three categories based on
  resource:" (Zone, Account, User); "The applicable scope of account permissions is `com.cloudflare.api.account`.";
  "Cloudflare Pages Edit": "Grants access to create, edit and delete Cloudflare Pages projects." (also listed as "Pages
  Write"); listed under Account permissions. No sentence offers a Pages project as a resource scope; the only
  per-resource sentence: "The `AI Gateway Read`, `Run`, and `Edit` permissions are account-scoped only — they cannot be
  restricted to a single gateway." => On the docs read, a Pages token reaches EVERY Pages project of the account it
  names. Sure: medium-high (absence in docs; the form or the List permission groups endpoint would settle it; not
  something a session may call with his token).
- WEB Cloudflare, "Use Direct Upload with continuous integration"
  https://developers.cloudflare.com/pages/how-to/use-direct-upload-with-continuous-integration/ (read 2026-11-25):
  token: "Under **Custom Token**, select **Get started**."; "Under **Permissions**, select *Account*, *Cloudflare
  Pages* and *Edit*"; example uses "cloudflare/wrangler-action@v4" (a tag, not a SHA) with "command: pages deploy
  YOUR_DIRECTORY_OF_STATIC_ASSETS --project-name=YOUR_PROJECT_NAME"; "with the `contents: read` and `deployments:
  write` permission." No project-level restriction offered. Sure: high.
- WEB Cloudflare, "Create API token" https://developers.cloudflare.com/fundamentals/api/get-started/create-token/
  (read 2026-11-25): "select resource rules within the policy to specify which resources the permissions apply to";
  "Restrict how a token is used in the Token expiration and Client IP Address Filtering fields."; "Use the
  `/user/tokens/verify` endpoint to fetch the current status of the given token."; "The token secret page also includes
  an example command to test the token." (PHC-TOKEN-SETUP: end the setup with that verify answer). Client IP filtering
  is useless for GitHub-hosted runners (their addresses change; inferred). Sure: high on the quotes.
- WEB Cloudflare, "Account API tokens" https://developers.cloudflare.com/fundamentals/api/get-started/account-owned-tokens/
  (read 2026-11-25): account tokens "act as service principals with their own specific set of permissions", unlike user
  tokens that "act on behalf of a particular user"; compatibility matrix lists Pages as supported. Optional for us
  (his account is the only member). Sure: medium (matrix read through the fetch summariser).
- WEB GitHub Docs, "Deployments and environments"
  https://docs.github.com/en/actions/reference/workflows-and-actions/deployments-and-environments (read 2026-11-25):
  "Selected branches and tags: Only branches and tags that match your specified name patterns can deploy to the
  environment."; "The deployment branch or tag rule is matched against the `GITHUB_REF` of the workflow run.";
  "Wildcard characters will not match `/`"; "Name patterns must be configured for branches or tags individually.";
  "If the environment requires approval, a job cannot access environment secrets until one of the required reviewers
  approves it."; "Deployment branches and tags are available for all public repositories."; "Adding another branch
  rule for `refs/pull/*/merge` would also allow workflows triggered by `pull_request` events to deploy"; "Prevent
  self-review": "users who initiate a deployment cannot approve the deployment job". => With GITHUB_REF = default
  branch for workflow_run (events page), an environment limited to the branch `main` admits a workflow_run job and
  refuses every `pull_request` job (refs/pull/N/merge) and every push/dispatch on another branch. Sure: high (two
  official pages agree, and the check measured the refusal side on 2026-11-19).
- WEB actions/download-artifact README https://github.com/actions/download-artifact (read 2026-11-25): examples use
  `@v8`; `run-id`: "The id of the workflow run where the desired download artifact was uploaded from."; `github-token`:
  "This is required when downloading artifacts from a different repository or from a different workflow run.";
  "By default, the permissions are scoped so they can only download Artifacts within the current workflow run."; the
  example's comment: "token with actions:read permissions on target repo"; `path` default `$GITHUB_WORKSPACE`. No
  security note on untrusted artifacts. => In a workflow_run deploy, `actions: read` is NEEDED (PHC-ACTIONS-READ
  answered: keep it, and nothing else read-write beyond deployments/pull-requests). The repo's pins: download v8.0.2
  9000827ccba6bdab643e8b6fd33ac0654aef8333, upload v7.0.2 cf430e030ddbb5b0abf93d22962f4752f3646cd9 (#66 App. B).
  Latest tags not read (gh api refused outside this repo); the build re-reads them. Sure: high on quotes.
- WEB cloudflare/wrangler-action README https://github.com/cloudflare/wrangler-action (read 2026-11-25): "You can pass
  `wranglerVersion` to install a specific version of Wrangler from NPM."; "If you omit `wranglerVersion` and Wrangler
  is already installed in your environment, the action uses the existing installation. If Wrangler is not installed,
  the action installs a default version."; "The action now defaults to **Wrangler v4**."; outputs `deployment-url`,
  `pages-deployment-alias-url` ("since Wrangler v3.78.0"), `command-output`. Release number not shown on the page.
  => The action itself installs Wrangler from npm at run time (no lockfile for its dependencies): it adds a layer, it
  does not remove PHC-LOCKFILE's problem. Sure: high on quotes; medium on "no lockfile" (inferred from "install ...
  from NPM").
- WEB Cloudflare, Wrangler commands for Pages https://developers.cloudflare.com/workers/wrangler/commands/pages/ (read
  2026-11-25): `wrangler pages deploy [DIRECTORY]` "The directory of static files to upload"; `--branch` "The name of
  the branch you want to deploy to"; `--commit-hash`, `--commit-message`, `--commit-dirty`; `--no-bundle` "Whether to
  run bundling on `_worker.js` before deploying"; `wrangler pages deployment delete <DEPLOYMENT-ID>` with `--force`:
  "Delete even if the deployment has an active alias"; `pages deployment list --environment`. Global `--config`
  "Path to Wrangler configuration file". Sure: high.
- WEB Cloudflare, Pages limits https://developers.cloudflare.com/pages/platform/limits/ (read 2026-11-25): builds
  "1 build at a time" and "500" a month on Free, no word on direct uploads; "Deployments are kept for 30 days.";
  "Preview deployments older than 30 days are deleted, unless they are one of the project's 100 most recent
  deployments."; "Deployments always kept": "100 most recent, plus your current production deployment"; "up to 20,000
  files on the Free plan"; "The maximum file size for a single Cloudflare Pages site asset is 25 MiB."; "a limit of
  100 projects" per account; "A `_headers` file can have a maximum of 100 header rules."; "An individual header ...
  maximum of 2,000 characters." Sure: high.
- WEB Cloudflare, Pages preview deployments https://developers.cloudflare.com/pages/configuration/preview-deployments/
  (read 2026-11-25): hash URL "`373f31e2.user-example.pages.dev`"; "Pages also creates an alias for `git` branch's
  name" and "updates it so that the alias always maps to the latest commit of that branch."; "Branch name aliases are
  lowercased and non-alphanumeric characters are replaced with a hyphen"; "By default, every preview deployment
  generated by Cloudflare Pages includes the X-Robots-Tag: noindex HTTP response header."; hash addresses "keep pointing
  to the same deployment until it is deleted"; "The latest deployment for a branch cannot be deleted." (without force:
  the check deleted it with `force=true`); "By default, these deployment URLs are public."; "Enabling the access policy
  will restrict viewing project deployments to your Cloudflare account." What a branch alias does when its last
  deployment is deleted by the 30-day rule: NOT stated (the check saw 404/gone after its own forced deletes:
  inferred to be the same). Sure: high on quotes; low on the alias-after-expiry behaviour.
- WEB Cloudflare, Pages Functions advanced mode https://developers.cloudflare.com/pages/functions/advanced-mode/
  (read 2026-11-25): "make sure your `_worker.js` file is placed in your Pages' project output directory."; "your
  Function will assume full control of all incoming HTTP requests to your domain." => a PR-built artifact holding
  `_worker.js` would put server code on the preview host: the deploy job refuses `_worker.js` (file or folder),
  `_routes.json` and runs wrangler from an empty working folder with no `functions/` (as the check's deploy job did,
  #66 App. B). Sure: high on quotes; medium that wrangler reads `functions/` from its working folder (not stated on the
  commands page).
- WEB Cloudflare, Pages headers https://developers.cloudflare.com/pages/configuration/headers/ (read 2026-11-25): the
  file goes "in the static asset directory of your project"; block syntax `[url]` then indented `[name]: [value]`;
  example "`https://:version.:project.pages.dev/*` / `X-Robots-Tag: noindex`" "to prevent your `*.pages.dev` and
  `*.*.pages.dev` URLs from being indexed"; detach with "! Header-Name"; "Custom headers defined in the `_headers` file
  are not applied to responses generated by Pages Functions."; "You may define up to 100 header rules." Sure: high.
- WEB GitHub Security Lab, "Keeping your GitHub Actions and workflows secure Part 1: Preventing pwn requests"
  (2021-08-03) https://securitylab.github.com/resources/github-actions-preventing-pwn-requests/ (read 2026-11-25):
  "the untrusted code must be handled via the `pull_request` trigger so that it is isolated in an unprivileged
  environment."; "The workflow processing the PR should then store any results ... in artifacts and exit."; "The
  following workflow then starts on `workflow_run` where it is granted write permission to the target repository and
  access to repository secrets"; "Incoming data from artifacts is potentially untrusted."; "Artifacts resulting from
  untrusted PR data are themselves untrusted"; "it would be a security vulnerability to run them in the privileged
  `workflow_run` workflow context."; example comment "# do not extract in the workspace dir that may contain executable
  scripts". Sure: high (GitHub's own security team; older than the 2025 ref change, which does not touch workflow_run).
- WEB GitHub Docs, "Secure use reference" https://docs.github.com/en/actions/reference/security/secure-use (read
  2026-11-25): "Pin references to actions to a commit SHA in your workflow files."; "When selecting a SHA, you should
  verify it is from the action's repository and not a repository fork."; a lockfile "supersedes SHA pinning for
  repositories that adopt it" (not read further); "Workflows triggered on `workflow_run` should treat artifacts
  uploaded from other workflows with caution."; "Workflows that use these triggers must not explicitly check out
  untrusted code"; untrusted input via an intermediate environment variable so it "doesn't interact with the script
  generation process"; "set the default permission for the `GITHUB_TOKEN` to read access only for repository
  contents" and raise per job. Sure: high.
- WEB GitHub Docs, "Workflow syntax" https://docs.github.com/en/actions/reference/workflows-and-actions/workflow-syntax
  (read 2026-11-25, first 100k chars): concurrency "To also cancel any currently running job or workflow in the same
  concurrency group, specify `cancel-in-progress: true`."; "By default, any existing `pending` job or workflow in the
  same concurrency group will be canceled" and "the new queued job or workflow will take its place."; "Since the
  actual start time of a job or run may vary, ordering is not guaranteed." (=> a head-SHA check before deploying, not
  concurrency alone); on.workflow_run `workflows` "you can specify which workflows can trigger your workflow",
  `branches` "what branches the triggering workflow must run on". Sure: high.
- REPO `packages/client/scripts/build.mjs:168-186`: the page's corner label is `git rev-parse --short=9 HEAD` (+
  "+changes"), set as VITE_SHAMBLE_COMMIT. In a `pull_request` run HEAD is the merge commit refs/pull/N/merge, so a
  preview built there shows the MERGE commit, not the PR head he is asked to approve. (inferred from checkout's
  default for pull_request; sure: medium-high)
- REPO activity, `git log origin/main` 2026-10-26..2026-11-26: 38 merges on main's first-parent line; 332 non-[skip ci]
  commits (907 checkpoint commits carry [skip ci], so they start no workflow). Pushes per PR not counted; at 3 to 8
  CI pushes per PR a month is roughly 110 to 300 deploys (estimate, not a count).
- WEB Cloudflare, Workers "Version URLs" (formerly preview URLs) https://developers.cloudflare.com/workers/configuration/previews/
  (read 2026-11-25): "`npx wrangler versions upload --preview-alias staging`"; URL "`<alias>-<worker-name>.<subdomain>.workers.dev`";
  "Only the 1000 most recently deployed aliases are retained."; "Aliases must use only lowercase letters, numbers, and
  dashes." "must begin with a lowercase letter"; alias + worker name "must not exceed 63 characters". Sure: high.
- WEB Cloudflare, Workers "Previews" https://developers.cloudflare.com/workers/previews/ (read 2026-11-25): "Previews
  give each branch an isolated, production-like environment under the same Worker."; "Running `npx wrangler preview`
  creates or updates a Preview for your current branch under the same Worker."; URL
  "<preview-name>-<worker-name>.<subdomain>.workers.dev"; "Previews per Worker | 100 | 500" (Free | Paid);
  "Deployments per Preview | 100 | 100"; "the Preview that was deployed to least recently is deleted"; delete "npx
  wrangler preview delete --name <preview-name>"; no retention in days; noindex on the workers.dev URL. Static assets /
  `_headers` on Previews: not stated on this page (ADR 0006 part 1 cites static-assets/headers for `_headers`). Sure:
  high on quotes; flags of `wrangler preview` for CI (branch name) not read.
- WEB actions/upload-artifact README https://github.com/actions/upload-artifact (read 2026-11-25): `retention-days`
  "Minimum 1 day." "Maximum 90 days unless changed from the repository settings page."; `if-no-files-found` default
  "warn"; `include-hidden-files` default "false": "Hidden files are defined as any file beginning with `.` or files
  within folders beginning with `.`."; "Artifact names must be unique"; "Artifacts created by upload-artifact@v4 are
  immutable." => toolchain's build lands in `.gate/dist-production/`, a folder under a dot-folder: whether the default
  skips it is not stated. The request sets `include-hidden-files: true` and `if-no-files-found: error`, or copies the
  folder to a plain path first. Sure: high on quotes; the `.gate` behaviour unknown (the first run proves it).
- WEB GitHub Docs events page, schedule and workflow_run sections (same URL, read 2026-11-25): "The `schedule` event
  can be delayed during periods of high loads of GitHub Actions workflow runs."; "If the load is sufficiently high
  enough, some queued jobs may be dropped."; "Scheduled workflows will only run on the default branch." (GITHUB_REF
  default branch); "The shortest interval you can run scheduled workflows is once every 5 minutes."; "In a public
  repository, scheduled workflows are automatically disabled when no repository activity has occurred in 60 days.";
  workflow_run types `completed`, `requested`, `in_progress`; "A workflow run is triggered regardless of the
  conclusion of the previous workflow." (so the job tests `github.event.workflow_run.conclusion == 'success'`). The
  check's own hourly probe: "3 scheduled runs in the 24 slots" (#66 report :28) = a schedule alone cannot carry the
  24 h close mark. Sure: high.
- REPO #66 Setup header (:343-404): project made "by his drag-and-drop upload of a placeholder page (plan A4), so
  production's first version is his"; production branch `main` (records :603); token "an Account API token ... "Pages
  Write" only (the form: "Developer Platform" 1/62, "Pages", "Edit" ticked ...), scope "Entire Account", client IP
  filtering empty" (the form offered the account as the scope: evidence, with the docs, that no per-project scope is
  on offer; sure: medium-high); later "Pages Read" and "Pages Write" (TOKEN-READ (a)); the single-date chip set a
  one-day window (measured not_before/expires_on). Environments: `preview` reviewer + "Deployment branches and tags":
  "No restriction"; `production` "Selected branches and tags", one rule, branch `main`; "Repository secrets": none;
  Actions settings: "Require actions to be pinned to a full-length commit SHA" ticked, workflow permissions "Read
  repository contents and packages permissions". The Claude app (SR-READ): "Read and write access to actions, checks,
  code, ... pull requests, ... and workflows"; "No write on administration or deployments" => a session cannot change
  an environment's branch rule or approve a deployment. Token check (:481-600): account token verify
  `/accounts/$a/tokens/verify` HTTP 200 `"status": "active"` with expires_on/not_before; `/user/tokens/verify` 401 for an
  account token; "adding Read did not help" (the failure was the window).
- REPO STATUS :17: main protected (PR, gate + smoke green). Whether admins may bypass main's rule: NOT READ (no doc
  says; the API needs admin). The design's wall is main's review, so this matters (see D1 residuals).
- QUOTE NOTE (for the verifier): the fetch tool returns quotes of at most 125 characters. The 2025-11-07 changelog's
  long sentences above are FRAGMENTS JOINED by me (re-fetched 2026-11-25: "The workflow file and checkout commit will
  always be taken from the repository's default branch," + "regardless of the pull request's base branch."; "evaluate
  against `refs/pull/number/merge`"; "For `pull_request_target`, environment rules evaluate against the default
  branch." is whole). Cite the fragments, not my joins.
- REPO root `package.json`: npm workspaces `packages/*` only; `tools/` already has its own package.json and lockfile,
  installed with `npm ci --prefix tools --omit=optional --ignore-scripts` (gate.yml:34) and `npm ci --prefix tools
  --ignore-scripts` (gate.yml:65): a precedent for a separate locked tool folder outside the workspaces.
- WEB GitHub REST "Get a workflow run" https://docs.github.com/en/rest/actions/workflow-runs#get-a-workflow-run (read
  2026-11-25): schema fields "`path`: required, string", "`head_sha`: required, string", "`head_branch`: required,
  string or null", "`event`: required, string", "`conclusion`: required, string or null", `head_repository` with
  "`full_name`: required, string", "`pull_requests`: required, array of `Pull Request Minimal` or null". => the job can
  require `workflow_run.path == '.github/workflows/toolchain.yml'` (the webhook payload is taken to carry the same
  object: inferred, medium). No word on `pull_requests` for forks.
- WEB Cloudflare, Pages custom domains https://developers.cloudflare.com/pages/configuration/custom-domains/ (read
  2026-11-25): for a zone in the same account "the CNAME record will be added automatically after you confirm your DNS
  record"; an apex custom domain "must be a zone on the Cloudflare account you have created your Pages project on".
  => production on shamblefrontlines.com's apex must live in the account that holds the zone (his main account); if
  tokens are ever to be separated by account, it is the PREVIEWS that move to a second account. Whether a Pages-only
  token can attach a custom domain of a same-account zone through the API: not read. Sure: high on quotes.

## Findings

### D1. Token safety without his click (recommendation, sure: high on the mechanism, medium on the first run)
Recommend: the preview workflow starts on `workflow_run` (workflows: [toolchain], types: [completed]) and holds the
token in environment `preview`, whose "Deployment branches and tags" is "Selected branches and tags" with ONE branch
rule, `main`, with NO required reviewer and "Allow administrators to bypass" unticked; the token and account id are
environment secrets of `preview` only, never repository secrets.
Why it keeps the token from code a pull request can change, with no click:
1. The rule is "matched against the `GITHUB_REF` of the workflow run" (GitHub, Deployments and environments). A
   `workflow_run` run has GITHUB_REF "Default branch" and its file must "exist on the default branch" (events page),
   so the only workflow text that can reach the token is main's, behind main's protection (PR + gate + smoke).
2. Every workflow a branch can add or rewrite runs on a ref the rule refuses: `pull_request` judges
   `refs/pull/N/merge` (changelog 2025-11-07; MEASURED by the check, sitting 3: "a `main`-only branch rule refuses a
   pull request's job that names production, judging the ref `refs/pull/10/merge`", #66 :114-115); `push` and
   `workflow_dispatch` on a branch judge that branch. So a branch that edits the deploy job, or adds a new workflow
   naming `preview`, gets no secret: the same measured refusal, now on `preview`.
3. Sessions cannot loosen it: the Claude app holds "No write on administration or deployments" (#66 :396-398), and
   "A session cannot approve a deployment with its tools" (#66 :106-107).
4. Nothing of the pull request runs where the token is: no checkout of PR code, no install from the PR; the build
   arrives as an artifact and is data (GitHub Security Lab 2021: "Incoming data from artifacts is potentially
   untrusted"; Secure use: "treat artifacts uploaded from other workflows with caution").
Job split (advisor's point; keeps write scopes and the token apart, and keeps skipped runs out of the counter):
- `check` (no environment, no secret; permissions contents read, actions read, pull-requests read): proceed only if
  `workflow_run.event == 'pull_request'`, `conclusion == 'success'`, `head_repository.full_name == github.repository`
  (fork guard), `workflow_run.path == '.github/workflows/toolchain.yml'`; look the PR up by API: open, same repo,
  `head.sha == workflow_run.head_sha` (else a newer push exists: skip); PR number must match `^[0-9]+$`, SHA
  `^[0-9a-f]{40}$`; every PR-derived string reaches shell only through `env:` (Secure use). Download `preview-site`
  by `run-id: ${{ github.event.workflow_run.id }}` with `github-token` (needs `actions: read`: download-artifact
  README; answers PHC-ACTIONS-READ: keep it, in this job only) into `$RUNNER_TEMP`, never the workspace (Security
  Lab's example). Refuse: symlinks, any path segment starting with `.`, `_worker.js` (file or folder; advanced mode
  "will assume full control of all incoming HTTP requests"), `_routes.json`, `functions/`, `_redirects`, more than
  20,000 files, any file of 25 MiB or more (Pages limits). Delete the artifact's own `_headers`, put main's preview
  header file in its place (D5), check it (PHC-PREVIEW-NOINDEX), upload `site-checked` (retention 1 day).
- `deploy` (environment `preview`; permissions contents read only): sparse checkout of main's host folder (the
  lockfile), `npm ci --ignore-scripts` there, download `site-checked` from this same run (no extra permission:
  "By default ... only download Artifacts within the current workflow run"), Cloudflare's verify call (PHC-TOKEN-SETUP),
  the one wrangler command (D2) from an empty working folder, branch `pr-<N>` only (refuse anything else).
- `after` (no secret; permissions pull-requests write, deployments read): header check on both links (D5), the month's
  count (D3), one PR comment updated in place (link, head SHA, the page's commit label, the count). GitHub's
  "View deployment" probably does NOT show on the PR here: the job-created deployment carries GITHUB_SHA = main's
  last commit, not the PR head (inferred from the events table; the check's PM-0006-10 "yes" was on a `pull_request`
  run). Hence the comment.
Why not the others:
- `pull_request` + fork guard, no reviewer: the check measured "with no required reviewer, a pull request's job gets
  the token" (#66 :114); for the environment to admit it the rule must include `refs/pull/*/merge` (GitHub's own
  words), so any branch can rewrite the job and take the token. Safe only if bounded (option b in the decision).
- `pull_request_target`: its workflow and ref are main's (changelog: "The workflow file and checkout commit will always
  be taken from the repository's default branch,"), so it is safe only while it runs nothing of the PR; a deploy needs
  the PR's build, and building it there is the "pwn request" (Security Lab: "the untrusted code must be handled via
  the `pull_request` trigger"); waiting for toolchain's artifact from it is a race. And from 2026-12-02 GitHub will
  "block `pull_request_target` events by default in all public repositories, unless an applicable Actions event policy
  explicitly allows them" (changelog 2026-11-02): one more setting of his.
- A reviewer on `preview` (B9 interim): "a job cannot access environment secrets until one of the required reviewers
  approves it" works, but costs his click per preview (#66 counted 5 and 14 per sitting had it stayed on).
Production and OQ-0006-13: the check found "A production deploy with the `preview` environment's token is not
refused"
(#66 :107-108). On Cloudflare's docs read today a Pages token is account-wide ("The applicable scope of account
permissions is `com.cloudflare.api.account`"; the check's form showed scope "Entire Account"), so a token limited to
one project is NOT on offer (sure: medium-high; absence, not a sentence) and two projects in ONE account separate
nothing. Only a second Cloudflare account separates tokens. Today no production project exists, so nothing of
production is in reach; the preview project's own production slot holds his placeholder and the workflow refuses every
branch but `pr-<digits>`. At the first release (OQ-0006-14) production's home is decided; because an apex custom domain
"must be a zone on the Cloudflare account you have created your Pages project on", the clean split is previews moving
to a previews-only account, not production.
Residuals (say them to him): (1) main is the wall: a reviewed but bad change to the preview workflow, or a direct push
to main if his rule lets admins bypass (not read), reaches the token; (2) supply chain: wrangler and its locked
dependencies and the pinned actions run with the token; a compromised release could steal it and deploy anything to
any Pages project of the account (today: previews and the placeholder); (3) the artifact is PR-controlled data: a PR
runs its OWN copy of toolchain.yml (pull_request takes the merge commit's workflow), and `workflows:` matches by name,
so "the tested build" means "the build that PR's checks passed"; the path check narrows the name match; the header file
comes from main, never from the artifact; a PR can still put any static page on its own preview link (only branches
of this repository: forks are refused).
Departures this creates (decisions, not findings): ADR 0002 part 10 says "No `pull_request_target`, no
`workflow_run`" and its options table rejects P3 ("`workflow_run` ... gets secrets and write tokens (R5-PREV-F3)");
the build plan's condition (4) names "a reviewer on previews"; OQ-0006-13's deciders are "the Advisor, with
security-auditor". The danger P3 names is real only if PR code runs in the privileged job, which this design forbids.

### D2. The deploy step, small and movable (sure: high on the command; medium on --ignore-scripts)
Recommend: plain Wrangler from a committed lockfile in lane 2's host folder (e.g. `hosting/deploy/package.json` and
`package-lock.json`, Wrangler pinned exactly; 4.156.0 is the version the check ran, a newer one is re-read at build
time), installed in the deploy job from MAIN's sparse checkout with `npm ci --prefix hosting/deploy --ignore-scripts
--no-audit --no-fund` (the tools/ precedent, gate.yml:34, :65; PHC-LOCKFILE). The step is one command:
`"$WRANGLER" pages deploy "$SITE" --project-name "$PROJECT" --branch "pr-$PR" --commit-hash "$HEAD_SHA"
--commit-message "pr-$PR run $RUN" --commit-dirty=true`, run from an empty folder, with WRANGLER_OUTPUT_FILE_PATH for
the `pages-deploy-detailed` record (url, alias, deployment_id), as the check did (#66 App. B :1544-1556).
Not `cloudflare/wrangler-action`: it installs Wrangler "from NPM" at run time ("If Wrangler is not installed, the action
installs a default version"), so it adds a pinned layer without locking Wrangler's own dependencies; Cloudflare's own
example pins it by tag (`@v4`), not by SHA. Not `npx wrangler@<pin>`: npx resolves the dependency tree fresh each run
and runs install scripts. Every action stays pinned by full SHA ("Pin references to actions to a commit SHA", Secure
use), checkout v7.0.1 3d3c42e5..., download v8.0.2 9000827c..., upload v7.0.2 cf430e03... (the repository's and the
check's pins; latest tags not re-read here).
Unproven: that Wrangler's `pages deploy` works with install scripts off (its esbuild and workerd dependencies have
install scripts; the check ran an unlocked `npm install` with scripts on). If it fails, the security-auditor chooses
between scripts on for this one locked tree and another route; the first run shows it.
Error path (PHC-ERROR-PATH): on failure print Wrangler's error lines and the account id's first and last 4
characters (the check's masking confusion), never the token; the verify call first prints `status`, `not_before`,
`expires_on` (account tokens answer at `/accounts/<id>/tokens/verify`, HTTP 200 "active"; `/user/tokens/verify`
gives 401 for them: #66 :498-510).
Branch name `pr-<N>`, not the git branch: the alias is "lowercased and non-alphanumeric characters are replaced with
a hyphen" (Pages previews page), long session branch names would make long labels, and `pr-<digits>` can never equal
the production branch, so a crafted branch name cannot steer a deploy to production.
A move to Workers, in two lines (his WORKERS-NEXT (a); sure: medium, `wrangler preview`'s CI flags not read):
1. Command and config: `pages deploy ... --branch pr-N` becomes `wrangler preview --name pr-N` (Previews: "creates or
   updates a Preview for your current branch under the same Worker"; delete: "npx wrangler preview delete --name
   <preview-name>") with a new `wrangler.jsonc` naming the Worker and its assets folder; the token takes Workers
   Scripts Edit instead of Pages Edit; links become `pr-N-<worker>.<subdomain>.workers.dev`; the cleanup's two Pages
   API calls become that delete command; the rest of the workflow (check, after, counter) does not change.
2. Headers and proof: the same `_headers` file (ADR 0006 part 1: Workers static assets "takes the same `_headers`
   file"), noindex by the host only on workers.dev Preview URLs; Previews keep "100" per Worker on Free and delete
   "the Preview that was deployed to least recently" (no 30-day rule); ADR 0006 part 1: "a move would mean running the
   check again on Workers": one sitting of his and about two calendar days, as the check took.

### D3. Only the newest push deploys; the month's deploys counted (sure: high on the mechanism; medium on the counter)
Only the newest push (STEP4-COUNT (b)), four layers, none needing a store:
1. toolchain's own concurrency (`toolchain-${{ github.workflow }}-${{ github.ref }}`, cancel-in-progress,
   toolchain.yml:20-22) cancels a PR's older run when a newer push arrives; a cancelled run ends "cancelled" and the
   preview's `check` job requires `conclusion == 'success'` ("A workflow run is triggered regardless of the conclusion
   of the previous workflow", events page).
2. The upload in toolchain runs for ONE event only (D6), so a push to a PR branch makes one artifact, not two (toolchain
   runs on both `push` and `pull_request`, toolchain.yml:13-15).
3. The `check` job compares the PR's current head with `workflow_run.head_sha` and skips if they differ (concurrency
   alone is not enough: "ordering is not guaranteed", workflow syntax page).
4. The deploy job's concurrency group per PR head branch (`preview-${{ github.event.workflow_run.head_branch }}`, used
   only as a key, never in shell), cancel-in-progress true: a newer deploy of the same PR cancels an older one still
   running. Cleanup jobs use their own group with cancel-in-progress false (a pending cleanup must not cancel a
   pending deploy of another PR: "any existing `pending` job or workflow in the same concurrency group will be
   canceled").
The counter, kept in GitHub's own records (no file, no store of ours): every job with `environment:` makes a GitHub
deployment record, so the month's deploys = `GET /repos/{owner}/{repo}/deployments?environment=preview` filtered to
the current UTC month (`deployments: read` in the `after` job), printed in each PR comment and the job summary next to
Pages' "500" builds a month (whether uploads count: not answered, PM-0006-7). Two choices:
(a) cleanup jobs use a second environment `preview-cleanup` (same main-only rule, same token pasted twice): `preview`
    then counts deploy attempts exactly (a failed upload still counts: an upper bound by those few); recommended.
(b) one environment: the figure is deploys plus cleanup runs (about 40 closes and 4 to 30 sweeps a month), an upper
    bound; one fewer environment for him to make.
Not Cloudflare's own list: the cleanup deletes deployments, so it undercounts. Expected volume, an estimate not a
count: 38 merges on main in the last 30 days and 332 commits without [skip ci]; at 3 to 8 CI pushes per PR, about 110
to 300 deploys a month, under 500 but not far at the top end. Recommend a warning in the comment from 400, no stop
(what Pages does at 500, if uploads count, is not stated; a stop would block his checks).

### D4. Cleanup (sure: high on deletion; low on the 30-day rule's reading)
Close and merge: the check's way, measured: list the project's preview deployments by API (`GET
.../pages/projects/$PROJECT/deployments?env=preview`, paginated) and `DELETE .../deployments/$id?force=true` (or
`wrangler pages deployment delete <id> --force`: "Delete even if the deployment has an active alias"); with it every
link of a closed PR was gone "3 to 22 minutes after the close" and still at 24.1-24.4 h and 144 h; without it the
links "still served after 24 hours" (#66 :87-93). PHC-DELETE answered: force, through the API.
How it starts without a click and without the token in PR-changeable code: the check's close job ran on
`pull_request: [closed]` with `preview` (#66 App. B :1599-1615): under the main-only rule that job would now be
refused (it judges `refs/pull/N/merge`). So:
- one RECONCILE job (environment `preview`, or `preview-cleanup` per D3 (a); main's file only) that needs no data from
  the PR: a no-secret job first lists the OPEN pull requests by API and passes their numbers as a job output; the
  reconcile job lists Pages preview deployments and deletes, with force, every deployment whose branch is `pr-<N>` for
  an N not open, and every older deployment of an open PR (keeps the newest per PR). Idempotent; forged input can at
  most make it delete a closed PR's leftovers.
- started three ways: (1) right after each deploy (same run); (2) on `workflow_run` of a tiny no-secret workflow
  `preview-closed` on `pull_request: types: [closed]` (one echo step; its text is PR-controlled but holds nothing:
  a PR that removes it only delays its own cleanup to (3)); (3) a daily schedule on main. A schedule alone cannot
  carry the 24 h mark: "The `schedule` event can be delayed ... some queued jobs may be dropped" (events page), and the
  check's hourly probe got "3 scheduled runs in the 24 slots" (#66 :28).
- The weekly sweep (ADR 0006 part 3: "0 previews of closed pull requests older than 7 days"; PHC-DELETE "a weekly sweep
  on `main`"): the daily schedule is that sweep, run more often. Not `pull_request_target` (D1).
The 30-day rule (Pages limits, read 2026-11-25: "Preview deployments older than 30 days are deleted, unless they are
one of the project's 100 most recent deployments"; also "Deployments always kept": "100 most recent, plus your current
production deployment"):
- An open PR with no new push for 30 days: its newest preview is older than 30 days, so it survives only if it is among
  the project's 100 most recent deployments. Which deployments that ranks (only those still present, or every one
  ever made) is NOT stated: unknown. If only present ones: the reconcile keeps about one deployment per open PR, so an
  idle PR's link never expires. If all ever made: at 110-300 deploys a month an idle PR falls out of the 100 in about
  10 to 27 days and is deleted after day 30.
- Whether the link then breaks: Pages says a hash link works "until that deployment is deleted"; what a branch alias
  does when its last deployment is gone is not stated. The check's own forced deletes left the alias gone (no build
  served), so expect the `pr-<N>` link to stop serving (inferred).
- The fallback: the artifact lives 7 days, so a refresh needs a rebuild: re-run the PR's toolchain run (a session can,
  it has actions write: #66 :396-397), which deploys again. Recommend: the daily reconcile posts one line on an open PR
  whose newest preview is older than 25 days ("this link may stop working after day 30; push or re-run the checks"),
  no automatic rebuild (it would spend deploys and CI minutes on idle PRs). On Workers the rule is a count instead (100
  Previews, least recently deployed deleted).
Artifacts: toolchain's upload `retention-days: 7` (ADR 0006 part 3; minimum 1, maximum 90: upload-artifact README); the
`site-checked` copy 1 day. The check's deployment records were kept 14 days (#66 App. B); step 2.6 keeps none beyond
the run's log and the PR comment.

### D5. Headers (sure: high that the build writes none today; medium on the bite header)
Fact: the build writes no `_headers` today (no `_headers`, `noindex` or `X-Robots` in packages/client/scripts; the
isolation flag lives only in Vite's dev and preview servers, packages/client/vite.config.mjs:7-12, :38-44). ADR 0002
part 10's "The build generates the `_headers` file" is not built, and `packages/` is lane 1's (Part 15 :103).
Recommend (a departure, so a decision): the preview header set is a host config file of lane 2's, on main (e.g.
`hosting/pages/preview.headers`), which the `check` job copies in as `_headers` after deleting any `_headers` the
artifact holds (D1). Header rules then come only from main's reviewed text, never from PR data, and lane 2 needs no
edit of lane 1's build. Its content follows GLOSSARY §5 (production set plus `X-Robots-Tag: noindex`, isolation flag
off, which is "the preview Mustafa checks"), e.g. `/*` with `X-Robots-Tag: noindex`, and `/assets/*` with
`Cache-Control: public, max-age=31536000, immutable` (Vite's hashed files: base `./`, default `assets/` folder; ADR 0006
part 1 already asks an immutable cache header for the content bundle). Isolated previews (COOP/COEP) are not needed now
(the flag is "on only for profiling builds", GLOSSARY §5) and get their own file when a profiling need comes.
The deploy job's header check (R5-PREV-REC4 mark 1, PM-0002-67: "passes, and fails when a header is removed"): in the
no-secret `after` job, curl with a named User-Agent (PHC-PROBES) fetches the alias and the hash link: `/`, the
`index.html`, one JS file, one `.wasm`, and the Worker's file; it requires `X-Robots-Tag: noindex` on every response,
`application/wasm` on the `.wasm`, and `immutable` on `/assets/*` responses; no COOP or COEP. The bite cannot use
`noindex`: Pages adds it to every preview itself ("By default, every preview deployment generated by Cloudflare Pages
includes the X-Robots-Tag: noindex HTTP response header"; test-engineer's F1 in #67: "No bite tests a preview build
that lacks its `noindex` rule, because the host adds one anyway"). The `immutable` rule is a header the host does not add
by default (inferred; Pages' default caching not read), so removing it from the file must fail the check: that is the
bite. Against GLOSSARY §5's "differs from production in anything but `noindex`": the cache rule must then sit in the
production set too, when production gets its file (both sets carry it; they still differ only in `noindex`). If the
reviewers rule the cache header out, the fallback bite is a marker header in both sets with different values
(weaker: a header for testing only).
PHC-PREVIEW-NOINDEX: a file test, before any deploy, in the `check` job: the assembled `_headers` has a `/*` block with
`X-Robots-Tag: noindex`; plus a no-secret test on pull requests that touch the host folder: the same check against the
real file passes and against a copy with that line removed fails (the bite). "production's does not": there is no
production file until production exists; that half of the test comes with it. The second half of the follow-up ("before
any preview is served where the host adds no `noindex`, the preview expectation is read there") applies to a move to
Workers custom domains, not to pages.dev.

### D6. The request to the CI Builder chat (a PROPOSAL; sure: high on the YAML keys, medium on `.gate` and hidden files)
What toolchain.yml must add: ONE step at the end of job `toolchain`, after "Build and test with the network cut off"
(toolchain.yml:132-197), so it runs only when every step above passed (a step's default condition is success). It
uploads the production build folder the job already makes (toolchain.yml:172, "the page as players get it"), kept 7
days (ADR 0006 part 3). No permission changes (upload-artifact needs none beyond the run; the check's build job had
`contents: read` only, #66 App. B). Proposed text (LABEL (a): the pull_request run):

```yaml
      # PROPOSAL (lane 2, step 2.6's plan; not in force until the CI Builder chat's own pull request): hand the tested
      # player build to the preview workflow (.github/workflows/preview.yml, which starts on `workflow_run` of this
      # workflow's name `toolchain`). Pull requests from this repository only, after every step above passed. The
      # preview workflow treats these files as data and runs none of them. Files under .gate/ may count as hidden for
      # upload-artifact ("files within folders beginning with `.`"), hence include-hidden-files; an empty upload fails.
      - name: Hand the tested build to the preview workflow (pull requests from this repository only)
        if: github.event_name == 'pull_request' && github.event.pull_request.head.repo.full_name == github.repository
        uses: actions/upload-artifact@cf430e030ddbb5b0abf93d22962f4752f3646cd9 # v7.0.2
        with:
          name: preview-site
          path: .gate/dist-production/
          include-hidden-files: true
          if-no-files-found: error
          retention-days: 7
```
LABEL (b) instead (the push run, page label = the PR head): the same step with
`if: github.event_name == 'push' && github.ref != 'refs/heads/main'`; the preview then accepts `event == 'push'` and finds
the open PR by head branch and SHA; a PR opened without a new push gets its link at its next push.
Three lines for the CI Builder chat with it: (1) when step 2.3 replaces this file ("Step 2.3 replaces this file with
the gate v2's workflow", toolchain.yml:3), keep the workflow name `toolchain`, the file path and the artifact name
`preview-site`, or tell lane 2 first on the Lane requests issue (#59): the preview matches the name and checks the path;
(2) the pin above is the check's (TR), not yet in this repository's workflows: re-read it from actions/upload-artifact
itself ("verify it is from the action's repository and not a repository fork", Secure use); (3) if his Brave checks
need the development page (the test-only Wasm), the path is `.gate/dist-development/` instead; one folder only (two
would double the deploys).
How the preview finds it (lane 2's file, for the request's context):
`on: workflow_run: { workflows: [toolchain], types: [completed] }`; the `check` job (permissions `actions: read`) uses
`actions/download-artifact@9000827ccba6bdab643e8b6fd33ac0654aef8333 # v8.0.2` with `name: preview-site`,
`run-id: ${{ github.event.workflow_run.id }}`, `github-token: ${{ github.token }}`, `path: ${{ runner.temp }}/site`
(README: `github-token` "is required when downloading artifacts from a different repository or from a different
workflow run"; its example: "token with actions:read permissions on target repo"). PHC-ACTIONS-READ: needed, in the
`check` job only. If no `preview-site` artifact exists (the request not merged yet), the job ends with a notice and no
deploy, so the two pull requests can merge in either order.

### D7. His clicks, once, in order, one sitting (about 20 minutes; can be done before the merge, must be before the
first run; names are proposals)
Cloudflare (dash.cloudflare.com, his existing account, unless he picks a previews-only account: decision ACCOUNT):
1. Workers & Pages, Create, Pages, "Upload assets": project name e.g. `shamble-previews`; drag in a folder with one
   placeholder `index.html` ("No game here"); Deploy. That upload is the project's production version and its only
   one (as in the check: "Made by his drag-and-drop upload of a placeholder page ... so production's first version is
   his"); production branch stays `main` (the check's project read `production_branch: main`). No job ever deploys to it.
2. The project's Settings, "Preview access": leave OFF (ADR 0006 part 2: public but unlisted). Switch it ON later only
   for unreleased art or the first multiplayer preview: the same setting, which made a Cloudflare Access application
   with an e-mail-code login and nothing paid (#66 :99-103); OFF again: the setting off, then Zero Trust, Access,
   Applications, delete the application.
3. Manage Account, Account API Tokens, Create Token, Custom token: name e.g. `shamble-previews-deploy`; permissions
   Account, Cloudflare Pages, Edit (and Read, as the check's working token held both; Read adds nothing risky);
   account: his one account (the form offers the whole account, no single project); Client IP Address Filtering:
   empty (GitHub's runners have changing addresses: inferred); Token expiration: set BOTH a start date (today) and an
   end date (decision TOKEN-WINDOW) by their own fields, never the single-date chip, which made a one-day window in the
   check (PHC-TOKEN-SETUP); Continue to summary, Create Token; keep the page open, copy the token.
4. Copy the Account ID (the account's overview page).
GitHub (github.com/Mergincanli/Shamble-Frontlines, Settings):
5. Environments, New environment `preview`: Required reviewers OFF; Wait timer off; "Allow administrators to bypass
   configured protection rules" UNTICKED (the check found it ticked at first, #66 :384-385); Deployment branches and
   tags: "Selected branches and tags", Add rule, Ref type Branch, `main`, Add. Environment secrets
   `CLOUDFLARE_API_TOKEN` (paste) and `CLOUDFLARE_ACCOUNT_ID` (paste).
6. If D3 (a): a second environment `preview-cleanup`, exactly as step 5, the same two secrets.
7. Secrets and variables, Actions, "Repository secrets": confirm none named CLOUDFLARE_*; a repository secret reaches
   every branch's workflow and would undo step 5.
8. Actions, General: confirm workflow permissions "Read repository contents and packages permissions" (as on the test
   repository); optionally tick "Require actions to be pinned to a full-length commit SHA" (every action in gate.yml
   and toolchain.yml is already pinned by SHA).
9. Branches or Rules, main's rule: read whether administrators may bypass it (decision MAIN-BYPASS): with the token
   behind a main-only rule, main's review is the whole wall.
The verify answer (PHC-TOKEN-SETUP): no screenshot from him; the deploy job's first step asks Cloudflare
(`/accounts/<id>/tokens/verify`) and prints `status`, `not_before` and `expires_on` in the run's log; the PR comment warns
14 days before the end date. After that: nothing per preview; he opens `https://pr-<N>.shamble-previews.pages.dev`
from the PR's comment. Untouched: the check's token is already revoked (#66).

### D8. The earliest honest day for the first link (working days, not promises)
The bootstrap (advisor's point): `workflow_run` "will only trigger a workflow run if the workflow file exists on the
default branch", so step 2.6's own pull request cannot make or prove its own preview. Proven only after the merge, on
the first real run: (1) the main-only rule admits the `workflow_run` job (docs say so; the refusal side was measured,
the admission side was not); (2) the `.gate/dist-production/` upload with include-hidden-files; (3) the payload fields
(`path`, `head_sha` equal to the PR head on a pull_request run, `pull_requests`); (4) Wrangler from the locked tree with
install scripts off; (5) the PR comment and its permission; (6) the header check passes and its bite fails (the bite can
be run on the first PR after the merge); (7) the counter; (8) the cleanup chain on a close; (9) the verify call on an
account token. Before the merge: the scripts' own tests with fixtures, and optionally a throwaway job in step 2.6's
pull request that names `preview` and must be refused (PM-0002-66's bite on the real environment, after his sitting).
Each fix after the merge is a tier-2 pull request: the advisor agent, the security-auditor, his go in a day session.
Count, from his go on step 2.6's plan:
- the build (workflow files, host folder with header file and lockfile, scripts and their tests, runbook, his two
  messages into DECISIONS): 1 working day;
- review round 1 (advisor agent, security-auditor on tier 2, infra-engineer, test-engineer) and the fix pass: 1;
  a round 2 if needed: +1;
- his go to merge, his sitting (D7, about 20 minutes, can come earlier), the first PR push after both merges: the link
  about 20 minutes after that push (toolchain's 15-minute cap plus the deploy): 0.5 to 1;
- the CI Builder chat's edit (D6), in parallel: its own small tier-2 pull request with its plan, the advisor agent, the
  security-auditor and his go: 1 to 2; the first link waits for it, the merge of step 2.6 does not;
- one fix cycle after the first run (likely, given the nine items above): +1.
So: best case 3 working days after his go; honest 4 to 6 (one fix cycle, maybe a second review round). If his go comes
on Thursday 26 November: best Tuesday 1 December, honest Wednesday 2 to Friday 4 December; the week of 7 December if a
review or the CI Builder's edit slips. A rehearsal in a throwaway repository first (decision FIRST-RUN (b)) adds about 2
calendar days and his clicks for a test project and token, and removes most of the fix cycle.

## Decisions this research raises (for the plan's "Decisions needed"; my recommendation marked)
- TRIGGER (his, with the Advisor and security-auditor for OQ-0006-13): how previews get the token with no click.
  (a) RECOMMENDED: `workflow_run` + `preview` limited to `main`, no reviewer: no click, and branch code cannot reach the
  token (meets both "without a click from me" and "show what keeps the token safe"); departs from ADR 0002 part 10
  ("no `workflow_run`", P3 rejected) and from the build plan's condition (4) ("a reviewer on previews"); first run only
  after the merge. (b) P1 as ADR 0002 has it (`pull_request`; the deploy job reached through toolchain, e.g. a reusable
  workflow called from it, so a lane 1 edit), no reviewer, the environment open to `refs/pull/*/merge`, previews in a
  previews-only Cloudflare account: no click, but any branch can take the token; safety is only the small reach (that
  account holds nothing else). (c) keep the reviewer: fails "I do not want to click for each preview".
- LABEL (main session; his if he cares): which run's build the preview shows. (a) RECOMMENDED: the pull_request run
  (the tested merge with main); the page's corner shows the merge commit, the PR comment names the head it came from.
  (b) the push run: the page shows the head commit he approves; a PR opened without a new push waits for its next push.
- HEADERS-SOURCE (the Advisor; departs from ADR 0002 part 10 "The build generates the `_headers` file"): (a)
  RECOMMENDED: lane 2's header file on main, copied in by the preview workflow; (b) lane 1's build generates it (a
  second request, in `packages/`), and the preview checks it.
- FIRST-RUN (his): (a) RECOMMENDED: merge, then fix on main (one fix cycle budgeted, each his go); (b) a rehearsal in
  a throwaway repository first (+2 calendar days, a test project and token from him).
- TOKEN-WINDOW (his): the token's end date. (a) RECOMMENDED: 6 months, with the PR comment warning 14 days before;
  (b) 90 days; (c) no end date (a lost token stays valid until he deletes it).
- COUNTER (main session): (a) RECOMMENDED: a second environment `preview-cleanup` so `preview` counts deploys exactly;
  (b) one environment, an upper bound.
- MAIN-BYPASS (his): may administrators bypass main's rule? (a) RECOMMENDED: no bypass (the token's wall is main's
  review); (b) keep it, written down as the residual.
- ACCOUNT (his, now or at the first release): (a) RECOMMENDED now: his existing account (no production exists to
  protect); at the first release (OQ-0006-14) move previews to a previews-only account if production joins this one,
  since only a second account separates tokens and the apex domain must stay where its zone is; (b) a previews-only
  account from the start (a new sign-up, nothing paid: not researched).
- STALE-30 (main session): (a) RECOMMENDED: a one-line notice on PRs whose preview is 25+ days old; (b) re-run CI
  automatically for them.
- FILES (main session, the Advisor): Part 15 :103 says "the workflow file it adds for previews (a new file)"; this
  design adds two workflow files (`preview.yml`, `preview-closed.yml`) and a host folder with a lockfile; check with
  the Advisor that the lockfile counts as "the host's config files it adds" (Part 15's condition 2 names lane 1's
  lockfile and new packages).

## Status
All D1-D8 blocks written 2026-11-25. Next: advisor call, then the handback.
