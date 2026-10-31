# Step 3d, R1+R4: the final check's report (verbatim)

Workflow run wf_6ff21603-bee (prompt `prompts/verifier-r14-3d-final.txt`; the script reused from the R3+R5+R6 final check, so its label reads "r356"), launched 2026-10-31 10:19 UTC, returned about 10:24 UTC. Notes: `verifier-r14-3d-final.md`.

VERDICT: PASS WITH NOTES. One problem is left: two UNREAD_SOURCES rows are wrong. Everything else is a nit.

1. **Fix items: closed.**
   - r1:2050-2051 now names the tick period.
   - All five bare 48s now read "the largest mode's N (48, or 54 under MODES-3 (b))": r1:2281, r1:2740, r4:778-779, r4:879.
   - The avoidance mark (r4:881-882) scales ×1.125 at N = 54. That matches GLOSSARY §1: I recomputed the 60 Hz sim share at N = 54 and got 4.142, which is 4.14 only if avoidance is scaled too.
   - The pointer lines (r1:2241, r1:2285, r4:898), both "Rests on" lines (r1:1915, r4:855) and the three hash-map ban additions (r1:1633, r1:2408, r1:3139) are all present.
2. **The 5 recommendation changes: closed.** Each value holds. The only new number is ×1.125 (54/48, stated in GLOSSARY §1), and the "(proposed)" marks are kept. Both MODES-3 readings are written out in each sentence.
3. **No new claim, number or source: closed.**
4. **Glossary hits on changed lines:** 9 judged, 9 right, 0 wrong. "Retire (exact)" is still 0 (I re-ran glossary_check.py: R1 0, R4 0).
5. **GLOSSARY.md: closed.** Only the five nits changed and no figure was removed. Nit 4 went into "The server tick budget" entry (G:177), which is where that wording actually was.
6. **UNREAD_SOURCES.md:** 31 of the 33 rows hold, 2 do not.
   - UNREAD:221 (disallowed_methods.rs) and UNREAD:223 (…/clippy_lints/src/disallowed) are tied to R5-RUSTLINT-REC3 and REC4. That hinge row is about the `panic` and `arithmetic_side_effects` source files. These two URLs are the disallowed lint, which R5 read through disallowed_items.rs (r5:2432, r5:2332).
   - The cause is in build_unread.py: the hand-tie list (MANUAL_TIES) uses the folder path `…/clippy_lints/src/` as a key, so it matches any file under it. That is the same prefix fault the fix was meant to remove. Both cells should read "no tie found in the file or notes".
   - The sample's rows 3, 5 and 10 now hold.
   - Nits on rows that did not change: UNREAD:102 and :103 (the sohu.com and juejin.cn links) and UNREAD:137 (gdcvault 1024041) may also belong under R2-SYNC-REC1 and R2-SYNC-REC3.
7. **Labels:** R1 keeps "verified (every recommendation claim)"; R4 keeps "spot-checked (50 of 52 key claims)".

**Claims checked at a source:** none. No new source claim needed one.

**Host audit:** curl 0, WebFetch 0.

Notes: /home/user/Shamble-Frontlines/.checkpoints/v2-research/verifier-r14-3d-final.md (ends with DONE).
