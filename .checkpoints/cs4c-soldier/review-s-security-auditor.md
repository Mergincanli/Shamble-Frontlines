# Step S review: security-auditor (content text shown on the page)

Head reviewed: c1de41e5 (`git diff dae6d107 c1de41e5 -- packages crates content`), plus `git diff c1de41e5 HEAD -- docs`
for secrets / personal details only.

## Plan
1. `app/content.ts`: `readMap` closed key sets, unknown-key refusal, prototype keys (`__proto__`, `constructor`),
   `Object.hasOwn` vs `in` / bracket lookups on plain objects, kind/format/version refusal, size/count bounds.
2. `overlay/report.ts` + `app/boot.ts`: every map-derived string to the DOM through `textContent`; no `innerHTML`,
   `insertAdjacentHTML`, attribute sinks; the two fixed sentences `NOT_THE_MAP`, `INVENTORY_UNKNOWN`.
3. `window.__shambleScene` and `inventory` only in the development build (`__SHAMBLE_DEVTOOLS__`), and
   `scripts/build-checks.mjs` markers that prove it.
4. `render/scene.ts`: no unbounded allocation sized by map data beyond loader bounds; no map string used as a key in a
   plain object.
5. Docs diff: secrets / keys / board addresses / personal details.

## Notes (one line per finding as I go)
- Block 1 (content.ts) done: `readMap` (content.ts:155-209) checks every object by `closed` (content.ts:118-120: own
  key count equals the set, each key own), reads only through `own` (content.ts:86-88, `Object.hasOwn`), numbers by
  `corner` (finite, exact length), enums by `word` against literal lists, ids as strings (the loader's grammar,
  crates/content/src/json.rs:73-91, holds them to `[a-z0-9][a-z0-9_-]*`). A JSON.parse-made `__proto__` key is an own
  key, so `closed` counts it and refuses it. `readContent` (content.ts:216-238) refuses unknown folders, formats, and
  versions outside 1..newest. Refusals are the two fixed sentences; nothing echoes input. No finding.
- Block 2 (DOM) done: map-derived text (ids, paths, formats) reaches the page only through `readoutLines` ->
  overlay.ts:36 `textContent`, and the copy report as plain text; boot.ts:36 `fail` is `textContent`. No innerHTML,
  insertAdjacentHTML, attribute or URL sink in packages/client/src (git grep). No finding.
- Block 3 (dev only) done: `__shambleScene` is set at boot.ts:172 inside `if (__SHAMBLE_DEVTOOLS__ && frames !== null)`
  (boot.ts:139); `inventory()` returns null unless `__SHAMBLE_DEVTOOLS__` (three-picture.ts:79-81); the flag is
  `VITE_SHAMBLE_DEVTOOLS === "1"` only (vite.config.mjs:93); both strings are production-scan markers
  (build-checks.mjs:206-213). The hook exposes the local scene only: no hidden or competitive data. No finding.
- Block 4 (scene.ts bounds) done: allocation sized by the map is bounded by the loader (bounds within +-327.67 m x,z and
  0..655.35 m y, map.rs:320-330; at most 512 shapes and 64 regions, map.rs:30-31), so the grid is at most ~1,312 lines.
  Material map keyed by grey number, not by map strings.
- FINDING S-1 (Medium): `overview`'s doubling loop never ends when the aspect is 0. scene.ts:286
  `while (!fits(far)) far *= 2;` with three-picture.ts:100 `camera.aspect = width / Math.max(1, height)` (width not
  clamped) and three-picture.ts:48/102 calling it on every resize once a map is drawn. Probe
  `.checkpoints/cs4c-soldier/sec-overview-aspect.mjs` (same math, loop capped at 5000): aspect 16/9 -> 7 doublings;
  aspect 0 -> never fits, far becomes Infinity/NaN (5000 doublings, far null); NaN the same. Failure case: the page
  laid out at innerWidth 0 (a zero-width iframe, a window or devtools layout that reports 0 for one resize) after the
  map is drawn: the main thread spins forever and the tab freezes. Small reach, reload is the workaround. Fix: clamp
  the width in `resize` (`Math.max(1, width)`) and make `overview` total: refuse a non-finite or non-positive aspect
  (keep the last camera) and cap the doubling (e.g. 64 steps, then a fixed far distance), with a unit test at aspect 0.
- FINDING S-2 (Low): no test refuses a map object with a JSON.parse-made own `__proto__` key. content.test.ts:81-84
  covers `__proto__` only at the bundle level (where it is accepted, by design) and :309-318 covers a polluted
  Object.prototype; the map's closed sets are tested with a key named `extra` (:248-268). Failure case: a later edit
  that copies a tree with `Object.assign({}, entry)` before the check (the assignment runs the `__proto__` setter: the
  key leaves the own keys, so `closed` sees the exact set and the extra key is taken silently; a spread `{...entry}`
  would keep it) would pass every test. Fix:
  one case `readMap(JSON.parse(text with "__proto__": {...} added to the map, a spawn and a shape))` -> NOT_THE_MAP.
- Block 5 (docs c1de41e5..HEAD) done: DECISIONS.md, LOOP_V2.md, EDITOR_V2.md, adr/README.md carry message ids, sha256
  sums, the session id (already in every commit trailer), branch and commit ids; no token, key, password, board
  address or the Testlines repository's address; the board line of 2026-11-27 is left out (his BOARD-WORDS (b)). No
  finding. Out of my lane, for docs-keeper: LOOP_V2.md:16's pointer sits mid-sentence ("Tech lifts the (2026-11-27:
  ...)" with the sentence going on in line 17).

## Anti-cheat coverage (this change)
| Player action | Where the server validates it | Gap |
|---|---|---|
| None new: the scene is drawn on the page from the bundle the build made | the Wasm loader (closed, ranged, canonical) before the page reads; the page re-checks shape only | none for this step; the future server must load its own map and never take the client's (the drawn map decides nothing: presentation only) |
| Keys (unchanged) | the Worker's simulation, as at dae6d107 | not in this diff |
