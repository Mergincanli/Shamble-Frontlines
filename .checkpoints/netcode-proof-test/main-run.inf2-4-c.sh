#!/usr/bin/env bash
# The main session's counted runs in the main folder (U4j's merge: advisor-u4j-r2.md M5, M6, M8.2; TE2-5's ten runs:
# B2.3 (b)), under W1-TARGET (advisor-u4-wave1.md): the shared target's lane lock held exclusively for the whole
# sequence, a wait until no other cargo or rustc runs, 4 GB free (check.mjs builds wasm32), every file under crates/
# touched first and every 2 s while the commands run (the loop starts and stops here), each command run from the main
# folder's spikes/netcode-proof with the shared target, offline. Everything goes to the log file; each command's exit
# code and, at the end, the provenance lines (Compiling/Checking/Documenting npt-*) are appended. The sequence stops at
# the first command that fails; the script exits with that command's code. It stops with exit 5 before taking the lock
# if the shared target has no CACHEDIR.TAG, and before any command if the lock's file cannot be opened or the lock
# taken (INF2-4: advisor-u4d2-m9.md, Block 2); no runner ever makes the shared target's folder.
# Usage: bash main-run.sh <log file> <command...> [:: <command...>]...
set -u
M=/home/user/Shamble-Frontlines
S=$M/spikes/netcode-proof
T=$M/.gate/shared-target
LOCK=$T/.lane.lock
export CARGO_TARGET_DIR=$T
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
flock -x 99 || { echo "STOP: cannot take the lane lock $LOCK"; exit 5; }
i=0
while pgrep -x cargo >/dev/null || pgrep -x rustc >/dev/null; do
  i=$((i + 1))
  if [ $i -gt 3600 ]; then
    echo "STOP: still busy after 60 min" > "$log"
    exit 4
  fi
  sleep 1
done
avail=$(df -BG --output=avail /home/user | tail -1 | tr -dc 0-9)
{
  echo "lock $LOCK held exclusively at $(date -u +%T) UTC; idle after ${i} s"
  echo "pwd $(pwd); free ${avail} GB; HEAD $(cd "$M" && git rev-parse --short HEAD)"
} > "$log"
if [ "$avail" -lt 4 ]; then
  echo "STOP: under 4 GB free" >> "$log"
  exit 3
fi
find "$S/crates" -type f -exec touch {} +
(while true; do
  find "$S/crates" -type f -exec touch {} +
  sleep 2
done) &
toucher=$!
trap 'kill $toucher 2>/dev/null' EXIT
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
kill $toucher 2>/dev/null
wait $toucher 2>/dev/null
prov=$(grep -E "^ *(Compiling|Checking|Documenting) npt-" "$log" | sed 's/^ *//' | sort | uniq -c)
{
  echo "--- provenance (Compiling/Checking/Documenting npt-* lines, counted):"
  echo "$prov"
  echo "--- other cargo or rustc now: $(pgrep -a cargo | head -3) $(pgrep -a rustc | head -3)"
  echo "--- end $(date -u +%T) UTC, exit $status"
} >> "$log"
exit $status
