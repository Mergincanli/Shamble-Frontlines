# Rewrites two of this chat's own DECISIONS entries (consultation 28): their headers stated the chat's reading as his
# decision. The reading moves into the "in the chat's words" sentence; his quoted lines are untouched (asserted).
import json, re, importlib.util, sys
spec = importlib.util.spec_from_file_location('b', '.checkpoints/preview-hosting-check/decisions/build_entry.py')
P = 'docs/DECISIONS.md'
doc = open(P, encoding='utf-8').read()
M = json.load(open('.checkpoints/preview-hosting-check/decisions/messages.json', encoding='utf-8'))
CHAT = 'the builder chat he calls Builder V2, lane 2 of the build, session_019YNMUSYpJGi1xxCjKgr4e5, a Claude Code chat on his personal account'
def entry(key, head, src, ans):
    r = M[key]; ts = r['timestamp']; d, t = ts[:10], ts[11:19]
    body = (f"**Taken by Mustafa, {d}, {t} UTC: {head}** ({CHAT}; {src}; message {r['uuid']}, whole; extracted by script "
            f"from the chat's transcript). {ans} Copied by script ({r['bytes']} bytes, sha256 {r['sha256']}):")
    out = []; cur = '- '
    for w in body.split(' '):
        if len(cur) + len(w) + (0 if cur in ('- ', '  ') else 1) > 120:
            out.append(cur); cur = '  ' + w
        else:
            cur = cur + w if cur in ('- ', '  ') else cur + ' ' + w
    out.append(cur)
    for line in r['text'].split('\n'):
        out.append('> ' + line if line else '>')
    return '\n'.join(out)
OLD = {
 'browsers': ("the browsers for the preview hosting check: Brave, Chrome, Firefox and maybe Edge, so Safari leaves the check; sitting 2 starts now",
              "his message as typed",
              "What it answers, in the chat's words (not his): it followed the chat's report that the Safari row read 1 load of 15 (a script fault); the row PM-0006-3 is then read on Chrome, Edge and Firefox by script and in Brave by him."),
 'reviewer-b': ("REVIEWER (b): for the rest of the preview hosting check he unticks \"Required reviewers\" on the test repository's preview environment, except for sitting 3, before which he ticks it again; it stays on for Day 2",
                "his message as typed",
                "What it answers, in the chat's words (not his): REVIEWER-NOW, after his question of 17:50:30 UTC \"Why do I have to approve these things, can't we just automate it?\"; option (b): \"You untick \"Required reviewers\" on preview for everything but sitting 3\"."),
}
NEW = {
 'browsers': ("the browsers for the preview hosting check: in his words, \"focus on brave, chrome, firefox and maybe edge (most used browsers)\"; sitting 2 starts now",
              "his message as typed",
              "What it answers, in the chat's words (not his): it followed the chat's report that the Safari row read 1 load of 15 (a script fault). The chat's reading of it, not his words: Safari leaves this check, so the row PM-0006-3 is read on Chrome, Edge and Firefox by script and in Brave by him; his BROWSER-SET of 2026-11-19 (above) keeps Safari a target of v2."),
 'reviewer-b': ("REVIEWER (b): \"Required reviewers\" on the test repository's preview environment is unticked for the rest of the preview hosting check except sitting 3, as option (b) reads",
                "his message as typed",
                "What it answers, in the chat's words (not his): REVIEWER-NOW, after his question of 17:50:30 UTC \"Why do I have to approve these things, can't we just automate it?\"; option (b): \"You untick \"Required reviewers\" on preview for everything but sitting 3\". The chat's plan, not his words: ticked again before sitting 3 and kept on for Day 2."),
}
for k in OLD:
    o = entry(k, *OLD[k]); n = entry(k, *NEW[k])
    assert doc.count(o) == 1, k
    doc = doc.replace(o, n)
    quoted = '\n'.join(('> ' + l if l else '>') for l in M[k]['text'].split('\n'))
    assert quoted in doc, k
if '--write' in sys.argv:
    open(P, 'w', encoding='utf-8').write(doc)
print('two entries rewritten' + (' (written)' if '--write' in sys.argv else ' (dry run)'))
