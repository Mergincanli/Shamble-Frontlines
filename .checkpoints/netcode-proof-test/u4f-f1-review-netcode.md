# U4f-F1 review round, netcode-architect seat (2026-11-20)

Read only. Brief: u4f-f1-panel-brief.md. Diff: wip/u4f-f1-build.patch (sha256 37d87a10bed52b81..., checked), base cb7e374.

## Plan
1. Read the patch whole; the build record (u4f-f1-build.md); the conditions 1-9 (advisor-u4f-f1-plan.md:186-221).
2. ADR 0001 part 5 (docs/adr/0001-multiplayer-netcode.md:220-275): does "each edge through the server's own state"
   match the new run_command (reload, fire, throw, melee; clock once; one shot per command)?
3. The room's counts: run_one's seen/lost/doubled from the shared fn; SERIES.md:42's meaning; option A and option B paths
   (input.rs kind 5, the late drops, the repeat path); doubled on a repeat.
4. The expected -20 figures (advisor-u4f-f1-plan.md:119-140): does the diff keep state-neutral for -20's bots (fire held
   only), so per-tick h equal to 350677f's? Anything in the diff that could move a hash or a summary line.
5. A session's first command (ADV-U4FF1-3): what F1 does now, what the build notes say, what U4f2 must carry.
6. Findings, verdict.

## Blocks
