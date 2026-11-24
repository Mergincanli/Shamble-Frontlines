# Builds DECISIONS.md entries for Mustafa's messages to this chat (Builder V2, lane 2) and inserts each, by the time in
# its header, into docs/DECISIONS.md's "Decisions, newest first", touching no other line (RECORD-FILES). His words come
# from messages.json (extract.py, byte for byte from the transcript); headers are written here. Run from the repo root.
import json, re, sys, hashlib
M = json.load(open('.checkpoints/preview-hosting-check/decisions/messages.json', encoding='utf-8'))
# notes: a queued message, taken from its saved file between the markers (entry 4c6ca7ef..., 914 bytes)
a = open('.checkpoints/preview-hosting-check/mustafa-2026-11-15-notes.md', encoding='utf-8').read()
nt = a[a.index('-----BEGIN HIS TEXT-----\n') + 25:a.index('\n-----END HIS TEXT-----')]
nb = nt.encode('utf-8')
assert len(nb) == 914 and hashlib.sha256(nb).hexdigest().startswith('7742caa2d586'), (len(nb), hashlib.sha256(nb).hexdigest())
M['notes'] = {'uuid': '4c6ca7ef-e16e-4efa-9288-30f5672764a9', 'timestamp': '2026-11-15T16:03:05.372Z', 'bytes': 914,
              'sha256': hashlib.sha256(nb).hexdigest(), 'text': nt}
CHAT = 'the builder chat he calls Builder V2, lane 2 of the build, session_019YNMUSYpJGi1xxCjKgr4e5, a Claude Code chat on his personal account'
E = {  # key: (header, source detail, what it answers / records, in the chat's words)
 'day2': ("the preview hosting check's Day 2 sitting now; the token ends 2026-11-25T22:59:59Z, so Day 2 finishes before then; the chat reads its checkpoint, takes the fresh readings its own rules ask for before he changes anything in Cloudflare, tells him the sitting's length first, then gives the list on one screen and goes one step at a time; plain words with him",
          'his label: "For Builder V2 (the preview hosting check), from me, written with my main Advisor (24 November)"',
          "What it answers, in the chat's words (not his): nothing the chat asked; Day 2 ran that evening (the report of the preview hosting check)."),
 'ads': ("one thing for lane 2, no work now: aiming down sights and the sprint-to-fire delay are game rules of Slice 1, which lane 1 prices and places in change-set 4's plan; lane 2's wishes W1 and W4 on issue #59 are needs now, and its comment there stays as it is; AIM-STATE of 2026-11-19, 00:59 stands for the poses",
         'his label: "My words of 2026-11-19, 12:00 (Amsterdam time)"; the message opens with the line that resumed this chat after a compaction',
         "Records, not his words: the decision itself (SLICE-FEEL) is in the entry of 2026-11-19, 13:36:37 UTC, from his message to the build chat; this entry is his message to lane 2."),
 'answers': ("PROJ-DRAW (a): lane 1 draws grenades and arrows in flight plainly in change-set 9, and they pass to lane 2 at its merge; AIM-STATE (a) as far as the block goes: lane 2 asks for no aiming flag and plans Slice 1's poses from his Testlines sandbox's rule, the idle pose brought as a proposal with pictures; the needs list on issue #59 stands as posted; on the preview hosting check, with no pass mark or reviewer's condition dropped or loosened: sitting 3 goes on at his \"unticked\", a setting is read from GitHub's public API where it can be, and step 2.6's plan designs the previews without a click from him and shows what keeps the token safe then",
             'his label: "My words of 2026-11-19, 00:59 (Amsterdam time)"',
             "What it answers, in the chat's words (not his): PROJ-DRAW (who draws grenades and arrows in flight in Slice 1) and AIM-STATE (whether lane 2 asks for an aiming flag), asked with the needs list of 2026-11-18."),
 'lane2': ("the second lane: this chat is lane 2 of the build; the preview hosting check keeps the first place; main merged into this chat's branch; Part 15 and his record-file answers read; lane 2's first list of needs posted on issue #59; the order: the check to its report and closing pull request, then step 2.6's plan, and no game code before change-set 5 is merged; no model file (GLB) made by a builder chat; main taken in at the start of each unit and before a pull request opens",
           'his label: "My words of 2026-11-18, 23:37 (Amsterdam time)"',
           "What it answers, in the chat's words (not his): nothing the chat asked."),
 'browsers': ("the browsers for the preview hosting check: Brave, Chrome, Firefox and maybe Edge, so Safari leaves the check; sitting 2 starts now",
              "his message as typed",
              "What it answers, in the chat's words (not his): it followed the chat's report that the Safari row read 1 load of 15 (a script fault); the row PM-0006-3 is then read on Chrome, Edge and Firefox by script and in Brave by him."),
 'reviewer-b': ("REVIEWER (b): for the rest of the preview hosting check he unticks \"Required reviewers\" on the test repository's preview environment, except for sitting 3, before which he ticks it again; it stays on for Day 2",
                "his message as typed",
                "What it answers, in the chat's words (not his): REVIEWER-NOW, after his question of 17:50:30 UTC \"Why do I have to approve these things, can't we just automate it?\"; option (b): \"You untick \"Required reviewers\" on preview for everything but sitting 3\"."),
 'token-scope': ("TOKEN-SCOPE (a): the test token keeps Cloudflare Pages Read for the rest of the check; Day 1 of the check today if possible; he compacts this chat afterwards",
                 "his message as typed",
                 "What it answers, in the chat's words (not his): 1, TOKEN-SCOPE ((a) Read stays; (b) Read removed after a measured run); 2, when Day 1 runs; 3, a request to compact the chat."),
 'token-window': ("TOKEN-WINDOW done by him: the test token's expiration set to the \"7 days\" preset; the token check re-run (CHECK-RERUN) and run a second time; the account ID's first and last four characters compared (ID-ENDS): they match",
                  "his message as typed",
                  "What it answers, in the chat's words (not his): the fix for the token's one-day window found by the token check (TOKEN-WINDOW), the check's re-run, and ID-ENDS."),
 'check-yes': ("CHECK-YES: yes, for one run of the token check (check-token.yml, sha256 0ad66aa0f2b36f7469bc3bdd57a4a05b7945e0037f96b19059000e0ca9b5aa71) and the acts its request lists; ID-ENDS: yes; no deploy run of any kind before his screenshot of the renamed Cloudflare account",
               'his label: "My words of 2026-11-18, 11:31 (Amsterdam time)"',
               "What it answers, in the chat's words (not his): the chat's two yes requests of 2026-11-17 on the token check."),
 'rename': ("yes to renaming his Cloudflare account, with step-by-step instructions from the chat",
            "his message as typed",
            "What it answers, in the chat's words (not his): the chat's point that a failed upload could print the account's default name (his e-mail address) in a public log; renaming the account removes the e-mail from that name."),
 'yes': ("yes to the batched diff of the check's workflows (deploy.yml 29593af18d34 and close.yml 3afdf389d636: \"deployments: write\" removed, the records kept 14 days); SA-R3 accepted for this check; for step 2.6, a locked install planned and brought with step 2.6's plan",
         'his label: "My words of 2026-11-16, 09:20 (Amsterdam time)"',
         "What it answers, in the chat's words (not his): the chat's batched diff after the review round, and the security-auditor's finding SA-R3 (the install step keeps the token out of the scripts' environment, not out of their reach)."),
 'go': ("his answers to the seven decisions of the preview hosting check's plan (TR-WHERE, TOKENS, REVIEWER, MACHINES, TOMBSTONE, ACCESS-CARD and TR-END, each (a)), his go on the plan (revision 2, commit 2a2f426), his yes to Part 7's six workflow texts by their sha256, what the chat may do in the test repository, and three things of his (SR-READ decides before the build; the security-auditor rules on \"deployments: write\"; one list before every sitting)",
        'his label: "My words for the second builder chat, written 2026-11-15, 18:35 (Amsterdam time)"',
        "What it answers, in the chat's words (not his): the plan's eight decision entries and its \"For Mustafa\" asks."),
 'notes': ("three notes on his task message: every \"today\" in it means 2026-11-14; the first builder chat's go comes on Tuesday 2026-11-17, so its change-set 0 and Part 15 do not exist yet; a third chat builds Shamble VFX in its own repository, not in this lane; the task does not change",
           'a queued message, entry 4c6ca7ef-e16e-4efa-9288-30f5672764a9 (taken from the chat\'s byte-for-byte copy, checked against its recorded length and sha256); his label: "My words of 2026-11-15, 17:00 (Amsterdam time)"',
           "What it answers, in the chat's words (not his): nothing the chat asked."),
 'task': ("the second builder chat's task: his lane in order (the preview hosting check now, then foundation step 2.6, the carry of his look decisions into docs/LOOK_V2.md, then the units Part 15 gives); PREVIEW-START: \"it starts now, on the infra draft as proposed\"; the main model Opus 5.5 with Fable 5.2 on the advisor tool; the plan for the check, with the eight things it must give him",
          'his label: "My words of 2026-11-14, for a fresh chat on my personal account: the second builder chat for the game"; written 2026-11-14, pasted 2026-11-15 (his note of 16:03:05 UTC below)',
          "Records, not his words: the decision of 2026-11-14 on two builder chats that this message quotes is recorded by the first builder chat's change-set 0, as the message says."),
}
def wrap(prefix, text, width=120, indent='  '):
    words, lines, cur = text.split(' '), [], prefix
    for w in words:
        if len(cur) + 1 + len(w) > width and cur.strip():
            lines.append(cur); cur = indent + w
        else:
            cur = (cur + ' ' + w) if cur not in (prefix,) or prefix.endswith(' ') is False else cur + w
    lines.append(cur); return lines
def entry(key):
    r = M[key]; ts = r['timestamp']; d, t = ts[:10], ts[11:19]
    head, src, ans = E[key]
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
    return ts, '\n'.join(out) + '\n'
P = 'docs/DECISIONS.md'
doc = open(P, encoding='utf-8').read()
start = doc.index('## Decisions, newest first\n') + len('## Decisions, newest first\n')
end = doc.index('\n## Also recorded elsewhere')
sec = doc[start:end]
lines = sec.split('\n')
TS = re.compile(r'(20\d\d-\d\d-\d\d),? (\d\d:\d\d(?::\d\d)?)')
ANCHOR = re.compile(r'^- (\*\*(Replaced.*?)?Taken by Mustafa, |_A pointer, not a decision: \*\*)')
def entry_ts(i):
    blob = ' '.join(lines[i:i+3])
    m = TS.search(blob)
    if not m:  # an older header with a date and no time (all before 2026-11-12): the day's start is enough to order by
        d = re.search(r'20\d\d-\d\d-\d\d', blob)
        return (d.group(0) + 'T00:00:00') if d else None
    t = m.group(2)
    if len(t) == 5: t += ':00'
    return m.group(1) + 'T' + t
# Only an entry's own header line (a decision or a pointer) can start a block; the session's texts and the left-out
# notes under a decision belong to it, so nothing is ever put between a decision and its sub-entries.
tops = [i for i, l in enumerate(lines) if ANCHOR.match(l)]
assert all(entry_ts(i) for i in tops), [lines[i][:80] for i in tops if not entry_ts(i)]
new = sorted((entry(k) for k in E), key=lambda x: x[0], reverse=True)
inserts = {}
for ts, text in new:
    key_ts = ts[:19]
    pos = None
    for i in tops:
        et = entry_ts(i)
        if et is not None and et < key_ts:
            pos = i; break
    if pos is None: pos = len(lines)
    inserts.setdefault(pos, []).append(text.rstrip('\n'))
for pos in sorted(inserts, reverse=True):
    block = '\n'.join(inserts[pos]).split('\n')
    lines[pos:pos] = block
newsec = '\n'.join(lines)
doc2 = doc[:start] + newsec + doc[end:]
if '--write' in sys.argv:
    open(P, 'w', encoding='utf-8').write(doc2)
# checks: every quoted block equals his bytes; no old line changed (old lines are a subsequence of new)
for k in E:
    r = M[k]; quoted = '\n'.join(('> ' + l if l else '>') for l in r['text'].split('\n'))
    assert quoted in doc2, k
old_lines = doc.split('\n'); new_lines = doc2.split('\n'); j = 0
for l in new_lines:
    if j < len(old_lines) and l == old_lines[j]: j += 1
assert j == len(old_lines), ('old lines not kept in order', j, len(old_lines))
print('entries:', len(E), 'insert points:', {p: len(v) for p, v in inserts.items()}, 'lines added:', len(new_lines) - len(old_lines), 'written' if '--write' in sys.argv else 'dry run')
