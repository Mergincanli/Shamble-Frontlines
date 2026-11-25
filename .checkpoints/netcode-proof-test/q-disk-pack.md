# DISK-PACK: a question for Mustafa (the main session, 2026-11-25 06:0x UTC)

**The question, in plain words:** may I pack seven more closed sets of test logs into archives, to free about 2.3 GiB
of disk?

**Why it matters (numbers corrected 06:2x UTC):** U7's check filled the build cache to about 9.0 GiB (9,013 MiB used;
2.9 GiB of it incremental), leaving 4,250 MiB, just over the build runner's 4 GiB floor (main-run.sh stops under it).
I emptied the debug cache by the approved part-clean (9,094 MiB freed): 13,344 MiB free now, the cache cold. From now
on the builds run with incremental off, so a warm cache is about 6.1 GiB and about 7.1 GiB stays free: still under the
8 GiB that Builder 2's merge needs at its check (NB2-C5), before U7's R-half and U8's fixes add their own builds. Each
time a bar or the floor is missed I must empty the cache again, and the next build is cold: about 20 to 40 minutes.

**Measured numbers (2026-11-25 05:4x UTC, `du -sh`):**
- The seven sets, all from closed units, none covered by your packing words so far (the card at 74bf901):
  u4r-P2-8ee453c22882 741 MiB, u4r-P2-fc411116c1f1 623, u4f-20-350677f0aef2 434, u4f-20-96de6acf8223 434,
  u4f-22-7192607d99a1 366, u4r-P3-d66e23cd78f2 298, u4r-P3-fd4612050cd6 291: 3,187 MiB raw.
- Earlier packs kept about 27 %: u4r-P2-ab10f058a828 779 -> 207 MiB, u4r-B1-64f3307d5c7f 821 -> 217,
  u4r-P3-0f066338dd8d 519 -> 143 (the archives in .gate/netcode-proof-test/kept-logs/). So all seven free about
  2,330 MiB; fc41 and fd46 alone about 670 MiB.
- Not in the question: u4f-47-b57221ed139d (51 MiB) stays unpacked until Builder 2's merge re-judges it; the evidence
  folders (.gate/u4f 984 MiB, .gate/u4r 990, .gate/brawl1-evidence 608, .gate/u4r-d4 320) stay as they are; the agent
  copies (.claude/worktrees, 1.6 GB, none holding a build folder) are removed only when merged, by plain
  `git worktree remove` (CHECKPOINTS.md §10).

**Options:**
- (a) Pack only fc411116c1f1 and fd4612050cd6 (the two the card names as waiting on you): about 670 MiB back.
- (b) Pack all seven, one at a time, between runs, by the kept-logs rule (kept-logs.mjs; CHECKPOINTS.md §11), each
  only after I confirm it is closed: about 2.3 GiB back.
- (c) Pack nothing: I work with part-cleans, each costing a cold rebuild.

**My recommendation:** (b). The archives keep every log (KEPT-LOGS), so nothing is lost; with it, about 9.4 GiB stays
free beside a warm cache, over Builder 2's 8 GiB, which saves several cold rebuilds before R1.
