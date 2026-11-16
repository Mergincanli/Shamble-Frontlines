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
  - 11 forged kinds = marks.json -21 bites :5063-5104; notes :5118. -21 checks :4907-5047 (unrejected_count max==0 :4932-4943; twin :4944-4953; clamp counted_count :4966-4977; fast clock :4988-5023; lag switch :5024-5047). -22 :5120-5202.
  - plan.md §4.4 :510-514 ("Each check gets a home and a -21 counter"; server column incl. size, trailing/unknown fields, wrong channel, per-connection and new-session rate limits, ping rate and the hold-time cap).
  - §4.10 :360-390; U4E-Q11 wave1 :492-495 (four checks: size, trailing bytes, wrong channel, the bucket; U4f adds -21 counters and judged runs on U4e's pump).
- B3 done (carries 46 :506-509, 80 :510-534, 86 :535-537, 57 :223-228, 74 :343-354, 21 :148-149, 27 :166-168, 18 :129-132, 39 :169-179, 50 :180-183, 61 :229-247, 58 :269-273, 76 :364-371, 77 :372-381 read). 80 (a): wrap-aware sort non-total, Rust sort may panic. 76: caller's session handling is a security boundary; U4r's lab tests r8/r9.
- B4 done (draft read whole; NA-6/7/8 at netcode-architect-u4f.md:15-17).
- B5 done. Code facts at 11fd190:
  - room/src/input.rs:80-125 `offer` order: seq-forward (Repeat, uncounted as refusal) :81-88; newest_seq = c.seq :89 BEFORE press :90-106, late :107-113, full :114-116. So a command refused PressRefused/Late/Full still advances newest_seq.
  - room.rs:1322 the snapshot header carries `newest_received` = that newest_seq; client/src/lib.rs:678-686 drops every unacked command at or below it (never resent). => a refused command is acknowledged as received and never resent.
  - room.rs:744-801 receive_input: kicked -> silent return :753-755; ack_groups 0 skips the ack check :756; NeverSent -> ack_refused (+1 per datagram) and ack_refused_commands (+n) :783-787; commands offered oldest first :791-799 with counters :794-798.
  - room.rs:713-742 receive: Resync limits counted; Leave; `_ => {}` :740 (ClientSettings, SessionControl ignored, uncounted).
  - room.rs:363-369 canonical: wrap-aware compare `(b.tick.wrapping_sub(a.tick) as i32).cmp(&0).reverse()`: pairs exactly 2^31 apart compare Greater both ways; cyclic triples spanning more than 2^31 are non-transitive.
  - room.rs:318-323 kick sends Kick(overflow), roster.leave; route_events :979-1038 overflow -> resync, second overflow inside unacked forced one -> kick; encode_all :1057-1063 forced full state unacked past FULL_STATE_ACK_TICKS -> kick. Kicked ClientSlot kept (skipped everywhere).
  - lab/src/scenario.rs:21-27 session handling doc; :441-455 drop_slot; :505-508 replaced dropped first; :527-538 closed decoder/link frees slot; :557-584 send then drop kicked session; tests r8 :2117-2170, r9 :2172-2224 (carry 76 met at U4r).
  - server/src/pump.rs Counters :110-161 hold 21 counters: too_large, truncated, bad_length, unknown_type, wrong_direction, wrong_channel, trailing_bytes, unknown_field, out_of_range, malformed, before_join, input_limited, ping_limited, pong_unknown, first_not_join, joining_dropped, stream_malformed, join_deadline, wrong_version, wrong_content_hash, join_again (+ pings_sent, pongs_sent). Stream decode error other than WrongChannel/WrongDirection closes the session as malformed :414-422 (so kind 9 on a stream closes the session). Input bucket applies to Input only :449-457; Ping limited :458-471; Pong unlimited, matched against <= 8 outstanding :500-508; hold time clamped to one tick, no counter :510.
  - catalogue :88: a room->client type from a client "is refused and counted as forged" (wrong_direction); :87 Input on a stream = wrong channel, counted as forged; :92 hold time counted up to one tick; :96 ClientSettings clamped (U4g).
  - proto quant.rs:34-42 BITES hold no-trailing and no-channel; strict() covers trailing bytes and reserved/pad bits :115-122; checks_channel :124-131; msg.rs:420. Lab applies a proto id to room, client and every decoder (scenario.rs:212-237, :261-270); stream_of :140-150 still works under no-channel because Control (2 B) and FullState (4 B) framings differ (msg.rs:220-226): checked, no finding.
  - sim: world.rs:1401-1420 refusals Reload, Ammunition, Cooldown with (slot, seq, kind) via refuse :1446-1452; seam.rs:507-520 RefusalKind; clamp world.rs:1285-1312 (per command, from the current origin; report (slot, seq, point)). World::BITES world.rs:937-938 holds 4 ids, none -21's (NA-9 holds); grep of -21 ids in crates: none.
  - bots/src/lib.rs:74-76 FAR_CM 32,000; greybox.rs:62 HALF_MAP 12,800, :151 the extent rect (NA-8 holds).
  - SERIES.md:158-159 forged_kind sent_count and unrejected_count = "sent minus rejected-and-counted" (a subtraction); :155-156 fast clock; :34, :33 lag switch; :21, :128 -22.
  - ADR 0001:231-239 part 5's checks; :247-250 hold time; :1611 -21 row (1.5x clock "runs at most the server's ticks ... plus the lead bound").
  Next B6 (threat list) then B7 (findings).
