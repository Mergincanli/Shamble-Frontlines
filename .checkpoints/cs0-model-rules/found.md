Each point names where it is written up and who decides it. None is decided here.

- **The draw cap: a question for Mustafa with change-set M's plan.** The content draft caps a soldier as drawn at
  four draw calls per pass (PM-0005-36). The soldier's four colour areas already make four, and a gun loaded from its
  own file draws on its own, whatever its material is named: with the rifle prototype's two meshes that is six. The
  shared Dark material keeps four materials, not four draws. Three ways out, each with its cost and what it needs
  proven: `docs/research/asset-export-rules.md`, open point O1.
- **The fifth material class has two names** in the drafts: "emissive" in the content draft, "visual effects" in the
  rendering draft (the same file, open point O2).
- **Twenty joints, not sixteen.** Part 11's rule 21 named 16 joints. Four coat joints join them, because the legacy
  coat's quarters follow the thighs with gains, a spring and limits, which skin weights alone cannot give
  (`docs/research/soldier-joint-list.md`, section 1). Whether the coat keeps that motion is Mustafa's call at
  change-set 10's pictures. If it does not, the four joints go; a Blender model exported before then would have to
  be rigged and exported again.
- **`root` is the node above the skeleton and the mesh, not a bone** (engine-architect's ruling; the advisor agent
  upheld it against the first draft of the rules). The first real Blender export must show what mark 10 lists. If it
  shows otherwise, the 20 names stand, and only marks 3 and 10 and the Blender recipe change.
- **Sockets.** Each gun's entry lists its own sockets: the rifle prototype has four, the SMG five. The reload's
  targets (the gun's reload point and the pouch) have no socket among the five: change-set 10 proposes data or a
  socket, and a sixth socket name would be Mustafa's.
- **What only a real Blender export can prove:** how the armature object is written, the neutral bone, the cut to
  four influences, a turned socket read back within 0.5 degrees, the test box's sizes and front, and the soldier's
  origin, facing and scaling. The script-written rifle can test marks 1, 2, 5, 7 and 8 only.
- **For the Art lead**, judged by Mustafa on the Blender model: where the joints sit that the brief does not place
  (hips, spine, chest, head, hip, knee, ankle, the coat's pivots), and painted weights beyond the rigid ones.
- **For change-set 10's plan:** three traps of the animation port (`docs/research/soldier-joint-list.md`, section 7),
  and his three notes on the poses (section 3 above).
- **Shamble VFX and change-set 11:** in Part 15.
