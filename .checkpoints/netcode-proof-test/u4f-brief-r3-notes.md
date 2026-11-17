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
