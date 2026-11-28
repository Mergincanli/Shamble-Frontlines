# U3C-HELD-1: netcode-architect's design notes (cause and fix)

Agent: netcode-architect. READ ONLY (no builds, tests, cargo). Writes only this file (Write tool; no Edit tool here,
so the file is rewritten whole each time). Branch: claude/netcode-proof-test-plan-hcky5g (PR #40); HEAD read at
25b262f (`git log --oneline -1`). All delay.rs / delay_tests.rs line numbers are at that HEAD.

## Plan
1. Read the card (netcode-proof-test.md:3870-3885, now :3879-3890), the kept evidence (merge.log:51-52,
   c62h-bound-build.md:426-455, advisor-c62h-bound-merge-stop-full.json C3-C4).
2. Read delay.rs: the failing test and every function it calls; grep held, down_free, released_model, 64 KB bound.
3. Explain the transient (order of writes/releases, which counter moves first, when the test samples, what
   held_over_bound [1, 0] means); the bites build's feature-gated code and why only once.
4. If the code does not support the release-transient story, say so and give the cause it does support.
5. Fix design: the check, where evaluated (every write), symbols; diff as text.
6. Planted bug: exact plant, place, expected failure line.
7. NC-U3c-2 bound sentence: every place, file:line, which become wrong.
8. Size estimate with basis.
Status: all 8 done (below). Nothing run; every number below that needs a run is marked "predicted".

## Findings (one line per finding, appended as found)
- F1 (evidence): the failure is held_over_bound == [1, 0] at delay_tests.rs:2300-2304 (c62h-bound-build.md:430-433:
  "left [1, 0], right [0, 0]"); name at merge.log:51; the card's "delay.rs:528-543" is check_held_bound (delay.rs:528).
- F2 (index): Life.held_over_bound is [u64; 2] indexed by direction, DOWN = 0, UP = 1 (delay.rs:121-122, :382, :538).
  [1, 0] = exactly one call of check_held_bound found down held > 64 KB + prefix; up never (this test sends nothing up).
- F3 (where the check runs today): check_held_bound is called only from held_now (delay.rs:518-519); held_now runs at
  poll_read (:1522), poll_write after its write (:1594), the pump's closing block (:1879), and the per-second and
  run rows (:1417, :1445; not called by this test).
- F4 (the accounting): free_budget(DOWN) = 65,536 - (write_seq - snd_una) = 65,536 - outstanding (tcp_model.rs:2211-
  2214, :91); down_free = free_budget - released_model[DOWN] (delay.rs:473-477); released_model += len+22 at release
  (harvest, :685) and -= len+22 only when S has taken the whole record (:1759-1761). A released record's unacked part
  is therefore in both outstanding and released_model (counted twice) until its ack: down_free is conservative.
- F5 (held only grows at writes): bytes[DOWN] grows only at poll_write (:1574, then model_write :1592); bytes[UP] only
  at move_up (:806, then model_write :807); bytes shrink only at :1748 (S took) and :1503 (tungstenite read). Grep
  `bytes\[...\]\.(extend|drain...)` over delay.rs: those four sites only.
- F6 (identity, model bytes): with W = model_end[DOWN], F = start of the oldest held record, R = end of the last
  released record, H = W - F = outstanding + released_model + (snd_una - R) at every instant (signed). Acks leave H
  unchanged (outstanding down, snd_una up), releases leave it unchanged (R and released_model up by the same size),
  takes lower it. So H only falls between writes.
- F7 (the bound holds at every write): after a write n+22 <= down_free, so outstanding + released_model <= 65,536.
  If snd_una <= R: H <= 65,536. If snd_una > R: snd_una - R is exactly check_held_bound's prefix (front unreleased
  record's acked part, < its size, since a record delivered whole is released by the step to now that precedes every
  write: poll_write's current() :1557, move_up after step_to :1814-1815), so H <= 65,536 + prefix. Plaintext held =
  H - 22 x records - front_done, so held <= 64 KB + prefix at every write, with 22 B per record to spare.
- F8 (the transient): the bound's prefix term drops at the release of the front record (prefix p -> the new front's,
  0 since its start R_new > snd_una), while held is unchanged until S takes the record (:1748). If held > 65,536 at
  that release (it is, whenever the writer back-filled p: held ~ 65,536 + p - 22N), every sample between that release
  and the take counts as over the bound. Nothing grew: H unchanged (F6).
- F9 (when the test samples it): the take happens in the pump's down loop (:1721-1784), which runs only after the
  pass's opening step (:1715). A release by the pass's own up-loop step (By::Mid, :1813-1814) or by another task's
  step while the lock is free (the test loop's ds.snapshot() -> current(), delay_tests.rs:2279, and write_all's
  poll_write -> current(), delay.rs:1557; the test body runs on the libtest thread via block_on, the pump on a tokio
  worker: wrap spawns it, delay.rs:944) after the down loop's last check (`released[DOWN] == 0`, :1727-1728) leaves
  the record released and untaken at the closing block's held_now (:1879). The code's own R1 comment says so
  (:1869-1872: "is still unwritten here"). That is the one counted sample.
- F10 (poll_write's own sample never trips): poll_write samples only after a write (:1594), when F7 holds; a
  poll_write whose current() released the prefixed record either returns Pending (free <= 22, :1562-1568, no sample)
  or writes within outstanding + released_model <= 65,536 (F7).
- F11 (bites build: no code path change): `git grep cfg(feature` over crates/net finds only l2.rs:31 (the wt module)
  and wt.rs/wt_tests.rs; delay.rs, delay_tests.rs, tcp_model.rs have none. npt-l1's `bites` (unified on by
  `cargo test --workspace --all-features`, check.mjs:2057) changes only L1Net's private Model/Bite
  (l1/src/lib.rs:760-819), never Profile, the one thing npt-net takes from npt-l1 (net/Cargo.toml comment). So the
  bites build runs the same DelayStream and test code; it can change only timing: its npt-net lib binary has 9 more
  tests (wt_tests.rs:249, :271, :288, :434, :455, :473, :509, :544, :574; QUIC servers on multi-thread runtimes, one
  with 4 workers, :508), run by libtest in parallel threads (no --test-threads at check.mjs:2057). 160 + 9 = 169, as
  the advisor counted. Whether they overlap this test in time is not shown (libtest starts tests in name order).
- F12 (why once): an over-sample needs (i) a back-filled acked prefix (a first-transmission loss inside a record, not
  in its first segment; P2 = 2 % per transmission, l1/src/lib.rs:186-188; 16,406 B / 1,448 B MSS (tcp_model.rs:87)
  = 12 segments, so about 1 - 0.98^12 = 21 % of records), (ii) its release by a non-opening step inside the R1 window
  (normally the clock fires at the delivery event, the opening step releases it, the down loop takes it at once),
  (iii) held still above 65,536 B then. A few episodes per run times a microsecond window, wider when a worker is
  preempted: about 1 in 100. One failure in about 97 logs cannot separate the builds (the measured build passed in
  the same run, c62h-bound-build.md:434).
- F13 (the code supports the release-transient cause; no other cause found): the only way the bound side falls is a
  release (prefix grows with acks, drops to the next front's at release); held never grows outside a write (F5);
  at every write held <= the bound (F7). The second assertion (excess < 16,406 B, delay_tests.rs:2305) passed in the
  failing run (the advisor read 14,326 B from the deleted test.log; no longer verifiable on disk).
- F14 (what any real overflow needs): by F6, H > 64 KB + prefix at a write needs released-but-untaken bytes that are
  already acknowledged, with down_free not subtracting them. The model itself caps outstanding at 64 KB
  (tcp_model.rs:2217-2224; model_write's debug_assert, delay.rs:601). So the check can only ever catch an accounting
  fault in released_model, and only in a scenario where S leaves released records untaken past their ack.
- F15 (the finding's example plant does not reach G-NC2's scenario): G-NC2's S is a 1 MiB duplex with a draining
  reader (delay_tests.rs:2268-2274): the pump takes every released record within the pass that released it (or the
  next, R1), while its ack comes a one-way delay later (P2: 50 ms +/- 10). So with `down_free` = free_budget only,
  G-NC2 still passes (prediction; to be shown by the plant run). The stalled-client test (a_full_budget_..., delay_
  tests.rs:741-839: duplex(1_024), the client never reads, quiet P2, 1.5 s hold) is where released records wait
  acknowledged and untaken (the model does not model the receive window, tcp_model.rs:62-65, so it acks them); its
  post-hold write is only 100 B (:780), too small to cross 64 KB (64,336 + 100).
- F16 (bound sentences): delay.rs:46-51 (module doc), :370-373 (Life doc), :522-526 (check doc); delay_tests.rs:2259-
  2265 (G-NC2 doc), :2293-2294 (println "samples above the bound"), :2303 (assert message); SERIES.md:613 and :642
  (ws_down_held_bytes, ws_up_held_bytes rows, read "at the second's end"); u3c-plan.md:122-124 (checkpoint plan).
  docs/: none (git grep). Historical records quoting the old message (c62h-bound-build.md:432, the card, u4r-report,
  c62h-bound-report-draft, u3c-fix2* files) record what ran then: unchanged.
- F17 (no other test depends on the counters): held_over_bound / held_beyond_budget are read only by G-NC2
  (delay_tests.rs:2278, :2295-2305); Life is built only by Life::default (delay.rs:1050); Life is inline, not heap,
  so ws_conn_fixed_bytes (309,583, delay_tests.rs:2700; delay.rs:1054) is unchanged by new Life fields. No check.mjs,
  check.test.mjs, marks.json or ops/ text pins the held wording (git grep -i held).
- F18 (the SERIES.md presence tests read names only): names.rs:143, delay_names.rs:449, socket_tests.rs:390
  include_str! SERIES.md and assert "no SERIES.md row for ..." by name, so rewording the two rows passes them.

## 1. The cause, in order (answers task items 1 and 2)
The order of moves for one down record r (size s = len + 22) that suffers a loss inside it:
1. Write (poll_write, delay.rs:1557-1594): current() steps the model to now; free = down_free; bytes[DOWN] grows
   (:1574); model_write: model.write (outstanding += s), model_end, written, recs (:598-613); held_now samples.
2. Acks of r's first segments: snd_una passes into r, outstanding falls; check_held_bound's prefix for r rises by
   the same amount, and so does down_free; the writer (woken by wakers(), :845) back-fills it: held reaches up to
   64 KB + p - 22 x records (the excess the test needs, its loop's `beyond > 0`, delay_tests.rs:2278-2281).
3. r's retransmission is delivered: harvest (:675-686) moves released[DOWN] past r and adds s to released_model.
   This is the first counter to move: the check's "front record not yet released" is now r's successor, whose prefix
   is 0 (its start is above snd_una), so the bound falls by p. held (bytes[DOWN]) has not moved.
4. The pump's down loop takes r into S: bytes[DOWN] drains (:1748), then released and released_model fall
   (:1759-1761). Only here does held fall.
5. Later, r's ack arrives: outstanding falls by s - p.
Between 3 and 4 the check reads held > 64 KB + 0 although H, the real holding, did not change (F6) and every write so
far met the bound (F7). The test samples exactly there when step 3 is done by the pump's up-loop step (:1814) or by
the test thread's current() (snapshot, delay_tests.rs:2279; write_all -> poll_write, delay.rs:1557) after the down
loop's last check (:1727) and before the closing block's held_now (:1879): R1's own documented state (:1869-1872).
`held_over_bound [1, 0]` in the test's terms: one held-bytes sample, down, above "64 KB + the front record's
acknowledged prefix"; none up (F2). The bites build cannot change the code path (F11), only the load and so the window
(F11, F12). The code supports the release-transient cause; I found no other (F13). Not shown by a run.

## 2. The fix (task item 3): check the bound at every write; check the falls between writes
What the check must compute: the same bound as today, held <= 64 KB + the acknowledged prefix of the front
unreleased record, but evaluated at each write of a record into the model (model_write, delay.rs:598, both
directions), immediately after the held bytes grew; that is every place held can grow (F5) and the only instants at
which the bound is an invariant (F7). Between writes, a second check (recommended, optional) replaces the bound with
what is true there: held never exceeds what the latest write left (F6: held only falls between writes). Together they
say: every sample is within the bound proven at the latest write. No bound is loosened: the formula is unchanged; it
is no longer applied at instants where its "front record" has changed under it.

Test code only: every new item is #[cfg(test)]; the non-test functions get only a #[cfg(test)] statement (model_write,
held_now, as held_now has today) and doc text. The measured and bites non-test builds, npt-lab, det-u4 and the hash
are unaffected (npt-lab does not depend on npt-net: the advisor's merge-stop ruling).

Symbols: Life gains `held_at_write: [u64; 2]` and `held_grew: [u64; 2]`; Conn::check_held_bound(&mut self) becomes
Conn::check_held_bound(&mut self, d: usize), called from model_write; new Conn::check_held_falls(&mut self), called
from held_now; tests: G-NC2 (delay_tests.rs:2259-2308) and a_full_budget_is_pending_until_it_frees_and_the_held_bytes_
stay_bounded (delay_tests.rs:741-839). Variant (a)-only, if the advisor drops the between-writes check: drop
held_at_write, held_grew, check_held_falls, the held_now change (delete its cfg(test) call) and the two held_grew
asserts.

### Proposed diff (text, not applied; rustfmt default width 100, doc lines at the file's 120)
--- delay.rs:46-51 (module doc)
-//! **Held bytes (NC-U3c-2).** Down at most the 64 KB budget (it covers released-but-untaken bytes) plus the
-//! acknowledged prefix of the front record not yet released: a loss inside a record lets the acknowledgement of its
-//! first segments free that prefix in the budget while the record waits to be delivered whole, so below 64 KB plus one
-//! record (16,406 B). Up likewise at most 64 KB plus the acknowledged prefix of its front unreleased record (below one
-//! record: 549 B, or the request's 8,214 B), plus the scanner's buffer (one frame, 527 B, or the request cap). Counted,
-//! current and largest, per direction.
+//! **Held bytes (NC-U3c-2, U3C-HELD-1).** Down, at each write, at most the 64 KB budget (it covers released-but-
+//! untaken bytes) plus the acknowledged prefix of the front record then not yet released: a loss inside a record lets
+//! the acknowledgement of its first segments free that prefix in the budget while the record waits to be delivered
+//! whole. Between writes the held bytes only fall, so at every moment below 64 KB plus one record (16,406 B); the bound
+//! with the current front record is no invariant there, since a release ends its prefix while its bytes wait for S.
+//! Up likewise, at each write, at most 64 KB plus the acknowledged prefix of its front unreleased record (below one
+//! record: 549 B, or the request's 8,214 B), plus the scanner's buffer (one frame, 527 B, or the request cap). Counted,
+//! current and largest, per direction.

--- delay.rs:370-384 (Life)
 /// The pump's lifetime for the shutdown tests (W3, M1): its polls, those at or after the shutdown deadline, the
 /// deadline and the stamp it ended at (DelayStream's clock). And the tests' counts: move_up's compactions that shifted
-/// records still pending (G-F10's vacuity), and per direction the held-bytes samples above NC-U3c-2's bound and the
-/// largest excess of the records held over the 64 KB budget (G-NC2; `Conn::check_held_bound`).
+/// records still pending (G-F10's vacuity), and per direction (G-NC2, U3C-HELD-1): the writes that left the held
+/// bytes above NC-U3c-2's bound, the largest excess of the records held over the 64 KB budget, the held bytes the
+/// latest write left, and the samples above those (`Conn::check_held_bound`, `Conn::check_held_falls`).
 ...
     pub(crate) held_over_bound: [u64; 2],
     pub(crate) held_beyond_budget: [u64; 2],
+    pub(crate) held_at_write: [u64; 2],
+    pub(crate) held_grew: [u64; 2],
 }

--- delay.rs:518-543 (held_now's call; the check)
         #[cfg(test)]
-        self.check_held_bound();
+        self.check_held_falls();
     }
 
-    /// NC-U3c-2's bound on every held-bytes sample (the tests; G-NC2): per direction the plaintext held in records (down:
-    /// all DelayStream holds; up: less the scanner's buffer) is at most the 64 KB budget plus the acknowledged prefix of
-    /// the front record not yet released (a loss inside it lets the acknowledgement of its first segments free that
-    /// prefix in the budget while it waits to be delivered whole). Counts the samples above it and keeps the largest
-    /// excess over the budget.
-    #[cfg(test)]
-    fn check_held_bound(&mut self) {
-        let budget = TcpConfig::default().write_budget;
-        for d in [DOWN, UP] {
-            let held = self.bytes[d].len() as u64;
-            let una = self.model_end[d] - self.model.outstanding(d);
-            let prefix = self.recs[d].get(self.released[d]).map_or(0, |r| {
-                let size = r.len as u64 + OVERHEAD;
-                una.saturating_sub(r.model_end - size).min(size)
-            });
-            if held > budget + prefix {
-                self.life.held_over_bound[d] += 1;
-            }
-            self.life.held_beyond_budget[d] =
-                self.life.held_beyond_budget[d].max(held.saturating_sub(budget));
-        }
-    }
+    /// NC-U3c-2's bound where the held records grow (the tests; G-NC2, U3C-HELD-1): at each write of a record into
+    /// `d`'s end (model_write, the only place they grow), the plaintext held in records (down: all DelayStream holds;
+    /// up: less the scanner's buffer) is at most the 64 KB budget plus the acknowledged prefix of the front record not
+    /// yet released (a loss inside it lets the acknowledgement of its first segments free that prefix in the budget
+    /// while it waits to be delivered whole). The write's room makes it so: the step to the write's now released every
+    /// record delivered whole, and down_free and up_room count the released-but-untaken bytes. Between writes the bound
+    /// is no invariant: the front record's release ends its prefix while its bytes stay held until S takes them (down)
+    /// or tungstenite reads them (up), so a sample then can read above it with nothing grown (the release transient);
+    /// `check_held_falls` covers those samples. Counts the writes above the bound and keeps the largest excess over the
+    /// budget and the held bytes this write left.
+    #[cfg(test)]
+    fn check_held_bound(&mut self, d: usize) {
+        let budget = TcpConfig::default().write_budget;
+        let held = self.bytes[d].len() as u64;
+        let una = self.model_end[d] - self.model.outstanding(d);
+        let prefix = self.recs[d].get(self.released[d]).map_or(0, |r| {
+            let size = r.len as u64 + OVERHEAD;
+            una.saturating_sub(r.model_end - size).min(size)
+        });
+        if held > budget + prefix {
+            self.life.held_over_bound[d] += 1;
+        }
+        self.life.held_beyond_budget[d] =
+            self.life.held_beyond_budget[d].max(held.saturating_sub(budget));
+        self.life.held_at_write[d] = held;
+    }
+
+    /// G-NC2 between writes (U3C-HELD-1): the records' plaintext only falls there (S takes it down, tungstenite reads
+    /// it up), so every held-bytes sample is at most what the latest write left, which `check_held_bound` held to the
+    /// bound. Counts the samples above it: a growth outside model_write.
+    #[cfg(test)]
+    fn check_held_falls(&mut self) {
+        for d in [DOWN, UP] {
+            if self.bytes[d].len() as u64 > self.life.held_at_write[d] {
+                self.life.held_grew[d] += 1;
+            }
+        }
+    }

--- delay.rs:596-613 (model_write)
     /// One record of `len` plaintext bytes into the model at `t` from `dir`'s end. The caller checked the room, so the
-    /// model takes all of it.
+    /// model takes all of it, and put the record's plaintext into `bytes[dir]` first (G-NC2's check at every write).
 ...
         #[cfg(test)]
         self.logs.steps.push((t, Step::Write { end: dir, bytes }));
+        #[cfg(test)]
+        self.check_held_bound(dir);
     }

--- delay_tests.rs:741-744 (a_full_budget_... doc; one sentence added at its end)
-/// 150 KB of frames); and the pump does not spin while S owes it a wake (R1's guard: its runs over the hold, counted).
-/// Quiet P2, about 2 s.
+/// 150 KB of frames); and the pump does not spin while S owes it a wake (R1's guard: its runs over the hold, counted).
+/// After the hold a whole record is offered: with the released records acknowledged and untaken, only down_free's
+/// released bytes keep it out (G-NC2's check at every write, U3C-HELD-1). Quiet P2, about 2 s.

--- delay_tests.rs:776-782
     let polls = ds.life().polls;
     tokio::time::sleep(Duration::from_millis(1_500)).await;
     let polls = ds.life().polls - polls;
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
(owed becomes 16,384 instead of 100 after that Pending; nothing in this test reads owed, :745-839.)

--- delay_tests.rs:2259-2265 (G-NC2 doc)
-/// G-NC2 (NC-U3c-2's corrected bound, under loss): the down plaintext DelayStream holds never exceeds the 64 KB budget
-/// plus the acknowledged prefix of the front record not yet released, checked on every held-bytes sample
-/// (`Conn::check_held_bound`), and that prefix does carry it beyond 64 KB. P2 with its loss and jitter; the writer keeps
-/// the budget full with 16 KB records while the client drains S. It runs until 100 records are written, a first
-/// transmission was lost, and a sample held more than 64 KB (model state; a hang guard only). Then: no sample above the
-/// bound either way, the largest excess over 64 KB is below one record (16,406 B), and first transmissions were lost.
-/// A few seconds.
+/// G-NC2 (NC-U3c-2's corrected bound, under loss; U3C-HELD-1): at every write, where the held bytes grow
+/// (`Conn::check_held_bound`), the down plaintext DelayStream holds is at most the 64 KB budget plus the acknowledged
+/// prefix of the front record not yet released, and that prefix does carry it beyond 64 KB; between writes it only
+/// falls (`Conn::check_held_falls`, every held-bytes sample), since there a release can end the prefix while the bytes
+/// wait for S. P2 with its loss and jitter; the writer keeps the budget full with 16 KB records while the client
+/// drains S. It runs until 100 records are written, a first transmission was lost, and a write held more than 64 KB
+/// (model state; a hang guard only). Then: no write above the bound and no sample above the latest write either way,
+/// the largest excess over 64 KB is below one record (16,406 B), and first transmissions were lost. A few seconds.

--- delay_tests.rs:2292-2304
     println!(
-        "{records} writes of 16 KB at P2: largest down excess over 64 KB {} B; samples above the bound {:?}; first \
-         losses {}, stalls {}",
+        "{records} writes of 16 KB at P2: largest down excess over 64 KB {} B; writes above the bound {:?}; samples \
+         above the latest write {:?}; first losses {}, stalls {}",
         life.held_beyond_budget[DOWN],
         life.held_over_bound,
+        life.held_grew,
         s.counters[DOWN].first_lost,
         s.counters[DOWN].stalls
     );
     assert_eq!(
         life.held_over_bound,
         [0, 0],
-        "held <= 64 KB + the front record's acknowledged prefix"
+        "held <= 64 KB + the front record's acknowledged prefix, at every write"
     );
+    assert_eq!(life.held_grew, [0, 0], "held only falls between writes");
     assert!(life.held_beyond_budget[DOWN] < RECORD_MAX as u64 + OVERHEAD);

--- marks/SERIES.md:613 and :642 (the same two phrases in each row; replace, each must match exactly twice)
- "at most the 64 KB budget plus the acknowledged prefix of the front record not yet released, so below 64 KB + 16,406 B"
+ "at most, at each write, the 64 KB budget plus the acknowledged prefix of the front record then not yet released,
+  and only falling between writes, so below 64 KB + 16,406 B"
- "plus the scanner's buffer: at most 64 KB plus the acknowledged prefix of the front unreleased record"
+ "plus the scanner's buffer: at most, at each write, 64 KB plus the acknowledged prefix of the then front unreleased
+  record"
(one table line each; no line break inside a row.)

--- checkpoint, not the spike: u3c-plan.md:122-124
- "Down: at most 64 KB plus the acknowledged prefix of the front record."
+ "Down: at each write at most 64 KB plus the acknowledged prefix of the front record then unreleased; between writes
+  the held bytes only fall (corrected 2026-11-17, U3C-HELD-1)."
Aside (pre-existing, not made wrong by this fix): u3c-plan.md:125's "Up: at most 64 KB in all, plus one frame in
parsing (513 + 14 B) and the request cap" lacks the up prefix that delay.rs:49 and SERIES.md state.

### Why the fixed check is not looser (the argument the reviewers should check)
- Same formula, applied at every growth (F5: the only two extend sites are each followed, under the same lock, by
  model_write, where the new call sits; no held_now runs between them).
- The between-writes check makes F6's premise (held only falls) a counted fact on every sample, in every test that
  drives DelayStream, so "checked only at writes" is not taken on trust.
- The old bite (bites.py:52: bound without the prefix) still bites (plant P2 below), so the prefix term stays reached.

## 3. Planted bugs (task item 4): what each must do with the fix applied (predicted; nothing run)
Each plant: one textual replacement in a worktree copy (the u3c-fix2-c1b-finish/bites.py pattern), the named test run
by name in the bites build (`cargo test --locked -p npt-net --lib --all-features -- <name> --exact`, through
main-run.sh by the main session), FAILED expected, then restored. (file, old, new):
- P1, the finding's example (down_free forgets released bytes): delay.rs:474-476
  old "            .free_budget(DOWN)\n            .saturating_sub(self.released_model[DOWN])"
  new "            .free_budget(DOWN)"   (unique: up_room's text is .free_budget(UP)).
  (a) a_full_budget_is_pending_until_it_frees_and_the_held_bytes_stay_bounded must FAIL at the new assert:
      assertion `left == right` failed: held <= 64 KB + the front record's acknowledged prefix, at every write
        left: [1, 0]
       right: [0, 0]
      Why exactly 1: phase 1 writes 7 x 8,192 + 8,016 = 65,360 B in 8 records before any release (the comment at
      delay_tests.rs:804-806); S takes 1,024 B (duplex(1_024)), so 64,336 B held; after the 1.5 s hold every record is
      delivered and acknowledged (quiet P2; no receive window modelled, tcp_model.rs:62-65), so the planted room is the
      whole 64 KB and the post-hold write takes 16,384 B: 80,720 B > 65,536 + 0 (the new record is the front unreleased
      one, prefix 0). That is the test's only write after phase 1, and held_grew stays [0, 0] (held only falls after).
  (b) G-NC2 under P1: predicted PASS (F15). Run it too: it records that G-NC2's own scenario cannot show this fault,
      which is why the plant lives in the stalled-client test.
- P2, the old G-NC2 bite (bound without the prefix): delay.rs:537 old "if held > budget + prefix {" new
  "if held > budget {". G-NC2 must FAIL: "held <= 64 KB + the front record's acknowledged prefix, at every write",
  left [k, 0], k >= 1: the loop only ends after a write left more than 64 KB (beyond > 0 is now set at writes), so k
  is certain to be at least 1 (the old bite counted samples: [366, 0], bites-run.txt:56-61).
- P3, a growth outside the checked write (only with the between-writes check): delay.rs poll_write: delete
  "        c.bytes[DOWN].extend(data);\n" (:1574) and insert it after "        c.model_write(DOWN, n, t, false);\n"
  (:1592). a_full_budget_... must FAIL: "held only falls between writes", left [k, 0], k >= 8 (each of the 8 phase-1
  writes' own sample at :1594, then the pump's closing-block samples through the hold). held_over_bound stays [0, 0]
  there (the check at the write sees the smaller held), so the first failing assert is the held_grew one.
Pass bar for the fix: on the fixed tree, both tests pass in both builds; P1 (a), P2 and P3 fail as above; P1 (b) is
recorded whatever it gives (a FAIL would mean released records wait acknowledged in G-NC2 after all; then it is
evidence, not a defect).

## 4. For the loop (the first step; the test-engineer owns it): what the cause predicts
A failing iteration on HEAD prints only [1, 0] and the println (delay_tests.rs:2292-2299). To make the loop confirm
the cause rather than the rate, run it on HEAD plus a diagnostic patch D-1 (cfg(test), never committed): keep today's
every-sample check and its assert; add Life counters `over_waiting: [u64; 2]` (over-samples with released[d] > 0) and
`over_at_write: [u64; 2]` (the same bound computed in model_write), printed in G-NC2's println before the assert.
Predicted in every failing iteration: held_over_bound == over_waiting == [k, 0] and over_at_write == [0, 0].
Refuted by: any over_at_write > 0 (a real overflow at a write) or held_over_bound > over_waiting (an over-sample with
no released record waiting). Its rate in the loop should match about 1 in 100 only if the loop keeps the bites
build's concurrency (the whole npt-net lib binary per iteration, not this test alone; F11). If the loop's budget
ends without a failure, the cause stays shown by reading only (F6-F9); a deterministic reproduction then needs a
cfg(test) forced-loss hook from delay_tests into TcpModel (`forced`, tcp_model.rs:2049, is private to the model's
own tests) plus a stalled-client test that back-fills a prefix and waits for the record's release: not proposed now.

## 5. NC-U3c-2's bound sentences (task item 5): where, and which the fix makes wrong
Made wrong (they state the bound at an arbitrary instant or "on every sample"):
- delay.rs:46-51 module doc ("Down at most the 64 KB budget ... plus the acknowledged prefix of the front record not
  yet released"; "Up likewise at most 64 KB plus ... its front unreleased record"): wrong between writes.
- delay.rs:372-373 Life doc ("the held-bytes samples above NC-U3c-2's bound"): the counter now counts writes.
- delay.rs:522-526 check doc ("NC-U3c-2's bound on every held-bytes sample"): wrong; it is the cause of U3C-HELD-1.
- delay_tests.rs:2259-2265 G-NC2 doc ("checked on every held-bytes sample", "a sample held more than 64 KB", "no sample
  above the bound either way").
- delay_tests.rs:2293 println ("samples above the bound") and :2303 assert message ("held <= 64 KB + the front
  record's acknowledged prefix"): the message is the exact sentence the finding names.
- marks/SERIES.md:613 (`second.v.ws_down_held_bytes`) and :642 (`second.v.ws_up_held_bytes`): "bytes DelayStream holds
  at the second's end (down: ... at most the 64 KB budget plus the acknowledged prefix of the front record not yet
  released, so below 64 KB + 16,406 B ...; up: ... at most 64 KB plus the acknowledged prefix of the front unreleased
  record ...)". A second's-end reading is a between-writes sample, so "at most ... front record not yet released" is
  wrong there; "so below 64 KB + 16,406 B" stays right. Same text in both rows.
- .checkpoints/netcode-proof-test/u3c-plan.md:122-124 ("Down: at most 64 KB plus the acknowledged prefix of the front
  record."): the approved plan's statement of NC-U3c-2.
Still right: delay.rs:13-17 (records, free = budget less released-and-untaken) and :57 ("below 81,942 B each way").
Unchanged on purpose (history: what ran then): c62h-bound-build.md:432, the card's U3C-HELD-1 entry, u4r-report.md:
1419, c62h-bound-report-draft.md:219, u3c-fix2b-result.txt:31, u3c-fix2-c1b-finish.md:40 and :97, u3c-fix2-c1b.md:42,
the bites-run files and final.patch. docs/: no occurrence.

## 6. Size (task item 6)
Counted from the diff text above (+ and - lines, the SIZE-CHECK convention):
- delay.rs: module doc -6 +8; Life doc -2 +3, fields +2; held_now -1 +1; check_held_bound -21 +25 (doc 5 -> 10, body
  re-indented out of its loop, +1 line); check_held_falls +11; model_write doc -1 +1, call +2. About -31 +53.
- delay_tests.rs: full-budget doc -2 +3, body -4 +9; G-NC2 doc -7 +8, println -2 +3, assert message -1 +1, held_grew
  assert +1. About -16 +25.
- SERIES.md: -2 +2 (two long rows).
Spike total about 130 changed lines (+80 -49), all in crates/net/src/l2/ and marks/SERIES.md. Without the
between-writes check (variant (a)): about 105. Plus u3c-plan.md (checkpoint, not counted): -1 +2. Plant scripts and
D-1 live in the notes folder, not counted.

## Open questions (for the plan's drafter and the advisor)
- Q1: keep the between-writes check (check_held_falls, held_grew; recommended: it makes F6 a counted fact) or the
  minimal at-write check alone?
- Q2: are SERIES.md's ws_*_held_bytes rows (613, 642) free to reword in this change, or frozen/shared with another
  lane's evidence (C62H-BOUND froze two other rows; figures.rs:564)?
- Q3: u3c-plan.md is the approved U3c plan record: correct :122-124 in place with a dated note (as above), or add a
  correction note below it?
- Q4: does the loop run on HEAD + D-1 (confirms the cause by its counters) or on HEAD alone (confirms the rate only)?
