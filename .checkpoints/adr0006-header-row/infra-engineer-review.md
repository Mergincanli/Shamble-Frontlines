# infra-engineer: review round 1 of ADR 0006's small revision (09faefa..1c45ed7)

Reviewer: infra-engineer, the draft's author. 2026-11-25. Notes only; no commit, no edit of docs/.
Line numbers are at head 1c45ed7 unless marked "base" (09faefa). E7 sits at :155-166, E3 at :140-146.
Sources: my notes `infra-engineer-workers.md` (S1-S22, Q1-Q8, all read 2026-11-25), plus one re-read today (S23).

## Plan
1. Read the diff, plan rev 2, my notes. (done)
2. E7 clause by clause against S-blocks; paths; "Pages sets no cap"; 25 MiB; outcome sentence vs Appendix A. (done)
3. E3 vs Q8 and DECISIONS :193 (STEP4-COUNT (b)). (done)
4. Consistency: part 2, part 3, options row, trade-off, Revisit when, Rollback. (done)
5. Follow-up ids. (done)
6. Advisor before writing (done); verdict via SubagentHandback.

## New source read for this review
### S23. Pages: Limits, re-read (2026-11-25)
- URL: https://developers.cloudflare.com/pages/platform/limits/ (dateModified 2026-11-19). Read twice today: WebFetch,
  and the raw markdown export (`/pages/platform/limits/index.md`, 8,592 bytes, grep).
- Section "Preview deployments", line 93 of the .md, word for word: "Preview deployments count toward the deployments
  that Cloudflare keeps for each project. Preview deployments older than 30 days are deleted, unless they are one of
  the project's 100 most recent deployments."
- The sentence R6-HOST-F3 quoted on 2026-10-28, "You can have an unlimited number of preview deployments active on your
  project at a time.", is ABSENT (grep -i "unlimited" finds only "managed by an unlimited number of users").
- Still present: "Builds per month | 500 | 5,000 | 20,000"; "Each time you push new code to your Git repository, Pages
  will build and deploy your site."; no direct-upload word; "The maximum file size for a single Cloudflare Pages site
  asset is 25 MiB."
- Sure: high (raw page text).

## Findings (ranked)

### F1. High (trivial wording fix, fix now): E7 :160 "where Pages sets no cap" has no source today
- E7's header dates its readings 2026-11-25 (:156). This clause carries no cite, and today's Pages limits page (S23)
  no longer has the "unlimited number of preview deployments" sentence that R6-HOST-F3 (r6 :72-79, read 2026-10-28)
  quoted. The page now names no count and adds a retention rule. So the claim is unsourced, not shown false.
- The verifier would fail it on the same page. That is the scale's "evidence that does not hold".
- "Pages stays" does not rest on it: the outcome rests on Appendix A.
- Fix, smallest wording. Replace "where Pages sets no cap" with "where Pages' limits page names no count
  (developers.cloudflare.com/pages/platform/limits/)". Keep the retention rule out of E7: every added word would need a
  source, and it belongs to follow-up ADR6-PAGES-RETENTION.

### F2. Medium (follow-up ADR6-ROLLBACK): Rollback :1574-1576 now reads against E7
- Rollback says "if PREV-DELETE or a header mark fails on Pages, move to Workers static assets".
- E7 (:155) says the header mark failed, and Pages stays (:165).
- The mark-fail rule (:1428-1429) says "Workers static assets weighed again", not "move". E7 follows that rule.
- The plan's reason for leaving Rollback alone ("the revised mark is met, so its trigger is not reached", plan.md:152)
  holds only if "a header mark" means the mark as revised. A session reading Rollback alone would think a move is owed.
- Not a trivial fix (it changes Rollback's rule), so it is a follow-up. It joins my earlier point: "only ADR 0002's
  deploy target changes" understates a move. The deploy command changes (`wrangler preview`, Wrangler 4.135.0+, S2),
  the `_headers` host rules for `*.pages.dev` would not match Workers' hosts (S11), and E7's own words say a move
  means running the check again.

### F3. Low (trivial, recommended): E7 :161 lists the delete command as a gain the ADR does not rule out
- "and a command that deletes it" reads as a Workers gain against the gap the ADR still shows as open: part 3
  :196-198 ("stays a spike item"), the options row :624 ("PREV-DELETE unresolved"), the trade-off :743-746.
- The merged report gives PM-0006-6 as "Met with a departure" (report :35). My notes' draft carried that qualifier
  ("a delete PREV-DELETE no longer needs"); the rewrite dropped it.
- Fix: after "(previews; wrangler/commands/workers)" add "; PREV-DELETE was met on Pages without it (the check's
  report, PM-0006-6)". This gives the report's verdict for that one mark only, and never calls the check passed.

### F4. Low (trivial): E7 :158 "silent" should also name production's own address
- Q1: the Workers docs are silent on production's own `workers.dev` address and on its route or custom domain (S8,
  S11, S13). That address is exactly what the revised row judges ("production none on its own address").
- Fix: "the docs are silent on production's own address and its version links". "Silent" stays "silent".

### F5. Low (trivial): E7 :157 "a Preview's `workers.dev` link" goes past the source's term
- S5's words are "workers.dev Preview URLs". A Preview also has `workers.dev` deployment URLs, and on those the docs
  are silent (Q1).
- Fix: "The host adds `noindex` to `workers.dev` Preview URLs, "but not to custom domain Preview URLs"".

### F6. Low (record in the report): E7 is 12 lines against the plan's 8
- E7 runs 12 lines (:155-166); plan.md:126 says "in at most 8 lines".
- His "short paragraph" is still met.
- Fix: record the deviation in the change-set report (a plain-words rewrite). Do not trim.

## Checked and holding (the caller's questions)
- Every claim in E7 is in my notes, apart from F1's clause (no source today) and the over-reaches in F4 and F5.
- "Silent" stays "silent" (:158).
- Paths, all right:
  - previews/custom-domains (S5); previews/configuration (S7); previews (S2); wrangler/commands/workers (S22);
  - configuration/cloudflare-access (S9); static-assets/headers (S11); platform/limits (S13); platform/pricing (S14);
  - the Pages overview, marked as developers.cloudflare.com/pages/ (S16).
- "25 MiB a file, as on Pages": right. Workers: "Individual file size | 25 MiB | 25 MiB" (S13). Pages: "The maximum
  file size for a single Cloudflare Pages site asset is 25 MiB." (S23, today).
- The outcome sentence (:164-166) says no more than the readings, and speaks of the header row only.
  - Report :31: "every other header as expected, and both planted mistakes were caught".
  - Appendix A :663-672 (round-a): production's own address passes; the one-off fails only on the host's noindex.
  - :816-818 (restore-production): the same.
  - :805-811 (bite-production): fails on production's own address.
  - "After the verifier" (:1371-1395) corrects no PM-0006-2 reading.
- E3 (:141-146):
  - "Cloudflare's docs do not say (read 2026-11-24 and 2026-11-25 ...)" matches my Q8 (S17-S19; S21 medium) and
    report :1384-1388.
  - "Saw no build count ... before or after" holds: three places before, the Deployments page after (report :749-751).
  - The STEP4-COUNT (b) clause is true to DECISIONS :193.
  - Its date, 2026-11-25, is his own label, "25 November, 00:55 Amsterdam" (= 2026-11-24 23:53:15 UTC).
  - Step 2.6 is the previews (SPIKE_PLAN :114).
- Part 2: E7 changes nothing it says about the wall ("a wall can cover previews only" fits :177-179), so the plan's
  security-auditor condition (plan.md:138) is not triggered.
- Part 3, Revisit when (:811), the trade-off (:740-746) and the options row (:624): none is now wrong because of this
  diff. Their PREV-DELETE wording predates the check's verdict. F3's fix keeps E7 from leaning on it. The "unlimited"
  wording goes to ADR6-PAGES-RETENTION.

## Follow-ups (for STATUS)
- **ADR6-WK-ROUTE.** r5 :851-852, :1125 and :1941-1942, and r6 :972-973, name `versions upload --preview-alias` and
  "1000 aliases kept, no deletion read". Today Cloudflare says "Do not use Version URLs for branch or pull request
  testing. Use Previews instead." (/workers/previews/compare-workflows/), and `wrangler preview delete` deletes "a
  Preview and all its deployments" (/workers/wrangler/commands/workers/), both read 2026-11-25. Give each a dated
  pointer at R5/R6's next revision.
- **ADR6-ROLLBACK.** F2: Rollback :1574-1576. Make its trigger match the mark-fail rule ("weighed again"), and replace
  "only the deploy target changes" with the real list: `wrangler preview` (Wrangler 4.135.0+), `_headers` host rules
  rewritten for Workers' hosts, and the check run again.
- **ADR6-WK-BIC.** Any Workers re-run's header and probe scripts must send a descriptive User-Agent. Browser Integrity
  Check covers every `workers.dev` hostname, cannot be turned off there, and answers default library agents with error
  1010 (/workers/configuration/routing/workers-dev/, read 2026-11-25). The Pages check's probe already sent one (report
  :1699).
- **ADR6-PAGES-RETENTION.** Pages' limits page (dateModified 2026-11-19, read 2026-11-25) says:
  - deployments are kept 30 days, plus the 100 newest and current production;
  - preview deployments count toward that;
  - a deleted deployment's hash URL "stops working";
  - the "unlimited" sentence is gone.
  Update R6-HOST-F3 (a dated pointer), part 1's *Why:* "no cap on previews" (:147-148) and the options row's
  "Unlimited previews" (:624), and check R5-PREV-F13 against it.
