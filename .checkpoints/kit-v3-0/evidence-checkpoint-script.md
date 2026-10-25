# checkpoint.mjs evidence (2026-10-25, final)
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
ok 20 - the value of a secret environment variable is refused, and only its name is printed
ok 21 - more token shapes are refused: URL-encoded GitHub, JWT, npm, Google
ok 22 - a flagged note is left out, the clean notes are still saved
ok 23 - a renamed note with a token added is refused
ok 24 - code added in a merge commit is never marked [skip ci]
# tests 24
# pass 24
# fail 0

Mutants (one guard removed per copy; CHECKPOINT_SCRIPT=<copy>): the tests that fail
pattern      not ok 5 - a branch outside cs/ and claude/ is refused # pass 23 # fail 1 
main         not ok 3 - main is refused # pass 23 # fail 1 
detached     not ok 4 - a detached head is refused # pass 23 # fail 1 
force        not ok 6 - a rejected push is reported and never forced # pass 23 # fail 1 
secret       not ok 12 - notes that look like a secret are not committed, and the secret is not printed not ok 18 - a note whose name git would quote is still scanned, and an unreadable note is refused not ok 20 - the value of a secret environment variable is refused, and only its name is printed not ok 21 - more token shapes are refused: URL-encoded GitHub, JWT, npm, Google not ok 22 - a flagged note is left out, the clean notes are still saved not ok 23 - a renamed note with a token added is refused # pass 18 # fail 6 
envsecret    not ok 20 - the value of a secret environment variable is refused, and only its name is printed # pass 23 # fail 1 
quotednames  not ok 11 - the task checkpoint file must exist: none yet, or deleted at the end not ok 18 - a note whose name git would quote is still scanned, and an unreadable note is refused not ok 19 - the commit holds exactly the scanned notes and leaves the index in step with HEAD not ok 22 - a flagged note is left out, the clean notes are still saved not ok 23 - a renamed note with a token added is refused # pass 19 # fail 5 
renames      not ok 23 - a renamed note with a token added is refused # pass 23 # fail 1  (after the test was made a real rename; a first version did not trip rename detection)
keepclean    not ok 22 - a flagged note is left out, the clean notes are still saved # pass 23 # fail 1 
taskfile     not ok 11 - the task checkpoint file must exist: none yet, or deleted at the end # pass 23 # fail 1 
skipmark     not ok 13 - skip markers from the caller are refused # pass 23 # fail 1 
publishcode  not ok 15 - watch never publishes code commits # pass 23 # fail 1 
mergediff    not ok 24 - code added in a merge commit is never marked [skip ci] # pass 23 # fail 1 

Not directly tested: pushing the saved SHA rather than HEAD (a race during the push retry wait); the fail-closed branch for an unreadable note (no input triggers it with -z names; the quotednames mutant shows the scan fails closed).
