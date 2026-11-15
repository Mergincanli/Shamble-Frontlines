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
- FINDING (High, verified docs + code): check-headers.mjs:50 fetches every manifest file incl. /index.html and /404.html (build.mjs:13,95); Pages redirects .html to extension-less (serving-pages, Last updated Nov 11 2026: "/contact.html will be redirected to /contact", "/about/index.html ... /about/"); lib.mjs:60 redirect manual -> 3xx -> judge line 22 "status 3xx" -> every link form FAIL, verify red every run, PM-0006-2 false fail. Local proof serve-headers.mjs:75 serves .html with 200, so it could not catch it. Fix in script only (no workflow text change).
- npm --prefix verified locally (npm 10.9.9, Node 22.23.4): fresh prefix folder gets node_modules/.bin/<bin> -> wrangler path exists.
- B5 more: WDIO 10.1.0 @wdio/utils build/node.js read (local copy .gate/.../wdio-read): chromedriverOptions.binary used (l.1065), gecko/edge binary -> customPath (l.1115-1124); no browserVersion -> preinstalled browser located (l.759-768), Windows version from Chrome folder name (l.565-570). Lock: no hasInstallScript -> --ignore-scripts drops nothing. browsers.yml OK.
- Cloudflare list API page (read 2026-11-15): page, per_page with no min/max/default stated -> close.yml paging risk stays inferred (Low).
- FINDING (Medium, verified files): records retention 7 d (deploy.yml:158, close.yml:98) vs probe filter expired==false (probe.yml:48) vs plan risk 8 window up to 10 days -> late PM-0006-8 sweep / late readings silently drop URLs (fewer rows, no error).
- FINDING (Low): close.yml:78 stops on length<25 (per_page cap unknown); cross-check "Uploads found" vs records.
- FINDING (Low): run.mjs:52 timeout records no reason (BIC/HeadlessChrome inferred).
- FINDING (Low): deploy.yml:52 echo "legs=$(node ...)" hides sets.mjs failure; build then fails at fromJSON('') (clear enough; no mark lost).
- FINDING (Low): deploy.yml:135 `test -n && test` under set -e: empty production_branch does not stop the step (wrangler gets --branch ""; not-in-git = production per notes, so harmless).
- B6 reopened (advisor): line numbers corrected (probe.yml:48, lib.mjs:60); gh api --jq object output verified compact one-per-line on a reachable endpoint; no agent memory dir found.

## B6 Answer (final; write the answer from this block)
Verdict: REVISE (one High in a script, no workflow text must change for it).
F1 High, tools/check-headers.mjs:50 (with :22, lib.mjs:60, build.mjs:13 and :95). Verified in code: manifest lists index.html and 404.html; fetch redirect manual; judge needs 200. Documented: Pages "will also redirect HTML pages to their extension-less counterparts: /contact.html -> /contact, /about/index.html -> /about/" (serving-pages, Last updated Nov 11 2026). Inferred: status 308, and /404.html covered by the same rule. Scenario: every link form of every upload gets "status 30x" on /index.html and /404.html -> FAIL, verify red every run, PM-0006-2 reads as a host failure (false). The rehearsal (setup PR) would show it, costing a fix cycle. Fix (script only, changes_approved_text false): in checkForm, skip /index.html (already '/'), fetch /404.html as /404, and judge it by its manifest name (pass the original path to judge so the .html isolation target stays); never judge the redirect itself; record the mapping. Make local-proof/serve-headers.mjs answer *.html with 308 to the extension-less path so proof.mjs proves the fix.
F2 Medium, deploy.yml:158 and close.yml:98 (retention 7) with probe.yml:48 (expired==false). Plan risk 8: window up to 10 days. Scenario: a sweep or reading more than 7 days after a deploy lists fewer records, probes fewer URLs, no error: PM-0006-8 (leftover URLs) or a late PM-0006-6 reading shrinks silently. Fix (changes_approved_text true): diff
  deploy.yml  name: deployment-record-${{ github.run_id }}-${{ matrix.leg }} / path: records/ / -retention-days: 7 / +retention-days: 14
  close.yml   name: deployment-record-${{ github.run_id }}-tombstone / path: records/ / -retention-days: 7 / +retention-days: 14
  No-text alternative: copy every run's record (deployment id, URLs) into the results log and compare the probe's row count with it before each reading; hold the PM-0006-8 sweep within 7 days of the oldest deploy it needs.
F3 Low, close.yml:76-78. Inferred: CF list docs state no per_page max/default (read 2026-11-15). If CF clamps per_page below 25, loop stops after page 1 and older uploads stay (PM-0006-6 false "serves"); if CF rejects the value, curl -f fails the step loudly before any deletion (the setup PR merge shows it). Fix: no text change now; session compares "Uploads of <branch> found: N" with that branch's records before the 24 h reading. Text fix only if it shows: break on .result_info.total_pages.
F4 Low, tools/wdio/run.mjs:52-53. Inferred: if Pages' Browser Integrity Check (cannot be turned off) answers HeadlessChrome with 403/1010, the page never sets data-done and the row says only "did not finish within 30 s". Fix: on timeout record session.getTitle() and a short body snippet (no headers/cookies).
F5 Low, deploy.yml:52. echo "legs=$(node ...)" hides a sets.mjs exit 2; build then errors at fromJSON('') evaluation: run still fails, message less clear. No mark lost. Defer.
F6 Low, deploy.yml:135. `test -n "$BRANCH" && test ...` under set -e: an empty production_branch does not stop the step (first test of an && list is exempt). Harmless here (wrangler with no branch outside git deploys production, the intended target). Defer.

Verified OK (file:line or command):
- npm install --prefix into a missing folder creates <prefix>/node_modules/.bin/<bin> (npm 10.9.9, Node 22.23.4, run locally) -> deploy.yml:141 and close.yml:47 wrangler path exists.
- wrangler 4.156.0 engines node >=22.0.0 (npm view, 2026-11-15); ubuntu-24.04 default Node 22.23.4 (docs notes, image 20261111) -> token jobs without setup-node are fine.
- Wrangler output entry pages-deploy-detailed has deployment_id, url, alias, environment, production_branch, timestamp (docs notes; 4.156.0 deploy.ts) -> deploy.yml:143-149 fields exist; file truncated each loop (:140).
- Production branch from GET project .result.production_branch (:131-133); curl failure stops via pipefail.
- Matrix: sets.mjs prints one JSON line of {leg,mode,branch,expect,count}; workflow reads matrix.leg/mode/branch/expect/count; artifacts site-<leg> written (:78) and read (:101) match.
- Records: deploy writes records/$RUN-$LEG-$i.json flat; verify pattern deployment-record-<run>-* merge-multiple flat; probe unzips each deployment-record* into records/<id>/; lib.mjs readRecords walks recursively -> both layouts read.
- download-artifact pinned SHA source (raw, 2026-11-15): pattern with zero matches succeeds, 0 downloaded; readRecords on a missing folder returns []; check-headers prints "no upload records" and exits 0 -> "revoked" run: deploy red (the reading), verify green with 0 records. Not a lost mark.
- check-sizes never fails (comment + no exit) -> bite-oversize reaches Wrangler; deploy fail-fast false keeps other legs; verify always() runs.
- close.yml: env=preview listing, branch match on deployment_trigger.metadata.branch (raw --branch), sort oldest first, DELETE ?force=true, tombstone kept by id, tombstone record uploaded as deployment-record-<run>-tombstone; probe treats marker "" as gone.
- probe.yml: gh api --paginate artifacts + /zip (actions: read granted); pulls with --jq object output: verified compact one object per line on a reachable endpoint (piped) -> probe.mjs line parse OK. Schedule needs the file on main (plan merges setup first).
- browsers.yml: WDIO 10.1.0 @wdio/utils build/node.js read: chromedriverOptions.binary used; edge/gecko binary -> custom driver path; no browserVersion -> preinstalled browser (Windows version from the Chrome folder name); safaridriver started by WDIO after sudo safaridriver --enable. Lock has no install scripts, so npm ci --ignore-scripts drops nothing. Driver file names chromedriver.exe, msedgedriver.exe, geckodriver.exe join with the env folders.

Per workflow, could a mark become unmeasurable?
- deploy.yml: PM-0006-2 yes, today, as a false fail (F1) until the script is fixed; PM-0006-5, -7, -10 measurable.
- close.yml: PM-0006-6 deletions measurable; F3 only if CF clamps per_page (inferred).
- probe.yml: PM-0006-6 and -8 measurable within 7 days of each deploy; beyond that silently shrinks (F2).
- browsers.yml: PM-0006-3 and -4 measurable; a block would show as "no reading" rows, never a pass (F4 adds the reason).
Not verified: anything on Cloudflare or GitHub runners themselves (nothing has run); the 308 code; Cloudflare per_page limits.
DONE
