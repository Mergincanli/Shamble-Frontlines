# Third pass-mark fix round: test-engineer (from the session, 2026-11-01)

Author: test-engineer, 19:18–19:40 Amsterdam. Every BEFORE was copied by script from the docs at 57c43c2, whole
lines, and is unique there and in the working tree (which carries another author's uncommitted round in 0006,
outside these rows); the blocks were dry-applied in this order to copies of both with scripts/apply_fixes.py.
Only test-engineer's pass-mark sections are touched.

### NOTE F-2 (the held note under the pass-mark table)
Deleted rather than adopted: with item 1 the row has one count, glue included, and no separate reading, so the
note's question (gate the control API and report the glue, or batch the inputs) is answered by part 5 and the
row itself; engine-architect's replacement only told the design's history, which part 5 now states. The heading
becomes "Notes on two rows." The engine-architect block held in
engine-architect-0002-fixes-2-held-for-test-engineer.md should not be applied.
END

### NOTE PM-0002-3 (agreement)
engine-architect's reading is adopted word for word. Two bites now: the per-soldier accessor (the count grows
with N) and a no-op export called 5 times per tick (over 4), so each half of the mark has a defect that must fail
it.
END

### FIX F-2 (PM-0002-3)
FILE: docs/adr/0002-code-structure-build-toolchain.md
BEFORE:
| PM-0002-3 | The Worker's calls per tick do not grow with N (part 5; part 6; OQ-0002-3) | calls from the Worker's host into the instance per tick: the same at N = 12 and N = 48 on every tick, counted with wasm-bindgen's own allocator calls; of them, calls of the control API at most 4 per tick for the online client and for a local match in one instance (OQ-0002-3 (a)), at most 8 in two instances (b); the allocator calls reported beside (proposed reading: R1 does not say whether the glue's calls count; see the note under the table) | CI (counts); ML: Chrome | BX at N = 12 and N = 48, local mode and online at P2, 1,800 ticks each (one minute, proposed); the counting build logs every tick's counts per export | 1 run per N and mode (counts repeat). Bites: a per-soldier accessor (one call per soldier per tick) | R1-BND-REC1 mark 2 (r1, 2261–2263: "the same number of Wasm calls per tick at N = 12 and N = 48 (at most 4)"); 8 = 4 per instance × 2 instances (the draft's proposal, spike item 1) |
AFTER:
| PM-0002-3 | The Worker's calls per tick do not grow with N (part 5; part 6; OQ-0002-3) | the same number of Wasm calls per tick at N = 12 and N = 48 on every tick, with wasm-bindgen's glue calls included and no separate reading (the glue's allocator calls are exports of the instance, so the counting build counts them): at most 4 for the online client and for a local match in one instance (OQ-0002-3 (a)), at most 8 in two instances (b) | CI (counts); ML: Chrome | BX at N = 12 and N = 48, local mode and online at P2, 1,800 ticks each (one minute, proposed); the counting build logs every tick's calls per export, glue included | 1 run per N and mode (counts repeat). Bites: a per-soldier accessor (one call per soldier per tick: the count grows with N); a no-op export called 5 times per tick (over 4) | R1-BND-REC1 mark 2 as written (r1, 2261–2263: "the worker makes the same number of Wasm calls per tick at N = 12 and N = 48 (at most 4)"); part 5 (one input call per tick carries the command and the received datagrams; the step returns where the outgoing datagrams and the block lie); 8 = 4 per instance × 2 instances (the draft's proposal, spike item 1) |
END

### FIX F-2 (the note on PM-0002-3, deleted)
FILE: docs/adr/0002-code-structure-build-toolchain.md
BEFORE:
**Notes on three rows.**
- **PM-0002-3, "at most 4".** The draft's own per-tick list for the online client (the command in; the tick's
  received datagrams in one call; the step; the outgoing datagrams in one call; the block's pointer and length) is 5
  or 6 calls unless inputs are batched into one call, and wasm-bindgen's glue makes its own allocator call whenever
  bytes are passed in. The row gates control-API calls at 4 and reports the glue's calls; the drafter confirms the
  reading, or the design batches its inputs.
AFTER:
**Notes on two rows.**
END

### FIX F11 (PM-0002-29)
FILE: docs/adr/0002-code-structure-build-toolchain.md
BEFORE:
| PM-0002-29 | What the Wasm file weighs (part 4; with the sim core spike, decides R1-RR-OQ5; feeds R5-CI-OQ4) | reported, no pass number: the `sim-wasm` file's bytes raw and gzipped, with and without wasm-opt, at `opt-level` "s" and "z", with parry3d once the sim core spike's sim is in; the wasm-opt version recorded; gated: every served file under 25 MiB (26,214,400 bytes); the first value becomes the budget file's first limit | CI | the spike's release builds; sizes read from the files, gzip at level 9 (proposed) | 1 build per setting (sizes repeat) | R1-RR-OQ5 (r1, 2946–2950); R5-CI-OQ4 (r5, 1833–1834); R6-HOST-REC1's mark (r6, 642; decision map N37); R5-GATE-REC4 (r5, 1737–1738); 25 × 1,048,576 = 26,214,400 |
AFTER:
| PM-0002-29 | What the Wasm file weighs (part 4; with the sim core spike, decides R1-RR-OQ5; feeds R5-CI-OQ4) | reported, no pass number: the `sim-wasm` file's bytes raw and gzipped, with and without wasm-opt, at `opt-level` "s" and "z", with parry3d once the sim core spike's sim is in; the wasm-opt version recorded; gated: every served file under 25 MiB (26,214,400 bytes). This measurement and the content and ability spike's share of the `content` crate (ADR 0005, OQ-0005-CD2) are the two inputs from which the Advisor sets the budget file's first limit (part 4); until then G7 records the `sim-wasm` size and does not gate it | CI | the spike's release builds; sizes read from the files, gzip at level 9 (proposed) | 1 build per setting (sizes repeat) | R1-RR-OQ5 (r1, 2946–2950); R5-CI-OQ4 (r5, 1833–1834); R6-HOST-REC1's mark (r6, 642; decision map N37); R5-GATE-REC4 (r5, 1737–1738); the draft's part 4 and spike item 8; 25 × 1,048,576 = 26,214,400 |
END

### FIX F12 (PM-0002-31)
FILE: docs/adr/0002-code-structure-build-toolchain.md
BEFORE:
| PM-0002-31 | How content files reach the Worker in development (decides R1-DATA-OQ8, which the map's part g and ADR 0005's CD-7 give this spike) | recorded per browser: whether Vite's hot update reaches the module Worker; then, by that route or by R1-DATA-REC5's page relay, a saved file's bytes in the Worker within 1 s of the save in 20 of 20 saves (ADR 0005's CD-7 mark) | ML: Chrome, Edge, Firefox | `vite` dev server; a script saves a content file 20 times; the Worker stamps arrival on the shared clock against the file's modification time (proposed) | 20 saves per browser | R1-DATA-OQ8 (r1, 2811–2812); R1-DATA-REC5 (r1, 2545–2547); ADR 0005 CD-7; decision map part g (line 942) |
AFTER:
| PM-0002-31 | How content files reach the Worker in development (part 5, content files in development; decides R1-DATA-OQ8, which `docs/SPIKE_PLAN.md` item 3 and ADR 0005's CD-7 give this spike) | recorded per browser: whether Vite's hot update reaches the module Worker; then, by that route or by R1-DATA-REC5's page relay, a saved file's bytes in the Worker within 1 s of the save in 20 of 20 saves (ADR 0005's CD-7 mark) | ML: Chrome, Edge, Firefox | `vite` dev server; a script saves a content file 20 times; the Worker stamps arrival on the shared clock against the file's modification time (proposed) | 20 saves per browser | R1-DATA-OQ8 (r1, 2811–2812); R1-DATA-REC5 (r1, 2545–2547); ADR 0005 CD-7; the draft's part 5 ("whether Vite's hot update reaches the Worker directly is PM-0002-31's to measure"); `docs/SPIKE_PLAN.md`, item 3 (the boundary spike: "ADR 0005's content files reaching the Worker") |
END

### FIX F13 (PM-0006-15)
FILE: docs/adr/0006-infrastructure-operations.md
BEFORE:
| PM-0006-15 | The firewall lets both game transports in and nothing else (part 5) | from each of the four browsers, 20 of 20 WebTransport sessions over UDP and 20 of 20 WebSocket sessions over TCP (`wss`) reach a room on RS; with no rule for a port, a probe to a listening service on that port gets no answer (a TCP connect times out, never "refused"; a UDP probe gets no reply); the firewall's rule list holds the UDP range, the TCP range and part 9's metrics rule (PM-0006-16), and 0 other rules. The `wss` endpoint presents a certificate the four browsers accept with no warning, and which one is recorded: the draft names none for development, and the WebSocket constructor takes only a URL and sub-protocols (R2-ROOM-F12, r2 lines 1494–1500), so a self-signed certificate cannot be pinned for `wss` | RS; ML: Chrome, Edge, Firefox; MAC: Safari; a probe from ML | WebdriverIO opens the proof test's client page with the transport forced by a URL flag; an attempt succeeds when the session is ready and one datagram or message echoes back; the probe opens a TCP connection and sends a UDP datagram to one closed port of each kind, with a test listener on each; the rule list is read from Hetzner's console or API and pasted into the results | 20 attempts per browser and transport (proposed: R2-TRN-REC5's count); 1 probe pair. Bite: a rule added for the probed TCP port makes the probe connect (the negative test can fail) | R6-HOST-REC2's mark (r6 lines 669–671); part 5 ("and nothing else"); R6-HOST-F18 (r6 lines 169–174); R2-TRN-REC3 (r2 lines 2639–2659) |
AFTER:
| PM-0006-15 | The firewall lets both game transports in and nothing else (part 5) | from each of the four browsers, 20 of 20 WebTransport sessions over UDP and 20 of 20 WebSocket sessions over TCP (`wss`) reach a room on RS; with no rule for a port, a probe to a listening service on that port gets no answer (a TCP connect times out, never "refused"; a UDP probe gets no reply); the firewall's rule list holds the UDP range, the TCP range and part 9's metrics rule (PM-0006-16), and 0 other rules. The `wss` endpoint presents a certificate the four browsers accept with no warning, and which one is recorded: the draft names none for development, and the WebSocket constructor takes only a URL and sub-protocols (R2-ROOM-F12, r2 lines 1494–1500), so a self-signed certificate cannot be pinned for `wss` | RS; ML: Chrome, Edge, Firefox; MAC: Safari; a probe from ML | WebdriverIO opens the netcode proof test's client page with the transport forced by a URL flag; an attempt succeeds when the session is ready and one datagram or message echoes back; the probe opens a TCP connection and sends a UDP datagram to one closed port of each kind, with a test listener on each; the rule list is read from Hetzner's console or API and pasted into the results | 20 attempts per browser and transport (proposed: R2-TRN-REC5's count); 1 probe pair. Bite: a rule added for the probed TCP port makes the probe connect (the negative test can fail) | R6-HOST-REC2's mark (r6 lines 669–671); part 5 ("and nothing else"); R6-HOST-F18 (r6 lines 169–174); R2-TRN-REC3 (r2 lines 2639–2659) |
END

### FIX F13 (PM-0006-17)
FILE: docs/adr/0006-infrastructure-operations.md
BEFORE:
| PM-0006-17 | Egress per 24v24 match-hour matches R2's estimate (part 8) | bytes sent from RS's network interface in one room-hour within ±20 % of R2's E3 estimate at the counts that ran (R2's formula, r2 lines 1764–1766, evaluated in the results with the run's P, V, shots and projectile spawns, the arithmetic shown; proposed, in place of R6-COST-REC1's fixed 3.16 GB, which holds only at R2's counts: the proof test's load need not have them, since it stands in for the stress scene, whose 700 shots per second (ADR 0001's PM-0001-2) are almost ten times R2's 72, 1.5 per player per second × 48, r2 line 1759); at R2's own counts (P = 48, V = 15 vehicles, 0.44 projectile spawns per player per second, every entity in view) that is 3.156 GB, so 2.52 to 3.79 GB. Beside it, the room's own bytes-sent metric agrees with the interface counter within 5 % (proposed: R6-COST-REC2's tolerance, a check that the counter measures the room). With interest management on: the figure reported, only if the proof test's sim has the interest filter | RS with LG; one room alone | 48 bot clients over IPv4 (R2's case), all in view; three 20-minute matches back to back, joins and full states included (proposed: a room-hour as a real room spends it); the interface's transmitted-byte counter read at the start and end | 1 room-hour per setting; P2 on LG. Bite: snapshots sent twice per tick (about 2 × the reference) must fail | R6-COST-REC1's mark (r6 lines 716–717); the egress table (r6 lines 438–446: 146.1 kbps × 48 = 7.01 Mbit/s; × 3,600 s ÷ 8 = 3.156 GB); R2's estimate (r2 lines 1759–1762, 1775); A-1 (r6 line 515) |
AFTER:
| PM-0006-17 | Egress per 24v24 match-hour matches R2's estimate (part 8) | bytes sent from RS's network interface in one room-hour within ±20 % of R2's E3 estimate at the counts that ran (R2's formula, r2 lines 1764–1766, evaluated in the results with the run's P, V, shots and projectile spawns, the arithmetic shown; proposed, in place of R6-COST-REC1's fixed 3.16 GB, which holds only at R2's counts: the netcode proof test's load need not have them, since it stands in for the stress scene, whose 700 shots per second (ADR 0001's PM-0001-2) are almost ten times R2's 72, 1.5 per player per second × 48, r2 line 1759); at R2's own counts (P = 48, V = 15 vehicles, 0.44 projectile spawns per player per second, every entity in view) that is 3.156 GB, so 2.52 to 3.79 GB. Beside it, the room's own bytes-sent metric agrees with the interface counter within 5 % (proposed: R6-COST-REC2's tolerance, a check that the counter measures the room). With interest management on: the figure reported, only if the netcode proof test's sim has the interest filter | RS with LG; one room alone | 48 bot clients over IPv4 (R2's case), all in view; three 20-minute matches back to back, joins and full states included (proposed: a room-hour as a real room spends it); the interface's transmitted-byte counter read at the start and end | 1 room-hour per setting; P2 on LG. Bite: snapshots sent twice per tick (about 2 × the reference) must fail | R6-COST-REC1's mark (r6 lines 716–717); the egress table (r6 lines 438–446: 146.1 kbps × 48 = 7.01 Mbit/s; × 3,600 s ÷ 8 = 3.156 GB); R2's estimate (r2 lines 1759–1762, 1775); A-1 (r6 line 515) |
END
