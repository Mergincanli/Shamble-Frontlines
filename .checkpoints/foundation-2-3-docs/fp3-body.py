# Fix pass 3 (the verifier's F-V1 and F-V3; plan revision 2e): the report body's edits in report-body.md, each an exact
# replacement found once; edited bullets re-wrapped to 120 columns. Run once: python3 -I fp3-body.py (notes folder)
p = 'report-body.md'
s = open(p, encoding='utf-8').read()
long_before = {l for l in s.split('\n') if len(l) > 120}
def rep(old, new):
    global s
    n = s.count(old)
    if n != 1: raise SystemExit(f'found {n} times: {old[:100]!r}')
    s = s.replace(old, new)
def rewrap(prefix, first='- ', rest='  '):
    global s
    lines = s.split('\n')
    hits = [i for i, l in enumerate(lines) if l.startswith(prefix)]
    if len(hits) != 1: raise SystemExit(f'rewrap: {len(hits)} lines start with {prefix!r}')
    a = hits[0]; b = a + 1
    while b < len(lines) and lines[b].startswith(rest) and not lines[b].startswith(rest + '- ') and lines[b].strip(): b += 1
    words = ' '.join(l.strip() for l in lines[a:b])[len(first):].split(' ')
    out, line = [], first
    for w in words:
        cand = line + w if line in (first, rest) else f'{line} {w}'
        if len(cand) > 120 and line not in (first, rest): out.append(line); line = rest + w
        else: line = cand
    out.append(line)
    lines[a:b] = out
    s = '\n'.join(lines)

def edit_bullet(prefix, old, new, first='- ', rest='  '):
    # Join the bullet's lines, replace old by new on the joined text (found once), then re-wrap the bullet.
    global s
    lines = s.split('\n')
    hits = [i for i, l in enumerate(lines) if l.startswith(prefix)]
    if len(hits) != 1: raise SystemExit(f'edit_bullet: {len(hits)} lines start with {prefix!r}')
    a = hits[0]; b = a + 1
    while b < len(lines) and lines[b].startswith(rest) and not lines[b].startswith(rest + '- ') and lines[b].strip(): b += 1
    joined = ' '.join(l.strip() for l in lines[a:b])
    if joined.count(old) != 1: raise SystemExit(f'edit_bullet: {joined.count(old)} hits of {old[:80]!r}')
    text = joined.replace(old, new)
    words = text[len(first):].split(' ')
    out, line = [], first
    for w in words:
        cand = line + w if line in (first, rest) else f'{line} {w}'
        if len(cand) > 120 and line not in (first, rest): out.append(line); line = rest + w
        else: line = cand
    out.append(line)
    lines[a:b] = out
    s = '\n'.join(lines)

# Outcome (E4): eight commits; the verifier's FAIL.
edit_bullet('- Built in seven commits', "Built in seven commits (1123175, 2f96f5c, 83c6ab6, d59ca43, 554face for review round 1's fix pass, 8e4fe7c for round 2's, then its follow-up), every script check and the gate passing; CI green on each through 8e4fe7c (E7).",
    "Built in eight commits (1123175, 2f96f5c, 83c6ab6, d59ca43, 554face for review round 1's fix pass, 8e4fe7c for round 2's, a1d0317 its follow-up, then the verifier's fix pass), every script check and the gate passing; CI green on each through a1d0317 (E7).")
edit_bullet('- Built in eight commits', "all fixed or disclosed. The verifier and the advisor agent's review of the final head follow.",
    "all fixed or disclosed. The verifier failed a1d0317 on one High (how Mustafa was told about the advisor model, written from recall), fixed in the verifier's fix pass; its re-run and the advisor agent's review of the final head follow.")

# Reviews (E3, E4).
def edit_para(prefix, old, new):
    # A plain paragraph (no bullet): join its lines up to the next blank line, replace once, re-wrap to 120 columns.
    global s
    lines = s.split('\n')
    hits = [i for i, l in enumerate(lines) if l.startswith(prefix)]
    if len(hits) != 1: raise SystemExit(f'edit_para: {len(hits)} lines start with {prefix!r}')
    a = hits[0]; b = a + 1
    while b < len(lines) and lines[b].strip(): b += 1
    joined = ' '.join(l.strip() for l in lines[a:b])
    if joined.count(old) != 1: raise SystemExit(f'edit_para: {joined.count(old)} hits of {old[:80]!r}')
    out, line = [], ''
    for w in joined.replace(old, new).split(' '):
        cand = w if not line else f'{line} {w}'
        if len(cand) > 120 and line: out.append(line); line = w
        else: line = cand
    out.append(line)
    lines[a:b] = out
    s = '\n'.join(lines)
edit_para('Round 1, on d59ca43', 'each a general agent on Opus 5.5 at effort max;', 'each a general agent on Opus 5.5, asked for effort max;')
edit_para('Round 2, on what changed from d59ca43 to 554face', 'each a general agent on Opus 5.5 at effort max;', 'each a general agent on Opus 5.5, asked for effort max;')
rep("""The verifier and the advisor agent's review of the final head: to come.
""", """The verifier (a general agent on Opus 5.5, asked for effort max, reading main's verifier.md), on a1d0317: FAIL, with
one High and one Low; everything else it checked held (the gate; the eight scripts; every SHA, time and run id; the
records of the consultations, the agents' models and the refusals; Appendix A and DECISIONS.md). Its scope note:
everything after 554face, the new DECISIONS entry included, had the verifier and the advisor agent but no reviewer.

| Id | Severity | Finding | Disposition |
|---|---|---|---|
| F-V1 | High | "Decisions and deviations" told how Mustafa was told about the advisor model from recall: it claimed two updates no record holds, left out the recorded one of 22:59:04 and dropped the "If true" of 22:48:47 | fixed: the recorded messages, each from its own text (the verifier's fix pass) |
| F-V3 | Low | the copies of his words: "round 2's 14 notes files" held five of round 1's; the list was incomplete | fixed: the certain copies named, with the check and its threshold |

Its re-run on the verifier's fix pass and the advisor agent's review of the final head: to come.
""")

# The advisor tool (E5): the "done" saves; entries 9 and 11; entry 12.
rep("pass 22:19:55 (461c82b, after number 8), review round 2 22:52:48 (ef1a7c1, after number 9).",
    "pass 22:19:55 (461c82b, after number 8), review round 2 22:52:48 (ef1a7c1, after number 9), round 2's fix pass\n23:49:35 (7249a13, after number 11).")
rep("""   advisor agent; Mustafa told in chat; the main session's and the agents' models checked in the records.""",
"""   advisor agent; the correction for Mustafa in chat (on record at 23:51:05, transcript line 2869); the main session's
   and the agents' models checked in the records.""")
rep("""   clause on how Mustafa was told, worded from the records; E2's check run on this tree; the fragment check widened;""",
"""   clause on how Mustafa was told reworded, still from recall: the verifier failed it, F-V1; E2's check run on this
   tree; the fragment check widened;""")
rep("""   CI tools, no words of his in its notes, 2.2's script copy, nothing posted on GitHub).
""", """   CI tools, no words of his in its notes, 2.2's script copy, nothing posted on GitHub).
12. 2026-11-14 00:34:39, moment 2 for the clause on how Mustafa was told (its wording failed a second time), moment 1
   for the verifier's fix pass and moment 4 for the form of F-V3's fix, before the advisor agent was asked: the clause
   from the recorded messages only, each read in full and described from its own text; nothing about messages the
   records do not hold; F-V3 narrowed (no counts; the certain copies named); "asked for effort max"; the verifier's FAIL
   recorded as it happened; the same verifier resumed with a narrow brief.
""")

# Decisions and deviations (E1, E2, E5).
rep("""  call's usage record). No chat message named the advisor model before round 2. At 22:48:47 UTC the session relayed the
  finding to Mustafa and wrongly added that it had told him the wrong thing at the start; its chat updates after the
  compaction first repeated that and then corrected it, but neither update is in the transcript or the session's event
  record, so the end message states the correction again.
""", """  call's usage record). No chat message named the advisor model before round 2. Four chat messages since bear on it (the
  transcript's lines 1887, 2122, 2869 and 2929): at 22:48:47 UTC the session relayed the finding and added "If true, I
  told you the wrong thing at the start."; at 22:59:04 it wrote that it was checking "where I first told you the advisor
  model"; at 23:51:05 it wrote that Fable 5.2 had answered all 11 advisor-tool consultations, that no chat message of its
  own had named the advisor model before 22:48 UTC, that its 22:48 message and its first update after the context reset
  had wrongly said it had told him Opus 5.5 at the start, and that the wrong claim had been only in the report and the
  checkpoint; at 00:31:35 on 2026-11-14 it wrote that the verifier had failed this bullet's earlier wording (F-V1), that
  its 22:59 update is on record, and that two correcting updates it believed it had sent are not.
""")
rewrap('- The advisor model (RW2-F1')
rep("""  writing to a variable path; route: literal paths).
- Retyped copies of his words""", """  writing to a variable path; route: literal paths); the verifier, 2026-11-14 00:12:07 (a compound command with an
  `rm -rf`, denied by the permission system; route: the Write tool, each step on its own, plain `rm` and `rmdir` of named
  files) and 00:16:46 (the guard: a loop writing to a variable path; route: literal paths).
- Retyped copies of his words""")
rewrap('- Commands refused by the guard')
i = s.index('- Retyped copies of his words'); j = s.index('- Two of the seven L2 logs')
s = s[:i] + """- Copies of his words in the notes folder, each pushed by the saver and kept by history, none of them in docs/. Certain copies
  (whole sentences or longer): the plan (his decision, his kit words and A1, which its pointer texts and checks used);
  this report's draft body (its quotes) and `fp2-body.py`; the checkpoint; the advisor agent's notes (A4 whole, and runs
  of A2 and A3); docs-keeper's `fillcount.mjs` (e6fc6a9; his decision's four sentences and his kit words,
  byte-identical to the script cuts), `dk-para.txt` and `dk-para-wrapped.txt`; the scripts that cut or check his words
  and the review briefs (`decisions.mjs`, `pointers.mjs`, `fixpass1.mjs`, `review-r1.js` and `review-r2.js` with their
  launched copies: anchors, negative controls, and his kit words for the reviewers); and R-WORDS's notes at ec8f1bd.
  Fragments, among others: R-STATE's notes and `rs-roadmap.mjs` (RS-SELF), R2-WORDS's notes at 461c82b and 7b5f53c
  (RW2-F5). `fp3-fragments-all.mjs` checks every version of every notes file for runs of 30 characters or more of his
  words; shorter shared runs, among them the base SHA, file paths and names his message also holds, exist in other
  files and are not listed. His words are public in DECISIONS.md already.
""" + s[j:]
rewrap('- Copies of his words in the notes folder')

long = [l for l in s.split('\n') if len(l) > 120 and not l.startswith('|') and l not in long_before]
if long: raise SystemExit(f'{len(long)} lines over 120 characters: {long[0][:90]!r}')
open(p, 'w', encoding='utf-8').write(s)
print('ok')
