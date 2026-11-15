# Plan: the preview hosting check (SPIKE_PLAN entry 6), revision 1

_For Mustafa and the Advisor. A plan only: nothing is built, no secret is made and nothing outside this checkpoint is
edited before Mustafa's go. Branch `claude/optimistic-albattani-mdoonz` (stands in for `cs/preview-hosting-check`),
base main 4460ef5. Line numbers are main's at 4460ef5. Written 2026-11-15 by the second builder chat (main model Opus
5.5; the advisor tool answered by Fable 5.2)._

## In one screen
- **What it is:** the small, throwaway test of Cloudflare Pages, the host ADR 0006 chose for the game and for every
  pull request's link. It must pass before step 2.6 gives every pull request a link. It is judged by 11 marks,
  PM-0006-1 to -10 and -32 (Part 1).
- **What it shows:**
  - the host sends the right headers on every file, so the WebAssembly loads, previews stay out of search engines, and
    the "isolated" profiling preview really is isolated, in Chrome, Edge, Firefox, Safari and Brave;
  - the content file is cached for good, and no file is over the 25 MiB limit;
  - **PREV-DELETE**, the open risk: whether a closed pull request's build is really gone from every link within 24
    hours, although Cloudflare says a branch's latest upload "cannot be deleted";
  - what the login wall covers;
  - that a pull request cannot get the deploy token without your click, nor deploy as production.

  It also records whether uploads count against the free plan's 500 builds a month, whether GitHub shows the link on
  the pull request page, and your verdict in Brave.
- **Where:**
  - a new public test repository of yours (`shamble-preview-check`) and a new test project in your Cloudflare account;
  - nothing in this repository's CI, nothing in `spikes/`;
  - shamblefrontlines.com is left alone, since the netcode proof test uses it.
- **What I build:** one test page with a worker, a tiny WebAssembly file and a stand-in content file; a build script
  that writes the header file in 7 modes; check scripts for headers, sizes, deletion, the login wall and four browsers;
  four throwaway workflows. About 1,110 lines and 14 hours of session work, in six units with planned sizes (Part 2).
- **What you do:**
  - about 40 minutes of setup: a repository, a Cloudflare project, one token with a 10-day end date, two GitHub
    environments (Part 3 A);
  - then four short sittings over two days, plus the setup pull request: about 24 approvals, three merges, the Brave
    checklist, switching the wall on, and revoking the token at the end.
- **Security in one line:** the token lives only in the test repository's `preview` environment, behind your approval.
  It can do anything Pages can do in your account. As far as Cloudflare's permission list shows, it can do nothing
  outside Pages; two questions about its reach stay open (Part 4). Because sessions act as your GitHub login, the
  approval holds against a session only as far as the sessions' tools and credential cannot approve. This session's
  tools cannot; the credential is your read, SR-READ (Part 4).
- **Calendar:** each close is watched for 24 hours. The earliest end is the day after the runs, so about two days after
  your go: for example, go on Monday the 16th, end on Wednesday the 18th (Part 5).
- **Laptop and Mac:** 7 marks name the laptop and 2 the Mac, but none is a timing. I recommend GitHub's own machines
  stand in, so that nothing waits (Part 6, decision MACHINES).
- **Part 7** holds the exact text of every workflow, for your yes.
- **Decisions:** seven, at the end. My recommendation is (a) in each.

## Part 0. How to read this plan
- **The test repository (TR):** the new public GitHub repository that holds the test site, the throwaway workflows,
  the deploy token and the test pull requests: ADR 0006's "TR" (:1280-1281, "Mustafa names it and adds its `preview`
  environment and token"). Proposed name: `Mergincanli/shamble-preview-check` (decision TR-WHERE).
- **The test project (CF):** a new Cloudflare Pages project in Mustafa's account, proposed name
  `shamble-preview-check`, address `https://shamble-preview-check.pages.dev` (Cloudflare may add a suffix if the name
  is taken; Part 3 says what to do then).
- **A deploy:** one upload of the built test site to the test project, on a named "branch" of the project. The
  project's production branch is production: whatever name Cloudflare gives it, which the workflows read from
  Cloudflare rather than assume. Every other branch is a preview with a stable link
  `https://<branch>.shamble-preview-check.pages.dev`, and every single upload also gets its own one-off link
  `https://<8 characters>.shamble-preview-check.pages.dev`.
- **The three links** (the hosting checklist's): production, the preview built with the isolation flag off (Pages
  branch `iso-off`), the preview built with it on (Pages branch `iso-on`).
- **An approval:** Mustafa's click on GitHub's "Review deployments", then "Approve and deploy", for a run that waits for
  the `preview` environment (decision REVIEWER).
- **A probe:** a script that fetches a link the way a stranger would (no login, no cookies) and records what came back.
- **ML, MAC, MP:** the minimum laptop, the Mac with Safari, Mustafa's PC in Brave (ADR 0006's key, :1306-1310).
- **The draft:** ADR 0006 (Proposed) unless another is named. "On the infra draft as proposed" is Mustafa's answer to
  PREVIEW-START: given on 2026-11-14 (his note 1: every "today" in the task means 2026-11-14) and sent with the task on
  2026-11-15. The check runs on the draft as it stands, its interim positions included.

## Part 1. What the check proves, and the pass marks it must meet (your item 1)

**In one line:** before every pull request gets its own web link (step 2.6), this small test shows that Cloudflare
Pages serves the game's files the way the drafts need, that a closed pull request's link really disappears, and that a
pull request cannot get at the deploy token or at production. It proves ADR 0006 parts 1, 2 and 3 and part 13's item
B9 (spike 1, :1198-1222; SPIKE_PLAN :416-438).

| Mark | In plain words | It passes when |
|---|---|---|
| PM-0006-1 | The setup is written down before the first run | the results log opens with: the test project's name and production link, the test repository, the commit of the workflows, the build-marker rule, the machines' systems, every browser's and WebdriverIO's version; checked again before the 24-hour reading |
| PM-0006-2 | The host sends each set of headers exactly as built, on both kinds of link (each branch's stable link and each upload's one-off link) | on 100 % of responses: every `.wasm` file as `application/wasm`; previews say `X-Robots-Tag: noindex` (keep out of search engines) and production does not; the preview built with the isolation flag on sends COOP `same-origin` and COEP `require-corp` on every page, script, worker and `.wasm` response, while production and the flag-off preview send neither. Two planted mistakes must be caught: COEP missing on the worker of the isolated build, and `noindex` added to production. It also answers R5-PREV-OQ6: does a header rule written for `https://:version.:project.pages.dev/*` reach a branch's stable link? |
| PM-0006-3 | The isolated preview really is isolated in every browser, and the others are not | the page and its worker report `crossOriginIsolated` true on the flag-on preview and false on the flag-off preview and on production, 3 loads of 3 per browser and link, in Chrome, Edge, Firefox and Safari by script, and in Brave by you |
| PM-0006-4 | The game's big content file is cached for good | it is sent with `immutable`, and a second visit in the same browser fetches 0 bytes of it, 3 times, each from a fresh profile, in Chrome; a planted `no-cache` must make the check fail |
| PM-0006-5 | No file is too big for the host | the largest file of every upload stays under 25 MiB (26,214,400 bytes); a planted 26 MiB file is flagged, and what happens to it (refused, or served) is written down |
| PM-0006-6 | **PREV-DELETE:** a closed pull request's build is gone from every link within 24 hours | 24 hours after each test pull request closes, none of its links (the stable branch link and the one-off link of every upload it ever had) answers with that build. "Gone" means a 404, any answer without that build's marker, or the login wall. It runs with the login wall off. Four cases: (a) closed without merging, branch kept; (b) three uploads, merged, branch deleted; (c) the control, still open, which must keep serving; (d) the bite, closed with the cleanup switched off, which must still serve. If (a) or (b) fails, the first workaround (a "tombstone": a page without the marker becomes the branch's last upload, and the other uploads are deleted) is read the same way. Three outcomes: all gone (pass); gone only with the workaround (pass, written down); still serving with no workaround (fail: the draft goes back to its author, and Workers static assets is weighed again) |
| PM-0006-7 | Do direct uploads count against the Free plan's 500 builds a month? | reported, no pass number: your dashboard's count before and after 10 uploads |
| PM-0006-8 | The login wall covers every preview and not production | with the wall switched on, a stranger gets no build from any link of at least 2 open preview branches, nor from PREV-DELETE's leftover links, while production still serves; read 10 minutes and 1 hour after you switch it on (the 1-hour reading counts); control: the same sweep with the wall off shows the builds |
| PM-0006-9 | Your verdict in Brave | every step of the hosting checklist (8 steps, ADR 0006 :1436-1457) "achieved" |
| PM-0006-10 | Does GitHub show the preview link on the pull request's page? | recorded, yes or no, with a picture of the page (this answers R5-PREV-OQ5) |
| PM-0006-32 | A pull request cannot reach a deploy secret or production | 2 of 2: a test pull request whose deploy job is rewritten to print a marker gets no secret until you approve it (the job prints only "present" or "absent", never the value); a pull-request job that names the `production` environment is refused by its `main`-only rule. Reported: whether the preview token can deploy production (the draft expects yes: OQ-0006-13), and how the `main`-only rule treats a pull request's merge ref. Bite: with the reviewer removed, the rewritten job must print "present". Your revocation of the token at the end is recorded |

**What it decides or feeds** (SPIKE_PLAN :436-438): PREV-DELETE (R5-PREV-OQ2); R5-PREV-OQ5; R5-PREV-OQ6; your
OQ-0006-4 (what the wall should cover, :876-887); and two inputs for OQ-0006-13 (:988-1006), which the Advisor decides
with security-auditor before the first client-host token enters this repository: what the reviewer costs you per pull
request, and whether a session can approve a deploy.

**What it does not prove** (the draft's own list, :1417-1434): the weekly sweep and the 7-day artifact rule (step
2.6's jobs), the staging game server (slice 3), the production certificate, the content security policy (the security
baseline). Also not in it: a custom domain (none is used: the shamblefrontlines.com zone stays untouched, since the
netcode proof test sets its records there).

### 1.1 Mark by mark: as the draft has it, as planned, and every departure
Kept unchanged unless the row says otherwise: every number, count, timing, case, branch name and bite of the draft.

| Mark | Machines: the draft → planned | Method, counts, bites, controls: the draft → planned | Departure, and who decides |
|---|---|---|---|
| PM-0006-1 | CF, TR, ML, MAC → CF, TR, GitHub's runners | the results-log header, as drafted, with each runner's image name and version where the draft records ML's and MAC's systems; checked again before the 24 h reading, as drafted | the machines: decision MACHINES |
| PM-0006-2 | the script runs on ML ("a header check, not a hardware result", :1367) → on GitHub's Linux runner, as the deploy workflow's last job, right after each deploy | as drafted: one test site deployed three ways (production; a preview with the flag off; a preview with it on), each read on its stable branch link and on its one-off link; the page lists every resource it and its worker loaded (Resource Timing in both); a cookie-less fetch compares every response with expected sets written by hand into the script from GLOSSARY §5; the bites as drafted (COEP removed from the worker's rule in the flag-on build; a `noindex` rule added to production). Each link form is recorded on its own. **Production's one-off link:** if the host sends `noindex` there (it adds `noindex` "if requesting the asset over a preview URL", and a one-off link may count as one), the row as written is not met. That is reported and goes to the draft's author (:1409-1414), never read as a pass, and it holds the check's end, and with it step 2.6, until the draft is revised (:1401: every mark that applies must pass). Added: the script also fetches every file in the build's file list, so no file escapes the check; and a test-only header, `X-Check-Rule`, on the two host rules answers R5-PREV-OQ6 (the script tolerates that extra header) | the machine: MACHINES; the advisor agent ruled the `X-Check-Rule` header an addition that changes no mark (its review of revision 1) |
| PM-0006-3 | ML: Chrome, Edge, Firefox; MAC: Safari; MP: Brave → GitHub's Windows runner (Chrome, Edge, Firefox); GitHub's macOS 26 runner (Safari 27.2 on the image read 2026-11-15; the version is recorded on the day); you in Brave | as drafted: WebdriverIO opens each link and reads what the page and its worker print, 3 loads per browser and link; the bite as drafted (PM-0006-2's missing COEP must give false): the browser run also opens the missing-COEP preview (Pages branch `bite-coep`) | the machines: MACHINES |
| PM-0006-4 | ML: Chrome → GitHub's Windows runner: Chrome | as drafted: first visit, tab closed, second visit; the bundle's `transferSize` read on the second; 3 repeats, each from a fresh profile; the bite as drafted (`no-cache`), on a preview branch of its own | the machine: MACHINES |
| PM-0006-5 | CF; ML → CF; GitHub's Linux runner (in the build job, before each deploy) | as drafted: every upload's largest file; the bite as drafted (a planted 26 MiB file, flagged, and what Wrangler or the host does with it written down) | the machine: MACHINES |
| PM-0006-6 | CF, TR; the probe from ML → CF, TR; the probe on GitHub's Linux runner | as drafted: cases (a) `cs/pd-one`, (b) `cs/pd-three`, (c) the open control, (d) the bite with the close workflow switched off; the throwaway close workflow deletes every deployment of the branch with the force option; a cookie-less probe every hour from the close to 24 h after; the access policy off; the tombstone if (a) or (b) fails. Changed: (1) the hourly probe runs on GitHub's schedule, and the session also starts one probe itself just after the 24 h mark, so the reading never rests on the schedule alone (GitHub may delay or skip a scheduled run); (2) under TOMBSTONE (a), the tombstone runs from the start on two more cases, (a') `cs/pdt-one` and (b') `cs/pdt-three`, built like (a) and (b), and counts only if (a) or (b) fails | the machine: MACHINES; the tombstone's timing: decision TOMBSTONE; the session's own 24 h run: ruled an addition that changes no mark (the advisor agent) |
| PM-0006-7 | CF; MP → as drafted | as drafted: your two readings around 10 uploads by the throwaway deploy workflow (one run that uploads 10 times) | none |
| PM-0006-8 | CF; the probe from ML → CF; the probe on GitHub's Linux runner | as drafted: you switch the access policy on; sweeps 10 minutes and 1 hour later, the 1-hour sweep counts; the control as drafted. No custom domain (CF has none) | the machine: MACHINES |
| PM-0006-9 | MP, Brave on Windows → as drafted | as drafted: the hosting checklist's 8 steps; steps 5 to 8 only after PM-0006-6's 24 h reading | none |
| PM-0006-10 | TR → as drafted | as drafted: the deploy job sets its environment's `url` to the preview's stable link, and the pull request's page is read after a deploy. The picture: the session takes it from the public page while logged out, and you say whether your own logged-in page shows the link too | the picture's route: ruled an addition that changes no mark (the advisor agent) |
| PM-0006-32 | TR, CF; MP → as drafted | as drafted: the rewritten deploy job; the job that names `production`; the reported production deploy with the preview token; each run's log and the environment's deployment history pasted into the results log; the control and the bite as drafted. Added: ADR 0002's PM-0002-66 bite (the `production` rule removed: the job must then print "present"), since PM-0002-66 names the test repository for these tests (0002:1332) and the draft calls them "the same test" (:1222). Under TOKENS (a) the reported production deploy with the preview token is the test project's ordinary production deploy. A job refused by the branch rule most likely runs no step at all (rules "must pass before a job ... is sent to a runner"), so its result and error note are recorded rather than a printed "absent". The report claims only this bite of PM-0002-66, never that mark as a whole | the extra bite: ruled an addition that changes no mark (the advisor agent); one token: decision TOKENS |

## Part 2. What I build (your item 2)
Everything is built in the test repository (TR). In this repository the check adds only its checkpoint, its results
log and, at the end, the report. Nothing goes into `spikes/`, and nothing touches this repository's CI.

**The test site** (`site/`, plus the files the build writes):
- `index.html`: one page, with no inline script or style (part 13's B3: the build stays ready for a strict content
  security policy). It holds `<meta name="build-marker" content="<commit>">` and shows the same commit. **The
  build-marker rule:** every build writes the commit it was built from into its HTML; the tombstone has no marker.
- `page.js`: fetches the content file, starts the module worker, and shows the build marker, `crossOriginIsolated` in
  the page and in the worker, whether the WebAssembly ran, and every resource the page and the worker loaded (Resource
  Timing in both).
- `worker.js`: a module worker. It loads `tiny.wasm` with `WebAssembly.instantiateStreaming`, which refuses a `.wasm`
  not served as `application/wasm` (R5-PREV-F20), calls it, and reports its own `crossOriginIsolated` and resources.
- `tiny.wasm`: a 41-byte module that adds two numbers. The build writes it from its bytes, so no binary is committed.
- `content.<hash>.bin`: the content-bundle stand-in, 64 KiB of fixed bytes named by their own SHA-256, as a real
  hashed bundle is.
- `404.html`: so that an unknown path answers 404. Without a top-level `404.html`, Pages serves the home page for every
  unknown path (serving-pages page, last updated 2026-11-11).
- `manifest.json`: every file of the build, with its size and hash; the check scripts read it.
- `_headers`: written by the build for its mode (table below).

**The build script** (`tools/build.mjs`, Node 22, no dependencies) copies `site/` and writes the marker, the `.wasm`,
the hashed content file, the manifest and `_headers`:

| Mode | The `_headers` it writes | Used for |
|---|---|---|
| production | the content file: `Cache-Control: public, max-age=31536000, immutable`; two test-only host rules for R5-PREV-OQ6: `X-Check-Rule: project-host` on `https://:project.pages.dev/*`, `X-Check-Rule: version-host` on `https://:version.:project.pages.dev/*` | production |
| preview | the same as production: the host itself adds `X-Robots-Tag: noindex` on previews ("By default, every preview deployment generated by Cloudflare Pages includes the X-Robots-Tag: noindex HTTP response header": preview-deployments page, 2026-10-12). If a link form misses it, the check shows it, and step 2.6 writes the rule itself | the flag-off preview; every test pull request |
| preview-isolated | preview, plus COOP `same-origin` and COEP `require-corp` on every path | the flag-on preview |
| bite-coep | preview-isolated, plus `! Cross-Origin-Embedder-Policy` on `/worker.js` (a detach line removes what earlier rules added: headers page, 2026-11-12) | the bite of PM-0006-2 and -3 |
| bite-noindex | production, plus `X-Robots-Tag: noindex` on every path | PM-0006-2's bite on production |
| bite-nocache | preview, with `Cache-Control: no-cache` on the content file | PM-0006-4's bite |
| bite-oversize | preview, plus one extra file of 26 MiB (27,262,976 bytes) | PM-0006-5's bite |

No mode sets `Content-Type`: Wrangler uploads `.wasm` as `application/wasm` (its MIME library, read in its source on
2026-11-15), and the check reads what the host really sends. Two rules that set the same header in one `_headers`
file are joined with a comma ("produces conflicting directives" for `Cache-Control`: headers page, 2026-11-12), so
every mode sets each header once.

**The check scripts** (`tools/`, Node 22; WebdriverIO is the one dependency, locked):
- `check-sizes.mjs` (PM-0006-5): lists every built file and its size, names the largest, and flags any file of
  26,214,400 bytes or more. It never stops the build, so the planted big file still reaches Wrangler, whose answer
  is recorded (Wrangler refuses it before upload: "Pages only supports files up to 25 MiB in size", its source).
- `check-headers.mjs` (PM-0006-2): for every upload of the run, it fetches every file in the manifest (and `/`) on
  both links: the branch link and the one-off link, or for production the project's own address and the one-off
  link. It sends no cookies and names itself in the User-Agent, since Pages answers default script User-Agents with
  a 403, error 1010 (known-issues page, 2026-11-06). It compares the headers with the expected sets, written into the
  script by hand from GLOSSARY §5, and records `X-Check-Rule`. A 403 with error 1010 counts as "probe blocked", never
  as a pass.
- `probe.mjs` (PM-0006-6 and -8): for every upload ever recorded, it fetches its links the same way and sorts each
  answer into one of four:
  - serves the build: 200 with that build's marker;
  - gone: a 404, or an answer without the marker;
  - the login wall: a redirect to Cloudflare Access;
  - blocked.

  It writes the hours since each pull request closed.
- `sets.mjs` and `sets.json`: the deploys a run makes:
  - for a pull request: one preview of its branch;
  - started by hand: one named set: `round-a` (production, `iso-off`, `iso-on`), `bites`, `bite-production`,
    `restore-production`, `uploads` (10 uploads to branch `uploads`) or `revoked`.
- `wdio/run.mjs` and its locked `package.json`:
  - PM-0006-3: opens each link 3 times in each browser and reads what the page and the worker print;
  - PM-0006-4, in Chrome: first visit, tab closed, second visit, and the content file's `transferSize`;
  - records every browser's version, WebdriverIO's version and the runner image.
- `links.json`: the test project's address, as Cloudflare gives it (Part 3, A4).

**The workflows** (exact text in Part 7):
- `deploy.yml`: pull requests and the sets started by hand; build, then deploy, then verify.
- `close.yml`: the cleanup when a pull request closes, with the tombstone for `cs/pdt-*` branches.
- `probe.yml`: hourly and by hand.
- `browsers.yml`: by hand.

**The test pull requests.** Each adds one small text file of its own, so none conflicts with another (a pull request
with a merge conflict runs no workflows: GitHub's events page).

| Case | Branch | What it does |
|---|---|---|
| (a) | `cs/pd-one` | |
| (b) | `cs/pd-three` | three pushes |
| (c) | `cs/pd-open` | the control |
| (d) | `cs/pd-off` | also deletes `close.yml` on its branch: the cleanup switched off by the pull request itself |
| (a') | `cs/pdt-one` | built like (a) |
| (b') | `cs/pdt-three` | built like (b) |
| token test | `cs/b9-edit` | Part 7's first variant |
| token test | `cs/b9-prod` | Part 7's second variant |

**The results log.** It lives in this repository at `.checkpoints/preview-hosting-check/results-log.md` while the check
runs, and is folded into the report. It holds:
- the setup header (PM-0006-1);
- every run's id, time, link and outcome;
- every reading, every approval with its time, the close times and the cleanup times;
- every refusal by the guard or the deny list, with the route taken.

**Units and their planned sizes** (the twice-the-plan rule applies to each):

| Unit | What | Planned size |
|---|---|---|
| U1 | the test site and the build script | about 190 hand-written lines; 1.5 h |
| U2 | the check scripts and the sets | about 520 lines (WebdriverIO's lockfile is generated, not counted); 3.5 h |
| U3 | the four workflows and the two token-test variants, as you approve them in Part 7 | about 400 lines; 0.5 h to place |
| U4 | the review round on the built test repository, its fixes, the advisor agent's review | 2 h |
| U5 | the runs, the readings and the results log, over 2 to 3 days | about 250 lines of log; 3.5 h |
| U6 | the report and the closing docs change-set in this repository | 3 h |
| All | | about 1,110 lines and 14 hours of session work |

That is more than the draft's "S of session work" (SPIKE_PLAN :434, "about a day"). The draft's size predates the
token tests, the tombstone cases, the browser runs on GitHub's machines and the report.

## Part 3. What you do yourself, step by step (your item 3)
Three rules for every step:
- Each step names the page it was read from and that page's date, all read on 2026-11-15; a step no page describes
  says so. **If your screen differs from a step, stop and tell me what you see.**
- **The token goes from Cloudflare's screen straight into GitHub's secret box**: never into this chat, a file, a note
  or a message.
- I take no action toward the test repository until you tell me it exists (A3).

### A. Setup, once, before the build (about 40 minutes)
**A1. Create the test repository (GitHub).** (GitHub's new-repository form; not read today.)
1. On github.com, top right, "+", then "New repository".
2. Owner: Mergincanli. Name: `shamble-preview-check`. **Public**: on GitHub Free, required reviewers and branch rules
   for environments work only on public repositories (deployments-and-environments page).
3. Tick "Add a README file"; no .gitignore, no licence; "Create repository".

**A2. Its Actions settings** (GitHub's "Managing GitHub Actions settings for a repository"). In the new repository:
Settings, then in the left bar Actions, then General.
1. Under "Actions permissions": tick "Require actions to be pinned to a full-length commit SHA". Every action in the
   workflows is pinned that way.
2. Under "Approval for running fork pull request workflows from contributors": choose "Require approval for all
   external contributors".
3. Under "Workflow permissions":
   - leave the read-only setting (the default for a new repository: GITHUB_TOKEN "only has read access for the
     `contents` and `packages` scopes");
   - leave "Allow GitHub Actions to create and approve pull requests" unticked (the default).

   Save.
4. Settings, then General, then "Pull Requests": leave "Automatically delete head branches" unticked, since case (a)
   must keep its branch.

**A3. Let this session reach it** (this session's own documentation: once the Claude GitHub App can reach a
repository, the session attaches it; the GitHub menu path is not read today):
1. Open https://claude.ai/connect-github, or on github.com: your picture, then Settings, then Applications, then
   "Installed GitHub Apps", then Claude, then "Configure".
2. Under "Repository access":
   - if it says "All repositories", there is nothing to do;
   - if it says "Only select repositories", add `shamble-preview-check` and save.
3. On the same page GitHub lists what the app may do. Tell me only whether "Administration" or "Deployments" appear
   with write access (ask SR-READ, Part 4).
4. Tell me "repository ready". I attach it with push access and check that I can see it, nothing more.

**A4. Create the test project (Cloudflare)** (direct-upload page, last updated 2026-04-21):
1. On your PC, make a folder `preview-check-start` holding one file, `index.html`, with the word placeholder in it. In
   Notepad: type `placeholder`, then File, then Save as, file name `index.html`, "Save as type: All files".
2. On dash.cloudflare.com: Workers & Pages, then "Create application", then "Get started", then "Drag and drop your
   files". If you are offered Workers or Pages, choose Pages.
3. Project name `shamble-preview-check`; drag the folder in; "Deploy site".
4. Tell me the address it shows: `https://shamble-preview-check.pages.dev`, or the name plus a few characters if
   it was taken (the page says so for its other creation path).

Why you, and why first: when Wrangler sees an AI agent creating a new static Pages project, it creates a Workers
project instead, but it never redirects a project that already exists (Wrangler's source, read today). Your upload
becomes production's first version until the first deploy replaces it. No page says which branch name Cloudflare
treats as production for a project made this way, so the workflows ask Cloudflare for it instead of assuming `main`.

**A5. Make the token (Cloudflare)** (account-owned-tokens page, 2026-10-23: "ideal for scenarios like CI/CD";
create-token page, 2026-11-06; Pages CI page, 2026-09-18; restrict-tokens page, 2026-04-20). The pages name the menu
slightly differently, so follow the screen:
1. On dash.cloudflare.com: Manage account, then "Account API tokens" (some pages say "API Tokens" there), then
   "Create Token", then the custom token ("Get started").
2. Token name: `shamble-preview-check-deploy`.
3. Permissions: **Account, Cloudflare Pages, Edit**. Nothing else.
4. If the form asks which account: yours only.
5. Token expiration: start today, end 10 days from today (dates count from 00:00 UTC). Client IP Address Filtering:
   leave empty, since GitHub's machines have no fixed addresses.
6. "Continue to summary". Check that it lists only Cloudflare Pages, Edit, then "Create Token".
7. Cloudflare shows the token once. Leave that page open for A6, and copy it nowhere else.

**A6. The `preview` environment (GitHub)** (manage-environments page; deployments-and-environments page;
review-deployments page). In the test repository:
1. Settings, then Environments, then "New environment". Name `preview`, then "Configure environment".
2. Tick "Required reviewers" and add yourself (Mergincanli).
3. Leave **"Prevent self-review" unticked.** Every run this session starts is recorded as started by your login (three
   runs read today), and with the box ticked "you will not be able to approve a deployment from a workflow run you
   initiated", so nobody could approve them.
4. **Untick "Allow administrators to bypass configured protection rules".** It is on by default, and it would give
   your login a one-click way around the reviewer.
5. "Deployment branches and tags": leave "No restriction". Then "Save protection rules".
6. Under "Environment secrets", "Add environment secret": name `CLOUDFLARE_API_TOKEN`; for the value, press the copy
   button on Cloudflare's token page and paste here; then "Add secret". Close Cloudflare's token page.
7. "Add environment secret" again: name `CLOUDFLARE_ACCOUNT_ID`; the value is on Cloudflare's Workers & Pages
   overview, under "Account Details": the copy button next to "Account ID" (find-account-and-zone-ids page,
   2026-08-03). The ID is not secret; it is stored there to keep it out of the public logs.

**A7. The `production` environment** (the same pages):
1. Environments, then "New environment". Name `production`, then "Configure environment". No reviewer.
2. Untick "Allow administrators to bypass configured protection rules".
3. "Deployment branches and tags": choose **"Selected branches and tags"**. Not "Protected branches only": in a
   repository with no branch protection, that option lets every branch deploy. Then "Add deployment branch or tag
   rule": Ref type Branch, name pattern `main`, then "Add rule".
4. "Add environment secret": name `CANARY`, value `canary-not-a-secret`. The token tests only need to see whether a job
   receives a secret, and this one is harmless.
5. Tell me "setup done". Send neither the token nor the account ID.

### B. After the build, before any run
I build on a branch `setup` of the test repository and push it **without opening a pull request**. No workflow listens
to a plain push (deploy and close listen to pull requests, probe to the schedule on `main`, browsers to a start by hand),
so nothing runs while the reviews read the branch. A pull request would run the workflows of its own merge commit at
once. If `preview` did not exist yet, GitHub would even create it unprotected ("Running a workflow that references an
environment that does not exist will create an environment with the referenced name": manage-environments page).
1. If a review changes a workflow's text, I show you the exact change for a new yes (Part 7).
2. **Only after your "setup done" (A7) and your yes,** I open the setup pull request ("Setup: the test site and the
   throwaway workflows"). Its first run waits on `preview`: **approve it.** It is the first deploy from end to end, on
   the branch `setup`: it shows that the token, the upload, the verify job and the link on the pull request page
   work before anything is measured.
3. Look over its files, then "Merge pull request", then "Confirm merge". I cannot merge there: this repository's guard
   refuses a session's merge without green `gate` and `smoke` checks, which the test repository does not have, and the
   deny list refuses every push to a `main`.
4. The merge starts the cleanup run for `setup`: **approve it.** It is a free rehearsal of what cases (b) and (b')
   rely on. I record whether a merged pull request's cleanup run starts at all, and which branch it names: a security
   note says a merged pull request's event may carry less data, and if so the case's cleanup would be skipped silently.

### C. Day 1: the runs (three sittings, about 1 h 20 min)
**Approving** (review-deployments page): the test repository, then Actions, then the run marked "Waiting", then "Review
deployments"; tick `preview`, then "Approve and deploy" (or "Reject"). Before each sitting I tell you exactly which runs
will wait. Reject anything else that waits, and tell me.

**Sitting 1 (about 35 minutes)**
1. Approve the run `round-a`. I then send you the three links.
2. In Brave, do the hosting checklist steps 1 to 3 (ADR 0006 :1444-1447): "achieved" or "not yet", with a note.
3. The build count (checklist step 4): in Cloudflare, look on the test project's page and on the Workers & Pages
   overview for a count of Pages builds used this month. Write down what you see, or "no count shown": no page says
   where such a count is (infra-engineer's read). Then:
   - tell me; I start the run `uploads`, and you approve it;
   - when I say it is done, read the count again.
4. Approve, in turn, the runs `bites`, `bite-production` and `restore-production`. Production carries the planted
   mistake for a few minutes between the last two.

**Sitting 2 (about 30 minutes)**
1. I open the six test pull requests and push their commits. Approve each waiting deploy run: 10 in all.
2. The closes: I close (a), (a') and (d) with my GitHub tool. You merge (b) and (b'): on each, "Merge pull request",
   "Confirm merge", then "Delete branch".
3. Approve the four cleanup runs (`close`) right away. **The 24-hour clocks start at the closes.** I record each close
   time and each cleanup run's start and end, and set myself a reminder for 24 hours later.

**Sitting 3 (about 15 minutes): the token tests (PM-0006-32)**
1. The control: I push `cs/b9-edit` (the deploy job rewritten to print a marker). See that its run says "Waiting",
   then approve it. It must print "secret: present".
2. The bite on the reviewer:
   - switch the reviewer off: Settings, Environments, `preview`, untick "Required reviewers", "Save protection rules";
   - tell me; I re-run the job, which must print "secret: present" without asking you;
   - then tick "Required reviewers" again, add yourself, and save.
3. The refusal: I push `cs/b9-prod` (a pull-request job that names `production`). It must be refused at once; nothing
   waits for you.
4. The bite on the branch rule (PM-0002-66's):
   - Settings, Environments, `production`, "Deployment branches and tags", "No restriction", save;
   - tell me; I re-run the job, which must print "secret: present";
   - then set it back to "Selected branches and tags" with the `main` rule.

### D. Day 2: 24 hours after the last close (one sitting, about 1 h 30 min with waits)
1. I read PREV-DELETE (the hourly probes, plus one probe I start myself just after the 24-hour mark) and tell you the
   result.
2. **The login wall** (preview-deployments page, 2026-10-12): Cloudflare, Workers & Pages, the test project, Settings,
   General, "Enable access policy". If Cloudflare asks you to choose a Zero Trust plan and enter payment details,
   follow your answer to ACCESS-CARD. Tell me the time you switched it on.
3. While I sweep, 10 minutes and 1 hour later, do the hosting checklist steps 5 to 8 in Brave (ADR 0006 :1450-1457):
   - quit Brave fully and start it again; open the flag-off preview link, where a Cloudflare login should appear;
   - log in with your Cloudflare account, which is the default login of a new Zero Trust organization; a login lasts
     24 hours by default (one-time-pin page, 2026-08-31; session-management page, 2026-11-10);
   - then open a link of another branch, then production, also in a private window;
   - then quit Brave and start it again.
4. I close the control (c) and the two token-test pull requests. Approve (c)'s cleanup run; reject the other two, which
   have nothing to delete.
5. **Revoke the token** (no page gives these steps): Cloudflare, Manage account, Account API tokens, the token's row,
   its "..." menu, Delete. If there is no Delete, use "Roll", which "will invalidate the previous token" (roll-token
   page, 2026-04-20); copy the new value nowhere, and delete that one too. Tell me the time.
6. I start the run `revoked`. Approve it. It must fail at Cloudflare's sign-in: the proof that the token is dead
   (PM-0006-32's record).

### E. After the check
1. Archive the test repository once I say the evidence is saved: Settings, General, "Danger Zone", "Archive this
   repository" (decision TR-END; not read today).
2. Once the report is merged, delete the test project: the project, Settings, "Delete project" (not read today). If
   Cloudflare made an Access application for the wall, delete it in Zero Trust, Access, Applications (the
   known-issues page names that list).
3. Your go on the report's pull request in this repository.

**If anything about the token looks wrong at any time**, delete it at once (D5) and tell me. Nothing in this check is
worth keeping a doubtful token.

## Part 4. The security side in plain words (your item 4)
Sources for this part: security-auditor's reads of 2026-11-15 (`security-auditor-docs.md` in this folder, G1-G10)
and infra-engineer's (`infra-engineer-docs.md`, C1-C9); the quotes are theirs, from GitHub's and Cloudflare's pages.

### The token
- **What it is:** one Cloudflare API token, Account · Cloudflare Pages · Edit, for your account only, with an end date
  10 days out. Cloudflare recommends account-owned tokens for CI.
- **Where it lives:** in one place only, the secret box of the test repository's `preview` environment. You move it
  there from Cloudflare's screen (A5-A6), which shows it once. It never passes through this chat, a file or a note.
- **Who can read it:** a workflow job receives it only if the job names `preview`, and only after you approve that job:
  "a job cannot access environment secrets until one of the required reviewers approves it". It then lives in that one
  step's environment. GitHub hides the exact value if it is printed, but "this redaction is not guaranteed" for
  transformed values, so no script ever prints it or anything made from it. Whether GitHub's screens ever show a stored
  secret again is not stated on the pages read ("not found" in the security notes).
- **What it can do:** everything Pages allows in your account:
  - create, change and delete Pages projects and their uploads;
  - add custom domains;
  - upload server code (`_worker.js`) into a project;
  - rewrite any project's headers;
  - deploy any project's production (a deploy's branch "defaults to the project's production branch").

  Every Pages call takes this one permission, and no narrower one for a single project was found.
- **What it cannot do:** DNS records and Workers scripts need other permissions, so the token holds neither
  (security-auditor's reading of Cloudflare's permission list).
- **Open (not found either way):**
  - whether it can bind other data of your account (KV, R2, D1, Workers) into a Pages project;
  - whether a custom domain it adds on your own zone starts serving without a DNS permission. Cloudflare's dashboard
    steps say the record "will be added automatically" for a zone in the same account; for a call made with a
    Pages-only token, no page says.

  **If your account holds any Pages project, Worker, KV, R2 or D1 data besides the test project, tell me before you
  make the token.**
- **When it dies:** at its end date at the latest. You delete it when the check ends (D5), and a last deploy must
  then fail, as proof (D6).

### What a pull request can reach, and what it cannot (in the test repository)
- **A stranger's fork:** gets no secrets ("With the exception of `GITHUB_TOKEN`, secrets are not passed to the runner
  when a workflow is triggered from a forked repository"), and its token is read-only. With "Require approval for all
  external contributors" (A2), none of its runs starts without your click. Never approve a deploy in a fork's run.
- **A branch of the test repository** (only this session pushes them, as your login):
  - it runs its own version of the workflows (the workflow file comes from the pull request's merge commit), so it
    can rewrite the deploy job;
  - the token sits in `preview`, whose jobs wait for your approval, so a rewritten job gets nothing until you click;
  - `production` admits `main` only, and a pull request's run carries `refs/pull/N/merge` ("the deployment branch or
    tag rule is matched against the `GITHUB_REF`"), so a pull-request job that names `production` is refused before
    it starts. After a merge, the cleanup run carries `refs/heads/main` and passes, but that code is already on
    `main`.
- **The weak point (security-auditor, High: it bears on ADR 0006 B9 and ADR 0002 part 10).** This session and you are
  one GitHub login, so GitHub cannot tell your approval from an approval made with the sessions' credential.
  - "Prevent self-review" cannot help: switched on, it stops you approving the session's runs too.
  - What holds today: this session's GitHub tool has no way to approve a deploy (its run tool offers run, re-run,
    cancel and delete-logs only), and this repository's deny list refuses the `gh api` forms that write.
  - What is unknown: whether the credential behind the tools could approve if used some other way. It could if it
    holds Deployments write, or the classic `repo` scope, on the test repository; with Administration write it could
    remove the reviewer outright. **Ask SR-READ** (A3, step 3) gives the answer.
  - The check records this for OQ-0006-13 and does not settle it. The read-supported way to keep a preview token away
    from production is OQ-0006-13's option (b): production in a second Cloudflare account. A token for one project
    (option (c)) was not found.
- **What else holds:**
  - the admin bypass unticked on both environments (on by default);
  - the read-only GITHUB_TOKEN default;
  - "Allow GitHub Actions to create and approve pull requests" off;
  - every action pinned to a full commit SHA, with the setting that enforces it;
  - the deploy job refusing to upload server code;
  - every value from a pull request (its branch name) passed to the scripts through environment variables, never
    pasted into a command.
- **A pull request can switch off its own cleanup,** by editing or deleting its close workflow, or by a merge conflict.
  Case (d) does it on purpose. In step 2.6 the weekly sweep from `main` is the cleanup a branch cannot switch off.

### What could go wrong with the token, and what limits it
1. **A rewritten workflow on a test branch sends the token away.** Limited by your approval (each waiting run names its
   job and branch, and I tell you beforehand which ones will wait), the 10-day end date, the revocation at the end,
   and the token's reach (Pages in your account only).
2. **A poisoned npm package.** The deploy job downloads Wrangler from npm while it holds the token, and a poisoned
   dependency could read it. Limited by an exact Wrangler version at least 14 days old (4.156.0, published
   2026-10-29), the job's short life and the end date. Wrangler's own dependencies are not locked: a residual risk,
   and a finding for step 2.6, which should lock them.
3. **A printed token.** No step prints it, and GitHub masks the exact value.
4. **A leaked token used.** It could delete or deface Pages projects (only the test project exists, unless you tell me
   otherwise), or host unwanted pages under your account. Abuse could reach the account that also holds
   shamblefrontlines.com. Limited by the end date; revoke at once if anything looks wrong.
5. **You approve something you did not expect.** Before every sitting I list the runs that will wait. Anything else:
   reject it.

## Part 5. The calendar (your item 5)
**Day 0: your go, and the build**
- You answer the decisions, give your go on this plan and your yes to Part 7's texts, then do the setup (Part 3 A,
  about 40 minutes) when it suits you.
- I build the test repository's content (U1 to U3) on its `setup` branch, pushed without a pull request, so nothing
  runs. Then:
  - the review round (U4): infra-engineer, security-auditor and test-engineer at once;
  - the fixes (any change to a workflow's text comes back to you as a diff);
  - the advisor agent's review of the built content.

  About a day of session work.
- After your "setup done", I open the setup pull request: you approve its deploy, merge it, and approve its cleanup
  (Part 3 B). The schedule and the by-hand runs exist on `main` from then on.

**Day 1: the runs** (three sittings, about 1 h 20 min, best within one afternoon; Part 3 C). The closes happen in
sitting 2: **the 24-hour clocks start there.**

**What waits 24 hours:** PREV-DELETE's reading (PM-0006-6), and with it the login-wall steps (PM-0006-8 and checklist
steps 5 to 8), because the wall "hides what PREV-DELETE must see" (:1441-1442). Nothing else waits.

**Day 2: 24 hours after the last close** (one sitting, about 1 h 30 min with waits; Part 3 D): the reading, the wall,
the Brave steps 5 to 8, the token revoked and proven dead.

**The earliest day it can end:** Day 2, the day after the closes. If your go comes in the morning and the build and its
review take that day, the runs are the next day.
- Example: go on Monday 16 November, runs on Tuesday 17, end on Wednesday 18.
- Under TOMBSTONE (b), if plain deletion fails, one day more.
- The report and its docs change-set follow: about half a day of session work, then the review and your go.

This check does not hold the first builder chat. Step 2.6 also waits for that chat's change-set 2, the first page to
deploy, which can start only after your go on its plan on Tuesday 2026-11-17 (your note of today).

**Your approvals and sittings**, an input for OQ-0006-13 ("the approvals' cost per pull request", :1003-1004):

| When | Approvals | Rejections |
|---|---|---|
| The setup pull request (Part 3 B) | 2 (its deploy, its merge's cleanup) | |
| Sitting 1 | 5 runs (`round-a`, `uploads`, `bites`, `bite-production`, `restore-production`) | |
| Sitting 2 | 14 (10 deploys of the test pull requests, 4 cleanups) | |
| Sitting 3 | 1 | |
| Day 2 | 2 ((c)'s cleanup, `revoked`) | 2 |
| **All** | **24** | **2** |

That holds if one approval releases every job of a run waiting on `preview` at once. No GitHub page says so (G2:
"not found"); if each job needs its own, `round-a` and `bites` take 3 each, 28 in all. The count is recorded either
way. Under REVIEWER (b): about 2.

**Your time in all:** about 40 minutes of setup, plus about 3 hours over Days 1 and 2, waits included.

## Part 6. The laptop and the Mac (your item 6)
**By the draft's letter** (each row's machine column): the minimum laptop (ML) is named in 7 marks: PM-0006-1 (its
system is recorded), -2 (the header script runs on it), -3 (Chrome, Edge, Firefox), -4 (Chrome), -5 (the size script),
-6 (the deletion probe) and -8 (the wall probe). The Mac (MAC) is named in 2: PM-0006-1 (its system is recorded) and
-3 (Safari). Your PC in Brave (MP) is named in -3 (Brave), -7, -9 and -32; those stay yours in every option.

**None of them is a timing.** The draft says so itself: "They carry no budget row here: every client mark in this
record is a yes-or-no or a count" (:1309), and of the header script, "a header check, not a hardware result" (:1367).
Four of the seven (-2, -5, -6, -8) are scripts that fetch links: the machine they run on does not change what
Cloudflare answers. Only -3 and -4 run a browser.

**Today:** the laptop is not named, and your own laptop stands in "for rough checks only" (SPIKE_PLAN :66-67), which
can never pass a mark; no Mac is available (your words of 2026-11-06, SPIKE_PLAN :61-63).

**Does a waiting row hold step 2.6?** Yes, as things stand: SPIKE_PLAN says the check "must end before the
foundation's preview step" (:431-433), and a check with a row still waiting has not ended.

**The options (decision MACHINES):**
- **(a)** GitHub's own machines stand in for these rows: its Linux runner for the four scripts; its Windows runner for
  Chrome, Edge and Firefox (the same branded browsers, installed on the runner's image); its macOS runners for Safari
  (real Safari driven through safaridriver, not a WebKit build). Every result names the runner image and each
  browser's version. Nothing waits, and step 2.6 can start when the check ends. The precedent covers part of this
  only: for the netcode proof test, your words of 2026-11-03 asked whether GitHub's macOS runners could serve as its
  Safari check, and the advisor's reading of that day let a runner stand in for the Mac in that test's yes-or-no rows,
  on three conditions (SPIKE_PLAN :45-57). This plan applies the two that fit here (the Safari version recorded on
  the day; real Safari through safaridriver). For a Windows runner standing in for the laptop there is no precedent:
  that part is new, and yours to allow.
- **(b)** The four scripts run on GitHub's Linux runner (they are not hardware results), and the two browser rows
  (-3 and -4) wait for the named laptop and the Mac. Step 2.6 then waits as well, unless you also say that it may
  start once every other row has passed, with -3 and -4 run later.
- **(c)** Every row waits for the named laptop and a Mac, as the draft's letter has it: the check cannot end, and step
  2.6 cannot start, until both exist.

**Recommendation: (a).** These rows test what a browser does with the host's headers, and the same browser does the
same on any machine; it is the only option in which step 2.6 does not wait for a Mac nobody can lend you now.
Optional, and holding nothing: once the laptop is named and a Mac is to hand, PM-0006-3 can run once more on them in a
few minutes, reported beside the runners' results.

## Part 7. Every workflow file, with its exact text (your item 7)
These are the texts I ask your yes to. They live only in the test repository, under `.github/workflows/`; nothing
changes in this repository's `.github/`. No such file is written before your yes, and a text that changes in review
comes back to you as an exact diff. The test repository's files are outside this repository, so the guard asks nothing
when they are written: your yes in this chat is the only gate.

Every one follows `gate.yml`'s conventions:
- `permissions: contents: read` at the top;
- actions pinned to full commit SHAs: actions/checkout v7.0.1 and actions/setup-node v7.0.0 as `gate.yml` pins them;
  actions/upload-artifact v7.0.2 (cf430e03) and actions/download-artifact v8.0.2 (9000827c), both released
  2026-10-07, read with `git ls-remote` and their release pages on 2026-11-15;
- `persist-credentials: false` on every checkout.

Wrangler is pinned to 4.156.0, published 2026-10-29: the newest version at least 14 days old (npm, read 2026-11-15).
It is installed in a step that holds no secret, and only the installed program runs in the step that holds the token.
Node 22 comes from setup-node in every job that runs a script without the token. The two token jobs use the runner's
own Node and print its version. The runners are named, not `-latest`: `ubuntu-24.04`, `windows-2025`, `macos-26`.

Each file below parses as YAML (checked with PyYAML 6.0.1), and its sha256 is given, so the file written later can be
checked byte for byte against this plan. The draft copies sit in this chat's notes folder on the public branch
(`tr-draft/`), as text that nothing runs.

### 7.0 What changed after the advisor agent's review (revision 1 at 27e9f50, then revision 2)
The advisor agent approved revision 1 on conditions; its findings A1, A2, A3 and A7 changed these lines. A1: Wrangler is
installed in a step without the token, so install scripts never see it. A2: the browser job runs in bash on Windows, so
its browser list reaches the script. A3: the comment names the missing-COEP preview the job also opens. A7: Node 22 is
pinned in the jobs that run scripts. The two token-test variants change only in their shared first part, which now
carries the same setup-node lines. security-auditor, infra-engineer and the advisor agent read these lines again at
the build's review (Part 8, steps 4 and 6), before anything runs.

**`deploy.yml`**
```diff
@@ -41,6 +41,9 @@
       - uses: actions/checkout@3d3c42e5aac5ba805825da76410c181273ba90b1 # v7.0.1
         with:
           persist-credentials: false
+      - uses: actions/setup-node@820762786026740c76f36085b0efc47a31fe5020 # v7.0.0
+        with:
+          node-version: 22
       - id: legs
         env:
           EVENT: ${{ github.event_name }}
@@ -59,6 +62,9 @@
       - uses: actions/checkout@3d3c42e5aac5ba805825da76410c181273ba90b1 # v7.0.1
         with:
           persist-credentials: false
+      - uses: actions/setup-node@820762786026740c76f36085b0efc47a31fe5020 # v7.0.0
+        with:
+          node-version: 22
       - name: Build the test site and report its file sizes (PM-0006-5)
         env:
           MODE: ${{ matrix.mode }}
@@ -99,6 +105,10 @@
           if [ -e dist/_worker.js ] || [ -e dist/_routes.json ] || [ -e dist/functions ]; then
             echo "The build holds server code; nothing is deployed."; exit 1
           fi
+      - name: Install Wrangler (this step holds no secret)
+        run: |
+          node --version
+          npm install --prefix "$RUNNER_TEMP/wrangler" --no-audit --no-fund "$WRANGLER"
       - id: upload
         name: Upload to Cloudflare Pages
         env:
@@ -128,7 +138,7 @@
           mkdir -p records
           for i in $(seq 1 "$COUNT"); do
             : > "$WRANGLER_OUTPUT_FILE_PATH"
-            npx --yes "$WRANGLER" pages deploy dist --project-name "$PROJECT" --branch "$BRANCH" \
+            "$RUNNER_TEMP/wrangler/node_modules/.bin/wrangler" pages deploy dist --project-name "$PROJECT" --branch "$BRANCH" \
               --commit-hash "$MARKER" --commit-message "preview-check run $RUN $LEG $i" --commit-dirty=true
             jq -c 'select(.type == "pages-deploy-detailed")' "$WRANGLER_OUTPUT_FILE_PATH" > detailed.ndjson
             test "$(wc -l < detailed.ndjson)" -eq 1
@@ -157,6 +167,9 @@
       - uses: actions/checkout@3d3c42e5aac5ba805825da76410c181273ba90b1 # v7.0.1
         with:
           persist-credentials: false
+      - uses: actions/setup-node@820762786026740c76f36085b0efc47a31fe5020 # v7.0.0
+        with:
+          node-version: 22
       - uses: actions/download-artifact@9000827ccba6bdab643e8b6fd33ac0654aef8333 # v8.0.2
         with:
           pattern: deployment-record-${{ github.run_id }}-*
```

**`close.yml`**
```diff
@@ -24,6 +24,11 @@
     timeout-minutes: 15
     environment: preview
     steps:
+      - name: Tombstone cases only (cs/pdt-*) - install Wrangler (this step holds no secret)
+        if: startsWith(github.event.pull_request.head.ref, 'cs/pdt-')
+        run: |
+          node --version
+          npm install --prefix "$RUNNER_TEMP/wrangler" --no-audit --no-fund "$WRANGLER"
       - id: tomb
         name: Tombstone cases only (cs/pdt-*) - upload a page without a build marker
         if: startsWith(github.event.pull_request.head.ref, 'cs/pdt-')
@@ -39,7 +44,7 @@
           mkdir -p tombstone records
           printf '<!doctype html>\n<meta charset="utf-8">\n<title>Preview gone</title>\n<p>This preview is gone.</p>\n' \
             > tombstone/index.html
-          npx --yes "$WRANGLER" pages deploy tombstone --project-name "$PROJECT" --branch "$BRANCH" \
+          "$RUNNER_TEMP/wrangler/node_modules/.bin/wrangler" pages deploy tombstone --project-name "$PROJECT" --branch "$BRANCH" \
             --commit-hash "$GITHUB_SHA" --commit-message "tombstone run $RUN" --commit-dirty=true
           jq -c 'select(.type == "pages-deploy-detailed")' "$WRANGLER_OUTPUT_FILE_PATH" > detailed.ndjson
           test "$(wc -l < detailed.ndjson)" -eq 1
```

**`probe.yml`**
```diff
@@ -34,6 +34,9 @@
       - uses: actions/checkout@3d3c42e5aac5ba805825da76410c181273ba90b1 # v7.0.1
         with:
           persist-credentials: false
+      - uses: actions/setup-node@820762786026740c76f36085b0efc47a31fe5020 # v7.0.0
+        with:
+          node-version: 22
       - name: Gather every upload's record and every pull request's state
         env:
           GH_TOKEN: ${{ github.token }}
```

**`browsers.yml`**
```diff
@@ -1,9 +1,9 @@
 # THROWAWAY: the preview hosting check for SHAMBLE FRONTLINES (ADR 0006, spike 1). Plan and results live in
 # Mergincanli/Shamble-Frontlines. This file is deleted with this test repository.
-# Started by hand: opens the three links (and the planted no-cache preview) in the branded browsers of GitHub's own
-# machines through WebdriverIO, and records what the page and its worker report: crossOriginIsolated (PM-0006-3) and
-# the content file's second-visit transfer size (PM-0006-4, Chrome). Every result names the runner image and each
-# browser's version. It holds no secret.
+# Started by hand: opens the three links, the planted missing-COEP preview and the planted no-cache preview in the
+# branded browsers of GitHub's own machines through WebdriverIO, and records what the page and its worker report:
+# crossOriginIsolated (PM-0006-3) and the content file's second-visit transfer size (PM-0006-4, Chrome). Every result
+# names the runner image and each browser's version. It holds no secret. Every step runs in bash, on Windows too.
 name: browsers
 
 on:
@@ -24,6 +24,9 @@
             browsers: safari
     runs-on: ${{ matrix.os }}
     timeout-minutes: 30
+    defaults:
+      run:
+        shell: bash
     steps:
       - uses: actions/checkout@3d3c42e5aac5ba805825da76410c181273ba90b1 # v7.0.1
         with:
```

### 7.1 `.github/workflows/deploy.yml` (185 lines; sha256 333e5a592a05764137c129857d087236f2e1131f49c74dcd36c320d336614291)
```yaml
# THROWAWAY: the preview hosting check for SHAMBLE FRONTLINES (ADR 0006, spike 1). Plan and results live in
# Mergincanli/Shamble-Frontlines. This file is deleted with this test repository.
# Deploys the test site to the test project on Cloudflare Pages:
#   - for every pull request from a branch of this repository: one preview of that branch;
#   - when started by hand on `main`: one named set of deploys (tools/sets.json).
# plan    turns the event into a list of deploys ("legs"); holds no secret
# build   builds the test site for each leg and reports its file sizes (PM-0006-5); holds no secret
# deploy  uploads each build; the Pages token reaches it only through the `preview` environment, whose jobs wait for
#         Mustafa's approval; it checks out and runs nothing from this repository
# verify  reads the headers of every file on both links of every upload (PM-0006-2); holds no secret
name: deploy

on:
  pull_request:
    types: [opened, synchronize, reopened]
  workflow_dispatch:
    inputs:
      set:
        description: Which set of deploys to make (tools/sets.json)
        required: true
        type: choice
        options: [round-a, bites, bite-production, restore-production, uploads, revoked]

permissions:
  contents: read

env:
  PROJECT: shamble-preview-check
  WRANGLER: wrangler@4.156.0

jobs:
  plan:
    if: >-
      (github.event_name == 'workflow_dispatch' && github.ref == 'refs/heads/main') ||
      (github.event_name == 'pull_request' && github.event.pull_request.head.repo.full_name == github.repository)
    runs-on: ubuntu-24.04
    timeout-minutes: 5
    outputs:
      legs: ${{ steps.legs.outputs.legs }}
    steps:
      - uses: actions/checkout@3d3c42e5aac5ba805825da76410c181273ba90b1 # v7.0.1
        with:
          persist-credentials: false
      - uses: actions/setup-node@820762786026740c76f36085b0efc47a31fe5020 # v7.0.0
        with:
          node-version: 22
      - id: legs
        env:
          EVENT: ${{ github.event_name }}
          SET: ${{ inputs.set }}
          BRANCH: ${{ github.event.pull_request.head.ref }}
        run: echo "legs=$(node tools/sets.mjs --event "$EVENT" --set "$SET" --branch "$BRANCH")" >> "$GITHUB_OUTPUT"

  build:
    needs: plan
    runs-on: ubuntu-24.04
    timeout-minutes: 10
    strategy:
      matrix:
        include: ${{ fromJSON(needs.plan.outputs.legs) }}
    steps:
      - uses: actions/checkout@3d3c42e5aac5ba805825da76410c181273ba90b1 # v7.0.1
        with:
          persist-credentials: false
      - uses: actions/setup-node@820762786026740c76f36085b0efc47a31fe5020 # v7.0.0
        with:
          node-version: 22
      - name: Build the test site and report its file sizes (PM-0006-5)
        env:
          MODE: ${{ matrix.mode }}
          MARKER: ${{ github.sha }}
        run: |
          node --version
          node tools/build.mjs --mode "$MODE" --marker "$MARKER" --out dist
          node tools/check-sizes.mjs dist
      - uses: actions/upload-artifact@cf430e030ddbb5b0abf93d22962f4752f3646cd9 # v7.0.2
        with:
          name: site-${{ matrix.leg }}
          path: dist/
          retention-days: 7
          if-no-files-found: error

  deploy:
    needs: [plan, build]
    runs-on: ubuntu-24.04
    timeout-minutes: 20
    strategy:
      fail-fast: false
      matrix:
        include: ${{ fromJSON(needs.plan.outputs.legs) }}
    permissions:
      contents: read
      actions: read
      deployments: write
    environment:
      name: preview
      url: ${{ steps.upload.outputs.url }}
    steps:
      - uses: actions/download-artifact@9000827ccba6bdab643e8b6fd33ac0654aef8333 # v8.0.2
        with:
          name: site-${{ matrix.leg }}
          path: dist
      - name: Refuse server code in the upload
        run: |
          if [ -e dist/_worker.js ] || [ -e dist/_routes.json ] || [ -e dist/functions ]; then
            echo "The build holds server code; nothing is deployed."; exit 1
          fi
      - name: Install Wrangler (this step holds no secret)
        run: |
          node --version
          npm install --prefix "$RUNNER_TEMP/wrangler" --no-audit --no-fund "$WRANGLER"
      - id: upload
        name: Upload to Cloudflare Pages
        env:
          CLOUDFLARE_API_TOKEN: ${{ secrets.CLOUDFLARE_API_TOKEN }}
          CLOUDFLARE_ACCOUNT_ID: ${{ secrets.CLOUDFLARE_ACCOUNT_ID }}
          LEG: ${{ matrix.leg }}
          MODE: ${{ matrix.mode }}
          EXPECT: ${{ matrix.expect }}
          BRANCH: ${{ matrix.branch }}
          COUNT: ${{ matrix.count }}
          MARKER: ${{ github.sha }}
          PR: ${{ github.event.pull_request.number }}
          RUN: ${{ github.run_id }}
          WRANGLER_OUTPUT_FILE_PATH: ${{ runner.temp }}/wrangler-output.ndjson
        run: |
          set -euo pipefail
          umask 077
          if [ "$BRANCH" = "@production" ]; then
            printf 'Authorization: Bearer %s\n' "$CLOUDFLARE_API_TOKEN" > "$RUNNER_TEMP/auth"
            BRANCH=$(curl -fsS -H @"$RUNNER_TEMP/auth" \
              "https://api.cloudflare.com/client/v4/accounts/$CLOUDFLARE_ACCOUNT_ID/pages/projects/$PROJECT" \
              | jq -r '.result.production_branch')
            rm -f "$RUNNER_TEMP/auth"
            test -n "$BRANCH" && test "$BRANCH" != "null"
            echo "The project's production branch: $BRANCH"
          fi
          mkdir -p records
          for i in $(seq 1 "$COUNT"); do
            : > "$WRANGLER_OUTPUT_FILE_PATH"
            "$RUNNER_TEMP/wrangler/node_modules/.bin/wrangler" pages deploy dist --project-name "$PROJECT" --branch "$BRANCH" \
              --commit-hash "$MARKER" --commit-message "preview-check run $RUN $LEG $i" --commit-dirty=true
            jq -c 'select(.type == "pages-deploy-detailed")' "$WRANGLER_OUTPUT_FILE_PATH" > detailed.ndjson
            test "$(wc -l < detailed.ndjson)" -eq 1
            jq -c --arg branch "$BRANCH" --arg marker "$MARKER" --arg pr "$PR" --arg run "$RUN" --arg leg "$LEG" \
              --arg mode "$MODE" --arg expect "$EXPECT" --arg upload "$i" \
              '{deployment_id, url, alias, environment, production_branch, branch: $branch, marker: $marker,
                pr: $pr, run: $run, leg: $leg, upload: $upload, mode: $mode, expect: $expect, made: .timestamp}' \
              detailed.ndjson > "records/$RUN-$LEG-$i.json"
            cat "records/$RUN-$LEG-$i.json"
          done
          echo "url=$(jq -r '.alias // .url' "records/$RUN-$LEG-$COUNT.json")" >> "$GITHUB_OUTPUT"
      - uses: actions/upload-artifact@cf430e030ddbb5b0abf93d22962f4752f3646cd9 # v7.0.2
        if: always()
        with:
          name: deployment-record-${{ github.run_id }}-${{ matrix.leg }}
          path: records/
          retention-days: 7
          if-no-files-found: warn

  verify:
    needs: deploy
    if: always() && needs.deploy.result != 'skipped'
    runs-on: ubuntu-24.04
    timeout-minutes: 10
    steps:
      - uses: actions/checkout@3d3c42e5aac5ba805825da76410c181273ba90b1 # v7.0.1
        with:
          persist-credentials: false
      - uses: actions/setup-node@820762786026740c76f36085b0efc47a31fe5020 # v7.0.0
        with:
          node-version: 22
      - uses: actions/download-artifact@9000827ccba6bdab643e8b6fd33ac0654aef8333 # v8.0.2
        with:
          pattern: deployment-record-${{ github.run_id }}-*
          path: records
          merge-multiple: true
      - name: Every file's headers on both links (PM-0006-2)
        run: node tools/check-headers.mjs --records records --out headers.json
      - uses: actions/upload-artifact@cf430e030ddbb5b0abf93d22962f4752f3646cd9 # v7.0.2
        if: always()
        with:
          name: headers-${{ github.run_id }}
          path: headers.json
          retention-days: 30
```

### 7.2 `.github/workflows/close.yml` (98 lines; sha256 98b42b8e59cf1c36bd2e9b035627d7e56b2dac246e3831e15674c1d20f51dc3b)
```yaml
# THROWAWAY: the preview hosting check for SHAMBLE FRONTLINES (ADR 0006, spike 1). Plan and results live in
# Mergincanli/Shamble-Frontlines. This file is deleted with this test repository.
# When a pull request from a branch of this repository closes (merged or not), delete every upload of its branch from
# the test project through Cloudflare's API, with the force option, oldest first (PREV-DELETE, PM-0006-6). For the
# tombstone cases (branches cs/pdt-*), first upload a page without a build marker as the branch's newest upload, and
# keep that one. The token reaches this job only through the `preview` environment (Mustafa's approval).
name: close

on:
  pull_request:
    types: [closed]

permissions:
  contents: read

env:
  PROJECT: shamble-preview-check
  WRANGLER: wrangler@4.156.0

jobs:
  cleanup:
    if: github.event.pull_request.head.repo.full_name == github.repository
    runs-on: ubuntu-24.04
    timeout-minutes: 15
    environment: preview
    steps:
      - name: Tombstone cases only (cs/pdt-*) - install Wrangler (this step holds no secret)
        if: startsWith(github.event.pull_request.head.ref, 'cs/pdt-')
        run: |
          node --version
          npm install --prefix "$RUNNER_TEMP/wrangler" --no-audit --no-fund "$WRANGLER"
      - id: tomb
        name: Tombstone cases only (cs/pdt-*) - upload a page without a build marker
        if: startsWith(github.event.pull_request.head.ref, 'cs/pdt-')
        env:
          CLOUDFLARE_API_TOKEN: ${{ secrets.CLOUDFLARE_API_TOKEN }}
          CLOUDFLARE_ACCOUNT_ID: ${{ secrets.CLOUDFLARE_ACCOUNT_ID }}
          BRANCH: ${{ github.event.pull_request.head.ref }}
          PR: ${{ github.event.pull_request.number }}
          RUN: ${{ github.run_id }}
          WRANGLER_OUTPUT_FILE_PATH: ${{ runner.temp }}/wrangler-output.ndjson
        run: |
          set -euo pipefail
          mkdir -p tombstone records
          printf '<!doctype html>\n<meta charset="utf-8">\n<title>Preview gone</title>\n<p>This preview is gone.</p>\n' \
            > tombstone/index.html
          "$RUNNER_TEMP/wrangler/node_modules/.bin/wrangler" pages deploy tombstone --project-name "$PROJECT" --branch "$BRANCH" \
            --commit-hash "$GITHUB_SHA" --commit-message "tombstone run $RUN" --commit-dirty=true
          jq -c 'select(.type == "pages-deploy-detailed")' "$WRANGLER_OUTPUT_FILE_PATH" > detailed.ndjson
          test "$(wc -l < detailed.ndjson)" -eq 1
          jq -c --arg branch "$BRANCH" --arg pr "$PR" --arg run "$RUN" \
            '{deployment_id, url, alias, environment, production_branch, branch: $branch, marker: "",
              pr: $pr, run: $run, mode: "tombstone", expect: "none", made: .timestamp}' detailed.ndjson \
            > "records/$RUN-tombstone.json"
          cat "records/$RUN-tombstone.json"
          echo "keep=$(jq -r .deployment_id "records/$RUN-tombstone.json")" >> "$GITHUB_OUTPUT"
      - name: Delete every other upload of this branch (force), oldest first
        env:
          CLOUDFLARE_API_TOKEN: ${{ secrets.CLOUDFLARE_API_TOKEN }}
          CLOUDFLARE_ACCOUNT_ID: ${{ secrets.CLOUDFLARE_ACCOUNT_ID }}
          BRANCH: ${{ github.event.pull_request.head.ref }}
          CLOSED_AT: ${{ github.event.pull_request.closed_at }}
          MERGED: ${{ github.event.pull_request.merged }}
          KEEP: ${{ steps.tomb.outputs.keep }}
        run: |
          set -euo pipefail
          umask 077
          test -n "$BRANCH" || { echo "The event names no head branch; nothing deleted."; exit 1; }
          echo "Pull request closed at $CLOSED_AT (merged: $MERGED); this job started at $(date -u +%FT%TZ)."
          slug=$(printf '%s' "$BRANCH" | tr 'A-Z' 'a-z' | sed 's/[^a-z0-9]/-/g')
          printf 'Authorization: Bearer %s\n' "$CLOUDFLARE_API_TOKEN" > "$RUNNER_TEMP/auth"
          api="https://api.cloudflare.com/client/v4/accounts/$CLOUDFLARE_ACCOUNT_ID/pages/projects/$PROJECT/deployments"
          : > all.ndjson
          page=1
          while :; do
            curl -fsS -H @"$RUNNER_TEMP/auth" "$api?env=preview&page=$page&per_page=25" > page.json
            jq -c '.result[]' page.json >> all.ndjson
            [ "$(jq '.result | length' page.json)" -lt 25 ] && break
            page=$((page + 1))
          done
          jq -sc --arg b "$BRANCH" --arg s "$slug" \
            'map(select(.deployment_trigger.metadata.branch == $b or .deployment_trigger.metadata.branch == $s))
             | sort_by(.created_on) | .[]' all.ndjson > mine.ndjson
          echo "Uploads of $BRANCH found: $(wc -l < mine.ndjson)"
          while read -r d; do
            id=$(jq -r .id <<< "$d")
            if [ -n "$KEEP" ] && [ "$id" = "$KEEP" ]; then echo "Kept (the tombstone): $id"; continue; fi
            code=$(curl -sS -o resp.json -w '%{http_code}' -X DELETE -H @"$RUNNER_TEMP/auth" "$api/$id?force=true")
            echo "Delete $id ($(jq -r .url <<< "$d")): HTTP $code $(jq -c '{success, errors}' resp.json)"
          done < mine.ndjson
          rm -f "$RUNNER_TEMP/auth"
          echo "This job ended at $(date -u +%FT%TZ)."
      - uses: actions/upload-artifact@cf430e030ddbb5b0abf93d22962f4752f3646cd9 # v7.0.2
        if: steps.tomb.outcome == 'success'
        with:
          name: deployment-record-${{ github.run_id }}-tombstone
          path: records/
          retention-days: 7
```

### 7.3 `.github/workflows/probe.yml` (66 lines; sha256 f24ad6e81daff7b20798c57d7a6a7982c2a6a5bb0e0f89e51459a4d279a84154)
```yaml
# THROWAWAY: the preview hosting check for SHAMBLE FRONTLINES (ADR 0006, spike 1). Plan and results live in
# Mergincanli/Shamble-Frontlines. This file is deleted with this test repository.
# Fetches every link of every upload the way a stranger would (no login, no cookies, a named User-Agent) and records
# what came back: hourly for the PREV-DELETE reading (PM-0006-6), and when started by hand for the 24-hour reading and
# for the login-wall sweeps (PM-0006-8). It holds no secret.
name: probe

on:
  schedule:
    - cron: '23 * * * *'
  workflow_dispatch:
    inputs:
      purpose:
        description: deletion (PM-0006-6) or wall (PM-0006-8)
        required: true
        default: deletion
        type: choice
        options: [deletion, wall]

permissions:
  contents: read
  actions: read
  pull-requests: read

concurrency:
  group: probe
  cancel-in-progress: false

jobs:
  probe:
    runs-on: ubuntu-24.04
    timeout-minutes: 20
    steps:
      - uses: actions/checkout@3d3c42e5aac5ba805825da76410c181273ba90b1 # v7.0.1
        with:
          persist-credentials: false
      - uses: actions/setup-node@820762786026740c76f36085b0efc47a31fe5020 # v7.0.0
        with:
          node-version: 22
      - name: Gather every upload's record and every pull request's state
        env:
          GH_TOKEN: ${{ github.token }}
          REPO: ${{ github.repository }}
        run: |
          set -euo pipefail
          mkdir -p records
          gh api --paginate "repos/$REPO/actions/artifacts?per_page=100" \
            --jq '.artifacts[] | select(.expired == false and (.name | startswith("deployment-record"))) | .id' > ids.txt
          echo "Records found: $(wc -l < ids.txt)"
          while read -r id; do
            gh api "repos/$REPO/actions/artifacts/$id/zip" > "a-$id.zip"
            mkdir -p "records/$id"
            unzip -q -o "a-$id.zip" -d "records/$id"
          done < ids.txt
          gh api --paginate "repos/$REPO/pulls?state=all&per_page=100" \
            --jq '.[] | {number, head: .head.ref, state, closed_at, merged_at}' > pulls.ndjson
      - name: Probe every link
        env:
          PURPOSE: ${{ inputs.purpose || 'deletion' }}
        run: node tools/probe.mjs --records records --pulls pulls.ndjson --purpose "$PURPOSE" --out probe-results.json
      - uses: actions/upload-artifact@cf430e030ddbb5b0abf93d22962f4752f3646cd9 # v7.0.2
        if: always()
        with:
          name: probe-results-${{ github.run_id }}
          path: probe-results.json
          retention-days: 30
```

### 7.4 `.github/workflows/browsers.yml` (51 lines; sha256 6791076916ae67191e3a4c6e194cde48dc0db7bb1f95f2da21b7ddb814e773f5)
```yaml
# THROWAWAY: the preview hosting check for SHAMBLE FRONTLINES (ADR 0006, spike 1). Plan and results live in
# Mergincanli/Shamble-Frontlines. This file is deleted with this test repository.
# Started by hand: opens the three links, the planted missing-COEP preview and the planted no-cache preview in the
# branded browsers of GitHub's own machines through WebdriverIO, and records what the page and its worker report:
# crossOriginIsolated (PM-0006-3) and the content file's second-visit transfer size (PM-0006-4, Chrome). Every result
# names the runner image and each browser's version. It holds no secret. Every step runs in bash, on Windows too.
name: browsers

on:
  workflow_dispatch:

permissions:
  contents: read

jobs:
  run:
    strategy:
      fail-fast: false
      matrix:
        include:
          - os: windows-2025
            browsers: chrome,MicrosoftEdge,firefox
          - os: macos-26
            browsers: safari
    runs-on: ${{ matrix.os }}
    timeout-minutes: 30
    defaults:
      run:
        shell: bash
    steps:
      - uses: actions/checkout@3d3c42e5aac5ba805825da76410c181273ba90b1 # v7.0.1
        with:
          persist-credentials: false
      - uses: actions/setup-node@820762786026740c76f36085b0efc47a31fe5020 # v7.0.0
        with:
          node-version: 22
      - name: Allow WebDriver in Safari
        if: runner.os == 'macOS'
        run: sudo safaridriver --enable
      - name: Install WebdriverIO from its lockfile, without install scripts
        run: npm ci --prefix tools/wdio --ignore-scripts --no-audit --no-fund
      - name: Isolation (PM-0006-3) and the content file's cache (PM-0006-4)
        env:
          BROWSERS: ${{ matrix.browsers }}
        run: node tools/wdio/run.mjs --browsers "$BROWSERS" --links tools/links.json --out browser-results.json
      - uses: actions/upload-artifact@cf430e030ddbb5b0abf93d22962f4752f3646cd9 # v7.0.2
        if: always()
        with:
          name: browsers-${{ matrix.os }}
          path: browser-results.json
          retention-days: 30
```

### 7.5 The two token-test variants (PM-0006-32), each on its own branch
Each is `deploy.yml` above with everything from the line `  deploy:` to the end replaced. The lines before it stay
byte-identical (checked by script).

**On branch `cs/b9-edit`** (the deploy job rewritten to print a marker; the verify job dropped, since it needs the
deploy's records). The whole file's sha256 is fd3dc4c6029b2d6a0e0bb311bc97dafe11ab2966f61258668ef782d0786d357f. The replacement:
```yaml
  deploy:
    needs: build
    runs-on: ubuntu-24.04
    timeout-minutes: 5
    environment: preview
    steps:
      - name: B9 marker (the deploy job, rewritten by this pull request)
        env:
          TOKEN: ${{ secrets.CLOUDFLARE_API_TOKEN }}
        run: |
          echo "B9-MARKER: the rewritten deploy job ran at $(date -u +%FT%TZ)"
          if [ -n "$TOKEN" ]; then echo "secret: present"; else echo "secret: absent"; fi
```

**On branch `cs/b9-prod`** (a pull-request job that names `production`; the deploy and verify jobs dropped, so nothing
waits for you). The whole file's sha256 is 8b745631a278813d7d5dc80c87bbfcaedeec0651111f5a54784c05f04d2d964d. The replacement:
```yaml
  b9-production:
    runs-on: ubuntu-24.04
    timeout-minutes: 5
    environment: production
    steps:
      - name: B9 marker (a pull-request job that names production)
        env:
          CANARY: ${{ secrets.CANARY }}
        run: |
          echo "B9-MARKER: the production job ran at $(date -u +%FT%TZ)"
          if [ -n "$CANARY" ]; then echo "secret: present"; else echo "secret: absent"; fi
```

The pull request (d), `cs/pd-off`, deletes `.github/workflows/close.yml` on its branch, and changes nothing else in
`.github/`.

### 7.6 What changes under the other options
- **TOKENS (b), two tokens:**
  - `production` holds a second Pages token instead of the canary;
  - `deploy.yml` gains a job `deploy-production`: the deploy job's steps for the legs whose branch is `@production`,
    with `environment: production` and `if: github.ref == 'refs/heads/main'`;
  - the `preview` job skips those legs;
  - the reported row (the preview token on production) gets a leg of its own in the `preview` job;
  - about 40 lines more, a second token to make and revoke, and both texts come back to you for a yes.
- **REVIEWER (b):** no workflow change. You untick "Required reviewers" on `preview` for everything but sitting 3.
- **MACHINES (b) or (c):** `browsers.yml` is not written. Its rows are run later on the named laptop and the Mac by a
  script I give you then.
- **TOMBSTONE (b):** no change. The tombstone path of `close.yml` is used only by `cs/pdt-*` pull requests opened after
  a failure.
- **ACCESS-CARD (b):** no change.
- **TR-WHERE (b), this repository:**
  - all four files move into this repository's `.github/workflows/` under the kit and CI rules (risk tier 2: the
    advisor agent and security-auditor, your yes in Accept edits);
  - each gains branch filters so it runs only for the test branches;
  - the test site needs a folder, and spikes live in `spikes/`, which your words forbid me to touch: that needs your
    word.

## Part 8. How the check is run, reviewed and reported
**CLAUDE.md's change-set protocol, step by step.**
1. **Plan:** this plan, then the advisor agent's review of it (no review rounds and no verifier on the plan, by your
   words).
2. **Your go,** your yes to Part 7's texts, and the asks of "For Mustafa".
3. **Build U1 to U3** in the test repository, on its branch `setup`, pushed without a pull request, so no workflow
   runs. In this repository a draft pull request opens from my branch, holding the checkpoint, so STATUS can list the
   work.
4. **One review round** on the built content, read from the branch: infra-engineer (CI and hosting), security-auditor
   (the token and CI) and test-engineer (are the marks proven, do the bites bite), at once. They run at their files'
   effort, high: your words ask for max for the agents that review, verify or design game and engine code, and this is
   neither. Say if you want max anyway.
5. **Fixes;** a workflow text that changes comes back to you.
6. **The advisor agent's review** of the built head.
7. **After your "setup done":** the setup pull request opened; you approve its deploy, merge it, and approve its
   cleanup (Part 3 B).
8. **The runs** (U5).
9. **The verifier** on the evidence. Research and infrastructure claims cite their sources, and the verifier
   spot-checks the key ones.
10. **The report** (`/advisor-report`, U6), then docs-keeper: STATUS, ROADMAP, and a dated pointer in SPIKE_PLAN entry 6.
11. **This repository's pull request** marked ready, the advisor agent's review of its head, and your go.

**Risk tier, for the advisor agent to rule:**
- The check itself handles a secret and sets up infrastructure: a deploy token, two environments, a Pages project.
  CLAUDE.md's table puts secrets and infrastructure in risk tier 2: the advisor agent and security-auditor review, and
  you give the go. Both review the built content before you merge the setup pull request (steps 4 and 6), and nothing
  runs before your merge.
- The closing pull request in this repository changes docs only: the report, STATUS, ROADMAP, a SPIKE_PLAN pointer. It
  records your words and checklist verdicts, so it is at least risk tier 1, with your go.
- Whether the check's tier 2 carries over to that pull request ("A PR takes the highest tier any changed file or
  effect reaches") is for the advisor agent to rule.

**What the closing docs change-set carries:**
- the report, with the results log in its appendix;
- the four workflow texts as run, with the test repository's commit;
- the PR-page picture of PM-0006-10: the session takes it from the public page, logged out, and you say what your own
  page shows;
- STATUS, ROADMAP and the SPIKE_PLAN pointer.

The answers the check settles (PREV-DELETE, R5-PREV-OQ5 and OQ6) and the inputs for OQ-0006-4 and OQ-0006-13 are stated
in the report. Writing them into the decision drafts is their own change-set, after your word. A failed mark goes back
to the draft's author by the draft's own route (:1401-1415).

**Checks while building:** `node --check` on every script; a local dry run of the build in every mode, listing every
file and every header rule it writes; a YAML parse of every workflow. This repository's gate runs on the closing docs
change-set.

**The advisor tool:** consulted at the five moments of your words of 2026-11-11, always before a step is saved as done;
each consultation recorded in the checkpoint in one line with its time.

**Checkpoints:** after every step, at least every 30 minutes, with the saver running while agents run. The 24-hour
reading rests on a reminder I set when the closes happen, which survives a restart of this session.

**Night queue:** nothing goes there. Every step needs you, or a reading by this chat.

**What step 2.6 waits for** (the first builder chat's plan, Part 8):
- this check ended;
- that chat's change-set 2 merged, a page to deploy (after your go on its plan, 2026-11-17);
- the 2.3 chat's CI change-set merged;
- your deploy token in this repository's `preview`, with B9's interim position, and OQ-0006-13 answered.

This check gives the last condition its inputs: the approvals' cost, SR-READ, and the reported production deploy with
the preview token.

**Findings for step 2.6 already** (from the reads):
- create the real project by hand, never through an agent session (Wrangler's redirect to Workers);
- always pass `--branch` and `--commit-hash`, and delete with `--force` or the API (Wrangler's delete does nothing in CI
  without it);
- give every probe a named User-Agent;
- `deployments: write` may not be needed (G6; the check keeps it, as ADR 0002 part 10 does);
- lock Wrangler's dependency tree;
- put a weekly sweep on `main`.

## Part 9. Risks
1. **PREV-DELETE fails even with the tombstone.** The draft goes back to its author, Workers static assets is weighed
   again, and step 2.6 waits for the revised draft and your decision. Cloudflare's own Pages page now says "Start new
   projects with Workers" (2026-08-25), which the draft's author should weigh whatever the check shows
   (infra-engineer's surprise 6).
2. **The login wall asks for a Zero Trust plan and payment details:** decision ACCESS-CARD.
3. **The reviewer gate is no wall against a session using your login** (Part 4): recorded for OQ-0006-13, with SR-READ.
4. **GitHub's schedule runs late or skips an hour:** my own probe just after 24 hours; every gap is recorded.
5. **Pages blocks the probes** (Browser Integrity Check, error 1010): a named User-Agent, and a 403 never counts as
   "gone". The control (c) would catch a blocked probe, since it must keep serving.
6. **The production branch has an unexpected name:** the workflows read it from Cloudflare.
7. **A runner image updates mid-check** (weekly): the versions are recorded in every run.
8. **Your sittings move:** the 10-day end date bounds the window. If it passes first, a new token (A5 and A6 again).
9. **A usage limit cuts this session:** the checkpoint, the saver, and reminders that survive a restart.
10. **Case (d)'s switched-off cleanup rests on an inference.** GitHub runs "the version of the workflow that is present
    in the associated commit SHA" (security-auditor's reading). If a cleanup run appears for (d) anyway, the bite did
    not bite, and the case is redone with the workflow switched off in the Actions tab (your click).
11. **The access policy has no documented off switch:** deleting the project ends it, and E2 removes the Access
    application.
12. **A merged pull request's cleanup run might be skipped.** A security note quotes GitHub saying the event's data "is
    empty for merged pull requests", while GitHub's own advice for merged pull requests reads that very data. If it is
    empty, `close.yml`'s first check fails quietly and (b) and (b') keep their uploads. The setup pull request's merge
    tests this first (Part 3 B, step 4). If it fails there, the fix comes back to you as a diff before (b) and (b')
    are opened.

## Decisions needed (your item 8)
1. **TR-WHERE: where does the check live?**
   - Why it matters: the token, the throwaway workflows and the test pull requests have to live in some repository.
     ADR 0006 says you name it (:1280-1281), and ADR 0002's PM-0002-66 already names "the test repository, ADR 0006's
     TR" (0002:1332).
   - Options:
     - (a) A new public repository of yours, `shamble-preview-check`. Nothing changes in this repository's CI or in
       `spikes/`; the token never sits where four chats push branches; test pull requests stay out of the PR Advisor's
       and the night shift's way. Costs: one repository to create, attach and archive; you merge its setup pull request
       and two test pull requests yourself.
     - (b) This repository. The workflows enter `.github/` as a CI change (risk tier 2); the token sits in this
       repository's `preview`, which every branch's workflow can name; the test site needs a folder, and spikes live
       in `spikes/`, which your words forbid me to touch.
   - Note: SPIKE_PLAN says "a spike ... lives in `spikes/`" (:12). (a) reads that rule as not applying to a check whose
     whole repository is throwaway. The advisor agent rules on that reading. If it rules the rule binding, I stop and
     tell you in three lines.
   - Recommendation: (a).
2. **TOKENS: one Cloudflare token, or two?**
   - Why it matters: part 13's interim position (a) gives production "its own token" in its own environment (:564-566).
   - Options:
     - (a) One token, in `preview` only. The test project's production is deployed by a `preview` job, and `production`
       holds a harmless canary, enough to show whether a job receives a secret. This departs from interim position (a)
       for the test project only. It also answers PM-0006-32's reported row on the way (the preview token deploying
       production).
     - (b) Two tokens, each in its own environment, as the interim position has it: one more token to make, store and
       revoke, and about 40 more workflow lines (Part 7.6). Both tokens reach the whole account (Part 4), so the second
       separates nothing that matters here.
   - Recommendation: (a).
3. **REVIEWER: does every deploy wait for your approval, or only the token tests?**
   - Why it matters: part 13's interim position (b) puts a reviewer on `preview`. That is "the infra draft as
     proposed", and it costs your clicks.
   - Options:
     - (a) The reviewer on from the first deploy: about 24 approvals, over the setup pull request and four sittings,
       and the check measures what a reviewer costs per pull request, an input for OQ-0006-13.
     - (b) The reviewer on only for sitting 3: about 2 approvals; deploys run unattended; no cost measured; the draft's
       interim position not exercised.
   - Recommendation: (a).
4. **MACHINES: may GitHub's machines stand in for the minimum laptop and the Mac?** (Part 6)
   - Why it matters: 7 marks name the laptop and 2 the Mac. Neither machine exists yet, and a waiting row holds step 2.6.
   - Options:
     - (a) GitHub's Linux, Windows and macOS runners stand in; nothing waits.
     - (b) The scripts on runners; the two browser rows wait for the machines (step 2.6 waits too, unless you let it
       start without them).
     - (c) Everything waits, by the draft's letter.
   - Recommendation: (a). The macOS part follows the netcode proof test's precedent; the Windows part is new.
5. **TOMBSTONE: does the tombstone workaround run from the start, or only after a failure?**
   - Why it matters: Cloudflare says "The latest deployment for a branch cannot be deleted", while its API's force
     option says aliased preview deployments can be. If plain deletion fails, the draft tries the tombstone "in the same
     spike" (:1371).
   - Options:
     - (a) Alongside from the start: two more test pull requests, (a') and (b'), counted only if (a) or (b) fails;
       about 6 more approvals; saves a calendar day if plain deletion fails.
     - (b) Only after a failure: one more day.
   - Recommendation: (a).
6. **ACCESS-CARD: if switching on the login wall asks for a Zero Trust plan and payment details, what do you do?**
   - Why it matters: Cloudflare's Zero Trust sign-up asks for payment details even on its Free plan ("this step is
     still needed but you will not be charged"). No page says whether the Pages switch starts that sign-up.
   - Options:
     - (a) Choose the Zero Trust Free plan and enter the details. The wall is needed from slice 3 anyway, under your
       PREVIEW-WALL (a).
     - (b) Don't. PM-0006-8 and the checklist's steps 5 to 8 are then recorded as not run, OQ-0006-4 stays open until
       slice 3, and the check ends without them, which needs your word that it may.
   - Recommendation: (a). It is your card and your call: if the screen asks for anything you did not expect (a charge,
     a paid plan), stop.
7. **TR-END: what happens to the test repository and the test project afterwards?**
   - Options:
     - (a) The repository archived (read-only, still public, its runs and logs kept as evidence); the project deleted
       once the report is merged.
     - (b) Both deleted. The report keeps the texts, but deleting a repository cannot be undone.
     - (c) Both kept for step 2.6. Step 2.6 builds in this repository with a project named for the game, so nothing
       would reuse them.
   - Recommendation: (a).

## For Mustafa
1. Answer the seven decisions above.
2. Give or refuse your go on this plan, and your yes to Part 7's texts: the four workflows and the two token-test
   variants.
3. Allow me to push these branches to the test repository, and only there: `setup`, `fix/*` for review fixes,
   `cs/pd-one`, `cs/pd-three`, `cs/pd-open`, `cs/pd-off`, `cs/pdt-one`, `cs/pdt-three`, `cs/b9-edit` and `cs/b9-prod`.
   My standing branch rule names only this repository's branch.
4. SR-READ (Part 3, A3 step 3): tell me whether the Claude app's page lists "Administration" or "Deployments" with
   write access. Names only.
5. Tell me if your Cloudflare account holds anything besides the domain: another Pages project, a Worker, or KV, R2 or
   D1 data (Part 4).
6. Then do the setup (Part 3 A) when it suits you, and tell me "repository ready" and "setup done" as the steps say.
