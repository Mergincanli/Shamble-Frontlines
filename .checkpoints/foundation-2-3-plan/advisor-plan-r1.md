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
- Advisor tool consulted before write-up. Closed its three checks: (1) PM-0002-38: old plan 26e0d82 plan.md:383 gave it to 2.2 with -37 ("Done when (PM-0002-37, -38, and part of -39)"); 2.2 report never names it -> orphan, frame "place it". (2) CS-E plants: skipped job and removed check are necessarily .github/ edits -> his 2026-11-13 rule covers each plant and revert; counts short (F-14, Medium). (3) K-10b verified: L4 is in archive "Open bugs and risks" (:988), kit-v3-3 entry (:1033), limit list :1069. Sharpened: F-6 one fix (scratch PRs after his ruleset step 2; D4 and skipped/removed clauses can stay at CS-E); F-1 restate, do not reorder (plan.md:236-237 rules-true principle); F-4 via plan row 56; F-9 as check-8 item (blocks CS-C1's review, not this plan); Rule 3 reported under check 3 (Medium); triggers without change-sets at plan.md:180, :184 folded with F-8; ruling (c) firm: accept `required` with conditions; ruling (a) home: CS-A rewrites STATUS :95 and the archive anchor.
- B11: final answer written below.

FINAL ANSWER (verbatim for hand-back):
**Verdict: REVISE** — plan revision 1 at 03d05f55a2e49cac3be98ddffdc228632a9ef0c7. The shape is right: items 1-9 and his three additions are all answered and the ending is as he asked (path and chat fullness in the end message). The cut and order are safe at every merge. F-FLOAT holds. K-4's two quotes match DECISIONS.md :245-248 and :470-480 exactly; K-8 and K-10b check out; Part 4's counts reproduce (107 = 49/47/6/2/2/1). But several "met" claims fail a strict reading, and his go is given on this text. All fixes are text; re-review covers only what changes. Notes: /home/user/Shamble-Frontlines/.checkpoints/foundation-2-3-plan/advisor-plan-r1.md.

**Findings**
1. **High.** PM-0002-57 and D2 are marked "met at CS-E" (plan.md:166, :190). ADR :1323 needs "cannot merge", read from the merge box on scratch-branch test PRs. D2 needs "the single required check" (ADR :1463-1470). But `required` enters the ruleset only after CS-E (:354, :725) and is single only after CS-F (:728). Fix: prove the merge-box clauses and D2 on scratch PRs after his ruleset step 2, and mark them met there; D4 and the skipped and removed-check clauses can stay at CS-E. Also restate :730 and :765-766: between ruleset step 1 and CS-F the ruleset requires more than the guard. That window is safe, but the sentence is false. Do not reorder: rules text must stay true at each merge (:236-237).
2. **Medium.** CS-E's skipped-job and removed-check plants are `.github/` edits, so each plant and each revert falls under his 2026-11-13 rule (exact text, his yes, Accept edits). CS-E's "one ask, 3 prompts" (:352) and the totals (:68-70, :376, :739) leave them out, as they would the scratch-branch plants. List and count them. Where the mark allows, plant the failing job as a failing test and cancel a run through the API.
3. **Medium: Part 2 cells.**
   - PM-0002-52 needs "every mutation of PM-0002-39 to -51" (ADR :1322), yet it is met at 2.3-E (:185, D1 :165), while the plan's own rows put some of those mutations at slices 1, 3 and 5.
   - Three cells name no change-set: PM-0002-53's "2.3-B at the earliest" (:186), PM-0002-47's "first such test" (:180) and PM-0002-51's baseline (:184).
   - The one-screen summary (:56-57) omits -44, -51 and -52.
   - PM-0002-38 (ADR :1304) is claimed by nobody. The old plan (26e0d82 :383) gave it to 2.2; the 2.2 report never names it; its method needs the aggregate job. Place it: version lines and the lint pair at CS-E, the wasm-bindgen line at 2.3-B.
4. **Medium.** Part 0's Rule 3 (:111-112) presents a wait for later code as a reading of his "needs a result". G11 (PM-0002-49/-50) then waits past its result, until 2.5's code. Say these wait by the plan's choice under his "may", and list them, so he can ask for any of them sooner.
5. **Medium.** K-4 (:950-956) tells "the caller" to pass `max` and makes no exception. Once the netcode chat takes main (:719), it would raise its efforts, against his quoted "keeps its models and efforts as they are" (:943). Add "except the netcode chat", and give K-4's text under MODELS-HOW (a) and (b). Extend [to verify] to `model: "fable"` (:946), and add "on his personal account" there.
6. **Medium.** K-11 misses advisor.md:40-43, escalation category 3's list of gate files, which :69, :76 and :183 rely on: add K-11c. K-12a/f/g widen "main breaks" to every v2 job. The plan's own row 56 says changes to those cases need his words, so label the change rather than calling it "same strength".
7. **Medium.** Until CS-E, only `gate` and `smoke` are required, yet CS-C changes root `package.json`, the lockfile and `Cargo.toml`, which only `toolchain` tests. Every change-set before CS-E should read `toolchain`'s push and pull_request runs by hand, as his #52 go did.
8. **Medium (check 8).** Row 94 (:505) drops finding 3, but CS-C1 edits `smoke.mjs`. My file's Never line ("Approve without running the gate yourself") and step 2's day-session seat both need a local smoke run. That blocks CS-C1's review, not this plan: make it a precondition of CS-C1.

Lows:
- OQ-0002-8-TEST (:458) has no time. My reading: PM-0002-64 is its test.
- Line 276 cites routine-guard.mjs:87; MERGE_PLACES is at :84.
- His Windows steps (:747) should say `npm ci --ignore-scripts`.
- 5.1 (:523) should say how the scratch copy is made and count its prompts.
- The checkpoint says 0 consultations but lists 1.

Not checked: PR #40's live state; F-HOOK and F-PR (I fetched neither page); K-6's claim about cloud modes; how the ruleset binds a check to its source.

**Rulings**
- **(a)** Move OVERFLOW-XCRATE-CI's anchor, as its own text allows (archive :656-658). Nothing depends on `sim` yet, and a test-only dependency now would add scaffolding to `sim` and still miss the Wasm half. The first change-set that adds any dependency on `sim` carries the native half. PM-0002-53 is met at the first change-set that has both a crate depending on `sim` and a Wasm runner, expected 2.3-B. CS-A rewrites STATUS :95 and the archive's anchor.
- **(b)** The classification is right. `sqrt` is on ADR 0004's allow-list (:262-263), and R1-DET-OQ10 belongs to the sim core spike (:1347). The std hash-map and clock bans rest on no open question (ADR 0002 :436-458). Condition: 2.3-S merges before any change that adds f32 or f64 arithmetic to `sim` (there is none today).
- **(c)** `required` is accepted, with three conditions: kit texts always write it in code font; the job id and its name are both exactly `required`; G-4 gets a test row where two runs carry that name, one red, and the merge is refused.
- **(d)** Agreed. `switch-exhaustiveness-check` is one of part 7's four rules (ADR :423), so it is on from CS-C. Its generated-union mutation needs the generator's output (the boundary spike's item 4), so it joins at 2.3-B. I recommend an interim plant on a hand-written union, not counted toward the mark.

**The five decisions:** each is rightly his.
- **GATE-PATH:** (a)'s cost should say that the gate v2's own imports resolve through `tools/node_modules` (acorn, eslint-scope, Playwright 1.56.1) before the root's.
- **PIN-SCOPE:** say what (b) leaves unprotected: the gate v2's npm tool pins in the root `package.json` and lockfile, and `Cargo.lock`, while `tools/package*.json` are gate files today.
- **FREEZE-END:** fair. I would pick (a) too.
- **MODELS-HOW:** quote his "Step 2.3 changes … and the agent files to match" (DECISIONS.md :91), since (c) moves the agent files to after #40.
- **ALLOW-GATE2:** fair, and his by his words of 2026-11-11.
DONE

FINAL ANSWER v2 (trimmed to fit the 900-word limit; supersedes the block above; same content):
**Verdict: REVISE** — plan revision 1 at 03d05f55a2e49cac3be98ddffdc228632a9ef0c7. The shape is right: items 1-9 and his three additions are answered, and the ending is as he asked. The cut and order are safe at each merge. F-FLOAT holds. K-4's quotes match DECISIONS.md :245-248 and :470-480, and Part 4's counts reproduce (107 = 49/47/6/2/2/1). But several "met" claims fail a strict reading, and his go rests on this text. All fixes are text; re-review covers only what changes. Notes: /home/user/Shamble-Frontlines/.checkpoints/foundation-2-3-plan/advisor-plan-r1.md.

**Findings**
1. **High.** PM-0002-57 and D2 are marked "met at CS-E" (plan.md:166, :190). ADR :1323 needs "cannot merge", read from the merge box; D2 needs "the single required check". `required` enters the ruleset only after CS-E (:725) and is single only after CS-F (:728). Fix: prove those clauses on scratch PRs after his ruleset step 2, and mark them met there. D4 and the skipped and removed-check clauses stay at CS-E. Also restate :730 and :765-766: between step 1 and CS-F the ruleset requires more than the guard (safe, but the sentence is false). Do not reorder (:236-237).
2. **Medium.** CS-E's skipped-job and removed-check plants, and their reverts, are `.github/` edits under his 2026-11-13 rule. CS-E's "one ask, 3 prompts" (:352) and the totals (:68-70, :376, :739) leave them out, as they would the scratch-branch plants. Count them. Plant the failing job as a failing test, and cancel a run through the API.
3. **Medium: Part 2 cells.**
   - PM-0002-52 needs "every mutation of PM-0002-39 to -51" (ADR :1322), yet it is met at 2.3-E (:185; D1 :165), while its own rows put mutations at slices 1, 3 and 5.
   - No change-set is named at :186 (PM-0002-53, "at the earliest"), :180 or :184.
   - The summary (:56-57) omits -44, -51 and -52.
   - PM-0002-38 (ADR :1304) is claimed by nobody. The old plan (26e0d82 :383) gave it to 2.2, whose report never names it, and its method needs the aggregate job. Place it at CS-E, with the wasm-bindgen line at 2.3-B.
4. **Medium.** Rule 3 (:111-112) presents a wait for later code as a reading of his "needs a result", and G11 (PM-0002-49/-50) then waits past its result. Say these wait by the plan's choice under his "may", and list them.
5. **Medium.** K-4 (:950-956) tells "the caller" to pass `max` with no exception. Once the netcode chat takes main (:719), it would raise its efforts, against his quoted words (:943). Add "except the netcode chat", and give K-4's text under MODELS-HOW (a) and (b). Extend [to verify] to `model: "fable"` (:946), and add "on his personal account" there.
6. **Medium.** K-11 misses advisor.md:40-43, escalation category 3's list of gate files, which :69, :76 and :183 rely on: add K-11c. K-12a/f/g widen "main breaks" to every v2 job. Your row 56 says such changes need his words, so label the change instead of calling it "same strength".
7. **Medium.** Until CS-E, only `gate` and `smoke` are required, yet CS-C changes root `package.json`, the lockfile and `Cargo.toml`, which only `toolchain` tests. Each change-set before CS-E should read `toolchain`'s push and pull_request runs by hand, as his #52 go did.
8. **Medium (check 8).** Row 94 (:505) drops finding 3, but CS-C1 edits `smoke.mjs`. My file's Never line ("Approve without running the gate yourself") and step 2's day-session seat need a local smoke run. That blocks CS-C1's review, not this plan: make it a precondition of CS-C1.

Lows:
- OQ-0002-8-TEST (:458) has no time. My reading: PM-0002-64 is its test.
- Line 276 cites routine-guard.mjs:87; MERGE_PLACES is at :84.
- His Windows steps (:747) should say `npm ci --ignore-scripts`.
- 5.1 (:523) should say how the scratch copy is made, and count its prompts.
- The checkpoint says 0 consultations but lists 1.

Not checked: PR #40's live state; F-HOOK and F-PR; K-6's claim about cloud modes; how the ruleset binds a check to its source.

**Rulings**
- **(a)** Move OVERFLOW-XCRATE-CI's anchor, as its own text allows (archive :656-658). Nothing depends on `sim` yet, and a test-only dependency would add scaffolding to `sim` and still miss the Wasm half. The first change-set that adds any dependency on `sim` carries the native half. PM-0002-53 is met where a crate depending on `sim` and a Wasm runner first exist together (expected 2.3-B). CS-A rewrites STATUS :95 and the archive's anchor.
- **(b)** Right. `sqrt` is on ADR 0004's allow-list (:262-263), and R1-DET-OQ10 belongs to the sim core spike (:1347). The std hash-map and clock bans rest on no open question (ADR 0002 :436-458). Condition: 2.3-S merges before any f32 or f64 arithmetic enters `sim` (there is none today).
- **(c)** `required` is accepted, with three conditions: kit texts always write it in code font; the job id and its name are both exactly `required`; G-4 gets a test row where two runs carry that name, one red, and the merge is refused.
- **(d)** Agreed. `switch-exhaustiveness-check` is one of part 7's four rules (ADR :423), so it is on from CS-C. Its generated-union mutation needs the generator's output (the boundary spike's item 4), so it joins at 2.3-B. I recommend an interim plant on a hand-written union, not counted toward the mark.

**The five decisions:** each is rightly his.
- **GATE-PATH:** (a)'s cost should say that the gate v2's own imports resolve through `tools/node_modules` (acorn, eslint-scope, Playwright 1.56.1) before the root's.
- **PIN-SCOPE:** say what (b) leaves unprotected: the gate v2's npm tool pins (the root `package.json` and lockfile) and `Cargo.lock`, while `tools/package*.json` are gate files today.
- **FREEZE-END:** fair. I would pick (a) too.
- **MODELS-HOW:** quote his "Step 2.3 changes … and the agent files to match" (DECISIONS.md :91), since (c) moves the agent files to after #40.
- **ALLOW-GATE2:** fair, and his by his words of 2026-11-11.
DONE

FINAL ANSWER v3 (the one handed back; v2 cut to under 900 words, same content):
**Verdict: REVISE** — plan revision 1 at 03d05f55a2e49cac3be98ddffdc228632a9ef0c7. The shape is sound:
- items 1-9 and his three additions are answered, and the ending is as he asked;
- the cut and order are safe at each merge;
- F-FLOAT holds;
- K-4's quotes are exact (DECISIONS.md :245-248, :470-480);
- Part 4's counts reproduce.

But several "met" claims fail a strict reading, and his go rests on this text. The fixes are text only; re-review covers what changes. Notes: /home/user/Shamble-Frontlines/.checkpoints/foundation-2-3-plan/advisor-plan-r1.md.

**Findings**
1. **High.** PM-0002-57 and D2 are marked "met at CS-E" (plan.md:166, :190).
   - ADR :1323 needs "cannot merge", read from the merge box; D2 needs "the single required check".
   - But `required` enters the ruleset only after CS-E (:725), and is the single required check only after CS-F (:728).
   - Fix: prove those clauses on scratch PRs after his ruleset step 2, and mark them met there. D4 and the skipped and removed-check clauses stay at CS-E.
   - Also restate :730 and :765-766: between step 1 and CS-F the ruleset requires more than the guard. That is safe, but the sentence is false. Do not reorder (:236-237).
2. **Medium.** CS-E's skipped-job and removed-check plants, and their reverts, are `.github/` edits, so his 2026-11-13 rule covers each one. CS-E's "one ask, 3 prompts" (:352) and the totals (:68-70, :376, :739) leave them out, as they would the scratch-branch plants. Count them, and use a failing test and a cancel through the API where the mark allows.
3. **Medium. Part 2's cells:**
   - PM-0002-52 needs "every mutation of PM-0002-39 to -51" (ADR :1322). The plan marks it met at 2.3-E (:185; D1 :165), but its own rows put mutations at slices 1, 3 and 5.
   - :186 ("at the earliest"), :180 and :184 name no change-set.
   - The summary (:56-57) omits -44, -51 and -52.
   - Nobody claims PM-0002-38 (ADR :1304). The old plan (26e0d82 :383) gave it to 2.2, whose report never names it, and its method needs the aggregate job. Place it at CS-E, with the wasm-bindgen line at 2.3-B.
4. **Medium.** Rule 3 (:111-112) presents a wait for later code as a reading of his "needs a result". G11 (PM-0002-49/-50) then waits past its result. Say these checks wait by the plan's choice under his "may", and list them.
5. **Medium.** K-4 (:950-956) tells "the caller" to pass `max`, with no exception. Once the netcode chat takes main (:719), it would raise its efforts, against his words quoted at :943. Add "except the netcode chat", and give K-4's text under MODELS-HOW (a) and (b). Extend [to verify] to `model: "fable"` (:946), with "on his personal account".
6. **Medium.**
   - K-11 misses advisor.md:40-43, escalation category 3's list of gate files, which :69, :76 and :183 rely on. Add K-11c.
   - K-12a/f/g widen "main breaks" to every v2 job. Row 56 says changes to those cases need his words, so label the widening rather than calling it "same strength".
7. **Medium.** Until CS-E, only `gate` and `smoke` are required, yet CS-C changes root `package.json`, the lockfile and `Cargo.toml`, which only `toolchain` tests. Before CS-E, each change-set reads `toolchain`'s push and pull_request runs by hand, as his #52 go did.
8. **Medium (check 8).** Row 94 (:505) drops finding 3, but CS-C1 edits `smoke.mjs`. My file's Never line ("Approve without running the gate yourself") and step 2's day seat need a local smoke run. This blocks CS-C1's review, not this plan: make it CS-C1's precondition.

**Lows**
- OQ-0002-8-TEST (:458) has no time. My reading: PM-0002-64 is its test.
- :276 cites routine-guard.mjs:87, but MERGE_PLACES is at :84.
- :747 should read `npm ci --ignore-scripts`.
- 5.1 (:523) should say how the scratch copy is made, and count its prompts.
- The checkpoint says 0 consultations but lists 1.

**Not checked:** PR #40's live state; F-HOOK and F-PR; K-6's claim about cloud modes; the ruleset's source binding.

**Rulings**
- **(a)** Move the anchor, as its own text allows (archive :656-658). Nothing depends on `sim` yet, and a test-only dependency would add scaffolding but still miss the Wasm half.
  - The first change-set that adds any dependency on `sim` carries the native half.
  - PM-0002-53 is met where a crate that depends on `sim` and a Wasm runner first coexist (expected 2.3-B).
  - CS-A rewrites STATUS :95 and the archive's anchor.
- **(b)** Right. `sqrt` is on ADR 0004's allow-list (:262-263), and R1-DET-OQ10 belongs to the sim core spike (:1347). The std hash-map and clock bans rest on no open question (ADR 0002 :436-458). Condition: 2.3-S merges before any f32 or f64 arithmetic enters `sim` (none today).
- **(c)** `required` accepted, with conditions:
  - it is always written in code font;
  - the job id and the job name are exactly `required`;
  - a new G-4 row: two runs with that name, one of them red, is refused.
- **(d)** Agreed. `switch-exhaustiveness-check` is one of part 7's four rules (ADR :423), so it is on from CS-C. Its generated-union mutation needs the generator's output (the boundary spike's item 4), so it joins at 2.3-B. I recommend an interim hand-written-union plant that does not count toward the mark.

**The five decisions** (each rightly his)
- **GATE-PATH:** (a)'s cost should note that the gate v2's own imports resolve through `tools/node_modules` (acorn, eslint-scope, Playwright 1.56.1) before the root's.
- **PIN-SCOPE:** say what (b) leaves unprotected: the gate v2's npm tool pins (the root `package.json` and its lockfile) and `Cargo.lock`. `tools/package*.json` are gate files today.
- **FREEZE-END:** fair; I'd pick (a).
- **MODELS-HOW:** quote his "Step 2.3 changes … and the agent files to match" (DECISIONS.md :91), since (c) moves the agent files to after #40.
- **ALLOW-GATE2:** fair, and his by his words of 2026-11-11.
DONE

FINAL ANSWER v4 (the one handed back; v3 was still 947 words; same content):
**Verdict: REVISE** — plan revision 1 at 03d05f55a2e49cac3be98ddffdc228632a9ef0c7. The shape is sound:
- items 1-9 and his three additions are answered, and the ending is as he asked;
- the cut and order are safe at each merge;
- F-FLOAT holds;
- K-4's quotes are exact (DECISIONS.md :245-248, :470-480);
- Part 4's counts reproduce.

But several "met" claims fail a strict reading, and his go rests on this text. The fixes are text only; re-review covers the changes. Notes: /home/user/Shamble-Frontlines/.checkpoints/foundation-2-3-plan/advisor-plan-r1.md.

**Findings**
1. **High.** PM-0002-57 and D2 are marked "met at CS-E" (plan.md:166, :190).
   - ADR :1323 needs "cannot merge", read from the merge box. D2 needs "the single required check".
   - But `required` enters the ruleset only after CS-E (:725), and is the single required check only after CS-F (:728).
   - Fix: prove those clauses on scratch PRs after his ruleset step 2, and mark them met there. D4 and the skipped-job and removed-check clauses stay at CS-E.
   - Restate :730 and :765-766: between ruleset step 1 and CS-F the ruleset requires more than the guard. That is safe, but the sentence is false. Do not reorder (:236-237).
2. **Medium.** CS-E's skipped-job and removed-check plants, and their reverts, are `.github/` edits under his 2026-11-13 rule. "One ask, 3 prompts" (:352) and the totals (:68-70, :376, :739) leave them out, and the scratch-branch plants too. Count them. Where the mark allows, plant a failing test and cancel through the API.
3. **Medium: Part 2's cells.**
   - PM-0002-52 needs "every mutation of PM-0002-39 to -51" (ADR :1322), yet the plan marks it met at 2.3-E (:185; D1 :165) while its own rows put mutations at slices 1, 3 and 5.
   - :186 ("at the earliest"), :180 and :184 name no change-set.
   - The summary (:56-57) leaves out -44, -51 and -52.
   - Nobody claims PM-0002-38 (ADR :1304). The old plan (26e0d82 :383) gave it to 2.2, whose report never names it, and its method needs the aggregate job. Place it at CS-E, with the wasm-bindgen line at 2.3-B.
4. **Medium.** Rule 3 (:111-112) presents waiting for later code as a reading of his "needs a result". G11 (PM-0002-49/-50) then waits past its result. Say these checks wait by the plan's choice under his "may", and list them.
5. **Medium.** K-4 (:950-956) tells "the caller" to pass `max`, with no exception. Once the netcode chat takes main (:719), it would raise its efforts, against his words quoted at :943.
   - Add "except the netcode chat".
   - Give K-4's text under MODELS-HOW (a) and (b).
   - Extend [to verify] to `model: "fable"` (:946), adding "on his personal account".
6. **Medium.**
   - K-11 misses advisor.md:40-43, escalation category 3's list of gate files, which :69, :76 and :183 rely on. Add K-11c.
   - K-12a/f/g widen "main breaks" to every v2 job. Row 56 says such changes need his words, so label it as a change, not "same strength".
7. **Medium.** Until CS-E, only `gate` and `smoke` are required. Yet CS-C changes the root `package.json`, the lockfile and `Cargo.toml`, which only `toolchain` tests. Each change-set before CS-E should read `toolchain`'s push and pull_request runs by hand, as his #52 go did.
8. **Medium (check 8).** Row 94 (:505) drops finding 3, but CS-C1 edits `smoke.mjs`. My file's Never ("Approve without running the gate yourself") and step 2's day seat need a local smoke run. This blocks CS-C1's review, not this plan: make it a precondition for CS-C1.

**Lows**
- OQ-0002-8-TEST (:458) has no time. My reading: PM-0002-64 is its test.
- :276 cites routine-guard.mjs:87; MERGE_PLACES is at :84.
- :747 should read `npm ci --ignore-scripts`.
- 5.1 (:523) should say how the scratch copy is made, and count its prompts.
- The checkpoint says 0 consultations but lists 1.

**Not checked:** PR #40's live state; F-HOOK and F-PR; K-6's claim about cloud modes; how the ruleset binds the check to its source.

**Rulings**
- **(a)** Move the anchor, as its own text allows (archive :656-658). Nothing depends on `sim` yet, and a test-only dependency would add scaffolding but still miss the Wasm half.
  - The first change-set that adds any dependency on `sim` carries the native half.
  - PM-0002-53 is met where a crate depending on `sim` and a Wasm runner first exist together (expected 2.3-B).
  - CS-A rewrites STATUS :95 and the archive's anchor.
- **(b)** Right. `sqrt` is on ADR 0004's allow-list (:262-263), and R1-DET-OQ10 belongs to the sim core spike (:1347). The std hash-map and clock bans rest on no open question (ADR 0002 :436-458). Condition: 2.3-S merges before any f32 or f64 arithmetic enters `sim` (there is none today).
- **(c)** `required` is accepted, on three conditions: kit texts always write it in code font; the job id and its name are exactly `required`; G-4 gets a test row where two runs with that name, one of them red, are refused.
- **(d)** Agreed. `switch-exhaustiveness-check` is one of part 7's four rules (ADR :423), so it is on from CS-C. Its generated-union mutation needs the generator's output (the boundary spike's item 4), so it joins at 2.3-B. Recommended: an interim plant on a hand-written union that does not count toward the mark.

**The five decisions** (each rightly his)
- **GATE-PATH:** (a)'s cost should note that the gate v2's own imports resolve through `tools/node_modules` (acorn, eslint-scope, Playwright 1.56.1) before the root's.
- **PIN-SCOPE:** say what (b) leaves unprotected: the gate v2's npm tool pins (the root `package.json` and lockfile) and `Cargo.lock`. `tools/package*.json` are gate files today.
- **FREEZE-END:** fair; I'd pick (a).
- **MODELS-HOW:** quote his "Step 2.3 changes … and the agent files to match" (DECISIONS.md :91), since (c) moves the agent files to after #40.
- **ALLOW-GATE2:** fair, and his by his words of 2026-11-11.
DONE

FINAL ANSWER v6 (the one handed back: v4 trimmed to 890 words; same content, same findings and rulings):
**Verdict: REVISE** — plan revision 1 at 03d05f55a2e49cac3be98ddffdc228632a9ef0c7. These check out: coverage of items 1-9 and his three additions, the ending, the order of merges, F-FLOAT, K-4's quotes (DECISIONS.md :245-248, :470-480) and Part 4's counts. But several "met" claims fail a strict reading, and his go rests on this text. Fixes are text; re-review covers the changes. Notes: /home/user/Shamble-Frontlines/.checkpoints/foundation-2-3-plan/advisor-plan-r1.md.

**Findings**
1. **High.** PM-0002-57 and D2 are marked "met at CS-E" (plan.md:166, :190).
   - ADR :1323 needs "cannot merge", read from the merge box, and D2 needs "the single required check". But `required` enters the ruleset only after CS-E (:725), and is the single check only after CS-F (:728).
   - Fix: prove those clauses on scratch PRs after his ruleset step 2, and mark them met there. D4 and the skipped- and removed-check clauses stay at CS-E.
   - Restate :730 and :765-766. Between step 1 and CS-F the ruleset requires more than the guard: that is safe, but the sentence is false. Don't reorder (:236-237).
2. **Medium.** CS-E's skipped-job and removed-check plants, and their reverts, are `.github/` edits under his 2026-11-13 rule.
   - "One ask, 3 prompts" (:352) and the totals (:68-70, :376, :739) leave them out, and the scratch-branch plants too. Count them.
   - Prefer a failing test and an API cancel where possible.
3. **Medium.** Part 2:
   - PM-0002-52 needs "every mutation of PM-0002-39 to -51" (ADR :1322). It is marked met at 2.3-E (:185; D1 :165), but the plan's rows put some of those mutations at slices 1, 3 and 5.
   - :186 ("at the earliest"), :180 and :184 name no change-set.
   - The summary (:56-57) leaves out -44, -51 and -52.
   - Nobody claims PM-0002-38 (ADR :1304). The old plan (26e0d82 :383) gave it to 2.2, whose report omits it; its method needs the aggregate job. Place it at CS-E, with the wasm-bindgen line at 2.3-B.
4. **Medium.** Rule 3 (:111-112) presents waiting for later code as a reading of his "needs a result", and G11 (PM-0002-49/-50) then waits past its result. Say these wait by the plan's choice under his "may", and list them.
5. **Medium.** K-4 (:950-956) tells "the caller" to pass `max`, with no exception. Once the netcode chat takes main (:719), it would raise its efforts, against his words quoted at :943.
   - Add "except the netcode chat".
   - Give K-4's text under MODELS-HOW (a) and (b).
   - Extend [to verify] to `model: "fable"` (:946), with "on his personal account".
6. **Medium.**
   - K-11 misses advisor.md:40-43, escalation category 3's list of gate files, which :69, :76 and :183 rely on. Add K-11c.
   - K-12a/f/g widen "main breaks" to every v2 job. Row 56 says such changes need his words, so label it, not "same strength".
7. **Medium.** Before CS-E only `gate` and `smoke` are required, yet CS-C changes root `package.json`, the lockfile and `Cargo.toml`, which only `toolchain` tests. Until then each change-set reads `toolchain`'s runs by hand, as his #52 go did.
8. **Medium (check 8).** Row 94 (:505) drops finding 3, but CS-C1 edits `smoke.mjs`. My file's Never ("Approve without running the gate yourself") and step 2's day seat need a local smoke run. That blocks CS-C1's review, not this plan: a CS-C1 precondition.

**Lows**
- OQ-0002-8-TEST (:458) has no time; my reading: PM-0002-64 is its test.
- :276 cites routine-guard.mjs:87; MERGE_PLACES is at :84.
- :747: `npm ci --ignore-scripts`.
- 5.1 (:523) should say how the scratch copy is made and count its prompts.
- The checkpoint says 0 consultations but lists 1.

**Not checked:** PR #40's live state; F-HOOK and F-PR; K-6's cloud-mode claim; the ruleset's source binding.

**Rulings**
- **(a)** Move the anchor, as its text allows (archive :656-658). Nothing depends on `sim`; a test-only dependency adds scaffolding yet misses the Wasm half.
  - The first change-set adding any dependency on `sim` carries the native half.
  - PM-0002-53 is met where a crate depending on `sim` and a Wasm runner first coexist (expected 2.3-B).
  - CS-A rewrites STATUS :95 and the archive's anchor.
- **(b)** Right.
  - `sqrt` is on ADR 0004's allow-list (:262-263), and R1-DET-OQ10 is the sim core spike's question (:1347).
  - The std hash-map and clock bans rest on no open question (ADR 0002 :436-458).
  - Condition: 2.3-S merges before f32 or f64 arithmetic enters `sim` (there is none today).
- **(c)** `required` is accepted, on three conditions:
  - it is always written in code font;
  - the job id and its name are exactly `required`;
  - a G-4 test row: two runs with that name, one red, are refused.
- **(d)** Agreed.
  - `switch-exhaustiveness-check` is one of part 7's four rules (ADR :423), so it is on from CS-C.
  - Its generated-union mutation needs the generator's output (the boundary spike's item 4), so it joins at 2.3-B.
  - Recommended: an uncounted interim plant on a hand-written union.

**The five decisions** (each rightly his)
- **GATE-PATH:** (a)'s cost should note the gate v2's imports resolve through `tools/node_modules` (acorn, eslint-scope, Playwright 1.56.1) before the root's.
- **PIN-SCOPE:** say what (b) leaves unprotected: the gate v2's npm tool pins (root `package.json`, lockfile) and `Cargo.lock`, while `tools/package*.json` are gate files today.
- **FREEZE-END:** fair. I'd pick (a).
- **MODELS-HOW:** quote his "Step 2.3 changes … and the agent files to match" (DECISIONS.md :91). (c) moves the agent files to after #40.
- **ALLOW-GATE2:** fair; his by his 2026-11-11 words.
DONE
