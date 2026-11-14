# Replace-once editor for the U4s conditions (netcode-architect, 2026-11-09).
# Usage: python3 -I u4s-cond-edit.py <target> <spec> dry|apply
# Spec blocks: "@@@ <id>", "<<<", old lines, "===", new lines, ">>>".
# Refuses (and writes nothing) unless every old text occurs exactly once, applied in order.
import hashlib
import pathlib
import sys


def parse(spec):
    edits, lines, i = [], spec.split("\n"), 0
    while i < len(lines):
        if lines[i].startswith("@@@ "):
            eid = lines[i][4:].strip()
            i += 1
            if lines[i] != "<<<":
                sys.exit(f"BAD SPEC {eid}: expected <<<")
            i += 1
            old = []
            while lines[i] != "===":
                old.append(lines[i])
                i += 1
            i += 1
            new = []
            while lines[i] != ">>>":
                new.append(lines[i])
                i += 1
            i += 1
            edits.append((eid, "\n".join(old), "\n".join(new)))
        else:
            i += 1
    return edits


def main():
    target, specf, mode = sys.argv[1], sys.argv[2], sys.argv[3]
    path = pathlib.Path(target)
    text = path.read_text(encoding="utf-8")
    print("before", hashlib.sha256(text.encode("utf-8")).hexdigest())
    edits = parse(pathlib.Path(specf).read_text(encoding="utf-8"))
    for eid, old, new in edits:
        n = text.count(old)
        if n != 1:
            sys.exit(f"REFUSED {eid}: old text occurs {n} times; nothing written")
        text = text.replace(old, new, 1)
        print("ok", eid)
    digest = hashlib.sha256(text.encode("utf-8")).hexdigest()
    if mode == "apply":
        path.write_text(text, encoding="utf-8")
        print("after", digest, "edits", len(edits))
    else:
        print("dry run; would be", digest, "edits", len(edits))


main()
