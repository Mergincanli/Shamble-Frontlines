"""Copy the advisor's words for U4g's brief word for word from advisor-brawl1-r1-full.md (lines 279-288: the section
"U4g's brief", which merges ADV-PT-M1, NA-B1-2 (c) and TE-B1-5's kinematic bound) into u4g-brief-carry.md, the file U4g's
brief takes them from. The ruling's sha256 and the section's bounds are asserted; only the header is the script's.
Run from the repo root:  python3 -I .checkpoints/netcode-proof-test/u4g_brief_carry.py"""
import hashlib

N = "/home/user/Shamble-Frontlines/.checkpoints/netcode-proof-test"
RULING = f"{N}/advisor-brawl1-r1-full.md"
RULING_SHA = "86bec63bfffbd676efbc09e09c35a952e63abc7865e89b20b5aebec11bee75c5"
OUT = f"{N}/u4g-brief-carry.md"
with open(RULING, "rb") as fh:
    assert hashlib.sha256(fh.read()).hexdigest() == RULING_SHA
with open(RULING, encoding="utf-8") as fh:
    r = fh.read().split("\n")
assert r[278] == "## U4g's brief" and r[288] == "" and r[289] == "## The size", (r[278], r[288], r[289])
words = r[278:288]
out = ["# Carried into U4g's brief: the advisor's words from BRAWL-1's round-1 ruling", "",
       f"Copied by u4g_brief_carry.py, word for word, from advisor-brawl1-r1-full.md lines 279-288 (sha256 "
       f"{RULING_SHA}). Nothing below is the main session's.", "", *words, ""]
text = "\n".join(out)
assert "\n".join(words) in text
with open(OUT, "w", encoding="utf-8") as fh:
    fh.write(text)
with open(OUT, "rb") as fh:
    print(f"{OUT}: {len(out)} lines, sha256 {hashlib.sha256(fh.read()).hexdigest()}")
