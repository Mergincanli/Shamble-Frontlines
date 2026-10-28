# rust-reviewer on R1 and R2's server stack: returned report (verbatim), workflow run wf_38e94d0f-403, 2026-10-28

**Rust review of R1 and R2 (Rust claims only): VERDICT REVISE for both**

The facts in both documents hold up: 110 of 112 Rust claims are confirmed, with one wrong and one partly unsupported. The verdict is REVISE because both documents miss Rust risks that should go into their recommendations before the drafts are written:
- **R2:** quinn's security advisory record this year.
- **Both:** client-sized allocations, panic and overflow policy, snapshot allocation, and FMA that differs by target.

Every source was fetched on 2026-10-28. "Confirmed" means the exact words came back from the source; the full quotes are in `/home/user/Shamble-Frontlines/.checkpoints/v2-research/rust-reviewer-r1-r2.md`, which ends with the full table and DONE. Out of scope and not re-checked (browser, JavaScript and spec data, not crate claims): DET-F5, F30–F32; BND-F11–F17, F20–F24, F26, F27, F29–F33; DATA-F15, F19, F22; R2's non-Rust findings.

Legend: C = confirmed, W = wrong, P = partly confirmed, U = unsupported.

**TOPIC R1-a: ECS crates and plain data**
- ECS-F1 · Bevy 0.16 to 0.20 dates; bevy_ecs 0.20.0 · bevy-website news folders, docs.rs · C · "2026-10-08-bevy-0.20"
- ECS-F2 · standalone use · docs.rs bevy_ecs · C · "can easily be used as a standalone crate in other projects"
- ECS-F3 · tables by default, sparse sets opt-in · docs.rs bevy_ecs · C · "This is the default storage type."
- ECS-F4 · query order not guaranteed · Query · C · "Iteration order is not guaranteed."
- ECS-F5 · sort is not cached · QueryIter · C · "The sort is not cached across system runs."
- ECS-F6 · ambiguities ignored; shuffle seed · ScheduleBuildSettings · C · "Defaults to `LogLevel::Ignore`."; "crate feature `debug`"
- ECS-F7 · single-threaded executor; multi_threaded off · executor page, features page · C · "Runs the schedule using a single thread."
- ECS-F8 · World is not Clone · World trait impls · C · the impl list has no Clone
- ECS-F9 · Entity is opaque, not for the wire · Entity · C · "Do not use for serialization between runs."
- ECS-F10 to F14 · bevy_ggrs registration, snapshot cost, traps · README, architecture.md, pitfalls.md · C · "a `HashMap<RollbackId, As>`"; "causing silent desyncs."; "will fire after every rollback."
- ECS-F15 · 0.16 relationships and no_std · 0.16 news post · C (R1 said "inferred") · "One of the hottest ECS features is finally here"
- ECS-F16 · hecs is a library; its goals in order · docs.rs hecs 0.11.2 · C (the order is now confirmed) · "It is a library, not a framework."
- ECS-F17 · arbitrary order, ids reused · hecs World · C · "Entities are yielded in arbitrary order."
- ECS-F18 · hecs World is not Clone · hecs World impls · C · Default, Extend, FromIterator, IntoIterator, Send, Sync
- ECS-F19 · freelist gives matching ids · hecs World · C (the condition is now quoted) · "May only be called when the set of live entity IDs exactly matches"
- ECS-F20 · serialization through a user context; column format · hecs serialize/column · C · "For some applications, a custom approach may be preferable."; "Components are not necessarily serializable…"
- ECS-F21, F22 · plain Vecs for few entity types; state in other structures · hecs README · C · "storing each type of entity in a separate plain Vec."
- ECS-F23 · HashMap seeded per instance · std HashMap 1.99.1 · C · "each `HashMap` instance uses a different seed"
- ECS-F24, F25 · BTreeMap in key order; slices start to end · std · C · "produce their items in key order"; "The iterator yields all items from start to end."
- ECS-F26, F27 · Vec::clone_from reuses its buffer; clone_from can be overridden · std Vec, std Clone · C · "as it avoids reallocation if possible." The default body is `*self = source.clone()`, and derive "calls [`clone`] on each field" (see risk 5).
- ECS-F28 · slotmap versioned keys, no_std · slotmap 1.1.1 · C (the indirection cost and serde support are now confirmed) · "once a key is removed, it stays removed"
- ECS-F33 · generics grow Wasm size; wasm-opt saves 15–20 % · Rust and WebAssembly book, code-size page · C · "can often get another 15-20% savings on code size."

Verdict: APPROVE on the facts. The snapshot recommendation still needs REVISE (risk 5).

**TOPIC R1-b: Rapier and its determinism features**
- ECS-F29 · RigidBodySet is Clone and Serialize; index-plus-generation handles · docs.rs rapier3d 0.36.1 · C · "just an index + generation". Its `clone_from` links to core's default implementation.
- ECS-F30 · whole state serializable; pipelines hold no state · rapier.rs serialization guide · C. R1 does not name the required feature: "When the `serde-serialize` feature of Rapier is enabled, most data structures…"
- ECS-F31 · byte-identical states with enhanced-determinism · same page · C · "will result in the exact same byte vectors"
- ECS-F32 · issue #910 (broad-phase state not serialized) · GitHub rapier #910 · C · "it appears to be because BvhWorkspace is not serialized"; "This was fixed in Parry 0.26.1" / "Rapier 0.33"
- TICK-F8 · physics step defaults to 1/60 s; one CCD substep · IntegrationParameters · C · "(default: `1.0 / 60.0`)"
- DET-F19 · deterministic on one machine only, by default · rapier.rs determinism guide · C · "may result in completely different results."
- DET-F20 · enhanced-determinism conditions (IEEE targets incl. Wasm, insertion order, nalgebra traits) · same · C · "as well as WASM targets."
- DET-F21 · not with simd8; works with parallel · same · C. R1 misses the next words: "which changes the SIMD lane width and is therefore its own determinism domain."
- DET-F22 · the flag forces libm in simba; the default 4-lane SIMD path stays on · rapier3d Cargo.toml · C · `enhanced-determinism = ["simba/libm_force", "parry3d/enhanced-determinism"]`
- DET-F23 · parry's flag forces libm and scalar maths in glamx · parry3d Cargo.toml and Rapier workspace manifest · C. The workspace pins and the fast-math note were "inferred" and are now quoted: "re-enables FMA contraction in" "glam's sse2 backend (a cross-platform determinism hazard)."
- DET-F24 · no stated cost and no test method · determinism guide · C (absence)

Verdict: REVISE (risk 2).

**TOPIC R1-c: floats on Wasm versus native, and libm**
- DET-F1 to F4 · Wasm round-to-nearest, NaN non-determinism, deterministic profile, relaxed SIMD · Wasm 3.0 spec (2026-10-27) · C · "Tools are generally expected to handle and produce code for the full profile by default."; "Relaxed multiply-add allows for fused or unfused results"
- DET-F6 · wasm32 default target features · rustc book · C · "the proposals that are enabled by default"
- DET-F7 · IEEE rounding of basic operations · std f32 · C · "rounded according to the roundTiesToEven direction"
- DET-F8 · NaN bits not portable · std f32 · C (location nit: the sentence is in the type-level NaN bullet, not the "NaN bit patterns" section) · "not guaranteed to be portable or even fully deterministic!"
- DET-F9 · transcendental precision varies · std f32 · C for `sin`; the other methods were listed but not quoted · "varies by platform, Rust version". Also found: "This function currently corresponds to the `tanf` from libc on Unix and Windows."
- DET-F10 · sqrt, mul_add and rounding functions are exact · std f32 · C · "specified by IEEE 754 as `fusedMultiplyAdd` and guaranteed not to change."
- DET-F11 · Rust never fuses on its own; algebraic methods are opt-in · RFC 3514, std f32 · C (stable since 1.98.0 is now confirmed) · "such as turning `a*b + c` into FMA operations"; "may have undefined precision"
- DET-F12 · 64-bit targets fully compliant · RFC 3514 · C. Note: the RFC does name wasm, for NaN: "even the underlying target behavior is non-deterministic, namely on wasm."
- DET-F13 · const evaluation may pick other NaN bits · RFC 3514 · C · "no guarantee that the choice made in const evaluation is consistent"
- DET-F14 · min, max, clamp and signed zero · std f32 · C. Missed: clamp "Panics if `min > max`, `min` is NaN, or `max` is NaN."
- DET-F15 · float-to-int casts defined; overflow depends on flags · Rust Reference · C · "`NaN` will return `0`"
- DET-F16 · Wasm gets the pure-Rust libm, Unix uses the system libm · compiler-builtins math/mod.rs · C · "/// - All unix targets (linux, macos, freebsd, android, etc)"
- DET-F17 · libm 0.2.16 is pure Rust; its `arch` feature touches only exact operations · docs.rs libm, arch/mod.rs · C · "libm in pure Rust"; `pub use x86::{sqrt, sqrtf, fma, fmaf};`
- DET-F18 · no accuracy or cross-platform claim · docs.rs libm, README · C (absence)

Verdict: APPROVE on the facts; REVISE the recommendations (risks 2, 6, 7).

**TOPIC R1-d: collections, RNG, fixed point**
- DET-F25 · HashMap "randomly seeded per process" · std HashMap · **W** · the source says per instance: "each `HashMap` instance uses a different seed". REC5's reason and pass mark repeat the error; REC6's conclusion still holds.
- DET-F26 · BTreeMap sorted by key · std · C · "sorted by key."
- DET-F27 · StdRng is not portable · rand 0.10.3 · C · "even with a fixed seed, output is not portable"
- DET-F28 · ChaCha and PCG are portable; 2^64 streams · rand_chacha 0.10.0, rand_pcg 0.10.2 · C · "deterministic and portable"; "unique streams of output are available per seed/key."
- DET-F29 · the fixed crate has no trig · fixed 1.31.0 · C · "No trigonometric functions are provided"

Verdict: REVISE (one wording fix).

**TOPIC R1-e: wasm-bindgen and type generation**
- BND-F1 · rustwasm org wound down · Inside Rust blog, 2025-07-21 · C (minor: it omits "or transferred to their existing maintainers") · "archived in place"
- BND-F2 · release cadence; CLI must match library; breaking changes in .122, .123, .128 · wasm-bindgen releases · **P** · the version-match rule and .123 are confirmed; **.122 breaking is U** (its notes contain no "breaking" line); the year comes from asset timestamps ("2026-09-25T…") · "Requires matching wasm-bindgen and CLI versions (schema bump)."
- BND-F3 · `.d.ts` written by default · guide cli.md · C · "This is on by default."
- BND-F4 · typescript_custom_section · guide · C · "The primary target for this feature is for code generation."
- BND-F5 · serde-wasm-bindgen makes many boundary calls · guide · C · "requires a lot of calls back and forth"
- BND-F6 · slices are copied · guide number-slices · C (`&[T]` from a TypedArray stays inferred) · "since the results are copied back into its buffer."
- BND-F7 · typed-array overlays avoid copies · Rust and WebAssembly book · C · "we avoid copying the cells across the boundary on every tick."
- BND-F8 · tsify and tsify-next · docs.rs · C (the js feature and the deprecation are now quoted) · "Deprecated in favour of using `Ts<T>`"
- BND-F9 · ts-rs writes types at `cargo test` · ts-rs 12.0.1 · C (MSRV now quoted) · "The Minimum Supported Rust Version for this crate is 1.88.0"
- BND-F10 · repr(C) layout · Rust Reference · C · "add padding bytes to the current offset"
- BND-F18 · Wasm threads need nightly · guide raytrace.md · C · "Note that this requires a nightly Rust toolchain."
- BND-F19 · std::thread::spawn panics on wasm32 · rustc book · C · "There is no means by which this can be overridden." Missed on the same page: "compiled with `-Cpanic=abort`".
- BND-F25 (guide part) · the guide's module-worker line is out of date · guide · C · "only Chrome supports modules in web workers"
- BND-F28 · all four browsers supported · guide · C · "Firefox, Chrome, Safari, and Edge browsers are all supported"

Verdict: REVISE (BND-F2 wording; risks 4, 7, 8).

**TOPIC R1-f: tick and ability crates**
- TICK-F6 · Bevy fixed step 64 Hz · bevy 0.20.0 Fixed · C · "The default timestep() is 64 hertz, or 15625 microseconds."
- TICK-F7 · paused clock stops the fixed schedule · bevy Fixed and Virtual · C · "If the virtual clock is paused, the FixedUpdate schedule will not run."
- TICK-F23 · tokio millisecond timers; Burst is the default · tokio 1.54.1 · C · "MissedTickBehavior::Burst is the default". Tokio 1.54.0 had RUSTSEC-2026-0384 (Interval race), patched ">=1.54.1".
- ABL-F23 · leafwing_abilities 0.13.0 · docs.rs · C (Bevy ^0.19 now confirmed) · "Built for the Bevy game engine."
- ABL-F24 · bevy_gauge 0.5.1 · docs.rs · C (R1 said "inferred") · "Define attributes as expressions, not update systems."
- ABL-F25 · bevy_alchemy 0.6.0 · docs.rs README · C (R1 said "inferred") · "New applications will overwrite the existing one."

Verdict: APPROVE.

**TOPIC R1-g: serialization and data formats**
- DATA-F1 to F5 · serde attributes (deny_unknown_fields, flatten, try_from, default, alias, untagged) · serde.rs sources · C · "Always error during deserialization when encountering unknown fields."
- DATA-F6 · float_roundtrip costs about 2x · serde_json 1.0.154 Cargo.toml · C · "approximately 2x performance cost"
- DATA-F7 · errors carry line, column and category · serde_json Error · C · "One-based line number at which the error was detected."
- DATA-F8 · serde_path_to_error · 0.1.20 · C · "exposes the chain of field names leading to the error"
- DATA-F9 · recursion limit on by default · serde_json · C (the limit's value is not stated) · "only available if serde_json is built with the `"unbounded_depth"` feature."
- DATA-F10, F11 · Value then from_value; preserve_order · serde_json · C · "maintains insertion order."
- DATA-F12 · RON limits · ron README, docs.rs · C (docs.rs now shows 0.12.3, dated 2026-10-28, after R1's read) · "does not yet make any guarantees about roundtrips"
- DATA-F13 · TOML has no null; spec 1.1.0 · toml 1.1.8+spec-1.1.0 · C · Value variants have no null
- DATA-F14 · serde_yaml archived · GitHub · C · "(This project is no longer maintained.)"
- DATA-F16 to F18 · schemars, its garde/validator support, garde · docs.rs, attributes.md · C · "currently conform to JSON Schema 2020-12"; "By default, the context is `()`."
- DATA-F20 · notify's known gaps · notify 8.2.1 · C · "may not emit any events"
- DATA-F21 · Bevy hot reload is desktop-only · bevy_asset 0.20.0 · C · "enable `bevy`’s `file_watcher` cargo feature."

Verdict: APPROVE on the facts (risks 3 and 7 apply to the recommendations).

**TOPIC R2: the Rust server stack**
- TRN-F6 · both WebTransport stacks speak only the draft-02/-07 settings, not -16 · wtransport-proto settings.rs; web-transport-proto 0.6.3 · C · `VarInt::from_u32(0x2b60_3742)`; "// TODO remove when 07 is in the wild"; 0x2c7cf000 appears in neither
- TRN-F15 · self_signed makes a P-256 certificate valid ≤ 14 days · wtransport 0.7.2 · C · "This function uses the `ECDSA P-256` algorithm."
- TRN-F17 · quinn's initial MTU is 1,200 · quinn TransportConfig · C. Also found: max_datagram_size "is guaranteed to be a little over a kilobyte at minimum", against R2's "No source read gives that exact number".
- TRN-F25 · the server chooses quinn's congestion controller; datagram API · quinn congestion, Connection · C · "Experimental! Use at your own risk."; "will wait for buffer space during congestion conditions"
- TRN-F28 · quinn 0.11.12, "mature and active" · crates.io · C for the registry numbers (download counts have moved a day later). "Mature" leaves out 8 RustSec advisories against quinn-proto in 2026 (risk 1).
- TRN-F29 · wtransport 0.7.2; "not … production-ready"; quinn 0.11.6, rustls 0.23.23 · crates.io, Cargo.toml, README · C · "is not considered completely production-ready."
- TRN-F30 · web-transport-quinn 0.13.5 and its crate family · crates.io, README · C (the release list is now confirmed; the qmux tail fragment came back as a paraphrase) · "mirrors the Quinn API, abstracting away the HTTP/3 setup."
- TRN-F31 · h3 is experimental; h3-webtransport idle since 2025-05 · GitHub h3, crates.io · C · "The `h3` crate is still very experimental."
- TRN-F33 · tokio-tungstenite 0.30.0; axum 0.8.9 `ws` · crates.io, docs.rs · C · "Available on **crate feature `ws`** only."; docs.rs lists "tokio-tungstenite ^0.29.0" for axum
- ROOM-F7 · tokio tasks are cooperative; long-running work goes on its own thread · tokio task, spawn_blocking · C (the "inferred" link is now quoted) · "prefer a dedicated thread created with `thread::spawn`."
- ROOM-F8 · a panic ends its thread; abort ends the process; 2 MiB stacks · std thread, Cargo profiles · C · "Terminate the process upon panic."

R2-TRN-OQ5, the maintenance check R2 hands to the Rust reviewer:
- **Releases in the last 6 months:** wtransport had 1 (0.7.2 on 2026-08-11; 0.7.1 on 2026-04-26 falls just outside). web-transport-quinn had 10 (0.11.11 on 2026-07-17 to 0.13.5 on 2026-10-24).
- **RustSec:** there are no advisory pages for wtransport, web-transport-quinn or h3.
- **Shared base:** both sit on quinn ^0.11 and rustls ^0.23, so both inherit the quinn-proto, rustls and ring advisories.
- **REC5 nuance:** web-transport-quinn lists rcgen only as a dev dependency, so "rcgen with web-transport-quinn" means adding rcgen ourselves.

Verdict: REVISE (risks 1, 3, 4).

**MISSED RUST RISKS, ranked**

1. **Open advisories in the network stack (DoS by remote panic or memory exhaustion).**
   - **quinn-proto:** 8 advisories in 2026.
     - RUSTSEC-2026-0037: "Receiving QUIC transport parameters containing invalid values could lead to a panic.", 8.7 HIGH, patched ">=0.11.14".
     - RUSTSEC-2026-0185: memory exhaustion, patched ">=0.11.15".
     - Six on 2026-10-12, RUSTSEC-2026-0338 to 0343: remote panics and buffer-limit bypasses, several 7.5 HIGH, patched ">=0.11.17" or ">=0.11.18".
     - Newest quinn-proto is 0.11.19.
   - **rustls:** RUSTSEC-2026-0285 (patched ">=0.23.45"), and RUSTSEC-2026-0346 and 0348 (patched ">=0.23.46").
   - **ring:** RUSTSEC-2025-0009, patched ">=0.17.12": "an attacker can induce this panic by sending a specially-crafted packet."
   - **tokio:** RUSTSEC-2026-0384, patched ">=1.54.1".
   - **Why it matters:** with one room per process (R2-ROOM-REC2), one remote panic ends a whole 48-player match.
   - **What to add:** lockfile floors for these crates; cargo-audit and cargo-deny as CI gates; the sim thread never parses raw bytes; each connection's task is isolated, under the default `panic = "unwind"`.

2. **FMA that differs by target in Rapier's default 4-lane SIMD path, not switched off by enhanced-determinism.**
   - **wide 1.7.1 `mul_add`:** the branches are `all(target_feature="sse2",target_feature="fma")` and `all(target_feature="neon",target_arch="aarch64")`, with a plain fallback; there is no branch for Wasm. Its doc says: "Platform-specific behavior (may change in the future)" and "Without FMA support: Uses `(self * m) + a` (two roundings)".
   - **Effect:** fused on AArch64 servers and on x86 built with +fma, unfused on wasm32.
   - **simba 0.10.2:** `simd_mul_add` is `$WideF32xX(self.0.mul_add(a.0, b.0))`, and `libm_force` only switches sin and cos.
   - **Rapier's own hint:** its Cargo.toml suggests `-C target-cpu=native`.
   - **Not established:** that the solver actually reaches this code. The two solver files I checked do not call mul_add.
   - **Signed zero:** wide's `max`/`min` also differ at ±0 between SSE, NEON and the fallback.
   - **What to add:** build the server for baseline x86-64 (no +fma, no target-cpu=native); add native AArch64 to the native-vs-Wasm hash test in DET-REC3 before R6 may pick ARM hosts; if the sim uses glam itself, use `scalar-math`, as parry does.

3. **Allocations a client controls.**
   - **WebSocket fallback:** tungstenite and axum default to max_message_size "64 MiB" and max_frame_size "16 MiB", and max_write_buffer_size "The default value is unlimited."
   - **quinn:** the docs state no defaults for the receive windows or the datagram receive buffer, and R2 sets none.
   - **Queues:** R2 does not say the per-client queues are bounded.
   - **Content:** R1's content pipeline caps file bytes but not array lengths.
   - **What to add:** caps of a few KiB per message; explicit quinn windows and stream counts; bounded per-client channels; length rules on collections.

4. **Panics reachable from network input and content.**
   - R2 plans no fuzzing of its decoders.
   - wasm32 builds with "-Cpanic=abort", so one panic kills the client sim's instance; R1 plans no recovery.
   - `f32::clamp` panics when its bounds come from bad data.
   - **What to add:** decoders return Result and are fuzzed (cargo-fuzz) with zero panics as the pass mark; the Worker restarts from the server's snapshot after a trap.

5. **The snapshot allocation target in ECS-REC1 is not reachable as written.**
   - `#[derive(Clone)]` only implements `clone`; the default `clone_from` is `*self = source.clone()`.
   - Rapier's `RigidBodySet::clone_from` is that same default.
   - So ECS-REC1 mark 2 ("0 heap allocations after warm-up", Rapier included) needs hand-written `clone_from` for our tables. It must also accept, or work around, Rapier allocating on every clone.

6. **Overflow behaviour differs between build profiles.**
   - Cargo defaults: `[profile.dev] overflow-checks = true`, `[profile.release] overflow-checks = false`. A debug client and a release server can diverge, or one of them panics.
   - Turning the checks on globally exposes dependencies to the ring-style panic in risk 1.
   - **What to add:** enable overflow-checks for the sim package only, use explicit wrapping or checked arithmetic, and add a clippy lint for it.

7. **Wasm size and speed have no budget.**
   - R1 records the size but sets no absolute limit.
   - The content crate compiled into the Worker brings serde_json, garde, and regex for pattern rules.
   - std `mul_add` on wasm32 has no hardware instruction: compiler-builtins supplies a software `fma` (inferred cost). DET-REC2 allows explicit `mul_add`, so measure it.
   - **What to add:** wasm-opt, `opt-level` "s" against "z" ("Always measure!"), and the panic strategy, measured before and after.

8. **Hashing state.**
   - `repr(C)` padding bytes are uninitialised, so the hash must read fields, never raw struct bytes.
   - Writing the output block safely needs a Pod derive that rejects padding, or `unsafe` with a written invariant and a test.

9. **rand distributions are not value-stable.**
   - Uniform floats "may equal `high`", and the docs make no stability statement across versions.
   - The sim should map raw ChaCha8 output itself and pin rand_chacha with test vectors.

10. **Duplicate dependency versions.**
   - axum 0.8.9 pulls tokio-tungstenite ^0.29, while REC3 names 0.30.x.
   - Keep wtransport's `dangerous-configuration` feature off.
   - The async model fits: sim on its own thread and I/O on tokio, as the spawn_blocking page advises.

**Corrections to make in the docs**
- **R1:**
  - DET-F25, DET-REC5 and DET-REC6: say "per instance", not "per process".
  - BND-F2: drop 0.2.122 from the breaking releases.
  - DET-F21: add the "determinism domain" sentence.
  - ECS-F30: name the `serde-serialize` feature.
  - Label upgrades to "confirmed": ECS-F15, F16 (order), F19 (condition), F20, F28 (indirection, serde); BND-F8, F9 (MSRV); ABL-F23 (Bevy ^0.19), F24, F25.
- **R2:**
  - ROOM-F7: the long-running-work link is now quoted ("confirmed").
  - TRN-F17: add quinn's minimum datagram size.

Notes: `/home/user/Shamble-Frontlines/.checkpoints/v2-research/rust-reviewer-r1-r2.md`. Nothing was committed and no other file was edited.