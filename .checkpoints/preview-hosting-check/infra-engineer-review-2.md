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
