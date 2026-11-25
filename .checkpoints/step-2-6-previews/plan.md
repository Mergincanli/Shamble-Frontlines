# Plan: step 2.6, a preview link for every pull request (lane 2, Builder V2)

Revision 2, 2026-11-25.
- **Branch:** `claude/optimistic-albattani-mdoonz`, standing in for `cs/step-2-6-previews`, at main c1da1bd plus
  checkpoint commits.
- **The design research:** infra-engineer's notes, `.checkpoints/step-2-6-previews/infra-engineer-design.md`.
- **The reads of revision 1** (081e7e9), in the same folder: `advisor-plan.md` (REVISE, C1 to C13, L1 to L8),
  `security-auditor-plan.md` (REVISE: one Critical that predates step 2.6, one High, Mediums) and
  `engine-architect-plan.md` (REVISE: one High, three Mediums). Section 13 maps each finding to its change.
- **Nothing is built before his go on this plan.** His go covers the three workflow texts in the Appendix, by their
  sha256. A line changed later in review comes back to him for a new yes (his rule of 2026-11-15, as in the hosting
  check).

## 1. Goal, in his words
These quotes are checked by script against his message of 2026-11-25, 18:15 Amsterdam: transcript 17:11:56 UTC, 1778
bytes, sha256 e2554c4e....

Item 3 reads: "Then step 2.6's plan, the preview links, as my order of 25 November has it. Plan first, your advisor
agent's review, then back to me for my go; nothing built before it."

The plan must hold four things:
- (a) "what my WORKERS-NEXT answer asks: the deploy step small enough to move, two lines on what a move would cost,
  and the 30-day rule for old previews" (sections 2.2 and 2.4);
- (b) "every click I have to do myself (the Cloudflare project, the deploy token, anything on GitHub), listed once, in
  the order I do them, so that I can do them in one sitting" (section 4);
- (c) "what you need from the CI Builder chat (the edit that hands the tested build to the preview job), as a request
  it can act on" (section 5);
- (d) "the day I can expect the first preview link, said honestly" (section 9).

His earlier words that bind the design:
- "I do not want to click for each preview" (2026-11-19, 00:01:25 UTC).
- STEP4-COUNT (b): "only the newest push of a pull request is deployed, and it counts its deploys per month".
- His locked-install point of 2026-11-16: a committed lockfile, with install scripts off.

What a player sees: nothing. What he gets: on every pull request from this repository, a comment with a link that
opens in Brave with no login, and no click of his before it.

## 2. The design

### 2.1 How the token stays safe without his click
**The trigger.** A new workflow, `preview`, starts when the workflow `toolchain` finishes (GitHub's `workflow_run`).
GitHub runs a `workflow_run` workflow from main's copy of its file, never from a branch's copy.

**The wall.** The token sits only in a GitHub environment named `preview`. Its one deployment rule is "branch `main`",
with no reviewer and no administrator bypass. Every job a branch can add or rewrite runs on a branch ref, and GitHub
refuses that ref the environment. The check measured the refusal (#66's report :113-114), and M1 measures it again
here, on this repository, before any real token goes in.

**Every job** (the texts are in the Appendix):

| Workflow and job | Starts on (its `if`) | Environment | Permissions | Secrets it names | Cache |
|---|---|---|---|---|---|
| preview / tests | its own pull requests (paths: `hosting/**`, the two workflow files) | none | contents: read | none | none |
| preview / check | `workflow_run` of `toolchain`: name, path `.github/workflows/toolchain.yml`, event `pull_request`, success, this repository | none | contents: read, actions: read, pull-requests: read | none | none |
| preview / deploy | `check` said go | `preview` (a deployment) | contents: read | the two, on two steps only | none |
| preview / after | `check` said go, whatever `deploy` did | none | contents: read, pull-requests: write, deployments: read | none | none |
| preview / reconcile | daily; the end of `preview-close`, matched by name and path; or a successful deploy | `preview` (no deployment record) | contents: read, pull-requests: read | the two, on one step | none |
| preview-close / closed | a pull request closes | none | none | none | none |
| wall-test / three jobs (thrown away before the review) | pull request; push to a branch | `preview` | none | `WALL_TEST`, a worthless test value | none |

**The guards the texts and scripts carry** (the security-auditor's G1 to G9):
- G1. No `${{ }}` of the event, of a job's outputs or of a step's outputs inside any shell block. Such values reach
  the shell only through `env:`, and are checked there:
  - a pull-request number must match `^[0-9]{1,6}$`;
  - a commit id must match `^[0-9a-f]{40}$`.

  The head branch, the title and the commit text are never read. Every checkout is main's, with
  `persist-credentials: false`, and the workflow sets `permissions: {}`.
- G2. The build is downloaded by its exact name, into a fresh folder outside the workspace. It never lands where it
  could cover main's `hosting/`.
- G3. The refuse list. `check` refuses a build that carries any of these:
  - a name starting with a dot;
  - a name outside a strict character set, a name starting with `-`, or `..`;
  - anything that is not a plain file or a folder;
  - `_worker.js`, `_routes.json`, `_redirects`, `_headers`, `functions/`, or `wrangler.toml`, `.json` or `.jsonc`;
  - more than 20,000 files, or any file of 25 MiB or more.

  Main's header file is then added.
- G4. Wrangler runs from a fresh empty folder outside the workspace, never a parent of the build's folder. So no
  `functions/`, `.env` or settings file can reach it.
- G5. `deploy` checks everything again:
  - the number and the commit;
  - the refuse list, on its own download from its own run;
  - the header file, byte for byte against main's.
- G6. The token sits on the token check and the upload step only, never job-wide. Wrangler's metrics are off, its
  debug log is never on, and no log of it is kept.
- G7. Nothing taken from the build is written to `GITHUB_ENV`, `GITHUB_OUTPUT`, `GITHUB_PATH` or the job summary. The
  link written to `GITHUB_OUTPUT` comes from Cloudflare's answer, checked against the link's shape
  (`^https://pr-[0-9]+\.[a-z0-9-]+\.pages\.dev$`).
- G8. No cache in any job (`cache-mode: none`; Node's package cache off).
- G9. A test fails when any of this breaks:
  - `${{` inside a shell block;
  - a job-wide token;
  - an `npm ci` without `--ignore-scripts`;
  - any cache.

**Production.** A token for one project is not on offer: the check's token form read "Entire Account" (#66 :352). A
reviewer is refused by his words. So OQ-0006-13's answer is this design plus his ACCOUNT choice, and only a separate
account keeps a future production out of the token's reach. No production deploy exists today, and this plan has
none.

**What GitHub does not enforce** (read today by the advisor agent and the security-auditor). Main's rule (ruleset
25069670) requires:
- a pull request, with 0 approvals;
- `gate` and `smoke` from the GitHub Actions app;
- no bypass actors ("current_user_can_bypass: never").

So nothing in GitHub reads a workflow's text before it reaches main. Worse, as the security-auditor infers (its
Critical, older than step 2.6): a branch's own workflow can ask for write permissions, pass its own `gate` and
`smoke`, and merge its own pull request. A planted `preview.yml` would then reach the token. In front of that today
stand only the kit's rules: tier 2 for `.github/` and `hosting/`, and the guard's ask before a `.github/` edit.
Decision MAIN-WALL (section 12) closes or accepts this.

**`hosting/` is tier 2:** a workflow runs its script with the token. The guard asks before a `.github/` edit but not
before a `hosting/` edit. Fixing that is a kit change, and the kit is frozen, so it becomes a STATUS follow-up.

### 2.2 The deploy step, small and movable (his WORKERS-NEXT)
- **The command:** `wrangler pages deploy <site> --project-name shamble-previews --branch pr-<N> --commit-hash <sha>
  --commit-message "pull request <N>" --commit-dirty=true`, run from an empty folder.
- **Where Wrangler comes from:** a committed lockfile, installed with `npm ci --ignore-scripts` into a fresh folder.
  Who writes that lockfile is decision LANE2-LOCKFILE. `cloudflare/wrangler-action` is not used: it installs at run
  time, with no lockfile.
- **Not yet proven:** that Wrangler's upload works with install scripts off. The check installed `wrangler@4.156.0`
  with scripts on and no lockfile (#66 :1431, :1512). If it fails, scripts stay off for the whole tree. At most, one
  named package that `pages deploy` loads, and whose script fetches nothing, may be rebuilt, as a recorded departure
  shown to him first.
- **What a move to Workers would cost**, in two lines:
  1. The command becomes `wrangler preview --name pr-<N>` with a `wrangler.jsonc`; the token needs Workers Scripts
     Edit; the cleanup becomes `wrangler preview delete`.
  2. `_headers` stays; the 30-day rule becomes a cap of 100 previews per Worker; and ADR 0006 asks for the hosting
     check to run again on Workers (one sitting of his, about two calendar days).

### 2.3 Only the newest push deploys, and the deploys are counted (STEP4-COUNT (b))
Four layers make sure of it:
- `toolchain` cancels its own older runs;
- the build is handed over from the pull-request run only;
- `check` deploys only when the run's commit is the pull request's newest commit;
- the `deploy` job runs one at a time per pull request, and a newer run cancels an older one.

The counter is GitHub's deployment records for `preview` this month. `reconcile` makes none, so the count is exact.
The comment prints it. Whether a direct upload counts against the 500 builds a month is not answered (PM-0006-7):
until someone shows a count, every deploy is treated as if it counts. The estimate is about 110 to 300 deploys a
month.

### 2.4 Cleanup, and the 30-day rule (his WORKERS-NEXT)
`reconcile` runs daily, after each deploy, and when a pull request closes. It takes the security-auditor's finding 4:
- it deletes a pull request's previews only after looking that pull request up and finding it closed, never because
  it is missing from a list;
- for an open pull request, it deletes its older uploads;
- the project name is fixed in the workflow;
- it touches preview deployments only, and only those whose branch, read from Cloudflare's record, matches
  `pr-<digits>`;
- it makes at most 20 deletions per run, and logs each before making it;
- it reads nothing from the run that started it.

**The 30-day rule.** Pages' limits page says "Preview deployments older than 30 days are deleted, unless they are one
of the project's 100 most recent deployments". Which deployments count among "the 100 most recent" is not stated:
- if only those still present count, an idle pull request's link never expires;
- if every deployment ever made counts, the link stops after 30 days.

The cheapest remedy is a new push, or a re-run of the pull request's `toolchain`. The runbook names it, and nothing is
built for it.

### 2.5 Headers (PHC-PREVIEW-NOINDEX)
- **The header file.** Lane 2's `hosting/_headers` carries `X-Robots-Tag: noindex` on `/*`, and `immutable` caching
  on the hashed asset folder; the immutable rule follows ADR 0006 part 1. Today the build writes no `_headers`. A
  build that carries one is refused, not silently replaced. The #59 request says that ADR 0002 part 10's header
  generator is parked (engine-architect's Medium).
- **The check.** After each deploy, `after` fetches each file of the build from the link's one host:
  - with no cookie and no redirect followed;
  - running no page script;
  - with file names passed as arguments, never through the shell.

  It compares the headers with the preview set.
- **The bite** (C1). The same check, run on the fetched responses with `immutable` removed in memory, must fail. That
  the host obeys a cache rule is already measured (#66 :33).
- **PHC-PREVIEW-NOINDEX:** a file test, in `tests`, that the header file carries `noindex` on `/*`, with a planted
  file that lacks it. No production file is deployed by this workflow. When production comes, its header set must
  carry the immutable rule too (GLOSSARY §5).

### 2.6 What the link shows (C11)
The link shows the pull-request run's build. That is the tested merge of the branch with main, so the page's corner
shows the merge commit. The comment names the pull request's newest commit and says this.

"Tested" means: the build is made in the same `toolchain` run whose tests, type checks and browser tests passed. The
browser tests run on the development build; the production folder is built in the same run and deployed.

### 2.7 Where a failure is seen
A `workflow_run` run shows under main in the Actions tab, not as a check on the pull request. So the comment says when
a preview failed, and why:
- the upload;
- the header check;
- a missing build once main's `toolchain.yml` carries the hand-over step.

Before that step is on main, a missing build ends with a notice only, so the two pull requests can merge in either
order (engine-architect's Medium).

### 2.8 Privacy
Previews are public but unlisted (ADR 0006 part 2): they run the match offline and carry no unreleased art. So
"Preview access" stays off. The same setting turns the wall on when unreleased art or the first multiplayer preview
comes (#66 :99-103); that is decided then.

## 3. Files
| File | What | Owner |
|---|---|---|
| `.github/workflows/preview.yml` | the jobs above (text in the Appendix) | lane 2 |
| `.github/workflows/preview-close.yml` | marks a close (text in the Appendix) | lane 2 |
| `.github/workflows/wall-test.yml` | M1's planted jobs; thrown away before the final review (text in the Appendix) | lane 2 |
| `hosting/package.json`, `hosting/package-lock.json` | Wrangler, pinned and locked; outside the npm workspace (root `package.json` covers `packages/*`) | decision LANE2-LOCKFILE |
| `hosting/_headers` | the preview header set | lane 2 |
| `hosting/preview.mjs` | one script with eight commands: `find-pr`, `check-build`, `recheck`, `verify-token`, `read-url`, `check-headers`, `comment`, `reconcile`; Node's built-ins only | lane 2 |
| `hosting/preview.test.mjs` | its tests on fixtures, each with a planted defect, plus G9's lint of `preview.yml`, run by the `tests` job | lane 2 |
| `docs/runbooks/previews.md` | when something fails; the 30-day rule; the wall's switch; the token's end date and renewal | lane 2 |
| STATUS, ROADMAP, REPO_MAP (`hosting/` and `docs/runbooks/` are new) and DECISIONS | the docs pass | lane 2's lines |

Not touched: `gate.yml`, `toolchain.yml`, the gate's files, `crates/`, `packages/`, `.claude/`, any draft under
`docs/adr/`.

## 4. His clicks, once, in order (one sitting, about 30 minutes)
The sitting comes after step 2.6's draft pull request is open, because M1 runs on it, and before its merge.

If ACCOUNT (b), first:
1. Cloudflare: Accounts, Create Account, named `Shamble previews`. If any screen shows a charge or asks for a card,
   stop and tell the session (as ACCESS-CARD).
2. Switch to that account. Steps 3, 4, 6 and 7 happen inside it.

Cloudflare:
3. Workers & Pages, Create, Pages, "Upload assets". Project `shamble-previews`. Drag in a folder with one placeholder
   `index.html` ("No game here"), then Deploy. That upload is the project's production version; no job ever deploys
   to it.
4. The project's Settings, "Preview access": leave it off.

GitHub (Shamble-Frontlines, Settings):
5. Environments, `preview`. A run of M1's file may already have made it; then open that one.
   - Required reviewers off; Wait timer off.
   - "Allow administrators to bypass configured protection rules" unticked.
   - Deployment branches and tags: "Selected branches and tags", Add rule, Branch, `main`.
   - Add the environment secret `WALL_TEST` with the value `not-a-token`.
   - Tell the session. It re-runs M1 and answers within about 3 minutes. If any planted job ran, stop: delete the
     environment, and no token goes in.

Cloudflare again:
6. Manage Account, Account API Tokens, Create Token, Custom token, named `shamble-previews-deploy`. It must be an
   account token, not one from his own profile.
   - Permissions: Account, Cloudflare Pages, Edit and Read.
   - Account: the summary must show only the previews account.
   - Client IP filtering: empty. GitHub "does not recommend" its runners' address list as an allow list.
   - Expiration: a start date of today AND an end date by decision TOKEN-WINDOW, each in its own field, never the
     single-date chip.
   - Create, then copy the token.
7. Copy the Account ID from the account's overview page.

GitHub again:
8. Environment `preview`: add the secrets `CLOUDFLARE_API_TOKEN` and `CLOUDFLARE_ACCOUNT_ID`. Tell the session:
   - it re-runs M1, and all three planted jobs must still be refused;
   - he re-reads the branch rule and the bypass box, then deletes `WALL_TEST`.
9. Secrets and variables: Actions and Dependabot. Confirm none is named `CLOUDFLARE_*`; a repository secret would
   reach every branch.
10. Actions, General:
    - workflow permissions "Read repository contents and packages permissions";
    - "Allow GitHub Actions to create and approve pull requests" unticked;
    - tick "Require actions to be pinned to a full-length commit SHA". Every action in the three workflows already is.

Click 8 waits for his MAIN-WALL answer. Under (a), it comes after main's new rule.

He sends no screenshot of the token. The first deploy asks Cloudflare whether the token is valid, and prints the
answer and its dates in its log. After the sitting, nothing is needed per preview.

## 5. The request to the CI Builder chat (on #59, as it can act on it)
> Lane 2 asks for one step at the end of job `toolchain` in `.github/workflows/toolchain.yml`, after "Build and test
> with the network cut off". Placed there, it runs outside the network cut, and only when every step above passed. The
> exact text is yours to write and his to approve; this is the proposal:
>
> ```yaml
>       # Hand the tested player build to the preview workflow (.github/workflows/preview.yml, which starts on
>       # `workflow_run` of this workflow's name `toolchain`). Pull requests from this repository only, after every step
>       # above passed. The preview workflow treats these files as data and runs none of them. Files under .gate/ may
>       # count as hidden for upload-artifact, hence include-hidden-files; an empty upload fails.
>       - name: Hand the tested build to the preview workflow (pull requests from this repository only)
>         if: github.event_name == 'pull_request' && github.event.pull_request.head.repo.full_name == github.repository
>         uses: actions/upload-artifact@cf430e030ddbb5b0abf93d22962f4752f3646cd9 # v7.0.2
>         with:
>           name: preview-site
>           path: .gate/dist-production/
>           include-hidden-files: true
>           if-no-files-found: error
>           retention-days: 7
> ```
> Notes:
> 1. When step 2.3 replaces this file, keep the workflow name `toolchain`, the file path and the artifact name
>    `preview-site`, or tell lane 2 first on #59.
> 2. Read the pin again from actions/upload-artifact itself.
> 3. One folder only, so the deploys do not double. If his Brave checks need the development page, the path is
>    `.gate/dist-development/` instead.
> 4. The preview refuses a build that carries `_headers`. ADR 0002 part 10's header generator is parked until the
>    drafts' next revision; please do not build it meanwhile.
>
> The preview downloads the build by the run's id, with `actions: read`, in its no-secret job only. Until this step is
> on main, a missing build ends with a notice, so the two pull requests can merge in either order.

## 6. Departures (decision DEPARTURES; no draft is edited, each goes to the drafts' next revision as a follow-up)
1. **B9's interim reviewer on `preview`** (ADR 0006 part 13; ADR 0002 :672-675) becomes the main-only rule plus
   `workflow_run`. The draft drops the reviewer itself "unless OQ-0006-13 puts production where the preview token
   cannot reach it": a previews-only account (ACCOUNT (b)) meets that clause.
2. **ADR 0002 part 10's "No `pull_request_target`, no `workflow_run`"** (and its options row, :889, "rejected for
   now"). The reason, finding 10, is that these triggers hand secrets to runs a pull request triggers. The research
   behind it sets two conditions for using `workflow_run` safely: treat artifacts with caution, and never check out
   pull-request code (r5 :775-782). This design meets both. engine-architect, the draft's owner, accepts the
   departure: firmly with ACCOUNT (b), and with (a) only while no production exists.
3. **The build plan's condition 4 for step 2.6** (:1021-1027): "a reviewer on previews" becomes section 2.1 and his
   sitting. Condition 3 is settled, not departed from: Part 15 :103 and his item 4 (DECISIONS :461) give the hand-over
   to CI Builder.
4. **Where `_headers` comes from.** ADR 0002 has the build generate the header sets. Here lane 2's file is used, a
   build that carries one is refused, and the generator is parked.
5. **The weekly sweep** (ADR 0006 part 3) becomes daily, plus a run after each deploy and on each close.
6. **Part 15's ownership** (:103, "the workflow file it adds for previews (a new file), the host's config files it
   adds, its runbook"): two workflow files and a script with tests, which is code rather than a config file. The
   lockfile is its own decision (LANE2-LOCKFILE).
7. **ADR 0002 part 10's one deploy job** becomes the jobs in the table. Only the token's job holds the token, with
   `contents: read`.
8. **The marks.**
   - PM-0002-65, the 5-minute link: M11 measures it.
   - PM-0002-66 and PM-0006-32, the reviewer marks: M1 replaces them.
   - Part 10's "View deployment" link on the pull request becomes the comment: a `workflow_run` deployment sits on
     main, not on the pull request.
9. **ADR 0002 :643-646 and :686-689** lean on the preview reviewer for the Sentry upload job and for the multiplayer
   preview's binary delivery. This plan's reasoning covers static files only, and is no precedent for those two jobs.

## 7. Pass marks
Before the merge:
- **M1, the wall.** The throwaway file holds three planted jobs naming `preview`, each with no permissions, one step
  each, printing only "present" or "absent":
  - a pull-request job, plainly;
  - a pull-request job with `deployment: false`;
  - a push to the branch.

  The bite: before click 5, they run and print "absent". The pass, after click 5 and again after click 8: GitHub
  refuses all three before any step runs, with its refusal line quoted, and makes no deployment record. The file
  leaves the branch before the final review.
- **M2, the script's tests.** On fixtures, each with a planted defect:
  - every refuse-list entry;
  - the commit match;
  - reconcile's look-up-first and its cap of 20;
  - the comment built from checked values only;
  - the header file test;
  - G9's lint of `preview.yml`.
- **M3, the lockfile.** The security-auditor's answer on the exact manifest and lockfile is in before the push (the
  dependency hold). The answer checks:
  - the manifest: private, Wrangler alone at an exact version, no lifecycle scripts, no overrides, outside the
    workspace;
  - the lockfile: version 3, every `resolved` on registry.npmjs.org, sha512 integrity on every entry, the release at
    least 7 days old, packages with install scripts listed;
  - the outputs of `npm ci --ignore-scripts`, `npm audit` and `npm audit signatures`.
- **M4.** The gate passes. Review round 1 on the final diff.

After the merge, on the first pull request after both merges (step 2.6's own pull request cannot preview itself):
- **M5:** the link opens with its build marker, and the token check prints a valid status.
- **M6:** a second push replaces the first.
- **M7:** the header check passes, and its in-memory bite fails.
- **M8:** a closed pull request's link is gone within 24 hours.
- **M9:** the month's count shows in the comment.
- **M10:** no secret appears in any log.
- **M11:** the time from `toolchain`'s end to the link comment, the median of the first 5 runs, against PM-0002-65's
  5 minutes.

**His Brave checklist** for the first link:
1. In the comment, compare the version the page will show with the merge commit named there.
2. Open the link and see the page load, with no login.
3. Read the build marker in the page's corner.
4. Open the link again in a private window.

## 8. Reviewers
- **Before revision 2 goes to him:** the security-auditor reads it, texts included, and the advisor agent re-reviews
  what changed (C13).
- **The build's final diff:** the security-auditor (tier 2), test-engineer (are the marks proven, do the bites bite?)
  and infra-engineer (CLAUDE.md step 5). engine-architect's read of departure 2 is already in.
- At most 3 agents at a time, on Opus, effort `xhigh` for the security and design reads.

## 9. The day of the first link, said honestly
The clock starts at his go on this revision. Then:
- the build and review round 1 with its fix pass: 2 working days;
- his sitting (section 4), with M1 in it;
- his go to merge;
- CI Builder's hand-over merged, in its own pull request, likely the slowest part;
- then the next pull request's push. The link comes about 20 minutes after it.

So the best case is 3 working days; with one fix after the first run (likely), 4 to 6. If his go comes on Thursday 26
November: the best day is Tuesday 1 December, the honest day Wednesday 2 to Friday 4 December. If a review or CI
Builder's pull request slips, it is the week of 7 December.

MAIN-WALL (a) adds the time to design and measure main's new rule before click 8: about 2 to 4 working days.

## 10. Order of work, after his go (each with a checkpoint save)
- **U1.** The host folder: the script, its tests and the header file. Then the manifest and lockfile, by
  LANE2-LOCKFILE, with M3 before the push.
- **U2.** The three workflow files, byte for byte as the Appendix gives them.
- **U3.** The records:
  - his messages of 2026-11-25 into DECISIONS by script: 17:05 Amsterdam (16:41:51 UTC, 1094 bytes, sha256 c41b21e8...)
    and 18:15 Amsterdam (17:11:56 UTC, 1778 bytes, sha256 e2554c4e...);
  - his answers to section 12;
  - the dated pointer in #67's report (his item 4).
- **U4.** The request to CI Builder on #59.
- **U5.** A draft pull request, then his sitting with M1 on it.
- **U6.** The throwaway file out.
- **U7.** Review round 1 on the final diff, then the fix pass. A changed line goes back to him for a new yes.
- **U8.** The report and the docs pass, then ready, then his go to merge.
- **U9.** The first link (M5 to M11) and his Brave checklist.

## 11. Risks
- **R1.** GitHub does not stop a change to `preview.yml` itself, and may let a branch merge itself (section 2.1).
  Bounded by ACCOUNT (b); closed or accepted by MAIN-WALL.
- **R2.** `deployment: false` may not hold the branch rule the way a normal job does: the docs imply it and do not say
  it. M1 tests exactly that, with a worthless secret, before any token goes in.
- **R3.** Wrangler with install scripts off is unproven (section 2.2).
- **R4.** `workflow_run`'s payload and the pull-request lookup are proven only on the first real run.
- **R5.** The 30-day rule's reading is unknown (section 2.4).
- **R6.** The guard does not ask before a `hosting/` edit (a STATUS follow-up; the kit is frozen).
- **R7.** Each fix after the merge is a tier-2 pull request: the advisor agent, the security-auditor, his go.

## 12. Decisions needed (his; the advisor agent settled FIRST-RUN (a), merge and fix on main; LABEL, the pull-request
run's build; and MAIN-BYPASS, read: no bypass actors)
1. **TRIGGER: how a preview gets the token without his click.**
   - (a) `workflow_run` plus the main-only environment.
   - (b) The ordinary pull-request run with no reviewer: any branch could take the token.
   - (c) Keep a reviewer, which is his click per preview.
   - Recommendation: (a).
2. **ACCOUNT: where the previews live.**
   - (a) His current Cloudflare account.
   - (b) A new free account for previews only: three more clicks in the same sitting. A leaked token then reaches
     previews only.
   - Recommendation: (b).
3. **LANE2-LOCKFILE: who writes the lockfile that pins Wrangler?** His accepted Part 15 keeps "every Cargo.toml and
   package.json, both lockfiles" with lane 1, and his words of 2026-11-18 to lane 2 read "You own no manifest and no
   lockfile".
   - (a) Lane 1 adds the two files on a #59 request (its own small pull request; slower).
   - (b) He lets lane 2 own this one lockfile, for the host tool only, outside the npm workspace.
   - Recommendation: (b), for speed. Either way, Wrangler stays out of the game's install, and the security review
     (M3) applies.
4. **DEPARTURES: may the nine departures of section 6 stand?**
   - (a) All stand.
   - (b) He names the ones that go back to the drafts first.
   - Recommendation: (a).
5. **TOKEN-WINDOW: the token's end date.**
   - (a) 90 days (the security-auditor).
   - (b) 6 months (infra-engineer and the advisor agent).
   - Recommendation: (a). A renewal is clicks 6 and 8 again, and the runbook and STATUS carry the date.
6. **MAIN-WALL: the gap that predates step 2.6** (the security-auditor's Critical; section 2.1).
   - (a) A main rule that lets only his account update main, so that no workflow can merge itself. infra-engineer
     designs it, measured on a throwaway repository, in its own small change-set; click 8 waits for it.
   - (b) He accepts the risk in writing for now, bounded by ACCOUNT (b), with (a) following as its own change-set.
   - Recommendation: the security-auditor recommends (a). This chat agrees: the gap reaches the whole repository, not
     only previews.

## 13. What changed from revision 1, by finding
| Finding | Change |
|---|---|
| Advisor C1 | 2.5: the bite is in memory |
| Advisor C2 | 6: departures 6 and 8; decision DEPARTURES |
| Advisor C3 | click 6's end date (TOKEN-WINDOW); click 11 deleted; 2.1 states main's rule as read |
| Advisor C4 | M1, and click 5's test secret |
| Advisor C5 | M11 |
| Advisor C6 | `hosting/` is tier 2; R6 |
| Advisor C7 | the jobs table |
| Advisor C8 | 10, the order |
| Advisor C9 | 1, the quotes checked |
| Advisor C10 | the Appendix |
| Advisor C11 | 2.6 and the Brave checklist |
| Advisor C12 | click 1 |
| Advisor C13 | 8 |
| Advisor L1 to L8 | 2.1 (G3), 2.2, 2.3, 3, 2.5, 6, the checkpoint |
| Security Critical | 2.1, R1, MAIN-WALL |
| Security G1 to G9 | 2.1 and the texts |
| Security findings 3 to 7 | M1, 2.4, 2.5, TOKEN-WINDOW and click 6, clicks 9 and 10 |
| Security Low | 2.2, M3 |
| engine-architect High | LANE2-LOCKFILE |
| engine-architect Mediums | G1, 2.5, 2.7 |
| engine-architect Lows | 6.9, 2.1, 2.6, 3 |

## Appendix: the three workflow texts his go covers (each byte for byte, with its sha256)

### `.github/workflows/preview.yml`: 10561 bytes, sha256 c66c0b0a9351a528695908c57f7256d2cd18713219c5698a17eaebaac469c28e
(the same bytes as `.checkpoints/step-2-6-previews/texts/preview.yml`)

```yaml
# Step 2.6 (lane 2): a preview link for every pull request from this repository.
# The plan: step 2.6's plan, revision 2 (in its report); what to do when something fails: docs/runbooks/previews.md.
# The token lives only in the environment `preview`, whose one deployment rule is the branch `main`; no reviewer.
# Its workflow_run and schedule events run main's copy of this file; on a pull request only the job `tests` runs,
# with no secret. Values from the event or from another job reach a shell only through `env:`, and are checked there.
# The build is data: it is checked and uploaded, never run. No cache in any job.
name: preview

on:
  workflow_run:
    workflows: [toolchain, preview-close]
    types: [completed]
  schedule:
    - cron: "41 4 * * *"
  pull_request:
    paths:
      - "hosting/**"
      - ".github/workflows/preview.yml"
      - ".github/workflows/preview-close.yml"

permissions: {}

defaults:
  run:
    shell: bash

env:
  PROJECT: shamble-previews
  WRANGLER_SEND_METRICS: "false"

jobs:
  tests:
    # No secret, no environment: the scripts' tests, this file's own lint and the header file's test, each with a
    # planted defect that must turn it red.
    if: github.event_name == 'pull_request'
    runs-on: ubuntu-24.04
    timeout-minutes: 10
    cache-mode: none
    permissions:
      contents: read
    steps:
      - uses: actions/checkout@3d3c42e5aac5ba805825da76410c181273ba90b1 # v7.0.1
        with:
          persist-credentials: false
      - uses: actions/setup-node@820762786026740c76f36085b0efc47a31fe5020 # v7.0.0
        with:
          node-version: 22
          package-manager-cache: false
      - name: The scripts' tests, the workflow lint and the header file test
        run: node --test hosting/

  check:
    # No secret. Only a successful pull-request run of main's toolchain workflow, from this repository.
    if: >-
      github.event_name == 'workflow_run' &&
      github.event.workflow_run.name == 'toolchain' &&
      github.event.workflow_run.path == '.github/workflows/toolchain.yml' &&
      github.event.workflow_run.event == 'pull_request' &&
      github.event.workflow_run.conclusion == 'success' &&
      github.event.workflow_run.head_repository.full_name == github.repository
    runs-on: ubuntu-24.04
    timeout-minutes: 10
    cache-mode: none
    permissions:
      contents: read
      actions: read
      pull-requests: read
    outputs:
      pr: ${{ steps.pr.outputs.pr }}
      sha: ${{ steps.pr.outputs.sha }}
      go: ${{ steps.site.outputs.go }}
    steps:
      - uses: actions/checkout@3d3c42e5aac5ba805825da76410c181273ba90b1 # v7.0.1
        with:
          persist-credentials: false
      - uses: actions/setup-node@820762786026740c76f36085b0efc47a31fe5020 # v7.0.0
        with:
          node-version: 22
          package-manager-cache: false
      - name: Find the open pull request whose newest commit is this run's commit
        id: pr
        env:
          GH_TOKEN: ${{ github.token }}
          REPO: ${{ github.repository }}
          HEAD_SHA: ${{ github.event.workflow_run.head_sha }}
        run: node hosting/preview.mjs find-pr
      - name: Fetch this run's build into a fresh folder outside the workspace
        id: dl
        if: steps.pr.outputs.pr != ''
        continue-on-error: true
        uses: actions/download-artifact@9000827ccba6bdab643e8b6fd33ac0654aef8333 # v8.0.2
        with:
          name: preview-site
          run-id: ${{ github.event.workflow_run.id }}
          github-token: ${{ github.token }}
          path: ${{ runner.temp }}/site-raw
      - name: Check the build (the refuse list), then add main's header file
        id: site
        if: steps.pr.outputs.pr != ''
        env:
          DOWNLOAD: ${{ steps.dl.outcome }}
        run: node hosting/preview.mjs check-build
      - name: Hand the checked build to the deploy job (this run only, kept one day)
        if: steps.site.outputs.go == 'true'
        uses: actions/upload-artifact@cf430e030ddbb5b0abf93d22962f4752f3646cd9 # v7.0.2
        with:
          name: preview-site-checked
          path: ${{ runner.temp }}/site-checked
          if-no-files-found: error
          retention-days: 1

  deploy:
    # The only job with the deploy token, on two steps only. It runs nothing from the build.
    needs: check
    if: needs.check.outputs.go == 'true'
    runs-on: ubuntu-24.04
    timeout-minutes: 10
    cache-mode: none
    environment:
      name: preview
      url: ${{ steps.url.outputs.url }}
    concurrency:
      group: preview-pr-${{ needs.check.outputs.pr }}
      cancel-in-progress: true
    permissions:
      contents: read
    outputs:
      url: ${{ steps.url.outputs.url }}
    steps:
      - uses: actions/checkout@3d3c42e5aac5ba805825da76410c181273ba90b1 # v7.0.1
        with:
          persist-credentials: false
      - uses: actions/setup-node@820762786026740c76f36085b0efc47a31fe5020 # v7.0.0
        with:
          node-version: 22
          package-manager-cache: false
      - name: Check the pull request's number and commit again
        env:
          PR: ${{ needs.check.outputs.pr }}
          SHA: ${{ needs.check.outputs.sha }}
        run: |
          [[ "$PR" =~ ^[0-9]{1,6}$ ]] || { echo "not a pull request number"; exit 1; }
          [[ "$SHA" =~ ^[0-9a-f]{40}$ ]] || { echo "not a commit id"; exit 1; }
      - name: Fetch the checked build from this run only
        uses: actions/download-artifact@9000827ccba6bdab643e8b6fd33ac0654aef8333 # v8.0.2
        with:
          name: preview-site-checked
          path: ${{ runner.temp }}/site
      - name: Check it again, and compare its header file with main's byte for byte
        run: node hosting/preview.mjs recheck
      - name: Install Wrangler from main's lockfile into a fresh folder, install scripts off
        run: |
          mkdir -p "$RUNNER_TEMP/wrangler" "$RUNNER_TEMP/empty"
          cp hosting/package.json hosting/package-lock.json "$RUNNER_TEMP/wrangler/"
          cd "$RUNNER_TEMP/wrangler"
          npm ci --ignore-scripts --no-audit --no-fund
      - name: Ask Cloudflare whether the token is valid (prints its status and dates, never the token)
        env:
          CLOUDFLARE_API_TOKEN: ${{ secrets.CLOUDFLARE_API_TOKEN }}
          CLOUDFLARE_ACCOUNT_ID: ${{ secrets.CLOUDFLARE_ACCOUNT_ID }}
        run: node hosting/preview.mjs verify-token
      - name: Upload the build as pr-<number>, run from an empty folder
        working-directory: ${{ runner.temp }}/empty
        env:
          CLOUDFLARE_API_TOKEN: ${{ secrets.CLOUDFLARE_API_TOKEN }}
          CLOUDFLARE_ACCOUNT_ID: ${{ secrets.CLOUDFLARE_ACCOUNT_ID }}
          PR: ${{ needs.check.outputs.pr }}
          SHA: ${{ needs.check.outputs.sha }}
          WRANGLER_OUTPUT_FILE_PATH: ${{ runner.temp }}/wrangler-output.ndjson
        run: |
          [[ "$PR" =~ ^[0-9]{1,6}$ ]] && [[ "$SHA" =~ ^[0-9a-f]{40}$ ]] || exit 1
          "$RUNNER_TEMP/wrangler/node_modules/.bin/wrangler" pages deploy "$RUNNER_TEMP/site" \
            --project-name "$PROJECT" --branch "pr-$PR" --commit-hash "$SHA" \
            --commit-message "pull request $PR" --commit-dirty=true
      - name: Read the link from Wrangler's record (checked against the link's shape)
        id: url
        run: node hosting/preview.mjs read-url

  after:
    # No secret. The served headers on the link (with the check's bite), then one comment on the pull request,
    # written from checked values only.
    needs: [check, deploy]
    if: always() && needs.check.outputs.go == 'true'
    runs-on: ubuntu-24.04
    timeout-minutes: 10
    cache-mode: none
    permissions:
      contents: read
      pull-requests: write
      deployments: read
    steps:
      - uses: actions/checkout@3d3c42e5aac5ba805825da76410c181273ba90b1 # v7.0.1
        with:
          persist-credentials: false
      - uses: actions/setup-node@820762786026740c76f36085b0efc47a31fe5020 # v7.0.0
        with:
          node-version: 22
          package-manager-cache: false
      - name: Fetch the checked build's file list from this run
        if: needs.deploy.result == 'success'
        uses: actions/download-artifact@9000827ccba6bdab643e8b6fd33ac0654aef8333 # v8.0.2
        with:
          name: preview-site-checked
          path: ${{ runner.temp }}/site
      - name: Check the served headers, then the bite (the same check on responses with the cache rule removed must fail)
        id: headers
        if: needs.deploy.result == 'success'
        env:
          URL: ${{ needs.deploy.outputs.url }}
        run: node hosting/preview.mjs check-headers
      - name: Write or update the link comment
        if: always()
        env:
          GH_TOKEN: ${{ github.token }}
          REPO: ${{ github.repository }}
          PR: ${{ needs.check.outputs.pr }}
          SHA: ${{ needs.check.outputs.sha }}
          URL: ${{ needs.deploy.outputs.url }}
          DEPLOY: ${{ needs.deploy.result }}
          HEADERS: ${{ steps.headers.outcome }}
        run: node hosting/preview.mjs comment

  reconcile:
    # Deletes the previews of closed pull requests, and the older uploads of open ones. It reads nothing from the run
    # that started it: it looks every pull request up itself, deletes at most 20 per run, and logs each before it.
    needs: deploy
    if: >-
      always() && (
        github.event_name == 'schedule' ||
        needs.deploy.result == 'success' ||
        (github.event_name == 'workflow_run' &&
         github.event.workflow_run.name == 'preview-close' &&
         github.event.workflow_run.path == '.github/workflows/preview-close.yml'))
    runs-on: ubuntu-24.04
    timeout-minutes: 10
    cache-mode: none
    environment:
      name: preview
      deployment: false
    concurrency:
      group: preview-reconcile
      cancel-in-progress: false
    permissions:
      contents: read
      pull-requests: read
    steps:
      - uses: actions/checkout@3d3c42e5aac5ba805825da76410c181273ba90b1 # v7.0.1
        with:
          persist-credentials: false
      - uses: actions/setup-node@820762786026740c76f36085b0efc47a31fe5020 # v7.0.0
        with:
          node-version: 22
          package-manager-cache: false
      - name: Reconcile the project's previews with the open pull requests
        env:
          CLOUDFLARE_API_TOKEN: ${{ secrets.CLOUDFLARE_API_TOKEN }}
          CLOUDFLARE_ACCOUNT_ID: ${{ secrets.CLOUDFLARE_ACCOUNT_ID }}
          GH_TOKEN: ${{ github.token }}
          REPO: ${{ github.repository }}
        run: node hosting/preview.mjs reconcile
```

### `.github/workflows/preview-close.yml`: 515 bytes, sha256 c2f1fa90511db0f558d4e9287d64e189877f8cbc03eb27e6efbe08787fe20e66
(the same bytes as `.checkpoints/step-2-6-previews/texts/preview-close.yml`)

```yaml
# Step 2.6 (lane 2): when a pull request closes, the end of this run starts the preview workflow's reconcile on main,
# which looks every pull request up itself. No secret, no checkout; nothing of the pull request is read.
name: preview-close

on:
  pull_request:
    types: [closed]

permissions: {}

jobs:
  closed:
    runs-on: ubuntu-24.04
    timeout-minutes: 2
    cache-mode: none
    steps:
      - name: Mark the close (the reconcile runs on main after this run)
        run: echo "A pull request closed."
```

### `.github/workflows/wall-test.yml (throwaway)`: 1415 bytes, sha256 3af067068060971beba8444346fdb836b7779a41403ac09d25e43027e8b18507
(the same bytes as `.checkpoints/step-2-6-previews/texts/wall-test.yml`)

```yaml
# THROWAWAY: step 2.6's pass mark M1, removed before the final review. Each job names the environment `preview` and
# prints only whether a worthless test secret is present. Before Mustafa's click 5 the jobs run and print "absent";
# after it, GitHub must refuse all three before any step runs.
name: wall-test

on:
  pull_request:
  push:
    branches-ignore: [main]

permissions: {}

jobs:
  pr-plain:
    if: github.event_name == 'pull_request'
    runs-on: ubuntu-24.04
    timeout-minutes: 2
    cache-mode: none
    environment: preview
    steps:
      - name: Is the test secret present?
        env:
          S: ${{ secrets.WALL_TEST }}
        run: if [ -n "$S" ]; then echo present; else echo absent; fi

  pr-no-deployment-record:
    if: github.event_name == 'pull_request'
    runs-on: ubuntu-24.04
    timeout-minutes: 2
    cache-mode: none
    environment:
      name: preview
      deployment: false
    steps:
      - name: Is the test secret present?
        env:
          S: ${{ secrets.WALL_TEST }}
        run: if [ -n "$S" ]; then echo present; else echo absent; fi

  push-to-branch:
    if: github.event_name == 'push'
    runs-on: ubuntu-24.04
    timeout-minutes: 2
    cache-mode: none
    environment: preview
    steps:
      - name: Is the test secret present?
        env:
          S: ${{ secrets.WALL_TEST }}
        run: if [ -n "$S" ]; then echo present; else echo absent; fi
```
