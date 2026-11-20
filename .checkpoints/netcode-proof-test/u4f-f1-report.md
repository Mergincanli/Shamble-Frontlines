# U4f-F1's report: the stand-in takes every press edge (ADV-U4F20-1), and -20's L1 set again (2026-11-20)

## In plain words
- -20's option-B part failed "0 presses lost" (3, 1 and 2) at 350677f. After late commands are dropped, U4f's room runs
  a command whose press counter is two presses on, and the stand-in simulation took only one press per changed counter.
  ADR 0001 part 5 keeps a press across a dropped command, so the defect was the stand-in's, not option (b)'s.
- U4f-F1 makes the simulation take every press through its usual checks (cooldown, ammunition, reload), with one shared
  rule that the room also counts with. Merged into the lane at 1faf894; CI green.
- -20's L1 set ran again on the fix (46ca143): every part PASSES, and every figure equals 350677f's except the lost
  presses, now 0. No pass mark, judge rule, marks.json text or run count changed; no money; nothing on main.

## What was built (1faf894: snapshot d0a66a68…829e, 6 files, +239 -24; 239 of its 300 stop)
- **The one rule:** `Command::edges_taken(last, now) -> u8` (sim/src/seam.rs:79) = `now.wrapping_sub(last)`,
  integer-only (U4FF1-Q1: one `pub fn` in npt-sim, called by npt-room).
- **npt-sim** (`run_command`, world.rs:1348): each button's edges come from the rule, taken in today's order: reload,
  fire (one shot per command; a second fire edge meets the cooldown or the ammunition and is refused, state-neutral),
  throw. A refused edge leaves no trace in the state (world.rs:30-41).
- **npt-room** (`run_one`, room.rs:1343): seen, lost and doubled are counted with the same rule; the docs (seam.rs:73-77,
  room.rs:353-357 and :1328-1331, FP-1) say the room counts with the rule and does not read the sim's take.

## How it is tested
- **T1** (room/tests/commands.rs:618): option B, two late drops, then a command two edges on: seen 2, lost 0, doubled 0,
  and the refusal [slot 0, seq 4, Cooldown] asserted first (FP-3). Plants: the old one-edge rule, and `min(advance, 1)`
  re-run after FP-3: each fails by name.
- **T2** (sim/tests/world.rs:1826-1921, the d10 twin form, cases (a) to (f)): two fire edges against one give equal state
  hashes when the weapon fires once; edges in the cooldown or on the last round are refused as Cooldown or Ammunition;
  two reload edges start one reload; each throw edge throws or meets a full pool, the pool of 16 and the fuse longer
  than the test's 20 ticks pinned first (FP-4). Two plants fail by name.
- **FP-2** (sim/tests/seam.rs:582-596): the rule across the u8 wrap: (255, 1) 2, (254, 0) 2, (1, 0) 255, (0, 255) 255,
  (7, 7) 0, (0, 1) 1. Plant `now.saturating_sub(last)`: fails by name at :591.
- **check.mjs**, all 11 steps: twice in the builder's copy on the final snapshot, and once in the main folder on the
  merged tree (01:54:18-02:13:49 UTC, .gate/u4f-f1-merge/m5-check.log): 642 measured and 739 bites tests, 0 failed,
  0 skipped (+8 on U4f's 634 and 731); wasm 27,000 ticks, 0 mismatching; marks 81, unchanged.
- **The gate** `--no-smoke` PASS on the merged tree (.gate/u4f-f1-merge/gate.txt); **CI green** on 1faf894 (gate,
  smoke, toolchain).

## Reviews and the advisor's checks
- **Plan** rev 1.1: APPROVE WITH CONDITIONS 1-10 (advisor-u4f-f1-plan.md).
- **Round 1, the only round:** rust-reviewer, test-engineer, netcode-architect and security-auditor, each APPROVE WITH
  FINDINGS (u4f-f1-review-{rust,test,netcode,security}.md). Mediums: TE-F1-1 = NC-F1-1, NC-F1-2, SA-F1-1. Every
  finding's route: the ruling's table (advisor-u4f-f1-r1.md:223-239).
- **The round ruling:** APPROVE WITH CONDITIONS FP-1 to FP-5, no round 2 (no High, no design change in the pass: the
  2026-11-17 choice, history.md:6818-6819).
- **The check of the pass:** APPROVE WITH CONDITIONS (advisor-u4f-f1-pass.md:181). **Correction (ADV-U4FF1H-1):**
  1faf894's commit message calls it "APPROVE"; the verdict was APPROVE WITH CONDITIONS. No history rewrite; 46ca143's
  message carries the same correction.
- **The head check of 1faf894:** APPROVE WITH FINDINGS (advisor-u4f-f1-head.md): ADV-U4FF1H-1 (above) and -2 (two
  times on the card), both Lows, fixed in the records.

## The fix pass, FP-1 to FP-4 (no item departs from its fix or its check: advisor-u4f-f1-pass.md:151-159)
- FP-1: the three docs bound (only `///` lines changed in src). FP-2: the wrap test. FP-3: T1's refusal assert first.
  FP-4: T2 (f)'s two pins.
- **Two departures in means, accepted:** FP-4 reaches the pins through the World, so scene.rs's `grenade_fuse_ms` and
  `projectile_capacity` became `pub(crate)`; seam.rs was restored after the plants to its post-FP-1 hash 901bb546…4c90
  (the ruling's 63057544…16499 was the pre-FP-1 file: the advisor's own correction, below in (v)). One rustfmt layout.

## The record lines FP-5 (i)-(v) (advisor-u4f-f1-r1.md:248; the -20 L1 record in u4f-report.md carries them too)
- **(i), Block 1's sentence:** "'0 presses lost' is 0 by the press-counter design after U4f-F1: npt-room counts with the
  rule npt-sim runs (`Command::edges_taken`), as '0 presses doubled' has been 0 by design since U4f; it guards that rule
  against the room's own wrapping difference, not the sim's take. The sim's take of every edge is proven by T1 and T2
  and their planted bugs. The run's evidence that no scripted press went missing is presses = 432,000 per option-B run,
  the scripted count (48 x 18,000 / 2); the per-tick state hashes equal to 350677f's show the 1-edge path unchanged and
  the extra fire edge state-neutral at run scale." (advisor-u4f-f1-r1.md:67-72)
- **(ii), Wasm:** by reading only: the rule is integer-only and an extra throw runs the single throw's float path; the
  wasm step's 27,000 ticks are not shown to hold a multi-edge command (RR-F1-4 = TE-F1-4: counting them needs a program
  run on the recordings, not worth it for a Low).
- **(iii), the throw's rate state:** the stand-in's throw has no cooldown, so neither T2 (f) nor any -21 figure shows ADR
  0001:236's ability check; a press bank of up to 127 throw edges bursts in one tick (SA-F1-3; the v2 rule "every press
  action has server-side rate state, or takes at most one edge per command" goes to the readings PR).
- **(iv), open and declined:** SA-F1-1 (Medium) stays open until C-U6-19 closes it (Security, below). RR-F1-1, the
  full-pool rescan: no action, because it is bounded (at most 255 x the pool's `live` reads per command, no allocation),
  its large reach leaves with C-U6-19's first command or a bites build, kind 5 caps a command at 127 edges inside a
  session, and changing `throw`'s return would move T2 (f)'s exact lists for no measured gain.
- **(v), the premise corrected:** the advisor's plan review (advisor-u4f-f1-plan.md:143-145) said the count "is still what
  the sim does not take, and the plant shows it can fail". As built, it is what the one rule does not take; the plant
  edits the rule, so room and sim move together. T1 (its refusal assert) and T2's count halves are what tie the sim's
  take to the rule. And r1's FP-3 restore hash 63057544…16499 was the pre-FP-1 file (blob b757470); with FP-1 in, "as
  before" is 901bb546…4c90, which the builder used (advisor-u4f-f1-pass.md:175-176).

## -20's L1 set on the fix (the record: u4f-report.md, "After the merge", item 5)
- Run at 96de6ac (a card save), 02:22-03:07 UTC, whose spikes/netcode-proof tree d4f6cd41b8d7… is 1faf894's; results
  committed at 46ca143 (results/u4f/96de6acf8223/pm-0001-20/). PM-0001-2's SS part PASS 6 of 6; PM-0001-20 L1 options A
  and B PASS 3 of 3 each (the mark's headline reads NOT-RUN: its L2 parts are not run yet); the bite shown.
- Held to the advisor's expected figures (advisor-u4f-f1-plan.md:119-140): all equal. Only the lost check and its
  reasons differ from 350677f (14 of 593 verdict leaves, 2 of 65 in the bites' verdict).

## Carries filed (FP-5; each read by the advisor, advisor-u4f-f1-pass.md:122-149, its four Lows fixed)
- U4f2 (u4s-carries.md): C-U4F2-9 (a record line: -21's report names kind 5's first-command bypass, that no -21 forger
  rejoins or throws, and C-U6-19 as its closure) and C-U4F2-10 (SA-F1-2's own-kind case under no-press-edge, with its
  plant; inside f33's range, no size change).
- U6 (plan.md:1253): C-U6-19, a session's first command, with C-U6-13 and C-U6-16; closed before -19 runs.
- U4g: TE-F1-5 (else left for crates/sim's next opener), NC-F1-4 beside carry 47, -58's press start, the "no such
  client" line; U4h and U5: the "no such client" line.
- The readings PR before R5: NC-F1-3's three sentences, SA-F1-3's rule, C-U6-19's session-start sentence.
- After the close (his, a marks.json change): a judged "presses seen equal the scripted count" check for -20 and -58.

## Security (for PR #40's report, in a line of its own)
- **SA-F1-1, Medium, open:** a session's first command is not edge-checked (room/src/input.rs:287), so a hostile client
  could throw a pool of grenades at once on each rejoin, and a restarted honest client makes phantom presses. No run
  before -19 meets it (no -20, -21 or -22 client rejoins or throws). It closes with C-U6-19 before -19's runs. PR #40
  stays needs-mustafa.

## Size (SIZE-CHECK; the plan: about 60-150, size S, the stop past 300)
- The build 205 added (37d87a10…), the pass +34: 239 added, 24 removed. Under the stop.

## Timing (UTC), and the cuts
- 2026-11-19 20:53:34-22:40:41: a spend-limit cut (during the advisor's -20 ruling, before F1); 23:18:18: a container
  restart stopped the builder (no half edit, resumed); about 00:0x: built; 00:49: the runner's 4 GB floor stopped the
  pass (part-clean, resumed); about 01:01: a container restart stopped the pass's first check.mjs (re-run, kept);
  01:44: the pass done; 02:14:10: 1faf894 pushed; 02:22-03:07: -20's L1 set. Each cut's notice, word for word, is in
  history.md (:7148-7175) and on the card.

## What was not verified
- That the Wasm build runs a multi-edge command the same way (by reading only: (ii)).
- The sim's take at run scale through the lost count: by design it cannot show it ((i)); the run-scale evidence is the
  press count equal to the scripted one and the per-tick hashes.
- The first-command gap (SA-F1-1): not built, not tested; C-U6-19.
- -20's L2 parts: not run (they come with the run days).

## Decisions needed
None from F1. (U4F2-SIZE, sent in the 2026-11-20 message, still waits for his answer; it is U4f2's, not F1's.)

## For Mustafa
Nothing to do. The 2026-11-21 message names this step under "Choices taken without you" (the card's 2026-11-20 entry).
