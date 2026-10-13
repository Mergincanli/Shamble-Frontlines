# Usage: python tools/village.py [core.js] [aabbs.json] [out.json]
#   defaults: <repo>/shamble-core.js, <tmp>/aabbs.json (from: node tools/preview.js . eval tools/aabbs.js > <tmp>/aabbs.json),
#   <tmp>/village.json (<tmp> = the OS temp folder). Paste the output into DM_Cabana.
# Cabana Republic village pass: design the LEFT half, mirror to the right (team symmetry), validate every piece.
# Output: JSON with houses/trees/props/covers in the exact shapes buildMap consumes.
import json, math, os, sys, tempfile

_REPO = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))
_ARGS = sys.argv[1:] + [None] * 3
CORE_JS = _ARGS[0] or os.path.join(_REPO, 'shamble-core.js')
AABBS_JSON = _ARGS[1] or os.path.join(tempfile.gettempdir(), 'aabbs.json')
OUT_JSON = _ARGS[2] or os.path.join(tempfile.gettempdir(), 'village.json')

GRID = [l.strip('",') for l in open(CORE_JS, encoding='utf-8').read()
        .split('const cabanaRepublicMap = [')[1].split('];')[0].strip().split('\n')]
H, W = len(GRID), len(GRID[0]); CX, CZ = (W - 1) / 2, (H - 1) / 2
cell = lambda r, c: ((c - CX) * 10.0, (r - CZ) * 10.0)
PATHS = [cell(r, c) for r in range(H) for c in range(W) if GRID[r][c] == '#']
aab = json.load(open(AABBS_JSON, encoding='utf-8-sig'))
BOXES = [(b[1], b[2], b[3], b[4]) for b in aab['boxes']]                       # existing colliders (minx,minz,maxx,maxz)
Z = aab['zones']
OBJ = [(s['x'], s['z']) for s in Z['silos']] + [(t['x'], t['z']) for t in Z['tech']] + [(Z['field']['x'], Z['field']['z'])]
SPAWN = [(Z['spawn']['x'], Z['spawn']['z']), (Z['resupply']['x'], Z['resupply']['z'])]
SPAWN += [(-x, z) for x, z in SPAWN]
S, S2, F, A, BV, CH, O, O2 = (-99, 90), (99, 90), (0, 0), (0, -60), (-70, 0), (70, 0), (-140, -90), (140, -90)
LANES = [(S, F), (S, BV), (S, A), (S, O), (S, S2), (S2, BV), (S2, O), (S2, S), (S, CH), (S, O2)]
LANES += [((-a[0], a[1]), (-b[0], b[1])) for a, b in LANES]                   # mirrored lanes
KEYPTS = [S, S2, F, A, BV, CH, O, O2]                                          # bots also travel objective -> objective
LANES += [(KEYPTS[i], KEYPTS[j]) for i in range(len(KEYPTS)) for j in range(i + 1, len(KEYPTS))]
HARV = [((-100, 78), (0, 0)), ((100, 78), (0, 0))]

HALF = {'timber': (3.25, 3.0), 'thatchCottage': (3.4, 3.5), 'brickFarm': (4.05, 3.25), 'manor': (6.65, 3.85),
        'haystack': (1.6, 1.6), 'haybales': (1.1, 0.9), 'cart': (1.8, 0.7), 'well': (1.35, 1.2), 'logpile': (1.1, 1.05),
        'barrels': (1.1, 1.0), 'vegpatch': (1.5, 1.0), 'trough': (1.25, 0.45)}
CANOPY = {'apple': 1.9, 'poplar': 1.6, 'oak': 2.95}
P2 = math.pi / 2

def fp(it):  # axis-aligned footprint rect (rotations are multiples of 90 deg for rect pieces)
    if it['cat'] == 'tree':
        r = CANOPY[it['kind']]; return (it['x'] - r, it['z'] - r, it['x'] + r, it['z'] + r)
    if it['cat'] == 'cover':
        hx, hz = it['len'] / 2 + (0.5 if it['kind'] == 'hedge' else 0), {'hedge': 1.45, 'drystone': 0.3, 'fence': 0.1}[it['kind']]
    else:
        hx, hz = HALF[it['kind']]
    q = round(it.get('rot', 0) / P2) % 2
    if q: hx, hz = hz, hx
    return (it['x'] - hx, it['z'] - hz, it['x'] + hx, it['z'] + hz)

def rect_d(a, b):
    dx = max(b[0] - a[2], a[0] - b[2], 0); dz = max(b[1] - a[3], a[1] - b[3], 0); return math.hypot(dx, dz)
def pt_rect(p, r): return rect_d((p[0], p[1], p[0], p[1]), r)
def seg_rect(a, b, r):  # min distance segment->rect, sampled finely (exact enough at 0.5 m)
    n = max(2, int(math.hypot(b[0] - a[0], b[1] - a[1]) / 0.5)); return min(pt_rect((a[0] + (b[0] - a[0]) * t / n, a[1] + (b[1] - a[1]) * t / n), r) for t in range(n + 1))

big = lambda it: it['cat'] == 'house' or (it['cat'] == 'cover' and it['kind'] in ('hedge', 'drystone'))
RULES = {  # minimum edge distances
    'path':  lambda it: 4 if it['cat'] == 'house' else 3 if big(it) else 1.0 if it['cat'] == 'tree' else 2,
    'box':   lambda it: 8 if it['cat'] == 'house' else 4,
    'obj':   lambda it: 20 if big(it) else 12,
    'spawn': lambda it: 15 if it['cat'] != 'prop' else 10,
    'lane':  lambda it: 8 if big(it) else 2.5 if it['cat'] == 'prop' else 1.2,   # trees: lane may pass under the canopy (trunk is tiny)
    'harv':  lambda it: 8 if it['cat'] != 'tree' else 3,
}
def check(it, others):
    r = fp(it); bad = []
    if not (-179 <= r[0] and r[2] <= 179 and -119 <= r[1] and r[3] <= 119): bad.append('out of bounds')
    dp = min(pt_rect(p, (r[0] - 5, r[1] - 5, r[2] + 5, r[3] + 5)) for p in PATHS)    # road tiles are 10x10 -> grow rect by 5
    if dp < RULES['path'](it): bad.append(f'road {dp:.1f}')
    db = min(rect_d(r, b) for b in BOXES)
    if db < RULES['box'](it): bad.append(f'collider {db:.1f}')
    do = min(pt_rect(o, r) for o in OBJ)
    if do < RULES['obj'](it): bad.append(f'objective {do:.1f}')
    ds = min(pt_rect(s, r) for s in SPAWN)
    if ds < RULES['spawn'](it): bad.append(f'spawn {ds:.1f}')
    dl = min(seg_rect(a, b, r) for a, b in LANES)
    if dl < RULES['lane'](it): bad.append(f'bot lane {dl:.1f}')
    dh = min(seg_rect(a, b, r) for a, b in HARV)
    if dh < RULES['harv'](it): bad.append(f'harvester route {dh:.1f}')
    for o in others:
        need = 3 if (it['cat'] == 'house' or o['cat'] == 'house') else 0.8
        d = rect_d(r, fp(o))
        if d < need: bad.append(f"overlaps {o['kind']}@{o['x']},{o['z']} ({d:.1f})"); break
    return bad

L = []   # ---- LEFT-HALF DESIGN (x<0). rot: 0 faces +z (south), P2 faces +x (east), -P2 faces -x, pi faces -z ----
def house(kind, x, z, rot): L.append(dict(cat='house', kind=kind, x=x, z=z, rot=rot))
def tree(kind, x, z, seed): L.append(dict(cat='tree', kind=kind, x=x, z=z, seed=seed))
def prop(kind, x, z, rot=0, seed=1): L.append(dict(cat='prop', kind=kind, x=x, z=z, rot=rot, seed=seed))
def cover(kind, x, z, ln, rot=0): L.append(dict(cat='cover', kind=kind, x=x, z=z, len=ln, rot=rot))
# D) west roadside hamlet: houses west of the outer road (x=-140), facing it; gardens + orchard behind
for i, (k, z) in enumerate([('thatchCottage', -44), ('timber', -21), ('brickFarm', 3), ('timber', 27)]):
    house(k, -155, z, P2)
    prop('vegpatch', -167, z - 1, P2, 10 + i)
    tree('apple', -175, z + 5, 20 + i)
cover('fence', -171.5, -32.5, 10, P2); cover('fence', -171.5, 15, 10, P2)
prop('well', -152, -32.5, 0, 3); prop('barrels', -163, -9, 0, 4); prop('logpile', -163, 15, P2, 5); prop('cart', -151, 39, P2, 6)
# C) north meadow: Norman manor on the axis (added once below), a brick farm + yard, an apple orchard, poplar rows lining the top road
house('brickFarm', -46, -104, 0)
prop('haystack', -60, -101, 0, 7); prop('haybales', -54, -93, 0.3, 8); prop('trough', -40, -95, 0, 9); prop('cart', -33, -103, P2, 10)
for ix, x in enumerate([-108, -99, -90, -81]):
    for iz, z in enumerate([-113, -104, -95]): tree('apple', x, z, 40 + ix * 3 + iz)
for i, x in enumerate([-100, -90, -80]): tree('poplar', x, -81, 60 + i)
for i, x in enumerate([-68, -56, -44, -32, -21]): tree('poplar', x, -71, 70 + i)
tree('poplar', -9, -93, 80); tree('poplar', -9, -83, 81)          # manor drive (mirrors to +9)
# A) bocage field around Bravo: hedgerows with gaps, big oaks, haystacks
cover('hedge', -97, 17, 12, P2); cover('hedge', -98, 2, 12, P2); cover('hedge', -108, -20, 12, P2)   # staggered bocage line west of Bravo, clear of every bot route
tree('oak', -100, -25, 90); tree('oak', -125, 5, 91); tree('oak', -55, -20, 92)
prop('haystack', -88, -38, 0, 11); prop('haystack', -122, -28, 0, 12)
# B) south-west pocket: small orchard + wood store
for ix, x in enumerate([-128, -119, -110]):
    for iz, z in enumerate([22, 31]): tree('apple', x, z, 100 + ix * 2 + iz)
prop('logpile', -106, 57, 0, 13); prop('barrels', -112, 60, 0, 14)
# bases: a little lived-in clutter behind the HQ / power plant
prop('barrels', -171, 96, 0, 15); prop('logpile', -171, 74, P2, 16)

def mirror(it):
    m = dict(it); m['x'] = -it['x']
    if 'rot' in it: m['rot'] = -it['rot']
    if 'seed' in it: m['seed'] = it['seed'] + 500
    return m
# auto-nudge: a left piece that breaks a rule moves to the nearest valid spot (<=12 m), its mirror follows
MANOR = dict(cat='house', kind='manor', x=0, z=-104, rot=0)
placed, moves = [MANOR], []
for it in L:
    if not check(it, placed) and not check(mirror(it), placed + [it]):
        placed += [it, mirror(it)]; continue
    best = None
    for rad in [x * 1.5 for x in range(1, 9)]:
        for k in range(16):
            a = k * math.pi / 8; c = dict(it, x=round(it['x'] + rad * math.cos(a), 1), z=round(it['z'] + rad * math.sin(a), 1))
            if not check(c, placed) and not check(mirror(c), placed + [c]): best = c; break
        if best: break
    if best: moves.append((it, best)); placed += [best, mirror(best)]
    else: moves.append((it, None))
for a, b in moves: print(f"  NUDGE {a['kind']} ({a['x']},{a['z']}) -> " + (f"({b['x']},{b['z']})" if b else 'DROPPED (no valid spot within 12 m)'), file=sys.stderr)
ALL = placed
report, keep = [], []
for it in ALL:
    bad = check(it, keep)
    (report.append((it, bad)) if bad else keep.append(it))
out = {'houses': [dict(bt=i['kind'], x=i['x'], z=i['z'], rot=round(i['rot'], 4)) for i in keep if i['cat'] == 'house'],
       'trees': [dict(kind=i['kind'], x=i['x'], z=i['z'], seed=i['seed']) for i in keep if i['cat'] == 'tree'],
       'props': [dict(kind=i['kind'], x=i['x'], z=i['z'], seed=i['seed'], rot=round(i['rot'], 4)) for i in keep if i['cat'] == 'prop'],
       'covers': [dict(type=i['kind'], x=i['x'], z=i['z'], len=i['len'], rot=round(i['rot'], 4)) for i in keep if i['cat'] == 'cover']}
print(json.dumps({k: len(v) for k, v in out.items()}), '| rejected:', len(report), file=sys.stderr)
for it, bad in report: print(f"  REJECT {it['cat']}:{it['kind']} @({it['x']},{it['z']}) -> {'; '.join(bad)}", file=sys.stderr)
json.dump(out, open(OUT_JSON, 'w', encoding='utf-8'))
print('wrote', OUT_JSON, file=sys.stderr)
