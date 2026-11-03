# verifier-fixcheck-e2 notes (unit e2: ADR 0003, 0005, 0006, GLOSSARY)
- 2026-11-03 start: HEAD 59d41e0, base 4374a1c. Next: read inputs, verdict, map; then diff per file.
- Rebuild done: 0003 (test-engineer, graphics-engineer), 0005 (test-engineer, game-designer, engine-architect-a, graphics-engineer), 0006 (test-engineer, infra-engineer), GLOSSARY (engine-architect-a): all blocks applied, all four byte-identical to HEAD 59d41e0. Scratch: verifier-fixcheck-e2-scratch/rebuild/.
- write_guard.sh: GUARD PASS (note: the script itself runs `git fetch -q origin main`; it ran once because the task named it). glossary_check_drafts.py: exact hits 0.
- Next: read the diff per file (GLOSSARY first, then 0003, 0006, 0005), claims table rows.
