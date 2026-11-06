# Extract the advisor's exact words for R-PR from advisor-verdict.md (joined, unwrapped).
import re, json, sys
src = open('/home/user/Shamble-Frontlines/.checkpoints/netcode-proof-test/advisor-verdict.md', encoding='utf-8').read()
sec = src.split('## Exact words for R-PR', 1)[1].split('\n## ', 1)[0]
items = []
cur = None
for line in sec.split('\n')[1:]:
    if line.startswith('- **'):
        if cur: items.append(cur)
        cur = line[2:]
    elif line.startswith('  ') and cur is not None:
        cur += ' ' + line.strip()
if cur: items.append(cur)
out = {}
for it in items:
    m = re.match(r'\*\*(.+?):\*\* "(.*)"$', it)
    assert m, it[:80]
    out[m.group(1)] = m.group(2)
json.dump(out, open(sys.argv[1], 'w', encoding='utf-8'), ensure_ascii=False, indent=1)
for k, v in out.items():
    print('==', k); print(v)
