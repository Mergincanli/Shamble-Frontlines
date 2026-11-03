# Plan revision 5: Mustafa's design answers, his answers to the escalations, COMMUNITY-SERVERS, and the last fix round (v2-decision-drafts, PR #33)

2026-11-03, about 17:34 Amsterdam. PR #33 is not merged, so everything goes in this PR. His words, verbatim:
`inputs/DESIGN-ANSWERS-mustafa-2026-11-03.md` (about 17:28) and `inputs/ANSWERS-2-mustafa-2026-11-03.md` (about 17:31).
(A first version of this plan, design answers only, went to the advisor at 17:31; the review was stopped at once when
his second message arrived, so one review covers both.)

## What his messages decide
1. **Design answers, recorded in README and STATUS** ("Record them in the README's design questions and in STATUS"):
   carry slots (OQ-0005-AB1, with "Assumed, to confirm: the class's own abilities stay"); looted ammo (OQ-0005-AB2,
   still open, the Advisor's lean); levelling (OQ-0005-AB3); the name "amplifications" ("amps") for the in-match picks;
   WEAPON-UNLOCKS (OQ-0005-AB5, leaning, not final); Banzai vs Riot Shield (OQ-0005-AB11, numbers tuned later);
   upgrade visibility (OQ-0005-AB15: enemies can see a player's build); spawn protection (OQ-0005-AB14: 3 s; firing
   ends it early is the Advisor's pencil, which Mustafa may veto); fragments as damage (visual only for now); still
   open: faction asymmetry (the unit list first) and the war around the matches.
2. **The four escalations of the stop, answered** (they close questions the drafts already record as answered in part,
   so the drafts' entries take the answers):
   - REF-MACHINES (OQ-0003-1): minimum, a typical student laptop with integrated graphics; recommended, a desktop with
     an RTX 3060/4060-class card; Mac, a MacBook Air, M1 or newer, for Safari; all borrowed; he names the machines
     before the look proof test; until then his laptop's integrated graphics (Brave "Power saving") stands in for the
     minimum, for rough checks only; "the netcode proof test doesn't measure client frames".
   - LOOK_V2-§8: kept, Blender for the test models only; when the asset research's export part reports, §8 says
     "Spline or Blender" (LOOK_V2 is his and the Advisor's file: not edited here).
   - SPIKE-COST (OQ-0006-20): one cap per run, from its planned hours, written in that run's plan for his OK; he
     creates and deletes the VMs.
   - DOMAIN (OQ-0006-15): shamblefrontlines.com, registered by him at Cloudflare (his second message drops "or the
     closest free variant").
3. **COMMUNITY-SERVERS** (new; "keep the door open, don't design it"): a new open question in ADR 0006 (OQ-0006-21)
   and a README row. Its text records his constraints: the room server stays one self-contained program with a config
   file (players, length, rules), checks the game's version and content hash, and marks each match as official or
   community, so progression can count on official servers only. Each draft's author checks that nothing in its text
   locks this out and adds at most a pointer where its text describes the room server or the match record.
4. **Verification stops after this fix round** ("after e2b, e3b and their fix round, stop. No further re-check rounds;
   anything left goes into the PR as notes"). So no fixcheck-f: the session runs the scripts (write guard,
   dispositions, glossary, tables, counts, the gate) and lists what is left in the report as notes. The advisor's
   review of the final head before the merge stays (CLAUDE.md's merge rule; it is a review, not a re-check round).

## The fix round (one round, two workflows, every author on its own text, xhigh)
- test-engineer: e1 F1 (PM-0002-49, the reading mark); e2 F2 (ADR 0006 key, the loss reading mark); e2 F1's key S
  place (ADR 0003:1122–1123); e3b 1 (PM-0005-4's presented kinds as CD-2 names them); the machine keys of ADR 0001 and
  0003 (MAC a MacBook Air, M1 or newer; the interim stand-in for ML, rough checks only); e2b F1 (= e3b 1) and F7's
  PM-0005-15 half (the client entries keyed by simulation ids among the client-content values).
- graphics-engineer: e2 F1's part 8 place (ADR 0003:404–405); OQ-0003-1 answered by REF-MACHINES (e3b 4's labels).
- infra-engineer: e2 F3 (three reading marks); OQ-0006-20 and OQ-0006-15 answered; the new OQ-0006-21
  (COMMUNITY-SERVERS) and the check that ADR 0006 locks nothing out.
- netcode-architect: COMMUNITY-SERVERS' check of ADR 0001 (part 11's rooms, directory and match record), a pointer to
  OQ-0006-21 at most.
- engine-architect (a): e1 F2 (ADR 0002's "the real sim"); LOOK_V2-§8's answer in ADR 0005's LOOK-TEST-INPUTS input;
  e2b F2 (Appendix A's counts: answered 10, open 7), F3 (two reading marks), F5 ("where blockouts ... are enough",
  upheld by its skeptic), F6 (the mark's date), F7's CD-6 half (the client entries keyed by simulation ids).
- engine-architect (b): README: the design answers' rows and one sentence (the name, the still open ones), the
  escalation answers' rows, the COMMUNITY-SERVERS row, the counts; SPIKE_PLAN: e3b 3, 4 and 5, the machines (MAC, the
  stand-in), the cost cap's rule in the proof test's "Depends on", LOOK_V2-§8; ENGINE_OVERVIEW only if a line changes.
- rust-reviewer: e1 F3 (ADR 0004 phase 0's sign-off), e3b 2 (phase 1's order as SPIKE_PLAN gives it).
- game-designer: e2b F4 (OQ-0005-AB12: name the specific claims the record relies on for the later check, as its
  skeptic corrected it: the GAS findings AB-1 to AB-3 cite, F4's summed multipliers and F17's cooldown quote among
  them; or say they are listed when the documentation can be read).
- The design answers do not change the drafts' own entries (DRAFT-ENTRIES below). Counts after the round, by script:
  the escalation answers close OQ-0003-1, OQ-0006-15 and OQ-0006-20, and OQ-0006-21 is new, so 115 numbered and 96 open
  if all three count as answered (OQ-0006-1 stays open); pass marks stay 341.

## Then
docs-keeper-4 (STATUS, with the design answers, the escalation answers, COMMUNITY-SERVERS and the next steps); the
report and appendix (§2 the words; "At the stop"; "Decisions needed": none left from the stop; "For Mustafa": the
console check, registering shamblefrontlines.com at Cloudflare, borrowing and naming the machines before the look test,
each run's cost cap in its plan; his merge go after the advisor's APPROVE); the close of plan revision 4 (conditions
7–9): stop the saver, the deletion commit, ready, needs-mustafa, the advisor on the final head, his go, the squash.

## Decisions for the advisor
- **DRAFT-ENTRIES:** (a) the design answers stay in README and STATUS only, and README says each draft takes them up at
  its next revision (recommended: his scope); (b) each author adds one line to its entry now.
- **OQ-0003-1:** answered (the classes, borrowed, named before the look test) or answered in part until the machines
  are named. Recommend answered, with the naming as his action before the look proof test.
- **"The netcode proof test doesn't measure client frames":** recorded as his words beside the proof test's ML and RD
  rows (they measure client CPU, not frames); which rows use the stand-in is for the proof test's own plan. Recommend
  no row changes now.
