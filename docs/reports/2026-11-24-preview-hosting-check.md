# Advisor report: the preview hosting check (2026-11-24, branch `claude/optimistic-albattani-mdoonz` standing in for `cs/preview-hosting-check`, base main 28254a7)

Pull request: #66 (https://github.com/Mergincanli/Shamble-Frontlines/pull/66). Risk tier 1: it adds this report and records Mustafa's words in `docs/DECISIONS.md`; it
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
departures that change no reading. Four decisions of his follow; this pull request waits for them and for his go.

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
| His verdict in Brave (PM-0006-9) | "every step of the hosting checklist below "achieved", 0 "not yet"" (:1377) | **Open:** steps 1 to 3 and 5 to 8 "achieved" in his words; step 4 (the build count) has no verdict |
| GitHub shows the preview link on the pull request (PM-0006-10) | "recorded: yes or no, with a screenshot of the page" (:1378) | Met: yes, logged out (this chat's picture) and logged in (his words) |
| A pull request cannot reach the token or production (PM-0006-32) | "2 of 2: ... gets no secret until the `preview` environment's required reviewer approves it ...; a job in a pull request that names the `production` environment is refused" (:1379) | Met: both cases, the control and the bite; reported: a production deploy with the preview token is not refused; recorded: the merge ref is judged and refused, and his revocation is proven by the refused upload; one departure: the environments' deployment history was read from the pull-request pages (the repository's deployments pages answer 404 when logged out) |

By the draft's rule the spike has not passed: one mark is not met as written (the headers) and two are open (the build
count, and his verdict on step 4, which asks for that count). The departures change no reading.

## How it ran
- **The plan** (`.checkpoints/preview-hosting-check/plan.md`, revision 2 at 2a2f426; readable at d28b8f1d4040 after the
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
  `verifier-evidence.md`) are readable at d28b8f1d4040.

## The docs pass
- `docs/DECISIONS.md`: his fourteen messages to this chat, from the task (2026-11-15, 15:33:37 UTC) to Day 2 (2026-11-24,
  19:07:46 UTC), each whole, copied by script from the chat's transcript and checked byte for byte against the saved
  copies and their sha256, each placed by its time between the decisions around it; 252 lines added, no other line
  changed (RECORD-FILES). Commit 7c803a6.
- His message of 2026-11-25 (Amsterdam; 2026-11-24, 23:16:11 UTC; lane 2 building beside lane 1; his question on
  change-set 6 and M before change-set 5) added
  the same way (20 lines), so that it outlives the checkpoint.
- docs-keeper's pass (its notes, readable at d28b8f1d4040, list every line): `docs/STATUS.md`, lane 2's lines (this pull
  request in "In flight"; step 2.6's plan and his question of 2026-11-25 in "Next"; his decisions, the probe's
  switch-off and the clean-up after the merge in "Waiting on Mustafa"; nine follow-ups named PHC-; two open risks,
  PHC-HEADER and PHC-PROBE-MAIL) and their texts in `docs/STATUS_ARCHIVE.md`. To keep STATUS within its 120 lines, the
  follow-up bullets of the merged #49, #50, #51 and #48 (STATUS lines 78-93 at 28254a7) moved by script, byte for byte,
  to the archive's new "Moved from STATUS (2026-11-24)" (his OLD-LINES); this chat checked all 16 lines verbatim.
  `docs/ROADMAP.md`: one entry, and the oldest entry (foundation 2.2, 17 lines) moved by script, byte for byte, to "Build
  history" (OLD-LINES; checked verbatim). `docs/REPO_MAP.md`: the report's row (the map check: "repo map OK: 178 entries
  cover 109 required paths"). `docs/SPIKE_PLAN.md`, entry 6: a dated pointer to this report. Lane 1's lines that #65's
  merge made stale (3a's row and bullets) are left for lane 1 (RECORD-FILES). No ADR changes.

## Lane 2's first list of needs (issue #59), folded from its own checkpoint
- His lane-2 words of 2026-11-18 (in DECISIONS) asked for lane 2's needs from the simulation's per-tick block before
  change-set 4's plan is reviewed. One agent (engine-architect) read the sources, one advisor-tool consultation checked
  the list, and one comment went on the "Lane requests" issue, naming lane 2:
  https://github.com/Mergincanli/Shamble-Frontlines/issues/59#issuecomment-6994614989 (2026-11-18, 23:24:14 UTC): 16
  fields (one of them conditional), 6 events, 5 wishes and 10 gaps or questions, each with its source.
- His answers since, in DECISIONS: PROJ-DRAW (a) and AIM-STATE (a) (2026-11-19, 00:01:25 UTC), and his words of
  2026-11-19, 12:00 (aiming down sights and the sprint-to-fire delay are game rules of Slice 1; the wishes W1 and W4 are
  needs now; the comment stays as it is). Change-set 10's plan starts from AIM-STATE's rule for the poses.
- That checkpoint (`.checkpoints/lane2-needs.md` with its notes) is readable at d28b8f1d4040.

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
  is his decision (below).
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
His four decisions below. Then, unless he decides that step 2.6 waits for the draft's revision: step 2.6's plan (the
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
     comes to him with these numbers, and step 2.6's plan starts meanwhile, as his order of 2026-11-18 has it (this
     skips the draft's own route for a header fail on Pages, weighing Workers static assets again: option (c)); (b) the
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
     was not run, by his words "focus on brave, chrome, firefox and maybe edge (most used browsers)". (5) The pictures
     stay out of the repository, the pull-request page's picture (PM-0006-10) among them, though the plan he approved
     said the closing pull request would carry that one; the report transcribes what each picture shows.
   - Options: (a) all five stand as recorded; (b) he names any that goes back to the draft (none can be re-run as it
     was: the Brave version of 18 November was not recorded, and the hourly series would need a new 24-hour case).
   - My recommendation: (a).
4. **HISTORY-ID: how this pull request is merged, given what its branch's history holds.**
   - What: the account ID's first and last four characters (not the ID, which is not a password either) sit in this
     branch's older checkpoint commits, from de1a902 to 23418fd, and the report points to d28b8f1 for the deleted
     checkpoint. A normal merge carries those commits into main's history; a squash merge gives main one commit
     without them. Rewriting history to remove them is never allowed.
   - Why it matters: the test's plan kept the account ID out of public logs; this is a fragment of it, in public
     history.
   - Options: (a) a normal merge: the history and the report's link to the checkpoint stay whole; (b) a squash merge:
     main never holds the fragment, and the link to d28b8f1 works only while this branch exists.
   - My recommendation: (a). Eight characters of an account ID open nothing, and his own words in DECISIONS already
     name the last four.

## For Mustafa
1. The four decisions above, one line each (for example "ONE-OFF-HEADER (a), STEP4-COUNT (b), DEPARTURES (a),
   HISTORY-ID (a)").
2. Before 2 December: switch off the test repository's hourly checker, so it cannot mail failures once its records
   expire. Go: any time now (no reading needs it any more). Clicks (GitHub's docs): Mergincanli/shamble-preview-check →
   Actions → in the left list, "probe" → the "..." menu at the top right → Disable workflow.
3. Your go on this pull request, once the advisor agent has reviewed its head (it is a draft until then).
4. After it is merged, on my go: archive the test repository (Settings, General, "Archive this repository"), then delete
   the test project in Cloudflare and its Access application (Zero Trust, Access, Applications).

## Appendix A: the results log
_Moved whole from `.checkpoints/preview-hosting-check/results-log.md` by script (103927 bytes before the move; its headings demoted two levels; 4 places where it gave the account ID's first and last four characters replaced by a marker). Every time is UTC unless marked; times about this chat's own process are transcript times._

### Results log: the preview hosting check (ADR 0006 spike 1, SPIKE_PLAN entry 6)
Kept while the check runs (plan Part 2, "The results log"); folded into the report at the end. Every time is UTC
unless marked; every time about this chat's own process is a transcript time. Written 2026-11-17, before the setup
pull request opens (plan Part 3 B; consultation 3).

#### Setup header (PM-0006-1)
To be checked again before the 24-hour reading.

**The test project (Cloudflare Pages).**
- Name `shamble-preview-check`; production link https://shamble-preview-check.pages.dev (his A4 answer, 2026-11-15
  20:42:20). Made by his drag-and-drop upload of a placeholder page (plan A4), so production's first version is his.
- His Cloudflare account holds nothing besides the domain shamblefrontlines.com and this project (his words,
  2026-11-15 20:42:20: "this is the first time that I've been using cloudflare").
- The token: an Account API token `shamble-preview-check-deploy`, "Pages Write" only (the form: "Developer Platform"
  1/62, "Pages", "Edit" ticked, "Read" unticked), scope "Entire Account", client IP filtering empty, token expiration
  "Nov 27 GMT+1" as chosen in the form (his screenshot, 2026-11-17 20:13:53). No screen after saving shows the end
  date; the "Account API tokens" list (20:29:51) shows the token, "Pages Write", "Last used" empty, created 2 minutes
  earlier. The token is meant to end ten days after it was made (his SA-R3 acceptance, 2026-11-16) and to be deleted
  by him on Day 2; Day 2 stays on or before 2026-11-26 (Amsterdam).
  Correction, 2026-11-18: the check measured the token's window as not_before 2026-11-26T23:00:00Z, expires_on
  2026-11-27T22:59:59Z (the token works only on 27 November); "Day 2 by the 26th" is withdrawn until the window is
  measured again after his fix.
  Update, 2026-11-18 (the second check, 14:27 UTC): the token holds "Pages Read" and "Pages Write" (Read added by him on
  2026-11-17, TOKEN-READ (a)); its window, measured: not_before null, expires_on 2026-11-25T22:59:59Z; the Cloudflare
  account is renamed "Shamble Frontlines" (his screenshot, 2026-11-18). Day 2's last day: 2026-11-25 (Amsterdam).

**The test repository (GitHub).**
- Mergincanli/shamble-preview-check, public; `main` 4a67b83b9ed672c81e1a6e3067f916841c02b4a9 (his README).
- The reviewed commit of `setup`: 1619164900553ee3e32339447b30865347711156 (the advisor agent's APPROVE, 2026-11-16,
  hand-back 09:02:47). After the merge, `main`'s merge commit is added here; it carries the same files and is what every
  later run uses. Added 2026-11-18: `main`'s merge commit 2162db55d93f59cf2ed7e371a3915409f0b5a587 (pull request #1
  merged by him at 14:57:11 UTC).
- The workflows at that commit, sha256 prefixes (equal to his yes of 2026-11-15 and 2026-11-16): deploy.yml
  29593af18d34, close.yml 3afdf389d636, probe.yml f24ad6e81daf, browsers.yml 6791076916ae. The two token-test variants
  (placed only in sitting 3): fd3dc4c6029b and 8b745631a278.
- Pins: actions/checkout v7.0.1 3d3c42e5aac5ba805825da76410c181273ba90b1, actions/setup-node v7.0.0
  820762786026740c76f36085b0efc47a31fe5020, actions/upload-artifact v7.0.2 cf430e030ddbb5b0abf93d22962f4752f3646cd9,
  actions/download-artifact v8.0.2 9000827ccba6bdab643e8b6fd33ac0654aef8333; Node 22 (setup-node); Wrangler 4.156.0;
  WebdriverIO 10.1.0 (tools/wdio/package-lock.json: 163 packages, all with integrity; installed with
  `npm ci --ignore-scripts`).
- Settings, from his screenshots (2026-11-16 11:34:31 and 12:34:49; 2026-11-17 19:17:53 and 20:29:51):
  - `preview`: required reviewer Mergincanli; "Prevent self-review" unticked; "Wait timer" unticked; "Allow
    administrators to bypass configured protection rules" unticked (found ticked on 2026-11-16, his fix on 2026-11-17);
    "Deployment branches and tags": "No restriction"; secrets CLOUDFLARE_API_TOKEN (pasted again 2026-11-17, "1 minute
    ago" at 20:29:51) and CLOUDFLARE_ACCOUNT_ID ("2 days ago" in that screenshot; from Cloudflare's Account Details
    and pasted again on 2026-11-17 by his words of 20:35:32, not by a screenshot).
  - `production`: no reviewer; "Wait timer" unticked; administrator bypass unticked; "Selected branches and tags",
    one rule, branch `main`; secret CANARY.
  - "Repository secrets": "This repository has no secrets."
  - Actions, General: "Allow all actions and reusable workflows"; "Require actions to be pinned to a full-length commit
    SHA" ticked; retention 90 days; "Require approval for all external contributors"; workflow permissions "Read
    repository contents and packages permissions"; "Allow GitHub Actions to create and approve pull requests"
    unticked.
- The Claude app (SR-READ, his screenshot of 2026-11-15 20:53:10): "Read access to Dependabot alerts, administration,
  commit statuses, merge queues, and metadata"; "Read and write access to actions, checks, code, discussions, issues,
  pull requests, repository creation, repository hooks, and workflows"; "All repositories". No write on administration
  or deployments.

**The build-marker rule.** Every build writes the commit it was built from into the page (`<meta name="build-marker">`
and the visible marker) and into manifest.json: `--marker "$MARKER"` with `MARKER: ${{ github.sha }}` (deploy.yml,
build job). For a pull request that is the merge commit GitHub makes for the run, not the branch's head; each record
names its marker, and the check scripts compare the served marker with the record's.

**The machines.** GitHub's hosted runners stand in for the draft's machines (MACHINES (a)): ubuntu-24.04 for every job
but the browser rows; windows-2025 (Chrome, Edge, Firefox) and macos-26 (Safari) for browsers.yml. Each run's image name
and version is read from its log (and recorded by run.mjs as `runner.image` and `runner.image_version`) and written next
to that run below. Browser versions: recorded per row by the browser run (to be added when it runs). Brave: his own,
on his PC (Day 2, steps 5 to 8).

**The rehearsal.** The setup pull request's two runs (its deploy, then its merge's cleanup) are a rehearsal outside the
marks (plan Part 3 B): kept and reported, counted toward no mark. The marks start with sitting 1.

#### Lines carried in for later readings
- F-C (the advisor agent, 2026-11-16): the lockfile's edgedriver 8.0.0 and geckodriver 8.0.0 declare install scripts;
  `npm ci --ignore-scripts` skips their driver downloads, and run.mjs passes the runner's drivers through
  CHROMEWEBDRIVER, EDGEWEBDRIVER and GECKOWEBDRIVER. That this is harmless is inferred until the browser run: a missing
  driver shows as "no reading", never as a pass.
- L3 (round 2): index.html and 404.html are not fetched by check-headers, since Pages answers their .html paths with a
  308 (finding F1/TE-1); their headers are read through `/` and a missing path.
- L5 (round 2): a verify of an earlier run reads a branch link that has had more pushes since as stale: an expected red.
- Procedures (round 1): each close log's "Uploads found" against the records (F3, TE-7); each SIZES line and the
  tombstone's size (TE-11); the page's resource list against the manifest (TE-12); on bite-coep, Chromium refuses the
  worker's script, the observed form of "false" (TE-3); a non-zero second visit is root-caused (TE-13).
- SA-R8 (round 1): at sitting 3, the empty "Repository secrets" list is recorded again.
- After the setup pull request's deploy, Cloudflare's "Last used" for the token should show a time (a cross-check).

#### Runs, readings and approvals
##### The rehearsal: the setup pull request (outside the marks)
- Pull request https://github.com/Mergincanli/shamble-preview-check/pull/1 ("Setup: the test site and the throwaway
  workflows"), opened by this session at 2026-11-17 20:41 UTC (the run below was created 20:41:39); head `setup`
  1619164900553ee3e32339447b30865347711156, base `main` 4a67b83.
- Run 1: `deploy` #1, id 44171977539, event pull_request, actor and triggering actor Mergincanli, created 20:41:39
  (https://github.com/Mergincanli/shamble-preview-check/actions/runs/44171977539). Job `plan` (id 134353201815,
  runner label ubuntu-24.04) started 20:41:41, success at 20:41:49. Job `build (pr, preview, setup, preview, 1)` (id
  134353302216) 20:41:51 to 20:42:07, success. Job `deploy (pr, preview, setup, preview, 1)` (id 134353491166):
  "waiting" on `preview` from 20:42:07 (read 2026-11-17 with the GitHub MCP tool). His list sent after that reading.
- Approval: Mergincanli approved `preview` (the run page's "Deployment protection rules", his screenshot sent
  20:50:58: "approved 1 minute ago"); the deploy job's log starts 20:49:39. Runner image ubuntu-24.04 version
  20261111.127.1 (Ubuntu 24.04.5 LTS). The build marker (`github.sha`, the merge commit): 64dcf68505f503df3f32ab258ca4363f240088ed.
- The deploy job failed (exit 1) at its first call to Cloudflare. Wrangler 4.156.0 (installed: "added 38 packages in
  5s"; Node v22.23.4): "A request to the Cloudflare API (/accounts/***/pages/projects/shamble-preview-check) failed.
  Authentication error [code: 10000]", then, on its own follow-up lookup, "A request to the Cloudflare API (/accounts)
  failed. Invalid access token [code: 9109]". No record was written ("No files were found with the provided path:
  records/"). The verify job then exited 3 (no records: "no reading"). Run status: failure, 8 min 28 s; artifacts 2
  (one `headers-44171977539`, 311 bytes). Reading: the value in CLOUDFLARE_API_TOKEN (or the account ID beside it) is
  not what Cloudflare expects; which of the two, and why, is not known from the log (both values are masked).
  A rehearsal finding (plan Part 3 B: a fix found here is not a re-run of a failed mark).
- Run data (read 2026-11-17 after the run): the deploy job was created 20:42:07 ("waiting") and started 20:49:39, so
  his approval came between those times (the approvals endpoint is not among this session's tools); steps: download
  20:49:42-45, "Refuse server code in the upload" passed, "Install Wrangler (this step holds no secret)" 20:49:46-55,
  "Upload to Cloudflare Pages" 20:49:55-57 failure; job done 20:49:58. Verify 20:50:00-07; its artifact
  `headers-44171977539` (id 14369645074, digest sha256:0073aac72dce..., equal to the downloaded zip) holds
  `{"checked_at": "2026-11-17T20:50:04.604Z", ..., "records": [], "summary": {"records": 0, "pass": 0, "fail": 0,
  "no_reading": 0}}` (saved here before its deletion for the re-run).
- The fix (consultation 10), by his screenshots and words (user message 21:09:07, 53e93cb6-2a07-4ea3-b488-0ee4dc515901): before any change,
  `preview`'s secrets showed CLOUDFLARE_ACCOUNT_ID "26 minutes ago" and CLOUDFLARE_API_TOKEN "34 minutes ago" (so the
  token box was not overwritten by the account ID), and Cloudflare's list showed the token with "Pages Write", "Last
  used" empty, created and modified "36m ago". Then (his words) the token rolled and pasted, the account ID pasted from
  Account Details, `headers-44171977539` deleted; after: both secrets "now" ("Environment "preview" updated.").
- Attempt 2 (his "Re-run failed jobs"): deploy job id 134368200375, created 21:06:06, started 21:06:47 (his approval
  between), "Upload to Cloudflare Pages" 21:06:56-58 failure with the same two errors (Authentication error [code:
  10000] on /accounts/***/pages/projects/shamble-preview-check; Invalid access token [code: 9109] on /accounts; his
  screenshot of the log). Verify job id 134368776685, 21:07:03-11, failure; its artifact upload succeeded (21:07:07-09).
  By the stop rule told to him: no third paste; the cause is looked for elsewhere.
- Attempt 2's verify artifact `headers-44171977539` (id 14369802505, digest sha256:9a38c73cece4..., equal to the
  downloaded zip): `{"checked_at": "2026-11-17T21:07:07.678Z", ..., "records": [], "summary": {"records": 0, "pass": 0,
  "fail": 0, "no_reading": 0}}`. The same name uploaded again in attempt 2 after attempt 1's copy was deleted.
- His words (22:12:28, ab54f723-7d7b-438b-bef8-02ccbfe60f9b): "I have also added read to the pages, and rerun the failed tasks, and its still
  not working." (TOKEN-READ (a), done by him; no screenshot of the token's page.) Run data (get_workflow_run:
  run_attempt 5, conclusion failure): attempt 3, deploy job 134390099209 created 21:41:38, started 21:42:05, upload
  failed 21:42:18-20; attempt 4, deploy job 134400842206 created 21:59:30, cancelled 21:59:45 (never started);
  attempt 5, deploy job 134402686180 created 22:02:34, started 22:08:53, upload failed 22:09:13-15, run with debug
  logging (his paste of the log: the secrets evaluate to '***', `github.sha` 64dcf68505f5..., the same two errors,
  Wrangler's log file named 2026-11-17_22-09-14). Verify failed in attempts 3, 4 and 5; each uploaded its artifact.
  So the permission is ruled out as the cause if Read was saved (only his words show it; no screen does): the test told
  to him (consultation 11). Left: the account ID or the token itself. (Two workers-sdk issue comments describe this
  pair of errors as a value Cloudflare does not recognise, and the second error is on a path without the account ID:
  a lean toward the token's value, not a finding.)
- Debug logging (his message 22:26:24, 145c9f63-a900-411c-9e2b-80fc93926e9e): turned on through the "Re-run failed jobs" dialog's "Enable
  debug logging" box (his screenshot of the dialog, not run); GitHub's re-run page: it applies to "the re-run". No
  repository secret or variable ACTIONS_STEP_DEBUG (his words: "I have not done such thing"), so the empty "Repository
  secrets" condition stands on his screenshot of 2026-11-16 and these words.

##### The token check (his yes of 2026-11-18; one run; outside the marks)
- Branch `check-token` made from `main` 4a67b83; one commit 863b639d128a994a47752fece88632d54344188e adding
  .github/workflows/check-token.yml (80 lines; `git show HEAD:` sha256 0ad66aa0f2b36f7469bc3bdd57a4a05b7945e0037f96b19059000e0ca9b5aa71,
  equal to his yes); pushed 2026-11-18 about 11:10 UTC (the push's own time is not in a record; the run below was
  created 11:10:59). Pull request https://github.com/Mergincanli/shamble-preview-check/pull/2.
- Run: `check-token` #1, id 44292323566, event pull_request, head 863b639d128a..., actor and triggering actor
  Mergincanli, created 11:10:59; job `check` (id 134760377971) "waiting" on `preview` from 11:11:00. No other run
  started (the run list holds only this one and the setup pull request's deploy run).
- His screenshots (user message 2026-11-18 13:49:17 UTC, 7ec06823-d86e-4e1f-ace6-22bde57d6101): Cloudflare's Configurations page, "Account
  name": "Shamble Frontlines" (the dashboard's top-left shows the same): the rename is done, so his gate "No deploy run
  of any kind starts before you have my screenshot of the renamed Cloudflare account" is met. The check's run page:
  "The deployments have been approved."; status Success; job "check" 15 s; "Mergincanli approved ... preview".
- The check's output, word for word (job 134760377971; log lines 2026-11-18 13:48:45 to 13:48:59 UTC; runner image
  ubuntu-24.04 version 20261111.127.1; "added 38 packages in 5s"):
  ```
  token: length 53; starts with cfat_: yes; whitespace in or around it: no
  account id: length 32; exactly 32 hex digits: yes; whitespace: no; first and last 4: [the account ID's first and last four characters, compared by him; not copied here]
  == A. the token, for this account: HTTP 200
  {
    "success": true,
    "errors": [],
    "result": {
      "status": "active",
      "expires_on": "2026-11-27T22:59:59Z",
      "not_before": "2026-11-26T23:00:00Z"
    }
  }
  == B. the token, without an account: HTTP 401
  {
    "success": false,
    "errors": [
      {
        "code": 1000,
        "message": "Invalid API Token"
      }
    ],
    "result": null
  }
  == C. the project the upload asks for: HTTP 401
  {
    "success": false,
    "errors": [
      {
        "code": 10000,
        "message": "Authentication error"
      }
    ],
    "result": null
  }
  == D: Wrangler exit 1
  D: shamble-preview-check named in its output: no
        1 [code: 10000]
        1 [code: 9109]
  ```
- Reading: both values have the right shape (53 = "cfat_" + 40 + an 8-character checksum, by the token-formats page's
  "cfat_[40 characters][checksum]"), and A answers 200 for this account, so the token is recognised and belongs to the
  account in the ID box. But Cloudflare reports the token's window as not_before 2026-11-26T23:00:00Z and expires_on
  2026-11-27T22:59:59Z: it may be used only on 27 November (Amsterdam time). Every call before then is refused: C and D
  give 10000, D's lookup 9109, B 1000. That explains attempts 2 to 5 (the rolled token), the empty "Last used", and why
  adding Read did not help; attempt 1 (the token before the roll, whose window was never read) is consistent with it. The form's chip "Nov 27 GMT+1" (his screenshot of
  2026-11-17 20:13:53) set both ends of the window to that one day; the session read that screenshot as "end date 27
  November" and called the choices right: a miss of the session's.
- After reading (his words of 2026-11-18): pull request #2 closed unmerged at 13:51:27 UTC (list_pull_requests:
  state closed, merged false); the run's logs deleted (delete_workflow_run_logs: "204 No Content"; get_job_logs for job
  134760377971 then answered 404). No run started on the close (the run list holds the same two runs).

##### The token check, second run (TOKEN-WINDOW and CHECK-RERUN done by him; ID-ENDS compared by him)
- His words (2026-11-18 14:30:11 UTC, 8b31c945-c07d-4b1b-8b62-185f409e0a0e): "Step 1 done, the api token is still the same, I have also set it to 7 days expiration / The rerun went well tool  / ran the check a 2nd time, which also ran properly. / Yes, cloudflare account id begins with that and ends with d8e7". His screenshots: Cloudflare's
  "Review token" page, NAME shamble-preview-check-deploy, EXPIRATION "November 25, 2026 (7 days)", "Entire Shamble
  Frontlines account" with "Pages Read" and "Pages Write", "All IP addresses allowed", buttons "Back to edit" and
  "Update token"; the check's run page, attempt "Latest #2", status Success, job "check" 12 s, "Mergincanli approved".
- Run 44292323566 attempt 2: job 134871714885 created 14:27:17, started 14:27:28 (his approval between), done 14:27:40.
  Its output, word for word (log lines 14:27:36 to 14:27:38 UTC):
  ```
  token: length 53; starts with cfat_: yes; whitespace in or around it: no
  account id: length 32; exactly 32 hex digits: yes; whitespace: no; first and last 4: [the account ID's first and last four characters, compared by him; not copied here]
  == A. the token, for this account: HTTP 200
  {
    "success": true,
    "errors": [],
    "result": {
      "status": "active",
      "expires_on": "2026-11-25T22:59:59Z",
      "not_before": null
    }
  }
  == B. the token, without an account: HTTP 401
  {
    "success": false,
    "errors": [
      {
        "code": 1000,
        "message": "Invalid API Token"
      }
    ],
    "result": null
  }
  == C. the project the upload asks for: HTTP 200
  {
    "success": true,
    "errors": [],
    "result": {
      "name": "shamble-preview-check"
    }
  }
  == D: Wrangler exit 0
  D: shamble-preview-check named in its output: yes
  D: no error code in its output
  ```
- Reading (outcome row 3): the token works now (not_before null) until 2026-11-25T22:59:59Z, the first record of its
  end date after saving; C finds the project; the pinned Wrangler answers. B's 1000 stays as before (the user verify
  method refuses an account token, consistent with both runs). The ID's ends [the account ID's first and last four characters, compared by him; not copied here] match Account Details (his
  words). Day 2's last day: 2026-11-25 (Amsterdam), from expires_on. The deploy's "Re-run failed jobs" must come before
  `site-pr` expires (2026-11-24T20:42:04Z).
- After reading: the run's logs deleted again (delete_workflow_run_logs, "204 No Content"); the run list unchanged (the
  deploy run still at attempt 5).

##### The rehearsal, completed (setup pull request #1)
- His message (2026-11-18 14:58:02 UTC, 2e16e511-3a92-408e-b805-574dfb6c61e4): "It finally worked, nice." / "I have merged the pull request."
  His screenshot: run 44171977539 "Latest #6", status Success, 53 s, artifacts 6, "Mergincanli approved ... preview".
- Attempt 6 (his "Re-run failed jobs", started 14:55:09): deploy job 134890100614 created 14:55:13, started 14:55:29
  (his approval between), "Upload to Cloudflare Pages" 14:55:38-46 success; its record, word for word:
  `{"deployment_id":"1f6487e4-0a85-43d1-8b1f-cd80c0e1d61c","url":"https://1f6487e4.shamble-preview-check.pages.dev","alias":"https://setup.shamble-preview-check.pages.dev","environment":"preview","production_branch":"main","branch":"setup","marker":"64dcf68505f503df3f32ab258ca4363f240088ed","pr":"1","run":"44171977539","leg":"pr","upload":"1","mode":"preview","expect":"preview","made":"2026-11-18T14:55:46.818Z"}`
  (artifact deployment-record-44171977539-pr, id 14438791220, 410 bytes; the environment URL set to the alias).
- Verify job 134890518875, 14:55:52 to 14:56:02, success; headers.json (artifact 14438806696, digest
  sha256:49e1998212c9..., equal to the downloaded zip): summary records 1, pass 2, fail 0, no_reading 0. Both links pass
  (stable https://setup.shamble-preview-check.pages.dev/ and one-off https://1f6487e4.shamble-preview-check.pages.dev/):
  every file 200 on the first try; `/tiny.wasm` as `application/wasm`; `X-Robots-Tag: noindex` on every response; no
  COOP or COEP (the preview mode without isolation, as expected); `content.ef463692.bin` with "public,
  max-age=31536000, immutable", the rest "public, max-age=0, must-revalidate"; `X-Check-Rule: version-host` on both
  links, the stable branch link included (a rehearsal observation on R5-PREV-OQ6, not a mark). index.html and 404.html
  not fetched, by design (L3).
- The merge: pull request #1 merged 14:57:11 UTC; `main` now 2162db55d93f...; the branch `setup` still exists (so
  "Automatically delete head branches" is off, as plan A2 step 4 needs). The merge's cleanup: run `close` #1, id
  44329986855, created 14:57:15, "waiting" on `preview` (read after his message). The question of plan step 4, whether
  a merged pull request's cleanup run starts at all: it does, and it names the branch `setup` (head_branch setup).
- The cleanup (his message "done", 2026-11-18 15:19:18 UTC, 00e898e1-427c-4966-8be1-11ca0bce6610; his screenshot: run `close` #1 Success, job
  "cleanup" 7 s, "Mergincanli approved ... preview"): job 134891480469 created 14:57:17, started 15:04:33 (his approval
  between); the tombstone steps skipped (not a cs/pdt-* branch); "Delete every other upload of this branch (force),
  oldest first" 15:04:36-38 success; its log, word for word: "Pull request closed at 2026-11-18T14:57:11Z (merged:
  true); this job started at 2026-11-18T15:04:36Z." / "Uploads of setup found: 1" / "Delete
  1f6487e4-0a85-43d1-8b1f-cd80c0e1d61c (https://1f6487e4.shamble-preview-check.pages.dev): HTTP 200
  {"success":true,"errors":[]}" / "This job ended at 2026-11-18T15:04:38Z." "Uploads found: 1" equals the records (one
  upload, 1f6487e4...: procedure F3/TE-7). The merged pull request's event carried the branch (BRANCH: setup, MERGED:
  true, CLOSED_AT set). From the close (14:57:11) to the deletion (15:04:38): 7 min 27 s, most of it the wait for his
  approval.
- The rehearsal is complete: deploy, verify and cleanup ran end to end. Still to read: the first `probe` run on `main`
  (schedule ":23"; none had run at 15:04:41 UTC, the cleanup run's last update).
- The first probe: none yet at 2026-11-18 15:39 UTC (list of probe.yml runs: total_count 0; the merge put probe.yml
  on `main` at 14:57:11, so the 15:23 slot was the first it could fill). Read again by a check-in at about 16:00 UTC.
- Still no probe run at 2026-11-18 16:02 UTC (total_count 0). The workflow is registered: list_workflows shows
  "probe", .github/workflows/probe.yml, state "active", created 2026-11-18T15:57:13+01:00 (14:57:13 UTC, the merge).
  A last check-in is set for 16:31 UTC, after the 16:23 slot.
- **Finding: the hourly schedule has not fired in its first two slots.** At 2026-11-18 16:31:34 UTC (check-in 3, the
  last), the list of probe.yml runs still gives total_count 0. The slots it could fill since the merge put probe.yml on
  `main` (14:57:11 UTC; workflow created 14:57:13 UTC): 15:23 and 16:23 UTC, both without a run. Why is not known
  (GitHub's page on the schedule event is not read in this session). Next (consultation 17): one probe run started by
  hand (`purpose: deletion`) after
  `round-a`'s verify, so the script has met Cloudflare before sitting 2's 24-hour clocks start. No further check-in.

##### Day 1, sitting 1 (the marks start here)
- `round-a`: deploy run #2, id 44347351522, started by the session (run_workflow, ref `main`, set `round-a`; created
  2026-11-18 16:26:57 UTC), head 2162db55d93f59cf2ed7e371a3915409f0b5a587. plan green (16:27:00-16:27:07); builds green:
  production (16:27:09-16:27:16), iso-off (16:27:09-16:27:34), iso-on (16:27:09-16:27:20); the three deploy legs
  (production, iso-off, iso-on) "waiting" on `preview` from 16:27:34 UTC. His list sent after that; still waiting at
  16:31:34 UTC.
- His approval (his message "Done:" with a screenshot of run #2's summary, 2026-11-18 17:06:23 UTC,
  3f82e55a-75de-4181-898f-d30fb10e81b9): the run page shows one event,
  "Mergincanli approved", Environments "preview and 2 more". The three deploy legs started at 17:01:51 (iso-on),
  17:01:53 (production) and 17:01:53 (iso-off) UTC, within 3 seconds: consistent with one approval releasing all three
  (his count of clicks asked). From waiting (16:27:34) to the release: 34 min 17 s (his list went out after 16:28).
- The uploads (each leg's log, word for word): production: "The project's production branch: main" / "✨ Success!
  Uploaded 1 files (7 already uploaded) (0.69 sec)" / deployment 0354374e-8cf6-4b8f-9735-15472d1529d9,
  https://0354374e.shamble-preview-check.pages.dev, alias null, environment "production", made 17:02:25.115Z. iso-off:
  "✨ Success! Uploaded 2 files (6 already uploaded) (2.17 sec)" / 6e61a4c4-37e0-443a-87ee-22ab90961be0, alias
  https://iso-off.shamble-preview-check.pages.dev, environment "preview", made 17:02:13.827Z. iso-on: "✨ Success!
  Uploaded 2 files (6 already uploaded) (2.71 sec)" / e48ee07b-2734-4be9-9e71-e46f217f6a9f, alias
  https://iso-on.shamble-preview-check.pages.dev, environment "preview", made 17:02:13.221Z. Every record's marker
  2162db55d93f59cf2ed7e371a3915409f0b5a587.
- PM-0006-5 (each build's SIZES line, word for word, the same for all three; node v22.23.4): "largest:
  content.ef463692.bin, 65536 bytes; limit 26214400 bytes; flagged: 0" / "SIZES
  {"largest":{"path":"content.ef463692.bin","size":65536},"limit":26214400,"flagged":[],"count":9}".
- PM-0006-2, verify (job 134977500343, 17:02:33-17:02:49; headers-44347351522, artifact 14448883620, zip sha256
  bbb324aadcea..., equal to the log's digest; read with jq from `.gate/preview-hosting-check/day1/round-a/`):
  "summary: {"records":3,"pass":5,"fail":1,"no_reading":0}", exit 1. Per link: iso-off stable and one-off pass;
  iso-on stable and one-off pass; production stable (https://shamble-preview-check.pages.dev/) pass; **production
  one-off (https://0354374e.shamble-preview-check.pages.dev/) fail on all 7 paths: "production carries X-Robots-Tag
  noindex"** (value "noindex"; production's build writes no noindex, so the host added it). Every response 200 on the
  first try. All other headers as expected: `.wasm` application/wasm on all 6 links; the content file "public,
  max-age=31536000, immutable", the rest "public, max-age=0, must-revalidate"; noindex on all 4 preview links;
  iso-on's COOP "same-origin" and COEP "require-corp" on all 7 paths of both links; no COOP/COEP elsewhere.
  X-Check-Rule: "project-host" on production's own address; "version-host" on the 5 other links, the branch links
  included (R5-PREV-OQ6, recorded, judged nowhere).
- **Risk 13 (plan.md:1358) has happened:** production's one-off link carries noindex, so PM-0006-2's row is not met as
  written. Recorded "not met as written", never a pass; it goes to the draft's author with the report (ADR 0006
  :1409-1414) and holds the check's end until the draft is revised (:1401). The sitting continues; `restore-production`
  reads it again.
- **The first probe run** (started by hand by the session, `purpose: deletion`, after `round-a`'s verify): probe run
  #1, id 44355175073, created 17:08:00 UTC, completed 17:08:26, success; probe-results-44355175073 (artifact
  14452700323, zip sha256 4f37fba6fad4..., equal to its digest), probed_at 2026-11-18T17:08:20.713Z, read with jq from
  `.gate/preview-hosting-check/day1/probe-1/`. Rows, word for word in substance: setup (case "rehearsal", pull request
  1 closed and merged 14:57:11Z, hours_since_close 2.19) stable https://setup.shamble-preview-check.pages.dev/ status
  404, outcome "gone", expected "gone"; one-off https://1f6487e4.shamble-preview-check.pages.dev/ 404 "gone", expected
  "gone"; production (main) stable 200 "serves", expected "serves"; production one-off (0354374e) 200 "serves",
  expected null; iso-off and iso-on, both links, 200 "serves", expected "serves". Every row 1 try. So the script works
  against Cloudflare, and the setup upload is gone from both links (step 22's first outcome: nothing to do).
- **The hosting checklist, steps 1 to 3 (Brave, his PC)**: his message "all three opened and screenshotted for you"
  (2026-11-18 17:12:23 UTC, 8fce8e5a-1ebc-4993-ae7b-0aef9f8339c8), three screenshots, transcribed (the images stay
  out of the repository):
  - iso-off.shamble-preview-check.pages.dev: Build marker 2162db55d93f59cf2ed7e371a3915409f0b5a587; Build mode
    preview; crossOriginIsolated (page) false; crossOriginIsolated (worker) false; WebAssembly "ran (2 + 3 = 5)";
    Content file "content.ef463692.bin (65536 bytes)"; Status done. Resources: page "/", "/page.css", "/page.js",
    "/manifest.json", "/favicon.ico", "/content.ef463692.bin", "/worker.js"; worker "/worker.js", "/tiny.wasm".
  - iso-on.shamble-preview-check.pages.dev: the same marker; Build mode preview-isolated; page true; worker true;
    WebAssembly "ran (2 + 3 = 5)"; the same content file and size; Status done. Resources: page "/", "/page.css",
    "/page.js", "/manifest.json", "/content.ef463692.bin", "/favicon.ico", "/worker.js"; worker "/worker.js",
    "/tiny.wasm".
  - shamble-preview-check.pages.dev: the same marker; Build mode production; page false; worker false; WebAssembly
    "ran (2 + 3 = 5)"; the same content file and size; Status done. Resources: the same seven page paths as iso-on;
    worker "/worker.js", "/tiny.wasm".
  - No login screen appears in any screenshot; each shows the page itself. Every value is the expected one (ADR 0006
    :1444-1447). His "achieved"/"not yet" words: asked of him (the checklist has him record them).
  - TE-12 (the page's resource list against the manifest): the build's files the page and worker loaded are all
    in the manifest; "/favicon.ico" is the one extra entry, Brave's own icon request (no such file in the build; not
    fetched by check-headers, which reads the manifest's files).
- **The build count, first reading (checklist step 4, PM-0006-7)**: his screenshot (no words; 2026-11-18 17:19:10 UTC,
  e98551a5-b3fa-47c1-973b-4f89cd4e5544) of Cloudflare, "Workers &
  Pages > shamble-preview-check", tab "Deployments" (tabs: Deployments, Metrics, Custom domains, Settings), transcribed:
  "Production", "Domains: shamble-preview-check.pages.dev": Production, main, "preview-check run 443473515...",
  https://0354374e.shamble-preview-check.pages.dev, "16 minutes ago". "All deployments": Production main
  (0354374e, "16 minutes ago"); Preview iso-off ("preview-check run 443473515...", 6e61a4c4, "16 minutes ago"); Preview
  iso-on (e48ee07b, "16 minutes ago"); Production main (no message), https://77605f36.shamble-preview-check.pages.dev,
  "3 days ago" (his first upload when he made the project, plan A4). The setup upload (1f6487e4) is not listed (deleted
  15:04:38 UTC). **No count of builds is shown on this page.** The overview and the Metrics tab: asked of him before
  `uploads` starts.
- His message "achieved" with a screenshot (2026-11-18 17:21:40 UTC, 001061fb-f067-456c-b4f2-bfad995c4127):
  - **Hosting checklist steps 1, 2 and 3: "achieved"** (his word, in answer to "Reply "achieved" for steps 1 to 3, or
    name any that are "not yet"").
  - The Metrics tab, transcribed: selector "Production", range "Last 24 hours"; Requests "Success 0", "Errors 0", "No
    data is available for this time range"; Subrequests 0; Errors: Internal 0, Script Threw Exception 0, Exceeded CPU
    Time Limits 0, Exceeded Memory 0, Client Disconnected 0; Median CPU Time and Request duration: every percentile 0.
    **No count of builds is shown on this tab.** (Requests 0 although production was fetched many times today: what
    this tab counts is not documented here; recorded, not explained.)
  - The Workers & Pages overview: not yet seen; asked of him before `uploads` starts (plan Part 3 C step 3 names it).
- His screenshot of the Workers & Pages overview (no words; 2026-11-18 17:23:36 UTC,
  33658e3d-8f06-4af5-ad77-4e4fae9bcbb4), transcribed: account "Shamble Frontlines"; "Workers & Pages", one application,
  shamble-preview-check (shamble-preview-check.pages.dev), "21m ago", "No Git connection", "Showing 1 - 1 of 1"; panel
  "Usage" (Today): "Requests today 0 / 100,000", "Observability events today 0 / 200,000"; "Account details": the
  account ID (shown in full on his screen, not copied here; its first and last 4 are [the account ID's first and last four characters, compared by him; not copied here], as the token check
  printed) and the workers.dev subdomain (not copied here); "Cloudflare Access": "Set up Zero Trust before requiring
  Access sign-in for Workers on this account." **No count of builds is shown.**
- **The first reading of the build count: "no count shown"** on all three places plan Part 3 C step 3 names (the
  project's Deployments page, its Metrics tab, the Workers & Pages overview).
- `uploads` started by the session right after (run_workflow, ref `main`, set `uploads`; 204 at about 17:23:55 UTC).
- `uploads` (PM-0006-7's 10 deploys): deploy run #3, id 44358217862 (created 17:23:54 UTC). plan green (17:23:58-17:24:16),
  build green (17:24:18-17:24:26; SIZES line identical to round-a's: "flagged":[], "count":9); the deploy job "waiting"
  on `preview` from 17:24:26; his message "approved and deployed" (2026-11-18 17:29:32 UTC,
  49b34555-967c-4301-89ab-9481638129cf); the deploy job's log runs 17:27:42-17:28:54 UTC (his approval just before
  17:27:42; one approval, one job).
  - The 10 uploads to branch `uploads`, each "✨ Success! Uploaded 0 files (8 already uploaded)" (0.22 to 0.40 sec),
    alias https://uploads.shamble-preview-check.pages.dev, environment "preview", marker 2162db55d93f...: 1 c28fc449
    (made 17:28:10.728Z), 2 deb45872 (17:28:14.759Z), 3 5b57fa5a (17:28:19.971Z), 4 f512e84d (17:28:24.211Z), 5
    283bba70 (17:28:30.563Z), 6 a845d2e3 (17:28:34.429Z), 7 37ac53a6 (17:28:38.498Z), 8 51e10edc (17:28:42.226Z), 9
    ff3ec055 (17:28:46.114Z), 10 44dd439a (17:28:52.521Z) (full IDs in deployment-record-44358217862-uploads, artifact
    14451668742).
  - verify (17:28:59-17:29:23; headers-44358217862, artifact 14453018448, zip sha256 1f38f9088428..., equal to its
    digest): "summary: {"records":10,"pass":20,"fail":0,"no_reading":0}", exit 0: every upload passes on its one-off
    link and on the branch link (the branch link read once per record), X-Check-Rule "version-host" on all.
  - Run conclusion: success. The second build-count reading asked of him now.
- **The build count, second reading**: his screenshot of the project's Deployments page (2026-11-18 17:33:15 UTC,
  7442b908-6d1c-48ba-a65b-36944fd3247f; his words "When are we done with these hideous tasks"), transcribed:
  Production main 0354374e "30 minutes ago"; "All deployments": ten Preview rows, branch `uploads`, "preview-check run
  443582178...", 44dd439a, ff3ec055, 51e10edc, 37ac53a6, a845d2e3, 283bba70, f512e84d, 5b57fa5a ("4 minutes ago"),
  deb45872, c28fc449 ("5 minutes ago"); then Production main 0354374e, Preview iso-off 6e61a4c4 ("31 minutes ago"), the
  rest below the screenshot's edge. The ten uploads are listed, matching the records one for one. **No count of builds
  is shown.** The overview was not read again: at the first reading it had no count field at all (only "Requests
  today" and "Observability events today"), and he was not asked twice. So PM-0006-7 reads "no count shown" before
  and after, on the places plan Part 3 C step 3 names; the deployments list shows the ten uploads.
- `bites` started by the session right after (run_workflow, ref `main`, set `bites`; 204 at about 17:33:50 UTC).
- `bites` (PM-0006-2's COEP bite, PM-0006-4's header bite, PM-0006-5's size bite): deploy run #4, id 44360058220
  (created 17:33:49 UTC). plan green; builds green (17:34:15-17:34:40). SIZES lines: bite-oversize "largest:
  oversize.bin, 27262976 bytes; limit 26214400 bytes; flagged: 1" / "SIZES {"largest":{"path":"oversize.bin","size":
  27262976},"limit":26214400,"flagged":[{"path":"oversize.bin","size":27262976}],"count":10}" (the size bite caught by
  the size check). The three deploy legs "waiting" on `preview` from 17:34:40; they started at 17:50:20 (oversize),
  17:50:21 (nocache) and 17:50:22 (coep) UTC: one approval (his), just before his message of 17:50:30.
  - bite-oversize: Wrangler refused it, word for word: " ⛅️ wrangler 4.156.0" / "✘ [ERROR] Error: Pages only supports
    files up to 25 MiB in size" / "  oversize.bin is 26 MiB in size" / "##[error]Process completed with exit code 1."
    No upload, no record (if-no-files-found: warn). The refusal comes before any call to Cloudflare: the log shows no
    account name, no account ID and no error code (the account-printout question: nothing printed on this path).
  - bite-nocache: "✨ Success! Uploaded 1 files (7 already uploaded) (0.99 sec)"; deployment
    29622b3b-8d8c-4d66-b9d5-2a67e5207523, alias https://bite-nocache.shamble-preview-check.pages.dev, "preview".
  - bite-coep: "✨ Success! Uploaded 1 files (7 already uploaded) (0.86 sec)"; deployment
    0b844b60-0458-44ac-a22e-d31ed505691d, alias https://bite-coep.shamble-preview-check.pages.dev, "preview".
  - verify (17:51:00-17:51:23; headers-44360058220, artifact 14456636722, zip sha256 c3bcd2713ff6..., equal to its
    digest): "summary: {"records":2,"pass":0,"fail":4,"no_reading":0}", exit 1. bite-coep, both links: only "/worker.js:
    COEP "", expected "require-corp"" (the COEP bite caught); bite-nocache, both links: only "/content.ef463692.bin:
    content file Cache-Control "no-cache", expected "immutable"" (the cache bite caught, header half). Nothing else
    failed. Run conclusion: failure, as planned: all three bites caught.
- `browsers` started by the session (run_workflow, ref `main`; 204 at about 17:55:39 UTC), after the records showed
  bite-coep and bite-nocache uploaded; production is still round-a's good build (0354374e).
- **`browsers`** (PM-0006-3 and PM-0006-4): browsers run #1, id 44364041726, started by the session 17:55:38 UTC,
  completed 17:59:13, conclusion failure (the macOS job). Artifacts read with jq from
  `.gate/preview-hosting-check/day1/browsers/` (zip sha256: windows f52bfb919353..., macos 4ca00fa3669e..., each equal
  to its digest).
  - windows-2025 (image win25-vs2026 20261103.298.1, WebdriverIO 10.1.0, ran_at 17:59:08.392Z): "summary":
    {"rows":51,"unexpected":0,"no_reading":0}. Chrome 156.0.8078.80, Edge 156.0.4314.58, Firefox 159.0.1, each link 3
    loads: production false/false, iso-off false/false, iso-on true/true, bite-nocache false/false ("as expected");
    bite-coep page true, worker "not reported (worker error: its script did not load)" ("bite caught", all 3 browsers,
    all 3 loads). PM-0006-4 in Chrome: production's content file on the second visit transferSize 0 (3 of 3, "as
    expected"); bite-nocache 300 (3 of 3, "bite caught"). Every page row: marker 2162db55d93f..., status done, "ran (2 +
    3 = 5)"; the resource lists hold the build's files plus "/favicon.ico" (as in Brave).
  - macos-26 (image macos26 20261102.0473.1, ran_at 17:56:16.077Z): "summary": {"rows":15,"unexpected":0,
    "no_reading":14}, exit 3. Safari 27.2, production load 1: false/false, "as expected". The other 14 loads: "no
    reading", error "There is already a Safaridriver instance running on port 49178!". Cause (read in tools/wdio/run.mjs
    at 2162db5, lines 44-50): withSession() starts a new WebDriver session with remote() for every load and deletes it
    afterwards; the second Safari session could not start while the first safaridriver still ran. A script fault, not a
    Cloudflare reading: PM-0006-3's Safari row has 1 reading of 15 and stays open ("no reading" never passes).
- His untick (REVIEWER (b)): his message "done" with a screenshot (2026-11-18 18:08:10 UTC,
  a23e7d93-5eb1-4093-a99d-e64ebb1df1b8), transcribed: banner "Environment "preview" updated."; "Environments /
  Configure preview": "Required reviewers" unticked; "Wait timer" unticked; "Allow administrators to bypass configured
  protection rules" unticked; "Deployment branches and tags": "No restriction"; secrets CLOUDFLARE_ACCOUNT_ID and
  CLOUDFLARE_API_TOKEN, both "yesterday"; "This environment has no variables." Only the reviewer changed.
- `bite-production` started by the session after that (run_workflow, ref `main`, set `bite-production`); it does not
  wait (no reviewer).
- **`bite-production`** (PM-0006-2's noindex bite on production): deploy run #5, id 44366750977 (created 18:10:20 UTC).
  plan green (18:10:23-18:10:28), build green (18:10:31-18:10:51), the deploy job ran at once, no wait (18:10:54-
  18:11:16: the reviewer is off), deployment e9a37459-a342-4d33-ba73-32f5f3549ad7 (environment "production"). verify
  (18:11:18-18:11:41; headers-44366750977, artifact 14458320188, zip sha256 d1ddcad0f8f2..., equal to the log's
  digest): "summary: {"records":1,"pass":0,"fail":2,"no_reading":0}"; both production links
  (https://shamble-preview-check.pages.dev/ and https://e9a37459.shamble-preview-check.pages.dev/) fail on all 7 paths
  with "production carries X-Robots-Tag noindex", and on nothing else: the bite caught on production's own address.
- `restore-production` started by the session right after (run_workflow, ref `main`, set `restore-production`).
- **`restore-production`**: deploy run #6, id 44367096948 (created 18:12:13 UTC); plan, build and deploy green, no
  wait; deployment de1c3e5e-1abb-4c8c-af5f-fb49165914d7 (record artifact created 18:13:27 UTC). verify (headers-44367096948,
  artifact 14458520412, zip sha256 67308e8e565f..., equal to its digest; checked_at 18:13:45.151Z): "summary":
  {"records":1,"pass":1,"fail":1,"no_reading":0}: production's own address passes (no noindex: the bite is gone);
  the one-off link https://de1c3e5e.shamble-preview-check.pages.dev/ fails on all 7 paths with "production carries
  X-Robots-Tag noindex": Risk 13 again, as round-a showed. Production carried the planted noindex from the bite's
  upload (between 18:11:02 and 18:11:13 UTC) to the restore's (by 18:13:27 UTC): about 2 minutes.
- **Sitting 1's readings in short**: Brave steps 1-3 achieved (his word); PM-0006-2: every header as expected on 15
  preview links (24 readings: round-a 4, uploads 20; counted with jq from the headers files) and on production's own
  address (round-a and restore-production), the two planted mistakes caught (COEP on the worker; noindex on
  production), Risk 13 on production's one-off link (twice); PM-0006-3: Chrome, Edge and Firefox as expected on all
  five links (bite caught), Safari 1 reading of 15 (script fault, open); PM-0006-4: Chrome 0 bytes on production's
  second visit, 300 on the no-cache bite (caught), and the header half caught on both links; PM-0006-5: the 26 MiB file
  flagged by the size check and refused by Wrangler; PM-0006-7: "no count shown" before and after, ten uploads listed.
  Approvals clicked in sitting 1: 3 (round-a, uploads, bites), one per run whatever its legs; then REVIEWER (b).
- The hourly probe: the 17:23 UTC slot passed without a scheduled run as well (list of probe.yml runs at about 18:14
  UTC: only run #1, started by hand). Three slots missed since the merge: 15:23, 16:23, 17:23.
- **Sitting 1's runs against its list** (all runs of the repository, read at about 18:16 UTC: total_count 10): the
  five listed deploy runs #2 round-a (44347351522), #3 uploads (44358217862), #4 bites (44360058220), #5
  bite-production (44366750977), #6 restore-production (44367096948); the two that start without him, browsers #1
  (44364041726) and probe #1 (44355175073, started by hand, told him); all on `main` 2162db55d93f, actor and
  triggering actor Mergincanli. The other three runs are the rehearsal's and the token check's (before sitting 1).
  Nothing ran that was not on the list.

##### Day 1, sitting 2 (PREV-DELETE's cases; REVIEWER (b): no approvals)
- His list sent at about 18:25 UTC (runs that will start; his two merges; the PR-page look); the go by this session's
  scheduled check-in (fired 18:27:22 UTC).
- The six branches pushed at 18:27:40 UTC (one `git push`, new branches): cs/pd-one a2cfef9, cs/pd-three d78dfb4
  (push 1), cs/pd-open 236a6d7, cs/pd-off 683e8c0, cs/pdt-one 7370d4b, cs/pdt-three 9ebeeb6 (push 1). Each commit's
  workflows hashed before the push: deploy 29593af18d34, close 3afdf389d636 (absent on 683e8c0, case (d), by design),
  probe f24ad6e81daf, browsers 6791076916ae; each adds only `test-prs/<branch>.txt`.
- The six pull requests opened right after with the GitHub MCP tool: #3 (a) cs/pd-one, #4 (b) cs/pd-three, #5 (c)
  cs/pd-open, #6 (d) cs/pd-off, #7 (a') cs/pdt-one, #8 (b') cs/pdt-three.
- Six deploy runs started on the pull_request event, none waiting (list read 18:28:28 UTC): #7 44370016948 (a) cs/pd-one
  (head a2cfef9, PR 3, created 18:28:02), #8 44370024556 (b) cs/pd-three push 1 (d78dfb4, PR 4, 18:28:05), #9
  44370034770 (c) cs/pd-open (236a6d7, PR 5, 18:28:08), #10 44370043893 (d) cs/pd-off (683e8c0, PR 6, 18:28:11), #11
  44370054922 (a') cs/pdt-one (7370d4b, PR 7, 18:28:15), #12 44370062021 (b') cs/pdt-three push 1 (9ebeeb6, PR 8,
  18:28:17). No other workflow started (no close run).
- **PM-0006-10 (does GitHub show the preview link on the pull request's page?): yes, logged out.** The session's
  picture of pull request #3's public page (headless Chromium, no login, 2026-11-18 18:29:33 UTC, HTTP 200;
  `.gate/preview-hosting-check/day1/pr-page/pr3.png`, 2560x2284, kept out of the repository; its text in pr3.json),
  transcribed: in the timeline, "Mergincanli deployed to preview now — with GitHub Actions" and a "View deployment"
  button; below, "This branch was successfully deployed", "1 active deployment", "Show environments". The page's links
  "preview" and "View deployment" both point to https://cs-pd-one.shamble-preview-check.pages.dev/ (the branch link,
  the deploy job's environment url). His logged-in page: asked of him (one line). (The deploy job holds no
  `deployments: write`: the link shows without it, as the security-auditor's L-finding asked to learn.)
- The six first runs, all conclusion success (verify exit 0: every link of the upload passed; list read 18:30:40 UTC):
  #7 (a) 18:28:02-18:29:24, #8 (b) push 1 18:28:05-18:29:30, #9 (c) 18:28:08-18:29:32, #10 (d) 18:28:11-18:29:14,
  #11 (a') 18:28:15-18:29:36, #12 (b') push 1 18:28:17-18:29:57 (created to last update, UTC). What was read of the
  ten test runs: their conclusions (all success, so verify passed every link) and, after the closes, their upload
  records through the probe: each cleanup's "Delete <id>" lines matched against probe-2's record rows per branch with
  jq (no new download): (a) 1b3c905e; (a') 672b3d0e, tombstone 2b214ae6 kept; (b) 03630e1d, f1e112f2, f516ed27; (b')
  6b6415e5, 5ad91df8, ed9c2421, tombstone 5ff46e6d kept: the deleted ids equal the recorded uploads, branch by branch.
  The ten headers files themselves were not downloaded.
- Push 2 of (b) and (b'), after each branch's first verify had finished (headers-44370024556 at 18:29:27,
  headers-44370062021 at 18:29:54): `git push` 18:30:19 UTC, cs/pd-three d78dfb4..2244b55, cs/pdt-three
  9ebeeb6..51130dd (both commits hashed before: the approved texts). Runs #13 44370443574 (b) and #14 44370444666
  (b'), created 18:30:25, no wait.
- Push 2 results: run #13 (b) verify done 18:31:23 UTC (headers-44370443574), run #14 (b') 18:31:33
  (headers-44370444666). Push 3, each after its branch's push-2 verify: cs/pd-three 2244b55..12f71d3 at 18:31:44 UTC,
  cs/pdt-three 51130dd..6188812 at 18:31:55 UTC (both hashed before: the approved texts).
- Push 3 results: run #15 (b) 44370706631 success (18:31:50-18:33:08 UTC), run #16 (b') 44370744826 success
  (18:32:02-18:33:21). **All ten test uploads passed verify** (ten runs, conclusion success).
- **The closes (the 24-hour clocks start here)**, by the session with the GitHub MCP tool (closed_at from each pull
  request's data): (a) #3 closed 2026-11-18T18:32:51Z; (a') #7 18:32:54Z; (d) #6 18:32:56Z; none merged, branches
  kept. (a)'s and (a')'s pushes had finished verify before; (b)'s and (b')'s push-3 runs were still running on other
  branches.
- Cleanup runs: close #2 44370905082 for (a) (cs/pd-one), created 18:32:54, success at 18:33:08; close #3 44370914405
  for (a') (cs/pdt-one), created 18:32:57, running at 18:33:30. **No cleanup run for (d)** (cs/pd-off): close.yml
  deleted on its branch, as planned (Risk 10 did not happen so far; the close.yml run list holds no cs/pd-off run).
- His message (2026-11-18 18:52:12 UTC, 8cc8eb72-82ae-48ed-ac2a-baaea512ac7a), word for word: "1. I think I do" /
  "2. merged" / "3. Merged"; then, queued while I worked, a screenshot of pull request #8 (18:52:36 UTC, attachment
  f154b837-2e59-4a7e-b81f-27873b4c7c97), transcribed: "Merged",
  "Mergincanli merged in 347d5e0 1 minute ago. Delete the cs/pdt-three branch to tidy up." with the buttons "Delete
  branch" and "Revert"; the sidebar "Deployments": "preview", "1 minute ago", with an open-link icon. **PM-0006-10,
  logged in: the page shows the deployment ("Deployments: preview" with a link)**; his words "I think I do".
- The merges (from the pull requests' data): (b) #4 merged 2026-11-18T18:50:33Z (merge commit 756d33f), (b') #8 merged
  18:50:43Z (347d5e0), both by Mergincanli. **The branches were not deleted** (git ls-remote at 18:52:58 UTC still
  lists cs/pd-three 12f71d3 and cs/pdt-three 6188812): his "Delete branch" asked again.
- `main` re-hashed after the merges (347d5e0): deploy 29593af18d34, close 3afdf389d636, probe f24ad6e81daf, browsers
  6791076916ae (unchanged); `git diff --name-status 2162db5 347d5e0`: only "A test-prs/pd-three.txt", "A
  test-prs/pdt-three.txt".
- The cleanups, word for word from their logs:
  - (a) close #2 44370905082: "Pull request closed at 2026-11-18T18:32:51Z (merged: false); this job started at
    2026-11-18T18:33:03Z." / "Uploads of cs/pd-one found: 1" / "Delete 1b3c905e-aa4c-4832-bb02-a0337ea78361
    (https://1b3c905e.shamble-preview-check.pages.dev): HTTP 200 {"success":true,"errors":[]}" / "This job ended at
    2026-11-18T18:33:05Z."
  - (a') close #3 44370914405: the tombstone "✨ Success! Uploaded 1 files (0.90 sec)", deployment
    2b214ae6-2566-4331-b348-c89e074b3764, alias https://cs-pdt-one.shamble-preview-check.pages.dev, made
    18:33:22.294Z; "Pull request closed at 2026-11-18T18:32:54Z (merged: false); this job started at
    2026-11-18T18:33:22Z." / "Uploads of cs/pdt-one found: 2" / "Delete 672b3d0e-d806-45ea-a286-5c2b8229e095
    (https://672b3d0e.shamble-preview-check.pages.dev): HTTP 200 {"success":true,"errors":[]}" / "Kept (the
    tombstone): 2b214ae6-2566-4331-b348-c89e074b3764" / "This job ended at 2026-11-18T18:33:23Z."
  - (b) close #4 44374077151: "Pull request closed at 2026-11-18T18:50:33Z (merged: true); this job started at
    2026-11-18T18:50:46Z." / "Uploads of cs/pd-three found: 3" / "Delete 03630e1d-0ce2-4de1-a230-0a752eb81165 ...:
    HTTP 200 ..." / "Delete f1e112f2-eefd-441c-b2c3-ad6d0201a2be ...: HTTP 200 ..." / "Delete
    f516ed27-3513-4737-a5b1-6a60e51ebd88 ...: HTTP 200 ..." / "This job ended at 2026-11-18T18:50:47Z."
  - (b') close #5 44374107847: the tombstone "✨ Success! Uploaded 0 files (1 already uploaded) (0.30 sec)",
    deployment 5ff46e6d-ea01-4c0e-b4e3-c0d46e50dc1c, alias https://cs-pdt-three.shamble-preview-check.pages.dev, made
    18:51:05.446Z; "Pull request closed at 2026-11-18T18:50:43Z (merged: true); this job started at
    2026-11-18T18:51:05Z." / "Uploads of cs/pdt-three found: 4" / three deletes, HTTP 200 each (6b6415e5..., 5ad91df8...,
    ed9c2421...) / "Kept (the tombstone): 5ff46e6d-ea01-4c0e-b4e3-c0d46e50dc1c" / "This job ended at
    2026-11-18T18:51:08Z."
  - "Uploads found" against the records (F3/TE-7): (a) 1 = 1 upload; (a') 2 = 1 upload + the tombstone; (b) 3 = 3
    uploads; (b') 4 = 3 uploads + the tombstone. From close to deletion: (a) 14 s, (a') 29 s, (b) 14 s, (b') 25 s.
  - No cleanup run for (d). Every cleanup ran at once (no reviewer).
- A probe started by hand right after the last cleanup (run_workflow, `purpose: deletion`).
- **The probe right after the cleanups**: probe run #2, id 44374666257 (by hand, `purpose: deletion`, head 347d5e0),
  created 18:53:54 UTC, success 18:54:34; probe-results-44374666257 (artifact 14461031992, zip sha256 db8b83a018fe...,
  equal to its digest), probed_at 2026-11-18T18:54:22.157Z, read with jq from `.gate/preview-hosting-check/day1/probe-2/`.
  By case: (a) 2 rows, gone 2; (a') tombstone 3 rows, gone 3; (b) 4 rows, gone 4; (b') tombstone 5 rows, gone 5; (c)
  control 2 rows, serves 2; (d) bite 2 rows, serves 2; the by-hand previews 19 rows, serves 19; the rehearsal 2 rows,
  gone 2; production 4 rows, serves 4 (its own address as expected; the one-off links of the three production uploads
  0354374e, e9a37459 (the noindex bite) and de1c3e5e still serve, "expected" null: no row judges them). Every judged
  row as expected. So 22 to 3 minutes after their closes, every link of (a), (a'), (b) and (b') is already gone; the
  mark itself is read at 24 hours.
- The hourly schedule: the 18:23 UTC slot passed without a scheduled run as well (four slots missed: 15:23, 16:23,
  17:23, 18:23). Stand-ins: a by-hand probe about an hour later (check-in at 19:55 UTC) and the 24-hour probe (check-in
  at 2026-11-19 18:55 UTC, after the last close's 24-hour mark, 18:50:43Z).
- **Sitting 2's runs against its list** (all runs of the repository, total_count 25 at about 18:56 UTC): the ten
  deploy runs #7-#16 (six first pushes, two second, two third), the four cleanup runs #2-#5 ((a), (a'), (b), (b')),
  the by-hand probe #2; with the ten runs before sitting 2, 25. No cleanup run for (d), no scheduled probe, nothing
  else. All by Mergincanli.
- His message "I have deleted both branches 4 and 8," / "You can do sitting 3" (2026-11-18 18:58:58 UTC,
  49c5e112-f261-48e8-9b31-ee3d56bf008b):
  git ls-remote at 18:59:29 UTC lists no cs/pd-three and no cs/pdt-three (deleted); cs/pd-one, cs/pd-off, cs/pd-open,
  cs/pdt-one, check-token, setup and main (347d5e0) remain. Case (b) and (b'): merged, branch deleted, as planned.

##### Day 1, sitting 3 (the token tests, PM-0006-32; consultation 20's order)
- His list sent at about 19:06 UTC (the order: off, on; "say "plan order" if you prefer"); no reply before the go; the
  go by this session's scheduled check-in (fired 19:10:59 UTC).
- Step 1 (the reviewer off since his untick, 18:08 UTC): cs/b9-edit 7f3a684 pushed at 19:11:17 UTC (deploy.yml = the
  approved variant fd3dc4c6029b; the other three workflows unchanged); pull request #9 "Token test: cs/b9-edit, the
  deploy job rewritten" opened right after.
- **Step 1's outcome row hit: "the off run waits".** Run #17 (44377870587): plan green (19:11:37-19:11:54 UTC), build
  green (19:11:58-19:12:20), then the deploy job (135062245511) "waiting" from 19:12:20 UTC, although the reviewer was
  off at his untick (18:08 UTC) and every run since then ran without waiting (the last: close #5 for (b'), 18:50:47 to
  18:51:11). So `preview` changed after 18:51 UTC; the likely reading (not yet shown by a record) is that he ticked
  "Required reviewers" after my list of about 19:06 UTC (its step 2). Per the row: stop, his preview page read (a
  screenshot asked). If it shows the reviewer on, this run is the control (approve; "secret: present") and the
  reviewer-off reading is taken in the plan's order (untick, re-run, re-tick).
- Check-in (fired 19:55:21 UTC, read 19:55:47): run #17's deploy job still "waiting" (no message from him since my
  question of about 19:13 UTC); the 19:23 UTC slot passed without a scheduled probe run (five slots missed:
  15:23-19:23; 26 runs in all, the 26th being run #17). A probe started by hand (`purpose: deletion`) as the hourly
  stand-in; it holds no secret and names no environment, so it does not touch the waiting run.
- **The hourly stand-in probe**: probe run #3, id 44385645676 (by hand, head 347d5e0), created 19:56:06 UTC, success
  19:56:56; probe-results-44385645676 (artifact 14466161509, zip sha256 5136260483b7..., equal to its digest),
  probed_at 2026-11-18T19:56:45.076Z, read with jq from `.gate/preview-hosting-check/day1/probe-3/`. By case: (a) 2
  rows gone (hours_since_close 1.4); (a') tombstone 3 rows gone (1.4); (b) 4 rows gone (1.1); (b') tombstone 5 rows
  gone (1.1); (c) control 2 rows serves; (d) bite 2 rows serves (1.4); the by-hand previews 19 rows serves;
  production 4 rows serves (its own address as expected; three one-off links unjudged); the rehearsal 2 rows gone
  (4.99). No row "not as expected" (as_expected false: 0). Nothing to tell him (the check-in's rule).
- His screenshot of `preview` with "approved the run also: https://github.com/Mergincanli/shamble-preview-check/actions/runs/44377870587"
  (2026-11-18 23:15:46 UTC, e6d8a826-ad3a-412f-be18-8ec09bc13788), transcribed: "Environments / Configure preview";
  "Required reviewers" ticked, "Add up to 5 more reviewers", Mergincanli listed; "Prevent self-review" unticked; "Wait
  timer" unticked; "Allow administrators to bypass configured protection rules" unticked; "No restriction"; secrets
  CLOUDFLARE_ACCOUNT_ID and CLOUDFLARE_API_TOKEN, "yesterday"; "This environment has no variables." So the reviewer
  was on when run #17 started (he ticked it after my list of about 19:06 UTC; when exactly, no record shows). The
  reviewer-off window (his untick 18:08 UTC) closed at that tick, at some time between 18:51:11 (the last run that ran
  without waiting) and 19:12:20 UTC (run #17's wait).
- **The control (PM-0006-32's first row): run #17's deploy job waited on `preview` from 19:12:20 UTC, he approved it,
  and it ran at 23:15:23 UTC**, word for word: "B9-MARKER: the rewritten deploy job ran at 2026-11-18T23:15:23Z" /
  "secret: present" (the env block shows "TOKEN: ***": the value masked). So with the reviewer on, a rewritten job
  waits, and after the approval it reaches the token. Sitting 3 now goes in the plan's order (untick, re-run without a
  wait, re-tick; SA-R4's re-run must wait and he rejects it; then cs/b9-prod).
- His message (2026-11-19 00:10:53 UTC, 5682ca4d-c2ca-4cca-924f-3d1539190ac9), word for word in part: "I have unticked
  Required Reviewers and saved it" / ""Repository secrets" list says  "This repository has no secrets"." (his words; no
  screenshot of that list came with it: SA-R8 asks for a screenshot, asked for with the re-tick) / "Open
  https://claude.ai/customize/connectors. I have opened it and the connection to shamble frontlines is working"; his
  screenshot is of claude.ai's GitHub plugin page ("GitHub account connected"; "Claude GitHub App", "Cloud sessions
  only", Mergincanli "Installed"; "Check repository status": "Claude Code cloud sessions can work in
  Mergincanli/Shamble-Frontlines"; "Connectors: This plugin doesn't include any connectors."): the main GitHub
  connection, not a way past the proxy's 403.
- **Window opened: the reviewer off on `preview`** from his "unticked" (00:10:53 UTC). Step 2 (the reviewer bite):
  run #17 re-run by the session with rerun_workflow_run (201 Created, about 00:11 UTC): attempt 2; its deploy job must
  run without waiting and print "secret: present".
- **The reviewer bite (PM-0006-32's second row): run #17 attempt 2** (re-run 00:11:09 UTC): plan green
  (00:11:13-00:11:32), build green (00:11:34-00:11:48); the deploy job 135234549298 created 00:12:06, status "queued"
  (never "waiting"), started 00:12:13, completed 00:12:15, success; its log, word for word: "B9-MARKER: the rewritten
  deploy job ran at 2026-11-19T00:12:14Z" / "secret: present" ("TOKEN: ***" in the env block). So with the reviewer
  off, a rewritten job reaches the token without anyone's click. Next: his re-tick (the window closes at its
  screenshot), then SA-R4's re-run, which must wait and which he rejects.
- **His re-tick**: screenshot (2026-11-19 00:13:42 UTC, ed9f3bac-f2e0-4cf1-a61b-05a1fd4c4dc0), transcribed: banner
  "Environment "preview" updated."; "Required reviewers" ticked, Mergincanli listed; "Prevent self-review" unticked;
  "Wait timer" unticked; "Allow administrators to bypass configured protection rules" unticked; "No restriction";
  the two secrets, "yesterday". **Window closed: the reviewer was off from 00:10:53 to about 00:13:42 UTC** (his two
  messages), and only run #17's attempt 2 ran in it (the session pushed nothing else).
- SA-R4 (the re-tick proven by a wait): run #17 re-run again by the session (rerun_workflow_run, 201 Created, about
  00:13:54 UTC): attempt 3; its deploy job must wait on `preview`, and he rejects it.
- **SA-R8**: his screenshot of "Actions secrets and variables" (queued 2026-11-19 00:14:02 UTC, attachment
  7f9bc63e-b2cf-4669-9886-807ca00a3403), transcribed: "Environment secrets": CANARY (production, "3 days ago"),
  CLOUDFLARE_ACCOUNT_ID (preview, "yesterday"), CLOUDFLARE_API_TOKEN (preview, "yesterday"); "Repository secrets":
  "This repository has no secrets." Met: no secret outside the two environments.
- **SA-R4 measured: run #17 attempt 3** (re-run 00:13:52 UTC): plan green (00:13:56-00:14:11), build green
  (00:14:13-00:14:37); the deploy job 135235739456 created 00:14:38, read "queued" once in the jobs list, then
  "status":"waiting" (get_workflow_job, about 00:14:45 UTC). So after his re-tick a rewritten job waits again. His
  rejection asked next.
- His "rejected" (2026-11-19 00:15:52 UTC, 193acd50-d5b0-4df0-9bec-25a5e4c23951): attempt 3's deploy job completed
  00:15:49 UTC, conclusion "failure", runner_id 0 (it never reached a runner). SA-R4 done.
- Step 4 (the refusal): cs/b9-prod 9965630 pushed 00:16:15 UTC (deploy.yml = the approved variant 8b745631a278; the
  other three workflows unchanged); pull request #10 "Token test: cs/b9-prod, a pull-request job that names
  production" opened right after. Its job `b9-production` must be refused at once (production admits `main` only).
- **The refusal (PM-0006-32's third row): run #18** (44427671656, created 00:16:27 UTC): the job `b9-production`
  (135236608357) completed 00:16:31 UTC, conclusion "failure", no runner (refused before any step; no log); its check
  run's output empty. The public run page, logged out (headless Chromium, 2026-11-19 00:17:28 UTC, HTTP 200; picture
  `.gate/preview-hosting-check/day1/sitting3/run18.png`), "Annotations: 2 errors", word for word: "b9-production" /
  "Branch "refs/pull/10/merge" is not allowed to deploy to production due to environment protection rules." /
  "b9-production" / "The deployment was rejected or didn't satisfy other protection rules." So a pull-request job
  that names `production` is refused at once; nothing waited for him.
- His "open" with a screenshot (2026-11-19 00:19:35 UTC, 66a03064-486c-4491-9c5e-8bf4639d2ac9): banner "Environment changes successfully saved: all
  branches can deploy."; "Configure production": "Required reviewers" unticked, "Wait timer" unticked, "Allow
  administrators to bypass configured protection rules" unticked; "Deployment branches and tags": "No restriction";
  secret CANARY, "3 days ago". **Window opened: production's branch rule off.** Step 5: run #18's failed job re-run by
  the session (rerun_failed_jobs): "secret: present" expected.
- **The branch-rule bite (PM-0002-66's row, in PM-0006-32): run #18 attempt 2** (rerun_failed_jobs, about 00:19:45
  UTC): `b9-production` (135238126711) started 00:19:48, completed 00:19:51, success; its log, word for word:
  "B9-MARKER: the production job ran at 2026-11-19T00:19:49Z" / "secret: present" ("CANARY: ***"). So with
  production's branch rule set to "No restriction", a pull-request job reaches production's secret. His restore asked
  now (the window stays open until his screenshot shows `main`).
- **His restore**: screenshot (2026-11-19 00:21:51 UTC, c31773fa-3a4e-4172-b55f-1d158e4557f1), transcribed: banner
  "Deployment branch rule "main" saved successfully."; "Configure production": "Required reviewers", "Wait timer" and
  "Allow administrators to bypass configured protection rules" unticked; "Deployment branches and tags": "Selected
  branches and tags", "1 branch and 0 tags allowed", rule `main`, "Currently applies to 1 branch"; secret CANARY.
  **Window closed: production's branch rule was off from 00:19:35 to about 00:21:51 UTC**; only run #18's attempt 2
  ran in it. The measurement: run #18 re-run (rerun_workflow_run, 201, about 00:22:00 UTC): attempt 3, its
  `b9-production` must be refused again.
- **The measurement after the restore: run #18 attempt 3** (re-run about 00:22:00 UTC): `b9-production` (135239185962)
  created 00:22:02, completed 00:22:06, conclusion "failure", no runner; the public run page for attempt 3 (logged
  out, 00:22:39 UTC; picture `.gate/preview-hosting-check/day1/sitting3/run18-attempt3.png`), "Annotations: 2 errors",
  word for word: "Branch "refs/pull/10/merge" is not allowed to deploy to production due to environment protection
  rules." / "The deployment was rejected or didn't satisfy other protection rules." So with `main` back, the
  pull-request job is refused again.
- **Sitting 3 in short (PM-0006-32 and SA-R4, SA-R8):** reviewer on: the rewritten job waited, approved, "secret:
  present" (#17 attempt 1); reviewer off: no wait, "secret: present" (attempt 2); reviewer on again: it waited and
  was rejected (attempt 3, no runner); a pull-request job naming production: refused at once (#18 attempt 1); the
  branch rule off: "secret: present" (attempt 2); `main` back: refused again (attempt 3); no repository secrets. Two
  windows, each closed by his screenshot: the reviewer off 00:10:53-00:13:42 UTC, production's rule off
  00:19:35-00:21:51 UTC. State left: `preview` with the reviewer (Mergincanli) on, `production` with `main` only;
  pull requests #5 (c), #9 (cs/b9-edit) and #10 (cs/b9-prod) open, for Day 2.
- **The schedule fired once**: probe run #4, id 44413340706, event "schedule", created 2026-11-18 22:37:26 UTC (the
  22:23 slot, 14 minutes late), success 22:37:53; probe-results-44413340706 (artifact 14478546185, zip sha256
  f497337be4af..., equal to its digest), probed_at 2026-11-18T22:37:46.669Z, read with jq from
  `.gate/preview-hosting-check/day1/probe-4/`: (a) 2 rows gone (hours_since_close 4.08); (a') 3 gone (4.08); (b) 4
  gone (3.79); (b') 5 gone (3.78); (c) 2 serves; (d) 2 serves (4.08); the by-hand previews 19 serves; production 4
  serves; the rehearsal 2 gone (7.68); no row not as expected. The slots 15:23 to 21:23 UTC (seven) had no run; the
  23:23 slot had none by 2026-11-19 00:23 UTC (29 runs in all).
- **Sitting 3's runs against its list** (29 runs at about 00:23 UTC): run #17 (cs/b9-edit, three attempts: the
  control, the reviewer bite, SA-R4's rejected one), run #18 (cs/b9-prod, three attempts: refused, the branch-rule
  bite, refused), the by-hand probe #3 and the scheduled probe #4; nothing else.
- **Direct records (logged-out public pages, 2026-11-19 00:26-00:27 UTC, pictures in
  `.gate/preview-hosting-check/day1/sitting3/`):** run #17 attempt 1: "Deployment protection rules" / "Mergincanli" /
  "approved" / "preview"; attempt 2: no "Deployment protection rules" section at all (no approval event: nobody
  clicked); attempt 3: "Mergincanli" / "rejected" / "preview", annotation "The deployment was rejected or didn't
  satisfy other protection rules." **The environments' deployment history for the token tests**, as the pull request
  pages show it (the repository's deployments pages answer 404 when logged out): #9 (cs/b9-edit): "Mergincanli
  deployed to preview — with GitHub Actions Active" (twice), then "Mergincanli had a problem deploying to preview —
  with GitHub Actions Failure"; #10 (cs/b9-prod): "Mergincanli had a problem deploying to production — with GitHub
  Actions Failure", "Mergincanli deployed to production — with GitHub Actions Active", "Mergincanli had a problem
  deploying to production — with GitHub Actions Failure" (attempts 1, 2, 3 in that order).
- **PM-0006-32 clause by clause** (ADR 0006 :1379; plan.md:103 and :130):
  - "a test pull request ... whose deploy job is edited to print a marker gets no secret until the `preview`
    environment's required reviewer approves it (the job prints only whether the secret is set, never its value)":
    met (#17 attempt 1 waited 19:12:20 to 23:15, approved, "secret: present"; nothing printed but presence).
  - "a job in a pull request that names the `production` environment is refused by its `main`-only deployment-branch
    rule": met (#18 attempt 1, and again attempt 3 after the restore).
  - "2 of 2": both met. Control (after approval, "secret set"): met. Bite (reviewer removed, "secret set"): met
    (attempt 2, no approval event). The plan's added PM-0002-66 bite (the rule removed, "present"): met (#18 attempt 2).
  - Reported, not a pass condition: a production deploy with the `preview` environment's token is NOT refused: round-a's
    production leg (deploy job in `preview`) made production deployment 0354374e (environment "production") on
    2026-11-18 17:02:25 UTC, as part 13 expected; for OQ-0006-13.
  - Recorded: how the rule treats a pull request's merge ref: it judges the ref "refs/pull/10/merge" and refuses it
    ("Branch "refs/pull/10/merge" is not allowed to deploy to production ...").
  - Evidence asked: each run's log (in this file) and the environments' deployment history (the pull request pages
    above). Still open for Day 2: "Mustafa's revocation of the test token when the check ends" (the revoke and the
    `revoked` run).

##### Day 2 (plan Part 3 D)
- **The schedule fired twice more overnight** (probe.yml's runs listed with the GitHub MCP tool, 2026-11-19
  13:39:21 UTC, transcript time, total_count 6; zips downloaded 13:39 UTC by the files' times, read with jq):
  - probe run #5, id 44457125198, event "schedule", head 347d5e0, created 2026-11-19 04:15:59 UTC, success 04:16:29;
    probe-results-44457125198 (artifact 14498371957, zip sha256 356c906a5d62..., equal to its digest), probed_at
    2026-11-19T04:16:20.864Z, `.gate/preview-hosting-check/day2/probe5/`: 43 rows. (a) 2 rows, gone 2
    (hours_since_close 9.72); (a') tombstone 3, gone 3 (9.72); (b) 4, gone 4 (9.43); (b') tombstone 5, gone 5 (9.43);
    (c) control 2, serves 2; (d) bite 2, serves 2 (9.72); the by-hand previews 19, serves 19; production 4, serves 4
    (its own address as expected; the one-off links of 0354374e, e9a37459 and de1c3e5e not judged); the rehearsal 2,
    gone 2 (13.32). Unexpected 0, no reading 0.
  - probe run #6, id 44533901175, event "schedule", head 347d5e0, created 13:29:10 UTC, success 13:29:43;
    probe-results-44533901175 (artifact 14534309293, zip sha256 00e8244c50ad..., equal to its digest), probed_at
    2026-11-19T13:29:36.470Z, `.gate/preview-hosting-check/day2/probe6/`: the same 43 rows with the same outcomes;
    hours_since_close (a) 18.95, (a') 18.95, (b) 18.65, (b') 18.65, (d) 18.94, the rehearsal 22.54. Unexpected 0, no
    reading 0.
  - The hourly schedule so far: 3 scheduled runs (created 2026-11-18 22:37:26, 2026-11-19 04:15:59 and 13:29:10 UTC)
    in the 23 slots from 2026-11-18 15:23 to 2026-11-19 13:23 UTC. The run data names no slot, so which slot each run
    stands for is not recorded.
- **PM-0006-1: the header checked again before the 24-hour reading** (2026-11-19 13:41-13:43 UTC, transcript times):
  - The test project: its production link serves (probes #5 and #6: its own address "serves", expected "serves").
  - The test repository: `main` is 347d5e04487189be05dd2bc3a2c397bed8415e4b (git ls-remote, 2026-11-19 13:41:31 UTC):
    the merges of (b) #4 and (b') #8 on 2162db5, which differ from it only by `test-prs/pd-three.txt` and
    `test-prs/pdt-three.txt` (git diff --name-status 2162db5 347d5e0). The four workflows at 347d5e0, re-hashed:
    deploy.yml 29593af18d34, close.yml 3afdf389d636, probe.yml f24ad6e81daf, browsers.yml 6791076916ae, equal to the
    header. Remote branches: check-token, cs/b9-edit 7f3a684, cs/b9-prod 9965630, cs/pd-off, cs/pd-one, cs/pd-open,
    cs/pdt-one, main, setup.
  - The build-marker rule: unchanged (deploy.yml unchanged).
  - The machines and versions: ubuntu-24.04 for the probes (the 24-hour probe's image version to be read from its log);
    the browser rows of sitting 1: windows-2025 (image win25-vs2026 20261103.298.1), Chrome 156.0.8078.80, Edge
    156.0.4314.58, Firefox 159.0.1, WebdriverIO 10.1.0; Safari left the check by his decision (2026-11-18 18:21:30 UTC).
    **Missing: his Brave version** (the header's "every browser's version"): asked on Day 2's list, before the wall.
  - The token: last measured 2026-11-18 14:27 UTC (Pages Read and Pages Write, not_before null, expires_on
    2026-11-25T22:59:59Z); not measured again (no run uses it before (c)'s cleanup).
  - Settings changed during the check: `preview`'s reviewer off from his untick (2026-11-18 18:08 UTC) until his tick
    (between 18:51:11 and 19:12:20 UTC by the run records; seen on in his screenshot of 23:15:46 UTC), and again
    2026-11-19 00:10:53-00:13:42 UTC (closed by his screenshot); `production`'s branch rule off 00:19:35-00:21:51 UTC
    (closed by his screenshot). State by his last screenshots: `preview` with reviewer
    Mergincanli (self-review and bypass off) since 00:13:42 UTC; `production` with `main` only since 00:21:51 UTC.
- **His message of 2026-11-19 14:09:28 UTC** (5d646c04-ecaf-4795-be37-ec9e201d024c), sent before the 24-hour reading
  and before my go for the wall (my list, sent 13:51:09 UTC by the transcript, put both visits "after my go"). His own lines, word for word
  (my quoted steps left out; the address's account ID not copied here): "Done:" / "I pressed try for free and got
  redirected to this page:" / "https://dash.cloudflare.com/<his account ID>/home" / "also done:" / "step 6: done, no
  second login" / "step 7: done, also load without any login" / "Achieved". One screenshot: the iso-off test page,
  Build marker 2162db55d93f59cf2ed7e371a3915409f0b5a587, Build mode preview, crossOriginIsolated (page) false and
  (worker) false, WebAssembly "ran (2 + 3 = 5)", Content file "content.ef463692.bin (65536 bytes)", Status done;
  resources page "/", "/page.css", "/page.js", "/manifest.json", "/favicon.ico", "/content.ef463692.bin",
  "/worker.js", worker "/worker.js", "/tiny.wasm". Not sent: a screenshot of the setting, the time, whether a login
  page appeared at step 5; step 8 not reported; his Brave version not sent.
- **Looked at from outside right after** (this container, curl with the probe's User-Agent, no cookies, 14:10:11-14:10:24
  UTC, transcript times): iso-off, iso-on, cs-pd-open (c), cs-pd-off (d), production, and the one-off links 6e61a4c4
  (iso-off) and 17d45ef9 (c): each HTTP 200 with no redirect, each page carrying its build marker (iso-off, iso-on,
  6e61a4c4: 2162db55...; (c) and 17d45ef9: 4a2b70464ed9...; (d): 5b965331aba9...). The access policy was not in
  effect for a stranger.
- **Probe run #7** (by hand, `purpose: deletion`, id 44541037774, head 347d5e0, created 14:11:20 UTC):
  probe-results-44541037774 (artifact 14539176107, zip sha256 a58091efb2ff..., equal to its digest), probed_at
  2026-11-19T14:11:37.363Z, `.gate/preview-hosting-check/day2/probe7/`: 43 rows, no `wall` row. (a) 2 gone (19.65 h);
  (a') 3 gone (19.65); (b) 4 gone (19.35); (b') 5 gone (19.35); (c) 2 serves; (d) 2 serves (19.64); the by-hand
  previews 19 serves; production 4 serves; the rehearsal 2 gone (23.24). Unexpected 0, no reading 0.
- Reading: the wall did not go on, so the 24-hour reading is not spoiled so far (its own (c) row stays the proof at
  18:55 UTC). His Brave steps 5 to 7 ran with no wall, so they are not the checklist's steps 5 to 7 (step 5 expects a
  Cloudflare login page first); they are done again once the wall is on. Whether his "try for free" made a Zero Trust
  organization, and what the project's access-policy setting shows now: not known.
- Guard check-in (fired 16:30:19 UTC): outside look 16:30:44-46 UTC, (c) and iso-off HTTP/2 200 with their markers,
  no redirect: no wall.
- **The 24-hour reading (PM-0006-6, PREV-DELETE)**: probe run #8, id 44593002877, started by the session (run_workflow,
  after the check-in trig_01Fd2yqXX6LL2QP4hLnrCuu6 fired at 18:55:35 UTC), `purpose: deletion`, head 347d5e0, created
  18:56:04 UTC, success 18:56:52; runner image ubuntu-24.04 version 20261116.133.1 (Ubuntu 24.04.5), Node v22.23.5;
  "Records found: 21"; probe-results-44593002877 (artifact 14564068902, zip sha256 a48470714d37..., equal to its digest
  and to the log's "SHA256 digest of uploaded artifact"), probed_at 2026-11-19T18:56:36.575Z, read with jq from
  `.gate/preview-hosting-check/day2/probe8-24h/` (the run's logs beside it). jq assertion: 18 closed-case rows, the
  smallest hours_since_close 24.1, all 24 or more: true. By case: (a) 2 rows, gone 2 (404, 404; 24.4 h); (a')
  tombstone 3, gone 3 (the stable link and the tombstone's one-off 2b214ae6 answer 200 without the marker, the deleted
  upload's one-off 672b3d0e 404; 24.4 h); (b) 4, gone 4 (404 each; 24.1 h); (b') tombstone 5, gone 5 (24.1 h); (c)
  control 2, serves 2 (200 with its marker, so the wall was off); (d) bite 2, serves 2 (24.39 h); the by-hand previews
  19, serves 19; production 4, serves 4 (its own address expected "serves"; the one-off links of 0354374e, e9a37459 and
  de1c3e5e not judged); the rehearsal 2, gone 2 (27.99 h). Unexpected 0, no reading 0, no `wall` row, every row 1 try.
  The log's last line: "probed 43 links for deletion; unexpected: 0; no reading: 0". An outside look from this
  container at 18:56:39 UTC (curl): (c), (d) and iso-off 200, no redirect.
- **PM-0006-6 read:** "24 hours after each test pull request closes, none of its links (the stable branch link and the
  one-off link of every upload it ever had) answers with that build": met for (a) and (b) without the tombstone, and
  for (a') and (b') with it; the control (c) still serves; the bite (d), closed with its cleanup switched off, still
  serves (it bites: without the cleanup a closed pull request's links stay up). Outcome: "all gone (pass)". The series
  behind it: probes #2 (by hand, 3 to 22 minutes after the closes), #3 (by hand, 1.1-1.4 h), #4 (scheduled, 3.8-4.1 h),
  #5 (scheduled, 9.4-9.7 h), #6 (scheduled, 18.7-19.0 h), #7 (by hand, 19.4-19.7 h) and #8 (by hand, 24.1-24.4 h):
  every closed case gone at each, (c) and (d) serving at each. A departure in the method, for the report: the draft's
  "a cookie-less probe every hour from the close to 24 h after" was not achieved; GitHub's hourly schedule fired 3
  times in the 28 slots from 2026-11-18 15:23 to 2026-11-19 18:23 UTC (none from 14:23 on), and 4 by-hand probes stood
  in. The 24-hour reading itself never rested on the schedule (plan Part 1.1, PM-0006-6, change (1)).
- **His answer to the go** (user message 2026-11-19 21:02:51 UTC, fb9888dc-68fd-4cb1-b747-8e1b0ec5c949; two
  screenshots), his words: "My brave version: 1.100.58 Chromium: 158.0.8127.42 (Official Build) (64-bit)" / (on the
  first screenshot asked) "I don't understand this one" / "When I press the link in the preview access it sends me to a
  page where it says try for free" / "I had done these steps yesterday idk why I have to do them again" / "Give me a
  refreshed list of things that needs to be done, step by step, strong explained straight to the point".
  - Screenshot 1, transcribed: the project's Settings tab (Deployments, Metrics, Custom domains, Settings), "General":
    Name shamble-preview-check; Production branch main; Notifications; **"Preview access"**: "Preview deployments are
    public by default. Restrict previews with Cloudflare Access so visitors must sign in before viewing them." / "You
    can customize who is allowed in Zero Trust." / "This protects preview deployment URLs only. Production pages.dev and
    custom domains are managed separately in Zero Trust.", with "Cloudflare Access" shown as a link and a button
    **"Restrict previews"** on the right; then the project's Delete row. Above: Runtime (Placement Default,
    Compatibility date 2026-11-15, Fail open). So the setting the plan calls "Enable access policy" (preview-deployments
    page as read 2026-11-15) is now "Preview access" with "Restrict previews"; it is still off.
  - Screenshot 2, transcribed: cloudflare.com/products/access/, Cloudflare's product page for Access ("Zero trust
    network access (ZTNA) for your self-hosted and SaaS applications"), buttons "Try it for free" and "View
    documentation". (His browser's bookmarks bar shows in it; not copied here.)
  - Reading: this afternoon's "try for free" was the product page's button, reached through the row's "Cloudflare
    Access" link, not the "Restrict previews" button; pressing it sent him, already signed in, to the dashboard's home.
    No plan, trial or payment screen appears in his account of it. The Brave version completes PM-0006-1's header.
- **PM-0006-1, the header's Brave line** (from his message above): Brave 1.100.58, Chromium 158.0.8127.42 (Official
  Build) (64-bit), on his Windows PC.
- **The control with the wall off, before his switch-on (PM-0006-8's control)**: probe run #9, id 44615090897, by hand,
  `purpose: deletion`, head 347d5e0, created 21:04:39 UTC, success 21:05:15; probe-results-44615090897 (artifact
  14574710921, zip sha256 ab7ee830af75..., equal to its digest), probed_at 2026-11-19T21:05:01.930Z,
  `.gate/preview-hosting-check/day2/probe9-control/`: 43 rows, no `wall` row; (a) 2 gone (26.54 h); (a') 3 gone
  (26.54); (b) 4 gone (26.24); (b') 5 gone (26.24); (c) 2 serves; (d) 2 serves (26.53); the by-hand previews 19
  serves; production 4 serves; the rehearsal 2 gone (30.13). Unexpected 0, no reading 0. The control holds if no
  deploy or cleanup run starts between it and his switch-on time (the run list is checked afterwards).

##### Day 2 resumed (2026-11-24)
- **His message of 2026-11-24 19:07:46 UTC** (15af9685-0caf-45cf-a404-14f591e2e9af; byte for byte in
  `mustafa-2026-11-24-day2.md`, 927 bytes, sha256 f340440ac215...): he is here for the Day 2 sitting now; the token ends
  2026-11-25T22:59:59Z; read the card first; do the fresh readings my rules ask for before he changes anything in
  Cloudflare; the sitting's length and whether its second visit can be tonight first; then the list in one screen, then
  one step at a time; plain words.
- **Fresh readings, 2026-11-24 19:08-19:10 UTC (transcript times):** this branch equals its remote (f34fc45); this
  repository's `main` moved to 1ec6f6c (lane 1's #61 and #64 merged), to be taken in before the report's pull request.
  The test repository: `main` 347d5e0 (workflows unchanged); the same nine branches; open pull requests #5 (c), #9
  (cs/b9-edit), #10 (cs/b9-prod) only. Runs: 50 in all; since the control of 2026-11-19 (run 34 of them) only probe runs
  #10 to #25: the schedule fired 15 times (#10 to #24, 2026-11-20 03:30 to 2026-11-24 12:41 UTC, all success) and my
  #25. No deploy or cleanup run since the closes, so nothing changed the uploads.
- **The control with the wall off, before his switch-on**: probe run #25, id 45467161902, by hand, `purpose: deletion`,
  head 347d5e0, created 19:09:11 UTC; probe-results-45467161902 (artifact 14955801668, zip sha256 f2b1c7a3ac50...,
  equal to its digest), probed_at 2026-11-24T19:09:30.966Z, `.gate/preview-hosting-check/day2/probe25-control/`: 43
  rows, no `wall` row; (a) 2 gone (144.61 h); (a') 3 gone (144.61); (b) 4 gone (144.32); (b') 5 gone (144.31); (c) 2
  serves; (d) 2 serves (144.61); the by-hand previews 19 serves; production 4 serves; the rehearsal 2 gone (148.21).
  Unexpected 0, no reading 0. So after six days nothing came back and nothing the check needs went away.
- get_session (about 19:10 UTC): 394,099 of 1,000,000 tokens; the seven-day limit "allowed" (reset; next 2026-12-01).
- **The wall switched on (his step 1)**: his message of 2026-11-24 20:14:13 UTC (f39e46f9-f1ea-4ea2-9267-d9f6bb287aa4),
  his words: "Done, the button turned into a 'manage' button" / "When pressing on it the following page shows (see
  screenshot), I have censored the policy ID incase its something sensitive". The switch-on time itself was not given:
  it lies after my list (sent 19:12:54 UTC by the transcript) and before 20:14:13 UTC; the sweeps count from 20:14:13 UTC, so each is
  at least its stated time after the switch-on. No Zero Trust set-up, plan or payment screen is mentioned.
  - Screenshot 1, transcribed: "Preview access": "Preview deployments are restricted by a Cloudflare Access policy." /
    "Visitors must sign in and match the policy before viewing previews." / "This protects preview deployment URLs
    only. Production pages.dev and custom domains are managed separately in Zero Trust.", button "Manage".
  - Screenshot 2, transcribed (the address holds his account ID and the application's ID; not copied here): Zero
    Trust, Access controls, Applications, "shamble-preview-check - Cloudflare Pages", "Application details";
    Destinations, "Public hostnames": Subdomain (optional) [a field that appears to hold a wildcard], Domain
    "shamble-preview-check.pages.dev", Path empty; "Allow access through browser-based RDP, SSH, or VNC sessions" Off;
    "Access policies": "Allow Members - Cloudflare Pages" (marked "Legacy"), Action "Allow", Rules 1, Policy ID
    (blacked out by him); "Policy tester"; "Authentication" (Identity, MFA). Left menu: "Zero Trust" with Overview,
    Insights & Logs, Team & Resources, Access controls (Overview, Applications, Policies).
- **Outside look right after** (this container, curl with the probe's User-Agent, no cookies, 20:15:15 UTC): iso-off,
  (c) cs-pd-open, (d) cs-pd-off, the one-off 6e61a4c4 (iso-off) and production's one-off 0354374e: each HTTP 302 to
  https://shamble-preview-check-pages.cloudflareaccess.com/... (path not copied); production's own address
  https://shamble-preview-check.pages.dev/ 200. So the wall is on for previews, production's own address stays open,
  and a production upload's one-off link is behind the wall too (recorded; the mark judges production's own address
  only). Sweeps scheduled: trig_012uYr31Q5puh8mgoMhPHnUR at 20:25 UTC (at least 10 minutes) and
  trig_01PgaNh9YUgYbGi8tzK2My4X at 21:15 UTC (at least 1 hour, the reading that counts).
- **The hosting checklist, steps 5 to 8, with the wall on (Brave, his PC)**: his message of 2026-11-24 20:18:53 UTC
  (065b61dd-5ad4-4ad0-a82c-ebf9a2f3c948), five screenshots (the images stay out of the repository), his words:
  "ttps://iso-off.shamble-preview-check.pages.dev/ led to a login page, I logged in with my credentials and led to this
  page" / "Other screenshot shows the private page (ctrl shift N), also asking to log in" / "reloaded the
  https://iso-on.shamble-preview-check.pages.dev/ on the normal window and didn't ask me to relog, and shower the
  preview hosting check page" / "https://shamble-preview-check.pages.dev/ showed no login (4th screenshot)" /
  "https://iso-off.shamble-preview-check.pages.dev/ quit brave and started it again, used the first link".
  - Screenshot 1 (normal window, iso-off.shamble-preview-check.pages.dev, after his login): Build marker
    2162db55d93f59cf2ed7e371a3915409f0b5a587, Build mode preview, crossOriginIsolated (page) false, (worker) false,
    WebAssembly "ran (2 + 3 = 5)", Content file "content.ef463692.bin (65536 bytes)", Status done; the usual resources.
  - Screenshot 2 (private window): tab "Sign in · Cloudflare Access", address
    shamble-preview-check-pages.cloudflareaccess.com/cdn-cgi/access/login/iso-off.shamble-preview-check.pages.dev?kid=...
    ; page "Cloudflare Access" / "shamble-preview-check - Cloudflare Access" / "Log in to shamble-preview-check -
    Cloudflare Pages" / "Email" (box "example@email.com") / button "Send login code". So the login method shown is a code
    sent by e-mail (the setup page of 2026-10-12 said new organizations sign in with the Cloudflare account; recorded,
    not judged).
  - Screenshot 3 (normal window, iso-on.shamble-preview-check.pages.dev): the same marker, Build mode
    preview-isolated, crossOriginIsolated (page) true, (worker) true, "ran (2 + 3 = 5)", the same content file, done.
    Isolation still holds behind the wall.
  - Screenshot 4 (normal window, shamble-preview-check.pages.dev): the same marker, Build mode production, false and
    false, "ran (2 + 3 = 5)", done; no login (his words).
  - Screenshot 5 (brave://version): "Brave: 1.100.58 Chromium: 158.0.8127.42 (Official Build) (64-bit)" (unchanged
    since 2026-11-19); "OS: Windows 11 Version 25H2 (Build 26200.9974)"; JavaScript V8 15.8.62.31. (His profile path,
    which holds his Windows user name, is not copied here.)
  - Read against the checklist (ADR 0006 :1450-1457): step 5 (a Cloudflare login page; after one login the test page
    loads): as expected; the extra private-window open of iso-off: a login page, as expected (the stop rule not hit);
    step 6 (another branch, no second login): as expected; step 7: production with no login in the same session, as
    expected; **its second half (production again in a private window) not reported**; **step 8's result (does the
    login page come back after a restart) not stated**. Both asked of him; his verdict words ("achieved" or "not yet")
    asked with them.
- PM-0006-1, the machines: his PC runs Windows 11 Version 25H2 (Build 26200.9974) (brave://version, 2026-11-24).
- **The wall sweep at 10 minutes or more (PM-0006-8)**: probe run #26, id 45480782801, started by the session after
  the check-in trig_012uYr31Q5puh8mgoMhPHnUR fired (20:25:51 UTC), `purpose: wall`, head 347d5e0, created 20:26:12
  UTC; probe-results-45480782801 (artifact 14961513330, zip sha256 eb5cd9e2ad5e..., equal to its digest), probed_at
  2026-11-24T20:26:39.259Z, 12 min 26 s after his message of 20:14:13 UTC (so at least that long after the switch-on),
  `.gate/preview-hosting-check/day2/probe26-wall10/`. 43 rows: 42 answer HTTP 302 to
  shamble-preview-check-pages.cloudflareaccess.com ("wall"); production's own address https://shamble-preview-check.pages.dev/
  200 "serves" (expected "serves"). By case: (a) 2 wall; (a') 3 wall; (b) 4 wall; (b') 5 wall; (c) 2 wall; (d) 2 wall;
  the by-hand previews 19 wall; the rehearsal 2 wall; production's three one-off links (0354374e, e9a37459, de1c3e5e)
  wall (not judged: the mark names production's own address). Every judged row as expected ("no build" for all but
  production's own address); unexpected 0, no reading 0. The wall on the stable links of open branches (the script's
  rule, counted from the rows): 6, bite-coep, bite-nocache, cs/pd-open, iso-off, iso-on, uploads (the mark needs 2).
  PREV-DELETE's leftovers ((d)'s two links) give no build.
- **The wall sweep at 1 hour or more, the reading that counts (PM-0006-8)**: probe run #27, id 45489339865, started by
  the session after the check-in trig_01PgaNh9YUgYbGi8tzK2My4X fired (21:15:28 UTC), `purpose: wall`, head 347d5e0,
  created 21:16:02 UTC; probe-results-45489339865 (artifact 14966750236, zip sha256 a6e1c2c7d932..., equal to its
  digest), probed_at 2026-11-24T21:16:27.123Z, 1 h 2 min 14 s after his message of 20:14:13 UTC (so at least that long
  after the switch-on), `.gate/preview-hosting-check/day2/probe27-wall60/`. The same 43 rows as at 10 minutes: 42
  answer 302 to shamble-preview-check-pages.cloudflareaccess.com; production's own address 200 "serves" (expected
  "serves"); production's three one-off links walled (not judged); unexpected 0, no reading 0; the wall on the stable
  links of 6 open branches (bite-coep, bite-nocache, cs/pd-open, iso-off, iso-on, uploads). Runs between the control
  (#25, 19:09:30 UTC) and this sweep: only the probes #26 and #27 (52 runs in all): no deploy or cleanup in between.
- **PM-0006-8 read:** "with the wall switched on, a stranger gets no build from any link of at least 2 open preview
  branches, nor from PREV-DELETE's leftover links, while production still serves; read 10 minutes and 1 hour after you
  switch it on (the 1-hour reading counts); control: the same sweep with the wall off shows the builds": met. At 1 hour
  or more: no build from any link of 6 open preview branches (every one of their links walled) nor from the leftover
  links of (d), production's own address serving; at 10 minutes or more: the same; the control (#25, 19:09:30 UTC, the same 43
  links with the wall off, before the switch-on) showed every build serving. The
  switch-on time is not recorded to the minute (between 19:12:54 and 20:14:13 UTC), so "10 minutes" and "1 hour" are
  lower bounds; production's one-off links are behind the wall too (recorded for OQ-0006-4, not judged).
- **Visit 2, step 8**: pull request #5 (the control, cs/pd-open) closed by the session (update_pull_request; closed_at
  2026-11-24T21:17:35Z); its cleanup run `close` #6, id 45489619919, created 21:17:40 UTC, "waiting" on `preview` at 21:17:42
  (so the reviewer is still on, as Day 2 expects). His approval asked, with the run's link.
- **His answers** (his message of 2026-11-24 21:56:20 UTC, 8e7517c1-f682-489c-9907-d705409258e0, his words): step 7's second half: "Opened in private window:
  https://shamble-preview-check.pages.dev/" / "> opened without a login"; step 8 (after restarting Brave, the iso-off
  link): "ioebed straight away" (the test page opened with no login page; his pasted page text: Build marker
  2162db55d93f59cf2ed7e371a3915409f0b5a587, Build mode preview, crossOriginIsolated false and false, WebAssembly "ran (2 +
  3 = 5)", the content file, Status done, the usual resources); **"Step 5 to 8 achieved"**; the cleanup run: "done, and
  deployed and has gone green". So the hosting checklist's steps 5 to 8 are achieved in his words; step 8 records that
  the login lasted across a Brave restart (how long a login lasts is recorded, not judged).
- **(c)'s cleanup (step 8 of visit 2)**: run `close` #6, id 45489619919, job `cleanup` 138784117668, success: "Pull
  request closed at 2026-11-24T21:17:35Z (merged: false); this job started at 2026-11-24T21:56:05Z." / "Uploads of
  cs/pd-open found: 1" / "Delete 17d45ef9-1187-4897-9073-2b79a2ed8b64
  (https://17d45ef9.shamble-preview-check.pages.dev): HTTP 200 {"success":true,"errors":[]}" / "This job ended at
  2026-11-24T21:56:06Z." "Uploads found" equals the records (1 upload, 17d45ef9). His approval: the job started
  21:56:04 UTC. The token worked at 21:56:06 UTC: the "alive" control for the revoke.
- **Visit 2, step 9**: pull requests #9 (cs/b9-edit) and #10 (cs/b9-prod) closed by the session (update_pull_request;
  closed_at 21:57:22Z and 21:57:32Z); their cleanup runs: `close` #7, id 45496209201 (cs/b9-edit), created 21:57:24, "waiting"; `close` #8, id
  45496236161 (cs/b9-prod), created 21:57:35, "waiting". His rejections asked (neither branch has an upload: the token
  tests uploaded nothing).
- **Step 9 done**: his words "I have rejected both" (his message 22:05:29 UTC, fa560161-cbc2-4544-a295-05499bf0116a); `close` #7 (45496209201, cs/b9-edit) completed "failure" (updated
  22:05:09 UTC) and `close` #8 (45496236161, cs/b9-prod) completed "failure" (22:05:20 UTC): rejected, so nothing ran.
  Next: step 10, his delete of the token (the control's cleanup, which needed it, ended 21:56:06 UTC).
- **Step 10, the token deleted (PM-0006-32's revocation)**: his message of 2026-11-24 22:14:34 UTC
  (1bb52d80-37b6-4b51-8f78-52411327fda2), his words: "I have deleted the token". His screenshot, transcribed (the
  address holds his account ID; not copied here): "Account API tokens" / "Manage account API tokens. User API tokens are
  found in My Profile → API Tokens." / an empty list: "Create your first API token" / "API tokens authenticate access to
  your Cloudflare account, allowing secure integration with tools, scripts, and third-party services." / "Create a
  token". The delete time itself was not given: it lies after my step-10 message (22:05:59 UTC by the transcript) and
  before 22:14:34 UTC. Next: the `revoked` run (my start, his approval); it must fail at Cloudflare's sign-in.
- **Step 11, `revoked` started** by the session (run_workflow, deploy.yml, ref main, set `revoked`): deploy run #19, id
  45499119373, head 347d5e0, created 22:15:00 UTC; plan green (22:15:05-19), build "build (revoked, preview, revoked,
  preview, 1)" green (22:15:26-38); the run "waiting" on `preview` at 22:15:44 UTC. His approval asked.
- **Step 11, the proof that the token is dead (PM-0006-32's revocation)**: his message "done" (22:23:26 UTC,
  d03fa9c3-4c67-46dc-8a7e-0630d5285d60) with a screenshot of the run page (deploy #19: Status "Failure", "Deployment protection rules": "Mergincanli approved 1 minute ago",
  "preview"; Annotations "2 errors and 1 warning": deploy "Process completed with exit code 1.", verify "Process
  completed with exit code 3.", the warning "No files were found with the provided path: records/. No artifacts will
  be uploaded."; Artifacts: headers-45499119373, 311 Bytes). Read with the GitHub MCP tool: deploy run #19, id
  45499119373, conclusion "failure"; job "deploy (revoked, preview, revoked, preview, 1)" (138817751333) started
  22:22:00 UTC after his approval; the download, the server-code check and Wrangler's install green; "Upload to
  Cloudflare Pages" failed 22:22:21-23 UTC, word for word: "⛅️ wrangler 4.156.0" / "✘ [ERROR] A request to the
  Cloudflare API (/accounts/***/pages/projects/shamble-preview-check) failed." / "Authentication error [code: 10000]" /
  "📎 It looks like you are authenticating Wrangler via a custom API token set in an environment variable." / "Please
  ensure it has the correct permissions for this operation." / "Getting User settings..." / "✘ [ERROR] A request to the
  Cloudflare API (/accounts) failed." / "Invalid access token [code: 9109]" / "##[error]Process completed with exit code
  1."; then the records upload: "##[warning]No files were found with the provided path: records/. No artifacts will be
  uploaded."; verify (138821514796) exit 3 (no records). The run's artifacts: headers-45499119373 (311 bytes) and
  site-revoked (the build) only: **no deployment-record artifact**, so no upload was made. No account name or ID
  printed (the path masked as ***; the user lookup failed with 9109).
- **The revocation read:** the token worked at 21:56:06 UTC (the control's cleanup deleted 17d45ef9 with HTTP 200); he
  deleted it between 22:05:59 and 22:14:34 UTC (the empty "Account API tokens" list); at 22:22:21-23 UTC Cloudflare
  refused it at its first call (10000, then 9109), the same refusal as the rehearsal's attempt 1 with an unknown token,
  and nothing was uploaded. The run's red colour is not the proof; the refusal words and the missing record are.
  PM-0006-32's last clause ("Your revocation of the token at the end is recorded"): recorded.
- **Day 2's runs against its list** (56 runs in the repository at 22:23 UTC; today's 7 after the 50 counted at 19:08
  UTC, which included the control #25): probes #25 (control), #26 (10 minutes), #27 (1 hour); `close` #6 (the control,
  approved, success), #7 and #8 (the token tests, rejected, failure); deploy #19 (`revoked`, approved, failure as
  required). Nothing off the list; no scheduled probe ran during the sitting. His clicks on GitHub today: 2 approvals,
  2 rejections; in Cloudflare: "Restrict previews" and the token's delete. Red mails expected: three (the two
  rejections and `revoked`).
- **Billing, after the wall** (his message of 2026-11-24 22:47:31 UTC, f0185b17-b019-44b2-b25e-493c285d4b18), his words:
  "I just checked the cloudflare's billing page, and nothing paid was added when the wall went on". His word, not a
  screenshot: no paid plan or charge came with "Restrict previews" and the Zero Trust organization it made.

##### After the verifier (2026-11-24, before the report)
- **PM-0006-5's coverage gap closed from the run logs** (the verifier: no size line was recorded for sitting 2's ten
  builds, bite-production, restore-production, the two tombstones and `revoked`; check-sizes.mjs never fails on a
  flagged file, so a green build proves nothing). Each run's whole log downloaded (GitHub MCP tool, the logs' zip, files
  written 23:09:10-23:09:52 UTC) to `.gate/preview-hosting-check/sizes/<run id>/` and read with grep. Every build's line, word for
  word: "SIZES {"largest":{"path":"content.ef463692.bin","size":65536},"limit":26214400,"flagged":[],"count":9}" in
  deploy runs #7 44370016948, #8 44370024556, #9 44370034770, #10 44370043893, #11 44370054922, #12 44370062021, #13
  44370443574, #14 44370444666, #15 44370706631, #16 44370744826 (sitting 2's ten builds), #5 44366750977
  (bite-production), #6 44367096948 (restore-production) and #19 45499119373 (`revoked`). With round-a's, uploads' and
  bites' lines already recorded, every build of the check has its size line: the largest file 65,536 bytes, flagged
  none, except the planted 26 MiB file of `bite-oversize` (flagged and refused by Wrangler). The two tombstones (cleanup
  runs #3 44370914405 and #5 44374107847) upload one file each, `index.html`, written by close.yml's printf (its text at
  347d5e0: 96 bytes, counted with wc -c; "✨ Success! Uploaded 1 files" and "Uploaded 0 files (1 already uploaded)").
- **PM-0006-7, the question behind it** (a web-fetch agent, model opus, effort high, after 23:09 UTC): Cloudflare's
  docs do not say whether direct uploads count against the Free plan's 500 builds a month ("Each time you push new code
  to your Git repository, Pages will build and deploy your site."; the limits table "Builds per month | 500 | 5,000 |
  20,000"; limits page, dateModified 2026-11-19), and no page says where the dashboard shows a count. So the question
  stays unanswered by the dashboard and by the docs.
- **Corrections from the verifier** (for the report; the lines above stand as written at their dates): ADR 0006's line
  numbers in this log are three lines low since main's 3747bb3 ("When a mark fails" is now :1404-1418, the checklist's
  steps 1-3 :1447-1450 and steps 5-8 :1453-1460); from the closes to the 24-hour mark there were 24 schedule slots, not
  28 (3 scheduled runs in them); "the wall on the stable links of 6 open branches" counts 1 branch with an open pull
  request (cs/pd-open) and 5 Pages branches with none (iso-off, iso-on, uploads, bite-coep, bite-nocache); his Brave
  checks were 1 load per link (all as expected), not 3.

## Appendix B: the workflow texts as run
_Copied by script from the test repository's commits (`git show <commit>:<path>`). `main` 347d5e0 holds the four workflows that ran every run on `main` and every test pull request; the two token-test variants ran only on their own pull requests (#9, #10) in sitting 3; check-token.yml ran once (two attempts) on pull request #2._

### deploy.yml
At commit 347d5e0 of Mergincanli/shamble-preview-check; 7719 bytes, sha256 29593af18d34837b92011af3b2f78a52941b36cb3a25600af5e5919e5e25a06f.

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
          retention-days: 14
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

### close.yml
At commit 347d5e0 of Mergincanli/shamble-preview-check; 5343 bytes, sha256 3afdf389d636a0f0abb07689b5fbb17616591451d4b78ef2d309255154134def.

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
          retention-days: 14
```

### probe.yml
At commit 347d5e0 of Mergincanli/shamble-preview-check; 2541 bytes, sha256 f24ad6e81daff7b20798c57d7a6a7982c2a6a5bb0e0f89e51459a4d279a84154.

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

### browsers.yml
At commit 347d5e0 of Mergincanli/shamble-preview-check; 2049 bytes, sha256 6791076916ae67191e3a4c6e194cde48dc0db7bb1f95f2da21b7ddb814e773f5.

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

### deploy.yml on cs/b9-edit (the first token-test variant)
At commit 7f3a684 of Mergincanli/shamble-preview-check; 3610 bytes, sha256 fd3dc4c6029b2d6a0e0bb311bc97dafe11ab2966f61258668ef782d0786d357f.

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

### deploy.yml on cs/b9-prod (the second token-test variant)
At commit 9965630 of Mergincanli/shamble-preview-check; 3579 bytes, sha256 8b745631a278813d7d5dc80c87bbfcaedeec0651111f5a54784c05f04d2d964d.

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

### check-token.yml on check-token (the token check, one run)
At commit 863b639 of Mergincanli/shamble-preview-check; 4372 bytes, sha256 0ad66aa0f2b36f7469bc3bdd57a4a05b7945e0037f96b19059000e0ca9b5aa71.

```yaml
# THROWAWAY: the preview hosting check for SHAMBLE FRONTLINES (ADR 0006, spike 1). Plan and results live in
# Mergincanli/Shamble-Frontlines. This file is deleted with this test repository.
# A one-off check of the two values in `preview`, run on its own pull request (closed, never merged). It prints the
# values' shapes (never the values; of the account ID, which is not a password, its first and last 4 characters), then
# asks Cloudflare with them: A whether the token works for this account, B whether the token works at all, C for the
# very project the upload fails on; and D runs the pinned Wrangler with the same values. It prints only success flags,
# error codes and messages, the token's status and dates, and project names.
name: check-token

on:
  pull_request:
    types: [opened, reopened, synchronize]

permissions: {}

env:
  PROJECT: shamble-preview-check
  WRANGLER: wrangler@4.156.0

jobs:
  check:
    if: github.event.pull_request.head.repo.full_name == github.repository
    runs-on: ubuntu-24.04
    timeout-minutes: 10
    environment: preview
    steps:
      - name: Install Wrangler (this step holds no secret)
        run: npm install --prefix "$RUNNER_TEMP/wrangler" --no-audit --no-fund "$WRANGLER"
      - name: The values' shapes, then Cloudflare's answers A, B, C (no value is printed)
        env:
          CLOUDFLARE_API_TOKEN: ${{ secrets.CLOUDFLARE_API_TOKEN }}
          CLOUDFLARE_ACCOUNT_ID: ${{ secrets.CLOUDFLARE_ACCOUNT_ID }}
        run: |
          set -uo pipefail
          umask 077
          t="$CLOUDFLARE_API_TOKEN"; a="$CLOUDFLARE_ACCOUNT_ID"
          case "$t" in cfat_*) p=yes ;; *) p=no ;; esac
          if [[ $t =~ [[:space:]] ]]; then w=yes; else w=no; fi
          if [[ $a =~ ^[0-9a-f]{32}$ ]]; then h=yes; else h=no; fi
          if [[ $a =~ [[:space:]] ]]; then aw=yes; else aw=no; fi
          echo "token: length ${#t}; starts with cfat_: $p; whitespace in or around it: $w"
          if [ "$h" = yes ]; then e="${a:0:4}...${a: -4}"; else e="not shown (not an ID's shape)"; fi
          echo "account id: length ${#a}; exactly 32 hex digits: $h; whitespace: $aw; first and last 4: $e"
          printf 'Authorization: Bearer %s\n' "$t" > "$RUNNER_TEMP/auth"
          ask() {
            local code
            code=$(curl -sS --max-time 30 -o "$RUNNER_TEMP/answer.json" -w '%{http_code}' -H @"$RUNNER_TEMP/auth" \
              "https://api.cloudflare.com/client/v4$2") || code="no answer"
            echo "== $1: HTTP $code"
            if jq -e . "$RUNNER_TEMP/answer.json" > /dev/null 2>&1; then
              jq "{success, errors: [(.errors // [])[] | {code, message}], result: ((.result // null) | if . == null then null else ($3) end)}" "$RUNNER_TEMP/answer.json"
            else
              echo "(the answer is not JSON; it is not printed)"
            fi
            rm -f "$RUNNER_TEMP/answer.json"
          }
          if [ "$h" = yes ]; then
            ask "A. the token, for this account" "/accounts/$a/tokens/verify" '{status, expires_on, not_before}'
          else
            echo "== A. skipped: the account ID box does not hold an ID's shape"
          fi
          ask "B. the token, without an account" "/user/tokens/verify" '{status, expires_on, not_before}'
          if [ "$h" = yes ]; then
            ask "C. the project the upload asks for" "/accounts/$a/pages/projects/$PROJECT" '{name}'
          else
            echo "== C. skipped: the account ID box does not hold an ID's shape"
          fi
          rm -f "$RUNNER_TEMP/auth"
      - name: D. The pinned Wrangler with the same values (only its exit code and error codes are printed)
        env:
          CLOUDFLARE_API_TOKEN: ${{ secrets.CLOUDFLARE_API_TOKEN }}
          CLOUDFLARE_ACCOUNT_ID: ${{ secrets.CLOUDFLARE_ACCOUNT_ID }}
        run: |
          set -uo pipefail
          umask 077
          if "$RUNNER_TEMP/wrangler/node_modules/.bin/wrangler" pages project list > "$RUNNER_TEMP/wrangler.txt" 2>&1; then r=0; else r=$?; fi
          echo "== D: Wrangler exit $r"
          if grep -qF "$PROJECT" "$RUNNER_TEMP/wrangler.txt"; then echo "D: $PROJECT named in its output: yes"; else echo "D: $PROJECT named in its output: no"; fi
          grep -oE '\[code: [0-9]+\]' "$RUNNER_TEMP/wrangler.txt" | sort | uniq -c || echo "D: no error code in its output"
          rm -f "$RUNNER_TEMP/wrangler.txt"
```

## Appendix C: the advisor tool's consultations
_Copied by script from the checkpoint's section "Advisor tool consultations" (his words of 2026-11-11: when, the question, what changed; each called before the step it bore on was saved as done, except the slip of 2026-11-15 told under "The process")._

1. 2026-11-15 15:49:20 UTC (called; answer 15:51:36 UTC; transcript times), moment 1 · Q: is the approach sound (a
   separate test repository; one Pages Edit token in `preview` only, a reviewer with self-review off; probes and
   browser rows on GitHub runners; no custom domain)? · Changed: the test repository's place, the runner stand-ins and
   the one-token choice become decisions with both branches costed (the `spikes/` tension put to the advisor agent by
   name); a mark-by-mark table (as drafted / as planned / departure and who decides); every click read from a cited
   page first (else "if your screen differs, stop and tell me"); the reviewer gate's clicks and sittings counted and the
   close-time vs close-job time both recorded; the 24 h reading not on cron alone; asks listed (push to named branches
   there, his merges and branch deletion, the token straight from Cloudflare into GitHub's secret box); the saver
   before agents; no action toward the test repository this turn.
2. 2026-11-15 16:22:00 UTC (called; transcript time), moment 5 for plan revision 1 (late: see Open findings) · Q: is
   revision 1 ready for the advisor agent? · Changed: the `setup` branch is pushed without a pull request and opened
   only after "setup done" and his yes, so no workflow starts before `preview` is protected; the setup pull request's
   deploy and its merge's cleanup become two named runs (a pre-flight of the merged-close path), added to the count;
   Part 4's two over-claims hedged (reading a secret back; a custom domain on his zone) and the summary's "nothing
   outside Pages"; the check's own risk tier stated for the advisor agent to rule; the agent's brief gives questions,
   not conclusions; the saver checked first.
3. 2026-11-15 16:45:35 UTC (called; transcript time), moment 5 for revision 2, before it was saved as done · Q: may
   revision 2 be marked done? · Changed: the rehearsal (the setup pull request's two runs) declared outside the marks,
   with the results-log header written before it (Part 3 B, Table 1.1 PM-0006-1), as the session's reading for him to
   overrule; "For Mustafa" item 3 widened from pushes to every act in the test repository (opening and closing pull
   requests, starting and re-running runs); revision 1's leftovers grepped (none: `npx` appears only in Part 7.0's
   removed lines); the advisor-model line and the fullness figure kept for the closing message.

4. 2026-11-15 20:54:47 UTC (called; transcript time), moments 1 and 3, before the build · Q: is the build's approach
   sound (branch `setup`, U1-U3, local checks, push without a pull request, the review round of three)? · Changed: commit
   and push `setup` after each unit, recording each SHA (the saver does not cover that clone); git run from inside the
   clone (no -C or -c); the scripts' interfaces taken from the six approved texts first (records flat in verify, one
   folder deeper in probe; no records must not crash verify); every script keeps "no reading" apart from "measured and
   failed" (retries after a deploy, transport errors and 5xx are no reading, an empty marker never matches); each bite
   proven locally against a small `_headers` server and the page opened in the preinstalled Chromium (proves the
   scripts, not Cloudflare); WebdriverIO's options read from the pinned package itself; "setup done" confirmed by
   screenshots of both environments and the Actions page, plus the token's end date, before the setup pull request;
   every change to an approved text batched into one diff for one new yes; get_session read after `setup` is pushed.

5. 2026-11-16 08:47:00 UTC (called; transcript time), moment 3, before the approved CI change was written · Q: may the
   approved diff go in now, and with what? · Changed: the approved change as a commit of its own (two files, checked
   with `git show --stat`), hashes checked before and after; the round-2 Lows kept out (no unreviewed script change);
   the checkpoint updated before anything launched (Part 7.1 and 7.2 superseded; his step-2.6 item recorded for the
   report); the saver restarted before the advisor agent; the screenshots' checks listed before any list goes to him.

6. 2026-11-16 09:07:06 UTC (called; transcript time), moments 1 and 5, before the record repair and before steps 18
   and 19 were saved as done · Q: is the repair plan right? · Changed: a scan of all his messages for "repository
   ready" before any note, which then states only the records (the go, plan Part 3 A, his messages with times, the
   checkpoint's words of the time) and where the advisor agent's wording differs; a sweep of every commit of this file
   (it found step 11 too); the restore done by script and diffed against git (step 12's and step 14's bullets: diffed,
   identical); step 19 lists its three conditions as open; F-F a hard gate before the setup pull request; Key numbers
   with sources and times (the lockfile count corrected; the 72% labelled as before compaction); the message to him
   leads with the extra screenshot, its click path from GitHub's docs page.

7. 2026-11-16 12:37:51 UTC (called; transcript time), moment 4 (more than one defensible answer: edit the token or make
   a new one) · Q: which route for the token's end date, and what to send him? · Changed: edit first, framed as a look
   (his rule on the token: editing keeps the value where it is; a new token goes through his clipboard and leaves the
   old one alive with no documented delete clicks); where to look (Manage account, Account API tokens; else My Profile,
   API Tokens, and he says which list, since a user token departs from the plan); the docs' gap said plainly (editing
   confirmed, no clicks shown); one end date with its basis (2026-11-25: plan A5 step 5's ten days from 2026-11-15,
   dates at 00:00 UTC, so Day 2 by 2026-11-24); guards (only the expiration, no "Roll", a new value only into the
   preview secret); evidence, not a typed date (one screenshot of the permission and the end date, no token value);
   the fallback is his own setup step again (A5 with the date, A6 step 6, the old token removed), no decision entry,
   and no offer to run without an end date (his SA-R3 acceptance says ten days); no record that the token lacks an end
   date until he has looked; two hard gates in Next steps 1; both setup misses into For the report.

8. 2026-11-17 19:20:12 UTC (called; transcript time), moments 2, 3 and 4 (a third setup miss; the token's handling;
   what to ask about the unknown secret) · Q: what to send him on the token? · Changed: the record says no more than
   his words (no user-token look yet; the secret's value unknown; the save message of 7c8c648 corrected); one look
   first (My Profile, API Tokens, pressing nothing, no "View"), with both branches in words, so no "wait" stays open;
   the miss told by the records (the steps were in Part 3 of 20:11 UTC; the wait at 3a; no "go ahead"), not his
   "never received" taken as fact; both secrets' sources asked (source only); the end date as a rule plus today's
   value; gate (b)'s evidence without a token value (the summary page, or the form after "Back"; then `preview`'s
   secrets list with a fresh "Last updated"); no guess to him about the old value (a Global API Key answer would
   become a "Decisions needed" entry).

9. 2026-11-17 20:38:50 UTC (called; transcript time), moment 5 (before step 20 was saved as done) and moment 1 (before
   the setup pull request) · Q: are both gates met, and may the pull request open now? · Changed: both gates met on the
   recorded evidence; the remote `setup` checked first (1619164, 0 pull requests, 0 runs); the header's account-ID
   line marked as his words, not a screenshot; open first and send the list only once plan and build are green and
   deploy waits (a red run fixed before he sits); the list with the real names and links, the three acts in order, the
   approve clicks and the time, one line each for SA-R3, SA-R2, SA-R5, SA-R6, what starts without him (the probe's
   result checked first), and "a rehearsal: a red run is a finding, not a redo"; Sunday's token value not asked again
   (unanswered, in For the report); SA-R3's loop closed in one line; after the merge, the branches listed.

10. 2026-11-17 20:53:18 UTC (called; transcript time), moment 2 (a token problem a second time) · Q: the rehearsal's
   deploy failed at Cloudflare's authentication: what fix, and what to send him? · Changed: both values replaced, each
   from its own fresh copy (the log cannot tell a bad token from a bad account ID); first, the artifact name: verify
   uploads `headers-<run id>` with overwrite off, so attempt 1's artifact is deleted first (his bin icon; its content
   saved in the results log first), and only "Re-run failed jobs" (not "Re-run all jobs", which would upload `site-pr`
   again); a screenshot of each page as he arrives, before any change (the secrets list's times; the token list's
   "Last used", a hint only); the token by Roll if the row's menu offers it, else he stops and sends the menu; copy then
   paste at once, token first, then the account ID from Account Details; a self-check before saving (the token starts
   with "cfat", the account ID does not); one screenshot of the secrets list after; the end date surviving a roll
   recorded as an inference, not told as fact; the re-run is his click (not among my allowed acts), on a new list (his
   condition 3), said plainly as two clicks again; a stop rule: if it fails the same way again, no third paste, options
   instead (such as a user token, a plan departure).

11. 2026-11-17 21:15:02 UTC (called; transcript time), moments 2 and 4 (the same failure a second time; adding a
   permission departs from the plan's "nothing else") · Q: send the Pages Read change? · Changed: sent as a test and
   as his decision (a "Decisions needed" entry with my recommendation; he decides by doing it; his words kept for
   DECISIONS.md), not as the cause found: the evidence for (Pages Write's role line names no viewing; #16667: a valid
   token lacking a permission gets 10000) and against (no page says Write excludes Read; Cloudflare's CI guide grants
   Edit alone; #13590 reads 10000 as "not recognized at all") both told, the 9109 on /accounts left unexplained, and
   both outcomes told before he clicks (green: the permission was the cause; the same error: it is ruled out, leaving
   the account ID and the token itself); the strongest line: the approved workflows already read from Cloudflare (the
   deploy fetches the project, the cleanup lists uploads), and Read stays inside Pages on his one account; one trip,
   replacing the earlier "look" (screenshot of the Edit page first, tick Read, save, screenshot of the list; branches:
   no Edit or anything unexpected such as an IP rule or a start date, he stops; a new value shown on save goes into the
   token box only); then `headers-44171977539` deleted again, "Re-run failed jobs", approve, green, merge, approve
   `close`, all on a new list; meanwhile the next step prepared (a verify method for account tokens in Cloudflare's API
   reference, a by-hand check that needs his yes; or a user token, a plan departure); on green, step 2.6 notes that
   "Edit only" does not hold in the new form.

12. 2026-11-17 22:15:05 UTC (called; transcript time), moments 2 and 4 (the same failure after the Read test) · Q:
   the next step? · Changed: no more fixing by trial; the next step is the measurement (the drafted check), not a user
   token (it would explain nothing; why the account token fails is a finding for step 2.6); the record says "ruled out
   if Read was saved"; the check made decisive before he sees it (a call to the same project path Wrangler fails on,
   printing only success, errors and the project name; his yes/no on whether the Account ID he copies matches the
   dashboard's address bar, once a source shows the address bar holds it; outcome → next action for every result, no
   row ending in "look further"; its error paths tested locally: a null result, a non-JSON body, a trailing newline);
   where it runs: GitHub's docs on a by-hand start from a non-default branch first, else a pull_request trigger on its
   own branch with the fork guard, never on `setup`; the order: trigger, then the security-auditor's review, then his
   yes on the reviewed hash, covering the branch, the pull request, its one approval and its close; to him now: no more
   re-runs until the check, and an offer to stop tonight (nothing runs; the token holds until the 27th); not blocking:
   how he turned debug logging on (a repository secret or variable would break the empty-secrets condition); whether
   Read stays: decided after the cause is known.

13. 2026-11-17 22:29:55 UTC (called; transcript time), moments 3 and 5 (a CI text holding the token, before it is
   called ready) · Q: may the yes request go out? · Changed: not yet. Reviews on the final hash: the security-auditor's
   second round on what changed, and the advisor agent (tier 2: the text, the acts outside his list, the account-ID
   line's departure from the plan); A and C skipped when the ID box lacks an ID's shape (an error message may repeat
   the path); the row "A and B refused, shapes right" judged against Cloudflare's token-formats page, its next action
   the token's own page (status, the orange dot), not a third token; the deploy.yml follow-up told to him now and made
   a gate before any deploy re-run (and whether he can rename the account, which needs no workflow change); the yes
   request names the checkpoint commit and sha256 (checked with `git show`), in two entries (the text and its acts;
   the ID's first and last 4 with where he compares them); the run's output copied word for word into the results log
   before any deletion, and deletion his choice; after his yes: push, check the placed hash, check only this workflow
   started and waits, then a one-item list with the run link and head SHA.

14. 2026-11-18 13:52:32 UTC (called; transcript time), moments 4 and 5 (the cause found; the next step) · Q: how to
   fix the token's window, and what to send him? · Changed: he edits the existing token's expiration to the "7 days"
   preset, nothing else (Read stays: his decision; that Edit alone is enough stays unmeasured); a new value shown on
   save goes into the token box only; the preset moves his deadline, so one "Decisions needed" entry, Day 2's last day
   taken from the check's expires_on, not the preset's label; blocking before any deploy re-run: the measurement (the
   check re-run: not_before empty or past, C finds the project, D answers), the first record of the end date after
   saving; the second run on a new list, with his yes again for the reading, copying and log deletion; GitHub's re-run
   docs read first (fallback: reopening pull request #2); "Enable debug logging" unticked; a row added (the window still
   starts in the future → a new token, made fresh with the preset); the list in two parts with my "go" in words between
   them; the deploy run's artifacts read first (several `headers-…` side by side → no bin step); `site-pr` expires
   2026-11-24 20:42 UTC, after which "Re-run failed jobs" cannot work; to him: the cause in two lines, my miss owned
   (gate (b) called met on a screenshot of the form, consultation 9), his yes/no on [the account ID's first and last four characters]; the results-log header
   corrected; attempt 1 "consistent with"; for step 2.6: token setup ends with the verify answer, not a form screenshot.

15. 2026-11-18 14:32:34 UTC (called; transcript time), moments 1 and 5 (the second check read; before the setup
   sitting) · Q: may the go for Part 2 go out? · Changed: yes, on a measurement this time; the state re-read first
   (`setup` 1619164, pull request #1 open, mergeable_state "unstable" from the failed run, the deploy run at attempt 5);
   the results-log header given a dated line (Pages Read and Write, the measured window, the rename); outcomes for Part
   2 (deploy and verify green → merge; deploy green, verify red → he stops, I read headers.json and say whether to
   merge; deploy red → no merge and no re-run without a new list; no `close` run waiting after the merge → he stops and
   tells me; `close` red → a finding, the remaining uploads recorded); the full list again in the go (his condition 3),
   what stays unmeasured (the upload and the cleanup), the open findings before the merge (SA-R3 accepted; SA-R2, SA-R5,
   SA-R6 deferred; the upload tool's account printout: the rename answers the name, the ID equals the secret so GitHub
   masks it), the calendar from expires_on (Day 1 by 2026-11-24), the second run's logs deleted on the first run's
   basis; during the sitting: read each job as it finishes, list the branches after the merge, add `main`'s merge
   commit to the header, read the first probe run at :23; before sitting 1: the token wider than the plan (Read), Edit
   alone unmeasured, a choice for him with the rehearsal's result.

16. 2026-11-18 15:20:36 UTC (called; transcript time), moment 5 (before step 21 was saved as done) · Q: may the
   rehearsal step close? · Changed: yes, read from run data; first "Next steps" and "Key numbers" rewritten (stale since
   2026-11-16, the F-B fault again); `main`'s claim checked (2162db5's tree equals 1619164's; the four workflows
   re-hashed there); step 21's open items listed (the probe, the token's scope, the printout finding and its
   disposition); the probe read by a scheduled check-in with its outcomes written first, and he is told a red mail
   could come; before Day 1: a get_session reading and a compaction ask; the pull request page's environment link
   recorded as not seen, the first reading of sitting 1; the token-scope choice as one decision entry (removing Read
   needs a measured run on a new list first).

17. 2026-11-18 16:22:58 UTC (called; transcript time), moment 1 (before sitting 1, where the marks start) · Q: is
   sitting 1's approach sound (`round-a` started first, the list sent once it waits)? · Changed: sitting 1 a step of its
   own, step 22 left open; the gates only I hold, since starting is my act (step 23); each "approve now" names the run's
   number, link and legs (no `run-name`); the reds on purpose told before the failure mails, each red read for its
   reason (a green where a red is due is a missed bite); rows written before `round-a` (exit 3 is no reading; Risk 13
   "not met as written", the sitting continues; a re-run is his click, on a new list); "5 runs, 5 to 9 clicks", asked
   and cross-checked with the legs' start times; his active time apart from the browser run's wait (he may step away;
   4 and 5 back to back); the oversize leg's log read for the account printout; files read with jq from disk; the probe
   blocks sitting 2's closes; the list kept short.

18. 2026-11-18 17:52:27 UTC (called; transcript time), moment 4 (his question "can't we just automate it?" reopens
   REVIEWER) · Q: what to answer, and what to recommend? · Changed: a decision entry with both plan branches (a) and
   (b) (plan.md:1392-1400, :1240), nothing changed until he chooses; first read run #4's state (done: approved, red as
   planned) and whether the security notes or Part 4 lean on the reviewer (they do: Part 4's risk 1 and "a rewritten
   job gets nothing until you click"), named as (b)'s cost; the why in three lines (his (a); his only look before a
   run gets the token, the session cannot approve; sitting 3 tests the gate); both options as clicks from
   plan.md:572-583, the two merges his either way; (b) gives up his look at each run, mine included, and the clicked
   tally (the report: "measured on these runs, computed for the rest"); the token's reach as Part 4 has it; if (b): no
   setting change while a run waits (he approved #4 first; the browser run is the gap), a screenshot after the untick
   and my go in words, the list still before every sitting as "runs that will start", every run checked against the
   list after each sitting, a re-tick before sitting 3 with a screenshot (Mergincanli as reviewer, bypass unticked; it
   blocks sitting 3), Day 2 with the reviewer on (four clicks); his answer saved byte for byte for DECISIONS.md; his
   words on the approvals kept with times for OQ-0006-13; one line for a click-free preview design in step 2.6; the
   "Last update" fixed.

19. 2026-11-18 18:15:12 UTC (called; transcript time), moment 5 (before steps 22 and 23 were saved as done) and moment 1
   (before sitting 2 under REVIEWER (b)) · Q: may steps 22 and 23 close, and what does sitting 2 need now? · Changed:
   the summary's link count recounted from the headers files (15 preview links, 24 readings, not 12); every run of the
   repository tallied against sitting 1's list (10 runs; nothing off the list); step 22 closed on the by-hand run, the
   silent schedule kept open, the 18:23 slot to read; sitting 2's seven points written before the first push (step
   24): pacing per branch, the workflow hashes before each push and `main` after the merges, case (d)'s Risk 10 row,
   the by-hand probes standing in for a silent schedule, the PR-page link's reading, his list after asking tonight or
   tomorrow, polling by artifacts or check-ins; SAFARI-RUN after the closes (cause confirmed in the packages first,
   the smallest change, a forced local run in Chrome, a branch and a by-hand start, the first Windows set counts, one
   entry); to him now: short, the two open findings in a line each and the one question.

20. 2026-11-18 19:00:57 UTC (called; transcript time), moment 5 (before step 24 was saved as done) and
   moment 1 (sitting 3's approach) · Q: may step 24 close, and how does sitting 3 go with the reviewer off? · Changed:
   the results-log line on the ten runs reworded to what was read, the deleted ids matched to the probe's records with
   jq; Next steps and Key numbers rewritten (main 347d5e0, the Safari fix removed, the check-in ids); "For the report"
   gains the reviewer-off window, Safari dropped, the silent schedule, "no count shown", old production uploads
   serving; sitting 3 reordered: b9-edit pushed with the reviewer off first (the bite), then his one tick and screenshot,
   then the re-run that must wait and that he approves (the control; SA-R4 covered, the rejection drops), then b9-prod
   as planned with a second refusal after the restore; no new yes needed (no approved text changes, all acts in his
   go), one line on his list for "plan order"; outcome rows before the first push; the refusal's words from the public
   run page; rerun_workflow_run for b9-edit; production's restore shown with `main`; open windows written into Current
   step; get_session first; the red mails said.

21. 2026-11-19 00:23:51 UTC (called; transcript time), moment 5 (before step 25 was saved as done) · Q: may sitting
   3 close, and what goes into tonight's message? · Changed: PM-0006-32 matched clause by clause against the six
   readings (the revoke open for Day 2); a direct record that no one clicked on attempt 2 (the run pages: "approved",
   no event, "rejected"); the environments' deployment history from the pull request pages; the stale sections
   rewritten (Next steps, Key numbers, the schedule line); for the report: the early tick, the click tally, a note
   for step 2.6; tonight's message: no change in Cloudflare until the 24-hour reading is recorded (the wall reads as
   "gone"; no documented off switch; the delete waits for (c)'s cleanup); Day 2 prep: get_session and a compaction ask
   with a resume line, Day 2's list with a consultation first, sent by the 18:55 check-in; (c) serving proves the
   wall still off; what else needs the links without a login, written before the wall.

22. 2026-11-19 13:45:19 UTC (called; answer 13:47:48 UTC; transcript times), moment 1 (Day 2's approach, before the
   list goes to him) · Q: are Day 2's list and outcome rows right, and when does the list go? · Changed: the list goes
   now (he is here and plans his evening), with the go for the wall as a separate message in words after the reading
   and "nothing in Cloudflare until my go" at the top; the shape told (two short visits about an hour apart, three red
   mails, last day 2026-11-25); Brave's version asked now, so the header is complete before the reading; the plan's
   order kept ((c) open through the 1-hour sweep, so the control and the sweep read the same links) and no new
   decision on #9 and #10; (c) closed alone first, #9 and #10 only after its cleanup is read; `revoked` judged on three
   things, not its colour (the upload step's refusal word for word, no record artifact, (c)'s cleanup as the alive
   control), the "before Wrangler signs in" row checked against deploy.yml at 347d5e0; wall rows added (no build but
   not classed `wall`; a cancelled sweep; scheduled probes off the marks; his login failing; his evidence of the
   setting); the 24-hour reading asserted with jq, a fresh control probe if the go comes over an hour later, and only
   the exact pass row covered by this consultation; lane2-needs.md's change-set 10 line labelled as this session's
   reading; a get_session reading.

23. 2026-11-19 14:13:09 UTC (called; answer 14:14:36 UTC; transcript times), moment 4 (an unexpected event: his press at
   the access-policy step and his Brave steps 5-7 before the reading and my go) · Q: is my reading right (no wall, the
   reading intact, steps 5-7 void), and what goes to him and into tonight's go? · Changed: the reading confirmed; its
   miss named: consultation 22's advice to send the full list at once put runnable steps before him five hours early,
   so a step that must wait now travels only inside the go; the reply short and without blame, the result first; one
   rule: Cloudflare not opened at all until the go (no look-only screenshot now: the page is one click from the wall);
   only questions he can answer from memory (a login page at step 5, the "try for free" screen's words, a payment
   screen, Brave's version); the check-in's go rewritten to start look-only (the access-policy row, the Zero Trust page,
   billing or subscriptions) since "try for free" may be a trial and ACCESS-CARD (a) covers only the Free plan; no
   onboarding clicks written from memory or docs alone; a mid-afternoon outside look; the row "the wall seen before the
   reading → stop and consult" (late reading allowed; Zero Trust, Access, Applications the only named removal); for
   step 5, whether a signed-in Cloudflare session passes the wall unseen (if so, a private-window open of iso-off).

24. 2026-11-19 21:05:23 UTC (called; answer 21:07:00 UTC; transcript times), moment 1 (the wall's clicks, now from his
   screen) · Q: may the refreshed list go out, and in what shape? · Changed: the fresh control probe read first (done:
   all as expected); the token-delete clicks taken out of part D (the one later step he could run early, stranding
   the control's cleanup), D sent as one line with its steps at the time; the look-only Zero Trust and billing
   screenshots dropped (his screenshot settled "try for free"): he screenshots each Zero Trust screen as he goes and
   stops only at a price above $0 or "trial", with a plan or billing screenshot afterwards if no plan screen shows;
   the control's "about an hour" dropped as a gate (it holds if no deploy or cleanup ran between it and the
   switch-on; the run list checked afterwards); the private window as his stop rule, with production opened in the
   same private window; defaults only, nothing else touched in Zero Trust; his screenshot and time before Brave, then
   on without waiting; on that message an outside look and both sweeps scheduled with send_later (the first at once if
   his report is late, its real offset recorded); the wall and Brave offered tonight (about 20 minutes), D tonight or
   any evening up to the 25th; plain words, one line of why, the numbered list as "For Mustafa".

25. 2026-11-24 19:10:53 UTC (called; answer 19:12:02 UTC; transcript times), moment 1 (Day 2 resumed after five days,
   before the sitting) · Q: are the fresh readings enough, and how does the list go to him? · Changed: the readings
   confirmed as the right ones; the token-delete clicks given in the one-screen list after all (he asked for every
   click), with the gate inside that step's sentence (only after I write that pull request 5's cleanup has finished);
   plain words only (no bare case letters, numbers or workflow names); his item 3 first (visit 1 about 20 minutes now;
   visit 2 about 15 minutes, at the earliest an hour after the wall, tonight or tomorrow with a latest start leaving a
   margin before 23:59); three red GitHub mails certain, other mails "may"; rows for approvals possibly off, the token
   possibly dead (stop and consult), Brave possibly updated; short turns near the sweep marks; the unread scheduled
   probes and Day 1's evidence as non-blocking checks; the card's "Last update" line confirmed changed.

26. 2026-11-24 22:24:35 UTC (called; answer 22:26:47 UTC; transcript times), moment 5 (before step 26 was saved as
   done) · Q: may Day 2 close, and what goes to him tonight? · Changed: the readings and the revoke proof confirmed
   (the refusal words, no record artifact, the same token alive 26 minutes earlier); my claim to him that steps 5-8
   "complete the Brave checklist" corrected (step 4 has readings but no verdict word); his verdict lines, the "done"
   and the close times given their message ids and times by script; step 26 moved intact, Next steps and Key numbers
   rewritten, the billing screenshot carried by name; tonight's message says every Day 2 reading came out as expected
   and the report gives the verdict mark by mark (four marks not a plain pass: Risk 13, Safari, the hourly series,
   Brave's version late), one ask only (the billing screenshot), the acts he leaves alone until my go named with no
   clicks, the 25 November deadline gone; not blocking: Risk 13 re-read before "then step 2.6", the records' end on
   2 December and the probe's exit 3, the local evidence listed (present).
   2 December and the probe's exit 3, the local evidence listed (present).

27. 2026-11-24 22:50:59 UTC (called; answer 22:53:09 UTC; transcript times), moment 1 (the closing change-set's
   approach) · Q: start the report tonight, and in what order? · Changed: start now, each unit saved before the next,
   no compaction stop; plan Part 8's order (the saver, the verifier on the evidence, his words by script, the report
   with the log as its appendix, docs-keeper with lane 2's lines only, the draft pull request, the advisor agent's
   review, the fold, ready); marks judged against ADR 0006's own words, the open-branches question and "When a mark
   fails" first; citations re-checked after main's 108 files; low context (the log moved by script, the body from its
   summaries; lane 1's merged report as the shape); what the delete takes cited or folded; the last commit by hand; the
   privacy sweep, the checks on 78c2d9a, the advisor agent's file, the picture's route; his message's two decisions and
   the 2 December probe question.

28. 2026-11-24 23:10:54 UTC (called; answer 23:12:51 UTC; transcript times), moment 4 (the verifier's verdict changes
   the report's outcome; how to word the verdicts and the decisions) · Q: what goes into the verdict table and the
   decisions? · Changed: the draft's own words read by this session for the table; the file-size mark judged by the
   verifier from the new size lines (met with a recorded departure), not by this chat; two DECISIONS headers that stated
   the chat's reading as his decision rewritten by script; step 4 and the build count as one entry (no count source
   exists; open, or back to the draft with the header mark); the production one-off link's header back to the draft
   (no ADR edit in this pull request; docs-keeper told); whether step 2.6 waits put to him with both texts (the plan's
   Risk 13 sentence and his later order), recommended, not decided; one entry for the recorded departures (Brave late,
   the hourly series in 24 slots, 1 pull-request branch plus 5 without, Safari in his words); plain words; the outcome
   not "DONE"; the consultations line made true; both guard refusals; only the extra bite of ADR 0002's mark claimed;
   the picture departure; the probe switch-off as a dated go before 2 December; who opens the pull request checked; the
   gate run after npm ci; the assembler re-run after this record.
