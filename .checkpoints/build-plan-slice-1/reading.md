# Reading notes: build-plan-slice-1 (the main session's own reads)

Plan of blocks: B1 kit and status · B2 ENGINE_OVERVIEW · B3 SPIKE_PLAN · B4 ADR 0002 (parts 3 to 9, migration plan) ·
B5 the migration plans of ADR 0001, 0003, 0004, 0005, 0006 · B6 ADR 0005 (the spike items of content and abilities) ·
B7 LOOP_V2 and LOOK_V2 · B8 DECISIONS (Queued, models, SLICE-ORDER) · B9 the 2.3 plan on its branch · B10 the
skeleton on main, CI, settings. The pass-mark tables are digested by agents into `marks-000N.md` (one file per draft).

## B1 kit and status (done)
- CLAUDE.md, CHECKPOINTS.md, REPO_MAP, STATUS read. STATUS is one merge behind (lists #52 as open); main is 4460ef5.
- CI on main: `gate.yml` (jobs `gate`, `smoke`, required) and `toolchain.yml` (job `toolchain`, not required): in a
  network cut it runs `cargo test --workspace --locked`, `cargo test --release -p sim -p protocol --locked`,
  `cargo build --locked --target wasm32-unknown-unknown -p sim -p protocol -p room -p client-endpoint -p sim-wasm`,
  `npm ci --offline`, `npm test --workspaces`, `tsc --version`. No clippy, rustfmt, ESLint, `tsc -b`, no Wasm run, no
  browser. So a test added to a crate or package runs in CI today with no workflow edit; a Wasm run or a browser run
  does not.
- Settings deny `npx`, `npm exec`, `npm run`, `npm test` in sessions: tools run by their path.

## B2 ENGINE_OVERVIEW (done)
- One Rust sim, native in the room process and as Wasm in one Web Worker (client endpoint; in local play also the local
  server). The page posts its newest input per drawn frame and gets one transferred block per tick; it makes no Wasm
  call and touches no network. Dependencies point one way (sim <- content only).

## B3 SPIKE_PLAN (done)
- The six tests of his decision and their marks by id: sim core spike PM-0004-1..-61, -73, -74 · 60 Hz run
  PM-0001-39, -40, -63, PM-0004-71, -72 · boundary spike PM-0002-1..-34, -77, -78, PM-0004-8, -28, -61, -68..-70,
  PM-0006-28..-31 (optional) · look proof test PM-0003-1..-17, PM-0005-33..-37 · content and ability spike
  PM-0005-1..-32, PM-0004-62..-66, -75 · render budget bench PM-0003-18..-42, -44..-47, -43 at each later upgrade.
- Machines: ML (minimum laptop), RD (recommended desktop), MAC: not named yet (OQ-0003-1); his laptop's integrated
  graphics with Brave on "Power saving" stands in for ML "for rough checks only"; a rough check is reported, never
  judged. RS (CCX23) and LG are created by him per run.

## B4 ADR 0002 (done)
- Migration plan :1431-1506. 2.4 done-when: "the marks of the spike's item 4 hold in CI" (PM-0002-15..-19, -63, with
  G4's bites -43). 2.5 done-when :1473-1478 (PM-0002-64). Slice 1 :1487-1491 (PM-0002-70, -71).
- Part 5: one module Worker; block B2 (header, sections, owner-only, events), one layout table drives writer, offsets
  and hash; newest input per frame with cursor ray and view time; control API through wasm-bindgen (one input call and
  one step call per tick, at most 4 Wasm calls); generated types from three sources into `packages/types`
  (`TS_RS_EXPORT_DIR` in `.cargo/config.toml` in the draft: see B9, no root `.cargo/config.toml` while the netcode
  spike's rule holds); isolation off; the Worker owns the connection.
- Part 6: local match order inside one Worker step (command for T reaches the local server before it runs T); one
  Wasm instance (OQ-0002-3 measured).
- Part 7: lint sets; the float ban as the complement of ADR 0004 part 7's allow-list; clock ban in the Worker's crates.
- Part 8: test layers; the replay runner; goldens from the legacy oracle; L1 and L2; flakes; the bench method (RS and
  ML). Part 9: G0-G13.

## B5 migration plans (done)
- ADR 0001 :1778-1786: foundation: protocol crate (messages, codec, thin transport interface, loopback), Join,
  JoinAccepted, Input, Snapshot over the loopback with L1 behind it in CI. Slice 1: input commands from day one,
  replayable; local server the authority; prediction and reconciliation of own movement and weapon state through the
  loopback. Done when: a recorded input replay gives the same per-tick state hash on two runs, and R2-SYNC-REC2 marks
  1-2 and R2-SYNC-REC3 marks 1-3 pass on the loopback under L1's profiles (PM-0001-20, -41, -42, -43 by their rows).
- ADR 0003 :1390-1399: foundation: `await renderer.init()`, the backend in the overlay and in every test screenshot,
  CI gates the render counts. Slice 1: one skinned soldier, the toon materials and ink kept in step 2, the feel kit's
  visual effects and voices for the slice's weapons, split feedback. Milestone: programs constant over a match, zero
  per-frame allocation in the render path, Mustafa's Brave check.
- ADR 0004 :1252-1253: foundation: sim crate with the hash-map ban and float rules as lints, a replay runner, the
  first recording in CI; hashes match natively twice and in Wasm. Slice 1: soldier table and ids, the tick and its
  loops, movement on parry3d against a greybox map document, hitscan and weapons from data in integer time, the local
  server in the Worker; every match input-driven and recorded. Milestone: S4, S5, S6 and S9 hold on the real crate;
  the determinism suite green on slice 1 recordings on every target.
- ADR 0005 :2439-2445: foundation: content's place, generated schema and types diffed by the gate, the glTF
  dependency check. Slice 1: CD-1 to CD-8 on weapons and movement (the import's weapon and movement tables, CD-10);
  weapons as data (AB-3, AB-4); the bundle and its hash in local play. Done when: content items 1, 2 and 6 and
  ability item 2 pass on the real crate in CI.
- ADR 0006 :1487-1494: foundation: preview links per PR (2.6), B9's interim position. Slices 1 and 2: no game server;
  previews run the match offline in the Worker; every preview link opens in Brave without a login.

## B6 ADR 0005 spike items (done)
- Content items 1 (float round trip, 10,000 doubles), 2 (fuzz corpus of 10,000 files), 6 (bundle hash, 3 tampering
  cases, cache, SHA-256 equals WebCrypto) :717-753; ability item 2 (AB-3: seven legacy weapons and the Juggernaut
  cannon per shot equal the legacy tables, integers exactly, floats within 1e-6 relative) :1433-1436.

## B7 LOOP_V2 and LOOK_V2 (done)
- LOOP_V2 §7 combat feel: the per-weapon feel kit as data (kick, muzzle flash, tracer, impact, hit marker, sound,
  target flinch, kill confirm), tuned live in a Feel Lab with the Imperial Warrior's SMG as the reference; split
  feedback; targets react; active reload; the AMK's rhythm; sprint-to-fire delay.
- LOOK_V2: 78 lines, as it stands on main (his look decisions of 13 and 14 November are not in the repo).

## B8 DECISIONS (done)
- Queued, the Slice 1 note (:73-75): "Slice 1 ships with a live camera tuning panel from its first playable build,
  ported from the legacy dev panel's camera sliders and presets (LEGACY_INVENTORY rows GP5 and CD5)."
- Models (:242-251): Fable 5.2 for chats and builder agents that write game or engine code; every other agent Opus
  5.5; reviewers, verifiers and designers of game and engine code always at max; "The guard, the gate and CI work
  stays with Opus 5.5."
- SLICE-ORDER (:933-948), conditions 1 to 3.
- LEGACY_INVENTORY CD5 (:87: height 40, boom 56.6, fov 15, yaw base 45 degrees, dead zone 0.60 x 0.52, lerp rates; six
  dead keys) and GP5 (:316: sliders boom, height, FOV, pitch override, ADS-camera mode cycle; five presets Classic,
  Tactical, Iso, Iso+, Foxhole; the perf overlay).

## B9 the 2.3 plan (branch claude/sweet-mayer-grsukw, revision 2.4; read only) (done)
- First part: CS-A docs (records his words of 2026-11-14), CS-B rules text, CS-C1 legacy gate, CS-D guard (protects
  the gate v2's folder and, by PIN-SCOPE (b), `rust-toolchain.toml`, the root `Cargo.toml`, `deny.toml`,
  `rustfmt.toml`, `tsconfig.base.json`, `eslint.config.mjs`, `.prettierrc.json`, `.prettierignore` and
  `crates/*/clippy.toml`; in Auto mode such an edit is refused), CS-C the gate v2's first part (G0, G1, G2, G3, G5,
  G6, G9 in part; the root lint and format configs; a `tsconfig.json` per package), CS-E CI (`required`), CS-F.
- Later parts, written before his decision of 2026-11-14: 2.3-B "rides step 2.4" (G4, G9's boundary layer and Wasm
  tests, G7 and the budget file, the wasm-bindgen install, PM-0002-53); 2.3-S "after the sim core spike", merged
  "before any f32 or f64 arithmetic enters `sim`" (the float ban, the map types, the same-feature check); 2.3-E "rides
  step 2.5" (G6's Vite build, G8, G10, G11 with L1, G12, the decoder panic lints, the full-gate timings); G13 at slice 2.
- Rules kept while PR #40's spike holds its rule: no tracked non-Markdown file outside `spikes/`, `docs/` and
  `.checkpoints/` may contain the text of the spike folder's name with its slash; no root `.cargo/config.toml`; no
  `"type"` in the root `package.json`; a change-set that adds or changes such a file runs the spike's own check on a
  local merge with #40's head, never pushed.
- Until `required` counts, each change-set reads `toolchain`'s runs by hand before a merge.

## B10 the skeleton on main (done)
- Six crates, each a stub with one test, no dependencies; five packages, each with one stub test; TypeScript 6.0.3,
  Vitest 5.1.0, @playwright/test 1.65.0; Rust 1.99.1 pinned. The container had no Rust toolchain: this session's
  `rustc --version` call (in the Bash call that printed the manifests, after the 13:59 UTC checkpoint save) made rustup
  start fetching the pinned 1.99.1: an install in the container, no repository change.
