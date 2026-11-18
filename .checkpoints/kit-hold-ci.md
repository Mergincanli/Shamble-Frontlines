# Checkpoint: kit-hold-ci
- **Goal:** one small pull request with three things and nothing else: the checkpoint script holds a push that changes
  a manifest or a lockfile until the security check of that change has answered (his DEP-HOLD (a), an exception to
  the kit freeze for this one change), and the two CI edits in `.github/workflows/toolchain.yml` (CI-TYPES: a type
  check and a regenerate-and-compare step; CI-CUT part 1: no cache in the toolchain job), each with the exact text
  of the report of change-set 1 at head 1bfe462, section "The exact texts asked for". His words (2026-11-18,
  15:38:13 UTC, his label 16:24 Amsterdam; `kit-hold-ci/mustafa-2026-11-18-1624-merge-58.md`, by script).
- **Branch:** claude/wonderful-hamilton-jdmlnf (stands in for cs/kit-hold-ci) · **Base:** main
  9b4b518f520aa2360a8007dc857ba74b343c8a40 (the merge of #58; the branch was fast-forwarded to it and pushed)
- **Last update:** 2026-11-18 19:20 CET (from the clock)

## What his words ask (read the file; these are pointers, not his words)
- DEP-HOLD (a): "Show me its exact text first, with how the hold is released. Keep it to that: no guard change and no
  longer watch list now." The CI check of option (c) goes on the list for the quality gate v2's plan.
- CI-TYPES (a): the exact text as in the report at that head. "Its pull request shows two things from a real run: the
  compare step failing once on a planted change, and passing on the clean tree."
- CI-CUT (a): part 1 now with the exact text; part 2 waits for the gate v2's workflow. "Its pull request shows "Cache
  mode: none" in the job's log."
- LANE-ISSUE (a): this chat opens the issue. "A comment there is never my word." WATCH-58 (b): no.
- After the merge: "One small pull request with the kit hold and the two CI edits, nothing else in it. Its ready
  report goes to the Advisor before my go." "You may write change-set 2's plan beside it. No new library of
  change-set 2 is committed before that pull request is merged."
- What he accepted with the merge of #58: the two misses as written; the size 940; round 2's Lows as follow-ups ("The
  two doc lines that claim more than the code does are corrected in the next pull request that touches those
  files."); what was not verified.

## Steps done
1. Pull request #58 merged on his go of 15:38:13 UTC: merge commit 9b4b518, 2026-11-18 15:39:28 UTC, parents 22c6afa
   and 1bfe462, tree equal to the approved head, his words in its message byte for byte (2,618 bytes, by script).
   Before the merge: the head read again (1bfe462) and its six check runs (gate, smoke, toolchain, for the push and
   the pull request): all success. After it: the branch fast-forwarded to main and pushed.
2. Session check at 15:38 UTC (get_session): model claude-fable-5-2 (configured and last served); context 671,689 of
   1,000,000 tokens (automatic compaction at 901,600): ask him to compact at a natural stop.

## Steps done (continued)
3. The "Lane requests" issue is open: #59 (15:41 UTC; body: what it is for, only comments from the `Mergincanli`
   account count, a comment there is never his word).
4. The three edits exist as scratch copies under `.gate/kit-hold-ci/` (gitignored; **no kit, gate or CI file is edited
   yet**) and as diffs in the notes: `kit-hold-ci/kit-edit.diff` (`checkpoint.mjs` +36 lines changed, its tests +87,
   one bullet in `CHECKPOINTS.md` section 3) and `kit-hold-ci/ci-edit.diff` (the workflow; built by script from the
   report at 1bfe462, blocks 1 to 3 of "The exact texts asked for"; `cache-mode: none` after `timeout-minutes: 15`;
   the scratch YAML parses).
   - The hold: `save` commits locally, does not push, names the files and exits 1 when the commits to push change
     `Cargo.toml`, `Cargo.lock`, `package.json`, `package-lock.json` or `npm-shrinkwrap.json` (any folder) against the
     remote branch (main's merge-base for a branch not on the remote). Released by
     `.checkpoints/<slug>/dependency-check.md` in the commit to push: one line per file, `<sha256>  <path>` as
     `sha256sum` prints it (`deleted  <path>` for a removed file).
   - Proven on the scratch copies: `node --test .gate/kit-hold-ci/checkpoint.test.mjs` 30 pass; the same test file
     against today's script (`CHECKPOINT_SCRIPT=$PWD/.claude/scripts/checkpoint.mjs`): 27 pass, the 3 new fail; the
     kit's own 27 tests against the edited script: 27 pass.
5. The plan is written (`kit-hold-ci/plan.md`): risk tier 2; the evidence he asked for (the compare step failing on
   a planted change: a generated file committed with the executable bit, because a content change fails one line
   earlier; the clean tree passing; "Cache mode: none" in the job's log; stop if `cache-mode` does not parse); the
   reading of "nothing else in it" (the report, the DECISIONS entry and the docs pass for #58's merge ride along);
   change-set 2's plan drafted beside it into this notes folder.

6. The advisor agent on the plan (15:46 to 16:07 UTC by its clock; `kit-hold-ci/advisor-plan.md`): APPROVE on eight
   conditions; no Critical or High; three Mediums, all wording (the five names are a reading; a manifest edit with no
   dependency change still takes a security answer; the stop rule covers every new CI line). It replayed today's
   miss in a scratch clone: the edited script held it, today's script pushed it. All taken into `plan.md` (its last
   section). Folded into the kit text beyond its conditions: a link or submodule named like a lockfile is held; the
   hold is computed for every save (so a lockfile moved into the notes folder is held, and the background saver's
   push is held too); a test that the record is read from the commit to push. The scratch copies were rebuilt:
   `node --test .gate/kit-hold-ci/checkpoint.test.mjs` 31 pass; against today's script 27 pass and the 4 new fail;
   the kit's own 27 tests against the edited script pass. `kit-edit.diff` regenerated (script +40 -1, tests +120 -1,
   rules +8); `ci-edit.diff` unchanged (+13). Size 181 lines against 150 planned (stop line 300).

7. The advisor agent on the revised kit diff (its line is appended to `kit-hold-ci/advisor-plan.md`): REVISE, one
   step of the first new test never reached the hold (the flagged record was the save's only change, so the script
   stopped at "nothing else to save"); "with that fix, and the check shown failing, it is APPROVE with no further
   review". Fixed: the step now changes a second note and asserts the hold's line. Two short tests added on its
   optional Low (the hold in front of the background saver's push; a lockfile moved into the notes under another
   name, which pins `--no-renames`). Run on the scratch copies (16:22 UTC):
   - `node --test .gate/kit-hold-ci/checkpoint.test.mjs`: 32 pass (27 existing, 5 new);
   - the same file against today's script: 27 pass, the 5 new fail;
   - against four broken copies of the edited script (`.gate/kit-hold-ci/broken-*.mjs`): the record read from the
     working tree: the first new test fails (the check the advisor agent asked for); renames followed: 2 fail; the
     hold in front of the foreground push only: 2 fail; the hold removed: 5 fail;
   - the kit's own 27 tests against the edited script: 27 pass.
   `kit-edit.diff` regenerated: by lines of `diff -u`: the script +41 -1, its tests +157 -1, the rules +8; the workflow
   +13 (`ci-edit.diff`, unchanged): 219 added lines against 150 planned (stop line 300).

8. His answer to KIT-TEXT (2026-11-18, 16:39:23 UTC; his label 17:30 Amsterdam;
   `kit-hold-ci/mustafa-2026-11-18-1730-kit-text.md`, by script, 1,542 bytes): "(b), yes, with one change." The
   change, in his words: "a manifest or lockfile is not held when its content in the push equals its content on main.
   That content was merged with my go, so a branch that takes main in is not stopped by it. Say so in the rule line
   too. Everything else stays exactly as in that file." Also: a test for it, shown failing on a broken copy; the
   advisor agent reads the changed lines once; then write the four files; the ready report shows the changed lines
   ("I do not need to see them before you write"). The planted change: yes as described. "Nothing else in it": the
   reading is right. The stop rule for the new CI lines stands.
   - His three checks after the compaction (16:39 to 16:41 UTC): the scratch copies gave `kit-edit.diff` (sha256
     70756f3d...6ce5, 262 lines) and `ci-edit.diff` (9a3b2945...) byte for byte; nothing in `.claude/` or `.github/`
     edited; the saver ran. Session (get_session, 16:40 UTC): model claude-fable-5-2 (configured and last served);
     permission mode "default" (every edit asks him; not auto); context 120,520 of 1,000,000.
9. The one change, on the scratch copies only (16:44 to 16:49 UTC), added lines only, no existing line touched:
   - `checkpoint.mjs` +5: two comment lines in the header, one in the function's comment, and two code lines after
     the plain-file check: the file's entry on `refs/remotes/origin/main` is read, and a plain file there with the
     same blob id is not held. Read as his words say: content (the blob), not the file mode; a link or submodule in
     the push stays held whatever main holds; a file removed in the push stays held until `deleted  <path>` is
     recorded, also when main removed it too (no content to compare: his words speak of content; told in the report).
   - `CHECKPOINTS.md` +2 (the rule line says it); `checkpoint.test.mjs` +46 (one new test, number 33, three steps).
   - Run: the scratch tests against the scratch script 33 pass; against the text he saw (`broken-main-rule-missing`)
     the new test fails at step 1 (held); against `broken-main-any-content` (any plain file that main also has is
     released) at step 2; against `broken-main-before-plain` (the comparison before the plain-file check) at step 3;
     against today's kit script 27 pass and the 6 new fail; the kit's own 27 tests against the scratch script pass.
   - Notes: `kit-edit.diff` is now the final text (314 lines, sha256 425731d7...); `kit-edit-as-shown.diff` is the
     text he answered on (70756f3d...); `kit-change.diff` is what changed between them (90 lines, 367137856...;
     52 added lines, none removed). The scratch copies as he saw them: `.gate/kit-hold-ci/as-shown/`.
   - Size by lines of `diff -u`: the script +46, its tests +203, the rules +10, the workflow +13: 272 added lines
     against 150 planned (stop line 300).

   - The planted change tried locally first (16:51 to 16:52 UTC), in a worktree of its own (`.gate/plant-check`,
     its own build folder `.gate/plant-check-target`; a local commit there, never pushed; the worktree is removed):
     `packages/types/src/generated/protocol/Join.ts` committed with the executable bit; the plain
     `cargo test --workspace --locked` passed; write mode passed; the compare step's two lines then printed
     "writing the generated types again changed:" and " M packages/types/src/generated/protocol/Join.ts" (the step
     would exit 1 there). So the plant is that file.
   - A refused command and the route taken (16:51 UTC): the guard refused `git -c user.name=... commit` in that
     worktree ("no option before the subcommand"); route: a plain `git commit` with the repository's own identity.
   - `kit-hold-ci/decisions.mjs` is written (not run yet): it will put his two messages of today into
     `docs/DECISIONS.md` in the docs pass, from the two files saved by script, with its checks.

10. The advisor agent read the changed lines once (16:51 to about 17:10 UTC; its notes: `kit-hold-ci/advisor-plan.md`,
    "The one change after his KIT-TEXT answer"): **REVISE, one Medium**: git reads a path that starts with ":" as a
    search pattern, so my new line compared a lockfile in a root folder named ":" with the root file on main and let
    it through with no record (the text he saw held it). Its exact fix: the new line reads main with
    `--literal-pathspecs`; and a fourth step for test 33 (21 lines, its text). "Once done and shown, APPROVE without
    another read from me." Its rulings on my readings: main as last fetched: inside his words, keep, one line in the
    report (a clone that has not fetched can release content main had at its last fetch); content is the blob, the
    file mode is not compared, a link stays held: inside, keep; a file removed in the push stays held also when main
    removed it too: inside his literal words, ask him as a decision of its own (its recommendation there: also pass
    it); naming him in the comment: keep.
    - Its Lows (report lines and STATUS follow-ups, no change here): the line he approved that reads the file from the
      push has the same pattern reading, in the safe direction (a record for a ":" path never matches; "Everything else
      stays exactly": not changed); the rule line says "a manifest or lockfile" while the code passes plain files
      only (stricter than the words); no test pins `origin/main` over the local `main` (by code reading and its
      probe; a test would pass the stop line); the count method (below).
    - Its note for the report: his reason holds for every manifest on main today (#2, #5, #48, #58); the PR Advisor's
      merge allow-list would let it merge a manifest under `docs/` or a Cargo file under `tools/` without his go
      (none exists; no guard change here). A link named like a manifest on main would stop every branch that takes
      main in (safe; none exists).
11. Both required changes applied on the scratch copies and shown (17:12 to 17:13 UTC): the scratch tests against
    the scratch script 33 pass; test 33 against the copy before the fix (`broken-main-pathspec.mjs`) fails at step
    4's status line (:652, the push went out); against the three other broken copies it still fails at steps 1, 2
    and 3 (:610, :620, :632); against today's kit script 27 pass and 6 fail; the kit's own 27 against the scratch
    script pass. So the advisor agent's condition is met: APPROVE.
    - Notes: `kit-edit.diff` is the final text (335 lines, sha256 fec3dcc5...); `kit-change.diff` is what changed
      against the text he answered on (111 lines, 327f9106...; 73 added lines, none removed: script 5, rules 2,
      tests 66); `kit-edit-as-shown.diff` is the text he answered on (70756f3d...); `ci-edit.diff` unchanged.
    - **Size, one method (`git diff --no-index --numstat`, added lines):** the final text: script 46, tests 223, rules
      10, workflow 12: **291** against 150 planned (stop line 300). The text he saw was 41 + 157 + 8 + 12 = 218 (I
      told him 219: the workflow was counted as 13; it is 12).
    - The kit's test counts before the write (16:54 UTC): routine-guard 1107 pass; settings 127 pass; kept-logs 10
      tests, 9 pass, 0 fail; checkpoint 27 pass.

12. **The four files are written** (17:15 to 17:16 UTC; one copy command per file from the scratch copies; the
    chat in "default" mode): `.claude/scripts/checkpoint.mjs`, `.claude/scripts/checkpoint.test.mjs`,
    `.claude/CHECKPOINTS.md`, `.github/workflows/toolchain.yml`. Checked (17:16:30): exactly these four files
    changed; each equals its scratch copy; their diff against main equals `kit-edit.diff` and `ci-edit.diff` hunk for
    hunk; numstat 46+223+10+12 = 291 added, 2 removed; the workflow parses (`cache-mode: none` on the job).
    - After the write (17:16:37 to 17:17:49): checkpoint tests 33 pass; routine-guard 1107 pass; settings 127 pass;
      kept-logs 10 tests, 9 pass, 0 fail (all three equal to before the write); `node tools/gate.mjs --no-smoke`
      PASS (5a skipped); the repository map check OK (139 entries, 109 paths).
    - Commits by path: 8bf4bee (the three kit files), 94c197f (the workflow). Pushed 17:18:16 UTC; head
      94c197fb0b9675251bd7d18ba1084f6b81df728c.
    - The saver was stopped (bzuzw7uzy) and started again on the branch's script: task **bhcevffhf** (17:18 UTC,
      2-hour limit: restart by about 19:15 UTC).
13. The draft pull request is open: **#60** (about 17:18:55 UTC).
14. **The clean run of the new CI lines** on 94c197f: `toolchain` for the push: run 44357176860, job 134988309631,
    17:18:27 to 17:19:18 UTC, success; for the pull request: run 44357282401, 17:18:56 to 17:19:41, success. In the
    push job's log (read whole, 581 lines): "Cache mode: none" at 17:18:27.9099464Z; inside the cut the plain test
    run, then the write-mode run (all tests again, 17:19:05.42), no "writing the generated types again changed:"
    line, the release tests, the wasm build, `npm ci --offline`, the workspace tests, "Version 6.0.3" (tsc), then
    1.04 s with no output (the two type-check runs print nothing when they pass) and the step ends in success. **The
    stop rule is not triggered.**
15. **The plant**: commit 0c0e27194bead3be4dea394b9d1b6c2c19532e74 (`packages/types/src/generated/protocol/Join.ts`
    mode 100644 to 100755, content unchanged), pushed 17:20:38 UTC. The `gate` run on 94c197f that was still running
    is cancelled by this push (the workflow's own rule).

16. **The plant's runs, both failed at the compare step** (read 17:21 UTC): `toolchain` for the push: run
    44357616602, job 134989872637: after the write-mode run's last test lines, "writing the generated types again
    changed:" (17:21:20.1392095Z), " M packages/types/src/generated/protocol/Join.ts" (17:21:20.1392577Z),
    "##[error]Process completed with exit code 1." (17:21:20.1413830Z); for the pull request: run 44357633651, job
    134989932210: step 7 "Build and test with the network cut off" failure, 17:21:17 to 17:21:21, job ended
    17:21:24.
17. **The take-back**: commit 415bd73b12cf8e9b2efa76e872956ab237320314 (the bit off again; the code tree equals
    94c197f), pushed 17:21:55 UTC, after both plant runs had ended. Its runs, all success: `toolchain` push
    44357864275 (17:22:01 to 17:22:45) and pull request 44357881263 (17:22:06 to 17:22:50); `gate` push 44357864589
    (to 17:23:10) and pull request 44357881162 (to 17:31:27).
    - The saver on the branch's new script saved and pushed twice by itself (3c34755 at 17:28, 643f915 at 17:38).
18. **A hole in my changed lines, found after the push** (17:21 to 17:45 UTC). The automated review of the push
    (Claude Code's security plugin, not his word) flagged the line that reads the file from the push
    (`git('ls-tree', rev, '--', f)`, a line of the text he approved): git reads a path that starts with ":" as a
    pattern. The advisor agent had rated that line Low (safe direction). My probe
    (scratchpad `probe2/probe.mjs`, a scratch repository) shows it is not only safe-direction once my new rule
    stands beside it: when main already holds a manifest in a root folder named ":" with the same content as the
    root file, and the push changes the file in that folder, the committed script **releases it with no record**
    (the text he saw held it). Main holds no such folder, so nothing is exposed today.
    - The complete fix, on scratch copies only (`.gate/kit-hold-ci/fix2/`; exact text: `kit-hold-ci/kit-fix2.diff`,
      45 lines, sha256 9197f070...): my two new code lines become three and read BOTH sides at the literal path
      (script +3 -2); step 4 of test 33 now builds exactly that case (tests +7 -6). No line of the text he saw
      changes. Run 17:44 to 17:45 UTC: 33 pass; test 33 fails at step 4 (:653) on the committed script (kept as
      `broken-main-push-read-as-pattern.mjs`) and on the copy before the advisor agent's fix, and still at steps 1,
      2, 3 on the three older broken copies; the committed tests pass against the fixed script; the probe: held.
      Size with it: 47 + 224 + 10 + 12 = 293 (stop line 300).
    - It is a kit edit, so it is asked of him in chat with its exact text (question KIT-FIX); not written before
      his yes.

19. **His answers** (2026-11-18, 18:01:08 UTC; his label 18:52 Amsterdam;
    `kit-hold-ci/mustafa-2026-11-18-1852-kit-fix.md`, by script, 779 bytes): KIT-FIX "(a), yes, as shown: the text of
    kit-fix2.diff as it stands in your notes at commit f7e232fb (45 lines, sha256 9197f070...). Write it into the two
    kit files. With this fix the kit lines are final for this pull request. A later finding on them that is not
    Critical or High becomes a follow-up." KIT-REMOVED "(a). A manifest that main removed stays held. Put the
    question on the list for the quality gate v2's plan, together with the line of the approved text that still reads
    a path beginning with ":" as a pattern." WATCH-60 "(b), no." "The size as you count it, 293 added lines against
    the stop line of 300, is accepted."
20. **The fix is written** (18:01 UTC; two copy commands, each asked him): the two files equal
    `.gate/kit-hold-ci/fix2/`; their diff against the pushed head equals `kit-fix2.diff` hunk for hunk (checked
    against the file at f7e232fb: 45 lines, sha256 9197f070...). Against main now: 47 + 224 + 10 + 12 = **293** added,
    2 removed. After it (18:01:45 to 18:02:58): checkpoint tests 33 pass; routine-guard 1107; settings 127;
    kept-logs 10 (9 pass, 0 fail); gate PASS (--no-smoke); map OK; the probe on the written script: held.
    Commit 4877599a9b85cfcc10b147a69e44a5276df5837e, pushed 18:03:10 UTC.
    - The saver stopped (bhcevffhf) and started again on the fixed script: task **b4bg0wkfu** (18:04 UTC; 2-hour
      limit: restart by about 20:00 UTC).

21. CI on 4877599: `toolchain` push run 44365431438 (18:03:14 to 18:03:53) and pull request run 44365446199
    (18:03:18 to 18:04:00): success. (`gate` runs: read later.)
22. **infra-engineer, review 1 of the workflow edit** (17:47 to 18:07 UTC; `kit-hold-ci/infra-engineer-review1.md`):
    APPROVE, no finding above Low. The text is byte for byte the report's blocks 1 to 3. Its six Lows (report lines
    or the gate v2's list, no change now): (1) if a commit moves `packages/types/src/generated`, the compare checks
    nothing (git status on a path that matches nothing is silent): a folder guard for the gate v2; (2) both new
    checks guard against honest mistakes, not a hostile dependency (dependency code runs earlier in the same step
    as the same user): the report must not present them as a supply-chain control; (3) the executable bit is the
    only difference that can reach the compare step today (the plain run fails first on content, a missing, extra or
    non-plain file; write mode rewrites with 0644), so the plant is the one reachable case; the untracked, deleted
    and new-subfolder cases rest on git's documented output, shown on a scratch repository, not by a CI run;
    (4) "Cache mode: none" proves the mode the runner received, not that the cache service refuses anything (no step
    tries a cache operation; setup-node's npm cache only turns on with a `packageManager` field, which the repository
    has not); what stays open until part 2: the post steps of checkout and setup-node still run after the cut with the
    network and the read-only token; (5) an empty file list stops the step with exit 1 and no message; (6) name the
    job-log endpoint as the source of the quoted log lines. Its gate v2 lines: CI-CUT part 2 (the job that runs the
    cut has no `uses:` step; plain git checkout of the exact commit; a gate test fails if it gains one); the
    lockfile check (first in every job that fetches, on push too; fails unless every added or changed lockfile entry
    is named in a committed review record; reads files only); any workflow linter must accept `cache-mode`.
    Sources it cites: docs.github.com workflow syntax and dependency caching, GitHub's changelog of 2026-09-10,
    TypeScript 6.0 release notes and tsc options (all read 2026-11-18).
    - `kit-edit.diff` (main to the final kit files, 337 lines) and `kit-change.diff` (the text he answered on to
      the final files, 113 lines: script 6, tests 67, rules 2 added; none removed) regenerated at 18:07 UTC.
    - `kit-hold-ci/decisions.mjs` now holds his three messages (not run yet).

23. engine-architect's draft of change-set 2's plan is written (17:20 to about 18:09 UTC):
    `kit-hold-ci/cs2-plan-draft.md` (777 lines) and its notes; nothing built, no library fetched. Its summary: the
    Rust simulation as WebAssembly in one Worker at 30 ticks a second, one block per tick to the page, an empty lit
    ground with a 1 m grid, the backend named, the small server for Brave. New libraries: wasm-bindgen =0.2.130 (and
    its command-line tool), three 0.187.0, @types/three 0.187.0, vite 8.3.7 declared directly. Planned size about
    2,900 lines (the build plan says 2,000). Open questions for Mustafa: RNG-CRATE, OVERFLOW-3P, CI-WASM, SIZE-2;
    twelve points where the build plan is wrong or out of date. NOT reviewed yet: it goes to the advisor agent
    after #60 is merged, and is taken up from the last checkpoint commit before the deletion.
24. The replay of change-set 1's miss against the final script (18:12:10 to 18:12:16 UTC; `kit-hold-ci/replay-miss.sh`,
    results in `replay-miss-results.txt`): the final script holds it (exit 1, names `Cargo.lock, Cargo.toml,
    crates/protocol/Cargo.toml`, nothing pushed); main's script pushes it (exit 0). The probe of the fixed hole is
    kept as `kit-hold-ci/probe-colon-folder.mjs`.
    - A slip (18:11:32 UTC): the first try of the replay script made its scratch clone inside the repository's
      folder (`run-final-GbS2`, untracked); removed 20 seconds later; nothing committed.
25. The report's draft is written: `kit-hold-ci/report-draft.md` (the parts that wait for the reviews are marked
    PENDING). The questions were put to him at 16:23:55 UTC (KIT-TEXT) and 17:48:13 UTC (KIT-FIX, KIT-REMOVED,
    WATCH-60), by the session's record.

## Current step
26. (18:20 UTC) Running: security-auditor (since 18:04), test-engineer (since 18:09), the verifier (since 18:19; on
    the code, the evidence and the draft's finished sections). When they are in: follow-ups into the draft (a High
    or Critical on the kit lines would need a fix and his word; below that: follow-ups, his word of 18:01); then
    docs (main is still 9b4b518: fetched 18:08; DECISIONS by `decisions.mjs`; docs-keeper for STATUS, ROADMAP, the
    archive, the map), the report into `docs/reports/2026-11-18-kit-hold-ci.md`, the advisor agent on the head, the
    checkpoint's deletion in the last commit (no skip mark), CI on that head, the pull request ready with the label
    needs-mustafa.

## Next steps
4. Build (three edits), the script's tests, the real CI runs (a planted change that the compare step fails on; the
   clean tree; "Cache mode: none" in the job's log), reviews (risk tier 2: the advisor agent and security-auditor;
   infra-engineer for the workflow), the verifier, the docs pass (DECISIONS by script: his go on #58 and his five
   answers; STATUS, ROADMAP, the map for #58's merge and this change), the report, the pull request; its ready
   report goes to the Advisor before his go.
5. Change-set 2's plan may be written beside it (no new library committed before this pull request is merged).

## Open findings
- From #58 (STATUS has them): round 2's Lows; the two doc lines to correct with the next change to those files.
- The gate v2's plan list gains: a CI check that refuses to build when the lockfile gains an entry with no recorded
  review (his words on DEP-HOLD's option (c)); CI-CUT part 2.

## Running work
- saver: background task b4bg0wkfu, started 18:04 UTC on 2026-11-18 on the fixed script (2-hour limit).
- agents: security-auditor (review 1, since 18:04 UTC); test-engineer (review 1, since 18:09 UTC); the verifier
  (since 18:19 UTC). Done: infra-engineer (review 1), engine-architect (change-set 2's plan draft).

## For the list of the quality gate v2's plan (his words of 15:38 and 18:01 UTC)
- A CI check that refuses to build when the lockfile gains an entry with no recorded review (DEP-HOLD's option (c)).
- CI-CUT part 2.
- KIT-REMOVED: should a manifest that main removed also pass when a branch takes main in (today it stays held).
- The line of the approved text that still reads a path beginning with ":" as a pattern
  (`const entry = git('ls-tree', rev, '--', f);`).
