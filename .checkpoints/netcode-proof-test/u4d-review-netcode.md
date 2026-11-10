# U4d build review: netcode-architect (read only)

Worktree: /home/user/Shamble-Frontlines/.claude/worktrees/wf_f54246ff-e0d-1 (uncommitted, base 097f8970)
Plan: u4-build-plan.md §5 U4d + §4.2, §4.10, §4.13, §4.15; advisor-u4.md; ADR 0004.

## Plan
1. Read plan §5 U4d section + §4.2/4.10/4.13/4.15 + carries 7-20, 36-38; advisor rulings.
2. git status / git diff in worktree; read each new/modified file.
3. Check: seam freeze kept; refusals outside hash; scenes; recordings; carries as worded; no U4e-U4h pull-forward.
4. Findings with file:line; verdict.

## Progress
- [1] Read plan §4.2/4.10/4.13/4.15, §5 U4d, carries 7-20/36-38 (u4s-carries.md), advisor-u4.md conditions 1-5 and U4s ruling cond 3 (freeze).
- [2] Diff: seam.rs (+button consts, TickInput spawns/teleports/attributes, Placement/AttributeChange/Attribute, layout completed within 12, HitVolumes soldiers/cover + hand Clone + Eq, VolumeBox, docs; non_finite removed), history.rs 1 MB cap (1<<20 / MAX_BYTES 4,536 = 231 ticks), grid.rs doc (carry 20), lib.rs re-exports. Freeze list (Command, Rewind, CoverVariant, EntityKind, EntityRecord, 5 grid fields): field shapes unchanged so far; HitVolumes not in freeze list.
