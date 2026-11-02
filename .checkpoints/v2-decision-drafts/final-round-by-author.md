# The final fix round, by author (the session, 2026-11-02; built from final-round-findings.md and the review files)

Ids: EA = review-engine-architect.md, SA = review-security-auditor.md, IE = review-infra-engineer.md, GD =
review-game-designer.md, NA = review-netcode-architect.md, PA = review-perf-auditor.md, RR = review-rust-reviewer.md,
GE = review-graphics-engineer.md, AD = review-art-director.md, TE = review-test-engineer.md (numbers are each review's
own finding numbers); FA, FB, FC = verifier-fixcheck-a/-b/-c-verdicts.md; SK4 = verifier-skeptic-4-results.md (all
upheld). The full text, evidence and proposed fix of each is in its file.

## netcode-architect: ADR 0001, all but "### Pass marks"
- High: EA-1 (no timing rule for events: release spatial events at the delayed time; R2-SYNC-REC6, r2:2210); GD-1
  (clamp the aim point to the weapon's range instead of refusing it); NA-1 (the rewind estimate leaves out the command
  buffer and the page's blend; "server time" in part 7 undefined; its Q1 extends OQ-0001-5).
- Medium: FA-L1 = A8 (SK4 upheld: part 6's event list leaves out blasts, harvester deliveries and kills, structure
  kills and stage changes; say whether blasts are resent or sent once); EA-2 (the 30 Hz input cost: the local player
  drawn one tick behind its prediction; its Q1); EA-5 and IE-1 (the release model: with infra-engineer, 0001:560,
  1340–1341); GD-2 (a buy message for units and vehicles); GD-3 (what the own-state section carries); NA-3 (own-weapon
  cues on prediction replays; the server's echo of the shooter's own shots); PA-2 (the E3b fallback cannot pass its
  budget); PA-3 (the upstream cost of part 5's command: delta-code repeats or lower the cap); SA-3 (the SameSite cookie
  needs one registrable domain); SA-5 (the per-command aim log's volume: with infra-engineer); SA-6 (the RTT claim
  overstated; log quinn's RTT beside the ping RTT); SA-7 (IPv6 /64 for per-address limits); SA-8 (each command's tick
  strictly newer); IE-7 (does the fallback need TCP 443: name the ports); IE-11 (how the lobby asks a VM for a room).
- Low: SA-9 (the WebSocket path's accept, TLS and upgrade deadline); SA-12 (display names); EA-10 (the endpoint for
  client field data and crash reports, with infra-engineer); GD-8's 0001 items (the hard-coded "three": 0001:382–383,
  493; honest double clicks counted as forged: 0001:383; E for pickup vs E for steals: 0001:386–387; OQ-0001-7 covers
  shots, not the active-reload tap); NA-9 (5–7 repeated commands at P2: a "newest received" ack); NA-12 (the 60 Hz run's
  snapshot rate); NA-13 (a client resync message); NA-14 (a generation change as removal + addition); NA-16 ("seated
  players"); the optional FA note (0001:809–811 could cite PM-0002-25).
- Questions to carry as open questions where they are yours: NA Q1 (the rewind cap), NA Q2 (the profiles' direction
  split), EA Q1 (own-movement delay at 30 Hz), SA Q2 (does a host's kick bar the player for the match), GD-REJOIN.
- From review-rust-reviewer.md: RR-2's 0001 half (0001:348–349: room timers count ticks; no clock read inside `room`
  or the client endpoint); RR-3 medium (0001:352–353, 554: with the sim on its own thread and `panic = "unwind"`, a panic
  in the tick leaves a zombie room: the match ends and the process exits non-zero; the tick and network threads share
  only channels; with rust-reviewer's 0004:472). Question ROOM-CRASH (the Advisor: the match ends, recommended, or the
  room resumes from a snapshot, a generic feature for later).

## infra-engineer: ADR 0006, all but "### Pass marks"
- High: SA-1 (the preview token: production deploys only from main with its own environment and token; a required
  reviewer on `preview` until the baseline; revoke the spike's token; 0006:385, 389–394; with engine-architect's ADR
  0002 part 10); IE-1 and EA-5 (a release path: how a client and servers reach production, two versions during a
  deploy, rollback of a bad server release; EA Q2 and IE RELEASE-WINDOW: an open question for the Advisor and Mustafa).
- Medium: SA-2 (0006:565, the access-wall claim: staging tokens from a walled endpoint, a separate staging key); SA-3
  (the guest cookie's registrable domain, 0006:380, with IE-6's DOMAIN question); SA-4 (room processes vs the VM's
  secrets: a new item B12); SA-5 (the aim log's store and retention, 0006:300–301, with netcode-architect); EA-6 (the
  archive of shipped content sets ADR 0005 hands to 0006, 0006:1022); IE-3 (macOS runners are standard and free on a
  public repository: 0006:523, OQ-0006-7); IE-4 (a certificate hash in a static preview goes stale: staging presents
  its trusted certificate on both transports; 0006:228–229); IE-5 (the firewall rules per VM role, 0006:185–190); IE-6
  (no domain or DNS provider named: OQ-0006-5, migration step 0; its DOMAIN question); IE-7 (OQ-0006-5 waits on a port
  plan ADR 0001 lacks); IE-8 (at the marks' ceilings two rooms per VM: the goal fleet about 967 EUR as a floor;
  0006:262–265); IE-9/10 (one load generator for 384 sessions from one address: size it by a calibration run, the cap
  per run; 0006:812–814); IE-11 (how the lobby asks a VM for a room, 0006:208–220); IE-12 (a public repository makes
  committed unreleased art public, 0006:144–149: its UNRELEASED-ART question); IE-14 (the tick bench's access path and
  cost; the Hetzner API token missing from B9; 0006:385); GD-6 (the match record keeps picks as totals with no tick:
  0006:342–350, with rust-reviewer's part 19); EA-10 (the endpoint for client field data and crash reports, 0006:321).
- Low: SA-10 (npm advisories; cargo-deny's `sources`; 0006:377); SA-11 (a strict-CSP-ready build, 0006:379); SA-14 (the
  crash endpoint's body cap, rate limit and retention, 0006:313–314); SA-16 (local-match records never in the store,
  part 12); IE-15 (an alert route, 0006:234); IE-17 (the location before the reference server, 0006:811); IE-18 and
  IE-20 where they are your text (0006:872–873).
- Questions: IE's UNRELEASED-ART, SPIKE-VM-ACCESS, RELEASE-WINDOW, DOMAIN; SA Q1 (read item B9 before the foundation's
  preview step?).

## test-engineer, unit A: the "### Pass marks" sections of ADR 0001 and ADR 0002 (and the netcode checklist)
- ADR 0001: GD-1 (PM-0001-21: drop "beyond weapon range" from the forged set; the point is clamped); NA-1 (PM-0001-20,
  -48, -49 and checklist step 10 with the rewind arithmetic, once netcode-architect's part 7 lands: follow its text);
  NA-2 and PA-1 (PM-0001-12: gate the 160 kbps mean in R2's estimate scene, the 256 kbps cap in the stress scene); NA-3
  (a row for own-weapon cues on prediction replays, R1-ABL-REC4); NA-5 (PM-0001-37, -3: "late" and the direction split;
  NA Q2); NA-6 (0001:1186–1190: which failures send parts 2 and 3 back); NA-10 (PM-0001-44's 6-tick replay); NA-11
  (PM-0001-42 depends on soldier collision); NA-12 (PM-0001-39's snapshot rate); PA-2 (PM-0001-54, the E3b fallback);
  PA-9 (PM-0001-2, -39: a door every 60 ticks lands on one tick parity); EA-2 (an absolute input-to-screen measurement);
  EA-4 (the aim-ray seam: a row that checks the Worker's cast against what the page drew); EA-7 and IE-2 (PM-0001-27 vs
  PM-0006-13: one tick thread alone per vCPU, the steal cap; align with unit B); SA-8 (a fast-clock row in PM-0001-21);
  SA-9 (PM-0001-25: the WebSocket deadline); SA-13 (fuzz control-message sequences, PM-0001-34); IE-9/10 (PM-0001-25
  with the per-address cap off and on); FC-7 (PM-0001-45's anchor leaves out the extrapolation sentence).
- ADR 0002: FB-1 (PM-0002-17, -42: the `enum` bite required again; SK4 upheld; an ambient `enum` not claimed); FB-3
  (PM-0002-45: a limit planted in the budget file for the run; SK4 upheld); FB-9 (PM-0002-23: "(2 × 1000 / 30 =
  66.67 ms)"); FC-3 (PM-0002-29's header still says a spike "decides R1-RR-OQ5"); FC-10 (PM-0002-3: the glue's allocator
  calls as instance exports has no source); EA-1's follow (PM-0002-9, once part 5 lands); SA-1's follow (PM-0002-66: a
  bite with a test PR that edits the deploy job); NA-7 (PM-0002-13 vs -64: when the page posts its command); PA-5
  (PM-0002-28 vs PM-0004-8: one reading of "30+ fps"; align with unit B).
- From review-rust-reviewer.md: RR-1's bite (PM-0002-41: a planted `powi` or `min` on signed zero); RR-6 (PM-0002-6:
  a bite for a stale view of Wasm memory after growth); RR-8 (PM-0002-29: measure link-time optimization and
  `codegen-units`, Brotli beside gzip); RR-3's mark (one pass mark: a panic in the tick ends the match and the process
  exits non-zero; with unit B if it lands in ADR 0004).

## test-engineer, unit B: the "### Pass marks" sections of ADR 0003, 0004, 0005 and 0006 (the look and Feel Lab checklists)
- ADR 0003: PA-4 (a row for VISION_V2 §6.9's "without dropping frames": count presented frames with the very large
  units on screen; PA DROPPED-FRAMES); PA-6 (the p99 rows: 6 frames per 600-frame run; a validity rule for the tail);
  PA-7 (the minimum laptop cold: a run after 15+ minutes; PA HEAT-SOAK); PA-12 (PM-0003-36: a method for the per-frame
  allocation check); PA-14 (PM-0003-19: the DOM HUD's layout work).
- ADR 0004: SA-13 (PM-0004-60: fuzz control-message sequences into `room`); PA-5 (PM-0004-8, with unit A).
- ADR 0005: FB-12 (PM-0005-42: "faction definition" vs CD-13's client definition); GD-5 (PM-0005-31's "distinct
  candidates" while OQ-0005-AB8 leaves it open); EA-9 (PM-0005-10 tests only the native dev server); GD-8's rows
  (PM-0005-25: the "Conscript beats Peacekeeper" leg; PM-0005-24: the minute-5 Tech 1 floor, 9,000 ticks at 30 Hz;
  PM-0005-29 assumes a voluntary drop; Feel Lab step 3, 0005:2165–2166).
- ADR 0006: IE-2 and EA-7 (PM-0006-13: eight tick threads cannot each be alone on a vCPU; with unit A); IE-5 (PM-0006-15's
  "0 other rules"); IE-9/10 (the load generator's calibration); IE-13 (PM-0006-18: the 5 EUR cap vs about 23 reference
  server hours); IE-16 (PM-0006-11: SMT siblings); IE-18, IE-20 (0006:872–873, if your text).
- From review-graphics-engineer.md: GE-1 (PM-0005-35 enforces one material per class: one per content key); GE-3
  (PM-0003-7, -33: the `programs` counter cannot see blend or MSAA pipeline variants: count pipeline creations too);
  GE-6 (0003:681: no mark that GPU memory and render objects return to baseline between matches); GE-7 (checklist step
  26: "the shot feels instant" cannot be judged with no Worker, tick or server); GE-8 (step 28: record Mustafa's PC's
  `devicePixelRatio`); GE-9 (PM-0003-6: "GPU time above 0 in 600 of 600 frames" passes on stale values, fails on a
  disjoint frame); GE-10 (PM-0005-34: a `.gltf` file's external buffers and images); GE-11 (PM-0005-42: reading colours
  from materials does not prove each backend draws them); GE-14 (PM-0003-2: WebGPU validation errors may not reach the
  console: a question, unverified).
- From review-rust-reviewer.md (its ADR 0004 pass-mark review, APPROVE WITH NOTES): RR-1's bite (PM-0004-23); RR-4's
  burst bite (PM-0004-6: fill a pool past its capacity); RR-11 (PM-0004-21: the state hash and serializer must not
  depend on `usize` width); RR-12 (PM-0004-24: the finite-state check works on the release builds it rides on); RR-13
  (PM-0004-60: name a small scene; at the stress scene 10 million ticks take about 23 hours).

## rust-reviewer: ADR 0004, all but "### Pass marks"
- Medium: EA-3 (the client's prediction step as a sim crate entry point: which phases, on what state, no random stream;
  0004:151–156, :977); GD-4 and FC-1 (0004:425, 750, 751: "steal by GE8's presence count" drops LOOP_V2 §4's decided
  "holding E"; two enemy teams at one uplink: an open question, GD-STEAL); GD-6 (part 19: picks recorded with their
  tick, so OQ-0005-AB3's snowball measurements can be made; with infra-engineer's part 12); GD-7 (spawn protection,
  barracks interiors closed to enemies, the Recruit respawn: a home in ADR 0004, 0004:422–427).
- Low: SA-15 (the authority-only seed's source: the OS random source, never sent; 0004:254); FB-10 (0004:339–340 "no
  Rust navigation crate read promises determinism"); NA-8 (online, the Worker's pacer follows the server-clock
  estimate; 0004:188–193); GD-8's 0004 items (the pending offer of three hard-codes "three", 0004:433, 444–445; the
  upgrade record keyed by slot loses a late rejoiner's tracks, 0004:431; money's unit and the rebuild-split remainder;
  the pickup pool's capacity and full-pool rule).
- Questions: GD-STEAL, GD-REJOIN, GD-CARRIER, GD-BOT-LOOT (Mustafa's; open questions where ADR 0004 holds the rule).
- Plus the findings of your own step-10 review against ADR 0004's pass marks go to test-engineer, not you.
- From review-rust-reviewer.md (your own review of other drafts; these touch your text): RR-3's 0004 half (0004:472:
  a panic in the tick ends the match); RR-4 medium (0004:126–131: no pool (projectiles, active effects, pickups) has a
  capacity or an overflow rule, yet the sim must not allocate in steady state: capacities from the mode and content at
  match start, one overflow rule on every target; with game-designer's 0005:816–817); RR-9 low (0004:473–474: a trap
  carries no panic message: a small panic hook writes it where the host can read it). Question POOL-FULL (Mustafa,
  through game-designer: refuse the new item with a counter, recommended, or end the oldest early).

## engine-architect, ADR 0002, all but "### Pass marks"
- High: SA-1 (part 10: the preview job holds no token a branch can rewrite; production deploys only from main, in its
  own environment; a required reviewer on `preview` until the baseline; with infra-engineer's part 13).
- Medium: EA-1's follow (part 5: the Worker releases spatial events at the delayed time, once ADR 0001 part 7 states
  the rule); EA-8 (its own: the client endpoint in its own crate, since ADR 0001's tests run native headless clients;
  0002:183); NA-4 (0002:280–282: received datagrams reach Rust with no arrival time: 0–33 ms of false jitter); NA-7
  (0002:274: when the page posts its command; PM-0002-13 vs -64); IE-3 (OQ-0002-5: macOS runners are free on a public
  repository; real Safari costs time, not money); IE-4 (0002:530–534: a certificate hash in a static preview goes
  stale); IE-14 (the tick bench's access path, 0002:427–433).
- Low: EA-12 (its own: the supervisor's crate and the lobby question, 191–192; the block's owner section omits the
  floor layer and area id of 0004:387–388); NA-15 (the order of the client and the local server in one Worker step,
  part 6); IE-19 (failure artefacts' retention, 0002:426); FB-8 (OQ-0002-8: the ids line adds R5-XT-OQ6; "no answer on
  one side" fits R4-XT-OQ5 only); PA-13 (no budget for the Worker as a whole: its tick, heap and Wasm memory, if ADR
  0002 owns it).
- From review-graphics-engineer.md: GE-2 medium (0002:494–497: on a runner with no GPU the research puts every CI
  browser on WebGL 2, r3:159–169, 279–283: CI never runs the WebGPU backend and the `forceWebGL` run repeats the default;
  state it and add a WebGPU smoke run; with test-engineer's PM-0003-36).
- From review-rust-reviewer.md (2026-11-02 about 11:20; ADR 0002 REVISE, small): RR-1 medium (0002:356–358: the float
  lint bans only "transcendental" methods; std 1.99.1 calls `powi`, `hypot`, `cbrt`, `to_degrees`, `to_radians`
  non-deterministic and `min`/`max`/`clamp` may return either signed zero: ban every method outside ADR 0004 part 7's
  allow-list, sim helpers for min/max/clamp); RR-2 medium (0002:182, 193–196: nothing stops `room` or the client
  endpoint from reading the clock; std's `Instant::now` panics on the browser's Wasm target: timers count ticks, the
  host passes time in as a value, the lint covers `room`, `protocol` and `sim-wasm`; with netcode-architect's
  0001:348–349); RR-5 low (0002:267–271: R1-RR-OQ6 confirmed by rust-reviewer: field by field, no `unsafe`, on three
  conditions: one layout table drives the writer, the generated offsets and the layout hash; fixed-width little-endian
  integers; alternatives only if the writer's p99 exceeds 0.67 ms in Wasm on the minimum laptop, proposed); RR-6 low
  (0002:281–282: a fresh view of Wasm memory after every call); RR-7 low (0002:238, 460: determinism runs use the shipped
  file after wasm-opt; the build-flag check fails on binaryen's `--fast-math`); RR-10 low (0002:200–203: the
  same-feature check covers `devtools` and `navbake`); RR-14 low (0002:365–366: the "indices come from its own pools"
  reason is wrong for client-sent ids); RR-15 low (0002:138, 151: a dated nightly for the fuzz jobs); RR-16 low
  (0002:287–289: ts-rs and schemars derives kept out of the shipped builds).

## engine-architect, ADR 0005: the frame, content data and the asset pipeline
- Medium: EA-6 (0005:388–389 hands the archive of shipped content sets to ADR 0006, which has none: with
  infra-engineer).
- Low: FB-7 (0005:1759: "or content's share takes the Wasm file past the limit ADR 0002 sets"); FB-11 (0005:5: the
  author line reads as if graphics-engineer wrote the pass marks); FB-13 (0005:1693: "until the Advisor sets the first
  limit", as ADR 0002 says); FC-8 (0005:1797: LOOK_V2 §2 names props and houses; "§8 names none" is true of §8 only);
  PA-11 (the triangle row counts the shadow pass: about 5.2k triangles per soldier at Low, an input to the per-unit
  budget questions OQ-0005-AS*).

## graphics-engineer: ADR 0003, all but "### Pass marks", and ADR 0005's "The asset pipeline: GPU rules"
- Medium: PA-4 (0003:326–329: VISION_V2 §6.9 "without dropping frames" needs a mark; with test-engineer's row); PA-8
  (0003:318–321: "about 786, inside 800" leaves out the post-processing passes, health bars, VFX pools and projectile
  kinds); FA-L2 = B25 (0003:707: GW17 "pending removed" while its camera-player half is Mustafa's call under
  OQ-0003-15).
- Low: EA-11 (0003:370–374 audio occlusion copies the world onto the page; 928–931 the bench's Worker load; 276–277
  wording; 381 a stale hedge); PA-10 (0003:204, :514: memory per attachment single-sample though MSAA 4× is the start);
  PA-11 (the triangle row counts the shadow pass); FC-9 (0003:704: the CC10 row's outcome reads as LOOK_V2 §3's words).
- From review-graphics-engineer.md (2026-11-02 about 11:15; ADR 0005 REVISE): GE-1 high (0005:1587–1588 vs 1544–1547:
  "one material per class (and per faction)" vs "one material per slot content": one material per content key, as
  legacy shared materials by full content, LEGACY_INVENTORY line 113; with test-engineer's PM-0005-35); GE-4 medium
  (0003:309, 199: the Low tier may switch shadows and AO off against LOOK_V2 §2's decided soft shadows: its Q1, an open
  question, decided after the look proof test's per-technique costs).

## game-designer: ADR 0005's "The ability and effect system"
- (none yet from the six reviews; the rust-reviewer, graphics-engineer and art-director reviews may add some)
- From review-rust-reviewer.md: RR-4's 0005 half (0005:816–817: the active-effect pool's capacity and overflow rule;
  with rust-reviewer's 0004 part; POOL-FULL is Mustafa's question).

## engine-architect, the synthesis: ENGINE_OVERVIEW, SPIKE_PLAN, docs/adr/README.md (last, after the drafts' batches)
- FC-2 (README:269–270: G0; ADR 0006's S1–S5 collide with seam S4; B1–B11 are baseline items: skeptic-5 first); FC-3
  (SPIKE_PLAN:181 "decides R1-RR-OQ5": the Advisor sets that limit); FC-4 (SPIKE_PLAN:282 cites ADR 0006 part 1 for the
  client-host token: part 13, item B9); FC-5 (ENGINE_OVERVIEW:56–57 labels omit §6 items the table gives; :40, :52
  "draw the newest block" while ADR 0003 part 10 interpolates between the two newest); FC-6 (SPIKE_PLAN:191 "seven
  targets" lists eight runtimes); IE-2 (SPIKE_PLAN:100–101 "read the same in both tables"); NA-17 (ENGINE_OVERVIEW:9–10:
  in local play the Worker is both the client endpoint and the local server); PA's note (ENGINE_OVERVIEW:112, §6.9,
  holds once PA-4 lands).
- Then: a README row for every new open question the drafts' final batches add that routes to Mustafa or the Advisor,
  and every review question listed below; counts again; draft_pointers.py "off 0".
- From review-graphics-engineer.md: GE-5 medium (SPIKE_PLAN:235–241: nobody named to make the look test's authored
  models in wave 1, no route; Spline's skinned glTF export with named slots is not researched: its Q2); GE-12
  (ENGINE_OVERVIEW:40, :92–94, the same as FC-5); GE-13 (SPIKE_PLAN:174, :238: no one named to build the client harness
  the three wave-1 spikes share). Questions for README: GE Q1 (does every decided LOOK_V2 item hold at Low?), GE Q2
  (who makes the look test's models, and by which route?).

