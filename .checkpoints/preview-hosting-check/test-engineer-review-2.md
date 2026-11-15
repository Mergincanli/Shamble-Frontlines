Task: test-engineer round-2 review of shamble-preview-check fix pass 2df0112 (vs round-1 head 1b37fae), cs preview-hosting-check.

Plan (blocks, in order):
B1 read round-1 notes test-engineer-review.md + checkpoint step 15 claims
B2 read git diff 1b37fae 2df0112 (stat + full)
B3 review check-headers.mjs + lib.mjs (TE-1/F1 fix)
B4 review probe.mjs, wdio/run.mjs, site/page.js, site/index.html
B5 check workflow call sites (command lines, file names) vs changed scripts
B6 run local proofs (proof.mjs, browser-proof.mjs) and mutation checks
B7 write findings + verdict
Notes:
- B1 done: round-1 TE-1..TE-13 read; fix pass claims F1/TE-1, F4, SA-R7, TE-2..6, TE-8..10; proof.mjs 29/29 with redirecting server. Next B2 diff.
- B2 done: diff = 6 files (check-headers NOT_FETCHED + manifestOf stale check + exit 3 on 0 records + tries; lib isTransient 429/challenge, location origin+path; probe 'other' class, exit 3, by-hand/production expect serves under deletion, wall count line; run.mjs done!=='1' -> no reading for every link, bite caught iff page true & worker not true, mode/marker in log; page/index mode field). To check: workflows' handling of new exit 3 (artifact upload if: always?), record field names mode/marker vs manifest, done='error' now no-reading for all links (weakening?), page.js on worker failure in Firefox/Safari. Next B3.
- B3 done: check-headers/lib read. Record fields mode/marker (deploy.yml jq) equal build.mjs manifest mode/marker -> stale check wired right; one-off link never stale; bite-noindex vs production modes differ so TE-6 scenario caught; uploads set (10 same mode+marker) indistinguishable but same content (harmless). NOT_FETCHED drops index.html/404.html; "/" covered by /* rules. Notes: manifestOf sleeps 25 s after its 5th (last) try (wasted, Low); 429 now retried 30 s per fetch -> uploads set (20 forms) under rate limiting can pass verify's timeout-minutes 10 with headers.json never written (written only at end) (Low, inferred). 404 response headers no longer judged at all (Low coverage note). Next B4.
- B4 notes: run.mjs:89 done!=='1' -> 'no reading' now for EVERY link, not just bite-coep: a page that finishes with done='error' on iso-on/production (was 'not as expected', exit 1) is now exit 3. page.js:47-49 sets 'error' only when main() throws (manifest.json not JSON / fetch reject; worker failure still resolves to done=1), so reach is small. Low (inferred).
- B4 notes: TE-9 only partly fixed: run.mjs:98 logs mode/marker, never compares them with the link's expected mode; for the PM-0006-3/-4 links a stale production alias (bite-noindex) has the same isolation and cache headers, so impact tiny. Low (partial).
- B4 notes: probe.mjs under deletion now expects 'serves' for by-hand previews and production stable (:85); 'rehearsal' case for setup. Wall-count line (:101-104) counts no-PR (by-hand) branches as "open branches" and is console-only, not an exit code or a row field. Low.
- B4 notes: TE-3 fix right (run.mjs:90). page.js/index.html mode field consistent (id 'mode'). lib: lower now a function decl (harmless), hostOf moved before isWall (harmless); location origin+path keeps isWall working.
- B4 done; B5: workflows: every artifact upload is if: always() (browsers.yml:47, deploy.yml:154/181, probe.yml:62) -> new exit 3 turns jobs red but keeps artifacts. Command lines unchanged and match scripts' args. Next B5 detail (revoked expectation in README), then B6 runs.
- B5 done: revoked run (plan :436) must fail at sign-in anyway; verify now exit 3 on 0 records (red, artifact kept) - no wrong reading. Command lines in workflows match scripts (deploy.yml:179, probe.yml:60, browsers.yml:45). Next B6 runs.
