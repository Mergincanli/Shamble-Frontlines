TASK: advisor re-review, foundation step 2.3 plan revision 2 (plan.md at 00984ae, vs revision 1 at 03d05f5), only what changed; day session.

PLAN (blocks, in order):
C1. Diff stat and read the plan.md diff, part 1 (summary, Parts 0-2).
C2. Read the plan.md diff, part 2 (Parts 3-11, Decisions, For Mustafa).
C3. Read the kit-texts.py / kit-texts.md diff (K-4, K-11c, K-12 labels).
C4. Check each revision-1 finding (H1, M2-M8, the Lows, the four rulings) against the new text; spot-check any new claims.
C5. Write the verdict and per-finding confirmations; DONE.

NOTES:
- C1 done (diff lines 1-170: header, summary, Rule 3, Part 2 table, CS-C1, CS-D, CS-E, CS-F start). Fixed so far: H1 (PROOF-57 row; D2/PM-0002-57 met at PROOF-57; CS-E evidence split), M2 (second ask, 4 writes, failing test + API cancel, 7 asks ~45 prompts), M3 (D1/-52 at slice 5; -47/-51/-53 named; -38 row; summary adds -44/-51/-52), M4 (Rule 3 relabelled + list, G11 named as waiting past result), M7 (toolchain read by hand), M8 (precondition), :84 Low, rulings (a) (b) (d) (c partly) folded. Leads:
  N-1: PM-0002-52 met at "slice 5's change-set" but PM-0002-51's draw-call clause joins "the later of slice 3's and PM-0003-36's" change-set: if PM-0003-36's baseline lands after slice 5, -52 is later. Check ADR 0003 PM-0003-36 (is it the count baseline?).
  N-2: toolchain-by-hand rule (plan rules list) cannot apply to CS-E itself (its PR removes toolchain.yml, so its head has no toolchain run): say CS-E reads its replacement jobs by hand. Low.
  N-3: Rule 3 still justifies by CLAUDE.md's "for later" clause (engine features not traced to VISION_V2 6), a stretch; Low, optional.
Next: C2 (rest of plan diff).
- C2 done (diff lines 170-640: CS-F evidence, PROOF-57, totals 4+2+4+21+7+7=45, Part 4 rows 22/47/94, 5.1 rewritten, K table, Part 8 order text, Part 9, Part 10 risk 4, Part 11 rev-2 list, GATE-PATH (a) cost, PIN-SCOPE (b) text, MODELS-HOW why + recommendation, ROLLBACK-SCOPE, K-4 How/effort, K-4 (a)/(b), K-12 labels, K-11c). All as asked. Leads:
  N-4: K-4 (a) cites "K-5 in this change-set": check K-5 is defined anywhere (else dangling id). Low.
  N-5: ROLLBACK-SCOPE (b) does not say K-12a/f/g are then dropped (lines kept); implied. Skip unless cheap.
  N-6: 5.1 code copies: git archive under .gate/<cs>/ (makes a nested CLAUDE.md: the one counted prompt); guard copy in the scratchpad to avoid W1. No rule forbids; OK.
Next: C3 (kit-texts.py/md diff), then N-1 check (PM-0003-36).
- C3 done (kit-texts.py diff: K-4 How/effort, K-4 (a)/(b), K-12a/f/g widening note, K-11c; plan.md Appendix K identical to kit-texts.md after heading-level normalization, 30 items; quotes still 19). Part 4 recount by awk: 107 = 50/48/5/2/2, matches. Checkpoint at 00984ae says 1 consultation (Low fixed).
- C4 done. Every revision-1 finding fixed (H1, M2-M8, five Lows, four rulings, decision notes). ROLLBACK-SCOPE new: rightly his (A4 rollback; row 56 standard); options fair. New Lows only:
  L-1: plan.md:179 (D1) and :200 (PM-0002-52) say slice 5, but :199 puts PM-0002-51's draw-call mutation at the later of slice 3's and PM-0003-36's change-set (ADR 0003 :1211, the committed baseline file); write "the later of slice 5's and PM-0003-36's change-set".
  L-2: plan.md:227 toolchain-by-hand rule covers CS-E literally, but CS-E's PR removes toolchain.yml, so its head has no toolchain run; say CS-E reads its replacement jobs (rust, typescript, required) by hand.
  L-3: plan.md:563-566 says no step leans on W1, but a whole-tree git archive under .gate/<cs>/ writes nested .claude/ unasked (W1) and a nested CLAUDE.md (the counted prompt); archive only the paths the change-set needs.
  L-4: plan.md:1092 "K-5 in this change-set" names an id defined nowhere; point to 5.3's agent-file lines.
  L-5 (optional): plan.md:116-118 Rule 3 still leans on CLAUDE.md's "for later" clause (engine features untraced to VISION_V2 6); his "may" alone carries it.
  Verdict: APPROVE; Lows non-blocking (fold before the end message without re-review, or carry into CS-E's, CS-B's and CS-C's own plans). Review budget: never REVISE on Lows alone.
Next: C5 write-up.

- C5: final answer (597 words), as handed back:
**Verdict: APPROVE.** Plan revision 2 at 00984ae3a8d8e2fff70d8337d34e8c12f9202da1 fixes every revision-1 finding, and the changes add nothing Critical, High or Medium. The five new findings below are all Low and don't block. Either fold them in before the end message (no new review needed), or carry them into the named change-sets' own plans.

What I verified:
- the diff from 03d05f5 to 00984ae;
- Appendix K is identical to the generated `kit-texts.md` (30 items, 19 quotes);
- Part 4 recounts to 107 rows: 50 / 48 / 5 / 2 / 2;
- the guard-prompt total adds up: 4 + 2 + 4 + 21 + 7 + 7 = 45;
- the checkpoint now shows the right count of advisor-tool consultations.

My notes: /home/user/Shamble-Frontlines/.checkpoints/foundation-2-3-plan/advisor-plan-r2.md.

**Revision-1 findings**
- **H1: fixed.** D2 and PM-0002-57's merge-box clauses now wait for PROOF-57, after his second ruleset step. Part 8 (:774) and Part 10 (:817) now describe the order truthfully.
- **M2: fixed.** CS-E has a second ask (4 writes), a failing test and an API cancel; totals are now 7 asks and about 45 prompts.
- **M3: fixed.** D1 and PM-0002-52 are met at slice 5. PM-0002-47, -51 and -53 name their change-sets, PM-0002-38 has a place, and the summary is complete. One nit remains (L-1).
- **M4: fixed.** Rule 3 is now the plan's own choice, with its list; G11 is named as the check that waits past its result.
- **M5: fixed.** K-4 exempts the netcode chat; K-4 (a) and (b) are written out; [to verify] covers the Fable model value.
- **M6: fixed.** K-11c is added, and K-12a/f/g are labelled a widening. The new decision ROLLBACK-SCOPE is rightly his (his A4 rollback rule; the plan's own row 56 sets the same standard), and its options are fair.
- **M7: fixed,** with one gap (L-2).
- **M8: fixed.** A local smoke run is now CS-C1's precondition.
- **Lows: all five fixed.**
- **Rulings: all four folded where I set them.**
  - (a): row 22, PM-0002-53, and CS-A's rewrite of STATUS :95.
  - (b): 2.3-S merges before f32 or f64 arithmetic enters `sim`.
  - (c): CS-E's naming and CS-F's two-run test row.
  - (d): PM-0002-42's interim plant.
- **Decision notes:** GATE-PATH, PIN-SCOPE and MODELS-HOW are updated as asked.

**New findings (all Low)**
- **L-1.** plan.md:179 and :200 meet D1 and PM-0002-52 at slice 5. But :199 puts PM-0002-51's draw-call mutation at the later of slice 3's change-set and PM-0003-36's (the baseline file of ADR 0003 :1211), so slice 5 may not be last. Fix: "the later of slice 5's change-set and PM-0003-36's".
- **L-2.** plan.md:227's rule ("read `toolchain` by hand") also covers CS-E. But CS-E's PR deletes `toolchain.yml`, so its head has no toolchain run. Fix: CS-E reads its replacement jobs (`rust`, `typescript`, `required`) by hand.
- **L-3.** plan.md:563-566 says no step leans on W1, the guard's gap for nested `.claude/` folders. But a whole-tree `git archive` under `.gate/<cs>/` writes nested `.claude/` files with no ask, plus a nested `CLAUDE.md`. Fix: archive only the paths the change-set needs, without `.claude/` or `CLAUDE.md`. That also removes the counted prompt.
- **L-4.** plan.md:1092 cites "K-5", which is defined nowhere. Fix: point to 5.3's agent-file lines.
- **L-5 (optional).** plan.md:116-118 still justifies Rule 3 with CLAUDE.md's "for later" clause, which is about engine features not traced to VISION_V2 §6. His "may" alone is enough.

**Not checked:**
- PR #40's live state.
- F-HOOK and F-PR (I didn't fetch either page).
- Whether an effort the caller passes overrides an agent file's effort. K-4 (a) relies on that; CLAUDE.md's current practice suggests it does.
DONE

- Advisor tool consulted after the first draft: checked his #48 go (DECISIONS.md :195 asks toolchain, gate and smoke runs green: the plan quotes him right); pinned checks to 00984ae (working tree equals it; 4460ef5..00984ae only .checkpoints/); added L-5 (order wording), L-6 (PM-0002-38 vs 5.6 byte-for-byte), the effort-override [to verify] in L-4; dropped the optional Rule 3 note.
- C5 (redone): final answer v2 (590 words). This exact file is what is handed back, unchanged; it supersedes the 597-word block above:
**Verdict: APPROVE.** I reviewed plan revision 2 at 00984ae3a8d8e2fff70d8337d34e8c12f9202da1. It fixes every revision-1 finding and adds nothing Critical, High or Medium. The new findings below are all Low and none blocks: fold them in before the end message without another review, or carry them into the named change-sets' plans.

**What I checked:**
- The working tree equals 00984ae for `plan.md` and `kit-texts.*`. Since 4460ef5 only `.checkpoints/` has changed.
- Appendix K is identical to `kit-texts.md`: 30 items, 19 quotes.
- Part 4 recounts as 107 = 50/48/5/2/2.
- The prompt total holds: 4+2+4+21+7+7 = 45.
- The checkpoint's consultation count is right.
- His gos for #48 and #52 both asked for the `toolchain` runs (DECISIONS.md :195; merge 4460ef5), so the reason given at :227-230 is his own words.

Notes: /home/user/Shamble-Frontlines/.checkpoints/foundation-2-3-plan/advisor-plan-r2.md.

**Revision-1 findings**
- **H1: fixed.** PROOF-57, after his second ruleset step, holds D2 and PM-0002-57's merge-box clauses. The order sentences at :774 and :817 are now safe, with one wording nit (L-5).
- **M2: fixed.** CS-E has a second ask (4 writes), uses a failing test and an API cancel, and the totals are 7 asks and about 45 prompts.
- **M3: fixed.** D1 and PM-0002-52 sit at slice 5. PM-0002-47, -51 and -53 name their change-sets, PM-0002-38 is placed, and the summary is complete. L-1 is a nit.
- **M4: fixed.** Rule 3 is labelled the plan's choice and lists its checks; G11 is named.
- **M5: fixed.** K-4 exempts the netcode chat, K-4 (a) and (b) are added, and [to verify] covers the Fable value.
- **M6: fixed.** K-11c is added and K-12a/f/g are labelled a widening. ROLLBACK-SCOPE is rightly his (his A4 rollback rule; row 56), and its options are fair.
- **M7: fixed** (L-2 is a gap).
- **M8: fixed.** A local smoke run is now CS-C1's precondition.
- **Lows:** all five fixed.
- **Rulings:**
  - (a): row 22, PM-0002-53, and CS-A's rewrite of STATUS :95;
  - (b): 2.3-S's condition;
  - (c): CS-E's naming and CS-F's two-run test row;
  - (d): PM-0002-42's interim plant.
- **Decision notes:** GATE-PATH, PIN-SCOPE and MODELS-HOW are updated as asked.

**New findings (all Low)**
- **L-1.** :179 and :200 meet D1 and PM-0002-52 at slice 5, but :199 joins PM-0002-51's draw-call mutation at the later of slice 3's change-set and PM-0003-36's (ADR 0003 :1211's baseline file). Fix: "the later of slice 5's change-set and PM-0003-36's".
- **L-2.** The rule at :227 covers CS-E, but CS-E's PR deletes `toolchain.yml`, so its head has no toolchain run. Fix: CS-E reads its replacement jobs (`rust`, `typescript`, `required`) by hand.
- **L-3.** :563-566 says no step leans on W1, but a whole-tree `git archive` under `.gate/<cs>/` writes nested `.claude/` files with no ask, plus a nested CLAUDE.md. Fix: archive only the paths the change-set needs, which also drops that prompt.
- **L-4.** :1092 cites "K-5", which is defined nowhere: point to 5.3's agent-file lines. K-4 (a) also assumes a caller's effort overrides an agent file's; add that to its [to verify].
- **L-5.** :774 and :817 say `gate` and `smoke` stay required by both walls throughout, with the ruleset at least as strict as the guard. After step 2, the ruleset holds them only through `required`. Fix: add "directly, or through `required`".
- **L-6.** PM-0002-38's "each job writes its version lines" (:372) conflicts with "the legacy jobs are moved byte for byte" (:629). Fix: say "each v2 job", or name the legacy jobs' version line as a 5.6 exception.

**Not checked:**
- PR #40's live state;
- F-HOOK and F-PR (I fetched neither page).
DONE
