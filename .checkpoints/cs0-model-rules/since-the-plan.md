# The build plan for foundation steps 2.4 and 2.5 and Slice 1 (revision 2), with what changed since

**What this file is.** The plan this repository builds the game from, until Slice 1 ends: fourteen change-sets, from
the generated types to the first Blender model. It was written on 2026-11-14 in the build chat
(session_019YNMUV9LFhASseJ9EP12kL, Mustafa's personal account) and reviewed twice by the advisor agent that day
(REVISE on revision 1, its notes committed at 15:09 UTC; APPROVE on revision 2, its notes committed at 15:26 UTC; the
plan is commit 68addf9, 15:26:47 UTC). Mustafa gave his go on it on 2026-11-17: "I give my go on the plan as written
(revision 2, commit 68addf9): build change-set 0 now, then the change-sets in the plan's order, each as its own pull
request that stops for my go on the merge. This go covers no merge."

**How to read it.** The plan stands below, from "The plan as approved" on, copied by script from commit 68addf9 and
not edited. The section "Since the plan" comes first: it says what Mustafa's answers and changes do to it. Where the
two differ, "Since the plan" is the newer word. His messages are in `docs/DECISIONS.md` (five entries of 2026-11-14
and 2026-11-17) and, byte for byte with the Art lead's texts, in the appendix
(`2026-11-17-build-plan-slice-1-appendix.md`, A1 to A11).

## Since the plan (2026-11-17)

### 1. His answers to the plan's twelve decisions (appendix A3)

| The question | His answer | What it does to the plan |
|---|---|---|
| SPLIT-25: is step 2.5 built as two change-sets? | "(a). Step 2.5 is built as two change-sets." | Change-sets 2 and 3 stand. ADR 0002 still says one change-set per step: change-set 2's docs pass carries that line. |
| FIRST-SOLDIER: which soldier and which gun first? | "(a). The Conscript of the sheets; the SMG and the rifle both in change-set 8; I judge the SMG's feel first." | As planned. |
| GATE-FOLLOW: who builds the gate and CI checks that follow steps 2.4 and 2.5? | "(b), with (i). You ask me for each gate and CI edit with its exact text; for those edits I set my words of 2026-11-12 ("The guard, the gate and CI work stays with Opus 5.5.") aside. The gate's network step waits for the netcode proof test's report." | The plan recommended (a). Now the build chat asks him for each gate and CI edit, after change-sets 1, 2 and 3 (Part 7). The network step stays out until the netcode proof test reports. |
| MACHINES: when are the laptop and the desktop named? | The classes are named now: "The recommended desktop is an RTX 2070 with an i7-9700K"; "The minimum laptop is of the class the rendering draft already names: Intel Iris Xe graphics or the AMD equivalent." The actual machines: "I name before change-set 6, your option (a)". | Timed readings before change-set 6 are rough and get their short re-run. New: "the look is built for the desktop on High, and Low is what the laptop gets"; "If Low cannot hold it on that laptop without hurting the look, bring me the numbers and I decide whether the floor rises. Do not lower the look on High to reach it." The machine lines in ADR 0001, ADR 0003 and SPIKE_PLAN are carried by change-set 6's docs pass, when the machines are named. |
| BRAVE-ROUTE: how do builds reach his laptop before preview links exist? | "A, if not possible: B" | The zip through the chat's file tool (Part 8). He could save the plan file, "so the route works for a file; change-set 1's test archive proves it for a zip." If it fails: a build made by CI, which needs a CI job first (asked of him with its exact text). |
| PREVIEW-START: does the preview hosting check start now? | "(a). It has started, in the second builder chat" | Not this chat's work (section 2 below). |
| STAND-IN-LOOK: are proportions and weapon shapes judged on the code-built soldier? | "(a). I judge proportions and weapon shapes on the Blender model." | As planned: at change-set M. |
| COMPARE: do all comparisons run? | "(b). The comparison against a library's character controller runs only if our own movement misses its marks. This is my word for the change to PM-0004-34's wording. It is the one exception to "No pass mark and no open question of theirs is dropped or loosened" in my decision of 2026-11-14; the other comparisons run." | The plan recommended (a). One day less after change-set 4 unless our movement misses its marks. PM-0004-34's row in ADR 0004 gets a dated pointer in this change-set. |
| MODEL-PUBLIC: may a committed model be public? | "(a), and my words: "A, once the game is up, I will look to make the repo private later on." The sheet notes you pushed may stay." | Models may be committed. By the plan's recommendation, which is not part of his answer, only the exported GLB is. OQ-0006-18 is answered in the register and in the infrastructure draft. |
| RULES-WHO: who answers the model-file rules, and when? | "(a), and my words: "A, but might eventually go with C with time depending on how it will go"" | This change-set (change-set 0). |
| SCOPE-RELOAD: is the active reload part of Slice 1? | "Yes active reload will be a part of slice 1, but a more simplified version, hitting it will make the reload end faster. The additional depth will be added later." | The plan recommended the plain reload. A simple active reload joins change-set 8: about 400 lines more (4,400 becomes about 4,800). Its judging may be redone after the netcode proof test (time stamps inside a tick). |
| START-RULES: what do the jump and two soldiers meeting start from? | "(b), I decide both now. The jump: the formula's value, a reach of 3.07 m (OQ-0002-6, its option (a)). Two soldiers meeting: they push each other softly and never block. Both are numbers and rules I can change after I have felt them in Brave." | Both are recorded in the register as his, changeable after Brave. |

### 2. His four changes (appendix A3)

1. **Change-set 0 records his decisions.** "Into docs/DECISIONS.md, by script from this chat's transcript: my block of
   2026-11-14 with its decision, my addition of 15:03 that day, and this message." The plan had left that to step
   2.3's docs change-set. "No game code is built before change-set 0 is on main." The 2.3 chat stays stopped, so its
   lint and format settings are not on main before change-set 2: Part 7's case for that applies (the checks run by
   hand, by path).
2. **A second builder chat.** His decision is in `docs/DECISIONS.md` word for word (it begins "Two chats build the game
   on my personal account."). "Every file has one owner: the first builder chat's plan says which change-sets and
   folders belong to the second chat, and neither chat edits the other's files." That is **Part 15, "The second lane"**:
   it was sent to him as a file on 2026-11-17 (revision 2, approved by the advisor agent, with six questions for him).
   It is not in this report: it goes to main once he has answered it. His proposal for it: the second chat takes the
   preview hosting check and step 2.6, the carry of his look decisions into `docs/LOOK_V2.md`, the comparisons that stay
   (Part 3.2), and from change-set 6 on the picture track (6 Look, 10 Body, 11 Feel, and M); this chat keeps 0 to 5, 7
   Lag, 8 Shoot, 9 Throw and 12.
3. **The soldier and the guns follow the Art lead's briefs** (appendix A4 to A6, and the correction, A10).
   "Everything in them is the Art lead's proposal unless a line says it is mine."
   - "The soldier brief replaces the sturdier sizes you typed into your notes on 2026-11-14 at 15:18 UTC." So Part 4's
     stand-in takes its sizes from the brief (A4), not from the sheet notes (A7).
   - "Its size, 1.80 m to the top of the hat, is a placeholder: hang the soldier on one root so it scales with one
     number."
   - "No skin colour on this first stand-in: hands and face take the Dark material, so four materials hold, the rifle
     included." and "the soldier stays a skinned mesh, as your Part 11 says."
   - "The brief's names for colour areas and bones are working names; the names of your change-set 0 win." The names
     are in `docs/research/asset-export-rules.md` and `docs/research/soldier-joint-list.md`.
   - "Line widths: docs/LOOK_V2.md stands until I decide otherwise. The Art sheet's widths, units thicker than
     buildings, are a proposal: make both switchable at change-set 6 so I judge them in Brave." New work in
     change-set 6: a switch between the two sets of widths.
   - "The poses. Where the addition's hand points and a gun's sockets disagree, the sockets win: the right hand's
     point and the gun's direction set the pose; the left hand, the butt and the muzzle follow from the gun model's
     sockets, as the SMG brief already does."
   - "The muzzle is where the flash and the tracer start. In the simulation a shot's start is data of its own".
   - A first model file exists, a rifle prototype written by a script (`AMK9_v4_stage1.glb`): "It stays with me
     until you ask for it."
   - "The words "no plain tubes or cylinders" in docs/LOOK_V2.md line 30 and in the rendering draft are not mine."
     and "Treat them as not decided; the carry of my look decisions removes them." No doc of this change-set cites
     them as decided, and this change-set does not edit that line.
4. **Shamble VFX exists** (his effects editor, in its own repository). "In Part 15, say how change-set 11 (Feel) uses
   it. My lean: the game loads Shamble VFX's exported effects through its runtime and builds no effect a second
   time." Part 15 answers it; change-set 11's size may change there.

### 3. His three later messages of 2026-11-17 (appendix A8, A9 and A11)

- The Advisor chat's log reached this chat by mistake. His words: "It is the Advisor chat's working record, not an
  input for you." Nothing in the plan or in this change-set rests on it (this change-set's report says what happened).
- A fourth Art text, the Art lead's correction of 2026-11-15, 18:40 (A10): "For change-set 0 take only its rule and
  the socket names. The pose numbers are for the body step (change-set 10)."
- Notes he passes to change-set 10 (Body) with it: in the rifle's aiming pose the butt's "top edge is at U 1.39. The
  file wins, by the correction's own rule."; "in the rifle's aiming pose the left arm uses 0.59 of its 0.62, so
  little is left for recoil and the walk."; "The SMG's side magazine has to stay clear of the left forearm, as the
  correction says itself."
- "Stamp times from the clock, never ahead of it."
- His answers of 21:33 (A11) to two questions of this change-set. SIZE-0, the size stop: "(a). It goes on as one pull
  request." and "The new stop line: 1,200 written lines; copied lines do not count." LOG-HISTORY: "(a). Leave the
  history; nothing is rebuilt." And on the correction's two copies: "In the appendix, A10 is the copy of 3,244 bytes:
  the Art lead's text alone."
- His SMG line drawing arrived with that message: one image, not copied into the repository. His words: "It is needed
  at change-set 8, not before. Until it arrives the SMG brief's numbers stand; they win over the drawing in any case."

### 4. What changes in the plan's numbers and files

- **Change-set 0** was planned at about 150 lines of docs, for the model rules alone. With the record of his
  decisions, the plan's carry and the ruling on the joints it writes about 900 lines, besides about 1,700 copied
  ones. It passed twice its plan, he was told in three lines, and he accepted the size (A11): his records and the
  plan's carry are built in commits of their own, apart from the rules and the joint list, "so a split stays cheap if
  it is ever needed".
- **Change-set 8** grows by about 400 lines (the simple active reload). **Change-set 6** gains the switch between
  the two sets of line widths. **Change-set 11** is weighed again in Part 15 (Shamble VFX).
- **The comparisons** (Part 3.2): all run but the character-controller one, which runs only if our own movement
  misses its marks.
- **Who builds what** follows Part 15 once he has answered it. Until then nothing but change-set 0 is built.
- **The plan's notes.** The plan cites working notes under `.checkpoints/build-plan-slice-1/`. They are deleted with
  this change-set, as `.claude/CHECKPOINTS.md` says; what a later reader needs is here or in the appendix, and the
  notes stay readable in the branch's history at the last commit before the deletion (named in this change-set's
  report).

### 5. Not yet in the plan: found while change-set 0 was built

FOUND_SECTION

---

## The plan as approved (revision 2, commit 68addf9, copied by script)

