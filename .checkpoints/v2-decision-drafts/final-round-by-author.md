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
- From review-test-engineer.md (2026-11-02 about 11:24): TE-1 high's 0001 part (0001:859–869, OQ-0001-2 (a): the sim
  feeding the wave-1 60 Hz margin run has no ability or effect step, economy, objectives or moments: each missing part
  burns its time ceiling in the same run, or the verdict waits for the real effect step; its Q1 for the Advisor; with
  rust-reviewer's S1 and the synthesis's SPIKE_PLAN); TE-3 medium (0001:955–956: "one recorded input replay drives both
  transports to identical server state hashes" depends on timing while OQ-0001-6 is open: a run counts only when the
  late-command counter is 0); TE-4 medium (0001:561, :192: only the 60 Hz soak proves counter wraps; the 30 Hz soak's
  54,000 ticks never reach 65,536; the sequence-number wrap has no proof: a deterministic gate test that starts the
  counters just below their wrap points).

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
- From review-test-engineer.md: TE-2 medium (0006:764–772: rooms per VM measured on the proof test's stand-in sim
  "fixes part 8's goal budget"; a lighter sim overstates density: the stand-in burns the sim step share's ceiling, and
  the budget stays provisional until slice 6; its Q2 for Mustafa on the Advisor's advice).

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
- From review-test-engineer.md (your own review): PM-0001-30 follows TE-3 (a run counts only when the late-command
  counter is 0); a deterministic counter-wrap row follows TE-4, once netcode-architect writes it.
- Routed by rust-reviewer's final batch (b815c85): PM-0002-33 checks that the panic message arrives; RR-3's server mark
  (a panic in the tick ends the match, the process exits non-zero) goes beside PM-0001-26.
- Routed by infra-engineer's final batch (ADR 0006 at 8fa7173): the PM-0002-66 bite (a test PR that edits the deploy
  job cannot reach the production token); PM-0001-25 and PM-0001-29 follow ADR 0006's load-generator key (a
  calibration run; the per-address cap raised for the run).
- Routed by engine-architect's ADR 0002 batch (its NOTEs name what each row needs): PM-0002-6, -9, -10, -29, -41, -44,
  -51, -64, -66, -76, and a reported Worker-budget row (PA-13).
- Routed by engine-architect's ADR 0002 batch, second part (61e9933): PM-0002-42 (G3: a planted `eval` in `client`),
  PM-0002-44 (G5: a planted git dependency), PM-0002-46 (G8: a planted inline script in the built HTML) need bites.
- Routed by netcode-architect's ADR 0001 final batch (75615d1; its NOTEs in netcode-architect-fixes-final.md say what
  each row needs): PM-0001-12, -13, -21 (the clamp; strictly newer ticks), -25, -29, -30, -39, -43, -44, -48, -49 (the
  completed rewind estimate), -54, and checklist step 10; new rows: own cues play once per shot; a tick-thread panic ends
  the match; counter wraps from just below their wrap points; 0 corrections from a pick or an effect.

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
- From review-art-director.md (the look checklist and the fail route are yours): AD-1's fail route (0003:1028–1030:
  never "move it to High only" for a [DECIDED] item: follow graphics-engineer's floor or LOOK-FLOOR); AD-2 high
  (0003:1051–1052: the checklist runs only on Mustafa's PC at High: a step showing the Low frame, ratio 1, scale 0.75,
  1024² shadows or none, AO off, beside High and legacy); AD-3 medium (no step shows the busy fight: 48 soldiers, 3
  teams, 4 very large units, 300 projectiles, at day and at dusk); AD-4 (aim step 4 at the ground-contact line); AD-5
  medium (0003:1052–1053, steps 2, 8, 9; 0005:2153–2156: "the game camera" and the closest/farthest framing undefined
  while the framing is open, R2-INT-OQ2: use legacy's camera as the test view, height 40, boom 56.6, fov 15, yaw 45°);
  AD-8 low (0003:1053: legacy screenshots taken in Brave on Mustafa's PC, not on the container's CPU renderer); AD-9 low
  (step 13: the animation LOD picks soldiers by importance, not distance); AD-12 low (0003:1084–1085: the trees are
  legacy's ported tree builders, GW18, judged for variety only); AD-14 (steps 10–12: the soldier shown is built at the
  4-material cap).
- From review-test-engineer.md: PM-0003-3 follows TE-5 (compare with every kept effect on).
- Routed by graphics-engineer's final batch (319d3b6; its NOTEs in graphics-engineer-fixes-final.md): PM-0003-3 drops the
  one-backend exclusion; PM-0003-9: the halftone overlay as frame time, memory per attachment at both sample counts;
  PM-0003-12 points to part 7's 799 to 817; PM-0003-16 records whether `info.memory.total` counts MSAA textures;
  PM-0003-23: "1024² (none only under OQ-0003-16)"; a spectacle row for OQ-0003-17 (part 7's presented-frame count);
  "When a mark fails": a [DECIDED] item's technique leaves Low only under OQ-0003-16; a Low-frame checklist step (steps
  2, 4, 6, 7, 20), the tier named per step; steps 4 (the contact line), 10–12 (the soldier at the 4-material cap) and 27
  (the flash over the HUD); BS records the Worker's load; PM-0005-34 and -35 count distinct slot contents.
- Routed by rust-reviewer's final batch (b815c85): PM-0004-29 names the prediction step (part 3's second entry point);
  PM-0004-68 reads jitter online against the server-time estimate's schedule; PM-0004-61 checks that the panic message
  arrives (the hook's fixed buffer); PM-0004-6 gets the pool-burst bite; PM-0004-2 counts the stand-in effect rows (384
  plus a 1 ms burn at 30 Hz; at 60 Hz 1 ms and 0.30 ms); PM-0004-65 starts money at $300; the "no measurable mark" lines
  for parts 17 and 19 point to the test plan and migration phase 6.
- Routed by infra-engineer's final batch (8fa7173): PM-0006-15 checks the firewall rule list per VM role; the LG key
  (a calibration run sizes the load generator; the cap raised); PM-0006-11 created after PM-0006-12; PM-0006-14 uses the
  padded stand-in (the sim step padded to 8.33 ms); a new row for spike 1's B9 test (the preview token cannot publish
  production); the seed added to PM-0006-21's canary (never logged or recorded).
- Routed by engine-architect's ADR 0002 batch: PM-0003-36 (the WebGPU smoke run, GE-2), PM-0004-23 and PM-0004-74 (the
  float lint's allow-list and the same-feature check, RR-1, RR-10); its NOTEs name what each row needs.
- Routed by rust-reviewer's follow-up b (ADR 0004 at 16e97aa): PM-0004-23 reads "the build fails on any std `f32` or
  `f64` method outside part 7's allow-list (`powi`, `hypot`, `cbrt`, `to_degrees`, `to_radians`, `min`, `max` and
  `clamp` named), as well as the items listed today"; its bites add `deg.to_radians()` and `a.min(b)` in `sim`, each
  failing G2, as in PM-0002-41; its control adds the three helpers and one allowed method (`abs` or `fract`) passing.
  Optional for PM-0004-22: the helpers' signed-zero results and a reversed clamp in the unit vectors on every target.
- Routed by engine-architect's ADR 0005 frame batch (b88714d): PM-0005-4 or -5 gains the pool-report leg (bite: a
  projectile lifetime doubled while the report stays the same).

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
- From review-test-engineer.md: TE-1's 0004 part (0004:826, S1: PM-0004-2 demands 384 active effects, which no wave-1 S1
  run has: the missing parts burn their ceilings, or say which wave-1 runs are valid); TE-6 low (0004:422, :454–458:
  parts 17 and 19 have no spike claim, test plan line or milestone: planted divergences that change only an event and
  only an economy value).
- Follow-up (routed by engine-architect's ADR 0002 batch): RR-1's ADR 0004 half: part 7's allow-list of float methods
  grows to what ADR 0002's lint now allows, and the sim gets helpers for `min`, `max` and `clamp` (signed zero); not in
  b815c85. A small batch: rust-reviewer-fixes-final-b.md, BEFORE at b815c85.
- Deferred (optional wording; correct as they stand), routed by game-designer's ability batch: OQ-0004-11's "What decides
  it" may add "ADR 0005 AB-1 holds the game-designer's proposal per kind"; part 17's "an effect in ADR 0005's data" may
  read "… (AB-6, read by AB-2's immunity rule; its scope is OQ-0005-AB14)". For the revision after the stop review.

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
- From review-test-engineer.md: TE-7 low (0002:416–421 vs :435: the nightly soak on shared CI runners is "measured …
  against the server tick budget", which the draft's own reason rules out; PM-0002-72 limits that mark to the reference
  server; say so in the decision text; name the machine of the 3-hour soak before each playtest).

## engine-architect, ADR 0005: the frame, content data and the asset pipeline
- Medium: EA-6 (0005:388–389 hands the archive of shipped content sets to ADR 0006, which has none: with
  infra-engineer).
- Low: FB-7 (0005:1759: "or content's share takes the Wasm file past the limit ADR 0002 sets"); FB-11 (0005:5: the
  author line reads as if graphics-engineer wrote the pass marks); FB-13 (0005:1693: "until the Advisor sets the first
  limit", as ADR 0002 says); FC-8 (0005:1797: LOOK_V2 §2 names props and houses; "§8 names none" is true of §8 only);
  PA-11 (the triangle row counts the shadow pass: about 5.2k triangles per soldier at Low, an input to the per-unit
  budget questions OQ-0005-AS*).
- Routed by graphics-engineer's final batch: AP-3's example list of material classes gains "structure" (the GPU
  section's part 2 now has a structure class, AD-6).

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
- From review-art-director.md (2026-11-02 about 11:22; ADR 0003 REVISE; 0005's asset parts and the look test APPROVE
  WITH NOTES): AD-1 high (0003 Rollback, about :1193–1194 "a tier change in data moves it to High only", with :163–164,
  :191, :212/:309, :665: the ink and soft shadows, LOOK_V2 [DECIDED], could drop out of the default Low tier with no
  look call by Mustafa: a look floor, lower them but never switch them off, or an open question LOOK-FLOOR for Mustafa;
  with GE-4 and test-engineer's fail route); AD-4 medium (0003:177–179, 187–190: screen-space ink finds no depth jump
  where buildings and boots meet the ground, so that line comes out at the thin inner weight: name it as a visible
  difference; an inference, not research); AD-6 medium (0003:220 with 0005:1553–1554: no material class for buildings,
  so the heaviest building ink has no input: a structure class or an ink-weight field per asset); AD-7 medium (0003:398,
  0005:1591–1592: the test assets hold one building while checklist step 14 shows both [DECIDED] houses); AD-10 low
  (0003:250–253: the halftone overlay's cost is invisible to GPU timers and it also dots the HUD, ART_DIRECTION.md:121);
  AD-11 low (0003:189, :316: ink width scales with pixel ratio × resolution scale); AD-14 low (0005:1569–1571: the cap of
  4 materials per soldier bounds its surface kinds). Questions: LOOK-FLOOR, REDUCED-MOTION, COLOUR-TELL (Mustafa's).
- From review-test-engineer.md: TE-5 medium (0003:845: "effects on one backend only are off for this test" contradicts
  part 1, 0003:125, one feature set on both backends: compare with every kept effect on; test-engineer's PM-0003-3
  follows).

## game-designer: ADR 0005's "The ability and effect system"
- (none yet from the six reviews; the rust-reviewer, graphics-engineer and art-director reviews may add some)
- From review-rust-reviewer.md: RR-4's 0005 half (0005:816–817: the active-effect pool's capacity and overflow rule;
  with rust-reviewer's 0004 part; POOL-FULL is Mustafa's question).
- From review-test-engineer.md: TE-8 low (0005:1256, :1277: searching the code for the ability ids is a weak proxy; the
  real proof is the test-only combination that works with zero Rust changes).
- Routed by rust-reviewer's final batch (ADR 0004 at b815c85): AB-1's effect overflow points to OQ-0004-11 (which row
  gives way when a pool is full; POOL-FULL: you propose, Mustafa confirms); AB8 says what a full offer queue does (offer
  size and queue are now data in ADR 0004); AB10 cites OQ-0004-10 (a late rejoiner's record) and OQ-0004-12 (do bots
  take pickups); check that AB-1 has an operation for spawn protection (ADR 0004 part 17 now gives 3 s of protection as
  an effect).

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
- From review-art-director.md: AD-7 (SPIKE_PLAN:236–237: the look test's assets list one building; step 14 shows both
  houses); AD-13 (SPIKE_PLAN:222: "each [DECIDED] item" leaves out the [DEFAULT] items the checklist also judges).
  Questions for README: LOOK-FLOOR, REDUCED-MOTION, COLOUR-TELL (Mustafa's).
- From review-test-engineer.md (SPIKE_PLAN REVISE): TE-1 high (SPIKE_PLAN:205–206, :147–150: the wave-1 60 Hz margin
  run's sim lacks the ability and effect step, economy, objectives and moments, which arrive with the wave-3 content and
  ability spike, SPIKE_PLAN:303: follow ADR 0001's and 0004's new text, each missing part burning its ceiling or the
  verdict waiting); README rows for TE Q1 (the Advisor: burn the ceilings, recommended, or wait) and TE Q2 (Mustafa on
  the Advisor's advice: the rooms-per-VM budget provisional until slice 6, recommended).
- From verifier-skeptic-5-results.md (FC-2 upheld in all three parts, low): README:269–270 reads "G0–G13"; "S1–S5"
  (ADR 0006 writes "seam S3/S4", so README's rule tells them apart); "… option labels (G1–G4, K1–K4, S1–S5) and its
  baseline items B1–B11 (part 13)"; and "for example" (ADR 0002 has its own option labels, two G1–G3 sets; ADR 0006
  cites ADR 0001's profile B1 beside its baseline item B1).
- Routed by graphics-engineer's final batch: README rows for OQ-0003-16 (the look floor; Mustafa), OQ-0003-17 (reading
  "without dropping frames"; the Advisor, confirmed by Mustafa), OQ-0003-18 (reduced motion; Mustafa), OQ-0003-19
  (telling factions apart by colour; Mustafa); SPIKE_PLAN:236–237 lists both houses; ENGINE_OVERVIEW's §6.9 row (about
  line 112) names part 7's presented-frame count (OQ-0003-17).
- Routed by rust-reviewer's final batch (b815c85): README rows for OQ-0004-8 (steals; Mustafa with the game-designer),
  -9 (is a carried cache public; Mustafa), -10 (a late rejoiner's record; Mustafa, with OQ-0005-AB10), -11 (which row
  gives way when a pool is full; the game-designer proposes, Mustafa confirms), -12 (do bots take pickups; Mustafa);
  a "Needs outside VISION_V2 §6" line: spawning, spawn protection, the Recruit respawn and the barracks mask (LOOP_V2 §2
  and §5 only); SPIKE_PLAN's 60 Hz stand-ins are 1 ms and 0.30 ms.
- Routed by infra-engineer's final batch (8fa7173): SPIKE_PLAN item 6 revokes the spike's token; README rows for
  OQ-0006-13 (the preview token and production; the Advisor with security-auditor, Mustafa for a second account), -14
  (releases; the Advisor for the model, Mustafa for the window), -15 (domain; Mustafa after the Advisor's OQ-0006-5
  read), -16 (port plan; infra-engineer with netcode-architect, the Advisor confirms), -17 (VMs for spikes and the
  bench; Mustafa with the Advisor), -18 (unreleased art; Mustafa), -19 (the provisional goal budget; Mustafa on the
  Advisor's advice); README's I-6, I-13 and lines 47 and 59 as its NOTE says.
- Routed by engine-architect's ADR 0002 batch: ENGINE_OVERVIEW lines 37 and 94 (the page posts its newest input each
  frame; the block's events released on the delayed timeline), the new `client-endpoint` crate in the overview's
  diagram and text, README rows for OQ-0002-9 (closing the deploy-job gap; Mustafa on the Advisor's advice), line 59's
  OQ-0002-5 row (macOS costs time, not money), I-13.
- Routed by engine-architect's ADR 0002 batch, second part: README rows for OQ-0006-13 and OQ-0006-17; no OQ-0002-9
  (ADR 0002 part 10 points to OQ-0006-13).
- Routed by engine-architect's ADR 0005 frame batch (b88714d): README rows for I-6 (the archive of shipped content
  sets, ADR 0006 part 12) and OQ-0005-AP1 (waits on OQ-0006-18).
- Routed by netcode-architect's ADR 0001 final batch (75615d1): README rows for OQ-0001-5 and -7 (new titles), new rows
  OQ-0001-15 (own movement one tick behind; Mustafa by feel with netcode-architect), -16 (the profiles' direction split;
  the Advisor), -17 (does a host's kick bar the player; Mustafa), -18 (ROOM-CRASH; the Advisor); I-11 "host and both
  ports"; ENGINE_OVERVIEW lines 37 and 94 (events released when their entity is drawn; the newest input posted each
  frame); SPIKE_PLAN reads OQ-0001-2 (a)'s stand-ins (ADR 0004 S1: 384 effect rows plus a 1 ms burn at 30 Hz; 1 ms and
  0.30 ms at 60 Hz).
- Deferred (optional, noted for the report): ADR 0004 part 20 could name OQ-0001-18 instead of "the Advisor's question
  in ADR 0001"; the pointer is correct as it stands.
- Routed by game-designer's ability batch (d57dc5a): README rows for OQ-0005-AB14 (spawn protection's scope; Mustafa)
  and OQ-0005-AB15 (who sees picks; Mustafa); ADR 0005's row count +2 (AB1–AB15); "Needs outside VISION_V2 §6" gains
  "ADR 0005: AB-2's immunity rule for spawn protection (LOOP_V2 §2 only)"; POOL-FULL is OQ-0004-11 with AB-1's proposal.
- Routed by test-engineer A and B (final batches, applied 2026-11-02 about 20:40): SPIKE_PLAN:75 and :99 change
  "PM-0001-1 to -57" to "-62"; :77 and :166 add PM-0002-77 and -78; the new rows PM-0003-44 to -47, PM-0004-75 and
  PM-0006-32 where SPIKE_PLAN lists a spike's pass marks; README rows for OQ-0003-20 (HEAT-SOAK; the Advisor) and
  OQ-0006-20 (the spike's bill cap; Mustafa), once their authors adopt them.
- Deferred (noted for the report): ADR 0002 part 4 writes the chosen `[profile.release]` settings once the boundary
  spike measures them (RR-8's part 4 half; a future action); ADR 0003 part 10 states event play-time only if EA-1's
  one-frame bound is wanted (PM-0001-61 is set at one tick plus one frame, 50 ms, a reasoned NOTE).

