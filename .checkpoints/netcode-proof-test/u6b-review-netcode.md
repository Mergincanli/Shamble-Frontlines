# U6b's one review round: the netcode seat (xhigh, read only)

Inputs, hashes checked 2026-11-23 (all four match): u6b-brief.md 32baaebd...b99b; advisor-u6b-brief.md 3c0f8121...018d;
wip/u6b-build.patch 49f4488f...c14c (base 21765e3, 29 files, 5,498 added, 254 removed); u6b-build.md be6807f4...3c01.
W = /home/user/Shamble-Frontlines/.claude/worktrees/agent-a2661dc320c9e6d00/spikes/netcode-proof (read only).
Resumed after a spend-limit cut (17:33-21:10 UTC); nothing written before it but the hash check.

## 1. Plan
1. Inputs read: the brief (rev 1.1, sections 0.1, 1-4, 7), the advisor's check, the builder's notes, U6a's fix pass
   (its carries section), my U6a findings NR-U6A-7 and NR2-U6A-2 and O2, advisor-u7-brief.md's C2 (the homes).
2. The pub surface: list every new or changed `pub` item in the patch (npt-net, npt-link, npt-proto, npt-server,
   npt-super); read plan.md §4.7's whole-surface rule and check each against it; `begin_tick`'s signature, default,
   doc text and the Box forward; `ConnCap` (condition 8).
3. Fix 1's path: WsLink's gate (begin_tick, send_unreliable, the announced count, the three conditions of
   `may_write`, owed, the plain path's inputs and free value); TickLoop's I/O send site (the per-tick batch boundary,
   `try_reserve_many`/the all-or-none reservation, `send_tick`, `ws_skipped_snaps_count`); check the row against
   SERIES.md:330 and against what the I/O side counts.
4. The reader stamp: delay.rs's closing block (the stamp R, the split into pass and reader, the early class, the
   identity and its violations count), the pool's reader parts with the window mark, the misattribution bite, the
   delay_names rows, the loopback self-test.
5. Added items 1 (npt-proto's three constants, `Channel::ws_tag`/`from_ws_tag`, catalogue line, the equality test),
   3 (NR-U6A-7: runtime enable, `io_handle`, `stop` off the async context), 4 (NR2-U6A-2: exited tids).
6. The new SERIES.md rows ("The transports (U6b)") against what R1 reads (R1.md in wip/u8-build.patch, the U8
   rulings C-U8-3, C-U8-5) and against the code that writes them (names and units match).
7. The carries of section 4 by number (SP-1 to SP-8, RC-1 to RC-12, C-U6-17's form, U6a's condition 6, C-U6-19's
   line), each met / moved / open with file:line.
8. The F-items: F-PING (rule it), F-MARK-HDR, F-NEG, F-POOL-BYTES, F-WT-UNSENT, F-SKIPPED-ROW, F-RT-IO, F-VALVE,
   F-JOIN-ONCE.
9. Tests through the runner only when a claim needs one (one command per call; exit 3 = disk floor, go on reading).
10. Verdict, findings table, F-item views, not verified, DONE.

## 2. Notes (one line per finding, saved as I go)
