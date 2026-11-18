# Change-set 1, the fix pass after round 1 (2026-11-18, 13:30 to 13:40 UTC, the main session)

Round 1: rust-reviewer APPROVE (six Lows), test-engineer REVISE (one High, three Mediums, four Lows), security-auditor
no finding at Medium or above on the diff (three Lows; A1 confirmed for CI, outside this diff), engine-architect REVISE
(the same High; one Medium on the report; Lows), netcode-architect APPROVE (seven Lows). Their notes are beside this file
(`<agent>-review1.md`). Keys below: RR = rust-reviewer, TE = test-engineer, SA = security-auditor, EA = engine-architect,
NA = netcode-architect.

## Taken, with where
- **TE High 1, EA High 1 (a test binary built in one tree ran in another; the 20-runs record did not hold).** Both
  export tests now stop when the folder `cargo test` runs them in (`CARGO_MANIFEST_DIR` at run time) is not the folder
  they were built in (`repository_root` in `layout_ts.rs` and `ts_export.rs`). `plant.sh` builds into a target folder
  of its own (`.gate/cs1-types/plant-target`); no brief tells a reviewer to share the main `target` again. The main
  `target` was removed with `cargo clean` (between 13:33 and 13:37 UTC) and every hand check was run again from an empty build folder.
  `runs20.sh` counts a run only when every generated file is newer than the run's start, and prints the worktree count.
  The leftover folder `.gate/cs1-types/plant/` was deleted file by file (13:16 UTC). New planted runs 6a and 6b.
- **TE Medium 2 (write mode made a missing tree again).** Write mode makes the one generated folder if it is missing and
  nothing above it (`create_dir`, not `create_dir_all`), and needs its parent to exist. Planted run 6c.
- **SA S1, RR 5 (write mode followed a linked folder; the `ends_with` check could not fail).** The check is gone. Write
  mode resolves the repository's root, and refuses unless the generated folder and its parent are their own real
  paths (no link in the path). Planted runs 6d and 6e (a file outside the worktree must survive).
- **RR 5, second half.** `client-endpoint`'s comparison now takes `layout.ts` only as a plain file (`entries_of`), as
  `protocol`'s does.
- **TE Medium 3 (a Vitest test that only tsc can fail).** The test's name and comment in `index.test.ts` say so. The
  report says: the type exports go unchecked in CI until the CI edit lands.
- **TE Medium 4, RR 1 (the hash test's blind spots; no test for the tag bytes).** `hash_covers_every_value` has four more
  tables (a field of the second section alone, the section order, one section more, one less), the pair of tables
  that share one stream without tag bytes (RR's A and B), and known answers for the fixture, the empty table and the
  pair, computed by a second implementation of the documented stream (a Python script, not this code). The last line
  that could not fail is gone. Planted runs 5a (no tag bytes) and 5b (later sections' fields skipped, TE's T1).
- **TE 5.** Overlap is tried both ways (a narrow field on a wide one, a wide one on a narrow one).
- **TE 6.** The Vitest test flips bit 31 as well as bit 0, and a block of zero bytes is tried; planted run 7 (a signed
  read). Rust: `const _: () = assert!(LAYOUT_HASH != 0);` (SA's nit), so a zeroed block can never pass.
- **TE 7.** `generated_layout_is_current` also fails when `generated/` holds a file of its own. Planted run 2e.
- **TE 8.** The failing messages say "delete by hand what does not belong there" before they name write mode.
- **RR 2, EA 4 (a section named like a reserved word; a top-level constant per section).** The generated file exports
  one object, `BLOCK_LAYOUT = { header: { stride, fields } } as const`; section and field names are property names.
  `index.ts` exports `BLOCK_LAYOUT` where it exported `header`.
- **EA 3.** `align` is gone from the generated file (the hash did not cover it and nothing read it).
- **EA 5.** `layout_hash` and `render_typescript` name every member of `Section` and `Field` in a pattern, so a member
  added later does not compile until both handle it.
- **RR 3.** `broken_rule` uses `checked_add`; a fixture with an offset past the `u32` range.
- **RR 4, SA's nit.** The scratch folder's loop goes on only after `AlreadyExists`; any other error stops with the path
  and the reason; the temporary folder must be an absolute path.
- **RR 6.** Every public item has a doc line (`-W missing_docs`: 0 warnings); the tag constants' doc says what holds.
- **SA S2.** The export test of `protocol` fails a line of a generated file that holds `*/` with other text (a doc
  comment that would end the comment early; a `*/` alone on a line still passes: round 2's follow-up). The first form, a count of `/**` against `*/`, was fooled by
  the planted run 2f (a glob that holds both); commit 65a7b13 looks at lines.
- **SA S3.** `checkLayoutHash`'s doc: a drift check, not a security check; "never throws" holds for an `ArrayBuffer`.
- **NA 1 to 6.** Doc lines in `messages.rs`: `AimPoint` is provisional as a whole (its step and widths are borrowed
  from E3; PM-0001-51 sets them; the encoder clamps); counters wrap at their wire width and are compared wrap-aware;
  `sequence` is provisional; the tick rate is fixed at build time and only checked by the client; the snapshot rate is
  nominal; `commands` are consecutive, their order is the codec's; `slot` is the soldier's sim index; how `squad`
  counts is open. The eight generated files were written again.
- **EA 10.** "the build plan's change-set 3" where the docs said "the protocol's first messages".

## Not taken, with the reason
- **TE Medium 4, its last clause (pin `LAYOUT_HASH == 0x2df7_8c30` in Rust).** The function is held by the known answers
  above; the block's own value is held by the committed `layout.ts` through the export test. A literal in Rust would be
  a second place to edit by hand at every layout change. Round 2 may disagree.
- **SA S1, its second clause (refuse write mode when `CI` is set).** The CI edit drafted for Mustafa runs write mode in
  CI on purpose (regenerate, then compare with git). The link refusal closes the hole itself.
- **EA 6 (read the hash before asking for the whole header).** Today both lengths are 8 bytes, so the extra branch
  could not be reached by any test. A comment in `layout-check.ts` says what to do when the header grows; the report
  lists it for change-set 4.
- **NA 7 (one constant for the 16 buttons), optional.** Both are provisional capacities; listed for change-set 3.

## For the report, not code
EA Medium 2 (PM-0002-15: its zero-difference clause is met in CI, its planted defects are shown here); EA 7 (never use
ts-rs's `repr(enum)`); EA 8 (the 2.3 chat's formatter must skip `packages/types/src/generated/**`); EA 9 (no TypeScript
code reads the message types in the chosen design); EA 11 and RR's notes for change-set 2 (an `exports` entry; a by-name
field handle; `Section::align()`); padding between sections for change-set 4; NA's provisional list and decoder checks
and SA's anti-cheat table for change-set 3; A1 with SA's fix text; DEP-HOLD with SA's four additions; L1 to L4.
