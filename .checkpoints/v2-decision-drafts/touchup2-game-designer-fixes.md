# touchup2-game-designer fixes (F1b, F8, F9, F10 of fixcheck-d2; LOST-AB1-WHY, a lost line of my own)

All five blocks are written against docs/adr/0005-content-abilities-assets.md at 41665c7 (the docs as at 750d46d), each BEFORE copied by script, whole lines, occurring once; none overlaps another. Line change per block: F1b +1, LOST-AB1-WHY +1, F10 -1, F9 +1, F8 0; net +2 for ADR 0005 (2499 to 2501 lines). Old lines move by +1 from 0005:877, +2 from 883, +3 from 901 and +2 from 950 on; README's 23 pointers into ADR 0005 at line 1186 or later each move by +2 (OQ-0005-AB1 1186 to 1188), and its 8 earlier ones (below 871) stay. scripts/apply_fixes.py applies a block only to a document whose path ends in its FILE path, so dry-run it on a copy at <scratch>/docs/adr/0005-content-abilities-assets.md; on a copy named otherwise it skips all five and writes nothing.


### FIX F1b
FILE: docs/adr/0005-content-abilities-assets.md
BEFORE:
  - Need: VISION_V2 §6.3 ("combos and counters can emerge"); the immunity rule: LOOP_V2 §2 only.
  - What: a weapon definition holds the sim fields only (fire interval, fire mode, magazine, reserve, pellets, the
AFTER:
  - Need: VISION_V2 §6.3 ("combos and counters can emerge"); the immunity rule: LOOP_V2 §2 only.
- **AB-3. Weapons keep their own bookkeeping and share the damage pipeline (option B1).**
  - What: a weapon definition holds the sim fields only (fire interval, fire mode, magazine, reserve, pellets, the
END

### FIX LOST-AB1-WHY
FILE: docs/adr/0005-content-abilities-assets.md
BEFORE:
      lines 2471–2472). Need: VISION_V2 §3 and §4 (a budget, not a feature, as AB-10).
    names, with no generic-engine feature.
AFTER:
      lines 2471–2472). Need: VISION_V2 §3 and §4 (a budget, not a feature, as AB-10).
  - Why: §6.3 asks for GAS-like data, and the subset covers every legacy ability and weapon and the primitives §6.4
    names, with no generic-engine feature.
END

### FIX F10
FILE: docs/adr/0005-content-abilities-assets.md
BEFORE:
  - Need: VISION_V2 §6.5; the faction's list: §6.7.
  - Need: VISION_V2 §6.5.
AFTER:
  - Need: VISION_V2 §6.5; the faction's list: §6.7.
END

### FIX F9
FILE: docs/adr/0005-content-abilities-assets.md
BEFORE:
    - Live projectiles: the pool holds the content's worst case at the mode's N, so honest play cannot fill it. Per
      shooter that is at most ⌊lifetime ÷ fire interval⌋ + 1 live shots of each projectile weapon it can carry at once
      (carry slots are OQ-0005-AB1's), times the weapon's pellets; legacy's Molotov (fuse 1.4 s, r1:1373; fire interval
      1.05 s, LEGACY_TUNING.md, `WEAPONS`, lines 167–203) gives ⌊1.4 ÷ 1.05⌋ + 1 = 2 per thrower, and 2 × 48 = 96 at
      N = 48 (arithmetic). Refusing the new row (option (a)), counted, stays as the safety net, and a non-zero count is
      a content or capacity bug, not a rule a player meets. Reason: a shot that never leaves the gun and a projectile
      that vanishes in flight would both be seen, so the pool is sized never to need either; R2-BUD-OQ5's caps, once
      set, bound the content (a set whose worst case exceeds them fails the check), not the play.
AFTER:
    - Live projectiles: the pool holds the content's worst case at the mode's N, so honest play cannot fill it. Per
      shooter that is at most ⌊lifetime ÷ fire interval⌋ + 1 live shots of each projectile weapon it can carry at once
      (carry slots are OQ-0005-AB1's), times the weapon's pellets (at least 1); legacy's Molotov (fuse 1.4 s, r1:1373;
      fire interval 1.05 s and `pellets` 0, LEGACY_TUNING.md, `WEAPONS`, lines 167–203) gives (⌊1.4 ÷ 1.05⌋ + 1) × 1 = 2
      per thrower, and 2 × 48 = 96 at N = 48 (arithmetic). Refusing the new row (option (a)), counted, stays as the
      safety net, and a non-zero count is a content or capacity bug, not a rule a player meets. Reason: a shot that
      never leaves the gun and a projectile that vanishes in flight would both be seen, so the pool is sized never to
      need either; R2-BUD-OQ5's caps, once set, bound the content (a set whose worst case exceeds them fails the check),
      not the play.
END

### FIX F8
FILE: docs/adr/0005-content-abilities-assets.md
BEFORE:
    keeps the order deterministic whichever is chosen); and which act a press means where a pickup lies within reach of
    an uplink or a cache, since a steal is holding E (LOOP_V2 §4 step 4), ADR 0001 part 12 has E place a cache too, and
    a pickup is asked for through the same button edges: the pickup wins, the steal or the placing wins, or a tap takes
    the pickup and a hold steals. A full pickup pool follows AB-1's proposal for OQ-0004-11.
AFTER:
    keeps the order deterministic whichever is chosen); and which act a press means where a pickup lies within reach of
    an uplink or a cache, since a steal is holding E (LOOP_V2 §4 step 4), picking up and placing a cache (steps 2 and 3)
    name no key yet, and a pickup is asked for through the same button edges: the pickup wins, the steal or the cache's
    act wins, or a tap takes the pickup and a hold steals. A full pickup pool follows AB-1's proposal for OQ-0004-11.
END
