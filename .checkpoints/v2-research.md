# Checkpoint: v2-research
- **Goal:** R1–R6 research files (`docs/research/r1-…` to `r6-…`), each in the README's 8-part shape, verified as
  the plan says, plus the README status column and the report · plan: `.checkpoints/v2-research/plan.md` (advisor
  APPROVE with 14 conditions, 2026-10-27, `advisor-verdict.md`) and `plan-amendments.md` (the Advisor's 2026-10-27
  instructions in this session), approved by the advisor with 8 conditions (`advisor-amendments-verdict.md`, part A)
- **Branch:** cs/v2-research (session_0173vVo5q59ameR6cnN9BENg; the main checkout, never switched while research runs)
  · **Base:** origin/main 07ab240 (R7 merged; `docs/LEGACY_INVENTORY.md` checked present)
- **Last update:** 2026-10-29 22:20 Amsterdam

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

## Next steps
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
| R4 | engine-architect (xhigh) | 1 owner, 5 question drafts | 5/5 (DONE 21:28) | yes, abdf477 (62 F, 18 REC, 23 OQ; 46 read: 36 official, 8 shipped game, 2 other; 32 not read) | spot-check then fix running: run wf_d3f13cf7-77e (launched 21:33) | `prompts/engine-architect-r4.txt` |
| R5 | test-engineer (high) + infra-engineer CI helper (high) | 1 owner, 5 drafts + helper draft | owner DONE 2026-10-29 about 22:2x (cut off once); helper DONE 2026-10-28 about 21:43 (44 F, 10 REC, 13 OQ) | yes, 9a623a0 (117 F, 36 REC, 44 OQ; 108 read: 105 official, 2 shipped game, 1 other; 38 not-read rows) | spot-check then fix running: run wf_888e5b21-090 (the fix prompt adds the session's shape finding: TL;DR 16 lines) | `prompts/test-engineer-r5.txt`, `prompts/infra-engineer-r5-ci.txt` |
| R6 | infra-engineer (high) | 1 owner, 4 question drafts + R6-SEC-OQ1 | 5/5 (DONE 21:31) | yes, 9d5e78a (43 F, 13 REC, 23 OQ; 42 read: 41 official, 1 other; 17 not read) | spot-check (run wf_12f840d1-878, resumed after the cut-off): 19 of 40 key claims, 19 confirmed; critical read: 4 medium (certificate plan departs from R2-TRN-REC5 silently; option K4's comparison; Rests-on gaps; Hetzner "unavailable" left out of HOST-REC2) + 9 low; owner fix applied (6 recs, 2 numbers, 23 nits; label "spot-checked (19 of 40 key claims)"); second pass on the fix running (run wf_d5608bdb-da9) | `prompts/infra-engineer-r6.txt`; security baseline deferred (Advisor) |
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
  Running: R4 fix, R6 second pass, R5 check-fix: 3 agents.
- Next: when the second pass returns: the owners fix R1 and R2 where it found wrong or unsupported claims (fix rule),
  then the R5 owner and helper; then each wave-2 file's verifier spot-check (high), fixes, the consistency critic,
  docs-keeper, gate, report, PR. At most 4–5 agents at once (the Advisor, 2026-10-28).
- Night stop (condition A2): at 23:50 Amsterdam, stop every research agent still running and list it; restart after
  07:30 with its saved prompt plus "resume from your notes at <path>" (a send_later reminder is set for 23:50).
- Saver: restarted 21:03 Amsterdam (task blnt2hgiq; the 17:17 one ended with the worker restart), stops after 110
  minutes (about 22:53).
