# r5 rust-reviewer fixes (ADR 0004)

### FIX e1-F3
FILE: docs/adr/0004-simulation-core.md
BEFORE:
| 0 | Mustafa and the Advisor approve the drafts | the sign-off line of this record |
AFTER:
| 0 | The drafts signed off by Mustafa as the basis for the spikes (2026-11-03, SPIKE-ORDER (b)) | the sign-off line of this record |
END

### FIX e3b-2
FILE: docs/adr/0004-simulation-core.md
BEFORE:
| 1 | Spikes, throwaway in `spikes/` (SPIKE-SCOPE (a)), in the order Mustafa accepted (SPIKE-ORDER (b), 2026-11-03: "wave 1 runs one heavy spike at a time"): the netcode proof test first; then the sim core spike; then the 60 Hz margin run, with the sim core spike's sim and S1's stand-ins (OQ-0001-2 (a)), an early signal only (TICK-60-SETUP, the Advisor, 2026-11-03); then the boundary spike; then the content and ability spike | S1–S14, S19 and S20 recorded in this record; S17 from the netcode proof test; S18 from the boundary spike; S15 and S16 from the content and ability spike; option C or B, the navigation generator, the units per tick and the hash function chosen from the numbers |
AFTER:
| 1 | Spikes, throwaway in `spikes/` (SPIKE-SCOPE (a)): those that carry this record's marks, in `docs/SPIKE_PLAN.md`'s order, which Mustafa accepted (SPIKE-ORDER (b), 2026-11-03: "in SPIKE_PLAN's order, but wave 1 runs one heavy spike at a time"). In wave 1, before the foundation: the netcode proof test first; then the sim core spike; then the 60 Hz margin run, with the sim core spike's sim and S1's stand-ins (OQ-0001-2 (a)), an early signal only (TICK-60-SETUP, the Advisor, 2026-11-03); then the boundary spike. In wave 3, after the foundation (phase 2) and before slice 1 (phase 3): the content and ability spike | S1–S14 and S19–S22 recorded in this record; S17 from the netcode proof test; S18 from the boundary spike; S15 and S16 from the content and ability spike (wave 3); option C or B, the navigation generator, the units per tick and the hash function chosen from the numbers |
END

