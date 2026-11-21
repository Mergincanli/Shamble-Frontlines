# U4c review round 2 (netcode-architect), on what changed only. Read only.

Pre-fix: .gate/u4c-prefix (lib.rs 1647e75d, tests.rs c21a362f, tests/l1.rs 639ab9c5 = the build's).
Fixed: .claude/worktrees/wf_f54246ff-e0d-2 (lib.rs d6e6e130, tests.rs e8014c67, tests/l1.rs 9c23f9cc = the fixer's
final hashes, u4c-build.md:278, :327). Diff: 771 lines (scratchpad u4c-r2.diff).

## Plan
1. Read the diff whole: lib.rs (doc line only?), tests.rs, tests/l1.rs.
2. Each round-1 finding the fix claims closed: M1 (=rust M), M2, L1-L4, N1, my F1 doc line. Check closure.
3. Did a fix change the model's behaviour without saying so? lib.rs diff must be docs only. C1' and C2 as ruled
   (advisor-u2.md:72-103, advisor-u3q.md:160-173). Event order (time, fate index).
4. Defects added by the fix (tests that can pass vacuously, wrong numbers, wrong claims in comments).
5. Verdict.

## Progress
- Step 1 done. lib.rs: one hunk (:51-57 -> +3 lines), doc only (the handshake sentence in "What L1 leaves out"); no
  code, no cfg(test) hook, no constant changed: the model's behaviour is unchanged, so C1' (a)-(g) and C2 stand as
  ruled (advisor-u3q.md:160-173, advisor-u2.md:79-83); event rank (lib.rs:505-509) and advance/apply (:947-991)
  untouched. tests.rs: Rig.at, drive (:854), woken (:889), Lc5 struct, L-c5 event mode on drive, host_at_next_event
  (:1084), packet_threshold (:1193), time_threshold (:1248), lc6 links 2/3 on fixed(0), fails(r, lc, check).
  tests/l1.rs: burst comment reworded + "no stream resend in [2.2 s, 3 s)" (:400-419), new
  acknowledgements_are_lost_only_inside_bursts (:543). Pins (v1s_tag test) untouched.
- Step 2 (closure):
  - M1/rust M CLOSED: drive calls only at next_event_ns and script send instants (every script instant sends:
    link 1 ticks at m%33==1); woken checks each hand-out == due (arrival, or send+1 for P0) and every log arrival
    <= horizon, PTO and declared instant is a call instant (trace at_ns = due time, lib.rs:955, so a missed instant
    shows); host test covers quiet PTO (2.126 s), loss timer (3.1125 s exact), P0 gate+1 (10 ms + 1 ns), held
    (80 ms) and overtaken (500 ms -> 560 ms + 1 ns, lib.rs:1134-1146) and a post-stop poll that must find nothing.
    Fixer's six plants each fail it.
  - M2 CLOSED: 5 one-piece messages 1 ms apart on a fixed 100 ms link; first sample at 1.101 s sets srtt 100 ->
    loss delay 112.5 ms; ack of p+3 at 1.103 s declares p by count (RFC A.10 pn <= largest - 3). 1M -> 1112.5 ms,
    2 -> 1102 ms: both fail. Consistent with C1' (b).
  - L1 CLOSED: trial = lost message + datagram same instant; one sample r; want = 9/8 max(srtt after r, r) (A.7 then
    A.10 order); needs ups and downs (so latest-only fails too). srtt-only plant fails.
  - L2 CLOSED (positive counts guard it: n.0>50, n.1>50, n.2>3000; inside = [1 s, 2 s) = in_burst lib.rs:222-226).
  - L3 CLOSED: fixed(0) links, both arrive 70 ms, applied in one call at 1 s; i3 < i2 pins (time, fate index) over
    link order; link-first plant fails.
  - L4 CLOSED: comment no longer claims the packet threshold; PTO claim asserted; parse guarded by the same test's
    resent > 1,000 (tests/l1.rs:430-442, same 64-byte/resends parse) and layout unit test (src/tests.rs:304-330).
  - N1 CLOSED: fails requires "L-cN: " prefix + check words; fixer's two plants show it.
  - My F1: doc half CLOSED (lib.rs:54-57; RFC 9002 §6.2.1 cite right). The ruling half (Join's 2 s rule vs
    1,024 then 3,072 ms on L1) stays a follow-up for the main session, not reopened.
  - Not claimed, stay follow-ups: F2 (lab-side L-c8 source), F3, F4 (lib.rs:13 still says "never moves another's
    fates"), F5, F6, F7; rust Lows; test O1, O2.
- Step 3-4 (new defects): drive sends then polls (old L-c5 polled then sent): documented (:851-853), harmless for
  the clean model (enter -> advance first, lib.rs:925-937) and makes L-c5 stronger. One new Low: time_threshold doc
  (src/tests.rs:1241-1242) says "±20 ms of jitter (50 ± 10 ms each way)", but jitter_ns 40 ms (:1256) is ±40 ms round
  trip by Profile's own doc (lib.rs:148-149, :155) and delay() (lib.rs:793, :813): 50 ± 20 ms each way, samples
  60-140 ms. Logic exact and asserted (no PTO, ups and downs), so a comment defect only.
- Verdict: APPROVE WITH FINDINGS (one new Low). DONE
