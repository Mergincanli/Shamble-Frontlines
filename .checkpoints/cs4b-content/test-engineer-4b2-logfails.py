# test-engineer review 4b-2: list each command of the plants log's last run with its failed tests (reads only).
import re, sys
text = open(sys.argv[1]).read()
start = int(sys.argv[2])
lines = text.split("\n")[start - 1:]
blocks = "\n".join(lines).split("\n$ ")
for b in blocks:
    head = b.split("\n", 1)[0][:110]
    rust = re.findall(r"^test (\S+) \.\.\. FAILED", b, re.M)
    vit = re.findall(r"^ FAIL  (.+)$", b, re.M)
    pw = re.findall(r"^\s+\d+\) (.+)$", b, re.M)
    ex = re.findall(r"^\[exit (\d+)\]", b, re.M)
    print(f"{ex[-1] if ex else '?'} | {head}")
    for n in rust + vit + pw[:3]: print(f"     - {n[:200]}")
