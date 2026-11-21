"""Record Mustafa's chat message of 2026-11-21 (about 13:17 UTC, 14:17 Amsterdam: his personal account, which holds his
Advisor chat, is out of usage until Tuesday) word for word at history.md's end. Refuses to run twice. Run from the repo
root: python3 -I .checkpoints/netcode-proof-test/record_tuesday.py"""
H = ".checkpoints/netcode-proof-test/history.md"
HEAD = ("## Mustafa, 2026-11-21 about 13:17 UTC (14:17 Amsterdam), in this chat: his personal account is out of usage "
        "until Tuesday (word for word)")
WORDS = ("I am out of usage in my personal account, hence the reason I can't answer until Tuesday. \n"
         "Is it possible for you to continue working further, or is this a hard checkpoint?")
hist = open(H, encoding="utf-8").read()
assert HEAD not in hist, "already recorded"
new = hist.rstrip("\n") + "\n\n" + HEAD + "\n" + WORDS + "\n"
open(H, "w", encoding="utf-8").write(new)
first = hist.rstrip("\n").count("\n") + 3
print(f"history.md now {new.count(chr(10))} lines; the record at :{first}-{new.count(chr(10))}")
