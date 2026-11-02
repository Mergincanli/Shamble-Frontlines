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

## 3. ADR 0002, engine-architect's text (engine-architect-0002-fixes-final.md, 52 FIX)

### 3a. Claims

| # | doc:line | claim (short) | cited | verdict | where and the words | note |
|---|---|---|---|---|---|---|
| E1 | 0002:140–142 | cargo-fuzz needs a nightly (R5-TEST-F4, r5 430–434); one dated nightly in `rust-toolchain.toml` (R5-CI-F12, r5 632–639) | r5 | confirmed | r5:430–432 "needs a nightly compiler since it uses some unstable command-line flags"; r5:633 `channel = "nightly-2020-07-10"` | |
| E2 | 0002:207–211 | cargo-deny's `sources` class (R5-GATE-F3, r5 954–960), its options not read | r5 | confirmed | r5:957–958 "(advisories, bans, licenses, sources in the chapter list)" | honest about the unread options |
| E3 | 0002:215–216 | Cargo unifies features over everything one command builds (R5-LAYOUT-F4) | r5 83–91 | confirmed | r5:85 "Cargo will use the union of all features enabled on that dependency" | |
| E4 | 0002:219–226 | `std::thread::spawn` panics on the default Wasm target (R1-BND-F19, r1 1019–1025); R2-TRN-REC4 (r2 2673–2698); R4-XT-OQ5 (r4 1195–1200); R4-NAV-REC1 (802–810); R4-NAV-REC4 (888–899) | r1, r2, r4 | confirmed | r1:1020 "`std::thread::spawn` will panic"; r4:1195–1196 "Where the bake code lives decides whether the generator ships in the player's Wasm"; r4:888 "Client prediction never runs it" | |
| E5 | 0002:256–261 | binaryen `--fast-math` (`-ffm`) "Optimize floats without handling corner cases of NaNs, rounding, and negative zero" | binaryen's options, read by rust-reviewer | confirmed (d0 #1) | d0 #1: `src/tools/optimization-options.h` lines 311–318, word for word | |
| E6 | 0002:261–262 | R1-BND-F19's default feature list (r1 1019–1025) | r1 | confirmed | r1:1023–1025 "multivalue, mutable-globals, reference-types, sign-ext, nontrapping-fptoint …, bulk-memory" | |
| E7 | 0002:269–303 | block sections incl. weapon pickups (ADR 0004 part 18), floor layer and enclosure area id (ADR 0004 part 14), the event list of ADR 0004 part 19; spatial events released when the interpolation clock reaches their tick (ADR 0001 part 7; R2-SYNC-REC6 line 2210) | ADR 0004; ADR 0001; r2 | confirmed | 0004:443–445 "the local player's floor layer and enclosure area id, written into the output block"; 0004:535–539 the event list; 0001 part 7 at :319–327 | consistent with ADR 0001 part 7 (see consistency table) |
| E8 | 0002:303–308 | writer trigger 0.05 × 13.33 = 0.67 ms (proposed by rust-reviewer) | arithmetic | confirmed | 0.6665 ms; 13.33 ms is GLOSSARY's client's sim share at 30 Hz | |
| E9 | 0002:317–322 | a post per block can leave a press up to 2 × 1000 / 30 = 66.67 ms; R1-BND-REC3 mark 2's one tick plus one frame (33.33 + 16.67 = 50.00 ms; r1 2286–2296) | r1; arithmetic | confirmed | r1:2294–2295 "at most one tick plus one frame at p95" | |
| E10 | 0002:327–330 | without arrival times, up to one tick period of false jitter (0 to 33.33 ms) | arithmetic (NA-4) | confirmed | 1000 ÷ 30 | |
| E11 | 0002:334–335 | catch-up counted per wake, at most 7 ticks (ADR 0004 part 5, S18) | ADR 0004 | confirmed | 0004:998 S17 "never more than 7 ticks in one wake" | the "at most 7" is S17's (the server loop) restated for the Worker; S18 (the Worker) carries the per-wake reading. Fine |
| E12 | 0002:336–338 | a fresh view after every call, since growth makes an old view stale (proposed: R1-BND-REC2 names re-binding on growth as a cost of B4, r1 2273–2274) | r1 | confirmed | r1:2273–2274 "B4's zero copies cost … re-binding on growth" | |
| E13 | 0002:346–349 | "Dev-dependencies do not activate features unless building a Cargo target that needs them" (R5-LAYOUT-F4, r5 83–91) | r5 | confirmed | r5:89 the quoted words | |
| E14 | 0002:397–400 | the client's sim share covers the sim step only (GLOSSARY lines 214–218) | GLOSSARY | confirmed (as cited in the drafts throughout) | | |
| E15 | 0002:436–443 | float ban as the allow-list's complement (R1-DET-REC2, r1 2146–2150); `powi`, `hypot`, `cbrt` (R1-DET-F9, r1 648–654, inferred there) and `to_degrees`, `to_radians` carry std's note (std 1.99.1, read by rust-reviewer); `min`/`max`/`clamp` signed zeros, `clamp` panics (R1-DET-F14, r1 679–683) | r1; std via the Rust review | confirmed (d0 #3) | r1:652–653 "that the same note sits on … `powi`, `hypot`, `cbrt` … is inferred"; r1:679–683 the min/max/clamp quotes; d0 #3: std 1.99.1 pages carry "§Unspecified precision" on all five | allow-list matches ADR 0004 part 7 (0004:257–263: `abs`, `fract`, `copysign` allowed) |
| E16 | 0002:451–455 | std's clock calls panic on `wasm32-unknown-unknown` (std's source, read by rust-reviewer); `-Cpanic=abort` makes it a trap (R1-BND-F19) | std; r1 1019–1025 | confirmed (d0 #2) | d0 #2 `unsupported.rs` lines 12–14, 34–36; r1:1021–1022 "compiled with `-Cpanic=abort`" | |
| E17 | 0002:528–531 | failure artefacts kept for the retention ADR 0006 sets (part 3; R5 leaves it to infra-engineer, r5 line 2083); "until it is set, the 7 days of build artifacts" | r5 2083; ADR 0006 part 3 | r5 confirmed; the conditional is stale | r5:2083 "infra-engineer: … artefact retention for failures (REC6)"; ADR 0006 part 3 (0006:171–175) now sets it: "CI failure artefacts … are kept 30 days" | Finding F9 (low): "until it is set, the 7 days of build artifacts" → "30 days (ADR 0006 part 3)" |
| E18 | 0002:539–543 | bench on a CCX23 per session, 20 bench hours a month, 20 × 0.1386 = 2.77 EUR (R6-HOST-F15, r6 150–158) | r6; arithmetic | confirmed | r6:153–154 "CCX23 4 vCPU / 16 GB 86.49 (0.1386)"; 20 × 0.1386 = 2.772 | |
| E19 | 0002:606–613 | Chrome on Linux WebGPU only on Intel Gen12+ (R3-WGPU-F17, r3 159–162); Firefox not on Linux (F18, 164–169); no adapter falls back (R3-GL2-F12, 279–283); headless Chrome "SwiftShader Device (Subzero)" (R3-MEASURE-F8, 648–653) | r3 | confirmed | r3:160 "Supported on ChromeOS, macOS, Windows, and Linux (Intel Gen12+ GPUs only)"; r3:279–280 "Firefox on Linux … ('Does not support Linux.')"; r3:649 "a 'SwiftShader Device (Subzero)' GPU" | the SwiftShader WebGPU job is labelled not researched |
| E20 | 0002:645–651 | a `pull_request` run takes its workflow from the PR's merge commit (the contrast R5-PREV-F2 draws, r5 770–774; the security review's reading) | r5; the security review | confirmed (d0 #5) | r5:771–772 "rather than in the context of the merge commit"; d0 #5: GitHub's "Securely using pull_request_target" page: "it runs the workflow file from the merge commit of the pull request" | optional (d0): cite that page's sentence beside R5-PREV-F2 |
| E21 | 0002:651–653 | the token is account-scoped and the `branch` option lets a deploy name any branch (R5-PREV-F10, r5 823–830); reaching production "an inference" | r5 | confirmed | r5:824–827 "an account-scoped 'Pages Edit' API token"; "unless you add a `branch` option" | inference labelled |
| E22 | 0002:655–663 | "a job cannot access environment secrets until one of the required reviewers approves it"; "Deployment branches and tags are available for all public repositories" (R5-PREV-F6, r5 796–804); the repository is public (r5 752–754) | r5 | confirmed | r5:798–799 both quotes; r5:752 "The repository is public" | |
| E23 | 0002:672–678 | a self-signed certificate lives at most 14 days (R2-TRN-REC5, r2 2700–2703); mark 2: new connections succeed with the new hash within one reload (r2 2720–2721); production a trusted certificate (r2 2704) | r2 | confirmed | r2:2700 "valid ≤ 14 days"; r2:2720–2721 "within one reload"; r2:2704 "a publicly trusted certificate per game server hostname" | the supervisor-held rotation is "proposed, not researched", as labelled |
| E24 | 0002:1020–1025 | macOS runners standard and "free and unlimited on public repositories" (the page R5-CI-F17 read, r5 673–678, which quoted only ubuntu rows; macOS rows read by infra-engineer) | r5; d0 #4 | confirmed (d0 #4) | r5:673–677 (ubuntu rows only); d0 #4: "macOS 3 (M1) … macos-latest" in the standard runners' table | |
| E25 | 0002:1029–1031 | "a private repository's runners draw on paid minutes (the same page, read at the step-10 review)" | GitHub's runners page | confirmed in substance, wording imprecise (d0 #8c) | d0 #8c: the page says they "draw from your GitHub account's included compute usage, and are then charged at the per-minute rates" | Finding F8 (low): "draw on paid minutes" → "draw on the account's included compute usage and are then billed per minute" |
| E26 | 0002:1055–1062 | R5-LAYOUT-REC2 puts rooms inside `server` (r5 1173–1174); R4-XT-OQ5 (r4 1195–1200, no answer on one side); R5-XT-OQ6 names both sides (r5 1996–2001); R4-XT-OQ5 = R5-XT-OQ7 | r5; r4 | confirmed | r5:1173 "`server` (the binary with transport and rooms …)"; r4:1197–1198 "Side B: missing answer"; r5:1998–1999 "Side A: R6-HOST-REC3; R2-TRN-REC5. Side B: R5-PREV-REC1"; r5:2002 R5-XT-OQ7 asks the same question | |
| E27 | 0002 Appendix A, R5-PREV-REC1 | the reviewer gate the REC allows "when wanted" | r5 1597 | confirmed | r5:1597 "and, when wanted, a reviewer gate (F6)" | |
| E28 | 0002:1057–1060, 645–664, Q4 row | the three new crates are "proposed (not researched)" with OQ-0002-8 for the Advisor | rule 3 | rule kept | the chosen Q4 row says "not researched as a placement … proposed only for the Advisor's ruling at the stop (OQ-0002-8)" | an unresearched option is the proposed structure, but openly and with the rule question routed to the Advisor; not a new breach (it predates the round; the round adds `client-endpoint` under the same label) |

### 3b. Blocks (engine-architect 0002, 52 FIX, 21 NOTE)

| block | finding | resolved? | new problem |
|---|---|---|---|
| NA-7 (summary, Decision opening, part 5 input and why, Q10 C1, trade-off, spike item 3, migration skeleton, Appendix A R1-BND-REC3) | the page's posting cadence | yes: newest input once per drawn frame, the Worker builds one command per tick (E9) | none |
| RR-15 (parts 1, 2) | a dated nightly for fuzz | yes (E1) | none |
| EA-8, RR-2, EA-12 (crate table, rules, R5-LAYOUT change, Q4, trade-off, migration, Appendix A) | `client-endpoint` crate; no clock in `room`/`client-endpoint`; supervisor crate; lobby crate waits | yes (E4, E28) | E28: `client-endpoint` joins the "proposed, not researched" crates under OQ-0002-8 (rule 3 routed, not new in kind) |
| RR-10, SA-10 (rules; G5) | same-feature check over four builds; cargo-deny `sources` | yes (E2, E3) | none |
| RR-7, EA-8, routed b815c85 (part 4) | trap message; `-ffm` in the flag check; shipped file | yes (E5, E6) | none |
| EA-1, EA-12, RR-5 (part 5, the block) | events on their entity's timeline; floor layer and area id; R1-RR-OQ6 confirmed with three conditions | yes (E7, E8) | none |
| NA-4, RR-2, RR-6, NA-8, routed b815c85 (control API) | arrival stamps; time as values; fresh memory view; next-tick time; per-wake catch-up | yes (E10–E12) | none |
| RR-16 (generated types) | derives kept out of shipped builds | yes (E13) | none |
| NA-15, NA-7, PA-13, routed b815c85 (part 6) | order inside one Worker step; Worker as a whole reported | yes | none |
| SA-11 (part 7 TS; G8) | CSP-ready build | yes, "proposed, not researched", ESLint pages not read (said) | none |
| RR-1, RR-2, RR-14 (part 7 Rust) | allow-list complement; clock ban in four crates; client-sent ids through checked lookups | yes (E15, E16) | none |
| RR-7 (part 8 tools; G10) | determinism on the shipped file | yes | none |
| TE-7, EA-8 (soak and load) | tick times judged only on the reference server | yes | none |
| IE-19 (flakes) | failure artefacts' retention | yes in substance | F9: "until it is set, the 7 days" is stale now that ADR 0006 part 3 sets 30 days (E17) |
| IE-14 (bench method) | the bench's access path and cost | yes (E18), OQ-0006-17 | none |
| GE-2 (WebGPU in CI; test plan; Appendix A R5-CI-OQ5) | CI never runs WebGPU | yes (E19); the SwiftShader WebGPU job is labelled not researched; the fallback is a reference-machine smoke run | none |
| SA-1 (Sentry job; part 10; revisit; migration previews; Appendix A R5-PREV-REC1) | production only from `main`; required reviewer on `preview` | yes (E20–E22); points to OQ-0006-13, no OQ-0002-9 | d0 #5's optional citation only |
| IE-4 (part 10; Appendix A R2-XT-OQ10) | stale hash in a static preview | yes (E23), both shapes, OQ-0006-5 | none |
| IE-3 (OQ-0002-5) | macOS free on a public repository | yes (E24) | F8: "paid minutes" is imprecise (d0 #8c) |
| FB-8, EA-8 (OQ-0002-8) | ids line; "no answer on one side" fits R4-XT-OQ5 only | yes (E26) | none |
| NA-4/NA-7 (spike item 1), EA-1/RR-5 (item 2), PA-13 (item 6), b815c85 (item 7) | spike items follow | yes | none |
| EA-12 (slice 3) | supervisor enters with `server` | yes | none |
| NOTEs (21) | routings and alignments | sound; IE-19's routed retention landed in ADR 0006 part 3 (30 days) but ADR 0002 part 8 was not updated to it (F9) | F9 |

### 3c. Claims and blocks (test-engineer A, ADR 0002: 25 FIX in the final file, 2 in -c)

| # | doc:line | claim (short) | cited | verdict | where and the words | note |
|---|---|---|---|---|---|---|
| T1 | PM-0002-3 | "the same number of Wasm calls per tick at N = 12 and N = 48 (at most 4)"; 8 = 4 × 2 | R1-BND-REC1 mark 2, r1 2261–2263 | confirmed | r1:2262–2263 "(at most 4)" | the glue-allocator reason now "proposed" (FC-10) |
| T2 | PM-0002-6, -9, -12, -13, -23 | R1-BND-REC2 marks 1, 4 (r1 2276–2277, 2280–2281); R1-BND-REC3 marks 1–2 (2292–2295); R1-BND-REC6 mark 2 (2335–2336); p95 = 9,500th of 10,000; 2 × 1000 / 30 = 66.67 | r1; arithmetic | confirmed | r1:2276 "at most one block copy in the worker"; r1:2281 "zero lost or reordered events over a 10-minute match"; r1:2335–2336 "a 100 ms busy loop injected on the page leaves no gap" | PM-0002-9's per-timeline order matches ADR 0002 part 5 and ADR 0001 part 7 |
| T3 | PM-0002-10 | 0.67 ms = 5 % of 13.33 ms; R1-BND-OQ6 (2737–2740), R1-RR-OQ6 (2951–2954) | r1 | confirmed | | |
| T4 | PM-0002-77 (new) | the Worker's cast against the newest block is up to one tick off, about 0.3 m at 9 m/s | arithmetic | confirmed | 0.0333 × 9 = 0.30 m | EA-4 resolved |
| T5 | PM-0002-17, -42 | `enum` bite required; R5-TSLINT-REC1 (r5 1261–1263), R5-TSLINT-F12 (221–224), R5-TSLINT-OQ3 (1770–1772) | r5 | confirmed | r5:1770 "Do the generated types pass `erasableSyntaxOnly`? If ts-rs or wasm-bindgen emit `enum`" | FB-1 resolved; the ambient-enum case honestly left unclaimed |
| T6 | PM-0002-28 | cites PM-0004-8's one reading (median ≤ 28 ms, p95 ≤ 33.3 ms, mode U); 40 % of 33.33 = 13.33 ms | ADR 0004 PM-0004-8 | confirmed | 0004:1113 "frame time median ≤ 28 ms and p95 ≤ 33.3 ms, mode U … ADR 0002's PM-0002-28 cites this row and sets none of its own" | PA-5 aligned both ways |
| T7 | PM-0002-29 | 25 × 1,048,576 = 26,214,400; R6-HOST-REC1's 25 MiB (r6 642); R1-RR-OQ5 (2946–2950) | r6; r1 | confirmed | r6:642 "the largest build file stays under 25 MiB" | FC-3: "an input to R1-RR-OQ5, which the Advisor decides" |
| T8 | PM-0002-33 | trap report with the hook's message; R6-OBS-REC3 (r6 765–766) | r6 | confirmed | r6:765–766 "a deliberate Wasm trap … arrives in Sentry with Rust function names in Chrome, Edge, Firefox and Safari (4 of 4)" | |
| T9 | PM-0002-41 | `powi`, `to_radians`, `min` bites; `Instant::now()` in `room` and `client-endpoint`; `-ffm`; controls `abs` and `fract` | ADR 0004 part 7; R5-GATE-REC3 G2 (r5 1714) | confirmed | 0004:257–261 allow-list holds `fract` and `abs`; r5:1714 the G2 row | |
| T10 | PM-0002-44, -45, -46, -51 | G5, G7, G8, G12 rows (r5 1717, 1718 "its size is set once REC4's budget exists", 1719, 1723); R5-LAYOUT-REC2's mark (1191–1192 "only the manifest walk sees a path built at run time") | r5 | confirmed | each line holds the quoted words | |
| T11 | PM-0002-64 | 1,800 ticks = 1 minute; "every tick runs the newest posted input" | arithmetic; part 5 | confirmed | | NA-7 resolved |
| T12 | PM-0002-65, -66 | link within 5 min after the build or after the approval (R5-PREV-REC1 mark 1, r5 1600–1601); no checkout or `npm ci` (mark 3, 1602–1603); a production job from a branch refused | r5; part 10 | confirmed | r5:1600 "within 5 min after the build job ends (median of 5 runs)"; r5:1602–1603 | SA-1 resolved; matches ADR 0006's PM-0006-32 (TR) |
| T13 | PM-0002-76 | 3-hour soak on the reference server; R5-TEST-REC5 (r5 1475–1489) | r5; part 8 | confirmed | | TE-7 resolved |
| T14 | PM-0002-78 (new) | the Worker's whole cost, reported; heap beside PM-0003-24's 0.5 MB flat rule "for reading, not as a pass" | ADR 0003 PM-0003-24 | confirmed | | PA-13 resolved; "proposed tool, not researched" for the CDP heap read |

Blocks: all 27 resolve their findings (FC-10, RR-6, EA-1, RR-5, NA-7 ×4, EA-4, FB-1 ×2, FB-9, PA-13 ×3, PA-5, FC-3/RR-8/RR-7, RR-9, RR-1/2/7, EA-8/SA-10/RR-10, FB-3, SA-11, GE-2 ×2, SA-1 ×2, TE-7); follow-up c points PM-0002-51 and -64 to PM-0003-46, which exists (0003:1180). New problems: none.
