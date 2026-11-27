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
- B1 done: test-list.md steps 6-11 and my T-must 1-14 / T-should 1-12 read.
- B3 done: plants-4b2.py read whole (84 PLANTS + S1-S4 + N1-N6 = 94 lines; 83 defects caught, 11 controls held; caught = exit!=0 AND every needle, controls = exit 0). Run1 vs run2 differ only at P19 (run1: no stamp after P10's refused build; fixed by chaining a clean WASM build). Log checked: S1 exit 0, S2 "left no stamp", S3/S4 "is not the file the built-file check walked" (log 14877-14945); run2 C9 refused only for "no content bundle" (17700), run1 C9 (9100) also for no stamp (confounded, needle present). P15/P16 ran after P10's refused build (no stamp) but the source scan throws first (build.mjs:255): caught on own words. Log holds several appended runs (P1 at 6686 and 15300).
- Note (Low): no clean control in the plant run for boundary replay / sim-wasm / determinism.spec (C18, C19, C21, C29 unpaired); covered by local-checks only. C20-C29 run on a players' file left FOLDed by C19 (restored only at the end): state-dependent, harmless for needles.
- Next: B2 map items to tests.
