Task: infra-engineer review (single round) of shamble-preview-check branch setup head 1b37fae: CI and hosting, do the four workflows and scripts work together on GitHub runners and Cloudflare Pages.

## Plan (blocks)
B1 read plan.md Part 7 workflow texts + Part 2 summary (skim)
B2 deploy.yml vs build.mjs, sets.mjs, lib.mjs, check-headers.mjs, check-sizes.mjs
B3 close.yml
B4 probe.yml vs probe.mjs, links.json
B5 browsers.yml vs wdio/run.mjs, package.json
B6 write findings section

## Notes
- B1 skipped as separate read (used docs notes greps instead: runner Node 22.23.4 default, wrangler 4.156.0 engines >=22.0.0 verified via npm view today).
- B2 deploy.yml read. Notes: build matrix no fail-fast:false (builds should not fail; check-sizes never fails -> Low). @production `test -n && test` under set -e: empty production_branch would not stop (Low, harmless). download-artifact pinned SHA: zero pattern matches -> success, 0 downloaded (raw source read 2026-11-15) -> revoked run verify = 0 records, exit 0 (fine). check-headers missing dir -> readRecords returns [] (fine).
- B3 close.yml read. Notes: paging breaks on length<25 (fine if CF honours per_page=25; if CF caps per_page lower, loop stops after page 1 -> missed deletions; result_info.total_pages more robust) - Medium-ish inferred. curl network failure mid-loop exits set -e leaving some undeleted (Low).
- B4 probe.yml + probe.mjs read. Notes: records retention-days 7 -> if PM-0006-6/-8 readings slip beyond 7 days after a deploy, records expire, probe lists nothing for that branch (expired filter) -> mark unmeasurable (Medium). links.json hardcodes project subdomain (CF may add random suffix if name taken) -> browsers.yml rows hit wrong host (Low/Medium; links.json not an approved workflow text).
- B5 browsers.yml + run.mjs read. Notes: HeadlessChrome UA vs Pages Browser Integrity Check unknown (inferred risk; page would not set data-done -> 'no reading', reason not recorded) Low.
