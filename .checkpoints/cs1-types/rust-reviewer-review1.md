# rust-reviewer, round 1 review of cs1-types (foundation step 2.4 "Types")

Head 9b8c668 (code = 82142fa), base main 22c6afa. Branch claude/wonderful-hamilton-jdmlnf.
No earlier rust-reviewer memory found (.claude/agent-memory-local has security-auditor only).

## Plan
1. Read the diff (Cargo.toml, crates/protocol, crates/client-endpoint, packages/types), plan.md, plan-draft.md
   (s4, s5, s7), cs1-types.md build notes + steps 6/7, plant-results.txt, runs20-results.txt.
2. Read ADR 0002 part 5 (block, layout table, hash), parts 4 and 7 (lints, overflow), ADR 0004 hasher/float rules.
3. Check 1: layout.rs hash vs RFC 9923 FNV-1a 32 (basis 0x811C9DC5, prime 0x01000193, xor then multiply), const eval,
   stream unambiguity, no usize, wasm32 vs native; public API shape.
4. Check 2: broken_rule + fixtures: alignment, stride, overlap, names, overflow, rules that cannot fail.
5. Check 3: layout_ts.rs determinism, paths, write mode, deletes, links, platform differences.
6. Check 4: messages.rs derives; cargo tree normal edges native + wasm32; cargo tree -i ts-rs.
7. Check 5: ts_export.rs scratch folder, files_in, two-way compare, cleanup; threads, --release, wasm32 clippy.
8. Check 6: manifests.
9. Check 7: fmt, clippy (native, wasm32), pedantic for info; rustfmt::skip; doc comments vs code.
10. Write findings as I go; DONE at the end; handback <= 40 lines.

## Findings (one line each, as found)

## Commands run (exit codes)

- (info, checked right) RFC 9923 = "The FNV Non-Cryptographic Hash Algorithm" (Feb 2026, Informational): basis 0x811C9DC5,
  prime 0x01000193, xor then multiply (rfc-editor.org/info/rfc9923, Tables 1-2 and pseudocode). layout.rs:63-64,98-105 match.
  Independent Python FNV-1a over the stream the doc states (layout.rs:69-71) gives 0x2df78c30 = layout.ts's LAYOUT_HASH.
- (info, checked right) Stream is prefix-decodable: tag byte (0x02/0x01) never occurs in a plain name, names end at the
  first 0x00, numbers are fixed 4 bytes LE, type names have no 0x00 -> injective for tables that pass broken_rule.
- (info) ts-rs 12.0.1: Config::new() == Config::default(), reads no env (ts-rs-12.0.1/src/lib.rs:593-609); only
  from_env() reads TS_RS_* (lib.rs:618-640); the auto test exists only with #[ts(export)] and uses from_env()
  (ts-rs-macros-12.0.1/src/lib.rs:41-43,189-210); imports sorted via BTreeMap (src/export.rs:336-377): deterministic.
  export_and_merge keeps a process-global map path->types and SKIPS a type already written to that path in this process
  (src/export.rs:20-24,142-167): safe here only because every run gets a new scratch path. File::create follows links
  (export.rs:154): safe because the scratch folder is new and exclusive.
- (info) cargo tree: protocol -e normal (host, wasm32, --target all) = protocol alone; -i ts-rs = dev-dependency of protocol only.
