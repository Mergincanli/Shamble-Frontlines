"""Save a background agent's hand-back report verbatim: save_handback.py <task output jsonl> <out.md> <title line>"""
import json, sys
src, out, title = sys.argv[1:4]
msg = None
for l in open(src):
    try:
        d = json.loads(l)
    except ValueError:
        continue
    m = d.get('message') or {}
    if isinstance(m, dict) and m.get('role') == 'assistant':
        for c in m.get('content') or []:
            if c.get('type') == 'tool_use' and c.get('name') == 'SubagentHandback':
                msg = c['input'].get('message')
assert msg, 'no hand-back found'
open(out, 'w').write(f'# {title} (verbatim)\n\n' + msg.strip() + '\n')
print('saved', out, len(msg), 'chars')
