# checkpoint.mjs evidence (2026-10-25, after the security fixes)
$ node --test --test-timeout=60000 .claude/scripts/checkpoint.test.mjs
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
ok 11 - the task checkpoint file must exist: none yet, or deleted at the end
ok 12 - notes that look like a secret are not committed, and the secret is not printed
ok 13 - skip markers from the caller are refused
ok 14 - --every and --for are bounded
ok 15 - watch never publishes code commits
ok 16 - watch stops when the checkpoint file is gone
ok 17 - a trailer with a newline is refused, and nothing runs through a shell
# tests 17
# pass 17
# fail 0

Mutants (each guard removed in a copy; CHECKPOINT_SCRIPT=<copy>): the test that fails
pattern      not ok 5 - a branch outside cs/ and claude/ is refused # pass 15 # fail 1 
main         not ok 3 - main is refused # pass 15 # fail 1 
detached     not ok 4 - a detached head is refused # pass 15 # fail 1 
force        not ok 6 - a rejected push is reported and never forced # pass 15 # fail 1 
secret       not ok 12 - notes that look like a secret are not committed, and the secret is not printed # pass 15 # fail 1 
taskfile     not ok 11 - the task checkpoint file must exist: none yet, or deleted at the end # pass 15 # fail 1 
skipmark     not ok 13 - skip markers from the caller are refused # pass 15 # fail 1 
publishcode  not ok 15 - watch never publishes code commits # pass 15 # fail 1 
