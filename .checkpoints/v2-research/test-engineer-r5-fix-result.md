# R5 wave 2 fix step: the owner's report, verbatim

Workflow run wf_888e5b21-090 (prompt `prompts/test-engineer-r5-fix.txt`), returned 2026-10-29 (about 21:40 UTC). Notes: `test-engineer-r5-fix.md`.

I applied the R5 spot-check fixes to `/home/user/Shamble-Frontlines/docs/research/r5-toolchain-quality.md`, in place. Nothing was committed or pushed.

**Changes by type:** 34 nit · 8 recommendation · 1 number · 0 source.
- **Recommendation (8):**
  - R5-TEST-REC1: Safari row fixed; benchmark row added.
  - R5-TEST-REC2: real Safari now through its WebDriver (safaridriver or WebdriverIO), not Playwright.
  - R5-CI-REC5: the final job always runs and fails unless every job it needs succeeded.
  - R5-RUSTLINT-REC3: the sim does not enable the three lints.
  - R5-GATE-REC2: new reason; the command path is a working name.
  - R5-TEST-REC4: the L2 round-trip mark now uses R2's own form.
  - R5-PREV-REC2: notes that a branch's latest deployment cannot be deleted.
  - R5-TEST-REC6: one fixed seed.
- **Number (1):** the 200 KB mutation for check G7 is dropped until a size budget exists.

**Became an open question:** none. I added one new open question, R5-TEST-OQ8 (cargo-nextest, coverage and the bench tool were not researched).

**Re-sourced:** none. I re-read the Prettier README, which was already a source, only to quote F22's language list.

**Not applied:** none. One partial disagreement: R5-LAYOUT-REC5 keeps §6.2 beside §6.9 and §4. Its line is about the sim running in a Worker, and R1's option A2 traces that Worker to those three. The reason is in part 7 and my notes.

**Final header status:** spot-checked (60 of 115 key claims).

Files are in /home/user/Shamble-Frontlines/.checkpoints/v2-research:
- verifier-r5-result.md
- test-engineer-r5-fix.md (ends with DONE)
