# Plan: the netcode proof test (SPIKE_PLAN item 1), revision 1

- **Change-set:** `netcode-proof-test` · **Branch:** `claude/netcode-proof-test-plan-hcky5g`. This cloud session may push
  only to this `claude/…` branch, so it stands in for `cs/netcode-proof-test` (CLAUDE.md, standing permission).
  **Base:** main `af8393e`.
- **Date:** 2026-11-05 · **Status:** for the advisor's plan review, then Mustafa's. No code until both approve.
- **Sources** (all in this folder, all line-cited): `netcode-architect-digest.md` (every PM-0001 row with its setup,
  the profiles, the budgets, the netcode checklist, the build list E1–E21, problems H1–H20, a run order);
  `infra-engineer-digest.md` (PM-0006-11 to -27, PM-0004-67, the operations checklist, Hetzner facts and prices read
  2026-11-05, the route, Safari runners, DNS, certificates); `test-engineer-digest.md` (L1 and L2, the client harness,
  `spikes/` rules, this container's toolchain, every planted defect); `ssh-probe.md` (item 1's evidence). Where this
  plan says "netcode digest H2" or "infra digest F", the detail and its draft lines are there.

## 0. In short
- **Item 1, SSH: no.** This session cannot reach any server over SSH. The image has no `ssh` client; TCP to port 22,
  and to every port but 443, times out; 443 goes through an egress gateway that re-terminates TLS. It can read any
  public HTTPS site. **Recommended route: the hybrid.** A reviewed cloud-init file (Hetzner's "Cloud config") builds
  every VM the same way at creation; Claude Code on Mustafa's laptop does the hands-on part over SSH, with a key made
  on that laptop; this session writes the code, proves everything that needs no VM, and judges the results.
- **What gets built:** a throwaway Rust and TypeScript spike in `spikes/netcode-proof/`. It has a stand-in sim (native
  and Wasm); a room server with both WebTransport crates and the WebSocket fallback; the link simulator's two layers
  (L1 on simulated time, L2 under the real socket); headless bots and a load generator; the client harness (page,
  Worker host, plain renderer); instrumentation; and a judging script that turns data into verdicts. The server and
  harness stay on main afterwards, for the 60 Hz run and the boundary spike. No legacy file changes.
- **What it proves:**
  - PM-0001-1 to -62, without the 60 Hz group (-39, -40, -63, which runs later in its own VM session);
  - PM-0004-67;
  - PM-0006-11 to -27.
  The scripts judge every measured mark. Mustafa judges "looks correct" at P2 in Brave, and runs the operations
  checklist. P2 is 100 ms round trip, ±20 ms jitter and 2 % loss, applied half each way, the loss on each direction
  on its own.
- **Order:**
  1. The container-side build and every logic proof. This needs no VM hours.
  2. A security review of everything that faces the internet.
  3. Eight VM sessions. Each has its own run plan, with hours and a cost cap for Mustafa's OK.
  4. His session in Brave, inside those eight.
  5. The reviews and the report.
  A ninth session, the named-machine re-run, comes later as its own short change-set.
- **Cost:** VS1 to VS8 plan 39 RS-hours and up to 84 LG-hours, EUR 6.88 before VAT at today's prices (CCX23
  EUR 0.1386/h, CAX21 EUR 0.0176/h), with caps adding up to EUR 8.60. The later re-run adds about EUR 0.47. Each cap
  comes to Mustafa in its own run plan.
- **Safari:** GitHub's macOS runners first. Today's images carry Safari 26.6.2 and 27.2, above the 26.4 that
  WebTransport needs. The runner workflow is a `.github/` change, so **it goes to Mustafa for approval**: its own
  small PR, because a `workflow_dispatch` workflow runs only from main.
- **DNS:** one record per test host, always "DNS only" (grey cloud). Type `A`, name `rs.nt`, value RS's IPv4 of the
  day, TTL 1 min, in every session. No AAAA.
- **Decisions:** two for Mustafa (ROUTE, SAFARI-TIME; §16). Twenty-four readings for the advisor (§15), each with a
  recommendation.

## 1. Item 1: can this session reach the servers over SSH?
### 1.1 What was tested (2026-11-05, 21:30–21:45 UTC; each probe once; `ssh-probe.md`)
| Probe | Result |
|---|---|
| `ssh`, `ssh-keygen`, `scp`, `sftp` in the image | all four missing |
| TCP to `github.com:22`, and to `1.1.1.1:53` (a non-443 port elsewhere) | both time out |
| Port 22 through the agent proxy (`curl -p -x $HTTPS_PROXY telnet://github.com:22`) | times out, no tunnel |
| TCP to `github.com:443` | connects, but the certificate is the egress gateway's ("Anthropic … Egress Gateway"): TLS is re-terminated, so SSH carried on 443 cannot pass either |
| HTTPS to unrelated public hosts (nasa.gov, ikea.com, bbc.co.uk, docs.hetzner.com, Let's Encrypt's ACME directory) | 200 |
| `shamblefrontlines.com` over DNS-over-HTTPS | on Cloudflare (NS `cora` and `steven.ns.cloudflare.com`); no A, AAAA or CAA record yet |

The proxy's own notes list "non-443 HTTPS ports, raw-TCP …" and "WebSocket upgrades" as not supported: "report, do
not work around". **So: no.** A probe against a real VM adds nothing: port 22 is blocked to every host. If Mustafa
wants it on record anyway, VS1 repeats the probe against RS and records it.

### 1.2 What follows
- This session cannot SSH to a VM. It cannot be a WebSocket or WebTransport client of RS (WebSocket upgrades are
  refused by the proxy, and QUIC is UDP). It cannot reach a non-443 port.
- It can build and test all the Rust and TypeScript here (Rust 1.99.1 installs: `static.rust-lang.org` answers). It
  can run every L1 row (simulated time), push to GitHub, start and read GitHub Actions runs, and read public HTTPS.
- Whatever the server route, the browser legs on the minimum-laptop stand-in (WebdriverIO, ADR 0006's PM-0006-15)
  need scripts running on Mustafa's own Windows machine (infra digest E).

### 1.3 The routes
| | (a) Hybrid (recommended) | (b) Cloud-init only | (c) Claude Code on his machine only |
|---|---|---|---|
| Provisioning | a cloud-init file from the repo at the run's commit: an unprivileged user, password and root login off, a second firewall (nftables), the pinned toolchain, a clone at a full commit SHA built with `--locked`, the `wss` certificate on RS | the same | the operator session over SSH, step by step |
| Who runs a session | Claude Code on his laptop, over SSH | a fixed run script in the cloud-config; nobody can look inside a running VM | the operator session |
| Results to this session | the laptop session pushes redacted summaries to its own results branch; this session reads them with `git fetch` and `git show` | over HTTPS from a results server on LG (its own DNS name and certificate: new surface) | as (a) |
| Mustafa, per session | OK the cap; create LG, then RS, pasting the file; set the day's firewall sources and DNS; start the laptop session with that session's prompt and stay near it; delete the VMs | OK, create, paste, DNS, delete; nothing to watch, but every failed start costs a new VM and a new paste | as (a), with a slower bring-up |
| A failed run | diagnosed in place | a new VM | diagnosed in place |
| Security | SSH by key, only from his address of the day; the key never leaves the laptop | no SSH at all (smallest surface on RS), but a new results web server on LG | as (a) |
| Fit with the drafts | yes | yes, but PM-0006-15's rule list then lacks SSH (R-18), and the browser legs still need someone at the laptop | yes |

### 1.4 Recommendation
(a), with the operator session on Mustafa's **laptop**. The laptop is the stand-in for the minimum laptop (ML), so the
browser legs run there anyway (SPIKE_PLAN:59-60). His PC stays for what he judges in Brave (MP, never a measured
result: SPIKE_PLAN:59-63). Either machine works; he chooses (ROUTE).

### 1.5 The SSH key
- **This session cannot give a key that works**: it cannot connect. Under (a), the laptop's Claude Code session makes
  the key pair there, with Windows' own OpenSSH (`ssh-keygen -t ed25519 -C npt-operator`). So the session that holds
  the private key is the one that made it (SPIKE_PLAN:154-156).
- The private key never leaves the laptop: never in the repository, a checkpoint, a chat or a paste.
- Mustafa adds the public half once to the spike's Hetzner project (Security, SSH keys) and picks it at each VM's
  creation: a key can be added only at creation (infra digest D).
- SSH (TCP 22) is open only from the laptop's public IPv4 of the day (/32), which he sets in the firewall that day.
- The cloud-init file turns off password and root login and gives the same public key to the unprivileged `npt` user.
  A public key is not a secret, so it may sit in the repository.

### 1.6 Checking each new VM's host keys (for security-auditor to pick; SPIKE_PLAN:65-66)
1. **Recommended for RS:** cloud-init publishes RS's SSH host-key fingerprints on RS's page host:
   `https://rs.nt.shamblefrontlines.com:31000/npt/ssh-host-keys.txt`, once the browser-trusted certificate is issued.
   The laptop session fetches the file over verified HTTPS, writes it to `known_hosts`, and connects with
   `StrictHostKeyChecking=yes`. The trust rests on Mustafa's DNS record and the CA's validation.
2. **For LG, and as RS's fallback:** cloud-init also writes the fingerprints onto the console's login screen
   (`/etc/issue`). Mustafa opens the VM's console in Hetzner and confirms they match what the laptop session shows,
   before it accepts. Alternatively LG gets its own name (`lg.nt`) and certificate, and the same file as option 1.
3. TOFU (`accept-new`) alone is not a check, so it is out.

Out as well: host keys made off the VM and passed in through user data. That would put a secret in the paste, and
every process on the VM can read its user data.

## 2. Goal and scope
- **Goal:** prove the online path that ADR 0001's "What a spike must prove" sets out (ADR 0001:1237-1335): the Rust sim
  native on the server and Wasm in the client; WebTransport with the WebSocket fallback on four browsers; 48 simulated
  clients in the stress scene at 30 Hz within the budgets; one human client at P2 seeing prediction, reconciliation
  and lag-compensated hits that look correct. Feed the questions it decides (SPIKE_PLAN:196-204), and leave its server
  and harness for the 60 Hz run and the boundary spike.
- **Why a player cares:** online play that still feels local at a 100 ms round trip. The netcode checklist is the
  part a player feels, and Mustafa judges it.
- **In scope:**
  - the marks in §6;
  - the message catalogue under `docs/design/` (`.claude/rules/netcode.md`);
  - the results written back into ADR 0001 at the close: the crate, the encoding, the process model, the datagram
    cap and the rewind cap (ADR 0001:1689-1691). That is an ADR edit, so it needs Mustafa's go.
- **Out of scope:**
  - the 60 Hz group (PM-0001-39, -40, -63): its own VM session after the sim core spike (SPIKE_PLAN:206-248);
  - the boundary spike's marks (PM-0002-*; PM-0004-8, -28, -61, -68 to -70). This test only leaves its server
    re-runnable at P2 for -61's and -70's online halves;
  - the look;
  - a lobby, real join tokens, reconnecting, private rooms and economy (ADR 0001:1613-1618);
  - a CI job for `spikes/` (R-8).
- **Not changed:** `shamble-*.js`, the two pages, `tools/`, `.claude/`, `CLAUDE.md`. `.github/` changes only through
  the separate Safari workflow PR, with Mustafa's approval (§9).

## 3. Rules this plan keeps
- SPIKE-SCOPE (a): throwaway, in `spikes/`, exempt from the legacy hard constraints. One heavy spike at a time
  (SPIKE-ORDER (b)). Server marks at 30 Hz, N = 48.
- A mark, threshold or setup changes only in the draft, by the advisor or Mustafa, never inside the spike
  (ADR 0001:1603-1604). The spike keeps its marks as data (`marks.json`, with each value's draft line), and any
  difference from the draft is a decision.
- **Bites:** every automated mark has a planted defect that must fail first, before its pass counts
  (ADR 0001:1499-1504).
- **No re-runs to green:** a failing run is kept and its cause found (ADR 0001:1505-1506; 0006:1375-1376).
- Rows on ML, RD or MAC are judged only on a named machine. Until then they are rough checks, reported and never
  judged, with a short re-run once the machine is named (SPIKE_PLAN:156-160; ADR 0001:1406-1408). §6.3 lists them.
- The Arm rule: a mismatch seen only on LG's aarch64 clients decides nothing until the same inputs replayed on x86-64
  show it too (ADR 0001:1337-1346).
- The repository is public: no secret, token, raw payload or home address in a commit or a note (§10).
- Checkpoints as `.claude/CHECKPOINTS.md` sets them: at most 3 agents at a time, notes saved as they go, other
  branches only in a separate worktree.

## 4. What gets built
### 4.1 Layout (all new; no existing file changes)
```
spikes/netcode-proof/
  README.md               what it is (throwaway), how to run each part
  rust-toolchain.toml     channel "1.99.1", components clippy + rustfmt, target wasm32-unknown-unknown
  Cargo.toml, Cargo.lock  one workspace, resolver 3, edition 2024; every build and test runs --locked
  crates/sim/             npt-sim: the stand-in sim (no I/O, no clock reads, no transport dependency)
  crates/proto/           npt-proto: the messages, quantisation (E3), the delta codec, the size limits
  crates/net/             npt-net: the thin transport interface; L1; L2 (impairment socket, delaying TCP proxy);
                          the WebTransport adapters (wtransport and web-transport-quinn, one feature each); WebSocket
  crates/server/          npt-room (one room process), npt-super (the supervisor stand-in), npt-host (the page host)
  crates/client/          npt-client: the client endpoint (prediction, reconciliation, interpolation, clock,
                          events); native and wasm32
  crates/bots/            npt-bots (headless, cheating and hostile clients), npt-load (the load orchestrator on LG)
  crates/judge/           npt-judge: results logs + marks.json -> one verdict per mark
  fuzz/                   cargo-fuzz targets, one per message type
  web/                    the client harness: TypeScript, Vite, three.js (exact pins); the page, the Worker host,
                          the plain renderer, the overlay and record form; WebdriverIO specs for the browser legs
  marks/marks.json        every mark as data: id, statistic, threshold, window, validity, the draft line
  ops/cloud-init/         rs.yaml, lg.yaml: Hetzner's Cloud config files (no secret)
  ops/firewall/           rs.txt, lg.txt: each role's expected rule list (PM-0006-15)
  ops/runs/               VS1.md … VS9.md: each session's run plan (content, hours, cap, DNS, Mustafa's steps)
  ops/operator/           the laptop session's prompt for each session, and its rules (§12.3)
  ops/checklists/         netcode.md (PM-0001-53, written by test-engineer), operations.md (ADR 0006)
  results/                committed verdicts and redacted summaries per session, never raw data
docs/design/netcode-proof-messages.md   the message catalogue (throwaway subset of ADR 0001:654-678 + spike controls)
```
`crates/server/` and `crates/net/` fall under `.claude/rules/netcode.md` (`**/server/**`, `**/net/**`), so those rules
load with them. Builds, raw logs and packet captures go under `.gate/netcode-proof-test/` (gitignored), never in the
repository.

### 4.2 Components, each traced to the marks that need it (netcode digest E has the design part for each)
| # | Component | Crate | Marks |
|---|---|---|---|
| E1 | Stand-in sim, native and Wasm: movement, hitscan, bots as protocol clients; the stress scene's counts (48 soldiers in 3 × 16, 15 big vehicles of which 4 have at least 20 parts, 300 live projectiles as trajectory records through one shared function, S1's effect stand-ins: 384 rows, 1 ms burned, the rest topped up to 2.80 ms); a DM_Tri-sized greybox with enterable buildings, areas, portals and doors at 1/s; the target course and the correction course; seeded RNG, no clock reads, a per-tick state hash | sim | -2, -4, -5, -17, -30, -41 to -62 |
| E2 | Room process: the tick thread alone on one vCPU, async I/O on the others, bounded channels only, `panic = "unwind"` with a top-level catch → RoomEnded(crash), a crash record and a non-zero exit; per-phase timers and counters; the S17 loop (elapsed time, at most 7 ticks per wake, dropped time counted) | server | -1, -4 to -11, -15, -16, -26, -59; PM-0004-67 |
| E3 | Commands: Input datagrams (tick, sequence, move, buttons, press counters, aim ≤ 80 bits, view time, newest-decoded ack), redundancy up to 8, U2 delta repeats; every server check (strictly newer tick, lead bound, catch-up cap, clamp beyond reach); the input buffer; **both OQ-0001-6 options, switchable**; the late-command counter | proto, server | -13, -20, -21, -30, -49, -56, -58 |
| E4 | Snapshots: delta against the last acked, E3 quantisation, entity groups each decoding alone, the datagram cap (1,000 B until -36 measures), FullState on its own stream, Resync, the bounded event queue, the per-client 256 kbps cap, newest-only over WebSocket | proto, server | -6, -8, -12, -14, -17, -18, -19, -22, -23, -31, -57, -58, -62 |
| E5 | Interest: the region circle with hysteresis on a grid; areas and portals flood-numbered, re-flooded on change; the priority accumulator (own state always, nearest 8 enemies, everyone at least 1 in 4) | server | -8, -9, -10, -12 |
| E6 | Transports behind one interface: WebTransport on quinn 0.11 through wtransport 0.7.x **and** web-transport-quinn 0.13.x, swappable; WebSocket through tokio-tungstenite 0.30.x; the 2 s Join deadline, handshake cap, per-address caps (/32 and /64), Origin allow-list (403), `wss` only; NewReno or Cubic; the version floors | net | -11, -14, -24, -25, -28 to -38 |
| E7 | The client harness, shared with the boundary spike: a TypeScript page plus a module Worker running npt-client as Wasm; the Worker owns the connection; WebTransport first (1.5 s timeout), then WebSocket; the plain renderer on both backends (three.js WebGPU renderer and its WebGL 2 path; not the look); per-frame logs; `maxDatagramSize` every 10 s; the isolation flag; the overlay and record form; a 100-session connect page | web, client | -19, -28, -29, -31 to -33, -36, -41 to -46, -51 to -53, -61, -62 |
| E8 | Prediction and reconciliation in npt-client (replay newer commands, 100 ms fade, 2 m snap, cues keyed once) | client | -41 to -44, -60, -62 |
| E9 | Interpolation: buffer in the Worker; delay from 3 intervals, adapting between 2 and 5; extrapolation at most one interval, then hold; placed respawns; events released with their poses | client | -45, -46, -61 |
| E10 | Clock: Ping/Pong with hold time, answered on the I/O runtime; samples timed at datagram arrival | client, server | -33, -47, -50 |
| E11 | Lag compensation: per-tick hit-volume history (at most 1 MB), the rewind query, the view-time tolerance, a switchable cap (a test cap above every honest rewind, then the rule cap), a per-shot log of its parts | server | -7, -48 to -51 |
| E12 | Headless bots on npt-client: 30 inputs per second, the brawl, the target-course shooter, the cheats (forged kinds, 1.5× clock, lag switch, a 150 ms pong delayer, the tolerance-edge stamper, the never-acker), raw byte logs for re-decoding | bots | -2, -17, -19 to -22, -48, -50, -54 |
| E13 | Load orchestration: clients spread over the CAX21s, the 5-minute calibration (under 80 % CPU every second), inputs per second within 1 %, the metrics role on one CAX21 | bots | validity of every LG row; -25 |
| E14 | Hostile clients and fuzzing: 1,000 sessions that never join, oversize datagrams and stream openers, slow-loris, a 1,000-address /64 cycler (if LG has IPv6), `ws://` and wrong-Origin pages; cargo-fuzz per message type with a counting allocator | bots, fuzz | -25, -34 |
| E15 | The link simulator: L1 (seeded, simulated time, packet-fate log, behind the thin interface) and L2 (impairment under quinn's socket, per peer; a delaying TCP proxy for WebSocket; rate limit; bottleneck; per-session profile switch; B1 trigger; UDP block); the reorder share per stream in every run | net | -3 and every P1–P4/B1 row |
| E16 | Density harness: 8 room processes unpinned under the supervisor, bracket B, SIGKILL, 384 sessions | server, bots | -27; PM-0006-13, -14 |
| E17 | Recordings and hashes: recorded input replays driving both transports; per-tick server state hashes; the client's decoded-state hashes; the x86-64 re-decode and replay tool for the Arm rule | sim, judge | -17, -19, -21, -30, -48, -54, -57, -58 |
| E18 | Instrumentation: monotonic reads at tick and phase edges; clock resolution per machine; per-thread CPU and RSS each second; queue lengths; bytes per client counted at UDP level (payload + 28 B, from the socket wrapper, never app payload sizes); packet capture for fragments; steal and affinity reads | server | -1, -4 to -16, -24, -25 |
| E19 | Bites: one planted defect per automated mark (test-engineer digest F, plus R-12's proposals) | all | every automated row |
| E20 | Results: JSON Lines per run (a header per machine, per-tick, per-second and event records); npt-judge with nearest-rank percentiles and its own fixtures | judge | all |
| E21 | Hosting and certificates: the page host on RS (HTTPS on its own origin, COOP/COEP headers, the hash file, the host-key file); the WebTransport development certificate; the `wss` certificate by ACME | server | -25, -28 to -32, -36, -53; PM-0006-15, -25 to -27 |
| E22 | Metrics and logs: `metrics` with `metrics-exporter-prometheus` on RS (scraped by Prometheus on LG), bounded labels; `tracing` JSON lines; canaries for PM-0006-21 | server | PM-0006-16, -19 to -24 |

### 4.3 Symbols to add (public items; nothing is removed or renamed anywhere)
- **npt-sim:** `World`, `World::step`, `World::state_hash`, `Scene::stress`, `Greybox`, `Areas`, `Portals`,
  `SimRng`, `trajectory_at`, `StandIns`.
- **npt-proto:**
  - the messages: `Input`, `Snapshot`, `FullState`, `Resync`, `Ping`, `Pong`, `Join`, `JoinAccepted`, `JoinRejected`,
    `ClientSettings`, `RoomEnded`, `Leave`, `Kick`;
  - the spike-only controls: `SetProfile`, `SetRewindCap`, `TriggerBurst`, `BlockUdp`;
  - the rest: `encode`, `decode`, `Quant`, `Limits`.
- **npt-net:**
  - the transport interface: the traits `Link` (`send_unreliable`, `send_reliable`, `recv`, `state`, `rtt`) and
    `Listener`;
  - L1: `L1Net`, `FateLog`, `Profile`;
  - L2: `ImpairSocket` (quinn's `AsyncUdpSocket`) and `TcpDelayProxy`;
  - the adapters `WtServer` (features `wt-wtransport` and `wt-wtq`) and `WsServer`.
- **npt-room** (binary): `Room`, `run_tick_loop`, `PhaseTimers`, `Counters`, `Interest`, `LagComp`, `InputBuffer`
  (option A or B), `EventQueue`.
- **Other binaries:** npt-super, npt-host, npt-bots, npt-load.
- **npt-client:** `ClientEndpoint`, `Predictor`, `Interp`, `ClockSync`, `CueLog`.
- **npt-judge:** `Verdict` (`Pass`, `Fail`, `Invalid`, `StandIn`, `Reported`, `NotRun`).
- **web:** `main.ts`, `worker.ts`, `render.ts`, `overlay.ts`, `record.ts`, `wdio/*.spec.ts`.
- **Legacy:** the gate's symbol conservation finds no change, since no legacy file is touched.

### 4.4 Toolchain and pins
- **Rust:** 1.99.1, today's stable (2026-10-13) and the foundation's pin. This container has 1.97.0, so unit U0
  installs 1.99.1 and the wasm32 target through rustup and records the result. `Cargo.lock` is committed; every
  build and test runs `--locked`.
- **Floors** (PM-0001-35): quinn-proto ≥ 0.11.18, rustls ≥ 0.23.46, ring ≥ 0.17.12, tokio ≥ 1.54.1; wtransport's
  dangerous configuration stays off.
- **Fuzzing:** cargo-fuzz on a dated nightly, recorded.
- **TypeScript side:** Node 22 (the container has 22.23.4); TypeScript 6.0.x, Vite, WebdriverIO and three.js, each
  exact. three.js is pinned at the newest release on the day, ADR 0003 part 2's rule. Installs run
  `npm ci --ignore-scripts`.
- **On the VMs:** the same `rust-toolchain.toml`, installed by cloud-init from a rustup-init with a pinned SHA-256.
  LG builds its aarch64 binaries natively from the same commit, and every binary's SHA-256 goes in each run's header.

## 5. Where each part runs
| Place | What runs there |
|---|---|
| This container (x86-64 Linux) | every build and unit or property test; every L1 row (simulated time, logic marks only: ADR 0001:1410-1411); fuzz (-34) and the dependency checks (-35); both transports and L2 on loopback (no egress needed); the harness in local Chromium as a rough check; the Arm rule's x86-64 re-decodes and replays (R-8); judging every result |
| RS (CCX23, 4 dedicated vCPU, x86-64) | room processes, supervisor, page host, L2, metrics exporter; every timed server row on one vCPU |
| LG (CAX21s, Arm64; 1 to 4 by calibration) | bot clients (aarch64), the load orchestrator, Prometheus (the metrics role, MV), the ping target |
| Mustafa's laptop (the ML stand-in) | the operator session (route (a)); the Chrome, Edge and Firefox legs as rough checks |
| Mustafa's PC (MP) | Brave: the smoke check and his netcode checklist; his ping |
| GitHub macOS runner | Safari's yes-or-no and count rows, once conditions (i) to (iii) are recorded (§9) |
| A borrowed Mac, later | Safari's timed rows (SAFARI-TIME) |
| The named ML and RD, later | the short re-run of their rows (R-14) |

## 6. Evidence: how every mark gets its verdict
### 6.1 Verdict rules
- **The checker:** `marks.json` holds every mark as data, with its draft line. `npt-judge` reads each run's JSON Lines
  results log and writes one verdict per mark. The log has a header per machine (machine, build commit, binary
  hashes, profile, seed, clock resolution, affinities), then per-tick, per-second and event records.
- **The verdicts:**
  - PASS or FAIL;
  - INVALID, when a validity rule failed: neither a pass nor a fail (ADR 0001:1497-1498);
  - STAND-IN, for PM-0001-4 if the sim is lighter than the stress scene (ADR 0001:1432-1435);
  - REPORTED, for a row with no pass number;
  - NOT-RUN.
  Percentiles are by nearest rank, ⌈0.99 n⌉ (ADR 0001:1465-1466).
- **The checker is tested too,** on fixed fixtures that must come out PASS, FAIL and INVALID.
- **Bites first:** every automated mark's planted defect runs first and must FAIL. Only then can a pass count.
- **No retries.** A failing run keeps its data, seed, commit, machine, browser and profile, and its cause is found
  before anything else runs under that mark.

### 6.2 Every mark: where and when (setups exactly as in the netcode and infra digests, section A)
Key:
- **C** = this container (L1 runs: 3 seeds per profile; any-machine rows);
- **VSn** = a VM session (§7);
- **ML\*** = the laptop stand-in (a rough check: reported, never judged; re-run on the named laptop);
- **MAC-r** = the GitHub macOS runner; **MAC-b** = a borrowed Mac.

| Mark | Where and when | Notes |
|---|---|---|
| PM-0001-1, -2 | every VS (headers; load counters) | preconditions of every timed row |
| -3 | C (L1, U2); VS1 and the start of every L2 session; MP's 100 ± 5 ms check in VS8 | R-6, R-9, R-17 |
| -4 to -9, -11 to -15 | VS2: 3 × S30 on one vCPU of RS (-12's ES runs 3 × 10 min; -13 also at P4) | bites by rules B-S and B-N |
| -10 | VS2: N = 24 and 36, 3 runs each | |
| -16 | VS1: at least 20 starts | |
| -17, -18, -20 | C (L1) + VS3 (L2, LG) | -17 Arm rule; -20 under both OQ-0001-6 options; R-5 |
| -57, -58, -21, -22, -47, -50 | C (L1) | -58: R-5 |
| -19 | VS3 (LG, 100 joins) + VS8 (ML\* Chrome) | Arm rule |
| -23, -24, -26, -37, -38, -59 | VS3 | -59: R-16 |
| -25 | VS4: floods from LG (WT and WS, cap raised and cap on); Origin from ML\* and MAC-r | |
| -27 | VS5, VS6, VS7 (PM-0006-13's runs) | |
| -28 | ML\* (server on the laptop's loopback: R-7) + MAC-r (server on RS) | both crates; control: a wrong hash fails 100 of 100 |
| -29 | VS4: ML\*; Safari on MAC-b (SAFARI-TIME) | a time row |
| -30 | C (loopback) + VS3 (RS) | invalid, not failed, if the late counter > 0 |
| -31 | VS3 (LG) + VS8 (ML\* Chrome) | |
| -32 | VS8: ML\*; Safari on MAC-r if R-13, else MAC-b | |
| -33 | VS8: ML\* Chrome over L2 | |
| -34, -35 | C | fuzz on a dated nightly |
| -36 | VS8: ML\* (loopback and the home EU path); Safari on MAC-b | sets the cap -14 uses |
| -41, -42, -45, -60, -61, -62 | C (L1) + VS8 (ML\* Chrome over L2; -42 and -61 with 47 bots from LG) | |
| -43 | VS8: ML\* (rough); RD later (nothing stands in for RD) | named machines only |
| -44, -46 | VS8: ML\* (Chrome, and Firefox for -44) | |
| -48, -49 | C (L1) + VS3 (L2, LG): first the test cap, then the rule cap by OQ-0001-5's rule | -48 Arm rule; the rule cap feeds checklist steps 7 and 10 |
| -51, -52 | C (L1, -51) + VS8 (ML\* Chrome; -52 also in Mustafa's session) | |
| -53 | VS8: Mustafa, Brave on his PC, at P2 (§11) | judged by Mustafa |
| -54, -55 | C (L1) + VS3 (-54 on L2) | conditional: R-11; -54 Arm rule |
| -56 | every run | the record is complete |
| -39, -40, -63 | not in this change-set | the 60 Hz run |
| PM-0004-67 | VS2 | R-15 |
| PM-0006-11 | every VS | |
| PM-0006-12 | VS1: Mustafa's `ping` from the laptop and his PC to LG (ICMP on LG only) | reported |
| PM-0006-13, -14 | VS5, VS6, VS7 (3 times of day; one between 18:00 and 23:00 Amsterdam) | R-19 before -14's first session |
| PM-0006-15 | VS4: 20 + 20 per browser, ML\* and MAC-r; the rule lists pasted from the console | R-18 |
| PM-0006-16, -21 to -24 | VS5: the 30-min soak with 8 rooms, the canaries, three stalls, the series count | -16's probe from ML\* |
| PM-0006-17, -19, -20 | VS2 | |
| PM-0006-18 | after the last VM session: Mustafa reads the bill | under each session's cap |
| PM-0006-25 to -27 | VS4: ML\*; Safari on MAC-r if R-13, else MAC-b | |

### 6.3 Rows run before ML, RD and MAC are named (SPIKE_PLAN:156-160 asks this plan for the list)
- **On the laptop stand-in, rough checks only** (reported, never judged; re-run on the named laptop in VS9):
  - the ML legs of PM-0001-14, -19, -25 (Origin), -28, -29, -31, -32, -33, -36, -41 to -46, -51, -52 and -60 to -62;
  - PM-0006-15, -16's probe, and -25 to -27 in Chrome, Edge and Firefox.
- **RD:** PM-0001-43 and OQ-0001-15's RD report. Nothing stands in for RD; they wait for it.
- **MAC:**
  - the runner covers -14, -25's Origin, -28 (with its server on RS) and PM-0006-15, plus -32 and PM-0006-25 to -27
    if the advisor extends the list (R-13);
  - a borrowed Mac covers -29 and -36, and anything the advisor does not extend (SAFARI-TIME).

### 6.4 The netcode evidence rule (CLAUDE.md protocol step 4)
"Deterministic tests with injected latency, jitter and loss" is what L1 is for.
- **How L1 works:** it is seeded and runs on simulated time, with the real server and client code in one process. It
  applies profiles P0 to P4 and B1, half each way, with each direction's loss drawn independently.
- **What makes it deterministic:** the same seed gives the same packet-fate log and the same per-tick hashes
  (PM-0001-3).
- **What sits under it:** unit tests, plus property tests on fixed seeds, for the samplers, the codec, the
  quantisers, the wrap-aware compare and every decoder.
- **How many runs:** every L1 row runs 3 seeds per profile.
- **What L2 adds:** it runs in real time on RS, for the time marks L1 may not judge.

## 7. The VM sessions
### 7.1 Prices and the cap rule
- **Prices** (read 2026-11-05 on hetzner.com, before VAT): CCX23 EUR 0.1386/h, CAX21 EUR 0.0176/h. The plan prices
  include the IPv4; if it were billed on top, it adds EUR 0.0008/h per VM.
- **Rounding:** each VM's hours round up per session.
- **Traffic:** RS and LG talk internally in `fsn1`, which is free; browser traffic is far under the 20 TB included.
- **Cap:** a session's cap is its planned hours plus one spare hour per VM, at those prices, rounded up to the next
  EUR 0.10.
- **Planning check** (PM-0006-18, arithmetic shown): VS1 to VS8 plan 39 RS-hours × 0.1386 = 5.4054, plus 84
  LG-hours × 0.0176 = 1.4784, so **EUR 6.88**. The caps add up to EUR 8.60. VS9, later, plans EUR 0.47 (cap 0.70).

### 7.2 The sessions (provisional; each gets its own run plan, `ops/runs/VSn.md`, for Mustafa's OK first)
| # | Content | Mustafa | RS h | LG h | Planned EUR | Cap EUR |
|---|---|---|---|---|---|---|
| VS1 bring-up | provisioning check; -1, -2, -3 on L2; PM-0006-11, -12 (his ping); LG calibration; certificate and page host; a five-browser smoke page incl. Brave; Safari runner conditions (i)–(iii); -16; the first bites | present, about 1.5 h | 4 | 4 (1 CAX21) | 0.62 | 0.80 |
| VS2 single room, timed | 3 × S30 (-4 to -9, -11 to -15); -10; -12 ES; -13 at P4; PM-0004-67 and its stall runs; PM-0006-17, -19, -20; their bites | no | 7 | 7 | 1.09 | 1.30 |
| VS3 behaviour on L2 | -17, -18, -19, -20 (both options), -23, -24 (both controllers), -26, -30, -31, -37, -38, -48 and -49 (test cap, then rule cap), -54, -59; bites | no | 6 | 6 | 0.94 | 1.10 |
| VS4 hostile input and certificates | -25 (4 floods), slow-loris, the /64 cycler; -29; Origin; PM-0006-15, -25, -26, -27 | present, about 2 h | 3 | 3 | 0.47 | 0.70 |
| VS5 density A | -27 and PM-0006-13 (bracket B, 8 rooms, SIGKILL); PM-0006-14 at N = 48; the 8-room soak for PM-0006-16, -21 to -24 | no | 5 | 20 (up to 4) | 1.05 | 1.30 |
| VS6 density B, ≥ 6 h later | -27 and PM-0006-13 again; PM-0006-14 at N = 36 and 18 | no | 5 | 20 | 1.05 | 1.30 |
| VS7 density C, between 18:00 and 23:00 Amsterdam | -27 and PM-0006-13 again; PM-0006-14's count and count + 1 | no | 5 | 20 | 1.05 | 1.30 |
| VS8 the human session | the ML stand-in legs (rough): -19, -31, -33, -36, -41 to -46, -51, -52, -60 to -62; then his checklist at P2 with the rule cap | present, about 3 h | 4 | 4 | 0.62 | 0.80 |
| VS9 later (own change-set) | the named-machine re-run (ML, RD); a borrowed Mac if SAFARI-TIME (b) | present | 3 | 3 | 0.47 | 0.70 |
| **Total** | | | **42** | **87** | **7.35** | **9.30** |

The hours are rough until each run plan fixes them; the density sessions depend on PM-0006-14's search and on how
many CAX21s the calibration asks for. A session that would pass its cap stops and comes back to Mustafa.

### 7.3 Mustafa's steps per session (each run plan repeats them with that day's values)
1. OK the session's run plan: its hours and cap.
2. In Hetzner, in the spike's own project, Falkenstein (`fsn1`), Ubuntu 24.04:
   1. create LG (CAX21) with the operator key and `ops/cloud-init/lg.yaml` at the run's commit pasted into "Cloud
      config";
   2. then RS (CCX23), the same way, with `ops/cloud-init/rs.yaml`;
   3. in density sessions, add the extra CAX21s the calibration asks for.
   Give both IPv4 addresses to the laptop session.
3. **Firewall:** apply the two firewall objects kept in the project (free; not VMs), and edit only the day's
   sources: SSH from the laptop's public IPv4, and the metrics rule from LG's IPv4.
4. **DNS:** add the record in §8.3 with RS's IPv4.
5. Start the laptop session with `ops/operator/PROMPT-VSn.md`, and check the host keys as §1.6 says.
6. Do the browser steps when the session has them.
7. **At the end:** delete every VM, then delete the DNS record. After the last session, read the project's costs
   (PM-0006-18).

## 8. Network, ports, DNS and certificates
### 8.1 Ports (OQ-0006-16 is open; its deciders are infra-engineer and netcode-architect, confirmed by the Advisor: R-1)
| Use | Port | Open to |
|---|---|---|
| WebTransport, room slot s (0–31) | UDP 30001 + s (30001–30032) | everyone (browsers and runners cannot be allowlisted) |
| WebSocket (`wss`), room slot s | TCP 31001 + s (31001–31032) | everyone |
| The page host: the harness, the certificate-hash file, the host-key file | TCP 31000 | everyone |
| Metrics exporter (scraped) | TCP 9100 | LG's IPv4 of the day only |
| SSH | TCP 22 | the laptop's public IPv4 of the day only |
| ACME http-01 | TCP 80 | everyone, while a certificate is issued (Let's Encrypt validates from unpublished places); RS's own nftables drops it otherwise |

- 32 slots cover PM-0006-14's search at N = 18.
- The page host sits inside the TCP range, so RS keeps part 5's shape: one UDP range, one TCP range, the metrics rule,
  SSH, and TCP 80 (ADR 0006:225-242).
- PM-0006-15's closed-port probe uses a port outside both ranges.

### 8.2 Firewall rule lists (inbound; outbound stays Hetzner's default "all"; the lists go in `ops/firewall/`)
| Role | Rules |
|---|---|
| RS | UDP 30001–30032 any · TCP 31000–31032 any · TCP 80 any · TCP 9100 from LG · TCP 22 from the operator; no ICMP (ADR 0006:1350) |
| LG (each CAX21) | TCP 22 from the operator · ICMP from the operator (PM-0006-12's ping) |

### 8.3 DNS records (Cloudflare, zone `shamblefrontlines.com`; every record "DNS only", grey cloud)
| Host | Type | Name, as typed | Value | TTL | Proxy status | When |
|---|---|---|---|---|---|---|
| RS | `A` | `rs.nt` (= `rs.nt.shamblefrontlines.com`) | RS's public IPv4 of that day | 1 min | DNS only | every session, as soon as RS has its address; RS's cloud-init waits for it before asking for the certificate |
| RS, spare name | `A` | `rs-b.nt` | RS's public IPv4 of that day | 1 min | DNS only | only when the session says `rs.nt` would get a 6th certificate within 7 days |
| LG | `A` | `lg.nt` | LG's public IPv4 of that day | 1 min | DNS only | only if security-auditor picks HTTPS-published host keys for LG too (R-4) |

- **"DNS only"** answers with the server's own address and carries no traffic through Cloudflare. A proxied record
  would carry only HTTP(S), never WebTransport's UDP (infra digest G).
- **No AAAA record.** The proof test runs on IPv4: IPv6 changes the header sizes that PM-0001-14 and PM-0006-17
  count.
- **No CAA record for now.** It is optional, and Cloudflare adds its own CAA records once any exists.
- **Delete each record when its VM is deleted,** so the name never points at an address Hetzner hands to someone
  else.

### 8.4 The WebSocket leg's browser-trusted certificate (OQ-0006-5, development case (i); with security-auditor: R-2)
- **Recommended: ACME http-01 from RS itself.** A pinned ACME client, run by cloud-init, makes RS's private key on RS;
  the key never leaves it. Each VM opens a new ACME account, so no secret exists anywhere.
- The first dry run goes to Let's Encrypt's staging service.
- **Rate limits:** Let's Encrypt allows 5 certificates per exact set of names per 7 days, and 50 per registered domain
  per 7 days. So the session results keep a ledger of every issuance, with the spare name ready.
- **Lifetime:** the 6-day profile if the client supports it, else 90 days.
- **Public names:** every issued name lands in the public Certificate Transparency logs, so stray probes will come.
  The firewall and the test join tokens contain them, and the results record them.
- **Not taken:**
  - dns-01 with a Cloudflare API token: a secret on a VM;
  - tls-alpn-01: it needs TCP 443, which part 5 keeps for the lobby (OQ-0006-16 leaves 443 to slice 3);
  - an IP-address certificate: PM-0006-15 names a certificate on the domain.
- **Fallback, if TCP 80 is refused:** dns-01 by hand. The client prints one TXT value, and Mustafa adds it per
  issuance.

### 8.5 The WebTransport development certificate (ADR 0006:302-305; PM-0006-25 to -27)
- **Made by:** the room server or supervisor, on RS, at start: ECDSA P-256, valid at most 14 days, never written to
  the repository.
- **Its hash:** the page host publishes it, with the next one for PM-0006-26's rotation, in
  `/npt/cert-hash.json` (`Cache-Control: no-store`) over the browser-trusted name. So the pin travels on a channel
  the browser already trusts. The bots read the same file.
- **PM-0006-27's 15-day certificate** is made the same way, on purpose.

## 9. The Safari leg (MACHINES-TIMING; SPIKE_PLAN:45-57)
- **Facts** (infra digest F, read 2026-11-05):
  - `macos-26` (arm64) carries Safari 26.6.2 with SafariDriver 26.6.2; `macos-15` (arm64) carries Safari 27.2. Both
    are at least the 26.4 that WebTransport needs.
  - Images update weekly; each job's log names its image.
  - Minutes are free on a public repository.
  - Nothing official says whether UDP or QUIC leaves a macOS runner, and a 2022 issue shows UDP 123 dropped. So
    condition (iii) is a real test.
- **The workflow, for security-auditor's and infra-engineer's review when written. Any `.github/` change goes to
  Mustafa for approval.**
  - **File and trigger:** one new file, `.github/workflows/npt-safari.yml`, `workflow_dispatch` only. Its inputs: the
    host name, the ports, the commit SHA, and the set of rows to run.
  - **Runners:** `macos-26` and `macos-15` in a matrix, labels pinned, never `-latest`.
  - **Permissions and secrets:** `permissions: contents: read`; no secrets and no environment;
    `persist-credentials: false`.
  - **Actions:** each pinned by its full SHA, as `gate.yml` does.
  - **Inputs:** passed through `env:` and checked before use: the host must end in `.nt.shamblefrontlines.com`, the
    ports must be numbers inside the ranges, the commit must be 40 hex characters. No `${{ inputs.* }}` inside a
    `run:` line.
  - **Limits:** `timeout-minutes: 30`, and a concurrency group so two runs never overlap.
  - **Steps:** check out the commit; `npm ci --ignore-scripts`; `sudo safaridriver --enable`; record `sw_vers`, the
    Safari, SafariDriver and image versions; run the WebdriverIO Safari spec.
  - **Output:** one JSON artifact with a short retention. It holds versions and verdict inputs, no address but the
    VMs', and no token.
- **Why a separate PR (W-PR):** `workflow_dispatch` runs only from a file on the default branch (docs.github.com, read
  2026-11-05). So the workflow reaches main in its own small PR before VS1, on Mustafa's approval and his go. The kit
  is frozen until this test is under way, and he decides when that is.
- **First dispatch (VS1):** it records conditions (i) to (iii):
  - (i) the image's Safari version, at least 26.4;
  - (ii) real Safari through safaridriver, not a WebKit build;
  - (iii) one WebTransport session to RS that opens and carries datagrams, and one `wss` session on the domain.
  If (i) or (iii) fails, Mustafa borrows the Mac before the test, by his own rule.
- **What the runner may stand in for** (the advisor's reading, SPIKE_PLAN:54-57): the yes-or-no and count rows
  PM-0001-28 (server on RS), -14, -25's Origin, and PM-0006-15. It cannot stand in for a time row (-29), for -36
  (which needs a home EU connection), or, unless the advisor extends the list (R-13), for -32 and PM-0006-25 to -27.

## 10. Security
- **What faces the internet while a VM lives:**
  - RS: the game and page ports, open to all (§8.1); TCP 80 while a certificate is issued; SSH from one address; the
    exporter to LG only.
  - LG: SSH and ICMP, from one address.
- **Hardening the marks demand anyway:** the 2 s Join deadline, the handshake cap, per-address caps, size limits, the
  Origin allow-list, panic isolation, and no unbounded queue (PM-0001-25, -26, -34, -59).
- **Secrets:** none in the repository, the cloud-config or a checkpoint. The SSH private key lives only on the
  laptop, and the TLS keys only on RS. There is no Hetzner or Cloudflare token anywhere.
- **Join tokens:** test tokens, made per session on RS (real tokens are out of scope).
- **Raw data stays off the repository:**
  - what stays out: LG's received-byte logs (about 1.4 MB/s), packet captures and per-tick logs. They stay on the VM,
    or in `.gate/` on the laptop;
  - what is committed: verdicts, numbers, hashes and redacted headers;
  - Mustafa's home address never appears: the results write "the operator's address";
  - PM-0006-21's canary values never leave the VM;
  - whether the day's VM addresses may be committed once the VMs are deleted is R-21.
- **The two-session split:**
  - the laptop session operates VMs and browsers only, and pushes redacted summaries to its own branch,
    `claude/npt-results` (R-23);
  - this session reads that branch with `git fetch` and `git show`. Its main folder never switches to the branch or
    merges it (CLAUDE.md, Never).
- **Supply chain:** pinned crates and npm packages; `--locked`; `npm ci --ignore-scripts`; rustup's own checksums;
  a pinned SHA-256 for rustup-init in the cloud-config.
- **Reviews:** security-auditor reviews this plan now, and every internet-facing part again at stop B, before the
  first VM exists (§12).

## 11. Mustafa's checks (in Brave on his PC, and on the laptop)
1. **VS1, the five-browser smoke check (about 10 minutes).**
   - On his PC, in Brave, open `https://rs.nt.shamblefrontlines.com:31000/?smoke=1`. Expect "WebTransport · Playing"
     within 3 s, and Brave's version shown on the page.
   - Open the same address with `&transport=ws`. Expect "WebSocket · Playing".
   - The laptop session repeats both in Chrome, Edge and Firefox.
   If Brave cannot pin a certificate hash, the page says so, and R-24 applies (netcode digest H4).
2. **VS1, the operations checklist, step 1 (PM-0006-12).** In a Windows terminal on the laptop and on the PC, run
   `ping -n 100 <LG's address>` and paste the "Packets" and "Approximate round trip times" lines into the chat.
3. **VS8, the netcode checklist (PM-0001-53; about 60 minutes).** test-engineer writes it into
   `ops/checklists/netcode.md` from ADR 0001:1631-1681, and the advisor checks it before VS8.
   - Setup: 14 steps at P2, with 47 bots and Mustafa in the 48th slot, plus the target course.
   - The overlay shows the profile, the transport, the server-measured round trip, the interpolation delay, the
     rewind cap and the counters.
   - He records each step as "achieved" or "not yet" in the page's form. The mark passes when every step is
     "achieved"; a "not yet" goes back to the draft.
   - Step 7 needs the rule cap that VS3's honest-rewind runs set.
4. **After the last session, the operations checklist, step 2 (PM-0006-18).** Read the spike project's costs: one
   amount before VAT.

## 12. Steps, stops, sessions, checkpoints and agents
### 12.1 Units (each ends with its tests green, the gate run, notes saved and the checkpoint pushed; S under a day, M one to three days)
| Unit | Size | Content |
|---|---|---|
| U0 | S | Container probes, each recorded: rustup 1.99.1 with the wasm32 target; the wasm-bindgen CLI at `Cargo.lock`'s version; a dated nightly for cargo-fuzz; Chromium for local rough checks; a WebTransport session over loopback QUIC |
| U1 | S | Skeleton: the workspace, pins, results-log format, `marks.json`, npt-judge with its PASS, FAIL and INVALID fixtures; the message catalogue in `docs/design/` (drafted by netcode-architect) |
| U2 | M | L1 with its unit and property tests; PM-0001-3's L1 half with its bites (R-6, R-17) |
| U3 | S | R5-TEST-OQ2 on loopback: each WebTransport crate on a custom quinn endpoint over `ImpairSocket`; the delaying TCP proxy. This fixes L2's shape; if neither crate allows it, stop (R-22) |
| U4 | M | The sim, the protocol, the room, the client endpoint and the bots; then the L1 row groups, each with its bites: codec (-17, -57, -58); commands and hostile input (-20, -21, -22); prediction (-41, -42, -60); interpolation, clock and events (-45, -47, -61, -62); lag compensation (-48 to -51); bursts (-18); stand-ins (-54, -55); -30 on loopback |
| U5 | S | Fuzzing (-34); the dependency and swap checks (-35) |
| U6 | M | Both WebTransport crates and WebSocket with L2 on loopback; interest; instrumentation; the supervisor, page host, metrics and logs |
| U7 | M | The Wasm build; the native-against-Wasm hash; the client harness (page, a Worker host in ADR 0002 part 5 and 6's shape, the plain renderer on both backends, the overlay, the record form); a local rough check in Chromium |
| U8 | S | The run package: cloud-init files, firewall lists, VS1's run plan, the operator prompt and rules, WebdriverIO specs, the netcode checklist (test-engineer), the Safari workflow (as text in the notes) |

### 12.2 Stops (where the work waits for a person)
- **Stop A, now:** this plan, reviewed by the advisor, then Mustafa.
- **Stop B, before any VM exists:**
  - security-auditor and infra-engineer review the run package and every internet-facing part, the Safari workflow
    file included;
  - fixes follow;
  - Mustafa approves W-PR (the workflow, its own small PR to main) and gives his go to merge it;
  - he OKs VS1's run plan and cap.
- **Each VM session:**
  - his OK of the cap first;
  - his presence in VS1, VS4 and VS8;
  - after each session: the results judged, the checkpoint updated, and a three-line summary for him.
- **A run that fails or comes out INVALID:** its cause is found before the next run under that mark. A needed change
  to a mark goes to the advisor.
- **Stop C, the close:**
  1. the final diff goes to the verifier and the reviewers (§14), and fixes follow;
  2. the report (`/advisor-report`) and docs-keeper's STATUS, ROADMAP and REPO_MAP;
  3. the results go into ADR 0001;
  4. the checkpoint is deleted, the PR is marked ready, and the advisor reviews it;
  5. it merges on Mustafa's go: his checklist is a feel check, and the ADR edit is a rules change.
- **VS9, later:** the named-machine re-run, as its own short change-set (R-14).

### 12.3 Two sessions, one writer of code (R-23)
- **This cloud session** (and any later one that resumes from the checkpoint) owns the change-set's branch and all
  code.
- **The laptop session** operates only:
  - SSH, the run steps, the host-key checks and the browser legs;
  - it never edits the spike's code;
  - it never pushes to the change-set's branch;
  - it pushes redacted results to `claude/npt-results`;
  - it starts from `ops/operator/PROMPT-VSn.md`, which carries its rules: what to redact, no secrets, the key stays on
    the laptop, stop and report when this session is needed.
- **A fix during a session:**
  1. this session pushes it;
  2. the laptop session moves its own worktree to that commit;
  3. on the VM it runs `git fetch`, a checkout of that full SHA, and `cargo build --locked`.

### 12.4 Checkpoints, agents, models
- **Checkpoint:** `.checkpoints/netcode-proof-test.md`, updated after every unit and at least every 30 minutes. The
  background saver runs while agents run. Notes and small scripts go in `.checkpoints/netcode-proof-test/`; builds,
  raw logs and captures go in `.gate/netcode-proof-test/`.
- **Gate and tests:** `node tools/gate.mjs` must PASS at every step (no legacy file changes). The spike's own tests
  (`cargo test --locked --workspace`, and the `web/` tests) run at every unit, with their output in the checkpoint.
  No machine checks `spikes/` today (test-engineer digest G9).
- **Agents:**
  - at most 3 at a time, workflow agents included;
  - every agent on Opus 5.5 at `xhigh`, since this is a netcode change-set: netcode-architect, security-auditor,
    test-engineer, rust-reviewer, engine-architect, infra-engineer, perf-auditor, the verifier and the advisor;
  - agents never edit source. They write notes here, plus what their agent files allow: netcode-architect the
    catalogue in `docs/design/` and, at the close, ADR 0001's results; test-engineer the checklist text, which the
    main session commits into `ops/checklists/`;
  - no parallel build agents: the main session is the one writer of code.
- **Night queue:** nothing. The routine guard runs no `cargo`, and every unit needs it.

## 13. Risks, ranked
1. **Pinned-hash WebTransport may not work on every browser** (Safari, Firefox, Brave). PM-0006-25 and -28 need it in
   all four. Handling: the local check in U7, the five-browser smoke page in VS1 before any long session, then R-24.
2. **R5-TEST-OQ2 has no fallback** if neither crate can sit on a custom quinn endpoint. Handling: U3 answers it
   before any VM hour, then R-22.
3. **The macOS runner's UDP is unknown.** If condition (iii) fails, the Mac is borrowed before the test.
4. **Size, L + M, and usage-limit cuts.** Handling: small units, checkpoints after each, at most 3 agents.
5. **The stand-in sim may be lighter than the stress scene.** Handling: PM-0001-4 says STAND-IN, and S1's stand-ins
   carry the load as ADR 0004 sets it.
6. **Two marks cannot count as written:** no reordering at P3 (R-5), and the packet count's flake (R-6).
7. **ML, RD and MAC are not named,** so the proof test closes in two steps (R-14).
8. **Two operating sessions** could collide. Handling: one writer of code, a separate results branch (R-23).
9. **Hetzner's plans show "currently unavailable"** on the public pages. Mustafa's console decides on the day; a
   session waits rather than change location.
10. **Let's Encrypt's limits.** Handling: the ledger, the spare name, staging first.
11. **Leaks into the public repository.** Handling: the redaction step, the saver's secret check, review at stops B
    and C.
12. **Native and Wasm must agree for prediction to be exact.** Handling: no transcendental or platform-dependent math
    in the step; U7's native-against-Wasm hash catches a difference.
13. **Mustafa's time:** about 15 minutes of console and DNS work per session, and presence in VS1, VS4, VS8 (and VS9).

## 14. Reviewers
- **This plan:** security-auditor (the SSH key, host keys, the certificate method, firewall, cloud-init, the
  workflow's shape, raw data), engine-architect (the layout, crate boundaries, the harness the boundary spike
  shares), and the verifier (spot-checks this plan's key claims against the drafts and digests); then the advisor.
- **Stop B:** security-auditor and infra-engineer.
- **The final diff,** in rounds of at most 3:
  1. netcode-architect, security-auditor, rust-reviewer;
  2. test-engineer, engine-architect, perf-auditor;
  3. infra-engineer, then the verifier.
- **Not called:** art-director and graphics-engineer (the plain renderer is not the look); game-designer (no rules or
  balance).

## 15. Readings and choices for the advisor (none is settled until the advisor rules; the netcode and test digests' H lists hold the arithmetic and lines)
| # | Question | Recommendation |
|---|---|---|
| R-1 | PORTS (OQ-0006-16, needed before this test): the numbers in §8.1 | confirm (proposed by infra-engineer; netcode-architect has no objection on record yet, so ask it if the advisor wants both deciders' word) |
| R-2 | CERT (OQ-0006-5 development case (i)): http-01 on RS as §8.4 says | approve with security-auditor |
| R-3 | METRICS path (ADR 0006:419-421, picked by this test): scrape on TCP 9100, LG's address only | scrape (push needs a component no draft names) |
| R-4 | HOST KEYS: §1.6's options | HTTPS-published for RS; the console for LG; security-auditor picks |
| R-5 | REORDER (PM-0001-20, -58; ADR 0001:1447-1450): half-split uniform jitter never reorders a 30 Hz flow at P2 or P3 (one-way range 30 ms at P3, below the 33.33 ms spacing; at P4, 1.39 % of adjacent pairs), so both rows cannot count | an explicit L1 reorder step for those rows (swap 1 % of adjacent datagrams per direction, recorded per run beside the profile), plus a recorded P4 run; a setup change, so the advisor's reading goes in the run records |
| R-6 | PACKETS (PM-0001-3 says 100,000 per direction; PM-0002-49 says 1,000,000, because 100,000 fails a correct simulator in about 1 seed in 9 at P1) | 1,000,000 on both layers |
| R-7 | LOOPBACK: PM-0001-28's "server on the same local network" | the server on the browsers' own machine, page on localhost (a secure context); also -36's loopback leg |
| R-8 | X86 REPLAY: the Arm rule names "CI's x86-64 runner" (ADR 0001:1383 and others) | this container (x86-64 Linux, same build) or RS; no CI job |
| R-9 | L2 DELAY (PM-0001-3): "half of the profile's RTT minus the base RTT" | (profile RTT − base RTT) ÷ 2 per direction, so the round trip equals the profile; MP's 100 ± 5 ms check confirms it |
| R-10 | COVER: OQ-0001-9's cover half names this test, but no mark carries it | one recorded item under PM-0001-56: the honest-rewind runs repeated with a destructible-wall stand-in and a moving vehicle stand-in as cover, the "hit after cover" share recorded; decides nothing alone (checklist step 10 watches it) |
| R-11 | STAND-INS: do trajectory-record projectiles and moving vehicle stand-ins meet PM-0001-54's and -55's conditions? | yes; both rows run, the collision rule named in the record |
| R-12 | BITES: rows with no planted defect, and bite length | the proposals: PM-0001-3 (five, test-engineer digest F); -18 (a baseline that never falls back); -36 (the size read once at start); -55 (vehicle collision that snaps); PM-0006-11 (a header without the SMT map); -14 (padding short of 8.33 ms must come out INVALID); -26 (a rotation that closes live sessions); PM-0004-67 (an uncapped catch-up; a drop that is not counted). PM-0006-12 and -18 are readings, with no bite. Memory and queue marks (-15) bite over their own window (a full S30), not 5 minutes |
| R-13 | RUNNER LIST: Safari's count rows not on the runner's list: -32, PM-0006-25, -26, -27 | add them (the same kind as PM-0006-15); keep -29 (a time) and -36 (a home path) on a Mac |
| R-14 | CLOSE: rows judged only on named machines (§6.3) | close in two steps: this PR merges with them "pending a named machine" (the rough results reported); VS9's re-run, its own change-set, judges them; migration step 1 (ADR 0001:1689-1691) waits for step two |
| R-15 | PM-0004-67's validity, and whether its ticks share the S30 runs | V-S applies; the jitter is read from the 3 S30 runs; the 200 ms and 1 s stall runs are separate (inside a soak they would break -4's zero overruns) |
| R-16 | -59's "random tick after warm-up" | drawn within the first 5 minutes after warm-up, in its own S30 room |
| R-17 | SIMULATOR GAPS (test-engineer digest G5) | the jitter band compares the one-way delay's standard deviation with the uniform's (half-range ÷ √3); B1's burst cuts both directions; the rate limiter is drop-tail with one snapshot interval of queue, so PM-0001-24 measures the server, not the simulator; duplication 0 % in every profile; each labelled proposed |
| R-18 | PM-0006-15's "equals part 5's list" | under route (a) RS's list is exactly part 5's; only route (b) would drop SSH |
| R-19 | OQ-0006-19's reading is due before the density runs (ADR 0006:1099) | the advisor gives it at stop B, before VS5 |
| R-20 | R2-XU-OQ7: SPIKE_PLAN:197 lists it as decided here, but no rule was found | the advisor names its rule (likely among PM-0001-49's parts) or takes it off the list |
| R-21 | ADDRESSES: may the deleted VMs' addresses of the day be committed (PM-0001-1 and PM-0006-11 record them)? | yes for the VMs; never Mustafa's address; security-auditor confirms |
| R-22 | L2 FALLBACK, if U3 finds that neither crate sits on a custom quinn endpoint | decided when U3 reports: (a) the crate that allows it carries the L2 rows; (b) a user-space UDP relay in front (it changes the source address the server sees, so -25's per-address rows run without it); (c) read tc netem's documentation and propose it (R5-TEST-OQ6) |
| R-23 | TWO SESSIONS: §12.3's split | yes |
| R-24 | BROWSER PIN: a browser that cannot open WebTransport with a pinned hash | its other WebTransport rows use the browser-trusted certificate on RS's UDP port for that browser only, recorded; PM-0006-25 to -27 then fail for that browser (a finding, not a workaround); decided with security-auditor when VS1's smoke page shows it |

## 16. Queue, decisions and asks
- **Queue:** STATUS (2026-11-05) names the netcode proof test as the next v2 work. B.1 and B.2 are queued docs
  change-sets after A3; they touch `docs/EDITOR_V2.md` and `docs/VISION_V2.md`, which this change-set never touches.
  A4 waits until this test is under way (A4-FREEZE (a)). The kit freeze also covers W-PR (a `.github/` change), and
  Mustafa's approval decides it.
- **At the close:** this plan is folded into the report's appendix and the checkpoint is deleted
  (`.claude/CHECKPOINTS.md` §9).

### Decisions needed (Mustafa)
1. **ROUTE: how should the hands-on server work run, since this session cannot SSH?**
   - *Why it matters:* every VM session needs an operator, and the choice sets your time per session and where the
     SSH key lives.
   - *Options:*
     - (a) the hybrid: cloud-init builds each VM, and Claude Code on your laptop runs the sessions over SSH, with a
       key made on the laptop;
     - (b) cloud-init only: the VMs run fixed scripts and publish results over HTTPS. There is no SSH and no fixing
       in place, and LG gets a results server;
     - (c) Claude Code on your machine only, without cloud-init.
   - *Also:* which machine runs that session: the laptop (the minimum-laptop rows' browser legs run there) or your
     PC.
   - *Recommendation:* (a), on the laptop.
2. **SAFARI-TIME: when do Safari's timed rows run?**
   - *What:* the runner may stand in only for Safari's yes-or-no and count rows. The 3-second fallback (PM-0001-29),
     the datagram size on a home connection (PM-0001-36), and the certificate rows unless the advisor extends the
     runner's list (R-13), need a real Mac.
   - *Why it matters:* you planned to borrow the Mac only for the look proof test.
   - *Options:*
     - (a) borrow the Mac once during this test: one session, about an hour, at home;
     - (b) re-run those Safari rows when you borrow it for the look proof test. They stay "pending" until then, and
       nothing before the foundation waits on them.
   - *Recommendation:* (b).

### For Mustafa
1. Read this plan and answer ROUTE and SAFARI-TIME.
2. Nothing to create yet: no VM, no DNS record, no key.
3. At stop B you will get:
   - the Safari workflow PR to approve;
   - VS1's run plan with its cap;
   - the cloud-config files;
   - the exact DNS record with that day's address.
