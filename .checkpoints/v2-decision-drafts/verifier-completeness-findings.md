# Completeness critic (plan step 8): what is missing in the v2 decision drafts

verifier (fresh, xhigh, opus), 2026-11-01. Read at head c05fc15 (branch claude/laughing-lovelace-ox61se), after the fix
commits 09803a9, 03df0ae, 45c957f, 7da03b1, 8eb2f67, c70c6d2 and b375829. Still running when I checked (so marked
"running", not "not met"): the second-round fixes for 0002 (engine-architect), 0005's frame and content (engine-architect),
0005's abilities (game-designer), the pass marks (test-engineer, round 2) and the synthesis (engine-architect, last).
I edited nothing; scripts were run read-only; the gate wrote only its own scratch. My notes:
`verifier-completeness-notes.md`.

States: **met** · **partly** (something is missing; named in the ranked list) · **not met** · **running** (depends on
work still in progress or on a later plan step).

## The checks

| # | Check (against its source) | State | Where, and the evidence |
|---|---|---|---|
| 1a | ENGINE_OVERVIEW: diagram with the parts and the Rust ↔ TypeScript boundary, the data flow per tick and per frame, where each VISION_V2 §6 item lives (brief §3.1) | met (one note) | `docs/ENGINE_OVERVIEW.md` 133 lines (condition 19: about 150 at most). Mermaid diagram of the page, Worker, server, sim and content crates with the per-tick edges; "The Rust ↔ TypeScript boundary" (down, up, types); "The data flow": per tick (4 steps) and per frame; the §6 table. Note: the per-frame flow and the §6 placement sit in prose and a table beside the diagram, not in the diagram itself (missing item 15). |
| 1b | Six drafts on template 0000: context, options and trade-offs, decision and consequences, open questions, what the spike must prove with measurable pass marks (numbers, hardware, method) (brief §3.2) | met | All six carry every section (check 2). Pass-mark tables in all six with the same 7 columns (mark · what it proves · number and statistic · hardware · method and tool · runs and noise · basis): 56, 76, 43, 72, 46, 31 rows. Hardware is named by role (RS, ML, RD, MAC, …); the models of ML, RD and MAC are open (OQ-0003-1) and the RS order is open (OQ-0006-1): honest open questions, listed for Mustafa. |
| 1c | Every claim cites its research file and section (brief §3.2) | partly | `cite_check.py` at head: 0 cited ids that exist nowhere in the research, in all six. Its coarse line-range check flags 78 citation groups (0001 3, 0002 35, 0003 8, 0004 12, 0005 2, 0006 18); only verifier 0004 (row N7: its 11 were script artifacts) and verifier 0003 (its own block check) judged theirs; 0001, 0002, 0005 and 0006's flags are judged nowhere (missing item 9). The verifiers' both-ways row checks found the per-claim citations (citation-off rows went to the fix rounds). |
| 1d | SPIKE_PLAN: every spike (at least the netcode proof test, the look proof test, the boundary spike) with pass marks, place in the order and rough size (brief §3.3) | met | `docs/SPIKE_PLAN.md`: table of 10 spikes plus the foundation's checks and the asset-pipeline research, each with size (S/M/L defined), place (waves), pass-mark ids; §1 netcode proof test (with the research spikes it subsumes, carry-forward 5), §3 boundary spike, §5 look proof test; the asset-pipeline research before the asset spike, the Feel Lab test, PREV-DELETE in §6 (conditions 4, 10). |
| 1e | Loose ends: the five carry-forwards and the critic's items resolved or listed (brief §3.4) | met | `docs/adr/README.md` "Loose-ends register": CF-1 to CF-5; the critic's 12 contradictions (research report: 7 resolved, 1 aligned ABL-PREDICT, 4 escalated MODES-3, BASELINE-WHEN ×2, PREVIEW-WALL; PREV-DELETE) all present; the 28 XT critic ranks (72 ids); LE-A to LE-P. |
| 2 | Each draft has every heading of the plan's shape, none empty; "Hard constraints in CLAUDE.md that change", Migration plan and Rollback filled (condition 13) | met | Headings of `prompts/prompt_common.md` "The draft's shape" all present in all six, none empty (smallest: 0004 Migration plan 13 non-blank lines, 0003 Rollback 11). Hard constraints: 0001:824, 0002:759, 0003:659, 0004:724, 0005:1745, 0006:587 (line numbers at the first read; each names the constraint and SPIKE-SCOPE (a) or LOOK_V2 §8). Inventory-row tables in all six. 0005's four section headings between Decision and Options are its frame by design (condition 11). The author-line form differs (F14, running). |
| 3 | The advisor's 19 conditions | partly | Table below: 13 met, 6 partly or running; nothing in a condition is "not met" in the drafts themselves. The open parts are the report (not written), step 10 (not started), the gate and the 18 inventory rows. |
| 4a | LOOK_V2's `[DECIDED]` and `[DEFAULT]` items each traced in ADR 0003's table (LOOK_V2 §9) | met | LOOK_V2 has 20 `[DECIDED]` items, 10 `[DEFAULT]`, 1 `[OPEN]`. ADR 0003 "How each LOOK_V2 item is achieved" (0003:398–443 at first read): one row per item (§1's four bullets, §2's colour and light in three, §6.5 in three), each with technique, research id or "not researched", what the look proof test shows, owner; the tree style last, nothing decided. The look checklist (PM-0003-17, 30 steps) names the items by section beside legacy. |
| 4b | Mustafa's UPGRADE-SCOPE update, clause by clause, each carried | met | ADR 0005 AB-8 table (0005:904–925): each of his six clauses plus "pick one of three", with data (0005), state (0004 part 18), authority and messages (0001 part 12); telemetry fields (0006 part 12); STATUS carries his words (docs-keeper); appendix §2 quotes them byte-identical to `inputs/UPGRADE-SCOPE-mustafa-2026-10-31.md` (script). It also answers brief §3's "how a pick applies after a switch" (tracks survive switching); no text says so in so many words (missing item 16). |
| 4c | His three open questions listed, with no pick | met | OQ-0005-AB1 carry slots, AB2 looted ammo, AB3 levelling and catch-up, each with options and decider Mustafa; "None of AB1 to AB11 has a pick here"; all three in README "Decisions the drafts ask for". The tension with LOOP_V2 §8 is OQ-0005-AB5 (WEAPON-UNLOCKS). |
| 5 | VISION_V2 §6's ten items each placed in ENGINE_OVERVIEW's table | met | Rows 1–10, each with draft and part; §6.4 `[OPEN]` seats, §6.8 `[LATER]` kill-cams and §6.10 `[LATER]` accounts get "no feature" and what keeps them possible (condition 8). Row 4's leech/marking wording is synthesis row 37 (skeptic: partly; synthesis round running). |
| 6 | Dispositions: 521 of 521, each once | met | Re-run at c05fc15 (`scripts/dispositions.py` on the six drafts, README, SPIKE_PLAN): 521 disposed, 0 missing, 0 more than once, 0 outside the map's owner, 0 unknown; 0001 127, 0002 104, 0003 71, 0004 82, 0005 34, 0006 65, DESIGN 38. `consistency-scripts.txt` itself still holds the 09:02 run (483 of 521, 38 missing): the 521 result is recorded only in the checkpoint line (missing item 12). |
| 7 | Every draft has a verifier verdicts file; claims still pending; claims with no verdict | partly | Verdict files: 0001 (629 rows), 0002 (538), 0003 (440), 0004 (665), 0005 (1,001), 0006 (588), synthesis (61). Every wrong or unsupported verdict had a skeptic (skeptic-1: 0001, 0004, 0006; skeptic-2: 0002, 0003; skeptic-3: 0005, synthesis; all three finished). **No verdict:** every text the fix rounds replaced or added, 269 inserted and 188 deleted lines in the six drafts since 80a27b3 (0001 90, 0002 10, 0003 133, 0004 103, 0005 16, 0006 105 changed lines; new OQ-0001-14 among them), plus the rounds still running (missing item 1). **Pending source checks** (fetchable today until 22:00): 0002's R5-TSLINT-F3, F20, F24, F25 and R5-PREV-F8 (its R6-OBS-F6 and F8 are confirmed at source in verifier 0006's file: close by reference); 0004 row 112 (Rapier's `target-cpu=native`, not load-bearing). **Source unverified but not labelled in the draft** (plan step 7): 0004 rows 28 (R4-NAV-F16, an absence) and 138 (padding bytes uninitialised, R1-RR-OQ6); 0002's R5-CI-F4 second source and R5-TEST-F11 (github.com 403), R5-PREV-F9's 90-day default. **Resolved in fact:** the three "GLOSSARY §8 anchors" pendings (0001 N10, 0004 N4, 0006 N7): my link check finds 242 anchored links in the drafts, synthesis, GLOSSARY, STATUS, ROADMAP and REPO_MAP, 0 broken. |
| 8 | STATUS's carry-forwards resolved or listed; the old ADR-001..003 entries and STATUS:257, :262, :273 placed | partly | Placed: README register rows CF-1..CF-5, OLD-ADR, ADR-001, ADR-002, ADR-003, SEC-MDL-2, CDN-SRI, WALL-CLOCK, each with its draft; STATUS:246–248 says the drafts replace ADR-001..003. Missing: STATUS's own state lines are stale (CF-1 "0001 PASS WITH NOTES, 0006 FAIL…, 0002–0005 running"; CF-3 "consistency pass (not yet run)"; CF-5 "being appended"); STATUS:314, :319, :330 (the old :257, :262, :273) still name only "ADR-001", with no pointer to 0005 CD-2/CD-9, 0002 + 0006 part 13, 0004 part 4 + 0002 part 7; README:208–214 and :333 and 0005:239 still cite STATUS by its 9e7db28 line numbers (0001 and 0003 fixed theirs). The README and 0005 fixes are running; docs-keeper's STATUS re-check is planned (missing item 7). |

## The advisor's 19 conditions

| C | What it asks | State | Where |
|---|---|---|---|
| 1 | Write guard before each commit; gate `node tools/gate.mjs 9e7db28` at step 12 | guard met; gate running | Guard: `scripts/write_guard.sh` (the GLOSSARY check as a byte-prefix test, a deviation in form the report must state). My local run of its checks: only allowed paths since 9e7db28; r1–r6, UNREAD_SOURCES and LEGACY_INVENTORY unchanged; base GLOSSARY a byte prefix. Gate (run by me at c05fc15): [1] syntax PASS 5/5, [2] concatenation PASS, [3] symbols PASS +0 −0, [4] player build PASS (51 guarded, 0 flagged), [5b] names, [5c] layering, [5d] script order PASS; [5a] smoke FAIL only because Playwright's Chromium is not installed in this container (`/opt/pw-browsers/...headless_shell` missing). The checkpoint reports CI green on the fix commits; step 12's own run is still to come. |
| 2 | TICK-60-HEADROOM in ADR 0001: setup, p99 ≤ 6.25 ms, every 60 Hz mark; three lines replaced; "(at N = 54, MODES-3 (b))" | met | 0001 part 2 (0001:124–140 at first read), :161, budget table row "The 60 Hz margin run", PM-0001-39/-40, OQ-0001-1. It replaces four lines, not three (r2:2302 added); the consistency critic says the report must name it for the Advisor (missing item 11). |
| 3 | GLOSSARY §8 appended after line 742: three entries and a note | met | GLOSSARY.md:744–797: heading exact; the foundation and the netcode proof test quoted from brief §2 word for word; the 60 Hz margin with Depends on, Marked by, Retire; the closing note on line 85; base file a byte prefix of today's. |
| 4 | ASSET-RESEARCH (a): flow as given; five items "not researched"; one research unit before the asset spike; look test's plain exports; report lists it under Decisions needed; risk 3 corrected | met in the drafts; report running | 0005 AP and GPU sections, OQ-0005-AS1 to AS6 ("All six are not researched"); SPIKE_PLAN "The asset-pipeline research" before item 9; 0003:836 plain glTF/GLB exports; README "Decisions the drafts ask for" row ASSET-RESEARCH; plan risk 3 corrected. The report's "Decisions needed" entry is still to be written. |
| 5 | CHECKPOINT-AT-STOP: the report folds the checkpoint in and says it is deleted before ready | running | The appendix's opening says the notes are deleted in the last commit before ready; the report itself is not written. |
| 6 | Dispositions across the drafts, README (DESIGN) and SPIKE_PLAN | met | Check 6: 521 of 521, DESIGN 38 in README "Design questions". |
| 7 | Decision-map addendum (a)–(e) | met | Map part j (J1–J6), carried into README "Codes the drafts cite" (I-1 to I-21, seams, gaps, N-codes, J1–J6). Codes still without a home (S11, "part g", "part h") are consistency F12 (running). |
| 8 | Traces: each feature names its §6 item or says its need is elsewhere; README lists those; nothing for `[LATER]`/`[OPEN]` | met; register fixes running | Every Decision part has a Need line (script; the only parts without one are not features: 0003 part 11, 0005 GPU parts 8–9). README "Needs outside VISION_V2 §6". Its gaps (R2-BUD-REC4 missing; roof fade "beside §6.2") are synthesis row 54, upheld (synthesis round running). |
| 9 | The look: one technique proposed per item, the visible difference Mustafa's call; visible values are tuning; LOOK_V2 rules built as stated; rule 6 in every reviewer prompt; art-director reviews 0003, 0005's asset part, SPIKE_PLAN's look test | met in the drafts; reviews running | 0003 part 5.b (screen-space ink proposed, hull measured as the alternative; OQ-0003-2), CL1/CL6/CL7 rows; fragment damage, damage-stage triggers as open design questions (OQ-0004-4, the grenade-fragment item). Step 10 (reviewer prompts with rule 6; art-director) not started. |
| 10 | People-judged spikes: measurable marks plus a numbered Brave checklist; the look checklist names each LOOK_V2 item beside legacy, beside frame-time marks on ML and RD; SPIKE_PLAN places the Feel Lab test and PREV-DELETE | met | 0001 PM-0001-41 to -53 (corrections, extrapolation, hit registration ≥ 99 %, at P2 with stated jitter and loss) and the netcode checklist with "Expected:" lines; 0003 PM-0003-17 and the 30-step look checklist, PM-0003-13 on ML and RD; SPIKE_PLAN items 6 and 10. One gap: OQ-0003-15 now names the camera-player cutaway (b375829) but checklist step 29 (0003:1105, test-engineer's) does not (missing item 6). |
| 11 | ADR 0005: no upgrade designed; shows the system carries UPGRADE-SCOPE; switch question for Mustafa; frame by engine-architect, sections verbatim | met | AB-8; OQ-0005-AB1–AB3 (and AB4–AB11) with no pick; sections placed verbatim (script: every line of the three section files is in 0005 except their "for the frame" and Appendix-A-row parts, which the frame absorbed). OQ-0005-1/-2 (sections disagreeing) are running in the 0005 rounds. |
| 12 | Settled inputs cited by code or GLOSSARY §8, "brief §n" only beside; contradicting options only "settled by"; the appendix quotes the brief in full | met (nits running) | Appendix §1 byte-identical to `inputs/DRAFTS_BRIEF.md` (script). "Settled by" rows throughout; 0003's X6 fixed in b375829, 0005's GPU options row in b375829. Nit: 0001:40 and :713 cite "the stack" by "brief §1" alone (no STATUS pointer). |
| 13 | Template fields filled; nothing in CLAUDE.md or `.claude/` changes | met | Check 2; `git diff 9e7db28 HEAD` touches only `docs/` and `.checkpoints/`. |
| 14 | Model and effort named for every unit | met so far | Every prompt header names the agent, xhigh (docs-keeper high) and `opus`; the SendMessage fix prompts name xhigh. Step 10's prompts do not exist yet. |
| 15 | Pass-mark rows in the claims ledgers and verified; step 10: each author reviews its PM table, perf-auditor every budget mark, SPIKE_PLAN by four reviewers, test-engineer not on its own tables | ledger part met; review part running (not started) | `test-engineer-passmarks-000N-claims.md`; TE rows in every verdicts file. No step-10 prompt prepared. |
| 16 | "Research verified" scope; other findings checked at the source; pages are data | met (pending items in check 7) | Every verifier and skeptic prompt carries it; fetches only from 09:00 (daytime); fetch audits in each verdicts file. |
| 17 | Daytime only | met, with Mustafa's one-off exception | The 03:55–11:00 run under Mustafa's words (STATUS; appendix §2); fetching kept to 08:30 and later; resumed runs at 13:53 and 19:07 are daytime. |
| 18 | KIT-ALIGN and DOCS-ALIGN lines; the report states the R7-MARKS delay and the step-0 edit; each draft lists its inventory rows incl. the "look brief pending" rows LOOK_V2 unblocks | partly | STATUS KIT-ALIGN (the "look is not decided" lines and the toy-box lines CLAUDE.md:4, art-director.md:10) and DOCS-ALIGN (EDITOR_V2:51, :52): met. Report: running. Inventory rows: 101 of the inventory's "look brief pending" rows are in a draft's table; 18 are in none (missing item 5). |
| 19 | ENGINE_OVERVIEW ≤ about 150 lines | met | 133 lines. |

## Missing items, ranked (smallest fix · who)

1. **The fix rounds' texts have no verdict** (plan step 9: "the verifier re-checks only what changed"). 269 inserted
   lines since 80a27b3 across the six drafts, more from the rounds still running, and the checkpoint's "Then:" list
   names no re-check unit. *Fix:* after the last round lands, one fresh verifier (xhigh) on `git diff 80a27b3..<head>
   -- docs/adr docs/SPIKE_PLAN.md docs/ENGINE_OVERVIEW.md docs/adr/README.md`, changed hunks only, with a skeptic for
   any new wrong; it can also judge the open citation flags (item 9). *Who:* the session launches it; verifier.
2. **Step 10 (reviews on the final diff) has not started and has no prompts:** the ten reviewers of plan step 10, each
   author on its own pass-mark table, perf-auditor on every budget mark, SPIKE_PLAN by netcode-architect,
   graphics-engineer, rust-reviewer and test-engineer, rule 6 in every prompt, art-director on 0003, 0005's asset part
   and the look proof test (conditions 9, 15). *Fix:* prepare the prompts now from one template, run ≤ 5 at once after
   item 1. *Who:* the session.
3. **The report is not written** (step 13), and with it conditions 2 (confirm TICK-60-HEADROOM), 4 (ASSET-RESEARCH under
   Decisions needed), 5 (fold in the checkpoint), 18 (the toy-box lines, the R7-MARKS delay, the step-0 LOOK_V2 edit)
   and the plan's Evidence list (ledger counts per draft, skeptic outcomes, consistency and completeness findings with
   their fixes, the script outputs, the gate, the repo-map check, the fetch audits of every verdicts file). Also for
   the report: the write guard's deviation in form; ADR 0001's fourth replaced line (r2:2302) for the Advisor; the
   0001 F8 assignment slip; Mustafa's night exception and the four spend-limit cuts. *Who:* the session (running).
4. **The appendix does not carry the advisor's verdict or the plan's rules**, yet about 66 mentions in the drafts,
   README and SPIKE_PLAN name "plan rule n", "plan step n", "plan revision 3" or the advisor's "condition n"
   (counted by grep, SLICE-ORDER's conditions 1–3 left out), STATUS:118 points to "the checkpoint's
   `advisor-verdicts.md` §1", and 0005:2280 says these inputs "leave the branch with the checkpoint". Consistency F12
   covers the decision map's codes only. *Fix:* append to the appendix §3 the advisor's verdict §1 verbatim and §4 the
   plan's "Settled inputs" (with TICK-60-HEADROOM) and "Rules every author follows" 1–12 verbatim; docs-keeper points
   STATUS:118 there. *Who:* the session; docs-keeper.
5. **18 of the inventory's "look brief pending" rows get no outcome from any draft:** CL5, CC10, GW5, GW6, GW7, GW8,
   GW10, GW11, GN2, GE3, GE4, GE7, GE9, GE10, GE14, GE16, GP8, KT5 (101 others are in a draft's table; GE4 is named in a
   draft but not in a rows table). When the inventory is updated after acceptance, these have no stated mark or
   pending note. *Fix:* one line per row (pending stays, lifted, or which draft) in the owning draft's table, or one
   README LE-H line listing them with their outcome. *Who:* graphics-engineer (CL5, CC10, GW5, GW10, GE3, GE14, GE16,
   GP8, KT5); engine-architect (0005: GW6–GW8, GW11, GE7); rust-reviewer (GN2, GE4, GE9, GE10, their sim halves); or the
   synthesis author in README.
6. **Two pass-mark texts no round covers:** 0003's look checklist step 29 (0003:1105) lacks the camera-player cutaway
   that OQ-0003-15 now names (skeptic-2 X5); 0002's "at most 4 Wasm calls" row (F-2, upheld) waits on engine-architect
   saying what it must read. Neither is in `prompts/fix2-test-engineer.txt`. *Who:* test-engineer (a third, small
   batch), after engine-architect's 0002 round returns.
7. **STATUS is stale** (check 8): CF-1/-3/-5 states; STATUS:314, :319, :330 still say only "ADR-001"; STATUS:118's
   pointer (item 4). *Fix:* docs-keeper's planned re-check, with these four named. The README and 0005:239 line cites
   are in the running synthesis and 0005 rounds. *Who:* docs-keeper (planned); engine-architect (running).
8. **No round for infra-engineer:** consistency F13 names it for 0006:550 ("The proof test's measurement replaces both
   figures": the short form outside ADR 0001). The other short forms in 0006 (PM-0006-15, -17, lines 882 and 884) are
   test-engineer's rows. *Fix:* "The netcode proof test's measurement". *Who:* infra-engineer (one block), or the
   session applies the critic's text with infra-engineer's consent.
9. **Citation-script evidence incomplete:** 78 coarse range flags at head; judged only for 0004 and 0003. *Fix:* the
   re-check verifier of item 1 marks each of 0001's 3, 0002's 35, 0005's 2 and 0006's 18 as real or a multi-file group
   (most are groups that cite two files). *Who:* verifier.
10. **Pending source checks and unlabelled "unverified" claims** (check 7): fetch 0002's R5-TSLINT-F3, F20, F24, F25,
    R5-PREV-F8 and 0004's row 112 today (before 22:00), close R6-OBS-F6/F8 by reference; label 0004 rows 28 and 138 and
    0002's R5-CI-F4 second source, R5-TEST-F11 and R5-PREV-F9's 90-day default "unverified at the source" in the
    drafts (or state in the report why not). *Who:* a short verifier unit; rust-reviewer and engine-architect for labels.
11. **Glossary hits on the synthesis not judged:** the glossary script at head (now including ENGINE_OVERVIEW,
    SPIKE_PLAN, README and §8's three entries) gives 0 exact hits and 537 in-context hits; the critic judged the 489 on
    the six drafts; about 44 in ENGINE_OVERVIEW (11), SPIKE_PLAN (18) and README (15) are judged by nobody. *Who:* the
    item 1 verifier, or the synthesis author.
12. **Script outputs not saved:** `consistency-scripts.txt` still shows the 09:02 run (483 of 521). *Fix:* re-run
    dispositions, citation and glossary scripts after the last round and save them for the report. *Who:* the session.
13. **The gate's smoke check** has not run in this container (Playwright Chromium missing); checks 1–4 and 5b–5d pass
    at c05fc15. The repo-map check fails until the report exists (REPO_MAP lists
    `docs/reports/2026-11-01-v2-decision-drafts.md`). *Who:* the session at step 12 (CI covers 5a).
14. **Small:** 0001:40 and :713 cite "the stack" by "brief §1" alone (add "STATUS, Decisions pending"; netcode-architect);
    the appendix records "I hit my usage limit…" once, dated "2026-10-31 late evening", while the checkpoint has the
    same words from Mustafa on 2026-11-01 at 13:52, and his 19:06 "Try again" is not recorded (the session).
15. **Judgement, optional:** the brief asks for "a diagram showing" the per-frame flow and the §6 placement too; they
    are prose and a table beside the diagram. Either add a per-frame note to the diagram or let the Advisor accept the
    page as is (engine-architect).
16. **Say it once:** no text states that Mustafa's UPGRADE-SCOPE update answered brief §3's "how a pick applies after a
    switch" question (the plan revision 3 says so). One sentence in the report (the session).

## Not verified, and why
- The PR #33 body and its CI runs: not read (no GitHub access asked for; the checkpoint says CI is green).
- The texts of the rounds still running (0002, 0005 frame and abilities, pass marks round 2, synthesis): not read;
  their prompts were read and cover the findings above except items 5, 6 and 8.
- Research content: not re-verified (the verifiers' ground); I re-ran only the scripts and the structural checks.
