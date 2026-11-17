# Part 15 of the build plan: the second lane, with Mustafa's answers

**What this file is.** Part 15 of the build plan for foundation steps 2.4 and 2.5 and Slice 1
(`2026-11-17-build-plan-slice-1.md`): which change-sets, folders and files belong to the second builder chat, how the
two chats' pull requests take turns, who owns each CI file, and how change-set 11 uses Shamble VFX. Mustafa asked for
it on 2026-11-17 (his CHANGE 2 and CHANGE 4, in `docs/DECISIONS.md`); it was sent to him as a file at 20:46 UTC
(revision 2, approved by the advisor agent at 20:39 UTC), and he answered its six questions at 22:10 UTC.

**How to read it.** "His answers" comes first. Part 15's text stands below it as he read it, copied by script from
commit ea851a8 and not edited. Where the two differ, his answers are the newer word. Two lines of the text are
overtaken: its title's "it waits for Mustafa's answers" (he has answered), and VFX-ROUTE's option (a), where the text
says the copy "may need changes" (his answer: nobody edits it by hand). His "as Part 15 has it" keeps the four
conditions of 15.1 in full: his one-line summaries of them do not narrow them. His two messages stand at the end,
byte for byte (P1, P2). Part 15 cites a reading of the Shamble VFX repository as "the digest": it stays readable at
commit d2c4caa, `.checkpoints/part-15/shamble-vfx-digest.md`.

## His answers (2026-11-17, 22:10 UTC; P1)

"Lane 1" is the first builder chat, "lane 2" the second, as in the text below.

| The question | His answer | What it adds to the text below |
|---|---|---|
| LANE-SPLIT: is the split of 15.2 to 15.4 the one the two chats work by? | "(a), as Part 15 has it. That is my yes to three things" | The four conditions of 15.1 hold. "one pull request at a time per lane, in place of the plan's one at a time". The owners of 15.4: "lane 1 owns gate.yml, toolchain.yml and the gate's files while the 2.3 chat is stopped; lane 2 owns the preview workflow it adds; the kit stays frozen." |
| RECORD-FILES: who writes the record files both lanes need? | "(a). Each pull request writes only its own lines there, in its docs pass, after taking main in. It does not reword, re-wrap or move another lane's lines." | New against 15.3: no rewording, re-wrapping or moving of another lane's lines. Which files these are is 15.3's list (this chat's reading: his answer names none). |
| VFX-ROUTE: how does the Shamble VFX runtime's code reach this repository? | "(a), a copy in a package here, pinned to a commit of the VFX repository and re-copied by a script. This is also my word on a seventh package beside the six the structure draft names." | Stricter than the text, which said the copy "may need changes" to pass the game's checks: "Nobody edits the copy by hand. If the copy cannot pass the game's checks as it is, that is a finding for the VFX chat." And: "The re-copy script can also check: it fails when the copy differs from the pinned commit." This chat's reading for 15.2, where the text gives lane 2 the copied source: lane 2 re-copies it, nobody edits it. |
| VFX-FIX: who fixes what the proofs find in the runtime? | "(a). The VFX chat fixes its repository and the copy here is renewed. I pass the findings on." | As the text has it. |
| VFX-MOMENTS: who builds camera kick, the shake and the sounds for Slice 1? | "(a). The game builds them for Slice 1, as planned." | New for change-set 11's plan: "They hang on the same event names as the effects and their numbers live in data files, so moving them into Shamble VFX later is a change of data. Hit-stop is in no card of the plan and I add none now." |
| GUN-FILES: may a gun file written by a script enter the game? | "(a), no. Every gun file that enters the game comes through Blender. The script-written rifle is used only to try the checks. Until the files exist, the code-built guns of change-set 6 stand in." | The model-file rules say so now (`docs/research/asset-export-rules.md`, rule AX-R1). |

**Still his, from the text's "For Mustafa":** giving lane 2 this Part 15 and asking it for its list of needs from the
block before change-set 4's plan is reviewed; telling lane 2 which chat on his PC makes the model files; having the
VFX chat export at least one effect folder for the VFX proof, and later the effects for the slice's five weapons; and,
with VFX-FIX (a), passing the digest's findings to the VFX chat ("I pass the findings on.").

---

## Part 15 as he read it (revision 2, commit ea851a8, copied by script)

