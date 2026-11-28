(Written by the main session from the advisor's structured output, workflow wf_f3167435-5b4, 2026-11-07: the advisor
agent has no Write tool. Its notes text follows verbatim, then its four fields verbatim.)

# Advisor: the docs PR's words and rulings after WS-SCOPE, RESUME-CRED and WS-BAR (2026-11-07)
Branch claude/netcode-proof-test-plan-hcky5g, head 5a5aba9. I worked from main's .claude/agents/advisor.md (fetched origin/main 69ea88e; the file there equals my brief). I edited no tracked file and committed nothing.

## Plan
1. Read the checkpoint (decisions, facts, running work), advisor-u3q.md §(2), u3-verify3-stats.md and -harness.md, u3-build-u3b.md, ADR 0001 at every line it touches, marks.json -18, SERIES.md, plan §4.4/§4.7/§6.2/§7.2/§12.1, fixes.rs (adaptive_limit, ws_bar_fix1) and tcp_model.rs's header.
2. Reproduce ws_bar_fix1 and the recommended-row pin.
3. Write the docs PR's exact words, the spike-branch changes, the rulings, and what goes to Mustafa.

## Findings (one line each, in the order found)
- F1 ADR 0001 on this branch equals main (git diff origin/main is empty for the file), so the line numbers hold on both: 424, 506-509, 747, 1250, 1295, 1462-1469, 1556, 1571, 1737-1738.
- F2 The *Reconnect:* clause (ADR 0001:506-509) is in part 11 (part 11 starts at :475), not part 10 as checkpoint :226 says ("fix 3 conflicts with part 10 / Q22"). The per-address cap is part 10 (:446).
- F3 I reproduced ws_bar_fix1 at 5a5aba9. The TCP model files have not changed since bf9d248 (git diff --stat is empty). The output matched u3v3-barfix1.txt line for line: fix 1 654/2,164/3,945/max 11,384; today 1,121/2,686/max 12,068; budget x2 p50 776; at 480, "half within 0.67 s, at most 5 over 2.5 s, none over 6.9 s" 0.9681, today and budget x2 0.0000, x1.05 0.001, +20 ms 0.159. 67 s.
- F4 the_recommended_fixes_are_pinned_at_b1 passes at 5a5aba9 (I ran it). From 1e020a7 to bf9d248, tcp_model.rs changed only in comments and an overflow guard on the RTO shift. So the buffer-plus-reconnect row (u3v3-tables1.txt:129: 654/1,051/p99.9 1,859/max 4,196, with 6,768 of 600,000 over 1 s) still holds at bf9d248.
- F5 MAJOR: the bf9d248 bound assumes independent single bursts. With 10 bursts per client, 20 s apart, fix 1 alone has p50 663 ms against 655 for single bursts (u3v3-tables1.txt:5-6), and a run's 48 clients share each burst's tick phase (u3-verify3-stats.md F10: body dispersion 1.39). By my arithmetic (median standard error about 7 ms at n = 480; density about 0.0032 per ms near 654 ms), a correct build fails the 0.67 s median about 15-25 % of the time, not 1 %. The numbers must come from a bound computed for the run as it runs.
- F6 The model's snapshot is a fixed SNAP = 1,050 B (fixes.rs:34). The bound must use the scene's real WebSocket message sizes, taken from L1's B1 runs.
- F7 "A baseline that never falls back" probably cannot bite on WebSocket: TCP loses nothing, and deltas against acknowledged baselines stay decodable. The WebSocket bite should be a send budget twice the formula's (budget_scale 2).
- F8 Plan §4.7 (plan.md:515) puts "WebSocket's newest-only replace" inside its backend. So fix 1 belongs in npt-net's WsServer send path, with no new public symbol.
- F9 Open point: on WebSocket connections without DelayStream (P0, ML's real path), fix 1 needs the real TCP_INFO and SIOCOUTQ. That needs an OS call, and M14 rules out unsafe code and libc in npt-net. U3c's plan must settle it.
- F10 SERIES.md:134 still defines burst_heal as "first delta decoded", not the fresh delta.
- F11 marks.json -18's "L2 B1" part has no params.transport select, so WebTransport and WebSocket runs would mix.
- F12 ADR 0001:1571 (-31) still says "L2's TCP proxy on RS" (the PR Advisor's Low note on #41). -31's L2 leg runs in R3, so this PR should fix it.
- F13 docs/design/ does not exist on main. The docs PR creates it, and REPO_MAP needs the folder. PR #40's REPO_MAP edit will conflict once, at the spike branch's next merge of main.
- F14 Register (docs/adr/README.md): the row for 0001 says "18 numbered (OQ-0001-1 to -18), 12 open" (:16), and the total says "118 numbered, 97 open" (:23). Adding OQ-0001-19 makes it 19/13 and 119/98; the session recounts by script.
- F15 The WebSocket bar describes Linux's TCP at the client (LG, and the model). A Windows peer (no timestamps) has fix 1 alone at p99 2,035 and max 12,550 (checkpoint :458). The ADR row says so.
- F16 The kernel facts in the new part 10 text (SIOCOUTQ = write_seq - snd_una, unsent plus unacknowledged; tcpi_rtt = srtt_us >> 3) are my reading of v6.8's tcp_ioctl and tcp_get_info. No kernel source is in this container, so I could not cite a line. The verifier must check both, with file:line, before the PR is marked ready. tcp(7) words SIOCOUTQ loosely ("unsent data").
- F17 R3 time: about +0.4 h (10.15 -> 10.55 h, inside 11 billed). EUR 1.72 planned and 1.90 cap stay the same.

## Blocks
(A) the docs PR: in adr_edits (PR text E-PR, E0-E12). (B) the spike-branch changes: in plan_and_marks_changes. (C) the rulings (480, NAME-L2PROBE, practice run, the rest): in rulings. (D) what goes to Mustafa: in escalate_to_mustafa.
DONE

## Field: adr_edits (the docs PR's exact words; apply each by matching its old text, not its line number)

### E-PR: the docs PR itself. A new branch claude/npt-readings, restarted from origin/main in a worktree (.gate/npt-readings), as README-BRANCH allows (Mustafa, 2026-11-06). Label needs-mustafa. It must merge on Mustafa's go before R3's first counted PM-0001-18 run, and before R3's run plan goes to him.

**Old text:**

```

```

**New text:**

```
TITLE: ADR 0001: the WebSocket send budget, PM-0001-18's split, and the fast reconnect as a real-client requirement (Mustafa, 2026-11-07)

BODY:
## What this is
This writes Mustafa's WebSocket decisions of 2026-11-07 (WS-SCOPE, RESUME-CRED, WS-BAR) into ADR 0001. They then hold before the netcode proof test's run day R3, and the reconnect requirement can't get lost. Docs only: no code, symbols +0 -0.

## His words (2026-11-07, about 13:45 UTC)
> WS-SCOPE: no. Measure fix 1 (the newest-snapshot-only buffer) in the proof test. The reconnect becomes a requirement for the real client, built and measured before the first online playtest, with today's model as its target (buffer + reconnect: 99 % within about 1.05 s). Record that in the docs PR so it can't get lost. RESUME-CRED: deferred with the reconnect. WS-BAR: the three-part form now; its numbers after one practice run on the real test setup, but never looser than the model predicts for a correct build with fix 1. If the practice run comes out worse than the model, find out why before setting the numbers.

## What changes (docs/adr/0001-multiplayer-netcode.md unless named)
1. Part 10 (the newest-only sentence): the server writes a tick's snapshot only while the client's unacknowledged bytes leave room within N snapshots. N = 1.5 x the smoothed round trip / the tick period, rounded up, plus 1, at least 3. The bytes come from SIOCOUTQ and the round trip from TCP_INFO.
2. PM-0001-18: on WebTransport, every client still heals within 500 ms. On WebSocket there are three checks: the median, at most k over Y, and none over Z. Their numbers are set after one unjudged practice run on L2, never looser than the model's bound for a correct build. The first bound, at 480 client-bursts: half within 0.67 s, at most 5 over 2.5 s, none over 6.9 s.
3. The failure-modes row for B1, and item 2 of "What a spike must prove": the same split.
4. Part 13's key: the WebSocket delay wrapper is now the Linux-faithful TCP model (U3-Q3 (b), 2026-11-06), no longer a fixed timeout.
5. Part 11's reconnect, a new open question OQ-0001-19, and migration step 5 (slice 3): the fast reconnect over WebSocket is required, built and measured before the first online playtest. Its target is 99 % within about 1.05 s. RESUME-CRED and the reconnect's other changes wait for that work.
6. PM-0001-31: "L2's TCP proxy" becomes the stream wrapper (the PR Advisor's Low note on #41).
7. docs/adr/README.md: OQ-0001-19's row, and the open counts go to 119 numbered, 98 open.
8. docs/design/ws-reconnect.md (new, by netcode-architect): the reconnect requirement in full.
9. docs/REPO_MAP.md and docs/STATUS.md (docs-keeper).

## Evidence
- The proof test's TCP model linux-6.8-tcp v3 was checked against Linux 6.8's source by three skeptic rounds. Over 600,000 B1 phases at P2 per row (p50 / p99 / max, ms): today's 64 KB buffer 1,121 / 2,686 / 12,068; the send budget alone 654 / 2,164 / 11,384; budget plus reconnect 654 / 1,051 / 4,196 (p99.9 1,859). ws_bar_fix1 at commit bf9d248 gives the bound: a correct build passes 0.968; today's buffer and a budget twice too large pass 0.000. The advisor reproduced it at 5a5aba9 (model files unchanged since bf9d248) with identical output, and ran the recommended row's pin, which passed.
- [the word-for-word verify script's result; gate checks 1-4 and 5b-5d PASS, symbols +0 -0; node .claude/scripts/repo-map-check.mjs OK; the verifier's spot-check table, including the two kernel facts (SIOCOUTQ, tcpi_rtt) with v6.8 file:line]

## Not verified, and known risks
- The first bound assumes independent single bursts. The real run has 48 clients sharing each burst and 10 bursts per client; the model's lineage rows put the median 8 ms slower (663 vs 655 ms). So the numbers are recomputed for the run as it runs before R3. That is the proof test's plan change, not this PR's.
- The median check is tight. By the model, a correct build only 5 % slower fails it almost always. The practice run exists to catch that, and an investigation is likely.
- By design, a correct build fails the WebSocket part about 3 % of the time (a 1 % / 2 % / 2 % allowance). Such a fail stands, with its investigation.
- The reconnect target is a model figure for a Linux client. Real browsers on Windows are measured by the reconnect work, before the first online playtest.

## Decisions needed
None. Mustafa's decisions are recorded word for word. The readings marked "the advisor's reading, 2026-11-07" are the advisor's: the 1 %/2 %/2 % definition of the bound, a better practice run keeping the bound, the burst offsets, and the WebSocket bite.

## For Mustafa
1. Read the top of this PR and the new PM-0001-18 row. If they say what you decided, say "merge #<n>" in a session's chat. It must merge before R3's first counted PM-0001-18 run.

[end with the session's own PR attribution lines]
```

**Why:** Mustafa asked for the decision to be recorded in the docs PR. ADR edits are escalation category 3, so the PR is needs-mustafa. It gets a fresh branch from main because PR #41 merged. Reviewers, once, on the final diff: netcode-architect (parts 10, 11, 13 and the record), test-engineer (PM-0001-18's checks), security-auditor (OQ-0001-19's credential wording), and the verifier (numbers and kernel facts against their sources).

### E0: ADR 0001 part 10, line 424 (the sentence inside the line). Apply by matching the text, not the line number. Wrap the inserted words as new lines (4-space indent, at most 120 columns) so the last new line ends with the existing "Inputs, snapshots", and lines 425 onward stay untouched.

**Old text:**

```
Over WebSocket the server sends only the newest snapshot and drops any unsent older one.
```

**New text:**

```
Over WebSocket the server sends only the newest snapshot and drops any unsent older one: it writes a tick's snapshot, all its messages, only while the bytes the client has not yet acknowledged (unsent and in flight: the socket's SIOCOUTQ, plus any still held above the socket) leave room for it within N snapshots of its size, N being 1.5 × the smoothed round trip (the socket's TCP_INFO) ÷ the tick period, rounded up, plus 1, and at least 3; otherwise that tick's snapshot is skipped and counted, and the next tick tries with its own (Mustafa, 2026-11-07: "Measure fix 1 (the newest-snapshot-only buffer) in the proof test"). In the netcode proof test the stream wrapper's model of TCP (part 13's key) gives both readings; by that model the first fresh snapshot after B1 arrives at a median of 1.12 s with a 64 KB send buffer of stale snapshots and 0.65 s with this budget (netcode proof test commit bf9d248).
```

**Why:** Fix 1 is Mustafa's decision. The old sentence says "unsent" only, which a 64 KB kernel buffer of stale snapshots satisfies. The formula is fixes.rs:164-169 (adaptive_limit) and the write rule fixes.rs:774-778. The medians are from u3v3-barfix1.txt (today p50 1,121; fix 1 p50 654), which I reproduced. The kernel facts (SIOCOUTQ, TCP_INFO) must be verified with v6.8 file:line before the PR is ready (finding F16).

### E1: ADR 0001 line 1556. Replace the PM-0001-18 row whole. This supersedes my earlier E1 ({L} approach, advisor-u3q.md:194-195); the line has not changed on main.

**Old text:**

```
| PM-0001-18 | A loss burst heals (part 6) | after B1's burst, every client decodes deltas again within 500 ms of the link coming back (a plain limit per client and burst) | L1; L2 with LG | B1 with 48 clients, 10 bursts per run (proposed) | L1: 3 seeds; L2: 1 run; every client after every burst. Bites: a baseline that never falls back (the advisor's reading, 2026-11-06) | r2 R2-SYNC-REC1 mark 2 (line 2020); the draft, "Failure modes", row "A burst of total loss (B1)" |
```

**New text:**

```
| PM-0001-18 | A loss burst heals (part 6) | per client and burst after B1's burst: the time from the link coming back until the client decodes a delta encoded at or after that moment, so deltas queued during the burst do not count (the advisor's reading, 2026-11-06). On WebTransport, every client within 500 ms (a plain limit). On the WebSocket fallback, with part 10's send budget, three checks over the run's client-bursts: the median at most X, at most k of them over Y, none over Z (Mustafa, 2026-11-07: "WS-BAR: the three-part form now; its numbers after one practice run on the real test setup, but never looser than the model predicts for a correct build with fix 1"). The numbers are set after that practice run, never looser than the bound the proof test's model of Linux's TCP (part 13's key) gives for a correct build with the send budget, computed for the run as it runs (its snapshot sizes, 48 clients sharing each burst, each client's bursts in turn): the values such a build stays within in 99 % of runs for the median, 98 % for the count over Y and 98 % for the maximum (the advisor's reading, 2026-11-07). First bound, for 480 independent single bursts with 1,050 B snapshots: half within 0.67 s, at most 5 over 2.5 s, none over 6.9 s (model `linux-6.8-tcp v3`, netcode proof test commit bf9d248). A practice run worse than the bound is explained before any number is set (Mustafa, 2026-11-07: "If the practice run comes out worse than the model, find out why before setting the numbers"), and so is one far better than the model; otherwise the bound becomes the numbers, never tightened from one run (the advisor's reading, 2026-11-07). Reported on WebSocket, not judged: p99, p99.9, the unacknowledged bytes at each return, and the time until a decoded snapshot's extra delay is back under PM-0001-31's 250 ms. The client's fast reconnect over WebSocket is not measured here (OQ-0001-19) | L1 (WebTransport's shape); L2 with LG on each transport | B1 with 48 clients, 10 bursts per run (proposed), each burst's start moved by a random offset of 0 to one tick period against the room's tick (the advisor's reading, 2026-11-07); the WebSocket run through the accept path's delay wrapper (part 13's key), whose clients run Linux's TCP as the model's do | L1: 3 seeds; L2: 1 run per transport, the WebSocket one after its practice run, which is not judged; every client after every burst (480 records a run). Bites: on WebTransport, a baseline that never falls back (the advisor's reading, 2026-11-06); on WebSocket, a send budget twice the formula's (the advisor's reading, 2026-11-07) | r2 R2-SYNC-REC1 mark 2 (line 2020), WebTransport's 500 ms; the WebSocket checks and their rule, Mustafa's decisions (2026-11-07); the draft, part 10 (the send budget) and "Failure modes", row "A burst of total loss (B1)" |
```

**Why:** Mustafa's split: WebTransport keeps 500 ms; WebSocket gets the three-part form with the practice-run rule. The 1 %/2 %/2 % allowance is ws_bar_fix1's (fixes.rs:2000-2007); I endorse it as the definition of "the model's bound". Y = 2.5 s. The first bound is quoted from u3v3-barfix1.txt, which I reproduced. The bound is recomputed for the run's structure because of findings F5 and F6. The no-fallback bite is kept off WebSocket (F7). The Linux-client caveat is F15.

### E2: ADR 0001 line 747, the failure-modes row. Replace it whole. This supersedes my earlier E2; the line has not changed.

**Old text:**

```
| A burst of total loss (B1) | the client holds; on return the server deltas from an older baseline or the defaults | deltas again within 500 ms (R2-SYNC-REC1 mark 2) |
```

**New text:**

```
| A burst of total loss (B1) | the client holds; on return the server deltas from an older baseline or the defaults; over WebSocket TCP resends what the burst lost, and part 10's send budget keeps stale snapshots from queueing behind it | deltas again within 500 ms on WebTransport (R2-SYNC-REC1 mark 2); on the WebSocket fallback PM-0001-18's three checks, set from a model of Linux's TCP (Mustafa, 2026-11-07); the real client's fast reconnect over WebSocket is required before the first online playtest (OQ-0001-19) |
```

**Why:** This row is the source cited by PM-0001-18 and must not contradict it. It also names the reconnect where the failure is described.

### E3: ADR 0001 line 1295, "What a spike must prove" item 2. Re-wrap only item 2's lines at 120 columns (3-space indent) and keep every other word.

**Old text:**

```
deltas again within 500 ms after B1,
```

**New text:**

```
deltas again within 500 ms after B1 on WebTransport and within PM-0001-18's three checks on the WebSocket fallback (Mustafa, 2026-11-07),
```

**Why:** This is the third and last place on main that states the heal limit (git grep "deltas again" finds 747, 1295 and 1556 only, outside docs/research/). This supersedes my earlier E3.

### E4: ADR 0001 part 13's key, R-25's sentence. It runs from the last word "For" on line 1462 through "(the advisor's reading, 2026-11-06)." on line 1469, and the text "The" that ends line 1469 stays. Re-wrap only these lines (2-space indent).

**Old text:**

```
For the proof test, the WebSocket delay is a stream wrapper inside the server's accept path, so the room sees real source addresses. Per direction: segments of at most 1,448 B; each delayed by the profile's one-way delay plus jitter, and delivered at max(the previous segment's delivery, its own time), so order holds; each lost at the profile's rate on its own, then delivered one retransmission timeout later: max(200 ms, 2 × the profile's round trip; U3 verifies the floor), holding back every later byte; the write side bounded at 64 KB, returning Pending beyond. Stalls are counted per run. The Join deadline runs from TCP accept. Bytes buffered per connection are capped and counted, and the message size limit applies before buffering (the advisor's reading, 2026-11-06).
```

**New text:**

```
For the proof test, the WebSocket delay is a stream wrapper inside the server's accept path, above TLS and below the WebSocket library, so the room sees real source addresses and the message size limit applies to whole frames before any delay buffering (the advisor's reading, 2026-11-06). Per direction it models Linux's TCP as Ubuntu 24.04's 6.8 kernel runs it with default settings (Mustafa, 2026-11-06: "model Linux TCP faithfully, so the result shows how the fallback really behaves"; the model `linux-6.8-tcp v3`, its rules read in the kernel's source and checked line by line in three reviews): segments of at most 1,448 B, each delayed by the profile's one-way delay plus jitter and lost at the profile's rate on its own, and acknowledgements, delayed and selective as Linux sends them, crossing the other direction with their own delay and loss. A lost segment is resent when Linux would resend it: by RACK (RFC 8985), once a later segment is acknowledged and a recent round trip plus a reordering window has passed since its send; by a tail-loss probe about two smoothed round trips after the last send; or at the retransmission timeout, the smoothed round trip plus its variance, the variance never below 200 ms (about 300 ms at P2), doubling at each consecutive expiry, after which the sender restarts in slow start from the segments still in flight plus one; F-RTO, the timestamp option, the loss undos and Cubic's window as Linux has them. Every resend draws its own fate, and bytes are delivered in order, so a lost segment holds back every later byte; the stream never reorders (the reorder step below is the datagrams'). The write side holds at most 64 KB unsent or unacknowledged, returning Pending beyond, and gives part 10's send budget its smoothed round trip and those bytes, as TCP_INFO and SIOCOUTQ give them to the real server. Stalls are counted per run, by repair path, and every WebSocket run's header names the model; U3 checks the model against the kernel on loopback and R1 against RS's own kernel, its version and CONFIG_HZ recorded. The Join deadline runs from TCP accept. Bytes buffered per connection are capped and counted.
```

**Why:** The key still describes the old constant model (max(200 ms, 2 x RTT)), which Mustafa's U3-Q3 (b) replaced. PM-0001-18's WebSocket bound and PM-0001-31 both rest on this text. The model's rules are quoted from tcp_model.rs:1-73 (S1-S10, the timeout and undo paths). This updates my earlier E4 (advisor-u3q.md:200-218) from model v1 to v3.

### E5: ADR 0001 part 11, the *Reconnect:* clause (lines 506-509). Insert one sentence after "...the record is ADR 0004's)." and before "*Private rooms:*". Re-wrap only the touched lines (4-space indent).

**Old text:**

```
OQ-0004-10 (GD-REJOIN: Mustafa's, with OQ-0005-AB10; the record is ADR 0004's). *Private rooms:*
```

**New text:**

```
OQ-0004-10 (GD-REJOIN: Mustafa's, with OQ-0005-AB10; the record is ADR 0004's). Over the WebSocket fallback the client also reconnects without waiting out TCP's backed-off retransmissions, a requirement of the real client, built and measured before the first online playtest with today's model as its target: 99 % of client-bursts decoding a fresh delta within about 1.05 s of a 1 s outage at P2 (Mustafa, 2026-11-07); its design, the credential an attempt presents (RESUME-CRED, "deferred with the reconnect": Mustafa, 2026-11-07) and what it changes in this part and in part 10's per-address cap wait for that work (OQ-0001-19; [the reconnect record](../design/ws-reconnect.md)). *Private rooms:*
```

**Why:** Mustafa: "Record that in the docs PR so it can't get lost." The rule a reconnect changes lives here ("the client fetches a fresh token; the newest connection for a player id wins"), so the requirement sits beside it. Note that this clause is in part 11, not part 10 (finding F2).

### E6: ADR 0001, Open questions. A new entry after OQ-0001-18 (it ends at line 1250 "...ends only its own match).") and before the blank line and "## What a spike must prove" (line 1252). Same bullet format and 2-space continuation indent as OQ-0001-17/18.

**Old text:**

```

```

**New text:**

```
- **OQ-0001-19. How does the client reconnect fast over the WebSocket fallback, and with which credential (WS-RECONNECT, RESUME-CRED)?** Mustafa (2026-11-07): "The reconnect becomes a requirement for the real client, built and measured before the first online playtest, with today's model as its target (buffer + reconnect: 99 % within about 1.05 s)." and "RESUME-CRED: deferred with the reconnect." After B1 over WebSocket, TCP's backed-off retransmissions leave 1 % of client-bursts more than 2.16 s from a fresh delta even with part 10's send budget; a fresh connection opened once the client has heard nothing for a short while, its session resumed on the newest `Join`, brings that to about 1.05 s by the proof test's model (a 1 s outage at P2 over 600,000 burst phases, a Linux client; the budget alone: p50 0.65 s, p99 2.16 s, max 11.4 s; the budget and the reconnect: p50 0.65 s, p99 1.05 s, p99.9 1.86 s, max 4.2 s; model `linux-6.8-tcp v3`, the netcode proof test's rows printed at commit 1e020a7 and unchanged at bf9d248). · *Why it matters:* how long a WebSocket player stays frozen after a short outage; the proof test measures the send budget alone (PM-0001-18). · *Options:* the trigger, the attempts, the takeover and when an attempt is abandoned; for the credential, a fresh lobby token per attempt (part 11 today) or spare single-use lobby tokens held in memory only, short-lived and bound to the session (Mustafa's earlier answer, 2026-11-06, now deferred), Q22 having weighed a room-issued resume secret (option B) and chosen fresh tokens (option A); each weighed in [the reconnect record](../design/ws-reconnect.md), with what the reconnect changes in part 11 (the newest connection wins) and part 10 (the per-address connection cap). · *Recommendation:* none yet (that work's). · *Decider:* Mustafa (the credential), netcode-architect with security-auditor (the design), the Advisor confirming. · *What decides it:* the reconnect built in the real client (migration step 5) and measured before the first online playtest against the target, with the loss applied below TCP so handshakes are lost too, on each browser engine and on Windows' TCP (its first SYN retry after 1 s). The trace is part 10's fallback: VISION_V2 §6.1; §4 (four browsers). · *Research ids:* R2-ROOM-REC4 (the rejoin it speeds up); Q22.
```

**Why:** An open question is the register's tracked home for a deferred decision, and it is counted in docs/adr/README.md, so it can't get lost. The numbers are u3v3-tables1.txt:125 (fix 1 alone) and :129 (recommended); I checked the pin at 5a5aba9. The measurement note follows round 2's H17 and round 3's V19 (u3-verify3-harness.md:187-195): L2 cannot measure a reconnect. The Windows SYN fact is from the checkpoint's "Facts from Mustafa".

### E7: ADR 0001 migration plan step 5 (slice 3), the *Done when* sentence, lines 1737-1738. Re-wrap the step's lines only (3-space indent).

**Old text:**

```
and the byte-level audit (R2-INT-REC5) pass, and Mustafa's first online playtest is run.
```

**New text:**

```
and the byte-level audit (R2-INT-REC5) pass, the WebSocket fallback's fast reconnect meets OQ-0001-19's target before the first online playtest (99 % of client-bursts with a fresh delta within about 1.05 s of a 1 s outage at P2; Mustafa, 2026-11-07), and Mustafa's first online playtest is run.
```

**Why:** "Before the first online playtest" is slice 3's gate (ADR 0001:1734-1738 already lists "reconnect" and "Mustafa's first online playtest"). Putting the target in Done-when makes the requirement block the playtest instead of being a note.

### E8: ADR 0001 line 1571, PM-0001-31's hardware cell only.

**Old text:**

```
| L2's TCP proxy on RS; LG; ML (Chrome) |
```

**New text:**

```
| L2's stream wrapper in RS's accept path (part 13's key); LG; ML (Chrome) |
```

**Why:** This is the PR Advisor's Low note on #41, planned for the readings PR before R5 (checkpoint :321-322). -31's L2 leg runs in R3, after this PR, and E4 rewrites the key it points to, so it belongs here. The row hash of marks.json -31 changes too.

### E9: docs/adr/README.md, row 0001 (line 16) and the Total sentence (line 23 onward). The session recounts by script before committing.

**Old text:**

```
18 numbered (OQ-0001-1 to -18), 12 open: || Total: 118 numbered open questions, 97 of them open || OQ-0001-9 and OQ-0006-16 in part, 2026-11-06);
```

**New text:**

```
19 numbered (OQ-0001-1 to -19; -19, WS-RECONNECT, added 2026-11-07), 13 open: || Total: 119 numbered open questions, 98 of them open || OQ-0001-9 and OQ-0006-16 in part, 2026-11-06; OQ-0001-19, WS-RECONNECT, is new from Mustafa, 2026-11-07);
```

**Why:** The register's counts must follow the drafts' entries (README :23-31). PR #41's C1 was exactly this kind of miss. The three edits are separated by "||".

### E10: docs/adr/README.md, the decisions table. A new row after the ROOM-CRASH (OQ-0001-18) row (line 87).

**Old text:**

```

```

**New text:**

```
| WS-RECONNECT and RESUME-CRED (OQ-0001-19) | How does the client reconnect fast over the WebSocket fallback, and which credential does an attempt present? A requirement of the real client, built and measured before the first online playtest (Mustafa, 2026-11-07: "The reconnect becomes a requirement for the real client, built and measured before the first online playtest, with today's model as its target (buffer + reconnect: 99 % within about 1.05 s)."); the credential waits for that work ("RESUME-CRED: deferred with the reconnect.") | Mustafa (the credential), netcode-architect with security-auditor (the design), the Advisor confirming | [ADR 0001](0001-multiplayer-netcode.md) part 11 and OQ-0001-19; [the reconnect record](../design/ws-reconnect.md) |
```

**Why:** The table lists "every decision code and open question the drafts route to Mustafa or the Advisor, one row each" (README :40). RESUME-CRED is his.

### E11: docs/design/ws-reconnect.md (new file in this docs PR), from netcode-architect's draft .checkpoints/netcode-proof-test/ws-reconnect-record-draft.md. Place approved: the same folder as the proof test's catalogue (docs/design/netcode-proof-messages.md on the spike branch), linked from part 11 (E5), OQ-0001-19 (E6) and README (E10).

**Old text:**

```

```

**New text:**

```
Required contents (the verifier checks each against its source):
(1) Mustafa's two sentences, word for word, with the date.
(2) The target, and the row it comes from: model linux-6.8-tcp v3; fixes.rs "recommended" row; 600,000 phases; P2; a 1 s B1 outage; p50 654, p99 1,051, p99.9 1,859, p99.99 2,092, max 4,196 ms; 6,768 over 1 s. Printed at 1e020a7 (u3v3-tables1.txt:129-130), unchanged at bf9d248 (the pin passes). Also the Windows-like client row (no timestamps, first SYN retry 1 s: p99 886 ms on 60,000 phases, :110) for information.
(3) The design as modelled, each item with its fixes.rs line: the trigger max(300 ms, 2 x the client's RTT + 100 ms); attempts every 300 ms in parallel, at most 8 per outage (H8); cancel when the old connection recovers (H6); the session resumed on the newest Join; Chromium's endpoint lock and throttler, Firefox's fail delay. The round-3 alternatives too: an RTT-scaled abort with join-or-close, better at 0.6, 1.5 and 2.5 s bursts (u3-verify3-harness.md V11, :232).
(4) The open choices: RESUME-CRED's options; abort policy; part 11's "newest connection wins"; part 10's per-address cap against parallel attempts; the 2 s Join deadline per attempt.
(5) How it must be measured: loss below TCP so SYN and TLS flights are lost; real browsers (each engine) and Windows' TCP. L2's DelayStream cannot measure it (H17, V19).
(6) The owner and the time: slice 3, before the first online playtest; its pass bar is set by that work, for Mustafa's go, and must reach the target.
(7) What the proof test measures instead: PM-0001-18's WebSocket part, the send budget alone.
(8) The trace: VISION_V2 §6.1 and §4.
(9) Branch and SHAs for every spike file it cites (the spike's code is not on main).
```

**Why:** Mustafa wants the requirement impossible to lose. The ADR carries the rule and the gate; the record carries the detail that slice 3 needs, so the design work does not have to rediscover the three skeptic rounds.

### E12: docs/REPO_MAP.md and docs/STATUS.md (docs-keeper, in this PR).

**Old text:**

```

```

**New text:**

```
REPO_MAP: a docs/design/ line ("design records: ws-reconnect.md, the WebSocket fallback's fast-reconnect requirement, ADR 0001 OQ-0001-19"), and node .claude/scripts/repo-map-check.mjs OK. STATUS: the netcode proof test's line gains Mustafa's 2026-11-07 decisions: the proof test measures the send budget alone; the reconnect is a real-client requirement before the first online playtest (OQ-0001-19, docs/design/ws-reconnect.md); RESUME-CRED deferred; WS-BAR in the three-part form with a practice run. Also the counts 119 numbered and 98 open.
```

**Why:** Docs-keeper keeps the map and status true in the same PR. The REPO_MAP edit will conflict once with PR #40's docs/design entry; resolve it at the spike branch's next merge of main (finding F13).

## Field: plan_and_marks_changes

- ORDER: (1) The docs PR opens from main and goes needs-mustafa. (2) Meanwhile, U3c and the fixes.rs bound (below) proceed on the spike branch. (3) After the docs PR merges on Mustafa's go: merge origin/main into the spike branch (as after #41, checkpoint :135) and re-take the row hashes for PM-0001-18 and -31 in marks.json in the same commit; check.mjs must pass all 9 steps. (4) R3's run plan goes to Mustafa only with the run-structured bound already in marks.json.
- marks.json PM-0001-18, parts: {"name":"L1 B1","select":{"layer":"L1","profile":"B1"},"min":3} (WebTransport's shape); {"name":"L2 B1 wt","select":{"layer":"L2","profile":"B1","machine.role":"LG","params.transport":"wt"},"min":1}; {"name":"L2 B1 ws","select":{"layer":"L2","profile":"B1","machine.role":"LG","params.transport":"ws"},"min":1,"validity":["ws-practice"]}. The transport values are "wt" and "ws", as PM-0001-25 uses them.
- marks.json PM-0001-18, checks: (a) "WebTransport: every client decodes a fresh delta within 500 ms of the link coming back": series event.burst_heal.after_link_ns, stat max <= 500000000, parts ["L1 B1","L2 B1 wt"]. (b) "every client after every burst: 48 x 10 heal records": count >= 480, every part (a client that never heals writes no record). (c) "WebSocket: half the client-bursts within X (provisional until the practice run)": stat p50 <= X, parts ["L2 B1 ws"]. (d) "WebSocket: at most k over Y": stat count, where {"op":">","value":2500000000}, <= k, parts ["L2 B1 ws"]. (e) "WebSocket: none over Z": stat max <= Z, parts ["L2 B1 ws"]. X, k and Z come from the run-structured bound (fixes.rs item below). Until it is computed, bf9d248's 670000000 / 5 / 6900000000 sit there as placeholders, named "provisional" in each check name and in notes. The judge already supports p50, count-with-where and max (u3-verify3-stats.md F13: marks.rs:419-478, eval.rs:260-285).
- marks.json PM-0001-18, report lines: p50 and p99 for every part. On the WebSocket part also p999, the count over 1 s, and the server's per-return series (unacknowledged bytes, the budget, srtt). Bites: keep "PM-0001-18/no-fallback" for the WebTransport parts only (L1 B1 and L2 B1 wt). Add "PM-0001-18/ws-budget-x2" ("a send budget twice the formula's", the budget_scale 2 knob), which must fail check (c) on the WebSocket part. Notes: rewrite to the new row (fresh delta, the practice run, the per-transport runs, the Linux-client caveat).
- New judge validity rule "ws-practice" (test-engineer), with PASS and INVALID fixtures: a run of "L2 B1 ws" is INVALID unless its results set holds a run with role "practice", the same build and profile, earlier in time, whose in-band check passed. The in-band check: (1) it passes checks (b)-(e) at the provisional values; (2) its median is at or above the model's 1st percentile of the run median (from the run-structured bound); (3) its mean WebSocket snapshot message size is within 5 % of the size the bound used. A practice run lists no mark for judging, so the judge never counts it for -18; it stays in the results ledger as "practice (WS-BAR)" and supersedes nothing. If the judge cannot carry it, the run script enforces it, with a test; the judge is preferred.
- SERIES.md: (1) Change event.burst_heal.after_link_ns (SERIES.md:134) to: "per client per burst: from the link's return to the decode of the first delta encoded at or after it (its server tick at or after the return's tick)". (2) Add event.burst_heal.burst (the burst's index, 0-9). (3) Add a server-side event.burst_return written by npt-server per burst: offset_ns (the random 0 to one-tick-period offset drawn from the run's seed) and tick_phase_ns (the return minus the room's last tick start); and per WebSocket client per burst: outstanding_bytes (DelayStream's unsent plus unacknowledged bytes at the return, plus any held above it), srtt_ns, budget_bytes and snap_bytes (the last written snapshot's size). (4) Add tick.v.ws_skipped_snaps_count (snapshots the send budget skipped this tick). (5) Header: params.ws_tcp_model ("linux-6.8-tcp v3", from tcp_model.rs:79 MODEL), machine.kernel (RS's uname -r) and machine.config_hz (RS's CONFIG_HZ), and role "practice". Test-engineer finalises the names under README §1's rule.
- Plan §6.2: split the "-17, -18, -20" row so -18 reads "C (L1, WebTransport's shape) + R3 (VS4: L2 with LG, one run per transport; on WebSocket the unjudged practice run first, then the counted run)". Plan §7.2 R3: -18 0.2 -> 0.5 (WebTransport run 0.2, WebSocket practice run 0.15, WebSocket counted run 0.15); bites 0.85 -> 0.95 (+0.1, ws-budget-x2); total 10.15 -> 10.55 h, billed 11, EUR 1.72 planned and 1.90 cap unchanged. Order rule: the practice run early in VS4 (after bring-up, the two L2 probes and the clean references); the counted WebSocket run and its bite late in VS5. The run script applies the in-band check mechanically between them. If the check fails, the counted WebSocket run and its bite are not run in R3 (0.25 h unused), the cause is investigated, and they move to a later run day's slack (R4 has 0.95 h) after the advisor reviews the cause.
- R3's run plan (ops/runs/R3.md): 10 bursts per run; each burst's start = the schedule's instant + an offset drawn uniformly over one tick period (33,333,333 ns) from the run's seed, recorded per burst; the spacing the bound was computed with (20 s); RS's kernel version and CONFIG_HZ recorded in the header against "linux-6.8-tcp v3" (if RS is not on 6.8 or CONFIG_HZ is not 1,000, the advisor rules before the WebSocket part counts; S14's kernel check at R1 stands); the room's per-address limits raised for LG's one address so 48 WebSocket clients connect.
- fixes.rs (before R3's run plan; in U3c or its own small unit): extend ws_bar_fix1 into the run-structured bound. It simulates whole runs, at least 2,000 (at least 960,000 client-bursts): 48 clients x 10 bursts as lineages at the run plan's spacing; all 48 share each burst's instant; each burst gets an offset uniform over one tick; each client's input phase hashed (splitmix64, as now); the WebSocket message size from L1's B1 runs of the same scene (payload + WebSocket frame header + TLS record overhead), replacing the fixed SNAP = 1,050 (fixes.rs:34). Output: X (the smallest 10 ms step with P(run median <= X) >= 0.99), k at Y = 2.5 s (P >= 0.98), Z (the smallest 100 ms step with P(none over Z) >= 0.98); the practice band's lower edge (the run median's 1st percentile) and the mean message size; pass chances of the correct build, budget x2 and today; sensitivity rows (x1.05, +20 ms). Also: pin one bite row (stats B5) and fix or remove the stale three_part() bar (stats B3, fixes.rs:1525-1534).
- U3c (DelayStream): (1) Expose pub(crate) srtt_ns() and outstanding_bytes() for each WebSocket connection's down direction. outstanding_bytes = the model's unsent + unacknowledged bytes; WsServer adds what it holds above the stream (tungstenite's pending bytes). (2) Apply fix 1 inside npt-net's WsServer send path (plan.md:515, "newest-only replace sits inside its backend"), not in npt-room: on each tick, write the whole tick's snapshot only if outstanding + its bytes <= N x its bytes; otherwise skip it, count it, and return SendResult::BufferFull; never write it later. (3) One pub(crate) fn for N (3 x srtt).div_ceil(2 x TICK) + 1, at least 3 (100 ms before the first sample, as fixes.rs:166), used by both WsServer and fixes.rs's adaptive_limit, so the model and the build cannot drift. (4) Write the per-return record (SERIES above). (5) No new public symbol; if one is needed, list it in §4.7 first (M6's rule). (6) Its notes cite v6.8's tcp_ioctl line (SIOCOUTQ = write_seq - snd_una) and tcp_get_info line (tcpi_rtt = srtt_us >> 3). (7) Open point for U3c's plan, to the advisor before it is built: WebSocket connections without DelayStream (P0, ML's real path) need the real TCP_INFO/SIOCOUTQ, which takes an OS call (M14 rules out unsafe code and libc in npt-net). Propose a vetted pinned crate or a small audited module (U5's audit); a pass-through DelayStream is not acceptable (it would model a path that isn't there).
- Mechanism tests (U3c, cargo test inside check.mjs, judged in the container): (a) Formula table at TICK = 33,333,333 ns: no sample -> 6 (the 100 ms default), srtt 20 ms -> 3 (the floor), 50 ms -> 4, 100 ms -> 6, 150 ms -> 8, 300 ms -> 15. (b) Properties (proptest or seeded sweeps): N never decreases as srtt grows and never falls below 3; a write happens if and only if outstanding + size <= N x size; after any write, outstanding <= N x size; writes happen only on a tick; a skipped snapshot is never written later; every written snapshot is the newest at its tick. (c) On DelayStream at P2 under B1: at every return, outstanding <= N(srtt then) x size. (d) Bites that must fail: budget_scale 2 fails (a) and (c); "outstanding = unsent only" (the SIOCOUTQNSD mistake) fails (c); a writer that queues a skipped snapshot fails (b). (e) The function fixes.rs uses is the same one (a compile-time reference, not a copy).
- Plan text: §4.3 E15's WebSocket sentence becomes the Linux-faithful model (S0-S14, v3) plus fix 1's readings. §4.4: WsServer applies the budget, and npt-room sees BufferFull and the unsent count. §12.1's U3 row: E15's constant becomes U3a's model, plus U3c's DelayStream with fix 1. §15: R-25's row (the faithful model) and a new row "WS-BAR's rule". A new §0.5 with Mustafa's 2026-11-07 words verbatim and where each is met. §16: the docs PR's go.
- NAME-L2PROBE (u3-build-u3b.md:108, SERIES.md:395-399): build it as approved (rulings) at U6. SERIES.md's "written by" for the probe rows already names it (SERIES.md:111-117), which meets U2's carried C5.

## Field: rulings

- DOCS PR: APPROVE the route and the words in adr_edits (E-PR, E0-E12). The PR is fresh from main on claude/npt-readings, needs-mustafa, and must merge on his go before R3's first counted -18 run. My earlier E1-E3 ({L} approach) are superseded. I checked the lines: 747, 1295 and 1556 are unchanged on main, and this branch's ADR 0001 equals main. My earlier E4 (part 13's key) is still needed, now for model v3, because the key still states the old constant. E8 (-31's "TCP proxy") is folded in, since -31's L2 leg runs in R3. Apply every edit by matching its text, not its line number, because the edits shift lines.
- FIX 1 in ADR 0001 part 10 (line 424): E0's words. The budget lives in the WebSocket backend (npt-net's WsServer), as plan §4.7 already places newest-only. It counts the client's unacknowledged bytes (SIOCOUTQ, plus bytes held above the socket) and reads srtt from TCP_INFO; in the proof test, DelayStream's model gives both. A skipped snapshot is dropped and counted, never sent later. Condition: the verifier confirms the two kernel facts with v6.8 file:line before the PR is marked ready. I could not; no kernel source is in this container.
- PM-0001-18 SPLIT: WebTransport keeps "every client within 500 ms" on L1 and on L2-wt. WebSocket gets three checks: the run median <= X, at most k over Y, none over Z. "The model's bound for a correct build" is defined as the values a correct fix-1 build stays within in 99 % (median), 98 % (count) and 98 % (max) of runs; that is ws_bar_fix1's 5 % false-fail split (fixes.rs:2000-2007), which I endorse. Y = 2.5 s: it sits above the backed-off-RTO cluster (2.0-2.5 s holds 2.1 % of phases), and the three Y rows behave the same under model error. The bound is computed FOR THE RUN AS IT RUNS: 48 clients sharing each burst instant, 10 bursts per client, random tick offsets, the scene's real message sizes. bf9d248's "half within 0.67 s, at most 5 of 480 over 2.5 s, none over 6.9 s" is cited as the first, single-burst bound only. Why: lineages put the median at 663 vs 655 ms (u3v3-tables1.txt:5-6), and by my estimate a correct build would fail the 0.67 s median about 15-25 % of the time, not 1 %.
- THE PRACTICE RUN: one unjudged run on L2 with the build to be judged. It is in band when it passes the provisional checks, its median is at or above the model's 1st percentile of the run median, its mean message size is within 5 % of the size the bound used, and every client has 480 records. WORSE (fails any provisional check): find out why before any number is set (Mustafa). FAR BETTER, or a size mismatch: also investigated (my reading); a recovery much faster than a faithful model points to the burst not applied, or stale deltas counted. BETTER BUT IN BAND: the bound becomes the numbers; it is never tightened from one run. Reasons: Mustafa's rule is a ceiling, not a target. One run's 480 client-bursts share 10 tick phases and are strongly alike (stats F10), so tightening from them overfits. A tighter bar raises a correct build's false-fail chance to an unknown value, and catches no break that the bound plus the mechanism tests do not already catch (budget x2 and today both fail at 0.000).
- INVESTIGATION RULES, for a practice run out of band: the cause is either the build (fix it, then run a new practice run), the setup (fix it, then a new practice run), or the model's inputs. Model inputs may change only through independently measured values (the run's message sizes, the burst schedule, the client clock error that PM-0001-47 measures), never values fitted to the practice run's recovery times. A change to the TCP model's rules (v3 -> v4) goes to a skeptic round, then to Mustafa with the new numbers before any is set (his standing rule of 2026-11-06: a changed number comes back to him).
- A COUNTED WEBSOCKET RUN THAT FAILS stands as FAIL with its investigation. There is no re-run without a found cause (the results-ledger rule from my U1 rulings). By the model a correct build fails about 3 % of the time; the PR says so.
- BURSTS PER RUN: 480 client-bursts (48 clients x 10 bursts), not 1,440. At 1,440 the bar is more fragile to model error (+20 ms: 0.039 vs 0.159 at 480; x1.05: 0.000 vs 0.001; stats F14: 1,440 gives way above a 1.06 time scale whatever the median). It catches no extra broken build: today and budget x2 are 0.000 at both sizes. It costs 3x the run time. It also adds 30-burst lineage effects the bar was not calibrated on. The count >= 480 check stays, and so does the ADR's "10 bursts per run (proposed)".
- BITES: "a baseline that never falls back" stays on the WebTransport parts only. Over WebSocket, TCP delivers every written snapshot, so deltas against acknowledged baselines stay decodable and the bite would likely be Missed. The WebSocket part's bite is "a send budget twice the formula's" (budget_scale 2); by the model it fails the median check (p50 776 ms). Budget x2 and "unsent only" are also caught exactly by U3c's mechanism tests, which do not depend on the model's time scale (stats F15's advice).
- BURST INSTANTS: each burst's start moves by a random offset uniform over one tick period, drawn from the run's seed and recorded, with per-burst records of the tick phase, the unacknowledged bytes and srtt at the return, and the over-bar counts per burst in the run summary. Why: 20 s spacing is 600 ticks, so without the offset every burst in a run meets the same tick phase (stats F10).
- RECONNECT REQUIREMENT, where it is recorded: (a) ADR 0001 part 11's *Reconnect:* clause gets one sentence (E5). (b) A new open question OQ-0001-19, WS-RECONNECT and RESUME-CRED (E6), counted in the register with a decisions-table row (E9, E10). (c) Migration step 5's Done-when, so it gates the first online playtest (E7). (d) The design record docs/design/ws-reconnect.md in the same PR (E11), with the required contents listed there. All four places, because one sentence alone can drift, and the gate in step 5 is what makes it "can't get lost". Target: buffer plus reconnect, 99 % within about 1.05 s after a 1 s outage at P2 (model linux-6.8-tcp v3, the recommended row printed at 1e020a7, unchanged at bf9d248; I ran its pin at 5a5aba9). RESUME-CRED and the reconnect's changes to parts 10 and 11 are deferred to that work.
- NAME-L2PROBE: APPROVE. npt-server's existing thin binary (plan §4.1) gets a probe mode (process "probe", matching marks.json PM-0001-3's L2 parts). Conditions: (1) its own process on a free slot port of RS with its own ImpairSocket, sharing nothing with a room process; (2) started in probe mode, the binary starts no room (a test); (3) its pacer is a dedicated thread with 1 ns slack, never tokio (M13); (4) the series are stamped on RS's clock from ImpairSocket's drain and release stamps; LG's npt-bots (probe) echoes and writes no probe series; (5) no new library symbol, unless it is listed in §4.7 first (M6). On U3b's open point 1 (the slot id as first-seen ordinal): fine for the proof test, since L2 has no fate-log twin; U6 adds a slot-id argument only if the room needs it.
- OPEN FOR U3c's PLAN (to the advisor, not Mustafa): fix 1 on WebSocket connections without DelayStream (P0, ML's real path) needs the real TCP_INFO and SIOCOUTQ. M14 bars unsafe code and libc in npt-net. U3c proposes how (a pinned, vetted crate or a small audited module, U5 audits it); a pass-through DelayStream is refused.
- WHAT I VERIFIED MYSELF: ws_bar_fix1 re-run at 5a5aba9 (67 s), output identical to u3v3-barfix1.txt; the recommended-row pin passes at 5a5aba9; ADR 0001 equals main; every old_text above matches the file. WHAT I DID NOT VERIFY: the kernel facts behind SIOCOUTQ and TCP_INFO (no source here); the netcode-architect's record (not written yet when I read the folder); the 15-25 % failure estimate for the single-burst median is my arithmetic, not a run.

## Field: escalate_to_mustafa

- MERGE GO (category 3, ADR edits): the docs PR on claude/npt-readings. It records fix 1 in part 10, PM-0001-18's split with the practice-run rule, the reconnect requirement (part 11, OQ-0001-19, migration step 5, docs/design/ws-reconnect.md) and -31's pointer. It must merge before R3's first counted PM-0001-18 run. His go is "merge #<n>" in a session's chat, after the advisor approves its head.
- FOR HIS INFORMATION, no decision: (1) The WebSocket median check is very tight. By the model, a correct build only 5 % slower than the model fails it almost always. The practice run is there to catch that, so expect an investigation. (2) The first bound (0.67 s / 5 over 2.5 s / 6.9 s) assumed independent single bursts. With 10 bursts per client the model's median is 8 ms slower, so the numbers are recomputed for the real run before R3. That is still "the model's prediction for a correct build", not a loosening. (3) By design, a correct build fails the WebSocket part about 3 % of the time; such a fail stands and is investigated, not re-run.
- CONDITIONAL, later: if the practice run's investigation finds the TCP model itself wrong (a v4), the corrected model's numbers come back to him before any number is set (his rule of 2026-11-06). Also, as with every run plan: R3's plan, now with the practice run, goes to him for his OK.
