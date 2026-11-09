# U4 build plan review (engine-architect), rev 1 of u4-build-plan.md

Memory: no agent-memory dir exists (checked); start fresh.
Branch claude/netcode-proof-test-plan-hcky5g; HEAD a99dd7f (autosaves on f985226, only .checkpoints changed).

## Plan
1. Read u4-build-plan.md (rev 1) in full; read plan.md's U4/U6/U7 sections and the marks.
2. Read the three scout notes; list claims the plan rests on.
3. Check against code: crate layout, Cargo.toml deps, sim seam, TICK_HZ, Link, public items.
4. Focus: §4.2 dep rules, §4.5 determinism native+Wasm, §4.7 public items, sim seam/TICK_HZ, Link use,
   no U6/U7 pull-forward, traced need per feature, sub-unit buildability.
5. Findings (severity, where, claim, evidence, fix), verdict.

## Findings log
