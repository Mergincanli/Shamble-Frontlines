Task: step 3c owner apply (R2): apply approved consistency resolutions, glossary terms and shared cross-topic OQs to docs/research/r2-netcode.md (prompt: .checkpoints/v2-research/prompts/netcode-architect-r2-consistency.txt)

## Plan (2026-10-30, before edits)
- Inputs read: prompt, plan-amendments-2.md, advisor-resolutions-verdict.md, resolutions.md (all 12; R2 changes: C1.7-C1.14 incl. C1.10b, C5.8-C5.10, C6.5-C6.7, C9.9-C9.11, C10.4-C10.11, C11.8-C11.12, C17.3-C17.4, C23.1-C23.2; NOT C17.b*/C23.b*), GLOSSARY.md, glossary-hits-split.tsv (R2: 20 exact, 193 context), cross-topic-oqs.md (R2: XT-OQ1-OQ15), glossary_check.py.
- Part 7 = "## Verifier spot-check"; add "### Consistency resolutions (2026-10-30)" at its end (before "## Sources").
- Course correction from the session (received mid-task): subsections are "### Shared open questions (the same words in every topic)" (MODES-3, ABL-PREDICT, SEC-SPREAD, BASELINE-WHEN for R2) then "### Cross-topic open questions (consistency critic, 2026-10-29)" (R2-XT-OQ1-15), both before "### Pointers between the R2 units". Not the prompt's "Shared open questions (consistency critic, 2026-10-30)". Type: nit (logged).
- Method: python apply script in scratchpad (whitespace-normalized unique match, idempotent), stage 1 resolutions, stage 2 glossary, stage 3 OQs, stage 4 header/part 7.

## Step 1: resolutions (applied 2026-10-30, script scratchpad r2c/apply1.py; every before quote matched exactly once)
- C1.7 SYNC-REC1 mark 4 (r2:2016-2017): recommendation (the share now has its value, 25 % of the tick period).
- C1.8 SYNC-REC9 mark 1: recommendation (listed).
- C1.9 BUD-REC6 heading and reason (as revised, item g): recommendation (listed).
- C1.10 BUD-REC6 spike (as revised, item h): recommendation (listed).
- C1.10b BUD-REC6 spike, new sentence: recommendation (inside C1.8-C1.11).
- C1.11 INT-REC3 spike: recommendation (listed).
- C1.12 ROOM-REC2 mark 1: nit (same 50 %, term only).
- C1.13 TRN-REC1 mark 2, sentence added: recommendation (how the transport runtime counts).
- C1.14 XU-OQ8 closing sentence: nit.
- C5.8 SYNC-REC3 list item added: recommendation (the client-not-predicted list gains abilities; same scope as before, now explicit).
- C5.9 SYNC-OQ10 closing sentence: nit.
- C6.5 SYNC-REC3 stream name: nit (server-only -> authority-only, offline Worker named; no change online).
- C6.6 SYNC-OQ11 sentence outside the dated bracket: nit.
- C9.9 SYNC-REC5 mark 3: recommendation (listed; 48+12 -> largest mode's N + 15).
- C9.10 INT-REC4 spike: recommendation (worst case -> the stress scene).
- C9.11 BUD-OQ5 closing sentence: nit.
- C10.4, C10.5 worked estimate row labels: nit.
- C10.6 INT-REC3 spike (+18v18v18 under (b)): recommendation.
- C10.7 ROOM-REC6 spike: recommendation.
- C10.8 ROOM-REC7 spike (test-mode list): recommendation.
- C10.9 BUD-OQ6, C10.10 XU-OQ13 closing sentences: nit.
- C11.8 BUD-REC6 spike (one vCPU of the CCX23): nit (unit name only, resolution 11 "nothing here changes a number").
- C11.9 TRN-REC1 mark 2 (1 CPU core -> 1 vCPU): nit (unit name, resolution 11).
- C11.10 ROOM-REC2 reference host: nit.
- C11.11 XU-OQ9 closing sentence: nit.
- C11.12 table E (r2:1721) and pointer (r2:3244), core -> vCPU: nit.
- C17.3 TRN-REC1 mark 5 cross-reference: nit (no policy change, resolution 17 "now").
- C23.1 TRN-REC5 production line, C23.2 TRN-OQ4 closing: nit (no position change). NOT applied: C17.b2, C23.b* (wait for BASELINE-WHEN).
- fmt: C5.8's list item indentation restored to 2 spaces (the regex kept the file's own indent): nit.
