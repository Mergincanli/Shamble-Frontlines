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

[[TABLE 1.1: filled below]]

## Part 2. What I build (your item 2)
[[PART 2]]

## Part 3. What you do yourself, step by step (your item 3)
[[PART 3]]

## Part 4. The security side in plain words (your item 4)
[[PART 4]]

## Part 5. The calendar (your item 5)
[[PART 5]]

## Part 6. The laptop and the Mac (your item 6)
[[PART 6]]

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
