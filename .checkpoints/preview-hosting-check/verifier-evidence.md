# Verifier notes: evidence of the preview hosting check (plan step 9)

Started 2026-11-24. Branch claude/optimistic-albattani-mdoonz. Edits nothing but this file.

## Plan
1. Quote ADR 0006 marks PM-0006-1..10, -32 (lines 1369-1379), the hosting checklist (1439-1457), "When a mark fails"
   (1404-1418). Done: read.
2. Read plan Part 1.1 (115-130) and Part 9 (1334-1361; Risk 13 at 1358-1361); Decisions taken in the checkpoint
   (1089+). Done: read.
3. Read results-log summaries (sitting 1 in short :483, sitting 3 :708, -32 clause by clause :735, -6 read :830,
   -8 read :962, revocation :1019) and the run sections around them.
4. Spot-checks with jq on .gate/preview-hosting-check/day1 and day2 (items 1-5 of the task).
5. Per-mark verdicts; answers a-f; DONE.

## ADR anchors (docs/adr/0006-infrastructure-operations.md)
- PM-0006-1 :1369 ... PM-0006-10 :1378, PM-0006-32 :1379; When a mark fails :1404-1418; checklist :1439-1457.
- ADR 0002 :1507-1512 step 2.6: "After ADR 0006's preview hosting check, with B9's interim position in place".
- DECISIONS.md:1449 (his words): "a draft is accepted only when its marks are met. If a mark fails, the code that rests
  on it is redone." ... "Step 2.6, the previews, follows the preview hosting check as planned."

## Findings (one line each, as found)
- SC1 probe8-24h (day2/probe8-24h/probe-results.json): PASS. jq group_by(.case): (a) 2 gone 404 24.4h; (a') 3 gone
  (200 tombstone x2, 404) 24.4h; (b) 4 gone 404 24.1h; (b') 5 gone 24.1h; (c) 2 serves 200 (pr open); (d) 2 serves
  24.39h; by-hand 19 serves; production 4 serves (own address expected "serves"; 3 one-offs expected null); rehearsal
  2 gone 27.99h. `[.rows[]|select(.pr.state=="closed")]|all(.hours_since_close>=24)` -> 18 rows, true (min 24.1).
  The 14 rows of (a),(a'),(b),(b') all "gone". unexpected 0, no reading 0, wall 0, max tries 1. Log 0_probe.txt:
  "Records found: 21", image ubuntu-24.04 20261116.133.1, "probed 43 links for deletion; unexpected: 0; no reading: 0".
  Log's "18 closed-case rows" = 14 + (d) 2 + rehearsal 2 (wording: (d) and rehearsal included; fine).
  Note: a 200 counts "gone" only by the script's marker test (probe.mjs:44-45); the JSON keeps no body/marker, so the
  tombstone rows' "no marker" rests on the reviewed script, not on stored bodies.
- SC2 wall sweeps (day2/probe26-wall10, probe27-wall60 probe-results.json): PASS. Each 43 rows; 42 outcome "wall",
  status 302, `location` host shamble-preview-check-pages.cloudflareaccess.com x42; the 1 non-wall row is production's
  own address https://shamble-preview-check.pages.dev/ 200 "serves" (expected "serves"); production's 3 one-offs wall
  (expected null). unexpected 0, no reading 0. Stable-link walls on branches with pr==null or open (probe.mjs:102 rule):
  6 = bite-coep, bite-nocache, cs/pd-open, iso-off, iso-on, uploads. Per branch (probe27): every link of each walled
  (uploads 11/11, cs/pd-open 2/2, iso-off 2/2, iso-on 2/2, bite-* 2/2); (d) cs/pd-off 2/2 walled (leftovers). Only
  cs/pd-open has an open PR (#5); the other five have pr==null (no PR). probed_at 20:26:39Z and 21:16:27Z.
- Timing note (PM-0006-8 method): the switch-on time is only bounded (19:12:54-20:14:13 UTC), so the "10 minutes" sweep
  was somewhere between 12 min and 74 min after the switch, and the counted "1 hour" sweep between 62 min and 124 min.
  The counted reading is >= 1 h (meets the purpose); the 10-minute reading is not a 10-minute reading. Low; the log
  states it as lower bounds (results-log:968-969), which is honest.
- SC3 control probe25-control: PASS. 43 rows, wall 0, no `location` field set, unexpected 0; all 25 rows with no PR or
  an open PR serve (19 by-hand + (c) 2 + production 4); (d) 2 serves; (a),(a'),(b),(b') and rehearsal gone as
  expected. Run with purpose "deletion" (same 43 links; only the expectation labels differ). probe9-control the same.
- SC4 headers (day1/round-a/headers.json, uploads/headers.json, restore-production, bites, bite-production): PASS.
  round-a summary {"records":3,"pass":5,"fail":1,"no_reading":0}; per link: iso-off x2 pass (noindex, no COOP/COEP),
  iso-on x2 pass (noindex, COOP same-origin, COEP require-corp on all 7 paths), production stable pass (robots null,
  X-Check-Rule project-host), production one-off 0354374e FAIL "production carries X-Robots-Tag noindex" on all 7
  paths; all 200, tries 1; .wasm application/wasm x6; content file "public, max-age=31536000, immutable", others
  "public, max-age=0, must-revalidate". uploads summary {"records":10,"pass":20,"fail":0,"no_reading":0}, all
  version-host + noindex. `jq -s` over both: 24 preview readings, 24 pass, 15 distinct links (matches log :483-484).
  restore-production: stable pass, one-off de1c3e5e fail on noindex x7 (Risk 13 again). bites: bite-coep only
  /worker.js COEP "" on both links; bite-nocache only content Cache-Control no-cache on both. bite-production: both
  links fail on noindex only (the one-off would fail anyway by Risk 13; the bite is shown by the stable link).
  Paths read: / /manifest.json /page.js /page.css /worker.js /tiny.wasm /content.*.bin (index.html via "/", 404.html
  not fetched: L3).
- SC5 browsers (day1/browsers/win/browser-results.json): PASS. ran_at 17:59:08.392Z, image win25-vs2026
  20261103.298.1, WebdriverIO 10.1.0, summary rows 51, unexpected 0, no_reading 0. group_by([mark,browser,link]):
  Chrome 156.0.8078.80, Edge 156.0.4314.58, Firefox 159.0.1, 3 loads each: production false/false, iso-off false/false,
  iso-on true/true, bite-nocache false/false "as expected"; bite-coep "true/not reported (worker error: its script did
  not load)" "bite caught". PM-0006-4 Chrome: production transferSize 0 x3, bite-nocache 300 x3. Markers 2162db55 on
  all PM-0006-3 rows. Mac: rows 15, no_reading 14 ("There is already a Safaridriver instance running on port 49178!"),
  1 production load false/false. All as log :448-460 states.
- F-3a (PM-0006-3 bite, Low): the ADR's bite reads "PM-0006-2's missing COEP gives `false` on the flag-on preview"
  (:1371). Observed in all three browsers: page `true`, worker no value (script refused). The bite does make the mark
  fail (worker not true), but not in the ADR's form "false". Recorded as TE-3's observed form (test-engineer review),
  not a departure anyone decided; the report should say so in those words.
- F-3b (PM-0006-3 Brave count, Medium): the ADR's count is "3 loads per browser and deployment" and names "MP: Brave
  (the hosting checklist, steps 1-3)" (:1371). Brave gave 1 load per deployment in steps 1-3 (2026-11-18 17:12,
  results-log:350-365). Counting every later Brave load with values: iso-off 4 (11-18, 11-19 14:09 no wall, 11-24
  screenshot 1, 11-24 step 8), iso-on 2 (11-18, 11-24), production 2 with values (11-18, 11-24; the private-window
  open at step 7 reported no values). So Brave is not 3 of 3 per deployment as written. The ADR is internally loose
  (the checklist opens each link once), and plan :95 puts "3 loads of 3" on the scripted browsers; no one recorded it
  as a departure. Report must not claim 3 of 3 for Brave; every Brave reading is as expected.
- F-5 (PM-0006-5 coverage, Medium): ADR: "the largest file in each deployed build under 25 MiB ... on every deployment
  of the check" (:1373). SIZES lines are in the log for round-a (3), uploads (one build, 10 uploads) and bites
  (results-log:323-325, 399, 424-426); the downloaded bites logs also show bite-coep and bite-nocache at 65536. Not in
  the log or the downloaded data: sitting 2's ten builds, bite-production, restore-production, the two tombstones
  (TE-11's procedure, log :81-82, names the tombstone's size), and `revoked`. check-sizes.mjs never exits non-zero on a
  flag (only exit 2 on a missing folder), so a green build is no evidence. Indirect evidence: Wrangler refuses any file
  over 25 MiB before upload (bites log), so every successful upload was under the limit. Verdict: open until those
  SIZES lines are read from the run logs (or the report states the Wrangler inference as the basis).
- PM-0006-10 records: day1/pr-page/pr3.json holds "Mergincanli deployed to preview — with GitHub Actions", "View
  deployment", "This branch was successfully deployed", link https://cs-pd-one.shamble-preview-check.pages.dev/;
  logged_out key set. Logged-in: his screenshot of #8 (results-log:546-551). Met.
- PM-0006-32 record: day1/sitting3/run18.json holds "Branch "refs/pull/10/merge" is not allowed to deploy to production
  due to environment protection rules." Matches log :682-683.
- F-8a (PM-0006-8 "open branches", question a): ADR :1376 "on every preview URL of at least 2 open branches (aliases
  and hash URLs)". The ADR does not define "open branch"; its control says "every open preview URL"; PM-0006-6 uses
  "an open pull request" for (c). In probe27 only cs/pd-open (#5) has an open PR (closed 21:17:35Z, 68 s after the
  sweep); cs/b9-edit and cs/b9-prod had open PRs but no uploads (no preview URL). The five by-hand ones (iso-off,
  iso-on, uploads, bite-coep, bite-nocache) are Pages preview branches only: not git branches in TR (remote list,
  results-log:776-777) and no PR. Reading 1 (open = a preview branch whose links are live): 6, met. Reading 2 (open =
  with an open PR): 1, not met as written. The ADR is silent; the evidence covers every one of the project's 39
  preview links (a superset of the mark's purpose, "covers every preview"). Needs a ruling (advisor / draft author);
  the report must state the count as 1 PR branch + 5 by-hand Pages branches, not "6 open branches" bare.
- F-8b (Low): production's three one-off links are walled (probe26/27). The mark judges "the production URL" (its own
  address serves), so met; but the row's title "covers every preview of the project and not production" is only true
  for production's own address. Recorded in the log for OQ-0006-4 (:907, :969); fine as long as the report says it.
- F-6 (PM-0006-6, question d): pass condition met as written (24.1-24.4 h, all 14 rows of (a),(a'),(b),(b') gone; (c)
  and (d) serving; wall off shown by (c)). Method departure: "A cookie-less probe reads every URL every hour from the
  close to 24 h after" (:1374) not achieved: 3 scheduled runs in 28 slots (checked: 15:23 11-18 .. 18:23 11-19 = 28),
  by-hand probes #2, #3, #7, #8 stood in; gaps 4.1-9.4 h and 9.7-18.7 h. "The hourly probes record when each URL
  stopped serving": recorded anyway by probe #2 (3.6-21.5 min after the closes) and the close logs (deletion 14-29 s
  after close). The shortfall was foreseen by plan Risk 4 ("GitHub's schedule runs late or skips an hour: my own probe
  just after 24 hours; every gap is recorded"), part of the plan Mustafa approved (his go 2026-11-15). Verdict: met
  with a recorded departure in method (the hourly series), route foreseen by plan Risk 4 in the plan he approved; not
  a separate decision of his on this shortfall.
- Extra checks (advisor consultation): probes 2-7 (day1/probe-2..4, day2/probe5..7) each 43 rows, unexpected 0, (c) 2
  serves and (d) 2 serves at every one, closed cases gone at every one: "(c) serves at every probe" holds (probes 2-9,
  25). Close logs (day1/closes/{a,ap,b,bp}/0_cleanup.txt) "Uploads found" 1/2/3/4 and Delete/Kept ids equal probe8's
  deployment_ids per branch: "every deployment it ever had" is covered.
- Commit times (git log on results-log.md): header f37cb7a 2026-11-17T20:38:22Z, before round-a (2026-11-18 16:26:57);
  re-check 87547f6 2026-11-19T13:44:30Z (found Brave missing), before the 24 h reading (18:56); Brave line 3172188
  2026-11-19T21:05:14Z, about 2 h after the 24 h reading.
- "Open branch" (question a): ADR has no definition (GLOSSARY "The access wall" :618-630 none; OQ-0006-4 :876-887
  none). But ADR :1360 gives the count's reason: "PM-0006-8's at least 2 open branches (with one, a wall set on that
  branch alone would pass too)". That reason is about distinct preview branches, not pull requests, so the five
  Pages-only branches serve it. probe.mjs:102 builds in the looser reading (pr==null counts as open).
- PM-0006-32 pictures day1/sitting3/run17-a1.png ("Mergincanli approved", preview, Success), -a2.png (no Deployment
  protection rules section, Success), -a3.png ("Mergincanli rejected", preview, Failure, annotation "The deployment was
  rejected or didn't satisfy other protection rules."): match log :725-729. The "secret: present" lines live only in
  run logs not in .gate: not verified from data (session's reading).
- Stale ADR line numbers (Low): ADR 0006 changed at 3747bb3 (2026-11-17T20:59Z), after the plan; lines moved by +3.
  Log cites :1401 (now :1404), :1409-1414 (now :1411-1418), :1444-1447 (steps 1-3 now :1447-1450), :1450-1457 (steps
  5-8 now :1453-1460); plan :101 ":1436-1457", :121 ":1401", ":1409-1414", :385 ":1444-1447", :1302 ":1401-1415".
- PM-0006-6 denominator (Low): log says 3 scheduled runs "in the 28 slots from 2026-11-18 15:23 to 2026-11-19 18:23"
  (:837-838); 4 of those slots (15:23-18:23) fall before the first close (18:32:51). From the close to 24 h after
  (19:23 11-18 .. 18:23 11-19): 24 slots, the same 3 runs. State both.
- Gate: `node tools/gate.mjs --no-smoke` stops at "Cannot find module 'acorn'" (tools/ deps not installed); not
  installed by me (writes outside .gate/). The change-set's diff is checkpoint/notes files only (no legacy code).

## Per-mark verdicts (ADR 0006 words at the cited line)
- PM-0006-1 (:1369): met with recorded departures: machines -> GitHub runner images, browser versions per row at run
  time (MACHINES (a), Mustafa 2026-11-15). Timing clause for one line not met as written: the Brave version was not in
  the header before the first run nor at the re-check before the 24 h reading; filled 2026-11-19 21:02:51 UTC (log
  :840-860), after the reading. No one decided that gap. Day 1's Brave steps ran on a version recorded only a day
  later (unchanged by 2026-11-24, :931).
- PM-0006-2 (:1370): NOT MET AS WRITTEN. "previews `X-Robots-Tag: noindex`, production none" on "both URL forms":
  production's one-off links 0354374e and de1c3e5e carry noindex on all 7 paths (round-a, restore-production).
  Everything else met; both bites caught; R5-PREV-OQ6 answered (X-Check-Rule version-host on branch links).
- PM-0006-3 (:1371): met on Chrome, Edge, Firefox (3 of 3, 45 rows; MACHINES). Safari: not run, by Mustafa's decision
  (2026-11-18 18:21:30 UTC); the ADR row still names it. Brave: 1 load per deployment at steps 1-3 (not "3 loads per
  browser and deployment"); all readings as expected; gap not decided by anyone (F-3b). Bite seen as worker refused,
  not "false" (F-3a).
- PM-0006-4 (:1372): met with recorded departure (MACHINES): immutable header; transferSize 0 x3 fresh profiles; bite
  300 x3 plus the header half.
- PM-0006-5 (:1373): open: met for round-a, uploads, bites (bite flagged 27262976 and refused by Wrangler); SIZES not
  recorded for sitting 2's ten builds, bite-production, restore-production, two tombstones, revoked (F-5).
- PM-0006-6 (:1374): met with a recorded departure in method (F-6).
- PM-0006-7 (:1375): reported mark; record complete in its letter ("no count shown" before and after on the three
  pages; ten uploads listed); its question (do direct uploads count) unanswered: goes to the draft's author.
- PM-0006-8 (:1376): met on the ADR's stated reason (:1360), conditional on "open branch" = a live preview branch
  (1 with an open PR + 5 Pages-only); if "open" needs a PR, 1 < 2, not met as written. Times are lower bounds.
- PM-0006-9 (:1377): open: steps 1-3 "achieved" (2026-11-18 17:21:40), steps 5-8 "achieved" (2026-11-24 21:56:20);
  step 4 has no "achieved", no "not yet", no two numbers. 7 of 8.
- PM-0006-10 (:1378): met (yes; logged out pr3.json/png; logged in his screenshot of #8).
- PM-0006-32 (:1379): met: 2 of 2, control, bite, plan's extra PM-0002-66 bite; reported: preview token not refused
  for production (round-a 0354374e); merge ref recorded; revocation recorded (deleted 22:05:59-22:14:34 2026-11-24;
  revoked run 10000/9109, no record). Small departure: "the environment's deployment history" read from PR page
  timelines (repository deployments page 404 logged out).

## Answers a-f
- a: ADR silent on "open"; its reason (:1360) favours distinct preview branches; verdict depends on the reading; state
  1 PR branch + 5 Pages-only (not git branches in TR); log's flat "6 open branches" overstates.
- b: :1404 "A spike passes when every mark that applies passes"; :1411-1416 "A failed mark goes back to the draft:
  infra-engineer revises the decision part the row names by the draft's own Rollback and "Revisit when" routes (a header
  or PREV-DELETE fail on Pages: Workers static assets weighed again; ...), and the revised part returns to the advisor
  for review and to Mustafa and the Advisor with the measured numbers"; :1416-1417 "A mark's number changes only in the
  draft, with its reason, never inside a spike". The paragraph does not name step 2.6. The wait comes from plan 1.1
  (:121) and Risk 13 (:1358-1361) "the check's end and step 2.6 wait for the revised draft", ADR 0002 :1507 "After ADR
  0006's preview hosting check", DECISIONS :1449 "a draft is accepted only when its marks are met" / "Step 2.6, the
  previews, follows the preview hosting check as planned". Not mine to decide.
- c: "Step 4: no verdict from him (neither "achieved" nor "not yet" nor the two numbers the step asks for; his three
  screenshots show no count)"; PM-0006-9 open at 7 of 8; never "achieved".
- d: met with a recorded departure in method; route = plan Risk 4 in the plan he approved; not a decision of his on
  this shortfall.
- e: "re-checked before the 24 h reading (2026-11-19 13:41-13:43 UTC); the re-check found the Brave version missing;
  it was recorded at 21:02:51 UTC, about 2 h after the reading": the timing clause not met for that line.
- f: quote his words; "Safari leaves the check" is the session's reading; ADR row unchanged (:1416-1417 only the draft
  changes a mark); PM-0006-3 met on Chrome, Edge, Firefox; Safari not run by his decision; the draft's author told.

## My verdict on the evidence: PASS WITH NOTES (the evidence holds against the data). The spike itself has NOT passed
by :1404: PM-0006-2 not met as written; PM-0006-9 and PM-0006-5 open; PM-0006-8 needs a ruling on "open".

## Addendum after the second consultation (supersedes the lines above where they differ)
- F-3b downgraded to Low: ADR :1440 "Beside it, the measurable marks: ... isolation in the four other browsers
  (PM-0006-3)" puts the scripted 3-load count on the four other browsers; Brave's reading is the checklist (steps 1-3,
  one open each, row :1371 "MP: Brave (the hosting checklist, steps 1–3)"). Wording note only: the report must say
  Brave gave 1 load per link (all as expected), not 3 of 3. :1440 also names Safari among "the four other browsers".
- F-5 tombstones: closes/ap and bp logs show "Uploaded 1 files" / "Uploaded 0 files (1 already uploaded)" and record
  artifacts of 390/391 bytes (the record, not the page); no SIZES line runs in close.yml. Tombstone size not recorded.
- Final labels (one each): PM-0006-1 met with a recorded departure (MACHINES (a), Mustafa 2026-11-15; caveat: the
  Brave line late, an undecided timing gap). PM-0006-2 not met as written. PM-0006-3 met with a recorded departure
  (Safari not run, Mustafa 2026-11-18 18:21:30 UTC; MACHINES). PM-0006-4 met with a recorded departure (MACHINES).
  PM-0006-5 open. PM-0006-6 met with a recorded departure (method; plan Risk 4). PM-0006-7 open (no count readable on
  the three pages; the step-4 "two numbers recorded" do not exist; question unanswered). PM-0006-8 met (caveat: on the
  reading of "open branch" the ADR's :1360 reason supports; 1 PR branch + 5 Pages-only). PM-0006-9 open (7 of 8).
  PM-0006-10 met. PM-0006-32 met.
- ADR pass words and results-log anchors per mark:
  - PM-0006-1 :1369 "a results-log header written before the first run" / "checked again before PM-0006-6's 24 h
    reading"; log :6-71, :770-789, :859-860, :940.
  - PM-0006-2 :1370 "100 % of responses carry their expected set ... previews `X-Robots-Tag: noindex`, production
    none"; log :326-340, :437-441, :468-482, :483-486.
  - PM-0006-3 :1371 "`true` on the flag-on preview and `false` on the flag-off preview and on production, read in the
    page and in its worker: 3 of 3 loads per browser and deployment"; log :350-365, :379-381, :444-460.
  - PM-0006-4 :1372 "its `Cache-Control` contains `immutable`; on a second visit ... transfers 0 bytes"; log :332-333,
    :437-441, :452-453.
  - PM-0006-5 :1373 "the largest file in each deployed build under 25 MiB = 26,214,400 bytes ... on every deployment
    of the check"; log :323-325, :399, :424-432.
  - PM-0006-6 :1374 "24 h after each test pull request closes ... none of its URLs ... answers HTTP 200 with that
    build's marker" / "A cookie-less probe reads every URL every hour from the close to 24 h after"; log :539-591,
    :621-627, :715-721, :753-769, :816-839.
  - PM-0006-7 :1375 "reported, no pass number: the project's monthly build count before and after 10 direct uploads"
    / "passes when both are recorded"; checklist step 4 :1451-1452 "Expected: two numbers recorded"; log :369-421.
  - PM-0006-8 :1376 "on every preview URL of at least 2 open branches (aliases and hash URLs) and on PM-0006-6's
    leftover URLs, a cookie-less client gets no build"; :1360 the reason; log :881-886, :903-909, :941-969.
  - PM-0006-9 :1377 "every step of the hosting checklist below "achieved", 0 "not yet""; :1443 "or the numbers it
    asks for"; log :379-381, :395-396, :420-421, :910-939, :973-979.
  - PM-0006-10 :1378 "recorded: yes or no, with a screenshot of the page"; log :514-521, :546-551.
  - PM-0006-32 :1379 "2 of 2 ..."; log :604-750, :993-1023.
- The jq commands, literal (run from /home/user/Shamble-Frontlines/.gate/preview-hosting-check):
  - SC1: `jq -r '.rows|group_by(.case)[]|"\(.[0].case) n=\(length) \([.[].outcome]|group_by(.)|map("\(.[0]):\(length)")|join(",")) minh=\([.[].hours_since_close]|min)"' day2/probe8-24h/probe-results.json`;
    `jq '[.rows[]|select(.pr.state=="closed")]|{n:length, all24: all(.hours_since_close>=24)}' day2/probe8-24h/probe-results.json`;
    `jq '[.rows[]|select(.case|test("^\\((a|b)\\)|^\\((a|b).\\) tomb"))]|{n:length, allgone: all(.outcome=="gone")}' day2/probe8-24h/probe-results.json`;
    `jq '{unexpected:[.rows[]|select(.as_expected==false)]|length, wall:[.rows[]|select(.outcome=="wall")]|length, outcomes:([.rows[].outcome]|unique), maxtries:([.rows[].tries]|max)}' day2/probe8-24h/probe-results.json`.
  - SC2: same group_by on day2/probe26-wall10 and day2/probe27-wall60; `jq -c '.rows[]|select(.outcome!="wall")|{case,form,url,status,outcome,expected}'`;
    `jq -r '[.rows[]|select(.outcome=="wall")|.location]|map(capture("^https://(?<h>[^/]+)").h)|group_by(.)|map("\(.[0]) x\(length)")|.[]'`;
    `jq -r '[.rows[]|select(.form=="stable" and .case!="production" and .outcome=="wall" and (.pr==null or .pr.state=="open"))|.branch]|unique'`;
    `jq -r '.rows|map(select(.case!="production"))|group_by(.branch)[]|"\(.[0].branch) pr=\(.[0].pr.number // "none")/\(.[0].pr.state // "-") links=\(length) wall=\([.[]|select(.outcome=="wall")]|length)"' day2/probe27-wall60/probe-results.json`.
  - SC3: `jq '{n:(.rows|length), wall:[.rows[]|select(.outcome=="wall")]|length, unexpected:[.rows[]|select(.as_expected==false)]|length, open_or_byhand_all_serve: ([.rows[]|select(.pr==null or .pr.state=="open")]|all(.outcome=="serves"))}' day2/probe25-control/probe-results.json`.
  - SC4: `jq -c '.summary' day1/round-a/headers.json day1/uploads/headers.json`;
    `jq -r '.records[]|. as $r|.forms[]|"\($r.branch) \(.form) \(.base) \(.result) xcr=\(.xCheckRule) robots=\([.responses[].headers["x-robots-tag"]]|unique) coop=\([.responses[].headers["cross-origin-opener-policy"]]|unique) coep=\([.responses[].headers["cross-origin-embedder-policy"]]|unique) fails=\([.responses[].failures[]]|unique)"' day1/round-a/headers.json`;
    `jq -s '[.[].records[]|select(.mode!="production")|.forms[]]|{n:length, pass:[.[]|select(.result=="pass")]|length, distinct_links:([.[].base]|unique|length)}' day1/round-a/headers.json day1/uploads/headers.json`.
  - SC5: `jq 'del(.rows)' day1/browsers/win/browser-results.json`;
    `jq -r '.rows|group_by([.mark,.browser,.link])[]|"\(.[0].mark) \(.[0].browser) \(.[0].version) \(.[0].link) loads=\(length) seen=\([.[]|(.seen // (.transferSize|tostring))]|unique) outcome=\([.[].outcome]|unique)"' day1/browsers/win/browser-results.json`.
- Not verified: the "secret: present" log lines (logs not in .gate); his screenshots and words (transcribed by the
  session, images kept out); run metadata (schedule events, run lists, approval times) from the GitHub tool; sitting
  2's ten headers files and SIZES lines; whether he quit Brave before step 5.
DONE
