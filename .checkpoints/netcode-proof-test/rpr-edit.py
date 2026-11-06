# R-PR edits for the netcode proof test: place the advisor's readings of 2026-11-06 and Mustafa's answers.
# Idempotent: every file is rebuilt from main af8393e's version, then written into the worktree.
# Run: python3 -I rpr-edit.py <worktree> <words.json>
import json, re, subprocess, sys

WT = sys.argv[1]
W = json.load(open(sys.argv[2], encoding='utf-8'))
BASE = 'af8393e'
MAXW = 120
TAG = " (the advisor's reading, 2026-11-06)"

def base(path):
    return subprocess.run(['git', '-C', WT, 'show', f'{BASE}:{path}'], check=True,
                          capture_output=True).stdout.decode('utf-8')

def word(key):
    hits = [v for k, v in W.items() if k.startswith(key)]
    assert len(hits) == 1, key
    return hits[0]

def tagged(s, end='.'):
    assert s.endswith('.'), s[-20:]
    return s[:-1] + TAG + end

BAD = re.compile(r'^([-+*]|\d+[.)]|#+|>|\|)$')

def wrap(first_prefix, words_text, indent):
    """Greedy wrap: the first line starts with first_prefix (kept whole), the rest with indent."""
    words = words_text.split()
    lines = [first_prefix]
    for w in words:
        cur = lines[-1]
        cand = (cur + ' ' + w) if cur.strip() else (cur + w)
        if len(cand) <= MAXW or not cur.strip():
            lines[-1] = cand
        else:
            lines.append(indent + w)
    # never start a continuation line with a Markdown block marker
    for i in range(1, len(lines)):
        first = lines[i][len(indent):].split(' ', 1)[0]
        if BAD.match(first):
            prev = lines[i - 1].rsplit(' ', 1)
            lines[i - 1] = prev[0]
            lines[i] = indent + prev[1] + ' ' + lines[i][len(indent):]
    for i, l in enumerate(lines):
        if i > 0:
            assert len(l) <= MAXW, (len(l), l)
            assert not BAD.match(l[len(indent):].split(' ', 1)[0]), l
    return '\n'.join(lines)

def prose_insert(text, anchor, insert, indent):
    """Insert ' ' + insert right after a unique anchor; re-wrap only the line the anchor ends on."""
    assert text.count(anchor) == 1, ('anchor count', text.count(anchor), anchor)
    p = text.index(anchor) + len(anchor)
    ls = text.rfind('\n', 0, p) + 1
    le = text.find('\n', p)
    prefix, suffix = text[ls:p], text[p:le]
    block = wrap(prefix, insert + suffix, indent)
    return text[:ls] + block + text[le:]

def line_insert_after(text, anchor_line_end, new_block):
    """Insert whole new lines after the line a unique anchor ends."""
    assert text.count(anchor_line_end) == 1, anchor_line_end
    p = text.index(anchor_line_end) + len(anchor_line_end)
    assert text[p] == '\n'
    return text[:p + 1] + new_block + '\n' + text[p + 1:]

def row_replace(text, row_id, old, new):
    """Replace a unique string inside one table row (the line starting '| <row_id> |')."""
    assert new.count('|') == old.count('|'), 'pipe in new text'
    lines = text.split('\n')
    idx = [i for i, l in enumerate(lines) if l.startswith('| ' + row_id + ' |')]
    assert len(idx) == 1, (row_id, idx)
    l = lines[idx[0]]
    assert l.count(old) == 1, (row_id, l.count(old), old)
    n = l.replace(old, new)
    assert n.count('|') == l.count('|'), (row_id, 'pipes changed')
    lines[idx[0]] = n
    return '\n'.join(lines)

# ---------- ADR 0006 ----------
P6 = 'docs/adr/0006-infrastructure-operations.md'
t = base(P6)
t = line_insert_after(t, 'must the WebSocket fallback be\n  reachable on TCP 443?**',
    wrap('  -', 'Answered by infra-engineer and netcode-architect, confirmed by the Advisor (2026-11-06): '
         + tagged(word('ADR 0006, OQ-0006-16')), '    '))
t = line_insert_after(t, '- **OQ-0006-19. Is the goal budget provisional until slice 6 measures rooms per VM with the '
                         'real sim?**',
    wrap('  -', 'Answered by Mustafa (2026-11-06): (a), a lower bound, provisional until slice 6. (DENSITY-FIGURE)',
         '    '))
t = row_replace(t, 'PM-0006-11', '| every session |',
    '| every session. Bites: a header without the SMT map' + TAG + ' |')
t = row_replace(t, 'PM-0006-14', '(it feeds PM-0006-18) |',
    "(it feeds PM-0006-18). PM-0006-14 has no pass number and no bite; npt-judge's fixtures prove that short padding "
    'comes out INVALID' + TAG + ' |')
t = row_replace(t, 'PM-0006-26', '0 dropped in each |',
    '0 dropped in each. Bites: a rotation that closes live sessions' + TAG + ' |')
t = prose_insert(t, 'a control is a correct case\n  that must pass. The planted defects and controls are proposed.',
    ' Memory and queue marks bite over a full S30' + TAG + '.', '  ')
t = prose_insert(t, 'the research says only "3 times of day", r6 line 669).',
    ' ' + tagged(word('ADR 0006:1314-1316')), '  ')
open(f'{WT}/{P6}', 'w', encoding='utf-8').write(t)

# ---------- ADR 0001 ----------
P1 = 'docs/adr/0001-multiplayer-netcode.md'
t = base(P1)
R5 = tagged(word('ADR 0001, part 13'))
t = prose_insert(t, "The simulator that runs the profiles is ADR 0002's.", ' ' + R5, '    ')
t = prose_insert(t, "(R5-TEST-REC4, lines 1454–1461; ADR 0002's simulator).",
    ' ' + tagged(word('ADR 0001:1444-1445')), '  ')
t = prose_insert(t, 'lines 1991–1992, and names\n  no rate).', ' ' + R5, '  ')
r69 = word('ADR 0001, PM-0001-3 ')
first, rest = r69.split('. ', 1)
assert first == '1,000,000 packets per profile and direction on both layers, so PM-0001-3 and PM-0002-49 read one count'
t = row_replace(t, 'PM-0001-3', 'jitter over 100,000 packets within 5 % (relative)',
    'jitter over ' + first + TAG + ', within 5 % (relative)')
t = row_replace(t, 'PM-0001-3', 'his own base RTT recorded |',
    'his own base RTT recorded. ' + tagged(rest, '') + ' |')
t = row_replace(t, 'PM-0001-18', '| L1: 3 seeds; L2: 1 run; every client after every burst |',
    '| L1: 3 seeds; L2: 1 run; every client after every burst. Bites: a baseline that never falls back' + TAG + ' |')
R17 = tagged(word('ADR 0001, PM-0001-23 and -24'), '')
t = row_replace(t, 'PM-0001-23', '| 10 throttles per run (proposed), P2 otherwise |',
    '| 10 throttles per run (proposed), P2 otherwise. ' + R17 + ' |')
t = row_replace(t, 'PM-0001-24', "from the server's send stamp and the clock estimate |",
    "from the server's send stamp and the clock estimate. " + R17 + ' |')
t = row_replace(t, 'PM-0001-59', 'the match record and the crash report read back after the last injection |',
    'the match record and the crash report read back after the last injection. '
    + tagged(word('ADR 0001, PM-0001-59'), '') + ' |')
t = row_replace(t, 'PM-0001-28', '(proposed: the research\'s "clean local network") |',
    '(proposed: the research\'s "clean local network"). ' + tagged(word('ADR 0001, PM-0001-28'), '') + ' |')
t = row_replace(t, 'PM-0001-36', '| 1 session per browser and path |',
    '| 1 session per browser and path. Bites: the size read once, at start only' + TAG + ' |')
t = row_replace(t, 'PM-0001-55', '| 3 seeds |',
    '| 3 seeds. Bites: a vehicle collision that snaps' + TAG + ' |')
t = prose_insert(t, 'measured margin plus 10 % of the mark must fail it.',
    ' Memory and queue marks bite over a full S30' + TAG + '.', '  ')
t = prose_insert(t, 'the Riot Shield block) at\n  the attacker\'s view time.',
    " The cover half is answered by the Advisor's rule (2026-11-06), which Mustafa may overrule at the close. "
    + tagged(word('ADR 0001, OQ-0001-9')), '  ')
t = prose_insert(t, 'so the server measures the\nprofile\'s latency (PM-0001-3).',
    ' ' + tagged(word('ADR 0001, netcode checklist')), '')
open(f'{WT}/{P1}', 'w', encoding='utf-8').write(t)

# ---------- ADR 0004 ----------
P4 = 'docs/adr/0004-simulation-core.md'
t = base(P4)
t = row_replace(t, 'PM-0004-67',
    '| 3 runs of 36,000 ticks; each meets the mark. Bites: a loop that counts ticks from its timer period instead of '
    'elapsed time |',
    '| 3 runs of 36,000 ticks; each meets the mark. ' + tagged(word('ADR 0004, PM-0004-67'))
    + ' Bites: a loop that counts ticks from its timer period instead of elapsed time. PM-0004-67 keeps timer-period '
    'counting and gains two: an uncapped catch-up, and an uncounted drop' + TAG + ' |')
open(f'{WT}/{P4}', 'w', encoding='utf-8').write(t)

# ---------- SPIKE_PLAN ----------
PS = 'docs/SPIKE_PLAN.md'
t = base(PS)
MAC = ('Mustafa, 2026-11-06: "No Mac available for now (the loan Macs are out of stock). Nothing in this test waits on '
       'one: the timed Safari rows wait until a Mac is available, at the latest for the look proof test." So if '
       "GitHub's macOS runner fails condition (i), (ii) or (iii), every Safari row of the netcode proof test waits for "
       "that Mac too, instead of a Mac being borrowed before the test (the advisor's reading of it, as the plan "
       'applies it, 2026-11-06).')
t = prose_insert(t, "Safari rows are each spike's own plan's, in the same way (the advisor's reading, 2026-11-03).",
    ' ' + tagged(word('SPIKE_PLAN:53-55')) + ' ' + MAC, '  ')
FIX = word('SPIKE_PLAN:64-66 and 173').replace('(Mustafa, <date>)', '(Mustafa, 2026-11-06)')
assert '<date>' not in FIX and FIX.startswith('With FIXED-IP (a) (Mustafa, 2026-11-06), ')
ROUTE = ('ROUTE (Mustafa, 2026-11-06): the hands-on server work runs through cloud-init at creation and Claude Code on '
         'his laptop over SSH, with a key made on the laptop; this cloud session cannot reach the servers over SSH.')
t = prose_insert(t, "of the new SSH host keys, on that day (ADR 0006's key; the advisor's reading, 2026-11-03).",
    ' ' + tagged(FIX) + ' ' + ROUTE, '  ')
t = prose_insert(t, "issuance method chosen in this test's own plan with\n  security-auditor (the advisor's reading, "
                    '2026-11-03);',
    ' w' + tagged(FIX, ';')[1:], '  ')
t = prose_insert(t, 'and reports separately (item 2; the\n  advisor\'s reading, 2026-11-03).',
    ' ' + tagged(word('SPIKE_PLAN item 1, Place')), '  ')
open(f'{WT}/{PS}', 'w', encoding='utf-8').write(t)

# ---------- README register ----------
PR = 'docs/adr/README.md'
t = base(PR)
t = row_replace(t, 'OQ-0001-9', '| OQ-0001-9 | What else does lag compensation cover? |',
    "| OQ-0001-9 | What else does lag compensation cover? The cover half is answered by the Advisor's rule "
    '(2026-11-06), which Mustafa may overrule at the close. ' + tagged(word('ADR 0001, OQ-0001-9')) + ' |')
t = row_replace(t, 'OQ-0006-16', 'reachable on TCP 443? | infra-engineer with netcode-architect',
    'reachable on TCP 443? Answered by infra-engineer and netcode-architect, confirmed by the Advisor (2026-11-06): '
    + tagged(word('ADR 0006, OQ-0006-16')) + ' | infra-engineer with netcode-architect')
lines = t.split('\n')
idx = [i for i, l in enumerate(lines) if l.startswith('| OQ-0006-19 | ')]
assert len(idx) == 1
old = ("| OQ-0006-19 | Is the goal budget provisional until slice 6 measures rooms per VM with the real sim? "
       "(the test review's Q2) |")
assert lines[idx[0]].count(old) == 1
n = lines[idx[0]].replace(old, "| OQ-0006-19 (DENSITY-FIGURE) | Is the goal budget provisional until slice 6 measures "
    "rooms per VM with the real sim? (the test review's Q2) Answered by Mustafa (2026-11-06): (a), a lower bound, "
    'provisional until slice 6. |')
assert n.count('|') == lines[idx[0]].count('|')
lines[idx[0]] = n
t = '\n'.join(lines)
open(f'{WT}/{PR}', 'w', encoding='utf-8').write(t)
print('ok')
