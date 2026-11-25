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
