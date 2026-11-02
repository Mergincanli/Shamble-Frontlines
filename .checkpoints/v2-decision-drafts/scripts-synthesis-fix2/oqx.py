import re, glob, sys
def extract(snap):
    names = {}
    out = []
    for f in sorted(glob.glob(f'{snap}/000[1-6]-*.md')):
        base = f.split('/')[-1]; a = base[:4]; names[a] = base
        L = open(f, encoding='utf8').read().split('\n')
        isstart = lambda l: re.match(r'^- \*\*OQ-000\d-[A-Z]*\d+[\.:]', l)
        for i, l in enumerate(L):
            if not isstart(l): continue
            j = i + 1
            while j < len(L) and not isstart(L[j]) and not L[j].startswith('#') and not L[j].startswith('- **') and j < i + 40:
                j += 1
            blk = ' '.join(x.strip() for x in L[i:j])
            m = re.match(r'- \*\*(OQ-[^ .:]+)[\.:]? ?(.*?)\*\*', blk)
            d = re.search(r'\*?Decider[s]?:\*?\s*(.*?)(?:\s[·]\s|\s-\s\*What decides|\s\*?What decides it|\.\s+What decides|\.\s+Research ids|$)', blk)
            out.append((a, i + 1, m.group(1), m.group(2).strip(), (d.group(1).strip().rstrip('.') if d else '')))
    return names, out
if __name__ == '__main__':
    names, out = extract(sys.argv[1])
    for r in out: print('\t'.join(map(str, r)))
