# Advisor verdict: plan revision 4 (stop answers), v2-decision-drafts

Status: DONE (verdict below the notes)

## Notes
- started: reading the stop answers, plan r4, the map
- read: stop answers (verbatim file), plan r4, the map (all 14 tables, counts, 14 questions). Next: spot-check the map's key places, CHECKPOINTS §9, the merge mechanics, the 31 MB location, the saver.
- checked: advisor.md same as main; base 9e7db28 = origin/main; CHECKPOINTS §5 (stop the saver before ready), §9 (fold, delete in last commit, CI runs, ls-tree empty). 31 MB is all under .checkpoints/ (294b3cf skeptic-6 scratch; HEAD tree 51.8 MB of which 46.1 MB .checkpoints). GLOSSARY §8 (lines 744+) is THIS PR's own appended text (engine-architect, synthesis), so the 60 Hz entry is in scope to amend, not a settled input. Next: report's Decisions needed, OQ texts.
- checked: report Decisions needed (SPIKE-ORDER asked "accept the six drafts as the basis for the spikes"; REWIND-CAP recommended "set it from the proof test's p99"); OQ-0001-5 (c) defines the measured honest rewind as "the p99 at P2"; OQ-0003-17 options (a) count / (b) p95 rows; OQ-0003-20 answer makes the soaked set gate PM-0003-18..21 (so those rows keep gating); AB13 design treats a new reviewed operation as part of the data model; ADR 0006 part 8 goal budget is a computed fleet cost, provisional until slice 6 (OQ-0006-19); security baseline is before the first online playtest, so the tick bench (slice 1) falls inside "no token until the baseline". README pointers name OQ ids in most rows. Next: merge mechanics (repo squash setting, PR state), the saver, then rulings.
- checked: repo allows squash (default squash message = COMMIT_MESSAGES, delete_branch_on_merge false); PR #33 draft, same repo, head 829069d (autosave), CI gate+smoke green on 4374a1c; saver running (pid 357, --every 5); ADR 0001 key: MP = his PC, never a measured result, so "Test PC: my own" adds nothing to OQ-0003-1; PM-0001-3 measures loss per direction; PM-0004-71/72 run with the 60 Hz test; OQ-0006-13's interim covers the preview check; README has 95 line pointers, 1 pinned "at 604a497". Not run: the gate (plan review, no edits yet; it runs on the final head). Next: write the verdict block.

---

## VERDICT (verbatim): APPROVE WITH CONDITIONS

Plan revision 4 (record the stop's answers, close the change-set), v2-decision-drafts, PR #33, reviewed at branch tip
829069d (content head 4374a1c, CI gate and smoke green there), base main 9e7db28. Advisor agent, 2026-11-03.

**Why, in one line:** the plan records the answers inside their words, keeps each author to its own text, closes by
CHECKPOINTS §9 and keeps CLAUDE.md's merge rules. But its table misses about 60 of the map's 126 places and lists one
row that doesn't exist (TOY-BOX). It also leaves GLOSSARY §8's 60 Hz entry, which this PR wrote itself, contradicting
two answers. And several answers settle only part of their question: the conditions and rulings below fix all three.

**TL;DR**
- Build from the map, not just the plan's table: every map row gets a change or a "no change" with its reason.
- STATUS-FIELD: (b). The drafts stay "Proposed", signed off by Mustafa as the basis for the spikes. POINTERS: (b).
- GLOSSARY §8's "The 60 Hz margin" entry is this PR's own text: amend it now. Don't just leave a note.
- Mustafa's "once it's approved, squash-merge it" sets the merge method. It is not his go: his go must come after
  the advisor approves the final head.
- One answer can't be closed by a reading, and it blocks the netcode proof test: the reference machines (OQ-0003-1).
  It goes to Mustafa.

### What I checked myself
- `.claude/agents/advisor.md` on this branch = main's (diff: identical). Base 9e7db28 = origin/main.
- The words file against Mustafa's relayed message: identical text. Which answers are his and which are the Advisor's,
  read from the message's own marks.
- The map: all 14 tables, the counts and the 14 questions. I spot-checked its key places at 4374a1c: ADR 0001 part 2
  (118–128), OQ-0001-1 to -6 (1022–1083), part 13 (537–547), PM-0001-3 (1382), the machine key (1266–1280); ADR 0003
  OQ-0003-17 and -20 (914–950), PM-0003-44 and its fail rule (1178, 1186–1188); ADR 0004 (45–47, 1083–1087, PM-0004-71
  and -72); ADR 0005 AB12, AB13 (1332–1348) and the Rollback (2410–2417); ADR 0006 part 8 (315–353), OQ-0006-17 to -20
  (962–1018) and part 13; the 60 Hz restatements in 0002, 0004, 0006 and ENGINE_OVERVIEW; README lines 1–62. All match
  the map.
- The report's "Decisions needed" (lines 105–133): SPIKE-ORDER asked to "accept the six drafts as the basis for the
  spikes"; REWIND-CAP recommended "set it from the proof test's p99".
- GLOSSARY §8 (lines 744 on) was added by this PR (the diff against main; engine-architect-glossary8.md). It is not a
  settled research input.
- The 31 MB sits entirely under `.checkpoints/` (294b3cf, the skeptic-6 scratch). The tree holds 51.8 MB, of which
  46.1 MB is `.checkpoints/`. A squash merge with the folder deleted keeps it all out of main.
- The repo allows squash merges. Its default squash message is COMMIT_MESSAGES, which would paste every autosave
  commit's message into main. delete_branch_on_merge is false. PR #33 is a draft from this repo, not a fork.
- The background saver is running (`checkpoint.mjs watch v2-decision-drafts --every 5`).
- Not run: the gate. This is a plan review and nothing has been edited yet; the gate runs on the final head at the PR
  review.

### Conditions
1. **The map is the work list.** Every map row (RC1–10, H1–24, S1–23, N1–16, RW1–16, O1–23, M1–24, L1–14, T1–4,
   E1–6, C1–11, X1–5, F1–7, Q1–6, A1–5, B1–7, LA1–4) gets a disposition from its author: a FIX block, or "no change:
   <reason>", following the rulings below. fixcheck-e checks the disposition of every row, not only the rows in the
   plan's table. Correct the plan's table as the map's question 14 says:
   - TOY-BOX has no README row.
   - ENGINE_OVERVIEW changes at :3–4, :15 and :117.
   - ADR 0006 needs places for TICK-60-HEADROOM and for OQ-0006-5's development case.
   - README:47 names all three crates.
2. **Attribution follows the message's marks.**
   - Mustafa's answers: SPIKE-ORDER, TICK-60-HEADROOM, REWIND-CAP, SPIKE-MACHINES, LOOK-TEST-INPUTS, TOY-BOX, LATER.
   - The Advisor's: TICK-60-SETUP, NET-PROFILES, OQ-0002-8, OQ-0005-1, OQ-0001-14, ROOM-CRASH, OQ-0003-20, OQ-0005-AB12
     and AB13.
   - OQ-0003-17: the Advisor's, confirmed by Mustafa.

   Quotes are word for word from the inputs file. A reading taken from this verdict is written "(the advisor's
   reading, 2026-11-03)", so it never passes for the deciders' words. The report's "At the stop" section lists every
   such reading, so Mustafa and the Advisor can overturn any of them.
3. **GLOSSARY §8, "The 60 Hz margin" (lines 782–797), is amended in this round** by engine-architect, who wrote it.
   This replaces the plan's NOTE-only rule for this one entry.
   - The definition stays (1 − p99 ÷ 8.33 ms).
   - "It is measured in the netcode proof test, in one run" becomes: one run, an early signal in wave 1, with the
     deciding run at slice 6 on the real sim.
   - The conditions add "the goal budget holds at the rooms per VM measured at 60 Hz", or the entry points to ADR 0001
     part 2 for them.
   - "Depends on" reads: confirmed by Mustafa, 2026-11-03, with his addition.
   - Lines 1–743 keep their bytes (the write guard's prefix check).
   - The older entry at lines 707–712 (map M24) stays a NOTE in the report's open findings.
4. **STATUS-FIELD (b), worded as follows.** "Status: Proposed" and "Signed off by Mustafa: as the basis for the spikes
   (2026-11-03, SPIKE-ORDER (b)); not yet accepted".
   - A draft becomes "Accepted" only when Mustafa signs it off after its spikes report. Marks passing never does it on
     its own.
   - The same wording goes in the headers of ENGINE_OVERVIEW and SPIKE_PLAN, README's Status column and README:24–25.
5. **POINTERS (b).** README's 95 pointers lose their line numbers, "(line 162 at 604a497)" included.
   - Each pointer keeps its file link and names its OQ id, part or section. Where a row has neither an OQ id nor a part,
     the pointer names the section.
   - README:10's sentence about line numbers changes to say so, and the report's risk 2 goes.
   - This is the last README edit, after every other README FIX block.
6. **Keep pass-mark changes minimal.**
   - Add one new row in ADR 0001's 60 Hz group: rooms per VM at 60 Hz, with the goal-budget check (test-engineer).
   - OQ-0005-1's rule becomes a bite on an existing CD-2 whole-set row (PM-0005-4 or -23), not a new row.
   - fixcheck-e recounts by script. Expected: 98 open questions, and 341 pass-mark rows if one row is added.
7. **Close the change-set (§5 and §9).**
   - Stop the background saver (SIGTERM to its `node` process, so its last save runs) before the final report edits.
   - Make the deletion commit by hand, with no "[skip ci]". It is the PR's head when the PR is marked ready.
   - `git ls-tree -r --name-only HEAD -- .checkpoints` prints nothing. Run the gate locally.
   - After that, nothing pushes to the branch except review fixes. A fix that takes more than one step turns the PR
     back into a draft with a new checkpoint (§9.4).
8. **Fold in before deleting.** The appendix carries, byte for byte:
   - Mustafa's words (the inputs file);
   - this verdict;
   - one "At the stop" table: each answer and where it landed, by commit.

   The report also names the last commit that still holds the checkpoint. It says that every SHA it cites is one of
   PR #33's commits: after the squash they are reachable through the PR and the kept branch, not in main's history.
9. **The merge.** Mustafa's message of 2026-11-03 fixes the method (squash only) and the order of work. It is not his
   go: it came before any APPROVE of the final head.
   - **Before the merge:**
     - The advisor has approved the final head's full SHA, in this session or in the PR review comment
       (Mergincanli) that names that SHA.
     - gate and smoke are green on that SHA.
     - Mustafa has said "merge" in this session's chat, about PR #33, after that APPROVE.
   - **The merge itself:**
     - Run `gh pr merge 33 --squash --match-head-commit <that SHA>`.
     - The session writes the subject and body (not the repo's default, which pastes about 100 autosave messages into
       main), ending with the attribution lines.
     - Don't delete the branch.
   - **The PR:**
     - The PR body's first line: "Merge by squash only (Mustafa, 2026-11-03)". It ends with the Claude Code footer.
     - The PR is needs-mustafa (ADRs, escalation category 3).
     - A later push needs the advisor's review again, then his go again.
10. **The report's last two sections** carry the escalations below.
    - "For Mustafa" lists:
      - the Hetzner console check (OQ-0006-1);
      - registering the domain for the WebSocket leg, before the proof test;
      - the reference machines;
      - his go for the merge, after the advisor's APPROVE.
    - STATUS's next step lists what the netcode proof test still waits on: those items, OQ-0006-20, the certificate
      method and the session's SSH key handling. The last two are for the proof test's own plan, with
      security-auditor.
11. **Order.** The netcode proof test's change-set branches from main after PR #33 merges, plan first.
12. **Models and effort (CLAUDE.md).** Opus for every agent. xhigh for the authors' FIX blocks on the ADRs,
    ENGINE_OVERVIEW and the netcode text; high for fixcheck-e, its skeptics and docs-keeper.
END CONDITIONS

### Rulings (the plan's decisions and the mapper's questions)
- **STATUS-FIELD:** (b). Mustafa accepted "the six drafts as the basis for the spikes" (the question he answered).
  Writing "Accepted" would claim more than he said, and accepting an ADR is his call alone (escalation category 3).
- **POINTERS:** (b), on condition 5. The pointer script leaves with the checkpoint, so re-pinned numbers would go stale
  at the next edit.
- **LOOK_V2-§8:** LOOK_V2 is not edited. The question goes into the report as an escalation (below).
- **GLOSSARY-8 (question 1):** amend the 60 Hz entry, this PR's own text (condition 3). M24 stays a NOTE.
- **TICK-60-HEADROOM (question 2):**
  - "The goal budget" is part 8's goal budget as it stands when the call is made: at slice 6, the figure that OQ-0006-19
    fixes. In the wave-1 early signal, the cost is reported beside part 8's provisional figures and decides nothing.
  - "Holds" means: the fleet the goal modes need at the rooms per VM measured at 60 Hz (part 8's formula, same machine,
    N = 48) costs no more than that figure.
  - The new mark lives in ADR 0001's 60 Hz group. ADR 0006 part 8 and spike item 2 point to it, and ADR 0006's "none
    runs at 60 Hz" lines stay true. OQ-0006-19 and OQ-0006-10 stay open, with a pointer.
  - Restatements that say "60 Hz only with the 60 Hz margin" and link §8 (0002:47, 0004:182 and 612, 0006:50) state a
    necessary condition. They may stay once §8 is amended.
  - Every line that says the proof test measures or decides the 60 Hz call changes: ADR 0001 at :11, part 2, the
    budgets row, spike item 3, migration steps 1 and 8, and the rollback; ADR 0004 at 45–47; SPIKE_PLAN item 2.
  - The summary lines (ADR 0001:11, ENGINE_OVERVIEW:15, README N2) state the full rule, or point to part 2.
- **TICK-60-SETUP (question 3):**
  - Slice 6 always runs the final 60 Hz measurement on the real sim, whatever the early signal shows.
  - The early run (OQ-0001-2 (a), with S1's stand-ins) decides nothing. A pass starts no 60 Hz work before slice 6:
    PM-0004-71 and -72 run with each 60 Hz run but decide only at slice 6, and R1-TICK-OQ1 (60 or 64 Hz) is asked only
    after slice 6 passes. A fail closes nothing.
  - Options (c) and (d) stay in the record as weighed.
- **SPIKE-ORDER (question 4):**
  - The 60 Hz run stays a run on the proof test's kept server and harness, in its own VM session after the sim core
    spike. The proof test's "Done when" closes at 30 Hz without it: its tick-rate clause goes, and the 60 Hz run reports
    separately.
  - The boundary spike now uses the sim core spike's sim, so its "stand-in until then" clause may go.
  - The asset research's export and slot-naming part is agent research, not a heavy spike, and it does not run
    alongside one: only the preview hosting check may. It sits between two wave-1 spikes, early enough that its export
    and slot rules reach Mustafa before he makes the models. Recommended: right after the netcode proof test.
- **REWIND-CAP (question 5):**
  - "The honest rewind the proof test measures at P2" is its p99 at P2, as option (c) defines it and as the
    recommendation he answered ("set it from the proof test's p99"). The run's largest rewind is reported beside it.
  - Rounded up to a whole tick at the run's tick rate (as PM-0001-48 does).
  - The one-interval tolerance and part 8's anti-cheat text are unchanged. The value is recorded when the proof test
    reports.
  - The checklist: step 10's 200 and 250 ms comparisons go, since no candidates are left to weigh. Step 7 uses the
    rule. Step 11 stays: it records the "hit after cover" window inside the cap.
  - SPIKE_PLAN item 10's Feel Lab weighing of the cap goes. "Revisit when" keeps his Brave check of the measured cap as
    the trigger for feel.
  - "The smallest input buffer that holds" guides OQ-0001-6, which the proof test still decides: of the options that
    pass its marks (corrections, fairness, the fast-clock bound), take the smallest buffer, with (a) run-on-arrival
    counting as none. The options stay.
  - PM-0001-48's test cap (the run's largest rewind) stays: it judges a different mark.
- **NET-PROFILES (question 6):**
  - "Loss as P2 defines it" means 2 % on each direction, applied independently, as PM-0001-3 already measures. About
    4 % of round trips lose a packet.
  - Part 13 gets one clause saying so. OQ-0001-3 and -16 are answered.
- **SPIKE-MACHINES (question 7):**
  - (a) Sessions get SSH access to the spike VMs. How the key is made and held without a session reading a secret it
    didn't make is for the netcode proof test's plan, with security-auditor and the advisor. The obvious candidate: a
    key pair the session makes for each run, whose public half Mustafa adds when he creates the VM. Anything that
    hands a session a secret goes to Mustafa.
  - (b) "No Hetzner token until the security baseline" covers every VM before the baseline, and the baseline comes
    before the first online playtest. So doing it by hand also covers the tick bench (from slice 1) and the 3-hour
    soaks: OQ-0006-17 is answered. The report notes the bench's hand work (a placeholder of about 20 h a month).
  - (c) LG is a CX33 as written; how many comes from the calibration run. SH stays.
  - (d) "Test PC: my own, in Brave" is MP as ADR 0001 already defines it (what he judges, never a measured result). It
    doesn't stand in for ML, RD or MAC, so OQ-0003-1 is unchanged in substance (escalated below).
  - (e) OQ-0006-5: the WebSocket leg uses development case (i), a trusted certificate on his domain. The issuance
    method is for the proof test's plan, with security-auditor: http-01 or tls-alpn-01 need an open port, dns-01 needs a
    secret, and OQ-0006-16's port plan applies. Production stays the security baseline's.
  - Partly answered: OQ-0006-15 and OQ-0006-20 (VMs only during a run, but no cap). OQ-0006-1 stays open.
- **LOOK-TEST-INPUTS (question 8):**
  - LOOK_V2 is untouched. Blender is recorded as the route for the look test's models and for the research part's
    export.
  - Part 11's list of objects stays; each object may be a blockout at Mustafa's bar.
  - Which look-checklist steps a blockout can judge is for the look test's own plan (graphics-engineer, test-engineer),
    and any visual call is Mustafa's. L12: no checklist edit now.
- **OQ-0003-17 (question 9):** the narrow reading.
  - (a), at most 6 of 600 frames, is §6.9's reading, and the p95 rows are diagnostics for it.
  - PM-0003-18 to -21 keep gating their own needs. OQ-0003-20's answer has the soaked set gate them, which only makes
    sense if they gate.
  - PM-0003-44 is gated on RD at High and reported on ML. Its bite (10 of 600) still bites.
- **OQ-0005-1 (question 10):** the rule binds the kinds the client presents.
  - CD-2's whole-set check fails when an id of a presented kind has no client entry.
  - engine-architect names the presented kinds in CD-2, from AP-2, CD-13 and AB-3 (game-designer confirms the ability
    kinds).
  - Kinds with nothing to present (tuning tables, modes) need no entry and are listed by name.
- **OQ-0005-AB13 (question 11):** "built from data" includes a new reviewed generic operation, as the drafts' model
  already has it (AB-6; "every new operation is a reviewed Rust change").
  - Scripting comes back when one designed ability needs ability-specific code that no operation can express.
  - "Needed often" and "many fail" are no longer triggers: align the rollback, "Revisit when", the frame's table and
    Appendix A.
  - (b), reading the Overwatch talk, stays as a weighed option, and nothing schedules it.
- **ROOM-CRASH (question 12):**
  - "Keep" means written and kept under ADR 0006 part 11's proposed retention. No new period.
  - One word: ADR 0006 part 12's stub record is "marked crashed", as in ADR 0001 part 11 and PM-0001-59.
  - PM-0001-59's tail about option (b) goes.
- **LATER (question 13):**
  - The twelve LATER questions go to the design session after the netcode proof test. The 38 design questions join its
    agenda: deciders unchanged, and Mustafa may trim the list.
  - Three keep their own timing, set by the evidence they need:
    - OQ-0003-16: in the look proof test.
    - OQ-0006-18: before the first model is committed. If one comes before the session, it goes to Mustafa then.
    - OQ-0006-13: before the foundation's client-host token. Until then the preview hosting check runs under B9's
      interim rule (every deploy waits for his approval).
  - The drafts' own entries for these questions are not edited (LA3). README gets one sentence and a marker on each
    row.
- **COUNTS:** a partly answered question stays in the open count, marked "answered in part". OQ-0001-5 stays open,
  since its value is pending. That makes 98 open (condition 6).
- **TOY-BOX:** nothing changes in the nine documents. STATUS and the report record the kit change-set that comes after
  PR #30, together with the checkpoint anti-waste rules.
- **OQ-0002-8:** (a). README names all three crates. **OQ-0001-14:** (a). No need line changes.
- **REASON-COUNT:** Mustafa's "seven usage-limit cuts in three days" is quoted as his words. The report keeps its own
  count, labelled as the checkpoint's.

### Findings (ranked)
1. Completeness: about 60 of the map's 126 places are missing from the plan's table (condition 1).
2. GLOSSARY §8's 60 Hz entry is this PR's own text and contradicts TICK-60-HEADROOM and TICK-60-SETUP (condition 3).
3. "Once it's approved, squash-merge it" could be taken as an advance go. It isn't one (condition 9).
4. The repo's default squash message would carry about 100 autosave messages into main (condition 9).
5. After a squash, the cited SHAs and the deleted notes live only in PR #33's commits, so the branch must be kept and
   the report must say where they are (condition 8).
6. OQ-0003-1 is unchanged by "Test PC: my own" (MP was already his PC), and it blocks the proof test (escalation 1).
7. The plan names no effort for its agents (condition 12).
END RULINGS

### Decisions needed (for the report; none blocks this plan)
- **REF-MACHINES (OQ-0003-1)**
  - The question: which laptop is the minimum (ML), which desktop the recommended (RD), and which Mac (MAC)?
  - Why it matters: about 22 of the netcode proof test's 62 marks are measured on ML or RD. Your PC (MP) only judges
    and is never a measured result, so the proof test can't start without them.
  - Options: (a) name, buy or borrow ML, RD and MAC before the proof test's plan; (b) let your PC stand in as RD for
    the proof test only, which changes MP's "never measured" rule; (c) start the rows that need neither and hold the
    rest.
  - Recommendation: (a) for ML and RD. MAC may follow, for the Safari rows.
- **LOOK_V2-§8**
  - The question: you chose Blender for the look test's models, while LOOK_V2 §8 says "a 3D model in Spline". Does §8
    change?
  - Why it matters: agents read §8 as the production art flow.
  - Options: (a) keep §8, with Blender for the test models only; (b) change §8 to Blender, in a change-set by you and
    the Advisor.
  - Recommendation: (a), until the asset research's export part reports on both tools.
- **SPIKE-COST (OQ-0006-20)**
  - The question: what may the proof test's VMs cost?
  - Why it matters: no cap is set, so each run plan's cost comes to you before you create its VMs.
  - Options: (a) 5 EUR for ADR 0006's own runs; (b) one cap set from the run plan's hours at the hourly prices; (c) 5
    EUR for everything.
  - Recommendation: (b), the test review's.
- **DOMAIN (OQ-0006-15)**
  - The question: which domain, which registrar and which DNS host for the WebSocket leg, and will production use it
    later?
  - Why it matters: the WebSocket leg needs a trusted certificate on a real hostname before the proof test.
  - Options: a fresh domain for the spikes only, or the game's future domain.
  - Recommendation: the game's future domain if you already know it; if not, a cheap spike-only one.

### For Mustafa
1. Check in your Hetzner console that a CCX23 and a CX33 can be ordered in an EU location (OQ-0006-1). This is still
   open.
2. Register the domain for the WebSocket leg before the netcode proof test.
3. Name the reference machines (REF-MACHINES above).
4. Once the advisor has approved PR #33's final head and gate and smoke are green on it, say "merge #33" in this
   session's chat. It merges by squash only.

Nothing in Brave: nothing a player sees changes.
END VERDICT

Status: DONE
