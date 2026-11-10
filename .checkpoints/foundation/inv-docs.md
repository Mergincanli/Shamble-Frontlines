# Task: foundation step 2.1 inventory, docs area (hits-c-docs.txt, 603 hits in docs/), plan only
## Plan (blocks, in order)
B1 per-file hit counts
B2 big citation files (LEGACY_INVENTORY, LEGACY_TUNING, legacy-tuning.json, EDITOR_V1_INVENTORY, research/*): counts + COMMAND/STALE lines
B3 current-truth docs (REPO_MAP, START_HERE, MODULE_MAP, TECHNICAL, STATUS, README etc.): classify each hit
B4 ADRs (0001-0006, README register): classify + PM-row check (Q4)
B5 remaining docs (design, look, reports, ROADMAP, others): classify
B6 Q2 ADR 0002:998-1007 + KT21; count path changes per option
B7 Q3 repo-map-check.mjs
B8 Q5 hard constraints / double-click docs
B9 design answer written
## Notes
- B1 DONE: counts: LEGACY_INVENTORY 238, legacy-tuning.json 60, LEGACY_TUNING 60, EDITOR_V1_INVENTORY 39, TECHNICAL 34, research/r1 26, MODULE_MAP 20, START_HERE 16, r2 15, r5 8, adr/0002 8, REPO_MAP 8, adr/0003 7, STATUS 7, r4 6, reports (14 files) 41, UNREAD_SOURCES 3, adr/0005 2, ART_DIRECTION 2, adr 0001/0004/0006 1 each, ROADMAP 1, EDITOR_V2 1. Next B2.
- Q3 finding (read early, .claude/scripts/repo-map-check.mjs:23-37): rows are first backticked path; required = depth 1 for every path, depth 2 only under docs/tools/.claude/.github. After move: 5 root rows (REPO_MAP.md:14-18) go stale -> exit 1 ("in the map but not in the repository"); `legacy/` becomes required (depth 1 only; legacy/* rows allowed, not required). After 2.2: `crates/`, `packages/` and each new root file (Cargo.toml, Cargo.lock, package.json, lockfile, rust-toolchain.toml, workspace file...) required; children not required. If legacy docs move to legacy/docs/, docs/ rows for them go stale too.
