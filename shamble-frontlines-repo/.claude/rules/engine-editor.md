---
paths:
  - "**/shamble-editor.js"
---
# Rules for shamble-editor.js (authoring; loaded by editor.html only)
- May reference game and core freely. Nothing on index.html's live path may resolve only here (gate check 4).
- A function called from top-level code must itself be top-level. Known bug until fixed: `refreshPrefabList` is declared
  inside `bootEditor()` but called by `enterEditor`, so every editor entry throws a ReferenceError.
- Maps round-trip through `serializeMap` / `buildMap` (game); keep saved maps and prefabs backward compatible.
