# Task: test-engineer review of the v2 decision drafts (plan step 10, wave 2), at commit 47eadd9 on claude/laughing-lovelace-ox61se; scope "What a spike must prove" sections + SPIKE_PLAN; skip own pass-mark tables and look checklist (condition 15)

- 2026-11-02 start: docs in scope unchanged between 47eadd9 and HEAD (only docs/reports changed). Reading inputs next.
- Read: prompt, brief, plan settled inputs + rules, advisor conditions, SPIKE_PLAN (393 lines), ADR 0001 decision parts 1-14, spike section 935-1017, OQs, failure modes, migration, test plan; ADR 0002 spike section 914-976, parts 8-9 (tests, gate v2); ADR 0003 spike section 820-907.
- Candidate findings so far:
  - F-a ADR 0001:560 failure-mode row "A tick counter wraps ... the 60 Hz margin run's 30-minute soak crosses 65,536 ticks": wrap proof rests on a perf run that may be deferred (OQ-0001-2 (c)) and the 30 Hz soak (54,000 ticks) never crosses; needs a deterministic start-offset test. (check line no.)
  - F-b ADR 0003:845 "effects on one backend only are off for this test" vs part 1 F-A (same features both backends, line 125): the parity check excludes exactly the failure F-A forbids. PM-0003-3 (mine, :980) repeats it.
  - F-c ADR 0001 OQ-0001-2 (:868-869) "The 30 Hz baseline marks may use a stand-in": PM-0001-27 rooms per VM / ADR 0006 density measured on stand-in sim -> optimistic; re-proved slice 6 (migration :1303). Check ADR 0006 use of density figure.
  - F-d ADR 0002 part 8 soak "measured ... against the server tick budget" nightly - where does it run? shared runners vs "timings on shared CI runners are not hardware results" (:~440). Check ADR 0006.
- Next: ADR 0004 spike section, ADR 0005, ADR 0006, ENGINE_OVERVIEW, README.
