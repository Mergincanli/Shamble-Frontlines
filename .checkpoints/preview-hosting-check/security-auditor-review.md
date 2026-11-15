Task: security-auditor review of the preview hosting check build (Mergincanli/shamble-preview-check, branch setup, head 1b37fae), single round; area: token and CI security, B9 tests (PM-0006-32), owner's condition 2 (deployments: write ruling).

## Plan (blocks, in order)
B1. Read the owner's go (conditions) and plan Part 4 + Part 7 (incl. 7.5 B9) + my G6 notes in security-auditor-docs.md.
B2. Review deploy.yml (token, env, checkout, install, refuse-server-code, deploy, tombstone, permissions).
B3. Review close.yml (tombstone/cleanup scope).
B4. Review probe.yml and browsers.yml (no secret, permissions, triggers).
B5. Review tools/ scripts for printing headers/cookies/bodies; build.mjs _headers; wdio lock.
B6. B9 variants vs PM-0006-32 (ADR 0006 lines).
B7. Ruling on deployments: write.
B8. Write findings + coverage table. DONE.

## Notes
