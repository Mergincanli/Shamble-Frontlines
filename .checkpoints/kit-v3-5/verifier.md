# kit-v3-5 verifier notes (PR #46, head f9728c9 vs main 44446a5), 2026-11-10

## Plan
1. E1 red/green: kit tests at head; red re-run with main's guard in scratch (vf5/).
2. E2 mutants: no survivors.
3. E4 gate (archive copy with tools deps) + repo-map-check.
4. E6 rule-diff 17 rows vs diff.
5. E7 his words byte for byte vs mustafa-*.md, and vs transcript if readable.
6. E8 scope.
7. kit files vs kb7/SHA256SUMS.
8. Advisor conditions 1-14, D1-D6 (as rev3 kept), E1-E5; limits in STATUS.
9. STATUS/ROADMAP truth (#44 merge facts); added lines <=120 chars outside tables.

## Results
- 1 E1 PASS. Head: routine-guard 1086/1086, settings 127/127, checkpoint 27/27 (run in the worktree itself). Red (vf5/red:
  git archive of head + main's guard): 1086 tests, 47 fail = 43 rows (3 night, 34 PR-Advisor merge, 2 check-list, 4 day)
  + 4 mutation tests (their unmutated baselines fail on main). Matches the claim.
- 2 E2 PASS. The mutation tests pass at head (15 routine mutants + 1 day mutant added). Own mutants (vf5/mut.sh):
  L7 list not lower-cased -> 5 fail (caught); gate names anywhere -> caught by docs/package.json row; dropping
  `.filter(Boolean)` on the names -> NO row fails (survivor). Probe row `docs/a.md\t\n.claude/settings.json\t\n`
  merges on that mutant (find returns '' first, `if (hit)` is falsy) and is refused at head. Low: unpinned line
  (routine-guard.mjs:1500), not a defect at head.
- 3 E4 PASS. vf5/gaterepo: local clone of a5 at f9728c9 + origin/main 44446a5, `npm ci --prefix tools --omit=optional
  --ignore-scripts --no-audit --no-fund` (9 packages). `node tools/gate.mjs origin/main --no-smoke`: GATE PASS, 1-4 and
  5b-5d PASS, symbols +0 -0 (core 138 / game 714 / editor 353), 5a skipped. repo-map-check: OK, 114 entries / 101 paths
  (worktree and clone).
- 4 E6 PASS WITH NOTE. All 17 rows' quoted before/after text matches the diff of 9bacf47 (advisor.md :42-43, :65-66,
  :71-83, :161-162, :199-200; CLAUDE.md :64-66, :98-99, :103; guard :29, :36-40, :75-76, :82-100, :1480-1503).
  Verdicts hold. Property check (vf5/prop.mjs, 15279 generated paths): 0 paths protectedRel refuses are allowed by
  mergeAllowed (row 1's "every path protectedRel refused is refused"). Note (Low): row 15 marks CLAUDE.md row 0
  stricter overall, but its first clause moves from "docs with no word or decision of Mustafa's" to "docs that add
  or change no word or decision", which read alone narrows the exclusion (a doc that already holds his words, not
  touched); it now equals advisor.md tier 0 (:57-58), which governs (CLAUDE.md:93) and keeps "Not tier 0 ... a doc
  that records his words". No effective loosening; the table does not say so.
- 5 E7 PASS. Transcript readable (~/.claude/projects/-home-user-Shamble-Frontlines/263eddf3-....jsonl). The four named
  entries (81ff92a6 2449 ch, 0396c02b 1275, e6340762 1484, fd00e368 2010) exist, their sha256 prefixes equal the
  ones in the saved files, and each saved ```text block equals the entry's text byte for byte (cmp). The 2026-11-09
  answers (no uuid named) equal user entry 04f223f9 (18:18:38 UTC, 384 ch) byte for byte. STATUS (vf5/quotes.py):
  all 16 added blockquote paragraphs (L206-742) are found in a saved source with whitespace from wrapping joined,
  and every source line break starts a quote line; inline quotes of his in added lines (lead line, ORDER (b),
  "Tell me in one line ...", "merge #44") found; older ones ("merge #42/#43", B.1/B.2 words) are carried from main.
  The kit-v3-4 report diff is 20 added / 0 removed lines and holds no quote of his; its recount holds (82 rows,
  37/26/19 by first word, mixed rows 32 39 46 47 51 52 55), ddb06a5 and 2894a1f are in main between 26255a9 and
  3d39527 and absent from Appendix D.
- 6 E8 PASS. `git diff --name-only origin/main...HEAD`: the 7 named files + 14 under .checkpoints/; no settings,
  .github/ or gate file.
- 7 PASS. sha256 of the 4 kit files at the worktree and at 9bacf47 equal kb7/SHA256SUMS (guard 6507f795, tests
  240ea071, advisor.md 9eb2b3ba = role-next.md, CLAUDE.md a7c1f741 = rules-next.md); 9bacf47..HEAD touches no kit file.
- 8 PASS WITH NOTES. Conditions 1-14: met in the form revision 3 kept (1: L5-L7 lower case, places exact, aliases
  recorded as limits by his words; 2: replaced by L2-L4, gitignored limit recorded; 3: each L-rule has a row only it
  refuses, mutants pass; 4: CANON + K5 win32 rows; 5: total_count rows + mutant; 6/D3: missing, 2.5, more, fewer,
  2999, 3000 matching lines, >= mutant; 7: limit in STATUS; 8: parse as stated; 9: texts exact; 11: F2 and F5 end
  with "Waits for the foundation's gate step"; 12: additions only; 13: STATUS routine line true (list_triggers:
  PR Advisor enabled false, updated 2026-11-09T18:22:27Z, last fired 12:47, mcp_connections []), queue conflict
  answer recorded; 14: SA5 answered 07:08 before 9bacf47). D1/D2/D6 fall away (no git call: confirmed, realIo
  unchanged); D4 rows present; D5 superseded by E3. E1-E4 met; E5: STATUS records both messages and the register
  line (STATUS:233); the second message's conflict answer was given in chat (transcript 2026-11-10T10:10:14Z).
  Open (expected later): condition 10's severities (STATUS:1013 says the review has not run) and the report's E5
  line (no report at this head). Gaps: no merge row for `.claude/agent-memory-local/advisor/MEMORY.md` (condition 2's
  listed row, his headline case; mergeAllowed refuses it); STATUS:1010 drops "after the gate step or by his explicit
  word" from the advisor's E4 foundation line.
  Live: #1 54 lines / 51 renames / changed_files 54; #44 15 lines; #40 exit 1 after 100 lines (proxy 403),
  changed_files now 438. mergeAllowed on real lists (vf5/e9.mjs): #1, #42, #43, #44, #45, #46 all refused (named).
- 9 PASS WITH NOTES. #44: merged_at 2026-11-09T18:21:13Z, 44446a5, head 7c36d5a: matches STATUS/ROADMAP. Main CI run
  42673290304: attempt 1 smoke only, attempt 2 gate+smoke success 06:36-06:37Z: matches. F3's nine citations now
  right (README :120 :161 :193-194 :203 :295-299 :409 checked). But STATUS:145 (untouched) still cites :103 :104
  :122 :160 :183 for rows now at :104 :105 :123 :161 :184 (Low). Added lines over 120 outside tables: STATUS:710
  (121, a quote of his), ROADMAP:110 (122) (Low). Guard code adds 7 long lines; main's guard already has 113.
- Head moved: PR #46 is at 2b8c596, one commit past f9728c9 touching only .checkpoints/kit-v3-5.md.

## Verdict: PASS WITH NOTES (no Critical, High or Medium; six Lows, two open items expected later)
