# Decode rustc .rmeta doc-comment symbols near a needle (heuristic: SYMBOL_STR=0, SYMBOL_OFFSET=1, SYMBOL_PREINTERNED=2)
import sys, re
p, needle, span = sys.argv[1], sys.argv[2].encode(), int(sys.argv[3])
d = open(p, 'rb').read()
def leb(i):
    r = s = 0
    while True:
        b = d[i]; i += 1; r |= (b & 0x7f) << s; s += 7
        if b < 0x80: return r, i
def strat(i):  # i points at the length (the tag byte 0 is just before it)
    assert d[i - 1] == 0, hex(d[i - 1]); n, j = leb(i); assert d[j + n] == 0xc1; return d[j:j + n].decode('utf8', 'replace')
for start in [int(needle)]:
    pass
    start = start; print('=== needle at', start)
    i = start - 60; end = start + span
    out = []
    while i < end:
        if d[i] in (0x5f, 0x60) and d[i + 1] in (0, 1, 2):
            t = d[i + 1]
            try:
                if t == 0:
                    n, j = leb(i + 2); s = d[j:j + n]
                    if j + n < len(d) and d[j + n] == 0xc1:
                        out.append(('STR', s.decode('utf8', 'replace'))); i = j + n + 1; continue
                elif t == 1:
                    off, j = leb(i + 2)
                    if 0 < off < len(d) and d[off - 1] == 0:
                        out.append(('OFF', strat(off))); i = j; continue
                elif t == 2:
                    idx, j = leb(i + 2); out.append(('PRE', idx)); i = j; continue
            except Exception as e:
                pass
        i += 1
    for k, v in out: print(k, repr(v))
