# U3C-HELD-1 — netcode-architect review (round 1, final diff)

Read only. Scope: `git diff -- spikes/netcode-proof` (delay.rs, delay_tests.rs, marks/SERIES.md; 74 + 42 = 116 lines)
and the u3c-plan.md in-place correction (104b234). Working-tree hashes read 2026-11-17: delay.rs 3dbbccf7…,
delay_tests.rs 8f8c0f9a…, SERIES.md 1dd67e2f… (= build.md's fixed hashes; = E3/E5/light headers' diff a7199f7a…).
Line numbers below are the working tree's.

## Plan
1. Read the plan draft, the advisor review JSON in full (C1-C10, HELD-Q1-Q12), the design notes, the evidence design.
2. Read the diff; read delay.rs around Life / model_write / check_held_bound / check_held_falls / held_grew, and the
   real (non-test) delay code's hold/release path, to judge:
   a. bound checked at every write, both directions, same formula (not looser);
   b. check_held_falls soundness: can held legitimately rise between writes anywhere;
   c. doc texts match code (module doc, Life, check_held_bound, check_held_falls, model_write; delay_tests docs;
      SERIES.md rows; u3c-plan.md).
3. Read build.md and the logs (l1, l1-1-94, light, e3a, e3b, e3c, p1-p5, e5) against what build.md claims.
4. C9 difference-set argument: is it carried in the evidence / report text?
5. Check the plan for what's still to come (C5 552/632 etc.).
6. Verdict + findings.
Status: all done. Verdict: APPROVE WITH FINDINGS (one Medium, checkpoint text only; three Lows).

## Checks (what holds)
- K1 at every write, both ends: bytes grow only at delay.rs:829 (UP, move_up) and :1597 (DOWN, poll_write); each is
  followed under the same lock guard by model_write (:830, :1615), whose last statement is the cfg(test)
  check_held_bound(dir) (:634-635). No held_now between (the down scan loop :1599-1614 calls none). model_write has
  exactly these two callers; scan_up/move_up one caller (:1838) after step_to(t, Mid) (:1837); poll_write's current()
  (:1580, :1186-1206) steps to now first. P1 (down) and P5 (up) bite at :782 — both ends proven.
- K2 same formula: check_held_bound's body (:539-550) is the old loop body verbatim (held = bytes[d].len(); una =
  model_end − outstanding; prefix = min(una −sat start, size) of recs[d][released[d]]; held > budget + prefix). At the
  write the bound's terms equal what the old check read at poll_write's held_now (:1617) — only `owed` moved. For up the
  old check never sampled at a write (move_up has no held_now): the new one is strictly wider there. Not looser at writes.
- K3 bound holds by construction at a write (independent of harvest lag): admission n+22 <= free_budget − released_model
  (:478-489, :1584-1595, :826) and held_model <= released_model + outstanding + prefix use the same released[] state.
- K4 check_held_falls sound: drains only at :1526 (poll_read) and :1771 (pump take); grows only at K1's two sites;
  parse_len (:1835) is outside bytes[UP]; Conn starts empty (:1021); the only test mutation (delay_tests.rs:1796) flips a
  byte, length unchanged. No legitimate rise between writes, up or down. held_at_write is per direction, set only by that
  direction's write. Limit (documented in NA-UH1-3): it checks "<= the latest write's held", not a monotone fall.
- K5 held_beyond_budget now kept only at writes (:549-550); with held_grew == 0 asserted first (delay_tests.rs:2313
  before :2314) the max at writes equals the max over all samples, so G-NC2's third assert is not looser. Loop exit
  (`beyond > 0`, :2285-2287) now needs a write above 64 KB — P4's hang guard shows the dependency.
- K6 non-test code unchanged by diff audit: Life is #[cfg(test)] (:377); held_now's call and model_write's call are
  #[cfg(test)] statements; both check fns #[cfg(test)]; delay_tests.rs is a cfg(test) module (:1915-1917); the rest is
  doc text. E5 (npt-lab 98c3967e…) consistent.
- K7 stalled-client test: RECORD_MAX post-hold write is Pending iff free <= 22, independent of buf size (:1584-1592), so
  the correct-code outcome equals the old 100 B one; under P1 it takes 16,384 B: 64,336 + 16,384 = 80,720 > 65,536.
- K8 Difference set (C9; my own derivation, agrees with the advisor's C6b): between writes una only rises (outstanding
  falls, model_end fixed), so for one front record p = min(una − start, size) never falls; pop_front at a take
  (:1530-1534, :1782-1785) decrements released[] with recs, so the front unreleased record is unchanged by takes; p falls
  only when harvest moves released[d] past the front record (:698-709). Old flagged ∧ new passes ⇒ held(s) <= held(w) <=
  B + p(w) and held(s) > B + p(s) ⇒ p(s) < p(w) ⇒ a release since w. Nothing is admitted between writes (K1). So the
  pair is looser only on the release transient, which admits no byte; every sample stays < 64 KB + 16,406 B.
- K9 evidence: l1.log run 94 (exit 101, the one failing test G-NC2; D-1 over_waiting [1, 0] = held_over_bound,
  over_at_write [0, 0]; excess 14,326; first losses 28) — Confirmed line whole. e3a 600/600 "1 passed; 0 failed",
  e3b 600/600, e3c 20/20 "169 passed; 0 failed", all with the final diff a7199f7a…; light PASS (fmt, clippy both builds,
  doc host+wasm32); p1-p5 as build.md states (values and lines checked in the logs; the per-run logs list both failing
  tests for P1/P3/P4); restores: the current hashes equal the fixed ones.
- K10 doc texts: module doc :46-53, Life :372-376, check_held_bound :527-536, check_held_falls :554-556, model_write
  :617-618, delay_tests :741-745, :2265-2272, SERIES :613/:642 (old phrases 0, new 2 and 2), u3c-plan :122-128 —
  exactly the design notes' texts; accurate except NA-UH1-3/-4 below.
- K11 still to come: C5's 552 / 632 stays right (no test added; e3c's 169 = L1's 169). Free now 4,953 MiB (df
  5,193,547,776 B): above the floor at E6's start; U4f's shortfall after E6 as C6 says. The E6 header's diff/untracked
  are scoped to spikes/netcode-proof (headers show e3b0… untracked while .checkpoints files were untracked). Nothing
  wrong found in the remaining steps.

## Findings log
- NA-UH1-1 (Medium): C9 not met yet. The plan's "Not looser" line is unchanged (plan sha256 still a53b2803…, one
  commit ec70096; :128-129), build.md carries no difference-set argument, and C9's two other plan corrections (the card
  entry by name, the saver sentence) are not made. The argument exists only in the advisor's notes (C6b) and here (K8).
  Fix: put K8's text, with its cites, into the plan's line (dated note) and the report; and say in the report that E3's
  1,220 passes show only that the new pair raises no false positive — they cannot show the old one gone (E3a's
  single-test mode was never run on HEAD, so its rate there is unknown; e3c's 20 whole-binary runs would see a
  1-in-100 flake with about 18 %); that rests on K8 plus L1's D-1 counters.
- NA-UH1-2 (Low): P1's G-NC2 FAIL ([1, 0] at :2308; p1.log) is recorded without its cause, against the design notes'
  F15 prediction (mine) that G-NC2 cannot show P1. Likely cause (reading, not proven): a record released after a loss
  inside it carries its already acknowledged prefix into the released-and-untaken state; when the writer's own step
  (poll_write's current(), By::App) releases it in the same step as acks that freed room, the planted down_free admits a
  write before the pump takes it: held ≈ 64 KB + p against 64 KB + 0 — the same window as the original false positive.
  Fix: one line in build.md and the report; never repeat F15's "cannot show it"; G-NC2 is a probabilistic second
  witness for P1, the stalled-client test the deterministic one.
- NA-UH1-3 (Low): "held only falls between writes" (assert messages delay_tests.rs:787, :2313; G-NC2 doc :2267-2268)
  states more than check_held_falls checks (delay.rs:558-564: never above the latest write's held). A growth after a
  drain that stays under held_at_write is not counted (harmless for the bound: the next write's check sees it). The
  check's own doc (:554-556) is exact. Fix (optional): message "held never above the latest write's, between writes",
  or keep and say in the report that "only falls" is the code reading (K4) and the count checks its consequence.
- NA-UH1-4 (Low): precision gaps in the new doc texts. (a) SERIES.md:613/:642's up part (and u3c-plan.md's Up line)
  says "at most, at each write, …" but, unlike the down part, nothing about a second's-end reading (a between-writes
  sample), which is what the row reports; the module doc's up sentence (:51-52) likewise. Fix: add "the records only
  falling between writes" to the up part. (b) Life's doc (:375) "the largest excess of the records held over the 64 KB
  budget" is now kept at writes only (:549-550). Fix: "at a write". (c) check_held_bound's "(model_write, the only place
  they grow)" (:528): the bytes grow in its two callers just before (:829, :1597); model_write's doc says so. Nit.
