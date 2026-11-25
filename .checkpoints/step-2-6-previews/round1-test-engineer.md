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
