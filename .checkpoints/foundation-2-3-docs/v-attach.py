# Verifier (read-only): print the harness's attachment payloads of one type in a window (harness text, console only).
#   python3 -I .checkpoints/foundation-2-3-docs/v-attach.py <type> <from-iso> <until-iso> [max]
import json, sys
T = '/root/.claude/projects/-home-user-Shamble-Frontlines/7261f3d7-f651-586a-9c39-c052553b68a3.jsonl'
typ, a, z = sys.argv[1], sys.argv[2], sys.argv[3]
mx = int(sys.argv[4]) if len(sys.argv) > 4 else 3
n = 0
for ln, line in enumerate(open(T, encoding='utf-8'), 1):
    o = json.loads(line)
    ts = o.get('timestamp') or ''
    att = o.get('attachment') or {}
    if o.get('type') == 'attachment' and att.get('type') == typ and a <= ts < z:
        n += 1
        if n <= mx:
            print('==', ln, ts, json.dumps(att)[:900])
print('count', n)
