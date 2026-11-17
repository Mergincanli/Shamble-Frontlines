# Plan: change-set 0, the model-file rules and the record of his decisions (docs only)

_The build chat, 2026-11-17. Base: main c94c81b (merged into the branch). Authority: the build plan
(`.checkpoints/build-plan-slice-1/plan.md`, revision 2, Parts 1, 11 and 12; approved by the advisor agent; his go in
`.checkpoints/build-plan-slice-1/mustafa-2026-11-17-go.md`) and his go message itself, which changes change-set 0: it
now also records his decisions (his CHANGE 1) and takes the Art lead's briefs (his CHANGE 3; the briefs are in
`.checkpoints/build-plan-slice-1/inputs/`)._

## Goal
One docs pull request, no game code, no kit, gate or CI file:
1. the rules a model file must follow, confirmed as far as reading can confirm them, with the names fixed;
2. his decisions of 2026-11-14 and 2026-11-17 in `docs/DECISIONS.md`, by script from this chat's transcript;
3. STATUS and ROADMAP changed only where his decision changes what they say;
4. the build plan on main as a report, with his messages and the briefs in its appendix.

## Files
| File | Change | Who writes it |
|---|---|---|
| `docs/research/asset-export-rules.md` (new) | the export and slot-naming part of the asset research (OQ-0005-AS3, AS4): what the drafts fix, what the sources say (URL and line per claim), the rules, the names, the checks a file passes, what stays open; the "not read" list | graphics-engineer (its agent file allows research topics), from its first reading `asset-rules.md` |
| `docs/research/README.md` | one row for the new topic, with its status | graphics-engineer |
| `docs/adr/0005-content-abilities-assets.md` | OQ-0005-AS3 and AS4: an "answered in part" line each (rules recorded, proof by a real export still to come at change-set M), pointing at the research file; the export part's pass marks appended after PM-0005-44 as PM-0005-47 onward (new ids; no existing mark changed), placed at change-set M | main session (a decision doc) |
| `docs/adr/README.md` | the register's rows for OQ-0005-AS3, AS4 and the new marks' count | main session |
| `docs/LOOK_V2.md` | §8, flow step 2 only: "a 3D model in Spline or Blender" (his words of 2026-11-03). Nothing else: line 30's "no plain tubes or cylinders" is left for the carry of his look decisions, as his words of 2026-11-17 say | main session |
| `docs/DECISIONS.md` | three entries at their places by date (2026-11-17 19:10:44 UTC; 2026-11-14 14:12:13 UTC; 2026-11-14 13:58:05 UTC), his words by script, byte for byte, from the three files saved from the transcript; dated pointers on the three older places his decision changes (SLICE-ORDER condition 3; his words of 2026-11-10 and of 2026-11-13); "Queued": no line leaves (the camera panel leaves with change-set 4) | main session, by script |
| `docs/SPIKE_PLAN.md`, `docs/adr/0002-code-structure-build-toolchain.md` | one dated pointer each, where the earlier pointers stand (the order of the spikes; migration step 1), quoting his decision of 2026-11-14 | main session |
| `docs/STATUS.md`, `docs/ROADMAP.md`, `docs/REPO_MAP.md` | STATUS: Phase, Next and Waiting lines that his decision changes, a row for this pull request; ROADMAP: one entry; REPO_MAP: the new research file and report | docs-keeper |
| `docs/reports/2026-11-17-build-plan-slice-1.md` (new) | the plan (revision 2, as approved), a short "since the plan" section: his answers to the twelve decisions and what each changes, his four changes | main session |
| `docs/reports/2026-11-17-build-plan-slice-1-appendix.md` (new) | his three messages byte for byte; the three Art briefs as received; the sheet notes of 2026-11-14; the digests' count line | main session, by script |
| `docs/reports/2026-11-17-cs0-model-rules.md` (new) | this change-set's report (`/advisor-report`) | main session |
| `.checkpoints/build-plan-slice-1*`, `.checkpoints/cs0-model-rules*` | deleted in the last commit before the pull request is marked ready | main session |

No symbol is added or removed: there is no code.

## The names this change-set fixes (the engine's naming, not the look)
His words of 2026-11-17: "The brief's names for colour areas and bones are working names; the names of your change-set
0 win." The briefs' structure is kept; only the spelling becomes the engine's.
- **Colour areas** (material names; lower case, digits and underscores; the prefix is the class): the Conscript's
  four: `faction_coat`, `faction_top`, `faction_trim`, `unit_dark` (the briefs' Coat, Top, Trim, Dark). A gun has one
  material, `unit_dark`, the same name as the soldier's, so soldier and gun stay within the drafts' 4 materials.
  Classes by prefix: `faction_` (changes colour with the faction; the faction's data holds one colour per such
  area), `unit_`, `emissive_`, `structure_`, `environment_`.
- **Joints of the Conscript** (the brief's rig, 13 joints under one root): `root` (the one node the soldier scales
  by); `hips`; `chest`; `head`; `upper_arm_l`, `forearm_l`, `hand_l` and the same three with `_r`; `leg_l`, `foot_l`,
  `leg_r`, `foot_r`. Parents: root, hips, chest, head; chest, upper arm, forearm, hand; hips, leg, foot. The leg is
  one piece from hip to ankle, as the brief builds it (the coat hides the knee). A later soldier that shows a knee
  brings its own list; the list belongs to the model's entry, not to the engine.
- **Sockets on a gun** (empty nodes; his words give the names): `socket_grip` (the gun's origin), `socket_support`,
  `socket_muzzle`, `socket_sight`, `socket_eject`. The right hand holds `socket_grip`, the left goes to
  `socket_support`; where a pose's hand points and the sockets disagree, the sockets win (his words). The muzzle
  socket is where flash and tracer start; the simulation never reads it (ADR 0005, AP-1).
- **Axes and origin.** Soldier: +Y up, front +Z, his left +X, origin on the ground between the feet, everything
  under `root`. Gun: +Y up, +Z to the muzzle, +X to the gun's left, origin at `socket_grip`. In Blender: Z up, the
  front (or the muzzle) towards -Y. One unit is one metre (EDITOR_V2, decided; his words of 2026-11-17).
- **Optional parts:** a node whose name starts with `marking_` is a part the game may hide (the faction's marking,
  off at game size); it uses one of the model's own materials.
- The plan's Part 11 proposed other joint names before the briefs arrived (spine, upper and lower leg); the list
  above replaces them and the report says so.

## The export part's own marks (new, placed at change-set M)
SPIKE_PLAN asks the research to append its marks to the asset spike. Proposed, as counts on the first real file
(the gun or the soldier, whichever is exported first), each with a planted defect that must fail:
1. the file is GLB, glTF version 2, with no required extension;
2. every primitive has a material, and every material name is in the slot table; 0 unnamed or unknown;
3. every joint name equals the model's list; no extra joint (the exporter's neutral bone is the planted defect);
4. at most one set of joints and weights per vertex (4 influences); every vertex weighted;
5. normals present; no image, animation, morph target, camera or light;
6. a 1 m test cube measures 1.0 m along +Y, and its front faces +Z, after export and load;
7. a gun's five sockets exist by name, `socket_grip` at the origin, `socket_muzzle` on +Z;
8. counts inside the two derived caps (4 materials for a soldier with his gun);
9. the same result on WebGPU and with `forceWebGL`.
Until a real export exists, marks 1 to 9 are written and not run. The rifle prototype his message names
(`AMK9_v4_stage1.glb`) can be the first file; it "stays with me until you ask for it", and change-set M's plan asks.

## What his go changes in the build plan, recorded in the report (not rewritten into the plan's text)
The plan stands as approved. The report adds "since the plan": his answers to the twelve decisions with what each
changes (GATE-FOLLOW (b) with (i); MACHINES and his words on quality; COMPARE (b), his word for PM-0004-34's change;
SCOPE-RELOAD: the simple active reload joins change-set 8; START-RULES: the jump's reach 3.07 m, soldiers push softly
and never block; the rest as recommended), his change 1 (this change-set records his decisions; the 2.3 chat stays
stopped), change 2 (the second builder chat; Part 15 follows as its own file, before change-set 1), change 3 (the
briefs; the weapons line struck from the Art list) and change 4 (Shamble VFX; Part 15).

## Evidence (research and docs: CLAUDE.md, protocol step 4)
- Every claim of the research file cites its source; the verifier spot-checks the key claims against the sources
  and against the drafts' lines (at least 15 claims, the exporter's option names and the loader's behaviour among them).
- His words: the script's output compared byte for byte with the three saved files and those with the transcript
  (sha256 and byte counts in the report); a grep shows no quote of his retyped.
- `node tools/gate.mjs` (the legacy gate; nothing in `legacy/` changes), `node .claude/scripts/repo-map-check.mjs`,
  CI's `gate`, `smoke` and `toolchain` on the head.
- What is not verified, said in the report: no real export has gone through the rules; the Blender option names
  were read on the exporter's main branch, not on his Blender's version; the two pictures of his message (Sheet 16,
  the SMG drawing) did not arrive.

## Reviewers (each at effort max; at most 3 at a time)
graphics-engineer writes; engine-architect reviews the names, the joints and the socket rule; the verifier checks
claims and his words; docs-keeper's pass; then the advisor agent on the final head. One round, a second only on what
changed.

## Risk tier, merge, Brave
Risk tier 1, decision docs (ADR 0005, the register, LOOK_V2, DECISIONS, SPIKE_PLAN, ADR 0002): needs his go. No Brave
check: nothing a player sees. Planned size: about 500 lines of new docs text besides the plan and the appendix,
which are carried, not written. If the written part passes 1,000 lines, stop and tell him in three lines.

## Risks
- The names are fixed before a real export proves the rules; a rule the export disproves is corrected at change-set M.
- Two chats and LOOK_V2: this change-set edits one line of §8; the carry of his look decisions (second lane) owns the
  rest of the file. Part 15 says so.
- The chat is 79 % full: the checkpoint is written so that a compacted or restarted session continues from it.
