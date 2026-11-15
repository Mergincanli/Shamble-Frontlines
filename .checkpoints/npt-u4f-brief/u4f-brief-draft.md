# U4f builder brief: clock and commands (rev 1, night draft)

rev 1, 2026-11-16 (night item 7, npt-u4f-brief), drafted by the netcode-architect from the spike branch at
11fd190e9536702f0dcfc72d584b637ebc6184be, read only. Not checked yet: the test-engineer's and the security-auditor's
checks and the revision (rev 2) come next; then the day session brings it up to date (section 12's list) and takes it
to the advisor before U4f's build (N/u4-build-plan.md:1054-1056; N/advisor-u4.md:154-158). Nothing here is ruled:
every choice the plan or a ruling leaves open is a U4F-Q question in section 10, and the brief follows its
recommendation only where it says "by U4F-Qn". The form is U4r's brief, rev 3.2 (N/u4r-brief.md), kept short: tables
and lists, cites rather than copies. The findings met while drafting (NA-1 to NA-12) are listed after section 12.

---

## 0. How to read the citations

- **Read at:** 11fd190 (the spike branch's head at the night's fetch). Every file:line below is at 11fd190. `<base>`
  is the day session's (section 12 item 1): the spike branch's head at launch. The card was edited while the scout ran
  (N/night-u4f-scout-full.md:5 names 0ad41a6), so card items are named by their text; their 11fd190 line numbers are
  given only for section 8's recheck.
- **Short names:** `N/` = .checkpoints/netcode-proof-test/; `plan` = N/u4-build-plan.md; `plan.md` = N/plan.md (the
  test plan, rev 3.3); `carries` = N/u4s-carries.md; `card` = .checkpoints/netcode-proof-test.md; `history` =
  N/history.md; `catalogue` = docs/design/netcode-proof-messages.md; `ADR 0001` = docs/adr/0001-multiplayer-netcode.md;
  `marks.json`, `SERIES.md`, `README` = spikes/netcode-proof/marks/'s; a crate path (`room/src/input.rs`) is under
  spikes/netcode-proof/crates/; `check.mjs` is spikes/netcode-proof/check.mjs; run scripts are under
  spikes/netcode-proof/ops/runs/.
- **The rulings** (closed records, all in N/): `advisor` advisor-u4.md; `wave1` advisor-u4-wave1.md; `e-ruling`
  advisor-u4e-brief.md; `e-r1` advisor-u4e-r1.md; `e-r2` advisor-u4e-r2.md; `r-check` advisor-u4r-brief.md; `r-r1`
  advisor-u4r-r1-full.md; `r-b2` advisor-u4r-build2-full.md; `c62h` advisor-u4r-c62h-full.md; `c62h-review`
  advisor-c62h-review-full.md; `te-m3` advisor-te-m3.md; `df-ruling` advisor-u4df.md; `j-r2` advisor-u4j-r2.md; `m9`
  advisor-u4d2-m9.md; `d2-r2` advisor-u4d2-r2.md; `b1-p3` advisor-brawl1-sets-p3-full.md.
- **Placeholders** (filled by the day session's script, never by hand: the night plan's rule 6 and item 7's shape 3
  and 9): `@@CARRY-<entry>@@` is carries' entry at the line range section 3 gives, word for word;
  `@@WORDS-<file>:<a>-<b>@@` is the text at that range (the quoted span inside it where section 12 item 3 says so).
  Mustafa's words appear only as cites into history by line range, or as a WORDS placeholder there; none is retyped
  or paraphrased as his.
- **Stamps:** the sha256 and line count of plan, plan.md, carries, history and the card at `<base>` are the day
  session's (section 12 item 2); a file whose sha256 moved since 11fd190 has its cites re-walked (section 8).
- **Mustafa's standing rules, by range only:** the shared target, history :411-420; LEFTOVER-DELETE, :404-410; the
  cleanup after each merge, :859-861; RESUME-CARD, WIP-CLEAN, TCP-FREEZE, SQUASH-40, PROMOTE, :379-403; SIZE-CHECK,
  :398-399 and :2039-2087; USAGE-SAVINGS, :291-299; NIGHT-BUDGET, the 9.5 floor and "a Low never pushes", :335-352;
  his words move by script and line ranges, :353-356; SPEND-LIMIT and review_packet's ALLOWED, :1927-1975; CHECK-IN,
  :1990-2012; KEPT-LOGS (c), :2088-2111; C62H-REVIEW (a) and the read rule, :2113-2129 (the rule's line :2127). The
  card's "Rules in force" lists them (card :3013-3088 at 11fd190).

---

## 1. Start

1. **Base:** `<base>` (section 12 item 1). U4f depends on U4r (plan :805), merged; U4f, U4g and U4h run one at a time
   (plan :882-883; advisor :186-187). In your worktree: `git merge --ff-only <base>`; if that cannot fast-forward,
   `git switch -c u4f-build <base>`; then `git log -1 --format=%H` shows `<base>`. Record it in your notes first.
2. **Who and where:** one builder, general-purpose, opus, effort xhigh, in an isolated worktree under
   `/home/user/Shamble-Frontlines/.claude/worktrees/` (`isolation: worktree`; plan §7.2 :925-927), under the shared
   target rule and W1-TARGET's lock (section 4). The judged runs are not yours: the main session runs them after the
   merge, in the main folder (plan :928-929; section 5).
3. **Your notes:** `.checkpoints/netcode-proof-test/u4f-build.md` in your worktree. First line: this task. Then your
   plan as blocks (one block: one type with its tests, one check with its counter, one cheat, one bite and its proof,
   one run script); each finished block written at once, a notes line per finding, one line after each block (done,
   what next). On a restart: your notes and `git diff` first, then go on after the last block marked done.
4. **The runner:** `u4f-run.sh` (section 12 item 4: made by the main session from u4r-run.sh, only the lane's names
   changed, with its .diff; sha256 and `bash -n` at launch). Its use as u4r-run.sh's (N/u4r-brief.md:265-274): `-x`
   for every counted run, bite run and plant; a run's result is its exit code and the test runner's own summary line,
   never a grep (section 4).
5. **What stops you** (each written in your notes and your report, then build the rest): a file outside section 4's
   lane; an item another unit merged changed beyond what this brief names; a manifest or Cargo.lock change (the main
   session's); a new external dependency; disk below section 4's floors; a part a U4F-Q leaves open at launch; a tick
   count or rate-derived constant you need (carry 39's rule: record name, definition, values at 30 and 60 Hz and site,
   build the rest); a wire change beyond U4F-Q6's ruling (the catalogue and its tests move with it:
   .claude/rules/netcode.md:15).
6. **Read before you build** (and nothing you do not need):
   - plan: §5 U4f (:805-816); §4.1's rows for npt-client, npt-room, npt-server and npt-bots (:193-197); §4.2's refusal
     rule (:239-249); §4.3 (:251-265); §4.4 (:267-290); §4.5's window (:301-303); §4.7 (:330-336); §4.8 (:338-341);
     §4.9's tolerance (:351-356, context only: U4h's); §4.10 (:360-390); §4.11 (:392-407); §4.12 (:409-425); §4.13's
     shot band (:437-447); §4.14 (:453-476); §5's preamble (:507-512); §5 U4e (:714-748) and U4r (:750-803); Lanes
     (:851-889); §7 (:909-932); §9 (:958-968); Q16, Q21, Q22 (:1042-1045, :1054-1060);
   - plan.md: E3, E10, E12 (:461, :468, :470); E7's endpoint (:485-491); §4.4's table (:506-525); §4.6 (:553-566);
     §4.7's npt-room, npt-client and npt-lab entries with RQ8's order and the host clocks (:596-598, :609, :617-628);
   - ADR 0001: Part 5's checks and Time (:226-255); OQ-0001-6 (:1156-1167); rows PM-0001-20, -21, -22 (:1610-1612) and
     -47 (:1644);
   - marks.json: -20 (:4618-4778), -21 (:4779-5119), -22 (:5120-5202), -47 (:9046-9109), PM-0001-2 (:151-625);
     SERIES.md's rows of 2.7; README §2.5 (:187-192), §4 (:247-290), §5 (:317-330);
   - catalogue: where each check runs (:52-59); the Input, Ping, Pong and Kick rows (:87, :91-92, :99); the authority
     note (:122-123);
   - u4-scout-netcode.md: F104 (:122), F117 (:135), F59 (:77), F79 (:97), F108 (:126), F119 (:137), F121 (:139);
   - the rulings sections 3 and 10 name, and advisor conditions 1, 4, 5 and 10 (:78-89 amended at :477-479,
     :112-124, :143-149), the F2 ruling's point (6) (:463-465), wave1 W1-U4D N-M2 (:197-239), U4E-Q6 and U4E-Q11
     (:473-479, :492-495), e-ruling U4E-Q12 (:217-244, :340-341), r-check U4R-Q4 and U4R-Q14 (:198-205, :289-358);
   - .claude/rules/netcode.md (:1-17): binding on proto, room, client and server code (plan §7.1 item 8, :921-922;
     advisor :192-193). U4f writes room and client code and feeds the server's pump: every rule there binds it;
   - the code at `<base>`, by section 8's rows.

---

## 2. What to build

U4f's row (plan :806-809): ClockSync; the client's tick-lead controller; `InputBuffer` option A; every other room-side
check with its -21 counter; the server-side checks' -21 counters and judged runs on U4e's pump (U4E-Q11); the cheats'
`Behaviour`s. The marks it moves (plan :810-815): -47, -20, -21 and -22 (section 5). Nothing here adds an engine
feature beyond those rows (CLAUDE.md's "for later" rule); an item a table marks "proposed" is the builder's to build
only once section 10's ruling names it.

### 2.1 ClockSync, the command clock (npt-client; plan §4.8 :338-341; ADR 0001:239-251; net F117 :135)

| Field | What U4f builds | Source |
|---|---|---|
| Home | `ClockSync` in npt-client, owned by `ClientEndpoint` | plan.md:609 (the symbol); plan §4.8 |
| What it estimates | the offset from the client's host clock to the server's clock, signed integer ns: the command clock only. The interpolation clock is `Interp`'s (U4g: plan :822); ClockSync exposes nothing for it now | plan §4.8; net F117 |
| Its samples | the client's own `Ping` (id, the host time of the step that sends it) and the room's `Pong` (id, echoed time, tick, hold time), each timed by its datagram's arrival stamp from `ClientEndpoint::input`'s `received` (client/src/lib.rs:414-424), never by the step's own `now` | ADR 0001:242-245; catalogue :91-92; plan.md:486 |
| The server's stamp | as U4F-Q6 rules: the room's `Pong` carries only the next tick to run (lab/src/scenario.rs:459; server/src/pump.rs:458-467), so as built each sample's server instant is known to one tick period (NA-4) | U4F-Q6 |
| The filter | ADR 0001:240-242's rules (drift at most 1-2 ms per frame, halve errors over 100 ms, reset over 500 ms), over the lowest-RTT samples; every threshold in ns, data, pinned by a test | ADR 0001:239-242; net F117 |
| Ping rate | data, at most the room's limit (10 a second per session: catalogue :91; the pump's bucket, pump.rs:458-472): a clean run shows `ping_limited` 0 | net F117 |
| Bounds | outstanding pings and kept samples each capped (as the pump's `OUTSTANDING` 8 and `SAMPLES` 8: pump.rs:24, :26); a `Pong` naming none of its pings is dropped and counted (`unknown_pongs`, client/src/lib.rs:403-410, today crate-private: public only if the lab logs it) | .claude/rules/netcode.md:13-14 |
| Clock reads | none: the endpoint reads no clock (ADR 0001:245-247; client/src/lib.rs:6) | ADR 0001 |
| What -47 reads | npt-lab writes `event.connect_clock.offset_err_ns` (SERIES.md:149) per connect: abs(estimate - true) at the first client step at or after 2 s from the connect's first sample (the builder states this instant in its doc and report); the true offset is the lab's seeded host offset (lab/src/scenario.rs:18-19; plan.md:625) | marks.json:9069-9094 |
| Bites | `PM-0001-47/newest-sample` (the estimator takes the newest raw sample) and `PM-0001-47/tick-quantised` (each sample timed at the step that reads it, not at its arrival), both in `npt_client::BITES` (client/src/lib.rs:28-32, growing to 6 with 2.11's -20 bite), through `ClientEndpoint::with_bite` (:314-326); the ids spelled as marks.json's (:9097, :9101), not as the plan's words at :810 (NA-11) | marks.json; plan §4.14's rule (:454-455) |

### 2.2 The client's tick-lead controller (npt-client; net F104 :122; carry 74; r-check U4R-Q14 :289-358)

| Field | What U4f builds | Source |
|---|---|---|
| What it replaces | the endpoint's fixed lead (`lead`, client/src/lib.rs:214, 1 by default :267; `with_lead` :296-301, whose doc names U4f's controller as its replacement); whether `with_lead` stays as a lab setting is U4F-Q7 | carry 74 |
| What it sets | each command's stamped tick (today `newest + lead`, :429) | ADR 0001:250-251 |
| Its form | U4F-Q7: (a) the newest snapshot tick plus a lead from the client's own RTT and jitter samples, by U4R-Q14's arithmetic (L = ceil((rtt + jitter) / P) + 1, r-check :296-302); or (b) ClockSync's estimate of the server's tick at the command's arrival plus a margin and the buffer target (net F104 :122) | U4F-Q7 |
| Same in both options | yes: one controller under option A and option B (net F104); f6 shows equal stamps | net F104 |
| Inputs | ClockSync's samples only (the client's own measure), never the profile (the lab's knowledge: lab/src/scenario.rs:126-131 is U4r's stand-in) | ADR 0001 part 1 |
| Bounds | at least 1; at most the room's lead bound less a margin (2.4 kind 1); the endpoint keeps at most `REDUNDANCY_CAP` = 8 unacknowledged commands (client/src/lib.rs:442-443) | carry 27; ADR 0001:231-233 |
| Pass marks it serves | -20's 99.9 % on time at P3 under both options (marks.json:4684-4697); carry 74's 1 % late rule on every short run (5.3); the lead it settles on per profile reported beside U4R-Q14's numbers (P0 2, P1 3, P2 and B1 5, P3 7; P4 9 by the same arithmetic: l1/src/lib.rs:193-196) | carry 74 |
| The bots' rules | unchanged for honest bots: one command per client step, only when the newest tick moved; at most one new edge per press counter per command (bots/src/lib.rs:14-21; U4R-Q14's (b1) and A4) | r-check :322-323 |

### 2.3 `InputBuffer` option A beside U4e's option B (npt-room; OQ-0001-6, ADR 0001:1156-1167)

| Field | Option A (U4f builds) | Option B (U4e's, kept) |
|---|---|---|
| When a command runs | at the first room tick after it arrives (pumped before the tick: plan.md:621-622), several in one tick within `CMD_CATCHUP` = 8 per client, each one's tick strictly newer than the last run | exactly one per client per tick, the one stamped for it (room/src/input.rs:129-151) |
| A tick with none | nothing runs for that client (plan §4.13 :439-441) | the last command runs again (:135-141) |
| Late | no late rule: a strictly newer command runs whenever it arrives; on time is still read against its stamp (SERIES.md:61) | stamped for a tick already run or buffered: dropped and counted `late` (:107-113) |
| The per-tick cap | at most 8 run per tick; the excess refused and counted under the catch-up kind (2.4 kind 10) | the buffer holds at most 8 (`Full`, :114-116), counted under the catch-up kind |
| The setting | `params.cmd_timing` "A" or "B", a start-up setting, never changed in a run, recorded in every header (README :191-192; plan.md:461); today fixed to "B" (lab/src/scenario.rs:1286) | the same |
| Depth recorded | U4F-Q3 (the card's DEPTH ruling and its option-A note, card :1930-1933 and :1962-1964 at 11fd190) | `buffer_depth_ticks` (input.rs:63-66); the measured L - 1 reading (card DEPTH ruling) |
| **Carry 80 (a) first** | before option A runs several commands of one slot in a tick: the lead bound (2.4 kind 1) refuses a command 2^31 ticks or more from the room's tick before the sort sees it, or `canonical` (room/src/room.rs:361-369) takes a total key; a test of one slot's commands 2^31 ticks apart; carry 21's room test (room/tests/room.rs:235) made to fail without the sort | carry 80 (a), carries :513-518 |

### 2.4 The command checks, each with its -21 counter (npt-room unless named; plan.md:512-514; catalogue :87)

**The counting rule (by U4F-Q9):** every refusal is counted under exactly one kind, the first check in the order below
that refuses; npt-room reports per tick each refused command as (slot, seq, kind), as npt-sim's `reports()` does for
its own refusals (plan §4.2 :246-247); npt-lab counts a forged command "rejected and counted" only when a record names
its own kind (SERIES.md:158-159). The strictly-newer check runs before option B's late rule, so an honest late command
(-20's signal) is never counted as forged and a forged old tick never as late (NA-7). Each kind is one check of ADR
0001:231-239 (plus the wrong channel, plan.md:489), 11 in all (marks.json:4920-4931, :5118).

| # | Kind (forged command) | Home, order | Today at 11fd190 | U4f builds | Counter (builder names it; SERIES row by U4F-Q1) | Bite (2.11) |
|---|---|---|---|---|---|---|
| 1 | tick too far ahead (the lead bound) | room, 1st | none (NA-7: 8 far-ahead commands fill option B's buffer) | the check; its form and value by U4F-Q8 | lead-bound refusals | PM-0001-21/no-lead-bound |
| 2 | tick equal to or older than the last run | room, 2nd | caught only as `late` under B (input.rs:107-113) | the check, apart from `late` | not-newer refusals | PM-0001-21/no-newer-tick; PM-0001-21/not-older-than (`>=` for `>`, plan :462) |
| 3 | sequence not forward | room, 3rd | `Repeat`, counted with honest copies (input.rs:81-88; room.rs:795) | a refusal apart from honest repeats, by U4F-Q9 (NA-6) | sequence refusals | PM-0001-21/no-seq-forward |
| 4 | move length above 1 | room, 4th | none (the sim scales a long vector) | x² + y² ≤ 127², integers (bots/src/lib.rs:30-31) | move refusals | PM-0001-21/no-move-length |
| 5 | more press edges than ticks covered | room, 5th | `PressRefused` (input.rs:90-106), counted `press_refused` | kept; the counter is this kind's | `press_refused` | PM-0001-21/no-press-edge |
| 6 | aim outside the map | room, 6th | none | x and y inside the greybox's extent (±12,800 cm: sim/src/greybox.rs:62, :151; the room's `extent`, room.rs:441), z inside the position grid; beyond-reach points pass on to the sim's clamp | aim refusals | PM-0001-21/no-aim-bounds |
| 7 | fire, reload or cooldown edge against the sim's state | npt-sim, in the step | the sim refuses and records (slot, seq, kind) in `reports()` (plan §4.2 :239-249) | the lab's attribution from `reports()`; the bite's site is npt-sim's (NA-9, U4F-Q1) | the sim's refusal counters | PM-0001-21/no-cooldown |
| 8 | acknowledgement of a snapshot never sent | room, before the commands | the datagram's commands refused (room.rs:783-787), `ack_refused`, `ack_refused_commands` | kept; counted as this kind | `ack_refused_commands` | PM-0001-21/no-ack-sent |
| 9 | trailing bytes or an unknown field | npt-server's pump (plan :287-290) | `trailing_bytes`, `unknown_field` (pump.rs:125-128) | the -21 series only (U4E-Q11) | those two | PM-0001-21/no-trailing (npt-proto, quant.rs:40) |
| 10 | more commands in a tick than the catch-up cap | room | option B's `Full` (`buffer_full`) | option A's per-tick cap; both counted as this kind | catch-up refusals | PM-0001-21/no-catchup-cap |
| 11 | a known type on the wrong channel | npt-server's pump | `wrong_channel` (pump.rs:122-123); refused and counted, never closing a joined session (pump.rs:414-421; catalogue :87 against :99) | the -21 series only | `wrong_channel` | PM-0001-21/no-channel (npt-proto, quant.rs:41) |
| - | beyond the weapon's reach | npt-sim's clamp (plan §4.2 :227-231) | clamped, reported, never refused (`event.clamp_kind.counted_count` counts refusals only: SERIES.md:144) | the clamp cheat (2.6) | none by design | PM-0001-21/refuse-not-clamp (npt-sim, NA-9) |

The plan's other room-column checks stay with their units: the ClientSettings clamp (U4g: plan :822); the view-time
tolerance and the rewind cap (U4h: plan :835-837); Resync, roster, token and the event cap (U4e, built).

### 2.5 The server-side checks' -21 counters on U4e's pump (U4E-Q11, wave1 :492-495)

U4e built the checks and their unit tests; U4f writes their -21 series and runs them judged. The counters exist per
session (`Counters`, pump.rs:110-161; plan §4.1 :195): `too_large` (the size at the socket), `trailing_bytes`,
`unknown_field`, `wrong_channel`, `input_limited` (the bucket of 8, refilled at 2 x TICK_HZ a second: catalogue :87).
npt-lab writes each per client into the room's log (names and SERIES rows by U4F-Q1), so the forged suite's kinds 9
and 11 are read from them, and the size and bucket counters are reported (no forged kind of the 11 is theirs: the
catch-up kind stays inside the bucket, plan :385-388). U4f changes no pump code by this brief; if it needs any, the
lane opens npt-server (U4F-Q1) and section 6's `--all-features` item applies.

### 2.6 The cheats' `Behaviour`s (npt-bots; plan.md E12 :470; plan :808-809; bots/src/lib.rs:10-11)

Every cheat is a measured-build behaviour (the suites are measured runs), clock-injected and transport-free like U4r's
(bots/src/lib.rs:5-13). A forged datagram is the honest endpoint's own datagram, decoded and re-encoded with only the
kind's change (npt-bots depends on npt-proto: bots/Cargo.toml:23), so forged content rides only inside datagrams the
honest script sends anyway, at the same instants, on the same flows (plan :376); the twins of 2.12 follow the same
rule.

| Cheat | Suite (marks.json select) | What it does | Count per run | Its twin |
|---|---|---|---|---|
| 11 forgers, one kind each (2.4 kinds 1-11) | `forged`, A and B | an open-loop seeded script (commands from seed, slot and tick only: plan :380-381) with every k-th command forged in its kind; 37 honest open-loop clients | 1,000 forged commands per kind (marks.json:4909-4919; plan :442) | 2.12 |
| the catch-up kind (kind 10) | `forged` | 16 new commands in two datagrams in one tick, inside the bucket, so the catch-up check refuses the excess, not the bucket (plan :385-388); its construction under each option and the lead bound it needs: U4F-Q8 | as kind 10's 1,000 | the same datagrams carrying repeats only (plan :387-388) |
| the clamp cheat | `clamp`, A and B | open-loop, aim points beyond the 6,000 cm reach (sim/src/scene.rs:66) yet inside the map (2.4 kind 6): U4r's stand-in's 32,000 cm (bots/src/lib.rs:74-76) lies outside the map and would be refused (NA-8) | 1,000 points beyond reach per weapon kind the sim has (hitscan: marks.json:4954-4965) | the forged run's clamp reports' points (SERIES.md:322) at the same instants (plan :363-366) |
| the 1.5x clock | `fast-clock`, A and B | a client whose host clock runs at rate 1.5 over a 60 s episode (plan.md:625; the host clock stays continuous), giving a command at every one of its steps (not the honest bots' skip rule) with the stamps its controller gives | 10 episodes of 60 s (marks.json:4988-4999) | none (the check compares with the bound: SERIES.md:155-156; the honest reference by U4F-Q11) |
| the lag switch | `lag-switch`, A and B, P1 | holds its `Input` datagrams 200 ms, then sends them in one burst | 100 bursts (marks.json:5024-5035) | none |
| the never-acker | `never-ack` | a brawl client whose `Input`s never acknowledge a snapshot or a full state (ack groups 0); every other client brawls | one among 48 (ADR 0001:1612) | none |

### 2.7 npt-lab: suites, settings and series (lab/src/scenario.rs; lab/src/logs.rs)

- **Settings:** `--cmd-timing A|B` into the header (scenario.rs:1286 today writes "B"); the suites `forged`, `clamp`,
  `fast-clock`, `lag-switch`, `never-ack` and `clock` (marks.json's selects; SERIES.md:413's suite param) and their
  behaviours by U4F-Q11; `--twin <run>` with role `twin` (README :22-23; scenario.rs:1404); the 1.5 host-clock rate
  per client and episode (scenario.rs:18-19 names it U4f's); the lead bound's value in the header (U4F-Q8).
- **The -47 suite:** 1,000 connects at P2 with seeded random true offsets, each connect's estimate compared at 2 s
  (2.1); 48 slots give about 21 rounds of fresh endpoints. Rejoins on one slot meet carry 62 (d)'s head rule and the
  slot-keyed logs of NC-U4R-5, so the suite's logging is by U4F-Q4.
- **The series U4f writes** (each name's writer as SERIES.md gives it; a name with no row gets one only by ruling, as
  U4R-Q2 (c): U4F-Q1):

| Mark | Series (SERIES.md row) | Writer |
|---|---|---|
| -20 | `tick.s.cmd_on_time` (:61), `tick.s.input_missing` (:63), `tick.v.press_count` (:40), `tick.v.press_lost_count` (:42), `tick.v.press_doubled_count` (:41), `second.s.buffer_depth_count` (:101); `header.params.reorder_permille` (:245, U4r's) | npt-room's counts, npt-lab's room log |
| -21 | `event.forged_kind.sent_count`, `.unrejected_count` (:158-159); `event.clamp_kind.sent_count`, `.counted_count` (:144-145); `event.clamp_report.x_um`, `.y_um`, `.z_um` (:322); `event.fast_clock.excess_cmds_count`, `.excess_move_um` (:155-156); `event.lag_burst.held_ns` (:170); `tick.v.lagswitch_excess_move_um` (:34); `tick.v.lagswitch_cmds_count` (:33) | npt-lab, from the room's records |
| -22 | `tick.v.ack_starved_headroom_count` (:21, from `ClientSlot::headroom`, room.rs:282-287); `event.ack_starved.dropped` (:128); `second.v.rss_bytes` (:93, whose writer is npt-server: on L1 by U4F-Q1, NA-2); `header.params.event_queue_cap_count` (:442) with `event_queue_cap_provisional` (:344) | npt-room, npt-lab |
| -47 | `event.connect_clock.offset_err_ns` (:149) | npt-lab |
| all four | `header.params.cmd_timing` (:412), `params.suite` (:413), `params.twin` (:426) | npt-lab |

- The -20 and -58 press series are the same names (SERIES.md:41-42 list both marks): U4g's -58 reads U4f's writers.
- **logs.rs's `room_tick`** (logs.rs:495) gains these series: RR-B1-7 (1) rides that edit (section 9).

### 2.8 The event cap's measurement before -22's runs (carry 57; EVQ-LOAD; U4E-Q6, wave1 :473-479)

1. The room's cap is data, provisional at 1,024 (room/src/events.rs:38-40; room.rs:460-461), recorded as provisional
   in every header (SERIES.md:344). U4f measures and records plan §4.7's value before any -22 run (carry 57).
2. **EVQ-LOAD**, carried word for word: @@WORDS-N/advisor-brawl1-sets-p3-full.md:158-158@@ (the quoted carry line;
   also N/u4r-report.md:1048-1052).
3. **The measurement** (the builder's short runs, SS, 48 clients, the brawl, under the runner, `-x`): P2, P3 and B1
   (B1 with at least 2 bursts), 3 seeds each; per client the relevant gameplay-event rate and the queue's high-water
   mark per burst and per 30 s bin (`ClientSlot::queue`, room.rs:277-280; carry 58's figures).
4. **The value, by U4F-Q12:** plan §4.7's formula (:335) lands below the observed need (NA-3: 560 a second x (1 s +
   RTT + margin) reaches about 1,008 at a 0.7 s margin, while 1,024 already overflowed at P3 without a burst), and
   carry 50 bounds the cap from above for -22 (NA-12: a never-acker's events expire at 256 ticks,
   room/src/events.rs:19-20, so a cap above its rate x 255 / 30 s, about 3,900 at 460 a second, is never reached and
   -22's headroom check fails).
5. The chosen value, its derivation and the high-water figures go into plan §4.7 (the main session's commit: plan risk
   11, :953-954) and into the room's default with `event_queue_cap_provisional` false; the judged -20, -21 and -22
   runs run at it.

### 2.9 The client carries 80 (b)-(e) (npt-client; carries :518-534)

| Part | Site at 11fd190 | Test (2.10) | Bearing on Mustafa's open answers |
|---|---|---|---|
| (b) a refused far-ahead snapshot header moves the newest tick and resolves the span lost | `receive`'s refused-datagram sentinel (client/src/lib.rs:508-522) into `sentinel` (:631-641) and `see` (:583-615) | f24 | **touches how the client files lost ticks**: C62H-BOUND is open with Mustafa (the card's "C62H-REVIEW ANSWERED" block, :479-492 at 11fd190; c62h-review :75-80). U4f designs no lost-tick bound and changes no -17 check; its report gives each judged run's lost count beside the share the profile's loss predicts, as c62h-review :55 records for the open answer |
| (c) a failing command drops the whole `Input`, acks included | `step` (:461-469) | f25 | none |
| (d) the client's two too-old cases | `too_old`, `old_header`, `bad_header` (:528-565) | f26 (carry 80 (d)'s endpoint test as worded) | **touches how the client files too-old ticks** (the sentinel and `stale`): C62H-KEEP and C62H-BOUND are open with Mustafa (c62h-review :68-80); no bound designed |
| (e) a full state after a snapshot of another hash files nothing | `full_state` (the scout's :810-839) and `file` (:645-658) | f27 | none |

KL-YES (the KEPT-LOGS kit lines) is open with Mustafa too; it bears on section 5's disk, not on these files.

### 2.10 Tests and negative cases (counts as deltas on `<base>`'s: plan §5 :507-512)

Each row adds the Δ tests in both builds; "neg" marks a negative case (a refusal or a hostile input). Minimum deltas:
**measured +60, bites +74** (the measured 60 plus 14 bites-only unit proofs, 2.11), **44 negative cases** (the Δ of
the rows marked neg); with T-6's five contract cases, +5 more in both builds if contract.rs is edited (section 9). The
builder states the exact counts and the `--list` lines that name them.

| Id | Test | Crate | Neg | Δ |
|---|---|---|---|---|
| f1 | ClockSync on a fixed P2 arrival trace: error at 2 s within the stated bound | client | | 1 |
| f2 | the three filter rules of ADR 0001:240-242, one test each | client | | 3 |
| f3 | outstanding pings and samples capped; an unknown pong id dropped and counted | client | neg | 2 |
| f4 | the ping rate never exceeds the room's limit (a faster rate is limited and counted by the pump) | client | neg | 1 |
| f5 | the controller's lead per profile (P0-P4, B1) against U4R-Q14's numbers; never above the bound | client | | 1 |
| f6 | the same stamps under option A and option B (net F104) | client | | 1 |
| f7 | option A runs several commands of one slot in a tick; the 9th is refused and counted (kind 10) | room | neg | 1 |
| f8 | carry 80 (a): one slot's commands 2^31 ticks apart: no panic, refused first or sorted by a total key | room | neg | 1 |
| f9 | carry 80 (a): carry 21's room test (tests/room.rs:235) fails with the sort removed (a plant, restored; the test itself changes, so +0) | room | | 0 |
| f10 | kinds 1-6, 8 and 10 each refused and counted under its own kind, under both options | room | neg | 8 |
| f11 | the counting rule: a command breaking two checks is counted once, under the first; an honest late command counted late, never not-newer | room | neg | 2 |
| f12 | an honest redundant repeat is not refused; a backward sequence with other content is refused under kind 3 (NA-6) | room | neg | 1 |
| f13 | 8 far-ahead commands, then an honest command: the honest one runs (NA-7) | room | neg | 1 |
| f14 | the pump's five counters (2.5) written as their series, one test each | lab | neg | 5 |
| f15 | each forger produces exactly its kind (its datagram decoded: only that check broken), 11 cases | bots | neg | 11 |
| f16 | forged run and twin: fate logs equal on every flow, snapshot group counts equal on every tick (plan :389-390): forged A, forged B, clamp A, clamp B | lab | | 4 |
| f17 | carry 46: a forged press stream and its twin give equal state hashes on every tick, at the room's level | room | | 1 |
| f18 | the clamp cheat's points lie beyond reach and inside the map; the twin's points from the clamp reports give equal hashes on a short run | bots, lab | | 2 |
| f19 | the 1.5x clock: excess commands at most 0 in the clean build | lab | neg | 1 |
| f20 | the lag switch under P1: at most `CMD_CATCHUP` commands run per tick; movement within the catch-up cap's | lab | neg | 1 |
| f21 | the never-acker: headroom reaches 0, never below; one resync, then `Kick(overflow)` within `FULL_STATE_ACK_TICKS`; its later datagrams reach no room call (carry 76) | lab | neg | 2 |
| f22 | the catch-up kind: 16 commands in two datagrams in one tick, inside the bucket: the catch-up check refuses the excess, never the bucket, under A and B | lab | neg | 2 |
| f23 | a known type on the control stream: refused and counted, the session kept (catalogue :87, :99) | lab | neg | 1 |
| f24 | carry 80 (b) | client | neg | 1 |
| f25 | carry 80 (c) | client | neg | 1 |
| f26 | carry 80 (d)'s endpoint test, each half failing under its plant | client | neg | 1 |
| f27 | carry 80 (e), failing under its plant | client | neg | 1 |
| f28 | every series of 2.7 written, with its SERIES.md row, on a short run of each suite (the contract test's own check: plan §4.12 :412-415) | lab | | 1 |
| f29 | the -47 suite: 1,000 connects, one event each at 2 s, true offsets seeded | lab | | 1 |
| f30 | §4.11's bites per instance for U4f's new ids: a bitten and a clean endpoint and room on parallel threads (plan :399-401) | client, room | | 1 |

Kept, not new: the Send asserts (plan :402-403) cover `ClientEndpoint` with ClockSync and `Room<World>` with option
A; the runner's twin test (plan :406-407).

### 2.11 The bites (17; plan §4.14 :453-476; marks.json)

Each planted where its code lives, shown by a unit test here, and run judged in its suite and option (plan :453-455,
:811-812). Bites-only, chosen per instance (plan :399-401); one home per id (check.mjs's `biteHomeProblems`).

| Bite | Site (crate) | Plant | Unit proof | Judged run (section 5) |
|---|---|---|---|---|
| PM-0001-20/redundancy-1 | the endpoint's redundancy (client/src/lib.rs:442-443) | repeats truncated to 1 | an `Input` holds one command (new) | -20 P3, option by U4F-Q10 (recommended B) |
| PM-0001-21/no-lead-bound | room, kind 1 | check removed | a far-ahead command buffered or run (new) | forged, option by U4F-Q10 |
| PM-0001-21/no-newer-tick | room, kind 2 | check removed | an equal-tick command runs (new) | forged, U4F-Q10 |
| PM-0001-21/no-seq-forward | room, kind 3 | check removed | a backward sequence runs (new) | forged, U4F-Q10 |
| PM-0001-21/no-move-length | room, kind 4 | check removed | a long move runs (new) | forged, U4F-Q10 |
| PM-0001-21/no-press-edge | room, kind 5 | check removed | two edges in one covered tick run (new) | forged, U4F-Q10 |
| PM-0001-21/no-aim-bounds | room, kind 6 | check removed | an off-map aim runs (new) | forged, U4F-Q10 |
| PM-0001-21/no-cooldown | npt-sim's shot phase (NA-9; U4F-Q1) | check removed | a cooldown-breaking edge fires (new) | forged, U4F-Q10 |
| PM-0001-21/no-ack-sent | room, kind 8 (room.rs:783-787) | check removed | a never-sent ack's commands run (new) | forged, U4F-Q10 |
| PM-0001-21/no-trailing | npt-proto (quant.rs:40) | U4a's | U4a's and U4e's (proto/tests/bites.rs:211; server/tests/pump.rs:477) | forged, U4F-Q10 |
| PM-0001-21/no-catchup-cap | room, kind 10 | the per-tick cap removed | 16 commands run in one tick (plan :463) (new) | forged, option A (plan :463) |
| PM-0001-21/no-channel | npt-proto (quant.rs:41) | U4a's | U4a's and U4e's (proto/tests/bites.rs:238; server/tests/pump.rs:483) | forged, U4F-Q10 |
| PM-0001-21/not-older-than | room, kind 2 | `>=` in place of `>` | an equal-tick command runs (plan :462) (new) | fast-clock, option A (plan :462; marks.json:5118) |
| PM-0001-21/refuse-not-clamp | npt-sim's clamp (NA-9; U4F-Q1) | a refusal in place of the clamp | a beyond-reach point refused and counted (new) | clamp, U4F-Q10 |
| PM-0001-22/unbounded | the room's event queue (events.rs:22-24; room lib.rs:44) | U4e's | U4e's (room/tests/room.rs:1302, :1367) | never-ack over a full S30 (marks.json:5195; e-ruling :340-341) |
| PM-0001-47/newest-sample | ClockSync (2.1) | the newest raw sample | the estimate equals the newest sample (new) | clock, P2 |
| PM-0001-47/tick-quantised | the endpoint's arrival stamps (2.1) | the step's time in place of arrival | a sample's time moves to its step (new) | clock, P2 |

New unit proofs: 14 (redundancy-1, 11 of -21's 13, both of -47's); U4a's and U4e's three stand. A bite run needs no
twin: each fails another check of its part (README §5 :317-326), and keeps its part's validity rules, `pre:PM-0001-2`
included (README :323-324), so each -20 and -21 bite run must also pass PM-0001-2 (U4F-Q10's reason for -20's option).

### 2.12 -21's twins (plan §4.10 :360-390; Q16 (a) :1042-1045; advisor condition 1 :78-89 as amended at :477-479; carry 46)

| Kind | The twin's datagram in the forged one's slot | Source |
|---|---|---|
| refused whole by the room (kinds 1-6, 10) | the same datagram with the forged command left out (the accepted stream is then the forged run's) | condition 1 (only the refused part removed) |
| a datagram voided whole (kind 9) or its commands refused (kind 8) | the header only, no commands, its ack equal to the previous datagram's | plan :377-379 |
| the catch-up kind (kind 10) | the same number of datagrams at the same instants, repeats only | plan :385-388 |
| a wrong-channel message on the control stream (kind 11) | a valid no-effect message (a ClientSettings repeating the current delay) | plan :382-384 |
| a sim-checked edge (kind 7) | the forged command with only the refused press removed: that button's counter one lower from the refused command on, every other field equal, exactly the presses `reports()` lists (carry 46) | plan :367-371; wave1 :197-235 |
| the clamp | the forged run's clamp reports' points at the same instants | plan :363-366; SERIES.md:322 |

Every client of the forged and clamp suites runs an open-loop seeded script (plan :380-381). f16 and f17 prove the
alignment and carry 46.

### 2.13 The seam rule and the public items, by crate (A19: plan §7.2 :930-931; §5 :512)

- **The seam:** U4f changes no seam type (`Command`, `Rewind`, `TickInput`, `SimCore`'s methods: plan §4.2). The two
  npt-sim bites (NA-9) add ids to `World::BITES` (sim/src/world.rs:937-938) and their sites, bites builds only, if
  U4F-Q1 opens crates/sim for them; any other sim change is a seam change for the advisor (carry 26's route).
- **Proposed public items** (engine-architect checks them in review; each named in the report against §4.1 and plan.md
  §4.7; anything else named as beyond them):

| Crate | Added or changed | Why |
|---|---|---|
| npt-client | `ClockSync` (plan.md:609; not in plan §4.1's row :193); a `ClientEndpoint` accessor for the clock estimate, which the lab reads for -47; the controller's setting if U4F-Q7 keeps one; `BITES` to 6 ids | 2.1, 2.2, 2.11 |
| npt-room | option A behind the `cmd_timing` setting (a `Room` constructor or setting); the per-tick refusal records (slot, seq, kind) by U4F-Q9; the new counters in `ClientCounters` (room.rs:63-136); the lead bound's setting if U4F-Q8 makes it data; `BITES` widened by 9 ids | 2.3, 2.4 |
| npt-bots | the cheats' and the never-acker's constructors on `Behaviour` (plan §4.1 :197 names them) | 2.6 |
| npt-sim | none, or two ids in `World::BITES` (bites builds only) by U4F-Q1; a lead-bound tick constant only by U4F-Q8 (a), through carry 18's route | 2.11, 2.4 |
| npt-server | none (2.5) | |
| npt-lab | binary settings only (2.7); no new library item | |

---

## 3. The carries

**The rule** (word for word): @@WORDS-N/u4s-carries.md:9-10@@. U4f's report lists each carry below as met (file:line,
test), not met (why) or not applicable (why), by number (advisor :334-335; U4r's wording, N/u4r-brief.md:695-696).

| Entry | Placeholder (filled word for word) | carries lines | Where U4f meets it |
|---|---|---|---|
| 46 | @@CARRY-46@@ | :506-509 | 2.12, f17 |
| 80 (a)-(e) | @@CARRY-80@@ | :510-534 ((a) :513-518, (b) :518-520, (c) :520-522, (d) :522-529, (e) :529-534) | (a) 2.3, f8, f9; (b)-(e) 2.9, f24-f27 |
| 86 | @@CARRY-86@@ | :535-537 (its text has no quotation marks: NA-1; the fill takes the three lines whole) | 5.3 |
| 57 | @@CARRY-57@@ | :223-228 | 2.8 |
| 74 | @@CARRY-74@@ | :343-354 | 2.2; 5.3's late share |
| 21 | @@CARRY-21@@ | :148-149 | 2.3 (option A's several commands per slot); f9 |
| 27 | @@CARRY-27@@ | :166-168 | 2.3 (`CMD_CATCHUP` and its const assert, input.rs:59-61, :155, kept; option A's per-tick cap reads it) |
| 18 | @@CARRY-18@@ | :129-132 | only by U4F-Q8 (a): the lead bound's tick constant by carry 18's route (advisor :463-465) |
| 39 | @@CARRY-39@@ | :169-179 | every rate-dependent value U4f adds (ClockSync's ns thresholds, the controller's ticks, the lab's 60 s and 200 ms) takes the rate as an argument; no local tick constant |
| 50 | @@CARRY-50@@ | :180-183 | 2.8 item 4 (the cap's upper bound, NA-12); no code change |
| 61 | @@CARRY-61@@ | :229-247 | 2.9 ((b) is the case carry 80 (e) completes) |
| 58 | @@CARRY-58@@ | :269-273 | 2.8 item 3 (high-water marks); no -17 or -18 run is U4f's; U4f's runs report their overflow resyncs beside them |
| 76 | @@CARRY-76@@ | :364-371 | 2.6's never-acker (the Kick path), f21; no forged kind closes a session (2.4 kind 11) |
| 77 | @@CARRY-77@@ | :372-381 | (a) wraps per counter name in U4f's runs; (c) `stale()` sums reported on 5.3's short runs |

Not met by this brief, each by its own condition: 63 (carries :322-327) only if U4f opens crates/log; 64 (:447-473)
only if it opens crates/judge (it adds round 2's judge follow-ups first); 67 (:476-493) only if U4f moves ES's
synthetic recording or the World's outcome on it. U4f opens none of these by section 4's proposed lane; a ruling that
opens one adds its placeholder here. Carry 62 (h) (:305-321) is not U4f's: its stay is C62H-KEEP (section 9).

---

## 4. Build rules

**The shared target** (Mustafa's rule: history :411-420; plan §7.1 :909-922; W1-TARGET), on every cargo, rustc and
rustdoc call in your worktree, as U4r's section 4 rules 1-9 give them (N/u4r-brief.md:938-992):
1. `CARGO_TARGET_DIR=/home/user/Shamble-Frontlines/.gate/shared-target` and `CARGO_NET_OFFLINE=true` on every call
   (N/u4r-brief.md:940; the runner sets both); checked once after the first build: no `target/` in your worktree.
2. The lock: every such call through the runner (shared for a plain build, `-x` for a counted run, a bite run, a
   plant, `--list` and clippy confirmations); a plant's commands go in one call, restored and its sha256 checked
   before the next.
3. Provenance from the builds run before check.mjs inside the same exclusive runner call (N/u4r-brief.md:951-957's
   form); a missing line means "not counted".
4. Touch before each build (plan :913); the runner does it.
5. **Counts are deltas** (plan :914-915, :884-885): `<base>`'s counts plus the tests you added (2.10), on check.mjs's
   measure; a mismatch means another copy's artifact ran: touch, rebuild, never report that run; `--list` only names.
6. **Disk** (N/u4r-brief.md:968-971): `df -h /home/user` before each build, in your notes; at least 4 GB free before
   any wasm32 or release build and every check.mjs run, 2 GB before every other build; the runner's floor is 4 GiB
   (c-u4r-runs.mjs:131-132); below it, stop and report; never clean anything (plan :920).
7. **Cleans and cargo probes are not yours.** The main session's cleans: @@WORDS-N/advisor-u4d2-m9.md:252-256@@; the
   two lessons: @@WORDS-N/advisor-u4d2-m9.md:259-264@@; the probe lesson: @@WORDS-N/advisor-u4d2-r2.md:304-308@@.
8. After the last bite or plant, touch and rebuild before any further claim (plan :916).
9. Cargo's own lock serializes builds: expected (plan :917). The main folder's cache is not yours (plan :918-919).

**§4.11's bans** (plan :392-407), which bind ClockSync, the 1.5x host clock and the lag switch, all on simulated time
(plan.md:524-525, :617-628): no HashMap or HashSet in room, client, server, lab or bots (each crate's clippy.toml at
11fd190: room, client, server, lab and bots hold one); no clock read in server, lab or bots (`std::time::Instant`,
`std::time::SystemTime`, `tokio::time::Instant` and their `now`: plan :393-396); the client reads no clock (ADR
0001:245-247); npt-room float-free (plan :397-398; check.mjs's grep), so every room check is in integers; bites per
instance, never a static (plan :399-401).

**Your files: the proposed lane (U4F-Q1; the plan names none: plan :805-816 and Lanes :851-889 give only "they share
the room's and the client's files", :882-883):**
- crates/room/src/ (input.rs, room.rs, lib.rs's `BITES`) and crates/room/tests/;
- crates/client/src/ and crates/client/tests/;
- crates/bots/src/lib.rs (but src/bin/) and crates/bots/tests/;
- crates/lab/src/ (scenario.rs, logs.rs, contract.rs if 5.3 needs it, their tests) and crates/lab/tests/;
- ops/runs/c-u4f-*.mjs (new: the r2 extension, the cap measurement's short runs and the judged-set runner, on
  c-u4r-r2.mjs's and c-u4r-runs.mjs's models);
- by U4F-Q1 only: crates/sim/src/world.rs for the two bites' sites and ids (bites builds only: NA-9);
  marks/SERIES.md, new rows only (U4R-Q2 (c)'s precedent: r-check :189-196); crates/proto and the catalogue if
  U4F-Q6 rules a wire change; crates/sim's carry 18 places if U4F-Q8 (a); crates/server if a pump change is needed.
- **Not yours:** every Cargo.toml and Cargo.lock (the main session's commit); marks.json and crates/judge (5.6);
  crates/log, crates/l1, crates/link, crates/net, crates/tcpinfo, crates/hash-wasm; check.mjs and check.test.mjs (but
  by U4F-Q8 (a)); docs/ (but by U4F-Q6); results/ (the main session's runs write it: plan :928-929). A part that
  needs anything more stops, goes in your report, and the rest is built.

**Files:** @@WORDS-N/advisor-u4df.md:462-464@@ (the sentence inside T4 item 1's quotation marks, word for word: the
advisor's U4d-F ruling). **Plants:** @@WORDS-N/advisor-u4j-r2.md:298-298@@ (the words after "From now on: "), then
restored and the sha256 checked against the clean file. A `cp`, `mv` or `tee`, and a `>` or `2>` into a file, are
shell writes too, in any folder (N/u4r-brief.md:1016-1019). An agent that slips says so at the top of its notes
(df-ruling :317-319).

**Git** only in your worktree, never `git -C` or `git -c`; commit nothing and push nothing; the main session merges
(plan :925-927). **Bite ids:** no crate's sources spell another crate's bite id as a string literal (check.mjs's
`biteHomeProblems`; N/u4r-brief.md:1023-1026).

**The stop clause.** TE2-5's rule ended at U4r's merge (the card's M6 line, card :1553 at 11fd190; TE-Q2, te-m3
:119-126). Its after-clause stands: @@WORDS-N/advisor-te-m3.md:127-128@@ (NA-10: the night plan's cite :126-127 is
one line early). Whether the U4d-F ruling's stop clause (the card's "Rules in force", its U4d-F item 2; df-ruling
:465-468) still applies now that TE2-5's rule has ended is U4F-Q5; if it does, it reads
@@WORDS-N/advisor-u4df.md:465-468@@ with its first `<crate>` replaced by "npt-room, npt-client, npt-bots or npt-lab
(and each crate U4F-Q1 opens)" and "<crate>'s files" by "their files" (section 12 item 3).

**The builders' limits** (Mustafa's words, by script): @@WORDS-N/history.md:292-293@@.

**SIZE-CHECK:** U4f is size M (plan :805); his rule is history :398-399 and :2039-2087 (the card's "Rules in force"
line gives what M means, card :3028-3035 at 11fd190): the main session sets U4f's measured size beside its plan on the
card at every snapshot, and past his threshold the build stops for his three lines. The builder puts its diff size
(`git diff --stat <base>`) in each block's notes line.

**.claude/rules/netcode.md** binds the proto, room, client and server code U4f writes or feeds (plan :921-922): inputs
only, never state; every inbound value validated; no unbounded buffer keyed by client data (ClockSync's samples, the
room's refusal records, the cheats' held datagrams all capped); the catalogue updated with any protocol change.

---

## 5. The judged runs (the main session's, after the merge, in the main folder: plan :766, :928-929)

### 5.1 The run list, with counts (plan §9 :958-968; marks.json)

| Mark | Part (marks.json) | Select beyond layer L1 | Measured | Twins | Bites (one run each) |
|---|---|---|---|---|---|
| -20 | L1 P3 option A (:4632) | SS, P3, 48 clients, `cmd_timing` A | 3 | | redundancy-1 (option by U4F-Q10) |
| -20 | L1 P3 option B (:4646) | the same, B | 3 | | |
| -21 | forged, option A / B (:4792, :4806) | SS, 48, suite `forged` | 3 + 3 | 3 + 3 | the 11 no-* bites (option by U4F-Q10; no-catchup-cap A) |
| -21 | clamp, option A / B (:4820, :4834) | SS, 48, suite `clamp` | 3 + 3 | 3 + 3 | refuse-not-clamp |
| -21 | fast-clock, option A / B (:4848, :4862) | SS, 48, suite `fast-clock` | 3 + 3 | | not-older-than (option A) |
| -21 | lag-switch, option A / B (:4876, :4890) | SS, 48, suite `lag-switch`, P1 | 3 + 3 | | |
| -22 | L1, one never-acker among 48 (:5133) | SS, 48, suite `never-ack` | 3 | | unbounded (a full S30) |
| -47 | L1 P2, 1,000 connects (:9059) | P2, suite `clock` | 3 | | newest-sample, tick-quantised |
| | **Totals** | | **36** | **12** | **17** (65 runs) |

Against plan §9 (:962-963): -20 6, -21 24 plus 12 twins, -22 3, -47 3 (48 measured and twin runs), plus 17 bite runs
(-20 1, -21 13, -22 1, -47 2). Each measured run of -20, -21 and -22 is 10 minutes (marks.json's notes), seeds 1 to 3
(plan.md:765-775); each bite run seed 1 (c-u4r-runs.mjs:326-329's pattern). -20's runs need 10,000 scripted presses
(marks.json:4712-4721: the brawl makes none, bots/src/lib.rs:279-281, so the suite's behaviour is U4F-Q11's) and a
reorder share above 0 (marks.json:4680-4682; README :276-277). The forged, clamp, fast-clock and never-ack parts select
no profile: U4F-Q11.

### 5.2 The precondition table (plan §4.12 :422-424; advisor conditions 4 and 5, :112-124; README :285-290)

| Mark | `pre:PM-0001-2` | Shown passing first by | Note |
|---|---|---|---|
| -20, option B | yes | U4r's r2 (its P3 brawl suite, c-u4r-r2.mjs:80; U4R-Q4, r-check :198-205) | -20's own suite (U4F-Q11) is not the brawl: 5.3 runs it |
| -20, option A | yes | 5.3 (carry 86) | |
| -21, option B (4 parts) | yes | U4r's r2 with open-loop stand-ins (c-u4r-r2.mjs:82-85), their own checks not resolved (r-check :198-201) | whether 5.3 re-runs them with the real cheats: U4F-Q11 (NA-8 shows the stand-ins differ) |
| -21, option A (4 parts) | yes | 5.3 (carry 86) | |
| -22 | **no** (README :285-290's exception) | none needed | its runs still log PM-0001-2's series (shots, projectiles, vehicles, teams, portals), and U4f's report states the load each run carried (advisor condition 4, :116-117) |
| -47 | no (its part selects no scene) | none | |

A bite run keeps its part's validity rules, the precondition included (README :323-324).

### 5.3 The r2 extension (carry 86; U4R-Q4, r-check :198-205)

Short runs (r2's length: a 10 s warm-up and 20 s measured, c-u4r-r2.mjs:8-9) of -21's four option-A suites (forged,
clamp, fast-clock, lag-switch at P1) and -20's P3 option A, with the real cheats and U4f's suites, each showing every
PM-0001-2 check passing, not only resolving, and the shot band (plan :437-447); plus, per run, carry 74's late share
against the commands sent (a share above 1 % comes to the advisor before the marks count) and carry 77 (c)'s `stale()`
sum. A suite that fails comes to the advisor first (carry 86). The script: c-u4f-r2.mjs on c-u4r-r2.mjs's model
(SUITES :77-92), run from the merge commit (r2's record rule, c-u4r-r2.mjs:2-4).

### 5.4 Each judge call (JDG-ONLY-1's interim rule)

The rule, word for word: @@WORDS-N/advisor-u4r-c62h-full.md:20-20@@ (the sentence from "Until the fix lands" to its
end; the card's JDG-ONLY-1 block, :1467-1474 at 11fd190). So: `npt-judge --only PM-0001-2,PM-0001-20` and `--only
PM-0001-2,PM-0001-21` (both carry `pre:PM-0001-2`); `--only PM-0001-22` and `--only PM-0001-47` (neither does). Each
-20 and -21 bite run's header lists PM-0001-2 and its own mark (c-u4r-runs.mjs:328's rule; c62h :44-46); a -22 or -47
bite run lists its own mark. The judge's memory by c-u4r-runs.mjs:176's fit (302.2 + 2.3403 x MiB of logs) against
the 10,240 MiB line (:178): -21's 49 runs at about 2,617 MiB give about 6,430 MiB (estimate), under the line; a set
projected over it is judged in two calls, measured and bites (U4r-F2's split, :163-166).

### 5.5 Time and disk per set (each an estimate; the run that measures it named)

| Set | Runs | Disk (estimate) | Wall time (estimate) | Measured by |
|---|---|---|---|---|
| -20 | 7 | 392 MB (56 MB a run, c-u4r-runs.mjs:153; -20's per-command series may add) | about 39 min (18,335 ticks at P3's 54.7 ticks/s: about 335 s a run) | the first -20 run |
| -21 | 49 | 2,744 MB | 3.6 to 4.8 h (263 s a run at P0's 69.6 ticks/s, up to 355.4 s, the longest judged run) | the first run of each suite, and f28's short runs |
| -22 | 4 | 336 MB (3 x 56; the S30 bite about 168, three 10-minute runs' worth) | about 34 min (the S30 bite about 1,015 s) | the bite run |
| -47 | 5 | no figure (1,000 connects in about 21 rounds of about 2.5 s: under 1 min of simulated time a run) | no figure | f29's short run |
| pre-runs | 9 + 5 (or 9) | about 145 MB (r2's 13 suites took about 145 MB: c-u4r-r2.mjs:40-43) | under 1 h | the runs themselves |
| **Total** | **65 + pre-runs** | **about 3,750 MB** | **about 5.5 to 7 h** | |

No figure exists for -47's connects, -21's suites with twins and bites, option A or a never-acker run (the scout's
Disk entry). At 11fd190's reading, 7,689 MiB free less the 4 GiB floor leaves 3,593 MiB, below the sets' total before
builds (NA-5): the sets cannot all run unless KEPT-LOGS' kit route (Mustafa's KL-YES, open; his KEPT-LOGS (c) rule,
history :2088-2111) or an approved clean frees room first. U4F-Q2 asks the run budget; KL-YES stays his.

### 5.6 marks.json and the judge

U4f changes neither marks.json nor crates/judge by this brief: its runs fit marks.json's parts as they stand, and the
figure -21 needs, `twin_hash_mismatch_count`, exists (marks.json:4944-4953). If the build finds a change it needs (a
bite entry, a note, a value), it stops and reports; such a change then owes a reviewer's read before U4f closes, named
in the report's own line (Mustafa's rule, history :2127; c62h-review :84, :89), and the test at close is
`git diff --stat <base>..<head> -- spikes/netcode-proof/marks/marks.json spikes/netcode-proof/crates/judge`
(c62h-review :84). A read of a SERIES.md change (U4F-Q1's new rows) is the advisor's recommendation, not his rule
(c62h-review :93).

---

## 6. End checks (each a counted run under section 4's rules: `-x`, `df` before each)

1. check.mjs twice, every step passing both times: 11 steps (fmt, clippy, clippy-wasm32, doc, test, deps, wasm, refs,
   marks, self, redact), in N/u4r-brief.md:1073-1079's form: each run's last line, the test step's two counts as deltas
   on `<base>`'s (2.10), the self count, the provenance lines.
2. clippy on the host and on wasm32, in both builds, warnings as errors: check.mjs's two clippy steps; wasm32 covers
   LEAF plus npt-tcpinfo (check.mjs:39, :1957), so npt-room and npt-client (ClockSync, the controller, option A) are
   linted for the Worker's target too; npt-bots and npt-lab on the host only.
3. fmt: check.mjs's fmt step; a diff applied by Edit (section 4's write rule).
4. **Only if U4F-Q1 opens npt-server before U4h:** `cargo test -p npt-server --all-features` compiling, with that
   command's run as the evidence (r-b2 :24-28; the card's follow-ups block, :1960-1961 at 11fd190): the follow-up's
   words @@WORDS-N/advisor-u4r-build2-full.md:28-28@@. Today it stops at E0425 in server/tests/loopback.rs (r-b2 :27).
5. The run scripts (c-u4f-*.mjs): each npt-lab command line they issue, run by hand on a short setting, with its line.
6. `node tools/gate.mjs --no-smoke` is the merge step's, in the main folder (plan :510): not yours.

---

## 7. Return list (your report, in this order)

1. `<base>`; your final diff's file list (`git status --porcelain`, `git diff --stat <base>`), each file inside section
   4's lane or a lane a ruling opened (named with it); `git diff --stat <base> -- spikes/netcode-proof/crates/sim`.
2. Each build item of 2.1-2.9: built (file:line) or not (why); the counting rule's order (2.4) as built.
3. The tests f1-f30 and any you added: name, file, what each shows; the counts in both builds as deltas, with the
   `--list` lines; the negative cases counted.
4. Each bite of 2.11: id, site (file:line), the test that fails by name in the bites build, that test passing clean.
5. The cap measurement (2.8): per profile and seed the rates and high-water marks; the value proposed and its
   derivation, against carry 50's upper bound.
6. The r2 extension (5.3), per suite: each PM-0001-2 check against its band, the shot band, the late share, `stale()`.
7. ClockSync's error on its short P2 runs and the controller's settled lead per profile beside U4R-Q14's numbers.
8. Every carry of section 3 by number: met, not met or not applicable, each with why.
9. Every pub item by crate against §4.1 and plan.md §4.7 (2.13); any beyond them named as such.
10. What stopped and goes to the main session: lane needs, tick counts or rate-derived constants (carry 39's form),
    seam or wire changes.
11. The end checks (section 6): each line, each `df` reading, each counted run's provenance and its `-x` use.
12. How each U4F-Q ruling was applied.
13. The open items that rode (section 9: RR-B1-7 (1), TE-B1-R2-1 with RR2-B1-1, T-6 if contract.rs opened) and how.
14. Whether marks.json or crates/judge changed (5.6's `git diff --stat` line).
15. Every stop-clause entry (by U4F-Q5) and every test failure, with its cause; no retry into green.
16. The diff's size (SIZE-CHECK); what you did not verify; what is left; who built it.

---

## 8. Recheck: every item from an earlier unit this brief cites (for the day session, against `<base>`)

| # | Item | Built by | Its use in U4f | file:line at 11fd190 |
|---|---|---|---|---|
| 1 | `InputBuffer`, `CMD_CATCHUP`, `buffer_depth_ticks`, `offer`, `take`, the const assert | U4e | option B kept; option A beside it; kinds 2, 3, 5, 10 | room/src/input.rs:17-30, :61, :64-66, :80-125, :129-151, :155 |
| 2 | `canonical` and its unit test | U4e | carry 80 (a) | room/src/room.rs:361-369, :1402 |
| 3 | `ClientCounters` | U4e | the new kinds' counters | room/src/room.rs:63-136 |
| 4 | `ClientSlot`'s `queue`, `headroom`, `buffer_depth_ticks`, `kicked` | U4e | 2.8, -22's series, U4F-Q3 | room/src/room.rs:277-297 |
| 5 | the room's event cap default and flag; `EventQueue`, `PROVISIONAL_CAP`, `MAX_EVENT_AGE` | U4e | 2.8 | room/src/room.rs:460-461; room/src/events.rs:19-20, :22-31, :40 |
| 6 | `receive`, `receive_input` and the ack check | U4e | kind 8; the counting rule | room/src/room.rs:713-742, :744-801 (:783-787) |
| 7 | `fill_input` | U4e | option A's run path | room/src/room.rs:831-878 |
| 8 | `npt_room::BITES` | U4e | 9 new ids | room/src/lib.rs:38-46 |
| 9 | carry 21's room test | U4e | carry 80 (a), f9 | room/tests/room.rs:235 |
| 10 | -22/unbounded's tests | U4e | the bite's unit proof | room/tests/room.rs:1302, :1367 |
| 11 | `Counters`, `RttSource`, `Decoder`, `pump`, the ping answer, `pong`, `PING_INTERVAL_NS`, `OUTSTANDING`, `SAMPLES` | U4e (U4r's wallclock site) | 2.5; ClockSync's samples (2.1) | server/src/pump.rs:110-161, :183-190, :204-220, :372-480 (:458-472), :500-516, :16, :24, :26 |
| 12 | `npt_server::BITES` | U4b, U4r | unchanged | server/src/schedule.rs:252-257 |
| 13 | no-trailing and no-channel and their tests | U4a, U4e | kinds 9 and 11's bites | proto/src/quant.rs:34-42, :108-109; proto/tests/bites.rs:211, :238; server/tests/pump.rs:477, :483 |
| 14 | `ClientEndpoint` (fields, `new`, `with_lead`, `input`, `step`, `receive`, the too-old cases, `see`, `file`, accessors) | U4e (BRAWL-1's `own_soldier`) | 2.1, 2.2, 2.9 | client/src/lib.rs:208-243, :260-294, :296-301, :414-445, :450-490, :492-525, :528-565, :583-615, :645-658, :343-410 |
| 15 | `full_state` | U4e | carry 80 (e) | client/src/lib.rs:810-839 (the scout's lines; not read at night) |
| 16 | `npt_client::BITES`, `grid_with_bite`, `with_bite` | U4e | 3 new ids | client/src/lib.rs:28-40, :314-326 |
| 17 | `Behaviour`, `Kind`, `script`, `step`, `command`, `FAR_CM`, `MOVES` | U4r, BRAWL-1 | 2.6's cheats; NA-8 | bots/src/lib.rs:79-112, :180-188, :213-231, :272-304, :74-76, :32-49 |
| 18 | `toward_centre`'s doc and the dead-input test (TE-B1-R2-1's home) | BRAWL-1 | rides (section 9) | bots/src/lib.rs:128-132, :404 (the scout's) |
| 19 | `lead_for`, `io` (the pong's tick), the endpoint's build, the behaviour dispatch, `params` (`cmd_timing` "B"), the suite, `LAB_BITES`, `Bites::of` | U4r | 2.2, 2.3, 2.7, NA-4 | lab/src/scenario.rs:126-131, :458-509 (:459), :846-861, :1282-1299 (:1286), :1405, :204, :212-216 |
| 20 | `room_tick` | U4r, BRAWL-1 | 2.7's series; RR-B1-7 (1) | lab/src/logs.rs:495 (the scout's) |
| 21 | the contract checker | U4r | 5.3; T-6 | lab/src/contract.rs:1-25 (the scout's) |
| 22 | r2's suites; the judged-set runner's floor, byte rates, limits, judge fit, `planJudged` | U4r | 5.3-5.5 | ops/runs/c-u4r-r2.mjs:77-92; ops/runs/c-u4r-runs.mjs:131-132, :152-153, :171, :176-178, :324-332 |
| 23 | `World::BITES`, `World::with_bite` | U4d | NA-9's two sites | sim/src/world.rs:937-938, :944 |
| 24 | the map's extent; the weapon's reach | U4d | kind 6; the clamp cheat | sim/src/greybox.rs:62, :151; sim/src/scene.rs:66 |
| 25 | `ticks_for_ms`; `ACK_RING_TICKS`, `FULL_STATE_ACK_TICKS` | U4s | 2.2; f21 | sim/src/ticks.rs:35 (r-check :334's cite); sim/src/lib.rs:22-26 (advisor :439-440's cite) |
| 26 | the L1 profiles | U4c | 2.2's leads; 5.1 | l1/src/lib.rs:177-202 |
| 27 | `LEAF`, `WASM_CRATES` | U4d2 | section 6 item 2 | check.mjs:39, :1957 |
| 28 | marks.json's -20, -21, -22, -47 and SERIES.md's rows | U4j | sections 2.7, 5 | as 2.7 and 5.1 cite |
| 29 | u4r-run.sh (u4f-run.sh's model) | U4r | section 1 item 4 | N/u4r-run.sh (sha256 349b51ad… at U4r's launch: N/u4r-brief.md:265) |

---

## 9. Open items and follow-ups (the scout's list, N/night-u4f-scout-full.md:180-196; each routed or left)

| Id | Its home (file:line) | In this brief |
|---|---|---|
| C62H-BOUND | card :479-492; c62h-review :59, :63, :75-81, :132 | **open with Mustafa.** 2.9 marks where carry 80 (b) and (d) touch how the client files lost and too-old ticks; no lost-tick bound is designed here |
| C62H-KEEP | card :483-484, :492; c62h-review :64, :68-74, :128; carries :314-321 | **open with Mustafa.** 2.9's (d) marker; carry 62 (h) is not U4f's |
| KEPT-LOGS (KL-YES) | card :419-461, :508-509, :521; N/u4r-report.md:1174-1180; history :2088-2111 | **open with Mustafa.** 5.5 and U4F-Q2: U4f's sets may not fit before it (NA-5) |
| JDG-ONLY-1 and its interim rule | card :1467-1474, :33-34, :515-516; N/u4r-report.md:257-264; c62h :8-20, :44-45 | 5.4 applies the interim rule; the fix rides npt-judge's next change, not U4f's (5.6) |
| check.mjs follow-up | N/u4r-report.md:265-274; card :1527-1528, :515-516 | left: rides check.mjs's next change; U4f opens check.mjs only by U4F-Q8 (a), and then it rides |
| npt-server `--all-features` compile | card :1960-1961, :1985-1987; r-b2 :28 | section 6 item 4 if U4F-Q1 opens npt-server; else left for U4h |
| Option-A depth note | card :1960-1964, :1930-1933 | U4F-Q3 |
| NC-U4R-5 / EA-U4R-7, the logs half | r-r1 :78-81, :200-204; N/u4r-review-r2-full.md:15; N/u4r-report.md:281-282 | U4F-Q4 (-47's rejoins) |
| T-6, the contract half | r-r1 :119-123; N/u4r-report.md:283 | rides if 5.3 edits contract.rs: @@WORDS-N/advisor-u4r-r1-full.md:122-122@@ (+5 tests, 2.10) |
| TE-B1-R2-1 with RR2-B1-1 | N/u4r-report.md:1137-1141; card :163-171; bots/src/lib.rs:130-132, :404 | rides (U4f edits bots/src/lib.rs, its home lane): @@WORDS-N/u4r-report.md:1137-1141@@ |
| RR-B1-7 (1) | N/u4r-report.md:1162; lab/src/logs.rs:495 | rides (2.7's series land in `room_tick`): @@WORDS-N/u4r-report.md:1162-1162@@ |
| EA-U4R-4, EA-B1-1's later half, NC-U4R-7 | N/u4r-report.md:282-283, :1163-1164; N/u4r-build.md:1941-1943 | left by U4F-Q1's recommendation (crates/sim opened for two bite sites only, bites builds); they ride if the ruling opens crates/sim wider |
| EVQ-LOAD | b1-p3 :156-158; N/u4r-report.md:1048-1052, :1155-1158; carries :223-228 | 2.8 item 2 (WORDS placeholder) and U4F-Q12 |
| RR-5 room bench | e-r1 :211-217 | left: U4f edits no encode path by this brief; if it does, the bench before and after |
| Readings PR before R5 | advisor :150-153; wave1 :236-240; plan :970-983 | U4f's runs rest on readings already listed (Q22's -22 exception, Q16's clamp, W1-U4D's press twin); U4F-Q3's answer may read ADR 0001:1610's depth words, and then it joins the list (the advisor says) |

---

## 10. Questions for the advisor (each where the plan or a ruling leaves a choice open)

**U4F-Q1, the lane's files.** *Question:* which files does U4f's lane hold? *Why it matters:* the plan names none
(plan :805-816, :882-883), and two of -21's bites have their sites in npt-sim (NA-9), the new series need SERIES.md rows,
and -22's `rss_bytes` has no L1 writer (NA-2). *Options:* (a) section 4's proposed list, plus crates/sim/src/world.rs
for the two bites' ids and sites only (bites builds), plus marks/SERIES.md new rows only (U4R-Q2 (c)'s precedent,
with `rss_bytes`'s L1 writer named); (b) as (a) but the two npt-sim sites by the main session's own commit before launch,
so U4f's lane holds no sim file; (c) as (a) with crates/sim opened wider, so EA-U4R-4, EA-B1-1's later half and NC-U4R-7
ride. *Recommendation:* (a): the narrowest lane in which every bite has its site; the riders need public npt-sim items
that are not U4f's.

**U4F-Q2, the run budget where no figure exists.** *Question:* how are U4f's judged sets budgeted and ordered, given
that no time or size figure exists for -47, -21's suites, option A or a never-acker run, and the estimated total (5.5:
about 3,750 MB) exceeds the 3,593 MiB above the floor at 11fd190's reading (NA-5)? *Why it matters:* the runner's floor
stops a set part way, and nothing may be packed or deleted before KEPT-LOGS' kit PR (history :2088-2111). *Options:* (a)
the builder measures ticks per second and bytes per suite on its short runs (f28, f29), the main session restates 5.5
before the first set, runs the sets one at a time (-47, -22, -20, -21) with the floor projection, and tells Mustafa
now that -21's set needs KL-YES or an approved clean first; (b) a fixed budget now from 5.5's estimates; (c) no U4f
set before KL-YES. *Recommendation:* (a); evidence is never shrunk to fit (plan :940-941), and KL-YES stays his.

**U4F-Q3, option A's reading of `buffer_depth_ticks_count`.** *Question:* what do option A's runs record as the input
buffer's depth? *Why it matters:* the card's DEPTH ruling reads the header's `buffer_depth_ticks_count` as the depth set
(option B's pacing lead L) with the room's measure L - 1 (card :1930-1933, :1962-1964), while option A runs commands on
arrival, which ADR 0001:1166 counts as no buffer; -20 reports the depth (marks.json:4743-4759) and -48's bite reads it
under option B (plan :467). *Options:* (a) under A, `second.s.buffer_depth_count` 0 per client per second and the header
param 0; (b) under A, the controller's lead as the depth set and the measured lead below it, as B's; (c) under A, no
`buffer_depth_ticks_count` (it is -30's param: SERIES.md:240), `second.s.buffer_depth_count` 0, and the ticks each
command ran before its stamp reported beside it. *Recommendation:* (c); if it reads ADR 0001:1610's "the input buffer
depth the server settles on", it joins the readings PR.

**U4F-Q4, the logs' keying for a replaced session (NC-U4R-5, EA-U4R-7).** *Question:* does U4f key the lab's logs by
session? *Why it matters:* r-r1 :78-81 and :200-204 send it to the first unit whose runs log a replacement; no -20, -21
or -22 suite replaces a session (the never-acker is kicked, not replaced), but -47's 1,000 connects rejoin 48 slots
about 21 times each (2.7), and carry 62 (d)'s head rule allows no later admission before -19's join design (C-U6-11).
*Options:* (a) U4f keys `ended` and `gone` by session and resets the slot in `Logs::admit`, with a design for a later
join's ticks; (b) -47's suite writes no per-tick client hash records (its logs hold its connect events and the room's),
so no slot files a second session's hashes, and NC-U4R-5 stays for U6; (c) -47 measures 1,000 fresh ClockSync starts on
live sessions, no rejoin. *Recommendation:* (b): -47 reads no hash, and (a) is the join design U6 owns.

**U4F-Q5, the U4d-F ruling's TE2-5 stop clause.** *Question:* does the stop clause (df-ruling :465-468; the card's
"Rules in force", U4d-F item 2) still go into U4f's brief now that TE2-5's rule ended at U4r's merge (the card's M6
line; TE-Q2, te-m3 :119-126)? *Why it matters:* the clause records expected failures "under TE2-5", a rule no longer in
force; without the clause, a builder's failing step has only TE-Q2's after-clause (te-m3 :127-128). *Options:* (a) keep
the clause as the builder's step-group stop, its "TE2-5" read as the notes' failure record; (b) drop it with the rule;
(c) the advisor rewords it. *Recommendation:* (a): it costs nothing and keeps a failure from being retried unnamed.

**U4F-Q6, ClockSync's server-time stamp (NA-4).** *Question:* what server time does a ClockSync sample carry? *Why it
matters:* the room's `Pong` carries only the next tick to run (server/src/pump.rs:458-467; lab/src/scenario.rs:459),
answered on arrival (hold about 0), so each sample's server instant is known to 33.3 ms; at P2 and 10 pings a second my
estimate is a p99 near 6 ms against -47's 5 ms (marks.json:9079-9085), and a filter tuned past it might also mask the
tick-quantised bite. *Options:* (a) the Pong as built, with averaging over random phases; (b) the Pong gains a sub-tick
server time (for example µs since its tick's due instant, 2 B, inside the Pong's 32 B limit, catalogue :92), a wire
change with the catalogue's row and the codec's tests (the lane opens crates/proto and the catalogue); (c) snapshot
arrivals (each leaves at its tick's due instant on L1, plan.md:622) with the ping RTT's floor. *Recommendation:* (b):
it keeps ADR 0001:240's "from its own ping samples" and is exact off L1 too; (c) is exact on L1 only; the arithmetic
for (a) comes with section 11's D4.

**U4F-Q7, the lead controller's form, and `with_lead`.** *Question:* how does the controller set a command's tick, and
does `with_lead` stay? *Why it matters:* U4R-Q14's lead is newest-relative and profile-set (r-check :296-302); net F104
puts the controller on ClockSync's server-time estimate (u4-scout-netcode.md:122); the fast-clock cheat exercises the
lead bound only if its stamps run ahead with its clock. *Options:* (a) newest + L, L from the client's own RTT and
jitter samples by U4R-Q14's arithmetic; (b) ClockSync's estimate of the server tick at arrival plus a jitter margin and
the buffer target (ADR 0001:250-251); for `with_lead`: (i) kept as a lab-only fixed setting (recorded as
`lead_ticks_count`, SERIES.md:342) for U4r's suites and the -30 pre-check, (ii) removed. *Recommendation:* (b) with
(i), the settled lead per client reported (a new series by U4F-Q1).

**U4F-Q8, the lead bound's form and value.** *Question:* is the lead bound a tick constant or a room setting, and what
is it? *Why it matters:* the F2 ruling (advisor :463-465) names it the first case of carry 18's route; ADR 0001:231
makes every limit data; the value must exceed every honest lead (P4's 9 by U4R-Q14's arithmetic) and let the catch-up
kind be built under option B (16 commands for future ticks: 8 buffered, 8 refused), while -21's fast-clock check grows
with it (marks.json:5000-5011). *Options:* (a) `lead_bound_ticks_at(TICK_HZ)` in npt-sim by carry 18's route, in U4f's
lane or the main session's commit first; (b) a room setting in ms turned into ticks at the room by `ticks_for_ms` with
the rate passed (carry 39's pattern), recorded in the header; value 600 ms (18 ticks at 30 Hz) or 1,000 ms (30).
*Recommendation:* (b) at 600 ms: no new constant, the smallest round value above P4's lead plus the catch-up kind's
need; under option A the catch-up forger holds its commands back 16 ticks, then sends 16 strictly newer ones.

**U4F-Q9, per-kind attribution and the sequence check (NA-6, NA-7).** *Question:* how is a forged command counted under
its own kind, and how is a forged backward sequence told from an honest repeat? *Why it matters:* as built, a backward
sequence is a counted `Repeat` like every honest copy, and an old tick under option B is `late` like every honest late
command, so -21's "every forged command rejected and counted" (marks.json:4932-4943) cannot pass for kind 3, and the
no-newer-tick bite under B could pass as late. *Options:* (a) 2.4's counting rule (one kind per refusal, a fixed order,
per-tick (slot, seq, kind) records, strictly newer before late), and kind 3 refused when a command's sequence is not
forward and its content differs from the command the room received under that sequence (the last 8 per client kept,
bounded), an identical repeat dropped uncounted as now; (b) kind 3 as a sequence more than the redundancy window behind
the newest, refused and counted, a backward one inside the window dropped as a repeat; (c) any refusal counts for any
kind. *Recommendation:* (a): (b) leaves a forgery inside the window uncounted, and (c) hides NA-7.

**U4F-Q10, each bite run's option.** *Question:* under which option does each bite run whose option the plan does not
name? *Why it matters:* plan :462-463 names only not-older-than (fast clock, A) and no-catchup-cap (forged, A);
marks.json:5118 puts the no-* bites in the forged suite and refuse-not-clamp in the clamp suite, either option; each
bite run also needs `pre:PM-0001-2` to pass (README :323-324). *Options:* (a) -21's other bites under option A (each
check then stands alone: option B's late and full rules overlap kinds 1, 2 and 10), -20's redundancy-1 under option B;
(b) all under B; (c) all under both (13 more runs). *Recommendation:* (a): under option A with no repeats about 5 % of
P3's commands never run, so the shots fall toward PM-0001-2's floor (about 700 x 0.95 = 665: plan :437-445) and the bite
run risks INVALID, while under B the last command's repeat keeps the shots and the 99.9 % check fails (about 95 %).

**U4F-Q11, the suites' behaviours, profiles and references.** *Question:* (i) which behaviour gives -20 its 10,000
scripted presses; (ii) which profile do the forged, clamp, fast-clock and never-ack suites use (their parts select
none); (iii) what is the fast clock's "honest client on the same inputs"; (iv) does 5.3 also re-run -21's four option-B
suites with the real cheats? *Why it matters:* the brawl makes no press edges (bots/src/lib.rs:279-281); U4r's stand-ins
ran forged, clamp and fast-clock at P0 (c-u4r-r2.mjs:82-84); SERIES.md:156 compares with an honest movement it does not
define; the stand-ins differ from the cheats (NA-8). *Options:* (i) (a) the brawl with a fire press every second
command (one edge per command), (b) the open-loop presses script; (ii) (a) P0, (b) P2; (iii) (a) the bound's
arithmetic (server ticks plus the lead bound, times the top speed per tick), (b) an honest client in the same run on
the same open-loop inputs; (iv) (a) yes, four more short runs, (b) no. *Recommendation:* (i) (a), the real load r2's P3
suite showed; (ii) (a), as the stand-ins and with no loss to confound the twins; (iii) (a), since a second client
collides differently; (iv) (a).

**U4F-Q12, the event cap's value.** *Question:* how is plan §4.7's cap set from 2.8's measurement? *Why it matters:*
the formula (plan :335) gives about 1,008 at most for EVQ-LOAD's rate (NA-3), already overflowed at P3, and carry 50
caps it from above for -22 at about 3,900 (NA-12). *Options:* (a) the formula as written; (b) the largest per-client
high-water mark seen on P2, P3 and B1 times a stated margin, checked below the never-acker's rate x 255 / 30 s; (c) a
fixed 2,048. *Recommendation:* (b), its figures to the advisor before -22's runs (carry 57), plan §4.7's text amended to
state the measure and both bounds (the main session's commit).

---

## 11. Needs a run (day)

| # | Claim | Command or run |
|---|---|---|
| D1 | `<base>`'s counts (measured, bites, self, steps, marks, redact) | a counted check.mjs run at `<base>` through main-run.sh, in N/u4r-brief.md:1779-1784's form |
| D2 | the disk the sets need against what is free (NA-5; 5.5) | `df -BM /home/user/Shamble-Frontlines` and `du -sm /home/user/Shamble-Frontlines/.gate/*` |
| D3 | L1 P3's reorder share is above 0, so -20's runs meet the `reorder` rule | `grep -rhoE '"reorder_permille":[0-9]+' /home/user/Shamble-Frontlines/.gate/netcode-proof-test/runs/u4r-P3-0f066338dd8d \| sort -u` (the folder c-u4r-runs.mjs:141 names) |
| D4 | NA-4's estimate (p99 near 6 ms under the Pong as built) | the day session's model, parameters fixed here: 1,000 connects x 3 seeds; per direction a one-way delay of 50 ms + U[-10, 10] ms; 20 samples in 2 s; each sample's server stamp floored to a 33.333 ms tick at a uniform phase; the mean-of-samples and lowest-RTT estimators; report the p99 of abs(error). No such script exists in the repository; the night ran none |
| D5 | NA-3: BRAWL-1's P3 high-water marks exceed what plan §4.7's formula gives | read the kept BRAWL-1 P3 room logs' queue and overflow-resync series (the folder N/u4r-report.md's BRAWL-1 section names; SERIES.md's row at :358): the exact command needs that folder's name, which the night did not read |
| D6 | every cite of this brief still points where it says (section 8) | `git diff --stat 11fd190 <base> -- spikes/netcode-proof docs/adr/0001-multiplayer-netcode.md docs/design/netcode-proof-messages.md .claude/rules/netcode.md .checkpoints/netcode-proof-test/u4-build-plan.md .checkpoints/netcode-proof-test/u4s-carries.md .checkpoints/netcode-proof-test/plan.md .checkpoints/netcode-proof-test/history.md`, then each moved file's rows re-walked |
| D7 | each U4f suite's ticks per second and bytes per 10 minutes, under both options; ClockSync's p99 at P2; the controller's settled lead per profile | after the build: the builder's D-steps, each command line in its notes (as U4r's D4 and D13: N/u4r-brief.md:1771-1844) |
| D8 | the event cap's value (2.8) | after the build: 2.8 item 3's nine short runs under the runner (`-x`), commands in the builder's notes |
| D9 | `cargo test -p npt-server --all-features` compiles (only if npt-server opens) | `CARGO_TARGET_DIR=/home/user/Shamble-Frontlines/.gate/shared-target cargo test -p npt-server --all-features` from spikes/netcode-proof through u4f-run.sh (section 6 item 4) |
| D10 | marks.json and the judge unchanged at U4f's close (5.6) | `git diff --stat <base>..<head> -- spikes/netcode-proof/marks/marks.json spikes/netcode-proof/crates/judge` |

---

## 12. Review and launch

**Review** (plan §7.2 :932: one round on the final diff, then a short report to the advisor):
- U4f's reviewers (plan :816): netcode-architect, security-auditor (the hostile input: 2.4's kinds, the clamp points,
  the 1.5x clock, the lag switch, the never-acker and carry 76's Kick path), test-engineer, rust-reviewer; plus
  engine-architect on the public items (A19; plan :930-931; 2.13). Each at effort xhigh and counted toward the 3-agent
  limit (advisor :147, condition 10 (c); plan :886-887): five seats in two batches, the builder counted while it runs.
- The read rule: if marks.json or crates/judge changed, a reviewer's read is owed before U4f closes (5.6); the
  test-engineer's review can be it if its notes name the lines read and marks.json's sha256 before and after
  (c62h-review :84).

**The builder's seat** (plan §7.2 :925-927): one builder, general-purpose, opus, xhigh, in an isolated worktree; bites
planted by Edit in place under §7.1; merged by the main session, one unit at a time.

**W1-TARGET's lock and the lane runner** (the card's "Rules in force", its U4 line, card :3070-3072 at 11fd190):
every build of the lane takes W1-TARGET's flock lock through u4f-run.sh; the main session's counted runs through
main-run.sh, which holds it exclusively.

**Left for the main session at launch** (on N/u4r-brief.md §12's model, :1845-1909):
1. **`<base>` and its counts:** the spike branch's head at launch; its counts by D1; filled in section 1 item 1 and
   section 4 rule 5.
2. **Stamps:** the sha256 and line counts of plan, plan.md, carries, history and the card at `<base>` into section 0;
   a file that moved since 11fd190 has its cites re-walked (D6; section 8).
3. **The placeholders, by script, each asserted after the copy** (N/u4r-brief.md:1857-1886's rules: the anchor found
   once, the copy's first and last words named, a closed file's sha256 asserted first, a wrapped span read with each
   line break as one space): every `@@CARRY-n@@` from carries' range in section 3 (the entry number at line start
   asserted; carry 86 taken whole, NA-1); every `@@WORDS-...@@`: df-ruling :462-464 (the sentence inside T4 item 1's
   quotation marks); df-ruling :465-468 (the clause inside T4 item 2's quotation marks, its first `<crate>` and
   "<crate>'s files" replaced as section 4 says, no `<crate>` left), only if U4F-Q5 keeps it; j-r2 :298 (the words
   after "From now on: "); m9 :252-256 (the text inside W1-TARGET item 3's quotation marks); m9 :259-264 (from "(1)
   never pass" to "by hand."); d2-r2 :304-308 (from "never run cargo" to "use it as one."); te-m3 :127-128 (from "After
   it ends" to the line's end, NA-10); c62h :20 (the sentence from "Until the fix lands"); b1-p3 :158 (the quoted
   line); r-b2 :28; r-r1 :122; N/u4r-report.md:1137-1141 and :1162; carries :9-10; history :292-293 (Mustafa's words,
   by script only). The living files (the card, carries, history) by text anchor, as their lines move.
4. **The runner u4f-run.sh:** made from u4r-run.sh by script, only the lane's names changed (the header, the log
   folder .gate/u4f/, the usage line), its .diff beside it; its sha256 and `bash -n` exit 0.
5. **The U4F-Q rulings folded:** each "by U4F-Qn" read as the ruling's words, section 4's lane widened or narrowed to
   match; any plan text a ruling changes (plan §4.7's cap, §4.1's rows) in the main session's own commit, as U4r's
   rulings were (r-check :416-424).
6. **A constant, manifest or wire change a ruling puts before launch** (U4F-Q8 (a)'s carry 18 commit; U4F-Q6 (b) if
   the main session makes it): its own commit on committed state with its counted run (r-check :425-427).
7. **Mustafa's open answers** recorded on the card at launch, none decided here: KL-YES (5.5's disk), C62H-KEEP and
   C62H-BOUND (2.9's markers).
8. **This brief's checks:** the night's test-engineer and security-auditor checks and rev 2, then the advisor's check.
9. **The judged runs' target:** the main folder's own cache, `CARGO_TARGET_DIR` unset (plan :918-919), as U4r's
   (N/u4r-brief.md:1901-1904).

---

## Findings met while drafting (none is carried in carries: each is new)

| Id | Severity | Where (11fd190) | What goes wrong | Cheapest proof (day) |
|---|---|---|---|---|
| NA-4 | High (an estimate) | server/src/pump.rs:458-467; lab/src/scenario.rs:459 | the room's Pong gives only a tick, so ClockSync's server stamps are quantised to 33.3 ms; at P2 and 10 pings a second -47's p99 lands near 6 ms against 5 ms, or a filter tuned past it may hide the tick-quantised bite | D4's model, or the builder's first short clock suite at P2 |
| NA-3 | Medium | plan :335; b1-p3 :158 | plan §4.7's cap formula gives at most about 1,008 for EVQ-LOAD's rate, while 1,024 already overflowed at P3 without a burst: a formula cap resyncs clients in judged runs | D5 |
| NA-12 | Medium | plan :335; carries :180-183, :223-228; room/src/events.rs:19-20 | nothing bounds the cap from above, yet a never-acker's events expire at 256 ticks: a cap above about 3,900 is never reached and -22's headroom check (marks.json:5145-5155) fails a correct build | arithmetic; U4e's test room/tests/room.rs:1302 at the chosen cap |
| NA-5 | Medium (an estimate) | c-u4r-runs.mjs:153; the scout's Disk entry; card :521 | U4f's sets need about 3,750 MB against 3,593 MiB above the floor, and KEPT-LOGS is scheduled before U4g's build, not U4f's runs | D2 |
| NA-6 | Medium | room/src/input.rs:80-89; room/src/room.rs:795 | a backward sequence is a counted `Repeat` like every honest copy: -21's kind 3 cannot be rejected and counted apart | a unit test (f12) |
| NA-7 | Medium | room/src/input.rs:107-116 | under option B a forged old tick counts as `late`, and no lead bound exists: 8 far-ahead commands fill the buffer and every later honest command is refused `Full` | a unit test (f13) |
| NA-8 | Medium | bots/src/lib.rs:74-76; sim/src/greybox.rs:62, :151 | U4r's clamp script aims 32,000 cm out, beyond the ±12,800 cm map: with the aim check its points are refused, failing "clamp: never refused or counted" (marks.json:4966-4977) | a unit test (f18) |
| NA-9 | Medium | marks.json:5086-5088, :5110-5112; sim/src/world.rs:937-938 | no-cooldown's and refuse-not-clamp's sites are npt-sim's, outside U4f's row (plan :806-809) and §4.14: without a lane ruling they cannot bite | `grep -rn "no-cooldown\|refuse-not-clamp" spikes/netcode-proof/crates` (none today) |
| NA-1 | Low | carries :535-537 | carry 86 has no quoted text, so a fill script cannot find where its rule begins | the fill's assert (section 12 item 3) |
| NA-2 | Low | SERIES.md:93; marks.json:5182-5190 | `second.v.rss_bytes`'s writer is npt-server, which runs no process on L1: -22's report line has no L1 writer | `grep -rn rss_bytes spikes/netcode-proof/crates` |
| NA-10 | Low | te-m3 :126-128 (the night plan's "Read first") | the night plan cites TE-Q2's after-clause at :126-127; it is :127-128 | the fill's first-word assert |
| NA-11 | Low | plan :810; marks.json:9097, :9101 | the plan's bite words differ from marks.json's ids: a builder spelling the plan's words mints ids no bite entry matches (the bite NOT-RUN) | grep the ids in `npt_client::BITES` against marks.json at the merge |
