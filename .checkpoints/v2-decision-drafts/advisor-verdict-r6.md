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
