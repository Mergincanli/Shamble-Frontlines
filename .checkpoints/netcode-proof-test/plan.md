# Plan: the netcode proof test (SPIKE_PLAN item 1), revision 2

- **Change-set:** `netcode-proof-test` · **Branch:** `claude/netcode-proof-test-plan-hcky5g`. This cloud session may push
  only to this `claude/…` branch, so it stands in for `cs/netcode-proof-test` (CLAUDE.md, standing permission).
  **Base:** main `af8393e`.
- **Date:** 2026-11-05 · **Status:** revision 2, for review round 2, then the advisor, then Mustafa. No code until the
  advisor and Mustafa approve.
- **History:** revision 1 is commit `786b7d2`. Review round 1 returned 49 findings: security-auditor REVISE (19),
  engine-architect REVISE (14), the verifier FAIL (16). Their compact copy is `review1-findings.md`; §0.1 says where
  each one is fixed.
- **Sources** (all in this folder, line-cited): `netcode-architect-digest.md` (every PM-0001 row and setup, problems
  H1–H20); `infra-engineer-digest.md` (ADR 0006's rows, Hetzner facts and prices read 2026-11-05, routes, runners,
  DNS, certificates); `test-engineer-digest.md` (L1 and L2, the harness, `spikes/` rules, the toolchain, every planted
  defect); `ssh-probe.md` (item 1). "Netcode digest H2" means section H, item 2 of that file.

## 0. In short
- **Item 1, SSH: no.** This session cannot reach any server over SSH. The image has no `ssh` client; TCP to port 22,
  and to every port but 443, times out; 443 passes an egress gateway that re-terminates TLS. It can read any public
  HTTPS site.
- **Recommended route: the hybrid.**
  - A reviewed cloud-init file (Hetzner's "Cloud config") builds every VM the same way at creation.
  - Claude Code on Mustafa's laptop does the hands-on part over SSH, with a key made on that laptop, under a reviewed
    settings file of its own.
  - This session writes all the code, proves everything that needs no VM, and judges the results.
- **What gets built:** a throwaway Rust and TypeScript spike in `spikes/netcode-proof/`. Its crate split follows ADR
  0002 part 3: a room library that builds for Wasm, a server binary, a poll-based link interface with L1 behind it and
  L2 under the real socket, the client endpoint, bots, a lab runner on simulated time, logs and a judge. The client
  harness (page, Worker host, plain renderer) is built in ADR 0002 part 5's shape, for the boundary spike to keep. No
  legacy file changes.
- **What it proves:** PM-0001-1 to -62 without the 60 Hz group (-39, -40, -63, later); PM-0004-67; PM-0006-11 to -27.
  Mustafa judges "looks correct" at P2 in Brave (100 ms round trip, ±20 ms jitter, 2 % loss, half each way) and runs
  the operations checklist. Rows that need named machines close in a short re-run later.
- **Runs:** seven runs (VM run days), each with its own run plan, hours and cost cap for his OK first. About 72
  RS-hours and up to 171 LG-hours, **EUR 12.99 planned before VAT**, caps adding up to EUR 14.50. The re-run on named
  machines adds about EUR 0.47 later. He is present in three runs; the others run unattended after bring-up.
- **Safari:** GitHub's macOS runners first (Safari 26.6.2 and 27.2 on today's images). The workflow is a `.github/`
  change, so **it goes to Mustafa for approval**, in its own small PR before the first run.
- **DNS:** one record per run, always "DNS only": type `A`, name `rs.nt`, value RS's IPv4, TTL 1 min. No AAAA. Or no
  per-run edit at all, if he keeps a fixed address (FIXED-IP).
- **Decisions:** three for Mustafa (ROUTE, SAFARI-TIME, FIXED-IP; §16). Twenty-seven readings for the advisor (§15).

### 0.1 What changed from revision 1 (each round-1 finding and where it is fixed)
| Theme | Findings | Fixed in |
|---|---|---|
| The spike-only controls get an authority: room-wide controls only through a local control socket on RS; Mustafa's own session controls with a per-run capability; INVALID if a control arrives without one; a bite | security F1 (Critical) | §10.2; E23; the catalogue |
| Firewalls chosen in the create form, before first boot; creation order LG, then RS | security F2 | §7.3 |
| Join tokens per participant, delivered out of band; a roster per room; a stray session makes a run INVALID | security F3 | §10.3 |
| Host keys: RS by HTTPS-published `known_hosts` lines from the production certificate, with DNS and CT checks; LG only through RS (ProxyJump), no SSH facing the internet; ed25519 host keys only | security F4 | §1.6 |
| The laptop session: a clone on main; Manual mode; an extra reviewed settings file; no client strings in logs; fresh browser profiles; a Windows check of the guard in R1 | security F5 | §12.3 |
| The SSH key command (`-f`, `-N`), an `ssh_config` block, removal at the close | security F6 | §1.5 |
| Page-host hardening: allow-listed files, GET and HEAD only, CSP and other headers, limits, a traversal bite | security F7 | E21; §10.4 |
| cloud-init: four users, sshd drop-in, nftables first, per-architecture pinned rustup-init, build as an unprivileged user, the user-data hash checked | security F8 | §10.5 |
| Certificate conditions: the 6-day profile required, a ledger checked against CT, revocation at teardown, port 80 open only while an order is live | security F9 | §8.4 |
| Firewall rules per IPv4 and IPv6; the exporter's bind address; outbound limits for rooms and the page host | security F10 | §8.2; §10.5 |
| Teardown: DNS first, then the VMs; a leftover-record check at each start | security F11 | §7.3 |
| Safari workflow: dispatch only from main, by Mustafa or a day session; exact host allow-list; `permissions: {}`; commit containment; the "any ref" wording | security F12; verifier F5 | §9 |
| WebSocket delay inside the server's accept path (real source addresses); a bounded per-address table; the page host and port 80 under hostile load | security F13 | E6, E14, E15; R-25 |
| Redaction enforced by a script before every push | security F14 | §10.6 |
| `cargo deny`, `cargo audit`, `npm audit` at U5, stop B and before each run | security F15 | §4.8 |
| Closed-port probes never from LG; flood rates stated; what to do on an abuse notice | security F16 | §10.7 |
| No browser weakening; a separate Brave profile | security F17 | §11; R-24 |
| The results log is not a shipped log | security F18 | §6.1 |
| 2FA on both accounts, confirmed in each run plan | security F19 | §7.3 |
| npt-room becomes a library (no I/O, async or clock; builds for Wasm); npt-server the binary; a table placing each check | engine EA-1 | §4.1; §4.4 |
| `Link` in a leaf crate, poll-based, with a Loopback backend; client adapters; npt-client never depends on npt-net | engine EA-2 | §4.1; §4.2; §4.7 |
| npt-lab: the simulated clock, the step order, the scenarios; `due_ticks` as a pure function tested on a fake clock | engine EA-3 | §4.1; §4.7 |
| The Worker host's shape written out; npt-client-wasm holds the bindings only | engine EA-4 | E7; §4.7 |
| "Left to the boundary spike" | engine EA-5 | §2 |
| The isolation flag is off by default, on only for PM-0001-43 and -44 | engine EA-6 | E21 |
| The determinism rules and how they are enforced | engine EA-7 | §4.5 |
| The native-against-Wasm hash: method, inputs, pass mark, bites; clippy for wasm32 from U1 | engine EA-8 | §4.5; §12.1 |
| One sim seam and one tick-rate constant, so the 60 Hz run swaps the sim and nothing else | engine EA-9 | §4.6 |
| "Kept" defined | engine EA-10 | §4.9 |
| npt-log; the replay tool moved to npt-lab | engine EA-11 | §4.1 |
| The symbols per crate; how a bite is selected | engine EA-12 | §4.7 |
| npt-super and npt-host as separate crates | engine EA-13 | §4.1; §4.2 |
| E1 and E2 wording; the panic modes | engine EA-14 | E1, E2 |
| PM-0006-14 gets three sessions per N, at three times of day | verifier F1 | §7.2 |
| Hours with their working; totals restated | verifier F2 | §7.2 |
| PM-0001-32 into the human run; -14 and -16 in every run they cover | verifier F3 | §6.2 |
| PM-0001-36's laptop legs first, in R1; size rows re-run at a lower cap | verifier F4 | §6.2; §7.2 |
| R-18 split into two readings | verifier F6 | R-18 |
| R-1's numbers are this plan's; both deciders asked in round 2 | verifier F7 | R-1 |
| R-5 restated per stream | verifier F8 | R-5 |
| Readings that change a number or setup go into the drafts first (R-PR) | verifier F9 | §3; §12.2; R-27 |
| L2's probe rate and minutes | verifier F10 | R-6; §7.2 |
| §6.3 additions; condition (ii); the runner checked first in R1 | verifier F11 | §6.3; §9 |
| `fsn1` missing from today's public price pages | verifier F12 | §13 |
| Density sessions for one N at least 6 h apart, pairwise | verifier F13 | §7.2 |
| ICMP from the laptop's and the PC's addresses | verifier F14 | §8.2 |
| ROUTE asks the laptop's OS, browsers and Claude Code | verifier F15 | §16 |
| Four accuracy points | verifier F16 | §1.5; §11; §12.1; R-12 |

## 1. Item 1: can this session reach the servers over SSH?
### 1.1 What was tested (2026-11-05, 21:30–21:45 UTC; each probe once; `ssh-probe.md`)
| Probe | Result |
|---|---|
| `ssh`, `ssh-keygen`, `scp`, `sftp` in the image | all four missing |
| TCP to `github.com:22`, and to `1.1.1.1:53` (a non-443 port elsewhere) | both time out |
| Port 22 through the agent proxy (`curl -p -x $HTTPS_PROXY telnet://github.com:22`) | times out, no tunnel |
| TCP to `github.com:443` | connects, but the certificate is the egress gateway's: TLS is re-terminated, so SSH carried on 443 cannot pass |
| HTTPS to unrelated public hosts (nasa.gov, ikea.com, bbc.co.uk, docs.hetzner.com, Let's Encrypt's ACME directory) | 200 |
| `shamblefrontlines.com` over DNS-over-HTTPS | on Cloudflare (NS `cora` and `steven.ns.cloudflare.com`); no A, AAAA or CAA record yet |

The proxy's own notes list "non-443 HTTPS ports, raw-TCP …" and "WebSocket upgrades" as not supported: "report, do
not work around". **So: no.** Port 22 is blocked to every host, so a probe of a real VM adds nothing. If Mustafa wants
it on record, R1 repeats the probe against RS.

### 1.2 What follows
- This session cannot SSH to a VM. It cannot be a WebSocket or WebTransport client of RS (the proxy refuses WebSocket
  upgrades, and QUIC is UDP). It cannot reach a port other than 443.
- It can build and test all the Rust and TypeScript here (Rust 1.99.1 installs: `static.rust-lang.org` answers). It
  can run every L1 row on simulated time, push to GitHub, start and read GitHub Actions runs, and read public HTTPS,
  DNS-over-HTTPS and Certificate Transparency logs included.
- Whatever the server route, the browser legs on the minimum-laptop stand-in need scripts run on Mustafa's own
  machine (infra digest E).

### 1.3 The routes
| | (a) Hybrid (recommended) | (b) Cloud-init only | (c) Claude Code on his machine only |
|---|---|---|---|
| Provisioning | a cloud-init file from the repo at the run's commit (§10.5) | the same | the operator session over SSH, step by step |
| Who runs a run | Claude Code on his laptop over SSH. It starts each block's run script on RS and may disconnect while a server-only block runs | a fixed run script from the cloud-config; nobody can look inside a running VM | the operator session |
| Results to this session | the laptop session pushes redacted summaries to its own branch `claude/npt-results`; this session reads them with `git fetch` and `git show` | over HTTPS from a results server on LG: new surface, its own name and certificate | as (a) |
| Mustafa, per run | OK the cap; create LG, then RS, with firewall and cloud-config chosen in the create form; DNS; start the laptop session; delete DNS, then the VMs | the same, but every failed start costs a new VM | as (a), with a slower bring-up |
| A failed run | diagnosed in place | a new VM | diagnosed in place |
| Security | SSH by key, from one address, to RS only (LG through RS) | no SSH at all, but a results web server on LG | as (a) |
| Fit with the drafts | yes | yes, but PM-0006-15's list lacks SSH, and the browser legs still need someone at the laptop | yes |

### 1.4 Recommendation
(a), with the operator session on Mustafa's **laptop**. The laptop is the minimum-laptop stand-in, so its browser
legs run there anyway (SPIKE_PLAN:59-60). His PC stays for what he judges in Brave (MP: SPIKE_PLAN:59-63).
- Server-only blocks run unattended: the laptop session starts the run script on RS, which drives LG over the
  internal path, and comes back to collect.
- ROUTE (§16) also asks three facts the plan assumes but no source states (verifier F15): the laptop's operating
  system, its browsers, and whether Claude Code is installed on it.

### 1.5 The SSH key (SPIKE_PLAN:153-156)
- **This session cannot give a key that works:** it cannot connect. Under (a), the laptop's Claude Code session makes
  the pair there, so the session holding the private key is the one that made it:
  `ssh-keygen -t ed25519 -f ~/.ssh/npt_operator -N "" -C npt-operator`
  - The key gets its own file, so it never overwrites `id_ed25519`, his default identity.
  - It has no passphrase, because it opens only the spike project's short-lived VMs, from one source address.
    Alternatively Mustafa makes a passphrase key and loads it into `ssh-agent` himself.
- The session prints only the `.pub` file and the fingerprint, and never opens the private file (the operator
  settings deny reading it, §12.3). If the private key is ever shown in a transcript, a new pair is made.
- One `ssh_config` block holds the setup: `IdentityFile ~/.ssh/npt_operator`, `IdentitiesOnly yes`, `ForwardAgent no`,
  a per-run `UserKnownHostsFile`, and `HostKeyAlias` per role.
- Mustafa adds the public half once to the spike's Hetzner project (Security, SSH keys) and picks it at each creation.
  Through the Hetzner Console a key can be added only at creation (infra digest D).
- At the close, the public key is removed from the project and the key files are deleted.
- SSH reaches **RS only**, from the laptop's public IPv4 of the day (/32). LG is reached through RS (§1.6).
- The cloud-init file gives the public key to the login user `npt` only, and turns off root and password login
  (§10.5). A public key is not a secret, so it may sit in the repository.

### 1.6 Checking each new VM's host keys (SPIKE_PLAN:65-66; security-auditor's method, R-4)
- **Host key type:** cloud-init makes ed25519 host keys only (`ssh_genkeytypes: [ed25519]`).
- **RS, before the first SSH:**
  1. cloud-init issues the production certificate (a staging dry run comes first, in R1 only).
  2. It then publishes the full `known_hosts` line: `https://rs.nt.shamblefrontlines.com:31000/npt/ssh-host-keys.txt`,
     root-owned, mode 0444, written after the SSH module ran.
  3. The laptop session checks over DNS-over-HTTPS that `rs.nt`'s A record equals the IPv4 shown in the Hetzner
     console.
  4. It fetches the file with normal TLS verification, and checks that the certificate served is today's: issued
     after RS was created, and present in CT.
  5. It connects to that IPv4 with `HostKeyAlias`, `StrictHostKeyChecking=yes` and the per-run `known_hosts` file.
- **Fallback, if issuance fails:** Mustafa opens RS's console in Hetzner and compares the full fingerprint string,
  which cloud-init also writes on the login screen and the laptop prints in groups of 4.
- **LG:** no DNS name and no console check. LG's SSH rule admits only RS's IPv4. The laptop reaches LG by `ProxyJump`
  through the verified RS, whose sshd allows forwarding only to LG's port 22 (`PermitOpen`). So LG has no SSH facing
  the internet, and its key is learned over Hetzner's internal path.
- **Out:** trust on first use alone (`accept-new`), and host keys made off the VM and passed in through user data
  (that would put a secret in the paste, and every process on the VM can read user data).

## 2. Goal and scope
- **Goal:** prove the online path that ADR 0001's "What a spike must prove" sets out (ADR 0001:1237-1335):
  - the Rust sim native on the server and as Wasm in the client;
  - WebTransport with the WebSocket fallback on four browsers;
  - 48 simulated clients in the stress scene at 30 Hz, within the budgets;
  - one human client at P2 seeing prediction, reconciliation and lag-compensated hits that look correct.
  It feeds the questions this test decides (SPIKE_PLAN:196-204), and leaves the server and harness for the 60 Hz run
  and the boundary spike.
- **Why a player cares:** online play that feels local at a 100 ms round trip. Mustafa judges that part.
- **In scope:**
  - the marks in §6;
  - the message catalogue in `docs/design/`, with every control's channel and authority;
  - the readings PR (R-PR, R-27);
  - the results written back into ADR 0001 at the close: the crate, the encoding, the process model, the datagram cap
    and the rewind cap (ADR 0001:1689-1691). That is an ADR edit, so it needs Mustafa's go.
- **Out of scope:**
  - the 60 Hz group (PM-0001-39, -40, -63; SPIKE_PLAN:206-248);
  - the look;
  - a lobby, real join tokens, reconnecting, private rooms and economy (ADR 0001:1613-1618);
  - a CI job for `spikes/` (R-8);
  - any legacy file.
- **Left to the boundary spike** (engine EA-5; SPIKE_PLAN:252-279; ADR 0002:1149-1175). The harness must not pre-empt
  any of these:
  - its marks: PM-0002-*, PM-0004-8, -28, -61, -68 to -70. This test only leaves its server re-runnable at P2 for
    -61's and -70's online halves;
  - generated types: TypeScript types for the block, the controls and the messages are hand-written here, with one
    layout-version constant checked at start, and no ts-rs or layout generator (its item 4);
  - B2 only: no B3, no SharedArrayBuffer, no counting build;
  - one Wasm instance, online mode only. Local mode and OQ-0002-3 are its own; the room library stays Wasm-buildable
    for it (§4.1);
  - Worker pacing: built to ADR 0004 part 5's rule, not judged;
  - the panic-hook buffer read after a trap (PM-0004-61). Here the harness only restarts the Worker and rejoins after
    a trap (PM-0001-19);
  - no measurement-harness work beyond E7's per-frame logs (SPIKE_PLAN:277-279).
- **Not changed:** `shamble-*.js`, the two pages, `tools/`, `.claude/`, `CLAUDE.md`. `.github/` changes only through the
  Safari workflow PR (W-PR, §9), with Mustafa's approval.

## 3. Rules this plan keeps
- SPIKE-SCOPE (a): throwaway, in `spikes/`, exempt from the legacy hard constraints. One heavy spike at a time. Server
  marks at 30 Hz, N = 48.
- **Marks change only in the drafts** (ADR 0001:1603-1604): a mark, threshold or setup changes only there, by the
  advisor or Mustafa. A reading that changes a number or a setup (§15) reaches the drafts through R-PR, merged on
  Mustafa's go before the first counted run under it (R-27). Debug runs may come earlier; they count for nothing.
  `marks.json` holds each value with its draft line.
- **Bites:** every automated mark has a planted defect, and it must fail first, before its pass counts
  (ADR 0001:1499-1504).
- **No re-runs to green:** a failing run is kept and its cause found (ADR 0001:1505-1506; 0006:1375-1376).
- **Named machines:** rows on ML, RD or MAC are judged only on a named machine. Until then they are rough checks,
  reported and never judged, and they are re-run once the machine is named (SPIKE_PLAN:156-160; ADR 0001:1406-1408;
  §6.3).
- **The Arm rule:** a mismatch seen only on LG's aarch64 clients decides nothing until the same inputs replayed on
  x86-64 show it too (ADR 0001:1337-1346).
- **Public repository:** no secret, token, capability, raw payload or home address in a commit or a note (§10.6).
- **Checkpoints** as `.claude/CHECKPOINTS.md` sets them: at most 3 agents at a time, notes saved as they go, other
  branches only in a separate worktree.

## 4. What gets built
### 4.1 Layout (all new; no existing file changes)
```
spikes/netcode-proof/
  README.md, rust-toolchain.toml ("1.99.1"; clippy, rustfmt; target wasm32-unknown-unknown)
  Cargo.toml, Cargo.lock, deny.toml     one workspace, resolver 3, edition 2024, every build --locked
  crates/link/      npt-link: the thin transport interface (poll-based) and its Loopback backend; no dependencies
  crates/l1/        npt-l1: L1 on simulated time (implements Link): profiles, fate log, per-stream reorder step
  crates/sim/       npt-sim: the stand-in sim behind the seam (§4.6); no I/O, clock, transport or bot logic
  crates/proto/     npt-proto: messages, controls, quantisation, delta codec, limits
  crates/room/      npt-room: the room library: slots and roster, command checks, input buffer (both OQ-0001-6
                    options), interest, hit history and rewind, per-client encoding, event queue; tick-counted
                    timers only; no I/O, async runtime or clock; builds for wasm32 and native
  crates/server/    npt-server: the room process binary: the S17 loop, the tick thread, the I/O runtime, the
                    wall-time limits, the local control socket, metrics, the panic catch
  crates/net/       npt-net: native adapters only: WtServer and WtClient (wtransport, web-transport-quinn: one
                    feature each), WsServer and WsClient; L2 (ImpairSocket under quinn; the delaying stream wrapper
                    in the WebSocket accept path)
  crates/client/    npt-client: the client endpoint library: no bindings, transport or clock; native and wasm32
  crates/client-wasm/  npt-client-wasm: the wasm-bindgen exports only
  crates/bots/      npt-bots (headless, cheating and hostile clients) and npt-load (the load orchestrator on LG)
  crates/super/     npt-super: the supervisor stand-in (process control, exit log, SIGKILL)
  crates/host/      npt-host: the page host and the ACME client
  crates/log/       npt-log: results-log record types and the JSON Lines writer (serde only)
  crates/judge/     npt-judge: npt-log + marks.json -> one verdict per mark
  crates/lab/       npt-lab: the L1 scenario runner (simulated clock, step order, 48 headless clients, seeds),
                    the x86-64 re-decode and replay tool, the native-against-Wasm hash comparer
  fuzz/             cargo-fuzz targets, one per message type
  web/              the client harness: TypeScript, Vite, three.js (exact pins): main.ts, host.ts, net.ts,
                    block.ts, render.ts, overlay.ts, record.ts, smoke.ts, connect.ts; wdio/*.spec.ts
  marks/marks.json  every mark as data: id, statistic, threshold, window, validity, the draft line
  ops/              cloud-init/ (rs.yaml, lg.yaml), firewall/ (rs.txt, lg.txt, each rule per IPv4/IPv6),
                    runs/ (R1.md … R7.md), operator/ (settings.json, rules.md, PROMPT-Rn.md, redact-check),
                    checklists/ (netcode.md, operations.md)
  results/          committed verdicts and redacted summaries per run, never raw data
docs/design/netcode-proof-messages.md   the message catalogue (ADR 0001:654-678's subset + controls)
```
Builds, raw logs and captures go under `.gate/netcode-proof-test/` (gitignored). `crates/server/` and `crates/net/`
fall under `.claude/rules/netcode.md`.

### 4.2 Dependency rules (enforced by the spike's test command: `cargo tree` checks and one grep)
- **Leaf crates:** npt-link, npt-l1, npt-sim, npt-proto, npt-room, npt-client and npt-log depend on no tokio, quinn,
  wtransport, web-transport-quinn or tungstenite. Each passes
  `cargo check --locked --target wasm32-unknown-unknown`. Clock reads are banned in them by clippy (§4.5).
- **The client:** npt-client never depends on npt-net; the bots get transports by composing npt-client with npt-net.
- **npt-super** depends on no npt-sim, npt-room or npt-proto. **npt-host** depends on none of the sim, room, proto or
  client crates.
- **npt-judge** depends only on npt-log, plus serde. Nothing depends on npt-lab.
- **No reference into `spikes/`** from outside it: a grep in the test command, until the foundation's G5 and G8 exist.
- **Narrow public surfaces:** each crate's public items are those in §4.7; everything else is `pub(crate)`.

### 4.3 Components, each traced to the marks that need it (netcode digest E gives each one's design part)
| # | Component | Crate | Marks |
|---|---|---|---|
| E1 | Stand-in sim, native and Wasm: movement and hitscan; its soldiers driven by protocol clients (npt-bots), with no bot logic in npt-sim; the stress scene's counts (48 soldiers in 3 × 16; 15 big vehicles, 4 of at least 20 parts; 300 live projectiles as trajectory records through one shared function; S1's stand-ins as data behind the seam, §4.6); a DM_Tri-sized greybox with enterable buildings, areas, portals, doors at 1/s; the target and correction courses; the authority-only spread stream; a per-tick state hash | sim | -2, -4, -5, -17, -30, -41 to -62 |
| E2 | The room process: the tick thread alone on one vCPU, async I/O on the others, bounded channels only; `panic = "unwind"` for npt-server's builds (the Wasm build keeps the target's abort), a top-level catch → RoomEnded(crash), a crash record, a non-zero exit; per-phase timers and counters; the S17 loop through `due_ticks` (§4.6) | server | -1, -4 to -11, -15, -16, -26, -59; PM-0004-67 |
| E3 | Commands: Input datagrams (tick, sequence, move, buttons, press counters, aim ≤ 80 bits, view time, newest-decoded ack), redundancy up to 8, U2 delta repeats; the checks of §4.4; the input buffer; **both OQ-0001-6 options, switchable**; the late-command counter | proto, room | -13, -20, -21, -30, -49, -56, -58 |
| E4 | Snapshots: delta against the last acked (`AckRing`), E3 quantisation, entity groups that each decode alone, the datagram cap (1,000 B until -36 measures), FullState on its own stream, Resync, the bounded event queue, the per-client 256 kbps cap, newest-only over WebSocket | proto, room | -6, -8, -12, -14, -17, -18, -19, -22, -23, -31, -57, -58, -62 |
| E5 | Interest: the region circle with hysteresis on a grid; areas and portals flood-numbered and re-flooded on change; the priority accumulator | room | -8, -9, -10, -12 |
| E6 | Transports under `Link`: WebTransport on quinn 0.11 through wtransport 0.7.x and web-transport-quinn 0.13.x, swappable; WebSocket through tokio-tungstenite 0.30.x; the wall-time limits of §4.4; the per-address table bounded (fixed capacity, eviction, a counter, tested at 100,000 synthetic addresses); `wss` only; NewReno or Cubic; the version floors; refused handshakes counted per reason | net, server | -11, -14, -24, -25, -28 to -38 |
| E7 | The client harness, in ADR 0002 part 5's shape (ADR 0002:313-373), for the boundary spike to keep: see the list below this table | client, client-wasm, web | -19, -28, -29, -31 to -33, -36, -41 to -46, -51 to -53, -61, -62 |
| E8 | Prediction and reconciliation (replay newer commands, 100 ms fade, 2 m snap, cues keyed once) | client | -41 to -44, -60, -62 |
| E9 | Interpolation: buffer in the Worker; delay from 3 intervals, adapting between 2 and 5; extrapolation at most one interval, then hold; placed respawns; events released with their poses | client | -45, -46, -61 |
| E10 | Clock: Ping/Pong with hold time, answered on the I/O runtime; samples stamped at datagram arrival by the host | client, server | -33, -47, -50 |
| E11 | Lag compensation: `HitHistory` (at most 1 MB), `rewind_at`, the view-time tolerance, the switchable cap (a test cap above every honest rewind, then the rule cap), a per-shot log of its parts | room | -7, -48 to -51 |
| E12 | Headless bots on npt-client with npt-net's clients: 30 inputs per second, the brawl, the target-course shooter, the cheats (forged kinds, 1.5× clock, lag switch, a 150 ms pong delayer, the tolerance-edge stamper, the never-acker), raw byte logs for re-decoding (kept on LG) | bots | -2, -17, -19 to -22, -48, -50, -54 |
| E13 | Load orchestration: clients over the CAX21s; the 5-minute calibration (under 80 % CPU every second); inputs per second within 1 %; the metrics role on one CAX21 | bots | validity of every LG row; -25 |
| E14 | Hostile clients and fuzzing: 1,000 sessions that never join, oversize datagrams and stream openers, slow-loris on the game ports and on TCP 31000 and 80, a 1,000-address /64 cycler, `ws://` and wrong-Origin pages; cargo-fuzz per message type with a counting allocator | bots, fuzz | -25, -34 |
| E15 | The link simulator: L1 (npt-l1: seeded, simulated time, fate log, the per-stream reorder step of R-5); L2 (npt-net: impairment under quinn's socket, per peer; the WebSocket delay as a delaying stream wrapper inside npt-server's accept path, so the room sees real source addresses (R-25); rate limit; bottleneck; per-session profile; B1; UDP block); every run records each stream's reorder share | l1, net | -3 and every P1–P4/B1 row |
| E16 | Density harness: 8 room processes unpinned under the supervisor, bracket B, SIGKILL, 384 sessions | super, server, bots | -27; PM-0006-13, -14 |
| E17 | Recordings and hashes: input recordings that drive both transports; the server's per-tick state hash; the client-held hash, without authority-only streams; the native-against-Wasm comparison (§4.5); the x86-64 re-decode and replay (npt-lab) | sim, lab | -17, -19, -21, -30, -48, -54, -57, -58 |
| E18 | Instrumentation: monotonic reads at tick and phase edges; clock resolution per machine; per-thread CPU and RSS each second; queue lengths; bytes per client counted at UDP level (payload + 28 B, from the socket wrapper); packet capture for fragments; steal and affinity reads | server, net | -1, -4 to -16, -24, -25 |
| E19 | Bites: one planted defect per automated mark (test-engineer digest F, plus R-12), compiled only into the `bites` build and chosen at run time (§4.7) | all | every automated row |
| E20 | Results: npt-log's JSON Lines (a header per machine, then per-tick, per-second and event records), apart from the tracing logs; npt-judge | log, judge | all |
| E21 | Hosting and certificates: npt-host serves only allow-listed files (§10.4), with **no COOP/COEP by default**; a separate path, `/iso/`, adds the two headers for the rows that run with the isolation flag on (PM-0001-43 and -44: ADR 0001:1479-1481, 1562-1563), and every browser run records `crossOriginIsolated` as the Worker reads it; the WebTransport development certificate (§8.5); the `wss` certificate by ACME (§8.4) | host, server | -25, -28 to -32, -36, -43, -44, -53; PM-0006-15, -25 to -27 |
| E22 | Metrics and logs: `metrics` with `metrics-exporter-prometheus` on RS, bound to RS's public IPv4, scraped by Prometheus on LG; bounded labels; `tracing` JSON lines with no client-supplied string verbatim (a length or hash, or escaped and cut to 64 B); the canaries of PM-0006-21 | server | PM-0006-16, -19 to -24 |
| E23 | The control plane (security F1): room-wide controls (rewind cap, per-address cap raise, panic and stall injections, roster) only through a local Unix socket on RS (mode 0600, owner `npt`); per-session controls for one browser session (profile, B1, UDP block, the record form) only with a per-run capability (§10.2) | server, room, web | -25, -26, -45, -46, -48, -49, -53, -59; PM-0004-67; PM-0006-23 |

**E7, the Worker host's shape** (engine EA-4; ADR 0002:313-373). Built now for the boundary spike to measure; none of
PM-0002-* is judged here.
- **npt-client** (no bindings, no transport, no clock):
  - `ClientEndpoint::input(page_input, datagrams: &[(bytes, arrival_ms)], prev_send_ms, now_ms)`;
  - `ClientEndpoint::step() -> StepOut { datagrams, block, next_due_ms }`;
  - `AimCast`: the aim cast against the state published for the page's view time;
  - `BlockWriter`, with one layout table.
- **npt-client-wasm:** the wasm-bindgen exports only: create, mode, input, step.
- **web:**
  - `net.ts`: the Worker's WebTransport first, with a 1.5 s timeout, then WebSocket. It uses `requireUnreliable`,
    reads `maxDatagramSize` every 10 s, and stamps `performance.now()` in each datagram's own event;
  - `host.ts`: the timer set from `next_due_ms`, at most 7 ticks per wake (from the tick constant, §4.6), a fresh
    `memory.buffer` view after each call, and a Worker restart plus rejoin after a trap;
  - `block.ts`: the page's typed-array views, keeping the two newest blocks;
  - `main.ts`: the page posts the newest input, the cursor ray and the view time once per drawn frame;
  - `render.ts`: the plain renderer on both backends (three.js's WebGPU renderer and its WebGL 2 path; not the look);
  - `overlay.ts` and `record.ts`: the overlay and the record form;
  - `smoke.ts`: the smoke page; `connect.ts`: the 100-session connect page (-28).
- **Design rules:** no per-entity call, at most 4 Wasm calls per tick, and the page never touches a packet. The
  boundary spike counts them later; they are not judged here.

### 4.4 Where each check lives (engine EA-1; ADR 0002:189, 457-459)
| In npt-room (counted in ticks; no clock) | In npt-server (wall time, from an injected `now`) |
|---|---|
| strictly newer tick; the lead bound; the catch-up cap (within the redundancy cap); the clamp beyond reach; input buffer depth and the late-command counter; the event queue's cap with resync or drop; the per-client 256 kbps cap per tick window; interest hysteresis; the rewind cap in ticks; the roster (§10.3) | the Join deadline (2 s from TCP accept or the first QUIC packet); the handshake cap; the bounded per-address table (/32 and /64); slow-loris deadlines; size limits at the socket (datagram, stream frame, WebSocket message); the Origin check; the capability check of per-session controls; the S17 loop; the control socket; the panic catch |

L1 runs npt-room and npt-server's limit code on its simulated clock, because the clock is always injected. So the L1
hostile rows (-21, -22) run on simulated time.

### 4.5 Determinism rules for npt-sim, and the native-against-Wasm check (engine EA-7, EA-8; ADR 0004 parts 4 and 7-9; ADR 0002 parts 2 and 7; GLOSSARY §7)
- **Floats:** ADR 0004's allow-list only, with its own min, max and clamp helpers; `rem_euclid`, `div_euclid` and `%`
  are off the list; transcendentals through `libm` only. NaN never enters state: finite checks at entry, and debug
  asserts each tick.
- **Bans (clippy):** the hash-map ban (`disallowed-types` plus `iter_over_hash_type`), in a `crates/sim/clippy.toml`
  copied from ADR 0002 part 7. The clock ban covers npt-sim, npt-proto, npt-room, npt-client and npt-l1.
- **Build flags:** baseline x86-64; no `+fma`, no `target-cpu=native`, no relaxed-simd; simd128 off; wasm-opt without
  `--fast-math`. `overflow-checks = true` for npt-sim and npt-proto in every profile, with `wrapping_` and `checked_`
  counters (the S30 soak starts the tick counter 1,800 below 65,536).
- **Features:** one feature set for npt-sim in the server and Wasm builds, compared with `cargo tree -e features`.
- **Randomness:** `ChaCha8Rng`, pinned; the seed passed in by the host; our own sampling functions; the authority-only
  spread stream kept apart.
- **Clippy and the wasm32 check run at every unit, from U1:** host and wasm32 for npt-sim, npt-proto, npt-client and
  npt-room.
- **The hash:**
  - computed by canonical serialization: field by field, fixed-width little-endian, no `usize`, floats by `to_bits`, a
    fixed unseeded hash, never std's `Hasher`;
  - spike-local: the sim core spike picks the real hash function (ADR 0004:350-351);
  - the client-held hash leaves out the authority-only streams.
- **The check (from U4):**
  - inputs: the 3 input recordings (3 × 5 min = 27,000 ticks);
  - each run natively twice in separate processes, once as Wasm in Node, and once in local Chromium (a rough check);
  - the report names the first mismatching tick, with a field diff;
  - pass mark: 0 mismatching ticks;
  - bites: one std `sin` and one hash-map iteration, each caught at the first tick it changes.

### 4.6 The sim seam and the tick constant (engine EA-9; SPIKE_PLAN:226-231; ADR 0004:185-188, 207-209)
- **One narrow seam** between npt-room and the sim. It is one trait with one implementation, not a plugin system:
  `step(commands)`, `state_hash`, the snapshot-field read-out, the hit-volume read-out for lag compensation, and spawn
  and teleport.
- **S1's stand-ins** sit behind the seam, with the top-up as data: 2.80 ms at 30 Hz, 0.30 ms at 60 Hz.
- **One tick constant:** `TICK_HZ` is one build-time constant. dt, the catch-up cap ⌊0.25 × TICK_HZ⌋ (7 at 30 Hz, 15
  at 60 Hz), the snapshot interval and every tick-counted timer derive from it; no literal 7 anywhere.
- **The S17 policy** is a pure function, `due_ticks(now, schedule) -> (to_run, dropped)`. It is unit-tested in this
  container on a fake clock, with PM-0004-67's bites (timer-period counting, an uncapped catch-up, an uncounted drop),
  before any VM hour.
- **The 60 Hz run's own change-set** replaces npt-sim through this seam and changes no other crate.

### 4.7 Symbols to add (each crate's whole public surface; nothing is removed or renamed anywhere)
- **npt-link:** `Link` (poll-based: `send_unreliable(bytes, now)`, `send_reliable(channel, bytes, now)`,
  `poll_recv(now) -> Option<(bytes, arrival)>`, `state`, `rtt`), `Listener`, `Channel`, `Loopback`.
- **npt-l1:** `L1Net`, `Profile`, `FateLog`, `ReorderStep`.
- **npt-sim:** `World` (implements the seam), `Scene::stress`, `Greybox`, `Areas`, `Portals`, `SimRng`,
  `trajectory_at`, `StandIns`.
- **npt-proto:**
  - the messages: `Input`, `Snapshot`, `FullState`, `Resync`, `Ping`, `Pong`, `Join`, `JoinAccepted`, `JoinRejected`,
    `ClientSettings`, `RoomEnded`, `Leave`, `Kick`;
  - the session controls (§10.2): `SessionControl`, with `SetProfile`, `TriggerBurst`, `BlockUdp` and `RecordStep`;
  - the rest: `encode`, `decode`, `Quant`, `Limits`.
- **npt-room:** `SimCore` (the seam trait), `Room`, `Roster`, `ClientSlot`, `InputBuffer` (option A or B),
  `EventQueue`, `AckRing`, `Encoder`, `InterestGrid`, `AreaFlood`, `Priority`, `HitHistory`, `rewind_at`.
- **npt-server:** `due_ticks`, `Schedule`, `TickLoop`, `PhaseTimers`, `Counters`, `ControlSocket`, `ConnLimits`,
  `AddrTable`.
- **npt-net:** `WtServer`, `WtClient`, `WsServer`, `WsClient` (features `wt-wtransport` and `wt-wtq`),
  `ImpairSocket`, `DelayStream`.
- **npt-client:** `ClientEndpoint`, `StepOut`, `Predictor`, `Interp`, `ClockSync`, `AimCast`, `BlockWriter`, `CueLog`.
- **npt-client-wasm:** the exports create, mode, input and step.
- **Binaries:** npt-bots, npt-load, npt-super, npt-host, npt-judge.
- **npt-log:** `Header`, `TickRecord`, `SecondRecord`, `EventRecord`, `Writer`.
- **npt-lab:** `SimClock` (integer ns), `L1Scenario` (each instant: due packets in fate-log order, then the server
  tick if due, then each client in slot order), `ReplayX86`, `HashCompare`.
- **web:** the files in E7; `wdio/*.spec.ts`.
- **Bites:** compiled only into the `bites` feature build, and chosen at run time by `--bite <id>`. The measured
  builds never contain them. Each run's header records the bite id, the enabled features and the binary's SHA-256.
- **Legacy:** the gate's symbol conservation finds no change.

### 4.8 Toolchain, pins and audits
- **Rust:** 1.99.1, today's stable (2026-10-13) and the foundation's pin. This container has 1.97.0, so U0 installs
  1.99.1 and the wasm32 target and records the result. `Cargo.lock` is committed; every build and test runs
  `--locked`. On the VMs: `cargo fetch --locked`, then `cargo build --locked --offline`.
- **Floors** (PM-0001-35): quinn-proto ≥ 0.11.18, rustls ≥ 0.23.46, ring ≥ 0.17.12, tokio ≥ 1.54.1; wtransport's
  dangerous configuration stays off.
- **Fuzzing:** cargo-fuzz on a dated nightly, recorded.
- **TypeScript side:** Node 22; TypeScript 6.0.x, Vite, WebdriverIO and three.js, each exact. three.js is the newest
  release on the day, ADR 0003 part 2's rule. Installs run `npm ci --ignore-scripts`.
- **Audits (security F15), at U5, at stop B and before every run:**
  - `cargo deny check advisories bans sources`: crates.io only, no git dependencies;
  - `cargo audit`, with the database date recorded;
  - `npm audit` on `web/`'s lockfile.
  An advisory on a network-facing crate (quinn, rustls, ring, h3, tungstenite, hyper, tokio) blocks the next run
  until it is handled.

### 4.9 Kept (engine EA-10; ADR 0002:140-146, 1422-1434)
- **What is kept:** the whole `spikes/netcode-proof/` merges to main, with the closing commit's SHA written into
  ADR 0001's results.
- **Who starts from it:** the 60 Hz run and the boundary spike each start from that SHA, in their own change-sets,
  and record their own SHAs.
- **Nothing outside `spikes/` references it** (the grep of §4.2). The foundation writes its crates fresh and reads
  the spike only as a reference.
- **When it is deleted:** once no spike in SPIKE_PLAN still names it. Today that is after the render budget bench,
  which still uses the boundary spike's Worker built on this harness. The advisor confirms the deleting change-set
  when it comes.

## 5. Where each part runs
| Place | What runs there |
|---|---|
| This container (x86-64 Linux) | every build, unit and property test; every L1 row (simulated time, logic marks only: ADR 0001:1410-1411); fuzzing (-34) and the dependency checks (-35); the audits (§4.8); both transports and L2 on loopback; the native-against-Wasm hash in Node and local Chromium; the Arm rule's x86-64 replays (R-8); judging every result; DNS-over-HTTPS and CT checks |
| RS (CCX23, 4 dedicated vCPU, x86-64) | room processes, supervisor, page host, L2, metrics exporter; every timed server row on one vCPU |
| LG (CAX21s, Arm64; 1 to 4 by calibration) | bot clients (aarch64), the load orchestrator, Prometheus (the metrics role), the ping target |
| Mustafa's laptop (the ML stand-in) | the operator session (route (a)); the Chrome, Edge and Firefox legs as rough checks, in fresh temporary browser profiles |
| Mustafa's PC (MP) | Brave in a separate profile: the smoke check and his netcode checklist; his ping |
| GitHub macOS runner | Safari's yes-or-no and count rows, once conditions (i) to (iii) are recorded (§9) |
| A borrowed Mac (SAFARI-TIME) | Safari's timed rows, and any row the runner may not stand in for |
| The named ML and RD, later | the short re-run (R8, its own change-set; R-14) |

## 6. Evidence: how every mark gets its verdict
### 6.1 Verdict rules
- **The checker:** `marks.json` holds every mark as data, with its draft line. `npt-judge` reads each run's npt-log
  results log (a header per machine: build commit, binary hashes, features, bite id, profile, seed, clock resolution,
  affinities; then per-tick, per-second and event records) and writes one verdict per mark:
  - PASS or FAIL;
  - INVALID, when a validity rule failed: neither a pass nor a fail (ADR 0001:1497-1498);
  - STAND-IN, for PM-0001-4 if the sim is lighter than the stress scene;
  - REPORTED, for a row with no pass number;
  - NOT-RUN.
  Percentiles are by nearest rank, ⌈0.99 n⌉.
- **Two more INVALID rules (security F1, F3):** a measured run is INVALID if it admitted any session outside its
  roster, or if any control arrived without its capability. Every control and form record carries its source.
- **The results log is not a shipped log** (security F18). It is written apart from the tracing output, so
  PM-0006-21's grep runs over the tracing logs, and the seed in the results header does not breach it.
- **Fixtures:** the checker is tested on fixed fixtures that must come out PASS, FAIL and INVALID.
- **Bites first:** every automated mark's planted defect must FAIL before its pass counts.
- **No retries:** a failing run keeps its data, and its cause is found first.

### 6.2 Every mark: where and when
Key:
- **C** = this container (L1: 3 seeds per profile; any-machine rows);
- **Rn** = a run (§7.2), and its block where it matters;
- **ML\*** = the laptop stand-in (a rough check: reported, never judged; re-run on the named laptop in R8);
- **MAC-r** = the GitHub macOS runner; **MAC-b** = a borrowed Mac.

| Mark | Where and when | Notes |
|---|---|---|
| PM-0001-1, -2 | every run (headers, load counters) | preconditions; -1 also names ML, RD and MAC once named |
| -3 | C (L1, U2); R1 and the start of every L2 block (R-6's probe); MP's 100 ± 5 ms at P2 in R7; ML\* leg in R7 | R-6, R-9, R-17 |
| -4 to -9, -11 to -13, -15 | R2 (VS2): 3 × S30 on one vCPU of RS; -12's ES runs 3 × 10 min; -13 also at P4 | bites by rules B-S and B-N |
| -10 | R2 (VS2): N = 24 and 36, 3 runs each | |
| -14 | every S30 (R2) and every browser session (R1, R4, R7; MAC-r) | the cap as -36 sets it |
| -16 | every room start in every run, at least 20; first counted in R1 | |
| -17, -18, -20 | C (L1) + R3 (VS4: L2, LG) | -17 Arm rule; -20 under both OQ-0001-6 options; R-5 |
| -57, -58, -21, -22, -47, -50 | C (L1) | -58: R-5 |
| -19 | R3 (VS4: LG, 100 joins) + R7 (ML\* Chrome, with Worker traps) | Arm rule |
| -23, -24, -31 (LG), -37, -38 | R3 (VS4) | |
| -26, -30, -59 | R3 (VS5) | -59: R-16; -30 also C (loopback) |
| -25 | R4: floods from LG (WebTransport and WebSocket, cap raised and cap on); Origin from ML\* and MAC-r | R-25 |
| -27 | R5 (D1, D2, D3: three times of day) | PM-0006-13's runs |
| -28 | ML\* (the server on the laptop's loopback: R-7), any time after U7; MAC-r with its server on RS, in R4 | both crates; a wrong hash fails 100 of 100 |
| -29 | R4: ML\*; Safari on MAC-b (SAFARI-TIME) | a time row |
| -31 (ML), -32, -33 | R7: ML\*; -32's Safari leg on MAC-r if R-13, else MAC-b | |
| -34, -35 | C | fuzzing on a dated nightly |
| -36 | R1: ML\* (loopback and the home EU path), first, beside the smoke page; Safari on MAC-b | sets -14's cap; size and bandwidth rows re-run at a lower cap if any browser, the Mac's Safari included, reports under 1,000 B; the cap is final only once the Mac reading exists |
| -41, -42, -45, -60, -61, -62 | C (L1) + R7 (ML\* Chrome over L2; -42 and -61 with 47 bots) | |
| -43, -44 | R7: ML\* through `/iso/` (the isolation flag on); RD later (nothing stands in for RD) | named machines only |
| -46, -51, -52 | C (-51) + R7 (ML\* Chrome; -52 also in Mustafa's session) | |
| -48, -49 | C (L1) + R3 (VS5: test cap, then rule cap) | -48 Arm rule; the rule cap feeds checklist steps 7 and 10 |
| -53 | R7: Mustafa, Brave in a separate profile on his PC, at P2 (§11) | judged by Mustafa |
| -54, -55 | C (L1) + R3 (VS5: -54 on L2) | R-11; -54 Arm rule |
| -56 | every run | the record is complete; R-10's cover item |
| -39, -40, -63 | not in this change-set | the 60 Hz run |
| PM-0004-67 | R2 (VS3) | R-15 |
| PM-0006-11 | every run | |
| PM-0006-12 | R1: `ping` from the laptop and from the PC to LG | reported |
| PM-0006-13 | R5 (D1, D2, D3) | the same runs as -27 |
| PM-0006-14 | R5 and R6: each N's search, then count and count + 1 at the other two times of day (§7.2) | R-19 first |
| PM-0006-15 | R4: 20 + 20 per browser, ML\* and MAC-r; the rule lists pasted from the console | R-18 |
| PM-0006-16, -21 to -24 | R5 (D1): the 8-room soak, the canaries, three stalls, the series count | -16's probe from ML\* |
| PM-0006-17, -19, -20 | R2 (VS3) | |
| PM-0006-18 | after the last run: Mustafa reads the bill | under each run's cap |
| PM-0006-25 to -27 | R4: ML\*; Safari on MAC-r if R-13, else MAC-b | R-24 |

### 6.3 Rows run before ML, RD and MAC are named (SPIKE_PLAN:156-160 asks this plan for the list)
- **On the laptop stand-in, rough checks only** (reported, never judged; re-run on the named laptop in R8):
  - PM-0001-1's ML hardware header, and -3's laptop leg;
  - the ML legs of -14, -19, -25 (Origin), -28, -29, -31, -32, -33, -36, -41 to -46, -51, -52 and -60 to -62;
  - PM-0006-15, -16's probe, and -25 to -27 in Chrome, Edge and Firefox.
- **RD:** PM-0001-43 and OQ-0001-15's RD report. Nothing stands in for RD; they wait for it.
- **MAC:**
  - the runner covers -14, -25's Origin, -28 (with its server on RS) and PM-0006-15, plus -32 and PM-0006-25 to -27
    if the advisor extends the list (R-13);
  - a borrowed Mac covers -29 and -36, and anything the advisor does not extend (SAFARI-TIME).
- **If the runner fails condition (i), (ii) or (iii):** it stands in for nothing, and the Mac is borrowed before R4,
  the first run with Safari rows.

### 6.4 The netcode evidence rule (CLAUDE.md protocol step 4)
"Deterministic tests with injected latency, jitter and loss" is what L1 is for.
- **How L1 works:** seeded, on simulated time (npt-lab's `SimClock`), with the real room and server code and 48
  headless clients in one process. It applies profiles P0 to P4 and B1, half each way, with each direction's loss
  drawn on its own.
- **What makes it deterministic:** the same seed gives the same packet-fate log and the same per-tick hashes
  (PM-0001-3).
- **What sits under it:** unit tests, plus property tests on fixed seeds, for the samplers, the codec, the quantisers,
  the wrap-aware compare, `due_ticks` and every decoder.
- **How many runs:** every L1 row runs 3 seeds per profile.
- **What L2 adds:** it runs in real time on RS, for the time marks L1 may not judge.

## 7. The runs
### 7.1 Prices and the cap rule
- **Prices** (read 2026-11-05 on hetzner.com, before VAT; the verifier re-read them the same day): CCX23
  EUR 0.1386/h, CAX21 EUR 0.0176/h. "Price incl. IPv4" appears on both pages; if the IPv4 were billed on top, it adds
  EUR 0.0008/h per VM.
- **Rounding:** each VM's hours round up per run.
- **Traffic:** RS and LG talk internally in `fsn1`, which is free.
- **What a run is:** one run plan's VM day. Its VMs are created once and deleted once, and it may hold several blocks.
  On the density days the VMs stay up between the three times of day, with the rooms stopped (R-26), so Mustafa
  creates them twice instead of six times.
- **Cap:** a run's cap is its planned hours plus one spare hour per VM, at those prices, rounded up to the next
  EUR 0.10.
- **Planning check** (PM-0006-18, arithmetic shown): 72 RS-hours × 0.1386 = 9.9792, plus 171 LG-hours × 0.0176 =
  3.0096, so **EUR 12.99**. The caps add up to EUR 14.50.
- **Against the drafts:** this is above their estimate of 38 to 53 RS-hours (OQ-0006-20, ADR 0006:1107-1113). The
  differences:
  - per-run bring-up, about 5 h;
  - the bites, about 6 h;
  - the idle hours on the density days, about 9.5 h, which save four VM creations;
  - the density rotation that gives each N three sessions (verifier F1): about 29 h against the drafts' 20.6 h;
  - R-6's L2 probes.

### 7.2 The runs (provisional; each gets its own plan, `ops/runs/Rn.md`, for Mustafa's OK first)
| Run | Blocks, with the working in hours | Mustafa | VMs | RS h | LG h | Planned EUR | Cap EUR |
|---|---|---|---|---|---|---|---|
| R1 bring-up | bring-up 0.75; certificate (staging, then production) and host-key check 0.25; headers (-1, -2, PM-0006-11) 0.15; L2 probe 0.2; calibration 0.1; -16 0.1; the Windows guard check 0.1; five-browser smoke 0.25; ping 0.1; Safari runner conditions 0.3; -36's laptop legs 0.5; first bites 0.1 → 2.9, with first-day slack to 4 | present, about 2 h | RS + 1 LG | 4 | 4 | 0.62 | 0.80 |
| R2 timed (VS2 + VS3; unattended) | bring-up 0.75. VS2: 3 × S30 1.65; -10 1.1; -12 ES 0.55; -13 at P4 0.2; bites 1.8. VS3: PM-0004-67's stall runs 0.2 and bites 0.25; PM-0006-17 1.0 and bite 0.35; PM-0006-19 1.2 and bite 0.2; PM-0006-20 0.4 and bite 0.2 → 9.85, to 11 | start and end | RS + 1 LG | 11 | 11 | 1.72 | 1.90 |
| R3 behaviour on L2 (VS4 + VS5; unattended) | bring-up 0.75; two L2 probes 0.4. VS4: -17 0.75; -18 0.2; -19 0.55; -20 0.35; -23 0.15; -24 0.35; -31 0.2; -37 0.2; -38 0.25; bites 0.85. VS5: -26 0.55; -30 0.5; -48 and -49 1.0; -54 0.3; -59 1.2; bites 0.85 → 9.4, to 11 | start and end | RS + 1 LG | 11 | 11 | 1.72 | 1.90 |
| R4 hostile input and certificates (VS6) | bring-up 0.75; issuance 0.1; -25's four floods and a clean reference 0.85; slow-loris, handshake cap and /64 cycler 0.3; Origin 0.25; -29 0.25; PM-0006-15 0.5; PM-0006-25 0.2, -26 0.55, -27 0.15; the runner's rows in parallel; bites 0.6 → 4.5, to 5 | present, about 2 h | RS + 1 LG | 5 | 5 | 0.78 | 1.00 |
| R5 density day 1 (D1 08:00, D2 14:00, D3 20:00 Amsterdam) | bring-up with a calibration for 384 sessions 0.85. D1: -27's bracket B 1.2; the N = 48 search 2.6; the 8-room ops soak 0.75. D2: -27 1.2; the N = 36 search 2.9; N = 48's count and count + 1 1.25. D3: -27 1.2; N = 48's and N = 36's count and count + 1 2.5. Idle about 3.5 → about 18 | the blocks are scheduled on RS; the laptop session checks each | RS + up to 4 LG | 18 | 72 | 3.76 | 4.00 |
| R6 density day 2 (D4 08:00, D5 14:00, D6 20:00) | bring-up 0.85. D4: the N = 18 search 4.4; N = 36's count and count + 1 1.25. D5: N = 18's count and count + 1 1.25. D6: the same 1.25. Idle about 6 → about 15 | the same | RS + up to 4 LG | 15 | 60 | 3.14 | 3.40 |
| R7 the human run (VS7) | bring-up 0.75; the ML\* legs about 5.3 (-44's 12 runs of 10 min take 2.0 of it); then his checklist 1.0 → 7.05, to 8 | present for the checklist, about 1.5 h at the end | RS + 1 LG | 8 | 8 | 1.25 | 1.50 |
| **Total** | | | | **72** | **171** | **12.99** | **14.50** |
| R8 later (own change-set) | the named-machine re-run; the Mac if SAFARI-TIME (b) | present | RS + 1 LG | 3 | 3 | 0.47 | 0.70 |

- **Density's time-of-day rule:** each N gets its search and its two confirmations at three times of day (morning,
  afternoon, and 18:00 to 23:00), each pair at least 6 h apart (ADR 0006:1314-1316, 1352):
  - N = 48: D1, D2, D3;
  - N = 36: D2, D3, D4;
  - N = 18: D4, D5, D6.
  -27 and PM-0006-13 use D1, D2 and D3.
- **The hours are rough** until each run plan fixes them. How many CAX21s comes from each run's calibration.
- **A run about to pass its cap stops** and comes back to Mustafa.
- **Server-only runs can be merged or split** if he prefers fewer VM creations or shorter days; the hours and caps add
  up the same.

### 7.3 Mustafa's steps per run (each run plan repeats them with the day's values; security F2, F11, F19)
0. Once: confirm two-factor login is on for the Hetzner and Cloudflare accounts.
1. OK the run plan: its hours and cap.
2. In the two firewall objects kept in the spike project (free; not VMs), set the day's sources:
   - `fw-rs`: SSH from the laptop's public IPv4;
   - `fw-lg`: ICMP from the laptop's and the PC's IPv4, and no SSH rule yet.
3. Create LG: CAX21, `fsn1`, Ubuntu 24.04, the operator key, **`fw-lg` chosen in the create form**, and the Cloud
   config `ops/cloud-init/lg.yaml` at the run's commit. Check the pasted file's SHA-256 against the run plan's.
4. In `fw-rs`, set the metrics rule's source to LG's IPv4.
5. Create RS the same way (CCX23, **`fw-rs` in the create form**, `rs.yaml`). On density days, create the extra
   CAX21s like LG.
6. In `fw-lg`, set the SSH source to RS's IPv4.
7. Add the DNS record of §8.3 (not needed with FIXED-IP (a)).
8. Start the laptop session with `ops/operator/PROMPT-Rn.md`. Its first commands:
   - check that no leftover `*.nt` record exists;
   - check the host keys (§1.6);
   - compare each VM's stored user-data hash and `git rev-parse HEAD` with the run plan's.
9. Do the browser steps when the run has them.
10. **Teardown:**
    1. the laptop session collects the results and has RS revoke its certificate;
    2. Mustafa **deletes the DNS record first**, and the laptop session confirms NXDOMAIN over DNS-over-HTTPS;
    3. then Mustafa deletes every VM.
    After the last run, he reads the project's costs (PM-0006-18).

**With FIXED-IP (a)** (§16): two Primary IPv4s stay in the project, one for RS and one for LG, and are assigned at
creation. Steps 4, 6 and 7 are then done once, the record never points at an address someone else gets, and teardown
deletes only the VMs.

## 8. Network, ports, DNS and certificates
### 8.1 Ports (OQ-0006-16: deciders infra-engineer and netcode-architect, confirmed by the Advisor; R-1)
| Use | Port | Open to |
|---|---|---|
| WebTransport, room slot s (0–31) | UDP 30001 + s (30001–30032) | everyone, IPv4 and IPv6 (browsers and runners cannot be allowlisted) |
| WebSocket (`wss`), room slot s | TCP 31001 + s (31001–31032) | everyone, IPv4 and IPv6 |
| The page host (§10.4) | TCP 31000 | everyone, IPv4 and IPv6 |
| Metrics exporter (scraped; bound to RS's public IPv4) | TCP 9100 | LG's IPv4 only |
| SSH | TCP 22 | RS: the laptop's IPv4 only; LG: RS's IPv4 only |
| ACME http-01 | TCP 80 | everyone, IPv4; RS's nftables opens it only while an order is live |

This plan changed infra-engineer's proposal (UDP 30000–30031, TCP 31000–31031) so that the page host sits inside the
one TCP range. Both deciders are asked in review round 2 (verifier F7). PM-0006-15's closed-port probe uses a port
outside every range.

### 8.2 Firewall rule lists (inbound; `ops/firewall/`, each rule written per IPv4 and IPv6; security F10)
| Role | Rules |
|---|---|
| RS (`fw-rs`) | UDP 30001–30032 from any (v4 and v6) · TCP 31000–31032 from any (v4 and v6) · TCP 80 from any (v4) · TCP 9100 from LG's IPv4 /32 · TCP 22 from the laptop's IPv4 /32 · no ICMP (ADR 0006:1350) |
| LG and each extra CAX21 (`fw-lg`) | TCP 22 from RS's IPv4 /32 · ICMP from the laptop's and the PC's IPv4 /32 (PM-0006-12) |

- **The VM's own wall:** nftables mirrors these lists in `table inet` (default drop) and loads before any service.
- **Outbound limits:** after build and issuance, nftables drops new outbound connections from the `npt-room` and
  `npt-host` users (`meta skuid`), except established ones, and blocks their access to 169.254.169.254. So a
  compromised room cannot send the TLS key out.
- No private network is attached.

### 8.3 DNS records (Cloudflare, zone `shamblefrontlines.com`; every record "DNS only", grey cloud)
| Host | Type | Name, as typed | Value | TTL | Proxy status | When |
|---|---|---|---|---|---|---|
| RS | `A` | `rs.nt` (= `rs.nt.shamblefrontlines.com`) | RS's public IPv4 of that run | 1 min | DNS only | every run, as soon as RS has its address (with FIXED-IP (a): once, to the kept address). RS's cloud-init waits for it before asking for the certificate |
| RS, spare name | `A` | `rs-b.nt` | RS's public IPv4 | 1 min | DNS only | only when the run plan says `rs.nt` would get a 6th certificate within 7 days |

- **"DNS only"** answers with the server's own address and carries no traffic through Cloudflare. A proxied record
  carries only HTTP(S), never WebTransport's UDP.
- **No AAAA record:** the proof test runs on IPv4 (header sizes for PM-0001-14 and PM-0006-17).
- **No CAA record for now.**
- **No record for LG:** it is reached only through RS (§1.6).
- **Teardown deletes the record first,** then the VMs (§7.3).

### 8.4 The WebSocket leg's browser-trusted certificate (OQ-0006-5 development case (i); security-auditor's conditions, R-2)
- **The method: ACME http-01 from RS itself.**
  - The ACME client sits inside the locked workspace (an ACME crate in npt-host; U6 checks it supports profiles). It
    requires Let's Encrypt's 6-day `shortlived` profile and fails closed. If no client in the workspace supports it,
    the plan returns to security-auditor, with a 90-day certificate revoked at teardown as the fallback.
  - The private key is made on RS, owned 0640 by a group of `npt-room` and `npt-host`, and never leaves the VM.
  - Each VM opens a new ACME account, so no secret exists anywhere.
  - Port 80 is answered by `npt-host`, with `CAP_NET_BIND_SERVICE`, only at `/.well-known/acme-challenge/<current
    token>`, with a header timeout. nftables opens it only while an order is live.
- **Staging first:** in R1 the first issuance goes to Let's Encrypt's staging service, then to production.
- **The ledger:**
  - before each run, the issuances are checked against CT (crt.sh over HTTPS, from this session);
  - limits: 5 per exact set of names per 7 days, 50 per registered domain;
  - any certificate for a `*.nt` name that is not in the ledger stops the run.
- **Revocation** at teardown, before the VMs are deleted.
- **Public names:** every issued name lands in the public CT logs, so stray probes come. The firewall, the roster and
  the join tokens contain them (§10.3), and the results count them.
- **Not taken:**
  - dns-01 with a Cloudflare API token: a secret on a VM;
  - tls-alpn-01: TCP 443 is kept for the lobby (OQ-0006-16 leaves 443 to slice 3);
  - an IP-address certificate: PM-0006-15 names a certificate on the domain.
- **Fallback, if port 80 is refused:** dns-01 by hand, with Mustafa adding one TXT record per issuance.

### 8.5 The WebTransport development certificate (ADR 0006:302-305; PM-0006-25 to -27)
- **Made by:** the room server or supervisor on RS at start: ECDSA P-256, valid at most 14 days, never in the
  repository.
- **Its hash:** npt-host publishes it, with the next one for PM-0006-26's rotation, in `/npt/cert-hash.json`
  (`Cache-Control: no-store`) over the browser-trusted name.
- **PM-0006-27's 15-day certificate** is made the same way, on purpose.

## 9. The Safari leg (MACHINES-TIMING; SPIKE_PLAN:45-57)
- **Facts** (read 2026-11-05; the verifier re-read them):
  - `macos-26` (arm64, image 20261026.0455.2) carries Safari and SafariDriver 26.6.2; `macos-15` (arm64, image
    20261028.0489.1) carries Safari 27.2. Both are at least the 26.4 WebTransport needs.
  - Minutes are free on a public repository.
  - Nothing official says UDP leaves a macOS runner, and a 2022 issue shows UDP 123 dropped. So condition (iii) is a
    real test.
- **The workflow** (security-auditor and infra-engineer review the file at stop B; **any `.github/` change goes to
  Mustafa for approval**):
  - **File and trigger:** `.github/workflows/npt-safari.yml`, `workflow_dispatch` only. No `pull_request`, no
    `workflow_run`, no cache.
  - **Who dispatches:** Mustafa or a day session, never the night shift or a routine.
  - **A guard step:** the first step fails unless `github.ref` is `refs/heads/main`. A dispatch against another ref
    runs that ref's copy of the file, which could drop every check (verifier F5). The guard catches mistakes; review
    holds the rest.
  - **Permissions:** `permissions: {}` at the top, and `contents: read` on the job; no secrets, no environment;
    `persist-credentials: false`.
  - **Actions:** each pinned by its full SHA, as `gate.yml` does.
  - **Inputs** (through `env:`, and checked before use):
    - the host, from an exact allow-list: `rs.nt.shamblefrontlines.com` or `rs-b.nt.shamblefrontlines.com`;
    - the ports, numbers inside the ranges;
    - the commit: 40 hex characters, and contained in `origin/main` or the change-set's branch;
    - a single-use join token minted for that dispatch. It is treated as public, since inputs show in the run's page.
    No `${{ inputs.* }}` appears inside a `run:` line.
  - **Runners:** `macos-26` and `macos-15` as a matrix, labels pinned.
  - **Limits:** `timeout-minutes: 30`, and a concurrency group.
  - **Steps:** check out the commit; `npm ci --ignore-scripts`; `sudo safaridriver --enable`; record `sw_vers`, the
    Safari, SafariDriver and image versions; run the WebdriverIO Safari spec against the page on RS.
  - **Output:** one JSON artifact with `retention-days: 3`, holding versions, verdict inputs and VM names only. The
    log carries the same.
- **Why a separate PR (W-PR):** a `workflow_dispatch` workflow triggers only if its file exists on the default branch,
  and once it has run, a dispatch may name any ref, whose copy of the file then runs (docs.github.com, read
  2026-11-05). So W-PR reaches main before R1, on Mustafa's approval and go. The kit is frozen until this test is
  under way; he decides.
- **The first dispatch,** first thing in R1, records conditions (i) to (iii):
  - (i) the image's Safari version, at least 26.4;
  - (ii) real Safari through safaridriver, not a WebKit build;
  - (iii) one WebTransport session to RS that opens and carries datagrams, and one `wss` session on the domain.
  If any of the three fails, the runner stands in for nothing, and Mustafa borrows the Mac before R4.
- **What the runner may stand in for** (the advisor's reading, SPIKE_PLAN:54-57): PM-0001-28 (server on RS), -14,
  -25's Origin and PM-0006-15. It cannot stand in for a time row (-29), for -36 (which needs a home EU connection), or,
  unless R-13 extends the list, for -32 and PM-0006-25 to -27.

## 10. Security (security-auditor's round-1 findings built in; it reviews this revision in round 2 and the run package at stop B)
### 10.1 What faces the internet while a VM lives
- **RS:** the game and page ports, open to all; TCP 80 only while a certificate order is live; SSH from the laptop's
  address only; the exporter to LG only.
- **LG:** SSH from RS only, and ICMP from two addresses.
- **On the density days** the VMs stay up between blocks, with every room stopped; the firewall and nftables stay in
  force.

### 10.2 The control plane (security F1, Critical)
- **Room-wide controls** go only through `ControlSocket`, a Unix socket on RS (mode 0600, owner `npt`), driven over
  SSH by the laptop session or by the run script on RS. They are never a network message. They cover the rewind cap,
  the per-address cap raise, the roster and its tokens, panic and stall injections, and starting and stopping rooms.
- **Per-session controls** (`SessionControl`: the profile, B1, the UDP block, the record form's steps) need a per-run
  capability:
  - 128 random bits, minted on RS through the control socket;
  - handed to the browser in the URL fragment, which never reaches the page host;
  - sent by the page on its session's reliable control channel and compared in constant time;
  - acting only on the sender's own session.
  The automated browser legs get their own capabilities the same way.
- **Logging and verdicts:** every control and form record is logged with its source. npt-judge marks a run INVALID if
  any control arrived without a valid capability.
- **A bite:** a control or a form post from a plain join is refused and counted.
- **The catalogue** (`docs/design/`) gives every message and control its channel, its authority (none, capability or
  local socket) and its size limit.

### 10.3 Join tokens and the roster (security F3)
- **Minting and delivery:** tokens are minted per participant on RS through the control socket, and delivered out of
  band:
  - bots by file, over the internal path;
  - browsers in the URL fragment, from the laptop session;
  - the runner a single-use token per dispatch, treated as public.
- **Binding:** each token is bound to one room, and expires at the run's end.
- **The roster:** a room refuses joins beyond its roster's N. npt-host never mints tokens.
- **Strays:** refused and stray sessions are counted per run. A measured run that admitted any session outside its
  roster is INVALID.

### 10.4 The page host (security F7)
- **Files:** it serves only a compiled-in allow-list from a dedicated dist directory: the index, the scripts, the
  Worker, the Wasm, `cert-hash.json`, `ssh-host-keys.txt` and `robots.txt`.
  - GET and HEAD only; no directory listing.
  - Never the clone, `.git`, logs, results, canaries or keys.
- **Headers:**
  - CSP: `default-src 'none'; script-src 'self' 'wasm-unsafe-eval'; worker-src 'self'; connect-src 'self'
    https://rs.nt.shamblefrontlines.com:* wss://rs.nt.shamblefrontlines.com:* https://rs-b.nt.shamblefrontlines.com:*
    wss://rs-b.nt.shamblefrontlines.com:*; style-src 'self'; img-src 'self'; base-uri 'none'; form-action 'none';
    frame-ancestors 'none'`;
  - `nosniff`; `Referrer-Policy: no-referrer`; `X-Robots-Tag: noindex`, and a `robots.txt` that disallows all;
  - `no-store` on the two files; `application/wasm` for the Wasm;
  - COOP and COEP only under `/iso/` (E21).
- **Limits:** a 5 s header timeout, an 8 KB header cap, per-address and total connection caps. E14's slow-loris and
  floods also hit TCP 31000 and 80.
- **A bite:** `/../Cargo.toml`, `%2e%2e` and `/npt/` must answer 404.
- **Origins:** RS's Origin allow-list is exactly the page origins (`https://rs.nt.shamblefrontlines.com:31000`, and
  the spare name's), with no localhost and no `null`. The runner loads the page from RS. Only the loopback server of
  R-7, on the laptop, admits `http://localhost`.

### 10.5 cloud-init and the VM (security F8, F10)
- **Users:**
  - `npt`: the SSH login, with sudo for a fixed command list only;
  - `npt-build`: clones and builds;
  - `npt-room`: runs the rooms, with no shell and no write access to the binaries or the clone. Each room slot gets
    its own user, B12's interim (ADR 0006:542). The mechanism is chosen at U6 and reviewed at stop B;
  - `npt-host`: the page host and ACME.
- **SSH and logins:**
  - `ssh_pwauth: false`, `disable_root: true`, `ssh_genkeytypes: [ed25519]`;
  - an `sshd_config.d` drop-in: `PasswordAuthentication no`, `KbdInteractiveAuthentication no`, `PermitRootLogin no`,
    `AllowUsers npt`, `AuthenticationMethods publickey`, `X11Forwarding no`, `AllowAgentForwarding no`,
    `MaxAuthTries 3`, `LoginGraceTime 20`;
  - on RS only, TCP forwarding with `PermitOpen` to LG's port 22 alone.
- **Order:** nftables loads first, before any service (§8.2).
- **Toolchain:** rustup-init by a pinned SHA-256 per architecture. No `curl | sh`, no `#include` URL.
- **Build:** as `npt-build`, a clone at the run's full commit SHA, then `cargo fetch --locked` and
  `cargo build --locked --offline`.
- **Upgrades:** security upgrades at creation; unattended upgrades and reboots off during runs.
- **The file itself:**
  - it holds no secret, since every process on the VM can read user data;
  - it stays under Hetzner's 32 KiB limit (checked at U8);
  - the run plan prints its SHA-256, and the laptop's first command compares the stored user data and
    `git rev-parse HEAD` with the run's values, stopping on any mismatch.

### 10.6 Raw data and redaction (security F14, F18; R-21)
- **Raw data stays off the repository:** LG's received-byte logs (about 1.4 MB/s), packet captures, per-tick logs and
  auth logs. They live on the VM, or in `.gate/` on the laptop, and are deleted with the VM or at the close.
- **What is committed:** verdicts, numbers, hashes and redacted headers, with the VMs' addresses and the date (R-21).
  Never the operator's address or Mustafa's.
- **`ops/operator/redact-check`** runs before every push from the laptop. It fails on:
  - any IP address not in the run's VM list (the operator's address is read from a local file and never printed);
  - token and capability patterns;
  - canary values;
  - PEM or OpenSSH private-key headers.
  Results never include auth logs, or `who`, `last` or `ss` output. PM-0006-21's canary grep runs on LG, and only
  counts leave it.

### 10.7 Floods, abuse, accounts and the supply chain
- **Probes and floods (security F16):** closed-port probes (PM-0006-15) come only from the laptop and the runner,
  never from LG. R4's run plan states the flood rates.
- **An abuse notice:** the run stops, and Mustafa answers that these are his own test servers.
- **Accounts:** two-factor login is confirmed on both accounts (§7.3 step 0).
- **Supply chain:** pinned dependencies and the audits of §4.8.

## 11. Mustafa's checks
1. **R1, the five-browser smoke check (about 10 minutes).**
   - In Brave, in a separate profile (not his everyday one), open the link the laptop session prints:
     `https://rs.nt.shamblefrontlines.com:31000/?smoke=1#cap=…` (the capability sits in the fragment). Expect
     "WebTransport · Playing" within 3 s, and Brave's version on the page.
   - The same link with `&transport=ws`: expect "WebSocket · Playing".
   - The laptop session repeats both in Chrome, Edge and Firefox.
   - If a browser cannot pin a certificate hash, the page says so and R-24 applies: never a browser flag, never a
     custom root.
2. **R1, the operations checklist, step 1 (PM-0006-12).** In a terminal on the laptop and on the PC, run
   `ping -n 100 <LG's address>` and paste the "Packets" and "Approximate round trip times" lines.
3. **R7, the netcode checklist (PM-0001-53; about 60 minutes).** test-engineer writes it into
   `ops/checklists/netcode.md` from ADR 0001:1631-1681, and the advisor checks it before R7.
   - 14 steps, mostly at P2: step 2 runs at P0, step 12 at P3 and P4, step 13 over WebSocket.
   - 47 bots, Mustafa in the 48th slot, plus the target course.
   - The overlay shows the profile, the transport, the server-measured round trip, the interpolation delay, the rewind
     cap and the counters.
   - He records each step in the page's form, which uses the capability.
   - The mark passes when every step is "achieved". Step 7 needs the rule cap that R3's honest-rewind runs set.
4. **After the last run, the operations checklist, step 2 (PM-0006-18).** Read the project's costs: one amount before
   VAT.

Besides these: his go for R-PR and W-PR, and his OK of each run's cap.

## 12. Steps, stops, sessions, checkpoints and agents
### 12.1 Units (each ends with its tests green, the gate run, notes saved and the checkpoint pushed)
| Unit | Size | Content |
|---|---|---|
| U0 | S | Container probes, each recorded: rustup 1.99.1 with the wasm32 target; the wasm-bindgen CLI at `Cargo.lock`'s version; a dated nightly for cargo-fuzz; Chromium for local rough checks; a WebTransport session over loopback QUIC |
| U1 | S | The skeleton: the workspace, pins, `deny.toml`, the clippy configs, npt-log, `marks.json`, npt-judge with its PASS, FAIL and INVALID fixtures, the dependency-rule checks (§4.2); clippy and the wasm32 check from here on; the message catalogue (netcode-architect drafts it) |
| U2 | M | npt-l1 with its unit and property tests; PM-0001-3's L1 half and its bites |
| U3 | S | R5-TEST-OQ2 on loopback: each WebTransport crate on a custom quinn endpoint over `ImpairSocket`; the WebSocket `DelayStream`. This fixes L2's shape; if neither crate allows it, stop (R-22) |
| U4 | M | npt-sim, npt-proto, npt-room and npt-client; npt-lab's scenario runner; `due_ticks` with PM-0004-67's bites on a fake clock; the Node-hosted native-against-Wasm hash test (§4.5); then the L1 row groups with their bites: codec (-17, -57, -58); commands and hostile input (-20, -21, -22); prediction (-41, -42, -60); interpolation, clock and events (-45, -47, -61, -62); lag compensation (-48 to -51); bursts (-18); stand-ins (-54, -55); -30 on loopback |
| U5 | S | Fuzzing (-34); the dependency and swap checks (-35); the audits (§4.8) |
| U6 | M | npt-server, npt-net, npt-super and npt-host: both WebTransport crates and WebSocket with L2 on loopback; the control socket and capabilities; the roster; interest under load; instrumentation; metrics and logs; the ACME client's profile check |
| U7 | M | npt-client-wasm and the harness (E7); the browser smoke in local Chromium; the hash check in Chromium |
| U8 | S | The run package: cloud-init files (32 KiB check), firewall lists, R1's run plan, the operator `settings.json`, `rules.md` and `redact-check`, the WebdriverIO specs, the netcode checklist (test-engineer), W-PR's workflow file |

### 12.2 Stops (where the work waits for a person)
- **Stop A, now:** review round 2 on this revision, then the advisor, then Mustafa.
- **Right after approval: R-PR.** One small docs PR writes the advisor's readings that change a number or a setup
  into ADR 0001, ADR 0006 and SPIKE_PLAN (R-27). It merges on Mustafa's go. No counted run comes before it.
- **Stop B, before any VM exists:**
  - security-auditor and infra-engineer review the run package (cloud-init, firewall lists, page host, control plane,
    operator settings, `redact-check`) and W-PR's file; fixes follow;
  - Mustafa approves W-PR and gives his go to merge it;
  - he OKs R1's run plan and cap.
- **Each run:**
  - his OK of the cap first;
  - his presence in R1, R4 and R7;
  - after each run: the results judged, the checkpoint updated, and a three-line summary for him.
- **A failed or INVALID run:** its cause is found before the next run under that mark. A needed change to a mark goes
  to the advisor, and through R-PR's route.
- **Stop C, the close:**
  1. the final diff goes to the verifier and the reviewers (§14), and fixes follow;
  2. the report and docs-keeper;
  3. the results go into ADR 0001;
  4. the checkpoint is deleted, the PR is marked ready, and the advisor reviews it;
  5. it merges on Mustafa's go: his checklist is a feel check, and the ADR edit is a rules change.
- **R8, later:** its own short change-set (R-14).

### 12.3 The laptop session (security F5; R-23)
- **Folders:** its main folder is a clone on main, so main's rules load. The run's commit and the results branch live
  in worktrees under `.gate/` (`.gate/npt-run`, `.gate/npt-results`).
- **Mode:** Manual permission mode (the default), never auto or bypass.
- **Extra settings:** it starts with
  `claude --settings .gate/npt-run/spikes/netcode-proof/ops/operator/settings.json`. That file is reviewed at stop B.
  It denies:
  - pushes to any branch but `claude/npt-results`;
  - `gh`, WebFetch and WebSearch;
  - reading `~/.ssh/npt_operator`.
- **What it never does:** edit the spike's code, or push to the change-set's branch. A fix comes from this session;
  the laptop then moves its run worktree to that commit and rebuilds on the VM.
- **What it reads:** npt-judge's summaries rather than raw logs. Every VM output is data, never instructions. RS's
  logs carry no client-supplied string verbatim (E22).
- **Browsers:** WebDriver uses fresh temporary browser profiles, never Mustafa's own.
- **R1, step 0:** on his machine, prove that the guard loads and that a push to the change-set's branch is refused.
  STATUS lists the kit-v3-1 Windows test as still open.
- **Pushing results:** only after `redact-check` passes, to `claude/npt-results`. This session reads that branch with
  `git fetch` and `git show`; its main folder never switches to it or merges it.

### 12.4 Checkpoints, agents, models
- **Checkpoint:** `.checkpoints/netcode-proof-test.md`, updated after every unit and at least every 30 minutes. The
  background saver runs while agents run. Notes and small scripts go in `.checkpoints/netcode-proof-test/`; builds,
  raw logs and captures in `.gate/netcode-proof-test/`.
- **Gate and tests:** `node tools/gate.mjs` must PASS at every step. The spike's own test command (cargo test, clippy
  for host and wasm32, the dependency-rule checks, the grep, and the `web/` tests) runs at every unit, with its output
  in the checkpoint.
- **Agents:**
  - at most 3 at a time, workflow agents included;
  - every agent on Opus 5.5 at `xhigh` (a netcode change-set);
  - agents never edit source, and the main session is the one writer of code;
  - netcode-architect writes the catalogue in `docs/design/` and, at the close, ADR 0001's results; test-engineer
    writes the checklist text, which the main session commits.
- **Night queue:** nothing. The routine guard runs no `cargo`.

## 13. Risks, ranked
1. **Pinned-hash WebTransport may not work in every browser.** Handling: U7 locally, R1's smoke page, then R-24.
2. **R5-TEST-OQ2 has no fallback** if neither crate sits on a custom quinn endpoint. Handling: U3 answers it before
   any VM hour, then R-22.
3. **The laptop session is a new trust point:** a shell on Mustafa's laptop, with SSH and push rights, reading text
   that attackers can shape. Handling: §12.3's settings, Manual mode, summaries rather than raw logs, and R1's check
   of the guard on his machine.
4. **The macOS runner's UDP is unknown.** If a condition fails, the Mac is borrowed before R4.
5. **Size, L + M, and usage-limit cuts.** Handling: small units, a checkpoint after each, at most 3 agents.
6. **The stand-in sim may be lighter than the stress scene.** Handling: PM-0001-4 comes out STAND-IN.
7. **Two marks cannot count as written:** R-5 and R-6.
8. **ML, RD and MAC are not named,** so the test closes in two steps (R-14).
9. **Hetzner:** today's public pages list only NBG1 and HEL1 for both plans, and say "currently unavailable"
   (verifier F12). Mustafa created both in `fsn1` on 2026-11-03. A run day with no `fsn1` stops and goes to him; the
   location is not reopened.
10. **Let's Encrypt's limits.** Handling: the ledger checked against CT, the spare name, staging first.
11. **Leaks into the public repository.** Handling: `redact-check`, the saver's secret check, review at stops B and C.
12. **Native and Wasm must agree for prediction to be exact.** Handling: §4.5's rules, and its check from U4.
13. **Stray traffic once names appear in CT.** Handling: the roster, the tokens, and the INVALID rule.
14. **Mustafa's time:** seven runs, about 15 minutes of console work each, and his presence in R1, R4 and R7.

## 14. Reviewers
- **Plan round 1 (done):** security-auditor, engine-architect, the verifier.
- **Plan round 2 (this revision):** security-auditor (its fixes), netcode-architect (OQ-0006-16's co-decider; the
  netcode design changes; R-5, R-6, R-25 and the control plane), infra-engineer (OQ-0006-16's co-decider; the runs,
  hours and caps, the firewall order, DNS and the workflow rules). Then the advisor.
- **Stop B:** security-auditor and infra-engineer.
- **The final diff,** in rounds of at most 3:
  1. netcode-architect, security-auditor, rust-reviewer;
  2. test-engineer, engine-architect, perf-auditor;
  3. infra-engineer, then the verifier.
- **Not called:** art-director and graphics-engineer (not the look); game-designer (no rules or balance).

## 15. Readings and choices for the advisor (none settled until the advisor rules; "→ R-PR" marks those that change a number or setup)
| # | Question | Recommendation |
|---|---|---|
| R-1 | PORTS (OQ-0006-16): §8.1's numbers, this plan's change to infra-engineer's proposal | confirm once both deciders agree in round 2 |
| R-2 | CERT: http-01 on RS with §8.4's conditions | approve (security-auditor approved it in round 1, with those conditions) |
| R-3 | METRICS path: scrape on TCP 9100, bound to RS's public IPv4, LG's address the only source in both walls | scrape |
| R-4 | HOST KEYS: §1.6 | approve (security-auditor's method) |
| R-5 | REORDER, per stream: inputs never reorder at P2 or P3 under half-split uniform jitter (one-way ranges 20 ms and 30 ms, below the 33.33 ms spacing; P4: 1.39 % of adjacent pairs). Snapshot groups split within a tick, and pings, may. So -20's presses and -58's wraps cannot count as written | an explicit L1 reorder step on the input stream for those two rows (swap 1 % of adjacent input datagrams per direction, recorded beside the profile), plus a recorded P4 run; every run records each stream's share → R-PR |
| R-6 | PACKETS: PM-0001-3 says 100,000; PM-0002-49 says 1,000,000, since 100,000 fails a correct simulator about 1 seed in 9 at P1 | 1,000,000 on both layers; on L2 a dedicated probe flow at 10,000 packets per second per direction, not game traffic (about 100 s per profile, about 10 minutes per L2 block) → R-PR |
| R-7 | LOOPBACK: -28's "same local network" | the server on the browsers' own machine, page on localhost, that server's Origin list localhost only |
| R-8 | X86 REPLAY: "CI's x86-64 runner" | this container or RS; no CI job |
| R-9 | L2 DELAY (PM-0001-3) | (profile RTT − base RTT) ÷ 2 per direction → R-PR |
| R-10 | COVER: OQ-0001-9's cover half | one recorded item under -56: honest-rewind runs with a destructible-wall stand-in and a moving vehicle stand-in as cover, the "hit after cover" share recorded → R-PR |
| R-11 | STAND-INS meet -54's and -55's conditions | yes; both rows run → R-PR |
| R-12 | BITES: rows without one, and bite length | test-engineer digest F's proposals plus: -18 (a baseline that never falls back), -36 (the size read once), -55 (vehicle collision that snaps), PM-0006-11 (a header without the SMT map), -14 (short padding must come out INVALID), -26 (a rotation that closes live sessions); PM-0004-67 keeps its draft bite (timer-period counting) and gains two (an uncapped catch-up; an uncounted drop); memory and queue marks bite over a full S30 → R-PR |
| R-13 | RUNNER LIST: Safari's count rows -32, PM-0006-25, -26, -27 | add them → R-PR (SPIKE_PLAN's text) |
| R-14 | CLOSE in two steps | this PR merges with the named-machine rows pending; R8 judges them; migration step 1 waits for R8 |
| R-15 | PM-0004-67's validity and runs | V-S applies; the jitter read from the 3 S30 runs; the stall runs separate → R-PR |
| R-16 | -59's random tick | within the first 5 minutes after warm-up, in its own S30 room → R-PR |
| R-17 | SIMULATOR GAPS | the jitter band compares the one-way delay's standard deviation with the uniform's (half-range ÷ √3); B1 cuts both directions; the rate limiter is drop-tail with one snapshot interval of queue; duplication 0 % → R-PR |
| R-18 | (a) the page host on TCP 31000 inside part 5's one TCP range; (b) LG's list is checked against `ops/firewall/lg.txt`, since part 5 has no LG role | (a) yes, or record it as a rule outside part 5; (b) yes |
| R-19 | OQ-0006-19's reading, due before the density runs | the advisor gives it at stop B, before R5 |
| R-20 | R2-XU-OQ7: listed as decided here, no rule found | name its rule, or take it off the list |
| R-21 | ADDRESSES | the VMs' IPv4 and IPv6 with the date, yes; never the operator's; enforced by `redact-check` |
| R-22 | L2 FALLBACK, if U3 fails | decided when U3 reports; any option stays below `Link` |
| R-23 | TWO SESSIONS | yes, with §12.3's conditions |
| R-24 | BROWSER PIN | that browser's other WebTransport rows use the 6-day certificate on UDP, recorded; PM-0006-25 to -27 fail for it; never a browser flag or a custom root |
| R-25 | WS DELAY: ADR 0001:1444-1445's "delaying TCP proxy" | built as a delaying stream wrapper inside npt-server's accept path, so the room sees real source addresses and keeps its deadlines and caps → R-PR |
| R-26 | DENSITY DAYS: may the three times of day share one VM set kept up all day? | yes: rooms stopped between blocks; each block keeps its own bracket B and validity; "session" read as a measurement session → R-PR |
| R-27 | R-PR: how a reading that changes a number or setup reaches the drafts (ADR 0001:1603-1604; verifier F9) | one small docs PR right after this plan's approval, merged on Mustafa's go before the first counted run; alternatively the advisor rules that a reading recorded in the run records meets 1603-1604 |

## 16. Queue, decisions and asks
- **Queue:** STATUS (2026-11-05) names the netcode proof test as the next v2 work. B.1 and B.2 (docs) touch other
  files. A4 waits until this test is under way (A4-FREEZE (a)). The kit freeze also covers W-PR, and Mustafa's
  approval decides it.
- **At the close:** this plan goes into the report's appendix, and the checkpoint is deleted (CHECKPOINTS.md §9).

### Decisions needed (Mustafa)
1. **ROUTE: how should the hands-on server work run, since this session cannot SSH?**
   - *Why it matters:* every run needs an operator, and the choice sets your time per run and where the SSH key lives.
   - *Options:*
     - (a) the hybrid: cloud-init builds each VM, and Claude Code on your laptop runs the sessions over SSH, with a
       key made there and a reviewed settings file of its own;
     - (b) cloud-init only: the VMs run fixed scripts and publish results over HTTPS. There is no SSH and no fixing in
       place, and LG gets a results server;
     - (c) Claude Code on your machine only, without cloud-init.
   - *Also:* which machine runs that session (the laptop, where the minimum-laptop browser legs run, or your PC), and
     three facts the plan assumes: the laptop's operating system, which browsers it has, and whether Claude Code is
     installed on it.
   - *Recommendation:* (a), on the laptop.
2. **SAFARI-TIME: when do Safari's timed rows run?**
   - *What:* the runner may stand in only for yes-or-no and count rows. The 3-second fallback (PM-0001-29), the
     datagram size on a home connection (PM-0001-36), and any row the advisor does not add to the runner's list,
     need a real Mac.
   - *Why it matters:* you planned to borrow the Mac only for the look proof test.
   - *Options:*
     - (a) borrow it once during this test: one session, about an hour, at home;
     - (b) re-run those rows when you borrow it for the look proof test. Until then they stay "pending", and nothing
       before the foundation waits on them.
   - *Recommendation:* (b).
3. **FIXED-IP: keep two fixed addresses for the test machines?**
   - *What:* two Primary IPv4s kept in the spike project, one for RS and one for LG, at about EUR 0.50 a month each
     before VAT, for as long as the test runs.
   - *Why it matters:* without them, every run needs a DNS edit and two firewall edits, and teardown must delete the
     record before the VMs. With them, those are set once, and the name can never point at an address Hetzner hands
     to someone else.
   - *Options:* (a) keep two fixed addresses; (b) a new address each run, as the drafts assume.
   - *Recommendation:* (a).

### For Mustafa
1. Read this plan, then answer ROUTE (with the laptop facts), SAFARI-TIME and FIXED-IP.
2. Nothing to create yet: no VM, no DNS record, no key.
3. Next you get R-PR (the advisor's readings into the drafts) for your go.
4. At stop B you get W-PR (the Safari workflow), R1's run plan with its cap, the cloud-config files, and the exact DNS
   record with that day's address.
