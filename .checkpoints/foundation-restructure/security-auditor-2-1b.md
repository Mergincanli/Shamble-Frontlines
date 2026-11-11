# security-auditor, step 2.1b (PR #51), round 1

Head seen: dc8582af455036fcc06568c403ec959c9f56de2d (expected dc8582a: matches). Base 76b4f7a.

## Plan
1. Kit lines: diff CLAUDE.md, verifier.md vs approved before/after (kit-check.mjs, plan).
2. Guard: guard.mjs / routine-guard.mjs protected paths, GATE_FILES, any root-path assumptions.
3. Gate move check gate.mjs:242-249; decoy/stray root page; letter case.
4. README build-folder paragraph vs smoke.mjs findBuild/LEGACY_JS, gate.mjs BASE_PREFIXES/basePrefix.
5. saves-console-lines.md: both lines, read/write, network, delete, restore-line eval of pasted text.
6. DECISIONS.md entries byte-check (decisions-entries.mjs).
7. Rest of diff skim (README, tools headers, docs) for secrets/loosened rules.

## Findings (a line each, as found)
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
  no untracked html. Steady state after merge (base and head legacy/): the check is off; a later root page is unjudged,
  like any non-build file (not a regression; noted for 2.3, no finding).
- Step 4 README paragraph vs code: findBuild (smoke.mjs:26-34) any of three engine files -> legacy/: true; both places ->
  gate die exit 2 (gate.mjs:41,69), smoke CLI setup:true exit 2 (smoke.mjs:137,227): true; BASE_PREFIXES order and
  five FILES (gate.mjs:38-39,105-109) exit 2: true; move check any case exit 2: true; smoke file-read rule now the build
  folder (smoke.mjs:167-169, root = legacy/): "outside legacy/" true. Last sentence: Windows/macOS case folding holds
  for working-tree reads, not for the base, read via `git cat-file -e` / `git show <sha>:<prefix><file>` (gate.mjs:107,
  :112), exact on every OS. -> F2 (Low).
- Step 5 console lines: report lines 107 and 120 equal saves-console-lines.md byte for byte (diff). Backup: reads keys and
  values, JSON.stringify, DevTools copy(); no network, no write, no delete. Restore: setItem only when getItem===null,
  never overwrites, no removeItem/clear, no network. The pasted text is evaluated as a JS expression (an object literal):
  JSON is a valid expression, but any other text runs as code -> F1 (Low). Clipboard/text file hold all file:// keys
  (shared storage, E5): game saves only, not secrets.
- Step 6 DECISIONS: independent check (python, no file written): each of the 7 captured messages' text block appears
  quoted in docs/DECISIONS.md and verbatim in the report; DECISIONS diff is +89 -0; SCOPE (a) quote in ADR 0002 Status
  equals the base DECISIONS entry. PASS.
- Step 7 other diff: no secrets pattern in added lines. For Mustafa step 1 and step 6 name the branch, not the reviewed
  SHA -> F3 (Low).

## Findings
- F1 Low: restore line evals pasted text. report:117-122, saves-console-lines.md §2. Fix: JSON.parse of the text, or a
  line saying "paste only the file the backup line made"; accepted risk acceptable (his own file).
- F2 Low: tools/README.md:16-17 last sentence over-reaches for base reads. Fix: "the head's file reads ...; a base is read
  from git, whose paths match exactly on every platform".
- F3 Low: report:101 (zip by branch) and :122-126 step 6 (origin/<branch>): Mustafa checks whatever the branch holds then,
  not the head the advisor approved. Fix: archive/<full SHA>.zip and `git worktree add ... <SHA>`, naming the SHA.
