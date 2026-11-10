#!/bin/bash
# fold-check-b scratch: step-5 merge+push semantics in a throwaway repo under .gate/foundation-toolchain/g1
set -u
cd /home/user/Shamble-Frontlines/.gate/foundation-toolchain/g1/a-merge || exit 1
git merge --no-edit origin/trunk 2>&1 | tail -1
echo "--- push HEAD:claude/b"
git push origin HEAD:claude/b 2>&1
git log --oneline --graph -4
echo "--- main-folder stand-in status"
cd /home/user/Shamble-Frontlines/.gate/foundation-toolchain/g1/a && git fetch -q origin && git status -sb | head -1
echo "--- unqualified dst for a branch the remote lacks"
cd /home/user/Shamble-Frontlines/.gate/foundation-toolchain/g1/a-merge && git push origin HEAD:claude/absent 2>&1 | tail -4
