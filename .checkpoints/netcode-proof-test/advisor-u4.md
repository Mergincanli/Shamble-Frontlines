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
