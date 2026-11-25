"""rust-reviewer r1 scratch: section sizes of a .wasm, per-function body sizes by name, and a base/head diff.
Usage: python3 -I rr_wasm_sizes.py <base.wasm> <head.wasm>
Reads only the two files it is given; writes nothing."""
import gzip
import sys
import zlib


def leb(data, at):
    result = shift = 0
    while True:
        byte = data[at]
        at += 1
        result |= (byte & 0x7F) << shift
        shift += 7
        if byte < 0x80:
            return result, at


def name_of(data, at):
    length, at = leb(data, at)
    return data[at:at + length].decode("utf-8", "replace"), at + length


def parse(path):
    data = open(path, "rb").read()
    assert data[:4] == b"\0asm", path
    at = 8
    sections = []
    imported_funcs = 0
    bodies = []
    names = {}
    exports = []
    data_bytes = 0
    while at < len(data):
        sid = data[at]
        size, body_at = leb(data, at + 1)
        end = body_at + size
        label = {0: "custom", 1: "type", 2: "import", 3: "function", 4: "table", 5: "memory", 6: "global",
                 7: "export", 8: "start", 9: "element", 10: "code", 11: "data", 12: "datacount"}.get(sid, str(sid))
        if sid == 0:
            cname, inner = name_of(data, body_at)
            label = "custom:" + cname
            if cname == "name":
                p = inner
                while p < end:
                    sub = data[p]
                    sub_size, sp = leb(data, p + 1)
                    if sub == 1:
                        count, q = leb(data, sp)
                        for _ in range(count):
                            idx, q = leb(data, q)
                            fname, q = name_of(data, q)
                            names[idx] = fname
                    p = sp + sub_size
        elif sid == 2:
            count, p = leb(data, body_at)
            for _ in range(count):
                _, p = name_of(data, p)
                _, p = name_of(data, p)
                kind = data[p]
                p += 1
                if kind == 0:
                    imported_funcs += 1
                    _, p = leb(data, p)
                elif kind == 1:
                    p += 1
                    flags, p = leb(data, p)
                    _, p = leb(data, p)
                    if flags & 1:
                        _, p = leb(data, p)
                elif kind == 2:
                    flags, p = leb(data, p)
                    _, p = leb(data, p)
                    if flags & 1:
                        _, p = leb(data, p)
                elif kind == 3:
                    p += 2
        elif sid == 7:
            count, p = leb(data, body_at)
            for _ in range(count):
                ename, p = name_of(data, p)
                kind = data[p]
                idx, p = leb(data, p + 1)
                exports.append((ename, kind, idx))
        elif sid == 10:
            count, p = leb(data, body_at)
            for i in range(count):
                bsize, q = leb(data, p)
                bodies.append((imported_funcs + i, q - p + bsize))
                p = q + bsize
        elif sid == 11:
            data_bytes = size
        sections.append((label, end - at))
        at = end
    return data, sections, bodies, names, exports


def report(path):
    data, sections, bodies, names, exports = parse(path)
    print(f"{path}: {len(data)} bytes raw, gzip -9 {len(gzip.compress(data, 9))}, deflate {len(zlib.compress(data, 9))}")
    for label, size in sections:
        print(f"  section {label:<28} {size:>7}")
    return data, sections, bodies, names, exports


def main():
    base = report(sys.argv[1])
    head = report(sys.argv[2])
    bsec = {}
    for label, size in base[1]:
        bsec[label] = bsec.get(label, 0) + size
    hsec = {}
    for label, size in head[1]:
        hsec[label] = hsec.get(label, 0) + size
    print("section deltas (head - base):")
    for label in sorted(set(bsec) | set(hsec)):
        delta = hsec.get(label, 0) - bsec.get(label, 0)
        if delta:
            print(f"  {label:<28} {delta:+d}")
    def by_name(parsed):
        _, _, bodies, names, _ = parsed
        out = {}
        for idx, size in bodies:
            key = names.get(idx, f"#{idx}")
            out[key] = out.get(key, 0) + size
        return out
    bf, hf = by_name(base), by_name(head)
    print(f"functions: base {len(base[2])}, head {len(head[2])}; names known base {len(base[3])}, head {len(head[3])}")
    rows = []
    for key in set(bf) | set(hf):
        delta = hf.get(key, 0) - bf.get(key, 0)
        if delta:
            rows.append((delta, key, bf.get(key, 0), hf.get(key, 0)))
    rows.sort(key=lambda row: -abs(row[0]))
    print("function body deltas (head - base), largest first:")
    for delta, key, b, h in rows[:60]:
        print(f"  {delta:+6d}  base {b:>6} head {h:>6}  {key[:150]}")
    print(f"  sum of body deltas: {sum(row[0] for row in rows):+d} over {len(rows)} functions")
    print("exports head:", [e[0] for e in head[4]])
    print("exports base:", [e[0] for e in base[4]])


main()
