Task: verifier, foundation step 2.1a, PR #50, pinned 0590d8c05227146ca255822269a8df5b997eac45, base main bb2d817.

## Plan (blocks, in order)
- B1 F1 write (3f29cb8 stat, blob, patch reproduces blob, smoke.mjs blob, kit diff empty)
- B2 full E3 compare (f1 vs recorded) + sample re-run
- B3 base cases and decoy
- B4 rows red/green + green re-run
- B5 gate on pinned head with smoke
- B6 his words (decisions check + appendix A vs sources)
- B7 docs facts
- B8 CI facts / blob equality
- B9 open findings handling
- B10 report

## Notes
### B1 F1 write: DONE, PASS
- `git show 3f29cb8 --stat`: only tools/gate.mjs (+9 -1). Blob 3f29cb8:tools/gate.mjs = 7a8480ce82..., parent 2bfab53.
- GNU patch 2.7.6 `patch -o` of f1-gate.patch onto 2bfab53 copy (.gate/foundation-restructure/f1check/) -> hash 7a8480ce8225f320de7627575321c052b8f76bd3 (match).
- 0590d8c:tools/smoke.mjs = 18058e49..., 0590d8c:tools/gate.mjs = 7a8480c.
- `git diff bb2d817...0590d8c -- .claude CLAUDE.md .github`: empty.
- 0590d8c..HEAD(9a937b0) touches only .checkpoints/.
- f1-gate.patch blob 7f274b9 identical at eaf5a45 (the commit his A3.3 yes names), HEAD and on disk; its index line 2bfab53..7a8480c.
- next: B2
### B2 (in progress; compare part done) E3 compare part: recorded vs f1, own script verifier-e3-compare.py: 31/31 ids, 0 diffs (verdict, exit, failing ids, basePrefix, per-check PASS/FAIL word, all 3 columns). 30 PASS + PC1 "PASS (judged on check 3)". Sample runner diff vs e3-run.mjs: only header comment, SAMPLE filter, OUT path. Worktrees clean before: fdn-old 44446a5, fdn-new 3f29cb8 (gate 7a8480c), fdn-newleg 7c22abf (parent 3f29cb8, 5 pure renames 100%). Sample run launched (log .gate/foundation-restructure/e3-sample/run.log).
### B8 CI facts / blobs: DONE, PASS
- tools tree f23190c and five game blobs (e528dca, 24e86d5, fce6f37, 008c66f, 5ba72df) identical at 3f29cb8, 0607b45, 76544aa, 0590d8c; game blobs equal main bb2d817.
- bb2d817..0590d8c outside docs/.checkpoints: only tools/{README.md,gate.mjs,smoke.mjs,village.py}.
- checkpoint steps 13-20 consistent: 13/16 3f29cb8 gate green, smoke failed at runner DNS/library step (smoke step never ran); 17 8d8c422 smoke failed (mirror); 19 0607b45 green (push 43063326391, pr 43063335859); 20 76544aa green (pr 43067074308, push 43067068538). Cannot read GitHub: not independently verified.
- next: B6, B7, B9 while E3 sample runs; then B2 finish, B3, B4, B5.
### B6 his words: DONE, PASS
- `python3 -I .checkpoints/foundation-restructure/docs-keeper-check-decisions.py`: RESULT PASS, exit 0, 12 appendix blocks (A1.1-A4), 13 source blocks each inserted once in DECISIONS, 0 old lines deleted (492/492 unchanged), 3 pointer lines.
- own script verifier-appendix-check.py (independent fence parser): 12 report blocks == 12 source blocks byte for byte (A1.1-A1.3 mustafa-2026-11-10.md; A2.1-A2.3 -answers; A3.1-A3.2 mustafa-2026-11-11.md; A3.3 -f1; A3.4 -prompts; A3.5 -prompts-correction; A4 = bb2d817 message's 5 "> " lines, prefix removed). RESULT PASS.
- notes sources unchanged 0590d8c..HEAD (only foundation-restructure.md and ie6-probe.txt changed).
- next: B7
### B7 docs facts: DONE, PASS (one Low)
- STATUS.md 116 lines (<=120). #49 facts match git: bb2d817 parents 2d22b69 + 2e6e5d2, commit dated 2026-11-11T09:03:10Z (STATUS/archive give merged_at 09:03:11 and say the commit is dated 09:03:10: consistent with checkpoint step 9's read-back); go ~09:02 UTC, session_0173vVo6Vtm2C54toWvTt8sR, PR comment 6779064873, run 42991918886: all equal bb2d817's message / archive row (STATUS_ARCHIVE.md:159).
- STATUS "From foundation 2.1a (PR #50)" bullet (STATUS.md:93) ids = archive group (STATUS_ARCHIVE.md:506-548): 2.1b IE-2, IE-3, IE-4, IE-7, SA-L1; 2.3 IE-1, IE-5 (Medium), TE-2, TE-7, R2-1, A-1, N2, L-4, v2 root page; drafts: five lines; docs passes: stale citations. Cited lines checked: tools/gate.mjs:247 (move-check regex), tools/smoke.mjs:31 ("keep one copy"), ADR 0002 :38 :724 :1115, SPIKE_PLAN :100 :202 all hold "before the foundation".
- ROADMAP 2.1a entry (ROADMAP.md:106-118) matches the diff (smoke findBuild, gate base prefixes + F1 move check, village.py legacy default, README gate-globals line); "Next:" clause deliberately unfinished.
- Pointers: DECISIONS :38, :349 (after ">" blocks, outside them), :567 (italic continuation of list item 3, after the closing quote mark of a quoted sentence; not a ">" block, his words unchanged); STATUS :40; SPIKE_PLAN x3 and ADR 0002 x1 (outside any ">" block). The quoted pointer text "Steps 2.3 to 2.6 keep their order and wait for their results." occurs in mustafa-2026-11-10.md and DECISIONS.
- `node .claude/scripts/repo-map-check.mjs`: repo map OK, 116 entries cover 103 required paths, exit 0.
- FINDING V-1 (Low): STATUS.md "Open risks" CI line says smoke "failed twice on #50's 3f29cb8 (a runner DNS outage)"; per checkpoint steps 13-17 and infra-engineer-round2 "Precision": on 3f29cb8 the push run failed both attempts, the pull_request run failed once and its attempt 2 was cancelled at the job timeout; 8d8c422's pull_request run failed too (mirror). Understated; archive IE-5 names 3f29cb8 and 8d8c422 correctly. Fix with the last commit or leave.
- next: B9
- V-1 stays Low (wording nit; infra-engineer-round2.md R2-2 called the line "accurate" counting runs): my own count from checkpoint steps 13-17: 3f29cb8 three failed attempts (push x2, pull_request x1) + one cancelled (pull_request attempt 2, job timeout); 8d8c422 one failure; the line omits 8d8c422's failure and that 3f29cb8's PR attempt 2 was cancelled (timeout); archive IE-5 has both.
### B9 open findings handling: DONE, PASS (TE-1 re-proved in B3)
- Medium and above across rounds: F1 High (security-auditor r1, advisor r1) -> fixed 3f29cb8 (B1 verifies the write), closed by security-auditor round 2 APPROVE ("F1 (High) CLOSED"). TE-1 Medium (revert-case base evidence not reproducible) -> raw evidence-f1/base-revert-*.{json,exit,stderr}; test-engineer round 2: "TE-1 (Medium) closed"; re-run by me in B3. IE-5 Medium (CI live mirrors; infra-engineer r2 as "R2-3") -> deferred in writing: STATUS_ARCHIVE.md "From foundation 2.1a" -> "For the gate step (2.3)" and STATUS.md:93. No other Medium+ in round 1 notes (infra-engineer r1: none; advisor r1: F1 + TE-1 only) or round 2 (test-engineer: "No Critical, High or Medium open").
- Lows: deferred in archive: IE-1..IE-4, IE-7, SA-L1, TE-2, TE-7, R2-1, A-1, N2, L-4, v2 root page. IE-6 settled not a finding (evidence-f1/ie6-probe.txt: exit 0, no stderr, header "base 89286ed (files in legacy/) | head = working tree (files at the root)"). Not in the archive and left to the report (allowed for Lows): TE-4, TE-5 ("say it in the report"), TE-8 (comment in a checkpoint script), L-1, L-3, SA-L2/L-2 (fixed per SA r2). Report body is deliberately unfinished: these must appear there.
- next: wait for the E3 sample, then B2 finish, B3, B4, B5.
### B-sym symbol diff vs plan: DONE
- Game files: unchanged blobs (B8) -> +0 -0 (gate check 3 in B5).
- Tools top-level names (own acorn script verifier-toplevel.mjs, bb2d817 -> 0590d8c): gate.mjs +[readdirSync (import), BUILD, BUILD_PREFIX, BASE_PREFIXES, basePrefix] -[]; smoke.mjs +[LEGACY_JS, findBuild] -[]. F1 (dd12aeb->3f29cb8) adds only the readdirSync import (inside the exact patch he said yes to).
- Plan rev3 1.3 names BUILD, BASE_PREFIXES, basePrefix (Symbols line) and findBuild, LEGACY_JS (as-built paragraph); "Removed names: none" holds.
- FINDING V-2 (Low): BUILD_PREFIX is not in plan rev3 (grep: 0 hits); it is disclosed in dd12aeb's message and the checkpoint's departure note (step 3) and was in round 1's reviewed diff. The report's departures list should name it.
- B3 prep: 44446a5 and bb2d817 have the same tools tree (0df745e; gate.mjs 277eb96), so fdn-old's gate is main's gate. 89286ed = parent 44446a5 + 5 renames R100 to legacy/ (no other change).
- B4 prep: .gate/fdn-scratch-head: five files = pinned head's game blobs, every tracked tools/ file = 0590d8c's blob (gate 7a8480c, smoke 18058e4). rows-test-f1.mjs writes only under <out>.
- B5 prep: `node tools/gate.mjs bb2d817 --no-smoke` in the main folder: GATE PASS, exit 0, +0 -0 (core 138 / game 714 / editor 353), 51 guarded refs; output .gate/foundation-restructure/verifier-gate-nosmoke.txt. Full gate with smoke after the sample.
- For the report (conditions from reviewers, not findings): security-auditor r1: report asks him to confirm A4, A1.1-A1.3, A2.1, A2.2 and states needs-mustafa; test-engineer: TE-4, TE-5 said in the report; Windows run before 2.1b.
- B2 compare extended (--full): also build and every non-5a check line text, 93 runs: TOTAL DIFFS 0. 5a lines differ in 52 of 93 (smoke walk distance): the f1 file is a fresh run, not a copy. Sample: P1, P2, P3 PASS so far, same failing ids as e3-results-f1.json.
- Extra (BRANCH-47 exception): `git diff bb2d817 d6a3fee -- .claude CLAUDE.md .github` empty; d6a3fee outside .checkpoints changes only the four tools files; gate.mjs 2bfab53 and smoke.mjs 18058e4 there = #47's head 26e0d82 = dd12aeb (no new gate content in the merge). Checkpoint step 6 records the one-time exception line.
### B2 full E3 + sample: DONE, PASS
- Recorded (2026-11-10) vs f1: 31/31 ids, 93 runs, 0 diffs (verdict, exit, failing ids, build, basePrefix, every non-5a line text); 5a differs in 52/93 (fresh run).
- Sample re-run (e3-run-sample.mjs, PLAYWRIGHT_BROWSERS_PATH=.gate/pw-browsers, 17:40-17:52 UTC, exit 0): "E3: 6 of 6 PASS": P1 [3,5a,5b], P2 [4,5a], P3 [1,2,3,4,5a,5b], P4 [3,5c], M5 [5a], C1 [] exit 0, identical on old/new/newleg; compare vs e3-results-f1.json (--ids, --full): TOTAL DIFFS 0. No smoke flake, no re-run needed. M5 5a: old "outside the repo", new/newleg "outside the build folder". newleg basePrefix "" build legacy/ (move, F1 check ran and passed).
- Worktrees clean after: fdn-old 44446a5, fdn-new 3f29cb8, fdn-newleg 7c22abf, `git status --short --untracked-files=no` empty. Output .gate/foundation-restructure/e3-sample/e3-results-sample.json, log run.log.
- next: B3
### B3 base cases and decoy: DONE, PASS
- Revert direction, .gate/fdn-new (3f29cb8, root head): `node tools/gate.mjs 7c22abf4d4... --json --no-smoke` exit 0 (no pipe), pass true, build ".", base.prefix "legacy/", check 3 +0 -0, stderr empty (.gate/foundation-restructure/v-b3-revert-7c22abf.json).
- Same vs reproducible base 89286ed2...: exit 0, build ".", base.prefix "legacy/", +0 -0, stderr empty (v-b3-revert-89286ed.json). Matches recorded evidence-f1/base-revert-root-head-vs-legacy-base.* (exit 0, prefix legacy/).
- Main's gate (.gate/fdn-old, gate.mjs 277eb96 = bb2d817's) on the legacy/ base: 7c22abf exit 2 "gate: shamble-core.js not found at base 7c22abf"; 89286ed exit 2 "... not found at base 89286ed" (= recorded base-main-gate-vs-legacy-base.txt).
- Extra: 2.1b direction, .gate/fdn-newleg vs 3f29cb8: exit 0, build legacy/, prefix "" (= recorded base-2-1b-*).
- Decoy outputs read: main gate 277eb96 exit 2 (core missing from working tree); unfixed 2bfab53 exit 0 all PASS (the hole); fixed 7a8480c exit 2 "the build folder moved from the repo root (base) to legacy/ (head), but the repo root still holds editor.html, index.html"; fixed --smoke-only exit 0 (by design: base null; only CI's gate job catches a decoy).
- Worktrees clean after (all three). TE-1 re-proved -> B9 complete.
- next: B4
