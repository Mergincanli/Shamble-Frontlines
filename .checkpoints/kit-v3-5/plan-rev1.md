# kit-v3-5 (F1) plan, revision 1 (2026-11-10): the guard's rename gap, agent memory included

Branch `claude/sleepy-tesla-ofsxsr` stands in for `cs/kit-v3-5` (draft PR #46); the worktree is from main 44446a5
(PR #44 merged). His words: `mustafa-2026-11-09-answers.md` (GUARD-RENAME (c)); the facts: `facts.md` §1-§8, B9.

- **Kind:** a kit change-set: guard code, its tests, and two rules lines, plus a docs part. No game code: symbols
  +0 −0.
- **Risk tier:** 2 (the guard). Reviews: the advisor and security-auditor, always. Merge: his go, from a day session.
- **Change type:** logic, so the evidence is Node tests on the real guard, with mutation rows.
- **Timing:**
  - The PR Advisor routine is paused until this merges (his words). Re-enabling it is his, after the merge.
  - After this merges, KIT-FREEZE-2 holds (his words of 2026-11-10): no kit change until the foundation's gate step,
    except a broken main, STATUS-DIET and LANE-RULES. Anything left out here waits.

## 1. Goal
On main today, a pr-advisor merge passes the guard's merge check (`routine-guard.mjs:1444-1467`) when the PR:
1. only renames a protected file away (the check reads `.filename` only, :1461);
2. adds `CLAUDE.local.md` or `AGENTS.md` anywhere (`protectedRel`, :32-43, names neither);
3. force-adds `.claude/agent-memory-local/**`, which `protectedRel` exempts (:38) so that memory agents can write;
   the first 200 lines of `MEMORY.md` load into that agent's prompt (facts §5);
4. adds a nested `<dir>/.claude/skills/**` or `<dir>/.claude/rules/**`, which Claude Code loads (facts §5-§6) and
   `protectedRel` misses (it checks `.claude` as the first segment only);
5. lists 3000 files or more (the API stops at 3000, facts §4), so a protected path can hide past the list (SA-8).

Close all five in code, with tests that fail when any new check is removed. Then make the rules text that describes
the gap true.

## 2. The parts
**A. `protectedRel` (writes and merges)** gains `CLAUDE.local.md` and `AGENTS.md` at any depth, beside `CLAUDE.md`
(STATUS F1's text).
- Stricter for routine writes: they are refused.
- Stricter for day-session edits: they ask Mustafa (`dayEdit`, :936).
- The `agent-memory-local` and `worktrees` handling is unchanged, so the memory agents' writes stay allowed (test rows
  :171, :681, :757 keep their results).
- The comment at :31 ("agent instructions anywhere") becomes true for these files.

**B. A merge-only test, `mergeProtected(rel)`**, used only by the merge check (:1463). A path is protected when any of
these holds:
- `protectedRel(rel)` is true;
- the path lies under `.claude/agent-memory-local/`: it is gitignored, so it never belongs in a PR;
- any path segment is `.claude`, at any depth: nested skills, rules, agents, commands and settings.
- Writes are unchanged: memory agents still write their memory, because only the merge check uses this test (facts B9,
  "needs its own predicate").

**C. The file list reads both names.** The `--jq` filter becomes
`.[] | [.filename, (.previous_filename // "")] | @tsv`: one line per file, the new name, a tab, the old name if any.
- Both names are checked, so a renamed or copied file is caught under its old path as well as its new one (STATUS
  F1's text).
- `@tsv` escapes tabs and newlines inside a name, so one file can never split into two lines.
- A failed `gh` call already fails closed (:1462).

**D. The 3000-file cap (SA-8).** The merge check refuses a pr-advisor merge in three cases:
- `pr.changed_files` is 3000 or more;
- `pr.changed_files` is not a whole number;
- `changed_files` is not the number of lines read (one per file, part C), so the list was cut short.

The PR object is already fetched (:1450).

**E. Tests** (`routine-guard.test.mjs`):
- `fakeIo`'s file list fakes the new `@tsv` output.
- `fakeIo` gains a `changedFiles` option (default: the number of `prFiles` lines).
- New pr-advisor merge rows, refused:
  - a pure rename of `.claude/settings.json` into `docs/`;
  - `CLAUDE.local.md` at the root, and in `docs/`;
  - `AGENTS.md` at the root, and in `tools/`;
  - `.claude/agent-memory-local/advisor/MEMORY.md`;
  - `docs/.claude/skills/x/SKILL.md`;
  - `src/.claude/rules/y.md`;
  - `changed_files` of 3000;
  - `changed_files` above the list's length;
  - a `gate` run still `in_progress`. That line (:1457) has no row and no mutant today (facts B9).
- New pr-advisor merge rows, merged:
  - a docs rename (`docs/a.md` to `docs/b.md`);
  - a docs PR with matching `changed_files`.
- New write rows:
  - a routine write to `CLAUDE.local.md` and to `docs/AGENTS.md` is refused;
  - a memory agent's write to `.claude/agent-memory-local/advisor/MEMORY.md` is still allowed.
- New `MUTANTS`, one per new deny: the previous-name read, `mergeProtected`'s three clauses, the cap, the short list,
  and the still-running line. The harness asserts that each mutant's target text occurs exactly once (:551), so every
  mutant string changed with its line is updated too. Rows :280-281 keep their results.

**F. Rules text that the fix makes stale** (facts §7):
- **advisor.md tier 2 (:71-77):**
  - "(agent memory aside)" becomes "agent memory included (gitignored: a PR that carries it is tier 2)";
  - the bracket "it does not yet see a renamed-away path, `CLAUDE.local.md` or `AGENTS.md`, so read the `--no-renames`
    list yourself" becomes one sentence: the guard reads both names of a rename, and refuses nested `.claude/`,
    agent memory, and lists of 3000 files or more;
  - the `--no-renames` reading in step 0 stays, as a second check.
- **CLAUDE.md:64:** "the guard's protected files: `.claude/` except agent memory, every `CLAUDE.md`" gains
  "`CLAUDE.local.md` and `AGENTS.md`" (part A makes them protected for day-session edits too).
- **F4-RIDE (the advisor's call):** CLAUDE.md's short tier table (:95-99): row 0 reads "that adds or changes no word or
  decision" and "not CLAUDE.md, CLAUDE.local.md, AGENTS.md, agent memory or a rename", plus one clause: "steps 4 and 5
  apply at every tier; decision docs always need his go". This follows STATUS F4, the verifier's L3 and the advisor's
  Low. Under KIT-FREEZE-2 it would otherwise wait until the foundation's gate step.

**G. The docs part** (no ask):
- **STATUS:**
  - #44 merged: 44446a5, 2026-11-09 18:21 UTC, on his go; main's CI was green after one re-run;
  - the stale lines :3, :44-45 and :114;
  - the routine paused by him until this merges;
  - F1 carried out here;
  - F4 and F5 by the ruling;
  - his queue entries word for word: the unit walk (2026-11-09) and the queue of 2026-11-10 (KIT-FREEZE-2,
    STATUS-DIET, LANE-RULES), with his GUARD-RENAME (c) and his go for #44;
  - :793 "no MCP connections" made precise: the routine's trigger has none, and its GitHub tools come from the
    environment.
- **ROADMAP:** #44 merged; kit-v3-5 in progress.
- **The kit-v3-4 report:** a correction after the merge, with the advisor's two Lows (mixed verdict rows counted by their
  first word; Appendix D missing ddb06a5 and 2894a1f) and its agent-memory finding.
- **This pass's report.**

## 3. Files and symbols
- **Kit** (each write asks him in a day session, one at a time):
  - `.claude/hooks/routine-guard.mjs`: `protectedRel` changed, `mergeProtected` added, `mergeCheck` changed;
  - `.claude/hooks/routine-guard.test.mjs`: rows and mutants;
  - `.claude/agents/advisor.md`: two lines;
  - `CLAUDE.md`: one line, plus F4 if ruled.
  - That makes four asks, or five with F4 in its own write.
- **Docs:** STATUS, ROADMAP, the kit-v3-4 report, this report.
- **Not changed:** no settings, `.github/` or gate file.
- **Game symbols:** +0 −0.

## 4. Build order
1. The tests first, against the real guard on main, to show each new row fails before the fix (red rows, recorded).
2. Then the guard edit, and all rows pass; then the mutants; then the rules lines; then docs.

## 5. Evidence
- **E1, red then green:** before the guard edit, each new refused row fails (the gap is real); after it, all rows pass.
  Counts before and after.
- **E2, mutants:** every new deny has a mutant that some row catches; the mutant harness passes (no survivors).
- **E3, all kit tests:**
  - routine-guard: 1034 plus the new rows, all passing;
  - settings: 127 (unchanged);
  - checkpoint: 27 (unchanged);
  - the counts before and after go in the report.
- **E4, the gate and the repo map:** `node tools/gate.mjs` passes with symbols +0 −0; `repo-map-check.mjs` is OK.
- **E5, a live read:** the exact new `gh api ... --jq` command on a real PR of this repo with a renamed file, if one
  exists (read-only), prints both names. If none exists, say so, and rest on the GitHub docs (facts §4).
- **E6, rules:** a rule-diff table for the advisor.md and CLAUDE.md lines (before, after, looser / stricter / same);
  only stricter or same is expected.
- **E7, his words:** byte for byte in STATUS's queue (from the saved copies, which match the transcript).
- **E8, scope:** `git diff --name-only origin/main`.

## 6. Reviews (one round, at most 3 agents, Opus 5.5)
- **security-auditor** (tier 2, the guard):
  - every gap in §1 closed, with no new way around;
  - writes still safe;
  - fail-closed paths.
- **test-engineer:** are the new rows and mutants enough to prove each check; are the fakes faithful to `gh`'s output.
- **The verifier:** E1-E8 (outside the review budget).

## 7. Decisions
- **SA5-RIDE (Mustafa's).** SA-5: should the guard also refuse the PR Advisor's merge of `docs/adr/**` and the decision
  docs?
  - Today that rule is the advisor's judgment only (advisor.md tier 0/1). Under KIT-FREEZE-2, "no" means it waits
    until the foundation's gate step.
  - (a) Yes: a fixed list in the guard (`docs/adr/`, VISION_V2, LOOP_V2, LOOK_V2, EDITOR_V2, V2_PHASE0_BRIEF,
    DESIGN_NOTES, ART_DIRECTION, GAME_DESIGN, UNITS, and STATUS-DIET's coming DECISIONS.md), with test rows; stricter.
    It cannot catch a quote of him in another doc, so the advisor's judgment stays too.
  - (b) No: judgment only, as now.
  - Recommendation: (a), while the guard is open anyway.
- **F4-RIDE (the advisor's).** CLAUDE.md's short-table wording (§2 F): fold it in now, since the freeze follows, or
  leave it as a follow-up. Recommended: fold in; strict side, one CLAUDE.md write that this pass makes anyway.
- **MERGE-ONLY (the advisor's).** Agent memory and nested `.claude/` are refused in merges only (`mergeProtected`), not
  in writes, because protecting agent memory for writes would break the memory agents. Recommended as above.

## 8. Mustafa's check
- His approvals for the four or five kit writes, one at a time.
- His answer to SA5-RIDE.
- His go on the final head (tier 2).
- Re-enabling the PR Advisor routine after the merge.
- No Brave check.

## 9. Risks
1. **A new check blocks a legitimate routine merge,** such as a docs PR whose list is cut short by a GitHub glitch. It
   fails closed: the routine leaves the PR to a day session, which is the safe side.
2. **The mutant strings must match the edited lines** exactly. The harness asserts it (:551), so a mismatch fails
   loudly.
3. **`--jq` per page under `--paginate`** was not checked live (facts B9). E5 checks it on a real PR if one exists.
4. **More than 100 check runs on one SHA** is not handled (:1454 reads one page). Out of scope: a follow-up.
5. **The freeze follows this pass,** so anything missed waits. Hence the explicit F4 and SA-5 decisions.
