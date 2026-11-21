# rust-reviewer's U4s LAST-ROUND plant tool: one plant at a time, in place, in the builder's worktree (no copy of any
# source file is made; the original is restored by the reverse replacement and its sha256 checked against the baseline
# below, which is the last small round's final state).
#   python3 -I u4s-fix2-rust-plant.py plant <name>    refuses unless every target is at its baseline and the site is unique
#   python3 -I u4s-fix2-rust-plant.py restore <name>  refuses unless the planted text is there exactly once
#   python3 -I u4s-fix2-rust-plant.py status          every target's sha256 against its baseline
import hashlib
import pathlib
import sys

SPIKE = pathlib.Path("/home/user/Shamble-Frontlines/.claude/worktrees/wf_aa2cc979-004-1/spikes/netcode-proof")
BASE = {
    "crates/sim/src/grid.rs": "ede88bbedfcc0413bbc999006272a43ec76913a79fb87da674f011a7ab16aa50",
    "crates/room/src/lib.rs": "fd4db3af919701ef93674868db94984ad30ad7fac273209609e0f8b0516b3494",
    "check.mjs": "81181036709009e5fdc7f40567e9d3aed4cc1f30f1bffb484675c022554107bb",
    "check.test.mjs": "6c2fc9935d2cb2dd219ca1621f69b0fbcdf22bdf7d9dd84f764793e2649b84e5",
}

# name: (file, [(old, new), ...]); each old is replaced by its new exactly once, in order. ("append", suffix) appends.
BITES = {
    # G1, real tree, a pair the builder did not plant: npt-l1's -3 id in npt-sim's grid. Expected: deps FAIL.
    "R1-l1-id-in-sim": ("crates/sim/src/grid.rs", ("append",
        '\n#[cfg(feature = "bites")]\npub const L1_BITE: &str = "PM-0001-3/rtt-each-way";\n')),
    # G1 false-positive probe: npt-client's id inside an inline #[cfg(test)] module of npt-sim. Expected: deps PASS.
    "R2-test-mod-legit": ("crates/sim/src/grid.rs", ("append",
        '\n#[cfg(test)]\nmod bite_ids {\n    #[test]\n    fn names_the_client_bite() {\n'
        '        assert_eq!("PM-0001-17/client-rounding".len(), 26);\n    }\n}\n')),
    # G1: a module under cfg(not(test)) ships; it must be read. Expected: deps FAIL.
    "R3-not-test-mod": ("crates/sim/src/grid.rs", ("append",
        '\n#[cfg(not(test))]\nmod shipped {\n    pub const ID: &str = "PM-0001-17/client-rounding";\n}\n')),
    # G1 false-positive probe: the id quoted in a comment. Expected: deps PASS.
    "R4-comment": ("crates/sim/src/grid.rs", ("append",
        '\n// "PM-0001-17/client-rounding" is npt-client\'s id, not the grid\'s.\n')),
    # G1 revert: the grep reads the mask (literals blanked) instead of the code. Expected: self-tests fail.
    "R5-mask-not-code": ("check.mjs", [(
        "for (const m of code.matchAll(BITE_ID_LITERAL)) {", "for (const m of mask.matchAll(BITE_ID_LITERAL)) {")]),
    # G1a's proposed fix, tried on the committed tree: marks.json's whole id alphabet. Expected: deps PASS, self 48/48.
    "R6-wide-id": ("check.mjs", [(
        "export const BITE_ID = /^PM-\\d{4}-\\d+\\/[a-z0-9-]+$/;",
        "export const BITE_ID = /^PM-\\d{4}-\\d+\\/[A-Za-z0-9.-]+$/;")]),
    # G2, a lint and crate the builder did not plant: a bare URL in a public doc of npt-room. Expected: doc FAIL.
    "D1-bare-url": ("crates/room/src/lib.rs", ("append",
        "\n/// The room's reference: https://example.com/room.\npub const ROOM_REF: u8 = 0;\n")),
}


def sha(p):
    return hashlib.sha256(p.read_bytes()).hexdigest()


def status():
    bad = 0
    for rel, h in BASE.items():
        now = sha(SPIKE / rel)
        print(f"{rel}: {now[:16]} {'==' if now == h else '!='} baseline")
        bad += now != h
    return bad


def main():
    cmd = sys.argv[1]
    if cmd == "status":
        sys.exit(1 if status() else 0)
    name = sys.argv[2]
    rel, spec = BITES[name]
    path = SPIKE / rel
    text = path.read_bytes().decode("utf-8")
    if cmd == "plant":
        if status():
            sys.exit(f"REFUSED {name}: a target is not at its baseline sha256 (one plant at a time)")
        if isinstance(spec, tuple):
            planted = text + spec[1]
        else:
            planted = text
            for old, new in spec:
                if planted.count(old) != 1:
                    sys.exit(f"REFUSED {name}: the site occurs {planted.count(old)} times in {rel}")
                planted = planted.replace(old, new)
                if planted.count(new) != 1:
                    sys.exit(f"REFUSED {name}: the planted text would occur {planted.count(new)} times")
        path.write_bytes(planted.encode("utf-8"))
        print(f"PLANTED {name} in {rel}: sha256 {sha(path)[:16]}")
    elif cmd == "restore":
        if isinstance(spec, tuple):
            if not text.endswith(spec[1]):
                sys.exit(f"REFUSED restore {name}: {rel} does not end with the planted suffix")
            restored = text[: -len(spec[1])]
        else:
            restored = text
            for old, new in reversed(spec):
                if restored.count(new) != 1:
                    sys.exit(f"REFUSED restore {name}: the planted text occurs {restored.count(new)} times in {rel}")
                restored = restored.replace(new, old)
        path.write_bytes(restored.encode("utf-8"))
        h = sha(path)
        print(f"RESTORED {name} in {rel}: sha256 {h[:16]} {'==' if h == BASE[rel] else '!='} baseline")
        sys.exit(0 if h == BASE[rel] else 1)


main()
