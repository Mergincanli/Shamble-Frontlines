# R-STATE round 2: every tool_result with is_error set, in the session transcript and in every agent transcript
# (subagents/agent-*.jsonl and subagents/workflows/*/agent-*.jsonl), with the timestamp, the tool's name (from the
# matching tool_use) and the first characters of the error text (printed only, never saved). Reads only.
#   python3 -I .checkpoints/foundation-2-3-docs/r2-errors.py <session.jsonl> <session-dir> [chars]
import glob, json, os, sys

session, sdir = sys.argv[1], sys.argv[2]
chars = int(sys.argv[3]) if len(sys.argv) > 3 else 160
files = [session] + sorted(glob.glob(os.path.join(sdir, 'subagents', 'agent-*.jsonl'))) + \
    sorted(glob.glob(os.path.join(sdir, 'subagents', 'workflows', '*', 'agent-*.jsonl')))


def text_of(c):
    if isinstance(c, str):
        return c
    if isinstance(c, list):
        return ' '.join(b.get('text', '') for b in c if isinstance(b, dict))
    return ''


for f in files:
    names = {}
    rows = []
    for line in open(f, encoding='utf-8'):
        try:
            o = json.loads(line)
        except Exception:
            continue
        m = o.get('message') or {}
        c = m.get('content')
        if not isinstance(c, list):
            continue
        for b in c:
            if not isinstance(b, dict):
                continue
            if b.get('type') in ('tool_use', 'server_tool_use'):
                names[b.get('id')] = b.get('name')
            if b.get('type') == 'tool_result' and b.get('is_error'):
                t = text_of(b.get('content')).replace('\n', ' ')
                rows.append((o.get('timestamp'), names.get(b.get('tool_use_id'), '?'), t[:chars]))
    label = os.path.relpath(f, sdir) if f != session else 'SESSION'
    print(f'== {label}: {len(rows)} errors')
    for r in rows:
        print('  ', r[0], r[1], '|', r[2])
