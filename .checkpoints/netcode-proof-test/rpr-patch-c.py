# Patch rpr-edit.py with the coordinator's consistency fixes C1-C8 (idempotent: refuses to patch twice).
p = '/home/user/Shamble-Frontlines/.checkpoints/netcode-proof-test/rpr-edit.py'
s = open(p, encoding='utf-8').read()
assert 'C1-C8' not in s, 'already patched'

def sub(old, new):
    global s
    assert s.count(old) == 1, ('count', s.count(old), old[:70])
    s = s.replace(old, new)

# helper: replace a unique string and re-wrap only the lines it spans
sub("def line_insert_after(text, anchor_line_end, new_block):",
'''def prose_replace(text, old, new, indent):
    """Replace a unique string (may span lines) and re-wrap only the lines it touches."""
    assert text.count(old) == 1, ('old count', text.count(old), old)
    a = text.index(old)
    b = a + len(old)
    ls = text.rfind('\\n', 0, a) + 1
    le = text.find('\\n', b)
    prefix = text[ls:a].rstrip(' ')
    block = wrap(prefix, new + text[b:le], indent)
    return text[:ls] + block + text[le:]

# C1-C8: the coordinator's consistency fixes (2026-11-06)

def line_insert_after(text, anchor_line_end, new_block):''')

# C4: OQ-0006-16's status, in ADR 0006 and the register
sub("""    wrap('  -', 'Answered by infra-engineer and netcode-architect, confirmed by the Advisor (2026-11-06): '""",
    """    wrap('  -', 'Answered in part, for the netcode proof test, by infra-engineer and netcode-architect, confirmed by '
         'the Advisor (2026-11-06); the TCP 443 question stays open until slice 3. '""")
sub("""    'reachable on TCP 443? Answered by infra-engineer and netcode-architect, confirmed by the Advisor (2026-11-06): '""",
    """    'reachable on TCP 443? Answered in part, for the netcode proof test, by infra-engineer and netcode-architect, '
    'confirmed by the Advisor (2026-11-06); the TCP 443 question stays open until slice 3. '""")

# C5: ADR 0006's copy of the bite sentence glosses S30
sub("""t = prose_insert(t, 'a control is a correct case\\n  that must pass. The planted defects and controls are proposed.',
    ' Memory and queue marks bite over a full S30' + TAG + '.', '  ')""",
    """t = prose_insert(t, 'a control is a correct case\\n  that must pass. The planted defects and controls are proposed.',
    " Memory and queue marks bite over a full S30 (ADR 0001's 30-minute soak, its key's Soaks)" + TAG + '.', '  ')
# C7: pointers to Mustafa's DENSITY-FIGURE answer
t = row_replace(t, 'PM-0006-14', "OQ-0006-19's option (a), the interim;",
    "OQ-0006-19's option (a), the interim (answered by Mustafa, 2026-11-06: (a));")
t = prose_insert(t, '(proposed interim, OQ-0006-19)', ' (answered by Mustafa, 2026-11-06: (a))', '  ')
t = prose_replace(t, "the Advisor's reading, before the netcode proof test's density runs.",
    "the Advisor's reading, before the netcode proof test's density runs (answered by Mustafa, 2026-11-06: (a)).",
    '    ')""")

# C8: the key's Profiles bullet gets a pointer instead of R-5's second copy
sub("""t = prose_insert(t, 'lines 1991–1992, and names\\n  no rate).', ' ' + R5, '  ')""",
    """t = prose_insert(t, 'lines 1991–1992, and names\\n  no rate).',
    " An explicit reorder step joins the profiles (part 13, the advisor's reading, 2026-11-06).", '  ')""")

# C2: PM-0001-3's delay formula moves into the method cell; the duplicate at the pass cell's end goes
sub("""    'his own base RTT recorded. ' + tagged(rest, '') + ' |')""",
    """    'his own base RTT recorded. ' + tagged(probe, '') + ' |')
FORMULA = "(the profile's RTT − the path's measured base RTT) ÷ 2"
assert delay == 'The injected delay in each direction is ' + FORMULA + '.'
t = row_replace(t, 'PM-0001-3', "set to half of the profile's RTT minus the path's measured base RTT",
    'set per direction to ' + FORMULA + TAG)
# C1 + C2: the runs cell: block, not session; test-engineer digest F's five bites
t = row_replace(t, 'PM-0001-3', 'and at the start of every L2 session |',
    'and at the start of every L2 block' + TAG + '. Bites: the full round trip applied each way fails the mean-delay '
    'band; loss drawn once per round trip fails the per-direction loss; a normal jitter in place of the uniform fails '
    "the jitter check; the RNG seeded from the clock fails the fate-log identity; on L2, the delay not net of the base "
    "RTT fails MP's 100 ± 5 ms" + TAG + ' |')
# C4: pointer at ADR 0001 part 10's port note
t = prose_insert(t, 'confirmed by the Advisor); until it is answered',
    ' (answered in part for the netcode proof test, 2026-11-06: ADR 0006, OQ-0006-16)', '    ')
# C6: pointer at ADR 0001's MACHINES-TIMING summary
t = prose_insert(t, '(if not, "I borrow the Mac before the netcode proof test")',
    " (overtaken by Mustafa's answer of 2026-11-06, SPIKE_PLAN's Machines: no Mac is available, and the Safari rows "
    'wait for one)', '  ')""")
sub("""first, rest = r69.split('. ', 1)""",
    """first, rest = r69.split('. ', 1)
probe, delay = rest.split('. The injected', 1)
probe, delay = probe + '.', 'The injected' + delay""")

# C3: the checklist's old clause loses its cap half (after R-28's insertion)
sub("""t = prose_insert(t, 'so the server measures the\\nprofile\\'s latency (PM-0001-3).',
    ' ' + tagged(word('ADR 0001, netcode checklist')), '')""",
    """t = prose_insert(t, 'so the server measures the\\nprofile\\'s latency (PM-0001-3).',
    ' ' + tagged(word('ADR 0001, netcode checklist')), '')
t = prose_replace(t, "The profile and the cap change from the overlay (a spike-only control on the server's impairment\\n"
    'layer);',
    "The profile changes from the overlay (a spike-only control on the server's impairment layer), and the cap is set "
    "as the advisor's reading below says (2026-11-06);", '')""")

# C6: pointer at SPIKE_PLAN's Depends-on Mac clause
sub("""t = prose_insert(t, 'and reports separately (item 2; the\\n  advisor\\'s reading, 2026-11-03).',""",
    """t = prose_insert(t, 'the Mac is borrowed before this test only if they cannot pass it (Machines, above)',
    " (overtaken by Mustafa's answer of 2026-11-06, Machines above: no Mac is available, and the Safari rows wait for "
    'one)', '  ')
t = prose_insert(t, 'and reports separately (item 2; the\\n  advisor\\'s reading, 2026-11-03).',""")

open(p, 'w', encoding='utf-8').write(s)
print('patched')
