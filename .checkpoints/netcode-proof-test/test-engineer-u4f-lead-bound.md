# test-engineer: one read of LEAD-BOUND's draft (u4f-lead-bound.md), 2026-11-18. Not a round.

## Verdict: SEND WITH EDITS (E1 and E2 needed; E3 to E5 Low, one-word fixes)

## Numbers checked (all right)
- 400 ms: ticks_for_ms_at(30, 400) = ceil(12,000 / 1,000) = 12; at 60 Hz 24 (ticks.rs:25-31).
- 240 cm: SS (Scene::stress) move_speed 600 cm/s (scene.rs:184, unit cm/s at :138) = 20 cm a tick at 30 Hz; 12 x 20 = 240;
  at 60 Hz 24 x 10 = 240 (the same 400 ms of walking). Sprint would be 900 cm/s (world.rs:484-486); no bot or lab code
  presses SPRINT today (only sim/src/replay.rs:502), so 240 holds for the lab's clients; E4 says so in plain words.
- Options: 600 ms = 18 ticks, 18 x 20 = 360 cm; 1,000 ms = 30 ticks, 600 cm. Right.
- Honest lead: P4's 9 fits under 12 - 2 = 10 (advisor :253-254); (d)'s "more than 11" is the same test (bound - 2 > 9).
- Catch-up kind under B: P0 lead 2, 8 - (2 - 1) = 7 free places, 8 distinct stamps 2..9, about 10 ticks (advisor :254-255).
- Idle bank: 12 + W (CMD_CATCHUP 8) = 20. Lag switch's 160 cm = 8 x 20 cm, a 30 Hz figure (CMD_CATCHUP is not rate-derived).

## Testable as stated
- f31's pins at 30 and 60 Hz: yes. The fence's argument order is swapped (E3); harmless in value because
  ms x tick_hz is symmetric, so no pin can catch the swap: the text must be right.
- Freeze script: testable once the header param has its name (5.6 B2; the builder's). For the build, not the entry:
  the script needs its own bite (a header or built default of 401 ms must make it fail).

## Unsaid change of a check's result (E1)
- The A1 bullet drops half of the advisor's A1 (advisor-u4f-brief.md:82-83, :90, :274): the fast clock's honest
  reference sums the same live ticks the measured movement sums. Without it a builder reading the entry keeps the
  reference over all ticks: each death then widens the movement check by its dead ticks (a 1,000 ms respawn = 30 ticks =
  600 cm), so a cheat that dies once gets 840 cm, 3.5 times the 240 cm he is asked to approve. With it a correct build
  passes (respawn runs after the movement step, world.rs:2108-2128, so a respawn tick has no step movement to lose) and
  a cheat still fails; the commands check (marks.json:5027) is untouched by A1 and still counts dead-tick commands.

## Misquote (E2)
- marks.json's movement check is named "moves no further than the honest client plus that many ticks of movement"
  (:5039), not "at most the server's ticks plus the lead bound"; only the commands check (:5027) has those words.

## Edits (exact old -> new)
E1 old: `tick in which the soldier is dead, or the respawn places it, adds no movement and is counted apart. Without this, the`
E1 new: `tick in which the soldier is dead at its start or end, or the respawn places it, adds no movement and is counted apart, and the honest reference counts the same live ticks, so a death buys a cheat no extra allowance. Without this, the`

E2 old (lines 24-25): `keep their words ("at most` + newline + `  the server's ticks plus the lead bound" of commands and of movement)`
E2 new: `keep their words ("at most the server's ticks plus the lead bound of commands" and "moves no further than the honest client plus that many ticks of movement")`

E3 old: `(ticks_for_ms_at(400, rate): 12 ticks at 30 Hz, 24 at 60 Hz)`
E3 new: `(ticks_for_ms_at(rate, 400): 12 ticks at 30 Hz, 24 at 60 Hz)`

E4 old: `12 ticks of top-speed movement (240 cm) more`
E4 new: `12 ticks of full-speed walking (240 cm at 600 cm/s; the test clients never sprint) more`

E5 old: `(more than 11 ticks)`
E5 new: `(more than 11 ticks at 30 Hz)`
