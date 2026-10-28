Task: wave 1 step 4 ("Fix") for R1 (simulation core): apply verifier + rust-reviewer results to docs/research/r1-simulation-core.md (engine-architect).

## Progress log
- 2026-10-28: started fresh (no prior notes). Next: read plan condition 8, topic file, verifier results, rust-reviewer report.
- Read: plan (fix rule cond. 8), rust-reviewer result, all six verifier reports (JSON). Summary of what must change:
  wrong: DET-F25b (per process -> per instance; REC5 mark, REC6 reason). unsupported: BND-F2b (0.2.122 breaking; year), BND-F15b (copy w/o transfer list; REC3), BND-F23c (allow-popups no isolation; REC5), ABL C1 (REC1 "only tooltips"), ABL C2 (REC2 hot path reason).
  Next: read the topic file structure.
- Read the whole topic file (2967 lines). Plan of fixes drafted (see "Plan" below). Next: check scratchpad for verifier downloads, fetch re-sources.

## Plan (decided)
- Wrong/unsupported -> action: DET-F25b -> re-source as new DET-F33 (per instance); BND-F15b -> re-source new BND-F34 (MDN structured clone "copies");
  BND-F23c -> re-source or drop clause (REC5 chain rests on F20 + F23a); BND-F2b -> drop 0.2.122 (no rec rests on it), OQ7 "three times" -> "twice";
  ABL C1 -> reword F5 + REC1 to the confirmed quote; ABL C2 -> REC2 reason re-based on F17's verbatim latency context.
- New supporting sources: BND-F35 (JS API 5.3 detach key, verifier finding 1); ECS-F35 (derive Clone / default clone_from; RigidBodySet clone_from default) for REC1 mark 2 fix; DET-F34 (Pcg32 quote) for REC4 fallback.
- Rust-reviewer missed risks 2-9 -> new open questions R1-RR-OQ1..n (no new recs, rule 6); in-place where they correct a statement (ECS-REC1 mark 2, BND-F19 panic=abort, DET-F14 clamp panics, DET-F21 domain sentence, DET-F12 wasm NaN).
- Tiers: Fiedler pages -> other (DET-F31 "quotes shipped-game developers", DET-F32); source table + counts; XU-OQ5, XU-OQ6 settled.
