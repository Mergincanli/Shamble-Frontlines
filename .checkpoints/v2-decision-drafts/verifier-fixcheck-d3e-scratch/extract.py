# Reads an HTML page on stdin, converts it to text lines, prints only lines matching the given regexes with context.
# Usage: curl ... | python3 -I extract.py <context_lines> <regex> [<regex> ...]
import sys, re, html
ctx = int(sys.argv[1]); pats = [re.compile(p) for p in sys.argv[2:]]
raw = sys.stdin.buffer.read().decode('utf-8', 'replace')
raw = re.sub(r'(?is)<(script|style)\b.*?</\1>', ' ', raw)
raw = re.sub(r'(?i)<br\s*/?>|</(p|div|li|h[1-6]|dt|dd|pre|tr|section|table|ul|ol|code-block)>', '\n', raw)
raw = re.sub(r'(?s)<[^>]+>', '', raw)
text = html.unescape(raw)
lines = [re.sub(r'[ \t\r\f\v]+', ' ', l).strip() for l in text.split('\n')]
lines = [l for l in lines if l]
keep = set()
for i, l in enumerate(lines):
    if any(p.search(l) for p in pats):
        keep.update(range(max(0, i - ctx), min(len(lines), i + ctx + 1)))
prev = -2
for i in sorted(keep):
    if i != prev + 1: print('---')
    print(f'{i+1}: {lines[i][:400]}')
    prev = i
print(f'=== text lines: {len(lines)}', file=sys.stderr)
