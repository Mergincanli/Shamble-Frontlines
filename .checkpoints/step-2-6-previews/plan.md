# Plan: step 2.6, a preview link for every pull request (lane 2, Builder V2)

Revision 1, 2026-11-25. Branch `claude/optimistic-albattani-mdoonz` (stands in for `cs/step-2-6-previews`), at main
c1da1bd plus checkpoint commits. The design research is infra-engineer's, with sources and dates:
`.checkpoints/step-2-6-previews/infra-engineer-design.md` (D1 to D8; its last section overrides the earlier blocks).
Nothing is built before his go on this plan.

## 1. Goal, in his words
His order of 2026-11-25, 18:15 Amsterdam, item 3: "Then step 2.6's plan, the preview links, as my order of 25 November
has it. Plan first, your advisor agent's review, then back to me for my go; nothing built before it."

The plan must hold four things:
- (a) "what my WORKERS-NEXT answer asks: the deploy step small enough to move, two lines on what a move would cost,
  and the 30-day rule for old previews" (section 2.2, 2.4);
- (b) "every click I have to do myself (the Cloudflare project, the deploy token, anything on GitHub), listed once, in
  the order I do them, so that I can do them in one sitting" (section 4);
- (c) "what you need from the CI Builder chat (the edit that hands the tested build to the preview job), as a request
  it can act on" (section 5);
- (d) "the day I can expect the first preview link, said honestly" (section 9).

Binding words of his that the design must meet:
- "I do not want to click for each preview" (2026-11-19, 00:01:25 UTC).
- STEP4-COUNT (b): "only the newest push of a pull request is deployed, and it counts its deploys per month".
- His locked-install point of 2026-11-16 (PHC-LOCKFILE): a committed lockfile, install scripts off.

What a player sees: nothing. What Mustafa gets: a link on every pull request from this repository, opened in Brave,
with no login and no click before it.

## 2. The design, in plain words

### 2.1 How the token stays safe without his click (D1)
The deploy token never meets code that a pull request can change.
- **The trigger.** A new workflow, `preview.yml`, starts when the `toolchain` workflow finishes (GitHub's
  `workflow_run`). GitHub always runs a `workflow_run` workflow from main's copy of its file, never the branch's.
- **The wall.** The token sits only in a GitHub environment named `preview`, with these settings:
  - its one deployment rule is "branch `main`";
  - no required reviewer;
  - administrators may not bypass it.

  Every job a branch can add or rewrite runs on a branch ref, and GitHub refuses that ref the environment, so the job
  never gets the token. The check measured that refusal: a `main`-only rule refused a pull request's job, judging the
  ref `refs/pull/10/merge` (#66's report :113-114).
- **The job split.**
  - `check` has no secret. It confirms the run came from a pull request of this repository and that its head is the
    pull request's newest commit. It downloads the tested build and checks it (the refuse list below).
  - `deploy` holds the token and only uploads files.
  - `after` has no secret. It runs the header check and writes or updates one comment on the pull request with the
    link.
- **The untrusted build.** The files come from a pull request's build, so the deploy job treats them as data:
  - nothing in them is run;
  - Wrangler runs from a clean folder of its own (section 2.2), not from the build;
  - the build may not carry `_worker.js`, `_routes.json`, `functions/`, symbolic links or a file of 25 MiB or more,
    since any of these would make Cloudflare run code or refuse the upload;
  - lane 2's own header file replaces any `_headers` the build carries (section 2.5).
- **Production** (OQ-0006-13). A Cloudflare Pages token reaches the whole account: Cloudflare's token form offers no
  single project, and the check's token read "Entire Account" (#66 :352). So only a separate account separates
  previews from a future production (decision ACCOUNT). No production deploy exists today, and none is in this plan.
- **What GitHub does not enforce** (said plainly). A pull request that rewrites `preview.yml` is not stopped by
  GitHub. On main, GitHub enforces only a pull request with `gate` and `smoke` green (STATUS :17). Whether main also
  requires an approval was not read: Mustafa's click 9 reads it. The wall against a bad change to the workflow is
  the kit's tier-2 rule for `.github/` (the advisor agent, the security-auditor and his go in a day session), and
  the guard's ask before such an edit.

### 2.2 The deploy step: small, and movable (D2; his WORKERS-NEXT)
- **One command:** `wrangler pages deploy <dir> --project-name <project> --branch pr-<N> --commit-hash <sha>`.
- **Where Wrangler comes from:** a committed lockfile in lane 2's host folder (`hosting/`), installed with `npm ci
  --ignore-scripts`. That is his locked-install point. `cloudflare/wrangler-action` is not used: it installs Wrangler
  at run time with no lockfile.
- **One open point:** that Wrangler's upload works with install scripts off is unproven. The check installed
  `wrangler@4.156.0` with scripts on and no lockfile (#66 Appendix B, :1431, :1512). The first run proves it. If it
  fails, the fix is a recorded departure that allows the scripts of Wrangler's own install only, shown to him first.
- **The link name** `pr-<N>` can never equal the production branch.
- **What a move to Workers would cost** (his two lines):
  1. The command becomes `wrangler preview --name pr-<N>` with a `wrangler.jsonc`. The token needs Workers Scripts
     Edit, and the cleanup becomes `wrangler preview delete`.
  2. The `_headers` file stays as it is. Pages' 30-day rule becomes Workers' cap of 100 previews per Worker, and ADR
     0006 asks for the hosting check to be run again on Workers.

### 2.3 Only the newest push deploys, and the month's deploys are counted (D3; STEP4-COUNT (b))
Four layers keep each pull request to one deploy per push:
- `toolchain`'s own cancel of older runs on the same branch;
- the build is handed over from one event only (the pull-request run, section 5);
- `check` refuses a build whose commit is no longer the pull request's head;
- a per-pull-request concurrency group with cancel-in-progress, on the `deploy` job only.

The counter is GitHub's own deployment records for `preview` in the current month. The cleanup job uses the
environment without making a deployment record (`deployment: false`), so the count stays exact. The comment prints
the month's count. The estimate is about 110 to 300 deploys a month, against the 500 builds of the Free plan.

### 2.4 Cleanup, and the 30-day rule (D4; his WORKERS-NEXT)
One reconcile job:
- deletes, with force, every `pr-<N>` deployment whose pull request is no longer open, and the older deployments of
  open ones;
- runs after each deploy, when a pull request closes (a small workflow with no secret, `preview-close.yml`, whose
  finish starts it), and once a day;
- deletes nothing when the list of open pull requests fails, comes back short, or is empty while previews exist;
- refuses any deployment whose branch is not `pr-<digits>`.

The check showed why a schedule alone is not enough: GitHub fired 3 of 24 hourly runs (#66 :35).

**The 30-day rule.** Pages' limits page reads "Preview deployments older than 30 days are deleted, unless they are one
of the project's 100 most recent deployments". Which deployments count among "the 100 most recent" is not stated:
- if only those still present count, an idle pull request's link never expires;
- if every deployment ever made counts, the link stops after 30 days.

The cheapest remedy, when it happens, is a new push or a re-run of the pull request's `toolchain`, which deploys again.
The build artifact itself lasts 7 days. Nothing is built for this rule; the runbook names it.

### 2.5 Headers (D5; PHC-PREVIEW-NOINDEX)
- Lane 2's preview header file, `hosting/_headers`, comes from main's copy and replaces any `_headers` in the build.
  It carries `X-Robots-Tag: noindex` on `/*`, and `immutable` caching on the hashed asset folder. Today the build
  writes no `_headers`.
- After each deploy, the header check fetches the link with a cookie-less client and compares the headers with
  the preview set. Its bite is a planted build without the `immutable` rule. Pages adds `noindex` by itself on
  previews, so a missing `noindex` rule cannot make the check fail. A cache rule can: the check's planted `no-cache`
  rule changed what the host served (#66 :33).
- PHC-PREVIEW-NOINDEX:
  - a file test before each deploy: the preview header file carries `noindex` on `/*`, and no production file is
    deployed by this workflow;
  - its bite, in a job with no secret on `preview.yml`'s own pull requests.

### 2.6 Privacy
Previews are public but unlisted (ADR 0006 part 2): they run the match offline and carry no unreleased art. So
"Preview access" stays off, and he logs in to nothing. When unreleased art or the first multiplayer preview comes,
that same setting turns the wall on (#66 :99-103); that is decided then.

## 3. Files (owners, and Part 15's line)
New files, all lane 2's:
- `.github/workflows/preview.yml`: the `workflow_run` trigger, a daily schedule, the jobs `check`, `deploy`, `after`
  and `reconcile`, and the bite jobs on its own pull requests.
- `.github/workflows/preview-close.yml`: on a pull request's close; no secret, no checkout of its code; it only
  starts `reconcile`.
- `hosting/package.json` and `hosting/package-lock.json`: Wrangler, pinned and locked. The dependency hold applies:
  the security-auditor answers on the exact files before the checkpoint script pushes them (DEP-HOLD).
- `hosting/_headers`: the preview header set.
- `hosting/check-build.mjs`, `hosting/check-headers.mjs` and `hosting/reconcile.mjs`, with tests on fixtures.
- `docs/runbooks/previews.md`: what to do when something fails, the 30-day rule, the wall's switch, the token's end
  date.

Part 15 gave lane 2 "the workflow file it adds for previews (a new file), the host's config files it adds, its
runbook". Two workflow files and a host folder with a lockfile go beyond "a new file": a departure (section 6). The
advisor agent judges whether it is in scope.

Not touched: `gate.yml`, `toolchain.yml`, the gate's files, `crates/`, `.claude/`, any draft under `docs/adr/`.

## 4. His clicks, listed once, in the order he does them (one sitting, about 25 minutes)
The sitting comes before step 2.6's merge. A workflow that names an environment that does not exist creates it with
no rules. Inside the sitting, the branch rule comes before the token: the session runs the refusal test (pass mark M1,
section 7) in between, a wait of about 3 minutes.

If his answer to ACCOUNT is (b), these come first (three more clicks):
1. Cloudflare: Accounts, Create Account, name `Shamble previews`. Cloudflare allows up to five more free accounts
   for a user of seven days or more, and his user qualifies.
2. Switch to that account. Steps 3 to 6 happen inside it.

Cloudflare:
3. Workers & Pages, Create, Pages, "Upload assets". Project name `shamble-previews`. Drag in a folder holding one
   placeholder `index.html` ("No game here"), then Deploy. That upload is the project's production version, and no
   job ever deploys to it.
4. The project's Settings, "Preview access": leave it off.

GitHub (Shamble-Frontlines, Settings):
5. Environments, New environment `preview`, or open it if a run already made it:
   - Required reviewers off; Wait timer off.
   - "Allow administrators to bypass configured protection rules" unticked.
   - Deployment branches and tags: "Selected branches and tags", Add rule, Branch, `main`, Add.
   - Tell the session. It runs the refusal test, and replies within about 3 minutes whether both planted jobs were
     refused. If not, stop here: no token goes in.

Cloudflare again:
6. Manage Account, Account API Tokens, Create Token, Custom token, named `shamble-previews-deploy`:
   - permissions: Account, Cloudflare Pages, Edit and Read;
   - Client IP filtering: empty;
   - Token expiration: a start date of today AND an end date, each in its own field, never the single-date chip
     (PHC-TOKEN-SETUP);
   - Create, then copy the token.
7. Copy the Account ID from the account's overview page.

GitHub again:
8. Environment `preview`: add the secrets `CLOUDFLARE_API_TOKEN` and `CLOUDFLARE_ACCOUNT_ID`.
9. Secrets and variables, Actions, Repository secrets: confirm none is named `CLOUDFLARE_*`. A repository secret
   would reach every branch.
10. Actions, General: confirm the workflow permissions read "Read repository contents and packages permissions".
11. Branches (or Rules), main's rule: tell the session whether administrators may bypass it (decision MAIN-BYPASS),
    and whether an approval is required.

He sends no screenshot of the token. The first deploy job asks Cloudflare whether the token is valid, and prints
the answer and its dates in its log. After the sitting, nothing is needed per preview.

## 5. The request to the CI Builder chat (D6), as it can act on it
The request goes on the Lane requests issue (#59), with this text. CI Builder makes the edit in its own small tier-2
pull request, with its plan, the advisor agent's and the security-auditor's reviews, and his go:

> Lane 2 asks for one step at the end of job `toolchain` in `.github/workflows/toolchain.yml`, after "Build and test
> with the network cut off" (it runs outside the cut, and only when every step above passed). It hands the tested
> player build to the preview workflow. A proposal; the exact text is yours to write and his to approve:
>
> ```yaml
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
> Three notes:
> 1. When step 2.3 replaces this file, keep the workflow name `toolchain`, the file path and the artifact name
>    `preview-site`, or tell lane 2 first on #59.
> 2. Read the pin again from actions/upload-artifact itself.
> 3. If his Brave checks need the development page, the path is `.gate/dist-development/`. Only one folder, so
>    the deploys do not double.
>
> The preview workflow downloads it by the run's id, with `actions: read` in its no-secret job only. Until the
> artifact exists, the preview ends with a notice, so the two pull requests can merge in either order.

## 6. Departures, listed (no draft is edited by step 2.6; each goes to the drafts' next revision as a follow-up)
1. **B9's interim reviewer on `preview`** (ADR 0006 part 13), which his words of 2026-11-19 refuse. Replaced by the
   main-only environment rule and `workflow_run`.
2. **ADR 0002 part 10's "No `pull_request_target`, no `workflow_run`"**, whose reason is finding 10: these triggers
   "hand secrets and write tokens to runs a pull request triggers".
   - Here `workflow_run` runs only main's file.
   - Its deploy job runs nothing from the build, and the build passes the refuse list.
   - engine-architect, the draft's owner, reads this departure.
3. **The build plan's conditions 3 and 4 for step 2.6** (:1021-1027).
   - Condition 3, "the 2.3 chat's CI change-set merged": replaced by CI Builder's one-step edit (section 5), by his
     words of 2026-11-25.
   - Condition 4, "a reviewer on previews": replaced by section 2.1, and by his sitting.
4. **Where `_headers` comes from.** ADR 0002 has the build generate the header sets; here lane 2's file on main
   replaces any in the build.
5. **The weekly sweep** (ADR 0006 part 3) becomes daily, plus one run after each deploy and each close.
6. **Part 15's "a new file"**: two workflow files and a host folder (section 3).
7. **ADR 0002 part 10's single deploy job**: it becomes three jobs plus a reconcile job.

## 7. Pass marks and evidence (risk tier 2: a workflow, a secret, hosting)
Before the merge:
- **M1, the refusal test:** two planted jobs on step 2.6's own pull request name `preview`, one plainly and one with
  `deployment: false`. Both must be refused, after click 5 and before any token is in the environment. Each job prints
  only whether a secret is present.
- **M2, the scripts' tests on fixtures:**
  - the refuse list: each forbidden file;
  - the head-commit match;
  - reconcile's "delete nothing" cases;
  - the header file test.

  Each has a planted defect that turns it red.
- **M3, the lockfile:** `npm ci --ignore-scripts` from `hosting/` gives the pinned Wrangler. The security-auditor's
  answer on the exact manifest and lockfile is in before the push (DEP-HOLD).
- **M4:** the gate passes; the advisor agent, the security-auditor and test-engineer review; engine-architect reads
  departure 2.

After the merge, on the first pull request that comes after both merges (step 2.6's own pull request cannot preview
itself, because `workflow_run` uses main's file):
- **M5:** the link opens, with its build marker, and the token check prints a valid status.
- **M6:** a second push replaces the first: one deploy per push, the older one deleted.
- **M7:** the header check passes, and its bite fails.
- **M8:** closing a pull request removes its link within 24 hours (the check's mark, now in daily use).
- **M9:** the month's count appears in the comment.
- **M10:** no secret value appears in any log.
- **Brave checklist for him,** on that first link: open the link from the comment, see the page load with no login,
  read the build marker, then open it again in a private window.

## 8. Reviewers
- The advisor agent, on this plan and on every pushed head.
- The security-auditor, on this plan (the token design and the untrusted build) and on the final diff, as tier 2
  asks.
- test-engineer: are the marks provable, and do the bites bite?
- engine-architect: reads departure 2 against ADR 0002.
- infra-engineer wrote the design, so it does not review it.
- At most 3 agents at a time, on Opus; effort `xhigh` for the security and design reads.

## 9. The day of the first link (D8), said honestly
- **The conditions:**
  - his go on this plan;
  - the build and one review round, with a fix pass;
  - his sitting (section 4);
  - his go to merge step 2.6;
  - the CI Builder chat's edit merged (its own pull request, in parallel);
  - then the next pull request's push.
- **The link comes about 20 minutes after that push:** `toolchain` takes up to about 15 minutes, then the deploy runs.
- **Best case: 3 working days after his go.** With one fix cycle after the first run, which is likely given that
  nine things are proven only then: 4 to 6. If his go comes on Thursday 26 November, the best day is Tuesday
  1 December; the honest day is Wednesday 2 to Friday 4 December. If a review or the CI Builder's edit slips, it is
  the week of 7 December.

## 10. Order of work (after his go), each with a checkpoint save
- **U1. The host folder.** Manifest and lockfile; the security-auditor answers on them first (DEP-HOLD). The header
  file, the scripts and their tests.
- **U2. The two workflow files.** Their exact texts are shown to him by sha256 for his yes before they run, as in the
  hosting check (his yes of 2026-11-15 to Part 7's texts by their sha256). The runbook.
- **U3. Records.** His messages of 2026-11-25:
  - 17:05 Amsterdam: transcript 16:41:51 UTC, 1094 bytes, sha256 c41b21e8...;
  - 18:15 Amsterdam: transcript 17:11:56 UTC, 1778 bytes, sha256 e2554c4e...;
  - both into DECISIONS by script;
  - and the dated pointer in #67's report (his item 4; the advisor agent's Low on 35adfbe).
- **U4.** The request to CI Builder on #59.
- **U5.** Review round 1 and its fix pass. The report. A draft pull request, then ready.
- **U6.** His sitting (section 4), with M1 in it; then his go to merge.
- **U7.** The first link (M5 to M10), and his Brave checklist.

## 11. Risks
- **R1.** The wall is the kit's, not GitHub's, for changes to `preview.yml` itself (section 2.1). A token taken
  through a bad merge or a compromised dependency reaches only the previews' account (decision ACCOUNT).
- **R2.** `deployment: false` may not hold the branch rule the way a normal job does: the docs imply it and do not
  say it. M1 tests exactly that before any token goes in.
- **R3.** Wrangler with install scripts off is unproven (section 2.2).
- **R4.** `workflow_run`'s payload and the pull-request lookup are proven only on the first real run. The design
  checks the head commit itself rather than trusting the payload.
- **R5.** The 30-day rule's reading is unknown (section 2.4).
- **R6.** GitHub blocks `pull_request_target` by default in public repositories from 2026-12-02. This design does
  not use it.

## 12. Decisions needed (for him; the advisor agent settles what his words already cover)
1. **TRIGGER: how a preview gets the token without his click.**
   - (a) `workflow_run` with the main-only environment (section 2.1). It meets both his asks, and it departs from ADR
     0002 part 10 and from B9's reviewer (section 6).
   - (b) The ordinary pull-request run with no reviewer, previews in their own account. Any branch could take the
     token; only the account bounds the damage.
   - (c) Keep the reviewer. That is his click per preview, which he refused.
   - Recommendation: (a).
2. **ACCOUNT: where the previews live.**
   - (a) His current Cloudflare account, moving at the first release.
   - (b) A new free account for previews only, from the start: three more clicks in the same sitting (section 4,
     clicks 1 and 2, and the switch). A leaked token then reaches previews only, never his domain or a future
     production.
   - Recommendation: (b).
