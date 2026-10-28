# Second verifier pass on R1: the verifier's report, verbatim

Workflow run wf_64d0bbcd-730 (script `v2-research-second-pass-run1.js`, prompt `prompts/verifier-r1-second-pass.txt`), returned 2026-10-28 (about 16:24 UTC). Notes with every quote: `verifier-r1-second-pass.md`.

VERDICT: PASS WITH NOTES. Two claims are wrong, both small and quick to fix (R1-DET-F34 with R1-DET-REC4, and R1-DATA-REC6). The five re-sourced findings hold where they quote their sources. Gate and symbol checks were skipped: this is a research check (task rule 1).

**Table** (scope · id · claim · source · verdict · location / exact words)

A (new findings)
- A · R1-DET-F33 · the HashMap seed is per instance · std HashMap 1.99.1 · confirmed · section "Usage in `const` and `static`": "As explained above, `HashMap` is randomly seeded: each `HashMap` instance uses a different seed, which means that `HashMap::new` normally cannot be used in a `const` or `static` initializer." No sentence ties the seed to "process".
- A · R1-BND-F34 · postMessage copies through structured clone · MDN Structured clone algorithm (last modified Jun 29, 2025) · confirmed · first paragraph: "The structured clone algorithm copies complex JavaScript objects. It is used internally when invoking structuredClone(), to transfer data between Workers via postMessage(), …". The page has no sentence on transferables, as the finding says.
- A · R1-BND-F35 · a Wasm memory's buffer has a detach key · WebAssembly JS API §5.3 · confirmed for the step · "create a fixed length memory buffer", step 5: "Set buffer.[[ArrayBufferDetachKey]] to "WebAssembly.Memory"." The draft now reads "Editor's Draft, 28 October 2026"; the file says 27 October (date only).
- A · R1-ECS-F35 · derive and default clone_from; RigidBodySet keeps the default · std Clone 1.99.1 and docs.rs rapier3d 0.36.1 · confirmed · "The `derive`d implementation of Clone calls clone on each field."; "`a.clone_from(&b)` is equivalent to `a = b.clone()` in functionality, but can be overridden to reuse the resources of `a` to avoid unnecessary allocations."; RigidBodySet clone_from: "Performs copy-assignment from `source`.", source link `https://doc.rust-lang.org/nightly/src/core/clone.rs.html#245-247`.
- A · R1-DET-F34a · Pcg32 suits 32- and 64-bit CPUs, takes a stream in its constructor, can jump ahead · docs.rs rand_pcg 0.10.2 (crate root and Lcg64Xsh32) · confirmed · "This is a good choice on both 32-bit and 64-bit CPUs (for 32-bit output)."; `pub fn new(state: u64, stream: u64) -> Self` "Construct an instance compatible with PCG seed and stream."; "Multi-step advance functions (jump-ahead, jump-back)".
- A · R1-DET-F34b · "the pages say nothing on how streams relate" · Lcg64Xsh32 page · **wrong** · `new` docs: "Note that two generators with different stream parameters may be closely correlated." (the struct docs say it again).

B (basis claims that had no first-pass "confirmed")
- B · R1-BND-F15 · a transfer moves the buffer and the sender loses it; typed arrays; WebTransport streams · MDN Transferable objects · confirmed (first-pass quotes, text unchanged) · the unsupported F15b clause is gone; R1-BND-REC3 now rests on F34.
- B · R1-BND-F2 · version match since 0.2.128; 0.2.123 breaking; dates; year from the asset timestamps · GitHub releases list · confirmed · "This rejects the invalid widening but is a breaking change for callers that relied on widening `&mut` references."; "10 Apr" … "25 Sep"; asset timestamp "2026-09-25T07:11:34Z"; the 0.2.122 notes have no "breaking".
- B · R1-BND-F23 · `same-origin` severs popups; `same-origin-allow-popups` is for OAuth and payments · MDN COOP (Jul 12, 2026) · confirmed · "A document with `same-origin` will always open a document with `unsafe-none` in a new BCG."; "…For example, when using a cross-origin service for OAuth or payments." The F23c clause is gone: it survives only as the correction note and R1-BND-OQ9, and R1-BND-REC5 does not use it.
- B · R1-BND-F32 · hardwareConcurrency is not a hardware fact · MDN (Oct 16, 2024) · confirmed · "The browser may, however, choose to report a lower number of logical cores … so don't treat this as an absolute measurement of the number of cores in the user's system."
- B · R1-DET-F25 · random seed, arbitrary order, hasher per map · std HashMap · confirmed · "The algorithm is randomly seeded, and a reasonable best-effort is made…"; "per-`HashMap` basis". "Per process" is gone from F25, R1-DET-REC5 and R1-DET-REC6.
- B · R1-ECS-F19 · hecs freelist and its condition · docs.rs hecs 0.11.2 World · confirmed · "Entity handles will be allocated deterministically between different worlds (e.g. across serialization)"; "May only be called when the set of live entity IDs exactly matches those that were live".
- B · R1-ECS-F20 · World has no Serialize; column format · hecs serialize and column pages · confirmed · "Components are not necessarily serializable, so we cannot directly implement serde::Serialize for World."; "Available on crate feature `column-serialize` only."
- B · R1-ECS-F34 · no source gives a Wasm size or compile time · claim of absence · unsupported (an absence cannot be quoted; harmless, since it only makes the size marks proposed).
- B · R1-ECS-F7 · single-threaded executor; multi_threaded off by default · docs.rs bevy_ecs 0.20.0 · confirmed · "Runs the schedule using a single thread."; the four defaults are async_executor, backtrace, bevy_reflect and std.

C (changed findings)
- **Label only (11):** R1-ABL-F2, F3, F14, F16; R1-BND-F33; R1-DATA-F15; R1-DET-F8, F31, F32; R1-TICK-F1, F3.
- **Confirmed from the first-pass notes, no fetch (14):**
  - R1-ABL-F4 (abilities notes l.8), F5 (l.50), F15 (l.19), F18 (l.22)
  - R1-BND-F24 (boundary notes l.35), F27 (l.43)
  - R1-DATA-F5 (data notes l.6)
  - R1-DET-F5 (determinism notes l.12), F30 (l.51)
  - R1-ECS-F30 (ecs notes l.36), F31 (l.37), F33 (l.42)
  - R1-TICK-F12 (tick notes l.33), F16 (l.22)
- **Confirmed at the source today (15):**
  - R1-ABL-F17: GAS README §11.1.1, question 7. Question: "In scenarios of high latency, this would let the Owning Client "try" to activate the ability again when its local cooldown expires" + "but it is still on cooldown on the Server." Answer: "The short answer there is not a way to prevent this and Paragon definitely had the problem." / "Higher latency connections would have a lower ROF with basic attacks."
  - R1-BND-F8: tsify: "`js` enables serialization through serde-wasm-bindgen and generates the appropriate types for it."
  - R1-BND-F9: ts-rs: "The Minimum Supported Rust Version for this crate is 1.88.0"
  - R1-BND-F17: threads Overview: "It is a validation error to attempt to import shared linear memory" "if the module's memory import doesn't specify that it allows shared memory."
  - R1-BND-F19: rustc book: "By default the `wasm32-unknown-unknown` target is compiled with `-Cpanic=abort`."
  - R1-DATA-F12: docs.rs shows "ron 0.12.3", dated "28 October 2026".
  - R1-DET-F9: on `tan`: "This function currently corresponds to the `tanf` from libc on Unix and Windows."
  - R1-DET-F11: `algebraic_add` is marked "1.98.0 (const: 1.98.0)".
  - R1-DET-F12: RFC 3514: "Also note that in some cases even the underlying target behavior is non-deterministic, namely on wasm."
  - R1-DET-F14: `clamp`: "Panics if `min > max`, `min` is NaN, or `max` is NaN."
  - R1-DET-F21: Rapier: "…which changes the SIMD lane width and is therefore its own determinism domain."
  - R1-DET-F23: Rapier's workspace pins 0.36.1, parry3d 0.31.1, nalgebra 0.35, simba 0.10.2 and glamx 0.3.1; its comment says fast-math "re-enables FMA contraction in glam's sse2 backend (a cross-platform determinism hazard)."
  - R1-ECS-F28: slotmap: "It uses two indirections per random access…"; "If you wish to use these features you must enable the `serde` feature flag…"
  - R1-ECS-F29: clone_from is not overridden (same check as R1-ECS-F35).
  - R1-TICK-F18: Chrome 88 post: "The page has been hidden for more than 5 minutes." / "The chain count is 5 or greater." / "The page has been silent for at least 30 seconds." / "WebRTC is not in use."

D (a claim inside a recommendation)
- D · R1-DATA-REC6 · "file:// and plain-HTTP LAN development need another path" (a secure context is needed) · MDN Secure contexts (Sep 14, 2026) · **wrong for file://** · its table of top-level documents: "`file:///path/to/resource.html` ✅ Secure (`file` URL)". The plain-HTTP LAN part follows from R1-DATA-F29.

**Counts:** 42 confirmed, 2 wrong, 1 unsupported, 11 label only.

**Changed recommendations**
- **R1-DET-REC4: problem.** The Pcg32 fallback lists "a stream number per generator" as a strength and leaves stream independence "for the spike". The source already warns that streams "may be closely correlated" (R1-DET-F34b is wrong). Options Q4 (line 1699) repeats the stream point.
- **R1-DATA-REC6: problem.** The file:// clause is wrong (see the D row).
- **R1-ECS-REC1: OK, one minor point.** It states flatly that derived and default clone_from do not reuse buffers, which R1-ECS-F35 marks as inferred. F35's "derive writes only `clone`" is also a reading: the quote says "calls clone on each field". R1-ECS-F34 is the absence row (harmless).
- **R1-DET-REC5 and R1-DET-REC6: OK, one minor point.** "Any two runs … also catch any order that leaks" claims more than F33 allows; F33's own inference says the maps "can" iterate differently. It is not marked inferred.
- **R1-TICK-REC2: OK on numbers, one minor point on Serves.** The legacy rates check out against LEGACY_INVENTORY: TL13 and TL15 at 1/60, GW14 at 20, 60 and 144 Hz, GN4 at 60 and 144 fps. The arithmetic holds: 0.2 s × 60 = 12 ticks and 0.2 s × 30 = 6. But Serves glosses §6.1 as "(modes up to 48+ with bots)", which reads §3 into §6.1. R1-TICK-REC3 had the same slip and the fix corrected it there.
- **R1-BND-REC5: OK, minor points.** "CORP or CORS": R1-BND-F21 quotes only CORP. Its Serves line names §4 and §5 and does not say it serves no §6 item.
- **OK:** R1-ECS-REC2, REC3 (GC15 "a stuck bolt follows the victim's mesh position"), REC4; R1-TICK-REC3, REC4 (0.25 s × 60 = 15 ticks), REC5; R1-DET-REC1 (STATUS.md:70), REC2; R1-BND-REC1, REC2, REC3, REC6; R1-ABL-REC1 (LOOP_V2.md:85-86 verbatim), REC2, REC3 (1 − 10.0/13.9 = 28.1 %), REC4, REC5, REC6 (V2_PHASE0_BRIEF.md:138-139); R1-DATA-REC1, REC2, REC5, REC7 (EDITOR_V2.md:39).

**E (the change log's number lines): all 4 check out.**
- BND-F2: 0.2.122's notes have no "breaking"; 0.2.123 is confirmed.
- BND-OQ7: "at least twice, in 0.2.123 and 0.2.128" holds.
- Header totals, counted in the file:
  - 185 findings (ECS 35, TICK 26, DET 34, BND 35, ABL 26, DATA 29, no gaps in the numbering).
  - 37 recommendations.
  - 76 open questions: 55 from the units, 12 from assembly, 9 from the fix step.
  - 10 fix-step reads, 3 of them new pages.
- Source tiers: official 138, other 16, shipped game 7, which sums to 161.

**F (header and part 7):**
- Part 7 has 243 rows: 224 verifier rows, 4 gap rows and 15 rows only rust-reviewer checked.
- Its totals add up: verifier 226 = 219 + 1 + 4 + 1 + 1; rust-reviewer 101 = 99 + 1 + 1.
- Missing rows: R1-BND-F32 (now part of what R1-BND-REC1 rests on) and the five new findings.
- Part 7's sentence "No recommendation rests on a claim whose final verdict is wrong or unsupported" is not true until R1-DET-F34/REC4 are fixed.

**Other findings, ranked**
1. R1-DET-F34b and R1-DET-REC4 (and options Q4): the PCG stream-correlation warning is missing, and F34 says the opposite.
2. R1-DATA-REC6: the file:// clause is wrong per MDN.
3. Part 7 needs rows for R1-BND-F32 and the five new findings, plus this pass's verdicts.
4. The minor points in R1-ECS-REC1, R1-DET-REC5 and REC6, R1-TICK-REC2's Serves line, and R1-BND-REC5.
5. Process: fix-step read F10 used the verifier's downloaded GAS README copy instead of the page. I opened the page myself and the quotes match.
6. R1-BND-F35 and fix-step table row F3 give the draft date as 27 October; the page now says 28 October. Date only.

**Header label after this pass:** "verified (every recommendation claim; second pass 2026-10-28: 2 fixes open, R1-DET-F34/REC4 and R1-DATA-REC6)". Once the owner applies those two fixes, it becomes "verified (every recommendation claim; second pass 2026-10-28)". The fix wording can quote the sentences above, which I have already confirmed.

**Not verified:** "a transfer of the Wasm memory's buffer throws" (ECMA-262 and the HTML standard are outside the allow list; the file already marks it inferred). The R1-ECS-F34 absence cannot be proven by a quote.

Notes: /home/user/Shamble-Frontlines/.checkpoints/v2-research/verifier-r1-second-pass.md (ends with DONE). File checked: /home/user/Shamble-Frontlines/docs/research/r1-simulation-core.md
