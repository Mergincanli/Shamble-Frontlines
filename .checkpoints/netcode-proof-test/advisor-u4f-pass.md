# The advisor's check of U4f's round-1 fix pass, 2026-11-19

Seat: a day session's advisor (general agent on main's `.claude/agents/advisor.md`, read whole with `git show
origin/main:`). Read only: this file is my one write. I built, ran and timed no Rust code; I re-derived the trees from the
two snapshots in my scratchpad and read every log myself. This check replaces round 2 (advisor-u4f-r1.md:311-323).

## 1. Verdict
**APPROVE WITH CONDITIONS: the merge may proceed on M1-M9. No round 2 on any item. Nothing goes to Mustafa.**
- **Every item FP1-FP19 matches its seat's named fix and its acceptance check.** The pass's whole diff (reviewed tree
  74ae26aa… to final tree 286aac60…) is 10 files, 330 insertions and 121 deletions. Every hunk maps to an FP item;
  nothing else changed. docs/design is untouched.
- **Each fails-first and plant log fails where the ruling says it must, and each acceptance log passes.**
- **Both end checks pass all 11 steps** (A2 and B: 634 measured, 731 bites, 0 failed, 0 skipped). U4f's share is met:
  +82 against +73, +99 against +88, and 46 negative cases against 45.
- **SERIES.md:** the pass changed only cells and the title of U4f's own new subsection. Against 5b29296b the file has 20
  added lines and none removed, so no existing row changed. marks.json, crates/judge and marks/README.md are unchanged.
- **The snapshot is the final tree, byte for byte:**
  - its sha256 is 286aac60561ba68409a082beb40babbd82706f00863ccef25b4f557200268519;
  - applied to b25b96a, it rebuilds builder 4's worktree exactly (`diff -rq` finds no difference in spikes/netcode-proof or
    docs/design);
  - I recomputed the runner hashes in that worktree: diff 2f07abc2…, untracked 1a767d5b…. They equal the pre-doc, A2
    and B headers.
- **Four Low findings (section 3), no higher.** They ride the merge commit as record lines or go to U4f2's lane.
- **Conditions** (record and operations only: no code change and no new review):
  - **C1, disk before M5:** I read 4,362 MiB free, 266 MiB above the 4,096 floor (ADV-U4FP-4).
    - M5 rebuilds every npt-* crate under the main folder's path. Builder 4's cold rebuild in a new path grew the shared
      target by about 465 MiB.
    - If `df` shows less than about 4,600 MiB, run P27's part-clean first. No lane is running, as before builders 2, 3
      and 4.
    - Keep builder 4's worktree until M4's `cmp` is done.
  - **C2, M5's background limit:** at least 3,600,000 ms. A2 took 16 min 40 s and B took 20 min 11 s, and builder 4's
    first A was killed by a 600 s limit. M5 adds the cold rebuild.
  - **C3, M8's report carries these lines:**
    - the snapshot sizes in bytes (ADV-U4FP-1);
    - the process slips (ADV-U4FP-2);
    - the size: 6,425 added lines, the pass +212 (its own diff 330 in, 121 out);
    - the counts: +82 and +99, with 46 negatives by row;
    - two notes on how items were built: FP7 writes the header in `run()`, and FP2 adds the stall harness;
    - ADV-U4FP-3 as a carry into U4f2's brief, beside C-U4F2-2.

## 2. The per-item table
"Departs" means a departure from the seat's named fix or from the ruling's acceptance check (:170-273). Log paths are
under .gate/u4f/. "Both builds" means the measured build and the bites build (`--all-features`).

| Item (seat) | The fix as built (pass diff) | Acceptance, as I read the logs | Fails first or plant | Departs? |
|---|---|---|---|---|
| FP1 (rust: RR-U4F-1, -3) | `drop_unencodable`: per-command retain, then the longest newest prefix that encodes whole, by trial encodes; the rest counted in `left_out`. `step()` builds the `Input` through a closure, no `input.clone()`, rebuilt only on the error path. The comment states the codec's two rules, which I checked against proto/src/input.rs:155-159: seq − 1 and a tick step 0..=255. `twin_left_out`'s doc names both causes | w4-fp1.log: carry80 "5 passed" (f25 and f25 (ii)); fmt 0; clippy both builds 0. f25 (ii) is a fixed-lead endpoint (`fed`: `with_lead(2)`): 107, then +300, then 407. The `Input` carries the newest (ack 405, tick 407); left out 1, unencodable 0 | w4-fp1-first.log: exit 101, `commands: []` (0 commands), on the reviewed lib.rs (diff hash 54cb6faa…, the reviewed tree's) | no |
| FP2 (netcode: NC-U4F-1) | `plan()` gains the arm `Some(t) if dist(goal, t) > Limits::REDUNDANCY_CAP as i64 => goal`, word for word, before the 2-a-step arm. The docs of `plan()`, `with_clock` and `stamps_skipped` name a reset or a stall | w4-fp2b.log: trace "7 passed". f32 (ii): skipped 339, least lead Some(3), late 0 of 170. Stall: skipped 29, late 0 of 269. f36: skipped 0, and f32 (i) asserts 0 (trace.rs:453, :537). f5's line prints skipped 0 on all six profiles. Its f5, f36 and f32 (ii) "most ahead, resets" figures are identical to the reviewed tree's (b3-client.log), so the new harness rule `host = next_due.max(host)` is inert there | w4-fp2-first.log: exit 101. Both cases' figures fail: skipped 0, lead −287, late 340 of 340; stall skipped 0, late 24 of 296. It ran on FP1's tree with `plan()` as reviewed, the ruling's order | no. The check counts the commands given from the mark on, by first arrival; at P0 that is the ruled "every stamp first arriving from then on" |
| FP3 (test: TE-U4F-1) | f5: late = ticks − on time − missing (arrived after `due(tick)`, the same rule as on time), counted apart from missing. Late is at most 1 % on all six profiles; the combined share is at most 1 % unless the profile has a burst. The 0.03 is gone; the doc says what the code checks | w4-fp3b.log: f5 passed. B1 late 0.00336, missing 23, not on time 0.01879; P4 0.00067; the rest 0 | none asked | no |
| FP4 (test: TE-U4F-2) | input.rs `mod tests`: `q9_a_…`, 10,000 sequences from 60,000 across the wrap, asserting at most K after each and K from the 16th on. f12 (iv): 1..=20 offered, then changed copies of 5 and 4 give forged `[0,0,1,0,0,0,0,0]` and outside 1, under both options (+0). This pins K = 16 both ways | w4-fp4b.log: q9_a passes in both builds; f12 "5 passed" in both builds; clippy 0 | w4-fp4-plant.log (`-x`): q9_a FAILED by name, "16: 17". The restore is proven by the whole pass diff: input.rs:229-231's `pop_front` is unchanged | no |
| FP5 (security: SEC-U4F-1) | `forger` returns `None` for `every < 2` and drops `every.max(1)`. Its doc states the precondition | w4-fp5.log: `f15_a_forgers_interval_below_2_is_refused` ok (1 is None, 2 is Some for kinds 1, 2, 4, 5, 6); f15's five pass; clippy 0 | none asked | no |
| FP6 (netcode: NC-U4F-2) | `--controller 1` in SETS[22].args (its bite takes `set.args`), in the header sentence, and in `never_ack_short`'s settings | w4-fp6.log: f21 and f14 (iii) pass; scenario.rs:3923 is f14 (iii), which is also f28's -22 half. Both end checks pass them. w4-hand.log: s22 and s22b use builder 2's settings (w2-hand.log) plus `--controller 1`. My read of w4-hand/: all six headers have `controller_max_lead_count` 9 and no `lead_ticks_count`. s22: headroom minimum 0, `overflow_resync` at t 91, `session_end` "kicked" at t 152. s22b: neither, headroom down to −3,765 | none asked | no |
| FP7 (security: SEC-U4F-2) | The header's `lead_bound_ms` is read in `run()` from `server.room.lead_bound().0`, beside `event_queue_cap_count`; `params()` runs before the room exists. Only the row's "Written by" cell changed (I compared cell by cell) | w4-fp7.log: f6 A and B pass (asserting 400 at scenario.rs:3655). `LEAD_BOUND_MS` is gone from the header write; the hand-run headers (room, lab, client) read 400 | none asked | no. Where it is written is the means; the room's value is what the seat asked |
| FP8 (arch: EA-U4F-8) | The ruled sentence, in the header comment (:24-25) and in r2.txt's head line | w4-fp8.log: `node --check` 0; grep finds :24 and :145. No reader takes r2.txt by line index (grep) | none asked | no |
| FP9 (S4, S5, S6) | USAGE gains the lag flags; the subsection title names `event.reorder.*` and the `reorder` rule; :7 reads "-47, -20, -22, then -21's" | w4-fp9.log: the greps hit. w4-fp9-check.log (`-x`): refs 1512 and marks 81 PASS, as in A2 and B | none asked | no |
| FP10 (netcode: NC-U4F-3 docs) | The three docs state the up-delay condition and C-U6-17. Docs only | w4-fp10.log: fmt 0; the grep hits the three files | none asked | no |
| FP11 (arch: EA-U4F-5 tie) | `CONTROLLER_MAX_LEAD` sits below the `use` lines, with `const _: () = assert!(… <= CMD_CATCHUP + 1 && … < ticks_for_ms_at(TICK_HZ, Room::<Sim>::LEAD_BOUND_MS))` | w4-fp11b.log (`-x`): both builds build; clippy 0 | none asked | no |
| FP12 (arch: EA-U4F-1, open files) | `join_tick()` is removed and no caller is left (grep). `KEPT_WINDOW` is pub(crate). f36 (ii) asserts `not_due()` 0. `fast_left_out()` is written per episode as `left_out_count` (cumulative: bots lib.rs:488), in U4f's own row; f19 asserts it 0 | w4-fp12b.log (`-x`): clippy 0 for the four crates in both builds; f36 (ii) ok; npt-client whole ok. w4-fp12.log: f19 and f28 fast-clock A and B pass, with `left_out_count` 0 | none asked | no |
| FP13 (arch: EA-U4F-4) | f18 and f15's kind 6 read `greybox().areas().rects[0]`, the extent room.rs:544 uses, inclusive as input.rs:302-303 is. FAR_CM's doc names f18 | w4-fp13.log: both pass; grep finds no `12_800` (grep's exit 1) | none asked | no |
| FP14 (arch: EA-U4F-6) | `ea_u4f_6_…`: TC's slot 0 walks [−127, 0] for 30 World steps; `moved_um` equals `MOVE_CM_S` × 10,000 | w4-fp14b.log: ok in both builds | none asked | no |
| FP15 (test: TE-U4F-3) | The bitten room under A gets seq 1 at j + 2, then seq 2 at j + 1; `refused` reads `[(2, 2)]` | w4-fp15.log: the ten bite proofs pass (`--all-features`) | w4-fp15-plant.log (`-x`): not-older-than FAILED by name (`[]` against `[(2, 2)]`); no-newer-tick stays ok. The restore is proven: the tracked diff hash went back to 624c1f79… (w4-fp16.log) | no |
| FP16 (test: TE-U4F-4) | f24 checks newest + 33 (dropped, `refused()` + 1, newest unchanged) and newest + 32 (taken, `(162, 0)`) | w4-fp16.log: f24 ok | none asked | no |
| FP17 (test: TE-U4F-7) | f2 loops over the two literal ids | w4-fp17.log: ok in the bites build; w4-fp17-deps.log: deps PASS | none asked | no |
| FP18 (test: TE-U4F-8, -5's phrase) | The ten doc lines name seeds 4, 5, 32, 33, 36, 47, 48, 20 and 30; f4 names e9 | w4-fp18.log: the grep finds ten lines. Each seed equals its `opts(…)` seed (my grep of trace.rs) | none asked | no |
| FP19 (rust: RR-U4F-5 (1), (2), (5)) | FastClock's three episode differences use `saturating_sub`, and so do the lag switch's two. `toward_centre`'s doc names the lane's offsets (its 32,768 + 3,600 is a true upper bound: LANE_X_CM 2,600, LANE_Y_CM −1,050) | w4-fp19b.log: f19, f20 and f28 fast-clock A pass; clippy 0. Its tracked diff hash is 2f07abc2…, the final tree's | none asked | no |

**End checks** (the ruling's :287-299):
- **Last lines:** "check: all 11 steps pass" in A2 (15:43:29-16:00:09) and B (17:44:46-18:04:57), both `-x`, both on the
  final hashes.
- **Test step:** 882.8 s and 834.0 s against 1,207 s.
  - This is longer than builder 3's 692.1 s and 685.6 s. The machine is slower: the C62H-BOUND reference test, which
    the pass did not touch, ran 13.59 s in w4-times.log against 11.41 s in w3-times.log (+19 %).
- **Self and provenance:** self 78 of 78. Every npt-* crate compiles from builder 4's worktree (A2 and B).
- **New tests:** w4-list.log has a `--list` line for each of the four new tests (lab in both builds). w4-times.log shows
  each at 0.00 s.
- **Item 4:** w4-item4.log has 0, 0, 6, 6, 20 and 17 passed.
- **Item 5:** w4-hand.log, every command exit 0, and `node --check` of both scripts exits 0.
- **Item 6** (the gate) belongs to the merge step.

## 3. Findings (CLAUDE.md's scale)
- **ADV-U4FP-1, Low (record): the snapshot sizes labelled "B" are character counts.**
  - worktree_patch.py prints `len(patch)` of a str. The final patch is 394,400 bytes (394,313 characters). Builder 3's
    74ae26aa… is 380,363 bytes (380,276 characters).
  - The wrong labels are in history.md:6978, commit 549ed77's message and the card's :178.
  - The sha256s are right and nothing reads the size. Correct the labels in the report and at the next card save.
- **ADV-U4FP-2, Low (process): builder 4's two named slips, plus one it did not name.**
  - The named slips:
    - two read-only commands chained in one call (P17), which I could not see on disk and take from its own account;
    - FP9's partial check.mjs (refs and marks) under the shared lock. It was re-run `-x` with the same 2 PASS
      (w4-fp9-check.log).
  - Not named: FP1-FP7's per-item clippy calls (16 in w4-fp1…fp7) ran `-s`, while u4f-run.sh's header asks `-x` for "a
    clippy confirmation". Builders 2 and 3 did the same (w2-b7, w2-b8a, w3-b8f, w3-b9a).
  - No counted result rests on any of them: the counted clippy and check.mjs runs (pre-doc, A2, B) were `-x`. The report
    names them; no action.
- **ADV-U4FP-3, Low (style):** client/src/lib.rs:360, in `with_clock`'s doc, is one 186-character line: FP2's sentence was
  appended without rewrapping. `plan()`'s doc leaves a short line ("… counted. No").
  - rustfmt.toml sets no `wrap_comments`, so fmt passes.
  - No push for it. It rides U4f2's lane, which opens client/src/lib.rs for C-U4F2-2's `twin_left_out` doc.
- **ADV-U4FP-4, Low (readiness): disk for the merge's counted run.**
  - At my check: 4,362 MiB free (`df -BM`), and the shared target holds 7,302 MiB.
  - M5 builds from the main folder's path, so every npt-* crate gets new artefacts (about 465 MiB by builder 4's cold
    rebuild, 6,730 to 7,195).
  - C1 handles it.
- **Notes, no finding:**
  - The judge was not run on the two FP6 hand runs, and FP6 names no such run. PM-0001-22's two checks (marks.json) read
    `tick.v.ack_starved_headroom_count` minimum = 0 and the `event.ack_starved.dropped` count ≥ 1. s22's room log has
    minimum 0 and two `ack_starved` events (t 91 dropped 0, t 152 dropped 1). -22's judged set is the proof.
  - f5 prints `stamps_skipped` rather than asserting it. f32 (i) and f36 assert 0, which the ruling's "unchanged" asks.

**Does anything need Mustafa? No.**
- No pass mark, judge, marks.json or existing SERIES.md row changed: D10's files are not among the 29, and SERIES.md has
  only additions against 5b29296b.
- No money, and nothing on main.
- 6,425 added lines sit under my 500-line guard (6,713). SIZE-U4F (a) covers them.

**The merge's readiness (M1-M9, advisor-u4f-r1.md:325-358):**

| Condition | Met now | The merge step does |
|---|---|---|
| M1 | yes: the snapshot is the final tree byte for byte; sha256 286aac60561ba684…; unchanged since 549ed77 (HEAD 50e1d01) | nothing more |
| M2 | yes: lane head 50e1d01 equals 07289a4 (and b25b96a) in spikes/netcode-proof and docs/design. Main is f5030a3 (`git ls-remote`), already in at a6acc8a | re-reads main just before the apply; if it moved with code, CI or rules: MAIN-52, plan first |
| M3, M4 | `git apply --check` exits 0 on 50e1d01 (my run) | `git apply`; `cmp` of the 29 files against builder 4's worktree; the runner's two hashes from spikes/netcode-proof (expected 2f07abc2… and 1a767d5b…, since HEAD's content there is b25b96a's) |
| M5 | no (not run yet) | C1, then one counted check.mjs run (11 steps, 634 and 731) with C2's limit |
| M6 | no (rule) | a failing test stops the merge and comes to me |
| M7 | no (not run yet) | `node tools/gate.mjs --no-smoke`; CI's gate and toolchain jobs green on the pushed head |
| D10 | yes, in the snapshot | runs `git diff --stat 5b29296b..<merge> -- …marks.json …crates/judge` (expected empty) and writes the report's own D10 line |
| DEP-HOLD | yes: no Cargo.toml, Cargo.lock, package*.json or .npmrc among the 29; the 9 new files are mode 100644 | nothing more |
| M8 | no (not built yet) | the one commit, as listed, plus FP20's items and C3's lines |
| M9 | no (comes after the push) | the saver, CI, the card; then 5.3's pre-runs (-21's rows an early read), the part-clean at A6's place, and the sets -47, -20, -22 |

The merge into the lane needs no go of his (his (a)); PR #40 keeps its own go.

## 4. What I ran and read
**Read:**
- main's advisor.md, whole;
- my ruling advisor-u4f-r1.md, whole (sha256 07c1bcd5…, checked);
- u4f-build.md:860-1170 (builder 4's block). It is byte-equal to the worktree's copy (`cmp`) and to HEAD;
- advisor-u4f-stop.md:78-130; advisor-u4f-build1.md:155-246; advisor-u4r-r2-full.md:8-16;
- the card's :176-198; history.md:6960-6995; u4f-run.sh, whole;
- the pass diff, whole (1,034 lines).

**Logs, read in full or by their summary lines:**
- every w4-* log: start; fp1-first and fp2-first; fp1 to fp19, with the b re-runs; the fp4 and fp15 plants;
  fp9-check; fp17-deps; pre-doc; end checks a, a2 and b; list, times, item4 and hand;
- w4-hand/'s six jsonl headers and events;
- for comparison: b3-client.log, review-test/f5-b1.log, w2-hand.log, w2-times.log, w3-times.log, and the w3 end checks'
  test lines.

**Code read in the final tree:**
- client lib.rs: `plan()`, `stamp`, `next_stamp`, `skip_command`;
- trace.rs: `run()`, `due`, `next_at` and the opts seeds; carry80.rs: `fed`;
- proto input.rs:129-180;
- room.rs:540-544; input.rs:299-306;
- bots lib.rs:336-372 and :466-494;
- scenario.rs: `run()`'s header write, f6, the never-ack short run, and the lag flags' parse;
- marks.json's PM-0001-22.

**Ran (all read only; my scratch files are in my scratchpad):**
- sha256sum and `wc -c` of both patches, and their byte and character counts;
- `git archive b25b96a` twice into the scratchpad, with each patch applied there by `git apply`;
- `diff -rq` of the final rebuild against builder 4's worktree (empty);
- `diff -ruN` of the reviewed rebuild against the final one (the pass diff);
- the runner's two hash commands in builder 4's worktree;
- `git diff --quiet` of b25b96a against 07289a4 and against HEAD (spikes/netcode-proof and docs/design);
- `git diff --stat 5b29296b b25b96a -- marks crates/judge` (empty);
- SERIES.md diffed from 5b29296b;
- a cell-by-cell compare of the lead_bound_ms row;
- `git apply --check` in the main folder (exit 0);
- `git ls-remote` for main and the lane;
- `df -BM` and `du -sm` of the shared target;
- greps.
- One `git status --short` in builder 4's worktree. It may refresh that worktree's index stat cache; no file changed
  (its hashes are equal after it).

**Not checked:**
- I ran no cargo, test, check.mjs, judge or gate. Every pass and count above is read from the builder's logs, and their
  headers tie them to the final tree's hashes.
- I did not read builder 4's transcript, so the P17 slip is taken from its own account.
- Not read: clock.rs; the parts of scenario.rs, room.rs and bots lib.rs outside the pass's hunks and the lines named
  above; and the review seats' notes.

## Decisions needed
None.

## For Mustafa
Nothing. The merge into the lane is the main session's, on M1-M9 with C1-C3.
