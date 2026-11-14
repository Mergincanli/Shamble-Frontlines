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
