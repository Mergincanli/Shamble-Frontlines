#!/usr/bin/env bash
# U4c fix pass: one cargo run in this copy, under plan §7.1's rules.
# Usage: run.sh <label> <command...>   (the command runs in spikes/netcode-proof; its output goes to <label>.log here)
# Waits until no other cargo or rustc has run for 5 s (up to 20 min), checks df (stops below 2 GB), touches every file
# under spikes/netcode-proof/crates in this copy before the run and every 2 s while it lasts, then stops the loop.
set -u
COPY=/home/user/Shamble-Frontlines/.claude/worktrees/wf_f54246ff-e0d-2
HERE="$COPY/.checkpoints/netcode-proof-test/u4c-fix"
export CARGO_TARGET_DIR=/home/user/Shamble-Frontlines/.gate/shared-target
export CARGO_NET_OFFLINE=true
label=$1
shift
log="$HERE/$label.log"
avail=$(df --output=avail -BG /home/user | tail -1 | tr -dc '0-9')
echo "df: ${avail} GB free" | tee "$log"
if [ "$avail" -lt 2 ]; then
  echo "STOP: below 2 GB free" | tee -a "$log"
  exit 3
fi
quiet=0
waited=0
while [ "$quiet" -lt 5 ]; do
  if pgrep -x cargo >/dev/null || pgrep -x rustc >/dev/null; then quiet=0; else quiet=$((quiet + 1)); fi
  sleep 1
  waited=$((waited + 1))
  if [ "$waited" -gt 1200 ]; then
    echo "STOP: other cargo for 20 min" | tee -a "$log"
    exit 4
  fi
done
echo "idle after ${waited} s; start $(date -u +%H:%M:%SZ)" | tee -a "$log"
find "$COPY/spikes/netcode-proof/crates" -type f -exec touch {} +
(
  while :; do
    find "$COPY/spikes/netcode-proof/crates" -type f -exec touch {} +
    sleep 2
  done
) &
toucher=$!
cd "$COPY/spikes/netcode-proof" || exit 1
"$@" >>"$log" 2>&1
status=$?
kill "$toucher"
wait "$toucher" 2>/dev/null
others=$( (pgrep -a cargo; pgrep -a rustc) | grep -v "$COPY" | head -3)
echo "end $(date -u +%H:%M:%SZ) status $status; foreign cargo/rustc at the end: ${others:-none}" | tee -a "$log"
exit "$status"
