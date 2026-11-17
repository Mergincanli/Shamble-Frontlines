"""TEST-FIRST's card cut (Mustafa, 2026-11-17 20:20 Amsterdam, history.md:2234-2282: "The resume card goes back to a
handoff: at most 200 lines. Everything older moves to the history file."): append the whole resume card, word for word,
to history.md inside a four-backtick fence under its own heading, after asserting the card's and history.md's sha256.
It does not touch the card (the main session then writes the new card with the Write tool). Refuses to run twice.
Prints the archive's first and last lines in history.md. Run from the repo root:
  python3 -I .checkpoints/netcode-proof-test/card_cut.py"""
import hashlib

CARD = ".checkpoints/netcode-proof-test.md"
H = ".checkpoints/netcode-proof-test/history.md"
CARD_SHA = "b8ac347309b34e307ec74919e088bb61af3ddcf08f1509ea727d9f6a0534cac7"
H_SHA = "82e281a9ca60c7fa2d1a4e157fc390e7c3016fa6352bf448aa00c0a0daad8dc4"
HEADING = "## Archive: the resume card before the TEST-FIRST cut (2026-11-17), moved whole by card_cut.py"

card = open(CARD, "rb").read()
hist = open(H, "rb").read()
assert hashlib.sha256(card).hexdigest() == CARD_SHA, "the card changed since its hash was read"
assert hashlib.sha256(hist).hexdigest() == H_SHA, "history.md changed since its hash was read"
text = card.decode("utf-8")
assert HEADING not in hist.decode("utf-8"), "already moved"
assert not any(line.startswith("````") for line in text.splitlines()), "a four-backtick line in the card"
assert text.endswith("\n")
n_card = text.count("\n")
block = (
    f"\n{HEADING}\n"
    f"- The card as it stood at 2026-11-17 about 22:50 UTC ({n_card} lines, sha256 {CARD_SHA}), word for word inside "
    "the fence below. A cite \"card, block X\" (or \"the card's ...\") written before the cut points here.\n\n"
    "````text\n" + text + "````\n"
)
new = hist.decode("utf-8") + block
open(H, "w", encoding="utf-8").write(new)
first = hist.decode("utf-8").count("\n") + 2
fence_open = first + 3
last = new.count("\n")
back = open(H, encoding="utf-8").read()
inner = back.split("````text\n", 1)[1].rsplit("````\n", 1)[0] if back.count("````text\n") == 1 else None
assert inner == text, "the archived copy differs from the card"
print(f"history.md: {hist.decode('utf-8').count(chr(10))} -> {last} lines; heading at :{first}; "
      f"the card's text at :{fence_open + 1}-{last - 1}; archived copy equals the card ({n_card} lines)")
