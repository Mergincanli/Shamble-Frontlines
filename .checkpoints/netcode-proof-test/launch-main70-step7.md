# MAIN-70 step 7: the brief for the advisor agent's read of the pushed head (written 02:3x UTC on 11-28)
Status: WRITTEN, not sent; the CI lines below are filled when the two toolchain runs end.

## The brief (sent as written, to a new advisor agent)
The head read of one pushed head: MAIN-70, the take-in of main dae6d107fdbe942567104bcf1c9a2cbc6b3a1e0f (#79, cs4b-2
"The world") into PR #40's branch `claude/netcode-proof-test-plan-hcky5g`: merge commit
2d379d9c7922deedf4e7fefe08d6eb04c6843afe (parents H0 e646ee5999e9c3bfdcee6577cee0adf45e9339fa and dae6d107...), pushed
under the card save 8d1306fe975dbafa62baf27f49e78bdb23f2c932. Read by these SHAs only: the branch may move while you read
(the next lane step, Y', commits and pushes a one-line change to spikes/netcode-proof/ops/runs/r1-dist.sha256); its tip
is not what you rule on.

As the advisor (main's .claude/agents/advisor.md, read by `git show origin/main:.claude/agents/advisor.md`; read only:
no cargo, no npm, no git write, no fetch), read and rule:
1. The plan and its record: .checkpoints/netcode-proof-test/main-70-plan.md (rev 0.2 as checked, plus its "Step 1,
   done" and "Steps 2-5, done" blocks) against .checkpoints/netcode-proof-test/advisor-main-70.md (APPROVE WITH
   CONDITIONS M70-1 to M70-9). Every done line holds at the SHAs above by your own printing reads; logs in
   /home/user/Shamble-Frontlines/.gate/main-70/ (light-base.log, light.log, gate.log, cached-names.txt, main-names.txt,
   head-names.txt).
2. MAIN-65 step 7's form (main-65-plan.md and advisor-main-65-head.md; main-68-plan.md:332-335 is the same form), with:
   - M70-3's three whole-tree bars at 2d379d9c and at 8d1306fe (8d1306fe adds only .checkpoints/ files to 2d379d9c:
     check it);
   - DEP-HOLD's line: `git diff --name-only origin/main 8d1306fe -- Cargo.toml Cargo.lock crates packages package.json
     package-lock.json` prints nothing;
   - M70-6's baseline: main's toolchain run 46081134408, job 140780125932 (23:12:11-23:22:00 UTC), compared line for line
     as M67-3 gives it (the `test result:` lines, the vitest `Test Files` and `Tests` lines, every Playwright "passed"
     line, Chromium and Firefox) with the toolchain jobs of PR #40's two runs on 8d1306fe (below);
   - #74's two pull-request steps by rev 4's step-6 bar (main-68-plan.md:327-330: on the pull-request toolchain run both
     succeed, the check's printed line "<n> files, <bytes> bytes" is recorded and the artifact `preview-site` is listed;
     on the push run both are skipped);
   - the toolchain job's run time beside the 25-minute limit (#79's report: 8 min 48 s to 13 min 2 s on its heads).
3. One departure, recorded in the plan's done lines: the merge message is git's own with its "# Conflicts:" lines (as
   MAIN-68's 6b4a0d3) plus the two attribution lines added by `git commit --no-edit --trailer ...`; M70-9 named git's
   message alone. Rule on it: stands, or what is owed (amending would rewrite a commit; it is not an option).

CI on 8d1306fe (read 02:3x UTC):
- push: gate 46098323772 success (02:29:31-02:30:14), toolchain 46098323769 <TO FILL>;
- pull request: gate 46098326092 success (02:29:33-02:30:55), toolchain 46098326060 <TO FILL>.

Rule: APPROVE, or what is owed. Write .checkpoints/netcode-proof-test/advisor-main-70-head.md (your plan first, then each
block as it is done, a notes line after each finding); end with "Decisions needed" and "For Mustafa", then, once every
block is written, the last line `Status: COMPLETE <date and UTC time>`.

Write files with the Write and Edit tools only (no sed, no shell redirect, no python writes). git and gh always start
their own command, with no other program in front of them; no `git -C`, no cd in front of git. In this tool a command that
exits non-zero without printing can show nothing at all, the same as a pass (seen with `test`, `git diff --quiet` and
`grep -q`): use only forms whose output carries the answer (`git diff --name-only` or `--stat`, `ls -d`, `cmp -s` with an
echo of its code, sha256 lists), and quote that output. His rule: "in a shell command, spell out every file you write with
its full path. No variable, substitution or wildcard in the name of a file that is written, moved, copied or deleted. Or
use the Write and Edit tools for it." Never print an environment variable's value (names only). GitHub reads through the
GitHub MCP tools (load them with ToolSearch); never comment or write on GitHub.
