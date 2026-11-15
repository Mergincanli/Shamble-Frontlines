"""Append the advisor's report words for BRAWL-1's review round 2 to u4r-report.md, word for word, under a new BRAWL-1
section: advisor-brawl1-r2-full.md lines 131-166 (its section "The report words", the quoted block; the file's sha256
and the block's bounds asserted). Only the two headings and the source line are the script's. The merge record follows
in the same section once the merge is done (the ruling's line 168). Refuses to run twice.
Run from the repo root:  python3 -I .checkpoints/netcode-proof-test/brawl1_report_words.py"""
import hashlib

N = "/home/user/Shamble-Frontlines/.checkpoints/netcode-proof-test"
RULING = f"{N}/advisor-brawl1-r2-full.md"
RULING_SHA = "64e361ab2342d8865c39bc354b2330ca7efd5ef9b9649a5bf75ac013bf3e8814"
REPORT = f"{N}/u4r-report.md"
HEAD = "## BRAWL-1 (the bots' leash toward the centre, from the client's own decoded position)"
with open(RULING, "rb") as fh:
    assert hashlib.sha256(fh.read()).hexdigest() == RULING_SHA
with open(RULING, encoding="utf-8") as fh:
    r = fh.read().split("\n")
assert r[126] == "## The report words" and r[128].startswith("For BRAWL-1's section of U4r's report"), (r[126], r[128])
assert r[130].startswith('"Review round 2 (workflow wf_19a5485a-e64;') and r[165].endswith('all three pins."'), (r[130], r[165])
assert r[166] == "" and r[167].startswith("Then the merge record"), (r[166], r[167])
words = r[130:166]
with open(REPORT, encoding="utf-8") as fh:
    text = fh.read()
assert HEAD not in text, "already appended"
add = ["", HEAD, "",
       "### Review round 2 and the advisor's round-2 ruling",
       "",
       f"_Copied by brawl1_report_words.py, word for word, from advisor-brawl1-r2-full.md lines 131-166 (sha256 "
       f"{RULING_SHA[:8]}...)._", "", *words, ""]
new = text.rstrip("\n") + "\n" + "\n".join(add)
assert "\n".join(words) in new
with open(REPORT, "w", encoding="utf-8") as fh:
    fh.write(new)
with open(REPORT, "rb") as fh:
    print(f"{REPORT}: +{len(add)} lines, sha256 {hashlib.sha256(fh.read()).hexdigest()}")
