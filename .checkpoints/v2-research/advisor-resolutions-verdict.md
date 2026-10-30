# Step 3b: the advisor's verdict on the resolutions, the glossary and the cross-topic open questions (verbatim)

The advisor agent, workflow run wf_d38cd72c-feb (prompt `prompts/advisor-resolutions-review.txt`), returned 2026-10-30 about 18:14 UTC. Notes: `advisor-resolutions-review.md`.

**Step 3b verdict: 4 APPROVE, 4 REVISE, 4 ESCALATE.** I approve the tick-budget choice (one server budget of 50 % of the tick period, split 25 % for the sim and 25 % for the server-only work). But resolution 1's claim that every topic's marks fit inside it is wrong at 60 Hz, and I give the corrected numbers below. Engine-architect's question: yes, with a rule.

**TL;DR**
- 25 % for the whole tick still fails worse than 50 % at every size. So 50 % stands, now with honest tight spots at 60 Hz.
- Two fixes run through several items. Previews carrying unreleased art stay behind the login (access wall), as R5 already says. Server marks need a snapshot-receiving client on every soldier, or encoding is never measured.
- The four escalated items keep interim wording that holds under every answer. Their decisions are still open (below).
- No resolution decides the look, a design rule, or anything that is Mustafa's or the Advisor's.

**What I checked myself**
- **Quotes:** my own script finds all 116 "before" quotes exactly once each, and each sits within 3 lines of its stated line. The topic files have not changed since 8dee746 (empty git diff).
- **Arithmetic:** I redid resolution 1's sums, the N = 54 scaling, the 25 % option, and resolution 9's sums (draw calls 786 and 834, enemy pairs 768 and 972).
- **Sources in the topic files:** the 700 shots × 20 µs mark (R2-SYNC-REC5), 0.2 and 0.5 ms (R2-INT-REC1, REC4), R6's cost rows 0.0173 and 0.0347 EUR, R6-HOST-F23's "1 core, 2 vCPU", R6-HOST-F6, R5-GATE-REC1, R5-RUSTLINT-F9, R1-BND-REC5, VISION_V2 §3, LOOP_V2 §2.
- **The other 29 items:** 72 insertions, R1 18 / R2 15 / R3 8 / R4 8 / R5 14 / R6 9, recounted from the critic's JSON. Six texts compared word for word with the critic's.
- **Not checked:** any source page (nothing fetched, as instructed). No gate run: nothing was edited.

## Per contradiction (by the critic's rank)

**1. What the tick budget covers: REVISE.** The decision stands (50 % total, 25 % sim step share, 25 % server-only share; the 60-or-30 Hz rule unchanged). Required changes:
- **a. Missing rows in the arithmetic table.**
  - Sim step share: avoidance, ≤ 0.2 ms p99 (R4-NAV-REC3, r4:860).
  - Server-only share: the enclosure pair test, 0.1 µs × 768 enemy pairs = 0.08 ms (R2-INT-REC2 (b)).
  - Server-only share, on a tick with a portal change: the re-flood, +1.00 ms (R2-INT-REC2 (b)).
  - Mark the physics row "mean mark; its p99 is reported, not capped".
- **b. New sums at N = 48.**

  | | 60 Hz | 30 Hz |
  |---|---|---|
  | Sim step share | 3.87 ms (limit 4.17) | 5.53 ms (limit 8.33) |
  | Server-only share | 3.41 ms (limit 4.17) | 3.64 ms (limit 8.33) |
  | Server-only, re-flood tick | 4.41 ms | 4.64 ms |
  | Server tick | 7.28 ms (limit 8.33) | 9.18 ms (limit 16.67) |
  | Server tick, re-flood tick | 8.28 ms | 10.18 ms |

  Left at 60 Hz: 0.30 ms in the sim step share, 0.76 ms in the server-only share (−0.24 on a re-flood tick).
- **c. New sums at N = 54** (the same 1.125 scaling, 972 pairs).

  | | 60 Hz | 30 Hz |
  |---|---|---|
  | Sim step share | 4.14 ms (left 0.03) | 5.81 ms |
  | Server-only share | 3.85 ms | 4.12 ms |
  | Server-only, re-flood tick | 4.85 ms | 5.12 ms |
  | Server tick | 7.99 ms | 9.93 ms |
  | Server tick, re-flood tick | 8.99 ms (over 8.33) | 10.93 ms |

- **d. The 25 % option, same rows:** at 60 Hz 3.03 + 3.41 = 6.44 ms (5.24 ms with encoding spread over two ticks), against 4.17. At 30 Hz 7.51 ms, or 8.51 ms with a re-flood, against 8.33.
- **e. "Why 50 %" point 1, and the "holds under either reading" sentence**, replaced by: "At 30 Hz the topics' marks fit inside it at N = 48 and N = 54, re-flood ticks included. At 60 Hz they fit on every tick without a re-flood; on a tick that encodes a snapshot and re-floods after a portal change, the server-only share's ceilings (4.41 ms at N = 48) pass its 4.17 ms and, at N = 54, the whole (8.99 ms) passes 8.33 ms; at N = 54 the sim step share keeps 0.03 ms for the rest of the sim. Under 25 % for the whole tick the marks do not fit at 60 Hz on any snapshot tick, nor at 30 Hz on a re-flood tick, which would decide the tick rate before the spike the rule waits for. The 60 Hz tight spots (the visibility pass, the re-flood tick, the rest of the sim at N = 54) are for the spike to measure."
- **f. The claim that adding p99s gives an upper bound is false.** It appears in resolution 1 and in the glossary's "Time statistics". Replace both with: "Adding the parts' p99 ceilings is a planning check that the marks fit together, not a bound: p99s do not add (two parts that each take 1 ms on 0.6 % of ticks and nothing otherwise each have a p99 of 0, but together they take 1 ms on about 1.2 % of ticks, so the sum's p99 is 1 ms). The server tick time's p99 is always measured directly, with the parts reported beside it."
- **g. C1.9's reason.** Replace "the topics' own part marks add up inside 50 % at both tick rates, while 25 % for the whole tick would not hold them at 60 Hz (the arithmetic is in GLOSSARY's tick-budget section)" with "the topics' own part marks fit inside 50 % at 30 Hz, and at 60 Hz on every tick but one that both encodes a snapshot and re-floods after a portal change, while 25 % for the whole tick would not hold them at 60 Hz on any snapshot tick (the arithmetic, a planning check of ceilings, is in GLOSSARY's tick-budget section)". Put the revised table into GLOSSARY §1, because resolutions.md is deleted at the end, and update the glossary §1 summary sums.
- **h. C1.10 (the R2-BUD-REC6 spike).**
  - Change "the stress scene (GLOSSARY), 30-minute soak:" to "the stress scene (GLOSSARY) with a snapshot-receiving client on every soldier (R2-SYNC-REC9 mark 1's "48 clients"), 30-minute soak:".
  - Add a sentence after `per client per snapshot, memory per room reported.` (r2:2387, unique): "The mark is checked at the tick rate R1-TICK-REC2's rule picks; if it fails there, it fails like any spike mark and goes back to the drafts, and the rule does not change."
- **i. C1.21 (R6 A-2).** Replace "if the measured ticks do not sit well below the ceiling" with "unless R2-ROOM-REC2 marks 1 and 2 pass on it".
- **j. C1.5 (R1-ABL-REC6).** Change "inside the sim step share (GLOSSARY)," to "inside the sim step share on the server and the client's sim share on the laptop (GLOSSARY),".
- **k. Glossary, server-only share.** Add R2-INT-REC2's pair test (0.1 µs per enemy pair) and its re-flood (1 ms). After "(each send call at most 1 ms, R2-TRN-REC1 mark 2)" add "per call, not per tick: the tick's sends have no per-tick mark and share what is left".

**5. Whether the client predicts its own abilities: APPROVE** (aligned as ABL-PREDICT, Mustafa after a Feel Lab test). The interim is that abilities are not predicted. Either answer needs no redesign. Nothing to decide now.

**6. The spread random stream: APPROVE.** Spread comes from an authority-only stream. SEC-SPREAD stays Mustafa's open call, and the design holds under both of its answers.

**9. The stress scene: REVISE** (wording only; the scene is approved).
- In resolution 9's table and the glossary, replace "it comes from a stated rule (squads of 4, LOOP_V2 §2; one harvester per team, GE6), where 4 and 8 come with none" with "it comes from R2's stated assumption (one vehicle per squad of 4, LOOP_V2 §2's squad size, plus one harvester per team, GE6; R2 labels the row "assumption", r2:1758), where 4 and 8 come with no rule".
- Glossary, stress scene: add "For a server mark that includes the server-only share, every soldier has a snapshot-receiving client (R2-SYNC-REC9 mark 1), as the budget's 48 × 50 µs assumes."

**10. Mode sizes, totals or per team: ESCALATE (MODES-3, Mustafa).** The interim wording C10.1–C10.23 is approved: every mark gives both values. Resolutions 1 and 9 hold under both readings.

**11. One core or one vCPU: APPROVE.** Server marks are measured on one vCPU of the CCX23.

**17. Who owns the dependency audit in CI: ESCALATE (BASELINE-WHEN, the Advisor).** Interim C17.1–C17.9 are approved: cross-references only, no policy changes. C17.b1–b3 are approved, to apply only if the Advisor picks (b).

**19. Previews on Workers or Pages: REVISE** (wording only; Pages first is approved, with PREV-DELETE as the shared spike item).
- C19.3: change "0 USD on Pages' Free plan (R6-COST-REC1, R6-HOST-F3)" to "… (R6-COST-REC1, R6-HOST-F3; the price is inferred from the plan's name, no Pages price page was read)".
- C19.8: change "0 USD (Pages' Free plan, R6-HOST-F3)" to "0 USD (Pages' Free plan, R6-HOST-F3; price inferred from the plan's name)".

**21. A login wall on previews: ESCALATE (PREVIEW-WALL, Mustafa); the interim wording needs a REVISE.** C21.1 silently dropped R5's unreleased-art trigger into a question that was asked only about the staging server. That loosening would be Mustafa's call. Changes:
- C21.1 after: "This changes once a preview carries unreleased art: then the access wall (GLOSSARY) is on. Whether it is also on once a preview talks to the staging game server (R6-HOST-REC3) is PREVIEW-WALL (Mustafa; below); until he answers, no preview talks to a game server."
- C21.2 after: "… not already public? Its staging-server part is shared as PREVIEW-WALL (below)."
- C21.4 after: "a preview holds no secrets, and once previews reach a game server they talk only to the staging game server (REC3); whether those previews sit behind the access wall is PREVIEW-WALL (Mustafa; GLOSSARY: the access wall), and until he answers no preview talks to a game server (R6-HOST-OQ5)."
- New C21.8, R6-HOST-REC3 (r6:655-656, unique). Before: `need no server` + line break + `(R1-BND-REC6).`. After: the same, plus "Until Mustafa answers PREVIEW-WALL (below), previews do not use the staging server."
- The shared PREVIEW-WALL text: add to its "Until Mustafa answers" sentence "; a preview that carries unreleased art is behind the access wall, as R5-PREV-REC5 already says (R5-PREV-OQ1 asks that part)".

This interim holds under (a), (b) and (c).

**22. Isolation headers on every preview, or a flag: APPROVE.** One isolation flag, off by default. It also carries the rank-41 fix through C22.1, which is correct: R1-BND-REC5 says "passes REC2's threshold".

**23. One wildcard certificate on every game server: ESCALATE (BASELINE-WHEN, the Advisor).** Interim C23.1–C23.3 are approved. The (b)-only set needs a REVISE:
- C23.b1: put C2's description in brackets: "… until the security baseline weighs one wildcard certificate (option C2: renewed by one job and copied to every game VM) (BASELINE-WHEN (b))."
- C23.b2 also changes r6:669, "It needs no TCP 80 on game VMs.", to "With dns-01 per name it needs no TCP 80 on game VMs (F24); C1's http-01 needs TCP 80."
- C23.b2 also changes the Reason (r6:670-671) to: "a certificate per hostname is R2-TRN-REC5's and puts no shared private key on the game VMs; RFC 8555 proves control through DNS (F24), so dns-01 per name needs no TCP 80. C2's gain, for the baseline to weigh: one certificate for all VMs makes adding a VM a copy, not an issuance; RFC 8555 allows wildcard orders (F24)."

**29. HashMap in the sim: REVISE** (one wording change). The type ban with a reviewed `#[allow]` is approved. C29.2: change "sim uses no default-hasher HashMap or HashSet" to "sim uses no std HashMap or HashSet (R5-RUSTLINT-REC2 bans the types)". The ban is by type, not by hasher.

## The glossary: REVISE
All 34 entries have the form condition 9 asks for. Every entry that turns on an escalated decision gives each reading and picks none. Changes:
- **Tick budgets:** the fixes in 1c, 1f, 1g and 1k above.
- **Stress scene:** the 15 vehicles' basis and the client sentence from rank 9.
- **Very large unit:** the definition turns a test parameter into a design term. Replace it with "A very large unit is one of VISION_V2 §6.9's 'very large units'; in the stress scene each is a big vehicle of at least 20 parts (R3-CHAR-REC4's test load). Which units are very large, and whether every one is a vehicle, is the roster's (the game-designer's)."
- **Shared stream:** delete the example "the local player's own ability procs if ABL-PREDICT says yes". It would pre-decide, for ability procs, the same cheat trade-off SEC-SPREAD leaves to Mustafa.
- **Access wall:** change "previews that reach no game server are public but unlisted" to "previews that reach no game server and carry no unreleased art are public but unlisted; one that carries unreleased art is behind the wall (R5-PREV-REC5, R5-PREV-OQ1)".
- **Preview:** add "Until PREVIEW-WALL is answered there are no multiplayer previews."
- **Time statistics:** the retire list adds R2-INT-REC2's pair test and re-flood.
- **After this verdict:** the status line becomes "terms approved 2026-10-30 (advisor, step 3b)". The two "Depends on: the advisor's review of resolution 1/19" lines become "Decided by the advisor, 2026-10-30".

## The cross-topic open questions: APPROVE, with two nits
The 28 questions are in the critic's own words, with both sides' ids. Rank 18 carries BASELINE-WHEN, and each copy ends with "rank N", so a search finds every copy. Nits:
- **Rank 15:** add to its note (not inserted) that resolution 21's interim also rests on offline previews. If Mustafa says offline play is not a need (R1-BND-OQ1), PREVIEW-WALL must be answered before the first preview.
- **Line references inside inserted text** (r4:1110-1129 in rank 3, r6:757 in rank 13) gain "at commit 8dee746", since the insertions shift lines.

## Engine-architect's question: YES
Dated records keep their wording, and the step-3c term search skips them, under this rule:
- The skip is by marker only:
  - bracketed dated notes starting "[Fix pass", "[Second pass", "[Verifier" or "[Wave";
  - the verifier spot-check tables and fix logs under headings that name them;
  - quoted source text.

  Everything else is searched, including the "Wave-1 items R4 changes or answers" lists.
- The script lists the skipped hits per topic, and the report shows them.
- Records are never edited. Where an old term inside a record could mislead, the owner adds the new term outside it, as C6.6 does.

## Conditions for step 3c
1. **Fold first.** The session folds this verdict's exact texts into resolutions.md (marked "as approved, 2026-10-30") and GLOSSARY.md, changes nothing else, commits, and records the SHA. A fold that departs from these texts comes back to me.
2. **Apply.** Owners apply the APPROVE and REVISE items as folded. For escalated items they apply only the "now" changes and the shared question. C17.b1–b3 and C23.b1–b3 wait for BASELINE-WHEN. An answer from Mustafa or the Advisor that arrives before the PR is ready is applied as a separate, logged change.
3. **Order in each file:** resolution changes, then glossary renames, then the cross-topic questions, then nits. A quote that no longer matches means OBJECT; owners never improvise.
4. **Log as recommendation changes** (the verifier checks them per condition 10): C1.1, C1.5, C1.8–C1.11, C1.15–C1.18, C1.21; C5.1–C5.4; C6.1, C6.2; C9.1, C9.3, C9.7, C9.9, C9.14, C9.18–C9.20; C19.1, C19.5, C19.6; C21.1, C21.4, C21.8; C22.1, C22.3, C22.5; C29.1–C29.3. Also every mark the "no statistic means p99" rule changes: at least R1-ABL-REC6, R2-INT-REC1, R2-INT-REC4 and R2-INT-REC2.
5. **R3's owner** changes only counts and arithmetic (C9.12–C9.16, C10.12–C10.13) and glossary terms. The look rows (post-processing, outline, style) stay word for word.
6. **GLOSSARY.md** is edited only by the engine-architect agent (its R1 and R4 run), and never while the other owners are renaming terms. Its "used in" lists are refreshed from the search script after step 3c.
7. **No fetching in step 3c; nothing new.** No new number, source or claim. Quoted source text and dated records are untouched. At most 4 agents at once, one per file, efforts as the plan says.
8. **Step 3d** recomputes resolution 1's revised table independently. The report carries the 12 resolutions and these verdicts word for word, including the 60 Hz tight spots and the cost note: if 8 rooms per VM fail, compute per room-hour is 0.0347 instead of 0.0173 EUR (R6's own rows).

## Decisions needed
- **MODES-3 (Mustafa): how should VISION_V2 §3's starting sizes be read?** The text says "two teams with 2–8 players, and three teams with 6–18 players".
  - **Why it matters:** it sets the largest mode, which the stress scene, the offline test, the soak runs and the cost rows use (48 or 54 players).
  - **Options:** (a) totals: 1v1 to 4v4, and 2v2v2 to 6v6v6, so the largest mode is 24v24 = 48. (b) per team: 2v2 to 8v8, and 6v6v6 to 18v18v18, so the largest mode is 54.
  - **Recommendation:** (a). Under (b), the start modes would be bigger than both goals.
  - **Note:** CLAUDE.md's own wording, "two teams of 2–8", reads like (b). It needs aligning after your answer, and CLAUDE.md changes are yours.
- **PREVIEW-WALL (Mustafa): should per-PR previews that talk to the staging game server be private, behind a Cloudflare login on your account?**
  - **Why it matters:** it trades one login per Brave check against strangers reaching unreleased work and the staging server.
  - **Options:** (a) private from the first such preview; (b) open, since previews hold no secrets; (c) decide after the security baseline.
  - **Recommendation:** (a).
  - **Meanwhile:** no preview talks to a game server, and previews carrying unreleased art stay behind the login, as R5 already says.
- **BASELINE-WHEN (the Advisor, relayed by Mustafa): research the deferred security baseline before ADR 0006, or list it open there?**
  - **Why it matters:** two contradictions wait on it: who owns the CI advisories check and whether it blocks a PR (#17), and one shared certificate key on every game server (#23).
  - **Options:** (a) research it now, as one security-auditor topic; (b) list it open, with interim positions (R5's scheduled advisories check stays the mechanism; one certificate per hostname stays, and R6's wildcard becomes an option the baseline weighs); (c) leave both as one shared open question.
  - **Recommendation:** (b).

## For Mustafa
1. Answer MODES-3 and PREVIEW-WALL above, if you haven't yet.
2. Pass BASELINE-WHEN to the Advisor.
3. Still open from my last verdict, if not done yet: create the night-shift routine that starts a fresh session on every run, and disable the old one in the same sitting.
4. Later: say "merge #28" once the advisor approves its head.

Files:
- My notes, ending in DONE: /home/user/Shamble-Frontlines/.checkpoints/v2-research/advisor-resolutions-review.md
- Quote check script: /tmp/claude-0/-home-user-Shamble-Frontlines/ed3ff506-9a45-5a14-adc3-75c262311890/scratchpad/adv/check.py
- Line check script: /tmp/claude-0/-home-user-Shamble-Frontlines/ed3ff506-9a45-5a14-adc3-75c262311890/scratchpad/adv/lines.py
