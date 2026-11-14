# U4r-F1 plan: drafter notes

Plan: read the advisor's r2 ruling (INF-R2-1, -3, -4, step 13), r2run (RUN-L1, step 7), c62h (step 4), the r2 findings,
both scripts whole at HEAD, the scratch helper test; then write u4r-f1-plan.md section by section (goal, items, risks,
type, evidence, builder, reviewers, Mustafa, size), a line here per section.

- started: reading sources.
- sources read: advisor-u4r-r2-full.md (INF-R2-1/-3/-4, step 13), r2run-full.md step 7 (RUN-L1), c62h-full.md step 4,
  u4r-review-r2-full.md, both scripts whole (391 and 223 lines at HEAD), the scratch test (72 lines), check.mjs run()
  :1897-1921 (no signal field, out = stdout+stderr), main-run.sh:80 (provenance grep: Compiling|Checking|Documenting).
- facts: no 'Fresh npt-' line in any kept log under .gate/u4r and .gate/u4r-merge (grep); r2.log 0 'Compiling npt-'
  lines, m5.log 91. Spike tree bc90e71e at 71ee13d and e59ece9 (b57488ad is spikes/, the r2run ruling's figure: no
  disagreement); HEAD 0b47e881 (5e563b9's marks.json). Two saver commits (a5ea23e, 7ff3823) landed while drafting.
  /dev/shm is a 16 GiB tmpfs (df), the disk 4.4 GiB free: the argv harness needs tmpfs so every set's projection passes.
- pitfall found: c-u4r-r2.mjs's sh() runs in SPIKE; a `git status -- spikes/netcode-proof` there is cwd-relative and
  would always be empty: the new calls must pass ROOT.
- choices left by the rulings: helper home; non-zero exit; recording stopped; judge stopped; lock header; Fresh lines.
- writing the plan now.
- sections written: 1 goal; 2 items A (INF-R2-1) B (INF-R2-3) C (INF-R2-4) D (RUN-L1) E (whole-as-lost); 3 risks;
  4 type/tier; 5 evidence (node --check, gate, helper test a/b/c + real git, argv harness on /dev/shm with stop cases);
  6 builder/reviews/Mustafa (nothing); 7 size about 100 lines changed (75 runs, 25 r2), item by item; 8 choices Q1-Q6.
- done: u4r-f1-plan.md, 135 lines, sha256 774b952d620c5517…
