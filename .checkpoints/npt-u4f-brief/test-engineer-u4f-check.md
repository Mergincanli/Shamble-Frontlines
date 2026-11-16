Task: night 2026-11-16 item 7 (npt-u4f-brief), test-engineer check of rev 1 of U4f's build brief (tests, negative cases, bites, runs, counts, carries).

(No Edit tool in this agent: the notes file is rewritten whole with Write at each save.)

## Plan (blocks, in order)
- B1-B8: DONE (the approved plan at 7812127; the draft, 837 lines; the drafter's notes; the scout's map; N/u4-build-plan.md; N/u4s-carries.md; marks.json -20/-21/-22/-47; marks/README.md :180-330; ADR 0001 :224-257, :1610-1612, :1644; N/plan.md :456-495, :506-525, :594-629; N/advisor-u4.md :76-160, :326-335, :455-479; N/advisor-u4r-brief.md :186-205, :289-359).
- B9: DONE (code and table spot checks, Read/Grep only).
- B10: DONE (this check).

## Notes (one line per block)
- B1-B8 done; B9 done (facts cited inline below); B10 done.

---

# Check of U4f's brief, rev 1 (test-engineer)

Read at 11fd190 (the worktree .gate/npt-spike-1116, read only). Draft lines are `D:<n>` (.checkpoints/npt-u4f-brief/u4f-brief-draft.md);
`N/` is .gate/npt-spike-1116/.checkpoints/netcode-proof-test/; `crates/`, `marks/`, `ops/` are under
.gate/npt-spike-1116/spikes/netcode-proof/. No build, test or run was made (night rules): a claim only a run settles
is marked "needs a run (day)" with its command. No finding below is carried in N/u4s-carries.md (each checked against
the U4f section :505-537 and the entries the draft places).

## Verdict for the day session: REVISE (one High, fourteen Medium, nine Low; none Critical)

The shape (items 0-12 of the night plan) is complete, the carry placeholders and their line ranges are right, and the
run counts match the plan. What needs a revision: the command-check order contradicts the draft's own repeat test
(TE-1); several checks or bites cannot fail, or may not, as the suites are worded (TE-4, TE-5, TE-6, TE-7); some
tests have no pass condition or no route (TE-11, TE-12, TE-15); the counts are one off in the measured build (TE-9);
the disk arithmetic mixes MB and MiB (TE-16).

## Counts

| Item | Present | Missing | Untestable or unable to fail as worded |
|---|---|---|---|
| Tests f1-f30 (D:254-285) | 30 rows | 5 kinds: per-kind attribution in the lab (TE-23), honest stamps strictly rising (TE-15), carry 39's 30/60 Hz pins (TE-10), the honest reordered command (TE-2), the conditional tests a ruling adds (TE-9) | f5 (no pass condition), f6 (cannot fail), f24 (no bound named), f25 (no failing route); f12 contradicts 2.4's order (TE-1); f28 cannot hold for fast-clock at r2's length (TE-13); f21 half duplicates r9 (TE-17) |
| Δ measured / bites (D:249-252) | +60 / +74 claimed; the rows add up to 60 | | f30 is bites-only, so measured is +59, bites +74 (TE-9) |
| Negative cases | 44 claimed; the neg rows add up to 44 (f3 2, f4 1, f7 1, f8 1, f10 8, f11 2, f12 1, f13 1, f14 5, f15 11, f19 1, f20 1, f21 2, f22 2, f23 1, f24-f27 4) | a forged command refused under another kind counts as unrejected (TE-23); the honest reordered command (TE-2) | |
| Bites (D:290-317) | 17: -20 1, -21 13, -22 1, -47 2, ids spelled as marks.json (:4768, :5061-5113, :5194, :9097, :9101); each with site, plant, unit proof, judged run | a short bitten run per suite before the judged sets (TE-6) | not-older-than may not bite under U4F-Q7 (b) (TE-5); tick-quantised may be masked by a lowest-RTT filter (TE-6) |
| Judged runs (D:460-477) | 36 measured + 12 twins + 17 bites = 65; matches plan §9 (N/u4-build-plan.md:962-963: -20 6, -21 24 + 12 twins, -22 3, -47 3) | -20 P3 option B with U4f's suite on a short run (TE-3) | the lag-switch parts' check (TE-7); carry 74's late share under option A (TE-4) |
| Carries (D:357-377) | 14 placeholders; every range checked at 11fd190: 46 :506-509, 80 :510-534 ((a) :513-518, (b) :518-520, (c) :520-522, (d) :522-529, (e) :529-534), 86 :535-537, 57 :223-228, 74 :343-354, 21 :148-149, 27 :166-168, 18 :129-132, 39 :169-179, 50 :180-183, 61 :229-247, 58 :269-273, 76 :364-371, 77 :372-381; conditionals 63 :322-327, 64 :447-473, 67 :476-493; 62 (h) from :305 | | 80 (b) (TE-11), 80 (c) (TE-12), 74 under A (TE-4), 39 (TE-10) |

**Budgets.** The judge memory adds up (2,744 MB = 2,617 MiB; 302.2 + 2.3403 x 2,617 = 6,427 MiB, under 10,240:
ops/runs/c-u4r-runs.mjs:176-178), and so do the wall times (-21 49 x 263 s = 3.58 h to 49 x 355.4 s = 4.84 h; -20 7 x
335 s = 39 min). The disk total does not: 5.5's rows add up to 392 + 2,744 + 336 + 145 = 3,617 MB, not "about 3,750",
and the comparison with "3,593 MiB" mixes units (TE-16).

## Findings, most severe first

**TE-1 · High · D:143-154 (the counting rule, kind 2 second, kind 3 third), D:267 (f12), D:720-723 (U4F-Q9 (a)) ·
source crates/room/src/input.rs:80-89, :107-113; crates/client/src/lib.rs:443.**
Today the sequence compare runs first (input.rs:81-83), and that is why an honest redundant copy is a `Repeat`
and not a refusal. Each `Input` re-sends up to `REDUNDANCY_CAP` = 8 unacknowledged commands (client lib.rs:443), so
most commands reach the room more than once, and every copy of a command already run has a tick equal to or older than
the last run. In the draft's order, kind 2 ("tick equal to or older than the last run", 2nd) refuses each such copy
and counts it before kind 3, the place U4F-Q9 (a) drops identical repeats uncounted, ever sees it. The scenario: either
f12 ("an honest redundant repeat is not refused") fails, or the builder reorders the checks without a ruling. If the
order ships as written, every honest client's kind-2 counter holds thousands of copies a run, and NA-7's sentence
("a forged old tick never as late, an honest late command never as forged") holds only for true late commands. Fix
for rev 2: say that the identical-repeat drop runs before every counted check (kind 1 included), and keep f12 as
the proof. Cheapest proof: f12 written against the table's order, failing as described; or the day session reading
input.rs:80-113 against D:150-163.

**TE-2 · Medium · D:266-267 (f11, f12); D:720-723 · source marks.json:4680-4682 (-20's `reorder` rule);
marks/README.md:276-277.** An honest command that was lost or delayed and arrives after a later one has run is not
newer than the last run, and the room never received it under its sequence before. U4F-Q9 (a)'s rule ("content differs
from the command the room received under that sequence") has no answer when nothing was received. -20 needs a reorder
share above 0, so this case comes up in every -20 run. The scenario: honest reordered commands get counted as forged
kind 2 or 3, under a kind no test defines. Cheapest proof: a room unit test (L1's reorder order: c2 run, then c1
arriving) that states the kind expected and, if the answer is "refused", that it is counted apart from the forged
kinds.

**TE-3 · Medium · D:483 (5.2's "-20, option B" row: "-20's own suite (U4F-Q11) is not the brawl: 5.3 runs it") against
D:494-496 (5.3 runs only -20's P3 option A) · source N/advisor-u4.md:119-124 (condition 5); N/advisor-u4r-brief.md:198-205
(U4R-Q4); ops/runs/c-u4r-r2.mjs:80.** U4r showed -20 P3 option B's precondition with the brawl at a fixed lead
(`with_lead`, lead 7). U4f changes both inputs of that run: the suite (U4F-Q11 (i): presses, for the 10,000-press
check) and the lead (the controller replaces `with_lead`). The note says 5.3 covers it, but 5.3 does not. The scenario:
-20's option-B marks count with PM-0001-2, the shot band and carry 74's late share never shown on the suite and
controller that actually run. -21's option-B suites are at least asked (U4F-Q11 (iv)); -20's B is not. Cheapest proof:
add "-20 P3 option B, U4f's suite and controller" to 5.3 (one more 30 s run, about 7 MB by c-u4r-r2.mjs:80's 6.5 MB).

**TE-4 · Medium · D:126 (2.2: "carry 74's 1 % late rule on every short run (5.3)"), D:496-497 (5.3), D:135 (2.3:
option A has "no late rule") · source N/u4s-carries.md:352-354 (carry 74: "the commands that arrived late (the room's
`late` count)"); marks/SERIES.md:61.** Every 5.3 run is under option A, where the room keeps no `late` count by the
draft's own 2.3. Carry 74's 1 % check therefore reads 0 on every 5.3 run and cannot fail. The controller (U4f's
replacement for `with_lead`) is the thing carry 74 guards, and it runs on no short option-B run. The scenario: a
controller that settles one tick short passes every short run and first shows at -20's judged option-B set. Fix:
under A, read carry 74's "late" as commands arriving after their stamped tick ran (1 - `tick.s.cmd_on_time`'s share,
SERIES.md:61), and add the option-B short runs (TE-3; U4F-Q11 (iv)). Cheapest proof: needs a run (day): 5.3's runs
reporting both readings.

**TE-5 · Medium · D:309 (not-older-than's judged run), D:191 (the fast clock "with the stamps its controller gives"),
D:696-703 (U4F-Q7) · source marks.json:5107-5108; docs/adr/0001-multiplayer-netcode.md:231-233.** The bite fails
-21's fast-clock check only if the fast client sends several commands with one stamped tick ("equal ticks would let it
run the per-tick cap on every tick"). Strictly newer ticks already bound the commands run by the highest stamp, and so
by the lead bound. Under U4F-Q7 (b), the recommended option, a client stepping at 1.5x whose stamp follows ClockSync's
1.5x estimate can step exactly one tick per command, which gives no equal stamps. The relaxed check then changes
nothing, the bite run passes, and the mark is FAIL ("the bite did not fail", README :320-321), found only after the
-21 set. The unit proof ("an equal-tick command runs") shows the state changes, not that the suite reaches the site.
Fix: 2.6 fixes the fast clock's step rate against its stamp rule so that equal stamps occur under every U4F-Q7 option,
and U4F-Q7's "why it matters" names this bite. Cheapest proof: needs a run (day): one short bitten fast-clock run
(option A, one full 60 s episode) showing `event.fast_clock.excess_cmds_count` above 0, beside the clean run at or
below 0.

**TE-6 · Medium · D:290-317 (2.11), D:492-499 (5.3), D:109 (2.1: "over the lowest-RTT samples"), D:763 (D4) · source
ops/runs/c-u4r-r2.mjs:88-91 (r2 ran U4r's new bites short); marks/README.md:317-326.** None of U4f's 17 bites is run
short before its judged set. U4r's r2 did that for tick16, no-fallback and whole-as-lost. A bite that does not fail
costs a whole set and makes its mark FAIL. The highest risk is -47/tick-quantised: the bite delays each sample's
receive time by up to one client step, and a lowest-RTT filter keeps exactly the samples with the least added delay,
so the p99 may stay under 5 ms. D4 models only the Pong's server-side quantisation, never the bitten arrival times.
Fix: 5.3 gains one short bitten run per suite (forged A with one no-* bite, clamp with refuse-not-clamp, fast-clock A
with not-older-than, clock with each -47 bite, -20 P3 with redundancy-1), and D4 also reports each estimator's p99 with
the bitten arrival times. Cheapest proof: needs a run (day): D4's model extended (arrival + U[0, P) at the client, P =
33.333 ms), then the short bitten clock runs.

**TE-7 · Medium · D:192 (the lag switch), D:467 (5.1's lag-switch row), D:275 (f20) · source marks/SERIES.md:34;
marks.json:5036-5047, :5061-5113, :5118; N/advisor-u4r-brief.md:322-323 (the bots' one-command-per-moved-tick rule);
crates/room/src/input.rs:61.** With the honest command rate (one command per newest-tick move, about 30 a second), a
200 ms hold holds about 6 commands, fewer than `CMD_CATCHUP` = 8. So `tick.v.lagswitch_excess_move_um` (movement
minus 8 ticks of top speed) stays at or below 0 even with the catch-up cap removed, under option A, and option B runs
one command a tick anyway. The lag-switch parts have no bite of their own: no-catchup-cap runs in the forged suite. As
worded, the check passes in a build with no catch-up cap at all. The draft does not set the lag switch's command rate.
The numbers are ADR 0001:1611's "(proposed)", and marks.json is not U4f's lane (D:421). Route it to the advisor as a
question: (a) the lag switch gives a command at every client step, so a hold carries more than 8; (b) a hold of at
least 10 ticks (334 ms); (c) the check reported as unable to fail at 200 ms. Cheapest proof: arithmetic, then needs a
run (day): one short lag-switch run under the no-catchup-cap bite, reporting `tick.v.lagswitch_cmds_count` max (at most
7 expected).

**TE-8 · Medium · D:735-745 (U4F-Q11 (iii), recommendation (a)) · source docs/adr/0001-multiplayer-netcode.md:1611
("moves no further than an honest client on the same inputs plus that many ticks of movement"); marks/SERIES.md:156;
N/u4-build-plan.md:970-976; N/advisor-u4.md:150-153.** Option (a), "server ticks plus the lead bound, times the top
speed per tick", is looser than the row's honest-client reference unless every fast-clock input moves at top speed in
a straight line: an exploit that moves the client further than honest, but less than top speed, would pass. The option
is a reading of an ADR row's words, which the plan sends to the readings PR, and the question does not say so. Nothing
is decided here; the question should state the loosening and the readings-PR route, and keep (b), or (a) only with
inputs fixed at top speed so the two coincide. Cheapest proof: none needed beyond the reading; a unit test on a
fixed-input episode shows whether (a) and (b) give the same number.

**TE-9 · Medium · D:249-252 (minimum deltas), D:285 (f30) · source N/u4-build-plan.md:193-194 (`with_bite`: "bites
builds only"), :399-401, :914-915; crates/client/src/lib.rs:311-315; crates/room/tests/room.rs (no parallel-thread test
today: Grep finds only the Send assert, :226).** f30 builds a bitten and a clean instance through constructors that
exist only in bites builds, so it cannot run in the measured build. The deltas are measured +59 and bites +74 (59 +
14 + f30), not +60 and +74. The builder's honest +59 then reads as below the brief's "minimum", which §7.1 item 3 treats
as a stale artifact (touch, rebuild, never report). 2.10 also lists no conditional deltas for rulings that add tests:
U4F-Q6 (b) (the Pong's new field: codec round trip, the 32 B limit, the catalogue's test), U4F-Q8 (carry 18's s1 pin, or
carry 39's pins), U4F-Q9 (a) (the bound on the last-8 store: .claude/rules/netcode.md). Cheapest proof: D1, then the
builder's `--list` in both builds.

**TE-10 · Medium · D:367 (section 3's row 39: met by "takes the rate as an argument"), D:254-285 (no pin test) · source
N/u4s-carries.md:169-179 (carry 39: "a test pins each at 30 and 60 Hz"); N/advisor-u4.md:458-460 (the F2 ruling's
point 2 pattern).** The rate-taking values U4f adds (the lead bound's ms to ticks by U4F-Q8 (b); the fast clock's 60 s
and the lag switch's 200 ms in ticks; ClockSync's tick period in ns) each need a test pinning them at 30 and 60 Hz.
2.10 has none. The scenario: the report lists carry 39 as met with no test behind it, and a 60 Hz value stays
unpinned. Cheapest proof: one test per value at 30 and 60 Hz, counted in 2.10.

**TE-11 · Medium · D:279 (f24: "carry 80 (b)"), D:240 (2.9 (b): "U4f designs no lost-tick bound") · source
N/u4s-carries.md:518-520 ("files its sentinel without moving its newest tick past a bound, with a test");
N/advisor-c62h-review-full.md:75-80.** Carry 80 (b) needs a bound on how far a refused header may move the client's
newest tick. That bound is not C62H-BOUND's lost-tick bound, which is Mustafa's open question. The draft names neither
the bound nor what f24 asserts, and "designs no lost-tick bound" invites the reading "no bound at all". The scenario:
80 (b) is reported met with a test that cannot fail, or not built. Fix: f24 names its bound (for example, a refused
header moves `newest_tick()` not at all, or by at most `ACK_RING_TICKS`), or rev 2 asks it as a U4F-Q, keeping it
apart from C62H-BOUND. Cheapest proof: an endpoint test feeding one refused header at newest + 2^20, asserting
`newest_tick()` and the lost count.

**TE-13 · Medium · D:283 (f28), D:494 (5.3 at r2's length: 10 s warm-up, 20 s measured) · source marks/SERIES.md:155
(one `fast_clock` event per 60 s episode); N/u4-build-plan.md:412-415 (every check of every U4 L1 part resolves).** A
20 s measured run holds no complete fast-clock episode, so `event.fast_clock.excess_cmds_count` and `.excess_move_um`
are never written on that suite's short run. f28's "every series written ... on a short run of each suite" and the
contract test's "every check resolves" cannot both hold for fast-clock. -47's 2 s-per-connect suite and the lag
switch's 100 bursts need lengths stated too. Fix: 5.3 and f28 give each suite's measured length (fast-clock at least
one 60 s episode plus its settling time), or an episode-length setting recorded in the header (a choice: ask it).
Cheapest proof: needs a run (day): the fast-clock short run's contract output.

**TE-15 · Medium · D:260 (f5: "against U4R-Q14's numbers"), D:127 (2.2's bots' rules "unchanged"), D:696-703 (U4F-Q7)
· source N/advisor-u4r-brief.md:296-302, :315-321.** f5 has no pass condition. U4R-Q14's leads are worst-case
arithmetic, and a controller driven by samples settles elsewhere ("equal"? "within 1"? "never above"?). And (b1)'s
strictly rising stamps rest on stamps being relative to the newest tick, with the bots skipping a step whose newest
did not move. Under U4F-Q7 (b) a stamp comes from ClockSync's estimate, whose slew can give two commands one stamp or
step a stamp back. An honest command is then refused under kind 2 (option A) or counted late (option B), and nothing
tests for it. Cheapest proof: f5 restated with a pass rule, plus a property test that over seeded P0-P4 and B1 sample
traces the honest stamps rise strictly and stay within the lead bound.

**TE-16 · Medium · D:519-525 (5.5), D:650-657 (U4F-Q2), D:829 (NA-5) · source ops/runs/c-u4r-runs.mjs:132, :153; the
scout's Disk entry (7,689 MiB free).** 5.5's rows add up to 3,617 MB (392 + 2,744 + 336 + 145; -47 has no figure), not
"about 3,750 MB". The room above the floor is 7,689 - 4,096 = 3,593 MiB, which is about 3,767 MB. The summed estimate,
3,449 MiB, fits with about 140 MiB to spare (and even "3,750 MB" is 3,576 MiB, under 3,593) before builds, -47's runs
and -20's extra series. NA-5's "below the sets' total" therefore does not hold as written. Its practical conclusion,
no real margin, does. U4F-Q2 (a) would tell Mustafa now on this number. Fix: one unit throughout, the total restated
from the rows, and "no margin" in place of "cannot". Cheapest proof: D2's `df -BM` and `du -sm` with the arithmetic
redone.

**TE-23 · Medium · D:143-148 (the counting rule), D:254-285 (no lab attribution test) · source marks.json:4932-4943;
marks/SERIES.md:158-159; crates/server/src/pump.rs:125-128 (as the draft cites: per-session counts, no per-command
record).** -21's forged check reads `unrejected_count` = sent minus "rejected and counted" per kind, and the draft has
npt-lab count a forged command only when a record names its own kind. No test shows that attribution for each of the
11 kinds. Kind 7 comes from the sim's `reports()`; kinds 9 and 11 can only be matched by count, since the pump keeps
counts, not (slot, seq, kind) records. There is also no negative case: a forged command refused under a different
kind must count as unrejected. f10, f11 and f14 stop at the room's or the pump's counters, and f28 only shows the
series written. The scenario: a lab that credits any refusal to the forger's kind passes every test, and -21's main
check is never shown to discriminate. Cheapest proof: a lab test per kind (11 cases) plus one wrong-kind negative case.

**TE-12 · Low · D:280 (f25) · source N/u4s-carries.md:520-522.** No route is named for making one command fail to
encode through the endpoint's input, and no name is given for the new count (2.13 lists no accessor for it). The test
may be unwritable without a bites-only hook. Cheapest proof: the builder names the failing input (for example an aim
outside the codec's range) and the counter, in 2.10 and 2.13.

**TE-14 · Low · D:261 (f6) · source marks/README.md:191-192 (`cmd_timing` is the room's or the scenario's setting).**
The endpoint is never told which option runs, so "the same stamps under A and B" cannot fail unless the endpoint reads
the option. What is worth testing is that the lab builds both options' endpoints with the same controller settings.
Cheapest proof: `grep -rn cmd_timing spikes/netcode-proof/crates/client` (none expected), then f6 restated.

**TE-17 · Low · D:276 (f21, Δ 2) · source crates/lab/src/scenario.rs:2172-2175 (r9, U4r's: a kicked never-acker's later
datagrams and `Leave` reach no room call).** f21's carry-76 half duplicates r9. Count only the never-acker Behaviour's
new test, or name r9 as kept. Cheapest proof: `cargo test -p npt-lab -- --list` at `<base>` (D1).

**TE-18 · Low · D:323, D:325 (2.12) · source N/u4-build-plan.md:385-388.** Kind 10 appears both in the "forged command
left out" row and in the "repeats only" row. Read together, the twin keeps the first datagram (the 8 accepted
commands) and sends repeats only in the second. A builder applying either row alone gives twin hashes that differ;
f16 would catch it, but rev 2 should say it.

**TE-19 · Low · D:585 (recheck row 1) · source crates/room/src/input.rs:10-13, :82, :98; N/u4-build-plan.md:829-831.**
-58's bites lt-cmd-seq and lt-press live in `InputBuffer::offer`, which U4f rewrites. Their judged runs are U4g's. The
brief should say that their sites, counter names and unit proofs stay. The bites build's tests would catch a break,
late.

**TE-20 · Low · D:207-213 (2.7), D:701-703 (U4F-Q7 (i)) · source marks/SERIES.md:342.** `header.params.lead_ticks_count`
is defined as "the command lead every client of the run stamps with". Under the controller no single lead exists, and
the draft neither retires it nor redefines it for U4f's runs. The scenario: the judged runs' headers carry a stale
profile lead. Cheapest proof: the contract test asserts that it is absent, or equal to a fixed lab setting, under the
controller.

**TE-21 · Low · D:139 (2.3, carry 80 (a)), D:298 (no-lead-bound) · source N/u4s-carries.md:513-518.** If carry 80 (a)
is met by "the lead bound refuses first" rather than by a total key, the no-lead-bound bite run removes that guard.
Three or more of one slot's commands spread around the tick circle in one option-A tick could then reach a non-total
sort, which may panic. The bite run would crash, and README :250-251 makes a crashed run INVALID. The likelihood is
low. Cheapest proof: f8 also run in the bites build under no-lead-bound, or the total key taken.

**TE-22 · Low · D:487 (5.2's -22 row), D:556-577 (the return list) · source N/advisor-u4.md:116-117 (condition 4).**
"U4f's report states the load each run carried" concerns the judged -22 runs, which are the main session's after the
merge. The return list has no item for it, and no set report names who writes it. Name the main session's -22 set
report as the place.

**TE-24 · Low · D:287-288 ("the runner's twin test" kept) · source N/u4-build-plan.md:406-407.** -21's twin figure
assumes that the forged run and its twin run identical honest streams across two processes. The runner's twin test
covers one scenario. No same-seed identity check covers a new suite (forged, clamp) before its 3 + 3 twin pairs run.
Cheapest proof: needs a run (day): one forged-suite short run twice at one seed, with the fate-log and per-tick hashes
compared.

## The drafter's findings (NA), as checked

| NA | Holds? | Why |
|---|---|---|
| NA-1 | confirmed | carries :535-537 has no quotation marks |
| NA-2 | confirmed | SERIES.md:93's writer is npt-server; it is a report (marks.json:5182-5190), not a check |
| NA-3 | partly | the arithmetic matches for a 0.7 s margin, but plan :335's margin is free, so "at most about 1,008" holds only for margins up to about 0.73 s; the overflow observation (b1-p3 :158) was not checked |
| NA-4 | not checked | netcode's; TE-6 adds that D4 omits the bitten case |
| NA-5 | partly refuted | TE-16: units mixed, total not the rows' sum; "no margin" stands |
| NA-6 | confirmed | input.rs:81-83: a sequence not forward is `Repeat`, the same path as honest copies |
| NA-7 | confirmed in part | input.rs:107-116: no lead bound; `Full` at 8. Its remedy (strictly newer before late) needs TE-1's repeat-first order |
| NA-8 | confirmed | bots/src/lib.rs:76 `FAR_CM` 32,000 against sim/src/greybox.rs:62 `HALF_MAP` 12,800; the aim's frame at bots :152 was not read in context |
| NA-9 | consistent | marks.json:5086-5088 and :5110-5112 name sim-side checks (plan §4.2 :239-249); the sites were not grepped |
| NA-10 | not checked | te-m3 not read |
| NA-11 | confirmed | plan :810's words against marks.json:9097 and :9101 |
| NA-12 | confirmed | 460 x 255 / 30 = 3,910; carries :180-183 ("never queues one past that age") |

## Scope

Nothing found outside the plan. The run scripts (c-u4f-*.mjs), `--twin`, the lab's -47 suite and the per-tick refusal
records all trace to U4f's row, carry 86 or the marks. The one protocol change, U4F-Q6 (b), is asked, not taken; if it
is ruled, 2.10 owes its codec tests (TE-9).

## Needs a run (day), from this check

1. TE-4, TE-3: 5.3's short runs with -20 P3 option B added, each reporting the room's `late` count and 1 -
   `tick.s.cmd_on_time`'s share, through the lane runner (`bash <u4f-run.sh> -x ...`, commands in the builder's
   notes as U4r's D13).
2. TE-5, TE-6: one short bitten run per suite in the bites build (`npt-lab scenario ... --role bite --bite <id>`, on
   c-u4r-r2.mjs:88-91's pattern), then `npt-judge --only PM-0001-2,<mark>` (JDG-ONLY-1), each bite failing a check by
   name.
3. TE-6: D4's model extended with the bitten arrival times (arrival + U[0, P) at the client, P = 33.333 ms): the p99 of
   abs(error) per estimator.
4. TE-7: one short lag-switch run under PM-0001-21/no-catchup-cap, reporting `tick.v.lagswitch_cmds_count` max.
5. TE-9, TE-17: D1 at `<base>`, then the builder's `--list` in both builds.
6. TE-16: D2 (`df -BM /home/user/Shamble-Frontlines`, `du -sm /home/user/Shamble-Frontlines/.gate/*`) with 5.5 redone in
   one unit.
7. TE-24: one forged-suite short run twice at seed 1, with the fate-log and per-tick hashes compared.

## Not done

No build, test or run (night rules). Not read: room.rs:795, client lib.rs:810-839, lab/src/logs.rs, lab/src/contract.rs,
N/advisor-te-m3.md, the catalogue. The hostile-input half is the security-auditor's.

DONE
