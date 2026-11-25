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
- 5e6711e..bae6232: the card, nb2-merge-plan.md, nb2-merge-request.txt only (launch note holds).
- marks.json: working tree sha256 c1935a5b… (= e2c0e896's blob f9aab0d); T's blob bd93d3d = 72651a3c's; sha256 of
  `git show T:…marks.json` c3396db2…f1b7a. Base marks.rs has no `moved` and `deny_unknown_fields` on every raw struct
  (:14,:22,:56,…): the old judge refuses the new file. All 81 drafts are in docs/adr/0001/0004/0006; this branch changed
  no docs/adr file, check.mjs, Cargo.toml or Cargo.lock since e2c0e896 (docs/ changes are DECISIONS, REPO_MAP, ROADMAP,
  STATUS, STATUS_ARCHIVE, reports: MAIN-65's).
- U7/U8 patches: u8-build.patch 43 headers, u7-t-build-t5i.patch 23; a grep of every wip/*.patch header for marks/,
  crates/judge, netcode-judge finds only older SERIES.md hunks (u4f2, u6b, u6c-build, u6c-fix); none in u7/u8. Holds.
- Builder 2's card at T read whole: counts 807/904 (base) -> 854/951 (72651a3c), self 78/78, redact 11/11 0 findings,
  132 judge tests; hand-off block says "Before the merge: the re-judge of the -47 set" (his item 4 says after: his word
  governs). plan-a.md:73 and ruling (5) narrowed the re-plan's real-data line to -47 only; "-57's folder still PASS"
  (replan-marks.md:207) was never run nor deferred by name.
- eval.rs at T: judge_mark :1737-1746 returns MOVED for a mark-level moved before parts, checks, bites; the headline
  order :1838-1849 applies only past it. main.rs: exit 0 whenever verdicts are written; 1 on malformed marks or logs.
  lib.rs: read_logs unchanged by Builder 2; summary_line `{:<12} {:<9} {first}` with control chars escaped.
  marks.rs:90-92 `moved to {to}: {why} ({by})`. marks.json:9243-9254 PM-0001-47 moved {to "Slice 1 change-set 7 (Lag)",
  why "the clock estimate is the real client's; its L1 pass here (a4cac39) stays as a record", by "NPT-TRIM, 2026-11-21
  (the re-plan, approved 2026-11-22)"}.
- crates/log b57221e..HEAD: +111/-0 in src/lib.rs only (Machine::read_local, cpu_list, a test): no record field
  changed, so the reader parses the b57221e logs as before. Neither U6c nor Builder 2 touches crates/log.
- -47 folder: measured/l1-47-s1..s3 and bites/bite-newest-sample, bite-tick-quantised, each client+lab+room .jsonl;
  du -sm 51. Headers: run = folder name, marks ["PM-0001-47"], role measured / bite, build b57221e release. So serving
  = 3 runs (measured), 2 (bites). Old verdicts: NOT-RUN both (by design of the split). runs.txt:4-12 two calls, peaks
  372 / 349 MiB projected.
- refs step (check.mjs:2625-2631) counts `git ls-files` (all tracked): N0 + 32 right. referenceProblems :1855 skips
  `.checkpoints/` wholesale: Builder 2's notes cannot fail refs. redact walks the spike folder on disk (redact-check.mjs
  :277-288, :378-385): items rise by the 10 fixtures.
- Cross-reads: SERIES.md is read by lab/contract.rs and tests (U6c's bots/tests/load.rs:287, server/src/probe_tests.rs
  :286, net/src/l2/socket_tests.rs:394: containment checks); marks.json by room/tests/bites.rs:31,
  server/tests/schedule.rs:355, check.test.mjs:1776. Builder 2 removes no SERIES.md row name (every - row returns with
  its first cell); neither side adds a name the other adds. Step 4 covers the rest.
- Guard (HEAD = main's kit, `git diff --stat HEAD origin/main -- .claude CLAUDE.md` empty): mainFolderMove
  routine-guard.mjs:1342-1346 — any merge but `--ff-only origin/main` or a one-word `origin/main` -> "only origin/main
  merges into the main folder" -> ask (:1360-1362; a subagent: deny). Step 2 WILL ask. settings.json allow: `git
  merge-base *` only; deny `git reset --hard*`, `--merge*` etc.; the guard has no `revert` rule.
- His rule 2026-11-16 21:50 (history.md:2187): after a 17:09-20:36 stall on "a call that waited for my approval": simple
  forms; "When a call has to ask me, name it in your message before you send it."
- checkpoint.mjs:196 "nothing to save" returns 0 (no push) when the card is unchanged; :108-113 onlyCheckpointsAhead
  is tree-to-tree vs the remote: the merge makes it code-ahead, no [skip ci]. Claim holds.
- Disk: `df -m` /home/user 10,246 MiB now. `.gate/netcode-proof-test/target` has no debug folder (u4r-disk-plan.md §5's
  clean is spent). The live part-clean: card :193-195 (flock on shared-target's lane lock, cargo clean --profile dev
  --target-dir shared-target, then tests and df) = advisor-u6b-m6.md:162-163. It runs outside main-run.sh, so it works
  under main-run.sh's 4 GiB floor (main-run.sh:38-47 exits 3 before any command).
- U7 0b: advisor-u7-brief.md:219-220 "Block 0b's binding check runs the hour U6c merges"; its items run cargo check,
  deny, audit, check.mjs counts (lane + main folder). advisor-main-65.md:159-160: "a list of commands, not an agent".
- -57: kept raw sets u4r-P2-8ee453c22882 741 MB, u4r-P2-fc411116c1f1 623, u4r-P3-d66e23cd78f2 298,
  u4r-P3-fd4612050cd6 291; P0's not kept. No folder named by the re-plan line.
