#!/usr/bin/env bash
# U4d fix pass round 1: one cargo (or node) run in this copy under the lane rules (u4-build-plan.md §7.1; W1-TARGET):
# wait until no other cargo or rustc runs, check the disk, touch every crate file, keep touching every 2 s while the
# run lasts (the loop started and stopped inside this script), print the provenance lines. Usage:
#   u4d-fix1-run.sh <log file> <command...>
# The script cds into this copy itself and runs the command in the foreground.
C=/home/user/Shamble-Frontlines/.claude/worktrees/wf_f54246ff-e0d-1
cd "$C/spikes/netcode-proof" || exit 1
export CARGO_TARGET_DIR=/home/user/Shamble-Frontlines/.gate/shared-target
export CARGO_NET_OFFLINE=true
log="$1"; shift
for i in $(seq 1 600); do
  if ! pgrep -x cargo >/dev/null && ! pgrep -x rustc >/dev/null; then break; fi
  sleep 1
done
echo "idle after ${i} s: $(pgrep -a cargo | head -3) $(pgrep -a rustc | head -3)" > "$log"
avail=$(df -BG --output=avail /home/user | tail -1 | tr -dc 0-9)
echo "pwd $(pwd); free ${avail} GB" >> "$log"
if [ "$avail" -lt 2 ]; then echo "below 2 GB free: stop" >> "$log"; cat "$log"; exit 3; fi
find "$C/spikes/netcode-proof/crates" -type f -exec touch {} +
( while true; do find "$C/spikes/netcode-proof/crates" -type f -exec touch {} +; sleep 2; done ) &
toucher=$!
start=$(date -u +%T)
"$@" >> "$log" 2>&1
status=$?
kill "$toucher" 2>/dev/null; wait "$toucher" 2>/dev/null
echo "run ${start}-$(date -u +%T) UTC, exit ${status}" >> "$log"
echo "--- provenance:" >> "$log"
grep -E "(Compiling|Checking|Documenting) npt-sim" "$log" | sort | uniq -c >> "$log"
echo "--- other cargo during the run (now): $(pgrep -a cargo | grep -v $$ | head -3)" >> "$log"
exit $status
