applier3-CC: R7 applier for area CC (fixes3)
- verifier-core#3 (CC4 v2_note): accepted. grep -n crest steps-1-6 report -> line 59; line 58 is the Empire coat sentence. Op: report:58 -> report:59.
- graphics-engineer#0 (CC4 v2_note): accepted in substance with my own op. grep SkinnedMesh|InstancedMesh|BatchedMesh|THREE.LOD over the engine files: 0 hits; mergeStaticGroup core:360-387 bakes transforms; buildEntity castShadow core:686. But the finding's "56 parts + 49 hulls" does not match the code now: headless eval (preview.js eval, SOLDIERS groups) -> styled bots 122-140 meshes = 66-77 parts + 56-64 hulls, player 154, distinct materials = parts. Op uses measured numbers and keeps the report's 105/49 as a dated count.
- graphics-engineer#2 (CC3 v2_note): accepted, fix_op with added code citations. applyFaction core:643-653 and buildEntity mats core:657-669 set only .color (a uniform in r128); report:50 gives 16 vs 9 without a cause.
- apply-ops.py CC applied cleanly (fixes2 + 3 fixes3 ops).
DONE
