# U4f2 review round 1: rust-reviewer seat (2026-11-20)

Patch: `.checkpoints/netcode-proof-test/wip/u4f2-build.patch`, sha256 e9a0ae3e3f48af18ea7a947fdf9aba849061fa32fbd91876a948c46f3046406b
(confirmed by sha256sum at the start of this review). Built tree read by absolute path:
`/home/user/Shamble-Frontlines/.claude/worktrees/agent-aa62d31b6fa5b9807` (below: WT; file:line are WT's unless "patch").
This agent has no Edit tool, so the file is rewritten whole with Write at each block. No cargo, check.mjs or run
script was run; every number below is read from the code, the builder's record or the end-check logs.

**Verdict: APPROVE WITH FINDINGS.** One Medium (RR-U4F2-1, to fix in the fix pass), three Lows. The judged
configuration (30 Hz, SS, P0, `--forge-every 12`) is sound by the arithmetic below. No HashMap/HashSet, no clock read,
no float in the room, no new panic path reachable from network input, no new allocation in the room's hot path,
wasm32 and native unaffected. Nothing here touches a pass mark, the judge, marks.json, money or main.

## Plan
1. Read the panel brief, u4f2-brief.md rev 1.2, u4f2-build.md, the security seat's file (SA-U4F2-1, SA-U4F2-2). Done.
2. Read the patch whole, file by file, and the built files it changes. Done.
3. Blocks: (1) the bans, wasm32 vs native, SA-U4F2-1 and SA-U4F2-2; (2) `HOLD_STEPS` against check.mjs:760 and carry
   39, and FIRE_REST; (3) integer arithmetic, panic and overflow paths in the forgers and the attribution; (4)
   allocation, clippy, docs, the doc-link fix, Wasm size, dependencies. All done.
4. Findings and verdict. Done.

## Notes (one line per finding as it landed)
- No earlier rust-reviewer memory folder (.claude/agent-memory/rust-reviewer/ absent); baselines from the builder's
  record and the end-check logs.
- Built tree matches the record: forged.rs sha256 7919200c… (the record's clean sha); bots lib.rs f5d72e9a… (after the
  rename and fmt fix, so not the plants' 0c8c6d21…, as expected).
- SA-U4F2-1 verified (block 1). SA-U4F2-2 verified, and its 60 Hz note is a real rate dependence: RR-U4F2-1.
- Attribution keyed by the 16-bit sequence: wrong past one sequence lap: RR-U4F2-2.
- The forged suite's P0 precondition is unchecked by the parser: RR-U4F2-3.
- `RefusalRecord`'s three public fields have no docs (the sim's struct documents each): RR-U4F2-4.
- `HOLD_STEPS`: a step count, the rename sound (block 2). Doc-link fix verified (block 4).

## Block 1: the bans, wasm32 vs native, SA-U4F2-1 and SA-U4F2-2 (done)
- **HashMap/HashSet, clock reads, unsafe, floats** (grep of the patch for `HashMap|HashSet|Instant|SystemTime|f32|f64|unsafe`):
  the only hits are three pre-existing context lines in lab/src/contract.rs (the band's display, `as f64`). New
  collections are `BTreeMap`/`BTreeSet` (forged.rs:16), `VecDeque` (bots `Forge::stamps`), `Vec`. Every crate the diff
  touches has the type and clock bans in its clippy.toml (bots, lab, room; the workspace denies `disallowed_types`,
  `disallowed_methods`, `iter_over_hash_type`: Cargo.toml:104-107), and clippy passed both targets on both counted
  runs (u4f2-end-a3.log:324-325, u4f2-end-b.log:324-325). Time reaches the forgers only as `now_ns` (kind 11's
  `Ping.sent_ns`; the host time `c.host(now)`, scenario.rs:1598, on the lab's simulated clock): deterministic.
- **The room float-free:** the room's diff is the rename and `RefusalRecord { slot: u8, seq: u16, kind }`;
  room/src/lib.rs:12 `#![deny(clippy::float_arithmetic)]`, and check.mjs's deps step reads "npt-room's sources hold no
  float" (u4f2-end-a3.log:328). Held.
- **wasm32 vs native:** the wasm32 crates are check.mjs:39's LEAF (npt-link, l1, sim, proto, room, client, log,
  hash-wasm). Of these the diff touches npt-room (a rename and a 4-byte Copy struct of integers, the same size and
  fields as the old tuple, no `usize`) and npt-client (doc lines and a native test). npt-bots and npt-lab are native
  only; npt-sim is untouched. The wasm step compared 27,000 ticks native-native and native-Wasm, 0 mismatching, on both
  counted runs (u4f2-end-a3.log:329, u4f2-end-b.log:329). Held.
- **Room hot path:** `incoming`/`refusals` stay the double buffer (`clear` then `mem::swap`, room.rs:1044-1045), capped at
  `REFUSALS_HELD` 4,096 with the rest counted (room.rs:37, :944-950); `tick_input()` and `refusals()` return references
  (room.rs:767, :646). No new allocation per tick or per command in the room.
- **SA-U4F2-1 verified (Low, agreed; no new id).** bots/src/lib.rs:944 `let period = f.every *
  u64::from(f.burst_refused());` with the parser scenario.rs:2240-2247 (`parse::<u64>()`, `k >= 2`, no maximum).
  Cargo.toml:116-120 turns release overflow checks on for npt-sim and npt-proto only, so a release npt-lab wraps and a
  debug one (the tests' build) panics for `--forge-every` above `u64::MAX / 8` (A) or `/ 4` (B). At 2^61 under A the
  product wraps to 0 and `given.is_multiple_of(0)` is false for `given > 0` (no panic; kind 10 never forges); at 2^61 + 1
  it wraps to 8. An operator's flag, not network input; no false pass (the other kinds then send nothing and fail
  "1,000 of every kind"). Its fix is right; one refinement: put the `checked_mul` in `forger()` (a `None`, so the
  existing usage error at scenario.rs:1469 names it), not at the step, so no step can panic whatever the parser allows.
- **SA-U4F2-2 verified from world.rs** (subsumed by RR-U4F2-1). The weapon clock (world.rs:1415-1416, :1440,
  :1473-1476): `cost = fire_interval_us x TICK_HZ`; each command run `wait = max(wait - 1,000,000, -999,999)`; ready iff
  `wait <= 0`; a shot adds `cost`. SS at 30 Hz: 68,571 us x 30 = 2,057,130.
  - At the judged 12 (and at 3), 30 Hz: the last held shot is at d - 4 or earlier, so at d - 1 `wait <= 2,057,130 -
    3,000,000 < 0` and the honest press fires; at d `wait >= -999,999 + 2,057,130 - 1,000,000 = 57,131 > 0`, so the
    forged press always meets the cooldown. Sound, with 57,131 units (5.7 % of a tick) at the lower end.
  - At 2: fire is released on every command (some n + k, k in 1..=3, is always even); honest presses at odd commands,
    forged at even. Each honest shot leaves `w + 57,130` at the next honest press, so from the floor -999,999 the 18th
    honest press finds `wait = -999,999 + 18 x 57,130 = 28,341 > 0` (refused: an honest sequence's Cooldown) and the
    forged press one command later finds -971,659: it fires, with no record, and counts unrejected. One fire item in
    about 18, as SA-U4F2-2 says. Confirmed.

## Block 2: `HOLD_STEPS` against check.mjs:760 and carry 39; FIRE_REST (done)
- **The rule:** check.mjs:755-765 `TICK_NAMED = /_TICKS$|CATCHUP/` refuses such a const unless it names TICK_HZ, calls
  `ticks_for_ms` or is a path to a tick name; `NOT_TICK_COUNTS` exempts `CMD_CATCHUP` alone ("the redundancy cap", a
  count of commands per tick, not derived from TICK_HZ). Its own text (check.mjs:791-792): "A tick count under another
  name ... is review's and the tests'." Carry 39 (u4s-carries.md:169-179; u4f-brief.md:317, :815): a rate-derived value
  outside npt-sim takes the rate as an argument and is pinned at 30 and 60 Hz, or is recorded (name, definition, values
  at 30 and 60 Hz, site) for the main session; no local form.
- **`HOLD_STEPS` (bots/src/lib.rs:156, `REDUNDANCY_CAP as u32 - 2` = 6; proto limits.rs:17 `REDUNDANCY_CAP = 8`) is a step
  count, not a tick-derived value.** Read from `catchup_step` (lib.rs:932-979): an episode is 8 steps (A: steps 1-6 give
  0, step 7 gives 8 held, step 8 gives 8 more and sends both; B: 8, then 4, then steps 3-8 give 0; `episode` returns to
  0 at `HOLD_STEPS + 2`), and `restamp` reuses the refused 8 (A) or 4 (B) stamps (`next_stamp = resume`, lib.rs:216-218
  in the patch), so the stamps advance 8 over 8 steps. The endpoint steps once a tick period at any rate
  (client/src/lib.rs:752-753: `steps x 1e9 / tick_hz`), so the lead is kept at 30 and at 60 Hz with the same 6. The
  value comes from the per-tick command cap (CMD_CATCHUP's 8, itself exempt as rate-free), not from a duration: at 60 Hz
  the hold is 100 ms instead of 200 ms and nothing in the construction reads milliseconds. So it is not a tick count in
  the rule's sense; the old name matched only through "CATCHUP" (the kind's name). The rename is the in-lane fix (the
  other route, a `NOT_TICK_COUNTS` entry, edits check.mjs, outside the lane). The builder's carry-39 form (u4f2-build.md:
  226-231: name, definition, 6 and 6, site :156) is right. **No finding.**
  - One assumption, stated for the netcode seat, not a finding: the burst's tail must lie inside the lead bound (A:
    lead + 7 ticks, B: lead + 9, against 12 at 30 Hz, 24 at 60 Hz), so the rate change only widens it. The lead side
    is RR-U4F2-3.
- **FIRE_REST (bots/src/lib.rs:153, `3`) is a rate-derived count, documented as "Counts of items and commands, no time"
  (lib.rs:151). RR-U4F2-1.** From block 1's arithmetic, kind 7's fire items are sound only while
  `2 x 1,000,000 <= cost <= min(FIRE_REST, every) x 1,000,000`, i.e. the weapon's interval in ticks lies in
  `[2, min(FIRE_REST, every)]`. The floor comes from the one-tick carry clamp (world.rs:1416), not "its interval is more
  than a tick" as the doc says (lib.rs:150-151: at 1.5 ticks the forged press fires). At 60 Hz SS's interval is
  68,571 x 60 = 4,114,260 units, 4.11 ticks: with the last held shot at d - 4 and `w` in [-999,999, 0], at d - 1 `wait =
  w + 1,114,260 > 0` (the honest press refused) and at d `wait = w + 114,260`, ready whenever `w <= -114,260`; the held
  trigger then fires at d, the forged edge leaves no record, and the item counts unrejected. With the last shot at d - 5
  the same happens whenever the honest press at d - 1 is not ready. So at 60 Hz a large share of kind 7's fire items
  read unrejected: a false FAIL of "every forged command rejected and counted", and the plan's 60 Hz run "changes no
  other crate" (u4-build-plan.md:119, RQ6; A5 at :88: "every tick-counted value a const fn of TICK_HZ, tested at 30
  and 60 Hz"). The same holds at `--forge-every 2` at 30 Hz (SA-U4F2-2) and for a scene whose weapon is slower than 3
  ticks (ES's 666,667 us, scene.rs:73, is 20 ticks at 30 Hz), if the forged suite is ever run on one. check.mjs's name
  rule cannot see it (its stated limit). The twin hash is not affected (the twin's held trigger fires the same shot),
  so only the attribution errs; never a false pass.

## Block 3: integer arithmetic, panic and overflow paths (done)
- **The forgers (bots, non-test code):** `forge_input`'s `i.commands[0] = …` (patch :145) follows `fresh`, taken from
  `i.commands.first()`, so the index exists. Kind 9's `out[PAD_BYTE] |= 0x80` (lib.rs:378, `PAD_BYTE` 27): `way` is set
  only for an `Input` with a fresh newest command, which encodes to at least 5 + 23 = 28 bytes; f15's kind-9 case
  catches a layout drift (a pad bit that is not one decodes, and the case fails). `forge_step`'s `(d / every) %
  RELOAD_ITEMS` and `is_multiple_of(every)`: `every >= 2` (forger, lib.rs:552). `self.presses[b]`: `b` is
  `PRESS_FIRE` or `PRESS_RELOAD`. Counters `wrapping_add` (u8 press counters, u16 seq, u32 stamps), `saturating_add` for
  kind 3's aim; kind 5's `covered + 1` is at most 101 (pre-existing, lib.rs:414-416). `Ping.id = self.sent as u8` wraps on
  purpose. `u16::try_from(out.len()).ok()?` for kind 11's frame (it matches the codec's own 2-byte control framing,
  proto msg.rs:402-421). The only unchecked product is SA-U4F2-1's. No `unwrap`/`expect`/`panic!` outside tests.
- **The attribution (lab/src/forged.rs, non-test):** no indexing, no unwrap; `ran` is a u32 per sequence (a sequence
  runs once or twice, never near 2^32); `items.len() as u64`, `count() as u64`; kinds 8, 9, 11 by `saturating_sub`
  (SA-U4F2-3's clip, the security seat's). Its one correctness limit is the 16-bit key: RR-U4F2-2.
- **The lab wiring (scenario.rs):** `cutoff = measured_end.saturating_sub(lead ticks)` (scenario.rs:1353); a run shorter
  than the bound gives cutoff 0, no items, no panic. `FORGERS.min(n)`, `slot + 1 <= 11` in u8. Kinds 8, 9, 11 read once at
  `k == cutoff`, after `k += 1`. The only `expect` added outside a test module is `#[cfg(test)]`
  (patch :1547, behind `get(..32)`).
- **`RefusalRecord` / `CommandRefusal`:** a rename and a struct; `receive_input`'s logic is unchanged (patch :2351-2394);
  no stale `npt_room::Refusal` anywhere in crates/ (grep), and the provenance builds of the whole workspace exit 0 on
  A3 and B (npt-server included).

## Block 4: allocation, clippy, docs, the doc-link fix, Wasm size, dependencies (done)
- **Allocation:** the room: none new (block 1). The lab's attribution per tick: 11 forgers x (the tick's records + its
  commands run + the sim's records), BTreeMap lookups, inserts only for forged items (bounded by the 16-bit key).
  The forgers' `outgoing` per step: one `decode` and one `Input` clone per datagram, an `encode` only when the datagram
  changes (the cheats' hook already did the same, lib.rs:752); `take_forged`/`take_copies` allocate only when a
  forged item was pushed (`mem::take` leaves an empty Vec; an honest client's `unwrap_or_default` allocates nothing).
  Harness code, about 11 x 30 small allocations a second: no finding.
- **Clippy and fmt:** both clippy targets PASS twice, fmt PASS twice (u4f2-end-a3.log:323-325, u4f2-end-b.log:323-325).
  Reading the diff found nothing default clippy would flag; no new `#[allow]`.
- **Docs on public items:** the new pub fns of npt-bots (`twin`, `catchup_under_a`, `take_copies`,
  `forge_unencodable`, `reliable`) and `CommandRefusal` are documented; `RefusalRecord`'s fields are not (RR-U4F2-4).
  The workspace has no `missing_docs` lint (Cargo.toml:77-107), so no step catches it.
- **The doc-link fix verified.** End check A failed doc on "public documentation for `forger` links to private item
  `RELOAD_ITEMS`" and `CATCHUP_HOLD` (u4f2-end-a.log:227, :245, :255). The fix names both as code text, no link
  (lib.rs:543 "every 40th item, the crate's `RELOAD_ITEMS`"; :548 "6 steps without a command (the crate's
  `HOLD_STEPS`)"); private items keep their intra-doc links to private constants (lib.rs:148-149, :887, :929), which
  rustdoc allows. Doc PASS on A3 and B, host and wasm32, measured and bites, warnings as errors
  (u4f2-end-a3.log:326, u4f2-end-b.log:326). The text matches the code (reload item at `i % 40 == 2`; hold 8 - 2).
- **Wasm size:** no artifact's crates change: npt-hash-wasm ships npt-sim only (deps step), and npt-room has no wasm
  consumer (only room, server and lab manifests name it). Expected delta 0 bytes; not measured (no build in this seat).
- **Dependencies:** no manifest or Cargo.lock in the diff; deps PASS on A3 and B.

## Findings (CLAUDE.md's one scale)
- **RR-U4F2-1 (Medium): kind 7's `FIRE_REST = 3` is a rate- and weapon-derived count written as a literal and
  documented as time-free; at 60 Hz (and at `--forge-every 2`, SA-U4F2-2) the construction leaves fire items
  unrecorded.** bots/src/lib.rs:147-153 (the doc and the constant), :892 (its use). What goes wrong: block 2's
  arithmetic; a false FAIL of -21's forged check at 60 Hz, at an interval of 2, or on a weapon slower than 3 ticks,
  against carry 39 and plan A5/RQ6. The judged 30 Hz, SS, `--forge-every 12` runs are sound. Fix (in the lane, carry
  39's rate-argument pattern): a `fn fire_rest_at(hz: u32, interval_us: u32) -> Option<u64>` returning
  `(interval_us x hz).div_ceil(1_000_000)` when `interval_us x hz >= 2_000_000` (else `None`), computed from
  `scene.weapon().interval_us` with `npt_sim::TICK_HZ` where the behaviour is built (one field on `Behaviour`);
  `forger(7, every, _)` returns `None` when the rest is `None` or `every` is below it (a usage error, so 2 is refused
  for kind 7 while the parser's floor of 2, C-U4F2-1's ruled text, stays); a test pins SS at 3 (30 Hz) and 5 (60 Hz)
  and `None` below 2 ticks; the doc's "more than a tick" corrected. About 15-25 lines. A World-level case tying the
  forger's commands to the sim's records at the built rate (the bots' sim-only harness, one slot) would close the gap
  the security seat notes (no unit case ties FIRE_REST to the weapon): about 30 lines, the test seat's call. The route
  (this pattern, or carry 39's record for the main session) is the advisor's. Run for the fix pass, before the fix:
  `npt-lab scenario` short forged run, 16 clients, P0, 2 s + 4 s, option A, `--forge-every 2`: expected kind 7
  `unrejected_count` above 0 (about one fire item in 18); after the fix: a usage error naming `--forge-every 2`.
- **RR-U4F2-2 (Low): the attribution keys every item by its 16-bit sequence, so a forged run longer than one sequence
  lap merges items.** lab/src/forged.rs:28-34 (`BTreeMap<u16, …>`, `BTreeSet<u16>`), :66-69 (`items.insert`), :92-99,
  :113-117. A forger gives one command a step (kind 10 under A: 96 sequences in 88 steps), so the lap is about
  2,180 s at 30 Hz (about 2,000 s for kind 10), about 1,000 s at 60 Hz; the parser allows runs up to 2^32 ticks
  (scenario.rs:2165-2173). Past a lap at the judged 12, an honest command's sequence equal to an earlier item counts
  as that item's run (false FAIL for kinds 1-6, 10). At an interval dividing 65,536 (2, 4, 8, 16, …) lap 2's items
  reuse lap 1's sequences: kind 7 inherits lap 1's refusals, and kinds 8, 9, 11's `sent` stops growing while their
  counters grow, clipped to 0: a false pass. The judged 610 s runs are inside one lap at 30 and at 60 Hz (about 41,900
  sequences at most), so nothing judged is affected. Fix: in `Attribution::forged`, count an `insert` that returns
  `Some` into a `reused` field added to `unrejected_count` (a lap fails loudly, never passes), and say the one-lap
  limit in the module text (its "bounded by its length" is true only within a lap); one f33-style case. About 5-10
  lines.
- **RR-U4F2-3 (Low): the forged suite's P0 precondition is unchecked.** scenario.rs:2240-2251 takes `--suite forged`
  on any profile and with `--lead` 1 to 8 (:2261-2266); the code's own text says P0 (scenario.rs:1564 "(P0)",
  lib.rs:549-550 "on a lossless profile (P0)"), and the plan runs it at P0 only (c-u4f-runs.mjs PARTS_21; c-u4f-r2.mjs
  FORGED). Off it: kind 10's tail is refused under kind 1, not 10, once lead + 9 (B) or lead + 7 (A) passes the
  12-tick bound at 30 Hz (lead 4 under B, 6 under A; `forged_kind` then counts them unrejected); under jitter the two
  burst `Input`s can arrive in different ticks, so the cap refuses nothing; and kinds 8, 9, 11's count at the cutoff
  can miss an item in flight. All false FAILs, never a pass. Fix: the parser refuses `--suite forged` with a profile
  other than P0 or with `--lead`, two cases in c_u4f2_1's test. About 6-10 lines.
- **RR-U4F2-4 (Low): `RefusalRecord`'s public fields are undocumented.** room/src/room.rs:42-46 (`pub slot`, `pub seq`,
  `pub kind`); the struct it mirrors documents each (sim/src/seam.rs:509-514). Fix: three doc lines. About 3 lines.
- **Verified, not re-filed:** SA-U4F2-1 (Low, agreed, with block 1's refinement); SA-U4F2-2 (confirmed; folded into
  RR-U4F2-1, whose fix closes it and the 60 Hz note).

## What is Mustafa's
- Nothing in these findings touches a pass mark, the judge, marks.json, money or main. RR-U4F2-1's fix changes no
  rule of the attribution (kind 7 still needs a sim Cooldown or Reload record), only the forger's construction and its
  usage error. F-B (dead or just-respawned forger) is not closed by any of these and stays the netcode seat's; any F-B
  fix that changes what counts as rejected would be his (TEST-FIRST), as the security seat says.
