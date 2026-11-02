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
