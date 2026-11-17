I finished rev 3.1 of the U4f brief. All 23 findings are addressed: 22 done in full, and SA3-2 done in part, with one part rejected on a code citation. I built and ran nothing. The only command was a read-only `python3 -I` check script.

**Files**
- Brief: /home/user/Shamble-Frontlines/.checkpoints/netcode-proof-test/u4f-brief.md (rev 3.1, 1,624 lines, sha256 738dc680…). A new first change-list block "rev 3 to rev 3.1" holds R1-R10 and the counts at rev 3.1.
- Notes: /home/user/Shamble-Frontlines/.checkpoints/netcode-proof-test/u4f-brief-r3-notes.md, section "Revision for the checks": the blocks done, then one line per finding with its reason or code citation.
- Check script: /home/user/Shamble-Frontlines/.checkpoints/netcode-proof-test/u4f-r3/r31_check.py. It finds P24's new start key once in advisor-u4df.md, every placeholder P1-P36 used with no unknown id, and no run of 7 of Mustafa's words outside section 12 item 3 (punctuation ignored).
- Cites are at head e86cf88. `git diff --stat 0ad9661 HEAD` over spikes/netcode-proof, ADR 0001, rules/netcode.md and the catalogue prints nothing.

**High findings**
- **TE3-1, done.** Under the controller, an honest client now gives a command for every stamp: at most 2 a step, and a "hold" (no command) to lower the lead. The old "only when the newest tick moved" rule stays for `with_lead` runs only. This is asked as U4F-Q7 (iii).
  - f5's bound is now "the lead bound less a margin", with carry 74's 1 % late rule.
  - New f36 (2 tests) checks -20's two share bars on a seeded P3 trace.
  - p5 and p6 read -20's two share checks by name; D15's bar is 0.1 %; a new stop is in section 1 item 5.
  - Not taken: the SERIES.md:63 reading route, because it changes what a bar's series computes and would go to Mustafa first (history :2125).
- **TE3-2, done.** 2.4 item 6 now says per kind what "rejected and counted" means: kind 3 is an added copy of an accepted sequence, and kind 7 is the command run with its press refused. 2.12 has its own kind-3 twin row, and f33's kind-3 and kind-7 cases are named.
- **SA3-1, done.** A join anchor under both options: a fresh slot's kind-2 reference is its join's tick (room.rs:681), tested by f13 (iii).
  - New U4F-Q18 recommends a past window for option A of W = `CMD_CATCHUP`. Commands older than that are counted late.
  - f38 and p20 run only if Q18 (a) is ruled.
  - Q8 and the LEAD-BOUND draft in section 12 item 6 now say "the bound plus W after an idle".
  - Q17 keeps the total-key recommendation, because the no-lead-bound bite run can still hold stamps 2^31 apart.

**Medium findings**
- **TE3-3, done.** 5.3 lists the five count checks that fail by length on every short run. Each bitten run names the one check that must fail, read by name, with a clean partner that passes it. New p21 is a clean clock run; D12 is rewritten.
- **TE3-4, done, partly another way.** 2.6 states the codec's two rules and a per-kind forging table. Kind 2 is forged at an equal tick, kind 1 stays within the codec's 255-tick step, and the fast clock's stamps never decrease. f15 and f16 assert every forged and twin `Input` encodes. Instead of TE3-4's "drop kind 1's later copies" route I used SA3-2's (the forger stores the honest command), so nothing is dropped.
- **TE3-5, done.** f16 compares fate logs entry by entry, leaving out each datagram's size and resends. A 9.1 row sends the reading of plan :389-390 to the advisor.
- **TE3-6, done.** The fast-clock schedule is one client, 1.5x from the measured start, ten back-to-back 60 s episodes. The suite's measured window is 610 s rather than "700 s plus settling", since no settling is needed. New f37 asserts the schedule at the judged settings, and 5.1's -21 citation is corrected.
- **TE3-7, done.** 2.10 gives every lab test a short scale in P2_SHORT's form. The episode length and connect count become lab settings, with a wall-time stop.
- **TE3-8, done.** 5.5 gains a build-phase row. The u4r-P3 pack and the shared-target part-clean happen at launch, before the builder starts (Q2 (a), section 12 item 13).
- **TE3-9, done.** P24's start key is now `word for word: "Write files with the`, found once.
- **SA3-5, done.** 5.5 packs only u4r-P3. Packing a U4f set's own folder waits for Mustafa's yes to KL-U4F, drafted in section 12 item 6; Q2 gains option (d).
- **SA3-2, done in part.** Done: kinds 1, 2, 4, 5 and 6 forge only the newest copy, a later copy of a refused sequence counts as a repeat, and kind 3 forges a copy of an accepted sequence.
  - Rejected: the twin "with no command and consecutive sequences". world.rs:1922-1925 leaves out only the press counters. The hash does carry each command's sequence: :1979 (projectile seq) and :1995-2009 (Shot and Hit events).
  - So U4F-Q13 (b)'s skipped index stays. Each `Input` leaves out any command older than a skip, which should be none at P0.

**Low findings:** all applied (TE3-10 to TE3-15, SA3-3, SA3-4, SA3-6 (1)-(4), SA3-7, SA3-8). None left.

**Counts now:**
- Tests: measured +107, bites build +122, negative cases 63. Conditional: U4F-Q6 (b) +4, U4F-Q18 (a) +1.
- Pre-runs: 15 (+4, +1, +1 by ruling). Questions: 18. Recheck rows: 50.
- Disk: about 3,608 to 3,728 MiB of logs plus the build phase.
- Planned size: about 4,750 added lines.
- Bars for Mustafa: unchanged (B3 recommended). KL-U4F goes to him only if D2 shows the room short.