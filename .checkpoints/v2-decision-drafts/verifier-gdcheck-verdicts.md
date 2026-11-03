# verifier-gdcheck verdicts

## Round 1

Checked: /home/user/Shamble-Frontlines/.checkpoints/v2-decision-drafts/touchup2-game-designer-fixes.md (5 FIX blocks, 0 NOTE blocks)
against docs/adr/0005-content-abilities-assets.md on disk at HEAD cda0577 (docs identical to 41665c7).

| block | finding | holds? | problem |
|---|---|---|---|
| F1b | fixcheck-d2 F1b (high): AB-3's heading lost between AB-2's Need (0005:900) and AB-3's What (0005:901) | yes | none. BEFORE once (on-disk 900–901, inside 746–1432). The inserted line is byte-identical to 47eadd9:0005:832 (`diff` empty) and sits at applied 903, between AB-2's Need (902) and AB-3's What (904). Headings AB-1..AB-10 now each appear once, in order. |
| LOST-AB1-WHY | own lost line: RR-4 (game-designer-fixes-final.md lines 3–44) took AB-1's Why first half into its BEFORE and left it out of its AFTER, leaving the dangle 0005:883 "    names, with no generic-engine feature." | yes | none. BEFORE once (on-disk 882–883, inside the section). The restored pair (applied 884–885) is byte-identical to 47eadd9:0005:819–820 (`diff` empty); no wording change, so no reason needed. Position: after the "When a pool is full" item that RR-4 inserted between What and Why, so AB-1 reads What, When a pool is full, Why, Research, Need, the order RR-4 intended. The dangle is gone (it is now the Why line's own second half). |
| F10 | fixcheck-d2 F10 (low): AB-5's Need line twice (0005:948 and 0005:949) | yes | none. BEFORE once (on-disk 948–949). Applied AB-5 (934–951) has one Need line, 951 "Need: VISION_V2 §6.5; the faction's list: §6.7.", which contains the deleted line's §6.5 and adds §6.7; nothing the two said is lost. |
| F9 | fixcheck-d2 F9 (low): AB-1's live-projectile formula multiplies by the weapon's pellets, which is 0 for its own example, the Molotov | yes | none. BEFORE once (on-disk 869–876). Sources: LEGACY_TUNING.md line 180 `pellets` molotov 0 and line 175 `fireInterval` molotov 1.05, both inside the cited `WEAPONS` table lines 167–203 (heading 167, last row `special` 203). Legacy throws one Molotov per press (shamble-game.js:1469 `throwMolotov(); ps.fireCd=w.fireInterval`); the pellet loop (:1474) is reached only by hitscan guns. Arithmetic: 1.4 ÷ 1.05 = 1.33, floor 1, + 1 = 2, × 1 = 2; 2 × 48 = 96 (unchanged). "(at least 1)" resolves the finding: with the explicit "`pellets` 0 … × 1" in the example it can only mean "count at least 1", and it also covers the Torque Bow (`pellets` 0). Paragraph 8 → 9 lines; longest line 120 characters (872, 876). |
| F8 | fixcheck-d2 F8 (medium): OQ-0005-AB9 says "ADR 0001 part 12 has E place a cache too" | yes | none. BEFORE once (on-disk 1297–1300, inside OQ-0005-AB9, inside the section). The false clause is gone; the new "picking up and placing a cache (steps 2 and 3) name no key yet" matches ADR 0001:523–524 ("picking up and placing a cache, steps 2 and 3, name no key") and LOOP_V2 §4 lines 42–43 (step 2 "picks it up and carries it", step 3 "place it in an uplink": no key); step 4 (line 44) "holding E" stays cited. "yet" marks it open, which is true (OQ-0005-AB9 is open). The option "the steal or the cache's act wins" now covers pickup and placing as well as the steal. It differs from fixcheck-d2's suggested wording only in leaving "may use the same key" implicit (carried by "yet" and the option); not wrong. Still 4 lines, longest 120 characters (1300). |

### Nothing else lost (step 4)
Method: verifier-gdcheck-scratch/structural.py and seq.py extract, from "## The ability and effect system (game-designer)" to the line before "## The asset pipeline (engine-architect)", every heading, every `- **AB-n.` / `- **OQ-0005-ABn.` start and each item's What, Why, Research, Need, "When a pool is full", Why it matters, Options and Decider line, at 47eadd9 and on the applied copy; mapblocks.py finds the block whose BEFORE holds each 47eadd9 line that is gone.
- On disk (before this round) the comparison shows exactly the three defects the author names: AB-3's heading missing (AB-2 reads What, Why, Research, Need twice), AB-1 without a Why line, AB-5 with Need twice. On the applied copy all three are gone: AB-1 What, When a pool is full (with its own nested Research), Why, Research, Need; AB-2 and AB-3 What, Why, Research, Need each; AB-5 one Need.
- 47eadd9 structural lines absent from the copy, each rewritten on purpose by a block with a stated reason (game-designer-fixes-final.md unless noted): AB-2 Research and Need (GD-7, the immunity rule); AB-5 Research (GD-2's data half); AB-5 "Need: VISION_V2 §6.5." (F10 of this round: folded into the kept Need line); AB-6 Need (GD-7, gains LOOP_V2 §2); OQ-0005-AB7 Decider (GD-PICK-VIS); OQ-0005-AB8 Why it matters (GD-8, full offer queue); OQ-0005-AB9 Decider (GD-8, E for a pickup); OQ-0005-AB10 Decider (GD-8, OQ-0004-10 and -12). None is lost.
- Doubles in one item on the copy: none. (AB-5's second bullet "What may be bought is data, …" begins with the word "What" but is not a "What:" line; OQ-0005-AB3's two "Options for …" lines are two option sets, the same at 47eadd9.) No heading or item start appears twice. Items present at 47eadd9: all 23 present, in order; OQ-0005-AB14 and AB15 are new from GD-7 and GD-PICK-VIS.
- Lost or doubled structural lines: none.

### Rules on the changed lines (step 5)
- Numbers: every number on the changed lines has a source or is arithmetic. F9: fuse 1.4 s (r1-simulation-core.md:1373, "fuse 1.4 s"), fire interval 1.05 s and `pellets` 0 (LEGACY_TUNING.md lines 175 and 180, inside the cited `WEAPONS` table 167–203), N = 48 (the largest mode's N, GLOSSARY "The largest mode, and its N": MODES-3 (a)), "at least 1" (the counting rule itself), 2 and 96 (arithmetic, rechecked). F8: steps 2, 3, 4 are LOOP_V2 §4's numbered steps (lines 42–44). F1b, LOST-AB1-WHY, F10: only section numbers (§6.3, §6.4, §6.5, §6.7), restored or kept as they were.
- GLOSSARY "Retire (exact)": glossary_check_drafts.py, its docs root pointed at a scratch tree (verifier-gdcheck-scratch/gc_base.py on the on-disk drafts, gc_tree.py on the tree with the applied 0005): exact 0 before and 0 after in every draft; 0005 in context 93 before and 93 after. The only in-context hits on changed lines are the two "48" of "2 × 48 = 96 at N = 48" (now both on line 873, before on 872–873): the same words, reflowed, already marked as N. The unsearchable exact pattern "the worst case (48 players plus vehicles and projectiles …)" does not occur on the changed lines (read).
- Sentence directly under a table: 0 in ADR 0005 before and after (a non-table line right after a `|` row; script in the notes).
- Line length: no changed line over 120 characters (longest 120: applied 872, 876, 1300).

### Line count and pointers (step 6)
- ADR 0005: 2499 → 2501 lines (+2). The section (heading 746 to the line before "## The asset pipeline (engine-architect)"): 687 → 689 lines, net +2 (F1b +1, LOST-AB1-WHY +1, F10 −1, F9 +1, F8 0). Lines 1–745 and everything from the asset pipeline heading on are byte-identical to the on-disk file (shifted by 2), so no text outside the section changes.
- README pointers (draft_pointers.py run from verifier-gdcheck-scratch/basetree and /tree): on-disk drafts 94 checked, 0 off; with the applied 0005, 94 checked, 71 ok, **23 off**, every one by exactly +2 (OQ-0005-AB1 1186 → 1188 through OQ-0005-4 2080 → 2082; README lines 40–43 and 96–114), as the author predicted. Its 8 earlier pointers into 0005 stay. No other doc (docs/, other ADRs, ENGINE_OVERVIEW, SPIKE_PLAN) points into ADR 0005 by line number (grep), and ADR 0005 has no line pointers to itself.
- The fix file holds no README block (README lies outside the author's section, so that is right); the 23 re-pins (+2 each) must be applied by README's owner in the same commit as these five blocks.

### Lost or doubled structural lines
None.

### Counts
5 FIX blocks, 0 NOTE blocks; 5 apply (each BEFORE once, dry-run and real run, result byte-identical to the author's touchup2-game-designer-scratch/applied copy); 5 inside the section; 5 hold; 0 lost or doubled structural lines; net +2 lines; pointers off 23 (all +2, none in the fix file).

### Notes for the session
- apply_fixes.py silently skips a block whose FILE does not match the copy's path suffix and still prints "all applied" (scripts/apply_fixes.py lines 13–14 `continue` with no message; read in the code). Apply to the real docs/adr/0005-content-abilities-assets.md path, or check that each block prints an "ok" line.
- The author's summary names HEAD 41665c7; HEAD is now cda0577, but `git diff 41665c7 HEAD -- docs/` touches only the report appendix, so every BEFORE was taken from the same ADR 0005.

### Verdict: PASS WITH NOTES
All five blocks apply, lie in the game-designer's section, resolve their findings and add nothing wrong; nothing structural is lost. The note: the 23 README pointers into ADR 0005 (line 1186 on) go off by +2 and must be re-pinned with this change.
