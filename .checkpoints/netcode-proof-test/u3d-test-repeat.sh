#!/usr/bin/env bash
# U3d build proof: T1, T2 and T7 (the live tests) repeated, alone and inside npt-net's whole lib suite, to look for a
# flake. Shared folder, touch first, df first. Run: bash <this> <spike dir>
set -uo pipefail
SPIKE="$1"
export CARGO_TARGET_DIR=/home/user/Shamble-Frontlines/.gate/shared-target
df -h /home/user | tail -1
pgrep -a cargo && { echo "another cargo runs: stop"; exit 2; }
find "$SPIKE/crates" -type f -exec touch {} +
BIN="$(cd "$SPIKE" && cargo +1.99.1 test --locked -p npt-net --lib --no-run --message-format=json 2>/dev/null | python3 -I -c '
import json, sys
exe = [m["executable"] for m in map(json.loads, filter(None, sys.stdin.read().splitlines()))
       if m.get("reason") == "compiler-artifact" and m.get("executable") and m["target"]["name"] == "npt_net" and m["profile"]["test"]]
assert len(exe) == 1, exe
print(exe[0])')"
echo "debug binary $BIN $(stat -c %y "$BIN")"
P=l2::socket::tests::
fails=0
for i in $(seq 1 30); do
  out="$("$BIN" "${P}the_task_poller_takes_lateness_where_the_task_runs" "${P}the_fate_log_and_the_histogram_agree" "${P}a_held_lock_makes_both_directions_late" --exact 2>&1)"
  r="$(grep -o 'test result: .*' <<<"$out" | head -1)"
  grep -q '3 passed; 0 failed' <<<"$r" || { fails=$((fails+1)); echo "run $i: $r"; grep -A3 'panicked at' <<<"$out" | head -8; }
done
echo "alone, 3 tests x 30 runs: $fails failed runs"
fails=0
for i in $(seq 1 8); do
  out="$("$BIN" 2>&1)"
  r="$(grep -o 'test result: .*' <<<"$out" | head -1)"
  echo "suite run $i: $r"
  grep -q ' 0 failed' <<<"$r" || { fails=$((fails+1)); grep -B1 -A3 'panicked at' <<<"$out" | head -12; }
done
echo "whole npt-net lib suite x 8: $fails failed runs"
