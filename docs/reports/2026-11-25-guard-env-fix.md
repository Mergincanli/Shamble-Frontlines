# Advisor report: guard-env-fix (2026-11-25, branch claude/confident-gates-tbnq38 for cs/guard-env-fix, base main 3ab9eaec)
**Risk tier 2** (a kit file, `.claude/hooks/`): needs-mustafa; the merge is his, from a day session. PR #69.
## Goal
Close SEC-U7B-1 (security-auditor, 2026-11-25): the day guard's `unwrap()` dropped env, nice and stdbuf option words
but not their values, so git or gh behind them got no check and no question in a day session or a subagent.
## Outcome
DONE. Options are read with their values; what the guard cannot follow asks the main session and refuses a subagent.
## Plan and approval
Plan: `unwrap()` reads the three wrappers' options from a table; an unknown option is never guessed. The advisor agent:
APPROVE with 7 conditions, all met: (1) a leftover `-` program word asks; (2) the reason names the form; (3) the flag is
set before reading on, with deny rows; (4) mutant targets occur once, no old row touched; (5) probe before/after;
(6) docs without the out-of-scope forms; (7) interim rule kept, gate and tests after each step.
## What changed
- `routine-guard.mjs` (+47/−6): `WRAP_OPTS` (a Map: coreutils 9.4's flags, value options, env `-C`/`-S`);
  `unwrap()` returns `{ argv, unreadable }`; env `-S`'s value is read on; `shownWord()` cuts echoed words; `bash()`
  asks or refuses an agent on an unreadable form, and the scan goes on, so a later deny wins. Symbols: +WRAP_OPTS,
  +shownWord, −none (matches the plan; shownWord came from SA-R1-4). Routines unchanged (R-PROG refuses wrappers first).
- `routine-guard.test.mjs` (+47/−0): 140 new rows (each known form with a force push, which must be denied, and with
  git status, which must stay allowed; each unfollowed form asks, and a subagent is refused; routine rows); 12 mutants.
## Evidence
- Steps: plan 20902d9 · edit 21615cb · round-1 fixes 11887d0 · round-2 rows c3827f4 (code head) · probe fix a163b88.
- Tests: routine-guard 1247/1247 (main 1107/1107); settings 127/127; checkpoint 33/33 (both = main). The new rows run
  on main's guard: 67 fail, 41 of them force pushes main let through (verifier).
- `unwrap()` called alone on 21 forms: before, 9 took a value for the program; after, every form reaches git or asks.
- Gate: checks 1-4 and 5b-5d PASS, symbols +0 −0. Smoke 5a could not start here (Playwright 1.56.1 wants Chromium
  build 1194, the container has 1243); no game file changed, CI runs it.
## Reviews
- security-auditor round 1 on 21615cb: REVISE. SA-R1-2 (Medium: env -S made a deny an ask) fixed 11887d0; SA-R1-1,
  SA-R1-4 (Low) fixed; SA-R1-3 (Low, on main too: the loop's turn cap) deferred. Round 2 on 11887d0: APPROVE; SA-R2-1
  (Low, three rows) fixed c3827f4; SA-R2-2 (Low, on main too: very long lines vs the hook timeout) and a nit deferred.
- Verifier on c3827f4: FAIL on V-1 (High: a stale probe file), fixed a163b88, re-run PASS; V-2 (Low: folder tracking
  after an env -S value; still an ask, 714-line sweep, no pass) deferred.
- An automated plugin review flagged env -S's blank split (Medium). Checked main against head: where env would split
  the value otherwise, main gave no check and the head asks; no deny turned into an ask. Kept.
## Decisions, risks, not verified
- Two review rounds used; deferred Lows are STATUS follow-ups; open security findings keep this needs-mustafa. Same-kind
  gaps in other wrappers and keywords: reported to Mustafa, not fixed, their forms kept out of the repository (GEF-SAME).
- Risk: an unusual wrapper form now asks where it passed before (by design). Not verified: smoke here, gh forms end to
  end, the hook-timeout behaviour (SA-R2-2, from memory). Next step: SAME-GAP's change-set, if he says (a).
## Decisions needed
- CAP: does his 150-line cap cover the code files only, or docs and report too? (a) code only; (b) everything. This
  change keeps (b): 94 code and test lines, the rest docs. Recommendation: (a) for later change-sets.
- SAME-GAP: fix the same kind of gap in the other wrappers? (a) a follow-up guard change-set; (b) a STATUS line only.
  Recommendation: (a), soon: it is the same severity as SEC-U7B-1.
## For Mustafa
1. Answer CAP and SAME-GAP. 2. After the advisor agent approves the final head: say "merge #69" for that head in a day
   session's chat (its full SHA is in the chat).
