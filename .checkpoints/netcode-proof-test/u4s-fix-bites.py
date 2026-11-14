# U4s fix pass: the bites. Each bite is planted in place in the builder's worktree (one or more exact, unique
# replacements in files of the spike), one bite at a time; its test, check or build is run; the result is compared with
# what the bite must give; every planted file is restored from the text held in memory and its sha256 checked against
# the sha256 taken at the start. Never a copy of a source file on disk. Build rules (u4-build-plan.md §7.1):
# CARGO_TARGET_DIR is the shared folder, CARGO_NET_OFFLINE is true, every file under crates/ is touched before each
# build, and each test run must show the expected number of tests.
# Run: python3 -I <this> <spike dir> [bite id ...]
import hashlib
import os
import pathlib
import re
import subprocess
import sys

SPIKE = pathlib.Path(sys.argv[1]).resolve()
ONLY = set(sys.argv[2:])
ENV = {**os.environ, "CARGO_TARGET_DIR": "/home/user/Shamble-Frontlines/.gate/shared-target", "CARGO_NET_OFFLINE": "true"}
GRID = SPIKE / "crates/sim/src/grid.rs"
HISTORY = SPIKE / "crates/sim/src/history.rs"
TICKS = SPIKE / "crates/sim/src/ticks.rs"
SEAM = SPIKE / "crates/sim/src/seam.rs"
SIMLIB = SPIKE / "crates/sim/src/lib.rs"
CLIENT = SPIKE / "crates/client/src/lib.rs"
ROOM = SPIKE / "crates/room/src/lib.rs"
CHECK = SPIKE / "check.mjs"
PROTO = SPIKE / "crates/proto/Cargo.toml"
LOCK = SPIKE / "Cargo.lock"
FILES = (GRID, HISTORY, TICKS, SEAM, SIMLIB, CLIENT, ROOM, CHECK, PROTO, LOCK)
TREE = ["cargo", "tree", "-d", "--all-features", "--target", "all", "--locked", "--offline"]
PROTO_DEV = PROTO.read_text()[PROTO.read_text().index("\n[dev-dependencies]\n"):]
LOCK_PROTO = ('name = "npt-proto"\nversion = "0.0.0"\ndependencies = [\n "npt-link",\n "npt-sim",\n "rand_chacha",\n'
              ' "rand_core",\n]\n')

SIM_TEST = ["cargo", "test", "--locked", "--offline", "-p", "npt-sim", "--test", "seam"]
SIM_BITES = ["cargo", "test", "--locked", "--offline", "-p", "npt-sim", "--features", "bites", "--test", "seam"]
CLIENT_BITES = ["cargo", "test", "--locked", "--offline", "-p", "npt-client", "--features", "bites", "--test", "bites"]
SIM_CHECK = ["cargo", "check", "--locked", "--offline", "-p", "npt-sim"]
SIM_DOC = ["cargo", "doc", "--locked", "--offline", "--no-deps", "-p", "npt-sim"]
DEPS = ["node", "check.mjs", "--only", "deps"]

R4 = "pub const LOOP_CATCHUP: u32 = 7 * TICK_HZ / 30;"
LOOP = "pub const LOOP_CATCHUP: u32 = loop_catchup_at(TICK_HZ);"
HZ = "pub const TICK_HZ: u32 = 30;"
RANGE_ASSERT = ('const _: () = assert!(\n    TICK_HZ >= 1 && TICK_HZ <= MAX_TICK_HZ,\n'
                '    "TICK_HZ is 1 to 1,000 ticks per second"\n);\n')
LOOP_ASSERT = ('const _: () = assert!(\n    LOOP_CATCHUP == loop_catchup_at(TICK_HZ),\n'
               '    "LOOP_CATCHUP is loop_catchup_at(TICK_HZ)"\n);\n')
SOLDIER = "const SOLDIER: [Field; 9] = [\n    XYZ[0], XYZ[1], XYZ[2], YAW, PITCH, VELOCITY, VELOCITY, VELOCITY, FLAGS,\n];"

# id, what, plants [(file, old, new)], kind, command, then per kind:
#   "test":  (filters, expected ran, names that must fail, text the output must hold)
#   "pass":  (filters, expected ran)                      -- the run must pass (a control or an evasion shown)
#   "fail":  (text the output must hold)                  -- a build, doc or check step must fail with it
#   "ok":    ()                                           -- a build must succeed (a control)
#   "node":  (name pattern, expected tests, names that must fail, text)
BITES = [
    # The build's nine bites, on their sites after the fix pass.
    ("O1", "grid floor-rounding in the wire grid (s2)", [(GRID, "field.fit(scaled.round() as i64)", "field.fit(scaled.floor() as i64)")],
     "test", SIM_TEST, (["s2_"], 2, ["s2_from_then_to_returns_every_integer_of_every_field",
                                     "s2_to_errs_by_at_most_half_a_step_plus_one_ulp_over_the_half_steps_and_the_range_ends"], "came back from")),
    ("O2", "the ring kept one short (s4)", [(HISTORY, "let capacity = crate::ticks_for_ms(span_ms) as usize;",
                                               "let capacity = (crate::ticks_for_ms(span_ms) as usize).saturating_sub(1);")],
     "test", SIM_TEST, (["s4"], 2, ["s4_the_hit_history_keeps_exactly_its_capacity_and_evicts_the_oldest"], "right: 64")),
    ("O3", "check: a call counts as a path outside npt-sim", [(CHECK, "        if (!pathTo(init, [name])) {", "        if (!(pathTo(init, [name]) || /\\(/.test(init))) {")],
     "node", None, ("tick constants: TICK_HZ is set once", 1, ["tick constants: TICK_HZ is set once"], "loop_catchup_at(30)")),
    ("O4", "check: TICK_HZ may be set anywhere first", [(CHECK, "        if (inHome && home === null) {", "        if (home === null) {")],
     "node", None, ("tick constants: TICK_HZ is set once", 1, ["tick constants: TICK_HZ is set once"], "outside crates/sim/src/")),
    ("O5", "check: float literals missed", [(CHECK, "const FLOAT_TOKEN =\n  /\\bf(?:32|64)\\b|", "const FLOAT_TOKEN =\n  /\\bf(?:32|64)\\b/g; const X =\n  /")],
     "node", None, ("npt-room is float-free", 1, ["npt-room is float-free"], "1.5")),
    ("O6", "check: the tick grep unhooked from the deps step", [(CHECK, "    ...tickConstantProblems(files, read),\n", "")],
     "node", None, ("the deps step is depsStep", 1, ["the deps step is depsStep"], "tickConstantProblems's plant is not reported")),
    ("O7", "check: the float grep unhooked from the deps step", [(CHECK, "    ...roomFloatProblems(files, read),\n", "")],
     "node", None, ("the deps step is depsStep", 1, ["the deps step is depsStep"], "roomFloatProblems's plant is not reported")),
    ("O8", "real tree: a float in npt-room", [(ROOM, None, "\npub const SCALE: f32 = 0.5;\n")],
     "fail", DEPS, ("crates/room/src/lib.rs:7 has `f32` (2 float token(s)): npt-room is float-free",)),
    ("O9", "real tree: LOOP_CATCHUP set from a literal", [(TICKS, LOOP, "pub const LOOP_CATCHUP: u32 = 7;")],
     "fail", DEPS, ("crates/sim/src/ticks.rs:47 sets LOOP_CATCHUP from `7`: in npt-sim it is its own function at the tick rate",)),
    # Item 9 and rust R4.
    ("N1", "real tree: R4, LOOP_CATCHUP = 7 * TICK_HZ / 30 (the grep)", [(TICKS, LOOP, R4)],
     "fail", DEPS, ("crates/sim/src/ticks.rs:47 sets LOOP_CATCHUP from `7 * TICK_HZ / 30`: in npt-sim it is its own function",)),
    ("N1s", "R4 at 30 Hz: s1 and the const asserts cannot tell (shown, the grep's case)", [(TICKS, LOOP, R4)],
     "pass", SIM_TEST, (["s1_"], 1)),
    ("N2", "R4 with TICK_HZ = 60: the const assert refuses the build", [(TICKS, LOOP, R4), (TICKS, HZ, "pub const TICK_HZ: u32 = 60;")],
     "fail", SIM_CHECK, ("LOOP_CATCHUP is loop_catchup_at(TICK_HZ)",)),
    ("N2c", "control: TICK_HZ = 60 alone builds (every constant is its function)", [(TICKS, HZ, "pub const TICK_HZ: u32 = 60;")],
     "ok", SIM_CHECK, ()),
    ("N2r", "reverted: R4 with TICK_HZ = 60 and LOOP_CATCHUP's assert removed builds", [(TICKS, LOOP, R4), (TICKS, HZ, "pub const TICK_HZ: u32 = 60;"), (TICKS, LOOP_ASSERT, "")],
     "ok", SIM_CHECK, ()),
    ("N3", "TICK_HZ = 0: the range assert refuses the build", [(TICKS, HZ, "pub const TICK_HZ: u32 = 0;")],
     "fail", SIM_CHECK, ("TICK_HZ is 1 to 1,000 ticks per second",)),
    ("N3b", "TICK_HZ = 1,001: the range assert refuses the build", [(TICKS, HZ, "pub const TICK_HZ: u32 = 1_001;")],
     "fail", SIM_CHECK, ("TICK_HZ is 1 to 1,000 ticks per second",)),
    ("N3r", "reverted: TICK_HZ = 0 with the range assert removed builds", [(TICKS, HZ, "pub const TICK_HZ: u32 = 0;"), (TICKS, RANGE_ASSERT, "")],
     "ok", SIM_CHECK, ()),
    # Item 4.
    ("N4", "HitHistory::for_ms(0) not refused (s4b)", [(HISTORY, "        if capacity == 0 {\n            return None;\n        }\n", "")],
     "test", SIM_TEST, (["s4b"], 1, ["s4b_a_hit_history_of_zero_ms_is_refused"], "0 ms is refused")),
    # Item 5.
    ("N5", "the own-copy snap also plants -17's id (one id, two sites)",
     [(SIMLIB, "        .contains(&bite)\n", '        .contains(&bite.replace("17/client-rounding", "41/server-quant-differs").as_str())\n')],
     "test", SIM_BITES, (["pm_0001_41"], 1, ["pm_0001_41_server_quant_differs_is_planted_at_the_own_copy_snap_alone"], "PM-0001-17/client-rounding")),
    ("N6", "the client also plants -41's id (one id, two sites)",
     [(CLIENT, "        .contains(&bite)\n", '        .contains(&bite.replace("41/server-quant-differs", "17/client-rounding").as_str())\n')],
     "test", CLIENT_BITES, (["pm_0001_17"], 1, ["pm_0001_17_client_rounding_is_planted_here_alone_and_moves_about_half_of_all_positions"], "PM-0001-41/server-quant-differs")),
    ("N7", "the grid's defect a no-op (floor_rounding rounds to the nearest)",
     [(GRID, "return field.fit(scaled.floor() as i64);", "return field.fit(scaled.round() as i64);")],
     "test", SIM_BITES, (["the_grids_one_defect", "pm_0001_41"], 2, ["the_grids_one_defect_rounds_down_and_moves_about_half_of_all_positions_off_the_wire_grid",
                                                                    "pm_0001_41_server_quant_differs_is_planted_at_the_own_copy_snap_alone"], "right: 12")),
    # Item 6.
    ("N8", "Field::integer one bit short (s6)", [(GRID, "bits: u64::BITS - span.leading_zeros(),", "bits: u64::BITS - span.leading_zeros() - 1,")],
     "test", SIM_TEST, (["s6_"], 1, ["s6_every_kinds_layout_fits_a_record_and_each_field_fits_its_bits"], "Soldier: 256 integers in 7 bits")),
    ("N9", "a soldier's layout past EntityRecord::FIELDS: the const assert refuses the build",
     [(SEAM, SOLDIER, SOLDIER.replace("[Field; 9]", "[Field; 13]").replace("FLAGS,\n", "FLAGS, FLAGS, FLAGS, FLAGS, FLAGS,\n"))],
     "fail", SIM_CHECK, ("every kind's layout fits EntityRecord::FIELDS",)),
    # Item 3: the docs' links (rustdoc -D warnings).
    ("N10", "a broken intra-doc link in Rewind's text", [(SEAM, "[`InputReports::rewind_outside`]", "[`InputReports::rewind_outsde`]")],
     "fail", SIM_DOC, ("unresolved link to `InputReports::rewind_outsde`",)),
    # Item 7.
    ("N11", "check: npt-sim's allow-list widened (serde let through)",
     [(CHECK, "    if (!SIM_DEPENDENCIES.includes(name)) problems.push", '    if (!SIM_DEPENDENCIES.includes(name) && name !== "serde") problems.push')],
     "node", None, ("npt-sim ships with libm", 1, ["npt-sim ships with libm"], "npt-sim depends on serde")),
    # Item 10.
    ("N12", "check: in npt-sim, any initializer naming TICK_HZ counts again (R4 through)",
     [(CHECK, "        if (init !== own) {", "        if (init !== own && !/\\bTICK_HZ\\b/.test(init)) {")],
     "node", None, ("tick constants: in npt-sim each derived name", 1, ["tick constants: in npt-sim each derived name"], "7 * TICK_HZ / 30")),
    ("N13", "check: outside npt-sim, a derived name re-derived from TICK_HZ passes (arch F6)",
     [(CHECK, "        if (!pathTo(init, [name])) {", "        if (!(pathTo(init, [name]) || /\\bTICK_HZ\\b/.test(init))) {")],
     "node", None, ("tick constants: TICK_HZ is set once", 1, ["tick constants: TICK_HZ is set once"], "npt_sim::TICK_HZ / 4")),
    ("N14", "check: the name rule dropped (only the listed names read)",
     [(CHECK, '      if (name !== "TICK_HZ" && !derived && !(named && nameRule)) continue;', '      if (name !== "TICK_HZ" && !derived) continue;')],
     "node", None, ("tick constants: in npt-sim each derived name", 1, ["tick constants: in npt-sim each derived name"], "RESEND_TICKS")),
    ("N14b", "check: outside npt-sim, a tick-count name may be set from anything (the name rule's literal check off)",
     [(CHECK, '        !(pathTo(init, ["TICK_HZ", ...TICK_DERIVED, "\\\\w*_TICKS", "\\\\w*CATCHUP\\\\w*"]) || /\\bTICK_HZ\\b/.test(init) || /\\bticks_for_ms\\(/.test(init))',
       "        false")],
     "node", None, ("tick constants: in npt-sim each derived name", 1, ["tick constants: in npt-sim each derived name"], "LOOP_CATCHUP_ALIAS")),
    ("N15", "real tree: a tick-count name in npt-room set from a literal", [(ROOM, None, "\npub const EVENT_RESEND_TICKS: u32 = 60;\n")],
     "fail", DEPS, ("crates/room/src/lib.rs:7 sets EVENT_RESEND_TICKS from `60`: a tick-counted value is a const fn of npt_sim::TICK_HZ, never a literal",)),
    # Item 8's one trace in `cargo tree -d --all-features --target all`: without npt-proto's two dev edges (manifest and
    # lock), the tree's text is this pass's baseline again (sha256 2da539bb...).
    ("T1", "control: npt-proto's dev edges removed, the duplicate tree is the pass's baseline",
     [(PROTO, PROTO_DEV, ""), (LOCK, LOCK_PROTO, LOCK_PROTO.replace(' "rand_chacha",\n "rand_core",\n', ""))],
     "hash", TREE, ("2da539bbd9d99ffc1f42cfd7811941acd4834c68816a450390e35ea8d6d22d25",)),
]


def sha(p):
    return hashlib.sha256(p.read_bytes()).hexdigest()


def touch():
    for p in (SPIKE / "crates").rglob("*"):
        if p.is_file():
            os.utime(p)


def run(args, env=ENV):
    r = subprocess.run(args, cwd=SPIKE, env=env, capture_output=True, text=True)
    return r.returncode, r.stdout + r.stderr


originals = {p: sha(p) for p in FILES}
for p, h in originals.items():
    print(f"original {p.relative_to(SPIKE)} sha256 {h}")

failures = 0
for bid, what, plants, kind, cmd, want in BITES:
    if ONLY and bid not in ONLY:
        continue
    texts = {}  # each planted file's original text, held in memory for the restore
    planted = {}  # its text with this bite's plants so far
    ok_plant = True
    for path, old, new in plants:
        if path not in texts:
            texts[path] = path.read_text()
            planted[path] = texts[path]
        text = planted[path]
        if old is None:
            text = text + new
        elif text.count(old) != 1:
            print(f"BITE {bid} ({what}): site not unique ({text.count(old)} matches): NOT RUN")
            ok_plant = False
            break
        else:
            text = text.replace(old, new)
        planted[path] = text
        path.write_text(text)
    try:
        if not ok_plant:
            failures += 1
            continue
        df = subprocess.run(["df", "-h", "/home/user"], capture_output=True, text=True).stdout.strip().splitlines()[-1]
        if kind in ("test", "pass", "fail", "ok") and cmd is not DEPS:
            touch()
        if kind in ("test", "pass"):
            filters, ran_want = want[0], want[1]
            code, out = run(cmd + ["--"] + filters)
            ran = re.search(r"running (\d+) tests?", out)
            ran = int(ran.group(1)) if ran else None
            failed = re.findall(r"^test (\S+) \.\.\. FAILED$", out, re.M)
            result = re.search(r"test result: \w+\. \d+ passed; \d+ failed", out)
            if kind == "test":
                names, says = want[2], want[3]
                ok = code != 0 and ran == ran_want and all(n in failed for n in names) and says in out
                print(f"BITE {bid} ({what}): ran {ran} (expected {ran_want}); "
                      f"{result.group(0) if result else 'no result line'}; failed {failed}; says {says!r}: {says in out}")
            else:
                ok = code == 0 and ran == ran_want and not failed
                print(f"BITE {bid} ({what}): ran {ran} (expected {ran_want}); "
                      f"{result.group(0) if result else 'no result line'}; exit {code} (expected 0)")
            m = re.search(r"panicked at ([^\n]+)\n([^\n]+)", out)
            if m:
                print(f"     {m.group(1)}: {m.group(2)}")
        elif kind in ("fail", "ok"):
            env = {**ENV, "RUSTDOCFLAGS": "-D warnings"} if cmd is SIM_DOC else ENV
            code, out = run(cmd, env)
            if kind == "fail":
                says = want[0]
                ok = code != 0 and says in out
                line = next((l for l in out.splitlines() if says[:30] in l), "no matching line")
                print(f"BITE {bid} ({what}): exit {code} (expected not 0); says {says[:70]!r}...: {says in out}")
                print(f"     {line.strip()[:200]}")
            else:
                ok = code == 0
                print(f"BITE {bid} ({what}): exit {code} (expected 0)")
        elif kind == "hash":
            r = subprocess.run(cmd, cwd=SPIKE, env=ENV, capture_output=True)
            got = hashlib.sha256(r.stdout + r.stderr).hexdigest()
            ok = r.returncode == 0 and got == want[0]
            out = (r.stdout + r.stderr).decode()
            print(f"BITE {bid} ({what}): exit {r.returncode}; output sha256 {got[:16]}... "
                  f"{'==' if got == want[0] else '!='} {want[0][:16]}...")
        else:
            pattern, tests_want, names, says = want
            code, out = run(["node", "--test", "--test-name-pattern", pattern, "check.test.mjs"])
            tests = re.search(r"^# tests (\d+)", out, re.M)
            fail = re.search(r"^# fail (\d+)", out, re.M)
            notok = re.findall(r"^not ok \d+ - (.+)$", out, re.M)
            tests, fail = (int(tests.group(1)) if tests else None), (int(fail.group(1)) if fail else None)
            ok = (code != 0 and tests == tests_want and fail == tests_want
                  and all(any(t.startswith(n) for t in notok) for n in names) and says in out)
            print(f"BITE {bid} ({what}): tests {tests} (expected {tests_want}); fail {fail}; not ok {[t[:60] for t in notok]}; "
                  f"says {says!r}: {says in out}")
        print(f"     df: {df}")
        if not ok:
            failures += 1
            print("     NOT AS EXPECTED; output tail:\n" + "\n".join(out.splitlines()[-30:]))
    finally:
        for path, text in texts.items():
            path.write_text(text)
            h = sha(path)
            same = h == originals[path]
            print(f"     restored {path.relative_to(SPIKE)}: sha256 {h[:16]}... {'==' if same else '!='} original")
            if not same:
                failures += 1

for p, h in originals.items():
    if sha(p) != h:
        print(f"MISMATCH after the run: {p}")
        failures += 1
print(f"bites done: {failures} not as expected")
sys.exit(1 if failures else 0)
