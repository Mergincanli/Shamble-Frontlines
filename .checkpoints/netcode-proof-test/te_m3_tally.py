"""TE-M3's tally (advisor-te-m3.md AM-1): for one te_m3_loop.sh tag in mode `one`, every run's log
(.gate/te-m3/<tag>-<loop>-<i>.log) and its exit line in the loop's log; prints the run count, the count of the test's
own lines (exactly one per run), every (pump runs, at or after the deadline, S's poll_shutdown calls) triple with its
count, how many runs had S calls = runs + 1, and every exit code. Reads only; writes nothing.
Run from the repo root:  python3 -I .checkpoints/netcode-proof-test/te_m3_tally.py <tag>"""
import collections
import glob
import re
import sys

tag = sys.argv[1]
D = "/home/user/Shamble-Frontlines/.gate/te-m3"
LINE = re.compile(r"the pump ran (\d+) times during the wait \((\d+) at or after the deadline\), S's poll_shutdown "
                  r"(\d+) times")
files = sorted(glob.glob(f"{D}/{tag}-[0-9]*-[0-9]*.log"))
triples = collections.Counter()
own = 0
per_run_bad = []
for f in files:
    hits = LINE.findall(open(f, encoding="utf-8", errors="replace").read())
    own += len(hits)
    if len(hits) != 1:
        per_run_bad.append((f, len(hits)))
    for h in hits:
        triples[tuple(int(x) for x in h)] += 1
exits = collections.Counter(re.findall(r" run \d+ exit (\d+) ", open(f"{D}/{tag}.log", encoding="utf-8").read()))
print(f"tag {tag}: {len(files)} run logs; {own} own lines; runs whose log has not exactly one: {len(per_run_bad)}")
for f, n in per_run_bad[:10]:
    print(f"  {f}: {n}")
print("exit codes (loop log):", dict(sorted(exits.items())))
for t, n in sorted(triples.items()):
    print(f"  (runs {t[0]}, at or after {t[1]}, S calls {t[2]}): {n}")
plus1 = sum(n for t, n in triples.items() if t[2] == t[0] + 1)
print(f"runs with S calls = runs + 1: {plus1}")
