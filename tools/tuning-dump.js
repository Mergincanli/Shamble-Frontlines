// Tuning dump (R7, read-only): every tuning table and constant of the live legacy build as JSON.
// RUN from the repo root (Linux: prefix xvfb-run -a), the same file twice in one command:
//   node tools/preview.js . eval tools/tuning-dump.js --pre=tools/tuning-dump.js [--map=tri] > <out.json>
// As --pre (engine scripts loaded, bootGame not yet run) it snapshots the source defaults. As eval (after bootGame) it
// prints { defaults, bootChanges, skipped }: bootChanges lists only the values boot changed, per path, old -> new.
// Serialisation rule: numbers, strings, booleans, null, arrays and plain objects are kept; THREE.Color -> "#rrggbb";
// THREE.Vector2/3 -> [x, y(, z)]; functions and any other object (meshes, materials, class instances) are left out and
// their paths listed in "skipped". Runtime state (SOLDIERS, TEAM_INDEX, SIM, DEV, WR) is not tuning and is not read.
// Imports nothing and writes no file: save its stdout. It reads globals only and changes nothing.
(function () {
  const NAMES = ['T', 'TEAMS', 'TUNE', 'WEAPONS', 'UNIT_TYPES', 'VEHICLE_TYPES', 'ENV_DEFAULT', 'CHAR_DEFAULT', 'CHAR_MALE',
    'CHAR_FEMALE', 'CPARTS', 'FACTION_PALETTES', 'GUN_KIT', 'BK', 'MELEE_IDLE', 'BTN', 'BODY', 'PART', 'FIXED_DT',
    'SUN_R', 'GROUND_W', 'INK', 'WORLD_OUTLINE', 'CHAR_OUTLINE', 'RIM_COL', 'OUTLINE_GROUND_Y', 'GRID_SIZE',
    'RULES_DEFAULT', 'SOLDIER_KIT', 'ELITE_KIT', 'ANIM', 'ARM_L1', 'ECO', 'HARV', 'TBOX', 'TEAM_COL', 'TEAM_HEX',
    'RIFLE_TIERS', 'SILO', 'RR_C', 'JUG', 'JUG_PILOT', 'JUG_CANNON', 'FIRE_H', 'AI_STATE', 'T_CHARGE', 'AIM_ARC_N',
    'RANK_TIERS', 'PRESTIGE', 'CAM_STYLES', 'STAM', 'SFX', 'VIG_W', 'OCC_FADE', 'NAV', 'WR_DEPLOY_MS',
    'EJUMP_REACH', 'GUN_WORDS', 'KILL_WORDS', 'ROMAN'];
  const get = (n) => { try { return { ok: true, v: (0, eval)(n) }; } catch (e) { return { ok: false }; } };
  function ser(v, path, skipped, seen, depth) {
    if (v === null || typeof v === 'number' || typeof v === 'string' || typeof v === 'boolean') return v;
    if (typeof v === 'undefined') return null;
    if (typeof v === 'function') { skipped.push(path + ' (function)'); return undefined; }
    if (typeof THREE !== 'undefined' && v instanceof THREE.Color) return '#' + v.getHexString();
    if (typeof THREE !== 'undefined' && (v instanceof THREE.Vector3)) return [v.x, v.y, v.z];
    if (typeof THREE !== 'undefined' && (v instanceof THREE.Vector2)) return [v.x, v.y];
    if (typeof v !== 'object') { skipped.push(path + ' (' + typeof v + ')'); return undefined; }
    if (seen.has(v)) { skipped.push(path + ' (repeat reference)'); return undefined; }
    if (depth > 8) { skipped.push(path + ' (deeper than 8)'); return undefined; }
    const proto = Object.getPrototypeOf(v);
    if (!Array.isArray(v) && proto !== Object.prototype && proto !== null) { skipped.push(path + ' (' + ((proto && proto.constructor && proto.constructor.name) || 'object') + ')'); return undefined; }
    seen.add(v);
    let out;
    if (Array.isArray(v)) { out = v.map((x, i) => { const s = ser(x, path + '[' + i + ']', skipped, seen, depth + 1); return s === undefined ? null : s; }); }
    else { out = {}; for (const k of Object.keys(v).sort()) { const s = ser(v[k], path + '.' + k, skipped, seen, depth + 1); if (s !== undefined) out[k] = s; } }
    seen.delete(v);
    return out;
  }
  function snapshot() {
    const values = {}, skipped = [], missing = [];
    for (const n of NAMES) { const g = get(n); if (!g.ok) { missing.push(n); continue; } const s = ser(g.v, n, skipped, new Set(), 0); if (s !== undefined) values[n] = s; }
    return { values, skipped, missing };
  }
  function diff(a, b, path, out) {
    if (JSON.stringify(a) === JSON.stringify(b)) return;
    const objs = a && b && typeof a === 'object' && typeof b === 'object' && Array.isArray(a) === Array.isArray(b);
    if (!objs) { out.push({ path, from: a === undefined ? null : a, to: b === undefined ? null : b }); return; }
    const keys = Array.from(new Set(Object.keys(a).concat(Object.keys(b)))).sort();
    for (const k of keys) diff(a[k], b[k], Array.isArray(a) ? path + '[' + k + ']' : path + '.' + k, out);
  }
  if (typeof globalThis.__tuningDefaults === 'undefined') { globalThis.__tuningDefaults = snapshot(); return undefined; }
  const d = globalThis.__tuningDefaults, live = snapshot(), bootChanges = [];
  for (const n of Object.keys(d.values)) diff(d.values[n], live.values[n], n, bootChanges);
  return { map: (typeof curMap !== 'undefined' && curMap && curMap.name) || null, names: NAMES.length, missing: d.missing,
    defaults: d.values, bootChanges, skipped: d.skipped };
})();
