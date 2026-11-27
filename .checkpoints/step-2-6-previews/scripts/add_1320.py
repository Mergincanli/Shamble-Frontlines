# Adds Mustafa's message of 2026-11-26 (13:20 Amsterdam label) to docs/DECISIONS.md, whole, copied from the chat's
# transcript and checked byte for byte. No git, no protected file (his GIT-IN-NODE (a)). Usage: python3 add_1320.py --write
import json, hashlib, sys, pathlib
T = '/root/.claude/projects/-home-user/a7e7f150-3195-52bd-a549-73b27426198b.jsonl'
D = pathlib.Path('/home/user/Shamble-Frontlines/docs/DECISIONS.md')
START = 'For Builder V2, from Mustafa (written with the main Advisor, 26 November, 13:20 Amsterdam).'
found = []
for line in open(T, encoding='utf-8'):
    try: e = json.loads(line)
    except Exception: continue
    if e.get('type') != 'user' or e.get('isSidechain'): continue
    c = (e.get('message') or {}).get('content')
    texts = [c] if isinstance(c, str) else [b.get('text', '') for b in (c or []) if isinstance(b, dict) and b.get('type') == 'text']
    for t in texts:
        if t.startswith(START): found.append((e.get('timestamp'), e.get('uuid'), t))
assert len({f[1] for f in found}) == 1, f'expected one message, found {len(found)}'
ts, uuid, text = found[0]
raw = text.encode('utf-8'); sha = hashlib.sha256(raw).hexdigest()
hms = ts[11:19]
head = (f"- **Taken by Mustafa, 2026-11-26, {hms} UTC: his answers on step 2.6 and the guard: GUARD-OFF (a), a new builder\n"
        "  chat with only this repository takes over the sensitive steps once this chat has finished the report and the docs\n"
        "  pass, its start block written by this chat, on Opus 5.5; MAIN-WALL-AGAIN (a); GUARD-KIT (a), the Guard fix chat\n"
        "  after its protection for main; GIT-IN-NODE (a), for this chat and the new one; TAG-CHECK (a), recorded as untested\n"
        "  with the reason; until the new chat has taken over, no change under .github/ or the kit, no main-folder move, no\n"
        "  merge** (the builder chat he calls Builder V2, lane 2 of the build, session_019YNMUSYpJGi1xxCjKgr4e5, a Claude Code\n"
        "  chat on his personal account; his label: \"For Builder V2, from Mustafa (written with the main Advisor, 26 November,\n"
        f"  13:20 Amsterdam).\"; message {uuid}, whole; extracted by script from the chat's transcript). Records, not\n"
        "  his words: the questions he answers are in `.checkpoints/step-2-6-previews/questions-2026-11-26-guard.md` at\n"
        "  702f42c0 (the first four) and the report `docs/reports/2026-11-26-step-2-6-previews.md` at 8c8ad0d0 (TAG-CHECK).\n"
        f"  Copied by script ({len(raw)} bytes, sha256 {sha}):\n")
quoted = ''.join(('> ' + l if l else '>') + '\n' for l in text.split('\n'))
entry = head + quoted
back = '\n'.join(l[2:] if l.startswith('> ') else '' for l in quoted.rstrip('\n').split('\n'))
assert back == text, 'the quoted lines do not give back the message byte for byte'
s = D.read_text(encoding='utf-8'); anchor = '## Decisions, newest first\n'
assert s.count(anchor) == 1
new = s.replace(anchor, anchor + entry, 1)
print(f'{ts} {uuid} {len(raw)} bytes sha256 {sha}; entry {entry.count(chr(10))} lines')
if '--write' in sys.argv: D.write_text(new, encoding='utf-8'); print('written')
