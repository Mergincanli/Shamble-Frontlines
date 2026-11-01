# Second fix round for ADR-0004 (plan step 9, after the consistency pass)

Author: rust-reviewer, 2026-11-01. Against `docs/adr/0004-simulation-core.md` at the branch head 0f451c9 (1,243 lines).
Every BEFORE was copied from the file by script and is unique in it; the fixes were applied in this order to a scratch
copy and each BEFORE stayed unique, so they do not overlap. No table row of "### Pass marks" is touched. Findings
fixed: F1, F2, F3, F4, F7, F14 (author line, timeline, fragments). New ledger rows: rust-reviewer-adr0004-claims.md,
section "Second fix round".

### FIX F14-author
FILE: docs/adr/0004-simulation-core.md
BEFORE:
**Author:** rust-reviewer · **Reviewed by:** (filled in at review) · **Signed off by Mustafa:** no
AFTER:
**Author:** rust-reviewer (the "Pass marks" table: test-engineer, plan step 4) · **Reviewed by:** (filled in at review) · **Signed off by Mustafa:** no
END

### FIX F3-part6-a
FILE: docs/adr/0004-simulation-core.md
BEFORE:
   [the tick period](../research/GLOSSARY.md#tick-period): 8.3 ms at 30 Hz (4.2 ms in the 60 Hz margin run only). Its
AFTER:
   [the tick period](../research/GLOSSARY.md#tick-period): 8.33 ms at 30 Hz (4.17 ms in the 60 Hz margin run only). Its
END

### FIX F3-part6-b
FILE: docs/adr/0004-simulation-core.md
BEFORE:
   step time in Wasm in the Worker on the minimum laptop of at most 40 % of the tick period: 13.3 ms at 30 Hz (6.7 ms at
   60 Hz), with Wasm at most 2 × native. It applies wherever the browser runs the whole sim: the local server of slices 1
   and 2 (SLICE-ORDER), at an N that R1-TICK-OQ7 sets (a design question for Mustafa and the Advisor, in
AFTER:
   step time in Wasm in the Worker on the minimum laptop of at most 40 % of the tick period: 13.33 ms at 30 Hz (6.67 ms
   at 60 Hz), with Wasm at most 2 × native. It applies wherever the browser runs the whole sim: the local server of
   slices 1 and 2 (SLICE-ORDER), at an N that R1-TICK-OQ7 sets (a design question for Mustafa and the Advisor, in
END

### FIX F1
FILE: docs/adr/0004-simulation-core.md
BEFORE:
    for ABL-PREDICT; the sim code is the same either way. Definitions are read by id (I-1), and a development reload swaps
    them between ticks, never inside one (R1-DATA-REC5, ADR 0005's).
AFTER:
    for ABL-PREDICT; the sim code is the same either way. Definitions are read by id (I-1). In development they change
    at a match boundary under ADR 0005's CD-7 (proposed); if OQ-0005-CD4 chooses its option (b), a reload swaps them
    between ticks, never inside one (R1-DATA-REC5).
END

### FIX F2-part18-drop
FILE: docs/adr/0004-simulation-core.md
BEFORE:
    unit definition (ADR 0005). When a soldier dies, its weapon becomes a pickup entity in the pickup pool: an id (part
    2), the weapon type id, a position settled on the ground with the movement query, and a lifetime in ticks from ADR
AFTER:
    unit definition (ADR 0005). When a soldier dies, the weapons the drop rule names (OQ-0005-AB9) become pickup entities
    in the pickup pool, each with an id (part 2), the weapon type id, a position settled on the ground with the movement
    query, and a lifetime in ticks from ADR
END

### FIX F2-part18-open
FILE: docs/adr/0004-simulation-core.md
BEFORE:
    level (ADR 0005's open questions); which deaths drop which weapons, how long a drop lies, who may pick it up and how
    a tie is broken (OQ-0004-5); and what bots and slot takeovers do with records (OQ-0004-6).
AFTER:
    level (ADR 0005's open questions); which deaths drop which weapons, how long a drop lies, who may pick it up and which
    order breaks a tie (OQ-0005-AB9); and what bots and slot takeovers do with records (OQ-0005-AB10).
END

### FIX F14-timeline
FILE: docs/adr/0004-simulation-core.md
BEFORE:
    R6-TEL-OQ3), so the moments and the telemetry read the same stream. The timeline is sampled every 10 s of game time.
AFTER:
    R6-TEL-OQ3), so the moments and the telemetry read the same stream. ADR 0006's record samples the timeline every 10 s
    of game time (R6-TEL-REC1); the sim supplies the samples at that cadence (part 4).
END

### FIX F2-OQs
FILE: docs/adr/0004-simulation-core.md
BEFORE:
- **OQ-0004-5. Which weapons drop, and who may take them?** (beyond ADR 0005's carry slots, looted ammo and levelling
  speed)
  - Why it matters: part 18 builds pickups as world entities, but UPGRADE-SCOPE says only "Other weapons come only from
    killed enemies' drops; dropped weapons are world pickups".
  - Options, each a rule for Mustafa: which deaths drop a weapon (any death, or only a death to an enemy); which weapons
    drop (the class's own weapon, looted ones, or both); how long a drop lies and how many may lie on the map; who may
    pick it up (the dead soldier's enemies only, or anyone); how two claims in one tick are broken (fixed order, a
    rotating start, nearest first).
  - Decider: Mustafa with the game-designer; ADR 0005 holds the answers as data.
  - What decides it: the in-match upgrade design session.
  - Research ids: none (a research gap); UPGRADE-SCOPE.
- **OQ-0004-6. What do bots and slot takeovers do with player records?**
  - Why it matters: bots fill slots and humans take over bot slots (R2-ROOM-REC6), so a slot's record can change hands
    mid-match.
  - Options: whether bots level tracks and take picks; whether a human taking over a bot's slot gets that slot's record
    or a fresh one.
  - Decider: Mustafa with the game-designer.
  - What decides it: the in-match upgrade design session.
  - Research ids: none; UPGRADE-SCOPE; cites R2-ROOM-REC6 (r2-netcode.md, lines 2842–2856).
AFTER:
- **OQ-0004-5. Which weapons drop, and who may take them?** Moved: one question, in ADR 0005, OQ-0005-AB9
  (UPGRADE-SCOPE's data model is ADR 0005's). This record keeps only the mechanism: two claims on one pickup in one
  tick resolve in the phase's fixed, deterministic order (part 18); which order is fair, which deaths drop which
  weapons, how long a drop lies and who may take it (the killer's team, the dead soldier's enemies, or anyone) are
  OQ-0005-AB9's.
- **OQ-0004-6. What do bots and slot takeovers do with player records?** Moved: one question, in ADR 0005,
  OQ-0005-AB10. How a bot picks, if bots pick, is bot behaviour (part 15).
END

### FIX F14-fragments
FILE: docs/adr/0004-simulation-core.md
BEFORE:
counts (R2-BUD-OQ5). Not yet in that register: whether grenade fragments deal damage per fragment (LOOK_V2 §3 and §7
bring fragmentation back as a visible effect, which ADR 0003 draws from the blast event; a rule for Mustafa, with no
research id).
AFTER:
counts (R2-BUD-OQ5). Also asked by ADR 0003 and listed once in `docs/adr/README.md`'s "Decisions the drafts ask for":
whether grenade fragments deal damage per fragment (LOOK_V2 §3 and §7 bring fragmentation back as a visible effect,
which ADR 0003 draws from the blast event; Mustafa with the game-designer; no research id).
END

### FIX F3-S1
FILE: docs/adr/0004-simulation-core.md
BEFORE:
| S1 | The sim fits the sim step share at N = 48 | 6 | p99 sim step time ≤ 8.3 ms at 30 Hz (≤ 4.2 ms for the 60 Hz margin run); mean, p50, p95, p99 and max reported per N at N = 12, 24, 36, 48 and the N = 64 [headroom](../research/GLOSSARY.md#the-design-point-and-headroom) run; 36,000 ticks per run (20 minutes of game time at 30 Hz, 10 at 60 Hz: arithmetic), 3 runs; each part of the share reported on its own against its ceiling (R1-TICK-REC2 P1, r1-simulation-core.md lines 2044–2053; R1-ECS-REC1 mark 1, lines 1923–1931; GLOSSARY.md lines 53–69) | the sim core spike |
AFTER:
| S1 | The sim fits the sim step share at N = 48 | 6 | p99 sim step time ≤ 8.33 ms at 30 Hz (≤ 4.17 ms for the 60 Hz margin run); mean, p50, p95, p99 and max reported per N at N = 12, 24, 36, 48 and the N = 64 [headroom](../research/GLOSSARY.md#the-design-point-and-headroom) run; 36,000 ticks per run (20 minutes of game time at 30 Hz, 10 at 60 Hz: arithmetic), 3 runs; each part of the share reported on its own against its ceiling (R1-TICK-REC2 P1, r1-simulation-core.md lines 2044–2053; R1-ECS-REC1 mark 1, lines 1923–1931; GLOSSARY.md lines 53–69) | the sim core spike |
END

### FIX F3-S2
FILE: docs/adr/0004-simulation-core.md
BEFORE:
| S2 | The browser runs the whole sim within the client's sim share | 6 | p99 sim step time ≤ 13.3 ms at 30 Hz in Wasm in a dedicated Worker on the minimum laptop while the page holds 30+ fps on Low for the whole run; Wasm ≤ 2 × native (R1-TICK-REC2 P2, lines 2054–2057; R1-ECS-REC1 mark 1); N for the pass from R1-TICK-OQ7 (a design question), the scale steps reported meanwhile | the sim core spike's sim in the boundary spike's Worker |
AFTER:
| S2 | The browser runs the whole sim within the client's sim share | 6 | p99 sim step time ≤ 13.33 ms at 30 Hz in Wasm in a dedicated Worker on the minimum laptop while the page holds 30+ fps on Low for the whole run; Wasm ≤ 2 × native (R1-TICK-REC2 P2, lines 2054–2057; R1-ECS-REC1 mark 1); N for the pass from R1-TICK-OQ7 (a design question), the scale steps reported meanwhile | the sim core spike's sim in the boundary spike's Worker |
END

### FIX F7
FILE: docs/adr/0004-simulation-core.md
BEFORE:
| S20 | No panic from input or content; recovery online | 20 | zero panics over the fuzzed content corpus (ADR 0005's mark, R1-RR-OQ3) and over fuzzed commands fed to the sim (proposed: part 20's rule needs its own test, the decoders being ADR 0001's); a trap injected into the client's Worker ends in a fresh instance back in step through ADR 0001's resync mark, and the trap reaches error reporting (R6-OBS-REC3) | the sim core spike with the boundary spike |
AFTER:
| S20 | No panic from input or content; recovery online | 20 | zero panics over the fuzzed content corpus (ADR 0005's mark, R1-RR-OQ3) and over fuzzed commands fed to the sim (proposed: part 20's rule needs its own test, the decoders being ADR 0001's); a trap injected into the client's Worker ends in a fresh instance back in step through ADR 0001's resync mark, and the trap reaches error reporting (R6-OBS-REC3) | the sim core spike with the boundary spike |
| S21 | Throws land the same on Wasm and native | 7, 10 | 10,000 random throws: the trajectory function gives bit-identical points on Wasm and native (R2-SYNC-REC6 mark 1, r2-netcode.md line 2218; the id is ADR 0001's, and the shared trajectory function runs in the sim crate) | the sim core spike |
| S22 | One feature set builds the sim on every target | 7 | the resolved feature list of every dependency of `sim` (`cargo tree -e features`) is identical for the `server` build and the `sim-wasm` build, and a planted extra feature on a shared dependency in `server` fails the check (R5-LAYOUT-REC4, r5-toolchain-quality.md lines 1216–1218; the id is ADR 0002's, and the foundation re-proves it as a gate check) | the sim core spike |
END

### FIX F4
FILE: docs/adr/0004-simulation-core.md
BEFORE:
  (proposed: the sim's speed differs by engine), and Edge is reported. **MAC**, a macOS machine with Safari 26, for the
  Safari runs (real Safari through its WebDriver: R5-TEST-REC2, r5 lines 1415–1418); its model recorded; never a budget
AFTER:
  (proposed: the sim's speed differs by engine), and Edge is reported. **MAC**, SPIKE_PLAN's macOS machine: the stable
  Safari on the day of the run, its version recorded (WebGPU from Safari 26.0, R3-WGPU-F19, r3 lines 171–174;
  WebTransport from 26.4, R2-TRN-F7, r2 lines 959–975), for the Safari runs (real Safari through its WebDriver:
  R5-TEST-REC2, r5 lines 1415–1418); its model recorded; never a budget
END

### FIX F2-PMtext
FILE: docs/adr/0004-simulation-core.md
BEFORE:
- Part 18's open rules (which deaths drop which weapons, who may pick up, ties, takeovers: OQ-0004-5 and -6; carry
AFTER:
- Part 18's open rules (which deaths drop which weapons, who may pick up, ties, takeovers: OQ-0005-AB9 and AB10; carry
END

### FIX F2-migration
FILE: docs/adr/0004-simulation-core.md
BEFORE:
| 6 | Slice 4, the loop: economy and objective phases, the ability system's execution, player records, pickups and offers (after OQ-0004-5, OQ-0004-6 and ADR 0005's upgrade questions), events and moments, structure states and damage stages (after OQ-0004-4) | S13, S15 and S16 on the real crate; a recorded full match replays with identical per-tick hashes on every target |
AFTER:
| 6 | Slice 4, the loop: economy and objective phases, the ability system's execution, player records, pickups and offers (after ADR 0005's upgrade questions, OQ-0005-AB9 and AB10 among them), events and moments, structure states and damage stages (after OQ-0004-4) | S13, S15 and S16 on the real crate; a recorded full match replays with identical per-tick hashes on every target |
END

### FIX F3-migration
FILE: docs/adr/0004-simulation-core.md
BEFORE:
| 8 | Slice 6, scale to 48: the stress scene on the real sim at N = 12, 24, 36, 48 and the N = 64 headroom run; the 60 Hz margin run if ADR 0001 schedules it | S1 on the real crate: p99 sim step time ≤ 8.3 ms at N = 48, 3 runs per build |
AFTER:
| 8 | Slice 6, scale to 48: the stress scene on the real sim at N = 12, 24, 36, 48 and the N = 64 headroom run; the 60 Hz margin run if ADR 0001 schedules it | S1 on the real crate: p99 sim step time ≤ 8.33 ms at N = 48, 3 runs per build |
END

### NOTE F4-F2-PM
FIX F4 (the MAC key, lines 873–874) and FIX F2-PMtext (line 1047) edit prose inside the "### Pass marks"
subsection, which test-engineer wrote; neither is a table row. The session assigned F4 and F2 to me, so the texts are
here; route them to test-engineer instead if preferred. Left for test-engineer (table rows): PM-0004-68's cell "MAC:
Safari 26" (read "MAC"), and the key's "(arithmetic; the draft and GLOSSARY round the first two)" (line 924), which
F3's fixes make stale for the draft (it now says 8.33 and 13.33). F7's rows for S21 and S22 are test-engineer's to add.
END

### NOTE F2-README
F2 needs ADR 0005's AB9 to absorb the two options OQ-0004-5 held (the tie order; "the dead soldier's enemies" as a
third pick-up option) and README to drop rows 69–70; FIX F2-OQs keeps OQ-0004-5 and -6 as pointers, as the session's
message asks (the critic proposed deleting them), so README's existing pointers still land.
END

