# Verifier notes: change-set 4c-1 "The page's scene" (brief: verifier-brief-4c1.md)

## Plan
1. Local checks (bash local-checks.sh -> .gate/verifier-local-checks-4c1.log), background; read counts out of my log.
2. Written lines: git diff --numstat base..0645432f -- crates packages -> .gate/verifier-numstat-4c1.txt; count-4c.mjs.
3. His words: decisions-4c.mjs check, section14-4c.mjs check, loop-check-4c.mjs, quote-check-12bc.mjs; quotes by hand.
4. Plants: 5 rows of plants-4c.txt vs .gate/plants-4c.log (F1 + one more F + 3 others); two own plants in .gate/verifier-4c1-scratch/.
5. Reviews: each reviewer note vs draft and tally; spot-check >= 6 fixed rows at 0645432f (GE-F1, TE-1 among them).
6. After picture: shot-4c.mjs on my development build; look at it.
7. Gate (optional) at HEAD.
8. Docs rows: STATUS, STATUS_ARCHIVE, ROADMAP, REPO_MAP.

## Log (one line per finished check)
- [1 started] local checks running in background -> .gate/verifier-local-checks-4c1.log
- [2 done] numstat base..0645432f -> .gate/verifier-numstat-4c1.txt; count-4c.mjs: official 1565 (raw 1565). Cumulative: base..c1de 1288, base..2b2f 1564, base..0645 1565 -> split 1277/11/276/1 holds as growth of the cumulative count (the fix pass's own diff adds 340, the comment's 2: Low wording). 31 files, +1974 -186 at 9283d302 outside notes: holds. 0645432f..HEAD on crates packages content Cargo: empty. No crates/ or content/ file in the change-set: holds.
- [3 done] decisions-4c check: 5 messages 14/16/7117/1284/4516 bytes word for word (merge route less its line 5 of 8 = item 3: draft "line 5" and PR "item 3" both hold); section14 check: once, last, 517 words and signs; loop-check: LOOP_V2 pointers at 8 lines, EDITOR_V2 at 47; quote-check-12bc: 175 and 136 bytes in STATUS and DECISIONS. STREET-GREY quote = DECISIONS:216 = saved answers line 21. Silo pointer at LOOP_V2:52 = his item 9 words. EDITOR_V2:47 "(2026-11-27: see §14)" holds.
- [4a done] plants-4c.txt vs .gate/plants-4c.log (log appended; the 2b2f44c2 run is lines 3102-7186, 48 plants mapped run by run): F1 log 6323-6331 exit 124, only "TIMED OUT after 90 s" (the harness's words; vitest printed no test before the hang, so the log does not name the hung test); F6 6486 exit 1, "× the ground and each patch lie flat ..." AssertionError made street corners z 2.15 -> 0; F17 7046 exit 1, "Error: street_house_a_roof_north" z -5.8 -> -8 at skeleton.spec.ts:95; S1c 5488 exit 1, "made ramp_mz volume: expected -3121.19 to be close to 18.80"; T5 4601 exit 1, 5 failed incl. "nothing is read through a prototype". Counts 45/45 and 3/3 and 7 files back hold in the result file.
