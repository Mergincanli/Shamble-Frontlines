# The WebSocket fallback's fast reconnect: a requirement for the online client

**Status:** Proposed. A requirement record (a design doc, not an ADR). The requirement and its target are Mustafa's
decision (§1.1); the docs PR that lands this record also writes them into ADR 0001 (part 11's reconnect clause,
OQ-0001-19 and migration step 5's *Done when*: §5.1). The design in §3 and §4 is proposed: the reconnect work brings
its further ADR 0001 changes (§5.2) for review when it starts.
**Date:** 2026-11-07 (revised the same day, after Mustafa's 14:30 UTC refinement of WS-BAR)
**Author:** netcode-architect
**Decided by:** Mustafa, 2026-11-07 (his words in §1.1)
**For:** whoever builds the online client's WebSocket fallback (ADR 0001's migration step 5, "Slice 3, online", which
ends when "Mustafa's first online playtest is run": `docs/adr/0001-multiplayer-netcode.md:1734-1738`), and the
reviewers of that work.
**Traces to:** ADR 0001 part 10's need, "VISION_V2 §6.1; §4 (four browsers)" (`0001-multiplayer-netcode.md:473`):
rooms and private matches (`docs/VISION_V2.md:82`), played in Chrome, Edge, Firefox and Safari (`:55`) and fair up to
about 100 ms of ping (`:58`). Part 10 tries WebTransport first and falls back to WebSocket (0001:423); this record
keeps the fallback playable after a short outage.
**Why it exists:** the netcode proof test measures the fallback's newest-snapshot-only buffer ("fix 1") but not the
reconnect. Mustafa asked for the reconnect to be recorded "so it can't get lost". The proof test's working notes are
deleted when its change-set ends, so this record carries its own numbers; the notes stay in git history and the model
outputs stay committed (§7).

**Where the lines point.** ADR 0001 as on main `69ea88e` and on the proof-test branch's `95a04a1` (the same file),
that is before the docs PR that lands this record; that PR shifts every line after 424, so each ADR cite in §5 also
quotes its words. `fixes.rs` and `tcp_model.rs` on branch `claude/netcode-proof-test-plan-hcky5g` at `bf9d248`, under
`spikes/netcode-proof/crates/net/src/` (`tcp_model/fixes.rs`, unchanged since; `tcp_model.rs`, last changed at
`709e44a`, equal at `bf9d248`). `.checkpoints/` files at `95a04a1` (§7 lists where each lives in git history). The
model's outputs are committed on the same branch at `975e72b` under `spikes/netcode-proof/results/u3-ws-fallback/`:
`tables-1e020a7.txt` (every table, printed at `1e020a7`), `bar-fix1-bf9d248.txt` (the fix-1 bound, `bf9d248`) and
`reconnect-policies-1e020a7.patch` (the third skeptic round's diagnostic patch); they are cited by file name and line.

## 0. In short

- **The requirement.** Before the first online playtest, the online client's WebSocket fallback reconnects fast after
  a total-loss burst, and the build is measured against today's model: with the newest-only buffer and the reconnect,
  99 % of client-bursts decode a fresh delta within about 1.05 s of the link's return after a 1 s outage at P2.
- **Where ADR 0001 holds it** (from the docs PR that lands this record): part 11's reconnect clause, the open question
  OQ-0001-19, and migration step 5's *Done when*, which asks for the reconnect to be built and measured against the
  target before the first online playtest, a miss going to Mustafa before that playtest (§5.1).
- **Why.** Fix 1 alone (what the proof test measures) brings the median down to 654 ms but leaves a long tail: p99
  2,164 ms, max 11,384 ms over 600,000 modelled bursts. The reconnect cuts the p99 to 1,051 ms and the max to 4,196 ms
  (Table 1).
- **The browser shapes the policy.** One connecting WebSocket per endpoint (RFC 6455 §4.1), Chromium's throttle on
  failed attempts, Firefox's fail delay. The client's operating system decides when an attempt made during the outage
  can connect (its SYN retry timer).
- **An open choice.** The modelled policy ("never abandoned") is tuned to the 1 s burst. An abort scaled to the round
  trip with "join or close" is more robust at other outage lengths but cost about 75 ms of p99 at the 1 s burst in a
  smaller diagnostic run; whether that meets "about 1.05 s" is Mustafa's call in the reconnect work (§3.7).
- **Credentials deferred.** How a reconnect proves who it is (RESUME-CRED) waits for the reconnect work; three
  readings of "bound to the session" are open (§4).
- **Measuring it.** The proof test's L2 setup cannot measure a reconnect as it is: it needs loss below TCP, a client
  that follows the browser's rules, and Windows-like clients (§6). How the pass bar is set stays open until the
  reconnect work's plan (WS-RECONNECT-BAR (b), Mustafa, 2026-11-08: §6.4).

## 1. The requirement and its target

### 1.1 Mustafa's words (2026-11-07, verbatim)

About 13:45 UTC:

> "WS-SCOPE: no. Measure fix 1 (the newest-snapshot-only buffer) in the proof test. The reconnect becomes a
> requirement for the real client, built and measured before the first online playtest, with today's model as its
> target (buffer + reconnect: 99 % within about 1.05 s). Record that in the docs PR so it can't get lost.
> RESUME-CRED: deferred with the reconnect. WS-BAR: the three-part form now; its numbers after one practice run on the
> real test setup, but never looser than the model predicts for a correct build with fix 1. If the practice run comes
> out worse than the model, find out why before setting the numbers."

About 14:30 UTC, refining WS-BAR:

> "Refinement to my WS-BAR rule: the model's bound has almost no margin (5 % slower already fails a correct build), so
> declare a tolerance now, before any practice run. If the practice run is up to 10 % slower than the model, the
> numbers may move by that much; beyond 10 %, investigate first and never loosen. The advisor may set a different
> tolerance with reasons, but it must be fixed before the run."

Source: the proof test's checkpoint, "Decisions taken" (`.checkpoints/netcode-proof-test.md` at `95a04a1`: the 13:45
words :357-362, the 14:30 words :349-353).

What each part means:
- **Fix 1, the newest-snapshot-only buffer.** The room writes a tick's snapshot over WebSocket only while the bytes the
  client has not yet acknowledged (written but unsent, and in flight: the socket's `SIOCOUTQ`, plus any still held
  above the socket) leave room for it within N snapshots, N = ceil(1.5 × srtt / tick) + 1, at least 3, the smoothed
  round trip from `TCP_INFO` (100 ms before its first sample). A snapshot that does not fit is skipped and counted,
  never sent later; nothing already written is dropped (`fixes.rs:4-5`, `:164-169`, `:776-789`). With no burst at P2
  the budget skips 21 snapshots per thousand (`tables-1e020a7.txt:66`). In Linux v6.8, `SIOCOUTQ` answers
  write_seq − snd_una, unsent plus unacknowledged (`net/ipv4/tcp.c:630`), and `tcpi_rtt` is srtt_us >> 3
  (`net/ipv4/tcp.c:3842`), as the verifier read them (`.checkpoints/netcode-proof-test/verify-ws-docs.md:27`); the
  model gives the same two readings (`tcp_model.rs:2167-2177`). ADR 0001 part 10 states the budget from the docs PR on
  (§5.1). The proof test builds and measures it; the online client's room keeps it.
- **The reconnect** ("fix 3" in the proof test's notes). After a silence, the client opens a fresh WebSocket
  connection and resumes its session on it with a `Join`; the newest `Join` takes the session over (`fixes.rs:8-14`;
  ADR 0001:506-508).
- **The real client.** The game's online client. ADR 0001's migration plan puts "reconnect" in step 5, slice 3
  (0001:1734-1738); its scope line lists part 11's reconnect among "slice 2 and slice 3 milestones" (0001:1640-1641).
  The docs PR adds the reconnect to slice 3's *Done when* (§5.1).
- **Built and measured before the first online playtest.** Built to §3, measured as §6 says, against §1.2's target.
  Mustafa's words say "measured against", not "meets": a miss goes to him before that playtest (§3.7, §5.1).
- **WS-BAR** is the proof test's bar for fix 1 alone (§2.3), not this record's. It and its 14:30 refinement are quoted
  for their rule about a practice run and a tolerance fixed before it, which §6.4 discusses borrowing.

### 1.2 "Today's model": exactly what the target is

- **The TCP model:** `linux-6.8-tcp v3` (`tcp_model.rs:79`), Linux 6.8's TCP with its default settings, every rule
  read in the kernel source (`tcp_model.rs:1-10`). The first two skeptic rounds found kernel paths missing, which v3
  added; the third found v3 faithful line by line, with no defect that moves the target's p99 (1,051 ms) or max
  (4,196 ms) (`u3-verify3-model.md:211-212`, `:236-242`); Linux 6.12's changes leave the rows identical (`:160-169`).
  Sure: high.
- **The harness:** `fixes.rs`. Its rows come from commit `1e020a7` (model and harness v3); `709e44a` and `bf9d248`
  move no number (`bf9d248` adds the fix-1 bound and a budget multiplier whose default, 1, changes nothing:
  `fixes.rs:74-75`, `:1208`, `:776`). Its printed outputs are committed (`results/u3-ws-fallback/README.md:10-14`).
- **The scenario:** profile P2 (100 ms round trip, ±20 ms jitter, 2 % loss, half each way:
  `.checkpoints/netcode-proof-test.md:417-418`); one 1 s total-loss burst per phase (B1); the room's 30 Hz tick;
  1,050 B snapshots and 88 B inputs; Chromium's rules; TLS resumed; a Linux client (`fixes.rs:30-42`, `:1199-1210`).
- **What "recovered" means:** the client decodes a delta the room wrote at or after the link's return; on a new
  connection, the first delta after the full state (`fixes.rs:19-21`, the mark at `:786-787`; PM-0001-18's wording,
  ADR 0001:1556, which the docs PR keeps as "a delta encoded at or after that moment").
- **"Buffer + reconnect"** is the row "fix 1 + fix 3: one endpoint, never abandoned (recommended)"
  (`fixes.rs:1236-1239`, `recommended()` at `:1259-1263`): trigger max(300 ms, 2 × echo RTT + 100 ms); one endpoint;
  one attempt at a time, starts at least 300 ms apart; never closed by a timer; at most 8 attempts per outage
  (`fixes.rs:1-14`, `serial()` at `:1187-1197`); an attempt still before its 101 closed once the old connection
  delivers fresh data (`:706-734`); the newest `Join` resuming the session, with a full state and no snapshot that tick
  (`:736-790`). It is pinned by `the_recommended_fixes_are_pinned_at_b1` (`fixes.rs:1833-1858`).

**Table 1. 600,000 bursts at P2, 1 s burst, ms** (`tables-1e020a7.txt:120-131`; the same rows reproduced by the
statistics skeptic, `u3-verify3-stats.md:30-31`)

| Row | p50 | p99 | p99.9 | p99.99 | max | over 1 s | over 1.5 s | over 2 s |
|---|---|---|---|---|---|---|---|---|
| Today (64 KB buffer, no fix) | 1,121 | 2,686 | 4,068 | 6,484 | 12,068 | 505,371 | 64,398 | 21,968 |
| Fix 1 alone (the proof test) | 654 | 2,164 | 3,945 | 5,801 | 11,384 | 52,406 | 24,442 | 15,226 |
| The reconnect alone (64 KB buffer) | 844 | 1,374 | 1,952 | 2,297 | 5,343 | 107,048 | 3,180 | 379 |
| **The target: buffer + reconnect** | 654 | **1,051** | 1,859 | 2,092 | 4,196 | 6,768 | 1,009 | 205 |

The target's share over 1 s is 11.28 per mille (Wilson 95 %: 11.02-11.55); 10 bursts are over 2.5 s and 4 over 3 s.
Per burst it makes 1.0 attempt; 0.664 attempts are closed because the old connection recovered first; 0.161 takeovers
happen before the recovery and 0.176 after it (`tables-1e020a7.txt:130-131`).

**Table 2. Client variants of the target, 60,000 bursts each, ms** (`tables-1e020a7.txt:88-117`, except the last row)

| Variant | p50 | p99 | p99.9 | p99.99 | max |
|---|---|---|---|---|---|
| The target's row at 60,000 (Linux client, Chromium) | 654 | 1,054 | 1,827 | 2,066 | 3,013 |
| The same in Firefox | 654 | 1,054 | 1,827 | 2,066 | 3,013 |
| Full TLS handshakes (no session resumption) | 654 | 1,091 | 1,838 | 2,121 | 3,013 |
| No TCP timestamps (a Windows peer) | 645 | 886 | 1,484 | 1,969 | 2,165 |
| Windows-like client, first SYN retry after 1 s (current Windows) | 645 | 886 | 1,484 | 2,046 | 2,871 |
| Windows-like client, first SYN retry after 3 s (older Windows) | 646 | 2,035 | 2,729 | 3,069 | 3,890 |
| Fix 1 alone on a no-timestamp client (the same-client baseline) | 646 | 2,035 | 3,829 | 5,295 | 12,550 |

The last row is from the third skeptic round's diagnostic patch (its `diag_rows`,
`reconnect-policies-1e020a7.patch:179`), not from `fixes.rs` (`u3-verify3-harness.md:79-87`). Maxima grow with the
sample (3,013 at 60,000, 4,196 at 600,000); the p99 is stable (1,054 vs 1,051) (`u3-verify3-model.md:200-210`). The
Windows-like rows model only the client's SYN timer and the missing timestamps; Windows' own loss recovery for the
client's sends is not modelled, which may move the median by tens of ms (`u3-verify3-harness.md:155-161`;
`u3-verify3-model.md:213-216`). Sure: medium.

- **Mustafa's laptop** (Windows): `Get-NetTCPSetting` shows MaxSynRetransmissions 4 in every template and
  InitialRtoMs blank (the built-in default), read as a first SYN retry after 1 s, then doubling: SYNs at 0, 1, 3, 7 and
  15 s (`.checkpoints/netcode-proof-test.md:337-346`; Catchpoint, "TCP SYN retries", read 2026-11-07:
  docs.catchpoint.com/docs/catchpoint-nodes-tcp-syn-retries). So the 1 s Windows-like row applies to him; the 3 s row
  applies to older Windows and Windows Server templates (`u3-verify3-harness.md:74-78`, `:196-206`). Sure: medium-high
  (his InitialRtoMs reads blank, not 1000).

## 2. Why fix 1 alone falls short, and what the reconnect does

### 2.1 What fix 1 fixes, and what it leaves
- **Today**, a backed-up WebSocket fills TCP's 64 KB write buffer with about 30 stale snapshots during the burst; after
  the return they drain first, often from a window of one segment (`.checkpoints/netcode-proof-test.md:234-235`).
  Fix 1 keeps only about 1.5 round trips of snapshots outstanding, so the first fresh snapshot leaves soon after the
  return: the median falls from 1,121 to 654 ms and the share over 1 s from 84 % to 8.7 % (Table 1).
- **What it leaves** is the old connection's own loss recovery. Every retransmission sent inside the burst is lost, and
  Linux doubles its retransmission timeout each time, so the next retry may come seconds after the link is back. After
  the burst, Linux's undo paths, a stale round-trip sample from the echoed timestamp (1.3-1.6 s) and a window rebuilt
  from one segment add more. Each step is a Linux 6.8 code path, traced in the model skeptic's notes
  (`u3-verify3-model.md:99-123`, `:146-159`, `:200-210`). Sure: high.
- **The tail it leaves:** 8.7 % of bursts over 1 s, 4.1 % over 1.5 s, 2.5 % over 2 s, max 11.4 s (Table 1). In a run of
  480 client-bursts that is on average 42, 20 and 12 slow recoveries (arithmetic: 480 × 87.3, 40.7 and 25.4 per mille,
  `tables-1e020a7.txt:124`).

### 2.2 What the reconnect does
- A fresh TCP connection inherits none of the old one's doubled timeouts or stale round-trip samples.
- The client starts it after a silence of max(300 ms, 2 × RTT + 100 ms), about 0.3-0.35 s into a 1 s burst at P2
  (`u3-verify2-harness.md:61-63`). Its first SYN is lost in the burst; the client's OS resends it 1 s later (Linux and
  current Windows), about 0.36 s after the return (`u3-verify3-harness.md:130-133`). Then TCP, TLS (resumed), the
  WebSocket upgrade, the `Join`, the full state and the first delta. Whichever connection delivers fresh data first
  wins.
- **Effect** (Table 1): p99 from 2,164 to 1,051 ms, p99.9 from 3,945 to 1,859 ms, max from 11,384 to 4,196 ms, bursts
  over 2 s from 15,226 to 205. The median does not move (654 ms): the reconnect decides only the tail
  (`u3-verify3-harness.md:286-288`).
- **Both are needed.** The reconnect alone, with the 64 KB buffer, gives 844 / 1,374 / max 5,343 ms: the stale backlog
  slows the old connection, so most bursts end in a takeover (0.72 per burst before the recovery,
  `tables-1e020a7.txt:126-128`).
- **Its worst bursts** are the reconnect's own handshake losing a packet and waiting on a new connection's 1 s timer
  and its doublings: the 4,196 ms burst lost a SYN-ACK and a ClientHello, each a 2 % draw
  (`u3-verify3-model.md:200-210`; `u3-verify3-harness.md:99-104`).

### 2.3 What the proof test measures instead
- **Fix 1 alone**, as PM-0001-18's WebSocket part: three checks over the run's client-bursts (the median, at most k
  over a time, none over a maximum), their numbers set from the model's bound for a correct fix-1 build and one
  unjudged practice run on L2, with a tolerance fixed before that run (Mustafa, §1.1). The rule, how "slower than the
  model" is measured and how the numbers move are PM-0001-18's row in ADR 0001 (from the docs PR on) and the proof
  test's plan. This record does not restate them, so the two cannot drift.
- **The first bound**, for 480 independent single bursts with 1,050 B snapshots: "half within 0.67 s, at most 5 of 480
  over 2.5 s, none over 6.9 s" (`bar-fix1-bf9d248.txt:11`; `ws_bar_fix1`, `fixes.rs:2000-2011`). A correct build
  passes it with chance 0.968; today's build and a budget twice too large 0.000; a correct build whose recoveries are
  all 5 % slower 0.001, or 20 ms later 0.159 (same line). That missing margin is what Mustafa's 14:30 tolerance answers.
  The proof test recomputes the bound for the run as it runs (48 clients sharing each burst, 10 bursts per client, each
  burst's start moved by a random offset within one tick, the scene's real snapshot sizes) before its run day.
- **What it gives the reconnect work:** the reconnect leaves the median where fix 1 put it (654 ms for both,
  `tables-1e020a7.txt:123`, `:129`), so the proof test's measured fix-1 median is also the median a correct reconnect
  build should show on the same profile. A reconnect build whose median moves away from it is doing something the
  model does not (false takeovers, a slower full state); §6.4 uses this.

## 3. The policy: what the builder must get right

Three rounds of skeptic review read the browsers' sources and ran the harness against them. What follows is what a
reconnect that browsers can run must do. "Rule" marks a requirement; the rest is the evidence.

### 3.1 What a page can see
- A page sees only `open`, `message`, `error` and `close`. `CONNECTING` has no sub-stages, a failure's cause is hidden
  (close code 1006), and TCP's round trip is not exposed (WHATWG WebSockets Standard, read 2026-11-06;
  `u3-verify-fixes-adr.md:153-157`). Sure: high.
- So a page cannot tell a lost SYN from a slow handshake, and its trigger must come from the game's own messages.

### 3.2 One connecting attempt per endpoint: the browsers serialize
- **RFC 6455 §4.1**, client requirement 2: "There MUST be no more than one connection in a CONNECTING state." and "If
  multiple connections to the same IP address are attempted simultaneously, the client MUST serialize them"
  (rfc-editor.org/rfc/rfc6455.html, re-read 2026-11-07; the verifier read the same in rfc6455.txt :814-822,
  `verify-ws-docs.md:39`). Sure: high.
- **Chromium** (so Brave, Chrome and Edge): an endpoint lock keyed by IP, port and network anonymization key, taken
  before the TCP connect, held through TLS and the upgrade, and freed 10 ms after the 101 or the attempt's end
  (`websocket_endpoint_lock_manager.cc:26`, `:28-45`, `:47-58`, `:142-159`; `transport_connect_sub_job.cc:117-131`;
  `u3-verify2-harness.md:24-36`). Its handshake timeout is 240 s and its connect job 4 minutes, so an attempt the page
  never closes lives until the OS gives up its SYNs (`u3-verify2-harness.md:37-41`). With IPv6 and IPv4 both
  published, Chromium starts the IPv4 try 300 ms after the IPv6 one, each with its own lock (same lines). Sure: high.
- **Firefox:** its admission manager lets one WebSocket per address be connecting, ignoring the port; an attempt not
  open 20 s after it began fails (`WebSocketChannel.cpp:540-552`, `:1181`, `:1393`; `u3-verify3-harness.md:57-64`;
  `u3-verify-fixes-adr.md:102-109`). Sure: high.
- **Consequences.** Parallel attempts to one endpoint run in no browser: the harness's parallel row is a reference
  only (`fixes.rs:1252-1255`; `tables-1e020a7.txt:96`). A second port helps Chromium only, since Firefox queues per
  address (`u3-verify-fixes-adr.md:110-118`). **Rule:** one endpoint per room (ADR 0001:429-432 stands) and one
  attempt at a time.

### 3.3 The browsers' throttles
- **Chromium's per-renderer throttler.** Each new WebSocket waits RandInt(1,000..5,000) × 2^min(p + f/(s+1), 16) /
  65,536 ms in integer arithmetic: p pending attempts, f failed (ended before their 101), s succeeded (reached their
  101), counted over the current and the previous 2-minute period; a period with no success erases it. On Windows
  (clang's Microsoft ABI) the delay is computed before the new attempt is counted
  (`services/network/websocket_throttler.cc:27-33`, `:46-55`, `:62-76`, `:124-144`; `websocket_factory.cc:93-101`;
  `u3-verify3-harness.md:35-56`). Sure: high.
- **Its effect.** A policy that closes attempts before their 101 ratchets it up. Aborting at 500 ms, 30 bursts per
  client: delays up to 4,994 ms and a worst recovery of 5,220 ms (`tables-1e020a7.txt:44-45`; mechanism
  `u3-verify3-harness.md:90-98`). The target's policy: delays up to 421 ms (Windows order) or 1,059 ms (other
  systems' order) after 30 bursts (`tables-1e020a7.txt:22-23`; `u3-verify3-harness.md:51-53`).
- **Firefox's fail delay.** After a failed or page-closed attempt with no record yet, the next attempt to that (IP,
  path, port) waits 200-400 ms; a further network failure grows it ×1.5 up to 60 s (a page's close does not grow it);
  a successful open clears it; it expires 60 s plus its delay after the last failure (`WebSocketChannel.cpp:113-160`,
  `:389-421`; `u3-verify2-harness.md:46-55`; `u3-verify3-harness.md:57-64`). The failure of an established connection
  (a reset, a close that times out) adds or grows it too (`u3-verify3-harness.md:65-73`). Sure: high.
- **Its effect.** Aborting at 1 s: 4,601 of 60,000 bursts over 1 s in Firefox against 700 in Chromium
  (`tables-1e020a7.txt:92`, `:108`).
- **Rules:** end attempts before their 101 as rarely as the policy allows (join-or-close, §3.6); the room never resets
  a connection it replaces (§3.9).

### 3.4 The trigger
- **Rule:** start an attempt after a silence of max(300 ms, 2 × RTT + 100 ms) between whole messages (a page sees
  only whole messages) (`fixes.rs:8-10`, `:171-174`; `u3-verify2-harness.md:148-151`).
- **Rule:** take the RTT from the input echo: the time from sending an input to the first snapshot whose header says
  the room received it, the header's "newest command received" (ADR 0001:668), not "last processed command", which
  adds the input buffer (33.3 ms a buffered tick) twice (`u3-verify2-harness.md:78-84`). Smooth it with gain 1/8
  (`fixes.rs:666-669`). At P2 the trigger lands at about 0.3-0.35 s (`u3-verify2-harness.md:61-63`).
- **False triggers** with no burst, in 30 minutes at P1/P2/P3/P4: 0/9/15/7 attempts and 0/2/1/1 takeovers
  (`tables-1e020a7.txt:69-72`). Each attempt is a TLS handshake on the room; a takeover's cost is in §3.8.
- **Test the trigger directly.** A trigger floor of 600 ms instead of 300 ms passes a 480-burst bar 98 % of the time,
  and a 1 s floor even shows a better median at a 1 s burst (its SYN leaves at the return) only by luck of the burst
  length (`u3-verify3-stats.md:95-107`). Unit and property tests of the trigger catch both exactly.

### 3.5 Attempts and their limits
- **Rule:** "busy" is what the page sees: an attempt it has not switched to or closed, never the room's state
  (`fixes.rs:584-605`; `u3-verify2-harness.md:68-71`).
- **Rule:** at most 8 attempts per outage, counted again from each whole snapshot on the current connection and at
  a switch. A count per session would stop a client reconnecting after its eighth outage
  (`fixes.rs:89-90`, `:670-676`, `:692-695`, `:792-797`; `u3-verify2-harness.md:72-77`).
- **Rule:** a WebSocket session reconnects over WebSocket at once. ADR 0001:423's "WebTransport is tried first with a
  timeout (proposed 1.5 s), then WebSocket" applies to the first connect only; read for reconnects too, it adds up to
  1.5 s and no number here holds (`u3-verify-fixes-adr.md:134-137`).
- **RFC 6455 §7.2.3:** "clients SHOULD use some form of backoff when trying to reconnect after abnormal closures", "The
  first reconnect attempt SHOULD be delayed by a random amount of time" (0-5 s suggested) (re-read 2026-11-07;
  `verify-ws-docs.md:39`). The trigger fires while the old connection is still open, so it is not that case. But a
  whole room reconnecting at once is what §7.2.3 guards against, so the room's limits must absorb it (§3.9), and after
  a closure the page does see (`close` or `error`) and after the cap, the page should back off as §7.2.3 says (my
  reading; the reconnect work decides).

### 3.6 When the old connection recovers first: cancel, or join-or-close
- **Cancel** (the target's policy). The page closes an attempt still before its 101 once the current connection
  delivers a whole snapshot the room wrote at or after the attempt's creation; after its 101 the attempt runs to its
  `Join` (`fixes.rs:706-734`, `:653-658`, `:803-806`). Without it, 99 % of bursts ended in a needless takeover in the
  second round (`u3-verify2-harness.md:61-67`). The page needs the snapshot header's server tick and its own estimate
  of the server tick when the attempt began; clock-sync error of milliseconds against a 33 ms tick does not matter
  (`u3-verify3-harness.md:177-179`).
- **Its cost.** Each cancel is a close before the 101: a failure in Chromium's throttler and a record in Firefox's fail
  delay (§3.3).
- **Join-or-close.** Never close an attempt because the old connection recovered. At its 101, send the `Join` only if
  the old connection has delivered no whole snapshot written at or after the attempt's creation; else close it cleanly
  (code 1000) without a `Join`. Single bursts recover exactly as with cancel; every such attempt counts as a success in
  Chromium and clears Firefox's record. Over 30 bursts no burst was throttled in any policy, and the 500 ms abort's
  p99.9 fell from 1,914 to 1,408 ms and its worst from 5,220 to 2,105 ms. Cost: extra TCP and TLS handshakes and
  upgrades that the room sees closed without a `Join`, about 0.55-0.67 per outage at a 1 s burst and up to 0.77 at
  other lengths; attempts may linger for seconds, holding the browser's lock (`u3-verify3-harness.md:109-121`,
  `:228-231`). Measured only with the diagnostic patch (`diag_nocancel`, `reconnect-policies-1e020a7.patch:239`).
  Sure: medium.
- **Rule:** once a `Join` is sent, let it land: closing it then could let the room move the session to a dead
  connection (`u3-verify3-harness.md:105-108`).
- **Recommendation:** build join-or-close whichever attempt policy §3.7 picks.

### 3.7 Never abandoned, or an abort scaled to the round trip
- **Never abandoned** (the target's policy): one attempt per outage, carried by the OS's SYN retries; the best p99 at
  a 1 s burst on Linux and current Windows; 1.0 attempt and 0.21-0.49 takeovers per outage; no browser rule binds, so
  Firefox equals Chromium. Its worst bursts are its own handshake stuck in backoff (`u3-verify3-harness.md:99-104`,
  `:208-215`).
- **Scaled abort:** an attempt still connecting max(500 ms, 4 × echo RTT) after it began is closed and the next starts
  at once, with join-or-close. A fixed 500 ms fails at P4, where attempts die before their 101
  (`u3-verify3-harness.md:122-129`).

**Table 3. Profiles, 1 s burst, 20,000 bursts each; p99 / p99.9 / max, ms** (`u3-verify3-harness.md:122-129`; the
diagnostic patch's `diag_profiles`, `reconnect-policies-1e020a7.patch:311`, not in `fixes.rs`; sure: medium, one run,
not pinned)

| Case | Never abandoned | Scaled abort + join-or-close |
|---|---|---|
| P2, Linux client | 1,053 / 1,840 / 3,013 | 1,126 / 1,498 / 2,544 |
| P3, Linux client | 1,887 / 2,781 / 4,872 | 1,907 / 2,734 / 3,630 |
| P4, Linux client | 2,410 / 3,087 / 4,559 | 2,502 / 3,418 / 6,189 |
| P2, first SYN retry after 3 s | 2,027 / 2,729 / 3,890 | 924 / 1,343 / 2,607 |
| P3, first SYN retry after 3 s | 2,774 / 3,869 / 7,205 | 1,727 / 2,551 / 3,640 |
| P4, first SYN retry after 3 s | 3,307 / 4,375 / 10,316 | 2,393 / 3,305 / 7,357 |

**Table 4. Outage lengths at P2, Linux client, 20,000 bursts each; p50 / p99 / max, ms** (`u3-verify3-harness.md:
130-140`; the patch's `diag_lengths`, `reconnect-policies-1e020a7.patch:274`; the 1 s row is Table 1's)

| Burst | Fix 1 alone | Never abandoned | Scaled abort + join-or-close |
|---|---|---|---|
| 0.6 s | 964 / 2,454 / 5,722 | 964 / 1,270 / 2,570 | 726 / 1,135 / 1,901 |
| 1.0 s | 654 / 2,164 / 11,384 | 654 / 1,051 / 4,196 | p99 1,126 (Table 3) |
| 1.5 s | 1,330 / 2,483 / 12,514 | 1,330 / 1,632 / 3,655 | 844 / 1,378 / 2,297 |
| 2.5 s | 493 / 6,462 / 20,855 | 493 / 2,466 / 4,289 | 488 / 1,463 / 3,228 |

- Over 1 s at a 0.6 s burst: 8,780 never abandoned, 1,225 scaled. Over 1.5 s at a 1.5 s burst: 613 against 35; at a
  2.5 s burst 2,501 against 166. On a 3 s-SYN client the scaled abort wins at every length (`u3-verify3-harness.md:
  126-127`, `:133-135`).
- Costs per outage at 0.6 / 1 / 1.5 / 2.5 s: scaled abort 1.9 / 2.8 / 4.0 / 5.4 attempts and 0.70 / 0.46 / 0.98 / 0.45
  takeovers; never abandoned 1.0 attempt and 0.21 / 0.34 / 0.49 / 0.41 takeovers (`u3-verify3-harness.md:136-137`).
- **Why the 1 s burst favours never abandoned:** its attempt starts about 0.36 s into the burst, and Linux resends the
  SYN 1 s later, 0.36 s after the return (`u3-verify3-harness.md:130-133`). Current Windows doubles (retries at 1, 3
  and 7 s after the first SYN), so at a 1.5 s burst its second retry comes 1.86 s after the return, worse than Linux's
  (`:138-140`; reasoned from the schedule, not run).
- **What this means for the builder.** The target (p99 about 1.05 s) was measured with never abandoned at a 1 s burst.
  The scaled abort gave p99 1,126 ms there, about 75 ms (7 %) above it, on 20,000 bursts of a throwaway patch. Picking
  the policy that is robust across outage lengths may miss "about 1.05 s" by that much; whether it counts as "about
  1.05 s" is Mustafa's call in that work, with both policies' 600,000-burst rows in front of him (ADR 0001's OQ-0001-19
  says the same, from the docs PR on). Any miss goes to him before the first online playtest (§5.1, slice 3's
  *Done when*).
- **Recommendation** (as the third skeptic round's, `u3-verify3-harness.md:218-236`): add the scaled abort and
  join-or-close to `fixes.rs` as options (the patch shows one way); re-run the 600,000-burst tail, the multi-burst and
  side-effect tables and the 0.6 / 1.5 / 2.5 s bursts; then choose.

### 3.8 Takeovers and their cost
- With the target's policy, about a third of bursts end on a new connection: 0.161 takeovers before the recovery and
  0.176 after it per burst (`tables-1e020a7.txt:131`).
- **A takeover costs** a TCP and TLS handshake on the room (a key exchange, and a signature unless resumed), a
  credential (§4), a full state (4,200 B in the model, `fixes.rs:42`), a replay-cache entry, the old connection's
  queued snapshots, and the inputs the client still sends on the old connection until the full state arrives: about
  half a round trip plus the full state's transfer, 3-5 commands at P4 (`u3-verify2-harness.md:105-116`). It costs no
  extra silence (same lines).
- **Rule:** the room keeps reading commands from the old connection until the new one's first command, or those
  commands are lost (`u3-verify2-harness.md:111-115`).
- **Rule:** the room closes the old connection gracefully, so its queued bytes still go out: in the second round, 50
  of 60,000 bursts recovered on the old connection after the room had already moved the session
  (`u3-verify2-harness.md:115-116`).

### 3.9 The room's side
- **Rule:** the newest `Join` for a player wins; the older connection is closed (ADR 0001:507-508).
- **Rule:** at a takeover the room sends a full state (the model: on the tick, with no delta that tick,
  `fixes.rs:736-762`) or a delta against the client's last acknowledged snapshot. ADR 0001:672 says `FullState` "at
  join and on resync": the reconnect work states which (`u3-verify2-harness.md:117-123`). The target counts the first
  delta after the full state; counting the full state itself gives p99 1,022 ms (`tables-1e020a7.txt:104`).
- **Rule:** close a replaced connection with a WebSocket close frame and a graceful TCP close (keep reading until the
  client closes), never a reset. Linux sends a reset when it closes a socket with unread input, and Firefox grows its
  fail-delay record for that (IP, path, port) on any failure, so repeated resets would delay later reconnects
  (`u3-verify3-harness.md:65-73`). Not measured.
- **Rule:** `Join` once per connection (ADR 0001:670 says "once").
- **The `Join` deadline per attempt (open).** ADR 0001:445-446 gives every connection 2 s from the TCP accept to its
  `Join`, "so TLS and the WebSocket upgrade fall inside it". The harness does not model it. Without a loss the
  handshake after the accept takes about 2.5 round trips (the resumed TLS flights, the upgrade and the `Join`:
  `fixes.rs:37-42`, `:108-135`; my arithmetic), about 0.25 s at P2. But a handshake retry inflates the new
  connection's first retransmission timeout, and a loss after it can pass 2 s: the 600,000-burst maximum's reconnect
  connected at +1,254 ms, had a first RTO of about 2.4 s from a 1.1 s SYN-ACK sample, lost its ClientHello and sent it
  again at +3,754 ms, about 2.5 s after the connect (`u3-verify3-model.md:205-208`). A room enforcing the rule would
  have closed that attempt, and the page would have started another. It takes two 2 % losses on one takeover, so the
  p99 stays where it is; p99.9 and the maximum may move either way (my reasoning, not run). **Choice for the reconnect
  work:** keep 2 s for every connection and add the rule to the harness before the bar is set, or lengthen it for
  every connection (a security-auditor question: the room cannot tell a reconnect from a new join before its `Join`).
  Join-or-close attempts that close without a `Join` stay inside the rule (`u3-verify3-harness.md:119-121`). Sure:
  medium.
- **Rule:** size the admission limits for a room-wide outage. Per player during an outage: 2 connections (old plus one
  attempt) and 1.5-5.4 SYNs. After a room-wide outage: about 48 TLS handshakes and 15-47 `Join`s within 1-2 s, from one
  address behind a LAN, a carrier NAT or the load generator; join-or-close adds 0.55-0.77 upgrades per player that
  close without a `Join`. ADR 0001:445-451's per-address cap, handshake cap and new-session rate must count per player
  or scale with the room (`u3-verify3-harness.md:242-246`).
- **TLS session resumption.** Chromium keys its session cache by host and port plus destination IP, network
  anonymization key, privacy mode and proxy chain, and keeps two single-use TLS 1.3 sessions per entry
  (`ssl_client_session_cache.h:45-62`, `:110-129`); rustls 0.23.46 servers send 2 tickets by default and keep 256
  sessions in memory (docs.rs, `ServerConfig`, read 2026-11-07) (`u3-verify3-harness.md:141-154`). **Rule:** the room
  keeps 2-3 tickets per player (256 cover one 48-player room). An abort policy uses a ticket per attempt and can run
  out, falling back to full handshakes: p99 1,054 → 1,091 ms (`tables-1e020a7.txt:100`). The key-exchange provider is
  not chosen yet: aws-lc-rs with defaults puts X25519MLKEM768 first (a bigger server flight); ring has no ML-KEM group
  (`u3-verify2-harness.md:85-93`). The model's resumed flights: a 1,900 B ClientHello and a 1,300 B server flight
  (`fixes.rs:108-135`).

### 3.10 The client OS's SYN timer
- **Linux:** the SYN is resent after 1 s, linearly for 4 tries, then doubling (`tcp_model.rs:105-106`, `:210-213`,
  `:230-235`).
- **Current Windows** (10 and 11): first retry after 1 s, then doubling, 4 retries: SYNs at 0, 1, 3, 7 and 15 s
  (Catchpoint, read 2026-11-07); Mustafa's laptop matches (§1.2).
- **Older Windows and Windows Server 2019's templates:** 3 s, 2 retries (Microsoft Tech Community, "TCP Templates for
  Windows Server 2019", 2018-10-03, read 2026-11-07; `u3-verify3-harness.md:74-78`).
- **Its effect.** The reconnect's gain at a 1 s burst rests on the 1 s retry. With 3 s, never abandoned gains nothing
  up to 2 s over fix 1 alone on the same client (both p99 2,035 ms) but still cuts the worst from 12,550 to 3,890 ms;
  the scaled abort never waits for a SYN retry, so it gives the same 645 / 903 / 1,347 / 2,607 ms (p50 / p99 / p99.9 /
  max) for 1, 2 and 3 s (`u3-verify3-harness.md:79-87`; `tables-1e020a7.txt:112`, `:116`).
- **Not modelled** at the Windows client end: its own retransmission rules for the client's sends (a 300 ms minimum
  in the template read), and whether it sends D-SACKs (`u3-verify3-harness.md:155-161`). Direction unclear, small.

## 4. RESUME-CRED: how a reconnect proves who it is (deferred)

- **Deferred** with the reconnect (Mustafa, §1.1). His earlier answer, before the deferral (2026-11-06, about 23:40
  UTC): "RESUME-CRED: (a), with the spare tokens kept in memory only, short-lived and bound to the session"
  (`.checkpoints/netcode-proof-test.md:370-371`). It is the lean on record, not a decision. ADR 0001's OQ-0001-19
  lists the same options, from the docs PR on (§5.1).
- **What the target assumes.** The `Join` on the new connection carries a credential already in hand: 140 B, no lobby
  round trip (`fixes.rs:37-42`). Fetching one during the outage would add to every number in §1.2.
- **ADR 0001 today.** A join token from the lobby, checked offline: player id, guest flag, display name, room id as
  audience, team or slot if pre-assigned, expiry 30 s after issue with at most 5 s of leeway, a unique id
  (0001:494-497); a bounded replay cache (0001:503-504); on a reconnect "the client fetches a fresh token"
  (0001:506-507). Q22 chose "A. Hold the slot, rejoin with a fresh token" and turned down "B. A room-issued resume
  secret" ("a second credential to protect", 0001:962-963). No rule says when the fresh token is fetched, so a spare
  fetched ahead breaks no token rule (`u3-verify-fixes-adr.md:149-152`).

### 4.1 The open question: what "bound to the session" means
Three readings (`u3-verify2-harness.md:195-207`; `u3-verify-fixes-adr.md:237-240`):
1. **Bound to the player and the room.** Already true: the token carries the player id and the room as audience.
   Only words change.
2. **Bound to the live session.** The room gives a session id in `JoinAccepted` (0001:671); the Worker sends it with
   each spare request (0001:682); the token gains a session claim (0001:494-497); the room refuses a `Join` whose
   session is not that player's current one or one inside the grace window. Gain: a spare leaked from one match is
   useless in a later one. It does not stop a hijack while the session lives (the newest `Join` wins); only expiry
   limits that. Cost: a field, a claim, a check, and the lobby signs a value it cannot verify.
3. **A room-issued resume secret.** That is Q22's option B, turned down in the ADR (0001:963). The proof test already
   uses one for browsers after a Worker trap (its plan, §10.3: `.checkpoints/netcode-proof-test/plan.md:1024-1025`).

### 4.2 Facts that hold whichever reading wins
- **Refresh.** With a 30 s expiry and 5 s of leeway (0001:496), spares must be refreshed about every 25 s whether used
  or not, and topped up at once after each use (`u3-verify-fixes-adr.md:144-148`; `u3-verify2-harness.md:167-169`).
  That is about 2.4 lobby calls a minute per player, about 2 a second for a 48-player room, and 48 top-ups at once
  after a room-wide outage (`u3-verify2-harness.md:175-176`).
- **Use.** One spare per takeover: 0.34 per outage under the target's policy (0.161 + 0.176, Table 1), up to 0.98 with
  the scaled abort at a 1.5 s outage; join-or-close attempts that close without a `Join` use none
  (`u3-verify3-harness.md:247-248`, `:136-137`).
- **Where they live.** The Worker owns the connection (0001:616-617), so spares sit in the Worker's memory, never in
  storage. ADR 0001's "the guest secret out of script's reach" (0001:783) stays true only if spares are short-lived:
  each sits in script for at most about 35 s (`u3-verify-fixes-adr.md:161-163`; `u3-verify2-harness.md:177`, `:184`).
- **The lobby down.** Reconnects keep working until the spares expire, about 25-35 s (0001:759 gains this).
- **Fetching during the reconnect** (a token fetched alongside the handshake): in a real outage the lobby sits behind
  the same broken link, so the fetch finishes only after the return plus its own TCP recovery, and the `Join` waits
  for it (`u3-verify2-harness.md:191-193`). A point for spares held in advance.
- **Who decides** (as OQ-0001-19 names them): Mustafa the credential, and the target if the chosen policy misses it;
  netcode-architect with security-auditor the design; the Advisor confirming.

## 5. The ADR 0001 lines: what the docs PR changes now, and what the reconnect work changes later

Checked on 2026-11-07 against `docs/adr/0001-multiplayer-netcode.md` at `95a04a1`, equal to main `69ea88e`. Line
numbers are as before the docs PR that lands this record; that PR shifts every line after 424, so each row quotes the
words that find it. "E0"-"E13" are the docs PR's edits as its words name them.

### 5.1 Changed by the docs PR that lands this record (Mustafa's 2026-11-07 decisions)

| Line (words to find it) | What the docs PR writes | What the reconnect work does to it later |
|---|---|---|
| 424 ("Over WebSocket the server sends only the newest snapshot and drops any unsent older one") | E0: fix 1's send budget (the bytes the client has not acknowledged leave room for the tick's snapshot within N snapshots; a snapshot that does not fit is skipped and counted, never sent later; `SIOCOUTQ` and `TCP_INFO`) | Nothing: the budget stays (§1.1). |
| 506-509 (part 11, "*Reconnect:* slot, team and squad are held"; ends "the record is ADR 0004's). *Private rooms:*") | E5: one sentence before "*Private rooms:*": the fast reconnect over the WebSocket fallback is a requirement of the real client, built and measured before the first online playtest (migration step 5, slice 3) with today's model as its target; its design, its credential (RESUME-CRED, deferred) and its changes to parts 10 and 11 wait for that work; links OQ-0001-19 and this record | Rewrites the clause's rule itself to §3 (§5.2). |
| New, between 1250 (OQ-0001-18's end, "a room process that dies ends only its own match") and 1252 ("## What a spike must prove") | E6: OQ-0001-19 (WS-RECONNECT, RESUME-CRED): Mustafa's words; fix 1 alone against buffer plus reconnect from Table 1; the options (the attempt policy, §3.6-§3.7; the credential, §4); the deciders; what decides it (built in slice 3, measured before the first online playtest with loss below TCP, on each browser engine and on Windows' TCP, §6); links this record | Answers it: the policy, the credential and the measurement against the target; the register then closes it. |
| 747 (failure row "A burst of total loss (B1)") | E2: over WebSocket TCP resends what the burst lost and the send budget keeps stale snapshots from queueing; WebTransport's 500 ms; the fallback's PM-0001-18 checks; the real client's fast reconnect required before the first online playtest (OQ-0001-19) | Adds the reconnect's own expectation (its bar, §6.4). |
| 1295 ("What a spike must prove" item 2: "deltas again within 500 ms after B1") | E3: 500 ms on WebTransport; PM-0001-18's three checks on the WebSocket fallback | Nothing: the item is the proof test's. |
| 1462-1469 (part 13's key: "For the proof test, the WebSocket delay is a stream wrapper inside the server's accept path" through "the message size limit applies before buffering (the advisor's reading, 2026-11-06).") | E4: the wrapper above TLS and below the WebSocket library (U3-Q4 (b)), modelling Linux 6.8's TCP (`linux-6.8-tcp v3`) instead of a fixed retransmission timeout | Nothing: the key scopes the wrapper to the proof test; §6.1 says why it cannot measure a reconnect. |
| 1556 (PM-0001-18, "every client decodes deltas again within 500 ms of the link coming back") | E1: WebTransport 500 ms; the WebSocket fallback's three checks from the model's bound and one practice run with a tolerance of 10 % fixed before it; "The client's fast reconnect over WebSocket is not measured here (OQ-0001-19)" | Nothing: the mark is the proof test's; the reconnect's bar is that work's (§6.4). |
| 1571 (PM-0001-31, "the server never holds more than one unsent snapshot per client"; "L2's TCP proxy on RS") | E8: "unsent" means not yet written to the socket; the bite includes the snapshots the send budget skips; the stream wrapper named | Nothing. |
| 1697-1699 (Mustafa's check item 11: "within half a second after it, the world catches up") | E13: half a second on WebTransport; on the WebSocket fallback fix 1's expectation (half within about two thirds of a second, about one burst in eleven over a second, one in forty over two: Table 1's fix-1 row) and the reconnect required before the first online playtest | Nothing: the checklist is the proof test's (its step 1 opens "the proof test's link", 0001:1673). |
| 1737-1738 (migration step 5, slice 3: "*Done when:* ... and the byte-level audit (R2-INT-REC5) pass, and Mustafa's first online playtest is run") | E7: the WebSocket fallback's fast reconnect built and measured against OQ-0001-19's target before the first online playtest, a miss going to Mustafa before that playtest; links this record | Meets it, or brings the miss to Mustafa (§3.7). |
| `docs/adr/README.md` on main: row 0001 ("18 numbered (OQ-0001-1 to -18), 12 open"); "Total: 118 numbered open questions, 97 of them open"; the decisions table after the ROOM-CRASH (OQ-0001-18) row | E9, E10: 19 numbered and 13 open; 119 and 98; a row "WS-RECONNECT and RESUME-CRED (OQ-0001-19)" linking part 11, OQ-0001-19 and this record | Closes the row when OQ-0001-19 is answered. |

### 5.2 Changed later by the reconnect work (deferred with it: Mustafa, §1.1)

| Line (words to find it) | What changes |
|---|---|
| 423 ("WebTransport is tried first with a timeout (proposed 1.5 s), then WebSocket") | Add: a WebSocket session reconnects over WebSocket at once; the timeout is for the first connect (§3.5). |
| 429-432 ("one TCP port for the WebSocket fallback") | Stands: one endpoint (§3.2). |
| 445-451 ("`Join` within 2 s"; "a cap on handshakes in progress"; "a modest cap on connections per address (above 1)"; "new sessions rate-limited") | Sized for reconnects: 2 connections per player in an outage; a room-wide outage's handshakes and `Join`s from one address; caps per player or scaled with the room; the 2 s `Join` deadline per attempt kept and modelled, or lengthened (§3.9). |
| 494-497 ("expiry 30 s after issue with at most 5 s of leeway, a unique id") | The spare's life and refresh; a session claim under reading 2 (§4). |
| 503-504 ("a bounded cache that refuses new tokens when full") | Sized for one entry per takeover (§4.2). |
| 506-509 (the rule: "the client fetches a fresh token; the newest connection for a player id wins and the older one is closed") | Rewritten to §3: the trigger, one attempt at a time, the cap per outage, the chosen attempt policy, join-or-close, a credential held in advance, the graceful close, commands read from the old connection until the new one's first. The docs PR's requirement sentence (§5.1) then points to the rule instead of the record. |
| 529-530 ("offline token checks keep the lobby off the join path") | Add: a periodic spare refresh per player during a match is new lobby load (§4.2). |
| 616-617 ("the Worker owns the connection") | Wording: the reconnect logic and the spares live in the Worker. |
| 668 (`Snapshot` header: "newest command received, last processed command") | Name what the reconnect reads: "newest command received" for the trigger's RTT, the server tick for cancel or join-or-close (§3.4, §3.6). Contents unchanged. |
| 670 (`Join`: "once, within 2 s") | Once per connection. |
| 671 (`JoinAccepted`) | A session id, under reading 2 (§4.1). |
| 672 (`FullState`: "at join and on resync") | Also at a takeover, or a delta against the last acknowledged snapshot (§3.9). |
| 682 (Lobby calls: "token request") | The spare refresh, with per-client and per-address rates. |
| 747 (failure row "A burst of total loss (B1)", as the docs PR leaves it) | The reconnect's expectation and bar (§6.4). |
| 758 ("A player drops"; "fresh token; newest connection wins") | A spare held in advance; the reconnect rule. |
| 759 ("The lobby is down"; "no new joins; running rooms go on") | Reconnects work until the spares expire (§4.2). |
| New failure rows | False reconnects (§3.4), takeovers after a burst (§3.8), browser throttles after repeated outages (§3.3). |
| 782-784 ("the guest secret out of script's reach") | Spares in the Worker's memory, short-lived: compatible; said so (§4.2). |
| 959-964 (Q22: "A. Hold the slot, rejoin with a fresh token (chosen)"; "B. A room-issued resume secret") | A's words gain "fetched ahead and held as a spare"; reading 3 would reopen B (§4.1). |
| OQ-0001-19 (new from the docs PR) | Answered and closed, with the register's counts (§5.1). |
| 1640-1641 ("Part 11's lobby, join tokens, reconnect ... are slice 2 and slice 3 milestones") | Stands; points to this record. |
| 1737-1738 (slice 3's *Done when*, as the docs PR leaves it) | Met, or the miss decided by Mustafa before the first online playtest. |
| 1756-1758 (Test plan: "a delaying TCP proxy for WebSocket") | Gains loss below TCP, or modelled handshakes, for reconnect measurements (§6.2). |

## 6. How to measure it faithfully

### 6.1 Why the proof test's setup cannot
- The proof test's WebSocket delay is a stream wrapper in the server's accept path (ADR 0001:1462-1463), above TLS and
  below the WebSocket library (Mustafa's U3-Q4 (b), 2026-11-06: `.checkpoints/netcode-proof-test.md:391-392` his
  agreement, `:400-401` the reading; the docs PR writes the placement into part 13's key, §5.1). So a reconnect's SYN
  and TLS flights cross the real, unimpaired path even during the burst; only the upgrade and what follows meet the
  model. A reconnect there is about 2 round trips (about 200 ms at P2) faster than a real one
  (`u3-verify2-harness.md:131-141`). Sure: high on the placement, medium on the size.
- The load generator's clients are Rust clients that no browser rule binds (`u3-verify2-harness.md:142-147`), and they
  run Linux (1 s linear SYN retries, timestamps on): the client best placed for never abandoned at a 1 s burst
  (`u3-verify3-harness.md:187-191`).

### 6.2 What a faithful setup needs
1. **Loss below TCP** for the WebSocket run, so the burst reaches a reconnect's SYN and TLS flights: for example a
   firewall drop on the server for the clients' addresses during the burst, with the real kernel's TCP at both ends;
   or the delay wrapper modelling each new connection's handshake under the burst (`TcpModel::connect`)
   (`u3-verify3-harness.md:268-272`).
2. **A browser-faithful client.** Each simulated client runs the page's policy and the browser's rules as data:
   Chromium's endpoint lock with its 10 ms, its throttler with Windows' argument order and the 2-minute roll; or
   Firefox's admission queue, fail delay and open timeout (`u3-verify3-harness.md:273-275`). The harness already has
   them (`fixes.rs:194-372`). Plus real browsers for a smaller sample, one per engine behind VISION_V2 §4's four
   browsers (`docs/VISION_V2.md:55`): Chromium's (Chrome, Edge, and Brave, Mustafa's browser) and Firefox's, on
   Windows, and Safari's WebKit, whose WebSocket rules are not researched yet (§8).
3. **Windows-like clients:** timestamps off and SYN retries doubling from 1 s (`tcp_syn_linear_timeouts` 0 and
   `tcp_timestamps` 0 in their network namespace), and a sockops BPF program answering `BPF_SOCK_OPS_TIMEOUT_INIT`
   for other first timeouts such as 3 s (`u3-verify3-harness.md:191-195`, `:276-278`).
4. **Per-address limits** raised for the load generator's address, to cover N clients × (2 connections + attempts), N
   handshakes in progress and about N `Join`s in a second after each burst (`u3-verify3-harness.md:279-280`); the
   real limits tested separately at their real values.
5. **Outage lengths** 0.6, 1, 1.5 and 2.5 s, not only 1 s (`u3-verify3-harness.md:281`), at P2, plus P3 and P4.
6. **Bursts spread against the tick:** with 20 s spacing every burst of a run meets the 30 Hz tick at the same phase;
   move each burst's start by a random 0 to one tick period (0-33 ms) and record each burst's phase
   (`u3-verify3-stats.md:117-132`, `:241-243`), as the proof test's PM-0001-18 runs do from the docs PR on.
7. **Repeated outages per client** (10 and 30), so browser throttles and the per-outage cap carry over
   (`fixes.rs:1375-1381`; `u3-verify3-harness.md:180-186`).
8. **The server's kernel and timer frequency recorded** against `linux-6.8-tcp v3` (`tcp_model.rs:79`;
   `u3-verify3-model.md:193-196`).

### 6.3 What to record per burst
The recovery time; attempts; takeovers before and after the recovery; attempts closed (cancel, join-or-close,
abort); throttle and fail delays; the silence before each attempt against the trigger; the bytes outstanding at the
return against fix 1's limit; the browser, the OS and whether TLS resumed; the burst's tick phase
(`u3-verify3-stats.md:176-181`; `fixes.rs:463-476`). A count of recoveries equal to the bursts expected, since a
client that never heals writes no record (`u3-verify3-stats.md:154-156`).

### 6.4 The bar (WS-RECONNECT-BAR (b): open until the reconnect work's plan; Mustafa, 2026-11-08)
- **The target** (Mustafa, §1.1): on a faithful setup at P2 with a 1 s burst, 99 % of client-bursts decode a fresh
  delta within about 1.05 s of the link's return. Slice 3's *Done when* asks for the reconnect to be built and measured
  against it before the first online playtest, a miss going to Mustafa (§5.1).
- **How the pass bar is set: open until the reconnect work's plan.** The docs PR that landed this record (#45) asked
  Mustafa (its "Decisions needed", WS-RECONNECT-BAR): (a) borrow WS-BAR's rule now, or (b) leave it open, so the
  reconnect work proposes its bar with its plan, for his go, against the target. The advisor recommended (b): the
  faithful setup, the policy and the sample size are not known yet, and nothing in the proof test depends on it. His
  answer, 2026-11-08: "WS-RECONNECT-BAR: (b), open until the reconnect work's plan." So the form below is a starting
  proposal for that plan, not a decision.
- **Proposed form, borrowed from WS-BAR** (§1.1): tail parts (at most k over a time, none over a maximum) plus
  mechanism checks; a median part only with its margin stated. Numbers from the model and one practice run on the
  faithful setup, never looser than the model predicts for a correct build; a practice run worse than the model is
  explained before any number is set. If the form is borrowed, Mustafa's 14:30 tolerance comes with it: a tolerance
  fixed before the practice run (10 %, or another the advisor sets with reasons), within which the numbers may move by
  the measured slowdown, beyond which the cause is found first and nothing is loosened.
- **What does not carry over unchanged.** The proof test measures "slower than the model" at the median, the only
  one-run statistic precise enough for fix 1 alone (PM-0001-18's row, from the docs PR on). The reconnect leaves the
  median where fix 1 put it (654 ms for both, `tables-1e020a7.txt:123`, `:129`) and changes only the tail, so a
  slowdown measured at the median says little about whether the reconnect works. Under the borrowed form the
  reconnect work must name the statistic that measures its slowdown (the share over a time near the target, say) and
  size its sample for it; the median then serves as the cross-check §2.3 describes.
- **What "99 % within 1.05 s" needs in samples.** For a build whose share over 1.05 s is the model's 1 %, the count
  over 1.05 s that a correct build stays within 95 % of the time is 9 of 480, 21 of 1,440, or 60 of 4,800 (exact
  binomial sums, this record's arithmetic; the verifier recomputed them, `verify-ws-docs.md:38`). So one run of 480
  that passes "at most 9 over 1.05 s" shows only that the share is below about 2 % (9 of 480 is 1.9 %), not 1 %;
  several runs or more clients are needed.
- **What a bar alone misses.** A trigger floor twice too slow, or a budget twice too large, sits inside the model's
  uncertainty (`u3-verify3-stats.md:95-107`, `:168-182`): catch them with unit and property tests of the trigger, the
  per-outage cap, cancel or join-or-close, and fix 1's budget formula.
- **The model's time scale.** A median part 7 % above the model's p50 turns a correct build's pass into a coin toss if
  the real recoveries run 7 % slower (`u3-verify3-stats.md:71-79`); that fragility is what Mustafa's 14:30 tolerance
  answers for the proof test (§1.1).
- **Tools.** The proof test's judge can already express a three-part bar (p50, a count with a threshold, a max) on a
  part selected by `params.transport` "ws" (`u3-verify3-stats.md:148-160`).

### 6.5 The harness and model to reuse
- **Model:** `spikes/netcode-proof/crates/net/src/tcp_model.rs` (`linux-6.8-tcp v3`) and its rule tests
  (`tcp_model/tests.rs`).
- **Harness:** `tcp_model/fixes.rs`: the options (`Opts`, `Reconnect`, `Browser`, `Tls`, `ClientOs`: `:63-162`), the
  browser rules (`Throttler`, `FailRecord`, `Gate`: `:194-372`), the event loop (`Sim`, from `:501`), the rows
  (`configs()`: `:1212-1257`; `recommended()`: `:1259-1263`), the tables (`ws_limit_table` `:1276-1327`,
  `side_effects_table` `:1329-1333`, `multi_burst_table` `:1375-1381`, `ws_limit_tail` `:1682-1683`, `ws_bar_fix1`
  `:2000-2011`), the pin (`:1833-1858`) and the statistics (`joint_dp` `:1552`, `wilson` `:1615`, `intervals`
  `:1626`).
- **Run:** from `spikes/netcode-proof/`, `cargo +1.99.1 test --release -p npt-net --lib fixes -- --ignored
  --nocapture --test-threads 1` (`results/u3-ws-fallback/README.md:12`; `fixes.rs:1276-1277`); four tables took 171 s
  (`tables-1e020a7.txt:154`). The fix-1 bound alone: `... --lib fixes::ws_bar_fix1 -- --ignored --nocapture`
  (`README.md:13`). The scaled abort and join-or-close: `git apply` the patch in a worktree at `1e020a7`
  (`README.md:14`).
- **If `spikes/` is gone by then:** `git show bf9d248:spikes/netcode-proof/crates/net/src/tcp_model/fixes.rs` (and the
  same for `tcp_model.rs` and `tcp_model/tests.rs`); the outputs and the patch at
  `975e72b:spikes/netcode-proof/results/u3-ws-fallback/`.
- **Add first:** the scaled abort and join-or-close as options (§3.7; the patch shows one way); the 0.6 / 1.5 / 2.5 s
  rows; the room's 2 s `Join` deadline per attempt (§3.9); the three-part bar in the lineage and tail tables still
  prints the second round's stale bar (`three_part()`, `fixes.rs:1528-1537`; `u3-verify3-stats.md:133-138`); pins for
  the bite rows (`u3-verify3-stats.md:139-147`); the Windows client end's own TCP rules (§3.10).

## 7. Sources

**The model and harness**, on branch `claude/netcode-proof-test-plan-hcky5g` at `bf9d248`, under
`spikes/netcode-proof/crates/net/src/`: `tcp_model.rs` (`linux-6.8-tcp v3`; last changed at `709e44a`, equal at
`bf9d248`), `tcp_model/tests.rs` and `tcp_model/fixes.rs` (last changed at `bf9d248`).

**The model's outputs**, committed on the same branch at `975e72b` under
`spikes/netcode-proof/results/u3-ws-fallback/` (`README.md` there says what each is and how to reproduce it):
- `tables-1e020a7.txt`: every table (60,000-phase rows, multi-burst lineages, side effects, the 600,000-phase tail),
  printed at `1e020a7`; the rows are unchanged at `bf9d248`.
- `bar-fix1-bf9d248.txt`: `ws_bar_fix1`'s bound for a correct fix-1 build, the broken builds and the model-error rows.
- `reconnect-policies-1e020a7.patch`: the third skeptic round's diagnostic tests (`diag_rows`, `diag_nocancel`,
  `diag_lengths`, `diag_profiles`, `diag_ratchet`, `diag_worst`), the source of Table 2's last row and Tables 3-4.
  Throwaway code, kept for reproduction, not built into the spike.

**The proof test's working notes**, deleted from the tree when its change-set ends; cited at `95a04a1`, where each
equals its last change, under `.checkpoints/netcode-proof-test/`:
- `u3-verify-fixes-adr.md` (first round, ADR 0001 conflicts, RFC 6455, WHATWG): `10b1a53`.
- `u3-verify2-harness.md` (second round, harness against the browsers' sources): `73f451d`.
- `u3-verify3-harness.md` (third round, harness and policy): `d9e0036`; `u3-verify3-stats.md` and
  `u3-verify3-model.md` (third round, statistics and model): `88856ea`.
- `u3-build-v3-harness.md` (the harness build; its numbers are on the superseded v2 model): `59baae1`.
- `verify-ws-docs.md` (the verifier's check of the docs PR's first words and this record's first draft, including
  Linux v6.8's `SIOCOUTQ` and `tcpi_rtt`): in the tree at `95a04a1`.
- The checkpoint `.checkpoints/netcode-proof-test.md` (Mustafa's words, his laptop's settings) and the plan
  `.checkpoints/netcode-proof-test/plan.md` (last changed at `709e44a`): `95a04a1`.

**Primary sources** (read by the reviewers on the dates given; how sure: high unless stated):
- RFC 6455 §4.1 and §7.2.3 (rfc-editor.org/rfc/rfc6455.html; 2026-11-06, re-read 2026-11-07).
- WHATWG WebSockets Standard (websockets.spec.whatwg.org; 2026-11-06).
- Chromium, main branch, fetched 2026-11-06 and 2026-11-07: `net/socket/websocket_endpoint_lock_manager.cc`,
  `transport_connect_sub_job.cc`, `services/network/websocket_throttler.cc`, `websocket_factory.cc`,
  `ssl_client_session_cache.h`; clang's argument order under the Microsoft ABI (`TargetCXXABI.h:188-190`,
  `CGCall.cpp:5155-5161`).
- Firefox, main branch: `netwerk/protocol/websocket/WebSocketChannel.cpp`, `dom/websocket/WebSocket.cpp`
  (2026-11-06 and 2026-11-07).
- Linux v6.8 TCP (`net/ipv4/tcp_*.c`, `include/net/tcp.h`), cited per rule in `tcp_model.rs`; v6.12 diffed;
  `net/ipv4/tcp.c:630` (`SIOCOUTQ`) and `:3842` (`tcpi_rtt`), read by the verifier (2026-11-07).
- rustls 0.23.46 on docs.rs (`ServerConfig` ticket and session defaults; features) (2026-11-07).
- Windows: Catchpoint, "TCP SYN retries" (docs.catchpoint.com/docs/catchpoint-nodes-tcp-syn-retries; 2026-11-07);
  Microsoft Tech Community, "TCP Templates for Windows Server 2019" (2018-10-03; read 2026-11-07); Mustafa's own
  `Get-NetTCPSetting` (2026-11-07). Sure: medium-high.
- The trace: `docs/VISION_V2.md:55`, `:58`, `:82`; ADR 0001's part 10 need (`0001-multiplayer-netcode.md:473`).

## 8. Open questions the reconnect work brings back

1. **Policy against target.** Never abandoned meets the target at a 1 s burst; the scaled abort with join-or-close is
   more robust at other lengths but about 75 ms over it at 1 s (§3.7). Which, once both have 600,000-burst rows?
   Mustafa's call if the robust policy misses "about 1.05 s" (OQ-0001-19).
2. **RESUME-CRED:** which reading of "bound to the session" (§4.1), and the spare's life against the 30 s expiry.
3. **At a takeover:** a full state or a delta against the last acknowledged snapshot (§3.9).
4. **The room's admission rules against reconnects:** part 11's "the newest connection for a player id wins" with
   the graceful close of the old one (§3.9; what a leaked credential allows, §4.1 reading 2); part 10's per-address
   cap, handshake cap and new-session rate sized for a room-wide outage (§3.9); and the 2 s `Join` deadline per
   attempt, kept and modelled or lengthened for every connection (§3.9).
5. **The bar** (§6.4): open until the reconnect work's plan, which proposes it for Mustafa's go (WS-RECONNECT-BAR (b),
   2026-11-08). That plan states the statistic that measures the reconnect's slowdown (the median barely moves), the
   sample size for "99 %", and the numbers after a practice run on the faithful setup.
6. **Not researched:** Safari's (WebKit's) WebSocket connect rules and throttles, though VISION_V2 §4 names Safari
   (`docs/VISION_V2.md:55`); the dual-stack case once the room publishes IPv6 (Chromium's 300 ms IPv4 fallback, a lock
   per address: §3.2); a backoff after real closures (§3.5).
