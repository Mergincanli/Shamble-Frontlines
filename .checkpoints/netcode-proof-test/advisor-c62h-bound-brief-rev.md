# Advisor: check of the revised C62H-BOUND brief (rev)

Seat: advisor, day session, read only (notes only). Advisor file read from origin/main (git show).

## Plan
1. Locate the previous version (sha256 ba6bdeb0...) and diff it against the revised brief.
2. Read my conditions 1-7 (advisor-c62h-bound-brief-full.json); check each is applied as written.
3. Check section 10's stop rules and size line word for word against c62h-bound-paste.md:120-124 and :89.
4. Read the test-engineer's conditions 1-5 and TE-CBB-1..12 (c62h-bound-brief-test-full.json) and the reviser's notes
   and map (c62h-bound-brief-rev-notes.md); rule each applied/deferred item.
5. Check frozen text (c62h-bound-freeze.txt) unchanged, figures' meanings unchanged, no other step changed, and the
   size estimate under the 1,050 stop.
6. Verdict + findings + conditions.

## Notes (one line per point)
- N0: HEAD's copy of the brief is already the revised one (autosave 09cca21, sha cffbe065...). The previous version
  ba6bdeb0... is at commit c0387c7 (540 lines). Diff base: c0387c7. Revised: 662 lines.
- N1: inputs hashed: paste cfac7502 (= freeze), texts v3 3f21c9b7 (= freeze), freeze.txt 9cb8be17 (= brief :17, :101).
  None of the three changed since bedca79 (his 20:50 record).
- N2: read the revised brief whole (662 lines) and the reviser's notes and item map.
- N3: git diff -U0 c0387c7 09cca21 on the brief: +241 -119, 46 hunks; read every hunk. Every hunk maps to an item in
  the reviser's map (ADV-CBB-1..7, B-1..3 rulings, TE-CBB-1..12, route-M removals). No hunk outside those.
- N4 cond 1 (ADV-CBB-1): route W only (:43-58); one launch per commit step, general-purpose/opus/xhigh, no isolation,
  told the path; ends clean with files staged; main session commits between launches; insert part 1 (after the built
  check's negative run) before step 1's launch; part 3 + built check before step 3's launch (:54-56, :283-284); fix
  pass = one more launch. APPLIED as written.
- N5 cond 2 (ADV-CBB-2): pathspec exactly as my fix (:345-346); after each commit incl. fix pass; total at last code
  commit; results = evidence, named not counted (:349-350, :569-571, :625-626). APPLIED.
- N6 cond 4/5/6 (B-1/B-2/B-3): :227-229, :641-642; :285-299, :351-352, :402-407, :570, :580-583, :632, :643-651;
  :447, :500, :652-654. APPLIED. Cond 7: ADV-CBB-4 :460-462, :466-469, :574-576; -6 :313-321, :548-549; -7 :35-36.
  APPLIED.
- N7 cond 3 (ADV-CBB-3): python -I compare: brief :603-607 (minus "> ") == paste :120-124 exactly (True); :609-611
  joined == paste :89 exactly (True). His words cited right: history.md :2172 (size), :2173 (sha/changes back), :2174
  (P0 re-judge), :2175 (stop rules stand). Paste :105 is the measured-scope line (my :106 was off by one; the brief's
  :105 is right). Reading :616-626 = my condition 3 text, plus the size reading from cond 2. APPLIED.
- N8 conjunction check: PM-0001-17's L1 P2 part carries only decoded_hash_mismatch_count (all parts) plus the two new
  checks ("setup" is on part "every S30", "loses no group" on "L1 P0"), so "the other new figure and
  decoded_hash_mismatch_count" IS "every other check" of that part: the reading is not narrower than his words.
- N9 TE-CBB-1: built-check negative :145-147 (pre-part-1 worktree; word-diff check must FAIL on an empty diff, which
  forces a strict "equals the sentence" check); compare :463-465 sha256 gate + self-test :474-478 (positive control +
  3 negatives); set check :524-527 (fc411116c1f1/P2 exists: runs.txt, verdict.json). ACCEPT.
- N10 TE-CBB-2: phrases :241-249 vs frozen row (texts v3:83): same eight gaps, in order; phrase 2 spells out "it"
  (needed to tell it from phrase 3). Case 1 pinned by first-gap order + its phrase + CB-P16; pair split by case 2's
  "second not first" + CB-P17; length case on the new list + CB-P18. ACCEPT.
- N11 TE-CBB-3: P0 runs.txt :37 vs :42 differ in name, client hashes (878352 -> 878304), incomplete (0 -> 48) and the
  wall time; summary format's first field is the run name (scenario.rs:1621). ACCEPT.
- N12 TE-CBB-4: 538+7+6+1 = 552; 616+9+6+1 = 632; six engine.rs functions listed; stop on any other count. ACCEPT.
- N13 TE-CBB-5: P0 verdict.json -17 why ends with whole-as-lost's "no run yet" (idx 10 of 11); verdict-bites.json -17
  why has no bite line (9 lines, last "every S30"): placement right. sha256 e926b812/b472784f re-hashed equal. ACCEPT.
- N14 TE-CBB-6..10, -12: all carried at the mapped lines; lib.rs:235-240 Down = 1, :262-268 + doc :343-345 flow
  code 0 = datagram, kind 1/2. ACCEPT. TE-CBB-11 deferral (B-3 kept; old-logs claim shown by gaps case 1 and named so
  in the report): ACCEPT, as the TE offered "or keep B-3".
- N15 point 3: section 2's insert algorithm and four expected hashes untouched; 3.2 items 2-4 (what figures compute,
  FIGURES `what` texts) untouched; 8.4's verdict lines untouched; order untouched; only checks added inside existing
  steps. Inputs (paste, texts v3, freeze) unchanged since bedca79. No line over 120. NOTHING ELSE CHANGED.
- N16 size: per-commit 285-400 / 255-355 / ~100 / 10-20 -> 650-875, under 1,050. But :662 still says "The plan's
  total holds (section 4)": stale, since 875 > 755. Low; one-sentence Edit (ADV-REV-1).

## Verdict: APPROVE WITH CONDITIONS (one Low Edit, then the sha on the card)
