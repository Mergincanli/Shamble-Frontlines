# Move ROADMAP's oldest "Recent work" entry (cs0, PR #56, merged) byte for byte to the end of the
# paragraph entries in "Build history" (after docs-after-54's block), per his OLD-LINES (a).
import sys
p = sys.argv[1]
b = open(p, 'rb').read()
lines = b.split(b'\n')
start = next(i for i, l in enumerate(lines) if l.startswith(b'**Change-set 0 of the build plan [2026-11-17, MERGED]**'))
end = len(lines)
while end > 0 and lines[end - 1] == b'':
    end -= 1
block = lines[start:end]
assert lines[start - 1] == b'', 'blank line before cs0 expected'
assert all(not l.startswith(b'**') for l in block[1:]), 'cs0 must be the last entry'
anchor = next(i for i, l in enumerate(lines) if l.startswith(b'2026-11-17 17:09:33 UTC by `merged_at`, by merge commit, on his go.'))
assert lines[anchor + 1] == b'' and lines[anchor + 2].startswith(b'- **Kit: KEPT-LOGS')
rest = lines[:start - 1] + lines[end:]
new = rest[:anchor + 1] + [b''] + block + rest[anchor + 1:]
out = b'\n'.join(new)
assert sorted(out.split(b'\n')) == sorted(b.split(b'\n')), 'multiset of lines must be equal'
open(p, 'wb').write(out)
print('moved', len(block), 'lines from', start + 1, 'to after', anchor + 1)
