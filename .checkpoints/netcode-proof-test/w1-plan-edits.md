# W1 plan-text edits: the advisor's wave-1 rulings applied to the plan files (plan-text editor, 2026-11-11)

Agent: general-purpose, opus, xhigh, at the main session's direction (workflow step). Source: advisor-u4-wave1.md (626
lines, read whole), plus Mustafa's NETEM-BAR (a) of 2026-11-10 (history.md:292-296, verbatim). Base for every diff:
commit 20702fdb948fee31d0a0a8261894aa4f91da59e7. Edit tool only; one change at a time.

Base sha256 (checked at start, all equal to the base):
- plan.md b039c8cd47eb22b93417794883222b148343778efbc34d61fe6b1467721872c6
- u4-build-plan.md 0d69332c3c60c83c9328504259c279609b546e15cc5510f528346471aabc3b84
- u4s-carries.md 505e54d6c0259092f08767a4052648898040437b4f2f01a8c47877adadff9c29
- advisor-u4.md 7f49acc895122bc5c9a06b286f7469312b92d59207a740fbcd5f1f543b78fb63

## Plan: every ordered change found (code, the ruling's line in advisor-u4-wave1.md, target file and section)
u4-build-plan.md:
- E1 W1-S1 (:99-105): Lanes, the freeze bullet (:824-831): the "Lifted for one change" sentence appended.
- E2 W1-S1 (:106) and W1-PUB (:426-432): §4.1, npt-sim's seam-data row (:184) gains the own-state kind, the
  records-to-volumes function, the refusal records, the plain-data types and Scene's and World's accessors.
- E3 W1-PUB (:418-423): §4.1, npt-proto gains one row (after :191).
- E4 W1-PUB (:424-425): §4.1, npt-server's row (:194) gains its items.
- E5 W1-PUB (:433) and W1-U4E Q3 (:453-455): §4.1, npt-room's row (:193) gains "bites builds only: `BITES`".
- E6 W1-U4D N-M2 (:215-225): §4.2 (:240-242), the press-counter sentence replaced.
- E7 W1-N2 (:321-325): §4.4 (:273-274), the snapshot-header bullet replaced.
- E8 W1-U4D T-M2 (:189): §4.9 (:336), "at most 1 MB" gains "(10^6 B, as PM-0001-7 judges it)".
- E9 W1-U4D N-M2 (:226-228): §4.10 (:356-358), the twin's-command bullet gains the press sentence.
- E10 W1-U4D plan text (:296-299): §4.13's SS bullet (:415-420) gains the text after "DM_Tri's extent".
- E11 W1-N2 (:326-330): §4.14, the -58/lt-tick-offset row (:454) replaced.
- E12 W1-U4D plan text (:300-301): §4.15 (:478), the generator's home replaced.
- E13 W1-U4D TUNE-2 (:267-268): §5 U4d, D-8 (:612-613) gains the bound clause.
- E14 W1-U4E Q8 (:484-485): §5 U4e (:692), the clippy.toml line replaced.
- E15 W1-U4E Q1 (:445) and Q10 (:491): §5 U4e's Bites line (:711-712) gains two items.
- E16 W1-N2 (:331): §5 U4r (:737), the -58 line deleted.
- E17 W1-U4E Q11 (:494-495): §5 U4f's Builds (:775), the server-side checks replaced.
- E18 W1-N2 (:332-334): §5 U4g, Marks moved (:791-795), -58 added.
- E19 W1-N2 (:335-338) and W1-U4D N-M2 (:236-239): §10's readings preamble (:926-930) gains the two readings.
- E20 W1-N2 (:335-338), W1-U4D N-M2 (:236-239) and TUNE-2 (:265-266): §11 (:1028-1033) gains the two readings and
  Q16's clause.
advisor-u4.md:
- E21 W1-U4D N-M2 (:229-231): one line under the U4 verdict (the end of the verdict section, before "Rulings on the
  plan's questions"; the verdict's text stays as returned, as for NF-D4).
u4s-carries.md:
- E22 W1-U4D N-M2 (:232-235): new carry (U4f), entry 46, in a new "U4f" section.
- E23 W1-U4D A-M2 (:242-247): new carry (U4g), entry 47, in a new "U4g" section.
- E24 W1-U4D C13 (:274-276): new carry (U5), entry 48, in a new "U5" section.
- E25 W1-U4D new carries (:292-294): new carry (U4r), entry 49, in the U4r section.
- E26 W1-N2 (:347-349): new carry (U4e), entry 50, in the U4e section.
- E27 W1-U4E Q4 (:462-465) and Q10 (:491): carry 24's text from "`npt_client::BITES` grows with" replaced.
- E28 W1-U4E Q5 (:472): carry 40 names the counter (intent only: words mine).
plan.md:
- E29 W1-PUB (:414-416): §4.7's npt-sim line, `SimRng` gains its parenthetical.
- E30 W1-PLAN TCP-FREEZE (:521-527): the U8 row (:1247), the insertion after "since this container has no tc".
- E31 NETEM-BAR (a), Mustafa 2026-11-10 (history.md:292-296): the U8 row, beside E30, his words quoted.
- E32 W1-PLAN (:528-529): §15's R-25 row (:1417), the TCP-FREEZE clause.
- E33 W1-PLAN SQUASH-CITES (a) (:505-512): §16 (:1436-1438), the PR #40 bullet replaced.

Not placed in these four files (others' or later, or no text ordered): see "Skipped" at the end.

## Log (one line per change: code and ruling line; file and lines; done or skipped and why)
- E1 W1-S1 (:99-105): u4-build-plan.md Lanes freeze bullet, appended after "(the advisor's U4s ruling, condition 1 (d))."
  (base :831; now :831-836). Done, the ruling's words exactly. (This one log line was appended by a shell heredoc; every
  later notes line goes in by the Edit tool.)
- E2 W1-S1 (:106) and W1-PUB (:426-432): u4-build-plan.md §4.1 :184 (npt-sim's seam-data row, the row W1-S1 names).
  Done. Ruling words: "the owner-only own-state kind (W1-S1)", "the plain-data types `Placement`, ... `replay::Step`"
  (W1-PUB's quoted item, exact), "the records-to-volumes function (W1-U4D A-M1)", "`InputReports`' refusal records
  (W1-U4D N-M2)". Words mine: the accessor list in brackets after "`Scene`'s and `World`'s accessors" (W1-PUB's "as
  listed above", spelled out from :399-401: World's and Scene's items only), and the Why cell's "; the advisor's W1-S1,
  W1-U4D and W1-PUB rulings". W1-S1's item is placed once (W1-PUB repeats it). Not added: U4d's Weapon, Scene::weapon,
  Scene::health (out of scope); the other W1-PUB U4d items (StandIns, HitVolumes' constants, Recording's, Replayer's,
  carry 17's constants, the bites-only World items), since W1-PUB's §4.1 list names only the five groups above.
- E3 W1-PUB (:418-423): u4-build-plan.md §4.1, a new npt-proto row after the last npt-proto row (base :191; now
  :192). Done, both cells the ruling's words exactly (the "|" in the ruling's text is the cell boundary).
- E4 W1-PUB (:424-425): u4-build-plan.md §4.1 npt-server's row (base :194; now :195), appended after "`Schedule`,
  `Due`". Done, the ruling's words exactly; words mine: the Why cell's "; the advisor's W1-PUB ruling".
- E5 W1-PUB (:433) and W1-U4E Q3 (:453-455): u4-build-plan.md §4.1 npt-room's row (base :193; now :194). Done:
  "bites builds only: `BITES`" exactly; words mine: the Why cell's "; U4E-Q3 (the advisor's W1-PUB ruling)".
- E6 W1-U4D N-M2 (:215-225): u4-build-plan.md §4.2, the sentence "Any per-soldier input state ... next accepted
  press)." (base :240-242) replaced (now :241-248, rewrapped at 120). Done, the ruling's words exactly.
- E7 W1-N2 (:321-325): u4-build-plan.md §4.4, the snapshot-header bullet (base :273-274) replaced (now :279-283).
  Done, the ruling's words exactly.
- E8 W1-U4D T-M2 (:189): u4-build-plan.md §4.9 (base :336; now :345-346): "at most 1 MB" → "at most 1 MB (10^6 B, as
  PM-0001-7 judges it)", the line rewrapped (it reached 123 characters). Done, the ruling's words exactly.
- E9 W1-U4D N-M2 (:226-228): u4-build-plan.md §4.10, the twin's-command bullet (base :356-358), the sentence appended
  after "(the advisor's condition 1)." Done, the ruling's words exactly.
- E10 W1-U4D plan text (:296-299): u4-build-plan.md §4.13's SS bullet (base :415-420), the text inserted after
  "DM_Tri's extent (LEGACY_INVENTORY.md:153)" with "; " (placed after the extent's own citation so the cite stays with
  its subject; the ruling says after "DM_Tri's extent"). Done, the ruling's words exactly.
- E11 W1-N2 (:326-330): u4-build-plan.md §4.14, the -58/lt-tick-offset row (base :454) replaced. Done, the ruling's
  words exactly (its "|" are the cell boundaries).
- E12 W1-U4D plan text (:300-301): u4-build-plan.md §4.15 Sources (base :478): "a seeded generator in npt-sim's test
  support" → "a seeded generator in npt-sim's library (`Recording::synthetic`), since U4d2's step regenerates the
  recordings from the manifest", rewrapped. Done, the ruling's words exactly. Not changed (not ordered): §5 U4d's
  Builds still says "the synthetic recording generator (test support)" (base :591), a follow-up for the advisor.
- E13 W1-U4D TUNE-2 (:267-268): u4-build-plan.md §5 U4d, D-8 (base :612-613; now :628-630), the clause appended after
  "a clamped point sent again is not moved". Done; ruling's words exact; words mine: the joining "; all three" (the
  clause covers D-8's three statements).
- E14 W1-U4E Q8 (:484-485): u4-build-plan.md §5 U4e Builds (base :692; now :709): "room/clippy.toml gains the hash-map
  type bans" → "room's and client's clippy.toml gain the hash-map type bans (§4.11; U4E-Q8)". Done, exact.
- E15 W1-U4E Q1 (:445) and Q10 (:491): u4-build-plan.md §5 U4e's Bites line (base :711-712; now :728-730): after
  "lt-event-seq (§4.14)," inserted "lt-cmd-seq and lt-press (the room's compares), server-echo (-62, the room's
  routing),"; the line (already 134 characters at the base) rewrapped. Done, both items exact, no tag added.
- E16 W1-N2 (:331): u4-build-plan.md §5 U4r judged runs (base :737), the bullet "- -58 for 10 s at P0 and P3, every
  counter a few steps below its wrap; its six bites (§4.14)." deleted. Done.
- E17 W1-U4E Q11 (:494-495): u4-build-plan.md §5 U4f Builds (base :775-776; now :792-794): "the server-side checks
  (size, trailing bytes, wrong channel, the bucket)" → the ruling's text, exact; the next line rewrapped. Done.
- E18 W1-N2 (:332-334): u4-build-plan.md §5 U4g Marks moved (base :791-795; now :812-815), appended as the list's last
  item after "-56 item (6)'s -42 share per option" with "; ". Done, the ruling's words exactly (its closing period
  ends the list).
- E19 W1-N2 (:335-338) and W1-U4D N-M2 (:236-239): u4-build-plan.md §10's preamble (base :926-930), one sentence
  inserted after "(checkpoint :320-324)." and before "Changed in rev 2". Done: both readings in the rulings' words,
  each in double quotes (W1-N2's own semicolon needs a delimiter), W1-U4D's first ("beside Q16's clamp"). Words mine:
  "The advisor's wave-1 rulings add two readings, in their words: beside Q16's clamp," and "; and". The parenthetical
  code list before it is left as it was (it lists condition 11's six).
- E20 W1-N2 (:335-338), W1-U4D N-M2 (:236-239) and TUNE-2 (:265-266): u4-build-plan.md §11 (base :1028-1033). Done:
  TUNE-2's "for fire origins inside the grid's range, as in every scene" after "the clamp twin's points at the
  weapon's reach" (exact; words mine: the tag "; W1-U4D TUNE-2" beside "Q16's clamp"); W1-U4D's reading inserted right
  after Q16's clamp (exact); W1-N2's reading as the list's last item (exact), the "and" moved to it from Q26. Words
  mine: "six readings ... (the advisor's condition 11)" → "eight readings ... (the advisor's condition 11; its W1-U4D
  and W1-N2 rulings)", since the list now holds eight.
- E21 W1-U4D N-M2 (:229-231): advisor-u4.md, one line (two wrapped lines) at the end of the U4 verdict section, after
  "What stands as planned" and before "## Rulings on the plan's questions" (base :193-195; now :195-196). Done: the
  ruling's line exactly; words mine: the attribution "**Main session, 2026-11-11:**" (the verdict's returned text is
  left unchanged, as for NF-D4 at :395-396). No other line in advisor-u4.md is ordered by the rulings.
- E22 W1-U4D N-M2 (:232-235): u4s-carries.md, a new section "## U4f (clock and commands)" after the U4r section, entry
  46 (now :197-201). Done: the quoted carry exactly; words mine: the heading and the bold label
  ("N-M2 = R-M2 = A-C, -21's press twins (advisor-u4-wave1.md:197-235; the advisor's W1-U4D ruling)").
- E23 W1-U4D A-M2 (:242-247): u4s-carries.md, a new section "## U4g (prediction, interpolation and events)" after U4f,
  entry 47 (now :203-209). Done: the quoted carry exactly; words mine: the heading and the bold label.
- E24 W1-U4D C13 (:274-276): u4s-carries.md, a new section "## U5 (fuzzing and the counting allocator)" after "U4's
  report", before "For the advisor", entry 48 (now :253-256). Done: the quoted carry exactly, plus the ruling's next
  sentence ("U5's plan states where its allocator's `unsafe` lives under ADR 0002:264-265.") in brackets after it, as
  entries 35 and 39 carry a bracketed note; words mine: the heading and the bold label.
- E25 W1-U4D new carries (:292-294): u4s-carries.md U4r section, entry 49 after entry 40 (now :192-195). Done: the
  quoted carry exactly; words mine: the bold label ("N-M4 and TUNE-2 (...; the advisor's W1-U4D ruling)").
- E26 W1-N2 (:347-349): u4s-carries.md U4e section, entry 50 after entry 39 (now :178-181). Done: the quoted carry
  exactly; words mine: the bold label.
- E27 W1-U4E Q4 (:462-465) and Q10 (:491): u4s-carries.md entry 24 (base :152-154; now :152-157): the quote's text
  from "`npt_client::BITES` grows with" to its end replaced by the ruling's text (the "..." kept the opening clause
  as it was). Done, exact; words mine: the label's "; reworded by the advisor's W1-U4E ruling, U4E-Q4 and U4E-Q10" (as
  entry 7's "reworded by").
- E28 W1-U4E Q5 (:467-472): u4s-carries.md entry 40 (base :177-182; now :184-191): the ruling gives the intent only
  ("carry 40 name[s] the counter"), so one sentence is appended inside the quote, words mine: "The room's count of
  commands that `rewind_at` gives no Rewind, because the ring does not hold their claimed view time's tick, counts as a
  replacement in that setup condition: a pre-check leg with any is VOID (U4E-Q5)."; the label gains "; its last
  sentence added by the advisor's W1-U4E ruling, U4E-Q5". The rest of the quote is unchanged (its "holds by
  construction" is about the tolerance and the cap; the new sentence names the exception). Done.
- E29 W1-PUB (:414-416): plan.md §4.7's npt-sim line (base :584-585; now :584-586): "`SimRng`," → "`SimRng`
  (crate-private: no other crate draws from the sim's streams; the advisor's W1-PUB ruling),", rewrapped. Done, exact.
- E30 W1-PLAN TCP-FREEZE (:521-527): plan.md the U8 row (:1248 now; base :1247), inserted after "since this container
  has no tc" and before "; and DelayStream's self-test on RS". Done, the ruling's words exactly.
- E31 NETEM-BAR (a) (history.md:292-296; the task's order): plan.md the U8 row, right after E30's "R1's report records
  the comparison and the switch": "; Mustafa's NETEM-BAR (a), 2026-11-10: "The bar stays capped as WS-BAR says. If a
  WebSocket row fails because the kernel is slower than the model, report it to me as the kernel's, with the netem
  numbers, and I decide."". Done: his two sentences verbatim (checked by script against history.md :292-295 joined:
  equal); words mine: only the lead-in "Mustafa's NETEM-BAR (a), 2026-11-10:" (the code, the option and the date).
- E32 W1-PLAN (:528-529): plan.md §15's R-25 row (base :1417), "; if they disagree, netem wins and the model is not
  repaired (TCP-FREEZE)" added after "against RS's at R1 (S14)". Done, exact.
- E33 W1-PLAN SQUASH-CITES (a) (:505-512): plan.md §16, the PR #40 bullet (base :1436-1438; now :1437-1443) replaced
  by the ruling's text, wrapped at 120. Done, exact.
- Wrap fixes (no words changed): u4-build-plan.md §10's preamble tail ("Changed in rev 2 ... new: Q23 to Q26", 124
  characters at the base, 125 after E19) and §11's tail ("Q10 is off the list. A third unsafe exception", 128 after
  E20) rewrapped at 120. Every added non-table line in the four diffs is now at most 120 characters; table rows stay
  one line each, as the files' tables do.

## Checks
- A script (the session scratchpad's w1check.py, whitespace-normalised) finds every ordered text exactly once in its
  file (37 texts; the two W1-N2 and W1-U4D readings twice each in u4-build-plan.md, as §10 and §11 both carry them),
  and none of the 9 replaced or deleted texts.
- Mustafa's NETEM-BAR words checked against history.md :292-295 joined: equal.
- The word diff against 20702fd shows no change outside E1-E33 and the two wraps.

## Result: new sha256 (lines)
- plan.md 4717029cf385738df2171c1b6c885afe1372542be7407e207a6f41f2dbb4eb79 (1,518 lines)
- u4-build-plan.md f54125ec6a5af2d8de6446c9b830bc831aa1d2c6a8258314c3753830538817c6 (1,069 lines)
- u4s-carries.md 19e7ca98ca17286dfdb8a92ad235f4f3ce505f6ae87c0f3ae7460cf68de59232 (272 lines)
- advisor-u4.md 25d1846b897803222d97f5b35ab48ae0194896c9f801954dbf6e0b15811c9f0b (478 lines)

## Skipped (ordered elsewhere, later, or no text for these four files)
- W1-S1: the catalogue's own-state section (at U4d's merge: the main session's); U4e's brief rev 3; the npt-sim and
  npt-proto code (U4d's fix pass; the main session's merge commit).
- W1-U4D: every code fix (H1, N-M1, N-M3, N-M4, N-M5, R-M1, A-M1, T-M1, T-M2's history.rs and carry 14's test, T-M3,
  the refusal records, the Lows); TUNE-1's conditions (run headers, U4d's report); TUNE-2's World doc and assert (code);
  C13's report line (U4d's report). Carry 14's own text still says "1 MB" (no rewording ordered; T-M2 fixes it as
  10^6 B in §4.9 only).
- W1-N2: marks.json (U4j's builder made it), the catalogue :135, u4j-brief.md's two lines.
- W1-PUB: the three nits (limits.rs rename, server lib.rs's two paths, `Recording`'s two paths: code); `SimRng` in
  U4d's report; U4j's §4.1 items (in review) and U4d's Weapon, Scene::weapon, Scene::health (to the advisor).
- W1-U4E: Q5's U4r brief (not yet written); Q6's two conditions (no text placed in these files: they go to the U4e and
  U4r briefs); Q7 and Q9 (no ruling needed); "U4e then reports carry 24 as met" (U4e's report).
- W1-PLAN: the TCP-FREEZE relay to Mustafa (the main session's); R1's and R3's run plans (later).
- W1-TARGET: wave 2's briefs (no plan text).
- Not ordered, noticed: §5 U4d's Builds still says "the synthetic recording generator (test support)" (u4-build-plan.md
  :607), beside §4.15's new "npt-sim's library"; u4s-carries.md's header still says "44 entries" (45 at the base, 50
  now). Both left for the advisor or the main session.
