checker3-CM: checker for area CM (fixes3). Started.
- apply-ops.py CM: 9 ops, 8 fields, 4 rows, clean. fixes3-ops-CM.json holds 1 op (CM13.evidence replace); other touched fields come from fixes2 (already final).
- engine-architect#9 (CM13 evidence): accepted, fixed fully and correctly. Only the STATUS sentence of CM13.evidence changed; rest identical. Checked: game:1458 is the houses loop (hb?hb[k]:houses[i].box, no array built); docs/reports/2026-10-16-three-teams-core.md:31 = fixes 9a4a757, 15b1971 "syncSimWorld no longer builds an array per house per call"; git show confirms both commits (15b1971 subject says the same); STATUS.md:118 still lists 561 B; team-prep-fixes.md:101 is PERF-4. No problem.
DONE
