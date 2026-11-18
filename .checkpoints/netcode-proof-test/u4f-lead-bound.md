# LEAD-BOUND: the lead bound's first value, for your yes (U4f; draft for the advisor's and the test-engineer's read)

Drafted 2026-11-18 by the main session from the advisor's list (advisor-u4f-brief.md:336-361; U4F-Q8 (iii) (e)). After
one read by the advisor and the test-engineer (no round), it goes to Mustafa in the day's message; nothing that reads
the bound is built before his yes in his own words.

## The question, in plain words
May U4f's room refuse any command a client stamps more than **400 ms ahead of the server** (12 ticks at 30 Hz, 24 at
60 Hz)? The same number sets how much a cheating client whose clock runs 1.5 times too fast can gain before -21's two
fast-clock checks fail: at most **12 more commands, and 12 ticks of top-speed movement (240 cm) more, per 60 s** than the
server's ticks allow. Why it matters: it sets how loose those two checks are, and, under input option B, how much
waiting in the input buffer a stamp far ahead can buy (U4h's view-time estimate caps that wait at the honest lead, not
at this bound: a carry to U4h).

## The value as it would be built (fixed at your yes)
```text
lead bound = 400 ms: a room setting in ms, turned into ticks at the room with the rate passed in
(ticks_for_ms_at(400, rate): 12 ticks at 30 Hz, 24 at 60 Hz), pinned at both rates by a unit test (f31),
recorded in every run's header as a report-only parameter (its new SERIES.md row: 5.6 B2).
A command stamped more than the bound ahead of the room's tick is refused and counted as kind 1 (2.4), never run.
```

## What it does not change
- No marks.json text, no SERIES.md row's words and no judge file: the two fast-clock checks keep their words ("at most
  the server's ticks plus the lead bound" of commands and of movement); only the bound they read gets its first value.
- Under input option A, after a client sends nothing for a while, the room's own limit is the bound plus the past
  window (8 more, 20 commands in all), while the checks still allow 12; the judged fast-clock client never idles.
- The limit holds per player slot across rejoins (the advisor's U4F-Q18 ruling), so rejoining does not refill it.
- **How the two movement checks read "movement"** (the advisor's A1): only the movement the simulation's step makes. A
  tick in which the soldier is dead, or the respawn places it, adds no movement and is counted apart. Without this, the
  first respawn of the lag-switch or fast-clock client (metres in one tick, against 160 cm allowed) would fail a correct
  build. This reads ADR 0001:1611's "moves" and goes on the readings PR's list.

## The options, with their arithmetic
- **(a) 400 ms (12 ticks): recommended** (the advisor's ruling on U4F-Q8). It is above every honest lead: the largest
  honest lead is 9 ticks (profile P4), under 12 less a margin of 2. It lets the catch-up cheat be built under option B
  (about 10 ticks are needed at P0). It is the smallest round value that does both.
- (b) 600 ms (18 ticks): the checks allow 18 commands and 360 cm more per 60 s.
- (c) 1,000 ms (30 ticks): 30 commands and 600 cm more per 60 s.
- (d) another value: it must exceed every honest lead plus a margin (more than 11 ticks) and be at least about 10 ticks.

## The planned size
U4f is size M (about 3,000 added lines for earlier M units). This brief's estimate after the advisor's ruling: **about
5,150 added lines (4,200 to 6,200)**. Your stop stays: past 6,000 the build stops and you get three lines (what grew,
why, the cheaper option).

## The runs it governs, the freeze, what stays open
- **Runs:** -21's two fast-clock parts (3 measured runs under each input option, and the not-older-than bite run under
  option A), and the kind-1 refusals counted in every -21 run.
- **Freeze:** the value is recorded at your yes (its line's sha256 on the card). Before the judged runs a script shows
  the room's built default and every judged run's header hold it. Any change after your yes comes back to you. The value
  is never changed to fit a run's result (your C62H-BOUND rule, applied the same way).
- **Open:** nothing else for you now. A disk entry (KL-U4F) comes only if the disk is short after the build.
