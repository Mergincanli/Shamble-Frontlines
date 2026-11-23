# security-auditor, OPT-PATH reading, 2026-11-23: set or show a POSIX ACL through its xattr (setfacl is not
# installed here). Only for scratch folders under .gate/ci-wasm/sa-opt-path/work/.
# Usage: python3 -B security-auditor-opt-acl.py set <path> access|default <spec>
#        python3 -B security-auditor-opt-acl.py show <path> access|default
# spec: comma list such as u::rwx,u:1001:rwx,g::r-x,m::rwx,o::r-x
import os
import struct
import sys

WORK = "/home/user/Shamble-Frontlines/.gate/ci-wasm-wt/.gate/ci-wasm/sa-opt-path/work/"
TAGS = {"u": (0x01, 0x02), "g": (0x04, 0x08), "m": (0x10, None), "o": (0x20, None)}
NAMES = {0x01: "user::", 0x02: "user:", 0x04: "group::", 0x08: "group:", 0x10: "mask::", 0x20: "other::"}
UNDEF = 0xFFFFFFFF
XATTR = {"access": "system.posix_acl_access", "default": "system.posix_acl_default"}


def perm(s):
    return (4 if s[0] == "r" else 0) | (2 if s[1] == "w" else 0) | (1 if s[2] == "x" else 0)


def pstr(p):
    return ("r" if p & 4 else "-") + ("w" if p & 2 else "-") + ("x" if p & 1 else "-")


def build(spec):
    ents = []
    for item in spec.split(","):
        kind, qual, ps = item.split(":")
        obj, named = TAGS[kind]
        ents.append((obj, perm(ps), UNDEF) if qual == "" else (named, perm(ps), int(qual)))
    ents.sort(key=lambda e: (e[0], e[2]))
    return struct.pack("<I", 2) + b"".join(struct.pack("<HHI", t, p, i) for t, p, i in ents)


def show(raw):
    out = []
    for off in range(4, len(raw), 8):
        t, p, i = struct.unpack_from("<HHI", raw, off)
        out.append(NAMES[t] + ("" if i == UNDEF else str(i) + ":") + pstr(p))
    return " ".join(out)


op, path, which = sys.argv[1], sys.argv[2], sys.argv[3]
if not os.path.realpath(path).startswith(WORK):
    sys.exit("refused: not under the scratch work folder")
if op == "set":
    os.setxattr(path, XATTR[which], build(sys.argv[4]), follow_symlinks=False)
    print("      | %s ACL set on %s: %s; mode now %o" % (which, path, sys.argv[4], os.lstat(path).st_mode & 0o7777))
else:
    try:
        raw = os.getxattr(path, XATTR[which], follow_symlinks=False)
        print("      | %s ACL of %s: %s; mode %o" % (which, path, show(raw), os.lstat(path).st_mode & 0o7777))
    except OSError as err:
        print("      | %s ACL of %s: none (%s)" % (which, path, err.strerror))
