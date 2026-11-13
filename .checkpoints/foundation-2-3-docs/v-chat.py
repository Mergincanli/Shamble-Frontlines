# Verifier (read-only): main-session assistant text blocks (non-sidechain) that mention the advisor or a model name,
# with timestamps; prints the session's own text windows on the console only (never his words; user entries skipped).
#   python3 -I .checkpoints/foundation-2-3-docs/v-chat.py <until-iso> [all]
import json, re, sys
T = '/root/.claude/projects/-home-user-Shamble-Frontlines/7261f3d7-f651-586a-9c39-c052553b68a3.jsonl'
until = sys.argv[1]
show_all = len(sys.argv) > 2
pat = re.compile(r'(advisor|fable|opus|claude-)', re.I)
n_text = 0
for ln, line in enumerate(open(T, encoding='utf-8'), 1):
    o = json.loads(line)
    if o.get('type') != 'assistant' or o.get('isSidechain'):
        continue
    ts = o.get('timestamp', '')
    if ts >= until:
        continue
    for b in (o.get('message') or {}).get('content') or []:
        if isinstance(b, dict) and b.get('type') == 'text':
            n_text += 1
            t = b.get('text', '')
            hits = [m.start() for m in pat.finditer(t)]
            if show_all or hits:
                print(f'== line {ln} {ts} len {len(t)} hits {len(hits)}')
                for h in hits[:12]:
                    print('   ...', t[max(0, h-90):h+110].replace('\n', ' '), '...')
print('main-session text blocks before', until, ':', n_text)
