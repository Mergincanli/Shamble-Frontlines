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
