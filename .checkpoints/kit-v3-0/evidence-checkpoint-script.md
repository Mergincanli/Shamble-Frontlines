# checkpoint.mjs evidence (2026-10-25)
$ node --test .claude/scripts/checkpoint.test.mjs
ok 1 - save commits only .checkpoints/, keeps other staged work staged, and pushes the branch
ok 2 - nothing to save makes no commit
ok 3 - main is refused
ok 4 - a detached head is refused
ok 5 - a branch outside cs/ and claude/ is refused
ok 6 - a rejected push is reported and never forced
ok 7 - CI is skipped only when every pushed commit touches .checkpoints/ alone
ok 8 - bad usage is refused
ok 9 - watch saves changes and stops when the branch changes
ok 10 - watch makes a last save when it is terminated
# tests 10
# pass 10
# fail 0
$ CHECKPOINT_SCRIPT=<mutant: pattern> node --test .claude/scripts/checkpoint.test.mjs
not ok 5 - a branch outside cs/ and claude/ is refused
# pass 9
# fail 1
$ CHECKPOINT_SCRIPT=<mutant: main> node --test .claude/scripts/checkpoint.test.mjs
not ok 3 - main is refused
# pass 9
# fail 1
$ CHECKPOINT_SCRIPT=<mutant: detached> node --test .claude/scripts/checkpoint.test.mjs
not ok 4 - a detached head is refused
# pass 9
# fail 1
$ CHECKPOINT_SCRIPT=<mutant: force> node --test .claude/scripts/checkpoint.test.mjs
not ok 6 - a rejected push is reported and never forced
# pass 9
# fail 1
