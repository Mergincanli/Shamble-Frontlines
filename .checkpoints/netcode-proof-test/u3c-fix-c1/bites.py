"""U3c fix C1's bites: each mutation applied to the worktree's source, its test run, the file restored (always).

Usage: python3 bites.py <spike dir> <out file>. A bite is shown when its test FAILS with the mutation in place.
"""
import os
import pathlib
import subprocess
import sys

spike = pathlib.Path(sys.argv[1])
out = pathlib.Path(sys.argv[2])
src = spike / "crates/net/src"
env = dict(os.environ, CARGO_TARGET_DIR="/home/user/Shamble-Frontlines/.gate/u3v3-harness-check-target")

M = [
    ("F1: no range refusal", "l2/delay.rs",
     ".filter(|&s| s > now && start_ns.checked_add(len_ns).is_some());",
     ".filter(|&s| s > now);",
     "a_burst_past_the_clocks_range"),
    ("NC-U3c-8: start at the model's now accepted", "l2/delay.rs",
     ".filter(|&s| s > now && start_ns.checked_add(len_ns).is_some());",
     ".filter(|&s| s >= now && start_ns.checked_add(len_ns).is_some());",
     "a_burst_past_the_clocks_range"),
    ("F2: current()'s wakes before the unlock", "l2/delay.rs",
     "struct Stepped<'a> {\n    c: MutexGuard<'a, Conn>,\n    _wakes: Wakes,\n}",
     "struct Stepped<'a> {\n    _wakes: Wakes,\n    c: MutexGuard<'a, Conn>,\n}",
     "the_pump_never_calls_s_under_the_lock"),
    ("F2: poll_shutdown wakes under the lock", "l2/delay.rs",
     "        drop(c);\n        if let Some(p) = pump {\n            p.wake();\n        }\n        Poll::Pending\n    }\n}\n\n/// The connection's pump",
     "        if let Some(p) = pump {\n            p.wake();\n        }\n        drop(c);\n        Poll::Pending\n    }\n}\n\n/// The connection's pump",
     "the_pump_never_calls_s_under_the_lock"),
    ("F3: S's write under the lock", "l2/delay.rs",
     "            let r = Pin::new(&mut *s).poll_write(cx, &this.scratch[this.sent..this.staged]);",
     "            let g = sh.lock();\n            let r = Pin::new(&mut *s).poll_write(cx, &this.scratch[this.sent..this.staged]);\n            drop(g);",
     "the_pump_never_calls_s_under_the_lock"),
    ("F4: the record copied again on every attempt", "l2/delay.rs",
     "            if this.staged == 0 {\n                let c = sh.lock();",
     "            if this.staged == 0 || this.sent < this.staged {\n                let c = sh.lock();",
     "the_pump_never_calls_s_under_the_lock"),
    ("F-R2: no read floor", "l2/delay.rs",
     "if c.up_eof || c.parse_len >= limit || c.up_room() < c.read_floor {",
     "if c.up_eof || c.parse_len >= limit {",
     "the_pump_never_calls_s_under_the_lock"),
    ("F5: a zero-capacity read waits", "l2/delay.rs",
     "if buf.remaining() == 0 && c.released[UP] > 0 {",
     "if buf.remaining() == 0 && c.released[UP] > usize::MAX - 1 {",
     "the_request_is_one_record"),
    ("S2: no bare-LF end", "l2/ws_frame.rs",
     "(Line::Start | Line::StartCr, b'\\n') => (Line::Start, true),",
     "(Line::StartCr, b'\\n') => (Line::Start, true),",
     "l2::ws_frame::tests::the_head"),
    ("S2: leading empty lines not skipped", "l2/ws_frame.rs",
     "(Line::Leading | Line::LeadingCr, b'\\n') => (Line::Leading, false),",
     "(Line::Leading | Line::LeadingCr, b'\\n') => (Line::Start, false),",
     "l2::ws_frame::tests::the_head"),
    ("F-B: add_burst without the origin (burst test alone)", "l2/delay.rs",
     "            .checked_sub(c.origin)\n            .filter(",
     "            .checked_sub(0)\n            .filter(",
     "an_added_burst_loses_every_transmission"),
    ("F-A: the profile's burst ends left on the clock's axis", "l2/delay.rs",
     "burst_ends.push(end - origin);",
     "burst_ends.push(end);",
     "wrap_moves_the_profiles_bursts"),
    ("F-G: EOF while records wait", "l2/delay.rs",
     "if c.pump_done || (c.up_eof && c.bounds.is_empty() && c.recs[UP].is_empty()) {",
     "if c.pump_done || c.up_eof {",
     "an_eof_from_s_ends_the_up_direction"),
    ("F7: the sparse mark never marks a bucket touched", "l2/stats.rs",
     "                    self.touched[w] |= bit;\n",
     "",
     "a_running_window_equals"),
    ("F7: a window stays dense after it closes", "l2/stats.rs",
     "        self.sparse.clear();\n        self.dense_on = false;",
     "        self.sparse.clear();",
     "a_running_window_equals"),
]

lines = []
for name, rel, old, new, test in M:
    f = src / rel
    orig = f.read_text()
    if orig.count(old) != 1:
        lines.append(f"SKIP  {name}: the snippet is not there exactly once")
        continue
    try:
        f.write_text(orig.replace(old, new))
        r = subprocess.run(
            ["cargo", "+1.99.1", "test", "--locked", "-p", "npt-net", "--lib", test],
            cwd=spike, env=env, capture_output=True, text=True, timeout=900,
        )
        text = r.stdout + r.stderr
        failed = r.returncode != 0
        why = ""
        for ln in text.splitlines():
            if "panicked at" in ln or "error[" in ln:
                why = ln.strip()[:160]
                break
        lines.append(f"{'BITES' if failed else 'MISSED'}  {name} ({test}): {why}")
    finally:
        f.write_text(orig)
    out.write_text("\n".join(lines) + "\n")
lines.append("DONE")
out.write_text("\n".join(lines) + "\n")
print("\n".join(lines))
