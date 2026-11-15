# C62H-BOUND build brief, revision: netcode-architect notes

Started 2026-11-15. Seat: netcode-architect. Input: c62h-bound-brief.md, 540 lines, sha256
ba6bdeb02e61aa83ff40aacdc72687871c8e9002350573cbfa395f95accc69b8 (= HEAD's blob, checked).
Tools: this seat has no Edit tool (Read, Grep, Glob, Bash, Write only). The brief is therefore rewritten whole with
the Write tool (no sed, no shell write), and the word diff against HEAD's blob (sha256 ba6bdeb0, checked equal) shows
that only the listed items changed. This notes file is rewritten whole with Write at each update.

## Plan
1. Read the brief, the advisor's read (ADV-CBB-1..7, conditions 1-9), the test-engineer's read (TE-CBB-1..12), the
   paste (stop rules :120-124, size line :89), the earlier notes.
2. Read the code facts the new text needs: P0 verdict sha256s (full), P0 and P2 run names, summary-line form,
   l1 canonical form, figures.rs gap/note form, history.md :2175 wording, PR number for the tier line.
3. Write the revised brief whole (Write), items: ADV-CBB-1 (route W only), -2 (SIZE-CHECK), -3 (section 10 quotes),
   -4, -5, -6, -7; B-1/B-2/B-3 rulings recorded; TE-CBB-1..4 conditions; TE-CBB-5..12 carried or deferred.
4. Checks: line length <= 120; grep -F each quoted stop-rule line in the paste; word diff vs HEAD; sha256sum, wc -l.
5. Map items to new line numbers here; return the structured result.

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
- Decisions on the Lows: TE-CBB-11 deferred in writing (B-3 kept as confirmed; old-logs claim shown by the gaps
  fixture's case 1, no judge call added). All other TE-CBB items carried. TE-CBB-2's and -8's new cases are asserts
  inside existing test functions, so the expected counts stay +7/+9, +6/+6, +1/+1, 0; final 552/632.
