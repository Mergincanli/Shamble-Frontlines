# security-auditor scratch (4b-2 step 6): checks that each unpacked registry source equals its cached .crate archive
# byte for byte (cargo does not re-check unpacked sources before a build), and lists build scripts, proc macros,
# `links` keys and `unsafe` counts per crate. Reads only; writes nothing.
# Usage: python3 -I security-auditor-deps-4b2-src.py name-version [name-version ...]
import os, re, sys, tarfile, tomllib

HOME = os.path.expanduser("~")
CACHE = os.path.join(HOME, ".cargo/registry/cache/index.crates.io-1949cf8c6b5b557f")
SRC = os.path.join(HOME, ".cargo/registry/src/index.crates.io-1949cf8c6b5b557f")

UNSAFE = re.compile(rb"\bunsafe\b")


def check(nv):
    crate = os.path.join(CACHE, nv + ".crate")
    root = os.path.join(SRC, nv)
    if not os.path.exists(crate):
        return f"{nv}: no .crate cached"
    seen = set()
    mismatch, missing = [], []
    with tarfile.open(crate, "r:gz") as tar:
        for m in tar.getmembers():
            if not m.isfile():
                continue
            name = m.name
            if not name.startswith(nv + "/"):
                mismatch.append(f"outside-prefix:{name}")
                continue
            rel = name[len(nv) + 1:]
            seen.add(rel)
            path = os.path.join(root, rel)
            if not os.path.exists(path):
                missing.append(rel)
                continue
            if tar.extractfile(m).read() != open(path, "rb").read():
                mismatch.append(rel)
    extra = []
    unsafe_files = 0
    unsafe_hits = 0
    for dp, _, fs in os.walk(root):
        for f in fs:
            rel = os.path.relpath(os.path.join(dp, f), root)
            if rel not in seen and rel != ".cargo-ok":
                extra.append(rel)
            if f.endswith(".rs"):
                n = len(UNSAFE.findall(open(os.path.join(dp, f), "rb").read()))
                if n:
                    unsafe_files += 1
                    unsafe_hits += n
    man = tomllib.load(open(os.path.join(root, "Cargo.toml"), "rb"))
    pkg = man.get("package", {})
    build = pkg.get("build")
    has_build_rs = os.path.exists(os.path.join(root, "build.rs"))
    proc = man.get("lib", {}).get("proc-macro", False)
    links = pkg.get("links")
    return (f"{nv}: files={len(seen)} mismatch={mismatch or 0} missing={missing or 0} extra={extra or 0} | "
            f"build-key={build!r} build.rs={'yes' if has_build_rs else 'no'} proc-macro={proc} links={links} "
            f"unsafe: {unsafe_hits} in {unsafe_files} files")


for nv in sys.argv[1:]:
    print(check(nv))
