"""Planted defects for the U3b fix pass (build agent). Works only on the throwaway copy under .gate/u3bfix-copy.

Usage: python3 -I u3bfix-plant.py <defect>... | all | restore
Each defect restores the originals first, then makes its textual changes (each must match exactly once), builds and
runs npt-net's l2 tests (release, all features) on the copy, and records the failing tests. The copy is restored at
the end. Results go to .gate/u3bfix-copy/results.txt.
"""
import pathlib
import re
import subprocess
import sys

COPY = pathlib.Path("/home/user/Shamble-Frontlines/.gate/u3bfix-copy/netcode-proof")
ROOT = COPY / "crates/net/src/l2"
TARGET = "/home/user/Shamble-Frontlines/.gate/u3bfix-copy-target"
RESULTS = COPY.parent / "results.txt"
FILES = ["impair.rs", "socket.rs", "wt.rs", "shaper.rs"]

DEFECTS = {
    # M4 (the skeptic's B): one stream for both directions of a peer.
    "B_stream_no_dir": [("impair.rs", "rng.set_stream(ordinal.wrapping_mul(2) | dir as u64);",
                         "rng.set_stream(ordinal.wrapping_mul(2)); let _ = dir;")],
    # M3 (A): the overtake no longer requires the successor to arrive before the held one is due.
    "A_no_due_guard": [("impair.rs", "                && t < h.due_ns\n", "                && t < u64::MAX\n")],
    # M3 variant: an overtake also at the due time itself (<= instead of <).
    "A2_due_guard_le": [("impair.rs", "                && t < h.due_ns\n", "                && t <= h.due_ns\n")],
    # M1 (D): inbound lateness taken as zero (not measured where quinn receives it).
    "D_inbound_late_zero": [("socket.rs", "st.im.finish(&e, Kind::Delivered, now, 0);",
                             "st.im.finish(&e, Kind::Delivered, e.due_ns, 0);")],
    # M19 (G): the shaper after the profile's delay instead of before it.
    "G_shaper_after_delay": [("impair.rs", "                    Some(sh) => sh.offer(t, bytes),\n",
                              "                    Some(sh) => sh.offer(t + s.delay(w[W_JITTER]), bytes)"
                              ".map(|d| d - s.delay(w[W_JITTER])),\n")],
    # M10 (H): evict the MOST recently seen unpinned peer.
    "H_evict_newest": [("impair.rs", "            .min();\n", "            .max();\n")],
    # Drop (K): Drop no longer stops the reader thread.
    "K_reader_not_stopped": [("socket.rs",
                              "        self.shared.cv.notify_all();\n        self.shared.reader_stop.notify_one();\n    }",
                              "        self.shared.cv.notify_all();\n    }")],
    # Item 3: the byte bounds charge the payload, not the buffer (the pre-fix accounting).
    "P_charge_payload": [
        ("impair.rs", "        } else if flow_bytes + CHARGE_BYTES > self.peer_queue_bytes\n"
                      "            || self.queued[dir] + CHARGE_BYTES > self.total_queue_bytes\n",
         "        } else if flow_bytes + len as u64 > self.peer_queue_bytes\n"
         "            || self.queued[dir] + len as u64 > self.total_queue_bytes\n"),
        ("impair.rs", "        self.peer_mut(slot).flows[dir].queued_bytes += CHARGE_BYTES;\n"
                      "        self.queued[dir] += CHARGE_BYTES;\n",
         "        self.peer_mut(slot).flows[dir].queued_bytes += bytes;\n        self.queued[dir] += bytes;\n"),
        ("impair.rs", "        let bytes = CHARGE_BYTES;\n", "        let bytes = u64::from(e.len);\n"),
    ],
    # Item 5: the peer is placed before the host checks (the pre-fix order).
    "S_place_before_checks": [("impair.rs", "        let target = self.target(key);\n        let (known, flow_bytes)",
                               "        let target = self.target(key);\n"
                               "        let target = Target::Known(self.place(key, target, t));\n"
                               "        let (known, flow_bytes)")],
    # Item 6: a changed shaper starts empty (the pre-fix apply).
    "T1_fresh_shaper": [("impair.rs", ".map(|(r, q)| Shaper::busy_until(r, q, f.last_depart_ns));",
                         ".map(|(r, q)| Shaper::new(r, q));")],
    # Item 6: without a shaper a datagram departs at its intake, even right after a throttle ends.
    "T2_no_floor": [("impair.rs", "None => Some(t.max(f.last_depart_ns)),", "None => Some(t),")],
    # Item 2: the header reports the slack read at the thread's start.
    "L_header_start_slack": [("socket.rs", "(\"l2_timer_slack_ns\", slack_now.unwrap_or(u64::MAX)),",
                              "(\"l2_timer_slack_ns\", info.slack.read_ns.unwrap_or(u64::MAX)),")],
    # Item 4: wtransport's ApplicationClosed counted as the QUIC stage (the pre-fix mapping).
    "W_appclose_quic": [("wt.rs", "            wtransport::error::ConnectionError::LocalH3Error(_)\n"
                                  "            | wtransport::error::ConnectionError::ApplicationClosed(_) => Failure::H3,\n",
                         "            wtransport::error::ConnectionError::LocalH3Error(_) => Failure::H3,\n")],
    # The skeptic's biting defects, re-run on the new code.
    "C_send_not_try_send": [("socket.rs", "let r = sh.udp.try_send((&sh.send_sock).into(), &t);",
                             "let r = sh.udp.send((&sh.send_sock).into(), &t);")],
    "I_fifo_order": [("impair.rs", "        (self.due_ns, self.seq).cmp(&(other.due_ns, other.seq))",
                      "        self.seq.cmp(&other.seq)")],
    "J_no_peer_bound": [("impair.rs", "        } else if flow_bytes + CHARGE_BYTES > self.peer_queue_bytes\n",
                         "        } else if flow_bytes + CHARGE_BYTES > u64::MAX - 1\n")],
}


def restore():
    for f in FILES:
        orig = ROOT / (f + ".orig")
        if orig.exists():
            (ROOT / f).write_text(orig.read_text())


def plant(name):
    for f in FILES:
        orig = ROOT / (f + ".orig")
        if not orig.exists():
            orig.write_text((ROOT / f).read_text())
    restore()
    for f, old, new in DEFECTS[name]:
        p = ROOT / f
        text = p.read_text()
        n = text.count(old)
        if n != 1:
            sys.exit(f"{name}: pattern found {n} times in {f}")
        p.write_text(text.replace(old, new))


def run_tests():
    r = subprocess.run(
        ["cargo", "+1.99.1", "test", "--release", "-p", "npt-net", "--all-features", "--target-dir", TARGET,
         "--", "l2::"],
        cwd=COPY, capture_output=True, text=True)
    out = r.stdout + r.stderr
    failed = re.findall(r"^test (\S+) \.\.\. FAILED$", out, re.M)
    summary = re.findall(r"^test result: .*$", out, re.M)
    if "error[" in out or "could not compile" in out:
        return ["(build failed)"], out[-3000:]
    return failed, (summary[0] if summary else out[-2000:])


def main():
    args = sys.argv[1:]
    if args == ["restore"]:
        restore()
        print("restored")
        return
    names = list(DEFECTS) if args == ["all"] else args
    lines = []
    for name in names:
        plant(name)
        failed, summary = run_tests()
        line = f"{name}: {'BIT' if failed else 'SURVIVED'} {failed} | {summary}"
        print(line, flush=True)
        lines.append(line)
    restore()
    # The clean copy passes (the baseline for every row above).
    failed, summary = run_tests()
    lines.append(f"restored copy: {'FAILS ' + str(failed) if failed else 'passes'} | {summary}")
    print(lines[-1], flush=True)
    RESULTS.write_text("\n".join(lines) + "\n")


main()
