# U4f2 review round 1: the netcode seat (netcode-architect), 2026-11-20

## Plan
1. Read the panel brief, u4f2-brief.md rev 1.2, u4f2-build.md (done), the advisor's brief check (Q1-Q5 rulings), the
   plan's -21 entry and marks.json's -21 checks (what each check reads).
2. Read the patch whole (forged.rs, bots lib.rs, scenario.rs, room input.rs/room.rs, contract.rs, client lib.rs, the
   scripts), with the forged suite against ADR 0001 / plan rev 3.3 (refused by server state, counted, attributed).
3. F-B: trace kind 7's item path (bots Forge::reload / fire items) -> room -> sim run_command (world.rs:1353 `!s.alive`
   return, respawn's weapon reset) -> forged.rs's kind-7 rule (Cooldown/Reload records). Find the cause in the code,
   cross-check with the hand-long logs if they carry anything, and propose a fix inside the plan (no pass mark, judge or
   marks.json change).
4. F-A (lag client standing), C-U4F2-5's route (c) against Q1, the fixed-lead endpoints, HOLD_STEPS vs carry 39.
5. Findings, verdict.

## Blocks
(written as each finishes)
