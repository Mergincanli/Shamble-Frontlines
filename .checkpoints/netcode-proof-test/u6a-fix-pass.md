# U6a's fix pass after its one review round (2026-11-23; the main session's list, TEST-FIRST: choices by my lean)

Sources: u6a-review-netcode.md (REVISE: NR-U6A-1 High, -2 and -3 Medium, -4 to -8 Low) and u6a-review-test.md (REVISE:
TR-U6A-1 High = NR-U6A-1, -2 to -4 Medium, -5 to -12 Low). Both read in full. NPT-TRIM: the High opens round 2, on
what this pass changes only (the netcode and test seats). Nothing here changes a pass mark, the judge, marks.json,
SERIES.md or a manifest; if a fix seems to need one, stop that item and say so.

## Start
- The builder's worktree /home/user/Shamble-Frontlines/.claude/worktrees/agent-a95ebf14368f277ff, as snapshotted in
  wip/u6a-build.patch (sha256 a9f8b6e0ca5d722896a1f9a18d8f61c484bdd32eff98eacbddc3baa98e5d1d39). Before the first edit,
  the runner's start line must read `diff 850abb561e1bc99105088a40ae580ff8d7197e3b0d02116dceddb020409463f9` and
  `untracked 3c0a31cb6d0a04db7d72c7a316c03e07b8a835f04b104d99c40a2a863c4e87d6`; if not, stop and report.
- Runs only through u6a-run.sh (lane lock, 4 GB floor; free disk about 10.6 GB). No new dependency (DEP-HOLD).

## The fixes (worktree paths under spikes/netcode-proof/)
- **FP-1 (NR-U6A-1 High = TR-U6A-1): npt-super's header carries its run's fields.** npt-super takes `--scene`,
  `--tick-hz` and an optional `--bite`, and `log()` (crates/super/src/main.rs:161-193) writes them into its header
  (bite, tick_hz, scene; features as npt-super's own build has them). Tests: one npt-super test,
  `h_the_log_header_carries_the_runs_bite_scene_and_tick_rate` (a fake room `sleep 2.1; echo OPEN; cat > /dev/null`,
  flags `--bite PM-0001-16/double-load --scene SS --tick-hz 30`): super.jsonl's header carries those three, and max
  `spawn_to_open_ns` >= 2,000,000,000, so -16's check fails by name on npt-super's own log. binary.rs's double-load
  bite (crates/server/tests/binary.rs:206-236) drops its hand-built super log and asserts only the room's OPEN delay
  >= 2 s plus the room header's bite and features ["bites"]. No judge change: an exemption for process `super` would be
  Mustafa's, not proposed.
- **FP-2 (NR-U6A-2 Medium): Q-U6A-5's SMT-sibling refusal.** In `run()` (crates/server/src/main.rs, after the SMT map is
  read near :448): when `tick_vcpu` is set and any `io_vcpus` entry shares an SMT sibling list with it, exit 2
  (EXIT_SETTINGS) with one stderr line. Reuse proc_read's sibling logic through a small crate-visible function, or let
  `TickLoop::start` return an error kind the binary maps to 2: the smaller. One refusal case on a fixed map.
- **FP-3 (NR-U6A-3 Medium, with TR-U6A-9's fixtures): a failed or partial read is never the best value.** In
  `affinity()` (crates/server/src/tick_loop.rs:1277-1285): `alone` 1 only when the walk holds the tick thread and tid 1
  and `tasks.len() < TASKS_MAX`; else 0 and one `eprintln!`. `sibling_free` 0 when any I/O tid's `allowed` is `None`
  or the SMT map is empty, with one `eprintln!`. Fixtures pinning these: an empty walk, a walk without tid 1, an empty
  SMT map (about 6-10 lines).
- **FP-4 (NR-U6A-4 Low, with TR-U6A-9's steal part):** a failed per-second read leaves its key out of the record after
  the first second (`steal_permille`, `rss_bytes`, and any CPU share that now falls back to 0), with one stderr line at
  the first failure, so the judge sees a gap, never a best value. A test feeds the steal path "" and a /proc/stat text
  without `cpu<N>`: no key, and the failed flag set.
- **FP-5 (NR-U6A-8 Low):** a failed offer of `Kick(replaced)` (tick_loop.rs:662-666) or `RoomEnded` (:751-755) is
  counted in `out_full` (2 lines).
- **FP-6 (TR-U6A-2 Medium): -2's everyone_in_view against the lab's rule.** In counted() (tick_loop/tests.rs:1006),
  compute `everyone` by the lab's rule (npt-lab mirror.rs:324-346: every other soldier, and ES's vehicles, in the
  client's frame; here `c.relevant()`), drop the `everyone: 0` override (:1071-1078) and compare the whole SceneRow,
  over the 90-tick SS run that already has hits and kills. If any tick differs, change neither rule: keep the test on
  the server's rule, write each differing tick and its cause in your notes and in your hand-back; round 2's netcode
  seat names the rule.
- **FP-7 (TR-U6A-3 Medium):** carry.rs (crates/server/tests/carry.rs:111-112, :196-212) runs both legs under
  `[CmdTiming::A, CmdTiming::B]` with `.with_cmd_timing(t)` on the room (about 4 lines); B is then set explicitly.
- **FP-8 (TR-U6A-4 Medium), the four flake risks:** (a) s5_every_counter_is_on_every_record (tick_loop/tests.rs:
  1166-1203) and (b) s4_a_pinned_room... (:814-837) wait on the loop's own progress (until `tl.ticks() >= 40`, at most
  4 s) instead of a fixed time; (c) npt-super's two fake-room tests (super main.rs:389-395, :452, :499): one `static`
  Mutex held across `fake()` and the spawns, or one test for both rooms; (d) h_time_starts... (:456-463): take `held`
  before spawning the ender.
- **FP-9 (TR-U6A-6 Low):** a pure `fn exit_code(..) -> u8` used by `finish()` (main.rs:531-537), with a unit test:
  Clean 0, Crash 3, Err 1.
- **FP-10 (TR-U6A-7 Low):** binary.rs's start helper (:60-66) reads the first line on a thread with `recv_timeout(5 s)`
  and kills the child on timeout; the stuck-writer test (tick_loop/tests.rs:1138-1151) runs its wakes on a thread
  joined with a timeout.
- **FP-11 (TR-U6A-8 Low):** the header test asserts `stand_in_burn_ns` 0 and `stand_in_top_up_ns` 0 (main.rs:739).
- **FP-12 (TR-U6A-10 Low):** host_step's test (crates/bots/tests/host_step.rs:85-113) gains a leg with the forge
  behaviour, asserting `out.reliable` holds the forged message.

- **FP-13 (round 2, 2026-11-23: TR2-U6A-1 Medium, with NR2-U6A-3; u6a-r2-test.md, u6a-r2-netcode.md):** FP-4 made a
  steal read with no time elapsed (no jiffy between two /proc/stat reads) count as a failed read: its key is left out
  with a false "failed" stderr line (tick_loop.rs:1037-1046), and the pinned test (tick_loop/tests.rs:843-851) wants
  steal_permille on every second record, so a writer lag of about 300 ms can fail it. Tell "no time" apart from a
  failed read (no stderr line for it), exempt the final partial second in the pinned test, and add one unit test for
  the no-time case (two equal /proc/stat texts: no key, no failure flag). About 10-20 lines; no other change. After it:
  rustfmt's diff by Edit; clippy and the touched tests through the runner (`--locked`); P9 once more on the final tree.

## Plants (scratch edits, one `-x -k` runner call each, restored and sha256-checked, on the final tree)
- Re-run on the final tree: P7 and P7b (TR-U6A-11; carry.rs changes in FP-7), P8 (FP-3 touches affinity), P9 (FP-4
  touches the steal path). Each fails its named test as before.
- New: P12 `log()` ignores the three flags (FP-1's npt-super test fails); P13 the sibling refusal removed (FP-2's
  refusal case fails); P14 `alone` 1 on an empty walk (FP-3's fixture fails).

## End checks and notes
- rustfmt's diff applied by Edit; then the full check.mjs once through the runner (.gate/u6a/check4.log): all 11 steps
  PASS; measured and bites counts as deltas on base's 709 and 806 (they were 744 and 844); the test step's time.
- Your notes (u6a-build.md, as before): one FP-n line per item (done, file:line, test name), each plant's failing test
  and restore hash, FP-6's outcome, added lines in all. SIZE-CHECK: past 6,000 added the pass stops and you report
  what grew; this list is about 150 lines (5,460 now).

## Not in this pass (carries; the card holds their takers)
NR-U6A-5 to U6d (a per-session share of the inbound queue); NR-U6A-6 to U6e (the -2 count's cost in a timed phase);
NR-U6A-7 to U6b's brief (TickLoop's runtime, `stop` on a plain thread; with it the bites-only `bite` field of
LoopSettings); TR-U6A-5 and O1 (kicked-session close test) to U6d, and into U6a's not-verified list; O2 (protocol
version, content hash and `mode_hash` in one home with a test) to U6b and U7; F4's S1 CPU burn to U6e (my lean, before
the first run that times the tick under S1); F1's room half of register 96 stays open; TR-U6A-12 and section 3's
not-verified gaps go into the merge report; NR-U6A-1's U8 carry (npt-super's log takes the room's run id; npt-super
ends by its stdin closing, never by a kill) to U8's fix pass.
Round 2's Lows: NR2-U6A-1 to U8's fix pass (npt-super's log takes the room's run id and the same eight header values,
set, role, layer, profile, date, scene, tick_hz and bite, from one source, bite runs included; npt-super now needs
`--scene` and `--tick-hz`); NR2-U6A-2 to U6b (exited I/O tids left out of sibling_free); to the merge report:
NR2-U6A-4 (the new pub `TickLoop::io_on_a_tick_sibling`), TR2-U6A-2 (C-U6-17's A half holds by option A's own rule;
its legs are a non-regression run), the fix-pass plants ran without `--locked`, the recorded 20-start command lacks
the new flags, FP-2's and FP-3's paths in the binary and in affinity() are reached by no test, and the double-load
bite shares the 5 s cap with its own 2 s of sleeps.
