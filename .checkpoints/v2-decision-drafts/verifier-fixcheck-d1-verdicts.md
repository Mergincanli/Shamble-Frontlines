# verifier fixcheck-d1: verdicts on the final fix round of ADR 0001, 0002 and 0006

Base 47eadd9 (before the final round) · HEAD 604a497 (branch claude/laughing-lovelace-ox61se) · research at 9e7db28
(r1–r6 unchanged at HEAD) · no web fetch (after fetch hours; web facts from verifier-fixcheck-d0-verdicts.md, "d0 #n").
Verdicts: confirmed · wrong (source quoted) · unsupported · citation off (right one given) · unverified (needs a source
d0 did not read). Line numbers are HEAD's.

## 0. Provenance of the changed text (done first)

- Rebuilt each doc from 47eadd9 by applying the fix files' FIX blocks in commit order (script
  verifier-fixcheck-d1-scratch/recon.py) and diffed with HEAD. ADR 0001 (netcode-architect final + b; test-engineer A
  final, b, c, d) and ADR 0002 (engine-architect 0002 final; test-engineer A final, c): identical to HEAD, 0 lines
  differ. ADR 0006 (infra-engineer final, b, c; test-engineer B final, b): HEAD = the rebuild minus test-engineer B's
  original OQ-0006-20 block, which was held (test-engineer-B-held-for-authors.md) and replaced by infra-engineer c's
  adapted block. So every changed line at HEAD comes from a FIX block in the named files; no unlogged session edit.
- Every BEFORE occurs once in its doc when its turn comes (0 failures).
- Glossary check (scripts/glossary_check_drafts.py, HEAD working tree = HEAD): exact hits 0 in all nine documents; the
  "worst case (48 players …)" pattern: no hit in the changed text (0006:1184 "a budget takes the worst case" is plain
  English, not the stress scene's retired phrase); "headroom" in changed text: once, 0002:1286 "N = 64 labelled
  headroom", the GLOSSARY's own meaning (a run above the largest mode's N).
- A sentence directly under a table (scratch undertable.py, base and HEAD): none in the three docs (the one hit,
  0001:567, is inside the components diagram's code block, as at base).

## 1. ADR 0001, netcode-architect's text (netcode-architect-fixes-final.md, -b)

### 1a. Claims

| # | doc:line | claim (short) | cited | verdict | where and the words | note |
|---|---|---|---|---|---|---|
| N1 | 0001:176–179 | own cues keyed by (entity, weapon, command sequence), quoted | R1-ABL-REC4, r1 2427–2428 | confirmed | r1:2427–2428 "Cues are events keyed by (entity, ability or weapon, command sequence), so a re-simulated or confirmed activation does not play twice." | |
| N2 | 0001:181–182 | replay depth 5 to 7 ticks at P2, (100+33.3+33.3)÷33.3 to (100+100+33.3)÷33.3 | arithmetic | confirmed | 166.6÷33.3 = 5.0; 233.3÷33.3 = 7.0 | uses P2's mean RTT 100 ms; at 80–120 ms the range is 4.4–7.6 (the text says "deeper at P3 and P4" and records the depth, so no change needed) |
| N3 | 0001:204 | U2 taken up by R2-BUD-REC1 "once measured" | r2 lines 2302–2303 | confirmed | r2:2302–2303 "inputs sampled every tick and sent every tick with the 3 previous inputs repeated (U1, U2 once measured)" | |
| N4 | 0001:205 | part 5's command "about twice R2's 8 B input" | the bandwidth estimate (15–20 B) | confirmed as arithmetic on the draft's own estimate | 15–20 B ÷ 8 B = 1.9–2.5 | rests on N12's 15–20 B |
| N5 | 0001:241–242 | a top-down cursor names points about 28 m from the look point, 44 m with the ADS pan | R2-INT-F18, r2 847–851 | confirmed | r2:849–851 "Its farthest corner is about 28 m from the look point … Style 5's ADS pan (foxPanMax 16 m) would push this to about 44 m." | |
| N6 | 0001:243 | legacy Beam Katana range 3.6 m, Molotov range 0 | LEGACY_TUNING.md:181 | confirmed | LEGACY_TUNING.md:181 "`range` \| 300 \| 34 \| 55 \| 0 \| 3.6 \| 95 \| 200" (Molotov 0, Beam Katana 3.6, by the header at :179) | |
| N7 | 0001:228 | std's clock panics on the browser's Wasm target | the Rust review, reading std's source | confirmed (d0 #2) | d0 #2: `unsupported.rs` "panic!(\"time not implemented on this platform\")" for `Instant::now` and `SystemTime::now`, picked for wasm32-unknown-unknown | |
| N8 | 0001:226–227 | timing at the tick adds up to one tick period (33.3 ms) of false jitter | arithmetic (NA-4) | confirmed | 1000 ÷ 30 = 33.3 ms | |
| N9 | 0001:655–658 | stress scene shots add (700 − 72) × 4 B × 8 ≈ 20.1 kbps; 72 a second = 1.5 × 48 | r2 line 1759; R2-SYNC-REC5 mark 4 | confirmed | r2:1759 "Shot events (cosmetic, sent once) \| 1.5 per player per second (30 % firing × 5 shots/s) × 4 B"; 628 × 32 = 20,096 bit/s | |
| N10 | 0001:662–664 | 8 B commands, newest + 3: about 22.3 kbps at 30 sends (U1's 44.6 at 60, halved) | r2 table C, 1704–1710 | confirmed | r2:1708 "U1 \| Send every 60 Hz tick: newest input + 3 previous, 8 bytes each \| 44.6 kbps"; 44.6 ÷ 2 = 22.3 | |
| N11 | 0001:667–668 | 80–186.7 ms at P2's 100 ± 20 ms, so 3 to 6 commands | arithmetic | confirmed | 80 + 0 = 80; 120 + 66.7 = 186.7; 80 ÷ 33.3 = 2.4 → 3; 186.7 ÷ 33.3 = 5.6 → 6 | |
| N12 | 0001:665 | part 5's command about 15–20 B in all | "the perf review's estimate, 2026-11-02" | unsupported (a review's estimate, its parts not shown) | review-perf-auditor.md:27 gives "about 15–20 B per command" with no sum of fields | rule 2: a number is research, shown arithmetic, settled or proposed; label it "proposed estimate" or show the sum (aim point 10 B + tick, sequence, move, buttons, counters, view time); the text already says the real size is measured (low) |
| N13 | 0001:668–672 | (3×15+63)×240 ≈ 25.9 to (6×20+63)×240 ≈ 43.9; at the cap (8×15+63)×240 ≈ 43.9 to (8×20+63)×240 ≈ 53.5 kbps; 48,000÷8÷30 = 200 B | arithmetic; 57 B at r2 line 1742 | confirmed | r2:1742 "Transport overhead per datagram \| 57 B IPv4, 77 B IPv6"; 108×240 = 25,920; 183×240 = 43,920; 223×240 = 53,520; 6 + 57 = 63 | the review's 57.4 kbps was its own slip; the draft's 53.5 is right |
| N14 | 0001:674–675 | QUIC acks on input packets about 1.9 kbps (3.8 at 60 Hz halved) | r2 line 1785 | confirmed | r2:1785 "QUIC acks riding on input packets, about 8 B each, add about 3.8 kbps at 60 Hz: assumption" | |
| N15 | 0001:677 | 48 × 25.9 to 48 × 43.9 kbps ≈ 1.2 to 2.1 Mbit/s in | arithmetic | confirmed | 1,243 and 2,107 kbps | |
| N16 | 0001:262–268 | E3b about 188.3 kbps at N = 48 (r2 line 1779); stress scene 300 × 5.5 B = 1,650 B per snapshot, above the 1,067 B cap (line 1755) | r2 1779, 1755 | confirmed | r2:1779 "48, E3b (projectiles in every snapshot) \| … \| 188.3 kbps"; r2:1755 "Projectile per-snapshot update (E3b) \| 5.5 B"; 256,000 ÷ 30 ÷ 8 = 1,066.7 B | |
| N17 | 0001:263 | PM-0004-73 holds the trajectory function bit-identical on every target | ADR 0004 | confirmed | 0004:1167 "the trajectory function gives bit-identical points on Wasm and native, 10,000 of 10,000" | |
| N18 | 0001:279–281 | the gameplay events of ADR 0004 part 19 (list) | ADR 0004 part 19 | confirmed | 0004:535–539 lists hits, deaths and kills, blasts, cosmetic shot events, purchases, uplink cache events, harvester deliveries and kills, structure kills and stage changes, tech tiers, upgrade picks and pickups | 0001 lists all but blasts and cosmetic shots, which the next sentences treat (as A8 asked) |
| N19 | 0001:287 | part 9 (f)'s sound radius | part 9 (f) | confirmed | 0001 part 9 (f): "Events have a radius per event type (data; sound larger than visuals)" | |
| N20 | 0001:288 | the shooter's hit confirmation is a gameplay event (LOOP_V2 §7) | LOOP_V2 §7 | confirmed | LOOP_V2 §7 "hit feedback arrives when the hit is confirmed" | |
| N21 | 0001:322–324 | spatial events into the first block whose delayed time reaches their tick; R2-SYNC-REC6 quote | r2 line 2210 | confirmed | r2:2210 "Blasts are server events with a tick, played on the timeline of the entity they hit." | the rule widens the quote's blast rule to every spatial event; the text says so ("its blast rule", changed) |
| N22 | 0001:316–318 | delay 3 intervals (100 ms), within 2 to 5 (66.7 to 166.7 ms) | R2-SYNC-REC4 | confirmed | r2:2152 "(REC4: 2 to 5 intervals)"; 2 × 33.3, 5 × 33.3 | |
| N23 | 0001:331–333 | a remote death 0.9–1.5 m from the drawn soldier at 9 m/s (100–166.7 ms × 9 m/s) | arithmetic; 9 m/s from legacy T (r2:458) | confirmed | 0.1 × 9 = 0.9; 0.1667 × 9 = 1.5 | |
| N24 | 0001:342–344 | the estimate is R2's "RTT plus that client's interpolation delay" | R2-SYNC-REC5, r2 2145–2146 | confirmed | r2:2145–2146 "(RTT plus that client's interpolation delay)" | the two added waits (buffer, half a tick) are marked "proposed" and the research id "changed" |
| N25 | 0001:348–349 | a client can delay its replies | r2 2150–2152 | confirmed | r2:2151 "because a client can delay its replies (security review H4)" | |
| N26 | 0001:355–360 | 48 × 30 = 1,440 entries/s; 1,728,000 in 20 min (LOOP_V2 §8); about 518 MB at 300 B (A-9, r6 568–569); about 27.6 MB at 16 B | arithmetic; LOOP_V2 §8; r6 568–569; ADR 0006 part 9 | confirmed | LOOP_V2:100 "the timer (about 20 min)"; r6:568 "about 300 B per line, about 200 lines per match"; 1,728,000 × 300 = 518.4 MB; × 16 = 27.6 MB; 0006:369 "16 B a raw entry (proposed for scale …)" | |
| N27 | 0001:362–366 | 200–250 ms by R2's formula; 233–383 ms with the buffer (33.3–100) and blend (0–33.3) | arithmetic | confirmed | 200 + 33.3 + 0 = 233.3; 250 + 100 + 33.3 = 383.3 | the trade-off (0001:968) and OQ-0001-5 (0001:1061) repeat it, the same |
| N28 | 0001:403–405 | one process per room gives one port per room | R2-ROOM-F18, r2 line 1562 | confirmed | r2:1562 "each session is reached at its own address and port" | |
| N29 | 0001:421–424 | IPv6 interface id is 64 bits, so one subnet gives 2^64 addresses (RFC 4291 §2.5.1, read by the security review, not in the research) | RFC 4291 §2.5.1 | confirmed (d0 #7) | d0 #7: RFC 4291 §2.5.1 p. 8 "Interface IDs are required to be 64 bits long" | d0's optional note: a home usually gets more than one /64 (RFC 6177 §2), so the prefix length may need to be shorter; the text keeps the length as data, which covers it (optional precision only) |
| N30 | 0001:448–449 | reconnect grace 60 s = 1,800 ticks at 30 Hz | arithmetic | confirmed | 60 × 30 | |
| N31 | 0001:455–457 | a connection panicking while holding a lock poisons it for the tick (std `Mutex`) | the Rust review | confirmed (d0 #8d) | d0 #8d: std 1.99.1 Mutex "Poisoning": "a mutex becomes poisoned if it recognizes that the thread holding it has panicked" | |
| N32 | 0001:460–462 | under `unwind` an uncaught panic "ends only its own thread" | R2-ROOM-F8, r2 1459–1462 | confirmed | r2:1459 "In Rust an uncaught panic ends only its own thread and is reported to `join`" | ADR 0004 part 20 (0004:559–560) says the same |
| N33 | 0001:487–492 | SameSite=Strict sends it only from the site that set it, a site being the registrable domain (MDN Set-Cookie and Glossary "Site") | MDN, via the security review | confirmed, with d0 #6's note | d0 #6: MDN Set-Cookie "Strict: Send the cookie only for requests originating from the same site that set the cookie"; Glossary "Site": "a site is determined by the registrable domain portion of the domain name", and for SameSite "the scheme is also considered" | optional precision (d0): "a site being the registrable domain and the scheme (https here)"; the conclusion holds |
| N34 | 0001:503–533 | offers three by default, size data (ADR 0005 AB-8); `Buy` "the purchase names a unit, never a weapon" (AB-8's table); tier gate AB-5; buy terminal LOOP_V2 §2; power-plant pricing LOOP_V2 §3; OQ-0005-AB9 (drop and pickup rules), -AB10 (bots' tracks), -AB15 (who sees picks) | ADR 0005; LOOP_V2 | confirmed | 0005:1004 "its size from data, 3"; 0005:1000 "the purchase names a unit, never a weapon"; 0005:931 AB-5 "Tech tiers gate abilities and purchases"; LOOP_V2:22 "in front of the buy terminal"; LOOP_V2:37 power-plant pricing; 0005:1288, :1305, :1365 the three OQ titles | |
| N35 | 0001:527–529 | a steal is holding E (LOOP_V2 §4 step 4); placing and picking up name no key (steps 2, 3) | LOOP_V2 §4 | confirmed | LOOP_V2 §4 step 4 "Enemies steal the uplink by holding E for a few seconds"; steps 2 and 3 name no key | from netcode-architect-fixes-final-b.md (GD-8's E keys) |
| N36 | 0001:535 | R2's estimate counts buys among the reliable gameplay events | r2 line 1760 | confirmed | r2:1760 "Reliable gameplay events (kills, captures, buys), resent until acked" | |
| N37 | 0001:540–542 | each direction half the latency and ± half the jitter (proposed; OQ-0001-16); round trip from r2 1987–1992 | r2 1987–1992 | confirmed (the profiles); the split is proposed and so labelled | r2:1987–1990 "the injected latency is round-trip time … P2 = 100 ms, ±20 ms, 2 %" | |
| N38 | 0001:799, 823 | E3b +42 kbps | arithmetic | confirmed | 188.3 − 146.1 = 42.2 | |
| N39 | 0001:813, 973–976 | +25 ms from input to screen against 60 Hz: +8.3 ms of input wait (16.7 − 8.3) and +16.7 ms of display (33.3 against 16.7); page draws between the two newest blocks (R1-TICK-REC5, r1 line 2099) | arithmetic; r1 2099 | confirmed | r1:2099 "The main thread draws every frame from the two newest ticks with `alpha` (F4)" | |
| N40 | 0001:830 | U2 about 36 kbps at 60 Hz with 8 B commands | r2 table C | confirmed | r2:1709 "U2 \| U1 with the 3 repeats delta-coded … \| 36.0 kbps" | |
| N41 | 0001:1031–1037 | at 60 Hz the parts' ceilings take 3.87 of 4.17 ms; 384 rows (8 × 48); 0.30 ms (4.17 − 3.87); 2.80 ms at 30 Hz | ADR 0004 part 6; GLOSSARY | confirmed | 0004:238–239 "2.80 ms (8.33 − 5.53, arithmetic). At 60 Hz the same parts sum to 3.87 of 4.17 ms"; GLOSSARY:79 "Left at 60 Hz: 0.30 ms in the sim step share" | |
| N42 | 0001:1045–1047 | "The 30 Hz baseline marks may use a stand-in (with S1's stand-ins at 30 Hz: 384 effect rows and a 1 ms burn, the 2.80 ms remainder topped up): on the planning check they leave 7.49 ms" | arithmetic on GLOSSARY §1 | wrong | GLOSSARY:68 "server tick, sum (limit 50 %) \| … \| 9.18 (16.67)" is 5.53 (sim parts, no remainder) + 3.64 (server-only); 16.67 − 9.18 = 7.49 holds only without the remainder. With the 2.80 ms remainder topped up, the sim share is 8.33 and the check is 8.33 + 3.64 = 11.97, leaving 4.70 ms (3.70 on a re-flood tick) | the parenthesis was added this round (at 47eadd9 the sentence had none, and 7.49 was right); Finding F1 |
| N43 | 0001:1053–1056 | OQ-0001-5 option (c) "not researched", no recommendation | rule 3 | confirmed (rule kept) | "(c) … (not researched: the research weighs 200 and 250 ms only) … but (c) is not researched, so choosing it is the deciders' call" | |
| N44 | 0001:1140–1143 | (20 − 16.7) ÷ 40 = 8.3 % late at P2 with ±20 ms on the downlink | arithmetic (the netcode review's) | confirmed | 3.3 ÷ 40 = 8.25 % (8.33 % unrounded) | |
| N45 | 0001:1155–1158 (test plan) | 2^32 ticks at 30 Hz is about 4.5 years; the 30 Hz soak's 54,000 ticks never reach 65,536 | arithmetic | confirmed | 4,294,967,296 ÷ 30 ÷ 86,400 ÷ 365 = 4.54; 30 × 60 × 30 = 54,000 | |
| N46 | 0001 Rollback | WT_DRAIN_SESSION can drain a session before a server shuts down | R2-TRN-F37, r2 3439–3440 | confirmed | r2:3439–3440 "WT_DRAIN_SESSION can drain a session before a server shuts down (R2-TRN-F37)" | the lines are R2's summary that cites F37, not F37 itself; acceptable |
| N47 | 0001 Appendix A, R2-SYNC-REC2 row | mark 3's forged set includes "beyond weapon range" (line 2062); "not older" (line 2044) | r2 2062, 2044 | confirmed | r2:2062 "an aim point off the map or beyond weapon range"; r2:2043–2044 "and not older than the last command run" | |
| N48 | 0001 Appendix A, R2-BUD-REC1 and -REC2 rows | Shabzon within 1 % of 13.9 shots/s (r2 2324–2325); precision "within 1 cm and angles within 0.01°" (2341); pitch 10 bits, 0.18° (1748); 16-bit step about 0.0055° (2263) | r2 | confirmed | r2:2324–2325, :2341, :1748 "10 bits (0.18° steps)", :2263 "about 0.0055°"; 360 ÷ 65,536 = 0.0055° | |

### 1b. Blocks (netcode-architect-fixes-final.md, 57 FIX; -b, 4 FIX)

| block | finding | resolved? | new problem |
|---|---|---|---|
| NA-12 (part 2) | snapshots 30 a second at both tick rates | yes | none |
| NA-3, NA-10, GD-3, GD-2, ADR 0004's prediction step, `client-endpoint` (part 4) | own cues keyed; replay depth; own-state contents; purchases never predicted | yes: R1-ABL-REC4 quoted at its lines; depth 5–7 at P2 recorded per profile; own-state list; "purchases" in never-predicted | none |
| GD-1, SA-8, NA-9, PA-3, NA-4, RR-2 (part 5) | clamp; strictly newer ticks; newest-received ack; U2; arrival stamps; no clock in the endpoint | yes, each | N12: the 15–20 B command size is a review's estimate, not shown (low) |
| FA-L1 = A8, PA-2, NA-3, NA-13, NA-14, NA-16 (part 6) | event list; blasts resent or once; E3b; no own-shot echo; resync; generation change; "the pilot" | yes: every ADR 0004 part 19 event named (N18); blasts resent (final trajectory change or a gameplay event) | none |
| EA-1, NA-1 (a), NA-4 (part 7) | events at the delayed time; "server time" defined; jitter from arrival stamps | yes | none |
| NA-1 (b, c), SA-5, SA-6 (part 8) | estimate + buffer + blend; P2 arithmetic; aim log volume; RTT inflatable; quinn RTT logged | yes (N24–N27); the estimate's half tick is "proposed" with a reason | none |
| NA-16 (part 9 (f)) | "parts, and the pilot" | yes | none |
| IE-7 (part 10, ports) | name the ports | yes: OQ-0006-16 owns the plan, option (a) the interim; TCP 443 left to that question | none |
| SA-9, SA-7 (part 10) | join deadline from accept; /64 key | yes (N29) | d0 #7's optional note only |
| IE-4 routed (part 10, staging certificate) | stale hash in a static preview | yes: OQ-0006-5 with ADR 0002 part 10's two shapes | none |
| RR-2, RR-3, SA-2, SA-3, SA-12, EA-5 + IE-1, IE-11, SA Q2, GD-REJOIN (part 11) | ticks not clocks; panic ends the match; key per environment; one registrable domain; names; directory build field; lobby asks a VM; kick question; rejoin pointer | yes, each | N33: d0 #6's optional precision ("registrable domain and the scheme") |
| GD-2, GD-8, GD-19, GD-3 (part 12) + -b's three blocks | `Buy`; "three" as data; honest races apart; E keys; pick visibility pointer | yes (N34, N35) | none |
| NA Q2 (part 13) | direction split | yes, as proposed + OQ-0001-16 | none |
| diagram, client tick, Ping/Pong, uplinks row, Input/Snapshot, Resync, PickUpgrade/Buy, lobby/supervisor rows, catalogue note | routed and own items | yes | none |
| PA-1 (downstream bullet, budgets row) | judge the mean in R2's scene | yes (N9) | none |
| PA-3, NA-9 (upstream bullet) | restate the upstream with part 5's command | yes (N11–N15) | N12 (low) |
| RR-3, NA-13, EA-5 + IE-1, TE-4 (failure modes) | tick-thread panic; resync; no reload loop; counter wraps | yes | none |
| SA-8, SA-6, GD-2, SA-2, SA-7 (anti-cheat bullets) | follow parts 5, 8, 10–12 | yes | none |
| SA-6 (Q5 B), PA-2 (Q6, Q9 E3b), EA-2 (Q8 T2), PA-3 (Q10 U1/U2) | option tables follow | yes (N38–N40) | none |
| NA-1 (c), EA-2, PA-2, fixcheck-a note (trade-offs 1, 2, 4, 5) | trade-offs restated | yes | none |
| PA-2 (Revisit when; migration step 1; rollback) | per-kind E3b | yes | none |
| TE-1, coordinator item 4 (OQ-0001-2) | stand-ins for the missing sim parts | yes in substance | F1 (N42): the added parenthesis makes "they leave 7.49 ms" wrong; with the 2.80 ms remainder topped up the planning check leaves 4.70 ms |
| NA-1 (d), NA Q1, SA-6, SA-8, GD-8 (OQ-0001-5, -6, -7) | cap options incl. measured cap (not researched, not recommended); arrival vs buffer; reload tap | yes; rule 3 kept (N43) | none |
| EA Q1, NA Q2, SA Q2, ROOM-CRASH (OQ-0001-15 to -18) | new questions | yes; each with options, decider, research ids | none |
| TE-3 (spike item 1; test plan) | hash replay valid only at 0 late commands | yes | none |
| PA-2, NA-1 (e), EA-2 (spike item 5) | measure rewind parts; absolute input-to-screen | yes | none |
| TE-3, TE-4 (test plan: determinism, wraps) | deterministic wrap test | yes (N45) | none |
| SA-8, SA-9, SA-7, SA-13, GD-2, RR-3 (test plan: hostile input) | fast clock, slow handshakes, /64 cycling, control-message fuzz, panic injection | yes | none |
| EA-5 + IE-1 (rollback after acceptance) | two versions side by side | yes: OQ-0006-14 (N46) | none |
| Appendix A rows (R1-ABL-REC4, R1-XT-OQ4, R2-SYNC-REC2, -REC5, -REC6, R2-BUD-REC1, -REC2, R2-ROOM-REC2, -REC8, R2-BUD-OQ8, R2-XU-OQ2, -OQ4) | dispositions follow the text | yes (N47, N48) | none |
| EA-8 routed (test plan: load and soak) | bots on `client-endpoint` | yes | none |
| NOTEs (39) | deferrals and routings | sound: each deferral names its owner (test-engineer rows, README rows, ADR 0005's AB9 list); PA-1's "no SS-BANDWIDTH question" is reasoned (the research's own scene restored); option (c) of OQ-0001-5 not recommended because not researched (rule 3) | none |

## 2. ADR 0001, test-engineer unit A's pass marks (test-engineer-A-fixes-final.md 0001 blocks, -b, -c, -d)

### 2a. Claims

| # | doc:line | claim (short) | cited | verdict | where and the words | note |
|---|---|---|---|---|---|---|
| A1 | 0001:1273 | LG's bot clients speak the real protocol | R5-TEST-REC5, r5 1475–1480 | confirmed | r5:1475–1476 "Bots are headless Rust clients that speak the real protocol" | |
| A2 | 0001:1308–1312 | the half split of latency is R5-TEST-REC4's, proposed there; the jitter split is part 13's proposal | r5 1455–1456 | confirmed | r5:1455–1456 "per direction (half the profile's round trip each way, proposed)" | ADR 0001 part 13 and OQ-0001-16 could cite this too (optional) |
| A3 | 0001:1322–1330 | S30 54,000 ticks, S60 108,000; counter starts 1,800 below 65,536; re-floods 3.3 % and 1.7 % (1 ÷ 30, 1 ÷ 60); changes 60 ticks apart land on one parity; 8.28 of 8.33 ms | arithmetic; GLOSSARY:62–75 | confirmed | GLOSSARY:69 "server tick on a re-flood tick \| \| 8.28 \| 10.18" | |
| A4 | 0001:1354–1366 | 3 runs (R1-TICK-REC2 P1, r1 2047); "repeated at 3 times of day (shared hosts vary with neighbours)" (r6 668–669); 2.5 % = half of 5 % | r1 2047; r6 668–669 | confirmed | r1:2047 "36,000 ticks per run, 3 runs"; r6:668–669 the quoted words | |
| A5 | 0001:1380 | stress scene counts; "a snapshot-receiving client on every soldier" (r2 2412); 700 shots/s (2186–2187, 2280); stand-ins as ADR 0004 S1 (384 rows, 1 ms burn, 2.80 / 0.30 ms) | r2; ADR 0004 S1 | confirmed | r2:2412 the quoted words; r2:2186–2187 "at 700 shots per second"; 0004:982 S1's stand-ins | |
| A6 | 0001:1391 | ES judges the 160 kbps mean; SS's shots add about 20.1 kbps to 146.1; IPv6 +4.8 kbps (line 1784) | r2 2339–2344, 1754–1779, 1784 | confirmed | r2:1784 "IPv6 adds 20 B per datagram: +4.8 kbps at 30 Hz" | |
| A7 | 0001:1397 | PM-0001-57's bites: yaw in 12 bits (0.088° steps), pitch in 8 bits (0.70° steps), positions to 4 cm (errors up to 2 cm) | arithmetic | confirmed | 360 ÷ 4,096 = 0.088°; 180 ÷ 256 = 0.70° (the 10-bit field's 0.18° implies a 180° range) | |
| A8 | 0001:1398 | wrap test starts the tick 30 ticks below a multiple of 65,536, within the run's first 2 s | arithmetic | confirmed | 30 ticks = 1 s at 30 Hz | |
| A9 | 0001:1401 | PM-0001-20: R2-SYNC-REC2 marks 1–2 (2058–2059); 5 % loss as P3 (2323–2324) | r2 | confirmed | r2:2058 "Under P3 (5 % random loss), at least 99.9 % of commands"; :2059 "0 lost and 0 doubled" | |
| A10 | 0001:1402 | PM-0001-21: an off-map point stays forged, a beyond-reach point is clamped; a tick equal or older joins the forged set; fast clock 1.5 × for 60 s | part 5; r2 2060–2066 | confirmed | matches part 5 at 0001:216–219; r2:2062 "an aim point off the map or beyond weapon range" (the research's set, changed and so labelled) | GD-1 resolved in the marks |
| A11 | 0001:1406 | PM-0001-25: never-`Join` flood (R2-TRN-REC1 mark 4, 2597–2609); suite on both transports (2670–2671) | r2 | confirmed | r2:2609 "refused with 403, from all four browsers"; r2:2670–2671 "One hostile-input suite … runs against both transports" | |
| A12 | 0001:1408 | an uncaught panic "will exit" only its own thread (R2-ROOM-F8, 1459–1462) | r2 | confirmed in substance | r2:1460 "If the panic is not caught the thread will exit"; r2:1459 "ends only its own thread" | the quote stitches two places; harmless |
| A13 | 0001:1409 | PM-0001-27: within 10 %; SIGKILL within 5 % ("unchanged (within 5 %)", R2-TRN-REC6 mark 2); R2-ROOM-REC2 marks 2 and 4 (2777, 2779–2780) | r2 | confirmed | r2:2777 "each room's p99 within 10 % of the single-room figure"; r2:2779 "killing one room's process (SIGKILL)" | consistent with PM-0006-13 (1.10 ×, 384 = 8 × 48 clients, bracket B) |
| A14 | 0001:1419 | PM-0001-37: lateness from the expected arrival (send + 50 ms at P2), more than 66.67 ms; research reading reported | r2 2633–2634 | confirmed | r2:2633–2634 "(arriving more than two snapshot intervals after sending)" | |
| A15 | 0001:1423 | PM-0001-39: 1 − 6.25 ÷ 8.33 = 25 %; tick 65,536 at 18.2 min (65,536 ÷ 60 ÷ 60); 60 snapshots a second would be about 286.1 kbps (line 1778) | arithmetic; r2 1778 | confirmed | r2:1778 "48, E3 at 60 Hz \| … \| 286.1 kbps" | |
| A16 | 0001:1430 | PM-0001-44: 7 ticks = (100 + 100 + 33.3) ÷ 33.3; 12 ticks at 60 Hz; 18,000 replays, p99 the 17,820th; "part 4 gives 6.5–8.5 ticks at P3 and 8–10 at P4" | arithmetic; part 4 | numbers confirmed; citation off | part 4 (0001:181–183) says only "deeper at P3 and P4"; the figures are the netcode review's finding 10 (review-netcode-architect.md:49), and they follow from part 4's formula: (150 + 33.3 + 33.3) ÷ 33.3 = 6.5 to (150 + 100 + 33.3) ÷ 33.3 = 8.5; (200 + 66.7) ÷ 33.3 = 8 to (200 + 133.3) ÷ 33.3 = 10 | "part 4 gives" → "part 4's formula gives (arithmetic)" (low; Finding F6) |
| A17 | 0001:1433 | PM-0001-61: one tick plus one refresh period, 33.33 + 16.67 = 50.00 ms; the bite plays 100–166.7 ms early | arithmetic; part 7 | confirmed | part 7's 2 to 5 intervals... the bite uses the 3–5 interval range (100–166.7 ms), as part 7 at 0001:331–332 | |
| A18 | 0001:1435 | PM-0001-48: at 9 m/s, 100 ms is about 0.9 m; about 20 misses at 99 % of 2,000; cap above every honest rewind; R2-SYNC-REC5 mark 1 (2179–2181) | arithmetic; r2 | confirmed | r2:2179–2181 "at least 99 % … up to P2 and 95 % at P3, … false hits … below 0.5 %" | |
| A19 | 0001:1436 | PM-0001-49: three candidate caps; per-shot parts incl. the 0–1 frame wait; mark 6 (2191–2192); the cap text (2154–2159) | r2 | confirmed | r2:2191 "No shot is resolved with a rewind above the cap: 100 % of shots under P0–P4"; r2:2154 "The cap bounds the final rewind" | the frame wait it records is not in part 8 (Finding F3) |
| A20 | 0001:1437 | PM-0001-50: gap at least 145 ms (150 − 5); mark 5 (2188–2190) | arithmetic; r2 | confirmed | r2:2188 "A scripted cheat client that announces the largest allowed delay" | |
| A21 | 0001:1440 | PM-0001-62: R1-ABL-REC4 quote (2427–2428) and its mark "every accepted activation plays its cue exactly once" (2443–2444) | r1 | confirmed | r1:2443–2444 "every accepted activation plays its cue exactly once (count over 1,000 scripted presses)" | |
| A22 | 0001:1428 | PM-0001-60: at most one correction, at the first snapshot that carries the change; 1 mm (R2-SYNC-REC3 mark 1, 2101–2102) | r2; part 4 | confirmed against r2; contradicts part 4 | r2:2101 "0 corrections larger than 1 mm whenever every command arrived in time"; part 4 at 0001:188–189 still says "so a pick or an effect that changes move speed or a fire interval causes no correction" | Finding F2: the decision text, not the mark, should change (test-engineer A's routed NOTE GD-3) |
| A23 | 0001:1442 | PM-0001-54: about 188.3 kbps in ES; 300 × 5.5 = 1,650 B; R2-BUD-REC2 (line 2342) | r2 | confirmed | r2:2342 "0 desyncs of deterministic projectile flight against the server over 1000 throws" | |
| A24 | 0001:1523 | step 10: at P2 an honest shot needs 233–383 ms, both caps clip most | part 8 | confirmed | part 8 at 0001:362–366 | |
| A25 | 0001:1354–1360 vs 1409 | V-S: "The first two clauses hold for single-room marks only" (so LG's clause, 30 inputs a second within 1 % and 80 % CPU, holds for PM-0001-27); PM-0001-27: "validity is PM-0006-13's (bracket B and LG's 80 % rule), not V-S" | the key | inconsistent (introduced by follow-up c) | the key keeps the 30-inputs-within-1 % check for PM-0001-27; the row drops it | Finding F7 (low) |

### 2b. Blocks (test-engineer A, ADR 0001: 41 FIX in the final file, 1 in -b, 2 in -c, 1 in -d)

| block | finding | resolved? | new problem |
|---|---|---|---|
| row list (PM-0001-58 to -62) | TE-4, RR-3, GD-3, EA-1, NA-3 | yes | none |
| machines key (pinning exemption; LG calibration; cap raised) | EA-7, IE-2, IE-9/10 | yes | none |
| ES key | NA-2, PA-1 | yes (A6) | none |
| profiles key (direction split) | NA-5, NA Q2 | yes (A2) | none |
| soaks key (door phase; wrap as second check) | PA-9, TE-4 | yes (A3) | none |
| V-S (rooms-per-VM row exempt from pinning and steal; LG's CPU) | EA-7, IE-2, IE-9/10 | yes | after follow-up c, the key and PM-0001-27 disagree on LG's 30-inputs clause (A25, F7) |
| PM-0001-1 (+ -c, -d) | EA-7, IE-2; SMT siblings as PM-0006-11 | yes; matches PM-0006-11 | none |
| PM-0001-2 | PA-9, TE-1's stand-ins, NA-2 | yes (A5) | none |
| PM-0001-3 | NA-5 | yes | none |
| PM-0001-12 | NA-2, PA-1 | yes | none |
| PM-0001-13 | PA-3, NA-9 | yes; 3 to 6 commands as part 5's estimate | none |
| PM-0001-58 (new) | TE-4 | yes (A8) | none |
| PM-0001-20 | NA-1 | yes | none |
| PM-0001-21 | GD-1, SA-8 | yes (A10) | none |
| PM-0001-25 | SA-9, IE-9/10, SA-7 | yes (A11) | none |
| PM-0001-26, PM-0001-59 (new) | RR-3 | yes; the 1 s is "proposed" in the failure-mode row | none |
| PM-0001-27 (+ -c) | EA-7, IE-2 | yes; matches PM-0006-13 | F7 (with V-S) |
| PM-0001-29 | IE-7, IE-9/10 | yes; TCP 443 reported only, for OQ-0006-16 | none |
| PM-0001-30 | TE-3 | yes, with a validity bite | none |
| PM-0001-33 | NA-4 | yes | none |
| PM-0001-34 | SA-13 | yes, one fuzz target in PM-0004-60 | none |
| PM-0001-37 | NA-5 | yes (A14) | none |
| PM-0001-39 | NA-12, PA-9 | yes (A15) | none |
| PM-0001-42, PM-0001-60 (new) | NA-11; GD-3 | yes; the row states what the section can guarantee | F2: part 4's decision text still claims "no correction" (routed NOTE GD-3 not applied) |
| PM-0001-43 | EA-2 | yes, reported only | none |
| PM-0001-44 | NA-10 | yes | A16 / F6: "part 4 gives 6.5–8.5 … 8–10" cites part 4 for figures it does not state |
| PM-0001-45 | FC-7 | yes | none |
| PM-0001-46, PM-0001-61 (new) | EA-1 | yes, restated to one tick plus one frame with a reason; the tighter bound routed | none |
| PM-0001-47 | NA-4 | yes | none |
| PM-0001-48 (+ -b) | NA-1 | yes (A18) | none |
| PM-0001-49 | NA-1 (e), SA-6 | yes (A19); records the frame wait part 8 omits | F3 (part 8 side) |
| PM-0001-50 | SA-6 | yes (A20) | none |
| PM-0001-52, PM-0001-62 (new) | NA-3 | yes (A21) | none |
| PM-0001-53 | new rows beside the checklist | yes | none |
| PM-0001-54 | PA-2 | yes (A23) | none |
| PM-0001-56 | EA-2, NA-1, NA-10 | yes | none |
| "When a mark fails" | NA-6, PA-2 | yes: only PM-0001-4 to -10 reopen parts 2 and 3 | none |
| no-mark list | GD-2 | yes, purchases at slice 4 (reasoned NOTE: the throwaway sim has no economy) | none |
| checklist intro, steps 7, 10, 11 | EA-1, NA-3, NA-1 | yes (A24) | none |
| NOTEs (12) | NA-1 frame wait, GD-3 wording, EA-1 one-frame bound, EA-7/IE-2, PA-5, IE-9/10, SA-13, RR-3, GD-2, RR-8, synthesis, index | sound; two route decision-text changes to netcode-architect (NA-1's frame wait, GD-3's "no correction") that HEAD does not carry and no NOTE of netcode-architect defers | F2, F3 |
