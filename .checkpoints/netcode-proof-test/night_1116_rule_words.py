"""Copy Mustafa's rule of 2026-11-09 into night-2026-11-16.md's header, word for word, from the card's verbatim record
("Mustafa, 2026-11-09 (verbatim, the adjusted version)"), and assert the words equal night-2026-11-10.md's copy. The
plan holds the placeholder @@RULE-2026-11-09@@ in its header paragraph; the script puts the quote there and rewraps
that one paragraph at 120 columns, changing no word. Refuses to run twice.
Run from the repo root:  python3 -I .checkpoints/netcode-proof-test/night_1116_rule_words.py"""
import hashlib
import textwrap

N = "/home/user/Shamble-Frontlines/.checkpoints/netcode-proof-test"
CARD = "/home/user/Shamble-Frontlines/.checkpoints/netcode-proof-test.md"
OLD = f"{N}/night-2026-11-10.md"
PLAN = f"{N}/night-2026-11-16.md"
PH = "@@RULE-2026-11-09@@"
START, END = '"From tonight, the night shift', 'with you during the day."'


def grab(path):
    with open(path, encoding="utf-8") as fh:
        text = fh.read()
    assert text.count(START) == 1, (path, text.count(START))
    a = text.index(START)
    b = text.index(END, a) + len(END)
    print(f"{path}: the quote at lines {text.count(chr(10), 0, a) + 1}-{text.count(chr(10), 0, b) + 1}")
    return " ".join(text[a:b].split())


words = grab(CARD)
assert words == grab(OLD), "the card's words differ from night-2026-11-10.md's copy"
with open(PLAN, encoding="utf-8") as fh:
    plan = fh.read()
assert plan.count(PH) == 1, plan.count(PH)
paras = plan.split("\n\n")
k = [i for i, p in enumerate(paras) if PH in p]
assert len(k) == 1
para = " ".join(paras[k[0]].split()).replace(PH, words)
paras[k[0]] = textwrap.fill(para, width=120, break_long_words=False, break_on_hyphens=False)
assert " ".join(paras[k[0]].split()) == para
plan = "\n\n".join(paras)
assert words in " ".join(plan.split())
with open(PLAN, "w", encoding="utf-8") as fh:
    fh.write(plan)
with open(PLAN, "rb") as fh:
    print(f"{PLAN}: sha256 {hashlib.sha256(fh.read()).hexdigest()}")
