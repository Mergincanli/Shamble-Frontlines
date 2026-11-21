"""L2 of the advisor's U4d2 brief check (advisor-u4d2-brief.md): the u4-build-plan.md edits for U4D2-Q2, Q3, Q6, Q7,
Q9 (Lanes), Q10, Q11, A1 with A12's seat, and A13's §4.11, in the main session's own commit.
Every old and new phrase is read from the ruling's own lines (the item's line range, wrapped lines joined), never
retyped: the old phrase is the text between `replace "` and `" with "`, the new one runs from there to the item's last
`"`. Each old phrase must occur once in the plan (line breaks read as spaces); the bullet that holds it is rewrapped
from the match's first line to the bullet's end, within 120 characters, its earlier lines untouched.
Run from the repo root:  python3 -I .checkpoints/netcode-proof-test/u4d2_plan_edits.py [--apply]
Without --apply it changes nothing and prints the diff it would make."""
import difflib
import hashlib
import re
import sys
import textwrap

N = ".checkpoints/netcode-proof-test"
APPLY = "--apply" in sys.argv
RULING = f"{N}/advisor-u4d2-brief.md"
PLAN = f"{N}/u4-build-plan.md"
with open(RULING, encoding="utf-8") as fh:
    SRC = fh.read().split("\n")

# (tag, first line, last line) of each plan item in the ruling, 1-based and inclusive.
ITEMS = [
    ("A1+A12 Reviewers", 173, 176),
    ("Q2 wasm step", 298, 300),
    ("Q3 begin", 307, 309),
    ("Q6 features", 345, 347),
    ("Q7 third exception", 353, 355),
    ("Q9 Lanes", 381, 385),
    ("Q10 lab clippy.toml", 392, 394),
    ("Q11 vector plants", 400, 403),
    ("A13 §4.11", 579, 582),
]


def joined(a, b):
    return " ".join(l.strip() for l in SRC[a - 1:b])


def phrases(tag, a, b):
    j = joined(a, b)
    assert "plan" in j.lower(), (tag, "not a plan item")
    assert j.count('replace "') == 1, (tag, j.count('replace "'))
    i0 = j.index('replace "') + len('replace "')
    i1 = j.index('" with "', i0)
    old = j[i0:i1]
    i2 = i1 + len('" with "')
    i3 = j.rindex('"')
    assert i3 > i2, tag
    return old, j[i2:i3]


def ws(text):
    return r"\s+".join(re.escape(w) for w in text.split())


def sha(path):
    with open(path, "rb") as fh:
        return hashlib.sha256(fh.read()).hexdigest()


with open(PLAN, encoding="utf-8") as fh:
    plan = fh.read().split("\n")
orig = list(plan)
BULLET = re.compile(r"^( *)- ")
log = []
for tag, a, b in ITEMS:
    old, new = phrases(tag, a, b)
    text = "\n".join(plan)
    ms = list(re.finditer(ws(old), text))
    if len(ms) != 1:
        sys.exit(f"{tag}: the old phrase occurs {len(ms)} times in the plan: {old!r}")
    m = ms[0]
    first = text.count("\n", 0, m.start())
    last = text.count("\n", 0, m.end())
    s = first
    while not BULLET.match(plan[s]):
        s -= 1
        assert s >= 0 and plan[s].strip() and not plan[s].startswith("#"), (tag, "no bullet above the match")
    e = s
    while e + 1 < len(plan) and plan[e + 1].strip() and not BULLET.match(plan[e + 1]) \
            and not plan[e + 1].startswith(("#", "|")):
        e += 1
    assert e >= last, (tag, "the match runs past its bullet", s, e, last)
    lead = BULLET.match(plan[s]).group(1)
    cont = lead + "  "
    block = "\n".join(plan[first:e + 1])
    off = m.start() - (len("\n".join(plan[:first])) + (1 if first else 0))
    end = m.end() - (len("\n".join(plan[:first])) + (1 if first else 0))
    assert block[off:end].split() == old.split(), (tag, block[off:end])
    body = block[:off] + new + block[end:]
    flat = " ".join(x.strip() for x in body.split("\n"))
    if first == s:
        flat = flat[len(lead) + 2:] if flat.startswith("- ") else flat
        if plan[s].startswith(lead + "- "):
            flat = " ".join(x.strip() for x in body.split("\n"))
            assert flat.startswith("- "), tag
            flat = flat[2:]
        wrap = textwrap.TextWrapper(width=120, initial_indent=lead + "- ", subsequent_indent=cont,
                                    break_long_words=False, break_on_hyphens=False)
    else:
        wrap = textwrap.TextWrapper(width=120, initial_indent=cont, subsequent_indent=cont,
                                    break_long_words=False, break_on_hyphens=False)
    plan[first:e + 1] = wrap.wrap(flat)
    log.append(f"{tag}: ruling :{a}-{b}; plan bullet :{s + 1}-{e + 1}, rewrapped from :{first + 1}; "
               f"{len(old)} chars -> {len(new)}")
text = "\n".join(plan)
for tag, a, b in ITEMS:
    old, new = phrases(tag, a, b)
    assert len(re.findall(ws(new), text)) == 1, (tag, "the new phrase is not in the plan once")
before = set(orig)
long = [i + 1 for i, l in enumerate(plan) if len(l) > 120 and l not in before]
assert not long, ("changed lines over 120 characters", long)
print("\n".join(log))
print(f"plan sha256 before {sha(PLAN)[:12]}; lines {len(orig)} -> {len(plan)}")
sys.stdout.writelines(difflib.unified_diff([l + "\n" for l in orig], [l + "\n" for l in plan], PLAN, PLAN, n=1))
if APPLY:
    with open(PLAN, "w", encoding="utf-8") as fh:
        fh.write(text)
    print(f"applied; plan sha256 after {sha(PLAN)[:12]}")
else:
    print("dry run: nothing written")
