# Advisor ruling D1 (RR-1: overflow checks do not reach cross-crate sim code) - notes

Seat: advisor agent, day session, run as a general agent reading main's .claude/agents/advisor.md (2aa5e9c).
Worktree: .gate/foundation-toolchain-wt, branch claude/laughing-pasteur-fwya24.
Head named in the brief: 4d78ad61e652f2e35ddf51c49557e725420248c8. Worktree HEAD at start: 04814cf
(4d78ad6 is its ancestor; 4d78ad6..04814cf touches only .checkpoints/ files, so the reviewed content is the same).
Scratch copies only under /home/user/Shamble-Frontlines/.gate/advisor-d1/.

## Plan
1. Read: role file (done), root Cargo.toml, ADR 0002 part 4 (:247-278) and PM-0002-53 (:1319), the plan (Part 2,
   Part 7, As built), the report draft, RR-1 in review-r1-findings.json, skeptic-r1-D1.md (C1-C15), D1 in
   review-r1-verdicts.json, the reproduction script.
2. Check the mechanism myself in a scratch copy: (a) the per-package override misses cross-crate inline/generic code
   (re-run the reviewer's script or an equivalent); (b) route A (release overflow-checks = true + "*" = false) covers
   workspace members and leaves dependencies off; (c) whether "*" also matches workspace members (Cargo docs:
   "*" = all non-workspace members? verify against cargo's own behaviour); (d) bench/custom profile inheritance;
   (e) generic sim code instantiated in a dependency crate (e.g. a third-party generic calling into sim) - edge case.
3. Weigh routes A/B/C against ADR part 4's words, the plan's pinned mechanism, the review budget and the escalation
   categories (ADR = decision doc; rule change?).
4. Answer questions 1-6, verdict, conditions.

## Checks (one line each, as done)
- R1 read: main's advisor.md (2aa5e9c); root Cargo.toml (:21-26 the two per-package blocks, comment ":21 Overflow
  checks in every profile for the simulation and the protocol"); ADR 0002 part 4 (:247-249: "Overflow checks are on for
  `sim` and `protocol` in every profile (`[profile.release.package.sim] overflow-checks = true`, the same for
  `protocol`) ... so a debug client and a release server never disagree on overflow"; Why: "Cargo's dev and release
  profiles differ on overflow by default (R5-RUSTLINT-F13), and one remote panic must not end a 48-player match");
  PM-0002-53 (:1319, "a committed test in `sim` run in the four builds"); ADR status: Proposed, part 1 accepted only;
  author engine-architect.
- R2 read: RR-1 (High) with fix A/B/fallback; RR-2, RR-3 (Low); EA-1..EA-5 (Low; EA agrees RR-1 stands, favours A,
  says rewording part 4 is the ADR owner's job, not this PR's); skeptic D1 C1-C15 (holds, High; fix A right and
  complete for D1; costs: evidence (M5/E1 prove deleted blocks), ADR wording, room/server own arithmetic panics,
  K1b cannot tell A from the blocks); verdicts D1 entry (same).
- R3 read: plan Part 2 Overflow row pins the per-package form verbatim, source "part 4", proof E1; Part 7 planted-
  failure procedure; As built step 5 (K1b, M5 proves protocol's block only; sim's rests on step 2's local control).
- R4 read: report draft :19 (Outcome: "overflow checks in the release profile for sim and protocol"), :93 (E1 incl. the
  step-2 control), :102 (M5), :167 (OVERFLOW-WASM: "CI proves native release for protocol (M5)"), :182-183
  (OVERFLOW-PER-CRATE), :337-344 (A10 item 2, his words: "K1b (a). Add the one line ... cargo test --release -p sim -p
  protocol --locked. Prove it with the planted failure as you described: remove the release overflow setting, wait
  for the red run, then revert exactly. Any other change to the CI job asks me again.").
- R5 read: the PR changes docs/adr/README.md only (row Q2.2-a, "answered by the advisor agent ...; His go for this row:
  PR #48's merge"), never ADR 0002's text. Register convention (README :24-27): answers are recorded in the register and
  "ADR 0005 takes them up at its next revision" - the precedent for leaving the ADR text to its owner.
- R6 next builder: Mustafa's message of 2026-11-12 (item 3, item 4): Fable 5.2 writes game and engine code on his
  personal account; "Fable will build the game on it" (the skeleton). So the first cross-crate sim call comes from the
  next builder, possibly before 2.3's gate (C11 holds).
- R7 stale lines under any fix: REPO_MAP.md:16 ("overflow checks in the release profile for `sim` and `protocol`"),
  :18 (tests pass only while checks are on: still true), ROADMAP.md:116, STATUS_ARCHIVE.md:590-592 (OVERFLOW-WASM),
  :631-632 (OVERFLOW-PER-CRATE: "needs the same block"; under fix A workspace crates are covered, but a THIRD-PARTY maths
  crate is not: "*" leaves it at the release default; a named override beats "*", so the item is rewritten, not moot).
- R8 toolchain here: rustup 1.28.2, cargo/rustc 1.99.1 (58cf8f9c3), wasm32-unknown-unknown installed, node v22.23.4.
- Working view before own checks: route A; keep "*" (smallest fix for D1, ADR never asked for dependency checks,
  identical today; dependency question anchored to the first third-party crate in sim's tree); no ADR text edit, a
  register row Q2.2-b beside Q2.2-a (convention + precedent), its own Decisions-needed entry; ruling = approval of the
  plan change on conditions; evidence M7 + local cross-crate re-run on the fixed tree (+ wasm32, flags, bench/custom
  profile) + M5/step-2 control relabelled historical + round 2 (rust-reviewer, test-engineer, security-auditor for the
  panic-vs-wrap change, engine-architect for the register row); no ESCALATE: the widening is forced by the mechanism
  (codegen crate's session decides optional overflow checks), so "sim checked wherever it runs" implies "every crate
  that generates sim code is checked".
- R9 register home: "Decisions the drafts ask for" says agent-decided questions are not listed there (README :44-45);
  Q2.2-a sits there only because Mustafa named it ("The ADR index entry ... I will give my go on it in the pull
  request"), which covers no second row. The loose-ends register's table "The critic's remaining items and the answered
  escalations" holds Q-A1 (README :233, on main): "the advisor agent's reading (Q-A1, 2026-11-10), recorded here on
  Mustafa's D1 (a) of 2026-11-11, never his decision; the PM row's own wording is fixed at ADR 0002's next revision".
  That is the template for this ruling's row; it needs its own Decisions-needed entry (his go), worded as asked, not given.
- (advisor tool consulted once after orientation: keep the route; split the CI cross-crate test out as its own named
  finding; scope the APPROVE; assert only what I ran; reproduce first.)
- C1 reviewer's script re-run (fresh folder .gate/advisor-d1/repro, head 4d78ad6, cargo 1.99.1): test profile 5 of 5
  panic; release with the head's blocks: plain, inline, generic, Tick::next FAILED ("did not panic"), only
  #[inline(never)] panics; route A: 5 of 5 panic. Matches RR-1 and skeptic C2.
- C2 route A on a pristine copy of 4d78ad6 (.gate/advisor-d1/m7; lines 21-26 replaced by the two route-A tables), CI's
  exact lines: `cargo test --workspace --locked` 6 of 6; K1b `cargo test --release -p sim -p protocol --locked` 2 of 2,
  exit 0, no Cargo warning ("*" matches nothing today); `cargo build --locked --target wasm32-unknown-unknown -p sim
  -p protocol -p room -p client-endpoint -p sim-wasm` exit 0; Cargo.lock byte-identical to the head's (cmp).
- C3 M7 locally (route A with `[profile.release] overflow-checks = true` removed, "*" kept), K1b's exact line: protocol
  "test did not panic as expected at crates/protocol/src/lib.rs:10:8", exit 101 (cargo stops at the first failing
  binary); with --no-fail-fast both protocol and sim fail ("2 targets failed"), so the one line governs both and one
  CI red proves it (unlike M5, which proved protocol's block only). Restored route A afterwards.
- C4 coupling at profile level (.gate/advisor-d1/coupling: sim gets #[inline] add_inline and generic add_generic;
  room depends on sim and has its own `pub fn room_add(a: u8, b: u8) -> u8 { a + b }`; values via catch_unwind, 255+1):
  debug (test profile): room_own_add panicked, sim::add_inline from room panicked, sim::add_generic from room panicked.
  release, head's blocks: all three returned 0 (wrapped). release, head's blocks + [profile.release.package.room]
  overflow-checks = true (route B for room): all three panicked. release, route A: all three panicked.
  So among profile settings, sim's code generated in room is checked exactly when room's own arithmetic is checked: the
  widening to callers' own arithmetic is inherent to any profile-level fix (A or B), not a separate choice. And route A
  makes release do what debug and test builds already do for room (they panic today). Not a claim about non-profile
  routes (e.g. a lint forcing explicit checked/strict ops in sim): not tested, not needed here.
- C5 "*" and named overrides (.gate/advisor-d1/star; extdep is a path crate outside the workspace folder, confirmed not
  a member by `cargo metadata`: members = the six crates; room depends on it; values via catch_unwind, 255+1, release):
  S1 route A: rustc flags `--crate-name extdep` (no overflow flag), `--crate-name room -C overflow-checks=on`; both
  ext_add (small, inlinable) and ext_add_never returned 0 (wrapped), so a non-member's arithmetic stays unchecked in
  release even when called from checked room code (its MIR carries no checks).
  S2 route A + [profile.release.package.extdep] overflow-checks = true: extdep and room both get -C overflow-checks=on;
  both functions panicked. So a named override beats "*" (tested, not inferred).
  S3 [profile.release] overflow-checks = true without "*": extdep and room both -C overflow-checks=on (after cargo clean
  of the two packages); both panicked.
- C6 settings spike item 8 may pick, on the reviewer's repro (room calls sim's plain, inline, generic, Tick::next and
  #[inline(never)] functions, release): route A passes 5 of 5 at opt-level "z", at "s", and with fat LTO plus
  codegen-units 1 (env overrides CARGO_PROFILE_RELEASE_*). The head's blocks fail the same 4 of 5 at opt-level "z" and
  at opt-level 1, so no opt-level choice closes the hole. (Not run before by the reviewer or the skeptic.)
- C7 Wasm by behaviour (.gate/advisor-d1/wasm: scratch sim-wasm cdylib exporting step_inline -> sim::tick_next
  (#[inline] t + 1) and step_generic -> sim::add_generic (a + 1); run in Node 22.23.4 with u32::MAX): dev build (the
  debug client), head's blocks: both trapped (RuntimeError: unreachable). Release, head's blocks: both returned 0
  (wrapped), 354 bytes. Release, route A: both trapped, 20,379 bytes (raw sizes of a toy export: the jump is the panic
  machinery, not a budget). So under the head's blocks a debug client and a release client disagree on the overflow;
  under route A they agree. Matches skeptic C5/C6.
- C8 inheritance under route A (repro copy + `[profile.ci-release] inherits = "release"`): `cargo test --profile bench
  -p room` 5 of 5 cross-crate panics; `--profile ci-release` 5 of 5; sim's and protocol's own tests pass under bench.
- C9 the head moved while I worked: worktree HEAD aee92cc; since 4d78ad6 the non-checkpoint changes are 1de98a9
  ("fix(review round 1)"): doc comments in crates/{sim,room,client-endpoint,sim-wasm}/src/lib.rs (EA-2, EA-3) and the
  report (required-check facts, E1/E2 wording, findings for 2.3). Cargo.toml and every crate's code are unchanged; the
  report's overflow wording (E1's step-2 control, M5, OVERFLOW-WASM, OVERFLOW-PER-CRATE) is unchanged, waiting for this
  ruling. So C1-C8 (run on 4d78ad6's Rust files) describe the current head's behaviour too.
- Not run by me: the legacy gate (no head with the fix exists); CI on route A (none yet); protocol in the cross-crate
  experiment (same mechanism as sim); Windows; any cost measurement (no code); non-profile routes (a lint forcing
  explicit checked/strict ops in sim); I did not re-read the rustc sources RR-1 and the skeptic cite: my claims rest on
  the behaviour runs above.

## Ruling (written before the final consultation)
APPROVE route A, on conditions 1-7. Scope: the route and the one change to the approved plan's pin (Part 2's Overflow
row); this ruling is the plan approval for that change. It approves no head (none carries the fix yet) and does not
cover the as-built changes of plan revisions 4-9 (my final-head review does). RR-1 stays High; it is fixed, not
deferred, so no relabel.
1. Setting: Cargo.toml:21-26 (two per-package blocks) -> exactly `[profile.release] overflow-checks = true` and
   `[profile.release.package."*"] overflow-checks = false`; no other profile key (spike item 8 / part 4 defaults), no
   `[profile.dev]` line; a short comment: every workspace crate, every profile (part 4); why not the per-package form
   (sim's inline/generic/small functions compile inside the callers); "*" = non-members only, dependencies keep the
   release default; the register row's code.
2. Plan revision 10: Part 2's Overflow row (value, source "part 4 widened by the advisor agent's ruling D1,
   2026-11-13, on RR-1", proof M7 + the local cross-crate run); As built; Part 7 gains M7 and the local run. Nothing
   else changes (no crate code or dependency, no CI edit, no ADR text edit); anything beyond comes back first (a CI edit
   asks Mustafa, A10). No new plan review if folded as written; my next head review checks the fold.
3. Register row in the loose-ends register's table "The critic's remaining items and the answered escalations", beside
   Q-A1 and in its form (not in "Decisions the drafts ask for"): the loose end (RR-1), "the advisor agent's ruling (D1,
   2026-11-13), never his decision", "his go for this row: PR #48's merge" (never worded as given), what Cargo.toml sets
   and its reach (every workspace crate; dependencies at the release default, decided at the first third-party crate in
   sim's dependency tree: a named override, which beats "*", or dropping "*", cost measured), PM-0002-53's test from a
   crate that depends on sim (release, native and wasm32), engine-architect rewords part 4 and PM-0002-53 at the next
   revision. No edit to ADR 0002's text.
4. Its own Decisions-needed entry in the report (OVERFLOW-ALL, drafted in the handback), with the "no" branch: Cargo.toml
   back to part 4's two blocks, comments narrowed, the row out, the gap an open High deferred on his word.
5. Split of D1: RR-1 fixed by 1. New finding OVERFLOW-XCRATE-CI (Medium, my label): no CI step can show the cross-crate
   reach at 2.2 (K1b/M7 cannot tell A from the old blocks); anchored to the first change adding a dependency on sim or
   2.3, whichever first; lands with PM-0002-53's test run from a crate depending on sim; a CI edit, asks Mustafa then.
   Replaces or rewrites OVERFLOW-WASM.
6. Stale lines in the same push: REPO_MAP.md:16, ROADMAP.md:116, STATUS_ARCHIVE OVERFLOW-WASM and OVERFLOW-PER-CRATE,
   the report's Outcome :19, E1, M5, OVERFLOW-WASM, OVERFLOW-PER-CRATE rows; STATUS name list if a name changes.
   OVERFLOW-PER-CRATE rewritten for third-party crates (the skeptic's "moot" corrected), not dropped.
7. Evidence before head approval: (a) M7 in CI (remove only the release line, red at K1b inside the cut, exact revert,
   green); (b) the local cross-crate run on the fixed head (script on the fixed head: room 5/5; -v flags after clean;
   bench + inheriting profile; wasm32 in Node); (c) M5 and step 2's control relabelled historical; (d) CI green
   (toolchain push+PR, gate, smoke), Cargo.lock unchanged, local gate PASS, E9; (e) round 2 on what changed:
   rust-reviewer, test-engineer, security-auditor (panic-vs-wrap for room/server own arithmetic), engine-architect
   (register row and entry); (f) the verifier on the final head.
Per question: 1 route A; 2 keep "*"; 3 ADR untouched, loose-end row beside Q-A1 with its own entry; 4 yes, this ruling
approves that one change on conditions; 5 condition 7; 6 no ESCALATE (inherent widening C4, release = debug C4,
reversible, no room/server code), the row goes to him as OVERFLOW-ALL with his #48 go.

## After the final consultation (corrections folded into the handback)
- C10 ruled on branch head aee92cc1fdd4bcb524e37ec840657f2a1a26ae5d: Cargo.toml, Cargo.lock, rust-toolchain.toml, the six
  crate manifests and toolchain.yml equal 4d78ad6's (0 diff lines); non-checkpoint changes since 4d78ad6 are only the
  four crates' doc comments and the report (1de98a9).
- Correction to C5: "its MIR carries no checks" is the skeptic's source reading (C9 there), not mine; I tested only
  concrete arithmetic in a non-member crate. Dependency GENERIC code instantiated in a workspace crate is untested.
  Wording for the row and comment: "dependencies keep the release default setting".
- Condition 6 made a check (line numbers moved with 1de98a9): every overflow mention in the PR's files is true on the
  fixed head. Known stale now: Cargo.toml's comment; REPO_MAP's Cargo.toml row; ROADMAP's 2.2 entry; STATUS_ARCHIVE's
  provenance list (OVERFLOW-WASM's PM-0002-53, OVERFLOW-PER-CRATE), OVERFLOW-WASM, WASM-ARTIFACT's "joins OVERFLOW-WASM",
  OVERFLOW-PER-CRATE; STATUS's name lists if names change; the report's Outcome (pins bullet), As built (K1b and M5),
  E1 (step-2 control), M5, the note under the evidence table, OVERFLOW-WASM, OVERFLOW-PER-CRATE; the plan's In one
  screen, Part 2 row, Part 7 E1. True as they stand: the crates' "passes only while overflow checks are on" comments,
  REPO_MAP's crates and toolchain.yml rows. Untouched: Mustafa's words (DECISIONS.md, the report's A10) and ADR 0002's
  own lines (part 4; Appendix A's R1-RR-OQ4 answer; R5-RUSTLINT-OQ4; PM-0002-53), which the row hands to the owner.
- Plan deleted before ready, so the report's "Plan and approval" and "What the build did differently" carry this ruling
  and the pin change.
- OVERFLOW-ALL sequence: answered before "merge #48"; (b) = a new head, my review, his go again; the route is my ruling,
  his "no" overrules it.
- At my head review I re-run the cross-crate script myself on the pushed head.
