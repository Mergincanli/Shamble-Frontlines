# C62H-BOUND build review, round 1: netcode-architect (effort xhigh)

Diff read: 044fab188419e8859dbca699d851563c44c078c9..42b964618eed741f65ac04c1e5a06da0a82524d0 in
.claude/worktrees/c62h-bound (branch c62h-bound-build, HEAD 42b9646, clean). Read only; no cargo, no build, no test.
Memory: no netcode-architect memory dir found (.claude/agent-memory-local holds security-auditor only).
Hashes checked: brief 7936074c..., texts v3 3f21c9b7..., freeze 9cb8be17...; built marks.json c1935a5b..., SERIES.md
6898e119..., README.md f3c3c804... (the brief's "after" values).

## Plan
1. Read brief sections 0-5, 8.4, 10-11, texts v3 + freeze, design, rulings (r1, r2, brief), step returns 1-4,
   lane notes. DONE
2. Read the diff per file: l1 lib.rs/tests.rs; lab scenario.rs/logs.rs; judge figures.rs/engine.rs; fixtures;
   marks/; c-u4r-runs.mjs. DONE
3. Check: the join and its call site; E1/E2 vs the frozen rows; figures A and B vs their rows (count, window, gaps,
   order); the lab tally and the summary phrase; bitesOf, SPLIT_LINE, header; section 8.4's expected lines. DONE
4. Rule on builder points (a)-(h). DONE
5. Findings with file:line, smallest fix, line count; verdict. DONE

## What I checked (evidence)
- l1 lib.rs:412-431 server_datagrams_since: Datagram + Dir::Down only, from `from` (get(from..) none past the end),
  outcome Lost / Burst decided at the push (lib.rs:1131-1139). Only datagram() pushes Datagram entries (lib.rs:1103,
  :1143), only from send_datagram; with_fate_log is a read-only borrow (lib.rs:1918-1920). So between the two reads
  around server.send(slot) the only Down datagram entries are that slot's: the join is exact, and an extra or a
  missing one fails loudly.
- l1 test: the independent count from the canonical bytes (direction b[4], code b[5], kind b[6]; `i > from` skips the
  tag call) is genuinely independent of the function. Plants CB-P1/-P12/-P13 bit (logs named in step1-full.json).
- scenario.rs:610-631 link_lost: the brief's three checks in the brief's order; Ok(c.lost + c.burst). Call site
  :1046-1074: from/has_session/hash read before send, c after; Err wrapped with slot/tick and returned through `?`
  (no Drop writes footers: grep finds only Scratch's), so the run's logs end without footers (judge INVALID).
  Measured totals from c (never the written value), phase(k) of the room tick. Plant cfg(bites), once per slot,
  written value only.
- logs.rs:427-470 tally: `logged` and the new `link_lost` map are both removed at write_client_tick(w); flush walks
  every room tick in `phases` (logs.rs:500-512), so the new map is pruned as `logged` is (bounded). E1/E2/N partition
  Res::Lost in measured client ticks, so E1+E2+N = incomplete_measured by construction; N matches figure A on a valid
  run (same flag source, same written list, same phase, same written records).
- figures.rs:579-667 bound: the frozen row A and B counted exactly (A: lost & server hash non-null & link == Some(0);
  B: whole hash non-zero at a measured tick & link >= 1; no room record skipped). Window: client records with
  ph == Measured only; room side read on every record. Gaps: the eight, first occurrence each, returned in the row's
  order. Errors go to the lab log, never the client log (logs.rs:485), so no measured client record lacks
  snap_incomplete on a real run.
- Section 8.4: the summary phrase (scenario.rs:1708-1710) is the brief's word for word; the set check's PHRASE regex
  (c62h_bound_set_check.py:26-27) matches it; overflowFlag reads "overflow resyncs N", which the phrase does not hold;
  no other ops/ script parses the summary line. Dry print (.gate/u4r/c62h-bound/plan-dry.log): 22/22 asserts PASS,
  P0 9 runs, P2 5 runs, P3/B1 3, judge peaks 2129/552/1530/2253 MiB, SPLIT_LINE profile-free.

## Findings (a line after each)
- NA-CBB-1 Low: c-u4r-runs.mjs:13-15, the operator's stop instruction still says "In a set with bites (P0)"; P2 has
  bites now (bitesOf :312-322). Fix: "(P0 and P2: bitesOf)", 1-2 lines, not a frozen text. (point g)
- NA-CBB-2 Low: the judge's `bound` reader (figures.rs:579-667) has read only engine.rs's built logs and B-2's
  hand-written fixtures; no npt-lab log has been judged with the new checks (the short P2 lab test uses its own
  test-side join, scenario.rs:2859-2897). The first real read is the judged P2 set (~29 min). A surprise there is a gap,
  so INVALID (never a wrong PASS), but it costs a set and a stop. Fix: before the P2 set, one scratch judge call
  (untracked, not counted) of the built npt-judge with the built marks.json on one short P2 run's logs, reading A, B
  and their notes on -17's L1 P2 checks. 0 counted lines.
- NA-CBB-3 Low: row A and B say "over the run's client logs (process `client`)"; the reader takes source("decoded_hash")
  (process client OR bots: figures.rs:583, :623, :224-233), as the brief and the design named it. Every run the checks
  read (L1 P2/P3/B1 parts) has only a `client` log, so the computed value is identical; no fix; the report may note it.
  A change of either side is not needed (and the row is frozen).

## Points (a)-(h)
- (a) Confirmed: L = sum of c.lost + c.burst, B = sum of c.burst, so B is part of L; it matches the per-tick record's
  definition (draw or burst) and the Summary doc. The report should say "L includes B" once, for U4g's B1 readers.
- (b) Confirmed: the built order is the brief's order (no session with groups; c.sent vs groups, 0 with no session;
  groups with no hash); each join test isolates its own check.
- (c) Confirmed: precheck's run_leg writes no room log and no record; nothing judges -17 on it; adding the join there
  is outside the brief and adds lines.
- (d) Confirmed: slot_hashes' refusals and "room logs disagree on clients" are structural preconditions; the brief
  keeps their texts; the eight are first-occurrence and returned in the row's order. Zero room logs falls to gap 1
  ("no room log gives ..."), the row's own words.
- (e) Confirmed: the design ties "an entry below 0" to the link read; snap_incomplete -1 is "other than 0 or 1";
  both are gaps, only the phrase differs.
- (f) Confirmed: the row counts pairs; BTreeSet de-duplicates as decoded's own incomplete set does; `whole` needs no
  set (slot_hashes refuses two logs at one slot and tick). Unreachable on L1 (one client log).
- (g) Low, NA-CBB-1: fix in the fix pass if there is one; never a push on its own.
- (h) Accepted as ruled (r2 TE-CB-2; brief 8.4): judge-side E1 is pinned by c62hb_each_explanation_alone_reads_0 and
  CB-P14; the lab tally's E1 branch has no positive lab test, but misfiling N into E1 or E1 into N is caught by the
  N = A stop rule or by A > 0; only an E1/E2 swap in the report-only split would go unseen. Not a stop.

## Verdict
APPROVE WITH FINDINGS: three Lows, none touching a frozen text or what a figure computes.

## State
- complete; returned to the workflow.
