# C62H-BOUND build brief, revision: netcode-architect notes

Started 2026-11-15. Seat: netcode-architect. Input: c62h-bound-brief.md, 540 lines, sha256
ba6bdeb02e61aa83ff40aacdc72687871c8e9002350573cbfa395f95accc69b8 (= HEAD's blob at the start, checked).
Tools: this seat has no Edit tool (Read, Grep, Glob, Bash, Write only). The brief is therefore rewritten whole with
the Write tool (no sed, no shell write), and the word diff against the old blob (sha256 ba6bdeb0, checked) shows
that only the listed items changed. This notes file is rewritten whole with Write at each update.

## Plan
1. Read the brief, the advisor's read (ADV-CBB-1..7, conditions 1-9), the test-engineer's read (TE-CBB-1..12), the
   paste (stop rules :120-124, size line :89), the earlier notes.
2. Read the code facts the new text needs: P0 verdict sha256s (full), P0 and P2 run names, summary-line form,
   l1 canonical form, figures.rs gap/note form, history.md :2175 wording, PR number for the tier line.
3. Write the revised brief whole (Write), items: ADV-CBB-1 (route W only), -2 (SIZE-CHECK), -3 (section 10 quotes),
   -4, -5, -6, -7; B-1/B-2/B-3 rulings recorded; TE-CBB-1..4 conditions; TE-CBB-5..12 carried or deferred.
4. Checks: line length <= 120; grep -F each quoted stop-rule line in the paste; word diff vs the old blob; sha256sum,
   wc -l.
5. Map items to new line numbers here; return the structured result.
ALL STEPS DONE.

## Progress
- 1 DONE.
- 2 DONE. Facts:
  - P0 verdicts (committed, results/u4r/93509dfd7fd1/P0/): verdict.json
    e926b812cbe57d81e251bfb5c6685b889a4d2859e9cac25022f6788bf74af095; verdict-bites.json
    b472784f2c7208aeef38af373a8bc257bf0b3b88a18dc5dc376fb6533536942e.
  - -17 why in verdict.json :338-340 ("bite ...: no run yet" x3, whole-as-lost last); verdict-bites.json -17 why
    :59-69 holds no bite line (TE-CBB-5's placement holds).
  - P0 run names in order (runs.txt:37-45): l1-p0-s1..s3, bite-tick16, bite-client-rounding, bite-whole-as-lost,
    bite-pos-4cm, bite-yaw-12bit, bite-pitch-8bit. :37 vs :42 differ in name, client hashes (-48), incomplete (+48)
    and the wall time (TE-CBB-3's precedent holds).
  - l1 for_each_canonical (lib.rs:340-398): byte 4 dir (Down = 1), byte 5 flow code (0 datagram), byte 6 kind
    (1 lost, 2 burst); one call per entry in index order after the tag.
  - judge: Lack::Gap(String) (figures.rs:12-20), the figure returns its first gap; CheckOut.note Option<String>
    (eval.rs:363-374). So each gap case can assert its own phrase on both checks' note.
  - his words: history.md:2172 (yes), :2173 (record the sha256; any change comes back), :2174 (P0 re-judge),
    :2175 ("The stop rules of your post stand. The check is never reworded to fit.").
  - paste :105 is the measured-scope line ("no lost tick is checked, as the wording says"); the advisor cited :106,
    off by one: the brief cites :105.
  - PR #40 (card :4).
  - u4r-run.sh usage: `bash u4r-run.sh [-x] [-k] <worktree> <log file> <command...>` (u4r-run.sh:16).
  - Judge peaks recomputed by hand from c-u4r-runs.mjs:152-153, :176 (not by running it): P2 measured 3 x 272.9 MB =
    780.77 MiB -> 2129.4 -> "2129"; P2 bites 2 x 56 MB = 106.81 MiB -> 552.2 -> "552"; P3 3 x 183.3 MB = 524.43 MiB
    -> 1529.502 -> "1530" (just over the half); B1 3 x 291.4 MB = 833.70 MiB -> 2253.3 -> "2253". All equal ADV-CBB-6.
- Decisions on the Lows: TE-CBB-11 deferred in writing (B-3 kept as confirmed; old-logs claim shown by the gaps
  fixture's case 1, no judge call added). All other TE-CBB items carried. TE-CBB-2's and -8's new cases are asserts
  inside existing test functions, so the expected counts stay +7/+9, +6/+6, +1/+1, 0; final 552/632.
- 3 DONE. Written whole three times with Write:
  - write 1: all items; 659 lines;
  - write 2: section 11 keeps the old "Recommended:" sentences and appends "Ruled: ..." (smaller diff); section 10's
    header quotes history.md:2172 for the size line; 3.3 item 2 keeps "proposed". Lines outside the changed ranges
    hashed equal before and after (2fbaa010...);
  - write 3: 3.3 item 2's heading shortened to "(ruled in)" so its four lines keep the old wrapping (write 2 left one
    line at 128 columns). Lines outside 285-288 hashed equal before and after (9b0d5c85...).
- 4 DONE. Final: 662 lines, sha256 cffbe0651f20d2ac7ea92a58e094a23184d6f063087bfa6cde800beca5e0a472; no line over
  120 (awk). Quote check: brief lines 603-607 each equal paste lines 120-124 exactly (grep -nxF after stripping "> ");
  lines 609-611 joined with single spaces equal paste line 89 exactly; every quoted line found by grep -F.
- Note: the session's background saver committed the final brief in autosave 09cca21 (its blob sha256 cffbe065...);
  e83ae52, its parent, still holds the old blob ba6bdeb0. This seat made no commit.

## Item map (new line numbers in c62h-bound-brief.md, sha256 cffbe065...)
- Header and binding order, the rulings recorded: 1, 3-6; section 0 items 3-4 at 19-23.
- ADV-CBB-1 (route W only; one launch per commit step; main session commits, insert part 1 before step 1, part 3 +
  built check before step 3): 43-58; consequential route-M removals and W wording at 133-135, 227-229, 283-284, 312,
  328-333, 342-344, 355, 399, 418, 420.
- ADV-CBB-2 (SIZE-CHECK excludes results/, runs after each commit incl. the fix pass, total at the last code commit):
  345-353; report 569-571; the advisor's reading 625-626.
- ADV-CBB-3 (section 10: his rules word for word, then the advisor's reading): 597-626 (header 599-601; stop rules
  quote 603-607 = paste :120-124; size line quote 609-611 = paste :89; his other words 613-614; the advisor's reading
  616-626).
- ADV-CBB-4 (two calls named; exactly one why line): 460-462, 466-469; report 574-576.
- ADV-CBB-5 (B-2's changed answers named; any other changed answer stops): 297-299, 406-407, 580-583; stop 632.
- ADV-CBB-6 (exact values; L reported, not a bar): 313-321, 548-549; report 576-577.
- ADV-CBB-7 (risk tier 1): 35-36.
- B-1 yes: 227-229, 638-642. B-2 in commit 3 with condition 5: 72-74, 87, 285-299, 403, 643-651. B-3 confirmed:
  652-654. Order as written: 655.
- TE-CBB-1 (three scripts shown to FAIL first; compare checks the earlier sha256s first): built check 144-147;
  compare 463-465 and 474-478, rejudge.txt 479-481; set check 524-527; report 574; stop 630-631.
- TE-CBB-2 (own note per gap on both figures; shadowing pair split; wrong length on the new list; plants): 239-249,
  271-278, plants CB-P16..18 at 386-391.
- TE-CBB-3 (bite summary-line comparison written out): 550-557; advisor stop 629.
- TE-CBB-4 (expected test counts +7/+9, +6/+6, +1/+1, 0; final 552/632): 334-339, 392-394; report 571-572; stop 630.
- TE-CBB-5 (exactly one why line per file, placed): 467-469, 477-478.
- TE-CBB-6 (exact 552/1530; run names in order): 313-321.
- TE-CBB-7: (a) 164-172 and CB-P12/-13 at 378-381; (b) 262-263 and CB-P14/-15 at 382-385; (c) 266-267 and CB-P8 at
  371-372; (d) 410-412; (e) 219-222.
- TE-CBB-8 (no-room-record whole hash: B 0, no gap): 268-270.
- TE-CBB-9 (the conjunction): 622-624.
- TE-CBB-10 (CB-P5 names its assert): 365-367.
- TE-CBB-11: deferred in writing, 656-660 (B-3 kept as confirmed; no old-logs judge call: the gaps fixture's case 1
  shows the reading).
- TE-CBB-12 (the read line names the changed fixture answer): 580-583.
- Size estimate (section 4): 349-352: 1 about 285-400; 2 about 255-355; 3 about 100 (B-2's 40 counted); 4 about 10-20;
  total about 650-875 (middle about 760), against the plan's 650 (545-755) and the 1,050 stop.

## Word diff against the old brief (git diff --word-diff e83ae52, whose blob is ba6bdeb0)
- Lines: 540 -> 662; git numstat +241 -119 (46 hunks). Porcelain tokens: about 378 words removed, 2,353 added.
- Removed words are only: the title's "draft for"; the header's "the build does not start until ... them"; route M's
  four lines and route-conditional phrases ("route M ...", "(route W: ...)", "(both routes)", "the route's runner");
  "if the advisor rules them in" / "if ruled in" / "(RULING NEEDED)"; the old SIZE-CHECK pathspec and plan numbers
  (270-375, 240-325, 60 (+40 with B-2), 620-800); "about" before 552/1,530 and "2,129" -> "2129"; "at most" (why
  line); "L about 2 % of S" as a bar; the old bite-line expected lines (rewritten per TE-CBB-3); the six paraphrased
  "His" stop bullets (now quoted word for word plus the advisor's reading); "the bites build's seed 1 differs from
  s1"; "Rulings needed ..., before the build" (now "Rulings on this brief"); the old gap list's wording (now each
  gap's frozen-row phrase in quotes); CB-P1's "down count", CB-P5's "fails, sum 0 and unexplained above 0", CB-P8's
  "figure A's" and "A 1"; the l1 test's "exact `sent`," and "(`for_each_canonical`)".
- No frozen text changed (texts v3 blocks are only referenced); no figure's meaning changed; no step added or
  reordered (the shown-to-fail runs and plants are checks inside existing steps).
- Left as written: section 11's "The plan's total holds (section 4)". With this revision the estimate's top (875)
  passes the plan's range top (755), though not the 1,050 stop; the main session may want that sentence read so.
