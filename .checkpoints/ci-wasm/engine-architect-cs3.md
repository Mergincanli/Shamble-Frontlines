Task: change-set 3 ("The loop closes"), the boundary and rule-test half of its plan (engine-architect, part "cs3").

## Plan (blocks, in order)
- Block 1. Read: the card + Step 2.5 table + Part 15 lanes; ADR 0004 parts 7 and 8 + PM-0004-27..29; ADR 0002 crate and
  package rules + PM-0002-53; code (crates/sim/src, crates/sim-wasm/src/lib.rs, crates/client-endpoint/src,
  packages/sim-worker/src + boundary/, packages/client/src/app + overlay/, packages/client/scripts/build.mjs +
  build-checks.mjs); cs2 report; STATUS_ARCHIVE PR #61 block "For change-set 3"; test-engineer-followups.md blocks 5-6;
  rust-reviewer-cs3-rng.md; browser-reports-reading.md (FPS-LINE, NEG-ZERO). Notes per source as read.
- Block 2. The boundaries after cs3 (EA-F9, EA-F10, WB-HEALTH (a), EA-F8, EA-N3, EA-N6/VF-L7, panic hook, follow-up table).
- Block 3. The page, alive (lanes, input path, readout + Copy report, FPS-LINE, NEG-ZERO, five plain lines).
- Block 4. The rule tests (a) float vectors, (b) RNG 10,000 outputs, (c) scan test, (d) PM-0002-53, (e) two-instance test.
- Block 5. Size and order (lines per file, build order with green steps, CI/gate edits named, what moves out first).
- Block 6. Where the build plan / drafts are wrong or out of date; questions for Mustafa.
- Then DONE, then the answer from these notes alone.

## Notes (one line per finding; one after each finished block)
- (start) No engine-architect memory exists under .claude/agent-memory-local/; notes file was absent: fresh start.
- cs3-context.md read (pointers only; card line 249, card 316-333, Step 2.5 463-476, scan ruling 949-953).
