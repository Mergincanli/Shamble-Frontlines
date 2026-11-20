# Advisor check of U4f-F1's fix pass (2026-11-20), from main's advisor.md

In place of a second round (advisor-u4f-f1-r1.md:173-189, :241-255). Read only: git show/diff/grep/log, sed -n, wc,
sha256sum. Nothing run from the change; nothing written under spikes/ or .gate/. Builder's worktree:
.claude/worktrees/agent-ad5f72ed146da5f93 (read from that folder).

## Plan
1. The diff: the final snapshot's sha256 and size; equal to `git diff cb7e374 -- spikes/` in the worktree; the
   interdiff against the reviewed patch 37d87a10 (5 files); in src only `///` lines differ (FP-1: seam.rs:73-78,
   room.rs:353-357, :1328-1332); the sixth file named.
2. FP-1: the three doc comments read against FP-1's text.
3. FP-2 and FP-3: the tests at their lines; pass, plant and restore logs (f1-b5-*.log); the restored seam.rs hash
   901bb546 (after FP-1) against my table's 63057544 (before FP-1): which is right.
4. FP-4: t2_f's new asserts; the `word_if` helper and the pub(crate) departure: does it meet FP-4's intent.
5. The rustfmt layout in t2_f (f1-b5-pass2.log).
6. The end checks: f1-b5-check1.log (cut), -check1b, -check2: 11 steps, 642/739, wasm 0 mismatches.
7. The size: +239 -24, 6 files, against the 300 stop.
8. The builder's B5 notes.
9. FP-5: the carries (u4s-carries.md U4f2 C-U4F2-5..-10, U4g, U4h, U5 lines; plan.md:1253 C-U6-19), the readings-PR
   list and "After the close" on the card; each against my ruling.
10. Verdict, findings, the merge's steps.

## Block 1: the diff (DONE)
- Final snapshot wip/u4f-f1-build.patch: sha256 d0a66a68a16669a541624e3c23598b73ae25b61fd4984c7fbb2cf93a7d38829e, 19,794
  B (my runs); `git diff cb7e374 -- spikes/` in the worktree: the same sha256 and size (my run, from that folder);
  worktree HEAD cb7e3745; 6 files, numstat +239 -24 (room.rs 12/8, commands.rs 34/0, seam.rs 10/0, world.rs 39/16,
  tests/seam.rs 16/0, tests/world.rs 128/0). Every worktree file's blob id equals the final patch's index line.
- The reviewed patch: `git show f1e5737:.../wip/u4f-f1-build.patch` sha256 37d87a10bed52b81ebc7...20e3d7 (snapshot 1's
  commit); 5 files. The final first appears at 7a1190d (snapshot 2), unchanged since (HEAD f6f647e: clean).
- The interdiff (`diff` of the two patches, my run): src changes are room.rs's two `///` blocks (3 lines to 3, 3 to 4)
  and seam.rs's one (2 to 3), plus the hunk headers and index lines they move; sim/src/world.rs's hunk is byte-equal
  (blob 1faee05 in both). Filtered check: each src file's patch section, with `+///` lines, `@@` and `index` lines
  removed, is equal between the two patches (room.rs, seam.rs, world.rs: "same"). So in src only `///` lines differ:
  FP-1's condition holds.
- The sixth file is sim/tests/seam.rs (FP-2's test, +16). Test files otherwise: commands.rs (FP-3's move: the presses
  assert after the refusal assert, +1 comment line), tests/world.rs (FP-4, and the helper refactor below).
- One change beyond the table's text, in tests/world.rs: `live_seqs`'s local closure `word` became the module fn
  `word_if` (shared with t2_f), and `.filter_map(.. word(seq))` became `.map(.. word(&d, seq))`, the file's panicking
  `word` (tests/world.rs:84-89). Stricter, not looser: a live slot with no `.seq` word now panics where it was skipped;
  t2_f's two lists are unchanged. It serves FP-4 (the pool count needs the Option form). No finding.

## Block 2: FP-1, the three docs (DONE)
- seam.rs:73-78 (`edges_taken`): "npt-room's press series counts with this one rule (lost: the advance less these), so
  the room's count and the rule cannot drift; the room does not read the sim's take, which T1 and T2 test": FP-1's
  words. room.rs:353-357 (`presses()`) and :1328-1332 (`run_one`): lost is "the advance less the edges npt-sim's one
  rule [`Command::edges_taken`] takes, counted here with that rule and not read from the sim, so 0 while the rule takes
  every edge": FP-1's content. The code under them (room.rs:1340-1347, seam.rs:79-81) is the reviewed code.
- Acceptance: the three docs read so; only `///` lines changed in src (Block 1); check.mjs's doc step PASS on d0a66a68
  (check1b, check2; also in the cut check1). **Met.**

## Block 3: FP-2 and FP-3, tests, plants and the restore hash (DONE)
- FP-2: sim/tests/seam.rs:579-593, `f1_edges_taken_is_the_wrapping_advance`, the six pairs exactly as the table
  ((255,1) 2, (254,0) 2, (1,0) 255, (0,255) 255, (7,7) 0, (0,1) 1), message "{last} to {now}"; not feature-gated (+1 in
  each build). The task's ":579-596" includes the blank line and the next doc line; the fn ends at :593.
- FP-3: room/tests/commands.rs:635-643: one comment line (:635), then the refusal assert (:636-641), then the presses
  assert (:642-643). The test's asserts are otherwise the reviewed ones.
- Pass, f1-b5-pass.log (00:54, diff 78717dac): seam 9 passed (the new test ok), t2_ 6 passed, commands 27 passed (T1
  ok); `cargo fmt --check` exit 1 on t2_f's two lines only. I rebuilt 78717dac in memory (the final patch with t2_f's
  three unformatted lines and their blob id): sha256 equal. So that pass ran on the final content but for t2_f's layout.
  f1-b5-pass2.log (00:54:37, diff d0a66a68 = final): t2_ 6 passed, fmt --check exit 0.
- FP-2 plant, f1-b5-fp2-plant.log (00:54:45, diff 9b75089e, exit 101): FAILED by name at tests/seam.rs:591 "255 to 1",
  left 0, right 2. I rebuilt 9b75089e in memory (the final patch with seam.rs:80 `now.saturating_sub(last)` and that
  file's blob id 2dd9b9f): sha256 equal. So the plant was exactly the named one on the final tree, nothing else moved.
- FP-3 plant, f1-b5-fp3-plant.log (00:54:55, diff 40038079, exit 101): T1 FAILED by name at commands.rs:641 "the second
  edge, refused", left [], right [Refusal { slot: 0, seq: 4, kind: Cooldown }]: the refusal assert, as the table asks.
  Rebuilt in memory (the final patch with edges_taken's body as B2's plant, `let advance = now.wrapping_sub(last);` /
  `if advance < 1 { advance } else { 1 }`, two lines, hunk +69,17): sha256 equal. The exact plant, on the final tree.
- **The restore hash.** My table's 63057544...16499 was seam.rs before FP-1 (B2's restore). I rebuilt the reviewed seam.rs
  in memory (the final file with FP-1's three doc lines put back as the reviewed patch has them): sha256
  63057544bdc8eea58601aa0a2febef3af2ae3abb420d9d1adb109aa61be16499, git blob b757470 = the reviewed patch's index line.
  The final seam.rs is 901bb546c2bff638eb1fdd8667bd1e4f3676c5ac318f24e1e08da31834fe4c90 (my run). The plants ran after
  FP-1, so "seam.rs's sha256 as before" (FP-2's own words) is 901bb546; restoring to 63057544 would have undone FP-1.
  **The builder is right; my table's hash for FP-3 was stale** (written as if the plant ran on the reviewed tree). The
  correction is mine, not a departure. Both plants' trees differ from the final by the planted body alone (above), and
  the final seam.rs is 901bb546, so nothing of either plant is left.
- Counts: +1 measured, +1 bites (641 to 642, 738 to 739: B4's logs against check1b/check2). **FP-2 and FP-3 met.**

## Block 4: FP-4 and its departure (DONE)
- The departure is forced: scene.rs:123 and :134 make `grenade_fuse_ms` and `projectile_capacity` `pub(crate)`, and
  sim/tests/world.rs is an integration test (another crate). A pub accessor would change code tokens in src, which FP-1's
  condition and my Block 4 exclude.
- Its route pins the same two facts, through what the World builds from them: world.rs:865 sizes `projectiles` from
  `scene.projectile_capacity`, and the dump writes every pool slot's `live` (world.rs:2034-2036), so the count of
  `projectile[i].live` words is the capacity: asserted 16, "Scene::course()'s projectile_capacity". world.rs:1556 sets
  each grenade's `fuse` to `ticks_for_ms(grenade_fuse_ms)`, a total (not a countdown: the blast test is
  `tick - start_tick >= fuse`, :1719), dumped at :2045; one throw at tick 0 into an empty pool lands in slot 0; asserted
  `fuse > 20`, "Scene::course()'s grenade fuse, {fuse} ticks, outlasts the stream's 20". t2_f's streams run ticks 0-19
  and throw from tick 0, so a total fuse above 20 ticks keeps every grenade live to the lists' read: the pin the lists
  need. If slot 0 were not the grenade, its fuse word reads 0 and the assert fails, so it cannot pass by accident.
- It is first in t2_f, each assert has a message naming what it pins. The pin is arguably closer than reading fields:
  it tests what the World does with them, which is what the lists rest on. **Meets FP-4's intent. Ruled: accepted.**
- By reading, as the table's acceptance says; it passes in pass.log, pass2.log and both check.mjs runs.

## Block 5: the rustfmt layout (DONE)
- pass.log's fmt diff asked for two layouts in t2_f: `room_tick(...)` split over four lines and the `assert!` split.
  The builder applied the assert's split as rustfmt gave it, and for `room_tick` named the stream (`let throw =
  course_stream(1, |_| 0, &[(2, 0)]);`, then `room_tick(&mut world, &mut ring(), &throw[0]);`), a layout rustfmt also
  accepts (pass2.log: fmt --check exit 0 on d0a66a68; check.mjs's fmt step PASS twice). Not rustfmt's diff word for
  word for that one statement, but by Edit, no tool wrote a file, the value is the same, and B5's notes say so. The rule
  ("never `cargo fmt` writing files: apply rustfmt's diff by Edit") exists to keep tools from writing; it is kept. No
  finding.

## Block 6: the end checks, the size, the notes (DONE)
- f1-b5-check1.log (00:55:24, diff d0a66a68): fmt, clippy, clippy-wasm32, doc PASS, then nothing (the container restart
  about 01:01 UTC); no test step, no last line. Kept as it is and not counted: right.
- f1-b5-check1b.log (01:04:53-01:27:15, diff d0a66a68, exit 0) and f1-b5-check2.log (01:27:29-01:44:19, the same diff,
  exit 0): "check: all 11 steps pass", every step PASS (fmt, clippy, clippy-wasm32, doc, test, deps, wasm, refs, marks,
  self, redact); test "measured build: 642 tests passed, 0 failed, 0 reported a skip; bites build: 739 tests passed, 0
  failed, 0 reported a skip" in both; wasm "27000 ticks compared, 0 mismatching" native-native and native-Wasm in both;
  marks 81 checked, docs/adr rows unchanged. The two logs differ only in times, free space and the hash-map bite's
  native digests, which vary per process by design (B4's two runs differ there too: af427be8/ff2a4e65 against
  d47d5214/85539222); both catch it at SS-s1 tick 1234. 642 = 634 + 8, 739 = 731 + 8 (T1, T2's six, FP-2's one; B4 had
  641/738). f1-b5-clippy.log: clippy on npt-sim and npt-room, all targets, measured and all features, -D warnings, exit
  0 both. **The end checks are met: two full runs on the final tree.**
- Size: +239 -24, 6 files, no new file (my numstat): 239 of the 300 stop. My ruling's estimate was 225-235; +4 over its
  top from the rustfmt layout of t2_f (+4 lines: 235 to 239, B5's SIZE-CHECKs). Inside the stop.
- The builder's notes (u4f-f1-build.md:132-196, the B5 section; the main folder's copy equals the worktree's, sha256
  77655104...f089): each claim I checked above holds (hashes, logs, counts, the order FP-1 then the plants, the
  departure stated as one with its reason, the stale hash stated as a finding with its reason). The disk STOP (00:49) is
  recorded with what it did not touch. Nothing in them is untrue that I found.

## Block 7: FP-5, the main session's records and carries (DONE)
Each line read against advisor-u4f-f1-r1.md (my ruling).
- **U4f2** (u4s-carries.md:548-552): C-U4F2-5 to -8 by pointer (advisor-u4f-r2.md:87-97, :110-113, :127-129;
  advisor-u4f-merge-head.md:57-64: each range opens on its route, read); C-U4F2-9 = my :126-127, word for word in
  substance; C-U4F2-10 = my :159-165, in short form with the pointer. **Match.**
- **U4g** (:579-584): TE-F1-5 beside 47 (:147-150), NC-F1-4 beside 47 (:156-158), -58's wrap start (:123-125), the
  "no such client" line (:120-122). **Match**, but the TE-F1-5 line drops my "else left for crates/sim's next opener"
  (:149): ADV-U4FF1P-3.
- **U4h** (:587-588) and **U5** (:689-690): the "no such client" line with C-U6-19, :120-122. **Match.**
- **C-U6-19** (plan.md:1253): its quoted text equals my :112-119 word for word (compared by script, whitespace only);
  its gate equals :120-122 and its security line :131-133; header cites :111-122. **Match**, but it does not say "with
  C-U6-13 and C-U6-16" (:111), the reason it sits in U6 (the same JoinAccepted change and review, :103-105):
  ADV-U4FF1P-2.
- **The readings PR before R5** (netcode-proof-test.md:123-127): NC-F1-3's three sentences (:153-155), SA-F1-3's v2
  rule (:166-169) and C-U6-19's session-start sentence (:119). **Match**, but SA-F1-3's rule is cut to "every press
  action has server-side rate state", dropping my ", or takes at most one edge per command" (:168): ADV-U4FF1P-1.
- **After the close** (netcode-proof-test.md:138-139): the judged "presses seen equal the scripted count" check for -20
  and -58, his, with :220-221 cited. **Match.** It sits as "Beside it" inside ADV-U4F20-2's bullet rather than a line of
  its own; acceptable, both are his marks.json items.
- **Block 6's entry for his next message** (my :207-221: the round and pass, "0 presses lost" by design, the
  first-command gap to U6 before -19): not on the card's "Choices taken without him" list (netcode-proof-test.md:90-106,
  where every other "to name in the next day's message" line lives) and not in msg-2026-11-20.md. Not FP-5's table,
  but my ruling's: ADV-U4FF1P-4.
- **The record lines (i)-(v)** go into F1's report and the -20 L1 record, not written yet: read there (FP-5's acceptance,
  "in the -20 record"), before that verdict counts. (v) now also carries this check's correction: my table's FP-3 hash
  was the pre-FP-1 one (Block 3).
- The lane head: `git diff cb7e374 HEAD -- spikes/` is empty and the 27 commits since cb7e374 touch only
  `.checkpoints/` (my runs), so the snapshot applies onto the lane as built.

## Each FP against its fix and its acceptance check
| FP | Fix as ruled | Acceptance | Departs? |
|---|---|---|---|
| FP-1 | the three `///` docs, FP-1's words | docs read so; only `///` lines in src (filtered interdiff equal); doc step PASS x2 | no |
| FP-2 | tests/seam.rs:579-593, the six pairs | pass (pass.log); plant `saturating_sub` FAILED by name :591 (diff 9b75089e rebuilt equal); restored 901bb546; +1/+1 | no |
| FP-3 | commands.rs:635-643, refusal assert first | pass; plant min(advance, 1) FAILED by name :641, [] against [slot 0, seq 4, Cooldown] (diff 40038079 rebuilt equal); restored 901bb546 | no: my table's 63057544 was stale (pre-FP-1); 901bb546 is "as before" |
| FP-4 | t2_f's first asserts: pool 16, fuse > 20 ticks, messages | passes; by reading: pins both facts through the World that sizes and fuses from them | in means only (pub(crate) fields); the intent and the check are met: accepted |
| FP-5 | records and carries | carries filed and matching in substance; four Lows on wording/placement; (i)-(v) read in the -20 record | no (Lows below) |
| End | check.mjs x2, 642/739, snapshot hash | check1b and check2 on d0a66a68: 11 of 11, 642/739, wasm 0 of 27,000; check1 cut, kept; d0a66a68 = worktree diff | no |

## Findings (CLAUDE.md's scale; none in the snapshot)
- **ADV-U4FF1P-1, Low**, netcode-proof-test.md:126: SA-F1-3's v2 rule cut to "every press action has server-side rate
  state"; my ruling's rule has an alternative (advisor-u4f-f1-r1.md:168). Fix: "(every press action has server-side
  rate state, or takes at most one edge per command)".
- **ADV-U4FF1P-2, Low**, plan.md:1253 (C-U6-19): the pairing with C-U6-13 and C-U6-16 (r1 :111; why: :103-105) is only
  reachable through the cited range. Fix: after its header add "with C-U6-13 and C-U6-16 (the press base rides the same
  JoinAccepted change and review)".
- **ADV-U4FF1P-3, Low**, u4s-carries.md:579-580: TE-F1-5's line drops "else left for crates/sim's next opener" (r1
  :149), so a U4g that declines it hands it to no one. Fix: add those words to the line.
- **ADV-U4FF1P-4, Low (due before his next message)**, netcode-proof-test.md:90-106: my Block 6's entry for his next
  message (r1 :207-221) is not on "Choices taken without him" nor in msg-2026-11-20.md. Fix: file it there as a "to
  name in the next day's message" line (the round and its pass checked by the advisor with no second round; "0 presses
  lost" now holds by design, the runs showing presses equal the scripted 432,000; the first-command gap to U6 before
  -19), so the 2026-11-21 message carries it (or 11-20's, if that has not gone out).
- **Correction (mine, for record line (v)):** r1's FP-3 acceptance named seam.rs 63057544...16499 as the restore hash;
  that is the pre-FP-1 file (blob b757470). With FP-1 in, "as before" is 901bb546...4c90, which the builder used.
All four are Lows on checkpoint records: none touches spikes/ or the snapshot, none needs a push of its own or a
re-check by me; the main session fixes them by Edit at its next checkpoint (ADV-U4FF1P-4 before the next message).

## Verdict
**APPROVE WITH CONDITIONS: the pass merges. No round 2.** FP-1 to FP-4 are built as ruled and meet their acceptance
checks, proven by logs whose tree hashes I rebuilt; FP-4's route through the World meets its intent; the restore hash
901bb546 is right and my table's was stale; the end checks are two full 11-of-11 runs on the final tree, 642/739, wasm
0 mismatches; 239 of 300; FP-5's carries are filed and match. No item departs from its named fix or acceptance check,
so no seat is recalled. Conditions:
1. The four Lows above, fixed by Edit in the records (not blocking the merge; ADV-U4FF1P-4 before his next message).
2. The merge as expected (r1 Block 4; advisor-u4f-f1-plan.md:116-118): the main session applies the snapshot to the lane
   branch and confirms `git diff cb7e374 <its commit> -- spikes/ | sha256sum` is d0a66a68...829e; `node tools/gate.mjs
   --no-smoke` PASS; check.mjs in the main folder, all 11 steps, 642 measured and 739 bites (the runner's 4 GB floor:
   free space first); CI green on the pushed head; my head check of that head (not a round; a later push needs it
   again).
3. Then -20's L1 re-run from the merge commit (condition 5), held to my expected figures (advisor-u4f-f1-plan.md:119-140,
   restated in r1 Block 5): summary lines equal to runs.txt:23-29 but wall time and binary sha256s; per-tick `h` equal
   to 350677f's kept logs; lost 0 (by construction), doubled 0; presses B 432,000 each, A 432,003 / 432,003 / 432,009;
   -2 SS PASS 6/6; -20 A and B PASS 3/3; L2 NOT-RUN; the bite at 0.894544 with lost 3,614 to 0; the reorder row "not
   available". Any other difference stops for me. The -20 L1 record and F1's report carry FP-5 (i)-(v); I read them
   there.
4. Then condition 10: U4f2's brief rebased on F1's merge, with C-U4F2-9 and -10 as ruled.
PR #40 stays needs-mustafa: SA-F1-1 is an open security finding until C-U6-19 closes it (advisor.md step 4).
Not checked by me: nothing was run (cargo, check.mjs, the lab): every pass/fail figure above is read from the logs,
with the logs' tree hashes rebuilt by me in memory to tie each log to the tree it names.
