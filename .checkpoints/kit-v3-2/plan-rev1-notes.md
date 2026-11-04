# kit-v3-2 plan revision 1 (A2): working notes
Notes so far (the plan body follows when complete):
- read: CHECKPOINTS.md full; CLAUDE.md 1-24,118-170,194-230; art-director.md full; LOOK_V2.md full; STATUS 18-30, 160-172, 185-210, 258-266, 350-370, 498-512; NIGHT_SHIFT.md full; a2-items, a4-spec.
- facts: .gate/ is the established scratch/build folder (gitignored; settings.json:49-50,71; tools/README.md:4; START_HERE:112; render-ab SKILL:8,14; verifier.md:10; rust-reviewer.md:13; perf-auditor.md:11; CLAUDE.md:213 "scratch under .gate/").
- routine guard: pr-advisor writes only .checkpoints/, .gate/ (routine-guard.mjs:292,376); routines never run .gate/ scripts or untracked scripts (469-471); kill refused, TaskStop (448).
- checkpoint.mjs watch stop rules: header 7-10; watch() 185-207: logs "watching ... for N min", "watch stopped: <why>", saves "saved and pushed"/"NOT pushed"/"NOT committed".
- TOY-BOX lines: CLAUDE.md:4, art-director.md:10 (STATUS:353-361). Related "look is not decided" lines: CLAUDE.md:5, 23-24, art-director.md:16, graphics-engineer.md:19 (KIT-ALIGN, not TOY-BOX).
- parallel restatements: changeset SKILL.md:25 (reviewers in parallel); V2_PHASE0_BRIEF:113 (Advisor's doc).
- plan written in full (plan-rev1.md); next: self-check line refs
DONE
