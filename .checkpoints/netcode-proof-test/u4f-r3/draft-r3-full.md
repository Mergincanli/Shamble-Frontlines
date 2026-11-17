## U4f brief rev 3 (draft): written

- **The brief:** /home/user/Shamble-Frontlines/.checkpoints/netcode-proof-test/u4f-brief.md (1,407 lines, sha256 efedb237…).
- **My notes:** /home/user/Shamble-Frontlines/.checkpoints/netcode-proof-test/u4f-brief-r3-notes.md, ending DONE.
- **Two read-only scripts:** /home/user/Shamble-Frontlines/.checkpoints/netcode-proof-test/u4f-r3/stamps_anchors.py and /home/user/Shamble-Frontlines/.checkpoints/netcode-proof-test/u4f-r3/brief_check.py. Their output is quoted in the brief.
- **Head read:** 7cfae885ca58beba55a08ead2cdb31942f881f15. The saver committed 892081a and 42e0912 afterwards; neither touches a cited file (git diff printed nothing). spikes/netcode-proof is identical to 0ad9661's.
- **Stamps:**
  - u4-build-plan.md 43ff74f2… (1,100 lines; unchanged since 11fd190)
  - plan.md 3eecd281… (1,525 lines)
  - u4s-carries.md 57600c9c… (682 lines)
  - history.md 3bfe1aa2… (2,232 lines)

### Change count
- **28 changes**, C1 to C28, each with its source.
- The rev 2 counts table and NA table are kept and updated.
- **Tests:** +102 in the measured build and +117 in the bites build; 61 negative cases.
- **Placeholders:** 36 `<<P…>>` replace rev 2's 29 `@@…@@`. Each anchor is found once at the head.

### Recheck rows
- **33 walked:** 25 hold, 8 changed.
  - Changed: rows 10, 19, 20, 22, 27, 28, 29 and 31.
  - Why: the -22 test lines (10, a cite fix only); scenario.rs's +311 lines (19, 31); logs.rs `room_tick` (20); c-u4r-runs.mjs (22); check.mjs `WASM_CRATES` at :1970 (27); marks.json +26 and the moved SERIES.md rows (28); u4r-run.sh's new sha256 e9437ae2… (29).
- **12 rows added (34-45):**
  - C62H-BOUND's npt-l1 read and npt-lab link record.
  - npt-judge's two figures and their shared reader.
  - PM-0001-17's two checks, its planted bug, and the four frozen SERIES.md rows.
  - The set runner c-u4r-runs.mjs, and the lab's short P2 test.
  - U3C-HELD-1's held-bound check, the REFS-KIT exception, and the kept-logs lists.
  - `Room::leave` and carry 75's admission bound in the runner.

### Questions
- **None of U4F-Q1 to Q16 is fully answered** by a ruling or by Mustafa's words. All 17 remain open, each with a status line:
  - Q2: its premise changed. KEPT-LOGS §11 and PACK-READ (a) now allow packing by the C62H-BOUND brief's section 8.1, so rev 2's option (c) is gone.
  - Q16: its "open with Mustafa" context is answered.
  - Q5: has a precedent (the merge-stop ruling applied TE2-5's stop pattern after the rule had ended).
  - Q1: gains (d), whether U4f takes RR-UH1-2's SERIES.md half; (a) no longer gives `rss_bytes` an L1 writer.
  - Q4: gains carry 75's admission bound.
  - Q6: gains option (e), the room's own pings, which carry the server's clock in ns.
  - Q8: gains (iii), whether the lead bound's value goes to Mustafa.
  - Q11 (iii) (a), Q14 (c) and Q15 (b): now marked as bar changes.
- **New: U4F-Q17**, which of carry 80 (a)'s two routes U4f takes (the night card's Next step 4).
- **SC-1:** met in section 2.1's ping-period rule and in D4's phase-locked model.

### D-items
- **Filled (4):** D1 (the known numbers at 0ad9661), D5 (no log writes the queue's length; the EVQ-LOAD record is the proof), D6 (cites walked at the head), and D9's premise.
- **Left (11), each with its command and pass bar:**
  - Before the advisor's check: D2 (disk) and D4 (the ClockSync model, which decides Q6).
  - D3: before the u4r-P3-0f066338dd8d folder is packed.
  - At launch: D1, D2 and D6.
  - After the build: D7, D8, D14, and D9 only if a ruling opens npt-server.
  - At U4f's close: D10.
  - After the merge: D11, D12, D13 and D15.

### Bar changes for Mustafa
- **By the recommendations:** no marks.json text and no judge file changes. SERIES.md gets only new rows for report-only series, which are not bars.
- **One recommended for his yes:** B3, the lead bound's value (400 ms, 12 ticks), which sets how much -21's two fast-clock checks allow. A draft LEAD-BOUND entry is in section 12 item 6.
- **Three only if a ruling picks them, all -21's:**
  - B5: Q11 (iii) (a) rewords SERIES.md:156.
  - B6: Q14 (c) adds new bars.
  - B7: Q15 (b) changes a bar and adds a bite run.
- Each would go to him first in c62h-bound-paste.md's form, with the reviewer's read named in the report.

### New findings
- **NA-14 (Medium):** under option A, `tick.s.input_missing` must be read by the command's stamp. Commands run on arrival, so arrivals bunch, and the other reading would fail -20 on a correct build.
- **NA-15 (Medium):** `Room::leave` drops a slot's counters whole, so the lab must read them before routing a `Leave` or closing a session.
- **NA-16 (Medium):** -47's suite of 1,000 connects runs past carry 75's 32,768-event admission bound with fighting clients.
- **Changed:** NA-2 (the -22 `rss_bytes` report reads "not available", never INVALID) and NA-5 (1,418 MiB of room at the last reading, against about 3,600 MiB needed).

### Deferred Lows
23 findings in 10 rows.
- **U4f takes 4**, because it opens those files anyway: RR-CB-2, TE-B1-R2-1 with RR2-B1-1, and RR-B1-7 (1).
- **Left, each with its taker:**
  - The judge Lows go to JDG-ONLY-1.
  - TE-CBH-5 = NA-CBB-1 goes to the next change of c-u4r-runs.mjs.
  - U3C-HELD-1's eight go to npt-net's next change, by Q1 (d)'s recommendation.
  - CHECK-NFF-1 goes to check.mjs's next owner.
  - ADV-CBR-1 goes to U4g.

### Could not settle
- **D14:** whether Rust's sort panics on the non-total order. Toolchain 1.99.1's std source was not on disk to read.
- **Current free disk:** not read; no df was run. The 5,514 MiB figure is from E6, before U4f.
- **Planned size:** about 4,400 added lines (3,500 to 5,500) is an estimate by component. It is 1.5 times M's 3,000; Mustafa's stop stays at more than 6,000.
- **Mustafa's words in the anchors:** section 12 item 3 names each placeholder's first and last words, as u4r-brief.md's form does. That is the only place any run of 7 of his words appears in the brief, and section 0.5 says so.
