A1-AM1: the advisor's verdict, word for word (2026-11-04, about 10:10 Amsterdam; advisor agent in the cloud session
session_0173vVo7FcYUpbWgRW8oQyHU, Opus 5.5, from main's advisor.md; its working notes: advisor-am1.md)

## Verdict on A1-AM1: APPROVE, with conditions

I reproduced BH-1 myself and confirmed the fix opened it. In the old form (5be7d37), an import cycle or a top-level await that never settles makes node exit 13, which blocks. In the fixed form (537c7a8), node exits 0 with no output, so a routine passes all three hooks. With BOOT', a routine is blocked in every failure state I tried (13 states). In the same states a day session does exactly what it does under the fix. With guard.mjs present, the results are byte-identical. This tightens the routine side of a decision Mustafa already made (GUARD-FILE: a routine fails closed, a day session goes on). So it is a technical choice inside the documented rules, and not an escalation. PR #30 stays `needs-mustafa` anyway (rules category).

**New C3 (C3'): this exact BOOT', byte-identical in all three guard handlers (451 characters):**
```
process.on('beforeExit',function(){if(process.env.SHAMBLE_ROLE){process.stderr.write('guard.mjs ended without a decision: a routine fails closed');process.exit(1)}});import('node:url').then(function(u){return import(u.pathToFileURL(process.argv[1]).href)}).catch(function(e){if(process.env.SHAMBLE_ROLE){process.stderr.write('guard.mjs could not load ('+String(e?e.message:e).slice(0,300)+'): a routine fails closed');process.exit(1)}process.exit(0)})
```
It is the amendment's item 1 prefix followed by the rev2 BOOT, unchanged. I checked this with a string comparison against the bypass hunter's settings-cand.json.

**Conditions** (C1, C2 and C4–C9 still stand):
1. **AM1-C1, settings.json diff.** The diff changes only the three BOOT strings. Keep the file's current layout: the candidate file reflowed the handlers across several lines, so don't copy it wholesale. Every other key stays as it is. (I checked that the candidate's non-hook settings are identical.)
2. **AM1-C2, the two new file states.** Test "unsettled await" (the import cycle in routine-guard.mjs) and "empty guard.mjs" on all three events, for a day session and for every value in `ROUTINES`.
   - A day session exits 0 with empty stdout.
   - A routine exits 1 with empty stdout.
   - At least one routine case asserts stderr matches `/ended without a decision.*fails closed/`. That tells this path apart from the "could not load" path.
   - `withProject`'s `change` callback must be able to edit routine-guard.mjs too (pass it the hooks folder, or a second argument). List that helper change in the symbol diff. All of this happens in the temp copy only: the real routine-guard.mjs stays byte-identical (C6).
3. **AM1-C3, a routine that is allowed still passes.** Add a case to the "guard.mjs present" test: a routine making an allowed call (for example `git status` with `SHAMBLE_ROLE=night-shift`) exits 0 with empty stdout. Today no test covers that path, and it is exactly the regression a `beforeExit` mistake would cause: every routine call blocked. It must pass on both 537c7a8's settings and the new ones.
4. **AM1-C4, the defect proof (extends C1).** Against 537c7a8's settings, the new routine cases fail with exit 0. Against the new settings they pass. Record both counts in the report.
5. **AM1-C5, guard.mjs header (C7).** Comment only, worded as in item 3, and the invariant is explicit:
   - all work stays inside the top-level try;
   - the file ends with its one unconditional `process.exit(0)`;
   - removing that exit blocks every routine call. I proved this with the "noexit" state: a routine gets exit 1 even after printing its decision.
   - "Only the bootstrap ever exits non-zero" stays true.
6. **AM1-C6, security-auditor's review of the delta** includes an adversarial pass:
   - re-run the bypass hunter's matrix (`scratchpad/bypass-hunter/matrix.mjs`) against the final settings.json;
   - confirm no normal path reaches `beforeExit`.
7. **AM1-C7, the report** carries the old / fix / AM1 table, measured on the final head, with the node version.

## Rulings on the dispositions
- **L1 wording:** upheld, use the item 4 texts.
  - routine-guard.mjs:601 says "for every session". That is true only while guard.mjs loads. C6 forbids touching that file, so the docs qualifier covers it. Note it in the report.
  - The server-side backstop already exists: STATUS.md line 3 records that main is protected and needs `gate` and `smoke` green (Mustafa, 2026-10-17). I confirmed the line is there. Nothing new is needed.
- **L2 (a broken pipe when the bootstrap exits before reading a large input, EPIPE): defer, upheld.** It happens only when guard.mjs is missing and the input is over about 60 KB. Either the host ignores it or it blocks, so a routine is never let through.
- **L3 and BH-3 as stated limits: upheld.** A guard.mjs that loads and then crashes, hangs or prints invalid JSON blocks the call. That costs availability, not safety. One addition for the limits text: a hang holds a day session up to the default 600-second hook timeout per call. Choosing a shorter `timeout` stays a separate decision.
- **N1, folded into the Q3 follow-up (hook tests in CI, a Linux job first): upheld.** The two new states live in settings.test.mjs, so that job picks them up.
- **N2, keep: upheld.** The v2.1.295 pin is recorded in STATUS.md:34 and the report's setup step.
- **N3, no action: upheld.** An argument starting with `-` can't occur with an absolute CLAUDE_PROJECT_DIR.
- **N4 and BH-2, closed by item 1: upheld.** Verified: empty and truncated guard.mjs now block a routine on all three events.

## Review path
Security-auditor on the AM1 change only, then my review of the whole PR at its final head (C8, C9). That is right, as long as AM1-C6 is met. Security-auditor is the GUARD-FILE reviewer, and the change is one statement in the bootstrap plus tests and comments.

## What I verified myself
All runs: Node v22.23.4, exec form, no shell, in scratchpad/advisor-am1/.
- The three settings versions: old and fix from git (`git show 5be7d37:` and `git show 537c7a8:` for `.claude/settings.json`). Worktree HEAD 6ce59f3's settings.json is byte-identical to 537c7a8's.
- My own harness, scratchpad/advisor-am1/h.mjs: 13 file states (present, missing, syntax error, top-level throw, folder, dangling symlink, empty, truncated, noexit, cycle, tla, tladecide, routine-guard missing), each with the three settings versions, three events, and a day session or a routine. BH-1 reproduced (fix form, cycle, tla and tladecide: a routine gets exit 0 with no output on all three events; the old form exits 13). BOOT' blocks a routine (exit 1) in every state where guard.mjs fails. Under BOOT' a day session matches the fix in every one of the 13 states. Present and routine-guard-missing: the same JSON decisions in all three forms.
- Equivalence with guard.mjs present, scratchpad/advisor-am1/eq.mjs: fix versus BOOT', 204 of 204 runs identical in exit code, stdout and stderr (17 inputs, three events, four role values: none, night-shift, pr-advisor, a single space). 18 of those runs were allowed routine calls; they exit 0 with no output under BOOT'. Average 37.9 ms per call (fix) against 37.2 ms (BOOT').
- BOOT' text: one distinct BOOT across the three events; it equals the AM1 prefix plus the rev2 BOOT; it holds none of `$ " \` \ & | < > ^ %` and no non-ASCII or control characters.
- No normal path reaches `beforeExit`: guard.mjs (40 lines) does all its work inside the top-level try and ends with an unconditional `process.exit(0)`; the bootstrap's `.catch` always exits; routine-guard.mjs does its I/O only through `spawnSync`, its `async` functions are wrappers, so nothing stays pending when the loop empties; an explicit `process.exit` doesn't emit `beforeExit`, which the 204 identical runs confirm.
- Hook semantics: `git show 2a75345:.checkpoints/kit-v3-1/hook-docs.md` says any exit other than 0 or 2 is a failure, and with `onFailure: block` a failure acts as exit 2 (a deny on PermissionRequest). The default timeout is 600 seconds.
- Tests at head 6ce59f3: settings.test plus routine-guard.test, 290 of 290 pass. With BOOT' as settings.json in a scratch copy, the existing settings.test passes 98 of 98, including the "present" test.
- The main session's harness (verify-bh1/run.mjs): read; mine covers its six states and seven more.

## What I could not verify
- Live Claude Code hook behaviour (that exit 1 plus `onFailure: block` really blocks in a routine): the recorded official docs quotes only; fetches were not allowed. The same basis as rev2.
- Other Node versions (for example Node 24 in the routine environments): `beforeExit` is long-standing Node behaviour, and a newer Node exiting 13 would still block, so the routine side fails closed either way.
- Windows: not run here. Routines run on Linux.
- The gate: not run, as this is a plan review; required at the whole-PR review (C9).

## Decisions needed
None. AM1 stays inside Mustafa's GUARD-FILE decision and only tightens the routine side.

## For Mustafa
Nothing now. PR #30 stays `needs-mustafa`: once its final head has the advisor's APPROVE, he reconfirms the merge in chat.
