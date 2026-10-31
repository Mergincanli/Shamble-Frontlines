"""The report's coverage table (plan condition 12): every brief §4 question of R1–R6, with its part of the topic file
(its id prefix), its recommendations and open questions (id ranges, from the file's own question table), the lead
recommendation's heading, and the topic's verification label. Usage: python3 coverage_table.py [<commit>]"""
import re, subprocess, sys

FILES = {'R1': 'r1-simulation-core.md', 'R2': 'r2-netcode.md', 'R3': 'r3-rendering.md',
         'R4': 'r4-physics-navigation.md', 'R5': 'r5-toolchain-quality.md', 'R6': 'r6-infrastructure.md'}
ref = sys.argv[1] if len(sys.argv) > 1 else None


def read(fn):
    if ref:
        return subprocess.run(['git', '-C', '/home/user/Shamble-Frontlines', 'show', f'{ref}:docs/research/{fn}'],
                              capture_output=True, text=True, check=True).stdout
    return open('/home/user/Shamble-Frontlines/docs/research/' + fn).read()


def plain(t):
    t = re.sub(r'\[([^\]]+)\]\([^)]*\)', r'\1', t)       # links read as their words
    return re.sub(r'[*`]', '', t).strip()


print('| Topic | # | Brief question | Recommendations | Lead recommendation (its heading, shortened) | Open questions '
      '| Verification |')
print('|---|---|---|---|---|---|---|')
for t, fn in FILES.items():
    s = read(fn)
    label = re.search(r'\*\*Status:\*\*\s*([^.]*\))', s)
    label = label.group(1) if label else '?'
    head = s.split('\n## ', 2)
    for row in re.finditer(r'^\| *(\d+) *\| *(.+?) *\|(.*)$', s[:20000], re.M):
        num, q, rest = row.group(1), plain(row.group(2)), row.group(3)
        ids = re.search(r'(' + t + r'-[A-Z0-9]+)-F\d+', rest) or re.search(r'(' + t + r'-[A-Z0-9]+)-(?:REC|OQ)\d+', rest)
        if not ids:
            print(f'| {t} | {num} | {q} | – | – | {plain(rest.split("|")[0])[:60]} | {label} |')
            continue
        pre = ids.group(1)
        recs = re.search(r'REC\d+[–-]REC\d+|REC\d+', rest)
        oqs = re.search(r'OQ\d+[–-]OQ\d+|OQ\d+', rest)
        lead = re.search(r'\*\*' + re.escape(pre) + r'-REC1\.\s*(.+?)(?:\*\*|\n\n)', s, re.S)
        lead = ' '.join(plain(lead.group(1)).split()) if lead else '–'
        lead = (lead[:110] + '…') if len(lead) > 110 else lead
        print(f'| {t} | {num} | {q} | {pre}-{recs.group(0) if recs else "–"} | {lead} | '
              f'{pre}-{oqs.group(0) if oqs else "–"} | {label} |')
