# U4f2 review round 1: rust-reviewer seat (2026-11-20)

Patch: `.checkpoints/netcode-proof-test/wip/u4f2-build.patch`, sha256 e9a0ae3e3f48af18ea7a947fdf9aba849061fa32fbd91876a948c46f3046406b
(confirmed by sha256sum at the start of this review). Built tree read by absolute path:
`/home/user/Shamble-Frontlines/.claude/worktrees/agent-aa62d31b6fa5b9807` (below: WT). This agent has no Edit tool, so
the file is rewritten whole with Write at each block.

## Plan
1. Read the panel brief (done), u4f2-brief.md rev 1.2 (sections 1-9), u4f2-build.md, the security seat's file
   (SA-U4F2-1, SA-U4F2-2). (Done.)
2. Read the patch whole, file by file: npt-bots (the forgers), lab/src/forged.rs, npt-room (`RefusalRecord`,
   `CommandRefusal`), the lab wiring, the run scripts, the doc-link fix. (Done.)
3. Checks, each a block below: (a) the §4.11 bans, wasm32 vs native, SA-U4F2-1 and SA-U4F2-2 (block 1); (b) the
   `HOLD_STEPS` rename against check.mjs:760 and carry 39, and FIRE_REST (block 2); (c) integer arithmetic, panic and
   overflow paths in the forgers and the attribution (block 3); (d) allocation in hot paths, clippy, docs, the doc-link
   fix (block 4).
4. Findings RR-U4F2-n, verdict.

## Notes (one line per finding as it lands)
- No earlier rust-reviewer memory folder (.claude/agent-memory/rust-reviewer/ absent); baselines from the builder's
  record and the end-check logs.
- Built tree matches the record: forged.rs sha256 7919200c… (the record's clean sha); bots lib.rs f5d72e9a… (after the
  rename and fmt fix, so not the plants' 0c8c6d21…, as expected).
- SA-U4F2-1 verified (block 1). SA-U4F2-2 verified, and its 60 Hz note is a real rate dependence: RR-U4F2-1.
- Attribution keyed by the 16-bit sequence: wrong past one sequence lap (about 36 min at 30 Hz): RR-U4F2-2.
- `RefusalRecord`'s three public fields have no docs (the sim's struct documents each): RR-U4F2-5.

## Block 1: the bans, wasm32 vs native, SA-U4F2-1 and SA-U4F2-2 (done)
- **HashMap/HashSet, clock reads, unsafe, floats** (grep of the patch for `HashMap|HashSet|Instant|SystemTime|f32|f64|unsafe`):
  the only hits are three pre-existing context lines in lab/src/contract.rs (the band's display, `as f64`). New
  collections are `BTreeMap`/`BTreeSet` (forged.rs:16), `VecDeque` (bots `Forge::stamps`), `Vec`. Every crate the diff
  touches has the type and clock bans in its clippy.toml (bots, lab, room; the workspace denies `disallowed_types`,
  `disallowed_methods`, `iter_over_hash_type`: Cargo.toml:104-107), and clippy passed both targets at A3 and B
  (u4f2-end-a3.log:324-325, u4f2-end-b.log:324-325). Time reaches the forgers only as `now_ns` (kind 11's `Ping.sent_ns`
  and the twin's `ClientSettings` carry the lab's host time, `c.host(now)`, scenario.rs:1598, simulated): deterministic.
- **The room float-free:** the room's diff is the rename and `RefusalRecord { slot: u8, seq: u16, kind }`; room/src/lib.rs:12
  `#![deny(clippy::float_arithmetic)]`, and check.mjs's deps step reads "npt-room's sources hold no float"
  (u4f2-end-a3.log:328). Held.
- **wasm32 vs native:** the wasm32 crates are check.mjs:39's LEAF (npt-link, l1, sim, proto, room, client, log,
  hash-wasm). Of these the diff touches npt-room (a rename and a 4-byte Copy struct of integers, the same size and
  fields as the old tuple: no target-dependent width, no `usize` in it) and npt-client (doc lines and a native test).
  npt-bots and npt-lab are native only. npt-sim is untouched. The wasm step compared 27,000 ticks native-native and
  native-Wasm, 0 mismatching, on both counted runs (u4f2-end-a3.log:329, u4f2-end-b.log:329). Held.
- **Room hot path:** `incoming`/`refusals` stay the double buffer (`clear` then `mem::swap`, room.rs:1044-1045), capped at
  `REFUSALS_HELD` 4,096 with the rest counted (room.rs:37, :944-950); `tick_input()` and `refusals()` return references
  (room.rs:767, :646). No new allocation per tick or per command in the room.
- **SA-U4F2-1 verified.** bots/src/lib.rs:944 `let period = f.every * u64::from(f.burst_refused());` with the parser
  scenario.rs:2240-2247 (`parse::<u64>()`, `k >= 2`, no maximum). Cargo.toml:116-120 turns release overflow checks on
  for npt-sim and npt-proto only, so a release npt-lab wraps and a debug one (the tests' build) panics on
  `--forge-every` above `u64::MAX / 8` (A) or `/ 4` (B). At 2^61 under A the product wraps to 0 and
  `given.is_multiple_of(0)` is false for `given > 0` (no panic, kind 10 never forges); at 2^61 + 1 it wraps to 8. Not
  network input (an operator's flag), no false pass (every other kind then sends 0 and fails "1,000 of every kind").
  Low, as the security seat rates it. Its fix (bound the parser like its neighbours, scenario.rs:2223-2233's "at most an
  hour", and `checked_mul` in `forger`) is right; one point added: put the `checked_mul` in `forger()` (a `None`, the
  existing usage error at scenario.rs:1469), not at the step, so no step can panic whatever the parser allows.
- **SA-U4F2-2 verified from world.rs.** The weapon clock (world.rs:1415-1416, :1440, :1473-1476): `cost =
  fire_interval_us x TICK_HZ`; each command run `wait = max(wait - 1,000,000, -999,999)`; ready iff `wait <= 0`; a shot
  adds `cost`. SS at 30 Hz: cost 2,057,130 (68,571 us x 30).
  - At the judged 12 (and at 3), 30 Hz: the honest press at d - 1 fires (the last held shot is at d - 4 or earlier, so
    `wait <= 2,057,130 - 3,000,000 < 0`), and at d `wait >= -999,999 + 2,057,130 - 1,000,000 = 57,131 > 0`: the forged
    press always meets the cooldown. Sound, with a margin of 57,131 units (5.7 % of a tick) at the lower end.
  - At 2: fire is released on every command (some n + k, k in 1..=3, is always even), honest presses at odd commands,
    forged at even. Each honest shot leaves `w + 57,130` at the next honest press, so from the clamp floor -999,999 the
    18th honest press finds `wait = -999,999 + 18 x 57,130 = 28,341 > 0` (refused, an honest seq's Cooldown), and the
    forged press one command later finds -971,659: it fires, with no record, and counts unrejected. One item in about
    18, as SA-U4F2-2 says. Confirmed.
  - The construction's real condition is `2 x UNITS_PER_TICK <= cost <= FIRE_REST x UNITS_PER_TICK` (the floor from the
    one-tick carry clamp, the ceiling from the rest), not "its interval is more than a tick" as the constant's doc says
    (bots/src/lib.rs:150-151): at 1.5 ticks the forged press would fire. Both ends are counts of ticks.
