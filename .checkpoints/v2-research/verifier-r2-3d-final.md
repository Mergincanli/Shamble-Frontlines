# verifier-r2-3d-final: step 3d final check of R2 (netcode), only what the fix round changed (diff f67245e..2382ea2 -- docs/research/r2-netcode.md). Task: prompts/verifier-r2-3d-final.txt
Started 2026-10-31. Notes saved as I go.

## Progress
- [ ] read inputs (first-pass result, skeptic, fix messages, owner replies, diff, hits tsv)
- [x] inputs read: verifier-r2-3d-result.md, skeptic result, prompts/r2-3d-fixes.txt + -2.txt, owner replies (netcode-architect-r2-consistency-result.md addenda), diff f67245e..2382ea2 (59 changed lines at head; +59 -35). Working tree docs == 2382ea2 == HEAD.
- [x] glossary script re-run on 2382ea2 (.gate/r2-3d-final/): R2 exact 0, in context 201 (was 198), header links GLOSSARY; changed_line_hits.py -> 32 hits, IDENTICAL to scripts/glossary-hits-3d-final-r2.tsv.
