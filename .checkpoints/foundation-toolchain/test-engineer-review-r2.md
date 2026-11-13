# test-engineer, review round 2 of foundation 2.2 (PR #48): 4d78ad6..4bccf55 (excluding .checkpoints/)

Angle: is it proven? Only what changed since round 1's head 4d78ad6.
Scratch and builds: /home/user/Shamble-Frontlines/.gate/test-engineer-r2/ (nothing in the worktree but this file).
Agent memory: none in this environment (no .claude/agent-memory-local/); round-1 notes read
(.checkpoints/foundation-toolchain/test-engineer-review-r1.md).

## Plan (blocks; each marked DONE with its evidence when finished)
1. Read the diff 4d78ad6..4bccf55 (code: Cargo.toml, 4 crates' doc comments; docs: report, register row OVERFLOW-ALL,
   REPO_MAP, ROADMAP, STATUS, STATUS_ARCHIVE); the ruling advisor-ruling-d1.md (conditions 5, 7, 7e); ADR 0002 near
   :1319 (PM-0002-53) and the overflow parts; round-1 dedupe and verdicts for my own findings' dispositions.
2. M7: 297d48a touches only the release setting; 8d622ac exact revert (git diff 297d48a^ 8d622ac empty); CI runs
   43415093334 (job 131850863497) push and 43415101662 pull_request: which step is red, which binary, which message;
   compare with the report's M7 row word by word; does it claim to distinguish route A from part 4's blocks?
   Also: CI on 8d622ac and the head 4bccf55 (green? cancelled?).
3. Relabelled evidence: M5 and step 2's control marked historical; grep report/STATUS/register for every use of M5,
   "control", "step 2" and check nothing still leans on them for a current claim.
4. E12: read xcrate-check.sh and both outputs; check "before" really is the pre-fix tree (what did the script archive,
   when; compare the archive's Cargo.toml with 4d78ad6/1de98a9's); re-run on a scratch copy of 1de98a9 and of 4bccf55,
   native and wasm32; check the probe can fail (a caller crate's inline/generic sim code, debug vs release,
   wasm32 run or build only).
5. OVERFLOW-XCRATE-CI anchor: wording in report/register/STATUS; is there any mechanism (CI, gate, checklist) that
   fires on "the first change that adds a dependency on sim"; who notices.
6. PM-0002-53 placement: ADR 0002:1319 text vs the report's and the register row's text; testable as written?
7. Remaining new rows and dispositions (round-1 findings TE-1..TE-3 and others' that changed text); facts in E12/M7.
8. Findings, claim/proof/gap table, verdict; advisor tool before finalising.

## Findings (a line each, as found)
