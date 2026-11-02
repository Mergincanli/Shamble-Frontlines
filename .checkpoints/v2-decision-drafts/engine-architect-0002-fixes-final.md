# ADR 0002: the final fix round (plan step 10's fix loop), engine-architect, 2026-11-02

Fixes against `docs/adr/0002-code-structure-build-toolchain.md` at 47eadd9 (unchanged through HEAD 208af0d). Every
BEFORE is copied by script (engine-architect-0002-fixes-final-gen.py) from that file by line range, occurs exactly
once there, and no two overlap; applying them in file order was checked. The "### Pass marks" section is not
touched (test-engineer's); what its rows must follow is in the NOTE blocks. Findings that touch the same lines
share one block, named by all their ids.

### FIX NA-7 (the summary)
FILE: docs/adr/0002-code-structure-build-toolchain.md
BEFORE:
above it, and a gate check proves that. In the browser the simulation runs in one Web Worker, which also runs the local
server from slice 1 and owns the network connection online; the page gets one block of data per tick and posts one
command per tick, so no call crosses the boundary per entity or per frame. TypeScript types are generated from the Rust
code. One command, the gate v2, runs the same checks locally and in CI and proves itself with planted defects; every
AFTER:
above it, and a gate check proves that. In the browser the simulation runs in one Web Worker, which also runs the local
server from slice 1 and owns the network connection online; the page gets one block of data per tick, posts its newest
input once per drawn frame and never calls into Wasm, so nothing crosses the boundary per entity. TypeScript types are
generated from the Rust code. One command, the gate v2, runs the same checks locally and in CI and proves itself with
planted defects; every
END

### FIX NA-7 (Decision, opening)
FILE: docs/adr/0002-code-structure-build-toolchain.md
BEFORE:
module Web Worker that also runs the local server and owns the network connection; the page and the Worker exchange one
command per tick one way and one transferred block per tick the other way, so nothing crosses per entity or per frame.
AFTER:
module Web Worker that also runs the local server and owns the network connection; the page posts its newest input to
the Worker once per drawn frame and the Worker sends the page one transferred block per tick, so nothing crosses per
entity and the page never calls into Wasm.
END

### FIX RR-15 (part 1)
FILE: docs/adr/0002-code-structure-build-toolchain.md
BEFORE:
   `fuzz/` (outside the workspace's default members, because cargo-fuzz needs a nightly compiler), and `docs/`.
AFTER:
   `fuzz/` (outside the workspace's default members, because cargo-fuzz needs a nightly compiler: R5-TEST-F4, r5 lines
   430–434; it carries its own `rust-toolchain.toml` that pins one dated nightly, a channel form R5-CI-F12 quotes, r5
   lines 632–639, so the fuzz jobs' compiler is pinned like every other: part 2), and `docs/`.
END

### FIX RR-15 (part 2)
FILE: docs/adr/0002-code-structure-build-toolchain.md
BEFORE:
   archived or handed to their maintainers (R1-BND-F1; wasm-pack's own pages were not read). Each toolchain bump
   (Rust, clippy and rustfmt, Node, TypeScript, wasm-bindgen) is its own change-set that reports the new warnings.
AFTER:
   archived or handed to their maintainers (R1-BND-F1; wasm-pack's own pages were not read). Each toolchain bump
   (Rust, clippy and rustfmt, the fuzz jobs' dated nightly, Node, TypeScript, wasm-bindgen) is its own change-set that
   reports the new warnings.
END

### FIX EA-8, RR-2, EA-12 (part 3, the crate table)
FILE: docs/adr/0002-code-structure-build-toolchain.md
BEFORE:
   | `room` | the room's match logic with no I/O and no async runtime: the slot table, bots filling slots through the input path, per-client command queues, per-client snapshot and event encoding through `protocol` (ADR 0001 decides what it does) | `sim`, `protocol`, `content` | the foundation | §6.1; SLICE-ORDER condition 1 |
   | `sim-wasm` | the wasm-bindgen bindings the Worker loads: the client endpoint (prediction, snapshot decoding, the block writer) and the local server (`room` + `sim`) | `sim`, `room`, `protocol`, `content` | the foundation | §6.2, §6.9 |
   | `server` | the room process: transport on quinn and the WebSocket fallback, the async runtime, the supervisor hook, metrics (ADR 0001, ADR 0006) | `room`, `protocol`, `sim`, `content` | slice 3 (online) | §6.1, §6.5, §6.6, §6.8 |
   | `devtools` | the replay runner, the hash comparer, the tick bench, the legacy-golden reader | anything | the foundation | §6.6, §6.8 |
AFTER:
   | `room` | the room's match logic with no I/O, no async runtime and no clock (its timers count ticks): the slot table, bots filling slots through the input path, per-client command queues, per-client snapshot and event encoding through `protocol` (ADR 0001 decides what it does) | `sim`, `protocol`, `content` | the foundation | §6.1; SLICE-ORDER condition 1 |
   | `client-endpoint` | the client endpoint, with no bindings, no transport, no async runtime and no clock: prediction and reconciliation, the interpolation buffer, snapshot decoding through `protocol`, the aim cast, the block writer and the block's layout table (part 5); built for Wasm inside `sim-wasm` and natively for the headless clients of part 8 | `sim`, `protocol`, `content` | the foundation | §6.2, §6.9 and §4, as `sim-wasm`, whose code it holds; §3, §6.1 and §6.6 through the headless clients (R5-TEST-REC4, REC5) |
   | `sim-wasm` | the wasm-bindgen bindings the Worker loads, and nothing else: around the client endpoint (`client-endpoint`) and the local server (`room` + `sim`) | `client-endpoint`, `room`, `sim`, `protocol`, `content` | the foundation | §6.2, §6.9 |
   | `server` | the room process: transport on quinn and the WebSocket fallback, the async runtime, the supervisor hook, metrics (ADR 0001, ADR 0006) | `room`, `protocol`, `sim`, `content` | slice 3 (online) | §6.1, §6.5, §6.6, §6.8 |
   | `supervisor` | the small supervisor on every game VM (R6-HOST-REC2, r6-infrastructure.md lines 645–647; ADR 0006 part 5): it starts, watches and ends room processes; what else it does is ADR 0006's | what ADR 0006 gives it; it runs room processes as programs and links no `sim`, `room` or `sim-wasm` code | slice 3 (online) | §6.1 |
   | `devtools` | the replay runner, the hash comparer, the tick bench, the legacy-golden reader, the headless protocol clients of the network and soak layers (part 8) | anything | the foundation | §6.6, §6.8; the headless clients: §3, §6.1 (R5-TEST-REC5) |
END

### FIX EA-12 (part 3, the lobby's crate)
FILE: docs/adr/0002-code-structure-build-toolchain.md
BEFORE:
   (Playwright tests). The lobby and the room supervisor get a crate when ADR 0006 decides where the lobby runs
   (OQ-0006-3: the supervisor's binary or its own service); until then no crate is created for them.
AFTER:
   (Playwright tests). The lobby gets a crate when ADR 0006 decides where it runs (OQ-0006-3: inside the supervisor's
   binary or as its own service); until then no lobby crate is created.
END

### FIX EA-8, RR-2, RR-10 (part 3, the rules)
FILE: docs/adr/0002-code-structure-build-toolchain.md
BEFORE:
   The rules, each a gate check (G5, G8, part 9): `sim` depends on `content` only, with no transport, no async runtime
   and no clock, checked with cargo-deny's `deny` plus `wrappers` and with `cargo tree`; `room` builds for
   `wasm32-unknown-unknown` with no async runtime and no transport crate (the same two tools); `content` depends on
   nothing above it; `sim-wasm` never depends on `navbake` or `server`, so the player's Wasm carries neither the bake
   nor the server's I/O (`cargo tree` on the `sim-wasm` build); `client` never reaches `editor`, checked twice: an
   ESLint `no-restricted-imports` rule and a walk of Vite's manifest from the player entry through `imports` and
   `dynamicImports`; `sim-worker` and `net` never import rendering or DOM packages; nothing imports `legacy/` or
   `spikes/`. `sim` is built with one feature set on every target: it exposes no feature that changes maths, the
   maths-relevant features of the dependencies it shares with `server` are set once in `[workspace.dependencies]`,
   and the gate compares the resolved feature list of every `sim` dependency (`cargo tree -e features`) between the
   `server` build and the `sim-wasm` build.
AFTER:
   The rules, each a gate check (G5, G8, part 9): `sim` depends on `content` only, with no transport, no async runtime
   and no clock, checked with cargo-deny's `deny` plus `wrappers` and with `cargo tree`; `room` and `client-endpoint`
   build for `wasm32-unknown-unknown` and natively, with no async runtime, no transport crate and no wasm-bindgen (the
   same two tools), and read no clock (part 7's clock ban); `content` depends on
   nothing above it; `sim-wasm` never depends on `navbake` or `server`, so the player's Wasm carries neither the bake
   nor the server's I/O (`cargo tree` on the `sim-wasm` build); `client` never reaches `editor`, checked twice: an
   ESLint `no-restricted-imports` rule and a walk of Vite's manifest from the player entry through `imports` and
   `dynamicImports`; `sim-worker` and `net` never import rendering or DOM packages; nothing imports `legacy/` or
   `spikes/`. `sim` is built with one feature set on every target: it exposes no feature that changes maths, the
   maths-relevant features of the dependencies it shares with `server` are set once in `[workspace.dependencies]`,
   and the gate compares the resolved feature list of every `sim` dependency (`cargo tree -e features`) across every
   build that runs the sim (`server`, `devtools`, `navbake` and `sim-wasm`), each made by the exact command CI and the
   release use, since Cargo unifies features over everything one command builds (R5-LAYOUT-F4).
END

### FIX EA-8 (part 3, the change from R5-LAYOUT-REC2)
FILE: docs/adr/0002-code-structure-build-toolchain.md
BEFORE:
   - Changed from R5-LAYOUT-REC2 (r5 lines 1170–1193): two crates are added and none removed. `room`, because
     SLICE-ORDER condition 1 runs the local server in the Worker, and the Worker cannot carry the server's async
     runtime and transport (on the default Wasm target `std::thread::spawn` panics: R1-BND-F19, r1 lines 1019–1025;
     the simulation depends on no I/O: R2-TRN-REC4, r2-netcode.md lines 2673–2698); R5-LAYOUT-REC2 predates
     SLICE-ORDER and puts rooms inside `server`. `navbake`, because where the bake lives decides whether the generator
     ships in the player's Wasm (the critic's R4-XT-OQ5, r4-physics-navigation.md lines 1195–1200), and the bake runs in
     CI, on map save and in the editor's Test (R4-NAV-REC1, r4 lines 802–810), never in prediction (R4-NAV-REC4, r4
     lines 888–899). Both placements answer questions the critic gave this record; no research compared them with
     other placements. Both are proposed (not researched): no research Options section offers a crate for the room
     logic or the bake, and SLICE-ORDER condition 1 and R4-XT-OQ5 leave them to this record; whether plan rule 3
     allows that is OQ-0002-8, for the Advisor at the stop.
AFTER:
   - Changed from R5-LAYOUT-REC2 (r5 lines 1170–1193): three crates are added and none removed. `room`, because
     SLICE-ORDER condition 1 runs the local server in the Worker, and the Worker cannot carry the server's async
     runtime and transport (on the default Wasm target `std::thread::spawn` panics: R1-BND-F19, r1 lines 1019–1025;
     the simulation depends on no I/O: R2-TRN-REC4, r2-netcode.md lines 2673–2698); R5-LAYOUT-REC2 predates
     SLICE-ORDER and puts rooms inside `server`. `navbake`, because where the bake lives decides whether the generator
     ships in the player's Wasm (the critic's R4-XT-OQ5, r4-physics-navigation.md lines 1195–1200), and the bake runs in
     CI, on map save and in the editor's Test (R4-NAV-REC1, r4 lines 802–810), never in prediction (R4-NAV-REC4, r4
     lines 888–899). `client-endpoint`, because ADR 0001's network marks run the client endpoint's Rust code natively
     and headless, and so do the soak and load layers (part 8), while R5-LAYOUT-REC2 names no crate for that code but
     the bindings crate: the headless clients would otherwise need a native build of a bindings crate or a second copy
     of the endpoint. The code is the same; only its crate is new. The first two answer questions the critic gave this
     record and the third follows from ADR 0001's headless tests; no research compared them with other placements. All
     three are proposed (not researched): no research Options section offers a crate for the room logic, the client
     endpoint or the bake, and SLICE-ORDER condition 1 and R4-XT-OQ5 leave them to this record; whether plan rule 3
     allows that is OQ-0002-8, for the Advisor at the stop. The supervisor's crate (slice 3) is beyond R5's list too,
     but the research asks for it: R6-HOST-REC2 puts a small supervisor on every VM (r6-infrastructure.md lines
     645–647), which ADR 0006 part 5 adopts.
END

### FIX RR-7, EA-8, routed-b815c85 (part 4, the trap's message and the build-flag check)
FILE: docs/adr/0002-code-structure-build-toolchain.md
BEFORE:
   the target's default `-Cpanic=abort` (R1-BND-F19), so a trap ends the sim's instance, and the restart after a trap
   is ADR 0004's (part 20) with ADR 0001's resync path. The server is built for baseline x86-64, never with `+fma`,
   `-C target-cpu=native` or the `relaxed-simd` target feature: the gate reads the build flags of `sim`, `room`,
   `sim-wasm` and `server` and fails if one appears (the rule is ADR 0004's, part 7; the check is this record's).
AFTER:
   the target's default `-Cpanic=abort` (R1-BND-F19), so a trap ends the sim's instance, and the restart after a trap
   is ADR 0004's (part 20) with ADR 0001's resync path. A trap carries no panic message, so the Wasm build's panic
   hook (ADR 0004 part 20, S20) writes the message and location into a fixed buffer in linear memory, and when the
   instance traps the Worker's host reads that buffer, through a fresh view of the memory (part 5), and attaches it to
   the trap's report. The server is built for baseline x86-64, never with `+fma`,
   `-C target-cpu=native` or the `relaxed-simd` target feature: the gate reads the build flags of `sim`, `room`,
   `client-endpoint`, `sim-wasm` and `server` and fails if one appears (the rule is ADR 0004's, part 7; the check is
   this record's). If wasm-opt is kept (R5-CI-OQ4), the check reads its command line too, because wasm-opt optimizes
   the shipped Wasm after rustc: it fails on binaryen's `--fast-math` (`-ffm`, "Optimize floats without handling
   corner cases of NaNs, rounding, and negative zero" in binaryen's own options, read by rust-reviewer at the step-10
   review; not in the research) and on any feature flag beyond the target's defaults (R1-BND-F19's list, r1 lines
   1019–1025), `relaxed-simd` above all.
END

### FIX EA-1, EA-12, RR-5 (part 5, the block)
FILE: docs/adr/0002-code-structure-build-toolchain.md
BEFORE:
   - **One flat, versioned output block per tick** (option B2): written by Rust into preallocated linear memory, copied
     once into a pooled `ArrayBuffer` and transferred to the page. It holds a header (tick, layout hash, counts per
     entity kind), aligned sections per entity kind (soldiers, vehicle parts, projectiles, structures, weapon pickups,
     which are sim entities under UPGRADE-SCOPE, ADR 0004 part 18), one section of owner-only data for the local player
     (own weapon state, money, upgrade tracks and offers: decision map I-21) and the event section for the tick (the
     events of ADR 0004 part 19: hits, deaths and kills, blasts, cosmetic shot events, purchases, uplink cache events,
     harvester deliveries and kills, structure kills and stage changes, tech tiers, upgrade picks and pickups, and the
     moments its moment phase emits; ADR 0001 sets their visibility). Online, remote entities are written at the
     interpolation delay from the Worker's interpolation buffer and the local player as predicted (ADR 0001 parts 4,
     6, 7); the fields of remote
     entities are ADR 0001's snapshot fields in this record's layout. The page binds typed-array views on the block
     with no copy, keeps the two newest blocks to interpolate between ticks (R1-TICK-REC5) and returns older ones to
     the pool. Whether a section is laid out per entity or per field is measured in the boundary spike (bytes per tick
     and copy time at N = 48); ADR 0003 reads typed arrays either way. The block writer serializes field by field
     (proposed: R1-DET-REC6's method for [the state hash](../research/GLOSSARY.md#state-hash), applied to the block;
     R1-RR-OQ6's decider, rust-reviewer, confirms at review), so struct padding never reaches the page and no `unsafe`
     is needed; if the spike's copy time says otherwise, R1-RR-OQ6's two alternatives (a derive that rejects padding,
     or `unsafe` with a written invariant and a test) are weighed then. The SharedArrayBuffer ring (B3) is the spike's
     measured alternative, never the sim's own shared Wasm memory (B4). At the 30 Hz baseline that is 30 blocks a second
     (one per tick).
AFTER:
   - **One flat, versioned output block per tick** (option B2): written by Rust into preallocated linear memory, copied
     once into a pooled `ArrayBuffer` and transferred to the page. It holds a header (tick, layout hash, counts per
     entity kind), aligned sections per entity kind (soldiers, vehicle parts, projectiles, structures, weapon pickups,
     which are sim entities under UPGRADE-SCOPE, ADR 0004 part 18), one section of owner-only data for the local player
     (own weapon state, money, upgrade tracks and offers: decision map I-21; the floor layer and enclosure area id the
     roof fade needs: ADR 0004 part 14) and the event section (the
     events of ADR 0004 part 19: hits, deaths and kills, blasts, cosmetic shot events, purchases, uplink cache events,
     harvester deliveries and kills, structure kills and stage changes, tech tiers, upgrade picks and pickups, and the
     moments its moment phase emits; ADR 0001 sets their visibility). Online, remote entities are written at the
     interpolation delay from the Worker's interpolation buffer and the local player as predicted (ADR 0001 parts 4,
     6, 7); the fields of remote
     entities are ADR 0001's snapshot fields in this record's layout. The event section holds the events due at that
     block, each on the timeline of the entity it belongs to: the Worker releases a remote entity's spatial event (an
     action start, a death, a blast on the entity it hit) when its interpolation clock reaches the event's tick, and
     the local player's own, predicted and owner events at once; a local match does the same, since its client
     endpoint is the same code (the rule is ADR 0001 part 7's; r2-netcode.md, Recommendations, Brief question 1,
     R2-SYNC-REC6, line 2210: blasts "played on the timeline of the entity they hit"). The page binds typed-array views
     on the block with no copy, keeps the two newest blocks to interpolate between ticks (R1-TICK-REC5) and returns
     older ones to the pool. Whether a section is laid out per entity or per field is measured in the boundary spike
     (bytes per tick and copy time at N = 48); ADR 0003 reads typed arrays either way. The block writer serializes
     field by field (R1-DET-REC6's method for [the state hash](../research/GLOSSARY.md#state-hash), applied to the
     block), so struct padding never reaches the page and no `unsafe` is needed. R1-RR-OQ6's decider, rust-reviewer,
     confirmed this at the step-10 review on three conditions: one layout table in Rust drives the writer, the
     generated offsets and the layout hash, so the three cannot drift; every integer is written at a fixed width (never
     `usize`) and little-endian (`to_le_bytes`); each section is aligned to its largest element type, so the page's
     typed-array views bind with no copy. R1-RR-OQ6's two alternatives (a derive that rejects padding, or `unsafe` with
     a written invariant and a test) are weighed only if the writer's p99 in Wasm on the minimum laptop exceeds 5 % of
     the client's sim share at 30 Hz (0.05 × 13.33 = 0.67 ms; proposed by rust-reviewer: below that the writer cannot
     decide that mark). The SharedArrayBuffer ring (B3) is the spike's
     measured alternative, never the sim's own shared Wasm memory (B4). At the 30 Hz baseline that is 30 blocks a second
     (one per tick).
END

### FIX NA-7 (part 5, the input)
FILE: docs/adr/0002-code-structure-build-toolchain.md
BEFORE:
   - **One command per tick, posted** from the page to the Worker (option C1): a sequence number, the move input, the
     held-button bitmask and press and release edge counters (their exact fields are ADR 0001's), plus the camera's
     cursor ray and the view time the page drew; the Worker casts that ray against the state it published and fills the
     network command's aim point (ADR 0001 part 5; decision map I-19), so the aim needs no second message. The Worker
     stamps the tick it applies the command to.
AFTER:
   - **The newest input, posted once per drawn frame,** from the page to the Worker (option C1, changed in its cadence):
     a sequence number, the move input, the held-button bitmask and press and release edge counters kept as running
     totals (their exact fields are ADR 0001's), plus the camera's cursor ray and the view time of the frame the page
     drew. At each tick the Worker takes the newest post (the running totals mean a superseded post loses no press),
     casts its ray against the state it published for that view time and fills the tick's one network command, aim
     point included (ADR 0001 part 5; decision map I-19), so the aim needs no second message. The Worker stamps the
     tick it applies the input to. Why per frame: the page has no tick clock (its only tick signal is the block), so a
     post per block can leave a press waiting up to two tick periods less the block's delivery before a tick applies it
     (2 × 1000 / 30 = 66.67 ms at 30 Hz), against R1-BND-REC3 mark 2's one tick plus one frame (33.33 + 16.67 =
     50.00 ms with a 60 Hz display; r1-simulation-core.md, Recommendations, unit boundary-worker, R1-BND-REC3, lines
     2286–2296); a post per frame bounds the wait by one frame plus one tick plus the message's delivery, and the
     page's work stays one small message per frame.
END

### FIX NA-4, RR-2, RR-6, NA-8, routed-b815c85 (part 5, the control API)
FILE: docs/adr/0002-code-structure-build-toolchain.md
BEFORE:
   - **The small control API** (create, mode flag, load the content bundle, one input call per tick, and the step) is
     wasm-bindgen's own, with its `.d.ts` (option D1). Each tick the Worker's host makes one input call, which carries
     the page's command and every datagram received since the last tick, and one step call; the step leaves the
     outgoing datagrams and the block in linear memory and returns where they lie. The Worker's Wasm calls per tick,
     wasm-bindgen's glue calls included, are meant to stay within R1-BND-REC1 mark 2's four, which the spike counts
     (item 1); none is made per entity.
AFTER:
   - **The small control API** (create, mode flag, load the content bundle, one input call per tick, and the step) is
     wasm-bindgen's own, with its `.d.ts` (option D1). Each tick the Worker's host makes one input call and one step
     call. The input call carries the page's newest input; every datagram received since the last tick, each with the
     time it arrived (the host's `performance.now()` in that datagram's own message event, never the tick's time); the
     time the host sent the previous step's datagrams; and the host's current time, all as plain values, since Rust
     reads no clock of its own (part 7). Without arrival times every clock and jitter sample of ADR 0001 parts 5 and 7
     would carry up to one tick period of false jitter (0 to 1000 / 30 = 33.33 ms at 30 Hz); those samples use the
     stamps, and how a ping answered at the next tick reports its hold time is ADR 0001's (part 5). The step leaves the
     outgoing datagrams and the block in linear memory and returns where they lie and when the next tick is due: in
     local play from the plain accumulator, online from the client endpoint's server-time estimate, since online the
     Worker's tick runs ahead of the server's by ADR 0001's rule for keeping commands just in time (OQ-0001-6). The
     host sets its timer from that, never from a count of its own, and the catch-up it allows is counted per wake of
     the Worker, at most 7 ticks (ADR 0004 part 5, S18). The host reads the step's output through a fresh view of the
     instance's `memory.buffer` after every call, never a cached one, since growth makes an old view stale (proposed:
     R1-BND-REC2 names re-binding on growth as a cost of B4, r1 lines 2273–2274). The Worker's Wasm calls per tick,
     wasm-bindgen's glue calls included, are meant to stay within R1-BND-REC1 mark 2's four, which the spike counts
     (item 1); none is made per entity.
END

### FIX RR-16 (part 5, generated types)
FILE: docs/adr/0002-code-structure-build-toolchain.md
BEFORE:
   - **Generated types, from three sources, into one committed package** `packages/types`: wasm-bindgen's `.d.ts` and
     glue are build output (ignored); our own small generator writes the block's layout (offsets, strides, element
     types) and its layout hash, which the page checks at startup (option D4); ts-rs writes the protocol and message
     types into `packages/types/src/generated/` on `cargo test`, through `TS_RS_EXPORT_DIR` in `.cargo/config.toml`
     (option D3); content's types land in the same package, from the source ADR 0005 picks (critic rank 31). Not tsify.
     The gate regenerates all of them and fails on any difference or hand edit (G4).
AFTER:
   - **Generated types, from three sources, into one committed package** `packages/types`: wasm-bindgen's `.d.ts` and
     glue are build output (ignored); our own small generator writes the block's layout (offsets, strides, element
     types) and its layout hash from the layout table the writer uses (above), and the page checks the hash at startup
     (option D4); ts-rs writes the protocol and message
     types into `packages/types/src/generated/` on `cargo test`, through `TS_RS_EXPORT_DIR` in `.cargo/config.toml`
     (option D3); content's types land in the same package, from the source ADR 0005 picks (critic rank 31). Not tsify.
     The generators' derives never reach a shipped build: ts-rs is a dev-dependency, and its derives sit behind
     `cfg_attr(test, …)` or, where one crate's types nest another's, behind a feature that only dev-dependency entries
     turn on ("Dev-dependencies do not activate features unless building a Cargo target that needs them": R5-LAYOUT-F4,
     r5 lines 83–91), so the builds part 3 compares and G7 measures are the shipped ones (proposed by rust-reviewer at
     the step-10 review); content's schema generator follows the same rule (ADR 0005, CD-3).
     The gate regenerates all of them and fails on any difference or hand edit (G4).
END

### FIX NA-7 (part 5, why)
FILE: docs/adr/0002-code-structure-build-toolchain.md
BEFORE:
   - Why: the tick leaves the render thread; one bounded copy per tick needs neither isolation nor nightly Rust; one
     small message per tick is constant work; the block's layout is the only thing tsify and ts-rs cannot type.
AFTER:
   - Why: the tick leaves the render thread; one bounded copy per tick needs neither isolation nor nightly Rust; one
     small message per frame is constant work and keeps a press within one tick plus one frame of the tick that applies
     it; the block's layout is the only thing tsify and ts-rs cannot type.
END

### FIX NA-15, NA-7, PA-13, routed-b815c85 (part 6)
FILE: docs/adr/0002-code-structure-build-toolchain.md
BEFORE:
   running, is measured in the boundary spike (OQ-0002-3). The Worker's host (TypeScript) receives the page's commands, paces the tick (ADR 0004 part 5), steps
   the instance or instances, and posts each block; the commands it forwards pass through it, outside the Wasm
   instance, which is what OQ-0004-7's recovery options need to know. The foundation's skeleton ends with an empty
   scene drawn from blocks that came through this whole path (Migration plan, phase 2).
AFTER:
   running, is measured in the boundary spike (OQ-0002-3). The Worker's host (TypeScript) receives the page's inputs,
   paces the tick (ADR 0004 part 5), steps the instance or instances, and posts each block; the inputs it forwards
   pass through it, outside the Wasm instance, which is what OQ-0004-7's recovery options need to know. In a local
   match one Worker step runs in this order: the client endpoint builds tick T's command from the newest input (its
   aim cast included) and predicts T with the sim's prediction step (ADR 0004 part 3); the command reaches the local
   server through the loopback; the local server runs tick T with the authority step and every slot's command (the
   bots' through the same input path) and encodes T's snapshot and events;
   the client endpoint decodes them, reconciles and writes the block. So the command for tick T reaches the local
   server before it runs T, and a local input never waits an extra step (the spike's item 3); with two instances
   (OQ-0002-3 (b)) the host makes its calls in the same order. The Worker as a whole has no budget in the research:
   the client's sim share covers the sim step only (GLOSSARY, the client's sim share, lines 214–218), so the boundary
   spike reports the Worker's busy time per tick in both modes and its JS heap and Wasm memory over a match (item 6),
   and this record sets any limit on them from those numbers. The foundation's skeleton ends with an empty scene drawn
   from blocks that came through this whole path (Migration plan, phase 2).
END

### FIX RR-1, RR-2, RR-14 (part 7, Rust)
FILE: docs/adr/0002-code-structure-build-toolchain.md
BEFORE:
   - **Rust:** one workspace lint table every crate opts into (`[lints] workspace = true`, itself a gate check):
     `unsafe_code = "deny"`, clippy's `all` at warn, and the gate runs `cargo clippy --workspace --all-targets --locked
     -- -D warnings` once for the host and once for `wasm32-unknown-unknown`; `pedantic` at warn on `sim`, `protocol`
     and `content` only. [The hash-map ban](../research/GLOSSARY.md#the-hash-map-ban-in-the-sim-crate) and the clock and
     float bans live in `crates/sim/clippy.toml` (`disallowed-types`: std `HashMap`, `HashSet`, `Instant`,
     `SystemTime`; `disallowed-methods`: `Instant::now`, `SystemTime::now`, std's transcendental float methods and the
     `algebraic_*` methods), at deny, with `iter_over_hash_type` as a backstop; `suboptimal_flops` and
     `imprecise_flops` are allowed with a comment, and `cargo clippy --fix` never runs on `sim`; the same set covers the
     parts of `content` the sim calls. The only escape from
     the ban is a reviewed `#[allow]` with a written reason for a map that is only looked up and lies outside the
     serialized sim state (resolution 29). Map types from the sim's dependencies join `disallowed-types` from a list
     rust-reviewer makes from `cargo tree` of `sim` once ADR 0004's dependencies are fixed. If clippy cannot name std's
     float methods, a reviewed text scan of `sim` for those method names is the fallback check (R5-RUSTLINT-OQ1). In
     `protocol`'s decoders and `content`'s loader, `unwrap_used`, `expect_used` and `indexing_slicing` are at deny,
     allowed in tests; the sim does not enable them, since its indices come from its own pools. rustfmt runs with stable
     options only (`edition` and `style_edition` 2024, `newline_style = "Unix"`), in check mode in the gate. The
     `panic` and `arithmetic_side_effects` lints are not added: their sources were not read.
AFTER:
   - **Rust:** one workspace lint table every crate opts into (`[lints] workspace = true`, itself a gate check):
     `unsafe_code = "deny"`, clippy's `all` at warn, and the gate runs `cargo clippy --workspace --all-targets --locked
     -- -D warnings` once for the host and once for `wasm32-unknown-unknown`; `pedantic` at warn on `sim`, `protocol`
     and `content` only. [The hash-map ban](../research/GLOSSARY.md#the-hash-map-ban-in-the-sim-crate) and the clock and
     float bans live in `crates/sim/clippy.toml` (`disallowed-types`: std `HashMap`, `HashSet`, `Instant`,
     `SystemTime`; `disallowed-methods`: `Instant::now`, `SystemTime::now`, the `algebraic_*` methods and every std
     `f32` and `f64` method outside ADR 0004 part 7's allow-list and the exact methods the rules themselves use, such as
     `is_finite`, `is_nan`, `to_bits` and `total_cmp`), at deny, with `iter_over_hash_type` as a backstop. The float
     ban is written as the allow-list's complement because ADR 0004 part 7 is an allow-list (R1-DET-REC2,
     r1-simulation-core.md, Recommendations, lines 2146–2150): it names every other method on the pinned std's `f32`
     and `f64` pages and is read again at every Rust bump (part 2). So it catches more than the transcendental
     functions: `powi`, `hypot` and `cbrt` carry std's note that their precision varies by platform and Rust version
     (R1-DET-F9, r1 lines 648–654, inferred there; rust-reviewer confirmed the note for these three and for
     `to_degrees` and `to_radians` on std 1.99.1's `f32` page at the step-10 review), and `min`, `max` and `clamp` may
     return either zero for equal signed zeros, while `clamp` panics on bad bounds (R1-DET-F14, r1 lines 679–683). The
     sim uses three helpers of its own for those three, written as plain comparisons, with one defined result for
     equal zeros and no panic (proposed). `suboptimal_flops` and
     `imprecise_flops` are allowed with a comment, and `cargo clippy --fix` never runs on `sim`; the same set covers the
     parts of `content` the sim calls. The only escape from
     the hash-map ban is a reviewed `#[allow]` with a written reason for a map that is only looked up and lies outside
     the serialized sim state (resolution 29). Map types from the sim's dependencies join `disallowed-types` from a list
     rust-reviewer makes from `cargo tree` of `sim` once ADR 0004's dependencies are fixed. If clippy cannot name std's
     float methods, a reviewed text scan of `sim` for those method names is the fallback check (R5-RUSTLINT-OQ1). The
     clock ban (`Instant::now` and `SystemTime::now` in `disallowed-methods`, at deny) also lives in the `clippy.toml`
     of `room`, `protocol`, `client-endpoint` and `sim-wasm`, for two reasons: these crates run in the Worker, where
     std's clock calls panic on `wasm32-unknown-unknown` (std's own source, read by rust-reviewer at the step-10
     review; not in the research) and the target's `-Cpanic=abort` makes that panic a trap (R1-BND-F19, r1 lines
     1019–1025), which native tests never see; and a timer counted in ticks replays exactly, as SLICE-ORDER's
     replayable matches need. So room timers count ticks, with their lengths in data; the client endpoint takes time
     only as the values the host passes in (part 5); only `server` reads `Instant`. Which timers are tick-counted in
     `room` and which run on wall time in `server` (such as the join deadline and the per-connection rate limits) is
     ADR 0001's (parts 5 and 11). In
     `protocol`'s decoders and `content`'s loader, `unwrap_used`, `expect_used` and `indexing_slicing` are at deny,
     allowed in tests. In the sim they are at deny in the module that applies commands, where values a client or
     content can set enter (an offer and its choice, a purchase's unit id, a team or slot setting), so those values
     reach the rest of the sim only through checked lookups (ADR 0004 part 20; PM-0004-60 fuzzes it); elsewhere the
     sim's indices come from its own pools. rustfmt runs with stable
     options only (`edition` and `style_edition` 2024, `newline_style = "Unix"`), in check mode in the gate. The
     `panic` and `arithmetic_side_effects` lints are not added: their sources were not read.
END

### FIX RR-7 (part 8, the determinism suite's tools)
FILE: docs/adr/0002-code-structure-build-toolchain.md
BEFORE:
     two processes and once as Wasm in Node; Playwright opens a replay page that runs the `sim-wasm` Worker in Chromium,
     Firefox and WebKit and returns the hash stream; a comparer reports the first mismatching tick and a field-level
AFTER:
     two processes and once as Wasm in Node; Playwright opens a replay page that runs the `sim-wasm` Worker in Chromium,
     Firefox and WebKit and returns the hash stream; every Wasm run uses the exact file G6 ships and G7 measures (after
     wasm-bindgen and wasm-opt, checked by its hash); a comparer reports the first mismatching tick and a field-level
END

### FIX TE-7, EA-8 (part 8, soak and load)
FILE: docs/adr/0002-code-structure-build-toolchain.md
BEFORE:
   - **Soak and load:** bots as headless Rust clients that speak the real protocol; one nightly run per mode of
     [the test-mode list](../research/GLOSSARY.md#the-test-mode-list) over L1 at P2, 30 minutes each, and one L2 run with
     4 real browser clients among the bots; measured: [server tick time](../research/GLOSSARY.md#server-tick-time) p50,
     [p99](../research/GLOSSARY.md#time-statistics) and max against [the server tick budget](../research/GLOSSARY.md#the-server-tick-budget) (ADR 0001's), bytes per
     client per second, the resident-memory slope, queue lengths, desyncs, panics and hangs, plus legacy TL13's match
     metrics; before each playtest one 3-hour soak.
AFTER:
   - **Soak and load:** bots as headless Rust clients that speak the real protocol (on `client-endpoint`, part 3); one
     nightly run per mode of
     [the test-mode list](../research/GLOSSARY.md#the-test-mode-list) over L1 at P2, 30 minutes each, and one L2 run with
     4 real browser clients among the bots. On CI's shared runners a run is judged only by what needs no particular
     machine: desyncs, panics and hangs, the resident-memory slope, queue lengths, bytes per client per second, plus
     legacy TL13's match metrics; [server tick time](../research/GLOSSARY.md#server-tick-time) p50,
     [p99](../research/GLOSSARY.md#time-statistics) and max is reported there, and judged against
     [the server tick budget](../research/GLOSSARY.md#the-server-tick-budget) (ADR 0001's) only from a run on one vCPU
     of the reference server (PM-0002-72), since timings on shared runners are not hardware results. Before each
     playtest one 3-hour soak runs on the reference server, so its tick times count (its VM: the bench's path below).
END

### FIX IE-19 (part 8, flakes)
FILE: docs/adr/0002-code-structure-build-toolchain.md
BEFORE:
     through a test-hook API and drive page time with Playwright's clock; no shared global state in Rust tests; every
     failure keeps its seed, recording and trace as CI artefacts and gets an issue.
AFTER:
     through a test-hook API and drive page time with Playwright's clock; no shared global state in Rust tests; every
     failure keeps its seed, recording and trace as CI artefacts, for the retention ADR 0006 sets for them (part 3), and
     gets an issue that carries the seed, so the failure still replays after its artefacts expire.
END

### FIX IE-14 (part 8, the bench method)
FILE: docs/adr/0002-code-structure-build-toolchain.md
BEFORE:
     natively and in Wasm in the Worker on the minimum laptop, as R1-TICK-REC2's P1 and P2 ask. It reports; it never
     passes or fails the gate. No bench crate, coverage tool or cargo-nextest is adopted: none was read.
AFTER:
     natively and in Wasm in the Worker on the minimum laptop, as R1-TICK-REC2's P1 and P2 ask. It reports; it never
     passes or fails the gate. The bench needs the reference server from slice 1, before ADR 0006's fleet exists, and CI
     holds no token for it (timings on shared runners are not hardware results): how a session gets that VM for a bench
     run without reading a secret, whether it is made and deleted per run, and its cost line are ADR 0006's (part 8 and
     part 13, item B9); the 3-hour soak before each playtest uses the same path. No bench crate, coverage tool or
     cargo-nextest is adopted: none was read.
END

### FIX RR-10 (part 9, G5)
FILE: docs/adr/0002-code-structure-build-toolchain.md
BEFORE:
   | G5 | dependency rules | part 3's rules (cargo-deny bans and `cargo tree`); `multiple-versions` and `wildcards` at deny with a reviewed skip list; the same-feature check for `sim` |
AFTER:
   | G5 | dependency rules | part 3's rules (cargo-deny bans and `cargo tree`); `multiple-versions` and `wildcards` at deny with a reviewed skip list; the same-feature check for `sim` across `server`, `devtools`, `navbake` and `sim-wasm`, each built by the command CI and the release use |
END

### FIX RR-7 (part 9, G10)
FILE: docs/adr/0002-code-structure-build-toolchain.md
BEFORE:
   | G10 | determinism | the short recordings: native in two processes, Wasm in Node, Wasm in Chromium, Firefox and WebKit; first mismatch reported |
AFTER:
   | G10 | determinism | the short recordings: native in two processes, Wasm in Node, Wasm in Chromium, Firefox and WebKit, each Wasm run on the exact file G6 ships (after wasm-opt, checked by its hash); first mismatch reported |
END

### FIX GE-2 (part 9, WebGPU in CI)
FILE: docs/adr/0002-code-structure-build-toolchain.md
BEFORE:
   - **WebGPU in CI:** standard runners have no GPU and headless Chromium renders on SwiftShader, so CI gates only
     counts (ADR 0003, R3-MEASURE-REC3) on whatever backend each runner gives, prints that backend on every shot, and
     runs the end-to-end suite once more with three.js's `forceWebGL` so the WebGL 2 fallback is always covered; frame
     and GPU times come only from the reference machines (ADR 0003). No GPU runner is proposed (a paid larger runner;
     ADR 0006 and Mustafa for any paid plan).
AFTER:
   - **WebGPU in CI:** standard runners have no GPU and headless Chromium renders on SwiftShader, so CI gates only
     counts (ADR 0003, R3-MEASURE-REC3) on whatever backend each runner gives, prints that backend on every shot, and
     runs the end-to-end suite once more with three.js's `forceWebGL` so the WebGL 2 fallback is always covered; frame
     and GPU times come only from the reference machines (ADR 0003). By the research's own findings every CI browser
     probably gets no WebGPU adapter and falls back to WebGL 2: Chrome on Linux offers WebGPU only on Intel Gen12+ GPUs
     (R3-WGPU-F17, r3-rendering.md lines 159–162), Firefox not on Linux (R3-WGPU-F18, r3 lines 164–169), any browser
     with no adapter falls back (R3-GL2-F12, r3 lines 279–283), and headless Chrome renders on "SwiftShader Device
     (Subzero)" (R3-MEASURE-F8, r3 lines 648–653); Playwright's WebKit on Linux, and whether headless Firefox and
     WebKit get WebGL 2 at all, were not researched. Then the default run repeats the `forceWebGL` run and CI never
     runs the WebGPU backend, so a shader that fails to build for WebGPU, a WebGPU validation error or a pipeline made
     mid-match only on WebGPU would pass CI. The foundation therefore records, per CI engine, the backend it gets (or
     none). If none gets WebGPU, the boundary spike tries a Chromium job with SwiftShader's WebGPU adapter turned on by
     flags, for counts and errors only (not researched; R3-MEASURE-F8 names GPU flags only for hardware runs); until a
     CI route works, every change-set that touches the render path carries a WebGPU smoke run (console errors, ADR
     0003's counts, the backend string) on a reference machine of ADR 0003 as stated evidence. No GPU runner is
     proposed (a paid larger runner; ADR 0006 and Mustafa for any paid plan).
END

### FIX SA-1 (part 9, the Sentry upload job)
FILE: docs/adr/0002-code-structure-build-toolchain.md
BEFORE:
     trap runs in the boundary spike, not in the gate. The job that uploads debug files for Sentry (R6-OBS-REC3) runs
     only on builds that are deployed, in its own GitHub environment that holds the Sentry token, with no checkout of
     pull-request code, shaped like the preview deploy job (part 10); its token scope is ADR 0006's (part 13, item B9).
AFTER:
     trap runs in the boundary spike, not in the gate. The job that uploads debug files for Sentry (R6-OBS-REC3) runs
     only on builds that are deployed, in its own GitHub environment that holds the Sentry token, with no checkout of
     pull-request code, shaped like the deploy job it follows and under that job's guard (part 10: `production`'s
     `main`-only branch rule, or the guard of OQ-0002-9 for a preview); its token scope is ADR 0006's (part 13, item
     B9).
END

### FIX SA-1, IE-4 (part 10)
FILE: docs/adr/0002-code-structure-build-toolchain.md
BEFORE:
10. **The preview build.** Shape P1: the ordinary `pull_request` workflow builds with no secret and uploads `dist` as
    an artifact (kept 7 days, the retention ADR 0006 sets); a separate deploy job (`needs: build`) runs in a GitHub
    environment named `preview`, holds the only client-host token, never checks out or runs pull-request code, runs
    only for pull requests from this repository, has the permissions `contents: read`, `actions: read`,
    `deployments: write` and `pull-requests: write`, turns automatic package caching off, sets the environment's URL to
    the preview link and posts or updates one comment with it. Fork pull requests get the full CI and no preview. No
    `pull_request_target`, no `workflow_run`. The build generates the `_headers` file for
    [the client host](../research/GLOSSARY.md#the-client-host): [the production header set](../research/GLOSSARY.md#the-production-header-set-and-the-preview-header-set)
    for production and the preview header set for previews (`X-Robots-Tag: noindex`), with COOP and COEP only in a
    build made with the isolation flag on; after deploying, the job fetches the preview's HTML and `.wasm` and fails if
    a header is missing. The close job and the weekly sweep that delete previews are workflows here; the policy (24 h,
    7 days) and PREV-DELETE are ADR 0006's. **Multiplayer previews** (from slice 3; behind the access wall under
    PREVIEW-WALL (a)): CI's native job builds the room binary like any other artifact; a delivery job shaped like the
    deploy job, in its own environment, hands that binary to the staging game server's supervisor (ADR 0006) and
    receives its current development-certificate hash; the preview deploy job writes the hash into the preview's own
    files, which the page fetches from its own HTTPS origin (ADR 0001, part 10; R2-TRN-REC5). That shape is proposed:
    no research compared it with another; the delivery token's scope, and whether pull-request-built native code may run
    on the staging game server, wait for the security baseline (ADR 0006, part 13, item B9).
    - Why: a fork run gets no secrets, and the two triggers that hand them out are avoided; splitting build and deploy
      keeps the token away from every build script and dependency; deploying the artifact CI tested means the link shows
      the tested build.
AFTER:
10. **The preview build.** Shape P1: the ordinary `pull_request` workflow builds with no secret and uploads `dist` as
    an artifact (kept 7 days, the retention ADR 0006 sets); a separate deploy job (`needs: build`) runs in a GitHub
    environment named `preview`, holds the client-host token for previews, never checks out or runs pull-request
    code, runs only for pull requests from this repository, has the permissions `contents: read`, `actions: read`,
    `deployments: write` and `pull-requests: write`, turns automatic package caching off, sets the environment's URL to
    the preview link and posts or updates one comment with it. Fork pull requests get the full CI and no preview. No
    `pull_request_target`, no `workflow_run`. One gap remains: a `pull_request` run takes its workflow from the pull
    request's own merge commit (the contrast R5-PREV-F2 draws with `pull_request_target`, r5 lines 770–774; the
    security review's reading at step 10), so any branch of this repository, which agent sessions push, can rewrite
    the deploy job and use its token. That token is account-scoped (R5-PREV-F10, r5 lines 823–830), and the deploy
    command's `branch` option that finding quotes lets one deploy name any branch, so the token can probably reach
    production too (an inference; the client host's production rules were not read). What closes the gap until the
    security baseline rules on CI tokens (ADR 0006, part 13, item B9) is OQ-0002-9; its proposed interim is a required
    reviewer on `preview` ("a job cannot access environment secrets until one of the required reviewers approves it":
    R5-PREV-F6, r5 lines 796–804). **Production** is deployed only by its own job, on pushes to `main`, from the
    artifact `main`'s CI built and tested, in its own environment `production` that holds its own client-host token
    behind a deployment-branch rule admitting `main` alone ("Deployment branches and tags are available for all public
    repositories": R5-PREV-F6; the repository is public: r5 lines 752–754); when a release goes out beside running
    rooms is ADR 0006's. The build generates the `_headers` file for
    [the client host](../research/GLOSSARY.md#the-client-host): [the production header set](../research/GLOSSARY.md#the-production-header-set-and-the-preview-header-set)
    for production and the preview header set for previews (`X-Robots-Tag: noindex`), with COOP and COEP only in a
    build made with the isolation flag on; after deploying, the job fetches the preview's HTML and `.wasm` and fails if
    a header is missing. The close job and the weekly sweep that delete previews are workflows here; the policy (24 h,
    7 days) and PREV-DELETE are ADR 0006's. **Multiplayer previews** (from slice 3; behind the access wall under
    PREVIEW-WALL (a)): CI's native job builds the room binary like any other artifact; a delivery job shaped like the
    deploy job, in its own environment under the same guard as `preview` (OQ-0002-9), hands that binary to the staging
    game server's supervisor (ADR 0006). How the page trusts the staging server is OQ-0006-5's, and a certificate hash
    written into a preview's static files does not survive a restart: the staging server makes a new self-signed
    certificate at each start, valid at most 14 days (R2-TRN-REC5, r2-netcode.md, Recommendations, Brief question 4,
    lines 2700–2703), while R2-TRN-REC5 mark 2 asks that new connections succeed with the new hash within one reload
    (r2 lines 2720–2721), which a static file cannot give. So either the staging server presents, on both transports,
    the browser-trusted certificate OQ-0006-5 gives its `wss` endpoint (the kind R2-TRN-REC5 gives production, r2 line
    2704), and a multiplayer preview pins no hash, the delivery job then handing over only the binary; or, if staging
    keeps pinning, the supervisor keeps one certificate per VM across restarts and makes the next one ahead, and the
    preview's file, fetched from its own HTTPS origin (ADR 0001, part 10), lists both hashes. Both shapes are proposed,
    not researched; the delivery token's scope, and whether pull-request-built native code may run on the staging game
    server, wait for the security baseline (ADR 0006, part 13, item B9).
    - Why: a fork run gets no secrets, and the two triggers that hand them out are avoided; splitting build and deploy
      keeps the token away from every build script and dependency; deploying the artifact CI tested means the link shows
      the tested build; a production job on `main` alone keeps every release behind `main`'s review and gate, and the
      one path left to the preview token, a rewritten job on a branch, is what OQ-0002-9's guard closes.
END

### FIX EA-8 (Q4, the chosen row)
FILE: docs/adr/0002-code-structure-build-toolchain.md
BEFORE:
| **R5-LAYOUT-REC2 plus an I/O-free `room` crate and a `navbake` crate above `sim` (chosen)** | low–medium: two more crates and two more dependency checks | – | the player's Wasm carries neither the bake nor the server's I/O | the same decoders and room checks run locally and online | none | new | one room logic for the local server and the room process; the bake shared by CI, map save and the editor | not researched as a placement (no Options section offers a crate for the room logic or the bake; SLICE-ORDER condition 1 and R4-XT-OQ5 leave it to this record): proposed only for the Advisor's ruling at the stop (OQ-0002-8) |
AFTER:
| **R5-LAYOUT-REC2 plus an I/O-free `room` crate, a `client-endpoint` crate with no bindings and a `navbake` crate above `sim` (chosen)** | low–medium: three more crates and three more dependency checks | – | the player's Wasm carries neither the bake nor the server's I/O | the same decoders and room checks run locally and online | none | new | one room logic for the local server and the room process; one client endpoint for the Worker and the headless clients; the bake shared by CI, map save and the editor | not researched as a placement (no Options section offers a crate for the room logic, the client endpoint or the bake; SLICE-ORDER condition 1 and R4-XT-OQ5 leave it to this record): proposed only for the Advisor's ruling at the stop (OQ-0002-8) |
END

### FIX NA-7 (Q10, option C1)
FILE: docs/adr/0002-code-structure-build-toolchain.md
BEFORE:
| **C1. A small message per tick (chosen)** | low | one message per tick | constant work | – | edge counters keep a press from being lost or doubled (row GP2) | the never-wired command (row CS1) gets wired | works without isolation | one message per tick |
AFTER:
| **C1. A small message per tick (chosen, posted once per drawn frame with the newest input: part 5)** | low | one message per tick (per frame as chosen) | constant work | – | edge counters keep a press from being lost or doubled (row GP2) | the never-wired command (row CS1) gets wired | works without isolation | one message per tick (per frame as chosen) |
END

### FIX NA-7 (trade-off, the Worker)
FILE: docs/adr/0002-code-structure-build-toolchain.md
BEFORE:
- **A Worker, one block out and one command in per tick** wins over the main thread and over shared memory because the
  tick leaves the render thread, the work per tick is constant in N, and nothing needs cross-origin isolation, nightly
  Rust or a fixed memory maximum. The strongest argument against it: one copy per tick and a block whose delivery waits
AFTER:
- **A Worker, one block out per tick and the newest input in per frame** wins over the main thread and over shared
  memory because the tick leaves the render thread, the work per tick is constant in N, and nothing needs cross-origin
  isolation, nightly Rust or a fixed memory maximum. The strongest argument against it: one copy per tick and a block
  whose delivery waits
END

### FIX EA-8 (trade-off, the crates)
FILE: docs/adr/0002-code-structure-build-toolchain.md
BEFORE:
- **Two crates beyond R5's list** (`room`, `navbake`) cost two more dependency checks; without them the local server
  cannot run in the Worker, and the bake's generator would risk shipping in the player's Wasm. No research compared
  other placements, so the first foundation milestone that builds them is also their test; whether rule 3 allows
  proposing them at all is OQ-0002-8.
AFTER:
- **Three crates beyond R5's list** (`room`, `client-endpoint`, `navbake`) cost three more dependency checks; without
  them the local server cannot run in the Worker, the headless clients of ADR 0001's network marks and of the soaks
  would need a native build of the bindings crate or a second copy of the client endpoint, and the bake's generator
  would risk shipping in the player's Wasm. No research compared other placements, so the first foundation milestone
  that builds them is also their test; whether rule 3 allows proposing them at all is OQ-0002-8.
END

### FIX SA-1 (consequences, revisit when)
FILE: docs/adr/0002-code-structure-build-toolchain.md
BEFORE:
  ADR 0006 decides where the lobby runs (its crate); the cache passes 8 GB; ADR 0006 picks Arm hosts (an arm64
  determinism leg); Mustafa answers OQ-0002-1.
AFTER:
  ADR 0006 decides where the lobby runs (its crate); the cache passes 8 GB; ADR 0006 picks Arm hosts (an arm64
  determinism leg); Mustafa answers OQ-0002-1; the security baseline rules on CI tokens (OQ-0002-9's interim ends).
END

### FIX IE-3 (OQ-0002-5)
FILE: docs/adr/0002-code-structure-build-toolchain.md
BEFORE:
- **OQ-0002-5. Does real Safari run on pull requests, and does the end-to-end suite need it?**
  - Why it matters: Playwright's WebKit is not Safari (R5-TEST-F8), so a Safari-only bug reaches players unless real
    Safari runs somewhere; the macOS runner's time and cost are not known.
  - Options: (a) on pull requests that touch `client`, `net`, `sim-worker` or `sim` (R5-TEST-REC1); (b) nightly only;
    (c) also an end-to-end suite in real Safari through WebdriverIO (not researched).
  - Proposed interim: (b), which R5-XQ-OQ1 says breaks neither part, until the cost is known.
  - Decider: infra-engineer's reading of the macOS runner's time and cost (ADR 0006 for costs; Mustafa for any paid
    plan), then this record.
  - What decides it: the macOS runner's time and price for a public repository; whether WebKit on macOS misses a Safari
    bug that a playtest or Mustafa's checks find.
  - Research ids: R5-XQ-OQ1, R5-TEST-OQ4.
AFTER:
- **OQ-0002-5. Does real Safari run on pull requests, and does the end-to-end suite need it?**
  - Why it matters: Playwright's WebKit is not Safari (R5-TEST-F8), so a Safari-only bug reaches players unless real
    Safari runs somewhere. While the repository is public the macOS runner costs nothing: GitHub's runner reference
    lists macOS among the standard runners, whose use "is free and unlimited on public repositories" (the page
    R5-CI-F17 read, r5 lines 673–678, which quoted only its ubuntu rows; its macOS rows were read by infra-engineer at
    the step-10 review). What is not known is the macOS job's wall time against CI's 10-minute mark (PM-0002-56).
  - Options: (a) on pull requests that touch `client`, `net`, `sim-worker` or `sim` (R5-TEST-REC1); (b) nightly only;
    (c) also an end-to-end suite in real Safari through WebdriverIO (not researched).
  - Proposed interim: (b), which R5-XQ-OQ1 says breaks neither part, until the macOS job's wall time is known.
  - Decider: this record, from the macOS job's wall time, which infra-engineer measures in the foundation; ADR 0006 and
    Mustafa only if the repository stops being public, since a private repository's runners draw on paid minutes (the
    same page, read at the step-10 review).
  - What decides it: the macOS job's wall time against PM-0002-56's 10 minutes; whether WebKit on macOS misses a Safari
    bug that a playtest or Mustafa's checks find.
  - Research ids: R5-XQ-OQ1, R5-TEST-OQ4.
END

### FIX FB-8, EA-8 (OQ-0002-8), SA-1 (new OQ-0002-9)
FILE: docs/adr/0002-code-structure-build-toolchain.md
BEFORE:
- **OQ-0002-8. May this record propose a structure no research Options section offers, when a settled input forces it
  and the research leaves the question to this record?**
  - Why it matters: plan rule 3 says an option the research lacks is labelled "not researched" and cannot be the
    proposed decision. SLICE-ORDER condition 1 runs the local server in the Worker, which needs room logic that builds
    for Wasm, while R5-LAYOUT-REC2 puts rooms inside `server` (r5-toolchain-quality.md, lines 1173–1174); and the critic's
    questions hand structure to this record with no answer on one side ("Decided by: ADR-0002": R4-XT-OQ5,
    r4-physics-navigation.md lines 1195–1200, the bake's place; R5-XT-OQ6, r5 lines 1996–2001, the multiplayer-preview
    workflow, whose parts part 10 takes from research recommendations).
  - Options: (a) rule 3 allows it in such cases: the `room` and `navbake` crates stay proposed, labelled "not
    researched", and the first foundation milestone that builds them is their test; (b) rule 3 holds: both become open
    questions, a short layout research unit answers them before the foundation, and part 3 keeps R5-LAYOUT-REC2's list
    until then.
  - Decider: the Advisor, at the stop.
  - What decides it: whether rule 3 is meant to cover structure that a settled input forces and the research hands to
    this record.
  - Research ids: none of its own; it concerns R5-LAYOUT-REC2 (changed), R4-XT-OQ5 and R5-XT-OQ7.
AFTER:
- **OQ-0002-8. May this record propose a structure no research Options section offers, when a settled input forces it
  and the research leaves the question to this record?**
  - Why it matters: plan rule 3 says an option the research lacks is labelled "not researched" and cannot be the
    proposed decision. SLICE-ORDER condition 1 runs the local server in the Worker, which needs room logic that builds
    for Wasm, while R5-LAYOUT-REC2 puts rooms inside `server` (r5-toolchain-quality.md, lines 1173–1174); ADR 0001's
    network marks run the client endpoint natively and headless, which needs it outside the bindings crate (part 3);
    and the critic's questions hand structure to this record ("Decided by: ADR-0002"): R4-XT-OQ5, with no answer on
    one side (r4-physics-navigation.md lines 1195–1200, the bake's place), and R5-XT-OQ6, which names both sides (r5
    lines 1996–2001, the multiplayer-preview workflow, whose parts part 10 takes from research recommendations).
  - Options: (a) rule 3 allows it in such cases: the `room`, `client-endpoint` and `navbake` crates stay proposed,
    labelled "not researched", and the first foundation milestone that builds them is their test; (b) rule 3 holds:
    the three become open questions, a short layout research unit answers them before the foundation, and part 3
    keeps R5-LAYOUT-REC2's list until then.
  - Decider: the Advisor, at the stop.
  - What decides it: whether rule 3 is meant to cover structure that a settled input forces and the research hands to
    this record.
  - Research ids: none of its own; it concerns R5-LAYOUT-REC2 (changed), R4-XT-OQ5 (the same question as R5-XT-OQ7)
    and R5-XT-OQ6.
- **OQ-0002-9. Until the security baseline rules on CI tokens, what keeps a rewritten deploy job from the client-host
  token?**
  - Why it matters: a `pull_request` run takes its workflow from the pull request's merge commit (part 10; R5-PREV-F2,
    r5 lines 770–774), so any branch of this repository, which agent sessions push, can rewrite the preview deploy job
    and use its account-scoped token (R5-PREV-F10, r5 lines 823–830), which can probably reach production too (an
    inference). The first such token arrives with ADR 0006's preview hosting check and the foundation's preview step,
    months before the baseline (BASELINE-WHEN (b)); the binary-delivery and Sentry tokens of parts 9 and 10 face the
    same path.
  - Options: (a) a required reviewer on `preview` and on the delivery and Sentry jobs' environments when they run for a
    preview, so a job gets its token only after a person approves that run ("a job cannot access environment secrets
    until one of the required reviewers approves it", and reviewers are on the Free plan for public repositories:
    R5-PREV-F6, r5 lines 796–804; the repository is public, r5 lines 752–754): one approval per preview deploy; who
    approves, when sessions push and post as Mustafa's account (CLAUDE.md), and whether GitHub can stop an account
    approving a run it started, were not read; (b) production in a client-host account the preview token cannot reach
    (not researched: a second account, Mustafa's); (c) both.
  - Proposed interim: (a), the guard the research read (R5-PREV-REC1: "The environment gives one place for the secret
    and, when wanted, a reviewer gate", r5 lines 1588–1604), until the baseline rules; (b) only if Mustafa finds the
    approvals too costly, after a read of the client host's account rules.
  - Decider: Mustafa (the approvals are his time; a second account is his), on the Advisor's advice, with ADR 0006,
    part 13, item B9 (the tokens' scope); answered before the first client-host token is added to this repository.
  - What decides it: the approvals' cost per pull request; a read of GitHub's reviewer rules for an account that
    started the run, and of whether the client-host token narrows to one project.
  - Research ids: none of its own; it concerns R5-PREV-REC1 and R6-XT-OQ4 (the delivery token, ADR 0006's B9).
END

### FIX NA-4, NA-7 (spike item 1)
FILE: docs/adr/0002-code-structure-build-toolchain.md
BEFORE:
   Wasm calls per tick at N = 12 and N = 48, wasm-bindgen's glue calls included (part 5 batches the command and the
   received datagrams into one input call), at most 4 for the online client and for a local match in one instance
AFTER:
   Wasm calls per tick at N = 12 and N = 48, wasm-bindgen's glue calls included (part 5 batches the newest input and
   the received datagrams, with their arrival times, into one input call), at most 4 for the online client and for a
   local match in one instance
END

### FIX EA-1, RR-5 (spike item 2)
FILE: docs/adr/0002-code-structure-build-toolchain.md
BEFORE:
   shared clock (`performance.timeOrigin + performance.now()` on both sides); zero lost or reordered events over a
   10-minute match at the largest mode's N (48); bytes per tick and copy time at N = 48 for a per-entity and a per-field
   layout (R1-BND-OQ6); B2 against B3 on the four browsers: B3 is weighed only if it saves at least 0.5 ms of page time
   per frame or one frame of latency at p95 on the minimum laptop. The field-by-field writer's copy time is reported
   beside the copy (R1-RR-OQ6).
AFTER:
   shared clock (`performance.timeOrigin + performance.now()` on both sides); zero lost events, and zero released out
   of order on their entity's timeline (part 5), over a
   10-minute match at the largest mode's N (48); bytes per tick and copy time at N = 48 for a per-entity and a per-field
   layout (R1-BND-OQ6); B2 against B3 on the four browsers: B3 is weighed only if it saves at least 0.5 ms of page time
   per frame or one frame of latency at p95 on the minimum laptop. The field-by-field writer's time is reported beside
   the copy and read against part 5's trigger, a p99 above 0.67 ms in Wasm on the minimum laptop (R1-RR-OQ6).
END

### FIX NA-7 (spike item 3)
FILE: docs/adr/0002-code-structure-build-toolchain.md
BEFORE:
3. **The command** (part 5; R1-BND-REC3, r1 lines 2286–2296): with injected delay, duplication and reordering of
   commands, zero lost and zero doubled presses over 10,000 scripted presses; in local play, from the key event's
   timestamp to the tick that applies it, at most one tick plus one frame at p95; the cursor ray and view time reach the
   Worker in the same message (proposed: part 5 sends no second message).
AFTER:
3. **The command** (part 5; R1-BND-REC3, r1 lines 2286–2296): with injected delay, duplication and reordering of
   commands, zero lost and zero doubled presses over 10,000 scripted presses; in local play, from the key event's
   timestamp to the tick that applies it, at most one tick plus one frame at p95, with the page posting its newest
   input once per drawn frame and every tick running the newest post (part 5); the cursor ray and view time reach the
   Worker in the same message (proposed: part 5 sends no second message).
END

### FIX PA-13 (spike item 6)
FILE: docs/adr/0002-code-structure-build-toolchain.md
BEFORE:
   differing only in the mode flag; the Wasm calls per tick, the Worker's time per tick and the memory of one instance
   against two in a local match (OQ-0002-3).
AFTER:
   differing only in the mode flag; the Wasm calls per tick, the Worker's time per tick and the memory of one instance
   against two in a local match (OQ-0002-3); and, reported with no limit (the research sets none: part 6), the Worker's
   busy time per tick (p99) online and in a local match at N = 48, and its JS heap and Wasm memory at minutes 5 and 10
   of a match.
END

### FIX routed-b815c85 (spike item 7)
FILE: docs/adr/0002-code-structure-build-toolchain.md
BEFORE:
   places them, item 3): ADR 0004's S18 (tick start jitter, smoothness on 60, 120 and 144 Hz displays, hidden tabs) and
AFTER:
   places them, item 3): ADR 0004's S18 (tick start jitter, read online against the schedule the step returns, part 5;
   smoothness on 60, 120 and 144 Hz displays; hidden tabs, with catch-up counted per wake) and
END

### FIX EA-8 (migration, the toolchain)
FILE: docs/adr/0002-code-structure-build-toolchain.md
BEFORE:
   2. *The toolchain.* The two workspaces and the pins (part 2), the first crates (`sim`, `protocol`, `room`,
      `sim-wasm`, `devtools`) and packages (`client`, `sim-worker`, `net`, `types`, `e2e`). *Done when:* from a fresh
AFTER:
   2. *The toolchain.* The two workspaces and the pins (part 2), the first crates (`sim`, `protocol`, `room`,
      `client-endpoint`, `sim-wasm`, `devtools`) and packages (`client`, `sim-worker`, `net`, `types`, `e2e`). *Done
      when:* from a fresh
END

### FIX NA-7 (migration, the skeleton)
FILE: docs/adr/0002-code-structure-build-toolchain.md
BEFORE:
      milestone); the page posts one command per tick, receives one block per tick and draws an empty scene from it,
AFTER:
      milestone); the page posts its newest input each frame, every tick runs on the newest post, and the page receives
      one block per tick and draws an empty scene from it,
END

### FIX SA-1 (migration, previews)
FILE: docs/adr/0002-code-structure-build-toolchain.md
BEFORE:
   6. *Previews.* After ADR 0006's preview hosting check: the build, deploy, close and sweep workflows and the generated
      header sets. *Done when:* a same-repository pull request has its link within 5 minutes after the build job ends
      (median of 5 runs, R5-PREV-REC1 mark 1); the deploy job's header check passes and fails when a header is removed
AFTER:
   6. *Previews.* After ADR 0006's preview hosting check, with OQ-0002-9's guard in place: the build, deploy, close and
      sweep workflows and the generated header sets. *Done when:* a same-repository pull request has its link within 5
      minutes after the build job ends, or after the reviewer's approval while OQ-0002-9's interim (a) holds
      (median of 5 runs, R5-PREV-REC1 mark 1); the deploy job's header check passes and fails when a header is removed
END

### FIX EA-12 (migration, slice 3)
FILE: docs/adr/0002-code-structure-build-toolchain.md
BEFORE:
5. **Slice 3, online:** `server` enters; L2 in the nightly run; G12 over both transport backends; the first multiplayer
AFTER:
5. **Slice 3, online:** `server` and `supervisor` enter; L2 in the nightly run; G12 over both transport backends; the
   first multiplayer
END

### FIX GE-2 (test plan, renders)
FILE: docs/adr/0002-code-structure-build-toolchain.md
BEFORE:
- **Renders and real pages:** Playwright on the built client and a local server, the backend named on every shot, once
  with three.js's `forceWebGL`, and once with and once without the isolation flag; ADR 0003's counts against their
  baseline (G12).
AFTER:
- **Renders and real pages:** Playwright on the built client and a local server, the backend named on every shot, once
  with three.js's `forceWebGL`, and once with and once without the isolation flag; ADR 0003's counts against their
  baseline (G12); the WebGPU backend through part 9's route (a CI job if the boundary spike finds one, otherwise a
  smoke run on a reference machine for every change-set that touches the render path).
END

### FIX EA-1, EA-12 (Appendix A, R1-BND-REC2)
FILE: docs/adr/0002-code-structure-build-toolchain.md
BEFORE:
| R1-BND-REC2 | changed: two sections added (weapon pickups; the local player's owner-only data), remote entities written at ADR 0001's interpolation delay, and mark 4 at N = 48 (MODES-3 (a)) | part 5: UPGRADE-SCOPE's pickups and the I-21 upgrade data must reach the page; ADR 0001 put the interpolation buffer in the Worker; the spike's item 2 |
AFTER:
| R1-BND-REC2 | changed: two sections added (weapon pickups; the local player's owner-only data, with the floor layer and enclosure area id of ADR 0004 part 14), remote entities written at ADR 0001's interpolation delay with their spatial events released on the same timeline, and mark 4 at N = 48 (MODES-3 (a)) | part 5: UPGRADE-SCOPE's pickups and the I-21 upgrade data must reach the page; ADR 0001 put the interpolation buffer in the Worker, and R2-SYNC-REC6 plays blasts on the timeline of the entity they hit; the spike's item 2 |
END

### FIX NA-7 (Appendix A, R1-BND-REC3)
FILE: docs/adr/0002-code-structure-build-toolchain.md
BEFORE:
| R1-BND-REC3 | changed: the per-tick message also carries the cursor ray and the view time | part 5: ADR 0001 part 5 has the Worker cast the aim ray (I-19); the fields are ADR 0001's; the spike's item 3 |
AFTER:
| R1-BND-REC3 | changed: the page posts its newest input once per drawn frame, and each tick the Worker builds the one command from the newest post; the message also carries the cursor ray and the view time | part 5: the page has no tick clock, so a post per block can miss the REC's own mark 2 (2 × 1000 / 30 = 66.67 ms against 50.00 ms); ADR 0001 part 5 has the Worker cast the aim ray (I-19); the fields are ADR 0001's; the spike's item 3 |
END

### FIX RR-5 (Appendix A, R1-RR-OQ6)
FILE: docs/adr/0002-code-structure-build-toolchain.md
BEFORE:
| R1-RR-OQ6 | answered (proposed; rust-reviewer, its named decider, confirms at review): the block writer serializes field by field, as the state hash does; the derive or `unsafe` only if the copy time says so | part 5; the spike's item 2 reports the copy time (PM-0002-10) |
AFTER:
| R1-RR-OQ6 | answered: confirmed by rust-reviewer, its named decider, at the step-10 review: the block writer serializes field by field, as the state hash does, with one layout table for the writer, the offsets and the hash, fixed-width little-endian integers and aligned sections; the derive or `unsafe` only if the writer's p99 exceeds 0.67 ms (proposed) | part 5; the spike's item 2 reports the writer's time (PM-0002-10) |
END

### FIX IE-4, SA-1 (Appendix A, R2-XT-OQ10)
FILE: docs/adr/0002-code-structure-build-toolchain.md
BEFORE:
| R2-XT-OQ10 | answered: the binary-delivery job and the hash written into the preview's files; the token's scope waits on the security baseline | part 10 |
AFTER:
| R2-XT-OQ10 | answered: the binary-delivery job; a certificate hash in a preview's static files goes stale, so whether a multiplayer preview pins one is OQ-0006-5's, with part 10's two shapes; the token's scope waits on the security baseline, its guard on OQ-0002-9 | part 10 |
END

### FIX EA-8, EA-12 (Appendix A, R5-LAYOUT-REC2)
FILE: docs/adr/0002-code-structure-build-toolchain.md
BEFORE:
| R5-LAYOUT-REC2 | changed: the `room` and `navbake` crates added (proposed, not researched: OQ-0002-8); nothing imports `spikes/` | part 3: SLICE-ORDER condition 1 runs the local server in the Worker; R4-XT-OQ5 |
AFTER:
| R5-LAYOUT-REC2 | changed: the `room`, `client-endpoint` and `navbake` crates added (proposed, not researched: OQ-0002-8), and the supervisor's crate at slice 3 (R6-HOST-REC2's supervisor, ADR 0006 part 5); nothing imports `spikes/` | part 3: SLICE-ORDER condition 1 runs the local server in the Worker; ADR 0001's network marks run the client endpoint natively; R4-XT-OQ5 |
END

### FIX SA-1 (Appendix A, R5-PREV-REC1)
FILE: docs/adr/0002-code-structure-build-toolchain.md
BEFORE:
| R5-PREV-REC1 | adopted | part 10 |
AFTER:
| R5-PREV-REC1 | changed: a second client-host token, production's, in its own environment on `main` alone; the reviewer gate the REC allows "when wanted" proposed until the security baseline (OQ-0002-9) | part 10: a branch of this repository can rewrite the deploy job (R5-PREV-F2's contrast) |
END

### FIX RR-1 (Appendix A, R5-RUSTLINT-OQ1)
FILE: docs/adr/0002-code-structure-build-toolchain.md
BEFORE:
| R5-RUSTLINT-OQ1 | answered: the method ban, with a reviewed text scan of `sim` as the fallback; G2's planted `x.sin()` proves which | part 7 |
AFTER:
| R5-RUSTLINT-OQ1 | answered: the method ban, written as the complement of ADR 0004 part 7's allow-list (every other std `f32` and `f64` method), with sim helpers for `min`, `max` and `clamp`, and a reviewed text scan of `sim` as the fallback; G2's planted `x.sin()` proves which | part 7 |
END

### FIX GE-2 (Appendix A, R5-CI-OQ5)
FILE: docs/adr/0002-code-structure-build-toolchain.md
BEFORE:
| R5-CI-OQ5 | answered: CI gates counts on whatever backend the runner gives, plus a `forceWebGL` run; no GPU runner | part 9 |
AFTER:
| R5-CI-OQ5 | answered: CI gates counts on whatever backend the runner gives, plus a `forceWebGL` run, and records the backend each CI engine gets (probably WebGL 2 everywhere on a GPU-less runner); the WebGPU route is part 9's (a SwiftShader WebGPU job if the boundary spike finds one, else a smoke run on a reference machine); no GPU runner | part 9 |
END

### NOTE SA-1 (routed)
routed: (1) infra-engineer, ADR 0006 part 13, B9's interim position for the deploy token should name OQ-0002-9 (a
required reviewer on `preview` proposed until the baseline), the `production` environment with its `main`-only branch
rule and its own token (ADR 0002 part 10), and the revocation of the spike's test-project token after the preview
hosting check (0006 part 14); B9's "held only by the deploy job's environment `preview`" (0006:389-394) then reads
"`preview` and `production`". (2) test-engineer: PM-0002-66 gains SA-1's bite (a test pull request that edits the
deploy job to print a marker gets no secret before approval), and the preview-link time row (R5-PREV-REC1 mark 1) is
timed from the reviewer's approval while OQ-0002-9 (a) holds, as migration step 2.6 now says. (3) the synthesis
(mine, later): a README row for OQ-0002-9 (decider Mustafa, on the Advisor's advice); SPIKE_PLAN item 6's token
follows B9.
END

### NOTE EA-1 (routed)
routed: netcode-architect, ADR 0001 part 7 states the rule ADR 0002 part 5 now points to: the Worker releases a
remote entity's spatial event (action start, death, a blast on the entity it hit) when its interpolation clock
reaches the event's tick; the local player's own, predicted and owner events go at once; online and in a local match
(R2-SYNC-REC6, r2:2210). test-engineer: PM-0002-9 checks order per timeline, and a new row checks that an event is
drawn within one frame of its entity's pose (EA review finding 1).
END

### NOTE EA-8 (routed)
routed: the new `client-endpoint` crate should appear wherever the drafts name the crate list: ENGINE_OVERVIEW's
diagram and SPIKE_PLAN (the synthesis, mine); ADR 0001's components diagram may name it beside "client endpoint"
(netcode-architect, optional); test-engineer's PM-0002-44 may add a G5 bite (wasm-bindgen or a transport crate added
to `client-endpoint`).
END

### NOTE EA-12
fixed: the supervisor gets a crate row (slice 3, R6-HOST-REC2 through ADR 0006 part 5) and only the lobby's crate
waits for OQ-0006-3; the floor layer and enclosure area id (ADR 0004 part 14, 0004:387-388) join the owner-only
section and Appendix A's R1-BND-REC2 row.
END

### NOTE NA-4 (routed)
routed: netcode-architect, ADR 0001 parts 5 and 7: clock and jitter samples use the arrival stamps ADR 0002 part 5
now passes in, and part 5 says how a ping answered at the next tick reports its hold time (or that `net` answers it on
arrival, which would put a message-kind test in TypeScript). test-engineer: PM-0001-47's bite (receive times
quantised to the tick).
END

### NOTE NA-7 (routed)
routed: netcode-architect, ADR 0001 part 5 ("the page posts, with its per-tick command, its cursor ray") reads "with
its newest input, posted once per drawn frame (ADR 0002 part 5)", and the components diagram's "per-tick command +
cursor ray" reads "newest input + cursor ray, each frame". The synthesis (mine): ENGINE_OVERVIEW:37 and :94 say the
same. test-engineer: PM-0002-64 "every tick runs the newest posted input" in place of "commands posted equal ticks
run"; PM-0002-13 holds as written.
END

### NOTE NA-8 and the routed ADR 0004 items (b815c85: part 5 with S18, part 20 with S20)
fixed: part 5's control API now says the step returns when the next tick is due (local play: the plain accumulator;
online: the client endpoint's server-time estimate under OQ-0001-6), the host sets its timer from that, and catch-up
is counted per wake, at most 7 ticks (ADR 0004 part 5, S18); spike item 7 reads S18 online against that schedule.
Part 4 now says the host reads the panic hook's fixed buffer when the instance traps (ADR 0004 part 20, S20). Part 6's
local order names ADR 0004 part 3's two entry points (the prediction step and the authority step). NA-8's ADR 0004
half landed in b815c85 (rust-reviewer).
END

### NOTE NA-15
fixed: part 6 states the order of one local Worker step (command for T, through the loopback, the local server runs
T, encodes, the client endpoint decodes, reconciles and writes the block).
END

### NOTE IE-3 (routed)
routed: infra-engineer's ADR 0006 Q13 and OQ-0006-7 split macOS (standard, free on a public repository) from GPU
larger runners; the synthesis (mine): README:59's OQ-0002-5 row reads "the macOS job's wall time" in place of "cost".
END

### NOTE IE-4 (routed)
routed: infra-engineer, OQ-0006-5 names the multiplayer preview's certificate among its consequences, with ADR 0002
part 10's two shapes (staging presents the trusted `wss` certificate on both transports and previews pin no hash; or
one certificate per VM kept across restarts, the next made ahead, both hashes listed), and ADR 0006 part 4 says which
the staging server presents. netcode-architect: ADR 0001 part 10's hash fetched from the page's own origin holds for
development and the proof test; staging follows OQ-0006-5. The synthesis (mine): README's I-13 row qualifies
"certificate hash".
END

### NOTE IE-14 (routed)
routed: infra-engineer, ADR 0006 part 8 (a cost line for the bench's reference-server hours) and part 13, B9 (the
Hetzner API token, never the production project's), with IE's SPIKE-VM-ACCESS question for Mustafa; ADR 0002 part 8
now points there for the bench and the 3-hour soak.
END

### NOTE IE-19 (routed)
routed: infra-engineer, ADR 0006 part 3 sets the failure artefacts' retention (IE proposes 30 days); ADR 0002 part 8
now points to it and makes the issue carry the seed.
END

### NOTE FB-8
fixed in OQ-0002-8 (folded into the same block as EA-8's wording and the new OQ-0002-9, since they touch the same
lines).
END

### NOTE PA-13 (routed)
fixed in part 6 (this record owns the Worker as a whole; reported, no limit until the spike's numbers) and spike item
6; routed: test-engineer adds the reported row (Worker busy time per tick p99 in both modes at N = 48; JS heap and
Wasm memory at minutes 5 and 10, read under PM-0003-24's flat rule if they choose).
END

### NOTE GE-2 (routed)
routed: test-engineer, PM-0003-36, PM-0002-51 and PM-0002-64 record the backend per CI engine and add the WebGPU
smoke row (CI job if the spike finds one, else on a reference machine per render-path change-set), as GE-2 asks.
END

### NOTE RR-1 (routed)
routed: rust-reviewer, ADR 0004 part 7's allow-list says whether `abs`, `copysign`, `rem_euclid` and `div_euclid`
join it, and names the sim helpers for `min`, `max` and `clamp` (they also remove 0004:470-471's `clamp` hazard);
test-engineer, PM-0002-41 and PM-0004-23 bites: `deg.to_radians()` and `a.min(b)` in `sim` each fail G2.
END

### NOTE RR-2 (routed)
routed: netcode-architect, ADR 0001 parts 5 and 11 say which timers are tick-counted in `room` (state timeouts, the
grace window) and which run on wall time in `server` (the join deadline, per-connection rate limits); test-engineer,
PM-0002-41's bite: `Instant::now()` in `room` fails G2.
END

### NOTE RR-5, RR-6, RR-7, RR-10 (routed)
routed to test-engineer: PM-0002-10 takes the 0.67 ms decision rule (RR-5); PM-0002-6 a bite with a planted
`memory.grow` between two ticks (RR-6); PM-0002-41 a bite with `-ffm` in the wasm-opt step, and PM-0002-29 measures
the shipped file (RR-7); PM-0004-74 a control (a dev-only feature on `devtools` that touches no `sim` dependency
passes) and PM-0002-44 the same comparison (RR-10).
END

### NOTE RR-16 (routed)
routed to my own ADR 0005 batch: CD-3's schemars derive sits behind `cfg_attr(test, ...)` (or a dev-only feature), and
the schema is written on `cargo test` or by a tool, never from a shipped build; ADR 0002 part 5 now says "content's
schema generator follows the same rule (ADR 0005, CD-3)".
END

### NOTE TE-7 (routed)
fixed in part 8; routed: test-engineer's PM-0002-76 names the reference server for the 3-hour soak's tick times.
END

### NOTE SA-10, SA-11 (not in this round's list; their ADR 0002 halves)
deferred: SA-10's G5 half (cargo-deny's `sources` class at deny, crates.io only) rests on a class the research only
lists (R5-GATE-F3, r5:954-960), so the rule belongs with the security baseline's B1 (infra-engineer's row) and joins
G5 when it rules. SA-11's part 7 half (`no-eval`, `no-new-func`) is deferred by part 7's own rule that lints whose
sources were not read are not added; B3's "CSP-ready build" (infra-engineer) can name them, and part 7 follows the
baseline.
END
