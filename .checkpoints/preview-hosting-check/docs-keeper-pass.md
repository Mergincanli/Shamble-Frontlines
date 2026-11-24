# docs-keeper: the docs pass for the preview hosting check (lane 2), 2026-11-24
Branch claude/optimistic-albattani-mdoonz (main 28254a7 merged in). Nothing committed by me.

## Plan
1. STATUS.md is at 120 of 120 lines. Lane 2 needs about 12. Room only by OLD-LINES (whole lines about merged pull
   requests, moved by script, byte for byte): the four Follow-ups bullets for the gate step (2.3) from merged PRs
   #49 (status-diet), #50 (2.1a), #51 (2.1b), #48 (2.2): STATUS lines 78-93 (16 lines) to the archive's Follow-ups, a
   new block "Moved from STATUS's Follow-ups" at its end; one pointer line of mine in STATUS in their place.
2. STATUS, lane 2's own new lines: In flight row; Next item 3; a Waiting on Mustafa bullet; a Follow-ups bullet (ids);
   an Open risks bullet. Not touched: lane 1's stale lines (3a's row and Waiting bullet, "Last merged", "Last
   updated", ROADMAP's 3a "[in flight]"): flagged to the caller.
3. STATUS_ARCHIVE.md: the follow-ups text block "From the preview hosting check" (ids PHC-*) after 3a's block; the
   risk block after 3a's v2 risk block in "Open bugs and risks". Nothing in the archive's "In flight" (merged only).
4. ROADMAP.md: one entry at the top of Recent work; 12 -> 13, so the oldest (foundation 2.2, merged #48) moves by
   script, byte for byte, to the end of Build history (after foundation 2.1b).
5. REPO_MAP.md: one row for the report after 3a's browser check row; run repo-map-check.
6. SPIKE_PLAN.md entry 6: one dated pointer line appended at the entry's end.

## Edits, as made
### STATUS.md (120 -> 119 lines; script edit-status.py in the session scratchpad, line-number asserts)
- MOVED out (OLD-LINES, byte for byte, checked: the 16 removed lines are one contiguous block among the archive's added
  lines): STATUS lines 78-93 at main 28254a7, the Follow-ups bullets "From status-diet (PR #49)", "From foundation
  2.1a (PR #50)", "From foundation 2.1b (PR #51)", "From foundation 2.2 (PR #48)". Why: STATUS was at 120 of 120.
- ADDED, In flight: row "| PR to open | the preview hosting check ... | his three decisions (ONE-OFF-HEADER,
  STEP4-COUNT, DEPARTURES); the advisor agent's review of its head; his go |" after 3a's row.
- ADDED, Next: item 3 (4 lines): lane 2's step 2.6 plan after his three decisions, unless ONE-OFF-HEADER makes it wait
  for ADR 0006's revision; his question of 2026-11-25 on change-set 6 and M before change-set 5: answered read-only in
  the chat, awaiting his yes; nothing built.
- ADDED, Waiting on Mustafa (5 lines, after 3a's bullet): ONE-OFF-HEADER, STEP4-COUNT, DEPARTURES with plain words; his
  go; the hourly checker off before 2 December; after the merge, archive the test repository, delete the test project
  and its Access application.
- ADDED, Follow-ups (2 lines) in the moved bullets' place: the pointer "For the gate step (2.3), from status-diet (#49),
  foundation 2.1a (#50), 2.1b (#51) and 2.2 (#48): moved by script ... the archive's "Follow-ups", "Moved from STATUS
  (2026-11-24)"."
- ADDED, Follow-ups (2 lines, after 3a's): "From the preview hosting check" with ids PHC-CLICKLESS, PHC-LOCKFILE,
  PHC-TOKEN-SETUP, PHC-PROBES, PHC-DELETE, PHC-ACTIONS-READ, PHC-ERROR-PATH, PHC-TR-END, PHC-SCRIPT-LOWS.
- ADDED, Open risks (1 line, last): "Preview hosting (lane 2): PHC-HEADER, the header mark not met as written (back to
  the draft); PHC-PROBE-MAIL."
- NOT touched (lane 1's, stale since #65 merged at 28254a7): 3a's In flight row and Waiting bullet, "Last merged",
  "Last updated", Next item 2's "After change-set 3a"; the Part 15 Waiting bullet ("give lane 2 Part 15").
### STATUS_ARCHIVE.md (+51 lines, additions only)
- Follow-ups, after 3a's block: "From the preview hosting check" (the nine PHC ids with the report's text, 19 lines);
  then an italic note "Moved from STATUS (2026-11-24)" (4 lines) and the 16 moved lines.
- Open bugs and risks, after 3a's v2 block: "The preview hosting check" with PHC-HEADER and PHC-PROBE-MAIL (12 lines).
### ROADMAP.md (+29 -17)
- ADDED at the top of "Recent work" (11 lines): "**The preview hosting check, ADR 0006's spike 1 [2026-11-15 to 11-24,
  in flight]** — preview-hosting-check ...": 56 runs; deletion through the cleanup job; the wall; no token or
  production for a pull request; the deleted token refused; not passed by the draft's rule (header mark; build count
  and step 4 open); two review rounds, the verifier PASS WITH NOTES; the report; waits for his three decisions, the
  advisor agent's review and his go.
- MOVED (OLD-LINES, by script, byte for byte, checked): the oldest entry, "**The foundation, 2.2: the toolchain
  [2026-11-10 to 11-13, MERGED]**" (16 lines, PR #48), from the end of "Recent work" to the end of "Build history",
  after the 2.1b entry. Recent work stays at 12 entries.
### REPO_MAP.md (+1)
- ADDED after 3a's browser-check row: "| `docs/reports/2026-11-24-preview-hosting-check.md` | The report of the preview
  hosting check (ADR 0006's spike 1, SPIKE_PLAN entry 6; lane 2, ...): the verdict per mark (...), his three
  decisions; appendices: the results log, the workflow texts as run, the advisor tool's consultations |".
- `node .claude/scripts/repo-map-check.mjs` output, verbatim: `repo map OK: 178 entries cover 109 required paths` (exit 0).
### SPIKE_PLAN.md (+4), entry 6
- ADDED at the entry's end, the file's dated-pointer form: "_Pointer (2026-11-24, the preview hosting check's report):
  the runs are done; ... By ADR 0006's rule ("A spike passes when every mark that applies passes", ADR 0006 :1404) the
  spike has not passed yet: the header mark (PM-0006-2) is not met as written (back to the draft), and the build-count
  mark (PM-0006-7) and his verdict on checklist step 4 (inside PM-0006-9) are open._" The quote checked against
  `docs/adr/0006-*.md:1404`. No word of the entry changed.
### Not done, for the caller
- His message of 2026-11-25 is in neither DECISIONS.md nor the report; only in
  `.checkpoints/preview-hosting-check/mustafa-2026-11-25-lane2-build.md`, deleted before ready. STATUS names it
  without a file pointer.
- Lane 1's stale lines (since #65 merged at 28254a7) left for lane 1: STATUS's 3a row, its Waiting bullet, "Last
  merged", "Last updated", Next item 2's "After change-set 3a"; ROADMAP's 3a "[in flight]".
- The report's DOCSKEEPERLINE is the caller's (the report is not mine to edit here).

### Corrections after the advisor tool's read (rendering)
- Archive: the moved-lines note made a top-level bullet "- **Moved from STATUS (2026-11-24)** (...)" (archive :1466-1469).
- SPIKE_PLAN: the pointer indented two spaces, the in-entry pointer form of entry "The foundation's acceptance checks".

### Moves, with ranges (OLD-LINES; source files unchanged between 28254a7 and the branch head)
- STATUS.md lines 78-93 at 28254a7 -> STATUS_ARCHIVE.md lines 1470-1485 (the four bullets below the note).
- ROADMAP.md lines 310-325 at 28254a7 (with the blank line 309) -> ROADMAP.md lines 158-173 (after a blank line 157).
- Method: python scripts (session scratchpad, not kept) that split the files into lines, assert the first and last
  line of each range by its text, cut the range and insert it unchanged; checked after: `git diff -U0`'s removed
  lines equal the saved range byte for byte, and appear as one contiguous block among the added lines.

### Checks
- wc -l docs/STATUS.md: 119. Only docs/ files and this notes file changed; nothing committed.
- PHC- prefix new (grep -rn 'PHC-' docs/: only this pass's lines). No his words keep the moved ids in STATUS (DECISIONS :2240 is the security bullet, untouched).
- Line citations into the lengthened files: only in dated reports and in LEGACY_INVENTORY / the archive citing ROADMAP lines 3-72 (before the insertion points); none corrected.

### The literal added lines
#### docs/STATUS.md
```
+| PR to open | the preview hosting check (ADR 0006's spike 1, SPIKE_PLAN entry 6), by lane 2 (the chat he calls Builder V2), run in his test repository Mergincanli/shamble-preview-check; report `docs/reports/2026-11-24-preview-hosting-check.md`: the runs are done; by the draft's rule not passed yet (the header mark not met as written; the build-count mark and his verdict on checklist step 4 open) | `claude/optimistic-albattani-mdoonz` (stands in for `cs/preview-hosting-check`) | draft; risk tier 1 (the report and his fourteen messages in `docs/DECISIONS.md`); main 28254a7 merged in | his three decisions (ONE-OFF-HEADER, STEP4-COUNT, DEPARTURES); the advisor agent's review of its head; his go |
+3. Lane 2 (Builder V2): after his three decisions on the preview hosting check, step 2.6's plan (the preview links),
+   unless his ONE-OFF-HEADER answer makes it wait for ADR 0006's revision. His question of 2026-11-25, what of
+   change-set 6 (Look) and M (the first model) lane 2 could build before change-set 5: answered read-only in the
+   chat, awaiting his yes; nothing built.
+- Lane 2's preview hosting check (its report's "Decisions needed"): ONE-OFF-HEADER (the header mark is not met: how
+  the draft's header row is revised, and whether step 2.6 waits for it), STEP4-COUNT (checklist step 4 and the build
+  count: no page shows a count), DEPARTURES (may four recorded departures stand); his go on the pull request. Before
+  2 December: switch off the test repository's hourly checker. After the merge: archive the test repository, delete
+  the test project and its Access application.
+- **For the gate step (2.3), from status-diet (#49), foundation 2.1a (#50), 2.1b (#51) and 2.2 (#48):** moved by
+  script, byte for byte, 2026-11-24 (OLD-LINES): the archive's "Follow-ups", "Moved from STATUS (2026-11-24)".
+- **From the preview hosting check** (lane 2; the archive's block): for step 2.6's plan PHC-CLICKLESS, PHC-LOCKFILE,
+  PHC-TOKEN-SETUP, PHC-PROBES, PHC-DELETE, PHC-ACTIONS-READ, PHC-ERROR-PATH; at the merge PHC-TR-END; PHC-SCRIPT-LOWS.
+- **Preview hosting (lane 2):** PHC-HEADER, the header mark not met as written (back to the draft); PHC-PROBE-MAIL.
```
#### docs/ROADMAP.md
```
+**The foundation, 2.2: the toolchain [2026-11-10 to 11-13, MERGED]** — foundation-toolchain, ADR 0002's
+migration plan step 2.2 (Mustafa's decision of 2026-11-10: the second account plans and builds steps 2.1 and 2.2 now).
+The pins: Rust 1.99.1 (`rust-toolchain.toml`); a virtual Cargo workspace and its lock with six empty crates in `crates/`
+(unsafe code denied, overflow checks in every profile for every workspace crate); npm workspaces with exact versions and
+one lock, five empty packages in `packages/` (TypeScript 6.0.3, Vitest 5.1.0, @playwright/test 1.65.0, Node 22). The CI
+job `toolchain` (`.github/workflows/toolchain.yml`; `gate.yml` unchanged) installs the pinned toolchain, checks rustc
+against the pin, then builds and tests with the network cut off (a TCP probe must fail inside); it is green on the
+build's heads, and eight planted failures each went red at the expected step and were reverted. On his yes: the four kit
+lines (the hard constraints bind `legacy/` only; his 2026-11-10 decision is the exception in the three lines that forbid
+v2 code) and the CI job's release-test line; main merged in (c901187); the overflow tests expect their panic's message.
+Review round 1 (five reviewers, 17 findings, each checked by a skeptic): the overflow setting widened to every workspace
+crate on the advisor agent's ruling (RR-1, High; his go asked, then given with the merge, OVERFLOW-ALL). Review round 2
+(four reviewers, nothing Critical or High): on the advisor agent's ruling D1-R2, `"*"` kept and the panic side handed to
+the drafts' owners. The Fable advisor's whole read before ready changed no file. The verifier passed (bd44a68). Merged:
+PR #48, 4d21c1a, 2026-11-13 16:20 UTC, by merge commit, on Mustafa's go after the advisor agent's APPROVE of 9b20b67; CI
+green on main (gate run 43461710700, toolchain run 43461710673).
+
+**The preview hosting check, ADR 0006's spike 1 [2026-11-15 to 11-24, in flight]** — preview-hosting-check
+(SPIKE_PLAN entry 6), by lane 2, run by the chat he calls Builder V2 in his throwaway test repository
+Mergincanli/shamble-preview-check on a test Cloudflare Pages project: 56 runs over the rehearsal and two days of
+sittings. A closed pull request's links were gone within 24 hours, through the cleanup job (without it the host kept
+them); the login wall covered every preview link and left production's own address open; a pull request reached no
+token without his approval and no production; the deleted token was refused. Not passed by the draft's rule:
+production's one-off links carry the host's `X-Robots-Tag: noindex` (the header mark, back to the draft), and the
+build count (no count shown anywhere) and his verdict on checklist step 4 are open; five marks met with departures
+that change no reading. Two review rounds on the build, the verifier PASS WITH NOTES on the evidence; report
+`docs/reports/2026-11-24-preview-hosting-check.md`. Waits for his three decisions (ONE-OFF-HEADER, STEP4-COUNT,
+DEPARTURES), the advisor agent's review of its head and his go.
+
```
#### docs/SPIKE_PLAN.md
```
+  _Pointer (2026-11-24, the preview hosting check's report): the runs are done; the results and the verdict per mark
+  are in `docs/reports/2026-11-24-preview-hosting-check.md`. By ADR 0006's rule ("A spike passes when every mark that
+  applies passes", ADR 0006 :1404) the spike has not passed yet: the header mark (PM-0006-2) is not met as written
+  (back to the draft), and the build-count mark (PM-0006-7) and his verdict on checklist step 4 (inside PM-0006-9) are
+  open._
```
#### docs/REPO_MAP.md
```
+| `docs/reports/2026-11-24-preview-hosting-check.md` | The report of the preview hosting check (ADR 0006's spike 1, SPIKE_PLAN entry 6; lane 2, run in his throwaway test repository on a test Cloudflare Pages project): the verdict per mark (the header mark not met as written, the build count and his checklist step 4 open, so not passed yet), his three decisions; appendices: the results log, the workflow texts as run, the advisor tool's consultations |
```
#### docs/STATUS_ARCHIVE.md
```
+- **From the preview hosting check** (lane 2, the chat he calls Builder V2; in flight, its pull request still to open;
+  branch `claude/optimistic-albattani-mdoonz` standing in for `cs/preview-hosting-check`; risk tier 1; the items of its
+  report's "Follow-ups (for STATUS)", `docs/reports/2026-11-24-preview-hosting-check.md`, their text kept; the ids are
+  this docs pass's):
+  - For step 2.6's plan:
+    - PHC-CLICKLESS: previews without his click per preview, and what keeps the token safe then (his words of
+      2026-11-19, 00:01:25 UTC).
+    - PHC-LOCKFILE: a locked install (a committed lockfile, install scripts off; his point 3 of 2026-11-16).
+    - PHC-TOKEN-SETUP: token setup ends with Cloudflare's verify answer, not a screenshot of the form; the form's
+      single-date chip sets a one-day window.
+    - PHC-PROBES: every probe gets a named User-Agent.
+    - PHC-DELETE: delete with `--force` or the API; a weekly sweep on `main`.
+    - PHC-ACTIONS-READ: `actions: read` reconsidered (SA-R2).
+    - PHC-ERROR-PATH: the deploy step's error path and the account printout (the rename answered the name; the ID is
+      masked when it equals the secret).
+  - PHC-TR-END (after the merge): the test repository: archive it (his TR-END (a)) once this report is merged; then
+    delete the test project and the Access application (Zero Trust, Access, Applications).
+  - PHC-SCRIPT-LOWS: Round 2's Lows (infra L1, L2, L4; test R2-1 to R2-6) and the deferred SA-R5, SA-R6, F5, F6 stay
+    with the throwaway scripts; none carries into step 2.6 unless its plan picks it up.
+- **Moved from STATUS (2026-11-24)** (by lane 2's docs pass for the preview hosting check, under his OLD-LINES: lines
+  about merged pull requests, moved by script, byte for byte): STATUS's Follow-ups lines 78-93 at main 28254a7, the
+  four bullets of the merged pull requests #49, #50, #51 and #48 for the gate step (2.3), are the four bullets below.
+  Their pointers to "the archive's" blocks lead to the blocks above.
+- **From status-diet (PR #49), after the gate step:** the STATUS citations left in advisor.md (:17, :286),
+  netcode-architect.md (:24) and the decision docs (one hop through the table below until then); SD-N1, SD-N2, SD-N3
+  and SD-N6 (security-auditor) and SD-L3 (the verifier): the archive's "Follow-ups", "From status-diet".
+- **From foundation 2.1a (PR #50), for the gate step (2.3):** IE-1, IE-5 (Medium: CI's live mirrors), TE-2, TE-7,
+  R2-1, A-1, N2, L-4 and the v2 root page against pre-move bases; for the drafts' next revision, five "before the
+  foundation" lines left without a pointer; the archive's "Follow-ups", "From foundation 2.1a".
+- **From foundation 2.1b (PR #51), for the gate step (2.3):** the Node 24 installer hang (his words: DECISIONS.md,
+  2026-11-11, 20:44:29 UTC); the gate's stray-page check stops after the merge (security-auditor's F4, Low, open);
+  PROMPTS-2.3 and the guard's `git -C` refusals (the 2.1b report, "Findings for 2.3").
+- **From foundation 2.2 (PR #48), for 2.3:** AUTO-ASK, MANUAL-MODE, GIT-C, ADVISOR-TOOL-WORDS, MODELS-2026-11-12,
+  REFUSED-PUSH, PIN-GUARD, G0, OVERFLOW-XCRATE-CI (or the first crate to depend on sim), TSCONFIG, REQUIRED-CHECK,
+  SAVER-SKIP-CI, CRATE-NAMES, CI-ENFORCED-LATER, WASM-ARTIFACT, TWO-PLAYWRIGHTS, TOOLKIT-RESOLVES-UP, CARGO-NET-OFFLINE,
+  CRATE-LICENCE, CUT-CARRY, CANCEL-ON-MAIN, LINT-TABLE-PEDANTIC, FEATURE-UNIFY, ROOT-WORKSPACE, CLIENT-NET-RULE,
+  NODE-26; later: OVERFLOW-PER-CRATE (with the first third-party crate), PACKAGE-ENTRIES, EMPTY-SLOTS, OQ-0002-8-TEST,
+  BRIEF-STALE. Accepted as left open (his go for #48, item 5): SETPRIV-HARDEN, SETUP-NODE-CACHE, CUT-ESCAPE (step 2.3,
+  each a CI edit that asks him); OVERFLOW-PANIC-RULE (with the first code in the room crate or a server crate).
+- **The preview hosting check (ADR 0006's spike 1, SPIKE_PLAN entry 6)** (lane 2, in flight, 2026-11-24; its report
+  `docs/reports/2026-11-24-preview-hosting-check.md`, "Risks"; the ids are this docs pass's):
+  1. PHC-HEADER: the header mark (PM-0006-2) is not met as written: production's one-off links (0354374e and
+     de1c3e5e) carry the "keep out of search engines" header (`X-Robots-Tag: noindex`), which the host adds to every
+     one-off address; production's own address stays clean. By ADR 0006's "When a mark fails" (ADR 0006 :1404-1416)
+     a failed mark goes back to the draft: infra-engineer revises the part the row names by the draft's own routes,
+     and the revised part returns to the advisor and to Mustafa and the Advisor with the measured numbers. No word of
+     ADR 0006 changes in this pull request. Whether step 2.6 waits for the revision is his decision (ONE-OFF-HEADER).
+  2. PHC-PROBE-MAIL: the test repository's hourly probe reads upload records kept 14 days; the last expires on
+     2026-12-02 (about 18:35 UTC). After that every scheduled run ends red and GitHub mails the failure to Mustafa's
+     account; GitHub's docs do not say whether archiving a repository stops its scheduled runs, while switching the
+     workflow off does. So the probe is switched off before 2 December (his to do).
```

DONE
