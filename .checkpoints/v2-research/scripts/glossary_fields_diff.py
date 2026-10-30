"""Which GLOSSARY.md fields changed between two commits (or the working tree, 'WT'), entry by entry, with the header
and the sections outside entries compared as text. Usage: python3 glossary_fields_diff.py <base> <head>"""
import re, subprocess, sys

def show(ref):
    if ref == 'WT':
        return open('/home/user/Shamble-Frontlines/docs/research/GLOSSARY.md').read()
    return subprocess.run(['git', '-C', '/home/user/Shamble-Frontlines', 'show', f'{ref}:docs/research/GLOSSARY.md'],
                          capture_output=True, text=True, check=True).stdout

def parse(text):
    entries, rest = {}, text
    for m in re.finditer(r'^### (.+?)\n(.*?)(?=^### |^## |\Z)', text, re.S | re.M):
        body = m.group(2)
        fields = dict((k, ' '.join(v.split())) for k, v in
                      re.findall(r'^- \*\*([^*]+?):\*\*(.*?)(?=^- \*\*|\Z)', body, re.S | re.M))
        lead = ' '.join(body.split('\n- **', 1)[0].split())
        entries[m.group(1).strip()] = (lead, fields)
        rest = rest.replace(m.group(0), '')
    return entries, ' '.join(rest.split())

(eb, rb), (ea, ra) = parse(show(sys.argv[1])), parse(show(sys.argv[2]))
assert set(eb) == set(ea), ('entries differ', set(eb) ^ set(ea))
changed = {}
for name in eb:
    lb, fb = eb[name]; la, fa = ea[name]
    ks = sorted(k for k in set(fb) | set(fa) if fb.get(k) != fa.get(k)) + (['(text before the fields)'] if lb != la else [])
    if ks:
        changed[name] = ks
import collections
c = collections.Counter(k for ks in changed.values() for k in ks)
print('entries', len(eb), '; changed fields by name:', dict(c))
for name, ks in changed.items():
    if ks != ['Used today']:
        print('  ', name, ':', ks)
print('text outside the entries changed:', rb != ra)
