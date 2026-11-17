# U4f brief rev 3: netcode-architect notes

Head read: 8afcc11b63906d3660b21613deb0729bdd8937bc (2026-11-17 17:15 UTC). Fresh run (no notes file before).

## Plan (blocks; a line after each when done)
- B1 Read rev 2 (34a2e55:u4f-brief-draft.md), the two checks, the drafter's and reviser's notes, the night card.
- B2 Read the sources at HEAD: u4-build-plan.md §5 U4f and its cites; u4s-carries.md (rule, U4f section, named carries);
  u4r-brief.md (form: section 0, section 4, section 12, placeholders); c62h-bound-brief.md (stops, texts-for-his-yes,
  13:35 read); history.md :2067-2075, :2125-2127, :2131-2232; CHECKPOINTS.md §11.
- B3 Rulings since 11fd190: advisor-* files changed since 11fd190; grep for U4f, U4F-, carries 46/80/86/57/74,
  "the next npt-judge"; the card's Open findings "From C62H-BOUND".
- B4 Code at HEAD: the recheck rows of rev 2 walked one by one (file:line); added rows for C62H-BOUND, U3C-HELD-1,
  REFS-KIT.
- B5 Placeholders: a python3 -I script under u4f-r3/ that checks each anchor is found once; sha256 of the 3 files.
- B6 Write rev 3 to u4f-brief.md.
- B7 Self-check, return summary.

## Block log
- B1 done: rev 2 read whole (1,117 lines: change list :12-66, counts :71-88, s0 :94-125, s1 :129-173, s2 :177-477,
  s3 :481-506, s4 :510-585, s5 :589-698, s6 :702-717, s7 :721-746, s8 recheck :750-786 (33 rows), s9 :790-810 (17
  rows), s10 Q1-Q16 :814-1020, s11 D1-D15 :1024-1042, s12 :1046-1097, NA table :1101-1117). Night card read (SC-1..3,
  NA-13, Next steps 1-4). Reviser's notes read (flag: 2.3 total-key route; limits). HEAD moved to e56c3dd while I
  read (the session's commit of this notes file); stamp the head at the end.
- B3 (part) rulings read: merge-stop (C1-C8; deferred Lows ruling; taker pooled), head check H1-H6 (ADV-CBH-1:
  `--only` must name PM-0001-2 with a pre:PM-0001-2 mark; ADV-CBH-2 hardenings: rebuild in call, hash checks with
  stop, --commit full HEAD after spike-tree check, --warmup-s 10 at 48 clients). Ten Lows: TE-CBH-3/-4/-6/-7, RR-CB-1
  (all crates/judge), NA-CBB-3 (note only), TE-CBH-5 = NA-CBB-1 (c-u4r-runs.mjs:13-15), RR-CB-2 (lab scenario.rs
  :800-805, :198-200; logs.rs:181-187 at 42b9646: U4f opens both files -> takes it, if lines hold), NA-CBB-2 (done as
  the pre-flight). U3C-HELD-1 Lows: delay.rs/delay_tests.rs/SERIES.md (RR-UH1-2 is SERIES.md :613/:642's up wording).
  Card :198-199 R4 (pack only by brief 8.1; never both P0 folders, fc411116c1f1, fd4612050cd6, c-u2 without his
  word); card :202-210 RECS-1043 read; card :4050-4084 Open findings read.
- B2 (part): history :2067-2075, :2088-2232 read. c62h-bound-paste.md read (the texts-for-his-yes form: plain
  words, exact texts in fences, planned size table, how it differs, still open, runs, freeze, Decisions needed).
  marks.json at HEAD: -20 :4644-4804 (parts A :4658, B :4672; checks :4709-4768; report :4769-4791; bite :4794;
  notes :4803); -21 :4805-5145 (parts :4816-4931; checks :4933-5074; report :5075-5086; bites :5087-5140; notes
  :5144); -22 :5146-5228 (part :5158; checks :5170-5193; report rss_bytes from process room :5208-5216; bite :5220;
  notes :5227, "U4f's report states the load each run carried"); -47 :9072-9135 (checks :9095-9112; bites
  :9121-9130). Judge: report gaps print "not available" (eval.rs:1504-1556), never INVALID (NA-2 stays Low);
  twin_hash_mismatch_count exists (figures.rs:195, :830). SERIES rows at HEAD: :21, :33, :34, :40-42, :61, :63,
  :93, :101, :128, :144-145, :149, :155-156, :158-159, :170, :240, :245, :324 (clamp_report), :344 (lead_ticks),
  :346 (cap provisional), params :415 (cmd_timing), :416 (suite), :429 (twin), :445 (event_queue_cap_count).
  NEW POINT (NA-14): SERIES :63 input_missing "no command for this tick had arrived": under option A must be read
  by stamp, never "nothing ran this tick" (bunched arrivals). Check room code.
- B2 (more): spikes/ since 11fd190 changed only: check.mjs +17, check.test.mjs +11, judge (fixtures README, two
  fixture files, figures.rs +172, tests engine.rs, fixtures.rs), l1 lib.rs +33, tests.rs, lab logs.rs +54,
  scenario.rs +311, net delay.rs/delay_tests.rs, marks README/SERIES/marks.json, c-u4r-runs.mjs +47, results/. NOT
  changed: room, client, bots, server, sim, proto, link, log, hash-wasm, c-u4r-r2.mjs, contract.rs. ADR 0001, the
  catalogue, rules/netcode.md unchanged. Plan files: u4-build-plan.md UNCHANGED since 11fd190; plan.md +1 at :453 and
  +6 at :662-667 (REFS-KIT); carries +2 at :322-323 (C62H-KEEP answered); history appended; u4r-report.md +584.
  Carries at HEAD: 18 :129-132; 21 :148-149; 27 :166-168; 39 :169-179; 50 :180-183; 57 :223-228; 61 :229-247; 58
  :269-273; 62 :289-323 ((h) :305-323); 63 :324-329; 74 :345-356; 76 :366-373; 77 :374-383; 64 :450-475; 67 :478-495;
  §U4f :507; 46 :508-511; 80 :512-536 ((a) :515-520 to "without the sort.", (b) :520-522, (c) :522-524, (d) :524-531,
  (e) :531-536); 86 :537-539. Rule :9-10.
  u4r-brief.md form read: :176-178 (placeholders `<<P...>>`), s0 :182-243 (head read, stamps, living vs closed files,
  re-point table), s4 :936-1066, s11 :1771-1841, s12 :1845-1908 (P1-P16 anchors, rules for the fill at :1857-1861).
- B4 (part) recheck walked so far: row 1 input.rs HOLDS (all cites exact). Row 14/16 client lib.rs HOLDS (fields
  :208-243, new :260-294, with_lead :296-301, accessors :343-410, input :414-445 (stamp :429, REDUNDANCY_CAP :443),
  step :450-490 (:485-488), receive :492-525 (sentinel path :508-522), too_old :528-532, old_header :538-552,
  bad_header :556-565, see :583-615, sentinel :631-641, file :645-658, rebuild_sequences :671-701 (rev 2 said
  :671-688: extend), BITES :28-32, grid_with_bite :38-40, with_bite :314-326). Row 15 full_state :810-839 HOLDS (now
  read). Row 11 pump.rs HOLDS (unchanged file): PING_INTERVAL :16, OUTSTANDING :24, SAMPLES :26, Counters :112-161,
  RttSource :183, Decoder :204, pump :372-480 (room ping sent_ns = now_ns :387-404; bucket :449-457; ping answer
  :458-472 with `tick` arg), pong :500-516 (hold clamp :510). NEW for U4F-Q6: the room's own Ping carries the
  server's exact ns (pump.rs:394-401; catalogue :91 "8 B sender's time"), arriving every 250 ms: option (e).
  Lab scenario.rs at HEAD: doc 1.5 rate :18-19; lead_for :132-137; Bites :184-208; LAB_BITES 3 ids :215-219; of
  :227; drop_slot :458-472; io :475-562 (tick :476, pump :480); send :574-601; link_lost :610-631; run :861;
  endpoint build :897-913 (with_lead :900); host offset seeded :930 (below 1,000 s); link record :1045-1060;
  client_counts footer :1259-1271 (overflow_resyncs_count :1270); fate sha :1242/:1276; params :1366-1418
  (suite :1369, cmd_timing "B" :1370, lead_ticks :1371, twin :1411); parse :1447 (twin :1488, suite :1489, role
  :1563); r8 :2212; r9 :2267; c62hb tests :2795-2990 (p2 short explained :2904).
- B4 (more): room.rs HOLDS (unchanged file): ClientCounters :63-136 (:103 overflow_resyncs; late :120), ClientSlot
  accessors :277-297, begin_tick :299-316, kick :318-323, canonical :361-369 (:367), its unit test :1399-1402, cap
  default :460-461, with_event_cap :481-484, event_cap :558-560, receive :713-742, receive_input :744-801 (ack
  NeverSent :783-788; oldest first :791; slot from session :792; repeat :795), tick :805-827, fill_input :831-878,
  route_events :979-1038 (:1023-1034), Kick path :1057-1063, newest_received :1322. events.rs HOLDS (:20, :26, :40,
  :87-92). room lib.rs BITES :38-46 (7). room/tests/room.rs: carry 21 :235 HOLDS; -22/unbounded tests CHANGED cite:
  unbounded_case :1303, bites test :1363, clean headroom test :1444 (rev 2 said :1302, :1367). proto quant.rs BITES
  :34-42, :108-109, strict :115-122 HOLD; bites.rs:211/:238, server/tests/pump.rs:477/:483 HOLD; schedule.rs:252-257
  HOLDS. bots lib.rs HOLDS (unchanged): doc rule :14-21, MOVES :30-49, FAR_CM :74-76, Kind :78-90 (Presses kind :85
  exists), Behaviour :92-112, toward_centre doc :128-132, brawl :174, script :180, every_step :193, step :213,
  command :272, dead-input test inside :362 (u4r-report.md:1139 says :404). logs.rs room_tick :524 (rev 2 :495:
  CHANGED), its lint allow :520-523, Counts :177-191 (RR-CB-2 site :181-187 HOLDS), admit :258, gone :290. RR-CB-2's
  scenario.rs sites :800-805 and :198-201 HOLD. RR-B1-7 (1) not taken by C62H-BOUND (no c62h file names it): rides
  with U4f. u4r-report.md: EVQ-LOAD :1049-1053, TE-B1-R2-1 with RR2-B1-1 :1138-1142, RR-B1-7 (1) :1163 (+1 vs rev 2).
  c-u4r-runs.mjs: stop note :13-17 ("(P0)" :14 = TE-CBH-5), split rule :36-45 ("a mark that compares runs (a twin
  figure) needs one call" :42-43: -21's measured + twins in ONE call), FLOOR :133, BYTES_10MIN_57 :153 /
  _OTHER 56 MB :154, SPLIT_LINE :167, RUN_LIMIT_MS :172, JUDGE_FIT :177, JUDGE_LINE_MIB :179, bitesOf :312,
  planJudged :327, --only listedOf :569. C62H-BOUND stop rule (c62h-bound-brief.md:599-607, his adoption
  history.md:2175): "the lab's short P2 test" shows a lost tick nothing explains -> stops for him: U4f's client
  edits (carry 80 (b), (d)) run that test in check.mjs (scenario.rs:2904).
- B4 (end): sim HOLDS (ticks.rs:12, :25, :35, :56, :70; world.rs BITES :937-938, with_bite :944, clamp :1283-1312,
  refusals :1401-1420, refuse :1445-1452; scene.rs reach :66). l1: Profile :149-163, profiles :177-202, one_way/
  half_jitter :214-221 HOLD (insert after them); NEW DatagramCounts :332, server_datagrams_since :417, with_fate_log
  :1918. check.mjs: LEAF :39 HOLDS; WASM_CRATES :1970 (rev 2 :1957: CHANGED by REFS-KIT +13); REFS-KIT
  KEPT_LOGS_FILES :1841-1862; biteHomeProblems :936; cargoCounts :1943; testBothBuilds :2052; TICK_NAMED :760.
  c-u4r-r2.mjs, contract.rs unchanged since 11fd190 (rows hold as cited). judge twin figure figures.rs:191-214
  (twin_of profile, seed, bite; role twin; every phase; a tick hashed in one only counts). Room::leave drops the
  ClientSlot whole (room.rs:689-695; Leave at :739): NEW NA-15 (2.5 must read room counters before routing a Leave).
  -47 suite vs carry 75's admission bound (scenario.rs:70, :995-1006, ~30 s of SS events): NEW NA-16.
- B5 DONE: stamps_anchors.py run at HEAD 7cfae88 (= 8afcc11 for every cited file: git diff --stat empty). plan
  43ff74f2..., 1,100 lines; plan.md 3eecd281..., 1,525; carries 57600c9c..., 682; history 3bfe1aa2..., 2,232;
  u4r-run.sh e9437ae2... (117); main-run.sh 8df2dee3... (88); u4r-report c5dee3fa... (1,764); df-ruling 842ec092...;
  j-r2 7efa9410...; m9 f31f7876...; d2-r2 e776f0ba...; te-m3 5ecc3e23...; c62h 18dd3d20...; b1-p3 1bc5889c...;
  r-b2 f4f5ac92...; r-r1 7b552e8c...; head-check a64259f5...; merge-stop 55ad3561...; c62h-review 7d2dd26b...;
  marks.json c1935a5b...; SERIES.md 1dd67e2f...; c-u4r-runs.mjs f2002890...; c-u4r-r2.mjs 4e007500....
  P1-P36 anchors each found once (lines in the script's output: P1 :507-540, P3 :223-228, P9 :180-183, P35
  :1138-1142, history P15 :2125, P16 :2127, P17 :2187, P18 :2048-2051, P19 :2141, P20 :2143-2146, P21 :2175, P22
  :2212, P23 :2216).
- B6 plan: write u4f-brief.md in four parts (W1 header, change list, counts, s0, s1; W2 s2-s3; W3 s4-s7; W4 s8-s12 and
  the NA table), each appended by Edit at the marker line `<!-- r3: next part -->`; a line here after each.
  Decisions taken for rev 3: Q17 new (carry 80 (a)'s route); Q6 (e) room pings; Q8 (iii) lead bound to him; Q1 (d)
  RR-UH1-2 leave (rec.); Q1 (a) drops rss writer; NA-14 (input_missing by stamp), NA-15 (Room::leave drops counters),
  NA-16 (-47 vs carry 75's bound); new tests f34 (NA-14), f35 (NA-15, neg); f29 asserts no refused admission; f16 the
  same tick set. Counts: measured +102, bites +117, neg 61. Bars for him: 0 by the recommendations; 1 recommended
  (the lead bound's value, -21 fast clock); 3 conditional (-21: Q15 (b), Q14 (c), Q11 (iii) (a)). Judge: none.
- B6 W1 DONE (header, change list C1-C28, counts, s0 0.1-0.5, s1). W2 DONE (s2.1-2.13, s3). Fix at the end: C24
  and the counts row say "24 hold, 9 changed" but the walk is 25 hold, 8 changed (rows 10, 19, 20, 22, 27, 28, 29,
  31 changed).
- B6 W3 DONE (s4 build rules with SIZE-CHECK table, s5.1-5.6 with the bar table B1-B7, s6, s7). Quote of the
  C62H-BOUND stop rule in 2.9 and s1 item 5 replaced by cites (no retyped adopted words). Next W4: s8 (45 rows), s9
  (9.1 open items, 9.2 deferred Lows), s10 Q1-Q17, s11 D1-D15, s12, NA table.
- B6 W4 DONE (s8 45 rows: 25 hold, 8 changed, 12 added; s9.1 16 rows, s9.2 10 rows / 23 findings, 4 taken; s10
  Q1-Q17; s11 D1-D15: 4 filled; s12 items 1-12; NA-1..NA-16). Counts rows fixed (25/8; 9.1 16 rows; 9.2 23 in 10).
- B7 DONE: brief_check.py: P1-P36 each used, no unknown id, no marker left, `@@` only in C2's mention; his 7-word
  runs appear only in s12 item 3's anchors (u4r form), so stated in 0.5; three retyped phrases rephrased (s3 C62H-KEEP,
  5.6's read line, s4 commands, s1 item 4, 2.9, 5.6 "reworded"). Head moved by autosaves to 42e0912; cited files
  unchanged since 7cfae88 (diff empty), noted in 0.1. Brief: u4f-brief.md, about 1,410 lines.
- DONE.
- Planned size estimate (B6 input): client ~700, room ~950, bots ~650, lab ~1,400, sim bites ~40, SERIES rows ~25,
  run scripts ~600: about 4,400 added lines (range 3,500-5,500); his SIZE-CHECK stop for an M unit: past 6,000.

## Revision for the checks (rev 3.1)

Task: one revision of rev 3 for the test-engineer's (TE3-1..TE3-15) and the security-auditor's (SA3-1..SA3-8) checks.
Edit u4f-brief.md in place; a "rev 3.1" block in its change list. Read the checks' notes first (done: te 82 lines, sa
96 lines). Fresh run of this section (no earlier part of it existed).

### Plan (blocks; a line after each)
- R1 TE3-1: bots/src/lib.rs:14-21; results/u4r/0f066338dd8d/P3/runs.txt; marks.json -20 checks; SERIES.md:61, :63;
  decide the honest-client rule for controller runs, f5's bounds, p5/p6's named checks.
- R2 TE3-2, SA3-2, TE3-4 (one construction): proto/src/input.rs (codec rules), client/src/lib.rs (redundancy, encode
  fail path, rebuild_sequences), room input.rs (repeat path), sim world.rs (kind 7); per-kind "rejected and counted",
  per-kind forging and twin rows; f15/f16/f33 cases.
- R3 SA3-1: input.rs ran_through and option A's lower anchor; ADR 0001:231-233; the join anchor; U4F-Q18 (past window);
  f13 (iii), f19, p20; Q8 and 12 item 6 text.
- R4 TE3-3, TE3-5, TE3-6, TE3-7: marks.json -21/-47 check names; l1 fate log form; fast-clock schedule; lab test scales.
- R5 TE3-8, SA3-5, TE3-10: 5.5 and Q2 (build-phase row, pack only u4r-P3, net figures, KL-U4F ask).
- R6 TE3-9 and the Lows (TE3-10..15, SA3-3, -4, -6, -7, -8): one-line fixes applied, others listed.
- R7 Change list rev 3.1 block, counts table, NA rows if any; self-check script (anchors, 7-word runs).
- R8 Notes per finding; handback.

### Block log
- R1 read: bots/src/lib.rs:14-21 (the rule), :224-227 (skip when newest unmoved); P3 runs.txt:18-20 (700,948 commands,
  178,595 steps without one = 20.3 %; late 1,112 = 0.16 % at lead 7); P0 runs.txt:22 (878,352, 0 without one, late 0);
  marks.json -20 checks :4710-4737 (names "at least 99.9 % of commands reach the server before their tick runs",
  "missing inputs at most 0.1 % of player-ticks"). TE3-1 holds. Fix: contiguous stamps for controller runs (each step
  gives the commands for every stamp from previous + 1 to the controller's target, at most 2; none = a hold); the old
  rule kept for `with_lead` runs; f5 restated; new f36 (client + bots: the P3 bars on a seeded trace); p5/p6 by name.
- R2 read: proto/src/input.rs:19-21, :140-198 (seq consecutive :155-157, NotCanonical; tick step 0..255 :158-159,
  OutOfRange; view offset u16 :92-95); client lib.rs:425-444 (stamp newest+lead :429, unacked :442-443), :458-470 (an
  Input every step with ack or unacked; encode fail -> nothing sent :467), :671-688 (acked dropped); room.rs:744-801
  (kind 8 returns before offer :783-788, so the forged datagram's commands are resent: both runs delay them equally);
  input.rs:80-125 (newest_seq set before refusals :89). **Code against SA3-2's fix:** the state hash DOES carry command
  sequences: world.rs:1979 (projectile seq), :1995-2009 (Shot{slot, seq}, Hit{shooter, seq}); world.rs:1922-1925 leaves
  out only the press counters. So a twin whose sequences shift by one fails on the forger's first shot: SA3-2's
  "no command at that step, sequences consecutive" is rejected; the twin keeps the forged run's sequence numbers (the
  skipped index, U4F-Q13 (b)) and leaves out of each Input the commands older than a skipped index (consecutive per
  Input); at P0 (U4F-Q11 (ii) (a)) the ack returns within a step, so none is left out (P0 runs.txt:22). Per-kind
  construction: 1, 2, 4, 5, 6 forge the newest copy only, endpoint keeps the honest one (SA3-2); later copies of a
  refused sequence are repeats whatever their content (SA3-2); kind 1 within the codec's 255-tick step; kind 2 at an
  equal tick (step 0); kind 3 an added older copy of an accepted sequence with changed content (not the tick), its twin
  the same datagram with the honest copy (TE3-2); kind 7 and the clamp forged as stored (accepted by the room: every
  copy equal, SA-3); kind 10 stamped within [t - W, t + bound]; fast clock's stamps never decrease (TE3-4).
- R3 read: input.rs:23, :107-113, :129-131 (ran_through set by take, every tick, every joined slot: room.rs:841-846, so
  option B's old stamps are Late); room.rs:681 (JoinAccepted.tick = next_tick); ADR 0001:231-239, :1159-1162. SA3-1
  holds. Fix: join anchor under both options (2.4 item 4; f13 (iii)); new U4F-Q18 (option A's past window W =
  CMD_CATCHUP, rec. (a)); f38 conditional (idle then fast, option A); p20 conditional; Q8 and 12 item 6 "bound plus W
  after an idle" under (a). Q17 (a) stays (the no-lead-bound bite run can still hold ticks 2^31 apart).
- R4 read: l1 lib.rs:353-362 (size in the canonical form), :1125-1150 (fate by flow index; size only in TooLarge and
  the entry); marks.json -21 checks :4934-5073 (names), -47 :9096-9111; c-u4r-r2.mjs:77-92 (expect lines);
  scenario.rs:2841-2843 (P2_SHORT 16 clients, 6 s + 4 s). TE3-3, -5, -6, -7 hold. -21 notes :5144 name no length.
- R5 read: card KEPT-LOGS packs (P2 572 of 779 MiB back, B1 603 of 820.5: about 73 %); card R4 (:198-199) and RECS-1043
  reading (:203-205: "B1 first, then P3"); history :2212 (PACK-READ (a): one or two closed, older folders; B1 used one);
  §11 :231-249; held-review :37, :89 ("the pack to do before U4f's counted run"). TE3-8, SA3-5, TE3-10 hold.
- R7 (part) edits DONE in the brief: title and header; the rev 3.1 change-list block (R1-R10, counts at rev 3.1); 0.5
  (P24 key); 2.2 (contiguous stamps row, bounds, pass marks, bots' rules); 2.3 (tick with none, Late under Q18 (a),
  join anchor row, per-tick cap on arrival, carry 80 (a) row note); 2.4 items 2, 3, 4, 6, kind 2 and kind 4 rows; 2.6
  (codec rules + per-kind table; catch-up row; fast-clock row: schedule, 610 s, ping_limited); 2.7 (settings, f29
  line); 2.10 (counts +107/+122/63, conditionals, generator rule, lab scale rule; f5, f12 (v), f13 (iii), f15, f16, f28,
  f29, f33; new f36, f37, f38); 2.12 (kinds 1, 2, 4-6 row with sequence-number reason; kind 3 row; open-loop note);
  2.13; section 4 no-sockets; 5.1 lengths; 5.3 (how a short run is read; named checks p1-p13; p20, p21; reports and
  stop sentence). Next: 5.5, 7, 9.1, Q2, Q4, Q6, Q7, Q8, Q11 (v), Q13, Q17, Q18 new, D3, D12, D15, 12 items 3, 6, 13.
