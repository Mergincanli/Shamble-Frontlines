# checkpoint.mjs evidence (2026-10-25, after the security fixes and the commit-what-was-scanned change)
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
ok 18 - a note whose name git would quote is still scanned, and an unreadable note is refused
ok 19 - the commit holds exactly the scanned notes and leaves the index in step with HEAD
# tests 19
# pass 19
# fail 0

Mutants (one guard removed per copy; CHECKPOINT_SCRIPT=<copy>): the tests that fail
pattern      not ok 5 - a branch outside cs/ and claude/ is refused # pass 18 # fail 1 
main         not ok 3 - main is refused # pass 18 # fail 1 
detached     not ok 4 - a detached head is refused # pass 18 # fail 1 
force        not ok 6 - a rejected push is reported and never forced # pass 18 # fail 1 
secret       not ok 12 - notes that look like a secret are not committed, and the secret is not printed not ok 18 - a note whose name git would quote is still scanned, and an unreadable note is refused # pass 17 # fail 2 
quotednames  not ok 11 - the task checkpoint file must exist: none yet, or deleted at the end not ok 19 - the commit holds exactly the scanned notes and leaves the index in step with HEAD # pass 17 # fail 2 
failopen     # pass 19 # fail 0 
taskfile     not ok 11 - the task checkpoint file must exist: none yet, or deleted at the end # pass 18 # fail 1 
skipmark     not ok 13 - skip markers from the caller are refused # pass 18 # fail 1 
publishcode  not ok 15 - watch never publishes code commits # pass 18 # fail 1 

failopen survives: with -z names every listed note is readable, so the unreadable-note branch has no direct test; the quotednames mutant shows the scan fails closed (it refuses saves rather than skipping notes).
