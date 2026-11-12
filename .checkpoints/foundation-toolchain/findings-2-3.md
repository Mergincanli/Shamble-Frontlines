# Foundation 2.2: findings for step 2.3 and follow-ups for later steps (the session's list, 2026-11-12)

Each item: a short name, what was found, the evidence, and where it goes. His words are not retyped here: where an item
rests on them, it names the report's Appendix A item (`docs/reports/2026-11-12-foundation-2-2.md`).

## Findings for step 2.3 (the gate v2; the kit unfreezes)
1. **AUTO-ASK** (his KIT-ASK-SEEN (b), A5): auto mode can pass the guard's ask. K1's write (2026-11-10 20:02:42 UTC): the
   guard's ask was answered "allow" six seconds later, the session log not saying by whom. Step 5's five kit writes
   (2026-11-12), made in the mode he chose as Accept edits: his count, five prompts of five (A11).
2. **MANUAL-MODE**: CLAUDE.md (the change-set protocol, step 3) says to make a `.claude/` edit "in Accept edits or Manual
   mode", but the cloud app offers Accept edits, Plan or Auto, with no Manual mode (the environment's documentation page
   on permissions, read 2026-11-12), and the session record called the mode he chose "default".
3. **GIT-C**: `.claude/CHECKPOINTS.md` §10 (:221) tells a session to check `git -C <path> status --short --ignored`,
   which the guard refuses ("no option before the subcommand", `routine-guard.mjs:1046`); sessions `cd` into the folder
   instead.
4. **ADVISOR-TOOL-WORDS**: CLAUDE.md takes his words of 2026-11-11 on the advisor tool at 2.3 (queued in DECISIONS.md).
5. **MODELS-2026-11-12** (A9, item 3): CLAUDE.md's "Models and effort" section and the agent files change to match his
   words of 2026-11-12, 16:25, each edit asked of him in chat with its exact text (queued in DECISIONS.md).
6. **REFUSED-PUSH** (A10, item 4): the first push of step 5's merge, written as a compound line (`git push -u origin
   <branch> 2>&1 | tail -4 && date ...`), was refused by the session's permission check with no reason given; the
   guard's own push rule allows that push; the plain `git push origin <branch>` then passed.
7. **PIN-GUARD** (his answer, A1): the guard does not protect the new pins (`rust-toolchain.toml`, `Cargo.toml`,
   `Cargo.lock`, `package.json`, `package-lock.json`): `protectedRel` false, so routines may write them on a branch and
   day sessions get no ask.
8. **G0** (the register row Q2.2-a: the advisor agent's ruling, final; his go for the row: PR #48's merge): G0's
   exit-2 mark moves to step 2.3, where 2.3's G0 is to prove "deleting the toolchain file fails G0 with exit 2" under
   `--prove` (PM-0002-39); until then the toolchain job fails closed without the file (M2).
9. **OVERFLOW-WASM** (S1's remainder): PM-0002-53 wants the overflow rule in four builds (dev and release, native and
   Wasm) with a committed counter test; 2.2's CI proves the native release build for `protocol` (M5; `sim` locally).
10. **TSCONFIG** (S2): TypeScript is pinned but never type-checked: no `tsconfig`, CI prints `tsc --version` only. ADR
    0002 part 7 gives the shared strict base tsconfig and `tsc -b` to the lint step (G3). The stubs import `./index.ts`,
    a convention `tsc -b` would need `allowImportingTsExtensions` (with no emit) or `rewriteRelativeImportExtensions`
    for: 2.3 decides it.
11. **REQUIRED-CHECK**: whether `toolchain` is a required check on main (step 7's infra-engineer answers); the file's
    header says 2.3 replaces it, and ADR 0002 gives 2.3 one aggregate required check (:642, PM-0002-57).
12. **SETUP-NODE-CACHE** (S4): setup-node v7.0.0 runs with `package-manager-cache: true` (the job log); what it caches
    if a `packageManager` field is added to the unprotected root `package.json` is unverified (step 7's
    security-auditor).
13. **SAVER-SKIP-CI**: the checkpoint saver marks a push of `.checkpoints/` alone `[skip ci]`, so a ready head made by
    the saver would carry no checks; the last commit before ready (the notes' deletion) is made by hand.
14. **NODE-26** (the advisor agent's Q2.2-c ruling): `tools/` and v2 move to Node 26 together in one change-set after
    2.3, merged before 2027-03-31 (Node 22's end of life is 2027-04-30); the toolkit's `package.json` and lockfile are
    gate files, so it asks him.

15. From consultation 4 (`findings-2-3-c4.md`, merged into the archive): **CRATE-NAMES**, **CI-ENFORCED-LATER**,
    **WASM-ARTIFACT**, **TWO-PLAYWRIGHTS**, **TOOLKIT-RESOLVES-UP** (also a Risk), **CARGO-NET-OFFLINE**,
    **CRATE-LICENCE**: their text is in `findings-2-3-c4.md` and the archive's "From foundation 2.2".

## Follow-ups for later steps
- **OVERFLOW-PER-CRATE** (S3): the release overflow setting is per package; a crate with simulation arithmetic that
  enters later (`content`, a maths crate) needs the same block (the first slice).
- **PACKAGE-ENTRIES** (S5): the packages have no `exports` or entry fields (the step that gives them code).
- **EMPTY-SLOTS** (plan Part 3): OQ-0002-7 and the wasm-bindgen pin; OQ-0002-3 (one Wasm instance or two); OQ-0002-4
  (isolation); OQ-0002-1 (the editor from `file://`); the boundary spike's items 4 (generated types) and 8 (sizes,
  wasm-opt, `skipLibCheck`, the three.js casts, Playwright's clock): each waits for the boundary spike.
- **BRIEF-STALE**: `docs/V2_PHASE0_BRIEF.md:20-21` (no engine code in phase 0 but throwaway spikes) and `:144` (its
  "Don't" list) still forbid engine code in phase 0, with no word of the exception for steps 2.1 and 2.2; it is
  Mustafa's and the Advisor's brief (its line 3), one of DOCS-ALIGN's documents, so no session edits it.
