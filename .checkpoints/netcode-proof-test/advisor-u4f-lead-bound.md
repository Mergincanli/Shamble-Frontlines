# Advisor: one read of LEAD-BOUND's draft (u4f-lead-bound.md), 2026-11-18, no round

**Verdict: SEND WITH EDITS.** All ten points are present; the tick and cm arithmetic holds (ticks.rs:25-31: 12/24, 18,
30; 20 cm a tick at scene.rs:184's 600 cm/s; P4's 9; about 10 at P0; 12 + 8 = 20). No word of Mustafa's is retyped as
his. Five edits, each old text found once in the draft (line breaks and indents as written):

**E1 (a reading that changes what two checks count sits under "What it does not change").** Old:
`- **How the two movement checks read "movement"** (the advisor's A1): only the movement the simulation's step makes. A`
New (a blank line, the heading, then the same line):
```
## One reading that does change what two checks count (shown so you see it; not a separate question)
- **How the two movement checks read "movement"** (the advisor's A1): only the movement the simulation's step makes. A
```

**E2 (the movement check's words misquoted).** Old:
```
- No marks.json text, no SERIES.md row's words and no judge file: the two fast-clock checks keep their words ("at most
  the server's ticks plus the lead bound" of commands and of movement); only the bound they read gets its first value.
```
New:
```
- No marks.json text, no judge file and no existing SERIES.md row's words change (the new rows, the header value above
  among them, are report-only: 5.6 B2). The two fast-clock checks keep their words ("at most the server's ticks plus the
  lead bound of commands"; "moves no further than the honest client plus that many ticks of movement"); only the bound
  they read gets its first value.
```

**E3 (the frozen fence's call has its arguments swapped: ticks.rs:25 takes the rate first; the result is the same).**
Old: `(ticks_for_ms_at(400, rate): 12 ticks at 30 Hz, 24 at 60 Hz)` New: `(ticks_for_ms_at(rate, 400): 12 ticks at 30 Hz, 24 at 60 Hz)`

**E4 ("top speed" is the sprint, 1.5 times the walk: world.rs:484-486, which would make it 360 cm; 240 cm is walking).**
Old: `12 ticks of top-speed movement (240 cm) more, per 60 s**` New: `12 ticks of full-speed walking (6 m/s: 240 cm) more, per 60 s**`
For the fold (L1), so the number holds: U4F-Q11 (iii) (c)'s fixed inputs walk, no sprint or crouch (the brawl bots hold
fire, so never sprint: bots/src/lib.rs:292).

**E5 (plain words: the options unexplained, "20 commands in all" reads as a total).** Old:
`under input option B, how much` New: `under input option B (the room keeps each command until the tick it is stamped for), how much`
Old:
```
- Under input option A, after a client sends nothing for a while, the room's own limit is the bound plus the past
  window (8 more, 20 commands in all), while the checks still allow 12;
```
New:
```
- Under input option A (the room runs each command at its first tick after it arrives), after a client sends nothing
  for a while it also takes commands stamped up to 8 ticks back, so its own limit is 20 more commands than the server's
  ticks, while the checks still allow 12;
```
