# docs-keeper, part B (STATUS, ROADMAP, REPO_MAP) for docs-after-kit-v3-1

Worktree: scratchpad/docs-pass-b (branch docs-pass-b, 0099b3c). Edit only docs/STATUS.md, docs/ROADMAP.md,
docs/REPO_MAP.md. No commit, no push.

## Sources
- Plan revisions 1 and 2: `.checkpoints/docs-after-kit-v3-1.md` (B1–B8, rev 2 "B" bullet, names rule).
- Advisor conditions 1, 8, 10, 15, 16 + REPO_MAP notes 76/110: `advisor-plan-rev1.md`.
- `scratchpad/queue/docs-pass-items.md`, `docs-pass-evidence.md`.
- The caller's fact list (2026-11-04 brief).
- Checked myself: e4d8b22 dated 2026-11-03T21:57:53Z; e2b5799 10:21:55Z; 868e165 13:34:59Z; #34 changed only
  `docs/reports/2026-10-27-kit-v3-1.md` (822aaa4: "kit-v3-1 report corrections (the advisor's #30 findings 1 and 2)");
  V2_PHASE0_BRIEF lines 103 and 108 ("effort high", "Keep its triggers"); repo-map-check OK before edits (113/100).

## Plan
1. STATUS header (B1): Last updated 2026-11-04; merged list + #33, #30, #34; in flight this pass (PR #35, draft).
2. STATUS Phase (B2): drafts merged; Next = "now"; drop PR #33 merge asks; "After PR #30 merges" -> done list with the
   API state (14:02 read), the one trigger wording; brief §3 step 2 outdated clause.
3. STATUS In flight (B3): drafts row merged; kit-v3-1 row merged, drop "a draft while A1 runs"; docs-after-33 row
   folded; new rows part-b-test (#34) and docs-after-kit-v3-1 (#35, draft).
4. STATUS Routines (B4): rewrite the section with the 14:02 API read; PR Advisor prompt verbatim; Guard test B1
   deleted; old night shift disabled; old PR #26 session archived; PROBE-FILE (c) known noise; effort (EFFORT-SOURCE).
5. STATUS Open bugs and risks (B5): kit-v3-1 entries stay (PR #30 merged); new: A3's list (his three items + three
   found by B2 / plan review); B2's read limits.
6. STATUS Decisions pending (B6): new top block for 2026-11-04: PORTAL-EMBED (web portal), KIT-FOLLOWUPS (a),
   EFFORT-SOURCE (renamed from his "ROUTINE-EFFORT", said once), PR-HOME-REPLACE (a), PROBE-FILE (c), queue; line
   next to ROUTINE-EFFORT (b) (premise no longer holds; rule stays). KIT-ALIGN/TOY-BOX "after PR #30" -> point at A2.
7. ROADMAP (B7): phase line 29; "v2, next" line 69; Kit v3.1 MERGED; drafts MERGED; new entries TEST-RUN part B
   (#34) and this pass (#35, in flight). Count entries (8–12), fold oldest if over.
8. REPO_MAP (B8): only stale lines; settings.json line note on effort (advisor's optional note); "Last checked";
   run repo-map-check.

## Names rule
- EFFORT-SOURCE (never the new question as ROUTINE-EFFORT); A3 = guard follow-ups; A2 = kit-v3-2 (CHECKPOINTS.md
  rules + TOY-BOX rewording); never call the Actions item "A2". "web portal", never bare "portal". EDITOR_V2
  plan-view note / VISION_V2 community-servers note by content, never "B1"/"B2".

## Progress (a line per finished section)
- [done] 1. STATUS header (B1)
- [done] 2. STATUS Phase (B2): drafts, #30, #34 in Done; In flight + Queue; Next 'now'; PR #33 merge ask removed; setup steps done list with 14:02 API read; brief §3 lines 103/108 clause
- [done] 3. STATUS In flight (B3): new rows docs-after-kit-v3-1 (#35) and part-b-test (#34); drafts row merged; docs-after-33 row removed (folded); kit-v3-1 row merged, 'a draft while A1 runs' dropped; shell-list claim tagged [verified] routine-guard.mjs:445-446, :458
- [done] 4. STATUS Routines (B4): rewritten from the 14:02 API read; prompt verbatim; TEST-RUN A/B1/B2; Guard test B1 deleted; old night shift; old PR #26 session; EFFORT-SOURCE evidence; PROBE-FILE (c) from plan B4 (not the items file's 'every run' wording: see Questions)
- [done] 5. STATUS Open bugs and risks (B5): kit-v3-1 entries kept (stand after merge); new A3 entry (his 3 items + 3 for its plan), verified checkpoint.mjs:29-30, routine-guard.mjs:445/:594, advisor.md tools; B2's label-tool error
- [done] 6. STATUS Decisions pending (B6): new 2026-11-04 block (TEST-RUN, PR-HOME-REPLACE (a), PROBE-FILE (c), KIT-FOLLOWUPS (a) verbatim, EFFORT-SOURCE (a) renamed once, PORTAL-EMBED web portal, counts [assumed]); line next to ROUTINE-EFFORT (b); TOY-BOX follow-up -> kit-v3-2
- [done] 7. ROADMAP (B7): phase line; v2 next (no 'after #33', queue pointer to STATUS); new entries #35 (in flight) and #34; Kit v3.1 MERGED; drafts MERGED; Kit v2.1 + v2.0 folded into Build history (12 entries)
- [done] 8. REPO_MAP (B8): only line 76 (settings.json) gains the routine-effort bracket (the advisor's optional note);
  line 110 left as is (it describes the test, still true); no file added/moved/removed by this pass. repo-map-check:
  "repo map OK: 113 entries cover 100 required paths".
- [done] Final greps: no bare "portal"; "A2" only as kit-v3-2 (never the Actions item); ROUTINE-EFFORT only in the
  rename sentence and the 2026-10-25 (b) entry; no "fires on pull request events" / "After PR #30 merges" left.

## Questions / not placed
- PROBE-FILE wording conflict: docs-pass-items.md says the PR Advisor's notes "set it off at the end of every run";
  plan B4 (later, from the transcripts) says B1's probe set it off and B2 left no files. Used plan B4, as told.
- Whether the new night shift has posted on issue #16 yet (old STATUS: "tomorrow morning's comment is the first
  proof"): no fact given; the clause was dropped, not replaced.
- The count line (118 / 98) rests on the main session's README edit in the other worktree: tagged [assumed].
- The 14:02 API read is the session's, not mine: tagged [assumed]; condition 15 asks for a re-read before the last
  docs commit.
- STATUS is far beyond one screen (pre-existing); this pass did not try to trim it.
