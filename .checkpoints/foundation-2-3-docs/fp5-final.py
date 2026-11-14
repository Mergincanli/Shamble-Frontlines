# Step 8, the final commit's texts (plan revision 2f, as ruled): report-body.md, docs/STATUS.md, docs/ROADMAP.md.
# Each edit an exact replacement found once (or on a joined bullet or paragraph, then re-wrapped to 120 columns).
# The consultation-14 entry and the header's notes SHA are written later, by hand. Run once from the repository root:
#   python3 -I .checkpoints/foundation-2-3-docs/fp5-final.py
import re
B = '.checkpoints/foundation-2-3-docs/report-body.md'
s = open(B, encoding='utf-8').read()
long_before = {l for l in s.split('\n') if len(l) > 120}
def rep(old, new):
    global s
    n = s.count(old)
    if n != 1: raise SystemExit(f'found {n} times: {old[:100]!r}')
    s = s.replace(old, new)
def wrapwords(words, first, rest):
    out, line = [], first
    for w in words:
        cand = line + w if line in (first, rest) else f'{line} {w}'
        if len(cand) > 120 and line not in (first, rest): out.append(line); line = rest + w
        else: line = cand
    out.append(line); return out
def edit_block(prefix, old, new, first='- ', rest='  ', para=False):
    # A bullet (first/rest indents) or a plain paragraph: join, replace once, re-wrap.
    global s
    lines = s.split('\n')
    hits = [i for i, l in enumerate(lines) if l.startswith(prefix)]
    if len(hits) != 1: raise SystemExit(f'edit_block: {len(hits)} lines start with {prefix!r}')
    a = hits[0]; b = a + 1
    if para:
        while b < len(lines) and lines[b].strip(): b += 1
    else:
        while b < len(lines) and lines[b].startswith(rest) and not lines[b].startswith(rest + '- ') and lines[b].strip() and not re.match(r'^\d+\. ', lines[b]): b += 1
    joined = ' '.join(l.strip() for l in lines[a:b])
    if old is not None:
        if joined.count(old) != 1: raise SystemExit(f'edit_block: {joined.count(old)} hits of {old[:80]!r} in {prefix!r}')
        joined = joined.replace(old, new)
    lines[a:b] = (wrapwords(joined.split(' '), '', '') if para else wrapwords(joined[len(first):].split(' '), first, rest))
    s = '\n'.join(lines)

# F5. Outcome.
edit_block('- Built in eight commits', "Built in eight commits (1123175, 2f96f5c, 83c6ab6, d59ca43, 554face for review round 1's fix pass, 8e4fe7c for round 2's, a1d0317 its follow-up, then the verifier's fix pass), every script check and the gate passing; CI green on each through a1d0317 (E7).",
    "Built in nine commits (1123175, 2f96f5c, 83c6ab6, d59ca43, 554face for review round 1's fix pass, 8e4fe7c for round 2's, a1d0317 its follow-up, 922cadf the verifier's fix pass, then the final commit, which deletes the notes), every script check and the gate passing; CI green on each through 922cadf (E7).")
edit_block('- Built in nine commits', "The verifier failed a1d0317 on one High (how Mustafa was told about the advisor model, written from recall), fixed in the verifier's fix pass; its re-run and the advisor agent's review of the final head follow.",
    "The verifier failed a1d0317 on one High (how Mustafa was told about the advisor model, written from recall), fixed in 922cadf; its re-run passed 922cadf with two Lows, fixed in the final commit; the advisor agent's review of the final head follows.")

# F7. Plan and approval: revisions 2e and 2f; C1 and C10 met.
rep("- The plan: `.checkpoints/foundation-2-3-docs/plan.md`, revisions 1, 2, 2a, 2b, 2c and 2d.",
    "- The plan: `.checkpoints/foundation-2-3-docs/plan.md`, revisions 1, 2, 2a, 2b, 2c, 2d, 2e and 2f.")
edit_block('- Its rulings: 2a', "no round 3, as no Critical or High stays open).",
    "no round 3, as no Critical or High stays open); 2e (the verifier's fix pass: the chat bullet in its own exact words, the rest approved, no new review round); 2f (the final commit's texts).")
edit_block('- The conditions: C1', "C1 #40's facts read at the build (the second read, on the final head: step 8);",
    "C1 #40's facts read at the build and again for the final commit (its checkpoint at d0d1cfd, read 2026-11-14 01:05 UTC);")
edit_block('- The conditions: C1', "C10 this header names the notes' last commit and the checkpoint is folded in here (step 8).",
    "C10 this header names the notes' last commit, and the checkpoint is folded in (\"From the checkpoint\").")

# F7. E7: a1d0317's and 922cadf's runs.
rep("8e4fe7c, round 2's fix pass (43528414752, 43528420177; 43528414776, 43528420127); its follow-up: listed in the final commit;",
    "8e4fe7c, round 2's fix pass (43528414752, 43528420177; 43528414776, 43528420127); a1d0317, its follow-up (43529793262, 43529799958; 43529793189, 43529799786); 922cadf, the verifier's fix pass (43536145305, 43536148552; 43536145352, 43536148433);")

# F3. The verifier's scope note, quoted from its hand-back.
edit_block('The verifier (a general agent on Opus 5.5, asked for effort max', "Its scope note: everything after 554face, the new DECISIONS entry included, had the verifier and the advisor agent but no reviewer.",
    "Its scope note, in its words (its hand-back, transcript line 2922): \"A4, E-STATUS and everything after 554face (8e4fe7c and a1d0317) had no reviewer. Only this verifier covered them.\" The advisor agent ruled on the texts (plan revisions 2d and 2e) and reviews the final head.", para=True)

# F4. The re-run, as it happened.
rep("""Its re-run on the verifier's fix pass and the advisor agent's review of the final head: to come.
""", """Its re-run, on 922cadf: PASS WITH NOTES. F-V1 fixed; nothing Critical, High or Medium; two Lows and two nits, all
fixed in the final commit.

| Id | Severity | Finding | Disposition |
|---|---|---|---|
| F-V5a | Low | four files named as certain copies hold fragments only (`review-r1.js` with its launched copy, `dk-para.txt`, `dk-para-wrapped.txt`) | fixed: moved to "Fragments", the 30-character threshold in the definition, `words.mjs`'s 28-character anchor named |
| F-V6 | Low | its scope note was reworded | fixed: quoted from its hand-back |

The verifier did not see the changes after 922cadf; the advisor agent's review of the final head covers them.
""")

# F7. The "done" saves; entry 11 re-wrapped; entry 13 (entry 14 by hand).
rep("pass 22:19:55 (461c82b, after number 8), review round 2 22:52:48 (ef1a7c1, after number 9), round 2's fix pass\n23:49:35 (7249a13, after number 11).",
    "pass 22:19:55 (461c82b, after number 8), review round 2 22:52:48 (ef1a7c1, after number 9), round 2's fix pass\n23:49:35 (7249a13, after number 11), the verifier's fix pass 2026-11-14 00:46:06 (dd4d880, after number 12), the\nverifier 01:04:17 (b37cdd2, after number 13).")
edit_block('11. 23:39:10, moment 5', None, None, first='11. ', rest='   ')
rep("""   recorded as it happened; the same verifier resumed with a narrow brief.
""", """   recorded as it happened; the same verifier resumed with a narrow brief.
13. 2026-11-14 01:01:09, moment 5 for step 7, the verifier (before its "done" save), and moment 1 for the final edits:
   step 7 done; revision 2f asked before writing; a keyword screen of every chat text from 22:40 UTC; no status line on
   how Mustafa was told until the final commit; the fold pointing at sections; the appendix intro's list re-derived from
   the scan; the saver stopped before the notes SHA; every count recomputed; nothing pushed after the final commit.
{{C14}}
""")

# F1. The D&D bullet: six messages.
edit_block('- The advisor model (RW2-F1', "Four chat messages since bear on it (the transcript's lines 1887, 2122, 2869 and 2929):",
    "Six chat messages since bear on it (the transcript's lines 1887, 2122, 2869, 2929, 3183 and 3195):")
edit_block('- The advisor model (RW2-F1', "and that two correcting updates it believed it had sent are not.",
    "and that two correcting updates it believed it had sent are not; at 00:46:25 it wrote that this bullet was now worded only from its four chat messages on record, in the advisor agent's exact wording; at 01:00:24 it wrote that its 00:46 update also bore on how it had told him about the advisor tool's model, and that by the advisor agent's rule the report had to list it too.")

# F2. The copies bullet.
P = '- Copies of his words in the notes folder'
edit_block(P, "Certain copies (whole sentences or longer):", "Certain copies, meaning files where some pushed version holds a run of 30 characters or more that is a whole sentence of his or longer:")
edit_block(P, "byte-identical to the script cuts), `dk-para.txt` and `dk-para-wrapped.txt`;", "byte-identical to the script cuts);")
edit_block(P, "the scripts that cut or check his words and the review briefs (`decisions.mjs`, `pointers.mjs`, `fixpass1.mjs`, `review-r1.js` and `review-r2.js` with their launched copies: anchors, negative controls, and his kit words for the reviewers); and R-WORDS's notes at ec8f1bd.",
    "the scripts that cut or check his words and a review brief (`decisions.mjs`, `pointers.mjs`, `fixpass1.mjs`, and `review-r2.js` with its launched copy: anchors, negative controls, and his kit words for the reviewers); and R-WORDS's notes at ec8f1bd. `words.mjs` also holds one whole sentence of his under that length: the decision's first sentence (28 characters), its cut anchor.")
edit_block(P, "Fragments, among others: R-STATE's notes and `rs-roadmap.mjs` (RS-SELF), R2-WORDS's notes at 461c82b and 7b5f53c (RW2-F5).",
    "Fragments, among others: R-STATE's notes and `rs-roadmap.mjs` (RS-SELF), R2-WORDS's notes at 461c82b and 7b5f53c (RW2-F5), `review-r1.js` with its launched copy, and docs-keeper's `dk-para.txt` and `dk-para-wrapped.txt`.")

# F6. The fold.
rep("""## Risks
""", """## From the checkpoint
The checkpoint's essentials (`.claude/CHECKPOINTS.md` §9): the steps and their commits are in Outcome, E7 and "The
advisor tool" (the "done" saves); the decisions in "Plan and approval" and "Decisions and deviations"; the findings in
"Reviews" and "Findings for step 2.3's plan"; none is open. Numbers not given above: main 4d21c1a's CI (gate run
43461710700, toolchain run 43461710673, both success); PR #40's checkpoint as read for STATUS (0066b63, read on
2026-11-13 at 20:29 UTC; d0d1cfd, read on 2026-11-14 at 01:05 UTC); STATUS at 120 lines.

## Risks
""")

long = [l for l in s.split('\n') if len(l) > 120 and not l.startswith('|') and l not in long_before]
if long: raise SystemExit(f'{len(long)} new lines over 120 characters: {long[0][:90]!r}')
open(B, 'w', encoding='utf-8').write(s)

# F7. STATUS and ROADMAP.
def edit_file(path, pairs):
    t = open(path, encoding='utf-8').read()
    for old, new in pairs:
        if t.count(old) != 1: raise SystemExit(f'{path}: {t.count(old)} hits of {old[:90]!r}')
        t = t.replace(old, new)
    open(path, 'w', encoding='utf-8').write(t)
edit_file('docs/STATUS.md', [
    ("_Last updated: 2026-11-13 (step 2.3's first docs pass, PR #52; main 4d21c1a)._", "_Last updated: 2026-11-14 (step 2.3's first docs pass, PR #52; main 4d21c1a)._"),
    ("| draft | the verifier, the advisor agent's review of its final head; then his go (risk tier 1, decision docs) |",
     "| final (marked ready after the advisor agent's review) | the advisor agent's review of its final head; then his go (risk tier 1, decision docs) |"),
    ("U4: U4d2 merged, U4r's builder running; main 4d21c1a merged in (7685b2b) (its checkpoint at 0066b63, read 2026-11-13 20:29 UTC)",
     "U4: U4d2 merged, U4r's build on its fourth builder copy; main 4d21c1a merged in (7685b2b) (its checkpoint at d0d1cfd, read 2026-11-14 01:05 UTC)"),
])
edit_file('docs/ROADMAP.md', [
    ("**The foundation, 2.3: the first docs pass [2026-11-13, PR #52: in review]**", "**The foundation, 2.3: the first docs pass [2026-11-13 to 11-14, PR #52: in review]**"),
])
t = open('docs/ROADMAP.md', encoding='utf-8').read()
lines = t.split('\n'); a = next(i for i, l in enumerate(lines) if l.startswith('**The foundation, 2.3: the first docs pass [2026-11-13 to 11-14'))
b = a + 1
while b < len(lines) and lines[b].strip(): b += 1
joined = ' '.join(l.strip() for l in lines[a:b])
old = "Next: the verifier, the advisor agent's review of the final head, then his go."
if joined.count(old) != 1: raise SystemExit('ROADMAP: the Next line is not found once')
lines[a:b] = wrapwords(joined.replace(old, "Next: the advisor agent's review of the final head, then his go.").split(' '), '', '')
open('docs/ROADMAP.md', 'w', encoding='utf-8').write('\n'.join(lines))
print('ok')
