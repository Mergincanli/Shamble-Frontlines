### FIX D3C-1
FILE: docs/adr/0001-multiplayer-netcode.md
BEFORE:
7. At P2, with the rewind cap at the honest rewind's p99 the proof test measured at P2 (PM-0001-49; the overlay names
   it, so the cap clips almost no honest shot), shoot the sprinting target for two minutes. Expected: when his crosshair is on the target as he sees it, the
   shot hits; when it is off, it misses. The shot (flash, tracer) is instant; the hit marker comes a moment later, when
   the server confirms ("the shot feels instant; hit feedback arrives when the hit is confirmed", LOOP_V2 §7).
AFTER:
7. At P2, with the rewind cap at the honest rewind's p99 the proof test measured at P2 (PM-0001-49; the overlay names
   it, so the cap clips almost no honest shot), shoot the sprinting target for two minutes. Expected: when his crosshair
   is on the target as he sees it, the shot hits; when it is off, it misses. The shot (flash, tracer) is instant; the
   hit marker comes a moment later, when the server confirms ("the shot feels instant; hit feedback arrives when the hit
   is confirmed", LOOP_V2 §7).
END

### FIX D3C-1b
FILE: docs/adr/0001-multiplayer-netcode.md
BEFORE:
10. At P2, repeat step 7 with the rewind cap at 200 ms, then at 250 ms, then again at step 7's measured cap (the overlay
    names the cap). Expected: with 200 and 250 ms, shots that looked like hits but missed, since at P2 an honest shot
    needs 233–383 ms of rewind (part 8) and both caps clip most of them; with the measured cap, hits as in step 7. Record: what he noticed with each cap
    (for OQ-0001-5's later Feel Lab weighing; no pick here).
11. At P2, run from open ground into cover while bots shoot at him, 10 runs with each of step 10's three caps. Expected: being hit after
    reaching cover on his own screen is rare, and never long after. Record: how often it felt unfair with each cap
    (OQ-0001-5, the victim's side).
AFTER:
10. At P2, repeat step 7 with the rewind cap at 200 ms, then at 250 ms, then again at step 7's measured cap (the overlay
    names the cap). Expected: with 200 and 250 ms, shots that looked like hits but missed, since at P2 an honest shot
    needs 233–383 ms of rewind (part 8) and both caps clip most of them; with the measured cap, hits as in step 7.
    Record: what he noticed with each cap (for OQ-0001-5's later Feel Lab weighing; no pick here).
11. At P2, run from open ground into cover while bots shoot at him, 10 runs with each of step 10's three caps. Expected:
    being hit after reaching cover on his own screen is rare, and never long after. Record: how often it felt unfair
    with each cap (OQ-0001-5, the victim's side).
END
