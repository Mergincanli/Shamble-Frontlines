# Plan: the preview hosting check (SPIKE_PLAN entry 6), revision 1 (DRAFT, being written)

_For Mustafa and the Advisor. A plan only: nothing is built, no secret is made and nothing outside this checkpoint is
edited before Mustafa's go. Branch `claude/optimistic-albattani-mdoonz` (stands in for `cs/preview-hosting-check`),
base main 4460ef5. Line numbers are main's at 4460ef5. Written 2026-11-15 by the second builder chat (main model Opus
5.5; the advisor tool answered by Fable 5.2)._

[[SUMMARY: written last]]

## Part 0. How to read this plan
- **The test repository (TR):** the new public GitHub repository that holds the test site, the throwaway workflows,
  the deploy token and the test pull requests: ADR 0006's "TR" (:1280-1281, "Mustafa names it and adds its `preview`
  environment and token"). Proposed name: `Mergincanli/shamble-preview-check` (decision TR-WHERE).
- **The test project (CF):** a new Cloudflare Pages project in Mustafa's account, proposed name
  `shamble-preview-check`, address `https://shamble-preview-check.pages.dev` (Cloudflare may add a suffix if the name
  is taken; Part 3 says what to do then).
- **A deploy:** one upload of the built test site to the test project, on a named "branch" of the project. The branch
  `main` is production; every other branch is a preview with a stable link `https://<branch>.shamble-preview-check.pages.dev`,
  and every single upload also gets its own one-off link `https://<8 characters>.shamble-preview-check.pages.dev`.
- **The three links** (the hosting checklist's): production, the preview built with the isolation flag off (Pages
  branch `iso-off`), the preview built with it on (Pages branch `iso-on`).
- **An approval:** Mustafa's click on GitHub's "Review deployments", then "Approve and deploy", for a run that waits for
  the `preview` environment (decision REVIEWER).
- **A probe:** a script that fetches a link the way a stranger would (no login, no cookies) and records what came back.
- **ML, MAC, MP:** the minimum laptop, the Mac with Safari, Mustafa's PC in Brave (ADR 0006's key, :1306-1310).
- **The draft:** ADR 0006 (Proposed) unless another is named; "the infra draft as proposed" is Mustafa's answer to
  PREVIEW-START (his task, 2026-11-15): the check runs on the draft as it stands, interim positions included.

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
| PM-0006-2 | the script runs on ML ("a header check, not a hardware result", :1367) → on GitHub's Linux runner, as the deploy workflow's last job, right after each deploy | as drafted: one test site deployed three ways (production; a preview with the flag off; a preview with it on), each read on its stable branch link and on its one-off link; the page lists every resource it and its worker loaded (Resource Timing in both); a cookie-less fetch compares every response with expected sets written by hand into the script from GLOSSARY §5; the bites as drafted (COEP removed from the worker's rule in the flag-on build; a `noindex` rule added to production). Added: the script also fetches every file in the build's file list, so no file escapes the check; and a test-only header, `X-Check-Rule`, on the two host rules answers R5-PREV-OQ6 | the machine: MACHINES; the `X-Check-Rule` header changes no mark: for the advisor agent to rule |
| PM-0006-3 | ML: Chrome, Edge, Firefox; MAC: Safari; MP: Brave → GitHub's Windows runner (Chrome, Edge, Firefox); GitHub's macOS runners (Safari, on two images); you in Brave | as drafted: WebdriverIO opens each link and reads what the page and its worker print, 3 loads per browser and link; the bite as drafted (PM-0006-2's missing COEP must give false) | the machines: MACHINES |
| PM-0006-4 | ML: Chrome → GitHub's Windows runner: Chrome | as drafted: first visit, tab closed, second visit; the bundle's `transferSize` read on the second; 3 repeats, each from a fresh profile; the bite as drafted (`no-cache`), on a preview branch of its own | the machine: MACHINES |
| PM-0006-5 | CF; ML → CF; GitHub's Linux runner (in the build job, before each deploy) | as drafted: every upload's largest file; the bite as drafted (a planted 26 MiB file, flagged, and what Wrangler or the host does with it written down) | the machine: MACHINES |
| PM-0006-6 | CF, TR; the probe from ML → CF, TR; the probe on GitHub's Linux runner | as drafted: cases (a) `cs/pd-one`, (b) `cs/pd-three`, (c) the open control, (d) the bite with the close workflow switched off; the throwaway close workflow deletes every deployment of the branch with the force option; a cookie-less probe every hour from the close to 24 h after; the access policy off; the tombstone if (a) or (b) fails. Changed: (1) the hourly probe runs on GitHub's schedule, and the session also starts one probe itself just after the 24 h mark, so the reading never rests on the schedule alone (GitHub may delay or skip a scheduled run); (2) under TOMBSTONE (a), the tombstone runs from the start on two more cases, (a') `cs/pdt-one` and (b') `cs/pdt-three`, built like (a) and (b), and counts only if (a) or (b) fails | the machine: MACHINES; the tombstone's timing: decision TOMBSTONE; the session's own 24 h run: an addition, for the advisor agent to rule |
| PM-0006-7 | CF; MP → as drafted | as drafted: your two readings around 10 uploads by the throwaway deploy workflow (one run that uploads 10 times) | none |
| PM-0006-8 | CF; the probe from ML → CF; the probe on GitHub's Linux runner | as drafted: you switch the access policy on; sweeps 10 minutes and 1 hour later, the 1-hour sweep counts; the control as drafted. No custom domain (CF has none) | the machine: MACHINES |
| PM-0006-9 | MP, Brave on Windows → as drafted | as drafted: the hosting checklist's 8 steps; steps 5 to 8 only after PM-0006-6's 24 h reading | none |
| PM-0006-10 | TR → as drafted | as drafted: the deploy job sets its environment's `url` to the preview's stable link, and the pull request's page is read after a deploy. The picture: the session takes it from the public page while logged out, and you say whether your own logged-in page shows the link too | the picture's route: an addition, for the advisor agent to rule |
| PM-0006-32 | TR, CF; MP → as drafted | as drafted: the rewritten deploy job; the job that names `production`; the reported production deploy with the preview token; each run's log and the environment's deployment history pasted into the results log; the control and the bite as drafted. Added: ADR 0002's PM-0002-66 bite (the `production` rule removed: the job must then print "present"), since PM-0002-66 names the test repository for these tests (0002:1332) and the draft calls them "the same test" (:1222). Under TOKENS (a) the reported production deploy with the preview token is the test project's ordinary production deploy | the extra bite: an addition, for the advisor agent to rule; one token: decision TOKENS |

## Part 2. What I build (your item 2)
[[PART 2]]

## Part 3. What you do yourself, step by step (your item 3)
[[PART 3]]

## Part 4. The security side in plain words (your item 4)
[[PART 4]]

## Part 5. The calendar (your item 5)
[[PART 5]]

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
[[PART 7]]

## Part 8. How the check is run, reviewed and reported
[[PART 8]]

## Part 9. Risks
[[PART 9]]

## Decisions needed (your item 8)
[[DECISIONS]]

## For Mustafa
[[FOR MUSTAFA]]
