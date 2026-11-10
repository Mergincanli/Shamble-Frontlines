#!/bin/bash
# fold-check-b scratch: main-folder stand-in vs remote after the push; the saver in the detached worktree
set -u
cd /home/user/Shamble-Frontlines/.gate/foundation-toolchain/g1/a || exit 1
echo "local claude/b: $(git rev-parse --short claude/b)  origin/claude/b: $(git rev-parse --short origin/claude/b)"
cd /home/user/Shamble-Frontlines/.gate/foundation-toolchain/g1/a-merge || exit 1
echo "--- saver in the detached worktree"
node /home/user/Shamble-Frontlines/.claude/scripts/checkpoint.mjs save x -m t 2>&1 | head -3
