# Verifier (read-only): for a refusal at a timestamp in an agent transcript: does the refused command hold a loop
# (for/while ... do) and a redirect; and the next 3 tool calls' names, descriptions and command starts (console only).
import json, sys, re
f, ts = sys.argv[1], sys.argv[2]
rows = []
for line in open(f, encoding='utf-8'):
    try: o = json.loads(line)
    except Exception: continue
    c = (o.get('message') or {}).get('content')
    if not isinstance(c, list): continue
    for b in c:
        if isinstance(b, dict) and b.get('type') == 'tool_use':
            rows.append((o.get('timestamp'), b.get('name'), b.get('input') or {}))
idx = max(i for i, r in enumerate(rows) if r[0] <= ts)
cmd = str(rows[idx][2].get('command', ''))
print('refused at', rows[idx][0], rows[idx][1], '| loop:', bool(re.search(r'\b(for|while)\b[^\n]*;\s*do\b|\bdo\b', cmd)), '| redirect:', '>' in cmd, '| sed -i:', 'sed -i' in cmd, '| node -e:', 'node -e' in cmd, '| has #:', '#' in cmd)
for r in rows[idx+1: idx+4]:
    inp = r[2]
    print('  next', r[0], r[1], '|', str(inp.get('description', ''))[:90], '|', str(inp.get('command', inp.get('file_path', '')))[:110].replace('\n', ' '))
