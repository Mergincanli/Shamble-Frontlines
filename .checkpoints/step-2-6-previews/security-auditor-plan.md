# security-auditor: plan-stage read of step 2.6 (previews), plan revision 1

Started 2026-11-25. Read-only review of `.checkpoints/step-2-6-previews/plan.md` rev 1 and
`infra-engineer-design.md` (D1-D8 + override section). No source edits. Not committed by me.
Process note: three early appends used `cd` + a relative path (against the caller's literal-absolute-path rule), and
one early call ran `cd ... && find ...; git status` (git behind another command, against the interim rule). This
file is now written by its literal absolute path. No memory folder exists for this agent (`memory: local`; none on
disk), so no durable decision was recorded there.

## Plan (blocks, in order) - all done
1. Read the plan, the design, toolchain.yml, gate.yml, CHECKPOINTS DEP-HOLD.
2. Q1 token reach. 3. Q2 branch rule + deployment:false + M1. 4. Q3 account-wide token. 5. Q4 reconcile.
6. Q5 comments. 7. Q6 supply chain. 8. Q7 his clicks. 9. Primary-source checks. 10. Verdict.

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
  Whether `pages deploy` loads `.env`, and its precedence over process env: NOT stated. => exfiltration via a `.env`
  in Wrangler's folder is INFERRED; setting the base URL explicitly is not offered as a guard.
- WEB Cloudflare Pages, "Direct Upload" (read 2026-11-25): "if a `functions` folder exists where the command is run,
  that `functions` folder will be uploaded with the project"; `_worker.js` "supported by both Wrangler and drag and
  drop"; file size "25 MiB". => `functions/` comes from the cwd, not the artifact: the empty cwd is the guard.
- WEB actions/download-artifact README (read 2026-11-25): `path` "Optional. Default is $GITHUB_WORKSPACE"; `name`
  "If unspecified, all artifacts for the run are downloaded", each into "a directory denoted by the artifacts
  name"; `merge-multiple` true puts all in one folder; `digest-mismatch` compares "the downloaded hash and the
  expected hash from the server" (accepted values not read). => a PR-made artifact named `hosting` would land on
  `$GITHUB_WORKSPACE/hosting` if name or path were left default.
- READ `gh api repos/Mergincanli/Shamble-Frontlines/rules/branches/main` and `.../rulesets/25069670` (2026-11-25):
  "Shamble Ruleset 1", active, ~DEFAULT_BRANCH: deletion, non_fast_forward, pull_request with
  `required_approving_review_count: 0`, required_status_checks `gate`, `smoke` with `integration_id: 15368`;
  `bypass_actors: []`; no update restriction. Classic protection `enabled: false`.
  `require_extra_approval_for_unattributed_changes: true` is not in GitHub's docs I read; one third-party page
  (umatechnology.org, "github branch protection settings for ai generated pull requests") says it has no effect at
  0 approvals (sure: low). Kept in notes only.
- READ `gh api repos/Mergincanli/Shamble-Frontlines/commits/main/check-runs` (2026-11-25): `gate`, `smoke`,
  `toolchain` all `app_id 15368`, `app_slug github-actions`. => 15368 is the GitHub Actions app (verified).
- WEB GitHub Docs, "GitHub-hosted runners" reference (read 2026-11-25): "The list of GitHub Actions IP addresses
  returned by the API is updated once a week"; allowlists: "we do not recommend adding these ranges to it"; fixed
  addresses only with "larger runners with a static IP address range, or self-hosted runners". => IP filter empty.
- WEB github.com release pages (read 2026-11-25): actions/upload-artifact v7.0.2 = cf430e030ddbb5b0abf93d22962f4752f3646cd9;
  actions/download-artifact v8.0.2 = 9000827ccba6bdab643e8b6fd33ac0654aef8333. Both match the plan/design pins.
  (`gh api` on those repos is refused in this session; checkout v7.0.1 and setup-node v7.0.0 are toolchain.yml:51,55,
  not re-read.) The final-diff review re-reads every pin.
- READ packages/client/scripts/build.mjs:185-222 + grep of packages/client: no `build.manifest`, no `public/`; the
  production build emits no dot-file (sure: medium-high, from config, not from a built listing).
- READ toolchain.yml:36-37 (`cache-mode: none`, "code planted for a later step cannot reach another run through it");
  gate.yml:75-76 (`actions/cache` of `~/.cache/ms-playwright`, so a main-scoped cache exists).

## Findings (one line each, as found; corrections marked)
- Plan 2.1 refuse list (plan.md:52) drops D1's dot-segments, `_redirects`, 20,000-file cap (design :393-396).
- Deploy must re-validate check's outputs and re-run the refuse list on `site-checked` (G5).
- Nothing artifact-derived into `$GITHUB_ENV`/`$GITHUB_OUTPUT`/`$GITHUB_PATH`/summary (G7).
- No cache in any preview job (G8); cache-scope poisoning path is INFERRED, precedent toolchain.yml:36-37.
- Comment from validated fields; edit only github-actions[bot]'s marked comment (edit rights of write access:
  INFERRED, not read).
- preview-close is matched by NAME: any branch can add a namesake and start reconcile; harmless only if reconcile
  reads nothing from the triggering run.
- SUPERSEDED: "M1 cannot tell refused from admitted". Corrected: refusal is M1's pass mark (plan :239-241); the gap
  is the constant fallback print before any secret exists, how "refused" is read, and the missing push ref (Medium).
- Main wall (read): 0 approvals, checks from github-actions, no bypass, no update restriction. Precondition: a changed
  workflow file must first reach a pushed branch (day sessions: the guard asks before a `.github/` edit; routines'
  guard on `.github/` and whether the sessions' push credential may write workflow files: NOT read).
- TOKEN-WINDOW is in design :698-699 but not in plan section 12; click 6 has no date.
- Click 10 is a default only (docs quote above).

## Report (full text as handed back)
VERDICT: REVISE (not BLOCK). With the guards below in the plan, no code a pull request controls reaches the token
directly. The one Critical predates step 2.6, but the token's only wall rests on it: click 8 waits on it.

1. CRITICAL, pre-existing, not caused by step 2.6 (Q1/Q7; sure: medium-high). GitHub Actions itself can cross
   main's wall. Read: ruleset 25069670 = PR with 0 approvals, gate/smoke from github-actions (15368), no bypass, no
   update restriction. Quoted: write access "can modify the permissions granted to the GITHUB_TOKEN ... by editing
   the permissions key". Inferred, never to be tested here: once a changed workflow file is on a pushed branch, a
   PR whose own gate.yml goes green and whose workflow asks contents + pull-requests write can merge itself, so a
   planted preview.yml reaches the token. Change: put this in 2.1 "What GitHub does not enforce" and R1; click 11
   becomes this reading; new decision MAIN-WALL: (a) a rule so that only his account can update main, designed by
   infra-engineer, measured on a throwaway repository; (b) accept in writing, bounded by ACCOUNT (b). Recommend (a).
   It goes to his Decisions needed whatever happens to this plan.
2. HIGH (Q1; sure: high on each guard's need). Restore from D1 (plan :52 vs design :393-396): dot-segments refused
   (the token-relevant one; today's build emits none), `_redirects`, the file cap; D2's full command (:447-448)
   instead of plan :65's. Keep what 2.1/D1 have (env-only, regexes, RUNNER_TEMP, empty folder, `pr-<N>`). New:
   G1 no `${{ }}` of event, needs- or step-outputs inside any `run:`; head_branch/title/commit never read; checkout
      of main with no `ref:` and `persist-credentials: false`; workflow-level `permissions: {}`.
   G2 download: exact `name`, a fresh folder under RUNNER_TEMP, never the workspace, merge-multiple false, digest
      check on (value names read at build).
   G3 refuse non-regular files, names outside a strict charset, a leading `-`, `..`, `wrangler.toml|json|jsonc`.
   G4 Wrangler's cwd: fresh, empty, outside the workspace, not an ancestor of the site folder (`functions/` "where
      the command is run"; `.env` "in your project directory": load by `pages deploy` inferred).
   G5 deploy repeats check: re-validates PR number and SHA from env, re-runs the refuse list on `site-checked`,
      compares `_headers` byte for byte with main's, downloads from its own run only.
   G6 token at step level on verify + wrangler only; WRANGLER_LOG not debug; WRANGLER_SEND_METRICS=false; no
      Wrangler log or cache uploaded.
   G7 nothing artifact-derived into GITHUB_ENV/OUTPUT/PATH or the summary.
   G8 no cache restored or saved in any preview job (toolchain.yml:36-37 precedent; poisoning inferred).
   G9 M2 test fails on `${{` in a `run:`, a job-level token, a missing `--ignore-scripts`, any cache use.
   `_headers`: not a token path (served by Cloudflare; read as text at upload: inferred); re-checked in G5.
3. MEDIUM (Q2). M1: click 5 adds a worthless canary secret; planted jobs (`permissions: {}`, one step) print only
   present/absent; add a planted job on a push run of the branch; pass = environment-protection refusal, zero steps
   run, no deployment record; planted jobs absent from the final head. If the wall fails: stop, delete the
   environment, no token.
4. MEDIUM (Q4). "Comes back short" (plan :96, design :777-780) has no detection. Delete a PR's previews only after
   that PR is looked up and found closed, never on absence from a list; project a constant; preview deployments
   only; branch from the deployment record, `pr-<digits>`; cap per run (e.g. 20), else stop and report; log before
   deleting; read nothing from the triggering run (any branch can add a namesake of preview-close).
5. MEDIUM (Q5). Comment only from validated values (PR number, SHA, alias on the project's host read at build,
   integers, fixed words; the page label only if it is a hex SHA); never page text, branch, title, commit message
   or tool output (the PR Advisor reads comments). Edit only the marked comment authored by github-actions[bot].
   Header check: that host only, no redirects, no page script, file names by argv.
6. MEDIUM (Q3). TOKEN-WINDOW into section 12; recommend 90 days. IP filter empty is right (GitHub: "do not
   recommend" runner ranges as an allowlist; "updated once a week"). Click 6: the summary shows only "Shamble
   previews", an Account API Token, not a My Profile token. Runbook: the account holds nothing else; 2FA on.
7. MEDIUM (Q7). Click 10 is a default: add "Allow GitHub Actions to create and approve pull requests" unticked and
   "Require actions to be pinned to a full-length commit SHA" ticked; click 9 also Dependabot secrets; after click
   8 re-read the rule and the bypass box.
8. LOW (Q6). Pins verified (above). Scripts off: if it fails, no scripts-on for the tree; at most `npm rebuild` of
   one named package that `pages deploy` itself loads, whose script fetches nothing outside the lockfile, as a
   recorded departure shown to him and to me on the exact lockfile (a step-level token does not cover install
   scripts: same runner). DEP-HOLD criteria (hosting/ does not exist; criteria, not the answer), on the exact two
   files with sha256: manifest private, one dependency `wrangler` at an exact version, no lifecycle scripts,
   overrides or workspaces, outside the root workspace (package.json:9-11); lockfile v3, root equals manifest, every
   `resolved` on https://registry.npmjs.org/, sha512 integrity on every entry, wrangler at the pin,
   `hasInstallScript` packages listed, the release at least 7 days old; and the session's `npm ci --ignore-scripts`,
   `npm audit`, `npm audit signatures` outputs. Each bump: a new DEP-HOLD and a tier-2 PR.

## Revision 2 (plan.md at 1ac984a; the advisor agent's C13), started 2026-11-25
Plan (blocks): R2-1 verify sha256 of the three texts; R2-2 preview.yml against G1-G9 (the coordinator's 7 points);
R2-3 preview-close.yml; R2-4 wall-test.yml + M1 + click 5; R2-5 clicks 5-10, TOKEN-WINDOW, MAIN-WALL (decision 6);
R2-6 section 13 mapping; R2-7 script descriptions (hosting/preview.mjs) for omissions; R2-8 verdict.
- R2-1 done: sha256 of texts/preview.yml c66c0b0a9351..., preview-close.yml c2f1fa905116..., wall-test.yml
  3af067068060... match the coordinator's values (sha256sum, 2026-11-25). Branch head eb36162c; plan at 1ac984a3.
- WEB GitHub Docs, "Workflow commands" (read 2026-11-25): commands "are then sent to the runner over `stdout`";
  `stop-commands` "allows you to log anything without accidentally running a workflow command". Whether stdout
  `set-output` still works: not stated there (not claimed).
- R2-2 preview.yml vs G1-G9 (read texts/preview.yml:1-260): permissions {} (:21); secrets only on step env of
  verify-token (:157-161), wrangler (:162-174), reconcile (:254-260); cache-mode none on all 5 jobs; setup-node
  package-manager-cache false; persist-credentials false on every checkout; no `${{` in any run block; npm ci
  --ignore-scripts in a fresh sibling folder (:151-156); wrangler cwd $RUNNER_TEMP/empty, site $RUNNER_TEMP/site
  (not an ancestor: G4 holds); recheck (:149-150) runs before any token step (G5 holds: a fooled `check` cannot
  reach the token). check `if` (:54-60) right. continue-on-error (:90) safe only if check-build treats anything but
  exact `success` as no-go (not in plan). concurrency from needs.check.outputs.pr (:123) is a string, safe.
  environment.url (:121) safe as a value; read-url's regex (plan :89) admits any pages.dev project and any number.
  reconcile `if` (:227-233) right on schedule / preview-close / deploy success, false on pull_request. after
  always() (:183) right; comment must re-check PR, SHA, URL (G1 lists only number and commit).
- R2-3 preview-close.yml: no secret, no checkout, permissions {}: right. Low: fork closes also start reconcile with
  the token (reads nothing; only token-use frequency); a head_repository test would stop that, text change only.
- R2-4 wall-test.yml: three jobs name only WALL_TEST, so even at click 8 an admitted job gets no real token. M1
  (plan :331-339) with click 5 (:234-240) and click 8 (:254-256): right.
- R2-5 clicks 5-10 right; TOKEN-WINDOW (a) 90 days: agree. MAIN-WALL (plan :447-452): (b) leans on ACCOUNT (b) but
  does not say it is void under ACCOUNT (a); (a)'s rule must keep gate/smoke for his own merges (a ruleset bypass
  covers every rule of that ruleset: inferred, to measure).
- R2-6 section 13: finding 5's author filter and finding 6's runbook invariants (nothing else in the account, 2FA,
  revoke) are not in the plan (grep: no "github-actions[bot]", "author", "2FA", "revoke").
- R2-7 script omissions: find-pr, check-build (DOWNLOAD exact success), read-url (pr-$PR + real subdomain),
  verify-token (fixed host, three fields only), comment (author filter, re-check, stale-head skip), escaped logs,
  newline cases in M2.
- R2-8 VERDICT: APPROVE to go to Mustafa, with three Mediums folded in that do not touch the three texts.
