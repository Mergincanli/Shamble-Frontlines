# Advisor ruling D1-R2 (round 2 on route A: "*", the row's wording, the panic clause, the fix pass) - notes

Seat: advisor agent, day session, run as a general agent reading main's .claude/agents/advisor.md (origin/main 2aa5e9c;
the branch changes that file, K4, so main's copy governs).
Worktree: .gate/foundation-toolchain-wt, branch claude/laughing-pasteur-fwya24, HEAD 8523d50.
Content under review: 4bccf55be7bbf54de6e114ddbc6aeadb2cab3f3e (4bccf55..8523d50 touches only .checkpoints/: checked,
`git diff --stat 4bccf55 HEAD -- . ':!.checkpoints'` is empty).
Scratch only under /home/user/Shamble-Frontlines/.gate/advisor-d1-r2/. No tracked file changed.

## Plan
1. Read the evidence first: review-r2-findings.md (table + TE/EA full texts), review-r2-verdicts.json (4 skeptic
   verdicts, exact wording), skeptic-r2-R2-D1..D4.md, the four reviewers' notes (RR/SA/TE/EA r2), session-std-share.txt.
2. Read the head's texts the drafts touch: Cargo.toml comment, docs/adr/README.md:234 row (OVERFLOW-ALL), ADR 0002
   part 4 / PM-0002-53 / Appendix A R5-RUSTLINT-REC4 (0002:1586), the report (Outcome, Plan and approval, As built,
   E-rows, OVERFLOW-* follow-ups, the decision entry :293-310), STATUS (2.2 bullet, :65-66, the 120-line cap), REPO_MAP,
   ROADMAP, STATUS_ARCHIVE overflow lines.
3. Then the candidate: fix-r2-drafts.md (A, B, C, D1-D10, E, F, G).
4. Own checks where a ruling rests on a mechanism claim: (a) re-run R2-D1's share-generics repro in scratch (opt-level
   0/1/s/z vs 2/3; workspace crate's own sum() via a dependency's unchecked copy), with and without "*"; (b) whether
   dropping "*" closes it; (c) the std-share claim; (d) column widths of draft A's lines; (e) STATUS line count.
5. Answer Q1-Q7 with exact approved texts; verdict, conditions, escalations. Severity on the one scale. Last fix pass:
   anything below High that arrives now becomes a report line or follow-up.

## Findings (one line each, as done)
- F1 read: main's advisor.md (2aa5e9c); the earlier ruling D1; review-r2-findings.md (12 items; TE/EA full texts);
  review-r2-verdicts.json (4 skeptics: R2-D1..D4 hold, Medium each; R2-D4's reviewers labelled it Low); the four skeptic
  notes; the reviewers' r2 notes (RR C1-C20; SA focus 1, SA2-1..3, durable decisions; EA C1-C14; TE blocks);
  session-std-share.txt (std supplies no unchecked shared Sum/Product at opt 0-3, s, z); fix-r2-drafts.md (A-G).
- F2 head texts read at 4bccf55: Cargo.toml:21-29 (comment :21-24, 117/115/118/54 cols); README:234 (row); ADR 0002
  :1-8 (author engine-architect; "Pass marks" table: test-engineer), part 4 :247-278, :465-471, PM-0002-53 :1319,
  Appendix A :1562 (R1-RR-OQ4), :1586 (REC4 adopted), :1617 (OQ2), :1619 (OQ4); ADR 0004 :3-5 (author rust-reviewer;
  pass marks test-engineer), part 20 :562-590, PM-0004-60 :1181; ADR 0001 :5 (author netcode-architect; pass marks
  test-engineer); the report :1-330 (Outcome :19-20, Plan and approval :39-52, As built :54-84, E1 :106, M5 :115, M7
  :117, E12 :118, Reviews :137-166, Risks :184-215, OVERFLOW-XCRATE-CI :231, follow-ups :253-273, the entry :293-310);
  STATUS :55-120 (120 lines; 2.2 bullet :95-99); STATUS_ARCHIVE :551-632, :676-678, :779-808; REPO_MAP :16, :18.
- F3 own run (scratch .gate/advisor-d1-r2/ws = git archive 4bccf55 + probe: room depends on sim and on extsum, a path
  crate outside the root with its own [workspace]; room's own #[inline(never)] sum of [200,100] (u8), sim's #[inline]
  sum called from room, extsum's own sum; values by catch_unwind; control [1,2] = 3), cargo/rustc 1.99.1, fresh target
  dir per run, `cargo test --release --offline -p room --test probe`:
  route A as shipped, CARGO_PROFILE_RELEASE_OPT_LEVEL 3 and 2: room's sum PANIC, sim's sum in room PANIC, extsum Ok(44);
  opt 1, "s", "z", 0: room's sum Ok(44), sim's sum in room Ok(44) (wrapped), extsum Ok(44). RR2-1 and K4 reproduced.
  "*" table removed, opt "s" and "z": all three PANIC. Route A + [profile.release.package.extsum] overflow-checks = true,
  "s" and "z": all three PANIC. Route A, test profile (dev): all three PANIC. Control 1+2 = 3 in every run.
  So both closing options close it (tested), the shipped opt-level 3 is checked, and dev is unaffected.
- F4 STATUS arithmetic: 120 lines at 4bccf55; draft E's two parentheticals, wrapped greedily at 120 (greedy is
  line-optimal), make the 2.2 bullet 7 lines (+2), so with only the night-shift compaction STATUS would be 121. With
  "OVERFLOW-XCRATE-CI (sooner if a crate depends on sim before 2.3)" and "OVERFLOW-PANIC-RULE (with room's first code)"
  the bullet is 6 lines (+1), and the compaction of :65-66 (117 cols; not his words: no DECISIONS.md hit) keeps 120.
- F5 comment: four lines (120/119/114/117 cols) in .gate/advisor-d1-r2/comment-final.txt, applied to a fresh archive
  of 4bccf55 (fixcheck/): `cargo build --workspace --locked --offline` exit 0, Cargo.lock byte-identical (cmp), K1b's
  release tests 1+1 passed.
- F6 owners: ADR 0002 author engine-architect, its pass marks (PM-0002-53) test-engineer (:5); ADR 0004 author
  rust-reviewer, pass marks test-engineer; ADR 0001 author netcode-architect, pass marks (PM-0001-34) test-engineer.
  D1's condition 3 named engine-architect alone for PM-0002-53: corrected here.
- Working view (before the advisor tool): Q1 keep "*" (zero reach today; both closing options tested; the ADR never
  asked for dependency checks; dropping it widens to every dependency with cost and security reach unmeasured). Q2
  approve with changes (state column: rulings D1 and D1-R2; owners incl. test-engineer; "non-member" plain; opt-level
  reopen as a trigger; PM-0002-53 home without the planted failure). Q3 (a): the panic hand-off as part of D1-R2 (tested
  fact + owners + anchor + pointer, no rule content). Q4 rule/trigger/owners/home in the row; method to follow-ups.
  Q5 D10 accurate with fixes ("directly"; the reason for the wider trigger; the reopen; "drafts"; open security
  item); (b) holds. Q6 the four lines above. Q7 fixes: D4 (five Wasm crates), D6 ("the guard" is ambiguous here), D7
  (CI-edit parenthetical), D8/F2 (in-repo path dep wording; #[inline(never)] precision; security read), D9/F3 (SA2-2
  open), E (6-line wording). R2-D4 relabelled Low -> Medium in writing.
- (advisor tool consulted once after orientation: D10's count of rules becomes three with (a); the report must carry
  the round-2 runs the row cites; run the protocol-home claim or attribute it; STATUS exact six lines + wc -l = 120 and
  OVERFLOW-PER-CRATE's tripwire decided; ROADMAP/STATUS #48 sync; no plan revision 11; 7d/7f and R2-L3 at my head
  review; measure every wrapped block. All folded below.)
- F7 own run, the test's home (scratch .gate/advisor-d1-r2/home = git archive 4bccf55 + sim `#[inline] pub fn
  tick_next(u16) -> u16 { t + 1 }` in sim's normal build; protocol and room each get `sim` as a path dependency and
  tests/home.rs: should_panic(expected = "attempt to add with overflow") on tick_next(u16::MAX) plus control 5 -> 6),
  `cargo test --release --offline -p <crate> --test home`: route A (4bccf55's Cargo.toml): protocol 2 passed, room 2
  passed. Part 4's two blocks (4d78ad6's Cargo.toml, byte for byte): protocol 2 passed; room 1 passed 1 failed "test
  did not panic as expected at crates/room/tests/home.rs:3:4". Test profile under part 4's blocks: both 2 passed. So a
  test in protocol cannot tell the settings apart; one in room can, in release only. R2-D4's Medium part reproduced.
- F8 read ADR 0001 :493-503 (part 11: tick panic fatal to the room; unwind; bounded channels) and :766-771 (a
  connection's panic closes only it; a tick panic ends the match, PM-0001-59; a Wasm trap restarts the Worker and
  resyncs): D6's and D10's three outcomes hold.
- F9 security-auditor's round-2 notes give a count verdict (0/0/2/1, "None calls for a round 3"), never the word
  APPROVE: the report's round-2 paragraph must not say "all four APPROVE" unless its handback did.
- F10 STATUS: the 2.2 bullet with "OVERFLOW-XCRATE-CI (or the first crate to depend on sim)", "OVERFLOW-PER-CRATE (with
  the first third-party crate)", "OVERFLOW-PANIC-RULE (with room's first code)" wraps to 6 lines (max 120;
  .gate/advisor-d1-r2/status-bullet.txt); with the night-shift line at 117 cols, STATUS stays 120 lines. Stale after
  the pass: STATUS :5 (last updated), :29 (#48's row: "review round 2" as next), :42-43; ROADMAP :125-126 ("Next:
  review round 2").
- F11 blocks composed and measured at 120 (awk; no code span split across lines): row.txt (one table line, 3 cells),
  D6-risks (6 lines, max 119), D8-per-crate (11, 120), D9-panic-rule (7, 120), D10-entry (22, 120),
  plan-approval-bullet (8, 119), as-built-addition (4, 118); all in .gate/advisor-d1-r2/blocks/.

## Ruling D1-R2 (2026-11-13; content under review 4bccf55; written after the advisor tool's consultation)

**Verdict: APPROVE the fix pass, with the changes below (conditions C1-C7).** Scope: round 2's open points (R2-D1 to
R2-D4) and the session's drafts A-G. It approves no head: none carries the fix pass yet; my review of the pushed head
(not a round) checks the fold. No setting changes (`Cargo.toml`'s values, `Cargo.lock` and every crate but sim-wasm's
doc comment stay as they are), so no plan revision follows. Nothing is Critical or High; nothing to escalate. This is
the budget's last fix pass.

Per question: Q1 keep `"*"`. Q2 approve the four rewordings, with my changes (the row below replaces draft C). Q3 (a):
the panic hand-off is part of this ruling, worded "to weigh", with no rule content. Q4 rule, triggers, owners and
PM-0002-53's home in the row; test method in the follow-ups. Q5 D10 is fair in substance but needs five fixes (my text
below); option (b) and the Recommendation hold as written. Q6 the four lines below. Q7 B, D1, D2, D5 (its three
corrections), F1, F4 and G as drafted; D3, D4, D6 to D10, E, F2 and F3 as below; plus the report's "Plan and
approval", "As built" and round 2's Reviews text.

Severity (the one scale; only I relabel, in writing): **R2-D4 goes from Low to Medium.** Reason: the record's home
for PM-0002-53's test, "a crate that depends on sim", admits `protocol`, where a counter test passes in release under
both settings (the R2-D4 skeptic's run; my F7), so a builder following the record could close OVERFLOW-XCRATE-CI with
a test that cannot see the gap. That is a real defect with a small reach; it is fixed in this pass. R2-L2 stays Medium
(EA2-1's own label; the merged item takes the highest). The other items keep their reviewers' labels.

Security: SA2-1 (R2-D2) and SA2-3 (R2-L5) are fixed in this pass. SA2-2 (R2-D3) stays open as the follow-up
OVERFLOW-PANIC-RULE: a security finding left open, so the PR is needs-mustafa on that count too (it is anyway: risk
tier 2, decision docs), and the report says so.

### Q1. `"*"`: keep it
- Today it changes nothing: no third-party crate exists (`Cargo.lock` holds the six crates only), so `"*"` matches
  nothing (D1's C2: no Cargo warning).
- RR2-1 needs both crates in one build at a sharing opt-level (the release profile, or one inheriting it, at 0, 1, "s"
  or "z"), with the dependency holding the same instance. The shipped profile is opt-level 3.
  - My F3, on 4bccf55's tree: at 3 and 2, room's own sum and sim's inline sum compiled in room panic.
  - At 1, "s", "z" and 0 they wrap (44).
  - Dropping `"*"` closes it, at "s" and at "z"; so does a named override for the dependency.
  - The test profile panics.
- So the choice can wait for its trigger at no cost, and both exits are proven.
- Dropping `"*"` now would turn on release overflow checks in every future dependency, the server's network stack
  included. ADR 0002 never asked for that (REC4: "the rest of the workspace keeps the release default"). Nothing has
  measured its cost or its security reach (SA2-2's point 6 asks for that read).
- That is the decision OVERFLOW-PER-CRATE takes at the first third-party crate. Its trigger is now widened to any
  workspace crate's tree, and it reopens if a release profile moves to a sharing opt-level.
- Tripwires:
  - the comment sits right above `[profile.release]`, where an opt-level would be added;
  - `[workspace.dependencies]`, where the first third-party crate enters, is in the same file;
  - STATUS names OVERFLOW-PER-CRATE's trigger (E below);
  - the row records both triggers.
- D1's condition 1 stands. Since `"*"` stays, drafts A, C, D8, D10 and F2 change only as written below.

### Q2. The row's rewording: approved with my changes (this text replaces draft C)
- The dependency sentence and the opt-level clause: approved, but in Cargo's own terms:
  - "every crate that is not a workspace member" (RR2-2: `"*"` follows membership);
  - "its own integer arithmetic" (TE2-1: its trait-generic code compiled in our crates takes our setting);
  - the opt-level case as its own sentence, with "the release profile, or one inheriting it".
- The wider trigger: approved (K4; my F3: sim's sum compiled in room wraps at "s" when only room uses the dependency).
  The opt-level move becomes a second trigger: "and again before such a profile moves to one of those opt-levels".
- PM-0002-53's home: approved as "a workspace crate that depends on sim, not from `protocol`" (my F7). The planted
  failure (`Cargo.toml` back to part 4's blocks) moves to OVERFLOW-XCRATE-CI: it is test method (Q4).
  - The row no longer lists builds: PM-0002-53's four builds stand.
  - This drops D1's "in release, native and wasm32", which TE2-3 showed can read as replacing them.
- The reword list: approved with REC4's Appendix A row (0002:1586). Two owner corrections:
  - PM-0002-53 is test-engineer's (ADR 0002 :5 credits the pass-marks table to test-engineer). D1's condition 3
    named engine-architect alone; corrected.
  - The state column names both rulings. Draft C's "its wording completed" understates D1-R2: it widens a trigger
    and adds the panic hand-off.
- Appendix A :1617 (OQ2's answer, "REC4 overflow") stays in OVERFLOW-PANIC-RULE's lines to weigh, not the reword list.
  Route A widens REC4's coverage, so that answer does not become untrue; whether to add `arithmetic_side_effects` is the
  owners' question.

The row (docs/adr/README.md:234), one table line:
```
| OVERFLOW-ALL: how far part 4's overflow setting reaches | the advisor agent's rulings D1 and D1-R2 (2026-11-13, on foundation 2.2's review rounds 1 and 2), never his decision; his go for this row: PR #48's merge; at ADR 0002's next revision engine-architect rewords part 4 and its Appendix A rows for R1-RR-OQ4 and R5-RUSTLINT-REC4, and test-engineer rewords PM-0002-53 | ADR 0002 part 4 and PM-0002-53: part 4's named setting (`[profile.release.package.sim] overflow-checks = true`, the same for `protocol`) reaches only code generated in sim's and protocol's own compilation, because sim's inline, generic and small functions are generated in the crates that call them (foundation 2.2's review, RR-1). The root `Cargo.toml` sets `[profile.release] overflow-checks = true` and `[profile.release.package."*"] overflow-checks = false`: every workspace crate checks overflow in every profile, and every crate that is not a workspace member keeps the release default setting, so its own integer arithmetic can still differ between dev and release builds. While `"*"` stays, the release profile, or one inheriting it, at opt-level 0, 1, "s" or "z" can also make a workspace crate run a dependency's unchecked copy of shared generic code (integer `sum()` and `product()`, foundation 2.2's review round 2). `"*"` is decided, with its cost measured, when the first third-party crate enters any workspace crate's dependency tree, and again before such a profile moves to one of those opt-levels: a named override for each such crate (it beats `"*"`), or dropping `"*"` (which checks every dependency). PM-0002-53's committed counter test runs from a workspace crate that depends on sim, not from `protocol`: part 4's two blocks check sim's and protocol's own packages, so a test inside either cannot see the gap. The cost measurement (R5-RUSTLINT-OQ4) covers the checks as compiled in the calling crates. The panic side: with this setting an overflow in `room`'s own arithmetic panics in release where part 4's two blocks let it wrap, and a future server crate's code would do the same; the lines that keep a value a client or content can set from reaching such a panic go to their owners to weigh at each draft's next revision (part 4's explicit operations, which name counters and tick numbers: engine-architect; ADR 0004 part 20, whose list of panic sources names no overflow: rust-reviewer; the fuzz marks PM-0004-60 and PM-0001-34: test-engineer), and the plan of the first change that gives `room` or a server crate code states the overflow rule its arithmetic follows (the follow-up OVERFLOW-PANIC-RULE). Shown by foundation 2.2's planted failure M7, its local cross-crate run (E12) and review round 2's local runs (`docs/reports/2026-11-12-foundation-2-2.md`) |
```

### Q3. The panic clause: (a), in the row, as part of this ruling
The register's loose-ends section holds "each loose end, with where it is resolved", and route A's panic side is one.
ADR 0002's owner reads this row when rewording part 4, and would miss a hand-off kept only in STATUS_ARCHIVE. Option
(b) would put text in the row that the row's own state column says is not the ruling it records. (a) avoids that: the
clause is now this ruling's.

Handing ADR lines to their owners "to weigh" is technical routing within my authority, and so is setting a future
plan's evidence (the plan of room's first code states its overflow rule). The clause carries:
- the tested fact: room's own release arithmetic panics where part 4's blocks let it wrap;
- the inference, worded "would": a future server crate's code would do the same;
- the lines and their owners: engine-architect, rust-reviewer, test-engineer;
- the anchor and the pointer.

It carries no rule content: the security review's proposals live in OVERFLOW-PANIC-RULE. Its verbs: "rewords" only
for lines route A makes untrue (part 4, its two Appendix A rows, PM-0002-53); "to weigh" for ADR 0004 part 20 and the
fuzz marks.

### Q4. Detail level
His go covers the row. The row therefore carries:
- the rule: what `Cargo.toml` sets, and its reach, with the two tested exceptions stated as facts;
- the triggers: the first third-party crate in any workspace crate's tree; a release-derived profile at a sharing
  opt-level; room's first code for the panic side;
- the owners;
- PM-0002-53's home: a rewording of an ADR pass mark, so rule level.

Test method goes to the follow-ups, where 2.3's plan can change it without a new go on a register row:
- the planted failure;
- the four builds' mechanics;
- the Wasm runner and the no_mangle constraint;
- "the server's future network crates included";
- TE2-1's split;
- the security proposals.

The row keeps one short reach fact for the dropping option, "(which checks every dependency)", because that is what
the option does.

### Q5. The decision entry (replaces draft D10; Options and Recommendation unchanged)
Fixes, against the evidence:
1. "calls them directly": TE2-7's wording. Its counter-case: sim's helpers reached through a sim function that is
   not inlined are checked even under the old blocks.
2. Three more points, not two: with Q3 (a), his go also covers the panic hand-off. Draft D10 would re-create EA2-2's
   gap.
3. The libraries point gives both reasons: their own arithmetic can differ between test and released builds (the
   reason the choice exists), and the size-saving build (the reason it widened). It also gives the reopen trigger.
4. "The drafts already say": the paragraph above calls them drafts, and part 20 forbids a panic from "network input
   or content data", so the text says "player input or game data".
5. It says plainly that the point stays an open security follow-up until the first plan of the server's match code.

Option (b) still holds as written. Its "this report say[s] they reach only those two crates' own code" covers
rewording the follow-ups that assume route A. Under (b), room's arithmetic would wrap again, and OVERFLOW-PANIC-RULE's
premise would flip to a silent wrap; the session handles that if he picks (b) (a new head, my review, his go).

The Recommendation (a) holds. With fix 5 he decides knowing the stop-versus-wrap trade.

Measured: 22 lines, at most 120 columns.
```
- **OVERFLOW-ALL: may overflow checks stay on in release builds for all our own Rust crates, recorded in the ADR
  register?** ADR 0002 turns on overflow checks for the simulation and the message protocol by naming their two crates;
  an overflow check stops the program when a number grows too big for its type. A review found that this misses the
  simulation's small, inline and generic functions whenever another crate calls them directly (tick counters, maths
  helpers), because that code is then compiled inside the caller: a test build stops on such an overflow, but the
  released game would carry on with a wrong number. The fix turns the checks on in release for every crate of ours (the
  register row OVERFLOW-ALL). Your go for the row also covers three more points: (1) outside libraries keep Rust's
  default (no checks in release builds), so their own arithmetic can still differ between test and released builds; the
  choice about them (check each one, or all) comes with the first outside library any of our crates uses, and again if a
  release build is ever set to optimise for size or less, because such a build can make our own code run a library's
  unchecked copy of shared code (adding up or multiplying lists of numbers); (2) ADR 0002's overflow test moves out of
  the simulation's crate into another of our crates that calls it (not the protocol crate), since a test inside either
  of those two cannot see the gap; (3) the rule that keeps player input from causing such a stop goes to the drafts'
  owners to weigh (below). This is the advisor agent's technical ruling; your "no" overrules it.
  - Why it matters: the register is a decision doc, so the row needs your go. And an overflow in the future server's or
    client's own code then stops with an error instead of carrying on with a wrong number (with the old setting, a
    release build turned 100 minus 150 into 4,294,967,246). By the design drafts (not yet accepted), that ends the match
    if it happens in the server's match loop, drops only one player if it happens in that player's connection, and in
    the browser stops that player's game, which restarts and rejoins online (offline: still an open question). The
    drafts already say no player input or game data may crash a match, but their detailed lists do not name this cause
    yet, so the first plan that writes the server's match code must say how it avoids it, and the drafts' owners weigh
    adding it; until then it stays an open security follow-up (OVERFLOW-PANIC-RULE).
```

### Q6. The Cargo.toml comment (lines 21-24)
"Give that crate" names the dependency, so the "it" is gone. "'*' keeps every non-member", in Cargo's term, drops the
ambiguous "which". F5: built on a fresh archive with `--locked`, `Cargo.lock` byte-identical, K1b's release tests
pass.

Columns: 120, 119, 114, 117.
```
# Overflow checks are on in all profiles for every workspace crate (ADR 0002 part 4, widened on foundation 2.2's review,
# RR-1: sim's inline, generic and small functions compile in their callers; register row OVERFLOW-ALL). "*" keeps every
# non-member at the release default setting, so release or an inheriting profile at opt-level 0, 1, "s" or "z" can
# make our sum() and product() run a dependency's unchecked copy: drop "*" or give that crate overflow-checks = true.
```

### Q7. The rest of the fix pass
- **As drafted:**
  - **B:** sim-wasm's doc comment, EA-3's own words.
  - **D1:** "revisions 1 to 10".
  - **D2:** :70-71.
  - **D5:** the D5 row "the cut asserts none of its privilege properties"; the D13 row "fixed: 1de98a9 (the crates);
    REPO_MAP's row in round 2's fix"; the capability-set sentence. Its "(pending)" line is replaced by the round-2
    text below.
  - **F1:** the R2-D4 skeptic's item (4), verbatim.
  - **F4:** the block's intro, plus "and its ruling D1-R2" beside "the advisor agent's ruling D1 of 2026-11-13" in its
    first sentence.
  - **G:** REPO_MAP:18 (ADR 0002 :190's cell).
- **D3 and D4:** the texts below. D4's correction: wasm32 builds five crates, not six.
- **D6:** the Risks bullet below. In this repository "the guard" names the kit's hook, so "part 20's fuzz marks are
  not built" replaces it. 6 lines, at most 119 columns.
- **D7:** the report's table row below. The planted counter is a code change, not a CI edit, so the CI edits are
  named as their own item. Evidence now includes F7.
- **D8:** OVERFLOW-PER-CRATE in the report, below. Three precision fixes:
  - a member's path dependency joins the workspace, and a `[patch]` path does not (RR2-2);
  - an `#[inline(never)]` generic is reused only when the dependency already holds the same instance (K9);
  - dropping `"*"` gets security-auditor's read (SA2-2's point 6).
  - 11 lines, at most 120 columns.
- **D9:** OVERFLOW-PANIC-RULE in the report, below. It is marked as an open security finding, and "a panic that
  network input or content data can reach" is part 20's own scope. 7 lines, at most 120 columns.
- **E:** draft E as worded wraps to 7 lines. A greedy wrap at 120 is line-optimal, so STATUS would be 121 lines. The
  six lines below name all three triggers. With the night-shift line compacted as the brief gives it (117 columns;
  not his words: no DECISIONS.md hit), STATUS stays at 120 lines. `wc -l docs/STATUS.md` must print 120 on the head.
- **F2 and F3:** the archive blocks below, at the archive's indent (4 + 2, continuation 6); 17 and 15 lines, at most
  120 columns each.
- **Also required** (not in drafts A-G): the report's "Plan and approval" bullet, the "As built" addition and round
  2's Reviews text and table, below. The docs-keeper sync of STATUS :5, :29 and :42-43 and of ROADMAP :125-126 (C3).

D3 and D4:
```
D3 (append to the M7 row after "the gate stayed green)"):
; `protocol`'s binary: cargo stops at the first failing test binary, so `sim`'s release test did not run in CI (it fails in the same state locally: the advisor agent's ruling D1, check C3, with `--no-fail-fast`)

D4 (in the E12 row, replace "the flag on `sim`, `room` and `sim_wasm`, none on"):
the flag on every workspace crate (the saved run lists `sim`, `room` and `sim_wasm`; rust-reviewer's and test-engineer's re-runs after a clean build, in review round 2, show all six natively and the five Wasm crates on wasm32), none on
```

D6 (Risks, replacing :213-215):
```
- With route A, an overflow in `room`'s or the future server's own arithmetic panics in release, as test builds do; the
  old blocks wrapped it silently (room's `100 - 150` as `u32` gave 4,294,967,246, a local run). By ADR 0001 part 11 and
  ADR 0004 part 20 (Proposed), a panic on the tick thread, where `room` runs, ends the match (repeatably, if a client
  drives the value); one in a connection's work closes only that connection; a browser trap ends that player's sim
  instance (online it resyncs; offline: OQ-0004-7). No server code yet; part 20's fuzz marks are not built
  (OVERFLOW-PANIC-RULE).
```

D7 (the report's OVERFLOW-XCRATE-CI row, replacing :231):
```
| OVERFLOW-XCRATE-CI | (Medium, the advisor agent's label) no CI step can yet show that the setting reaches sim's code compiled in another crate: K1b and M7 cannot tell route A from the old blocks. PM-0002-53's committed counter test runs, in its four builds, from a workspace crate that depends on sim, not from `protocol` (a test in `protocol` passes under part 4's two blocks too), and must go red with `Cargo.toml` set back to those blocks; it needs a planted counter in sim's normal build, a way to run Wasm (the tree has none) and CI lines for its release and Wasm halves (CI edits, each asked of him); anchored to the first change that adds a dependency on sim, or 2.3, whichever comes first (if nothing depends on sim when 2.3 starts, 2.3's plan adds a test-only dependency or asks the advisor agent to move the anchor); a STATUS follow-up | E12 (local); the ruling D1, condition 5; the R2-D4 skeptic's run and the advisor agent's ruling D1-R2 (in release, a counter test in `protocol` passes under both settings; in `room`, only under route A) |
```

D8 (the report's OVERFLOW-PER-CRATE, replacing :254-257):
```
- OVERFLOW-PER-CRATE: workspace crates are covered by the root setting now; `"*"` keeps every crate that is not a
  workspace member at the release default setting (a member's path dependency inside the repository joins the workspace
  and is checked, unless excluded; a `[patch]` path does not). It leaves a dependency's own arithmetic unchecked,
  concrete arithmetic inside its generic functions included; arithmetic its generic code does through a trait (`T: Add`,
  `Sum`) takes our crate's setting when our crate compiles it, but not when rustc reuses the dependency's own copy (an
  `#[inline(never)]` function the dependency already uses with the same types, at any opt-level; any shared generic it
  holds, at opt-level 0, 1, "s" or "z", where our own integer `sum()` and `product()` can then wrap too). The first
  third-party crate that any workspace crate uses decides, with its cost measured, between a named override for each
  such crate (it beats `"*"`) and dropping `"*"` (which checks every dependency, the server's future network crates
  included, so security-auditor reads it then); while `"*"` stays, moving the release profile, or one inheriting it, to
  opt-level 0, 1, "s" or "z" reopens it (the register row OVERFLOW-ALL).
```

D9 (new, after OVERFLOW-PER-CRATE in "Follow-ups for later steps"):
```
- OVERFLOW-PANIC-RULE (security-auditor's SA2-2, Medium, open): with route A, an overflow in `room`'s or a future server
  crate's own arithmetic panics in release instead of wrapping. ADR 0004 part 20 already forbids a panic that network
  input or content data can reach, but the detailed lines are narrower: part 4's explicit operations name counters and
  tick numbers, part 20's list of causes names no overflow, and the fuzz marks name no build setting. The plan of the
  first change that gives `room` or a server crate code (2.5 by ADR 0002's migration plan) states the rule its
  arithmetic follows; the lines go to their owners to weigh at each draft's next revision (the register row
  OVERFLOW-ALL).
```

E (STATUS.md: the 2.2 bullet replaces :95-99; the night-shift line replaces :65-66):
```
- **From foundation 2.2 (PR #48), for 2.3:** AUTO-ASK, MANUAL-MODE, GIT-C, ADVISOR-TOOL-WORDS, MODELS-2026-11-12,
  REFUSED-PUSH, PIN-GUARD, G0, OVERFLOW-XCRATE-CI (or the first crate to depend on sim), TSCONFIG, REQUIRED-CHECK,
  SETUP-NODE-CACHE, SAVER-SKIP-CI, CRATE-NAMES, CI-ENFORCED-LATER, WASM-ARTIFACT, TWO-PLAYWRIGHTS, TOOLKIT-RESOLVES-UP,
  CARGO-NET-OFFLINE, CRATE-LICENCE, CUT-CARRY, CUT-ESCAPE, SETPRIV-HARDEN, CANCEL-ON-MAIN, LINT-TABLE-PEDANTIC,
  FEATURE-UNIFY, ROOT-WORKSPACE, CLIENT-NET-RULE, NODE-26; later: OVERFLOW-PER-CRATE (with the first third-party crate),
  OVERFLOW-PANIC-RULE (with room's first code), PACKAGE-ENTRIES, EMPTY-SLOTS, OQ-0002-8-TEST, BRIEF-STALE.

- **Old "Night shift"** (trig_019LtRqmAdzddyk6K3e3kDJz): disabled, for him to delete (that also deletes its session).
```

F2 (STATUS_ARCHIVE, replacing OVERFLOW-PER-CRATE :779-784):
```
    - **OVERFLOW-PER-CRATE** (S3; rewritten on the advisor agent's ruling D1, condition 6, and widened on its ruling
      D1-R2, on review round 2's R2-D1): workspace crates are covered by the root setting now
      (`[profile.release] overflow-checks = true`); `[profile.release.package."*"] overflow-checks = false` keeps every
      crate that is not a workspace member at the release default setting (a member's path dependency inside the
      repository joins the workspace and is checked, unless excluded; a `[patch]` path does not). It leaves a
      dependency's own arithmetic unchecked, concrete arithmetic inside its generic functions included; arithmetic its
      generic code does through a trait (`T: Add`, `Sum`) takes our crate's setting when our crate compiles it, but not
      when rustc reuses the dependency's own copy (an `#[inline(never)]` function the dependency already uses with the
      same types, at any opt-level; any shared generic it holds, at opt-level 0, 1, "s" or "z", where our own integer
      `sum()` and `product()`, and sim's code compiled in our crates, can then wrap too). The first third-party crate
      that any workspace crate uses decides, with its cost measured, between a named override for each such crate (a
      named package override beats `"*"`) and dropping `"*"` (which checks every dependency, the server's future network
      crates included, so security-auditor reads it then: SA2-2's point 6); while `"*"` stays, moving the release
      profile, or one inheriting it, to opt-level 0, 1, "s" or "z" reopens it (the register row OVERFLOW-ALL, his go
      asked and still waiting). The runs: review round 2's, in the report's "Reviews" (rust-reviewer and the R2-D1
      skeptic for the shared generics and both closing options, repeated by the advisor agent's ruling D1-R2; the R2-D1
      skeptic and test-engineer for the dependency's generic code).
```

F3 (STATUS_ARCHIVE, "For later steps", new, after OVERFLOW-PER-CRATE):
```
    - **OVERFLOW-PANIC-RULE** (review round 2, security-auditor's SA2-2; Medium, open; handed to the owners by the
      advisor agent's ruling D1-R2 and the register row OVERFLOW-ALL): with route A, an overflow in `room`'s own
      arithmetic panics in release instead of wrapping (local native runs; test builds already panic), and a future
      server crate's code would do the same; on the server's tick that would end the match (ADR 0004 part 20),
      repeatably if a client can drive the value. ADR 0004 part 20's first sentence already forbids any panic reachable
      from network input or content data, but its list of panic sources names no overflow (:562-566), and the other
      lines are worded for part 4's two-crate setting: ADR 0002 part 4's explicit operations name counters and tick
      numbers (:247-249); part 7 leaves out `arithmetic_side_effects`, with REC4 as what covers overflow (:471, :1617);
      PM-0004-60 and PM-0001-34 name no build setting. Handed to the owners to weigh, each at its ADR's next revision,
      any wording with his go (engine-architect: ADR 0002; rust-reviewer: part 20; test-engineer: the marks), with the
      security review's proposals: `wrapping_*` only where a wrap is meant, `checked_*` or `saturating_*` with a defined
      result for values a client or content can set or add up; part 20 names overflow, and room's and the server's tick
      code beside the sim; the fuzz builds keep overflow checks on; `arithmetic_side_effects` weighed for room's command
      handling. Anchor: the plan of the first change that gives `room` or a server crate code (2.5 by ADR 0002's
      migration plan) states the overflow rule its arithmetic follows.
```

The report's "Plan and approval": a new bullet after the ruling D1 bullet (:47-49):
```
- The advisor agent's ruling D1-R2 (2026-11-13, on review round 2's items R2-D1 to R2-D4 and the fix pass's drafts): it
  keeps `"*"` and changes no setting, so no plan revision follows; it approves this fix pass's texts with its own
  changes: the register row OVERFLOW-ALL (the `"*"` choice comes with the first third-party crate in any workspace
  crate's dependency tree, and again before the release profile, or one inheriting it, moves to opt-level 0, 1, "s" or
  "z"; PM-0002-53's test runs from a workspace crate that depends on sim, not from `protocol`; test-engineer rewords
  PM-0002-53, and engine-architect also REC4's Appendix A row; the panic side goes to the drafts' owners to weigh,
  OVERFLOW-PANIC-RULE), `Cargo.toml`'s comment, the decision entry OVERFLOW-ALL and the follow-ups; it relabels R2-D4
  from Low to Medium.
```

The report's "As built": appended to the overflow bullet (:72-78), which otherwise stays:
```
  On review round 2 only the wording changed, no setting (the advisor agent's ruling D1-R2): `Cargo.toml`'s comment
  names the one case where `"*"` reaches our crates (the release profile, or one inheriting it, at opt-level 0, 1, "s"
  or "z" with a dependency present), and the row, the decision entry and the follow-ups OVERFLOW-XCRATE-CI,
  OVERFLOW-PER-CRATE and OVERFLOW-PANIC-RULE say what that round found.
```

The report's Reviews: replaces ":166 Round 2 ... (pending)" with the paragraph, the run lines (the row cites them)
and the table. The session may reword cells, but not the severities, the dispositions or the run results.
```
Round 2, on head 4bccf55 (2026-11-13): rust-reviewer, security-auditor, test-engineer and engine-architect, each on Opus
5.5 at effort max. rust-reviewer, test-engineer and engine-architect APPROVE; security-auditor found two Medium and one
Low; nothing is Critical or High, and none calls for a round 3. 18 findings, deduped by the session into 12 items: R2-D1
to R2-D4 were each tested by a skeptic of its own (Opus 5.5 at max), and all four hold, at Medium; the session checked
R2-L1 to R2-L8 inline. The advisor agent's ruling D1-R2 settled the open points (Plan and approval); security-auditor's
SA2-2 stays open as the follow-up OVERFLOW-PANIC-RULE.

Round 2's local runs, which the register row OVERFLOW-ALL cites (cargo 1.99.1, scratch copies of 4bccf55):
- Shared generics: with route A, the release profile at opt-level 0, 1, "s" or "z" let a workspace crate's own integer
  `sum()`, and sim's `#[inline]` sum compiled in it, run a dependency's unchecked copy (200 + 100 as `u8` gave 44) when
  that dependency, a crate outside the workspace, was in the same build; at opt-level 2 and 3, and in the test profile,
  both panicked; dropping `"*"`, or a named override for the dependency, closed it (rust-reviewer; the R2-D1 skeptic;
  the advisor agent's ruling D1-R2). The prebuilt std holds no such copy (the session).
- `room`'s own arithmetic: with part 4's two blocks, release gave 4,294,967,246 for `100 - 150` as `u32` and 0 for
  `u16::MAX` plus one; with route A both panicked; the test profile panicked under both (security-auditor; the R2-D2 and
  R2-D3 skeptics).
- The test's home: a planted counter test in `protocol` passed in release under both settings; the same test in `room`
  failed under part 4's two blocks ("test did not panic as expected") and passed under route A (the R2-D4 skeptic; the
  advisor agent's ruling D1-R2).

| Id (reviewer ids) | Severity | Finding | Disposition |
|---|---|---|---|
| R2-D1 (RR2-1, TE2-1, RR2-2) | Medium | what `"*"` reaches: at opt-level 0, 1, "s" or "z" a workspace crate can run a dependency's unchecked copy of shared generic code; a dependency's trait generics compiled in our crate are checked; `"*"` follows membership, not folders | fixed: `Cargo.toml`'s comment, the row OVERFLOW-ALL and OVERFLOW-PER-CRATE; `"*"` kept (the advisor agent's ruling D1-R2) |
| R2-D2 (SA2-1) | Medium | the panic outcomes worded as "likely ... (inferred)" from the wrong rule, and an IOU to round 2 | fixed: Risks and the entry OVERFLOW-ALL |
| R2-D3 (SA2-2) | Medium | no record hands route A's panic side to the drafts' owners | the row's hand-off and the follow-up OVERFLOW-PANIC-RULE: a security finding left open |
| R2-D4 (TE2-2, TE2-3, EA2-4) | Medium (the reviewers' Low, relabelled by the advisor agent's ruling D1-R2) | OVERFLOW-XCRATE-CI: the test's home admits `protocol`, where it proves nothing; two missing constraints; no tripwire | fixed: the row, OVERFLOW-XCRATE-CI and STATUS's trigger |
| R2-L1 (TE2-6, RR2-3) | Low | M7's row names no failing binary; "M7 proves the setting the tree ships" reads wider than M7 shows | fixed: As built and M7's row |
| R2-L2 (TE2-7, EA2-1) | Medium | the entry's "misses most of the simulation's code" | fixed: the entry |
| R2-L3 (TE2-4) | Low | E12's saved record is thin | fixed: E12's row; the advisor agent's head-review re-run keeps the summary lines and a wasm32 control |
| R2-L4 (TE2-5) | Low | "revisions 1 to 9" | fixed |
| R2-L5 (SA2-3) | Low | the D5 row and the capability-set sentence overstate round 1 | fixed |
| R2-L6 (RR2-4) | Low | sim-wasm's "comes first"; D13's REPO_MAP half | fixed: sim-wasm's doc comment and REPO_MAP's crates row |
| R2-L7 (EA2-2) | Low | the entry asks only about the setting | fixed: the entry |
| R2-L8 (EA2-3) | Low | the row's reword list leaves out REC4's Appendix A row | fixed: the row |
```

### Conditions
- C1. Fold exactly the texts above, and the as-drafted ones, in one push.
  - No setting change, `Cargo.lock` unchanged, no crate change but sim-wasm's doc comment, no plan revision.
  - No ADR text edit, no CI or kit file.
- C2. Because the row cites them, the report carries round 2's runs (the run lines above). It lists SA2-2 as left
  open (OVERFLOW-PANIC-RULE).
  - The round-2 paragraph does not say "all four APPROVE": security-auditor's notes give a count verdict (F9), unless
    its handback says APPROVE in words.
- C3. STATUS keeps exactly 120 lines. `wc -l docs/STATUS.md` = 120 on the head.
  - The 2.2 bullet and the night-shift line are as given.
  - docs-keeper syncs STATUS :5, :29 (#48's row: round 2 and its fix pass done, D1-R2) and :42-43 (OVERFLOW-ALL
    still waiting) and ROADMAP :125-126 ("Next:" moves past review round 2), adding no STATUS line.
- C4. The new head needs, before I approve it:
  - CI green on it: toolchain (push and pull_request), gate and smoke;
  - a local gate PASS;
  - `Cargo.lock` unchanged;
  - E9 re-run on the final file list (sim-wasm's lib.rs is in it);
  - the verifier's PASS on the final head;
  - the Fable advisor's whole read, where his item 4 places it;
  - my review of that head (not a round): I check the fold against this ruling, run the gate myself, and re-run the
    cross-crate script, keeping its summary lines and adding a wasm32 no-overflow control (R2-L3). D1's 7d and 7f
    stand.
- C5. The budget: any later finding below High becomes a report line or a follow-up. A High follows the budget
  (round 3 only for it).
- C6. The report carries this ruling's substance (Plan and approval, As built, Reviews). The checkpoint files go, as
  always, in the last commit before ready.
- C7. Severities as above: R2-D4 Medium by my relabel; R2-L2 Medium.

### Not run by me (my claims rest only on F3, F5 and F7 and the reads F1, F2, F6 and F8)
- TE2-1's split, which D8 and F2 word: a dependency's trait generics compiled in our crate are checked at opt-level 3,
  and the `#[inline(never)]` exception. These are the R2-D1 skeptic's K5 and K9 and test-engineer's probe.
- RR2-1 on wasm32 (rust-reviewer's C15), Product and Sum<&u8> (C16), fat LTO (m3), and registry crates (path crates
  only).
- The no_mangle refusal under the workspace lints (the R2-D4 skeptic; test-engineer).
- cargo-fuzz's defaults and the lock-poison spread (the R2-D3 skeptic; security-auditor).
- Room's own arithmetic in this round: I ran room's own add in D1's C4, not the 100 - 150 case.
- CI on any new head (none yet), Windows, the report beyond :1-330, and STATUS_ARCHIVE beyond the overflow items and
  their neighbours.

