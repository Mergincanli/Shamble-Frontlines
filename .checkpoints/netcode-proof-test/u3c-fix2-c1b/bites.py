"""U3c C1b bites: each fix reverted in place (one exact, unique text replacement), its test(s) run, the file restored.
A bite BITES when the named tests fail. Output: one block per bite, appended to the result file.

Usage: python3 -I bites.py <result file> [bite names...]   (no names: all)
       python3 -I bites.py --apply <name>   /   --restore   (for a manual run under one mutation)
"""
import json
import os
import subprocess
import sys

ROOT = "/home/user/Shamble-Frontlines/.claude/worktrees/wf_ac5e4575-9da-1/spikes/netcode-proof"
SRC = ROOT + "/crates/net/src/l2/"
ENV = dict(os.environ, CARGO_TARGET_DIR="/home/user/Shamble-Frontlines/.gate/u3v3-harness-check-target")
SAVE = "/tmp/claude-0/-home-user-Shamble-Frontlines/077a208e-2c05-50f2-8333-d6ac6effb7f2/scratchpad/bite-saved.json"

T = "l2::delay::tests::"
BITES = {
    "R1": ("delay.rs",
           "let again = c.released[DOWN] > 0 && !s_owes_write;",
           "let again = c.released[DOWN] > 0 && !s_owes_write && false;",
           [T + "a_down_record_released_in_a_pump_pass_is_written_or_owed_a_wake"]),
    "G-PRE-WAKE": ("delay.rs",
                   "(got, got[DOWN] || reopened)",
                   "(got, got[DOWN] || (reopened && false))",
                   [T + "an_acknowledgement_an_application_task_steps_that_reopens_the_up_room_wakes_the_pump"]),
    "G-PRE-PANIC": ("delay.rs",
                    "if std::thread::panicking() && c.failed.is_none() {",
                    "if std::thread::panicking() && c.failed.is_none() && false {",
                    [T + "a_pump_that_panics_gives_the_reader_an_error_never_a_clean_eof"]),
    "G-F10": ("delay.rs",
              "                b.0 -= start;\n",
              "                let _ = (b.0, start);\n",
              [T + "tiny_frames_that_fill_the_up_room_drain_whole_and_in_order"]),
    "G-NC4": ("delay.rs",
              "            owed: self.owed,\n",
              "            owed: 0,\n",
              [T + "one_record_per_write_never_beyond_the_budget"]),
    "G-FH": ("delay.rs",
             "        if free <= OVERHEAD {\n            c.owed = buf.len() as u64;",
             "        if free <= 20_480 {\n            c.owed = buf.len() as u64;",
             [T + "a_full_budget_is_pending_until_it_frees_and_the_held_bytes_stay_bounded"]),
    "G-NC2": ("delay.rs",
              "if held > budget + prefix {",
              "if held > budget {",
              [T + "under_loss_the_held_bytes_stay_within_64_kb_and_the_front_records_acknowledged_prefix"]),
    "SPLIT-CHECK": ("delay.rs",
                    "        if parts[0].saturating_add(parts[1]).saturating_add(parts[2]) != total {",
                    "        if false {",
                    [T + "the_lateness_split_sums_to_the_total_per_release_and_its_bites_break_it"]),
    "SPLIT-POOL": ("delay.rs",
                   "            p[HOP].record(parts[1]);\n",
                   "",
                   [T + "the_lateness_split_sums_to_the_total_per_release_and_its_bites_break_it"]),
    "POOLED-ATOMIC": ("stats.rs",
                      "        self.sum[b].fetch_add(ns, Relaxed);\n",
                      "        let s = self.sum[b].load(Relaxed);\n        std::thread::yield_now();\n"
                      "        self.sum[b].store(s + ns, Relaxed);\n",
                      ["l2::stats::tests::a_pooled_histogram_fed_from_threads_equals_one_fed_the_same_samples"]),
}


def apply(name):
    f, old, new, _ = BITES[name]
    path = SRC + f
    text = open(path, encoding="utf-8").read()
    assert text.count(old) == 1, (name, text.count(old))
    json.dump({"path": path, "text": text}, open(SAVE, "w"))
    open(path, "w", encoding="utf-8").write(text.replace(old, new))


def restore():
    if os.path.exists(SAVE):
        s = json.load(open(SAVE))
        open(s["path"], "w", encoding="utf-8").write(s["text"])
        os.remove(SAVE)


def run(names, out):
    for name in names:
        apply(name)
        try:
            tests = BITES[name][3]
            r = subprocess.run(
                ["cargo", "+1.99.1", "test", "--locked", "-p", "npt-net", "--lib", "--", "--exact", *tests],
                cwd=ROOT, env=ENV, capture_output=True, text=True, timeout=900)
            log = r.stdout + r.stderr
            keep = [l for l in log.splitlines()
                    if l.startswith("test ") or "panicked" in l or "test result" in l or l.startswith("error")
                    or "assertion" in l or "left:" in l or "right:" in l]
            verdict = "BITES" if r.returncode != 0 and "test result: FAILED" in log else (
                "MISSED" if r.returncode == 0 else "ERROR (no test result)")
            with open(out, "a") as o:
                o.write(f"== {name}: {verdict}\n" + "\n".join(keep[:14]) + "\n")
            print(name, verdict, flush=True)
        finally:
            restore()


if __name__ == "__main__":
    if sys.argv[1] == "--apply":
        apply(sys.argv[2])
    elif sys.argv[1] == "--restore":
        restore()
    else:
        run(sys.argv[2:] or list(BITES), sys.argv[1])
