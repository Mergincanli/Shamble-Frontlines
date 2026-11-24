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
