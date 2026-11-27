Task: test-engineer review of change-set 4b-2 "The world", round 1, code head c442a806d83dceac6a272b22ad67b7f01de476fa.

## Plan (blocks, in order)
B1. Read test-engineer-testlist.md and test-list.md (steps 6-11): list each T-must / T-should item.
B2. Map each item to the test that holds it (grep the tree at c442a80).
B3. Plants: plants-4b2.txt + plants-4b2-run1.txt + .gate/plants-4b2.log: each caught on its own words, controls held, files put back.
B4. Not-verified claims: step-8 swap within an equal pair; Firefox coverage. Card claims spot-check.
B5. Targeted reads: tests that cannot fail / timing-dependent / missing negatives (collision, replay, content_bundle, testlines, schema test).
B6. A cargo test run in my own target dir (sim/content/devtools) if time allows.
B7. Findings written; DONE.

## Notes
