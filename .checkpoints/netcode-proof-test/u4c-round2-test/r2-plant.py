# U4c review round 2 (test-engineer): in-place plants on the FIXED npt-l1 (lib.rs d6e6e130, tests.rs e8014c67,
# tests/l1.rs 9c23f9cc), one at a time, and one counted check.mjs run. Reuses round 1's machinery by loading
# ../u4c-test-plant.py by its path (exact-match edits with occurrence counts, a stderr marker in L1Net::new as the
# positive control that the planted build ran, touch every file under crates/ before and every 2 s during the run,
# foreign cargo/rustc recorded, original bytes restored, sha256 checked before and after). Overrides: the hashes (the
# fixed files), the log folder (this folder), and the idle wait (10 s with no cargo/rustc/rustdoc anywhere for a plant
# run, 20 s for the counted check.mjs run; plan §7.1); if that quiet is not reached in 240 s the script stops BEFORE
# planting anything (so a tool timeout can never leave a plant in place during a wait). This agent has no Edit tool:
# the plan's §7.2 "bites planted by a scratch script under §7.1" is the method, as in round 1.
# Run: python3 -I -u <this> <worktree> <name> [<name> ...]   ("check" = the counted check.mjs run)
import hashlib
import importlib.util
import pathlib
import re
import subprocess
import sys
import threading
import time

HERE = pathlib.Path(__file__).resolve().parent
spec = importlib.util.spec_from_file_location("u4c_plant", HERE.parent / "u4c-test-plant.py")
m = importlib.util.module_from_spec(spec)
spec.loader.exec_module(m)  # its __main__ block does not run: __name__ is "u4c_plant"
LIB, UNIT, INTEG = m.LIB, m.UNIT, m.INTEG
m.SHA[LIB] = "d6e6e13054a798e6244a27dd512ab9af0104f7266b2acad453308f906eb4a2ee"
m.SHA[UNIT] = "e8014c6766d6fb1fcc76203d690e33bee892c097a686b7b636e0391d3300f32a"
m.SHA[INTEG] = "9c23f9cc5fa9e75a852233e0c8670882869e47755cc45856c5b956cce7d7370c"
m.LOGS = HERE
BACKUP = pathlib.Path("/tmp/claude-0/-home-user-Shamble-Frontlines/077a208e-2c05-50f2-8333-d6ac6effb7f2/scratchpad")


def wait_quiet(quiet_s, limit_s=240):
    """Returns once no cargo/rustc/rustdoc has run anywhere for quiet_s seconds in a row; exits after limit_s."""
    t0 = time.time()
    calm = 0
    while calm < quiet_s:
        if time.time() - t0 > limit_s:
            sys.exit(f"STOP: no {quiet_s} s of quiet in {limit_s} s (other lanes building); nothing planted")
        calm = 0 if m.foreign(None) else calm + 1
        time.sleep(1)
    return round(time.time() - t0)


m.wait_idle = lambda: f"10 s quiet after {wait_quiet(10)} s"

m.PLANTS.update({
    # Round 1's gaps are re-planted with round 1's exact edits (already in m.PLANTS, anchors unchanged in the fixed
    # lib.rs): next-event-none, next-event-no-timers, next-event-no-ready, no-packet-threshold, packet-threshold-2,
    # time-threshold-srtt-only, acks-lost-at-profile-rate, event-order-link-first, pto-from-oldest-unacked.
    # New plants, not the fixer's.
    "m1-gate-blind": ("", [
        (LIB, ".map(|r| r.0.arrival_ns.max(r.0.gate_ns.saturating_add(1)))", ".map(|r| r.0.arrival_ns)", 1),
    ], "the host test fails: next_event_ns names the send instant of a same-instant item (P0), not gate + 1"),
    "m1-late-1ns": ("", [
        (LIB, "(Some(a), Some(b)) => Some(a.min(b)),\n            (a, b) => a.or(b),",
         "(Some(a), Some(b)) => Some(a.min(b) + 1),\n            (a, b) => a.or(b).map(|x| x + 1),", 1),
    ], "L-c5 and the host test fail: next_event_ns names each instant 1 ns late"),
    "m2-off-by-one": ("", [
        (LIB, "let by_count = largest >= pn + PACKET_THRESHOLD;", "let by_count = largest > pn + PACKET_THRESHOLD;", 1),
    ], "the packet-threshold test fails (declared at the fourth later packet's acknowledgement, 1,104 ms)"),
    "l1-latest-only": ("", [
        (LIB, "(self.smoothed_ns().max(self.latest) * 9 / 8)", "(self.latest * 9 / 8)", 1),
    ], "the time-threshold test fails in a trial with a step down"),
    "l1-time-5-4": ("", [
        (LIB, "(self.smoothed_ns().max(self.latest) * 9 / 8)", "(self.smoothed_ns().max(self.latest) * 5 / 4)", 1),
    ], "the time-threshold test fails (kTimeThreshold 5/4, not 9/8)"),
    "l2-acks-never-lost": ("", [
        (LIB, "let lost = l.profile.in_burst(t);", "let lost = false;", 1),
    ], "acknowledgements_are_lost_only_inside_bursts fails (acknowledgements sent in the burst delivered)"),
    "l4-send-no-restart": ("", [
        (LIB, "            self.set_timer(link, dir, 0, now_ns);\n        }\n        self.leave(now_ns);\n        SendResult::Sent",
         "        }\n        self.leave(now_ns);\n        SendResult::Sent", 1),
    ], "streams_lose_nothing_... fails on its new check: a reliable send no longer restarts the PTO"),
})


def check_run(label):
    """One counted check.mjs run: 20 s of quiet first, touch before and every 2 s, foreign processes recorded."""
    m.check_all(f"before {label}")
    df = subprocess.run(["df", "-h", "/home/user"], capture_output=True, text=True).stdout.strip().splitlines()[-1]
    avail = subprocess.run(["df", "--output=avail", "-BG", "/home/user"], capture_output=True, text=True).stdout
    if int(re.sub(r"\D", "", avail.splitlines()[-1])) < 2:
        sys.exit(f"STOP: below 2 GB free ({df})")
    waited = wait_quiet(20)
    seen = []
    stop = threading.Event()
    m.touch_all()
    start = time.strftime("%H:%M:%SZ", time.gmtime())
    proc = subprocess.Popen(["node", "spikes/netcode-proof/check.mjs"], cwd=m.WT, env=m.ENV,
                            stdout=subprocess.PIPE, stderr=subprocess.STDOUT, text=True)

    def toucher():
        while not stop.is_set():
            m.touch_all()
            for p in m.foreign(proc.pid):
                seen.append((time.strftime("%H:%M:%SZ", time.gmtime()), p))
            stop.wait(2)

    th = threading.Thread(target=toucher)
    th.start()
    out, _ = proc.communicate()
    stop.set()
    th.join()
    end = time.strftime("%H:%M:%SZ", time.gmtime())
    m.check_all(f"after {label}")
    (HERE / f"{label}.log").write_text(out)
    fl = [f"{w} pid {p[0]} {p[2]} {p[3]}" for w, p in seen]
    (HERE / f"{label}.foreign.log").write_text("\n".join(fl) + "\n")
    print(f"== {label} [{start}-{end}; {df}; 20 s quiet after {waited} s; exit {proc.returncode}]")
    for line in out.splitlines()[-40:]:
        print(f"   {line}")
    print(f"   foreign cargo/rustc samples during the run: {len(fl)}")
    for f in fl[:12]:
        print(f"   foreign: {f}")


if __name__ == "__main__":
    names = sys.argv[2:]
    for n in names:
        if n != "check" and n not in m.PLANTS:
            sys.exit(f"no plant {n}: {sorted(m.PLANTS)}")
    m.check_all("start")
    for p in m.SHA:
        b = BACKUP / f"u4c-r2-backup-{p.parent.name}-{p.name}"
        if not b.exists():
            b.write_bytes(p.read_bytes())
        if hashlib.sha256(b.read_bytes()).hexdigest() != m.SHA[p]:
            sys.exit(f"ABORT: backup {b} differs")
    for n in names:
        if n == "check":
            check_run("r2-check")
        else:
            m.run(n)
