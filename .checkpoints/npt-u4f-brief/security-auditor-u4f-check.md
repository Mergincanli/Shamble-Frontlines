Task: night 2026-11-16 item 7 (npt-u4f-brief), security-auditor check of U4f brief rev 1 (hostile-input half). Notes and check file.
(No Edit tool in this agent: the file is rewritten whole with Write at each save.)

## Plan (blocks, in order)
- B1. Read the approved plan (781212 night-2026-11-16.md) - its Agents item 2 bullet is the authority.
- B2. Read sources: N/u4-build-plan.md §4.10 (:360-390), §4.12, §4.14 (:453-476), §5 U4f; N/plan.md §4.4; N/advisor-u4-wave1.md:492-495 (U4E-Q11); marks.json -21/-22; N/night-u4f-scout-full.md (map).
- B3. Read the carries' text at N/u4s-carries.md for 46, 80, 76, 74, 57, 86, 21, 27, 18, 39, 50, 61, 58, 77.
- B4. Read the draft rev 1 (837 lines) and the drafter's notes (NA-6, NA-7, NA-8).
- B5. Read the code the draft cites for the hostile-input half (room checks, -21 counters, Kick path, pump) at 11fd190.
- B6. Threat model: list what a forged client can send or do; table refusal/counter/twin/bite per kind vs the draft.
- B7. Findings SA-n.
- B8. NA findings verdicts; could-not-do list; DONE.

## Notes
- B1 done (plan read; my bullet: §4.10's kinds, clamp points, 1.5x clock, lag switch, never-acker, U4E-Q11 server-side counters, carry 76 Kick path; refusal, counter, twin, bite each; nothing forged uncounted).
- B2 done. Key source facts (11fd190):
  - 11 forged kinds = marks.json -21 bites :5063-5104 (lead bound, newer tick, seq forward, move length, press edge, aim bounds, cooldown/ammo/reload, ack-sent, trailing+unknown fields, catch-up cap, wrong channel); notes :5118 (part 5's checks plus the wrong channel, 11 in all; the beyond-reach point leaves the set (clamped), the equal-or-older tick joins it).
  - -21 checks :4907-5047: forged sent_count min>=1000, count>=11, unrejected_count max==0, twin_hash_mismatch==0; clamp sent>=1000 per weapon kind, clamp counted_count sum==0, twin==0; fast-clock excess_cmds_count count>=10 and max<=0, excess_move_um max<=0; lag burst held_ns count>=100, lagswitch_excess_move_um max<=0. Report: lagswitch_cmds_count. 13 bites incl. not-older-than, refuse-not-clamp.
  - -22 :5120-5202: never-ack part, headroom min==0 (room), ack_starved.dropped count>=1; bite unbounded.
  - plan.md §4.4 :510-514: room column and server column (size limits, trailing+unknown fields, wrong channel, per-connection and new-session rate limits, ping rate and hold-time cap, ...). "Each check gets a home and a -21 counter" (:510).
  - §4.10 :360-390 (u4-build-plan); U4E-Q11 :492-495 (server checks size, trailing, wrong channel, bucket; U4f adds -21 counters + judged runs).
  - Scout map read; carry ranges: 46 :506-509, 80 :510-534, 86 :535-537, 57 :223-228, 74 :343-354, 21 :148-149, 27 :166-168, 18 :129-132, 39 :169-179, 50 :180-183, 61 :229-247, 58 :269-273, 76 :364-371, 77 :372-381.
- B3 done (carries read at their ranges). Security-relevant content:
  - 46 (:506-509): press twin removes exactly refused presses (slot, seq, kind), counter one lower; room-level hash test.
  - 80 (a) (:513-518): canonical sort wrap-aware is no total order for ticks 2^31+ apart; Rust sorts may panic on a non-total order -> a forged-tick panic (DoS) is reachable once option A runs several commands of one slot per tick; U4f's lead bound refuses first or a total key, with a test 2^31 apart, carry 21's test made to fail without the sort. (b)-(e) client side.
  - 76 (:364-371): room decides replacement and Kick(overflow) but sends old connection nothing; Decoder has no end-session call; Room::receive takes any message by slot alone -> caller's session handling is a security boundary; U4r lab sends Kick(replaced), drops Decoder and link; kicked or Leave'd session dropped; two tests (replacement; kicked session's later datagrams reach no room call).
  - 74 (:343-354): lead default 1; U4r with_lead formula; late > 1 % to the advisor.
  - 57 (:223-228) cap data, provisional 1,024; U4f measures; overflow resync counted per client. 58 (:269-273). 50 (:180-183) event older than 255 ticks never queued. 27 (:166-168) CMD_CATCHUP = REDUNDANCY_CAP. 39/18 tick-count route. 61 hs rules. 77 per-counter wraps, stale() sums.
- B4 done (draft rev 1 read whole, 837 lines; drafter notes NA-6/7/8 at netcode-architect-u4f.md:15-17). Draft map for my half:
  - 2.4 (:141-166) kinds table 1-11 + clamp row; counting rule :143-148 (by U4F-Q9); other room-column checks left to U4g/U4h/U4e :165-166.
  - 2.5 (:168-176) pump counters too_large, trailing_bytes, unknown_field, wrong_channel, input_limited; "no forged kind of the 11 is theirs" for size and bucket (:174-175); U4f changes no pump code (:175).
  - 2.6 (:178-193) cheats table; never-acker :193; forgers re-encode honest datagram :181-184.
  - 2.10 tests f7-f23 (:262-278); f21 never-acker Kick + carry 76 (:276); f23 wrong channel (:278).
  - 2.11 bites (:290-317), 17 bites; no-trailing/no-channel U4a's (:306, :308).
  - 2.12 twins (:319-331). 3 carries (:352-377); 76 -> 2.6, f21 (:371).
  - 4 netcode.md rule (:450-452). 10 Q8 lead bound (:705-713), Q9 counting (:715-724), Q11 (:735-745).
  - 12 review: security-auditor scope (:776-777).
  Next B5 (code at 11fd190: room input.rs, room.rs receive/receive_input/kick; server pump.rs; bots; lab scenario Kick routing).
