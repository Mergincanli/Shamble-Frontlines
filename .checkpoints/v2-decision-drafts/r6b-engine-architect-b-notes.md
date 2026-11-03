# r6b engine-architect-b notes (steps in order; started 22:28 Amsterdam, by `TZ=Europe/Amsterdam date`)
- setup: no prior r6b blocks; scratch tree, mk.py, quote_check.py (nine inputs files; HETZNER-ANSWERS excluded as
  superseded), glossary script copied to the scratch tree.
- README: FIX IDX-1 (:21), IDX-2 (:23-25), IDX-3 (:133), IDX-4 (:134), IDX-5 (OQ-0006-22 row after :151), IDX-6 (:145),
  IDX-7 (:62, the runner exception and branch 2), IDX-8 (:66, registered pointer); NOTE IDX-N1.
- SPIKE_PLAN: FIX SPK-1 (:30-45), SPK-2 (:112-119), SPK-3 (:129-154), SPK-4 (:160-162), SPK-5 (:175-176), SPK-6
  (:244-247), SPK-7 (:281-284), SPK-8 (:512-518); NOTE SPK-N1. ENGINE_OVERVIEW: NOTE OVW-N1.
- checks (22:37): apply_fixes dry-run on the real docs: 16 of 16 FIX blocks apply (README 8,
  SPIKE_PLAN 8); scratch tree applied; glossary exact 0 with and without --skip-records (in context IDX 23, SPK 20,
  OVW 12, unchanged); quotes 30 verbatim in the inputs (also with HETZNER-CORRECTION excluded), 1 other is a section
  name ("Machines"); tables keep cell counts, no text under a table; every prose line over 120 in the two docs
  pre-exists (none in a block); counts 116/96, ADR 0006 22/17, 341 untouched.
DONE
