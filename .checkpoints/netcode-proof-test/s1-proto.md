# W1-S1's npt-proto half (the main session's), for U4d's merge

The advisor's words: advisor-u4-wave1.md:84-94 (W1-S1, "npt-proto (the main session, in U4d's merge commit, after U4a
has merged)"). Built 2026-11-11 from about 10:15 UTC by the main session in the scratch copy `.gate/u4d-s1` (detached
at 21a8597, the branch with U4s, U4b and U4a merged), with U4d's final fix pass applied first (wip/u4d.patch of
10:18; the copy's crates/sim is byte-identical to U4d's worktree `.claude/worktrees/wf_f54246ff-e0d-1`). Snapshot of
the whole copy: wip/u4d-s1.patch (U4d plus this half; re-taken after each step). Nothing is committed in the copy.

## Plan
1. The two GROUPS lines: `EntityKind::OwnState` joins the "own state" row of `Snapshot::GROUPS` and of
   `FullState::GROUPS`.
2. The section rule (SEC-U4a-2): a record whose kind its section's rows do not list is refused both ways, with the
   existing error kinds. The own-state section's rows are the table's owner rows; the entity section's every other
   row. The encoder returns `EncodeError::OutOfRange` (before anything of the record is written; `encode` truncates
   on any error, so nothing is written); the decoder returns `DecodeError::Malformed` as soon as the kind is read.
   Both `Snapshot` (deltas: add, update, removal) and `FullState` (whole records).
3. The GROUPS test as the ruling's three checks: every kind has a home in both tables; the owner-only kinds (Effect,
   OwnState) sit only in owner rows; a remote soldier's add carries exactly the catalogue's remote row (the widths
   16/16/12/16/10/8/10/4, and on the wire 3 + 16 + 2 + 16 + 92 = 129 bits, measured exactly by a marker bit).
4. tests/bites.rs's -57 test also sends the owner-only record (in its own section), so "velocity untouched" still has
   a velocity field to check.
5. What the section rule breaks in npt-proto's own tests, fixed: the generators (tests/common/mod.rs: `delta`,
   `snapshot`, `full_state` drew any kind for either section) and a1's mapping test (tests/wire.rs: every kind in both
   sections). Plus one new test for the refusals.
6. Evidence: npt-proto's tests in both builds; fmt, clippy (host both builds, wasm32), rustdoc; a plant per new check;
   then check.mjs on the combined copy, counted, once the shared target is quiet (expected: measured 311 + 39 (U4d) +
   1 = 351, bites 342 + 45 + 1 = 388, self 52).

## Changes (files under spikes/netcode-proof/crates/proto)
- src/snapshot.rs: the module text's section paragraph; `Section`, `Groups`, `carries()`; `put_delta`, `get_delta`,
  `put_record`, `get_record` take the table and the section and check the kind first; `Snapshot::put`/`get` and
  `FullState::put`/`get` pass them; both GROUPS rows gain `OwnState`; the `own` fields' and GROUPS' docs; the unit test
  renamed `groups_home_every_kind_keep_owner_only_kinds_to_owner_rows_and_send_the_remote_row`, with the three checks.
- src/error.rs: `DecodeError::Malformed` and `EncodeError::OutOfRange` name the new case.
- tests/common/mod.rs: `section_kinds()` (from the same tables), `Gen::kind_of()`, `Gen::delta(kinds, ..)`;
  `snapshot()` and `full_state()` draw each section's kinds from its rows (full_state's own section was Soldier only;
  now Soldier, Effect or OwnState).
- tests/wire.rs: a1's mapping test sends each kind in each section that carries it; new test
  `w1_s1_a_record_in_a_section_that_does_not_list_its_kind_is_refused_both_ways` (+1 test in each build).
- tests/bites.rs: `own_state()`; `sent_and_decoded()` sends a record in its own section; the -57 test checks the
  owner-only record arrives untouched under each of the three bites, at half a coarse step and at the top end.
- No public item added or removed (`Section`, `Groups` and `carries` are private; the test helpers are test-only).

## Progress
- 10:23 measured build, npt-proto: lib 13, bites.rs 0, catalogue 7, wire 9 (was 8), doc 0: all pass. Each run showed
  "Compiling npt-sim" and "Compiling npt-proto" from this copy's path, with the 2 s touch loop and no other cargo.
- 10:24 bites build, npt-proto: lib 16, bites.rs 5, catalogue 7, wire 9: all pass.
- 10:24 `cargo fmt --all --check`: two diffs (a test name too long for one line, the import list's wrap); fixed by
  Edit (the name shortened, the list as rustfmt lays it out); clean on the re-run.
- 10:24 clippy -D warnings, host, all targets, measured and bites: clean. wasm32 lib: clean. rustdoc -D warnings:
  clean.
- Reference sha256 (scratchpad s1-ref.sha256): snapshot.rs 48ca1183, error.rs f52dbb94, common/mod.rs b23b687b,
  wire.rs 69ef15ec, bites.rs 96d07cc9; sim seam.rs 0c41b406 (= U4d's), quant.rs 0d41e906 (unchanged).

## Plants (each by Edit in place, the one test run alone, restored by Edit, then `sha256sum -c`)
