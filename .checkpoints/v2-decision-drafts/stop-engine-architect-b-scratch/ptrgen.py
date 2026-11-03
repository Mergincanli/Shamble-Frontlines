"""Pointer step (condition 5, POINTERS (b)): from the scratch README with every earlier block applied, write one FIX
block per line that carries a line pointer into the drafts; each pointer keeps its file link and names its OQ id,
part or section. Checks that each named OQ id exists in the linked draft. Prints the blocks to stdout."""
import re, glob
T = 'tree/docs/adr/README.md'
L = open(T, encoding='utf8').read().split('\n')
heads = {}
for f in glob.glob('/home/user/Shamble-Frontlines/docs/adr/000[1-6]-*.md'):
    heads[re.search(r'(000\d)', f).group(1)] = set(re.findall(r'\*\*(OQ-000\d-[A-Z]*\d+)[.*]', open(f, encoding='utf8').read()))
P = re.compile(r'\[ADR (000\d)\]\(([^)]*)\)( part (\d+),)? line \d+')
out = []; n = 0; problems = []
for i, l in enumerate(L, 1):
    new = l
    if i in (10, 11):
        continue
    if P.search(l):
        first = l.split('|')[1]
        ids = re.findall(r'OQ-\d{4}-[A-Z]*\d+', first)
        def rep(m):
            adr, link, part = m.group(1), m.group(2), m.group(4)
            if 'line 76 and the asset pipeline part' in l and adr == '0005' and not ids:
                return f'[ADR {adr}]({link}) "Settled inputs" (ASSET-RESEARCH (a))'
            if not ids:
                problems.append((i, 'no OQ id in the first cell')); return m.group(0)
            oq = ids[0]
            if oq not in heads[adr]:
                problems.append((i, f'{oq} not in ADR {adr}'))
            return f'[ADR {adr}]({link}) part {part} and {oq}' if part else f'[ADR {adr}]({link}) {oq}'
        new = P.sub(rep, l)
    if ' (line 162 at 604a497)' in new:
        new = new.replace(' (line 162 at 604a497)', '')
    if new != l:
        n += 1
        out.append((i, l, new))
print(f'# {n} lines changed; problems: {problems}', flush=True)
import json
json.dump(out, open('ptr-lines.json', 'w'), ensure_ascii=False)
