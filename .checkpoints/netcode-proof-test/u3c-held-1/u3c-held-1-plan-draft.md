# U3C-HELD-1: change-set plan (DRAFT for the advisor's plan review)
Drafted 2026-11-17 by the plan drafter (read only; ran nothing). Branch claude/netcode-proof-test-plan-hcky5g (PR #40),
HEAD 6d24666; the spike is unchanged since 43b02e8. Inputs: netcode-architect-design.md (sha256 ae88c5cd…, "the design
notes"), test-engineer-evidence.md (1c06246c…), drafter-notes.md (D1-D23). Lines at HEAD (delay.rs aaa6a588…).
Main-session edits before the review (08:35 UTC): step 6's commit form and step 8's save, both with the attribution
lines; step 9 reads CI with the GitHub MCP tools (this session uses gh for search only); section 6's free-disk figure
re-measured (the draft's "about 08:40" was no clock reading); one recorded fact added to HELD-Q7.

## 1. Goal
G-NC2, the network layer's held-bytes test, failed once in a counted run though nothing grew past its limit: its check
reads the limit at moments when the limit itself has just moved. Fix the check (test code only) so it still checks the
limit everywhere the held bytes can grow (every write), prove by planted bugs that it still catches a real overflow,
and correct every sentence stating the old form. Merged before another lane's counted check.mjs run (U4f's at latest).

## 2. Why (the finding and its sources)
- The card's U3C-HELD-1, Medium (netcode-proof-test.md:3877-3890); the advisor's merge-stop ruling C2-C3
  (advisor-c62h-bound-merge-stop-full.json: "a timing false positive in a test-only check from U3c").
- C62H-BOUND's first counted call, bites build, binary npt_net-ed8673eeeaf4db12 (.gate/u4r/c62h-bound/merge.log:48-51):
  delay_tests.rs:2300-2304, "held <= 64 KB + the front record's acknowledged prefix", left [1, 0], right [0, 0]
  (c62h-bound-build.md:430-433). It passed in the measured build of that run and in both builds of the second call
  (:447-449); 1 failure against about 96 kept passes (:435). The whole output is gone by design (check.mjs:2655-2660, :2677).

## 3. The cause, as the code shows it (read only; step 1 confirms it by a run)
- The check (Conn::check_held_bound, delay.rs:527-543) runs on every held-bytes sample (held_now :518-519): held =
  bytes[d].len() against 64 KB + the acknowledged prefix of recs[d][released[d]], the front record not yet released.
- Held grows only at writes (down :1574 then model_write :1592; up :806 then :807) and shrinks only at :1748 (S took
  bytes) and :1503 (the application read) (design notes F5). At every write the bound holds by construction: n + 22 <=
  down_free = free_budget - released_model (:473-477) gives held <= 64 KB + prefix (design notes F7; drafter D3).
- The transient: acks free the front record's prefix p; the writer back-fills it (held up to 64 KB + p); the record is
  delivered whole and harvest moves released[DOWN] past it (:684-685), so the prefix term falls to 0 while held falls
  only when the pump hands the record to S (:1748, :1759-1761). A release by the pump's up-loop step (:1814) or the
  test thread's (snapshot, delay_tests.rs:2279; poll_write's current(), delay.rs:1557) after the down loop's last check
  (:1727) is still untaken at the closing sample (:1879), as R1's comment says (:1869-1872). [1, 0] = one such sample.
- The bites build has no feature-gated code on this path (design notes F11): only timing differs (169 tests in one
  binary, in parallel: check.mjs:2057). About 1 run in 100 (test-engineer F6).

## 4. Steps, in order (one command per call, in the simple forms that need no approval: history.md:2187-2190)
Every cargo call goes through main-run.sh (lane lock, idle wait, 4 GiB floor at start, crates touched: :21-53; main
folder only, :13-14, :26). Logs: /home/user/Shamble-Frontlines/.gate/u3c-held-1/. The saver adds only its folder
(checkpoint.mjs:155) but pushes the head (:132): stopped from step 6 to step 8 (as in c62h-bound-build.md:424).

**Step 0.** This plan → the advisor's plan review; it also rules HELD-Q1 to HELD-Q12 (section 9).

**Step 1. The diagnostic loop.** The main session writes u3c-held-1/u3c_held_loop.sh (Write tool): te_m3_loop.sh with
OUT=.gate/u3c-held-1; TEST = G-NC2's full name (l2::delay::tests::under_loss_the_held_bytes_stay_within_64_kb_and_the_
front_records_acknowledged_prefix), or a 6th argument; build `cargo test --locked --workspace --all-features --no-run`
(check.mjs's bites command; never `-p npt-net`, which builds another binary, 301de27: test-engineer F3); BIN from the
line matching `/npt_net-[0-9a-f]{16}` (never `tail -1`: F4), exit 2 unless it is npt_net-ed8673eeeaf4db12; sha256 of
delay.rs, delay_tests.rs and BIN; each run's line adds the test's own `... ok|FAILED` line; a failure prints libtest's
failures list and the `---- <TEST> stdout ----` block. Then the diagnostic patch D-1 goes in by Edit (never committed;
sha256sum before; HELD-Q1):
```
delay.rs Life, after held_beyond_budget: `pub(crate) over_waiting: [u64; 2],` and `pub(crate) over_at_write: [u64; 2],`
delay.rs check_held_bound, after `self.life.held_over_bound[d] += 1;`:
+                if self.released[d] > 0 {
+                    self.life.over_waiting[d] += 1;
+                }
delay.rs model_write, after its `self.logs.steps.push(...)` line: a `#[cfg(test)]` block holding the check's own six
lines that compute held, una and prefix (:531-536, with `dir` for `d`), then:
+            if held > TcpConfig::default().write_budget + prefix {
+                self.life.over_at_write[dir] += 1;
+            }
delay_tests.rs G-NC2's println: the format string gains "; D-1: over-samples with a released record waiting {:?},
at writes {:?}" after "stalls {}", and the arguments `life.over_waiting, life.over_at_write` after the stalls one.
```
- L1: `bash /home/user/Shamble-Frontlines/.checkpoints/netcode-proof-test/main-run.sh /home/user/Shamble-Frontlines/.gate/u3c-held-1/l1.log df -B1 /home/user :: bash /home/user/Shamble-Frontlines/.checkpoints/netcode-proof-test/u3c-held-1/u3c_held_loop.sh l1 all 150 2 0 :: df -B1 /home/user`
  (the whole binary each run, keeping its parallel load; 300 runs, stops at the first failure; about 90 min; 95.5 %
  to see it at 1 in 97, 86.6 % at 1 in 150: test-engineer F5-F6). If none: L2, the same with `l2 one 300 2 6` (the
  test alone, --nocapture, 6 busy loops: how TE-M3 caught its failure, F9).
- D-1 out by Edit; sha256sum equal to the start; `git diff --quiet HEAD -- spikes/netcode-proof` exits 0.
- **Confirmed:** a failing run whose only failing test is G-NC2, at :2300-2304, left [k, 0], k >= 1, its println
  showing over_waiting == held_over_bound and over_at_write == [0, 0], excess under 16,406 B, first losses above 0.
- **Stop for the advisor, whole:** BIN not ed8673; another test fails, or G-NC2 on another line (:2283, :2305); UP
  above 0; over_at_write above 0, or over_waiting below held_over_bound (a real defect: severity up); nothing in L1
  and L2 (unless pre-ruled: HELD-Q11); main-run.sh exit 3.

**Step 2. The fix, by Edit in the main folder (test code and doc text only).** Code, exactly (design notes §2):
```
delay.rs:383 (Life), after held_beyond_budget:
+    pub(crate) held_at_write: [u64; 2],
+    pub(crate) held_grew: [u64; 2],
delay.rs:519 (held_now):
-        self.check_held_bound();
+        self.check_held_falls();
delay.rs:528-543 (the check; its doc :522-526 replaced by the design notes' :201-210):
-    fn check_held_bound(&mut self) {
+    fn check_held_bound(&mut self, d: usize) {
         let budget = TcpConfig::default().write_budget;
-        for d in [DOWN, UP] {
     (the loop's body, :531-541, unchanged, four spaces less indented)
-        }
+        self.life.held_at_write[d] = held;
     }
+
+    /// (its doc: the design notes' :228-230)
+    #[cfg(test)]
+    fn check_held_falls(&mut self) {
+        for d in [DOWN, UP] {
+            if self.bytes[d].len() as u64 > self.life.held_at_write[d] {
+                self.life.held_grew[d] += 1;
+            }
+        }
+    }
delay.rs:612 (model_write's end; its doc :597 as the design notes' :241-243):
+        #[cfg(test)]
+        self.check_held_bound(dir);
delay_tests.rs:779-782 (the stalled-client test, a_full_budget_is_pending_until_it_frees_and_the_held_bytes_…):
-    assert!(matches!(
-        Pin::new(&mut ds).poll_write(&mut noop(), &[0x82; 100]),
-        Poll::Pending
-    ));
+    let r = Pin::new(&mut ds).poll_write(&mut noop(), &[0x82; RECORD_MAX]);
+    let life = ds.life();
+    assert_eq!(
+        life.held_over_bound,
+        [0, 0],
+        "held <= 64 KB + the front record's acknowledged prefix, at every write"
+    );
+    assert_eq!(life.held_grew, [0, 0], "held only falls between writes");
+    assert!(matches!(r, Poll::Pending));
delay_tests.rs:2292-2304 (G-NC2): the println reads "…{} B; writes above the bound {:?}; samples above the latest
write {:?}; first losses {}, stalls {}", with `life.held_grew` after `life.held_over_bound`; the assert message becomes
"held <= 64 KB + the front record's acknowledged prefix, at every write"; after that assert:
+    assert_eq!(life.held_grew, [0, 0], "held only falls between writes");
```
Doc text exactly as the design notes write it: delay.rs:46-51 (notes :149-156), :370-373 (:163-165); delay_tests.rs:
741-744 (:254-256), :2259-2265 (:285-292); SERIES.md:613 and :642, two phrases each, each old phrase matching exactly
twice (:315-322). Then rustfmt clean (its diff applied by Edit). Not looser: the same formula at every growth; "held
only falls between writes" becomes a counted fact (held_grew); the old bite still bites (P2).

**Step 3. The fix's evidence** (each its own main-run.sh call and log; plants by Edit in the main folder: HELD-Q10):
- E3, loops on the fixed tree: `… u3c_held_loop.sh e3a one 300 2 6` (G-NC2, 600 of 600 exit 0); `… e3b one 300 2 6
  l2::delay::tests::a_full_budget_is_pending_until_it_frees_and_the_held_bytes_stay_bounded` (600 of 600); `… e3c all
  10 2 4` (20 of 20).
- E4, plants: each sha256sum, Edit in, `… u3c_held_loop.sh pN all 1 1 0` (whole ed8673 binary; the header's diff hash
  shows the plant), Edit back, sha256sum equal to the fix's. Each must fail on the named line (a compile error also
  exits 101: never the exit code alone); every other failing test is listed (P1 and P5 change admission). Predicted
  from the code; actuals recorded; a difference goes back to review.

| Plant | Edit | Must fail |
|---|---|---|
| P1, the finding's example | down_free: drop `.saturating_sub(self.released_model[DOWN])` (delay.rs:476) | stalled-client test, "…at every write", [1, 0] (64,336 B held + a 16,384 B write = 80,720 > 65,536); G-NC2 predicted PASS, recorded (design notes F15) |
| P2, the old bite | `if held > budget + prefix {` → `if held > budget {` | G-NC2, "…at every write", [k >= 1, 0] (its loop ends only after a write held over 64 KB) |
| P3, growth outside the write | move `c.bytes[DOWN].extend(data);` (:1574) after `c.model_write(DOWN, n, t, false);` (:1592) | stalled-client test, "held only falls between writes", [k >= 8, 0]; G-NC2 too (held_grew, or its hang guard :2283) |
| P4, the at-write call gone | delete model_write's two new lines | stalled-client test, "held only falls…", [k >= 1, k' >= 1] (held_at_write stays 0); G-NC2 at its hang guard after 30 s (drafter D14) |
| P5, up room forgets released bytes | up_room: drop `.saturating_sub(self.released_model[UP])` (:483) | stalled-client test, "…at every write", [0, k >= 1] (the 300-frame flood passes 64 KB: drafter D16; HELD-Q12) |

- E5: `bash …/main-run.sh …/e5.log cargo build --locked --release -p npt-lab :: sha256sum /home/user/Shamble-Frontlines/.gate/shared-target/release/npt-lab`,
  expected 98c3967e04e4260985d24173b1fb2126ea1444636f4725e12cf76333d90b5113 (preflight.log:24): the lab binary is
  unaffected (no source in crates/server, bots or lab names npt_net: D11). That npt-net's non-test code is unchanged
  rests on a diff audit: every changed non-doc line in crates/net sits in a #[cfg(test)] item or statement or in
  delay_tests.rs (the main session and both reviewers read it).
- The gate: `node tools/gate.mjs --no-smoke`, GATE PASS (the lane's form, c62h-bound-build.md:454).

**Step 4. One review round on the final working-tree diff:** netcode-architect and test-engineer at xhigh (netcode),
rust-reviewer if ruled (HELD-Q8). One fix pass; then E3, E4 and the gate again on what changed only.
**Step 5.** The disk check and any packing (section 6). **Step 6.** Stop the saver; commit (`git add` of the named
files, then `git commit -F <message file>`, the message written by Write first and ending with both attribution
lines; separate calls).
**Step 7. The counted whole run (E6):** `bash /home/user/Shamble-Frontlines/.checkpoints/netcode-proof-test/main-run.sh /home/user/Shamble-Frontlines/.gate/u3c-held-1/counted.log df -B1 /home/user :: node check.mjs :: df -B1 /home/user`
Pass bar, every item, written before it runs: the header's HEAD is the fix commit, diff and untracked e3b0c442…;
check.mjs exit 0, "check: all 11 steps pass"; test "measured build: 552 tests passed, 0 failed, 0 reported a skip;
bites build: 632 tests passed, 0 failed, 0 reported a skip" (with T-NEW 553 / 633); refs = `git ls-files | wc -l` at
the fix commit, read just before (1357 at 6d24666); marks 81; self 78 of 78; redact 10 of 10, 0 findings, 305 items
plus one per pack list since (main-54/light.log:8); deps 137 crate files; wasm 27000 ticks, 0 mismatching both ways
(digests never pinned). Pass is the exit code and the runner's summary, never a grep. One failed item: stop for the
advisor, whole; never a second counted call without it. det-u4 and the hash regeneration stay out (D11, E5).
**Step 8.** The gate on the commit; save the card (`node .claude/scripts/checkpoint.mjs save netcode-proof-test -m
"<step>"` with both attribution lines as `--trailer`: it pushes); restart the saver. **Step 9.** CI green on the pushed
head (read with the GitHub MCP tools, actions_list and pull_request_read get_check_runs; never gh, which this session
keeps for search); the advisor's check of the pushed head, naming its full SHA; U3C-HELD-1 then closes on the card.

## 5. Files, symbols and the sentences corrected (design notes F16, §5)
- spikes/netcode-proof/crates/net/src/l2/delay.rs: Life (+held_at_write, +held_grew), held_now's cfg(test) call,
  check_held_bound (one direction, from model_write), new check_held_falls, model_write (one cfg(test) call); docs at
  :46-51, :370-373, :522-526 ("on every held-bytes sample": the cause itself), :597. No non-test statement changes.
- delay_tests.rs: G-NC2 (doc :2259-2265, println :2293, message :2303, the exact sentence the finding names; one
  assert); the stalled-client test (doc sentence, whole-record post-hold write, three asserts).
- marks/SERIES.md:613, :642 (read at the second's end, a between-writes sample; HELD-Q5). Not pinned: marks.json's row
  hashes name docs/adr rows only (check.mjs:1894-1906; D18); the presence tests read names only (design notes F18).
- u3c-plan.md:122-124 ("Down: at most 64 KB plus the acknowledged prefix of the front record."; new text: the design
  notes' :326-327; HELD-Q6). docs/ has none. History stays as it ran (c62h-bound-build.md:432, the card's entry,
  u4r-report.md, the u3c-fix2 records).

## 6. Disk budget and packing
Free 4,988 MiB (`df -B1 /home/user` at 08:34 UTC: 5,230,366,720 B; 4,951 at 07:29, main-54/light.log:2); floor 4,096 at each call's start. Costs
(test-engineer F14, block 3): about 7 bites test builds in steps 1-3 at 16-33 MiB net (up to 230 MiB); logs about 4 MB
per 300 runs; E6 up to 1,022 MiB net. Worst case about 3,680 MiB after E6: every call here starts above the floor, but
U4f's counted run would stop (main-run.sh:44-47). Bar at step 5: 5,220 MiB (4,096 + 1,022 + 100). Below it, and only
if R4's precondition holds (HELD-Q7), pack by brief 8.1 (c62h-bound-brief.md:489-501), one at a time, each citing
advisor-u4r-sets-r2-full.md:45, with its list, commit, pack and free lines: u4r-B1-64f3307d5c7f (821 MiB, about 600
back), then if still short u4r-P3-0f066338dd8d (519). Never either P0 folder (93509dfd7fd1, 051995db9cea),
fc411116c1f1, fd4612050cd6 or the c-u2 folders without Mustafa's word (R4, card :197-198).

## 7. Risks
- The loop may miss a 1-in-100 failure (4.5-13.4 % at 300 runs); without D-1 a catch shows the symptom only.
- The finding's example plant cannot fail G-NC2 (S drains at once: design notes F15, test-engineer F11), so the
  overflow proof rests on the stalled-client test's whole-record write (or T-NEW); P1 would count 0 only if 1,222 B or
  less were free after the hold. Every plant number is a prediction from reading.
- D-1 and the plants are main-folder edits: restores shown by sha256 and git diff, and E6's header (diff e3b0c442…)
  catches a missed one. The disk worst case leaves U4f short, and R4 may block packing (card :202-203).

## 8. Type, tier, size, reviewers, Brave, stops
- Netcode logic, test code only; evidence: deterministic plants, loops under the bites build's loss and jitter, the
  counted run. Risk tier 1 (spike code and tests). Size S: about 130 changed spike lines (+80 -49; about 105 without
  held_grew: design notes §6); SIZE-CHECK: grown well past that, stop and tell Mustafa in three lines (history.md:398).
- Reviewers: netcode-architect and test-engineer; rust-reviewer per HELD-Q8; the advisor on this plan and the pushed
  head. CHECK-NFF-1 stays out (one change-set at a time; its taker is check.mjs's next owner, ruling C4).
- **Mustafa: nothing to check in Brave** (no player-visible change); nothing to merge (the spike's draft PR #40).
- Stops: never a looser bound without its cause; never a re-run to get green; any FAIL, a plant failing otherwise
  than named, one failure in a fixed-tree loop, or exit 3 stops for the advisor, whole, with its log. Never R4's
  folders; never a second counted call without the advisor.

## 9. Open questions (the notes' disagreements and the drafter's additions; none picked silently)
- **HELD-Q1, the loop's bytes.** Design notes §4: HEAD plus D-1, whose counters confirm the cause. Test-engineer (block
  1, F7): HEAD unchanged, the cause by a probe or T-NEW. Both say unchanged code cannot confirm the cause; this draft
  runs HEAD + D-1 because the finding asks the loop to confirm the cause.
- **HELD-Q2, a new test T-NEW** (S never reads, P2 loss). Test-engineer (E2, F13): it shows the cause every run.
  Design notes §4: that needs a forced-loss hook (`forced`, tcp_model.rs:2049, private to the model's tests: D5), so no
  new test. Drafter: with S stalled only about 64 KB plus back-fill is sent, so a well-placed loss is not certain (D6);
  but a prefix also arises with no loss when a record spans two slow-start flights (10 x 1,448 = 14,480 B; the second
  record spans 8,214-16,428 B: D17). Unverified, for netcode-architect. T-NEW adds one test per build (553 / 633).
- **HELD-Q3, held_grew:** keep the between-writes check (the design notes' choice; P3 and P4 need it), or drop it?
- **HELD-Q4, a coverage count** (test-engineer E1, P-c: the write check ran at least `records` times). Drafter (D14):
  held_grew already catches a removed call (P4). P4 alone, or the count too?
- **HELD-Q5, SERIES.md:** may :613 and :642 be reworded here, or are they frozen or another lane's (figures.rs:564)?
- **HELD-Q6, u3c-plan.md:** :122-124 corrected in place with a dated note, or a note below? (Its :125 up sentence lacks
  the up prefix delay.rs:49 states; older, not caused by this fix.)
- **HELD-Q7, packing:** does R4's precondition hold (card :202-203; card :10 still waiting at 07:38; the recorded fact:
  the C62H-BOUND report reached Mustafa in chat at 03:14:03 UTC, the card's NOW block, so R4's "until then" has passed,
  while both P0 folders, fc411116c1f1, fd4612050cd6 and the c-u2 folders still need his word), and does 8.1
  ("Before each set" of C62H-BOUND, c62h-bound-brief.md:489) cover this? If not, E6 runs above the floor and the
  shortfall goes on the card for U4f.
- **HELD-Q8, rust-reviewer:** CLAUDE.md step 5 names it for Rust code; the card names the two reviewers only.
- **HELD-Q9:** the advisor's "npt-lab does not depend on npt-net" is wrong on manifests (crates/lab/Cargo.toml:30,32 →
  server :29, bots :25), right in effect (no source there names npt_net). The design notes (§2) lean on it; the
  test-engineer (F14) corrects it. A Low on the card?
- **HELD-Q10, the plants' build and place.** Design notes §3: `cargo test -p npt-net --lib --all-features` by name, in
  a worktree copy. Test-engineer (F3, E4): that builds another binary (301de27); use the workspace bites build's whole
  ed8673 binary, by Edit in the main folder. This draft takes the latter (main-run.sh runs only in the main folder).
- **HELD-Q11, if L1 and L2 catch nothing:** what carries the cause (the reading plus P2, T-NEW, or a stop for
  Mustafa)? Pre-ruled now, so it costs no extra round.
- **HELD-Q12, plant P5 (up)**, the drafter's addition (the stalled-client test reaches the up bound,
  delay_tests.rs:816-822): keep or drop?
