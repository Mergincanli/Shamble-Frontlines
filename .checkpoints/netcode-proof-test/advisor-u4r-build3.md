# Advisor: U4r build, copy 3's eight points (2026-11-14)

Notes file for the advisor's ruling on u4r-build3-questions.md. Plan first, then a line per item as it is done.
On a restart: read this file, continue after the last item marked done.

## Plan
1. Read sources: questions (done), builder notes :571-906, the brief's relevant sections (2.4 r2, 7.1, conditions L1-L6,
   lanes), carries 49/57-62/73/77/78, the U4 plan §4.5/4.12/4.14, rulings 1 and 2, history.md (Mustafa's words on
   PM-0001-2, review panels, Low never triggers push, LEFTOVER-DELETE, condition 5).
2. Check key code/log claims myself per point (spot-check, read-only).
3. Rule on each point 1-8 and write the ruling here, a line per item.
4. Return the structured result.

## Items
- [done] Read: advisor.md (main), questions, notes :571-906, ruling 2, brief 2.4 r1-r10 and 2.5-2.7, carries 40-78,
  history.md :280-420 (Mustafa: "The 9.5 floor (PM-0001-2) does not move." :338-339; "A Low never triggers a push."
  :340; "The review panels stay as they are until I decide." :310; LEFTOVER-DELETE :404-406). pre:PM-0001-2 on -17/-18
  is the advisor's Q22 ruling (plan §4.12 :416-424), not Mustafa's. ADR 0001:1589 (PM-0001-2: "every client's relevant
  set holds all 47 other soldiers on at least 99 % of its snapshots ... proposed share"; downstream "every run of
  PM-0001-4 to PM-0001-27"). SERIES.md:368 flag's writer "npt-room"; U4r's lab computes it from the mirror frame
  (logs.rs:538-541; mirror.rs:1-8, "the frame the room meant that client to hold").
- [done] Restart (copy 2 of this advisor, fresh context): re-read advisor.md, questions, notes :571-906, ruling 2
  whole, brief 2.4 and 7.1 and section 4's lanes, carries 40-78, history.md :281-345, :364-415, :1928-2027 (also
  SIZE-CHECK :398-399, USAGE-SAVINGS :291-299).
- [done] Code/doc checks for point 1: mirror.rs:1-8, :314-337 (flag = the mirror frame holds every other soldier; ES
  and vehicles), :396-414 (a missing baseline frame is a breach and returns early: that tick's stats not updated);
  room.rs:1163-1170 (baseline, stale under no-fallback, else defaults age 0), :1208-1235 (events oldest first before
  any entity change), :1236 on (nearest 8 then the rest). ADR 0001 is "Status: Proposed" (:3); :1589 scopes the share
  "for the downstream rows", last column "every run of PM-0001-4 to PM-0001-27"; :705 the downstream estimate
  "everyone in view". marks.json: -17 and -18 L1 parts validity [pre:PM-0001-2] (notes: "the advisor's Q22 ruling");
  -57 parts none; -58 window all. Plan §4.12 :416-424 (Q22: -17 "the codec under the real load", -18 "B1 ... inside the
  band"), §4.5 :296-300, §4.14 :461 (-18 bite "a gap evicts B"), §5 U4g :818-833, order :882 (U4r, then U4f, U4g,
  U4h); RQ1 :114 is U4-wide ("bursts (-18)"). Plan §10/§11 :970-976, :1080-1093: the readings list has Q22's -22
  exception only, not Q22's share on -17/-18. Carry 78 (u4s-carries.md:546-553): "so no U4e or U4r run meets it".
- Point 1 draft: High (npt-room's recovery, not U4r's code). Q22 stands, no bar moves; the FAIL stands (condition 5);
  B1 judged runs stay in U4r's set for -57 (no pre), -17/-18 B1 recorded INVALID, never counted; their passing evidence
  moves to U4g with carry 78 widened (relevant soldiers the baseline lacks first; form is U4g's plan; target share
  >= 0.99 at 10 bursts in 600 s = at most 18 flag-0 ticks per client per burst on average). Readings PR gains Q22's
  share on -17/-18 (ADR's "for the downstream rows"). Not to Mustafa now.
- Next: spot-check b1 logs; point 2 (ES owner of -12 runs; brawl aim); point 3 (figures.rs orphans; tick16c segments);
  point 4 (scenario.rs heal record; b1-nofallback breaches); points 5-8.
