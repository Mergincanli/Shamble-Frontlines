# Fills the report body's placeholders with the texts below (written by hand in this chat; run once).
p = '.checkpoints/preview-hosting-check/report-body.md'
s = open(p, encoding='utf-8').read()
T = {}
T['VERDICTTABLE'] = '''| What it checks (the mark) | The draft's own words, shortened (ADR 0006) | Verdict |
|---|---|---|
| The setup is written down first (PM-0006-1) | "recorded: ... each browser's and WebdriverIO's versions"; "checked again before PM-0006-6's 24 h reading" (:1369) | Met with departures: GitHub's machines stand in for the laptop and the Mac (his MACHINES (a)); his Brave version reached the record about 2 hours after the 24-hour reading |
| The headers as built, on both kinds of link (PM-0006-2) | "100 % of responses carry their expected set: ... previews `X-Robots-Tag: noindex`, production none" (:1370) | **Not met as written:** production's one-off links (0354374e and de1c3e5e) carry the "keep out of search engines" header on all 7 paths, added by the host; every other header as expected, and both planted mistakes were caught |
| Isolation follows the flag in every browser (PM-0006-3) | "3 of 3 loads per browser and deployment", in Chrome, Edge, Firefox, Safari and Brave (:1371) | Met with a departure: Safari not run (his words: "focus on brave, chrome, firefox and maybe edge (most used browsers)"); Chrome, Edge and Firefox 3 of 3 on all five links, the planted mistake caught; Brave 1 load per link, each as expected |
| The content file is cached for good (PM-0006-4) | "immutable"; on a second visit "it transfers 0 bytes", "3 repeats" (:1372) | Met with a departure (a GitHub machine for the laptop): 0 bytes in 3 of 3; the planted `no-cache` caught |
| Every file fits the host (PM-0006-5) | "the largest file in each deployed build under 25 MiB ... on every deployment of the check" (:1373) | Met with a departure (the same): every build's size line read, the largest file 65,536 bytes; the planted 26 MiB file flagged and refused by the upload tool |
| A closed pull request's build is gone in 24 hours (PM-0006-6) | "24 h after each test pull request closes, none of its URLs ... answers HTTP 200 with that build's marker"; run "with the access policy off" (:1374) | Met with a departure: at 24.1 to 24.4 hours every link of the four closed cases gone, the open control and the bite still serving, no wall; the hourly probes fell short (3 scheduled runs in the 24 slots; by-hand probes stood in) |
| Do uploads count against the 500 builds a month (PM-0006-7) | "reported, no pass number: the project's monthly build count before and after 10 direct uploads ... passes when both are recorded" (:1375) | **Open:** no count is shown on any page read, before or after; Cloudflare's docs do not say |
| The wall covers the previews and not production (PM-0006-8) | "on every preview URL of at least 2 open branches ... and on PM-0006-6's leftover URLs, a cookie-less client gets no build"; "on the production URL ... it gets the build"; "the mark reads the 1-hour sweep" (:1376) | Met, on a reading of "open": at 1 hour or more no build from any link of six Pages branches (one with an open pull request, five without) or of the leftover links; production's own address served; with the wall off, every build served |
| His verdict in Brave (PM-0006-9) | "every step of the hosting checklist below \\"achieved\\", 0 \\"not yet\\"" (:1377) | **Open:** steps 1 to 3 and 5 to 8 "achieved" in his words; step 4 (the build count) has no verdict |
| GitHub shows the preview link on the pull request (PM-0006-10) | "recorded: yes or no, with a screenshot of the page" (:1378) | Met: yes, logged out (this chat's picture) and logged in (his words) |
| A pull request cannot reach the token or production (PM-0006-32) | "2 of 2: ... gets no secret until the `preview` environment's required reviewer approves it ...; a job in a pull request that names the `production` environment is refused" (:1379) | Met: both cases, the control and the bite; reported: a production deploy with the preview token is not refused; recorded: the merge ref is judged and refused, and his revocation is proven by the refused upload |

By the draft's rule the spike has not passed: one mark is not met as written (the headers) and two are open (the build
count, and his verdict on step 4, which asks for that count). The departures change no reading.'''.replace('\\\\"', '"')
T['EVIDENCE'] = '''Every reading stands word for word in Appendix A, by sitting and in time order; the verifier spot-checked the key ones
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
- The token: "Day 1, sitting 3", "PM-0006-32 clause by clause", the revoke and "The revocation read".'''
T['VERIFIERLINE'] = '''PASS WITH NOTES on the evidence: five spot-checks against the downloaded data (the 24-hour
  reading, both wall sweeps, the control, the header summaries, the browser rows), all PASS; its verdicts per mark are
  the table above. It first called the file-size mark open (no size line recorded for 13 builds); this chat then read
  every build's size line from the run logs, and the verifier, given them, judged it met with a departure. Its
  corrections are in Appendix A, "After the verifier". Not verified by it: the "secret: present" lines, his screenshots
  and words, sitting 2's header files. The gate did not run for it (the toolkit was not installed); this chat ran it
  afterwards: PASS, the smoke test skipped (the branch changes only docs and notes).'''
T['RISK13ROUTE'] = '''"A failed mark goes back to the draft: infra-engineer revises the decision
  part the row names by the draft's own Rollback and "Revisit when" routes (a header or PREV-DELETE fail on Pages:
  Workers static assets weighed again ...), and the revised part returns to the advisor for review and to Mustafa and
  the Advisor with the measured numbers" (ADR 0006 :1411-1416). Nothing in ADR 0006 changes in this pull request. That
  paragraph does not name step 2.6; the plan he approved does ("the check's end and step 2.6 wait for the revised
  draft", Risk 13), while his order of 2026-11-18 puts step 2.6's plan right after this report. Whether step 2.6 waits
  is his decision (below).'''
T['PROBEAFTER'] = '''The upload records are kept 14 days; the last of them expires on
  2026-12-02 (about 18:35 UTC). After that the probe finds nothing to probe and ends every run red (it exits 3 when it
  has no rows), and GitHub mails the failure of a scheduled workflow to the person who made it: Mustafa's account.
  GitHub's docs do not say whether archiving a repository stops its scheduled runs; switching the workflow off does. So
  the probe is switched off before 2 December (For Mustafa).'''
T['STEPNEXT'] = '''His three decisions below. Then, unless he decides that step 2.6 waits for the draft's revision: step 2.6's plan (the
previews without his click and what keeps the token safe then, the locked install), brought to him for his go; the
revision of ADR 0006's header row is infra-engineer's work by the draft's route, after his word. The test repository's
probe is switched off before 2 December, and the repository archived and the test project deleted after this report is
merged (his TR-END (a)).'''
T['DECISIONSNEEDED'] = '''1. **ONE-OFF-HEADER: what happens to the header mark that is not met.**
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
   - My recommendation: (a).'''
T['FORMUSTAFA'] = '''1. The three decisions above, one line each (for example "ONE-OFF-HEADER (a), STEP4-COUNT (b), DEPARTURES (a)").
2. Before 2 December: switch off the test repository's hourly checker, so it cannot mail failures once its records
   expire. Go: any time now (no reading needs it any more). Clicks (GitHub's docs): Mergincanli/shamble-preview-check →
   Actions → in the left list, "probe" → the "..." menu at the top right → Disable workflow.
3. Your go on this pull request, once the advisor agent has reviewed its head (it is a draft until then).
4. After it is merged, on my go: archive the test repository (Settings, General, "Archive this repository"), then delete
   the test project in Cloudflare and its Access application (Zero Trust, Access, Applications).'''
for k, v in T.items():
    assert s.count(k) == 1, (k, s.count(k))
    s = s.replace(k, v)
open(p, 'w', encoding='utf-8').write(s)
print('filled:', ', '.join(T))
