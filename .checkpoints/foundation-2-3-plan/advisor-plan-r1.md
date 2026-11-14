TASK: advisor plan review, foundation step 2.3 plan revision 1 (plan.md at 03d05f5, branch claude/sweet-mayer-grsukw), day session.

PLAN (blocks, in order):
B1. Read DECISIONS.md entry 2026-11-13 18:47:55 UTC (items 1-9, how the plan ends) + merge 4460ef5 message (his two rules of 2026-11-14).
B2. Read plan.md part 1 (top, Parts 0-2).
B3. Read plan.md part 2 (Parts 3-5).
B4. Read plan.md part 3 (Parts 6-8+, end, decisions).
B5. Read reading.md, approach.md (skim), checkpoint.
B6. Spot-check ADR 0002 rows :1305-1328, :1463-1470, PM-0002-39..62 against plan Part 2.
B7. SPIKE_PLAN.md:440-467 vs needs-result reading, F-FLOAT.
B8. STATUS_ARCHIVE OVERFLOW-XCRATE-CI + follow-ups/open bugs; report 2026-11-13-foundation-2-3-docs findings.
B9. Appendix K (kit-texts.md) vs main texts; quotes vs DECISIONS.md.
B10. Rules check (CLAUDE.md, advisor.md) against proposals; required-check sequence Part 8.
B11. Write findings, rulings, decisions read; DONE.

NOTES:
- B1 done. His task = DECISIONS.md:147-164 (items 1-9; ending: path, one-screen summary, table of checks, decisions, chat fullness). Kit rule DECISIONS.md:158 (every kit/gate/CI edit asked in chat with exact text, written only after his yes, chat in Accept edits; guard prompt is not a yes). Repo-wins rule :160 (stop and tell him in 3 lines). 4460ef5 msg items 2 (advisor tool before saving a step done) and 3 (report on own process only what a record shows, with time). Next: B2.
- B2 done (plan.md:1-388: header, one screen, Parts 0-3). Leads to check later (not yet findings):
  L-a: PM-0002-57 met at CS-E (plan.md:190): clause "block the merge" cannot be shown on CS-E's own PR before his ruleset requires `required` (added only after CS-E's first main push run, plan.md:354). Check ADR row text in B6.
  L-b: D2 "single required check" and D7 met at CS-E (plan.md:166,171) while ruleset still also requires gate+smoke until after CS-F (plan.md:371). Check ADR wording in B6.
  L-c: aggregate `required` compares ids with gate-v2 --list from the PR head (plan.md:330): a PR that drops a check from both workflow and --list passes. Check ADR wording of "removing a check fails the aggregate".
  L-d: PM-0002-54 machines (plan.md:187): check ADR for machine clause.
  L-e: clock ban built now (plan.md:134): check it rests on no open result (ADR 0004 part 7 / ADR 0002 part 7).
  L-f: CS-D before CS-C rationale cites routine-guard.mjs:87 (PR Advisor merge places incl tools/): verify.
Next: B3 (plan.md Part 4-5).
- B3a done (plan.md:389-519, Part 4 table). Leads:
  L-g: row 94 (plan.md:505) drops 2.3-docs finding 3 (no tools/ install, local smoke cannot start) "for 2.3". But CS-C1 edits smoke.mjs and gate.mjs; advisor.md step 2 day seat must run full gate incl smoke itself, Never "approve without running the gate yourself". Check finding 3 text + whether tools/node_modules exists here.
  L-h: row 47 OQ-0002-8-TEST (plan.md:458): "later" without a when. Check source 2.2 :388.
  L-i: row 28 partial drop of CI-ENFORCED-LATER "has a test" (plan.md:439); row 26 SAVER-SKIP-CI dropped (plan.md:437); row 13 dropped. Check source texts.
Next: B3b (plan.md Part 5).
- B3b done (plan.md:520-846: Parts 5-11, Decisions, For Mustafa). Findings/leads:
  F-1 (Medium, candidate): plan.md:730 "at every moment the guard and the ruleset require the same checks, or the guard requires more" and :765-766 (Risk 4) are false for the window after ruleset step 1 (adds required, :725) and before CS-F merges (guard still gate+smoke only, G-4 at :641). Order is still safe (ruleset is the binding wall; nothing loosened vs today). Fix: restate invariant truthfully, or reorder: CS-F merges first (guard requires all 3), then ONE ruleset edit replacing gate+smoke with required (fewer steps for him; guard >= ruleset always). Check guard mergeCheck scope.
  L-a reinforced: PM-0002-57 "block the merge" on CS-E's own PR (plan.md:344-345) impossible before ruleset requires required.
  L-j: 5.1 (plan.md:532-535) only covers Write; CS-E removes gate.yml and toolchain.yml (plan.md:352): route for protected deletions unnamed. Low, for CS-E's plan.
  L-k: KIT-EFFORT reading (plan.md:559-567) and MODELS-HOW (plan.md:817-826): docs-keeper excluded though his item 5 says "the 13 agent files"; check his 2026-11-12 words.
  L-l: S-1 (plan.md:649-656) says the allow rule means "also for routines" can run unattended: check routines' shell limits in guard (routine runs node on main's tracked scripts anyway?).
Next: B4 (Appendix K, kit-texts.md, quotes), then sources.
- B4 done (Appendix K, plan.md:842-1363). Leads/findings:
  F-2 (Medium, candidate): K-4 effort bullet (plan.md:950-956) tells "the caller" to pass effort max until PR #40 merges, with no carve-out for the netcode chat as a caller; after it takes main (must, before its next ready, Part 8 :719) it would raise its efforts, against his "keeps its models and efforts as they are" (quoted :943). Plan claims (c) keeps both sentences true (:826). Fix: add "except the netcode chat, which keeps its efforts". Also K-4 hardcodes MODELS-HOW (c): say K-4 text changes under (a)/(b).
  F-3 (Low): K-4 "How" (plan.md:946) `model: "fable"` value unverified as an Agent-tool model alias; extend the [to verify] to it; add "on his personal account" to the How line.
  F-4 (Low-Med, candidate): K-12a/f/g (plan.md:1124,1194,1208) widen "main breaks" (the no-plan revert trigger) from gate/smoke to every v2 job; labelled "same strength". Label as wider trigger for his yes.
  L-m: K-11 (plan.md:1262-1297) updates CLAUDE.md :68-69 and advisor.md :164-166 only; check other gate-file lists (advisor.md escalation cat 3 "today" list, Risk tiers "tools/ except gate files", tier-2 bullet, PR-Advisor allow-list text) for consistency.
  L-n: verify K-8 settings.json:12 form; K-10b L4 location in archive.
Next: B5 (reading.md, approach.md, checkpoint).
- B5 done (reading.md, approach.md, checkpoint). Note F-5 (Low): checkpoint "Key numbers" says 0 advisor-tool consultations while its consultations list has 1 (checkpoint, not plan). Next: B6 (ADR 0002 rows).
