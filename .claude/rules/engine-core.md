---
paths:
  - "**/shamble-core.js"
---
# Rules for shamble-core.js (engine / shared truth)
- Core references NOTHING above it: no game or editor symbols, guarded or not.
- The fenced SIM block at the bottom (`rayAabb`, `rayStatic`, `rayStatics`, `resolveShot`, `hasLOS`, `SIM`, `FIXED_DT`,
  `BODY`, `PART`) is pure math: no THREE scene, DOM or render globals. Loops over statics go through `rayStatics` (slab
  test inlined): a call per static boxes its floats, about 80 B each. Shots, line of sight and camera occlusion depend on its API;
  change signatures only inside an approved change-set. It is the seed of the future shared client/server simulation.
- NPR constants are art direction, not tuning knobs: `INK` 0x11121a, `WORLD_OUTLINE` 0.09, `CHAR_OUTLINE` 0.8,
  `OUTLINE_GROUND_Y` 0.02, the cel ramps, `ENV_DEFAULT`, `FACTION_PALETTES`. Changing them needs the art-director's
  review and Mustafa's Brave sign-off (values: docs/ART_DIRECTION.md).
- Output is `LinearEncoding` (authored hex = on-screen colour). New environment materials set `userData.nature = true`.
- Runtime builders call `stylizeOne()`; static kit pieces then call `mergeStaticGroup()`. Tag `userData.noMerge` on
  anything that animates or fades per part. Merge by material content (`_matSig`), never by reference.
- GLSL lives inside JS strings, so brace counts are only a delta check.
- Persistence only through `KVStore`; renaming a key needs a fallback read so old saves survive.
