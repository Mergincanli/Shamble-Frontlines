# security-auditor: round 1 on step 2.6's final diff (PR #72, head 8fef0f6, code head ad2658a), 2026-11-25
Scratch: .gate/step26-r1-sec/. Read-only. No wall detail in this file (his words of 2026-11-25).

## Plan (blocks)
R1 sha256 of both workflow files + diff list. R2 run `node --test hosting/preview.test.mjs` (and the G9 lint on the real
file). R3 preview.yml whole vs G1-G9. R4 preview-close.yml. R5 hosting/preview.mjs whole (8 commands, escaping, refuse
list, reconcile fail-closed, comment checks). R6 _headers + package files (sha256 vs dependency-check). R7 runbook
security statements + docs records. R8 M1 evidence (B10-B12) + environment settings + token sitting's instructions.
R9 O2/O3 + read-url hardening. R10 verdict.

## R1 hashes (sha256sum, real tree at 8fef0f6)
- preview.yml 31119c06e7d0...abc (10,577 bytes) = his approved text. preview-close.yml c2f1fa90511d...e66 = approved.
- package.json b47551b1..., package-lock.json df24ffc3... = dependency-check.md. _headers 77562be5...
- Diff (07d673b0...HEAD, excl. .checkpoints and wall-test.yml): 11 files, as the caller lists.

## R2 tests
- `node --test hosting/preview.test.mjs` (Node v22.23.4, real tree, workflow texts placed): tests 117, suites 12,
  pass 117, fail 0. The G9 lint suite runs against the real .github/workflows/preview.yml (open item from early read: done).

## R3 preview.yml vs G1-G9 (read whole, 260 lines)
- Pins re-read by `git ls-remote` (2026-11-25): checkout v7.0.1 = 3d3c42e5..., setup-node v7.0.0 = 82076278...,
  download-artifact v8.0.2 = 9000827c..., upload-artifact v7.0.2 = cf430e03...: all four match the text.
- G1: `${{` on lines 69-71,83-85,94-96,101,108,121,123,128,139-140,148,159-169,204,209,214-220,256-259; none inside a
  run block (run blocks :86,:102,:141-143,:150,:152-156,:161,:170-174,:177,:210,:221,:260). permissions {} (:21); every
  checkout has no ref + persist-credentials false. G6: secrets only on step env :159-160, :165-166, :256-257.
  G8: cache-mode none on all 5 jobs; package-manager-cache false on all setup-node. G4: wrangler cwd $RUNNER_TEMP/empty.
  G5: deploy re-checks PR/SHA (:141-143) and recheck (:150) before any token step. timeout-minutes 10 everywhere.

## R4 preview-close.yml (18 lines, sha c2f1fa90...)
- permissions {}, no secret, no checkout, one echo, timeout 2. Unchanged from R2-3: a branch can edit it at the same path
  and start reconcile; reconcile reads nothing from the triggering run (confirmed in code, R5). Fork closes also start
  reconcile (Low from R2-3, accepted with the text; not re-raised).

## R5 hosting/preview.mjs (629 lines, read whole)
- Printing: show() = JSON + every char >= U+007F escaped (surrogates included); say() throws on a leading "::";
  annotate() escapes %, CR, LF; every outside value passes show(). No token printed: GH/CF tokens only in headers
  (:137, :142); cloudflareErrors keeps code + message cut to 200 chars (:155-156).
- request(): fixed hosts (GITHUB_API, CLOUDFLARE_API constants), redirect 'manual', 30 s timeout (:117-133).
- find-pr :213-231: HEAD_SHA hex40; open, head.sha, head.repo == base.repo == REPO, number pattern; exactly one match.
- check-build :235-267: only DOWNLOAD === 'success' reads the build; refuse list on raw, copy plain files with
  COPYFILE_EXCL, add main's _headers (flag wx), refuse list again with byte-equal header; go only true/false.
- refuse list :159-167, :173-204: lstat walk (no link followed); leading dot / "-", "..", charset [A-Za-z0-9_][A-Za-z0-9._-]
  {0,127}; HOST_FILES case-insensitive at any depth (_worker.js, _routes.json, _redirects, _headers, functions,
  wrangler.*); non-regular refused; >= 25 MiB; > 20,000 files; index.html at top. Directories are not counted toward a
  cap (bounded by the artifact and the 10-minute job: our own runner only; no finding).
- recheck :270-276: same walk on the deploy job's copy, _headers byte-equal to main's.
- verify-token :280-311: account hex32, token printable 20-255; account-token verify; status active; project name,
  subdomain pattern, production_branch not pr-<n>; subdomain to RUNNER_TEMP 0600.
- read-url :316-342: one pages-deploy-detailed entry; project, environment preview, uuid, url = id8.<sub>, alias
  pr-<n>.<sub> (literal-escaped sub) and PATTERNS.link. PR not tied here (F3 split); comment ties it (:485).
- check-headers :400-420: URL by PATTERNS.link (any pages.dev, but its only source is read-url's checked output);
  file names already charset-checked, so `${url}${path}` cannot carry ?, #, % or a host change; redirect manual.
- comment :474-515: PR, SHA, DEPLOY/HEADERS in fixed sets, link tied to pr-${PR}. (trailing dot), runUrl from patterns;
  re-reads the PR (open, head sha, head repo) before writing; body from checked values only; edits only a comment by
  github-actions[bot] (type Bot) starting with the marker.
- reconcile :572-600 + decide :524-551: listing incomplete -> refuse, nothing deleted (:578); env=preview and
  d.environment preview; branch pr-<n>; uuid id; each PR looked up, number equal, state open/closed else left alone;
  closed -> all; open -> only older than newest successful; oldest first, cap 20; each logged before delete; a failed
  delete turns the step red. Fail-closed holds.
- run() :614-626: unknown command -> usage, exit 2; errors become one escaped ::error:: line.

## R6 _headers, package files, lint
- _headers: /* X-Robots-Tag noindex; /assets/* immutable; no CSP (ADR 0006 B3: the CSP waits for the security baseline,
  BASELINE-WHEN (b): not a 2.6 finding). package files = the DEP-HOLD sha256 (R1).
- G9 lint (preview.test.mjs:655-760): L0-L5 + pull_request_target, 16 planted variants; it lints the real
  .github/workflows/preview.yml (passes, R2). preview-close.yml is not linted (no secret in it; nit, no finding).
- Diff scan: no 32-hex account id, no real subdomain, no token pattern outside test fixtures (0123..., x1, evil).
- DECISIONS: his NEW-TEXT (a) names 31119c06... (docs/DECISIONS.md:172-176) = the placed text.
- Wrangler's config/.env look-up from cwd $RUNNER_TEMP/empty upward: _temp, work, runner, home, / : none written by
  the build (zip-slip safe extraction into $RUNNER_TEMP/site). npm ci in $RUNNER_TEMP/wrangler: only the two main files.

## R7 runbook (docs/runbooks/previews.md, 81 lines)
- :14-15 token only in environment preview, only main. :16-19 the bypass box unticked AND saved. :20, :69-71 account
  holds nothing else, 2FA. :59-68 90-day window, renew, revoke. :73-76 noindex, preview access off.
- Gaps (Low): renew (:62-65) has no "then confirm no repository/Dependabot secret named CLOUDFLARE_" and no "reload
  the environment page and check the box and the one rule"; no line on bumping Wrangler (O3: a new DEP-HOLD, tier 2)
  or watching advisories for hosting/ (O2; no CI job runs npm audit on hosting/: the advisories policy is the security
  baseline's). :17-19 states the cause as fact; one confound (below).

## R8 M1 evidence (read by me with gh, 2026-11-25)
- Refusal lines read (check-run annotations): attempt 3 jobs 139566690597 (push): 'Branch "claude/optimistic-albattani-
  mdoonz" is not allowed to deploy to preview due to environment protection rules.'; 139566698136 and 139566697600:
  'Branch "refs/pull/72/merge" is not allowed ...'. B12's "not read" gap is closed.
- NEW, not in B12: the push of ad2658a made FRESH runs (attempt 1, not re-runs), 22:59 UTC: wall-test 45717715486 (push,
  job push-to-branch 139567961169) and 45717720531 (pull_request, jobs pr-plain 139567978673, pr-no-deployment-record
  139567979434): all failure, 0 steps, no runner, same refusal lines. triggering_actor Mergincanli on all.
- Deployment records ARE made for refused jobs without `deployment: false`: environment preview records 8071628786,
  8071628124 (22:59), 8071582742, 8071582463 (22:56) each with statuses waiting -> failure; the admitted attempt 2
  record 8071101088: in_progress -> success. pr-no-deployment-record makes none. So a record is not the pass mark; its
  status is (failure = refused).
- attempt 2 of 45699354868: triggering_actor Mergincanli (admin), run_started_at 22:29:03.
- Confound: B11's screenshot (after 22:31) shows the banner 'Deployment branch rule "main" saved successfully', so the
  rule may have been saved after 22:29; attempt 2's admission is explained by the unsaved box OR a not-yet-saved rule.
  Either way the fix (both saved, checked after a reload) and attempt 3 + the fresh runs hold.
- Not readable here: the environment's settings API and the repository secret list (the proxy refuses both: HTTP 403).

## R3b G2, G7, G6 detail (added after the advisor tool's read)
- G7: grep of preview.mjs and preview.yml for GITHUB_ENV|GITHUB_PATH|GITHUB_STEP_SUMMARY|set-output|save-state: no hit.
  The only runner-file write is setOutput (:78-83): pattern-checked value, `name=value\n` to GITHUB_OUTPUT.
- G2: the three downloads (:91-96, :145-148, :201-204) name an exact artifact, write to a fresh $RUNNER_TEMP folder,
  set no merge-multiple and no digest-mismatch override (default "error", E5); only check's names run-id + token (the
  toolchain run); deploy and after read this run only.
- G6: no WRANGLER_LOG anywhere; deploy uploads nothing (no upload-artifact in deploy); the token only on :159-160,
  :165-166, :256-257.
- Item 3 (missing build): check-build :238-247 fails closed (go=false, then exit 1 with ::error:: when main's toolchain
  hands a build over; notice + exit 0 before that). Runbook :24-26 says it shows only as a red run under main. No finding.
- Test file: fake fetch only (harness default fakeFetch), files in an mkdtemp folder removed after; WORKFLOW and HEADERS
  read the real paths (:18-19); node:net only for a local socket file (refuse-list case :187-190).
- My own mutations (.gate/step26-r1-sec/m1, m2): reconcile's `if (!listing.complete)` disabled -> 1 test fails; the
  comment's `startsWith(pr-${pr}.)` tie removed -> 1 test fails. Both caught.

## R9 O2, O3, read-url hardening
- O1 stays an observation (early read E6). O2/O3: the runbook names no Wrangler bump rule and no advisory watch for
  hosting/ (Low, runbook line; the advisories policy is the security baseline's). read-url hardening (PR in its env):
  not needed, comment ties the number (:485, tested by m2); it would change preview.yml's text, so not asked.
- monthCount (:454-472) counts every `preview` deployment record whatever its status: refused and failed jobs count
  (today 9 wall-test records), and any branch job naming the environment adds one. Errs high (earlier warning).
  Low; fix: call it an upper bound in the runbook/comment, or count only records whose newest status is success.

## R10 verdict
APPROVE on the diff; no change to preview.yml (31119c06... stays) or to the code. One condition for the token
sitting's message (Medium, not a file of this diff). Four Lows (runbook wording x3, the month count), fix pass or STATUS
follow-up. Detail in the handback.
