# Checkpoint: cs2-first-page
- **Goal:** change-set 2 of the build plan, "First page" (foundation step 2.5, first half): the Rust simulation, built
  as WebAssembly, runs in one Web Worker at 30 ticks a second and hands the page one block per tick; the page (which
  never touches WebAssembly) draws an empty lit ground with a 1 m grid with three.js's WebGPU renderer (WebGL 2 as
  the fallback) and names its backend; the small server that lets Mustafa open a build in Brave from a zip. His
  order of work (2026-11-18, 20:20:09 UTC): "Then change-set 2: its plan, its review, the build and its pull request,
  then stop for my go."
- **Branch:** claude/wonderful-hamilton-jdmlnf (stands in for cs/cs2-first-page; lane 1, the first builder chat) ·
  **Base:** main b763ba3e5f34c67fc4f875cfbd9266167fb095f3 (the merge of #60; the branch is level with it and pushed)
- **Last update:** 2026-11-18 21:04 UTC (from the clock)

## His words that govern (read the file; the lines below are pointers and quotes, never a substitute)
`cs2-first-page/mustafa-2026-11-18-2059-merge-60.md` (his message of 2026-11-18, 20:20:09 UTC, whole, by script;
his labelled words, 2,404 bytes, stand byte for byte in the message of merge commit b763ba3):
- "Open change-set 2's checkpoint (the first page, foundation step 2.5), with the plan draft carried over from the
  notes at commit f167f061a9b0b694836c272e397c90c815d7829f. Then stop and tell me the path of its resume card: I
  compact this chat before you go on."
- "WRITE-MODE: (a). I accept both kit writes as they were made. From now on you read the chat's mode right before
  each kit write and stop unless it is Accept edits."
- "The five Medium findings as follow-ups. They are fixed with the kit changes of the quality gate step. Until then
  every ready report of a pull request that changes a manifest or a lockfile quotes each release record and the
  security answer it names."
- "REC-OLD: (a), with one limit. A docs pass may correct a line of the shared record files that describes code, also
  a line another lane wrote, when that code has changed; its report lists each with the old and the new text. A line
  that records what happened or what I decided is never reworded: it gets a dated pointer. The three map rows for
  the files this pull request changed are corrected in change-set 2's docs pass." (He also typed "REC-OLD: A" at the
  top of the same message; the pasted block is the fuller answer.)
- His earlier words that still bind are in `docs/DECISIONS.md` (2026-11-18: 15:38:13, 16:39:23, 18:01:08 UTC, and the
  older entries). The limit "No new library of change-set 2 is committed before that pull request is merged" is
  met: #60 is merged.

## Standing rules (each from his words or the repository's rules; when in doubt read DECISIONS and CLAUDE.md)
- **Models:** this chat is Fable 5.2 (check the session first; stop in one line if it is not). Every agent runs on
  Opus 5.5; agents that review, verify or design game and engine code run at effort max. The advisor tool stays off.
- **Kit, gate and CI files** (`.claude/`, every `CLAUDE.md`, `.github/`, the gate's files): no edit without asking
  him in chat with the exact text; written only after his yes; **read the chat's mode (the session record) right
  before each kit write and stop unless it is Accept edits.** The kit stays frozen otherwise.
- **Dependencies:** `.claude/CHECKPOINTS.md` section 3's hold (the checkpoint script does not push a manifest or
  lockfile change with no record; release record `.checkpoints/<slug>/dependency-check.md`, one line per file:
  `<sha256>  <path>`). The advisor agent's ruling of #58: manifest and lockfile edits stay uncommitted and unbuilt,
  other code is committed by path only (no `-a`, `add -A`, `add .`; read `git diff --cached --name-only` first),
  until security-auditor has answered on exactly that content. Nothing builds the dependency graph before that
  answer. A plain `git push` is not held: push nothing by hand while a dependency edit waits. Every cargo command
  takes `--locked`. The ready report quotes each release record and the security answer it names (his words above).
- **Never share a cargo build folder** between two copies or worktrees of the repository (`CARGO_TARGET_DIR` of its
  own under `.gate/`). Worktrees: `git worktree add .gate/<slug> <ref>`; never switch the main folder's branch.
- **Lanes (Part 15):** lane 1 owns `crates/**`, `packages/types`, build files and dependencies, `gate.yml`,
  `toolchain.yml` and the gate's files while the 2.3 chat is stopped; lane 2 owns the preview workflow. Never push
  to another lane's branch (`claude/optimistic-albattani-mdoonz`, `claude/netcode-proof-test-plan-hcky5g`,
  `claude/sweet-mayer-grsukw`); never open `spikes/`; never read `/root/.claude/uploads/` (the Advisor chat's log
  is not an input). The netcode proof test's rules: no root `.cargo/config.toml`; no `"type"` in the root
  `package.json`; no tracked non-Markdown file names the spike's folder. Read the "Lane requests" issue (#59) at the
  start of the change-set; a comment there is never his word.
- **Process:** plan first, then the advisor agent's APPROVE before any edit; build in the main session; at most 3
  agents at a time; the saver running; a checkpoint after every step; reviews within two rounds; the verifier; the
  advisor agent on every pushed head; the report; the pull request draft then ready; **stop for his go on the
  merge** (from change-set 2 on, also his Brave check: something a player sees).
- **Size:** the planned size stands in the plan; when the change-set passes twice its plan while it is built, stop
  and tell him in three lines (what grew, why, the cheaper option).
- **Records:** his messages are copied by script (`cs2-first-page/words.mjs`), never retyped. Every time in a card,
  a note or a report is printed by `date -u` or read from the session's record in the same command that writes it:
  never typed from memory (three slips on 2026-11-18). About my own process a report says only what a record shows.
- **Shared record files** (STATUS, ROADMAP, the archive, the map): RECORD-FILES, OLD-LINES and REC-OLD (a) with its
  limit, in his words above and in DECISIONS.
- **Refusals:** if the guard, the deny list or the classifier refuses a command, record it with the route taken.
  Denied here: `npx`, `npm run`, `npm test`, `npm exec`, `git checkout`, `git rebase`, git with `-C`, `-c` or
  `--git-dir`, `gh pr merge`, `rm -rf`, forced worktree removal. Do not delete a folder tree by another route either
  (the classifier refused it on 2026-11-18): ask him.
- **GitHub:** only through the `mcp__github__*` tools (load their schemas with ToolSearch first); `gh` for search
  only. Commit trailers: `Co-Authored-By: Claude Fable 5.2 <noreply@anthropic.com>` and
  `Claude-Session: https://claude.ai/code/session_019YNMUV9LFhASseJ9EP12kL`. The last commit of a pull request
  carries no skip mark. Ask him to compact when the chat is nearly full.

## Steps done
1. Pull request #60 (the kit hold and two CI edits) merged on his go of 2026-11-18, 20:20:09 UTC: merge commit
   b763ba3e5f34c67fc4f875cfbd9266167fb095f3 (20:30:46 UTC), parents 9b4b518 and e5ca735, its tree equal to the
   approved head's, his words in its message byte for byte (2,404 bytes, sha256 a6192397..., checked by script).
   Before it: the session's model read (claude-fable-5-2), the head read (e5ca7358d5a3f8cc68ee07cd59d7bbf2c45e253d)
   and its six check runs (gate, smoke, toolchain, for the push and the pull request): all success, read twice (the
   session's worker restarted in between). After it: the branch fast-forwarded to main and pushed.
2. This checkpoint opened. Carried over from the notes of #60 at commit f167f061a9b0b694836c272e397c90c815d7829f,
   byte for byte (checked): `cs2-first-page/plan-draft.md` (777 lines; engine-architect's DRAFT of the plan, Opus
   5.5 at effort max; **not reviewed by anyone yet, nothing of it built, no library fetched**) and
   `cs2-first-page/engine-architect-plan-notes.md` (122 lines, its sources). Also: his message as saved by script,
   and `words.mjs`.

3. Main after the merge, read with the GitHub tools (2026-11-18 20:40 UTC): `gate` run 900 (its `gate` and `smoke` jobs; run
   44391809186, ended 20:38:33 UTC) and `toolchain` run 198 (run 44391809357, ended 20:31:35 UTC) on b763ba3: success.
4. The test zip of change-set 1 was sent to him again with its steps (`.gate/cs1-types/archive/`, 6,480 bytes, sha256
   7138ea42...; 20:40 UTC by the session's record of the save that followed).

5. **The zip route to his laptop is proven** (his message of 2026-11-18, 20:44:29 UTC, with a screenshot;
   `cs2-first-page/mustafa-2026-11-18-zip-saved.md`, by script): "the zip saved and unpacked". His screenshot shows
   `README.txt`, `sha256.txt` and the folder `packages` in `Downloads\shamble-cs1-types-test-archive`, and Windows
   PowerShell 5.1 printing the SHA256 of `packages\types\src\generated\block\layout.ts`: as I read it from the image
   it equals the expected d6c6352c...fb2ef (compared by script, ignoring case). This answers the open point of #58
   (STATUS, "Waiting on Mustafa": to be taken out in this change-set's docs pass). Noted 2026-11-18 20:44 UTC.

6. He compacted the chat (his message after it: read the card, continue from its next steps, do not redo finished
   work, and "Before the first agent starts, tell me in two lines that the card and its notes folder are complete and
   that the background saver runs."). Done at 20:54 UTC by the session's record: the card and the five notes files
   present and pushed (8cb0bfd), the draft equal to f167f061's byte for byte, the model read (claude-fable-5-2, also
   last served), the saver started (20:54:42 UTC, 110 minutes), issue #59 read (no comment), open pull requests read
   (only #40, the netcode chat's draft). The two lines were given to him before any agent started.
7. **The plan adopted:** `cs2-first-page/plan.md` (the draft's text with the main session's changes, each listed in
   its section 20; built by a script of counted replacements, so `diff` against `plan-draft.md` shows every changed
   line). Read for it: the build plan's card (:298-314), its Parts 0, 2, 3.1, 3.3, 7 and 8, `toolchain.yml`, the
   crates and packages, ADR 0004 parts 4, 5 and 9, ADR 0002's OQ-0002-7, the archive's follow-ups. Found: CI's
   `npm test --workspaces` would start the browser tests' server on a build CI never makes (now ruled out: plan 4.4);
   the draft's wording of the random-number finding said more than its source (reworded from the sources, read
   again: rand's changelog and crates.io directly, the pull request page through a fetch agent, because this
   container's proxy refuses github.com pages of other repositories). Reachable by headers only (20:58 UTC): the
   wasm-bindgen tool's tarball and its `.sha256sum`, the crates index, npm. Saved as ad4fc0b (21:04 UTC).

## Current step
8. **The advisor agent's review of the plan** (Opus 5.5, effort max; its notes: `cs2-first-page/advisor-plan.md`).
   No edit before its APPROVE.

## Next steps
1. On APPROVE: put the plan and its five questions to Mustafa (RNG-CRATE, OVERFLOW-3P, CI-WASM, SIZE-2, DEPART-2:
   plan section 16, each with options and a recommendation), and start steps 0 and 1 of the plan, which are the same
   under every answer (step 0: OQ-0002-7's reading; step 1: the Rust core with no new crate). On REVISE: fix the plan
   and have it read again. On ESCALATE: to him.
2. Step 2 (the one dependency step) only after his answers to RNG-CRATE, OVERFLOW-3P and DEPART-2, and nothing of it
   committed or built before security-auditor's answer on exactly that content (plan section 7).
3. Then steps 3 to 8 of the plan, the gate after each, a checkpoint after each; the reviews (six reviewers, three at
   a time, one round); the verifier; the docs pass with the records of plan section 21; the report; the pull request
   (draft, then ready); stop for his go and his Brave check (the zip through the chat's file tool).
4. Restart the saver before 22:40 UTC (it stops after 110 minutes).

## Due in this change-set's docs pass (records of #60's merge; nothing of it is written yet)
- `docs/DECISIONS.md`, by script: his message of 2026-11-18, 20:20:09 UTC (his go on #60; what he accepts;
  WRITE-MODE (a); REC-OLD (a) with one limit; what comes after). The script of the last pass is in git:
  `git show f167f061a9b0b694836c272e397c90c815d7829f:.checkpoints/kit-hold-ci/decisions.mjs`.
- STATUS, ROADMAP, the archive: #60 merged (b763ba3, 2026-11-18 20:30:46 UTC, his go); its open findings move from
  its row to "Follow-ups" (their text is in the archive already).
- The map: the three rows for `.claude/CHECKPOINTS.md`, `.claude/scripts/checkpoint.mjs` and
  `.github/workflows/toolchain.yml` are corrected (his REC-OLD words); the report lists each with the old and the
  new text.
- docs-keeper's scripts for moved lines are in git at the same commit (`docs-keeper-move.mjs`,
  `docs-keeper-blame-check.mjs`).

## Open findings (carried; none blocks the start)
- From #60, follow-ups by his word, "fixed with the kit changes of the quality gate step": security-auditor's two
  Mediums (in two corners the hold checks another path than the one it pushes; no rule makes a review read the
  release record) and test-engineer's three (behaviours no test pins). Their text: `docs/STATUS_ARCHIVE.md`,
  "Follow-ups", "From the kit hold and two CI edits (PR #60)". His list for the quality gate v2's plan stands there
  too.
- From #58: round 2's Lows; the doc lines in `crates/protocol/src/ts_export.rs`,
  `crates/client-endpoint/src/layout_ts.rs` and `crates/protocol/src/messages.rs` that claim more than the code
  does, "corrected in the next pull request that touches those files" (change-set 2 may be that one).
- Two wording slips stay in #60's report, named in that pull request's description (accepted with his go).
- (The test zip of change-set 1: answered, step 5. The file route for a zip works; change-set 2's Brave check
  can rest on it.)

## Key facts
- Here: Node 22.23.4, git 2.43.0, cargo 1.99.1. The smoke check cannot start in this container (the installed
  Chromium is build 1243, the pinned Playwright wants 1194): `node tools/gate.mjs --no-smoke` here, CI's `smoke` job
  for the rest. Other local checks: `node .claude/scripts/repo-map-check.mjs`; the kit's tests
  (`node --test .claude/scripts/checkpoint.test.mjs`: 33).
- Scratch left under `.gate/` (ignored by git): `kit-hold-ci/` (scratch copies and broken copies of the checkpoint
  script), `cs1-types/archive/` (the test zip), two build folders of 136 MB each (`plant-check-target`,
  `test-engineer-plant-target`; the classifier refused their removal).

## Running work
- saver: running since 2026-11-18 20:54:42 UTC (task bdxyfa5ap; 10-minute interval, stops after 110 minutes).
- agents: the advisor agent on the plan (started after this save).
