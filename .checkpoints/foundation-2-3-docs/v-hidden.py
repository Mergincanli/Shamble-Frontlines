# Verifier (read-only): the scan's blind spots in the main transcript. (1) tool_result blocks that carry a person's
# rejection or typed feedback (Claude Code's wording for a refused tool call); (2) attachments of any type other than
# queued_command whose payload names a human origin or carries a prompt; (3) Skill tool calls by the session near a
# time. Prints timestamps, types and booleans only (no message text).
#   python3 -I .checkpoints/foundation-2-3-docs/v-hidden.py
import json, re
T = '/root/.claude/projects/-home-user-Shamble-Frontlines/7261f3d7-f651-586a-9c39-c052553b68a3.jsonl'
rej = re.compile(r"(doesn't want to proceed|user said|the user provided|user rejected|rejected this|User denied|denied by the user|user feedback)", re.I)
att_types = {}
for ln, line in enumerate(open(T, encoding='utf-8'), 1):
    try:
        o = json.loads(line)
    except Exception:
        continue
    ts = o.get('timestamp')
    c = (o.get('message') or {}).get('content')
    if isinstance(c, list):
        for b in c:
            if isinstance(b, dict) and b.get('type') == 'tool_result':
                cc = b.get('content')
                s = cc if isinstance(cc, str) else ' '.join(x.get('text', '') for x in cc if isinstance(x, dict)) if isinstance(cc, list) else ''
                if rej.search(s[:600]):
                    print('TOOL_RESULT-REJECTION?', ln, ts, 'is_error', b.get('is_error'), 'len', len(s), 'match', rej.search(s[:600]).group(0))
            if isinstance(b, dict) and b.get('type') == 'tool_use' and b.get('name') == 'Skill' and not o.get('isSidechain'):
                print('SKILL-CALL', ln, ts, 'skill', (b.get('input') or {}).get('skill'))
    if o.get('type') == 'attachment':
        a = o.get('attachment') or {}
        t = a.get('type')
        att_types[t] = att_types.get(t, 0) + 1
        origin = (a.get('origin') or {}).get('kind') if isinstance(a.get('origin'), dict) else None
        if t != 'queued_command' and (origin == 'human' or 'prompt' in a):
            print('ATTACH-WITH-PROMPT-OR-HUMAN', ln, ts, t, 'origin', origin, 'has_prompt', 'prompt' in a)
    if o.get('type') == 'user' and not o.get('isMeta') and isinstance((o.get('message') or {}).get('content'), str):
        k = (o.get('origin') or {}).get('kind') if isinstance(o.get('origin'), dict) else None
        print('USER-STRING', ln, ts, 'origin', k, 'compact', bool(o.get('isCompactSummary')), 'uuid8', str(o.get('uuid'))[:8])
print('attachment types:', dict(sorted(att_types.items())))
