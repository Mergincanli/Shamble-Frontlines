# graphics-engineer: fix round 2 (plan step 9), 2026-11-01

Every BEFORE was copied by script from the file at HEAD and occurs exactly once there; AFTER applies the change.
G10 is applied because verifier-skeptic-3's notes record it UPHELD (its results file was not yet written at 14:30).

### FIX F13 (author line)
FILE: docs/adr/0003-rendering.md
BEFORE:
**Author:** graphics-engineer · **Reviewed by:** (filled in at review) · **Signed off by Mustafa:** no
AFTER:
**Author:** graphics-engineer (the "Pass marks" table: test-engineer, plan step 4) · **Reviewed by:** (filled in at review) · **Signed off by Mustafa:** no
END

### FIX F12 (the decision map's codes)
FILE: docs/adr/0003-rendering.md
BEFORE:
How to read the citations: each gives the research file, its section, the id and the lines at commit 9e7db28 (the
topic files do not change in this PR). "Findings Q1" means the Findings section, question 1, of that file; legacy ids
(CD15, CL6, GA15 …) are rows of `docs/LEGACY_INVENTORY.md`. Terms are the
[glossary's](../research/GLOSSARY.md); each is linked at its first use.
AFTER:
How to read the citations: each gives the research file, its section, the id and the lines at commit 9e7db28 (the
topic files do not change in this PR). "Findings Q1" means the Findings section, question 1, of that file; legacy ids
(CD15, CL6, GA15 …) are rows of `docs/LEGACY_INVENTORY.md`. Terms are the
[glossary's](../research/GLOSSARY.md); each is linked at its first use. The
codes this record takes from the decision drafts' working map (interfaces I-n, research gaps "gap Gn", loose ends
LE-x, the map's addendum parts J-n and shared numbers N-n) are kept in [the register's codes](README.md#codes-the-drafts-cite).
END

### FIX F10 (carry-forward 3)
FILE: docs/adr/0003-rendering.md
BEFORE:
  48 and 54, the 48 value holds (carry-forward 3, STATUS:133–134).
AFTER:
  48 and 54, the 48 value holds (carry-forward 3).
END

### FIX F10 (SLICE-ORDER)
FILE: docs/adr/0003-rendering.md
BEFORE:
- **SLICE-ORDER** and its conditions (STATUS:94–101; brief §1): from slice 1 the simulation runs in a Web Worker behind
AFTER:
- **SLICE-ORDER** and its conditions (brief §1): from slice 1 the simulation runs in a Web Worker behind
END

### FIX F10 (SEC-SPREAD)
FILE: docs/adr/0003-rendering.md
BEFORE:
- **SEC-SPREAD (a)** (STATUS:108–109): weapon spread is known only to the server; "tracers use a similar-looking spread
AFTER:
- **SEC-SPREAD (a)**: weapon spread is known only to the server; "tracers use a similar-looking spread
END

### FIX F10 (TICK-30)
FILE: docs/adr/0003-rendering.md
BEFORE:
- **TICK-30** (STATUS:118–120): a 30 Hz server tick is the baseline. The page interpolates between ticks whatever the
AFTER:
- **TICK-30**: a 30 Hz server tick is the baseline. The page interpolates between ticks whatever the
END

### FIX F10 (UPGRADE-SCOPE: checkpoint input and map pointer)
FILE: docs/adr/0003-rendering.md
BEFORE:
- **UPGRADE-SCOPE** (Mustafa, 2026-10-31; `inputs/UPGRADE-SCOPE-mustafa-2026-10-31.md`, recorded in STATUS by
  docs-keeper): the upgrade tracks live on the player; the page only shows the owner's offers and tracks (I-21 of the
  decision map; ADR 0005 designs them).
AFTER:
- **UPGRADE-SCOPE** (Mustafa, 2026-10-31, recorded in STATUS "Decisions pending"): the upgrade tracks live on the
  player; the page only shows the owner's offers and tracks (interface I-21; ADR 0005 designs them).
END

### FIX F10 (SPIKE-SCOPE)
FILE: docs/adr/0003-rendering.md
BEFORE:
- **SPIKE-SCOPE (a)** (STATUS:187–188): spikes are throwaway in `spikes/`, exempt from the legacy constraints.
AFTER:
- **SPIKE-SCOPE (a)**: spikes are throwaway in `spikes/`, exempt from the legacy constraints.
END

### FIX F10 (R7-MARKS)
FILE: docs/adr/0003-rendering.md
BEFORE:
- **R7-MARKS (a)** (STATUS:159–160): the inventory's marks are working recommendations; this draft lists the rows whose
AFTER:
- **R7-MARKS (a)**: the inventory's marks are working recommendations; this draft lists the rows whose
END

### FIX F10 (ADR-003 entry, part 2)
FILE: docs/adr/0003-rendering.md
BEFORE:
   within a year (R3-WGPU-F3, lines 78–83). This also disposes of the old ADR-003 entry ("stay on r128 vs upgrade",
   STATUS:202): the stack settles the upgrade, and this part is its policy. *Ids:* R3-WGPU-REC2 (Recommendations Q1,
AFTER:
   within a year (R3-WGPU-F3, lines 78–83). This also disposes of the old ADR-003 entry ("stay on r128 vs upgrade",
   STATUS, ADR-003): the stack settles the upgrade, and this part is its policy. *Ids:* R3-WGPU-REC2 (Recommendations Q1,
END

### FIX row 73 (5.b)
FILE: docs/adr/0003-rendering.md
BEFORE:
     objects meet the ground (CL6); the screen-space pass draws a line in pixels from depth and normal changes, can
     mark folds inside a shape, and must be masked where the roof fade hides structures (r3-rendering.md, Options Q3,
AFTER:
     objects meet the ground (CL6); the screen-space pass draws a line in pixels from depth and normal changes, can
     mark folds inside a shape (inferred from R3-LOOK-F7; OQ-0003-2), and must be masked where the roof fade hides structures (r3-rendering.md, Options Q3,
END

### FIX X5, rows 95 and X4 (5.i)
FILE: docs/adr/0003-rendering.md
BEFORE:
     r4-physics-navigation.md, Recommendations, R4-FLOORS-REC2, lines 925–939) and between the camera and the player. Its blend state is
     a precompiled variant (part 3), and the ink is masked on faded structures. It follows the server's visibility
     rule: "The client's cutaway (GW17) follows the same rule, so what is not sent is also what the screen would not
     show" (r2-netcode.md, Recommendations, R2-INT-REC2, lines 2445–2466, quote at 2450–2451); what a player may see
AFTER:
     r4-physics-navigation.md, Recommendations, R4-FLOORS-REC2, lines 925–939) and, as legacy GW17 does, between the
     camera and the player (a behaviour LOOK_V2 does not name: OQ-0003-15). Its blend state is
     a precompiled variant (part 3), and the ink is masked on faded structures. It follows the server's visibility
     rule: "The client's cutaway (GW17) follows the same rule, so what is not sent is also what the screen would not
     show" (r2-netcode.md, Recommendations, R2-INT-REC2, lines 2445–2466, quote at 2451–2452); what a player may see
END

### FIX row 100 (5.j)
FILE: docs/adr/0003-rendering.md
BEFORE:
     visual effects (5.k). Planks and debris as routes are ADR 0004's typed links (r4-physics-navigation.md, Recommendations, R4-FLOORS-REC1, lines 903–924); this
AFTER:
     visual effects (5.k). Planks and debris as routes are ADR 0004's (its part 14: planks as typed links to upper floors, debris
     as the ruin's walkable surfaces; the link type list is data: r4-physics-navigation.md, Recommendations,
     R4-FLOORS-REC1, lines 903–924); this
END

### FIX F13 (fragments as damage, 5.k)
FILE: docs/adr/0003-rendering.md
BEFORE:
     technique was not researched: OQ-0003-7. Whether a fragment deals damage is a design question for Mustafa; this
AFTER:
     technique was not researched: OQ-0003-7. Whether a fragment deals damage is a design question for Mustafa (`docs/adr/README.md`,
     "Decisions the drafts ask for": fragments as damage); this
END

### FIX G10 (part 6: per-instance colour)
FILE: docs/adr/0003-rendering.md
BEFORE:
     (ADR 0005, I-8): one shared material per slot and faction, or per-instance colour where meshes are instanced
     (`instanceColor`, R3-CHAR-F4, F6, lines 452–467).
AFTER:
     (ADR 0005, I-8): one shared material per slot and faction, or per-instance colour where meshes are instanced
     (`instanceColor` on `InstancedMesh`, R3-CHAR-F4, lines 452–455; `BatchedMesh` also carries per-instance colour,
     R3-CHAR-F6, lines 463–467).
END

### FIX F6 (part 11: the kit builders)
FILE: docs/adr/0003-rendering.md
BEFORE:
    (CL10, `LEGACY_INVENTORY.md` line 119). It decides nothing about compression or LODs (ASSET-RESEARCH (a), the advisor's ruling; decision map J4).
AFTER:
    (CL10, `LEGACY_INVENTORY.md` line 119). It decides nothing about compression or LODs (ASSET-RESEARCH (a), the advisor's ruling; J4).
    Beside them run ADR 0005's ported kit builders, for the asset families OQ-0005-AP2 asks Mustafa to judge
    (authored model or procedural builder).
END

### FIX F13 (fragments as damage, trace row)
FILE: docs/adr/0003-rendering.md
BEFORE:
| §3 `[DECIDED]` fragmentation comes back | pooled fragments as visual effects from the blast event, on both backends (5.k) | R3-GL2-REC1 (955–967); R3-BUDGET-REC2's visual-effects row (1132); the particle technique not researched: OQ-0003-7 | a grenade blast with flying fragments beside legacy's smoke and fire | 0004 (the blast event); damage per fragment is a design question |
AFTER:
| §3 `[DECIDED]` fragmentation comes back | pooled fragments as visual effects from the blast event, on both backends (5.k) | R3-GL2-REC1 (955–967); R3-BUDGET-REC2's visual-effects row (1132); the particle technique not researched: OQ-0003-7 | a grenade blast with flying fragments beside legacy's smoke and fire | 0004 (the blast event); damage per fragment is a design question (`docs/adr/README.md`, "Decisions the drafts ask for") |
END

### FIX X6 (Q3 bullet)
FILE: docs/adr/0003-rendering.md
BEFORE:
- "WebGPU required, no fallback" was not researched as an option (R3-GL2-OQ5, lines 1317–1320); it stays Mustafa's
  question (OQ-0003-11).
AFTER:
- "WebGPU required, no fallback" appears only as settled by the stack (`docs/V2_PHASE0_BRIEF.md` §0, line 8; R3-GL2-OQ5,
  lines 1317–1320); it is not weighed.
END

### FIX row 73 (Q5 cell)
FILE: docs/adr/0003-rendering.md
BEFORE:
| **Screen-space depth and normal edges** (proposed, 5.b; F7) | Medium: a custom TSL node | one full-screen pass plus one normal attachment, about 16.6 MB at 1080p (F14) | scales with pixels only; Low stays at 800 draws | presentation only | lines in pixels; can mark folds inside a shape; threshold tuning; thin lines alias; needs masking for the roof fade | new |
AFTER:
| **Screen-space depth and normal edges** (proposed, 5.b; F7) | Medium: a custom TSL node | one full-screen pass plus one normal attachment, about 16.6 MB at 1080p (F14) | scales with pixels only; Low stays at 800 draws | presentation only | lines in pixels; can mark folds inside a shape (inferred: OQ-0003-2); threshold tuning; thin lines alias; needs masking for the roof fade | new |
END

### FIX X3 (Q5 hulls bullet)
FILE: docs/adr/0003-rendering.md
BEFORE:
- **The hulls.** Pros: legacy's proven look; a per-object weight for buildings is a plain thickness value (F5).
AFTER:
- **The hulls.** Pros: legacy's proven look (our own TSL hull); in our own hull a per-object weight is a plain
  thickness value, as legacy's CL6 does (characters 0.8×); the shipped pass takes one thickness for all objects (F5),
  so a per-object weight there is not researched.
END

### FIX X2 (M-D cost)
FILE: docs/adr/0003-rendering.md
BEFORE:
| M-D. A cloud GPU instance | Medium | rented by the minute | repeatable; not a laptop | runner access is ADR 0006's | – | CI setup |
AFTER:
| M-D. A cloud GPU instance | Medium | rented (GitHub's GPU runners bill per minute, R3-MEASURE-F9, lines 655–658); R6's cost | repeatable; not a laptop | runner access is ADR 0006's | – | CI setup |
END

### FIX X3 (trade-off)
FILE: docs/adr/0003-rendering.md
BEFORE:
  strongest argument against:* LOOK_V2 §2 keeps legacy's ink, and legacy's ink is a hull; the hull is shipped code
  with a known look and a trivial per-object weight for buildings, while the screen-space pass is our own TSL node with
AFTER:
  strongest argument against:* LOOK_V2 §2 keeps legacy's ink, and legacy's ink is a hull; the hull is legacy's known
  look (our own TSL hull, with a plain per-object weight for buildings, CL6) or shipped code (one thickness for the
  pass, R3-LOOK-F5), while the screen-space pass is our own TSL node with
END

### FIX X6 (revisit when)
FILE: docs/adr/0003-rendering.md
BEFORE:
  the fallback share (R3-GL2-OQ2), which may reopen dropping the fallback (OQ-0003-11) or allow F-B extras; the camera
AFTER:
  the fallback share (R3-GL2-OQ2), which may allow F-B extras; the camera
END

### FIX X5 (GW17 row)
FILE: docs/adr/0003-rendering.md
BEFORE:
| GW17 (205) | port to TypeScript (pending) | port to TypeScript: the roof fade by flag over the player's enclosure, precompiled blend variant, ink masked (part 5.i); pending removed | LOOK_V2 §4 `[DEFAULT]` roofs fade |
AFTER:
| GW17 (205) | port to TypeScript (pending) | port to TypeScript: the roof fade by flag over the player's enclosure and the camera-player cutaway (OQ-0003-15), precompiled blend variant, ink masked (part 5.i); pending removed | LOOK_V2 §4 `[DEFAULT]` roofs fade |
END

### FIX row 211 (the HUD line split by mark)
FILE: docs/adr/0003-rendering.md
BEFORE:
| GP7, GP10, GP12, PG2–PG8, PG10–PG12, PG14–PG16, PG20 (317, 320, 322, 360–366, 368–370, 372–374, 378) | port to TypeScript (pending) | unchanged; pending stays for the HUD's layout, which LOOK_V2 does not set | LOOK_V2 covers the 3D look, not the HUD |
AFTER:
| GP7, GP10, GP12, PG3, PG7, PG8, PG10, PG14, PG16 (317, 320, 322, 361, 365, 366, 368, 372, 374) | port to TypeScript (pending) | unchanged; pending stays for the HUD's layout, which LOOK_V2 does not set | LOOK_V2 covers the 3D look, not the HUD |
| PG2, PG4–PG6, PG11, PG12, PG15, PG20 (360, 362–364, 369, 370, 373, 378) | redesign (pending) | unchanged; pending stays for the HUD's layout, which LOOK_V2 does not set | LOOK_V2 covers the 3D look, not the HUD |
END

### FIX X3 (OQ-0003-2 options)
FILE: docs/adr/0003-rendering.md
BEFORE:
  (screen-space); inner lines from normal edges (screen-space) or none beyond the silhouette (hull); the building
  weight as a per-pixel class value in an MRT output (screen-space, not researched) or a per-object thickness (hull).
AFTER:
  (screen-space); inner lines from normal edges (screen-space) or none beyond the silhouette (hull); the building
  weight as a per-pixel class value in an MRT output (screen-space, not researched) or a per-object thickness (our own TSL hull, as CL6; the
  shipped pass has one thickness for all objects, R3-LOOK-F5).
END

### FIX X6 (OQ-0003-11)
FILE: docs/adr/0003-rendering.md
BEFORE:
- **OQ-0003-11. Should v2 one day drop the WebGL 2 fallback, and how many players use it?**
  Why it matters: the fallback doubles the test matrix; dropping it shuts out Firefox on Linux and Intel Macs, Chrome
  on Linux without Intel Gen12+, and Safari before 26 (R3-GL2-F12, lines 279–283). Options: keep it with the same
  features (part 1); "WebGPU required" (not researched). Decider: Mustafa. What decides it: the fallback share from
  client field data after launch (part 8). Research ids: R3-GL2-OQ5 (lines 1317–1320), R3-GL2-OQ2 (lines
  1308–1311).
AFTER:
- **OQ-0003-11. How many players use the WebGL 2 fallback?**
  Why it matters: the fallback keeps Firefox on Linux and Intel Macs, Chrome on Linux without Intel Gen12+, and Safari
  before 26 in the game (R3-GL2-F12, lines 279–283), and its share decides whether WebGPU-only extras (F-B) are worth
  their fallback work. Options: none to weigh: the share is measured, from client field data (part 8). The fallback
  itself stays (settled by the stack: R3-GL2-OQ5 is answered in Appendix A). Decider: Mustafa, on client field data after
  launch. What decides it: the backend each match reports (part 8). Research ids: R3-GL2-OQ2 (lines
  1308–1311).
END

### FIX X8 (OQ-0003-14 answered)
FILE: docs/adr/0003-rendering.md
BEFORE:
- **OQ-0003-14. Do the Web Audio specification's PannerNode and spatialization sections change part 9?**
  Why it matters: MDN stood in for the specification, whose fetched text stopped before those sections
  (R3-AUDIO-F9, lines 743–747). Options: a read of the specification's sections; keep MDN's pages. Decider: the
  verifier (plan step 7, VERIFY-FETCH (a)) or the render budget bench's preparation. What decides it: whether the
  specification's text differs from MDN on the panning and distance models. Research ids: R3-AUDIO-OQ4 (lines
  1401–1402).
AFTER:
- **OQ-0003-14. Do the Web Audio specification's PannerNode and spatialization sections change part 9?**
  Answered by its decider, the verifier (plan step 7, 2026-11-01), from the W3C Working Draft of 22 September 2026
  (https://www.w3.org/TR/webaudio-1.1/): `panningModel` "Defaults to "equalpower"", `distanceModel` "Defaults to
  "inverse""; equal power is "a simple and relatively inexpensive algorithm" and HRTF "somewhat more costly than
  "equalpower"", as MDN says (R3-AUDIO-F2, F3, lines 696–709). Part 9 is unchanged. Research ids: R3-AUDIO-OQ4 (lines
  1401–1402).
END

### FIX X5 (OQ-0003-15 items)
FILE: docs/adr/0003-rendering.md
BEFORE:
  comic words (CL12, GW3), the kill vignette (GP9), the toy-soldier death fall (GA14) and the skin rim (CL3).
AFTER:
  comic words (CL12, GW3), the kill vignette (GP9), the toy-soldier death fall (GA14), the skin rim (CL3) and the
  camera-player cutaway, which fades structures between the camera and the player beyond LOOK_V2 §4's roofs (GW17,
  `LEGACY_INVENTORY.md` line 205).
END

### FIX F4 (look proof test: MAC)
FILE: docs/adr/0003-rendering.md
BEFORE:
  Safari 26, and on the WebGL 2 backend (`forceWebGL`) in the same browsers, with zero console errors; the two
AFTER:
  Safari on MAC (`docs/SPIKE_PLAN.md`, shared fixtures), and on the WebGL 2 backend (`forceWebGL`) in the same browsers, with zero console errors; the two
END

### FIX F6 (look proof test: ADR 0005's share)
FILE: docs/adr/0003-rendering.md
BEFORE:
- **Needs:** LOOK_V2; both reference machines (OQ-0003-1); Mustafa's time in Brave; a few authored test assets as
  plain glTF/GLB exports (part 11); legacy screenshots at the same framing.
AFTER:
- **ADR 0005's share runs in the same test:** its asset marks PM-0005-33 to PM-0005-37 and the families checklist for
  OQ-0005-AP2 (which asset families become authored models and which stay procedural builders: Mustafa's pick).
- **Needs:** LOOK_V2; both reference machines (OQ-0003-1); Mustafa's time in Brave; a few authored test assets as
  plain glTF/GLB exports (part 11), with ADR 0005's ported kit builders beside them and a minimal content loader that
  runs the content crate's whole-set check (ADR 0005); legacy screenshots at the same framing.
END

### FIX F4 (test plan: MAC)
FILE: docs/adr/0003-rendering.md
BEFORE:
- **Backend matrix:** Chrome, Edge, Firefox (Windows) and Safari 26 on WebGPU; the same with WebGL 2 forced; Firefox on
AFTER:
- **Backend matrix:** Chrome, Edge, Firefox (Windows) and Safari on MAC (`docs/SPIKE_PLAN.md`) on WebGPU; the same with WebGL 2 forced; Firefox on
END

### FIX X4 (Appendix A, R2-XT-OQ15)
FILE: docs/adr/0003-rendering.md
BEFORE:
| R2-XT-OQ15 | answered: part 5.i | the cutaway follows R2-INT-REC2's rule (r2-netcode.md lines 2450–2451); what a player may see stays R2-INT-OQ1, a design question |
AFTER:
| R2-XT-OQ15 | answered: part 5.i | the cutaway follows R2-INT-REC2's rule (r2-netcode.md lines 2451–2452); what a player may see stays R2-INT-OQ1, a design question |
END

### FIX F10 (Appendix A, R3-WGPU-REC2)
FILE: docs/adr/0003-rendering.md
BEFORE:
| R3-WGPU-REC2 | adopted | part 2 (also disposes of the old ADR-003 entry, STATUS:202) |
AFTER:
| R3-WGPU-REC2 | adopted | part 2 (also disposes of the old ADR-003 entry in STATUS) |
END

### FIX X6 (Appendix A, R3-GL2-OQ2)
FILE: docs/adr/0003-rendering.md
BEFORE:
| R3-GL2-OQ2 | open: OQ-0003-11 | the share waits for client field data; what the client reports is part 8 |
AFTER:
| R3-GL2-OQ2 | open: OQ-0003-11 | the share, from client field data; what the client reports is part 8 |
END

### FIX X6 (Appendix A, R3-GL2-OQ5)
FILE: docs/adr/0003-rendering.md
BEFORE:
| R3-GL2-OQ5 | open: OQ-0003-11 | Mustafa, with the share |
AFTER:
| R3-GL2-OQ5 | answered: settled by the stack | part 1: the fallback stays (`docs/V2_PHASE0_BRIEF.md` §0, line 8) |
END

### FIX X8 (Appendix A, R3-AUDIO-OQ4)
FILE: docs/adr/0003-rendering.md
BEFORE:
| R3-AUDIO-OQ4 | open: OQ-0003-14 | a read of the Web Audio specification |
AFTER:
| R3-AUDIO-OQ4 | answered: OQ-0003-14 | the verifier read the Web Audio 1.1 draft (2026-11-01); part 9 unchanged |
END

### FIX X6 and X8 (Appendix A counts)
FILE: docs/adr/0003-rendering.md
BEFORE:
Counts: adopted 23 · changed 2 · rejected 0 · answered 15 · open 12 · spike 19 (71 in all).
AFTER:
Counts: adopted 23 · changed 2 · rejected 0 · answered 17 · open 10 · spike 19 (71 in all).
END

### FIX F12 (Sources: GLOSSARY §8)
FILE: docs/adr/0003-rendering.md
BEFORE:
  and visual effect), §4 (the authority; shared stream and authority-only stream), §5 (preview); §8 as the decision map's addendum (J1) gives it.
AFTER:
  and visual effect), §4 (the authority; shared stream and authority-only stream), §5 (preview), §8 (the foundation,
  the netcode proof test).
END

### FIX F10 (Sources: STATUS)
FILE: docs/adr/0003-rendering.md
BEFORE:
  `docs/V2_PHASE0_BRIEF.md` §0 (line 8); `docs/STATUS.md` "Decisions pending" (lines 93–202).
AFTER:
  `docs/V2_PHASE0_BRIEF.md` §0 (line 8); `docs/STATUS.md` "Decisions pending" (cited by decision code).
END

### FIX F12 (Sources: the map)
FILE: docs/adr/0003-rendering.md
BEFORE:
- The decision drafts' working files: the decision map (`engine-architect-decision-map.md`, parts a, c, d, e, g, h
  and j, and its id table), the plan (revision 3) and the advisor's verdict on it (§1, the ASSET-RESEARCH ruling).
AFTER:
- `docs/adr/README.md`, "Codes the drafts cite" (the decision map's codes this record uses: I-n, gaps, LE-B, J4,
  N12) and "Decisions the drafts ask for"; ASSET-RESEARCH (a), the advisor's ruling, as STATUS "Decisions pending"
  records it.
END

### NOTE F4 (0003:897)
0003:897's "MAC, a macOS machine with Safari 26" is in the machine key under the "Pass marks" table, which test-engineer
wrote (plan step 4); my brief leaves those rows to it, so the key's MAC line is test-engineer's to change to "MAC
(SPIKE_PLAN)". My own Safari mentions (look proof test, test plan) are fixed above.
END

### NOTE F13 (0003:1076)
0003:1076 ("Whether fragments deal damage is not judged here (a design question).") sits under the Pass marks table
(test-engineer's text). Its README pointer is test-engineer's to add; 5.k and the trace row now cite README's row.
END

### NOTE X5 (checklist step 29)
The camera-player cutaway is added to OQ-0003-15, 5.i and the GW17 row; the Brave checklist's step 29 is
test-engineer's (as my brief says), so the step itself is left to it.
END

### NOTE X6 and X8 (README)
docs/adr/README.md:63 still carries OQ-0003-11's old question ("Should v2 one day drop the WebGL 2 fallback, and how
many players use it?"); with the FIX above it reads "How many players use the WebGL 2 fallback?", and OQ-0003-14 is
answered. Both are the synthesis's rows to update.
END

### FIX G10 and F12 (part 2: per-instance colour; the I-8 pointer)
FILE: docs/adr/0005-content-abilities-assets.md
BEFORE:
   example environment, unit, faction colour, emissive); the list and the names are this ADR's content data (decision
   map I-8), consumed by ADR 0003. Faction colour slots take the faction's colour at load, as one shared material per
   slot and faction, or as per-instance colour where a mesh is instanced (`instanceColor` on `InstancedMesh` and
   `BatchedMesh`: r3, Findings Q4, R3-CHAR-F4, F6, lines 452–467; R3-CHAR-REC3, Recommendations Q4, lines
   1065–1075). Emissive slots write the emissive output that drives the glow, so only they glow (r3, Findings Q3,
AFTER:
   example environment, unit, faction colour, emissive); the list and the names are this ADR's content data (interface
   I-8 in `docs/adr/README.md`), consumed by ADR 0003. Faction colour slots take the faction's colour at load, as one shared material per
   slot and faction, or as per-instance colour where a mesh is instanced (`instanceColor` on
   `InstancedMesh`: r3, Findings Q4, R3-CHAR-F4, lines 452–455; `BatchedMesh` also carries per-instance colour:
   R3-CHAR-F6, lines 463–467; R3-CHAR-REC3, Recommendations Q4, lines
   1065–1075). Emissive slots write the emissive output that drives the glow, so only they glow (r3, Findings Q3,
END

### FIX G22 (part 4: the Low draw-call total)
FILE: docs/adr/0005-content-abilities-assets.md
BEFORE:
   - The frame totals every asset shares stay ADR 0003's (r3, R3-BUDGET-REC2, lines 1121–1137): at Low, ≤ 800 draw
     calls, ≤ 500k triangles, ≤ 512 MB of GPU memory; at High, ≤ 1,500, ≤ 1.5M, ≤ 1 GB.
AFTER:
   - The frame totals every asset shares stay ADR 0003's (r3, R3-BUDGET-REC2, lines 1121–1137): at Low, ≤ 800 draw
     calls (without a per-object outline pass; about 1,100 with one, r3 line 1133), ≤ 500k triangles, ≤ 512 MB of GPU memory; at High, ≤ 1,500, ≤ 1.5M, ≤ 1 GB.
END

### FIX condition 12 (Options: a baked look)
FILE: docs/adr/0005-content-abilities-assets.md
BEFORE:
| A look baked into each asset by the modelling tool | not researched | – | – | – | – | – |

- **Shared per class.** Pros: bounded materials; one place to tune the look. Cons: every slot must be named into a
AFTER:

- A look baked into each asset by the modelling tool: settled by LOOK_V2 §8 (step 5, "the engine applies the comic
  look"); not weighed.
- **Shared per class.** Pros: bounded materials; one place to tune the look. Cons: every slot must be named into a
END

### FIX G10 (Options: per-instance colour)
FILE: docs/adr/0005-content-abilities-assets.md
BEFORE:
| Per-instance colour (`instanceColor`, R3-CHAR-F4, F6) | Low | none per faction | only with instanced meshes (C-D, C-E: measured against C-B in ADR 0003) | presentation only | the same | new |
AFTER:
| Per-instance colour (`instanceColor` on `InstancedMesh`, R3-CHAR-F4; per-instance colour on `BatchedMesh`, R3-CHAR-F6) | Low | none per faction | only with instanced meshes (C-D, C-E: measured against C-B in ADR 0003) | presentation only | the same | new |
END
