# Round 2's fix pass, its follow-up (consultation 11, 2026-11-13 23:39:10 UTC): the report body's edits in
# report-body.md, each an exact replacement found once; lines written stay within 120 characters.
# Run once: python3 -I fp2b-body.py (from the notes folder)
p = 'report-body.md'
s = open(p, encoding='utf-8').read()
long_before = {l for l in s.split('\n') if len(l) > 120}
def rep(old, new):
    global s
    n = s.count(old)
    if n != 1: raise SystemExit(f'found {n} times: {old[:100]!r}')
    s = s.replace(old, new)

# Outcome: the commits.
rep("""- Built in six commits (1123175, 2f96f5c, 83c6ab6, d59ca43, 554face for review round 1's fix pass, then round 2's fix
  pass), every script check and the gate passing; CI green on each through 554face (E7). Review round 1 (two reviewers,
""", """- Built in seven commits (1123175, 2f96f5c, 83c6ab6, d59ca43, 554face for review round 1's fix pass, 8e4fe7c for round
  2's, then its follow-up), every script check and the gate passing; CI green on each through 8e4fe7c (E7). Review
  round 1 (two reviewers,
""")
# E2: 2.2's check run on this tree.
rep("2.2's `decisions-entries.mjs --check`: OK, 13 entries |", "2.2's `decisions-entries.mjs --check` on this tree: OK, 13 entries |")
# E7: 8e4fe7c's runs.
rep("""554face, round 1's fix pass (43517490537, 43517496130; 43517490505, 43517496110); round 2's fix pass: listed in the final commit;""",
"""554face, round 1's fix pass (43517490537, 43517496130; 43517490505, 43517496110); 8e4fe7c, round 2's fix pass (43528414752, 43528420177; 43528414776, 43528420127); its follow-up: listed in the final commit;""")
# The advisor tool: entry 11.
rep("""   scan-gated write; the intro's list checked against the scan; midnight.
""", """   scan-gated write; the intro's list checked against the scan; midnight.
11. 23:39:10, moment 5 for round 2's fix pass, after its commit 8e4fe7c and before any "done" save: not done yet (the
   clause on how Mustafa was told, worded from the records; E2's check run on this tree; the fragment check widened;
   what is already known added now), and moment 1 for the verifier (its brief: the head, what is pending by design, the
   CI tools, no words of his in its notes, 2.2's script copy, nothing posted on GitHub).
""")
# Decisions and deviations: how Mustafa was told, from the records.
rep("""  call's usage record). No chat message named the advisor model before round 2; Mustafa was told in chat after it, at
  first with the claim wrongly placed in an opening message of the session's, which its next message corrected.
""", """  call's usage record). No chat message named the advisor model before round 2. At 22:48:47 UTC the session relayed the
  finding to Mustafa and wrongly added that it had told him the wrong thing at the start; its chat updates after the
  compaction first repeated that and then corrected it, but neither update is in the transcript or the session's event
  record, so the end message states the correction again.
""")
# The refusals: the harness's sleep block.
rep("""  23:33:46 UTC (a `git -C` command; route: git run from the folder). The agents', all by the guard, each with the route
""", """  23:33:46 UTC (a `git -C` command; route: git run from the folder); besides, the harness's own Bash rule stopped a
  foreground `sleep` at 23:38:22 (route: CI read through the API instead). The agents', all by the guard, each with the route
""")
# The scratch copies: the widened check.
rep("""  (fragments); two versions of R2-WORDS's notes (461c82b and 7b5f53c, fragments; its notes now hold pointers, and no
  pushed version of round 2's other notes holds a run of 30 characters of his words: `fp2-fragments-history.mjs`). His
  words are public in DECISIONS.md already.
""", """  (fragments); two versions of R2-WORDS's notes (461c82b and 7b5f53c, fragments; its notes now hold pointers). No other
  pushed version of round 2's 14 notes files holds a run of 30 characters of his words (`fp2-fragments-history.mjs`).
  His words are public in DECISIONS.md already.
""")
def rewrap(prefix):
    # Re-wrap one bullet (its first line starts with prefix; its other lines are indented two spaces) to 120 columns.
    global s
    lines = s.split('\n')
    hits = [i for i, l in enumerate(lines) if l.startswith(prefix)]
    if len(hits) != 1: raise SystemExit(f'rewrap: {len(hits)} lines start with {prefix!r}')
    a = hits[0]; b = a + 1
    while b < len(lines) and lines[b].startswith('  ') and not lines[b].startswith('  - '): b += 1
    words = ' '.join(l.strip() for l in lines[a:b])[2:].split(' ')
    out, line = [], '- '
    for w in words:
        cand = line + w if line in ('- ', '  ') else f'{line} {w}'
        if len(cand) > 120 and line not in ('- ', '  '): out.append(line); line = '  ' + w
        else: line = cand
    out.append(line)
    lines[a:b] = out
    s = '\n'.join(lines)
rewrap('- Built in seven commits')
rewrap('- Commands refused by the guard')
long = [l for l in s.split('\n') if len(l) > 120 and not l.startswith('|') and l not in long_before]
if long: raise SystemExit(f'{len(long)} lines over 120 characters: {long[0][:90]!r}')
open(p, 'w', encoding='utf-8').write(s)
print('ok')
