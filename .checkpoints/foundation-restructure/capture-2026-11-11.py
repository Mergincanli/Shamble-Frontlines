# Writes mustafa-2026-11-11.md: Mustafa's messages of 2026-11-11 in this session, extracted from the session's own
# events (list_events output saved by the tools), never retyped. Run: python3 -I capture-2026-11-11.py <events1> <events2>
import json, sys, pathlib

HERE = pathlib.Path(__file__).resolve().parent
WANT = {
    '299f29d1-2972-4258-b929-be121def97c4': 'opening',   # 10:01:35 UTC, the prompt re-sent after the switch to Opus 5.5
    '561b29da-e339-4540-9379-622cd7e9d199': 'first',     # 09:59:00 UTC, the same prompt under Fable 5.2 (stopped)
    '8794e858-ae9d-4ebf-938b-e78d2fe3a014': 'answer',    # 11:56:44 UTC, BRANCH-47 and the advisor-tool words
}
found = {}

def walk(o):
    if isinstance(o, dict):
        u = o.get('user')
        if isinstance(u, dict) and u.get('uuid') in WANT:
            msg = (u.get('internal_anthropic_catchall') or {}).get('message') or {}
            c = msg.get('content')
            assert isinstance(c, str), f"{u['uuid']}: content is not a plain message"
            found[WANT[u['uuid']]] = (o.get('created_at'), c)
        for v in o.values():
            walk(v)
    elif isinstance(o, list):
        for v in o:
            walk(v)

for p in sys.argv[1:]:
    walk(json.loads(pathlib.Path(p).read_text()))
missing = [k for k in ('opening', 'answer') if k not in found]
assert not missing, f'events not found: {missing}'
for k, (_, t) in found.items():
    assert '```' not in t, k
if 'first' in found:
    assert found['first'][1] == found['opening'][1], 'the two sends of the opening prompt differ'

card = (HERE / 'resume-card.md').read_text().split('\n\n', 1)[1].rstrip('\n')
opening = found['opening'][1]
cut = opening.index('\n\nTwo additions, my words of 2026-11-11:')
same_card = opening[:cut].replace('\n\n1.', '\n1.') == card

out = f"""# Mustafa's messages of 2026-11-11 in the replacement session, word for word

Source: his chat messages in session_019YNMUWLffMoE47SbvkqJ5P (his personal account, branch
`claude/pensive-heisenberg-aazidf`, PR #50 replacing #47), extracted by `capture-2026-11-11.py` from the session's
own events (`list_events`), never retyped. Times are UTC.

## 1. His prompt ({found['opening'][0]}; first sent at {found.get('first', ('not found',))[0]}, the same text, under Fable 5.2)

The prompt is the resume card (`resume-card.md`) as pasted{' (its text matches the card, apart from one blank line before the list)' if same_card else ' (its text differs from the card: compare them)'},
plus his two additions at the end, "my words of 2026-11-11".

```text
{opening}
```

## 2. His answer to BRANCH-47 and his words on the advisor tool ({found['answer'][0]})

Before it, at 10:51 UTC, he ran `/advisor fable` (Claude Code's reply: "Advisor set to Fable 5.2").

```text
{found['answer'][1]}
```
"""
(HERE / 'mustafa-2026-11-11.md').write_text(out)
print('wrote mustafa-2026-11-11.md;', 'card matches' if same_card else 'CARD DIFFERS', '; events:',
      {k: v[0] for k, v in found.items()})
