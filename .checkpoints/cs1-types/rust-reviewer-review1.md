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
- (info, checked right) Same hash on every target: const assert LAYOUT_HASH == 0x2df7_8c30 compiles for wasm32 and native
  (scratch copy .gate/rr-cs1, release); run time on wasm32 (Node) and native (ctypes) both 0x2df78c30.
- (info) Wasm size: no shipped .wasm yet (sim-wasm is an empty stub, no dep on client-endpoint). Proxy cdylib of
  client-endpoint, rustc 1.99.1 release, no wasm-opt, gzip -9 (Node zlib): const-only export 349 B raw / 302 B gz;
  with a run-time layout_hash() call 1,343 / 772. Keep writing the const.
- L-a (Low) layout.rs:124-132: names like delete/enum/default/package/class/new pass broken_rule (probe test in scratch,
  ok), but layout_ts.rs:26 prints `export const <name>` -> invalid TS; doc says "printable as TypeScript names".
- L-b (Low) layout.rs:154,157,160: `at + size` / `offset + size` overflow for offsets > u32::MAX-4; overflow checks on
  in every profile (Cargo.toml profile.release) -> panic "attempt to add with overflow" (probe offset 0xFFFF_FFFC, debug
  and release, exit 101). Never a silent pass; names no rule.
- L-c (Low) ts_export.rs:18-21: any create_dir error = "name taken"; TMPDIR=/nonexistent-rr -> 100 tries then panic
  "a scratch folder can be made in the temporary folder" without path or io error (exit 101).
- L-d (Low) layout_ts.rs:41-44, ts_export.rs:45-48: write-mode guard ends_with(GENERATED_FOLDER) is a tautology (folder
  built from the same const at layout_ts.rs:59-61, ts_export.rs:77-79); ../.. = repo root is unchecked. Also
  client-endpoint's compare follows a link at layout.ts (layout_ts.rs:67-84) where protocol's files_in refuses links.
- L-e (Low) 15 public items without docs (missing_docs run): messages.rs:49,50,79,80,87,90,97; layout.rs:9,10,36-38,44-46.
- Commands so far: fmt --check 0; clippy native -D warnings 0; clippy wasm32 -D warnings 0; pedantic (both crates) 0,
  no warnings; cargo test --workspace 0 (ce 5, protocol 2); test --release -p protocol -p client-endpoint 0; 8 parallel
  runs of the protocol export binary all ok, 0 leftovers; wasm32 build of 5 crates 0; cargo doc -D warnings 0;
  cargo tree checks 0; cargo audit/deny: not installed (not run).
- L-f (Low) Deviation 1's tag bytes have no test that bites: scratch copy with both tag lines removed, layout.ts
  regenerated (0xafcb2f9a): the 5 committed client-endpoint tests pass; only my probe fails. Pair valid under
  broken_rule, one stream without tags (Python: both 0x60d6dbf4; tagged 0x4265359c vs 0xbd51cd2f):
  A=[section("a",0xFFFF_FFFF,[field("b",U8,0x7a79_7800),field("c",U8,0)])],
  B=[section("a",0xFFFF_FFFF,[]),section("b",0x0000_3875,[field("xyzc",U8,0)])]. Add as a must-differ pair.
- L-a evidence: node 22.23.4 `export const delete = ...` -> SyntaxError Unexpected token 'delete'; enum -> Unexpected
  reserved word. Field names are object keys: reserved words fine there.
- Forward F1 (cs2 writer): no by-name handle (positional fields[1] retargets on reorder) -> const fn offset(section,
  field) with a compile-time panic; Section::align() (computed twice: layout.rs:139-140, layout_ts.rs:22-23); section
  start padding in the block not representable in the table: one owner + a test in cs2/cs4.
- Forward F2 (cs3 codec): Join.token, mode_id, Input.acks/commands sender-sized -> cap before allocating; token as a
  credential: no derived Debug (redact), no derived == for auth.
- Forward F3: temp_dir() and process::id() trap on wasm32-unknown-unknown (scratch, Node: RuntimeError unreachable):
  gate ts_export/layout_ts with not(target_family = "wasm") if a wasm32 lane ever runs these #[test]s.
- Verdict: APPROVE (no finding at Medium or above; six Lows L-a..L-f).
DONE 2026-11-18 (scratch copy left at .gate/rr-cs1, gitignored).
