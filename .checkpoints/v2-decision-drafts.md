# Checkpoint: v2-decision-drafts
- **Goal:** the v2 decision drafts (`docs/ENGINE_OVERVIEW.md`, `docs/adr/0001`–`0006`, the spike plan, the loose ends)
  from the drafts brief (`v2-decision-drafts/inputs/DRAFTS_BRIEF.md`), then a draft PR, the report and a stop for Mustafa
  and the Advisor · plan: `v2-decision-drafts/plan.md` (revision 3: the advisor's APPROVE of revision 1 with 19 conditions folded in revision 2, verdict verbatim in `advisor-verdicts.md` §1; revision 3 applies Mustafa's UPGRADE-SCOPE update)
- **Branch:** claude/laughing-lovelace-ox61se (the session's branch, standing in for cs/v2-decision-drafts) ·
  **Base:** origin/main 9e7db28 (PR #32's merge)
- **Last update:** 2026-11-01 13:52 Amsterdam

## Steps done
(Earlier steps below; the run of 2026-11-01:)
- 5bdad85 · ADR 0006 assembled (infra-engineer's draft + test-engineer's PM-0006-1..31), write guard PASS.
- 2a0e5f6 · ADR 0003 assembled (graphics-engineer's draft + PM-0003-1..43), write guard PASS.
- 6b175de · ADR 0001 assembled (netcode-architect's draft + PM-0001-1..56), write guard PASS.
- 789bedb · ADR 0002 assembled (engine-architect's draft + PM-0002-1..76), write guard PASS.
- d321a13 · ADR 0004 assembled (rust-reviewer's draft + PM-0004-1..72), write guard PASS.
- 09803a9 · ADR 0006 fix round (infra-engineer's 22 exact replacements; verifier findings 1, 2, 4, 7–13).
- 80a27b3 · ENGINE_OVERVIEW, SPIKE_PLAN, docs/adr/README.md, GLOSSARY §8 (synthesis); 521/521 ids disposed.
- 549490f · docs-keeper: STATUS, ROADMAP, REPO_MAP.
- cf00def · the pass-mark files' cut marker removed from all six drafts (verifier 0006 finding 3); assemble.py fixed.
- 233f6f7 · ADR 0005 assembled (engine-architect's frame with the four sections byte for byte + PM-0005-1..46), write
  guard PASS. Draft PR #33 opened on it.
1. Step 0 (brief §0): `docs/LOOK_V2.md` added byte for byte (sha256 52b4fc06…) · 6617e25; the brief copied to `inputs/`;
   plan revision 1 written · d6c71e4.
2. Plan review: the advisor's APPROVE with 19 conditions (about 18:35); folded into plan revision 2 (no second review
   needed, per the verdict); `origin/main` fetched to 9e7db28 (condition 1); the id list rebuilt (REC 165: the two R3
   rows were references, condition 6) · 5ab5eae.
3. Mustafa's UPGRADE-SCOPE update (about 18:41, in the session's chat; it replaces the brief's upgrade note): saved
   verbatim to `inputs/UPGRADE-SCOPE-mustafa-2026-10-31.md`; plan revision 3 applies it (settled inputs, the ADR 0005
   paragraph, the map addendum's part (e), docs-keeper's STATUS record) · f927ab3.
4. Decision map (step 2) done; coverage script: 521 of 521 ids, one owner each; addendum sent; drafting started
   (0003, 0006, 0005's ability section) · b2719db.
5. The map's addendum done (part j); 0001 and 0004 launched · deeea0d.
6. Spend-limit cut (about 19:18) recorded; resumed after the reset (22:52); 0006 and 0005's ability section finished;
   0002 and 0005's content part launched before 23:00 · this commit.

## Current step
Third spend-limit cut at about 09:27 Amsterdam ("your 5-hour limit resets 12:50pm (UTC)", 13:50 Amsterdam): the
consistency critic, the synthesis' verifier, verifier 0005 and netcode-architect's 0001 fix round failed with HTTP 429
(their notes hold their progress); verifier 0003 finished first (PASS WITH NOTES: 431 of 440 confirmed; 1 wrong, 4
unsupported, 2 citation off, 2 rule findings; 24 fetches, all 200). The session itself was cut too; the 10:40 wrap-up
reminder fired at 10:40 but reached the session only at 13:52 and is superseded. Mustafa, 13:52, in the session's
chat: "I hit my usage limit while you were working, but it has reset now. Please continue from where you left off."
His one-off night exception ended at 11:00; the normal rules apply again (daytime: fetching until 22:00, no new agent
after 23:00). Resumed at about 13:53: the four cut agents, and rust-reviewer for 0004's fixes.

## Next steps (the scheduled run, 2026-11-01 03:55–11:00 Amsterdam: Mustafa's one-off exception, below)
Time plan (the session leads it; Mustafa: "decide on when it start and when its done"):
1. 03:55: re-read this checkpoint and the plan (revision 3); `git log --oneline -3` shows this commit or a later
   checkpoint commit; start the saver (`checkpoint.mjs watch`, every 5 minutes).
2. Wave A (≤ 5 agents): ADR 0005's frame (`prompts/engine-architect-adr0005-frame.txt`); pass marks for 0001, 0002,
   0004, 0006 (`prompts/_test-engineer-passmarks-TEMPLATE.txt`, filled and saved per draft). Then pass marks for 0005
   once the frame lands; a script check that the four 0005 sections sit in the frame byte for byte.
3. Assembly (step 5): `scripts/assemble.py` per draft into `docs/adr/000N-<name>.md`; write guard; commit.
4. Synthesis (step 6, engine-architect): ENGINE_OVERVIEW, SPIKE_PLAN, docs/adr/README.md (the 38 DESIGN ids; the map's
   codes the drafts cite; the features whose need is outside VISION_V2 §6), GLOSSARY §8 appended after line 742 (text in
   the map's J1); `scripts/dispositions.py` → 521 of 521; commit.
5. Verification against the research only, no web (step 7, part 1; one verifier per draft and one for the synthesis;
   every source check marked "pending (before 08:30)"); skeptics on wrong or unsupported verdicts; the consistency
   critic and the completeness critic (step 8); the glossary and citation scripts.
6. Fix round (step 9): each finding to its author agent; re-verify what changed.
7. From 08:30 (daytime): the source checks (step 7, part 2: allow-listed pages, curl, the host audit); their fixes.
8. Reviews on the final diff (step 10), ≤ 5 at once; docs-keeper (step 11); the gate (step 12); the report and the draft
   PR (step 13).
9. Stop by 11:00: no new agent after 10:30; at 10:40 (a scheduled reminder) the session checkpoints, records what is
   not done, and reports. Priorities if time runs short: the deliverables exist and are assembled (2–4) > checked
   against the research and consistent (5–6) > source checks (7) > reviews (8) > docs-keeper and the PR; whatever is
   not done is said plainly in the report, and the next day session finishes it.
- A spend-limit cut during the run: record it, launch nothing new, resume after the reset (the window that opens at
  03:50 closes at 08:50). It happened at about 05:30; resumed about 09:00.

## Open findings
- The research has no asset-pipeline findings (ASSET-RESEARCH (a), ruled; asset-pipeline research unit before the asset
  spike).
- For the synthesis and the consistency pass: drafts cite the decision map's ids (interfaces I-n, research gaps G-n, loose
  ends LE-x, J-n), and the map is a checkpoint file deleted before the PR is marked ready: carry the needed ones into
  `docs/adr/README.md` (the register) or reword them (graphics-engineer's and rust-reviewer's notes flag it).
- Map N48 says 32 audio voices on both tiers; r3:1246 says 32 on Low and 48 on High; ADR 0003 follows the research
  (graphics-engineer). The map's part g had spikes decide R3-BUDGET-OQ2, OQ3, OQ6; ADR 0003 makes them open questions
  (OQ-0003-10, -12, -4).
- ADR 0006: ids its author thinks belong elsewhere (R5-PREV-REC3's jobs → 0002; R1-RR-OQ7 and R5-GATE-OQ4's CI side →
  0002; R2-ROOM-OQ11 → 0001); goal costs shown at two densities (8 rooms per VM: about 361 EUR; 4 per VM: about 534 EUR);
  `inputs/findings-checked.tsv` misses R6-HOST-F8 and F20, which R6's second pass confirmed (r6:1170): the topic's own
  section is the authority (the verifier prompt says so).
- ADR 0005's ability section: AB11 (on legacy numbers the Banzai blade kills through the Riot Shield) is listed for
  Mustafa; AB12 and AB13 are for the Advisor.
- ADR 0004: uses the map's interface and seam codes (I-1, I-9, I-10, I-12, I-16, I-17, I-19, I-20, I-21, S1): they need a
  lasting home (ENGINE_OVERVIEW or adr/README) or removal; OQ-0004-5 (drops and pickups) and OQ-0004-6 (bots' player
  records) may sit better with ADR 0005's upgrade questions (consistency pass); R1-XU-OQ6 disposed as answered.
- ADR 0001: R1-TICK-OQ1 (60 or 64 Hz) and R2-INT-OQ5 may belong to 0004 (consistency pass); it replaces four lines
  under TICK-30 (r1:2029/2061–2062, R2-BUD-REC1's tick half at r2:2302, r2:2415–2416, GLOSSARY:85), one more than
  condition 2 lists: the verifier checks it.
- Pass marks 0003 (test-engineer), for the 0003 author's fix round: (a) vsync makes the desktop's median ≤ 14 ms
  unpassable and the laptop's median ≤ 28 ms a hidden 60 fps demand; "WebGPU ≤ 1.1 × WebGL 2" and the overlay mark can't
  fail at vsync (proposed: measure uncapped; the browser switch is not researched); (b) the overlay's 1 % mark is below
  the harness's own 3 % spread; (c) sub-ms marks sit near the 100 µs timer step; (d) the first-sight stutter check passes
  if the browser cached shaders; (e) "zero bytes per frame" needs a planted allocation to prove the method; (f) the look
  proof test needs a synthetic recording for its backend marks; (g) the Safari 26 marks need a Mac the draft never names.
  The page's cost of the sim gates twice (R3 p99 in the bench, PM-20; R1-BND-REC1's p95 in 0002's boundary spike). The
  map's part g put R3-MEASURE-REC1/REC4 in the look test, the draft in the bench (the draft holds). The map's N12 cites
  r1:2258–2260; it is 2259–2261.
- ADR 0005 content (engine-architect): CD-7 changes R1-DATA-REC5 (in development, sim numbers apply from the next match,
  proposed; OQ-0005-CD4), so ADR 0001's `ContentReload` message and ADR 0004 part 16 ("a development reload swaps them
  between ticks") hold only under the mid-match option: consistency pass. Legacy rows ED5 and ED7 (Character Lab, Anim
  Lab) are listed by no draft. Its "For the frame" carries the record-level parts.
- Pass marks 0001 (test-engineer), for the 0001 author's fix round: the 160 kbps mean rests on R2's estimate with 32
  live projectiles, the stress scene has 300 (about 162–192 kbps with E3; with E3b the projectiles alone need 1,650 B
  per snapshot, over the 1,067 B cap), so the Rollback's "E3b about +42 kbps" holds only at 32: the draft must name the
  scene; "join within 300 ms at P2" needs a start point and a loss rule; "RTT within ±5 ms" is R5-TEST-REC4's at a fixed
  100 ms, not "at P2"; the throwaway sim has no vehicles, projectiles or effects and nothing sets the portal-change
  rate (stand-ins proposed); "What a spike must prove" omits R2-SYNC-REC7 mark 3 though Appendix A sends R2-SYNC-OQ5 to
  the spike; the tick-wrap check runs only at 60 Hz; R2-ROOM-REC2's "rooms of bots" would leave out encoding and
  transport work.
- Pass marks 0002 (test-engineer), for the 0002 author's fix round: "at most 4 Wasm calls per tick" vs the draft's own
  5–6 for the online client (gated as control-API calls, glue reported: the drafter confirms the reading); R1-BND-REC1
  mark 1 needs vsync off and a defined no-sim baseline (fixed in the method); block latency within one frame cannot fail
  on a light page (page loaded to 16 ms); "heap growth" by allocation sampling measures a rate, Chrome only; R5's L1 mark
  at P1's 1 % loss misses its 5 % band about 1 seed in 9 at 100,000 packets; server tick time in the nightly soak needs
  the reference server's vCPU; gaps: R1-DATA-OQ8 had no spike item (PM-0002-31 added), the map lists R1-BND-REC6 under the
  netcode proof test but 0002's spike item 6 holds it, no limit for the overflow checks' cost, no Mac and no 120/144 Hz
  displays named, B2 vs B3 only on an isolated build.
- Pass marks 0004 (test-engineer), for the 0004 author's fix round: "Wasm ≤ 2 × native" names no machine (both measured
  on the laptop); S7's planted divergences could pass without catching anything (now triggered at a known tick with a
  known-different value); per-phase times need clock reads S4 forbids inside the sim (phases called one by one from the
  bench); S2 cannot pass until R1-TICK-OQ7 sets N; the "at 30 and 60 Hz" halves of S3, S4, S9, S11, S13 moved into the
  60 Hz set (rust-reviewer confirms); S17 placed in the netcode proof test but ADR 0001 never mentions R1-TICK-REC4; the
  map puts R2-SYNC-REC6 mark 1 and R5-LAYOUT-REC4 in the sim core spike but no draft's table names them (SPIKE_PLAN
  places them); S9's "largest planned map" read as part 7's 2 km bound; PM-0004-2 checks the stress scene's load every
  tick; S20's online recovery needs the proof test's server.
- Pass marks 0005 (test-engineer), for the 0005 authors' fix round: "0 accepted invalid" is met by a loader that
  refuses everything (a labelled corpus with valid controls added); CD-1's "written back as the same text" may fail on
  serde_json's number format (not researched; the spike's first run settles it); R1-ABL-REC2's 1e-6 tolerance read as
  stored values, shots within one tick; "zero matches for the ids" cannot prove no ability-specific code (a rename test
  added); fuzzed definitions that load run 300 ticks each; OQ-0005-AP2 needs the look proof test to port the kit
  builders, which 0003's spike entry does not list; the glTF dependency check sits in the asset spike in one place and
  the foundation in another; "every legacy number present once" vs CD-10's drops (read as mapped once or dropped with a
  reason); CD-10's "value" read as each leaf (725 outside the look and state groups, 2,011 in the look group).
- docs-keeper's notes for later: STATUS points to `advisor-verdicts.md` §1, which goes with the checkpoint (move the
  pointer to the report's appendix before ready); four STATUS statements rest on the README draft or unfinished work
  (re-check before ready); STATUS is now 345 lines (was 288): trimming old legacy rows is a separate cleanup.
- Write guard: condition 1's `cmp - <(head -n 742 GLOSSARY.md)` fails on an unchanged file, because the base GLOSSARY ends
  without a final newline (742 newlines, then an unterminated last line); `scripts/write_guard.sh` checks the exact
  intent instead (the base file is a byte prefix of today's), so §8 can be appended and nothing above it may change.
  Report it as a deviation in form, not in intent.
- Pass marks 0006 (test-engineer), for the 0006 author's fix round: PREV-DELETE's "or the remaining case is written
  down" cannot fail (read it as a fail with its reason); with the access policy on, leftover links pass trivially (run
  with the policy off); Pages has a branch link plus one per deployment (all checked); the `wss` fallback cannot use the
  pinned self-signed certificate (the WebSocket constructor takes no hashes, R2-ROOM-F12); "nothing else" in the
  firewall leaves no admin or deploy path; rooms per VM on the throwaway sim stands for the real one only if its tick
  costs at least as much; the egress mark's reference must be recomputed from the counts run (no vehicles: 112.3 vs
  146.1 kbps); "under 5 EUR" is 36 CCX23-hours; the CX33 start-up under 2 s restored; "with interest on" needs the
  filter in the proof test's sim; the access-wall login check's place differs between the map and the draft.
- ADR 0005's frame (engine-architect): sections disagree on model ids and faction colours in sim definitions (AP-2 and
  CD-13 vs CD-6, AB-3/AB-4 and inventory row CD13): OQ-0005-1, the frame leans to client definitions keyed by the same
  id; CD-6 "a weapon definition is a weapon type" pre-empts OQ-0005-AB4 (b): OQ-0005-2; frame-only OQ-0005-3 (what the
  content hash covers) and OQ-0005-4 (Character Lab and Anim Lab, rows ED5 and ED7); AB5 says "ADR 0006 cites this", 0006
  does not; the author line follows the brief (0002 also credits test-engineer: align at assembly); spike item 4 (map
  migration) moved to slice 2.
- ADR 0002 (engine-architect): its ledger adds two tiers outside the plan's six ("cross-draft" 18, "inventory" 10; the
  verifier may fold them); ids its author thinks belong elsewhere: R5-TEST-OQ3 (DESIGN), R5-TEST-OQ2 (0001), R5-PREV-OQ5
  and OQ6 (0006); judgement calls marked: the `room` and `navbake` crates, the multiplayer-preview delivery shape, the
  wasm-bindgen CLI over wasm-pack, two labelled choices (a second production-isolation trigger, a `forceWebGL` CI run);
  loopback in the protocol crate (aligned with 0001); no legacy row's mark changes.

## Key numbers
- Script checks on the six assembled drafts (about 09:05; outputs in `consistency-scripts.txt`): dispositions 483 of
  521, each once, all in the map's draft (the 38 DESIGN ids wait for the README); citations: 3,355 research ids cited,
  0 that exist nowhere, 75 line ranges flagged (many false: groups citing two files); glossary (the research's own check,
  pointed at the drafts): 2 exact hits ("whole tick" in PM-0001-4 and PM-0001-39, test-engineer's rows: fix to "the
  server tick time measured directly"), 489 in-context hits to judge (244 of them "the largest mode, and its N").
- Dispositions across the six drafts (23:26; 0005 from its three sections' Appendix A rows): 483 of 521 ids, each
  exactly once, every one in the draft the map assigns; the 38 not yet disposed are exactly the map's DESIGN-owned ids
  (they go into docs/adr/README.md's register at step 6).
- Early script checks on the finished parts (22:57): `dispositions.py` parses 0003's 71 and 0006's 65 Appendix A rows,
  0 duplicates, 0 in the wrong draft; `cite_check.py` (a coarse pre-check: it reads a citation group's line range
  against the first file named in it, so groups citing two files give false hits) flags 7 ranges in 0003, 16 in 0006
  (3 of them clearly r2 ranges inside an r6 group), 2 in the ability section, 0 in the GPU section: hints for the
  verifiers, not findings.
- Research ids defined in r1–r6 at 9e7db28 (`inputs/research-ids.tsv`, built by script, rebuilt after the review): 533
  rows, 521 unique ids: REC 165, OQ 241, RR 7, XU 30, XT 72, 6 shared codes (18 copies). Matches the research report's
  165 recommendations and 278 own open questions (241 + 30 + 7).
- Findings checked by each topic's verifier (`inputs/findings-checked.tsv`, script-built guide): R1 172/185, R2 136/152,
  R3 44/92, R4 49/62, R5 49/117, R6 18/42.

## Decisions taken
- 2026-10-31 between about 23:30 and 23:50, Mustafa (in the session's chat), word for word: "the next usage reset is going to be on 03:50
  AM, so you can maybe tell it to prepare accordingly (starting the task at the right time)" and "Just for today
  (tonight): 03:55, it can work until 11:00 because I will not have much time to use claude, so it can work if needed.
  You can take the lead in ensuring things get done accordingly, decide on when it start and when its done. Remember:
  just for today (upcoming session)". It sets aside, for the run of 2026-11-01 03:55–11:00 only, the brief's "daytime
  only" and the advisor's condition 17 (no new agent between 23:00 and 08:30). The session keeps web fetching to 08:30
  and later, since daytime-only exists for unattended fetching before the write guard (DOMAINS-ACCEPT (a),
  NIGHT-RESEARCH (a)). Every other condition holds. After this run the normal rules apply again.
- 2026-10-31: the drafts brief (Mustafa and the Advisor) and LOOK_V2 (attachments of the session's first message).
- 2026-10-31 about 18:41, Mustafa: the UPGRADE-SCOPE update (tracks per unit and per weapon type, pick one of three per
  level, owned by the player, surviving death and switching, reset every match; a class comes with its own weapon;
  other weapons only from enemy drops, as world pickups; progress on the weapon type; on the PlayerState). Open for him:
  carry slots, ammo for looted weapons, track levelling speed with catch-up.
- 2026-10-31 about 18:35, the advisor: plan APPROVE with 19 conditions; rulings ASSET-RESEARCH (a), TICK-60-HEADROOM
  ruled now (base 8.33 ms, p99 ≤ 6.25 ms; confirmed at the stop), GLOSSARY-8 yes (three entries and a note),
  CHECKPOINT-AT-STOP keep.

## Running work
- advisor plan review: done (APPROVE), notes `advisor-plan-review.md` (DONE).
- engine-architect decision map: done about 18:45 (521 of 521 ids, one owner each: 0001 127, 0002 104, 0003 71,
  0004 82, 0005 34, 0006 65, DESIGN 38); its addendum (part j, from line 1032) done about 18:52, owners unchanged,
  coverage re-run 521/521; GLOSSARY §8 text in part J1.
- Every drafting unit finished by 23:25 (notes DONE). The run's saver: `checkpoint.mjs watch` every 5 minutes, started
  03:56. Wave A (launched about 03:57, background agents, opus, xhigh): engine-architect (0005 frame; notes
  `engine-architect-adr0005-frame-notes.md`), test-engineer ×4 (notes `test-engineer-passmarks-000N-notes.md` for 0001,
  0002, 0004, 0006).
- Scheduled tasks (send_later into this session, human_request): trig_01BH1UStnQLgHuzsethFGmTU fired 2026-11-01 03:55
  Amsterdam (02:55 UTC): the run's start (done); trig_01WnvjBEFsLQPXjoj7WdwN7d fires 10:40 Amsterdam (09:40 UTC): the wrap-up.
- Ready for the run: pass-mark prompts filled and saved for 0001, 0002, 0004, 0005, 0006
  (`prompts/test-engineer-passmarks-000N.txt`); the verifier template now allows fetching only between 08:30 and 22:00.
- Drafting, launched about 18:48 (background agents, effort xhigh, opus): graphics-engineer (0003 + 0005 asset section;
  notes `graphics-engineer-adr0003-notes.md`), infra-engineer (0006; `infra-engineer-adr0006-notes.md`), game-designer
  (0005 ability section; `game-designer-adr0005-notes.md`). Prompts in `prompts/`.
- checkpoint saver: `checkpoint.mjs watch` ran 18:22–20:12 and ended (its 110 minutes); restarted at about 22:55.
- Spend-limit cut, 2026-10-31 about 19:18 Amsterdam: agents netcode-architect (0001), infra-engineer (0006),
  rust-reviewer (0004), graphics-engineer (0003, 0005 asset section), game-designer (0005 ability section) failed with
  HTTP 429 "org's monthly spend limit … your 5-hour limit resets 9:50pm (UTC)"; each resumed from its notes by message
  after the reset.
