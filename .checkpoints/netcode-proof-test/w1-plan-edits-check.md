# Test-engineer: check of the wave-1 plan-text batch (W1 rulings + NETEM-BAR (a))

Read-only on the plan files. Base 20702fdb948fee31d0a0a8261894aa4f91da59e7. Started 2026-11-11.

## Plan
1. Read advisor-u4-wave1.md whole (done: 626 lines) and history.md :292-296 (NETEM-BAR (a), verbatim).
2. Write my own numbered list of every ordered change (below), before reading w1-plan-edits.md.
3. Compare with the editor's list (w1-plan-edits.md) and with `git diff 20702fd -- <file>` for the four files.
4. For each item: present, word for word, right place, Mustafa's words exact.
5. Whole diff: nothing unordered, nothing out of scope, no stale cross-reference, no contradiction left.
6. Findings with severity and exact fix; return the schema.

## My independent list (from advisor-u4-wave1.md only)
Mustafa's NETEM-BAR (a), verbatim (history.md:292-295): "NETEM-BAR (may the WebSocket pass bar loosen when real Linux
disagrees with the model): (a). The bar stays capped as WS-BAR says. If a WebSocket row fails because the kernel is
slower than the model, report it to me as the kernel's, with the netem numbers, and I decide."

u4-build-plan.md:
- L1 W1-S1 (:99-105): Lanes' freeze bullet (:824-831), append "**Lifted for one change (the advisor's W1-S1 ruling,
  2026-11-10):** ... after U4a has merged."
- L2 W1-S1 (:106) + W1-PUB (:426-432): §4.1 npt-sim row (:184) gains the plain-data types list (quoted), "the owner-only
  own-state kind (W1-S1)", the records-to-volumes function (W1-U4D A-M1), `InputReports`' refusal records (W1-U4D N-M2),
  Scene's and World's accessors "as listed above" (words partly the editor's).
- L3 W1-PUB (:418-423): §4.1 npt-proto rows gain one row (quoted, with its Why cell after the `|`).
- L4 W1-PUB (:424-425): npt-server's row gains "`Dropped`, ... `BITES`".
- L5 W1-PUB (:433) / U4E-Q3 (:453-455): npt-room's row gains "bites builds only: `BITES`" (U4E-Q3).
- L6 W1-U4D N-M2 (:215-225): §4.2 (:240-242) sentence replaced (quoted, long).
- L7 W1-N2 (:321-325): §4.4 (:273-274) bullet replaced (quoted).
- L8 W1-U4D T-M2 (:189): §4.9 (:336) "at most 1 MB" -> "at most 1 MB (10^6 B, as PM-0001-7 judges it)".
- L9 W1-U4D N-M2 (:226-228): §4.10 (:356-358) bullet appended (quoted).
- L10 W1-U4D (:296-299): §4.13 SS bullet (:415-420), after "DM_Tri's extent", gains (quoted).
- L11 W1-N2 (:326-330): §4.14 -58/lt-tick-offset row (:454) replaced (quoted, a table row).
- L12 W1-U4D (:300-301): §4.15 (:478) generator text replaced (quoted).
- L13 W1-U4D TUNE-2 (:267-268): §5 U4d D-8 (:612-613) gains (quoted).
- L14 U4E-Q8 (:484-485): §5 U4e (:692) clippy.toml text replaced (quoted).
- L15 U4E-Q1 (:445): §5 U4e Bites line gains "lt-cmd-seq and lt-press (the room's compares)".
- L16 U4E-Q10 (:491): §5 U4e Bites line gains "server-echo (-62, the room's routing)".
- L17 U4E-Q11 (:494-495): §5 U4f Builds (:775) text replaced (quoted).
- L18 W1-N2 (:331) / U4E-Q2 (:452): §5 U4r (:737) delete "-58 for 10 s at P0 and P3, every counter a few steps below its
  wrap; its six bites (§4.14)."
- L19 W1-N2 (:332-334) / U4E-Q2: §5 U4g Marks moved (:791-795) add "-58 (10 s at P0 and P3, ...)" (quoted).
- L20 W1-N2 (:335-338): §10 readings preamble (:927-928) gains the -58 reading (quoted).
- L21 W1-N2 (:335-338): §11 (:1028-1033) gains the same -58 reading.
- L22 W1-U4D N-M2 (:236-239): §10's list gains the -21 press-twin reading (quoted), beside Q16's clamp.
- L23 W1-U4D N-M2 (:236-239): §11 gains the same -21 reading.
- L24 W1-U4D TUNE-2 (:265-266): the readings PR's Q16 text adds "for fire origins inside the grid's range, as in every
  scene" (where Q16's text lives in §10/§11).
advisor-u4.md:
- L25 W1-U4D N-M2 (:229-231): one line under the U4 verdict: "Condition 1's 'Any per-soldier input state the sim keeps
  holds what the server accepted, not what the client claimed' is amended by the W1-U4D ruling (advisor-u4-wave1.md)
  for the press counters."
u4s-carries.md:
- L26 W1-U4D N-M2 (:232-235): new carry (U4f) "-21's press twins: ..." (quoted).
- L27 W1-U4D A-M2 (:242-247): new carry (U4g) "The predicting World's reset: ..." (quoted).
- L28 W1-U4D C13 (:274-276): new carry (U5) "Carry 13's second half: ..." (quoted); "U5's plan states where its
  allocator's `unsafe` lives under ADR 0002:264-265" sits outside the quotes (an instruction to U5's plan).
- L29 W1-U4D (:292-294): new carry (U4r) "The contract test's report gives, ..." (quoted).
- L30 W1-N2 (:347-349): new carry (U4e) "An event older than 255 ticks cannot be coded: ..." (quoted).
- L31 U4E-Q4 (:462-465) / Q10 (:491): carry 24's text changes (quoted, with a leading "...").
- L32 U4E-Q5 (:472): carry 40 names the counter (no-Rewind-outside-the-ring count); words not given.
plan.md:
- L33 W1-PUB (:414-416): §4.7's npt-sim line gains "`SimRng` (crate-private: no other crate draws from the sim's
  streams; the advisor's W1-PUB ruling)".
- L34 W1-PLAN TCP-FREEZE (:521-527): U8 row (:1247), after "...since this container has no tc", insert (quoted).
- L35 NETEM-BAR (a): beside L34 in the U8 row, his words verbatim with code and date.
- L36 W1-PLAN (:528-529): §15 R-25 row (:1417), after "against RS's at R1 (S14)", add "; if they disagree, netem wins and
  the model is not repaired (TCP-FREEZE)".
- L37 W1-PLAN SQUASH-CITES (:505-512): plan.md:1436-1438 bullet replaced (quoted).

Not ordered in these four files (my reading): W1-S1's code, catalogue, U4e brief; W1-U4D's code fixes, TUNE-1's
conditions (headers, report), C13's report line; W1-N2's marks.json, catalogue, u4j-brief; W1-PUB's two nits (code);
U4E-Q6's two conditions (no file named); Q7, Q9 (no ruling); W1-TARGET (wave-2 briefs); the TCP-FREEZE relay.
Open question to check: U4E-Q6's conditions and TUNE-2's/TUNE-1's conditions have no named home in the plan files.

## Comparison log (one line per item, appended as checked)
