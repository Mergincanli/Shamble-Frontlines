"""rust-reviewer, scratch copy only: (1) the pump's polls during the 1.5 s a_full_budget test holds a non-reading client
(S Pending, released records untaken), with the fix and with R1-SPIN (no `!s_owes_write`); (2) probe 2 (the fix agent's
probe2.rs appended) with the fix and with R1 off, 3 runs each; (3) the new tests repeated for flakiness. Every edit is
restored from the saved text; the scratch l2 hashes are checked by the caller.

Usage: python3 -I spin_and_probe2.py <out>
"""
import os
import subprocess
import sys

ROOT = "/home/user/Shamble-Frontlines/.gate/c1b-review-scratch/netcode-proof"
SRC = ROOT + "/crates/net/src/l2/"
ENV = dict(os.environ, CARGO_TARGET_DIR="/home/user/Shamble-Frontlines/.gate/u3v3-harness-check-target")
NOTES = "/home/user/Shamble-Frontlines/.checkpoints/netcode-proof-test/u3c-fix2-review-c1b/"
T = "l2::delay::tests::"
AGAIN = "let again = c.released[DOWN] > 0 && !s_owes_write;"
SLEEP = "    tokio::time::sleep(Duration::from_millis(1_500)).await;\n"
SPIN_PRINT = ("    let p0 = ds.life().polls;\n" + SLEEP +
              "    println!(\"SPINPROBE polls during the 1.5 s hold: {}\", ds.life().polls - p0);\n")


def cargo(args):
    r = subprocess.run(["cargo", "+1.99.1", "test", "--locked", "-p", "npt-net", "--lib", "--", *args],
                       cwd=ROOT, env=ENV, capture_output=True, text=True, timeout=1200)
    return r.stdout + r.stderr


def with_edits(edits, fn):
    saved = {}
    try:
        for f, old, new in edits:
            p = SRC + f
            text = saved.get(p) or open(p, encoding="utf-8").read()
            saved.setdefault(p, text)
            cur = open(p, encoding="utf-8").read()
            if old is None:
                cur = cur + new
            else:
                assert cur.count(old) == 1, (f, old)
                cur = cur.replace(old, new)
            open(p, "w", encoding="utf-8").write(cur)
        return fn()
    finally:
        for p, text in saved.items():
            open(p, "w", encoding="utf-8").write(text)


def main(out):
    o = open(out, "a")
    full = T + "a_full_budget_is_pending_until_it_frees_and_the_held_bytes_stay_bounded"
    for label, extra in [("FIXED", []), ("R1-SPIN", [("delay.rs", AGAIN, "let again = c.released[DOWN] > 0;")])]:
        def go():
            lines = []
            for i in range(3):
                log = cargo(["--exact", full, "--nocapture"])
                lines += [f"{label} run {i + 1}: {l}" for l in log.splitlines() if "SPINPROBE" in l or "test result" in l]
            return lines
        o.write("\n".join(with_edits([("delay_tests.rs", SLEEP, SPIN_PRINT)] + extra, go)) + "\n")
        o.flush()
    probe = open(NOTES + "probe2.rs", encoding="utf-8").read()
    for label, extra in [("FIXED", []), ("R1-OFF", [("delay.rs", AGAIN, "let again = false;")])]:
        def go():
            lines = []
            for i in range(3):
                log = cargo(["--exact", T + "zz_probe2_second_record_goes_out_soon_after_its_release", "--nocapture"])
                lines += [f"PROBE2 {label} run {i + 1}: {l}" for l in log.splitlines() if "PROBE2" in l or "test result" in l]
            return lines
        o.write("\n".join(with_edits([("delay_tests.rs", None, "\n" + probe)] + extra, go)) + "\n")
        o.flush()
    for t in ["a_down_record_released_in_a_pump_pass_is_written_or_owed_a_wake",
              "an_acknowledgement_an_application_task_steps_that_reopens_the_up_room_wakes_the_pump",
              "a_pump_that_panics_gives_the_reader_an_error_never_a_clean_eof"]:
        ok = sum("test result: ok. 1 passed" in cargo(["--exact", T + t]) for _ in range(10))
        o.write(f"REPEAT {t}: {ok}/10 passed\n")
        o.flush()


if __name__ == "__main__":
    main(sys.argv[1])
