# Advisor review of PR #53 head a637c7a (KEPT-LOGS rule), day session, seat: advisor from main's advisor.md

## Plan
1. Load: main's advisor.md (done), history.md 2088-2111, 2131-2146, 2148-2161, the earlier advisor reviews
   (advisor-kept-logs-rule-full.md C1-C9/F1-F6; advisor-kl-change-full.md C1-C4, KLC-F1/F2, build_plan_changes 1-7).
2. Tier and diff: git diff --name-only/--raw --no-renames origin/main...a637c7a; modes; no .checkpoints/.
3. Equality: kept-logs-final-text.md blocks + kl-change-v2.diff == head's three kit files; check kl_build.py,
   kl_evidence.py compare, compare-head.log; reproduce one comparison myself (sha256 of the approved texts too).
4. Build vs plan: tests 1-10, M1-M14 homes and bites (re-run at least one bite), evidence (a)-(c), kit test counts vs
   main, repo-map-check, gate logs, guard check, quotes check, worktree from origin/main, nothing real packed/deleted.
5. Run the test suite myself from the worktree (safe: temp scratch repos).
6. PR body via GitHub tools (if loadable): every claim vs evidence; his three ready-report items; C8's tier-2 table;
   KIT-FREEZE-2 confirmed for this one rule; his words cited by line, not retyped.
7. Verdict + findings + conditions + to_mustafa.

## Progress
- Block 1 (load, tier): origin/main = 4460ef5; remote branch head = a637c7a (ls-remote). Diff --raw: 4 files, all
  100644 (no link, no submodule): M .claude/CHECKPOINTS.md, A kept-logs.mjs, A kept-logs.test.mjs, M docs/REPO_MAP.md;
  671 insertions, 0 deletions. No .checkpoints/ file. Risk tier 2 (.claude/). Approved texts' sha256 recomputed:
  97c8165655869d9a... and c80dbbb346459d46... (102 lines) = his words (history.md:2136, 2155).
- Block 2 (equality, my own): compare.mjs (.gate/kept-logs-rule/review/), node, positional splice of every hunk at its
  header's line with every context/removed line checked there (independent of kl_build.py's replace): cp 18->19,
  mjs 313->323 (7 hunks), test 292->326; widest 118/120/120. EQUAL to git show a637c7a for all four files (CHECKPOINTS
  = main + blank + §11; REPO_MAP = main + 2 rows after checkpoint.test.mjs row). Hashes equal compare-head.log's.
- Block 3 (my runs, worktree, env without SHAMBLE_ROLE/KEPT_LOGS_*): GNU tar 1.35, node v22.23.4. Suite: 10 entries,
  9 pass, 1 skipped (dry run alone); "ok 2 - GNU tar unpacks..." no SKIP. Planted copies M4/M11/M12/M14 differ from the
  head by exactly their one replacement (diff). Bites re-run: M14 -> only test 5 fails (actual 0, expected 1); M11 ->
  only test 8 (actual 0, expected 2); M9 -> only test 4 (actual 1, expected 2). All match evidence-1.log.
- Block 4 (gate, kit, map, nothing real moved): worktree's own full gate (NODE_PATH=main's tools/node_modules, writes
  only to the system temp): 1-4, 5b-5d PASS; 5a FAIL only because Chromium cannot start (same as gate.log). PR touches
  no legacy/, tools/ or .github/ file, so CI's green stands: gate + smoke + toolchain success on a637c7a (runs
  43748934361 PR, 43748694216 push). Kit tests at head: checkpoint 27/27, settings 127/127, routine-guard 1107/1107,
  exit 0 = base/ taps. repo-map-check OK (122 entries, 109 paths). Real folders' fingerprints now 3ae12298... and
  113c4734... = the dry run's before/after; .gate/netcode-proof-test/kept-logs/ absent; cache present, nothing newer
  than 07:20 UTC; no raw-logs.sha256 in any commit. Worktree clean at a637c7a.
- Block 5 (PR body, loaded via mcp__github__pull_request_read get): head, files table, compare-by-script, (a)(b)(c),
  M1-M14 table, kit counts, map, gate, guard, tier-2 table, KIT-FREEZE-2 line 2136: all match the evidence.
  FINDING B1 (Low): 13 links have their target wrapped in growing backtick runs ([..](`` url ``) up to 14 backticks),
  so per CommonMark the code span wins and they render as text + code, not links (history.md:2088-2111, 2095-2097,
  2105, 2136-2139, 2155-2161, kept-logs-final-text.md, compare-head.log, full.tap, dry x2, evidence-1.log, gate x2).
  Line numbers are still in the text. Fix in the ready report's body edit (no push).
  Body is still a progress body: "Results: to come", "Decisions needed: none yet" -> becomes the ready report.
  FINDING B2 (Medium): the body links compare-head.log, evidence-1.log, gate.log, gate-no-smoke.log at e10e1de, but
  .gitignore:7 (`*.log`) ignores them: e10e1de's kl-evidence/ holds only the 17 TAP files; no commit on any branch
  holds the 5 logs (git log -- ... empty). So the compare-by-script output (his item 3, history.md:2159), the M1-M14
  summary and the dry-run fingerprints (only in evidence-1.log) and the gate logs are on this container's disk only;
  Mustafa and his Advisor cannot open them. Claims themselves hold (I reproduced compare + fingerprints). TAPs, scripts,
  approved texts, history.md lines at e10e1de = local (sha256 checked). Fix: commit the logs (renamed .txt or add -f)
  on the netcode branch, or paste compare/fingerprint output verbatim into the ready report; relink.
- Block 6 (guard probe, guard-probe.mjs, read only): day session -> no decision (exit 0, empty) = PR claim; pr-advisor
  -> deny (runs no program); night-shift -> deny (script differs from main's copy: not on main yet). After merge the
  routine guard would let a routine run main's tracked script, and the script's own SHAMBLE_ROLE refusal (test 6, M3)
  stops it; security-auditor: routines cannot prefix SHAMBLE_ROLE= or env -u.
- Block 7 (reviews in): security-auditor APPROVE WITH FINDINGS, SA-KL-1..4 Low (follow-ups) + a pre-existing note
  (settings.json:50-51 allow `node .gate/*.mjs|js` unprompted in a day session: wider route around rm -rf; main, not
  #53). test-engineer APPROVE WITH FINDINGS, Lows only (KLC-F2 4th case; fingerprint no pipefail; untested fail-safe
  paths; links). Security Lows left open -> needs-mustafa (already tier 2); must be deferred in writing in the ready
  report. Fixing any changes kit text he said yes to -> his yes + KIT-FREEZE-2.
- FINDING B3 (Low): no STATUS/ROADMAP/DECISIONS update in the PR; plan deferred DECISIONS to docs-keeper (KL-5) and
  kept the PR to 4 files (his "small and in one place"); the body names DECISIONS only. Ready report should say
  STATUS/ROADMAP/DECISIONS come in docs-keeper's next pass after merge.
- FINDING B4 (Low): REPO_MAP kept-logs.mjs row (docs/REPO_MAP.md:120) does not mention the second walk; it is the
  approved text (rows not in KL-CHANGE), so no change; docs-keeper follow-up.
- Note: the 18:40 "go" for the .claude/ write (cp) is recorded on the card only (netcode-proof-test.md:586), not in
  history.md word for word. Seatbelt only; no finding on the head.

## Verdict (block 8, complete)
APPROVE WITH CONDITIONS for head a637c7a357e79614f896ddddf7488e08b5fc9180: the head equals what he said yes to
(reproduced independently), the build met C7-C9 and build_plan_changes 1-7, both reviewers APPROVE WITH FINDINGS
(Lows only), no push needed. Conditions are on the ready report and the merge, not the head:
1. Make the compare/gate/evidence logs reachable (commit them under a non-ignored name on the netcode branch, or paste
   compare-head.log and the two fingerprint lines verbatim in the body) and relink (B2).
2. Fix the 13 broken links (B1).
3. Ready report: both reviews' verdicts; SA-KL-1..4 and the test-engineer Lows each deferred in writing with where
   tracked (STATUS follow-ups by docs-keeper); the pre-existing settings.json:50-51 note; Decisions needed and For
   Mustafa filled; STATUS/ROADMAP/DECISIONS in docs-keeper's next pass (B3).
4. Merge only from a day session on his own go for a637c7a, expected head = a637c7a, gate+smoke green (they are).
   Any push = new advisor review + his go again. Nothing real packed or deleted before merge + netcode branch takes main.
Wrote only this notes file and two read-only scratch scripts under .gate/kept-logs-rule/review/.
