# A1 plan amendment 1 (A1-AM1), 2026-11-04: close BH-1 (the bootstrap's own fail-open for routines)

Asks the advisor to amend condition C3 of plan revision 2. Everything else in the plan and C1–C9 stands.

## Why
- **The finding.** The bypass hunter (workflow `wf_dadc7576-164`, notes `bypass-hunter-a1.md`) reported BH-1 (Medium).
  The main session reproduced it with its own harness (`scratchpad/verify-bh1/run.mjs`).
  - In the old form (`node guard.mjs`, 5be7d37), an unsettled top-level await in guard.mjs or anything it awaits makes
    node exit 13 ("Detected unsettled top-level await"). That is a hook failure, a block for everyone.
  - In the fixed form (`node -e BOOT guard.mjs`, 537c7a8), guard.mjs is a dynamic import. Its promise just stays
    pending, the event loop drains, and node exits 0 with no output. That means "no decision", so **a routine passes
    all three hooks**.
- **Concrete file states:**
  - `routine-guard.mjs` gains `import './guard.mjs';` (an import cycle deadlocks guard.mjs's
    `await import('./routine-guard.mjs')`);
  - or guard.mjs gets an `await` on a promise that never settles.
- **Who can cause it.** A routine cannot write these files itself (protectedRel; checkout refused). It would take a buggy
  edit, or another branch's files. A security fix should not open a routine fail-open that the old form did not have.
- **Related, BH-2 (Low, pre-existing).** An empty or truncated guard.mjs lets a routine through in the old and new forms
  alike. This is security-auditor's N4 as well.

## The change
1. **`.claude/settings.json`**, the three guard handlers, still byte-identical to one another. BOOT' is the plan
   revision 2 BOOT with one statement in front:
   `process.on('beforeExit',function(){if(process.env.SHAMBLE_ROLE){process.stderr.write('guard.mjs ended without a decision: a routine fails closed');process.exit(1)}});`
   followed by the revision-2 text, unchanged.
   - `beforeExit` fires only when the event loop empties without an explicit exit.
     - guard.mjs ends with an unconditional `process.exit(0)`, and the bootstrap's `.catch` always exits too, so on
       every normal path the handler never runs.
     - It runs only when guard.mjs never finishes (an unsettled await) or finishes without exiting (an empty or
       truncated file).
     - Then a routine exits 1, which is a block; a day session does nothing and exits 0, no decision, as today.
   - BOOT' still holds none of `$ " \` \ & | < > ^ %`.
2. **`.claude/hooks/settings.test.mjs`**, two more file states in the GUARD-FILE section:
   - an unsettled top-level await (the import cycle above);
   - an empty guard.mjs.

   In both, a day session must exit 0 with no output and a routine must exit 1.
   - The defect proof for them: against 537c7a8's settings, both routine cases fail (exit 0).
3. **`.claude/hooks/guard.mjs`**, header comment only (C7):
   - "missing or broken" becomes "missing or can't load" (security-auditor L3);
   - it adds "or, for a routine, ends without its own exit";
   - it states the invariant: all work stays inside the top-level try, and the file ends with its single explicit
     `process.exit(0)`.
4. **Docs, the wording only (security-auditor L1):**
   - STATUS's kit-v3-1 row and ROADMAP's entry: "merges fail closed" becomes "merges fail closed while the guard loads".
   - STATUS's GUARD-FILE note: "missing or broken" becomes "missing or can't load".
   - REPO_MAP's guard.mjs row gains the routine-ends-without-a-decision case.
5. **Not changed:** `routine-guard.mjs` (C6), and guard.mjs's code (C7).

## Evidence already in hand (main session, Node 22.23.4, exec form, no shell)

| guard.mjs state | old (5be7d37) | fix (537c7a8) | AM1 |
|---|---|---|---|
| present | as designed | as designed | identical decisions; 39 ms per call, as the fix |
| missing | day **blocked**, routine blocked | day passes, routine blocked | the same as the fix |
| syntax error | day **blocked**, routine blocked | day passes, routine blocked | the same as the fix |
| empty | day passes, routine **passes** | day passes, routine **passes** | day passes, routine blocked |
| import cycle | day blocked (exit 13), routine blocked | day passes, routine **passes** | day passes, routine blocked |
| unsettled TLA | day blocked (exit 13), routine blocked | day passes, routine **passes** | day passes, routine blocked |

## Dispositions of the other findings (for the advisor to confirm)
- **L1 F5** (introduced; Mustafa's decision): accepted. The wording is fixed in item 4.
  - security-auditor's infra follow-up is a server-side rule on main requiring gate and smoke. STATUS already records
    that main is protected that way (Mustafa, 2026-10-17). What the merge check adds on top is the expected-head check.
- **L2 EPIPE** (pre-existing): defer. It is host behaviour, and the ConfigChange reload (small input) is unaffected. A
  later stdin drain needs its own BOOT change.
- **L3 and BH-3** (pre-existing; outside GUARD-FILE's "missing" scope): stated as limits. A guard.mjs that loads and then
  crashes, hangs, or prints invalid JSON still blocks a day session; the real guard.mjs can't do any of these as
  written. A short handler `timeout` is a separate decision.
- **N1:** folded into the Q3 follow-up (the hook tests in CI, a Linux job first).
- **N2:** keep; the routine environments pin Claude Code v2.1.295 or later (#30's setup step).
- **N3:** no action (`--` only if BOOT is revised for another reason).
- **N4 and BH-2:** closed by item 1.

## Reviews
After the build, security-auditor reviews only this delta (GUARD-FILE: "security-auditor reviews that change"). Then the
advisor reviews the whole PR at its final head, as planned (C8, C9).

## The question
- **AM1:** approve the amended BOOT' (C3 becomes C3', with BOOT' exactly as in item 1) together with items 2–4?
  Recommendation: yes.
- The alternative: keep C3, and record BH-1 as a known limit, since a routine cannot create the file state itself.
