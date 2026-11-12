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
- 1 gate: `node tools/gate.mjs 76b4f7a` exit 0, GATE PASS; header "base 76b4f7a = 76b4f7a (files at the root) | head = working tree (files in legacy/)"; +0 -0; smoke 8/8, 5/5; 4: 51 guarded 0 flagged. Output .gate/verifier-2-1b-gate.txt
- 2 kit tests: 1261 pass, 0 fail, exit 0 (.gate/verifier-2-1b-kittests.txt)
- 3 repo-map-check: "repo map OK: 112 entries cover 99 required paths" exit 0
- 4 docs-check: 20/20 PASS, "docs check: PASS"
- 5 report-appendix check: A1-A6 + fresh build PASS, "appendix: PASS"
- 6 kit-check b6b3b81: 14/14 PASS, "kit lines: PASS"
- 7 e1-final HEAD: 12 PASS, only FAIL "no .checkpoints/ file left on the head" (by design). Ran on 6abd099: the saver committed a checkpoint during the run; 43733e5..6abd099 touches only .checkpoints/foundation-restructure.md and this notes file.
- 8 sim: `xvfb-run -a node tools/preview.js legacy eval tools/sim.js` exit 0, match JSON (Cabana Republic, 90 s, err null). .gate/verifier-2-1b-sim.txt
- 9 evidence table vs files:
  - E1 = e1-e2-result.json (5 R100, commit-only, blobs e528dca 24e86d5 fce6f37 008c66f 5ba72df at A/OLD/move). OK
  - E2 = e1-e2-result.json e2: 44446a5 gate, exit 0, +0 -0, 8/8 5/5. OK
  - E3 = e3-results-2-1b.json: 31/31 old.failing == head.failing; 26 P+M; C1-C3, PC2 exit 0 both; PC1 [5a,5b] vs P1 [3,5a,5b]; M19/M20 reportChecksInOld false. Bases file: 5 bases PASS, 8e58937 +34 -0. OK
  - E4 = e4-result.json: 35792/37063 ("bytes" field, string length; files on disk 35794/37065), builder 78862/22018. "default-map cmp" is NOT in e4/e6 result JSON: only checkpoint :183 and test-engineer F2. Reproduced myself: cmp .gate/.../e6/tuning-cabana.json .gate/.../e4/a-cabana-1.json exit 0. Low (provenance).
  - E5 = e5-result.json: SHARED, control + reverse, backend localStorage run 1, console lines 10/10; copy() stub in e5-saves.mjs:92-105, headless:true. OK
  - E6 = e6-commands-result.json: 14/14 exit 0. OK
  - E8 = e8-result.md: M c33ac99, R 5f91112, tree = 76b4f7a, gate c33ac99 PASS header, smoke-only PASS. Table wording "merge of the PR head (c33ac99)": c33ac99 is the merge commit M, the PR head was f0322d2 (deviations line says so). Low wording.
  - Kit = kit-check-result.json 14/14 (my run rewrote it, same content). Case = case-test-result.json r1/r2 not counted, r3/r4 controls. OK
  - CI #674/#675 confirmed via gh: success on c090843, headers match. CI on 43733e5: #695 push, #696 PR success.
- 10 console lines: both equal byte for byte (198 and 214 bytes, sha256 4f3eacc5…, 1d02cb64…) after the 3-space markdown list indent. E5 run 4 reads them from the md file (e5-saves.mjs:94).
- No kit/gate/.github/spikes change after b6b3b81; vs main only verifier.md:28 (kit line 4).
- 11 fix pass 43733e5 on tools/README.md: prose only (build-folder paragraph sentence; syntax-hook instruction names legacy/shamble-game.js); no shell command line changed, so E6's 14/14 still covers every changed command line.
- 12 letter-case row j: traced to 2.1a's notes at 1779ba7 (.checkpoints/foundation-restructure/evidence-f1/rows-f1-on-3f29cb8.txt:10, "PASS j_letter_case: exit 2 ... still holds Index.html"); tools/gate.mjs blob 7a8480c at 3f29cb8 = HEAD. OK
- Final recheck: HEAD 6abd099 (43733e5 + checkpoint-only); git status shows only this notes file modified, so the kit-check rerun rewrote kit-check-result.json with identical content.
- VERDICT: PASS. Lows: E4 cmp + local move-commit gate recorded only in the checkpoint (cmp reproduced, cabana only); E8 "(c33ac99)" wording; not verified list.
