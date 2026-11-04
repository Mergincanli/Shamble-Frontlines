# B3 final diff: completeness critic (2026-11-04)

## Plan
1. Every item of his 2026-11-04 message and his three answers, verbatim (wraps joined), with its marker.
2. Every replaced line marked or replaced, and listed in LOOP_V2 §12.
3. Grep live docs (docs/*.md, docs/adr/*.md; not reports, not .checkpoints) for the old wordings.
4. STATUS and ROADMAP against queue/docs-pass-items.md "For the NEXT docs pass".

## Results
1. Verbatim: script (scratchpad/cc-review/e1.py) joins wrapped lines; all 12 Decided, 4 Ideas and the Open item are
   found whole (amps item split as planned: §13 + §8). Answers: every clause found in LOOP_V2 (and STATUS); only
   deviations are the approved ones ("I" -> "Mustafa", answer letters -> lead-ins). Markers: Decided under
   `[DECIDED]`, ideas `[IDEA]` (LOOP 2, UNITS 4), war `[OPEN]`. Counts: `[OPEN]` LOOP 6, UNITS 0, DESIGN_NOTES 1;
   "Open since" 0. PASS.
2. LOOP §12 (LOOP_V2.md:202-219) lists all nine in-file replacements word for word; old strings survive only there.
   Outside pointers: missing ADR 0004 part 17 and ADR 0005's Spy row / inputs list (finding CC-1).
3. Grep hits, judged:
   - "more classes": DESIGN_NOTES:23 marked superseded at :24-25; LOOP §12 only. OK.
   - "tech building": his words + "(the HQ)" pointers. OK.
   - "Tier 2 or 3", "T3 units (the Spy)": §12 only. OK.
   - "shared heavy walker": LOOP_V2:107 kept on purpose with B3-Q1 pointer :108. OK.
   - "short damage boost": ADR 0005:988, pointed to in §12. OK (no ADR edit by plan).
   - "in-match progression is the team's tech tiers": extended with amps in LOOP §12 and DESIGN_NOTES:21. OK.
   - Spy's refinery hack: DESIGN_NOTES:38-39 old pointer left unmarked (finding CC-2). STATUS:113 is a merged row's
     history. OK.
   - STATUS:422 "LOOP_V2 [OPEN] items" list misses the two new `[OPEN]`s (finding CC-3).
   - VISION_V2:37, :86: handed to B2 by plan (B3-VISION), but the hand-over is recorded nowhere live (finding CC-4).
4. #35 follow-ups: item 1 (draft -> merged: STATUS:3, :19, :85; ROADMAP:93) done; item 2 (A4 in ROADMAP:71) done;
   item 3 (14:02 -> 14:03; no 14:02 left) done; item 5 (qualifier in STATUS:19-20, :84, ROADMAP:91) done, but the
   register adds two new bare "B3" uses (finding CC-5); item 6 (STATUS:230 wrap) done; routine proofs: STATUS:429-432
   says neither had run. Condition 9's bookkeeping ("taken by B3 (PR #36)" in the queue file) not done (CC-6).
   STATUS:3 lost its "and" (CC-7).
