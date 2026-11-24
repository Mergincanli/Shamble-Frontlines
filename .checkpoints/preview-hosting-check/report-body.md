# Advisor report: the preview hosting check (2026-11-24, branch `claude/optimistic-albattani-mdoonz` standing in for `cs/preview-hosting-check`, base main 28254a7)

Pull request: PRNUMBER. Risk tier 1: it adds this report and records Mustafa's words in `docs/DECISIONS.md`; it
touches no secret, no CI file and no infrastructure. The check itself (risk tier 2: a deploy token, CI and hosting) ran
in his separate test repository, Mergincanli/shamble-preview-check. Written by the builder chat Mustafa calls Builder
V2 (lane 2 of the build; session_019YNMUSYpJGi1xxCjKgr4e5; main model Opus 5.5, its advisor tool answered by Fable 5.2,
both read from the session's records on 2026-11-15).

## Goal
ADR 0006's spike 1 (SPIKE_PLAN entry 6): before every pull request gets its own web link (foundation step 2.6), show
on a throwaway Cloudflare Pages project that the host sends the game's files the way the drafts need (headers,
isolation, caching, file sizes), that a closed pull request's link really disappears within 24 hours, that a login
wall covers the previews and not production, and that a pull request cannot reach the deploy token or production. Its
pass marks are PM-0006-1 to PM-0006-10 and PM-0006-32 (ADR 0006, the marks table and the hosting checklist). His task
of 2026-11-14 (PREVIEW-START: "it starts now, on the infra draft as proposed") and his go of 2026-11-15 stand in
`docs/DECISIONS.md`.

## Outcome
The runs are finished and every reading of the last day came out as expected: 24 hours after the closes every closed
test's links were gone (and still were six days later), the login wall stopped a stranger on every preview link while
production stayed open, and the deleted token was refused at Cloudflare's sign-in with nothing uploaded. But **the
check has not passed by the draft's own rule** ("A spike passes when every mark that applies passes", ADR 0006 :1404):
the header mark is not met as written (production's one-off links carry the "keep out of search engines" header), and
two marks are open (the build count, and his verdict on checklist step 4, which depends on it). Five marks are met with
departures that change no reading. Three decisions of his follow; this pull request waits for them and for his go.

| What it checks (the mark) | The draft's own words, shortened (ADR 0006) | Verdict |
|---|---|---|
| The setup is written down first (PM-0006-1) | "recorded: ... each browser's and WebdriverIO's versions"; "checked again before PM-0006-6's 24 h reading" (:1369) | Met with departures: GitHub's machines stand in for the laptop and the Mac (his MACHINES (a)); his Brave version reached the record about 2 hours after the 24-hour reading |
| The headers as built, on both kinds of link (PM-0006-2) | "100 % of responses carry their expected set: ... previews `X-Robots-Tag: noindex`, production none" (:1370) | **Not met as written:** production's one-off links (0354374e and de1c3e5e) carry the "keep out of search engines" header on all 7 paths, added by the host; every other header as expected, and both planted mistakes were caught |
| Isolation follows the flag in every browser (PM-0006-3) | "3 of 3 loads per browser and deployment", in Chrome, Edge, Firefox, Safari and Brave (:1371) | Met with a departure: Safari not run (his words: "focus on brave, chrome, firefox and maybe edge (most used browsers)"); Chrome, Edge and Firefox 3 of 3 on all five links, the planted mistake caught; Brave 1 load per link, each as expected |
| The content file is cached for good (PM-0006-4) | "immutable"; on a second visit "it transfers 0 bytes", "3 repeats" (:1372) | Met with a departure (a GitHub machine for the laptop): 0 bytes in 3 of 3; the planted `no-cache` caught |
| Every file fits the host (PM-0006-5) | "the largest file in each deployed build under 25 MiB ... on every deployment of the check" (:1373) | Met with a departure (the same): every build's size line read, the largest file 65,536 bytes; the planted 26 MiB file flagged and refused by the upload tool |
| A closed pull request's build is gone in 24 hours (PM-0006-6) | "24 h after each test pull request closes, none of its URLs ... answers HTTP 200 with that build's marker"; run "with the access policy off" (:1374) | Met with a departure: at 24.1 to 24.4 hours every link of the four closed cases gone, the open control and the bite still serving, no wall; the hourly probes fell short (3 scheduled runs in the 24 slots; by-hand probes stood in) |
| Do uploads count against the 500 builds a month (PM-0006-7) | "reported, no pass number: the project's monthly build count before and after 10 direct uploads ... passes when both are recorded" (:1375) | **Open:** no count is shown on any page read, before or after; Cloudflare's docs do not say |
| The wall covers the previews and not production (PM-0006-8) | "on every preview URL of at least 2 open branches ... and on PM-0006-6's leftover URLs, a cookie-less client gets no build"; "on the production URL ... it gets the build"; "the mark reads the 1-hour sweep" (:1376) | Met, on a reading of "open": at 1 hour or more no build from any link of six Pages branches (one with an open pull request, five without) or of the leftover links; production's own address served; with the wall off, every build served |
| His verdict in Brave (PM-0006-9) | "every step of the hosting checklist below \"achieved\", 0 \"not yet\"" (:1377) | **Open:** steps 1 to 3 and 5 to 8 "achieved" in his words; step 4 (the build count) has no verdict |
| GitHub shows the preview link on the pull request (PM-0006-10) | "recorded: yes or no, with a screenshot of the page" (:1378) | Met: yes, logged out (this chat's picture) and logged in (his words) |
| A pull request cannot reach the token or production (PM-0006-32) | "2 of 2: ... gets no secret until the `preview` environment's required reviewer approves it ...; a job in a pull request that names the `production` environment is refused" (:1379) | Met: both cases, the control and the bite; reported: a production deploy with the preview token is not refused; recorded: the merge ref is judged and refused, and his revocation is proven by the refused upload |

By the draft's rule the spike has not passed: one mark is not met as written (the headers) and two are open (the build
count, and his verdict on step 4, which asks for that count). The departures change no reading.

## How it ran
- **The plan** (`.checkpoints/preview-hosting-check/plan.md`, revision 2 at 2a2f426; readable at CARDSHA after the
  notes leave the branch): the advisor agent's review of revision 1 was APPROVE with conditions (A1 to A13, all folded
  into revision 2). His answers to its seven decisions (each (a)), his go and his yes to Part 7's six workflow texts by
  their sha256: 2026-11-15, 18:42:30 UTC.
- **The build**, in the test repository on branch `setup` (U1 to U3): the test site and its build script, the check
  scripts (headers, sizes, the deletion and wall probe, the browser rows with WebdriverIO 10.1.0), four throwaway
  workflows (deploy, close, probe, browsers) and two token-test variants. Proven locally before any run: proof.mjs 29
  of 29 and browser-proof 5 of 5 (Chromium 153.0.8010.12); these prove the scripts, not Cloudflare.
- **Reviews of the build** (details under "Reviews"): round 1 found one fault twice (Pages redirects `.html` paths, so
  every link check would have failed on `/index.html`); fixed and proven; round 2 approved; the workflow diff that
  followed (`deployments: write` removed, the upload records kept 14 days) has his yes of 2026-11-16; the advisor
  agent approved head 1619164 with three conditions, all met before the first run.
- **Setup and the rehearsal** (outside the marks): his settings were checked by his screenshots; two setup misses
  showed only there (the administrator bypass left ticked; no token made after a wait of mine that was never lifted in
  words). The rehearsal's upload failed five times at Cloudflare's sign-in; a by-hand token check (reviewed by the
  security-auditor and the advisor agent, his yes for one run) measured the cause: the token form's single-date chip
  had set a one-day window, which this chat had read from his screenshot as "end date 27 November" (its miss, told to
  him). After his fix the rehearsal ran end to end (2026-11-18, 14:55 to 15:04 UTC).
- **The sittings**: Day 1 (2026-11-18): sitting 1 (headers, browsers, caching, file sizes, the build count), sitting 2
  (the four deletion cases) and sitting 3 (the token tests, 2026-11-19, 00:10 to 00:22 UTC). Day 2: the 24-hour
  reading (2026-11-19, 18:56 UTC), then on 2026-11-24 the login wall, his Brave steps 5 to 8, the closes and the
  revoke.
- **Runs**: 56 in the test repository, each tallied against its sitting's list after the sitting: nothing ran that was
  not on a list. All acts in the test repository were within his go (the named branches, the setup and eight test pull
  requests, the six closes without a merge, the by-hand runs and the token-test re-runs); every merge, approval and
  setting was his.

## Evidence, mark by mark
Every reading stands word for word in Appendix A, by sitting and in time order; the verifier spot-checked the key ones
against the downloaded run data (below, "Reviews"). Where to look, mark by mark:
- The setup record: "Setup header (PM-0006-1)", "PM-0006-1: the header checked again before the 24-hour reading", "PM-0006-1, the header's Brave line".
- The headers: round-a, `uploads`, `bites`, `bite-production`, `restore-production`, and "Sitting 1's readings in short" (24 preview readings on 15 links, production's own address twice).
- Isolation and caching: `browsers` (51 Windows rows; Safari 1 reading of 15, then dropped).
- File sizes: "After the verifier" (every build's size line).
- Deletion: "Day 1, sitting 2", the probes after it, "The 24-hour reading" and "PM-0006-6 read".
- The build count: "The build count, first reading" and "second reading".
- The wall: "The control with the wall off", "The wall sweep at 10 minutes or more", "at 1 hour or more" and "PM-0006-8 read".
- Brave: "The hosting checklist, steps 1 to 3" and "steps 5 to 8, with the wall on".
- The pull-request page: "PM-0006-10" in sitting 2.
- The token: "Day 1, sitting 3", "PM-0006-32 clause by clause", the revoke and "The revocation read".

## What the check answers, and what it gives the open questions
- **PREV-DELETE (R5-PREV-OQ2, "Does a closed PR's preview really disappear on Cloudflare Pages?")**: yes, through the
  cleanup job. With the job, every link of a closed pull request was gone 3 to 22 minutes after the close and at every
  later reading (24.1 to 24.4 hours; and 144 hours on 2026-11-24). Without the job (the bite (d)), its links still
  served after 24 hours: the host itself does not remove a closed pull request's build. The tombstone workaround was
  not needed.
- **R5-PREV-OQ5 ("Does GitHub show the environment URL on the PR page?")**: yes: pull request #3's page shows "View
  deployment" with the branch link, logged out (this chat's picture) and logged in (his words).
- **R5-PREV-OQ6 ("Do branch aliases match `https://:version.:project.pages.dev/*`?")**: yes: the header rule written
  for that pattern (it marks its responses `X-Check-Rule: version-host`) applied on every branch link as well as on
  the one-off links; production's own address got the project rule.
- **For OQ-0006-4 (what the wall should cover)**: "Restrict previews" puts every preview link behind Cloudflare Access,
  branch links included, and also production's one-off links (the hash links of production uploads); production's own
  address stays open. Its login screen asks for an e-mail address and sends a code. His login lasted across a Brave
  restart. Setting it up made a Zero Trust organization with no plan or payment screen, and nothing paid was added
  (his words, 2026-11-24, 22:47:31 UTC).
- **For OQ-0006-13 (what the reviewer costs, and whether a session can approve)**: approvals he clicked within the
  marks: sitting 1: 3 (one per run, whatever its legs); sitting 2: 0 (the reviewer off by his REVIEWER (b)); sitting 3:
  1 approval and 1 rejection; Day 2: 2 approvals and 2 rejections. Had the reviewer stayed on, sitting 1 would have
  needed 5 and sitting 2 14 (one per run: computed, not measured). Outside the marks: the rehearsal 7 (six upload
  attempts and a cleanup) and the token check 2. A session cannot approve a deployment with its tools (no such method in
  the GitHub tool; SR-READ: the Claude app holds no write access to administration or deployments). A production
  deploy with the `preview` environment's token is not refused (round-a's production leg; part 13 expected this). His
  words on the approvals: "When are we done with these hideous tasks" (2026-11-18, 17:33:15 UTC); "Why do I have to
  approve these things, can't we just automate it?" (17:50:30 UTC); and for step 2.6's plan: "I do not want to click
  for each preview" (2026-11-19, 00:01:25 UTC).
- **Measured for step 2.6** (sitting 3): with no required reviewer, a pull request's job gets the token; a `main`-only
  branch rule refuses a pull request's job that names production, judging the ref `refs/pull/10/merge`.

## Reviews
- **The plan**: the advisor agent, revision 1: APPROVE with conditions (A1 to A6 Medium, A7 to A13 Low; all folded).
- **The build, round 1** (infra-engineer, security-auditor, test-engineer, at effort high): two Highs (F1 and TE-1, the
  same fault: Pages answers `/index.html` with a redirect); Mediums F2, SA-R3, SA-R4, TE-2 to TE-6. Fixed in 2df0112
  (scripts and page only; the approved workflows untouched); SA-R1 (remove `deployments: write`) and F2 (records kept
  14 days) went into the workflow diff his yes of 2026-11-16 covers; SA-R3 he accepted in his words; SA-R4 and SA-R8
  became steps of sitting 3. Deferred: SA-R2 (`actions: read`, to step 2.6), SA-R5, SA-R6, F5, F6.
- **Round 2** (infra-engineer, test-engineer, on what changed): APPROVE; Lows kept as follow-ups.
- **The advisor agent on head 1619164**: APPROVE with three conditions (his screenshot of the empty repository-secrets
  list before the setup pull request; two record repairs; the open security findings named on his setup list), all met.
- **The token check**: security-auditor REVISE (an error path could print the account's name and ID), then APPROVE on
  sha256 0ad66aa0f2b3...; the advisor agent APPROVE with conditions (the outcome table read in order). His yes covered
  one run; its logs were deleted after reading, as he asked.
- **The verifier on the evidence** (2026-11-24): PASS WITH NOTES on the evidence: five spot-checks against the downloaded data (the 24-hour
  reading, both wall sweeps, the control, the header summaries, the browser rows), all PASS; its verdicts per mark are
  the table above. It first called the file-size mark open (no size line recorded for 13 builds); this chat then read
  every build's size line from the run logs, and the verifier, given them, judged it met with a departure. Its
  corrections are in Appendix A, "After the verifier". Not verified by it: the "secret: present" lines, his screenshots
  and words, sitting 2's header files. The gate did not run for it (the toolkit was not installed); this chat ran it
  afterwards: PASS, the smoke test skipped (the branch changes only docs and notes).
- The review notes (`.checkpoints/preview-hosting-check/*-review*.md`, `advisor-*.md`, `security-auditor-diag.md`,
  `verifier-evidence.md`) are readable at CARDSHA.

## The docs pass
- `docs/DECISIONS.md`: his fourteen messages to this chat, from the task (2026-11-15, 15:33:37 UTC) to Day 2 (2026-11-24,
  19:07:46 UTC), each whole, copied by script from the chat's transcript and checked byte for byte against the saved
  copies and their sha256, each placed by its time between the decisions around it; 252 lines added, no other line
  changed (RECORD-FILES). Commit 7c803a6.
- DOCSKEEPERLINE

## Decisions and deviations
- **Machines** (his MACHINES (a)): GitHub's hosted runners stood in for the minimum laptop and the Mac; each run names
  its image.
- **Safari** left the check by his decision (2026-11-18, 18:21:30 UTC, after its row read 1 load of 15 through a script
  fault): PM-0006-3 was read on Chrome, Edge and Firefox by script and in Brave by him. The draft's author is told.
- **The reviewer** (his REVIEWER (b), 2026-11-18, 18:04:10 UTC): off for the rest of Day 1 but sitting 3, on again for
  sitting 3 and Day 2. Windows with it off: 2026-11-18 18:08 UTC until his tick (between 18:51:11 and 19:12:20 UTC by the
  run records) and 2026-11-19 00:10:53 to 00:13:42 UTC (the bite); `production`'s branch rule off 00:19:35 to 00:21:51
  UTC (the bite). Each closed by his screenshot or seen closed in one.
- **The token**: Cloudflare Pages Read and Edit (his TOKEN-SCOPE (a): Read stays; "Edit alone" was never measured); its
  window fixed to 7 days by him (TOKEN-WINDOW); the Cloudflare account renamed so a failed upload cannot print his
  e-mail address (his yes of 2026-11-17).
- **The hourly series** (PM-0006-6's method): GitHub's hourly schedule fired 3 times in the 24 slots from the closes to
  the 24-hour mark; by-hand probes stood in at 4 more points; the 24-hour reading itself was taken by hand, as planned
  (plan Risk 4).
- **The token tests claim only the extra bite of ADR 0002's PM-0002-66** (the `production` rule removed, the job printed
  "present"), never that mark as a whole.
- **Pictures stay out of the repository**: the pull-request page's picture (PM-0006-10), the logged-out run pages and his
  screenshots are under `.gate/` (gitignored) or in the chat; the results log transcribes what each shows. The plan
  listed the PR-page picture among the closing files: a departure, made because the other chats' records keep
  pictures out of the repository too.
- **The wall's "at least 2 open branches"**: six Pages branches were walled on every link, one with an open pull
  request (cs/pd-open) and five made by the by-hand runs with no pull request (iso-off, iso-on, uploads, bite-coep,
  bite-nocache). The draft does not define "open"; its reason for two ("with one, a wall set on that branch alone would
  pass too", ADR 0006 :1360) fits counting all six.
- **His Brave version** reached the setup record on 2026-11-19, 21:02:51 UTC, about 2 hours after the 24-hour reading
  (the record was to be checked again before it); his Brave steps 1-3 of 2026-11-18 ran on a version no one recorded
  then. Decided by no one: one of the departures put to him.
- **The wall's switch-on time** is known only as a window (19:12:54 to 20:14:13 UTC on 2026-11-24), so "10 minutes" and
  "1 hour" are lower bounds. The setting is now "Preview access" with the button "Restrict previews"; the row's
  "Cloudflare Access" link opens Cloudflare's product page with "Try it for free", which he pressed on 2026-11-19 before
  my go: it switched nothing on (measured at once from outside and by a probe).

## The process, as the records show it (his rule: only what a record shows, with its time)
- 2026-11-15: steps 5 to 8 were saved as done before the advisor tool was consulted (16:16:22 and 16:21:42 UTC;
  consultation 2 at 16:22:00 UTC); from then on each step was saved as done only after its consultation.
- 2026-11-15, 20:44:03 UTC: a wait of mine at the token step was never lifted in words; on 2026-11-17 he wrote that he
  "had never received instructions to create them". Two other setup steps that needed a change from a default were
  caught only by his screenshots (2026-11-16).
- 2026-11-16: rewriting the checkpoint's "Current step" had dropped four finished steps from it; restored by script
  from git (the advisor agent's finding F-A).
- 2026-11-17 to 18: this chat read his token-form screenshot as "end date 27 November" and called the gate met; the
  form had set a one-day window, the cause of the rehearsal's five failures (found by the token check, 2026-11-18).
- 2026-11-18, sitting 3: this chat's reorder assumed the reviewer was still off; it was on (his tick came after my
  list); the outcome row "the off run waits" caught it, and the order went back to the plan's.
- 2026-11-19, 13:51:09 UTC: my Day 2 list put steps that had to wait in front of him five hours early; he pressed at the
  access-policy step at 14:09 UTC (nothing switched on). Since then a step that must wait goes only with the go.
- 2026-11-24: in the closing message I first wrote that his steps 5 to 8 "complete the Brave checklist"; step 4 still
  has no verdict word (corrected in the same evening's message).
- The advisor tool was consulted 28 times (Appendix C, with times); apart from the slip of 2026-11-15 above, each before
  the step it bore on was saved as done.
- Refused by the guard: a `git -C` of this chat (2026-11-15, 15:54:02 UTC; the route: run git from inside the folder);
  and, while the advisor agent wrote its notes, a relative path and then a variable target (its route: a literal
  absolute path).

## Not verified, not covered
- What the draft itself leaves out (ADR 0006): the weekly sweep and the 7-day artifact rule (step 2.6's jobs), the
  staging game server, the production certificate, the content security policy; no custom domain was used.
- Safari (dropped); Edge-on-Mac and phones were never in the check.
- "Edit alone is enough" for the token: never measured (Read was added before the cause was found).
- The build count: no count is shown on the three places the plan names (PM-0006-7 reads "no count shown").
- The local evidence (downloaded artifacts and pictures under `.gate/`, gitignored) stays outside the repository; the
  results log in Appendix A transcribes what each shows.

## Risks
- **Risk 13 happened**: production's one-off link carries the "keep out of search engines" header, which the host adds
  to every preview address. By ADR 0006's "When a mark fails", "A failed mark goes back to the draft: infra-engineer revises the decision
  part the row names by the draft's own Rollback and "Revisit when" routes (a header or PREV-DELETE fail on Pages:
  Workers static assets weighed again ...), and the revised part returns to the advisor for review and to Mustafa and
  the Advisor with the measured numbers" (ADR 0006 :1411-1416). Nothing in ADR 0006 changes in this pull request. That
  paragraph does not name step 2.6; the plan he approved does ("the check's end and step 2.6 wait for the revised
  draft", Risk 13), while his order of 2026-11-18 puts step 2.6's plan right after this report. Whether step 2.6 waits
  is his decision (below)..
- **The test repository's hourly probe after 2026-12-02**: the upload records it reads expire then (14 days).
  The upload records are kept 14 days; the last of them expires on
  2026-12-02 (about 18:35 UTC). After that the probe finds nothing to probe and ends every run red (it exits 3 when it
  has no rows), and GitHub mails the failure of a scheduled workflow to the person who made it: Mustafa's account.
  GitHub's docs do not say whether archiving a repository stops its scheduled runs; switching the workflow off does. So
  the probe is switched off before 2 December (For Mustafa).

## Follow-ups (for STATUS)
- For step 2.6's plan: previews without his click per preview, and what keeps the token safe then (his words of
  2026-11-19, 00:01:25 UTC); a locked install (a committed lockfile, install scripts off; his point 3 of 2026-11-16);
  token setup ends with Cloudflare's verify answer, not a screenshot of the form; the form's single-date chip sets a
  one-day window; every probe gets a named User-Agent; delete with `--force` or the API; a weekly sweep on `main`;
  `actions: read` reconsidered (SA-R2); the deploy step's error path and the account printout (the rename answered
  the name; the ID is masked when it equals the secret).
- The test repository: archive it (his TR-END (a)) once this report is merged; then delete the test project and the
  Access application (Zero Trust, Access, Applications).
- Round 2's Lows (infra L1, L2, L4; test R2-1 to R2-6) and the deferred SA-R5, SA-R6, F5, F6 stay with the throwaway
  scripts; none carries into step 2.6 unless its plan picks it up.

## Proposed next step
His three decisions below. Then, unless he decides that step 2.6 waits for the draft's revision: step 2.6's plan (the
previews without his click and what keeps the token safe then, the locked install), brought to him for his go; the
revision of ADR 0006's header row is infra-engineer's work by the draft's route, after his word. The test repository's
probe is switched off before 2 December, and the repository archived and the test project deleted after this report is
merged (his TR-END (a)).

## Decisions needed
1. **ONE-OFF-HEADER: what happens to the header mark that is not met.**
   - What: Cloudflare adds the "keep out of search engines" header (`X-Robots-Tag: noindex`) to every one-off address
     of the project, production's uploads included; production's own address stays clean. The draft's header mark
     asks for "production none", so it is not met as written; by the draft's rule it goes back to the draft.
   - Why it matters: the test cannot pass until the draft is revised, and the plan he approved says step 2.6 waits for
     that revision.
   - Options: (a) infra-engineer revises the draft's header row to judge production's own address only (a one-off
     address works like a preview address, and the header there keeps old builds out of search engines), the revision
     comes to him with these numbers, and step 2.6's plan starts meanwhile, as his order of 2026-11-18 has it; (b) the
     same revision, and step 2.6 waits for it, as the plan's Risk 13 says; (c) the draft weighs Workers static assets
     again (its own route for a header fail on Pages).
   - My recommendation: (a). The finding changes nothing a player or a preview link meets, and step 2.6 does not use
     production's one-off links.
2. **STEP4-COUNT: checklist step 4 and the build-count question.**
   - What: step 4 asks for Cloudflare's count of builds this month before and after 10 uploads, to learn whether
     uploads count against the Free plan's 500 builds a month. No page he read shows a count, and Cloudflare's docs do
     not say. So step 4 has no verdict and the build-count mark is open.
   - Why it matters: both stand between the test and a pass; and if uploads count, step 2.6 must count its deploys.
   - Options: (a) both stay open until a count source exists (a usage page, or Cloudflare's answer to a question);
     (b) both go back to the draft with the header row: the question is re-stated as one the dashboard cannot answer,
     and step 2.6 counts its own deploys and stays far below 500 a month whatever the answer; (c) he gives step 4 a
     verdict now (only "not yet" would match what he saw: no count).
   - My recommendation: (b).
3. **DEPARTURES: may these recorded departures stand?** None of them changes a reading.
   - (1) His Brave version reached the setup record about 2 hours after the 24-hour reading (it was to be checked
     before). (2) GitHub's hourly checker ran 3 times in the 24 hours, so the deletion series has 7 readings, by-hand
     checks standing in; the 24-hour reading itself was taken. (3) The wall's "at least 2 open branches" counted six
     Pages branches, only one of them with an open pull request; the draft's reason for two fits all six. (4) Safari
     was not run, by his words "focus on brave, chrome, firefox and maybe edge (most used browsers)".
   - Options: (a) all four stand as recorded; (b) he names any that goes back to the draft (none can be re-run as it
     was: the Brave version of 18 November was not recorded, and the hourly series would need a new 24-hour case).
   - My recommendation: (a).

## For Mustafa
1. The three decisions above, one line each (for example "ONE-OFF-HEADER (a), STEP4-COUNT (b), DEPARTURES (a)").
2. Before 2 December: switch off the test repository's hourly checker, so it cannot mail failures once its records
   expire. Go: any time now (no reading needs it any more). Clicks (GitHub's docs): Mergincanli/shamble-preview-check →
   Actions → in the left list, "probe" → the "..." menu at the top right → Disable workflow.
3. Your go on this pull request, once the advisor agent has reviewed its head (it is a draft until then).
4. After it is merged, on my go: archive the test repository (Settings, General, "Archive this repository"), then delete
   the test project in Cloudflare and its Access application (Zero Trust, Access, Applications).

## Appendix A: the results log
APPENDIXA

## Appendix B: the workflow texts as run
APPENDIXB

## Appendix C: the advisor tool's consultations
APPENDIXC
