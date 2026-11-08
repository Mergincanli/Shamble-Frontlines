# Verifier: kit-v3-4 (A4, the review policy), head ad1f9bd vs origin/main 626df67

## Plan
1. E1: his words byte for byte (kit files, VISION_V2 + register for XP-TERM (a), STATUS for LOOK (a),
   CHECKPOINTS.md §10 for disk hygiene, STATUS queue removal with pointer). C12: not yet checkable (report comes later).
2. E2: rule-diff table (kit-draft.md §8) vs every changed/added sentence in the five kit files; verdict per row.
3. E3: three kit test files on head and on main (temp checkout under .gate/).
4. E4: gate.mjs origin/main --no-smoke; repo-map-check.
5. E5: STATUS, ROADMAP, START_HERE, REPO_MAP facts (merge times, SHAs, run ids, line numbers); docs parts.
6. E6: scope (no hooks/settings/CI/gate changed; CLAUDE.md growth <= ~25 lines).
7. C1-C13 and the addendum's two text changes.
8. Lines > 120 chars outside table rows, STATUS:3, ROADMAP entries.

## Results
(filled as they come)

### E1 (his words), done
- Script `.gate/verifier-a4/e1.py` (sources: origin/main STATUS's A4 block, joined; the three answer files' text
  blocks): 17/17 PASS, FAILS 0. advisor.md quotes items 1, 2, 3 and 5 whole, byte for byte (joined); SPIKE_PLAN item 4
  and "next to the fast checks on save"; ADR 0006 and README item-6 fragments; XP-TERM (a) in VISION_V2 (whole),
  README:183 (first clause), STATUS (whole, with "XP-TERM: (a), "); "LOOK: (a)." in STATUS; disk hygiene in
  CHECKPOINTS.md §10 (whole, from "clean finished ..." to "... stays denied.").
- STATUS queue: A4's quote, A4's addition, A4-FREEZE and the session's notes all removed; pointer to
  `docs/reports/2026-11-08-kit-v3-4.md`'s appendix present (STATUS:563-568).
- Added quotes not found in a source (`.gate/verifier-a4/quotes.py`): none of his; only the docs' own phrases.
- C12 (the report's appendix): not yet checkable; the report does not exist at ad1f9bd.

### E3 (kit tests), done
- Head ad1f9bd: routine-guard 1034/1034, settings 127/127, checkpoint 27/27 (fail 0 each).
- Main 626df67 in a detached git worktree under `.gate/verifier-a4/main-wt` (removed after): 1034/1034, 127/127,
  27/27. Equal counts. (A `git archive` copy without `.git` failed settings S4 "git switch in the main folder asks":
  the test needs a git repository; environmental, not a defect.) `.claude/hooks`, `.claude/scripts` and
  `.claude/settings.json` are byte-identical to main (`git diff --quiet`).

### E4 and E6, done
- `node tools/gate.mjs origin/main --no-smoke`: GATE PASS, exit 0; [1] 5/5 parse, [2] PASS, [3] +0 −0 (core 138 / game
  714 / editor 353), [4] 51 guarded 0 flagged, [5b]-[5d] PASS, 5a skipped (`.gate/verifier-a4/gate.txt`).
- `repo-map-check.mjs`: OK, 114 entries cover 101 required paths.
- E6: `git diff --name-only origin/main...HEAD` holds no `.claude/hooks/`, `.claude/settings*`, `.github/`, `tools/`,
  `.npmrc`, game file or page. Files outside `.checkpoints/`: 5 kit files + 9 docs. CHECKPOINTS.md (addendum, his
  DISK-WHEN (a)), START_HERE and REPO_MAP (docs-keeper drift) are beyond plan §3's list but each is approved or a
  docs-keeper duty. Line counts: CLAUDE.md 252 → 277 (+25; numstat +41 −16), advisor.md 172 → 301, SKILL.md 32 → 36,
  security-auditor.md 54 → 56, CHECKPOINTS.md 208 → 220. C13's "about 25 at most": met (+25).
