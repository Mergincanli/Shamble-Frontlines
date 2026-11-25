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
  `CLOUDFLARE_ACCOUNT_ID`. Only `main` may use that environment. The token lives nowhere else.
- The host: the Cloudflare account `Shamble previews`, project `shamble-previews`. That account holds nothing else.

## When something fails
Open the Actions tab, choose the workflow `preview`, and find the run started by the pull request's `toolchain` run.
The comment covers two things only: the upload's result and the header check. Anything that fails or stops the job
`check` (a missing build once main's `toolchain` hands one over, a refused build, a failed read of GitHub) shows only
as a red run under main in the Actions tab, with an error line; the comment says nothing that time.

1. **No comment after `toolchain` passed.**
   - `check` is red with "no build to preview: main's toolchain workflow hands one over, but this run's could not be
     fetched": re-run the pull request's `toolchain` run, or push again.
   - `check` is green, with "nothing to deploy: 0 open pull requests ... have <commit> as their newest commit": a newer
     push came first. Its own run brings the link.
   - `check` is red with "the build is refused": the build carried a file the refuse list refuses. The log names each
     one. Fix the build, not the list. The list changes only in a pull request of risk tier 2.
2. **The comment says the upload failed.** Read the `deploy` job's log.
   - "the token is not active": the token has ended or was revoked. Make a new one (below).
   - A Wrangler error: re-run the failed jobs once. If it fails again, it needs a fix in a pull request.
3. **The comment says the header check failed.** The link works, but a header is not the planned one. The `after`
   job's log lists every file with its headers and what is wrong. Fix `hosting/_headers` or the build in a pull
   request.
4. **`deploy` passed but no comment came.** Read the `after` job's comment step. If GitHub refused the job's token
   (`pull-requests: write`) the right to write the comment, that needs a fix in a pull request (risk R-COMMENT-PERM in
   step 2.6's report).
5. **Old previews are not deleted.** Read the `reconcile` job's log. It runs daily at 04:41 UTC, after each upload,
   and after each close. "could not all be read: nothing deleted" means Cloudflare's list failed, and the next run
   tries again. At most 20 previews are deleted per run; the rest wait for the next run. A pull request it could not
   look up is left alone.

## The 30-day rule
Cloudflare deletes "Preview deployments older than 30 days ... unless they are one of the project's 100 most recent
deployments" (Pages' limits page). If a pull request's link stops working after a long pause, push again or re-run its
`toolchain` run. A new upload brings the link back.

## The month's count
The comment shows the preview uploads made this month (UTC). Whether an upload counts against Cloudflare's 500 builds a
month is not known, so every upload is treated as if it counts. From 400 the comment and the log warn. Then hold back
re-runs. Anything beyond that (a paid plan) is Mustafa's decision, after a price read.

## The token
- It was made with a start date and an end date 90 days later. Every upload's log prints its status and both dates,
  never the token. The log warns from 14 days before the end.
- **To renew:** in Cloudflare, make a new token the same way (in the previews account: Manage Account, Account API
  Tokens, Create Token, Custom token, permission Account / Cloudflare Pages / Edit and Read, only that account, a
  start date and an end date 90 days on, no IP filter). Then replace `CLOUDFLARE_API_TOKEN` in the environment
  `preview`, and delete the old token in Cloudflare.
- **To revoke** (a leak, or when previews stop): Cloudflare, the previews account, Manage Account, Account API Tokens,
  the token, Delete. Previews stop until a new token is made as above.
- The token goes from Cloudflare's page into GitHub's secret box and nowhere else.
- Keep the previews account holding nothing else: no domain, no Worker, no storage, no other project, no other
  member. A leaked token then reaches previews only.
- Two-factor login stays on for Mustafa's Cloudflare user.

## Privacy
Previews are public but unlisted: every response carries `X-Robots-Tag: noindex`. Cloudflare's "Preview access" stays
off. Turning it on puts a login in front of every preview; that is decided when unreleased art or the first
multiplayer preview comes.

## Moving to Workers later
The upload becomes `wrangler preview --name pr-<number>` with a `wrangler.jsonc`, and the token needs Workers Scripts
Edit; the cleanup becomes `wrangler preview delete`. `_headers` stays, the 30-day rule becomes a cap of 100 previews
per Worker, and ADR 0006 asks for the hosting check to run again on Workers.
