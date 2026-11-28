"""C62H-BOUND's insert script (the build brief c62h-bound-brief.md section 2, sha256 7936074c...; Mustafa's
C62H-BOUND-YES (a), 2026-11-15 20:50 Amsterdam, history.md:2163-2175). It puts the frozen texts of
c62h-bound-texts-v3.md, never retyped, into <root>/spikes/netcode-proof/marks/:
  --part 1 (commit 1, B-1): SERIES.md's `(18)` header becomes `(19)`; the tick.s.snap_link_lost_count row goes in.
  --part 3 (commit 3): marks.json's two checks and one bite; SERIES.md's `(36)` header becomes `(38)`, the two figure
  rows go in and the tick.s.snap_incomplete row is replaced; README.md's sentence goes in and lines 67-77 are refilled.
First, every time: texts v3's and the freeze's sha256; each block, parsed with c62h_bound_freeze.py's own regex, has
the sha256 the freeze gives it; each target file has its LANE_BASE sha256 (for part 3, SERIES.md has part 1's). Every
anchor must match exactly once. Each new file is built in memory and written only if its sha256 equals the brief's
expected one (part 3 writes its three files only if all three equal), so a second run, a moved anchor or a different
result writes nothing and exits 1. It prints each file's sha256 before and after.
Run from the repository root:
  python3 -I .checkpoints/netcode-proof-test/c62h_bound_insert.py <root> --part 1|3"""
import hashlib
import json
import os
import re
import sys
import textwrap

N = "/home/user/Shamble-Frontlines/.checkpoints/netcode-proof-test"
TEXTS_SHA = "3f21c9b7e5d5116db7cae7c0d8d9a4011d2ab9797c75b907f00554f0996844eb"
FREEZE_SHA = "9cb8be172eaa73608ecee5b46e2dc5cfd44e8c2b3a26ca7a74bffb5ef4e82950"
BASE = {"marks.json": "9fb5be578b27d4c378a890d297c5ba636af0ac239bb6d50eae157ed44884a3c5",
        "SERIES.md": "3411eee7f2b9398198203036caf0c38e58829a2c1676fc077c53473c474ba6e5",
        "README.md": "b49f3441f86721ab8d74818d0361d5b9021ec91170ee4c0e48facc9f0736577d"}
SERIES_P1 = "89295b57d5c9ceec4e9790d8466b1ea5981215b242865e93f68872bb529774fc"
AFTER = {"marks.json": "c1935a5b996fb0aaff82b0b3c1c604255e1534f6ca7e8d2e55a9f472cd3b22a9",
         "SERIES.md": "6898e119109a794a7779ad0395301e48a461848b60cfdc16431ec64faad87460",
         "README.md": "f3c3c804c4e4d078ca07461f956eb60bc96e74eb4ed507bf28f652383a5742fe"}
BLOCK_RE = r"^<!-- BEGIN (\S+) -->\n```\n(.*?)\n```\n<!-- END \1 -->$"  # c62h_bound_freeze.py's regex


def sha(b):
    return hashlib.sha256(b).hexdigest()


def stop(msg):
    print(f"c62h_bound_insert: STOP, nothing written: {msg}")
    sys.exit(1)


def one(idx, what):
    if len(idx) != 1:
        stop(f"{what} matches {len(idx)} times, not exactly once")
    return idx[0]


def frozen_blocks():
    v3raw = open(f"{N}/c62h-bound-texts-v3.md", "rb").read()
    fz = open(f"{N}/c62h-bound-freeze.txt", "rb").read()
    if sha(v3raw) != TEXTS_SHA:
        stop(f"c62h-bound-texts-v3.md sha256 {sha(v3raw)}, not {TEXTS_SHA}")
    if sha(fz) != FREEZE_SHA:
        stop(f"c62h-bound-freeze.txt sha256 {sha(fz)}, not {FREEZE_SHA}")
    print(f"texts v3 sha256 {sha(v3raw)} (as the brief gives); freeze sha256 {sha(fz)} (as the brief gives)")
    blocks = {m.group(1): m.group(2) for m in re.finditer(BLOCK_RE, v3raw.decode("utf-8"), re.M | re.S)}
    want = {}
    for line in fz.decode("utf-8").split("\n"):
        if line and not line.startswith("#"):
            name, _, h = line.rpartition(" ")
            want[name] = h
    if want.get("c62h-bound-texts-v3.md") != TEXTS_SHA:
        stop("the freeze does not record texts v3's sha256")
    names = [n for n in want if n not in ("c62h-bound-paste.md (as posted)", "c62h-bound-texts-v3.md")]
    if len(names) != 8:
        stop(f"the freeze names {len(names)} blocks, not 8")
    for n in names:
        if n not in blocks:
            stop(f"texts v3 has no block {n}")
        if sha(blocks[n].encode("utf-8")) != want[n]:
            stop(f"block {n} sha256 {sha(blocks[n].encode('utf-8'))}, not the freeze's {want[n]}")
        print(f"block {n}: sha256 {want[n]} (as the freeze gives)")
    return blocks


def part1_series(text, b):
    lines = text.split("\n")
    h = one([i for i, l in enumerate(lines) if l == "## Series read only by figures (18)"], "the (18) header")
    r = one([i for i, l in enumerate(lines) if l.startswith("| `tick.s.relevant_count` |")], "the relevant_count row")
    lines[h] = "## Series read only by figures (19)"
    lines.insert(r + 1, b["series-row-tick.s.snap_link_lost_count"])
    return "\n".join(lines)


def part3_marks(text, b):
    lines = text.split("\n")
    i = one([k for k, l in enumerate(lines)
             if l == '          "name": "L1 P0 loses no group (no client-tick left incomplete)",'],
            "the L1 P0 loses-no-group check's name line")
    j = next((k for k in range(i + 1, len(lines)) if lines[k] == "        }"), None)
    if j is None or lines[j + 1] != "      ]," or lines[j + 2] != '      "report": [':
        stop("the loses-no-group check does not end with `        }` just before `      ],` and `      \"report\": [`")
    k = one([n for n, l in enumerate(lines) if l == '          "what": "a client writer that files a tick it decoded '
             'whole as lost: null and incomplete 1"'], "the whole-as-lost bite's what line")
    if lines[k + 1] != "        }":
        stop("the whole-as-lost entry does not end with `        }` just after its what line")
    if not k > j:
        stop("the bite is not after the check")
    lines[k + 1] = "        },"
    lines[k + 2:k + 2] = b["bite-object"].split("\n")
    lines[j] = "        },"
    lines[j + 1:j + 1] = b["check-objects"].split("\n")
    new = "\n".join(lines)
    exp = json.loads(text)
    mark = exp["marks"][one([n for n, m in enumerate(exp["marks"]) if m.get("id") == "PM-0001-17"], "PM-0001-17")]
    ci = one([n for n, c in enumerate(mark["checks"])
              if c.get("name") == "L1 P0 loses no group (no client-tick left incomplete)"], "the check in the JSON")
    mark["checks"][ci + 1:ci + 1] = json.loads("[" + b["check-objects"] + "]")
    bi = one([n for n, x in enumerate(mark["bites"]) if x.get("id") == "PM-0001-17/whole-as-lost"], "the bite in the JSON")
    mark["bites"][bi + 1:bi + 1] = [json.loads(b["bite-object"])]
    if json.loads(new) != exp:
        stop("json.loads of the result is not the base's with exactly the two checks and the bite added")
    print("marks.json: json.loads of the result equals the base's with exactly the two checks and the bite added")
    return new


def part3_series(text, b):
    lines = text.split("\n")
    h = one([i for i, l in enumerate(lines) if l == "## Figures (36): computations npt-judge carries in code"],
            "the (36) header")
    s = one([i for i, l in enumerate(lines) if l.startswith("| `session_memory_growth_count` |")],
            "the session_memory_growth_count row")
    if not lines[s + 1].startswith("| `sync_cost_ratio` |"):
        stop("the line after session_memory_growth_count's row does not start with the sync_cost_ratio row")
    t = one([i for i, l in enumerate(lines) if l.startswith("| `tick.s.snap_incomplete` |")], "the snap_incomplete row")
    if not t > s:
        stop("the snap_incomplete row is not after the figures' rows")
    lines[t] = b["series-row-tick.s.snap_incomplete"]
    lines[s + 1:s + 1] = [b["series-row-snap_lost_unexplained_count"],
                          b["series-row-snap_whole_despite_link_loss_count"]]
    lines[h] = "## Figures (38): computations npt-judge carries in code"
    return "\n".join(lines)


def part3_readme(text, b):
    lines = text.split("\n")
    a = one([i for i, l in enumerate(lines) if "never the sentinel. That covers" in l], "README's sentinel line")
    e = one([i for i, l in enumerate(lines) if l.startswith("  - `replay_hash`")], "README's replay_hash bullet")
    if (a + 1, e) != (67, 77):
        stop(f"the refill range is lines {a + 1} to {e}, not 67 to 77")
    sentence = b["readme-sentence"]
    if len(sentence.split()) != 73:
        stop(f"the sentence has {len(sentence.split())} words, not 73")
    if lines[a].count("never the sentinel. ") != 1:
        stop("`never the sentinel. ` is not on README.md:67 exactly once")
    lines[a] = lines[a].replace("never the sentinel. ", "never the sentinel. " + sentence + " ", 1)
    words = " ".join(" ".join(lines[a:e]).split())
    filled = textwrap.fill(words, width=120, initial_indent="    ", subsequent_indent="    ",
                           break_long_words=False, break_on_hyphens=False).split("\n")
    if max(len(l) for l in filled) > 120:
        stop("a refilled line is over 120 characters")
    lines[a:e] = filled
    print(f"README.md: lines 67 to 77 refilled into {len(filled)} lines (the longest {max(len(l) for l in filled)})")
    return "\n".join(lines)


def main():
    if len(sys.argv) != 4 or sys.argv[2] != "--part" or sys.argv[3] not in ("1", "3"):
        print(__doc__)
        sys.exit(2)
    root, part = os.path.abspath(sys.argv[1]), sys.argv[3]
    d = os.path.join(root, "spikes", "netcode-proof", "marks")
    if not os.path.isdir(d):
        stop(f"{d} is not a directory")
    print(f"c62h_bound_insert: part {part} on {d}")
    blocks = frozen_blocks()
    files = ["SERIES.md"] if part == "1" else ["marks.json", "SERIES.md", "README.md"]
    old = {f: open(os.path.join(d, f), "rb").read() for f in files}
    before = dict(BASE) if part == "1" else dict(BASE, **{"SERIES.md": SERIES_P1})
    for f in files:
        print(f"{f} before: sha256 {sha(old[f])}")
        if sha(old[f]) != before[f]:
            stop(f"{f} sha256 is not {before[f]} (a second run, or the file moved)")
    text = {f: old[f].decode("utf-8") for f in files}
    if part == "1":
        new = {"SERIES.md": part1_series(text["SERIES.md"], blocks)}
        want = {"SERIES.md": SERIES_P1}
    else:
        new = {"marks.json": part3_marks(text["marks.json"], blocks),
               "SERIES.md": part3_series(text["SERIES.md"], blocks),
               "README.md": part3_readme(text["README.md"], blocks)}
        want = AFTER
    for f in files:
        got = sha(new[f].encode("utf-8"))
        if got != want[f]:
            stop(f"{f} would be sha256 {got}, not the brief's expected {want[f]}")
    for f in files:
        with open(os.path.join(d, f), "wb") as fh:
            fh.write(new[f].encode("utf-8"))
    for f in files:
        print(f"{f} after: sha256 {sha(open(os.path.join(d, f), 'rb').read())} (the brief's expected {want[f]})")
    print(f"c62h_bound_insert: part {part} written")


main()
