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
  the largest allowed delay" (ADR 0001:1647; r2-netcode.md:2186-2188). SERIES.md:153 and :187 define the gain against
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
