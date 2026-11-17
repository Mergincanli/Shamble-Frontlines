# U4f brief rev 3.1: test-engineer check, round 2 (what changed only)

Target: .checkpoints/netcode-proof-test/u4f-brief.md at 9f76a02 (rev 3.1, 1,624 lines, sha256 738dc680...),
against rev 3 (e1508bc). Code cites at head (spikes/ unchanged since 0ad9661).

## Plan
1. Read my round-1 findings (check-te-r3-full.json, TE3-1..15) and the diff e1508bc..9f76a02 of the brief.
2. Per finding: fixed in full / in part / not, with the rev 3.1 line.
3. Soundness of the two Highs' fixes: TE3-1 (contiguous stamps, 2 a step, hold, redundancy 8, P3 5 % each way,
   -20's two shares, f36 trace, stamps_skipped); TE3-2 (2.4 item 6, 2.6 table, 2.12 twin rows per kind).
4. New items: (a) join anchor vs client/src/lib.rs:429 stamping; (b) option A past window W=CMD_CATCHUP vs
   resent copies / lag switch / p19 holds, effect on p13, p19, p20; (c) 5.3 by-name reading; (d) f36-f38,
   p20, p21, 2.10 scale rule, 5.5 disk row vs disk reading.
5. Counts: +107 tests, +122 bites, 63 negatives, conditional deltas, 15 pre-runs.
6. Anything broken: counts, cites, placeholders, questions, section 12.
7. Findings + StructuredOutput.

## Blocks (a line after each)
