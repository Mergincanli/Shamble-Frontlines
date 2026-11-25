# security-auditor: plan-stage read of step 2.6 (previews), plan revision 1

Started 2026-11-25. Read-only review of `.checkpoints/step-2-6-previews/plan.md` rev 1 and
`infra-engineer-design.md` (D1-D8 + override section). No source edits. Not committed by me.

## Plan (blocks, in order)
1. Read the plan, the design (D1-D8, override), toolchain.yml, gate.yml, CHECKPOINTS DEP-HOLD. [done]
2. Q1 token reach: workflow_run payload, artifact poisoning (deploy/after/Wrangler), download-artifact
   path shadowing, cache. Name injection points + guards.
3. Q2 environment branch rule with deployment:false; M1 sufficiency; how planted jobs report.
4. Q3 account-wide token: ACCOUNT (b), end date, IP filtering.
5. Q4 reconcile force deletes: what stops wrong deletes.
6. Q5 PR comment injection from workflow_run context.
7. Q6 supply chain: Wrangler lockfile + --ignore-scripts, SHA pins, DEP-HOLD answer criteria.
8. Q7 his clicks (section 4).
9. Primary-source checks (GitHub docs: environments, workflow_run, download-artifact; Cloudflare Pages
   _headers/_redirects/functions, token verify) where a finding turns on them.
10. Verdict + ranked findings with exact plan changes.

## Sources read (summary saved as read)
- WEB GitHub Docs, "Managing GitHub Actions settings for a repository" (read 2026-11-25): "Anyone with write access
  to a repository can modify the permissions granted to the `GITHUB_TOKEN`", "by editing the `permissions` key in
  the workflow file"; "Allow GitHub Actions to create and approve pull requests": for a new personal-account repo
  "workflows are not allowed to create or approve pull requests" (default off). So click 10 sets a default only.
- WEB GitHub Docs, Workflow syntax, `permissions` (read 2026-11-25): forks: "typically you can't grant write
  access"; "You can use the `permissions` key in your workflow to increase the access for the token."
- WEB Cloudflare, Wrangler "System environment variables" (read 2026-11-25): set them by "an `.env` file in your
  project directory" ("the recommended way"); `CLOUDFLARE_API_BASE_URL` default "https://api.cloudflare.com/client/v4";
  `WRANGLER_SEND_METRICS` "Defaults to `true`"; `WRANGLER_LOG`: "Logs can include requests to Cloudflare's API".
  Precedence of `.env` vs process env: not stated. => a `.env` in Wrangler's folder can point the token elsewhere.
- WEB Cloudflare Pages, "Direct Upload" (read 2026-11-25): "if a `functions` folder exists where the command is run,
  that `functions` folder will be uploaded with the project"; `_worker.js` "supported by both Wrangler and drag and
  drop"; file size "25 MiB". => `functions/` is taken from the cwd, not the artifact: the empty cwd is the guard.

- WEB actions/download-artifact README (read 2026-11-25): `path` "Optional. Default is $GITHUB_WORKSPACE"; `name`
  "If unspecified, all artifacts for the run are downloaded", each into "a directory denoted by the artifacts
  name"; `merge-multiple` true puts all in one folder; `digest-mismatch` checks "the downloaded hash and the expected
  hash from the server". => a PR-made artifact named `hosting` would land on `$GITHUB_WORKSPACE/hosting` if name or
  path were left default: name exact, path in RUNNER_TEMP, merge-multiple false, digest-mismatch error.

## Findings (one line each, as found)
- (found, to verify) Plan 2.1 refuse list drops D1's `_redirects`, dot-segments, 20,000-file cap; dot-files matter
  (Wrangler `.env`/config). plan.md:52 vs design :393-396.
- (found) Deploy must re-validate check's outputs (PR number, SHA) and re-run the refuse list on `site-checked`; a
  compromised `check` must not equal token theft. design :398-401.
- (found) Nothing artifact-derived may reach `$GITHUB_ENV`/`$GITHUB_OUTPUT`/`$GITHUB_PATH`/summary (newline in a
  file name = env injection); file names to a strict charset.
- (found) No cache restore/save in any preview.yml job; gate.yml:75-76 writes a main-scoped Playwright cache, and
  check/after run in main's cache scope on PR data (poisoning path).
- (found) PR comment: built only from validated fields; never page label, branch, title, Wrangler output; edit only
  a comment whose author is github-actions[bot] (write access can edit others' comments). design :402-406.
- (found, to verify) A branch workflow can raise its GITHUB_TOKEN to contents/pull-requests write regardless of
  click 10's default; gate/smoke come from the PR's own gate.yml (pull_request runs the merge commit's file) -> a
  branch could merge itself if no rule stops the Actions bot. Pre-existing; the token's wall rests on it.
- (found) preview-close is matched by NAME: any branch can add a workflow named like it and start reconcile;
  harmless only if reconcile uses no payload data. Say so as a guard.
- (found) M1 prints "whether a secret is present", but before click 8 `preview` holds no secret: an admitted job
  prints "absent" too. M1 cannot tell refused from admitted. Needs a worthless canary secret + the refusal line.
- (found) Plan 2.2 command omits D2's cwd, --commit-message, --commit-dirty; Wrangler reads `.env`/config/functions
  from where it runs (sources above): cwd fresh under RUNNER_TEMP, outside workspace, not an ancestor of the site.
- (found) Token at step level on verify + wrangler steps only, never job-level (npm ci never sees it); no
  WRANGLER_LOG=debug; WRANGLER_SEND_METRICS=false.
- (found) Reconcile: delete a PR's previews only on positive evidence (per-PR lookup says closed), not absence from a
  list; cap deletes per run; constant project; env=preview; branch read from the deployment object. D4 :517-518's
  "forged input can at most delete a closed PR's leftovers" is wrong for a short list.
- (found) TOKEN-WINDOW is in the design's decisions (:698-699) but not in plan section 12; click 6 has no date.
