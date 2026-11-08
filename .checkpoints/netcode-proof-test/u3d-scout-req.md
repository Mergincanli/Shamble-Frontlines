# U3d scout: what U3d (the timing self-test) must contain, and what exists (netcode-architect)
Branch claude/netcode-proof-test-plan-hcky5g, head 95dac9c. Read-only: notes only, no code, no builds, no runs.

## Plan
1. Requirement sources: u3-build-plan.md:45-46 and :5-12; advisor-u3q.md M1, M12-M15, :30; Mustafa's U3-Q1 (c) and
   U3-Q9 words (checkpoint "Facts from Mustafa" / "Decisions taken"); advisor-u3c-plan.md 2026-11-08 rulings
   (condition 10, diagnostic, UP-STAMP, ImpairSocket up E ~10 us); plan.md §12.1 U3/U6/U8 rows + other self-test,
   R1 run plan and U3-Q9 mentions.
2. The code as built: l2/socket.rs (ImpairSocket, release thread), socket_tests.rs (M13 run), stats.rs (E),
   delay_tests.rs (lateness_evidence_run), crates/lab (probe binary), marks.json + SERIES.md, crates/log, ops/runs/.
3. Latest outputs: results/u3c-lateness/c10-diagnostic-20261108T2152Z.txt.
4. Per requirement: source (file:line quoted), code (file:line), gap, here-vs-RS. Smallest U3d; engine-feature flags.

## Notes
