# Advisor: NB2-MERGE plan check (2026-11-25, started 03:5x UTC)

Brief: .checkpoints/netcode-proof-test/nb2-merge-request.txt, sha256 df3f2576cd9dea5a5a432a75d6ed6d43d7384ce441af9d0aa211af6cd3b2acb6 (checked: match).
Plan: .checkpoints/netcode-proof-test/nb2-merge-plan.md, sha256 2b7fb042c1fff30b57a64de15f4e2fa4045918dba7d7ac5d4ec1d9613fcde479 (checked: match).
Rules: `git show origin/main:.claude/agents/advisor.md` (read).
HEAD at launch: bae62327fc89be8f9c6e479ace44062a29f5c2a4 on claude/netcode-proof-test-plan-hcky5g; working tree holds U6c's
merge in progress (uncommitted; not part of this check). Builder 2's tip T = 0e621dae6e5b009a4cf00b6401b46d771b2215a3.
Seat: read only. No cargo, test, gate, light run or plant; one simple command per Bash call; no web.
(Deviation, honest: my very first Bash call was compound: `cd … && sha256sum … && git rev-parse HEAD && git branch
--show-current && git status --short | head`, read only. Every later call is one simple command.)

## My plan
1. Facts checked, line by line (git show, diff, rev-list, merge-base, merge-tree --write-tree, ls, du).
   - his condition: every commit in 72651a3c..T touches only Builder 2's notes.
   - guard pathspec (main-61-plan.md:37-40) over e2c0e896..T.
   - trial merge on HEAD (bae6232) and on a stand-in for the post-U6c head where possible (read only).
   - SERIES.md overlap (Builder 2's hunks vs U6c's 51 rows in the working tree).
   - -47 logs folder (ls, du).
   - marks.json shas, U7/U8 patch file lists, counts.
2. Steps vs his item 4 (history.md:7909): order, his two checks in his order, the bars.
3. Re-judge: crates/judge/src/eval.rs at T: early return for mark-level moved, headline order, headline text, exit code.
4. Stop rules, risks, tier, the review line of his 2026-11-15 rule (history.md:2127).
5. Q1, Q2, step 5's disk fallback, -57 re-judge.
6. Verdict, findings on CLAUDE.md's scale, conditions, DONE.

## Notes (one line per check, as I go)
- His words: history.md:7909 read; heading line 7902 "Mustafa, 2026-11-25 00:07 UTC, in this chat … (word for word)". The
  plan's four quoted lines match item 4. His 2026-11-15 rule: history.md:2127 "every change to marks.json or to the judge
  gets a reviewer's read before its unit closes, and your report to me names it in its own line with what covered it."
- T object present (`git cat-file -t` commit). 72651a3c is an ancestor (exit 0). `rev-list --count` 5. The five, linear,
  one parent each: f54422b, ff6c84d, 9325873 (each M .checkpoints/netcode-judge.md only), 71b32bf (M netcode-judge.md, A
  netcode-judge/mustafa-2026-11-25.md, M netcode-judge/save-message.mjs), 0e621da (M netcode-judge.md). `diff --raw`
  72651a3c..T: 3 files, all 100644; `--stat` 3 files +96/-60. HIS CONDITION HOLDS at T.
- Merge base HEAD (bae6232) and T: e2c0e896dedd5c09c2536e2fbe9323353756fe99. origin's netcode branch = bae6232 (= HEAD).
- e2c0e896..T: `--name-status --no-renames` 46 paths: 22 notes (all A), 24 spike (fixtures/README.md M, 10 fixture logs
  A, src 7 M, tests 3 M, marks README/SERIES/marks.json M) = 32 A, 14 M, 0 D/R. `--shortstat` 46, +6,951/-208; spike part
  24, +4,712/-208. `--raw`: every path 100644 (no symlink, submodule or executable bit).
- Guard pathspec (main-61-plan.md:37-40) over e2c0e896..T: empty. Extra pathspec (tools, **/Cargo.toml, Cargo.lock,
  package*.json, rust-toolchain*, .cargo/**, build.rs): empty. `git check-ignore --no-index` on 36 of the paths (all 22
  notes, the 11 fixtures/README, eval.rs, marks.json, SERIES.md): exit 1, none ignored.
- This branch e2c0e896..HEAD under crates/judge, marks, .checkpoints/netcode-judge*: empty (74 commits). Working tree
  (U6c, uncommitted) in the same places: SERIES.md +51 only. Full `git status --untracked-files=all`: 12 M + 12 new spike
  files = U6c's 24, plus the card; none under crates/judge, marks.json, marks/README.md, netcode-judge*. Only shared path:
  SERIES.md.
- SERIES.md: Builder 2's hunks at base 262-264, 267-272, 278-279, 282-285, 287-288, 290, 293-294, 297, 402, 407, 418-419
  (+421); U6c's one insert after base 908 (new 909-959, 51 lines; 31 data rows + header + separator = the plan's "33").
  Both on base blob 665ab3f. ~490 lines apart: no overlapping or adjacent hunk. Builder 2's 25 row names (21 metric rows,
  the "(19)"->"(21)" heading, relevant_count, admit/session_end, iface, room_end) share none with U6c's probe_*,
  header.params.*, steal_cpus, udp_*, snmp*, cpu_* names (read by eye from both -U0 diffs; I ran no comm pipe).
- Trial merge `git merge-tree --write-tree --name-only HEAD T` at bae6232: one line, tree 8012ac0db8cd…, exit 0, no
  conflict. (Plan's cbb0d520 was at 5e6711e; three notes-only commits since: a different tree id is expected.)
