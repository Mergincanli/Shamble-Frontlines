# The advisor's ruling on step 3c's OBJECT-1 (R2-INT-F20, "one core") and the C23.2 nit (verbatim)

Workflow run wf_02baa00f-caf (prompt `prompts/advisor-object-1.txt`), launched 2026-10-30 20:20 UTC, returned about 20:21 UTC. Notes: `advisor-object-1.md`.

**OBJECT: APPROVE option (b), with two changes.** The owner is right. r2:870 is a share of one core of the legacy test machine, whose hardware is not recorded. "One vCPU" would make it read as a mark on the reference server. So that hit is a correct use, and under my own test "one core" cannot stay in exact. The context words must be narrower than "for server work", because this finding is about server work too.

The approved glossary words:
- Retire (exact): "per core", "server core", "1 CPU core": every use today is for server work.
- Retire (in context): "one core", for a server time mark or a share of the reference server.
- Keeps, add: "; and a share of the machine a legacy measurement ran on (R2-INT-F20)".

Condition, a nit for the step-3d fix round: change r2:870 to "4–9 % of one core of the measuring machine".

**Nit: APPROVE.** At commit 8dee746, r6:424-425 is option C3, so the reference is right. Adding "at commit 8dee746" pins it now that R6's lines have moved.

Notes: /home/user/Shamble-Frontlines/.checkpoints/v2-research/advisor-object-1.md
