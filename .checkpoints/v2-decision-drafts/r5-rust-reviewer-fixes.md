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

### NOTE COMMUNITY-SERVERS
FILE: docs/adr/0004-simulation-core.md
no change: the grep of ADR 0004 ("room process|match record|supervisor", and wider: lobby, official, community,
directory, config, room, content hash, mode id) finds no sentence that needs the official lobby or supervisor for a
room to start or run, so no pointer to OQ-0006-21 (condition 6: "A pointer outside ADR 0006 goes only where a sentence
would otherwise read as closing the door"). The hits: the sim runs "natively in each room process on the server" (:123);
its capacities are "set at match start from the mode and the content set" (:137); "the host draws" the seed "and passes
it in at match start" (:305-307); "the room writes the match record at Ended (ADR 0001), and ADR 0006 owns the record's
shape" (:549); part 20 (:562-573), where after a fatal panic the room reports it, tells its clients and "exits non-zero,
so the supervisor records the exit and the VM's other rooms run on". That last clause says what the official deployment
does with the exit code. The room's own steps need no supervisor, so a room run on its own ends the same way. The lobby
appears nowhere in ADR 0004. The version and content-hash check is ADR 0001's `Join`, and an official or community mark
would go in ADR 0006's record (OQ-0006-21); neither is in this record. Not checked: OQ-0006-21's own text (it is being
written in this round).
END

