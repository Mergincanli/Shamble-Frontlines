docs-keeper-cs1: the docs pass of change-set 1 "Types" (foundation step 2.4), PR #58, lane 1; edits only STATUS,
STATUS_ARCHIVE, ROADMAP, REPO_MAP; never commits.

## Plan (blocks, in order)
B0. Main check: origin/main must be 22c6afa (else stop, edit nothing).
B1. Read DECISIONS: RECORD-FILES (2026-11-17 22:10:34 UTC) and OLD-LINES (2026-11-18 11:08:25 UTC), word for word.
B2. Read the report docs/reports/2026-11-18-cs1-types.md (Follow-ups, Two misses, decisions asked).
B3. Read STATUS, STATUS_ARCHIVE, ROADMAP, REPO_MAP; blame each; list this lane's commits (#56, #57, #58).
B4. Moves under OLD-LINES (merged PRs only): pick lines, write the byte-copy script, run it, list every moved line in
    docs-keeper-moved-lines.md (file, old line, new place, first words). Script beside it.
B5. STATUS edits in this lane's own lines: Last merged (#57 + pointer to ROADMAP's list), Next ("the netcode proof
    test's result"; then change-set 2), In flight (#58), Waiting on Mustafa, Follow-ups, Open risks.
B6. STATUS_ARCHIVE edits: #57's merged row, follow-up texts, health if it changed.
B7. ROADMAP: entry for #57 (merged) and #58 as the repo's style has it; keep 8-12 entries (fold oldest by script).
B8. REPO_MAP: new paths, changed descriptions, the regenerate command, the target-folder rule; stale lines of other
    lanes listed, not touched.
B9. Checks: repo-map-check, wc -l STATUS (<= 120), line widths (<= 120), RECORD-FILES blame proof (command + result).
B10. DONE and the answer.

## Notes
- B0 done: origin/main = 22c6afa0341c36c40479f671736a569adfdb2663, merge-base(HEAD, origin/main) = 22c6afa. Next B1.
- B1 done: RECORD-FILES (DECISIONS.md:202) "Each pull request writes only its own lines there, in its docs pass, after
  taking main in. It does not reword, re-wrap or move another lane's lines." OLD-LINES (DECISIONS.md:155) "(a), for
  lines about merged pull requests. A docs pass of either lane may move those lines by script, byte for byte, never
  reworded, also when another lane wrote them. Its report lists each line it moved. Every other line stays with the
  lane that wrote it". Next B2.
- B2 done: report read (309 lines). Follow-ups (report 195-223): CI-CUT; security check (licence, rust 1.78.0 vs 1.88.0,
  cut stops IP only); cs2 (exports entry, by-name handle, Section::align, native-only export tests, layout check per
  Worker start); cs3 (provisional list, decoder caps, redacting Debug on Join); cs4 (row counts and offsets, padding
  owner, read hash at 4 bytes, content export test folder rules); round 2 Lows (guards untested, path-string compare,
  doc-comment check, generated/ link or third folder, tag-byte pair, case-insensitive FS); 2.3 chat (formatter and linter
  skip generated/**, gate regeneration own build folder); drafts' next revision (who reads message types in TS; ADR 0002
  pointer). Dependency hold: report 32-35. Decisions: DEP-HOLD, CI-TYPES, CI-CUT, LANE-ISSUE (report 235-262). Risks
  (report 226-229): provisional widths; type error caught by hand only; 2.3 chat's settings differ. Next B3.
- B3 finding: this lane's commits = #56 (c94c81b..cd61b28), #57 (d7dad97..f26ac5b), #58 (22c6afa..HEAD). STATUS blame:
  own (#57) lines 5 (1aaa067), 13 (d5529ca), 29 (d5529ca, #57's row), 36 (d5529ca), 39-40 (1aaa067), 99-100 (1aaa067).
  All other lines: 3f98c0b/554face/bfd719b (#52, merged 4460ef5) and 9660197 (#55, merged c94c81b): not this lane's.
- B3 finding: #57's pass treated #52/#55 lines as another lane's ("Left as found", part-15 report :59); follow that.
- B3 finding: old Last merged pointer was "every merge: ROADMAP." (`git show d7dad97:docs/STATUS.md` :13).
- B3 finding: archive "In flight" = 4 columns (Work | Branch | State | Waiting on), newest first at :159 (#56's row,
  1aaa067, own). STATUS's table has 5 (PR first). #57's row (STATUS :29, own) moves by script, then its cells edited.
- B3 finding: security check's Lows: L1 licence, L2 Rust 1.78.0 not 1.88.0, L4 cut stops IP only; L3 CLOSED
  (security-auditor-review1.md:27).
- B3 finding: ROADMAP Recent work = 13 entries (#57 125-129 own; #56 131-137 own; #55 139-147; #54 149-154; #53
  156-162; #52 164-172; #48 174-189; #51 191-200; #50 202-217; #49 219-226; #46 228-239; #44 241-255; #43 257, one
  line). Build history folds: #37-#42 as "- " bullets at 85-88 (88 by 94aaae8, #56, prefixed "- " before OLD-LINES).
  Plan: fold #43 and #44 byte for byte after line 88, blank lines around (no byte of a moved line changes).
- B3 finding: STATUS movable lines about merged PRs are few: #57's row (own); the STATUS-DIET pointer table 113-120
  (#49's move; referenced by line 85 "the table below": leave a one-line pointer of mine). Open follow-up ids (83-98)
  stay: docs-keeper keeps open follow-ups by id in STATUS.
- B3 finding: REPO_MAP :18 (`crates/` "each still an empty stub", 790ac31) and :21 (`packages/` "each empty with one
  stub test", 550856a) are #48's lines (merged 4d21c1a): stale now, not this lane's: leave and list. :6 "Last checked:
  2026-11-17" is #55's (9660197): leave. Own REPO_MAP lines: 70-71, 78-80 (94aaae8, #56), 81-82 (1aaa067, #57).
- B3 verified against the code (daf3d98): messages.rs has 8 pub structs (Join, JoinAccepted, Input, SnapshotAck,
  Command, AimPoint, ViewTime, Snapshot); lib.rs:6-7 `#[cfg(test)] mod ts_export`; client-endpoint lib.rs:6-8 `pub mod
  layout` + `#[cfg(test)] mod layout_ts`; layout.rs:61 BLOCK_LAYOUT, :69 LAYOUT_HASH, :86 layout_hash; generated/
  protocol: 8 files, ts-rs header "Do not edit this file manually."; generated/block/layout.ts: "Generated by the test
  generated_layout_is_current ... Do not edit", LAYOUT_HASH = 0x2df78c30; index.ts re-exports the 8 types, BLOCK_LAYOUT,
  LAYOUT_HASH, checkLayoutHash, LayoutCheck; layout-check.ts:18 checkLayoutHash; Cargo.toml:13 ts-rs =12.0.1 default
  features off; protocol Cargo.toml:9-10 dev-dependency; SHAMBLE_WRITE_TYPES=="1" (layout_ts.rs:106); build-folder
  rule: ts_export.rs:15-27 and layout_ts.rs:45-46 (CARGO_MANIFEST_DIR built vs run). Map check baseline: OK, 129 entries
  cover 109 required paths. HEAD moved to daf3d98 (main session's saver; the report committed in 5f1b58f, unchanged:
  309 lines). B3 done. Next B4 (moves).
