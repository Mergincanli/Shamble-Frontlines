# game-designer: fix round 3 for ADR 0005, "The ability and effect system" (2026-11-01)

Source: verifier-adr0005-verdicts.md, X2 (and engine-architect-0005-fixes-2.md, NOTE X2). The BEFORE was copied by
script from docs/adr/0005-content-abilities-assets.md at commit 57c43c2 (lines 1087–1090; the working tree is the same
file) and occurs once there. Line 1001 ("adds a / runtime no §6 item asks for.", option A3's Cons) does not present the
words as a quote, so it does not change.

### FIX X2
FILE: docs/adr/0005-content-abilities-assets.md
BEFORE:
  the closest shipped team-shooter precedent, Overwatch's scripted abilities, was not read (R1-ABL-OQ2). So the case
  against scripting rests on cost and on "no §6 item asks for a runtime", not on proof that a closed set is enough for
  a live game. And a closed set means each new mechanic Mustafa designs that lacks an operation waits for a reviewed
  Rust change. The content and ability spike's marks 1 and 2 test exactly this; OQ-0005-AB13 names the trigger.
AFTER:
  the closest shipped team-shooter precedent, Overwatch's scripted abilities, was not read (R1-ABL-OQ2). So the case
  against scripting rests on cost and on the research's own judgement that A3 "adds a runtime no §6 item asks for"
  (r1-simulation-core.md, Options §5, Question A, line 1798), not on proof that a closed set is enough for a live game.
  And a closed set means each new mechanic Mustafa designs that lacks an operation waits for a reviewed
  Rust change. The content and ability spike's marks 1 and 2 test exactly this; OQ-0005-AB13 names the trigger.
END
