"""U6c's probe reading (advisor-u6c-r1.md lines 73-81; reported, never judged). It starts the release npt-server in
probe mode with its stdin held open (the probe mode ends when stdin closes), waits for its OPEN line, then starts
npt-load in probe mode against it. It waits for both, and prints each one's exit code, its stdout lines, the tail of
its stderr and the wall times. It writes no file itself; the two programs write their own logs.
Usage: python3 -I probe_reading.py <npt-server> <server settings> <npt-load> <load settings>"""
import subprocess
import sys
import threading
import time

server_bin, server_settings, load_bin, load_settings = sys.argv[1:5]
t0 = time.monotonic()


def now():
    return round(time.monotonic() - t0, 3)


srv = subprocess.Popen([server_bin, "--settings", server_settings], stdin=subprocess.PIPE,
                       stdout=subprocess.PIPE, stderr=subprocess.PIPE, text=True)
out_lines = []
err_tail = []
opened = threading.Event()


def read_out():
    for line in srv.stdout:
        out_lines.append((now(), line.rstrip("\n")))
        if line.strip() == "OPEN":
            opened.set()


def read_err():
    for line in srv.stderr:
        err_tail.append(line.rstrip("\n"))
        del err_tail[:-40]


threading.Thread(target=read_out, daemon=True).start()
threading.Thread(target=read_err, daemon=True).start()
if not opened.wait(30):
    print(f"no OPEN within 30 s; server stdout {out_lines!r}; stderr tail {err_tail!r}")
    srv.kill()
    sys.exit(1)
print(f"server OPEN at {now()} s")
lg = subprocess.run([load_bin, load_settings], capture_output=True, text=True, timeout=900)
print(f"npt-load exit {lg.returncode} at {now()} s; stdout {lg.stdout!r}; stderr tail {lg.stderr[-2000:]!r}")
try:
    rc = srv.wait(timeout=120)
except subprocess.TimeoutExpired:
    print(f"the server still runs at {now()} s, 120 s after npt-load ended: its stdin is closed now")
    srv.stdin.close()
    rc = srv.wait(timeout=60)
time.sleep(1)
print(f"npt-server exit {rc} at {now()} s")
for t, line in out_lines:
    print(f"server stdout at {t} s: {line}")
for line in err_tail:
    print(f"server stderr: {line}")
sys.exit(0 if rc == 0 and lg.returncode == 0 else 1)
