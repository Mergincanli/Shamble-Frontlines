# u3-build-v3-harness: the WebSocket fallback's harness and statistics, v3 (fixes.rs only)

(Copied by the main session on 2026-11-07 from the build agent's worktree copy at
.claude/worktrees/agent-a487e419ee806d37b/.checkpoints/netcode-proof-test/u3-build-v3-harness.md: the Write tool
refuses the main folder's path from an isolated worktree. Content unchanged below this line.)

Agent: build agent (general-purpose, opus). Worktree /home/user/Shamble-Frontlines/.claude/worktrees/agent-a487e419ee806d37b,
fast-forwarded from d8c485a to 72e6fab (claude/netcode-proof-test-plan-hcky5g). Edits ONLY
spikes/netcode-proof/crates/net/src/tcp_model/fixes.rs; nothing committed. Target dir .gate/u3v3-harness-target.
Scratch: scratchpad/u3v3h/ (Chromium websocket_factory.cc fetched 2026-11-07 to check the throttler's argument order).
Note: the Write tool refuses the main folder's path from this isolated worktree, so these notes live in the
worktree's copy of .checkpoints/netcode-proof-test/ (the main session copies them over).

## Plan
1. Restructure run() into one event loop over a list of bursts (none = side effects; one = a phase; N = the multi-burst
   run), with per-burst records (recovery, attempts, takeovers before/after the recovery, cancels, throttle and fail
   delays, cap hits).
2. H6 cancel-on-fresh: a pending attempt (waiting or before its 101) closes when the current connection delivers a
   whole snapshot written at or after the attempt was created; after its 101 it runs to its Join. A phase follows on
   until no attempt is pending, at most 3 s past the recovery; takeovers per burst reported.
3. H7 busy = waiting or a live connection the client has not switched to (client-visible). H8 the cap per outage,
   reset on a whole snapshot on the current connection. H19 last_data on whole messages only.
4. Input phase = TICK + splitmix64(seed) mod TICK.
5. Browser option: Chromium (default: per-endpoint lock freed 10 ms after the 101 or the end; the per-renderer
   throttler, integer ms, p excluding the new attempt as clang's MS ABI orders the arguments; two-bucket roll every
   2 min from t = 0; its own seeded stream) and Firefox (one admission slot per IP; fail-delay record per endpoint:
   200-400 ms, x1.5 on a network failure, page close adds but never grows (NS_ERROR_NOT_CONNECTED), cleared by an open,
   expired 60 s + delay after the last failure; open timeout 20 s).
6. TLS flights: resumption default (1,900 B ClientHello, 1,300 B server flight), the full handshake (1,800 / 3,800 B)
   as an option.
7. Multi-burst harness (N bursts S apart on one lineage; N = 10 and 30, S = 20 s; parameters).
8. Statistics: DP joint (classes <= X, X-Y, Y-Z, > Z; median optional) cross-checked against a closed form; Wilson
   intervals on rates; block-bootstrap intervals on pass chances; 600 sets on a worker pool; bite rows today / fix 1
   alone / fix 3 alone / correct; two-part and three-part bars at 480 and 1,440; a search over nearby bars.
9. Pinned test: FNV hash of the sorted recoveries + counts over 0.8/1.0/1.2/1.5 s (+ attempts and takeovers), no goal
   assertion. Fix 2 rows out; parallel row a labelled reference. Slot for the Windows SYN option (attempt_cfg).
10. cargo test (fixes), the tables (--ignored), fmt, clippy -D warnings, node check.mjs. DONE.

## Source checks (finished block)
- Chromium services/network/websocket_throttler.cc (saved copy scratchpad/cr2, fetched 2026-11-07): delay =
  base::Milliseconds(RandIntInclusive(1000, 5000) * (1 << min(p + f / (s + 1), 16)) / 65536): int arithmetic, so whole
  milliseconds, 0 ms until the exponent reaches 4 (:46-55); failed = a tracker destroyed before OnCompleteHandshake
  (:27-33); success at OnAddChannelResponse (services/network/websocket.cc:371-374, the 101); Roll() previous = current,
  current = 0 (:70-76), timer every 2 min started by the first tracker (:124-127).
- websocket_factory.cc (raw.githubusercontent.com chromium main, 2026-11-07): the WebSocket constructor gets
  throttler_.IssuePendingConnectionTracker(process_id) then throttler_.CalculateDelay(process_id) as arguments
  (:99-100). C++ leaves the order unspecified; clang evaluates right to left under the Microsoft ABI (Windows), so there
  p excludes the new attempt; left to right elsewhere (p includes it: one more doubling). Modelled: Windows order.
- Firefox WebSocketChannel.cpp (saved copy scratchpad/chromium-src/ff_WebSocketChannel.cpp): FailDelay 200 +
  rand() % 200 ms (:113-131), FailedAgain x1.5 capped 60 s (:134-145), RemainingDelay from the last failure
  (:148-154), expired at last failure + 60 s + delay (:156-160); keyed (address, path, port); admission queue keyed
  by address without the port (IndexOf); OnConnected removes the record (:376-379); OnStopSession: a known record
  grows only for a reason other than NS_ERROR_NOT_CONNECTED (a page close before the 101), else a new record (:389-421);
  the next queued channel starts at once on ConnectNext; open timer 20 s from BeginOpenInternal (:1181, :1393).

## Progress (a line per item built)
- Items 1-11 written in one pass (fixes.rs rewritten around a `Sim` struct: one event loop over B1's burst list,
  per-burst `BurstOut`, `Gate` for the browser, `Throttler`, `FailRecord`, `Waiting` attempts, `Tls`, `ClientOs` slot,
  DP + closed-form joint, Wilson, block bootstrap, worker pool). Compiles clean (release, tests). The two new unit
  tests pass (joint vs brute force and closed form; throttler and fail delay vs their sources). First pinned run
  (6,000 phases, recommended): p50 435, p99 663, p99.9 877, max 1,354 ms; over 0.8/1.0/1.2/1.5 s 31/4/4/0; attempts
  6,000, cancelled as the old connection recovered 5,916, takeovers 56 before and 28 after the recovery (v2: ~99 %
  of phases ended in a takeover after the recovery). Next: tables, then pin, fmt, clippy, check.mjs.
- ws_limit_table (60,000 phases a row, 40 s wall; output scratchpad/u3v3h/table1.txt): today 746/953/1,840/max 4,757
  (446 over 1 s); fix 1 alone 436/664/1,523/max 3,814 (422 over 1 s, 75 over 1.5 s); fix 3 alone 744/908/1,018/max
  2,017 (72 over 1 s); recommended 436/659/943/p99.99 1,409/max 1,715 (45 over 1 s, 3 over 1.5 s), takeovers per
  phase 0.0079 before and 0.0059 after the recovery, 0.986 attempts cancelled; Firefox identical; full TLS 47/2;
  no timestamps 50/4; abort 500 Chromium 69 over 1 s, Firefox 66 but 0.19 takeovers after the recovery a phase (the
  fail delay pushes the 2nd attempt past the return, so its 101 beats the old connection's recovery); abort 1 s
  Chromium 49, Firefox 330 over 1 s (H12 reproduced: 370); two endpoints 19 over 1 s, 4 attempts a phase; parallel
  (reference) 7 over 1 s.
- side_effects_table (scratchpad/u3v3h/side1.txt, 0.4 s): skipped per mille P1-P4: adaptive 4/18/135/119; with the
  reconnect 4/18/81/119; 30 min attempts 0/0/10/21, closed as data came back 0/0/5/21, takeovers 0/0/5/0 (Firefox
  and abort 1 s the same). P3's 135 vs 81: to check (one seed's stall?).
- multi_burst_table (scratchpad/u3v3h/multi1.txt, 51 s): ACCUMULATION SHOWS, in TCP's state above all: fix 1 alone
  burst #1 p99 631 ms, #5 1,455, #10 1,509 (single bursts 664); recommended #1 631, #5 983, #10 967; burst #3's p50
  is 523-525 ms in every row (others ~435). Pass chance of "half within 0.6 s, at most 1 of 480 over 1.2 s, none over
  2.0 s" from the 10-burst lineage rates: recommended 0.777 vs 0.989 from single bursts; 30 bursts 0.628. Chromium's
  throttler: recommended 30 bursts, 8,085 of 30,000 bursts throttled >= 1 ms, longest 444 ms (#25); abort at 500 ms
  longest 4,930 ms (#30; f grows ~2 a burst, s ~0). Firefox never-abort = Chromium without the throttler. Suspect the
  model's S2 (cwnd never grows back after a timeout: no Cubic) for the TCP part: diagnosing; the model v3 build adds
  Cubic, so the main session must re-run this table after the merge.
- DIAG (temporary tests, removed): the accumulation is the model's S2, not the harness. A single burst later in a
  connection's life is no worse (fix 1 alone, one burst at 5/25/45/65/105/205 s: p99 664/660/662/675/668/658,
  over 1 s 146/144/126/161/146/132 per 20,000; recommended 14/21/20/15/25/13). Along a 10-burst lineage the room's
  cwnd at each burst start falls 10 -> 7 -> 4 -> 2 and stays at 2 from burst #4 (300 lineages, every one): each
  timeout cuts ssthresh to 0.7 cwnd and the model never grows cwnd above ssthresh (S2: no Cubic). Quiet runs show the
  same ratchet (P3/P4 cwnd 14-16 falling to 4-8 within 10 minutes), which is why P3's skipped share is 135 per mille
  without the reconnect and 81 with it (takeovers start fresh connections). The model v3 build adds Cubic: the
  main session must re-run multi_burst_table and side_effects_table after the merge before reading them.
- ws_limit_tail first run (scratchpad/u3v3h/tail1.txt, 117 s): recommended 600,000: 435/661/967/p99.99 1,448/max
  2,067; over 1/1.2/1.5/2 s 532/260/40/1. Fix 1 alone 4,313/4,284/813/81 (max 4,750); fix 3 alone 767/241/159/5
  (p50 745); today 4,404/3,905/3,904/218. Pass at 480: (1.0, 1.5) rec 0.968 [boot 0.958-0.978], fix 1 0.442, fix 3
  0.880, today 0.044; three-part 480 rec 0.981 [0.975-0.985], fix 1 0.141, fix 3 0.000, today 0.000; three-part
  1,440 rec 0.994 [0.989-0.997], fix 1 0.008. BUG found in my "rates" interval (q0 not absorbing for two-part bars:
  values above 1, ends reversed): fixed (the free class absorbs). Search picked Z = 1.5 bars at rec 0.955 (interval
  0.944-0.965): changed to require the bootstrap's lower end >= 0.95, dedupe looser medians. Re-running.
- Slot for the Windows SYN option: ClientOs::Windows { initial_rto_ms } (allow(dead_code) until wired) and
  client_cfg()'s Windows arm (unimplemented! naming what to set); used for the established connection and every
  attempt.
- RESUMED after the worker restart (~10:15 UTC): fixes.rs intact (fmt applied, the DIAG block cut, 0 "DIAG" left).
  The coordinator asks for a mirror of these notes at the main folder's path (Write tool, whole file): tried again,
  the Write tool refuses it ("isolated in the worktree ... edit the worktree copy"); no shell write tried (the brief
  bans them for notes, and it would cross the isolation). The main session copies this file. Next: clippy, the tail
  re-run (fixed intervals and search), the pin, check.mjs.
- clippy -D warnings clean (release, all targets; and with --all-features). Tail re-run 2 (scratchpad/u3v3h/tail2.txt,
  84 s): the rates intervals now bracket the point (rec (1.0, 1.5) 0.968 [boot 0.958-0.978; rates 0.957-0.977]).
  Search with the bootstrap constraint: at 480 "half within 0.5 s, at most 1 over 1.2 s, none over 1.6 s": rec 0.966
  [0.957-0.974], fix 1 alone 0.130, fix 3 alone 0.000, today 0.000 (Z 1.8: 0.977/0.138; Z 2.0 = the skeptic's bar:
  0.981/0.141); at 1,440 "half within 0.65 s, at most 2 over 1.2 s, none over 2.2 s": rec 0.974 [0.966-0.982], fix 1
  0.002; (k 2, Y 1.3 s, Z 2.2 s): 0.990/0.006. Ties on the median were ordered by float noise: sort now rounds the
  correct build's chance to thousandths before preferring the loosest median.
- PINNED: PINNED_RECOMMENDED = (0x20e2_d621_a6c1_1f84, over 0.8/1.0/1.2/1.5 s [31, 4, 4, 0], attempts / cancelled /
  takeovers before / after [6,000, 5,916, 56, 28]); the goal assertion dropped. fixes tests 3 pass, 0.28 s release.
  Next: the last tail run (sort tweak), check.mjs.
- Tail run 3 (scratchpad/u3v3h/tail3.txt, 80 s; the numbers above unchanged): search picks now show the loosest
  equivalent median, 0.7 s: at 480 "half within 0.7 s, at most 1 over 1.2 s, none over 1.6 s" rec 0.966
  [0.957-0.974] / fix 1 alone 0.130; at 1,440 "half within 0.7 s, at most 2 over 1.2 s, none over 2.2 s" rec 0.974
  [0.966-0.982] / fix 1 alone 0.002. check.mjs running (CARGO_TARGET_DIR .gate/u3v3-harness-check-target, output
  scratchpad/u3v3h/check1.txt).
- check.mjs in the worktree: all 9 steps PASS (fmt; clippy and clippy-wasm32 on both builds; test 112 / 117 passed;
  deps; refs; marks; self; redact). Final diff: fixes.rs only (+1,664 / -425); nothing committed. Report handed back.

DONE
