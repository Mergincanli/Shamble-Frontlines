# LOOK V2: the look brief

Written by Mustafa and the Advisor on 31 October 2026, from Mustafa's voice notes. It is the input for the rendering
decision (ADR 0003) and the asset pipeline (ADR 0005). Status tags as in LOOP_V2.md: `[DECIDED]` Mustafa decided it ·
`[DEFAULT]` stands unless vetoed · `[OPEN]` needs a decision.

## 1. In one line
`[DECIDED]` **"XIII meets Red Alert 3, lit like Planetary Annihilation":**
- cel-shaded comic rendering with bold ink;
- iconic, RA-style unit silhouettes;
- clean, well-lit scenes with emissive accents;
- lush environments, made for an adult audience.

## 2. Keep: what already works in the legacy game
- `[DECIDED]` **Colour and light:** vibrant, beautiful colours, soft shadows, and geometry that reads clearly.
- `[DECIDED]` **Ink:** thick black outlines, especially strong on buildings.
- `[DECIDED]` **Props:** logs and trees with variety, as a base to build on.
- `[DECIDED]` **Uniform details** as the seed of faction identity: the conscript's round shoulder pieces, centre stripe
  and long trench-coat jacket.
- `[DECIDED]` **Small movement touches,** like the body reacting to strafing.
- `[DECIDED]` **The architecture direction:** the half-timbered house with the brick-tile roof, and the plastered house
  with the silo tower.
- `[DECIDED]` **The grenade concussion feel:** a screen shake that tilts away from the blast and grows stronger closer
  in (Tarkov-inspired).

## 3. Change
- `[DECIDED]` **The audience is adults.** It must not look like a children's game.
- `[DECIDED]` **Characters:** no chunky bodies or round, goofy heads. Slimmer, serious proportions; the references are
  XIII and Valiant Hearts.
- `[DECIDED]` **Weapons:** real silhouettes, no plain tubes or cylinders.
- `[DECIDED]` **The thatched roof** goes in its current form.
- `[DECIDED]` **Buildings sized for play:** interiors, upper floors, verticality.
- `[DECIDED]` **Fragmentation comes back:** grenades show fragments flying, not just smoke and fire.

## 4. Places
- `[DECIDED]` **Lush countryside** (fields, forests, villages) **plus contested urban centres** in the spirit of Foxhole:
  - ruined buildings you can enter, collapsed roofs, planks up to upper floors;
  - corners to hold, and street-to-street fights, as in Band of Brothers and Company of Heroes.
- `[DEFAULT]` **Damage stages:** buildings go from intact to damaged to a ruin with the interior exposed. Debris and
  planks become routes.
- `[DEFAULT]` **Roofs fade** when the player is inside, because the camera looks down from above.

## 5. Units and shape language
- `[DECIDED]` **Iconic silhouettes** you could draw from memory, like RA3's Stingray, Kirov, Apocalypse, Guardian and
  Mirage, and the unit designs of C&C 4.
- `[DECIDED]` **Planetary Annihilation's principle:** simple and readable from a distance, with real depth up close. Never
  "a block on wheels with a tube".

## 6. The Advisor's additions `[DEFAULT]`
1. **A value hierarchy:** the ground and environment sit one step calmer, so units, faction colours and effects are the
   brightest things on screen.
2. **Line weight:** thick ink on silhouettes, thinner lines inside.
3. **Weapons and headgear first:** they're what a top-down camera sees most.
4. **A glow language:** objectives, faction tech and muzzle flashes are emissive.
5. **Lighting:**
   - warm sun with cool shadows;
   - soft ambient occlusion in corners, so interiors read from above;
   - different moods per map, for example dusk.

## 7. Juice
- `[DECIDED]` **Punchy weapons** through VFX, sound and feedback: the feel kit in LOOP_V2 §7.
- `[DECIDED]` **Grenades:** fragmentation first, with the concussion screen effect.
- `[DEFAULT]` **Comic impact touches for big moments,** used sparingly: halftone flashes, ink "line boil".

## 8. The asset pipeline
- `[DECIDED]` **v2 may use authored 3D models.** The legacy rule "no external assets" ends with the legacy engine.
- `[DEFAULT]` **The flow:**
  1. a concept sketch;
  2. a 3D model in Spline: AI-generated for drafts, or a clean model built by Claude;
  3. a glTF/GLB export;
  4. an automated step that compresses it, builds LODs and renames materials into the faction colour slots;
  5. the engine applies the comic look.
- `[DEFAULT]` **Budgets per unit:** triangles, materials and textures. The game never runs Spline's own runtime.
- `[OPEN]` **The tree style.**

## 9. What the rendering decision must show
The rendering draft (ADR 0003) shows how each `[DECIDED]` item above is achieved. Its look proof test is judged by
Mustafa in the browser, side by side with screenshots of the legacy game.
