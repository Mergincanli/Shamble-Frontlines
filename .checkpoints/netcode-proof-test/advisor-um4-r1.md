# Advisor: UM-4's round-1 ruling (head 8fb32bc8, pushed in 787eb984), 2026-11-27, effort xhigh

Plan: read main's advisor.md (its severity scale is CLAUDE.md's; PR review step 4; the review budget);
review-um4-nb2b-test.md block 1 (T-1 to T-5, T's fix); review-um4-security.md whole (SEC-UM4-1 to -3, UM4H-1's
corrected clause, :51-53, :136-138, :165-171); my advisor-um4-nb2b-head.md:56-60; um4-plan.md's "Review round 1";
check.mjs's opsStep and run (the env passed to children), check.test.mjs's ops tests and the web step's test
(:1985-1995); operator.test.mjs's git helper (:157); sec-um4-1-probe.md; checkpoint.mjs's save path (what a save would
push); history.md:8133-8151 (U8-FILLS (a)) and :7561-7593 (NPT-TRIM). Then one block per item 1-6 and the memory line;
Decisions needed; For Mustafa; status line. One simple command per Bash call, git first; no test, cargo or gate run.

## Notes (one line per check)
- advisor.md (origin/main, c9bd268): severity is CLAUDE.md's; PR review step 4 counts "a security finding left open (at
  any severity; one not fixed in the PR, a STATUS follow-up included, counts as left open)" as needs-mustafa; Review
  budget: "A finding's severity is its reviewer's label; only you change it, in writing, with the reason."
- review-um4-security.md:21-55: SEC-UM4-1 Medium; S's fix names six variables (incl. GIT_NAMESPACE) for opsStep and
  the helper, plus GIT_CEILING_DIRECTORIES in the helper; :163-166 "keeps the PR needs-mustafa while open".
- sec-um4-1-probe.md: the probe (stand-in folders only) recorded "real HEAD tree: verdict.txt"; not re-run (read only).
- check.mjs:2685-2706 opsStep passes no env to either call (`run`'s env defaults to process.env, :1951);
  webStep (:2664) already takes `base = process.env` and passes it to each call: the injection form exists.
- check.test.mjs:1910-1917 recorder records `env` per call: an env pin needs no real process.env change.
- operator.test.mjs:151-188: helper :157 has no env; `git add .` stages deletions, which is why S's tree was verdict.txt.
- NEW, outside both reviews: ops/operator/redact-check.test.mjs:157-191 and :193-230 build `{ ...process.env, GIT_AUTHOR_*
  ... }` and run `git init`, `add`, `commit` (message naming a public address, :179), and :212-216 `checkout -b side`,
  `checkout main`, `merge`, in a mkdtemp folder. They run in the ops glob AND in STEPS.redact (check.mjs:2741-2746,
  `run` with process.env), which predates UM-4 (added d9845355/41e55653, 2026-11-06, U1). Reasoned from S's probe, not
  run: an absolute GIT_DIR would put those commits, a `side` branch and a checkout of `main` into the named repository.
- PRECEDENT, check.test.mjs:1497-1525 and results/u3c-kernel-check/test_siocoutq_root_check.py:22-46 (u3c review C2b
  F2): the same hazard met before; fixed by leaving out EVERY GIT_* variable in the test's own git, core.hooksPath an
  empty folder, commit.gpgsign=false, proven by a victim-repository test (GIT_DIR names a scratch victim; byte-for-byte
  unchanged). No one exported a real GIT_DIR for that proof.
- The ops tests' git calls are four, all local (operator.test.mjs:157, browser.test.mjs:109 `ls-files`,
  redact-check.test.mjs:167, :203; redact-check.mjs:291 inherits); wrappers.test.sh names git 0 times (grep -c).
- Repo owner uid 0 = this shell's uid 0 (`ls -ldn`, `id -u`): no ownership refusal expected once GIT_CONFIG_* is gone.
- history.md:8137-8142 U8-FILLS (a), word for word: "The values filled in at Stop B and the `ops` step in the check
  script sit outside U8's stop. The two one-line security notes are built at that fill and count, so the stop becomes
  6,284." msg-2026-11-28.md:74: the ops step is "outside U8's own files"; u8-f1-brief.md:14-15: the stop counts
  numstat of the files U8 touches (ops/). So check.mjs and check.test.mjs lines are outside; ops/ lines count.
- history.md:7561-7593 NPT-TRIM: "One review round per unit ... A second round only for an open Critical or High."
- review-um4-nb2b-test.md:73-130: T-1 Medium (M1, M3 survive; fix shown in scratch), T-2 to T-5 Low; :96-97 the web
  step's W1 survives too (check.test.mjs:1985-1993, stand-in :1988 `out: "boom\n"`, prefix-only assert :1993).
- um4-plan.md:185-212: the round recorded; the commit message's 3-of-5 CORRECTION recorded, "History is not rewritten."
