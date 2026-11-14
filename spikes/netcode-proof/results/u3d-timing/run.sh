#!/usr/bin/env bash
# U3d's dry run (u3d-plan.md §3.4 and §5.1): ImpairSocket's timing self-test at P1 (l2_timing_self_test, 100 s) and
# M13's run at P2 (l2_sustains_10000_per_second_each_way, 10 s), interleaved, three rounds, in a quiet container (no
# agent or build alongside, nothing else on loopback). Reported, never judged here: this container is no evidence for a
# µs-level bar (advisor-u3c-plan.md:114); RS judges at R1, on the -3 probe's P1 leg (plan §12.1 U8). A miss goes to the
# advisor in U3d's report; it is not retried into a pass (plan.md:683) and it is not a stop (advisor-u3c-plan.md:163-165).
# The main session runs it after the merge, one evidence run at a time, from the main folder, with CARGO_TARGET_DIR
# unset, so the spike's own cache (.gate/netcode-proof-test/target) builds it (plan §7.1 item 6). About 6-7 minutes.
#
# What it records, in this folder's selftest-<UTC>.txt (the runs' own output unchanged, like c10's file):
# - The header: the commit, `rustc -V`, the test binary's path and SHA-256, the date (UTC), nproc, CARGO_TARGET_DIR;
#   then U3-Q9's facts and the THP mode, each read or written "unreadable", never guessed: the kernel (uname -r), the
#   clocksource (current and available), the cpuidle driver and governor, CONFIG_HZ (from /boot/config-$(uname -r),
#   else /proc/config.gz; never `getconf CLK_TCK`, which is USER_HZ), and /sys/kernel/mm/transparent_hugepage/enabled.
# - Around each run: `uptime` and every ^cpu line of /proc/stat (the total and each vCPU), before and after; beside the
#   raw lines, per line, the steal share Δsteal / Δ(user + nice + system + idle + iowait + irq + softirq + steal), steal
#   being the 8th value (proc(5)). The raw lines stay in the record, so anyone can recompute.
# - Each run's exit status. A failed (INVALID) run stays in the record with its cause, and the script goes on.
#
# Build once, run that binary: cargo builds the release test binary once; its path comes from cargo's JSON (the
# `executable` field), its SHA-256 is recorded, `<binary> --ignored --list` must name both runs, and every run executes
# that binary directly, so the hashed binary is the one that ran.
#
# Use (from anywhere; cargo +1.99.1 and python3 on PATH, CARGO_TARGET_DIR unset):
#   bash spikes/netcode-proof/results/u3d-timing/run.sh
# NPT_L2_SELFTEST_SECS (default 100), NPT_L2_SECS (default 10) and ROUNDS (default 3) change the lengths and rounds;
# the header records them. With CARGO_TARGET_DIR set the script refuses: a shared target folder can hold another copy's
# newer artifacts, which cargo may read as fresh (the G-METHOD incident). A build copy checking the script sets
# U3D_CHECK=1 as well: then it touches every file under the copy's crates/ first (mtime only: §7.1 item 2) and marks
# the record a check run, not evidence.

set -euo pipefail

HERE="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
SPIKE="$(cd "$HERE/../.." && pwd)"
SELF_SECS="${NPT_L2_SELFTEST_SECS:-100}"
M13_SECS="${NPT_L2_SECS:-10}"
ROUNDS="${ROUNDS:-3}"
SELF=l2::socket::tests::l2_timing_self_test
M13=l2::socket::tests::l2_sustains_10000_per_second_each_way
OUT="$HERE/selftest-$(date -u +%Y%m%dT%H%M%SZ).txt"
KIND="evidence run"

command -v cargo >/dev/null || { echo "run.sh: no cargo" >&2; exit 2; }
command -v python3 >/dev/null || { echo "run.sh: no python3 (it reads cargo's JSON)" >&2; exit 2; }
if [ -n "${CARGO_TARGET_DIR:-}" ]; then
  if [ "${U3D_CHECK:-}" != 1 ]; then
    echo "run.sh: CARGO_TARGET_DIR is set ($CARGO_TARGET_DIR); the dry run builds in the spike's own cache (plan §7.1 item 6): unset it" >&2
    exit 2
  fi
  KIND="CHECK RUN (a build copy's check of this script, built in $CARGO_TARGET_DIR): not evidence"
  find "$SPIKE/crates" -type f -exec touch {} +
fi
if [ -e "$OUT" ]; then
  echo "run.sh: $OUT exists already; never overwritten" >&2
  exit 2
fi

# A file's first line, or "unreadable".
fact() {
  local v
  if v="$(head -n 1 "$1" 2>/dev/null)" && [ -n "$v" ]; then echo "$v"; else echo unreadable; fi
}

config_hz() {
  local f v
  f="/boot/config-$(uname -r)"
  v="$(grep -E '^CONFIG_HZ=' "$f" 2>/dev/null | cut -d= -f2 || true)"
  if [ -n "$v" ]; then echo "$v (from $f)"; return; fi
  v="$(zcat /proc/config.gz 2>/dev/null | grep -E '^CONFIG_HZ=' | cut -d= -f2 || true)"
  if [ -n "$v" ]; then echo "$v (from /proc/config.gz)"; return; fi
  echo unreadable
}

# Per ^cpu line: the steal share between two reads of /proc/stat ($1 before, $2 after).
steal_shares() {
  awk 'NR == FNR { for (i = 2; i <= 9; i++) b[$1, i] = $i; next }
       { t = 0; for (i = 2; i <= 9; i++) t += $i - b[$1, i]; s = $9 - b[$1, 9]
         printf "steal %s: %d of %d ticks = %.4f\n", $1, s, t, (t > 0 ? s / t : 0) }' \
    <(printf '%s\n' "$1") <(printf '%s\n' "$2")
}

# Build once, from the spike folder (its .cargo/config.toml and rust-toolchain.toml apply).
JSON="$(cd "$SPIKE" && cargo +1.99.1 test --release --locked -p npt-net --lib --no-run --message-format=json)"
BIN="$(printf '%s\n' "$JSON" | python3 -I -c '
import json, sys
exe = [m["executable"] for m in map(json.loads, filter(None, sys.stdin.read().splitlines()))
       if m.get("reason") == "compiler-artifact" and m.get("executable")
       and m["target"]["name"] == "npt_net" and m["profile"]["test"]]
if len(exe) != 1:
    sys.exit(f"run.sh: expected one npt_net test executable in cargo JSON, got {exe}")
print(exe[0])
')"
LIST="$("$BIN" --ignored --list)"
for t in "$SELF" "$M13"; do
  if ! grep -qxF "$t: test" <<<"$LIST"; then
    echo "run.sh: $BIN does not list $t among its ignored tests" >&2
    exit 2
  fi
done

COMMIT="$(cd "$SPIKE" && git rev-parse HEAD 2>/dev/null)" || COMMIT=unreadable
DIRTY="$(cd "$SPIKE" && git status --porcelain -- . 2>/dev/null | head -n 1)" || DIRTY=""
GOVERNOR="$(fact /sys/devices/system/cpu/cpuidle/current_governor)"
if [ "$GOVERNOR" = unreadable ]; then
  GOVERNOR="$(fact /sys/devices/system/cpu/cpuidle/current_governor_ro) (current_governor_ro)"
fi
{
  echo "# U3d dry run (u3d-plan.md §3.4, §5.1): $KIND. Reported, not judged here; RS judges at R1 (U8)."
  echo "commit $COMMIT${DIRTY:+ (uncommitted changes under spikes/netcode-proof)}"
  echo "rustc $(rustc +1.99.1 -V 2>/dev/null || echo unreadable)"
  echo "binary $BIN"
  echo "binary_sha256 $(sha256sum "$BIN" | cut -d' ' -f1)"
  echo "date_utc $(date -u +%Y-%m-%dT%H:%M:%SZ)"
  echo "nproc $(nproc)"
  echo "cargo_target_dir ${CARGO_TARGET_DIR:-unset}"
  echo "lengths: self-test ${SELF_SECS} s (NPT_L2_SELFTEST_SECS), M13 ${M13_SECS} s (NPT_L2_SECS); rounds ${ROUNDS}"
  echo "kernel $(uname -r)"
  echo "clocksource $(fact /sys/devices/system/clocksource/clocksource0/current_clocksource) (available: $(fact /sys/devices/system/clocksource/clocksource0/available_clocksource))"
  echo "cpuidle_driver $(fact /sys/devices/system/cpu/cpuidle/current_driver)"
  echo "cpuidle_governor $GOVERNOR"
  echo "config_hz $(config_hz)"
  echo "thp_enabled $(fact /sys/kernel/mm/transparent_hugepage/enabled)"
  echo "loadavg_start $(fact /proc/loadavg)"
} > "$OUT"
cat "$OUT"

STATUSES=()
# One run: $1 its label, $2 the test, $3 the length variable, $4 its value.
run_one() {
  local before after status
  before="$(grep '^cpu' /proc/stat)"
  {
    echo "=== $1 $(date -u +%Y%m%dT%H%M%SZ)"
    uptime
    echo "/proc/stat before:"
    printf '%s\n' "$before"
  } >> "$OUT"
  status=0
  env "$3=$4" "$BIN" --ignored --nocapture --exact "$2" >> "$OUT" 2>&1 || status=$?
  after="$(grep '^cpu' /proc/stat)"
  {
    echo "/proc/stat after:"
    printf '%s\n' "$after"
    steal_shares "$before" "$after"
    uptime
    if [ "$status" -eq 0 ]; then
      echo "exit_status 0"
    else
      echo "exit_status $status (the run failed or was INVALID: its output above says why; it stays in the record)"
    fi
  } >> "$OUT"
  STATUSES+=("$1: $status")
  echo "$1: exit status $status"
}

for round in $(seq 1 "$ROUNDS"); do
  run_one "SELFTEST $round (P1, ${SELF_SECS} s)" "$SELF" NPT_L2_SELFTEST_SECS "$SELF_SECS"
  run_one "M13 $round (P2, ${M13_SECS} s)" "$M13" NPT_L2_SECS "$M13_SECS"
done
{
  echo "=== summary"
  printf '%s\n' "${STATUSES[@]}"
  echo "loadavg_end $(fact /proc/loadavg)"
} >> "$OUT"
echo "record: $OUT"
