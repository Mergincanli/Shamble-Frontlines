#!/usr/bin/env bash
# Replays change-set 1's miss in a scratch clone: the remote branch stands at 55397c8, main at 22c6afa (as on
# 2026-11-18, 12:04 UTC); the code commit 566d1d1 (three manifest and lockfile changes) is ahead, unpushed; the notes of
# da22f96 are on disk, uncommitted; then a foreground save with the given copy of the checkpoint script.
set -euo pipefail
script=$1; name=$2; src=/home/user/Shamble-Frontlines; b=claude/wonderful-hamilton-jdmlnf
root=$(mktemp -d "/tmp/claude-0/-home-user-Shamble-Frontlines/f707c56b-26b4-5848-b1e1-237a99a6a881/scratchpad/replay/run-$name-XXXX")
git clone --quiet --bare --no-local "$src" "$root/origin.git" 2>/dev/null
(cd "$root/origin.git" && git config receive.shallowUpdate true && git update-ref "refs/heads/$b" 55397c8 && git update-ref refs/heads/main 22c6afa0341c36c40479f671736a569adfdb2663)
git clone --quiet --no-local "$src" "$root/repo" 2>/dev/null
cd "$root/repo"
git config user.name Replay; git config user.email replay@example.invalid; git config commit.gpgsign false
git remote set-url origin "$root/origin.git"
git fetch --quiet --prune origin
git switch --quiet -C "$b" 566d1d1
git restore --source=da22f96 -- .checkpoints
before=$(cd "$root/origin.git" && git rev-parse "refs/heads/$b")
set +e; out=$(node "$script" save cs1-types -m "build step A done (replay)" 2>&1); code=$?; set -e
after=$(cd "$root/origin.git" && git rev-parse "refs/heads/$b")
echo "[$name] exit $code; remote branch $( [ "$before" = "$after" ] && echo 'unchanged (nothing pushed)' || echo "MOVED to ${after:0:7} (pushed)" )"
echo "[$name] $(echo "$out" | cut -c26-330)"
echo "[$name] the local head's subject: $(git log -1 --format=%s | cut -c1-110)"
