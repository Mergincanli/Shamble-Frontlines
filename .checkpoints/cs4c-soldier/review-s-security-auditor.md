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
