"""Step 3c rule check: "quoted source text never changes" (plan-amendments-2 condition 6; the step-3c prompts' rule 1).

For each topic file, lists every quoted string (straight quotes paired inside each block, as glossary_check.py pairs
them; curly quotes by their marks) found at the base commit and not found, with the same words, at the head commit,
and the reverse (quotes that are new at head, with whether each is found in the approved texts: resolutions.md,
cross-topic-oqs.md and the step-3b verdict, block-quote marks removed). Spaces and line breaks are normalized. A quote can leave legitimately
when an approved "before" text that held it was replaced; the step-3d verifiers judge each one.
Usage: python3 quotes_check.py <base> <head> [--out out.tsv]
"""
import re, subprocess, sys, collections

FILES = {'R1': 'r1-simulation-core.md', 'R2': 'r2-netcode.md', 'R3': 'r3-rendering.md',
         'R4': 'r4-physics-navigation.md', 'R5': 'r5-toolchain-quality.md', 'R6': 'r6-infrastructure.md'}
BLOCK_END = re.compile(r'\n(?=[ \t]*\n|[ \t]*[-*] |[ \t]*\d+\. |[ \t]*\||#)')
base, head = sys.argv[1], sys.argv[2]
out = sys.argv[sys.argv.index('--out') + 1] if '--out' in sys.argv else None


def show(ref, fn):
    return subprocess.run(['git', '-C', '/home/user/Shamble-Frontlines', 'show', f'{ref}:docs/research/{fn}'],
                          capture_output=True, text=True, check=True).stdout


def quotes(text):
    found = collections.Counter()
    starts = [0] + [m.end() for m in BLOCK_END.finditer(text)] + [len(text)]
    for a, b in zip(starts, starts[1:]):
        block = text[a:b]
        pos = [i for i, c in enumerate(block) if c == '"']
        for i in range(0, len(pos) - 1, 2):
            q = ' '.join(block[pos[i] + 1:pos[i + 1]].split())
            if len(q) >= 12:          # short quoted words ("N", "start") are terms, not source text
                found[q] += 1
        for m in re.finditer(r'“([^”]*)”', block):
            q = ' '.join(m.group(1).split())
            if len(q) >= 12:
                found[q] += 1
    return found


C = '/home/user/Shamble-Frontlines/.checkpoints/v2-research/'
APPROVED = ' '.join(re.sub(r'^\s*> ?', '', open(C + f).read(), flags=re.M).split()) for f in ()) if False else ' '.join(
    re.sub(r'^\s*> ?', '', ''.join(open(C + f).read() for f in
           ('resolutions.md', 'cross-topic-oqs.md', 'advisor-resolutions-verdict.md')), flags=re.M).split())
rows = []
for t, fn in FILES.items():
    qb, qh = quotes(show(base, fn)), quotes(show(head, fn))
    gone = [q for q in qb if q not in qh]
    new = [q for q in qh if q not in qb]
    print(f'{t}: quotes at base {sum(qb.values())}, at head {sum(qh.values())}; gone {len(gone)}, new {len(new)}')
    rows += [(t, 'gone', q, '') for q in gone] + [(t, 'new', q, 'yes' if q in APPROVED else 'no') for q in new]
if out:
    with open(out, 'w') as f:
        f.write('topic\tgone or new\tquoted text (spaces normalized)\tnew quote found in the approved texts\n')
        for r in rows:
            f.write('\t'.join(r) + '\n')
