# security-auditor scratch (4b-2 step 6 dependency check): compares three lockfiles and checks every registry
# package's checksum against the cached .crate file and the cached sparse-index entry. Reads only; writes nothing.
# Usage: python3 -I security-auditor-deps-4b2-lock.py <base Cargo.lock> <step0 Cargo.lock> <batch Cargo.lock>
import hashlib, json, os, sys, tomllib

REG = "registry+https://github.com/rust-lang/crates.io-index"
HOME = os.path.expanduser("~")
CACHE = os.path.join(HOME, ".cargo/registry/cache/index.crates.io-1949cf8c6b5b557f")
INDEX = os.path.join(HOME, ".cargo/registry/index/index.crates.io-1949cf8c6b5b557f/.cache")


def load(path):
    with open(path, "rb") as f:
        data = tomllib.load(f)
    pkgs = {}
    for p in data["package"]:
        key = (p["name"], p["version"])
        if key in pkgs:
            raise SystemExit(f"duplicate {key} in {path}")
        pkgs[key] = p
    return data.get("version"), pkgs


def index_path(name):
    n = name.lower()
    if len(n) == 1:
        return os.path.join(INDEX, "1", n)
    if len(n) == 2:
        return os.path.join(INDEX, "2", n)
    if len(n) == 3:
        return os.path.join(INDEX, "3", n[0], n)
    return os.path.join(INDEX, n[0:2], n[2:4], n)


def index_entry(name, version):
    path = index_path(name)
    if not os.path.exists(path):
        return None
    raw = open(path, "rb").read()
    # header: cache version byte, u32 index version, etag/revision string NUL-terminated; then (version NUL json NUL)*
    rest = raw[5:]
    _, _, rest = rest.partition(b"\0")
    parts = rest.split(b"\0")
    for i in range(0, len(parts) - 1, 2):
        if parts[i].decode() == version:
            return json.loads(parts[i + 1])
    return "missing-version"


def main():
    base_v, base = load(sys.argv[1])
    s0_v, step0 = load(sys.argv[2])
    b_v, batch = load(sys.argv[3])
    print(f"lock format versions: base {base_v}, step0 {s0_v}, batch {b_v}")
    print(f"packages: base {len(base)}, step0 {len(step0)}, batch {len(batch)}")

    sources = sorted({p.get("source", "<path>") for p in batch.values()})
    print("sources in batch:", sources)

    added = sorted(set(batch) - set(base))
    removed = sorted(set(base) - set(batch))
    print(f"\nadded vs base ({len(added)}):")
    for k in added:
        p = batch[k]
        print(f"  {k[0]} {k[1]} source={'reg' if p.get('source') == REG else p.get('source', '<path>')}")
    print(f"removed vs base ({len(removed)}): {removed}")
    for k in sorted(set(base) & set(batch)):
        if base[k] != batch[k]:
            print(f"  changed vs base: {k}: deps {base[k].get('dependencies')} -> {batch[k].get('dependencies')}")

    reg_added = [k for k in added if batch[k].get("source") == REG]
    print(f"registry packages added: {len(reg_added)}")

    print("\nagainst step 0's lockfile:")
    only_b = sorted(set(batch) - set(step0))
    only_s = sorted(set(step0) - set(batch))
    print(f"  only in batch: {only_b}")
    print(f"  only in step0: {only_s}")
    for k in sorted(set(batch) & set(step0)):
        a, b = step0[k], batch[k]
        if a != b:
            diff = {f: (a.get(f), b.get(f)) for f in set(a) | set(b) if a.get(f) != b.get(f)}
            print(f"  differs {k}: {diff}")
    same_reg = [k for k in batch if batch[k].get("source") == REG and step0.get(k) == batch[k]]
    print(f"  registry packages identical to step 0 (all fields): {len(same_reg)} of "
          f"{sum(1 for p in batch.values() if p.get('source') == REG)}")

    print("\nchecksums (lock vs cached .crate sha256 vs cached index cksum; yanked as of the cached index):")
    bad = 0
    for k in sorted(batch):
        p = batch[k]
        if p.get("source") is None:
            continue
        if p.get("source") != REG:
            print(f"  NON-CRATES.IO SOURCE {k}: {p.get('source')}")
            bad += 1
            continue
        lock_ck = p.get("checksum")
        crate = os.path.join(CACHE, f"{k[0]}-{k[1]}.crate")
        file_ck = hashlib.sha256(open(crate, "rb").read()).hexdigest() if os.path.exists(crate) else None
        ent = index_entry(k[0], k[1])
        idx_ck = ent.get("cksum") if isinstance(ent, dict) else ent
        yanked = ent.get("yanked") if isinstance(ent, dict) else None
        ok = lock_ck == idx_ck and (file_ck is None or file_ck == lock_ck)
        if not ok:
            bad += 1
        mark = "OK " if ok else "BAD"
        print(f"  {mark} {k[0]:20} {k[1]:10} lock={lock_ck[:12]} file={(file_ck or 'not-cached')[:12]} "
              f"index={(idx_ck or 'none')[:12]} yanked={yanked} new={'yes' if k in reg_added else 'no'}")
    print(f"checksum problems: {bad}")


main()
