# Reads HTML on stdin; prints the [src] hrefs found near the given method ids (rustdoc "method.NAME").
import sys, re
raw = sys.stdin.buffer.read().decode('utf-8', 'replace')
for name in sys.argv[1:]:
    for m in re.finditer(r'id="method\.' + re.escape(name) + r'"', raw):
        seg = raw[m.start(): m.start() + 3000]
        for h in re.findall(r'href="([^"]*src/[^"]+)"', seg)[:2]:
            print(name, h)
