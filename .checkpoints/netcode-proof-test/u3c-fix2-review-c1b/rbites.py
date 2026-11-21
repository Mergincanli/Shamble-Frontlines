"""rust-reviewer's own bites on a SCRATCH COPY of the C1b tree (never the worktree). Each bite edits the scratch copy,
runs the named tests (--exact) or a filter, then restores the scratch file from the saved text. The shared target dir is
used (no new target dir); the last step (--final) restores everything and touches the scratch sources so the shared
artifacts are rebuilt from content identical to the worktree's.

Usage: python3 -I rbites.py <out> [names...] | python3 -I rbites.py --final <out>
"""
import os
import subprocess
import sys
import time

ROOT = "/home/user/Shamble-Frontlines/.gate/c1b-review-scratch/netcode-proof"
SRC = ROOT + "/crates/net/src/l2/"
ENV = dict(os.environ, CARGO_TARGET_DIR="/home/user/Shamble-Frontlines/.gate/u3v3-harness-check-target")
T = "l2::delay::tests::"
AGAIN = "let again = c.released[DOWN] > 0 && !s_owes_write;"
PUMPW = "        let pump = if wake_pump {\n"
R1T = T + "a_down_record_released_in_a_pump_pass_is_written_or_owed_a_wake"
WAKET = T + "an_acknowledgement_an_application_task_steps_that_reopens_the_up_room_wakes_the_pump"
PANT = T + "a_pump_that_panics_gives_the_reader_an_error_never_a_clean_eof"
# name: (expect, file, old, new, occurrence index or None for unique, tests (exact) or filter string)
BITES = {
    "R1-OFF": ("BITES", "delay.rs", AGAIN, "let again = false;", None, [R1T]),
    "R1-SPIN": ("INFO", "delay.rs", AGAIN, "let again = c.released[DOWN] > 0;", None, "l2::delay::"),
    "WAKE-CURRENT-ONLY-OFF": ("BITES", "delay.rs", PUMPW, "        let pump = if got[DOWN] {\n", 1, [WAKET]),
    "WAKE-ADDBURST-ONLY-OFF": ("INFO", "delay.rs", PUMPW, "        let pump = if got[DOWN] {\n", 0, "l2::delay::"),
    "PANIC-OFF": ("BITES", "delay.rs", "if std::thread::panicking() && c.failed.is_none() {",
                  "if std::thread::panicking() && c.failed.is_none() && false {", None, [PANT]),
    "PANIC-ALWAYS": ("INFO", "delay.rs", "if std::thread::panicking() && c.failed.is_none() {",
                     "if c.failed.is_none() {", None, "l2::delay::"),
    "F10-OFF": ("BITES", "delay.rs", "                b.0 -= start;\n", "                let _ = (b.0, start);\n", None,
                [T + "tiny_frames_that_fill_the_up_room_drain_whole_and_in_order"]),
    "FH-20K": ("BITES", "delay.rs", "        if free <= OVERHEAD {\n", "        if free <= 20_480 {\n", None,
               [T + "a_full_budget_is_pending_until_it_frees_and_the_held_bytes_stay_bounded"]),
    "NC2-TIGHT": ("BITES", "delay.rs", "if held > budget + prefix {", "if held > budget {", None,
                  [T + "under_loss_the_held_bytes_stay_within_64_kb_and_the_front_records_acknowledged_prefix"]),
    "NC4-ZERO": ("BITES", "delay.rs", "            owed: self.owed,\n", "            owed: 0,\n", None,
                 [T + "one_record_per_write_never_beyond_the_budget"]),
}


def edit(path, old, new, occ):
    text = open(path, encoding="utf-8").read()
    n = text.count(old)
    if occ is None:
        assert n == 1, (path, old, n)
        return text, text.replace(old, new)
    assert n > occ, (path, old, n)
    i = -1
    for _ in range(occ + 1):
        i = text.index(old, i + 1)
    return text, text[:i] + new + text[i + len(old):]


def cargo_test(tests):
    args = ["cargo", "+1.99.1", "test", "--locked", "-p", "npt-net", "--lib", "--"]
    args += (["--exact", *tests] if isinstance(tests, list) else [tests])
    t0 = time.time()
    r = subprocess.run(args, cwd=ROOT, env=ENV, capture_output=True, text=True, timeout=1200)
    return r, time.time() - t0


def run(names, out):
    for name in names:
        expect, f, old, new, occ, tests = BITES[name]
        path = SRC + f
        saved, bitten = edit(path, old, new, occ)
        open(path, "w", encoding="utf-8").write(bitten)
        try:
            r, secs = cargo_test(tests)
        finally:
            open(path, "w", encoding="utf-8").write(saved)
        log = r.stdout + r.stderr
        keep = [l for l in log.splitlines() if "FAILED" in l or "panicked" in l or "test result" in l
                or l.startswith("error") or "left:" in l or "right:" in l or "R1" in l]
        failed = "test result: FAILED" in log
        passed = r.returncode == 0 and "test result: ok." in log
        verdict = ("BITES" if failed else "MISSED") if expect == "BITES" else ("some test FAILED" if failed else
                                                                                 ("all passed" if passed else "ERROR"))
        with open(out, "a") as o:
            o.write(f"== {name} [{expect}] {verdict} ({secs:.1f} s)\n" + "\n".join(keep[:16]) + "\n")
        print(name, verdict, f"{secs:.1f}s", flush=True)


def repeat(out, n):
    """The new tests on the unbitten scratch tree, n times each (flakiness)."""
    for t in [R1T, WAKET, PANT]:
        fails = 0
        for _ in range(n):
            r, _ = cargo_test([t])
            if "test result: ok. 1 passed" not in r.stdout + r.stderr:
                fails += 1
        with open(out, "a") as o:
            o.write(f"== REPEAT {t.split('::')[-1]}: {n - fails}/{n} passed\n")
        print(t, n - fails, "of", n, flush=True)


if __name__ == "__main__":
    if sys.argv[1] == "--repeat":
        repeat(sys.argv[2], int(sys.argv[3]))
    elif sys.argv[1] == "--final":
        for f in os.listdir(SRC):
            os.utime(SRC + f)
        os.utime(ROOT + "/marks/SERIES.md")
        r, secs = cargo_test(T + "its_rows_use_the_named_series")
        with open(sys.argv[2], "a") as o:
            o.write(f"== FINAL rebuild from restored scratch: rc {r.returncode} ({secs:.1f} s)\n")
    else:
        run(sys.argv[2:] or list(BITES), sys.argv[1])
