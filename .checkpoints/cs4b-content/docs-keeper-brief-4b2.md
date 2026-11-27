# Brief for docs-keeper: change-set 4b-2's docs pass (after the reviews and the verifier)

Written by Builder V6 (the main session), 2026-11-27, after review round 2 and its fix pass. The verifier reads the
same tree at the same time: it edits nothing, and you edit only `docs/`; neither of you runs the plants script.

**Where.** The git worktree `/home/user/Shamble-Frontlines/.gate/cs4b-2`, local branch `cs/4b-content` (pushed as
`claude/ecstatic-wozniak-5cbdzs`). The code's final head: `git log -1 --format=%H -- crates packages content
Cargo.toml Cargo.lock` names it (914f3b4fc5298968b775c81f0fb15ca270a88040). The change-set: `git diff b8f02b0149cc928964e54e194cd9c4d68f52bd94 HEAD --
. ":(exclude).checkpoints"`. What it is: the card `.checkpoints/cs4b-content.md`, from "## 4b-2 (the second pull
request)" to "## Slips", and the report's draft `.checkpoints/cs4b-content/report-draft-4b2.md`.

**Rules (as at 4b-1):** you edit only under `docs/`, and there never `docs/DECISIONS.md` (his messages of the
change-set are in it already, copied by script: `.checkpoints/cs4b-content/decisions-4b2.mjs check` passes), never
an ADR's own text, never `docs/reports/` (the main session writes the report). Notes and scripts under
`.checkpoints/cs4b-content/` (yours: `docs-keeper-4b2.md`, your plan first, then each block as you finish it). Git
read-only, each git command alone in its shell call with nothing in front of it; never git add, commit, push, switch,
checkout, restore, stash, merge or reset. A text of his moved between files goes by script, byte for byte. The
repository is public. Check each claim you write against the tree (the code is the source of truth).

## 1. ROADMAP and REPO_MAP: make them true for 4b-2
- ROADMAP: 4b-2 built steps 6 to 11 (the collision library's batch, the built-file check, the tools and the data
  from his Export, content to the simulation, the static collision world, the schema writer); **step 12, the page's
  scene, with the map's reading field by field, moved to the start of 4c** (his STEP12-GO (a), 2026-11-27, received
  20:03:18 UTC). One terse entry.
- REPO_MAP: every new file or folder of 4b-2 (read them from `git diff --stat --diff-filter=A
  b8f02b0149cc928964e54e194cd9c4d68f52bd94 HEAD -- . ":(exclude).checkpoints"`), each with its one line: among them
  `content/` (`source/`, `movement/`, `maps/`), `crates/sim/src/collision.rs` and `vec.rs`,
  `crates/content/src/schema.rs` (test-only) and `ts_export.rs`, `packages/types/src/generated/content/` and its
  `schema/`, `packages/client/src/app/content.ts` and `content-ties.test.mjs`, `packages/client/scripts/testlines.mjs`,
  `canonical.mjs`, `maths-allowed.mjs`, the content tool `crates/devtools/src/bin/content_bundle.rs`,
  `crates/devtools/src/world_proofs.rs`, `crates/devtools/recordings/first-loop.content.json`, the new specs. Then
  `node .claude/scripts/repo-map-check.mjs`.

## 2. His VISION-RULE addition (his item 9 of 2026-11-27, 13:57:21 UTC, a to e; no code)
It sharpens point 2c of VISION-RULE version 2; its words are in `docs/DECISIONS.md` (his message of 13:57:21 UTC,
item 9). Put a pointer to them beside version 2's own pointers: `docs/adr/README.md` (the paragraph that names
version 2, and the rows R2-INT-OQ1, R4-LOS-OQ1 and R4-LOS-OQ2 where version 2 answers them) and STATUS's VISION-RULE
line, with his two open points in his own words: "the exact eye height, which reaches the game through my Export,
and which points of an enemy's body the line is tested against". Quote nothing else of his; never reword him.

## 3. STATUS: 4b-2's row and its follow-ups
- The in-flight row: 4b-2 is "built (steps 6 to 11) and reviewed (two rounds); the verifier and the advisor agent's
  verdict on the final head are in its report"; the pull request is opened by his Advisor (leave "none yet"; write no
  head id: the report holds it).
- Follow-ups, each an id in your 4B- series in STATUS's 4b-2 line, its text in the archive's block (who found it):
  - heapless 0.9.4 or later when rstar or parry3d next move (security-auditor's dependency read; the card's step 6);
  - the licence texts for the hosted build (security-auditor);
  - the `unit` column ships in the players' file and nothing reads it at run time (the advisor agent's step 11
    ruling, `advisor-step11.md`, a Low);
  - the first field with a default goes to the advisor agent first, with `read.rs`'s "nothing missing" rule
    (`advisor-step11.md`, Q1);
  - 4c carries the re-read's condition 4: the first collision query from the match's step is 4c's, so a recording
    that covers it is 4c's;
  - the generated content types have no reader in 4b-2 but their tests (engine-architect, EA-3);
  - the local match owns the authority's world; when 4c's step queries it inside the room, it moves to the room,
    and no rule yet keeps `content` out of the room's and the client endpoint's manifests (engine-architect, EA-7);
  - 4c's editor panel imports the schema limit function from product code, never a second copy (engine-architect,
    EA-8);
  - the Chromium run uses 3.6 to 3.9 of its 5 minutes in CI: a split or a new limit before 4c adds a long spec, a
    CI file and so the CI Builder's (infra-engineer, IE-1);
  - `content.json` keeps a fixed name beside hashed files: when hosting is built, serve `index.html` and
    `content.json` without caching, or hash the bundle's name (infra-engineer, IE-2; a hosting change-set's);
  - the per-class minimum counts of the committed rays: 4c, with the casts' first caller (test-engineer, TE-4);
  - `static-world.spec.ts`'s wall-clock bound stays as a backstop; a follow-up only if it ever flakes
    (test-engineer, TE-6);
  - from round 2: `packages/client/serve.test.mjs` still skips its two link tests under CI where a link cannot be
    made (security-auditor; outside 4b-2's diff, from change-set 3a; about 1 line); RR-1's three new paths run
    natively only: one committed ray of each kind (beyond the reach, a limit cut to it, a ramp that overflows) with
    TE-4's per-class counts in 4c (netcode-architect); the types-to-schemas test compares keys, corner lengths and
    words, not a member's kind (number, string or which word list): 4c's reader of the types (engine-architect);
    `boot.ts`'s two calls of the page's tested handling are read and type-checked, not tested (engine-architect);
  - [after: anything else the verifier or the advisor agent leaves open.]
- STATUS's length: at most 120 lines (his STATUS-DIET); lane 1's own lines only; text moved to the archive by
  script, byte for byte. Other lanes' lines are not yours to reword: if STATUS is still over 120, say which and how
  many lines each would free.

## Your final answer (at most 20 lines)
What changed per file with line numbers; STATUS's length before and after; what you could not place; what you did
not check.
