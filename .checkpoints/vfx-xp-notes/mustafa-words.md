# Mustafa's words for the docs pass after B.2, word for word

Both items say "For the next docs pass" (2026-11-06). Each block is copied byte for byte from its source.

## 1. The VFX editor item
Source: his chat message in the B.2 day session (session_0173vVo7FcYUpbWgRW8oQyHU), user event
0f5f8a45-d668-49d9-8689-9a5f5df40cbc, 2026-11-06T23:27:12Z; saved in B.2's notes at a79fef1
(`.checkpoints/b2-community-servers/mustafa-2026-11-06-vfx.md`) and queued in STATUS's "Queued, in Mustafa's words".

```text
For the next docs pass (Mustafa, 2026-11-06): record a VFX editor in EDITOR_V2 as a future tool, after Editor v2's map features. Decided principles, design later:
- Essential: the game's punch depends on it. Inspired by Unreal's Niagara (emitters with a stack of modules: spawn, velocity, colour over life, size over life, gravity, collision), but without its clutter.
- What you see is what you get: the editor uses the game's own renderer and lighting, and saves the same effect data the game loads, triggered by the game's events (hits, blasts, deaths).
- Test in context: trigger a shot or blast in a live preview, from the real game camera, and keep editing while it plays.
- Easy to pick up: each effect opens with a few big sliders (size, intensity, colour, duration); the full emitter and module stack folds open on request. One clean panel; curves edited inline. Presets to start from (muzzle flash, explosion, smoke, sparks).
- Whole impact moments: one asset can bundle particles, sound, camera shake and hit-stop, timed together, so punch is authored in one place.
- Faction colours: effects use the same role colour slots as units, so one explosion recolours per faction.
- Cost and quality: each effect shows its cost against a budget, and has quality tiers (Low drops extras but stays readable).
- Comic flavour: ink-outlined smoke, impact frames, halftone flashes, onomatopoeia.
- A VFX style guide comes first, from a separate reference study, so the tool fits our look.
- Built to iterate; no bloat.
```

## 2. The private-match progression item
Source: his chat message in the same session, user event 634a1bb3-cc61-492c-a40d-f7fa02800e42,
2026-11-06T23:50:53Z (read with list_events). It arrived while PR #42 waited for its merge, so it was kept off that
branch and first committed here.

```text
For the next docs pass (Mustafa, 2026-11-06): add to VISION_V2's community-servers note, on progression in private (rented) matches:
- [DECIDED] With bots in the match: 25 % of normal XP; humans only: 100 %.
- Open, Advisor's suggestions: (1) weigh XP per action (kills on bots give little), so public matches with bot fillers aren't punished; (2) against friends trading easy wins: full XP only with a minimum number of humans per team, plus a daily XP cap on private matches.
```

## 3. His answer VFX-ORDER, 2026-11-06 (in his "merge #42" message before the advisor's approval)
```text
VFX-ORDER: (a).
```
