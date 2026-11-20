# The advisor's check of U4h's brief, rev 1 (2026-11-20)

Seat: a day session's advisor (general agent acting on main's `.claude/agents/advisor.md`, read whole with `git show
origin/main:`), branch claude/netcode-proof-test-plan-hcky5g. Read only; this file is my one write; no cargo,
check.mjs, npt-lab or judge run; no commit.

## Plan
1. Read TEST-FIRST (history.md:2234-2282), SIZE-CHECK (:2048-2051), the brief (296 lines) and its notes whole, my U4g
   brief check (form and ADV-U4GB-1 to -15).
2. Read u4-build-plan.md §5 U4h (:903-918), the lanes (:920-956), §4.9, §4.14, §9; u4s-carries.md's U4h section and
   every carry naming U4h; plan.md's E11/E12 and PM rows as needed.
3. Check the brief's code claims at the head (crates/sim, room, server, client, bots, lab, judge, proto), marks.json's
   rows -7, -48, -49, -50, -51, -54, -56 and SERIES.md's rows the brief names.
4. Item 1: completeness against plan and carries, the three unregistered items and how each gets on the register, any
   other missing item (search by "U4h").
5. Item 2: size; item 3: the U4g dependency and U4G-Q1; item 4: disk and run counts.
6. Item 5: rule U4H-Q1 to Q12; item 6: Q4 and Q7 against marks' meaning; item 7: U4H-SIZE and U4H-JUDGE for him.
7. Findings ADV-U4HB-n, verdict, what I did not check, Decisions needed, For Mustafa.
STATUS: all blocks done (2026-11-20). Verdict REVISE; rev 2 comes back for sections 1-4, 6 and 8.

## Notes (one line per check, written as I go)
- Head f1a7f73; `git diff --stat a289def HEAD -- spikes/netcode-proof` empty (the brief's 33dabe8 reading holds);
  marks.json c1935a5b…, SERIES.md 7f11afbe… (as in my U4g check).
- Code claims, each read: HitHistory::for_ms, 220 ticks / 7,333 ms (sim/src/history.rs:23-39; one tick at most 4,536 B,
  seam.rs:414-415); HISTORY_MS 2,000 (room.rs:33, :550); ROOM_HISTORY_MS (lab scenario.rs:78-80); rewind_at's integer
  half (room/src/rewind.rs:13-33) and its call (room.rs:1361-1368); RttSource, Decoder::rtt_ns (server/src/pump.rs:
  183-190, :339-344); pump_report has no RTT (room.rs:877-885); its callers are the lab (scenario.rs:704) and npt-server's
  tests (tests/common/mod.rs:290, tests/loopback.rs:213), not npt-server's src; ray_cast (world.rs:1092-1102); the box
  blend (world.rs:209-223); trajectory_at (trajectory.rs:13); STAND_IN/GRENADE (world.rs:111-113); PRESS_THROW
  (seam.rs:68); TC (scene.rs:252-265); bots lib.rs:10; the four planned figures (judge/src/figures.rs:841-860); the aim
  widths private (proto/src/input.rs:53-54), 44 bits (:310); hash.rs's span check (:370-376). All hold.
- **TC's weapon is CO's** (`..Scene::course()`, scene.rs:263): spread 5,000 µrad, magazine 30, reload 2 s, 100 ms
  (scene.rs:236-243). The authority applies the spread from its own stream, up to spread x distance per axis
  (world.rs:1622-1631); the plates stand 30 m from the shooter (scene.rs:338-342), the target about 20 m (homes,
  scene.rs:262): up to 15 cm and 10 cm per axis. No client can reproduce it.
- A throw press makes only a GRENADE (world.rs:1522-1545); TC spawns no stand-in (CO's spawn 0, live 0: scene.rs:225-231).
- The server's shot casts against f32 boxes blended at the fraction (world.rs:1574-1610), private; `World::ray_cast`
  takes integer HitVolumes; the fire origin is npt-sim's private CHEST_CM (world.rs:82; lab copy scenario.rs:84-86).
- -50's bar 33,330,000 ns (marks.json:10045); one interval at 30 Hz is 33,333,333 ns. The ADR row's cheat "announces
  the largest allowed delay" (ADR 0001:1647; r2-netcode.md:2188-2190). SERIES.md:153 and :187 define the gain against
  "its honest twin shot"; marks.json:10108 "pairs each cheat shot with an honest twin shot at the same RTT".
- SERIES.md:86's sign is the reverse of the check's (min >= 145 ms on a delayer that inflates the ping RTT).
- The brawl stamps view_tick = its newest snapshot tick, fraction 0 (bots/src/lib.rs:22-24); U4g's brief does not
  change it (no "view time" in u4g-brief.md).
- The controller's padded target is client-side (client/src/lib.rs:634-646; u4f-review-r1-full.md:39-40).
- SERIES.md:265 gives U4h the shot view-test and throw-landing kinds of cross_arch_mismatch_count; :274, :295, :299 carry
  L2 Arm-rule halves. JDG-ONLY-1's home is npt-judge's next change (history.md:4611-4614).
- Not taken anywhere yet, routed to "the next lane that opens crates/sim": EA-U4R-4 (advisor-u4r-r1-full.md:182-186) and
  EA-B1-1's later half (u4r-report.md:1164-1165); U4g's brief does not name them. SERIES.md:368's writer cell "at the
  next SERIES.md edit" (history.md:4976), now :396, still "npt-room": U4f's edit (25d316e) did not take it.
- The "next npt-lab lane" items (N2-1, TE-R2-2, RR4-L1, RR4-L2, EA-U4R-1/-2, RR3-L2 (a)) were all done in BRAWL-1's Part
  1 (u4r-report.md:275-280); T-6 rides U4f2 (u4f2-brief.md:136); RR-5's encode wins are optional (advisor-u4e-r1.md:217).
- Counts: 24 + 6 + 7 + 12 + 3 = 52 measured, 5 bites, 12 calibration = §9's U4h figures (u4-build-plan.md:1031-1035).
- Section 6's columns sum to their totals (2,035-3,640; 780-1,575; whole 2,815-5,215; at 1.5: 4,220-7,820).
- det-u4 at U4r's merge: 223.7 s of 240, of which npt-lab hash 220.3 s and Wasm 2.5 s (u4r-report.md:154-157): RR-L4
  (Wasm leg only: advisor-u4r-build1-full.md:99) can buy back at most 2.5 s.

## Verdict: REVISE (one High; rev 2 comes back to me for the changed parts only)
What holds: the lane's shape apart from crates/sim, the code claims, the seven marks' parts, checks and bites, the 52 +
5 + 12 count against §9, the carries of u4s-carries.md:609-651 by number, C-U6-19's line (a "throwing" client is one
that presses throw: advisor-u4f-f1-r1.md:120-122; -54's thrower is honest), LEAD-BOUND, the rule that the build
changes no marks.json text, the size arithmetic, the U4g dependency. What does not: TC as it stands cannot pass -48 or
-51 with a correct build, and several run designs the marks need are not yet buildable as written.

## Findings (CLAUDE.md's scale)
- **ADV-U4HB-1 (High): TC's weapon spreads every shot, so a correct build fails -51 and loses -48's registration.**
  TC inherits CO's weapon (scene.rs:263, :236-243): 5,000 µrad spread, which the authority draws from its own stream,
  up to spread x distance per axis (world.rs:1622-1631). The plates are 30 m from the shooter (scene.rs:338-342): up to
  15 cm per axis against -51's "within 5 cm in 99.9 %" (marks.json:10206-10222); the target at about 20 m: up to 10 cm,
  so view hits fail to register and view misses register (-48's 99 % and 0.5 %). No client can match it. The 30-round
  magazine and 2 s reload also break "one view test per server shot" (ADV-U4HB-2). Section 1.3 says TC needs only "the
  suites"; Q5 says "no scene change". **Fix:** TC gets its own weapon with spread 0 (and a magazine and reload the
  shooter's script meets, or the shooter reads its own-state record), recorded in every header (carry 60); crates/sim's
  lane gains scene.rs's TC weapon; a unit test that a TC shot's server ray runs from the server's fire origin through
  the command's aim point; recordings.json's synthetic TC entry re-pinned only if its SHA-256 moves, with the reason.
- **ADV-U4HB-2 (Medium): the shot id and the view-test join are undefined.** Under option B a rerun fires again under
  the same command sequence (room.rs:1361-1368 builds a Rewind for every run with fire held), so (slot, seq) is not
  unique; a command the sim refuses to fire (cooldown, ammunition, reload) leaves a view test with no server shot. The
  brief's own figure test makes "an unjoined shot id a gap" (section 3), so -48 would come out a gap. **Fix:** define
  the shot id in SERIES.md (one per server shot); the TC shooter fires by press edges, so a rerun fires nothing (an edge
  needs counters that differ: u4-build-plan.md §4.2 :247-252) and carry 81's reruns are then counted, never joined; the
  view test is logged only for a command whose press the shooter sent, and a press the sim refused is "not fired"
  (counted, printed), never a gap; a contract case holds one rerun and one refused press.
- **ADV-U4HB-3 (Medium): U4H-Q2 (a) leaves the other runs undefined.** It does not say what cap a run without the
  parameter gets (SS, ES, CO, the pre-check, carry 93's ES runs, the TC recording), which span the TC B1 seed 3 entry
  uses against hash.rs:370-376's one constant, or what U4h's tolerance does to the brawl, which stamps its newest
  snapshot tick (bots/src/lib.rs:22-24) against an estimate holding an announced delay the room assumes for a client
  that sent no ClientSettings (unstated). If that default is above 0, nearly every brawl shot is replaced: carry 88's
  pre-check legs go VOID by their setup condition (carry 40), carry 93's ES value is measured on replaced shots, and
  "no existing runner entry is re-pinned" fails at det-u4 (det-recordings.json pins SS-P2-s1's and ES-P3-s2's SHA-256).
  **Fix:** U4H-Q2's ruling below.
- **ADV-U4HB-4 (Medium): -50 can fail by 3.3 µs, or pass with a weak cheat.** One interval is 33,333,333 ns; the bar is
  33,330,000 ns (marks.json:10045), so a stamper accepted at exactly one interval fails a correct room. A stamper inside
  the edge, or one whose shots are replaced, passes vacuously; and block 5's stamper does not "announce the largest
  allowed delay" (ADR 0001:1647). **Fix:** U4H-Q5's conditions below.
- **ADV-U4HB-5 (Medium): the buffer-wait cap has no room-side source.** Block 1 caps the wait at "the controller's
  padded target for that client", which only the client holds (client/src/lib.rs:634-646); the room reads neither a
  client figure nor a run parameter (u4-build-plan.md §4.9 :385-386; .claude/rules/netcode.md, first rule). **Fix:**
  rev 2 states the room's own formula (from the pump's RTT, the tick period and LEAD-BOUND, never a client figure), with
  the security-auditor's read in the round; tests: an honest option-B client at P4 never meets it, and a stamp-ahead buys
  nothing past it (carry 90).
- **ADV-U4HB-6 (Medium): AimCast cannot equal the server's cast through `ray_cast`.** The server casts against f32
  boxes blended at the Rewind's fraction (world.rs:1574-1610; FBox::blend :209-223), both private; `World::ray_cast`
  takes integer HitVolumes (:1092-1102); the fire origin is the private CHEST_CM (:82). So section 3's "equals the
  server's blended cast, hull parts included" needs either a client copy of the blend and of 140 cm (the copy EA-U4R-4
  exists to remove) or a shared entry point. **Fix:** crates/sim opens for one pub item, the shot phase's own
  blend-and-cast over two HitVolumes at a fraction (used by both sides), and the fire-origin height as a pub item
  (EA-U4R-4, advisor-u4r-r1-full.md:182-186; with EA-B1-1's later half if U4g has not taken it, u4r-report.md:
  1164-1165); the public items listed by crate (A19).
- **ADV-U4HB-7 (Medium): -54 has no source for its second kind.** A throw press makes only a grenade (world.rs:
  1522-1545); TC spawns no stand-in (scene.rs:225-231 inherited). **Fix:** U4H-Q6's ruling below.
- **ADV-U4HB-8 (Medium): U4H-JUDGE asks about less than U4h builds.** SERIES.md:265 gives U4h the shot view-test and
  throw-landing kinds of cross_arch_mismatch_count; the three figures carry L2 Arm-rule halves (:274, :295, :299);
  JDG-ONLY-1's fix is npt-judge's next change (history.md:4611-4614). The text names four computations only. **Fix:**
  U4H-JUDGE's text below; the L2 halves and the two kinds built with fixtures only (T17's route, u4-build-plan.md:
  435-436), JDG-ONLY-1 in the same change (U4H-Q12).
- **ADV-U4HB-9 (Medium): the size moves with ADV-U4HB-1 to -8.** My rounded figures: the judge's part about 400-900
  (rev 1's 260-550, the L2 halves and two kinds 100-250, JDG-ONLY-1 40-100); the lag half's other additions about
  230-580 (spread and weapon 20-60, shot id 40-100, the edge 10-30, the cheats' forms and prints 30-70, the open cap
  30-70, the sim entry points 30-80, the ping accessor 10-30, Q3 and Q8 rows 20-40, Q10's row 10-20, SERIES rows and
  cells 10-30, the wait cap 20-50); the aim half about +30-90 (the stand-ins' source, the cover rows). With the judge as
  its own change (U4H-JUDGE (b)): U4h about 2,100-3,850 (up to about 5,750 at 1.5, under 6,000); U4h2 about 750-1,550
  (planned about 1,150, stop past 2,300); whole without the judge about 2,850-5,350 (middle 4,100; at 1.5 about even to
  pass 6,000); with the judge inside, the first half reaches about 7,100 at 1.5. **Fix:** section 6 recomputed item by
  item; U4H-SIZE and U4H-JUDGE as below. These are judgements on the code's shape, not counts.
- **ADV-U4HB-10 (Low): the calibration's figure.** The setup check reads `event.shot.clipped` on the rewind used
  (SERIES.md:209, :214), the accepted claim or the estimate, so the calibration takes the larger of the max
  `honest_rewind_ns` and the max `rewind_ns` of honest shots; the brief names neither.
- **ADV-U4HB-11 (Low): U4H-Q10 records before it settles the route.** INF-R2-2 says "U4h's plan settles the route
  (RR-L4 weighed first) before recording" (advisor-u4r-r2-full.md:45); the TC entry's length is unstated.
  **Fix:** U4H-Q10's ruling.
- **ADV-U4HB-12 (Low): the register.** Eight items reach U4h outside u4s-carries.md: the drafter's three and five I
  found (item 1 below). **Fix:** as item 1 says.
- **ADV-U4HB-13 (Low), text:** block 3's hand-off is the lab's (scenario.rs:704) and npt-server's tests'
  (common/mod.rs:290, loopback.rs:213); npt-server's src gains only an accessor for a Link session's ping estimate (the
  -50 gap's input: pump.rs has none, :339-344), and U6's TickLoop carry names the same hand-off. `frame_wait_ns`'s
  source is not stated (SERIES.md:211; the room sees no frame). Q1's "about 9 KB" (one tick is at most 4,536 B). The
  disk table's 69 runs leave out Q9's recorded runs and the TC recording. 1.1 should name the plan's second dependency,
  U4c's C2 (u4-build-plan.md:145, :903), met at U4c's merge. -48's report name "the run's largest measured rewind,
  rounded up" (marks.json:9413) predates T14: on rule 1's after-close list. -54's "differs" defined (the same landing
  tick and a bit-equal point, both from trajectory_at).

## Rulings on U4H-Q1 to Q12 (mine under TEST-FIRST rule 2 except Q4; the main session records each with its reason and
## names it in the day's message that carries U4H-SIZE)
- **U4H-Q1, the margin: (b), 3 ticks.** A larger margin costs only ring (one tick at most 4,536 B: under 14 KB for
  three, far under -7's 1 MB) and loosens no bar: the test cap clips, it does not judge. With ADV-U4HB-10's figure (the
  larger of the max honest rewind and the max rewind used for honest shots, per profile and option), ceil to ticks,
  plus 3; the ring holds the cap plus the margin plus one tick for the blend. A judged run with `clipped` above 0 still
  comes to me before its set counts; never re-run into a pass.
- **U4H-Q2, where the cap and span live: (a), with these terms.** (1) The room's cap is never off: `--cap` takes `open`
  (the default), `test` with its tick count, and later `rule`; `open` clips at the ring's reach (the span less the
  blend tick) and counts, so a correct room's Rewind is always held. Every header records `params.cap`,
  `rewind_cap_ticks_count` and `history_ticks_count`; the `cap` row's Values cell (SERIES.md:450) gains `open` in U4h's
  one SERIES.md edit (Q7). (2) HISTORY_MS stays 2,000 as the default span, made readable outside npt-room, and the
  lab's ROOM_HISTORY_MS gives way to it (carry 89's first two parts); the TC sets pass the test span. (3) The TC B1 seed
  3 entry is recorded at the default (`open`, 2,000 ms), so hash.rs:370-376's check stays as it is and the manifest
  schema does not change. (4) "No existing runner entry is re-pinned" is struck: det-u4 regenerates SS-P2-s1 and
  ES-P3-s2 on U4h's build; any whose SHA-256 moves is re-pinned with its reason (carry 89's third part), under RR4-L1's
  cap. (5) Rev 2 states the announced delay the room assumes for a client that sent no ClientSettings, and whether the
  brawl sends its own; a short SS P2 brawl row in c-u4h-r2.mjs prints the replaced and clipped shares, and a replaced
  share above 1 % there, in carry 93's ES runs or in carry 88's pre-check comes to me before any of them counts.
  (6) R2's run plan takes the test span for -7 (plan.md:724), named in U4h's report. No code commit sits between the
  calibration and the sets.
- **U4H-Q3, no RTT yet: (a), with one term.** The TC shooter and both cheats fire only in the measured window, and the
  warm-up holds every client's first RTT sample, the delay's settling and every admission; -48's checks read window
  `all` (marks.json:9378, :9390; README.md:145), so the contract case shows 0 no-RTT shots in every TC run's whole log.
  SS and ES clients' early shots run at the present, counted apart under a new SERIES.md name. (c) would hand a fresh
  session unchecked rewind; (b) invents an RTT.
- **U4H-Q4, -50's honest twin: his** (it decides the mark's verdict; text below). The build makes both figures:
  `event.edge_cheat_shot.gain_ns` and `event.pong_cheat_shot.gain_ns` under (a), the honest reference taken with the
  link's RTT (the true RTT) even on the ws leg, so the ws report shows what the inflated ping RTT buys; the real-twin
  figure printed beside it. His answer only picks which goes in the series; only -50's judged set waits.
- **U4H-Q5, -50's runs: as written, with these terms.** (1) The edge stamper announces the largest allowed delay (the
  clamp's top: ADR 0001:1647). (2) The tolerance's ns value and comparison are stated so the largest accepted edge gain
  is at most 33,330,000 ns (ADR 0001:363: "at most one snapshot interval ... data"), with a boundary unit test (accepted
  at the value, replaced one step past it). (3) The pre-run's short P1 edge run expects every edge shot accepted (0
  replaced) and its gains within one view-time step (1/256 tick, about 130 µs) of the edge; a weaker stamper comes to
  me. (4) The bite's alternating setting runs clean too in the pre-runs: the beyond-edge shots replaced, the max gain at
  most 33.33 ms. (5) The pong delayer stamps as a client that believed its ping RTT would (older by its hold), so P0 and
  P1 show the transport RTT's defence and the ws leg (P1, every session on `RttSource::Pings`, reported only) records
  the extra rewind it buys, held by the cap. Slot 0 the stamper, slot 1 the sprinting delayer: no scene change for -50.
- **U4H-Q6, -54's kinds: yes, both, 1,000 each, with a named source for the stand-ins.** A throw makes a grenade only
  (world.rs:1522-1545), so the `throws` suite turns TC's stand-in spawner on at a rate recorded in the header (scene
  data, no sim code), the client comparing each live projectile's landing with the server's; a stand-in kind with fewer
  than 1,000 a run comes to me. `event.throw_kind.throws_count` per kind, min >= 1,000 (marks.json:10659-10664).
- **U4H-Q7, the pong gap's sign: (a), mine.** Every word and number of -50 (the check's "gap between", 145 = 150 - 5,
  ADR 0001:1647) needs ping minus transport for a delayer that inflates the ping RTT; SERIES.md:86's order makes the
  check unpassable by any build. The cell is corrected in U4h's lane with test-engineer's read; no check, number or
  marks.json text changes. The same edit takes SERIES.md:396's writer cell (U4r build ruling 3 (e), history.md:4976)
  and Q2's `open`; :153 and :187 change only by his U4H-Q4 answer.
- **U4H-Q8, the TC shooter's aim: (c), with one term.** The offset band straddling the box edge is fixed in rev 2 and
  recorded in the header, never tuned after a run; each run prints its view hits and view misses (about 1,000 and 500 at
  2,000 shots), and a part whose run holds fewer than 400 view misses comes to me. (a) leaves the false-hit figure a gap.
- **U4H-Q9, the cover record: (b), on SS, not TC.** TC's lines of fire meet no cover item: no vehicles (scene.rs:226-
  228), the doors ring the centre 22 m out (greybox.rs:91) and TC stands at x -57 to -63 m (scene.rs:262, :341). So
  three SS P2 brawl runs under `--cover present` beside three under rewound, the same settings, each printing shots,
  rays that met cover and hits per variant (or carry 93's frozen-value ES runs as the rewound half, three present runs
  beside them); recorded, never judged. (a) widens the sim and doubles the shot's cast for a record.
- **U4H-Q10, det-u4's 240 s: (a), amended to settle first (ADV-U4HB-11).** A c-u4h-r2.mjs row measures the TC
  runner's rate and the hash leg on a short TC B1 run; the entry's length is chosen so the projected non-cargo sum is at
  most about 230 s, with at least one burst inside and within RR4-L1's 36,000 ticks; then record, then one det-u4 run
  with `uptime` first. Over 240 s comes to me whole, never retried. RR-L4 is weighed in one line: it acts on the Wasm
  leg, 2.5 s of 223.7.
- **U4H-Q11, the bite sites: as listed.** -48's two and -50's in npt-room; -51 in npt-sim's shot phase in
  `CHECK_BITES`' form, armed in the authority's World only, apart from `World::BITES` (npt-hash-wasm's index); -54 in
  `npt_client::BITES`; no id in two crates' lists (carry 24), each in lab/tests/bite_homes.rs.
- **U4H-Q12, JDG-ONLY-1: in the same judge change as the figures, not a change-set of its own.** Its home is npt-judge's
  next change (history.md:4611-4614); no U4h mark carries `pre:PM-0001-2`, so nothing orders it first; one change, one
  read by test-engineer and rust-reviewer, one yes from him (U4H-JUDGE names it). Its own block and fixture (the
  interim rule's case).

## Item 1: completeness, and how the unregistered items get on the register
- **Against the plan:** every "Builds" and "Marks moved" item of §5 U4h (u4-build-plan.md:903-918) is in the blocks,
  with T14's order; §4.12's contract, §4.14's two -48 bites and §4.15's third runner recording are in. Missing in
  substance: TC's weapon (ADV-U4HB-1) and the sim entry points AimCast needs (ADV-U4HB-6), both crates/sim.
- **Against the carries:** u4s-carries.md:609-651 met by number; carry 59's pointer to 93 is in (:284-285).
- **On the register, by the main session before U4h's launch** (u4s-carries.md's U4h section, the next free numbers,
  each quoting its source as ruling 3's task (a) did; the plan's U4h entry needs no line, as all fall under its
  "Builds"):
  1. N2-L3 (b): "U4h must blend composed boxes or interpolate hull yaw wrap-aware" (u4d-round2-netcode.md:81-82;
     routed by advisor-u4d-close.md:305-306); met by ADV-U4HB-6's shared blend.
  2. INF-U4R-6 with INF-R2-2: advisor-u4r-r1-full.md:289's line and advisor-u4r-r2-full.md:45's base and route words;
     met by U4H-Q10 as ruled.
  3. `pump_report`'s RTT: advisor-u4e-r1.md:662-664; with ADV-U4HB-13's site and U6's TickLoop line.
  4. EA-U4R-4 (advisor-u4r-r1-full.md:185) and EA-B1-1's later half (u4r-report.md:1164-1165), "the next lane that
     opens crates/sim": U4h's if U4g's report does not list them.
  5. SERIES.md's everyone_in_view writer cell (now :396), U4r build ruling 3 (e) (history.md:4976): U4h's SERIES.md
     edit.
  6. cross_arch_mismatch_count's shot and throw kinds (SERIES.md:265, "U4h"): the judge change.
  7. JDG-ONLY-1 (history.md:4611-4614): the judge change (U4H-Q12).
  8. ADV-U4HB-1, TC's weapon: a carry line of its own, so U4h2 (if split) and R3's TC runs inherit it.
- **Nothing else found** in the files I searched (every advisor-*.md line naming U4h, u4r-report.md's follow-ups,
  u4s-carries.md, the plan, history.md's ruling-3 tasks). Taken already: the next npt-lab lane's items (BRAWL-1),
  T-6 (U4f2), npt-server's tests compiling (U4f); RR-5's encode wins are optional.

## Item 2: the size
- Rev 1's table is sound item by item on the code's shape; its sums hold. With the findings: ADV-U4HB-9's figures. U4f
  ran about 1.5 whole and 2.0 on client and room (advisor-u4g-brief.md:94-97); U4f2 about 1.35 (2,077 against about
  1,550: history.md:7350-7351; u4-build-plan.md:874). The room is the largest item and where U4f ran 2.0: at the top
  of the range, the room (about 1,150) at 2.0 and the rest (about 2,700) at 1.5, the first half could reach about
  6,300 (my arithmetic). That is what the stop is for; raising the figure to make room would set the plan to the
  overrun.
- The lean (b) stands, as for U4g: the cheaper option in his sense is the one that keeps each half under its stop.
  And the judge's part as its own change (U4H-JUDGE (b)) is what keeps the first half under 6,000 at 1.5.

## Item 3: the dependency on U4g, and U4G-Q1
- Section 1.1 is right: U4h needs `Interp`'s published set and render time, the ClientSettings send and clamp, and the
  delay's adaptation (the honest rewind the calibration measures); not `Predictor` or `CueLog` (the shooter stands at
  its spawn). Add the brawl's view stamp (Q2 term 5) as an interface item U4g's report states.
- Under U4G-Q1 (a) U4h launches after U4g's merge and its sets; under (b) after U4g2's merge and its sets, since all
  three items land in U4g2 (u4g-brief.md:86-93). Unanswered, it moves U4h's launch, never its content. A later change to
  Interp's delay rule re-runs the calibration first (1.1's last line): it stands.

## Item 4: the disk and the run lists
- **Judged and calibration (L1, TC):** -48 24, -49 P4 6, -50 7, -51 12, -54 3 = 52 measured; 5 bites; 12 calibration;
  69 in all, as §9 (u4-build-plan.md:1031-1035). **Recorded beside them:** the TC B1 seed 3 recording (1), Q9's cover
  runs (6 on SS, or 3 with carry 93's), carry 93's ES runs (6-9), carry 88's pre-check (6, plus a re-record if needed),
  the pre-runs. About 85-95 runs.
- **Logs:** TC about 0.15-0.7 GiB (unmeasured), Q9 on SS about 0.4-1.3 GiB (62-210 MiB a run), ES 0.4-1.9 GiB,
  pre-check about 0.1 GiB: about 1.0 to 4.0 GiB in all. The build's disk dance holds as U4f2 ran it (about 10.1 GB free
  after each part-clean, a counted run leaving about 3.5 GB: history.md:7338-7353). D2 at launch against the 4,096 MiB
  floor; a set past it waits for a pack of a closed set (c62h-bound-brief.md 8.1 :485; KEPT-LOGS). Rev 2's table adds
  the recorded runs.

## Item 6: do Q4 or Q7 change a mark's meaning?
- **Q4: yes, his.** marks.json's note and SERIES.md:153 define the gain against "an honest twin shot"; a real honest
  shot carries the frame-to-tick wait (0 to 1 tick, which the estimate only centres) and the jitter, so a room built to
  ADR 0001 reads up to about 50 ms against the 33.33 ms bar, and about 100 ms more if the twin announced a normal
  delay while the cheat announces the largest. Reading (a) is the only one under which -50 can pass, so the reading
  decides the verdict: that is a pass mark's meaning (TEST-FIRST rule 2, "any change to a pass mark").
- **Q7: no, mine.** The SERIES.md cell's order contradicts the check's own words and number; correcting it makes the
  series measure what marks.json and the ADR row already say.

## For Mustafa, paste-ready (not 2026-11-21's message)
```text
U4H-Q4: what "an honest twin" means in the cheat test PM-0001-50 (a cheat cannot buy rewind). A cheating test client
claims the largest view delay the server allows and stamps its shots at the very edge of what the server accepts. The
test passes if each such shot gets at most 33.33 ms more rewind than "an honest client at the same RTT"; marks.json's
note says the lab pairs each cheat shot with "an honest twin shot".
(a) The honest twin is the server's own estimate for an honest client at the same true RTT and the same announced
    delay; the gain is the cheat's rewind minus that. A server built as ADR 0001 says passes; the planted bug (no
    tolerance check) fails by about 100 ms.
(b) The honest twin is a real honest shot from the same moment. A real shot also carries the wait from its drawn frame
    to the next tick and the network jitter, which the server's estimate can only centre, so a server built exactly as
    ADR 0001 says reads up to about 50 ms (about 100 ms if the honest player announced a normal delay): -50 would then
    fail unless its bar changed, which would be a separate decision of yours.
Under (a), marks.json keeps its words, the series file's two lines for the gain say what is measured, the reading goes
on the readings list, and (b)'s figure is printed beside it in every run's report, never judged. Only -50's judged
runs wait for this.
My lean, and the advisor's: (a).
```
```text
U4H-SIZE: the size of unit U4h (hits as the shooter saw them, the rewind cap, the rewind cheats, aiming at height,
thrown projectiles). It is planned as a medium unit (about 3,000 lines; your stop at 6,000). Our estimate is about
2,850 to 5,350 lines, not counting the judge's new computations (U4H-JUDGE). U4f, the last medium unit, came to about
1.5 times its estimate in all, so U4h built whole stands about even to pass 6,000 and stop mid-build.
(a) Build it whole as planned: one review, one merge. The stop past 6,000 stays; if it is hit, you get three lines then.
(b) Split it in two, each with its own review and merge: U4h, lag compensation (hits as the shooter saw them, the
    rewind cap, the two rewind cheats: about 2,100 to 3,850 lines; stop past 6,000), then U4h2, aiming at height,
    thrown projectiles, the cover record and the busy-scene hit rule (about 750 to 1,550 lines; planned about 1,150,
    stop past 2,300). No pass mark, judge rule or marks.json text changes, and no run is dropped.
The cheaper option is (b): one more review and merge, and no mid-build stop waiting on you. The build starts with the
lag-compensation half either way, so nothing waits for this answer until that half is done.
My lean, and the advisor's: (b).
```
```text
U4H-JUDGE: the judge gains the computations U4h's tests need, and one fix. marks.json already names figures the judge
cannot compute yet: the share of shots that hit in the shooter's view and that the server registered; the share of
view misses the server counted as hits; whether the later rule cap equals its rule; and the thrown projectiles whose
flight differs from the server's. Their formulas are written in the series file (marks/SERIES.md), with the parts for
the later network runs, where a difference seen only on the Arm load machine is reported and decides nothing; those
parts are built now too, tested on made-up logs only. The same change fixes JDG-ONLY-1: the judge's option that
narrows a run to some marks can skip a precondition mark (today a rule of use covers it). This changes the judge's
code, not marks.json, a pass mark or a number. Each computation gets pass, fail and gap test files, and the
test-engineer and the rust-reviewer read every judge line.
(a) Yes, built inside U4h, after the rest of its build (its first half then reaches about 7,100 lines at U4f's 1.5,
    past your 6,000).
(b) Yes, as its own small change (the judge only, about 400 to 900 lines; stop past 1,300), reviewed and merged before
    U4h's judged runs. It shares no file with U4h, so it need not wait for U4h's build.
(c) No: the hit-registration and throw checks stay not-run at this stage, and JDG-ONLY-1's fix comes back to you
    alone.
My lean, and the advisor's: (b). Only U4h's judged runs wait for it.
```
- **Whose:** all three his under TEST-FIRST: Q4 a pass mark's meaning, U4H-JUDGE "any change ... to the judge", U4H-SIZE
  a change of the plan's units (rule 2's four conditions fail on "inside your approved plan"). The figures are mine,
  rounded; rev 2's recomputed sums replace them in the texts if they differ, nothing else changes.
- **Which day:** not 2026-11-21's (it already carries F-B-COUNT and U4G-Q1; his U4G-Q1 answer frames U4H-SIZE, and
  rev 2's figures are not in). The first day's message after rev 2 is checked and his U4G-Q1 answer is in: 2026-11-22 at
  the earliest, all three together, with my Q1-Q3 and Q5-Q12 rulings named under its choices. Nothing of U4h waits for
  them before U4g (or U4g2) is merged and its sets are done; an answer by then lets U4H-JUDGE (b) be built first.

## What rev 2 must change (REVISE: the exact changes)
1. ADV-U4HB-1: TC's own weapon (spread 0; magazine and reload met), its header, its test; crates/sim's lane gains it.
2. ADV-U4HB-2: the shot id, press-edge firing, reruns and refused presses counted, the contract case.
3. ADV-U4HB-3, -4, -7: U4H-Q2, Q5 and Q6 folded as ruled.
4. ADV-U4HB-5: the room-side wait cap, stated, with its two tests.
5. ADV-U4HB-6: crates/sim's two pub items (the shared blend-and-cast, the fire-origin height), A19's list.
6. ADV-U4HB-8, -9: section 6 recomputed; section 8's three texts as above (figures from the recomputation); blocks
   reordered so the judge's block is a change of its own under U4H-JUDGE (b), and the lag half's last block keeps the
   no-answer line.
7. Rulings Q1-Q3, Q5-Q12 folded, one "Ruled" line each pointing here; Q4 "his, text: advisor-u4h-brief.md".
8. Lows -10 to -13 in the same pass. Item 1's register lines are the main session's, before launch.
9. Rev 2 comes back to me for sections 1-4, 6 and 8 only.

## What I did not check
- I ran nothing. The spread's effect is arithmetic on world.rs's formula and scene.rs's positions, not a run; the
  brawl's replaced share under U4h depends on a default rev 2 must state; my size figures are rounded judgements.
- Whether TC's lines of fire cross any door or the wall stand-in exactly (I read the greybox's building ring only);
  whether U4g's build takes EA-U4R-4 or changes the brawl's stamp; every stamp's sha256 beyond marks.json and
  SERIES.md; u4f-brief.md's sections the brief points to beyond their cites.

## Decisions needed
- **U4H-Q4** (his): what -50's "honest twin" is. Lean (a), the server's own estimate for an honest client at the same
  true RTT and announced delay. Text above.
- **U4H-SIZE** (his): split U4h in two or build it whole. Lean (b), split. Text above.
- **U4H-JUDGE** (his): the judge's computations for U4h and JDG-ONLY-1's fix, inside U4h, as their own small change, or
  not now. Lean (b), their own change. Text above.
- All three in the first day's message after rev 2 and his U4G-Q1 answer (2026-11-22 at the earliest), none a stop.

## For Mustafa
Nothing now from this check: the three questions ride a later day's message, after rev 2.

## Rev 2 check
### Plan
1. Read u4h-brief.md rev 2 whole (its change list first) and the notes' "## Rev 2" block.
2. Check sections 1-4, 6 and 8 against my list (:299-311 of this file), section 9's eight register items, and the
   drafter's added change (TC's own health) against world.rs.
3. Rule the two open points: (a) Q6's stand-in source (a scene builder in crates/sim or TC's own data); (b) SERIES.md:211's
   writer cell.
4. Check the new cites and figures (the wait cap, section 6's sums, section 8's texts).
5. Verdict, findings ADV-U4HB-14 on, the texts' finality and day, the eight register lines word for word.

### Notes (as I go)

