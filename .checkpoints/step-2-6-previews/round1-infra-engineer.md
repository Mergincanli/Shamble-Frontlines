# infra-engineer: step 2.6 round-1 review (as built), PR #72 head 8fef0f6 (code ad2658a), base main 07d673b0

Started 2026-11-25. Memory: no agent-memory folder in this checkout. Scratch: .gate/step26-r1-infra/.

## Plan
1. Read preview.yml, preview-close.yml, hosting/preview.mjs, _headers, package.json, runbook (as built).
2. Verify preview.yml sha256 vs Mustafa's approved 31119c06...
3. Judge 1: triggers, concurrency, hand-over, PR runs.
4. Judge 2: Cloudflare calls (paging, delete force, subdomain, production_branch, verify, ND-JSON, alias); first-run risks.
5. Judge 3: month count.
6. Judge 4: runbook vs B9-B12 and docs.
7. Judge 5: costs, limits, rate limits, cap 20.
8. Verdict + findings; flag any preview.yml text change.

## Findings (one line each, as found)
- R0. preview.yml sha256 = 31119c06... (matches his approval); preview-close.yml c2f1fa90... (matches plan appendix).
- R1 (read) jobs/triggers read; script read whole; runbook read. Candidate points: runbook renewal lacks bypass-saved
  re-check (click 8 had it); monthCount null after 1000 records if GitHub's order is ascending; deployment records
  over-count (failed/cancelled deploys, wall-test records in Nov); reconcile cancellations of pending runs (GitHub
  concurrency drops a pending run when a newer one queues); first-run: account-owned token with Wrangler pages deploy.
- R2 (measured, gh api 2026-11-25): toolchain run 45686704452 object: path ".github/workflows/toolchain.yml" (no @ref
  suffix), event pull_request, head_repository Mergincanli/Shamble-Frontlines: check's `if` fields match the real shape.
- R3 (measured, gh api deployments?environment=preview 2026-11-25): 9 records, all from wall-test (ref = the branch),
  newest first (22:59:06 down to 20:54:15). The refused jobs of attempt 3 (22:56:12/13) DID make records. So: M1's
  "deployment record made?" = yes (2 per refused run); November's count is inflated by >= 9 wall-test records; the
  observed order is newest first (undocumented, consistent with the early stop).
- R4 (#66 report :351-354, :580-592): the hosting check deployed with an Account API token through pinned Wrangler
  (exit 0): the account-owned token works for `pages deploy`; not a first-run risk. #66 also hit the single-date chip
  (token valid one day only): the runbook's renewal omits plan click 6's "each in its own field, never the chip".
- R5 runbook renewal (:62-65) lacks: the bypass box re-check + "Save protection rules"; the chip warning; a check
  after (next upload's verify line). No rebuild-from-zero section (account creation, Continue to Pages path, B9).
- R6 (#66 report :603, :652, :1183, :1674): a dashboard drag-and-drop Pages project had production_branch "main";
  Wrangler's url = https://<first 8 of id>.<sub>, alias https://<branch>.<sub> (no trailing slash), environment
  preview; the cleanup's listing filtered on deployment_trigger.metadata.branch and worked. So read-url's and
  verify-token's checks match measured shapes (Wrangler 4.156 then; 4.164 code read in B4). R4 residue: small.
- Sources read 2026-11-25:
  - docs.github.com re-run page: "up to 30 days after its initial run"; re-runs "use the same GITHUB_SHA ... and
    GITHUB_REF of the original event"; failed-job re-runs re-run dependents, others keep outputs.
  - docs.github.com workflow-syntax: pending job in a group "canceled and replaced" (queue: single default; queue: max
    keeps up to 100 pending); job concurrency contexts include needs; environment.url may use steps; cache-mode values
    read/write/write-only/none (so `cache-mode: none` is a real key).
  - developers.cloudflare.com/pages/platform/limits (dateModified 2026-11-19): 500 builds/month Free, direct uploads
    not addressed; 20,000 files Free; 25 MiB per asset; 100 header rules; the 30-day sentence unchanged.
  - developers.cloudflare.com/pages/configuration/preview-deployments (dateModified 2026-11-19): "every preview
    deployment ... includes the X-Robots-Tag: noindex HTTP response header" by default; aliases lowercased.
  - developers.cloudflare.com/fundamentals/api/reference/limits (2026-08-25): 1,200 requests / 5 min per user; 429
    and blocked for the rest of the window.
  - docs.github.com billing for Actions: standard runners free in public repositories (artifact storage for public
    repos not stated).
  - docs.github.com disable-and-enable-workflows: Actions tab, the workflow, "..." menu, "Disable workflow"; public
    repo schedules auto-disabled after 60 days with no repository activity.
