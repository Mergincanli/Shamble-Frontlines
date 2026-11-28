# U4r review round 2: rust-reviewer seat (notes)

Snapshot: wip/u4r.patch sha256 21ca46be23704131c77cc20981a4bfe866811b33f9ca853279f7ddbfb5d58bce (verified)
Packet: u4r-r2-packet.md sha256 b1526bde... (verified); tests file 637249a5... (verified); runner 6efb9157... (verified)
Read copies: .gate/u4r-snap2 (new; diff -rq equals the builder worktree's spikes/netcode-proof), .gate/u4r-snap1 (round 1)

## Plan
1. Read ruling advisor-u4r-r1-full.md "## Round 2" whole; my seat's line under "Who reads what"; "The fix pass, in order"; "Each finding" for RR3- ids. DONE
2. Read my round-1 findings (u4r-review-r1-full.md "## review-rust"). DONE
3. Diff snap1 vs snap2 (changed lines only). DONE for hash.rs, scenario.rs, logs.rs, precheck.rs, replay.rs (+ pub-item scan of the rest)
4. Check each RR3- finding fixed/deferred as ruled, file:line in snap2. DONE
5. Check builder report "## Fix pass, round 1: report" claims for my seat against code and c6-*.log. DONE
6. New findings on changed lines only. Build only if a finding needs it. DONE (no build: both findings shown by reading and arithmetic; 4.57 GiB free)
7. Result via StructuredOutput.

My list (ruling "Who reads what"): FP-3, RR3-L4's cfg gating in both builds, RR3-L7, EA-U4R-3/RR3-L8 with the new expects.

## Round-1 findings
- RR3-M1: fixed as ruled. hash.rs:237 RUNNER_PROFILES, :339-344 profile by name; scenario.rs:589-637 record() argv as separate items (no split). Plant c6-plant-fp3.log: r10_fp3 fails at hash.rs:1596 "the profile \"P2 --record x\"". c6-fp3-1.log 9 passed + clippy exit 0. The argv rewrite keeps settings: c6-det-u4.log (SS-P2-s1, ES-P3-s2) and c6-regen-precheck.log:20 (SS-P0-s4, sha256 6933b56a..., "as the manifest's") regenerate through it.
- RR3-L1: fixed as ruled. hash.rs:234 MAX_ENTRY_BYTES 64 MiB, :325-331 cap; :345-356 ticks == (w+m) x 30 checked; :357-364 span_ms == ROOM_HISTORY_MS (scenario.rs:58, which run() writes at :855). Test hash.rs:1594-1650. (Residual: RR4-L1.)
- RR3-L2: (b) fixed as ruled (doc scenario.rs:889-891; true: room_tick only under `if s.logs` :971-975). (a) deferred as ruled; listed in u4r-build.md:1944.
- RR3-L3: fixed as ruled. scenario.rs:1365-1381 checked products/sums; :1442-1453 last burst checked (bounds every tick-sum in burst_starts :1306); test :2420-2431. (Residual on the ns side and the count: RR4-L2.)
- RR3-L4: fixed as ruled. logs.rs:163-166, :196-197, :215-218, :308-322, :337-342; precheck.rs:167-172, :176-197, :221-234, :319-321, :336, :357-360. Logic equal to snap1 by reading (held/release, as_lost then resolved). Both builds: c6-end2/end3 clippy "measured build and bites build both linted", test 531/609 (+5/+6 = the new tests only). Bites still act after the edit (edits 11:39-11:47): c6-fp6-3.log:128 precheck_bite_late_command..., :131 p4_bite_whole_as_lost... ok.
- RR3-L5: fixed as ruled (report u4r-build.md:1927-1929). Recounted now: 124 total, 93 older than 00:28, 31 newer, 0 newer than 00:35.
- RR3-L6: deferred as ruled (u4r-build.md:1944-1945).
- RR3-L7: fixed as ruled. replay.rs:264-274 u8 1..=255 before ReplayX86::of and the loop; test replay.rs:370-414 (exact message; bytes folder absent).
- RR3-L8 (= EA-U4R-3): fixed as ruled. scenario.rs:219 schedule from with_bite.is_some(); expects :273-274, :309-310, :320-321. Unreachable and pinned: World::with_bite Some for every World::BITES id (sim/tests/world.rs:2309-2315), L1Net::with_bite Some for every L1Net::BITES id (l1/tests/l1.rs:736-738), schedule by construction. Test scenario.rs:2440-2451 (schedule false, pump true).

No new pub item, dependency or seam change (scan of the diff: only pub(crate)/private/test items; LinkState from the existing npt-link dep).

## New findings (changed lines)
- RR4-L1 (Low): hash.rs:229-234, :319-331 - bytes cap + ticks <= bytes/12 still allows 5,592,405 ticks; committed SS/ES recordings are 1,855-1,883 B/tick (35,639 ticks fit 64 MiB), so an entry within every check makes generate() pre-allocate sim/src/replay.rs:296 `Vec::with_capacity(HEADER_BYTES + ticks * 1_900)` = 10.6 GB (synthetic encode; runner record_mem grows likewise, plus RR3-L2 (a)'s res) before check(). Fix: MAX_ENTRY_TICKS 36,000 (4x today's 9,000), refused by name in manifest(), with one case.
- RR4-L2 (Low): scenario.rs:1442-1453 + comment - the check bounds burst_starts' tick sums only. due_ns (:130-131) truncates `as u64` above 553,402,322,211 ticks, and :1307 `boundary + below(..)` is a plain add (npt-lab has no release overflow-checks: Cargo.toml:116-120). E.g. --profile B1 --bursts 2 --burst-every-s 600000000000 passes (tick 18,000,000,001,200), due_ns gives 6e20 ns, truncated. --burst-every-s 0 with a huge --bursts passes too and sizes `(0..count).collect()` unchecked (capacity-overflow panic or OOM). Fix: refuse a run's last tick and the last burst's tick above u64::from(u32::MAX) (due_ns(u32::MAX) = 1.43e17 ns fits; also the u32 tick wrap at :862), and refuse --bursts above a small cap (e.g. 1,000) or --burst-every-s 0 with more than one burst.

Verdict: APPROVE WITH FINDINGS (two Lows: STATUS follow-ups under the budget).
