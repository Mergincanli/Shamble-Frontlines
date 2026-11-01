# ADR 0001, third fix round (netcode-architect): the stack cited by its lasting record

Finding: verifier-completeness-findings.md item 14 (the advisor's condition 12). Doc at 57c43c2. BEFORE texts copied
by script from the doc; each occurs exactly once; applying all three in order was tested on a copy (FIX 1 and FIX 2 keep
the line count, so the line numbers that docs/adr/README.md and the pass marks cite do not move; FIX 3 adds two lines
after line 1500, which nothing cites).

### FIX 1
FILE: docs/adr/0001-multiplayer-netcode.md
BEFORE:
- **Settled inputs:** the stack (WebTransport with a WebSocket fallback, our own netcode, server authority: brief §1);
  MODES-3 (a); TICK-30; SLICE-ORDER and its three conditions (STATUS, "Decisions pending"); SEC-SPREAD (a); ABL-PREDICT
  (waits for a Feel Lab test); BASELINE-WHEN (b); UPGRADE-SCOPE (Mustafa, 2026-10-31); SPIKE-SCOPE (a); PREVIEW-WALL
  (a); [the foundation](../research/GLOSSARY.md#the-foundation) and
AFTER:
- **Settled inputs:** the stack (WebTransport with a WebSocket fallback, our own netcode, server authority:
  STATUS, "Decisions pending"; brief §1); MODES-3 (a); TICK-30; SLICE-ORDER and its three conditions
  (STATUS, "Decisions pending"); SEC-SPREAD (a); ABL-PREDICT (waits for a Feel Lab test); BASELINE-WHEN (b);
  UPGRADE-SCOPE (Mustafa, 2026-10-31); SPIKE-SCOPE (a); PREVIEW-WALL (a); [the foundation](../research/GLOSSARY.md#the-foundation) and
END

### FIX 2
FILE: docs/adr/0001-multiplayer-netcode.md
BEFORE:
B (WebTransport over HTTP/2) and D (no fallback): settled by the stack (WebTransport with a WebSocket fallback, brief §1).
AFTER:
B (WebTransport over HTTP/2) and D (no fallback): settled by the stack (WebTransport with a WebSocket fallback: STATUS, "Decisions pending"; brief §1).
END

### FIX 3
FILE: docs/adr/0001-multiplayer-netcode.md
BEFORE:
- `docs/STATUS.md`, "Decisions pending": SLICE-ORDER, MODES-3, PREVIEW-WALL, SEC-SPREAD, ABL-PREDICT,
  BASELINE-WHEN, TICK-30, the five carry-forwards, SPIKE-SCOPE, R7-MARKS and the ADR-001 entry; UPGRADE-SCOPE as
  Mustafa wrote it on 2026-10-31 (recorded in STATUS with that day's decisions).
AFTER:
- `docs/STATUS.md`, "Decisions pending": the stack (the drafts brief's settled inputs), SLICE-ORDER, MODES-3,
  PREVIEW-WALL, SEC-SPREAD, ABL-PREDICT, BASELINE-WHEN, TICK-30, the five carry-forwards, SPIKE-SCOPE, R7-MARKS and
  the ADR-001 entry; UPGRADE-SCOPE as Mustafa wrote it on 2026-10-31 (recorded in STATUS with that day's decisions).
- `CLAUDE.md`, "The v2 rebuild", and `docs/V2_PHASE0_BRIEF.md` §0 (line 9): the stack's netcode part as decided (our
  own netcode: server authority, WebTransport with a WebSocket fallback).
END

### NOTE A (layout, no action needed)
FIX 1 keeps both `STATUS, "Decisions pending"` citations on one line each, so a line-based check finds them; the price is
one 134-character line (the doc already has 57 prose lines over 120, line 44 just below at 130). FIX 2 stays one line
(151 characters) on purpose: a wrap would add a line, and lines after it are cited by number (next note); on GitHub a second
line under a table may also render as a table row (NOTE C).

### NOTE B (pre-existing, outside this finding; not fixed here)
Line numbers that point into ADR 0001 are already stale at 57c43c2 (unchanged since 45c957f), so a re-pin is due once all
fix rounds land. `docs/adr/README.md` cites OQ-0001-1, -2, -3, -4, -5, -7, -8, -9, -10, -11, -12 at lines 840, 846, 856,
861, 867, 881, 886, 890, 896, 900, 905; they stand at 847, 854, 864, 869, 875, 889, 894, 898, 904, 908, 913. Inside the
doc, the pass marks' "the draft, … (line N)" pointers are off too, for example PM-0001-4 "budgets row (line 527)" (the
row is at 533), PM-0001-18 "failure modes (line 542)" (B1's row is at 548), PM-0001-48 "OQ-0001-7 (lines 881–885)" (now
889–893). FIX 1 and FIX 2 keep the line count, so they add no drift; FIX 3 moves only lines 1501–1503, which nothing cites.

### NOTE C (pre-existing, outside this finding; not fixed here)
Five plain lines in ADR 0001 sit directly under a table row with no blank line: 711, 718, 726, 734 and 783 (the other
five ADRs have none). Under the GFM spec's tables extension a table ends only at a blank line or another block, so on
GitHub each of these lines likely renders as one extra table row instead of a sentence (high confidence from the spec;
not checked on GitHub this round, no fetch; pandoc 3.1.3's gfm reader here renders it as a paragraph). Fix: a blank line
before each, done together with NOTE B's re-pin, since each blank line moves the lines after it.
