# R1's report: SKELETON WRITTEN 2026-11-28, BEFORE R1 RAN. NOTHING BELOW IS A RESULT.
Status: skeleton 0.1 (19:1x UTC on 11-28), notes only. R1 is Thu 3 Dec. Every `<FILL: ...>` names the file it is filled
from after the run: R1's results push to `claude/npt-results` (R1.md:294-301: `collect/`, `probe/presence.txt`,
`ds-selftest/ws_split.txt`, `bringup/facts.txt`, `s14/` whole, `s14-redact.txt`), the laptop session's records, and S14's
model side run here after R1 (R1.md:192-196). Rulings are cited by file:line, never copied (LEAN-MODE item 5). The
obligations below were collected by a grep for "R1's report" over R1.md, plan.md, the notes and the card (19:0x UTC).

## 0. Owed before this report is final
- MOVED merged before it (history.md:7863; replan.md:20, :92): merged 11-26 (48efc9c5), so moved parts read MOVED.
- The probe's three stamp tests, built here after R1's run (STAMP-HOME (a): history.md:7908; u6c-report.md:136-137).
- The burst setting `NPT_KC_BURSTS`, built here after R1's run (history.md:7910; the card's take-overs line); the burst
  half of S14 is judged with it (section 3).
- S14's model side, run here on the copied `s14/` folder (R1.md:192-196).
- The advisor's slack-value ruling from presence.txt's printed reads (R1.md:303-304; ADV-U8H-2, advisor-u8-head.md:172).
- The judge over R1's logs with C's -3 bite logs, part verdicts where a bite R1 does not run leaves a mark NOT-RUN
  (R1.md:345-351; u8-stopb-netcode.md:221).

## 1. What ran
- Location `<FILL: fsn1, or his Stop B pick; nbg1-plan.md>`; run commit Y `<FILL: 1ceecc32, or N>`; fill X
  `<FILL: 6e623788, or X''>`; both VMs' `npt-userdata head` showing Y (R1.md:19-20) `<FILL: bring-up prints>`.
- RS (CCX23) and LG (CAX21, or the held question's answer: nbg1-plan.md) created `<FILL: UTC>`, delete-bys
  (R1.md:130-131) `<FILL>`, deleted `<FILL: UTC>`; cost `<FILL: EUR>` against planned 0.62 and the cap 0.80
  (R1.md:127-129; prices re-read 2026-11-28 18:55 UTC, equal: stopb-plan.md D4's fourth read).
- Bring-up (`bringup/facts.txt`): C-U8-1's kernel, CONFIG_HZ, cpuidle driver, clocksource and THP (R1.md:228)
  `<FILL>`; C5's two refusals (R1.md:251-252) `<FILL>`; condition 4's `alone` and, if 0, the task list (R1.md:241-246)
  `<FILL>`; the dist against r1-dist.sha256 (R1.md:314-319: `DIST-DIFFERS` or not; `dist-accept` only on his typed
  word) `<FILL>`.
- Blocks against R1.md:27-44, each with its start, end and done or stopped marker: `<FILL: the session's records>`.

## 2. The marks, block by block (npt-judge's summaries in `collect/`; laptop readings rough, judged at R8: R1.md:5-6)
| Block (R1.md:27-44) | Marks or checks | Verdict and source |
|---|---|---|
| Headers | -1, -2, PM-0006-11, -16 | `<FILL: collect/>` |
| L2 probe, ImpairSocket's self-test | -3; C-U8-2, -4, -6 | `<FILL: collect/, probe/presence.txt>` |
| Calibration | load spread; -16 | `<FILL: collect/>` |
| Room starts | -16 | `<FILL: collect/>` |
| DelayStream's loaded self-test | condition 10 (C-U8-3, -5); -16 | `<FILL: ds-selftest/ws_split.txt, collect/>` |
| S14 veth and netem | TCP-FREEZE | `<FILL: s14/, model side here>` |
| Four-browser smoke, both transports | -14 (rough), -56's item 2 | `<FILL: collect/, the laptop's records>` |
| Dev certificate by hash | PM-0006-25 (rough) | `<FILL: collect/>` |
| QUIC-blocked fallback | -29 (rough) | `<FILL: collect/>` |
| Ping, laptop and PC | PM-0006-12 (a reading; decides nothing) | `<FILL: the record form's ping events>` |
| -36's home EU path | -36 (rough) | `<FILL: collect/>` |
| First bites | U6a's three room bites (R1.md:345-346) | `<FILL: collect/>` |
-1, -14 and -36 carry moved parts (replan-marks.md:213): those read MOVED, never PASS, and their kept parts are judged
(replan.md:20; replan-marks.md:205).

## 3. The instruments (R1.md section 5; the rules stay there)
- **S14 (TCP-FREEZE):** the kernel side from `s14/` (summary.txt, INVALID-s14.txt, rerun/) and the model side here;
  stall and burst-recovery p50 and p99 against compare.py's fixed rule (R1.md:180-196) `<FILL>`. This report records
  the comparison, and the switch if netem wins, under NETEM-BAR (a) (R1.md:196-200) `<FILL>`. The burst half is judged
  with `NPT_KC_BURSTS`; without it, this report says the burst half is NOT-RUN and why, and it goes to the advisor
  before -18's WebSocket part counts (R1.md:200-202) `<FILL>`.
- **DelayStream's loaded self-test:** per direction from `ds-selftest/ws_split.txt`, the judged form and its run
  conditions at R1.md:212-220 `<FILL>`; not run if `alone` read 0 (R1.md:241-246) `<FILL>`.
- **ImpairSocket's self-test:** per direction from the probe's P1 leg (R1.md:221-227) `<FILL>`; U3D-MISS's sampling
  case side by side if it applies (plan.md:1256: n, E, -3's count above d + 1.05a, n × E / 2a) `<FILL>`.
- **Per probe leg, reported and never judged:** the drops' rise, the up span's excess, the rise less the excess
  (R1.md:235-237) `<FILL: probe/presence.txt>`.
- **The slack-value rule:** the advisor's ruling at this report (R1.md:303-304) `<FILL>`.

## 4. Known limits (written now; each stays an open finding with its taker until it is closed)
- **The page host's connection limits** (his HOST-LIMITS (b), 11-28: history.md:8263, :8265): it sets no limit on
  connections or their waiting time. Taker: the security baseline, before the first online playtest.
- **The page host's sandbox options** (his HOST-SANDBOX (b), 11-28: history.md:8264-8265): five of its service's
  sandbox settings are not set. Taker: R8, at the latest the security baseline. The R1 reading of the unit's
  sandbox is dropped. No file the run server installs changed for either.
- **The probe's first drop reading** (his NPT-EDGE-SLIVER (a): history.md:7943): it comes just after a leg starts, not
  before; the gap is under a millisecond by reading the code, not measured; the measured window stays fully counted.
- **No hostile client** (R1.md:8-14): R1 holds no throwing or rejoining hostile client and no client restarted mid-run;
  SA-F1-1 stays open until U6d merges.
- **Laptop readings** (R1.md:5-6, LAPTOP (b)): rough, not judged; R8 judges the seven marks.
- **If `alone` read 0** (R1.md:241-246): the blocks on the tick vCPU are not run; whether DelayStream's numbers lean on
  the tick thread is a follow-up (Stop B's message, section 3).

## 5. Stale words (named, not edited before R1; fixed after it by their own change)
- R1.md:267-275 counts MOVED and U6j as this lane's (both merged 11-26 through Builder 2, 48efc9c5).
- R1.md:322-323 says U6c's container reading of one probe leg does not exist yet; one does (u6c-r2-netcode.md:90-93).
- npt-host.service:6-8 and cloudinit.test.mjs:215 say the page host's remaining sandbox settings come at U6c's merge
  (HOST-SANDBOX (b) moved them to R8).
- If nbg1 was picked: nbg1-plan.md's "Left as they are" list.

## 6. Open findings
PR #40's closing list, with their takers `<FILL: read at the report>`.

## Decisions needed
`<FILL after the run>`

## For Mustafa
`<FILL after the run>`
