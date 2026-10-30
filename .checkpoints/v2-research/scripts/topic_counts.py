"""Counts per topic file for the report: distinct ids of the topic's own findings, recommendations and open questions
(shared cross-topic open questions R<n>-XT-OQ<k> counted apart), the rows of the "Sources read" and "Sources not read"
tables, and the header's status label. Usage: python3 topic_counts.py [<commit>]   (default: the working tree)."""
import re, subprocess, sys

FILES = {'R1': 'r1-simulation-core.md', 'R2': 'r2-netcode.md', 'R3': 'r3-rendering.md',
         'R4': 'r4-physics-navigation.md', 'R5': 'r5-toolchain-quality.md', 'R6': 'r6-infrastructure.md'}
ref = sys.argv[1] if len(sys.argv) > 1 else None


def read(fn):
    if ref:
        return subprocess.run(['git', '-C', '/home/user/Shamble-Frontlines', 'show', f'{ref}:docs/research/{fn}'],
                              capture_output=True, text=True, check=True).stdout
    return open('/home/user/Shamble-Frontlines/docs/research/' + fn).read()


def table_rows(text, heading):
    m = re.search(r'^### ' + heading + r'\b.*?\n(.*?)(?=^#{2,3} |\Z)', text, re.S | re.M)
    if not m:
        return 0
    return sum(1 for l in m.group(1).splitlines() if l.startswith('| ') and not re.match(r'\|\s*#?\s*\|?\s*(URL|Source)\b', l))


print('| Topic | Findings | Recommendations | Open questions | Shared cross-topic OQs | Sources read | Not read | Status |')
print('|---|---|---|---|---|---|---|---|')
for t, fn in FILES.items():
    s = read(fn)
    ids = set(re.findall(r'\b' + t + r'-([A-Z0-9]+)-(F|REC|OQ)(\d+)[a-z]?\b', s))
    f = len({i for i in ids if i[1] == 'F'})
    r = len({i for i in ids if i[1] == 'REC'})
    oq = len({i for i in ids if i[1] == 'OQ' and i[0] != 'XT'})
    xt = len({i for i in ids if i[1] == 'OQ' and i[0] == 'XT'})
    st = re.search(r'\*\*Status:\*\*\s*([^.]*\))', s)
    print(f'| {t} | {f} | {r} | {oq} | {xt} | {table_rows(s, "Sources read")} | {table_rows(s, "Sources not read")} | '
          f'{st.group(1) if st else "?"} |')
