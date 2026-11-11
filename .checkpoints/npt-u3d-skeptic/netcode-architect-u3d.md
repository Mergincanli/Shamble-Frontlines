Task: npt-u3d-skeptic (night-shift item 5) - netcode-architect skeptic of U3d (commit 1ccff06), angle: measurement validity.

## Plan (blocks, in order)
- B1 Read the approved plan for item 5 (night-2026-11-10.md at 2dc1f31: Rules, Item 5).
- B2 Read U3D-BAR's words: u3d-plan.md, advisor-u3d.md:146-271, plan.md R-31 (~:1403), C-U6-1..7 (~:1225), C-U8-1..6 (~:1227), u3d-report.md.
- B3 Read the diff 1ccff06 vs d012437 (socket_tests.rs, SERIES.md, run.sh).
- B4 Read the driven code: l2/socket.rs, impair.rs, stats.rs, thread.rs (slack rule), and what they call.
- B5 Read the dry-run record selftest-20261109T035556Z.txt (78b0c07).
- B6 Read earlier reviews (7 files) to avoid re-reporting.
- B7 Analyse Q1 (E == U3D-BAR quantity), Q2 (R-31 "never the reverse"), Q3 (C-U8-6 counters), clocks, inbound task stalls; write findings.
- B8 Summary + DONE.

## Notes
- B1 done. Rules: findings with file:line at df8975b, severity, scenario, cheapest proof; "needs a run (day)" + command; carried items marked.
- B2 done. Key texts:
  - U3D-BAR (advisor-u3d.md:113-115): "E <= 10,000 ns per direction, inclusive". U3D-L0 (b) (:116-118): judge E with L0 = 0 for ImpairSocket. U3D-WINDOW W1 (:119-120): the probe's E over the P1 leg's measured records (C-U6-5).
  - Plan's E: E[(L - L0 - 0.25 ms)+], L = out - due, over histogram sample set (finished fates but Oversize), divided by n (u3d-plan.md:95, :177-179, T5 :344).
  - Derivation (u3d-plan.md:108-111): p99.9 upper check d + 1.05a holds iff E[(L' - 0.25ms)+] <= 0.001 x 2a, L' independent of the delay. The close ruling (advisor-u3d.md:206-217) corrects with cap: P = E[min(1,(L-0.05a)+/2a)] (u3-verify-timing.md:46); E overstates past 2a+0.25ms; "never the reverse".
  - R-31 amended text: plan.md:1423 (not :1403 as the brief says; at df8975b it is line 1423). C-U6 at plan.md:1245, U8 row incl. C-U8-1..6 at :1247 (brief's ~:1225/:1227 are the old numbers).
  - C-U8-6 text (plan.md:1247; advisor-u3d.md:251-270): before and after every L2 probe leg, the socket's own drops column in /proc/net/udp(6) matched by local port, and /proc/net/snmp Udp lines; "a drop on the probe's P1 socket between the two reads makes ImpairSocket's self-test INVALID".
  - Report known limits (u3d-report.md:96-98): reader stall delays t_in, never enters L; over ~28 ms only kernel drops.
- B3 done. Net crate unchanged 1ccff06..df8975b (only l1 doc comment, sim crates), so 1ccff06 lines = df8975b lines.
  - socket_tests.rs self-test :1352-1694. E = excess(sm[d].late, 0) (:775-782, :799), late = out.saturating_sub(due) over in_e_set (Delivered|SendError|WouldBlock) (:715-717, :743), whole fate log = socket's whole life (no window), integer floor division by n_E.
  - Cross-check delay = out - t_in, Delivered only (:745-750); p999 nearest rank (:759-772); within inclusive (:876). Bands: p999 = d+a +- a/20 (:846). Agree line compares would(E) to x.pass[5] (:904-910, :1593-1600).
  - Stop: pacers joined + (d+a) + 500 ms, then notify_one (:1451-1458). Reads after stop (:1462-1471).
  - Run conditions (:1637-1693): rate, touched, fate-log-full, own drops, delivered+lost==n (the only kernel-drop catcher), sm.fates==n, early==0, loss band, no send err, poller count == delivered, slack.
  - run.sh reads NO UDP counter (/proc/net/snmp, /proc/net/udp) around runs; only /proc/stat + uptime. (C-U8-6 came at the close, after the dry run.)
  - start_poller :676-712: loops poll_recv until Pending, then polls stop; first_at stamped with Instant::now on first Ready.
- B4 done. Facts:
  - One clock: Shared::now_ns = epoch.elapsed() (Instant, CLOCK_MONOTONIC) socket.rs:113-115. Reads: try_send BEFORE the lock (:465-466: conservative, t_in early); reader after recv BEFORE the lock (:607-608: conservative); poll_recv AFTER the lock (:501-503: includes lock wait; one stamp for up to 32, taken before the copies and finishes); release: pop under lock with now from :678, out per datagram after each try_send outside the lock (:724-726), finish under lock later (:729-735). Wake of poller before sends (:712-714).
  - Condvar wait_timeout (CLOCK_MONOTONIC futex) :696-700; slack only on the release thread (thread.rs:39-55); policy read only at header time (end of run).
  - Reorder step impair.rs:558-572, :587-603: a held datagram (1 %: l1 lib.rs:83 permille 10) whose successor is taken in before its due becomes due = successor's due + 1 (:566), with its OWN t_in kept (:430 push_datagram uses h.t_in_ns). So its -3 delay = (t_next - t_held) + D_next + 1 + L, while L is measured from the moved due: an intake gap G enters -3's delay (for held datagrams) and never E.
  - Delay = d - a + U[0,2a] integer (impair.rs:121-124); one_way = (rtt - base)/2, base 0 on loopback (config.rs:142-144).
  - Lost etc. logged at intake with due 0 out 0 (impair.rs:477-488, :573-583), not in E set. late = out.saturating_sub(due) :681.
  - E floor: excess() (socket_tests.rs:781) floors; would_one `<=` (:805) => a true E in (10,000, 10,001) prints PASS.
  - Fate pre-touch writes all-zero Fate values (socket_tests.rs:1388-1403).
- FINDING CANDIDATES (to finalise in B7):
  - NA1 (Medium?): "exactly"/"never the reverse" (R-31 plan.md:1423; advisor-u3d.md:216, :235-237; u3d-report.md:69, :99) hold only in expectation; -3's p99.9 is a count of ~990 above threshold out of 990k delivered, sd ~31 => E PASS with -3 FAIL ~49 % at E = 10,000, ~5 % at 9,500; E MISS with -3 PASS ~6 % at 10,500 with no lateness over 10.25 ms. Code prints "disagree" for these (:1593-1600), contradicting R-31.
  - NA2 (Low): reorder-step coupling: intake gaps (reader stall up, caller stall down) enter -3's delay for overtaken held datagrams, never E => (h) "never enters ... -3's L2 delays" (u3d-report.md:96-98; advisor-u3d.md:231-233) false for 1 % held; also a reverse-direction mechanism; magnitude tiny.
  - NA3 (Low): floor in excess() lenient by < 1 ns against an inclusive bar.
  - NA4 (Low/Med?): policy read only at the end (header) - a chrt switch mid-leg is undetectable; leg recorded as FIFO.
  - NA5 (C-U8-6): per-socket drops row exists only while socket open; one socket per leg (C-U6-2) => after-read must precede the socket's drop; warm-up drops counted (strict); pre-socket drops (softnet/NIC) not covered; sk_drops counts checksum/filter drops too.
- Next: B5 (dry-run record).
