# rust-reviewer's U4s plant tool: one bite at a time, in place, in the builder's worktree (no copy of any source file
# is made; the original is restored by the reverse replacement and its sha256 checked against the baseline below).
#   python3 -I u4s-rust-plant.py plant <name>    refuses unless the file is at its baseline sha256 and the site is unique
#   python3 -I u4s-rust-plant.py restore <name>  refuses unless the planted text is there exactly once; checks sha256
#   python3 -I u4s-rust-plant.py status          every target's sha256 against its baseline
import hashlib
import pathlib
import sys

SPIKE = pathlib.Path("/home/user/Shamble-Frontlines/.claude/worktrees/wf_aa2cc979-004-1/spikes/netcode-proof")
BASE = {
    "crates/sim/src/grid.rs": "2446152df3c8e81c56a02baa3562b43febd17639b0dd80ce149c6294e3bf0216",
    "crates/sim/src/history.rs": "d4351e0841255e921ab8c329f2a6203c5534fdad4fd9819b40016e7a580556f3",
    "crates/sim/src/ticks.rs": "b5bbcbf20bc56d409d601df83b038be46aa5e8746a5201fd2111ec8fbf579dbb",
    "crates/room/src/lib.rs": "fd4db3af919701ef93674868db94984ad30ad7fac273209609e0f8b0516b3494",
    "check.mjs": "74f75b74b1534b21473bc0484074bbe84e3e46868dacafa02cc2f6fdef989a11",
}

# name: (file, old, new); old is replaced by new exactly once. ("append", suffix) appends.
BITES = {
    # The plan's two (u4-build-plan.md §5 U4s).
    "G1-floor": ("crates/sim/src/grid.rs",
                 "field.fit(scaled.round() as i64)", "field.fit(scaled.floor() as i64)"),
    "H1-one-short-capacity": ("crates/sim/src/history.rs",
                              "let capacity = capacity_ticks as usize;", "let capacity = capacity_ticks as usize - 1;"),
    # Mine: the ring answers one short at get (the oldest held tick treated as evicted).
    "H2-one-short-get": ("crates/sim/src/history.rs",
                         "if age >= self.slots.len() {", "if age + 1 >= self.slots.len() {"),
    # check.mjs self-test bites: the tick grep.
    "C1-call-counts-as-derived": ("check.mjs",
                                  r"|| /\bticks_for_ms\s*\(/.test(init))) {",
                                  r"|| /\(/.test(init) || /\bticks_for_ms\s*\(/.test(init))) {"),
    "C2-tick-hz-anywhere-first": ("check.mjs",
                                  "if (f.startsWith(TICK_HZ_HOME) && home === null) {", "if (home === null) {"),
    "C3-static-not-read": ("check.mjs",
                           r"`\\b(?:const|static)\\s+(?:mut\\s+)?(TICK_HZ|", r"`\\b(?:const)\\s+(?:mut\\s+)?(TICK_HZ|"),
    "C4-tick-comments-unmasked": ("check.mjs",
                                  "    const { mask } = lexRust(read(f));\n    for (const m of mask.matchAll(TICK_ITEM)) {",
                                  "    const mask = read(f);\n    for (const m of mask.matchAll(TICK_ITEM)) {"),
    # check.mjs self-test bites: the float grep.
    "C5-exponent-missed": ("check.mjs",
                           r"|\.(?![\w.])|[eE][+-]?\d[\d_]*|_*f(?:32|64)\b)/g;", r"|\.(?![\w.])|_*f(?:32|64)\b)/g;"),
    "C6-room-tests-read": ("check.mjs",
                           'if (!f.startsWith(FLOAT_FREE) || !f.endsWith(".rs")) continue;',
                           'if (!f.startsWith("crates/room/") || !f.endsWith(".rs")) continue;'),
    "C7-lookbehind-dropped": ("check.mjs",
                              r"/\bf(?:32|64)\b|(?<![\w.])\d[\d_]*(?:", r"/\bf(?:32|64)\b|\d[\d_]*(?:"),
    "C8-float-literals-missed": ("check.mjs",
                                 "const FLOAT_TOKEN =\n  /\\bf(?:32|64)\\b|",
                                 "const FLOAT_TOKEN =\n  /\\bf(?:32|64)\\b/g; const X =\n  /"),
    # check.mjs: each grep unhooked from the deps step.
    "C9-tick-grep-unhooked": ("check.mjs",
                              "    ...unsafeProblems(files, read),\n    ...tickConstantProblems(files, read),\n",
                              "    ...unsafeProblems(files, read),\n"),
    "C10-float-grep-unhooked": ("check.mjs",
                                "    ...roomFloatProblems(files, read),\n  ];", "  ];"),
    # The real tree, read by the deps step.
    "R1-room-float": ("crates/room/src/lib.rs", ("append", "\npub const SCALE: f32 = 0.5;\n")),
    "R2-loop-catchup-literal": ("crates/sim/src/ticks.rs",
                                "pub const LOOP_CATCHUP: u32 = loop_catchup_at(TICK_HZ);",
                                "pub const LOOP_CATCHUP: u32 = 7;"),
    "R3-room-tick-hz": ("crates/room/src/lib.rs", ("append", "\npub const TICK_HZ: u32 = 30;\n")),
}


def sha(p):
    return hashlib.sha256(p.read_bytes()).hexdigest()


def main():
    cmd = sys.argv[1]
    if cmd == "status":
        bad = 0
        for rel, h in BASE.items():
            now = sha(SPIKE / rel)
            print(f"{rel}: {now} {'==' if now == h else '!='} baseline")
            bad += now != h
        sys.exit(1 if bad else 0)
    name = sys.argv[2]
    spec = BITES[name]
    rel = spec[0]
    path = SPIKE / rel
    text = path.read_bytes().decode("utf-8")
    if cmd == "plant":
        if sha(path) != BASE[rel]:
            sys.exit(f"REFUSED {name}: {rel} is not at its baseline sha256")
        if isinstance(spec[1], tuple):
            planted = text + spec[1][1]
        else:
            old, new = spec[1], spec[2]
            if text.count(old) != 1:
                sys.exit(f"REFUSED {name}: the site occurs {text.count(old)} times in {rel}")
            planted = text.replace(old, new)
            if planted.count(new) != 1:
                sys.exit(f"REFUSED {name}: the planted text would occur {planted.count(new)} times (no unique restore)")
        path.write_bytes(planted.encode("utf-8"))
        print(f"PLANTED {name} in {rel}: sha256 {sha(path)}")
    elif cmd == "restore":
        if isinstance(spec[1], tuple):
            suffix = spec[1][1]
            if not text.endswith(suffix):
                sys.exit(f"REFUSED restore {name}: {rel} does not end with the planted suffix")
            restored = text[: -len(suffix)]
        else:
            old, new = spec[1], spec[2]
            if text.count(new) != 1:
                sys.exit(f"REFUSED restore {name}: the planted text occurs {text.count(new)} times in {rel}")
            restored = text.replace(new, old)
        path.write_bytes(restored.encode("utf-8"))
        h = sha(path)
        print(f"RESTORED {name} in {rel}: sha256 {h} {'==' if h == BASE[rel] else '!='} baseline")
        sys.exit(0 if h == BASE[rel] else 1)


main()
