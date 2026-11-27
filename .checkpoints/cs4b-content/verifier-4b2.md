# Verifier notes: change-set 4b-2 on 914f3b4 (2026-11-27)

## Plan
1. Local checks (run 1) in background -> .gate/verifier-local-checks.log; read the numbers.
2. HEAD vs 914f3b4: code diff empty; change-set file and line counts against main b8f02b0; commit count.
3. Written lines (run 2) via count-4b2.mjs.
4. His words (run 3): decisions-4b2.mjs check; quotes in draft and pr-text.
5. Tools on real files (run 4): testlines --check, content_bundle --check, recordings unchanged since e1e2dd4.
6. Plants (run 5): result file vs log for 5 plants (2+ from W14-W17, C38-C62); two plants of my own in a copy.
7. Reviews (run 6): each verdict and finding vs notes; spot-check >= 6 fixed rows (TE-1, RR-1 among them).
8. Not mine: CI runs, Firefox, legacy smoke.

## Findings
- F1 OK: `git diff 914f3b4 HEAD -- crates packages content Cargo.toml Cargo.lock` empty.
- F2 OK: `git diff --shortstat b8f02b0 914f3b4 -- . :(exclude).checkpoints`: 102 files, 10381 +, 339 - (draft: same).
- F3 OK: `git log b9aad01..914f3b4 -- crates packages content Cargo.toml Cargo.lock`: 23 commits, same ids and order as draft's list.
- F4 OK: count-4b2.mjs on numstat b9aad01..914f3b4: official 8408, raw 8426; at c442a80 7978; at 70f11e1 8347 (369, then 61). 8408/3155 = 2.665 -> 2.7; rows sum 3155.
- F5 OK: decisions-4b2.mjs check: 6 of 6 word for word, exit 0. Quotes: STEP12-GO 20:03:18 UTC (DECISIONS.md:183), SOURCE-GO "first lines of its report" (:596), SIZE-4B two-thirds sentence (:533-534). 5,668 sentence: card .checkpoints/cs4b-content.md:1173 (chat itself not readable by me).
- F6 Low (pending [after]): pr-text-4b2.md "Evidence, in short" still says 8,347 and 97 of 97 / 17 of 17 (marked [after: round 2] / [after]); final: 8,408 and 104 of 104 / 19 of 19.
- F7 OK: testlines.mjs --check exit 0 (15 values, sha256 a9d47785...); content_bundle --check exit 0 (2 files, 8375 bytes, b9901bc9...); git diff e1e2dd4 914f3b4 -- crates/devtools/recordings empty.
