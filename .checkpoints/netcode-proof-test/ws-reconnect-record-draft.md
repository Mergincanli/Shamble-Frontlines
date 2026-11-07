# The WebSocket fallback's fast reconnect: a requirement for the online client

**Status:** Proposed. A requirement record (a design doc, not an ADR). It changes no decision by itself: the reconnect
work brings its ADR 0001 changes (§5) for review when it starts.
**Date:** 2026-11-07
**Author:** netcode-architect
**Decided by:** Mustafa, 2026-11-07 (his words in §1.1)
**For:** whoever builds the online client's WebSocket fallback (ADR 0001's migration step 5, "Slice 3, online", which
ends when "Mustafa's first online playtest is run": `docs/adr/0001-multiplayer-netcode.md:1734-1738`), and the
reviewers of that work.
**Why it exists:** the netcode proof test measures the fallback's newest-snapshot-only buffer ("fix 1") but not the
reconnect. Mustafa asked for the reconnect to be recorded "so it can't get lost". The proof test's working notes are
deleted when its change-set ends, so this record carries its own numbers; the notes stay in git history (§7).

Line numbers: ADR 0001 as on main `69ea88e` and the proof-test branch's `803c6c1` (the same file); `fixes.rs` and
`tcp_model.rs` at `bf9d248` under `spikes/netcode-proof/crates/net/src/` (`tcp_model/fixes.rs`, `tcp_model.rs`). A
later docs PR may shift ADR lines, so each ADR cite in §5 also quotes its words.

## 0. In short

- **The requirement.** Before the first online playtest, the online client's WebSocket fallback reconnects fast after
  a total-loss burst, and the build is measured against today's model: with the newest-only buffer and the reconnect,
  99 % of client-bursts decode deltas again within about 1.05 s of the link's return (P2, a 1 s burst).
- **Why.** Fix 1 alone (what the proof test measures) brings the median down to 654 ms but leaves a long tail: p99
  2,164 ms, max 11,384 ms over 600,000 modelled bursts. The reconnect cuts the p99 to 1,051 ms and the max to 4,196 ms.
- **The browser shapes the policy.** One connecting WebSocket per endpoint (RFC 6455 §4.1), Chromium's throttle on
  failed attempts, Firefox's fail delay. The client's operating system decides when an attempt made during the outage
  can connect (its SYN retry timer).
- **An open choice.** The modelled policy ("never abandoned") is tuned to the 1 s burst. An abort scaled to the round
  trip with "join or close" is more robust at other outage lengths but cost about 75 ms of p99 at the 1 s burst in a
  smaller diagnostic run. The reconnect work re-measures both and picks (§3.7).
- **Credentials deferred.** How a reconnect proves who it is (RESUME-CRED) waits for the reconnect work; three
  readings of "bound to the session" are open (§4).
- **Measuring it.** The proof test's L2 setup cannot measure a reconnect as it is: it needs loss below TCP, a client
  that follows the browser's rules, and Windows-like clients (§6).

## 1. The requirement and its target

### 1.1 Mustafa's words (2026-11-07, about 13:45 UTC, verbatim)

> "WS-SCOPE: no. Measure fix 1 (the newest-snapshot-only buffer) in the proof test. The reconnect becomes a
> requirement for the real client, built and measured before the first online playtest, with today's model as its
> target (buffer + reconnect: 99 % within about 1.05 s). Record that in the docs PR so it can't get lost.
> RESUME-CRED: deferred with the reconnect. WS-BAR: the three-part form now; its numbers after one practice run on the
> real test setup, but never looser than the model predicts for a correct build with fix 1. If the practice run comes
> out worse than the model, find out why before setting the numbers."

Source: the proof test's checkpoint, "Decisions taken" (`.checkpoints/netcode-proof-test.md:349-354` at `803c6c1`).

What each part means:
- **Fix 1, the newest-snapshot-only buffer.** The room writes a snapshot over WebSocket only while the socket's unsent
  plus unacknowledged bytes (`SIOCOUTQ`) leave room for it within ceil(1.5 × srtt / tick) + 1 snapshots, at least 3
  (srtt from `TCP_INFO`); older unsent snapshots are dropped (`fixes.rs:4-5`, `:164-169`). The proof test builds and
  measures it; the online client's room keeps it.
- **The reconnect** ("fix 3" in the proof test's notes). After a silence, the client opens a fresh WebSocket
  connection and resumes its session on it with a `Join`; the newest `Join` takes the session over (`fixes.rs:8-14`;
  ADR 0001:506-508).
- **The real client.** The game's online client. ADR 0001's migration plan puts "reconnect" in step 5, slice 3
  (0001:1734-1738); its scope line lists part 11's reconnect among "slice 2 and slice 3 milestones" (0001:1640-1641).
- **Built and measured before the first online playtest.** Built to §3, measured as §6 says, against §1.2's target.
- **WS-BAR** is the proof test's bar for fix 1 alone (§2.3), not this record's; it is quoted for its rule about
  practice runs, which §6.4 proposes to reuse.

### 1.2 "Today's model": exactly what the target is

- **The TCP model:** `linux-6.8-tcp v3` (`tcp_model.rs:79`), Linux 6.8's TCP with its default settings, every rule
  read in the kernel source (`tcp_model.rs:1-10`). Three rounds of skeptic review found it faithful; no defect found
  moves the target's p99 (1,051 ms) or max (4,196 ms) (`u3-verify3-model.md:211-212`), and Linux 6.12's changes leave
  the rows identical (`:160-169`). Sure: high.
- **The harness:** `fixes.rs`. Its rows come from commit `1e020a7` (model and harness v3); `709e44a` and `bf9d248`
  move no number (`bf9d248` adds the fix-1 bound and a budget multiplier whose default, 1, changes nothing:
  `fixes.rs:74-75`, `:1208`, `:776`).
- **The scenario:** profile P2 (100 ms round trip, ±20 ms jitter, 2 % loss, half each way:
  `.checkpoints/netcode-proof-test.md:409-410`); one 1 s total-loss burst per phase (B1); the room's 30 Hz tick;
  1,050 B snapshots and 88 B inputs; Chromium's rules; TLS resumed; a Linux client (`fixes.rs:30-42`, `:1199-1210`).
- **What "recovered" means:** the client decodes a delta the room wrote at or after the link's return; on a new
  connection, the first delta after the full state (`fixes.rs:19-21`; PM-0001-18's wording, ADR 0001:1556).
- **"Buffer + reconnect"** is the row "fix 1 + fix 3: one endpoint, never abandoned (recommended)"
  (`fixes.rs:1236-1239`, `recommended()` at `:1259-1263`): trigger max(300 ms, 2 × echo RTT + 100 ms); one endpoint;
  one attempt at a time; never closed by a timer; at most 8 attempts per outage; an attempt still before its 101 closed
  once the old connection delivers fresh data (`fixes.rs:1-14`, `:1187-1197`).

**Table 1. 600,000 bursts at P2, 1 s burst, ms** (`.gate/netcode-proof-test/u3v3-tables1.txt:120-131`; the same rows
reproduced by the statistics skeptic, `u3-verify3-stats.md:30-31`)

| Row | p50 | p99 | p99.9 | p99.99 | max | over 1 s | over 1.5 s | over 2 s |
|---|---|---|---|---|---|---|---|---|
| Today (64 KB buffer, no fix) | 1,121 | 2,686 | 4,068 | 6,484 | 12,068 | 505,371 | 64,398 | 21,968 |
| Fix 1 alone (the proof test) | 654 | 2,164 | 3,945 | 5,801 | 11,384 | 52,406 | 24,442 | 15,226 |
| The reconnect alone (64 KB buffer) | 844 | 1,374 | 1,952 | 2,297 | 5,343 | 107,048 | 3,180 | 379 |
| **Buffer + reconnect: the target** | **654** | **1,051** | **1,859** | **2,092** | **4,196** | **6,768** | **1,009** | **205** |

The target's share over 1 s is 11.28 per mille (Wilson 95 %: 11.02-11.55); 10 bursts are over 2.5 s and 4 over 3 s.
Per burst it makes 1.0 attempt; 0.664 attempts are closed because the old connection recovered first; 0.161 takeovers
happen before the recovery and 0.176 after it (`u3v3-tables1.txt:130-131`).

**Table 2. Client variants of the target, 60,000 bursts each, ms** (`u3v3-tables1.txt:88-117`, except the last row)

| Variant | p50 | p99 | p99.9 | p99.99 | max |
|---|---|---|---|---|---|
| The target's row at 60,000 (Linux client, Chromium) | 654 | 1,054 | 1,827 | 2,066 | 3,013 |
| The same in Firefox | 654 | 1,054 | 1,827 | 2,066 | 3,013 |
| Full TLS handshakes (no session resumption) | 654 | 1,091 | 1,838 | 2,121 | 3,013 |
| No TCP timestamps (a Windows peer) | 645 | 886 | 1,484 | 1,969 | 2,165 |
| Windows-like client, first SYN retry after 1 s (current Windows) | 645 | 886 | 1,484 | 2,046 | 2,871 |
| Windows-like client, first SYN retry after 3 s (older Windows, server templates) | 646 | 2,035 | 2,729 | 3,069 | 3,890 |
| Fix 1 alone on a no-timestamp client (the same-client baseline) | 646 | 2,035 | 3,829 | 5,295 | 12,550 |

The last row is from the third skeptic round's diagnostic patch, not from `fixes.rs` (`u3-verify3-harness.md:79-87`).
Maxima grow with the sample (3,013 at 60,000, 4,196 at 600,000); the p99 is stable (1,054 vs 1,051)
(`u3-verify3-model.md:200-210`). The Windows-like rows model only the client's SYN timer and the missing timestamps;
Windows' own loss recovery for the client's sends is not modelled, which may move the median by tens of ms
(`u3-verify3-harness.md:155-161`; `u3-verify3-model.md:213-216`). Sure: medium.

- **Mustafa's laptop** (Windows): `Get-NetTCPSetting` shows MaxSynRetransmissions 4 in every template and
  InitialRtoMs blank (the built-in default), read as a first SYN retry after 1 s, then doubling: SYNs at 0, 1, 3, 7 and
  15 s (`.checkpoints/netcode-proof-test.md:336-346`; Catchpoint, "TCP SYN retries", read 2026-11-07:
  docs.catchpoint.com/docs/catchpoint-nodes-tcp-syn-retries). So the 1 s Windows-like row applies to him; the 3 s row
  applies to older Windows and Windows Server templates (`u3-verify3-harness.md:74-78`, `:196-206`). Sure: medium-high
  (his InitialRtoMs reads blank, not 1000).

## 2. Why fix 1 alone falls short, and what the reconnect does

### 2.1 What fix 1 fixes, and what it leaves
- **Today**, a backed-up WebSocket fills TCP's 64 KB write buffer with about 30 stale snapshots during the burst; after
  the return they drain first, often from a window of one segment (`.checkpoints/netcode-proof-test.md:233-235`).
  Fix 1 keeps only about 1.5 round trips of snapshots outstanding, so the first fresh snapshot leaves soon after the
  return: the median falls from 1,121 to 654 ms and the share over 1 s from 84 % to 8.7 % (Table 1).
- **What it leaves** is the old connection's own loss recovery. Every retransmission sent inside the burst is lost, and
  Linux doubles its retransmission timeout each time, so the next retry may come seconds after the link is back. After
  the burst, Linux's undo paths, a stale round-trip sample from the echoed timestamp (1.3-1.6 s) and a window rebuilt
  from one segment add more. Each step is a Linux 6.8 code path, traced in the model skeptic's notes
  (`u3-verify3-model.md:99-123`, `:146-159`, `:200-210`). Sure: high.
- **The tail it leaves:** 8.7 % of bursts over 1 s, 4.1 % over 1.5 s, 2.5 % over 2 s, max 11.4 s (Table 1). In a run of
  480 client-bursts that is on average 42, 20 and 12 slow recoveries (arithmetic: 480 × 87.3, 40.7 and 25.4 per mille,
  `u3v3-tables1.txt:124`).

### 2.2 What the reconnect does
- A fresh TCP connection inherits none of the old one's doubled timeouts or stale round-trip samples.
- The client starts it after a silence of max(300 ms, 2 × RTT + 100 ms), about 0.3-0.35 s into a 1 s burst at P2
  (`u3-verify2-harness.md:61-63`). Its first SYN is lost in the burst; the client's OS resends it 1 s later (Linux and
  current Windows), about 0.36 s after the return (`u3-verify3-harness.md:130-133`). Then TCP, TLS (resumed), the
  WebSocket upgrade, the `Join`, the full state and the first delta. Whichever connection delivers fresh data first
  wins.
- **Effect** (Table 1): p99 2,164 → 1,051 ms, p99.9 3,945 → 1,859 ms, max 11,384 → 4,196 ms, over 2 s 15,226 → 205.
  The median does not move (654 ms): the reconnect only decides the tail (`u3-verify3-harness.md:286-288`).
- **Both are needed.** The reconnect alone, with the 64 KB buffer, gives 844 / 1,374 / max 5,343 ms: the stale backlog
  slows the old connection, so most bursts end in a takeover (0.72 per burst before the recovery,
  `u3v3-tables1.txt:126-128`).
- **Its worst bursts** are the reconnect's own handshake losing a packet and waiting on a new connection's 1 s timer
  and its doublings: the 4,196 ms burst lost a SYN-ACK and a ClientHello, each a 2 % draw (`u3-verify3-model.md:200-210`;
  `u3-verify3-harness.md:99-104`).

### 2.3 What the proof test measures instead
- Fix 1 alone, with PM-0001-18's WebSocket part in a three-part form (a median part, a count over a time, a maximum).
  Its numbers come after one practice run on the real test setup, never looser than the model predicts for a correct
  fix-1 build (Mustafa, §1.1). The advisor writes the mark's exact words in the docs PR; this record does not.
- The model's bound for a correct fix-1 build at 480 client-bursts: "half within 0.67 s, at most 5 of 480 over 2.5 s,
  none over 6.9 s". A correct build passes it with chance 0.968 [0.960-0.975]; today's build 0.000; a budget twice too
  large 0.000; a correct build whose recoveries are all 5 % slower 0.001, or 20 ms later 0.159
  (`.gate/netcode-proof-test/u3v3-barfix1.txt`, from `ws_bar_fix1`, `fixes.rs:2004-2011`). At 1,440 client-bursts:
  "half within 0.67 s, at most 11 of 1,440 over 2.5 s, none over 9.8 s", 0.967. The fragility under a small time shift
  is why the numbers wait for the practice run.
