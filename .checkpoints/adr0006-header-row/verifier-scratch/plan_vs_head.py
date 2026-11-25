# For each planned After-text (plan.md section 3), check its pieces (split at [..] shortenings) occur in the head ADR.
import re, subprocess
R = "/home/user/Shamble-Frontlines"
plan = open(R + "/.checkpoints/adr0006-header-row/plan.md", encoding="utf-8").read()
head = subprocess.run(["git", "show", "1c45ed7:docs/adr/0006-infrastructure-operations.md"], capture_output=True, text=True, cwd=R).stdout
norm = lambda s: re.sub(r"\s+", " ", s).strip()
H = norm(head)
for m in re.finditer(r"\*\*(E\d+)\.[^\n]*\n(?:.*\n)*?- After: (.*(?:\n  .*)*)", plan):
    eid, after = m.group(1), norm(m.group(2))
    after = after.strip('"').rstrip('"')
    pieces = [p.strip(' ,;"') for p in re.split(r"\[[^\]]*\]", after) if len(p.strip(' ,;"')) > 8]
    res = [(p in H) for p in pieces]
    print(eid, "pieces", len(pieces), "found", sum(res))
    for p, ok in zip(pieces, res):
        if not ok: print("   MISSING:", p[:200])
