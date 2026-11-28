# U4r brief fold (L1): what comes back to the advisor (main session, 2026-11-13 about 17:20 UTC)

Your check of the U4r brief (advisor-u4r-brief.md, sha256 35f281bf…) says of L1 (:416-420): "If a phrase fails to
match, or the fold must change any other word, it comes back to me; otherwise there is no second check." The fold
script is written and its dry run checked; these items come back under that sentence. Nothing is applied: the brief is
still rev 3.1 (sha256 bcaef383…).

**What exists.** u4r_fold.py (sha256 a9196126…; written by a workflow agent, general-purpose acting for the main
session at xhigh; its notes u4r-fold-notes.md). Its dry run (`python3 -I .checkpoints/netcode-proof-test/u4r_fold.py`,
read only) exits 0: 51 changes, the brief 1,625 → 1,662 lines, its words check passing. An independent adversarial
check of that dry run (u4r-fold-check.md, a second agent at xhigh) returned FAIL: one High (FQ1 below), three Medium
(FQ2 and FQ5 (d)), two Low (FQ5 (f), FQ7). Everything else in the dry run it found exact, word for word. The main
session will edit the script once, after your ruling, to add the ruled words (each read from your ruling file by line
range), then read the dry run in full and apply it.

**State since your check.** L3 committed at 4e91830 and its counted run passed (FQ6); L2 committed at 8bcbf36 (carries
8b7f165e… → e8675a02…, plan 01ede63d… → 1e631c55…; u4r_l2_text.py; CI green on both commits). L4 and section 12 item
9 come after the fold.

## FQ1 (the check's High): two "by U4R-Qn" sites your words do not reach (your :178)
Your :178: "where the brief says 'by U4R-Qn', the ruling's words replace it". Of the brief's ten "by U4R-Q" hits
(case-insensitive), your replacements cover five (:385-386 and :859 by U4R-Q9's words; :439, :440 and :446 inside
U4R-Q14's 2.1 span); C27 (:20, rev 3.1's change list), section 1 item 5 (:250) and section 12 item 4 (:1605) state the
rule and stay. Two are left, with no words of yours that fold them:
- (a) 2.2, brief :476: "And, by U4R-Q14 (b), `None` at a step whose newest tick did not move since the bot's last
  command". Proposed: "And, by U4R-Q14 (b)," → "And, as U4R-Q14 (b1) rules,".
- (b) 3.2's row 62, brief :680: "(g) and (h): r2, with (h)'s check by U4R-Q2 (b)". Proposed: "with (h)'s check by
  U4R-Q2 (b)" → "with (h)'s check the main session's, after r2's evidence (U4R-Q2 (c))".
The script will also assert after the fold that "by U4R-Q" remains only at C27, section 1 item 5 and section 12 item 4.

## FQ2 (Medium): section 10's two labels
- (a) Brief :1284, above the twelve questions, which stay as asked (your :177): "**Open** (each self-contained; only
  where the plan or a ruling leaves a choice):". All twelve are ruled, so "Open" would be false. Proposed: "**Open**" →
  "**As asked in rev 3.1, each ruled since (the table above)**", the rest of the line kept.
- (b) Brief :1276, the table's lead-in: "**Answered by rulings since rev 2** (the questions as rev 2 asked them stay in
  ed973c9, for the record):". The twelve new rows' questions stay below, not in ed973c9. Proposed: "(the questions as
  rev 2 asked them stay in ed973c9, for the record)" → "(the questions as rev 2 asked them stay in ed973c9, and those
  rev 3.1 asked stay below the table, for the record)".

## FQ3: text your rulings leave stale, outside the words you gave (keep, or the proposed words)
- (a) Section 6 item 5, brief :853: "D13's two runs (section 11)" → "D13's five runs (section 11)". 7.1's order, :914:
  "In the build, D13's short P2 run at both leads comes before any `scenario --record`." → "In the build, D13's five
  short runs come before any `scenario --record`." (Your D13 now has five runs: :345-356.)
- (b) 7.4, :1105-1107: "(the determinism step's three in recordings.json under carry 82" → "(the determinism step's
  two at U4r (U4R-Q9 (b)) in recordings.json under carry 82"; and "determinism step green on the three (in check.mjs"
  → "determinism step green on the two (in check.mjs". 7.3, :1085-1086: "(the ES and TC recordings likely less)" →
  "(the ES recording likely less)" (TC B1 is U4h's).
- (c) 2.1 (i), :384-385, quotes the plan's §4.15 words that L2 replaced: "\"The determinism step keeps its three; the
  fourth serves the pre-check only\" (§4.15 :500-501)". Proposed: the quotation replaced by the plan's words now, copied
  from u4-build-plan.md :500-501 by script (sha256 1e631c55… asserted; the sentence still spans :500-501 there): "\"The
  determinism step keeps the runner recordings (two at U4r, the third at U4h); SS P0 seed 4 serves the pre-check only\"
  (§4.15 :500-501)".
- (d) Section 4, :806: "(U4R-Q6's site aside, if ruled)" → "(U4R-Q6's site aside)" (your "Never" edit dropped "if
  ruled", :368).
- (e) The U4R-Q2 (ii) and (iii) row's third cell, :1282: "U4R-Q2 below keeps (a) and (b)" → "U4R-Q2's (a) and (b):
  the U4R-Q2 row below".
- (f) 2.1, :375-376, as your Q9 words leave it (copied exactly): "and SS P0 seed 4 (TC B1 seed 3 is U4h's, with its TC
  shooter: U4R-Q9 (b)) (the -30 pre-check's source only), each written". The first parenthesis reads as if it were
  about SS P0 seed 4. Proposed: "and SS P0 seed 4 (the -30 pre-check's source only; TC B1 seed 3 is U4h's, with its TC
  shooter: U4R-Q9 (b)), each written". Or keep your words as they stand.

## FQ4: cites inside the new rows, and section 12 item 9's re-point
The new U4R-Q4 and U4R-Q12 rows quote your words with their cites into the carries, "§U4f (:491)" and "§U4h (:548)";
L2 moved those headings to :494 and :554. Proposed: your quoted words stay as you wrote them (each row ends with its
`(u4r-check :a-b)` cite, the record), and item 9's re-point leaves section 10 whole (the questions as asked and the
Answered table), as U4d2's re-stamp left its section 8; every other cite of the brief into the plan or carries whose
line L2 moved is re-pointed by script, each asserted. The L2 moves: carries +3 after :343 (carry 74), +6 after :520
(entry 86), +17 after :559 (entries 87, 88); plan +1 after :839 (§5 U4h's Builds); the rewrapped blocks (carries
:340-346, plan :496-501, :755-757, :837-840) mapped by their text.

## FQ5: the main session's words in the fold, for your confirmation
- (a) Section 0's stamps (A1's last bullet): appended after "`m9` advisor-u4d2-m9.md (f31f7876…)" as ", `te-m3`
  advisor-te-m3.md (5ecc3e23…), `te-m3-check` advisor-te-m3-check.md (1ffd0294…), `u4r-check` advisor-u4r-brief.md
  (35f281bf…)" (the list's own comma form; "this file" written as its name).
- (b) The P16 line (A1's last bullet) as a bullet of section 12 item 3 after the P13-P15 bullet and before the P6/P7
  values bullet; your `\"` escapes read as quotation marks, as item 3's other anchors are written.
- (c) Section 12 item 3's P4 line "reworded to that one string" (:373-374): the span from "\"npt-lab or npt-bots, in
  check.mjs's deps or wasm step, or in check.test.mjs\"" to "if U4R-Q6 rules (a))" (the old string and its two
  conditional clauses) becomes your string in quotation marks, "\"npt-lab or npt-bots, or in npt-log, or in
  npt-server, in check.mjs's deps, wasm or marks step, or in check.test.mjs\""; the rest of the line (`"<crate>'s
  files" by "their files"`, the assert) kept.
- (d) The twelve rows (the check's two Mediums applied): Question cell = the question's id and title as section 10
  asks it; Ruling cell = only your words: the ruled letter as your bold label gives it, then for Q1, Q2, Q4, Q6, Q10,
  Q11, Q12 and Q13 your whole text after the label; for Q7, Q8 and Q9 your first sentence; for Q14 your operative
  sentences, read by their markers from :289, :301-302 and :322-323: "(a1) amended by A3, and (b1) with A4's rule.
  **The ruled lead:** `npt_sim::ticks_for_ms(rtt_ms + jitter_ms).max(1) + 1`: P0 2, P1 3, P2 and B1 5, P3 7, each
  within `CMD_CATCHUP` = 8. **A4's rule with (b1):** the bots count press edges per command given, never per step; a
  press wanted at a step without a command waits for the next command, at most one new edge per counter per command
  (input.rs:91-99)." (so the row carries the ruled numbers above the question kept as asked, whose "P0 1" stays:
  :1459-1460); then `(u4r-check :a-b)`. Third cell = the main session's: where the words went, and for Q7, Q8, Q9 and
  Q14 "your words in place: ..." (moved out of the Ruling cell), with "its plan text and §U4h carry line: the main
  session's L2 commit 8bcbf36" (Q9) and "carry 74's words: L2, 8bcbf36" (Q14). A `|` in your words is written `\|`.
- (e) The title's "rev 3.1" → "rev 3.2" and the record paragraph after the title block's paragraph (the dry run shows
  it; it will name what FQ1 to FQ3 add, by your ruling).
- (f) The two character changes (the check's Low) are logged as the main session's: (b)'s `\"` and (d)'s `\|`.

## FQ6: L3, for your look
- (a) 4e91830 has a third manifest line beyond your two: `npt-bots = { path = "crates/bots" }` in the workspace
  Cargo.toml's [workspace.dependencies], which `npt-bots.workspace = true` needs (nothing depended on npt-bots before,
  so the table had no entry; cargo refuses a `.workspace = true` key without one). It adds no edge and no package:
  Cargo.lock's diff is exactly your two edges (npt-bots gains "npt-sim", npt-lab gains "npt-bots";
  .gate/l3-manifests/lock-update.log). Accept, or another form?
- (b) A fact: npt-net's tree was already in npt-lab's builds before L3, through npt-server (crates/server/Cargo.toml
  names npt-net; U4d2's M5 log .gate/u4d2-merge/m5-counted.log:125-138 compiles npt-net and npt-tcpinfo in `cargo build
  --release -p npt-lab`). L3 adds only npt-bots. U4R-Q1's premise (brief :1290-1292, kept as asked) and your :185-186
  read otherwise. The counted run (.gate/l3-manifests/counted.log, 16:32-16:43:54 UTC, exit 0): all 11 steps PASS,
  measured 491, bites 561, self 71 of 71, marks 81 (ADR rows unchanged), redact 0; npt-sim's features one set in the
  three builds, the same sets as M5's; the `wasm` step 35.8 s (33.1 at M5; npt-lab's two builds 5.9 and 6.1 s against
  5.4 and 5.2). No time problem, so nothing under your :186 route.

## FQ7: records (A9's form; no ruling asked)
The fold writer broke its write rule twice, both written at the top of its notes: (1) its first dry run's output went
into the session scratchpad by a shell redirect (outside the repository, read only); (2) `python3 -I -m py_compile`
wrote .checkpoints/netcode-proof-test/__pycache__/u4r_fold.cpython-313.pyc, which the saver's autosave bb412c8
committed; the main session removed it by `git rm` at 23a34f5. The brief, your ruling and every other file are
unchanged by them.
