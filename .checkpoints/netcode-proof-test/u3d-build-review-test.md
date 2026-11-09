# U3d build review (test-engineer): proving the timing self-test, 2026-11-09

Worktree: /home/user/Shamble-Frontlines/.claude/worktrees/wf_2c76ec82-8fc-1 on dae4f0f (uncommitted build). I am the
only one touching it now. Build rules: CARGO_TARGET_DIR=/home/user/Shamble-Frontlines/.gate/shared-target on every
cargo call (check.mjs included); touch the crate sources (mtime only) before each build; df -h /home/user before each
build (stop under 2 GB); check each run's test count against the one before. No iptables, no tc, no network; git
without -C / -c.

## Plan
1. Record the builder's sha256 of every changed file (baseline), git status.
2. check.mjs, unchanged, first: nine step lines and counts.
3. Bites, in place, one at a time, my own script (u3d-test-bites.py in this folder): T1a-b, T2a-b, T3a-b, T4a-e,
   T5a-d, T6a-d, T7a-c as §4 names them; then bites the plan missed: the self-test's run conditions (rate, drops,
   accounting, loss band, send errors, poller count, fate_log_full, slack, length bounds), the pre-touch, prints
   before asserts under a failing condition. Each: plant, run its test, record fail + how, restore, sha256 == builder's.
4. Reviewers' settleable findings: F1 slack under RT (chrt if the container allows), env NotUnicode, touched,
   samples() panic path, run.sh touch scope.
5. Self-test with NPT_L2_SELFTEST_SECS=10 (release): prints vs §3.1 and §5.1.
6. run.sh short (U3D_CHECK=1): U3-Q9 facts, THP, every ^cpu before/after; move its record out of results/.
7. Fresh touch, check.mjs again; git status + sha256 unchanged.
8. Findings, verdict, DONE.

## Progress
