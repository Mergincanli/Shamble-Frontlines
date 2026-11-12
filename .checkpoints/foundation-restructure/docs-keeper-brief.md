# Brief: docs-keeper, foundation 2.1b step 10 (the docs pass)

You are docs-keeper, at xhigh effort (this pass edits an ADR and the register). Task slug: `foundation-restructure`.
Your notes: `.checkpoints/foundation-restructure/docs-keeper-2-1b.md` (your plan first, then one line per edit, saved as
you go). Edit only files under `docs/`, with the Edit tool. Never commit, never push. The session commits.

## What landed (PR #51, branch `claude/clever-fermi-oudsfq`, standing in for `cs/foundation-restructure`; base main 76b4f7a)
- c090843, the move: `git mv` of the five legacy files (`shamble-core.js`, `shamble-game.js`, `shamble-editor.js`,
  `index.html`, `editor.html`) into `legacy/`, bytes unchanged, nothing else in the commit.
- 1e6f84e, the commands in text: `tools/README.md` (every folder argument `.` → `legacy`, a paragraph on the build-folder
  rule), the tool header comments, `tools/village.py`'s doc line, the root `README.md`.
- b6b3b81, the four kit lines on Mustafa's chat yes: CLAUDE.md :19, :43, :44 and `.claude/agents/verifier.md:28`.
- 2a37121, by the session's scripts (byte-checked): `docs/DECISIONS.md` (his six messages of 2026-11-11 as entries; his
  render-budget note under "Queued"), ADR 0002's Status line (his SCOPE (a) quoted whole), and the 2.1b report's
  Appendix A (`docs/reports/2026-11-11-foundation-2-1b.md`; its body comes at step 11).
- Evidence, all PASS (the report gives the numbers): E1 the bytes are unchanged (five R100 renames; blobs equal at
  76b4f7a and 44446a5); E2 OLD's untouched gate passes on the moved bytes; E3 31 of 31 mutations fail the same checks in
  OLD's gate (files at the root) and the head's gate (files in `legacy/`), and the head's gate passes against five
  bases; E4 the legacy tuning oracle: the dumps are byte-identical and R7's builder reproduces `docs/legacy-tuning.json`
  and `docs/LEGACY_TUNING.md` byte for byte; E5 saves under `file://` are shared across folders in headless Chromium
  (Brave on Windows not verified: his backup step stays); E6 every changed command runs; E8 a revert of the merged PR
  keeps the gate and the smoke green. CI on c090843, 1e6f84e and b6b3b81: gate and smoke green (runs #674 to #679).

## His words that govern this pass (read them; quote nothing of his except by pointer)
- D1 (a) and REG-TRUE (b): `docs/DECISIONS.md`, the entries "Taken by Mustafa, 2026-11-11, 20:44:29 UTC: ..." and
  "Taken by Mustafa, 2026-11-11, 21:39:44 UTC: ...". REG-TRUE (b) replaces D1 (a)'s "answered" wording and new count;
  the rest of D1 (a) stands: no new register row, nothing wider.
- SCOPE (a): `docs/DECISIONS.md`, the line that starts "> SCOPE: (a). I accept". It is already quoted whole in ADR
  0002's Status line, by script: do not touch line 3 of ADR 0002.

## The edits (find every target by its text; the line numbers are main 76b4f7a's)
1. `docs/REPO_MAP.md`: :14-18 (the five root rows of the legacy files) → one `legacy/` row: the legacy build (the three
   engine files and the two pages), frozen; it also says that `file:line` citations keep their numbers after the move
   (`shamble-game.js:N` means `legacy/shamble-game.js:N`: `git mv` kept the bytes). :6's date → 2026-11-11 if it carries
   one. `node .claude/scripts/repo-map-check.mjs` must exit 0.
2. `docs/START_HERE.md`: :12 (the double-click rule) names `legacy/index.html`; :22-23 (the five code files sit at the
   repo root) → in `legacy/`; the commands at :105 and :115-117: the folder argument `.` → `legacy`, the same strings
   `tools/README.md` now uses (for example `node tools/battery.js legacy`, `node tools/preview.js legacy eval
   tools/sim.js`). One banner line marking START_HERE as a legacy doc (row KT21: the legacy docs stay in `docs/`, marked
   legacy) only if it has none yet.
3. `docs/MODULE_MAP.md`: one line above the file table (:17): all five files live in `legacy/`.
4. `docs/TECHNICAL.md`: §9's file inventory (:238-239): `legacy/`.
5. `docs/STATUS.md`: 2.1b in flight (PR #51, a draft until its report; risk tier 2, needs Mustafa's go); 2.1a (PR #50)
   merged as main 76b4f7a; the risk at :106 ("the inventory's line numbers go stale once phase 1 moves the legacy
   files") corrected: `git mv` kept bytes and line numbers, so the citations hold with the prefix `legacy/`. A 2.3
   follow-up: his finding of 2026-11-11 from his Windows run (the pinned Playwright's installer, before 1.60.0, hangs
   after the download reaches 100% under Node 24.16.0 and later; it finished under Node 24.15.0): a pointer to his words
   (`docs/DECISIONS.md`, the entry of 2026-11-11, 20:44:29 UTC), no quote.
6. `docs/STATUS_ARCHIVE.md`: #50's row, if the archive keeps merged rows; the same risk at :843 corrected the same way.
7. `docs/ROADMAP.md`: 2.1b's entry, terse: the move, the commands, the four kit lines, the docs; PR #51.
8. ADR 0002 (`docs/adr/0002-code-structure-build-toolchain.md`), never its line 3:
   a. :7: after "not yet accepted" add exactly ` except ADR 0002's part 1, accepted by Mustafa on 2026-11-10 (SCOPE
      (a))`, so the line reads `**Signed off by Mustafa:** as the basis for the spikes (2026-11-03, SPIKE-ORDER (b)); not
      yet accepted except ADR 0002's part 1, accepted by Mustafa on 2026-11-10 (SCOPE (a))` (one line, as now).
   b. The OQ-0002-2 entry (from "- **OQ-0002-2." to just before "- **OQ-0002-3."): its existing lines unchanged; add its
      answer as new sub-bullet(s) at its end. It must say: answered in part, 2026-11-11, by its decider, the phase 1
      restructure plan (foundation step 2.1, plan revision 3; the main session with docs-keeper), with the reasons inline
      (the plan file is deleted before the PR is ready: point to `docs/reports/2026-11-11-foundation-2-1b.md`, never to
      `.checkpoints/`):
      (1) the legacy gate and tools stay in `tools/`, option (a), learning the `legacy/` path (step 2.1a, PR #50): 2 gate
      files and 2 kit files edited, against option (b)'s cost (the guard's gate-file lists and its npm check, 17
      `tools/` allow rules in `.claude/settings.json`, CI's `--prefix tools` and both gate steps, and kit lines in about
      8 more files); R5-LAYOUT-REC6's marks hold (E2, E3 and E4 in the report);
      (2) the legacy docs stay in `docs/`, marked legacy (row KT21): no path changes, against 27 lines in 12 files (8 of
      them protected), 2 broken relative links and about 25 dangling references for `legacy/docs/`; START_HERE gets
      rewritten for v2 later (KT21's mark "redesign"), not archived;
      (3) the gate v2's script path is left to step 2.3's plan, which edits the guard anyway; so the question stays open,
      marked "answered in part", by the register's own rule. That status is Mustafa's REG-TRUE (b) of 2026-11-11
      (`docs/DECISIONS.md`); the answer itself is the plan's.
   c. The two rows `| R5-LAYOUT-OQ5 | open: OQ-0002-2 | part 1 |` and `| R5-GATE-OQ3 | open: OQ-0002-2 | part 9 |`
      (find them by text: the answer's new lines move them): `open: OQ-0002-2` → `answered in part: OQ-0002-2`.
9. The register (`docs/adr/README.md`):
   a. The ADR 0002 row (:17), its Status column: after "not yet accepted" add ` except ADR 0002's part 1, accepted by
      Mustafa on 2026-11-10 (SCOPE (a); his words: ADR 0002's Status line)`. Its open-question column stays exactly as
      it is ("8 open" and the rest).
   b. The Total paragraph (:23 on): unchanged.
   c. The status sentence (:33-34, "Status of every draft: Proposed. Signed off by Mustafa: as the basis for the spikes
      (2026-11-03, SPIKE-ORDER (b)); not yet accepted. His words: ..."): after "not yet accepted" add ` except ADR 0002's
      part 1, accepted by Mustafa on 2026-11-10 (SCOPE (a))`, before the period. Rewrap only those lines.
   d. One added row in a table under "## Loose-ends register" whose columns fit (for example "The critic's remaining
      items and the answered escalations": Code | State | Resolved in / listed by): PM-0002-36's reading. The mark's
      command (`node tools/preview.js legacy eval tools/tuning-dump.js` reproduces `docs/legacy-tuning.json`) is read
      as: the dump (`--pre=tools/tuning-dump.js`, both maps) at the base with the folder `.` and at the head with the
      folder `legacy`, byte-identical over two runs each; then R7's builder (`build-tuning.py` at a715be2) on the head's
      dumps reproduces `docs/legacy-tuning.json` and `docs/LEGACY_TUNING.md` (`cmp`). The reading is the advisor
      agent's (Q-A1, 2026-11-10), recorded here on Mustafa's D1 (a) of 2026-11-11, never his decision; the PM row's own
      wording is fixed at ADR 0002's next revision. Shown by 2.1b's evidence E4 (the report).

## Never
- `docs/DECISIONS.md` (done by script), ADR 0002's line 3, the 2.1b report (the session writes it), any PM row of any
  ADR, ADRs 0001 and 0003-0006, SPIKE_PLAN, the research files, LEGACY_INVENTORY, EDITOR_V1_INVENTORY,
  `docs/legacy-tuning.json`, `docs/LEGACY_TUNING.md`, every other report, ROADMAP's history, `spikes/`, anything
  outside `docs/`.
- His render-budget note in any other file (STATUS, ROADMAP, ADR 0003): his words, "change no other file for it".
- PR #40's lines: its branch adds lines to DECISIONS.md, REPO_MAP.md, STATUS.md and STATUS_ARCHIVE.md; do not add
  them or touch them.
- A pointer to a `.checkpoints/` path in any doc; a paraphrase of his words inside quote marks.
- Anything wider than the list above. First grep ADR 0002 for every `OQ-0002-2`, and `docs/` for other lines that now
  misstate ADR 0002's status or where the legacy files live: list them in your notes and your answer as "stale, left for
  the report", unedited.

## How
- The Edit tool for docs. In the shell, one plain command with a literal path; never `git -C`, `git -c` or `--git-dir`
  (the guard refuses them; run git from the repo root). You cannot be asked: a refused command is a note, not a retry.
- When done, run `node .claude/scripts/repo-map-check.mjs` and `node .gate/foundation-restructure/docs-check.mjs` (the
  session's check script) and fix what they flag within this list.
- Your answer: the files you changed, a line each; the stale lines outside the list; the last lines of both checks;
  anything you could not do, and why.
