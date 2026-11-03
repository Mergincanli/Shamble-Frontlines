# Plan revision 5: record Mustafa's design answers in README and STATUS (v2-decision-drafts, PR #33)

2026-11-03, about 17:31 Amsterdam. PR #33 is not merged, so his answers go in this PR, as he asked: "Record them in the
README's design questions and in STATUS (in PR #33, or a docs follow-up if it's merged). They don't change the spike
order." His words, verbatim: `inputs/DESIGN-ANSWERS-mustafa-2026-11-03.md` (about 17:28).

## Scope
- **README** (engine-architect's synthesis): each row his answers touch records them, word for word, marked as
  answered, answered in part, leaning or still open as his words say:
  - "Decisions the drafts ask for": WEAPON-UNLOCKS (OQ-0005-AB5) row 49, leaning, not final; carry slots
    (OQ-0005-AB1) row 50, answered, with his "Assumed, to confirm: the class's own abilities stay"; looted ammo
    (OQ-0005-AB2) row 51, still open, with the Advisor's lean; levelling (OQ-0005-AB3) row 52, answered; reference
    machines (OQ-0003-1) row 53, answered in part (classes chosen, both borrowed; the machines named when borrowed);
    fragments as damage row 58, answered (visual only for now); OQ-0005-AB11 row 111, answered (numbers tuned later);
    OQ-0005-AB14 row 114, answered (3 s; firing ends it early is the Advisor's pencil, which Mustafa may veto);
    OQ-0005-AB15 row 115, answered ("enemies can see a player's build"); OQ-0006-15 (DOMAIN) row 136, answered
    (shamblefrontlines.com or the closest free variant, bought by Mustafa at Cloudflare).
  - "Design questions": one sentence recording the name ("amplifications", "amps", for the in-match picks) and what
    stays open (faction asymmetry, the unit list first, beside OQ-0005-CD5's faction design session; the war around
    the matches).
  - The open count (README:23–25): it counts the drafts' own entries, so it stays 98; one sentence says which of those
    README now records design answers for, and that each draft takes them up in its next revision.
- **STATUS** (docs-keeper, in docs-keeper-4): a "Design answers (Mustafa, 2026-11-03)" entry, one line per answer,
  pointing to the appendix §2 for the words; the still-open ones in "Next".
- **The report and appendix** (the session): §2 the words; the report's "At the stop" names the design answers; its
  "Decisions needed" drops DOMAIN (answered) and shows REF-MACHINES answered in part; "For Mustafa" becomes: buy
  shamblefrontlines.com at Cloudflare; borrow the two machines and name them.
- **Not touched:** the six drafts' own entries (ADR 0005's AB1, AB2, AB3, AB5, AB11, AB14, AB15; ADR 0003's OQ-0003-1;
  ADR 0006's OQ-0006-15; ADR 0004's fragment question and OQ-0003-7): his scope is README and STATUS; README's rows say
  the draft's entry takes the answer up at its next revision. No spike order change. No glossary change (the name
  "amplifications" is recorded in README and STATUS, not in GLOSSARY).

## Steps
1. (session) The words saved verbatim; this plan; the advisor reviews it.
2. (workflow) engine-architect writes the README blocks, in the same fix round as fixcheck-e's findings (one README
   unit, so README's edits stay one author's, in file order).
3. (docs-keeper-4) STATUS, with the design answers added.
4. (fixcheck-f, high) a fresh verifier on the fix round and these blocks: every quote word for word, attribution
   (Mustafa's; the Advisor's lean and pencil marked as the Advisor's), nothing over-read ("leaning", "not final",
   "assumed, to confirm", "still open" kept), README's counts.
5. Then the close as plan revision 4 sets it (conditions 7–9).

## Decision for the advisor
- **DRAFT-ENTRIES:** (a) the drafts' entries stay as they are in this PR, README says each draft takes the answer up at
  its next revision (recommended: his scope, and no ADR text changes after the stop's recording was verified); (b) each
  draft's author adds one "Answered by Mustafa (2026-11-03)" line to its entry now (more edits, another verification).

## Evidence, risks
- Evidence: fixcheck-f's verdict; the scripts; CI.
- Risk: README and the drafts disagree on these questions until each draft's next revision; README says so in one
  sentence, and the drafts' count stays the count of their own entries.
