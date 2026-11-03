# touchup2-game-designer fixes (F1b, F8, F9, F10 of fixcheck-d2)


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
