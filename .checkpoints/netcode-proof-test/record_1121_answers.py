"""Record Mustafa's chat message of 2026-11-21 about 14:31 UTC (15:31 Amsterdam) word for word at history.md's end: his
WAIT-PLAN (a), his U4G-Q1 answer ((b), split) and his ask to have F-B-COUNT explained in plain words. Refuses to run
twice. Run from the repo root: python3 -I .checkpoints/netcode-proof-test/record_1121_answers.py"""
H = ".checkpoints/netcode-proof-test/history.md"
HEAD = ("## Decision: Mustafa, 2026-11-21 about 14:31 UTC (15:31 Amsterdam), in this chat: WAIT-PLAN (a); U4G-Q1 (b), "
        "split; F-B-COUNT to be explained in plain words (word for word)")
WORDS = ("F-B-COUNT: Elaborate it to me in laymen terms on what it is and how it affects the project\n"
         "\n"
         "WAIT-PLAN A\n"
         "\n"
         "U4G-Q1: the advisor's: split (with two halves and with a review + merge each).")
hist = open(H, encoding="utf-8").read()
assert HEAD not in hist, "already recorded"
new = hist.rstrip("\n") + "\n\n" + HEAD + "\n" + WORDS + "\n"
open(H, "w", encoding="utf-8").write(new)
first = hist.rstrip("\n").count("\n") + 3
print(f"history.md now {new.count(chr(10))} lines; the record at :{first}-{new.count(chr(10))}")
