Task: wave 2 verifier spot-check of R4 (docs/research/r4-physics-navigation.md), started 2026-10-28.

## Progress
- Read the whole file (1223 lines). 18 recs. M = 51 distinct R4 finding ids in Rests-on lists (PHYS 20, NAV 16, FLOORS 7, LOS 4, OBST 4; script count). Legacy/wave-1 ids not counted in M (R3 convention).
- FLOORS-REC3 and LOS-REC3 name no R4 finding: their basis checked against local docs (GW14, GC3 in docs/LEGACY_INVENTORY.md), not counted in N.
- Plan of checks: PHYS F24,F23,F15,F6,F1,F3,F2,F4,F8,F21,F9,F7,F26,F17; NAV F8,F4,F19,F17,F15,F18; OBST F3,F1,F2,F4; FLOORS F5,F1; NAV-F2; LOS F1,F2.
- Next: fetch sources.

## Verdicts (saved as found)
- V1 R4-PHYS-F24 (Q3 kinematic: 5 clip planes, 4 bumps, STEPSIZE 18, MIN_WALK_NORMAL 0.7, box trace) · bg_local.h + bg_slidemove.c · CONFIRMED · bg_local.h: "#define	MIN_WALK_NORMAL	0.7f		// can't walk on very steep slopes"; "#define	STEPSIZE		18"; bg_slidemove.c: "#define	MAX_CLIP_PLANES	5"; "numbumps = 4;"; "pm->trace ( &trace, pm->ps->origin, pm->mins, pm->maxs, end, pm->ps->clientNum, pm->tracemask);"; "up[2] += STEPSIZE;". The "trace.plane.normal[2] < MIN_WALK_NORMAL" line is not in bg_slidemove.c (presumably bg_pmove.c, listed; not rechecked). Fetch: no rigid-body/mass/impulse code in bg_slidemove.c (paraphrase; matches file's "inferred").
- V2 R4-PHYS-F23 (shared server/client movement) · bg_pmove.c header + comment above Pmove · CONFIRMED · "// bg_pmove.c -- both games player movement code"; "Can be called by either the server or the client".
