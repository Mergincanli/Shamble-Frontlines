B8 notes: u4s-carries.md at df8975b (241 lines; 44 entries + 45).

- §U4r :176-182: only entry 40 routed to U4r. Word for word (lines 177-182):
40. **O1 (u4-conditions-check.md:42-45; advisor-u4.md:343-347; the advisor's U4s ruling, O1):** "U4r's brief, which
    comes to the advisor, states that at U4r the room has neither the view-time tolerance nor the cap (both land in
    U4h: u4-build-plan.md §5 U4e's `rewind_at` and §5 U4h), so the -30 pre-check's setup condition (0 view-time
    replacements and 0 clipped shots on both legs: the advisor's condition 3) holds by construction at U4r, and its
    RTT argument cannot be tested there; and it either re-runs the pre-check pair after U4h merges (6 runs) or says
    why not (the advisor's lean is the re-run)."
  (Carry 40 cites by section, "§5 U4e's `rewind_at` and §5 U4h", not by line: at df8975b these are :690-691 and :799-800. Its O1 source cite u4-conditions-check.md:42-45 is right at df8975b; advisor's own O1 line cites :39-42, stale.)
- Header :9-10: "Each entry: the finding (review, id), then **the text the brief must carry**, word for word. A brief may reword the plain parts, never the rule."
- Carries routed elsewhere whose words reach U4r's files (U4r's crates are crates/lab and crates/bots; ops/runs):
  - carry 6 (U4b, :72-75) names "npt-server's and npt-lab's callers build `Schedule::new(npt_sim::TICK_HZ, npt_sim::LOOP_CATCHUP)`; the 60 Hz tests pass `npt_sim::loop_catchup_at(60)`, never a literal 15; a test asserts the server's schedule's catch-up cap equals `npt_sim::LOOP_CATCHUP`." -> npt-lab's L1Scenario's caller is U4r's code; U4b (main session) could only meet npt-server's half if npt-lab had no caller then. Brief: apply word for word; report its npt-lab half.
  - carry 27's second sentence (U4e, :161-163): "every other `*_TICKS` or `*CATCHUP*` const or static under crates/ but crates/net/ is never set from a literal, tests included." -> crates/lab and crates/bots are under crates/.
  - carry 39/35's rule (U4e/U4b): "A tick count needed outside npt-sim goes into npt-sim as `<name>_at` plus its constant (carry 18's route), and the crate uses it by path." F2 ruling (6) (advisor :463-465) lets U4f-U4h's briefs give their unit carry 18's places; U4r not named -> question.
  - carry 14 (U4d, :114-117): "The recording header carries the ring's span in ms beside `TICK_HZ` (nit b's capacity), and the replayer rebuilds the ring with `HitHistory::for_ms(span_ms)`." -> U4r's scenario --record writes runner recordings with U4d's Recording: the header must carry the span the room used. Recheck row.
  - carry 20 (U4d, :133-134): "`grid::Config` has a bites-only field, so its shape differs by build: never hash, record or compare it across builds; pass it by value." -> replay's floored-grid bite and the recordings.
  - carry 21 (U4e): "-30's comparer relies on it" (canonical order before step). Recheck row.
  - carry 28 (U4h, :185-186): span = U4h's test cap + margin; header carries span. At U4r the room's ring span is not yet U4h's: what span does the room use at U4r? (U4e's choice) -> recheck row.
- B8 done. Next: B9 (marks.json, catalogue).
