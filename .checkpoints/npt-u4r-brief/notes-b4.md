B4 notes (u4-build-plan.md §4.11-§4.15, :379-488, and §5 preamble :489-495), at df8975b

§4.11 (:379-394)
- :380-383 HashMap/HashSet bans join room's and client's clippy.toml; NEW clippy.toml files for npt-server and npt-lab, and for npt-bots (all three with the clock ban naming std::time::Instant, std::time::SystemTime, tokio::time::Instant and their `now`: A14). npt-bots' file crate-wide; U6's runtime half + npt-net glue carry reviewed per-item allows.
  -> who builds npt-lab's and npt-bots' clippy.toml? U4b builds server/clippy.toml (:543-544). Not named in U4r's §5. Check df8975b for crates/lab/clippy.toml, crates/bots/clippy.toml.
  -> clock ban in npt-lab vs "the runner's measured ticks per second" (:731-732): where is wall time read? Q candidate.
- :384-385 npt-room float-free.
- :386-388 Bites per instance (T10): constructor arg or config value (World::with_bite, Room::with_bite, grid config), never a static, as L1Net::with_bite (l1/src/lib.rs:856-868). A test runs a bite instance and a clean one on parallel threads in one process.
- :389-390 Send asserts.
- :391-392 paths no L1 run reaches get a named unit test in U4.
- :393-394 The runner's own twin test: two processes, same seed and scenario: equal fate-log hashes, equal per-tick server and client hashes; bite: a HashMap-ordered client order (marks F29).

§4.12 (:396-412)
- :397-398 On L1 one npt-lab process writes one log per process value the checks select on (room, client, lab), each with own header (marks/README.md:9; marks F28).
- :399-402 contract test (a short scenario per suite): every check of every U4 L1 part resolves (no gap), every written name has its SERIES.md row, validity inputs exist (T18): one `admit` event per client (roster rule README:171 not vacuous), a footer per log, `tick.v.late_count` wherever `late-zero` applies. Also resolves PM-0001-2's checks on every SS suite (T1).
- table :405-412: -17 P0,P2,P3,B1 SS 48 pre yes; -18 B1 SS 48 pre yes; -20 P3 A and B; -21 8 suites; -22 never-ack pre NO (U4j writes README §4 exception); others none at U4.

§4.13 (:414-435)
- SS :415-420: 48 soldiers 3x16 one cluster; 15 big vehicles; 300 projectiles; 384 effect rows; doors one portal change/s; one destructible-wall stand-in; 700 shots/s; DM_Tri's extent. soldiers_count counts alive or awaiting respawn.
- shot band :421-431: 700/(48x30) = 0.486 shots per command-tick; B1 option A ~691.5 >= 665; option B ~700; -21 forged ~691; one silent client 685.4 >= 665; none above 735. :430-431 "U4r's contract test shows the band per suite on its short runs; a suite outside it goes to the advisor before its marks count."
- :432-433 S1 stand-ins (1 ms burn, 2.80 ms top-up) off on L1, recorded in params.stand_ins; 384 effect rows stay.
- :434-435 ES, CO, TC per ADR 0001:1470-1482; TC thin plates.

§4.14 (:437-460) relevant to U4r runs:
- :438-439 each bite planted where its code lives, unit-shown in that sub-unit, run judged in the sub-unit that moves its mark.
- :443 -17/tick16, -58/tick16: site proto header writer + client tick rebuild; check decoded hash: post-wrap frames under wrong tick (orphan pair) or sentinel; run setting -17: tick_start = 63,736; -58: ticks_below_wrap = 30.
- :444 -17/client-rounding: any -17 profile; check decoded hash.
- :445 -18/no-fallback: check burst_heal (the fresh delta); B1, ring ages 0-31.
- :452 -58/lt-cmd-seq: 0 commands refused; sequence 2 below its wrap.
- :453 -58/lt-press: 0 presses lost or doubled; presses scripted through the wrap.
- :454 -58/lt-tick-offset: 0 untraced corrections; offsets crossing the wrap. (needs U4g's Predictor per U4E-Q2)
- :455 -58/lt-event-seq: decoded hash; event numbers 3 below wrap; SS's hits flow.
- :456 -58/lt-ack: 0 commands refused; acks crossing the wrap.
- -58 six bites = :443 (tick16) + :452-456 (five) = six.
- :460 PM-0004-21 hash-map: D-2 and U4d2.
- -57's bites pos-4cm, yaw-12bit, pitch-8bit NOT in §4.14 (they live in Quant, U4a a4 :529-531 per U4e brief R3).

§4.15 (:462-487)
- :463-466 fields: header (scene, seed, TICK_HZ, pool sizes, HitHistory's capacity (nit b)); per tick a TickInput: per soldier Command (slot 1, tick 4, seq 2, buttons 2, press counters 4, move 2, aim cm 12, view tick 4, fraction 1) and its Rewind (present 1, tick_a 4, fraction 1, cover 1); spawns, teleports, attribute changes. Fixed-width LE, no usize.
- :467-470 size 39 B/command; ~1,872 B/tick for 48; ~16.8 MB per 9,000 ticks; 50.5 MB for three.
- :471-475 both uses: sim-only replay; -30 pre-check takes SS P0 recording, re-encodes commands as Input (stamped ticks, view times, sequences, press counters from recording; ack from what that leg's replay client decodes live) and lets the room compute its own Rewinds.
- :476-477 one driver npt_sim::replay::Replayer; npt-lab's `hash` subcommand runs it natively (U4d2).
- :478-484 sources: U4d's 3 synthetic; U4r's three runner recordings (`scenario --record`: SS P2 seed 1, ES P3 seed 2, TC B1 seed 3) holding the room's real Rewinds; fourth SS P0 seed 4, -30 pre-check source. "Replayed at P0, the other three's view times sit about one RTT older than P0's estimate, so the room would replace them on both legs, each from its own RTT; a recording made at P0 keeps them inside the tolerance." Determinism step keeps its three; fourth serves the pre-check only.
  -> NOTE: :481-483 argues via the tolerance, which at U4r does not exist (U4h). O1 relevant.
  -> NOTE: TC B1 recording at U4r: TC scene, B1 profile; TC's shooter Behaviour is U4h's (:801). U4r's recording of TC uses which behaviour? Q candidate (brawl Behaviour? open-loop?).
  -> NOTE: "the determinism step keeps its three": U4d2's step green on recordings (Evidence :769). Three = U4d's synthetic or U4r's runner three? "The determinism step keeps its three" reads as the runner three (U4r's) added to the step... ambiguous. Evidence :769-770: "the recordings' SHA-256s and U4d2's step green on them (in check.mjs if the step stays within 5 minutes, else through ops/runs/det-u4.mjs ...)".
- :485-487 regenerated at check time under .gate/ from a committed recordings manifest (scene, seed, script, generator version) with each SHA-256; different hash fails step, names recording. Never committed as files.
  -> NOTE: U4r's runner recordings regenerated at check time = running scenario --record (an L1 run) inside check.mjs? Cost question. "if the step stays within 5 minutes".

§5 preamble (:489-495)
- every sub-unit ends with check.mjs green on both builds; counts as deltas on its base (from U3d's 267 measured, 281 bites, self 43) recomputed after merge; clippy -D warnings host + wasm32 both builds; fmt; Cargo.lock changed only by the dependencies named; merge into main folder one at a time; check.mjs and `node tools/gate.mjs --no-smoke` there on its own cache; commit, checkpoint save, push. Bite = planted defect compiled only in bites build, chosen at run time (plan.md:609-610), must fail the named test or check. Each report lists new pub items (§7.2).
