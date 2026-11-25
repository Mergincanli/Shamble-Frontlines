# Advisor check of MAIN-66's plan (take main c1da1bd into the netcode branch)

Advisor agent, day session, read only, effort xhigh. Rules: `git show origin/main:.claude/agents/advisor.md` (read at start).
Plan checked: .checkpoints/netcode-proof-test/main-66-plan.md, 248 lines, sha256
5d6003017b5f93458360e36b744a165481ae0526412ccd7a439bf8f5782b859e (matches the request's 5d6003017b5f9345…f5782b859e).
HEAD at start: 7e2d6f333ae2f1dce76b96ccbc341d047e4972bb (notes saves may move it; checked at the end).

## Plan of this check
1. Read the inputs: main-65-plan.md, advisor-main-65.md, advisor-main-64.md:15-48, main-61-plan.md,
   advisor-main-61.md:19-104, advisor-main-65-head.md, history.md:2055-2057, :7862, :7988-7998, the card.
2. Facts checked, line by line, with read-only git commands (rev-parse, merge-base, diff, merge-tree, grep, ls-files,
   patch-id), df, and the GitHub MCP tools for main's CI on c1da1bd.
3. The guard line (settings.json:271-292, guard.mjs:42, routine-guard.mjs:1342-1352).
4. The numbers C1-C6, M64-1, M65-2.
5. Steps, stop rules, risks (U7's T-builder, u7-run.sh:25-29/:55, launch-0850.md section 3 item 7, the saver, 5e).
6. Q1-Q4.
7. Verdict, findings, DONE.

## Notes (one line after each check)
- Read: main-65-plan.md, advisor-main-65.md (whole), advisor-main-64.md:1-60, main-61-plan.md (whole),
  advisor-main-61.md:15-109, advisor-main-65-head.md (whole), history.md:2050-2061 and :7855-8025, the card (whole).
- SHAs: origin/main (local) = ls-remote main = c1da1bd24faae21794195e2384895460753a41eb. ls-remote branch = HEAD =
  7e2d6f3 at start. Merge base HEAD/origin/main = 28254a73ab22… = 39a0d04^2. First-parent merges 28254a7..c1da1bd:
  09faefa7 (#66, 11:10:10 UTC), 3ab9eae (#68, 13:57:02), 311c048 (#69, 16:34:47), c1da1bd (#67, 17:12:59) - as stated.
- 92 paths, +11,561/-631; A 31 (+8,062), M 61 (+3,499/-631); DRCTUXB none. Without STATUS.md: 91 files +11,512/-582;
  STATUS.md 49/49. Per merge: #66 6 M + 1 A (docs); #69 routine-guard.mjs, its test, ROADMAP, STATUS + 1 report;
  #67 7 docs M (ADR 0006, DECISIONS, REPO_MAP, ROADMAP, SPIKE_PLAN, STATUS, STATUS_ARCHIVE) + 1 report; #68 code in
  crates client-endpoint/devtools/protocol/room/sim/sim-wasm, packages client/e2e/sim-worker/types, Cargo.toml,
  Cargo.lock, docs. Merge messages: #66 and #67 "from the Builder V2 chat on Mustafa's go there"; #67 names the advisor
  agent's APPROVE of 35adfbe and gate/smoke/toolchain green; #68 "on his tap of 13:56:26 UTC"; #69's message has only
  its title. All as the plan says.
- HEAD moved 7e2d6f3 -> 02ef60a during this check (a save; it took this file's first text). 2935f0a..02ef60a: 4
  commits, one parent each, touching only .checkpoints/ (the card, this file, launch-0850.md, the plan, the request).
- merge-tree HEAD origin/main (at 02ef60a): tree dc9eff48, one conflict docs/STATUS.md (content); auto-merge lines for
  DECISIONS, REPO_MAP, STATUS, STATUS_ARCHIVE. dc9eff48 vs the plan's f3a420bc: .checkpoints/ (five names) and
  docs/STATUS.md, whose only difference is the marker label (`>>>>>>> c1da1bd` in f3a420bc, made with the SHA;
  `>>>>>>> origin/main` here, as step 2's merge will print). The anchor excludes both, so it is not moved.
- Trees (shortstat vs the empty tree): 2935f0a 1,886, f3a420bc 1,917; 02ef60a 1,887, dc9eff48 1,918 (+31 both).
- STATUS.md: markers :24, :27, :31 in dc9eff48; ours :25 (#40, 2026-11-15 text), :26 (3a "none yet", as 28254a7);
  theirs :28 (#40, 2026-11-14 text), :29 (#67), :30 (#69). `git diff c1da1bd dc9eff48 -- docs/STATUS.md`: only the
  block; outside it the merged file equals main's. Main's file 120 lines; #67's numstat 10/10. Branch's own change
  since 28254a7: the #40 row only (1/1). Resolved as C6 says -> main's file with line 24 swapped: 120 lines, "1 1".
- Plan :99 history slip (Low): main's #66 row came with #66 itself (09faefa7: rows #40, 3a, #66), not "at #68"; #68
  replaced the 3a row by the 3b row; #69 took out 3b and #66 and added #69; #67 added #67 above it. No step reads it.
- Guard pathspec (main-61-plan.md:37-40): origin/main...HEAD empty; HEAD...origin/main exactly the two guard files.
  28254a7..origin/main over .claude CLAUDE.md AGENTS.md .github tools legacy .gitignore (+ globbed names): the two guard
  files only. 28254a7..HEAD over kit, code and manifests: empty.
- Guard merge rule: HEAD's routine-guard.mjs:1342-1352 and main's :1383-1393 are the same text (shifted 41 lines by
  #69): a plain `git merge origin/main` asks only if protectedHit(origin/main...HEAD) names a kit file. protectedRel
  (main :35-48) and GATE_FILES (:28-29) match the plan's pathspec (it also covers agent-memory-local: stricter).
- settings.json:271-292: PreToolUse, PermissionRequest, ConfigChange each run `node -e` importing
  ${CLAUDE_PROJECT_DIR}/.claude/hooks/guard.mjs from disk; guard.mjs:42 imports ./routine-guard.mjs. #69 changes
  neither. The plan's guard line holds.
- #69's guard change (3ab9eae..311c048): env/nice/stdbuf option words read with their values (WRAP_OPTS); a wrapper
  form it does not follow (env -C/-S, unknown option, cluster) asks in a day session and is denied for an agent. It
  only adds asks or denies. The plan's steps 2-7 put no env, nice or stdbuf in front of any command.
- git status clean at 02ef60a; .claude/worktrees/ ignored by .git/info/exclude:7; .gate/ by .gitignore:2.
- Main's CI on c1da1bd (GitHub MCP, by run id): gate run 45662377295 (push, head_branch main, attempt 1, completed
  success; jobs gate 139375346631 and smoke 139375347099, success) and toolchain run 45662377271 (push, main, attempt
  1, success; job 139375347198), 17:13:05-17:18:09 UTC. As stated. c1da1bd's commit: author date 17:12:59 UTC.
- PR #40 (18:58 UTC read): open, draft, mergeable_state "dirty", base 28254a7, head c21582d (notes saves since).
- ADR 0006 (#67): 5 hunks at default context, one the table (only the PM-0006-2 and -7 rows), four prose; -U0 shows no
  other `| PM-` line. marks.json: 81 rows point at docs/adr/, 17 at 0006 (ids -11 to -27); `^\| PM-0006-(1[1-9]|2[0-7]) \|`
  counts 17 in HEAD, origin/main and dc9eff48. check.mjs:1880-1905 finds each row by `| <id> |` and compares its
  sha256; :1887 takes expected ids from ADR 0001, which main does not change (docs/adr: only 0006 changed). The marks
  line's "drafts at <sha>" is marks.json's own drafts_commit field (judge main.rs:77-81), not a hash of the ADRs.
  So the 81 rows hash as before.
- check.mjs:2632-2641's marks step runs `cargo run -q --locked -p npt-judge` (it builds the judge), and main-run.sh
  touches every crates/ file first and every 2 s (:48-52). The shared target was emptied at 06:1x (18:5x: 1 MiB;
  launch-0850.md section 3 item 3). So the plan's "the light runs build nothing" (:216) is not right: each light run
  compiles the judge, and the first may also compile its registry deps (MAIN-65's marks step took 32.2 s on a warm
  cache). Disk is small; the time the -x lock is held grows. Low.
- SPIKE_PLAN: +5 lines in entry 6 (#66), +6 after them (#67); item 1 untouched. Spike names it in Cargo.toml:1,
  README.md:3, check.test.mjs:1565/:1574 (a fixture). Root Cargo.toml: the wasm-bindgen comment only (:14-16).
- refs: `git grep -I "spikes/"` on main's code/manifest/CI set: nothing; `-a -l` outside spikes/, docs/, .checkpoints/,
  *.md: kept-logs.mjs and its test only.
- DEP-HOLD: main's manifests are Cargo.lock, Cargo.toml, crates/{client-endpoint,devtools,room,sim-wasm}/Cargo.toml
  (no package.json or lock). `git diff --stat origin/main dc9eff48 -- Cargo.lock Cargo.toml crates packages
  package.json package-lock.json .claude .github tools legacy .gitignore`: empty.
- Whole tree: dc9eff48 equals main outside spikes/, .checkpoints/, docs/ (exit 0); docs/ differs in the six names.
  HEAD..dc9eff48 without STATUS.md and .checkpoints/: "91 files changed, 11512 insertions(+), 582 deletions(-)";
  2935f0a..f3a420bc without STATUS.md: the same line.
- Auto-merged docs: branch numstat 28254a7..HEAD = main..dc9eff48 (DECISIONS 24/1, REPO_MAP 17/0, STATUS_ARCHIVE
  18/5); HEAD..dc9eff48 = main's own numstat (689/0, 38/18, 243/7). -U0 both ways: the same lines, only positions
  moved. (patch-id not run: it needs a pipe, outside the one-command rule; the -U0 read stands for it.)
- Untracked or ignored files in main's paths (docs, .claude/hooks, crates, packages, Cargo.*): none. df 19:0x: 15,050
  MiB free.
- #69's report: "routine-guard 1247/1247 (main 1107/1107); settings 127/127; checkpoint 33/33" (:22); security-auditor
  round 2 APPROVE (:28-29); verifier FAIL V-1 then re-run PASS (:31). .claude/scripts, settings.json, guard.mjs
  unchanged 28254a7..c1da1bd (exit 0). The guard tests write only under os.tmpdir() (mkdtemp, rmSync in finally).
- #67's report: two rounds (infra-engineer REVISE F1 High then APPROVE; test-engineer APPROVE; verifier FAIL then
  PASS WITH NOTES), the advisor agent's plan approvals; merge message names its APPROVE of 35adfbe. As the plan says.
- U8's snapshot wip/u8-build.patch: 43 `diff --git` entries, all spikes/netcode-proof/ops/. U7's T-half snapshot
  wip/u7-t-build-t5i.patch: 23, all spikes/netcode-proof/web/. Both disjoint from main's 92.
- u7-run.sh: mode -s by default, -x by flag (:25-29); flock at :55; log written only after lock and idle wait
  (:66-70). main-run.sh: flock -x (:28), idle wait up to 60 min (:30-37). As the plan says.
- Saver: `pgrep -af "checkpoint.mjs watch"` at 19:06 UTC shows the watch running (bash wrapper and node), the card's
  bhe11brxq (from 18:07:38 for 110 min, ends about 19:57). Step 1 will meet a running saver: TaskStop and C5 (b).
- Card: `wc -l` 199 at 19:0x (cap 200): the resume line must replace lines (M65-3), as step 1 says.
