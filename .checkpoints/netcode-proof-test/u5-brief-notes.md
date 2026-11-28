# U5's brief: working notes (main session, 2026-11-21, KEEP-PROGRESSING work while F-B-COUNT waits)

## Plan
1. Gather U5's inputs (this file, block "Facts"), each with its source.
2. Draft u5-brief.md in u4h-brief.md's shape (sections 0-9), filled at launch (`<base>`, counts).
3. The advisor's check (general-purpose, opus, xhigh, reading main's advisor.md); revise to its verdict.
4. Card line, task #94; the build waits for U4's close or the advisor's ruling on order (no build now).

## Facts (block 1, 04:40 UTC)
- U5's row (plan.md:1252): "Fuzzing (-34); the dependency and swap checks (-35); the audits (§4.8)" plus C-U5-1 (word
  for word there: an Input fuzz target through `Room::receive` under options A and B, seeded, in the debug test build;
  no panic, exactly one counter per offered command, held <= 8, refusals <= 4,096 a tick, kept window <= 16).
- u4-build-plan.md:961-962 (§6, what U4 does not build): "U5: fuzzing every message type and the room-wide controls
  (-34); the counting allocator and the allocation half of catalogue item 3 (Q8); the audit of libm and every other U4
  dependency (-35)."
- u4s-carries.md:759-764 (U5 section): C-U6-19's one line (advisor-u4f-f1-r1.md:120-122) and carry 48 (C13's second
  half: a push into a full `HitHistory` allocates nothing, with -34's counting allocator; "U5's plan states where its
  allocator's `unsafe` lives under ADR 0002:264-265").
- plan.md:588-596 (§4.7): npt-proto's 13 messages + `SessionStatus` ("with its own fuzz target", the advisor's A1
  ruling) + `SessionControl` (4 variants) + `RoomControl` and its decoder, "built at U4 for the catalogue's item 6 and
  U5's fuzz target (the advisor's Q7 ruling, 2026-11-09, advisor-u4.md)".
- plan.md:635-653 (§4.8): floors (-35); "Fuzzing: cargo-fuzz on a dated nightly, recorded"; audits at U5, stop B and
  before every run: `cargo deny check advisories bans sources` (crates.io only, no git deps), `cargo audit` with the
  database date, `npm audit` on web/'s lockfile; an advisory on a network-facing crate (quinn, rustls, ring, h3,
  tungstenite, hyper, tokio) blocks the next run until handled. U3c's additions: tokio-rustls =0.26.6 on that list;
  npt-tcpinfo's four unsafe blocks with SAFETY invariants and tests; DelayStream's request and frame parser as a fuzz
  target (-34). plan.md:77: the audit row (security F15).
- marks.json:7054-7224 PM-0001-34 "No decoder panics or over-allocates": where C; 14 parts, one per message by
  `params.suite` "fuzz" and `params.msg` (Input, Snapshot, FullState, Resync, Ping, Pong, Join, JoinAccepted,
  JoinRejected, ClientSettings, RoomEnded, Leave, Kick, SessionControl), each min 1; checks: `event.fuzz_end.inputs_count`
  min >= 10,000,000; `event.fuzz_end.panics_count` sum == 0; `event.fuzz_end.over_limit_allocs_count` sum == 0; report
  `event.fuzz_end.corpus_count` max; bite PM-0001-34/unchecked-index ("an unchecked index in one decoder is found within
  the run"); notes: "Any machine: cargo-fuzz on a dated nightly (recorded in the header), a counting allocator; 1 run
  per message type of the catalogue (plan §4.7's npt-proto messages and SessionControl). The decoded control messages
  in random sequences driven into `room` (PM-0004-60's, with this row's count) are not judged here. The fuzz harness
  writes one `fuzz_end` event per run."
  - **No part for `SessionStatus`, `RoomControl` or DelayStream's parser,** though plan §4.7 and §4.8 give each a fuzz
    target. marks.json is his: a question for the brief (run them reported-only beside -34, or a marks.json change to
    him), never a change by the build.
  - PM-0004-60 is not in marks.json (only named in -34's notes).
- marks.json:7225-7322 PM-0001-35 "The transport stays swappable and current": where C; one part `params.suite`
  "deps", min 1; checks: `event.swap_diff.files_outside_transport_count` sum == 0 and count >= 1 ("the diff was read");
  `event.cargo_tree.sim_transport_deps_count` max == 0; header params quinn_proto_version >= 11018, rustls_version >=
  23046, ring_version >= 17012, tokio_version >= 1054001, wt_dangerous_config == 0; bites PM-0001-35/quinn-in-sim
  (R5-GATE-REC3's G5 mutation) and PM-0001-35/quinn-proto-0.11.17; notes: "Any machine, once, after the swap: the diff,
  `cargo tree`, the lockfile read by script. A version is written as one integer, major x 1,000,000 + minor x 1,000 +
  patch ..."
- Catalogue docs/design/netcode-proof-messages.md: §2 size limits per type (Input 166 B, Snapshot 1,000 B send / 1,150
  B refuse, FullState 65,536 B, Resync 8, Ping 32, Pong 32, Join 128, JoinAccepted 256, JoinRejected 16, ClientSettings
  8, RoomEnded 64, Leave 16, Kick 16, SessionControl 512, SessionStatus 32); §5 room-wide controls at most 1,024 B a
  message, "the same panic-free parsing as network messages"; §6 at U4 item 3 (:245-248): "a buffer one byte over its
  limit is refused before anything is allocated ... The allocation half ... is checked with PM-0001-34's counting
  allocator in U5: the advisor's Q8 ruling"; item 6 (:255): "The control-socket messages decode with the 1,024 B limit
  and never panic (plan §10.2; Q6)".
- u0-probes.md:9: `nightly-2026-11-01` exists (rust 1.101.0-nightly, f9ceb4a4d 2026-10-31); "installed at U5 when
  fuzzing starts".
- This container (04:3x UTC): toolchains stable and 1.99.1 (active) only; /root/.cargo/bin has no cargo-fuzz,
  cargo-deny or cargo-audit. U5 installs the nightly and three tools: pinned, `--locked`, recorded (a DEP-HOLD-like read
  by security-auditor before first use: the brief asks).
- Other U5 pointers in advisor rulings: advisor-u3c-plan.md:87 (futures-util audited at U5); advisor-u3q.md:279 (cargo
  deny and audit at U5); advisor-u4-wave1.md:273-277 (carry 48's words); advisor-u4.md:166 ((e) catalogue §6 item 3's
  allocation half moves to U5); advisor-u4d-close.md:349; advisor-u4f-r1.md:121 (RR-U4F-4 -> C-U5-1);
  advisor-ws-docs-r2.md:410 (npt-tcpinfo: "U5 audits it").
- The rest of U4 (card, 04:3x): -54 in U4h, -55 in U4g, -18 and -30 run in U4r; U4f2 waits on F-B-COUNT; U4g after
  U4f2's merge; U4h after U4g; the lane rule (u4-build-plan.md:951-952) covers U4f, U4f2, U4g and U4h only.

## Facts (block 2, 04:5x UTC)
- SERIES.md: -34's rows :160-163 written by "fuzz" (corpus_count, inputs_count, over_limit_allocs_count,
  panics_count); -35's rows written by npt-lab: :143 `event.cargo_tree.sim_transport_deps_count`, :229
  `event.swap_diff.files_outside_transport_count`, :244 `header.params.quinn_proto_version` (from the lockfile), :256
  `header.params.wt_dangerous_config`; SERIES.md:55 `tick.v.tick_allocs_count` (npt-server, counting GlobalAlloc,
  PM-0006-20: U6's).
- Unsafe: Cargo.toml:77-101 `unsafe_code = "forbid"` workspace-wide; three exceptions with their own [lints] tables
  (npt-client-wasm allow; npt-tcpinfo deny + two #[allow]s; npt-hash-wasm deny + seven #[allow]s); check.mjs's deps step
  enforces the table "(allow, deny, deny)" and scans the crates. ADR 0002's sentence is now at :269 ("`unsafe_code` is
  denied in every crate; an exception needs a written invariant, a test and an `#[allow]` with its reason"): the
  manifest cites it as :264-265 (drifted). A counting allocator (`unsafe impl GlobalAlloc`) is a fourth exception: its
  home, its invariant, its test and check.mjs's table change.
- Transport features: crates/net/Cargo.toml:12-15 `wt-wtransport` and `wt-wtq`, "swappable behind npt-link
  (PM-0001-35)"; **no crate outside npt-net enables either** (grep of every Cargo.toml and .rs: only npt-net's own
  cfgs and tests). So at U5 no binary picks a WebTransport crate: a swap diff then reads 0 files outside the transport
  only because nothing outside uses one (vacuous). WtServer and WtClient are U6's (plan.md:1253).
- Versions (Cargo.toml:46-75): wtransport =0.7.2, web-transport-quinn =0.13.6, quinn =0.11.12, tokio =1.54.1,
  tokio-tungstenite =0.30.0, libc =0.2.191, tokio-rustls =0.26.6, futures-util =0.3.34 (dev), libm =0.2.16, serde
  =1.0.229, serde_json =1.0.154, rand_chacha =0.10.0, rand_core =0.10.1, sha2 =0.11.1. quinn-proto, rustls and ring
  come through the lock (to read at launch).
- deny.toml (U1): advisories ignore = [] ("every advisory is an error unless listed with a reason and a date"); bans
  multiple-versions warn, wildcards deny; sources crates.io only.
- No package.json under spikes/netcode-proof (web/ is U7's): `npm audit` on web/'s lockfile has no target at U5.
- Decoders: npt-proto `decode(way, channel, bytes)` (msg.rs:303; `Quant::decode` :396 carries the bites build's
  planted defect), `RoomControl::decode(bytes)` (control.rs:601), Limits (limits.rs:12-51: one const per type,
  ROOM_CONTROL 1,024, NOTE 480, PEERS 16). DelayStream's scanner: crates/net/src/l2/ws_frame.rs `Scanner` (pub(crate);
  `client(max_payload, max_head)`, `server()`, `scan(&mut self, input)`), already with a seeded test
  `random_input_never_panics_and_stays_within_its_bounds` (:607). Room: `Room::receive(&mut self, slot: u8, msg:
  &Message)` (room.rs:890). Sim: `HitHistory::push(&mut self, volumes: &HitVolumes)` (history.rs:59).
- Toolchain: rust-toolchain.toml pins 1.99.1; cargo-fuzz runs `cargo +nightly-2026-11-01 fuzz ...` (the + override wins
  over the file). C and C++ compilers present (c++, g++, clang, clang++) for libfuzzer-sys's build.
- DEP-HOLD (main's .claude/CHECKPOINTS.md:60-69): a push changing any Cargo.toml or Cargo.lock (any folder) is held
  until dependency-check.md exists after security-auditor's answer on exactly that content; "Until then build nothing
  that holds the change". U5 adds manifests (a fuzz crate's Cargo.toml and Cargo.lock; a new member if the allocator
  gets its own crate): Block 0 is manifests only, then the read, then the build.
- The advisor's Q7 (advisor-u4.md:204-205): RoomControl at U4 for catalogue item 6 and U5's fuzz target; Q8 (:206) with
  nit (e) (:166): catalogue §6 item 3's allocation half moves to U5.

## Open for the brief (block 2: read; kept for the record)
- SERIES.md's `fuzz_end`, `swap_diff`, `cargo_tree` rows and the header params (written by whom).
- ADR 0002:264-265 (where an allocator's `unsafe` may live); the workspace's unsafe exceptions today.
- npt-net's features (`wt-wtransport`, `wt-wtq`) and what "the swap" changes today; npt-sim's `cargo tree`.
- Cargo.lock: quinn-proto, rustls, ring, tokio, tokio-rustls, tokio-tungstenite, futures-util, wtransport versions.
- package.json files under spikes/netcode-proof (npm audit's target before web/ exists: U7).
- deny.toml (U1) as it stands.
- The advisor's Q7 and Q8 rulings (advisor-u4.md).
