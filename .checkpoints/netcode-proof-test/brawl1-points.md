# BRAWL-1: the builder's points for the advisor (facts and options only; no option preferred)

Written by the main session on 2026-11-15 from the builder's notes and logs. Check every fact yourself: the read copy
is `/home/user/Shamble-Frontlines/.claude/worktrees/brawl1` (detached at 352a245, the change uncommitted; the same
change is `.checkpoints/netcode-proof-test/wip/brawl1.patch`, sha256 ee2973aa…, committed at 0e2bf9a). The builder's
logs are under `/home/user/Shamble-Frontlines/.gate/u4r/brawl1/`. Its notes are `brawl1-build.md` and its report,
word for word, is `brawl1-build-report.md` (both in this folder).

## Point 1 (TICKS-NAME): the form of RR4-L1's constant

- **The ruling:** advisor-u4r-r2-full.md:92-97 (RR4-L1): hash.rs gets `const MAX_ENTRY_TICKS: u32 = 36_000;`, with
  the refusal message given there and one r10_fp3 case.
- **The rule it meets:** check.mjs:755-771 in the read copy. `TICK_NAMED = /_TICKS$|CATCHUP/`. A const or static
  outside npt-sim whose name matches, and which is not in `NOT_TICK_COUNTS`, must be derived (check.mjs:785-806). The
  builder's first proof run, `p1-a-check.log`, failed the deps step on Part 1's code; check that log for the line.
- **Carry 39** (u4s-carries.md:169-176): a tick count needed outside npt-sim goes into npt-sim as `<name>_at` plus its
  constant (carry 18's route), and the crate uses it by path.
- **As built:** hash.rs:239 `const MAX_ENTRY_TICK_COUNT: u32 = 36_000;`. The refusal is at hash.rs:338-342, with the
  ruled message text (it prints the constant). The r10_fp3 cases are at hash.rs:1651-1668.
- **Options the builder listed:** (a) MAX_ENTRY_TICK_COUNT, the literal 36,000, as built; (b)
  `MAX_ENTRY_TICKS = 4 * npt_sim::ticks_for_ms(300_000)`, which check.mjs accepts and carry 39 bars; (c) add
  MAX_ENTRY_TICKS to check.mjs's NOT_TICK_COUNTS. That is a check.mjs change, and the round-2 ruling's step 18 homes
  the check.mjs follow-up "at check.mjs's next change".
- **Not listed by the builder:** carry 39's own route, a constant in npt-sim used by path.

## Point 2 (PROOF-E): proof (e)'s two differences (condition 6 (ii))

- **Condition 6 (ii):** the precheck tests' --nocapture output (r6, the late-command bite, wallclock), captured before
  Part 1 and after, must diff identical; any difference comes to the advisor whole.
- **The logs:** before, at `<base>`: `p1-before-a.log` and `p1-before-b.log` (two runs, exit 0 each). After Part 1:
  `p1-after-e.log`, then `p1-after-e2.log` on the final Part 1 code (after the check.mjs fixes of point 1 and point 3).
  The normaliser the builder used is in brawl1-build.md:52.
- **The differences the builder reports:**
  - (i) the wallclock bite's comparer and result lines differ in the L1 leg's World hash. The two "before" runs A and B
    already differ there. The bite XORs aims with wall-clock ns (server pump.rs:228-246).
  - (ii) libtest's "filtered out" counts: 52 to 53 (measured build) and 58 to 59 (bites build), from RR3-L2 (a)'s new
    test.
- **Options:** accept proof (e) with these two differences; or name another comparison.

## Point 3: r10_carry_82's boundary

- **The test:** hash.rs:1554-1577 in the read copy (`r10_carry_82_the_manifest_refuses_a_bad_name_and_ticks_above_
  bytes_over_12`). At `<base>` it used bytes 799,693, so its ticks-at-bytes/12 boundary was 66,641 ticks. That is
  above the new 36,000 cap, so the cap would refuse the entry first.
- **As built:** the test rewrites the entry to 360,000 bytes, a boundary of 30,000 ticks, below the cap.
- **The facts workflow's text** (brawl1-facts-full.md:113-116) named the synthetic 36,000 entry and did not name this
  test.

## Also recorded by the builder (no option asked)

- Two runs ran 0 tests because of the selector: `p1-before-1.log` (`--lib`) and `p2-full-release.log` (`--exact`
  without the module path). Both were run again with the right selector (`p1-before-a.log`, `p2-full-release-2.log`).
- Proof (a)'s first run, `p1-a-check.log`, failed the test and deps steps on Part 1's own code. Part 1 was fixed and
  check.mjs run again on the changed code (`p1-a-check2.log`). Proofs (b) to (e) are on that same code state.
- The size: `git diff --numstat` in the read copy gives 875 insertions and 123 deletions in 10 files (998 changed
  lines, the main session's SERIES.md row of 6 lines included), against about 700 planned (450-750 with your
  conditions). Your "well past" line is more than about 1,400.
- Pattern (iii) (condition 3 (iii), printed only): no area != 0 in any seed. The largest distance from the centre is
  1,832 cm in each of seeds 1-3, each at a burst's end.
