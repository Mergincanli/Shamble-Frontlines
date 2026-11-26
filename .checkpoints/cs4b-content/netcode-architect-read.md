# netcode-architect: short read for change-set 4b (advisor re-read R3, R4)

Asked 2026-11-26 by the main session. Read-only. No cargo, no build.

## Plan
1. Read advisor-reread.md (R3, R4, ruling F.1) and plan.md:680-720 (DEPART-4).
2. Q1: find the three limits in crates/room and crates/protocol; read the build plan for change-set 7 and Slice 3; ADR 0001 PM rows.
3. Q2: read crates/devtools/src/recording.rs (header, VERSION, content hash field), files.rs, replay.rs, make_recordings,
   tests; Node and Chromium replay; STATUS_ARCHIVE "From change-set 4a"; my cs4 design block 1.
4. Q3: messages.rs:137-144 AimPoint; ADR 0001 PM-0001-51; aim clamp in my design.
5. Answers, one line each below as reached; DONE.

## Answers (a line each as reached)
- [Q1] Three limits = REDUNDANCY_CAP 8 (protocol/src/messages.rs:16-18), LEAD_MAX_TICKS 8 (room/src/lib.rs:36-37),
  CATCH_UP_CAP 2 (room/src/lib.rs:39-40); source: my cs3 design item 7 (80a51e42 netcode-architect-cs3.md:486-487),
  DEPART-3 in cs3-plan (80a51e42 :311-313). All three are also capacities: codec MAX_INPUT_BYTES + decoder bound
  (codec.rs:42,46,305), endpoint loop and SENT_RING assert (endpoint.rs:34,250), room's fixed array (lib.rs:260).
  ADR 0001:239 ties two: catch-up cap <= redundancy cap. CS7 = room's scheduler behind one seam, both timings
  (build plan :389-391, :920-922); repeat cap and ring sizes "data" in Part 6 table (:916). CONFIRM cs7, with the
  rule: data value <= the code's capacity, loader checks catch-up <= repeats; capacities stay code (the codec's
  widths and U2 at PM-0001-51, Slice 3, may move the capacity, not the data).
- [Q1 note] DEPART-3's "(line 221)" covers the repeat cap only ("data, proposed 8"); the other two are ADR 0001:231
  ("every limit is data") and :238-239 (catch-up cap). Low, wording. When they move (cs7), defaults = today's numbers,
  so 4b's frozen bundle (which lacks them) keeps replaying (additive rule, EA 1.3 stage 4).
- [Q2] Route (i) + the header's EXISTING field: 4a writes 32 zero bytes (recording.rs:7, scripts.rs:115-116; refused
  non-zero by replay.rs:50-51 and recording.ts:137); 4b writes SHA-256 of the frozen bundle's exact bytes (= ADR 0005
  CD-8's bundle hash, 0005:431-441) and the map id; VERSION stays 1 (premise of (ii) false: the field exists).
  Frozen file `<name>.content.json` flat in recordings/ (read_dir is flat, files.rs:124-134); an INPUT of made()/check
  (never regenerated; only `make_recordings --freeze <name>` writes it); manifest lists it. Runner checks sha256 before
  Sim::new; Node reads + node:crypto; Chromium spec reads + WebCrypto, posts bytes to the test Worker. Test module
  replay_hashes/replay_state_at take the bundle. Proof vs `git show 2483e9a:` copies: lines 2..1801, vector.txt, the
  122,400 step bytes equal; plant (content byte in shared state at construction) fails at step 1 on all three; never
  run make_recordings with the plant.
- [Q3] Agree F.1, sharpened: none of Slice 1's twelve brings such a map (test scene = street corner + yard, build plan
  :250, :347-348; 2 km PM-0004-39 sim-only); the codec's widths are the proof test's (PM-0001-51 is ADR 0001's row
  :1648; build plan :1064 its own track) and land before online play = Slice 3 (messages.rs:2-6). Reach: symmetric
  +-327.67 m (|cm| <= 32767): i16::MIN has no negation (room/protocol deny arithmetic_side_effects), matches DEPART-4
  entry 6 text (plan.md:702-703). y: 0..655.35 m (u16) -> no shape below 0. Not strictly inside: bounds on whole cm,
  client quantises (saturating) then clamps in integer cm; room checks the same integers (design 3.2 :335-339, 3.7 :456).
DONE
