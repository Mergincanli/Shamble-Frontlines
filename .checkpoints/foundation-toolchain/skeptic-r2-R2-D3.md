# Skeptic, review round 2, R2-D3 (merges SA2-2), PR #48 head 4bccf55

Plan: read SA2-2's full text and the auditor's threat model; read every cited line myself (ADR 0002 part 4 :247-278,
ADR 0004 part 20 :560-575 and PM-0004-60 near :1181, r5 :1350-1360 and :1788-1789, README:234, advisor-ruling-d1.md);
grep the ADRs, register, STATUS, STATUS_ARCHIVE and the report for any record that already hands overflow-panic rules
to an owner; check Cargo.toml and CI for what the shipped setting does; then judge (a)-(e) and the fix wording.

## Checks
- [checked] head: worktree HEAD 550856a; `git diff --stat 4bccf55 HEAD -- . ':(exclude).checkpoints'` is empty, so the
  content is 4bccf55's.
- [read] SA2-2 in full (security-auditor-review-r2.md:149-173), its focus-1 answer (:112-136), corrections (:236-247),
  durable decisions (:221-227); R2-D3's row (review-r2-findings.md:15); the ruling D1 (advisor-ruling-d1.md), whose
  condition 7(e) sent "panic-vs-wrap for room/server own arithmetic" to round 2's security-auditor (:153) and set no
  rule or follow-up for it.
- (a) ADR 0002 :247-249 [read]: "counters and tick numbers use explicit `wrapping_*` or `checked_*` operations, so a
  debug client and a release server never disagree on overflow". Holds on the value class (counters and tick numbers
  only) and the purpose (client/server agreement, not "no panic"). The crate scope "in sim and protocol" is a reading of
  the sentence (its first clause names the two crates; the explicit-operations clause itself names none; Appendix A's
  R1-RR-OQ4 answer, :1562, pairs them the same way). Fair, but say "names counters and tick numbers, beside sim and
  protocol" rather than "covers only ... in sim and protocol".
- (a) ADR 0004 :562-566 [read]: first sentence "No panic is reachable from network input or content data" (general,
  no crate named); the named sources are `unwrap`, an unchecked index and std's `clamp`, "inside the sim". Integer
  overflow is not named. Holds as stated ("names no overflow among its panic sources"); note the general first
  sentence does reach an overflow panic in plain words (SA2-2 itself says so), and :566-575 put the tick-thread panic
  rule on the server (room), so part 20 is not sim-only in reach.
- (a) r5 :1353-1358 [read]: REC4's reason ends "so the rest of the workspace keeps the release default". Holds; route
  A reverses it. r5 :1788-1789 [read]: OQ2, `panic` and `arithmetic_side_effects` "not read ... candidates for REC3 and
  REC4". Holds.
- (a) PM-0004-60 (ADR 0004 :1181) [read]: cargo-fuzz target, CI nightly; no profile, flags or overflow setting named;
  its "Bites" plants an `.unwrap()`, no overflow; room's control-message sequences fuzzed "once `room` exists ... since a
  panic there ends the match too (part 20)". "Pins no profile": holds literally. PM-0001-34 (ADR 0001 :1626): same, no
  profile.
- (a) README:234 [read]: the row's "dropping `"*"`" sits in "that is decided when the first third-party crate enters
  sim's dependency tree, with its cost measured (a named override ... or dropping "*")": only cost is named, not reach.
  Holds.
- (b) found in passing, strengthens the finding: ADR 0002 part 7 :471 "The `panic` and `arithmetic_side_effects` lints
  are not added: their sources were not read", and Appendix A :1617 answers R5-RUSTLINT-OQ2 "not added ...; REC3's
  lints and fuzzing cover panics, REC4 overflow": an ADR answer that rests on REC4's two-crate scope; the row hands the
  owner part 4, R1-RR-OQ4's answer and PM-0002-53, not OQ2's answer.
- [ran] own reproduction, /home/user/Shamble-Frontlines/.gate/skeptic-r2-R2-D3/{routeA,p4} (git archive 4bccf55 of
  Cargo.toml, Cargo.lock, rust-toolchain.toml, crates/; p4 has 4d78ad6's Cargo.toml), cargo/rustc 1.99.1, --locked
  --offline; added only crates/room/tests/skeptic_overflow.rs (room's own `credits - price` u32 and `c + 1` u16,
  #[inline(never)], black_box, catch_unwind; controls 150-100 and 5+1). Route A: release and test profile both panic
  (buy(100,150), bump(u16::MAX)); controls Ok(50), Ok(6). Part 4's blocks: test profile panics; release Ok(4294967246)
  and Ok(0). So route A turns room's own release arithmetic from a silent wrap into a panic: the premise holds (agrees
  with SA2-2's scratch run and the ruling's C4). room's doc comment at 4bccf55: "Empty until the skeleton step
  (foundation 2.5)"; no `server` crate exists yet.
- [ran] point (6), /home/user/Shamble-Frontlines/.gate/skeptic-r2-R2-D3/star/ (route A tree + a path crate `extdep`
  outside the workspace folder, room depends on it, #[inline(never)] `ext_add(255, 1)`, release, -v, fresh target
  folders): with "*": extdep gets no overflow flag, room `-C overflow-checks=on`, ext_add = Ok(0) (wraps). "*" removed:
  extdep and room both `-C overflow-checks=on`, ext_add = PANIC. So dropping "*" checks every non-member crate, which
  would include the server's network crates once they exist (agrees with the ruling's S3). The row names only the
  option's cost (README:234). Holds.
- (b) [grep] REPO_MAP, ROADMAP, STATUS, DECISIONS, STATUS_ARCHIVE (all of it), the report: no record hands the panic
  side to an owner with an anchor. What exists: the report's Risks bullet :213-215 (names it, then an IOU to "Round 2's
  security review"); the entry OVERFLOW-ALL :299-302 (tells Mustafa "would likely end that match (inferred)");
  OVERFLOW-XCRATE-CI (STATUS_ARCHIVE :617-632) and OVERFLOW-PER-CRATE (:779-784) are the determinism side only. ADR
  lines that partly cover it, none of which names route A's change: ADR 0004 part 20's first sentence (plain words);
  ADR 0001's failure row :776 and PM-0001-58 (every wire tick and sequence counter tested just below its wrap, an L1
  test: counters only); ADR 0002 part 7 :465-468 (unwrap/expect/indexing at deny in the sim's command module: no
  overflow). engine-architect's round-2 notes :39 concur ("SA2-2's panic side belongs in the row's hand-off list";
  part 4's rule "is mine to reword"); its EA2-3 (Low) also edits the row's hand-off list (adds Appendix A's REC4 row,
  :1586 "adopted"; notes route A's cost falls on room and the server too). The two edits must be one clause.
- ADR 0002's migration plan [read]: room gets code at 2.5, "the client + local server skeleton" (:1467-1472; room's
  doc comment at 4bccf55 says the same), and runs there in the Worker, in Wasm, so an overflow traps the local sim
  (local recovery not researched, ADR 0004 OQ-0004-7); `server` enters at slice 3, online (:1489). So "room's first
  code" is 2.5 at the latest by plan, sooner if the next builder writes room code.
- [read] cargo-fuzz source at main (raw.githubusercontent.com, rust-fuzz/cargo-fuzz, src/options.rs and src/project.rs,
  read 2026-11-13, as data; no cargo-fuzz version is pinned here): `-a` "Build artifacts with debug assertions and
  overflow checks enabled (default if not -O)"; project.rs passes `--release` unless `--dev`, and adds
  `-Cdebug-assertions` "if !build.release || build.debug_assertions || build.careful_mode". So a default `cargo fuzz`
  build checks overflow whatever the workspace; `-O` without `-a` does not. Point (4) is a pin against `-O`, small.
  "PM-0004-60 pins no profile" still holds literally.
- [checked] STATUS cap: docs-keeper.md:14 quotes Mustafa's STATUS-DIET: "docs/STATUS.md: at most 120 lines."
  `wc -l docs/STATUS.md` = 120, so any STATUS edit in the fix pass must add no line (STATUS.md:99 is 113 characters;
  other lines run to 1436, so a longer :99 is allowed, a new line is not).
- Severity: Medium holds. Real record gap for a consequence the PR's own change brings (my run); nothing reaches a
  player or session today (room is an empty stub, no server crate); part 20's first sentence already forbids it in
  plain words, but nothing anchors the rewording to room's first code. Not High: no rule of his loosened (ADR 0002 part
  4 not accepted; route A waits for his go), nothing of his misrecorded, no evidence fails.
- (c) A clause in the row is right, on four conditions: (1) its own provenance: the row's State says "the advisor
  agent's ruling (D1)", whose condition 3 lists the row's content with no panic hand-off and whose condition 7(e) sent
  the question to round 2; (2) a tested fact plus a pointer only, no rule content (the row falls under his merge go;
  rule content belongs in the archive as the security review's proposal for the owners); (3) the decision entry
  OVERFLOW-ALL (R2-D2's rewrite) points to the follow-up, so his go covers it knowingly (EA2-2's point); (4) one edit of
  README:234 with R2-D1's, EA2-3's and EA2-4's, not repeating the disputed "dependencies keep the release default,
  unchecked".
- (d) Owners per line: engine-architect (ADR 0002 parts 4 and 7, Appendix A), rust-reviewer (ADR 0004 part 20),
  test-engineer (PM-0004-60, PM-0001-34), netcode-architect (ADR 0001, if point 3 is kept). Anchor a builder can act on
  without an ADR revision: the plan of the first change that gives `room` or a server crate code states the overflow
  rule its arithmetic follows (2.5 at the latest by ADR 0002 :1467-1472); ADR wording at each ADR's next revision, with
  his go. Tripwire (R2-D4's same defect): the name with its trigger in STATUS's "later" group (:99), no new line.
  Mirrors: STATUS_ARCHIVE "For later steps"; the report's "Follow-ups for later steps" (:253); R2-D2 rewrites the Risks
  IOU and the entry to point here.
- (e) Points (1), (2), (4): in the follow-up as lines to revisit, their content the owners' to decide. (5): a question
  only (ADR 0002 :471, :1617 already answered "not added", its reason "REC4 overflow" now rests on the old scope). (3):
  not route A's (any handler panic poisons a std Mutex; ADR 0001 :495-496 covers only a lock the tick unwraps; not
  run): out, or one clause as input for netcode-architect. (6): into OVERFLOW-PER-CRATE and the row's dependency
  sentence as a tested fact (a path crate's own arithmetic), not into the new follow-up.
- also_noticed: review-r2-findings.md predates engine-architect's findings (its :3 "pending"); EA2-1 (Medium,
  engine-architect-review-r2.md:34) is the same sentence as R2-L2 (Low, review-r2-findings.md:18, TE2-7), so the merged
  item takes Medium; EA2-3 and EA2-4 also edit README:234.
- Scratch leftovers (not deleted): /home/user/Shamble-Frontlines/.gate/skeptic-r2-R2-D3/ (routeA/, p4/, star/,
  target-a/, target-p4/, target-star1/, target-star2/, target-star3/, star1.log, star2.log). Runs: native only, toy
  functions, a path crate (not a registry crate); no wasm32 run of room's own arithmetic.
- Not verified: cargo-fuzz at any pinned version (read at main only); a registry crate without "*" (path crate only);
  the lock-poison spread (point 3); room's own arithmetic in a wasm32 build (the ruling's C7 ran sim functions only).

## Exact wording proposed (corrected after the final advisor consultation; supersedes the first draft)
Corrections: the entry sentence no longer says the rules are "not yet written" (ADR 0004 part 20's first sentence
already forbids a reachable panic); "are reworded by their owners" became "handed to their owners to weigh" (only
engine-architect concurred, and (e) leaves the content, and whether to reword at all, to each owner); the cargo-fuzz
parenthetical left the archive text (read at main through a summarising fetch, never run here); point (6) keeps its
"path crate" qualifier; "a server crate" is not claimed as run; "2.5 at the latest" became "2.5 by ADR 0002's migration
plan".
- Row README:234, third column, before "Shown by" (one edit with R2-D1, EA2-3, EA2-4): "Added at review round 2
  (security-auditor's SA2-2), not part of the ruling: with this setting an overflow in `room`'s own arithmetic panics in
  release too, where part 4's two blocks let it wrap (review round 2's local native runs), and a future server crate's
  code would do the same. The lines that keep a value a client or content can set from reaching such a panic (part 4's
  explicit operations, which name counters and tick numbers; ADR 0004 part 20, whose list of panic sources names no
  overflow; the fuzz marks PM-0004-60 and PM-0001-34) are handed to their owners to weigh: the follow-up
  OVERFLOW-PANIC-RULE."
- Row's dependency sentence, point (6): "(a named override for that crate, which beats `"*"`, or dropping `"*"`, which
  would also check every dependency, the server's future network crates included, as a local run showed for a path
  crate's own arithmetic)"; the same words in OVERFLOW-PER-CRATE (archive :779-784, report :254-257).
- STATUS.md:99, no new line (Mustafa's 120-line cap): "later: OVERFLOW-PER-CRATE, OVERFLOW-PANIC-RULE (with room's
  first code), PACKAGE-ENTRIES, ...".
- STATUS_ARCHIVE, "For later steps", after OVERFLOW-PER-CRATE: "**OVERFLOW-PANIC-RULE** (review round 2,
  security-auditor's SA2-2; Medium): with route A, an overflow in `room`'s own arithmetic panics in release instead of
  wrapping (a local native run; test builds already panic), and a future server crate's code would do the same; on the
  server's tick that would end the match (ADR 0004 part 20), repeatably if a client can drive the value. ADR 0004 part
  20's first sentence already forbids any panic reachable from network input or content data, but its list of panic
  sources names no overflow (:562-566), and the other lines are worded for part 4's two-crate setting: ADR 0002 part
  4's explicit operations name counters and tick numbers (:247-249); part 7 leaves out `arithmetic_side_effects`, with
  REC4 as what covers overflow (:471, :1617); PM-0004-60 and PM-0001-34 name no build setting. Handed to the owners to
  weigh, each at its ADR's next revision, any wording with his go (engine-architect: ADR 0002; rust-reviewer: part 20;
  test-engineer: the marks), with the security review's proposals: `wrapping_*` only where a wrap is meant, `checked_*`
  or `saturating_*` with a defined result for values a client or content can set or add up; part 20 names overflow,
  and room's and the server's tick code beside the sim; the fuzz builds keep overflow checks on; `arithmetic_side_effects`
  weighed for room's command handling. Anchor: the plan of the first change that gives `room` or a server crate code
  (2.5 by ADR 0002's migration plan) states the overflow rule its arithmetic follows."
- Report, "Follow-ups for later steps", after OVERFLOW-PER-CRATE: "OVERFLOW-PANIC-RULE: with route A, an overflow in
  `room`'s or a future server crate's own arithmetic panics in release instead of wrapping. ADR 0004 part 20 already
  forbids a panic a client can reach, but the detailed lines are narrower: part 4's explicit operations name counters
  and tick numbers, part 20's list of causes names no overflow, and the fuzz marks name no build setting. The plan of
  the first change that gives `room` or a server crate code (2.5 by ADR 0002's migration plan) states the rule its
  arithmetic follows; the lines go to their owners to weigh at each ADR's next revision (the register row
  OVERFLOW-ALL)."
- Entry OVERFLOW-ALL, "Why it matters": an addition to R2-D2's rewrite, not a competing replacement (plain words): "With
  the setting, a number that grows too big in the future server's own code can end that match instead of quietly
  carrying on with a wrong value. The design records already say no player input may crash a match, but their
  detailed lists do not name this cause yet, so the first plan that writes that code must say how it avoids it, and
  the records' owners weigh adding it (OVERFLOW-PANIC-RULE)."

## Verdict
R2-D3 holds, Medium. The fix is right in shape (a named follow-up plus a row clause), incomplete as written: provenance
in the row, hand-off ("to weigh") not commitment, an owner per line, a builder-actionable anchor, the STATUS trigger
without a new line (120-line cap), the report's follow-up list, one combined edit of README:234, point (6) moved to
OVERFLOW-PER-CRATE, Appendix A :1617 added. Wording: the section above. Returned through StructuredOutput.
