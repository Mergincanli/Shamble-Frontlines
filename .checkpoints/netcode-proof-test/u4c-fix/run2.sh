#!/usr/bin/env bash
# U4c fix pass (resumed fixer): one COUNTED run in this copy, under plan §7.1's rules. As run.sh, plus:
# - waits until no cargo, rustc, clippy or rustdoc process has run anywhere for IDLE s (default 20; up to 30 min);
# - while the run lasts, every 2 s, beside the touch, logs each such process whose working directory is outside this
#   copy (a foreign lane), with its CARGO_TARGET_DIR, to <label>.foreign.log, for the notes to weigh.
# (Edited after r2-check-1 and -2: the IDLE knob added; "one in the shared target voids the run" replaced, see notes.)
# Usage: run2.sh <label> <command...>   (the command runs in spikes/netcode-proof; its output goes to <label>.log here)
set -u
COPY=/home/user/Shamble-Frontlines/.claude/worktrees/wf_f54246ff-e0d-2
HERE="$COPY/.checkpoints/netcode-proof-test/u4c-fix"
export CARGO_TARGET_DIR=/home/user/Shamble-Frontlines/.gate/shared-target
export CARGO_NET_OFFLINE=true
label=$1
shift
log="$HERE/$label.log"
flog="$HERE/$label.foreign.log"
procs() {
  pgrep -x cargo
  pgrep -x rustc
  pgrep -x cargo-clippy
  pgrep -x clippy-driver
  pgrep -x rustdoc
}
foreign() {
  for pid in $(procs); do
    cwd=$(readlink "/proc/$pid/cwd" 2>/dev/null) || continue
    case "$cwd" in
      "$COPY" | "$COPY"/*) ;;
      *)
        target=$(tr '\0' '\n' <"/proc/$pid/environ" 2>/dev/null | grep '^CARGO_TARGET_DIR=' | head -1)
        cmd=$(tr '\0' ' ' <"/proc/$pid/cmdline" 2>/dev/null | cut -c1-140)
        echo "$(date -u +%H:%M:%SZ) pid $pid cwd $cwd ${target:-CARGO_TARGET_DIR unset}: $cmd"
        ;;
    esac
  done
}
avail=$(df --output=avail -BG /home/user | tail -1 | tr -dc '0-9')
echo "df: ${avail} GB free" | tee "$log"
echo "foreign cargo/rustc/clippy/rustdoc seen during $label (none if empty)" >"$flog"
if [ "$avail" -lt 2 ]; then
  echo "STOP: below 2 GB free" | tee -a "$log"
  exit 3
fi
idle=${IDLE:-20}
quiet=0
waited=0
while [ "$quiet" -lt "$idle" ]; do
  if [ -n "$(procs)" ]; then quiet=0; else quiet=$((quiet + 1)); fi
  sleep 1
  waited=$((waited + 1))
  if [ "$waited" -gt 1800 ]; then
    echo "STOP: other cargo for 30 min" | tee -a "$log"
    exit 4
  fi
done
echo "idle ${idle} s after ${waited} s; start $(date -u +%H:%M:%SZ)" | tee -a "$log"
find "$COPY/spikes/netcode-proof/crates" -type f -exec touch {} +
(
  while :; do
    find "$COPY/spikes/netcode-proof/crates" -type f -exec touch {} +
    foreign >>"$flog"
    sleep 2
  done
) &
toucher=$!
cd "$COPY/spikes/netcode-proof" || exit 1
"$@" >>"$log" 2>&1
status=$?
kill "$toucher"
wait "$toucher" 2>/dev/null
foreign >>"$flog"
seen=$(($(wc -l <"$flog") - 1))
echo "end $(date -u +%H:%M:%SZ) status $status; foreign process samples during and at the end: $seen" | tee -a "$log"
exit "$status"
