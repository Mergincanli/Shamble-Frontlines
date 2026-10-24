# Plan: R7 legacy harvest (legacy-systems-inventory)

Approved item: Night queue #16 item 1, [Advisor verdict 2026-10-17](https://github.com/Mergincanli/Shamble-Frontlines/issues/16#issuecomment-6238312490).
Brief §4 R7 adds one thing: "Extract the tuning numbers into data tables." That addition is what this plan asks the
advisor to approve. Read-only on the game: docs and one read-only tool, symbols +0 −0.
Branch: `claude/bold-faraday-i49gug` (the session's only push branch), restarted from main after kit-v3-0 merges.
Until then the work runs in a local git worktree on a local branch, `r7-legacy-inventory`, cut from main 48ffb51,
with local commits after every step; it is pushed once the branch is free (said plainly in the report).

## Goal
One document that tells the v2 team what the legacy game has, where, how well it is proven, and what to do with each
piece: port to TypeScript, rewrite in Rust, redesign or drop. Plus every tuning number as data, so v2's content
tables start from the real values, not from memory. Every other research topic (R1–R6) reads it.

## Deliverables
1. `docs/LEGACY_INVENTORY.md`, in the format of `docs/EDITOR_V1_INVENTORY.md`:
   - header: purpose; the four marks and what each means for v2; the tally; date and main's SHA; how each state was
     checked ([C] code read at main 48ffb51, [M] measured in a report or by a harness, [D] a doc claim not re-checked);
   - tables by area, one row per system: ID · System · What it does · Where (file:line and main functions) · Evidence
     and numbers (report file and figure) · v2 note · Mark (recommended) with a one-line reason;
   - areas: core (config and arsenal, look pipeline, characters and factions, gun kit, building kit, map schema and
     maps, persistence, draw-call merging, SIM math); game (boot and dispatch, input, player and camera, combat and
     shots, teams and damage, bots and navigation, economy and eras, objectives and structures, vehicles, animation,
     FX and audio, HUD, menus and war-room UI, progression, tutorial, occlusion, dev tools); editor (summary rows that
     point to EDITOR_V1_INVENTORY for the map editor, plus Character Lab and Anim Lab); pages; tools; kit (`.claude/`,
     CI, the gate, routines).
2. `docs/LEGACY_TUNING.md`: the tuning numbers as tables (weapons, unit types, vehicles, `T`, `TUNE`, `TEAMS`, the
   economy and rules objects, AI and navigation constants, animation metrics where they are data), each table naming
   its source symbol and file:line.
3. `docs/legacy-tuning.json`: the same numbers as data, dumped from the live objects (not copied by hand).
4. `tools/tuning-dump.js`: the read-only snippet that produces 3 through `node tools/preview.js . eval`, so anyone can
   re-run it on a later commit. It reads globals only; it changes nothing.
5. The report `docs/reports/2026-10-2x-r7-legacy-inventory.md`.

## Marks (recommended; Mustafa confirms or changes each)
- **rewrite in Rust**: anything the authoritative simulation decides (movement and collision, shots and line of
  sight, damage, teams, economy, objectives, bots and navigation, vehicle physics, match rules). v2's simulation core
  is Rust, so these move to the server and the Wasm client.
- **port to TypeScript**: client-side code worth keeping in shape (input mapping, camera, procedural builders,
  cosmetic animation, synthesized audio, HUD and menu flows), re-expressed for the new client.
- **redesign**: a system v2 already plans to change (the map editor per EDITOR_V2; the loop per LOOP_V2; the look
  pipeline, pending the look brief, never decided here; progression).
- **drop**: dead code, legacy-only workarounds (classic-script globals, the symbol-conservation checks, r128 limits).
A mark is a recommendation with a reason, never a decision; anything about the look is marked "redesign (look brief
pending)" and goes no further.

## Method (on checkpoints, brief 2b)
- Checkpoint `.checkpoints/r7-legacy-inventory.md`; agent notes `.checkpoints/r7-legacy-inventory/<agent>-<area>.md`.
- Scout inline: the work list of areas and their files, from MODULE_MAP, START_HERE §6 and each file's manifest.
- Workflow 1 (read): one reader per area (about 10, each `effort: high`, Opus), each writing rows to its notes file one
  system at a time and returning a short structured summary. Every row cites file:line at 48ffb51 and, for numbers,
  the report file.
- Tuning: `tools/tuning-dump.js` run on main; one agent maps every table to its source line.
- Workflow 2 (verify): per area, a skeptic re-opens every Where and every number and tries to refute it (default:
  refuted when it can't confirm); a coverage critic lists every top-level declaration of the three engine files and
  each page that no row covers. Fix what is confirmed; record what isn't.
- Assemble and tally in the main session; gate; verifier spot-check; report.

## Evidence
- Gate PASS, symbols +0 −0 (no engine file touched).
- Coverage: the share of top-level declarations in each engine file that some row names, with the uncovered list.
- Skeptic results: rows checked, confirmed, corrected, with counts per area.
- The tuning dump re-run gives byte-identical JSON on the same commit.

## Reviewers
advisor (this plan; the PR) · verifier (claims spot-check) · engine-architect, netcode-architect, graphics-engineer
read the marks for their areas once kit-v3-0 has added them (a review of the final diff only).

## Risks
1. The engine files are dense (long lines): readers miss systems. Contained by the coverage critic.
2. Marks drift into design decisions. Contained: marks are recommendations, look items stop at "look brief pending".
3. Pushing waits for kit-v3-0's merge: a lost container before then loses local commits. Contained: kit-v3-0 is
   small and goes first; R7's notes are also kept in the session scratchpad.

## For Mustafa
Nothing to check in Brave. After the PR: confirm or change the marks (as with the editor inventory).
