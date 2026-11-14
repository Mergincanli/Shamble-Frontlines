# The advisor's ruling on the U4r brief fold's questions (FQ1 to FQ7), 2026-11-13

Seat: a day session's advisor (general-purpose agent, opus, effort xhigh), on main's `.claude/agents/advisor.md` (read
with `git show origin/main:.claude/agents/advisor.md`). Read only: no build, no edit of a source, the brief, a plan or a
carries file, no commit. Branch `claude/netcode-proof-test-plan-hcky5g`, head a98c790 at the end (checkpoint commits
only since 4e91830 in spikes/). Notes: advisor-u4r-fold-notes.md.

Inputs, each sha256 checked: u4r-fold-questions.md (645488be…, read whole); my check advisor-u4r-brief.md (35f281bf…);
u4r-brief.md rev 3.1 (bcaef383…); u4r_fold.py (a9196126…) and its dry run (exit 0, 51 changes, 1,625 → 1,662 lines,
"dry run: nothing written"); u4r-fold-check.md (FAIL: F1 High, F2 to F4 Medium, F5 and F6 Low) and u4r-fold-notes.md;
u4s-carries.md (e8675a02…) and u4-build-plan.md (1e631c55…) as L2 left them (8bcbf36); L3's commit 4e91830, its two logs
and U4d2's M5 log; advisor-te-m3.md (5ecc3e23…) and advisor-te-m3-check.md (1ffd0294…).

## Verdict
**THE FOLD GOES AHEAD WITH THESE WORDS.** Each of FQ1 to FQ6 is ruled below with its exact words; FQ7 is noted.
- FQ1 ruled (my words for both sites); FQ2 ruled ((a) my words, (b) as proposed); FQ3 ruled ((a), (b), (c), (e) as
  proposed; (d) and (f) changed by me); FQ4 kept as proposed, plus the new rows' third cells naming the carry entries;
  FQ5 (a) to (f) confirmed; FQ6 (a) accepted, (b) accepted as fact, and the brief's words that L3 made false are ruled.
- My own findings at this check: **two Medium** (N1: 2.6's "only exceptions" list misses the items the rulings open;
  L3-1: 2.2 states the manifests as they were before L3) and five Low (below). None High; no security point.
- Nothing needs Mustafa: every word here is the text of my own conditions and rulings, in a notes-folder brief. No rule
  or pass bar is loosened, and no word or decision of Mustafa's changes.

## How the script reads this ruling
- Every replacement is one bullet on one line: a dash, its id, its site in brackets and a colon, then the old phrase in
  double quotes, the arrow, and the new phrase in double quotes; `\"` is a quotation mark inside a phrase. Only the
  bullets with an id (F1-1 to F6-9, N1 to N7b, F-D13) are replacements. Old phrases are rev 3.1's words exactly, with
  each line break and the next line's indent read as one space; I counted each in the flattened rev 3.1 (print-only
  `python3 -I` from stdin): every one found once, and every new phrase found 0 times in rev 3.1 and in the dry run's
  added lines.
- Seven of my check's bullets are **replaced** by bullets here (same old phrase, new words from this file): A2's first,
  second, third, fourth, seventh and eighth (my check :112-119, :124-130; the script's A2.1 to A2.4, A2.7, A2.8) by
  F6-4 to F6-9, and U4R-Q9's second phrase (my check :232-234; the script's Q9.2) by F3-6. A2.5, A2.6 and A2.9 stay as
  my check wrote them.
- One bullet changes words of my check, not of the brief (F-D13): the script applies it to Q14.4's new text (my check
  :345-356) before it folds that text in.
- The fold's assertions stand as L1 set them, plus FQ1's: after the fold, "by U4R-Q" (any case) is found only at C27
  (:20), section 1 item 5 (:250) and section 12 item 4 (:1605).
- **The second check.** If an old phrase here fails to match, or the fold must change a word that neither my check nor
  this ruling gives, it comes back to me. The record paragraph (FQ5 (e)) and the third cells are the main session's
  words; they need no second check while they state only where words went and what this ruling and my check say.

## FQ1: the two "by U4R-Qn" sites (the check's F1, High): ruled
My :178 sentence covers them; my check gave no words for them, which was my gap. 2.2's rule now lives in carry 74 as
amended (L2, 8bcbf36: "its bots give a command only at a client step whose newest snapshot tick moved since their last
command"), so 2.2 cites the carry, as 2.1's lead bullet does. Row 62 takes U4R-Q2 (c)'s owner and order.
- F1-1 (2.2, :476): "And, by U4R-Q14 (b)," → "And, by carry 74 as U4R-Q14 (b1) amended it,"
- F1-2 (3.2's row 62, :680): "with (h)'s check by U4R-Q2 (b)" → "with (h)'s check in marks.json the main session's, after r2's evidence (U4R-Q2 (c))"

The planned assertion is right. It would also have failed on one word of mine (F-D13 below).

## FQ2: section 10's two labels (the check's F4, Medium): ruled
(a) My words, close to the proposal: the label says what the questions now are, and "left" is the tense they were
asked in. (b) As proposed.
- F2-1 (:1284): "**Open** (each self-contained; only where the plan or a ruling leaves a choice):" → "**As rev 3.1 asked them, each ruled since in the table above** (each self-contained; only where the plan or a ruling left a choice):"
- F2-2 (:1276): "(the questions as rev 2 asked them stay in ed973c9, for the record)" → "(the questions as rev 2 asked them stay in ed973c9, and those rev 3.1 asked stay below the table, for the record)"

## FQ3: the text my rulings leave stale: ruled
(a), (b), (c) and (e) as proposed; (d) changed by me (U4R-Q6 (a) puts server items in the lane, and the rule binds them:
my ruling's ".claude/rules/netcode.md binds these edits"); (f) changed by me as proposed, as a replacement of Q9.2, so
the two parentheses never exist.
- F3-1 (section 6 item 5, :853): "D13's two runs (section 11)" → "D13's five runs (section 11)"
- F3-2 (7.1's order, :914): "In the build, D13's short P2 run at both leads comes before any `scenario --record`." → "In the build, D13's five short runs come before any `scenario --record`."
- F3-3 (7.4, :1105-1106): "(the determinism step's three in recordings.json under carry 82" → "(the determinism step's two at U4r (U4R-Q9 (b)) in recordings.json under carry 82"
- F3-4 (7.4, :1107): "determinism step green on the three (in check.mjs" → "determinism step green on the two (in check.mjs"
- F3-5 (7.3, :1085-1086): "(the ES and TC recordings likely less)" → "(the ES recording likely less)"
- F3-6 (2.1, :375-376; replaces Q9.2): "`scenario --record` at SS P2 seed 1, ES P3 seed 2, TC B1 seed 3 (U4R-Q9), and SS P0 seed 4 (the -30 pre-check's source only)" → "`scenario --record` at SS P2 seed 1 and ES P3 seed 2, and SS P0 seed 4 (the -30 pre-check's source only; TC B1 seed 3 is U4h's, with its TC shooter: U4R-Q9 (b))"
- F3-7 (2.1 (i), :384-385): "\"The determinism step keeps its three; the fourth serves the pre-check only\" (§4.15 :500-501)" → "\"The determinism step keeps the runner recordings (two at U4r, the third at U4h); SS P0 seed 4 serves the pre-check only\" (§4.15 :500-501)"
  (The script copies the quoted words from u4-build-plan.md :500-501, sha256 1e631c55… asserted first, and asserts they
  equal these; I read them there: the sentence starts at :500's "The determinism step" and ends at :501.)
- F3-8 (section 4, :806): "(U4R-Q6's site aside, if ruled)" → "(U4R-Q6 (a)'s items aside, which it binds)"
- F3-9 (section 10's U4R-Q2 (ii) and (iii) row, :1282): "U4R-Q2 below keeps (a) and (b)" → "U4R-Q2's (a) and (b): the U4R-Q2 row below"

**My additions: the same kind of stale text, outside FQ3's list.** A conditional on a question now ruled, or a list a
ruling widened.
- **N1, Medium: 2.6's exceptions.** 2.6 says the carries' named edits "are the only exceptions" to "no existing item
  changes in what the earlier units merged", lists carry 63 and carry 62 (h) by conditionals, and names neither U4R-Q6
  (a)'s npt-server items (U4e's crate: `npt_server::BITES` widened, the site in `pump`) nor SERIES.md's rows (U4j's
  marks/). The folded section 4 opens them, so a builder reading 2.6 meets a contradiction and could stop the wallclock
  bite (workaround: section 4's explicit lane; the stop rule sends it to the main session).
- N1 (2.6, :629-630): "carry 63 only if U4R-Q13 gives U4r crates/log; carry 62 (h)'s marks.json check only if U4R-Q2 (b) gives it." → "carry 63's two refusals and their tests in crates/log/src/lib.rs (U4R-Q13 (a)); and, opened by the rulings, the wallclock bite's items in npt-server (U4R-Q6 (a)) and marks/SERIES.md's new rows (U4R-Q2 (c)), as section 4's lane names them. Carry 62 (h)'s marks.json check is the main session's (U4R-Q2 (c))."
- **N2 to N7, Low:** each true once its ruling is read, but each reads as still open.
- N2 (section 1 item 5, :248-249): "a manifest change before U4R-Q1's ruling;" → "a manifest change (the main session's: U4R-Q1 (b));"
- N3 (2.4 r2 (d), :537): "in U4R-Q4's scope (-17's four profiles, -18's B1, -20's P3, -21's suites)" → "in U4R-Q4's scope (-17's four profiles, -18's B1, -20's P3 under option B, -21's option-B suites: U4R-Q4 (a))"
- N4a (2.4 r7, :585): "(only if U4R-Q10 rules (b'))" → "(U4R-Q10 (b'))"
- N4b (section 6 item 3, :845): "(r7 only if U4R-Q10 rules (b'))" → "(r7 at U4r: U4R-Q10 (b'))"
- N5 (3.2's row 63, :681): "crates/log is not in U4r's lane unless the ruling gives it" → "crates/log/src/lib.rs is in U4r's lane for carry 63's two refusals and their tests only (U4R-Q13 (a))"
- N6 (8.3, :1192): "U4R-Q12's reading (ii) adds to these (U4R-Q12 gives its totals)." → "U4R-Q12 (a) ruled reading (i), so these are its totals."
- N7a (8.4, :1209-1210): "(a line the main session adds to u4s-carries.md's §U4h, carries :548-560, or to U4h's brief: U4R-Q12)" → "(the main session's line in u4s-carries.md's §U4h, carries :548-560: entry 88, U4R-Q12 (a), added at 8bcbf36)"
  (Its ":548-560" is then re-pointed by section 12 item 9 like every other moved carries cite: FQ4.)
- N7b (section 12 item 7, :1613-1614): "rust-reviewer; plus security-auditor (carry 76's session boundary, carry 85's refusals, carry 63 if U4R-Q13 gives it to U4r)" → "rust-reviewer (the npt-server and npt-log edits included: U4R-Q6 (a), U4R-Q13 (a)); plus security-auditor (carry 76's session boundary, carry 85's refusals, carry 63's two refusals: U4R-Q13 (a); the wall-clock read in server code: U4R-Q6 (a))"
  (These are the panel's scopes my check confirmed, :377-380, and U4R-Q6's and Q13's readers.)
- Left as they stand: the cites of U4R-Q2's parts "(a)" and "(b)" as the question names them (:459, :612, :916,
  :1111): each points at the part asked, and its ruling is the table's row; the change lists C1 to C31 (a record);
  section 12 item 4 (a launch step, done by this fold).

**F-D13, Low: one word of mine.** My D13 words hold "about 10 % by U4R-Q14's arithmetic", the dry run's only added "by
U4R-Q" (its output line 449), which FQ1's assertion would refuse. The sense is "in", not the placeholder form.
- F-D13 (in Q14.4's new text, my check :350; not a brief phrase): "about 10 % by U4R-Q14's arithmetic" → "about 10 % in U4R-Q14's arithmetic"

## FQ4: the cites inside the new rows, and item 9's re-point: kept as proposed, with one addition
The Ruling cells quote my words as I wrote them, with their `(u4r-check :a-b)` cite: a record, so "§U4f (:491)" and
"§U4h (:548)" stay. Section 12 item 9 leaves section 10 whole (the questions as asked and the Answered table), as U4d2's
re-stamp left its section 8, and re-points every other cite of the brief into the plan or the carries whose line L2
moved, each asserted; I checked the moves you list against `git diff 8bcbf36~1 8bcbf36` (carries +3 after :343, +6
after :520, +17 after :559; plan +1 after :839; of the four rewrapped blocks, carries :340-346 grows by 3 lines and
plan :837-840 by 1, plan :496-501 and :755-757 keep their counts). So a reader can find the carry lines without the old
cites, each row's third cell names the entry:
- Q4's third cell: "2.4 r2; 2.2; its §U4f carry line: u4s-carries.md entry 86, the main session's L2 commit 8bcbf36"
- Q9's third cell: the main session's, with " (u4s-carries.md entry 87)" after "§U4h carry line".
- Q12's third cell: "8.3; 8.4; 3.2's row 40; its §U4h carry line: u4s-carries.md entry 88, the main session's L2 commit 8bcbf36"
- Q14's third cell: the main session's, with "carry 74's amended words: the main session's L2 commit 8bcbf36".

## FQ5: the main session's words in the fold: confirmed
- (a) Confirmed: the three stamps after `m9`'s in the list's comma form, `u4r-check` as "advisor-u4r-brief.md
  (35f281bf…)" (the file my "this file" named; its sha256 is unchanged since).
- (b) Confirmed: P16's line as a bullet of section 12 item 3 after the P13 to P15 bullet and before the P6 and P7 values
  bullet; my `\"` read as quotation marks.
- (c) Confirmed: P4's line's span from the old quoted string to "if U4R-Q6 rules (a))" becomes my one string in
  quotation marks, the rest of the line kept.
- (d) Confirmed, with the check's F2 and F3 applied: the Question cell as section 10 asks it; the Ruling cell only my
  words (the ruled letter as my bold label gives it, then the whole text for Q1, Q2, Q4, Q6, Q10 to Q13, the first
  sentence for Q7 to Q9, and for Q14 the three operative pieces you quote from :289, :301-302 and :322-323, so the row
  carries P0 2 above the question's kept "P0 1"), then `(u4r-check :a-b)`; the third cell the main session's. Write the
  third cells in the third person ("its words in place: ..."), never "your". Q1's third cell is FQ6's below; Q4's, Q9's,
  Q12's and Q14's take FQ4's additions.
- (e) Confirmed: the title's rev 3.1 becomes rev 3.2, and the record paragraph after the title block's paragraph. It
  also names this ruling (advisor-u4r-fold.md, with its sha256 at the fold), says A2 and Q9's second phrase are folded
  as this ruling replaces them, and lists what FQ1 to FQ3, N1 to N7, F-D13 and FQ6 add.
- (f) Confirmed: the two character changes are the main session's and are logged as such.

## FQ6: L3: (a) accepted; (b) accepted as fact; the brief's words it moves, ruled
- **(a) Accepted.** The workspace table's `npt-bots = { path = "crates/bots" }` (Cargo.toml:42) is what my own
  `npt-bots.workspace = true` form needs: the table names every internal crate a member depends on, and npt-bots had
  none before. It adds no edge and no package. My "and nothing else" meant no other dependency, edge or package, and
  that holds: `git show 4e91830` is 4 files, 5 added lines, Cargo.lock gaining exactly "npt-sim" under npt-bots and
  "npt-bots" under npt-lab. No other form.
- **(b) Accepted, and my premise was wrong (Low, mine).** crates/server/Cargo.toml:29 names npt-net, and U4d2's M5 log
  (.gate/u4d2-merge/m5-counted.log:125-138) compiles npt-net and npt-tcpinfo in `cargo build --locked --release -p
  npt-lab`, before any npt-bots edge. So my "since npt-lab's two release builds (WASM_BUILDS) now carry npt-net's tree"
  (check :185-186) and the question's premise (brief :1290-1292) read it wrong: L3 brought only npt-bots into npt-lab's
  builds. The route I named at :186 (npt-net optional in npt-bots) would have removed nothing, and it is moot: the
  counted run (.gate/l3-manifests/counted.log, HEAD 4e91830, 16:32:15 to 16:43:54 UTC, exit 0) passes all 11 steps with
  measured 491, bites 561, self 71 of 71, marks 81, redact 0 findings; the deps step's npt-sim feature sets are one set
  in the npt-server, npt-lab and npt-hash-wasm builds, the same text as M5's; the `wasm` step 35.8 s against M5's 33.1 s
  (npt-lab's two builds 5.9 and 6.1 s against 5.4 and 5.2; the other steps also ran slower in this run, test 380.7
  against 356.6 s). Nothing comes under :186. The question and my ruling's words stay as written (a record); Q1's third
  cell carries the correction:
- Q1's third cell: "section 4's lane; 2.2; made at 4e91830 (L3), with the workspace table's `npt-bots` line the `.workspace = true` form needs (no edge, no package); its counted run (.gate/l3-manifests/counted.log, exit 0): 11 of 11 steps with the base's counts, npt-sim's features one set, the `wasm` step 35.8 s; npt-net's tree was already in npt-lab's builds through npt-server (server/Cargo.toml:29), so the edges brought only npt-bots into them (advisor-u4r-fold.md, FQ6)"

**L3-1, Medium: 2.2 states the manifests as they were.** 2.2 (:490-493) tells the builder that npt-bots does not
depend on npt-sim and npt-lab not on npt-bots: false at any `<base>` after 4e91830, and section 12 item 9 re-walks
section 9's rows only. **L3-2 to L3-4, Low:** r1's cite of the workspace lint moved one line; D5 (filled at the old
head) misses L3; and my A2 words, written before L3, make "is now c49773f's" false and send section 12 item 1 to a
second counted run that L3's run already is (my L3: "section 12 item 1 takes its counts from that counted run"). The
count and ancestry tests move to 4e91830, which holds c49773f and cc8595d (`git merge-base --is-ancestor c49773f
4e91830` exits 0). Section 9's lead-in and section 12 item 9 keep c49773f (A2.5, A2.6, A2.9 as written), so item 9
re-walks R11 for L3's change by its own rule (R11 now changes: lab/Cargo.toml:32 names npt-bots, bots/Cargo.toml:22
npt-sim).
- F6-1 (2.2, :491-493): "At the head npt-bots depends on npt-link, npt-proto, npt-client, npt-net and npt-log, not on npt-sim (bots/Cargo.toml:20-25), npt-proto re-exports no npt-sim item (proto/src/lib.rs:31-44), and npt-lab does not depend on npt-bots (lab/Cargo.toml:23-34): U4R-Q1." → "Since L3 (4e91830, the main session's commit: U4R-Q1 (b)) npt-bots depends on npt-sim (`npt-sim.workspace = true`, no features: bots/Cargo.toml:22) beside npt-link, npt-proto, npt-client, npt-net and npt-log (:20-26), and npt-lab on npt-bots (lab/Cargo.toml:32); npt-proto re-exports no npt-sim item (proto/src/lib.rs:31-44), so the bots name those constants from npt-sim."
- F6-2 (2.4 r1, :528): "(Cargo.toml:106)" → "(Cargo.toml:107)"
- F6-3 (D5, :1519-1520): "with no npt-lab to npt-bots edge and no npt-bots to npt-sim edge. R11; U4R-Q1." → "with no npt-lab to npt-bots edge and no npt-bots to npt-sim edge. R11; U4R-Q1. Since then L3 (4e91830) adds both edges and the workspace table's `npt-bots` line; Cargo.lock gains those two edges only (`git show 4e91830 -- spikes/netcode-proof/Cargo.lock`: two added lines)."
- F6-4 (section 0, :168-169; replaces A2.1): "Its spikes/ is cc8595d's (`git diff --quiet cc8595d HEAD -- spikes/netcode-proof` exits 0), U4d2's merge, which holds" → "Its spikes/ was cc8595d's, U4d2's merge, then c49773f's (TE-M3's fix, delay_tests.rs alone: `git diff --stat cc8595d c49773f -- spikes/` names that one file), and is now 4e91830's (L3, U4R-Q1 (b)'s manifests: `git diff --stat c49773f 4e91830 -- spikes/` names four files, Cargo.lock, Cargo.toml and the two crates' Cargo.toml; `git diff --quiet 4e91830 HEAD -- spikes/netcode-proof` exits 0); cc8595d holds"
- F6-5 (section 1 item 1, :225-226; replaces A2.2): "`git merge-base --is-ancestor cc8595d HEAD` exits 0. Record both" → "`git merge-base --is-ancestor 4e91830 HEAD` exits 0 (L3's manifests, after TE-M3's fix c49773f on U4d2's merge cc8595d). Record both"
- F6-6 (section 4 rule 5, :742; replaces A2.3): "at cc8595d, U4d2's merge, the same: D1)" → "at cc8595d, U4d2's merge, at c49773f, TE-M3's fix, and at 4e91830, L3's manifests, the same: D1)"
- F6-7 (section 6 item 1, :839; replaces A2.4): "the `git merge-base --is-ancestor cc8595d HEAD` result" → "the `git merge-base --is-ancestor 4e91830 HEAD` result"
- F6-8 (D1, :1503-1504; replaces A2.7): "They are `<base>`'s counts if `cd /home/user/Shamble-Frontlines && git diff --quiet cc8595d <base> -- spikes/netcode-proof` exits 0 (it exits 0 at fa8e335)" → "The same at c49773f, TE-M3's fix (its P5: 11 of 11 steps, 491 / 561, self 71 of 71, marks 81, redact 0: advisor-te-m3-check.md), and at 4e91830, L3's manifests (its counted run in the main folder through main-run.sh, .gate/l3-manifests/counted.log, exit 0 at 16:43:54 UTC: the same counts). They are `<base>`'s counts if `cd /home/user/Shamble-Frontlines && git diff --quiet 4e91830 <base> -- spikes/netcode-proof` exits 0 (it exits 0 at a98c790)"
- F6-9 (section 12 item 1, :1566-1567; replaces A2.8): "If `cd /home/user/Shamble-Frontlines && git diff --quiet cc8595d <base> -- spikes/netcode-proof` exits 0, the counts are D1's" → "If `cd /home/user/Shamble-Frontlines && git diff --quiet 4e91830 <base> -- spikes/netcode-proof` exits 0, the counts are D1's"

No brief figure changes for the `wasm` step: D3's 33.1 s warm stays the measured reference beside L3's 35.8 s (the
estimates are arithmetic; r5 and section 5 item 4 measure).

## FQ7: the fold writer's two slips: noted
A9's form. (1) Its first dry run's output went into the session scratchpad by a shell redirect: outside the
repository, the script's printed output only, read only. (2) `python3 -I -m py_compile` wrote
.checkpoints/netcode-proof-test/__pycache__/u4r_fold.cpython-313.pyc, which the autosave bb412c8 committed; 23a34f5
removes it by `git rm` (I read both commits; `git ls-files` of the folder now names only u4c-test-plant's older .pyc,
not this fold's). It stays in bb412c8's history: no rewrite (CLAUDE.md, Never). Running the script writes no .pyc (I
checked after my own dry runs). The brief, my check and every other file are unchanged by them. Nothing redone. For
later briefs' limits: a syntax check that writes nothing is `python3 -I -c "import ast,sys;
ast.parse(open(sys.argv[1]).read())" <file>`.

## The fold check's findings, as they close
F1 High: FQ1's words. F2 and F3 Medium: FQ5 (d) as confirmed. F4 Medium: FQ2's words. F5 Low: FQ5 (f). F6 Low: FQ7.
The check's "Questions for the advisor" list: FQ1 to FQ4, each ruled above.

## What the main session does next
1. Edit u4r_fold.py once: read every bullet's words from this file by line range (sha256 of this file asserted first);
   replace A2.1 to A2.4, A2.7, A2.8 and Q9.2 by F6-4 to F6-9 and F3-6; apply F-D13 to Q14.4's new text; add F1-1, F1-2,
   F2-1, F2-2, F3-1 to F3-5, F3-7 (with its copy from the plan), F3-8, F3-9, N1 to N7b, F6-1 to F6-3; the third cells of
   FQ4 and FQ6; the record paragraph (FQ5 (e)).
2. Dry run: each old phrase found once, each new phrase once after, "by U4R-Q" only at :20, :250 and :1605, no changed
   line over 120 (rows exempt); read it in full; apply.
3. Then L4 and section 12 item 9 as my check orders them: item 9 re-sets section 0's plan and carries stamps, re-points
   the moved cites (section 10 left whole), and re-walks section 9's R11 for L3's change; item 1 takes D1's counts from
   L3's counted run when `git diff --quiet 4e91830 <base> -- spikes/netcode-proof` exits 0.

## Risk tier
Risk tier 1: the U4r unit is spike code on PR #40's branch, and this ruling and the fold change notes-folder text only
(.checkpoints/): no kit, gate, CI or decision-doc file, no word of Mustafa's. Nothing a player sees, hears or feels.

## What I checked, and what I did not
- **Checked, by reading and print-only `python3 -I`:** the questions file whole; my check :1-449; the brief at every
  line each FQ names and every "U4R-Q" cite outside section 10; the dry run's whole output (log, the main session's
  words, the diff); the fold check and the writer's notes; L2's diff (8bcbf36) and the plan's :492-505 now; L3's commit
  (4e91830), its lock and counted logs, M5's log :120-140; the manifests at the head; the TE-M3 stamps; the .pyc commits.
  Every old phrase here counted once in the flattened rev 3.1, and every new phrase 0 times there and in the dry run's
  added lines.
- **Not checked:** the folded brief itself (it does not exist until the main session applies the fold: step 2's
  assertions are that check); the record paragraph's final words (the main session's, under the rule above); any run.

## Decisions needed (Mustafa)
None. Each item is the text of my own conditions and rulings in the U4r brief, inside the plan; no rule or pass bar
moves, and nothing here is his to decide.

## For Mustafa
Nothing.
