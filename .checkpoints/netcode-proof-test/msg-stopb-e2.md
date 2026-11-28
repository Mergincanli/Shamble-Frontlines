# E2: Stop B's message to him (DRAFT 0.3, 06:3x UTC on 11-28; sent after X, B6's read of X first; one screen)
Status: 0.3 after the advisor tool's read of 06:3x: the three cuts said once, in Decisions needed (his B.2 and B.3),
with codes of their own (a bare "(b)" would collide with HOST-LIMITS (b)); section 1 on R1.md's own dates (:267-282);
every cite re-printed 06:3x (R1.md:93-94, :242-246, :267-282, :323; r1.sh:38; rs.yaml:69, lg.yaml:52; plan.md:410).
At send: X's full SHA in For Mustafa 2; his host answers in section 3's last line; re-print the opening's "no kit, CI or
dependency change against main" and Hetzner's two pages (stopb-plan.md D4). The advisor tool reads it before it goes.

---
**Stop B: R1's run plan is ready for your review (R1 stays Thursday 3 December).**
R1's path is built and merged (U6a, U6b, U6c, U7, U8, with U8-F1 and UM-4). Stop B's fills are in: your two security
notes, the browser build's expected file hashes, and X: the run commit in both cloud-init files and Let's Encrypt's
terms link in R1.md. Main was taken in three times this week; PR #40 holds no kit, CI or dependency change against main.

**1. The pieces still on my lane** (estimate from the re-plan; likely = 3.3 times, the growth of the last four units):
| Piece | Estimate | Likely | Needed by |
|---|---|---|---|
| U4k, U4's kept server rows | 540-1,280 | 1,782-4,224 | R2, R3, R5 |
| U6d, room work | 1,800 | 5,940 (just under the 6,000 stop) | R2, R3 |
| U6e, control, metrics, rooms | 1,575-1,825 | 5,198-6,023 (its top end passes 6,000) | R2, R3, R5 |
At R1.md's cadence of about 1.5 days a unit (:269; the last five merged about one a day), the three take Fri 4 to about
Tue 8, with R1's report and the three run plans beside them. R1.md's run dates hold: R2 Sat 12, R3 Sun 13, R5 Mon 14,
Tue 15 spare, the close Fri 18 (:274-282). The lane ends about three days before R2, so a cut, a 6,000 stop or a plan
waiting on you spends that first, then Tue 15. R1.md:267-275 still counts MOVED and U6j as mine: both merged 11-26
through Builder 2; X may not edit it.

**2. Pieces Netcode Builder 2 could build alone** (your 19:14 item 2; its MOVED and U6j merged 11-26, 48efc9c5):
- B3, -30's container runner (from U6e): fence ops/runs/, one new c-*.mjs file (the container's run scripts live
  there: plan.md:410); planned 225 lines (150-300), about 740 at 3.3x; waits for nothing (J1, which its counted run
  needs, came with U6j: replan-marks.md:50). Needed by R3.
- H3, npt-super's rest (from U6e: several rooms per VM and the room kill): fence crates/super/ (653 lines today) plus
  one change in crates/server/ first: -27's built figure reads room ticks in phase `after` (figures.rs:1691-1692),
  which the room never writes (tick_loop.rs:555-559). Planned 300 at most, about 990. R5 only: with CUT-A it moves.
- Not alone: U6d, U4k and U6e's other parts change crates/room/, crates/proto/ or crates/server/ together.
With CUT-A: B3 alone. Without it: B3, and H3 once its server change lands on my lane.

**3. One line each:**
- The expected page list's Wasm carries wasm-bindgen's plain version, as the run server builds it; a first list
  carried our commit's name (through wasm-bindgen's build script) and was voided before the fill.
- R1.md:323 still says "no reading exists yet" for the probe's window: one exists (u6c-r2-netcode.md:90-93); named
  here, not edited.
- If bring-up finds the tick core shared (`alone` 0), the DelayStream self-test is skipped (r1.sh:38, R1.md:242-246):
  its clock thread never runs on the tick core; whether its numbers lean on the tick thread is a follow-up after R1.
- The count: your two notes add 2 lines to U8's files, so U8 stands at 6,284, your stop; the two test stand-ins were
  rewritten in place at 0 net (+2 from you if you count in-place lines); the fills sit outside the stop, as you ruled.
- The page host: <TO FILL: his HOST-LIMITS and HOST-SANDBOX answers, and 45 (kept as merged unless he said otherwise)>.

**Decisions needed** (your B.2 and B.3: what each cut saves in lines and days, and my lean; none decided yet)
- **CUT-A (your (a)): move R5 to Slice 6?** R5 is the 48-player run plus the 8-room soak (Mon 14 Dec); its eight
  marks (-27, PM-0006-13, -14, -16, -21 to -24) would be reported as MOVED, never as PASS.
  - Saves: about 1.5 days (the run day, its plan and its report), EUR 3.34 planned (cap 3.60), and at least about
    1,650-2,475 likely lines: U4k's "before R5" part (660-1,485), H3 (about 990), plus R5-only parts of U6e (the
    metrics exporter, the soak's log checks, the stall injection) that the estimates do not split.
  - Options: yes (R1-R3 close the test; Mon 14 joins Tue 15 as spare); no (R5 stays; Tue 15 the one spare day).
  - **Lean: yes:** R1-R3 prove the netcode; R5's evidence is hosting cost, which Slice 6 measures on the real server.
- **CUT-B (your (b)): one reviewer per piece, and security only on parts that face the internet during a run?**
  - Saves: a round has three seats today (netcode, test and security; U6c had two rounds); under CUT-B one, two where
    clients' bytes arrive: 2 to 4 fewer reviewer runs a piece. Usage only: a round's seats run as one batch, so no
    days and no lines. The advisor's head read stays.
  - Options: yes; no (three seats a round stay).
  - **Lean: yes,** security kept where clients' bytes reach the run server's open ports (rs.yaml:43-49): U6d's join
    change (finding SA-F1-1 still open), U4k's ack fix and, if R5 stays, the metrics exporter.
- **CUT-C (your (c)): a stop per piece at twice its estimate (U4k 1,080-2,560, U6d 3,600, U6e 3,150-3,650) in place
  of 6,000 lines?**
  - Saves: lines only where you then cut. At 3.3 times every piece reaches its line before it is done, so each adds a
    stop for your word.
  - Options: yes; no (6,000 stays, and each piece's size is reported against its estimate at its merge).
  - **Lean: no.**

**For Mustafa**
1. Review R1's run plan (spikes/netcode-proof/ops/runs/R1.md): its cap EUR 0.80 (planned 0.62) and its delete-by
   times on Thu 3 Dec, LG about 13:20 and RS 13:30 Amsterdam.
2. At Stop B (Tue 1 Dec, before any VM): R1.md's "Once" list (:57-94, about 75 minutes). Its step 3: `fsn1`, R1.md's
   location for both machines, is not listed today on Hetzner's pages for CCX23 or CAX21 (both list NBG1 and HEL1), so
   check it in the create form, else `nbg1`. Its step 7: approve settings.json and PROMPT-R1.md, OK the plan, its cap
   and delete-bys, and first read Let's Encrypt's terms: https://letsencrypt.org/documents/LE-SA-v1.8-July-06-2026.pdf
   The Cloud config, at X <TO FILL: X's full SHA in both links>:
   https://raw.githubusercontent.com/Mergincanli/Shamble-Frontlines/<X>/spikes/netcode-proof/ops/cloud-init/rs.yaml
   https://raw.githubusercontent.com/Mergincanli/Shamble-Frontlines/<X>/spikes/netcode-proof/ops/cloud-init/lg.yaml
3. Answer CUT-A, CUT-B and CUT-C. The day you do, I hand Builder 2 its pieces (section 2).
4. Nothing to check in Brave.
