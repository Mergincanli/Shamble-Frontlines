#!/usr/bin/env bash
# The main session's calls on the main folder's own build cache (.gate/netcode-proof-test/target, the target-dir that
# spikes/netcode-proof/.cargo/config.toml names): the disk plan's part-clean (u4r-disk-plan.md section 5) and D4 (the
# round-2 ruling's step 14: r5's command in this cache). It works like main-run.sh, with two differences.
# CARGO_TARGET_DIR is unset, so cargo uses this cache. And it runs no toucher: a clean and the timed runs need none,
# and the build prints its Compiling/Fresh npt-* lines itself through -v.
# It holds the shared target's lane lock exclusively for the whole sequence, waits until no other cargo or rustc runs,
# needs 4 GiB free (a release build may follow), and runs offline. Each command runs from spikes/netcode-proof.
# Everything goes to the log file, with each command's exit code, free space, uptime and load average at the start and
# at the end, and the npt-* provenance lines. The sequence stops at the first command that fails, and the script exits
# with that command's code. It exits 5 if the shared target has no CACHEDIR.TAG, or if the lock's file cannot be
# opened or the lock taken. It exits 3 under 4 GiB free, and 4 if cargo or rustc is still busy after 60 min.
# Usage: bash cache-run.sh <log file> <command...> [:: <command...>]...
set -u
M=/home/user/Shamble-Frontlines
S=$M/spikes/netcode-proof
T=$M/.gate/shared-target
LOCK=$T/.lane.lock
unset CARGO_TARGET_DIR
export CARGO_NET_OFFLINE=true
log="$1"
shift
if [ ! -f "$T/CACHEDIR.TAG" ]; then
  echo "STOP: $T has no CACHEDIR.TAG (missing, or not cargo's own folder); no runner makes it"
  exit 5
fi
mkdir -p "$(dirname "$log")"
cd "$S" || exit 1
exec 9>"$LOCK" || { echo "STOP: cannot open the lane lock $LOCK"; exit 5; }
flock -x 9 || { echo "STOP: cannot take the lane lock $LOCK"; exit 5; }
i=0
while pgrep -x cargo >/dev/null || pgrep -x rustc >/dev/null; do
  i=$((i + 1))
  if [ $i -gt 3600 ]; then
    echo "STOP: still busy after 60 min" > "$log"
    exit 4
  fi
  sleep 1
done
avail=$(df -B1 --output=avail /home/user | tail -1 | tr -dc 0-9)
{
  echo "lock $LOCK held exclusively at $(date -u +%T) UTC; idle after ${i} s"
  echo "pwd $(pwd); free $((avail/1048576)) MiB ($avail B); HEAD $(cd "$M" && git rev-parse HEAD)"
  echo "CARGO_TARGET_DIR ${CARGO_TARGET_DIR:-unset}; uptime: $(uptime)"
} > "$log"
if [ "$avail" -lt $((4*1024*1024*1024)) ]; then
  echo "STOP: under 4 GiB free" >> "$log"
  exit 3
fi
status=0
run_one() {
  echo "=== $(date -u +%T) UTC run: $*" >> "$log"
  "$@" >> "$log" 2>&1
  local rc=$?
  echo "=== $(date -u +%T) UTC exit $rc: $*" >> "$log"
  return $rc
}
cmd=()
for a in "$@" "::"; do
  if [ "$a" = "::" ]; then
    if [ ${#cmd[@]} -gt 0 ]; then
      run_one "${cmd[@]}"
      rc=$?
      if [ $rc -ne 0 ]; then
        status=$rc
        break
      fi
    fi
    cmd=()
  else
    cmd+=("$a")
  fi
done
prov=$(grep -E "^ *(Compiling|Fresh|Checking|Documenting) npt-" "$log" | sed 's/^ *//' | sort | uniq -c)
avail=$(df -B1 --output=avail /home/user | tail -1 | tr -dc 0-9)
{
  echo "--- provenance (Compiling/Fresh/Checking/Documenting npt-* lines, counted):"
  echo "$prov"
  echo "--- other cargo or rustc now: $(pgrep -a cargo | head -3) $(pgrep -a rustc | head -3)"
  echo "--- free $((avail/1048576)) MiB ($avail B); uptime: $(uptime)"
  echo "--- end $(date -u +%T) UTC, exit $status"
} >> "$log"
exit $status
