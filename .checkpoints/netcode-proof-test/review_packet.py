"""Cut one review packet by script (Mustafa's USAGE-SAVINGS (a), 2026-11-11, verbatim in history.md: "The review
packet is cut by script and holds no conclusion of the builder or the main session. It ends with the ids of the carries
it left out. Reviewers stay free to read any file and run any test.").
  python3 -I .checkpoints/netcode-proof-test/review_packet.py <config.json>
The config (JSON) names only facts and sources: "title"; "out" (the packet) and "side_out" (a second file for the
diffs marked "side"); "heads" (SHAs and paths); "diffs" ({"label", "a", "b", "side"}: `git diff --no-index a b`);
"docs" ({"label", "path"} plus "lines": [first, last] or "heading": a regex; neither = the whole file); "cited"
({"path", "heading", "ref", "section"}: the sections of "path" that the section under "heading" cites, found by the
regex "ref"); "carries" ({"path", "include"}: entries whose text matches "include"; every other entry is listed by id
at the end). Builder notes, briefs, the checkpoint card and main-session summaries are never named in a config."""
import hashlib
import json
import os
import re
import subprocess
import sys

ROOT = "/home/user/Shamble-Frontlines"
with open(sys.argv[1], encoding="utf-8") as fh:
    cfg = json.load(fh)
FORBIDDEN = re.compile(r"(-build\.md|-brief[^/]*\.md|netcode-proof-test\.md|-report\.md|/history\.md)$")
# The exact paths that pass although FORBIDDEN matches them (Mustafa, 2026-11-12, verbatim in history.md): only files
# the advisor agent wrote as its ruling, added one at a time when a packet needs one, each addition noted in
# history.md. A file written by the main session or a builder never goes on this list.
ALLOWED = (".checkpoints/netcode-proof-test/advisor-u4e-brief.md",
           ".checkpoints/netcode-proof-test/advisor-u4d2-brief.md",  # addition 1 (history.md's note at its end)
           ".checkpoints/netcode-proof-test/advisor-u4r-brief.md")  # addition 2 (history.md's note at its end)
# Mustafa's own words reach a packet only from history.md, by line range, with "words_of_mustafa" set.
WORDS_OF_MUSTAFA = ".checkpoints/netcode-proof-test/history.md"


def lines_of(path):
    with open(os.path.join(ROOT, path), encoding="utf-8") as fh:
        return fh.read().split("\n")


def sha(path):
    with open(os.path.join(ROOT, path), "rb") as fh:
        return hashlib.sha256(fh.read()).hexdigest()


def section(lines, head):
    """1-based (first, last) of the section whose heading matches `head`, up to the next heading of its level or
    higher."""
    starts = [i for i, l in enumerate(lines) if re.match(head, l)]
    assert len(starts) == 1, (head, len(starts))
    s = starts[0]
    level = len(re.match(r"^(#+) ", lines[s]).group(1))
    e = next((i for i in range(s + 1, len(lines)) if re.match(r"^#{1,%d} " % level, lines[i])), len(lines))
    while e > s + 1 and not lines[e - 1].strip():
        e -= 1
    return s + 1, e


def block(label, path, first, last, lines):
    return [f"### {label}", f"_`{path}` lines {first}-{last} (sha256 {sha(path)[:12]}), as they are:_", "",
            "````text", *lines[first - 1:last], "````", ""]


out = [f"# Review packet: {cfg['title']}", "",
       "_Cut by `review_packet.py` from `" + os.path.basename(sys.argv[1]) + "`. It holds sources only: code diffs, "
       "the approved plan's lines, the advisor's rulings, the last round's findings and Mustafa's words, each copied "
       "by line range. It holds no conclusion of the builder or the main session (no builder notes, brief, checkpoint "
       "card or summary). You stay free to read any file and run any test._", "", "## Heads", ""]
out += [f"- {h}" for h in cfg["heads"]] + [""]
side = []
out += ["## What changed (the diffs)", ""]
for d in cfg["diffs"]:
    r = subprocess.run(["git", "diff", "--no-index", "--stat=160", d["a"], d["b"]], cwd=ROOT, capture_output=True,
                       text=True)
    assert r.returncode in (0, 1), r.stderr
    full = subprocess.run(["git", "diff", "--no-index", d["a"], d["b"]], cwd=ROOT, capture_output=True, text=True)
    assert full.returncode in (0, 1), full.stderr
    stat = r.stdout.rstrip("\n").split("\n")[-1] if r.stdout.strip() else "no change"
    if d.get("side"):
        side += [f"# {d['label']}: git diff --no-index {d['a']} {d['b']}", full.stdout]
        out += [f"### {d['label']}", f"_{stat}; the diff itself is in `{cfg['side_out']}` (`git diff --no-index "
                f"{d['a']} {d['b']}`)._", ""]
    else:
        out += [f"### {d['label']}", f"_`git diff --no-index {d['a']} {d['b']}`: {stat}._", "", "````diff",
                full.stdout.rstrip("\n"), "````", ""]
out += ["## What it is judged against", ""]
for doc in cfg["docs"]:
    assert (not FORBIDDEN.search(doc["path"]) or doc["path"] in ALLOWED
            or (doc.get("words_of_mustafa") and doc["path"] == WORDS_OF_MUSTAFA)), doc["path"]
    ls = lines_of(doc["path"])
    if "lines" in doc:
        first, last = doc["lines"]
    elif "heading" in doc:
        first, last = section(ls, doc["heading"])
    else:
        first, last = 1, len(ls) - (1 if ls and ls[-1] == "" else 0)
    out += block(doc["label"], doc["path"], first, last, ls)
for c in cfg.get("cited", []):
    ls = lines_of(c["path"])
    first, last = section(ls, c["heading"])
    refs = sorted(set(re.findall(c["ref"], "\n".join(ls[first - 1:last]))), key=lambda x: [int(n) for n in
                                                                                             re.findall(r"\d+", x)])
    for ref in refs:
        num = re.search(r"[\d.]+", ref).group(0)
        heads = [i for i, l in enumerate(ls) if re.match(c["section"].format(num=re.escape(num)), l)]
        if len(heads) != 1:
            out += [f"### {ref} of `{c['path']}`", f"_cited by the section above; no single heading found for it, "
                    f"so not copied._", ""]
            continue
        f2, l2 = section(ls, c["section"].format(num=re.escape(num)))
        out += block(f"{ref} of {os.path.basename(c['path'])} (cited by {c['label']})", c["path"], f2, l2, ls)
left = []
if "carries" in cfg:
    cp = cfg["carries"]["path"]
    ls = lines_of(cp)
    entries, cur = [], None
    for i, l in enumerate(ls):
        m = re.match(r"^(\d+)\. \*\*", l)
        if m:
            cur = [int(m.group(1)), i, i]
            entries.append(cur)
        elif cur and l.startswith("    "):
            cur[2] = i
        else:
            cur = None
    took = [e for e in entries if re.search(cfg["carries"]["include"], "\n".join(ls[e[1]:e[2] + 1]))]
    out += ["## The carries this packet holds", "",
            f"_Rule: every entry of `{cp}` whose text matches `{cfg['carries']['include']}`._", ""]
    for n, s, e in took:
        out += block(f"Carry {n}", cp, s + 1, e + 1, ls)
    left = sorted(e[0] for e in entries if e not in took)
if cfg.get("side_out"):
    with open(os.path.join(ROOT, cfg["side_out"]), "w", encoding="utf-8") as fh:
        fh.write("\n".join(side))
out += ["## The carries this packet left out", "",
        "Ids: " + (", ".join(str(n) for n in left) if left else "none") + ".", ""]
with open(os.path.join(ROOT, cfg["out"]), "w", encoding="utf-8") as fh:
    fh.write("\n".join(out))
size = os.path.getsize(os.path.join(ROOT, cfg["out"]))
print(f"{cfg['out']}: {len(out)} lines, {size} B, sha256 {sha(cfg['out'])[:12]}; carries left out: {len(left)}")
if cfg.get("side_out"):
    print(f"{cfg['side_out']}: {os.path.getsize(os.path.join(ROOT, cfg['side_out']))} B")
