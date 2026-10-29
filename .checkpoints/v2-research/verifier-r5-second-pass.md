Task: R5 wave 2 second pass (verify the owner's fix to docs/research/r5-toolchain-quality.md, diff 9a623a0..HEAD), started 2026-10-28
- Read inputs: verifier-r5-result.md, test-engineer-r5-fix(-result).md, full diff 9a623a0..HEAD (748 lines). Owner reports 34 nit, 8 rec, 1 number, 0 source; 1 new OQ (TEST-OQ8).
- Suspects to check: (a) CI-REC5 headline "Jobs skip by condition" vs new "skipped dep fails the final job" (+ TEST-REC1 conditional Safari/bench rows); (b) F6 cited for "safaridriver"/macOS (F6 only says webdriverio supports safari); (c) TEST-OQ8 "said to run each test in its own process" unsourced; (d) G9 "fixed seeds" plural vs REC6 one seed; (e) bench row R1-TICK bench not in Rests on; (f) GATE-REC2 G0-G2 meaning; (g) R1-ECS-REC2 / R1 A2 / R2-TRN-REC4 mark 2 traces; (h) counts 45 OQ, 39 not-read rows, 60 claims.
Next: line numbers in current file, then the cross-file checks.
- Item 5 arithmetic done: OQ bullets 45 = owner 29 (LAYOUT 5+TSLINT 6+RUSTLINT 5+TEST 8+GATE 5) + helper 13 (CI 7+PREV 6) + XQ 3; REC bullets 36 (26+10); not-read rows 25+10+4=39 (base 24 -> 25); part 7 table: 48 rows, 59 distinct ids + CI-F10 = 60 (GATE-F4 appears twice, counted once); fixed 6 + no change 54 = 60; M = 117-2 = 115. Header table TEST OQ1-OQ8 matches. All match.
  Nit: header says "every source below was read that day (2026-10-28)" and Counts says "Every source was read 2026-10-28", but F22's README re-read is dated 2026-10-29.
Next: internal checks of rewritten RECs (GATE-REC1 G0-G2, CI-REC5, TEST-REC1, RUSTLINT F1/F2/F6, TEST-F5/F6, TSLINT-F14/F20).
- Internal read done (RUSTLINT-REC1/3, TEST-REC1/2/4/6, GATE-REC1/2/3, CI-REC5, PREV-REC1). Found:
  * CI-REC5 (file:1545-1567): headline still "Jobs skip by condition" (1547) and reason "conditional skips count as success" (1556), but body now fails the final job on any skipped dependency "until R5-GATE-OQ1 allows a deliberate skip" (1559-1560). Also PREV-REC1's deploy job (skipped on fork PRs, mark 2, file:1586) and TEST-REC1's conditional Safari row: CI-REC5 does not say they sit outside the final job's needs. Wording gap introduced by the fix; safe direction (blocks, does not let a failure merge). Marks (1) and (5) do not contradict (GATE-OQ1: until then every check runs).
  * RUSTLINT-REC3 (1335-1347): body now cites F1 ("picked one by one") but Rests on (1344) lacks F1. Nit. F1 quote supports it ("cherry-pick lints").
  * TEST-REC2 reason (1414-1415) and options row (1066) attribute "safaridriver" to F6; F6 only says webdriverio supports Safari. Nit.
  * TEST-REC1 bench row (1394) cites "R1-TICK's tick bench" but Rests on (1398) names no R1-TICK id. Check R1.
  * G9 "fixed seeds" plural vs REC6 one seed: harmless, not a contradiction.
  * GATE-REC2: G0 toolchain check and --quick = G0-G5 incl G1 fmt, G2 clippy: matches table (1637-1639). Legacy lines re-checked at HEAD: gate.mjs:37 exit 2, :71-89 resolveBase, :194 ORDER, :213 exit codes, :372 [5a] smoke: all hold.
  * F21 "ESLint 10 is now current (F14, F20)": F14 (233) and F20 (263) both say latest v10.13.0. Holds.
Next: cross-file checks in r1/r2/r3 research files.
- Cross-file checks done:
  * R1-ECS-REC2 Serves (r1-simulation-core.md:1937-1938): "Serves VISION_V2 §4 (fair to about 100 ms of ping: prediction and replay need reproducible ticks) and §6.6 and §6.8" -> the §4/§6.6/§6.8 retraces hold.
  * R1 option A2 (r1:1725): "| A2 One dedicated module worker for the sim | ... | Serves §6.2, §6.9, §4 |" -> owner's LAYOUT-REC5 disagreement is sound.
  * R2-TRN-REC4 mark 2 (r2-netcode.md:2657): "App-level RTT agrees with quinn's server-side `rtt()` within ±5 ms at 100 ms injected RTT. Proposed." -> TEST-REC4 fix matches R2 exactly (±5 ms, 100 ms).
  * R3-MEASURE-REC3 (r3-rendering.md:1197-1200): "CI (W-A) gates only hardware-independent numbers ... frame times and GPU times come only from the reference machines" -> bench row's "CI gates counts, not timings" holds.
  * R1-TICK bench: r1:2948 "a Rust tick bench and a Worker tick bench in CI (R1-TICK-REC2 P1–P3)" -> exists; TEST-REC1 Rests on lacks R1-TICK-REC2 (nit).
  * R2-TRN-OQ8 (r2:3019), R1-DET-OQ3 (r1:2605), R1-BND-REC5 (r1:2248) exist.
Next: legacy CD ids; then WebFetch nextest README (TEST-OQ8's "process per test") and vitest browser page (safaridriver).
- Legacy CD4, CD6, CD10-CD14 exist (docs/LEGACY_INVENTORY.md:85,87,91-95). TEST-REC3 Rests-on fix holds.
- WebFetch github.com/nextest-rs/nextest: no sentence on process-per-test; only "a new, faster Cargo test runner." and "A next-generation test runner for Rust." docs.rs/crate/cargo-nextest/latest: same, "A next-generation test runner for Rust.", no process sentence. => TEST-OQ8's "said to run each test in its own process" is unsupported on allow-listed pages (marked unverified in the file; still a new unsourced fact inside an OQ).
- WebFetch vitest.dev/guide/browser/ (v5.0.5): "`webdriverio` supports these browsers:" Firefox, Chrome, Edge, Safari; no sentence mentions safaridriver or macOS. => "real Safari is reached through safaridriver on macOS (F5, F6)" (TEST-REC2 reason) over-attributes to F6. Nit.
- The fix's added quotes (F8 video playback, F13 nontrivial, TEST-F2 random seed, CI-F2 Cargo.toml) match verifier-r5.md lines 22, 25, 30, 48 verbatim. Next: re-open those sources myself (F8, F13, F16, F22, TEST-F2, CI-F2).
- Sources re-opened myself (all confirmed, exact words):
  * playwright.dev/docs/browsers: "for the closest-to-Safari experience you should run WebKit on mac, for example if you do video playback."; "Playwright doesn't work with the branded version of Safari since it relies on patches."; "...often before these updates are incorporated" + "into Apple Safari and other WebKit-based browsers." -> F8 fix confirmed.
  * typescript-eslint.io/rules/no-floating-promises: "Extending "plugin:@typescript-eslint/recommended-type-checked" in an ESLint configuration enables this rule." -> F16 fix confirmed.
  * github.com/prettier/prettier: "JavaScript · TypeScript · Flow · JSX · JSON" / "CSS · SCSS · Less" / "HTML · Vue · Angular" / "GraphQL · Markdown · YAML"; "Prettier is an opinionated code formatter." -> F22 fix confirmed.
  * typescript-eslint.io/users/configs: "We recommend a TypeScript project extend from `plugin:@typescript-eslint/strict` only if a nontrivial percentage of its developers are highly proficient in TypeScript." -> F13 fix confirmed.
  * docs.rs proptest Config (1.12.0): "Seed used for the RNG. Set by using the PROPTEST_RNG_SEED environment variable"; "If the environment variable is undefined, a random seed is generated (this is the default option)." -> TEST-F2 fix confirmed.
  * github.com/Swatinem/rust-cache: "a hash of the workspace's Cargo.lock and of the Cargo.toml of the workspace root and every workspace member (if present)." -> CI-F2 fix confirmed.
Next: line numbers for item 1, grep for leftover §6.2 / "section 1" / "part 8" / 200 KB / "real Safari" phrasing.
- Rests-on coverage recounted by script (scratchpad restson.py, recommendations section only): HEAD 115 of 117 findings named (not named: R5-CI-F8, R5-PREV-F16), same as base. M = 115 holds.
- TSLINT-F22 (a key claim, in TSLINT-REC3's Rests on; not in the verifier's 60) is now confirmed at its source by this pass -> label could read 61 of 115 (R6 precedent) or stay 60 if only the wave 2 spot-check counts.
- More nits found: TEST-REC1 mark (:1401) "one planted defect per layer fails that layer" cannot hold for the new Benchmarks row (:1394, "never a gate pass or fail"); XQ-OQ3 (:1880) "should carry the header set production will ship" asserts an answer while naming engine-architect as decider; F22 re-read dated 2026-10-29 vs "Every source was read 2026-10-28" (:2274) and header date line (:10).
- Line map for item 1 done (see answer). Remaining: write answer. Verdict: only nits/minor wording remain; CI-REC5 headline is the largest.
DONE
