# Questions for Mustafa, with pull request #67 (lane 2, Builder V2), 2026-11-25

Saved by his rule of 2026-11-25, 01:35 Amsterdam, item 6: "From now on, when you wait on me: save the full question as
a file in your notes, push it, and name the file and the commit in your message." The chat's message names this file
and its commit. The questions also stand in the report, `docs/reports/2026-11-25-adr0006-header-row.md`, under
"Decisions needed" and "For Mustafa".

## What waits on you
1. **Your go on pull request #67, for its final head.**
   - The chat gives you that head's full commit id once the advisor agent has reviewed it.
   - Your go is your go on the revised row. Your words: step 2.6's first edit "waits for my go on the revised row".
2. **PASS-WORDS: may the re-worded pass words stand?** (Decisions needed)
   - The question in plain words: your answer STEP4-COUNT (b) said the build-count question goes back to the draft
     and is not answered. The advisor agent read your (b) as also re-wording three places, so that they can be met by
     the readings already taken, while the question itself stays unanswered:
     - the build-count mark (PM-0006-7): "passes when both are recorded, each as the count the page shows or as "no
       count shown"";
     - checklist step 4: "Expected: two readings recorded, each a number or "no count shown"";
     - the checklist's opening line: "or the readings it asks for".
   - Each place carries the tag "the advisor agent's reading of Mustafa's STEP4-COUNT (b), never his decision".
   - Why it matters: with the old words ("two numbers recorded"), the build-count mark and step 4 can never be met,
     because no page shows a count. With the new words they can be met on the readings you already took.
   - Options:
     - (a) Your go on #67 covers all three.
     - (b) All three are removed by one push before the merge, and both stay open until someone shows a count.
   - Recommendation: (a). The question stays plainly unanswered in the draft either way, and step 2.6 works as if
     uploads count either way.
3. **Your word on checklist step 4, under its new words: "achieved" or "not yet".**
   - What you read on 18 November: no count of builds shown on the dashboard pages, before or after the ten uploads.
   - Nothing is written in your name until you say it.
   - Your verdict on the preview links in Brave (PM-0006-9) waits only on this word: steps 1 to 3 and 5 to 8 are
     "achieved" in your words.
4. **WORKERS-NEXT (optional; #67 does not need it).**
   - The question: does Workers static assets (Cloudflare's other product) stay only the draft's fallback, or is the
     preview hosting check run again on it now?
   - Why it matters: two things have changed since our research of 28 October.
     - Cloudflare's Pages page now says "Start new projects with Workers".
     - On Workers, a link for each pull request now comes from a feature called Previews.
   - Options:
     - (a) Keep Pages. At the draft's next revision, add Cloudflare's statement to its "Revisit when" line and refresh
       the Workers route.
     - (b) Run the check on Workers now: a new setup in your Cloudflare account, about two days.
   - Recommendation: (a). Nothing failed on Pages that Workers would fix, and the revised header row is met on Pages.
5. **The test repository's clean-up, at your own timing.** Your TR-END (a) said "I archive the repository, and delete
   the project after the report is merged", and #66 is merged.
   - The order matters, because an archived repository is read-only:
     - first, switch off the hourly checker, before 2 December (your words): Mergincanli/shamble-preview-check →
       Actions → "probe" → "..." → Disable workflow;
     - then archive the repository;
     - then delete the Cloudflare test project and its Access application (Zero Trust → Access → Applications).
6. **Only if you want step 2.6's plan written sooner:** name a second branch this chat may push. Otherwise step 2.6's
   plan starts after #67 merges (the advisor agent's order; it fits your "after it").

## What changed since the plan you know of
- The header mark is judged on production's own addresses. On production's one-off links the host's header is
  recorded, not judged, and the row says why. Every other header stays judged on both kinds of link.
- Part 1 weighs Workers static assets in one short paragraph (15 lines). Its sources are Cloudflare's docs, read
  2026-11-25. Nothing was measured on Workers, and Pages stays.
- Pages' limits page has changed. It no longer says previews are unlimited, and now reads "Preview deployments older
  than 30 days are deleted, unless they are one of the project's 100 most recent deployments". The draft's claim of
  "no cap on previews" is now dated, and follow-up ADR6-PAGES-RETENTION re-reads every place that relies on it.
