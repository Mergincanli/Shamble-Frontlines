Task: infra-engineer round-2 review of shamble-preview-check branch setup, fix pass 2df0112 vs round-1 head 1b37fae (F1/TE-1 fixed? anything broken in check-headers, lib, probe, wdio/run, site/page.js, site/index.html?).

## Plan (blocks)
B1 read git diff 1b37fae 2df0112 + round-1 SA/TE findings + checkpoint step 15 claims
B2 review check-headers.mjs + lib.mjs (F1/TE-1) against build.mjs manifest and deploy.yml call lines
B3 review probe.mjs, wdio/run.mjs (F4), site/page.js, site/index.html vs workflows
B4 run local proofs (proof.mjs, browser-proof.mjs) if time
B5 write answer

## Notes
- B1 diff read (6 files). next: SA/TE findings + checkpoint step 15
- B1 done (SA/TE findings + checkpoint step 15 read: fix pass claims F1/TE-1, F4, SA-R7, TE-2..6, TE-8..10). next B2
- B2 done (check-headers + lib read in full vs build.mjs, deploy.yml:145-149, :179). F1 fixed in code: manifest paths minus index.html/404.html; '/' still judged (index.html's response, isolation target); remaining paths page.js, page.css, worker.js, tiny.wasm, content.<hash>.bin, manifest.json: none .html. record has marker+mode (deploy.yml:147-148) = manifest marker+mode (build job MARKER github.sha, same run) -> stale check matches in normal runs; uploads set (10 same mode+marker) matches. 404.html not judged: ADR PM-0006-2 method = URLs the page loads; 404.html not loaded -> acceptable (note only). lower() now hoisted function; exports intact for both importers.
- verify exit 3 on 0 records: revoked run's verify now red (deploy red anyway); plan D6 does not need verify green -> OK. headers.json written before exit; upload if: always() (deploy.yml:181).
- Stale loop cost: <=75 s sleep per stale form, verify timeout 10 min (deploy.yml:165); no within-run sibling stale case in sets.json -> OK.
- B3 partial: probe.mjs read; probe.yml:60 call unchanged, upload if: always() :62, out written :95 before exit :106. New: exit 3 when 0 rows -> hourly scheduled runs red until the first record exists (and on any no-reading) -> GitHub failure notifications (Low noise). Deletion purpose now expects 'serves' on by-hand previews and production stable (was null).
- run.mjs read: F4 fixed (title + 200 chars innerText on timeout, :58-63). readPage t('mode') needs #mode in index.html (added) - any deployment built before 2df0112 would throw -> no reading; none exist (nothing deployed). page.js show('mode') after manifest fetch; fine.
