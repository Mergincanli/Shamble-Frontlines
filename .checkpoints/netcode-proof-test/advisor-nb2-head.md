# Advisor: NB2-MERGE step 9, the pushed head by SHA (2026-11-26)
Merge M = 48efc9c59b06347c79d0e9ecbdcc78c6992eebde (H0 22cd65d7, T 0e621dae). Pushed save S = 23e9e92d11a8bc78f7aa9d0a2862790f1e7d0736.
Branch head at launch 1fa5410. Form: MAIN-65 step 7 (advisor-main-65-head.md) and advisor-main-67-head.md.
Read only: commits by SHA; from disk only .gate/nb2-merge/ and the notes. No fetch, cargo, tests.

## My plan
1. Rules: main's advisor.md (read at NB2's plan check; re-read the head of it only if changed: `git log -1 origin/main -- .claude/agents/advisor.md`).
2. The merge by SHA: parents; tree = merge-tree(H0, T); H0..M = e2c0e896..T (shortstat, name-status); T..M on judge,
   marks.json, README, notes empty; SERIES.md numstat vs T; kit pathspec H0..M empty; marks.json sha; raw modes.
3. H0: is it 0b's head plus saves; clean-tree evidence (NB2-C1) from the main-run.sh headers; the history of
   H0's parent chain since bae6232 (U6c, 0b, U8?).
4. S and 1fa5410: S^ chain to M; M..1fa5410 names only .checkpoints/; S's message (no skip marker).
5. The record: step1-record.txt, merge-msg.txt, light-base.log, check.log, rejudge-47-*.log/.json: each bar.
6. His go (history.md:8123-8131) and the named command.
7. CI on S: read from the record/card (no fetch, no GitHub tool unless allowed) — note what I could not read.
8. Each departure 1-4: rule.
9. NB2-C1..C12 met or not.
10. Verdict, findings, DONE.

## Notes (one line per check, as I go)
