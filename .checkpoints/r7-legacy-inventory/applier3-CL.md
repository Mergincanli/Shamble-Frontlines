# applier3-CL: R7 final-review fixes (fixes3) for area CL
Findings: 4 (verifier-core#0, #1, #2; graphics-engineer#3).
- verifier-core#0 (CL8 evidence): accepted. Report 2026-10-14-editor-undo-ground.md:12-15 (root cause, ink lost), :18 (clearMap), :26 (70/36 -> 140/72 -> 280/144), :27, :29 (171/134/127 'ink lost'), :30 (heap). Op: set evidence, draw-call/heap growth attributed to clearMap; also reworded the 3a sentence as a near-miss (steps-1-6 report:88-95).
- verifier-core#1 (CL8 v2_note): accepted. steps-1-6 report:88-91 "reviewer's fix as written would have left the 7 bots parked at boot unstyled", fixed in 5695e56. Op: replace "two real bugs" -> one bug + one near-miss.
- verifier-core#2 (CL12 where): accepted. shamble-game.js:2761 spawnComic('IMPALE!'), :2911 spawnComic('SLASH!'). Op: replace in where; also extended CL12.what trigger list.
- graphics-engineer#3 (CL10 what): accepted. shamble-game.js:127 sex||'m'; callers :683, :796 never pass 'f'; shamble-core.js:629 female only on sel.sex==='f'; editor :446-450 Lab. Op: replace the sentence.
apply-ops.py CL: 9 ops applied to 8 fields of 4 rows, clean.
DONE
