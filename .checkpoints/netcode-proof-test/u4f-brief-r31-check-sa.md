# U4f brief rev 3.1: security-auditor round 2 check (what changed only)

Task: round 2 on `git diff e1508bc 9f76a02 -- .checkpoints/netcode-proof-test/u4f-brief.md`. Head 9f76a02.
Read only; no builds, no scripts except read-only python3 -I under u4f-r3/r2/.

## Plan
1. Read the diff (R1-R10) and the reviser's notes.
2. Item 1: SA3-1..SA3-8 each, fixed in full / in part / not, with rev 3.1 line.
3. Item 2: SA3-2 rejection (state hash carries seq?): sim/src/world.rs:1922-1925, :1979, :1995-2009; proto/src/input.rs encoder rules; twin construction.
4. Item 3: SA3-1 fix as a defence: join anchor (room.rs:681), W = CMD_CATCHUP, bank cases (idle, rejoin, replaced session, kick-rejoin, u32 wrap, signed key), honest first command before snapshot (client/src/lib.rs:429), arrival cap, LEAD-BOUND text.
5. Item 4: per-kind forging table 2.6, kind 3 construction, kind 4 widened arithmetic.
6. Item 5: packing rule vs CHECKPOINTS.md §11, PACK-READ (a) history.md:2212, R4; no-socket rule.
7. Item 6: anything broken / inconsistent.
8. Findings + coverage table; return schema.

## Blocks done
