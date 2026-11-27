# Runbook: the preview links (foundation step 2.6, lane 2)

What it does: on every pull request from this repository, once the `toolchain` workflow has passed, the `preview`
workflow uploads the tested player build to Cloudflare Pages and writes one comment on the pull request with the link,
`https://pr-<number>.<the project's address>`. Nobody clicks anything per preview. The link opens with no login.

Where everything is:
- `.github/workflows/preview.yml`: the jobs (`tests`, `check`, `deploy`, `after`, `reconcile`). Its `workflow_run`
  and schedule runs use main's copy of the file, and they show in the Actions tab under main, not as a check on the
  pull request. `.github/workflows/preview-close.yml` starts a cleanup when a pull request closes.
- `hosting/preview.mjs`: the one script, one command per step. `hosting/preview.test.mjs`: its tests
  (`node --test hosting/preview.test.mjs`). `hosting/_headers`: the header set every preview gets.
  `hosting/package.json` and `hosting/package-lock.json`: Wrangler, the upload tool, pinned.
- The token: GitHub, the repository's Settings, Environments, `preview`: the secrets `CLOUDFLARE_API_TOKEN` and
  `CLOUDFLARE_ACCOUNT_ID`, added with that page's "Add environment secret". Never as a repository, Dependabot or
  Codespaces secret, and never as a variable: those reach every branch. The token lives nowhere else.
- The host: the Cloudflare account `Shamble previews`, project `shamble-previews`. That account holds nothing else.

## The environment page (the wall in front of the token)
GitHub, Settings, Environments, `preview` must keep:
- Deployment branches and tags: "Selected branches and tags", "1 branch and 0 tags allowed", the rule `main`.
  "No restriction", "Protected branches only", a wildcard or a tag rule would let other branches in.
- Required reviewers and Wait timer off.
- "Allow administrators to bypass configured protection rules" unticked AND saved: the box takes effect only after
  "Save protection rules".

After any change on that page, reload it and check the box and the rule again. Why it matters: every run in this
repository starts from Mustafa's account, an administrator, and a re-run keeps the first run's account. On 2026-11-25
three planted branch jobs reached the environment's test secret at 22:29 UTC. After the page was saved and reloaded,
GitHub refused all three at 22:56 UTC ("Branch ... is not allowed to deploy to preview due to environment protection
rules"). The likely cause was the unsaved bypass box; the `main` rule may also have been saved only after 22:29.

## The hand-over from `toolchain`
Since PR #74 (2026-11-26; the CI Builder chat's step, asked on issue #59), main's `toolchain` workflow ends, on pull
requests from this repository only, with a check that the tested build holds only plain files and folders, then hands
it over as the artifact `preview-site`, kept 7 days. The `check` job fetches it from that run. If the hand-over step
ever leaves main's `toolchain.yml`, every preview run ends in `check` with the notice "no build to preview: main's
toolchain workflow does not hand one over yet" and writes no comment.

## When something fails
Open the Actions tab, choose the workflow `preview`, and find the run started by the pull request's `toolchain` run.
The comment covers two things only: the upload's result and the header check. Anything that fails or stops the job
`check` (a missing build once main's `toolchain` hands one over, a refused build, a failed read of GitHub) shows only
as a red run under main in the Actions tab, with an error line; the comment says nothing that time.

1. **No comment after `toolchain` passed.**
   - `check` is red with "no build to preview: main's toolchain workflow hands one over, but this run's could not be
     fetched": re-run the pull request's `toolchain` run (possible up to 30 days after it), or push a new commit.
   - `check` is green, with "nothing to deploy: 0 open pull requests ... have <commit> as their newest commit": a newer
     push came first. Its own run brings the link.
   - `check` is red with "the build is refused": the build carried a file the refuse list refuses. The log names each
     one. Fix the build, not the list. The list changes only in a pull request of risk tier 2.
2. **The comment says the upload failed.** Read the `deploy` job's log.
   - "the token is not active": the token has ended or was revoked. Make a new one (below).
   - "the project ... could not be read", or "the project's production branch is pr-...": the project was removed,
     renamed or changed in Cloudflare. Put it right there (see "Rebuild from zero").
   - A Wrangler error: re-run the failed jobs within a day (the checked build is kept for one day); after that, push a
     new commit. A re-run runs main's files as they were at the first run. If Cloudflare refuses because a monthly limit
     is reached, a re-run does not help (see "The month's count").
3. **The comment says the header check failed.** The link works, but a header is not the planned one. The `after`
   job's log lists every file with its headers and what is wrong. Fix `hosting/_headers` or the build in a pull
   request.
4. **`deploy` passed but no comment came.** Read the `after` job's comment step. The job holds
   `pull-requests: write`, which GitHub's docs say is enough to write the comment; if GitHub refused it all the same,
   that needs a fix in a pull request.
5. **Old previews are not deleted.** Read the `reconcile` job's log. It runs daily at 04:41 UTC, after each upload,
   and after each close. "could not all be read: nothing deleted" means Cloudflare's list failed, and the next run
   tries again. At most 20 previews are deleted per run; the rest wait for the next run. A pull request it could not
   look up is left alone. A `reconcile` run shown as cancelled is normal: a newer one took its place in the queue.

## Stopping previews at once
Actions, the workflow `preview`, the "..." menu, "Disable workflow"; then revoke the token (below). With the token
gone and the workflow still on, the daily `reconcile` turns red until the workflow is disabled. GitHub also switches
off schedules in a public repository after 60 days without activity.

## The 30-day rule
Cloudflare deletes "Preview deployments older than 30 days ... unless they are one of the project's 100 most recent
deployments" (Pages' limits page). If a pull request's link stops working after a long pause, push a new commit to it:
a new upload brings the link back. A re-run of its `toolchain` run works only up to 30 days after that run, so it
cannot bring back a link this rule removed.

## The month's count
The comment shows the preview deploy attempts made this month (UTC): GitHub's deployment records for `preview`. Every
attempt counts, refused, failed, cancelled and re-run ones included, so the number is an upper bound on uploads; in
November 2026 it includes the environment check's planted jobs. Whether an upload counts against Cloudflare's 500
builds a month is not known, so every upload is treated as if it counts. From 400 the comment and the log warn. Then
hold back re-runs. Anything beyond that (a paid plan) is Mustafa's decision, after a price read.

## The token
- It is made with a start date and an end date 90 days later, each in its own field, never the single-date chip (the
  hosting check's chip gave a token that worked for one day). Its end date is written in STATUS when it is made, with
  a reminder 14 days before. Every upload's log prints the token's status and both dates, never the token, and warns
  from 14 days before the end.
- **To renew**, in this order:
  1. GitHub, Settings, Environments, `preview`: check the page as described above, click "Save protection rules",
     reload and check again.
  2. Cloudflare, the previews account: Manage Account, Account API Tokens, Create Token, Custom token. Permissions:
     Account, Cloudflare Pages, Edit, and a second row with Read. Only the previews account. No IP filter. A start date
     and an end date 90 days on, each in its own field. Create, then copy the token.
  3. Replace `CLOUDFLARE_API_TOKEN` with the environment page's secret box. Then check Settings, Secrets and
     variables (Actions, Dependabot, Codespaces): no name starts with `CLOUDFLARE_`.
  4. On the next upload, the `deploy` log's token line shows "active" with the new dates.
  5. Then delete the old token in Cloudflare, and update the end date in STATUS.
- **To revoke** (a leak, or when previews stop): Cloudflare, the previews account, Manage Account, Account API Tokens,
  the token, Delete. Previews stop until a new token is made as above.
- The token goes from Cloudflare's page into GitHub's secret box and nowhere else.
- Keep the previews account holding nothing else: no domain, no Worker, no storage, no other project, no other
  member. A leaked token then reaches previews only.
- Two-factor login stays on for Mustafa's Cloudflare user.

## The upload tool (Wrangler)
`hosting/package.json` pins Wrangler at one exact version and `hosting/package-lock.json` locks every package under it;
the `deploy` job installs them with install scripts off, beside the token. A bump is a pull request of risk tier 2,
with a new dependency check before it is pushed: the security-auditor's answer on the exact two files, `npm audit`, and
the registry signatures, including any package `npm audit signatures` skips (in 4.164.0: `@img/sharp-wasm32`).

## Rebuild from zero
If the previews account, its project or the environment has to be made again:
1. Cloudflare: Accounts, Create Account, named `Shamble previews` (the user must be at least seven days old and a
   Super Administrator; up to five extra free accounts). Switch to it.
2. Workers & Pages, Create application, then at the bottom "Continue to Pages", "Drag and drop your files", Get
   started. Project name `shamble-previews`; drag in a folder or zip holding one `index.html` ("No game here"); Deploy
   site. Leave the project's production branch as given, never a `pr-<number>` name. Settings, "Preview access": off.
3. GitHub: Settings, Environments, `preview`. A run that names a missing environment creates it with no rules, so set
   it up at once: as in "The environment page" above, including "Save protection rules" and the reload.
4. The token: "To renew", steps 2 to 5, with two differences. In step 3 both secrets are new, so each is added with the
   environment page's "Add environment secret": the token as `CLOUDFLARE_API_TOKEN`, and the account ID (on the
   account's overview page) as `CLOUDFLARE_ACCOUNT_ID`. In step 5 there is no old token to delete, but the end date
   still goes in STATUS, with a reminder 14 days before.
5. A new project address changes nothing else: the `deploy` job reads it from Cloudflare on every upload.

## Privacy
Previews are public but unlisted: every response carries `X-Robots-Tag: noindex`. Pages adds that header to previews
itself, so the served header check cannot show that `hosting/_headers`'s own noindex rule applied; the file's test
does. Cloudflare's "Preview access" stays off. Turning it on puts a login in front of every preview; that is decided
when unreleased art or the first multiplayer preview comes.

## Moving to Workers later
The upload becomes `wrangler preview --name pr-<number>` with a `wrangler.jsonc`, and the token needs Workers Scripts
Edit; the cleanup becomes `wrangler preview delete`. `_headers` stays, the 30-day rule becomes a cap of 100 previews
per Worker, and ADR 0006 asks for the hosting check to run again on Workers.
