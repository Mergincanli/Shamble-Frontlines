# Verifier (read-only): print an advisor_tool_result's text (the advisor model's words, console only) at a line.
#   python3 -I .checkpoints/foundation-2-3-docs/v-advres.py <line>
import json, sys
T = '/root/.claude/projects/-home-user-Shamble-Frontlines/7261f3d7-f651-586a-9c39-c052553b68a3.jsonl'
n = int(sys.argv[1])
o = json.loads(open(T, encoding='utf-8').read().split('\n')[n-1])
for b in (o.get('message') or {}).get('content') or []:
    if isinstance(b, dict) and b.get('type') == 'advisor_tool_result':
        c = b.get('content')
        print(json.dumps(c)[:6000] if not isinstance(c, str) else c[:6000])
