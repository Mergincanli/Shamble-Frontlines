# E2: Stop B's message to him (DRAFT 0.2, 04:3x UTC on 11-28; sent after X, B6's read of X first; one screen)
Status: skeleton from msg-stopb.md, replan.md:14-45, replan-units.md:55-66, history.md:7866-7872 (LEAN-MODE B) and
:8027-8044 (his 19:14 item 2). 0.2: sections 1-4 checked against today's files after the advisor tool's read of 0.1's
fills (U6j merged, npt-super not a stub, the killed-room figure built, the control socket local). <TO FILL> marks what
is still open; trim to one screen before it goes. The advisor tool reads it before it goes.

---
**Stop B: R1's run plan is ready for your review (R1 stays Thursday 3 December).**
R1's path is built and merged (U6a, U6b, U6c, U7, U8, with U8-F1 and UM-4). Stop B's fills are in: Y0 (the two
security notes), Y' (R1's expected page files), X (the commit and the certificate terms' URL in both cloud-init files).
Main was taken in three times this week; PR #40 holds no kit, CI or dependency change against main.

**1. The pieces still on my lane** (estimate from the re-plan; likely = 3.3 times, the growth of the last four units):
| Piece | Estimate | Likely | Needed by |
|---|---|---|---|
| U4k, U4's kept server rows | 540-1,280 | 1,782-4,224 | R2, R3, R5 |
| U6d, room work | 1,800 | 5,940 (just under the 6,000 stop) | R2, R3 |
| U6e, control, metrics, rooms | 1,575-1,825 | 5,198-6,023 (its top end passes 6,000) | R2, R3, R5 |
Dates (a forecast at one unit merged a day, the pace since 11-23; to re-check before sending): R1 Thu 3 Dec, its report
Fri 4; U4k Sat 5-Sun 6, U6d Mon 7-Tue 8, U6e Wed 9-Thu 10 (U6j, the judge's figures, merged 11-26); R2 about Fri 11
Dec (needs U6d, U6e and U4k's hit rule), R3 about Mon 14 (U4k's recovery and ack fix), R5 about Wed 16, the report
Fri 18. The close holds with no slack: one lost day (a cut, a stop at 6,000, a run plan waiting on you) moves it past
18 Dec; with (a) it holds with one day spare.

**2. What each would save** (none decided):
- (a) R5 to Slice 6, its eight marks MOVED (-27, PM-0006-13, -14, -16, -21 to -24): one run day with its plan and
  report (about 1.5 days), EUR 3.34 planned (cap 3.60), and at least about 1,650-2,475 likely lines: U4k's "before R5"
  part (660-1,485) and U6e's H3, npt-super's several rooms and room kill (300, about 990), plus the R5-only parts of
  U6e's S5 (the metrics exporter, the soak's log checks) and S9 (the stall injection), which the estimates do not split.
- (b) one reviewer per piece, security only where a part faces the internet during a run: today three seats a round
  (U6c: netcode, test and security, two rounds); under (b) one, two where client bytes arrive: 2 to 4 fewer reviewer
  runs a piece. It saves usage, not days: a round's seats run as one batch. No lines.
- (c) a stop at twice each estimate (U4k 1,080-2,560, U6d 3,600, U6e 3,150-3,650): at 3.3 times every piece reaches its
  line before it is done, so it adds stops for your word and saves lines only where you then cut.

**3. My lean** (draft, the advisor tool reads it first):
- (a) yes: R1-R3 prove the netcode; R5's rooms-per-VM and soak are hosting-cost evidence Slice 6 needs on the real
  server anyway, and it buys the close its one spare day.
- (b) yes as worded: one netcode reviewer per piece, the test reader folded into the advisor's head read. Security stays
  where RS's open ports take client bytes (rs.yaml:43-49): on the game ports, U6d's join change (it checks a session's
  first commands; finding SA-F1-1 is still open) and U4k's ack fix (room/src/ack.rs acts on clients' acks); on port 9100,
  the metrics exporter, only if R5 stays. The control socket is local to RS (plan.md:63), so it leaves the seat, though
  it is the answer to a Critical security finding (F1).
- (c) no: at 3.3x every piece would hit its line, adding stops without saving work; keep 6,000 and report each piece's
  size against its estimate at its merge.

**4. Pieces Netcode Builder 2 could build alone** (your 19:14 item 2; checked against today's files; its MOVED and U6j
are merged, 11-26 at 48efc9c5):
- B3, -30's container runner (U6e): fence ops/runs/, one new c-*.mjs (the container's run scripts live there:
  plan.md:410; U4k's (c) edits another file there, c-u4r-runs.mjs); planned 225 (150-300; about 740 at 3.3x); waits for
  nothing: the binaries and npt-load are merged, and J1, which its counted run also needs, came with U6j
  (replan-marks.md:50). Needed by R3 (-30's judged container part).
- H3, npt-super's rest (U6e): fence crates/super/ (U6a built its start timer and hold form there, 653 lines) plus one
  change in crates/server/: -27's built figure reads the room's ticks in phase `after` (figures.rs:1691-1692), and the
  room writes ticks as warm-up or measured only (tick_loop.rs:555-559); alone once that change lands on my lane.
  Planned 300 at most (the estimate predates U6a taking the timer), about 990; the security seat confirmed rooms as
  npt-super's children on 11-23 (u8-stopb-security.md:156-173). R5 only: with (a) it moves.
- Not alone: U6d, U4k, and U6e's S5, S7 and S9 change crates/room/, crates/proto/ or crates/server/ together.
With (a): B3 alone. Without (a): B3, and H3 after its server change.

**5. One line each:**
- The expected page list's Wasm carries wasm-bindgen's plain version, as RS will build it. A first list carried our own
  commit's name through wasm-bindgen's build script and was voided before the fill. The day's dist compare stays the wall.
- R1.md:321 still says "no reading exists yet" for the probe's window: one exists (u6c-r2-netcode.md:90-93); named
  here, not edited.
- If bring-up finds the tick core shared (`alone` 0), the DelayStream self-test stays skipped (r1.sh:38, R1.md:243-245):
  its clock thread never runs on the tick core, but whether its numbers lean on the tick thread is not yet read; a
  netcode follow-up after R1 (adv-u8h-3-read.md).
- The count: your two notes add 2 lines to U8's files, so U8 stands at 6,284, your stop; the two test stand-ins were
  rewritten in place at 0 net (if you count in-place lines, they need +2 from you); the fills (the page list at Y', the
  commit and URL at X) sit outside the stop by your U8-FILLS (a).
- The page host: HOST-LIMITS and HOST-SANDBOX <TO FILL: his answers, or still open>.
- FSN1 (R1.md's plan for both machines) is not listed on Hetzner's pages for CCX23 or CAX21 today (CCX23: NBG1, HEL1,
  HIL1, ASH1, SIN1; CAX21: NBG1, HEL1): an early warning only; your console decides on the day.

**Decisions needed** <TO FILL: (a), (b), (c) each with options and my lean; HOST-LIMITS and HOST-SANDBOX if open;
choice 45 (RS's `npm ci` audit call) if open>.
**For Mustafa** <TO FILL: review R1's run plan (R1.md) and its cap EUR 0.80 with the delete-by time; the "Once" list
with X's raw URLs; your OK of R1's plan and cost cap>.
