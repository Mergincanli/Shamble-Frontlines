Task: completeness critic on plan revision 2 (b6c20be) of foundation-toolchain, checking gaps the three fold checkers (A, B, C) left.
Plan (blocks, in order):
1. Read advisor-plan-rev1.md in full; list each condition, F-item, note, ruling; map to checker coverage.
2. Read the plan revision 2 sections for untested items (F-8, F-9 parts, rulings Q2.2-*, notes, E-list claims build/Mustafa rely on).
3. Spot-check the gaps on main / in sources.
4. Write gap checks G1.. and other findings. DONE.
- B1 done: conditions/rulings mapped. Untested by A/B/C: three-changes ruling, globs-to-2.5, per-edit ask note (K1), Q2.2-a register entry, build start, Part 11, agent concurrency, M1 retarget. Next B2.
- B2 done: plan rev2 read in full (494 lines). Candidate gaps: per-edit ask note vs K2a "covered" (session file); Part 11 omits register go; E2 "only lo" vs 4.3 "does not rest on it"; 1.3 NIGHT_SHIFT row cites CHECKPOINTS §10 for guard refusing cargo; guard on `git worktree add --detach`; CLAUDE.md :45-50; rev1->rev2 removed lines outside C's list. Next B3.
- B3a: session file: his prompt says 'ask Mustafa whether his FREEZE-FDN covers each' kit edit; K2a marked covered without asking (advisor sanctioned via cond 1). CHECKPOINTS §10 :214 supports 1.3 NIGHT_SHIFT row (holds). CLAUDE.md:44 is the heading on 2d22b69, not :45 -> check.
- B3b: rev1->rev2 removed lines scanned; nothing beyond C's list except rev1 1.3 advisor.md:323 'before 2.2 merges' narrowed to 'before K2a/K2b written' (consistent with cond 5). CLAUDE.md:45 heading confirmed. Next: command -v multi-arg, E10 test files.
- B3c: Q2.2-a was set up as advisor-ruled with register on his go (828a3e0 plan :417-420, :554-555): rev2 consistent; only Part 11 omits the register go (Low). E10 test files exist on main. Next: verify B's §8 claim, then write.
- B3 done: §8 (CHECKPOINTS.md:190-193 on 2d22b69) is a branch worktree `git worktree add .gate/<slug>-wt <branch>` + saver there: B's C2b evidence holds.
Findings (B4, written):
G1 holds: three-changes ruling kept (Part 2 .gitignore row plan.md:~118; 1.4 lint tools to 2.3; 4.1 own file, gate.yml byte-identical, E5).
G2 holds: globs to 2.5 kept (Part 6 bullet; Decisions needed 'Not asked now').
G3 unclear/Low: his prompt (session file) says ask whether FREEZE-FDN covers EACH kit edit; K2a treated as covered without asking (advisor cond 1 sanctions); Decisions needed 'Already answered' does not say K2a is taken as covered.
G4 holds w/ Low: Q2.2-a register entry 'his go' in 4.5, step 6, Part 9; Part 11 omits it. Restructure plan 828a3e0 :417-420/:554-555 set it as advisor-ruled.
G5 holds: build needs his start (In one screen, Part 11).
G6 fails Low: E2 pass 'ip -o link shows only lo (up)' vs 4.3 'job does not rest on it'; kernel fallback tunnel devices (sit0/tunl0/ip6tnl0) appear in new netns when those modules are loaded (net.core.fb_tunnels_only_for_init_net=0 default) -> letter of E2 could fail; unverified (no checker, advisor called it INFERRED).
G7 holds: 1.3 NIGHT_SHIFT row's CHECKPOINTS §10 citation (:214 'cannot run cargo ... the guard refuses both').
G8 holds: E10's three test files exist on 2d22b69.
G9 holds: rev1->rev2 removed lines: nothing dropped beyond C's list; 1.3 advisor.md:323 row narrowed from 'before 2.2 merges' to step 5 check before K2a/K2b (consistent with cond 5).
G10 holds: CLAUDE.md:45 heading, :52, :21 on 2d22b69 (grep).
DONE
