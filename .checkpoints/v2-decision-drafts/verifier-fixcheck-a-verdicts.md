# Verifier fix-check A: the fix rounds' changed text in ADR 0001, 0003 and 0006 (plan step 9)

verifier (xhigh, opus), 2026-11-01 from 19:19 Amsterdam. Branch claude/laughing-lovelace-ox61se at 57c43c2 (the three
docs in the working tree are identical to 57c43c2). Range: `git diff -U3 80a27b3 57c43c2` on the three docs (0001: 35
hunks, 62 new-side lines; 0003: 90; 0006: 74). Research files: unchanged since 80a27b3 (line numbers as cited).
Line numbers "doc:n" are the file at 57c43c2. Verdicts: confirmed · wrong · unsupported · citation off · stale ·
rule. I edited nothing; scratch in `.gate/fixcheck-a/`.

## 1. The changed text

### ADR 0001 (docs/adr/0001-multiplayer-netcode.md)
| # | doc:line | claim (short) | cited | verdict | where and the words | note |
|---|---|---|---|---|---|---|
| A1 | 0001:24–25 | TICK-30 quote "a 30 Hz server tick is the baseline for 48 players; 60 Hz only if the netcode proof test shows at least 25 % headroom on the target hardware" | STATUS, TICK-30 | confirmed | STATUS.md:161–163 "a 30 Hz server tick is the baseline for 48 players; 60 Hz only if the netcode proof test shows at least 25 % headroom on the target hardware" | "headroom" inside the TICK-30 quote: allowed (rule 5) |
| A2 | 0001:44 | "below also 'the proof test'" (one definition line) | GLOSSARY §8; brief §2 | confirmed | consistency F13's outcome, worded as asked | — |
| A3 | 0001:97 | STATUS's ADR-001 inputs: the server repeats the team allow-list; humans take over bot slots | STATUS, the ADR-001 entry | confirmed | STATUS.md:249 "the server must repeat the team allow-list (`teamOf`)" … "humans take over bot slots" | — |
| A4 | 0001:126 | R2-BUD-REC6's two shares at 60 Hz, 4.17 ms each | R2-BUD-REC6 | confirmed (arithmetic) | r2:2398–2401 "25 % each"; 25 % × 16.667 = 4.17; GLOSSARY.md:59, 64 "(4.17)" | r1:2048 writes P1's share "4.2 ms (25 % of 16.7 ms)": two decimals is the consistency F3 form |
| A5 | 0001:145–148 | 30 Hz: p99 ≤ 16.67 ms (50 %), max < 33.33 ms, shares ≤ 8.33 ms each | R2-BUD-REC6; GLOSSARY | confirmed (arithmetic) | 1000 ÷ 30 = 33.33; × 0.5 = 16.67; × 0.25 = 8.33; GLOSSARY.md:59, 64, 68 "(8.33)", "(16.67)" | — |
| A6 | 0001:159–161 | R2-BUD-REC6's reason restated without "re-flood ticks included" inside the quote | r2:2402–2403 | confirmed | r2:2402–2403 "fit inside the server tick budget's 50 % of the tick period at 30 Hz, and at 60 Hz on every tick but one that both encodes a snapshot and re-floods after a portal change" | the words after "re-floods after a portal change" inside the marks are condition 2's restatement (unchanged) |
| A7 | 0001:162–163 | at 30 Hz the marks fit with re-flood ticks included, 10.18 of 16.67 ms | GLOSSARY.md:69, 76–77 | confirmed | GLOSSARY.md:69 "server tick on a re-flood tick \| \| 8.28 \| 10.18"; :76–77 "At 30 Hz the marks fit at N = 48 and N = 54, re-flood ticks included" | the limit 16.67 is on :68 (the row above); not worth a fix |
| A8 | 0001:230–231 | gameplay events = "the events of ADR 0004 part 19: hits, deaths and kills, purchases, uplink cache events, tech tiers, upgrade picks and pickups" | ADR 0004 part 19 | unsupported (incomplete list) | 0004:454–458 lists also "blasts (position and strength …)", "harvester deliveries and kills, structure kills and stage changes" (and "cosmetic shot events", which 0001:234 sends once) | finding L1 |
| A9 | 0001:338–342 | dev and preview self-signed pinned certificates are for WebTransport; the WebSocket fallback cannot pin a hash, its constructor takes only a URL and sub-protocols | R2-TRN-REC5; R2-ROOM-F12, r2:1494–1500 | confirmed | r2:1494 "The WebSocket constructor takes only a URL and sub-protocols"; r2:1499 "`new WebSocket(url)` `new WebSocket(url, protocols)`" | "cannot pin" is a sound inference from the signature (no certificate-hash option); ADR 0006 OQ-0006-5 (0006:658–659) words it the same |
| A10 | 0001:341–342 | the fallback presents the certificate OQ-0006-5 names for development and staging | ADR 0006 OQ-0006-5 | confirmed | 0006:655–656 "which certificate does the WebSocket fallback present on development and staging servers?" | 0001 says "preview", 0006 "staging": previews talk to the staging game server (0006:149), same servers |
| A11 | 0001:391–393 | the drop rules and bots' records are Mustafa's; they are OQ-0005-AB9 and OQ-0005-AB10 | ADR 0005 | confirmed | 0005@57c43c2:1198 AB9 "Which drop and pickup rules …", Decider "Mustafa, with the game-designer"; :1211 AB10 "Do bots have tracks …", Decider "Mustafa" | — |
| A12 | 0001:494 | `ContentReload` only if OQ-0005-CD4 chooses (b), the mid-match swap; under (a) nothing changes, no message | ADR 0005 OQ-0005-CD4; R1-DATA-REC5 | confirmed | 0005@57c43c2:616–618 "(a) at the next match (proposed, CD-7): nothing in a running match changes, no reload message; (b) swapped between two ticks … which needs ADR 0001's `ContentReload` message" | — |
| A13 | 0001:533–536 | budgets rows at two decimals: 16.67, 33.33, 8.33, 8.33; 60 Hz each share ≤ 4.17 | R2-BUD-REC6; GLOSSARY | confirmed (arithmetic) | as A4, A5 | — |
| A14 | 0001:628 | Q6 cites encodings E3 and E3b at r2 lines 1694–1702 | r2:1694–1702 | confirmed | r2:1694 "#### B. Downstream encoding"; :1700 E3; :1701 E3b; :1702 E4 | fixes verifier F10 |
| A15 | 0001:653 | Q9 cites table B at lines 1694–1702 | r2:1694–1702 | confirmed | as A14 | — |
| A16 | 0001:665 | U1: "the catch-up cap no larger than the redundancy cap" | part 5; r2:2039–2041 | confirmed | r2:2039–2040 "So the catch-up cap is data, proposed no larger than the redundancy cap above (8 commands)" | — |
| A17 | 0001:808–811 | the Worker's interpolation, aim ray and loopback codec cost "is reported beside the client's sim share … not inside it, since that share is sim step time only" | GLOSSARY.md:214–218 | confirmed, one pointer missing | GLOSSARY.md:215 "in the client, the sim step time in Wasm in the Worker …"; the report exists only as ADR 0002's PM-0002-25 (0002:1064 "the Worker's time per tick (median, p95, p99) beside the client's sim share (13.33 ms at 30 Hz)", local match), which this sentence does not name; no PM-0001 row reports it | note N-a (cite PM-0002-25) |
| A18 | 0001:850–851 | OQ-0001-1 (b): "not confirmed: Mustafa and the Advisor name the base, and part 2's mark and GLOSSARY §8 follow" | consistency F14 | confirmed | the critic's outcome text, word for word | — |
| A19 | 0001:922–923 | OQ-0001-14: R2-BUD-REC4, REC5 and R2-TRN-REC6 serve only VISION_V2 §4's "matches feel fair up to about 100 ms of ping" | r2:3260–3269 | confirmed | r2:3261–3263 "R2-BUD-REC4 and R2-BUD-REC5 serve '§4 (fair to about 100 ms…)'. R2-TRN-REC6 serves 'the §4 fairness target (about 100 ms) at the §3 scale (48+ players)'"; VISION_V2.md:55 "Matches feel fair up to about 100 ms of ping" | TRN-REC6 also names the §3 scale; the question itself asks "§3 or §4", so nothing is hidden |
| A20 | 0001:924–928 | OQ-0001-14 options (a) the drafts' reading, (b) trace to §6 or drop; decider the Advisor at the stop | r2:3260–3269 | confirmed | r2:3263–3264 "Decided by: the fixer … or the Advisor (whether a §3 or §4 target is trace enough)"; :3268 "Left for the Advisor: R2-BUD-REC4, R2-BUD-REC5 and R2-TRN-REC6" | condition 12: the trace rule is not a settled input reopened; the research leaves it open |
| A21 | 0001:962–964 | the WebSocket leg first needs OQ-0006-5's development certificate (the Advisor, before the test; PM-0006-15) | ADR 0006 | confirmed | 0006:667–668 "the development and staging case before the netcode proof test starts (the Advisor, from a short read)"; 0006:882 PM-0006-15 "20 of 20 WebSocket sessions over TCP (`wss`) … presents a certificate the four browsers accept" | — |
| A22 | 0001:983 | spike item 3: both shares within 4.17 ms | as A4 | confirmed (arithmetic) | — | — |
| A23 | 0001:1002–1003 | the proof test decides whether each candidate crate runs on a custom quinn endpoint so the simulator's second layer sits under it | R5-TEST-OQ2 | confirmed | r5:1804–1805 "Can wtransport or web-transport-quinn be built on a custom quinn endpoint, so L2 sits under them? … Decides it: R2's interop spike." | fixes consistency F14 |
| A24 | 0001:1016–1017 | order note: PM-0001-57 after PM-0001-17 | the table | confirmed | 0001:1129 PM-0001-17, :1130 PM-0001-57 | SPIKE_PLAN:74, :97 still say "PM-0001-1 to -56": finding M2 |
| A25 | 0001:1090 | the one frame-based tolerance, one tick = 33.33 ms, is two refresh periods | arithmetic | confirmed | 2 × (1000 ÷ 60) = 33.33 | — |
| A26 | 0001:1116 | PM-0001-4: p99 ≤ 16.67, max < 33.33; "the server tick time measured directly on the tick thread" | R2-BUD-REC6; GLOSSARY | confirmed | as A5; GLOSSARY's retired exact "whole tick" gone (script: 0 exact hits in 0001) | fixes verifier F1 / consistency F5 |
| A27 | 0001:1117 | PM-0001-5: both shares ≤ 8.33 ms | R2-BUD-REC6 | confirmed (arithmetic) | as A5 | — |
| A28 | 0001:1130 | PM-0001-57: positions ≤ 1 cm, yaw ≤ 0.01°, pitch ≤ one 0.18° step | R2-BUD-REC2 line 2341; line 1748; line 2263 | confirmed | r2:2341 "decoded positions within 1 cm and angles within 0.01° of the server's values"; r2:1748 "Aim pitch … 10 bits (0.18° steps)"; r2:2263 "A 16-bit angle step (F19: 65,536 steps per turn) is about 0.0055°" | the pitch mark is labelled proposed with its reason (rule 2 holds); a rounding quantiser errs by half a step (0.09°), so one step also passes a truncating one (note N-c) |
| A29 | 0001:1130 | PM-0001-57 bites: 4 cm positions (errors up to 2 cm), yaw in 12 bits (0.088°), pitch in 8 bits (0.70°) | arithmetic | confirmed (arithmetic) | 4 ÷ 2 = 2 cm > 1 cm; 360 ÷ 4096 = 0.088°, half-step 0.044° > 0.01°; 180 ÷ 256 = 0.70° > 0.18° (0.18 × 4 = 0.72 from the rounded step) | each bite fails its field's mark, as a bite must |
| A30 | 0001:1130 | PM-0001-57: E3's widths make 1 cm and the 16-bit yaw reachable | r2:2326, 1745–1747 | confirmed | r2:2326 "quantize (1 cm positions in 16/16/12 bits, 16-bit yaw, 10-bit pitch)"; r2:1745 "16 bits per axis at 1 cm" | — |
| A31 | 0001:1154 | PM-0001-39: shares ≤ 4.17 ms each, max < 16.67 ms | R2-BUD-REC6; R1-TICK-REC2 | confirmed (arithmetic) | 1000 ÷ 60 = 16.67; × 0.25 = 4.17; "the server tick time measured directly, never summed from its phases" (rule 5 fixed) | — |
| A32 | 0001:1155 | PM-0001-40: P1 at 60 Hz ≤ 4.17 ms, max ≤ 16.67 ms; P2 at 60 Hz ≤ 6.67 ms | r1:2044–2060 | confirmed (arithmetic) | r1:2048–2049 "at most 4.2 ms (… 25 % of 16.7 ms) … max at most 16.7 ms"; r1:2056 "at most 40 % of the tick period, 6.7 ms at 60 Hz"; 0.4 × 16.667 = 6.67 | two-decimal restatement (consistency F3); the stricter side of R1's rounding by 0.03 ms |
| A33 | 0001:1159, 1166 | PM-0001-43 and PM-0001-50: one tick / one snapshot interval = 33.33 ms | arithmetic | confirmed | 1000 ÷ 30 = 33.33 | part 8's prose (0001:273) keeps "33.3 ms at 30 Hz, data", allowed for prose by consistency F3 |
| A34 | 0001:1182–1183 | "by this record's own Rollback section" | 0001 Rollback | confirmed | 0001:1323–1324 "## Rollback … The 30 Hz baseline fails at N = 48: back to the drafts with the measured numbers" | the same paragraph's "PM-0001-4 to PM-0001-27" leaves out PM-0001-57, which falls to "Every other failure goes back to the drafts" (part 6): right handling, note N-d |
| A35 | 0001:1309–1310 | load and soak: a full room at each mode's N, every mode on the test-mode list, 2 to 64 players | GLOSSARY.md:375–388 | confirmed | GLOSSARY.md:378 "24v24 (N = 48) and 12v12v12 (N = 36)"; :383 "the headroom run, N = 64"; :386 "items 3 and 4 are 1v1, 4v4, 2v2v2 and 6v6v6" (1v1 = 2 players) | fixes verifier F6 |
| A36 | 0001:1375 | R2-BUD-REC1's Shabzon mark (within 1 % of 13.9 shots/s at the chosen tick, r2:2324–2325) is a sim mark, carried by ADR 0004's S4 | r2:2324–2325; 0004 S4 | confirmed, one word short | r2:2324–2325 "mean delivered fire rate of the Shabzon within 1 % of 13.9 shots/s at the chosen tick"; 0004:825 S4 "which also meets R1-ABL-REC3's 1 % for the Shabzon (1 shot in 833 is 0.12 %: arithmetic)"; 60 ÷ 0.072 = 833.3 | R2's mark sits under a network profile (r2:2320–2321: 100 ms RTT, ±10 ms jitter, 2 % loss); S4 runs in the sim only. The network half is bounded by PM-0001-20 (≤ 0.1 % missing inputs at 5 % loss) but the row does not say so (note N-b) |
| A37 | 0001:1376 | R2-BUD-REC2 changed: precision mark cannot pass for 10-bit pitch; restated (1 cm, 0.01° yaw, one 0.18° pitch step) | r2:2341, 1748, 2263 | confirmed | as A28 | fixes verifier F4 (a) |
| A38 | 0001:1453 | R2-XU-OQ18 open: OQ-0001-14; r2:3268 leaves it to the Advisor | r2:3268 | confirmed | r2:3268 "Left for the Advisor: R2-BUD-REC4, R2-BUD-REC5 and R2-TRN-REC6 (whether the §4 fairness target is trace enough)" | fixes verifier F5 |
| A39 | 0001:1498 | Sources: STATUS "Decisions pending" without line numbers | consistency F10 | confirmed | STATUS.md:93 "## Decisions pending" | — |

### ADR 0003 (docs/adr/0003-rendering.md)
| # | doc:line | claim (short) | cited | verdict | where and the words | note |
|---|---|---|---|---|---|---|
| B1 | 0003:5 | author line credits test-engineer for the Pass marks table | consistency F14 | confirmed | the critic's form, word for word | — |
| B2 | 0003:18–20 | the map's codes (I-n, gap Gn, LE-x, J-n, N-n) are kept in the README's codes section | README | confirmed | README.md:260 "## Codes the drafts cite"; every code 0003 uses has a row there (I-2, I-3, I-8, I-9, I-14, I-15, I-17 to I-21, G2–G5, G7–G9, LE-B, J4, N12, N41, N48: script, 1 row each) | 0003:434's "decision map part h" is not one of the listed families (unchanged text; its G4 has a row) |
| B3 | 0003:47 | carry-forward 3: the 48 value holds | STATUS (code only) | confirmed | consistency F10's outcome (codes alone) | — |
| B4 | 0003:48 | SLICE-ORDER and its conditions (brief §1) | code | confirmed | the code is named; "brief §1" beside it (condition 12) | — |
| B5 | 0003:52–53 | SEC-SPREAD (a): "tracers use a similar-looking spread that the hit corrects" | code | confirmed | STATUS.md:151–152 "the spread stays known only to the server; tracers use a similar-looking spread that the hit corrects" | — |
| B6 | 0003:54 | TICK-30: a 30 Hz server tick is the baseline | code | confirmed | STATUS.md:161–162 | — |
| B7 | 0003:58–59 | UPGRADE-SCOPE recorded in STATUS "Decisions pending"; interface I-21 | STATUS; README | confirmed | STATUS.md:96 "UPGRADE-SCOPE, a settled input for ADR 0005"; README.md:288 I-21 | — |
| B8 | 0003:60–61 | SPIKE-SCOPE (a), R7-MARKS (a) | codes | confirmed | STATUS.md:235–236, :207 | — |
| B9 | 0003:142 | the old ADR-003 entry ("stay on r128 vs upgrade", STATUS, ADR-003) | STATUS | confirmed | STATUS.md:252 "**ADR-003 Three.js version** (stay on r128 vs upgrade)" | — |
| B10 | 0003:190 | the screen-space pass "can mark folds inside a shape (inferred from R3-LOOK-F7; OQ-0003-2)" | R3-LOOK-F7 | confirmed (labelled inferred) | r3:360–363 F7: a depth-and-normal edge filter "is a custom TSL node"; F7 itself is "inferred"; no folds in it, so the label is right | fixes skeptic-2 row 73 |
| B11 | 0003:226–227 | the cutaway also fades between the camera and the player, as legacy GW17 does; LOOK_V2 does not name it | INV:205; LOOK_V2 §4 | confirmed | LEGACY_INVENTORY.md:205 "Any structure between the camera and the player fades to 24 % opacity"; LOOK_V2.md:41 only "Roofs fade when the player is inside" | fixes X5 (rule 6) by routing it to OQ-0003-15 |
| B12 | 0003:229–230 | the cutaway quote at r2:2451–2452 | R2-INT-REC2 | confirmed | r2:2451–2452 "The client's cutaway (GW17) follows the same rule, so what is not sent is also what the screen would not show." | fixes rows 95, X4 |
| B13 | 0003:238–240 | planks as typed links to upper floors, debris as the ruin's walkable surfaces (ADR 0004 part 14); the link type list is data | 0004 part 14; R4-FLOORS-REC1 | confirmed | 0004:380–381 "links (planks as typed links to upper floors, debris as the ruin's walkable surfaces)"; r4:906–907 "the type list is data, open to later ones" | fixes row 100 |
| B14 | 0003:247–248 | fragment damage is a question in README "Decisions the drafts ask for": fragments as damage | README | unsupported (dangling pointer) | README.md (57c43c2 = HEAD): "Decisions the drafts ask for" (lines 25–104) has no fragment row; the only "fragment" in README is gap G5 (line 306) | finding M1 (also B18, B38) |
| B15 | 0003:280–281 | `instanceColor` on `InstancedMesh` (R3-CHAR-F4, 452–455); `BatchedMesh` also carries per-instance colour (R3-CHAR-F6, 463–467) | r3 | confirmed | r3:454 "it carries per-instance colour (`instanceColor`)"; r3:465 F6 "has per-instance colour" | fixes skeptic-3 G10 for 0003 (BatchedMesh's colour no longer called `instanceColor`) |
| B16 | 0003:400 | "(ASSET-RESEARCH (a), the advisor's ruling; J4)" | README J4 | confirmed | README.md:335 J4 | — |
| B17 | 0003:401–402 | beside the authored exports run ADR 0005's ported kit builders, for the families OQ-0005-AP2 asks Mustafa to judge | ADR 0005 | confirmed | 0005@57c43c2:1497–1503 AP2 "Which asset families become authored models, and which stay procedural builders? … (b) buildings and props from the ported kit … Decider: Mustafa, in the look proof test"; 0005:2137, 2139 checklist "as the ported kit" | fixes consistency F6 |
| B18 | 0003:432 | trace row: damage per fragment is a question in README "Decisions the drafts ask for" | README | unsupported (dangling pointer) | as B14 | finding M1 |
| B19 | 0003:489–490 | "WebGPU required, no fallback" appears only as settled by the stack; not weighed | brief §0 line 8; R3-GL2-OQ5 | confirmed | V2_PHASE0_BRIEF.md:8 "the **WebGPU** renderer (WebGL 2 fallback)"; r3:1317–1320 "'WebGPU required, no fallback' as a reference point … Not researched" | fixes X6 (condition 12); README.md:63 still carries the old question: finding M2 |
| B20 | 0003:516 | Q5 cell: "can mark folds inside a shape (inferred: OQ-0003-2)" | — | confirmed (labelled) | as B10 | — |
| B21 | 0003:517–519 | own TSL hull: per-object weight is a plain thickness value, as CL6 (characters 0.8×); the shipped pass takes one thickness for all objects (F5) | CL6; R3-LOOK-F5 | confirmed | r3:421 "legacy's hull is in world units (0.09, characters 0.8×)"; INV:115 "Character parts get 0.8x the width"; r3:345–347 F5 "pushed out by `norm.mul( this.thicknessNode ).mul( pos.w )` … no per-vertex width"; one pass-wide `thicknessNode` (skeptic-2 at source, r186 l.58, l.141) | fixes X3 |
| B22 | 0003:577 | M-D cost: rented; GitHub's GPU runners bill per minute (R3-MEASURE-F9, 655–658); R6's cost | r3:655–658, 846 | confirmed | r3:655 "GPU-powered runners … 'billed at the per-minute rate'"; r3:846 "a rented GPU … R6's cost" | fixes X2 |
| B23 | 0003:632–634 | trade-off: the hull is legacy's known look (own TSL hull, plain per-object weight for buildings, CL6) or shipped code (one thickness, F5) | CL6; F5 | confirmed (one nuance) | as B21 | CL6 weights characters (0.8×), not buildings; "for buildings" is LOOK_V2 §2's ask met by the same mechanism (skeptic-2's own text). Not a finding |
| B24 | 0003:667 | Revisit when: the fallback share "may allow F-B extras" (no reopening of the fallback) | R3-GL2-OQ2 | confirmed | r3:1308–1311 | fixes X6 |
| B25 | 0003:707 | GW17 row: the roof fade and the camera-player cutaway (OQ-0003-15) …; "pending removed" | OQ-0003-15 | stale (new inconsistency) | the other OQ-0003-15 items keep a pending note: 0003:703 CL12/GW3 "pending becomes 'Mustafa's call: …'", :705 GA14, :714 GP9; GW17's camera-player half is now Mustafa's open call too | finding L2 |
| B26 | 0003:712–713 | HUD rows split: GP7, GP10, GP12, PG3, PG7, PG8, PG10, PG14, PG16 port; PG2, PG4–PG6, PG11, PG12, PG15, PG20 redesign | INV lines | confirmed | script over INV:317, 320, 322, 360–366, 368–370, 372–374, 378: the 9 are "**port to TypeScript**", the 8 "**redesign**"; all 17 carry "pending" | fixes skeptic-2 row 211 |
| B27 | 0003:730–731 | OQ-0003-2: per-object thickness in our own TSL hull (CL6); the shipped pass has one thickness for all objects (F5) | CL6; F5 | confirmed | as B21 | — |
| B28 | 0003:784–790 | OQ-0003-11 "How many players use the WebGL 2 fallback?": the fallback keeps Firefox on Linux and Intel Macs, Chrome on Linux without Gen12+, Safari before 26; its share decides F-B extras; the fallback stays (R3-GL2-OQ5 answered) | R3-GL2-F12, 279–283; R3-GL2-OQ2, 1308–1311 | confirmed | r3:279–281 "Firefox on Linux and on Intel Macs …, Chrome on Linux without an Intel Gen12+ GPU, Safari before 26"; r3:1308–1311 "Decided by: client telemetry of the backend after launch" | fixes X6 |
| B29 | 0003:802–806 | OQ-0003-14 answered from the W3C WD of 22 September 2026: `panningModel` "Defaults to "equalpower"", `distanceModel` "Defaults to "inverse"", equal power "a simple and relatively inexpensive algorithm", HRTF "somewhat more costly than "equalpower"" | www.w3.org/TR/webaudio-1.1/; R3-AUDIO-F2, F3 (696–709) | confirmed at source | fetched 19:29 (fetch audit): title "Web Audio API 1.1 W3C Working Draft, 22 September 2026"; §PannerNode attributes "Defaults to "equalpower"." and "Defaults to "inverse"."; §6.3.1 "This is a simple and relatively inexpensive algorithm"; HRTF "It is somewhat more costly than "equalpower""; r3:696–709 F2, F3 | fixes X8 |
| B30 | 0003:810–812 | OQ-0003-15 adds the camera-player cutaway (GW17, INV line 205) | INV:205 | confirmed | as B11 | — |
| B31 | 0003:838 | the benchmark scene on Safari on MAC (SPIKE_PLAN, shared fixtures) | SPIKE_PLAN | confirmed | SPIKE_PLAN.md:20 "## Shared fixtures", :30 "MAC, a macOS machine with Safari" | consistency F4 |
| B32 | 0003:851–852 | ADR 0005's share runs in the look test: PM-0005-33 to PM-0005-37 and the families checklist for OQ-0005-AP2 | ADR 0005 | confirmed | 0005@57c43c2:2053–2057 (PM-0005-33 to -37, -37 "Mustafa's verdict on each asset family (OQ-0005-AP2; AP-7)") | fixes consistency F6 |
| B33 | 0003:853–855 | Needs: the ported kit builders beside the exports, and a minimal content loader running the content crate's whole-set check (ADR 0005) | ADR 0005; SPIKE_PLAN | confirmed | SPIKE_PLAN.md:231–232 "the content crate's whole-set check (a minimal loader, since the content and ability spike comes later); for OQ-0005-AP2, the ported kit builders"; 0005@57c43c2:1514–1516 (load-time failures) | — |
| B34 | 0003:916–919 | MAC: stable Safari on the day, version recorded; WebGPU from 26.0 (R3-WGPU-F19, 171–174); WebTransport from 26.4 (R2-TRN-F7, r2 959–975) | r3; r2 | confirmed | r3:171 "WebGPU 'is now shipping in Safari 26.0 for macOS …'"; r2:959–967 F7 table "Safari (macOS …) \| 26.4 \| 2026-03-24" | SPIKE_PLAN:30 has not taken the version rule (consistency F4's synthesis half): note N-e |
| B35 | 0003:974, 975, 977, 978, 1014 | PM-0003-2, -3, -5, -6, -41: "MAC: Safari" | the key | confirmed | consistent with B34 | — |
| B36 | 0003:976 | PM-0003-4: "median CPU frame time" | R3-WGPU-REC1 mark 3 | confirmed | r3:914–915 "the WebGPU backend's median CPU frame time is no worse than 1.1 × the WebGL 2 backend's" | fixes TE44 |
| B37 | 0003:984 | PM-0003-12: count is the whole frame's, post passes included; R3's budget is "all passes of one frame"; `drawCalls` "The number of draw calls of the current frame."; margins 14 and 42; bloom takes 12 | r3:1121; R3-MEASURE-F3 (617–618); R3-LOOK-F13 (397–402) | confirmed | r3:1121 "Budget (all passes of one frame)"; r3:617–618 "(\"The number of draw calls of the current frame.\")"; r3:400 "twelve full-screen passes per frame", r3:401–402 "the pass count of twelve is inferred"; 800 − 786 = 14, 1,100 − 1,058 = 42; r3:1142–1146's sums hold soldiers, world and very large units only | fixes X7; "if the look brief keeps it" decides nothing (rule 6 holds) |
| B38 | 0003:1097–1098 | checklist step 24: fragment damage is a design question for Mustafa with the game-designer (README "Decisions the drafts ask for": fragments as damage) | README | unsupported (dangling pointer) | as B14 | finding M1 |
| B39 | 0003:1108–1110 | checklist step 29 adds the camera-player cutaway; walk the player behind a building | OQ-0003-15 | confirmed | INV:205 | fixes completeness item 6 |
| B40 | 0003:1166 | backend matrix: Safari on MAC (SPIKE_PLAN) | SPIKE_PLAN | confirmed | as B31 | — |
| B41 | 0003:1203, 1205, 1235, 1238, 1260 | Appendix A: R2-XT-OQ15 lines 2451–2452; R3-WGPU-REC2 disposes of the ADR-003 entry in STATUS; R3-GL2-OQ2 open; R3-GL2-OQ5 answered, settled by the stack; R3-AUDIO-OQ4 answered | r2; STATUS; r3; brief | confirmed | as B12, B9, B28, B19, B29 | — |
| B42 | 0003:1271 | counts: adopted 23 · changed 2 · rejected 0 · answered 17 · open 10 · spike 19 (71) | Appendix A | confirmed (script) | rows counted at 57c43c2: 23, 2, 0, 17, 10, 19 = 71 | — |
| B43 | 0003:1291–1301 | Sources: GLOSSARY §8 (the foundation, the netcode proof test); STATUS "Decisions pending" by code; README codes and decisions; ASSET-RESEARCH (a) as STATUS records it | STATUS; README | confirmed | STATUS.md:93 section, :119 ASSET-RESEARCH (a); README.md:260 | README's "Decisions the drafts ask for" lacks the fragments row (M1) |

### ADR 0006 (docs/adr/0006-infrastructure-operations.md)
| # | doc:line | claim (short) | cited | verdict | where and the words | note |
|---|---|---|---|---|---|---|
| C1 | 0006:77–78 | Sentry's storage location: "once selected, your data storage location can’t be changed" | R6-OBS-F9, r6:318–331 | confirmed | r6:321–322 "once selected, your data storage location can’t be changed." | fixes finding 8 (quote) |
| C2 | 0006:83–84 | first use of the header sets linked | GLOSSARY | confirmed | GLOSSARY.md:652 "### The production header set and the preview header set" (the anchor resolves) | fixes finding 12 |
| C3 | 0006:147–149 | the foundation's "private preview links" read through the access wall: "previews that reach no game server and carry no unreleased art are public but unlisted" | GLOSSARY.md:623–624 | confirmed | GLOSSARY.md:623–624 "Under every option, previews that reach no game server and carry no unreleased art are public but unlisted"; GLOSSARY §8 (:753) "CI gate v2, and private preview links" | fixes finding 13 |
| C4 | 0006:155–156 | the access policy covers previews, not production: R6-HOST-F4 inferred, Cloudflare's page states it | r6:80–86; Cloudflare page | confirmed at source | r6:86 "(… it protects preview deployments only, not the production `*.pages.dev` or custom domains) is inferred (summary)"; fetched 2026-11-02 09:16: "Note that this will only protect your preview deployments (for example, 373f31e2.user-example.pages.dev and every other randomly generated preview link) and not your *.pages.dev domain or custom domain." | fixes finding 11 |
| C5 | 0006:186–188 | the metrics rule is a proposed change to R6-HOST-REC2's "plus nothing else" (r6 line 649): a scrape needs an inbound port, a push none (R6-OBS-F4, 279–284) | r6:649; r6:279–284 | confirmed | r6:649 "plus nothing else"; r6:279–281 F4 "exposes … on an HTTP endpoint for Prometheus to scrape, or pushes them to a push gateway" | "a push needs none" is the draft's reading (outbound); fixes finding 4 |
| C6 | 0006:232–233 | the proof test runs on the development certificates for WebTransport; the WebSocket's certificate is OQ-0006-5 | part 7; OQ-0006-5 | confirmed | r2:2700–2703 (WebTransport pinning only) | fixes finding 7 (part 7 half) |
| C7 | 0006:308–311 | `userInfo` off: no IP inferred; deny lists for headers, cookies and query parameters set too, since IPs can still reach Sentry through them; R6-OBS-F10 "inferred" there; Sentry's page says it | r6:327–331; Sentry page | confirmed at source | r6:331 "the IP address won't be inferred." … "The headers, cookies and query point is inferred (summary)"; fetched 09:16: "Even when this is disabled, IP addresses can still reach Sentry through collected HTTP headers, cookies, or query parameters (for example, the X-Forwarded-For header). Add these terms to the partially-matched deny lists" | fixes finding 1 / skeptic-1 |
| C8 | 0006:311–313 | in the browser the SDK sends only Referer and User-Agent, no cookies unless httpClient is on, so query parameters are the channel left | Sentry page (skeptic-1) | confirmed at source | "In the browser, the SDK sends the request headers it gathers itself ( Referer and User-Agent ). The httpClient integration, which is not enabled by default, adds …"; "In the browser, the SDK doesn't send cookies unless you enable the httpClient integration" | "the channel left" is the draft's reading (query strings, the Referer's included) |
| C9 | 0006:317 | the EU location "can’t be changed" later | R6-OBS-F9 | confirmed | as C1 | — |
| C10 | 0006:392 | the deploy token scope: "select *Account*, *Cloudflare Pages* and *Edit*" | R5-PREV-F10, r5:823–830 | confirmed | r5:827 "for the token, "select *Account*, *Cloudflare Pages* and *Edit*"" | fixes finding 8 |
| C11 | 0006:435 | G1: 1 EUR per extra TB is "an inferred reading" (F15, F16) | r6:150–163 | confirmed | r6:163 "The reading "1.00 EUR per extra TB" is inferred." | fixes finding 9 |
| C12 | 0006:439 | Netcup, OVHcloud, Scaleway read only on a shortage; Hetzner's root servers, "a further candidate at goal scale", read before the goal budget is fixed whatever OQ-0006-1 says | R6-HOST-OQ1, OQ4; r6:852–855, 862–865 | confirmed | r6:854 "if it is a real shortage, Netcup, OVHcloud or Scaleway … need a read"; r6:863–865 "Hetzner's dedicated root servers … are a further candidate at goal scale; decided by a read of their price and location pages before the goal budget (R6-COST-REC1) is fixed" | fixes finding 2 / skeptic-1 |
| C13 | 0006:482 | Q8: IP off by setting plus deny lists | as C7 | confirmed | as C7 | — |
| C14 | 0006:530–531 | the host comparison cites R5-PREV-REC2 (r5:1607–1609) | r5 | confirmed | r5:1607–1608 "Cloudflare Pages meets every need except full cleanup … Cloudflare Workers static assets, Netlify and Vercel meet most" | fixes finding 10 |
| C15 | 0006:547–549 | A-2: "a density to measure at the tick rate R1-TICK-REC2's rule picks (R2-ROOM-REC2 mark 2), not one the server tick budget guarantees"; that rate is now TICK-30's 30 Hz | r6:516–522 | confirmed | r6:518 word for word | "now TICK-30's 30 Hz" names the baseline; 60 Hz stays possible on the 60 Hz margin (note N-f); fixes finding 8 |
| C16 | 0006:617–621 | OQ-0006-1 (b): the shortage fallbacks (Fly.io, GameLift, Netcup, OVHcloud, Scaleway; r6:852–855, 862–863); root servers read whatever the answer (r6:863–865; migration step 1) | r6 | confirmed | as C12 | — |
| C17 | 0006:652–653 | OQ-0006-4: Cloudflare's page confirms F4; branch-alias links not stated | Cloudflare page | confirmed at source | as C4; the page's "Preview aliases" section says only that "the alias always maps to the latest commit of that branch", nothing on the policy | — |
| C18 | 0006:655–672 | OQ-0006-5 widened to the WebSocket fallback's dev/staging certificate; "not researched"; decider the Advisor for that case; carries the verifier's finding 7 | R2-ROOM-F12, r2:1494–1500; R2-TRN-REC5, r2:2700–2703 | confirmed | r2:1494 "The WebSocket constructor takes only a URL and sub-protocols"; r2:2702 "pinned through `serverCertificateHashes`" (WebTransport's option) | no "not researched" option proposed (rule 3 holds); fixes finding 7 |
| C19 | 0006:758–759 | spike 1: F4 confirmed on Cloudflare's page, not for branch-alias links | as C4 | confirmed | as C4, C17 | — |
| C20 | 0006:852–862 | Counts and settings: the reasons for PM-0006-4's 3 repeats, -6's four pull requests and hourly probes, -7's 10 uploads (2 % of 500), -8's 2 branches, -12's 100 echoes (1 % each), -22's quarter on WebSocket and kill at minute 20 | the rows | confirmed (arithmetic and rows) | 0006:870 PM-0006-4 "3 repeats, each from a fresh profile"; :869 PM-0006-3 "3 of 3 loads"; :872 PM-0006-6 (a)–(d), "every hour"; 10 ÷ 500 = 2 %; 1 ÷ 100 = 1 %; :889 PM-0006-22 "30-minute soak", "killed at minute 20" | fixes finding 6 (reasons now in the record, labelled proposed) |
| C21 | 0006:880 | PM-0006-13: 8 rooms at N = 48; mark 1 p99 ≤ 16.67 ms, max < 33.33 ms | R2-ROOM-REC2 mark 1, r2:2774–2780 | confirmed (arithmetic) | r2:2775–2776 "within the server tick budget (… 50 % of the tick period …)"; 0.5 × 33.33 = 16.67 | consistency F3, F13 |
| C22 | 0006:884 | PM-0006-17: proposed in place of R6's fixed 3.16 GB; 700 shots/s almost ten times R2's 72 (1.5 × 48); V = 15 vehicles; 2.52 to 3.79 GB | r6:714–717; r2:1759–1775 | confirmed (arithmetic) | r6:716 "within ±20 % of 3.16 GB"; r2:1759 "1.5 per player per second"; 1.5 × 48 = 72, 700 ÷ 72 = 9.7; r2:1765 "V = vehicles", r2:1775 "48 \| 15"; 3.156 × 0.8 = 2.52, × 1.2 = 3.79 | fixes finding 5 and consistency F13 |
| C23 | 0006:890 | PM-0006-23: one tick period 33.33 ms | arithmetic | confirmed | 1000 ÷ 30 = 33.33 | — |
| C24 | 0006:976–977 | migration step 1: the goal budget fixed only after a read of Hetzner's root servers' price and location pages | r6:863–865 | confirmed | as C12 | — |
| C25 | 0006:1076 | R5-PREV-REC5's mark: "a preview link opens in Brave on Windows without a login" (r5 line 1649) | r5:1649 | confirmed | r5:1649 word for word | fixes finding 8 |
| C26 | 0006:1089 | R6-HOST-REC2 changed: one firewall rule added for part 9's metrics path | r6:649; r6:279–284 | confirmed | as C5 | — |
| C27 | 0006:1104 | R6-HOST-OQ4: fallbacks only on a shortage; root servers before the goal budget, whatever OQ-0006-1's answer | r6:862–865 | confirmed | as C12 | Appendix A has no counts line in 0006, so the adopted → changed move leaves no stale total |

## 2. Did each fix do what its finding asked?
Method: the 80a27b3 copies of the three docs, with the fix files applied in commit order by `scripts/apply_fixes.py`
(scratch `.gate/fixcheck-a/replay/`), are byte-identical to 57c43c2: every changed line comes from one of the 114
blocks below, and nothing else changed. rust-reviewer-fixes-2-held-for-test-engineer.md holds only ADR 0004 blocks
(test-engineer took both over for 0004): none touches these three docs.

| block | finding | resolved? | new problem |
|---|---|---|---|
| netcode r1 F2 (trade-off 5) | verifier 0001 F2; skeptic-1 | yes: the cost is outside the client's sim share | "It is reported beside" names no row; the report is ADR 0002's PM-0002-25 (note N-a) |
| netcode r1 F4 (R2-BUD-REC1, REC2 rows) | verifier F4 (N13); skeptic-1 | yes: precision restated (carried by PM-0001-57), Shabzon mark disposed to S4 | S4 runs without R2's network profile and the row does not say so (note N-b) |
| netcode r1 F5 (Appendix A R2-XU-OQ18) | verifier F5 (N15); skeptic-1 | yes: "open: OQ-0001-14" | none |
| netcode r1 F5 (OQ-0001-14) | verifier F5 | yes | none in 0001 (README names it only as "Trace reading": consistency F9, synthesis) |
| netcode r1 F6 (load and soak) | verifier F6 (N16); skeptic-1 | yes | none |
| netcode r1 F7 (Q10 U1) | verifier F7 (N17) | yes | none |
| netcode r1 F8 ("When a mark fails") | verifier F8 (TE217, N18) | yes | the paragraph is test-engineer's (the verifier named it owner); content right. Its "PM-0001-4 to PM-0001-27" leaves out the later PM-0001-57, which falls to "every other failure" (note N-d) |
| netcode r1 F9 (the restated reason) | verifier F9 (N20) | yes: the words left the quote; the fact is stated beside it with GLOSSARY.md:69, 76–77 | none |
| netcode r1 F10 (Q6) | verifier F10 (rows 272, 276) | yes | none |
| netcode r1 F10 (Q9) | verifier F10 | yes | none |
| netcode r2 F1 (`ContentReload`) | consistency F1 | yes | none |
| netcode r2 F2 (part 12's open rules) | consistency F2 | yes | none |
| netcode r2 F3 (part 2) | consistency F3 | yes | none |
| netcode r2 F3 (part 3) | consistency F3 | yes | none |
| netcode r2 F3 (budgets table) | consistency F3 | yes | none |
| netcode r2 F3 (spike item 3) | consistency F3 | yes | none |
| netcode r2 F8 (part 10) | consistency F8 | yes (0001's half) | SPIKE_PLAN item 1 and README still lack OQ-0006-5 (the critic's synthesis half): M2 |
| netcode r2 F8 (spike item 1) | consistency F8 | yes | as above |
| netcode r2 F10 (Context, Tick) | consistency F10 | yes | none |
| netcode r2 F10 (legacy) | consistency F10 | yes | none |
| netcode r2 F10 (Sources) | consistency F10 | yes | none |
| netcode r2 F13 (event names) | consistency F13 | partly: "captures" gone, 0004's words used | the list, presented as part 19's, omits blasts, harvester deliveries and kills, structure kills and stage changes: L1 |
| netcode r2 F13 (the proof test) | consistency F13 | yes | none |
| netcode r2 F14 (R5-TEST-OQ2) | consistency F14 | yes | none |
| netcode r2 F14 (OQ-0001-1 (b)) | consistency F14 | yes | none |
| TE r1 0001 F1 (PM-0001-4) | verifier F1; consistency F5 | yes (glossary script: 0 exact hits) | none |
| TE r1 0001 F1 (PM-0001-39) | verifier F1; consistency F5 | yes | none |
| TE r2 F3 (0001 key) | consistency F3 | yes | none |
| TE r2 F3 (PM-0001-4) | consistency F3 | yes | none |
| TE r2 F3 (PM-0001-5) | consistency F3 | yes | none |
| TE r2 F3 (PM-0001-39) | consistency F3 | yes | none |
| TE r2 F3 (PM-0001-40) | consistency F3 | yes | none |
| TE r2 F3 (PM-0001-43) | consistency F3 | yes | none |
| TE r2 F3 (PM-0001-50) | consistency F3 | yes | none |
| TE r2 R2-BUD-REC2 (order note) | verifier F4 (a) | yes | SPIKE_PLAN:74, :97 "PM-0001-1 to -56": M2 |
| TE r2 R2-BUD-REC2 (PM-0001-57) | verifier F4 (a); skeptic-1 | yes: the hash does not bound quantising error, the new row does | none (the pitch bound is one step, note N-c) |
| TE r1 0003 TE44 (PM-0003-4) | verifier 0003 finding 9 | yes | none |
| TE r1 0003 X7 (PM-0003-12) | verifier 0003 finding 10 | yes | none |
| GE r2 F13 (author line) | consistency F14 (author lines) | yes | none |
| GE r2 F12 (the map's codes) | consistency F12 | yes: every code 0003 uses has a README row | none |
| GE r2 F10 (carry-forward 3) | consistency F10 | yes | none |
| GE r2 F10 (SLICE-ORDER) | consistency F10 | yes | none |
| GE r2 F10 (SEC-SPREAD) | consistency F10 | yes | none |
| GE r2 F10 (TICK-30) | consistency F10 | yes | none |
| GE r2 F10 (UPGRADE-SCOPE) | consistency F10 | yes | none |
| GE r2 F10 (SPIKE-SCOPE) | consistency F10 | yes | none |
| GE r2 F10 (R7-MARKS) | consistency F10 | yes | none |
| GE r2 F10 (ADR-003 entry, part 2) | consistency F10 | yes | none |
| GE r2 row 73 (5.b) | verifier 0003 finding 5; skeptic-2 | yes | none |
| GE r2 X5, rows 95, X4 (5.i) | findings 4, 7; skeptic-2 | yes | none |
| GE r2 row 100 (5.j) | finding 6; skeptic-2 (debris nit) | yes | none |
| GE r2 F13 (fragments, 5.k) | consistency F14 (fragment damage) | partly | points to a README row that does not exist: M1 |
| GE r2 G10 (part 6) | skeptic-3 G10 | yes | none |
| GE r2 F6 (part 11) | consistency F6 | yes | none |
| GE r2 F13 (fragments, trace row) | consistency F14 | partly | M1 |
| GE r2 X6 (Q3 bullet) | finding 3; skeptic-2 | yes in 0003 | README:63 keeps the old question: M2 |
| GE r2 row 73 (Q5 cell) | finding 5 | yes | none |
| GE r2 X3 (Q5 hulls) | finding 2; skeptic-2 | yes | none |
| GE r2 X2 (M-D cost) | finding 8; skeptic-2 | yes | none |
| GE r2 X3 (trade-off) | finding 2 | yes | none (CL6 weights characters; "for buildings" is the same mechanism, skeptic-2's text) |
| GE r2 X6 (revisit when) | finding 3 | yes | none |
| GE r2 X5 (GW17 row) | finding 4; skeptic-2 | partly | "pending removed" although the camera-player half is now Mustafa's open call: L2 |
| GE r2 row 211 (HUD split) | finding 1; skeptic-2 | yes (script over INV) | none |
| GE r2 X3 (OQ-0003-2) | finding 2 | yes | none |
| GE r2 X6 (OQ-0003-11) | finding 3 | yes | README:63: M2 |
| GE r2 X8 (OQ-0003-14) | finding 11 | yes, confirmed at source today | none |
| GE r2 X5 (OQ-0003-15) | finding 4 | yes | none |
| GE r2 F4 (look test: MAC) | consistency F4 | yes | none |
| GE r2 F6 (look test: ADR 0005's share) | consistency F6 | yes | none |
| GE r2 F4 (test plan: MAC) | consistency F4 | yes | none |
| GE r2 X4 (Appendix R2-XT-OQ15) | finding 7 | yes | none |
| GE r2 F10 (Appendix R3-WGPU-REC2) | consistency F10 | yes | none |
| GE r2 X6 (Appendix R3-GL2-OQ2) | finding 3 | yes | none |
| GE r2 X6 (Appendix R3-GL2-OQ5) | finding 3 | yes | none |
| GE r2 X8 (Appendix R3-AUDIO-OQ4) | finding 11 | yes | none |
| GE r2 X6 and X8 (counts) | — | yes (recounted: 71) | none |
| GE r2 F12 (Sources: GLOSSARY §8) | consistency F12 | yes | none |
| GE r2 F10 (Sources: STATUS) | consistency F10 | yes | none |
| GE r2 F12 (Sources: the map) | consistency F12 | yes | none |
| TE r2 F4 (0003 key, MAC) | consistency F4 | yes | SPIKE_PLAN:30 itself has not taken the version rule (note N-e) |
| TE r2 F4 (PM-0003-2) | consistency F4 | yes | none |
| TE r2 F4 (PM-0003-3) | consistency F4 | yes | none |
| TE r2 F4 (PM-0003-5) | consistency F4 | yes | none |
| TE r2 F4 (PM-0003-6) | consistency F4 | yes | none |
| TE r2 F4 (PM-0003-41) | consistency F4 | yes | none |
| TE r2 F14 (checklist step 24) | consistency F14 | partly | M1 |
| TE r2 X5 (checklist step 29) | completeness item 6; skeptic-2 X5 | yes | none |
| infra r1 1 (part 10) | verifier 0006 finding 1; skeptic-1 | yes, confirmed at source today | none |
| infra r1 1 (Q8) | finding 1 | yes | none |
| infra r1 2 (Q3) | finding 2; skeptic-1 | yes | none |
| infra r1 2 (OQ-0006-1) | finding 2 | yes | none |
| infra r1 2 (migration step 1) | finding 2 | yes | none |
| infra r1 2 (Appendix R6-HOST-OQ4) | finding 2 | yes | none |
| infra r1 4 (part 5) | finding 4 | yes | none |
| infra r1 4 (Appendix R6-HOST-REC2) | finding 4 | yes ("changed") | none |
| infra r1 7 (part 7) | finding 7 | yes | none |
| infra r1 7 (OQ-0006-5) | finding 7 | yes | README has no OQ-0006-5 row although its dev/staging case now goes to the Advisor before the proof test: M2 |
| infra r1 8 (Context) | finding 8 | yes | none |
| infra r1 8 (part 10, apostrophe) | finding 8 | yes | none |
| infra r1 8 (trade-off, A-2) | finding 8 | yes | none (note N-f) |
| infra r1 8 (part 13, token scope) | finding 8 | yes | none |
| infra r1 8 (Appendix R5-PREV-REC5) | finding 8 | yes | none |
| infra r1 9 (G1) | finding 9 | yes | none |
| infra r1 10 (trade-off) | finding 10 | yes | none |
| infra r1 11 (part 2) | finding 11 | yes, confirmed at source today | none |
| infra r1 11 (OQ-0006-4) | finding 11 | yes | none |
| infra r1 11 (spike 1) | finding 11 | yes | none |
| infra r1 12 (Context link) | finding 12 | yes | none |
| infra r1 13 (part 2) | finding 13 | yes | none |
| TE r1 0006 5 (PM-0006-17) | finding 5 (TE140) | yes | adds the short form "the proof test's load" outside ADR 0001: L3 |
| TE r1 0006 6 (counts and settings) | finding 6 | yes | none |
| TE r2 F3 and F13 (PM-0006-13) | consistency F3, F13 | yes | none |
| TE r2 F3 (PM-0006-23) | consistency F3 | yes | none |
| TE r2 F13 (PM-0006-17) | consistency F13 | yes ("V = 15 vehicles", R2's own word) | the short forms in the same row stay (L3) |

Findings the range leaves open on these docs (not blocks, for the record): verifier 0001 F3 and 0006 N1 (cut markers:
the session's assembly, outside these blocks; `grep -c '8<' ` on both docs at 57c43c2 is 0, so they are gone too);
consistency F13's short "proof test" at 0006:550 and 0006:1040 (infra-engineer's round 3, after 57c43c2).

## 3. The rules on the changed text
- Rule 2 (a number with no source): none. Every new number is research (r2:2341, 1748, 2263; r6:518, 863–865), arithmetic
  shown (16.67, 33.33, 8.33, 4.17, 6.67; 14 and 42 draws; 2 %, 1 %; 2.52–3.79 GB; 0.088°, 0.70°), or labelled proposed
  with a reason (PM-0001-57's pitch step; 0006's counts and settings; PM-0006-17's reference; the metrics rule).
- Rule 3 (a "not researched" option proposed): none. OQ-0006-5's WebSocket case is "not researched" and left to the
  Advisor; OQ-0003-2's MRT weight stays an option, not the proposal.
- Rule 4 (a feature with no trace): none new. The metrics firewall rule serves part 9 (monitoring); OQ-0001-14 asks the
  trace question itself and changes no feature.
- Rule 5: glossary script over the drafts (working tree; 0001, 0003, 0006 equal 57c43c2 apart from later rounds):
  0 exact hits. "headroom" on changed lines: only the TICK-30 quote (0001:25) and the code TICK-60-HEADROOM (0001:536,
  1154). Term use: L3 (the short "proof test" in 0006's PM-0006-17).
- Rule 6 (the look): nothing decided. The camera-player cutaway goes to Mustafa (OQ-0003-15, step 29); "if the look
  brief keeps it" (PM-0003-12) and AP2's families stay his.
- Condition 12 (a settled input reopened): fixed in 0003 (WebGPU required is "settled by the stack", not weighed);
  nothing new reopened. README:63 still asks the old question (M2, outside these docs).

## 4. The citation script's line-range flags
`python3 scripts/cite_check.py docs/research <0001 at 57c43c2> <0006 at 57c43c2>` (run on the snapshots, since the
working tree has later rounds): 1,217 ids cited, 0 missing; 21 range flags (0001: 3, 0006: 18). All 21 are script
artifacts (a group with two ranges or two files, read against one; or an id more than 20 lines above its own mark):

| # | doc:line | flag | verdict | where the cited lines and the id are |
|---|---|---|---|---|
| 1 | 0001:52–54 | R2-BUD-REC1 not within r2 1770–1790 | script artifact | R2-BUD-REC1 is the group's second range, r2:2302–2325 (r2:2302 its heading); 1770–1790 hold the estimate's results (146.1 kbps r2:1775, 286.1 kbps r2:1778, Fiedler's 256 kbit/s r2:1789) |
| 2 | 0001:62–64 | R2-TRN-REC3 not within r2 2565–2615 | script artifact | R2-TRN-REC3 is at its own range, r2:2639 |
| 3 | 0001:62–64 | R2-TRN-REC1 not within r2 2639–2671 | script artifact | R2-TRN-REC1 is at its own range, r2:2565 |
| 4 | 0006:66 | R5-PREV-F12 not within r6 60–65 | script artifact | R5-PREV-F12 is r5's (r5:838); r6:60 R6-HOST-F1, r6:72 F3 |
| 5 | 0006:66 | R6-HOST-F1 not within 838–849 (read in r6) | script artifact | 838–849 are r5's: R5-PREV-F12 r5:838, F13 r5:845; R6-HOST-F1 at r6:60 |
| 6 | 0006:66 | R5-PREV-F12 not within r6 838–849 | script artifact | as 5: r5:838 |
| 7 | 0006:136–138 | r6 lines 1605–1615 out of range | script artifact | the range is r5's: R5-PREV-REC2, r5:1605–1615 |
| 8 | 0006:141 | R6-HOST-F5 not within r6 476 | script artifact | r6:476 is the start scenario's Pages row; R6-HOST-F5 at its own range, r6:87 |
| 9 | 0006:222–223 | r6 lines 2699–2723 out of range | script artifact | r2's: R2-TRN-REC5, r2:2699 |
| 10 | 0006:231 | r6 lines 3050–3054 out of range | script artifact | r2's: R2-TRN-OQ4, r2:3050 |
| 11 | 0006:269–270 | R6-COST-REC1 not within r6 499–500 | script artifact | 499–500 are the "Scaling rule (arithmetic)"; R6-COST-REC1 at its own range, r6:703 |
| 12 | 0006:275–276 | R5-CI-F17 not within r5 913–914 | script artifact | r5:913 "Repository visibility. Public"; R5-CI-F17 at r5:673, F5 at r5:583 |
| 13 | 0006:391 | r6 lines 2104–2106 out of range | script artifact | r5's: r5:2104 "For Mustafa (settings he owns, never changed by agents)" (under r5:2091 "From R5-CI and R5-PREV"); R6-SEC-OQ1 r6:926–945 |
| 14 | 0006:450 | R6-HOST-OQ6 not within r6 420–427 | script artifact | r6:420 "Q3. Production certificates"; R6-HOST-OQ6 at its own range, r6:869 |
| 15 | 0006:530 | R6-HOST-REC1 not within 627–634 (read in r5) | script artifact | "r6 lines 627–634" (the script does not read the short "r6"); R6-HOST-REC1 runs from r6:619, so 627–634 are inside it. This group is changed text: confirmed (C14) |
| 16 | 0006:530 | R5-PREV-REC2 not within 627–634 (read in r5) | script artifact | its own range is r5:1607–1609 (R5-PREV-REC2 from r5:1605) |
| 17 | 0006:677 | r6 lines 2955–2961 out of range | script artifact | r1's: R1-RR-OQ7, r1:2955–2961; R5-GATE-OQ4 r5:1873 |
| 18 | 0006:753 | r6 lines 1622–1624 out of range | script artifact | r5's: R5-PREV-REC3 (r5:1616), its mark at r5:1622–1624; PREV-DELETE r6:970 |
| 19 | 0006:769 | R6-HOST-REC2 not within r6 667–671 | script artifact | r6:667 "*Spike must prove (proposed):* R2-ROOM-REC2's marks on both CCX23 and CX33" is REC2's mark; its heading r6:645 is 22 lines up |
| 20 | 0006:769 | r6 lines 2775–2782 out of range | script artifact | r2's: R2-ROOM-REC2's marks, r2:2775–2782 (mark 2 at r2:2777) |
| 21 | 0006:792–793 | R6-OBS-OQ3 not within r6 765–768 | script artifact | r6:765–768 are R6-OBS-REC3's mark (a Wasm trap in Sentry with Rust names); OQ3 is named as what the mark answers, not as located there |

## 5. Counts
- Changed-text rows: 109 (0001: 39, 0003: 43, 0006: 27). Confirmed 104 (6 of them also at the original source today:
  B29, C4, C7, C8, C17, C19) · wrong 0 · unsupported 4 (A8; B14, B18, B38, one defect in three places) · citation off 0 ·
  stale 1 (B25) · rule 1 (L3, rule 5's term use, on the changed line of C22).
- Fix blocks: 114 (0001: 36, 0003: 51, of them 41 graphics-engineer's, 0006: 27): resolved 109, partly 5 (netcode r2 F13 events; GE r2 F13 ×2 and
  TE r2 F14, the README pointer; GE r2 X5 GW17 row); with a new problem 1 (TE r1 0006 fix 5, L3), plus 2 that leave
  synthesis work undone (M2). Replay: byte-identical to 57c43c2.
- Citation flags: 21 (0001: 3, 0006: 18): real 0, script artifacts 21.
- Findings: 2 medium (M1, M2), 3 low (L1–L3); notes N-a to N-f.

## 6. Findings (ranked)
**M1 (unsupported: a dangling pointer). ADR 0003 cites a README row that does not exist.** 0003:247–248 (5.k):
"Whether a fragment deals damage is a design question for Mustafa (`docs/adr/README.md`, "Decisions the drafts ask
for": fragments as damage)"; 0003:432 (trace row): "damage per fragment is a design question (`docs/adr/README.md`,
"Decisions the drafts ask for")"; 0003:1097–1098 (checklist step 24): "a design question for Mustafa with the
game-designer (`docs/adr/README.md`, "Decisions the drafts ask for": fragments as damage)". README at 57c43c2 (and in
the working tree at 213dfb6): "Decisions the drafts ask for" (README.md:25–104) has no fragment row; the only
"fragment" is gap G5 (README.md:306). Consistency F14's outcome gave the row to the synthesis. Smallest fix: README adds
"| Fragments as damage | Does each grenade fragment deal damage, or only the blast? (LOOK_V2 §3, §7 bring fragments
back as an effect) | Mustafa, with the game-designer | ADR 0003 part 5.k; ADR 0004 |". Author: engine-architect
(synthesis, README); if the row is not added, graphics-engineer (5.k, trace row) and test-engineer (step 24) drop the
pointer.

**M2 (stale across documents). README and SPIKE_PLAN lag behind this range's changes.** (a) README.md:63 still reads
"OQ-0003-11 | Should v2 one day drop the WebGL 2 fallback, and how many players use it? | Mustafa | … line 771", the
option condition 12 forbids weighing; 0003:784 now asks "How many players use the WebGL 2 fallback?" (b) README has
no OQ-0006-5 row, though 0006:667–668 and 0001:962–964 now route its development and staging case to the Advisor
before the netcode proof test (consistency F8 asked for the row). (c) SPIKE_PLAN item 1 "Depends on"
(SPIKE_PLAN.md:104–110) names "the development certificates (ADR 0006)" only, not OQ-0006-5's answer (consistency F8).
(d) SPIKE_PLAN.md:74 "PM-0001-1 to -56" and :97 "PM-0001-1 to PM-0001-56", while 0001:1130 adds PM-0001-57 to the
proof test. Smallest fix: README:63 "How many players use the WebGL 2 fallback? | Mustafa, on client field data";
README adds "| OQ-0006-5 (development and staging) | Which certificate does the WebSocket fallback present on
development and staging servers? | the Advisor, before the netcode proof test | ADR 0006 OQ-0006-5 |"; SPIKE_PLAN item 1
adds "OQ-0006-5's development and staging answer (the Advisor, before the test starts)"; SPIKE_PLAN:74, :97 "-57".
All four still hold in the working tree at 213dfb6 (the synthesis round is running). Author: engine-architect
(synthesis).

**L1 (unsupported: an incomplete list). ADR 0001 part 6's event list.** 0001:230–231: "tick-stamped gameplay events
(the events of ADR 0004 part 19: hits, deaths and kills, purchases, uplink cache events, tech tiers, upgrade picks and
pickups)". ADR 0004:454–458 lists also "blasts (position and strength, for the concussion shake and the fragments ADR
0003 draws)", "harvester deliveries and kills, structure kills and stage changes" (cosmetic shot events go once,
0001:234). As written, a structure kill or a harvester delivery is not among the events resent until acknowledged.
Smallest fix: "(the gameplay events of ADR 0004 part 19: hits, deaths and kills, blasts, purchases, uplink cache
events, harvester deliveries and kills, structure kills and stage changes, tech tiers, upgrade picks and pickups)".
Author: netcode-architect.

**L2 (stale: a new inconsistency). ADR 0003's GW17 row.** 0003:707: "port to TypeScript: the roof fade by flag over
the player's enclosure and the camera-player cutaway (OQ-0003-15), precompiled blend variant, ink masked (part 5.i);
pending removed". The other OQ-0003-15 items keep a pending note naming Mustafa's call (0003:703 CL12/GW3, :705 GA14,
:714 GP9), and the camera-player half is now his open call (0003:810–812, step 29). Smallest fix: "…; pending stays
for the camera-player cutaway: "Mustafa's call: LOOK_V2 §4 names only roofs (OQ-0003-15)"". Author: graphics-engineer.

**L3 (rule 5, term use). ADR 0006's PM-0006-17 adds the short "proof test".** 0006:884, added by test-engineer's
fix 5: "the proof test's load need not have them"; the same row keeps "only if the proof test's sim has the interest
filter". Consistency F13: GLOSSARY §8 marks the term only as "the netcode proof test", and outside ADR 0001 the full
name is used (completeness item 8 names these rows as test-engineer's). Smallest fix: "the netcode proof test's load",
"the netcode proof test's sim" (and PM-0006-15's "the proof test's client page", 0006:882, in the same pass). Author:
test-engineer.

**Notes (no fix required; for the authors' judgement).**
- N-a. 0001:809–811 "It is reported beside the client's sim share": the row that reports it is ADR 0002's PM-0002-25
  (0002:1064, local match); adding "(ADR 0002's PM-0002-25)" would close skeptic-1's "no row behind it" (netcode-architect).
- N-b. 0001:1375: R2-BUD-REC1's Shabzon mark sits under a network profile (r2:2320–2321: 100 ms RTT, ±10 ms jitter,
  2 % loss); S4 runs in the sim only. PM-0001-20 bounds the network half (≤ 0.1 % missing inputs at 5 % loss), so the
  substance holds; "(in the sim; the network half is PM-0001-20's)" would say it (netcode-architect).
- N-c. PM-0001-57's pitch bound is one 0.18° step; a rounding quantiser errs by half a step (0.09°), so the mark also
  passes a truncating one. Labelled proposed with its reason; tighten to 0.09° only if wanted (test-engineer).
- N-d. 0001:1182–1183 "a 30 Hz baseline mark (PM-0001-4 to PM-0001-27)": PM-0001-57 falls to "Every other failure goes
  back to the drafts" (part 6), which is the right handling (test-engineer).
- N-e. SPIKE_PLAN.md:30 still defines MAC as "a macOS machine with Safari"; the version rule (stable Safari, WebGPU
  from 26.0, WebTransport from 26.4) lives in the drafts' keys (0003:916–919), not in the shared fixture (consistency F4's
  synthesis half; engine-architect).
- N-f. 0006:549 "that rate is now TICK-30's 30 Hz": TICK-30 makes 30 Hz the baseline and keeps 60 Hz possible on the
  60 Hz margin; "the 30 Hz baseline (TICK-30)" would be exact (infra-engineer).

## 7. Fetch audit
All with `curl -sSL --max-time 30` through the session's proxy; hosts on `.claude/settings.json`'s WebFetch list;
pages saved under `.gate/fixcheck-a/fetch/` and read as data only (no instruction in them was followed).
| time (Amsterdam) | host | URL | status | bytes | used for |
|---|---|---|---|---|---|
| 2026-11-01 19:29:43 | www.w3.org | https://www.w3.org/TR/webaudio-1.1/ | 200 | 3,087,268 | B29 (OQ-0003-14's four quotes, the draft's date) |
| 2026-11-02 09:16:54 | developers.cloudflare.com | https://developers.cloudflare.com/pages/configuration/preview-deployments/ | 200 | 127,307 | C4, C17, C19 (the access policy's scope; no word on branch aliases) |
| 2026-11-02 09:16:55 | docs.sentry.io | https://docs.sentry.io/platforms/javascript/data-management/data-collected/ | 200 | 281,298 | C7, C8 (IP, headers, cookies in the browser) |

## Not verified
- Text after 57c43c2 (9ed8d67 to 9a03999 and the uncommitted round in the working tree): a later unit's.
- README and SPIKE_PLAN were read only where this range's changes point at them (M1, M2, N-e).
- skeptic-2's three.js r186 source read (ToonOutlinePassNode.js) was not re-fetched; B21 rests on r3:343–350 and the
  skeptic's saved copy.
