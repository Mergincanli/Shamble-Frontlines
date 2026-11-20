# U4f2 review round 1: rust-reviewer seat (2026-11-20)

Patch: `.checkpoints/netcode-proof-test/wip/u4f2-build.patch`, sha256 e9a0ae3e3f48af18ea7a947fdf9aba849061fa32fbd91876a948c46f3046406b
(confirmed by sha256sum at the start of this review). Built tree read by absolute path:
`/home/user/Shamble-Frontlines/.claude/worktrees/agent-aa62d31b6fa5b9807`.

## Plan
1. Read the panel brief (done), u4f2-brief.md rev 1.2 (sections 1-9), u4f2-build.md, the security seat's file
   (SA-U4F2-1, SA-U4F2-2), and my last seat file (u4f-review-rust.md, u4f-f1-review-rust.md) for baselines.
2. Read the patch whole, file by file: npt-bots (the forgers), lab/src/forged.rs, npt-room (`RefusalRecord`,
   `CommandRefusal`), npt-sim changes (C-U4F2-5, `HOLD_STEPS`), the run scripts, the doc-link fix.
3. Checks, each a block below: (a) the §4.11 bans (HashMap/HashSet, clock reads, floats in the room) by grep over the
   built tree; (b) integer arithmetic, panic and overflow paths in the forgers and the attribution; (c) wasm32 vs
   native identity; (d) allocation in hot paths (per-command refusal records); (e) clippy-class issues by reading;
   (f) the `HOLD_STEPS` rename vs check.mjs:760 and carry 39; (g) the doc-link fix; (h) SA-U4F2-1 and SA-U4F2-2
   verified or refuted from the code.
4. Findings RR-U4F2-n, verdict.

## Notes (one line per finding as it lands)
