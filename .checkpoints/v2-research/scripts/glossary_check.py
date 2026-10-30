"""Glossary check (advisor condition 9, with the advisor's script fixes of 2026-10-30).

For every glossary entry, find each retired phrase in each topic file, outside quoted source text and (with
--skip-records) outside the dated records the advisor's step-3b answer lets the search skip. Whole words only; a hit
inside the entry's own term name, inside one of its "Marked by" phrases or inside a longer retired phrase does not
count; each place in a file is counted once, under the longest phrase that covers it. A phrase matches across a line
break (any run of spaces and line breaks between its words).

Quoted text: straight quotes are paired in order inside each block of text (a paragraph, a list item or a table row,
so a quote may run over a line break), curly quotes as opening and closing marks; a block with an odd number of
straight quotes leaves its last one unpaired (counted and printed per topic). Dated records (with --skip-records):
bracketed notes starting "[Fix pass", "[Second fix pass", "[Second pass", "[Verifier", "[Second verifier pass" or
"[Wave", and part 7 ("## Verifier spot-check", the verifier tables and fix logs). Every hit skipped as quoted text or
as a dated record is listed with --skipped, so the report can show them (the advisor's step-3b answer).

The Retire field may be split (the advisor, 2026-10-30) into "Retire (exact)" (must reach zero hits after step 3c)
and "Retire (in context)" (listed for the step-3d verifiers to judge); an unsplit "Retire" counts as in context.
Phrases under "Keeps:" are not retired. A retired phrase with "…" is a pattern in words: it is not searched, and it
is printed for the verifiers to read for.

With --terms, it also lists every use of each term (its name and its "Marked by" phrases, outside quotes and, with
--skip-records, dated records), for the refresh of the "Used today" lists (condition 9), and with --first-uses the
first use of each term per topic and whether "GLOSSARY.md" is linked on that line or the next. A "Marked by" phrase
written "X(s)" matches X and Xs; a trailing bracket such as "(the alternative)" is not part of the phrase; a phrase
with "…" is not searched; a one-letter phrase ("N") matches only as a capital letter.

Usage: python3 glossary_check.py [--skip-records] [--glossary file] [--json out.json] [--hits out.tsv]
       [--terms out.tsv] [--first-uses out.tsv] [--skipped out.tsv]
"""
import re, sys, json, collections, bisect

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

BLOCK_END = re.compile(r'\n(?=[ \t]*\n|[ \t]*[-*] |[ \t]*\d+\. |[ \t]*\||#)')


def quote_spans(text):
    """Quoted text: straight quotes paired in order inside each block, curly quotes by their marks."""
    spans, odd = [], 0
    starts = [0] + [m.end() for m in BLOCK_END.finditer(text)] + [len(text)]
    for a, b in zip(starts, starts[1:]):
        pos = [a + i for i, c in enumerate(text[a:b]) if c == '"']
        odd += len(pos) % 2
        spans += [(pos[i], pos[i + 1] + 1) for i in range(0, len(pos) - 1, 2)]
        spans += [(a + m.start(), a + m.end()) for m in re.finditer(r'“[^”]*”', text[a:b])]
    return spans, odd


def record_spans(text):
    spans = [m.span() for m in re.finditer(
        r'\[(?:Fix pass|Second fix pass|Second pass|Verifier|Second verifier pass|Wave)[^\]]*\]', text, re.I)]
    spans += [m.span() for m in re.finditer(r'^## (?:7\. )?Verifier spot-check.*?(?=^## )', text, re.S | re.M)]
    return spans


def pat(ph, marked=False):
    """The whole-word pattern for a phrase (lower case), or None for a phrase that is a pattern in words. Words may be
    split by any run of spaces and line breaks. For a "Marked by" phrase (marked=True), a trailing bracket that is not
    "(s)" is dropped when words remain before it."""
    if '…' in ph:
        return None
    ph = ' '.join(ph.split())
    if marked and not ph.endswith('(s)') and re.sub(r'\s*\([^)]*\)$', '', ph).strip():
        ph = re.sub(r'\s*\([^)]*\)$', '', ph)
    core = re.escape(ph.lower()).replace(re.escape('(s)'), 's?').replace(re.escape(' '), r'\s+')
    return r'(?<![\w-])' + core + r'(?![\w-])'


def finder(ph, text, low):
    """Matches of a term or "Marked by" phrase: a one-letter phrase as a capital letter only."""
    if len(ph.strip()) == 1:
        return re.finditer(r'(?<![\w-])' + re.escape(ph.strip()) + r'(?![\w-])', text)
    return re.finditer(pat(ph, marked=True), low)


def nearest_id(text, pos, topic):
    ids = re.findall(r'\*\*(' + topic + r'-[A-Z0-9]+-(?:F|REC|OQ)\d+)[.*]', text[max(0, pos - 4000):pos])
    return ids[-1] if ids else ''


hits = []          # (topic, line, nearest id, entry, kind, phrase)
skipped = []       # (topic, line, nearest id, entry, kind, phrase, where): hits inside quoted text or dated records
term_hits = []     # (topic, line, nearest id, entry, phrase)
first_uses = []    # (topic, entry, line, phrase, linked)
unsearchable = set()   # retired phrases written as a pattern in words (with "…"): listed for the verifiers to read
counts, skip_counts, odd_blocks = collections.Counter(), collections.Counter(), {}
for topic, fn in FILES.items():
    raw = open(R + fn).read()
    qs, odd_blocks[topic] = quote_spans(raw)
    rs = record_spans(raw) if skip_records else []
    where = bytearray(len(raw))      # 1 inside quoted text, 2 inside a dated record (a record wins)
    for a, b in qs:
        where[a:b] = b'\x01' * (b - a)
    for a, b in rs:
        where[a:b] = b'\x02' * (b - a)
    # masked copy for term uses: quoted text and records become NUL (not a space, so no phrase matches across them)
    text = ''.join(c if (not where[i] or c == '\n') else '\x00' for i, c in enumerate(raw))
    low, rawlow = text.lower(), raw.lower()
    protect = []   # spans of term names and "Marked by" phrases (hits inside them do not count)
    term_cands = []
    for e in entries:
        for ph in [e['name']] + e['marked']:
            if pat(ph, marked=True) is None:
                continue
            term_cands.append((len(ph), e['name'], ph))
            protect += [m.span() for m in finder(ph, text, low)]
    term_taken = bytearray(len(text))   # 1 where a counted term use already sits
    first = {}
    for _, name, ph in sorted(term_cands, reverse=True):
        for m in finder(ph, text, low):
            a, b = m.span()
            if any(term_taken[a:b]):
                continue
            term_taken[a:b] = b'\x01' * (b - a)
            line = raw.count('\n', 0, a) + 1
            term_hits.append((topic, line, nearest_id(raw, a, topic), name, ph))
            if name not in first or a < first[name][0]:
                first[name] = (a, line, ph)
    rawlines = raw.split('\n')
    for name, (a, line, ph) in first.items():
        near = '\n'.join(rawlines[line - 1:line + 1])
        first_uses.append((topic, name, line, ph, 'yes' if 'GLOSSARY.md' in near else 'no'))
    cands = []
    for e in entries:
        for kind in ('exact', 'context'):
            for ph in e[kind]:
                if pat(ph) is None:
                    unsearchable.add((e['name'], kind, ' '.join(ph.split())))
                    continue
                cands.append((len(ph), e['name'], kind, ph))
    protect.sort()
    starts = [pa for pa, pb in protect]
    max_end, run = [], -1
    for pa, pb in protect:
        run = max(run, pb)
        max_end.append(run)
    taken = bytearray(len(text))        # 1 where a counted hit already sits
    skip_taken = bytearray(len(text))   # the same for skipped hits
    for _, name, kind, ph in sorted(cands, reverse=True):
        for m in re.finditer(pat(ph), rawlow):
            a, b = m.span()
            line = raw.count('\n', 0, a) + 1
            inside = max(where[a:b])
            if inside:
                if not any(skip_taken[a:b]):
                    skip_taken[a:b] = b'\x01' * (b - a)
                    w = 'dated record' if inside == 2 else 'quoted text'
                    skipped.append((topic, line, nearest_id(raw, a, topic), name, kind, ph, w))
                    skip_counts[(topic, w)] += 1
                continue
            i = bisect.bisect_right(starts, a) - 1   # a protect span with start <= a and end >= b holds this hit
            if i >= 0 and max_end[i] >= b:
                continue
            if any(taken[a:b]):
                continue
            taken[a:b] = b'\x01' * (b - a)
            hits.append((topic, line, nearest_id(raw, a, topic), name, kind, ph))
            counts[(topic, kind)] += 1

exact_total = sum(v for (t, k), v in counts.items() if k == 'exact')
context_total = sum(v for (t, k), v in counts.items() if k == 'context')
def header(text):
    """The file's header: the text before the first part heading, or R6's own "## 1. Header" section."""
    parts = re.split(r'\n(?=## )', text)
    return parts[0] + ('\n' + parts[1] if len(parts) > 1 and re.match(r'## (1\. )?Header\b', parts[1]) else '')


links = {t: ('GLOSSARY.md' in header(open(R + f).read())) for t, f in FILES.items()}
print(f'entries {len(entries)}; exact hits {exact_total}; in-context hits {context_total}'
      f'{" (quotes and dated records skipped)" if skip_records else " (quotes skipped)"}')
for t in FILES:
    print(f'  {t}: exact {counts[(t, "exact")]}, in context {counts[(t, "context")]}, header links GLOSSARY.md: '
          f'{links[t]}; skipped: {skip_counts[(t, "quoted text")]} in quoted text, {skip_counts[(t, "dated record")]} '
          f'in dated records; blocks with an unpaired straight quote: {odd_blocks[t]}')
for name, kind, ph in sorted(unsearchable):
    print(f'  not searchable (a pattern in words; the step-3d verifiers read for it): {name} / {kind}: "{ph}"')


def write(path, head, rows):
    with open(path, 'w') as f:
        f.write(head + '\n')
        for h in sorted(rows):
            f.write('\t'.join(' '.join(str(x).split()) for x in h) + '\n')   # a phrase quoted over a line break: one line


if arg('--hits'):
    write(arg('--hits'), 'topic\tline\tnearest id\tentry\tkind\tphrase', hits)
if arg('--skipped'):
    write(arg('--skipped'), 'topic\tline\tnearest id\tentry\tkind\tphrase\tskipped as', skipped)
if arg('--terms'):
    write(arg('--terms'), 'topic\tline\tnearest id\tentry\tphrase', term_hits)
    print(f'term uses: {len(term_hits)} (written to {arg("--terms")})')
if arg('--first-uses'):
    write(arg('--first-uses'), 'topic\tentry\tline of first use\tphrase\tGLOSSARY.md on that line or the next',
          first_uses)
    print(f'first uses: {len(first_uses)}, of which linked: {sum(1 for u in first_uses if u[4] == "yes")}')
if arg('--json'):
    json.dump({'exact_total': exact_total, 'context_total': context_total, 'links': links,
               'by_topic': {f'{t}/{k}': v for (t, k), v in counts.items()},
               'skipped': {f'{t}/{w}': v for (t, w), v in skip_counts.items()},
               'odd_quote_blocks': odd_blocks, 'term_uses': len(term_hits),
               'first_uses_linked': sum(1 for u in first_uses if u[4] == 'yes'), 'first_uses': len(first_uses),
               'not_searchable': sorted(unsearchable)}, open(arg('--json'), 'w'), indent=1)
