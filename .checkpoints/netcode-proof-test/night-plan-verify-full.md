# Verifier: the references of night-2026-11-16.md (workflow wf_6e5acbfd-5c3), 2026-11-15

Written by script, word for word, from the workflow's journal. Nothing below is the main session's. Its notes: night-plan-verify.md.

- **Verdict:** PASS
- **References checked:** 105

## Summary

VERDICT: PASS, with 5 Low findings. Each one is a wrong or partial cite; none sends the night to the wrong text for its work.

Gate checks done:
- The pinned commit 11fd190e95 exists and is on origin/claude/netcode-proof-test-plan-hcky5g.
- All 47 paths named in the plan or the scout map exist at 11fd190.
- After 11fd190 the card changed only in its LATEST status lines, so the header's claim that nothing the item reads changed holds.
- About 80 references in the plan were checked, all opened at 11fd190: the card's verbatim 2026-11-09 words (equal to the plan's quote), NIGHT_SHIFT step 4, CHECKPOINTS §4 and §8, the u4-build-plan ranges, the advisor-u4 conditions, the ADR 0001 rows and Time paragraph, the marks.json ranges (-21 has 8 parts and 13 bites), F104 (defined only at u4-scout-netcode.md:122), U4R-Q14, the 17 carries by number, history.md's SIZE-CHECK and 2026-11-15 rules, the card's Rules in force, M6/TE-Q2, DEPTH and the option-A note, check.mjs's 11 steps, and build2 point 31. All hold except those listed below.

Wrong or partial cites, with corrections:
1. Build rules: §7 (u4-build-plan.md:909-932) has no 'offline' and no df floor. Those are at u4r-brief.md:940 and :968-971.
2. 'Reported met by number' is not at u4s-carries.md:9-14. It is at advisor-u4.md:334-335.
3. The Write/Edit sentence is the U4d-F ruling's R6 (advisor-u4df.md:317-318, text at :461-464), not R1 (R1 is NF-M1). The words themselves are right.
4. Mustafa's 2026-11-15 read rule names marks.json and the judge only. The SERIES.md read is the advisor's recommendation (advisor-c62h-review-full.md:93).
5. In the scout map, advisor-u4r-r1-full.md:61 is NC-U4R-1, not NC-U4R-5. The correct ranges are :78-81 for NC-U4R-5 and :200-204 for EA-U4R-7.

Scout spot-check: I checked 25 entries across Sections, Carries, Rulings, Open items and the code lane. 23 hold; 2 are partly wrong (items 2 and 5 above). One more is one line off and harmless: U4E-Q11 is at :492-495, not :491.

Not verified: issue #16's night-shift comments and PR #40, because they are on GitHub and this check allowed no web access.

Notes: /home/user/Shamble-Frontlines/.checkpoints/netcode-proof-test/night-plan-verify.md

## Wrong references

- **Where in the plan:** Item 7, The brief's shape, item 4 (Build rules)
  - **Cited:** §7 (`N/u4-build-plan.md:909-932`: the shared target, offline, touch, counts as deltas, the df floor)
  - **Finding:** Low. §7 has the shared target (:911-912), touch (:913) and counts as deltas (:914-915), but it has no 'offline' and no df floor. The night will look for those two rules in §7 and not find them.
  - **Correct:** Keep `N/u4-build-plan.md:909-932` and add U4r's brief §4: CARGO_NET_OFFLINE at `N/u4r-brief.md:940`, the disk floors at `N/u4r-brief.md:968-971` (4 GB before wasm32/release builds, 2 GB otherwise; the runner's FLOOR is at spikes/netcode-proof/ops/runs/c-u4r-runs.mjs:131-132).
- **Where in the plan:** Item 7, The brief's shape, item 3 (The carries)
  - **Cited:** Each is to be reported met by number, or not met with why (`N/u4s-carries.md:9-14`)
  - **Finding:** Low. u4s-carries.md:9-14 holds the rule that briefs carry entries word for word (:9-10, :14). The rule that each report says met or not met, by number, is not there. The scout's Carries 'header' entry has the same mis-cite.
  - **Correct:** `N/advisor-u4.md:334-335` (the advisor's U4s ruling, condition 4: 'Each wave-1 report lists the carries it met, by number'). U4r's wording of it is at `N/u4r-brief.md:695-696`. Keep `N/u4s-carries.md:9-10` for the word-for-word rule.
- **Where in the plan:** Rules for the item, rule 4 (Agents)
  - **Cited:** the advisor's U4d-F ruling, R1
  - **Finding:** Low. In advisor-u4df.md, R1 (:28) is NF-M1, the ES level, which has nothing to do with this sentence. The Write/Edit sentence comes from R6 (:301-320, ruling at :317-318), and its text is in T4 item 1 (:461-464). On the card, at 11fd190 :3080-3083, the sentence is item 1 of 'U4d-F ruling (advisor-u4df.md, R1, R4.1 and R6)'. The plan quotes the sentence word for word correctly.
  - **Correct:** the advisor's U4d-F ruling, R6 (`N/advisor-u4df.md:317-318`; text T4 item 1, `:461-464`; card 'Rules in force')
- **Where in the plan:** Item 7, The brief's shape, item 5 (closing paragraph)
  - **Cited:** If U4f changes marks.json, SERIES.md or the judge, ... the reviewer's read that his rule of 2026-11-15 asks for (`N/history.md:2113-2129`; `N/advisor-c62h-review-full.md:89`)
  - **Finding:** Low (wording). Both cites are right. But Mustafa's words at history.md:2127 name only marks.json and the judge. A SERIES.md read is the advisor's recommendation, 'though his words do not name them'. As written, a brief built from this line could record the SERIES.md read as his rule.
  - **Correct:** Keep the read rule for marks.json and the judge, citing `N/history.md:2127`. Cite the SERIES.md read to the advisor at `N/advisor-c62h-review-full.md:93`.
- **Where in the plan:** Read first: N/night-u4f-scout-full.md, Rulings and Open items entries for NC-U4R-5 / EA-U4R-7 (the plan tells the night to read every Rulings entry)
  - **Cited:** N/advisor-u4r-r1-full.md · lines: 61, 80
  - **Finding:** Low. Line :80 is NC-U4R-5's 'Where it goes', which is right. Line :61 is NC-U4R-1's 'Why' (the closed-session glue), not NC-U4R-5 or EA-U4R-7. The plan itself names these findings by id only.
  - **Correct:** NC-U4R-5 is at `N/advisor-u4r-r1-full.md:78-81`; EA-U4R-7 is at `:200-204`.

## Spot checks of the scout's map

- **Sections: N/u4-build-plan.md:409-425 (§4.12; -20 :422, -21 :423, -22 exception :424)** · **holds:** yes · The table rows are at :422-424, and -22 has 'no' precondition with its reason.
- **Sections: N/u4-build-plan.md:453-476 (§4.14 bites :462, :463, :467, :475)** · **holds:** yes · -21/not-older-than :462, -21/no-catchup-cap :463, -48/no-buffer-wait reading -20's depth :467, -22/unbounded :475.
- **Sections: N/plan.md:617-628 (SimClock; rate 1.5 :625)** · **holds:** yes · :625 'rate 1.5 for -21's cheat'.
- **Sections: docs/design/netcode-proof-messages.md:87 and :99 (Input and Kick rows)** · **holds:** yes · :87 has the bucket refilled at 2 x TICK_HZ. :99 lists the Kick reasons (replaced, malformed, overflow, stopped). The words 'never for a forged kind' are not in :99; they are stated at u4-build-plan.md:383-384.
- **Sections: spikes/netcode-proof/marks/SERIES.md:21, 93, 128, 442 (-22 rows)** · **holds:** yes · ack_starved_headroom_count, rss_bytes, ack_starved.dropped and event_queue_cap_count are all at the stated lines.
- **Sections: docs/adr/0001-multiplayer-netcode.md:226-255 (Part 5; lead bound :231-233, catch-up cap :238-239, Time :239-251)** · **holds:** yes · Holds.
- **Carries: 80 (a)-(e), N/u4s-carries.md:510-534** · **holds:** yes · (a) :513-518, (b) :518-520, (c) :520-522, (d) :522-529, (e) :529-534. The texts match the scout.
- **Carries: 74, N/u4s-carries.md:343-354** · **holds:** yes · Default lead 1; U4f's controller comes after U4r; the U4R-Q14 formula and the 1 % late rule.
- **Carries: 76, N/u4s-carries.md:364-371** · **holds:** yes · Session-ending glue: Kick(overflow) and replaced sessions reach no room call.
- **Carries: header, N/u4s-carries.md:9-14** · **holds:** NO · The word-for-word rule is there. 'Reports list carries met by number' is not: it is at advisor-u4.md:334-335.
- **Rulings: JDG-ONLY-1, N/advisor-u4r-c62h-full.md:8-20, :44-45** · **holds:** yes · The reach naming -20 and -21 is at :19, the home at :20, and the bite run listing PM-0001-2 at :45.
- **Rulings: EVQ-LOAD, N/advisor-brawl1-sets-p3-full.md:156-158** · **holds:** yes · The carry line (460-560 events per second at P3, cap 1,024) is at :158.
- **Rulings: F2 U4-TICKROUTE, N/advisor-u4.md:436-465 (point 6 :463-465)** · **holds:** yes · :463-465 let U4f's brief give carry 18's places; the first case is U4f's lead bound.
- **Rulings: U4E-Q12, N/advisor-u4e-brief.md:217-244 and :340-341** · **holds:** yes · Holds.
- **Rulings: U4E-Q11, N/advisor-u4-wave1.md:491-495** · **holds:** yes · The ruling is at :492-495; :491 is the end of U4E-Q10, one line early. Harmless.
- **Rulings: NC-U4R-5 / EA-U4R-7, N/advisor-u4r-r1-full.md:61, :80** · **holds:** NO · :80 is right. :61 is NC-U4R-1's Why. The correct ranges are NC-U4R-5 :78-81 and EA-U4R-7 :200-204.
- **Open items: TE-B1-R2-1 with RR2-B1-1 (u4r-report.md:1137-1141; card :163-171; bots lib.rs:130-132, :404)** · **holds:** yes · All hold at 11fd190.
- **Open items: RR-B1-7 (1) (u4r-report.md:1162; logs.rs:495)** · **holds:** yes · room_tick is at logs.rs:495. The report's own :402-405 is stale; the scout's line is right.
- **Open items: C62H-BOUND (advisor-c62h-review-full.md:59, :63, :75-81, :132)** · **holds:** yes · Holds.
- **Code lane: crates/room/src/input.rs (doc :1-5, CMD_CATCHUP :61, buffer_depth_ticks :64, offer :80, take :129)** · **holds:** yes · The doc names option A and the other checks as U4f's.
- **Code lane: crates/client/src/lib.rs (lead :214/:267, with_lead :296-301, input :414, stamp :429, step :450, receive :492, too_old :528, see :583, file :645, full_state :810)** · **holds:** yes · All anchors hold.
- **Code lane: crates/server/src/pump.rs (Counters :112, Decoder :204, pump :372, pong :500-517)** · **holds:** yes · Holds.
- **Code lane: crates/room/src/room.rs (canonical :361-369, event_cap PROVISIONAL :460, carry 21 test :1402)** · **holds:** yes · Holds. A second carry 21 test is at tests/room.rs:235.
- **Code lane: crates/lab/src/scenario.rs (1.5 rate :18-19, lead :126, with_lead :848, dispatch :851, suite :1405)** · **holds:** yes · Holds.
- **Code lane: ops/runs/c-u4r-r2.mjs SUITES :72-92 (-21 stand-ins :82-85); crates/room/src/events.rs EventQueue :26, PROVISIONAL_CAP :40** · **holds:** yes · Holds.
