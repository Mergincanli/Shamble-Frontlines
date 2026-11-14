# Verifier (read-only): for each hook/harness refusal in the session and agent transcripts, print the time, the file,
# the refusal text (the guard's/harness's own words) and the refused command's first 100 characters (console only).
import glob, json, os
S = '/root/.claude/projects/-home-user-Shamble-Frontlines/7261f3d7-f651-586a-9c39-c052553b68a3'
files = [S + '.jsonl'] + sorted(glob.glob(S + '/subagents/agent-*.jsonl')) + sorted(glob.glob(S + '/subagents/workflows/*/agent-*.jsonl'))
for f in files:
    uses = {}
    for line in open(f, encoding='utf-8'):
        try: o = json.loads(line)
        except Exception: continue
        c = (o.get('message') or {}).get('content')
        if not isinstance(c, list): continue
        for b in c:
            if not isinstance(b, dict): continue
            if b.get('type') == 'tool_use':
                uses[b['id']] = str((b.get('input') or {}).get('command', ''))
            if b.get('type') == 'tool_result' and b.get('is_error'):
                cc = b.get('content')
                t = cc if isinstance(cc, str) else ' '.join(x.get('text', '') for x in cc if isinstance(x, dict)) if isinstance(cc, list) else ''
                if 'hook error' in t or 'Blocked' in t or 'denied' in t.lower() or 'not allowed' in t.lower():
                    print('==', os.path.relpath(f, S) if f != S + '.jsonl' else 'SESSION', o.get('timestamp'))
                    print('   refusal:', t[:420].replace('\n', ' '))
                    print('   command:', uses.get(b.get('tool_use_id'), '?')[:100].replace('\n', ' '))
