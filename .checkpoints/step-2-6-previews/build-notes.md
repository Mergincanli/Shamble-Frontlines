# Step 2.6, the build's evidence (lane 2, Builder V2), from 2026-11-25 ~19:45 UTC

Each block is written once it is complete. Scratch work lives in the session's scratchpad (named below); nothing of it
is shipped.

## B1. The test runner and a folder argument (finding F1)
- `node --test hosting/` on Node v22.23.4, in a scratch folder holding `hosting/preview.mjs`, `hosting/preview.test.mjs`
  and `hosting/package.json` (no "main"): exit 1, "# Error: Cannot find module 'hosting'"; the folder is loaded as a
  module, not searched for tests. Without a package.json: the same.
- `node --test hosting/preview.test.mjs`: exit 0. `node --test hosting/missing.test.mjs`: exit 1 (a missing named file
  fails loudly).
- A `"main": "preview.test.mjs"` in hosting/package.json makes the folder argument run that file (measured): not used
  (the advisor tool: it is outside "for the upload tool only" and changes the manifest M3 checks).
- So the approved preview.yml's job `tests` (`run: node --test hosting/`) would fail on every run. Proposed: that one
  line becomes `run: node --test hosting/preview.test.mjs`, brought to him with the new sha256 before his sitting.

## B2. The real build's files
toolchain run 45686704452, job 139461080011 (2026-11-25 19:36 UTC), "production page in .gate/dist-production:":
`assets/index-CYgNJTPa.css`, `assets/index-zPOY-_Jj.js`, `assets/sim_wasm_bg-BiMaAlnD.wasm`,
`assets/worker-CyhXt1A5.js`, `index.html`. Hash characters include `_` and `-`; the refuse list's name rule allows
letters, digits, `.`, `_`, `-` (no leading dot or `-`, no `..`, at most 128 characters).
`workflow_run.head_sha` is the branch head: the pull-request run 45686704452 and the push run 45686692982 of the same
push both show head 69d9b5f3 (the run list, 2026-11-25 ~19:45 UTC). find-pr compares it with each open pull request's
`head.sha`.

## B3. Docs read (2026-11-25, by two web-fetch agents; quotes in their reports)
Cloudflare (developers.cloudflare.com):
- List deployments (api/resources/pages/subresources/projects/subresources/deployments/methods/list/): query `env`
  ("production" or "preview"), `page`, `per_page` (no default or maximum stated; example 20); fields `id`, `aliases`,
  `created_on`, `deployment_trigger.metadata.branch`, `environment`, `url`, `short_id` ("Short Id (8 character)"),
  `latest_stage` (Stage: `name` queued|initialize|clone_repo|build|deploy, `status`
  success|idle|active|failure|canceled|skipped); `result_info` optional (count, page, per_page, total_count,
  total_pages). Order: not stated.
- Delete deployment: `force` "Allow deletion when a non-production deployment has an active alias."
- Get project: `subdomain` "The Cloudflare subdomain associated with the project." (example "helloworld.pages.dev");
  `production_branch` "Production branch of the project. Used to identify production deployments."
- Verify account token (accounts/subresources/tokens/methods/verify/): result `id`, `status` active|disabled|expired,
  `expires_on`, `not_before`. The hosting check read HTTP 200 on /accounts/<id>/tokens/verify and 401 on
  /user/tokens/verify for an account token (report :498, :508).
- WRANGLER_OUTPUT_FILE_PATH (workers/wrangler/system-environment-variables/): ND-JSON, each entry with `timestamp` and
  `type`; the page lists `pages-deploy`, not `pages-deploy-detailed` (see B4: the code writes both).
- `_headers` (pages/configuration/headers/): a pattern line, then indented `Name: value` lines; matching rules combine;
  a header set twice is joined with a comma; up to 100 rules; 2,000 characters per line; `# This is a comment` in the
  example; not applied to Pages Functions responses.
- Serving (pages/configuration/serving-pages/): "/contact.html will be redirected to /contact, and /about/index.html
  will be redirected to /about/" (status not stated).
- Limits (pages/platform/limits/): 20,000 files per site (Free); 25 MiB per asset; "Preview deployments older than 30
  days are deleted, unless they are one of the project's 100 most recent deployments."
GitHub (docs.github.com):
- Issue comments (rest/issues/comments): max per_page 100; "Issue comments are ordered by ascending ID."
- List deployments (rest/deployments/deployments): `environment` parameter; max per_page 100; order not stated.
- Pulls (rest/pulls/pulls): `state` open|closed|all; objects carry `state`, `head.sha`, `head.repo.full_name`,
  `base.repo.full_name`.
- Permissions: the per-endpoint "fine-grained" boxes are not in the text the tool receives. The GitHub Apps page
  (rest/authentication/permissions-required-for-github-apps) lists POST and PATCH of issue comments under both
  "Issues" (write) and "Pull requests" (write), each with the "Additional Permissions" checkmark, which the page says
  marks either "more than one permission" or "any one permission from a set". So whether `pull-requests: write` alone
  can write the comment is NOT settled by a quote: risk R-COMMENT-PERM, proven or refuted on the first real run (the
  comment appears, M5/M9). `GET .../deployments` is under "Deployments" (read), no checkmark.
- REST pages now name `X-GitHub-Api-Version: 2026-03-10`; the script sends 2022-11-28 (the workflow-syntax page's links
  still carry it).

## B4. Wrangler 4.164.0's own code (node_modules/wrangler/wrangler-dist/cli.js, installed in the scratchpad)
- `pages deploy` writes two entries: `{type: "pages-deploy", version: 1, pages_project, deployment_id, url}` then
  `{type: "pages-deploy-detailed", version: 1, pages_project, deployment_id, url, alias, environment,
  production_branch, deployment_trigger: {metadata: {commit_hash}}}` (lines ~305054-305076); `writeOutput` appends
  `JSON.stringify({...entry, timestamp})` plus a line break.
- `alias` is the first of the deployment's `aliases` ending in ".pages.dev", read while polling until the latest stage
  is deploy/success (or failure); it can be missing, and then read-url refuses.
- Wrangler's own test of a finished upload: `latest_stage.name === "deploy" && latest_stage.status === "success"`;
  reconcile uses the same test for "the newest successful upload".

## B5. The lockfile (M3's inputs; scratchpad npm26/)
- Manifest: name shamble-frontlines-hosting, private, license UNLICENSED, one dependency `wrangler` at exactly 4.164.0
  (published 2026-11-17 11:15:02 UTC; 4.165.0 came 2026-11-18 21:44 UTC), no scripts, overrides or workspaces.
- `npm install --package-lock-only --ignore-scripts --no-audit --no-fund --before=2026-11-18T20:00:00Z` (npm 10.9.9,
  2026-11-25 20:05 UTC): lockfileVersion 3, 92 entries; every `resolved` on https://registry.npmjs.org/; every
  `integrity` sha512. Install scripts: esbuild 0.28.2, fsevents 2.3.3 (macOS only, optional), workerd 1.20261117.1.
- Every one of the 91 package versions was published before 2026-11-18T20:00Z (npm view <name> time, each); the
  newest is wrangler 4.164.0 itself. So the whole tree is at least 7 days old.
- In a fresh folder with only the two files: `npm ci --ignore-scripts --no-audit --no-fund`: "added 40 packages";
  `npm audit`: "found 0 vulnerabilities"; `npm audit signatures`: "39 packages have verified registry signatures",
  "24 packages have verified attestations"; `wrangler --version`: 4.164.0.
- The probe (install scripts off, from an empty folder, dummy token, neutral project `neutral-probe-project`):
  `wrangler pages deploy <site> --project-name neutral-probe-project --branch pr-1 ...`: exit 1 at the API ("A request
  to the Cloudflare API (/accounts/0000.../pages/projects/neutral-probe-project) failed. Authentication failed
  (status: 400) [code: 9106]"); the empty folder stayed empty. So the deploy path loads with install scripts off; the
  upload itself is proven only by M5.
- Wrangler wrote its own log to ~/.config/.wrangler/logs/ (4,762 bytes): no line holds the token or "authorization"
  (its code has WRANGLER_LOG_SANITIZE). On a runner that folder goes with the machine; no step uploads it.

## B6. The script, its tests, the bites
- hosting/preview.mjs: the eight commands of plan 2.8, each against the exact env names and paths its step in the
  approved preview.yml gives (checked line by line): find-pr (GH_TOKEN, REPO, HEAD_SHA), check-build (DOWNLOAD),
  recheck (none), verify-token (the two secrets), read-url (none), check-headers (URL), comment (GH_TOKEN, REPO, PR,
  SHA, URL, DEPLOY, HEADERS), reconcile (the two secrets, GH_TOKEN, REPO); PROJECT from the workflow's env, RUNNER_TEMP,
  GITHUB_OUTPUT and GITHUB_RUN_ID from the runner. main's files are read beside the script (its own checkout).
- Finding F3: read-url's step has no PR in its env (the approved text), so read-url checks the link against the
  project's real address (from verify-token), the one-off address (https://<first 8 of the id>.<address>), the
  project and `environment: preview`; the match with the pull request's number is made in `comment`, which has PR, SHA
  and URL. Together: the link is https://pr-<that pull request>.<the real address>.
- Finding F2 (as recorded in the checkpoint): a missing build after the hand-over step is on main fails `check`
  visibly (red under main's Actions tab, an ::error:: line); the comment cannot say it with the approved texts.
- Stricter than the plan's words (no loosening): pull-request numbers without a leading zero; an index.html required
  at the top of a build; reconcile keeps an open pull request's newest successful upload and anything newer, and
  deletes nothing of it when none succeeded; a second bite (noindex removed in memory must fail too); a warning when
  the token ends within 14 days.
- Tests: `node --test hosting/preview.test.mjs` in a scratch copy with the approved texts placed: 117 tests, 117 pass,
  2.5 s. Every pattern has a trailing-line-break case; every refuse-list entry a planted file; reconcile's look-up-first
  (the order of calls), its cap and its fail-closed listing; the comment from checked values only; the header file's
  test; G9's lint (L0 layout, L1 `${{` in run, L2 secrets outside a step env, L3 installers, L4 cache, L5 pins and
  credentials), each with a planted variant of the real preview.yml.
- The mutation sweep (scratchpad mutate.py): 28 mutations of the script, each breaking one guard; the suite fails on
  all 28 (one survivor, the output's own shape check, got its own test first).
- Gate: `node tools/gate.mjs --no-smoke` PASS; `node .claude/scripts/repo-map-check.mjs` OK (198 entries, 110 paths)
  after REPO_MAP's new `hosting/` row.
