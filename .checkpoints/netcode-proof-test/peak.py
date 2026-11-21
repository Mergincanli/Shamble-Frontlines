"""Run a command, then print its wall time, its exit and the peak resident memory of the largest process in its tree.
The peak is resource.getrusage(RUSAGE_CHILDREN).ru_maxrss: on Linux, KiB, the largest waited-for descendant. That is
what the round-2 ruling's step 17 asks for (advisor-u4r-r2-full.md:23), since /usr/bin/time is not installed here.
The command's own output passes through unchanged. The one line this script adds comes last. It writes no file and
exits with the command's exit code (128 + n for a signal n).
Usage: python3 -I peak.py <command...>"""
import resource
import subprocess
import sys
import time

if len(sys.argv) < 2:
    sys.exit("usage: peak.py <command...>")
t0 = time.monotonic()
r = subprocess.run(sys.argv[1:])
wall = time.monotonic() - t0
kib = resource.getrusage(resource.RUSAGE_CHILDREN).ru_maxrss
print(f"peak.py: wall {wall:.1f} s; exit {r.returncode}; peak resident memory of the largest process "
      f"(getrusage RUSAGE_CHILDREN ru_maxrss) {kib} KiB = {kib / 1024:.1f} MiB", flush=True)
sys.exit(r.returncode if r.returncode >= 0 else 128 - r.returncode)
