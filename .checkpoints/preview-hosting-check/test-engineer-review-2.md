Task: test-engineer round-2 review of shamble-preview-check fix pass 2df0112 (vs round-1 head 1b37fae), cs preview-hosting-check.

Plan (blocks, in order):
B1 read round-1 notes test-engineer-review.md + checkpoint step 15 claims
B2 read git diff 1b37fae 2df0112 (stat + full)
B3 review check-headers.mjs + lib.mjs (TE-1/F1 fix)
B4 review probe.mjs, wdio/run.mjs, site/page.js, site/index.html
B5 check workflow call sites (command lines, file names) vs changed scripts
B6 run local proofs (proof.mjs, browser-proof.mjs) and mutation checks
B7 write findings + verdict
