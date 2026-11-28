# The night-u4f scout: every input of U4f's brief, with line ranges (workflow wf_24144405-900), 2026-11-15

Written by script, word for word, from the workflow's journal. Nothing below is the main session's. Its notes: night-u4f-scout.md.

- **HEAD:** 0ad41a62b019a3e6c3db6bf4736ca48346429128

## Sections

- **file:** N/u4-build-plan.md · **lines:** 805-816 · **what:** U4f's own section: Builds :806-809, Marks moved :810-815 (the advisor's condition 4 on -22's load :813-815), Reviewers :816
- **file:** N/u4-build-plan.md · **lines:** 882-883 · **what:** U4 order: U4r, then U4f, U4g, U4h one at a time (they share the room's and client's files)
- **file:** N/u4-build-plan.md · **lines:** 1054-1056 · **what:** Q21 U4-BRIEFS: U4f gets a short brief (field tables, budgets, run lists with counts) the advisor checks before build
- **file:** N/u4-build-plan.md · **lines:** 507-512 · **what:** §5 preamble: every sub-unit's end checks, counts as deltas, bite definition, pub items in the report
- **file:** N/u4-build-plan.md · **lines:** 178-203 · **what:** §4.1 public items: InputBuffer/buffer_depth_ticks :194, npt-server Counters 'U4f's -21 series' :195, npt-bots Behaviour incl. cheats and never-acker :197
- **file:** N/u4-build-plan.md · **lines:** 251-265 · **what:** §4.3 tick constants; CMD_CATCHUP = REDUNDANCY_CAP with const assert :261
- **file:** N/u4-build-plan.md · **lines:** 267-290 · **what:** §4.4 wire: Input layout, the header's two command acks, Wrap, A26 reading (trailing bytes and wrong channel counted in the pump)
- **file:** N/u4-build-plan.md · **lines:** 292-304 · **what:** §4.5 baselines, groups and the room-owned 256 kbps window (the task's 'window'; §4.7 is Events)
- **file:** N/u4-build-plan.md · **lines:** 330-336 · **what:** §4.7 Events: bounded queue, cap as data, resync then Kick(overflow); cap from the measured event rate (-22's site; carry 57; EVQ-LOAD)
- **file:** N/u4-build-plan.md · **lines:** 338-341 · **what:** §4.8 clocks on the client: command clock vs interpolation clock, integer ns API (ClockSync)
- **file:** N/u4-build-plan.md · **lines:** 343-358 · **what:** §4.9 lag compensation: tolerance includes the input-buffer wait; RTT from the pump
- **file:** N/u4-build-plan.md · **lines:** 360-390 · **what:** §4.10 hostile input: every check gets a -21 counter; clamp twin; press twin; fate- and ack-aligned twins; open-loop scripts; catch-up kind
- **file:** N/u4-build-plan.md · **lines:** 392-407 · **what:** §4.11 hash-map and clock bans, bites per instance, Send asserts, paths no L1 run reaches (T17)
- **file:** N/u4-build-plan.md · **lines:** 409-425 · **what:** §4.12 logs, contract test, precondition table: -20 :422, -21 :423, -22 exception :424
- **file:** N/u4-build-plan.md · **lines:** 427-451 · **what:** §4.13 scenes and the shot band (-21 forged-suite arithmetic :442-443; one silent client :444)
- **file:** N/u4-build-plan.md · **lines:** 453-476 · **what:** §4.14 bites with sites: -21/not-older-than :462, -21/no-catchup-cap :463, -48 reads -20's depth :467, -22/unbounded :475
- **file:** N/u4-build-plan.md · **lines:** 714-748 · **what:** §5 U4e: what U4f extends (InputBuffer option B :719, server checks :722-724, unbounded bite run judged by U4f :744)
- **file:** N/u4-build-plan.md · **lines:** 750-803 · **what:** §5 U4r: L1 runner, contract test r2, judged-run pattern U4f follows
- **file:** N/u4-build-plan.md · **lines:** 851-889 · **what:** Lanes: wave-2 file ownership :874-881; reviewers count toward the 3-agent limit at xhigh :886-887
- **file:** N/u4-build-plan.md · **lines:** 891-907 · **what:** §6 what U4 does not build (U5/U6/U7 deferrals)
- **file:** N/u4-build-plan.md · **lines:** 909-932 · **what:** §7 build rules: shared target §7.1, every brief names .claude/rules/netcode.md :921-922, §7.2 who builds
- **file:** N/u4-build-plan.md · **lines:** 958-968 · **what:** §9 change type, evidence rule, run counts (-20 6, -21 24 + 12 twins, -22 3, -47 3)
- **file:** N/u4-build-plan.md · **lines:** 970-983 · **what:** §10 preamble: readings for the readings PR, incl. W1-U4D's -21 press reading :975-976
- **file:** N/u4-build-plan.md · **lines:** 1042-1045 · **what:** Q16 U4-TWIN: -21's fate- and ack-aligned twins
- **file:** N/u4-build-plan.md · **lines:** 1057-1060 · **what:** Q22 U4-SCENES: -20/-21 gain SS, 48 clients and pre:PM-0001-2; -22 scene and clients only
- **file:** N/plan.md · **lines:** 456-481 · **what:** Test plan (rev 3.3) components: E3 commands :461, E10 clock :468, E12 cheats incl. never-acker :470
- **file:** N/plan.md · **lines:** 483-504 · **what:** E7 ClientEndpoint API in integer ns; wrong-channel message counted as forged (-21) :489
- **file:** N/plan.md · **lines:** 506-525 · **what:** §4.4 where each check lives: room column incl. the lead bound :514; per-tick inputs; L1 hostile rows on simulated time :524-525
- **file:** N/plan.md · **lines:** 553-566 · **what:** §4.6 seam and tick constant; CMD_CATCHUP separate from TICK_HZ :561-562
- **file:** N/plan.md · **lines:** 596-598 · **what:** §4.7 npt-room symbols: InputBuffer (option A or B)
- **file:** N/plan.md · **lines:** 609 · **what:** §4.7 npt-client symbols: ClockSync among them
- **file:** N/plan.md · **lines:** 617-628 · **what:** npt-lab SimClock instant order; each client's host clock offset + rate, rate 1.5 for -21's cheat :625
- **file:** N/plan.md · **lines:** 677-704 · **what:** §6.1 verdict rules, precedence, bites first, no retries
- **file:** N/plan.md · **lines:** 713-722 · **what:** §6.2 rows: -20 C (L1) under both options :721; -21, -22, -47 C (L1) :722
- **file:** N/plan.md · **lines:** 765-775 · **what:** §6.4 netcode evidence rule on L1 (3 seeds per profile)
- **file:** docs/adr/0001-multiplayer-netcode.md · **lines:** 1610 · **what:** PM-0001-20 row (commands on time, presses counted once, buffer depth recorded; bite redundancy 1)
- **file:** docs/adr/0001-multiplayer-netcode.md · **lines:** 1611 · **what:** PM-0001-21 row (forged set, clamp, 1.5x clock, lag switch; its bites)
- **file:** docs/adr/0001-multiplayer-netcode.md · **lines:** 1612 · **what:** PM-0001-22 row (never-acker among 48 for 10 min; bite unbounded queue)
- **file:** docs/adr/0001-multiplayer-netcode.md · **lines:** 1644 · **what:** PM-0001-47 row (clock offset error at most 5 ms after 2 s at P2; bites newest-raw-sample, tick-quantised receive)
- **file:** docs/adr/0001-multiplayer-netcode.md · **lines:** 226-255 · **what:** Part 5: command checks incl. lead bound :231-233 and catch-up cap :238-239; Time paragraph (clock filter, arrival stamps, pong hold time) :239-251
- **file:** docs/adr/0001-multiplayer-netcode.md · **lines:** 1156-1167 · **what:** OQ-0001-6: option (a) run on arrival vs (b) buffer; decided by the proof test
- **file:** N/u4-scout-netcode.md · **lines:** 122 · **what:** net F104: OQ-0001-6 options; the client's tick-lead controller is the same in both; -30 replay setting
- **file:** N/u4-scout-netcode.md · **lines:** 135 · **what:** net F117: two client clocks; -47's newest-raw-sample bite reasoning
- **file:** N/u4-scout-netcode.md · **lines:** 77 · **what:** net F59: PM-0001-21's parts, counts and bites summarised
- **file:** N/u4-scout-netcode.md · **lines:** 97 · **what:** net F79: catalogue's room vs server Input checks split
- **file:** N/u4-scout-netcode.md · **lines:** 126 · **what:** net F108: -21 clamp twin stability
- **file:** N/u4-scout-netcode.md · **lines:** 137 · **what:** net F119: EventQueue rule for -22
- **file:** N/u4-scout-netcode.md · **lines:** 164 · **what:** Scout's original U4f proposal (ClockSync, lead controller, A and B, hostile suites; marks -47, -20, -21, -22)
- **file:** docs/design/netcode-proof-messages.md · **lines:** 52-60 · **what:** Catalogue: where each check runs (plan §4.4)
- **file:** docs/design/netcode-proof-messages.md · **lines:** 87 · **what:** Input row (checks, bucket refilled at 2 x TICK_HZ)
- **file:** docs/design/netcode-proof-messages.md · **lines:** 91-92 · **what:** Ping and Pong rows (ClockSync's samples, hold time)
- **file:** docs/design/netcode-proof-messages.md · **lines:** 99 · **what:** Kick row (overflow; never for a forged kind)
- **file:** docs/design/netcode-proof-messages.md · **lines:** 122-123 · **what:** Authority note: no client-reported RTT; pong hold time bounded
- **file:** .claude/rules/netcode.md · **lines:** 1-17 · **what:** Rules file every brief names as binding on proto, room, client and server code
- **file:** spikes/netcode-proof/marks/marks.json · **lines:** 4618-4778 · **what:** PM-0001-20: parts :4632-4670, checks :4683-4765, bite redundancy-1 :4768, notes :4777
- **file:** spikes/netcode-proof/marks/marks.json · **lines:** 4779-5119 · **what:** PM-0001-21: 8 parts :4792-4891, checks :4907-5060, 13 bites :5061-5115, notes :5118
- **file:** spikes/netcode-proof/marks/marks.json · **lines:** 5120-5202 · **what:** PM-0001-22: part never-ack :5133, checks :5144-5191, bite unbounded :5192-5197, notes :5201
- **file:** spikes/netcode-proof/marks/marks.json · **lines:** 9046-9109 · **what:** PM-0001-47: part L1 P2 1,000 connects :9059, checks :9069-9094, bites :9097 and :9101, notes :9108
- **file:** spikes/netcode-proof/marks/marks.json · **lines:** 151-625 · **what:** PM-0001-2, the precondition -20 and -21 carry
- **file:** spikes/netcode-proof/marks/README.md · **lines:** 280-290 · **what:** §4 pre: rule and -22's written exception :285-290
- **file:** spikes/netcode-proof/marks/README.md · **lines:** 247-279 · **what:** §4 validity rules (build/C4, roster, late-zero :275, reorder :276-277)
- **file:** spikes/netcode-proof/marks/README.md · **lines:** 317-330 · **what:** §5 bites first
- **file:** spikes/netcode-proof/marks/README.md · **lines:** 187-192 · **what:** §2.5 header values; params.cmd_timing A or B :191-192
- **file:** spikes/netcode-proof/marks/README.md · **lines:** 22-23 · **what:** role twin and params.twin
- **file:** spikes/netcode-proof/marks/SERIES.md · **lines:** 21, 93, 128, 442 · **what:** -22 rows: ack_starved_headroom_count, rss_bytes, ack_starved.dropped, event_queue_cap_count
- **file:** spikes/netcode-proof/marks/SERIES.md · **lines:** 33-34, 144-145, 155-159, 170, 322 · **what:** -21 rows: lagswitch, clamp_kind, fast_clock, forged_kind, lag_burst, clamp_report
- **file:** spikes/netcode-proof/marks/SERIES.md · **lines:** 40-42, 61, 63, 101, 245 · **what:** -20 rows: press counts, cmd_on_time, input_missing, buffer_depth_count, reorder_permille
- **file:** spikes/netcode-proof/marks/SERIES.md · **lines:** 149 · **what:** -47 row: event.connect_clock.offset_err_ns
- **file:** spikes/netcode-proof/marks/SERIES.md · **lines:** 240, 295, 412-413, 426 · **what:** buffer_depth_ticks_count (-30), twin_hash_mismatch_count, cmd_timing and suite params, twin param
- **file:** N/night-2026-11-10.md · **lines:** 15-39 · **what:** Night plan rules 1-6 (the form tonight's plan takes)
- **file:** N/night-2026-11-10.md · **lines:** 41-69 · **what:** Item 2 (U4e brief draft): Read list :45-50, brief shape :51-60, recheck and questions :61-64, agents :65-68, done :69
- **file:** N/night-2026-11-10.md · **lines:** 91-110 · **what:** Item 4 (U4r brief draft): read, shape, O1, agents, done
- **file:** N/night-2026-11-12.md · **lines:** 15-36 · **what:** Later night plan rules (no build, no script from the spike branch)
- **file:** N/u4r-report.md · **lines:** 171-285 · **what:** U4r's follow-ups with homes (JDG-ONLY-1 :257-264, check.mjs follow-up :265-274, round-1 others :281-284)
- **file:** N/u4r-report.md · **lines:** 1048-1052 · **what:** EVQ-LOAD carry line for U4f's and U4g's briefs, word for word
- **file:** N/u4r-report.md · **lines:** 1135-1167 · **what:** BRAWL-1 follow-ups with homes (npt-bots lib.rs, logs.rs room_tick, crates/sim)
- **file:** N/u4r-report.md · **lines:** 1169-1180 · **what:** KEPT-LOGS under Decisions needed (disk numbers, options)

## Carries

- **entry:** header · **lines:** N/u4s-carries.md:9-14 · **what:** Briefs carry each entry's text word for word; reports list carries met by number
- **entry:** 46 · **lines:** N/u4s-carries.md:506-509 · **what:** -21's press twins: twin removes exactly the refused presses (slot, seq, kind), counter one lower; room-level hash test
- **entry:** 80 (a) · **lines:** N/u4s-carries.md:510-518 · **what:** Room canonical sort is wrap-aware, not total; before option A runs several commands per slot, lead bound refuses first or total key, with tests
- **entry:** 80 (b) · **lines:** N/u4s-carries.md:518-520 · **what:** A refused far-ahead snapshot header must not move the client's newest tick past a bound (receive, see), with a test
- **entry:** 80 (c) · **lines:** N/u4s-carries.md:520-522 · **what:** ClientEndpoint::step drops one failing command, counts it, keeps the acks, with a test
- **entry:** 80 (d) · **lines:** N/u4s-carries.md:522-529 · **what:** U4f adds one endpoint test for the client's two too-old cases, each half failing under its plant
- **entry:** 80 (e) · **lines:** N/u4s-carries.md:529-534 · **what:** full_state compares the frame's hash with one already filed and files the sentinel on difference, with a test
- **entry:** 86 · **lines:** N/u4s-carries.md:535-537 · **what:** U4R-Q4: U4f extends r2 to -21's option-A suites and -20 P3 option A with PM-0001-2 passing and the shot band
- **entry:** 57 · **lines:** N/u4s-carries.md:223-228 · **what:** Event cap provisional (1,024); U4f measures the rate and records §4.7's value before -22's runs
- **entry:** 74 · **lines:** N/u4s-carries.md:343-354 · **what:** Endpoint lead default 1; U4f's lead controller comes after U4r; U4r's with_lead formula and late-command report
- **entry:** 21 · **lines:** N/u4s-carries.md:148-149 · **what:** Room sorts accepted commands into canonical order before step (the sort carry 80 (a) touches)
- **entry:** 27 · **lines:** N/u4s-carries.md:166-168 · **what:** CMD_CATCHUP = REDUNDANCY_CAP with const assert; tick grep exemption
- **entry:** 18 · **lines:** N/u4s-carries.md:129-132 · **what:** Route for a new tick-count constant in crates/sim (TICK_DERIVED, s1 pin, const assert)
- **entry:** 39 · **lines:** N/u4s-carries.md:169-179 · **what:** A tick count needed outside npt-sim goes into npt-sim by carry 18's route; rate-dependent values take the rate as an argument
- **entry:** 50 · **lines:** N/u4s-carries.md:180-183 · **what:** EventQueue never queues an event older than 255 ticks
- **entry:** 61 · **lines:** N/u4s-carries.md:229-247 · **what:** hs filing rules for room and client ((b) is the case carry 80 (e) completes)
- **entry:** 58 · **lines:** N/u4s-carries.md:269-273 · **what:** Overflow resyncs in judged -17/-18 runs reported to the advisor (cap sizing bears on it)
- **entry:** 62 (h) · **lines:** N/u4s-carries.md:289-321 · **what:** -17's L1 P0 no-lost-group check; its stay is Mustafa's C62H-KEEP
- **entry:** 76 · **lines:** N/u4s-carries.md:364-371 · **what:** Session-ending glue: kicked or replaced sessions reach no room call (never-acker's Kick path)
- **entry:** 77 · **lines:** N/u4s-carries.md:372-381 · **what:** U4r's readings of room and client (per-counter wraps, stale() sums to the advisor)
- **entry:** 63 · **lines:** N/u4s-carries.md:322-327 · **what:** Next change to npt-log adds two refusals (applies if U4f opens crates/log)
- **entry:** 64 · **lines:** N/u4s-carries.md:447-473 · **what:** Next change to crates/judge adds round 2's judge follow-ups first (applies if U4f opens the judge)
- **entry:** 67 · **lines:** N/u4s-carries.md:476-493 · **what:** RM re-measure for any change that moves ES's synthetic recording or the World's outcome on it

## Rulings

- **file:** N/advisor-u4.md · **lines:** 78-89 · **id:** U4 condition 1 · **what:** Refused input leaves no trace in the hash; twin = forged command with only the refused part removed (amended :477-479)
- **file:** N/advisor-u4.md · **lines:** 112-118 · **id:** U4 condition 4 · **what:** -22's precondition exception; its runs log PM-0001-2's series and U4f's report states each run's load
- **file:** N/advisor-u4.md · **lines:** 119-124 · **id:** U4 condition 5 · **what:** Precondition must pass on short runs of -20 P3 and each of -21's eight suites
- **file:** N/advisor-u4.md · **lines:** 143-157 · **id:** U4 conditions 10-12 · **what:** Lanes and reviewers at xhigh; readings PR list; the five briefs come to the advisor before builds
- **file:** N/advisor-u4.md · **lines:** 186-187 · **id:** U4 verdict, order · **what:** U4f, U4g, U4h run one at a time
- **file:** N/advisor-u4.md · **lines:** 436-465 · **id:** F2 (U4-TICKROUTE) · **what:** Tick-count route; point (6) :463-465 lets U4f's brief give it carry 18's places (first case U4f's lead bound)
- **file:** N/advisor-u4-wave1.md · **lines:** 473-479 · **id:** U4E-Q6 · **what:** Event cap as data, provisional; U4f measures the rate and records §4.7's value before -22's runs
- **file:** N/advisor-u4-wave1.md · **lines:** 491-495 · **id:** U4E-Q11 · **what:** U4e builds server-side checks; U4f adds their -21 counters and judged runs on U4e's pump
- **file:** N/advisor-u4-wave1.md · **lines:** 197-240 · **id:** W1-U4D N-M2 · **what:** Press counters ruling; new U4f carry (press twins) :232-235; -21 reading for the readings PR :236-240
- **file:** N/advisor-u4e-brief.md · **lines:** 217-244 · **id:** U4E-Q12 · **what:** -22/unbounded planted and unit-shown by U4e; U4f runs it judged; headroom written per tick
- **file:** N/advisor-u4e-brief.md · **lines:** 340-341 · **id:** U4E-Q12 (later briefs) · **what:** U4f's -22 runs write ack_starved_headroom_count for the never-acker and select -22/unbounded
- **file:** N/advisor-u4e-r1.md · **lines:** 202-205 · **id:** U4e R3 (NC-M6) · **what:** Default lead 1 is late above half a tick; U4r sets lead per profile until U4f's controller
- **file:** N/advisor-u4e-r1.md · **lines:** 210-217 · **id:** U4e R3 (RR-5) · **what:** Room tick bench; any unit editing the encode path before U6 (U4f, U4g, U4h) may take the cheap wins with a bench
- **file:** N/advisor-u4e-r1.md · **lines:** 279-281, 343, 560-567 · **id:** U4e R5 (SEC-U4E-9, RR-7, TE-9) · **what:** Routes to carry 80 (a)-(c) at U4f and the carry's inserted text
- **file:** N/advisor-u4e-r2.md · **lines:** 117-125, 229-247, 290, 346 · **id:** U4e round-2 Q2 (NC2-L1, TE2-L1, RR2-2) · **what:** Carry 80 (d) and (e) routed to U4f, with the carry block's words
- **file:** N/advisor-u4r-brief.md · **lines:** 198-205 · **id:** U4R-Q4 · **what:** r2 scope at U4r narrowed to option B; option-A half moves to U4f with the same bar (carry 86)
- **file:** N/advisor-u4r-brief.md · **lines:** 289-359 · **id:** U4R-Q14 (with A3 :133-141, A4 :143-147) · **what:** Lead arithmetic L = ceil((rtt + jitter)/P) + 1 and the per-command press rule the bots follow
- **file:** N/advisor-u4r-build2-full.md · **lines:** 28 · **id:** U4r build ruling 2, point 31 · **what:** Make cargo test -p npt-server --all-features compile in the next lane opening npt-server (U4h, or U4f if sooner)
- **file:** N/advisor-u4r-r1-full.md · **lines:** 61, 80 · **id:** NC-U4R-5 / EA-U4R-7 · **what:** Logs keyed by slot mis-mark a replaced session; follow-up for the first unit logging a replacement (U4f's hostile clients or U6)
- **file:** N/advisor-u4r-r1-full.md · **lines:** 119-123 · **id:** T-6 · **what:** Contract half: next unit editing contract.rs adds five doctored negative cases
- **file:** N/advisor-u4r-c62h-full.md · **lines:** 8-20, 44-45 · **id:** JDG-ONLY-1 · **what:** --only drops unnamed marks so pre:PM-0001-2 fails; reach names -20 and -21 :19; home npt-judge's next change :20; bite runs list PM-0001-2 :44-45
- **file:** N/advisor-brawl1-sets-p3-full.md · **lines:** 156-158 · **id:** EVQ-LOAD · **what:** Carry line for U4f's and U4g's briefs: full-view event rate 460-560/s; U4f's cap measurement uses it
- **file:** N/advisor-brawl1-sets-p0-full.md · **lines:** 194 · **id:** BRAWL-1 P0 ruling · **what:** EVQ-LOAD's homes: U4f and U4g
- **file:** N/advisor-c62h-review-full.md · **lines:** 82-93 · **id:** C62H-REVIEW units ahead · **what:** Every marks.json or judge change owes a read; item 4 :89: U4f's brief must say whether marks.json changes
- **file:** N/advisor-c62h-review-full.md · **lines:** 59, 63-64, 68-81 · **id:** C62H-KEEP / C62H-BOUND · **what:** Mustafa's two open decisions on -17's lost-tick checks; ADV-C62H-2 design conditions for (a)

## The card's lines

- **lines:** .checkpoints/netcode-proof-test.md:17-43 · **what:** LATEST: tonight's night item is U4f's brief draft; inputs named :27-32; not-now items :33-34; scout and next steps :36-43
- **lines:** .checkpoints/netcode-proof-test.md:24-32 · **what:** Night plan file named night-2026-11-16.md in night-2026-11-10.md's form; rule words copied by script
- **lines:** .checkpoints/netcode-proof-test.md:419-461 · **what:** KEPT-LOGS answered (c); kit lines posted; waiting for KL-YES; nothing packed or deleted before the kit PR
- **lines:** .checkpoints/netcode-proof-test.md:462-506 · **what:** C62H-REVIEW answered; units owing a read incl. U4f if marks change :488-491; new standing read rule :499-502
- **lines:** .checkpoints/netcode-proof-test.md:479-492 · **what:** C62H-BOUND and C62H-KEEP with Mustafa, recommended (a) and (a), waiting
- **lines:** .checkpoints/netcode-proof-test.md:507-523 · **what:** NEXT (task #74): U4f after step (7), brief to the advisor first; KEPT-LOGS before U4g's build
- **lines:** .checkpoints/netcode-proof-test.md:1930-1933 · **what:** DEPTH ruling: buffer_depth_ticks_count L kept; room measures L - 1 after each client's first take
- **lines:** .checkpoints/netcode-proof-test.md:1960-1964 · **what:** Follow-ups: npt-server --all-features (U4f if sooner); option-A note: buffer_depth_ticks_count is the depth set, measured L - 1 (moved from :1936-1937)
- **lines:** .checkpoints/netcode-proof-test.md:1982-1987 · **what:** Copy 2's DEPTH finding and the npt-server tests/loopback.rs --all-features failure
- **lines:** .checkpoints/netcode-proof-test.md:1467-1474 · **what:** JDG-ONLY-1 (Medium) and the interim rule for npt-judge --only with pre:PM-0001-2
- **lines:** .checkpoints/netcode-proof-test.md:1527-1529 · **what:** The check.mjs follow-up list and the next npt-lab lane items
- **lines:** .checkpoints/netcode-proof-test.md:163-171 · **what:** TE-B1-R2-1 and RR2-B1-1 homed at the next lane editing npt-bots' lib.rs
- **lines:** .checkpoints/netcode-proof-test.md:340, 388-405 · **what:** EVQ-LOAD recorded on the card and accepted (ADV-CL-1)
- **lines:** .checkpoints/netcode-proof-test.md:1234-1256 · **what:** Disk plan numbers, the part-clean, D4 ticks per second and set times
- **lines:** .checkpoints/netcode-proof-test.md:265-266 · **what:** M7-4: main folder's cold full set refilled about 5.0 GiB
- **lines:** .checkpoints/netcode-proof-test.md:873-874 · **what:** BRAWL-1 runner start figures per set (P3 524.4, P2 780.8, P0 3,614.6 MiB plus floor)
- **lines:** .checkpoints/netcode-proof-test.md:3013-3072 · **what:** Rules in force: read rule :3015-3021, KEPT-LOGS :3022-3027, SIZE-CHECK :3028-3035, USAGE-SAVINGS :3050-3059, U4 briefs to advisor :3070-3072
- **lines:** .checkpoints/netcode-proof-test.md:3073-3088 · **what:** U4j r2 TE2-5 and B6 rules; U4d-F ruling's Write/Edit sentence and TE2-5 stop clause every brief carries
- **lines:** .checkpoints/netcode-proof-test.md:3362-3367 · **what:** The night shift's role (Mustafa 2026-11-09, verbatim record): next unit's plan drafts each evening

## Mustafa's standing rules (pointers only)

- **file:** N/history.md · **lines:** 2113-2129 · **name:** C62H-REVIEW (a) and the read rule (2026-11-15 13:35) · **bearing:** Rule line :2127: any U4f change to marks.json or the judge needs a reviewer's read before U4f closes, named in its own report line
- **file:** N/history.md · **lines:** 2088-2111 · **name:** KEPT-LOGS (c) (2026-11-15 08:20) · **bearing:** No raw-log folder packed or deleted until the kit PR is on main and taken in; U4f's disk must come from approved cleans
- **file:** N/history.md · **lines:** 2039-2087 · **name:** SIZE-CHECK, COMPACT-WAIT, MAIN-52 (2026-11-14) · **bearing:** U4f (size M) counted at every snapshot; past more than twice the plan (M about 3,000) stops for three lines; /compact never waited on; main taken in only on code/CI/rule change
- **file:** N/history.md · **lines:** 398-399 · **name:** SIZE-CHECK (2026-11-10) · **bearing:** The original three-line stop when a unit grows well past its plan
- **file:** N/history.md · **lines:** 291-299 · **name:** USAGE-SAVINGS (a) and its limits (2026-11-11) · **bearing:** Builders' pass/fail from exit code and runner summary; main session reads every Medium+ and all judge/marks/pass-bar work; packets cut by script; chat size in reports
- **file:** N/history.md · **lines:** 335-352 · **name:** NIGHT-BUDGET (a), the 9.5 floor of PM-0001-2, Low never pushes (2026-11-11) · **bearing:** PM-0001-2's ES event floor does not move under U4f's -20/-21 precondition runs; a Low triggers no push
- **file:** N/history.md · **lines:** 353-356 · **name:** His words move by script and line ranges (2026-11-11) · **bearing:** The brief quotes his words only by script from line ranges, never retyped
- **file:** N/history.md · **lines:** 411-420 · **name:** Shared Cargo target (2026-11-09) · **bearing:** Every U4f copy builds in .gate/shared-target with touch and count safeguards (plan §7.1)
- **file:** N/history.md · **lines:** 404-410 · **name:** LEFTOVER-DELETE (2026-11-09) · **bearing:** No one-off deletes; critics' scratch only in forms the approved methods remove
- **file:** N/history.md · **lines:** 859-861 · **name:** Cache and copy cleanup after each unit merges (2026-11-08) · **bearing:** After U4f merges, its cache and finished copies are removed by approved methods, asked once
- **file:** N/history.md · **lines:** 1927-1975 · **name:** SPEND-LIMIT (a) and review_packet ALLOWED (2026-11-12) · **bearing:** Restore and resume after cuts with the notice on the card; U4f's review packets admit only listed advisor rulings
- **file:** N/history.md · **lines:** 1990-2012 · **name:** CHECK-IN (2026-11-13) · **bearing:** One check-in about every two hours while U4f's agents run
- **file:** N/history.md · **lines:** 379-403 · **name:** RESUME-CARD, WIP-CLEAN, TCP-FREEZE, SQUASH-40, PROMOTE (2026-11-10) · **bearing:** WIP snapshot per live builder deleted on merge; PROMOTE only at U4's close
- **file:** .checkpoints/netcode-proof-test.md · **lines:** 3362-3367 · **name:** The night shift's role (2026-11-09, adjusted version) · **bearing:** The night drafts U4f's brief; building and testing stay with the day session

## Open items

- **id:** C62H-BOUND · **refs:** card :479-492; N/advisor-c62h-review-full.md:59, :63, :75-81, :132 · **bearing:** Carry 80 (b) and (d) change how the client files lost and too-old ticks, which a lost-tick bound on P2/P3/B1 would count; open with Mustafa
- **id:** C62H-KEEP · **refs:** card :483-484, :492; N/advisor-c62h-review-full.md:64, :68-74, :128; N/u4s-carries.md:314-321 · **bearing:** Whether -17's L1 P0 no-lost-group check stays; it reads the same client filing paths carry 80 touches
- **id:** KEPT-LOGS (KL-YES) · **refs:** card :419-461, :508-509, :521; N/u4r-report.md:1174-1180; N/history.md:2088-2111 · **bearing:** Disk for U4f's builds and judged runs; nothing packed or deleted before the kit PR merges; due before U4g's build
- **id:** JDG-ONLY-1 and its interim rule · **refs:** card :1467-1474, :33-34, :515-516; N/u4r-report.md:257-264; N/advisor-u4r-c62h-full.md:8-20, :44-45 · **bearing:** -20 and -21 carry pre:PM-0001-2, so every U4f npt-judge --only naming them also names PM-0001-2; fix rides npt-judge's next change
- **id:** check.mjs follow-up · **refs:** N/u4r-report.md:265-274; card :1527-1528, :515-516 · **bearing:** Rides check.mjs's next change; applies if U4f's lane takes check.mjs via carry 18's places
- **id:** npt-server --all-features compile · **refs:** card :1960-1961, :1985-1987; N/advisor-u4r-build2-full.md:28 · **bearing:** U4f opens npt-server (U4E-Q11 counters); if before U4h it makes the test build compile, its run as evidence
- **id:** Option-A depth note · **refs:** card :1960-1964, :1930-1933 · **bearing:** buffer_depth_ticks_count is option B's set depth with measured L - 1; U4f's option A needs its own stated reading
- **id:** NC-U4R-5 / EA-U4R-7 logs half · **refs:** N/advisor-u4r-r1-full.md:61, :80; N/u4r-review-r2-full.md:15; N/u4r-report.md:281-282 · **bearing:** First unit whose runs log a replacement (U4f's hostile clients or U6) keys ended/gone by session
- **id:** T-6 contract half · **refs:** N/advisor-u4r-r1-full.md:119-123; N/u4r-report.md:283 · **bearing:** Carry 86 extends r2, so the next contract.rs edit adds the five doctored negative cases
- **id:** TE-B1-R2-1 with RR2-B1-1 · **refs:** N/u4r-report.md:1137-1141; card :163-171; spikes/netcode-proof/crates/bots/src/lib.rs:130-132, :404 · **bearing:** U4f's cheats edit npt-bots lib.rs, the follow-up's home lane
- **id:** RR-B1-7 (1) · **refs:** N/u4r-report.md:1162; spikes/netcode-proof/crates/lab/src/logs.rs:495 · **bearing:** Next lane editing logs.rs's room_tick rewrites its loop; U4f's -20/-21/-22 series may land there
- **id:** EA-U4R-4, EA-B1-1 later half, NC-U4R-7 · **refs:** N/u4r-report.md:282-283, :1163-1164; N/u4r-build.md:1941-1943 · **bearing:** Ride the next lane opening crates/sim, if U4f's brief gives it carry 18's places
- **id:** EVQ-LOAD · **refs:** N/advisor-brawl1-sets-p3-full.md:156-158; N/u4r-report.md:1048-1052, :1155-1158; N/u4s-carries.md:223-228 · **bearing:** U4f's §4.7 cap measurement uses the full-view event rate before -22's runs
- **id:** RR-5 room bench · **refs:** N/advisor-u4e-r1.md:210-217 · **bearing:** Optional cheap wins with a bench before and after if U4f edits the encode path
- **id:** Readings PR before R5 · **refs:** N/advisor-u4.md:150-153; N/advisor-u4-wave1.md:236-240; N/u4-build-plan.md:970-983 · **bearing:** -22's exception (Q22) and -21's press and clamp twin readings go to Mustafa through the readings PR

## The latest brief's headings

- **lines:** N/u4r-brief.md:1-32 · **heading:** Title: U4r builder brief (rev 3.2) with revision history paragraph
- **lines:** N/u4r-brief.md:33-77 · **heading:** Changes in rev 3.1
- **lines:** N/u4r-brief.md:78-181 · **heading:** Changes in rev 3
- **lines:** N/u4r-brief.md:182-246 · **heading:** 0. How to read the citations
- **lines:** N/u4r-brief.md:247-324 · **heading:** 1. Start
- **lines:** N/u4r-brief.md:325-689 · **heading:** 2. What to build (2.1 npt-lab :339, 2.2 npt-bots :492, 2.3 run scripts :532, 2.4 tests :551, 2.5 bites :633, 2.6 seam rule :648, 2.7 public items :679)
- **lines:** N/u4r-brief.md:690-935 · **heading:** 3. The carries (3.1 word for word :692, 3.2 where met :884, 3.3 routed elsewhere :910)
- **lines:** N/u4r-brief.md:936-1069 · **heading:** 4. Build rules (lane statement :994-1011)
- **lines:** N/u4r-brief.md:1070-1097 · **heading:** 5. End checks
- **lines:** N/u4r-brief.md:1098-1139 · **heading:** 6. Return list
- **lines:** N/u4r-brief.md:1140-1373 · **heading:** 7. The judged runs and the -30 pre-check (7.1-7.4)
- **lines:** N/u4r-brief.md:1374-1472 · **heading:** 8. O1 (carry 40)
- **lines:** N/u4r-brief.md:1473-1533 · **heading:** 9. Recheck after wave 1 and wave 2
- **lines:** N/u4r-brief.md:1534-1770 · **heading:** 10. Questions for the advisor
- **lines:** N/u4r-brief.md:1771-1844 · **heading:** 11. Needs a run (day)
- **lines:** N/u4r-brief.md:1845-1909 · **heading:** 12. Left for the main session at launch

## The advisor's conditions on the latest brief

- **id:** L1 · **lines:** N/advisor-u4r-brief.md:416-420 · **required:** Fold the ruling into the brief by script (rev 3.2), each old phrase asserted found once; any other change back to the advisor
- **id:** L2 · **lines:** N/advisor-u4r-brief.md:421-424 · **required:** Carries and plan text in the main session's own commit before launch, then section 0's stamps re-set and moved cites re-pointed
- **id:** L3 · **lines:** N/advisor-u4r-brief.md:425-427 · **required:** The manifest commit on committed state with its counted run's results on the card; base is that commit or later
- **id:** L4 · **lines:** N/advisor-u4r-brief.md:428-429 · **required:** Placeholders and values filled (section 12 items 1-3, P16), each anchor once, closed files' sha256 asserted first
- **id:** L5 · **lines:** N/advisor-u4r-brief.md:430-431 · **required:** Card keeps TE2-5's rule until U4r's merge and names the ruling and rev 3.2 in the brief's entry
- **id:** L6 · **lines:** N/advisor-u4r-brief.md:432-433 · **required:** Launch one builder (general-purpose, opus, xhigh) in its own worktree under the lock and runner, at most 3 agents counting reviewers

## Disk

- **Free (MiB):** 7689
- **Estimates:** df -BM . at the repo root: 7,689 MiB available (81 % used). .gate: netcode-proof-test 12,793 MiB (kept raw logs and main cache), u3c-review-target 4,051, u4r 970, brawl1-evidence 608, u4r-d4 320, shared-target 263. Existing figures: 56 MB per 10-minute run whose header lacks -57 (about 3,050 B a measured tick), -57 runs 183-604 MB by profile; 4 GiB floor before every build and run; D4 slowest ticks per second SS P0 69.6, P2 51.6, P3 54.7, B1 58.4, ES P3 148.0; a judged run is 18,335 ticks (355.4 s longest); RUN_LIMIT_MS 3,600 s; judge peak memory 302.2 + 2.3403 x MiB of logs, line 10,240 MiB; BRAWL-1 set walls P3 1,020.5 s, P2 1,046.7 s, P0 2,566.1 s; cold main-folder refill about 5.0 GiB (M7-4), U4g's cache refill quoted as 8-9 GiB. No figure exists for -47's 1,000 connects, -21's 8 suites with twins and 13 bites, option-A runs, or a never-acker run.
- **Refs:** spikes/netcode-proof/ops/runs/c-u4r-runs.mjs:131-132, :137-153, :154-159, :167-178, :206-215, :324; N/u4r-disk-plan.md:16-46, :79-92; N/u4r-report.md:369-408, :409-423, :466, :476, :503, :538, :567, :592, :993, :1004, :1012, :1175; card :265-266, :873-874, :1234-1256; N/advisor-u4r-r2-full.md:21; spikes/netcode-proof/ops/runs/c-u4r-r2.mjs:40-43, :66

## The code lane

- **path:** spikes/netcode-proof/crates/room/src/input.rs · **rule:** Option A beside U4e's option B and the room-side checks with -21 counters; its doc :1-5 names them U4f's · **refs:** input.rs:1-5, :17-156 (CMD_CATCHUP :61, buffer_depth_ticks :64-68, offer :80-128, take :129-156); N/u4-build-plan.md:806-809
- **path:** spikes/netcode-proof/crates/room/src/room.rs · **rule:** Room's checks and counters; canonical sort (carry 80 (a)); option B rerun; -22 headroom · **refs:** canonical :361-369; ClientCounters :65-160; queue/headroom/buffer_depth_ticks :278-294; event_cap PROVISIONAL :460; receive :713-743; receive_input :744-804; fill_input :831-879; carry 21 test :1402
- **path:** spikes/netcode-proof/crates/room/src/events.rs · **rule:** Event cap value from U4f's rate measurement (carry 57; EVQ-LOAD); -22/unbounded site is U4e's · **refs:** EventQueue :26-45; PROVISIONAL_CAP :40
- **path:** spikes/netcode-proof/crates/room/tests/room.rs · **rule:** Carry 80 (a): carry 21's room test made to fail without the sort · **refs:** N/u4s-carries.md:513-518
- **path:** spikes/netcode-proof/crates/client/src/lib.rs · **rule:** ClockSync and the tick-lead controller (doc :1-11 names the clock U4f's); carry 80 (b)-(e) sites · **refs:** lead :214, :267; with_lead :296-301; refused :388; stale :394; input :414-449 (stamp :429); step :450-491; receive :492-527; too_old/old_header/bad_header :528-567; see :583-618; file :645-659; full_state :810-839
- **path:** spikes/netcode-proof/crates/client/tests/endpoint.rs · **rule:** Home of carry 80 (d)'s endpoint test and (b), (c), (e) tests · **refs:** N/u4s-carries.md:518-534
- **path:** spikes/netcode-proof/crates/server/src/pump.rs · **rule:** U4E-Q11: the server-side checks' -21 counters; ping answers for ClockSync · **refs:** Counters :112-182; Decoder :204-236; pump :372-482; pong :500-517; lib.rs exports :11-17; N/advisor-u4-wave1.md:491-495
- **path:** spikes/netcode-proof/crates/server/tests/loopback.rs · **rule:** npt-server --all-features compile follow-up if U4f opens npt-server before U4h · **refs:** card :1960-1961, :1985-1987
- **path:** spikes/netcode-proof/crates/bots/src/lib.rs · **rule:** The cheats' Behaviours and the never-acker (doc :10-11 names them U4f's); TE-B1-R2-1/RR2-B1-1 ride · **refs:** doc :1-24; Behaviour :94-327 (brawl :174, script :180, step :213); toward_centre doc :130-132; dead-input test :404
- **path:** spikes/netcode-proof/crates/lab/src/scenario.rs · **rule:** Suites, option A setting (cmd_timing is fixed to B today), 1.5x host clock, -47's connects, lead wiring · **refs:** doc :1-27 (1.5 rate U4f's :18-19); lead_for :126-131; with_lead :848; behaviour dispatch :851-860; params :1282-1290; suite default :1405
- **path:** spikes/netcode-proof/crates/lab/src/contract.rs · **rule:** r2 extended to option-A suites (carry 86); T-6's negative cases ride · **refs:** contract.rs:1-25; N/advisor-u4r-r1-full.md:119-123
- **path:** spikes/netcode-proof/crates/lab/src/logs.rs · **rule:** Series writers; RR-B1-7 (1) rides an edit of room_tick · **refs:** room_tick :495; N/u4r-report.md:1162
- **path:** spikes/netcode-proof/ops/runs/c-u4r-r2.mjs · **rule:** r2 suites; -21 option-B stand-ins to extend with option A (carry 86) · **refs:** header :1-43; SUITES :72-92 (-21 stand-ins :82-85)
- **path:** spikes/netcode-proof/ops/runs/c-u4r-runs.mjs · **rule:** Judged-set runner model and size constants for U4f's judged runs · **refs:** :131-215, planJudged :324
- **path:** spikes/netcode-proof/marks/SERIES.md and marks.json · **rule:** New SERIES rows only by ruling (U4r precedent); marks.json changes owe a reviewer's read · **refs:** N/advisor-u4r-brief.md:189-196; N/advisor-c62h-review-full.md:82-93
- **path:** spikes/netcode-proof/crates/sim/src (ticks.rs) and check.mjs · **rule:** Only through carry 18's places, which U4f's brief may give (no lane holds them then) · **refs:** N/advisor-u4.md:463-465; N/u4s-carries.md:129-132
- **path:** (lane model) · **rule:** Plan gives no explicit U4f file list; room and client shared with U4g/U4h one at a time; U4e's and U4r's lane statements are the models · **refs:** N/u4-build-plan.md:874-883; N/u4r-brief.md:994-1011

## Gaps

- No explicit U4f file lane in the plan (searched u4-build-plan.md :805-816 and Lanes :851-889); only 'shares the room's and the client's files'.
- No per-run size or time figure for -47's 1,000 connects, -21's 8 suites with 12 twins and 13 bites, option-A runs, or a never-acker run (searched c-u4r-runs.mjs, u4r-disk-plan.md, u4r-report.md, the card).
- night-2026-11-16.md does not exist yet (card :27 names it); only night-2026-11-10.md and night-2026-11-12.md are in N.
- u4r-brief.md is now rev 3.2 (the fold of the advisor's check), not rev 3.1; advisor-u4r-brief.md's L1-L6 checked rev 3.1.
- The card's ':1936-1937' note has moved: the option-A / buffer_depth_ticks_count note is at :1960-1964; :1936-1937 now holds point 31.
- The task's 'the window (§4.7)': §4.7 is Events (the queue cap, -22's site); the window is §4.5 (:292-304).
- net F104 is defined only in u4-scout-netcode.md:122; u4-build-plan.md only cites it at :806.
- The test plan has no per-mark section for -47, -20, -21 or -22; their definitions are ADR 0001 :1610-1612 and :1644 and marks.json; plan.md gives only components and §6.2 rows.
- No ruling question id is specific to ClockSync or the lead controller's design (no U4F-Q*); searched every advisor-*.md and *-full.md naming U4f.
- No Decision in history.md yet for KL-YES, C62H-KEEP or C62H-BOUND (card :17-19: none in at 14:50).
- The test plan is not under docs/ or spikes/: it is N/plan.md (rev 3.3).
