# applier3-CM: R7 fixes3 applier for area CM (1 finding: engine-architect#9 on CM13 evidence)
- engine-architect#9 (CM13 evidence): accepted. game:1458 houses loop uses `hb?hb[k]:houses[i].box`, no array built; docs/reports/2026-10-16-three-teams-core.md:31 says fixes 9a4a757, 15b1971 made syncSimWorld stop building an array per house; STATUS.md:118 still lists 561 B. Op: replace the STATUS sentence with a version that marks the figure stale (my wording, fix_op's intent). apply-ops.py CM applied cleanly.
DONE
