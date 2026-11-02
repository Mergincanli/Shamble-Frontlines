# The final fix round: findings to batch per author (the session's tracking file, 2026-11-02)

Collected from fixcheck-a, -b, -c, the skeptics and the step-10 reviews; one round per author at the end.

## From verifier fixcheck-b (verifier-fixcheck-b-verdicts.md; PASS WITH NOTES, 2026-11-02 about 09:25)
- test-engineer: finding 1 (PM-0002-17, -42: the `enum` bite required again; skeptic-4 first), finding 3 (PM-0002-45
  cannot fail G7 in the foundation; skeptic-4 first), finding 9 (PM-0002-23: "(2 × 1000 / 30 = 66.67 ms)"), finding 12
  (PM-0005-42 "faction definition" vs CD-13's client definition). Findings 2, 4, 6: fixed in round 3 (81d2a65); fixcheck-c
  checks them.
- engine-architect (0005 frame): finding 7 (0005:1759 "Revisit when" lost its verb), finding 11 (0005:5 author line
  reads as if graphics-engineer wrote the pass marks), finding 13 (0005:1693 "until the spikes have measured it" vs
  ADR 0002's "until the Advisor sets the first limit").
- engine-architect (0002): finding 8 (OQ-0002-8's ids line: add R5-XT-OQ6; "no answer on one side" fits R4-XT-OQ5 only).
- rust-reviewer: finding 10 (0004:339–340 "no Rust navigation crate read promises determinism"); finding 5 (0004:805–807,
  "listed once in README": true once the synthesis adds the fragments row; skeptic-4 first).
- The synthesis (sent 09:30, queued): README rows 69–70, 72, 82, 109; OQ-0002-8 row; fragments row; SPIKE_PLAN:30, spike 4
  table (PM-0004-73, -74), :176.
- For the report: R5-TSLINT-F12's construct list (frozen in r5 as "general knowledge, unverified") is now confirmed at the
  source (typescriptlang.org/tsconfig, "Erasable Syntax Only"); the six pending source checks are confirmed; of the five
  claims on unread sources, four confirmed and 0004 row 138 supported (Unsafe Code Guidelines glossary).

## From verifier fixcheck-a (verifier-fixcheck-a-verdicts.md; PASS WITH NOTES, 2026-11-02 about 09:35)
109 rows (0001 39, 0003 43, 0006 27): 104 confirmed, 0 wrong, 4 unsupported (A8; B14, B18, B38), 1 stale (B25), 1 rule
(L3); 114 blocks, 109 resolved, 5 partly; the fix files applied in commit order to 80a27b3 give 57c43c2 byte for byte;
21 citation flags (0001 3, 0006 18), all script artifacts; 3 fetches, all 200.
- netcode-architect: L1 = A8 (0001:230–231 says "the events of ADR 0004 part 19" but leaves out blasts, harvester
  deliveries and kills, structure kills and stage changes; skeptic-4 first). Optional note: 0001:809–811 could cite
  PM-0002-25 as the row reporting the Worker's extra cost.
- graphics-engineer: L2 = B25 (0003:707, GW17 "pending removed", while its camera-player half is now Mustafa's open
  call under OQ-0003-15; the other OQ-0003-15 rows keep a pending note).
- test-engineer: L3 (0006:884 "the proof test's load"): fixed in round 3 (81d2a65); fixcheck-c confirms.
- The synthesis (sent about 09:36, queued): M1 (the fragments row), M2 (README:63 OQ-0003-11's old question; OQ-0006-5
  row; SPIKE_PLAN item 1 "Depends on" OQ-0006-5; SPIKE_PLAN:74 and :97 "PM-0001-1 to -56" → -57).
- Skeptic-4 (added about 09:36): A8; B14/B18/B38.

## From the step-10 review by engine-architect (review-engine-architect.md; 2026-11-02 about 09:45)
Verdicts: ADR 0001 REVISE (small: one high gap); 0003, 0004, 0006, 0002's pass marks, 0005's CD/AP pass-mark rows:
APPROVE WITH NOTES. Findings (its numbering; owners as the review names them):
1. High: no timing rule for events (0001:445, 255–262; 0002:258–262): spatial events play ahead of the delayed poses;
   R2-SYNC-REC6 (r2:2210, blasts "on the timeline of the entity they hit") is marked adopted but not carried. Fix: a
   part 7 rule (the Worker releases spatial events at the delayed time); ADR 0002 part 5 and PM-0002-9 follow.
   Owners: netcode-architect, engine-architect (0002), test-engineer.
2. Medium: the 30 Hz input cost understated (0001:649, 807–808, 1165): the local player is drawn one tick behind its
   prediction; no mark measures input-to-screen. Its Q1 (decision).
3. Medium: the client's prediction step is not defined as a sim crate entry point (0004:151–156, :977). rust-reviewer.
4. Medium: the aim-ray seam is untested (0002:1056, 0001:1170). test-engineer.
5. Medium: no production release model (ADR 0006 with 0001:560): a refused client reloads the same bundle and loops.
   Its Q2 (decision: an open question in 0006 now).
6. Medium: ADR 0005 hands the archive of shipped content sets to ADR 0006, which has none (0005:388–389, 0006:1022).
7. Medium: the 8-rooms-per-VM mark contradicts PM-0001-1 (one tick thread per vCPU) and PM-0006-13 (0001:1118, 1146;
   0006:880). test-engineer.
8. Medium: the client endpoint lives only in `sim-wasm`, while ADR 0001's tests run native headless clients (0002:183,
   its own text). engine-architect (0002).
9. Low: PM-0005-10 tests only the native dev server (0005:2034). 10. Low: no endpoint named for client field data or
   crash reports (0006:321, 0001:497). 11. Lows in ADR 0003 (370–374 audio occlusion, 928–931 bench Worker load,
   276–277 wording, 381 stale hedge). 12. Lows in ADR 0002, its own text (191–192 supervisor crate; 255–262 the block's
   owner section omits the floor layer and area id of 0004:387–388).
Questions: Q1 own-movement delay at 30 Hz (Mustafa by feel after the netcode proof test; add the absolute measurement
now); Q2 the release model (an open question in 0006 now; decider the Advisor).

## From the step-10 review by security-auditor (review-security-auditor.md; 2026-11-02 about 09:30)
Verdicts: ADR 0001 APPROVE WITH NOTES; ADR 0002 REVISE (finding 1); ADR 0006 REVISE (finding 1's interim position B9,
findings 2–5); synthesis docs APPROVE WITH NOTES. Its README pointer note is fixed by 47eadd9 (69 of 69 on target).
1. High: the preview token is "held only by the deploy job", but a `pull_request` run uses the branch's own workflow
   (R5-PREV-F2, r5:770–774), so a branch can rewrite the deploy job and take the account-wide Pages token (0002:519–525;
   0006:385, 389–394). Fix: production deploys only from main, own environment and token; a required reviewer on
   `preview` until the baseline (R5-PREV-F6); revoke the spike's token; a bite in PM-0002-66. Owners: engine-architect
   (0002), infra-engineer (0006), test-engineer (PM-0002-66).
2. Medium: 0006:565 "no player reaches a game server … except through a multiplayer preview behind the access wall" is
   unsupported (the wall guards the page; staging is public). Fix: staging tokens only from a walled endpoint, a
   separate staging key. infra-engineer.
3. Medium: the SameSite=Strict guest cookie needs page and lobby on one registrable domain; none decided (0001:369–371,
   0006:380). netcode-architect, infra-engineer.
4. Medium: room processes not separated from the VM's secrets (0006:179, 386, 355, 635, 455). Fix: a new item B12.
   infra-engineer.
5. Medium: the per-command aim log is 1,728,000 lines per 20-minute match, 8,640 × the 0.8 GB/month log budget
   (0001:279–280 vs 0006:300–301); no store or retention. netcode-architect, infra-engineer.
6. Medium: "the server's own RTT stops a client from buying extra rewind" overstated (0001:570–571): log quinn's RTT
   beside the ping RTT; state the accepted risk. netcode-architect.
7. Medium: per-address limits need an IPv6 /64 rule (0001:330–331, 363–364, 497). netcode-architect.
8. Medium: "not older than the last command run" allows several commands per tick (0001:191–197). Fix: strictly newer,
   a fast-clock row in PM-0001-21. netcode-architect, test-engineer.
9–16. Lows: WebSocket accept/TLS/upgrade deadline (0001:330, PM-0001-25); npm advisories and cargo-deny `sources`
   (GLOSSARY:691 is research text, frozen: the fix goes in 0006:377); a strict-CSP-ready build (0006:379); display-name
   validation (0001:355); fuzzing control-message sequences (PM-0001-34, PM-0004-60); the crash endpoint's caps
   (0006:313–314); the authority-only seed's source (0004:254, rust-reviewer); local-match records never in the store
   (0006 part 12).
Questions: Q1 (the Advisor) read item B9 before the foundation's preview step? It recommends no: keep BASELINE-WHEN (b)
and adopt finding 1's interim fix. Q2 (Mustafa) does a host's kick bar the player for the rest of the match? It
recommends yes.

## From skeptic-4 (verifier-skeptic-4-results.md; 2026-11-02 about 09:31): all five upheld, none refuted
- fixcheck-b finding 1 (PM-0002-17, -42): upheld; the tsconfig entry names `enum` twice; the TypeScript 5.8 release
  notes tie the flag to the same list. Not verified: an ambient `enum` (`declare enum`, `.d.ts`); three repository
  files returned 404. → test-engineer: the `enum` bite required again (an ambient `enum` not claimed).
- fixcheck-b finding 3 (PM-0002-45): upheld (R5's own G7 row had "(its size is set once REC4's budget exists)", which the
  ADR dropped). → test-engineer: a test limit planted in the budget file for the run.
- fixcheck-b finding 5 (0004:805–807) and fixcheck-a M1 (0003's three pointers): upheld as the text stood; the synthesis
  row "Fragments as damage" (applied in 47eadd9) makes all four true. → fixcheck-c confirms; no author fix.
- fixcheck-a A8 / L1 (0001 part 6's event list): upheld; three part-19 events have no class (blasts; harvester
  deliveries and kills; structure kills and stage changes). → netcode-architect: give them a class, and say whether
  blasts are resent or sent once (ADR 0004 gives blasts only presentation uses).

## From the step-10 review by infra-engineer (review-infra-engineer.md; 2026-11-02 about 09:38)
Verdicts: ADR 0001 and 0002 APPROVE WITH NOTES; ADR 0006 decision text REVISE (its own text: it says so); ADR 0006
pass marks REVISE. Cross-references security-auditor's F-I, F-C′, F-O, F-Q (not repeated).
1. High: no release path to production, no two versions during a deploy, no rollback of a bad server release
   (0006:259–260; 0001:1340–1341, :560). Same as engine-architect's #5. infra-engineer, netcode-architect.
2. High: PM-0006-13 (8 tick threads) vs PM-0001-27 and its validity rule (one thread alone per vCPU; the 1 % steal cap
   voids shared-host runs) (0006:880; 0001:1146, 1098–1101); SPIKE_PLAN:100–101 "read the same" while the reference,
   run count and pinning differ. Same as engine-architect's #7. test-engineer; the synthesis for SPIKE_PLAN.
3. Medium: macOS runners are standard and free on a public repository (GitHub's runner page, read 2026-11-02): real
   Safari costs time, not money (0006:523, OQ-0006-7; OQ-0002-5). infra-engineer, engine-architect (0002).
4. Medium: a certificate hash written into a static preview goes stale at every restart or rotation (0002:530–534;
   0006:228–229; r2:3441, :2720–2721): staging presents its trusted certificate on both transports.
5. Medium: "nothing else" / "0 other rules" vs TCP 80, the lobby's HTTPS, staging delivery, admin access: list the rules
   per VM role (0006:185–190; PM-0006-15).
6. Medium: no domain or DNS provider named (0006:221–235, OQ-0006-5, migration step 0): its DOMAIN question.
7. Medium: OQ-0006-5 waits on ADR 0001's port plan, which names no port: does the fallback need TCP 443? (0006:667;
   0001:310–347). netcode-architect, infra-engineer.
8. Medium: at the marks' ceilings a room may use 1.5 vCPU → 2 rooms per VM; the goal fleet is then about 967 EUR, not
   534: state it as a floor (0006:262–265; 0001:538).
9–10. Medium: one 4-vCPU load generator for 384 sessions, all from one address that meets the per-address cap: size it
   by a calibration run; the cap per run; PM-0001-25 with the cap off and on (0006:812–814, :880; 0001:330–331).
11. Medium: how the lobby asks a VM for a room and learns its port (0001:498; 0006:208–220; README:285).
12. Medium: the repository is public, so committed unreleased art is public (0006:144–149): its UNRELEASED-ART question.
13. Medium: the 5 EUR bill cap (r6:716–717) was for "a few hours of one or two VMs": about 23 reference-server hours
   (3.19 EUR) for ADR 0006's own rows (PM-0006-18, 0006:885).
14. Medium-low: the phase-1 tick bench has no access path or cost line; the Hetzner API token missing from B9
   (0006:878, :385; 0002:427–433).
15–20. Low: no alert route (0006:234); SMT siblings (PM-0006-11); location before the reference server (0006:811); the
   500-builds follow-up (0006:873); failure artefacts' retention (0002:426); a tombstone workaround for PREV-DELETE
   (0006:872).
Questions: UNRELEASED-ART (Mustafa: (a) a login is enough, recommended until a model needs hiding, with
ASSET-RESEARCH); ROOMS-COUNT-RULE (the Advisor with test-engineer: count rooms by "inside the tick budget" only,
recommended, the 10 % share reported beside); SPIKE-VM-ACCESS (Mustafa: (b) a separate Hetzner project with its own
token, recommended); RELEASE-WINDOW (Mustafa: (a) a release window while playtests are scheduled, recommended);
DOMAIN (Mustafa after the Advisor's OQ-0006-5 read: (a) a private test CA for the proof test, a domain before slice 3,
recommended).

## From the step-10 review by game-designer (review-game-designer.md; 2026-11-02 about 09:42)
Verdicts: ADR 0001 REVISE; ADR 0004 REVISE (small: wording and one new open question); ADR 0005's ability pass-mark
rows REVISE (one row). Every golden number in the ability rows checks out.
1. High: the server refuses an aim point "beyond weapon range" (0001:193–194; PM-0001-21, 0001:1140; r2:2062), but a
   top-down cursor reaches about 28 m (44 m with the ADS pan, r2:847–851), and legacy ranges are Katana 3.6 m, Molotov
   0, Bagpipe 34 m (LEGACY_TUNING:181): almost every blade swing and Molotov throw would be refused. Fix: clamp the
   point to the weapon's range; drop "beyond weapon range" from the forged set. netcode-architect, test-engineer.
2. Medium: no command or message names what a player buys, while ADR 0005 relies on the purchase naming a unit
   (0001:484, 482–498; 0005:868–869, 920, 922): a buy message checked against money, tier and terminal.
   netcode-architect.
3. Medium: track state sends "levels" only; the own-state section's contents are unstated (0001:383–384, 174; 0005
   AB-8, 930–931): effects of picks, the Shield's slow, the Banzai's speed must reach the predicting client.
   netcode-architect.
4. Medium: "steal by GE8's presence count" contradicts LOOP_V2 §4 [DECIDED] "holding E" (0004:425, 750, 751;
   LEGACY_INVENTORY:294); no draft asks about two enemy teams at one uplink. rust-reviewer (its GD-STEAL question).
5. Medium: PM-0005-31 requires "distinct candidates", which OQ-0005-AB8 leaves open (0005:2056, 1197). test-engineer.
6. Medium: the match record keeps picks only as per-player totals, no tick (0004:446, 454–458 → 0006:342–350), so
   OQ-0005-AB3's snowball measurements cannot be made from telemetry. infra-engineer, rust-reviewer.
7. Low–medium: spawn protection, barracks interiors closed to enemies and the Recruit respawn (LOOP_V2 §2, §5) have no
   home in ADR 0004 (0004:422–427; 0005:1799). rust-reviewer.
8. Lows: PM-0005-25 lacks "Conscript beats Peacekeeper"; PM-0005-24 lacks the minute-5 Tech 1 floor (9,000 ticks);
   "the pending offer of three" blocks AB8's queue option and "three" is hard-coded (0004:433, 444–445; 0001:382–383,
   493); honest double clicks counted as forged (0001:383); E for pickup vs E for steals (0001:386–387); PM-0005-29
   assumes a voluntary drop; Feel Lab step 3 (0005:2165–2166); the upgrade record keyed by slot loses a late
   rejoiner's tracks (0004:431, 0005:925); money's unit and the rebuild-split remainder; the pickup pool's capacity;
   OQ-0001-7 covers shots, not the active-reload tap.
Questions: GD-STEAL (presence rule as a data switch, decided in the uplinks change-set from 3 sim runs each);
GD-REJOIN (yes, keyed by player); GD-PICK-VIS (owner only to start, with OQ-0005-AB7); GD-CARRIER (yes, public, carry
speed from sims); GD-BOT-LOOT (no, for the slice 2 greybox). All Mustafa's.

## From the step-10 review by netcode-architect (review-netcode-architect.md; 2026-11-02 about 09:44)
Verdicts: ADR 0001's pass marks and netcode checklist REVISE (small); ADR 0001 parts 5, 7, 8 (its own text,
self-review) REVISE (small); ADR 0002, 0004, ENGINE_OVERVIEW, SPIKE_PLAN items 1–2 APPROVE WITH NOTES; README APPROVE.
It leaves out what engine-architect (1–4, 8) and security-auditor (6, 8) already raise.
1. High: the rewind estimate "RTT + interpolation delay" (Source's model) leaves out part 5's 1–3 tick command buffer
   and the page's one-tick blend: an honest shot at P2 needs 233–383 ms of rewind, above the 200 ms cap (and 250 ms
   but at the low end); PM-0001-20 implies a 3-tick buffer; PM-0001-48 fails by design; "server time" in part 7
   undefined (0001:199–200, 272–276, 280–282, 803–806; OQ-0001-5, -6; PM-0001-20, -48, -49; checklist step 10).
   netcode-architect, test-engineer. Its Q1 (the cap).
2. Medium: PM-0001-12's 160 kbps downstream mean is judged in the stress scene, but the research set it on a normal
   48-player bot match (r2:2339; GLOSSARY:415–416): 700 shots/s alone give 166.2 kbps. test-engineer.
3. Medium: no rule or mark stops own-weapon cues replaying on every prediction replay, or the server echoing the
   shooter's own shots (0001:166–168, 234; 0005:2113; SPIKE_PLAN:123; R1-ABL-REC4 handed to 0001, no row).
   netcode-architect, test-engineer.
4. Medium: received datagrams reach Rust once per tick with no arrival time: 0–33 ms of false jitter against the
   ±5 ms marks PM-0001-33, -47 (0002:280–282). engine-architect (0002).
5. Medium: "late" = more than 66.7 ms after sending, only 16.7 ms above P2's 50 ms one-way; the profiles never split
   latency and jitter by direction: 8.3 % late against a 3 % mark (PM-0001-37, -3). test-engineer. Its Q2.
6. Medium: any failure in PM-0001-4 to -27 sends parts 2 and 3 back, but only -4 to -11 time the tick; the sentence
   contradicts itself on -15, -16 (0001:1186–1190). test-engineer.
7. Medium: when the page posts its command is unstated; once per tick can delay an input by up to 66.7 ms past
   PM-0002-13's 50 ms; every frame meets it but PM-0002-64 forbids it (0002:274; PM-0002-64, -13; ENGINE_OVERVIEW:37,
   :94). engine-architect (0002), test-engineer.
8–17. Lows: the Worker's pacer and the server-clock estimate (0004:188–193); 5–7 repeated commands at P2, not 4: a
   "newest received" ack (0001:513–518); PM-0001-44's 6-tick replay; PM-0001-42 depends on soldier collision; the 60 Hz
   run's snapshot rate (0001:127, PM-0001-39); no client resync message (0001:489, 556–557); a generation change as
   removal + addition (0001:241–242); client and local server order in one Worker step (0002 part 6); "seated
   players" (0001:240, 302); ENGINE_OVERVIEW:9–10 wording.
Questions: Q1 (Mustafa and the Advisor, extends OQ-0001-5): the rewind cap with the buffer counted (recommend: set from
the proof test's measured honest rewind, p99 of PM-0001-49, with run-on-arrival measured beside it); Q2 (the
Advisor): may the profiles split latency and jitter half each way, uniform, as a proposed value (recommend yes).

## From the step-10 review by perf-auditor (review-perf-auditor.md; 2026-11-02 about 09:47)
Verdicts: ADR 0001 REVISE (small: three text changes, two rows); ADR 0003 REVISE (small: three pass-mark rules, two
Decision sentences); ADR 0002, 0004, 0005, 0006, ENGINE_OVERVIEW APPROVE WITH NOTES; SPIKE_PLAN, README APPROVE.
Nothing measured: arithmetic on the drafts' and the research's own figures. Agrees with infra-engineer #8 and
engine-architect #7.
1. High: the 160 kbps mean is gated in the stress scene, but R2 set it on its estimate scene: the stress scene costs
   157–182 kbps (shots +20.1, spawns +15.7, the 11 big vehicles −24.8) (0001:529, :539; PM-0001-12). Gate the mean in
   the estimate scene, the 256 kbps cap in the stress scene. Same as netcode-architect #2. test-engineer,
   netcode-architect. Its SS-BANDWIDTH question.
2. Medium: the E3b fallback cannot pass the budget it is re-run against (188.3 kbps in the estimate scene; 1,650 B of
   projectiles per snapshot over the 1,067 B cap); a desync is a bug against PM-0004-73, not a reason to switch
   (0001:222–223, :1189; PM-0001-54). netcode-architect, test-engineer.
3. Medium: the upstream 22.3 kbps assumes R2's 8 B commands; part 5's command (an 80-bit aim point, 5–7 repeats at P2,
   a cap of 8) costs about 37–57 kbps against 48 kbps (0001:513–519 vs 186–190): delta-code the repeats (R2's U2) or
   lower the cap. netcode-architect.
4. Medium: VISION_V2 §6.9's "without dropping frames" has no mark; the frame rows are an uncapped p95 (0003:326–329;
   PM-0003-30): a row counting presented frames with the very large units on screen. test-engineer, graphics-engineer.
   Its DROPPED-FRAMES question.
5. Medium: one research mark, two readings: PM-0002-28 (every second 30 presented frames) vs PM-0004-8 (median
   ≤ 28 ms, p95 ≤ 33.3 ms) (0002:1070; 0004:956). test-engineer.
6. Medium: the p99 rows rest on 6 frames per 600-frame run; the validity rule checks only medians (0003:941, :946).
   test-engineer.
7. Medium: the minimum laptop is measured cold (about 30 s per run) while a match lasts about 20 minutes (LOOP_V2:100):
   gate a run after 15+ minutes (engineering experience, not researched) (0003:936–939). test-engineer. Its HEAT-SOAK
   question.
8. Medium: "about 786, inside 800" leaves out post-processing passes, health bars, VFX pools and projectile kinds
   (13–31 draws against the 14 left) (0003:318–321). graphics-engineer.
9–14. Lows: a door every 60 ticks always lands on the same tick parity at 60 Hz (PM-0001-2, -39); memory per
   attachment stated single-sample though MSAA 4× (0003:204, :514); the triangle row counts the shadow pass (about
   5.2k triangles per soldier at Low; ADR 0005's per-unit budget questions need it); the per-frame allocation CI check
   has no method (PM-0003-36); no budget for the Worker as a whole (tick, heap, Wasm memory); the main-thread CPU row
   leaves out the DOM HUD's layout work (PM-0003-19).
Questions: DROPPED-FRAMES (the Advisor, confirmed by Mustafa: count presented frames with the very large units on
screen); SS-BANDWIDTH (the Advisor with netcode-architect: the estimate scene for the mean, the cap in the stress
scene); EFFECTS-WITH-PICKS (the Advisor after Mustafa's levelling answer: keep 8, gate again at slice 4); HEAT-SOAK
(the Advisor: the soaked run, the cold one reported).

## For fixcheck-d: facts the final round cites to the step-10 reviews, not to the research (check them at their sources)
From engine-architect's ADR 0002 batch (engine-architect-adr0002-claims.md rows F1–F25): binaryen's `--fast-math` help
text; std's clock calls panicking on the browser's Wasm target (rust-lang/rust library/std/src/sys/time/unsupported.rs);
`to_degrees`/`to_radians` carrying std's precision note; macOS runners free on a public repository (GitHub's runner
page); the merge-commit reading behind SA-1 (a `pull_request` run uses the PR's merge commit's workflow, R5-PREV-F2).
From rust-reviewer's ADR 0004 batch: R2-ROOM-F8 (r2:1459–1467) for the panic rule; its panic-hook claim was checked
natively only (Wasm untested).
