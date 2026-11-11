B10 notes: run-script model, crates/lab, crates/bots, crates/judge at df8975b.

ops/runs/c-u2-pm-0001-3.mjs (114 lines), the model:
- :3 usage `node spikes/netcode-proof/ops/runs/c-u2-pm-0001-3.mjs --date YYYY-MM-DD`; :40-42 refuses a bad date.
- :8, :43-45 refuses uncommitted changes to tracked files (every log names its commit).
- :9-10, :48-50 raw logs .gate/netcode-proof-test/runs/<SET>-<short>/ (gitignored, plan §10.6); results results/<SET>/<short>/ (short = 12 chars, :47); "Neither is ever overwritten" (exists -> throw).
- :52-61 builds npt-lab release --locked (measured), copies it out to .gate/netcode-proof-test/bin/<short>/npt-lab, then `--features bites` -> npt-lab-bites, then npt-judge; target path .gate/netcode-proof-test/target/release (main folder's cache, §7.1 item 6).
- :65-72 each run = one npt-lab call (`probe --run --set --role --profile --seed --date --commit --out <logs>/<run>.jsonl` + extra); stdout line kept for runs.txt.
- :73-84 measured runs per profile x seed with twins; bites in bites build at P2 seed 1 with own twin.
- :86-92 `npt-judge --marks marks/marks.json --logs <logs> --only PM-0001-3 --out verdict.json`; verdict copied to results.
- :93-104 runs.txt: set + date line; commit; sha256 of both npt-lab builds and npt-judge; raw-log path; judge's line; then one line per run.
- No wall time measured anywhere in the model.

crates/lab at df8975b
- Cargo.toml :12-21 bites forwards to npt-l1, npt-sim, npt-proto, npt-room, npt-server, npt-client; :23-34 deps npt-link, npt-l1, npt-sim, npt-proto, npt-room, npt-server, npt-client, npt-log, serde_json, sha2. NO npt-bots.
  -> FINDING (stop-class): U4r builds "npt-bots' library: the brawl `Behaviour` and the open-loop scripts" (plan :723) for L1Scenario's 48 clients; npt-lab cannot call them without a dependency on npt-bots: a lab/Cargo.toml + Cargo.lock edit no plan line names (U4s made wave 1's and U4e's manifest lines: Lanes :815-819; condition 10 (b): only U4d2 edits crates/lab in wave 2). Also npt-bots/Cargo.toml :20-25 deps npt-link, npt-proto, npt-client, npt-net, npt-log: no npt-sim (carry 17's button constants are npt-sim's, "the one home the sim, the client and the bots read"), no `bites` feature. QUESTION (manifests).
- src/lib.rs :1-6 doc only: names SimClock, L1Scenario (fixed order per instant: due packets, server I/O step, server tick if due, due client frames, due client ticks), ReplayX86, HashCompare.
- src/main.rs: one subcommand `probe` (:1-11, :186-213); probe and machine are modules of the binary; `profile()` refuses P0 for probe (:89-97; test :269); probe header `tick_hz: 30` literal (:151), `clients: 1`, `scene: "none"`, `process: "probe"`, marks ["PM-0001-3"] (:139-158); Writer + finish(Ended::Clean) (:159-165).
- src/machine.rs: container() :30, build(commit) :77.
- No Instant/SystemTime in crates/lab/src; no crates/lab/clippy.toml, no crates/bots/clippy.toml, no crates/server/clippy.toml (U4b adds server's). clippy.toml exist for client, l1, link, log, proto, room, sim.
  -> FINDING: §4.11 :380-383's npt-lab and npt-bots clippy.toml (clock ban) are named by no §5 unit; U4r owns both crates' U4 code. And the runner's measured ticks/s (:731-732) needs a wall clock: with npt-lab clock-banned, the run script must time it. QUESTION.
- plan.md §4.7 :616-627: npt-lab's surface `SimClock` (integer ns), `L1Scenario`, `ReplayX86`, `HashCompare`; each instant in order: 1 packets due (by arrival time then fate-log index, never delivered the instant sent, P0 included); 2 server I/O step; 3 server tick if due (tick k due at base + floor(k x 10^9 / TICK_HZ) ns); 4 due client frames (60 Hz, seeded phase); 5 each client whose next_due_ns has come, in slot order. Host clock per client: offset + rate x SimClock (rate 1.5 for -21's cheat). L1's RTT RFC 9002-style. Fates per (seed, link, direction, packet index). plan.md :610-611 binaries: npt-lab with probe (U2); scenario, replay, hash join in U4. plan.md :629-630 bites only in bites build, chosen by `--bite <id>`; header records bite id, enabled features, binary's SHA-256.
- plan §4.1 :196 npt-bots (library) `Behaviour` (brawl, open-loop scripts, TC shooter, cheats, never-acker), clock-injected; :197 npt-lab subcommands `scenario` (with `--record`), `replay`, `hash` (plan.md:590-591).

crates/l1: Profile::p0 (l1/src/lib.rs:116), by_name "P0" (:143), Profile::b1(burst_starts_ns) (:135), L1Net::with_bite (:858).

crates/bots: Cargo.toml bins npt-bots, npt-load (:12-18); lib.rs :1-3 doc only ("Built in units U4 and U6"); bins are stubs (fn main only).

crates/judge (U4j changes it before U4r)
- eval.rs:75-80 Missing::Gap (run invalid) vs NotBuilt (NOT-RUN); :310-323 an empty tick or second series is a gap ("Every tick and second record carries its counters and flags, 0 included (SERIES.md)"); an event count of none is a real 0; :1048-1051 any gap with no failure -> INVALID.
  -> confirms the -58 finding: `tick.v.untraced_corrections_count` must be written every tick or -58's runs are INVALID; written at U4r with no Predictor it is a structural 0.
- B10 done. Next: B11.
