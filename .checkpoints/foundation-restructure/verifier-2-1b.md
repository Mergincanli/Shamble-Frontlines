# Verifier notes: foundation 2.1b (PR #51)

Head seen at start: 43733e55c12f756cfa8427a8126c23e88fbb1272 (expected 43733e5: matches). Base 76b4f7a.

## Plan
1. gate.mjs 76b4f7a (full, smoke)
2. node --test hooks + checkpoint tests (expect 1261/0)
3. repo-map-check
4. docs-check
5. report-appendix check
6. kit-check b6b3b81
7. e1-final HEAD (only the notes-folder check may fail)
8. sim.js on legacy (exit 0, JSON)
9. evidence table vs result files
10. console lines byte check vs saves-console-lines.md

## Results
