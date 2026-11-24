# npt-judge's fixtures: fixed results logs that must come out PASS, FAIL and INVALID (plan §6.1)

Each mark has a `base/` folder of results logs (npt-log's format, `marks/README.md` §1) and one folder per case that
holds only the files it changes: a case file replaces the base file of the same name. `tests/fixtures.rs` judges
each case with the real `marks/marks.json` and checks the verdict and its reason. Addresses are documentation ones
(RFC 5737), so redact-check passes them.

| Mark | Case | What changes | Verdict |
|---|---|---|---|
| PM-0001-4 | `base` | three S30 runs at P2 (room on RS, bots on LG), all under 16.67 ms, V-S held, each carrying the stress scene's load (PM-0001-2, its precondition); the B-S bite run fails | PASS |
| PM-0001-4 | `fail` | run 2's slowest measured tick is 16.68 ms, so its p99 (the 3rd of 3) is over 16.67 ms | FAIL |
| PM-0001-4 | `invalid` | run 3's RS steal averages 11.5 ‰, over V-S's 1 % | INVALID |
| PM-0001-4 | `load-short` | run 1 has 47 soldiers on one measured tick: it did not carry the stress scene's load (`pre:PM-0001-2`) | INVALID |
| PM-0001-4 | `fail-with-gap` | run 2 is over 16.67 ms and also lacks its overrun counter: the failure stands, the gap listed beside it | FAIL |
| PM-0001-30 | `base` | the validity bite: a replay with one command past its tick; its run comes out INVALID by `late-zero` alone | bite shown |
| PM-0001-30 | `missed` | the same bite run with no late command counted: the validity check let it through | FAIL |
| PM-0001-30 | `base` (the C part) | six measured room logs on the container, 3 recordings x `wt` and `ws`, each naming its other-transport run in `params.twin` (C-U6-9), equal state hashes, `late_count` 0; RS's six runs and the wallclock bite not run | the C part PASS; the mark NOT-RUN |
| PM-0001-30 | `twin-differs` | recording 2's `ws` run has another hash at tick 2: both runs of the recording fail, each against the other | FAIL |
| PM-0001-30 | `same-transport` | recording 3's two runs both say `wt` | INVALID (the C part) |
| PM-0001-30 | `not-mutual` | recording 1's `ws` run names another run as its twin | INVALID (the C part) |
| PM-0001-30 | `twin-missing` | recording 1's `ws` file holds another run, serving no mark: the `wt` run's twin is not in the set yet | NOT-RUN (the C part) |
| PM-0006-14 | `base` | the N = 48 session: both bracket runs and one count run padded to 8.33 ms on every tick; its N = 36 and N = 18 parts are moved (`marks/README.md` §2.6), so the mark reads MOVED, with N = 48's count reported | MOVED |
| PM-0006-14 | `short` | one measured tick of the count run padded to 8.329 ms, short of 8.33 ms | INVALID |
| PM-0001-4 | `measured-bites-build` | C4: run 1's room log comes from a bites build, naming no bite | INVALID |
| PM-0001-4 | `measured-names-bite` | C4: run 2 is a measured run that names the B-S bite | INVALID |
| PM-0001-4 | `bite-measured-build` | C4: the B-S bite run's room comes from the measured build (no log of it lists `bites`) | INVALID |
| PM-0001-17 | `base` | one L1 P2 run in the stress scene (its precondition held): every client hash equals the server's at its tick (6 compared); every `hs` and `snap_incomplete` list holds the header's 48 entries (slots 2 to 47 not connected: `null` in both logs); the room log's `tick.s.snap_link_lost_count` 0 on every slot (C62H-BOUND); the other parts and the bites not run | NOT-RUN, the part's run valid, 0 mismatches |
| PM-0001-17 | `lost-group` | slot 1's tick 2 incomplete (`null`, `snap_incomplete` 1): not compared, not a mismatch (5 compared); its own room log has the link losing a snapshot datagram of it (`tick.s.snap_link_lost_count` 1 there: the link's explanation, E2), so it is explained | NOT-RUN, the part's run valid, 0 mismatches |
| PM-0001-17 | `differ` | slot 1's tick 2 hash differs from the server's | FAIL |
| PM-0001-17 | `refused-group` | slot 1's tick 2 is the sentinel 0 (a refused group) | FAIL |
| PM-0001-17 | `orphan` | slot 0 logs a hash at tick 65,539, which the server never logged (a wrong tick rebuild) | FAIL |
| PM-0001-17 | `none-compared` | every client entry `null`: 0 compared, an empty comparison | FAIL |
| PM-0001-17 | `permuted-slots` | slots 0 and 1 swapped in the client log on every tick: pairing is by the room's slot | FAIL (6 mismatches) |
| PM-0001-17 | `sparse` | one client hash of six, the rest `null` with `snap_incomplete` 0: the span misses the room's measured ticks | INVALID |
| PM-0001-17 | `sampled` | the client log has no tick 2 record: a server hash inside the span that no client log resolves | INVALID |
| PM-0001-17 | `slot-unlogged` | slot 1 `null` everywhere with `snap_incomplete` 0, while the room logged its hashes | INVALID |
| PM-0001-17 | `hashed-and-incomplete` | slot 1's tick 2 has a hash and `snap_incomplete` 1 at once: filed lost though the room sent it a snapshot and the link lost none of it (`snap_lost_unexplained_count` 1), the coverage gap listed beside the failure (INVALID before C62H-BOUND) | FAIL |
| PM-0001-17 | `short-lists` | 2-entry lists under the header's 48 clients | INVALID |
| PM-0001-17 | `l1-aarch64-claim` | the L1 client (role C) says aarch64 and logs a hash at tick 9, which the server never logged; an x86-64 replay agrees elsewhere: its only mismatch is that orphan, which counts on every machine (FX-5 (c)); the Arm rule's LG key is shown by the engine Arm test's role C and ML cases (TE2-1) | FAIL |
| PM-0001-17 | `lg-cross-arch` | an L2 P2 run (run `l2-p2-1`: RS's room on x86_64, LG's bots on aarch64, a `replay` log on x86_64; 2 clients): slot 1's tick 2 differs on LG and the x86-64 re-decode equals the server's | the L2 P2 part's run valid, 0 mismatches, aarch64-only 1 |
| PM-0001-17 | `lg-replay-differs` | the same, the re-decode differing too | FAIL |
| PM-0001-17 | `lg-replay-in-client-log` | LG's bots log carries its own `replay_hash` (no replay log) | INVALID (a name in another process's log) |
| PM-0001-17 | `lg-replay-on-aarch64` | the replay log is on aarch64 | INVALID (a name in another process's log) |
| PM-0001-17 | `lg-orphan` | LG logs a hash at tick 9, which the server never logged; the replay present | FAIL |
| PM-0001-17 | `lg-replay-missing-entry` | LG's tick 2 mismatch, the replay log without tick 2 | INVALID (an incomplete re-decode) |
| PM-0001-17 | `lg-orphan-no-replay` | an LG orphan and an LG mismatch, no replay log: the waiting mismatch never hides the orphan's count | FAIL |
| PM-0001-21 | `base` | one forged run (option A, the stress scene's load, 11 kinds x 1,000 commands, all rejected) and its valid-parts twin with equal state hashes | NOT-RUN, the part's run valid, twin check 0 |
| PM-0001-21 | `twin-differs` | the twin's tick 2 hash differs | FAIL |
| PM-0001-21 | `twin-bites-build` | C4's fourth case: the measured run's twin comes from a bites build | INVALID |
| PM-0001-21 | `twin-unnamed` | the forged run names no twin (`params.twin` missing): a gap | INVALID |
| PM-0001-21 | `twin-missing` | the named twin is not in the set (another run's twin is) | NOT-RUN |
| PM-0001-21 | `twin-is-measured` | two forged measured runs name each other, with equal state hashes: a twin must have role `twin` | INVALID |
| PM-0001-1 | `laptop` | an ML log naming its machine in full with no address (a run of its own: PM-0001-1 has no base folder) | the ML part PASS |
| PM-0006-11 | `base` | RS fully named (with its day address and SMT map); the no-smt-map bite run's header has an empty SMT map and fails `machine_named` | PASS, bite shown |
| PM-0006-11 | `unnamed` | RS's header has no kernel | FAIL |

PM-0001-18's `ws-practice` rule needs 480 heal records per run, too long to write by hand, so its PASS and INVALID cases
are built in code and judged with the real `marks/marks.json` in `tests/engine.rs`
(`u4j_ws_practice_puts_the_practice_runs_numbers_in_force`, `u4j_ws_practice_refuses_every_number_not_set_by_its_rules`,
`u4j_ws_far_better_past_the_cap_is_invalid`, `u4j_ws_a_practice_run_after_the_counted_run_changes_nothing`,
`u4j_ws_one_practice_run_per_build_over_every_set`, `u4j_ws_a_bite_run_takes_its_counted_runs_practice_run`,
`u4j_ws_the_record_carries_the_marks_bound_line`, `u4j_ws_the_tolerance_and_band_boundaries`).

PM-0006-14 has no pass number, so it is never PASS or FAIL. With the real `marks/marks.json` its N = 36 and N = 18
parts are moved, so its padded N = 48 session reads MOVED (the `base` case); REPORTED is covered by the engine tests,
on made-up marks with no moved part. Its fixture proves that short padding comes out INVALID (ADR 0006's PM-0006-14
row: "npt-judge's fixtures prove that short padding comes out INVALID").

MOVED with the real `marks/marks.json` (`tests/fixtures.rs`): which marks, parts, checks and bites the file labels
(`the_real_marks_file_labels_what_the_trim_moved`); a label added at every position of a passing case never reads PASS
(`a_label_anywhere_on_a_passing_mark_never_reads_pass`, on PM-0001-4 `base` and PM-0006-11 `base`); and a label on a
bite hides no kept FAIL or INVALID (`a_moved_bite_hides_no_kept_failure_or_invalid_run`).
