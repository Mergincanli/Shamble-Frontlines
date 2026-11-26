# advisor agent: re-read of the cs4 plan before 4b (2026-11-26, main 2483e9a, branch head e284d2e)

## Plan
1. Read the sheet (done), his words of today (go-merge-70), DECISIONS (MUL-ADD, takeover point 8, "other questions wait"),
   CHECKPOINTS.md section 3 (DEP-HOLD).
2. Read plan.md sections 3, 5 (5.1-5.3), 9 (4b row), 11, 13 (CONTENT-SOURCE, DEPART-4), 14, 17.
3. Read the designs: engine-architect block 1, 3.6, 4.1, 4.2, 4.5; rust-reviewer blocks 1, 2, 4; testlines-cs4.
4. Check today's main: Cargo.toml + crates/*/Cargo.toml; devtools path edges used or not (grep sources);
   build-checks.mjs and build.mjs functions; messages.rs AimPoint; routine-guard protectedRel; STATUS "From change-set 4a".
5. Build plan slice 1 report + ADR 0001: where the command compression and aim width are planned (F.1).
6. Judge a-f; rule F.1-F.5; verdict; DONE.

## Findings (a line each, as found)
- [read] sheet, his words of today, DECISIONS (STEP-60 a, MUL-ADD, DEP-HOLD text in CHECKPOINTS s3), plan s3/5/9/11/13/14/17/20, EA block 1 + 3.6 + 4.1-4.5, RR blocks 1/2/4/5/6, testlines-cs4.
- [main] Cargo manifests read; devtools names no client_endpoint::/protocol::/room:: path (grep; EA N59 agrees): the three edges are unused.
- [main] protectedRel names no manifest: root Cargo.toml is tier 1 today (sheet A true).
- [main] AimPoint i16/u16/i16 cm (messages.rs:137-144): x,z -327.68..+327.67 m, y 0..655.35 m; "Provisional as a whole ... the codec sets them under PM-0001-51" (messages.rs:132-134).
- [F1 High] thiserror feature clash: ts-rs 12.0.1 depends on `thiserror = "2"` with default features (registry src on disk), thiserror 2.0.21 default = ["std"]; with parry3d std off thiserror enters sim's closure without std (RR B1-2/B1-7, sheet D "thiserror has one"); build.mjs checkCrates "native tests" list (normal,dev --workspace) then holds thiserror feature "std"/"default", the wasm lists do not -> simFeatureDifferences refuses every build from step 0. Not 3A-FEATURE-VERSION (same version) - same mechanism as 3A-PART5-CLASH.
- [F2 High] cut grammar vs files 4b's own tools read: Export has 5 `true`; docs/legacy-tuning.json has 79 true, 58 false, 57 null, 3 non-ASCII bytes (time report step 4, compare-old step 5, corpus 3b read it).
- [F3 Medium] B.2's aim-reach constants "beside AimPoint" in protocol vs content with no dependency (sheet D) and EA 1.12's plant "protocol added to content's dependencies".
- [F4 Medium] step 7 omits every other caller of Sim::create/new: sim-wasm tests, local_match tests, step_alloc.rs, devtools replay.rs:91, boundary replay.test.mjs (4), sim-wasm.test.mjs (3), e2e determinism.spec.ts:20; and 4A-EA-4B's first-loop re-make with frozen content + byte-identical hash lines.
- [F5 Medium] ADR 0005 foundation line: no step builds the "sim depends on content + parry3d's tree only; content on nothing" check (EA 1.12) that DEPART-4 entry 10 presumes.
- [F6 Medium] D: `cargo check` of devtools without the three edges "before the batch is shown to security-auditor" = a build holding the change before the answer (CHECKPOINTS s3: "build nothing that holds the change"); prove by grep now.
- [F7 Medium] nativeTargetProblems in build.mjs sees only build.mjs's own env (which already refuses CARGO_ENCODED_RUSTFLAGS etc.); the native test build that runs sim is never checked -> a cfg!(target_feature="fma"/"avx2") assertion in sim's/devtools' native tests is the real check.
- [lockfile] change-set 2's exact commands not reachable (shallow clone; cc9fc71 absent); 3a (0412814) and 3b (1ee3e91) recorded `cargo update --workspace --offline` ("Locking 0 packages"). cargo update -w is index-only; cargo metadata (no --no-deps) fetches every package (man page).
