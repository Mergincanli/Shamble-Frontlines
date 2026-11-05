# read-only: check that each word-for-word text in docs/STATUS.md matches its source (whitespace folded)
import re, sys
root = sys.argv[1]
N = root + '/.checkpoints/kit-v3-3/'
def lines(p): return open(p, encoding='utf-8').read().split('\n')
def fold(s): return re.sub(r'\s+', ' ', s).strip()
adv, q = lines(N + 'advisor-pass4.md'), lines(N + 'queue/mustafa-2026-11-05-a4-b1.md')
st = open(root + '/docs/STATUS.md', encoding='utf-8').read()
stF = fold(st)
stQ = fold('\n'.join(re.sub(r'^\s*> ?', '', l) for l in st.split('\n')))  # quote markers removed
def span(L, a, b, strip=''):
    t = fold(' '.join(L[a-1:b]))
    return t[len(strip):] if strip and t.startswith(strip) else t
checks = [
  ('R-PLAIN line (advisor-pass4.md:200-210)', span(adv, 200, 210, '- '), stF),
  ('fix-pass-4 summary (advisor-pass4.md:212-220, inside the quotes)', re.search(r'"(Fix pass 4 .*?)"$', span(adv, 212, 220)).group(1), stF),
  ('FU-P4a (advisor-pass4.md:235-239)', span(adv, 235, 239, '1. '), stF),
  ('FU-P4b (advisor-pass4.md:240-246)', span(adv, 240, 246, '2. '), stF),
  ('FU-P4c (advisor-pass4.md:247-252)', span(adv, 247, 252, '3. '), stF),
  ('A4-FREEZE (queue file :8)', span(q, 8, 8), stQ),
  ('B.1 addition, whole list (queue file :9-17)', span(q, 9, 17), stQ),
  ('scope stop (mustafa-words.md:27, unchanged)', fold(lines(N + 'mustafa-words.md')[26]).strip('"'), stF),
]
for name, text, hay in checks:
    print(('FOUND ' if text in hay else 'MISSING ') + name + f' ({len(text)} chars, {hay.count(text)} occurrence(s))')
for name, old in [('old R-PLAIN text', 'and `>&` or `<&` before a number or `-`. Inside double'),
                  ('old A4 queue line', 'Queued after A3, plan first'),
                  ('old WT-NIGHT ref', 'NIGHT_SHIFT.md:27-31')]:
    print(('GONE ' if fold(old) not in stF else 'STILL THERE ') + name)
