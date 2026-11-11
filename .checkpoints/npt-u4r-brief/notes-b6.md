B6 notes: §5 of U4c (:552-583), U4d (:585-624, read for World/Scene/Recording), U4j (:626-645), U4d2 (:647-683), U4e (:685-716), at df8975b. What U4r uses.

U4c (npt-l1) :552-583
- :553-554 C1' event-driven; internal events in (time, fate index) order when a call's now passes them; `L1Net::next_event_ns`.
- :555-558 RFC 9002 starting state (T12): before first sample srtt 333 ms, var 166.5 ms; first PTO 1,024 ms; first sample sets both; time threshold.
- :558-560 acks immediate, delayed by other direction's draw, lost inside bursts only; acks draw own per-flow stream (T23).
- :562-563 fate log: stream and ack entries; datagram entries 40 B; tag `npt-l1 fate log v2`.
- :571 L-c6 at one instant, internal events before application sends (the RTT argument :754-755 leans on it).
- bites per instance: L1Net::with_bite (l1/src/lib.rs:856-868 cited at §4.11 :387-388).
- -> U4r uses: L1Net (the L1 link between lab's clients and the room), next_event_ns (SimClock advance), rtt_ns per link, the fate log (twin test's fate-log hashes :393), the 333 ms starting RTT.

U4d (npt-sim) :585-624
- :586 World: SimCore, fixed pools; Scene (SS, ES, CO, TC: §4.13).
- :589-591 SimRng, event list, canonical hash FNV-1a-64, StateDump, Recording, replay::Replayer, synthetic generator (test support).
- :607-610 D-6 SS counters assert PM-0001-2 thresholds exactly (cond 5).
- -> U4r uses: World + Scene (all runs), Recording (scenario --record writes one), Replayer (U4d2's hash runs the recordings).

U4j (npt-judge, npt-log, marks/) :626-645
- :627 npt-log `hs` and its schema test.
- :627-630 npt-judge: C4 with fixtures; figures decoded_hash_mismatch_count (§4.6 rules), decoded_hash_compared_count, twin_hash_mismatch_count; cross_arch_mismatch_count and machine_named fixtures only.
- :630-631 No PRE- id, no marks/prechecks.json, no "kind": "precheck" rule: -30's pre-check a recorded npt-lab test the judge never reads (cond 2).
- :632 marks.json: -18's split, `burst_heal` and `burst_return`, the `ws-practice` rule (Q20); §4.12's table (Q22).
- :633-635 README: `loopback` layer value in §1 (pre-check's Loopback leg writes an npt-log header; comparer reads both legs' room logs), no judge change; `hs` with `null` for a client with no hash at a tick (nit c).
- :636-637 README §4 -22 exception.
- :638-640 README and SERIES fix the run-structured bound line format and the WS practice record (Q20; nit f); SERIES.md rows: snap_incomplete, burst_eviction, clamp events, the precheck suite; figure rows' formulas.
- -> U4r uses: the log schema (hs, admit events, footers, late_count), the figures, marks.json's -17/-18/-57/-58 checks and selects (§4.12 table), SERIES rows (contract test: "every written name has its SERIES.md row" :400), the `loopback` layer value, `npt-judge --only`.

U4d2 :647-683
- :648-654 crates/hash-wasm; npt-lab's `hash` subcommand (release build) runs two native legs in two processes + `HashCompare` prints the first mismatching tick and the field diff; "HashCompare takes two per-tick hash lists, so U4r's -30 pre-check reuses it on the two legs' per-tick server hashes (cond 2)".
- :655-657 check.mjs `wasm` step: release build, Node run, comparison; wall time printed, budget 5 minutes.
- :675 tests: 0 mismatching ticks over the 27,000 synthetic ticks, native twice and Wasm once.
- Lanes :832-833: U4d2 owns check.mjs, Cargo.toml's members, Cargo.lock, crates/hash-wasm, npt-lab's `hash`.
- -> U4r uses: HashCompare (pre-check comparer), `hash` subcommand (evidence :769 "U4d2's step green on them" = the recordings).
- finding: Evidence :769-770 says the runner recordings get U4d2's step green on them "in check.mjs if the step stays within 5 minutes, else through ops/runs/det-u4.mjs". Either route edits a file outside the night plan's U4r lane (check.mjs; or det-u4.mjs, which is not c-u4r-*.mjs), and §4.15 :485-487 has them regenerated at check time from a committed recordings manifest (who writes the manifest; does regeneration mean an L1 scenario run at check time?). -> advisor question.

U4e :685-716
- :687 Room<S>; Roster + slot token compare (lab passes tokens in; minting U6's); ClientSlot; :688-689 AckRing, Encoder (window remainder, groups, layout table, class filter, mirror hash §4.6); :689 EventQueue; InterestGrid, AreaFlood, Priority; :690 InputBuffer option B with buffer_depth_ticks; :690-691 rewind_at integer half (tolerance and cap U4h); room-side checks -58's counters pass through; Resync one in flight.
- :693-695 Decoder::pump (decode, socket checks, ping sender and answers with hold time, per-client RTT and bytes sent: §4.5, §4.9), ConnLimits, Counters, rate buckets, Join deadline on injected now.
- :696-697 npt-client: decoding into integer frames and held floats, complete-tick rule + sentinel, acks with group mask, minimal ClientEndpoint (input, step, next due time in ns).
- :707-710 -18 end to end; :711-712 bites (no-fallback, tick16, client-rounding, lt-ack, lt-tick-offset, lt-event-seq, owner-to-all, no-trailing, no-channel). :713 marks moved none judged (U4r runs them).
- -> U4r uses: Room::with_bite, Roster (lab passes tokens), Decoder::pump (per-client RTT used by the pre-check's first-sample log), ClientEndpoint (its next due time drives SimClock), Encoder's counters (burst_eviction, snap_incomplete, mirror hash), InputBuffer's buffer_depth_ticks (pre-check records it), rewind_at (pre-check's Rewinds).
