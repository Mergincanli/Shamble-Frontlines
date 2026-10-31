# Checkpoint: v2-research
- **Goal:** R1–R6 research files (`docs/research/r1-…` to `r6-…`), each in the README's 8-part shape, verified as
  the plan says, plus the README status column and the report · plan: `.checkpoints/v2-research/plan.md` (advisor
  APPROVE with 14 conditions, 2026-10-27, `advisor-verdict.md`) and `plan-amendments.md` (the Advisor's 2026-10-27
  instructions in this session), approved by the advisor with 8 conditions (`advisor-amendments-verdict.md`, part A)
- **Branch:** cs/v2-research (session_0173vVo5q59ameR6cnN9BENg; the main checkout, never switched while research runs)
  · **Base:** origin/main 07ab240 (R7 merged; `docs/LEGACY_INVENTORY.md` checked present)
- **Last update:** 2026-10-31 11:13 Amsterdam (10:13 UTC)

## Steps done
1. Plan and verdict recovered verbatim from the R7 session's transcript (`provenance.md`): one review round, APPROVE
   with 14 conditions at 11:36 UTC; final plan written at 11:41 UTC with the conditions applied.
2. Branch created from origin/main 07ab240 after PR #27 merged (17:36 UTC); R7's three files present · 8ff1c74
   (first push of cs/v2-research: the proxy accepts cs/ pushes).
3. Advisor check of the amendments: APPROVE with 8 conditions (part A of `advisor-amendments-verdict.md`), 18:22 UTC.
4. Wave 1 units launched 19:18 Amsterdam; all 11 cut off by the org spend limit at about 19:25 (HTTP 429, "resets
   9:20pm UTC"), each with its notes saved (19–93 lines). Resumed 22:24 Amsterdam by message, each from its own
   transcript and notes, with one added instruction: exact quoted source text for every finding a recommendation
   rests on; a claim seen only in WebFetch's summary is "inferred".
5. Gate on this branch: PASS, all 8 checks (`tools/node_modules` installed with `npm ci --omit=optional`, so the
   smoke test uses the pre-installed Playwright 1.63 and its Chromium; toolkit pins 1.56.1).

## Current step
6. Wave 1 step 4 (second verifier pass, running) and wave 2 (R3, R4, R6 owners running; R5 owner and its CI helper
   start when two slots free up: at most 5 agents at once). Wave-1 tally written before wave 2 launched
   (`wave1-tally.md`; the second pass's counts are added when it returns). Units, drafts and notes as before; wave-2
   prompts `prompts/<agent>-r<n>.txt` and `prompts/infra-engineer-r5-ci.txt`, each launched with a one-line message
   that points the agent at its saved prompt file (the message is in each file's header).

## The Advisor's decisions, 2026-10-30 (relayed by Mustafa at about 17:29 Amsterdam, verbatim)
> - VERIFY-FETCH: (a). Daytime only, read-only fetches from allow-listed hosts, with the host
>   audit in each report. The night shift stays on WebFetch only until kit-v3-1 is merged.
> - Unread sources: commit the full list to the research branch as
>   docs/research/UNREAD_SOURCES.md, with what each source was needed for. The Advisor will look
>   for other copies from there.
> - The critic's 41 items: resolve the 12 contradictions before the decision drafts. Add a shared
>   glossary (docs/research/GLOSSARY.md) with one definition per term, starting with the tick
>   budgets, and make every topic use it.
> - Schedule: back to normal from tonight (the window ends at 07:30, as decided under
>   WINDOW-0730).
> - NS-2 (from the night shift): (c). When you restore the night schedule, make the "Night
>   shift" routine start a fresh session on every firing, keeping the same one-line prompt, the
>   repository, Opus 5.5 and the hourly 00:10–07:10 schedule. If the routine can't be switched,
>   create a new one with those settings and remove the old one, or list it for Mustafa to delete.
(NS-2 relayed at about 17:40.) Checked read-only: update_trigger cannot switch the persistent-session routine
trig_019LtRqmAdzddyk6K3e3kDJz; create_trigger can make a fresh-session one but has no repository field, so the new
routine's stored sources must be checked after creation (the night shift reads NIGHT_SHIFT.md from the repo).
These change the approved plan (the critic "resolves none"): from 18:00, write `plan-amendments-2.md` (how the 12
contradictions get resolved, who writes the glossary and applies it to each file, UNREAD_SOURCES.md, curl by day) and
have the advisor agent review it before any edit (CLAUDE.md protocol step 2).

## Running work (2026-10-30 evening)
- 18:06: night schedule restored (cron 0-7), issue #16's "Tonight only" line removed; NS-2 attempted: a fresh-session
  routine created from this session had no repository and no connectors (`sources: []`), deleted at once; the old
  routine kept. NS-2 (2026-10-30, a fresh session per run) goes to Mustafa (Routines page).
- `plan-amendments-2.md`: advisor APPROVE with 13 conditions (`advisor-amendments-2-verdict.md`), conditions applied in
  the amendment (3d1b733).
- Step 3a: engine-architect (xhigh), workflow run wf_02517947-3e7 (task w4wxzze92), launched about 17:20 UTC; writes
  docs/research/GLOSSARY.md, `resolutions.md`, `cross-topic-oqs.md`; notes `engine-architect-resolve.md`.
- UNREAD_SOURCES.md committed 82298ab (script `scripts/build_unread.py`, check `scripts/unread-check.json`: 8 tables,
  187 rows, 180 sources, 148 addresses each once, 67 hinge rows; needed-for: 44 hinge, 4 named, 24 note, 108 no tie).
  Re-run after step 3c; step 3d samples 10 rows.
- Asked now (advisor condition 5): MODES-3 and PREVIEW-WALL (Mustafa), BASELINE-WHEN (the Advisor).
- Step 3a DONE (run wf_02517947-3e7, about 40 min): 7 resolved, 1 aligned (ABL-PREDICT), 4 escalated (MODES-3,
  PREVIEW-WALL, BASELINE-WHEN for #17 and #23); GLOSSARY.md 34 terms (committed as a draft); 28 shared cross-topic open
  questions (72 insertions) + rank 41's nit in C22.1; report `engine-architect-resolve-result.md`.
- Step 3b: advisor (xhigh) on the resolutions, run wf_d38cd72c-feb (task wd5bl1f22), launched about 18:00 UTC.
- Session finding for the condition-9 check (`scripts/glossary_check.py`, baseline `scripts/glossary-baseline.json`):
  the glossary's "Retire" lists hold common words with conditions ("48", "players", "always", "Core", "vehicles",
  "effects": 1,335 hits today outside quotes and dated records), so "zero retired synonyms" cannot be checked as
  written. Needed before step 3c: per term, an unconditional list of exact phrases (must reach zero) and a conditional
  list (listed for the step-3d verifier to judge in context). Take it up with the advisor's verdict.
- Step 3c prompts drafted (generator `scp sp3/mk3c.py`, not yet run): five owners, background agents, 4 at once.
- Step 3b DONE (run wf_d38cd72c-feb, about 18:14 UTC): 4 APPROVE, 4 REVISE, 4 ESCALATE; verdict verbatim in
  `advisor-resolutions-verdict.md`; its changes folded into `resolutions.md`, GLOSSARY.md and `cross-topic-oqs.md`
  by the session (99c280d; GLOSSARY "terms approved" 8909523). **SHA before step 3c: 99c280d** (topic files unchanged
  since 8dee746). Pre-fold copy of resolutions: `scratchpad/sp3/resolutions.before-fold.md`.
- The Retire split (condition 9 made checkable): advisor APPROVE with conditions (run wf_06dbf34d-b30, about 18:21
  UTC; verbatim `advisor-retire-split-verdict.md`); check script fixed per its item 2 (whole words, each place once,
  hits inside term names and "Marked by" phrases skipped): baseline before the split 0 exact, 817 in context
  (`scripts/glossary-baseline2.json`, hits `scripts/glossary-hits-before-split.tsv`). The step-3c prompt generator
  (`scripts/mk3c.py`) carries the renaming rule; `scripts/retire_split_compare.py` checks that the quoted phrases match
  before and after (item 4).
- The split edit: engine-architect (format only), workflow run wf_1eb87409-4bd (task wlqyojhoj), launched 18:21 UTC;
  prompt `prompts/engine-architect-retire-split.txt`, notes `engine-architect-retire-split.md`; the pre-split glossary
  is in `scratchpad/sp3/GLOSSARY.before-split.md` (from HEAD).
- The split DONE (about 18:34 UTC; report verbatim `engine-architect-retire-split-result.md`): 30 phrases exact, 73 in
  context, 8 keeps (111 quoted phrases before); 8 glossary phrases rejoined across line breaks. Compare: 34 entries, 0
  problems (`scripts/retire-split-compare.txt`). Committed **7c6e0d7** (the SHA before step 3c; the topic files are
  unchanged since 8dee746).
- Check script fixed again (the agent's gap and the session's own): phrases match across line breaks; straight quotes
  paired inside each block (a quote may span lines); hits skipped as quotes or dated records are listed (`--skipped`,
  as the advisor's step-3b answer asks); term uses and first uses listed (`--terms`, `--first-uses`). **Both counts
  (advisor ruling item 4), final script, quotes and dated records skipped:** before the split 0 exact, 737 in context
  (`scripts/glossary-baseline-before-split.json`); after it 42 exact, 640 in context (`scripts/glossary-baseline-split.json`,
  hits `scripts/glossary-hits-split.tsv`); 737 − 682 = 55, exactly the hits of the phrases moved to Keeps (Core 30, big
  19, Whole ticks 5, at least 4 cores and 8 threads 1). Earlier scripts gave 1,335 and 817 (history). Skipped after
  the split: 104 in quoted text, 64 in dated records (`scripts/glossary-skipped-split.tsv`). Note for the report: the
  script also skips "[Second fix pass" (2) and "[Second verifier pass" (4), the same records as "[Fix pass" and
  "[Verifier"; their hits are in the skipped list. Two retired phrases are word patterns with "…", not searchable: the
  step-3d verifiers read for them.
- **Step 3c launched 18:39 UTC** (19:39 Amsterdam), background agents with one-line pointer messages (each prompt's
  header holds its launch record): engine-architect R1+R4 (xhigh), netcode-architect R2 (xhigh), test-engineer R5
  (high), infra-engineer R6 (high); prompts `prompts/<agent>-<k>-consistency.txt`, notes `<agent>-<k>-consistency.md`.
  graphics-engineer R3 (high) starts when a slot frees. If cut off: resume each by message from its notes.
- 18:44 UTC: correction sent to the four running owners (verbatim `prompts/step3c-correction-1.txt`): the prompt's
  step 3 named the new subsection "Shared open questions (consistency critic, 2026-10-30)"; the approved files name
  two, "### Shared open questions (the same words in every topic)" and "### Cross-topic open questions (consistency
  critic, 2026-10-29)". `mk3c.py` fixed and now writes only the keys asked for (R3's prompt regenerated; launched
  prompts unchanged, md5 checked). Night-stop reminder set for 22:50 UTC (23:50 Amsterdam). Saver restarted 18:41 UTC
  (task bd7o9zhjo, 110 minutes). Fetch audit script `scripts/fetch_audit.py` (all transcripts): every host requested
  is on the allow list except api.github.com (the advisor's #29 review: 3 Bash calls, 10 curl requests via `$A`);
  off-list hosts appear only as WebFetch redirect targets, not fetched (gdconf.com, bevy.org, clippy.rust-lang.org,
  docs.fly.io).
- R5 owner DONE about 18:48 UTC (report verbatim `test-engineer-r5-consistency-result.md`): 4 recommendation changes
  (C19.1, C21.1, C22.1, C22.3), 64 nits, no OBJECT; C17.b3 waits; 0 exact hits; committed **1da4e5f**. R6 owner DONE
  about 18:49 UTC (`infra-engineer-r6-consistency-result.md`): 6 recommendation changes (C1.21, C19.5, C19.6, C21.4,
  C21.8, C22.5), 51 nits, no OBJECT; C23.b1–b3 wait; 0 exact hits; committed **de0a003**. Its two notes, for step 3d
  and later: R6-HOST-REC1's "Rests on" does not list the R5 findings C19.6 now cites (the verifier checks it); if
  BASELINE-WHEN picks (b), C23.b2's quote "…rejects a shared key." needs adjusting, since C23.3 replaced its final
  period. Check script: the header-link test now reads R6's "## 1. Header" section (R6 was a false "no").
- R3 owner launched 18:49 UTC (background, pointer message; header stamped). Running: R1+R4, R2, R3.
- **Fourth spend-limit cut**, about 18:52 UTC (HTTP 429, "your 5-hour limit resets 8:10pm (UTC)"): the R1+R4, R2 and R3
  owners stopped part way, notes saved (R1+R4 63 lines, R2 38, R3 1). Resumed by message at 20:11 UTC on Mustafa's
  "Try again", each from its notes and the file's current state (restart rule).
- Session checks on R5 and R6 (scripts, for step 3d and the report): `scripts/quotes_check.py 7c6e0d7 de0a003`: no quoted
  text left either file (R5 572 → 589 quotes, R6 254 → 265; the new ones come from the approved texts, the PREV-DELETE
  question's source quotes included, or are the step's own labels); `scripts/apply_check.py de0a003 --topics R5,R6`:
  45 change items, every after text or insert found except C17.5 (R5), whose "R6's security baseline" the owner then
  renamed to the glossary's "the security baseline" (a rename, as the step's order allows); C17.b3 and C23.b1–b3 not
  applied (they wait for BASELINE-WHEN), as required.
- R3 owner DONE about 20:15 UTC (report verbatim `graphics-engineer-r3-consistency-result.md`): C9.12–C9.16, C10.12–C10.13;
  1 number (C9.13), 4 recommendation changes (C9.14; the bare 48 for the largest mode in R3-CHAR-REC1–REC3 now reads
  "the largest mode's N (48 or 54, MODES-3)", a value that changes only under MODES-3 (b): the verifier judges it), 27
  nits; no OBJECT; R3-LOOK-REC3's counts kept word for word (a look row). 0 exact hits; quotes: none gone; apply
  check: all 7 items found. Committed **41f9cf9**. Running: R1+R4 and R2 owners.
- R2 owner DONE about 20:19 UTC (report verbatim `netcode-architect-r2-consistency-result.md`): 20 recommendation
  changes, 94 nits; one **OBJECT**, not applied: R2-INT-F20's "4–9 % of one core" (r2:870, a legacy browser
  measurement on unrecorded hardware), the one exact hit left; its nit: C23.2's "r6:424-425" lacks "at commit
  8dee746". Quotes: none gone; apply check: 6 after texts differ by later renames or links (the owner lists them; the
  verifier judges). Committed **2684735**. OBJECT and nit to the advisor (prompt `prompts/advisor-object-1.txt`).
- Step 3d unit R3+R5+R6 launched 20:18 UTC as workflow run wf_1359fb87-b9e (task w6xk2bo06): verifier, then skeptic,
  both high, on R3, R5 and R6 at 41f9cf9 (final files); inputs `scripts/glossary-hits-3c-r356.tsv` (210 in-context
  hits, 0 exact), `apply-3c-r356.tsv`, `quotes-3c-r356.tsv`. The 10-row source sample moves to the R1+R4 unit (last).
- **OBJECT-1 ruled** (advisor, run wf_02baa00f-caf, about 20:21 UTC; verbatim `advisor-object-1-verdict.md`): APPROVE (b)
  with two changes: "one core" moves to "Retire (in context)" ("for a server time mark or a share of the reference
  server"), exact keeps "per core", "server core", "1 CPU core"; Keeps adds the legacy-measurement place; condition:
  r2:870 reads "4–9 % of one core of the measuring machine". The C23.2 nit: APPROVE ("at commit 8dee746"). Applied:
  the two R2 nits by the R2 owner (message `prompts/r2-object-1-nits.txt`); the glossary change by engine-architect in
  its "Used today" refresh (item 3 added to `prompts/engine-architect-r14-usedin.txt` before sending).
- R1+R4 owner DONE about 20:23 UTC (report verbatim `engine-architect-r14-consistency-result.md`): 25 recommendation
  changes (R1 16, R4 9), no OBJECT, C17.b1 waits; exact R1 13 → 0, R4 5 → 0; quotes: none gone; apply check: C9.4 and
  C9.8 differ by logged renames. Committed **09de883**. **Step 3c's five owners are all done** (R5 1da4e5f, R6 de0a003,
  R3 41f9cf9, R2 2684735 + e9ff1f9, R1+R4 09de883). Check at 09de883 (glossary before its refresh): exact 1 (r2:870,
  OBJECT-1's place, in context once the glossary changes), in context 581; term uses 945; first uses 122, 102 linked
  (`scripts/glossary-check-3c-prerefresh.txt`).
- "Used today" refresh plus OBJECT-1's glossary change sent 20:23 UTC to the same engine-architect agent (prompt
  `prompts/engine-architect-r14-usedin.txt`, HEAD 09de883). UNREAD_SOURCES.md re-run after step 3c: same counts,
  only the notes' commit moves (36cc386). The 10-row sample drawn: `unread-sample.md` (hinge 3, named 2, note 3, no
  tie 2), for the R1+R4 unit.
- **Step 3c complete**: GLOSSARY.md's "Used today" refresh, OBJECT-1's change and four stale header lines done by
  engine-architect (reports verbatim `engine-architect-r14-usedin-result.md`); field check: only Used today, the
  measuring-unit entry's three fields and the header changed. Committed **f67245e** (HEAD for steps 3d R2 and R1+R4).
  Final check at f67245e: **0 exact hits**, 607 in context (R1 129, R2 198, R3 80, R4 55, R5 49, R6 96), after the
  script counts a bare number inside a compound ("48-player", the skeptic's gap: +27 hits) and not in decimals.
- **Step 3d R3+R5+R6 DONE** (run wf_1359fb87-b9e, about 20:34 UTC; reports verbatim `verifier-r356-3d-result.md`,
  `verifier-r356-3d-skeptic-result.md`): PASS WITH NOTES; all 52 approved changes present; 11 shared and 31 cross-topic
  questions word for word; 210 in-context hits judged, 5 wrong; skeptic: 2 more unsure (counted wrong), 2 hash-map
  pattern variants, the compound gap (fixed; 15 new R3/R5/R6 hits in `scripts/glossary-hits-3c-r356-added.tsv`). One
  claim checked at its source (Pages' static requests free: confirmed; egress part unsupported). Labels unchanged.
  For the owners: R6 r6:46 TL;DR scope, r6:709 both MODES-3 values, r6:799 and r6:665 bare 48, R5 r5:401-402, r5:1477,
  r5:1561, r5:2087, nits (R6-OBS-REC4 pointer, R6-HOST-REC1 Rests on). For the advisor: R3-LOOK-REC3's bare 48 (a look
  row) and R3's marks without a statistic (prompt `prompts/advisor-3d-r3.txt`).
- Launched 20:39 UTC: the advisor on the two R3 items (run wf_5f85b87e-ee4, task wwhsfkwhd); step 3d R2 (xhigh; run
  wf_efa16a80-44a, task wc9qcslll) and R1+R4 (high; run wf_5519b651-63d, task wf6cv24xc) units, each verifier then
  skeptic, on f67245e. Running: 3 agents. If cut off: resume each with its script path and resumeFromRunId.
- The advisor's R3 ruling (run wf_5f85b87e-ee4, about 20:40 UTC; verbatim `advisor-3d-r3-verdict.md`): item 5 APPROVE (b),
  a term note after R3-LOOK-REC3 in the advisor's words; item 6 APPROVE (a), "p99" written into the listed marks, each a
  recommendation change; engine-architect corrects R3's "Used today" line under Time statistics afterwards.
- Step-3d fix round sent about 20:41 UTC to the R6 and R5 owners (messages `prompts/r6-3d-fixes.txt`,
  `prompts/r5-3d-fixes.txt`); R3's (`prompts/r3-3d-fixes.txt`) goes when a slot frees. Then a final check of only
  what the fix round changed (verifier, high), and the glossary's Time-statistics line for R3.
- Fix round: R5 DONE (2 recommendation changes, r5:1429 and r5:1487; nits; r5:1426 kept: "the 48-player scale" is
  VISION_V2's design point), committed **d768190**; R6 DONE (4 recommendation changes, r6:665, 709, 744, 799; TL;DR
  scoped to Workers; r6:212 and r6:256 kept: the legacy perfbots figure), committed **10662c4**; replies appended
  verbatim to each owner's result file. Both: 0 exact hits, no quote gone. R3's fix round sent about 20:43 UTC.
- R3's fix round DONE about 20:46 UTC (replies appended to `graphics-engineer-r3-consistency-result.md`): the advisor's two
  rulings applied (term note outside R3-LOOK-REC3, the row unchanged; p99 written into the listed marks), plus
  R3-AUDIO-REC1's two marks, which the owner found by the same rule and the session asked it to write out
  (`prompts/r3-3d-fixes-2.txt`); compound 48s: r3:885 and r3:1248 renamed, r3:1396 kept (the inventory's words) with
  the term beside. 7 recommendation changes. Committed **1b69364**. Glossary's Time-statistics R3 line sent to
  engine-architect (`prompts/engine-architect-glossary-r3-timestats.txt`).
- GLOSSARY.md's Time-statistics R3 line done (reply appended to `engine-architect-r14-usedin-result.md`); field check: only
  that Used today field; committed **d54f3c0**. Check at d54f3c0: 0 exact, 611 in context. Step 3d final check of R3,
  R5 and R6 launched 20:47 UTC (run wf_ca6f48ca-efd, task w6zjv4ew7; prompt `prompts/verifier-r356-3d-final.txt`;
  24 hits on changed lines in `scripts/glossary-hits-3d-final-r356.tsv`). Running: R2 and R1+R4 units, the final
  check (3 agents). Write-guard grep now allows `.checkpoints/v2-research.md` (it is on the allowed list).
- **Step 3d final check of R3+R5+R6 DONE** (run wf_ca6f48ca-efd, about 20:52 UTC; verbatim `verifier-r356-3d-final-result.md`):
  PASS WITH NOTES; all fix items closed; 24 hits on changed lines judged, 24 right; labels unchanged (R3 45 of 77, R5
  61 of 115, R6 21 of 42); no fetch. One open item: R6-TEL-REC1's 1 MB record mark fails under MODES-3 (b) (54 players
  ≈ 1.08 MB); nits: r3:1243's "adds … (p99)", GLOSSARY:156's credit for R3-AUDIO-REC1, R6's part-7 lines lack types,
  r5:1426 could say "the design point". Sent about 20:53 UTC: the R6 item and nit (`prompts/r6-3d-final-fix.txt`), the
  R3 nit (`prompts/r3-3d-final-fix.txt`); the R5 and glossary nits ride with the next round.
- The final check's follow-ups DONE about 20:53 UTC: R3's nit ("raises that p99 by less than 0.3 ms"), committed **902c249**;
  R6-TEL-REC1's record mark ("a 24v24 record is at most 1 MB uncompressed (A-10), and a record of the largest mode at
  most 1 MB × N / 48 (arithmetic: 1.125 MB at N = 54 under MODES-3 (b))"; the session redid 54/48 = 1.125) and R6's
  typed part-7 lines, committed **1bfd235**. R3, R5 and R6 are final for step 3d (the R5 nit and the glossary credit nit
  ride with the next round; the completeness critic reads the final state).
- **Step 3d R1+R4 DONE** (run wf_5519b651-63d, about 20:57 UTC; verbatim `verifier-r14-3d-result.md`,
  `verifier-r14-3d-skeptic-result.md`): PASS WITH NOTES; all 50 R1/R4 approved changes present (C9.4, C9.8 by logged
  renames); 11 basis claims confirmed (2 by curl, raw.githubusercontent.com and doc.rust-lang.org, both 200): R1 keeps
  "verified (every recommendation claim)", R4 keeps "50 of 52"; 184 hits judged, 5 wrong; skeptic 1 more (r1:2049's
  "25 % of the server tick budget" names the wrong whole). Resolution 1's table recomputed: N = 48 and 25 % option all
  match; N = 54 at 60 Hz only if the 0.7 ms interest mark scales ×1.125 (unstated); N = 54 at 30 Hz sums of rounded
  parts, 0.01 ms high. Source sample: 7 of 10 hold, 3 wrong (UNREAD script faults).
- UNREAD script fixed (whole addresses; name-only hinge ties; 19 ties read by hand and listed in the script; units'
  notes only): 33 needed-for cells corrected (hinge 44 → 72, named 4 → 3, note 24 → 23, no tie 108 → 82); every hinge
  row now tied back except 3 sources no not-read table lists (named in the header) and R5's "–" row. Committed
  **f2239f9**; changed rows in `scripts/unread-changes-3d.tsv` for the final check.
- Fix round sent about 21:01 UTC: R1+R4 (`prompts/r14-3d-fixes.txt`: r1:2049, five bare 48s, pointers and Rests-on nits)
  and R5's design-point nit (`prompts/r5-3d-final-nit.txt`). GLOSSARY items (single events have no p99; §1's N = 54
  notes; the R3-AUDIO-REC1 credit; Used today's "25 % of the server tick budget") wait for the R2 unit's results.
- **Fifth spend-limit cut**, about 21:04 UTC on 2026-10-30 ("resets 1:10am (UTC)"): the R2 unit's verifier had just finished
  (report verbatim `verifier-r2-3d-result.md`: PASS WITH NOTES, R2 keeps "verified", 10 claims confirmed, 1 wrong
  (r2:3896 attributes CLAUDE.md's "Design for 48+" to VISION_V2), 198 hits judged, 16 wrong (bare 48s); table of
  resolution 1 recomputed: all match except the N = 54, 30 Hz rounding; GLOSSARY:74-75 and :168 use the retired "whole
  tick"); its skeptic failed at once; the R1+R4 owner stopped part way through its fix round (R1 mostly applied, R4
  part; working tree, notes saved). The night-stop reminder fired 22:51 UTC with nothing running. The session's worker
  restarted; resumed 2026-10-31 about 10:13 UTC (11:13 Amsterdam, inside the day window): saver restarted; the R1+R4
  owner resumed by message; the R2 skeptic resumed with resumeFromRunId (the verifier cached).
- Ultracode on (system reminder, about 18:46 UTC): step 3d keeps the plan's three verifier units and adds two
  read-only checks as more evidence: a skeptic per unit that tries to refute the verifier's "right" judgments of the
  in-context hits (same rule, when unsure wrong), and a completeness critic across all six topics after the units
  (prompts from `scripts/mk3d.py`). Their findings go the same way (owner fixes, final check). Units run as one
  workflow per unit (verifier, then skeptic), three at once, within the 4-at-once limit.
- Next: commit each owner's file as it returns after the write-guard check (and save its return verbatim as
  `<agent>-<k>-consistency-result.md`); then the GLOSSARY "Used today" refresh (engine-architect); then step 3d (three
  verifier units, `scripts/mk3d.py 7c6e0d7 <after>`, `scripts/unread_sample.py`), docs-keeper, gate, report, PR ready.
  Research stops at 00:10 Amsterdam (23:10 UTC).

## Next steps (from 30 October, 18:00 Amsterdam; superseded in part by the Advisor's decisions above)
1. Restore the Night shift cron (0-7) and remove issue #16's "Tonight only" rule line.
2. Insert the critic's 41 items as cross-topic open questions (R<n>-XT-OQ<k>) in the affected topic files, in the
   critic's words (main session, mechanical); no re-check (plan).
3. docs-keeper: STATUS, ROADMAP, docs/research/README.md status column, REPO_MAP (new files); the stale STATUS lines
   (the Routines line, the R1–R6 rows); then the gate and `node .claude/scripts/repo-map-check.mjs`.
4. The report `docs/reports/<date>-v2-research.md` with /advisor-report: coverage table, source counts by tier, the
   unread list (SRC-ACCESS), the critic's list, the SPEND-1 statement, the fetch-method audit, VERIFY-FETCH, the three
   spend-limit cut-offs. Fold this checkpoint into it and delete `.checkpoints/v2-research*` in the last commit.
5. Mark PR #28 ready; the advisor agent reviews its head; Mustafa's "merge #28".

## Earlier next steps (done)
1. Count the 11 unit results (a draft file each, ending sources lists); restart any unit with no draft: its saved
   prompt plus "resume from your notes at <notes path>".
2. Assemble (save each prompt first under `prompts/`): engine-architect → `docs/research/r1-simulation-core.md`,
   netcode-architect → `docs/research/r2-netcode.md`; nothing new claimed; the prompts say the inventory's marks are
   working recommendations and "look brief pending" rows stay pending (advisor condition A6).
3. Verify: verifier on R1 (high), verifier on R2 (xhigh), rust-reviewer on R1 + R2's server stack (xhigh),
   security-auditor on R2's protocol (xhigh). The verifier counts a claim confirmed only with the exact quoted source
   text (condition A8); default "unsupported" when it can't confirm.
4. Fix (owners, xhigh); a second verifier pass only where a fix changed a recommendation, a number or a source.
5. Wave 1 tally (shape followed, share of claims unsupported, what went unread); then wave 2 (R3, R4, R5, R6 with the
   amended R6 scope), the consistency critic, docs-keeper, gate, report, PR.

## Carried forward (into the report, the synthesis and later prompts)
- Condition 9: "the synthesis plan verifies every ADR claim and number, including those from R3–R6".
- The security baseline is deferred by the Advisor (2026-10-27): the report's coverage table shows it as "deferred by
  the Advisor, 2026-10-27", not as a gap; it goes to the synthesis: ADR 0006 lists it as open, or it is researched
  before 0006 is accepted (condition A3, A4).
- R1's cross-origin isolation question (the headers shared memory needs) stays in R1; R6's hosting part states, per
  host, whether it can send those headers (condition A4).
- If docs-after-r7 has not merged when research reaches its docs-keeper step, condition 11 applies in full: this PR
  adds the earlier STATUS and ROADMAP entries itself (condition A5).
- The research PR's "For Mustafa" says it merges only on his "merge #<n>", because the PR Advisor routine is off
  (condition A7).
- Night stop (condition A2): background agents have no workflow cache. At 00:10 Amsterdam, stop every agent still
  running and list which; after 08:30, restart each with its saved prompt plus "resume from your notes at <path>".
- `checkpoint.mjs save -m` without the "checkpoint(<slug>):" prefix (the script adds it; 8ff1c74 has it twice).

## Per-topic table
| Topic | Owner | Units | Drafts | Assembled | Verification | Prompts |
|---|---|---|---|---|---|---|
| R1 | engine-architect | 6 | 6/6 | yes, a6a97fe; fixed e4cda31; second fix + final check ae492a6 (verified, every recommendation claim) | verified by six verifiers (run wf_3fe63170-264): 224 claims, 218 confirmed, 1 partly, 1 wrong (DET F25b), 4 unsupported (BND 2, ABL 2), 1 not checkable; rust-reviewer REVISE (facts hold: 110/112 Rust claims confirmed across R1 and R2; missing Rust risks); fixing | `prompts/verifier-r1-*.txt` |
| R2 | netcode-architect | 5 | 5/5 | yes, 7907f5c (147 F, 36 REC, 70 OQ; 113 read, 26 not read); fixed e4cda31; second fix + final check ae492a6 (verified, every recommendation claim) | verifier-r2: 135 claims, 133 confirmed (TRN-F12 wrong location), 0 wrong, 2 unsupported (SYNC-F34, ROOM-F17 part); security-auditor: REVISE, no Critical, 4 High (H1 backtrack window, H2 no-spread if shared seed, H3 no input limits, H4 weaker WebSocket fallback), 9 Medium; rust-reviewer REVISE (quinn advisories; client-sized allocations, panic and overflow policy, snapshot allocation, FMA by target); fixing | `prompts/netcode-architect-r2-*.txt` |
| R3 | graphics-engineer (high) | 1 owner, 7 question drafts | 7/7 (DONE 21:21) | yes, 605c09d (92 F, 25 REC, 33 OQ; 62 read: 61 official, 1 other; 22 not read) | spot-check (run wf_1f477dcb-7cb): 44 of 77 key claims checked, 44 confirmed; critical read: 4 more than nits (L1, L2, C1, C2) + C3–C9; owner fix applied (ddd45df: 9 recs, 4 numbers, 5 new OQs, 38 OQ in all); second pass (run wf_7260c3bd-5b1, resumed after the cut-off): L1, L2, C1, C2 closed, 14 confirmed, 0 wrong, 2 unsupported in new OQs; its two one-sentence gaps and the nits applied by the session (c22675f). **Done: spot-checked (45 of 77 key claims)**, review stopped at nits | `prompts/graphics-engineer-r3.txt` |
| R4 | engine-architect (xhigh) | 1 owner, 5 question drafts | 5/5 (DONE 21:28) | yes, abdf477 (62 F, 18 REC, 23 OQ; 46 read: 36 official, 8 shipped game, 2 other; 32 not read) | spot-check (run wf_d3f13cf7-77e): 49 of 51 key claims, 46 confirmed, 2 wrong (PHYS-F20, FLOORS-F5), 1 unsupported (NAV-F15), slightly more than nits; owner fix applied after the cut-off (6e87db8: 10 recs, 3 numbers, 1 source row, 18 nits; label then "49 of 51"); second pass (run wf_50c96d66-3f6): all fixed, 17 confirmed, nits only, applied by the session (16212b6). **Done: spot-checked (50 of 52 key claims)**, review stopped at nits | `prompts/engine-architect-r4.txt` |
| R5 | test-engineer (high) + infra-engineer CI helper (high) | 1 owner, 5 drafts + helper draft | owner DONE 2026-10-29 about 22:19 (cut off once); helper DONE 2026-10-28 about 21:43 (44 F, 10 REC, 13 OQ) | yes, 9a623a0 (117 F, 36 REC, 44 OQ; 108 read: 105 official, 2 shipped game, 1 other; 38 not-read rows) | spot-check (run wf_888e5b21-090): 60 of 115 key claims, 60 confirmed; critical read: 2 substantive (real Safari does not follow from Playwright's F8; the single required CI check can be skipped) + wording beyond nits; owner fix applied (c923d8d: 8 recs, 1 number, 34 nits, TL;DR to 5 lines; label then "60 of 115"); second pass (run wf_22f1f23c-b99): all ten points closed, 13 confirmed, 2 unsupported in wording the fix added, nits only, applied by the session (8948522). **Done: spot-checked (61 of 115 key claims)**, review stopped at nits | `prompts/test-engineer-r5.txt`, `prompts/infra-engineer-r5-ci.txt` |
| R6 | infra-engineer (high) | 1 owner, 4 question drafts + R6-SEC-OQ1 | 5/5 (DONE 21:31) | yes, 9d5e78a (43 F, 13 REC, 23 OQ; 42 read: 41 official, 1 other; 17 not read) | spot-check (run wf_12f840d1-878, resumed after the cut-off): 19 of 40 key claims, 19 confirmed; critical read: 4 medium (certificate plan departs from R2-TRN-REC5 silently; option K4's comparison; Rests-on gaps; Hetzner "unavailable" left out of HOST-REC2) + 9 low; owner fix applied (6 recs, 2 numbers, 23 nits); second pass (run wf_d5608bdb-da9): four medium closed, 19 confirmed, 2 wrong and 2 unsupported, all nits, applied by the session (1decc70). **Done: spot-checked (21 of 42 key claims)**, review stopped at nits | `prompts/infra-engineer-r6.txt`; security baseline deferred (Advisor) |
Agent ids are internal to this session and are not written here; the saved prompt plus the notes path is the restart
key (condition A1).

## Open findings
- Unread sources (SRC-ACCESS (a)): Valve's developer wiki answers HTTP 403 to the fetcher; GDC Vault redirects to a
  maintenance page off the allow list (and the Overwatch talks are video only); Unreal's docs are outside the allow
  list. The units used other copies where they could (Valve's SDK code on GitHub, a community GAS write-up). The
  assemblers list every recommendation that hinges on an unread source; the full list goes to the Advisor with the
  wave-1 tally.
- Spend: the org's spend limit cut all 11 units at once about 7 minutes after launch (2026-10-27; reset 21:20 UTC),
  and cut 5 agents again on 2026-10-28 at about 16:26 UTC (R3, R4, R6 owners; the second pass had just finished),
  reset 20:00 UTC. Now 4 agents at once.

## Key numbers
- Wave 1 agents planned: 11 units + 2 assemblers + 4 checkers + 2 fixers + up to 2 second passes (about 21).

## Decisions taken
- BRANCH-2 (a) and SPEND-1 (b): the Advisor, 2026-10-27 (R7 session; `request-and-decisions.md`).
- kit-v3-1 shelved; research daytime only and night shift no-web items only until it returns; PR Advisor routine off,
  the advisor agent reviews PRs in the day session; R7-MARKS (a) with look-brief rows pending; MANAGED (b); R6 without
  the security baseline: the Advisor, 2026-10-27, this session (`plan-amendments.md`).
- Amendments approved with 8 conditions: the advisor agent, 2026-10-27 18:22 UTC (`advisor-amendments-verdict.md`).
- PR #27 (R7) merged by the R7 session at 17:36 UTC on Mustafa's go (merge 07ab240).

- KIT31-STATE (b) and SRC-ACCESS (a): the Advisor, relayed by Mustafa about 21:36 UTC: the R7 session finishes kit-v3-1
  on Opus 5.5 and its PR goes to Mustafa ("shelved" meant "not in this session"); unread sources stay open questions
  for now, the full list comes with the wave-1 tally, and the Advisor looks for another copy of any source a
  recommendation hinges on. The assembler prompts list those recommendations (rule 5 added before launch).
- PR Advisor routine: off (Mustafa, 21:35 UTC; get_trigger reads enabled false).

- Night window (Mustafa, 2026-10-27 about 23:00 Amsterdam): "go back to the old schedule" and the night shift ends at
  07:30 instead of 08:30. Done: the Night shift routine's cron is back to `CRON_TZ=Europe/Amsterdam 10 0-7 * * *`
  (starts 00:10 to 07:10). So research's day window is now 07:30 to 00:10 (plan condition 2, amended). The
  NIGHT_SHIFT.md window text (stop by 08:30, no new item after 07:45) still needs its rules PR (decision WINDOW-0730).
- R1 verification split into six verifiers, one per brief question (CHECKPOINTS §6; the same rule, smaller units);
  prompts `prompts/verifier-r1-<unit>.txt`. Not launched tonight: Mustafa warned the usage limit is near.
- PR #29 (docs-after-r7) merged on Mustafa's "Merg #29" after the advisor's APPROVE of d973b3f (main e9f11be).

- WINDOW-0730 (a): the R7 session puts the 07:30 end into kit-v3-1 · NIGHT-RESEARCH (a): keep the rule (research by
  day only) · spend limit: run fewer agents at once (4–5), the same total work, so units finish before a cut-off:
  the Advisor, relayed by Mustafa, 2026-10-28 about 15:50 Amsterdam.

- Method deviation found at the R2 tally (corrected 2026-10-28 17:15 from the transcripts, see `wave1-tally.md`: it
  was two of R1's six verifiers, tick and abilities, R2's verifier, and also the R2 fixer): verifiers fetched sources with curl and matched quotes
  by exact text on the raw page, instead of WebFetch (prompt rule 2/3). Stronger evidence (exact text), but outside
  the WebFetch permission layer. Audit of every agent transcript: 23 curl calls plus the R2 verifier's loop over
  `scratchpad/r2v/bin/urls.txt` (111 URLs); every host is on the allow list except api.github.com, read once by the
  advisor's #29 review on the session's own instruction. For the Advisor: VERIFY-FETCH.

## Running work
- R2 chain: run wf_bce24fc0-5c6 DONE (3/3): reports verbatim in `verifier-r2-result.md`,
  `security-auditor-r2-protocol-result.md`, `netcode-architect-r2-assemble-result.md`.
- Rust review: run wf_38e94d0f-403 DONE: report verbatim in `rust-reviewer-r1-r2-result.md`, table in
  `rust-reviewer-r1-r2.md`.
- Fix: run wf_7dd61ce0-f84 DONE; topic files committed e4cda31 (R1: 185 F, 76 OQ; R2: 152 F, 72 OQ). Change logs by
  type in `engine-architect-r1-fix.md` and `netcode-architect-r2-fix.md`.
- Second pass: scope built by script from the files at a6a97fe / 7907f5c vs e4cda31 (`second-pass-scope-r1.md`,
  `-r2.md`): R1 5 new findings, 9 basis claims without a first-pass verifier "confirmed" (incl. ECS-F7, F19, F20,
  BND-F32 added to Rests-on lists at the fix), 46 changed basis findings, 29 changed recs; R2 5 new, 1 (TRN-F36), 32
  changed, 27 changed recs. Prompts `prompts/verifier-r{1,2}-second-pass.txt` (WebFetch only, no curl, pending
  VERIFY-FETCH); script `v2-research-second-pass-run1.js`; verifier R1 high, R2 xhigh, 2 agents. Run wf_64d0bbcd-730
  DONE (2/2, 17:24 Amsterdam), reports verbatim in `verifier-r{1,2}-second-pass-result.md`. Both PASS WITH NOTES:
  R1 42 confirmed, 2 wrong (DET-F34's stream note, so DET-REC4 and options Q4; DATA-REC6's file:// clause), 1
  unsupported (ECS-F34, an absence), 11 label only; R2 17 confirmed, 2 wrong (SYNC-F36's "200 ms of movement" clause;
  ROOM-REC3's "no header option": the W3C spec has `headers`), 2 unsupported (TRN-REC3 "Safari only" HTTP/2; "no
  transport RTT" on WebSocket), 25 label only; minor unmarked reasoning in several recs.
- Fix 2 + final check: run wf_0ad9f0c0-59d (task w0uq5j48e), launched 21:03 Amsterdam, script
  `v2-research-fix2-run1.js`: pipeline per topic, the owner's second fix (xhigh; prompts `prompts/*-r{1,2}-fix2.txt`,
  notes `*-fix2.md`), then a final check of only what it changed (verifier R1 high, R2 xhigh; prompts
  `prompts/verifier-r{1,2}-final-check.txt`, notes `verifier-r{1,2}-final-check.md`). DONE 4/4 (21:24 Amsterdam),
  reports verbatim in `<agent>-r{1,2}-fix2-result.md` and `verifier-r{1,2}-final-check-result.md`: R1 7 confirmed, 0
  wrong, nits only; R2 14 confirmed, 1 wrong as worded (BUD-REC5, load-bearing for nothing), nits only. The session
  applied the nits in the verifiers' words and added a "Final check" subsection to each part 7; gate PASS; committed
  ae492a6. **Wave 1 is verified** (both headers: "verified (every recommendation claim)").
- R1 verify: run wf_3fe63170-264 DONE, 6/6 returned; reports verbatim in `verifier-r1-results.json`, tables in
  `verifier-r1-<unit>.md`.
- Wave 2 owners R3, R4, R6: background agents launched 16:16 UTC (17:16 Amsterdam); all three cut off by the org
  spend limit at about 16:26 UTC (HTTP 429, "resets 8pm (UTC)"), notes saved: R3 Q1 drafted
  (`graphics-engineer-r3-r3-wgpu-draft.md`), R6 Q1 drafted (`infra-engineer-r6-r6-host-draft.md`), R4 none yet.
  Resumed by message at 20:03 UTC: R3 and R4; R3 DONE at 20:21 UTC (file committed 605c09d); R6 resumed by message
  at 20:22 UTC. R3 spot-check then R3 fix: workflow run wf_1f477dcb-7cb (task wytqhqr2s, script
  `v2-research-r3-check-fix-run1.js`, prompts `prompts/verifier-r3.txt`, `prompts/graphics-engineer-r3-fix.txt`),
  launched 20:27 UTC. R5 CI helper: background agent launched 20:27 UTC (`prompts/infra-engineer-r5-ci.txt`, notes
  `infra-engineer-r5-ci.md`). R4 DONE at about 20:28 UTC (file abdf477; it departs from R1's Rapier assumption openly:
  R4-PHYS-REC1 recommends parry3d plus own kinematic movement and lists the R1 recommendations it changes, a
  consistency-critic item). R5 owner: background agent launched 20:29 UTC (`prompts/test-engineer-r5.txt`, notes
  `test-engineer-r5.md`). R6 DONE at about 20:31 UTC (file 9d5e78a; Hetzner's site showed every EU plan "unavailable",
  R6-HOST-OQ1). R4 spot-check then fix: run wf_d3f13cf7-77e (task wnbsqi9ru), launched 20:33 UTC. Next slot: the R6
  spot-check then fix: run wf_12f840d1-878 (task w0ks9l4k4), launched 20:43 UTC. R5 CI helper DONE at about 20:43 UTC.
  R3 check-fix DONE at about 20:45 UTC (reports `verifier-r3-result.md`, `graphics-engineer-r3-fix-result.md`; the
  fixed file is in ddd45df). R3 second pass: run wf_7260c3bd-5b1 (task wyeyt2bgh), launched 20:46 UTC.
  Running: R3 second pass, R4 and R6 check-fix workflows, and the R5 owner: 4 agents.
- Third spend-limit cut, 2026-10-28 about 20:50 UTC ("resets 1am (UTC)"): R4's spot-check had finished
  (`verifier-r4-result.md`: 49 of 51 key claims checked, 46 confirmed, 2 wrong (PHYS-F20 "any two shapes", FLOORS-F5's
  count), 1 unsupported (NAV-F15's hashbrown default); slightly more than nits); cut part way: the R4 fix, the R6
  spot-check, the R5 owner (1 of 5 drafts: layout), the R3 second pass. Notes saved for each. The 23:50 night-stop
  reminder found nothing running (it fired 22:51 UTC, after the cut). The session stayed idle until Mustafa's "Try
  again" on 2026-10-29 about 21:07 UTC.
- Resumed 2026-10-29 about 21:10 UTC (22:10 Amsterdam): workflows resumed with resumeFromRunId (R4 wf_d3f13cf7-77e:
  spot-check cached, fix live, task wewlvbjlr; R6 wf_12f840d1-878, task wtgxyoez2; R3 second pass wf_7260c3bd-5b1,
  task wf7io9kwu); the R5 owner by message. Saver restarted (task bt9yig2mb). Night-stop reminder for 23:50 tonight
  set (send_later, 22:50 UTC). 4 agents at once.
- 2026-10-29: R3 second pass DONE (R3 finished, c22675f). R6 check-fix DONE (reports `verifier-r6-result.md`,
  `infra-engineer-r6-fix-result.md`); R6 second pass launched 21:20 UTC (run wf_d5608bdb-da9, task w4k55nhch; prompt
  carries the session's note that the inventory's CM4 row also holds the 0.80–1.00 ms figure, so R2's "legacy CM4" is
  not wrong). R5 owner DONE (9a623a0); R5 spot-check then fix launched 21:20 UTC (run wf_888e5b21-090, task we8kho4jw).
  R4 fix DONE (6e87db8; report `engine-architect-r4-fix-result.md`); R4 second pass launched 21:21 UTC (run
  wf_50c96d66-3f6, task w0s7mnauk). R6 second pass DONE; its nits applied by the session (1decc70): R6 finished.
  R4 second pass DONE; its nits applied by the session (16212b6): R4 finished. R5 check-fix DONE (c923d8d); R5
  second pass and the consistency critic launched 21:42 UTC (runs wf_22f1f23c-b99, task whb1251wf; wf_23aff73b-38d,
  task wgashsj8a; the critic reads R5 last). Running: 2 agents. The 23:50 stop applies to both (notes saved).
- R5 second pass DONE; its nits applied by the session (8948522). **Wave 2 is done: all six topics final.** Running:
  the consistency critic (1 agent).
- Main merged in (569f792: PR #29's STATUS and ROADMAP, no conflicts), so the docs-keeper step builds on main's STATUS.
- Consistency critic DONE (run wf_23aff73b-38d, finished about 21:56 UTC before a container restart; its output file
  survived): 41 items (12 contradictions, 13 hidden dependencies, 15 gaps, 1 wrong cross-reference), saved verbatim as
  `engine-architect-consistency-result.json`; notes `engine-architect-consistency.md` (end with DONE). Top item: "25 %"
  of the tick means the sim alone in R1, the whole tick in R2-BUD-REC6 and the sync work in R2-SYNC-REC9.
- **Pause (Mustafa, 2026-10-30 about 00:35 Amsterdam):** "Today you and the nightshift bot are only allowed to work until
  3 am, because I will need the usage for tomorrow, we will continue the project tomorrow after 18:00 again." Done: the
  Night shift routine's cron is `CRON_TZ=Europe/Amsterdam 10 0-2 * * *` for tonight (its prompt cannot be changed from
  this session: only from the conversation it posts into); issue #16's rules carry a dated "stop by 03:00, no new item
  after 02:30, no work before 18:00 on 30 October" line. A send_later at 18:05 Amsterdam on 30 October
  (trig: "Resume v2-research and restore the night shift") restores the cron to 0-7, removes the #16 line, and resumes
  this change-set. Nothing of this change-set runs before then.
- Next: when the second pass returns: the owners fix R1 and R2 where it found wrong or unsupported claims (fix rule),
  then the R5 owner and helper; then each wave-2 file's verifier spot-check (high), fixes, the consistency critic,
  docs-keeper, gate, report, PR. At most 4–5 agents at once (the Advisor, 2026-10-28).
- Night stop (condition A2): at 23:50 Amsterdam, stop every research agent still running and list it; restart after
  07:30 with its saved prompt plus "resume from your notes at <path>" (a send_later reminder is set for 23:50).
- Saver: restarted 21:03 Amsterdam (task blnt2hgiq; the 17:17 one ended with the worker restart), stops after 110
  minutes (about 22:53).
