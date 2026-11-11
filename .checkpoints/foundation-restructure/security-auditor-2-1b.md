# security-auditor, step 2.1b (PR #51), round 1

Head seen: dc8582af455036fcc06568c403ec959c9f56de2d (expected dc8582a: matches). Base 76b4f7a.

## Plan
1. Kit lines: diff CLAUDE.md, verifier.md vs approved before/after (kit-check.mjs, plan).
2. Guard: guard.mjs / routine-guard.mjs protected paths, GATE_FILES, any root-path assumptions.
3. Gate move check gate.mjs:242-249; decoy/stray root page; letter case.
4. README build-folder paragraph vs smoke.mjs findBuild/LEGACY_JS, gate.mjs BASE_PREFIXES/basePrefix.
5. saves-console-lines.md: both lines, read/write, network, delete, restore-line eval of pasted text.
6. DECISIONS.md entries byte-check (decisions-entries.mjs).
7. Rest of diff skim (README, tools headers, docs) for secrets/loosened rules.

## Findings (a line each, as found)
