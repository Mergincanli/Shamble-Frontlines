Task: rust-reviewer, wave 1 step 3 (Verify): R1 Rust crate/determinism claims + R2 Rust server stack claims (v2-research plan).

## Progress
- started 2026-10-28: no prior notes; reading r1 and r2 docs next.
- read r1 (ecs 59-251, tick F6-F8/F23, det 548-733, bnd 802-1027, abl F23-F25, data 1266-1395) and r2 (TRN F6,F15,F17,F25,F28-F33; ROOM F7,F8; REC1-REC6, ROOM-REC2).
- scope list (claims to check): R1 ECS-F1..F33; TICK-F6,F7,F8,F23; DET-F6..F29; BND-F1..F10,F18,F19,F28; ABL-F23..F25; DATA-F1..F14,F16..F18,F20,F21; R2 TRN-F6,F15,F17,F25,F28..F31,F33; ROOM-F7,F8.
- verifier already found: DET-F25 "per process" wrong (per instance); BND-F2 0.2.122 breaking unsupported. I re-check independently.
- method: WebFetch with "quote verbatim" prompt; verdict confirmed only with exact words.

## Verdicts (as I go)
