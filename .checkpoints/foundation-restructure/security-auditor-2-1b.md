# security-auditor, step 2.1b (PR #51), round 1

Head seen: dc8582af455036fcc06568c403ec959c9f56de2d at start (expected dc8582a: matches); the tip moved to 722d2c1
(checkpoint-only) during the review; `git diff --stat dc8582a HEAD -- . ':!.checkpoints'` is empty. Base 76b4f7a.

## Plan
1. Kit lines: diff CLAUDE.md, verifier.md vs approved before/after (kit-check.mjs, plan).
2. Guard: guard.mjs / routine-guard.mjs protected paths, GATE_FILES, any root-path assumptions.
3. Gate move check gate.mjs:242-249; decoy/stray root page; letter case.
4. README build-folder paragraph vs smoke.mjs findBuild/LEGACY_JS, gate.mjs BASE_PREFIXES/basePrefix.
5. saves-console-lines.md: both lines, read/write, network, delete, restore-line eval of pasted text.
6. DECISIONS.md entries byte-check (decisions-entries.mjs).
7. Rest of diff skim (README, tools headers, docs) for secrets/loosened rules.

## Notes (a line each, as found)
- Step 1 kit lines: `git diff 76b4f7a HEAD -- CLAUDE.md .claude/` = exactly CLAUDE.md :19, :43, :44 and verifier.md:28; the
  four pairs equal kit-check-result.json (approved Before/After, 14/14). b6b3b81 touches only those 2 files; no other
  commit in 76b4f7a..HEAD touches .claude/, CLAUDE.md, .github/ or a GATE_FILES path. Mustafa's yes
  (mustafa-2026-11-11-answers.md, event 45563b1b): "exactly the four lines ... in one commit that holds nothing else". Nothing
  loosened: the lines only rename paths. PASS.
- Step 2 guard: routine-guard.mjs GATE_FILES (:28-29), protectedRel (:35-48), MERGE_PLACES (:84), DENIED_SCRIPTS (:464):
  none names a game file or the root; the five files were never protected nor PR-Advisor-mergeable and still are not
  (legacy/ is not in MERGE_PLACES). guard.mjs only imports routine-guard. syntax-gate.mjs by extension. gate.yml names no
  game path. Rules globs `**/<name>` match legacy/. `node --test` routine-guard + settings tests on head: 1234/1234 pass.
  advisor.md:69 tier-1 basenames still match. PASS.
- Step 3 move check: gate.mjs:244-249 runs only when base.prefix !== BUILD_PREFIX (the move, its revert); regex /i over
  readdirSync of the base folder. Head tree: no page or engine file outside legacy/ (git ls-tree -r), no case variants,
  no untracked html. Steady state after merge (base and head legacy/): the check is off -> F4 (Low, for 2.3).
- Step 4 README paragraph vs code: findBuild (smoke.mjs:26-34) any of three engine files -> legacy/: true; both places ->
  gate die exit 2 (gate.mjs:41,69), smoke CLI setup:true exit 2 (smoke.mjs:137,227): true; BASE_PREFIXES order and
  five FILES (gate.mjs:38-39,105-109) exit 2: true; move check any case exit 2: true; smoke file-read rule now the build
  folder (smoke.mjs:167-169, root = legacy/): "outside legacy/" true. Last sentence: Windows/macOS case folding holds
  for working-tree reads, not for the base, read via `git cat-file -e` / `git show <sha>:<prefix><file>` (gate.mjs:107,
  :112), exact on every OS (by reading, not run). -> F2 (Low).
- Step 5 console lines: report lines 107 and 120 equal saves-console-lines.md byte for byte (diff). Backup: reads keys and
  values, JSON.stringify, DevTools copy(); no network, no write, no delete. Restore: setItem only when getItem===null,
  never overwrites, no removeItem/clear, no network. The pasted text is evaluated as a JS expression (an object literal):
  JSON is a valid expression, but any other text runs as code -> F1 (Low). Clipboard/text file hold all file:// keys
  (shared storage, E5): game saves only, not secrets.
- Step 6 DECISIONS: independent check (python, no file written): each of the 7 captured messages' text block appears
  quoted in docs/DECISIONS.md and verbatim in the report; DECISIONS diff is +89 -0; SCOPE (a) quote in ADR 0002 Status
  equals the base DECISIONS entry. PASS.
- Step 7 other diff: no secrets pattern in added lines. Tool headers (tools/*.js, village.py:2): comment-only, `.` ->
  `legacy`; village.py:2's default claim matches village.py:10-11. For Mustafa steps 1 and 6 name the branch -> F3.
- Report body read whole (1-264): the auto-mode messages (A4, A6) framed as "a session cannot answer a prompt, and your
  rule bars it from editing settings or allow-lists" (:89-92): nothing recorded wider than his words. PROMPTS-2.3 (a)
  (:95-98) -> F5 (Low, advisory).
- STATUS diff read whole: STATUS.md:93 "its report closes each" (IE-2/3/4/7, SA-L1); the README paragraph covers SA-L1's
  substance, but the report never names these ids -> F6 (Low).
- Memory: no local agent memory in this cloud checkout (.claude/agent-memory-local absent); 2.1a's report (:55-58, :71)
  records F1 fixed and no acceptance of the steady-state dormant check. No memory write (task limits me to this file).

## Findings
- F1 Low: restore line evaluates the pasted backup as a JS expression. report:117-122, saves-console-lines.md §2. Fix in
  the fix pass (a pre-merge step for Mustafa; a follow-up would come too late): add one sentence, "paste only the text
  the backup line made; it starts with `{`", keeping the line (changing it voids the 10/10 proof); or accept explicitly.
- F2 Low (by reading, not run): tools/README.md:14-16, last sentence: case folding applies to working-tree reads only;
  base reads via git (gate.mjs:107, :112) match exactly. Fix: "the head's file reads ...; a base is read from git, whose
  paths match exactly everywhere". Follow-up acceptable.
- F3 Low: report:101 (zip by branch) and :125 (worktree from origin/<branch>): his checks follow the branch tip, not the
  approved head (the tip moved once during this review). Fix: the PR body / chat For Mustafa names the final head SHA
  (archive/<SHA>.zip; `git worktree add ..\shamble-2-1b <SHA>`). Fix pass.
- F4 Low (for 2.3): after merge the move check is off; a later root page or shamble-*.js is unjudged. Mitigation: not
  PR-Advisor-mergeable (MERGE_PLACES). Fix in 2.3: refuse page/engine names outside the build folder always.
- F5 Low (advisory): report:95-98: a settings allow rule cannot see the folder, so git merge/revert/worktree remove/sed -i
  allowed "inside .gate/" would also pass in the main folder. Fix: the option says guard readings that check the folder.
- F6 Low: docs/STATUS.md:93 vs the report: add one line mapping IE-2/3/4/7 and SA-L1 to their closures.

## Verdict
APPROVE WITH LOWS (no Critical/High). Tier 2: needs-mustafa regardless. Anti-cheat table: not applicable (no network,
server or game-logic change; five R100 renames). Not verified: captures vs his live chat, Windows/macOS, Brave.
