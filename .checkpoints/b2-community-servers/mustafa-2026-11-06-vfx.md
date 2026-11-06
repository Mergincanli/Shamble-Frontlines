# Mustafa's VFX editor item, 2026-11-06, word for word

Source: his chat message in this day session (session_0173vVo7FcYUpbWgRW8oQyHU), user event
0f5f8a45-d668-49d9-8689-9a5f5df40cbc, created 2026-11-06T23:27:12Z (read with list_events). It arrived while B.2 was
in its docs-keeper pass. "For the next docs pass": a docs change-set of its own after B.2; B.2 records it in STATUS's
"Queued, in Mustafa's words" so it lives in the repository.

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
