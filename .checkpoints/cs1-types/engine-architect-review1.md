# engine-architect, round 1 review of cs1-types (foundation step 2.4, "Types")

Head reviewed: code of commit 82142fa (branch claude/wonderful-hamilton-jdmlnf); base main 22c6afa.
Memory: no engine-architect memory file exists (.claude/agent-memory-local/ holds security-auditor only).

## Plan
1. Read the diff (Cargo.toml, crates/protocol, crates/client-endpoint, packages/types), the approved plan
   (plan.md over plan-draft.md), my plan notes, the checkpoint's build notes, rust-reviewer's answer.
2. Read the drafts: ADR 0002 part 3 (lines ~137-217), part 5 (~284-360), migration step 4, PM-0002-15..19, -43, -63;
   ENGINE_OVERVIEW (block, generated types); build plan card 1 and Part 7; part-15 file (lanes).
3. Check 1: beyond-card / missing; public symbols and tests vs plan-draft section 4 + plan item 5(g).
4. Check 2: BLOCK_LAYOUT as the one description; what CS2's writer and the page need; hash coverage.
5. Check 3: the three deviations.
6. Check 4: packages/types shape, exports, naming scale, checkLayoutHash's home, ts-rs output style.
7. Check 5: ADR departure (no TS_RS_EXPORT_DIR/.cargo/config.toml, no #[ts(export)]); pointer text;
   R5-LAYOUT-OQ4, R5-TSLINT-OQ3 answers.
8. Check 6: cargo tree --locked; imports in packages/types.
9. Check 7: PM-0002-15/-16/-17/-19/-63 met vs shown.
10. Answer, at most 45 lines, via SubagentHandback.

## Findings (a line each, as found)

- (info, check 6) cargo tree --locked --offline: client-endpoint has no edge of any kind (-e normal,build,dev --target
  all); protocol has no normal or build edge on any target; ts-rs is reached only as protocol's dev-dependency (-i
  ts-rs -e all --target all); wasm32 normal edges: the six crates alone. packages/types imports only its own files
  (index.ts, layout-check.ts, generated/protocol siblings) and vitest in tests; its package.json has no dependencies.
- (info) tsc 6.0.3 by path: .gate/cs1-types/tsconfig.json exit 0, tsconfig.shipped.json exit 0 (lists the 9 generated
  files, layout-check.ts, index.ts). Both configs carry A2:421-426's flags plus noEmit, allowImportingTsExtensions,
  module preserve, target/lib es2023, types []; tests config adds skipLibCheck.
- (info) symbols: everything in plan-draft s4 and plan 5(g) is built under its planned name; added public items are
  ElementType::name(), ElementType::size() and the derives (Debug, Clone, Copy, PartialEq, Eq) on the three table
  types; added private: fnv1a, FNV consts, SECTION_TAG/FIELD_TAG (deviation 1), test helpers (field, section,
  name_is_plain, broken_rule, GOOD, BAD, BASE, SECOND, CHANGED), write_generated x2, new_scratch_folder, files_in,
  GENERATED_FOLDER/FILE_NAME. Removed as planned: stub_builds, PACKAGE. Missing: none. Renamed: none.
