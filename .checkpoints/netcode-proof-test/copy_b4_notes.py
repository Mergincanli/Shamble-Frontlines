"""Copy builder 4's notes block word for word from its worktree's copy of u4f-build.md into the main folder's: the
harness keeps an isolated agent inside its worktree, so its notes are there. Asserts the main folder's file is a byte
prefix of the worktree's copy (its :1-866 equal), then makes the main folder's file equal to the worktree's copy.
Safe to run again (later blocks of the same notes are copied the same way). Run from the repo root:
python3 -I .checkpoints/netcode-proof-test/copy_b4_notes.py"""
import hashlib

W = ".claude/worktrees/agent-a5b1297a320917dfb/.checkpoints/netcode-proof-test/u4f-build.md"
M = ".checkpoints/netcode-proof-test/u4f-build.md"
w = open(W, "rb").read()
m = open(M, "rb").read()
assert w.startswith(m), "the main folder's notes are not a prefix of the worktree's copy: stop"
assert b"## Fix pass (round 1): builder 4" in w[len(m):] or b"## Fix pass (round 1): builder 4" in m
open(M, "wb").write(w)
print(f"{M}: {m.count(10)} -> {w.count(10)} lines; added {len(w) - len(m)} bytes; sha256 {hashlib.sha256(w).hexdigest()}")
