# U4c review, round 1 (test-engineer)

Started 2026-11-10 10:15Z. Copy: /home/user/Shamble-Frontlines/.claude/worktrees/wf_f54246ff-e0d-2 (branch
worktree-wf_f54246ff-e0d-2, base 097f897). Build uncommitted: l1/src/lib.rs (M), l1/src/tests.rs (new),
l1/tests/l1.rs (M). No notes file from a cut run existed: fresh start.
Rules: plan §7.1 (shared target .gate/shared-target, CARGO_NET_OFFLINE, touch crates/ before and every 2 s during a
counted run, wait for no other cargo/rustc, df before each build). Plants in place, one at a time, restore, sha256.
No commit. Only crates/l1 touched, and only to plant and restore. This agent has no Edit tool: notes are rewritten
whole with Write; plants go through a plant script written with Write into this notes folder (u4c-test-plant.py:
exact unique-match replace, run, restore, sha256 check; the precedent of u4s-bites.py / u3d-test-bites.py), never
sed, redirects, heredocs or python3 -c.

## Plan
1. sha256 of the three changed files (and every file under crates/l1) before anything; compare with the builder's.
2. Read plan §5 U4c (L-c1..L-c9 and its bites), §4.1, §7.1; the builder notes; the two reviewers' notes.
3. Step 1 of the task: check.mjs unchanged, record ten step lines and counts (expected: measured 285, bites 312).
4. Step 2: plant each §5 U4c bite in place (8 model bites + any listed elsewhere), run the named L-c test, record the
   failure by name, restore, sha256. Add bites the plan missed (candidates from reading + reviewers' findings).
5. Step 3: each test can fail, none vacuous, numbers vs their sources (RFC 9002 constants, 1,200 B, PTO 1,024 ms,
   L-c2 bands, L-c8 pins).
6. Step 4: check.mjs again after a fresh touch; sha256 of every file = step 1.
7. Findings list + verdict; DONE.

## Progress
- 10:15Z Plan step 1: sha256 before anything = the builder's: lib.rs 1647e75d8b5bea59..., src/tests.rs
  c21a362f460a7fcd..., tests/l1.rs 639ab9c5e1e73982..., Cargo.toml bf80e546..., clippy.toml 3ffe7dc6.... Full
  hashes of all 158 spike files: scratchpad u4c-sha-before.txt. git status: only lib.rs M, tests/l1.rs M, src/tests.rs
  new (+ builder notes, lc8-scratch).
- 10:16-10:20Z Task step 1, check.mjs run R1 (no other cargo/rustc at start; touch first and every 2 s; df 12 GB):
  all 10 PASS. fmt 12.7 s; clippy 44.6 s; clippy-wasm32 10.4 s; doc 20.9 s; test 163.2 s measured 285 passed 0
  failed 0 skip, bites 312 passed 0 failed 0 skip; deps 1.7 s; refs 0.6 s (562 tracked files); marks 3.5 s (81
  marks, drafts 626df678); self 1.2 s 48/48; redact 0.8 s 10/10 (158 items, 0 findings). Counts = 275+10 / 292+20.
- Reviewers' candidates a run can settle: rust M "L-c5 cannot fail on a wrong next_event_ns" (plant next_event_ns
  -> None / wrong value); netcode F3 (C2 stricter than RFC A.7; read); netcode F1 (first-loss 1,024 ms; L-c9 pins it).
