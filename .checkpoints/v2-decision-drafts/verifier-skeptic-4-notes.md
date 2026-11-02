# verifier-skeptic-4 notes (skeptic, plan step 7)
Started 2026-11-02 09:22 Amsterdam. Branch claude/laughing-lovelace-ox61se, docs at 4de24d2.
Tasks: F1 (0002 rows 27/33, erasableSyntaxOnly enum), F3 (0002 PM-0002-45 vs G7), F5 (0004 row 52, README row).

## Progress
- [ ] read verifier-fixcheck-b-verdicts.md rows 27, 33, 52, findings 1, 3, 5
- [ ] F1 history: verifier-adr0002-verdicts F-14, TE98; skeptic-2 results
- [ ] F1 fetch tsconfig page
- [ ] F3 read 0002 part 4, G7, PM-0002-45, SPIKE_PLAN
- [ ] F5 read 0004:805-807, README

## 09:25 F1 so far
- Docs at 4de24d2 = HEAD (git diff 4de24d2 HEAD -- docs/ empty; tree clean). Rows now at 0002:1059 (PM-0002-17), :1085 (-42), :1088 (-45).
- Skeptic-2 results/notes: 0 hits for F-14 (it never judged F-14). F-14 itself (verifier-adr0002 :673-676, TE98 :399, S-18 :571) rested on fetch G02 quoting only the last sentence + parameter-property example.
- Fetched tsconfig 09:22:41, 200, 812109 bytes, byte-identical to .gate/fixcheck-b/fetch/tsconfig.html. My own extraction: .gate/skeptic4/erasable-section.txt.
- Section text: "Node.js supports running TypeScript files directly as of v23.6; however, only TypeScript-specific syntax that does not have runtime semantics are supported under this mode. ... That means the following constructs are not supported: enum declarations; namespaces and modules with runtime code; parameter properties in classes; Non-ECMAScript import = and export = assignments; <prefix>-style type assertions", code block "// ❌ error: An enum declaration.", then "The --erasableSyntaxOnly flag will cause TypeScript to error on most TypeScript-specific constructs that have runtime behavior." + param-property example "error! This syntax is not allowed when 'erasableSyntaxOnly' is enabled."
- Skeptic angle: the "not supported" list + ❌ block are framed as Node's strip-types limits; the flag sentence says "most". So "names no enum" is plainly false; "rejection unverified" is weaker but the entry ties them. Next: TS 5.8 release notes (same host) + TS test baseline (raw.githubusercontent.com) to settle "most".

## 09:26 coordinator added two findings (deadline now 11:15 Amsterdam)
- ADR 0001 row A8 (fixcheck-a): 0001:230-231 lists "the events of ADR 0004 part 19: hits, deaths and kills, purchases, uplink cache events, tech tiers, upgrade picks and pickups"; verifier: 0004:454-458 also lists blasts, harvester deliveries and kills, structure kills and stage changes, plus cosmetic shot events (0001:234 sends once).
- ADR 0003 rows B14, B18, B38 (fixcheck-a): 0003:247-248, :432, look checklist step 24 (~:1097-1098) point to README row "fragments as damage"; verifier: README has none, only gap G5. Same as F5.
- Same results file; add rows.

## 09:27 F1 evidence
- TS 5.8 notes fetched 09:23:44 200 226596: section "The --erasableSyntaxOnly Option": same construct list + "❌ error: An enum declaration." then "That's why TypeScript 5.8 introduces the --erasableSyntaxOnly flag. When this flag is enabled, TypeScript will error on most TypeScript-specific constructs that have runtime behavior." Extract: .gate/skeptic4/ts58-erasable.txt
- Tried TS repo test/baseline/checker on raw.githubusercontent main: all 404 (URLs mine, not from a page). Stopped guessing.
- r5:221-224 F12 (general knowledge, unverified) and r5:1262 REC1 "an `enum`" confirmed; research unchanged 9e7db28..4de24d2 except GLOSSARY.
- F1 verdict leaning: UPHELD (doc wrong): "names no enum" false; flag tie explicit in 5.8 notes. Residual: "most" = a hedge only tsc closes; ambient (.d.ts) enum not covered by either source.

## 09:40 F1 row and F3 row written (both upheld)
- F3: part 4 0002:237-241, G7 :455, item 8 :960-962, PM-29 :1071, App A :1326, SPIKE_PLAN :81/:299/:57-58, PM-52 :1095, R5 G7 row r5:1718 "(its size is set once REC4's budget exists)".
- Next: F5 (0004:805-807 vs README), then A8 (0001 vs 0004 part 19), then B14/B18/B38 (0003 vs README).

## 09:33 rows F5, B14/B18/B38, A8 written (all upheld; F5 and B-rows true once synthesis FIX at fixes-2.md:537 lands)
- README at 4de24d2: Decisions 25-104, no fragment row; G5 at :306 under Codes (:260). Planned row fixes-2.md:537 sits before "The design questions" row (README:44), inside Decisions.
- A8: 0001:229-234 vs 0004:454-458; 0001 has no other class for blasts/harvester/structure events; blasts arguably cosmetic -> fix must place them.
- Next: fetch audit, counts line, DONE.
- (correction: the 09:40 and 09:33 stamps above were estimates; the real clock was about 09:25 and 09:29)
- 09:31 fixed line numbers in results (G6 0002:456, G7 0002:457; README 72 data rows :32-103). Fetch audit + counts line written.
DONE
