# R-WORDS round 2 (read-only): STATUS's kit quote against his A2 paragraph, byte for byte.
# Holds none of his words: it finds his message by uuid, STATUS's quote by STATUS's own lead-in,
# and prints booleans, counts and lengths only.
import json, sys
T = '/root/.claude/projects/-home-user-Shamble-Frontlines/7261f3d7-f651-586a-9c39-c052553b68a3.jsonl'
UUID = 'e5fa7ca0-6494-44d0-8366-81fc612c7958'
ROOT = '/home/user/Shamble-Frontlines/'
a2 = None
n = 0
for line in open(T, encoding='utf-8'):
    try:
        o = json.loads(line)
    except Exception:
        continue
    if o.get('uuid') == UUID:
        n += 1
        a2 = o['message']['content']
print('A2 found', n, 'type str', isinstance(a2, str), 'chars', len(a2))
st = open(ROOT + 'docs/STATUS.md', encoding='utf-8').read()
lead = 'His words of 2026-11-13 for step 2.3:'  # STATUS's own lead-in (not his words)
i = st.find(lead)
print('lead-in found once', i >= 0 and st.find(lead, i + 1) < 0)
q0 = st.find('"', i)
q1 = st.find('"', q0 + 1)
raw = st[q0 + 1:q1]
print('STATUS quote raw chars', len(raw), 'line breaks inside', raw.count('\n'))
segs = raw.split('\n')
strict = segs[0] + ''.join(' ' + s.lstrip(' ') for s in segs[1:])
# every continuation must be indented by exactly two spaces (the list's own indent), no trailing spaces
print('continuations indented by 2 exactly', all(s.startswith('  ') and not s.startswith('   ') for s in segs[1:]))
print('no trailing spaces before breaks', all(not s.endswith(' ') for s in segs[:-1]))
cnt = a2.count(strict)
print('joined quote is a substring of A2', cnt >= 1, 'occurrences', cnt)
k = a2.find(strict)
ls = a2.rfind('\n', 0, k) + 1
le = a2.find('\n', k)
le = len(a2) if le < 0 else le
para = a2[ls:le]
print('A2 paragraph (line) chars', len(para), 'quote chars', len(strict))
print('quote ends exactly at the paragraph end', k + len(strict) == le)
pre = a2[ls:k]
print('prefix before quote: chars', len(pre), 'sentences', pre.count('. '), 'ends with ". "', pre.endswith('. '))
print('chars left out at the end', le - (k + len(strict)))
# sentence count of the quote (by ". " plus the final ".")
print('quote sentences', strict.count('. ') + (1 if strict.endswith('.') else 0))
# byte level: utf-8 bytes equal
print('utf-8 bytes equal to A2 slice', strict.encode('utf-8') == a2[k:k + len(strict)].encode('utf-8'))
# non-ASCII code points in the quote, by code point only
print('non-ascii code points in quote', sorted({hex(ord(c)) for c in strict if ord(c) > 127}))
