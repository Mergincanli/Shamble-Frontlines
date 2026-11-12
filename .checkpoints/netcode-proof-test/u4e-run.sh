#!/usr/bin/env bash
# U4e's runner (u4e-brief.md section 4, rules 1-8; W1-TARGET, advisor-u4-wave1.md): one command, or a sequence, in the
# U4e builder's worktree, with the shared target. It takes the shared target's lane lock (`-x`: exclusive, for a
# counted run, a check.mjs run, a `--list` or clippy confirmation, each bite run and each scratch switch or plant;
# otherwise shared), waits until no other cargo or rustc runs, checks 4 GB free, touches every file under the copy's
# crates/ first and every 2 s while the commands run (the loop starts and stops here), and runs each command from the
# copy's spikes/netcode-proof with CARGO_TARGET_DIR on the shared target and CARGO_NET_OFFLINE=true. Everything goes to
# the log file, which must be under /home/user/Shamble-Frontlines/.gate/u4e/; each command's exit code and, at the
# end, the provenance lines (Compiling/Checking/Documenting npt-*) are appended. The sequence stops at the first command
# that fails, and the script exits with that command's code. A run's result is that exit code and the test runner's own
# summary lines, read from the log.
# Usage: bash u4e-run.sh [-x] <worktree> <log file> <command...> [:: <command...>]...
set -u
M=/home/user/Shamble-Frontlines
LOCK=$M/.gate/shared-target/.lane.lock
export CARGO_TARGET_DIR=$M/.gate/shared-target
export CARGO_NET_OFFLINE=true
mode=-s
if [ "${1:-}" = "-x" ]; then
  mode=-x
  shift
fi
W="${1:-}"
log="${2:-}"
shift 2 || exit 2
case "$W" in
  "$M"/.claude/worktrees/*) ;;
  *) echo "STOP: the worktree must be under $M/.claude/worktrees/ (got: $W)"; exit 2 ;;
esac
case "$log" in
  "$M"/.gate/u4e/*) ;;
  *) echo "STOP: the log must be under $M/.gate/u4e/ (got: $log)"; exit 2 ;;
esac
S=$W/spikes/netcode-proof
[ -d "$S/crates" ] || { echo "STOP: no $S/crates"; exit 2; }
mkdir -p "$(dirname "$log")"
cd "$S" || exit 1
exec 9>"$LOCK"
flock "$mode" 9
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
  echo "lock $LOCK held ($mode) at $(date -u +%T) UTC; idle after ${i} s"
  echo "pwd $(pwd); free ${avail} GB; HEAD $(git rev-parse --short HEAD); CARGO_TARGET_DIR $CARGO_TARGET_DIR"
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
