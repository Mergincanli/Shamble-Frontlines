# U4r build, copy 5: two points for the advisor (the main session, 2026-11-14 about 02:05 UTC)

Copy 5 (agent a507d682db446aa0a, 01:41-02:00 UTC) made the third ruling's code changes (green in both builds) and ran
point 3's pair once each, then stopped by the ruling's stop rule on one differing item. Its notes:
`.checkpoints/netcode-proof-test/u4r-build.md` (1,325 lines, an exact copy of the worktree's; copy 5's part is
"## Copy 5" on, its state section :1292 on). The snapshot: `wip/u4r.patch` (26 files on 2fbec77, sha256
707cc3df3c409d5fd3e21249567acc1b54a67f5833f2ee4df0abcb383a4bdaa5). The logs: `.gate/u4r/c5-*.log`; the runs under
`.gate/u4r/r2/tick16d/` and `.gate/u4r/r2/tick16-clean/`. Your third ruling: `advisor-u4r-build3-full.md`.

Copy 5 is resumed meanwhile on what does not depend on these points (point 8 (A)'s runs, c-u4r-r2.mjs's lines but its
tick16 expected text, the report words, and ruling 3's two script items below); r5 and the end checks wait for this
ruling. For each point the ruling gives: the decision, the severity, what the builder does (to be sent by message),
what the main session does, and whether it goes to Mustafa, and why.

## 1. P3-CAP: the tick16 bite run's `after_capped` is false (ruling 3, point 3's expected outcome said true)

- The run (`--tick-start 64651 --warmup-s 10 --measured-s 20 --after-max-s 5 --role bite --bite PM-0001-17/tick16`,
  bites build; c5-p3.log): ticks_below_wrap 885, the wrap at measured tick 585; run valid, PM-0001-2's share 1.0 (600 of
  600 measured ticks); -17 FAIL by decoded_hash_mismatch_count, 2,880 mismatches ("shown"); mirror 0 breaches; contract
  PASS; **after_capped false**: 946 ticks, 46 after the measured phase, before the 150-tick cap.
- Copy 5's reading from the logs: the flag drops to 0 at 65,567 (wrap + 31, as in tick16c); at 65,596 (wrap + 60) all
  48 overflow resyncs send full states, which resolve the 15 post-wrap measured ticks as lost (measured incomplete 720 =
  48 x 15), so the run ends by carry 62 (e)'s normal path.
- The clean partner (`--tick-start 64936`, measured build) matched every expected item.
- Copy 5's options: (a) after_capped is not required at this setting (the overflow resync resolves the measured ticks
  before the cap) while carry 73's FAIL, never INVALID, holds; (b) a new setting, by a new ruling, never a re-run.

## 2. CR-ID: the class-audit exception's bite id is read at run time (ruling 3, point 6 (1))

- contract.rs fails a run whose class audit holds a breach, except for r4's owner-to-all bite. To keep brief section 4's
  rule that npt-lab's sources never spell another crate's bite id, the code does not write `catalogue-5/owner-to-all`:
  it finds the id at run time as the one entry of `npt_room::BITES` outside the `PM-` form (room/src/lib.rs:32-46). So a
  measured-build contract never applies the exception. Copy 5's options: keep this, or spell the id (check.mjs's scan
  would not flag it, being outside the `PM-` form).

## 3. Two script items the main session reads as ruling 3's own (confirm or correct)

- c-u4r-runs.mjs's judged tick16 bite run still starts at 63,736: ruling 3's point 3 (main session item 4) says B12's
  tick16 script computes tick_start = 65,551 - (warm-up + measured ticks) (47,251 for 300 + 18,000).
- c-u4r-runs.mjs still runs -18's no-fallback bite on B1: ruling 3's point 1 moves -18's bitten judged run to U4g.
- The main session sends both to copy 5 now as ruling 3's items. Say if either reads otherwise.
