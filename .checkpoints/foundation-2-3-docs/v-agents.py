# Verifier (read-only): per agent transcript: meta.json, the models of its assistant messages, first/last timestamps,
# and its brief's first 110 characters (console only).
import glob, json, os, collections
S = '/root/.claude/projects/-home-user-Shamble-Frontlines/7261f3d7-f651-586a-9c39-c052553b68a3'
for f in sorted(glob.glob(S + '/subagents/agent-*.jsonl')) + sorted(glob.glob(S + '/subagents/workflows/*/agent-*.jsonl')):
    meta = f.replace('.jsonl', '.meta.json')
    m = json.load(open(meta)) if os.path.exists(meta) else {}
    models = collections.Counter(); first = last = None; brief = None
    for line in open(f, encoding='utf-8'):
        try: o = json.loads(line)
        except Exception: continue
        ts = o.get('timestamp')
        if ts:
            first = first or ts; last = ts
        if o.get('type') == 'assistant':
            models[(o.get('message') or {}).get('model')] += 1
        if brief is None and o.get('type') == 'user':
            c = (o.get('message') or {}).get('content')
            brief = c if isinstance(c, str) else ' '.join(x.get('text', '') for x in (c or []) if isinstance(x, dict))
    print('==', os.path.relpath(f, S))
    print('   meta:', json.dumps(m)[:400])
    print('   models:', dict(models), '|', first, '->', last)
    print('   brief:', (brief or '')[:110].replace('\n', ' '))
