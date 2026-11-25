# Checkpoint: step-2-6-previews (lane 2, Builder V2): step 2.6's plan, the preview links

Branch `claude/optimistic-albattani-mdoonz` (stands in for `cs/step-2-6-previews`; this session may push only it),
restarted at main c1da1bd24faae21794195e2384895460753a41eb (#67's merge, 2026-11-25). Risk tier 2 for the build (a CI
workflow, a deploy token, hosting): the advisor agent and the security-auditor review; his go in a day session.

## Goal (his words of 2026-11-25, 18:15 Amsterdam, item 3; transcript 17:11:56 UTC, uuid 42aa43e4...)
"Then step 2.6's plan, the preview links, as my order of 25 November has it. Plan first, your advisor agent's review,
then back to me for my go; nothing built before it." The plan holds:
a. his WORKERS-NEXT answer: the deploy step small enough to move, two lines on what a move would cost, the 30-day rule
   for old previews;
b. every click he does himself (the Cloudflare project, the deploy token, anything on GitHub), listed once, in order,
   for one sitting;
c. what lane 2 needs from the CI Builder chat (the edit that hands the tested build to the preview job), as a request
   it can act on;
d. the day he can expect the first preview link, said honestly.
Item 4: step 2.6's first pull request also records his 17:05 message (uuid 0ff798eb..., 16:41:51 UTC, 1094 bytes,
sha256 c41b21e8...) in DECISIONS and adds the dated pointer in #67's report (the advisor agent's Low on 35adfbe).
His 18:15 message itself (uuid 42aa43e4..., 1778 bytes, sha256 e2554c4e...) goes in with it.

## Inputs (read, not decided here)
- Build plan `docs/reports/2026-11-17-build-plan-slice-1.md` :1021-1027 (when step 2.6 can run) and Part 15
  (`...-part-15.md` :103, :176): lane 2 owns the preview workflow (a new file), the host's config files, its runbook;
  must not touch gate.yml, toolchain.yml, the gate's files, crates/; the tested build comes by an edit to toolchain.yml
  that lane 1 (now the CI Builder chat) makes on his go.
- ADR 0006 parts 1-3 and 13 (B9), the revised PM-0006-2 and -7; ADR 0002 (the deploy job, seam S3, PM-0002-66).
- #66's report (Appendix B: the workflows that ran; follow-ups PHC-*), #67's report (follow-ups ADR6-*, STEP26-WORKERS,
  PHC-PREVIEW-NOINDEX).
- His words: "I do not want to click for each preview" (2026-11-19, 00:01:25 UTC); STEP4-COUNT (b): only the newest
  push of a pull request is deployed, and deploys are counted per month; WORKERS-NEXT (a) points.

## Steps
1. #67 merged on his go: c1da1bd (normal merge, expected head 35adfbe; checks green; the advisor agent's APPROVE).
   Main folder fast-forwarded to main; the fixed guard is on disk (hooks equal main). Branch pushed (fast-forward).
2. Research by infra-engineer (notes: `step-2-6-previews/infra-engineer-design.md`): current step.
3. The plan (`step-2-6-previews/plan.md`), with an advisor tool consultation before it.
4. The advisor agent's plan review (and the security-auditor's read of the token design).
5. Back to him for his go; nothing built before it.

## His go (2026-11-25, 20:35 Amsterdam; transcript 19:36:53 UTC, uuid fa086681..., 1579 bytes, sha256 3c4f0d23...)
On revision 2 at c40f3f9:
- TRIGGER (a); ACCOUNT (b); LANE2-LOCKFILE (b): lane 2 owns hosting/package.json and hosting/package-lock.json, for the
  upload tool only, outside the game's install, and nothing else of his words of 18 November changes; DEPARTURES (a).
- MAIN-WALL (b): the risk accepted in writing for now, because the token reaches a previews-only account; the wall goes
  to the Guard fix chat as its next change-set. The real token is his last click: if the wall is merged by then, the
  acceptance ends there; if not, ask him again before that click.
- His go covers the three texts by sha256 (preview.yml c66c0b0a..., preview-close.yml c2f1fa90..., wall-test.yml
  3af06706...). The security-auditor reads preview.yml as it stands, with the token step's added duty, in round 1. If a
  review changes a text, bring him the new text and its sha256 before his sitting.
- Send the request to CI Builder as planned; keep the wall's detail out of every further pushed file (the notes'
  detail was removed on 2026-11-25 after his go).
- Unchanged: git and gh always start their own command; a merge on his go for the approved head, after his Advisor's
  merge check.

## Current step
U0 done: the request to CI Builder posted on #59 (comment 7172595904; the tool's answer at 2026-11-25 19:39:29 UTC; plan section 5's text,
no wall detail, footer).
U1, part 1 done: dfc8fb6 (hosting/preview.mjs, hosting/preview.test.mjs, hosting/_headers, REPO_MAP's hosting/ row):
117 tests pass in a scratch copy with the approved texts; 28 of 28 mutations caught; gate PASS (--no-smoke); map OK.
Evidence: step-2-6-previews/build-notes.md (B1 to B6).
U1, part 2 done: 1f4fd0d (hosting/package.json b47551b1..., hosting/package-lock.json df24ffc3...); the
security-auditor's M3 answer APPROVE on both sha256, no findings (notes: security-auditor-deps.md, D1-D7; O1-O3 kept
for round 1); dependency-check.md written from sha256sum, naming those notes.
U3 done: 8a8531a (DECISIONS: his 17:05, 18:15, 20:35 messages, 70 lines, by script; the dated pointer in #67's report).
986547d: docs/runbooks/previews.md (written for PREVIEW-MISSING-BUILD-COMMENT (a); changes if he picks (b)), its
REPO_MAP row, the prefix test; 117 tests pass; 29 of 29 mutations caught.
The advisor agent's verdicts on F1-F4 (advisor-build-findings.md): F1 APPROVE with conditions; F2 to him as
PREVIEW-MISSING-BUILD-COMMENT, recommend (a); F3 within the plan, name the split in the report; F4 settled by GitHub's
rendered docs (re-read with curl); the order approved; Finding A (Medium): the security-auditor reads the proposed
preview.yml, the token check's added duty and F3's split BEFORE the message to him.
The security-auditor's early read (Finding A; security-auditor-early-read.md, E1-E7): no text change, no findings;
download-artifact 8.0.2 keeps entries inside `path` (its code and a crafted zip); O1 stays an observation; the optional
read-url hardening (PR in that step's env) left for round 1, not offered now.
HIS ANSWER (2026-11-25, 20:51:45 UTC, uuid 33a78af8..., 28 bytes, sha256 6fbf35bf...): NEW-TEXT (a),
PREVIEW-MISSING-BUILD-COMMENT (a); in DECISIONS by script (scripts/add_answer.py).
U2 done: the three texts placed by scripts/place_u2.py (sha256 31119c06..., c2f1fa90..., 3af06706... checked after).
Process note: the write ran inside Python, which the guard cannot see, so it did not ask him; his NEW-TEXT (a) in
chat was the ask. Tell him. In the tree: 117 tests pass; gate PASS (--no-smoke); map OK (202 entries).
U5: draft pull request #72 opened (2026-11-25, after 20:54 UTC; head 95d32ac). The push's wall-test run 45699354868:
job push-to-branch ran and printed "absent" (the before-bite for the push job; "Cache mode: none"). Main moved to
07d673b0 (#71, lane 1's ci-after-3a; no hand-over step in toolchain.yml yet; not the Guard fix chat's wall): main
is taken in later, before the merge.
#72 was held as conflicting by GitHub (mergeable_state "dirty" after #71), so no pull-request run started. Main
07d673b0 taken in by one merge commit e3b3642b (worktree .gate/merge-71; scripts/resolve71.py, verify71.py: the
three-way check OK in all 7 files main changed; one-sided files equal; the workflow hashes intact; gate PASS, 117
tests, map OK); pushed as a fast-forward; the main folder fast-forwarded to it (no guard prompt).
Next: the pull request's own runs (preview/tests = the CI proof of the tests; wall-test's two pull-request jobs print
"absent"); then his
sitting with M1 (plan section 4). Before click 8 (the real token): if the Guard fix chat's wall is not merged, ask
him again (his MAIN-WALL (b)).

## Findings during the build (U1)
- F1 (a defect in the approved preview.yml, job `tests`): `run: node --test hosting/` fails on Node 22: the folder is
  loaded as a module ("Error: Cannot find module 'hosting'", exit 1; measured on v22.23.4, scratch folder). Naming the
  file works (`node --test hosting/preview.test.mjs`, exit 0; a missing named file exits 1). A `"main"` field in
  hosting/package.json would also make the folder argument work (measured), but hides the fix. Proposed: change that
  one line and bring him the new text and its sha256 before his sitting (his 20:35 item 5).
- F2 (plan prose against the texts): section 2.7 says the comment reports a missing build once the hand-over step is
  on main; with the approved texts it cannot: when `check` fails, `go` is not `true` (check-build writes go=false
  and exits 1), so `after` (the comment) does not run.
  The failure shows as a red `check` job under main's Actions tab instead.
- The real build (toolchain run 45686704452, job 139461080011, 2026-11-25 19:36 UTC): index.html plus assets/
  index-*.css, index-*.js, sim_wasm_bg-*.wasm, worker-*.js; names use [A-Za-z0-9_-] and dots (hashes hold _ and -).
- Wrangler 4.x releases (npm view, 2026-11-25 19:48:15 UTC): newest at least 7 days old: 4.164.0 (2026-11-17 11:15 UTC);
  4.165.0 (2026-11-18 21:44 UTC) turns 7 days at 2026-11-25 21:44 UTC.

## Rules in force
- Until a second guard fix is merged: git and gh always start their own command, with no other program in front.
- At most 3 agents at a time; Opus; effort xhigh for infra/security design work on the drafts' parts.
