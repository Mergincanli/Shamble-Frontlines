# Advisor: U4 build plan review (u4-build-plan.md)

Returned by the advisor (read-only) from workflow wf_899b9cf1-c90 on 2026-11-09; recorded by the main session from the
agent's returned text, as the verdict's condition 12 asks.

## Verdict: U4 build plan rev 2 (2026-11-09)

**APPROVE WITH CONDITIONS.** The split is sound and in dependency order. The plan meets the U4 row (plan.md:1223) and
every carry that names U4. It adds no engine feature without a traced need.

Three things must change in the text before the builds:
- **-21's twins would fail on a correct build.** Refused input leaves a trace in the hashed state.
- **-30's pre-check would fail on a correct build.** It replays a P2 recording at P0, so the room replaces the view
  times on both legs, each from a different RTT.
- **-30's pre-check is bigger than it needs to be.** It does not need a new `PRE-` judge concept.

All three fixes are text and change no structure. All 26 questions are mine to rule; none goes to Mustafa now. Six are
readings of ADR rows and reach him through the readings PR before R5.

**What I reviewed:**
- The plan as committed at aebe8ea: /home/user/Shamble-Frontlines/.checkpoints/netcode-proof-test/u4-build-plan.md,
  901 lines, sha256 27e0315857e51bd5667605d032b57a0ce75701d0ba6a08bd2b9319315e3b4dc2. This is rev 2, and the disk copy
  is identical.
- The code is as at f985226: `git diff --name-only f985226 HEAD` touches only .checkpoints/.
- I edited, built and ran nothing. marks.json was read by a read-only Python parse.

**What I checked (file:line; spike paths under spikes/netcode-proof/):**
- **The binding texts:**
  - plan.md:1223 (the U4 row); :495-511, :532-552, :562, :577, :591, :696, :720.
  - My earlier rulings: advisor-u2.md:72-103 (C1-C4 and the U6 note), advisor-u3q.md:160-173 (C1'),
    advisor-u3c-plan.md:50 (C18: met by §4.5's whole-tick unit), advisor-ws-docs-r2.md:375-377 (-18's split).
  - The checkpoint: :404-413 (the shared-target rule; §7.1 matches it), :325-328 (the scene selects), :298-305 (C1's
    text is stale; C3).
- **marks.json, parsed:**
  - The 21 marks' parts, checks, validity rules and bites: 46 bites, plus PM-0004-67's 3, makes 49 (T21 holds).
  - No U4 mark carries `pre:PM-0001-2` today.
  - PM-0001-2's "SS runs" checks include `everyone_in_view_min_share` ≥ 0.99 and 300 projectiles on every tick (min).
  - -30's C part selects suite, P0 and role C, and no layer.
  - -42's P2 and P4 parts have no checks.
  - -41's parts pool both options.
  - -21's twins: forged and clamp only.
- **The judge:**
  - eval.rs:850-887 (`pre` applies the matched part's checks) and :921-930 (part matching ignores only `role`).
  - marks.rs:315 and :602 (`PM-000` ids only).
  - figures.rs:21-46 (a slot-count change is a gap) and :170-224.
- **README and SERIES:**
  - README:32 (the layer values), :147 (`hs`), :171 (the roster rule), :193-197 (`pre:PM-0001-2` is on the parts of
    PM-0001-4 to -27 that select on the scene).
  - SERIES.md:52-53, :94, :266, :294, :314, :342.
- **The ADRs:**
  - ADR 0001:205-209 (the two own-state forms), :236-242, :297-303 (hits are resent gameplay events), :1466-1467,
    :1589 (PM-0001-2: "every run of PM-0001-4 to PM-0001-27"), :1636.
  - ADR 0002:186-191 (protocol may use sim types; room depends on sim), :264-265 (the unsafe exception rule), :500-505
    ("the exact file G6 ships").
  - ADR 0004:166-178 and :544-556 ("The event list is part of the state and its hash").
- **The code:**
  - check.mjs:24, :94-102, :135-137, :1110, :1130-1158; Cargo.toml:103-107; the sim, proto and room clippy.toml files.
  - link/src/lib.rs:203-205 (Loopback's `rtt_ns` is always Some(0)).
  - l1/src/lib.rs:9-12, :21-41, :114-138, :856-868.
  - lab/src/probe.rs:200-201 (datagrams only, so L-c8's hash comparison can hold).
  - Every stub crate and every manifest.
- **The catalogue:** :240-253.
- **Numbers I re-derived:**
  - The shot band: 0.486 shots per command-tick; 691.5, 691.1 and 685.4. Even if each B1 burst costs 1.1 s of shots
    under option A, the mean is about 687.
  - The initial PTO: 1,024 ms.
  - The recordings: 41 B a command, 1,973 B a tick, 17.8 MB per 9,000 ticks.
  - `Input`: 161 B at the cap.
  - The constants: `LOOP_CATCHUP` 7 and 15; `ticks_for_ms(2,000)` 60 and 120; positions ±327.68 m and 0 to 40.95 m.
  - The run count: 196 measured runs.

**What I did not check:**
- RFC 9002's text (no network). The plan's numbers match my own knowledge and two agents' reads.
- wasm32's default target features. Condition 7 makes the step refuse simd128 rather than rely on the default.
- Whether clippy can ban primitive types. The grep stands in for it.
- The stand-in scene's tuning. D-6 and the contract test measure it.

**Conditions:**
1. **Refused input must leave no trace in the state hash.** As written, this fails -21 by design.
   - The problem: ADR 0004:544-556 hashes the tick's event list. §4.2 returns `SimEvent::Clamped` from `step`, and the
     sim "refuses and counts" Rewinds, non-finite components, and edges blocked by cooldown, ammo or reload. A forged
     run and its valid-parts twin then differ on every refusing tick, and so do the clamp run and its at-reach twin.
   - Add to §4.2 and §4.10: the clamp reports and every refusal counter stay outside `state_hash` and outside the
     hashed event list. They are read through their own accessor and logged by the room.
   - The twin's command for a kind the sim checks is the forged command with only the refused part removed.
   - Any per-soldier input state the sim keeps holds what the server accepted, not what the client claimed.
   - U4d proves it with one unit test per kind the sim checks (cooldown, ammunition and reload edges; the clamp; a
     Rewind outside the ring): the forged command and its twin give equal state hashes on that tick and after.
   - `event.clamp_kind.counted_count` counts refusals only, never clamp reports.
2. **-30's pre-check becomes a recorded test, not a judged id (Q10).**
   - Keep: both legs (Loopback and L1 P0), the replay clients, both bites and the suite `precheck-30`.
   - Drop from U4j: `PRE-` ids, marks/prechecks.json, the "kind": "precheck" rule and their tests.
   - npt-lab's comparer (built in U4d2) compares the two legs' per-tick server hashes and names the first tick that
     differs. `tick.v.late_count` must sum to 0 on both legs.
   - The bites: wallclock must produce a mismatch; late-command must make the pair void, never a pass.
   - The headers list no mark, so the judge never reads these runs.
   - Record results/u4r-precheck-30/<commit>/runs.txt and the comparer's output. I read them in U4r's review, as I did
     for U3d's self-test (advisor-u3d.md, U3D-JUDGE: nothing goes into marks.json).
   - If the Loopback leg writes an npt-log header, README §1 gains the layer value `loopback` in one line. No judge
     change.
   - -30 stays NOT-RUN until U6.
3. **-30's pre-check replays a recording made at P0, and 0 replacements is its setup condition.**
   - The problem: §4.15's runner recordings are SS P2, ES P3 and TC B1. Replayed at P0, their view times sit about one
     RTT older than P0's estimate, so the room replaces them on both legs. Each leg replaces them from its own RTT:
     Loopback is always Some(0), while L1 starts at 333 ms after U4c.
   - Record a P0 source with `scenario --record` (SS P0), as a fourth recording or by moving one of the three to P0.
   - Setup condition: 0 view-time replacements and 0 clipped shots on both legs. A leg with any is void and reported,
     never a pass or a fail.
   - State how the L1 leg's RTT before its first sample cannot lead the room to a decision the Loopback leg makes
     differently.
   - Record both legs' input-buffer depth (marks.json -30's notes).
4. **-22's exception goes where the rule lives.** §4.12's -22 row is right: the drop leaves 47 clients, and the slot
   count of `everyone_in_view` changes (figures.rs:26-36). But README §4 (:193-197) and ADR 0001:1589 both put the
   precondition on -22.
   - U4j writes the exception and its reason into README §4's sentence.
   - -22's runs still log PM-0001-2's series (shots, projectiles, vehicles, teams, portals), and U4f's report states
     the load each run carried.
   - The exception is a reading of ADR 0001:1589, so it joins the readings PR (condition 11).
5. **The precondition must pass, not only resolve.**
   - U4r's contract test shows every PM-0001-2 check passing on the short runs of each precondition suite: -17's four
     profiles, -18's B1, -20's P3, and each of -21's eight suites with its open-loop scripts.
   - A suite that fails comes to me before its marks count, as §4.13 already says for the shot band.
   - D-6 asserts PM-0001-2's thresholds exactly, never "about": at least 300 projectiles and teams of at least 16 on
     every tick; shots and portal changes inside their bands.
6. **-18's bite is shown end to end.**
   - §4.14's unit proof is the server half only. U4e's proof runs from the encoder to the client: under no-fallback the
     client refuses every delta after the burst, so no fresh delta decodes within 500 ms; the clean build decodes one.
   - If the bite's baseline turns out to be one the client still holds, the bite is not a defect on L1, and -18's L1
     part comes to me before it counts.
7. **U4d2's checks refuse, and the new exception follows ADR 0002:264-265.**
   - (a) The `rustc --print cfg` read-back fails the step on fma (x86-64), simd128 or relaxed-simd (wasm32), with a
     self-test bite. It never only reports.
   - (b) hash-wasm gets a written invariant (no exported name collides with another symbol; integer signatures only)
     and a test (the Node loader lists the module's exports and refuses any not on the list).
   - (c) Before the code merges, plan.md §4.2's "two named exceptions" becomes three, and check.mjs's
     `UNSAFE_EXCEPTIONS` gains crates/hash-wasm/.
8. **New carry C-U7-1.** U4d2 does not cover wasm-opt (ADR 0002:500-505 uses "the exact file G6 ships"). U7 reruns
   U4d2's check on hash-wasm passed through the same wasm-opt step and flags as the shipped npt-client-wasm, or states
   why not. This goes into plan.md's U7 row and the checkpoint.
9. **`replay` gets a test at U4, or moves.** U4r's brief says what `replay` is (the x86-64 re-decode, R-8) and names
   its U4 test: one L1 run's logged client bytes, re-decoded, give the clients' logged hashes, with a bite. Otherwise
   `replay` and `cross_arch_mismatch_count` move to U6 together (§4.11's own T17 rule).
10. **Lanes.**
    - (a) U4s also adds every wave-1 crate's `bites` feature and npt-lab's forwarding to them.
    - (b) In wave 2, only U4d2 edits crates/lab, its Cargo.toml included. U4e's tests live in its own crates (Loopback
      as a dev dependency).
    - (c) Reviewers count toward the 3-agent limit and run at xhigh (netcode work).
    - (d) The main session builds U4b and U4j only on committed state, never with uncommitted edits while it merges a
      lane.
11. **The readings list (§10's preamble and §11).**
    - The readings PR carries Q11, Q16's clamp, Q17, Q22's -22 exception, Q24, and Q26 (a reading of ADR 0001:1589's
      stand-in sentence).
    - Q10 comes off the list.
12. **Pin what I approved.**
    - The approved plan is aebe8ea's rev 2, at the sha256 above. Only these edits are allowed without coming back to
      me: conditions 1-11, the nits below, and risk 11's moves of plan.md text.
    - The five briefs (Q21) come to me before their builds.
    - The main session records this verdict in advisor-u4.md.
13. **Nits, folded at the author's discretion:**
    - (a) -3's re-run is U2's whole set: 15 measured runs, 15 twins, 4 bites and the clock-seed twin (checkpoint
      :269-271). That is not "about 20", so the total is about 285 runs.
    - (b) The recording header carries HitHistory's capacity.
    - (c) README states how `hs` writes a client with no hash at a tick (null), in the six fixtures.
    - (d) Catalogue item 2 says "on both sides": npt-client drops and counts a wrong-channel message (plan E7), tested
      in U4e.
    - (e) Catalogue §6 item 3 notes that its allocation half moves to U5.
    - (f) Q20: U4j fixes the format of the bound's machine-readable line and of the practice record in README and
      SERIES. The fixes.rs unit writes exactly that format and reads it back with the judge's parser.
    - (g) A stray spikes/netcode-proof/target exists: 204 MB, dated 2026-11-08 15:16, gitignored, with a CACHEDIR.TAG.
      It shows one cargo call bypassed both the spike's config and CARGO_TARGET_DIR. §7.1 item 1's first-build check
      also confirms no such folder appears in a copy. This one can go by `cargo clean --target-dir`, an approved method.

**What stands as planned:**
- **The seam:**
  - `SimCore` and the plain-data types live in npt-sim, and the room is generic over them. ADR 0002:186-191 allows each
    dependency edge.
  - The integer Rewind and the room-owned ring keep every float in the sim and keep recordings small.
- **The tick constants:** `TICK_HZ` and every tick-counted value are const fns in npt-sim, tested at 30 and 60 Hz and
  guarded by a grep. This meets RQ6, A4 and A5.
- **C1' and L-c8:** C1' gets RFC 9002's starting state. L-c8 is a real regression test.
- **-17 and -18:** §4.6's complete-tick rule gives each -17 bite a place to fail. §4.12's rows for -17, -18, -20 and
  -21 hold up, and so does the shot-band arithmetic.
- **The bites:**
  - §4.14 gives each bite its site and its run setting.
  - Every bite is chosen per instance.
- **The order:** U4s alone; U4j before U4c's re-judge; each build merges before its judged runs; U4f, U4g and U4h one at
  a time.
- **The deferrals:**
  - AddrTable and the handshake cap go to U6 with -25.
  - `loss` goes to U6 with rate adaptation.
  - `BlockWriter` goes to U7.
- **The rules file:** widening .claude/rules/netcode.md's paths is a kit edit, and only Mustafa can make it. Naming the
  file in every brief covers U4 without it.

## Rulings on the plan's questions
- **Q1 U4-SEAM:** (a).
- **Q2 U4-HIT:** (a).
- **Q3 U4-WASM:** (B), with conditions 7 and 8. The third unsafe exception is mine under ADR 0002:264-265, as
  npt-tcpinfo's was (advisor-u3c-plan.md:56).
- **Q4 U4-REPLAY:** (a).
- **Q5 U4-SERVER:** (a); `BlockWriter` goes to U7.
- **Q6 U4-LOSS:** (b). It amends my C3 by C3's own principle: `loss` joins with its first consumer, rate adaptation at
  U6. plan.md:562, §4.4's per-tick list and the checkpoint's C3 line move with it.
- **Q7 U4-CTL:** (a), since catalogue item 6 asks for it at U4. plan §4.7's npt-proto list gains `RoomControl` before
  the code merges.
- **Q8 U4-ALLOC:** (a), with nit (e).
- **Q9 U4-BANS:** (a).
- **Q10 U4-30:** none of the options offered. It becomes a recorded npt-lab test (conditions 2 and 3).
- **Q11 U4-MIRROR:** (a); a reading for the readings PR.
- **Q12 U4-BASELINE:** (a).
- **Q13 U4-WINDOW:** (a). plan §4.4's per-tick inputs gain the bytes the server side sent.
- **Q14 U4-EVENTS:** (a). ADR 0001:297-303 resends hits as gameplay events. (b) would be a design change and is not
  taken.
- **Q15 U4-ACKS:** (a).
- **Q16 U4-TWIN:** (a), plus condition 1. The clamp twin's "points at the reach" is a reading.
- **Q17 U4-TARGETS:** (a); a reading.
- **Q18 U4-49:** (a).
- **Q19 U4-UNITS:** (a). E7's text changes in plan.md before the client code merges.
- **Q20 U4-18:** (a), with nit (f).
- **Q21 U4-BRIEFS:** (a). I check each brief before its build.
- **Q22 U4-SCENES:** (a), with conditions 4 and 5.
- **Q23 U4-OWNSNAP:** (a). It picks one of the two forms ADR 0001:207-208 offers, the one -41's second bite needs.
  Mine.
- **Q24 U4-41RUN:** (a); a reading.
- **Q25 U4-CLAMP:** (a). plan §4.4's room column marks the clamp "(in the sim)".
- **Q26 U4-L1BURN:** (a); a reading (condition 11).

## Decisions needed
None. Every question is technical and sits inside the approved spike. The six ADR readings go to Mustafa in the
readings PR before R5, the plan's standing route. None of them lowers a threshold. -22's exception drops a
precondition its own design cannot meet; its own checks stay, and its runs log the load.

## For Mustafa
Nothing now. Later, your go on the readings PR before R5, which will carry these readings.

## Ruling: U4s (2026-11-09)
(Returned by the advisor, read-only, about 17:45 UTC; recorded by the main session. Formatting compacted; the words
are the advisor's.)

**APPROVE WITH CONDITIONS. U4s is accepted. Wave 1 (U4a, U4b, U4c, U4d, U4j) may start once conditions 1 and 2 are in
the text.** It covers the spike code at f08055134b03a6ea6e2b25d8f6cc8a8457f3eadd (HEAD e816a5f adds only the checkpoint
and the report). It meets §5 U4s; §4.1-§4.4; §4.11's float grep; §4.14's two U4s sites; condition 10 (a) and (b). I
reproduced check.mjs (10 of 10 steps) and the gate myself. The conditions are text only: no code changes and no re-run.
One real gap: the fix pass wrote four plan-text carries that had to land "before the code merges"
(u4s-carries.md:144-157, plus item 5 at :55-57). None landed: the plan is still the 12:05 text (sha256 f7961cfe…, the
conditions-applied version), so §5 U4a's a4 (u4-build-plan.md:524-525) still sends -57's bites "through the grid
config", and the fix pass removed that route. Condition 12 makes these edits mine to approve; I approve them below.

**What I reviewed:** u4s-report.md (77 lines, sha256 97054a02…); u4s-carries.md (33 entries, sha256 1500ca04…); the
five review notes; advisor-u4.md; u4-build-plan.md at f7961cfe… (unchanged since 12:05, equal to the version the
checkpoint records as checked complete, u4-conditions-check.md); `git show f080551` (19 files, +1,979 -13) and the code
at HEAD. I edited nothing.

**What I ran** (main folder, its own cache, CARGO_TARGET_DIR unset; `git status` clean before and after): check.mjs
(ended 17:32:14 UTC): all 10 steps pass (fmt, clippy and clippy-wasm32 pass; doc 2.0 s; test: measured 275, bites
292, 0 failed, 0 skips; deps pass; refs 556 tracked files; marks 81; self 48 of 48; redact 157 items, 0 findings). The
gate `node tools/gate.mjs --no-smoke` (17:32:47 UTC): GATE PASS (base 626df67; checks 1-4 and 5b-5d pass; symbols +0
-0; 5a skipped). CI: `gate` and `smoke` success on f080551. marks.json, a read-only Node parse: 114 bite ids, all
unique; G1a's old pattern misses 16 of them; check.mjs:678's pattern misses 0 (the reviewer's exact fix text,
u4s-fix2-review-rust.md:85-89).

**What I checked (file:line; spike paths under spikes/netcode-proof/):**
- The counts: measured 267 + 8 = 275; bites 281 + 11 = 292 (sim +10, client +1); self 43 + 5 = 48: deltas on U3d's
  base, as §5 asks.
- §4.1, the public items: crates/sim/src/lib.rs:12-26 re-exports only items in §4.1's npt-sim rows, the five field and
  config types the report names, and carry 30's additions; the bites-only items: sim lib.rs:33 and :39-43; grid.rs:198;
  npt-client: bites-only `BITES` (client/src/lib.rs:12) and `grid_with_bite` (:18-22); npt-proto adds manifest lines
  only. Report gap: its public-items line leaves out `OWN_COPY_SNAP_BITES`, `own_copy_snap_with_bite`,
  `Config::floor_rounding`, `Field::integer`, the five field constants and `EntityKind::ALL`/`layout`; carry 30 lists
  them (condition 2).
- §4.2, the seam: `step` is word for word §4.2:203 (seam.rs:271); `reports()` stays outside the hash (:273, :286;
  `InputReports` :247-265); `ClampReport` is not an event (:234-242); `Rewind` is integers only (:66-74); outside the
  ring names `tick_a + 1` when fraction > 0, and a refused Rewind acts as `None`, which D-10's twin needs (:58-65); what
  the sim reads (:15-20, netcode F1); the canonical order (:82-87, F2); the grid/raw rule (:167-175, F4).
- §4.3, the tick constants (ticks.rs): :12 `TICK_HZ` 30; :18-21 rate held in 1-1,000; :25-32 the ceiling, in u64 and
  saturating; :47 `LOOP_CATCHUP`; :83-98 the const asserts; s1 (tests/seam.rs:83-120) pins 7/15, 60/120, 32/64 and 30/60.
- §4.4, the grid (grid.rs): the fields :106-158 (XY 16 bits, ±327.68 m; Z 12 bits, 0 to 40.95 m; yaw 16 bits wrapping;
  pitch ±511 in 10 bits; velocity 1 cm/s); ties round away from zero (:5-9, :178); the defect is chosen per value and
  exists in bites builds only (:162-167, :193-200); s2 is exhaustive over 201,727 integers (tests/seam.rs:122-152); the
  error-bound sweep :154-213; s3 :215-239.
- HitHistory (history.rs): :22-32 `for_ms` via `ticks_for_ms`, None at 0 ms; :52-67 push; :70-83 get, wrap-safe; s4
  and s4b: tests/seam.rs:241-293.
- check.mjs greps (each wired into deps at :1466-1468): sim allow-list :74; tick grep :581; room float grep :657;
  bite-home grep :678 and :723; doc step :1365 and :1480; their self-tests: check.test.mjs:160, :1316, :1362, :1405,
  :1460-1474.
- §4.14, the bite ids at their sites: -17 lives in npt-client (client/src/lib.rs:12; client/tests/bites.rs); -41 lives
  at the own-copy snap side (sim lib.rs:33); the grid holds no id (grid.rs:21-26); both match §4.14:441 and :446. -57
  moves to U4a's `Quant` as integer coarsening: that changes the approved §5 U4a a4, so condition 1 covers it; the
  coarsening reproduces marks.json -57's three defects (4 cm, 0.088° and 0.70° steps).
- Condition 10: (a) bites features: sim Cargo.toml:15, proto :15, server :22; lab/Cargo.toml:14-21 forwards all six;
  (b) room :15; client :15; npt-server's dev dependency on npt-client. Cargo.lock: +14 lines, 0 new `[[package]]`.
- The reviews: three APPROVE WITH FINDINGS, no high; the four mediums settled in the code (netcode F1/F2: seam.rs:15-20,
  :82-87; arch F1: grid.rs:21-26, sim lib.rs:28-43, client lib.rs:8-22; arch F2: seam.rs:134-152, grid.rs:17-19); the
  rust-reviewer checked the fix pass (items 1-11, no regression) and the last round (G1, G2).
- The builder: u4-build-plan.md:803 (Lanes, step 0) says "U4s alone (the main session or one builder)"; the approved
  text at aebe8ea:709 says the same; §7.2 (:860-861) is what contradicts it.

**What I did not check:** I planted no bite (read-only; the plants are the builder's and the rust-reviewer's: 9, 31 and
the last round's, each restored by sha256 per their notes); I did not re-run `cargo tree -d` against the base
(Cargo.lock adds no package, so no new duplicate is possible); I did not repeat the rust-reviewer's native-against-
wasm32 grid probe (u4s-review-rust.md:70-75) or its reading of libm; no 60 Hz build (the const asserts and s1's 60 Hz
rows stand in for it); the 3-agent cap during the U4s workflows (from the checkpoint only).

**Conditions:**
1. **Plan text, before any wave-1 build starts.** Under condition 12 I approve exactly these edits to
   u4-build-plan.md: (a) carries 30-33 and 5, as written (u4s-carries.md:55-57, :145-157): §4.1's npt-sim and
   npt-client rows; §5 U4a a4 and §4.14: -57's bites go through `Quant`'s own bite choice, as integer coarsening, and
   the ids' homes are named; §4.3's FullState line; §4.15: 39 B a command, about 1,872 B a tick of commands. (b)
   NF-D4's text (below): §4.1:183 and §4.2:236-237 drop "non-finite components" and `non_finite`; §5 U4d's D-4 (:593)
   is replaced. (c) §7.2 (:860-861) matches Lanes: "…or, for U4s, U4b and U4j, the main session or one builder in a
   free lane". (d) Condition 3's freeze joins Lanes' wave-1 bullet (:808). Record the plan's new sha256 here, under this
   ruling. Any edit beyond (a)-(d) comes back to me.
2. **Complete u4s-carries.md before the wave-1 briefs are written:** add to U4d: s6 ties `EntityKind::ALL` to the enum
   with an exhaustive match (u4s-fix-review-rust.md:112-113; the report lists this carry, but the file does not);
   `TickInput` stays integer-only, its spawn, teleport and attribute fields included (NF-D4); `StateDump` holds exactly
   what `state_hash` hashes, never `InputReports` (condition 1's rule), so a twin's field diff never shows a counter.
   Reword carry 7: "no state depends on any other `Command` field" (carry 8's order assert reads `tick`, and that is
   correct). Add to U4b and U4e: a tick count needed outside npt-sim goes into npt-sim as `<name>_at` plus its constant
   (carry 18's route), and the crate uses it by path (why: check.mjs:627 lets an initializer that merely names `TICK_HZ`
   through, and nothing pins such a value at 60 Hz; u4s-fix-review-rust.md:114-115). The check.mjs nits
   (u4s-fix2-review-rust.md:90-93): nothing pins `STEPS.doc`; the doc step documents the host build only;
   `lintBothBuilds` says "both linted" when only the bites build fails; a measured-build doc failure's summary line is
   cargo's last line (the report leaves this fourth nit out); home: U4a's brief (wave 1's only check.mjs editor, :809)
   or U4d2's, never the main session while U4a owns check.mjs. The next edit of crates/proto/Cargo.toml (U4d2 or
   later): its bites comment (:13-14) still says the grid's bites are chosen through its config; after carry 2,
   npt-proto's bites are its own. Two report lines, fixed when U4's report folds this one: the 1 MB cap is carry 14, at
   U4d, once `HitVolumes` holds its volumes, not U4h; the public-items line takes carry 30's list.
3. **Freeze the seam's shapes for wave 1.** Through wave 1, U4d keeps unchanged every existing field, variant and width
   of `Command`, `Rewind`, `CoverVariant`, `EntityKind` and `EntityRecord` (FIELDS 12), and the five grid fields as
   built at f080551. It may add items (World, carry 17's button constants, `TickInput`'s new fields), complete
   `layout()` within FIELDS (carry 12), and drop `InputReports::non_finite` (NF-D4). A change to an existing item waits
   for U4d's merge and goes to the main session, as carry 1 already says for the grid. Why: U4a builds its codec and
   field tables against these types at the same time, so a silent change breaks the second merge.
4. **The wave-1 briefs carry the file's entries word for word** (u4s-carries.md:9-10). Each wave-1 report lists the
   carries it met, by number.

**What stands as built:** the seam and the integer Rewind, with `reports()` outside the hash; `HitHistory` built from
milliseconds; the tick constants as functions of `TICK_HZ`, held by const asserts and by s1 at 30 and 60 Hz; the grid's
one float-side defect, with each bite id at its own site; the four greps and the doc step (review-driven additions
guarding plan rules, §4.3, §4.11, §4.14, plan §4.5; none is scope creep); the proto dev dependencies (arch F7: already
in the lock, same pins, never shipped).

**Not asked, still open (O1, u4-conditions-check.md:39-42).** At U4r the room has neither the view-time tolerance nor
the cap; both land in U4h (u4-build-plan.md:678-679, :787). So the -30 pre-check's setup condition (0 replacements and
0 clips: my condition 3) holds by construction, and the RTT argument cannot be tested. This never reached me. U4r's
brief, which comes to me, must state it and either re-run the pre-check pair after U4h merges (6 runs) or say why not.
My lean is the re-run.

## Rulings (U4s)
- **U4S-ACCEPT:** accepted with conditions 1-4. Wave 1 may start once conditions 1 and 2 are in the text.
- **NF-D4:** option (a), in the words below (the report's (a) is the carries file's (b), because the letters are
  swapped; this ruling is on the words). Why: `Command` and `Rewind` hold integers only (seam.rs:21-43, :66-74), so the
  counter can never count, and a test that cannot fail is evidence of nothing. The rule is still met: ADR 0004:285-286
  ("every input and data value is checked finite where it enters", plus the assert each tick) and plan.md:529-530
  hold: an integer is finite by type, and the per-tick assert stays. **New D-4:** "every value that enters the sim from
  outside is finite by type: `Command`, `Rewind` and `TickInput`, its spawn, teleport and attribute fields included,
  hold integers only, so nothing is refused or counted; a debug and test assertion each tick that the World holds no
  non-finite value (every float read by `to_bits`, as the hash reads it), shown by a planted `0.0 / 0.0` in one phase
  that trips it at its tick (a scratch plant, §7.2); and an overflow test: recorded commands with `move_xy` at -128 and
  127 and `aim` at `i32::MIN` and `i32::MAX` step in a debug build (overflow checks on) without a panic (carry 15)."
  The field goes: U4d removes `InputReports::non_finite`. If a float field ever joins `TickInput`, the counter and its
  refusal test come back (none is planned, and the float-free room could not fill one anyway). `World::ray_cast`'s ray
  is the only float that reaches npt-sim code from outside; it takes `&self` and changes no state; U4h's AimCast states
  what a non-finite ray does. My condition 1's list loses "non-finite components"; the rest of it stands.
- **U4S-BUILDER:** acceptable. It is not a deviation from the approved plan: Lanes step 0 (:803; aebe8ea:709) allows
  one builder, and §7.2's contrary text is fixed by condition 1 (c). U4b and U4j: the same holds, under Lanes' wave-1
  "(or the first free lane)" (:808) and U4j's header (:614). A builder agent may build either, on these terms: only in a
  free slot, never as a 4th agent (reviewers count: condition 10 (c)); general-purpose, opus, effort xhigh, in an
  isolated worktree under §7.1; its files as Lanes gives them (U4b may also add its one `mod schedule;` line to
  crates/server/src/lib.rs, which no other wave-1 lane touches); merged by the main session, one at a time, with the
  reviewers §5 names; its report says who built it. Condition 10 (d) binds only the main session's own builds.
- **U4S-CARRIES:** almost complete: I matched every finding of the five review notes to a fix or a carry; the gaps are
  listed in condition 2. Before wave 1: only text (the plan-text carries 30-33 and 5, condition 1, and the freeze,
  condition 3). Every code carry can wait: each needs code its own later unit builds (World, Quant, the room, the ring's
  span).

## Decisions needed (U4s)
None. All four questions are technical and sit inside the approved spike: NF-D4 keeps ADR 0004 part 7's rule and lowers
no bar; the -57 bite move keeps §4.14's "planted where its code lives" and the float-free room; no ADR reading changes,
so nothing joins the readings PR.

## For Mustafa (U4s)
Nothing.

## Main session: the U4s ruling's conditions applied (2026-11-09 about 18:15 UTC)
- **Recorded, as condition 1 asks:** u4-build-plan.md after condition 1 (a)-(d), sha256
  7ec1b324287faefc7b1968184b599c8da691fcde9f992c888f195ed57579ff87 (1,031 lines; 12 edits, each marked "(the advisor's
  U4s ruling, condition 1 (x))"); checked by the test-engineer (u4s-conditions-check.md): every changed line traces to
  (a)-(d), the new D-4 and the freeze word for word.
- **u4s-carries.md after condition 2 and O1:** 44 entries (0b4b896b…), then the main session's edits from the check:
  entries 35 and 39's unruled route replaced by "open: the advisor rules on it before U4b's (U4e's) brief is written
  (F2)"; entry 34's "or a stated deferral" made "a fix listed in its report (a deferral leaves this entry unmet, and
  entry 42 applies)"; the header's stale "uncommitted" text and the open NF-D4 text marked as superseded; entry 45 added
  (NF-D4's U4h line, the check's F3). Its sha256 after these edits is in the checkpoint's Running work.
- **The U4 verdict's condition 1 list above** loses "non-finite components", as the U4s ruling's NF-D4 says; its text
  is left as returned, and this line records the amendment.
- **To the advisor:** F1 (§4.15's recording sizes still derived from the old 1,973 B a tick: about 16.8 MB per 9,000
  ticks and 50.5 MB for three, about 4.2 million Wasm calls each, at 1,872 B) and F2 (where U4b and U4e put a tick
  constant they need outside npt-sim, when crates/sim is U4d's lane in wave 1 and check.mjs U4a's, U4d2's in wave 2).

## Ruling: F1 and F2 (2026-11-09)
(Returned by the advisor, read-only, about 18:10 UTC; recorded by the main session. Formatting compacted; the words are
the advisor's.)

F1 is approved, and the same fix covers three more lines the conditions check missed. On F2, the main session's
option (a) has nothing to carry: as the plan stands, neither U4b nor U4e needs a new tick count. Two rate values U4e
builds get a rule, and anything found later takes route (b), timed for when no lane owns the files. Nothing goes to
Mustafa.

**What I checked (file:line; spike paths under spikes/netcode-proof/):** the size arithmetic: plan :463 gives the
field widths; the command is 1+4+2+2+4+2+12+4+1 = 32 B and its Rewind 1+4+1+1 = 7 B, so 39 B in all; 39 × 48 = 1,872
B a tick of commands (the old 1,973 B was 41 × 48 + 5); 1,872 × 9,000 = 16,848,000 B (16.8 MB); three recordings
50,544,000 B (50.5 MB); divided by 4, 4,212,000 words (4.2 million); with the 5 B tick header: 16.9 MB, 50.7 MB, 4.22
million; MB means 10⁶ B, as in the old 17.757 → 17.8; the aebe8ea text (its :437-438) confirms the old figures came
from 1,973 B. The same stale figure elsewhere: "about 18 MB" in three more plan lines: :46 (§0), :138 (§1, "§4.15, my
arithmetic") and :946 (§10, Q4); the conditions check's grep (u4s-conditions-check.md:33-35) looked only for "17.8",
"53 MB" and "4.4 million", so it missed them; plan.md, the spike's docs and code, and the main checkpoint hold none.
F2, the rule and its guards: carry 18 (u4s-carries.md:125-128) and carries 35 and 39 (:76-81, :162-167); check.mjs:541
(`TICK_DERIVED`), :581-633 (`tickConstantProblems`; :621-625 refuses a tick-count name in npt-sim that is not in
`TICK_DERIVED`; :627 the outside-npt-sim pass); ticks.rs:12-112, sim lib.rs:22-26 (the re-exports), tests/seam.rs:21-39
and :83-120 (s1's table and pins); u4s-fix-review-rust.md:114-115 (the nit itself). F2, what U4b and U4e need: plan
§4.3 :243-257, §4.5 :285-293, §4.7 :319-325, §1 :130-134; §5 U4b :540-548, U4e :683-713, U4f :771-774; Lanes :812-842;
carry 6 (:72-75); the catalogue's Input row (docs/design/netcode-proof-messages.md:87: "refilled at 2 × TICK_HZ a
second"); the current server, room and client sources: no use of `tick_hz` or `TICK_HZ` yet. Not checked: I did not
run check.mjs or the gate (text edits); I did not search the code for a constant U4e's builder might invent later
(the builder's and the review's job).

**F1 (U4-SIZES): APPROVE.** Replace u4-build-plan.md :467-468 exactly as proposed (the arithmetic holds: 16.8 MB, 50.5
MB, 4.2 million): "about 16.8 MB per 9,000 ticks and 50.5 MB for three, commands only (my arithmetic; U4d states the
measured size). Fed to Wasm as 32-bit words: about 4.2 million calls each." Also approved: "about 18 MB" becomes "about
17 MB" at :46, :138 and :946 (they derive from the same §4.15 arithmetic, and :138 cites it). Mark each edit "(the
advisor's F1 ruling)". Record the plan's new sha256 under this ruling. No other edit is covered. My U4 verdict's
"Numbers I re-derived" line (advisor-u4.md:67: 41 B, 1,973 B, 17.8 MB) is replaced by this ruling (39 B, 1,872 B, 16.8
MB); leave that line as it was returned; this ruling is the record of the change.

**F2 (U4-TICKROUTE): none of (a), (b) or (c) as offered.** (1) Nothing is needed today, so (a) has nothing to put in
U4d's brief: U4b needs no new tick count (`Schedule::new` takes the rate and the catch-up cap as arguments, §4.3:254-256,
carry 6; the period ⌊k × 10⁹ / tick_hz⌋ is computed from the rate passed in, tested at 30 and 60 Hz, :543); U4e's tick
counts are the FullState bound (`FULL_STATE_ACK_TICKS`: §4.7, the second-Resync rule) and the AckRing and client frame
ring (`ACK_RING_TICKS`: §4.5), both in npt-sim since U4s (lib.rs:22-26) and pinned by s1; U4e's Join deadline (2 s) and
Ping limit (10 a second) run in ns on the injected now (:692-693, :701-703) and do not depend on the tick rate; naming
constants in U4d's brief "for U4b and U4e" would add items nobody uses yet: the "for later" pattern, so no. (2) Two
values U4e builds depend on the tick rate but are not tick counts, and neither carry 35 nor the grep covers them: the
Input bucket's refill, 2 × TICK_HZ a second (:133-134; catalogue :87; tested at 60 a second, :702); the window's
per-tick budget, 256 kbps at TICK_HZ ticks a second (:132; §4.5:290-292). The rule: they follow carry 6's pattern: the
type takes the rate as an argument, production callers pass `npt_sim::TICK_HZ` by path, and a test pins each value at
30 and 60 Hz (the refill 60 and 120 a second; the window 32,000 B a second at both rates); no const or static outside
npt-sim is derived from TICK_HZ (the grep cannot see one under a name that does not end in `_TICKS` or contain
`CATCHUP`); this keeps the window's integer design with U4e instead of fixing it in npt-sim now. (3) Anything found
later takes (b): a new tick count, or a named constant (an array length, a const generic) that needs a value derived
from the rate: the builder writes no local form (no literal, no local formula, no local `ticks_for_ms` constant),
records the need in its report (name, definition, values at 30 and 60 Hz, the site) and builds the rest; the main
session then adds the constant by carry 18's route, as one commit of its own on committed state (condition 10 (d)): the
`_at` function, the constant and the const assert in ticks.rs; the re-export in lib.rs; s1's DERIVED row; check.mjs's
`TICK_DERIVED`; plan §4.1 and §4.3; timing: only when no lane owns crates/sim or check.mjs (in wave 1 after U4d and
U4a have merged; in wave 2 after U4d2 has merged), check.mjs and the gate green; the site is finished and the lane
merges after that commit; the needing lane's rust-reviewer reviews the commit; this costs nothing on the critical path
(U4e already waits for all of wave 1, and U4r already waits for U4d2). (4) (c) is not taken: pinning a local constant at
both rates needs a local `_at` function, which gives a derived value a second home (§4.3; arch F6's "never derived
again"); point 2's pattern already gives the 60 Hz pin wherever no constant is needed. (5) Replacement text for the
"open: the advisor rules…" parentheticals (the quoted rule in each entry stays word for word): entry 35 and entry 39 as
the main session has now written them in u4s-carries.md; record the carries file's new sha256 under this ruling. (6)
Note for later briefs: U4f, U4g and U4h run one at a time, and no lane holds crates/sim or check.mjs then, so each of
their briefs may give that unit the places carry 18 names, without a main-session step; the first known case is U4f's
lead bound (plan.md:514; U4f's "every other room-side check", :772-773).

**Decisions needed:** none (both points technical, inside the approved spike, text only; no ADR reading changes and no
bar lowered, so nothing joins the readings PR). **For Mustafa:** nothing.

**Recorded by the main session (about 18:15 UTC):** u4-build-plan.md after F1's four edits (each marked "(the
advisor's F1 ruling)"), sha256 0d69332c3c60c83c9328504259c279609b546e15cc5510f528346471aabc3b84; u4s-carries.md after
F2's two parentheticals (entries 35 and 39, each "(Route, the advisor's F2 ruling, 2026-11-09: …)"), sha256
505e54d6c0259092f08767a4052648898040437b4f2f01a8c47877adadff9c29. The wave-1 builders already running (U4d, U4c) were
briefed before these edits: F1 changes only §4.15's derived sizes (U4d states the measured size anyway) and F2 only
U4b's and U4e's entries, so neither running brief is affected.
