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
- B6 done (ADR 0002 :1305-1328 rows, :1463-1470 step 2.3). Verified D1-D7 wording matches plan. Findings:
  F-6 (High): PM-0002-57 (ADR :1323) says failing/cancelled job PRs "turn it red and cannot merge", method "test pull requests on scratch branches; GitHub's merge box read"; D2 (ADR :1464-1465) "a TypeScript-only pull request runs and passes the single required check". Plan marks both met at CS-E (plan.md:166,:190) with planted failures on CS-E's own PR (plan.md:344-345), but `required` enters the ruleset only after CS-E merges (plan.md:354,:725) and becomes the single required check only after CS-F (plan.md:371,:728). The merge box cannot show "cannot merge" for `required` before then: evidence that would not hold. Fix: run the merge-box proofs (scratch PRs on scratch branches, as the ADR's method) after his ruleset step (1 for "cannot merge"; 2 for "single"), and write "met at his ruleset step after CS-F" (or have him add `required` while CS-E's PR is open, and say so in Part 8/9).
  F-7 (Medium): PM-0002-52 (ADR :1322) needs "every mutation of PM-0002-39 to PM-0002-51"; plan says met at 2.3-E (plan.md:185, D1 :165), but its own rows put mutations/controls of -41 (server, slice 3), -42 and -46 (editor, slice 5), -47 (golden, slice 1), -51 (WebSocket slice 3; draw-call baseline) later. Strictly met at the last of those. Summary plan.md:56-57 lists -41,-42,-46,-47 only (omits -44 per-crate, -51, -52). Fix the Met-at cells and the summary list.
  F-8 (Medium): PM-0002-53 Met-at "2.3-B at the earliest" (plan.md:186) is not a named change-set; settled by ruling (a).
  Note: Rule 3 (plan.md:111-112) is a deferral by choice (his "may"), not a needs-result reading; CLAUDE.md's "for later" clause is about engine features, not ADR-traced gate checks. Low: relabel honestly.
- B7 done (SPIKE_PLAN :440-467; ADR 0002 part 7 :433-465; ADR 0004 :262-264, :1143, :1347; crates/sim/src/lib.rs has no f32/f64). F-FLOAT confirmed: sqrt on allow-list, OQ10 decided by sim core spike (PM-0004-22); clock ban rests on std clock panic on wasm32 + tick replay, no OQ; hash-map std ban no OQ; dependency map types wait (rust-reviewer list once ADR 0004 deps fixed). Ruling (b): agree; condition: 2.3-S merges before any change adds f32/f64 arithmetic to sim.
Next: B8 (OVERFLOW-XCRATE-CI text, archive follow-ups, 2.3-docs report findings).
- B8 done (archive :631-662 OVERFLOW-XCRATE-CI; STATUS :95; crates have no dependencies; 2.3-docs report :287-296; archive SAVER-SKIP-CI :693, OQ-0002-8-TEST :853; 2.2 report :388). Notes:
  Ruling (a): move the anchor (its own text allows it, archive :656-658; STATUS :95 already names "or the first crate to depend on sim"); no crate depends on sim, so no cross-crate exposure until then; test-only dep now would add sim scaffolding and still miss the Wasm half. Tighten: one trigger = the first change-set that adds a dependency (normal or dev) on sim to another workspace crate carries the native half at once; Wasm half + PM-0002-53 met at the first change-set with both a sim dependent and a Wasm runner (expected 2.3-B); STATUS names it.
  F-9 (Medium): row 94 (plan.md:505) drops 2.3-docs finding 3 (no local smoke: tools/node_modules now present but no Playwright browser cache here). CS-C1 edits smoke.mjs (TOOLKIT-RESOLVES-UP) and the day-seat advisor must run the full gate incl smoke (advisor.md step 2, Never). Fix: carry as a CS-C1 precondition (install the toolkit's Chromium by the allowed cli line, or the plan says how smoke is reproduced locally); not dropped.
  F-10 (Low): row 47 OQ-0002-8-TEST (plan.md:458) has no when; the reading is mine to give: PM-0002-64 at 2.5 exercises both reasons; PM-0002-44 at CS-C enforces rules only.
Next: B9/B10 (guard lines, gate-file lists in advisor.md/CLAUDE.md, settings.json:12, K quotes vs DECISIONS).
- B9 done (K quotes vs DECISIONS.md :245-248 and :470-480: exact; omissions as stated; F1-FIX :426-447 mechanism). Notes:
  MODELS-HOW fairness: his words :250 "Step 2.3 changes CLAUDE.md's Models and effort section and the agent files to match": option (c) moves agent files past 2.3's first part; "why it matters" (plan.md:819-820) omits that sentence.
  Low: his Windows steps (plan.md:747) say "npm ci at the root": write `npm ci --ignore-scripts` (ADR part 9).
  Low: 5.1 scratch copy under .gate/ (plan.md:523): CLAUDE.md at any depth and worktree kit files ask too; name how the copy is made and count its prompts (before his yes, not his yes).
- B10 done (guard :28,:35,:76,:84,:1218,:1474-1510; settings :5-13,:235-241; advisor.md :40-43,:69,:76,:183; Part 4 counts by awk: 107 = 49/47/6/2/2/1 match). Findings:
  F-11 (Medium): K-11 (plan.md:1262-1297) misses advisor.md:40-43, escalation category 3's "today" list of gate files, which Risk tiers (:69,:76) and step 4 (:183) rely on; a PR Advisor reading it would class a tools/gate-v2/ PR tier 1 ready (guard G-2 refuses the merge: backstop). Fix: K-11c updates that list (folder + PIN-SCOPE paths).
  F-12 (Medium): before CS-E, toolchain is not a required check and the guard reads only gate+smoke; CS-C changes root package.json/lock and Cargo.toml that only toolchain.yml tests, yet CS-C's evidence (plan.md:311-315) and CS-B/C1/D omit toolchain. His own #52 go required toolchain runs read by hand. Fix: every CS before CS-E reads toolchain push+PR runs green by hand.
  F-13 (Medium): PM-0002-38 (ADR :1304, "the aggregate job reads the version lines", lint pair a week apart, wasm-bindgen CLI version) is claimed by no report (grep: only the ADR names it) and its method needs 2.3's aggregate job; plan Part 2 starts at -39. Fix: place it (CS-E for version lines + lint pair; 2.3-B for the wasm-bindgen line) or say where it belongs.
  GATE-PATH fairness: (a) puts the gate v2 under tools/, whose node_modules (acorn, eslint-scope, playwright 1.56.1) sits on the gate v2's own import path before the root's (TOOLKIT-RESOLVES-UP's mirror; TWO-PLAYWRIGHTS when G12 joins). Name it in (a)'s cost; CS-C resolves every package from the root by path.
  PIN-SCOPE fairness: (b) leaves the gate v2's own npm tool pins (ESLint, typescript-eslint, Prettier in root package.json/lock) unprotected, unlike tools/package*.json today (advisor.md:40-43 definition "read as settings"); say so, or CS-C keeps those pins inside the gate v2's folder.
  Rulings drafted: (a) move anchor, tightened; (b) agree float waits, hash-map std + clock now, condition 2.3-S before float code in sim; (c) aggregate name: distinctive id preferred over `required` (grep noise in kit prose; "the required check `required`"); (d) agree: switch-exhaustiveness-check is one of part 7's four rules (ADR :423) so it is on from CS-C; its generated-union mutation needs the generator's output shape (boundary spike item 4) -> 2.3-B; interim hand-written-union plant recommended.
Next: advisor tool consult, then B11 write-up.
