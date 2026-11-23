# U8 Stop B round 2: netcode-architect seat (NC-R2), xhigh

Plan: (1) check hashes; read advisor-u8-stopb.md :227-285 (items) and :294-305 (round-2 scope); u8-build.md "Fix pass 2".
(2) interdiff old (.gate/u8-r2-old) vs new (W) for items 1-5, 10-15, 20, item 16's F9/F20/NC-SB-18 lines.
(3) per item: closed or not, against W and merged code. (4) builder's choices. (5) verdict.
Status: all parts done.

Inputs checked: advisor-u8-stopb.md sha256 daf55033...0859192 (matches); wip/u8-build.patch bca52b2c...e6eb7 (matches);
u8-build.md "Fix pass 2" read (sha256 6a67751d). Old tree: .gate/u8-r2-old/spikes/netcode-proof (60205020's). New tree:
W = .claude/worktrees/agent-ac59cce3e5d002441/spikes/netcode-proof, read only. Interdiff read for r1.sh, npt-lib.sh,
npt-load-ctl, npt-s14, probe.py, compare.py, lib.mjs, R1.md, PROMPT-R1.md, and the new r1sh test cases. Merged code
read (M = spikes/netcode-proof/crates): log/src/lib.rs read_log (:636-734), judge/src/lib.rs (:18-46),
super/src/main.rs, server/src/main.rs, net/src/l2/socket.rs (:319, :362), marks/SERIES.md (:564-565).

## VERDICT: REVISE
The four round-1 Highs are closed as the ruling words them. One new High comes from the fix pass itself (NC-R2-1): the
probe block saves LG's npt-log as `.jsonl` after stopping LG with TERM. That is the probe block's normal path, so
collect's whole-tree read fails on every R1. Once the whole-tree read fails, the bites and pooled checks are lost,
and the probe folder's own read fails too. Count: 1 High, 2 Medium, 3 Low.

## The round-1 Highs
- **NC-SB-1 (judge's file rule): closed.** Every npt-log path now ends `.jsonl`, the one extension M judge/src/lib.rs:27
  reads: the room log (r1.sh:259); npt-super's log, through npt-lib.sh:221's pattern and :224's `super-` name; the
  probe's legs, with the leg before the extension (r1.sh:555); keep (r1.sh:392-393); room_up and the waits (r1.sh:286,
  :290, :377, :521); the alone read (r1.sh:451); LG's log_path (r1.sh:337) and its RS copy (r1.sh:504, :531, :603,
  :632); the browser legs (lib.mjs:162). Test 7 reads the extension from the judge's source (r1sh.test.mjs:272-293).
  ADV-SB-1's per-block fallback is built (r1.sh:713-730), but it does not protect the probe block: NC-R2-1.
- **NC-SB-2 (windows inside the load; the READY guard): closed.**
  - Headers and calibrate rooms run 300,000 ms with LG at 330 s; ds runs 420,000 ms with LG at 450 s (r1.sh:228,
    :230, :231, :369).
  - The block waits for npt-super's room_exit, then LG's end (r1.sh:376-381). -2's measured ticks, from 60 s to the
    room's end at OPEN + 300 s, all fall while LG's 48 clients run (LG ends about OPEN + 333 s). Condition 10's pool
    runs from the mark to the room's end, OPEN + 120 to OPEN + 420 s: exactly the ruled 5 minutes, with LG on
    (u6b-brief.md:46-58).
  - READY is due 10 s before the warm-up ends (r1.sh:370). Measured from OPEN that is about 51.3 s, since t0 is taken
    after room_up's new 1 s check: still about 8.6 s before the first measured tick.
  - The guard's worst case is NC-R2-2 (Medium). Calibrate's LG outliving its room is NC-R2-3 (Medium).
- **NC-SB-3 (six consistent leg runs): closed on U8's side.** probe_presence is given the settings' run (r1.sh:533). It
  reads each leg's header (run `<settings run>-<leg>`, profile = the leg, process probe, layer L2) and its footer, and
  makes any miss INVALID (r1.sh:562-576). The fixtures cover the settings run on a leg, two legs sharing a run, a
  wrong profile and a missing footer. The interface line is in R1.md section 8, and the header form is in U6c's message
  (advisor-u8-stopb.md:212-213). With `-aN`, the leg names and runs carry the attempt consistently
  (room-02-a2-P0.jsonl, R1-probe-02-a2-P0). npt-super's log and LG's probe log share the settings' run (super's
  profile P1, LG's from the settings). Neither has process `probe`, so no part of -3 matches them.
- **NC-SB-4 (S14's n, ranks, overlap guards): closed.**
  - The sample: steady messages k x gap < 599 s give 17,971 a run. At 2 % loss that is about 352 episodes a run and
    about 1,057 pooled (sd about 32), so the stall p99 is rank ceil(0.99 n), about the 11th largest. It sits well
    inside the slow-repair tier, which holds 2-4 % of episodes.
  - 34 bursts x 3 runs give 102 recoveries, so the burst p99 is the 2nd largest. The 10 s spacing leaves about 7.5 s
    of steady flow after each recovery of about 1.2-1.5 s.
  - One schedule in both files: probe.py:31-35 and compare.py:50-52 agree on 28,500 messages, and `compare.py env`
    passes the same count to the model. The noise guard now reads the steady window's own retransmissions: probe.py's
    client reads them at i = 17,971, before the first post-window send, and compare.py makes an unread value INVALID.
    Episodes merged from back-to-back losses keep retransmissions above episodes, so the cut at 599 s cannot flip the
    guard. NC-SB-17's recording takes the data direction's window (probe.py bursts), so the start offset is one `tc`
    call, inside the 20 ms slack.
  - The overlap is sound as the ruling reads it. S14's traffic stays on its own veth and netem (ops/sbin/npt-s14:39-50),
    away from eth0 and the root namespace's nftables, including `quic off`'s set. The browser rooms are light. Any
    harm shows: a receiver wake delay below half a gap biases L by milliseconds against a 13 ms band; one above it is
    a VOID; send lateness is INVALID. The stamps are kept (r1.sh:695, :705, :661).

## The other items in my scope
- **Item 2 (INF-SB-1, NC-SB-11): closed.**
  - Only browser rooms name the production pair (r1.sh:232-237); every other kind writes `ws_cert_chain`/`ws_cert_key`
    null.
  - load_settings pins the room's hash file unless the room names the production chain (r1.sh:326-329). The outage
    path in R1.md section 3 now holds.
  - The null-pair form and "the WebSocket presents the same leaf as the hash file" are U6b's to settle at its merge;
    R1.md names both. If U6b serves WebSocket from a different leaf, ds's LG cannot pin, so no READY and the run reads
    INVALID, never a FAIL.
- **Item 10 (NC-SB-8): closed in the ruled form.**
  - `alone` is read after room_down, from the closed log, as the minimum of the affinity events (r1.sh:448-451, :179).
  - Under TIMING-INVALID, chain_run records the five timing blocks as "not run: alone 0 (C1)" beside the task list,
    and calibrate and s14 still run (r1.sh:750-755).
  - collect's judge.txt opens with the C1 line (r1.sh:718).
  - Two Lows: NC-R2-5 (the breadth of the skip; one stale PROMPT line).
- **Item 11 (NC-SB-10): closed.**
  - An EXIT trap brings every opened room down (r1.sh:86-102).
  - busy() refuses a block while a room's holder or another chain runs, with S14 exempt (r1.sh:105-115).
  - `-aN` is carried into run ids and log names (r1.sh:221, :277, :391); a done block without an INVALID file is refused
    a re-run (r1.sh:94-96).
  - The LG fallback (deadline_ok, r1.sh:78-81) is sound: if lg-boot is unrecorded, LG is taken as made 15 minutes
    before RS. That is the earliest his 09:15 step allows, so it errs early, and the 20-minute margin covers more.
- **Item 12 (NC-SB-7): closed for the header fields.** load_settings passes role, layer, profile, scene and tick_hz
  (r1.sh:339), so LG's log agrees with its room's run. The new verb is fixed-path (npt-load-ctl:76). Saving its output
  is NC-R2-1.
- **Item 13 (NC-SB-9): closed in logic.** The rate per direction comes from l2_summary's window counts, 10,000 a second
  within 1 %, and an unread count is INVALID (r1.sh:583-587). P1's slack and policy reads are checked at both edges
  (r1.sh:588-589). Where those reads are placed: NC-R2-4 (Low).
- **Item 14 (NC-SB-12): closed.**
  - The guard is above.
  - s14_run writes `compare.py summary` on RS (r1.sh:637-644).
  - The re-run rule is built (r1.sh:647-659). On the planned timeline it always falls to R2: S14 ends about 12:09, and
    12:09 + 48 + 5 minutes passes 12:57. The ruling expects this.
- **Item 15 (NC-SB-5, NC-SB-6): closed.** The probe lists PM-0001-3 only. -16 is on alone, start, headers, calibrate,
  ds and every browser room; bites keep their own marks (r1.sh:226-241). R1.md's rows match.
- **Item 20: closed.**
  - NC-SB-13: room_up dies when room_exit follows room_start within 1 s (r1.sh:286-293). npt-super writes room_exit
    straight after room_start for a deadline kill and for an exit before OPEN (M super/src/main.rs:262-278,
    :325-341), so 1 s is enough once F28's flush is in.
  - NC-SB-14: a clock deadline (r1.sh:347-355).
  - NC-SB-15: both ends trimmed (r1.sh:250).
  - NC-SB-16: the interface line in R1.md section 8.
  - NC-SB-17: per-direction windows (probe.py).
- **Item 16's lines: closed.**
  - F9: R1.md section 5 calls it a validity rule, never a pass bar.
  - F20: R1.md section 8 lists each bite with its judge, and says R1's report reads part verdicts.
  - NC-SB-18: PROMPT-R1.md section 4 runs collect once the server chain ends (about 12:09, with him present).
- **F28:** I read the waits assuming npt-super's flush. room_up's room_start and room_exit reads, lg_block's room_exit
  wait and the probe's exit wait all hold. The alone read no longer needs a live read.

## The builder's own choices
- **Null `ws_cert_chain`/`ws_cert_key` for a room's own certificate: sound as an interim form.** U6b's merged struct
  decides, and test 7 reads it at the merge. Any mismatch fails loudly: an unknown field or wrong type is refused, and
  a wrong pin means no READY, which reads INVALID.
- **`l2_timer_slack_ns` and `l2_sched_policy`: the names are right; the place is open.** They are ImpairSocket's header
  params (M net/src/l2/socket.rs:319, :362; SERIES.md:564-565, "read ... when the header is built"). See NC-R2-4.
- **LG taken as made 15 minutes before RS: sound** (above).
- **npt-load-ctl's start deleting the previous LG npt-log (npt-load-ctl:61): sound.** It is needed if npt-load refuses
  an existing path, as U6a's logs do. The one cost is in NC-R2-6.

## New findings (in the changed lines)

### NC-R2-1 (High) LG's npt-log is saved as `.jsonl` even when cut or empty, and the probe block cuts it on its normal path
- **Where:** r1.sh:517, :527, :531; also r1.sh:504, :603, :632; npt-load-ctl:67-70.
- **What fails:** the probe gives LG a duration of PROBE_TOTAL_S + 60 = 792 s (r1.sh:517). The block waits for the
  probe's own exit, at about 732 s, then stops LG (r1.sh:527). That is TERM, with KILL after 10 s (npt-load-ctl:67-70).
  npt-load has no signal handling (tokio without `signal`: SEC-U6AD-1's set, advisor-u6c-brief.md section 4 item 4),
  so it dies with whatever its log writer had flushed. The copy is then saved as load-02.jsonl (r1.sh:531).
  - read_log refuses a cut line ("not one JSON record") and an empty file ("empty log: no header")
    (M log/src/lib.rs:656-658, :722-726).
  - npt-judge never skips a bad log (M judge/src/lib.rs:36-46).
- **Scenario, every R1:**
  - collect's whole-tree call fails and falls back to per-block judging (r1.sh:719-726).
  - The probe folder's own read then fails on the same file, so -3's six L2 legs and ImpairSocket's self-test get no
    verdict on the day.
  - The per-block verdicts of the other folders lose the bites and the pooled checks, which the whole-tree call exists
    for (advisor-u8-stopb.md:76). -16's double-load bite and PM-0006-11's no-smt-map bite sit in the bites folder,
    so the starts and headers verdicts read NOT-RUN.
  - The laptop reads only these summaries (R1.md section 7).
  - The same happens whenever `lg "npt-log NN"` meets a stopped or unreachable LG: the READY guard's `lg stop`
    (r1.sh:372), a die after `lg start`, or an LG that is down.
- **Dependency:** this needs npt-load's log to be buffered across line ends. That is the pattern of every merged
  writer (npt-log's Writer over a BufWriter) and it is not yet built (U6c). A file cut at a line end with no footer
  reads, and only makes the super-plus-LG run INVALID.
- **Smallest fix:**
  - In the probe block, after the probe exits, wait for LG's own end: `load_wait 02 120`, since LG ends at its
    duration with its footer. `lg stop` stays the abort path only.
  - In every LG block, save LG's copy under `.jsonl` only when it is non-empty and its last line is a footer record.
    Otherwise save it as `load-NN$ATT.jsonl.cut` beside the others, which the judge's rule skips, and name it in the
    block's results.
  - A test 7 fixture: a cut LG copy leaves collect's whole-tree call passing.

### NC-R2-2 (Medium) The READY guard can still let measured ticks run with fewer than 48 clients
- **Where:** r1.sh:370-374; r1.sh:347-355 and :67-70.
- **What fails:** on a missed READY the guard runs `lg stop` before `room_down` (r1.sh:372-373). load_ready's last poll
  can also start just before the deadline and run into it, since each `lg` call has a 10 s connect timeout and no bound
  once connected (r1.sh:67-70, :349-351). In the normal case the room is down by about OPEN + 54 s, before the warm-up
  ends at about OPEN + 59.9 s. Each slow ssh or slow TERM adds seconds.
- **Scenario:** LG stalls or becomes unreachable during the warm-up (the case where READY fails). A 10 s connect
  timeout in the last poll, then another in `lg stop`, puts room_down at about OPEN + 72-81 s. Up to 600 measured ticks
  then run with fewer than 48 snapshot clients, so -2's minimum check FAILs instead of reading INVALID, and a FAIL is
  kept (ADR 0001:1582). Q1's guarantee, "before its first measured tick", fails in exactly the case it exists for.
- **Smallest fix:** on the deadline, `room_down` first, then `lg stop`. Also bound each READY poll to the time left,
  for example `timeout $((deadline - now)) ssh ...` or a lower ConnectTimeout in load_ready. Flip the order in the "2.3"
  trace test (r1sh.test.mjs:340).

### NC-R2-3 (Medium) Calibrate's LG now outlives its room by 30 s, so the calibration reads seconds with no server
- **Where:** r1.sh:230, :369, :601.
- **What fails:** calibrate took headers' durations: room 300 s, LG 330 s. U6c's calibration summary includes "each
  client's inputs a second, smallest and largest" over the run (u6c-brief.md:233-235), and a client whose link closes
  stays out. LG's last ~33 s therefore have no inputs.
- **Scenario:** the smallest inputs a second reads 0, so the calibration says LG fails V-S's 30 inputs/s rule
  (marks/README.md:269-276) when it did not. That can feed a wrong LG count into R2's plan, where V-S does judge.
  Calibrate carries no measured mark, so its room needs no end inside the load.
- **Smallest fix:** calibrate keeps duration_ms 0 (or a room duration of LG's plus 30 s). The block keeps the old order:
  LG's end, then room_down. Correct the "2.3" test's calibrate row. Alternatively, U6c's summary counts connected
  seconds only; if so, say it in U6c's message.

### NC-R2-4 (Low) Where P1's slack and policy reads sit is not the plan's pair
- **Where:** r1.sh:550, :588-589.
- plan.md:1256's rule compares two reads: the header's and the measured window's start edge. The names are
  ImpairSocket's header params (SERIES.md:564-565). probe_presence looks for them inside `leg_edge` at `warmup` (before
  the warm-up) and at `after`.
- If U6c writes them only in the leg header's params and at the window's start edge, a correct P1 reads INVALID.
  That is safe, never a false pass, but it costs the self-test.
- **Fix:** U6c's message names which records carry the two reads, and probe_presence reads exactly those. This could
  be one line in the interface list.

### NC-R2-5 (Low) The C1 skip is wider than what alone 0 touches, and one PROMPT line is stale
- **Where:** r1.sh:37, :750-755; PROMPT-R1.md section 2.
- Only -1 reads `alone`. The probe pins nothing, and starts' spawn-to-Open and DelayStream's release lateness run on the
  housekeeping vCPUs. Yet under alone 0 the chain no longer runs probe, starts or ds-selftest, so R1 loses -3's L2
  probe, ImpairSocket's self-test and condition 10 for a state none of them reads.
- This is the advisor's ruled form (advisor-u8-stopb.md:40), so it is a question, not a fix: would the skip be headers
  and bites only?
- PROMPT-R1.md section 2 still says "the timing blocks are recorded INVALID"; it should say "not run (C1)".

### NC-R2-6 (Low) An attempt's LG stdout copy is overwritten, and an unfetched LG npt-log is lost at the next start
- **Where:** r1.sh:503, :530, :602, :631; npt-load-ctl:61.
- `lg "log NN"` is saved as load-NN.log with no `-aN`, so a re-run in the same folder overwrites the first attempt's
  copy. Every attempt is meant to be kept (NC-SB-10's ruling).
- npt-load-ctl's start deletes LG's previous npt-log. A block that died before its fetch loses that log at the next
  attempt.
- **Fix:** load-NN$ATT.log; and fetch the npt-log in the EXIT trap path too (down_all), before any re-run.

## What I could not check
- No VM, no U6b or U6c code.
- npt-load's log buffering and TERM behaviour (NC-R2-1) and the probe's placement of the slack and policy reads
  (NC-R2-4) come from the briefs and the merged writers' pattern.
- The calibration summary's use of disconnected seconds (NC-R2-3) comes from u6c-brief.md's wording.
- I ran no test.

DONE
