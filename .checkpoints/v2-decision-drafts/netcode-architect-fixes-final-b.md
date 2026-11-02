# ADR 0001, follow-up batch b (netcode-architect), 2026-11-02, from game-designer's ability batch (d57dc5a)

Every BEFORE is whole lines copied by script from ADR 0001 at 75615d1 (= HEAD); each occurs once; applied in order on a copy, they succeed and add 2 lines.

### FIX GD-PICK-VIS (part 12: its home is OQ-0005-AB15)
FILE: docs/adr/0001-multiplayer-netcode.md
BEFORE:
    and is sent to the owner when it changes; owner is the starting class, and who else sees picks is GD-PICK-VIS
    (Mustafa's); what a pick changes inside the prediction scope reaches the owner in the own-state section (part 4).
AFTER:
    and is sent to the owner when it changes; owner is the starting class, and who else sees picks is ADR 0005's
    OQ-0005-AB15 (GD-PICK-VIS, Mustafa's); what a pick changes inside the prediction scope reaches the owner in the
    own-state section (part 4).
END

### FIX GD-2 (part 12: the purchase conditions cite AB-5)
FILE: docs/adr/0001-multiplayer-netcode.md
BEFORE:
    LOOP_V2 §3, power-plant pricing included) and stands at the team's own buy terminal (LOOP_V2 §2, §5), these
    conditions being rule data (ADR 0005); it reports the result to the owner as an event, counts refusals as it
AFTER:
    LOOP_V2 §3, power-plant pricing included) and stands at the team's own buy terminal (LOOP_V2 §2, §5), these
    conditions being rule data (ADR 0005 AB-5); it reports the result to the owner as an event, counts refusals as it
END

### FIX GD-8, LOOP_V2 wording (part 12: which key LOOP_V2 names)
FILE: docs/adr/0001-multiplayer-netcode.md
BEFORE:
    a press means where a pickup lies within reach of an uplink or a cache (E also steals and places, LOOP_V2 §4) is
AFTER:
    a press means where a pickup lies within reach of an uplink or a cache (a steal is holding E, LOOP_V2 §4 step 4;
    picking up and placing a cache, steps 2 and 3, name no key) is
END

### FIX GD-2 (the `Buy` row: the conditions cite AB-5)
FILE: docs/adr/0001-multiplayer-netcode.md
BEFORE:
| `Buy` | client → room | control stream | n/a | when the player buys | unit or vehicle id | the id exists in the mode's content; the team has its tier; the player can pay the price; the player stands at the team's own buy terminal (part 12; the conditions are rule data, ADR 0005); refusals counted as for picks |
AFTER:
| `Buy` | client → room | control stream | n/a | when the player buys | unit or vehicle id | the id exists in the mode's content; the team has its tier; the player can pay the price; the player stands at the team's own buy terminal (part 12; the conditions are rule data, ADR 0005 AB-5); refusals counted as for picks |
END
