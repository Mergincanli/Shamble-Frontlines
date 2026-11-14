#!/usr/bin/env bash
# U4j fix pass runner (round 1): waits until no other cargo or rustc runs, checks 2 GB free, touches every file under
# the copy's crates/, keeps touching them every 2 s while the command runs (the loop starts and stops here), and runs
# the command from the copy's spikes/netcode-proof with the shared target, offline.
# Usage: bash run.sh <command...>
set -u
COPY=/home/user/Shamble-Frontlines/.claude/worktrees/agent-a3b8dfeb01ed52b50/spikes/netcode-proof
cd "$COPY" || exit 1
export CARGO_TARGET_DIR=/home/user/Shamble-Frontlines/.gate/shared-target
export CARGO_NET_OFFLINE=true
echo "df: $(df -h /home/user | tail -1)"
avail=$(df -k --output=avail /home/user | tail -1 | tr -d ' ')
if [ "$avail" -lt 2097152 ]; then
  echo "STOP: under 2 GB free"
  exit 3
fi
n=0
while pgrep cargo >/dev/null || pgrep rustc >/dev/null; do
  if [ $n -eq 0 ]; then
    echo "waiting for:"
    pgrep -a cargo
    pgrep -a rustc
  fi
  n=$((n + 1))
  if [ $n -gt 720 ]; then
    echo "STOP: still busy after 60 min"
    exit 4
  fi
  sleep 5
done
echo "idle at $(date -u +%H:%M:%S) (waited $((n * 5)) s)"
find "$COPY/crates" -type f -exec touch {} +
(while true; do
  find "$COPY/crates" -type f -exec touch {} +
  sleep 2
done) &
loop=$!
trap 'kill $loop 2>/dev/null' EXIT
"$@"
rc=$?
kill $loop 2>/dev/null
wait $loop 2>/dev/null
echo "rc=$rc at $(date -u +%H:%M:%S)"
exit $rc
