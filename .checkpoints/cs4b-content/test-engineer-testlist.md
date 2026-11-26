# test-engineer: 4b test-list read (what a planted defect slips past)

Task: before 4b ("Content and the world") is built, read its test list; per step 1-9 and the links check, say which
plausible defect the listed tests miss and the smallest test/plant that closes it. Not a review round (advisor ruling).
Read-only; notes only here. Started 2026-11-26, worktree .gate/lane1-takeover (cs/4b-content, main 2483e9a).

Memory: no local agent memory exists (.claude/agent-memory-local/test-engineer absent in main checkout and worktree).

## Plan
1. reread-sheet.md (4b steps, B.1 grammar cut, Mustafa's answers of today).
2. plan.md s5 (5.1-5.3), s9 row 4b, s11 (testlines --check), s17.
3. engine-architect-cs4.md 1.12 (+1.3, 1.4); rust-reviewer-cs4.md blocks 2 and 5.
4. Mark texts as needed: build-plan report Part 2/3; ADR 0005, 0004 PM rows.
5. 3a/3b/4a reports (Evidence, Reviews, Not verified); plants-4a.py at c17a054.
6. Today's code where tests hook in (build.mjs, build-checks, rules_scan, files.rs, wasm.rs, CI, playwright config).
7. Per step: gap -> closing test/plant, where, lines. Rank must/should/optional.
8. Handback (<~120 lines).

## Source notes (one line per source as read)
- reread-sheet.md: steps 0-9 + LINKS-CHECK; B.1 cut grammar (objects/arrays/strings/numbers; refused by name: null,
  true, false, dup keys, BOM, trailing comma, comments, NaN/Infinity, leading zero, depth>32, string bytes >0x7E or
  <0x20, escapes other than \" \\ \n); numbers keep full RFC 8259 grammar; 3b green before any content/ file (but
  content/source/); B.2 MapBeyondAim (x,z within +-327.67, y 0..655.35, constants beside AimPoint; plant 1 cm too wide).
- plan.md s3.5 (this read), s5 (steps, ladder: never moved = built-file check, collision world + PM-0004-32, 7 stages
  + 3b proofs, Testlines check), 5.3 (4 refusal classes, 5 plants + 2 controls, native baseline, target-cpu=native
  plant), s9 row 4b, s11d (testlines --check, 3 plants), s14 (no CI edit; tools' --check run by plain tests), s17.
- engine-architect-cs4.md 1.3 (7 stages; stage 2 still says "escapes per RFC 8259, lone surrogate refused": superseded
  by B.1), 1.4 (writer = JSON.stringify(sortedKeys, null, 2)+"\n", -0 written "-0"), 1.12 (mark table; PM-0005-4
  classes list "bad UTF-8", "__proto__", 10,000-deep, 50 MB files; written before the B.1 cut).
- rust-reviewer-cs4.md block 1 (parry3d =0.31.1, std off untried; 31 overflow blocks; 3A-FEATURE-VERSION live),
  block 2 (twin names + opcode walk of production + imports; rules list; P1-P5 planted in real source by plants
  script; controls; native: rustc --print cfg, planted CARGO_ENCODED_RUSTFLAGS=-Ctarget-cpu=native; P2 decides
  whether release inlines libm fmaf -> dev-profile twin "before Walk merges"), block 3 (static world: -0 -> +0 at
  validation; tree dump via traverse + accessors, FNV-1a 64, LE to_bits; depth <= 31), block 5 rows -11, -32, -33.
- plants-4a.py (c17a054): caught() needs non-zero exit AND all needles; restores world.rs + recordings byte for byte;
  clean tree green after. Weak spot: plant A Chromium accepts needle "the build failed" when the build fails.
- 4a report Evidence/Reviews/Not verified: 15 caught/4 held; 4A-NA-4B, 4A-EA-4B (first-loop remade with frozen
  content, every hash line byte for byte); round 1 found a plant caught by chance (1 in 64).
- 3b report: CI has no lint step (line 89: clippy by hand only); decoder fuzz full count only in release, plants found
  at input 17 and 0 (in-test planted decoders).
- 3a report (plant lines only): verifier 40 plants, 2 caught "at their named test for a neighbouring reason";
  3A-PLANTS (a plant's build left on disk shows in the next plant).
- Code: build.mjs (both flavours build sim-wasm --release: one twin path :55; wasm-bindgen then size table; no FMA
  check yet), build-checks.mjs (CARGO_OVERRIDES refuses CARGO_ENCODED_RUSTFLAGS/RUSTFLAGS/RUSTC* :218-229; feature
  lines carry no version :304), build-checks.test.mjs (link test skips if no link :117-123), rules_scan.rs (reads
  crates/sim/src only), files.rs plain() (lstat of last component; missing = Ok), wasm.rs (test-module exports),
  Cargo.toml (unsafe_code deny :21; release overflow-checks on for members :30-31), toolchain.yml (release test line
  :162 has no `content`; no clippy; job timeout 15 min :38; SHAMBLE_WRITE_TYPES diff :159-161), playwright.config.ts
  (firefox runs tests/rules/ :37-39), protocol messages.rs AimPoint i16/u16/i16 cm (:137-144); content may not
  depend on protocol (EA 1.12 foundation-line plant).
- ADR 0005 PM-0005-2..8 rows (:2298-2304), ADR 0004 PM-0004-11, -23, -32, -33 rows (:1132, :1144, :1153-1154).
- Probe (scratchpad, rustc 1.99.1 direct, no cargo): a #[target_feature(enable="relaxed-simd")] fn calling
  f32x4_relaxed_madd compiles for wasm32 at -O; module target_features lists +relaxed-simd and +simd128; this crate's
  own symbols come out v0-mangled, and a generic instantiation's v0 name ends in the instantiating crate
  ("..._18f32x4_extract_laneKj0_ECsk3U3ordMx5T_1p"), not the function's name.
- Saved Export (cs4-walk/testlines-export.json): 15,567 B, ASCII only, no escapes, 371 number tokens (no exponents,
  no -0), and `true` 5 times (values.measuring_grid.grid_on, values.stamina.stamina_on, values.pushing.dummies_walk,
  values.grenade.fragments, values.aim_down_sights.sight_line).
- Host: fma and avx2 present (/proc/cpuinfo).

## Findings (one line per point as found; M = must, S = should, O = optional)
- M1 step 1: plants must be caught by the claimed check (needle = the check's own message; never "the build
  failed"): P3 may be refused by wasm-bindgen before the check (run twin rules before wasm-bindgen); manifest plants
  (protocol into content; P2 if it needs an edge) fail cargo --locked first (update Cargo.lock offline in the plant,
  restore byte for byte); after all plants the players' file rebuilt to the pre-plant sha256 (3A-PLANTS).
- M2 step 1: native side cannot fail: the planted CARGO_ENCODED_RUSTFLAGS is refused by refuseSettings (today's code),
  not by nativeTargetProblems; and rustc --print cfg in build.mjs says nothing about CI's `cargo test`. Close: a
  cfg!(target_feature = "fma"/"avx2"/"fma4") assertion test in sim/content/devtools native tests; plant
  RUSTFLAGS=-Ctarget-feature=+fma on that test (~8 lines).
- M3 step 1: name rule "path ends in" misses v0 generic instantiations (end in the instantiating crate) and legacy
  names end in a hash: match any segment exactly; positive control (name section present, >= N names, sentinel);
  near-miss controls fmaxf/fminf; P2's inlining answer (dev-profile twin) decided in step 1, not "before Walk merges".
- M4 step 1: linkedLibmFunctions is only a report; with parry std off, transcendentals route to libm (B1-12) and are
  not on the C-name ban list -> make libm functions outside the vectored allow-list fail; plant a glam/simba acos/sin
  reachable from an export.
- M5 step 1: walker exactness: each body ends exactly at its size at depth 0; unit modules with padded 5-byte LEBs,
  overlong call_indirect, f32/f64.const holding 0xFD bytes (pass), 0xFC sub-ops, unknown op fails; real-file count.
- M6 step 2/3b: ground truth per generated file (class + expected kind + the path or line/col the generator changed,
  PM-0005-4's own words) asserted natively; cross-target compares the whole record + message() + accepted files'
  canonical bytes and value bits (shared defects like "depth check after descending" pass a native==Wasm compare).
- M7 step 2/3b: generator classes must follow B.1 (EA 1.12's list predates the cut): literals, BOM, comments, control
  and >0x7E bytes in and out of strings, refused escapes, unterminated, overflow 1e309, >2^53 in all forms, exponent
  digit floods, depth 32/33/10,000, size caps, typed classes, MapBeyondAim; plus decide tab/CR whitespace, top-level
  non-object, empty key (B.1 silent).
- M8 step 3b: oracle independence: random-bit half's texts from V8 String(x), decimal half's expected canonical text
  = JSON.stringify(JSON.parse(t)); comparisons in CI (Vitest + rules spec, which Firefox also runs), not a one-off
  script; writer plant that only the JSON.stringify compare catches (17 digits always; exponent switch at 1e16);
  overflow numbers are refusals, not corpus values (bits of inf match JSON.parse).
- M9 step 3b: fuzz reach: >= 90 % structure-aware mutations of valid seeds, stage histogram with minimums, per-input
  max time, seed printed, fresh seeds by hand into notes; planted unwrap on a mutation-only path, run alone, index
  recorded; content is not in CI's release line and CI has no clippy: put the full count in a devtools test or ask
  the CI Builder before 3b.
- M10 step 2/5: the Export holds `true` x5; B.1 refuses `true` -> testlines cannot read it with the content reader;
  name the reader and its tests before step 2 (a boolean mode must be proven off for content files).
- M11 step 3: MapBeyondAim six cases + six bound controls; constants cannot be read from protocol by content (the
  foundation line forbids the edge) -> equality test in a crate that sees both, against AimPoint's field types.
- M12 step 5: testlines: 1-ulp plant (the 0.001 plant passes a tolerance check), bits compare; plants for "a row
  naming a place that does not exist" (claimed in 11d, unplanted) and "a content field no row maps".
- M13 step 7: no-state-change proof: per-tick hash lines diffed against main 2483e9a's file (not regenerated);
  plant content folded into state at start, caught N/W/C at step 1; frozen copy's digest asserted (else untestable
  in 4b).
- M14 step 8: dump content (node index, child links, leaf flag, leaf data, collider list in index order, to_bits);
  whole-dump compare + committed known answer; -0 map == +0 map; permuted-authoring map == id-order map; ray classes
  (axis-aligned with +-0, grazing, from inside, shared edges, wedge slope, misses) with a histogram.
- S1 step 1: import rule by token, near-miss controls, a Math.sin import and a __wbg_now import plant.
- S2 step 1: collision-only rule by exact path, refuse pub use / alias re-export; extend scan (hashed, clocks,
  unwrap/expect/panic/todo) to crates/content/src (no clippy in CI; HashMap order would split error records).
- S3 step 1: simFeatureList by version: two-version fixture (today's code is the plant); record the check's result at
  step 8 (first parry link) and in 4c (casts); 4b claims nothing about cast paths.
- S4 step 2: writer structure vs JSON.stringify over random valid trees (empty {} [], nesting, ", \, LF); plant "[\n]".
- S5 step 2: integer getters (fraction, negative, -0, >u32); plant `as u32`; range tests at next_up/next_down (PM-0005-7).
- S6 step 4: oracle from the number text (3 x ms), u32 boundary, plant truncation; content's 3000 == sim's constant;
  the 0.02 s plant must fail --check, not just add a row.
- S7 step 5: content-bundle plants (CRLF, no final newline, unsorted keys, "1.0"); nested `source` refused; link/FIFO
  under content/ refused before reading; bounded read.
- S8 step 5/6: Export sha256 pinned in reviewed code; write->check->bundle --check round trip; 3-5 values pinned by
  hand from the Export text (a mis-mapped row is shared by write and check).
- S9 step 7: players'-file boundary classes (6) as Error not RuntimeError, instance usable after; Worker start refuses
  non-ArrayBuffer/>8 MiB; bytes reaching the Worker == served bytes; page content.json == tool output.
- S10 step 8: StaticWorld::build on hand-made documents (NaN, inverted, 0/1/512 shapes) never panics; how a 33-level
  tree is made; extend the bounded fuzz to ContentSet -> StaticWorld::build.
- S11 step 9: drawn objects vs map (dev hook); broken-bundle spec with a bundle JSON.parse accepts and Rust refuses;
  sentence pinned; render counts derived from the map.
- S12 LINKS: scan that the 4 Rust files and 2 TS readers read recordings only through the helper; TS link test must
  not skip in CI; bounded read.
- O1-O7: Export and legacy tokens as written in the corpus; >20-digit cases vs an exact reference; O(n^2) dup check
  and 8 MiB memory high-water; yard shapes pinned; PM-0004-33 partial in 4b; ENSURE_NO_PANIC unit case; drawn vs
  collision rays (4c).

DONE 2026-11-26.

## Added after DONE (evidence for M2 and M13)
- Probe: `RUSTFLAGS=-Ctarget-cpu=native CARGO_ENCODED_RUSTFLAGS=-Ctarget-cpu=native rustc --print cfg` shows no fma/avx2
  (rustc never reads cargo's flag variables); `rustc -Ctarget-cpu=native --print cfg` shows both. So the designed
  nativeTargetProblems cannot see a planted flag in the environment at all.
- recording.rs:377-393: the hashes file's line 1 is sha256(recording) + count, so remaking first-loop with a frozen
  content header changes line 1 by design; the proof is per-tick lines (and first-loop.vector.txt) equal to main
  2483e9a's, since make_recordings rewrites both sides and its --check then compares the tool with itself.
- gate.yml: no cargo, no clippy (confirmed).
- Report handed back 2026-11-26 (14 must, 12 should, 7 optional; two decisions for the main session to route:
  TL-EXPORT-READER, TL-AIM-CONSTANTS). DONE.
