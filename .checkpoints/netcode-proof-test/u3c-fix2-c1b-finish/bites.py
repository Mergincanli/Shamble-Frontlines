"""U3c C1b finish: bites on this tree. Each bite reverts one fix in place (exact, unique text replacements), runs its
test(s), restores the files. A bite with expect BITES must make the named tests fail; a control with expect HOLDS must
leave them passing (G-NC8: the stall alone passes, the stall with the old 10 ms margin fails).

Usage: python3 -I bites.py <result file> [bite names...]   (no names: all)
       python3 -I bites.py --restore                        (after an interrupted run)
"""
import json
import os
import subprocess
import sys

ROOT = "/home/user/Shamble-Frontlines/.claude/worktrees/wf_10e244da-89a-1/spikes/netcode-proof"
SRC = ROOT + "/crates/net/src/l2/"
ENV = dict(os.environ, CARGO_TARGET_DIR="/home/user/Shamble-Frontlines/.gate/u3v3-harness-check-target")
SAVE = "/tmp/claude-0/-home-user-Shamble-Frontlines/077a208e-2c05-50f2-8333-d6ac6effb7f2/scratchpad/c1b-finish-bite-saved.json"

T = "l2::delay::tests::"
STALL_OLD = '    assert!(d2.add_burst(at_now + 1, SEC).is_ok(), "just after it");\n'
STALL_NEW = "    std::thread::sleep(Duration::from_millis(15));\n" + STALL_OLD
# name: (expect, [(file, old, new)], [tests])
BITES = {
    "R1": ("BITES", [("delay.rs",
                      "let again = c.released[DOWN] > 0 && !s_owes_write;",
                      "let again = c.released[DOWN] > 0 && !s_owes_write && false;")],
           [T + "a_down_record_released_in_a_pump_pass_is_written_or_owed_a_wake"]),
    "G-PRE-WAKE": ("BITES", [("delay.rs",
                              "(got, got[DOWN] || reopened)",
                              "(got, got[DOWN] || (reopened && false))")],
                   [T + "an_acknowledgement_an_application_task_steps_that_reopens_the_up_room_wakes_the_pump"]),
    "G-PRE-PANIC": ("BITES", [("delay.rs",
                               "if std::thread::panicking() && c.failed.is_none() {",
                               "if std::thread::panicking() && c.failed.is_none() && false {")],
                    [T + "a_pump_that_panics_gives_the_reader_an_error_never_a_clean_eof"]),
    "G-F10": ("BITES", [("delay.rs", "                b.0 -= start;\n", "                let _ = (b.0, start);\n")],
              [T + "tiny_frames_that_fill_the_up_room_drain_whole_and_in_order"]),
    "G-NC4": ("BITES", [("delay.rs", "            owed: self.owed,\n", "            owed: 0,\n")],
              [T + "one_record_per_write_never_beyond_the_budget"]),
    "G-NC4-NONZERO": ("BITES", [("delay.rs", "            owed: self.owed,\n", "            owed: self.owed + 1,\n")],
                      [T + "c_the_writer_keeps_outstanding_within_n_snapshots_at_every_return"]),
    "G-FH": ("BITES", [("delay.rs",
                        "        if free <= OVERHEAD {\n            c.owed = buf.len() as u64;",
                        "        if free <= 20_480 {\n            c.owed = buf.len() as u64;")],
             [T + "a_full_budget_is_pending_until_it_frees_and_the_held_bytes_stay_bounded"]),
    "G-NC8-STALL-ONLY": ("HOLDS", [("delay_tests.rs", STALL_OLD, STALL_NEW)],
                         [T + "a_burst_past_the_clocks_range_is_refused_and_never_wraps"]),
    "G-NC8": ("BITES", [("delay_tests.rs", STALL_OLD, STALL_NEW),
                        ("delay_tests.rs",
                         "let t = c.model_t(delay_clock::clock().now_ns() + 10 * SEC);",
                         "let t = c.model_t(delay_clock::clock().now_ns() + 10 * MS);")],
              [T + "a_burst_past_the_clocks_range_is_refused_and_never_wraps"]),
    "G-NC2": ("BITES", [("delay.rs", "if held > budget + prefix {", "if held > budget {")],
              [T + "under_loss_the_held_bytes_stay_within_64_kb_and_the_front_records_acknowledged_prefix"]),
    "SPLIT-CHECK": ("BITES", [("delay.rs",
                               "        if parts[0].saturating_add(parts[1]).saturating_add(parts[2]) != total {",
                               "        if false {")],
                    [T + "the_lateness_split_sums_to_the_total_per_release_and_its_bites_break_it"]),
    "SPLIT-POOL": ("BITES", [("delay.rs", "            p[HOP].record(parts[1]);\n", "")],
                   [T + "the_lateness_split_sums_to_the_total_per_release_and_its_bites_break_it"]),
    "POOLED-ATOMIC": ("BITES", [("stats.rs",
                                 "        self.sum[b].fetch_add(ns, Relaxed);\n",
                                 "        let s = self.sum[b].load(Relaxed);\n        std::thread::yield_now();\n"
                                 "        self.sum[b].store(s + ns, Relaxed);\n")],
                      ["l2::stats::tests::a_pooled_histogram_fed_from_threads_equals_one_fed_the_same_samples"]),
    "SPLIT-SIZE": ("BITES", [("delay.rs", "        2 * PARTS as u64 * Pooled::heap_bytes()\n",
                              "        2 * (PARTS as u64 + 1) * Pooled::heap_bytes()\n")],
                   [T + "its_rows_use_the_named_series"]),
}


def apply(name):
    _, edits, _ = BITES[name]
    saved = {}
    for f, old, new in edits:
        path = SRC + f
        text = saved.get(path) or open(path, encoding="utf-8").read()
        saved.setdefault(path, text)
    json.dump(saved, open(SAVE, "w"))
    cur = dict(saved)
    for f, old, new in edits:
        path = SRC + f
        assert cur[path].count(old) == 1, (name, f, cur[path].count(old))
        cur[path] = cur[path].replace(old, new)
    for path, text in cur.items():
        open(path, "w", encoding="utf-8").write(text)


def restore():
    if os.path.exists(SAVE):
        for path, text in json.load(open(SAVE)).items():
            open(path, "w", encoding="utf-8").write(text)
        os.remove(SAVE)


def run(names, out):
    for name in names:
        expect, _, tests = BITES[name]
        apply(name)
        try:
            r = subprocess.run(
                ["cargo", "+1.99.1", "test", "--locked", "-p", "npt-net", "--lib", "--", "--exact", *tests],
                cwd=ROOT, env=ENV, capture_output=True, text=True, timeout=900)
            log = r.stdout + r.stderr
            keep = [l for l in log.splitlines()
                    if l.startswith("test ") or "panicked" in l or "test result" in l or l.startswith("error")
                    or "assertion" in l or "left:" in l or "right:" in l or "a panicked" in l]
            failed = r.returncode != 0 and "test result: FAILED" in log
            passed = r.returncode == 0 and "test result: ok. " in log and " 0 passed" not in log
            if expect == "BITES":
                verdict = "BITES" if failed else ("MISSED" if passed else "ERROR (no test result)")
            else:
                verdict = "HOLDS (control)" if passed else ("CONTROL FAILED" if failed else "ERROR (no test result)")
            with open(out, "a") as o:
                o.write(f"== {name}: {verdict}\n" + "\n".join(keep[:14]) + "\n")
            print(name, verdict, flush=True)
        finally:
            restore()


if __name__ == "__main__":
    if sys.argv[1] == "--restore":
        restore()
    else:
        run(sys.argv[2:] or list(BITES), sys.argv[1])
