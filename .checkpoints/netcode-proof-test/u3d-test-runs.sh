#!/usr/bin/env bash
# U3d build proof (test-engineer): the clean release build, then (a) the self-test at 10 s (plan §3.1 / §5.1 prints),
# (b) NPT_L2_SELFTEST_SECS bounds, (c) reviewers' F1: the recorded chrt switch applied to the release thread after it
# started, and the whole process born under chrt. Shared target (§7.1), touch first, df first. Outputs go next to this
# script. Run: bash <this> <spike dir> <out prefix>
set -uo pipefail
SPIKE="$1"
OUT="$2"
export CARGO_TARGET_DIR=/home/user/Shamble-Frontlines/.gate/shared-target
T=l2::socket::tests::l2_timing_self_test
df -h /home/user | tail -1
pgrep -a cargo && { echo "another cargo runs: stop"; exit 2; }
find "$SPIKE/crates" -type f -exec touch {} +
JSON="$(cd "$SPIKE" && cargo +1.99.1 test --release --locked -p npt-net --lib --no-run --message-format=json 2>/dev/null)"
BIN="$(printf '%s\n' "$JSON" | python3 -I -c '
import json, sys
exe = [m["executable"] for m in map(json.loads, filter(None, sys.stdin.read().splitlines()))
       if m.get("reason") == "compiler-artifact" and m.get("executable") and m["target"]["name"] == "npt_net" and m["profile"]["test"]]
assert len(exe) == 1, exe
print(exe[0])')"
echo "binary $BIN $(stat -c %y "$BIN") sha256 $(sha256sum "$BIN" | cut -c1-16)"
echo "listed (not ignored): $("$BIN" --list 2>/dev/null | grep -c ': test$'); ignored: $("$BIN" --ignored --list 2>/dev/null | grep ': test$' | tr '\n' ' ')"
for t in the_task_poller_takes_lateness_where_the_task_runs the_fate_log_and_the_histogram_agree \
  the_verdict_line_is_per_direction_and_inclusive nearest_rank_and_loss_band_on_fixed_inputs \
  the_exact_excess_on_fixed_samples the_cross_check_reads_delay_plus_lateness a_held_lock_makes_both_directions_late; do
  "$BIN" --list 2>/dev/null | grep -qx "l2::socket::tests::$t: test" && echo "listed $t" || echo "NOT LISTED $t"
done

echo "=== (a) self-test, 10 s"
NPT_L2_SELFTEST_SECS=10 "$BIN" --ignored --nocapture --exact "$T" > "$OUT-selftest10.txt" 2>&1
echo "exit $?" >> "$OUT-selftest10.txt"
tail -3 "$OUT-selftest10.txt"

echo "=== (b) bounds"
for v in 0 601 abc "" -5 1.5 " 10"; do
  out="$(NPT_L2_SELFTEST_SECS="$v" timeout 20 "$BIN" --ignored --nocapture --exact "$T" 2>&1)"
  st=$?
  echo "value [$v]: exit $st; $(grep -o 'INVALID: NPT_L2_SELFTEST_SECS=[^;]*' <<<"$out" | head -1); prints $(grep -c '^  \|^U3d self-test' <<<"$out")"
done
out="$(NPT_L2_SELFTEST_SECS=$'\xff' timeout 5 "$BIN" --ignored --nocapture --exact "$T" 2>&1)"
st=$?
echo "value [\\xff] (not UTF-8): exit $st (124 = still running at 5 s: taken as the 100 s default); INVALID printed: $(grep -c INVALID <<<"$out")"

echo "=== (c1) chrt -f -p 1 on npt-l2-release about 1 s into a 6 s run"
NPT_L2_SELFTEST_SECS=6 "$BIN" --ignored --nocapture --exact "$T" > "$OUT-chrt-after.txt" 2>&1 &
PID=$!
tid=""
for _ in $(seq 1 200); do
  for f in /proc/$PID/task/*/comm; do
    if [ "$(cat "$f" 2>/dev/null)" = npt-l2-release ]; then tid="$(basename "$(dirname "$f")")"; fi
  done
  [ -n "$tid" ] && break
  sleep 0.01
done
sleep 1
echo "release thread tid $tid; before: $(chrt -p "$tid" 2>&1 | tr '\n' ' ')"
chrt -f -p 1 "$tid" && echo "after: $(chrt -p "$tid" 2>&1 | tr '\n' ' '); its slack now $(cat /proc/$tid/timerslack_ns 2>&1)"
wait "$PID"
echo "exit $?" >> "$OUT-chrt-after.txt"
grep -E 'slack now|panicked|INVALID|left|right|exit' "$OUT-chrt-after.txt" | head -8

echo "=== (c2) the whole process born under chrt -f 1, 3 s"
NPT_L2_SELFTEST_SECS=3 chrt -f 1 "$BIN" --ignored --nocapture --exact "$T" > "$OUT-chrt-born.txt" 2>&1
echo "exit $?" >> "$OUT-chrt-born.txt"
grep -E 'slack now|panicked|INVALID|left|right|exit' "$OUT-chrt-born.txt" | head -8
echo DONE
