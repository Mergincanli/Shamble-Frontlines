# Step 2.6, the build's evidence (lane 2, Builder V2), from 2026-11-25, 19:45:18 UTC (this chat's record)

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
push both show head 69d9b5f3 (the run list, read 2026-11-25 19:45:18 UTC). find-pr compares it with each open pull request's
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
- Permissions, settled (the advisor agent found it; this session re-read the rendered pages with curl, 2026-11-25
  20:28:46 to 20:29:05 UTC, HTTP 200; the web-fetch tool's text lacked these boxes):
  - https://docs.github.com/en/rest/issues/comments, "Create an issue comment" and "Update an issue comment": "This
    endpoint works with the following fine-grained token types : ... GitHub App installation access tokens ..." and
    "The fine-grained token must have at least one of the following permission sets: "Issues" repository permissions
    (write) "Pull requests" repository permissions (write)". "List issue comments": Issues (read) or Pull requests
    (read). So `after`'s `pull-requests: write` is enough; `issues: write` stays out.
  - https://docs.github.com/en/rest/deployments/deployments, "List deployments": "The fine-grained token must have the
    following permission set: "Deployments" repository permissions (read)" (`after` holds deployments: read).
  - https://docs.github.com/en/rest/pulls/pulls, "List pull requests": "Pull requests" (read); "Get a pull request":
    "Pull requests" (read) or "Contents" (read) (`check` and `reconcile` hold pull-requests: read).
  - The first run confirms it (M5/M9). If it were ever refuted: a tier-2 fix adding `issues: write` to `after`, a new
    sha256 and his yes.
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
- The mutation sweep (scratchpad mutate.py): 29 mutations of the script, each breaking one guard; the suite fails on
  all 29. Two survivors along the way got their own tests first: the output's own shape check, and the link's prefix
  (pull request 1 against pr-12's link: the test's fake GitHub now answers every number, so only the link check can
  refuse; the advisor agent's Low on F3).
- Until U2 places preview.yml, `node --test hosting/preview.test.mjs` in the real tree fails loudly (ENOENT on
  .github/workflows/preview.yml): the suite runs in a scratch copy with the texts placed (the advisor agent's Low).
- Gate: `node tools/gate.mjs --no-smoke` PASS; `node .claude/scripts/repo-map-check.mjs` OK (198 entries, 110 paths)
  after REPO_MAP's new `hosting/` row.

## B7. The checks before the message to him (the advisor tool's five, 2026-11-25 20:42 to 20:47 UTC)
- CI on the pushed code: f0e5593 (the lockfile, the records, the runbook): gate run 45696431844 success (jobs gate and
  smoke, "Smoke (check 5a)" success), toolchain run 45696431804 success; e5e385c (dfc8fb6, the script): gate run
  45694111052 and toolchain run 45694111099, both success.
- Times in the notes replaced by the record's: the CI request's tool answer 19:39:29 UTC; the run list 19:45:18; the
  first test-runner check 19:48:13; the release list 19:48:15; the advisor agent's verdict 20:27:50; the curl re-reads
  20:28:46 to 20:29:05; the security-auditor's M3 answer 20:30:14; dependency-check.md written 20:35:03 (its two
  sha256 lines unchanged).
- security-auditor-deps.md (135 lines) and security-auditor-early-read.md (79 lines) read whole: nothing of the wall's
  detail (his item 7).
- YAML: PyYAML 6.0.3 (SafeLoader, duplicate keys refused) parses preview-proposed.yml, preview-close.yml, wall-test.yml
  and the approved preview.yml; names, triggers, jobs, `permissions: {}` and `cache-mode: none` on every job as
  intended. GitHub's own reading comes only when the draft pull request opens.
- The scratch scripts are now in scripts/ (mutate.py: `python3 mutate.py <suite> hosting/preview.mjs`, with the suite a
  copy holding hosting/ and the texts under .github/workflows/; add_three.py; place_u2.py).
- REPO_MAP's hosting/ row now names the manifest and lockfile.

## B8. Draft pull request #72 and the first half of M1 (2026-11-25, 20:54 to 21:01 UTC)
- #72 opened 20:54:50 UTC (draft). GitHub held it as conflicting (mergeable_state "dirty", main 07d673b0 after #71),
  so no pull-request run started; main taken in at e3b3642b (scripts/resolve71.py, verify71.py), pushed 20:59 UTC.
- The CI proof of the tests: preview run 45700220336, job `tests` 139508091622 (pull_request, e3b3642b): "# tests 117",
  "# suites 12", "# pass 117", "# fail 0" at 21:00:19 UTC; jobs check, deploy, after and reconcile skipped, as
  designed. GitHub accepted the workflow text (the run exists and ran).
- M1, before click 5 (the bite: the jobs run and see no secret):
  - push-to-branch: wall-test run 45699354868 (push, 95d32ac), job 139505057787: "absent" at 20:54:21 UTC.
  - pr-plain: wall-test run 45700220147 (pull_request, e3b3642b), job 139508091935: "absent" at 21:00:09 UTC.
  - pr-no-deployment-record: the same run, job 139508091246: "absent" at 21:00:07 UTC.
  These runs are the ones re-run after click 5 and after the token click.
- Main's toolchain.yml (07d673b0) has no hand-over step yet: until it has, a preview ends with a notice.
- The order of his clicks: his words of 20:35 Amsterdam, item 5 ("The real token is my last click") put the GitHub
  settings (plan clicks 9 and 10) before the token clicks (6, 7, 8). The wall is not merged (main 07d673b0), so he is
  asked again before the token, as his item 5 says.

## B9. His sitting, click 3 (2026-11-25, his screenshots after 21:33 UTC; no account ID or subdomain copied here)
- His screenshots show the existing account (it holds the old test project shamble-preview-check), not a new
  previews account: clicks 1 and 2 (Accounts, Create Account, "Shamble previews"; switch to it) come first.
- Cloudflare's dashboard changed: "Create application" opens "Create an app" (GitHub, GitLab, Hello World, a template,
  "Upload your static files": the Workers flow) with "Need to use the legacy Pages workflow? Continue to Pages" below.
  A Pages project for direct upload: Continue to Pages, "Drag and drop your files", Get started. Docs: Direct Upload
  (developers.cloudflare.com/pages/get-started/direct-upload/, "Last updated Apr 21, 2026"): "Select Create application
  > Get started > Drag and drop your files . Enter your project name in the provided field and drag and drop your
  assets. Select Deploy site ." Create account (developers.cloudflare.com/fundamentals/account/create-account/, "Last
  updated Sep 17, 2026"): "go to Accounts", "Select Create Account . Enter an account name. Select Create Account ."
  Eligibility: user at least seven days old, Super Administrator on an existing account, up to five extra Free accounts.

## B10. M1 after click 5: FAILED (2026-11-25, re-runs at 22:29 UTC)
- His message (uuid 7903e9f9-6b78-4951-a72e-c82cd6c84b6d, 22:28:33 UTC, 941 bytes, sha256 4591957d...): "5 done";
  no CLOUDFLARE_ secret (click 9); click 10 done; TOKEN-TONIGHT: B.
- Re-run by this chat (actions_run_trigger, rerun_workflow_run, 22:29 UTC) of wall-test runs 45699354868 (push) and
  45700220147 (pull_request), attempt 2: ALL THREE planted jobs ran, each with "S: ***" and "present":
  push-to-branch job 139553977636 ("present" 22:29:10), pr-plain job 139553983737 ("present" 22:29:11),
  pr-no-deployment-record job 139553984267 ("present" 22:29:12). GitHub did not refuse any of them.
- The plan's stop rule applies (click 5: "If any planted job ran, stop: delete the environment, and no token goes in").
  No token was going in (TOKEN-TONIGHT (b)); WALL_TEST is the worthless "not-a-token".
- Read (docs.github.com, 2026-11-25 ~22:31 UTC): manage-environments: "Deselect Allow administrators to bypass
  configured protection rules . Click Save protection rules ." (a separate save, not in the plan's click 5);
  "If you chose Selected branches and tags , to add a new rule, click Add deployment branch or tag rule ... Click Add
  rule ." deployments-and-environments: "By default, administrators can bypass the protection rules and force
  deployments to specific environments."; "Deployment branches and tags are available for all public repositories.";
  "Allowing administrators to bypass protection rules is only available for public repositories for users on GitHub
  Free, GitHub Pro, and GitHub Team plans." The repository is public (list_repos). Every run here is triggered by his
  account (an administrator), this chat's re-runs included.
- Likely cause (not yet confirmed): the bypass box unticked without "Save protection rules", or the branch rule not
  saved. Needs his screenshot of the environment page.

## B11. His screenshot of the environment page (2026-11-25, after 22:31 UTC)
- Settings, Environments, Configure preview: banner "Deployment branch rule "main" saved successfully."; Required
  reviewers and Wait timer unticked; "Allow administrators to bypass configured protection rules" shown unticked, with
  the "Save protection rules" button below it (whether that state was saved before 22:29 is not visible); Deployment
  branches and tags: "Selected branches and tags", "1 branch and 0 tags allowed", rule `main`, "Currently applies to 1
  branch"; Environment secrets: WALL_TEST ("34 minutes ago"). So the rule and the secret are as planned; the open point
  is the bypass box's saved state. His question "which environment?" (WALL-FAIL-NEXT (a)): the GitHub environment
  `preview`, this page.
- Next: he clicks "Save protection rules", reloads, checks the box stays unticked, says "fixed"; this chat re-runs.

## B12. M1 after click 5, second try: PASSED for what the tools show (2026-11-25, 22:56 UTC)
- His "fixed" (uuid 5ff5bcf4-7ef7-42d0-9e5e-46203589ee68, 22:56:01 UTC, 86 bytes): the bypass box saved unticked;
  and his yes to watching #72 (this chat subscribed to its activity).
- Re-run by this chat at 22:56 UTC, attempt 3: push-to-branch job 139566690597 (run 45699354868), pr-plain job
  139566698136 and pr-no-deployment-record job 139566697600 (run 45700220147): each "completed", "failure", with no
  step listed, no runner assigned (22:56:12 to 22:56:15), and no log (get_job_logs: HTTP 404). get_check_run's output
  is empty. So GitHub refused all three before any step.
- Not read by the tools: GitHub's refusal line (a run annotation; asked of him as a screenshot, as the plan allows),
  and whether a deployment record was made (no tool for the deployments list here): recorded as not read.
- What the two tries show together: with the box unticked but not saved (attempt 2, 22:29), all three ran and saw
  the test value; after "Save protection rules" (attempt 3), all three were refused. The plan's click 5 lacked the save;
  the runbook now names it, and the token sitting's instructions must too.
- His sitting messages (7903e9f9: TOKEN-TONIGHT (b); 5ff5bcf4) are in DECISIONS (scripts/add_sitting.py).

## B13. Review round 1 (2026-11-25, launched ~23:03 UTC; head 8fef0f6, code head ad2658a)
- infra-engineer (round1-infra-engineer.md): REVISE, runbook only; no preview.yml change (31119c06 stands).
  Mediums: INFRA-M1 renewal order and a rebuild section; INFRA-M2 re-run limits (30 days; the checked build kept one
  day); INFRA-M3 the token's end date in STATUS with a reminder. Lows L1 to L7 and an observation (Pages adds noindex
  to previews itself). Answered M1's open point: refused jobs did make deployment records (9 records, all wall-test).
- security-auditor (round1-security-auditor.md): APPROVE; no code change; no preview.yml change. It read the refusal
  lines through the check-run annotations API: push job 139566690597 'Branch "claude/optimistic-albattani-mdoonz" is
  not allowed to deploy to preview due to environment protection rules.'; jobs 139566698136 and 139566697600 'Branch
  "refs/pull/72/merge" is not allowed ...'; the fresh runs on ad2658a (45717715486, 45717720531) refused the same way.
  A refused job leaves a deployment record with status failure (an admitted one: success). The cause of try 1 is not
  singled out (the rule may have been saved after 22:29 too). One Medium condition on the token sitting's message
  (a) reload and screenshot the environment page (box unticked, "1 branch and 0 tags allowed", `main`, no custom
  rule); (b) secrets only by "Add environment secret"; (c) click 9 again after the secrets (Actions, Dependabot,
  Codespaces); (d) the session re-runs M1: the refusal line plus zero steps for all three; a failure record expected;
  (e) delete WALL_TEST; wall-test.yml leaves. Lows: the runbook's cause as likely and both checks after any change;
  secrets as environment secrets in renewal; a Wrangler bump rule; the count is an upper bound.
- test-engineer: running.
- The fix pass, prepared (not committed until all three are in): the runbook rewritten (the environment page section,
  before the hand-over, re-run limits, the off-switch, the count as an upper bound, the renewal order with the save,
  the end date in STATUS, the Wrangler bump rule, rebuild from zero, the noindex observation); preview.mjs: reconcile
  removes duplicate records by id (L7), the hand-over match takes a quoted name or a trailing comment (L6), the count
  worded as deploy attempts (an upper bound); tests: 119 pass; the sweep: 31 of 31 caught (two new mutations).
  Follow-ups, not in this pass: L4 (check-headers one file at a time: sample later), L6's #59 note (covered by the
  request's note 1).
- test-engineer (round1-test-engineer.md; ~23:21 UTC): REVISE, no Critical or High; its re-run of the 29 mutations: all
  caught, each by its target. Mediums: TE-A the lint misses `toJSON(secrets)`, `secrets['X']`, `--ignore-scripts=false`,
  `npm rebuild`, `npm --prefix . install`; TE-B the lint misses a checkout of another ref (the pwn-request shape);
  TE-C the comment's failure paths (a refused write, an unreadable pull request) unpinned; TE-D the command line and
  defaultIo() never run in the suite; TE-E the count is not exact (refused jobs make records: 9 this month, all
  wall-test); TE-F M1 does not measure a CLOUDFLARE_ secret kept elsewhere, nor a tag ref. Lows: TE-G the refusal
  lines and deployment records are readable (annotations API); TE-H more guards unlinted (G6's exact places, ...);
  TE-I the reader's blind spots (quoted keys, flow mappings, `run :`), no structure assertion; TE-J 20 of its own 26
  mutations survived (unpinned paths); TE-K `preview / tests` is not a required check, and the gate is blind to
  hosting/.
- The fix pass, part 2 (tests only; preview.yml unchanged at 31119c06): the lint's L2 matches the word `secrets` in any
  spelling outside a step env, and the token must sit in exactly deploy, deploy, reconcile; L3 allows npm only as
  `npm ci --ignore-scripts` (not `=false`) and flags corepack; L5 flags a checkout with `ref:` or `repository:`; L0
  flags quoted keys, flow-mapping steps and `key :`; the real-file test asserts 5 jobs and at least 12 run steps; 10
  new planted lint variants. Tests for the comment's refused write (403 update, 500 new), an unreadable pull request
  (500, nothing written), a closed pull request or a fork head (nothing written), an unreadable comment list (red, no
  duplicate), an unreadable count ("not counted", never 0); find-pr: a listed non-open state ignored, more than 10
  full pages refused; verify-token: HTTP 401, 404 or success false refused; read-url: a malformed id refused even with
  a matching one-off address; check-headers: a 503 then a 1010 retried, a lasting 503 tried 5 times, a refused file in
  the list refused before any call; reconcile: success false on HTTP 200 counts as failed, more than 40 full pages
  incomplete (nothing deleted); setOutput: a relative GITHUB_OUTPUT refused; check-build: 20,000 files pass the first
  look but not with the header file (go=false); the command line (spawned): no command exits 2 with the usage line,
  recheck passes main's header file and refuses a changed one, find-pr refuses a malformed commit before any call.
  148 tests pass (13 suites). Sweeps: mine 32 of 32 caught (T26, the command line never running, added); the
  test-engineer's 26 re-run (below).
- TE-G taken: the refusal lines are read (the security-auditor's annotations read, B13); no screenshot needed.
- TE-E: the count is worded as an upper bound (part 1); the report corrects plan 2.3's "exact"; M9 compares the comment's
  number with Cloudflare's list for the month minus the wall-test records.
- TE-F: (a) covered at the token sitting by the security-auditor's condition (c): screenshots of the Actions,
  Dependabot and Codespaces secrets pages; (b) the tag case: a decision for him in the report (measure it with a tag job
  in wall-test.yml, a new sha256 and his yes, or record it as an unmeasured residual); recommended: the residual.
- TE-K: the report says `preview / tests` is not a required check and the gate does not read hosting/.
- The test-engineer's own mutations (scripts/mutate_te.py, its T1 to T25; its T26 is in scripts/mutate.py) re-run on
  the part-2 suite (2026-11-25 ~23:38 UTC): 22 caught; the 3 that survive (T11 a thrown fetch leaves status 200, T20
  annotate's line-break escape removed, T23 a loose "closed" test) are the three it judged equivalent itself (the outcome
  is the same: a failed check, messages already escaped by show(), reconcile stores only open or closed). All 20 of its
  real survivors are now caught; this chat's sweep: 32 of 32.

## B14. Review round 2, on what changed (8fef0f6..6fd0375d)
- infra-engineer (round2-infra-engineer.md): APPROVE. INFRA-M1 and INFRA-M2 closed; INFRA-M3 closed in the runbook only
  (STATUS gets the end date when the token is made: carry it in the report and the token sitting's message); Lows L1,
  L2, L3, L5, L6, L7 and the noindex observation closed; the deferrals (L4, L6's #59 note) agreed. Two new Lows, for the
  next push: preview.mjs :291 and :295 point to "the runbook: clicks 6 and 8" (the runbook has "The token", "To
  renew"); the runbook's rebuild step 4 points to "To renew, steps 2 to 4", skipping step 5 (the end date in STATUS),
  and step 3's "Replace" reads "Add" in a rebuild.
- test-engineer: running.
