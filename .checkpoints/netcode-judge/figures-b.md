# figures-b: part b (U6j) spec sheet, test-engineer, read-only, 2026-11-24
Plan: (1) read figures.rs/eval.rs helpers + Compute signature; (2) jq marks.json per mark; (3) SERIES.md Figures rows +
series rows; (4) ADR rows; (5) grep writers in crates/; (6) notes (plan.md:1254, u6-brief, u6-facts, u4s-carries 64,
history JDG-ONLY-1, replan-*); then write A (19 figures, grouped by mark), B (C-U6-9/10/11), C (entry 64 + JDG-ONLY-1),
D (size + order). Each block appended when done. Unverified claims are marked [unverified].

## Progress
- plan written
- read: figures.rs whole; eval.rs (measure:296-338 figures single-run only "cannot be pooled"; Window has All/Measured/
  Warmup only, no After though npt_log Phase has After: crates/log/src/lib.rs:174-178); marks.json checks/reports of
  the 19 figures (jq) + notes of -10,-12,-13,-15,-24,-27,-38,-59, 0006-17,-19,-20.
- read: SERIES.md Figures rows 258-299, figures-only rows 402-424, input rows (:49,:55,:60,:82,:89-94,:104,:126);
  README §1, §2.1-2.5; ADR 0001 rows -10,-12,-13,-15,-24,-27,-38,-59,-30,-19; ADR 0006 -17,-19,-20; r2:1740-1785.
- writers in crates/ today (string literals, judge excluded): rss_bytes, out_queue_count, work_ns (server tick_loop),
  tick_panic.server_ns (tick_loop.rs:784, phase AFTER), shots/reliable_events/projectile_spawns_count (tick_loop.rs:
  1188-1193), room_exit (super/main.rs:311). NONE for server_only_ns, relevant_count, down_bits, down_datagrams_count,
  up_bits, event_queue_count, session_buffered_bytes, one_shots_*, control_delay_*, room_end, iface, room_metric,
  tick_allocs_count, a `kill` event (advisor-u6-brief.md:34-44 lists them unwritten).
- notes read: plan.md:1254 (C-U6-9/10/11), u6-brief §1/§2/§8, u6-facts lines, u4s-carries 64 (:455-481), 62(d),
  history JDG-ONLY-1 (:4609-4614), replan-units:40-41 (C-U6-11 KEEP for LG; ML browser writer MOVES), :63 (19 figures,
  1,650-3,050), replan-marks rows (all 11 marks KEEP; -33 MOVED; -19 ML part MOVED).
- found: SERIES.md r2_down_formula_bits cites "ADR 0001 lines 1754-1779": those are r2's lines (ADR 0001:1479 cites
  r2 1754-1779); the ADR's lines there are the Brave steps/Migration plan. Wording fix in the row.
- C4's kill record (README §2.5:202-203, advisor-u1.md:124-126 "built at U6 ... Fixture at U6") is a 4th waiting judge
  change NOT in the hand-off list; flagged in §0.

## 0. Shared facts every block uses (verified in the code at b53eaf54)
- Compute today: `fn(run, set) -> Result<Ratio, Lack>` (figures.rs:768); `set` = the run's own set only (eval.rs:1203,
  :1327). `measure` refuses a figure over more than one run: "figure {name} cannot be pooled over runs" (eval.rs:331-336),
  so a figure check is always per run; `from` and `window` are ignored for figures (eval.rs:331). Lack::Gap -> run
  INVALID unless a check fails (eval.rs:1242-1258); Lack::Waiting -> NOT-RUN (figures.rs:1010); Lack::Empty -> check
  `empty`: fails, shows no bite (eval.rs:491-497, :1497-1511). Reports: `report_values` prints "not available: ..." for any
  Lack (eval.rs:1548-1553), per valid run of each part (eval.rs:1559-1590). Ratio::new panics on den 0 (ratio.rs:31-33)
  and every op panics on i128 overflow (ratio.rs:24-27): each new division needs a den-0 guard (a Gap), each sum an
  overflow bound. Display prints a non-integer through f64 to 6 decimals (ratio.rs:149-159).
- Window: marks.rs's Window has All/Measured/Warmup only; npt_log Phase has After (crates/log/src/lib.rs:174-178).
  Figures read phases themselves. tick_panic is written in phase AFTER (server/src/tick_loop.rs:784).
- Conventions (named per block): CV-GAP "every tick and second record carries its counters, 0 included" (README §2.3
  :152-155; eval.rs:315-327): a missing/empty tick or second series is a Gap. CV-EVENT: a count of an event that never
  happened is a real 0 (same lines). CV-REF: a run compared against counts only if `always_rules` is empty
  (`reference`, eval.rs:382-386; `twin_of`, figures.rs:122-129). CV-SRC: a name in a log of a process that does not
  write it is a Gap (`source`, figures.rs:222-247). CV-EMPTY: a comparison of pairs that compared nothing is Empty
  (README §2.3:180-182). CV-WAIT: a run compared with that is not in the set yet is Waiting (figures.rs:116-121).
- U6-JUDGE (b), his yes (u6-brief.md:166-176): "This changes the judge's code and those two texts, not marks.json, a
  pass mark or a number." So every figure must be built against marks.json as it stands (no `pooled`, no new check).

### 0.1 The signature: what `compute(run, set)` cannot serve (needs a ruling: SIG)
- sync_cost_ratio needs the runs of parts N=24 and N=36, which may sit in other sets (-10 notes: N=48 are -4's S30
  runs; N=24/36 "run 10 minutes each"); logging_cost_ratio needs "the part's 3 runs" (SERIES.md:279) while its check
  is per run (marks.json -19 has no `pooled`, and `measure` cannot pool a figure). `set` alone gives neither.
- Proposal (lean): `Compute = fn(&Inputs) -> Result<Ratio, Lack>`, `Inputs { run, set, parts: &BTreeMap<String,
  Vec<&Run>> }`, `parts` = each KEPT part's VALID members (its own rules and setup checks held), built by judge_mark
  in a first pass before any check runs (members are already computed per part at eval.rs:1715-1722; validity is
  `validity` + setup checks, eval.rs:1204-1238). Existing 8 figures take `inputs.run`/`inputs.set` unchanged. Size
  ~150 code, ~50 tests. Alternative (cheaper, weaker): references by CV-REF only (`always_rules`), as `against` does:
  a reference run failing V-S or pre:PM-0001-2 would still enter the mean, so a FAIL could come from an invalid
  reference (FAIL outranks INVALID, eval.rs:1742-1745): a High-class wrong result. Lean: the two-pass form.
- A figure that must FAIL without a number (crash_close_max_ns: "a client with no room_end fails", SERIES.md:264):
  Lack has no such case. Lean: `Lack::Fail(why)` -> `Missing::Fail` -> Outcome::Fail, value None, the note shown
  (shows a bite, unlike Empty). ~25 lines. Alternative: a sentinel value (rejected: prints a fake number).
- "the arithmetic printed/shown" (SERIES.md:284; ADR 0006 PM-0006-17 "the arithmetic shown"): a report value is one
  Ratio's text (eval.rs:1548-1553). Lean: a figure may return a detail string beside its value (`Ok((Ratio,
  Option<String>))` or a `Figure::explain`), printed in ReportOut as "<value> (<arithmetic>)". ~40 lines. Ruling: ARITH.

## A. The 19 figures (grouped by mark)

### A1. sync_unit_cost, sync_cost_ratio (PM-0001-10)
1. Check "unit cost at 48 at most 1.1 x the unit cost at 24 and at 36": figure sync_cost_ratio, `<=` "1.1", parts
   ["N=48"]. Report "unit cost per part: ..." figure sync_unit_cost, no parts (every part, per valid run). Parts N=48
   (L2 SS P2 30 Hz clients 48 soak S30), N=24, N=36 (clients 24/36), min 3 each, pre:PM-0001-2; mark V-S. Bite
   n-squared. Notes (PM-0001-10): "each N = 48 run's unit cost against the mean unit cost of the 3 runs at N = 24 and,
   separately, at N = 36; the figure is the larger ratio, so the check runs per N = 48 run. Means, not p99s (the row).
   The bite run should match N = 48's select (clients 48) so the judge reads the figure."
2. U = mean(tick.v.server_only_ns) / (header.clients x mean(tick.s.relevant_count)), measured ticks, room log
   (SERIES.md:293-294; ADR 0001:1598). Exact and small: with CV-GAP's length rule (each relevant_count list holds
   `clients` entries on every measured tick), mean(r) = Sum r / (n_t x C), so U = Sum w / Sum r (ns per client-entity;
   den <= ~1e8 per run, so a mean of 3 never nears i128). Ratio = max over N in {24,36} of U(run) / ((U1+U2+U3)/3).
3. Needs the N=24 and N=36 parts' valid runs (SIG). A reference part with fewer than its `min` valid runs: lean
   Waiting (no FAIL on a partial reference); flag REF-MIN.
4. Gaps (CV-GAP): no room log; a measured tick without server_only_ns or relevant_count; a list not holding the room
   header's `clients` entries; Sum r = 0 (den 0). Waiting: a reference part with < min valid runs. Empty: none (a
   unit cost is no pair comparison). A real 0: Sum w = 0 gives U = 0 (passes; fine). A reference mean of 0 -> Gap.
5. Writers: NONE today for server_only_ns (SERIES.md:49 npt-server) and relevant_count (SERIES.md:407 npt-room;
   advisor-u6-brief.md:37-44 lists both unwritten; U6a S1b "the interest writers", u6-brief.md:32). Built on made-up logs.
6. Tests: PASS N=48 U=110, N=24 runs U=100,100,100, N=36 U=105 x3 -> ratio 11/10 PASS at "1.1"; FAIL U48=111 -> 111/100;
   GAP one N=48 tick lacks relevant_count -> INVALID; WAIT N=36 has 2 valid runs -> NOT-RUN; unit-cost report = 110.
   Plant: drop the `max` (use N=24 only) -> the N=36-larger case passes wrongly; second plant: mean over all three
   parts' runs.
7. Open: SIG/REF-MIN as above. "header.clients" vs list length: same under the length rule (no choice left).
   SERIES.md:294 calls U "a decimal": Display prints 6 decimals (fine, reported only).

### A2. down_client_mean_max_bits, down_mean_ipv6_bits, r2_down_formula_bits (PM-0001-12)
1. Check "ES: each client's mean at most 160 kbps": down_client_mean_max_bits `<=` 160000, parts ["ES at P2"].
   Reports: "SS: R2's formula at the run's own counts" r2_down_formula_bits parts ["SS at P2"]; "the IPv6 figure, if
   LG reaches RS over IPv6" down_mean_ipv6_bits, no parts (both parts). Parts ES at P2 (L2 ES P2 30 Hz clients 48),
   SS at P2 (+ soak S30), min 3, pre:PM-0001-2; mark V-S; bites B-N-ES, B-N-SS. Notes (PM-0001-12): "Counted per client
   at the server's send side, UDP payload + 28 B per datagram; kbps = 1,000 bit/s. Reading (question): 'mean at most
   160 kbps' per client is the largest client mean (a figure)"; "R2's formula uses each client's set, shots, spawns
   and events per second (lines 1764-1766), as PM-0006-17 does."
2. down_client_mean_max_bits = max over slots of Sum_sec down_bits[slot] / n_sec, measured seconds, room log
   (SERIES.md:268, :104 "one per client: bits sent ... UDP payload + 28 B"). Exact: Ratio::new(sum, n).
   down_mean_ipv6_bits = Sum over client-seconds (down_bits + 160 x down_datagrams_count) / (n_sec x C) (SERIES.md:269;
   :414 down_datagrams_count). r2_down_formula_bits: E3 at the run's counts (r2-netcode.md:1764-1766): bits/s per client
   = 8 x (tick_hz x (90 + 15/2 x E + 47/5 x V) + 4 x S + 24 x R + 11 x J), 90 = 65+16+7+2 B, S shots/s, R reliable
   events/s (x 8 B x 3 sends), J projectile spawns/s (x 11 B) (r2:1757-1759 per-item sizes); E = mean relevant_count
   (measured ticks), S/R/J = means of second.v.shots_count, reliable_events_count, projectile_spawns_count over measured
   seconds (SERIES.md:284). Golden: E=47, V=15, S=72, R=10, J=528/25 -> 3653064/25 bits/s (= 146,122.56; R2 rounds
   15.84 to 15.8 x P and prints 146.1 kbps, r2:1764, :1775).
3. Own run only; `compute(run,set)` serves all three (no SIG need).
4. Gaps: no room log; a measured second without down_bits (CV-GAP); lists not holding `clients` entries or changing
   length; down_bits in a non-room log (CV-SRC, lean); 0 measured seconds; for the formula, any of its five inputs
   missing. No Waiting, no Empty. A real 0 bits is a real 0.
5. Writers: NONE for down_bits, down_datagrams_count, relevant_count (advisor-u6-brief.md:37, :42). shots_count,
   reliable_events_count, projectile_spawns_count ARE written (server/src/tick_loop.rs:1188-1193; lab logs.rs:875-878).
6. Tests: PASS 2 slots x 3 s, means 159,999 and 160,000 -> 160000; FAIL one slot 480,003 bits over 3 s -> 160001; GAP a measured second lacking down_bits -> INVALID; ipv6: 3 s of 1,000 bits and 2 datagrams -> 1320;
   formula golden above. Plant: mean over all client-seconds instead of max over slots (the pass/fail pair flips);
   formula plant: 9.4 -> 7.5 for V.
7. Open: R2-INPUTS (Medium): the formula has a soldier term ((P-1) x 7.5 B) and a vehicle term (V x 9.4 B), but
   relevant_count is one count of "entities" (SERIES.md:407); SERIES.md:284 names no V source. Options: (a) E =
   mean relevant_count, all at 7.5 B, V from mean tick.v.big_vehicles_count (written: tick_loop.rs:1173) at 9.4 B
   (double counts vehicles if relevant_count includes them); (b) a soldier-only relevant count added (a writer change);
   (c) E - V at 7.5 B and V at 9.4 B (assumes relevant_count counts vehicles). Lean: ask netcode-architect; SS has
   `big_vehicles_count` (tick_loop.rs:1173), so it matters. IPV6-WHEN (Low): "if LG reaches RS over IPv6": compute
   always, or "not available" unless params.lg_ipv6 is true (SERIES.md:454 param `lg_ipv6`, used by -25 only)?
   Lean: always (it is IPv4 counters + 20 B, a what-if). Also the IPv6 figure is the pooled mean while the check is the
   largest client mean: as SERIES.md:269 says; flag only. Cite fix (Low): SERIES.md:284 says "ADR 0001 lines
   1754-1779"; those are r2's lines (ADR 0001:1479 cites "worked bandwidth estimate, lines 1754-1779").

### A3. up_client_mean_max_bits (PM-0001-13)
1. Check "each client's mean at most 48 kbps (IPv4, UDP payload + 28 B)" `<=` 48000, parts ["P2"]; report "P4: each
   client's mean" parts ["P4 (report only)"]. Parts P2 (L2 SS P2 30 Hz clients 48 soak S30, min 3), P4 (P4, min 1), both
   pre:PM-0001-2; V-S; bite B-N. Notes (PM-0001-13): "The mean is the proposed reading of the row; per client it is the
   largest client mean (question)."
2. As down_client_mean_max_bits over second.s.up_bits (SERIES.md:298, :126 "bits received from it ... + 28 B"),
   measured seconds, room log; ADR 0001:1601. One shared helper `slot_mean_max(run, name)` for both.
3-4. Own run. Gaps as A2 (CV-GAP, CV-SRC lean). No Waiting/Empty.
5. Writer: NONE for up_bits (npt-server row; no literal in crates/). Made-up logs.
6. Tests: PASS max mean 48,000; FAIL 48,001 (144,003 over 3 s); GAP a second without up_bits; the P4 report prints the
   value on a P4 run. Plant: the helper reads down_bits for both (a test with different up/down lists catches it).
7. Open: none beyond A2's "largest client mean" reading (marks notes call it a question; the figure is named for it).

### A4. rss_slope_bytes_per_10min, queue_growth_count (PM-0001-15)
1. Checks "slope after warm-up within 1 MB per 10 minutes, high" `<=` 1000000 and "..., low" `>=` -1000000 (figure
   rss_slope_bytes_per_10min, no parts); "every per-client queue flat: max of the last 10 minutes at most the max of the
   first 10" queue_growth_count `==` 0. Part S30 at P2 (L2 SS P2 30 Hz clients 48 soak S30), min 3, pre:PM-0001-2; V-S;
   bite leak. Notes (PM-0001-15): "queue lengths (outbound, unacknowledged events) per client from the server's
   counters"; "MB = 10^6 B. The slope method (least squares over the measured seconds) is proposed; 'flat' is the row's
   proposed reading."
2. Slope: x = `sec`, y = second.v.rss_bytes, measured seconds, room log (SERIES.md:287, :93; ADR 0001:1603). Closed
   form in integers: n = count, Sx, Sy, Sxx, Sxy (i128); value = Ratio::new(600 x (n Sxy - Sx Sy), n Sxx - Sx^2).
   Bounds: n, x <= ~4,000, y <= ~1e9 -> num <= ~4e22, den = n Sxx - Sx^2 <= ~2e13; the check's cross-multiply
   (ratio.rs:136-138) stays near 1e23, far below i128 (1.7e38); state it in a comment, test a 2-hour 1 GB case.
   queue_growth_count: for each slot of second.s.out_queue_count and of second.s.event_queue_count: 1 if max over the
   last 600 measured seconds > max over the first 600; sum (SERIES.md:283).
3. Own run only.
4. Gaps: fewer than 2 measured seconds or all x equal (den 0); a measured second without rss_bytes / either queue list
   (CV-GAP); a duplicate `sec`; lists not holding `clients` entries or changing length; more than one room log. Lean:
   fewer than 1,200 measured seconds is a Gap for queue_growth (the two windows would overlap; S30's measured length
   [unverified]: if it is 30 min, 1,800). No Waiting, no Empty; 0 grown queues is a real 0.
5. Writers: rss_bytes (server/src/tick_loop.rs:1074) and out_queue_count (tick_loop.rs:1133-1139) ARE written by
   npt-server; event_queue_count (SERIES.md:415, npt-room) has NO writer. Made-up logs for the event queue.
6. Tests: PASS rss flat with noise -> slope 0 (exact); y = 1,000 x sec -> slope 600,000 (PASS high); y = 2,000 x sec
   -> 1,200,000 (FAIL high); y = -2,000 x sec -> -1,200,000 (FAIL low); one sec -> GAP. Queues: 1,200 s, slot 1's
   out_queue max 5 then 6 -> 1 (FAIL); equal maxima -> 0 (PASS); a second without event_queue_count -> GAP.
   Plants: slope sign flipped (Sx Sy - n Sxy) -> the low/high pair swaps; x = record index instead of `sec` (a test
   with a missing second in the middle, slope differs); queue windows by `>=` instead of `>`.
7. Open: QWIN (Medium): "last 10 measured minutes" by record count (first/last 600 measured second records) or by
   time (sec < first+600, sec > last-600)? Lean: by `sec` (time), robust to a missing record, which CV-GAP does not
   catch for seconds. MB = 10^6 (notes) matches the values 1000000 (no conflict).

### A5. session_memory_growth_count (PM-0001-24)
1. Check "memory per session flat over 10 minutes (PM-0001-15's rule, halves of the window)" `==` 0, no parts. Parts
   NewReno, Cubic (L2 P2 suite bottleneck, cc), min 1; mark V-S and clean-ref; bite blocking-send. Notes (PM-0001-24):
   "'Flat over 10 minutes' compares the second half's max with the first half's (the 10-minute run has no 'first and
   last 10 minutes': proposed)."
2. For each index of second.s.session_buffered_bytes, out_queue_count, event_queue_count: 1 if max over the second
   half of the measured seconds > max over the first half; sum (SERIES.md:290; :415-417). Room log.
3. Own run. 4. Gaps as A4 (fewer than 2 measured seconds; missing lists; length changes). Real 0.
5. Writers: out_queue_count yes (tick_loop.rs:1133); session_buffered_bytes (npt-server, SERIES.md:417) and
   event_queue_count NO. 6. Tests: PASS halves equal; FAIL session 0's buffered max 10 then 11 -> 1; GAP missing
   session_buffered_bytes on one second; odd count (5 s) split per HALF. Plant: halves swapped (fires on a fall, not
   a rise): the FAIL case passes. Shared helper with A4 (`growth_count(lists, split)`).
7. Open: HALF (Low): with an odd number of measured seconds, where does the middle second go? Lean: halves by time,
   mid = (first_sec + last_sec) / 2 exact, second half `sec > mid`. Index semantics: session_buffered_bytes is "one per
   session" (SERIES.md:417), the queues "one per client"; counted per (series, index) either way.

### A6. kill_p99_ratio (PM-0001-27)
1. Check "after SIGKILL of one room, the others' p99 within 5 % of their own before it" `<=` "1.05", parts ["D1
   surviving rooms", "D2 surviving rooms", "D3 surviving rooms"] (params.density 8-rooms, session D1-3, killed false,
   min 7); killed-room parts (killed true, min 1). Mark validity PM-0006-13; no bites. Notes (PM-0001-27): "Each room is
   its own run, the session its set"; "The kill comes after the 20 measured minutes; the survivors' ticks after it are
   phase `after`, and the figure compares their p99 with the measured p99 ('within 5 %' read as not worse by more than
   5 %: proposed, question). The killed room's log has `params.killed` true."
2. p99(tick.v.work_ns, ph = after) / p99(tick.v.work_ns, ph = measured), the run's room log, nearest rank via
   `stat_of(&Stat::Pct(99/100))` (eval.rs:276-293) (SERIES.md:278 "after the set's `kill` event (ph = after)";
   ADR 0001:1618). Exact: Ratio::new(p_after, p_measured).
3. Needs from the set: the killed room's run (params.killed true) as evidence the kill happened. `set` serves it.
4. Gaps: no room log; no measured or no after tick with work_ns (CV-GAP); p_measured = 0 (den 0); two killed runs in
   the set (lean). Waiting: no killed run in the set yet (CV-WAIT). No Empty; real values only.
5. Writers: work_ns yes (tick_loop.rs:1162); a `kill` event: NO writer and NO SERIES row (only the figure row names
   it); npt-super writes only room_start/room_exit (super/src/main.rs:305-311). The `after`-phase tick writer: the
   room phases exist (Phase::After), whether a survivor logs `after` ticks after a sibling's kill is U6's [unverified].
6. Tests: PASS measured p99 100, after p99 105 -> 21/20 PASS; FAIL after 106 -> 53/50; GAP survivor has no after tick;
   WAIT the set lacks the killed run -> NOT-RUN. Plant: the windows swapped (ratio inverted: the FAIL case passes);
   second: Window::All for the "before" p99 (the after ticks leak in: a case with a high after tail catches it).
7. Open: KILL-EVENT (Medium): SERIES.md:278 says "after the set's `kill` event", but no `kill` event is defined (no
   SERIES row, no writer), and the survivors' tick numbers are their own clocks. Options: (a) phase `after` alone plus
   the killed run present (lean); (b) a `kill` event (npt-super's, with the killed slot) required in the set, the same
   record C4 needs (below). C4-KILL (Medium, scope): README §2.5:202-203 "the exemption from `complete` holds only if
   npt-super's log records that kill of that slot (built at U6: C4)" and advisor-u1.md:124-126 "Fixture at U6": a
   waiting judge change in `always_rules` (eval.rs:576-584 exempts any `killed` log today) that the hand-off does not
   list. Lean: in part b with this figure, one record serving both (needs the record's name and fields from the
   netcode chat), or a named deferral. "Within 5 %" read one-sided (notes: proposed, question): the check's `<=` 1.05
   already encodes it; nothing for the judge to choose.

### A7. bulk_control_p99_rise_ns, one_shot_dropped_count (PM-0001-38)
1. Checks "control stream p99 message delay with a transfer minus without at most 5 ms" bulk_control_p99_rise_ns `<=`
   5000000; "0 dropped (sent by LG's clients minus received by the room)" one_shot_dropped_count `==` 0; no parts.
   Part L2 P3 LG (suite one-shots, machine.role LG), min 1; no validity; bite bulk-on-control. Notes (PM-0001-38): "The
   control stream's message delays are written apart for the seconds with and without a transfer
   (`second.s.control_delay_bulk_ns`, `second.s.control_delay_idle_ns`); the figure is the difference of their p99s."
2. Rise = p99(flatten second.s.control_delay_bulk_ns) - p99(flatten ...idle_ns), measured seconds, the LG (process
   bots) logs pooled (SERIES.md:263, :412-413; ADR 0001:1630). Integer, may be negative (passes).
   Dropped = Sum second.v.one_shots_sent_count over LG logs - Sum second.v.one_shots_received_count over the room log,
   EVERY phase ("over the run", SERIES.md:282; :82, :410). Integer; negative (more received than sent) fails `== 0`.
3. Own run. 4. Gaps: no LG log / no room log; an empty bulk or idle list over the window (no transfer measured: a
   Gap, not 0; CV-GAP); a second record without one_shots_* (CV-GAP); the names in another process's log (CV-SRC,
   lean). No Waiting; no Empty (lean: a missing bulk list is a gap, the "10 transfers" check counts transfers).
5. Writers: NONE for control_delay_* (npt-bots), one_shots_sent_count (npt-bots), one_shots_received_count (npt-room).
6. Tests: PASS bulk p99 7 ms, idle 2 ms -> 5000000; FAIL 5000001; GAP no bulk sample; dropped PASS 10,000 - 10,000;
   FAIL 10,000 - 9,999 -> 1; one sent in warm-up and received in measured -> 0 (every phase); received in `after`
   counted. Plants: window measured-only for dropped (the warm-up case fails); p50 for p99.
7. Open: BULK-WIN (Low): SERIES.md:263 names no window: lean measured. ONE-SHOT-TAIL (Low): one-shots in flight at the
   stop: the room must log until they land (as README §1:81-82 asks of hashes); nothing for the judge.

### A8. crash_close_max_ns (PM-0001-59)
1. Check "every client gets RoomEnded(crash) or a close within that second (0 left open)" `<=` 1000000000, no parts.
   Part "10 S30 rooms, one tick panic each" (L2 SS suite tick-panic), min 10; bite no-catch. Notes (PM-0001-59): "The
   figure joins each LG client's close (on its server-clock estimate) with the room's panic tick."; "Each room's log
   is cut by its own exit (`params.killed` true exempts it from the `complete` rule, or the catch writes the footer
   with `ended` = crash)."
2. max over the run's clients of (event.room_end.server_est_ns, LG bots log) - event.tick_panic.server_ns (room log),
   any phase (tick_panic is written in phase after: tick_loop.rs:784) (SERIES.md:264, :419, :423; ADR 0001:1617).
   Integer ns; "a client with no room_end fails".
3. Own run. 4. Gaps: not exactly one tick_panic in the room log; a room_end without an integer server_est_ns (eval.rs
   style) or without a client id; no LG log. FAIL with no number: a client of the room's `clients` with no room_end
   (needs `Lack::Fail`, §0.1). No Waiting, no Empty.
5. Writers: tick_panic.server_ns YES (tick_loop.rs:784, `clock::now_ns()`); room_end: NO writer (npt-bots, SERIES.md:419).
6. Tests: PASS 48 clients, slowest 999,999,999 ns after; FAIL 1,000,000,001; FAIL-no-number slot 7 has no room_end;
   GAP two tick_panic events; a negative difference (estimate early) counts as is. Plant: min for max; the missing-
   client rule dropped (the no-room_end case passes).
7. Open: CLIENT-ID (Medium): SERIES.md:419 gives room_end no client field, so "each client" cannot be counted; lean a
   `slot` field (as admit/session_end carry: SERIES.md:355; lab/src/scenario.rs:1974-1976), each slot of the room
   header's `clients` once, a duplicate a Gap. CLOCKS (Low): server_ns is "the room's clock" (tick_loop.rs:784 reads
   `clock::now_ns()`), room_end is LG's estimate of it: the difference is as good as the estimate; reported as is.

### A9. egress_interface_bytes, egress_metric_vs_interface_ratio, r2_egress_formula_bytes, egress_vs_r2_ratio (PM-0006-17)
1. Checks (parts ["one room-hour, 48 bots all in view"]: suite egress, clients 48, interest off, min 1):
   egress_vs_r2_ratio `>=` "0.8" and `<=` "1.2"; egress_metric_vs_interface_ratio `>=` "0.95" and `<=` "1.05". Reports:
   r2_egress_formula_bytes and egress_interface_bytes (no parts: both parts), egress_interface_bytes again for "interest
   on (reported)". No validity; bite double-send. Notes (PM-0006-17): "The interface's transmitted-byte counter is read
   at the start and end (`event.iface.tx_bytes`); the room's metric at the same moments (`event.room_metric.sent_bytes`).
   R2's formula (r2 lines 1764-1766) with the run's P, V, shots and projectile spawns; at R2's own counts it is 3.156 GB
   (band 2.52 to 3.79 GB)."
2. egress_interface_bytes = last iface.tx_bytes - first (SERIES.md:270, :418); metric ratio = (last - first
   room_metric.sent_bytes) / egress_interface_bytes (:271, :420); r2_egress_formula_bytes = r2_down_formula_bits x
   clients x 3,600 / 8 (:285); egress_vs_r2_ratio = interface / formula (:272). ADR 0006:1387. Golden: A2's counts at
   48 clients -> 3653064/25 x 48 x 450 = 3,156,247,296 B (R2 prints 3.156 GB).
3. Own run (room log); no SIG need.
4. Gaps: not exactly two iface (or room_metric) events in the room log (lean; start and end by record order, README
   §1:9-10 "in time order"); a non-integer field (collect's rule, eval.rs:219-226); interface bytes <= 0 (den 0 or a
   counter reset); the formula's inputs as A2. No Waiting, no Empty.
5. Writers: NONE for iface.tx_bytes, room_metric.sent_bytes (npt-server rows :418, :420; `sent_bytes` literals in
   crates/ are other records); the formula's inputs as A2.
6. Tests: PASS tx 0 -> 3,156,247,296 with A2's golden counts -> egress_vs_r2 1 (PASS both); FAIL double (2.0);
   boundary 0.8 exactly PASS; metric 1.05 exactly PASS, 1.050001 FAIL; GAP three iface events; GAP tx end < start.
   Plant: x 3,600 dropped; second plant: start/end swapped (negative bytes -> must be a Gap, never a ratio).
7. Open: EGRESS-SPAN (Medium): x 3,600 s fixed (SERIES.md:285) while the counter covers the real span between reads
   (three 20-minute matches with joins): lean as written, and print the span beside the arithmetic (ARITH).
   EGRESS-INPUTS: ADR 0006:1387 names "P, V, shots and projectile spawns" (no reliable events) while SERIES.md:285 says
   "as r2_down_formula_bits" (events included) and -12's notes say "as PM-0006-17 does": lean the -12 form (events at the
   run's count), flagged to the owner. Window of the counts: lean the measured window (the egress run's whole hour
   measured) [unverified: the egress run's phases are U6's/U8's]. R2-INPUTS (A2) applies here too.

### A10. logging_cost_ratio (PM-0006-19)
1. Check "pooled p99 of the 3 'on' runs at most 1.02 x the pooled p99 of the 3 'off' runs (54,000 ticks each)" `<=`
   "1.02", parts ["logging on (3 runs)"] (suite logging-cost, min 3). Report "pooled p99, logging on" (series, not
   ours). Validity PM-0006-19 (Rule::LoggingOff, eval.rs:918-957); bite busy-50us. Notes (PM-0006-19): "The 'off' runs
   have role `logging-off` in the same set. The pooled comparison is a figure: `against` combines per-run statistics,
   not pooled ones (question)."
2. p99(tick.v.work_ns over the measured ticks of the part's valid runs in this set, pooled) / p99(the same over the
   set's role logging-off runs, pooled), nearest rank (SERIES.md:279; ADR 0006:1389). Exact Ratio::new(on, off).
3. Needs "the part's 3 runs" (SIG) and the set's logging-off runs (`set`, as Rule::LoggingOff reads them, eval.rs:920-
   924). Every on run computes the same value (3 identical check results): fine, say so in the report text.
4. Gaps: an off run breaking `always_rules` (CV-REF; Rule::LoggingOff does not check it today, eval.rs:920-948); no
   work_ns in a run (CV-GAP); off p99 = 0. Waiting: fewer than 3 logging-off runs in the set (CV-WAIT); fewer than 3
   valid on runs (SIG, or the part's min makes it NOT-RUN anyway). Lean Gap for more than 3 off runs (which 3?).
5. Writers: work_ns YES (tick_loop.rs:1162); role `logging-off` and suite logging-cost: NO writer yet (run scripts).
6. Tests: PASS on pool p99 102, off 100 -> 51/50; FAIL 103; WAIT 2 off runs; GAP an off run with no footer; pooled vs
   per-run: a case where the per-run mean of p99s passes but the pooled p99 fails (proves pooling). Plant: pool only
   this run's ticks (the pooled case flips); second: off runs taken by `!= measured` role (a `clean` run leaks in).
7. Open: ON-POOL-SET (Low): the part's members may span sets; lean pool only those in this run's set (one session of
   six). TICKS-54000 (Low): "(54,000 ticks each)" is in the check's name only; lean report the pooled counts, judge
   nothing. Rule::LoggingOff's own CV-REF gap (above) is a pre-existing Medium in eval.rs, part b's to fix or defer.

### A11. alloc_twin_diff_count (PM-0006-20)
1. Check "on every tick after the warm-up, the tick thread's allocation count with logging on equals the count with
   it off (tick by tick)" `==` 0, no parts. Part "logging on" (suite logging-alloc, min 1); no validity; bite
   planted-format. Notes (PM-0006-20): "The 'off' run has role `logging-off` and is named in the 'on' run's
   `params.twin`; the figure compares the two runs' `tick_allocs_count` at equal tick numbers."
2. Count of tick numbers in the measured window of either run where tick.v.tick_allocs_count differs between the run
   and its twin, a measured tick in one only counting (as twin_hash_mismatch_count, figures.rs:205-213) (SERIES.md:262,
   :55; ADR 0006:1390). Integer.
3. The twin from `set` via `twin_of` (figures.rs:99-149); role must be `logging-off` (not `twin`). For the bite run:
   C-U6-10 (B2).
4. Gaps: as twin_of (no twin named, itself, invalid twin, other profile/seed/bite); twin role not logging-off; a
   measured tick without tick_allocs_count (CV-GAP). Waiting: twin not in set (CV-WAIT). Empty: no measured tick in
   either run (CV-EMPTY). Real 0 otherwise.
5. Writer: NONE for tick_allocs_count (SERIES.md:55; U6c S5 "PM-0006-20's allocator twin", u6-brief.md:34). Made-up logs.
6. Tests: PASS equal counts; FAIL tick 3 differs -> 1; FAIL a measured tick in one only -> 1; WAIT twin missing;
   GAP twin role `twin`; EMPTY no measured ticks. Plant: compare at record index instead of tick number (a twin with
   one extra warm-up record catches it); second: drop the one-sided ticks.
7. Open: ONE-SIDED (Low): a measured tick in one run only counts (lean, as twin hash) vs is a Gap; the replay drives
   both, so tick numbers align [unverified until U6c's writer].

## B. The three waiting judge changes

### B1. C-U6-9: -30's twin form in twin_hash_mismatch_count (plan.md:1254; u6-brief.md:57-58)
- Today: `twin_hash_mismatch_count` takes a twin of role `twin` only (figures.rs:196-203); -30's six runs are all
  measured and each names its other-transport run (marks.json -30 notes; check "identical server state hashes on every
  tick (each run against its other-transport twin)"; parts RS and C, suite both-transports, P0, min 6, late-zero), so
  every -30 run is INVALID today ("has role measured, not twin"). twin_of's own rules stay (named, not itself, in set or
  Waiting, always_rules, same profile/seed/bite).
- Rule to implement: accept the twin if role `twin` (as now) OR all of: run and twin both role `measured`; the twin's
  `params.twin` names the run (mutual); `params.transport` present in both, each `wt` or `ws`, and different. Refuse
  (Gap) every other measured twin: same or missing transport (keeps -21's `twin-is-measured` fixture INVALID, its logs
  carry no transport: fixtures/pm-0001-21/twin-is-measured/twin-lab.jsonl:1), one-way naming, `both` (SERIES.md:903
  allows `both` for a run's clients; SERIES.md:446's param table says "wt or ws": flag PARAM-TRANSPORT, Low). The bite
  `wallclock` keeps the role-twin form (C4: only bite or twin may name a bite, eval.rs:552-565). Lean: data-defined (no
  mark id in figures.rs), since -21's runs never carry a transport.
- Tests (fixtures, real marks.json; base gains the C part's 6 measured runs, 3 recordings x wt/ws, each a room log of 3
  ticks with `h` and late_count 0): base -> C part PASS (mark NOT-RUN: RS 0 of 6, wallclock no run); `twin-differs`
  (one tick's h) -> FAIL "= 1, needs == 0"; `same-transport` (a pair both ws) -> INVALID; `not-mutual` -> INVALID;
  `twin-missing` -> NOT-RUN; -21's `twin-is-measured` still INVALID (existing test). Engine: a measured pair with
  transports differing and role `twin` on one side -> today's role-twin path. Plant: the transport inequality check
  removed (same-transport passes); second: the mutual check removed (not-mutual passes).
- Texts: SERIES.md:297 row (both forms, the mutual and transport words); the doc comment figures.rs:191-195 ("-30's
  form comes with U6"); fixtures/README.md rows for the four cases. ~40 code, ~60 tests, ~60 fixture lines, ~10 text.

### B2. C-U6-10: PM-0006-20's logging-off twin vs C4's role list (plan.md:1254)
- Conflict, verified: `always_rules` refuses a run that names a bite unless its role is `bite` or `twin`
  (eval.rs:552-559); `twin_of` calls `always_rules(twin)` (figures.rs:122-129) and, when `bite` is a field, demands the
  same bite (figures.rs:139). The planted-format bite run (role bite) needs an off twin: role `logging-off` naming the
  same bite -> always_rules refuses it ("a logging-off run names the bite ...") -> Gap -> the bite can never be shown.
- Options: (A, lean) the off twin of a bite run comes from the same bites build, names the same bite, role
  `logging-off`; `twin_of` accepts it when the run is role `bite` and the twin's only broken rule is the role-list rule
  (the twin is named by a bite run of its own set with the same bite); C4 stays as is for every other run. (B) the off
  twin is a measured-build run (bite null): `alloc_twin_diff_count` calls `twin_of` without "bite"; the two binaries
  differ, so a diff may come from the build, not the bite (weak bite evidence). (C) add `logging-off` to C4's list
  everywhere: loosens C4 for PM-0006-19's off pool too (Rule::LoggingOff pools any logging-off run, eval.rs:920-924):
  reject. Owner's ruling: TWIN-ROLE.
- What alloc_twin_diff_count must do for the bite: count measured tick numbers whose tick_allocs_count differs (A11);
  the planted `format!` allocates on every logged tick, so the count is >= 1 and `== 0` fails: shown. Tests: bite run
  + same-bite logging-off twin -> bite Shown; the same twin with role `measured` -> Gap (bite Invalid); a measured on run
  whose off twin names a bite -> Gap (C4 still bites outside the bite pair); PM-0006-19's off pool unaffected (a
  logging-off run naming a bite in a -19 set still breaks always_rules). Plant: the role-list exemption widened to any
  run (the measured-on case passes wrongly). ~30 code, ~70 tests; texts: SERIES.md:262 (the bite's twin), README §4's
  build rule sentence (the bite pair exception).

### B3. C-U6-11: -19's join hash under the coverage rule (plan.md:1254; replan-units.md:40-41; u6-facts.md:345)
- Today: `coverage` keeps ONE span per slot, first to last resolved tick (figures.rs:483-486), (c) every room-logged
  hash inside it resolved (:487-499), (d) the span covers the room's first..last MEASURED hash of the slot (:500-534).
  README §1:79-81: "every client is admitted, and its first snapshot resolved, in the warm-up (a client admitted later
  is -19's join, built with U6)". -19's kept part "LG joins" (L2 P2 suite joins, LG, min 1) has 100 joins into a
  running S30 room, bots leaving and rejoining (replan-units.md:43-44, C-U6-14); a slot reused across sessions has
  room hashes between a leave and the room processing it that no client resolves -> (c) Gap; a client admitted
  mid-measured-phase starts after the room's first measured hash for its slot under an earlier session -> (d) Gap.
- Rule (lean; Q10 is "a marks/README.md rule, chosen in U6j's brief with test-engineer's read", u6-brief.md:183-184):
  spans per SESSION, cut at the room log's `admit` events (fields slot, session, `t`: SERIES.md:355; lab/src/logs.rs:
  304-309). A session's span runs from the client's first resolved tick at or after its admit `t` to its last resolved
  tick before the slot's next admit. Head: the room's first non-null hash of the session is at or after that first
  resolved tick (the full state's tick). Inside: (c) per session. Tail of a session that ends before the run does: room
  hashes after the client's last resolved tick are in flight, allowed only up to the slot's `session_end` (or next
  admit) and only if the room log records that end (else a Gap: a client that silently stopped resolving must not pass);
  the last session keeps today's (d). A room hash after a recorded session end is a Gap.
- Browser writer: NO. -19's "ML Chrome" part is moved ("Slice 3, with the human run", marks.json) and
  replan-units.md:40-41 says "its ML-browser `decoded_hash` writer MOVES with -19's R7/ML parts (the plain client has
  no hash)"; `source` stays client|bots (figures.rs:227-229). What must land first (plan.md:1254): u4s-carries entry
  64 (a)'s LG-key case; the row ties it to the browser joining, which moved, but entry 64's own text "at the latest
  before U6 builds -19's figure" still puts all of 64 before this block (C below).
- Tests (engine, made-up logs): a slot with two sessions, leave in flight (room hashes ticks 10-11 after the client's
  last resolved 9, session_end at 11, re-admit at 20) -> 0 mismatches PASS; the same without session_end -> INVALID;
  a mid-measured admit (t 50) whose first resolved tick is 50 -> PASS; first resolved 52 while the room hashed 50 ->
  INVALID; a room hash after session_end -> INVALID; a warm-up-only run unchanged (existing -17 fixtures all keep their
  verdicts). Plant: spans per slot again (the two-session case goes INVALID); second: the session_end requirement
  dropped (the silent-stop case passes). ~90 code, ~120 tests.
- Texts: README §1:79-82 (the warm-up sentence: "every client is admitted in the warm-up except -19's joins: a client
  admitted later starts its span at its first resolved tick at or after its `admit` ..."); SERIES.md:267's coverage
  clause ("per slot" -> "per session"). Writers needed on L2 (U6): `admit` with slot and t and `session_end` with slot
  and t in the ROOM log (today session_end is the L1 lab log's: lab/src/scenario.rs:1971-1979): flag SESSION-END-LOG.
