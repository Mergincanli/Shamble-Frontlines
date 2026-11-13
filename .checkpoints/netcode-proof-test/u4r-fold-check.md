# U4r fold check (adversarial check of u4r_fold.py)

## Plan
1. Hash the inputs (ruling, brief, te-m3, te-m3-check, the script) against the task's stamps.
2. Read the script whole; run the dry run (`python3 -I .checkpoints/netcode-proof-test/u4r_fold.py`), read its whole output.
3. Read the ruling :74-382 and :415-420; list every item L1 includes/excludes.
4. For each item compare the diff hunk to the ruling's text word for word (white space aside).
5. Find every "by U4R-Qn" and every "P0 1" in the brief; say how the fold treats each (:178, :357-358).
6. Check section 10's new table rows against the ruling's lines; check stamps vs sha256; check no line outside items changed.
7. Verdict + findings.

## Stamps
- ruling 35f281bf… (463 lines) OK; brief bcaef383… (1,625) OK; te-m3 5ecc3e23… OK; te-m3-check 1ffd0294… OK.
- script checked: u4r_fold.py sha256 a9196126b2efdd9b9d6ad89ee96735fbc4abfcd6daeb8debc708e972d0802d4e (working tree, M).
- dry run: exit 0; 51 changes; 1,625 -> 1,662 lines; every hunk is an item's (no stray hunk).

## Step log
- Steps 1-3 done (hashes, script read, dry run read whole, ruling :74-382/:415-420 read).
- Hunks compared word for word: A1.1-A1.5, A2.1-A2.9, A5.1-2, A6, A7.1-2, A8, Q7, Q8.1-2, Q9.1-Q9.15, Q14.1-4,
  S4.1-S4.5: each new phrase equals the ruling's (white space aside). Mechanical check: every old/new phrase taken
  from the ruling is in the ruling's flattened text, but the MAIN anchors (A1.4 anchor, A1.5 form, A2.8 built, S4.5 ends).

## "by U4R-Qn" in rev 3.1 (grep, flattened)
- :20 C27 record ("reads "by U4R-Q14"") - a record of rev 3.1, left: right.
- :250 section 1 item 5 (the rule itself, quoted) - left: right.
- :385-386 "or two by U4R-Q9" - Q9.3 replaced. OK.
- :439, :440, :446 - inside Q14.1's span, replaced. OK.
- :476 2.2 "And, by U4R-Q14 (b), `None` at a step..." - NOT folded, not logged. FINDING.
- :680 3.2 row 62 "with (h)'s check by U4R-Q2 (b)" - NOT folded, not logged. FINDING.
- :859 "(or three, by U4R-Q9)" - Q9.7 replaced. OK.
- :1605 section 12 item 4 (the rule, quoted) - left: right.

## "P0 1" in rev 3.1
- :19 C27 record - left (a record).
- :442 2.1's lead bullet - Q14.1 -> "P0 2". OK.
- :1459 U4R-Q14 text, carry 74's one-way formula's numbers - kept as asked. OK.
- :1460 U4R-Q14 text, "The round-trip form gives P0 1" - kept as asked; :357-358 says the fold reads the ruled
  numbers, but the Q14 table row carries no ruled number (its cell is the arithmetic's premise sentence). FINDING.

## Findings
- F1 High: :476 and :680 "by U4R-Qn" not folded, not in the script's log or the record's "Not folded here"
  (ruling :178; brief :250-251, :1605). No ruling words exist for them: to the advisor (L1 :419-420).
- F2 Medium: Q14's row cell = first sentence = the arithmetic's premises (:289-294), not the ruling: no lead, no
  P0 2, no A4 rule; the question below keeps "P0 1" (:1460). Fix: cell from :301-302 and :322-323 by markers.
- F3 Medium: WENT sentences (main session's words) inside the "Ruling" cell for Q7, Q8, Q9, Q14, before the cite;
  the record says only the third cell is the main session's. Fix: move to the third cell.
- F4 Medium: S10.open relabel "**Open**" (rev 3.1 :1284) is a changed brief word the ruling does not give; L1
  :419-420 sends it to the advisor. The lead-in :1276 "(the questions as rev 2 asked them stay in ed973c9...)" no
  longer fits the new rows and is left.
- F5 Low: unlogged transforms: P16's \" -> " (script :246), Q6 row's | -> \| (script :370). Right, but not in MAIN.
- F6 Low (not the fold's): the saver's autosave bb412c8 committed __pycache__/u4r_fold.cpython-313.pyc (fold agent's
  slip 2; not gitignored).

## Checked and right (no finding)
- Every other hunk: the ruling's words exactly; rewraps word-preserving (norm asserts; my read of every hunk); no
  hunk outside the items; table rows 3 cells each; non-in-place rows = the ruling's whole paragraph after its label.
- Stamps: te-m3 5ecc3e23, te-m3-check 1ffd0294, u4r-check 35f281bf equal the files' sha256.
- 7.3 arithmetic (the ruling's): 13,500 s, 405,000 ticks, R rows, 27 runs / 810 s: hold.
- Q9.1+Q9.2 give "SS P0 seed 4 (TC B1 ... U4R-Q9 (b)) (the -30 pre-check's source only)": the ruling's words exact.
- Out of scope, kept: Q9 plan text and carry line :250-262, carry 74 words :324-330 (L2, 8bcbf36), A3/A4, A9.

## Questions for the advisor (agreed needed)
- the words for :476 and :680; the "**Open**" label and :1276's lead-in; the stale list (D13 :853/:914; 7.4
  :1105-1107; 7.3 :1085-1086; 2.1 (i)'s plan quote :384-385 vs L2's plan; :806 "if ruled"; :1282 "keeps (a) and (b)");
  the rows' quoted carries cites (:491, :548) moved by L2 to :494, :554.

## Status
- DONE: verdict FAIL (F1 High; F2-F4 Medium; F5-F6 Low). Result returned.
