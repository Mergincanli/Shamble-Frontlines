# rust-reviewer's U4s FIX-PASS plant tool: one bite at a time, in place, in the builder's worktree (no copy of any
# source file is made; the original is restored by the reverse replacements and its sha256 checked against the
# baseline below, which is the fix pass's final state).
#   python3 -I u4s-fix-rust-plant.py plant <name>    refuses unless the file is at its baseline and each site is unique
#   python3 -I u4s-fix-rust-plant.py restore <name>  refuses unless each planted text is there exactly once
#   python3 -I u4s-fix-rust-plant.py status          every target's sha256 against its baseline
import hashlib
import pathlib
import sys

SPIKE = pathlib.Path("/home/user/Shamble-Frontlines/.claude/worktrees/wf_aa2cc979-004-1/spikes/netcode-proof")
BASE = {
    "crates/sim/src/grid.rs": "ede88bbedfcc0413bbc999006272a43ec76913a79fb87da674f011a7ab16aa50",
    "crates/sim/src/history.rs": "1c62e5aaf0c3a7d3fd6c60dd77c11fe2f827c5414119e8e0a7ab996ba775ba3f",
    "crates/sim/src/ticks.rs": "5d852f1a81f72e958a89f5489bce5d556da92d096e86aebc4bd130e447dece5b",
    "crates/sim/src/seam.rs": "a203150c12361d0b54c1abb36466bfbb2ae36eddc223dedfbf3c7a50d76f7f20",
    "crates/client/src/lib.rs": "c4c4797df588d9abbea890c49f80b105bf877cc0a1b65c3ea8254c12a0a0197e",
    "check.mjs": "508cd49ffd7f1571b13c168c7cd3463b57d383f15da194c6d15cdb0cb3706dde",
}

# name: (file, [(old, new), ...]); each old is replaced by its new exactly once, in order. ("append", suffix) appends.
BITES = {
    # Item 4: the ring's span rounded down, not up (ms * hz / 1000 floored).
    "F4-floor-span": ("crates/sim/src/history.rs", [(
        "let capacity = crate::ticks_for_ms(span_ms) as usize;",
        "let capacity = (span_ms as usize * crate::TICK_HZ as usize) / 1_000;")]),
    # Item 5 probe: the grid gets an id list back (a second home for -17's id). Expected to PASS everything.
    "F5a-second-home": ("crates/sim/src/grid.rs", ("append",
        '\n#[cfg(feature = "bites")]\npub const BITES: [&str; 1] = ["PM-0001-17/client-rounding"];\n')),
    # Item 5: the client's site maps its id to no defect.
    "F5b-client-no-defect": ("crates/client/src/lib.rs", [(
        ".then(npt_sim::grid::Config::floor_rounding)", ".then(npt_sim::grid::Config::default)")]),
    # Item 6: EntityKind::ALL names one kind twice and drops another.
    "F6-all-duplicate": ("crates/sim/src/seam.rs", [(
        "        EntityKind::Effect,\n    ];", "        EntityKind::Wall,\n    ];")]),
    # Item 7: a build dependency of npt-sim no longer counts as shipped.
    "F7-build-dep-unshipped": ("check.mjs", [(
        "for (const d of (simNode?.deps ?? []).filter(isShipped)) {",
        "for (const d of (simNode?.deps ?? []).filter((d) => d.dep_kinds.some((k) => k.kind === null))) {")]),
    # Item 9: the function itself drifts (32 at 30 Hz, 62 at 60 Hz); the constant's form stays right.
    "F9a-fn-drift": ("crates/sim/src/ticks.rs", [("    32 * tick_hz / 30\n", "    tick_hz + 2\n")]),
    # Item 9: at 60 Hz, a constant at a literal rate: the const assert must refuse it at compile time.
    "F9b-60hz-literal-rate": ("crates/sim/src/ticks.rs", [
        ("pub const TICK_HZ: u32 = 30;", "pub const TICK_HZ: u32 = 60;"),
        ("pub const FULL_STATE_ACK_TICKS: u32 = full_state_ack_ticks_at(TICK_HZ);",
         "pub const FULL_STATE_ACK_TICKS: u32 = full_state_ack_ticks_at(30);")]),
    # Item 10, real tree: npt-client re-derives a derived name from TICK_HZ.
    "F10a-client-rederives": ("crates/client/src/lib.rs", ("append",
        "\npub const LOOP_CATCHUP: u32 = npt_sim::TICK_HZ / 4;\n")),
    # Item 10, self-test: the name rule forgets `*CATCHUP*` names.
    "F10b-catchup-names-dropped": ("check.mjs", [(
        "export const TICK_NAMED = /_TICKS$|CATCHUP/;", "export const TICK_NAMED = /_TICKS$/;")]),
    # Item 3: a broken intra-doc link in the Rewind rule's text (rustdoc -D warnings must refuse it; check.mjs runs no
    # rustdoc, so it is expected to PASS check.mjs).
    "F3-doc-link": ("crates/sim/src/seam.rs", [(
        "[`InputReports::rewind_outside`]", "[`InputReports::rewind_outsde`]")]),
}


def sha(p):
    return hashlib.sha256(p.read_bytes()).hexdigest()


def main():
    cmd = sys.argv[1]
    if cmd == "status":
        bad = 0
        for rel, h in BASE.items():
            now = sha(SPIKE / rel)
            print(f"{rel}: {now[:16]} {'==' if now == h else '!='} baseline")
            bad += now != h
        sys.exit(1 if bad else 0)
    name = sys.argv[2]
    rel, spec = BITES[name]
    path = SPIKE / rel
    text = path.read_bytes().decode("utf-8")
    if cmd == "plant":
        if sha(path) != BASE[rel]:
            sys.exit(f"REFUSED {name}: {rel} is not at its baseline sha256")
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
