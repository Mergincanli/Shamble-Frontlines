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
- Memory: no security-auditor memory exists in this environment (none under /home/user/Shamble-Frontlines/.claude
  or the worktree); worked from the brief and the code. Durable decision proposed for memory (not written: this run
  writes only its review file): the page's drawn map is presentation only; the future server loads its own map and
  never takes the client's.
- Block 1 (content.ts) done: `readMap` (content.ts:155-203) checks every object by `closed` (content.ts:118-120: own
  key count equals the set, each key own), reads only through `own` (content.ts:86-88, `Object.hasOwn`), numbers by
  `corner` (finite, exact length), enums by `word` against literal lists, ids as strings (the loader's grammar,
  crates/content/src/json.rs:73-91, holds them to `[a-z0-9][a-z0-9_-]*`). A JSON.parse-made `__proto__` key is an own
  key, so `closed` counts it and refuses it. `readContent` (content.ts:210-238) refuses unknown folders, formats, and
  versions outside 1..newest. Refusals are the two fixed sentences; nothing echoes input. No finding.
- Block 2 (DOM) done: map-derived text (ids, paths, formats) reaches the page only through `readoutLines` ->
  overlay.ts:36 `textContent`, and the copy report as plain text; boot.ts:36 `fail` is `textContent`. No innerHTML,
  insertAdjacentHTML, attribute or URL sink in packages/client/src (git grep). No finding.
- Block 3 (dev only) done: `__shambleScene` is set at boot.ts:172 inside `if (__SHAMBLE_DEVTOOLS__ && frames !== null)`
  (boot.ts:124); `inventory()` returns null unless `__SHAMBLE_DEVTOOLS__` (three-picture.ts:79-81); the flag is
  `VITE_SHAMBLE_DEVTOOLS === "1"` only (vite.config.mjs:93); both strings are production-scan markers
  (build-checks.mjs:205-213), scanned at build-checks.mjs:253-257 (a production build holding
  either fails; a development build missing either fails), and build-checks.test.mjs:243-257 now expects 7 problems
  for the 7 markers both ways. The hook exposes the local scene only: no hidden or competitive data. No finding.
- Block 4 (scene.ts bounds) done: allocation sized by the map is bounded by the loader (bounds within +-327.67 m x,z and
  0..655.35 m y, map.rs:317-330; at most 512 shapes and 64 regions, map.rs:30-31), so the grid is at most ~1,312 lines.
  Material map keyed by grey number, not by map strings.
- FINDING S-1 (Medium; robustness, not a security finding; cross-lane: the overview's fit is graphics-engineer's
  part): `overview`'s doubling loop never ends when the aspect is 0. scene.ts:286 `while (!fits(far)) far *= 2;`;
  three-picture.ts:100 sets `camera.aspect = width / Math.max(1, height)` (the width is not clamped); `frame()` runs
  `overview` from `setContent` (three-picture.ts:77) and from every `resize` once a map is drawn (three-picture.ts:102).
  boot.ts:113-115 calls `resize()` before the map arrives, so the strongest failure case is: the page opened at
  innerWidth 0 hangs at the Worker's `ready`, with no later resize needed; a later resize to width 0 hangs it too. The
  main thread spins forever and the tab freezes; reload is the workaround. What is proven: the probe
  `/home/user/Shamble-Frontlines/.gate/cs4b-2/.checkpoints/cs4c-soldier/sec-overview-aspect.mjs` copies `overview`'s
  math on plain `three` (it does not call scene.ts) with the loop capped at 5000: aspect 16/9 -> fits after 7
  doublings; aspect 0 and NaN -> never fit, far goes to Infinity (5000 doublings, far null). NOT proven: no browser was
  shown reporting innerWidth 0 to this page; a zero-width iframe is the plausible case, any other is a guess. Why not a
  security finding: only the player's own window or a page that embeds this one sets the width, and the effect is a
  self-hang of that tab, with nothing gained and no data reached. Fix (cheap, for this fix pass rather than a
  follow-up): clamp the width in `resize` (`Math.max(1, width)`), make `overview` total (a non-finite or non-positive
  aspect keeps the last camera; the doubling capped, e.g. 64 steps), and one unit test at aspect 0.
- FINDING S-2 (Low): no test refuses a map object with a JSON.parse-made own `__proto__` key. content.test.ts:81-84
  covers `__proto__` only at the bundle level (where it is accepted, by design) and :309-318 covers a polluted
  Object.prototype; the map's closed sets are tested with a key named `extra` (:251-269). Failure case: a later edit
  that copies a tree with `Object.assign({}, entry)` before the check (the assignment runs the `__proto__` setter: the
  key leaves the own keys, so `closed` sees the exact set and the extra key is taken silently; a spread `{...entry}`
  would keep it) would pass every test. Fix:
  one case `readMap(JSON.parse(text with "__proto__": {...} added to the map, a spawn and a shape))` -> NOT_THE_MAP.
- Block 5 (docs c1de41e5..HEAD) done: DECISIONS.md, LOOP_V2.md, EDITOR_V2.md, adr/README.md carry message ids, sha256
  sums, the session id (already in every commit trailer), branch and commit ids; no token, key, password, board
  address or the Testlines repository's address; the board line of 2026-11-27 is left out (his BOARD-WORDS (b)).
  Personal details: his absence window ("away until Sunday 29 November, 12:00"), his message times and the account
  descriptions ("his second personal account") are his own messages copied whole, or roles CLAUDE.md already states;
  no name beyond Mustafa, no address, no contact detail. No finding. Out of my lane, for docs-keeper: LOOP_V2.md:16's pointer sits mid-sentence ("Tech lifts the (2026-11-27:
  ...)" with the sentence going on in line 17).

## Anti-cheat coverage (this change)
| Player action | Where the server validates it | Gap |
|---|---|---|
| None new: the scene is drawn on the page from the bundle the build made | the Wasm loader (closed, ranged, canonical) before the page reads; the page re-checks shape only | none for this step; the future server must load its own map and never take the client's (the drawn map decides nothing: presentation only) |
| Keys (unchanged) | the Worker's simulation, as at dae6d107 | not in this diff |

## Verdict: APPROVE
Item 4 holds: map text reaches the DOM only through `textContent`; every map object is a closed key set read from own
properties only; unknown kinds, formats and versions are refused; every refusal is one of the fixed sentences
(`NOT_A_BUNDLE`, `NOT_THE_MAP`, `INVENTORY_UNKNOWN`), which quote nothing read; `window.__shambleScene` and the inventory
exist in the development build only, and the production scan proves it. No security finding is open.
- S-1 Medium (robustness, cross-lane, not security): scene.ts:286 with three-picture.ts:100 and :77, the overview hangs
  the tab at aspect 0. Recommended for this fix pass (three small edits and one test); if deferred, a STATUS follow-up.
- S-2 Low: content.test.ts:251-269, no `__proto__` own-key case for the map's closed sets. A follow-up, or with S-1's fix.
- Scratch file left in the notes folder: `.checkpoints/cs4c-soldier/sec-overview-aspect.mjs` (the S-1 probe), for the
  main session to keep as evidence or delete with the checkpoint.
- Out of lane, for docs-keeper: docs/LOOP_V2.md:16, a pointer placed mid-sentence.

DONE
