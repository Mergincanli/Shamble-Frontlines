import sys, re
path, prefix = sys.argv[1], sys.argv[2]
for i, line in enumerate(open(path, encoding='utf-8'), 1):
    if not line.startswith('| ' + prefix): continue
    cells = [c.strip() for c in line.strip().strip('|').split(' | ')]
    mid = cells[0]
    hw = cells[3] if len(cells) > 3 else ''
    m = re.search(r'Bites?[^:]*:(.*?)(\| r[0-9]|\| R[0-9]|\| GLOSSARY|$)', line)
    b = m.group(1).strip() if m else 'NONE'
    print(f"{i} {mid} HW[{hw[:90]}] BITES[{b[:420]}]")
