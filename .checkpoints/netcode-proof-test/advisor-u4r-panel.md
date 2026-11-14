# Advisor: U4r review panel confirmation (brief section 12 item 7)

Started 2026-11-14. Seat: advisor agent (day session, workflow step). Read main's advisor.md (origin/main, 335 lines).

## Plan
1. Read brief section 12 item 7 (u4r-brief.md :1895-1900). DONE
2. Read the plan's review lines it cites: u4-build-plan.md :803, :886-887, :930-931 (and neighbours).
3. Read U4R-Q6 (a) and U4R-Q13 (a) rulings (advisor-u4r-brief.md / brief section 10) and carries 63, 76, 84, 85.
4. Read the four build rulings (advisor-u4r-build1..4-full.md): every main-session item naming a reviewer or a review read.
5. Check the facts: base 614823c, worktree HEAD 2fbec77, patch sha256, snap1 worktree, counts logs.
6. Build the panel: seats, types, review lists with sources; two batches; lacks/excess; what every seat is told.
7. Write result blocks here, then return.

## Blocks

### B1: the sources read (steps 1-3)
- Brief :1895-1900 (section 12 item 7): netcode-architect, test-engineer, engine-architect (public items: plan
  :930-931), rust-reviewer (npt-server and npt-log edits: U4R-Q6 (a), U4R-Q13 (a)); plus security-auditor (carry 76's
  session boundary, carry 85's refusals, carry 63's two refusals: U4R-Q13 (a); the wall-clock read in server code:
  U4R-Q6 (a)) and infra-engineer (carry 84's time bound, the `wasm` step's time against its budget: no CI job runs
  check.mjs); xhigh; at most three at a time, reviewers included (plan :886-887); two batches; advisor confirms.
- Plan u4-build-plan.md :803: "Reviewers: netcode-architect, test-engineer, engine-architect, rust-reviewer."
  :886-887: reviewers count toward the 3-agent limit, xhigh (condition 10 (c)). :930-931: public items (A19),
  engine-architect checks them; check.mjs does not check narrow surfaces. :932: one review round per sub-unit on its
  final diff, then a short report to the advisor.
- Brief check advisor-u4r-brief.md :377-381: panel confirmed (same six; engine-architect's public items include
  `Decoder::with_bite`), "at xhigh, at most three agents at a time with the builder counted, so two batches of three,
  one round." U4R-Q6 (a) :216: "rust-reviewer and engine-architect read them" (the npt-server wallclock-bite items).
  U4R-Q13 (a) :286-287: "security-auditor reads them (the seat that raised FU-3 and FU-7)" (carry 63's refusals).
- Carries (u4s-carries.md): 63 :317-322 (read_log duplicate key; check_header control chars); 76 :359-367 (Kick on
  replacement, dropped sessions; two tests); 84 :402-416 (shared 300 s deadline; stopped call takes its processes;
  two self-tests); 85 :417-440 (deps step refusals (a)-(e), loader.mjs header).

### B2: ruling 1 (advisor-u4r-build1-full.md) items naming a review
- Point 1 main session (:14): "The review panel (section 12 item 7): infra-engineer reads det-u4.mjs and wasmStep's two
  options (carry 84's seat), and engine-architect counts the check.mjs changes (2.7)."
- Main session's own reads in U4r's review (not seats): point 2 (:21, r4's wording and counts), point 6 (:49, the dump
  reader's test), point 8 (:63, D13 as a fact), point 9 (:70, the labels; SS-P0-s4's rewinds_refused). Point 4 (:35):
  the main session runs the judge on .gate/u4r/whole-as-lost/ (after the review, before the counted run).
- No other seat named in points 5, 7, 10 or the further findings (grep of :141-170: none).

### B3: rulings 2-4, main-session items naming a review read (no ruling names a new seat)
- R2 (advisor-u4r-build2-full.md): P1 :24 depth lines in runs.txt; P2 :31 runs.txt's build and recording lines; P4 :62
  both tests (radius pin None fails; late-command exact asserts); P5 :70 "any review of copy 3's work names its own
  snapshot" and the contract.rs design; P33 :52 builder logs are builder evidence only, labelled.
- R3 (advisor-u4r-build3-full.md): P3 :29 item 5 the two tick16 runs' verdicts; P5 :43 r6's assertion; P6 :50 item 3
  contract.rs against its four points; P7 :57 scenario.rs:1034-1049 and mirror.rs:396-414; P8 :64 mirror.rs's
  last_acked keep and the bitten run's 0 breaches. P6 also accepts `contract` and `precheck` as subcommands beyond
  2.7's list, named in the report's public-items line (engine-architect's check, plan :930-931).
- R4 (advisor-u4r-build4-full.md): P1 :12 item 5 tick16d's logs and c-u4r-r2.mjs's amended expect text; P2 :21 item 3
  contract.rs:248-301 and the new assertion; P3 :28 item 2 c-u4r-runs.mjs's live lines against ruling 3 points 1, 3.
- These are the main session's own reads; I place each also in the matching seat (B5), and they stay the main
  session's.

### B4: facts checked (read only)
- wip/u4r.patch sha256 572c4f69435d... (equal at b084998: `git show b084998:<path> | sha256sum`; last changed in
  8d6025e); 26 files, 13 modified, 13 new, all under spikes/netcode-proof.
- .gate/u4r-snap1 HEAD 2fbec77, status 26 entries; worktree agent-a11e2354169f39acf HEAD 2fbec77, 26 entries under
  spikes; cmp of all 26 files snap1 vs worktree: 0 differ. 614823c is an ancestor of 2fbec77, `git diff --quiet 614823c
  2fbec77 -- spikes` exit 0 (only .checkpoints/ differ). Main folder (HEAD faa030b): spikes/ equal to 2fbec77, no
  working-tree change.
- c5-end1.log :168-179 and c5-end2.log :169-179: all 11 steps pass; test 526 and 603 passed, 0 failed; self 76 of 76;
  wasm 39.6 s and 41.9 s (three synthetic recordings). det-u4.log:24 PASS det-u4 200.5 s (two runner recordings,
  bites half off).
- No .github/workflows file names spikes or check.mjs (grep exit 1): "no CI job runs check.mjs" holds.
- .claude/agents/ on the branch equals origin/main's (diff empty).
- Stamps: brief 59d570b8a73f (equal to the facts), plan 1e631c55f4c4, carries c74e52553bd8, rulings 1-4 476121ab65e6,
  f4f5ac925633, 78ebb5af7d48, 9d49b484cd3f, brief check 35f281bfc11c.
- No builder report file found (u4r-build.md :1539-1541: "Next: the report (section 6's return list)").
- Deviation, mine: I ran `git fetch -q origin main` once (outside my read-only list). origin/main did not move (its
  reflog's newest entry is 2026-11-13 17:20, 4d21c1a); at most .git/FETCH_HEAD was rewritten. No file of the work
  changed.

### B5: the panel (confirmed, make-up unchanged) -- see the returned result for the full lists
- Batch 1: netcode-architect, test-engineer, security-auditor. Batch 2: engine-architect, rust-reviewer,
  infra-engineer. One round on 572c4f69, then one fix pass.
- Lacks: none. Excess: none. My placements within seats are named as mine.

Status: DONE (result returned by StructuredOutput).
