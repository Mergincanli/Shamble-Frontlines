# docs-keeper (ci-preview-handover): moves lines by line ranges, byte for byte, never retyped (STATUS-DIET; OLD-LINES).
# Run once from the worktree root on the pristine files (their sha256 is checked first): python3 -I -B <this file>
# 1. STATUS line 25 (#70's In-flight row, merged 2026-11-26 as 2483e9a) -> the archive's "In flight", before "## Targets".
# 2. STATUS lines 104-105 (Follow-ups bullets of guard-env-fix #69 and ci-after-3a #71, both merged) -> the end of the
#    archive's "Follow-ups", before "## Decisions pending: the rest".
# 3. ROADMAP lines 389-398 (a blank line and the oldest "Recent work" entry, Part 15) -> "Build history", after line 208.
import hashlib, sys

def read(p):
    b = open(p, 'rb').read()
    return b, b.decode('utf-8').split('\n')

def write(p, lines):
    open(p, 'wb').write('\n'.join(lines).encode('utf-8'))

def sha(b):
    return hashlib.sha256(b).hexdigest()

S, A, R = 'docs/STATUS.md', 'docs/STATUS_ARCHIVE.md', 'docs/ROADMAP.md'
sb, s = read(S); ab, a = read(A); rb, r = read(R)
print('before', S, sha(sb), A, sha(ab), R, sha(rb))

def expect(lines, n, start):
    if not lines[n - 1].startswith(start):
        sys.exit('line %d does not start with %r: %r' % (n, start, lines[n - 1][:80]))

expect(s, 25, '| #70 | change-set 4a, "The recording"')
expect(s, 104, '- **From guard-env-fix (#69):**')
expect(s, 105, '- **From ci-after-3a** (the small CI pull request;')
expect(r, 390, '**Part 15 of the build plan, the second lane, to main')
if r[388] != '' or r[398] != '':
    sys.exit('ROADMAP: line 389 or the trailing empty element is not empty')
expect(r, 210, '- **Kit: KEPT-LOGS (2026-11-15):**')
if r[208] != '':
    sys.exit('ROADMAP line 209 is not empty')

row70 = s[24]
bullets = s[103:105]
part15 = r[388:398]  # the blank line 389 and lines 390-398

# Archive 1: In flight, before "## Targets".
t = a.index('## Targets')
if a[t - 1] != '':
    sys.exit('no blank line before ## Targets')
head1 = [
    "_Moved from STATUS (2026-11-26, ci-preview-handover's docs pass, under his OLD-LINES of 2026-11-18): STATUS's row for the merged change-set 4a (PR #70, 2483e9a, merged 2026-11-26 12:22:01 UTC by `merged_at`, read with gh's REST route), moved by script, byte for byte, so its cells read as they stood when it left STATUS:_",
    '',
    '| PR | Work | Branch | State | Waiting on |',
    '|---|---|---|---|---|',
    row70,
    '',
]
a[t:t] = head1

# Archive 2: end of Follow-ups, before "## Decisions pending: the rest".
d = a.index('## Decisions pending: the rest')
if a[d - 1] != '':
    sys.exit('no blank line before ## Decisions pending')
head2 = [
    "- **Moved from STATUS (2026-11-26, ci-preview-handover's docs pass)** (under his OLD-LINES: lines about merged pull",
    "  requests, moved by script, byte for byte): STATUS's Follow-ups bullets for guard-env-fix (PR #69, merged 2026-11-25",
    "  as 311c048) and ci-after-3a (PR #71, merged 2026-11-25 as 07d673b) are the two bullets below. The second one's",
    '  pointer leads to the block "From ci-after-3a" above; its "TOKEN-WORDS deferred again" is done on ci-preview-handover\'s',
    '  branch (the block "From ci-preview-handover" above).',
]
a[d - 1:d - 1] = head2 + bullets

# STATUS: remove the row and the two bullets (the session's new text goes in by Edit afterwards).
del s[103:105]
del s[24]

# ROADMAP: fold Part 15 into Build history.
del r[388:398]
r[208:208] = part15

write(S, s); write(A, a); write(R, r)
sb2, _ = read(S); ab2, _ = read(A); rb2, _ = read(R)
print('after ', S, sha(sb2), A, sha(ab2), R, sha(rb2))
# Check: every moved line is present, once, in its new home and gone from its old one.
_, s3 = read(S); _, a3 = read(A); _, r3 = read(R)
for line in [row70] + bullets:
    assert line not in s3 and a3.count(line) == 1, line[:60]
assert r3[208:218] == part15 and r3.count(part15[1]) == 1
print('moved: STATUS 25 ->', A, a3.index(row70) + 1, '; STATUS 104-105 ->', A, a3.index(bullets[0]) + 1, '-', a3.index(bullets[1]) + 1,
      '; ROADMAP 389-398 -> 209-218; lines now', len(s3) - 1, len(a3) - 1, len(r3) - 1)
