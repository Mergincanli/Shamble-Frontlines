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

---

# Revision 2: changes after the advisor agent's review (REVISE, 2026-11-17; its notes: `advisor-plan-r1.md`)
Where this part and the text above differ, this part stands. Numbers are the review's findings.

**1 (High). The joints.** The 13-joint list above is withdrawn, and so is the sentence that a list "belongs to the
model's entry". There is one engine joint list for Slice 1's soldier. engine-architect rules on it against the legacy
animation rows GA1 to GA15 and ADR 0003 lines 794 to 797 (knees, the spine, the coat's panels), with the brief's
rigid parts mapped onto it (boot and trouser follow the lower leg): `joint-list.md` beside this plan. The research
file and the rules take that list word for word; this plan does not restate it.

**2. `root`.** Settled by the same ruling (a skin joint, or the parent node the game scales), with what it means in
glTF and in Blender. The joint-list mark (mark 3 below) follows it.

**3. Sockets have axes.** Each socket's own axes are part of the rule, so that a direction survives export (the SMG's
eject direction, "right and 20 degrees up"): the ruling states them in glTF's and in Blender's axes.

**4. `marking_` parts.** The engine's rule is only this: a node whose name starts with `marking_` is an optional part
the game can show or hide, and it uses one of the model's own materials. That the marking is "off at game size and on
in close views" is the Art lead's proposal (the addition, A2); the call is Mustafa's. How a marking's shape and
colour change with the faction stays open.

**5. His sentence on sockets is quoted whole,** wherever a doc of this change-set states the rule: "The poses. Where
the addition's hand points and a gun's sockets disagree, the sockets win: the right hand's point and the gun's
direction set the pose; the left hand, the butt and the muzzle follow from the gun model's sockets, as the SMG brief
already does." (2026-11-17).

**6. The marks, rewritten where they did not work.** Every mark has a planted defect of its own.
- Mark 6 (scale and axes): an asymmetric test box, 1.0 m wide, 2.0 m high and 0.5 m deep, with a small named marker
  on its front face. After export and load it measures 2.0 m along +Y, 1.0 m along X and 0.5 m along Z, each within
  1 mm, and the marker's centre has the greatest Z. Planted defects: "+Y Up" off; a scale of 0.01 left unapplied.
- Mark 7 (sockets): all five named nodes exist; `socket_grip` lies within 1 mm of the origin; `socket_muzzle` has a
  positive Z and lies within 1 mm of the gun's centre plane (X = 0); each socket's forward axis points as the ruling
  says, within 0.5 degrees. Planted defects: one socket renamed; the muzzle socket turned by 5 degrees.
- Mark 9 (both backends): on WebGPU and with `forceWebGL` the loaded model has the same node names, the same count
  of meshes and materials, the same material on each mesh and the same bounding box within 1 mm. Planted defect: a
  loader option that differs between the two runs.
- Mark 4's defect is its own: a vertex with five influences (mark 3 keeps the exporter's neutral bone).

**7. What the rifle file can prove.** `AMK9_v4_stage1.glb` was "written by a script", so it never went through
Blender's exporter. It can pass or fail marks 1, 2, 5, 7 and 8. Marks 3, 4 and 6 need a real Blender export: the
test box and a small skinned figure of Part 11.

**8. Reviewers and models.** graphics-engineer writes the research file (Opus 5.5, effort max, passed by this chat).
Reviewers, each at effort max: engine-architect (the names, the joint list it ruled on is checked by graphics-engineer
in turn), test-engineer (the new marks: it owns ADR 0005's pass marks), the verifier (claims against sources; his
words byte for byte), docs-keeper's pass; then the advisor agent on the final head.

**9. DECISIONS.md: the pointers that were missing.** Dated pointers of 2026-11-17, each naming the new entry that
changes the line: SPIKE-ORDER (b), "wave 1 runs one heavy spike at a time" (changed for these steps by his decision
of 2026-11-14); his words of 2026-11-12 on Fable 5.2 for every chat that writes game or engine code (changed for the
second builder chat only by his CHANGE 2); his words of 2026-11-12 "The guard, the gate and CI work stays with Opus
5.5." (set aside for the gate and CI edits this chat asks of him, by his answer GATE-FOLLOW (b)); the queued model
item in "Queued" (the same two pointers). LOOK-TEST-INPUTS gets no pointer: his addition keeps it.

**10. SPIKE_PLAN.** The earlier pointers stand in three places and each still says "Steps 2.4 to 2.6 still wait for
their results": the new pointer is added at all three. ADR 0002 has one place: one pointer.

**11. The register (`docs/adr/README.md`), with its count line.** OQ-0006-18: answered by MODEL-PUBLIC (a), his words
quoted. OQ-0002-6: answered, the formula's value, a reach of 3.07 m. The rows that ask whether soldiers block each
other: answered, "they push each other softly and never block". OQ-0003-1: answered further in part (the desktop is
an RTX 2070 with an i7-9700K; the laptop's class; the actual machines before change-set 6). The draft lines these
answers touch: PM-0004-34's row in ADR 0004 gets a dated pointer here (his word for the change to that mark). The
others are carried later, and the report names the carrier of each: ADR 0002's "one change-set per step" against
SPLIT-25 with change-set 2's docs pass; the machine lines in ADR 0001, ADR 0003 and SPIKE_PLAN with change-set 6's,
when the actual machines are named.

**12. Size.** The build plan's row for change-set 0 says about 150 lines of docs. This plan's work is larger because
his CHANGE 1 added the record of his decisions: about 500 lines written, besides what is carried (the plan, the
appendix). Stop lines: the research file at 300 lines, the written total at 1,000.

**13. ADR 0005's new marks** go into the pass-mark table with the asset spike's group, in the table's own columns,
and the group's count line is updated; nothing is added to "What this section decides".

**14. LOOK_V2.** The §8 line cites its source (his words of 2026-11-03, SPIKE_PLAN's research entry). When this
change-set merges, `docs/LOOK_V2.md` belongs to the second lane's carry (one owner per file; Part 15). No doc of this
change-set cites line 30's "no plain tubes or cylinders" as decided.

**15. STATUS and the report.** STATUS also says: the 2.3 chat is stopped and its docs change-set no longer records the
decision of 2026-11-14; the second builder chat exists; #55 is merged. The report's "since the plan" also says: the
Art lead's brief replaces Part 4's sizes; both line widths are switchable at change-set 6; change-set 8 grows by
about 400 lines (the simple active reload); the plan's text is copied by script from commit 68addf9.

**16. Two more rules for the research file:** how the hand meets `socket_grip` (from the ruling), and a check of the
soldier's origin and facing (mark 10, placed with mark 6's export).

**17. The Art lead's correction of 2026-11-15, 18:40** (attached by Mustafa on 2026-11-17 with his message of 20:58;
saved as `inputs/ART_BRIEF_CORRECTION_2026-11-15_1840_gun_poses.md`; the appendix carries it as A10 and his message
as A9). His words: "For change-set 0 take only its rule and the socket names. The pose numbers are for the body step
(change-set 10)." So the research file takes two things from it, both as the Art lead's proposal that agrees with his
go: the rule ("The right hand is on socket_grip and the left hand on socket_support. The butt and the muzzle follow
from the gun's file.") and the five socket names. No pose number enters a rule of this change-set. The report's "since
the plan" passes three notes to change-set 10, in his words: the rifle's butt in the aiming pose ("its top edge is at
U 1.39. The file wins, by the correction's own rule."); "in the rifle's aiming pose the left arm uses 0.59 of its
0.62, so little is left for recoil and the walk"; "The SMG's side magazine has to stay clear of the left forearm".

**18. The Advisor chat's log** (his words of 20:20 and 20:58: not an input; not read; no part of it in the repo).
Nothing in this plan rests on it: the LOOK_V2 §8 line comes from the build plan's Part 11 and his words of
2026-11-03, and "the Art lead's proposal unless a line says it is mine" from his go.

**19. The ruling on the joints is in** (`joint-list.md`, engine-architect, effort max, 98 lines; it answers findings
1, 2, 3 and 16).
- One engine joint list for Slice 1's soldier, 20 joints: the build plan's 16 (Part 11, rule 21), names unchanged,
  plus four coat joints (`coat_front_l`, `coat_front_r`, `coat_back_l`, `coat_back_r`, on `hips`). They are the 20
  things legacy already moves (its 14 joints, the 2 ankles it adds at load, its 4 coat panels), so change-set 10
  ports the animation by name. The spine stays (legacy splits the aim twist between spine and chest).
- `root` is not a skin joint: it is the one node above skeleton and mesh that the game places, turns and scales with
  one number (his words: "hang the soldier on one root so it scales with one number"); `hips` is the top joint.
  graphics-engineer's draft read `root` as the armature's top bone; the ruling disagrees and says why. The research
  file takes the ruling; graphics-engineer may answer it with a sourced point; how Blender writes the armature object
  is settled by the first real export. **The advisor agent is asked to rule on this point in its re-read.**
- Sockets: a socket's local +Z is where it points and +Y its up (in Blender: local -Y and +Z); grip, support, muzzle
  and sight carry no rotation; each gun's entry lists its own sockets, with grip, support and muzzle required. The
  Blender half of this rule is inferred for rotations, so a real export of a turned socket must prove it.
- Hands: the right arm reaches `socket_grip`, the left `socket_support`; the fist's point lies `palm` metres beyond
  the wrist, one number in the soldier's entry.
- The soldier's origin and facing: five lines a file passes or fails (mark 10), with four planted files.
- The brief's 17 parts are mapped onto the joints (the stand-in's weights). The tolerances the ruling marks
  "[proposal]" become the marks' numbers through test-engineer's review.
- Left open, for the report and for change-set 10's plan: whether the coat keeps legacy's motion (Mustafa, at change-set
  10's pictures; if not, the four coat joints go before the Blender model is exported); where the joints the brief
  does not place sit, and painted weights (the Art lead, judged by Mustafa); the reload's targets have no socket.

**20. Two research files, not one.** `docs/research/asset-export-rules.md` (graphics-engineer: the export rules, the
names, the marks) and `docs/research/soldier-joint-list.md` (engine-architect: the ruling, as it stands, with its
citations repointed from the notes to the plan's report and its appendix, and from the draft's lines to the rules'
ids). The second lane's change-sets 6 and 10 build from the second file. Both get rows in `docs/research/README.md`
and in the repository map.

**21. The size has passed twice the build plan's figure, so Mustafa is told and the docs build waits for his word**
(his rule: "when one passes twice its plan while it is built, stop and tell me in three lines"). Planned: about 150
lines of docs. Now: about 900 written lines (the rules file about 380, the joint ruling about 100, the plan's report
with "since the plan" about 280, entry headers, pointers, register and draft rows about 120), besides about 1,700
copied lines (his three messages, the plan, the appendix). Why: his CHANGE 1 put the record of his decisions and the
plan's carry into this change-set; the rules need their sources and marks; finding 1 needed the ruling. The cheaper
option: change-set 0 keeps his records, the plan's carry and a short list of the rules (names and export settings),
and the sources, the marks and the ruling follow in a change-set 0b beside change-set 1. Review of what exists goes
on meanwhile (the re-read of this plan, the draft's update to the ruling); no file outside `.checkpoints/` is edited.

**22. Part 15 is not carried by this change-set.** It is sent to him as a file after the advisor agent's review and
goes to main once he has answered it. If `.checkpoints/part-15*` is deleted with this change-set's last commit, the
report and STATUS name the commit that holds it.

**23. His answers of 2026-11-17, 21:33** (`mustafa-2026-11-17-2133.md`; the appendix's A11; they close point 21).
- SIZE-0: "(a). It goes on as one pull request." "The new stop line: 1,200 written lines; copied lines do not count."
- "Build my records and the plan's carry in commits of their own, apart from the rules and the joint list, so a
  split stays cheap if it is ever needed." So the build's commits are: (1) his records: DECISIONS (five entries, six
  pointers), SPIKE_PLAN, the dated lines in ADR 0002, 0004 and 0006, the register's rows for his answers, the
  appendix; (2) the plan's carry: the plan's report; (3) the rules and the joint list: the two research files, their
  index rows, ADR 0005, the register's ADR 0005 row and row G1, LOOK_V2 section 8's line; (4) docs-keeper's pass;
  (5) this change-set's report and the deletion of the checkpoints.
- The split rule, his words: "If review round 1 returns a Critical or High on the rules or the joint list that one
  fix pass cannot close, split at that point: my records and the plan's carry go first as change-set 0, the rules and
  the joint list follow as change-set 0b. You need no new word from me for that; tell me in three lines when it
  happens."
- LOG-HISTORY: "(a). Leave the history; nothing is rebuilt." (The advisor agent's finding 17 asked the same question
  and recommended a fresh branch; his answer decides it. The report says plainly that a merge commit brings the
  branch's commits, that one included, into main's history.)
- A10 is the Art lead's text alone (3,244 bytes); the notes' copy was replaced, and the ruling's citations of the
  correction are repointed to its lines when the ruling moves to `docs/research/`.
- DECISIONS gets five entries, not three: his messages of 20:58 and 21:33 are recorded too, as the foundation steps
  recorded each message of his in their sessions; his 20:20 message, which the 20:58 one re-sends, is in the appendix
  (A8).

**24. The advisor agent's re-read: APPROVE with conditions** (`advisor-plan-r2.md`, 2026-11-17, about 20:45 UTC).
Its conditions, each done before the build's first commit:
- 18: two more pointers in DECISIONS (the entries of 2026-11-10 and 2026-11-13); the script now places five and
  expects six with SLICE-ORDER's. 19: every pointer that says the marks stay also carries his one exception (COMPARE)
  and his sentence "Those tests are not built as separate tests unless the plan shows a question the real game cannot
  answer." Both in `pointers-insert.mjs`, dry-run clean.
- 20: mark 8 counts what PM-0005-36 counts, draw calls per pass; the shared `unit_dark` name keeps four materials,
  not four draws; the ways out go to him with change-set M's plan (the research file's O1; the report's section 5).
- 21: a dated "Answered by Mustafa (2026-11-17)" line under OQ-0006-18 in ADR 0006 and under OQ-0002-6 in ADR 0002,
  as OQ-0006-19 has one.
- 22: the stop was told in three lines and is answered (point 23).
- The Lows: point 17 reads "no pose number enters a rule"; the 20-joint table lives only in the joint-list file; no
  "under `root`"; the research file's limit is 400 lines; the report's new section corrected; the go's entry header
  names what GATE-FOLLOW and MACHINES change; no pointer says more than his words; the SLICE-ORDER pointer quotes
  his sentence; the verifier spot-checks both research files; LOOK_V2's hand-over waits for his answer on Part 15; a
  fix at change-set M that loosens a mark needs his word; the ruling's knee rule is marked as holding for rigid
  weights only.
- ROOT: the advisor agent rules for engine-architect: `root` is the node above skeleton and mesh. What the first
  real export must show, and what changes if it shows otherwise, is written into mark 10.

**The files, in one table (final).**

| File | Change | Commit |
|---|---|---|
| `docs/DECISIONS.md` | five entries by script; six dated pointers | 1 |
| `docs/SPIKE_PLAN.md` | three dated pointers | 1 |
| `docs/adr/0002-code-structure-build-toolchain.md` | one dated pointer (migration step 1); a dated line under OQ-0002-6 | 1 |
| `docs/adr/0004-simulation-core.md` | a dated pointer in PM-0004-34's row | 1 |
| `docs/adr/0006-infrastructure-operations.md` | a dated line under OQ-0006-18 | 1 |
| `docs/adr/README.md` | the rows of OQ-0006-18, OQ-0002-6, OQ-0003-1 and the three "do soldiers block" rows; the counts | 1 |
| `docs/reports/2026-11-17-build-plan-slice-1-appendix.md` | new, by script (A1 to A11) | 1 |
| `docs/reports/2026-11-17-build-plan-slice-1.md` | new, by script: "since the plan" and the plan from commit 68addf9 | 2 |
| `docs/research/asset-export-rules.md` | new (graphics-engineer) | 3 |
| `docs/research/soldier-joint-list.md` | new (engine-architect) | 3 |
| `docs/research/README.md` | two rows | 3 |
| `docs/adr/0005-content-abilities-assets.md` | OQ-0005-AS3 and AS4 answered in part (the entries and the table's rows); the new marks from PM-0005-47 in the asset spike's group; the count line | 3 |
| `docs/adr/README.md` | ADR 0005's row; row G1 | 3 |
| `docs/LOOK_V2.md` | section 8, flow step 2: one line | 3 |
| `docs/STATUS.md`, `docs/ROADMAP.md`, `docs/REPO_MAP.md`, `docs/reports/README.md` | docs-keeper's pass | 4 |
| `docs/reports/2026-11-17-cs0-model-rules.md` | new: this change-set's report | 5 |
| `.checkpoints/build-plan-slice-1*`, `.checkpoints/cs0-model-rules*`, `.checkpoints/part-15*` | deleted | 5 |

Risk tier 1 (decision docs): his go; no Brave check. Reviewers on the final diff, each at effort max: engine-architect,
test-engineer, the verifier (both research files; his words byte for byte), docs-keeper's pass; then the advisor
agent on the final head.

