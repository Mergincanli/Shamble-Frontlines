# Verifier notes, change-set 4b-1 "Content", final code 2039e3c

## Plan
1. Full local checks -> `.gate/verifier-local-checks.log` (started); read test counts, packages, boundary, browser, wasm size/sha.
2. Fuzz histogram + caps table: release `content_fuzz -- --nocapture` (after 1).
3. Corpus `--check`, own count of parts, own script for ties and both signs of 2^-24.
4. legacy-extract `--check`; time report vs "68 of 69; 600 and 60".
5. numstat vs origin/main, added lines minus 7 data files = 7,164? Also the per-step and 47-file / 36,429-line claims.
6. decisions-4b1.mjs check; every quote of his in the draft vs docs/DECISIONS.md.
7. plants: 5 of plants-4b1.txt vs .gate/plants-4b1.log; two own plants in .gate/verifier-4b1-scratch/.
8. reviews: each reviewer's findings vs the draft's tables; spot-check >= 6 "fixed" rows in code.

## Results
