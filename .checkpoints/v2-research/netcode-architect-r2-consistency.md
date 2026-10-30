Task: step 3c owner apply (R2): apply approved consistency resolutions, glossary terms and shared cross-topic OQs to docs/research/r2-netcode.md (prompt: .checkpoints/v2-research/prompts/netcode-architect-r2-consistency.txt)

## Plan (2026-10-30, before edits)
- Inputs read: prompt, plan-amendments-2.md, advisor-resolutions-verdict.md, resolutions.md (all 12; R2 changes: C1.7-C1.14 incl. C1.10b, C5.8-C5.10, C6.5-C6.7, C9.9-C9.11, C10.4-C10.11, C11.8-C11.12, C17.3-C17.4, C23.1-C23.2; NOT C17.b*/C23.b*), GLOSSARY.md, glossary-hits-split.tsv (R2: 20 exact, 193 context), cross-topic-oqs.md (R2: XT-OQ1-OQ15), glossary_check.py.
- Part 7 = "## Verifier spot-check"; add "### Consistency resolutions (2026-10-30)" at its end (before "## Sources").
- Course correction from the session (received mid-task): subsections are "### Shared open questions (the same words in every topic)" (MODES-3, ABL-PREDICT, SEC-SPREAD, BASELINE-WHEN for R2) then "### Cross-topic open questions (consistency critic, 2026-10-29)" (R2-XT-OQ1-15), both before "### Pointers between the R2 units". Not the prompt's "Shared open questions (consistency critic, 2026-10-30)". Type: nit (logged).
- Method: python apply script in scratchpad (whitespace-normalized unique match, idempotent), stage 1 resolutions, stage 2 glossary, stage 3 OQs, stage 4 header/part 7.
