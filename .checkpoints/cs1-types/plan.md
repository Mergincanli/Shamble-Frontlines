# Plan: change-set 1, Types (foundation step 2.4)

Owner: the main session (the first builder chat, lane 1). Branch `claude/wonderful-hamilton-jdmlnf` (stands in for
`cs/cs1-types`), base main 22c6afa. Risk tier 1 (game code and tests): reviews by CI, the advisor agent and the
type's reviewers; it merges on Mustafa's go (`crates/` and `packages/` are outside the PR Advisor's allow-list). No
Brave check: nothing a player sees.

**The plan's body is `plan-draft.md` beside this file** (169 lines, by engine-architect at effort max, every fact
with its file and line). The main session read it whole and adopts it as written, sections 1 to 9, with the decisions
and additions below. Where the two differ, this file holds.

## The main session's decisions on the draft's open points (for the advisor agent to confirm or change)
- O1: the provisional widths and names stay as drafted; each carries "provisional" in its doc line, and the report
  lists them for netcode-architect at change-set 3's plan.
- O2: no count field in the header before the first entity section (change-set 4).
- O3: `checkLayoutHash` lives in `packages/types` (the client's folders are laid out in change-set 2; Part 15, 15.1).
- O4: R5-TSLINT-OQ3 is answered here for ts-rs and our generator only; the report says so.
- O5: security-auditor joins the reviewers (a new dependency: ts-rs and what it brings).

## Additions to the draft
1. **Who builds:** the main session, by hand, in the main folder (no builder agent). Order: `protocol` types and
   their export test; `client-endpoint`'s table, hash, generator and export test; the generated files; `packages/types`
   (check, tests, index); section 6's checks; the planted defects in a throwaway worktree; the 20 runs.
2. **The gate and CI edits that follow** (his answer GATE-FOLLOW (b): "You ask me for each gate and CI edit with its
   exact text"): none is made in this change-set. Its report drafts the first one, with its exact text, for him to
   answer: the type check and the generated-types check that today's `toolchain` job does not run (the draft's
   section 5, last bullet).
3. **The file route for a zip** (his words of 2026-11-17: "change-set 1's test archive proves it for a zip"): the
   report comes with a small test archive, sent through this chat's file tool and never committed; he tries to save
   and unpack it, and says whether it worked.
4. **The docs pass** (docs-keeper, under his answers RECORD-FILES and OLD-LINES): 
   - `docs/DECISIONS.md`, by script: his go on the merge of #57 with his answer OLD-LINES (2026-11-18, 11:08:25 UTC;
     saved in `mustafa-2026-11-18-1131-merge-57.md`; equal to merge commit 22c6afa's message);
   - STATUS, ROADMAP, the archive and the repository map for the merge of #57 and for this change-set. Under OLD-LINES
     (a) it may move lines about merged pull requests by script, byte for byte, and the report lists each line moved;
     every other line stays with the lane that wrote it;
   - the advisor agent's three Lows on #57's final head: STATUS's "Last merged" line gets its pointer to ROADMAP's
     list back; STATUS's "Next" says "the netcode proof test's result", his words, where it says "#40"; the Part 15
     change's report keeps its size line (he accepted "the size as counted: 220") and this change-set's report notes
     the recount of 223;
   - the pass marks: no ADR row changes. The report says per mark "met" or "shown", as the draft's section 2 has it.
5. **The report** (`docs/reports/<date>-cs1-types.md`, at most about 60 lines): the checks' output in short, the
   planted defects with command, exit code and message, the 20 runs' hashes, the provisional list, the size, what
   was not verified (Firefox, WebKit, Safari; any timed mark; the gate's own planted-defect runs).
6. **Size:** planned 600 lines (the build plan's row); the draft counts about 505 hand-written. Stop line 1,200: past
   it, stop and tell him in three lines. Generated files and the lockfile are not counted (the build plan's rule).
7. **Rules kept:** no root `.cargo/config.toml`; no `"type"` in the root `package.json`; no tracked non-Markdown
   file names the spike's folder; `spikes/` is never touched or read; no kit, gate or CI file is edited; no settings
   file of the 2.3 chat's (tsconfig, rustfmt, clippy, ESLint, Prettier) is written.

## What Mustafa must check
Nothing in Brave. After the report: whether the test archive saved and unpacked on his laptop; the drafted CI edit
(yes or no to its exact text); his go on the merge.

## After the advisor agent's review (2026-11-18, 11:58 UTC): APPROVE on conditions, each taken as written
Its notes: `advisor-plan.md` (verdict, rulings, findings 1 to 5). O1 to O5 confirmed. What changes in the plan:
1. **The verifier** (its finding 1, Medium): runs on the final head at effort max with section 4's symbol list; its
   PASS comes before the report.
2. **netcode-architect** (finding 2, Medium) joins round 1 (two batches of at most 3 agents are one round); it gets
   the provisional list now; its brief says never to read `spikes/`.
3. **The dependency before its first build** (finding 3, Medium): after the two manifest lines, `cargo fetch` only
   (it writes the lockfile and downloads; no build script or proc macro runs). security-auditor then checks every new
   lockfile entry (crates.io source and checksum, licence, advisories, the build-script and proc-macro code). Until its
   answer nothing that compiles ts-rs runs; a finding at Medium or above stops the build and goes to the advisor agent.
4. **Planted defect (2b)** (finding 4, Medium): one offset changed by hand in `generated/block/layout.ts`, hash line
   untouched: `cargo test -p client-endpoint --locked` must fail and name `layout.ts`.
5. **The Lows, folded in:** (a) defect (4) also grows the header's stride, so only `generated_layout_is_current`
   fails; the report names the failing test and message of defects (1) and (4); the defects run again if a later fix
   touches an export test or the generator. (b) PM-0002-17's bites sit in the worktree's `generated/` folder, read by
   the same scratch config; the report says tsc ran with `-p`, not `-b`. (c) Write mode deletes only `*.ts` files
   directly in its own subfolder, after checking the path, never recursively; a test clears its scratch folder first.
   (d) The throwaway worktree is removed without `--force` or `rm -rf`: files restored, planted files deleted one by
   one, then `git worktree remove`. (e) The spike-folder search uses `--diff-filter=d`. (f) The report names the
   departure from ADR 0002 (A2:350-352: no `#[ts(export)]`), with a pointer in the ADR as a follow-up, and says
   R5-LAYOUT-OQ4's answer holds only while one writer owns each folder. (g) Symbols completed: modules `messages`,
   `ts_export` (test only), `layout`, `layout_ts` (test only); `stub_builds` goes (real tests replace it); `PACKAGE`
   goes; `packages/types/package.json`'s description changes ("Empty until…" is stale). (h) The gate and the cargo and
   Vitest checks run after every step; section 6 adds `node .claude/scripts/repo-map-check.mjs`; refusals are recorded
   with the route taken; the size stop's three lines are his ("what grew, why, and the cheaper option"); the docs pass
   takes main in first. (i) The report says what the test archive holds and how he checks it; the drafted CI edit
   would be its own risk-tier-2 pull request and commits no tsconfig (type settings are the 2.3 chat's).
6. **O1's additions:** the two acknowledgement fields of `Snapshot` are provisional too; the message module's doc line
   names what may move the widths (OQ-0001-6, OQ-0001-7, R2-XU-OQ4, the mode hash, the join token's format).
