# Advisor verdict: plan revision 6 (Mustafa's Hetzner console check), v2-decision-drafts

Status: DONE (verdict below the notes)

## Notes
- 18:32 started: no r6 verdict existed. Read plan r6, HETZNER-CHECK words file, verdict r5 (cond. 8, COUNTS 115/97, 0006 21/18), verdict r4 (SPIKE-MACHINES (c) "LG is a CX33 as written"; cond. 7-9). Branch tip 898ba05 (autosave). Next: his earlier words files, then grep CX33 over the nine docs.
- 18:34 grep CX33 over the nine docs: 0001 x4 (1293, 1354, 1356, 1527=PM-0001-63), 0004 x3 (1032, 1117=PM-0004-1, 1120=PM-0004-4), 0006 x27, README x1 (133), SPIKE_PLAN x3 (32, 33, 129), 0002/0003/0005/ENGINE_OVERVIEW 0. Plan's map misses as listed places: 0006:1268 (PM-0006-12's machine "one throwaway CX33 per offered location": the ping-target role, a role the plan doesn't name) and 0006:1274 (PM-0006-18's planning price "0.0144 EUR (CX33) an hour"); both routed to test-engineer but not in the map. GLOSSARY:227, :605 base text (CX33).
- R6-HOST-REC2 (r6:645-655): "Arm (CAX) stays an option only after R1-DET-REC6's matrix includes it" (server CPU); "Shared CX33 is 9.6 times cheaper". F13: verifier's re-read "NBG1 HEL1" (no FSN1) for CX33, CAX21, CCX23... unverified. F15: CAX21 10.99 (0.0176), CPX32 35.99 (0.0577), excl. VAT.
- 18:33 COURSE CORRECTION from the session: HETZNER-CORRECTION file read (his words: "the game server is a CCX23 for development and "does it work" runs. The CCX23 (dedicated) is used only for the measurement runs: tick timing, the 60 Hz margin and rooms per VM. Each run's plan says which server it needs."). Sentence 1 contradicts sentence 2 (session asks him). Screenshot (session's description): order summary "Nuremberg" (his note says Falkenstein; session asks). Session's 86.49 x 1.21 = 104.65 is its arithmetic, matches the TOTAL incl. IPv4 line, VAT rate not on the page: never written as fact.
- Key insight: if the location turns out to be Nuremberg, his CX33 finding ("in Falkenstein") may not hold there, so SH/MV/staging may stay CX33 (research's x86 pick): HZ-SH depends on the location answer. Ask him in the same question.
- 18:35 HIS ANSWERS (HETZNER-ANSWERS file): Q1 dev/"does it work" server = "CCX13 (Recommended)" (option text, session's wording he chose: "the same x86 server build as the CCX23, so no second build target for the room server"); Q2 "Nuremberg (nbg1)" (option text: "whether the CAX21 (or another client VM) can be ordered there is then checked in the proof test's plan"). Plan addendum read (5 points). Nothing else pending from him.
- Consequence for HZ-SH: his chosen option's reason ("no second build target for the room server") rules out a CAX21 as SH (an Arm room server). His "used only for the measurement runs" limits what the CCX23 runs, not what else may be measured: SH not excluded by his words.
- ADR 0001 rows with LG in the machine column (script): PM-0001-1, -2, -3, -14, -17, -18, -19, -20, -23, -24, -25, -26, -59, -27, -31, -37, -38, -63, -42, -61, -48, -49, -54. Candidates where an LG client's own computation is compared with the server's: -54 (flight landing), -48 (shooter's view vs server's rewind), -17/-18/-19 (decoded state hash; integer if quantised), -42/-61 (LG = load only, ML Chrome measured).
- 18:45 ADR 0004: part 7 (289-290) "native AArch64 joins the native-against-Wasm hash test before ADR 0006 may pick ARM hosts"; PM-0004-26 (1142) A64 "only before ADR 0006 may pick ARM hosts"; PM-0004-73 (1178) trajectory bit-identical Wasm vs native, CI only (x86 native + Wasm): aarch64 identity of the trajectory fn is unproven. Key 1032 code **CX33** (reported, never gated); A64 key 1042. ADR 0002:626-627 "No arm64 leg ... the first fleet is x86-64 ... before ADR 0006 may pick Arm hosts"; :935 revisit "ADR 0006 picks Arm hosts"; R5-CI-OQ7 row 1587. R5-TEST-REC5 (r5:1475-1481): bots headless Rust clients; desyncs "from REC2's hashes on sampled clients".
- PM-0001-48 (1538): LG's scripted shooter's view hit test (client-side) vs server registration: a client float computation compared (percent marks). PM-0001-54 (1546): landing vs server's, 0 desyncs over 1,000 throws, "L1; L2 with LG". -17/-18/-19: hashes of quantised state (author confirms no float work; -17's bite names "a client quantiser").
- Rulings forming: HZ-SH x86 only (CX33 if orderable in nbg1, else SH out, K3 waits), never CAX21 as SH; staging x86 (Arm staging = Arm host, PM-0004-26 first); OQ-0006-1 answered in part (client VM order check moved to the proof test's plan: r4 COUNTS precedent, like OQ-0003-1), OQ-0006-2 answered => expect 96 open, 0006 17 open; verify by script.
- 18:41 more places the plan's map misses (not "CX33" lines, but OQ-0006-1/-2 and "every server mark" places the answers change): 0006:244-251 (part 5: "every native server mark is measured on one vCPU of it"; location "nbg1 or hel1 ... picked by a round-trip measurement"), :373 (the PLAYTEST SOAK's bot clients "on a CX33 ... as spike 2's load generators": LG beyond the spike, i.e. `devtools` (ADR 0002:194) built for aarch64 after the spikes, and the soak's desyncs), :634, :715, :753 ("every server mark ... on one named machine"), :1133, :1300, :1364-1370, :1446-1448, :1499, Appendix A :1511-1517 (R6-HOST-OQ1/OQ2 "open: OQ-0006-1/-2"); SPIKE_PLAN:176 ("it feeds OQ-0006-2"), :281 ("RS orderable (OQ-0006-1)"), :513. ADR 0001:756 failure row stays.
- "development server" already means the local content-watching server in ADR 0005 (:411, :417, :744, :2379) and ADR 0002:945 (Vite's dev server): the CCX13 needs its own code, not that name. "DS" is unused in the nine docs (grep -w).
- ADR 0002:519-528: the 3-hour playtest soak runs on RS (tick times count), bots on a VM beside it (0006:373).
- 18:43 checked: .claude/agents/advisor.md = origin/main's (no diff); origin/main 9e7db28 = base; the nine docs at 898ba05 = working tree (no diff); ADR 0006 has 21 OQ entries. README now: 115/97, 0006 21/18. Composing the verdict.

---

## VERDICT (verbatim): APPROVE WITH CONDITIONS

Plan revision 6 with its addendum (Mustafa's Hetzner check, his correction, his two answers), v2-decision-drafts,
PR #33 (draft), branch tip 898ba05 (content as 21ea733), base main 9e7db28. Advisor agent, 2026-11-03, about 18:45.

**Why, in one line:** his words settle the load generator, the two game servers and the location, and the plan keeps
every author on its own text without a re-check round. But four of its recommendations go further than the evidence:
HZ-SH (a) and HZ-STAGING (a) would put the room server on Arm, HZ-OQ1 (a) closes a question that still has an order
check left, and ADR 0001's rows can't rely on aarch64 agreeing with x86-64. Its map also misses about 20 places that
the correction and the location answer change. The conditions below fix all of this inside the one round.

**TL;DR**
- Two game servers. RS (the CCX23) runs only his three kinds of measurement runs. A new code, DS (the CCX13), covers
  development and "does it work" runs. Each run's plan names its server. Rows keep their machine columns.
- Location: Nuremberg (`nbg1`). OQ-0006-2 is answered. OQ-0006-1 is answered in part: the client VM's order check
  moves to the proof test's plan. Counts: 115 numbered, 96 open (ADR 0006: 21 and 17). Pass marks stay at 341.
- SH stays x86 only: a CX33 if it can be ordered in Nuremberg, otherwise SH leaves the proof test. Never a CAX21.
  Staging stays x86 too.
- An Arm load generator (LG) gets one rule in ADR 0001: no pass mark is judged on aarch64 agreeing with x86-64. A
  mismatch seen only on LG is replayed on x86-64 before it counts.
- Nothing new for Mustafa now. All nine decisions are mine, or already settled by his words.

### What I checked myself
- The six inputs files. HETZNER-CHECK, HETZNER-CORRECTION and HETZNER-ANSWERS were read in full. In HETZNER-ANSWERS,
  the option texts are the session's wording; his words there are only his choices. The screenshot description is
  the session's.
- `grep -n CX33` over the nine documents: 0001 ×4 (1293, 1354, 1356, 1527 = PM-0001-63), 0004 ×3 (1032, PM-0004-1,
  PM-0004-4), 0006 ×27, README ×1 (133), SPIKE_PLAN ×3 (32, 33, 129); 0002, 0003, 0005 and ENGINE_OVERVIEW ×0. Two of
  them are routed to test-engineer but missing from the plan's map: 0006:1268 (PM-0006-12's "one throwaway CX33 per
  offered location") and 0006:1274 (PM-0006-18's "0.0144 EUR (CX33) an hour").
- The other places the answers change, found by grep: 0006:244–251 (part 5: "every native server mark is measured on
  one vCPU of it"; the location "picked by a round-trip measurement"), :373 (the playtest soak's bot clients "on a
  CX33 … as spike 2's load generators"), :634, :715, :753, :1133–1134, :1300, :1364–1370, :1446–1448, :1499 and
  Appendix A :1511–1517; SPIKE_PLAN :176 ("it feeds OQ-0006-2"), :281 ("RS orderable (OQ-0006-1)") and :513.
- Research:
  - R6-HOST-F13: the verifier's "NBG1 HEL1" note.
  - R6-HOST-F15: CAX21 10.99 (0.0176), CCX13 43.49 (0.0697), CCX23 86.49 (0.1386), CX33 8.99 (0.0144), CPX32 35.99,
    all excluding VAT.
  - R6-HOST-REC2 (r6:650–652): "Arm (CAX) stays an option only after R1-DET-REC6's matrix includes it".
  - R5-TEST-REC5 (r5:1475–1481): the bots are headless Rust clients; desyncs come "from REC2's hashes on sampled
    clients".
- ADR 0004:
  - Part 7 (289–290): AArch64 joins the hash test "before ADR 0006 may pick ARM hosts".
  - PM-0004-26 (A64, "only before ADR 0006 may pick ARM hosts").
  - PM-0004-73: the trajectory function is bit-identical, checked on CI native (x86) and Wasm only. So aarch64 identity
    is unproven.
  - The CX33 key (1032).
- ADR 0002: 626–627 ("No arm64 leg … the first fleet is x86-64"), 935, R5-CI-OQ7 (1587). The soak runs on RS with its
  bots beside it (519–528). `devtools` holds the soak's headless clients (194).
- ADR 0001:
  - 23 rows have LG in the machine column (script).
  - PM-0001-54 compares each landing with the server's. PM-0001-48 compares the LG shooter's view with the server's
    registration.
  - PM-0001-17, -18 and -19 compare hashes of quantised state.
  - The RS key gives the transport three vCPUs (1350–1354).
- "Development server" already means the local, content-watching server in ADR 0005 (:411, :417, :744, :2379) and
  Vite's dev server in ADR 0002 (:945). "DS" is unused in the nine documents.
- `.claude/agents/advisor.md` is identical to main's. Base 9e7db28 = origin/main. The drafts at 898ba05 match the
  working tree.
- Not run: the gate. This is a plan review and nothing has been edited; the gate runs at my final-head review. Not
  checked: anything in Hetzner's console. What can be ordered is his check, and the rest moves to the proof test's
  plan.

### Rulings (who decides: all nine are mine or already settled by his words; none escalates)
- **HZ-SH: neither (a) nor (d). SH stays x86 only.** SH is a CX33 if the proof test's plan finds it orderable in
  Nuremberg. Otherwise SH leaves the proof test, and K3 waits for a later measurement of a shared x86 plan. Never a
  CAX21:
  - an Arm room server is an Arm host (ADR 0004 part 7, PM-0004-26 first; R6-HOST-REC2);
  - the option he chose keeps "no second build target for the room server" (the session's wording).

  His "used only for the measurement runs" limits what the CCX23 does. It doesn't drop SH (the advisor's reading,
  2026-11-03). Mine to decide: it is an evidence choice, and its hours fall under the cap he OKs for each run.
- **HZ-MV: (b), with limits.** A small VM in Nuremberg running Prometheus only, never RS. The proof test's plan names
  it after its order check. The drafts name no CX33 or CAX21 for it as a fact, and make no claim about Prometheus's
  builds without a source. Mine.
- **HZ-STAGING: not (a)'s CAX21.** Staging and the lobby's placeholder keep "proposed a CX33", with "if it can be
  ordered in Nuremberg (Mustafa found it not orderable in Falkenstein, 2026-11-03); otherwise an x86 plan chosen before
  slice 3". An Arm staging server is an Arm host, and part 7 asks for PM-0004-26 first (the advisor's reading,
  2026-11-03).
  - The budgets and PM-0001-63's formula keep 8.99, labelled as the CX33's price (R6-HOST-F15), a placeholder. No
    21.98 lean case.
  - No new open question (115 stays).
  - The lobby's "for example a CX33" stays an example.
  - Mine now. The staging server's monthly plan and price go to Mustafa with slice 3's plan (money, later).
- **HZ-LOCATION: (a). His answer, OQ-0006-2 answered.** Label: "Answered by Mustafa (2026-11-03): Nuremberg (`nbg1`),
  the option he chose". No quotation marks: the option text is the session's.
  - PM-0006-12 becomes a check, reported against VISION_V2 §4's "about 100 ms", and decides nothing. A surprise goes to
    Mustafa.
  - "Falkenstein" appears only as his words about what his check found there.
- **HZ-OQ1: (b), answered in part.** The answered part: option (a), the reference server stands ("I checked it can be
  ordered there"). Still open: whether the client VM ("the CAX21 (or another client VM)", his chosen option), SH's CX33
  and MV can be ordered in Nuremberg. That is checked in the proof test's plan.
  - This follows r4's COUNTS rule and r5's OQ-0003-1 precedent.
  - It no longer blocks the drafts or the start of the proof test's plan.
  - R6-HOST-OQ4 (the root servers, read before the goal budget is fixed) and OQ7 (the IPv4 price) stay.
- **HZ-PRICE: approved, widened.**
  - The drafts use only R6-HOST-F15's prices excluding VAT: CCX23 0.1386, CCX13 0.0697, CAX21 0.0176, CX33 0.0144.
  - His "about €0.020/h incl. VAT" is quoted as his, once.
  - The screenshot's figures and the option text's "€0.083/h" are the session's: never quoted as his.
  - No VAT rate: the session's 86.49 × 1.21 stays out of the docs.
- **HZ-RECORD: approved.**
  - The weighed options (G2, OQ-0006-20's options, OQ-0006-17's "with OQ-0006-1's console check"), the examples and
    Appendix A's record rows keep their words, with at most one pointer.
  - Appendix A's status cells ("open: OQ-0006-1", "open: OQ-0006-2", 1511–1517) are status, not record: they change
    to "answered in part" and "answered".
- **HZ-COUNTS: corrected.**
  - 115 numbered, 96 open; ADR 0006: 21 numbered, 17 open (-2 answered, -1 answered in part). The plan's 95 and 16 are
    one too few.
  - Pass marks: 341 (no row added or dropped; rows change their machines).
- **HZ-VERIFY: approved, with condition 11.** His stop holds: no verifier, no skeptic. The quote script runs against
  all six inputs files, not four.
END RULINGS

### Conditions
1. **The work list comes from a script, not the plan's map** (as r4's condition 1).
   - The search covers the nine documents for: CX33; OQ-0006-1; OQ-0006-2; R6-HOST-OQ1; R6-HOST-OQ2;
     nbg1|hel1|fsn1|Falkenstein|Nuremberg|Helsinki; and "every server row|every native server mark|every server mark".
   - Each hit gets a disposition from its author: a FIX, or "no change: <reason>". That includes the places listed in
     "What I checked myself" that the map misses.
   - The report's §9 gets one row for each place.
2. **Two game servers** (the correction and his first answer).
   - **RS:** his words, quoted: "The CCX23 (dedicated) is used only for the measurement runs: tick timing, the 60 Hz
     margin and rooms per VM. Each run's plan says which server it needs."
   - **The new code DS:** the CCX13 (dedicated x86, 2 vCPU, 8 GB; 43.49 EUR a month, 0.0697 an hour, excluding VAT,
     R6-HOST-F15), for development and "does it work" runs. Source: his answer to the session's question 1 (the option
     he chose, in the session's wording). Never call it "the development server" alone: ADR 0005 and ADR 0002 use that
     name for the local server.
   - **Where DS goes:** the keys of ADR 0001 and ADR 0006, and of ADR 0004 if any of its rows can run there; SPIKE_PLAN's
     machines paragraph; PM-0001-1 (the rig); PM-0004-1 (the machines recorded); PM-0006-18 (DS's hours at 0.0697);
     PM-0006-11. PM-0006-11's mark changes:
     - every measurement row ran on RS (SH where its row says);
     - every other server row ran on the server its run's plan names;
     - DS is recorded the way RS is.
   - **The machine columns stay as they are.** ADR 0001's key gets one sentence (test-engineer), and ADR 0006's key
     points to it: "A row whose column names RS runs on RS when it is one of the three measurement kinds; any other row
     runs on the server its run's plan names; a time-limited row that its plan puts on DS and that fails there is re-run
     on RS before it counts as failed." Mark it as the advisor's reading, 2026-11-03: DS has 2 vCPU, where RS's key
     gives the transport three.
   - **Part 5 and 0006:753** (infra-engineer): "every native server mark …" becomes "every server time mark …", or
     equivalent.
   - **The correction's first sentence** is not quoted in the drafts. The appendix carries it byte for byte, with the
     answer that resolved it.
3. **Location** (his second answer): Nuremberg (`nbg1`) for the game servers and the simulated clients, labelled as in
   HZ-LOCATION.
   - OQ-0006-2's options stay as the record.
   - Part 5, :632–634, :1189, PM-0006-11 and PM-0006-12 say `nbg1`. None of them keeps "nbg1 or hel1" or "picked by a
     round-trip measurement".
   - PM-0006-12's target is a VM of the run in `nbg1` other than RS and DS, with ICMP allowed on it only. No CX33.
4. **LG.**
   - His words, quoted: "Use a CAX21 (Arm64, 4 vCPU, 8 GB, about €0.020/h incl. VAT) for the simulated clients instead
     of the CX33, so their build targets aarch64."
   - In Nuremberg: whether it "(or another client VM)" can be ordered is checked in the proof test's plan (his chosen
     option, the session's wording). The calibration run still sets how many.
   - The drafts say nothing more. A note for the proof test's plan, not for the drafts: if a CX33 can be ordered there,
     that plan may weigh it as the client VM (one x86 build, so condition 5 costs nothing) and put the choice to him
     with its cost cap.
5. **Arm in ADR 0001** (netcode-architect; test-engineer writes the row text).
   - The rule: no pass mark's verdict rests on native aarch64 agreeing with x86-64 or Wasm. That agreement is unproven:
     PM-0004-73 runs on x86 native and Wasm only, and PM-0004-26 adds AArch64 only before Arm hosts are picked.
   - It applies to every row that compares a result an LG client computes with the server's: at least PM-0001-54 (the
     landings) and PM-0001-48 (the shooter's view). It applies to -17, -18 and -19 too if their client side does any
     float work; the author confirms it (-17's bite names "a client quantiser").
   - Each such row picks one of two ways:
     - (i) it takes its verdict from L1, an x86-64 client or a browser (Wasm), with LG's clients as load only;
     - (ii) a mismatch seen on LG is replayed on x86-64 (the same inputs on L1) before it counts. A mismatch that shows
       only on aarch64 is reported as a cross-architecture finding and decides nothing.
   - "Part 7's rules make them agree" is not enough on its own.
   - The rule holds whatever LG turns out to be. No row is added.
6. **ADR 0004** (rust-reviewer).
   - An Arm load generator is not "an ARM host" in part 7's sense: it serves no player and runs no room (the advisor's
     reading, 2026-11-03). S7, PM-0004-26 and the A64 key stay as they are.
   - At most one sentence, pointing to condition 5's rule in ADR 0001.
   - The CX33 code stays. Its key says it is measured only if the proof test's plan finds it orderable in Nuremberg,
     and PM-0004-4's CX33 part runs only then.
7. **ADR 0002** (engine-architect a).
   - "No arm64 leg" stays: RS, DS and SH are all x86-64.
   - At most one sentence: the bot clients' aarch64 build is the netcode proof test's own. The playtest soak's bot VM
     is named when the first soak is planned. If that VM is Arm, `devtools` gains an aarch64 build target in that
     change-set (a build, not a determinism leg), and the soak's desyncs follow condition 5's rule.
   - The plan's "the LG binary is throwaway spike code" misses the soak at 0006:373.
   - 0006:373 (infra-engineer) becomes "on a client VM beside the reference server, never on it (as spike 2's load
     generators)", priced at the CAX21's rate: 3 × (0.1386 + 0.0176) = 0.47 EUR.
8. **SH, K3 and the budgets**, as HZ-SH and HZ-STAGING rule.
   - PM-0001-27, 0006:1134 and PM-0004-4: SH's part runs only if SH exists.
   - K3 (0006:352, :768, :1448) becomes conditional on SH being measured.
   - 0001:1293 reads "on the CCX23 and SH (ADR 0006)".
   - 0006:1435: "on the staging VM's plan (proposed a CX33, part 4)".
   - MV as HZ-MV rules.
9. **His words.**
   - His words come only from the six inputs files. The option texts in HETZNER-ANSWERS are labelled as the options he
     chose, in the session's wording.
   - The screenshot description is never presented as his words.
   - Every reading is labelled "(the advisor's reading, 2026-11-03)".
   - Times come from the inputs files: about 18:26, 18:32 and 18:34. The addendum's "about 18:40" is the session's
     note, not a time the inputs files give.
10. **The routing.** It is the plan's, plus:
    - infra-engineer: condition 1's ADR 0006 places, including the Appendix A status cells;
    - test-engineer: DS in the keys, PM-0001-1, PM-0004-1, PM-0006-11, -12 and -18, and condition 2's sentence;
    - engine-architect (b): SPIKE_PLAN :176, :281 and :513, and README's counts at lines 21 and 23–25.
11. **Verification (his stop).** No fixcheck and no skeptic. The session runs:
    - the write guard, dispositions, glossary, tables and counts;
    - quotes against all six inputs files;
    - a grep that every "CX33" left is his words, a record, a NOTE, or a proposal that depends on the order check in
      Nuremberg;
    - a grep that no "Falkenstein" or "fsn1" is left outside his words and the options' record;
    - a grep that no "nbg1 or hel1" or "picked by a round-trip measurement" is left as a live rule;
    - a check that PM-0006-11 no longer says "every server row ran on", and that DS is defined in every key that uses
      it;
    - the counts 115/96, ADR 0006 21/17, and 341;
    - the gate.

    My final-head review is the merge rule's review, not a round (r5's condition 8).
12. **Close as r4's conditions 7 to 9 and r5's condition 9.** The appendix also carries, byte for byte, HETZNER-CHECK,
    HETZNER-CORRECTION (with the screenshot description, marked as the session's) and HETZNER-ANSWERS, and this verdict.
    docs-keeper (high) updates STATUS:
    - the check, the correction and the answers;
    - the new counts;
    - "For Mustafa" without the console check;
    - STATUS:117's record kept, with the new words beside it.
13. **Models and effort:** Opus for every agent. The authors work at xhigh, docs-keeper at high. One round, each author
    on its own text.
END CONDITIONS

### Findings (ranked)
1. HZ-SH (a) and HZ-STAGING (a) would run the room server on Arm. That goes against ADR 0004 part 7, R6-HOST-REC2 and
   the option he chose ("no second build target for the room server").
2. The map misses about 20 places that the correction and the location answer change. It also misses two CX33 places
   (0006:1268, :1274), though both are routed (condition 1).
3. Relying on "part 7's rules" for aarch64 agreeing is not proven (condition 5).
4. HZ-OQ1 (a) and the count of 95 close a question whose order check moved to the proof test's plan. The count is 96
   (ADR 0006: 17).
5. The playtest soak (0006:373) carries LG beyond the spike, so the aarch64 build isn't only "the spike's own"
   (condition 7).
6. "Development server" is already taken in ADR 0005 and ADR 0002 (condition 2).
7. HZ-VERIFY names four inputs files; there are six. HETZNER-ANSWERS's option texts are the session's (condition 9).
END FINDINGS

### Decisions needed
None new from this plan. The round can run in full now: nothing is pending from Mustafa. MACHINES-TIMING (r5) stays in
the report.

### For Mustafa
1. Nothing new from this round. The console check is done, and it leaves the list.
2. Still on the list from r5: register shamblefrontlines.com at Cloudflare before the netcode proof test; borrow and
   name the laptop, desktop and Mac; OK each run's cost cap when its plan reaches you. That plan also checks which
   client VM can be ordered in Nuremberg.
3. Once I have approved PR #33's final head and gate and smoke are green on it, say "merge #33" in this session's chat.
   It merges by squash only.

Nothing in Brave: nothing a player sees changes.
END VERDICT

Status: DONE

## AMENDMENT (verbatim)

### Notes (amendment)
- 18:49 read HETZNER-CORRECTION-2 (his words, about 18:48) and HETZNER-ANSWERS-2 (about 18:49; "No, drop the CCX13", option text the session's: "Every server run uses the CCX23 (the measurement-only rule goes); the rig is just the CCX23 and the CAX21."), and the plan's Addendum 2 (7 points). The round launched on the first verdict wrote no block (session's word).
- His "a CAX21 for the 48 simulated players" is singular and says 48; the rooms-per-VM rows run 8 rooms at N = 48 (384 bot sessions: PM-0001-27, PM-0006-13, -14, -22, -24) and the 0006 key already says 384 sessions "may need more than one 4-vCPU VM" (proposed). Money, his: routed through each run's cap (SPIKE-COST). Non-blocking escalation LG-COUNT.
- Rows with SH, MV or CX33 in the machine column (script): 0001 PM-0001-1, -16, -27; 0004 PM-0004-1, -4; 0006 PM-0006-11, -12 (CX33 throwaways), -13, -14 (SH); -16, -22, -23, -24 (MV). None runs on SH or MV alone, so no row drops: 341 stays.
- OQ-0006-1 "Carries: R6-HOST-OQ1, R6-HOST-OQ4, R6-HOST-OQ7": closing it must not orphan OQ4 (root servers before the goal budget) and OQ7 (IPv4 price); Appendix A rows 1514, 1517 say "open: OQ-0006-1".
- With the CX33 not orderable in Falkenstein, part 8's lean case (production CX33 + staging CX33) and the staging VM's proposed CX33 rest on a machine his check found unorderable in the test servers' location: a money choice for later (before slice 3), so a numbered open question is the honest home, not a label.
- VMs created fresh each run day: new addresses AND new SSH host keys; capacity on the day is not guaranteed (ADR 0001:756's failure row stays live).
- 18:53 composing the amendment.

### AMENDED VERDICT: APPROVE WITH CONDITIONS (replaces the rulings and conditions above where they differ; read alone)

Plan revision 6 with Addendum and Addendum 2, v2-decision-drafts, PR #33 (draft), branch tip 898ba05, base main
9e7db28. Advisor agent, 2026-11-03, about 19:00.

**Why, in one line:** his second correction and answer simplify the rig to two machines in Falkenstein, a CCX23 and a
CAX21. So DS goes, SH and the separate metrics VM go, and OQ-0006-1 and -2 are answered. Three things still need care
in the drafts: the staging server and the lean budget now rest on a machine that can't be ordered there, "a CAX21 for
the 48 simulated players" is singular while some rows run 384 bot sessions, and every run day brings new addresses and
host keys.

**TL;DR**
- The rig is RS (the CCX23) and LG (the CAX21), both in Falkenstein (`fsn1`), in his words. Every server row runs on
  RS. No DS, no SH, no separate MV.
- OQ-0006-1 and OQ-0006-2 are answered. One new open question, OQ-0006-22: the staging server's x86 plan (money,
  Mustafa, before slice 3). Counts: 116 numbered, 96 open (ADR 0006: 22 and 17). Pass marks stay at 341.
- The Arm rules for LG stand (ADR 0001's condition 5, ADR 0004, ADR 0002).
- The drafts assume no fixed address. Every address-bound setting is made on the run day.
- One non-blocking question for Mustafa (LG-COUNT), answered when the first run's plan reaches him.

### What I checked myself (amendment)
- Both new inputs files, read in full. In HETZNER-ANSWERS-2, his words are his choice; the option text is the
  session's.
- The plan's Addendum 2.
- By script, the rows whose machine column names SH, MV or CX33:
  - ADR 0001: PM-0001-1, -16, -27;
  - ADR 0004: PM-0004-1, -4;
  - ADR 0006: PM-0006-11, -12, -13, -14, -16, -22, -23, -24.

  None of them runs on SH or MV alone, so no row drops.
- OQ-0006-1's "Carries" field (0006:810) and Appendix A at 1511–1517.
- PM-0006-13 and -14: 8 rooms at N = 48.
- The 0006 key's 384-session note (1194–1196).
- ADR 0001:756, the failure row "No Hetzner capacity in the EU".
- Not run: the gate (nothing edited). Not checked: the console. Both machines being orderable is his statement.

### Amended rulings (all mine or settled by his words; one non-blocking escalation, LG-COUNT)
- **HZ-LOCATION (OQ-0006-2): answered by his words.**
  - Quote: "both test servers will be in Falkenstein, not Nuremberg: a CCX23 for the game server and a CAX21 for the
    48 simulated players". Label: "Answered by Mustafa (2026-11-03)".
  - `fsn1` in every live place.
  - "Nuremberg" stays only in OQ-0006-2's options (the record) and inside his quoted words. "Helsinki" and `hel1`
    likewise.
  - His answer to the session's question 2 ("Nuremberg (nbg1)") is superseded. It is never quoted in the drafts; the
    appendix carries it.
  - Production: part 5 says the first fleet starts in the test servers' location unless playtesters' round trips
    (R6-HOST-OQ2, still open) say otherwise (the advisor's reading, 2026-11-03). His words name only the test servers.
- **HZ-OQ1 (OQ-0006-1): answered** (not "in part": nothing is left to check).
  - His words: "I created both once to try the console and deleted them right away", and, from his first note, "in
    Falkenstein the cost-optimized x86 line (CX33) can't be ordered; only its Arm64 version can."
  - Option (a) holds: the reference server stands.
  - The entry keeps carrying R6-HOST-OQ4 (the root servers, read before part 8's goal budget is fixed) and R6-HOST-OQ7
    (the IPv4 price), each still open with its own trigger as written. Appendix A's status cells say so; they don't say
    "answered" for OQ4 and OQ7.
  - "Created fresh on run day" means capacity on the day isn't guaranteed. ADR 0001:756's failure row and OQ-0006-1's
    option (b) route stay as the fallback (the advisor's reading, 2026-11-03).
- **DS: dropped.** It is his choice ("No, drop the CCX13"; the option text, the session's wording: "Every server run
  uses the CCX23 (the measurement-only rule goes); the rig is just the CCX23 and the CAX21.").
  - Nothing of DS goes into the drafts: no code, no CCX13, no key sentence, no re-run rule.
  - PM-0006-11's mark "the machine every server row ran on" stays: it is true again. Part 5's "every native server mark
    is measured on one vCPU of it" stays.
  - His first correction ("used only for the measurement runs") and his CCX13 answer are not quoted in the drafts. The
    appendix carries both, with the later answer that superseded them.
- **HZ-SH: SH leaves the proof test; K3 waits.** His check makes the CX33 unorderable in Falkenstein, and "the rig is
  just the CCX23 and the CAX21" (the option he chose).
  - Never a CAX21 as SH (ADR 0004 part 7; R6-HOST-REC2).
  - No other shared x86 plan is checked now. K3 (a shared fleet) waits for a later measurement of a shared x86 plan
    that can be ordered in the fleet's location.
  - G2's option row stays as weighed, with one pointer.
- **HZ-MV: no separate VM.** MV becomes a role on the CAX21, never on RS: Prometheus runs on LG's VM, inside LG's 80 %
  CPU rule (the calibration run includes it).
  - It still stands in for the staging game server.
  - The proof test's plan reads Prometheus's Arm64 build from its own release page. The drafts make no claim about it.
  - If LG-COUNT gives LG more than one CAX21, the plan says which one carries MV.
  - The firewall rule of PM-0006-16 opens the exporter's port to that VM's address of the day only.
- **HZ-STAGING: new open question OQ-0006-22** (infra-engineer).
  - The question: which x86 plan runs the staging game server, now that the proposed CX33 cannot be ordered in
    Falkenstein (Mustafa's check, 2026-11-03)?
  - Why it matters: staging runs the room server, and an Arm plan is an Arm host (PM-0004-26 first). Part 8's lean case
    rests on the CX33's price.
  - Options:
    - a CX33 (8.99) only where it can be ordered;
    - a CCX13 (43.49, R6-HOST-F15) if it can be ordered in the fleet's location;
    - a shared x86 "Regular Performance" plan (CPX32 35.99, R6-HOST-F15), whose locations are not researched.
  - Decider: Mustafa (money), with the Advisor. When: before slice 3.
  - Part 4 points to it. Part 8's lean case is labelled: it "rests on the CX33's price for production (K3) and staging;
    the CX33 cannot be ordered in Falkenstein (Mustafa, 2026-11-03), so it holds only where a shared x86 plan can be
    ordered in the fleet's location and K3 passes" (the advisor's reading, 2026-11-03).
  - The reference case and PM-0001-63's formula keep 8.99, labelled "the staging VM's placeholder price (the CX33's,
    R6-HOST-F15); its plan is OQ-0006-22".
  - 0006:1435 reads "on the staging VM's plan (OQ-0006-22)".
  - The lobby's "for example a CX33" stays in OQ-0006-3, an example.
- **PM-0006-12: a check, without DS or a throwaway VM.**
  - Target: the run's CAX21 in `fsn1`, with ICMP allowed on it only (never on RS), pinged from ML and MP.
  - Reported against VISION_V2 §4's "about 100 ms". It decides nothing; a surprise goes to Mustafa.
  - The row stays; its title loses "each candidate location" and "feeds OQ-0006-2".
- **New addresses each run day (his words: "They'll be created fresh on run day, so their IP addresses will be new
  then.").**
  - The drafts write no address and assume none is fixed. The run records the day's addresses with its results.
  - Every address-bound setting is made on the run day:
    - the per-address connection cap raised on LG's addresses;
    - any firewall rule that names a source (PM-0006-16's scrape);
    - the WebSocket leg's DNS record on shamblefrontlines.com, set before its certificate is issued.
  - New VMs also bring new SSH host keys. How a session checks a host key before trusting it is the proof test's plan's,
    with security-auditor (r4's SPIKE-MACHINES (a)).
  - The WebTransport certificate is pinned by hash, not by address, and is made per run.
  - This is the advisor's reading, 2026-11-03, written as at most one sentence in ADR 0006's key and one in ADR 0001's.
- **HZ-PRICE:**
  - Prices excluding VAT come from R6-HOST-F15 only: CCX23 0.1386 and CAX21 0.0176 an hour. The CX33's 0.0144 appears
    only in records and placeholders.
  - His "about €0.020/h incl. VAT" is quoted as his.
  - The screenshot's figures and the option texts are the session's. No VAT rate.
- **HZ-RECORD:** as ruled above.
- **HZ-COUNTS:** 116 numbered, 96 open; ADR 0006: 22 numbered (OQ-0006-1 to -22), 17 open (-1 and -2 answered; -22
  new). Pass marks: 341 (rows lose their SH, MV and CX33 parts; none drops).
- **HZ-VERIFY:** his stop holds; no verifier and no skeptic (condition 11). Quotes are checked against all eight
  inputs files.
END AMENDED RULINGS

### Amended conditions (complete; they replace conditions 1 to 13 above)
1. **The work list comes from a script.**
   - The search covers the nine documents for: CX33; CCX13; `DS` (expected 0); SH and MV in machine columns and keys;
     OQ-0006-1; OQ-0006-2; R6-HOST-OQ1; R6-HOST-OQ2; Nuremberg|nbg1|Helsinki|hel1|Falkenstein|fsn1; "every server
     row|every native server mark|every server mark"; and "address" in the machine keys and the operations checklist.
   - Each hit gets a disposition from its author: a FIX, or "no change: <reason>". The report's §9 gets a row for each
     place.
   - It must include:
     - ADR 0006: 119, 189, 244–254, 267, 325–326, 352, 373, 607, 632–634, 713, 715, 753, 768, 799–816, 825, 1073–1077,
       1133–1134, 1189–1199, 1267–1280, 1300, 1364–1370, 1435, 1446–1448, 1499 and 1510–1517;
     - ADR 0001: 1293, 1350–1357, PM-0001-1, -16, -27 and -63;
     - ADR 0004: 1030–1032, PM-0004-1 and -4;
     - README: 21, 23–25, 133 and 134;
     - SPIKE_PLAN: 31–33, 129–130, 176, 281 and 513.
2. **The rig: two machines, his words.**
   - Quote "a CCX23 for the game server and a CAX21 for the 48 simulated players" and "I created both once to try the
     console and deleted them right away. They'll be created fresh on run day, so their IP addresses will be new then."
   - RS: the CCX23 (x86-64 AMD) in `fsn1`. Every server row runs on it.
   - No DS and no CCX13 anywhere.
   - Not quoted in the drafts: his first correction's sentences and his answers to the session's questions 1 and 2.
3. **Location:** `fsn1` in every live place, as HZ-LOCATION rules.
   - OQ-0006-2's options stay as the record, labelled answered.
   - Nothing keeps "nbg1 or hel1", "Falkenstein … only if offered" or "picked by a round-trip measurement" as a live
     rule.
   - Production follows the test servers' location unless R6-HOST-OQ2's playtester round trips say otherwise (the
     advisor's reading, 2026-11-03).
4. **LG:** "a CAX21 for the 48 simulated players" (his words), with "so their build targets aarch64" (his first note).
   - Created in `fsn1` on the run day.
   - The calibration run's 80 % rule stays.
   - A run whose sessions one CAX21 cannot carry under 80 % (the 8-room rows, 384 sessions) names the extra CAX21s in
     its plan, for his OK with that run's cost cap (LG-COUNT below). Until he answers, the drafts say exactly that.
   - The order-check note and the CX33 client-VM note from the first verdict go.
5. **Arm in ADR 0001** (netcode-architect; test-engineer writes the row text). Unchanged.
   - No pass mark's verdict rests on native aarch64 agreeing with x86-64 or Wasm.
   - It covers PM-0001-54 and -48, and -17, -18 and -19 if their client side does float work.
   - Each such row picks one of two ways:
     - (i) it takes its verdict from L1, an x86-64 client or a browser, with LG as load only;
     - (ii) a mismatch seen on LG is replayed on x86-64 before it counts. An aarch64-only mismatch is reported and
       decides nothing.
   - No row is added.
6. **ADR 0004** (rust-reviewer and test-engineer).
   - An Arm load generator is not "an ARM host" in part 7's sense (the advisor's reading, 2026-11-03). S7, PM-0004-26
     and the A64 key stay; at most one sentence pointing to condition 5.
   - The CX33 code leaves the key, which says instead: "the shared CX33 is not measured: it cannot be ordered in
     Falkenstein (Mustafa, 2026-11-03)".
   - PM-0004-1 drops CX33. PM-0004-4 drops "and on the cheaper host" and its N = 48 CX33 part. Its author states
     whether R1-ECS-OQ4 and R1-TICK-OQ5 are still decided from RS alone, or whether either stays open.
   - The RS key (1030): `fsn1`, and R6-HOST-OQ1 answered.
7. **ADR 0002** (engine-architect a).
   - "No arm64 leg" stays: every room server runs on x86-64 RS, and the rig's only Arm machine is LG.
   - At most one sentence: the bot clients' aarch64 build is the netcode proof test's own. The playtest soak's bot VM
     is named when the first soak is planned; if it is Arm, `devtools` gains an aarch64 build target then (a build, not
     a determinism leg), and the soak's desyncs follow condition 5.
   - 0006:373 (infra-engineer): "on a client VM beside the reference server, never on it (as spike 2's load
     generators)", 3 × (0.1386 + 0.0176) = 0.47 EUR.
8. **SH leaves, and the rows and texts that carried it change.**
   - ADR 0001: PM-0001-1, -16 and -27 lose SH. 0001:1293 reads "on the CCX23 (ADR 0006)". The key drops SH.
   - ADR 0006:
     - PM-0006-11 creates "the CCX23 … in `fsn1`" and records it; there is no SH.
     - PM-0006-13 and -14 run on RS only, and -13's SH-only bite goes.
     - The key drops SH.
     - 119, 244, 254 and 713 no longer measure a shared CX33 beside RS. They say K3 waits, with one pointer to his check.
     - K3 at 352, 768 and 1448 waits for a later measurement of a shared x86 plan in the fleet's location.
     - G2's row stays as weighed, with one pointer.
   - SPIKE_PLAN:30–31: "SH, its shared neighbour" goes.
9. **MV and the metrics.**
   - MV is defined in ADR 0006's key as "the metrics role: Prometheus on LG's CAX21, never on RS, inside LG's 80 %
     rule, standing in for the staging game server" (the advisor's reading, 2026-11-03).
   - PM-0006-16, -22, -23 and -24 keep MV as that role. PM-0006-16's firewall rule names the day's address of that VM.
   - SPIKE_PLAN:33 says the same.
   - 0006:1073–1077 (OQ-0006-20's options and interim): the record keeps its words and gains one pointer.
10. **Staging and budgets:** OQ-0006-22 as HZ-STAGING rules (infra-engineer: the entry, part 4's pointer, part 8's
    labels, 1435). README gets its row (engine-architect b). PM-0001-63's label: test-engineer.
11. **PM-0006-12 and new addresses**, as the rulings say.
    - test-engineer: PM-0006-12, and one sentence in ADR 0006's key: the day's addresses, the settings bound to them,
      the host keys going to the proof test's plan.
    - netcode-architect: at most one clause in ADR 0001's key, at the connection cap on LG's addresses.
12. **His words.**
    - They come only from the eight inputs files.
    - The option texts in both HETZNER-ANSWERS files are the session's wording. Superseded words (the first
      correction's rule, the CCX13, Nuremberg) are never quoted in the drafts.
    - The screenshot description is never presented as his.
    - Every reading is labelled "(the advisor's reading, 2026-11-03)".
    - Times come from the inputs files: about 18:26, 18:32, 18:34, 18:48 and 18:49.
13. **Routing** (one round, every author on its own text):
    - infra-engineer: ADR 0006's places outside the pass marks, OQ-0006-1, -2 and -22, and Appendix A.
    - test-engineer: the keys and rows of ADR 0001 (PM-0001-1, -16, -27, -63, and -48 and -54 as netcode-architect
      answers), ADR 0004 (PM-0004-1, -4, the keys) and ADR 0006 (PM-0006-11 to -14, -16, -18, -22 to -24, the key).
    - netcode-architect: 0001:1293, condition 5, condition 11's clause.
    - rust-reviewer: condition 6's sentence.
    - engine-architect (a): condition 7.
    - engine-architect (b): README (rows OQ-0006-1, -2 and -22; counts at 21 and 23–25) and SPIKE_PLAN (31–33, 129–130,
      176, 281, 513).
    - Then docs-keeper and the report.
14. **Verification (his stop).** No fixcheck and no skeptic. The session runs:
    - the write guard, dispositions, glossary, tables and counts;
    - quotes against the eight inputs files;
    - greps: no CCX13, no `DS`, no SH in any machine column or key;
    - no live `nbg1`, `hel1`, Nuremberg or Helsinki (only OQ-0006-2's options and his quoted words);
    - every CX33 left is his words, a record, a NOTE or a labelled placeholder;
    - no address written as fixed;
    - counts 116/96, ADR 0006 22/17, 341;
    - the gate.

    My final-head review is the merge rule's review, not a round.
15. **Close as r4's conditions 7 to 9 and r5's condition 9.**
    - The appendix carries byte for byte: the five Hetzner inputs files (CHECK, CORRECTION with the session's
      screenshot description marked as the session's, ANSWERS, CORRECTION-2, ANSWERS-2) and this verdict with its
      amendment.
    - docs-keeper (high) updates STATUS: the check, both corrections and the answers; the counts; "For Mustafa" without
      the console check; STATUS:117's record kept, with his new words beside it.
    - The report's "Decisions needed" holds MACHINES-TIMING (r5) and LG-COUNT.
16. **Models and effort:** Opus for every agent. The authors at xhigh, docs-keeper at high.
END AMENDED CONDITIONS

### Amended findings (ranked)
1. Part 8's lean case and the staging VM rest on the CX33, which can't be ordered in Falkenstein: a money choice for
   later, now OQ-0006-22.
2. "A CAX21 for the 48 simulated players" against rows of 384 bot sessions (LG-COUNT).
3. Run-day VMs bring new addresses and host keys, and capacity on the day isn't guaranteed (condition 11; ADR 0001:756
   stays).
4. Closing OQ-0006-1 must not orphan R6-HOST-OQ4 and OQ7.
5. The SH and MV parts of 13 rows, which the plan's map doesn't list.
6. The first verdict's Arm findings stand (conditions 5 to 7).
END AMENDED FINDINGS

### Decisions needed (for the report; none blocks this round)
- **LG-COUNT**
  - The question: you named "a CAX21 for the 48 simulated players". Some proof-test rows run 8 rooms of 48 bots, which
    is 384 simulated players. If one CAX21 can't carry them under 80 % CPU, may that run add CAX21s?
  - Why it matters: an overloaded load generator makes the run invalid. Each extra CAX21 costs about 0.0176 EUR an hour
    (excluding VAT, R6-HOST-F15).
  - Options:
    - (a) the run's plan adds as many CAX21s as the 5-minute calibration needs, inside the cost cap you OK;
    - (b) one CAX21 only, and the 8-room rows run with as many bots as it carries (they would no longer test 8 rooms of
      48);
    - (c) decide when the calibration reports.
  - Recommendation: (a). The decision lands in the run's plan, with its cost cap, which reaches you anyway. The round
    proceeds with the drafts saying so.
- OQ-0006-22 (the staging server's plan) is yours (money), but not now: before slice 3.

### For Mustafa
1. Nothing to do for this round. The console check is done and leaves the list.
2. LG-COUNT above, when the first run's plan and its cost cap reach you.
3. Still from r5: register shamblefrontlines.com at Cloudflare before the netcode proof test; borrow and name the
   laptop, desktop and Mac; OK each run's cost cap.
4. Once I have approved PR #33's final head and gate and smoke are green on it, say "merge #33" in this session's chat.
   It merges by squash only.

Nothing in Brave: nothing a player sees changes.
END AMENDMENT

Status: DONE (amended)

## AMENDMENT 2 (verbatim)

### Notes (amendment 2)
- 22:25 read ANSWERS-3 (his words, about 19:04), LG-COUNT (about 18:56; option text the session's) and the plan's Addendum 3. Grep "register": ADR 0001:464, :1274; ADR 0006:289-290, 301, 761, 847, 982-984, 1367-1369; README:145; SPIKE_PLAN:148-149. macOS runners: ADR 0002 OQ-0002-5 (1027-1031: free while public, R5-CI-F17 + infra-engineer's read), ADR 0006 part 8 (360-363), Q13 (694), :890. Short on time (deadline 22:38): rulings first.

### Rulings (amendment 2; everything in AMENDMENT stands unless changed here)
- **MACHINES-TIMING: answered by his words (c, "with one step first").** Quote in full, from ANSWERS-3, in SPIKE_PLAN's
  machines paragraph (engine-architect b); the other places point to it.
  - **The step:** the netcode proof test's plan first checks "whether GitHub's macOS runners (real Safari, free while
    the repository is public) have a Safari new enough for WebTransport and can reach the test servers".
  - **Branch 1 (they can):** "that is the Safari check, and I borrow the Mac only for the look proof test".
  - **Branch 2 (they can't):** "I borrow the Mac before the netcode proof test."
  - **Laptop and desktop:** "rough checks first, then a short re-run once they're named". These words replace the
    advisor's reading in SPIKE_PLAN items 1, 3 and 4 where it says the same thing. Each spike's plan still lists the rows
    that run before the machines are named. Nothing is judged on a rough check.
- **May a GitHub macOS runner stand as MAC?** Yes, for the netcode proof test's Safari rows only, under branch 1 (his
  words). It must show first, recorded in the run's results:
  - (i) the runner image's Safari version on the day, at least the version R2-TRN-F7 gives for WebTransport (26.4);
  - (ii) real Safari driven through safaridriver, not a WebKit build;
  - (iii) from the runner: one WebTransport session to RS that opens and carries datagrams, and one WebSocket session on
    the domain.

  Limits (the advisor's reading, 2026-11-03):
  - The runner stands in only for yes-or-no and count rows (PM-0001-28's four-browser path and library pick, PM-0001-14,
    PM-0001-25's Origin, PM-0006-15). A row whose limit is a time, or which ADR 0003's look proof test needs, stays on
    the borrowed Mac.
  - The workflow that runs it is part of the proof test's plan: security-auditor and infra-engineer review it, and any
    `.github/` change in this repository goes to Mustafa (escalation category 3).
  - The later spikes' Safari rows are each spike's own plan's, in the same way.
  - Where it goes: ADR 0001's key, at MAC (test-engineer), one sentence with the step, (i) to (iii) and the limits;
    ADR 0006's key points to it (test-engineer).
  - ADR 0002 OQ-0002-5 and ADR 0006 part 8 already read the runner's price (free while public): no change, or at most
    one pointer (engine-architect a; infra-engineer). OQ-0002-5 stays open: it asks about pull requests, not the spike.
- **Counts:** unchanged. MACHINES-TIMING was a decision in the report, not a numbered question. OQ-0003-1 stays "answered
  in part" (the actual machines are still to be named). 116 numbered, 96 open; ADR 0006: 22 and 17; 341.
- **DOMAIN: registered.** His words: "DOMAIN: done. shamblefrontlines.com is registered at Cloudflare."
  - His earlier quotes ("a domain I register", "which I register at Cloudflare") stay as his words where they are
    quoted, with one pointer to "registered (Mustafa, 2026-11-03)".
  - Live sentences ("he registers …", "the domain he registers", "before it" / "before the netcode proof test") become
    "registered".
  - Who writes what:
    - ADR 0006:289–290, 301, 761, 847, 982–984 and 1367–1369 (infra-engineer). The milestone at 1367–1370 loses the
      registration as a to-do.
    - ADR 0001:464 (netcode-architect) and :1274 (its author: test-engineer if in the checklist or key).
    - README:145 and SPIKE_PLAN:148–149 (engine-architect b).
    - STATUS and the report (docs-keeper): "For Mustafa" loses the registration.
  - The DNS record and certificate for each run day stay as AMENDMENT condition 11 says.
- **LG-COUNT: answered.** He chose the session's option "Add as needed (Recommended)", whose text is the session's
  wording: "The run's plan adds as many CAX21s as its 5-minute calibration shows are needed, inside the cost cap you OK
  for that run." AMENDMENT condition 4's last sentence now reads: "A run whose bots one CAX21 cannot carry under 80 %
  adds as many CAX21s as its 5-minute calibration shows are needed, inside the cost cap Mustafa OKs for that run
  (LG-COUNT, the option he chose, 2026-11-03)." When the run has more than one CAX21, its plan says which one carries MV
  (AMENDMENT, HZ-MV).
- **Inputs:** now ten files for the quote check (STOP, DESIGN, ANSWERS-2, HETZNER-CHECK, HETZNER-CORRECTION,
  HETZNER-ANSWERS, HETZNER-CORRECTION-2, HETZNER-ANSWERS-2, LG-COUNT, ANSWERS-3). The option texts in HETZNER-ANSWERS,
  HETZNER-ANSWERS-2 and LG-COUNT are the session's. The appendix carries all ten files' Hetzner and answer texts byte
  for byte (the new two included).
- **"Decisions needed" after this:** none open from PR #33. MACHINES-TIMING and LG-COUNT are answered and leave it.
  OQ-0006-22 (the staging server's plan: Mustafa, money, before slice 3) is listed as a later item, not a decision now.
- **"For Mustafa" after this:**
  1. Borrow and name the laptop and desktop (rough checks first, then a short re-run); the Mac as branch 1 or 2 decides.
  2. OK each run's cost cap.
  3. The `.github/` workflow for the Safari check, if the proof test's plan puts it in this repository.
  4. Once the advisor has approved PR #33's final head and gate and smoke are green on it, say "merge #33" in this
     session's chat (squash only).
- Routing: these items join AMENDMENT condition 13 for the same authors. One round, xhigh; docs-keeper at high.
END AMENDMENT 2

Status: DONE (amended twice)
