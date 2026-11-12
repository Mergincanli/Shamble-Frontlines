# U4d2 round 1: test-engineer review notes (log prefix te)

Role: test-engineer (origin/main .claude/agents/test-engineer.md). Copy I run in: .claude/worktrees/u4d2-build (the only runner there now).
Read copy (byte-identical): .gate/u4d2-r1. Base copy: .gate/u4d2-prefix (51520b4, spikes/ = bfda3cd).
Every cargo/check.mjs/loader run goes through u4d2-run.sh; logs .gate/u4d2/r1-te-<name>.log.

## Plan
1. sha256 of every changed and new file under spikes/netcode-proof in u4d2-build; compare with u4d2-r1.
2. Bites (§5 U4d2 Bites line; §4.14 hash-map row; U4D2-Q5, Q11, Q12):
   - the builder's six plants (p1-features .. p6-usize): read the logs; re-plant in my own form where I doubt it;
     each seen to fail by name in the check or test that must catch it.
   - PM-0004-21 hash-map and std-sin in the wasm step: can either verdict pass for the wrong reason
     (Wasm leg not run, equal digests because nothing hashed, committed scan stale)?
   - export-signature bite (an export taking an f64, Q7): plant it if the builder did not.
3. h1..h8 (brief's numbering) and check.test.mjs's new and re-pinned self-tests: assertion vs print, wrong-reason pass,
   missing test/clause.
4. Carries with a test: 52 (generator refusal), 65 (failing-test names), 66 (#[ignore] test run by name): one plant each where cheap.
5. The builder's TE2-5 entry: the check.test.mjs test that failed mid-build; does it still pin what it pinned at <base>?
6. One counted `node check.mjs` with -x at the end; step lines; counts as deltas on 399 / 446 / 52 / 10 steps.
7. End: every file byte-identical to (1)'s hashes; `git diff --stat -- spikes/netcode-proof/crates/sim` empty.
Also: settle by run any finding of rust/security/infra notes a run can settle.

## Notes (a line after each finding)
