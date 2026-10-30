"""Glossary check (advisor condition 9, with the advisor's script fixes of 2026-10-30).

For every glossary entry, find each retired phrase in each topic file, outside quoted source text and (with
--skip-records) outside the dated records the advisor's step-3b answer lets the search skip. Whole words only; a hit
inside the entry's own term name, inside one of its "Marked by" phrases or inside a longer retired phrase does not
count; each place in a file is counted once, under the longest phrase that covers it.

The Retire field may be split (the advisor, 2026-10-30) into "Retire (exact)" (must reach zero hits after step 3c)
and "Retire (in context)" (listed for the step-3d verifiers to judge); an unsplit "Retire" counts as in context.
Phrases under "Keeps:" are not retired.

With --terms, it also lists every use of each term (its name and its "Marked by" phrases, outside quotes and, with
--skip-records, dated records), for the refresh of the "Used today" lists (condition 9). A "Marked by" phrase written
"X(s)" matches X and Xs; a trailing bracket such as "(the alternative)" is not part of the phrase; a phrase with "…"
is a pattern in words and is not searched; a one-letter phrase ("N") matches only as a capital letter.

Usage: python3 glossary_check.py [--skip-records] [--glossary file] [--json out.json] [--hits out.tsv] [--terms out.tsv]
"""
import re, sys, json, collections

R = '/home/user/Shamble-Frontlines/docs/research/'
FILES = {'R1': 'r1-simulation-core.md', 'R2': 'r2-netcode.md', 'R3': 'r3-rendering.md',
         'R4': 'r4-physics-navigation.md', 'R5': 'r5-toolchain-quality.md', 'R6': 'r6-infrastructure.md'}
skip_records = '--skip-records' in sys.argv


def arg(name):
    return sys.argv[sys.argv.index(name) + 1] if name in sys.argv else None


def field(body, key):
    m = re.search(r'\*\*' + re.escape(key) + r':\*\*(.*?)(?=\n- \*\*|\Z)', body, re.S)
    return m.group(1) if m else ''


def quoted(text):
    return [q for q in re.findall(r'"([^"]+)"', text) if q.strip()]


glossary = open(arg('--glossary') or R + 'GLOSSARY.md').read()   # --glossary: another copy, for before/after counts
entries = []
for m in re.finditer(r'^### (.+?)\n(.*?)(?=^### |^## |\Z)', glossary, re.S | re.M):
    name, body = m.group(1).strip(), m.group(2)
    exact = quoted(field(body, 'Retire (exact)'))
    context = quoted(field(body, 'Retire (in context)')) + quoted(field(body, 'Retire'))
    entries.append({'name': name, 'marked': quoted(field(body, 'Marked by')), 'exact': exact, 'context': context})


def masked(text):
    """Blank out quoted source text and, with --skip-records, the dated records, keeping offsets and lines."""
    spans = [m.span() for m in re.finditer(r'"[^"\n]{0,400}"|“[^”\n]{0,400}”', text)]
    if skip_records:
        spans += [m.span() for m in re.finditer(
            r'\[(?:Fix pass|Second fix pass|Second pass|Verifier|Second verifier pass|Wave)[^\]]*\]', text, re.I)]
        # the verifier spot-check part (part 7) and the fix logs under headings that name them
        for m in re.finditer(r'^## (?:7\. )?Verifier spot-check.*?(?=^## )', text, re.S | re.M):
            spans.append(m.span())
    chars = list(text)
    for a, b in spans:
        for i in range(a, b):
            if chars[i] != '\n':
                chars[i] = ' '
    return ''.join(chars)


def pat(ph):
    """The whole-word pattern for a phrase (lower case), or None for a phrase that is a pattern in words."""
    if '…' in ph:
        return None
    ph = re.sub(r'\s*\([^)]*\)$', '', ph) if not ph.endswith('(s)') else ph
    core = re.escape(ph.lower()).replace(re.escape('(s)'), 's?')
    return r'(?<![\w-])' + core + r'(?![\w-])'


def nearest_id(text, pos, topic):
    ids = re.findall(r'\*\*(' + topic + r'-[A-Z0-9]+-(?:F|REC|OQ)\d+)[.*]', text[max(0, pos - 4000):pos])
    return ids[-1] if ids else ''


hits = []          # (topic, line, nearest id, entry, kind, phrase)
term_hits = []     # (topic, line, nearest id, entry, phrase)
unsearchable = set()   # retired phrases written as a pattern in words (with "…"): listed for the verifiers to read
counts = collections.Counter()
for topic, fn in FILES.items():
    raw = open(R + fn).read()
    text = masked(raw)
    low = text.lower()
    taken = []     # spans already counted (each place once, longest phrase first)
    protect = []   # spans of term names and "Marked by" phrases (hits inside them do not count)
    term_cands = []
    for e in entries:
        for ph in [e['name']] + e['marked']:
            if pat(ph) is None:
                continue
            term_cands.append((len(ph), e['name'], ph))
            if len(ph.strip()) == 1:
                found = re.finditer(r'(?<![\w-])' + re.escape(ph.strip()) + r'(?![\w-])', text)
            else:
                found = re.finditer(pat(ph), low)
            for m in found:
                protect.append(m.span())
    term_taken = []
    for _, name, ph in sorted(term_cands, reverse=True):
        found = (re.finditer(r'(?<![\w-])' + re.escape(ph.strip()) + r'(?![\w-])', text) if len(ph.strip()) == 1
                 else re.finditer(pat(ph), low))
        for m in found:
            a, b = m.span()
            if any(not (b <= ta or a >= tb) for ta, tb in term_taken):
                continue
            term_taken.append((a, b))
            term_hits.append((topic, raw.count('\n', 0, a) + 1, nearest_id(raw, a, topic), name, ph))
    cands = []
    for e in entries:
        for kind in ('exact', 'context'):
            for ph in e[kind]:
                if pat(ph) is None:
                    unsearchable.add((e['name'], kind, ' '.join(ph.split())))
                    continue
                cands.append((len(ph), e['name'], kind, ph))
    for _, name, kind, ph in sorted(cands, reverse=True):
        for m in re.finditer(pat(ph), low):
            a, b = m.span()
            if any(pa <= a and b <= pb for pa, pb in protect):
                continue
            if any(not (b <= ta or a >= tb) for ta, tb in taken):
                continue
            taken.append((a, b))
            line = raw.count('\n', 0, a) + 1
            hits.append((topic, line, nearest_id(raw, a, topic), name, kind, ph))
            counts[(topic, kind)] += 1

exact_total = sum(v for (t, k), v in counts.items() if k == 'exact')
context_total = sum(v for (t, k), v in counts.items() if k == 'context')
links = {t: ('GLOSSARY.md' in open(R + f).read().split('\n## ', 1)[0]) for t, f in FILES.items()}
print(f'entries {len(entries)}; exact hits {exact_total}; in-context hits {context_total}'
      f'{" (quotes and dated records skipped)" if skip_records else " (quotes skipped)"}')
for t in FILES:
    print(f'  {t}: exact {counts[(t, "exact")]}, in context {counts[(t, "context")]}, header links GLOSSARY.md: {links[t]}')
for name, kind, ph in sorted(unsearchable):
    print(f'  not searchable (a pattern in words; the step-3d verifiers read for it): {name} / {kind}: "{ph}"')
if arg('--hits'):
    with open(arg('--hits'), 'w') as f:
        f.write('topic\tline\tnearest id\tentry\tkind\tphrase\n')
        for h in sorted(hits):
            f.write('\t'.join(str(x) for x in h) + '\n')
if arg('--terms'):
    with open(arg('--terms'), 'w') as f:
        f.write('topic\tline\tnearest id\tentry\tphrase\n')
        for h in sorted(term_hits):
            f.write('\t'.join(str(x) for x in h) + '\n')
    print(f'term uses: {len(term_hits)} (written to {arg("--terms")})')
if arg('--json'):
    json.dump({'exact_total': exact_total, 'context_total': context_total, 'links': links,
               'by_topic': {f'{t}/{k}': v for (t, k), v in counts.items()},
               'term_uses': len(term_hits), 'not_searchable': sorted(unsearchable)}, open(arg('--json'), 'w'), indent=1)
