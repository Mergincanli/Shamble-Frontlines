#!/usr/bin/env bash
# U3c's kernel cross-check (u3c-plan.md 1.6, C11; the advisor's S13): the Linux-faithful TCP model against this
# container's real kernel on loopback, on the scout's pattern (u3-tcp-stall-probe.py). An evidence run outside
# check.mjs: it needs root and iptables. The main session runs it after the merge, one evidence run at a time, with
# nothing else on loopback; a build agent never runs it. R1's S14 check on RS (veth + netem, kernel 6.8) stays the
# binding one: this container runs 6.18.44, not 6.8, and has no tc.
#
# ---- Fixed in writing before any run (C11; the advisor's side rulings of 2026-11-08) ----------------------------
# PATTERN. Loopback TCP from a client to a server on PORT (default 47999), 200 B messages every 33,333,333 ns
#   (30 Hz), TCP_NODELAY on both sockets, TCP_CONGESTION set to cubic on both probe sockets and the listener (this
#   container defaults to bbr, u3-scout-timing.md:388; if cubic cannot be set, the congestion control read back is
#   recorded and the verdict says so). 3,000 messages per run, 3 runs. Loss: one iptables rule, inserted at the top of
#   INPUT so no earlier rule can accept the packets first,
#     iptables -I INPUT 1 -i lo -p tcp --dport PORT -m statistic --mode random --probability 0.02 -j DROP
#   so 2 % of client-to-server packets of the probe's port only (data; the server's ACKs come from PORT and pass).
#   The model (tcp_model::tests::kernel_cross_check_model, release build): the same pattern, 100 seeds, no jitter,
#   loss 2 % on the data direction only (the cfg(test) knob with_loss_from), at the loopback's measured RTT: the mean
#   over the 3 runs of the client's tcpi_rtt read at the run's end.
# DELAY. A message's delay is its arrival at the server minus its send stamp (both CLOCK_MONOTONIC in one process; the
#   model: its in-order release minus its write). BASE is the run's median delay (nearest rank, the ceil(n/2)-th).
# STALL (judged). An episode is a maximal run of consecutive messages each more than HALF A SEND GAP (16,666,666 ns)
#   above BASE; its length L is its first message's delay minus BASE (the first delayed message per stall, as the
#   scout counted; the messages queued behind it are the same episode). The plan's rule was "more than one send gap"
#   (C11); the advisor's ruling of 2026-11-08, made before any run, replaced it with half a gap: the scout's RACK
#   cluster sits AT one gap (45 stalls, p50 33.5 ms against a 33.3 ms gap, u3-scout-timing.md:392-393), since a RACK
#   repair's L on loopback is one gap plus the RTT (about 0.1-0.2 ms) plus the difference of two sends' pacing
#   lateness (tens of µs, either sign), so a one-gap threshold would cut that cluster in two; half a gap is far from
#   both the no-loss delays (under 1 ms) and the shortest repair (one gap). The count at one gap is reported beside
#   the judged one, never judged.
# REPAIR PATH. An episode is RACK if L <= RTT + gap + 2 ms + J, with RTT the measured RTT above, gap 33,333,333 ns,
#   2 ms RACK's reorder-timer slop (TCP_TIMEOUT_MIN) and J = 1 ms the jitter allowance for the sender's pacing; else it
#   is the probe-or-timeout path (on loopback the probe's timer, 2 x srtt + 200 ms with one segment out, is capped by
#   the RTO, so these are timeouts; the model's own counters tell probe from timeout on the same pattern and are
#   reported). Two messages lost in a row also land above the bound (their episode is two gaps long), on both sides
#   alike: the rule is applied to the kernel's and the model's delays by the same code (compare.py), so the shares
#   compare like with like.
# A KERNEL RUN IS (compare.py applies these before any judgement):
#   INVALID if the probe's send lateness (send stamp minus due time) has p99 > 1 ms (J would not hold), or it did not
#     deliver all 3,000 messages;
#   VOID if the kernel retransmitted nothing in it (the client's tcpi_total_retrans is 0: no loss happened, so there is
#     nothing to compare; never a MISS), or if its judged episodes outnumber the client's tcpi_total_retrans (every
#     real stall needs at least one retransmission, so the excess is receiver-side scheduling noise: the advisor's
#     noise guard, 2026-11-08). The system-wide counters (/proc/net/snmp Tcp RetransSegs; /proc/net/netstat TcpExt
#     TCPFastRetrans, TCPLossProbes, TCPTimeouts, TCPLostRetransmit), read before and after each run, are recorded
#     beside them, not judged. (Linux names RetransSegs in /proc/net/snmp; /proc/net/netstat has no TCPRetransSegs.)
#   The verdict is INVALID if any run is INVALID (or the model gave no runs), else VOID if any run is VOID, else
#   judged:
# PASS (both):
#   (1) the RACK-path stall length's p50, kernel (3 runs pooled) against model (100 seeds pooled): within 2 ms;
#   (2) the timeout-path share: the kernel's count of probe-or-timeout episodes k out of its n episodes lies inside the
#       99 % two-sided band of Binomial(n, p), p the model's pooled share (the band: the smallest lo and hi with
#       P(X < lo) <= 0.005 and P(X > hi) <= 0.005).
# RERUNS (the advisor, 2026-11-08: no re-running until it passes). Every run keeps its own output, out-<UTC stamp>/
#   in this folder, never overwritten, and is committed. A MISS is never re-run: it goes to the advisor before U3c
#   closes. A VOID or INVALID verdict may be re-run once, on a quiet container (the load average is recorded); a second
#   VOID or INVALID goes to the advisor. A PASS is final. This script enforces it from the earlier out-*/verdict.json
#   files: it refuses to start after a PASS or a MISS, or after two VOID or INVALID verdicts. An earlier folder with no
#   verdict (a run that broke off) blocks it too, until a file named ABORTED in that folder says why (then that folder
#   is set aside).
# RECORDED: uname -r, CONFIG_HZ (from /proc/config.gz), and the sysctls tcp_recovery, tcp_early_retrans, tcp_sack,
#   tcp_frto, tcp_rto_min_us, tcp_thin_linear_timeouts, tcp_timestamps, tcp_dsack, tcp_congestion_control,
#   tcp_available_congestion_control; /proc/loadavg before and after each run.
# SAFETY: the rule matches only PORT on lo; it is inserted before each run and deleted after it, and a trap deletes
#   any copy left on every exit; `iptables -S` is saved before and after and must read the same (else exit 3,
#   loudly); the script refuses to start if a rule on PORT already exists or PORT is in use.
# ------------------------------------------------------------------------------------------------------------------
#
# Use (root, from anywhere; cargo +1.99.1 on PATH; CARGO_TARGET_DIR as the session sets it):
#   bash spikes/netcode-proof/results/u3c-kernel-check/run.sh
# Output: this folder's out-<UTC stamp>/: kernel.txt, iptables-before.txt, iptables-after.txt, kernel-run{1,2,3}.json,
# model.txt, verdict.txt and verdict.json.

set -euo pipefail

HERE="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
SPIKE="$(cd "$HERE/../.." && pwd)"
PORT="${PORT:-47999}"
MESSAGES=3000
RUNS=3
MODEL_RUNS=100
OUT="$HERE/out-$(date -u +%Y%m%dT%H%M%SZ)"

if [ "$(id -u)" -ne 0 ]; then
  echo "run.sh: needs root for iptables" >&2
  exit 2
fi
command -v iptables >/dev/null || { echo "run.sh: no iptables" >&2; exit 2; }
command -v python3 >/dev/null || { echo "run.sh: no python3" >&2; exit 2; }

# The rerun rule, from the earlier verdicts (the header's RERUNS).
python3 -I "$HERE/compare.py" may-run "$HERE"

if [ -e "$OUT" ]; then
  echo "run.sh: $OUT exists already; never overwritten" >&2
  exit 2
fi
mkdir "$OUT"

RULE=(-i lo -p tcp --dport "$PORT" -m statistic --mode random --probability 0.02 -j DROP)

iptables -S > "$OUT/iptables-before.txt"
if grep -q -- "--dport $PORT " "$OUT/iptables-before.txt"; then
  echo "run.sh: a rule on port $PORT exists already; refusing" >&2
  echo "refused: a rule on port $PORT existed" > "$OUT/ABORTED"
  exit 2
fi
if ! python3 -I -c 'import socket,sys; s=socket.socket(); s.bind(("127.0.0.1", int(sys.argv[1]))); s.close()' "$PORT"; then
  echo "run.sh: port $PORT is in use; refusing" >&2
  echo "refused: port $PORT was in use" > "$OUT/ABORTED"
  exit 2
fi

cleanup() {
  while iptables -D INPUT "${RULE[@]}" 2>/dev/null; do :; done
  iptables -S > "$OUT/iptables-after.txt"
  if ! diff -u "$OUT/iptables-before.txt" "$OUT/iptables-after.txt"; then
    echo "run.sh: iptables -S DIFFERS after the run: check the firewall now" >&2
    exit 3
  fi
}
trap cleanup EXIT
trap 'exit 130' INT TERM

{
  echo "uname_r $(uname -r)"
  if [ -r /proc/config.gz ]; then
    echo "config_hz $(zcat /proc/config.gz | grep -E '^CONFIG_HZ=' | cut -d= -f2)"
  else
    echo "config_hz unknown"
  fi
  for s in tcp_recovery tcp_early_retrans tcp_sack tcp_frto tcp_rto_min_us tcp_thin_linear_timeouts tcp_timestamps \
           tcp_dsack tcp_congestion_control tcp_available_congestion_control; do
    if [ -r "/proc/sys/net/ipv4/$s" ]; then
      echo "$s $(cat "/proc/sys/net/ipv4/$s")"
    else
      echo "$s absent"
    fi
  done
  echo "port $PORT"
  echo "started_utc $(date -u +%Y-%m-%dT%H:%M:%SZ)"
} > "$OUT/kernel.txt"
cat "$OUT/kernel.txt"

for run in $(seq 1 "$RUNS"); do
  echo "loadavg_before_run$run $(cat /proc/loadavg)" >> "$OUT/kernel.txt"
  iptables -I INPUT 1 "${RULE[@]}"
  python3 -I "$HERE/probe.py" "$PORT" "$MESSAGES" > "$OUT/kernel-run$run.json"
  iptables -D INPUT "${RULE[@]}"
  echo "loadavg_after_run$run $(cat /proc/loadavg)" >> "$OUT/kernel.txt"
  python3 -I "$HERE/compare.py" summary "$OUT/kernel-run$run.json"
done

RTT_NS="$(python3 -I "$HERE/compare.py" rtt "$OUT")"
echo "model at the measured loopback RTT: $RTT_NS ns"
(
  cd "$SPIKE"
  NPT_KC_RTT_NS="$RTT_NS" NPT_KC_MESSAGES="$MESSAGES" NPT_KC_RUNS="$MODEL_RUNS" \
    cargo +1.99.1 test --release --locked -p npt-net --lib tcp_model::tests::kernel_cross_check_model \
    -- --ignored --nocapture --exact
) > "$OUT/model.txt"

python3 -I "$HERE/compare.py" judge "$OUT" | tee "$OUT/verdict.txt"
