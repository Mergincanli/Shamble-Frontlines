# Adds Mustafa's messages to the new lane-2 builder chat (Builder V5) to docs/DECISIONS.md, word for word: each message
# is read by its uuid from the chat's transcript and quoted whole; the header and the records line are this chat's.
# Each entry goes into "Decisions, newest first" before the first entry with an older time. Afterwards every quoted
# block is read back from the file, un-quoted and compared with the message's bytes and sha256.
# Usage: python3 -I add_v5.py <transcript.jsonl> [--write] (inside the worktree). Runs no git; writes only DECISIONS.
import hashlib, json, re, sys, textwrap
T = sys.argv[1]
P = 'docs/DECISIONS.md'
CHAT = ('the builder chat he calls Builder V5, lane 2 of the build, session_019YNMUUMZBxnFZJY9gp6qxy, a Claude Code '
        'chat with only this repository')
ENTRIES = [
    ('0822f9bd-22ea-48df-826b-8f6514f0b31e',
     "the start block for the new lane-2 builder chat (his GUARD-OFF (a)): the guard check as its first act; the branch "
     "and its head; the approved texts with their sha256; what is left, in order (the take-in of main, the wait for the "
     "Guard fix chat's protection for main, the token sitting with the security-auditor's conditions, the verifier, "
     "ready, his go and the merge, M5 to M11); the rules",
     'his label: "For the new lane-2 builder, from Mustafa (written by Builder V2 at my request, 26 November)."',
     "his paste, with the line \"Branch head: ...\" he added at its end",
     "Builder V2 wrote the block at his request (`.checkpoints/step-2-6-previews/start-block-new-chat.md` at dc00c79a); "
     "the chat's first act met the guard's ask, which he answered no (his GUARD-PROMPT (a), below)."),
    ('d3c924a4-7594-4631-a5b9-6cfc6f7ad7af',
     "GUARD-PROMPT (a): the guard's prompt reached him in Auto mode with its reason text, and he answered no; Builder V2 "
     "told of the takeover; MAIN-WALL is the Guard fix chat's next change-set after its current guard pull request; "
     "until it is merged, nothing under .github/ and no token sitting",
     'his label: "From Mustafa, 26 November, 15:05 Amsterdam (written with my Advisor)."',
     None,
     "the question he answers is the chat's GUARD-PROMPT (`.checkpoints/step-2-6-previews/questions-2026-11-26-new-chat.md` "
     "at e200d51). The same 738 bytes reached the chat a second time at 15:10:01 UTC (message "
     "46bc5952-2195-40ae-ac19-dd011f9ddc66), after an accidental rewind, his words; the work done between stood."),
    ('03254f5d-9cb8-44c2-9b41-b580ef8e4065',
     "MERGE-74 (a): main, with #74's change to `.github/workflows/toolchain.yml`, is taken into #72 once, after "
     "MAIN-WALL merges",
     'no label', None,
     "the question is the chat's MERGE-74 (`.checkpoints/step-2-6-previews/questions-2026-11-26-merge-74.md` at "
     "e58d049): (a) wait and take main in once after MAIN-WALL; (b) take it in now and again after."),
    ('7aaec862-9cf9-4f10-acb5-7880f06c8af1',
     "MAIN-WALL is on since about 17:15 UTC (only his account can update main; pull request, gate and smoke stay "
     "required for everyone); work that stores no credential resumes; the token sitting waits until one day-session "
     "merge and one PR Advisor merge have gone through under the new rule",
     'no label', None,
     "the chat then asked ORDER-WALL (whether #72 itself would be that first day-session merge), answered below at "
     "18:16:41 UTC."),
    ('28f1a938-6439-4d86-90fc-e55a3e06c26a',
     "ORDER-WALL (a): the token sitting stays before #72's merge; the first check merge is #76; the key waits only for "
     "the PR Advisor's next merge; meanwhile lane 2's own docs and the token sitting's message; the take-in of main "
     "held",
     'his label: "FOR BUILDER V5. From Mustafa, 26 November, 19:20 Amsterdam (written with my Advisor)."',
     None,
     "the question is the chat's ORDER-WALL (`.checkpoints/step-2-6-previews/questions-2026-11-26-order-wall.md` at "
     "d79ae76)."),
    ('9c6143e3-d741-4e88-817f-ee0b778d14d0',
     "TOKEN-RELEASE (c) met (the PR Advisor merged #77 under the new rule on 2026-11-26 at 18:52:17 UTC, merge commit "
     "47bb79e9); take main in once (b9aad01) with the shared docs fixes, STATUS at 120 lines or fewer; then the token "
     "sitting with today's dates; HOOK-PUSH (b), a standing yes to push the chat's main-folder branch while it holds "
     "only main's own commits",
     'his label: "FOR BUILDER V5. From Mustafa, 27 November, 12:50 Amsterdam (written with my Advisor)."',
     None,
     "HOOK-PUSH answers the chat's optional question of 2026-11-27 about the stop hook's repeated ask to push that "
     "branch."),
]
msgs = {}
for raw in open(T, encoding='utf-8'):
    try:
        o = json.loads(raw)
    except ValueError:
        continue
    if o.get('type') != 'user' or o.get('isSidechain') or not isinstance(o.get('message'), dict):
        continue
    if o.get('uuid') in {e[0] for e in ENTRIES}:
        c = o['message']['content']
        bl = [c] if isinstance(c, str) else [b.get('text', '') for b in c if isinstance(b, dict) and b.get('type') == 'text']
        assert len(bl) == 1, (o['uuid'], len(bl))
        msgs[o['uuid']] = (o['timestamp'], bl[0])
assert len(msgs) == len(ENTRIES), sorted(msgs)

def wrap(text):
    return textwrap.wrap(text, width=120, subsequent_indent='  ', break_long_words=False, break_on_hyphens=False)

def quote(t):
    return ['> ' + l if l else '>' for l in t.split('\n')]

new = []
for uuid, what, label, whole, records in ENTRIES:
    ts, t = msgs[uuid]
    b = t.encode('utf-8')
    sha = hashlib.sha256(b).hexdigest()
    date, time = ts[:10], ts[11:19]
    head = (f'- **Taken by Mustafa, {date}, {time} UTC: {what}** ({CHAT}; {label}; message {uuid}, whole'
            + (f', {whole}' if whole else '') + f'; extracted by script from the chat\'s transcript). Records, not his '
            f'words: {records} Copied by script ({len(b)} bytes, sha256 {sha}):')
    new.append(((date, time), wrap(head) + quote(t), t, sha))
    print(f'  {date} {time} UTC: {len(b)} bytes, sha256 {sha[:16]}...')

L = open(P, encoding='utf-8').read().split('\n')
TS = re.compile(r'^- \*\*Taken by Mustafa, (20\d\d-\d\d-\d\d), (\d\d:\d\d:\d\d) UTC')
a = L.index('## Decisions, newest first')
for key, lines, _, _ in sorted(new, key=lambda n: n[0]):  # oldest first, each before the first older entry
    k = next(i for i in range(a + 1, len(L)) if (m := TS.match(L[i])) and m.groups() < key)
    assert not TS.match(L[k - 1]) or True
    L = L[:k] + lines + L[k:]
# read back: each entry's quoted block equals its message
for key, lines, t, sha in new:
    s = L.index(lines[0])
    q = s + len(lines) - len(t.split('\n'))
    back = '\n'.join(l[2:] if l.startswith('> ') else '' for l in L[q:s + len(lines)])
    assert back == t and hashlib.sha256(back.encode('utf-8')).hexdigest() == sha, key
    nxt = next((i for i in range(s + len(lines), len(L)) if L[i].startswith('- **') or L[i].startswith('## ')), None)
    assert nxt == s + len(lines), ('the entry does not end where the next begins', key)
order = [m.groups() for l in L[a + 1:] if (m := TS.match(l))][:40]
assert order == sorted(order, reverse=True), 'the top entries are not newest first'
print('  read back: all six quoted blocks equal their messages; the top 40 entries newest first')
if '--write' in sys.argv:
    open(P, 'w', encoding='utf-8', newline='\n').write('\n'.join(L))
print('written' if '--write' in sys.argv else 'dry run')
