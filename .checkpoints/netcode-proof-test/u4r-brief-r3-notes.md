# U4r brief rev 3: test-engineer's notes (workflow draft step, 2026-11-13)

SLIP (R6, df-ruling :317-318): in the draft step I wrote one read-only copy of df8975b's plan to the scratchpad with a
shell redirect (`git show ... > plan-df.md`), against the write rule; details at B3a below. None in the revision.

Deliverable: /home/user/Shamble-Frontlines/.checkpoints/netcode-proof-test/u4r-brief.md (rev 3).
Head read: (filled in block 1)

## Plan (blocks; a line after each when done)
- B1. Read rev 2 (ed973c9:.checkpoints/npt-u4r-brief/u4r-brief-draft.md), its check, rev 2 notes. Record its
  sections, R1-R34, Q1-Q12, D1-D12 in short here.
- B2. Read the models: u4d2-brief.md (section 0, section 4, 7.2, the verbatim paragraph), u4e-brief.md (rev 3.3 form).
- B3. Read u4-build-plan.md §5 U4r (and other sections rev 2 cites), plan.md (U7 row, C-U7-2), u4s-carries.md §U4r.
- B4. Read the advisor rulings naming U4r (grep), and the ones the task names; list each applied change.
- B5. Walk R1-R34 at the head (code reads); add rows for merged work since df8975b.
- B6. D-items: fill D1, D2, D3, D5, D6, D8, D10, D11, D12 from greps; D4, D7, D9 stay with commands.
- B7. Write the brief (Write, in parts if needed), then a self-check (cites, placeholders, change list).
- B8. Return summary through SubagentHandback.

## Block log
- B1 done. Rev 2 read whole (sections 0-11; R1-R34; Q1-Q12 all open; D1-D12). Check (16 findings) and rev 2 notes
  read (heads). Head at start ea7bf27, then fa8e335 (the main session's checkpoint commit: only .checkpoints/ changed);
  `git diff --quiet cc8595d HEAD -- spikes/netcode-proof` exit 0.
- Stamps at fa8e335: u4-build-plan.md 01ede63d… (1,092 lines); plan.md 3ca4e12a… (1,518); u4s-carries.md 8b7f165e…
  (644); history.md 88bc195f… (2,011); u4r-run.sh 349b51ad…; u4d2-brief.md 146acf18…; u4e-brief.md 0fa37e8e…;
  advisor-u4df.md 842ec092…; advisor-u4j-r2.md 7efa9410…; advisor-u4d2-m9.md f31f7876…; advisor-u4d2-r2.md
  e776f0ba…; main-run.sh 89851e77…
- B2 done. Models read: u4d2-brief.md (head :1-180; sections 3-8 :560-899: the verbatim paragraph :50-52, section 0
  :54-101, 4 :648-732, 7.2 :827-876, the ruled-questions table :882-898); u4e-brief.md headers (0 :120, 7.1 :840,
  7.2 :890).
- B3a (plan) done. linemap.py (scratchpad, print-only) maps df8975b plan lines to the head. §5 U4r now :750-803 (was
  :717-772); its only text change: the -58 line is gone (moved to U4g :828-831, "judged after U4g merges ... W1-N2
  ruling and U4E-Q2"). Other plan changes U4r feels: §4.1 rows expanded (:184-202: npt-client `with_lead` "set by
  U4r's lab" (carry 74), Room/ClientSlot getters incl. `buffer_depth_ticks`, `burst_eviction`, `server_hash`,
  `overflow_resync`; Decoder `rtt_ns`, `RttSource` (Link, Pings); npt-lab `HashCompare` of/Equal/Empty/Mismatch,
  `TickHash`, param `fate_log_v1_datagram_sha256`); §4.4 header: last command run as 16-bit sequences (:280-284);
  §4.11 :393-396 (npt-lab's and npt-bots' clippy.toml with type bans and clock ban); §4.14 -18/no-fallback Loopback
  gap (:461), -58/lt-tick-offset rewritten (:470); §4.15 :494-501 (`Recording::synthetic` in the library); U4d2
  :666-713 (`begin(bite, arm_tick)`; npt-lab clippy.toml by U4D2-Q10); U4e :714-749; Lanes :850-888 (Then U4r :881).
  SLIP (own): I wrote the df8975b plan to the scratchpad with a shell redirect (`git show ... > plan-df.md`), against
  the task's write rule; later old-version reads use process substitution only. The file is a read-only copy, unused
  for any write.
- B3b (carries) done. §U4r = u4s-carries.md :255 ("## U4r (the L1 runner and its judged runs)") to :432 (the line
  before "## The next change to crates/judge"); entries 40, 49, 58, 59, 60, 62, 63, 69, 73, 74, 75, 76, 77, 82, 83,
  84, 85 (17). Choice points the carries leave: 62's last sentence (marks/ "not U4r's lane unless its brief gives it
  this one check; else the main session"); 63 ("U4r's, if its brief takes crates/log"); 84 ("or in
  ops/runs/det-u4.mjs if U4r's Evidence decision moves the step's runs there"). Lane widened by carries: 82
  (crates/lab/recordings.json, hash.rs's `manifest` and `regenerate_all`), 83-85 (check.mjs's wasm and deps steps,
  check.test.mjs, loader.mjs's header sentence, lab/src/lib.rs's TickHash doc).
- B4a rulings read: W1-N2 (wave1 :305-351: -58 out of U4r, to U4g; plan edits made); W1-U4E (:437-497: U4E-Q2 widened,
  -58 all after U4g; Q3 room BITES; Q4 tick16 in npt_proto::BITES; Q5 the no-Rewind counter counts as a replacement;
  Q6 two conditions = carry 58); W1-TARGET (:558-586; item 3 amended by m9 Block 2 :252-256); m9 Block 1 (:82-147:
  cold figure 61.6 s, "the figure is U4r's det-u4.mjs input"; over 300 s it comes as that question); m9 Block 2
  (:149-266: runner form; later lane runners copied from u4d2-run.sh keep the tag check and two lock lines :202-206;
  P3's second call may be U4r's first runner call :215-216; the two lessons :259-264); r2 Block 1 (INF2-1, carry 84),
  Block 2 (Lows to 85; loader.mjs header sentence in U4r's lane :103-104), Block 6 (:294-311, the lesson :304-308);
  df-ruling R1, R3 (NF-L1 = carry 69), R4.1 (TE2-5 reading, T4 item 2 next time), R6, T4 (:459-470: items 1-3);
  j-r2 B2.3 (:146-171; the rule :167-171), B6 (:283-298; the plant line :298); u4j "Carries" U4r (:402-406), R2.4
  (:438-449), FU-3 (:377-378), FU-6 (:384-391: "once U4r logs per-message sizes"); close B1.1-B1.5, F3 (:322-327),
  B3's placement of 58-60.
- B4b rulings read: e-ruling Block 5 (U4E-Q13 (a): `catalogue-5/owner-to-all`, class audit headers list no mark,
  :262-264, :339); U4E-Q14 Loopback gap; F2-ROUTE (carry 66 went to U4d2, met there). e-r1 R1 (:57-122: tick16's
  judged half U4r's = carry 73), R2 A5 (:157-166: `with_lead` public for U4r), R4 NC-L1 (:224-232: "-18's L1 part
  counts at U4r as planned, with no further check of mine" :231 = rev 2's condition-6 branch answered), NC-L4
  (:238-241: `stale()` sum reported, carry 77 (c)), A14 (:245-247: `event.snap_refused.tick_count` row in SERIES.md),
  R5 NC-L9/L10/L2 = carry 77 (:282-285). e-r2: NC2-M1 = C-U6-16 (a) (U6, not U4r); its M9 "U4r's brief naming carries
  73-77 and 62 (h)'s measured window" (:196). d2-brief A13 (:568-592; "U4r's brief gives npt-bots its clippy.toml
  with both bans (§4.11)" :588-589, :725). d2-r1 Block 5 TE-M3 (:287-310: B2.3's rule after U4d2's merge "read as met
  for the name only ... stays in force until TE-M3's cause is found and fixed" :308-310; TE2-5 (c) before U4r's merge
  :303-304); RR-L4 (:209-212: replay_to chunks if U4r's Evidence decision brings time into question); RR-L5 = 82.
  advisor-u4.md: only 4 lines appended at its end (:476-479); every rev 2 cite holds.
- Carries cites: carry 40 now :256-264 (was :176-182); carry 21 :148-153 (was :146-147); U4h section :548-560; carry 6
  :72-75, 14 :116-120, 20 :135-136 (to check).
- B5a code (l1, server pump, room rewind):
  - R1 holds: `L1Net::next_event_ns(&self) -> Option<u64>` l1/src/lib.rs:1880-1882 (doc :1876-1879, L-c5).
  - R2 CHANGED vs the plan's words: `rtt_ns` is `None` until the first sample (doc :64; `End::rtt_ns` :1748-1750
    reads `smoothed`, an Option); 333 ms (INITIAL_RTT_NS :118) is the internal start for the PTO only
    (`smoothed_ns` :633-635, `Rtt::START` :612-616). The first sample sets smoothed (:619-631). So before its first
    sample an L1 session's `Decoder::rtt_ns` is None, not 333 ms (plan :782 says "L1 starts at U4c's 333 ms").
  - R3: acks at arrival (doc :27-30; `send_ack` :1418-1465 at the arrival instant `t`, delay drawn the other way);
    events applied in (time, index) at every call that takes now (doc :65-68; `advance` :947-959). L-c6's test name:
    still to read (tests.rs).
  - R4: no accessor for "first sample instant"; the lab reads `Link::rtt_ns()` (or `Decoder::rtt_ns(link)`) after each
    call: Some = has a sample. At P0 the ack of a packet sent at T is an event at T (delay 0), applied at the next
    call with now >= T after the send (`advance` at `enter`; `leave` advances only under the sends-before-events
    bite :941-945). The sample is 0 (arrival minus send). Delivery to the application is never in the send instant
    (`poll` :1667-1681: `gate_ns < now_ns`).
  - R7 holds with new lines: p0 :178-180, p2 :186-188, p3 :190-192, b1 :198-202 (1 s bursts, P2's base); every
    profile has ReorderStep::PART_13 (`named` :166-175); P0 finds nothing to swap (doc :20-22).
  - R6 holds: `with_bite(seed, bite) -> Option<L1Net>` :1828-1840; `L1Net::BITES` :1823-1824 (4 ids: BITE_IDS
    :752-758 incl. PM-0001-3/clock-seed); model bites unit-test only (:764-773).
  - R15: `Decoder::pump(&mut self, link: &mut dyn Link, now_ns, tick: u32, out: &mut Vec<(Message, u64)>)`
    pump.rs:331-447; it never reads an RTT; `Decoder::new(quant, tick_hz, accept_ns, version, content_hash, rtt:
    RttSource)` :224-247; `RttSource::{Link, Pings}` :183-190; `Decoder::rtt_ns(&self, link) -> Option<u64>`
    :295-303 (Link => link.rtt_ns(); Pings => min of samples). The caller sets the kind. The pump sends pings when
    Joined (:346-363) on every session, L1 and Loopback included; pong samples only feed Pings sessions.
  - R19/R33 hold: rewind.rs:18-33 `rewind_at -> Option<Rewind>`, None for an out-of-ring view time;
    `ClientCounters::rewinds_refused` room.rs:133-135 ("Commands holding or pressing fire whose view time the ring did
    not hold ... a replacement in -30's setup condition (carry 40)"); per tick `TickValues.rewinds_refused: Vec<u16>`
    :155. D10's greps: no RTT reader in crates/room/src or crates/client/src; in crates/server/src only pump.rs's
    RttSource and getter; no tolerance, clip or estimate path in crates/room/src (rewind.rs:2: "The tolerance and the
    cap are U4h's").
- B5b code (room, client, sim, lab, bots, check.mjs):
  - R13: `Room::new(sim, scene, greybox, tokens: Vec<[u8;16]>, n, tick_hz)` room.rs:431-478 (first tick = the tick
    after the World's hit volumes' tick :424-426, :442); builders with_event_cap :481, with_event_start :488,
    with_hashes :494, with_cover :501, with_bite(&str) -> Option (own ids or npt-proto's Quant) :510-518;
    `npt_room::BITES` [7] lib.rs:37-46 (no-fallback, lt-ack, lt-cmd-seq, lt-press, server-echo, unbounded,
    catalogue-5/owner-to-all). HISTORY_MS 2,000 provisional until U4h :33, :446. Event cap PROVISIONAL_CAP 1,024
    events.rs:40.
  - R14: tokens `Vec<[u8; 16]>` into Room::new (Roster::new(tokens, n) :447).
  - R17 CHANGED: option B has no depth setting; `buffer_depth_ticks` is measured (how far ahead the newest buffered
    command was at the last take: input.rs:28-29, :60-63, :142-145; ClientSlot :289-292); the buffer keeps at most
    CMD_CATCHUP = REDUNDANCY_CAP = 8 commands (input.rs:56-58, :118-120; limits.rs:17). Late: offered for a tick
    already run or already buffered (input.rs:111-117, counted room.rs:797) or dropped at take (input.rs:131-134,
    room.rs:846). The forward compare comes first (input.rs:84-89): a command whose sequence is not after the newest
    received is a Repeat, never Late. So rev 2's late-command bite (hold T, send T+1.. on time, send T after its tick)
    gives Repeat, `late` 0, and a mismatch (option B reruns T-1's command at T on that leg): the bite must send no
    later command before the delayed one. NEW FINDING for 7.2.
  - R16 CHANGED: `ClientEndpoint::input(command, received, now)` stamps tick = newest + lead and seq = its own counter
    (client/src/lib.rs:390-400), so a replay client cannot send the recording's ticks and sequences through it;
    StepOut {datagrams, reliable, next_due_ns, hashes, lost, wraps} :189-202; no accessor for decoded records or held
    floats (pub fns :253-365), so -57's pairing must read what each snapshot carried elsewhere (the room's sent bytes,
    decoded by npt-proto; EntityDelta carries absolute grid integers per changed field, snapshot.rs:62-77). with_lead
    :291-295 (default 1 :262), with_starts(start, event_start) :298-302, with_bite :308-320; BITES [3] :28-32.
  - R34 CHANGED: npt-client never sends `Resync` (git grep Resync in crates/client/src: no hit); FullStates after the
    join come only from the room's overflow resync (room.rs: `full_state_due` forced; ClientSlot::overflow_resync
    :253). The client files a FullState's hash like a snapshot's (full_state :775-800), so StepOut cannot tell them
    apart; the lab sees a FullState by its channel. Room's Resync handling with the limit (room.rs:713-741;
    resyncs_limited :107-110 = NC2-M1/C-U6-16, U6's).
  - R30 holds/answered: `Recording { scene, seed, first_tick, span_ms, generator (0 for a runner's), ticks }`
    replay.rs:41-54, header 15 words :7-15; `World::new(scene, seed, first_tick)` world.rs:701;
    `SimCore::state_hash(&self) -> u64` seam.rs:551 (via `room.sim()` room.rs:563); `room.tick_input()` :579 = the
    tick's whole input (a recording's tick). GENERATOR 2 :288.
  - R11/D5: `git diff df8975b HEAD` of lab/Cargo.toml and bots/Cargo.toml: empty. npt-lab's deps: link, l1, sim, proto,
    room, server, client, log, serde_json, sha2 (no npt-bots); npt-bots' deps: link, proto, client, net, log (no sim;
    npt-proto re-exports no npt-sim item: proto lib.rs:31-44). NEW for Q1: npt-bots names npt-net
    (bots/Cargo.toml:24), so npt-lab -> npt-bots brings npt-net's tree into npt-lab.
  - npt-lab clippy.toml exists (U4d2; both bans + clock ban); npt-bots has none (ls crates/*/clippy.toml).
  - lab lib.rs: TickHash {recording, tick, hash} :8-17 (struct doc :8 still "the recording (or run) it replays" =
    carry 83 (a)); HashCompare {Equal{entries}, Empty, Mismatch{index, ours, theirs, compared, mismatching}} :19-39;
    `of` :43-61 (a shorter list: Mismatch at the first missing index, partner None).
  - check.mjs: STEPS :2468-2503 (11: fmt, clippy, clippy-wasm32, doc, test, deps, wasm, refs, marks, self, redact);
    main's last line :2548; LEAF :39 (npt-hash-wasm in it); WASM_CRATES :1891; lintBothBuilds :1935;
    testBothBuilds :1973-1982; cargoCounts :1864; BITE_ID :879; biteHomeProblems :914-924; "nothing depends on
    npt-lab" :175-178; depsStep :2062; RECORDINGS_MANIFEST :2099; WASM_BUILDS :2120-2140; WASM_CALL_LIMIT_MS
    :2212 (per call: carry 84); wasmStep :2254.
- B5c code (rest) done:
  - R3 L-c6 `lc6_at_one_instant_internal_events_come_before_application_sends` l1/src/tests.rs:1432 (body :1344);
    L-c7 :1561 (body :1445); L-c5 :1043. R5 tag `npt-l1 fate log v2` l1/src/lib.rs:330, for_each_canonical :351;
    probe's fate_log_sha256 probe.rs:47, header params main.rs:120-123 (+ v1 datagram hash, carry 66 met at U4d2).
  - lab main.rs: doc :1-14 (exit 2 on usage :12-13), dispatch hash/probe :195-235; tick_hz literal 30 :159 (was
    :151); burst_starts_ns :142. R31 `Schedule::new(tick_hz, catchup)` schedule.rs:77; npt_server::BITES [3] :251,
    `Schedule::with_bite(tick_hz, catchup, bite)` :257. R32 `Quant::with_bite` quant.rs:102-112; npt_proto::BITES [7]
    :34-42 (-57 x3, -17/tick16, -58/tick16, -21/no-trailing, -21/no-channel). World::with_bite(scene, seed,
    first_tick, bite, arm_tick) world.rs:944; World::BITES [4] :937-938.
  - Proto ids reach every wire instance: Room::with_bite forwards to Quant (room.rs:510-518), ClientEndpoint::with_bite
    too (client :308-320), Decoder::new takes a Quant (pump.rs:224).
  - D6: -17/tick16 quant.rs:38, -17/client-rounding client :29; -18/no-fallback room lib.rs:39; -57 x3 quant.rs:35-37;
    -58: tick16 quant.rs:39, lt-ack/lt-cmd-seq/lt-press room lib.rs:40-42, lt-tick-offset/lt-event-seq client :30-31;
    catalogue-5/owner-to-all room lib.rs:45; PM-0001-30/*: planted nowhere.
  - marks.json at head: -17 :3845 (parts L1 P0/P2/P3/B1 select layer, scene SS, profile, clients 48; part validity
    pre:PM-0001-2; tick_start setup check only on 'every S30'; bites :3991, :3995); -57 :4008 (parts by profile only,
    no scene/clients; checks pos/yaw/pitch; bites :4089-4097); -58 :4107; -18 :4262 (parts L1 B1 SS 48, L2 B1 wt, L2
    B1 ws; series event.burst_heal.after_link_ns; >= 480 records; notes :4442 hold the fresh-delta reading; bites
    no-fallback :4427, ws-budget-x2 :4431); -30 :6512 (validity late-zero; report buffer_depth_ticks_count :6557;
    bites wallclock :6563, late-command :6567).
  - SERIES.md (763 lines): rows exist for pos/yaw/pitch err (:67, :73, :66), counter_wrap (:150), burst_heal (:134,
    :323), burst_eviction (:321), snap_incomplete (:320), late_count (:365), hs rows (:317-319), snap_refused (:218),
    buffer_depth_ticks_count (:240), tick_start (:252), ticks_below_wrap (:253), suite (:376), stand_ins (:374),
    cmd_timing (:375), burst_starts_ns (:403), fate_log_v1 (:398), snap_datagram_bytes/snap_wire_bytes (:70, :72;
    writer npt-server). No row: admit (an event, README :80), event rate / queue high-water (carry 58), overflow
    resync, the RTT rule's instants, admit tick, FullState flag, lead, weapon/health params, input payload and the
    Join/JoinAccepted/FullState sizes.
  - README §1 :8-86 (`h` per tick = state hash :15; `loopback` layer :36; `hs` :44-79; admit event :80); §4 :247-316
    (complete :250, roster :258, late-zero :275). npt-log: TickRecord set_hash :144, set_hashes :158; Writer :338-412;
    check_header :305 (carry 63's cite holds); read_log :454.
  - judge eval.rs at head: select by marks :32 (was :30); gap :314-327 (was :310-323); run judging :1165-1168 (was
    :1048-1051); expect-invalid :1412 (was :1283); C4 :539-570. figures.rs decoded_hash_mismatch_count :542.
  - catalogue: :73, :125-158, :144-145, :150, :153-154, :237-238 hold; §6 item 5 now :251-254 (was :249-252); Q8 now
    :284 (was :282); Resync row :90 (gains the limit, T7).
  - plan.md: §4.2 :421-454 holds; npt-lab §4.7 entry :617-628 (was :616-627), RQ8 order :618-624, host clock :625,
    RTT :626, bites :630-631, binaries :611-612, npt-client :609; U7 row :1247 (C-U7-1, C-U7-2: context only).
  - link/src/lib.rs unchanged: Loopback send in the instant :154-163, poll :185-190, rtt_ns Some(0) :203-205.
- B6 D-items: D1 (cc8595d counts; spikes/ = cc8595d at fa8e335), D2, D3 (11 steps; 33.1 s warm / 61.6 s cold), D5,
  D6, D8 (no clock in lab/bots sources; lab/clippy.toml bans them), D10, D11 (no Resync sender; cap 1,024
  provisional events.rs:40), D12 (P0 sample at the send instant, applied at the next net call at or after it) filled;
  D4, D7, D9 stay.
- B7 plan of the brief: Q answered: Q3 (U4D2-Q10 + A13), Q5 (W1-N2/U4E-Q2 widened). Q2 partly (manifest and the
  determinism step: carries 82-85; open: SERIES.md rows and carry 62 (h)'s marks.json check). New Q13 (carry 63:
  crates/log or not). Others open with updated facts. New findings: R16 (replay client cannot use
  ClientEndpoint::input; -57 pairing source), R17 (late-command bite must not send a later command first), R34 (no
  client Resync), R2 (None before first sample), 8.2 item 1 (setup condition CAN fail at U4r through rewinds_refused:
  carry 40's last sentence / U4E-Q5).
- B7a written: u4r-brief.md header, change list C1-C26, verbatim paragraph, section 0 (stamps at fa8e335, moved-cite
  table), section 1. To fix at the end: C20's counts; close cites ":20-215" -> ":21-228" and ":320-327" -> ":321-327"
  (F3 at close :321). End marker `<!-- rev3-continues -->` for appends.
- B7b written: section 2 (2.1-2.7). Cites verified: room.rs:840, :855 (FIRE, PRESS_FIRE); lab/Cargo.toml:23-34;
  bots/Cargo.toml:20-25; c-u2:55, :61 (the built paths); DEPS_CHECKED check.mjs:2020; buildInputProblems :1560.
  Next: sections 3 (P1, 3.2 table, 3.3 P13-P15), 4, 5, 6.
- B7c written: sections 3 (3.1 P1; 3.2 table of 17 entries + carry 66 not U4r's; 3.3 P13-P15), 4 (rules 1-9, lane,
  P2-P5, P8-P12, TE-M3, no-retry), 5, 6 (18 items). Next: 7 (7.1 without -58; 7.2 rewritten for C6, C9-C13; 7.3
  recomputed; 7.4), 8.
- B7d written: sections 7 (7.1-7.4) and 8 (8.1-8.4). Fix at the end: lab/clippy.toml is 16 lines (":1-15" ->
  ":1-16" in C3 and 2.2). Next: 9 (table), 10 (questions + ruled table), 11, 12; then the end fixes and self-check.
- B7e written: sections 9 (R1-R34 walked: 18 hold, 16 changed; R35-R44 added), 10 (ruled table: Q3, Q5, Q2 (ii)
  and (iii); open Q1, Q2 (a)/(b), Q4, Q6-Q12, new Q13), 11 (D1-D3, D5, D6, D8, D10-D12 filled; D4, D7, D9 stay), 12
  (launch items 1-9, placeholders P1-P15).
- B7f self-check done: end fixes applied (C20 counts; close cites; clippy.toml lengths 16/17; l1 allow :768-770; C9's
  row R38); placeholders appear once in their home and once in section 12's list (the change list names them without
  angle brackets); no prose line over 120 (python print-only check); no end marker left. Brief 1,413 lines.
  Verified late: pump.rs:328-330 (returns after a Join), l1 :768-770, server/clippy.toml 17 lines.
- B8: handback. DONE.

## Revision for the check (rev 3.1, 2026-11-13)

Task: apply the check's findings (u4r-brief-r3-check.md): F-j High; F-a, F-k, F-l, F-m Medium; Lows F-n..F-s, F-g,
F-b, F-d..F-f, F-h, F-i (one-line text fixes applied, the rest listed). Edit u4r-brief.md in place; add "rev 3.1" to
its change list. No runs. A line per finding below as each is done or rejected.

Plan (blocks):
- V1. Read the brief's affected parts (2.1, 2.2, 2.4, 2.5, 3.2, 4, 5, 7.1-7.4, 10, 12) and its change list form.
- V2. Verify each finding's code cites at the head (client lib.rs, room input.rs/rewind.rs/room.rs, l1, pump.rs,
  schedule.rs, server/clippy.toml, check.mjs, hash.rs, replay.rs, proto input.rs, server/tests/common/mod.rs,
  lab/clippy.toml) and the plan/ruling cites (§4.15, §4.11, §7.1 item 6, advisor-u4df.md R6, advisor-u4d2-r1.md).
- V3. F-j: check the model's premise against the stamping code and the late rule; F-j's arithmetic (rtt/jitter per
  profile, CMD_CATCHUP); write U4R-Q14, the 2.1 pointer, the D-item.
- V4. Mediums F-a, F-k, F-l, F-m: edits.
- V5. Lows: apply the one-line ones; list the rest.
- V6. Change list rev 3.1 block; self-check (line widths, cites); notes lines; handback.

Block log:
- V1 done: the brief read whole at head e913b43 (1,412 lines; only .checkpoints/ and TE-M3's delay_tests.rs in the
  working tree changed since fa8e335). Sections hit: 2.1 :258-375, 2.2 :377-392, 2.4 :412-480, 2.5 :482-495, 3.2
  :551-575, 4 :596-681, 5 :685-703, 7.1 :755-781, 7.2 :783-904, 7.3 :906-943, 7.4 :945-961, 10 :1124-1289, 11
  :1293-1349, 12 :1353-1411.
- V3a F-j verified: stamp `tick = newest + lead` (client/src/lib.rs:390), newest moves only on a newer header (`see`
  :585-600); Profile P2 100/20, P3 150/30, B1 = P2 (l1 :176-202); `one_way_ns` = rtt/2, `half_jitter_ns` = jitter/2
  (:211-218); delay = one_way - a + U[0, 2a] (:810). Needed: d_down + wait (< 1 tick) + d_up <= lead x P, so lead >=
  (rtt + jitter)/P + 1: ceil(...) + 1 gives P0 1, P2/B1 5 (120/33.3 = 3.6), P3 7 (180/33.3 = 5.4); carry 74's form
  gives 1, 3, 4. CMD_CATCHUP 8 (input.rs:59). Holds.
  NEW (own, from the same read): input.rs:111-117 also counts LATE a command stamped for a tick already buffered.
  With jitter, a client step inside a snapshot's arrival window (width = jitter_ns: 20 ms at P2/B1, 30 ms at P3, of
  33.3 ms) sees that snapshot arrived or not by a fresh draw per step, so two steps in a row read the same newest with
  probability p(1-p) (up to 25 % at p = 1/2; analytic, about jitter/P x 1/6 averaged over phase: ~10 % P2, ~15 %
  P3) and the second command is stamped for a tick already buffered: room.rs:797 counts it late, whatever the lead.
  P0 (no jitter, rate 1) has none. So the round-trip lead alone does not meet carry 74's 1 % on P2, P3, B1: goes into
  U4R-Q14 as its part (b). Not modelled by run (no runs allowed): the D-item measures both.
  Also: before the first snapshot `newest` is None and the stamp is tick 0 (:390), late once ran_through is set: the
  bots give None until newest_tick() is Some (folded into F-r's line).
  late-zero applies to -30 only (marks.json:6544, :6570), so the duplicates make no -17 run INVALID; carry 74's 1 %
  rule is what trips.
- V2 done (cites at e913b43; spikes/ = cc8595d but TE-M3's delay_tests.rs in the working tree): client :196 (sentinel
  doc), :336 newest_tick, :379-410 input (stamp :394, view-time swap :401-406), :548-560 see, :712 (no baseline ->
  sentinel); room input.rs: forward compare :81-89, late :107-113, Full :114-116, CMD_CATCHUP :59-61, getter :63-66,
  take :129-145 (the brief's :84-89, :111-117, :56-58, :118-120, :60-63 are off by 3-4: re-pointed in rev 3.1);
  room.rs:797 (Late counted), :805-829 tick (history push :819 after the step), :831-838 scripts into fill_input,
  :603-606 take_control's channel doc, :617-630 script; rewind.rs:8-11 holds (fraction > 0 needs tick_a + 1);
  pump.rs:224-247 new (Quant only, no bite), :265-275 accept/reject; schedule.rs:240-251 BITE_IDS/BITES [&str; 3],
  re-exported server lib.rs:16; pins server/tests/schedule.rs:347 and loopback.rs:347; server/clippy.toml 17 lines,
  lab/clippy.toml 16; workspace Cargo.toml:104-106 (disallowed_* and iter_over_hash_type deny); l1 MODEL_BITE_IDS
  :756-766, allow site :768-771, with_model_bite :1840-1848; server/tests/common/mod.rs stream_of :47, accept/reject
  :272-276, send on stream_of :306; proto input.rs:47-49 (newest first, at most REDUNDANCY_CAP = 8, limits.rs:17);
  link Loopback send :154-164 (arrival = send instant), poll :185-193 (arrival <= now); check.mjs:2289-2293 (every
  manifest entry into the wasm loader); hash.rs:213-260 manifest, :292-321 regenerate_all (generator != 2 refused);
  replay.rs:50-51 (generator 0 for a runner's); npt_sim::ticks_for_ms (ticks.rs:35, rounds up); plan :500-501
  ("The determinism step keeps its three; the fourth serves the pre-check only."), :502-504, :396-401 (§4.11 bites
  per instance), §7.1 item 6 :917-918; df-ruling R6 :317-318; d2-r1 :298-299, :716-719; m9 :215-216; the card's
  INF2-4 record (P3 at 11:01 UTC with u4d2-run.sh, card lines 139-140 today; cited by item, not line). Stamps at
  e913b43 equal section 0's (plan 01ede63d, plan.md 3ca4e12a, carries 8b7f165e, history 88bc195f, u4r-run.sh 349b51ad).
- V4a edits made in the brief: title rev 3.1 and the header's "Next"; section 1 item 6 (U4e's lab as the model, F-n);
  2.1 step (2) (F-n), the four recordings (i)-(iii) (F-a), -18's decode (F-k), the lead by U4R-Q14 (F-j), carry 49's
  unit test (F-b). Next: 2.2 (F-l, F-r), 2.3 (F-h), 2.4 (F-g, r2 bullets), 2.5, 3.2, section 4, 5, 7, 10, 11, 12.
- V4b edits made: 2.2's new bullet (commands per step, None before the first snapshot and at a step whose newest did
  not move by U4R-Q14 (b); the view time newest/0 and rewinds_refused 0: F-r, F-l); 2.3 (F-h); r1 (F-g); r2's carry 74
  bullet and a new bullet (rewinds_refused, -18's bite check: F-l, F-k); r6 (rewinds_refused); 2.5's late-command row
  (F-o); 3.2 row 74. Next: section 4 (lane, Q6 wait list, R6 line), 5 (F-i, F-a), 7.1-7.4, 10 (Q6, Q14), 11 (D13),
  12, input.rs cites, change list.
- V4c edits made: section 4 lane (precheck-recordings.json), wait list (Q6 (a)'s items; only schedule.rs:347 pins
  BITES, loopback.rs:347 reads it whole: checked), Never line, R6's slip line (F-f); my notes' SLIP line moved to the
  top (R6); section 5 item 1 (F-i), item 4 (F-a cost); 7.1 preamble (lead, warm-up, after phase: F-s, F-j) and order
  (rewinds_refused stop, D13 first: F-l, F-j); 7.2 scripts and Loopback instants (F-q, F-p), L <= 7 (F-o, both
  places); 7.3 assumptions (warm-ups at most 31 runs x 30 s = 930 s: F-s); 7.4 manifests (F-a). Next: 10 (Q6, Q14),
  11 (D13), 12 (F-d, F-e, Q14, F-h), input.rs cites, change list, self-check.
- V4d edits made: U4R-Q6 rewritten (what a pump site needs; (a) with Decoder::with_bite, the allow, BITES to four and
  schedule.rs:347's pin, U4E-Q3's precedent wave1 :453-455 checked; (c)'s cost: F-m); U4R-Q14 added after Q13 ((a)
  the round-trip lead with the check's arithmetic and model numbers, cited as not re-run; (b) the duplicate stamps,
  my own reading with the arithmetic marked as arithmetic; options a1/a2, b1-b3; recommendation a1 + b1: F-j).
  Checked: client step period :450-453, l1 delay :790-813. Next: 11 (D13), 12, input.rs cites, change list.
- V5 done: D13 added (section 11) and section 6 item 5; section 12 items 2 (F-d), 3's P4 (F-e), 4 (Q14), 8 (F-h);
  input.rs cites re-pointed by replace_all (same lengths: no rewrap); change list "rev 3.1" C27-C33 added above rev
  3's. Next: self-check (line widths, the new cites, placeholders untouched), then the per-finding lines and handback.
- V6 self-check done: 0 prose lines over 120 characters (grep -P in C.utf8; 147 at 119-120, so the grep counts);
  placeholders as before (P1-P5, P8-P15 twice each: home and section 12's list; P6, P7 once); 7.2's setup condition
  gained one sentence (the recording's view times are the brawl's: F-l). Brief 1,625 lines. git status: only my two
  files and the main session's delay_tests.rs changed.

Per finding (all done; none rejected):
- F-j High: done. U4R-Q14 (a) as the fix asks (arithmetic, the check's model numbers cited as not re-run,
  recommendation yes before launch); 2.1's lead "by U4R-Q14"; r2's late report kept; D13 added. Beyond the fix, from
  my own read: U4R-Q14 (b), duplicate stamps (input.rs:110 "or one already buffered" + newest moving only on a newer
  header, client :557-560) count late at any lead under jitter, about 10-15 % by arithmetic; recommendation (b1), the
  bots skip a step whose newest did not move (2.2). Marked in C27 as added by the revision, not a run.
- F-a Medium: done. 2.1 (i) own manifest precheck-recordings.json for SS P0 s4 (chosen over a skipped key, so
  check.mjs:2289-2293 stays out of the lane); (ii) regeneration = scenario --record inside `npt-lab hash`, 3 x 9,000/R
  s (section 5 item 4); (iii) the re-pin line for U4f-U4h; lane and 7.4 follow.
- F-k Medium: done (2.1's decode definition; r2's bite check).
- F-l Medium: done (2.2's view time newest/0, never the default 0; r2, r6; 7.1's stop; 7.2's sentence).
- F-m Medium: done (U4R-Q6 rewritten; only schedule.rs:347 pins BITES, loopback.rs:347 reads it whole: so one pin,
  not two; (c)'s cost restated; section 4's wait list).
- Lows, all applied: F-n, F-o, F-p, F-q, F-r (with None before the first snapshot: the stamp is tick 0 then, client
  :394), F-s (7.3: warm-ups at most 31 runs x 30 s = 930 s; after phases unbounded here), F-g (allow site :768-771),
  F-b, F-d (the card cited by item, its P3 record), F-e, F-f (and my SLIP line moved to the notes' top), F-h, F-i.
- In passing: input.rs cites re-pointed (C33).
- Done. Handback next.
