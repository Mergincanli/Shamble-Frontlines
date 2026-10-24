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

## Advisor verdict on this plan (2026-10-24, general agent reading main's advisor.md): APPROVE
R7 stays inside Shamble Frontlines, the queue item and brief §4 R7, and touches no game code. Amend the plan with the conditions below before the first commit; no second review is needed unless a condition can't be met.

Conditions, verbatim; this plan follows them where they differ from the text above:

1. Tuning dump timing (correctness). The `eval` command in tools/preview.js runs the snippet after bootGame() (preview.js:107). The `--pre` file runs before it (preview.js:95). Boot overwrites some literals: `T.arenaX/arenaZ` (shamble-core.js:225, and per map), `T.resupplyCost` and the other values `applyRules` sets (shamble-game.js:43), and the camera fields `applyCamStyle` sets (shamble-game.js:2202). In deliverable 4, replace "It reads globals only; it changes nothing." with: "It runs twice. First through `--pre`, after the engine scripts load and before bootGame, giving the source defaults that docs/legacy-tuning.json records. Then after boot on the default map and on `--map=tri`, recording only the fields whose live value differs from the default, each with the function and file:line that set it. It reads globals only and changes nothing. Functions and THREE objects are serialized by a stated rule (THREE.Color as hex, functions skipped and listed)."

2. Tool evidence (logic change type). Under Evidence, add: "A Node check (run through `node tools/preview.js . eval`) asserts at least 10 dumped values against their literal at file:line on 48ffb51, for example T.moveSpeed 6.3 (shamble-core.js:67) and TUNE.teams.size 4 (shamble-core.js:102). Two runs give byte-identical JSON. tools/tuning-dump.js imports nothing and writes no file: the session saves its stdout." The gate does not check files in tools/ (gate.mjs:34 FILES), so syntax-check it with `node --check tools/tuning-dump.js`.

3. Marks where LOOP_V2 replaces a legacy rule. Under Marks, add: "Where LOOP_V2 replaces a legacy rule, the mark is redesign, citing the LOOP_V2 section. This covers the eras and income economy, the SILO and TBOX objectives, win conditions, spawning and buying, and progression (§8). Rewrite in Rust is used only for mechanics LOOP_V2 keeps: movement, collision, shots, line of sight, damage, team filtering, bot navigation and vehicle physics. Any 'rewrite in Rust' row for a rule LOOP_V2 marks [OPEN] says 'rule pending LOOP_V2 [OPEN]'." Without this, a 'rewrite in Rust' mark on the legacy economy silently keeps rules that LOOP_V2 replaced.

4. The look (brief §5: never decide it). Replace "anything about the look is marked 'redesign (look brief pending)' and goes no further" with: "Every row whose output a player sees carries the note 'look brief pending', whatever its mark. That covers procedural builders, materials, characters, FX, HUD and war-room UI, and the look pipeline. For those rows a mark is about the code technique only, never the appearance. The look pipeline itself is marked 'redesign (look brief pending)' with no further recommendation."

5. Honest risk 3. Replace it with: "Until kit-v3-0 merges, R7 commits stay in a local worktree and are not pushed. This breaks brief 2b rule 1 (commit and push each time) for that period. kit-v3-0 is not small: it is a rules PR that waits on Mustafa's own go. Usage limits are covered, because the checkpoint, the agent notes and the local commits are all on disk. Losing or archiving the container is not covered: the scratchpad is in the same container, so it is no backup. To limit this: the read workflow writes notes one row at a time. If kit-v3-0 is still unmerged when workflow 1 ends, the session raises decision R7-PUSH (allow a second push branch, or wait). R7 work is never stacked on the kit-v3-0 branch."

6. STATUS and ROADMAP. Add a line under Deliverables: "STATUS and ROADMAP are not edited in this PR, because open PR #25 edits both files (verified: its only files are docs/ROADMAP.md and docs/STATUS.md). The docs pass after the merge records R7, as kit-v3-0 does." The report states this under its deviations.

7. Checkpoint lifecycle (brief 2b rules 1 and 5). Add to Method: "Every step updates `.checkpoints/r7-legacy-inventory.md` (goal, steps with SHAs, current and next step, findings, key numbers, decisions). It is committed locally after every step, and pushed after every step once the branch is free. In the last commit before the PR is marked ready, delete `.checkpoints/r7-legacy-inventory*` and fold its essentials into the report."

8. Kit rows and routines. Kit rows describe main at 48ffb51 and add the note 'changed by kit-v3-0' where that PR changes the row. The 'routines' rows list only SHAMBLE FRONTLINES routines: PR Advisor (trig_01YPyWAsQmsaQoydhMGG3CkJ, model claude-opus-5-5, enabled) and the three disabled night-shift routines (Night shift, Night Shift 2, Night Shift 3 (Morning)). No Unit Forge routine, repository or runbook is read, cited or listed anywhere in R7 (Mustafa today: 'Unit forge is a totally different tool which is nowhere done yet').

9. Night queue. When the PR opens, the session edits issue #16: item 1 points to the PR and becomes a plain bullet (the convention the night shift skips). This prevents a duplicate run once the new Night shift routine exists. Verified that no SHAMBLE night-shift routine is enabled today, so tonight has no clash.

10. Reviewers and model policy. engine-architect, netcode-architect and graphics-engineer review the final diff only, as agents read from main after kit-v3-0 merges. If they are not on main by then, they run as general agents that read their files from kit-v3-0's approved head. Every R7 agent (readers, the tuning mapper, skeptics, the coverage critic, the verifier) runs on Opus at effort high, stated explicitly (brief 2a; Opus 5.5 defaults to medium).

11. Merging kit-v3-0 (ruling asked for). Mustafa's advance message ("Once kit-v3-0 PR is up, you can merge it yourself") delegates who presses merge. It is not the go. CLAUDE.md on main and advisor.md step 6 both require his go to be given after the advisor's APPROVE of the PR's current head. kit-v3-0 is a rules change (escalation category 3). So the session merges kit-v3-0 only when all of these hold: (a) the advisor has approved the final head, naming its full SHA; (b) the gate and smoke jobs are green on that SHA; (c) Mustafa writes 'merge #<n>' in this session's chat after (a); (d) the merge passes that SHA as the expected head. A later push restarts this from (a). The rules on main apply, not the PR's new CLAUDE.md. For Mustafa: one line, 'Reconfirm the merge of #<n> (rules change): say merge #<n>'.

Escalation, verbatim:

R7-PUSH (raise only if kit-v3-0 is still unmerged when the read workflow ends). The question: may the R7 work be pushed before kit-v3-0 merges? Why it matters: until it is pushed, the work exists only in this container, and a lost container loses it. Options: (a) Mustafa allows a second push branch for this session, for example `claude/bold-faraday-i49gug-r7`, so R7 is pushed after every step; (b) R7 waits, local only, until kit-v3-0 merges. Recommendation: (a), if the session's settings allow it; otherwise (b), with the pause written in the checkpoint.
