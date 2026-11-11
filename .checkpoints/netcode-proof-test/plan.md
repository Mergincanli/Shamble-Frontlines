# Plan: the netcode proof test (SPIKE_PLAN item 1), revision 3.3

- **Change-set:** `netcode-proof-test` · **Branch:** `claude/netcode-proof-test-plan-hcky5g`. This cloud session may push
  only to this `claude/…` branch, so it stands in for `cs/netcode-proof-test` (CLAUDE.md, standing permission).
  **Base:** main `af8393e`.
- **Date:** 2026-11-06 · **Status: approved.** The advisor approved revision 3 with 12 conditions (C1–C12), folded in
  as revision 3.1 (§0.3). **Mustafa approved revision 3.1 on 2026-11-06** and answered the five decisions; revision 3.2
  records his answers (§0.4) and changes nothing else.
- **History:**
  - Revision 1 (`786b7d2`): review round 1 returned 49 findings (`review1-findings.md`; mapped in §0.1).
  - Revision 2 (`d8f0712`): review round 2 returned 46 (`review2-findings.md`; mapped in §0.2):
    - security-auditor: APPROVE WITH CONDITIONS;
    - netcode-architect: REVISE, text changes only;
    - infra-engineer: APPROVE WITH CONDITIONS.
    Both deciders of OQ-0006-16 agree with the port plan.
  - Revision 3 (`a6607bc`): the advisor's plan review, APPROVE with C1–C12. Its verdict, rulings and the exact words for
    R-PR are in `advisor-verdict.md`. Its notes are in `advisor-plan-review.md`.
  - Revision 3.3 (2026-11-07): the units' rulings folded in, with Mustafa's approval of 3.1 standing. It changes no mark,
    run or cost.
    - What: Mustafa's WebSocket decisions of 2026-11-07 (§0.5), and U3c's plan text per `u3c-plan.md` 1.9 (the
      advisor's APPROVE of the U3c plan with C1-C16).
    - Where: §4.1, §4.2 (npt-tcpinfo and the unsafe rule), §4.3 E15 (the faithful model and fix 1), §4.4, §4.7, §4.8,
      §12.1's U3 and U6 rows, §15 (R-25, R-30) and §16.
    - Already listed in §4.7 before this revision: L2Config (a0d7916).
- **Sources** (all in this folder, line-cited): `netcode-architect-digest.md`, `infra-engineer-digest.md`,
  `test-engineer-digest.md`, `ssh-probe.md`, and the two review files.

## 0. In short
- **Item 1, SSH: no.** This session cannot reach any server over SSH. The image has no `ssh` client; TCP to port 22,
  and to every port but 443, times out; 443 passes an egress gateway that re-terminates TLS. It can read any public
  HTTPS site.
- **Recommended route: the hybrid.**
  - A reviewed cloud-init file (Hetzner's "Cloud config") builds every VM the same way, holding no secret and no
    address.
  - Claude Code on Mustafa's laptop does the hands-on part over SSH. It uses a key it makes there, under a fixed,
    hash-checked settings file of its own, in Manual mode. His prompts are the wall the settings cannot be.
  - This session writes all the code, proves everything that needs no VM, and judges the results.
- **What gets built:** a throwaway Rust and TypeScript spike in `spikes/netcode-proof/`, split as ADR 0002 part 3
  splits the real engine:
  - a room library that builds for Wasm, and a server library with a thin binary;
  - a poll-based link interface with channels, with L1 behind it and L2 under the real socket;
  - the client endpoint;
  - bots, a lab runner on simulated time, logs and a judge.
  The client harness (page, Worker host, plain renderer) is built in ADR 0002 part 5's shape, for the boundary spike to
  keep. No legacy file changes.
- **What it proves:** PM-0001-1 to -62 without the 60 Hz group (-39, -40, -63, later); PM-0004-67; PM-0006-11 to -27.
  Mustafa judges "looks correct" at P2 in Brave (100 ms round trip, ±20 ms jitter, 2 % loss, half each way), and runs
  the operations checklist. Rows that need named machines close in a short re-run later.
- **Runs:** seven run days, each with its own run plan, hours, cost cap and delete-by time for his OK first.
  - About 75 RS-hours and up to 177 LG-hours: **EUR 13.51 planned before VAT**, with caps adding up to EUR 14.90.
  - With FIXED-IP, EUR 1.00 a month for two kept addresses. The re-run on named machines adds about EUR 0.47 later.
  - He is present in three runs; the others run unattended after bring-up.
- **Safari:** GitHub's macOS runners first (Safari 26.6.2 and 27.2 on today's images). The workflow is a `.github/`
  change, so **it goes to Mustafa for approval**, in its own small PR before the first run.
- **DNS:** one record per test host, always "DNS only": type `A`, name `rs.nt`, value RS's IPv4, TTL 1 min. Set once
  with FIXED-IP, or every run without it. No AAAA.
- **Decisions:** five for Mustafa (ROUTE, SAFARI-TIME, FIXED-IP, DENSITY-FIGURE, UNDER-WAY; §16). The advisor has
  ruled the 29 readings (§15): one is his (DENSITY-FIGURE), and fifteen go into the drafts through R-PR.

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


### 0.2 What changed from revision 2 (each round-2 finding and where it is fixed)
| Theme | Findings | Fixed in |
|---|---|---|
| The laptop's worktrees: `node .gate/*` denied; the results branch an orphan holding results only; both worktrees checked before reading; `redact-check` from a fixed copy | security N1 | §12.3 |
| What the operator settings can enforce: denies in Bash and PowerShell forms; every other push prompts; the sandbox where the OS has one; a fixed, hashed copy of the file; never "don't ask again"; the guard check at stop B with a push that changes nothing | security N2; infra INF2-12 | §12.2; §12.3 |
| Safari: dispatched on the change-set branch, running `github.sha` only; no commit input; no cache; the dispatcher named | security N3; infra INF2-15 | §9 |
| The runner's token: a dedicated slot, a 30-minute expiry, bound to a nonce the job makes | security N4 | §10.3 |
| Browser tokens bind at first use, with a resume secret for rejoins; `replaceState` drops the fragment; a local route for the link; a bare Brave profile; bots reuse their slot's token | security N5; netcode NC2-F12 | §10.3; §11 |
| Control-plane identities: systemd template units, an unprivileged supervisor, per-slot sockets with a peer-credential check | security N6 | §10.2; §10.5 |
| LG through RS: first-use trust stated and done at bring-up; `PermitOpen` set after boot; forwarding closed; an RS-made key for LG, restricted | security N7; infra INF2-2, INF2-3 | §1.6; §10.5 |
| FIXED-IP made safe: the IPs created on their own, auto-delete off, no protection; their bill; teardown checks | security N8; infra INF2-4 | §7.3; §16 |
| Egress claims corrected; `npt-cert issue` and `npt-cert revoke`; IPv6 metadata blocked | security N9; infra INF2-8 | §8.2; §8.4 |
| The operator's address never in a committed file; `redact-check` as a pre-push hook, and in this session too | security N10 | §10.6 |
| INVALID only when a control is applied without a capability; a control before Join closes the session | security N11 | §6.1; §10.2 |
| Node, npm, wasm-bindgen and the web build on RS, pinned; the dist's hashes compared with this session's build | security N12; infra INF2-13 | §10.5 |
| The sudo list as root-owned wrappers; a capture wrapper; a GTFOBins check | security N13 | §10.5 |
| Idle hours: the page host stopped, the game ranges dropped | security N14 | §10.1 |
| The ACME client named; a bounded DNS wait; authoritative DNS answers; Cert Spotter beside crt.sh | security N15; infra INF2-9, INF2-10 | §1.6; §8.4 |
| Channels in `Link` and the endpoint; send results; the datagram size; the unsent count | netcode NC2-F1 | §4.7; E7 |
| The WebSocket delay's loss and stall model | netcode NC2-F2 | E15; R-25 |
| The rate limiter's queue depths | netcode NC2-F3 | R-17 |
| Both command-timing options for -42, -48 and -49; the option picked before the rule cap; -49's rule-cap run moved to R7 | netcode NC2-F4 | §6.2; §7.2; §11 |
| A frame loop in the target-course shooter; Chrome's honest rewind checked against the cap | netcode NC2-F5 | E12; §7.2 |
| Three clean reference runs at the start of R3 and R4 | netcode NC2-F6 | §7.2 |
| L2 probes in R2 and R7; one-way delays stamped on RS's clock | netcode NC2-F7 | §7.2; R-6 |
| R2-XU-OQ7's rule is PM-0001-7; the history sized for the test cap | netcode NC2-F8 | R-20; §6.2 |
| L1's step order, client clocks, RTT estimate and fate draws | netcode NC2-F9 | §4.7 |
| npt-server as a library plus a thin binary; decoding on the I/O runtime; every check placed | netcode NC2-F10 | §4.1; §4.4 |
| Per-session controls keyed on the holder's address; L2 settings over the socket; the Worker sends controls | netcode NC2-F11 | §10.2 |
| The catalogue's class column; RoomState | netcode NC2-F12 | §4.1 |
| Reordering in both directions, and on L2 | netcode NC2-F13 | R-5 |
| Cover per item, at present and rewound | netcode NC2-F14 | R-10; E1 |
| The cap read-only on the overlay; the checklist's base round trip and counters | netcode NC2-F15 | R-28; §11 |
| Five accuracy points | netcode NC2-F16 | R-12; §4.6; §6.2; §7.2 |
| The density sessions' order inside each block | infra INF2-1 | §7.2 |
| The day's addresses kept out of the hashed cloud-config; `npt-ctl peers` at bring-up | infra INF2-2 | §8.2; §10.5; R-29 |
| Delete-by times; a teardown slot; account limits | infra INF2-5 | §7.3; §12.2 |
| An hours table that adds up | infra INF2-6 | §7.1 |
| ICMPv6 neighbour discovery and packet-too-big kept | infra INF2-7 | §8.2 |
| The user-data check normalised; the raw-file URL | infra INF2-11 | §7.3 |
| The sshd drop-in's order, nftables in `bootcmd`, label selectors, LG's disk | infra INF2-14 | §10.5 |


### 0.3 The advisor's conditions (APPROVE of revision 3, 2026-11-06) and where each is met
| Condition | What | Met in |
|---|---|---|
| C1 | Main's settings let `git push origin claude/*` run without a prompt, so the laptop's settings add an `ask` rule for every `git push`, in Bash and PowerShell forms. Stop B's guard check adds a fourth attempt: a push to `claude/npt-results` must prompt | §12.2; §12.3 |
| C2 | OQ-0006-19 is Mustafa's: DENSITY-FIGURE | §16; R-19 |
| C3 | "Under way" is undefined: UNDER-WAY goes to Mustafa | §16 |
| C4 | Mustafa approves `ops/operator/settings.json` and the `PROMPT-Rn.md` files explicitly at stop B | §12.2 |
| C5 | Stop B records the tree hash of main's `CLAUDE.md` and `.claude/`. Each run start compares it, and re-runs the guard check if it changed | §12.2; §12.3 |
| C6 | The laptop session's start command states Opus 5.5 at `xhigh` | §12.3 |
| C7 | PM-0006-26 on the runner needs the job limit and the token's life to cover its 30-minute session; PM-0001-32's WebGPU half needs a WebGPU adapter in the runner's Safari | §9; R-13 |
| C8 | Migration step 1 waits for R8's ML and RD rows only | R-14 |
| C9 | R-PR uses the advisor's exact words, adds ADR 0004, R-7, R-9, R-14 (and FIXED-IP's line if (a)), sits on its own branch from main, is needs-mustafa, and merges before the first counted run | §12.2; §15 |
| C10 | §16 states the total against the drafts' estimate | §16 |
| C11 | A fallback after U3 that changes a crate or the transport goes back to the drafts, Mustafa and the advisor; the 90-day certificate needs the advisor's OK too | §8.4; R-2; R-22 |
| C12 | SAFARI-TIME (b) matches Mustafa's own words; PM-0001-36's cap stays provisional until the Mac reading | §16 |
| Minor | docs-keeper adds `spikes/` to REPO_MAP at the first build commit; STATUS's stale "PR #38 in flight" line; R-29: if the laptop's address changes mid-run, Mustafa updates `fw-rs` and the session runs `npt-ctl peers` again | §12.3; §12.4 |


### 0.4 Mustafa's approval and answers (2026-11-06, his words in short)
| Item | His answer | What follows |
|---|---|---|
| The plan | approved: revision 3.1, with the advisor's 12 conditions | the build starts (U0), and R-PR is opened |
| ROUTE | (a), on his laptop: an MSI Vector 18 HX AI (Intel Core Ultra 9, RTX 5080), native Windows with PowerShell, not WSL. Browsers: Brave, plus Edge, which comes with Windows; he can install Chrome or Firefox if the plan needs them. Claude Code is installed (the CLI, under his university's Enterprise login); Manual mode for these sessions | see the notes below |
| SAFARI-TIME | (b): the timed rows re-run when he borrows the Mac for the look proof test | R-14's words; PM-0001-36's cap stays provisional until then |
| FIXED-IP | (a): keep the two addresses; delete them at the close | §7.3's "once" steps apply; FIXED-IP's line goes into R-PR |
| DENSITY-FIGURE | (a): a lower bound, provisional until slice 6 | OQ-0006-19's answer goes into R-PR |
| UNDER-WAY | (a): once the plan is approved and the first build step is committed | the kit freeze lifts, and A4 may start, when U0 is committed |
| Branches | yes: this session may push `claude/npt-readings` (R-PR) and `claude/npt-safari-wf` (W-PR) | both made in separate worktrees, from main |

**Notes on the laptop** (they change no mark):
- **It needs Chrome and Firefox:** the laptop legs run Chrome, Edge and Firefox (PM-0001-28 and the rows of §6.3), so
  Chrome and Firefox, stable, go on the laptop before R1. Their versions are recorded with each run.
- **Its rough numbers read optimistic:** it stands in for ML, "a typical student laptop with integrated graphics", but
  its CPU and GPU are far above that. Its rough checks are never judged, and the named laptop re-runs them in R8.
- **Brave's "Power saving" must really mean the integrated GPU.** If MSI Center offers a GPU switch, it stays in
  hybrid mode during the rough checks; each run records the WebGPU adapter the browser reports.
- **Native Windows has no sandbox,** so only his Manual-mode prompts hold (§12.3, recorded). The operator settings
  carry the PowerShell forms of every deny and ask.
- **Checks at stop B, because of the university's Enterprise login:**
  - the laptop session reports its model and effort: Opus 5.5 at `xhigh` must be available under that login (C6);
  - the guard check shows whether the organisation's managed settings add or override rules.
- **Where its transcripts go:** the laptop session's transcripts belong to his university's Claude organisation.
  Nothing secret enters them by design: the private key is never read or printed, and results pass `redact-check`.

### 0.5 Mustafa's WebSocket decisions (2026-11-07, his words verbatim) and where each is met
About 13:45 UTC:
> WS-SCOPE: no. Measure fix 1 (the newest-snapshot-only buffer) in the proof test. The reconnect becomes a requirement
> for the real client, built and measured before the first online playtest, with today's model as its target (buffer +
> reconnect: 99 % within about 1.05 s). Record that in the docs PR so it can't get lost. RESUME-CRED: deferred with the
> reconnect. WS-BAR: the three-part form now; its numbers after one practice run on the real test setup, but never
> looser than the model predicts for a correct build with fix 1. If the practice run comes out worse than the model,
> find out why before setting the numbers.

About 14:30 UTC:
> Refinement to my WS-BAR rule: the model's bound has almost no margin (5 % slower already fails a correct build), so
> declare a tolerance now, before any practice run. If the practice run is up to 10 % slower than the model, the
> numbers may move by that much; beyond 10 %, investigate first and never loosen. The advisor may set a different
> tolerance with reasons, but it must be fixed before the run.

| Decision | Where it is met |
|---|---|
| Fix 1 (the send budget) measured in this test | **Built:** U3c builds the send gate (`snapshot_budget`, `may_write`, the per-tick writer) on DelayStream, with its mechanism tests (E15). **Wired:** U6 puts it in WsServer's send path, with npt-tcpinfo's kernel values on the plain path. **Measured:** R3 measures it in PM-0001-18's WebSocket part. **Recorded:** ADR 0001 part 10, docs PR #45 |
| The reconnect as a real-client requirement, target 99 % within about 1.05 s of the link's return | Docs PR #45: ADR 0001 part 11, OQ-0001-19, migration step 5 (slice 3) and docs/design/ws-reconnect.md. Not built in this test |
| RESUME-CRED deferred | Docs PR #45 (OQ-0001-19; the reconnect record) |
| WS-BAR: three parts, numbers after one practice run, never looser than the model's fix-1 bound; investigate a worse run | **Recorded:** docs PR #45's PM-0001-18 row. **Before R3:** marks.json's -18 WebSocket part (after #45 merges), and fixes.rs's run-structured bound (before R3's run plan). **At R3:** the practice run. §15 R-30 |
| The 10 % tolerance, fixed before the run | Docs PR #45's PM-0001-18 row: 10 %, which the advisor kept. The bound's guard may lower it, never raise it. marks.json carries it before R3. §15 R-30 |

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
  1. `npt-cert issue` (§8.4) gets the production certificate. A staging dry run comes first, in R1 only.
  2. cloud-init publishes the full `known_hosts` line:
     `https://rs.nt.shamblefrontlines.com:31000/npt/ssh-host-keys.txt`, root-owned, mode 0444, written after the SSH
     module ran.
  3. The laptop session checks over DNS-over-HTTPS that `rs.nt`'s A record equals the IPv4 shown in the Hetzner
     console. With FIXED-IP this is only a typo check.
  4. **Mandatory:** it fetches the file with normal TLS verification. The served certificate must be today's: issued
     after RS was created, and present in CT. It asks crt.sh and Cert Spotter (no account) and waits at most 20
     minutes after issuance; past that, it uses the fallback.
  5. It connects to that IPv4 with `HostKeyAlias`, `StrictHostKeyChecking=yes` and the per-run `known_hosts` file.
- **Fallback:** Mustafa opens RS's console in Hetzner and compares the full fingerprint string, which cloud-init also
  writes on the login screen and the laptop prints in groups of 4.
- **LG** (security N7):
  - LG has no DNS name, and its SSH rule admits only RS's IPv4. The laptop reaches LG by `ProxyJump` through the
    verified RS.
  - LG's key is accepted by first-use trust over Hetzner's internal path. This is stated, not hidden. It happens at
    bring-up, before any room or the page host starts on RS, and every later connection then uses
    `StrictHostKeyChecking=yes`. The same `known_hosts` lines are copied to RS.
  - RS's sshd forwards only to LG's port 22 (`PermitOpen`, set after boot by `npt-ctl peers`, §10.5); nothing else.
- **Out:** trust on first use for RS, and host keys made off the VM and passed in through user data (that would put a
  secret in the paste, and every process on the VM can read user data).

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
  - a lobby, real join tokens, reconnecting, private rooms and economy (ADR 0001:1640-1641);
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
  rustfmt.toml      ADR 0002 part 7's stable options; .cargo/config.toml (builds under .gate/) and .gitignore
  crates/*/clippy.toml  the determinism and clock bans (§4.5), per crate
  check.mjs, check.test.mjs  the test command (§12.4) and its own tests (the advisor's C9)
  crates/link/      npt-link: the thin transport interface (poll-based) and its Loopback backend; no dependencies
  crates/l1/        npt-l1: L1 on simulated time (implements Link): profiles, fate log, per-stream reorder step
  crates/sim/       npt-sim: the stand-in sim behind the seam (§4.6); no I/O, clock, transport or bot logic
  crates/proto/     npt-proto: messages, controls, quantisation, delta codec, limits
  crates/room/      npt-room: the room library: slots and roster, command checks, input buffer (both OQ-0001-6
                    options), interest, hit history and rewind, per-client encoding, event queue; tick-counted
                    timers only; no I/O, async runtime or clock; builds for wasm32 and native
  crates/server/    npt-server: a library plus a thin binary (npt-lab imports the library): the S17 loop, the
                    tick thread, the I/O runtime (where every inbound message is decoded), the wall-time limits,
                    the local control socket, metrics, the panic catch
  crates/net/       npt-net: native adapters only: WtServer and WtClient (wtransport, web-transport-quinn: one
                    feature each), WsServer and WsClient; L2 (ImpairSocket under quinn; the delaying stream wrapper
                    in the WebSocket accept path, DelayStream, on the Linux-faithful TCP model)
  crates/tcpinfo/   npt-tcpinfo (U3c): the kernel's smoothed RTT (TCP_INFO) and unsent plus unacknowledged bytes
                    (SIOCOUTQ) for WsServer's send budget on the plain path; two Linux calls, the workspace's second
                    unsafe exception (§4.2)
  crates/client/    npt-client: the client endpoint library: no bindings, transport or clock; native and wasm32
  crates/client-wasm/  npt-client-wasm: the wasm-bindgen exports only
  crates/hash-wasm/    npt-hash-wasm (U4): a check-only cdylib over npt-sim with integer-only exports, which Node runs
                    for the native-against-Wasm check (§4.5); the workspace's third unsafe exception (§4.2)
  crates/bots/      npt-bots (headless, cheating and hostile clients) and npt-load (the load orchestrator on LG)
  crates/super/     npt-super: the supervisor stand-in (process control, exit log, SIGKILL)
  crates/host/      npt-host: the page host and the ACME client
  crates/log/       npt-log: results-log record types and the JSON Lines writer (serde only)
  crates/judge/     npt-judge: npt-log + marks.json -> one verdict per mark; tests/ (engine, fixtures, cli) and
                    fixtures/ (fixed results logs judged with the real marks.json)
  crates/proto/tests/catalogue.rs  the message catalogue's class audit test
  crates/lab/       npt-lab: the L1 scenario runner (simulated clock, step order, 48 headless clients, seeds),
                    the x86-64 re-decode and replay tool, the native-against-Wasm hash comparer
  fuzz/             cargo-fuzz targets, one per message type
  web/              the client harness: TypeScript, Vite, three.js (exact pins): main.ts, host.ts, net.ts,
                    block.ts, render.ts, overlay.ts, record.ts, smoke.ts, connect.ts; wdio/*.spec.ts
  marks/marks.json  every mark as data: id, statistic, threshold, window, validity, the draft line; marks/README.md
                    (the contract between the results log, marks.json and the judge) and marks/SERIES.md (the names)
  ops/              cloud-init/ (rs.yaml, lg.yaml), firewall/ (rs.txt, lg.txt, each rule per IPv4/IPv6),
                    runs/ (R1.md … R7.md, and the container's run scripts c-*.mjs, such as U2's c-u2-pm-0001-3.mjs),
                    operator/ (settings.json, rules.md, PROMPT-Rn.md, redact-check.mjs and redact-check.test.mjs),
                    checklists/ (netcode.md, operations.md)
  results/          committed verdicts and redacted summaries per run, never raw data: results/<set>/<commit>/ holds
                    the judge's verdict.json and a runs.txt (the commit, the binaries' SHA-256s, one line per run)
docs/design/netcode-proof-messages.md   the message catalogue (ADR 0001:654-678's subset + controls): every
                    type with its channel, visibility class (the draft's Class column), authority and size limit;
                    RoomState listed as omitted, with the reason; a class audit test in U1
```
Builds, raw logs and captures go under `.gate/netcode-proof-test/` (gitignored). `crates/server/` and `crates/net/`
fall under `.claude/rules/netcode.md`.

### 4.2 Dependency rules (enforced by the spike's test command: `cargo tree` checks and one grep)
- **Leaf crates:** npt-link, npt-l1, npt-sim, npt-proto, npt-room, npt-client and npt-log depend on no tokio, quinn,
  wtransport, web-transport-quinn or tungstenite. Each passes
  `cargo check --locked --target wasm32-unknown-unknown`. Clock reads are banned in them by clippy (§4.5).
- **The client:** npt-client never depends on npt-net; the bots get transports by composing npt-client with npt-net.
- **The protocol and the sim:** npt-proto may depend on npt-sim, for the wire grid (one function for the sim's
  own-copy snap and both codec ends) and the seam's plain-data types (ADR 0002:188: the `protocol` crate uses "`sim`
  and `content` types only"; the advisor's Q23 ruling, 2026-11-09, advisor-u4.md).
- **npt-super** depends on no npt-sim, npt-room or npt-proto. **npt-host** depends on none of the sim, room, proto or
  client crates.
- **npt-judge** depends only on npt-log, plus serde ("serde" here and in §4.1 means serde and serde_json). Nothing
  depends on npt-lab.
- **npt-tcpinfo** depends on libc only (pinned =0.2.191, already in the lock through tokio, mio and quinn-udp), and
  only npt-net uses it (U3c; the advisor's C12).
- **Unsafe code** is forbidden in the workspace (`unsafe_code = "forbid"`, the compiler backstop of the advisor's
  ruling of 2026-11-08: no `#[allow]` can relax it in a crate that inherits the workspace's lints), with three named
  exceptions under ADR 0002:264-265's rule ("an exception needs a written invariant, a test and an `#[allow]` with its
  reason"), each carrying its own `[lints]` table equal to the workspace's but for unsafe_code:
  - npt-client-wasm (`allow`), for wasm-bindgen's generated code;
  - npt-tcpinfo (`deny`): its two functions (getsockopt TCP_INFO and ioctl SIOCOUTQ), each `#[allow]` on the function
    only, holding four unsafe blocks of one foreign call each, each with its SAFETY comment;
  - npt-hash-wasm (`deny`; U4, the advisor's Q3 ruling and condition 7, 2026-11-09, advisor-u4.md): the check-only
    cdylib over npt-sim that Node runs for the native-against-Wasm check (§4.5); each listed export carries one
    `#[allow]` for its `#[unsafe(no_mangle)]` attribute, and no `unsafe` block exists; its written invariant: no
    exported name collides with another symbol, and every export has integer signatures only; its test: the Node
    loader lists the module's exports and refuses any not on the list.
  The spike's test command checks all three, and greps that `allow(unsafe_code` appears nowhere else. npt-net stays
  free of unsafe code and libc (the advisor's M14).
- **How they are read** (the advisor's C10): with every feature and every target; dev dependencies do not count (they
  never ship), except for "nothing depends on npt-lab", which counts every kind.
- **No reference into `spikes/`** from outside it: a grep in the test command, until the foundation's G5 and G8 exist.
  It skips docs/, .checkpoints/ and Markdown, whose prose may name the folder; W-PR's workflow file, which runs the
  Safari leg, gets an exception that names it by path, in the same change (C11).
- **Narrow public surfaces:** each crate's public items are those in §4.7; everything else is `pub(crate)`.

### 4.3 Components, each traced to the marks that need it (netcode digest E gives each one's design part)
| # | Component | Crate | Marks |
|---|---|---|---|
| E1 | Stand-in sim, native and Wasm: movement and hitscan; its soldiers driven by protocol clients (npt-bots), with no bot logic in npt-sim; the stress scene's counts (48 soldiers in 3 × 16; 15 big vehicles, 4 of at least 20 parts; 300 live projectiles as trajectory records through one shared function; S1's stand-ins as data behind the seam, §4.6); a DM_Tri-sized greybox with enterable buildings, areas, portals, doors at 1/s, and a destructible-wall stand-in (R-10); the target and correction courses; the authority-only spread stream; a per-tick state hash | sim | -2, -4, -5, -17, -30, -41 to -62 |
| E2 | The room process: the tick thread alone on one vCPU, async I/O on the others, bounded channels only; `panic = "unwind"` for npt-server's builds (the Wasm build keeps the target's abort), a top-level catch → RoomEnded(crash), a crash record, a non-zero exit; per-phase timers and counters; the S17 loop through `due_ticks` (§4.6) | server | -1, -4 to -11, -15, -16, -26, -59; PM-0004-67 |
| E3 | Commands: Input datagrams (tick, sequence, move, buttons, press counters, aim ≤ 80 bits, view time, newest-decoded ack), redundancy up to 8, U2 delta repeats; the checks of §4.4; the input buffer; **both OQ-0001-6 options, switchable** (a start-up setting, `params.cmd_timing`, from the room unit's configuration on L2 or the scenario on L1, recorded in every header and never changed during a run: the advisor's A2); the late-command counter | proto, room | -13, -20, -21, -30, -49, -56, -58 |
| E4 | Snapshots: delta against the last acked (`AckRing`), E3 quantisation, entity groups that each decode alone, the datagram cap (1,000 B until -36 measures), FullState on its own stream, Resync, the bounded event queue, the per-client 256 kbps cap, newest-only over WebSocket | proto, room | -6, -8, -12, -14, -17, -18, -19, -22, -23, -31, -57, -58, -62 |
| E5 | Interest: the region circle with hysteresis on a grid; areas and portals flood-numbered and re-flooded on change; the priority accumulator | room | -8, -9, -10, -12 |
| E6 | Transports under `Link`: WebTransport on quinn 0.11 through wtransport 0.7.x and web-transport-quinn 0.13.x, swappable; WebSocket through tokio-tungstenite 0.30.x; the wall-time limits of §4.4; the per-address table bounded (fixed capacity, eviction, a counter, tested at 100,000 synthetic addresses); `wss` only; NewReno or Cubic (a start-up setting, `params.cc`, like `cmd_timing`: A2); the version floors; refused handshakes counted per reason | net, server | -11, -14, -24, -25, -28 to -38 |
| E7 | The client harness, in ADR 0002 part 5's shape (ADR 0002:313-373), for the boundary spike to keep: see the list below this table | client, client-wasm, web | -19, -28, -29, -31 to -33, -36, -41 to -46, -51 to -53, -61, -62 |
| E8 | Prediction and reconciliation (replay newer commands, 100 ms fade, 2 m snap, cues keyed once) | client | -41 to -44, -60, -62 |
| E9 | Interpolation: buffer in the Worker; delay from 3 intervals, adapting between 2 and 5; extrapolation at most one interval, then hold; placed respawns; events released with their poses | client | -45, -46, -61 |
| E10 | Clock: Ping/Pong with hold time, answered on the I/O runtime; samples stamped at datagram arrival by the host | client, server | -33, -47, -50 |
| E11 | Lag compensation: `HitHistory` (at most 1 MB), `rewind_at`, the view-time tolerance, the switchable cap (a test cap above every honest rewind, then the rule cap), a per-shot log of its parts | room | -7, -48 to -51 |
| E12 | Headless bots on npt-client with npt-net's clients: 30 inputs per second, the brawl; the target-course shooter running a page-like 60 Hz frame loop with a seeded phase, posting input and view time each frame (so the honest rewind includes the frame-to-tick wait); the cheats (forged kinds, 1.5× clock, lag switch, a 150 ms pong delayer, the tolerance-edge stamper, the never-acker); raw byte logs for re-decoding, kept on LG and run only in -17's and -19's blocks, with free disk checked first | bots | -2, -17, -19 to -22, -48, -50, -54 |
| E13 | Load orchestration: clients over the CAX21s; the 5-minute calibration (under 80 % CPU every second); inputs per second within 1 %; the metrics role on one CAX21 | bots | validity of every LG row; -25 |
| E14 | Hostile clients and fuzzing: 1,000 sessions that never join, oversize datagrams and stream openers, slow-loris on the game ports and on TCP 31000 and 80, a 1,000-address /64 cycler, `ws://` and wrong-Origin pages; cargo-fuzz per message type with a counting allocator | bots, fuzz | -25, -34 |
| E15 | The link simulator: L1 (npt-l1: seeded, simulated time, fate log, the per-flow reorder step of R-5) and L2 (npt-net: impairment under quinn's socket, per peer, with one-way delays stamped on RS's clock; rate limit and bottleneck with R-17's queue depths; per-peer profiles; B1; UDP block). The WebSocket delay is a stream wrapper inside npt-server's accept path (R-25), so the room sees real source addresses. **The model** is Linux 6.8's TCP at both ends, to the advisor's spec S0-S14 (Mustafa's U3-Q3 (b), 2026-11-06). U3a built it as `linux-6.8-tcp v3`, which three skeptic rounds checked line by line against the kernel source:<br>• segments of at most 1,448 B, each with its own delay, jitter and loss;<br>• delayed and selective acknowledgements;<br>• Linux's RTT estimator and RTO: srtt + rttvar, rttvar never below 200 ms, doubling;<br>• RACK, the tail-loss probe, F-RTO and the undos, and Cubic;<br>• bytes released in order;<br>• the write side bounded at 64 KB (ADR 0001:442), returning Pending beyond;<br>• each TLS record counted as plaintext + 22 B.<br>**DelayStream** (U3c) runs it per connection from a dedicated release clock, above TLS and below tungstenite (U3-Q4 (b)). **The send budget, fix 1** (ADR 0001 part 10 after docs PR #45): a tick's whole snapshot is written only if all three hold (fixes.rs:777's conditions, W22); otherwise it is skipped, counted, and never written later.<br>• nothing is held above the kernel socket;<br>• the stream's 64 KB room takes it;<br>• the unacknowledged bytes plus it stay within N snapshots, N = ⌈1.5 × srtt ÷ tick⌉ + 1, at least 3.<br>srtt and the unacknowledged bytes come from DelayStream's model on L2, and from the kernel through npt-tcpinfo on the plain path. Stalls are counted per run, by repair path. Every run records each stream's reorder share and stall share | l1, net | -3 and every P1–P4/B1 row |
| E16 | Density harness: 8 room processes unpinned under the supervisor, bracket B, SIGKILL, 384 sessions | super, server, bots | -27; PM-0006-13, -14 |
| E17 | Recordings and hashes: input recordings that drive both transports; the server's per-tick state hash; the client-held hash, without authority-only streams; the native-against-Wasm comparison (§4.5); the x86-64 re-decode and replay (npt-lab) | sim, lab | -17, -19, -21, -30, -48, -54, -57, -58 |
| E18 | Instrumentation: monotonic reads at tick and phase edges; clock resolution per machine; per-thread CPU and RSS each second; queue lengths; bytes per client counted at UDP level (payload + 28 B, from the socket wrapper); packet capture for fragments; steal and affinity reads | server, net | -1, -4 to -16, -24, -25 |
| E19 | Bites: one planted defect per automated mark (test-engineer digest F, plus R-12), compiled only into the `bites` build and chosen at run time (§4.7) | all | every automated row |
| E20 | Results: npt-log's JSON Lines (a header per machine, then per-tick, per-second and event records), apart from the tracing logs; npt-judge | log, judge | all |
| E21 | Hosting and certificates: npt-host serves only allow-listed files (§10.4), with **no COOP/COEP by default**; a separate path, `/iso/`, adds the two headers for the rows that run with the isolation flag on (PM-0001-43 and -44: ADR 0001:1479-1481, 1562-1563), and every browser run records `crossOriginIsolated` as the Worker reads it; the WebTransport development certificate (§8.5); the `wss` certificate by ACME (§8.4) | host, server | -25, -28 to -32, -36, -43, -44, -53; PM-0006-15, -25 to -27 |
| E22 | Metrics and logs: `metrics` with `metrics-exporter-prometheus` on RS, bound to RS's public IPv4, scraped by Prometheus on LG; bounded labels; `tracing` JSON lines with no client-supplied string verbatim (a length or hash, or escaped and cut to 64 B); the canaries of PM-0006-21 | server | PM-0006-16, -19 to -24 |
| E23 | The control plane (security F1, N6, N11; netcode NC2-F11): room-wide controls (rewind cap, per-address cap raise for the listed LG addresses, panic and stall injections, roster, L2 settings per peer set: LG's profile, -23's throttle, -24's bottleneck, -18's bursts, -29's UDP block) only through the per-slot local Unix sockets on RS (§10.2); per-session controls for one browser session (profile, B1, UDP block, the record form) only with a per-run capability. The overlay posts a control to the Worker, which sends it on the control stream | server, room, web | -18, -23 to -26, -29, -45, -46, -48, -49, -53, -59; PM-0004-67; PM-0006-23 |

**E7, the Worker host's shape** (engine EA-4; ADR 0002:313-373). Built now for the boundary spike to measure; none of
PM-0002-* is judged here.
- **npt-client** (no bindings, no transport, no clock):
  - `ClientEndpoint::input(page_input, received: &[(Channel, bytes, arrival_ns)], prev_send_ns, now_ns)`;
  - `ClientEndpoint::step() -> StepOut { datagrams, reliable: [(Channel, bytes)], block, next_due_ns }`, every time in
    integer ns (the advisor's Q19 ruling, 2026-11-09, advisor-u4.md). Still one input call and one step call per tick.
    A message on the wrong channel is dropped and counted as forged (-21);
  - `AimCast`: the aim cast against the state published for the page's view time;
  - `BlockWriter`, with one layout table.
- **npt-client-wasm:** the wasm-bindgen exports only: create, mode, input, step.
- **web:**
  - `net.ts`: the Worker's WebTransport first, with a 1.5 s timeout, then WebSocket. It uses `requireUnreliable`,
    reads `maxDatagramSize` every 10 s, and stamps `performance.now()` in each datagram's own event;
  - `host.ts`: the timer set from `next_due_ns`, at most 7 ticks per wake (from the tick constant, §4.6), a fresh
    `memory.buffer` view after each call, and a Worker restart plus rejoin after a trap;
  - `block.ts`: the page's typed-array views, keeping the two newest blocks;
  - `main.ts`: the page posts the newest input, the cursor ray and the view time once per drawn frame;
  - `render.ts`: the plain renderer on both backends (three.js's WebGPU renderer and its WebGL 2 path; not the look);
  - `overlay.ts` and `record.ts`: the overlay and the record form;
  - `smoke.ts`: the smoke page; `connect.ts`: the 100-session connect page (-28).
- **Design rules:** no per-entity call, at most 4 Wasm calls per tick, and the page never touches a packet. The
  boundary spike counts them later; they are not judged here.

### 4.4 Where each check lives (engine EA-1; netcode NC2-F10; ADR 0002:189, 457-459)
- **Decoding:** every inbound message is decoded on npt-server's I/O runtime, and typed messages reach the tick thread
  over bounded channels. npt-room accepts only decoded messages, so the sim thread never parses raw bytes. Encoding
  stays on the tick, as -6's share.
- **Each check gets a home and a -21 counter:**

| In npt-room (counted in ticks; no clock) | In npt-server (wall time, from an injected `now`) |
|---|---|
| strictly newer tick; the lead bound; commands per client per tick within the redundancy cap (`CMD_CATCHUP`, §4.6); the clamp beyond reach (in the sim); sequence forward and wrap; move length at most 1; one press edge per tick; aim inside the map bounds; cooldown, ammo and reload (in the sim); the acknowledgement names a sent snapshot; one Resync in flight; the ClientSettings clamp; the view-time tolerance; input buffer depth and the late-command counter; the event queue's cap, with resync or drop; the per-client 256 kbps cap per tick window; interest hysteresis; the rewind cap in ticks; the roster and the slot's token (§10.3) | the Join deadline (2 s from TCP accept or the first QUIC packet, never from the end of a delay); the handshake cap; the bounded per-address table (/32 and /64); slow-loris deadlines; size limits at the socket (datagram, stream frame, WebSocket message, applied before any delay buffering); trailing bytes and unknown fields; the wrong channel; per-connection and new-session rate limits; ping rate and the hold-time cap; stream count and windows; the Join's version and content hash; the Origin check; the capability check of per-session controls; the S17 loop; the control sockets; the panic catch |

- **What npt-room gets from npt-server each tick:** the decoded messages with their arrival stamps, and per client the
  RTT, the bytes the server side sent since the last tick (pings and pongs, each counted as payload + 28 B, so the
  room's 256 kbps window counts them: the advisor's Q13 ruling), maximum datagram size and unsent count; loss and
  send-buffer space join at U6 with rate adaptation, their first consumer (the advisor's Q6 ruling, amending its C3;
  both 2026-11-09, advisor-u4.md).
- **WebSocket's send budget (fix 1; E15):** WsServer in npt-net applies it in its send path through U3c's shared
  functions, never in npt-room. npt-room sees `SendResult::BufferFull` and the skipped count. DelayStream's free budget
  is the send-buffer space it reports.
- **On L1:** npt-lab drives the npt-server library and npt-room on its simulated clock, since the clock is always
  injected. So the L1 hostile rows (-21, -22) run on simulated time.

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
- **One tick constant:** `TICK_HZ` is one build-time constant. dt, the loop's catch-up cap `LOOP_CATCHUP` =
  ⌊0.25 × TICK_HZ⌋ ticks per wake (7 at 30 Hz, 15 at 60 Hz), the snapshot interval and every tick-counted timer derive
  from it; no literal 7 anywhere.
- **The other catch-up cap is separate:** `CMD_CATCHUP` limits commands per client per tick to the redundancy cap of
  8. It is not derived from TICK_HZ (netcode NC2-F16).
- **The S17 policy** is a pure function, `due_ticks(now, schedule) -> (to_run, dropped)`. It is unit-tested in this
  container on a fake clock, with PM-0004-67's bites (timer-period counting, an uncapped catch-up, an uncounted drop),
  before any VM hour.
- **The 60 Hz run's own change-set** replaces npt-sim through this seam and changes no other crate.

### 4.7 Symbols to add (each crate's whole public surface; nothing is removed or renamed anywhere)
- **npt-link** (netcode NC2-F1):
  - `Link`, poll-based:
    - `send_unreliable(bytes, now) -> SendResult { Sent, BufferFull, TooLarge }`, counted, which feeds rate
      adaptation and -24's bite;
    - `send_reliable(channel, bytes, now)`;
    - `poll_recv(now) -> Option<Recv { channel: Datagram | Control | FullState, bytes, arrival_ns }>`;
    - `state -> LinkState { Connecting, Open, Closed }`, `rtt_ns`, `max_datagram_size`, and `unsent_unreliable`.
      WebSocket's newest-only replace sits inside its backend; `loss` joins at U6 with rate adaptation, its first
      consumer, and §4.4's send-buffer space with it (the advisor's C3, 2026-11-06, as amended by its Q6 ruling of
      2026-11-09, advisor-u4.md);
  - `Listener`, which also yields each peer's address;
  - `Channel` and `Loopback` (as built in U2; the advisor approved the names, U2-R4).
- **npt-l1:** `L1Net`, `Profile`, `FateLog`, `ReorderStep`.
- **npt-sim:** `SimCore` (the seam trait, with its plain-data input and output types; moved from npt-room: the
  advisor's Q1 ruling, 2026-11-09, advisor-u4.md), `World` (implements the seam), `HitHistory` (the ring of past hit
  volumes, owned by the room and lent to `step`; moved from npt-room: the advisor's Q2 ruling), `Scene::stress`,
  `Greybox`, `Areas`, `Portals`, `SimRng` (crate-private: no other crate draws from the sim's streams; the advisor's
  W1-PUB ruling), `trajectory_at`, `StandIns`.
- **npt-proto:**
  - the messages: `Input`, `Snapshot`, `FullState`, `Resync`, `Ping`, `Pong`, `Join`, `JoinAccepted`, `JoinRejected`,
    `ClientSettings`, `RoomEnded`, `Leave`, `Kick`, and `SessionStatus` (room → client, owner, to the capability
    holder: the overlay's room-side values and the form's last saved step; the advisor's A1 ruling, 2026-11-06, with
    its own fuzz target);
  - the session controls (§10.2): `SessionControl`, with `SetProfile`, `TriggerBurst`, `BlockUdp` and `RecordStep`;
  - the room-wide controls (§10.2): `RoomControl` and its decoder, unfeatured (npt-client never calls them), built at
    U4 for the catalogue's item 6 and U5's fuzz target (the advisor's Q7 ruling, 2026-11-09, advisor-u4.md);
  - the rest: `encode`, `decode`, `Quant`, `Limits`.
- **npt-room:** `Room` (generic over npt-sim's `SimCore`; it owns the `HitHistory` instance), `Roster`,
  `ClientSlot`, `InputBuffer` (option A or B), `EventQueue`, `AckRing`, `Encoder`, `InterestGrid`, `AreaFlood`,
  `Priority`, `rewind_at` (the advisor's Q1 and Q2 rulings, 2026-11-09, advisor-u4.md).
- **npt-server** (library): `due_ticks`, `Schedule`, `TickLoop`, `PhaseTimers`, `Counters`, `ControlSocket`,
  `ConnLimits`, `AddrTable`, `Decoder`; plus its thin binary.
- **npt-net:** `WtServer`, `WtClient`, `WsServer`, `WsClient` (features `wt-wtransport` and `wt-wtq`),
  `ImpairSocket`, `DelayStream`, and `L2Config`: the one small L2 configuration type (Mustafa's U3-Q7, 2026-11-06: it
  reuses npt-l1's `Profile` and `ReorderStep`; the advisor's M6: listed here before the code). Everything else in
  npt-net stays pub(crate).
  - DelayStream's public items (U3c, the advisor's C5): `wrap`, `now_ns`, the AsyncRead and AsyncWrite impls, the
    burst method, and its header, per-second and summary rows as std types.
  - L2Config gains the fields `ws_max_message_bytes` and `ws_max_request_bytes`.
- **npt-tcpinfo** (U3c): `srtt_us` and `unacked_and_unsent`, Linux only; elsewhere a stub returns Unsupported.
- **npt-client:** `ClientEndpoint`, `StepOut`, `Predictor`, `Interp`, `ClockSync`, `AimCast`, `BlockWriter`, `CueLog`.
- **npt-client-wasm:** the exports create, mode, input and step.
- **Binaries:** npt-bots, npt-load, npt-super, npt-host, npt-judge, and npt-lab with its `probe` subcommand (U2;
  the scenario runner, the replay and the hash comparer join it in U4).
- **npt-log:** `Header`, `TickRecord`, `SecondRecord`, `EventRecord`, `Writer`; their field types `Machine`, `Build`,
  `Phase`, `Ended` and the `Footer` record; `SCHEMA`; the reader `read_log`, `Log`, `ReadError` (the advisor's C8).
- **npt-judge's library** (for its own binary and tests only): the modules `marks`, `eval`, `ratio`, `figures`, and
  `log_files`, `read_logs`, `summary_line` (C8).
- **npt-lab:** `SimClock` (integer ns), `L1Scenario`, `ReplayX86`, `HashCompare` (netcode NC2-F9).
  - **Each simulated instant, in this order:**
    1. packets due, ordered by arrival time and then fate-log index, never delivered in the instant they were sent
       (P0 included);
    2. the server's I/O step: decoding, socket checks, ping answers;
    3. the server tick if due: tick k is due at base + ⌊k × 10⁹ / TICK_HZ⌋ ns;
    4. the client frames that are due (60 Hz, seeded phase);
    5. each client whose `next_due_ns` has come, in slot order.
  - **Each client's own host clock:** offset + rate × SimClock, both seeded; rate 1.5 for -21's cheat.
  - **L1's RTT** is an RFC 9002-style smoothed estimate, not the simulator's true value.
  - **Packet fates** are drawn per (seed, link, direction, packet index), so both command-timing options see the same
    losses.
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
- **U3c's additions to the audits:**
  - tokio-rustls =0.26.6 (default features off, `ring`; network-facing, so on the list above);
  - npt-tcpinfo's four unsafe blocks (in its two functions), with their SAFETY invariants and tests;
  - DelayStream's request and frame parser as a fuzz target (-34).

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
- **Two more INVALID rules (security F1, F3, N11):** a measured run is INVALID if it admitted any session outside its
  roster, or if a control was **applied** without a valid capability. A rostered session sending a control without
  one also counts, since that is a bug in our code.
  - Session controls are parsed only on a joined, rostered session.
  - A control-type message before Join closes that session as malformed: counted by reason, never INVALID, so an
    outsider cannot void a run.
  - Every control and form record carries its source.
- **The results log is not a shipped log** (security F18). It is written apart from the tracing output, so
  PM-0006-21's grep runs over the tracing logs, and the seed in the results header does not breach it.
- **Fixtures:** the checker is tested on fixed fixtures that must come out PASS, FAIL and INVALID.
- **A run's precedence** (the advisor's C2): its validity rules first, then its setup checks (a failure or gap makes
  it INVALID), then its checks: any failing check fails the run, its gaps listed beside the failure; with no failure,
  a gap makes it INVALID. A count of an event that never happened is 0; every other empty series is a gap.
- **Bites first:** every automated mark's planted defect must FAIL before its pass counts. A bite that targets a
  validity rule (`expect: "invalid"`, PM-0001-30's "the validity check itself") is shown only when its run is INVALID
  by that rule alone; a valid run makes it missed and the mark FAIL (B13).
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
| -4 to -9, -11 to -13, -15 | R2 (VS2): 3 × S30 on one vCPU of RS; -12's ES runs 3 × 10 min; -13 also at P4 | bites by rules B-S and B-N; -7's history is sized for the largest cap the test uses (-48's test cap, fixed in U4 from L1's largest honest rewind at P3 and P4) plus the margin as data, recorded; the rule cap can only shrink it, so -7 needs no re-run |
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
| -36 | its loopback legs on the laptop before R1 (a local server, no VM: R-7); its home EU path in R1, the three browsers at once; Safari on MAC-b | sets -14's cap. R2 to R7 run at 1,000 B until all four browsers have a reading; a later reading under 1,000 B makes the size and bandwidth rows re-run; the cap is final only once the Mac reading exists |
| -41, -42, -45, -60, -61, -62 | C (L1) + R7 (ML\* Chrome over L2; -42 and -61 with 47 bots) | -42 under both OQ-0001-6 options at P2 to P4 (netcode NC2-F4) |
| -43, -44 | R7: ML\* through `/iso/` (the isolation flag on); RD later (nothing stands in for RD) | named machines only |
| -46, -51, -52 | C (-51) + R7 (ML\* Chrome; -52 also in Mustafa's session) | |
| -48, -49 | C (L1, both options) + R3 (VS5: the test-cap runs under both OQ-0001-6 options); then this container picks the option by its rule and the advisor confirms; -49's rule-cap run opens R7 | -48 Arm rule; the rule cap is the picked option's honest-rewind p99 at P2, rounded up to whole ticks; it feeds checklist steps 7 and 10 |
| -53 | R7: Mustafa, Brave in a separate profile on his PC, at P2 (§11) | judged by Mustafa |
| -54, -55 | C (L1) + R3 (VS5: -54 on L2) | R-11; -54 Arm rule |
| -56 | every run | the record is complete; R-10's cover item |
| -39, -40, -63 | not in this change-set | the 60 Hz run |
| PM-0004-67 | R2 (VS3) | R-15 |
| PM-0006-11 | every run | |
| PM-0006-12 | R1: `ping` from the laptop and from the PC to LG | reported |
| PM-0006-13 | R5 (D1, D2, D3) | the same runs as -27 |
| PM-0006-14 | R5 and R6: each N's search, then count and count + 1 at the other two times of day (§7.2) | DENSITY-FIGURE answered first (§16) |
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
- **Planning check** (PM-0006-18, arithmetic shown): 75 RS-hours × 0.1386 = 10.395, plus 177 LG-hours × 0.0176 =
  3.1152, so **EUR 13.51**. The caps add up to EUR 14.90. With FIXED-IP (a), add EUR 1.00 for each month the two
  addresses are kept.
- **Against the drafts, part by part** (infra INF2-6; the drafts estimate 38 to 53 RS-hours, ADR 0006:1107-1113):

| Part | RS-hours | Where |
|---|---|---|
| ADR 0001's rows, measured, bites apart | 19.3 | R1 0.6, R2 3.5, R3 6.55, R4 1.65, R7 7.0 (the drafts guessed 15 to 30) |
| ADR 0006's rows | 27.35 | R1 0.1, R2 2.6, R4 1.4, R5 12.85, R6 10.4 (the drafts: about 23; the three-times-of-day rotation adds the rest) |
| ADR 0004's PM-0004-67 | 0.2 | R2 |
| Bites | 5.7 | R1 0.1, R2 3.3, R3 1.7, R4 0.6 |
| Bring-up, with the dist build | 6.5 | 0.9 per run, 1.0 on the density days |
| R1's set-up items (certificate, headers, calibration, smoke page, runner) | 1.05 | R1 |
| L2 probes (R-6) | 1.0 | R1 0.2, R2 0.2, R3 0.4, R7 0.2 |
| Clean references and issuance | 1.1 | R3 0.6, R4 0.4 + 0.1 |
| Idle and teardown on the density days | 7.5 | R5 1.85 + 0.55, R6 4.6 + 0.5 |
| Rounding slack | 5.3 | R1 1.05, R2 0.3, R3 0.85, R4 0.95, R5 0.75, R6 0.5, R7 0.9 |
| **Total** | **75.0** | |

### 7.2 The runs (provisional; each gets its own plan, `ops/runs/Rn.md`, for Mustafa's OK first)
| Run | Blocks, with the working in hours | Mustafa | VMs | RS h | LG h | Planned EUR | Cap EUR |
|---|---|---|---|---|---|---|---|
| R1 bring-up | bring-up 0.9; certificate (staging, then production) and host keys 0.25; headers (-1, -2, PM-0006-11) 0.15; L2 probe 0.2; calibration 0.1; -16 0.1; five-browser smoke page 0.25; ping 0.1; Safari runner conditions 0.3; -36's home EU path, three browsers at once, 0.5; first bites 0.1 → 2.95, first-day slack to 4 | present, about 2 h | RS + 1 LG | 4 | 4 | 0.62 | 0.80 |
| R2 timed (VS2 + VS3; unattended) | bring-up 0.9; L2 probe 0.2. VS2: 3 × S30 1.65; -10 1.1; -12 ES 0.55; -13 at P4 0.2; bites 2.3 (none shared). VS3: PM-0004-67's stall runs 0.2 and bites 0.25; PM-0006-17 1.0 and bite 0.35; PM-0006-19 1.2 and bite 0.2; PM-0006-20 0.4 and bite 0.2 → 10.7, to 11 | start and end | RS + 1 LG | 11 | 11 | 1.72 | 1.90 |
| R3 behaviour on L2 (VS4 + VS5; unattended) | bring-up 0.9; two L2 probes 0.4; 3 clean reference runs 0.6. VS4: -17 0.75; -18 0.2; -19 0.55; -20 0.35; -23 0.15; -24 0.35; -31 0.2; -37 0.2; -38 0.25; bites 0.85. VS5: -26 0.55; -30 0.5; -48 and -49's test-cap runs under both options 1.0; -54 0.3; -59 1.2; bites 0.85 → 10.15, to 11 | start and end | RS + 1 LG | 11 | 11 | 1.72 | 1.90 |
| R4 hostile input and certificates (VS6) | bring-up 0.9; issuance 0.1; 3 clean reference runs 0.4; -25's four floods 0.85; slow-loris, handshake cap and /64 cycler 0.3; Origin 0.25; -29 0.25; PM-0006-15 0.5; PM-0006-25 0.2, -26 0.55, -27 0.15; the runner's rows in parallel; bites 0.6 → 5.05, to 6 | present, about 2 h | RS + 1 LG | 6 | 6 | 0.94 | 1.10 |
| R5 density day 1 | bring-up 1.0. D1 08:00: -27 1.2; the N = 48 search 2.6; the 8-room ops soak 0.75; idle 1.45. D2 14:00: -27 1.2; N = 48's count and count + 1 1.5; the N = 36 search 2.9; idle 0.4. D3 20:00: -27 1.2; N = 48's count and count + 1 1.5 (ends about 22:42). Teardown 0.55 → about 16.25, billed 17 | blocks scheduled on RS; the laptop checks each | RS + up to 4 LG | 17 | 68 | 3.55 | 3.80 |
| R6 density day 2 | bring-up 1.0. D4 08:00: the N = 18 search 4.4; N = 36's count and count + 1 1.5; idle 0.1. D5 14:00: N = 18's count and count + 1 1.5; idle 4.5. D6 20:00: N = 18's count and count + 1 1.5; N = 36's count and count + 1 1.5 (ends about 23:00). Teardown 0.5 → about 16.5, billed 17 | the same | RS + up to 4 LG | 17 | 68 | 3.55 | 3.80 |
| R7 the human run (VS7) | bring-up 0.9; L2 probe 0.2; -49's rule-cap run and the Chrome honest-rewind check 0.7; the ML\* legs about 5.3 (-44's 12 runs of 10 min take 2.0 of it); then his checklist 1.0 → 8.1, to 9 | present for the checklist, about 1.5 h at the end | RS + 1 LG | 9 | 9 | 1.41 | 1.60 |
| **Total** | | | | **75** | **177** | **13.51** | **14.90** |
| R8 later (own change-set) | the named-machine re-run; the Mac if SAFARI-TIME (b) | present | RS + 1 LG | 3 | 3 | 0.47 | 0.70 |

**Density's time-of-day rule** (infra INF2-1; ADR 0006:1314-1316, 1352). Each mark gets three starts, at least 6 h
apart, and one session wholly inside 18:00 to 23:00 Amsterdam:

| Mark | First start | Second | Third | Evening session |
|---|---|---|---|---|
| -27 and PM-0006-13 | 08:00 (D1) | 14:00 (D2) | 20:00 (D3) | 20:00–21:12 |
| N = 48 | the search, 09:12 (D1) | 15:12 (D2) | 21:12 (D3) | 21:12–22:42 |
| N = 36 | the search, 16:42 (D2, day 1) | about 12:24 (D4, day 2) | 21:30 (D6) | 21:30–23:00 |
| N = 18 | the search, 08:00 (D4) | 14:00 (D5) | 20:00 (D6) | 20:00–21:30 |

- The run plans write each session's start time, and npt-judge checks the rule from the results' UTC times.
- Each count-and-count+1 block keeps its own bracket B (about 1.5 h each).
- The results say that each N's sessions shared one host (R-26).
- The CPU model is re-read in every block (PM-0006-11).

**Order rules** (netcode NC2-F4 to -F8, -F16):
- **Clean references:** R3 and R4 each open with 3 clean 10-minute runs that match the compared rows' N, profile,
  transport and build, within 2.5 % of their median. A reference never comes from another run's VM.
- **The command-timing option:** R3's -48 and -49 test-cap runs cover both OQ-0001-6 options. This container then
  judges them and picks the option by its rule, using -20, -21, -41, -42 and -60 on L1 and -48. The advisor confirms
  the pick. The rule cap is that option's honest-rewind p99 at P2, rounded up to whole ticks.
- **R7's order:** it opens with -49's rule-cap run. Then the laptop's Chrome honest-rewind p99 at P2 is compared with
  the cap; above it, the run stops and goes to the advisor. Then comes the checklist, under that option and cap.
- **The datagram cap:** R2 to R7 run at 1,000 B until all four browsers have a `maxDatagramSize` reading.
- **DENSITY-FIGURE first:** Mustafa's answer (§16) comes before R5's run plan is written. Under option (a) the pad
  caps k at about 15 rooms on 4 vCPUs, which sets the search lengths.
- **Shared bites:** a run plan says which bites share a run. R2's 2.3 h assumes none do.

**Deadlines** (infra INF2-5):
- Each run plan states its creation time, its delete-by time (creation plus the planned hours, with the spare hour as
  the margin), and Mustafa's teardown slot. The laptop session warns 30 minutes before the delete-by time.
- On the density days the extra CAX21s are deleted right after the last block that needs them.
- If a teardown near 23:30 does not suit him, a density day can open with its evening block instead. The VMs then stay
  up overnight, for about EUR 1.30 more.

- **The hours are rough** until each run plan fixes them; how many CAX21s comes from each run's calibration.
- **A run about to pass its cap** stops and comes back to Mustafa.
- **Server-only runs can be merged or split** if he prefers; the hours and caps add up the same.

### 7.3 Mustafa's steps per run (each run plan repeats them with the day's values; security F2, F11, F19, N8; infra INF2-4, -9, -11, -14)
0. **Once:**
   - confirm two-factor login is on for the Hetzner and Cloudflare accounts;
   - before R1, read the project's Limits tab (servers, cores, Primary IPs), and ask for an increase if RS plus five
     CAX21s and two Primary IPs do not fit;
   - with FIXED-IP (a): create two Primary IPv4s in `fsn1`, named `npt-rs` and `npt-lg`, with auto-delete off and
     protection off, and add the DNS record (§8.3) for `npt-rs`'s address once.
1. OK the run plan: its hours, its cap and its delete-by time.
2. In the two firewall objects kept in the project, set the day's sources. Each object has a label selector as a
   second binding (`role=rs`, `role=lg`).
   - `fw-rs`: SSH from the laptop's public IPv4, typed only here and never in a file;
   - `fw-lg`: ICMP from the laptop's and the PC's IPv4.
3. Create LG: CAX21, `fsn1`, Ubuntu 24.04, the operator key, and `fw-lg` with the label `role=lg` in the create form.
   With FIXED-IP, pick `npt-lg` in the Networking section. Paste the Cloud config from the raw-file URL the run plan
   gives, which names the commit SHA.
4. In `fw-rs`, set the metrics rule's source to LG's IPv4 (once, with FIXED-IP).
5. Create RS the same way: CCX23, `fw-rs`, `role=rs`, and `npt-rs` with FIXED-IP. On density days, create the extra
   CAX21s like LG, without a kept address.
6. **As soon as RS shows its IPv4:** add the DNS record (§8.3; not needed with FIXED-IP). Then, in `fw-lg`, set the SSH
   source to RS's IPv4 (once, with FIXED-IP).
7. Start the laptop session with `ops/operator/PROMPT-Rn.md`. Its first commands:
   - **the leftover check.** With FIXED-IP: `rs.nt` points at the kept address, which exists and was unassigned
     before creation. Without it: no `*.nt` record is left over;
   - **the user-data check:** each VM's stored user data is compared with the run plan's file, normalised for line
     endings and the final newline, and read through a sudo wrapper. So is `git rev-parse HEAD`. A mismatch prints a
     diff and stops the run;
   - the host keys (§1.6);
   - `npt-ctl peers` (§10.5).
8. Do the browser steps when the run has them.
9. **Teardown,** inside the run plan's slot and before its delete-by time:
   1. the laptop session collects the results and runs `npt-cert revoke`;
   2. without FIXED-IP: Mustafa deletes the DNS record first, and the laptop session confirms NXDOMAIN over
      DNS-over-HTTPS;
   3. Mustafa deletes every VM, first checking that no kept Primary IP shows the auto-delete (bin) icon.
   After the last run he reads the project's costs (PM-0006-18), the kept addresses included. At the close: the record
   first (NXDOMAIN confirmed), then the kept addresses.

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

### 8.2 Firewall rule lists (inbound; `ops/firewall/`, each rule per IPv4 and IPv6, the operator's address only as a placeholder; security F10, N9, N14; infra INF2-2, INF2-7)
| Role | Hetzner firewall (`fw-rs`, `fw-lg`) |
|---|---|
| RS | UDP 30001–30032 from any (v4 and v6) · TCP 31000–31032 from any (v4 and v6) · TCP 80 from any (v4) · TCP 9100 from LG's IPv4 /32 · TCP 22 from the laptop's IPv4 /32 · no ICMP echo (ADR 0006:1350) |
| LG and each extra CAX21 | TCP 22 from RS's IPv4 /32 · ICMP from the laptop's and the PC's IPv4 /32 (PM-0006-12) |

- **The VM's own wall:** nftables `table inet`, default drop, written inline in `bootcmd`, so it loads before any
  service. It mirrors the lists, with differences that follow from the cloud-config holding no address (R-29):
  - TCP 22 is accepted from any source in nftables. The Hetzner firewall holds the /32, and sshd's own rules
    (key-only, `AllowUsers`, `MaxAuthTries`) are the second layer;
  - TCP 9100 is accepted only from a named set, `metrics_src`, which starts empty and is filled at bring-up by
    `npt-ctl peers` (§10.5);
  - with FIXED-IP (a), the kept VM addresses may be written into the file instead.
- **ICMP on the VM:** nftables also accepts ICMPv6 neighbour discovery (the three NDP types) and packet-too-big, and
  `ct state established,related`; it drops invalid packets and accepts no echo request. So "no ICMP on RS" still means
  no ping (infra INF2-7). R1 records one IPv6 fetch from each VM.
- **Outbound:** after build and issuance, nftables drops new outbound connections from the slot users and `npt-host`
  (`meta skuid`), except established ones, and their access to 169.254.169.254 and fe80::a9fe:a9fe.
  - That stops callbacks, and attacks on other hosts, from a compromised room.
  - It does not stop a compromised room from returning the `wss` key on its attacker's own inbound session. The key is
    limited by its 6-day life, the DNS removal and the revocation (security N9).
- **Idle hours on the density days:** nftables drops the game ranges, and the page host is stopped (security N14).
- No private network is attached.

### 8.3 DNS records (Cloudflare, zone `shamblefrontlines.com`; every record "DNS only", grey cloud)
| Host | Type | Name, as typed | Value | TTL | Proxy status | When |
|---|---|---|---|---|---|---|
| RS | `A` | `rs.nt` (= `rs.nt.shamblefrontlines.com`) | RS's public IPv4 (with FIXED-IP: `npt-rs`'s kept address) | 1 min | DNS only | with FIXED-IP (a), once; otherwise every run, as soon as RS shows its address (§7.3 step 6) |
| RS, spare name | `A` | `rs-b.nt` | the same address | 1 min | DNS only | only when the run plan says `rs.nt` would get a 6th certificate within 7 days |

- **RS waits for its own record:** RS's cloud-init asks Cloudflare's authoritative servers (`cora` and
  `steven.ns.cloudflare.com`) directly, with no recursion, and orders the certificate only once they answer with its
  own address. A recursive resolver could otherwise cache "no such name" for up to 30 minutes (infra INF2-9).
  Optionally, Mustafa sets the zone's SOA minimum TTL to 60 s once.
- **"DNS only"** answers with the server's own address and carries no traffic through Cloudflare. A proxied record
  carries only HTTP(S), never WebTransport's UDP.
- **What is not created:** no AAAA record (the proof test runs on IPv4), no CAA record for now, and no record for LG,
  which is reached only through RS.
- **Teardown,** without FIXED-IP: the record is deleted first, then the VMs (§7.3).

### 8.4 The WebSocket leg's browser-trusted certificate (OQ-0006-5 development case (i); security-auditor's conditions, R-2)
- **The method: ACME http-01 from RS,** through two root-owned oneshots on `npt`'s sudo list (infra INF2-8):
  - `npt-cert issue` opens TCP 80 in nftables, and npt-host's egress to the ACME host for the order only. It runs the
    order as `npt-host`, then closes both;
  - `npt-cert revoke` opens the same egress window, revokes, and closes it again.
- **The client** (security N15): instant-acme 0.8.5 inside the locked workspace, with its default features off plus
  ring and hyper-rustls. That keeps the ring floor and avoids aws-lc-rs's C build.
  - It requires Let's Encrypt's 6-day `shortlived` profile and fails closed: `NewOrder::profile` errors when the
    profile is unsupported.
  - U6 confirms its revocation call.
  - A 90-day fallback is a recorded risk that needs security-auditor's and the advisor's OK at U6 (C11).
- **Staging:** R1 reads and records both directories' profile lists. If staging lacks `shortlived`, the dry run uses
  staging's default profile; production still requires `shortlived`.
- **The DNS wait:** cloud-init polls the authoritative servers for at most 30 minutes before the first order, and
  never retries a failed validation in a loop.
- **The key:** made on RS, mode 0640, readable by a group of the slot users and `npt-host` (the `wss` leg ends in the
  room processes). It never leaves the VM. Each VM opens a new ACME account, so no secret exists anywhere.
- **The ledger:**
  - before each run, it is checked against CT through crt.sh and Cert Spotter (no account), from this session over
    HTTPS;
  - limits: 5 per exact set of names per 7 days, 50 per registered domain;
  - any certificate for a `*.nt` name that is not in the ledger stops the run;
  - if neither source answers, the run waits.
- **Revocation** at teardown, before the VMs are deleted.
- **Public names:** every issued name lands in the public CT logs, so stray probes come. The firewall, the roster and
  the tokens contain them (§10.3), and the results count them.
- **Not taken:**
  - dns-01 with a Cloudflare API token: a secret on a VM;
  - tls-alpn-01: TCP 443 is kept for the lobby;
  - an IP-address certificate: PM-0006-15 names a certificate on the domain.
- **Fallback, if port 80 is refused:** dns-01 by hand, with Mustafa adding one TXT record per issuance.

### 8.5 The WebTransport development certificate (ADR 0006:302-305; PM-0006-25 to -27)
- **Made by:** the room server or supervisor on RS at start: ECDSA P-256, valid at most 14 days, never in the
  repository.
- **Its hash:** npt-host publishes it, with the next one for PM-0006-26's rotation, in `/npt/cert-hash.json`
  (`Cache-Control: no-store`) over the browser-trusted name.
- **PM-0006-27's 15-day certificate** is made the same way, on purpose.

## 9. The Safari leg (MACHINES-TIMING; SPIKE_PLAN:45-57)
- **Facts** (read 2026-11-05, and re-read by the verifier):
  - `macos-26` (arm64, image 20261026.0455.2) carries Safari and SafariDriver 26.6.2; `macos-15` (arm64, image
    20261028.0489.1) carries Safari 27.2. Both are at least the 26.4 WebTransport needs.
  - Minutes are free on a public repository.
  - Nothing official says UDP leaves a macOS runner, and a 2022 issue shows UDP 123 dropped. So condition (iii) is a
    real test.
- **The workflow** (security-auditor and infra-engineer review the file at stop B; **any `.github/` change goes to
  Mustafa for approval**):
  - **File and trigger:** `.github/workflows/npt-safari.yml`, `workflow_dispatch` only. No `pull_request` and no
    `workflow_run` trigger; no cache, neither restored nor saved.
  - **Who dispatches:** Mustafa on the Actions page, or this session; each run plan names which. Never the night shift
    or a routine.
  - **The ref** (security N3):
    - after W-PR merges, the change-set's branch takes main in through one merge commit (the guard asks Mustafa), so
      it carries the workflow;
    - every dispatch names that branch with `--ref`, and the job checks out `github.sha` only. There is no commit
      input;
    - a guard step fails unless `github.ref` is exactly that branch.
    So change-set code never runs in main's cache scope, which gate.yml's required smoke job restores from. The
    leftover risk is ADR 0006:551-555's accepted one: that branch's copy of the file runs, and it holds no secret.
  - **Permissions:** `permissions: {}` at the top, and `contents: read` on the job; no secrets, no environment;
    `persist-credentials: false`.
  - **Actions:** each pinned by its full SHA, as `gate.yml` does.
  - **Inputs** (through `env:`, checked before use):
    - the host, from an exact allow-list: `rs.nt.shamblefrontlines.com` or `rs-b.nt.shamblefrontlines.com`;
    - the ports: numbers inside the ranges;
    - the runner's join token.
    No `${{ inputs.* }}` appears inside a `run:` line.
  - **The runner's token** (security N4; infra INF2-15):
    - it admits only a dedicated slot that no measured row shares. Unused, it expires 30 minutes after minting; once
      bound, it lives until its session ends, at most 60 minutes, so PM-0006-26's session fits (C7);
    - at first use it binds to a nonce the job makes at run time, never an input. The job prints the nonce's hash, and
      npt-judge matches it against RS's record; on a mismatch, the runner's rows are INVALID;
    - the runner gets no session-control capability;
    - `redact-check` allow-lists the token's prefix, since it is public by design.
  - **Runners:** `macos-26` and `macos-15` as a matrix, labels pinned.
  - **Limits:** `timeout-minutes: 60`, so PM-0006-26's whole 30-minute session with three rotations fits (C7); and a
    concurrency group.
  - **Steps:** check out `github.sha`; `npm ci --ignore-scripts`; `sudo safaridriver --enable`; record `sw_vers`, the
    Safari, SafariDriver and image versions; run the WebdriverIO Safari spec against the page on RS.
  - **Output:** one JSON artifact with `retention-days: 3`, holding versions, verdict inputs and VM names only. The
    log carries the same.
- **Why a separate PR (W-PR):** a `workflow_dispatch` workflow triggers only if its file exists on the default branch,
  and once it has run, a dispatch may name any ref, whose copy of the file runs (docs.github.com, read 2026-11-05). So
  W-PR reaches main before R1, on Mustafa's approval and go. The kit is frozen until this test is under way; he
  decides.
- **The first dispatch,** first thing in R1, records conditions (i) to (iii):
  - (i) the image's Safari version, at least 26.4;
  - (ii) real Safari through safaridriver, not a WebKit build;
  - (iii) one WebTransport session to RS that opens and carries datagrams, and one `wss` session on the domain.
  If any of the three fails, the runner stands in for nothing, and Mustafa borrows the Mac before R4.
- **What the runner may stand in for** (the advisor's reading, SPIKE_PLAN:54-57):
  - PM-0001-28 (server on RS), -14, -25's Origin and PM-0006-15;
  - with R-13 (ruled): PM-0001-32 and PM-0006-25, -26 and -27, on two conditions (C7): PM-0001-32's WebGPU half
    counts only if the runner's Safari returns a WebGPU adapter, and PM-0006-26 only if the job limit and the token's
    life cover its whole session. Otherwise those parts wait for the Mac.
  It never stands in for a time row (-29), or for -36, which needs a home EU connection.

## 10. Security (security-auditor's round-1 findings built in; it reviews this revision in round 2 and the run package at stop B)
### 10.1 What faces the internet while a VM lives
- **RS:** the game and page ports, open to all; TCP 80 only while a certificate order is live; SSH from the laptop's
  address only; the exporter to LG only.
- **LG:** SSH from RS only, and ICMP from two addresses.
- **On the density days** the VMs stay up between blocks. Then every room and the page host are stopped, and the game
  ranges are dropped in nftables (security N14).

### 10.2 The control plane (security F1, N6, N11; netcode NC2-F11)
- **Room-wide controls** go only through local Unix sockets on RS, one per room slot, at `/run/npt/ctl/NN`.
  - The directory is 0750 `root:npt-ctl`, and each socket 0660. The group `npt-ctl` holds `npt` alone, and no room
    user is in it. Each room checks with `SO_PEERCRED` that the peer's uid is `npt`.
  - The sockets are driven by the laptop session over SSH, or by the run script on RS. They are never a network
    message.
  - They cover:
    - the rewind cap;
    - the per-address cap raise, for the listed LG addresses only;
    - the roster and its tokens;
    - panic and stall injections;
    - L2 settings per peer set: LG's profile, -23's throttle, -24's bottleneck, -18's bursts, -29's UDP block.
  - Socket messages get the same size limits and panic-free parsing as network ones.
- **Rooms run as systemd template units:** `User=npt-room-NN` (or `DynamicUser`), `NoNewPrivileges`,
  `ProtectSystem=strict`, an empty `CapabilityBoundingSet`, and a `MemoryMax` above the expected RSS.
  - The supervisor stand-in stays unprivileged, and starts, stops and kills rooms through `systemctl`, by `npt`'s
    fixed sudo wrappers (§10.5).
  - Each slot's state, WebTransport key and results directory is mode 0700, owned by that slot's user.
- **Per-session controls** (`SessionControl`: the profile, B1, the UDP block, the record form's steps) need a
  capability, one per browser slot of the run (the advisor's A7: it binds to one session, and a run can have several
  browser legs):
  - 128 random bits, minted through the socket, delivered as §10.3 says, and compared in constant time;
  - `SetProfile` and `BlockUdp` key on the holder's source address. They cover all its connections, new ones
    included, from QUIC's first packet and the TCP accept, until cleared or the run ends. A new peer gets the run's
    default profile, set on the socket;
  - `TriggerBurst` acts on the live session;
  - the overlay posts a control to the Worker, which sends it on the control stream.
- **Parsing and logging:** session controls are parsed only on a joined, rostered session. A control-type message
  before Join closes that session as malformed. Every control and form record is logged with its source.
- **Bites:** a control or form post from a plain join is refused and counted, and a pre-Join control closes the
  session.
- **The catalogue** gives every message and control its channel, class, authority (none, capability or local socket)
  and size limit.

### 10.3 Join tokens and the roster (security F3, N4, N5; netcode NC2-F12)
- **What a token is:** 128 random bits, minted on RS through the socket, bound to one room and one slot, compared in
  constant time against the roster, and expiring at the run's end. The Join also checks the version and content hash.
  A room refuses joins beyond its roster's N, and npt-host never mints tokens.
- **Bots:** they get their tokens by file over the internal path. A bot's token is reusable for its own slot. A Join on
  a slot that still has a live session replaces that session (`Kick(replaced)`). -19's pattern: bots leave and rejoin,
  keeping the room at 47 to 48.
- **Browsers:** they get their token and capability in the URL fragment from the laptop session.
  - Both bind at first use to that session. A rejoin after a Worker trap uses a resume secret kept in page memory, so
    a later copy of the URL is useless.
  - The page reads the fragment and calls `history.replaceState` to drop it before the first render.
  - The link reaches Mustafa's PC by a local route the run plan names: typed from the laptop's screen, or a file on a
    USB stick. Never chat, email, GitHub or a synced clipboard.
- **The runner:** a single-use public token for a dedicated slot, bound to a job nonce (§9).
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

### 10.5 cloud-init and the VM (security F8, F10, N6, N7, N12, N13; infra INF2-2, INF2-3, INF2-13, INF2-14)
- **Users:**
  - `npt`: the SSH login. Its sudo list holds only root-owned wrappers, `/usr/local/sbin/npt-*`, with no free arguments
    (or validated ones) and no pager. The wrappers are `npt-ctl peers`, `npt-cert issue` and `revoke`, a capture
    wrapper with a fixed interface, filter and output path, the room unit controls, and the user-data read. Stop B
    checks the list against GTFOBins;
  - `npt-build`: clones and builds;
  - one user per room slot (`npt-room-NN`, or systemd's DynamicUser), with no shell and no write access to the binaries
    or the clone (B12's interim, ADR 0006:542);
  - `npt-host`: the page host and ACME;
  - on LG, `npt-load`: the load orchestrator's restricted login.
- **SSH and logins:**
  - `ssh_pwauth: false`, `disable_root: true`, `ssh_genkeytypes: [ed25519]`.
  - A drop-in named `00-npt.conf`, so it sorts before Ubuntu's own `50-cloud-init.conf` and `60-cloudimg-settings.conf`,
    since sshd keeps the first value it reads. `sshd -T` checks it at bring-up. It sets: `PasswordAuthentication no`,
    `KbdInteractiveAuthentication no`, `PermitRootLogin no`, `AuthenticationMethods publickey`, `X11Forwarding no`,
    `AllowAgentForwarding no`, `MaxAuthTries 3`, `LoginGraceTime 20`.
  - RS: `AllowUsers npt`; `AllowTcpForwarding local`; `PermitListen none`; `AllowStreamLocalForwarding no`;
    `PermitTunnel no`; `GatewayPorts no`; `PermitOpen none` until `npt-ctl peers` sets LG's port 22.
  - LG: `AllowUsers npt npt-load`; `DisableForwarding yes`.
- **The RS-to-LG key** (infra INF2-3; security N7):
  - cloud-init on RS makes an ed25519 key for `npt` (mode 0600), which never leaves RS;
  - at bring-up the laptop session reads only its `.pub` and installs it on LG through ProxyJump, for `npt-load`,
    with `from="<RS IPv4>",restrict,command="/usr/local/bin/npt-load-ctl"`;
  - so RS's run script can start, feed and stop the load, and copy token files, while the laptop is away;
  - the key dies with the VMs. It is listed in the catalogue's control-plane table and in stop B's review.
- **`npt-ctl peers <validated IPv4s>`** (infra INF2-2): run by the laptop session at bring-up. It fills nftables'
  `metrics_src` set, writes `PermitOpen <LG IPv4>:22`, runs `sshd -t`, reloads sshd, and records the values (VM
  addresses only) in the results. The day's addresses are therefore never inside the hashed cloud-config, except the
  kept ones under FIXED-IP (a).
- **Order:** nftables is written inline in `bootcmd`, so it loads before any service (§8.2).
- **Toolchains,** each pinned and never through `curl | sh` or an `#include` URL:
  - rustup-init by a pinned SHA-256 per architecture.
  - On RS only, for the page host's dist (security N12; infra INF2-13): Node 22 from the official tarball, pinned by
    SHA-256; `cargo install wasm-bindgen-cli --locked` at `Cargo.lock`'s version; `npm ci --ignore-scripts` and the
    Vite build, all as `npt-build`.
  - The built files' SHA-256s are compared with this session's build of the same commit. That is a cheap
    reproducibility check, worth about 10 minutes of bring-up. npt-host serves only that tree.
- **Build:** as `npt-build`, a clone at the run's full commit SHA, then `cargo fetch --locked` and
  `cargo build --locked --offline`.
- **Upgrades:** security upgrades at creation; unattended upgrades and reboots off during runs.
- **The file itself:**
  - it holds no secret, since every process on the VM can read user data, and no address of the day;
  - it stays under Hetzner's 32 KiB limit (checked at U8);
  - it is pasted from a raw-file URL that names the commit;
  - the laptop session's first check compares the stored user data with it (§7.3 step 7).
- **LG's disk:** LG's byte logs (about 5 GB per room-hour) run only in -17's and -19's blocks, with free disk checked
  first.

### 10.6 Raw data and redaction (security F14, F18, N10; R-21)
- **Raw data stays off the repository:** LG's received-byte logs, packet captures, per-tick logs and auth logs. They
  live on the VM, or in `.gate/` on the laptop, and are deleted with the VM or at the close.
- **What is committed:** verdicts, numbers, hashes and redacted headers, with the VMs' addresses and the date (R-21).
  Never the operator's address or Mustafa's.
- **No committed file ever holds the operator's address:** run plans and `ops/firewall/` carry a placeholder, and
  Mustafa types the address only in the Hetzner console. On the laptop, the file holding it sits outside every
  worktree and outside `.checkpoints/`.
- **`redact-check`** fails on:
  - any IPv4 or IPv6 address not in the run's VM list or a short allow-list, including IPv4-mapped, decimal and
    rDNS-embedded forms;
  - token and capability patterns, except the runner's public token prefix;
  - canary values;
  - PEM or OpenSSH private-key headers.
- **Where it runs:**
  - on the laptop, as a pre-push hook in the clone, from a fixed copy whose SHA-256 stop B records;
  - in this session, before every commit under `spikes/netcode-proof/` and `.checkpoints/`.
- **Results never include** auth logs, or `who`, `last` or `ss` output. PM-0006-21's canary grep runs on LG, and only
  counts leave it.
- **The laptop's git author** is set to his GitHub noreply address.

### 10.7 Floods, abuse, accounts and the supply chain
- **Probes and floods (security F16):** closed-port probes (PM-0006-15) come only from the laptop and the runner,
  never from LG. R4's run plan states the flood rates.
- **An abuse notice:** the run stops, and Mustafa answers that these are his own test servers.
- **Accounts:** two-factor login is confirmed on both accounts (§7.3 step 0).
- **Supply chain:** pinned dependencies and the audits of §4.8.

## 11. Mustafa's checks
1. **R1, the five-browser smoke check (about 10 minutes).**
   - In Brave, in a separate profile with no extensions and no Sync, open the link the laptop session shows:
     `https://rs.nt.shamblefrontlines.com:31000/?smoke=1#cap=…`. Type it from the laptop's screen, or open it from a
     file on a USB stick; never send it through chat, email, GitHub or a synced clipboard (§10.3). Expect
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
   - At the start: his base round trip (10 s of pings) is measured and recorded, and -3's 100 ± 5 ms check runs on his
     PC.
   - The rewind cap is set over the socket before step 1 and shown read-only (R-28). The command-timing option and
     the cap in force are named on the overlay.
   - 47 bots, Mustafa in the 48th slot, plus the target course.
   - The overlay shows the profile, the transport, the server-measured round trip, the interpolation delay, the rewind
     cap and four counters: corrections, snaps, extrapolated frames and clipped shots. The clipped-shot count is logged
     for steps 7 and 10.
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
| U1 | S | The skeleton: the workspace, pins, `deny.toml`, the clippy configs, npt-log, `marks.json`, npt-judge with its PASS, FAIL and INVALID fixtures (one of them PM-0006-14's short padding), the dependency-rule checks (§4.2); clippy and the wasm32 check from here on; the message catalogue with its class column, and its class audit test (netcode-architect drafts it); `redact-check` |
| U2 | M | npt-l1 with its unit and property tests; PM-0001-3's L1 half and its bites |
| U3 | S | R5-TEST-OQ2 on loopback: each WebTransport crate on a custom quinn endpoint over `ImpairSocket` (U3b). U3a: the Linux-faithful TCP model (S0-S14, `linux-6.8-tcp v3`); the retransmission-timeout floor verified (it is on rttvar, not on the timeout). U3c: the WebSocket `DelayStream` running that model, fix 1's send gate with its mechanism tests, npt-tcpinfo for the plain path, the kernel cross-check here, and its stall count checked at P2. U3d: the timing self-test: ImpairSocket at P1, 20,000 releases per second, inbound polled from a spawned tokio task as quinn polls; reported here, judged on RS at R1 (U8; u3d-plan.md, the advisor's APPROVE WITH CONDITIONS of 2026-11-09 in advisor-u3d.md). This fixes L2's shape; if neither crate allows it, stop (R-22) |
| U4 | M | npt-sim, npt-proto, npt-room and npt-client; npt-lab's scenario runner; `due_ticks` with PM-0004-67's bites on a fake clock; the Node-hosted native-against-Wasm hash test (§4.5); then the L1 row groups with their bites: codec (-17, -57, -58); commands and hostile input (-20, -21, -22); prediction (-41, -42, -60); interpolation, clock and events (-45, -47, -61, -62); lag compensation (-48 to -51); bursts (-18); stand-ins (-54, -55); -30 on loopback |
| U5 | S | Fuzzing (-34); the dependency and swap checks (-35); the audits (§4.8) |
| U6 | M | npt-server, npt-net, npt-super and npt-host: both WebTransport crates and WebSocket with L2 on loopback; the per-slot control sockets with their peer check, the systemd units and the capabilities; the roster and tokens; interest under load; instrumentation; metrics and logs; instant-acme's profile and revocation calls; the `npt-ctl` and `npt-cert` wrappers. **WsServer's send path with fix 1** (U3c's carries, the advisor's C14):<br>• the gate: `snapshot_budget` and `may_write`, shared, not copied; its inputs come from DelayStream on L2 and from npt-tcpinfo on the plain path; owed = everything above the kernel socket (tungstenite's buffer plus rustls's unsent TLS); the plain path's free value stated with its reason;<br>• U3c's writer properties, test (c) and the queueing-writer bite re-run on WsServer;<br>• a plain-path check: a client with a small receive buffer stops reading; after any write SIOCOUTQ ≤ N × size, and writes stop;<br>• the run-time bite ws-budget-x2 in a npt-net `bites` feature (R11);<br>• how WsServer sees one tick's whole snapshot through Link's per-message send (W18);<br>• SetProfile on a DelayStream connection;<br>• where the per-tick syscalls run;<br>• the header records whether the wrapper is on.<br>**More carries from U3c's review rounds (2026-11-08):**<br>• WsServer reads `gate_inputs()` once per tick: one lock, one model step (F11);<br>• npt-server range-checks a client-triggered B1 burst's start and length before `DelayStream::add_burst` (S7, F1);<br>• `header_numbers` blocks on the clock thread, so call it through spawn_blocking or once at start-up (F6);<br>• size the accept cap at about 0.31 MB per DelayStream connection, plus a fixed amount per process (F7; the advisor's split ruling (b));<br>• record whether -25's never-joining sessions get a DelayStream at all (at P0 the wrapper is refused);<br>• build tungstenite's limits and DelayStream's from one `&L2Config` (S5 a);<br>• the Join deadline covers the TLS accept too (S5 b);<br>• the plain path's free value is stated with its reason (S5 d);<br>• WsServer uses tokio-tungstenite with futures-util as a normal dependency, listed in §4.8 and audited at U5;<br>• the lateness split's histograms are pooled per process and never reset (delay.rs's POOL; C1b's review, INFO 5): one measured run per server process, or the pool reset at each run's start, stated in the run's header;<br>• DelayStream's shutdown now returns S's own error when the pump ended on one (C1c): a browser that drops TCP without TLS close_notify gives rustls's UnexpectedEof there, so WsServer treats a shutdown error after the peer's close as a normal end, not a run failure (C1c's review, INFO);<br>• **the reader stamp (the advisor's UP-STAMP ruling, 2026-11-08):** stamp R at the reader's wake (DelayStream's closing `wake_all`, delay.rs:1884, not at the release, which is already P or T), so up's woken work splits into pass = R − P and reader = (3) − R (in the other class the pass is from T); a reader that returns before R gets its own counted class with no reader part, so the sum stays exact; an identity test and a misattribution bite, as b5c9009 did; U6's loopback self-test of WsServer (its own reader task, split halves) reports the four parts before the run package is built; the bar stays on L as defined.<br>**U3d's carries (u3d-plan.md §3.6, the advisor's APPROVE WITH CONDITIONS of 2026-11-09):**<br>• **C-U6-1 the poller:** the -3 probe's inbound poller is a task spawned on npt-server's multi-thread runtime (never `block_on`), calling `poll_recv` with quinn's 32 buffers of 1,472 B until Pending, so -3's L2 up series and the leg's E include the hop quinn pays; U3d's blocked-worker test (T1) re-run on the probe's own poller; the pacer stays a dedicated thread;<br>• **C-U6-2 one socket per leg:** one ImpairSocket per profile leg, so each `l2_summary` is one leg's; no reset method (plan §4.7 lists none) and `set_default` never carries a socket across legs; a test shows two legs' summaries never mix;<br>• **C-U6-3 the fate log:** U6 chooses the drain cadence (per second with capacity for one to two seconds, its pages touched in the warm-up; or one log per leg pre-faulted before the measured phase); the drain's closure only copies into a buffer the probe owns, no I/O under the socket's lock; the lock hold per drain measured and reported;<br>• **C-U6-4:** the probe process writes `second.v.steal_permille` as npt-server does;<br>• **C-U6-5 the judged E's producer:** at the P1 leg's end the probe computes per direction, from its fate log and over exactly -3's measured records: n, E with L0 = 0 (judged), E with L0 = the window's nearest-rank p50, the over count, p50, p99, p99.9 and max, on the histogram's sample set (finished fates but Oversize; -3's delays use Delivered only), in the leg's summary event under names U6 adds to SERIES.md; tested on fixed fate entries (a late datagram in the warm-up stays out of E; inside the window it enters E with exactly its excess over n in both forms; T5's samples give 10,000 and 10,100 ns) and live (U3d's T2 identity re-run);<br>• **C-U6-6 rate and composition:** U6 states the up stream's composition (LG's fresh sends against its echoes) so the probe's socket sees 10,000 per second each way; another rate makes the leg's self-test INVALID and is recorded, but never by design: if -3's sends plus echoes cannot give 10,000 per direction through the probe's socket (ADR 0001:1590), U6's plan brings it to the advisor before R1's run plan is built;<br>• **C-U6-7 observer calls:** inside the measured window the probe calls only `try_send`, `poll_recv`, `take_second` once per second and the drain; `header_numbers`, `header_texts` and `run_summary` run at the leg's edges |
| U7 | M | npt-client-wasm and the harness (E7); the browser smoke in local Chromium; the hash check in Chromium. **C-U7-1** (the advisor's condition 8, 2026-11-09, advisor-u4.md): U4's native-against-Wasm check (U4d2) does not cover wasm-opt, while ADR 0002:500-505 checks "the exact file G6 ships" (after wasm-bindgen and wasm-opt); U7 reruns U4d2's check on npt-hash-wasm passed through the same wasm-opt step and flags as the shipped npt-client-wasm, or states why not |
| U8 | S | The run package: cloud-init files (32 KiB check), firewall lists, R1's run plan (with S14's veth + netem check, which also reads npt-tcpinfo's srtt against netem's 100 ms round trip: U3c's C12 (f), since this container has no tc; if that check disagrees with the model (S14: the kernel's stall or burst-recovery p50 or p99 more than 10 % from the model's, at zero jitter), netem wins (Mustafa's TCP-FREEZE, 2026-11-10): the WebSocket rows are then judged on VM runs with netem in place of DelayStream's model, the model is not repaired, and the disagreement no longer comes to the advisor for a remedy (this replaces S14's 'A miss goes to the advisor before the WebSocket part of -18 counts', advisor-u3q.md:143-144); DelayStream's loaded self-test below then no longer gates -18's and -31's WebSocket parts, which no longer run through it; R1's report records the comparison and the switch; Mustafa's NETEM-BAR (a), 2026-11-10: "The bar stays capped as WS-BAR says. If a WebSocket row fails because the kernel is slower than the model, report it to me as the kernel's, with the netem numbers, and I decide."; and DelayStream's self-test on RS, the advisor's condition-10 ruling of 2026-11-08: 48 connections at P2 with the room running, at least 10,000 samples per direction, judged per direction E[(L − L0 − 0.25 ms)+] < about 10 µs and p99 ≤ 1 ms, with p50, p99.9 and max reported; the split built in U3c (b5c9009) plus U6's reader stamp; steal recorded (/proc/stat read before and after each DelayStream and ImpairSocket self-test run); a max over 10 ms comes to the advisor with the result; a miss goes to the advisor before -18's and -31's WebSocket parts count, and R1 then follows the advisor's fallback order of 2026-11-08: the split with the reader stamp to the advisor first; a miss in up's pass part: first measure waking the reader right after the step that releases its record, and yielding; (b), the pumps off tokio, only for a miss in the hop); **ImpairSocket's R1 self-test (u3d-plan.md §5.2; the advisor's APPROVE WITH CONDITIONS of 2026-11-09):** on the -3 L2 probe's P1 leg in R1's L2 probe block (npt-server's probe mode, one ImpairSocket per leg, 10,000 per second each way, 0 added hours), judged per direction by the advisor (U3D-JUDGE (J3)) as **E with L0 = 0, ≤ 10,000 ns per direction (U3D-BAR, U3D-L0), over the probe's P1 measured records** (at least 1,000,000 sent per direction; U3D-WINDOW (W1)); run conditions that make it INVALID, never a pass: -3's own count, the rate within 1 %, slack: under SCHED_OTHER 1 at both reads (start and header); under SCHED_FIFO or SCHED_RR 0 or 1 at both reads (0 only on a kernel with ed4fb6d7ef68); under any other policy, SCHED_DEADLINE included, 1 at both reads (U3d's built rule, socket_tests.rs:1480-1486), no queue, pool, oversize or shaper drop and no send error, the steal reads and U3-Q9's facts present; reported: p50, p99, p99.9, max, over, the median form and `l2_summary`'s E, the release thread's CPU, slack and policy, the poller kind and stream composition, steal, U3-Q9's facts, THP, loadavg, the drain's lock hold, a max over 10 ms flagged; a miss goes to the advisor before any L2 mark counts, the first remedy the recorded `chrt -f` switch run as its own leg, a spin lead to Mustafa (U3D-MISS); -3's own L2 P1 checks judged by npt-judge beside it; **C-U8-1** U3-Q9's four facts (kernel, CONFIG_HZ, cpuidle driver, clocksource) and the THP mode read by shell before any timing judgement, with U3d's dry-run commands; **C-U8-2** every `^cpu` line of /proc/stat read before and after the probe's P1 leg; **C-U8-3** DelayStream's loaded self-test has no hours in R1's row yet: U8 sizes it; **C-U8-4** the self-test's run conditions, the rate among them, and C-U8-6's: no drop on the probe's P1 socket between its two reads; **C-U8-6** (the advisor's U3D-RCVBUF ruling, 2026-11-09, advisor-u3d.md) before and after every L2 probe leg (P1-P4 and B1), in the probe's network namespace and outside the measured path, read alongside C-U8-2's leg-edge reads by the same mechanism: the probe socket's own `drops` column in /proc/net/udp or /proc/net/udp6 (matched by its local port) and /proc/net/snmp's Udp lines (/proc/net/snmp6's Udp6 lines when present); a drop on the probe's P1 socket between the two reads makes ImpairSocket's self-test INVALID with its cause named, and that leg comes to the advisor before any L2 mark counts with its printed E, its max and the counters, never set aside and re-run as noise; no SO_RCVBUF sizing without the advisor's ruling (quinn leaves the OS default; a bigger buffer would erase the only trace of a long reader stall); if U8's plan cannot place the reads at the leg edges, U6 adds them to C-U6-7's leg-edge calls and brings any new npt-log name to the advisor; **C-U8-5** U3D-L0 covers ImpairSocket's self-test only: U8's plan states DelayStream's judged form against U3D-L0 and brings it to the advisor, so one bar is never read in two forms without a ruling; the operator `settings.json` and `rules.md`, the pre-push hook, the sudo wrapper list, the WebdriverIO specs, the netcode checklist (test-engineer), W-PR's workflow file |

### 12.2 Stops (where the work waits for a person)
- **Stop A, now:** review round 2 on this revision, then the advisor, then Mustafa.
- **Right after approval: R-PR** (C9).
  - One small docs PR writes the advisor's readings, in his exact words (`advisor-verdict.md`), into ADR 0001,
    ADR 0004, ADR 0006 and SPIKE_PLAN. It carries R-1, R-5, R-6, R-7, R-9, R-10, R-12 to R-17, R-25, R-26 and R-28,
    plus FIXED-IP's line if Mustafa picks (a).
  - It sits on its own branch from main, is classed needs-mustafa (ADR edits), and merges on his go before the first
    counted run.
  - This session may push only to its own branch, so it needs his OK for one more branch (`claude/npt-readings`), or
    another session makes R-PR. W-PR (§9) likewise needs `claude/npt-safari-wf`.
- **Stop B, before any VM exists:**
  - security-auditor and infra-engineer review the run package (cloud-init, firewall lists, page host, control plane,
    operator settings, `redact-check`, the sudo wrappers checked against GTFOBins) and W-PR's file; fixes follow;
  - the laptop's guard check, moved here from R1 (infra INF2-12; security N2), runs on whatever OS the laptop has, in
    the clone on main with the operator settings. Four attempts (C1):
    1. a push to the change-set branch, with nothing new to push so a failed guard writes nothing: refused;
    2. a `gh` call: refused;
    3. a WebFetch: refused;
    4. a push to `claude/npt-results`, of its own remote head: it must prompt;
  - stop B records the tree hash of main's `CLAUDE.md` and `.claude/` (C5);
  - **Mustafa approves `ops/operator/settings.json` and the `PROMPT-Rn.md` files explicitly** (C4: security rules for a
    Claude session on his machine);
  - he reads the Limits tab (§7.3 step 0), approves W-PR and gives his go to merge it, and OKs R1's run plan, cap and
    delete-by time.
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

### 12.3 The laptop session (security F5, N1, N2; R-23)
- **Folders** (security N1):
  - its main folder is a clone on main, so main's rules load;
  - the run's commit and the results live in worktrees under `.gate/` (`.gate/npt-run`, `.gate/npt-results`);
  - `claude/npt-results` is an orphan branch that holds `results/` only;
  - before reading either worktree, the session checks it. The run worktree's `CLAUDE.md` and `.claude/` must equal
    main's (`git diff --quiet origin/main -- CLAUDE.md .claude`), and the results worktree must hold neither. On a
    mismatch it stops.
- **Mode:** Manual (the default), never auto or bypass.
  - Never answer "Yes, and don't ask again", which would save a lasting allow.
  - At each start, check that the clone's `.claude/settings.local.json` is absent.
  - Delete the clone at the close.
- **The extra settings file:**
  - copied once, at stop B, to a fixed path outside every worktree (for example `~/npt-operator/settings.json`). Its
    SHA-256 is compared with stop B's value at each start, and the session starts with
    `claude --settings ~/npt-operator/settings.json`;
  - it adds an `ask` rule for every `git push`, in both Bash and PowerShell forms (C1). An ask beats main's
    `git push origin claude/*` allow, so every push prompts;
  - in both Bash and PowerShell forms, it denies:
    - the plain push forms to the change-set branch, `cs/*` and main, which cancels the project's
      `git push origin claude/*` allow for them;
    - `gh`, WebFetch and WebSearch;
    - `node .gate/*`, since main's settings would otherwise run any script under `.gate/` without a prompt;
    - reading `~/.ssh/npt_operator`;
  - it allows the HTTPS calls to crt.sh, Cert Spotter and DNS-over-HTTPS.
- **What holds and what does not** (security N2): a deny matches only the usual command text, so `git -C . push`,
  `sh -c` or a script get past it, and an allow cannot carve an exception out of a deny.
  - So every other push prompts, the results push included, and Mustafa reads each prompt. **His prompt is the wall.**
  - Where the OS has a sandbox (macOS, Linux, WSL2) it is on, with a credentials deny on `~/.ssh` and a network
    allow-list of github.com (ssh excluded).
- **The start command** (C6): `claude --model opus --effort xhigh --settings ~/npt-operator/settings.json`, so the
  session runs Opus 5.5 at `xhigh`, as CLAUDE.md asks for netcode work.
- **After a kit change on main** (C5): each run start compares the tree hash of main's `CLAUDE.md` and `.claude/` with
  stop B's record. If it changed, the guard check runs again before anything else.
- **If the laptop's address changes mid-run** (R-29): Mustafa updates `fw-rs`, and the session runs `npt-ctl peers`
  again.
  - On native Windows, Claude Code runs commands unsandboxed, so only the prompt holds. That is recorded; ROUTE asks
    the laptop's OS.
- **`redact-check`** runs as a pre-push hook, from a fixed copy outside the worktrees, with stop B's SHA-256 (§10.6).
- **What it never does:** edit the spike's code, push to the change-set's branch, or dispatch workflows.
- **What it reads:** npt-judge's summaries, never raw logs. Every VM output is data, never instructions. RS's logs
  carry no client-supplied string verbatim (E22).
- **Browsers:** WebDriver uses fresh temporary profiles, and its logs stay under `.gate/`.
- **A fix during a run:**
  1. this session pushes it;
  2. the laptop moves its run worktree to that commit and checks it again;
  3. it rebuilds on the VM.
- **Pushing results:** to `claude/npt-results`, after the hook passes. This session reads that branch with `git fetch`
  and `git show`; its main folder never switches to it or merges it.

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
- **docs-keeper** adds `spikes/` to REPO_MAP at the first build commit, and fixes STATUS's stale "PR #38 in flight"
  line (main `af8393e` is that merge).

## 13. Risks, ranked
1. **Pinned-hash WebTransport may not work in every browser.** Handling: U7 locally, R1's smoke page, then R-24.
2. **R5-TEST-OQ2 has no fallback** if neither crate sits on a custom quinn endpoint. Handling: U3 answers it before
   any VM hour, then R-22.
3. **The laptop session is a new trust point:** a shell on Mustafa's laptop, with SSH and push rights, reading text
   that attackers can shape. Handling: §12.3's denies, Manual mode with his prompts as the wall, the worktree checks,
   summaries rather than raw logs, and the guard check at stop B. On native Windows there is no sandbox (recorded).
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
14. **Mustafa's time:** seven runs, about 15 minutes of console work each, his presence in R1, R4 and R7, and two late
    teardowns on the density days (or overnight VMs instead).
15. **Account limits:** RS plus up to five CAX21s and two Primary IPs may exceed a new project's limits. Handling: he
    reads the Limits tab at stop B.

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

## 15. The readings, as the advisor ruled them (2026-11-06; the exact words for R-PR are in `advisor-verdict.md`)
| # | Reading | Ruling | Into the drafts by R-PR |
|---|---|---|---|
| R-1 | Ports (OQ-0006-16): §8.1 | confirmed; OQ-0006-16's deciders agree, with their conditions in the R-PR words | yes (ADR 0006) |
| R-2 | The `wss` certificate by http-01 on RS | approved with §8.4's conditions; a 90-day fallback needs security-auditor's and the advisor's OK (C11) | no |
| R-3 | Metrics by scrape | approved | no |
| R-4 | Host keys, §1.6 | approved | no |
| R-5 | An explicit reorder step per flow and direction | approved; as built in U2 (the advisor's U2-R1, 2026-11-06): a drawn datagram is overtaken by its successor when the timing allows, so it is never below the profile's floor but may arrive up to one spacing above its ceiling; the step is on in every run, and P0 and one-datagram-per-tick flows at P1 measure 0 by timing | yes (ADR 0001 part 13) |
| R-6 | 1,000,000 packets on both layers; L2's probe | approved | yes (PM-0001-3) |
| R-7 | Loopback as "the same local network" | approved; it changes -28's setup | yes (PM-0001-28) |
| R-8 | The x86-64 replays in this container or on RS | approved; no CI job | no |
| R-9 | L2's delay: (profile RTT − base RTT) ÷ 2 | the reading is right; R-PR fixes the draft's ambiguous wording | yes (PM-0001-3) |
| R-10 | Cover (OQ-0001-9) | **changed:** the advisor states the rule. It follows OQ-0001-5's answer: inside the cap, the shooter's view wins, so cover is judged rewound with the victim, at the shooter's view time. Present state only if rewinding fails PM-0001-7's memory or PM-0001-6's or -51's cost. Both variants are recorded on L1; Mustafa may overrule at the close | yes (OQ-0001-9) |
| R-11 | Stand-ins meet -54's and -55's conditions | approved | no |
| R-12 | The added bites | approved | yes (ADR 0001, 0004, 0006 rows) |
| R-13 | The runner list | approved with C7's two conditions | yes (SPIKE_PLAN) |
| R-14 | Closing in two steps | approved, reworded (C8): migration step 1 waits for R8's ML and RD rows only; Safari's time rows close at the Mac's first visit | yes (SPIKE_PLAN) |
| R-15 | PM-0004-67's runs | approved | yes (ADR 0004) |
| R-16 | -59's tick | approved | yes (PM-0001-59) |
| R-17 | The simulator's gaps | approved | yes, for the queue depths (PM-0001-23, -24) |
| R-18 | The page host in the TCP range; LG's list | approved | inside R-1's words |
| R-19 | OQ-0006-19 | **Mustafa's** (money): DENSITY-FIGURE, §16; the advisor advises (a) | after his answer |
| R-20 | R2-XU-OQ7's rule is PM-0001-7 | approved | no |
| R-21 | VM addresses in the results | approved | no |
| R-22 | L2's fallback if U3 fails | approved; a fallback that changes a crate or the transport goes back to the drafts, Mustafa and the advisor (C11; ADR 0001:1596-1597) | no |
| R-23 | Two sessions | approved, with C1 and C5 | no |
| R-24 | A browser that cannot pin | approved | no |
| R-25 | The WebSocket delay wrapper | approved. Its model became Linux 6.8's TCP (Mustafa's U3-Q3 (b), 2026-11-06; the advisor's S0-S14), built in U3a, checked against this container's kernel in U3c and against RS's at R1 (S14); if they disagree, netem wins and the model is not repaired (TCP-FREEZE). Fix 1's send budget is applied in WsServer, not in the wrapper (E15) | yes (ADR 0001:1444-1445); the model's words go in through part 13's key in docs PR #45 |
| R-26 | Density days on one VM set | approved | yes (ADR 0006) |
| R-27 | R-PR | approved, per C9 | — |
| R-28 | The cap read-only on the overlay | approved | yes (the netcode checklist) |
| R-29 | SSH's second wall | approved, with the address-change step (§12.3); security-auditor confirms at stop B | no |
| R-30 | WS-BAR's rule (Mustafa, 2026-11-07, §0.5; the advisor's readings of the same date) | **The practice run:** one unjudged L2 run with the same build, before the counted WebSocket run of PM-0001-18. **The tolerance:** 10 %, fixed before it. The run-structured bound's guard may lower it to the largest whole percent at which a budget twice the formula's passes in at most 1 % of runs; it is never raised. **The slowdown:** the practice run's median divided by the model's median run median for the same run. **Within the tolerance:** X, Y and Z move by the slowdown and k stays; at or below 1 the bound stands, never tightened from one run. **Beyond it,** or failing the count or maximum at the moved numbers: the cause is found first, and no number is ever looser than the bound times a slowdown measured within the tolerance. A recomputed bound that comes out looser goes to Mustafa with its numbers before any is set. A practice run short of 480 records counts as beyond (W23) | yes (PM-0001-18, docs PR #45) |
| R-31 | U3d's self-test bar (the advisor, 2026-11-09, advisor-u3d.md; readings of Mustafa's U3-Q1 (c) that never loosen it) | **U3D-BAR:** E ≤ 10,000 ns per direction, inclusive: the exact value of the derivation behind the recommendation Mustafa agreed to ("about" is the derivation's approximation, not a tolerance). **U3D-L0:** for ImpairSocket's self-test, E is judged with L0 = 0 (the impairer subtracts nothing, so this form predicts -3's P1 p99.9 exactly while no lateness exceeds 2a + 0.25 ms (10.25 ms at P1); past that it counts more than -3 sees, so it can MISS where -3 passes, never the reverse (corrected at U3d's close, advisor-u3d.md)); M1's median form is reported as built; DelayStream's judged form is not changed by this (C-U8-5). **U3D-WINDOW:** R1 reads the probe's E over the P1 leg's measured records, computed from its fate log (C-U6-5), with `l2_summary`'s E reported beside it | no |

## 16. Queue, decisions and asks
- **Queue:** STATUS (2026-11-05) names the netcode proof test as the next v2 work. B.1 and B.2 (docs) touch other
  files. A4 waits until this test is "under way" (A4-FREEZE (a)), which UNDER-WAY defines. W-PR needs Mustafa's
  approval anyway, since it is a `.github/` change.
- **The total against the drafts** (C10): this plan's 75 RS-hours exceed the drafts' estimate of 38 to 53 (ADR
  0006:1107-1113). §7.1 shows why, part by part. The planned spend is EUR 13.51 before VAT, with caps of EUR 14.90, so
  no single run's OK should surprise him.
- **At the close:** this plan goes into the report's appendix, and the checkpoint is deleted (CHECKPOINTS.md §9).
- **The docs PR #45:** Mustafa's WebSocket decisions of 2026-11-07 (§0.5) in ADR 0001, approved by the advisor at
  6ae3571. **Its go is pending until Mustafa gives it** ("merge #45" in a session's chat). It merges before R3's first
  counted PM-0001-18 run.
- **PR #40, this test's own PR, merges by squash** (Mustafa's SQUASH-40, 2026-11-10: its history carries about 28 MB,
  most of it checkpoints), with no `.checkpoints/` in the final tree. Before the merge, the closing commit re-points
  every citation of the commits bf9d248, 975e72b, 95a04a1 and 1e020a7 in ADR 0001 and docs/design/ws-reconnect.md (the
  advisor's finding 4 on #45) to files in the final tree, and the advisor checks each re-pointed citation against the
  file and lines it names (Mustafa's SQUASH-CITES (a), 2026-11-10). A citation of a `.checkpoints/` file is re-pointed
  to a copy of the cited words in a file the final tree keeps (the closing report's appendix), since no
  `.checkpoints/` file survives.

### Decisions needed (Mustafa): all five answered on 2026-11-06 (§0.4)
1. **ROUTE: how should the hands-on server work run, since this session cannot SSH?**
   - *Why it matters:* every run needs an operator, and the choice sets your time per run and where the SSH key lives.
   - *Options:*
     - (a) the hybrid: cloud-init builds each VM, and Claude Code on your laptop runs the sessions over SSH, with a key
       made there, a fixed settings file of its own, and your prompts as the last wall;
     - (b) cloud-init only: the VMs run fixed scripts and publish results over HTTPS. There is no SSH and no fixing in
       place, and LG gets a results server;
     - (c) Claude Code on your machine only, without cloud-init.
   - *Also:*
     - which machine runs that session: the laptop, where the minimum-laptop browser legs run, or your PC;
     - the laptop's operating system, which browsers it has, and whether Claude Code is installed on it;
     - on native Windows, Claude Code has no sandbox, so only your prompts hold. That is recorded, unless you already
       use WSL2.
   - *Recommendation:* (a), on the laptop.
2. **SAFARI-TIME: when do Safari's timed rows run?**
   - *What:* the runner may stand in only for yes-or-no and count rows. The 3-second fallback (PM-0001-29), the
     datagram size on a home connection (PM-0001-36), and any row the advisor does not add to the runner's list,
     need a real Mac.
   - *Why it matters:* you planned to borrow the Mac only for the look proof test.
   - *Options:*
     - (a) borrow it once during this test: one session, about an hour, at home;
     - (b) re-run those rows when you borrow it for the look proof test. This matches your own MACHINES-TIMING words:
       "I borrow the Mac only for the look proof test". Until then those rows stay "pending", and nothing before the
       foundation waits on them. PM-0001-36's datagram cap stays provisional meanwhile: 1,000 B, or the smallest value
       measured. That is a data value only (C12).
   - *Recommendation:* (b).
3. **FIXED-IP: keep two fixed addresses for the test machines?**
   - *What:* two Primary IPv4s created once in Falkenstein, named `npt-rs` and `npt-lg`, with auto-delete off and
     protection off. They are billed for as long as they exist, assigned or not: about EUR 1.00 a month for both,
     before VAT, counted in the bill check (PM-0006-18). They are deleted at the close: the DNS record first, then the
     addresses.
   - *Why it matters:* without them, every run needs a DNS edit and two firewall edits, and teardown must delete the
     record before the VMs, or the name could point at an address Hetzner hands to someone else. With them, those are
     set once.
   - *Options:* (a) keep two fixed addresses; (b) a new address each run, as the drafts assume.
   - *Recommendation:* (a).
4. **DENSITY-FIGURE (OQ-0006-19): is the rooms-per-server figure this test measures only a provisional lower bound
   until slice 6?**
   - *What:* the density runs pad the stand-in sim's step to its ceiling (8.33 ms per tick), because the real sim does
     not exist yet. So the count of rooms per server is a floor, not the real game's figure.
   - *Why it matters:* that count sets the monthly server budget the goal modes need (ADR 0006 part 8): money. It is
     your decision, on the advisor's advice (ADR 0006:1098).
   - *Options:*
     - (a) pad to the ceiling, and label the figure "a lower bound, provisional until slice 6";
     - (b) take the figure as measured;
     - (c) set no budget until slice 6.
   - *Recommendation* (the advisor's): (a). The drafts and PM-0006-14 already assume it.
5. **UNDER-WAY: when does the netcode proof test count as "under way"?** That is the point at which your kit freeze
   lifts, and A4 (the review policy) may start.
   - *Why it matters:* it decides when kit work runs beside the spike.
   - *Options:*
     - (a) once both approvals are in and the first build step (U0) is committed;
     - (b) after the first server run (R1);
     - (c) after the test closes.
   - *Recommendation* (the advisor's): (a). C5's re-check protects the laptop session from a kit change made
     meanwhile.

### For Mustafa
1. **Read this plan,** then answer ROUTE (with the laptop facts), SAFARI-TIME, FIXED-IP, DENSITY-FIGURE and UNDER-WAY,
   and give or refuse your approval of the plan.
2. **Two branches:** say whether this session may push two more branches, `claude/npt-readings` for R-PR and
   `claude/npt-safari-wf` for W-PR. It may push only to its own branch unless you say so. Otherwise, say which session
   should make them.
3. **Nothing to create yet:** no VM, no DNS record, no key.
4. **Next you get R-PR** (the advisor's readings into the drafts) for your go. No counted run comes before it merges.
5. **At stop B you get:**
   - the laptop's guard check (four attempts), run with you;
   - the operator settings file and the prompt files to approve;
   - the Limits tab to read;
   - W-PR (the Safari workflow) to approve;
   - R1's run plan, with its cap and delete-by time;
   - the cloud-config files;
   - the exact DNS record with that day's address (or, with FIXED-IP, the record once).
