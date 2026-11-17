"""Re-wrap prose paragraphs and list items of a Markdown file to a width, keeping backtick spans whole.
Tables, headings, code fences and blank lines are kept as they are. Usage: python3 rewrap.py IN OUT [WIDTH]"""
import re, sys
src, out = sys.argv[1], sys.argv[2]
W = int(sys.argv[3]) if len(sys.argv) > 3 else 120
RISKY = re.compile(r'^([-+*>]|#+|\d+[.)]|\|.*)$')
ITEM = re.compile(r'^(- |\d+\. )')

def toks(text):
    out, cur, inb = [], '', False
    for ch in text:
        if ch == '`':
            inb = not inb; cur += ch
        elif ch == ' ' and not inb:
            if cur: out.append(cur); cur = ''
        else:
            cur += ch
    if cur: out.append(cur)
    return out

def wrap(text, p0, pc):
    t = toks(text); lines = []; cur = p0; first = True
    for i, w in enumerate(t):
        base = p0 if first else pc
        cand = (cur + w) if cur == base else (cur + ' ' + w)
        if len(cand) <= W or cur == base:
            cur = cand
        else:
            if RISKY.match(w):  # never start a continuation line with a list or block marker
                head, _, last = cur.rpartition(' ')
                lines.append(head); cur = pc + last + ' ' + w
            else:
                lines.append(cur); cur = pc + w
            first = False
    lines.append(cur)
    return lines

L = open(src, encoding='utf-8').read().split('\n')
res, i, fence = [], 0, False
while i < len(L):
    ln = L[i]
    if ln.startswith('```'):
        fence = not fence; res.append(ln); i += 1; continue
    if fence or ln == '' or ln.startswith('|') or ln.startswith('#'):
        res.append(ln); i += 1; continue
    m = ITEM.match(ln)
    if m:
        p0 = m.group(1); pc = ' ' * len(p0); buf = [ln[len(p0):]]; i += 1
        while i < len(L) and L[i].startswith(pc) and L[i].strip() and not ITEM.match(L[i]):
            buf.append(L[i].strip()); i += 1
        res.extend(wrap(' '.join(buf), p0, pc)); continue
    buf = [ln]; i += 1
    while i < len(L) and L[i] and not L[i].startswith(('|', '#', '```')) and not ITEM.match(L[i]):
        buf.append(L[i].strip()); i += 1
    res.extend(wrap(' '.join(buf), '', ''))
open(out, 'w', encoding='utf-8').write('\n'.join(res))
a = toks(' '.join(x.strip() for x in L)); b = toks(' '.join(x.strip() for x in res))
print('lines', len(L), '->', len(res), '| tokens equal:', a == b, '| over width (non-table):',
      [k + 1 for k, x in enumerate(res) if len(x) > W and not x.startswith('|')])
