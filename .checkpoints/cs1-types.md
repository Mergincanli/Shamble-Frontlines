# Checkpoint: cs1-types
- **Goal:** change-set 1 of the build plan, "Types" (foundation step 2.4): `packages/types` and its generators. The
  page and the simulation share one generated description of their data, and a test fails when they drift. Card:
  `docs/reports/2026-11-17-build-plan-slice-1.md`, Part 1, "1. Types (step 2.4)" (planned size 600 lines, about 7 h;
  reviewers engine-architect, rust-reviewer, test-engineer; no Brave check; his go on the merge). His words
  (2026-11-18, 11:08:25 UTC; `cs1-types/mustafa-2026-11-18-1131-merge-57.md`): "Change-set 1, Types (foundation step
  2.4): its plan, its review, the build and its pull request, then stop for my go."
- **Branch:** claude/wonderful-hamilton-jdmlnf (stands in for cs/cs1-types) · **Base:** main
  22c6afa0341c36c40479f671736a569adfdb2663 (the merge of #57; the branch was fast-forwarded to it)
- **Last update:** 2026-11-18 13:06 CET (from the clock)

## Steps done
1. Pull request #57 (Part 15 with his six answers) merged on his go of 11:08:25 UTC: merge commit 22c6afa,
   2026-11-18 11:09:31 UTC, parents d7dad97 and f26ac5b, tree equal to the approved head, his words in its message
   byte for byte (1,554 bytes, checked by script). Before the merge: head f26ac5b read again; gate, smoke and
   toolchain green on it (six check runs). After it: the branch fast-forwarded to main and pushed.
2. Session check at 11:09 UTC (get_session): model claude-fable-5-2 (configured and last served); context 862,376 of
   1,000,000 tokens used (automatic compaction at 901,600): he is asked to compact; heavy reading goes to agents.

3. engine-architect's plan draft is done (`cs1-types/plan-draft.md`, 169 lines; notes `engine-architect-plan.md`):
   ts-rs 12.0.1 as a dev-dependency of `protocol` only, derives behind `cfg_attr(test)`; export tests that write to a
   scratch folder and compare with the committed files (write mode: `SHAMBLE_WRITE_TYPES=1`); the layout table as a
   `const` in `client-endpoint`, header `tick` u32 at 0 and `layout_hash` u32 at 4, hash FNV-1a 32-bit at compile
   time; `checkLayoutHash` in `packages/types`; about 505 hand-written lines. The main session read it whole and
   adopted it in `cs1-types/plan.md` with its decisions on O1 to O5 and seven additions (who builds, the CI edit
   drafted for him, the test archive, the docs pass, the report, the size, the rules kept).

4. The advisor agent on the plan, 11:58 UTC: APPROVE on four conditions (a verifier run; netcode-architect as a
   reviewer; security-auditor checks the new lockfile entries after `cargo fetch` and before the first build of
   ts-rs; planted defect (2b), a hand edit of `layout.ts`) and nine Lows to fold in. All taken into `plan.md`
   ("After the advisor agent's review"). O1 to O5 confirmed.

5. Build, step A (12:02 UTC): the first edits outside `.checkpoints/`. `Cargo.toml` (workspace table: `ts-rs = { version
   = "=12.0.1", default-features = false }`), `crates/protocol/Cargo.toml` (`[dev-dependencies] ts-rs = { workspace =
   true }`), then `cargo fetch` (download only, exit 0). `Cargo.lock` gained nine entries: proc-macro2 1.0.107, quote
   1.0.47, syn 2.0.119 (asked by ts-rs-macros), syn 3.0.10 (asked by thiserror-impl), thiserror 2.0.21, thiserror-impl
   2.0.21, ts-rs 12.0.1, ts-rs-macros 12.0.1, unicode-ident 1.0.27. Sources unpacked under
   `~/.cargo/registry/src/index.crates.io-*/`. Committed with "[skip ci]" (both workflows run on every push, and CI's
   `cargo test` would compile ts-rs before the security check). NOTHING has been compiled yet.

## Current step
6. (12:04 UTC) Two things at once:
   - security-auditor (Opus 5.5, effort max; started 12:03 UTC; notes `cs1-types/security-auditor-deps.md`) checks
     the nine lockfile entries: source and checksum, licence, advisories, build-script and proc-macro code, what
     ts-rs writes at test time, its API facts (`Config`, `with_out_dir`, `with_import_extension`, `export_all`), reach
     (`cargo tree -e normal`). Until its answer says CLEAR TO BUILD, no command that compiles ts-rs runs (so no
     `cargo test/check/clippy` on `protocol` or on `--workspace`); a finding at Medium or above stops the build and
     goes to the advisor agent. If the agent is lost, start a fresh one from its notes file with the same brief.
   - the main session writes `client-endpoint` (no ts-rs there: `cargo test -p client-endpoint` is safe to run).
   Mustafa, 12:03 UTC (two messages while this ran): "Save your progress I'm going to run the compact command" and
   "Same for the agents": this save; the agent was told to save its notes at once and carry on.

## A process miss to report (records, with their times)
- **CI built the new dependency once before the security check's answer.** The manifests' commit 566d1d1 carried
  "[skip ci]", but the checkpoint commit pushed on top of it (da22f96, 12:04:37 UTC) did not: `checkpoint.mjs` leaves
  the mark out when a code commit is ahead (its line 167). The push started `toolchain` run 44300471741 (number 168,
  job 134788575344, GitHub-hosted runner). Its step "Build and test with the network cut off" ran 12:04:55 to
  12:05:07 UTC; the job's log reaches the script's last command (`tsc --version`: "Version 6.0.3") before the cancel
  the main session sent landed (conclusion: cancelled). So `cargo test --workspace --locked`, the release tests and
  the wasm32 build ran on that runner with the nine crates: their build scripts and the thiserror-impl proc macro ran
  there, inside the job's network cut. Nothing was compiled on this machine. The advisor agent's condition 3 (check
  before the first build) is missed for that one CI run. security-auditor was told at 12:06 UTC and adds to its
  answer what that run could reach and the miss's severity; the miss goes to Mustafa (told in chat), to the advisor
  agent with the security answer, and into the report.
- **Rule until the security answer says CLEAR TO BUILD:** no code commit (working-tree edits only), so every push is a
  checkpoint-only push and carries "[skip ci]".

## Build notes (the main session's design choices, to carry out and to list in the report's deviations)
- `crates/client-endpoint/src/layout.rs`: `ElementType { U8, U32 }` with `const fn name()` ("u8", "u32") and `const fn
  size()` (1, 4); `Field { name, ty, offset }`, `Section { name, stride, fields }`, all fields `pub`, derives `Debug,
  Clone, Copy, PartialEq, Eq`; `BLOCK_LAYOUT` (one section `header`, stride 8: `tick` u32 at 0, `layout_hash` u32 at
  4; ADR 0002 part 5: "a header (tick, layout hash, counts per entity kind)", no count yet, O2); `LAYOUT_HASH`;
  `pub const fn layout_hash`; a private `const fn fnv1a(hash: u32, bytes: &[u8]) -> u32` (basis 0x811C9DC5, prime
  0x01000193; `while` loops, no `usize` cast).
- **Deviation 1 (hash stream made unambiguous):** the draft's stream is per section name, 0x00, stride u32 LE; per
  field name, 0x00, type name, 0x00, offset u32 LE. The build puts a tag byte in front of each entry: 0x02 before a
  section, 0x01 before a field. Why: without it a field entry and the next section's entry cannot be told apart for
  some strides (two tables could give one stream). The rules test also requires names to be `[a-z][a-z0-9_]*`
  (no 0x00 in a name; safe to print as TypeScript identifiers), unique sections, unique fields per section.
- **Deviation 2 (client-endpoint's export test compares in memory):** `render_typescript` is pure and returns the
  file's text, so `generated_layout_is_current` compares that text with the committed `generated/block/layout.ts` and
  checks the folder holds exactly that one file; no scratch folder is needed there (the card says "regenerates into a
  scratch folder"; `protocol`'s test does, because ts-rs writes files). Write mode (`SHAMBLE_WRITE_TYPES=1`): check
  the folder's path ends with `packages/types/src/generated/block`, delete only `*.ts` files directly in it, write.
- `layout_ts.rs` output: a two-line "generated, do not edit" comment naming its source and the regenerate command;
  `export const LAYOUT_HASH = 0x%08x;`; per section `export const <name> = { stride, align, fields: { <field>: {
  offset, type: "u32" }, … } } as const;` (align = the largest field size).
- Tests in `layout.rs`: `layout_rules_hold`, `hash_covers_every_value` (fixture tables as literals: changed section
  name, stride, field name, type, offset, an added field, a field moved to the next section), `fnv1a_matches_rfc_9923`
  ("" 0x811c9dc5, "a" 0xe40c292c, "foobar" 0xbf9cf968; the RFC number comes from engine-architect's source S13, the
  datatracker page of draft-eastlake-fnv, "later published as RFC 9923"); in `layout_ts.rs`:
  `generated_layout_is_current`, `typescript_carries_the_hash`. `stub_builds` goes.
- Then `protocol` (after the security answer): `messages.rs` with the eight types of the draft's section 4 (doc lines
  mark every provisional field, the two acknowledgement fields of `Snapshot` included, and the module doc names
  OQ-0001-6, OQ-0001-7, R2-XU-OQ4, the mode hash and the join token's format); `ts_export.rs`
  (`generated_types_are_current`: scratch folder `temp_dir()/shamble-types-protocol-<pid>`, cleared first).
- Then `packages/types` (`layout-check.ts`, its test, `index.ts`, `index.test.ts`, the package's description line),
  section 6's checks, the planted defects (1), (2), (2b), (3), (4) in `.gate/cs1-types/plant`, the 20 runs.

## Next steps
6. Build (Rust: the first message types of `protocol`, the block's layout table in `client-endpoint`, our own small
   generator, ts-rs for the message types, an export test per crate; TypeScript: the generated files committed, the
   layout-hash check with its test; the mark's planted defects run once and shown). Checks by hand (the plan's Part
   7): cargo fmt, clippy host and wasm32 with -D warnings, cargo test, the wasm32 build, tsc and Vitest by path, the
   legacy gate. No root `.cargo/config.toml`, no "type" in the root package.json, no non-Markdown file naming the
   spike folder (the netcode proof test's rules).
7. Reviews at effort max (engine-architect, rust-reviewer, test-engineer, security-auditor, netcode-architect; the verifier; then the advisor agent on the
   final head), the report, the docs pass, the pull request, stop for his go.

## Open findings (to take into this change-set's docs pass)
- Record in DECISIONS, by script: his go on the merge of #57 with his answer OLD-LINES ("(a), for lines about merged
  pull requests. A docs pass of either lane may move those lines by script, byte for byte, never reworded, also when
  another lane wrote them. Its report lists each line it moved. Every other line stays with the lane that wrote it").
- The advisor agent's three Lows on #57's final head: the Part 15 change's report says 220 written lines (223 counted
  on the final head; he accepted the size "as counted: 220"); STATUS's "Last merged" line lost its pointer "every
  merge: ROADMAP"; STATUS's "Next" says "Slice 3 waits for #40" where his words say the netcode proof test's result.
- STATUS and ROADMAP up to date for the merge of #57; under OLD-LINES (a) the docs pass may now move lines about
  merged pull requests by script and lists each moved line in the report.
- Gate and CI edits that follow this change-set (GATE-FOLLOW (b)): each asked of him with its exact text (after 1:
  the generated types in step, G4).
- Lane rules (Part 15, his LANE-SPLIT (a)): lane 1 owns `crates/**`, `packages/types`, build files; one pull request
  at a time per lane; RECORD-FILES: only this lane's own lines in the record files.

## Key numbers
- Planned size 600 lines; twice that is his stop line (1,200): past it, stop and tell him in three lines.

## Decisions taken
- His go on #57 and OLD-LINES (a), 2026-11-18, 11:08:25 UTC. His go on #56, 2026-11-17, 22:59:16 UTC. His six answers
  to Part 15, all (a), 22:10:34 UTC.

## Running work
- saver: background task b98z35xsq, started 11:11 UTC on 2026-11-18.
- agents (1 of 3): security-auditor on the nine lockfile entries (started 12:03 UTC).
