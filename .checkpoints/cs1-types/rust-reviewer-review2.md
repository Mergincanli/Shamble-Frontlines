# rust-reviewer, round 2 review of cs1-types (foundation step 2.4 "Types")

Code = 65a7b13 (branch claude/wonderful-hamilton-jdmlnf, head 6c3a24a); round 1 reviewed 82142fa; base main 22c6afa.
Round 1: APPROVE with six Lows L-a..L-f (rust-reviewer-review1.md).

## Plan
1. Read `git diff 82142fa..65a7b13 -- crates` and `-- packages/types/src/generated`; fix-pass-1.md.
2. Round-1 Lows L-a..L-f: closed or not, one line each, with file:line.
3. New code: repository_root (CARGO_MANIFEST_DIR run time vs compile time: cargo test variants, env -i, symlinks,
   trailing slash, Windows), write_generated (canonicalize, unlinked, create_dir, delete loop, writes), entries_of +
   generated_layout_is_current checks, new_scratch_folder, the doc-comment check vs ts-rs 12.0.1 format_docs,
   layout.rs (destructuring in const fn, const assert LAYOUT_HASH != 0, fixtures, known answers recomputed in Python).
4. Probes only in a scratch copy with its own target folder (.gate/rr-cs1-r2 or refresh .gate/rr-cs1), never the main target.
5. Commands (all --locked): fmt --check; clippy native and wasm32 -D warnings; pedantic on protocol + client-endpoint;
   test --workspace; test --release -p protocol -p client-endpoint; wasm32 build of 5 crates; cargo tree -p protocol
   -e normal; cargo doc -D warnings; -W missing_docs on both crates.
6. Findings one line each as found; DONE at the end; handback <= 35 lines.

## Findings (one line each, as found)

## Commands run (exit codes)
- Main folder (own target; code = 65a7b13, HEAD d34cfc5 only adds docs): fmt --check 0; clippy native -D warnings 0;
  clippy wasm32 -D warnings 0; pedantic protocol+client-endpoint native 0 and wasm32 0, no warnings; test --workspace 0
  (ce 5, protocol 2, others stubs); test --release -p protocol -p client-endpoint 0 (5 + 2); wasm32 build of 5 crates 0;
  cargo tree -p protocol -e normal (host, wasm32, all) = protocol alone, 0; -i ts-rs = dev-dep of protocol only;
  cargo doc -D warnings 0 (target-dir .gate/rr-cs1-r2/target-doc-main); cargo rustc --lib -W missing_docs both crates
  0 warnings (target-dir .gate/rr-cs1-r2/target-md-main). Scratch copy .gate/rr-cs1-r2 = git archive 65a7b13, own target.
- (info, checked right) Known answers recomputed by an independent Python FNV-1a over the documented stream
  (.gate/rr-cs1-r2/probe/fnv.py): BASE 0x9adbecc7, empty 0x811c9dc5, TAG_PAIR 0x4265359c / 0xbd51cd2f (untagged streams
  byte-identical, both 0x60d6dbf4), block 0x2df78c30, BASE + 12 CHANGED all distinct; RFC 9923 vectors match.
- (info) Cargo book, "Environment variables Cargo sets for crates": "this applies for running binaries with cargo run
  and cargo test as well" -> CARGO_MANIFEST_DIR is set for the test process whatever the parent env (CI's env -i ok).
  Same binary name in the main target and the scratch target (client_endpoint-bdbcb2c4df73f18c): -C metadata of a
  workspace member is path-independent, which is how round 1's cross-tree reuse happened.
- L2-1 (Low, follow-up) layout_ts.rs:44-53, ts_export.rs:17-26: one tree reached by two path spellings across
  invocations fails an honest run. Probe: link .gate/rr-link-r2 -> rr-cs1-r2; cargo test from the real path, then
  `cargo test -p client-endpoint --manifest-path <link>/Cargo.toml`: no rebuild (fresh), panic at layout_ts.rs:47,
  left ".../rr-link-r2/crates/client-endpoint" right ".../rr-cs1-r2/crates/client-endpoint", exit 101; message blames
  "another copy". cd into the link: cargo uses the physical cwd -> passes (exit 0). Fails closed, never a wrong pass.
  Fix: compare fs::canonicalize of both (same physical tree passes, another copy still stops), or name the case + cargo
  clean -p in the message.
- (info, checked right) Write mode through a symlinked repository path, one invocation (fresh build, --manifest-path
  via the link, SHAMBLE_WRITE_TYPES=1 in the scratch copy): exit 0, all 9 files rewritten after the start, identical
  to the main tree. The canonicalized root makes an honest linked checkout pass.
- (info) ts-rs 12.0.1 format_docs (src/lib.rs:1188-1213): `///` lines -> "/**\n" + " *<line>" each + "\n */\n"; one doc
  string holding '\n' (a Rust block doc) -> "/**{doc}*/\n" verbatim. So for `///` docs the closing " */" is always on
  its own line and any doc "*/" lands mid-line: the line check (ts_export.rs:123-125) catches every honest `///` case.
- L2-2 (Low, follow-up) ts_export.rs:120-125: (a) bypass: a doc line that is exactly "/" (only via #[doc = "/"]; `////`
  is not a doc comment) prints " */", which the check takes as an end line. Probe in scratch: #[doc = "/"] +
  #[doc = "export const injected = 1;"] on ViewTime.tick -> spill check passed, the test failed later only on the
  compare (ts_export.rs:142); ViewTime.ts shows "/**\n */\n *export const injected = 1;\n */". In write mode it would be
  written and pass the Rust test; tsc would refuse it (stray `*`). (b) false positive: a Rust block doc closing on a
  text line ("/** The interpolation tick,\n    in ticks. */") is refused ("holds */; reword it", ts_export.rs:126)
  though the TS is valid. Neither is an honest `///` mistake; the comment at :121 overstates the converse.
- My two leftover scratch folders from the failing probes (/tmp/shamble-types-protocol-14095-0, -14312-0) deleted file
  by file; scratch messages.rs restored (cmp = main).
- L2-1 more evidence: the reverse also bites (built via the link in run 5, then `SHAMBLE_WRITE_TYPES=1 cargo test -p
  client-endpoint` from the real path: panic layout_ts.rs:47 before write mode, exit 101; touch -> rebuild -> pass).
- (info, checked right) write_generated probes in the scratch copy (SHAMBLE_WRITE_TYPES=1, own target):
  (i) block/ missing, generated/ present -> create_dir, written, = main's, exit 0; (ii) protocol/Evil.ts soft link to an
  outside file + Join.ts a hard link to an outside file + Input.ts.tmp a link -> Evil.ts link deleted, Join.ts unlinked
  and written fresh, both victims unchanged (hard link count back to 1), Input.ts.tmp kept and the compare refused it
  ("not a plain file", ts_export.rs:63), exit 101; (iii) generated/ itself a link -> both crates refuse at
  layout_ts.rs:64 / ts_export.rs:82, nothing written (mtimes before start), exit 101. Planted 6c-6e cover the rest.
  Leftover scratch folders of these failing runs (-15689-0, -16216-0) deleted file by file; scratch tree restored.
- (info, checked right) repository_root on honest runs in the scratch copy: from crates/client-endpoint 0; from there
  with --manifest-path ../../Cargo.toml -p protocol 0; --manifest-path of the crate's own Cargo.toml 0; CI's form
  `env -i HOME PATH USER CI LANG CARGO_HOME RUSTUP_HOME ...` + test --workspace and --release -p protocol -p
  client-endpoint 0 (all suites ok). Trailing slash: cargo's pkg.root() is the manifest's parent (none); both sides
  come from the same Package in one invocation, so only cross-invocation reuse with another spelling can differ (L2-1).
- L2-1 reach on Windows (reasoned, not run): the Windows cwd is kept as typed (PowerShell keeps the typed case, subst
  drives and junctions stay unresolved), so two terminals spelling one checkout differently would hit L2-1 without
  --manifest-path. Same fix (compare canonicalized paths).
- (info, checked right) Windows join after canonicalize: fs::canonicalize docs warn "you can only join
  backslash-delimited paths to it", but std's PathBuf::_push rebuilds a verbatim (\\?\) self from components with
  MAIN_SEPARATOR_STR (doc.rust-lang.org/src/std/path.rs.html, fn _push, verbatim branch; PathBuf::push docs: "the new
  path is normalized"), so root.join("packages/types/src/generated/block") is a backslash path. Not run on Windows.
- L2-3 (Low, follow-up; reasoned, not run: no case-insensitive FS here, ext4 casefold absent in this kernel)
  layout_ts.rs:79-83, ts_export.rs:97-103: on a case-insensitive FS (macOS default APFS, Windows NTFS) a link named
  with another case of the extension (Join.TS, LAYOUT.TS) survives the delete loop (`ext == "ts"` is case-sensitive),
  then fs::write(".../Join.ts") opens that entry and follows the link: the doc's "never writes outside the repository"
  (layout_ts.rs:57-58, ts_export.rs:75-76) does not hold there. Needs a planted link (a committer, who can change the
  test anyway): no escalation. Fix: OpenOptions::new().write(true).create_new(true) for the writes (O_EXCL never follows
  a link and refuses any existing name), or eq_ignore_ascii_case("ts").
- (info, checked right) const _: () = assert!(LAYOUT_HASH != 0) is evaluated: flipped to == 0 in scratch -> E0080 on
  cargo check native, check wasm32 and build --release wasm32 (exit 101 each); restored (cmp = main).
- L-a check: the new form with section `delete` and fields enum/default/class/new: node 22.23.4 (type stripping) runs it
  and prints 0 3 4; repo tsc --noEmit --strict exit 0.
- L2-4 (Low, follow-up, test-engineer's area) layout_ts.rs:124-135: "no file of its own" keeps only plain files
  (filter entry.1), so a link (e.g. generated/x.ts -> a file) or a third folder directly in generated/ passes.
- Round-1 Lows: L-f closed (TAG_PAIR layout.rs:250-255 + known answers :273-279; untagged streams identical, so removing
  the tags fails them; planted 5a); L-a closed/moot (one object, names are property names, layout_ts.rs:21-38; node +
  tsc accept reserved words); L-b closed (checked_add layout.rs:172, fixture :206; the unchecked `ends` at :180 only
  sees fields that passed); L-c closed (ts_export.rs:31-53, probes); L-d closed (canonicalized root + unlinked parent
  and folder, layout_ts.rs:59-75 / ts_export.rs:77-93; entries_of plain-file check layout_ts.rs:86-116); L-e closed
  (missing_docs 0; SECTION_TAG doc now claims only distinct streams, which holds).
- Not taken "pin LAYOUT_HASH == 0x2df7_8c30 in Rust": agree (function held by known answers, value by layout.ts).
- Non-test code changed only by doc lines, the compile-time assert and the destructuring (const-evaluated): Wasm size
  not re-measured. cargo audit / cargo deny: not installed (not run). Windows/macOS: not run (reasoned).
- Main folder unchanged but for this notes file; probe link .gate/rr-link-r2 removed; scratch copy .gate/rr-cs1-r2
  (208 MB, gitignored, = main code) can go, as can round 1's .gate/rr-cs1. /tmp/shamble-types-protocol-19942-0
  (13:59:02) is not mine: left alone.
- Verdict: APPROVE (no finding at Medium or above; four new Lows L2-1..L2-4, all follow-ups).
DONE 2026-11-18
