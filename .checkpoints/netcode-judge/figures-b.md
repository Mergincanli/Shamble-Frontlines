# figures-b: part b (U6j) spec sheet. test-engineer, read-only, 2026-11-24, worktree at b53eaf54
Plan (done): figures.rs/eval.rs read; marks.json by jq; SERIES.md rows; ADR rows; writers grepped in crates/; the
netcode notes. Then A (19 figures by mark), B (C-U6-9..11), C (entry 64, JDG-ONLY-1), D (size, order), E (rulings).
STATUS: COMPLETE. [unverified] marks what was not checked in code or data. Quotes from marks.json are its `notes`.
Writers today (string literals in crates/, judge excluded): rss_bytes, out_queue_count, work_ns, tick_panic.server_ns
(phase after), shots/reliable_events/projectile_spawns_count (server/src/tick_loop.rs), room_exit (super/src/main.rs:
311). NONE for the other 16 input series (advisor-u6-brief.md:34-44 agrees). Every figure is built on made-up logs.

## 0. Shared facts (verified at b53eaf54)
- Compute: `fn(run, set) -> Result<Ratio, Lack>` (figures.rs:768); `set` is the run's own set only (eval.rs:1203,
  :1327). `measure` refuses a figure over more than one run ("cannot be pooled over runs", eval.rs:331-336), so a figure
  check is per run; `from`/`window` are ignored for figures. Lack::Gap -> run INVALID unless a check fails (eval.rs:
  1242-1258); Waiting -> NOT-RUN (figures.rs:1010); Empty -> `empty`: fails, shows no bite (eval.rs:491-497,
  :1497-1511). Reports print "not available: ..." for any Lack (eval.rs:1548-1553), per valid run per part. Ratio::new
  panics on den 0 (ratio.rs:31-33), every op on i128 overflow (ratio.rs:24-27): each division needs a den-0 Gap, each
  sum a bound. Display goes through f64, 6 decimals (ratio.rs:149-159).
- Window has All/Measured/Warmup only (marks.rs); Phase has After (crates/log/src/lib.rs:174-178); figures pick phases
  themselves. tick_panic is written in phase AFTER (server/src/tick_loop.rs:784).
- Conventions named below: CV-GAP, every tick and second record carries its counters, 0 included: a missing or empty
  tick/second series is a Gap (README §2.3:152-155; eval.rs:315-327). CV-EVENT: a count of an event that never happened
  is a real 0 (same). CV-REF: a run compared against counts only with `always_rules` empty (eval.rs:382-386;
  figures.rs:122-129). CV-SRC: a name in a log of a process that does not write it is a Gap (figures.rs:222-247).
  CV-EMPTY: a pair comparison that compared nothing is Empty (README §2.3:180-182). CV-WAIT: a run compared with that
  is not in the set yet is Waiting (figures.rs:116-121).
- His U6-JUDGE (b) text (u6-brief.md:160-178, the sentence :168-169): "This changes the judge's code and those two texts, not marks.json, a
  pass mark or a number." So every figure is built against marks.json as it stands (no `pooled` flag, no new check).

### 0.1 What `compute(run, set)` cannot serve
- SIG: sync_cost_ratio needs parts N=24 and N=36's runs, possibly in other sets (-10: N=48 are -4's S30 runs, N=24/36
  "run 10 minutes each"); logging_cost_ratio needs "the part's 3 runs" (SERIES.md:279) while its check is per run.
  Lean: `Compute = fn(&Inputs) -> Result<Ratio, Lack>`, `Inputs { run, set, parts: &BTreeMap<String, Vec<&Run>> }`,
  `parts` = each kept part's VALID members, from a first pass in judge_mark before any check (members exist at
  eval.rs:1715-1722; validity = `validity` + setup checks, eval.rs:1204-1238); the 8 built figures read `run`/`set`
  unchanged. Weaker alternative: references by CV-REF only (as `against`): a reference failing V-S or pre:PM-0001-2
  would enter the mean, and a FAIL it causes outranks INVALID (eval.rs:1742-1745): a wrong result. Lean two-pass.
- FAILNUM: crash_close_max_ns must fail with no number ("a client with no room_end fails", SERIES.md:264). Lean
  `Lack::Fail(why)` -> Outcome::Fail, value None, note shown (shows a bite, unlike Empty). Not a sentinel number.
- ARITH: "the arithmetic printed/shown" (SERIES.md:284; ADR 0006:1387) cannot pass through a report (one Ratio's text,
  eval.rs:1548-1553). Lean: a figure may return a detail string, printed as "<value> (<arithmetic>)".

## A. The 19 figures (by mark)

### A1. sync_unit_cost, sync_cost_ratio (PM-0001-10)
1. Check "unit cost at 48 at most 1.1 x the unit cost at 24 and at 36": sync_cost_ratio `<=` "1.1", parts ["N=48"];
   report sync_unit_cost, no parts. Parts N=48 (L2 SS P2 30 Hz clients 48 soak S30), N=24, N=36, min 3, pre:PM-0001-2;
   V-S; bite n-squared. Notes: "each N = 48 run's unit cost against the mean unit cost of the 3 runs at N = 24 and,
   separately, at N = 36; the figure is the larger ratio, so the check runs per N = 48 run. Means, not p99s".
2. U = mean(tick.v.server_only_ns) / (header.clients x mean(tick.s.relevant_count)), measured ticks, room log
   (SERIES.md:293-294; ADR 0001:1598). With CV-GAP's length rule (every list holds `clients` entries), U = Sum w /
   Sum r exactly (den <= ~1e8 a run: a mean of 3 stays far inside i128; the unreduced form could overflow).
   Ratio = max over N in {24, 36} of U(run) / ((U1 + U2 + U3) / 3).
3. Needs N=24's and N=36's valid runs (SIG). REF-MIN: a reference part with fewer than `min` valid runs -> lean Waiting.
4. Gap: no room log; a measured tick without either series; a list not holding `clients` entries; Sum r = 0; a
   reference mean of 0. Waiting: REF-MIN. No Empty. Sum w = 0 is a real 0.
5. Writers: NONE for server_only_ns (SERIES.md:49) and relevant_count (:407) (U6a S1b, u6-brief.md:32).
6. Tests: PASS U48 110, N=24 100 x3, N=36 105 x3 -> 11/10; FAIL-24 U48 111 -> 111/100; FAIL-36 N=24 100, N=36 95 ->
   22/19; GAP a tick without relevant_count; WAIT N=36 with 2 valid runs; report 110. Plant: N=24 only (FAIL-36 passes).
7. Open: SIG, REF-MIN.

### A2. down_client_mean_max_bits, down_mean_ipv6_bits, r2_down_formula_bits (PM-0001-12)
1. Check "ES: each client's mean at most 160 kbps" `<=` 160000, parts ["ES at P2"]; reports r2_down_formula_bits parts
   ["SS at P2"], down_mean_ipv6_bits "if LG reaches RS over IPv6", no parts. Parts ES at P2 (L2 ES P2 clients 48), SS at
   P2 (+ soak S30), min 3, pre:PM-0001-2; V-S; bites B-N-ES, B-N-SS. Notes: "UDP payload + 28 B per datagram; kbps =
   1,000 bit/s. Reading (question): 'mean at most 160 kbps' per client is the largest client mean (a figure)"; "R2's
   formula uses each client's set, shots, spawns and events per second (lines 1764-1766), as PM-0006-17 does."
2. Max over slots of (Sum down_bits[slot]) / n_sec, measured seconds, room log (SERIES.md:268, :104). IPv6: Sum over
   client-seconds of (down_bits + 160 x down_datagrams_count) / (n_sec x C) (:269, :414). Formula (r2-netcode.md:
   1764-1766; item sizes r2:1742-1760): bits/s per client = 8 x (tick_hz x (90 + 15/2 x E + 47/5 x V) + 4 S + 24 R +
   11 J); 90 = 65 + 16 + 7 + 2 B; E = mean relevant_count (measured ticks); S, R, J = mean shots_count,
   reliable_events_count (x 8 B x 3 sends), projectile_spawns_count (11 B) per measured second (SERIES.md:284). Golden:
   E 47, V 15, S 72, R 10, J 528/25 -> 3653064/25 (146,122.56; R2 rounds 15.84 x P to 15.8 and prints 146.1, r2:1775).
3. Own run. 4. Gap: no room log; a measured second without down_bits (CV-GAP); a list not holding `clients` entries or
   changing length; down_bits outside the room log (CV-SRC, lean); 0 measured seconds; a formula input missing. Real 0s.
5. Writers: NONE for down_bits, down_datagrams_count, relevant_count; shots/reliable/spawns YES (tick_loop.rs:1188-1193).
6. Tests: PASS slot means 159,999 and 160,000; FAIL 480,003 over 3 s -> 160001; GAP a second without down_bits; IPv6 3 s
   of 1,000 bits and 2 datagrams -> 1320; the formula golden. Plants: mean over client-seconds for max over slots; 9.4
   -> 7.5 on V.
7. Open: R2-INPUTS (Medium): E3 has a soldier term ((P-1) x 7.5 B) and a vehicle term (V x 9.4 B); relevant_count is
   one count of "entities" (:407) and :284 names no V. Options: (a) E all at 7.5, V = mean tick.v.big_vehicles_count
   (written, tick_loop.rs:1173; SERIES.md:22) at 9.4 (double count if relevant_count holds vehicles); (b) a soldier-only
   count (a writer change); (c) (E - V) at 7.5 and V at 9.4. Lean: netcode-architect rules. IPV6-WHEN (Low): always, or
   only with params.lg_ipv6 true (SERIES.md:454, used by -25 only)? Lean always (a what-if on IPv4 counters); note it
   is the pooled mean while the check is the largest client mean (as :269 says). Cite fix (Low): :284 cites "ADR 0001
   lines 1754-1779"; those are r2's lines (ADR 0001:1479 cites "worked bandwidth estimate, lines 1754-1779").

### A3. up_client_mean_max_bits (PM-0001-13)
1. Check "each client's mean at most 48 kbps (IPv4, UDP payload + 28 B)" `<=` 48000, parts ["P2"]; report "P4: each
   client's mean" parts ["P4 (report only)"]. Parts P2 (SS P2 clients 48 soak S30, min 3), P4 (min 1), pre:PM-0001-2;
   V-S; bite B-N. Notes: "per client it is the largest client mean (question)."
2. A2's max over slots on second.s.up_bits (SERIES.md:298, :126), measured, room log; ADR 0001:1601. One helper
   `slot_mean_max(run, name)` serves A2 and A3. 3-4. As A2.
5. Writer: NONE for up_bits. 6. Tests: PASS 48,000; FAIL 144,003 over 3 s -> 48001; GAP a second without up_bits; the
   P4 report on a P4 run. Plant: the helper reads down_bits (a test with different up/down lists). 7. None new.

### A4. rss_slope_bytes_per_10min, queue_growth_count (PM-0001-15)
1. Checks "slope after warm-up within 1 MB per 10 minutes, high" `<=` 1000000, "..., low" `>=` -1000000; "every
   per-client queue flat: max of the last 10 minutes at most the max of the first 10" queue_growth_count `==` 0; no
   parts. Part S30 at P2 (SS P2 clients 48 soak S30, min 3), pre:PM-0001-2; V-S; bite leak. Notes: "queue lengths
   (outbound, unacknowledged events) per client from the server's counters"; "MB = 10^6 B. The slope method (least
   squares over the measured seconds) is proposed; 'flat' is the row's proposed reading."
2. Slope: x = `sec`, y = second.v.rss_bytes, measured, room log (SERIES.md:287, :93; ADR 0001:1603): value =
   Ratio::new(600 x (n Sxy - Sx Sy), n Sxx - Sx^2), all i128. Bound: n, x <= ~4,000, y <= ~1e9 -> num <= ~4e22, den <=
   ~2e13; the compare's cross-multiply (ratio.rs:136-138) stays near 1e23 (i128: 1.7e38); a 2-hour 1 GB test pins it.
   Queues: per slot of second.s.out_queue_count and of event_queue_count, 1 if max(last 600 measured s) > max(first
   600); summed (SERIES.md:283).
3. Own run. 4. Gap: < 2 measured seconds or all x equal (den 0); a second without rss_bytes or a queue list (CV-GAP);
   a duplicate `sec`; list length != `clients` or changing; two room logs; lean < 1,200 measured s for queues (windows
   overlap; S30's measured length [unverified]). Real 0.
5. Writers: rss_bytes YES (tick_loop.rs:1074), out_queue_count YES (:1133-1139); event_queue_count (SERIES.md:415) NO.
6. Tests: rss constant -> 0; y = 1,000 x sec -> 600,000 PASS; 2,000 x sec -> FAIL high; -2,000 x sec -> FAIL low; one
   second -> GAP. Queues: slot 1 max 5 then 6 -> 1 FAIL; equal -> 0; a second without event_queue_count -> GAP.
   Plants: sign flipped (the high/low pair swaps); x = record index (a run missing one middle second); `>=` for `>`.
7. Open: QWIN (Medium): "the last 10 minutes" by record count or by time? Lean by `sec` (a missing second record
   escapes CV-GAP). MB = 10^6 matches the values. No conflict.

### A5. session_memory_growth_count (PM-0001-24)
1. Check "memory per session flat over 10 minutes (PM-0001-15's rule, halves of the window)" `==` 0. Parts NewReno,
   Cubic (L2 P2 suite bottleneck, cc), min 1; V-S, clean-ref; bite blocking-send. Notes: "'Flat over 10 minutes'
   compares the second half's max with the first half's (... proposed)."
2. Per index of second.s.session_buffered_bytes, out_queue_count, event_queue_count: 1 if max(second half of measured
   seconds) > max(first half); summed (SERIES.md:290, :415-417); room log. Shared helper with A4 `growth_count`.
3-4. As A4. 5. Writers: out_queue_count YES; session_buffered_bytes (:417), event_queue_count NO.
6. Tests: PASS equal halves; FAIL session 0 buffered max 10 then 11 -> 1; GAP one second without session_buffered_bytes;
   5 seconds split. Plant: halves swapped (fires on a fall): the FAIL case passes.
7. Open: HALF (Low): the middle second of an odd count; lean by time, mid = (first + last) / 2 exact, second half
   `sec > mid`. session_buffered_bytes is "one per session", the queues "one per client": counted per (series, index).

### A6. kill_p99_ratio (PM-0001-27)
1. Check "after SIGKILL of one room, the others' p99 within 5 % of their own before it" `<=` "1.05", parts D1/D2/D3
   surviving rooms (density 8-rooms, session, killed false, min 7); killed-room parts (killed true, min 1); validity
   PM-0006-13; no bites. Notes: "Each room is its own run, the session its set"; "The kill comes after the 20 measured
   minutes; the survivors' ticks after it are phase `after`, and the figure compares their p99 with the measured p99
   ('within 5 %' read as not worse by more than 5 %: proposed, question)."
2. p99(tick.v.work_ns, ph after) / p99(work_ns, ph measured), room log, nearest rank via `stat_of` (eval.rs:276-293)
   (SERIES.md:278 "after the set's `kill` event (ph = after)"; ADR 0001:1618). Ratio::new(after, measured).
3. From `set`: the killed room's run (params.killed true), as evidence of the kill.
4. Gap: no room log; no measured or no after work_ns (CV-GAP); measured p99 0; two killed runs (lean). Waiting: no
   killed run in the set (CV-WAIT). No Empty.
5. Writers: work_ns YES (tick_loop.rs:1162); a `kill` event: NO writer, NO SERIES row (npt-super writes room_start and
   room_exit only, super/src/main.rs:305-311); survivors logging `after` ticks: U6's [unverified].
6. Tests: PASS 100 then 105 -> 21/20; FAIL 106; GAP no after tick; WAIT no killed run. Plants: ratio inverted (FAIL
   passes); Window::All for "before" (a high after tail catches it).
7. Open: KILL-EVENT (Medium): no `kill` event exists, and survivors count their own ticks; (a, lean) phase `after` plus
   the killed run present; (b) an npt-super `kill` record in the set. C4-KILL (Medium, scope): README §2.5:202-203
   "the exemption from `complete` holds only if npt-super's log records that kill of that slot (built at U6: C4)";
   advisor-u1.md:124-126 "Fixture at U6"; `always_rules` exempts any `killed` log today (eval.rs:576-584). A waiting
   judge change the hand-off does not list: in part b with (b) (needs the record's name and fields), or named deferred.

### A7. bulk_control_p99_rise_ns, one_shot_dropped_count (PM-0001-38)
1. Checks "control stream p99 message delay with a transfer minus without at most 5 ms" `<=` 5000000; "0 dropped (sent
   by LG's clients minus received by the room)" `==` 0; no parts. Part L2 P3 LG (suite one-shots), min 1; bite
   bulk-on-control. Notes: "The control stream's message delays are written apart for the seconds with and without a
   transfer (`second.s.control_delay_bulk_ns`, `second.s.control_delay_idle_ns`); the figure is the difference of their
   p99s."
2. Rise = p99(all bulk samples) - p99(all idle samples), measured seconds, LG bots logs pooled (SERIES.md:263,
   :412-413; ADR 0001:1630); may be negative. Dropped = Sum one_shots_sent_count (LG logs) - Sum
   one_shots_received_count (room log), EVERY phase ("over the run", :282; :82, :410); negative fails `== 0`.
3. Own run. 4. Gap: no LG or room log; no bulk or no idle sample in the window (a gap, not 0: the "10 transfers"
   check counts transfers); a second without one_shots_* (CV-GAP); a name in another process's log (CV-SRC, lean).
5. Writers: NONE for control_delay_*, one_shots_sent_count, one_shots_received_count.
6. Tests: PASS 7 ms vs 2 ms -> 5000000; FAIL 5000001; GAP no bulk sample; dropped 10,000 - 10,000 PASS; 10,000 - 9,999
   FAIL; sent in warm-up and received in measured -> 0. Plants: measured-only for dropped; p50 for p99.
7. Open: BULK-WIN (Low): :263 names no window; lean measured.

### A8. crash_close_max_ns (PM-0001-59)
1. Check "every client gets RoomEnded(crash) or a close within that second (0 left open)" `<=` 1000000000. Part "10 S30
   rooms, one tick panic each" (L2 SS suite tick-panic), min 10; bite no-catch. Notes: "The figure joins each LG
   client's close (on its server-clock estimate) with the room's panic tick."
2. Max over the run's clients of event.room_end.server_est_ns (LG bots log) - event.tick_panic.server_ns (room log),
   any phase (SERIES.md:264, :419, :423; ADR 0001:1617); "a client with no room_end fails" (FAILNUM).
3. Own run. 4. Gap: not exactly one tick_panic; a room_end without an integer server_est_ns or a client id; no LG log.
   Fail without a number: a client of the room's `clients` with no room_end. No Waiting, no Empty.
5. Writers: tick_panic.server_ns YES (tick_loop.rs:784, phase after); room_end NO (npt-bots).
6. Tests: PASS slowest 999,999,999; FAIL 1,000,000,001; FAIL slot 7 has no room_end; GAP two tick_panic; a negative
   difference counts as is. Plants: min for max; the missing-client rule dropped.
7. Open: CLIENT-ID (Medium): SERIES.md:419 gives room_end no client field; lean `slot` (as admit and session_end carry:
   SERIES.md:355; lab/src/scenario.rs:1974-1976), each slot once, a duplicate a Gap.

### A9. egress_interface_bytes, egress_metric_vs_interface_ratio, r2_egress_formula_bytes, egress_vs_r2_ratio (0006-17)
1. Checks (parts ["one room-hour, 48 bots all in view"]: suite egress, clients 48, interest off, min 1):
   egress_vs_r2_ratio `>=` "0.8", `<=` "1.2"; egress_metric_vs_interface_ratio `>=` "0.95", `<=` "1.05". Reports
   r2_egress_formula_bytes, egress_interface_bytes (no parts), egress_interface_bytes for "interest on (reported)". Bite
   double-send. Notes: "The interface's transmitted-byte counter is read at the start and end (`event.iface.tx_bytes`);
   the room's metric at the same moments (`event.room_metric.sent_bytes`). R2's formula (r2 lines 1764-1766) with the
   run's P, V, shots and projectile spawns; at R2's own counts it is 3.156 GB".
2. Interface = last - first iface.tx_bytes (SERIES.md:270, :418); metric ratio = (last - first sent_bytes) / interface
   (:271, :420); formula bytes = r2_down_formula_bits x clients x 3,600 / 8 (:285); egress_vs_r2 = interface / formula
   (:272); ADR 0006:1387. Golden: A2's counts, 48 clients -> 3,156,247,296 B.
3. Own run. 4. Gap: not exactly two iface (or room_metric) events, start and end by record order (README §1:9-10); a
   non-integer field (eval.rs:219-226); interface <= 0; formula inputs as A2.
5. Writers: NONE for iface.tx_bytes, room_metric.sent_bytes (other `sent_bytes` literals are other records).
6. Tests: PASS golden counts with tx 3,156,247,296 -> 1; FAIL 2.0 (double); 0.8 and 1.05 exactly PASS; 1.050001 FAIL;
   GAP three iface events; GAP end < start. Plants: x 3,600 dropped; start and end swapped (must be a Gap).
7. Open: EGRESS-SPAN (Medium): x 3,600 fixed (:285) while the counter spans the real interval; lean as written, the
   span printed (ARITH). EGRESS-INPUTS (Low): ADR 0006:1387 names "P, V, shots and projectile spawns" (no events),
   :285 says "as r2_down_formula_bits" (events in); lean the -12 form. Counts' window: lean measured [unverified].

### A10. logging_cost_ratio (PM-0006-19)
1. Check "pooled p99 of the 3 'on' runs at most 1.02 x the pooled p99 of the 3 'off' runs (54,000 ticks each)" `<=`
   "1.02", parts ["logging on (3 runs)"] (suite logging-cost, min 3). Validity PM-0006-19 (Rule::LoggingOff,
   eval.rs:918-957); bite busy-50us. Notes: "The 'off' runs have role `logging-off` in the same set. The pooled
   comparison is a figure: `against` combines per-run statistics, not pooled ones (question)."
2. p99(work_ns over the measured ticks of the part's valid runs in this set, pooled) / p99(the same over the set's
   logging-off runs, pooled), nearest rank (SERIES.md:279; ADR 0006:1389). Each on run gets the same value.
3. Needs the part's runs (SIG) and the set's logging-off runs (as Rule::LoggingOff reads them, eval.rs:920-924).
4. Gap: an off run breaking `always_rules` (CV-REF); no work_ns (CV-GAP); off p99 0; lean > 3 off runs. Waiting: < 3
   off runs (CV-WAIT).
5. Writers: work_ns YES; the logging-off role and suite come from run scripts (not built).
6. Tests: PASS 102 vs 100 -> 51/50; FAIL 103; WAIT 2 off runs; GAP an off run with no footer; a case where the mean of
   per-run p99s passes but the pooled p99 fails. Plants: pool only this run; off runs taken by `role != measured`.
7. Open: ON-POOL-SET (Low): lean pool only the part's runs in this run's set. TICKS-54000 (Low): in the name only; judge
   nothing. Rule::LoggingOff skips CV-REF on off runs (eval.rs:920-948): a pre-existing Medium, fix here or defer.

### A11. alloc_twin_diff_count (PM-0006-20)
1. Check "on every tick after the warm-up, the tick thread's allocation count with logging on equals the count with
   it off (tick by tick)" `==` 0. Part "logging on" (suite logging-alloc, min 1); bite planted-format. Notes: "The 'off'
   run has role `logging-off` and is named in the 'on' run's `params.twin`; the figure compares the two runs'
   `tick_allocs_count` at equal tick numbers."
2. Measured tick numbers whose tick.v.tick_allocs_count differs between run and twin, a tick in one only counting (as
   figures.rs:205-213) (SERIES.md:262, :55; ADR 0006:1390).
3. Twin by `twin_of` (figures.rs:99-149), role `logging-off`. The bite run: B2.
4. Gap: twin_of's gaps; twin role not logging-off; a measured tick without tick_allocs_count. Waiting: twin not in set.
   Empty: no measured tick in either (CV-EMPTY).
5. Writer: NONE for tick_allocs_count (U6c S5 "PM-0006-20's allocator twin", u6-brief.md:34).
6. Tests: PASS equal; FAIL tick 3 differs; FAIL a tick in one only; WAIT; GAP twin role `twin`; EMPTY. Plants: compare
   by record index (a twin with one extra warm-up record); drop one-sided ticks.
7. Open: ONE-SIDED (Low): lean counts (as twin hash), not a Gap.

## B. The three waiting judge changes

### B1. C-U6-9: -30's form in twin_hash_mismatch_count (plan.md:1254; u6-brief.md:57-58)
- Today the twin must have role `twin` (figures.rs:196-203). -30's six runs are all measured, each naming its other-
  transport run (-30 notes; check "... (each run against its other-transport twin)"; parts RS and C, suite
  both-transports, P0, min 6, late-zero), so each is INVALID today.
- Rule: accept role `twin` (as now) OR all of: both runs role `measured`; the twin's `params.twin` names the run back;
  `params.transport` present in both, each `wt` or `ws`, different. Any other measured twin is a Gap: same or missing
  transport (keeps -21's `twin-is-measured` INVALID; its logs carry no transport, fixtures/pm-0001-21/twin-is-measured/
  twin-lab.jsonl:1), one-way naming, `both`. twin_of's rules stay. The wallclock bite keeps the role-twin form (C4,
  eval.rs:552-565). Lean data-defined (no mark id in figures.rs). PARAM-TRANSPORT (Low): SERIES.md:449 "wt or ws" vs
  :903 "`wt`, `ws` or `both`".
- Tests (fixtures/pm-0001-30, real marks.json; base gains the C part's 6 measured room logs, 3 recordings x wt/ws, 3
  ticks with `h` and late_count 0): base -> C part PASS (mark NOT-RUN: RS 0 of 6); `twin-differs` -> FAIL; `same-
  transport` -> INVALID; `not-mutual` -> INVALID; `twin-missing` -> NOT-RUN; -21 `twin-is-measured` still INVALID; the
  late-command tests keep their verdicts (tests/fixtures.rs:196-221). Plants: transport check removed; mutual check removed.
- Texts: SERIES.md:297; figures.rs:191-195's doc; fixtures/README.md rows.

### B2. C-U6-10: PM-0006-20's logging-off twin vs C4 (plan.md:1254)
- Verified conflict: `always_rules` refuses a run naming a bite unless role `bite` or `twin` (eval.rs:552-559; README
  §4:284-286); `twin_of` runs `always_rules(twin)` (figures.rs:122-129) and with `bite` as a field demands the same bite
  (:139). The planted-format bite run's off twin (role logging-off, same bite) is refused -> Gap -> never shown.
- Options (TWIN-ROLE): (A, lean) the bite run's off twin is the same bites build, same bite, role `logging-off`;
  `twin_of` accepts it only when the run is role `bite` and the twin's one broken rule is the role-list rule; C4 stays
  for every other run. (B) a measured-build off twin (bite null), `twin_of` without "bite": the binaries differ, so a
  diff may come from the build (weak bite evidence). (C) `logging-off` added to C4's list everywhere: loosens C4 for
  -19's off pool (Rule::LoggingOff takes any logging-off run, eval.rs:920-924): reject.
- alloc_twin_diff_count on the bite: the planted `format!` allocates on every logged tick, so the count is >= 1 and
  `== 0` fails: shown. Tests: bite + same-bite logging-off twin -> Shown; that twin with role `measured` -> Gap; a
  measured on run whose off twin names a bite -> Gap; a -19 set's logging-off run naming a bite still breaks always_rules.
  Plant: the exemption widened to any run (the measured-on case passes). Texts: SERIES.md:262; README §4:284-286.

### B3. C-U6-11: -19's join hash under the coverage rule (plan.md:1254; replan-units.md:40-41; u6-facts.md:344-345)
- Today: one span per slot, first to last resolved tick (figures.rs:483-486); (c) every room hash inside resolved
  (:487-499); (d) the span covers the room's first..last measured hash (:500-534). README §1:79-81: "every client is
  admitted, and its first snapshot resolved, in the warm-up (a client admitted later is -19's join, built with U6)".
  -19's kept part "LG joins" (L2 P2 suite joins, LG) has 100 joins into a running S30 room with bots leaving and
  rejoining (replan-units.md:43-44): room hashes sent to a leaving client go unresolved -> (c) Gap; a slot's later
  session starts after the room's first measured hash -> (d) Gap.
- Rule (lean; Q10 "a marks/README.md rule, chosen in U6j's brief with test-engineer's read", u6-brief.md:186-187):
  spans per SESSION, cut at the room log's `admit` events (slot, session, `t`: SERIES.md:355; lab/src/logs.rs:304-309).
  A session runs from the client's first resolved tick at or after its admit `t` to its last resolved tick before the
  slot's next admit. Head: the session's first room hash is at or after that first resolved tick. (c) per session.
  Tail of a session that ends early: room hashes after the client's last resolved tick are in flight, allowed only up
  to a recorded `session_end` for the slot (else a Gap: a silent stop must not pass); a room hash after it is a Gap; the
  last session keeps (d).
- Browser writer: NO. -19's "ML Chrome" part is moved ("Slice 3, with the human run"); replan-units.md:40-41: "its
  ML-browser `decoded_hash` writer MOVES with -19's R7/ML parts (the plain client has no hash)"; `source` stays
  client|bots (figures.rs:227-229). Must land first: entry 64 (a) (plan.md:1254), and entry 64 says all of it "at the
  latest before U6 builds -19's figure" (C1).
- Tests: two sessions, leave in flight (room hashes 10-11 after last resolved 9, session_end 11, re-admit 20) -> PASS;
  the same without session_end -> INVALID; mid-measured admit at 50, first resolved 50 -> PASS; first resolved 52 with
  a room hash at 50 -> INVALID; a room hash after session_end -> INVALID; every -17 fixture keeps its verdict. Plants:
  spans per slot again; the session_end requirement dropped.
- Texts: README §1:79-82 (the warm-up sentence gains the joins); SERIES.md:267's coverage clause ("per slot" -> "per
  session"). SESSION-END-LOG (Medium): on L2 `admit` and `session_end` with slot and `t` must be in the ROOM log;
  session_end is the L1 lab log's today (lab/src/scenario.rs:1971-1979).

## C. The two standing items (first)

### C1. u4s-carries.md entry 64 (:455-481): each shown failing under its plant, passing restored
- Entry 64 says "Before any other edit to crates/judge/src"; part a edited src first (netcode-judge.md:91-98): the
  advisor rules whether part a stands (card lean: yes).
- (a) TE2-1, Medium, OPEN: the Arm test (tests/engine.rs:1436-1493) has LG cases only. Add role C and role ML aarch64
  `client` logs with `wrong` hashes, room x86_64, replay agreeing -> FAIL 1, "aarch64 only" 0. Plant: figures.rs:248
  `h.machine.role == "LG" &&` dropped. Text: fixtures/README.md's l1-aarch64-claim row, "(TE2-1: carry 64 (a))" ->
  "the Arm rule's LG key is shown by the engine Arm test's role C and ML cases (TE2-1)".
- (b) TE2-2, Low, OPEN: `u4j_ws_far_better_past_the_cap_is_invalid` (engine.rs:2438-2463) asserts "ws-practice: " only.
  Assert "its bound line is not PM-0001-18's ws_bound_line, byte for byte" (practice.rs:504); add probe A-pinned (the
  record carries marks.json's line; FAR BETTER at s = 1.9, chance ruling) -> INVALID 'is "FAR BETTER" at s = 1.9'.
  Plants: the pin (practice.rs:495-505); `s < bound.band_low` (practice.rs:578).
- (c) TE2-3, Low, OPEN: the bite-run practice test (engine.rs:2764) has cases (1)-(4); add (5), two measured runs of one
  build with different practice runs in force (the later after a second practice run with `second_ruling`) -> bite
  INVALID "have practice runs ... in force". Plant: that refusal in `find` (practice.rs:460-466).
- (d) TE2-4, Low, OPEN: a coverage gap and an LG mismatch waiting for its re-decode in one run -> INVALID, never
  NOT-RUN. Plant: figures.rs:713-718's gap and waiting checks swapped.
- (e) words, Low, OPEN: eval.rs:1821 "an aarch64-only mismatch is a cross-architecture finding" -> "an LG aarch64
  mismatch (the room on x86_64) whose x86-64 re-decode agrees is a cross-architecture finding"; practice.rs:4 "f =
  max(1, s)" -> "f = max(1, s), or 1 for FAR BETTER". No plant; no test asserts the old text (grep of tests/: none).
- (f) SEC-R2-N2, Low, OPEN: `summary_line` (lib.rs:49-52) prints the first reason raw. Escape every control character.
  Test: a run id "r1\nPM-0001-4   PASS" in a reason -> one line printed. Plant: the escape removed.

### C2. JDG-ONLY-1, Medium (history.md:4609-4614)
- Verified: main.rs:94 `marks.marks.retain(..)` runs before `judge` (:106); `Rule::Pre` looks in `ctx.marks.marks`
  (eval.rs:960-962), so `--only PM-0001-4` alone reads INVALID "pre:PM-0001-2: not in marks.json". Interim: name
  PM-0001-2 too (history.md:4614).
- Fix: `judge_only(marks, logs, only)` judges the filtered marks with `Ctx { marks: <whole file> }` (eval.rs:1830-1847);
  `judge` = `judge_only(.., &[])`; main.rs keeps the unknown-id refusal (:84-93), drops the retain.
- Tests: cli.rs, real marks.json, `--logs fixtures/pm-0001-4/base --only PM-0001-4` -> one line, PASS (fixtures/
  README.md: -4 base PASS); an engine test, a two-mark file judged with only the pre: mark. Plant: the retain restored.

## D. Size and build order (added lines: code / tests incl. fixtures / texts / sum)
| Block | Code | Tests | Texts | Sum |
|---|---|---|---|---|
| J0 entry 64 (a)-(f) | 17 | 200 | 3 | 220 |
| J0 JDG-ONLY-1 | 20 | 45 | 0 | 65 |
| Infra: SIG two-pass `Inputs`, FAILNUM, ARITH | 215 | 60 | 10 | 285 |
| B1 C-U6-9 (6 base + 4 case fixture logs) | 40 | 120 | 10 | 170 |
| B2 C-U6-10 | 30 | 70 | 6 | 106 |
| B3 C-U6-11 | 90 | 120 | 10 | 220 |
| A1 sync x2 | 60 | 110 | 2 | 172 |
| A2 + A3 down, up, IPv6 (one helper) | 55 | 120 | 2 | 177 |
| A2 + A9 R2 formula, egress x4 | 120 | 180 | 4 | 304 |
| A4 + A5 slope, growth x2 (one helper) | 100 | 190 | 2 | 292 |
| A6 kill (C4-KILL if ruled in: +60) | 45 | 90 | 2 | 137 |
| A7 bulk, dropped | 55 | 110 | 2 | 167 |
| A8 crash close | 55 | 90 | 2 | 147 |
| A10 logging cost | 50 | 100 | 2 | 152 |
| A11 alloc twin | 45 | 90 | 2 | 137 |
| FIGURES list (19 `planned` -> `Figure`), docs | 25 | 0 | 0 | 25 |
| **Total** | **1,022** | **1,695** | **59** | **about 2,780** |
- Range about 2,400 to 3,300 (u6-brief/replan-units.md:63's 1,650-3,050 for J1+J2, plus J0 ~285 and the infra found
  here). At past growth of 1.35 to 1.5 (u6-brief.md:138): about 3,250 to 4,950, so the 4,900 stop is in reach at the
  top. Cuts, in order: C4-KILL deferred by name; ARITH as values only; one plant per figure instead of two.
- Order (each block compiles with its tests, clippy and fmt before the next; each <= ~400): b1 J0 (285); b2 Infra
  (285: the 8 built figures on `Inputs`, all 98 tests unchanged); b3 B1 + B2 + A11 (413); b4 B3 (220, -17 fixtures
  unchanged); b5 A2 + A3 + A7 (344); b6 A4 + A5 (292); b7 A1 + A6 (309); b8 R2 formula + egress (304); b9 A8 + A10 +
  the SERIES.md, README.md and fixtures/README.md texts (299).

## E. Rulings needed before the build (lean in brackets)
- SIG: figures get the mark's parts' valid runs (two-pass) [yes; the against-style shortcut can FAIL on an invalid ref].
- REF-MIN: a reference part under its min valid runs [Waiting]. FAILNUM: `Lack::Fail` [yes]. ARITH: detail text [yes].
- TWIN-ROLE (C-U6-10): [A: the bite pair's logging-off twin accepted in twin_of only].
- C-U6-9 form: data-defined (measured, mutual, transports wt/ws differing) [yes].
- Q10 / C-U6-11 span start: per-session spans at the room's `admit`, tail allowed only to a recorded `session_end` [yes].
- C4-KILL: in part b with a named npt-super kill record, or deferred by name [the netcode chat names the record first].
- KILL-EVENT: phase `after` + killed run present [yes]. CLIENT-ID: room_end `slot` [yes].
- R2-INPUTS: the formula's soldier and vehicle terms from relevant_count [netcode-architect]. EGRESS-SPAN [x 3,600].
- QWIN [by time]; HALF [by time]; BULK-WIN [measured]; IPV6-WHEN [always]; ON-POOL-SET [this set]; ONE-SIDED [counts].
- Part a's edit before entry 64 [stands; the advisor rules].
