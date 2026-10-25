# R7: builds docs/LEGACY_INVENTORY.md from the editors' final rows (final-<area>.json) and the second skeptics'
# fixes (fixes2.json). Run from the repo root: python3 -I .checkpoints/r7-legacy-inventory/build-inventory.py
# fixes2.json: {"<id>": {"<field>": "<new text>", ...}} replaces fields; {"<id>": {"_remove": "<why>"}} drops a row.
import json, os, re, collections

D = '.checkpoints/r7-legacy-inventory'
AREAS = [  # code, title, scope (as the readers were given it)
    ('Core (`shamble-core.js`)', [
        ('CD', 'Core data, tuning and persistence', 'shamble-core.js:1-232'),
        ('CL', 'Core look pipeline', 'shamble-core.js:233-559'),
        ('CC', 'Core characters, factions and guns', 'shamble-core.js:560-845'),
        ('CM', 'Core maps and building kit', 'shamble-core.js:846-1473'),
        ('CS', 'Core headless SIM math', 'shamble-core.js:1474-1543'),
    ]),
    ('Game (`shamble-game.js`)', [
        ('GW', 'Game world, map building and movement physics', 'shamble-game.js, several ranges'),
        ('GA', 'Game animation (ANIM v2)', 'shamble-game.js:224-573 and the held melee and shield models'),
        ('GC', 'Game combat, teams and vehicles', 'shamble-game.js, several ranges'),
        ('GN', 'Game bots and navigation', 'shamble-game.js, about 1434-1653 and 2609-2965'),
        ('GE', 'Game economy, objectives and progression', 'shamble-game.js, several ranges'),
        ('GP', 'Game presentation, input and dispatch', 'shamble-game.js, several ranges'),
    ]),
    ('Editor, pages, tools and kit', [
        ('ED', 'Editor (`shamble-editor.js`)', 'shamble-editor.js; the map editor\'s features are listed row by row in `docs/EDITOR_V1_INVENTORY.md`'),
        ('PG', 'Pages (`index.html`, `editor.html`)', 'both pages: head, DOM, script tags, inline script'),
        ('TL', 'Tools (the verification toolkit)', '`tools/`, `.claude/hooks/syntax-gate.mjs`'),
        ('KT', 'Kit (process)', '`CLAUDE.md`, `.claude/`, `.github/workflows/`, the docs process'),
    ]),
]
CODES = [c for _, g in AREAS for c, _, _ in g]
MARKS = ['rewrite in Rust', 'port to TypeScript', 'redesign', 'drop']
FIELDS = ['id', 'system', 'what', 'where', 'evidence', 'state', 'v2_note', 'mark', 'reason', 'look_brief_pending']

rows = {}
for c in CODES:  # after-<area>.json: the second skeptics' corrections applied by apply-ops.py
    r = json.load(open(f'{D}/after-{c}.json' if os.path.exists(f'{D}/after-{c}.json') else f'{D}/final-{c}.json'))
    rows[c] = r['rows'] if isinstance(r, dict) else r
fixes = json.load(open(f'{D}/fixes2.json')) if os.path.exists(f'{D}/fixes2.json') else {}
used, removed = set(), {}
for c in CODES:
    keep = []
    for r in rows[c]:
        f = fixes.get(r['id'])
        if f:
            used.add(r['id'])
            if '_remove' in f:
                removed[r['id']] = f['_remove']; continue
            for k, v in f.items():
                assert k in FIELDS and k != 'id', (r['id'], k)
                r[k] = v
        keep.append(r)
    rows[c] = keep
assert used == set(fixes), set(fixes) - used

# checks: every field present and typed, ids unique and in their area, marks and states from the lists
seen = set()
for c in CODES:
    for r in rows[c]:
        assert set(r) >= set(FIELDS), (r.get('id'), set(FIELDS) - set(r))
        assert r['id'] not in seen and re.fullmatch(c + r'\d+', r['id']), r['id']
        seen.add(r['id'])
        assert r['mark'] in MARKS and r['state'] in 'CMD' and isinstance(r['look_brief_pending'], bool), r['id']
        for k in FIELDS[:-1]:
            assert isinstance(r[k], str) and r[k].strip(), (r['id'], k)
all_rows = [r for c in CODES for r in rows[c]]

def cell(s):
    return re.sub(r'\s*\n\s*', ' ', s.strip()).replace('|', '\\|')
def mark_cell(r):
    s = f"**{r['mark']}** — {cell(r['reason'])}"
    return s + (' *Look brief pending.*' if r['look_brief_pending'] else '')

tally = collections.Counter(r['mark'] for r in all_rows)
look = sum(r['look_brief_pending'] for r in all_rows)
states = collections.Counter(r['state'] for r in all_rows)
rc = json.load(open(f'{D}/rows-and-checks.json'))
first = sum(len(v['rows']) for v in rc.values())
v1 = collections.Counter(x['verdict'] for v in rc.values() for x in v['checks'])
cov = json.load(open(f'{D}/coverage-final.json')) if os.path.exists(f'{D}/coverage-final.json') else None
meta = json.load(open(f'{D}/inventory-meta.json'))  # numbers from the edit pass, filled in by the session

L = ['# Legacy inventory: every system of the legacy build', '',
     '**Purpose.** R7 of the v2 phase 0 brief (`docs/V2_PHASE0_BRIEF.md` §4): "Every system of the current game: what it',
     'does, where it lives, the evidence and numbers we have, and a pre-marked recommendation (port to TypeScript /',
     'rewrite in Rust / redesign / drop) with a reason." The other research topics (R1–R6) read it. Its tuning numbers are',
     'in `docs/LEGACY_TUNING.md` and `docs/legacy-tuning.json`.', '',
     '**Marks:** each one is Claude\'s recommendation with a reason; Mustafa confirms or changes it in the last column.',
     '- **rewrite in Rust**: a mechanic the authoritative v2 simulation decides and LOOP_V2 keeps (movement, collision,',
     '  shots, line of sight, damage, team filtering, bot navigation and behaviour, vehicle physics). A rule LOOP_V2 marks',
     '  [OPEN] says "rule pending LOOP_V2 [OPEN]".',
     '- **port to TypeScript**: client code worth keeping in shape (input, camera, procedural builders, cosmetic animation,',
     '  synthesized audio, HUD and menu flows), re-expressed for the new client.',
     '- **redesign**: something v2 already plans to change. Where LOOP_V2 replaces a legacy rule, the row cites its section',
     '  (eras and income, silos and tech boxes, win conditions, spawning and buying, progression); the map editor follows',
     '  EDITOR_V2; the look pipeline is "redesign (look brief pending)" and nothing more.',
     '- **drop**: dead code and legacy-only workarounds (classic-script globals, the symbol-conservation checks, r128',
     '  limits).',
     '- **Look brief pending**: every row whose output a player sees carries it, whatever its mark. For those rows the',
     '  mark is about the code technique only, never the appearance: the v2 look is not decided (Mustafa and the Advisor',
     '  write the look brief).', '',
     f'**Tally of recommended marks ({len(all_rows)} rows):** ' + ' · '.join(f'{m} {tally[m]}' for m in MARKS)
     + f'. Look brief pending on {look} rows. Per area: the table under "Tally by area" at the end.', '',
     '**Date:** 2026-10-25. **Code:** main at f2ccc13. The engine files (`shamble-core.js`, `shamble-game.js`,',
     '`shamble-editor.js`) and both pages are byte-identical to 48ffb51, where the reading started, so every line number',
     'holds on both. In the Where column, `core`, `game` and `editor` mean those three files.', '',
     '**How each state was checked:**',
     f'- **[C]** ({states["C"]} rows): code read at main; every Where re-opened by a second reader.',
     f'- **[M]** ({states["M"]} rows): measured, in a report under `docs/reports/` or by a harness in `tools/`; the row names it.',
     f'- **[D]** ({states["D"]} rows): a claim from a doc, not re-checked here; the row names the doc.', '',
     '**How this file was made.** Four passes, each by Opus agents at effort high, with notes on checkpoints:',
     f'1. Read: one reader per area (15) wrote {first} rows, each citing file:line.',
     f'2. Verify: one skeptic per area re-opened every Where and every number and tried to refute it: {v1["confirmed"]}',
     f'   confirmed, {v1["corrected"]} corrected, {v1.get("refuted", 0)} refuted. They also listed {meta["missing"]} systems no row covered.',
     '3. Coverage: a critic parsed the three engine files (acorn) and listed every top-level declaration no row names:',
     '   core 138/138, game 584/714, editor 204/353 named before the edit pass.',
     f'4. Edit and re-verify: one editor per area applied every correction, added or widened rows for the missing systems',
     f'   and the critic\'s suggestions ({meta["added"]} rows added, {meta["widened"]} widened, {meta["rejected"]} suggestions not taken,',
     f'   each with a reason), and a second skeptic checked every added or widened row ({meta["v2_confirmed"]} confirmed,',
     f'   {meta["v2_corrected"]} corrected, {meta["v2_refuted"]} refuted; corrections applied, refuted rows removed or fixed).']
if cov:
    L += [f'   Coverage after the edit pass (top-level declarations named in some row\'s Where, mechanical match): ' +
          ', '.join(f'{f} {cov[f]["named"]}/{cov[f]["total"]}' for f in ('core', 'game', 'editor')) + '.']
L += ['', 'The notes of every pass are in the history of the R7 branch (`.checkpoints/r7-legacy-inventory/`, removed in',
      'the last commit before review, as `.claude/CHECKPOINTS.md` §9 requires).', '']

L += ['## Contents', '']
for g, areas in AREAS:
    L.append(f'- **{g}:** ' + ' · '.join(f'{c}. {t} ({len(rows[c])})' for c, t, _ in areas))
L += ['- **Tally by area**', '- **Not covered and not verified**', '']

HEAD = ['| ID | System | What it does | Where | Evidence and numbers | State | v2 note | Mark (recommended) |',
        '|---|---|---|---|---|---|---|---|']
for g, areas in AREAS:
    L += [f'# {g}', '']
    for c, t, scope in areas:
        L += [f'## {c}. {t}', '', f'Scope: {scope}.', ''] + HEAD
        for r in rows[c]:
            L.append('| ' + ' | '.join([r['id'], cell(r['system']), cell(r['what']), cell(r['where']),
                                         cell(r['evidence']), f"[{r['state']}]", cell(r['v2_note']), mark_cell(r)]) + ' |')
        L.append('')

L += ['## Tally by area', '', '| Area | Rows | ' + ' | '.join(MARKS) + ' | Look brief pending |', '|---|---|' + '---|' * (len(MARKS) + 1)]
for c in CODES:
    t = collections.Counter(r['mark'] for r in rows[c])
    L.append(f'| {c} | {len(rows[c])} | ' + ' | '.join(str(t[m]) for m in MARKS) + f' | {sum(r["look_brief_pending"] for r in rows[c])} |')
L.append(f'| **All** | **{len(all_rows)}** | ' + ' | '.join(f'**{tally[m]}**' for m in MARKS) + f' | **{look}** |')
L += ['']

L += ['## Not covered and not verified', '']
if cov:
    for f in ('core', 'game', 'editor'):
        un = cov[f]['unnamed']
        if not un:
            L.append(f'- **{f}**: every top-level declaration is named in some row.'); continue
        L.append(f'- **{f}**: {cov[f]["in_range"]} top-level declarations are not named but sit inside a line range a row'
                 f' cites (scratch vectors, DOM handles, small helpers of that row\'s system). {len(un)} are neither named nor'
                 ' inside a cited range' + (': ' + ', '.join(f'`{d["n"]}` ({f}:{d["l"]})' for d in un) + '.' if un else '.'))
L += [f'- **[D] rows** ({states["D"]}): doc claims, not re-checked; each names its doc.',
      '- **Marks** are recommendations; none is a decision until Mustafa confirms it.',
      '- **The look**: no row decides it; the look pipeline row says "redesign (look brief pending)" and nothing more.', '']
open('docs/LEGACY_INVENTORY.md', 'w').write('\n'.join(L))
print('rows', len(all_rows), dict(tally), 'look', look, 'states', dict(states), 'removed', removed, 'lines', len(L))
