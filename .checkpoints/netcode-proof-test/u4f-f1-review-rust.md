# U4f-F1 review, rust-reviewer seat (one round, 2026-11-20)

Read only (git show/diff/grep, sed -n, sha256sum, the evidence logs). Nothing run under cargo. Diff: worktree
`.claude/worktrees/agent-ad5f72ed146da5f93`, `git diff cb7e374 -- spikes/` (sha256 37d87a10bed52b81..., equal to
`.checkpoints/netcode-proof-test/wip/u4f-f1-build.patch`, 17,495 B: checked; 5 files, +205 -24). The worktree's
seam.rs and world.rs hash 63057544... and 4a4e138c..., the builder's restored hashes (u4f-f1-build.md:80, :113): the
plants are out. Line numbers below are the worktree's.

## Plan
1. seam.rs `edges_taken`: integer-only, const, wrap semantics, doc.
2. world.rs `run_command`: the loops (reload, fire, throw), bounds, cost per edge, allocation, order vs condition 1,
   1-edge path unchanged, clock once, the bites.
3. room.rs `run_one`: counts' arithmetic (overflow, saturating), doubled on repeat.
4. Determinism wasm32 vs native: any float, HashMap, usize-width dependence introduced.
5. Panic / overflow paths reachable from client input (255 edges, refusal Vec growth).
6. Tests T1, T2 from the Rust side; clippy/fmt/doc evidence in logs.
7. Findings, verdict.

## Block 1: the shared rule (seam.rs:72-80)
- `pub const fn edges_taken(last: u8, now: u8) -> u8 { now.wrapping_sub(last) }`: integer-only, no panic
  (`wrapping_sub` never traps, overflow-checks on or off: Cargo.toml:116-118 keeps them on for npt-sim in release),
  const-callable, documented; its doc's cites hold (ADR 0001:226 "a resent command can neither lose nor double a
  press"; :234-237 "a press counter advances by at most one edge ... fire, ability and reload edges meet the server's
  own cooldown, ammunition and reload state").
- Wrap: (250 -> 2) = 8, (7 -> 7) = 0, (now = last - 1) = 255. The last is the rule's one sharp edge: a counter that
  moves *back* reads as 255 minus the step forward (RR-F1-3). No test asserts an advance across 255 -> 0 above one edge
  (RR-F1-2).
- Condition 2's contract: the fn gives the edges taken; the sim loops that count; the room's lost = advance - taken
  (0 while the rule is the identity). Met. One copy: grep finds no other `wrapping_sub` press rule in sim or room
  (the bots' kind-5 mirror at bots/src/lib.rs:1108 is the client's own check, as before).

## Block 2: run_command (world.rs:1338-1511)
- **1-edge and 0-edge paths are today's, token for token.** `edge = edges.map(|n| n > 0)` (:1350) equals the old
  `c.presses[k] != claimed[k]` for every value (advance > 0 iff the counters differ). Reload: `for _ in 0..edges[R]`
  (:1426) runs today's body 0 or 1 times. Fire: the first edge (bites at :1446-1460, the kind test, `fired`) reads
  `edge[FIRE]` as before; the further-edge loop `1..edges[FIRE]` (:1490) is empty for 0 and 1 (`1..0` and `1..1` are
  empty ranges, no panic). Throw: `0..edges[THROW]` (:1506) under `authority`, as the old `edge && authority`.
  So World::BITES and every 1-edge pin hold by construction; the logs agree (641/738, +7 each, 0 failed).
- **Condition 1's order and rule:** the clock once (:1415-1422, outside every loop); reload edges, then fire, then the
  further fire edges, then the flags and the write-back, then throw edges: today's places. Fire: at most one shot
  (`fired` set once, `s.wait += cost` once, :1473-1476); each further edge refused by the kind test on the local
  `s` after the shot (Ammunition iff `s.ammo == 0`). Reload: each edge against the state the last left (the first
  start sets `reload_left > 0`, so the rest are refused). Throw: each edge throws or PoolFull. Melee: untaken, as today.
- **Predict (authority = false):** the reload loop runs (state-changing, the same on both sides); the further fire
  edges and the throws are authority-only and change no state; `fired` unchanged. No predict/authority split added.
- **Bounds and cost per edge.** Each loop is at most 255 iterations of O(1) work, but the throw's: `throw` scans the
  pool (`position`, :1526) per edge, so once the pool is full each further edge is a full scan before its PoolFull:
  255 x 320 (SS, scene.rs:181) = 81,600 `live` reads for one command, once per session at most (RR-F1-1). Inside a
  session kind 5 bounds an advance to the ticks covered and below 128 (input.rs:285-299), about one edge a tick.
- **Allocation:** no heap work on the 1-edge path. A refused edge pushes one `Refusal` (u8, u16, enum: seam.rs:507-514)
  into `reports.refusals`, which `step` clears keeping its capacity (:2170): amortised, no allocation per edge in
  steady state; a burst grows it once (worst case per command about 765 entries: 255 reload + 255 fire + 255 throw).
- **Overflow (checks on in release for npt-sim):** no new arithmetic but `wrapping_sub` and the u64 report counters
  (+1 per edge). `throw`'s `spawn_acc -= 30,000` (i64) runs only after a free slot is found, so per tick at most the
  pool's capacity: unreachable. Range<u8> iteration to 255 cannot overflow. No new index: `edges[Command::PRESS_*]`
  are constant indices into `[u8; 4]`; `claimed[slot]` is behind the existing `slot < soldiers.len()` return (:1340).
- **reload_ms = 0 (note, pre-existing, no finding):** World::new admits it (`w.reload_ms <= RELOAD_MAX_MS`, :740, no
  floor); then every reload edge "starts" a zero reload unrefused, so the comment at :1424-1425 ("which refuses the
  rest") would not hold; but such a weapon never refills either (the refill needs `reload_left > 0`, :1417-1422), and no
  scene has it (scene.rs:68, :241: 2,000 ms). State-neutral; outside F1.

## Block 3: run_one (room.rs:1328-1372)
- `advance = u32::from(p.wrapping_sub(presses[k]))`, `taken = u32::from(Command::edges_taken(presses[k], p))`
  (room.rs:1341-1342): equal by construction, so seen += advance, lost += 0 (`saturating_sub` guards a plant that
  returns more than the advance), doubled += taken (= advance, the old value). u32 per-tick sums reset each tick
  (room.rs:388): at most 4 x 255 per command run, unreachable overflow.
- Repeat bookkeeping (pre-existing, checked for drift): the sim's `claimed` takes a repeat's counters (world.rs:1351),
  `run_presses` does not (room.rs:1350-1352). A repeat is `last_run` (input.rs:362-381), whose counters equal
  `run_presses` since its own run set them, so its advance is 0 in both and the two references stay equal. No drift
  added.
- npt-room stays float-free (the deps step: "npt-room's sources hold no float").

## Block 4: identical results on wasm32 and native
- The rule and the loops are integer-only; no HashMap, no usize-width value (u8 counts, u8 ranges), no clock. The
  only float work on an extra edge is `throw`'s (`sqrt`, `/`, `*`: IEEE correctly rounded on both targets), the same
  code a single throw runs; the deps step shows no fma, simd128 or relaxed-simd and no rustflags (check1.log:10).
- check.mjs's wasm step: native-native and native-Wasm, 27,000 ticks each, 0 mismatching (check1.log:11, check2 the
  same). Whether those runs hold a multi-edge command is not counted (the synthetic recordings lose a command now and
  then and press fire 1 in 40 and throw 1 in 400: replay.rs:346-350, :510-521), so the multi-edge path's parity is by
  reading, not by a run shown to contain it (RR-F1-4).

## Block 5: tests and tooling (Rust side)
- T1 (commands.rs:618-643): option B, j+2/j+3 late, cmd 4 two fire edges on: (2, 0, 0) and the sim's one Cooldown
  refusal with seq 4. Sound; `Vec == [T; 1]` comparison is std.
- T2 (world.rs:1813-1920): `two_edges` builds forged/twin via `course_stream` with a step named twice (each listed
  counter +1 at its step: :1609-1612); `twins` asserts hashes, events and dumps every tick. t2_f reads `pool_full`
  (cumulative since the world began: seam.rs:540-550) against the refusals gathered per tick (cleared each step):
  consistent. `live_seqs` stops at the first missing `projectile[i]` (`map_while`): terminates.
- Plants by name: f1-b2-plant.log (T1 at commands.rs:636), f1-b3-plant1.log (all six at their count asserts),
  f1-b3-plant2.log (a-d at world.rs:573, the twin half). Restored hashes match the worktree now (above).
- fmt, clippy (measured and bites, host and wasm32, -D warnings), doc (rustdoc warnings as errors, host and wasm32):
  PASS in both check.mjs runs (check1.log:5-8). The intra-doc links to `Command::edges_taken` (room.rs:356, :1329;
  world.rs module doc) resolve (doc step).
- No unsafe, no new dependency (deps step PASS). Wasm size and a bench before/after: not measured (no seat rule ran
  cargo); expected negligible (three small loops, no dep). Not verified.

## Findings
| Id | Severity | Where | What goes wrong | Fix |
|---|---|---|---|---|
| RR-F1-1 | Low | sim/src/world.rs:1506-1508, :1526 | Each throw edge rescans the pool; once it is full every further edge is a full scan before its PoolFull: O(edges x pool), 255 x 320 = 81,600 reads on one command (a session's first command, or no-press-edge bitten). Bounded, no allocation, once per session | `throw` returns whether it threw; on the first PoolFull push the remaining edges' refusals (one per edge, so twins and lists are unchanged) without rescanning. Or carry with ADV-U4FF1-3 to U4f2 |
| RR-F1-2 | Low | sim/src/seam.rs:78-80 | No test asserts an advance of more than one edge across the u8 wrap (d4 crosses 255 -> 0 with one edge and asserts grid ranges only: tests/world.rs:426, :475-481; T1/T2 stay below 3) | `const _: () = assert!(Command::edges_taken(254, 1) == 3 && Command::edges_taken(9, 9) == 0 && Command::edges_taken(1, 0) == 255);` beside the fn, or a 3-line unit test |
| RR-F1-3 | Low | seam.rs:78-80; world.rs:682 (`claimed`), room.rs:507-510 (`run_presses`), room.rs:851-861 (JoinAccepted carries no counters) | The wrap reads a counter that moves back as a near-255 advance. A restarted honest client rejoining its slot starts its counters at 0 (bots: lib.rs:289) against the previous session's reference, so ADV-U4FF1-3's unbounded first command is reachable by an honest client too, not only a hostile one (a pool of grenades on rejoin). No run meets it today: -20 has no rejoin; the clock suite reconnects but `never_fires` (lab/src/scenario.rs:1213-1214) | Not F1's: widen the item carried to U4f2 (condition 7) to name the honest restart, and decide there (e.g. JoinAccepted carries the slot's run counters, or the reference resets on a new session) |
| RR-F1-4 | Low | .gate/u4f/f1-b4-check1.log:11; sim/src/replay.rs:346-350 | The native-Wasm parity of the multi-edge path rests on reading (integer-only rule, the single-throw float path); no parity run is shown to contain a multi-edge command | Optional: count multi-edge commands in the SS/ES/TC recordings, or state "by construction" in the report |

Notes, no finding: reload_ms = 0 (Block 2, pre-existing); room.rs:1046 `self.reports.clone_from(self.sim.reports())`
reallocates both Vecs on any tick they are non-empty, since InputReports derives Clone (seam.rs:534) and
`#[derive(Clone)]` leaves `clone_from` at its default `*self = source.clone()` (core clone.rs; the Rust internals
thread "Make derive(Clone) implement clone_from as well",
https://internals.rust-lang.org/t/make-derive-clone-implement-clone-from-as-well/20844): pre-existing, once per tick
not per edge, a follow-up candidate only.

Nothing here touches a pass mark, the judge, marks.json, money or main.

## Verdict
**APPROVE WITH FINDINGS** (RR-F1-1 to -4, all Low: none blocks the merge). The fix is integer-only and panic-free,
the 1-edge path is today's code exactly, condition 1's order and one-shot rule hold, the loops are bounded at 255 with
no allocation per edge, and the evidence (plants by name, check.mjs twice, wasm parity 0 mismatching) holds as cited.
