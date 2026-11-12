# Skeptic, review round 1, D8 (= IE-4, Low): G0's 2.3 finding should record the image's stable = the pin and the active-toolchain check

Plan: re-read IE-4 (review-r1-findings.json), toolchain.yml and the report's G0 row at 4d78ad6, plus G0's full text in
STATUS_ARCHIVE, the register row Q2.2-a and ADR 0002's G0; re-fetch the head job's and M2's CI logs; re-read the
runner image's software list and install-rust.sh, and rustup's CHANGELOG, book and release-stable.toml; check the plant
list and toolchain.yml's history for any run without the install step; reproduce rustup's override order locally only if
rustup is on this machine (in .gate/skeptic-D8/, never in the worktree); then judge holds, severity and the fix.
Default: does not hold unless the evidence is there.

## Checks
