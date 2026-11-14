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
Method: my script (scratchpad te_check.py) pulls each ordered text from advisor-u4-wave1.md by line range and anchors
(never retyped), normalises whitespace, and counts it in the target file; plus junction checks (old text | new text) and
a read of all four `git diff 20702fd` outputs hunk by hunk. sha256 of all four files equal the editor's; base sha256 equal
the task's. (No Edit tool in this run: this notes file was rewritten whole by Write, its earlier lines unchanged.)
- L1 = E1: present once, exact, after "(the advisor's U4s ruling, condition 1 (d))." (u4-build-plan.md:851-856). OK.
- L2 = E2: :184 row; own-state kind, plain-data types exact; accessor list spelled from :399-401 (words theirs, marked). OK.
- L3 = E3: new npt-proto row :192, both cells exact. OK. (F5: its "§4.7" reads as the build plan's own §4.7.)
- L4 = E4, L5 = E5: exact (my first L5 anchor was wrong: the row reads `InputBuffer::CMD_CATCHUP`; rechecked: 1). OK.
- L6 = E6: §4.2 :241-248, exact; old sentence gone. OK. (F10: the ruling's quote of condition 1 says "never"; :86 "not".)
- L7 = E7: §4.4 :279-283, exact; old "16-bit offsets" gone; no other "offset" left that means the header. OK.
- L8 = E8: §4.9 :345-346, exact. OK.
- L9 = E9: §4.10 :368-370, exact, after "(the advisor's condition 1)." OK.
- L10 = E10: §4.13 :431-435, exact, after the extent's cite. OK.
- L11 = E11: §4.14 :469, exact table row. OK.
- L12 = E12: §4.15 :493-495, exact; old "test support" gone from §4.15 but still at §5 U4d :607 (F2). Parenthetical
  now trails "the manifest" (F9).
- L13 = E13: D-8 :629-630, exact plus "all three" (theirs, marked; harmless: within-reach also holds below the bound).
- L14 = E14, L15+L16 = E15, L17 = E17: exact, in place. OK.
- L18 = E16: U4r's -58 bullet deleted; no other U4r text names -58 (run count :943 is a total). OK.
- L19 = E18: U4g Marks moved :813-816, exact, last item. OK.
- L20-L23 = E19/E20: both readings exact, twice each (§10 :955-961, §11 :1059-1069). "eight" count correct. OK.
- L24 = E20: TUNE-2 clause in §11 only; right, since Q16's reading words live only in §11 (§10 lists codes). OK.
- L25 = E21: exact; matches condition 1 at advisor-u4.md:86 ("not"). Placed :195-196 + blank: shifts :195-475 by 3 (F1).
- L26-L30 = E22-E26: carries 46-50 exact; labels theirs; cites into advisor-u4-wave1.md checked. OK.
- L31 = E27: carry 24 exact from "`npt_client::BITES` grows with". OK.
- L32 = E28: carry 40, words theirs (ruling gives intent only). Conflicts with its kept "holds by construction" (F3).
- L33 = E29: plan.md §4.7 :585-586, exact. OK.
- L34 = E30: U8 row, exact, after "since this container has no tc". Its cite advisor-u3q.md:143-144 is :142-143 (F4).
- L35 = E31: Mustafa's two sentences verbatim (history.md:292-295), code, option and date in the lead-in. OK.
- L36 = E32, L37 = E33: exact; SQUASH-40's reason matches his words (history.md:319). OK.
- Old texts: all 10 gone (4.2, 4.4, 4.9, 4.15, clippy, U4f, 4.14 row, carry 24, "never squashed", "six readings").
- Unordered changes: none (every hunk maps to L1-L37; two rewraps change no word). Width: no added non-table line > 120.
- Out of scope untouched since base: docs/, marks/, history.md, the checkpoint, u4j-brief.md, advisor-u4-wave1.md. The
  only other change is uncommitted spikes/netcode-proof/crates/l1 (U4c's lane, not the editor's).
- Editor's list vs mine: same 37 texts; E-numbers group some (E2, E15, E19, E20). Nothing missing.

## Findings (severity; fix)
- F1 Medium: E21's 3 lines at advisor-u4.md:195-197 shift every later line by 3; now wrong: u4s-carries.md:12-13
  (":236-375", "(:313-327)", "(:343-347)"), :138 (":351-364"), :184 (":343-347"), :251 (":268-270"); out of scope,
  u4j-brief.md:3 (":365-371", U4j in review). Fix: move the paragraph to the file's end (NF-D4's precedent, :398, is a
  later main-session line naming the verdict "above"), label "**Main session, 2026-11-11 (the U4 verdict above,
  condition 1 at :86):**", the ruling's line unchanged. Else +3 on each cite.
- F2 Medium: §5 U4d Builds :607 "the synthetic recording generator (test support)" vs §4.15 :493 and W1-PUB :408-409.
  Fix: "the synthetic recording generator (`Recording::synthetic`, in npt-sim's library: §4.15; the advisor's W1-PUB
  ruling)".
- F3 Low: carry 40 :184-191: kept "holds by construction at U4r" vs the new VOID sentence; label credits the words to
  the ruling. Fix words in the result.
- F4 Low: plan.md:1248 "advisor-u3q.md:143-144" -> ":142-143" (the ruling's own slip, W1-PLAN :525).
- F5 Low: u4-build-plan.md:192 Why cell "§4.7's items" -> "plan §4.7's items" (bare §4.7 there is "Events", :329).
- F6 Low: §4.10 :362-365 and §4.2 :226-230 still claim clamp idempotence unconditionally (TUNE-2 bounds it).
- F7 Low: u4s-carries.md:14 "44 entries" (50 now); carry 13 not marked as split to entry 48.
- F8 Low: U4E-Q6's two conditions (:476-479) have no home; suggest a U4r carry 51 in the ruling's words.
- F9 Low: §4.15 :493-495 parenthetical trails "the manifest"; move it after "U4d's three synthetic recordings".
- F10 Low: u4-build-plan.md:247 quotes condition 1 as "never what the client claimed"; :86 says "not" (ruling's words).
- F11 Low (to the advisor): §4.1's npt-sim row lists none of W1-PUB's approved bites-only World items, carry 17's
  constants or `Recording::synthetic`, while other crates' rows list their bites-only items; the ruling's list omitted
  them, so the editor was right not to add them.
Verdict: PASS WITH FINDINGS. Done 2026-11-11.

## Re-check after the editor's fix pass (2026-11-11; F1, F2, F10 and what the pass changed)
Method: the four files' sha256 equal the fix pass's table (plan.md 4717029c…, u4-build-plan.md a81b230f…, u4s-carries.md
19e7ca98…, advisor-u4.md 7b3e76ca…). HEAD (cc6019b) holds the first-pass state (u4-build-plan.md f54125ec…,
advisor-u4.md 25d1846b…), so `git diff HEAD` isolates the pass: one hunk in each of the two files, nothing else;
plan.md and u4s-carries.md equal HEAD.
- F1: closed. advisor-u4.md :1-475 equal the base byte for byte (cmp against `git show 20702fd:`). The paragraph sits
  at :477-479 after a blank line, after the "Recorded by the main session (about 18:15 UTC)" paragraph (same bold-label
  pattern). The ruling's quoted line (advisor-u4-wave1.md:229-231) is found exactly once, whitespace-normalised. The
  label's ":86" is condition 1's amended sentence. Every advisor-u4.md line cite in the notes folder and the checkpoint
  points at :475 or earlier (u4s-carries.md:12, :13, :138, :142, :184, :251; u4j-brief.md:3's :365-371, U4S-BUILDER;
  u4s-cond-* and u4c-* notes), so all of them hold. The three new lines are each 120 characters or fewer (the base's
  longer lines, :276, :340, :422, :437-438 and :461, are untouched).
- F2: closed. u4-build-plan.md:607-608 reads exactly my fix's words. The editor's notes mark them "words mine". §4.15 is
  right (heading :477, "npt-sim's library (`Recording::synthetic`)" at :493; W1-PUB :408-409). The wrap is 115 and 39
  characters, with the list's two-space continuation indent. The one added line does no harm: u4-build-plan.md has no
  self-cite past :607 (only :507's "checkpoint :620-621", another file). plan.md, u4s-carries.md, the checkpoint and
  history.md have no cite into u4-build-plan.md past :607. advisor-u4.md's cites into it (:246, :295, :344, :428) record
  older numberings, as returned. No file records the first-pass hashes. My own line numbers above for u4-build-plan.md
  past :607 (L1's :851-856; L13 on: :629-630, :813-816, :955-961, :1059-1069) now read one line early. That affects
  these notes only.
- F10: I accept the rejection. The ruling orders that sentence word for word (advisor-u4-wave1.md:223-225), and my fix
  needed the advisor's nod. u4-build-plan.md:247-248 is unchanged since the first pass. It stays open as a Low for the
  advisor: one word, "never" to "not". It is not a defect in the batch.
- Out of scope: since the base, only the main session's commits changed the checkpoint and spikes/netcode-proof/crates/l1
  (f3f7c2f, fc8a9ec, cc6019b). The working tree changes only advisor-u4.md, u4-build-plan.md and w1-plan-edits.md.
  docs/, marks/, history.md, u4j-brief.md and advisor-u4-wave1.md are untouched.
- New findings: none. Not in this pass and unchanged: F3-F9 and F11 (all Low).
Re-check verdict: PASS WITH FINDINGS (only Lows open: F10 for the advisor; F3-F9 and F11 from the first check).
