Task: night item 7 (npt-u4f-brief), 2026-11-16: netcode-architect drafts rev 1 of U4f's build brief (clock and commands) into .checkpoints/npt-u4f-brief/u4f-brief-draft.md; these are its notes.

Authority: the approved plan at 781212774986118183a7c6521c34cca31b5fe2a0:.checkpoints/netcode-proof-test/night-2026-11-16.md ("Rules for the item" 1-7, "Item 7"). Inputs read at 11fd190 in /home/user/Shamble-Frontlines/.gate/npt-spike-1116 (read only). N/ = that worktree's .checkpoints/netcode-proof-test/.

## Plan (blocks, in order)
B1. Read the plan (done at start).
B2. Read N/night-u4f-scout-full.md (the map).
B3. Read N/u4-build-plan.md sections the scout lists.
B4. Read N/u4s-carries.md entries (line ranges for placeholders).
B5. Read rulings the scout lists (5a, 5b).
B6. Read the card (N/../netcode-proof-test.md) lines the scout lists.
B7. Read ADR 0001 rows, marks.json blocks, marks README, N/plan.md, N/u4-scout-netcode.md lines (7a done; 7b: README, plan.md, catalogue).
B8. Read N/u4r-brief.md rev 3.2 (form).
B9. Read the code lane at 11fd190.
B10-B22. Write draft sections 0-12 (one block per section, saving the draft whole after each).
B23. Final counts and DONE.

## Findings (one line each: id, severity, file:line, scenario)
- NA-1 Low: N/u4s-carries.md:535-537 carry 86 has no quoted text (every other entry quotes "the text the brief must carry" per :9-10), so the fill script cannot tell where the label ends and the carried rule begins; proof: the fill script asserts one quoted span per entry and reports 86 as the exception.
- NA-2 Low: spikes/netcode-proof/marks/SERIES.md:93 names npt-server as `second.v.rss_bytes`'s writer, but on L1 the room log is npt-lab's process, so -22's report line (marks.json:5182-5190) has no named L1 writer; proof: grep rss_bytes under crates/ at 11fd190.
- NA-3 Medium: N/u4-build-plan.md:335 sizes the event cap as rate x (B1's 1 s + RTT + margin); with EVQ-LOAD's full-view rate (460-560/s, N/advisor-brawl1-sets-p3-full.md:158) that gives about 560 x 1.3 = 730 to 1,008 for margins up to 0.7 s, at or below the provisional 1,024 that already overflowed inside BRAWL-1's P3 measured window (s1: 27 resyncs, no burst), because the queue drains at the 1,000 B window's pace (mean 999.2 B), not at the RTT's; a cap set by the formula resyncs clients in -17/-18/-20/-21 judged runs (carry 58's stop); proof: the day session reads BRAWL-1 P3 s1's per-client queue high-water marks against 560 x (1 s + RTT) and the cap.

## Key facts (kept as I read)
- Scout map: N/night-u4f-scout-full.md (HEAD 0ad41a6 when it ran; line numbers hold at 11fd190 except the card).
- Scout Gaps: no U4f file lane in plan; no run size/time figures for -47 1,000 connects, -21 suites+twins+bites, option A, never-acker; "window" is §4.5 not §4.7; F104 only at u4-scout-netcode.md:122; no U4F-Q ids exist yet.
- Disk (scout): 7,689 MiB free; 4 GiB floor; 56 MB per 10-min run without -57; judged run 18,335 ticks (355.4 s longest); RUN_LIMIT_MS 3,600 s; judge memory 302.2 + 2.3403 x log MiB; D4 slowest tps SS P0 69.6, P2 51.6, P3 54.7, B1 58.4, ES P3 148.0.
- Plan u4-build-plan.md: :805-816 U4f row; §4.1 :193-197 (ClockSync not in §4.1); §4.3 :252-261; §4.4 :277-290; §4.7 :330-336; §4.8 :338-341; §4.9 :351-356; §4.10 :360-390; §4.11 :392-407; §4.12 :422-424; §4.13 :442-447; §4.14 :462, :463, :467, :475; §5 :507-512; U4e :719, :722-724, :744; U4r :766; Lanes :874-883, :886-887; §7.1 :909-922; §7.2 :925-931; §9 :958-968; Q16 :1042-1045; Q21 :1054-1056; Q22 :1057-1060.
- Carries, N/u4s-carries.md: header :9-10; 18 :129-132; 21 :148-149; 27 :166-168; 39 :169-179; 50 :180-183; 57 :223-228; 58 :269-273; 61 :229-247; 62 (h) :305-321; 63 :322-327; 64 :448-473; 67 :476-493; 74 :343-354; 76 :364-371; 77 :372-381; 46 :506-509; 80 :510-534 ((a) :513-518, (b) :518-520, (c) :520-522, (d) :522-529, (e) :529-534); 86 :535-537.
- Rulings:
  - advisor-u4.md: cond 1 :78-89 (amended :477-479); cond 4 :112-118; cond 5 :119-124; cond 10 :143-149 ((c) :147); cond 11 :150-153; cond 12 :154-158; order :186-187; F2 point (6) :463-465; :334-335 report lists carries by number.
  - advisor-u4-wave1.md: W1-U4D N-M2 :197-239 (press twin :232-235; reading :236-239); U4E-Q6 :473-479; U4E-Q11 :492-495.
  - advisor-u4e-brief.md: U4E-Q12 :217-244; :340-341.
  - advisor-u4e-r1.md: NC-M6 :202-205; A6 :206-210; RR-5 :211-217.
  - advisor-u4r-brief.md: A3 :133-141; A4 :143-147; U4R-Q2 :189-196; U4R-Q4 :198-205; U4R-Q14 :289-358; lane model :360-375; panel :377-381; L1-L6 :416-433.
  - advisor-u4r-r1-full.md: NC-U4R-1 :58-61 (closed session never leaves room; U4f's hostile clients run on this glue; fixed in U4r FP-1); NC-U4R-5 :78-81; T-6 :119-123 (five doctored contract cases: carry 60's missing header param, missing class_audit event, duplicate admit, log npt-log refuses exit 1, usage error exit 2); EA-U4R-4 :182-186 (clamp bound pub npt-sim item if crates/sim opened); EA-U4R-7 :200-204.
  - advisor-u4r-c62h-full.md: JDG-ONLY-1 :18-20 (reach names -20, -21 :19; home :20; interim rule :20); bite runs list PM-0001-2 and own mark :44-46.
  - advisor-c62h-review-full.md: ADV-C62H-2 :59; PB-1 :63; C62H-KEEP :68-73; C62H-BOUND :75-80; units ahead :82-94 (U4f item 4 :89; SERIES/README read recommended :93).
  - advisor-te-m3.md: TE-Q2 :119-128 (after-clause :127-128: FX-10's saved output stays; a failing test is still a defect until its cause is known).
  - advisor-u4df.md: R6 :317-320; T4 text :461-468 (item 1 Write/Edit sentence :462-464; item 2 TE2-5 stop clause :465-468).
  - advisor-u4r-build2-full.md: point 31 :24-29 (main session line :28).
  - advisor-brawl1-sets-p3-full.md: EVQ-LOAD :156-158 (text :158).
- Card (11fd190 numbering, found by text): LATEST :17-43; NEXT :507-523; JDG-ONLY-1 :1467-1474; M6 TE2-5 ends :1553; DEPTH :1930-1933; follow-ups :1960-1964 (option-A note :1962-1964); copy 2 DEPTH :1982-1987; Rules in force :3013-3088 (read rule :3015-3021; KEPT-LOGS :3022-3027; SIZE-CHECK :3028-3035; TE-M3 :3040-3042; USAGE-SAVINGS :3050-3059; U4 line :3070-3072 (W1-TARGET flock); U4j r2 TE2-5 :3073-3079; U4d-F :3080-3088).
- ADR 0001: Part 5 checks :231-239; Time :239-251; OQ-0001-6 :1156-1167; -20 :1610; -21 :1611; -22 :1612; -47 :1644.
- marks.json: -20 :4618-4778; -21 :4779-5119 (13 bites :5061-5113; notes :5118); -22 :5120-5202; -47 :9046-9109 (bite ids PM-0001-47/newest-sample, /tick-quantised). Twins 12 = forged A/B + clamp A/B x 3. Bites at U4f: 17 (-20 1, -21 13, -22 1, -47 2). Option for 10 no-* bites unnamed -> question.
- SERIES.md: :21, :33-34, :40-42, :61, :63, :93, :101, :128, :144-145, :149, :155-156, :158-159, :170, :240, :245, :322, :342, :344, :412, :442.
- Scout-netcode: F104 :122; F117 :135; F59 :77; F79 :97; F108 :126; F119 :137; F121 :139.

## Block log
- B1-B4 done. B5a, B5b done. B6 done (card). B7a done. Next: B7b (README 187-192, 247-290, 317-330; plan.md 456-525, 553-628; catalogue 52-123), then B8, B9.
