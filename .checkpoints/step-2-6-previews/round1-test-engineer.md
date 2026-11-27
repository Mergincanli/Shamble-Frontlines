# Round 1, test-engineer: "is it proven?" (PR #72, head 8fef0f6, code head ad2658a)

## Plan
1. Read plan 2.8 / section 7, build-notes, advisor-build-findings, mutate.py.
2. Read hosting/preview.mjs and preview.test.mjs; map each 2.8 duty to its test.
3. Scratch copy under .gate/step26-r1-test/; re-run mutate.py; record result.
4. Add own mutations where gaps suspected; record which survive.
5. G9 lint soundness on YAML shapes.
6. Untested paths list.
7. M1 evidence (B8, B10, B12); M3/M4/CI jobs; M5-M11.
8. Verdict + findings.

## Blocks (saved as done)

### Block 1: baseline and the 29-mutation sweep (done 2026-11-25 ~23:06 UTC)
- Scratch copy .gate/step26-r1-test/ (hosting/ + preview.yml, preview-close.yml, wall-test.yml). preview.yml in tree
  sha256 31119c06... (= approved), preview-close.yml c2f1fa90... (= approved).
- `node --test hosting/preview.test.mjs` (Node v22.23.4): 117 tests, 12 suites, 117 pass, 2.26 s.
- mutate.py re-run: 29 of 29 caught, no bad anchor. Claim B6 reproduced.

### Block 2: my own mutations of preview.mjs (te/mutate_te.py under .gate/step26-r1-test/), 25 run
Caught (3): T16 find-pr list failure ignored; T22 decide `<=` deletes the newest success; T24 no 14-day warning.
SURVIVED (22):
- T1 comment: a failed PATCH/POST (403/500) passes green  -> the failure channel itself unpinned (Medium)
- T2 comment: closed PR still written; T3 head repo not checked; T4 PR read failure not fatal (green, no comment);
  T5 comment-list failure goes on (duplicate POST)  -> Low/Medium
- T6 githubList >10 pages = complete; T18 listPreviews >40 pages = complete (Low, no wrong deletion possible)
- T7/T8 verify-token: HTTP status / success flag ignored (Low; status/name still checked)
- T9 no retry on 5xx; T10 no retry on 403/1010; T21 2 tries instead of 5 (retry policy unpinned; Low-Medium)
- T11 a thrown fetch -> status 200 (equivalent: empty headers still fail judge; but no thrown-fetch test exists)
- T12 monthCount unreadable -> 0 instead of "not counted" (wrong number in the comment; Low)
- T13 read-url id shape unchecked (Low); T14 check-build second pass skipped; T15 check-headers tree refusals ignored
  (defence-in-depth, Low); T17 find-pr state field (Low; list asks state=open); T19 GITHUB_OUTPUT absolute (Low)
- T20 annotate \r\n escape removed: equivalent (messages are show()-escaped); T23 loose closed state: equivalent
  (reconcile only stores open/closed)
- T25 delete: success:false at HTTP 200 logged as deleted (Low)

### Block 3: G9 lint probe (te/lintprobe.mjs extracts lintWorkflow from the test file; planted variants of preview.yml)
Base passes (0 problems). Caught: flow `with: { persist-credentials: true }` (L5), `cache: 'npm'` (L4), cache-mode
changed (L4), 4-space re-indent (only via L2 false-positive on secrets lines), pull_request_target.
MISSED, within G9's own list:
- job-wide `ALL: ${{ toJSON(secrets) }}` and `${{ secrets['CLOUDFLARE_API_TOKEN'] }}` (L2 keys on the text "secrets.")
- `npm ci --ignore-scripts=false` (L3 keys on the substring), `npm rebuild` after ci, `npm --prefix . install`
- `${{` in run with a quoted key `"run":`, with `run :`, in a flow-mapping step `- { name, run }`, or in a comment line
  after a `run: | # c` header (unusual shapes; low likelihood)
MISSED, G1/G6 but outside G9's list: checkout `ref: ${{ github.event.workflow_run.head_sha }}` (the classic
pwn-request: deploy would then run the PR's preview.mjs with the token); the token on a third step's env (npm ci,
read-url); WRANGLER_LOG=debug / metrics "true"; job permissions widened; github-script `${{` in `script:`;
download path into the workspace (G2); wrangler without the empty working-directory (G4).
Structure: the reader assumes 2-space block style; a 4-space re-indent with no secrets lines passes silently (no
assertion that it found 5 jobs / the run steps).
Shapes met in the real file: `|` blocks in item and key form, folded `if: >-` at job level (content inBlock, step -1),
list items under `on.schedule`, flow sequences: all read correctly.

### Block 4: M1, CI, gate (read with gh api, 2026-11-25 ~23:15 UTC)
- B12 holds, and more is readable than B12 says:
  - refusal line IS readable: check-runs/<job>/annotations. 139566690597 (push-to-branch, attempt 3): "Branch
    \"claude/optimistic-albattani-mdoonz\" is not allowed to deploy to preview due to environment protection rules."
    + "The deployment was rejected or didn't satisfy other protection rules."; 139566698136 and 139566697600 (pr jobs):
    "Branch \"refs/pull/72/merge\" is not allowed to deploy to preview ...". steps 0, runner_name "".
  - FRESH runs (attempt 1, not re-runs) on ad2658a after click 5: wall-test 45717720531 (pull_request): pr-plain
    139567978673 + pr-no-deployment-record 139567979434 failure, 0 steps, same refusal line; 45717715486 (push):
    push-to-branch 139567961169 failure, 0 steps, branch refusal line. Not in B12.
  - deployment records ARE readable (deployments?environment=preview): 9 records this month, all wall-test, creator
    Mergincanli. Attempt 2 (22:29) records -> statuses in_progress, success (ran). Attempt 3 (22:56) and the ad2658a
    runs (22:59) -> waiting, failure: a refused job still MAKES a deployment record. 2 records per attempt of 3 jobs:
    `deployment: false` made none (consistent).
  - Consequence for M9 / plan 2.3 "reconcile makes none, so the count is exact": every refused/failed/cancelled
    deploy-job start also makes a record; the count over-counts (safe direction for the 500 cap, but not "exact").
- Environment settings (can_admins_bypass, branch policy, secret names) NOT readable here: proxy 403 on
  environments/* and actions/secrets; dependabot/secrets 403 "Resource not accessible by integration".
- CI tests job: 139508091622 (e3b3642b) and 139567979216 (ad2658a, node v22.23.5): "# tests 117 / # pass 117 /
  # fail 0". check, deploy, after, reconcile skipped. Required checks (ruleset 25069670): only gate + smoke, so
  preview/tests is advisory (red would not block a merge).
- Gate on ad2658a: run 45717720457, jobs gate + smoke success; toolchain 45717720474 success. tools/gate.mjs has
  no reference to hosting/ (the gate proves nothing about the script; M4 is a no-regression mark only).

### Block 5: CLI entry, runbook, M1 second half
- CLI works: `node hosting/preview.mjs` -> usage, exit 2; check-build with DOWNLOAD=failure in a root without
  toolchain.yml -> red "unexpected error ENOENT", exit 1 (fail-closed).
- Mutation T26 `if (false && process.argv[1] ...)` (the entry guard): 117/117 still pass; the mutant exits 0 for every
  command and does nothing (silent green: check-build writes no go, deploy/after skip, reconcile no-op). defaultIo()
  and its `root` resolution are never executed by the suite.
- Runbook item 4 relies on the comment step turning red when GitHub refuses the write: exactly the path T1 shows
  unpinned.
- M1 second half needs: fresh run AND re-run after the token click, 0 steps, refusal line via
  check-runs/<id>/annotations, deployment record statuses (waiting -> failure) via deployments API; plus what M1 does
  not measure today: (a) no repository/Dependabot CLOUDFLARE_* secret (click 9 rests on his word; this session's
  proxy refuses actions/secrets), measurable by a no-environment planted job printing present/absent; (b) a tag ref
  (push of a tag) is not among the planted jobs, while the design's argument is "every job a branch can add runs on a
  branch ref".

### Findings draft -> see final report (verdict REVISE; no High; Mediums: lint secrets-context + checkout ref, comment
write failure + PR read failure untested, CLI entry untested, M1 token-elsewhere unmeasured)

### Block 6: the advisor tool's checks (2026-11-25 ~23:30 UTC)
- Reviewed scratch = committed code: git hash-object of .gate/step26-r1-test/hosting/preview.mjs = 2d34a615 =
  ad2658a:hosting/preview.mjs = 532780de:hosting/preview.mjs; preview.test.mjs 6f0e0c93 = ad2658a's. NOTE: the main
  folder's working tree now holds UNCOMMITTED edits to hosting/preview.mjs, preview.test.mjs, the runbook and
  mutate.py (count wording "deploy attempts ... upper bound", quoted handsOver regex, listing de-dup): another
  reviewer's fix pass, not reviewed here.
- `git diff --stat ad2658a 8fef0f6`: only .checkpoints/step-2-6-previews.md. So the CI proof on ad2658a covers the
  reviewed head's code.
- M3: sha256 hosting/package.json b47551b1..., package-lock.json df24ffc3... = dependency-check.md's two lines;
  security-auditor APPROVE 20:30:14 UTC; lockfile commit 1f4fd0d (20:21, held), first pushed with f0e5593 (CI 20:35:37):
  answer before push. Holds.
- Sweeps re-run with TAP: every caught mutant shows "# tests 117" (the module loaded) and the failing test names
  are the targeted tests (e.g. "planted: two pull requests share the commit"). "immutable not required" and "no
  noindex check" are caught via the bite (good-case test fails "the bite did not bite"), as designed.
- Lint reader on the real file: jobs [tests, check, deploy, after, reconcile], 26 steps, 12 run steps, 22 block lines
  (6 + 6 in the two folded `if: >-`, 10 in the three `run: |`), all key-form. Item-form `- run: |` controls planted:
  with `${{` caught (L1), without: clean. PyYAML (-I, SafeLoader) parses every MISSED variant (02-17, 23-24, 26) with the
  5 jobs; the two 4-space re-indent variants do NOT parse (my re-indent broke the text): that item is dropped.
- Docs (docs.github.com .../deployments-and-environments, fetched ~23:32 UTC): "Only branches and tags that match your
  specified name patterns can deploy to the environment."; "The deployment branch or tag rule is matched against the
  GITHUB_REF of the workflow run."; "Name patterns must be configured for branches or tags individually." The page does
  not say how a branch rule `main` treats refs/tags/main; for "Protected branches only" it notes runs "triggered by
  tags with the same name as a protected branch" cannot deploy. So the tag case is UNMEASURED and the docs do not
  settle it.

### Block 7: question 6, what first meets the real world at M5-M11 (unproven before merge)
- main ADMITTED to `preview` (M1 measures only refusal) and the secrets reaching verify-token (M5).
- Wrangler's real upload with install scripts off (B5: loads, fails at auth with a dummy token) (M5).
- Cross-run download (download-artifact run-id + actions: read) of a real `preview-site` artifact: needs the CI
  Builder's hand-over step on main first; until then every check ends with a notice (no preview at all).
- Cloudflare's real response shapes (verify, project subdomain, deployments list paging/result_info, delete with
  force, Wrangler's ND-JSON alias): every fake is built from docs + Wrangler 4.164.0 source (M5, M6, M8).
- GitHub permissions: comment write with pull-requests: write, deployments: read for the count (M5, M9).
- 404 propagation and Pages' 403/1010 to a runner's scripted fetch (M7); the retry policy is unpinned (T9/T10/T21).
- The CLI entry itself (T26): first exercised by the first real run; a broken guard = all steps green, nothing done.
- The count's meaning (M9): records include refused/failed/cancelled deploy-job starts; November already holds 9
  wall-test records, and the click-8 re-runs add more.
- Secret hygiene in real logs (M10): wrangler's own output with a real token; the runner masks registered secrets.
- M11's median needs 5 real runs.

### Block 8: verdict REVISE (no Critical, no High). Findings as handed back:
M-A lint misses literal G9 bullets (toJSON(secrets)/secrets[...] job-wide; --ignore-scripts=false) test:696,:702
M-B lint misses checkout ref:/repository: (G1) test:718
M-C comment write failure + PR read failure unpinned (T1, T4) preview.mjs:512, :491
M-D CLI entry + defaultIo never run (T26) preview.mjs:607-629
M-E count not exact (refused jobs make records; measured) plan 2.3 :144, preview.mjs:454-472
M-F M1 does not measure token-elsewhere (repo/Dependabot secret) nor a tag ref
L-G B12 under-reports (annotations + deployments readable; fresh ad2658a refusals)
L-H G6/G2/G4/permissions not linted; L-I unusual YAML shapes / no structure assert; L-J other unpinned paths
(T2,T3,T5-T10,T12-T15,T17-T19,T21,T25); L-K preview/tests not required, gate blind to hosting/.
Own mutations: 26; caught 3; equivalent 3 (T11, T20, T23); real survivors 20.
Disclosure: early calls had `cd ... &&` before git and a `for` loop around gh (read-only).
