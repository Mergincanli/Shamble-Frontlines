# Advisor review: PR #72 head 6c8127005a8e805429571ec243632609fc143f1a (merge of main 2483e9a7)
Rules: main's .claude/agents/advisor.md (read via git show origin/main:...).
## Plan
1. file lists: dc00c79a..6c812700 vs 07d673b0..2483e9a7; 2483e9a7..6c812700 vs 07d673b0..dc00c79a
2. git show --cc on the 4 docs; verify70.py run; DECISIONS order
3. protected paths; five sha256
4. commit message counts
5. CI check runs
6. notes-only head question
## Findings (one line each, as I go)
- 1 OK: merge base 07d673b0; dc00..6c81 file list == 07d6..2483 (39 files); 2483..6c81 == 07d6..dc00 (62 files). Non-conflict files equal one side exactly.
- 3 OK: raw diff vs main: all modes 100644 (no symlink/submodule); no .claude/, CLAUDE.md, gate file; .github/ files not in dc00..6c81 list (equal dc00c79a); five sha256 match at 6c812700.
- 2 OK: side copies in scratch m70 equal git blobs (git hash-object == ls-tree at 07d673b0, dc00c79a, 2483e9a7). verify70.py (run by me): PASS; DECISIONS 138/145(+7)/143(+5)/150, by entry, each side's order, newest first.
- 2 OK: git diff 2483e9a7 6c812700: DECISIONS +150/-0, REPO_MAP +6/-0 (pure insertions: lane 1's lines and order untouched). ROADMAP/STATUS: the only main lines removed are lane 2's own (Next 3, "Lane 2's #67" bullet, PHC-HEADER line) and the Part 15 entry (moved byte for byte to Build history after change-set 0's entry, line 210).
- STATUS: both "Last updated" lines kept byte for byte, ours (11-26) above main's (11-25); lane 2's own Waiting bullet joined (one space, no word changed); 120 lines. Right reading of item 6b: main's line is lane 1's text; dropping it would break "the other lane's lines byte for byte". Same pattern as 35adfbe (approved, merged as #67).
- OLD-LINES (a) (DECISIONS.md:2160): "for lines about merged pull requests. A docs pass of either lane may move those lines by script, byte for byte ... Its report lists each line it moved." Part 15 = PR #57, merged; moved by script, byte for byte; ROADMAP.md:5 rule "keep ~8-12". A merge commit's resolution is this lane's docs pass on its branch; precedents 35adfbe, 5e02ee7. Allowed.
- FINDING Low (L1): OLD-LINES' condition "Its report lists each line it moved": the merge message lists it, but docs/reports/2026-11-26-step-2-6-previews.md does not yet (grep: no "Part 15", no "2483e9a7"); 5e02ee7's precedent put it in the report. Add at the next docs head (with STATUS #72 row / ROADMAP entry noting the takeover and this merge).
- Info (not this PR's): main's own #70 row in STATUS and ROADMAP entry still say "in flight"/"merges on his go" though #70 merged; lane 1's lines, left byte for byte as required.
- 4 OK: 138/145/143/150; 39 and 62 files (shortstat); repo-map-check (node, main-identical .claude): "repo map OK: 222 entries cover 114 required paths"; Recent work 12; STATUS 120. Not re-run by me: 175 tests, gate --no-smoke (CI ran gate+smoke).
- 5 CI on 6c812700 (17 runs): gate x2 success, smoke x2 success, preview/tests success; toolchain x2 in_progress; wall-test pr-plain, pr-no-deployment-record, push-to-branch failure, annotations "not allowed to deploy to preview due to environment protection rules" (by design); deploy/check/after/reconcile skipped.
