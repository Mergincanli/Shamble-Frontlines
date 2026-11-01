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
- **Settled inputs:** the stack (WebTransport with a WebSocket fallback, our own netcode, server authority: STATUS,
  "Decisions pending"; brief §1); MODES-3 (a); TICK-30; SLICE-ORDER and its three conditions (STATUS, "Decisions
  pending"); SEC-SPREAD (a); ABL-PREDICT (waits for a Feel Lab test); BASELINE-WHEN (b); UPGRADE-SCOPE (Mustafa,
  2026-10-31); SPIKE-SCOPE (a); PREVIEW-WALL (a); [the foundation](../research/GLOSSARY.md#the-foundation) and
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

