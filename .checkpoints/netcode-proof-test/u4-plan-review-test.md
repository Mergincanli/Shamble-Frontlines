# U4 build plan (rev 1) review: test-engineer

Task: adversarial review of .checkpoints/netcode-proof-test/u4-build-plan.md (rev 1), against scouts u4-scout-arch.md,
u4-scout-marks.md, u4-scout-netcode.md, plan.md and the code. Focus: requirements and carries met and proven, marks
with tests that bite, judge figures, fixtures and evidence honest, nothing judged here that only a run can judge,
split testable unit by unit. No code edits, no builds, no runs. Memory: no agent-memory dir found (.claude/ has none).
Rule (checkpoint :294-295): notes by the Write tool only, no shell writes, no cd. (The first append went in by a shell
heredoc before I read that rule; this rewrite carries it unchanged.)

## Plan
1. Read the build plan in full.
2. Read plan.md U4 section, R/C carries that name U4; the marks for U4.
3. Read the 3 scouts; check each claim the plan leans on.
4. Check code file:line for the plan's claims (crate layout, existing harness, judge).
5. Findings list; verdict.

## Progress
- step 1 done: read u4-build-plan.md rev 1 in full (646 lines).
- step 3 done: scouts read (marks F1-F32, arch F1-F48, net F1-F124) and the plan's notes (M1-M25).
- step 2/4 done: marks.json read by python (read only) for -1, -2, -3, -17, -18, -20, -21, -22, -30, -41, -42, -45,
  -47, -48, -49, -50, -51, -54, -55, -56, -57, -58, -60, -61, -62, PM-0004-67; judge eval.rs :15-75, :460-516,
  :741-790, :921-935, :1052-1360; README §3-§5 (:140-215); SERIES figures (:259-300, :736 Q30); figures.rs
  :150-240; log lib.rs :36-120; l1 lib.rs :1-60, :105-170, :225-270, :348-365, :858-868; Cargo.toml :90-107;
  advisor-u3q.md:158-190; advisor-ws-docs-r2.md:368-386; advisor-u2.md:15-35; checkpoint :296-330, :615-620;
  ADR 0001:1465-1467, :1636, :1641, :1650; ADR 0002:498-506; ADR 0004:1142; routine-guard.mjs:24-44;
  catalogue :240-253; plan.md:532-552, :607-611.
- step 5 done: findings T1-T23 below; verdict REVISE.

## Sources read (summary per source, as read)
- S-RFC9002 (www.rfc-editor.org/rfc/rfc9002.html, fetched 2026-11-09, first 100,000 of 105,234 chars): §6.2.2 "When
  no previous RTT is available, the initial RTT SHOULD be set to 333 milliseconds"; §5.3 before any sample
  smoothed_rtt = kInitialRtt, rttvar = kInitialRtt / 2, the first sample resets the estimator; §6.1.2 time threshold
  max(kTimeThreshold x max(smoothed_rtt, latest_rtt), kGranularity), kGranularity 1 ms recommended; §6.2.1 restart the
  PTO timer on each ack-eliciting send or acknowledgement, backoff doubles at expiry, reset on an ack (client Initial
  exception). Sure: high (spec text).

## Hypotheses checked (H-n -> finding or dropped)
- H12 check.mjs protected? DROPPED: routine-guard.mjs:26-27 GATE_FILES lists only tools/ files; spikes/ is not.
- H6 -41 per option: LOW (T21). Checks are per run (eval.rs:1141-1160), so pooled A+B runs each must pass.
- H19 process `server`: no U4 L1 check reads it; nit only.
- H20 machine_named: -1's parts select RS/LG/ML/RD/MAC only (marks.json -1): no U4 run feeds it; fixtures only. Dropped.
- H1 -18 eviction: recomputed (T15): at P2 B's age at the first tick after return is at least 32 ticks (one-way
  minimum 40 ms; acks sent before the burst; tick grid floor/ceil), so eviction is certain iff the ring keeps ages 0-31;
  an off-by-one (ages 1-32) keeps B on the age-32 bursts. The run still fails under the bite (other bursts evict).

## Findings (one line each; the final message carries the full text)
- T1 HIGH Q22 pre:PM-0001-2 makes -22 INVALID by design (never-acker kicked: plan :173-176; PM-0001-2's >= 48 clients).
- T2 HIGH -30 pre-check: matches the C part (no layer select) or cannot be judged; Loopback has no layer value.
- T3 HIGH -17's mirror hash undefined under group splits and loss (plan :156-170; SS near the cap).
- T4 MEDIUM -17's tick16 and client-rounding bites have no site that must change the hash.
- T5 MEDIUM hash-map bite flake (first-key outcome p ~ 1/32; PM-0004-21's "two orders logged" omitted).
- T6 MEDIUM bites needing run configuration: -45, -48, -62, -58, -21 not-older-than, -41 reading.
- T7 MEDIUM -21 twin not ack-aligned for voided datagrams; feedback from decoded state.
- T8 MEDIUM -21 catch-up forged kind vs the alignment rule (count field max, datagrams per tick).
- T9 MEDIUM determinism legs' profile (ADR 0002:500-505) and check.mjs time; recordings' source in check.mjs.
- T10 MEDIUM bite selection per instance (parallel tests; l1 lib.rs:858-868 precedent).
- T11 MEDIUM split: judge change its own sub-unit before U4c's -3 re-judge; U4e build apart from its runs.
- T12 MEDIUM C1' gaps: initial PTO, kGranularity, bites for one packet-number space, poll independence, tie rule.
- T13 MEDIUM unbitten parts (-42 L1, -49 test cap) and vacuous PASS of -42's P2/P4 parts (no checks).
- T14 LOW-MED test cap calibrated on judged seeds makes clipped == 0 true by construction.
- T15 LOW-MED -18 eviction rests on the ring's off-by-one; pin it and log eviction per burst.
- T16 LOW-MED check-script additions without self-test bites; TICK_HZ derivation beyond LOOP_CATCHUP untested.
- T17 LOW-MED negative cases: Join rejections, pre-Join, rate buckets, deadline, AddrTable, Resync, ML/RS-only logic.
- T18 LOW-MED contract test must cover validity inputs (roster vacuous without admit events).
- T19 LOW codec test wording (a2 prefixes, a3 precedence, a4 tolerance).
- T20 LOW -57 pairing with the carrying snapshot.
- T21 LOW accounting and citation slips.
- T22 LOW D-7's "no usize" untestable; use PM-0004-21's cross-target bytes.
- T23 LOW-MED L-c8: compare SHA-256 of (v1 tag + new entries) to the committed hashes; say whether acks write entries.

DONE
