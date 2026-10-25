# R7 legacy harvest: editor of area PG (task: apply skeptic corrections, missing systems, coverage suggestions for PG; write final-PG.json)
Inputs: rows-and-checks.json PG (20 rows, 20 checks, 3 missing), coverage.json suggested_rows (none name PG; GP rows are another area).
- PG1, PG2, PG5, PG8, PG9, PG11, PG12, PG13, PG14, PG15, PG16, PG17, PG18, PG19: skeptic corrections applied as written; spot-checked index.html 87/256/635/786/795/988/992/1000/1001/1026/1038, editor.html 1256, game 32/290/784/1105/1870/1899/1903/1914/1918/1927/1931/1940/2215/2336/3053/3254/3261/3271, editor 432/462/1133/1235/1246: all hold.
- PG4, PG6, PG10, PG20: confirmed, unchanged.
- PG3: correction applied, then widened for missing system 2 (credits) and 3 (rank label): CSS :540-541 #rankLabel, game rankLabel 1861, updateRank 1865 (rankLabel.textContent = weapon tier; pipWrap hidden), appended 3309, called 2413; updateCreditHUD 2416. v2_note: credits -> LOOP_V2 §3 personal $.
- PG7: correction applied; its sentence "VISION_V2.md is not in this checkout" is false now (docs/VISION_V2.md exists): replaced with VISION_V2 §5 (guest play first) and §6.1 (rooms and private matches).
- Missing 1 halftone: rejected, covered by CD16 (core:148-155, index.html:30, game 3329-3331); PG13 holds the halftone sliders DOM, PG17 notes the CSS is live.
- Missing 2 credits: belongs to PG3 (correction already added #credits :795 / creditValEl 2415); widened with updateCreditHUD 2416.
- Missing 3 rank label + pips: #rankLabel is live -> PG3; #pipWrap dead -> already in PG17 by correction; code row GE17.
- coverage.json suggested_rows: none name PG (GP is another area).
- final-PG.json written (20 rows).
DONE
- Post-DONE consistency fixes: PG2 reason "three" -> "four token sets" (matches the corrected what/v2_note); PG15 where TutStore 2336-2338 with tutorialEl/TUT_KEY at 2335 (skeptic evidence, line checked). DONE
