# Verifier (read-only): every main-session assistant text block in a time window, in full, console only (the
# session's own words; user entries are skipped). Also lists user/system entries' type and timestamp in the window.
#   python3 -I .checkpoints/foundation-2-3-docs/v-window.py <from-iso> <until-iso>
import json, sys
T = '/root/.claude/projects/-home-user-Shamble-Frontlines/7261f3d7-f651-586a-9c39-c052553b68a3.jsonl'
a, z = sys.argv[1], sys.argv[2]
for ln, line in enumerate(open(T, encoding='utf-8'), 1):
    o = json.loads(line)
    ts = o.get('timestamp') or ''
    if not (a <= ts < z):
        continue
    t = o.get('type')
    if t == 'assistant' and not o.get('isSidechain'):
        for b in (o.get('message') or {}).get('content') or []:
            if isinstance(b, dict) and b.get('type') == 'text':
                print(f'== TEXT line {ln} {ts} len {len(b.get("text",""))}')
                print(b.get('text', ''))
    elif t in ('user', 'system') or (t == 'attachment'):
        sub = o.get('subtype') or ((o.get('attachment') or {}).get('type') if t == 'attachment' else '')
        print(f'-- {t} line {ln} {ts} {sub} isMeta={bool(o.get("isMeta"))} compact={bool(o.get("isCompactSummary"))}')
