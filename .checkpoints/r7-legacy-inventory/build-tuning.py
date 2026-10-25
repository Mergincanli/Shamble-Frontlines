# R7: builds docs/legacy-tuning.json and docs/LEGACY_TUNING.md from the two tuning dumps (tools/tuning-dump.js).
# Run from the repo root: python3 -I .checkpoints/r7-legacy-inventory/build-tuning.py
import json, re, subprocess

DUMPS = {'DM_Cabana (default)': '.checkpoints/r7-legacy-inventory/tuning/cabana-1.json',
         'DM_Tri (?map=tri)': '.checkpoints/r7-legacy-inventory/tuning/tri-1.json'}
COMMIT = '48ffb51'
GROUPS = [
    ('gameplay', 'Gameplay: movement, combat, units, vehicles, teams, rules and economy', False, [
        'T', 'TUNE', 'TEAMS', 'WEAPONS', 'UNIT_TYPES', 'VEHICLE_TYPES', 'SOLDIER_KIT', 'ELITE_KIT', 'RIFLE_TIERS',
        'RULES_DEFAULT', 'ECO', 'SILO', 'TBOX', 'HARV', 'JUG', 'JUG_PILOT', 'JUG_CANNON', 'T_CHARGE', 'STAM', 'FIRE_H',
        'EJUMP_REACH', 'AIM_ARC_N', 'GROUND_W', 'GRID_SIZE']),
    ('simulation', 'Simulation: tick, input bits, hit bodies', False, ['FIXED_DT', 'BTN', 'BODY', 'PART']),
    ('progression', 'Progression and presentation: ranks, prestige, words, camera styles, UI timings', False, [
        'RANK_TIERS', 'PRESTIGE', 'ROMAN', 'GUN_WORDS', 'KILL_WORDS', 'CAM_STYLES', 'VIG_W', 'WR_DEPLOY_MS', 'RR_C',
        'OCC_FADE', 'SFX']),
    ('look', 'Look and builders (look brief pending: listed as data only, never a decision)', True, [
        'ENV_DEFAULT', 'SUN_R', 'INK', 'WORLD_OUTLINE', 'CHAR_OUTLINE', 'RIM_COL', 'OUTLINE_GROUND_Y', 'TEAM_COL',
        'TEAM_HEX', 'FACTION_PALETTES', 'CHAR_DEFAULT', 'CHAR_MALE', 'CHAR_FEMALE', 'CPARTS', 'GUN_KIT', 'BK',
        'MELEE_IDLE', 'ANIM', 'ARM_L1']),
    ('state', 'Runtime state, not tuning (kept in the JSON for completeness)', False, ['NAV', 'AI_STATE']),
]

src = {f: subprocess.run(['git', 'show', f'{COMMIT}:shamble-{f}.js'], capture_output=True, text=True, check=True).stdout.split('\n')
       for f in ('core', 'game')}
def where(n):
    for f in ('core', 'game'):
        for i, l in enumerate(src[f], 1):
            if re.match(rf'^(const|let)\s+{re.escape(n)}\s*=', l):
                return f'shamble-{f}.js:{i}'
    raise SystemExit(f'no declaration found for {n}')

dumps = {k: json.load(open(p)) for k, p in DUMPS.items()}
base = dumps['DM_Cabana (default)']
assert all(json.dumps(d['defaults'], sort_keys=True) == json.dumps(base['defaults'], sort_keys=True) for d in dumps.values())
listed = [n for _, _, _, names in GROUPS for n in names]
assert sorted(listed) == sorted(base['defaults']), set(listed) ^ set(base['defaults'])

out = {'about': {
    'what': 'Every tuning table and constant of the legacy build, as data, for the v2 content tables. Starting values, not decisions.',
    'commit': f'{COMMIT} (the engine files are unchanged on main since)',
    'produced_by': 'node tools/preview.js . eval tools/tuning-dump.js --pre=tools/tuning-dump.js [--map=tri]',
    'defaults': 'values after the engine scripts load and before bootGame()',
    'serialisation': 'numbers, strings, booleans, null, arrays, plain objects kept; THREE.Color as "#rrggbb", Vector2/3 as arrays; functions and other objects left out and listed under skipped',
    'repeatable': 'two runs per map gave byte-identical output',
  },
  'tables': {}, 'boot_changes': {}, 'skipped': base['skipped']}
for key, title, look, names in GROUPS:
    for n in names:
        out['tables'][n] = {'where': where(n), 'group': key, 'look_brief_pending': look, 'values': base['defaults'][n]}
for k, d in dumps.items():
    out['boot_changes'][k] = d['bootChanges']
json.dump(out, open('docs/legacy-tuning.json', 'w'), indent=1, ensure_ascii=False, sort_keys=False)
open('docs/legacy-tuning.json', 'a').write('\n')

def fmt(v):
    s = json.dumps(v, ensure_ascii=False)
    return s.replace('|', '\\|')
def flat(v, path, rows):
    if isinstance(v, dict) and v:
        for k in v: flat(v[k], f'{path}.{k}' if path else k, rows)
    elif isinstance(v, list) and v and all(isinstance(x, dict) for x in v):
        for i, x in enumerate(v): flat(x, f'{path}[{i}]', rows)
    else:
        rows.append((path, v))
def leaves(v):
    rows = []; flat(v, '', rows); return len(rows)

L = []
L += ['# Legacy tuning: every number, as data', '',
      '**Purpose.** R7 of the v2 phase 0 brief (`docs/V2_PHASE0_BRIEF.md` §4): "Extract the tuning numbers into data tables."',
      'These are the legacy game\'s starting values for v2\'s content tables (LOOP_V2: every number in one data table). They are',
      'what the legacy build does, not decisions for v2: LOOP_V2 replaces several of these rules (see `docs/LEGACY_INVENTORY.md`).', '',
      f'**Source.** `docs/legacy-tuning.json` is authoritative; this page renders it. Main at {COMMIT} (the engine files are',
      'unchanged on main since). Produced by `tools/tuning-dump.js`, run through the real build:',
      '`node tools/preview.js . eval tools/tuning-dump.js --pre=tools/tuning-dump.js [--map=tri]` (Linux: prefix `xvfb-run -a`).', '',
      '**How to read it.**',
      '- "Default" is the value after the engine scripts load and before `bootGame()`. Boot changes some values per map; they',
      '  are listed at the end, per map.',
      '- 17 dumped values were checked against their literal in the source: all equal (the R7 report lists them).',
      '- Two runs per map gave byte-identical output.',
      '- The look group is data only. The v2 look is not decided (Mustafa and the Advisor write the look brief).', '']
L += ['## Contents', '']
for key, title, look, names in GROUPS:
    L.append(f'- **{title}:** ' + ', '.join(f'`{n}`' for n in names))
L += ['- **Boot changes per map**', '- **Left out** (functions and objects the dump does not serialise)', '']
for key, title, look, names in GROUPS:
    L += [f'## {title}', '']
    scal = [n for n in names if not isinstance(base['defaults'][n], (dict, list))]
    if scal:
        L += ['| Constant | Value | Where |', '|---|---|---|']
        for n in scal: L.append(f'| `{n}` | {fmt(base["defaults"][n])} | {where(n)} |')
        L.append('')
    for n in names:
        v = base['defaults'][n]
        if not isinstance(v, (dict, list)): continue
        L.append(f'### `{n}` ({where(n)})')
        if look or key == 'state':
            L += ['', f'{leaves(v)} values; see `docs/legacy-tuning.json` → `tables.{n}`.', '']
            continue
        if isinstance(v, list) and v and all(isinstance(x, dict) for x in v):
            cols = []
            for x in v:
                for k in x:
                    if k not in cols: cols.append(k)
            if len(cols) > len(v):   # wide: one row per field, one column per item (named by id or name)
                heads = [str(x.get('id', x.get('name', i))) for i, x in enumerate(v)]
                L += ['', '| Field | ' + ' | '.join(f'`{h}`' for h in heads) + ' |', '|---|' + '---|' * len(v)]
                for c in cols:
                    L.append(f'| `{c}` | ' + ' | '.join(fmt(x[c]) if c in x else '' for x in v) + ' |')
            else:
                L += ['', '| # | ' + ' | '.join(f'`{c}`' for c in cols) + ' |', '|---|' + '---|' * len(cols)]
                for i, x in enumerate(v):
                    L.append(f'| {i} | ' + ' | '.join(fmt(x[c]) if c in x else '' for c in cols) + ' |')
            L.append('')
        elif isinstance(v, list):
            L += ['', fmt(v), '']
        else:
            rows = []; flat(v, '', rows)
            L += ['', '| Key | Value |', '|---|---|']
            for p, x in rows: L.append(f'| `{p}` | {fmt(x)} |')
            L.append('')
L += ['## Boot changes per map', '',
      'Values that `bootGame()` changes after the defaults, by map. NAV and BK entries are the navigation grid and the',
      'building-kit registries filled at boot (state, not tuning).', '']
for k, d in dumps.items():
    ch = d['bootChanges']
    L += [f'### {k}: {len(ch)} changes', '', '| Path | Default | After boot |', '|---|---|---|']
    for c in ch:
        to = c['to']
        if isinstance(to, (dict, list)) and len(json.dumps(to)) > 120:
            to = f'({leaves(to)} values; see the JSON)'
            L.append(f'| `{c["path"]}` | {fmt(c["from"])} | {to} |')
        else:
            L.append(f'| `{c["path"]}` | {fmt(c["from"])} | {fmt(to)} |')
    L.append('')
L += ['## Left out', '', f'{len(base["skipped"])} paths the dump does not serialise (functions, mostly the character-part and gun-model makers):', '']
L.append(', '.join(f'`{s}`' for s in base['skipped']))
L.append('')
open('docs/LEGACY_TUNING.md', 'w').write('\n'.join(L))
print('tables', len(out['tables']), 'md lines', len(L))
