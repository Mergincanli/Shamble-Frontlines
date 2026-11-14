# u4d2_merge_text.py: notes (the agent writing the script for U4d2's merge texts)

## Plan
1. Read the model script u4e_merge_text.py whole; the rulings' "## The texts" (r1 :548-649, r2 :313-467), r1 Block 9
   and r2 Block 5 (M8's rules); the target files at the main folder's head.
2. Hash the rulings (sha256sum) and the targets; find each old text / anchor in its target (count once).
3. Write u4d2_merge_text.py in the model's form: phase A = T4 (r1); phase B = T1 (r1), T2, T3, T5 (r2). Each code block
   read by its heading (the first fenced block after "### Tn."), the rulings' and targets' sha256 asserted, each old
   text found once, each new text compared byte for byte in place after, the diff's removed/added lines asserted to be
   exactly the ruled ones, no changed prose line over 120, the dry run printing the unified diff per file.
4. Run `--phase A` and `--phase B` without `--apply`; record exit codes and summaries.
5. Return path, sha256, the two dry runs' results, and anything not placeable as ruled.

## Notes (one line per step, as done)
- Step 1 done: read u4e_merge_text.py whole; r1 :470-649 (Block 9, the texts), r2 :172-467 (Blocks 3-5, the texts).
- Step 2 done: sha256 r1 9978b43b763958f9066b708a05319dfd5b03fa61dbc1b920b0375465298d9b6f, r2
  e776f0ba7c4894ef72f05fb12ebfa1c955a61bd6018b200404f010f5b310f8d4; targets at the head (99c7ed9): SERIES.md
  f118a8de..., u4-build-plan.md 52dd3186..., u4s-carries.md efdd7b6b..., plan.md aac32c3d...; u4d2-report.md absent.
  Anchors found once: SERIES.md:397 (r1's ":397 at the head"), u4-build-plan.md:198 and :202 (§4.1), u4s-carries.md
  entry 77 (§U4r, :361-370) above the named heading (:372), plan.md:1247 (r2's "plan.md:1247 at the head"); no entry
  82-85, no C-U7-2, no `fate_log_v1_datagram_sha256` yet. wip/u4d2.patch touches no target (its 18 paths read).
- Place note: the task's "/home/user/Shamble-Frontlines/marks/SERIES.md" does not exist; the ruling's
  "marks/SERIES.md" is spikes/netcode-proof/marks/SERIES.md (as u4e_merge_text.py uses; :397 matches), so the script
  targets that file.
- Step 3 done: u4d2_merge_text.py written (Write, then Edit for an overlap check and widths); lines all <= 120.
- Step 4 done: `--phase A` exit 0 (SERIES.md -0 +1, T4); `--phase B` exit 0 (u4-build-plan.md -2 +2, u4s-carries.md
  -0 +61, plan.md -1 +1, u4d2-report.md stand-in -1 +73); git status shows only the new script; no target moved.
- Extra check (in memory, nothing written): phase B with a faked report holding the anchor once passes (T5 at the
  anchor's line); anchor twice or absent stops it; `--phase C` and a run outside the repo root stop it.
- Slips, mine: the first two dry runs' output went to scratchpad files by a shell redirect (outside the repo, read by
  nothing else); a `py_compile` syntax check wrote __pycache__/u4d2_merge_text.cpython-313.pyc in the notes folder
  (untracked), which I removed at once with rm; git status after shows only the script.
- Script sha256 (final): af8e955e702835523baf119d9aa3e272ed36d5e8b5a5ff25633b84f915b84e65.
