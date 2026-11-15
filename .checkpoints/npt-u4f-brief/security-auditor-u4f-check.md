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
  Next B3 (carries).
