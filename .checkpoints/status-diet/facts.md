# status-diet: facts gathered (step 1), for plan revision 1
Task: STATUS-DIET with LANE-RULES (ACCOUNTS, the Models addition, the read-list line), Mustafa's queue of 2026-11-10;
his words: main's docs/STATUS.md:674-761 (2d22b69) and docs/reports/2026-11-10-kit-v3-5.md Appendix A.

## Base
- main 2d22b69 = PR #46's merge (parents 44446a5, 0573db3), merged_at 2026-11-10T16:05:52Z, by merge commit, on
  Mustafa's go after the advisor's APPROVE WITH NOTES of 0573db365db053eeaa2637200512cfa177eb4495 (comment 6772168492).
- docs/STATUS.md at 2d22b69: 1233 lines, 177445 bytes. Blank lines: 28 140 186 196 786 947 1009 1218.
- Headings: 1 title; 29 Phase; 141 In flight; 187 Targets; 197 Decisions pending; 948 Routines; 1010 Open bugs and
  risks; 1219 Health.

## Planned destinations by line range (draft; the advisor reviews)
- DECISIONS.md (his decisions, and the Advisor's he relayed; whole blocks by header; newest first by date, same-day
  blocks keep STATUS's order so "above/below" still read right):
  198-199 (A3 notes note), 200-239 (2026-11-09/10 for the record), 240-272 (A3 answered 11-05), 281-291, 292-316,
  317-339 (11-04 x3), 340-370, 371-391, 392-400, 401-420 (11-03 x4), queue 674-690 + 727-774 (KIT-FREEZE-2, unit walk,
  Slice 1 note, pointers) with 691-726 (STATUS-DIET, LANE-RULES, carried out here) moved to the dated list under a new
  2026-11-10 header and a new pointer line after 762; 775-787 (UPGRADE-SCOPE 10-31; 786 blank), 788-799 (drafts
  brief), 813-819 (one-off night run), 820-827 (SLICE-ORDER), 828-836 (MODES-3 etc.), 837-846 (Advisor 10-31),
  880-884 (10-30), 885-887 (10-28), 888-907 (10-27), 908-911 (10-27 R7), 912-925 (10-25), 926-928 (10-14).
  Plus a script-made index of answered codes (`CODE (x)`) that stand only in the archive (not his words).
- STATUS.md (new, <= 120 lines): new dashboard text + 847-849 (Waiting: ABL-PREDICT) moved in. Sections: Phase (last
  merged #46), In flight (one line per PR: #40, #47, this PR), Next (his order), Waiting on Mustafa, Routines (read
  today), Follow-ups (ids, A4's "STATUS follow-up"; details in the archive), Open risks (pointers), where the old
  sections went (old section -> new file and heading).
- STATUS_ARCHIVE.md ("the rest"): 2-27 (old header), 30-139 (Phase), 142-185 (In flight table, all done), 188-195
  (Targets), 273-280 (advisor's A3 rulings), 421-673 + 862-879 (Follow-ups + KIT-ALIGN, DOCS-ALIGN), 800-812 (advisor
  agent 10-31), 850-861 (carry-forwards), 929-946 (ADR-001..3, old design lines), 949-1008 (Routines), 1011-1217 (Open
  bugs and risks), 1220-1233 (Health). Archive headings named after the old sections so old citations resolve.
- Replaced, not moved (structure only, no word of his): 1, 29, 141, 187, 197, 948, 1010, 1219 and the blank lines.
- After the pure-move commit: a small commit records #46 (the archive's kit-v3-5 row -> merged; STATUS; ROADMAP).

## Live facts (read 2026-11-10 ~16:30-17:20 Amsterdam)
- PR #40 netcode proof test: draft, branch claude/netcode-proof-test-plan-hcky5g, base 44446a5, head 3ac1634; its
  branch's STATUS adds a block of his 2026-11-06 answers (ROUTE (a), SAFARI-TIME (b), FIXED-IP (a), DENSITY-FIGURE (a),
  UNDER-WAY (a), branches) NOT on main: its lane moves it into DECISIONS.md when it merges main. Also edits REPO_MAP.
- PR #47 foundation 2.1a: draft, branch claude/advisor-fable-4hxq72 (his personal account), files: .checkpoints/,
  tools/gate.mjs, smoke.mjs, README.md, village.py. Its plan expects DECISIONS.md if STATUS-DIET lands first; 2.1b
  shares CLAUDE.md, STATUS, REPO_MAP, ROADMAP; merges after this PR.
- PR Advisor routine trig_01YPyWAsQmsaQoydhMGG3CkJ: enabled (updated 2026-11-10 16:07:30Z), cron
  `CRON_TZ=Europe/Amsterdam 45 7,13,19 * * *`, last run 16:07:31-16:08:59Z SUCCEEDED (cse_0173vVo6Nq6fSQQdQmHVarpM).
- Night shift trig_01QpV6y6UaG1Na2q3jumRDxh: enabled, cron NOW `CRON_TZ=Europe/Amsterdam 0 6 * * *` (daily ~06:00;
  changed 2026-11-07 00:02Z; STATUS still says hourly 00:10-07:10), last run 2026-11-10 05:04Z SUCCEEDED.
- ROADMAP: 144 lines; check its Recent work for kit-v3-5.

## Kit references to STATUS (re-point decisions for the plan)
- CLAUDE.md :27 read list (edit: his ask), :85, :107 "STATUS follow-up" (keep: STATUS keeps a Follow-ups section),
  :167 "the evidence is in STATUS, Routines" (re-point or a pointer line), :173 (times stay in STATUS Routines), :200.
- advisor.md :17, :103, :180, :263, :267, :169, :225, :286 "(STATUS, limit L4)", :289, :297: leave (pointer table);
  editing it would force main-copy advisor runs. Follow-up candidates.
- docs-keeper.md item 1 (STATUS contents) and item 5 ("Decisions pending" lines): needs the minimal edit so the diet
  holds (his STATUS-DIET words). Kit edit, asks Mustafa.
- changeset SKILL.md :15, :32, :35: still true. netcode-architect.md:24: one hop via pointers (follow-up).
- Decision docs citing STATUS (ADRs 44 cites, adr/README 12, LOOP_V2 :202 :214 :227, ENGINE_OVERVIEW:140, research):
  not edited (decision docs, dated records); the pointer table resolves them; follow-up for the drafts' next revision.
- The guard's decision-doc list already holds DECISIONS (any case): PRs touching docs/DECISIONS.md merge from a day
  session on his go. STATUS.md is not on it.

## LANE-RULES plan (his words verbatim in CLAUDE.md, quoted as his)
- New "Lanes and accounts" block: his Lanes bullet + ACCOUNTS' five bullets, word for word.
- Models: his Fable line + his addition, word for word, as the exception to "No other model".
- Read list item 3: his line "for v2 work, LOOK_V2.md is the look brief and wins; ART_DIRECTION.md describes the
  legacy game only."; ART_DIRECTION's "(locked look, authoritative)" scoped to legacy. Item 1 or 3: DECISIONS.md and
  STATUS_ARCHIVE.md.
- Risk tier 2 (CLAUDE.md, docs-keeper.md); DECISIONS.md is a decision doc. Reviews: advisor + security-auditor +
  verifier. Kit writes: one Write per file, one ask each.
DONE (step 1 facts)
