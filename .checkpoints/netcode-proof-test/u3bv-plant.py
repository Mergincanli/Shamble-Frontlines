"""Planted-defect helper for the U3b skeptic (rust-reviewer). Works only on the throwaway copy under .gate/u3bv-copy.

Usage: python3 -I u3bv-plant.py <defect>   (or "restore")
Each defect restores the originals first, then makes exactly one textual change (it must match exactly once).
"""
import pathlib
import sys

ROOT = pathlib.Path("/home/user/Shamble-Frontlines/.gate/u3bv-copy/netcode-proof/crates/net/src/l2")
FILES = ["impair.rs", "socket.rs", "impair_tests.rs", "shaper.rs"]

DEFECTS = {
    # M4: one stream for both directions of a peer (up and down fates correlated).
    "B_stream_no_dir": ("impair.rs", "rng.set_stream(ordinal.wrapping_mul(2) | dir as u64);",
                        "rng.set_stream(ordinal.wrapping_mul(2)); let _ = dir;"),
    # M3: the overtake no longer requires the successor to arrive before the held one is due.
    "A_no_due_guard": ("impair.rs", "                && t < h.due_ns\n", "                && t < u64::MAX\n"),
    # M1: inbound lateness taken as zero (not measured where quinn receives it).
    "D_inbound_late_zero": ("socket.rs", "st.im.finish(&e, Kind::Delivered, now, 0);",
                            "st.im.finish(&e, Kind::Delivered, e.due_ns, 0);"),
    # M19: the shaper after the profile's delay instead of before it.
    "G_shaper_after_delay": ("impair.rs",
                             "                let depart = match f.shaper.as_mut() {\n"
                             "                    Some(sh) => sh.offer(t, bytes),\n"
                             "                    None => Some(t),\n"
                             "                };",
                             "                let dl = s.delay(w[W_JITTER]);\n"
                             "                let depart = match f.shaper.as_mut() {\n"
                             "                    Some(sh) => sh.offer(t + dl, bytes).map(|d| d - dl),\n"
                             "                    None => Some(t),\n"
                             "                };"),
    # M10: evict the MOST recently seen unpinned peer.
    "H_evict_newest": ("impair.rs", "                    .min();\n                let Some((_, v)) = victim",
                       "                    .max();\n                let Some((_, v)) = victim"),
    # M5: the release thread uses quinn-udp's send (swallows errors) instead of try_send.
    "C_send_not_try_send": ("socket.rs", "let r = sh.udp.try_send((&sh.send_sock).into(), &t);",
                            "let r = sh.udp.send((&sh.send_sock).into(), &t);"),
    # M2: release order by sequence only (a FIFO across peers), ignoring due time in the heap order.
    "I_fifo_order": ("impair.rs", "        (self.due_ns, self.seq).cmp(&(other.due_ns, other.seq))",
                     "        self.seq.cmp(&other.seq)"),
    # M10: no byte bound per peer (only the total).
    "J_no_peer_bound": ("impair.rs", "        } else if flow_bytes + bytes > self.peer_queue_bytes\n",
                        "        } else if flow_bytes + bytes > u64::MAX - 1\n"),
    # Drop no longer stops the reader thread.
    "K_reader_not_stopped": ("socket.rs", "        self.shared.cv.notify_all();\n        self.shared.reader_stop.notify_one();\n    }",
                             "        self.shared.cv.notify_all();\n    }"),
}


def restore():
    for f in FILES:
        orig = ROOT / (f + ".orig")
        if orig.exists():
            (ROOT / f).write_text(orig.read_text())


def main():
    name = sys.argv[1]
    for f in FILES:
        orig = ROOT / (f + ".orig")
        if not orig.exists():
            orig.write_text((ROOT / f).read_text())
    restore()
    if name == "restore":
        print("restored")
        return
    f, old, new = DEFECTS[name]
    p = ROOT / f
    text = p.read_text()
    n = text.count(old)
    if n != 1:
        sys.exit(f"{name}: pattern found {n} times in {f}")
    p.write_text(text.replace(old, new))
    print(f"planted {name} in {f}")


main()
