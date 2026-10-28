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
- Re-sources fetched 2026-10-28 (exact words):
  * HashMap (doc.rust-lang.org std 1.99.1, "Usage in const and static"): "As explained above, `HashMap` is randomly seeded: each `HashMap` instance uses a different seed, which means that `HashMap::new` normally cannot be used in a `const` or `static` initializer." -> DET-F33
  * MDN Structured clone algorithm (last modified 2025-06-29): "The structured clone algorithm copies complex JavaScript objects. It is used internally when invoking structuredClone(), to transfer data between Workers via postMessage(), storing objects with IndexedDB, or copying objects for other APIs." -> BND-F34
  * WebAssembly JS API (Editor's Draft 2026-10-27) §5.3 Memories, "create a fixed length memory buffer" step 5: "Set buffer.[[ArrayBufferDetachKey]] to "WebAssembly.Memory"." -> BND-F35
  * MDN COOP (2026-07-12): same-origin-allow-popups "is used to relax the `same-origin` restriction for integrations where a document needs the benefits of cross-origin isolation but also needs to open and retain a reference to trusted cross-origin documents. For example, when using a cross-origin service for OAuth or payments."; "To use these features in a document, you will need to set the COOP header to `same-origin` and the `Cross-Origin-Embedder-Policy` header to `require-corp` (or `credentialless`)." -> F23c cannot be sourced; the two statements pull apart -> new OQ BND-OQ9; REC5 rests on F20 + F23a.
  * std Clone 1.99.1: "The `derive`d implementation of `Clone` calls `clone` on each field."; "`a.clone_from(&b)` is equivalent to `a = b.clone()` in functionality, but can be overridden to reuse the resources of `a` to avoid unnecessary allocations." -> ECS-F35
  * docs.rs rapier3d 0.36.1 RigidBodySet impl Clone: clone_from "Performs copy-assignment from `source`." with source link to core's clone.rs (provided default, not overridden) -> ECS-F35 (second part)
  * docs.rs rand_pcg 0.10.2: "This is a good choice on both 32-bit and 64-bit CPUs (for 32-bit output)."; Lcg64Xsh32 `pub fn new(state: u64, stream: u64) -> Self` "Construct an instance compatible with PCG seed and stream."; advance "Multi-step advance functions (jump-ahead, jump-back)" -> DET-F34
  * threads Overview.md "Shared Linear Memory": "It is a validation error to attempt to import shared linear memory if the module's memory import" "doesn't specify that it allows shared memory." -> BND-F17 quote fixed (same source; nit)
- GAS README local copy (verifier's download): §4.5.5 l.1050 "Stacks also have policies for expiration, duration refresh, and period reset. They have helpful hover tooltips in the `GameplayEffect` Blueprint."; §11.1.1 Q7 answer: "The short answer there is not a way to prevent this and Paragon definitely had the problem. Higher latency connections would have a lower ROF with basic attacks."; l.914 Override: "Any `Override` `Modifiers` will override the final value with the last applied `Modifier` taking precedence."; l.2555 "*Dave Ratti from Epic's comment from the new Network Prediction Plugin*"; l.2942 "the prediction key always starts at zero each time the game starts"; "the client could easily hack their game to only send the same seed value every time."
  Next: apply edits batch by batch (findings ECS -> TICK -> DET -> BND -> ABL -> DATA), then options, recs, OQs, part 7, header.
- Edit method: python heredocs (no files written besides the topic file), whitespace-flexible exact-match replacements asserting one match each. Part 7 table built by parsing the six verifier reports (JSON) and the rust-reviewer full table (notes l.161-262).
- Starting batch 1 (ECS findings).
- Batch 1 done (ECS findings F15,F16,F19,F20,F28,F29,F30,F31,F33 + new F35; options snapshot cell). Batch 2 done (TICK F1,F3,F12,F16,F18 locations/labels; GN10 legacy bullet; options 4a; REC2 text/Rests on/P3 note; REC3, REC4 Serves; REC5 XU-OQ3 note). Reflow long lines at the end. Next: batch 3 determinism.
- Batch 3 done (DET F5,F8,F9,F11,F12,F14,F21,F23,F25,F30,F31,F32 + new F33,F34; options Q4,Q5; REC1,REC2 Rests on; REC4 Pcg32+F34; REC5 reason/mark/Rests on; REC6 reason/Rests on/Serves). Next: batch 4 boundary.
- Batch 4 done (BND F1,F2,F8,F9,F15,F17,F19,F23,F24,F27 + new F34,F35; REC1 Rests on/Serves; REC2 reason/Rests on; REC3 Rests on (F34); REC5 reason/login chain; REC6 Serves; OQ7 count; new OQ9). Next: batch 5 abilities.
- Batch 5 done (ABL F2,F3,F4,F5,F14,F15,F16,F17,F18,F23,F24,F25; REC1 reason C1 + marking inference + mark 3; REC2 reason C2 + Rests on F21; REC3 Rests on F18/GC5; REC4 removal gap + F17 failure + Rests on; REC5 Rests on legacy; REC6 Rests on brief+VISION §3, Serves). Next: batch 6 data.
- Batch 6 done (DATA F5,F12,F15; REC1,REC2,REC7 Rests on; REC5 F21 inference + partial trace -> OQ12; REC6 secure-context caveat; OQ11 note; new OQ12). Next: batch 7 ECS recs, ECS-OQ2, DET-OQ2/5/7, XU-OQ5/6 settled, RR-OQ1..7, hinge rows.
- Batch 7 done (ECS-REC1 reason/mark 2/Rests on; REC2, REC3 Rests on; REC3 GC15 gloss; REC4 serde-serialize; ECS-OQ2 settled; DET-OQ2/5/7 notes; XU-OQ5/6 settled; new section R1-RR-OQ1..7; 2 hinge rows). Next: batch 8 header, TL;DR, sources.
- Batch 8 done (header status/date/ids/totals, reading rules, TL;DR, sources tiers rows 30/34/82, per-unit note, "Sources read at the fix step" table). Reflowed 57 edited blocks to 120 cols. Next: build part 7 (merged table + counts).
- Parsed: verifier table rows ecs 29, tick 36 (verifier counts 37), det 41, bnd 46, abl 40, data 32 (verifier counts 33); stated totals 226 = 219 C, 1 W, 4 U, 1 P, 1 not checkable. rust-reviewer R1 rows 101 = 99 C, 1 W (DET-F25), 1 P (BND-F2). Next: generate and insert part 7.
- Part 7 inserted: merged table 243 rows (224 verifier rows + 15 rust-only rows + 4 gap rows), counts, "other findings" and "not applied" lists. Temp scratch file removed. Next: consistency checks (ids referenced exist; RR-OQ refs; DET-REC2 F5 mention; part 7 counts), then change log by type, DONE.
- Structure repairs: removed 6 blank lines my replacements had added inside lists; fixed 5 doubled indents; restored line starts (Reason / Rests on / Serves / A spike must prove) in ECS-REC4, DET-REC1/2/4/6, DATA-REC1/2, TICK-REC2/5 after reflow. Table column check: 0 mismatches.

## Change log by type (one line each)
### source (9)
- source: R1-DET-F33 added (std HashMap "Usage in const and static", re-read 2026-10-28); re-sources DET-F25's wrong "per process".
- source: R1-BND-F34 added (MDN Structured clone algorithm, new page); re-sources BND-F15's unsupported copy-by-default clause.
- source: R1-BND-F35 added (WebAssembly JS API §5.3, new page); the basis REC2 was missing (verifier BND finding 1).
- source: R1-ECS-F35 added (std Clone re-read + docs.rs RigidBodySet re-read); basis for the corrected ECS-REC1 mark 2.
- source: R1-DET-F34 added (docs.rs rand_pcg re-read + Lcg64Xsh32 new page); basis for DET-REC4's Pcg32 fallback.
- source: R1-BND-F17 quote replaced by the rule the claim needs (threads Overview.md re-read).
- source: R1-BND-F23 unsupported clause dropped; MDN COOP re-read, both quotes recorded; -> R1-BND-OQ9.
- source: R1-DET-F30 "no seed" now cites MDN Math/random (verifier's read).
- source: R1-ECS-F33 tier dated: rustwasm/book archive notice (verifier's read).
### recommendation (15)
- recommendation: ECS-REC1 reason (clone_from overreach fixed with F35; bevy_ggrs named third-party; hecs F19/F20 weighed; uneven 0.x argument, churn inferred).
- recommendation: ECS-REC1 mark 2 rewritten (own tables 0 allocations; Rapier's allocations counted separately; numbers unchanged).
- recommendation: TICK-REC2 reason ("halves" marked inferred; "every legacy measurement" corrected to the harness rates).
- recommendation: DET-REC4 Pcg32 fallback now rests on F34.
- recommendation: DET-REC5 reason and pass mark: per instance (F33), any two runs.
- recommendation: DET-REC6 reason: per instance (F33).
- recommendation: BND-REC2 reason: why one copy (F15, F35).
- recommendation: BND-REC3 basis F15 -> F34 (+F15 for the transfer exception).
- recommendation: BND-REC5 reason: each cost names a finding (F20, F21, F23, F24), memory API added; login chain on F20 + F23a; popup sub-question -> OQ9.
- recommendation: ABL-REC1 reason rewritten to the source's words (C1); "marking as tag-granting effect" marked our reading.
- recommendation: ABL-REC1 mark 3 follows LOOP_V2 §6 ("item", "last hit or tick").
- recommendation: ABL-REC2 reason re-based on F17's latency answer (C2); hot-path point marked inferred; Rests on + F21.
- recommendation: ABL-REC4 text: effect-removal gap and F17's known failure stated; Rests on + F17.
- recommendation: DATA-REC5 basis wording (F21 inference) and partial §6 trace stated; mid-match choice -> new OQ12.
- recommendation: DATA-REC6 secure-context caveat (F29) added, pointer to OQ11.
### number (4)
- number: BND-F2 breaking releases: 0.2.122 dropped (0.2.123 and 0.2.128 remain).
- number: BND-OQ7 "three times" -> "at least twice" in 2026.
- number: header totals (findings 180 -> 185; OQs 67 -> 76 with 3 settled; 10 fix-step reads).
- number: source tier counts (shipped game 8 -> 7, other 14 -> 16, none dual-labelled); determinism unit row note.
### nit
- nit: labels upgraded to confirmed: ECS-F15, F16, F19, F20, F28; TICK-F12, F16; DET-F5, F9, F11, F23; BND-F8, F9, F15 (list), F24; ABL-F4, F15, F17, F18, F23, F24, F25; DATA-F12 (features).
- nit: locations: TICK-F1, F3, F18 (heading); DET-F8 (note); ABL-F2, F3, F4, F14, F16; DATA-F15 (§9.3).
- nit: missing words/conditions: ECS-F29, F30 (serde-serialize), F31 (platform condition); TICK-F18 (Chrome's four conditions); DET-F12, F14, F21; BND-F1, F19, F27; DATA-F5; ABL-F5 ("only" removed).
- nit: tiers/versions: DET-F31, F32 -> other; sources rows 30, 34, 82; reading-rules tier line; DATA-F12 ron 0.12.3.
- nit: Rests-on additions: ECS-REC1, REC2, REC3; TICK-REC2; DET-REC1, REC2, REC5, REC6; BND-REC1, REC2; ABL-REC3, REC5, REC6; DATA-REC1, REC2, REC7.
- nit: Serves fixes: TICK-REC3 (§3), TICK-REC4 (counters trace to no §6 item), DET-REC6, BND-REC1, BND-REC6 (§6.1 dropped), ABL-REC6 (no §6 item).
- nit: wording: ECS-REC3 gloss; ECS-REC4 serde-serialize; TICK-REC2 P3 explanation; TICK-REC5 XU-OQ3 pointer; options (ECS snapshot cell; DET Q4, Q5; TICK 4a three cells); TL;DR "by instance"; GN10 legacy bullet; header status/date.
- nit: OQ bookkeeping: ECS-OQ2, XU-OQ5, XU-OQ6 settled; DET-OQ2, OQ5 (risk 9), OQ7 notes; DATA-OQ11 note; R1-RR-OQ1..7 added (rust-reviewer risks, reported evidence only); 2 hinge-table rows; "Sources read at the fix step" table; part 7 merged table + counts + applied/not-applied lists.

## Not applied (and why)
- rust-reviewer risks 2,3,4,6,7,8 (+tokio part of 1) -> open questions R1-RR-OQ1..7, not rec changes: fix rule 6 (no new recs/claims); evidence read only by the reviewer.
- rust-reviewer risks 1 (beyond tokio), 10: R2's.
- TICK verifier #7 (REC5 "shared memory"): kept; inter-unit conflict, XU-OQ3 decides; pointer added.
- DATA verifier #3 (REC5 argue or cut back): not cut; trace stated partial; choice -> DATA-OQ12 (Mustafa/Advisor).
- TICK verifier #6 (REC4 counters): kept, trace stated honestly; own pass marks need them.
- DET verifier #7 (REC5 §6.5 loose): kept; order decides server-side outcomes (inferred).
- ABL verifier #9 (one GAS source): no change possible; ABL-OQ1 stands.
- Noticed, not named by reviewers, left unchanged (rule 6): TICK options 4a 30 Hz "a rate legacy never measured" (GC5 did measure fire rate at 30 Hz).

## Final header status
verified (every recommendation claim; 5 re-sourced claims await the second pass)

DONE
