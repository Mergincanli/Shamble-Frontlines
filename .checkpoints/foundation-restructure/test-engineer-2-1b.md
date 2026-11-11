# test-engineer, review of foundation 2.1b (PR #51), round 1

Head seen: dc8582af455036fcc06568c403ec959c9f56de2d (branch claude/clever-fermi-oudsfq), base 76b4f7a.
Memory: none found (memory: local; no .claude/agent-memory-local/test-engineer).

## Plan
1. Read the report (docs/reports/2026-11-11-foundation-2-1b.md) and its evidence claims.
2. E1: e1-e2-result.json + e1-final.mjs vs plan 1.5 (A = 76b4f7a, five R100, blobs equal); re-run the name-status
   on the head myself.
3. E2: same file; OLD gate in legacy/tools; +0 -0, smoke pass.
4. E3: e3-run-2-1b.mjs, e3-mutations.json, e3-results-2-1b.json: 26 mutations + controls; equal non-empty sets;
   controls pass both; M19/M20 notes; bases (plan says three; brief says five).
5. E4: e4-tuning.mjs + result: A's copies of preview.js / tuning-dump.js, both maps, twice; builder cmp.
6. E5: e5-saves.mjs + result: relaunch control, backend assertion; limits.
7. E6: e6-commands.mjs + result: commands as written; repo-map-check; kit tests counts.
8. E8: e8-result.md.
9. README build-folder paragraph vs smoke.mjs findBuild / gate.mjs; case-test-result.json; rows-test j_letter_case.
10. What is NOT proven. Findings, verdict.

## Findings (a line each, as found)
- E1 re-run on dc8582a: five R100 rows, blobs equal at 76b4f7a, 44446a5 and HEAD:legacy/, modes 100644 both sides. HOLDS.
- E2: OLD blobs 44446a5 written to legacy/tools, PASS +0 -0, 8/8 5/5. HOLDS (independent judge).
- E3: 26 mutations equal non-empty sets; per-check messages equal modulo legacy/ and smoke's "moved X m"; M5 text
  "outside the build folder" (2.1a's intended change). Bases: 5 rows PASS. Head column b6b3b81: gate files and five
  blobs identical to dc8582a (diff b6b3b81..HEAD touches only docs). HOLDS.
- F1 (Medium): PC1 (--allow-drop refreshRect) exits 1 in both columns (failing 5a,5b); e3-run-2-1b.mjs special-cases it
  ("judged on check 3"); plan 1.5 says every control passes in both; report :44 says "controls pass in both". Undisclosed.
- M19/M20: 5b only / 5d only vs report's "5b + 5d": disclosed note, plan allows. OK.
- E4: dumps byte-identical (strings), A's tools, both maps x2; builder cmp-equal. HOLDS.
- F2 (Low): report :45 "the head's edited tools give the same dump (cmp)": no result file records it (checkpoint
  line only); I reproduced cmp for cabana (.gate/.../e6/tuning-cabana.json == e4/a-cabana-1.json); tri never run with edited tools.
- E5: control + reverse + unrelated folder; SHARED in Chromium 1194 headless Linux. Console lines 10/10 incl. restore
  no-overwrite. No page global `copy` in legacy/ (grep), so DevTools copy() not shadowed.
- F3 (Low): e5-saves.mjs records KVStore.backend but no check enforces 'localStorage' (plan 1.5 says asserting).
- F4 (Low): console-line run stubs window.copy; real DevTools copy()/paste path untested; report :46 "10/10" doesn't say so.
- E6: 14/14; kit tests re-run by me on dc8582a: 1261/1261; repo-map-check exit 0 on dc8582a; gate vs 76b4f7a PASS (mine).
- E8: run on f0322d2; f0322d2..dc8582a = docs only, none read by the gate: extends. Disclosed.
- README build-folder paragraph: every sentence matches smoke.mjs:26-34,135-138,227 and gate.mjs:65-71,105-108,243-249;
  proofs: 2.1a rows a-m (c8f4bb3 evidence-f1, with unfixed control j FAIL on 2bfab53), case-test r1-r4 (Linux), smoke CLI c exit 2.
- F5 (Low): tools/README.md:72 still says "put a syntax error into `shamble-game.js`" (now legacy/shamble-game.js).
- Note (not a finding of this PR): rows k_no_move_root_page: after the move, a stray root index.html with base legacy/
  passes (exit 0) by design; README states the move-only scope correctly.
- Console lines: the two ```js blocks of saves-console-lines.md equal report :107 and :120 byte for byte (script). No
  element id/name "copy" in legacy/*.html. So E5 run 4 covers what he pastes.
- PC1 history: 2.1a's notes (c8f4bb3: agent-mutations.md:18,:38 U1; e3-summary.md:13,:31) found and chose "judged on
  check 3"; 2.1a's report and 2.1b's report never say so. F1 stays Medium.
- F6 (Low): report :159 placeholder "TBD-COUNT".
- Head: started dc8582a; at the end 722d2c1 (delta: .checkpoints/foundation-restructure/security-auditor-2-1b.md only).

## Claim · proof · gap
| Claim (report line) | Proof | Gap |
|---|---|---|
| Bytes unchanged (:27, :42) | E1 on c090843 + my re-run on dc8582a: 5 R100, blobs = 76b4f7a = 44446a5, modes 100644 | final head after .checkpoints deletion: PR body only |
| OLD's gate passes moved bytes (:43) | E2, OLD blobs verified = 44446a5 | none |
| Gate bites as OLD (:44) | E3 26/26 equal non-empty sets, messages equal modulo path/timing | PC1 exits 1 both sides, report says controls pass (F1) |
| Old/new bases (:44) | E3 bases 5/5 PASS incl. 8e58937 | none |
| Oracle (:45) | E4 dumps identical x2 both maps; builder cmp | edited-tools cmp unrecorded, cabana only (F2) |
| Saves shared (:46) | E5 control, reverse, unrelated folder | Linux headless Chromium only; backend not asserted (F3) |
| Console lines (:46, :107, :120) | E5 run 4, 10/10, same bytes as report | copy() stubbed, DevTools path untried (F4) |
| Commands (:47) | E6 14/14; kit tests 1261 (re-run by me); repo-map-check 0 (mine) | none |
| Revert green (:48) | E8 on f0322d2; since then docs only | not re-run on final head (not needed: gate reads no doc) |
| README build-folder paragraph (tools/README.md:7-16) | 2.1a rows a-m incl. unfixed control; case-test r1-r4; smoke CLI c exit 2 | Windows/macOS by reading (disclosed) |

## Verdict: APPROVE (no Critical/High). F1 Medium, F2-F6 Low: fix pass or STATUS follow-ups.
## NOT proven: Brave on Windows (saves, pages); the legacy/ layout gate on Windows (his step 6; his run was on
76b4f7a); E1 and CI on the final head (PR body; e1-final.mjs fails by design until .checkpoints is deleted); real
DevTools copy()/paste; Windows/macOS letter case.
