# Verifier (read-only): independent look at the advisor tool in the session transcript.
# Prints timestamps, entry types, attachment types, model names, booleans and lengths only; never message text.
#   python3 -I .checkpoints/foundation-2-3-docs/v-advisor.py
import json
T = '/root/.claude/projects/-home-user-Shamble-Frontlines/7261f3d7-f651-586a-9c39-c052553b68a3.jsonl'
rows = []
for n, line in enumerate(open(T, encoding='utf-8'), 1):
    try:
        o = json.loads(line)
    except Exception:
        continue
    ts = o.get('timestamp')
    t = o.get('type')
    att = o.get('attachment') or {}
    if t == 'attachment' and 'advisor' in json.dumps(att.get('type', '')).lower():
        keys = sorted(att.keys())
        model = att.get('model') or att.get('advisorModel') or att.get('advisor_model')
        print('ATTACH', n, ts, att.get('type'), 'keys', keys, 'model', model)
    m = o.get('message') or {}
    c = m.get('content')
    if isinstance(c, list):
        for b in c:
            if isinstance(b, dict) and (b.get('name') == 'advisor' or 'advisor' in str(b.get('type', ''))):
                print('BLOCK', n, ts, t, 'sidechain', bool(o.get('isSidechain')), 'btype', b.get('type'), 'name', b.get('name'))
    its = (m.get('usage') or {}).get('iterations') or []
    adv = [x.get('model') for x in its if isinstance(x, dict) and x.get('type') == 'advisor_message']
    if adv:
        print('USAGE', n, ts, 'msgid_tail', str(m.get('id'))[-6:], 'advisor_message models', adv)
    if t == 'system':
        sub = o.get('subtype')
        content = o.get('content')
        s = content if isinstance(content, str) else json.dumps(content)
        if s and ('advisor' in s.lower() or 'fable' in s.lower()):
            print('SYSTEM', n, ts, 'subtype', sub, 'len', len(s), 'has_Fable5.2', 'Fable 5.2' in s, 'has_Opus5.5', 'Opus 5.5' in s)
