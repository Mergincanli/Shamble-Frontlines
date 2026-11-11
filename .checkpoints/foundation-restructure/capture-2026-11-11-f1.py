# Writes mustafa-2026-11-11-f1.md: Mustafa's message of 2026-11-11, 14:54:11 UTC, in this session (F1-FIX, FREEZE-FDN,
# his yes to the exact gate change), extracted from the session's own events (list_events output saved by the tools),
# never retyped. Run: python3 -I capture-2026-11-11-f1.py <events-file>
import json, sys, pathlib

HERE = pathlib.Path(__file__).resolve().parent
UUID = '8e7ad9dc-cdf8-43a7-817b-ceaaba67081e'
found = []

def walk(o):
    if isinstance(o, dict):
        u = o.get('user')
        if isinstance(u, dict) and u.get('uuid') == UUID:
            c = ((u.get('internal_anthropic_catchall') or {}).get('message') or {}).get('content')
            assert isinstance(c, str), 'content is not a plain message'
            found.append((o.get('created_at'), c))
        for v in o.values():
            walk(v)
    elif isinstance(o, list):
        for v in o:
            walk(v)

walk(json.loads(pathlib.Path(sys.argv[1]).read_text()))
assert len(found) == 1, f'found {len(found)} events'
when, text = found[0]
assert text.startswith('My words of 2026-11-11, for the foundation chat (PR #50, foundation step 2.1a).'), text[:80]
assert '```' not in text
out = f"""# Mustafa's message of 2026-11-11, 14:54 UTC, in the replacement session, word for word

Source: his chat message in session_019YNMUWLffMoE47SbvkqJ5P (his personal account, PR #50), event {UUID}, at
{when}, extracted by `capture-2026-11-11-f1.py` from the session's own events (`list_events`), never retyped. It
answers the session's ask on the security review's F1 (F1-FIX, FREEZE-FDN, and his yes to the exact change in
`f1-gate.patch`).

```text
{text}
```
"""
(HERE / 'mustafa-2026-11-11-f1.md').write_text(out)
print('wrote mustafa-2026-11-11-f1.md;', len(text), 'characters; event at', when)
