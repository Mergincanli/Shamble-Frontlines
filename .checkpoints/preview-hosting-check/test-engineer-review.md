Task: test-engineer review, preview-hosting-check build (branch setup of shamble-preview-check, head 1b37fae), single review round.
Plan (blocks, in order):
B1 read marks: ADR 0006 1366-1376, 1198-1222; plan Part 1/1.1; GLOSSARY 636-665
B2 review tools/build.mjs + lib.mjs + check-headers.mjs vs GLOSSARY
B3 review probe.mjs + sets.mjs/json + links.json
B4 review wdio/run.mjs
B5 review workflows (deploy/close/probe/browsers) + check-sizes.mjs
B6 review local proofs coverage
B7 write findings + DONE
Notes:
- B1 done: marks read (ADR 1366-1376 table PM-0006-1..10, -32; spike 1198-1222), plan Part 1/1.1/2, GLOSSARY 636-665 (preview set = production set + noindex; wasm application/wasm; COOP/COEP only with flag; CSP not checked).
- B2 notes: check-headers.mjs:50 fetches /index.html and /404.html from manifest; Pages 308s .html to extension-less (infra-engineer found same, serving-pages page Nov 11 2026); lib.mjs:169 redirect manual -> judge :22 'status 308' -> every link form FAIL -> PM-0006-2 cannot pass (High; independently confirming below).
- B2 notes: check-headers ignores the page's Resource Timing list (ADR method: page lists resources, script fetches each); uses manifest instead -> a resource the page/worker loads that is not in manifest escapes (Low; manifest is superset today).
- B2 notes: header expectations vs GLOSSARY: wasm type, noindex preview/none prod, COOP/COEP isolation targets /,html,js,wasm, none elsewhere; immutable on bundle. Matches. Preview noindex check cannot fail as host adds noindex itself (not a mark gap; record).
- B2 done; next B3 probe.mjs + sets.
- B3 notes: probe.mjs:38-43 'gone' = any non-wall, non-200-with-marker answer incl. 429 (rate limit), 403 challenge (not 1010), 401, other 4xx -> counted as gone/no build, as_expected true (:84): a 'no reading' read as PREV-DELETE pass. Mark text literally allows "any other response without the marker", but task asks: Medium; control (c) partly guards.
- B3 notes: probe as_expected null on no-reading; summary 'unexpected: 0' with all rows no-reading looks green; need probe.yml exit code check (B5).
- B3 notes: wall purpose has no control expectation (policy-off sweep shows as_expected false everywhere = control passing); 'at least 2 open branches' count not checked by script; 'no build' accepts 'gone' so wall mark can pass with no wall seen if open previews 404 -> control must be same-day. Low/Medium.
- B3 notes: completeness of URL set rests on records: a deployment without a record (deploy run failed after upload, or record expired at 7 d) is never probed -> PM-0006-6 "every deployment it ever had" can pass with URLs unprobed. Check deploy.yml/close.yml (B5).
- B3 done; next B4 wdio/run.mjs.
- B4 notes: run.mjs:79 bite-coep 'bite caught' on anything but true/true, no done==='1' and page==='true' check -> a broken bite deployment (manifest fetch error -> done=error, worker 'waiting'; or page not isolated false/false) reads as bite caught: bite that cannot fail. Medium. Fix: caught only if done==='1' && page==='true' && worker!=='true'; else no reading.
- B4 notes: run.mjs reads marker (:58) but never compares it with the deployment's commit: ADR's reason for 3 loads (stale older deployment) is not tested; 3 identical stale loads pass. Low (marker recorded; log can compare).
- B4 notes: cache test run.mjs:91-107 matches ADR (first visit, tab closed, second visit, transferSize, 3 fresh sessions, bite no-cache >0). null entry -> no reading (good). Race: tab closed right after done; disk-cache write may lag -> spurious fail (flake to root-cause, not false pass). Low.
- B4 done; next B5 workflows.
- B5 deploy.yml notes: marker = github.sha (deploy.yml:71,122): every leg of one by-hand run (production, iso-off, iso-on) and every rerun from the same main commit share one marker -> marker cannot tell deployments apart on by-hand sets. check-headers never compares the served manifest's marker/mode (build.mjs:91-95 writes them) with the record -> a stale alias (production alias reused by production, bite-noindex, restore-production) is read as the new deployment. Medium.
- B5 deploy.yml notes: check-headers with 0 records exits 0 (check-headers.mjs:72,86) -> verify green with no reading when every deploy failed. Low (deploy job red; revoked run expects 0).
- B5 deploy.yml notes: records only written after wrangler success + exactly 1 detailed line (deploy.yml:143-149); an upload whose record is missing is never probed by PM-0006-6. Low-Medium inferred.
- B5 deploy.yml notes: check-sizes output only in job log (no artifact); tombstone uploads (close.yml) skip check-sizes. Low.
- B5 probe.yml/probe.mjs notes: probe.mjs never sets an exit code -> probe job green whatever the rows (all no-reading, all unexpected). Verdicts rest on reading the artifact. Medium (no-reading readable as pass by a glance at green runs); fix: exit 1 if any as_expected false, 3 if any no-reading.
- B5 close.yml notes: probe URL set = records (deploy/close artifacts), deletions = CF list (close.yml:75-83); no cross-check of "Uploads found" vs records per branch -> an unrecorded deployment's hash URL is never probed. Low.
- B5 browsers.yml: run.mjs exit 1/3 -> job red (good). Safari: no headless, safaridriver --enable (fine). wdio rows carry no deployment id (marker shared across legs) vs PM-0006-1 "every later row names its deployment". Low.
- B5 done; next B6 local proofs.
