# verifier fixcheck-d1: verdicts on the final fix round of ADR 0001, 0002 and 0006

Base 47eadd9 (before the final round) · HEAD 604a497 (branch claude/laughing-lovelace-ox61se) · research at 9e7db28
(r1–r6 unchanged at HEAD) · no web fetch (after fetch hours; web facts from verifier-fixcheck-d0-verdicts.md, "d0 #n").
Verdicts: confirmed · wrong (source quoted) · unsupported · citation off (right one given) · unverified (needs a source
d0 did not read). Line numbers are HEAD's.

## 0. Provenance of the changed text (done first)

- Rebuilt each doc from 47eadd9 by applying the fix files' FIX blocks in commit order (script
  verifier-fixcheck-d1-scratch/recon.py) and diffed with HEAD. ADR 0001 (netcode-architect final + b; test-engineer A
  final, b, c, d) and ADR 0002 (engine-architect 0002 final; test-engineer A final, c): identical to HEAD, 0 lines
  differ. ADR 0006 (infra-engineer final, b, c; test-engineer B final, b): HEAD = the rebuild minus test-engineer B's
  original OQ-0006-20 block, which was held (test-engineer-B-held-for-authors.md) and replaced by infra-engineer c's
  adapted block. So every changed line at HEAD comes from a FIX block in the named files; no unlogged session edit.
- Every BEFORE occurs once in its doc when its turn comes (0 failures).
- Glossary check (scripts/glossary_check_drafts.py, HEAD working tree = HEAD): exact hits 0 in all nine documents; the
  "worst case (48 players …)" pattern: no hit in the changed text (0006:1184 "a budget takes the worst case" is plain
  English, not the stress scene's retired phrase); "headroom" in changed text: once, 0002:1286 "N = 64 labelled
  headroom", the GLOSSARY's own meaning (a run above the largest mode's N).
- A sentence directly under a table (scratch undertable.py, base and HEAD): none in the three docs (the one hit,
  0001:567, is inside the components diagram's code block, as at base).

