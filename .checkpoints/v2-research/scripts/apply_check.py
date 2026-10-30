"""Step 3c apply check: is each approved change of resolutions.md in its file?

For every change item C<rank>.<n> of resolutions.md (as approved, 2026-10-30): with an "after" text, whether that text
is in the file at the given commit (spaces and line breaks normalized; glossary links read as their words, so
"[the stress scene](GLOSSARY.md#the-stress-scene)" counts as "the stress scene", and the "(GLOSSARY)" first-use marks
of the after texts are dropped); with "insert <code>", whether the file's "Shared open questions" subsection holds
the bold code; the C17.b and C23.b items, which wait for BASELINE-WHEN, must be absent. A miss is not an error by
itself (a later glossary rename may change a word of an after text); the step-3d verifiers judge each one.
Usage: python3 apply_check.py <commit> [--topics R5,R6] [--out out.tsv]
"""
import re, subprocess, sys

C = '/home/user/Shamble-Frontlines/.checkpoints/v2-research/'
FILES = {'r1': 'r1-simulation-core.md', 'r2': 'r2-netcode.md', 'r3': 'r3-rendering.md',
         'r4': 'r4-physics-navigation.md', 'r5': 'r5-toolchain-quality.md', 'r6': 'r6-infrastructure.md'}
ref = sys.argv[1]
topics = sys.argv[sys.argv.index('--topics') + 1].lower().split(',') if '--topics' in sys.argv else list(FILES)
out = sys.argv[sys.argv.index('--out') + 1] if '--out' in sys.argv else None


def show(fn):
    if ref == 'WT':   # the working tree, for a file not yet committed
        return open('/home/user/Shamble-Frontlines/docs/research/' + fn).read()
    return subprocess.run(['git', '-C', '/home/user/Shamble-Frontlines', 'show', f'{ref}:docs/research/{fn}'],
                          capture_output=True, text=True, check=True).stdout


def norm(t):
    t = re.sub(r'\(\[GLOSSARY\]\(GLOSSARY\.md[^)]*\)\)', '', t)          # a "([GLOSSARY](GLOSSARY.md#x))" link
    t = re.sub(r'\[([^\]]+)\]\(GLOSSARY\.md[^)]*\)', r'\1', t)          # a linked term reads as its words
    t = re.sub(r';?\s*\[GLOSSARY\]\(GLOSSARY\.md[^)]*\)', '', t)    # a "; [GLOSSARY](GLOSSARY.md#x)" mark
    t = t.replace('(GLOSSARY)', '').replace('; GLOSSARY)', ')').replace('GLOSSARY: ', '')
    t = re.sub(r'\s+([,.;:)])', r'\1', ' '.join(t.split()))           # the space a dropped mark leaves
    return t


def backtick_text(body, label):
    """The text of a "- label:" sub-bullet between its opening backtick and its closing one: the first backtick at the
    end of a line with an even number of backticks before it (inner backticks are the file's own, in pairs). The text
    may run over lines that start at column 0 (the file's own line breaks)."""
    m = re.search(r'^  - ' + label + r':[ \t]*`', body, re.M)
    if not m:
        return None
    a = m.end()
    inner = 0
    for k in range(a, len(body)):
        if body[k] == '`':
            if inner % 2 == 0 and re.match(r'[ \t]*(\n|$)', body[k + 1:]):
                return body[a:k]
            inner += 1
    return None


def inline_after(body):
    """An item written on one line: '"before" → "after"' or 'append "text"'."""
    head = body.split('\n  - ')[0]
    m = re.search(r'→\s*"(.*)"', head, re.S) or re.search(r'\bappend\s+"(.*)"', head, re.S)
    return m.group(1) if m else None


res = open(C + 'resolutions.md').read()
texts = {t: norm(show(FILES[t])) for t in topics}
raw = {t: show(FILES[t]) for t in topics}
rows = []
current = None   # the topic of the per-file list an item sits in ("R5:" lines)
for m in re.finditer(r'^(?:(R[1-6](?:, R[1-6])*):\s*$|- (C\d+\.b?\d+[a-z]?)\b(.*?)(?=^- C\d|^## |^### |^R[1-6][:,–]|^\*Changes only|\Z))',
                     res, re.S | re.M):
    if m.group(1):
        current = m.group(1).split(',')[0].lower()
        continue
    cid, body = m.group(2), m.group(3)
    first = body.split('\n')[0]
    f = re.search(r'\b(r[1-6]):', first) or re.search(r'\b(R[1-6])-[A-Z]', first)
    t = f.group(1).lower() if f else current
    if t not in topics:
        continue
    after = backtick_text(body, 'after') or inline_after(body)
    waits = '.b' in cid
    if after is not None:
        found = norm(after) in texts[t]
        status = ('applied, but waits for BASELINE-WHEN' if found else 'not applied (waits for BASELINE-WHEN): right') \
            if waits else ('after text found' if found else 'after text NOT found verbatim')
    else:
        code = re.search(r'insert (?:the shared question )?\*?\*?([A-Z]+(?:-[A-Z0-9]+)+)', body)
        if code:
            sec = re.search(r'^### Shared open questions.*?(?=^### |^## |\Z)', raw[t], re.S | re.M)
            found = bool(sec and ('**' + code.group(1)) in sec.group(0))
            status = (f'insert {code.group(1)}: ' + ('found' if found else 'NOT found')) + \
                (' (waits for BASELINE-WHEN)' if waits else '')
        else:
            status = 'no after text or insert (read by hand)'
    rows.append((cid, t.upper(), status))
for r in rows:
    print('\t'.join(r))
bad = [r for r in rows if 'NOT' in r[2] or 'but waits' in r[2]]
print(f'items {len(rows)}; to read by hand {len(bad)}')
if out:
    with open(out, 'w') as fo:
        fo.write('change id\ttopic\tstatus\n' + ''.join('\t'.join(r) + '\n' for r in rows))
