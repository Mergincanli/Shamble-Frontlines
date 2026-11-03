# Plan revision 6: Mustafa's Hetzner console check (v2-decision-drafts, PR #33)

2026-11-03, about 18:31 Amsterdam. PR #33 is not merged and not yet marked ready (the close was about to start: plan
revision 4's conditions 7 to 9; the checkpoint is still on the branch, at 21ea733, and the saver runs). His words,
verbatim: `inputs/HETZNER-CHECK-mustafa-2026-11-03.md` (received about 18:26 by the session clock):

> Hetzner check (Mustafa, 2026-11-03): in Falkenstein the cost-optimized x86 line (CX33) can't be ordered; only its
> Arm64 version can. Use a CAX21 (Arm64, 4 vCPU, 8 GB, about €0.020/h incl. VAT) for the simulated clients instead of
> the CX33, so their build targets aarch64. The game server stays a CCX23 (dedicated, x86) in Falkenstein; I checked it
> can be ordered there. Update SPIKE-MACHINES and every place the drafts name the CX33.

## What his words decide
1. **OQ-0006-1** (his console check): the CCX23 can be ordered in Falkenstein; the CX33 cannot be ordered there, only
   the cost-optimized line's Arm64 version. The research had flagged both as possibly not offered in Falkenstein
   (R6-HOST-F13: the location column read "NBG1 HEL1", unverified); his check settles it for Falkenstein.
2. **SPIKE-MACHINES, the load generator (LG):** CAX21s (Arm64, 4 vCPU, 8 GB) run the simulated clients, "so their build
   targets aarch64". Price: his "about €0.020/h incl. VAT"; the research's figure is 10.99 EUR a month, 0.0176 an hour,
   excluding VAT (R6-HOST-F15, read 2026-10-28), which the bill mark (PM-0006-18, "excluding VAT") uses.
3. **The game server (RS):** a CCX23 (dedicated, x86), in Falkenstein.
4. **"Every place the drafts name the CX33"** is updated. Where his words name the replacement (LG), it is the CAX21;
   where they do not, the place says what his check changes and what stays open (the decisions below).

## Every place the drafts name the CX33 (by script; owner by section)
- **LG (the simulated clients):** 0001:1356 key (test-engineer); 0006:1191-1192 key (test-engineer); 0006:373 (an
  analysis VM "as spike 2's load generators", infra-engineer); 0006:1073, 1077 (OQ-0006-20's options: "the CX33 hours
  of SH, MV and the load generators", "0.0144 a CX33-hour"; infra-engineer); SPIKE_PLAN:32-33 (engine-architect b).
- **SH (the shared CX33 measured beside RS, for rooms per VM; option G2, K3):** 0006:119, 244, 254, 352, 607, 713, 768,
  1134, 1448 (infra-engineer); 0006:1189 key and PM-0006-11 (test-engineer); 0001:1293 (netcode-architect); 0001:1354
  key, PM-0001-27's machines "RS, then SH" (test-engineer); 0004:1032 key "**CX33**", PM-0004-1, PM-0004-4 ("on the
  cheaper host") (test-engineer).
- **MV (the metrics VM, "a CX33 in the same location running Prometheus, standing in for the staging game server"):**
  0006:1199 key (test-engineer); 0006:1073 (infra-engineer); SPIKE_PLAN:33.
- **The staging game server (part 4, "proposed a CX33", from slice 3) and the budgets built on it:** 0006:189, 325-326
  (part 8: "about 18 EUR a month lean (production CX33 8.99 + staging CX33 8.99)", "102 EUR + 5 USD reference
  (production CCX23 86.49 + staging CX33 8.99 + …)"), 1435 (the analysis "on a CX33, the staging VM's plan"), 1510
  (Appendix A, R6-TEL-REC3) (infra-engineer); PM-0001-63's formula ("plus the staging CX33 (8.99 EUR)",
  test-engineer).
- **The lobby's placeholder VM:** 0006:267, 825 ("for example a CX33 at 8.99 EUR a month", infra-engineer).
- **OQ-0006-1 and the location:** 0006:250, 632, 799-809, 811-813 (infra-engineer); 0006:1189 ("in the location
  OQ-0006-2 picks (`nbg1` or `hel1`)"), PM-0006-11 ("created in `nbg1` or `hel1`"), PM-0006-12 ("feeds OQ-0006-2")
  (test-engineer); README:133-134; SPIKE_PLAN:129 ("OQ-0006-1 answered (the CCX23 and CX33 orderable in one EU
  location, …)") (engine-architect b).
- **Not edited:** GLOSSARY:227 and :605 (base text, lines 1–743, research: the write guard keeps them; a NOTE for the
  report). STATUS:117 quotes his SPIKE-MACHINES words of the stop ("the CX33 runs the simulated clients"), a record
  kept word for word; docs-keeper adds the Hetzner check beside it. ENGINE_OVERVIEW names no CX33 (grep).

## What aarch64 touches (each author checks its own text; no new engine feature)
- **ADR 0001 (netcode-architect; test-engineer's rows):** LG runs "the headless bot clients, which speak the real
  protocol" (R5-TEST-REC5). Rows that compare a result an LG client computes with the server's could count a
  cross-architecture difference as a fault: PM-0001-54 (1,000 scripted throws, "each landing compared with the
  server's", on L1 and L2 with LG). Rows that compare only decoded, quantised values (PM-0001-17's hashes, -57, -58)
  carry no float work. The author says which rows, and how each stays sound: the client-side computation follows ADR
  0004 part 7's rules (no platform maths, so x86-64 and aarch64 agree), or that comparison is taken from L1 or the
  browsers (Wasm), with LG's clients as load only.
- **ADR 0004 (rust-reviewer):** part 7 says native AArch64 joins the native-against-Wasm hash test before ADR 0006 may
  pick ARM hosts (PM-0004-26). Whether an ARM load generator is "an ARM host" in that sense (it serves no player; it
  computes nothing a mark compares, if ADR 0001's answer above holds), and whether S7 or PM-0004-26 must move.
- **ADR 0002 (engine-architect a):** "No arm64 leg in the determinism job: the first fleet is x86-64" stays true for
  the fleet (RS is x86). The LG binary is throwaway spike code (SPIKE-SCOPE (a)); its aarch64 build
  (`aarch64-unknown-linux-gnu`) is the spike's own, not a CI leg. The author checks its text and its R5-CI-OQ7 row and
  writes at most one sentence.

## Decisions for the advisor (recommendations first)
- **HZ-SH, the shared host measured beside RS.** The CX33 cannot be ordered in Falkenstein. (a) Recommended: the CAX21
  becomes SH, the cost-optimized line's Arm64 version his check found orderable there, at 10.99 against 86.49 EUR a
  month (7.9 times cheaper; R6-HOST-F15); the proof test builds its room server for aarch64 for SH's rows only, and a
  production pick of a shared Arm fleet (K3) waits for PM-0004-26 (ADR 0004 part 7's rule). (b) SH leaves the proof
  test; K3 and PM-0004-4's "cheaper host" wait for a later measurement. (c) Mustafa checks a shared x86 line in
  Falkenstein (CPX32, 35.99 EUR a month, R6-HOST-F15; its locations not researched). (d) To Mustafa now.
- **HZ-MV, the metrics VM.** (a) Recommended: a CAX21 in the same location (Prometheus only, no game code on it). (b)
  Left open for the proof test's plan.
- **HZ-STAGING, the staging game server and the budgets.** It exists from slice 3, so nothing is picked now. (a)
  Recommended: part 4 says "a small shared VM in the game servers' location (proposed; the CX33 cannot be ordered in
  Falkenstein, Mustafa, 2026-11-03): a CAX21 (Arm64: the room server built for aarch64 after PM-0004-26) or a shared
  x86 plan where one is offered, chosen before slice 3"; part 8's budgets and PM-0001-63's formula keep the CX33's
  8.99 EUR as that VM's placeholder price, labelled so, or take the CAX21's 10.99 (+2 EUR a month; the lean case
  21.98 EUR on two CAX21s if HZ-SH (a)); the lobby's placeholder VM the same way. (b) A new open question for the
  staging VM's plan (115 → 116 numbered).
- **HZ-LOCATION, OQ-0006-2.** "The game server stays a CCX23 (dedicated, x86) in Falkenstein". (a) Recommended:
  answered by Mustafa, Falkenstein (`fsn1`); PM-0006-12's round trip from his machine to `fsn1` is still reported
  against VISION_V2 §4's "about 100 ms", as a check, not a pick; R6-HOST-OQ2's playtester round trips stay for later.
  (b) Answered in part: Falkenstein is orderable; the measurement still picks, he confirms.
- **HZ-OQ1, OQ-0006-1.** (a) Recommended: answered by Mustafa (2026-11-03), option (a) for the reference server, the
  CX33's role taken by the CAX21; the dedicated root servers' read before the goal budget is fixed (R6-HOST-OQ4) and
  the IPv4 price stay as written. (b) Answered in part, until SH's machine is settled.
- **HZ-PRICE.** His "about €0.020/h incl. VAT" is quoted as his; the excluding-VAT figure the bill mark uses is
  R6-HOST-F15's 0.0176 an hour; no VAT rate is computed (no source).
- **HZ-RECORD.** Text that records what was weighed (OQ-0006-20's options and interim, option G2's row, Appendix A's
  rows) keeps its words as the record and gains at most one pointer to his check; text that states the rig, the plan
  or a price now is rewritten.
- **HZ-COUNTS.** With HZ-LOCATION (a) and HZ-OQ1 (a): 115 numbered, 95 open (ADR 0006: 21 numbered, 16 open); pass
  marks stay 341 (no row added or dropped; rows change their machines).
- **HZ-VERIFY.** His stop holds ("No further re-check rounds; anything left goes into the PR as notes"): no verifier,
  no skeptic. The session runs its scripts (write guard, dispositions, glossary, tables, counts, quotes against all
  four inputs files, a grep that every remaining "CX33" is a quote, a record or a NOTE), the gate, and the advisor's
  final-head review.

## The round (one round, two workflows, every author on its own text, xhigh; Opus)
- infra-engineer: every ADR 0006 place above outside the pass marks; OQ-0006-1 and -2 entries; part 5's location.
- test-engineer: the keys and rows of ADR 0001 (SH, LG; PM-0001-27, -54 if netcode-architect's answer moves it, -63's
  formula), ADR 0004 (the CX33 key, PM-0004-1, -4), ADR 0006 (RS's location, SH, LG, MV; PM-0006-11, -12, -18).
- netcode-architect: ADR 0001:1293 and the aarch64 question above (which rows, how each stays sound), in its text.
- rust-reviewer: ADR 0004 part 7 and S7 against an ARM load generator (and SH if HZ-SH (a)); at most a sentence.
- engine-architect (a): ADR 0002's arm64 text and R5-CI-OQ7; at most a sentence.
- engine-architect (b): README's OQ-0006-1 and -2 rows and the counts; SPIKE_PLAN's machines paragraph, item 1's
  "Depends on" and the open list; ENGINE_OVERVIEW only if a line changes.
- Then: docs-keeper (STATUS: the Hetzner check, the counts, "For Mustafa" without the console check; high); the
  report and appendix (§2 his words, §9 a row per place, the notes); the close as before (r4 conditions 7–9).

## For Mustafa after this round
The console check is done (item 1 leaves "For Mustafa"), unless HZ-SH (c) asks him for one more check.

## Addendum (about 18:40): his correction and his two answers
Verbatim: `inputs/HETZNER-CORRECTION-mustafa-2026-11-03.md` (with the session's description of his screenshot) and
`inputs/HETZNER-ANSWERS-mustafa-2026-11-03.md` (the session's two questions and the options he chose). They change the
plan as follows (each for the advisor to confirm):
1. **Two game servers.** RS, the CCX23 (dedicated, x86), runs only the measurement runs: "tick timing, the 60 Hz margin
   and rooms per VM". A development server, the CCX13 (dedicated x86, 2 vCPU, 8 GB; 43.49 EUR a month, 0.0697 an hour,
   excluding VAT, R6-HOST-F15; €0.083/h on his screenshot), runs development and "does it work" runs. "Each run's plan
   says which server it needs." Proposed: a new machine code in the keys (for example DS) only if a row needs it;
   otherwise the keys say which rows are measurement rows and leave the rest to each run's plan.
2. **The location: Nuremberg (`nbg1`)**, his answer, for the game servers and the simulated clients (OQ-0006-2
   answered). Whether the CAX21 "(or another client VM)" can be ordered there "is then checked in the proof test's
   plan" (his chosen option). His first note's Falkenstein facts stay as what his check found there.
3. **HZ-LOCATION** is settled by his answer; **HZ-OQ1** follows: the CCX23 orderable (his check), the client VM's order
   in Nuremberg checked in the proof test's plan (answered, or answered in part: for the advisor).
4. **HZ-SH** now asks whether a shared host is still measured beside RS for rooms per VM, given that he lists rooms per
   VM among the CCX23's measurement runs; in Nuremberg his screenshot shows the Cost-Optimized line with both x86 and
   Arm64 ("Limited availability"), so a CX33 may be orderable there (to be checked like the client VM).
5. **Costs:** the cap per run comes from its planned hours on the server its plan names (SPIKE-COST), so the CCX13's
   hours replace the CCX23's in every run that measures nothing.

## Addendum 2 (about 18:50): his second correction and his answer on the CCX13
Verbatim: `inputs/HETZNER-CORRECTION-2-mustafa-2026-11-03.md` (about 18:48: "both test servers will be in Falkenstein, not
Nuremberg: a CCX23 for the game server and a CAX21 for the 48 simulated players. I created both once to try the console
and deleted them right away. They'll be created fresh on run day, so their IP addresses will be new then.") and
`inputs/HETZNER-ANSWERS-2-mustafa-2026-11-03.md` (the CCX13 dropped; the option he chose, in the session's wording:
"Every server run uses the CCX23 (the measurement-only rule goes); the rig is just the CCX23 and the CAX21."). The round
launched at about 18:47 was stopped before any block was written (its files hold none). What changes, for the advisor:
1. **The location is Falkenstein (`fsn1`)**, in his words; OQ-0006-2 answered by them (not by the option text).
2. **OQ-0006-1:** both machines were created there ("I created both once to try the console and deleted them right
   away"): the CCX23 and the CAX21 can be ordered in Falkenstein; the CX33 cannot (his first check). Answered?
3. **No DS:** every server run uses the CCX23 (RS); his correction's "used only for the measurement runs" and the CCX13
   are superseded by his later answer; nothing of DS goes into the drafts.
4. **SH:** the CX33 cannot be ordered in Falkenstein, so by HZ-SH SH leaves the proof test and K3 waits (no shared x86
   plan is checked there unless the advisor asks for one).
5. **MV:** "the rig is just the CCX23 and the CAX21" (the option he chose): no separate metrics VM; where the metrics
   stack runs (never on RS) is the proof test plan's? 
6. **New addresses each run day:** the run records RS's and the client VMs' addresses on the day (the per-address
   connection cap raised on LG's addresses, the firewall's sources, the WebSocket leg's DNS record and certificate);
   each author checks that no sentence assumes a fixed address.
7. **Counts:** if OQ-0006-1 and -2 are both answered: 115 numbered, 95 open (ADR 0006: 21 numbered, 16 open); pass
   marks 341.
