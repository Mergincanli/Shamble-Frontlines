# security-auditor, review round 2, foundation 2.2 (PR #48), round-1 head 4d78ad6 -> head 4bccf55

Scope: only what changed since 4d78ad6 outside .checkpoints/ (`git diff 4d78ad6..4bccf55 -- . ':(exclude).checkpoints'`).
Memory: no agent memory folder for security-auditor in this container (none under .claude/, ~/.claude/), as in round 1.
Writes: this file; scratch copies under /home/user/Shamble-Frontlines/.gate/security-auditor-r2/.

## Plan
1. Confirm the head and the scope: worktree HEAD = 4bccf55 (or later, .checkpoints/ only); list the diff's files;
   confirm nothing under .github/, .claude/, any CLAUDE.md changed since 4d78ad6; Cargo.lock unchanged (focus 4).
2. Read the diff whole: Cargo.toml (route A), the four crates' doc comments, the register row OVERFLOW-ALL
   (docs/adr/README.md), the report (docs/reports/2026-11-12-foundation-2-2.md: Risks, findings table, decision entry
   OVERFLOW-ALL), STATUS, STATUS_ARCHIVE ("From foundation 2.2"), REPO_MAP, ROADMAP; 297d48a/8d622ac exact revert.
3. Focus 1, panic versus wrap: read ADR 0002 part 4 (:247-278) and R2-ROOM-REC2 (r2-netcode.md:2762-2772), and whatever
   part 4 says on panic = "unwind", per-connection isolation, decoders that never panic. Weigh: (a) does route A open a
   client-triggerable DoS path in room/server code that part 4's form avoided; (b) does part 4's isolation already
   cover it (catch_unwind around per-connection work; a panic inside the room's tick?); (c) is a silent wrap worse
   (wrong authoritative state: credits, health, ammo, tick counters; security reading); (d) the rule for when room
   gets code. Sources: Rust reference (integer overflow), Cargo profiles docs (overflow-checks, package overrides,
   "*"), std::panic::catch_unwind docs (UnwindSafe, abort cases), RFC 560 (integer overflow), clippy
   arithmetic_side_effects lint. Optional scratch test: catch_unwind around an overflowing room-style function in
   release under route A (behaviour, not only reading).
4. Focus 2, honesty: the report's Risks, the register row OVERFLOW-ALL, the decision entry OVERFLOW-ALL (options and
   its "no" branch), STATUS_ARCHIVE OVERFLOW-XCRATE-CI and OVERFLOW-PER-CRATE: is the consequence (every workspace
   crate panics on overflow in release, room/server/devtools included; dependencies keep the release default) stated
   truly and completely, with no claim beyond the evidence?
5. Focus 3: CUT-ESCAPE and SETPRIV-HARDEN as worded (report Risks + findings table + STATUS_ARCHIVE) vs my round-1 SA-1
   and SA-2, skeptic-r1-D4.md, skeptic-r1-D5.md, review-r1-verdicts.json.
6. Write findings one line each as found; the verdict; what I checked; what I did not verify.

## Findings and checks (one line each, as done)
- [checked] head: worktree HEAD b79424c; 4bccf55..b79424c touches only .checkpoints/ (foundation-toolchain.md and this
  file, committed by the session's save); so the reviewed content is 4bccf55's.
- [checked] focus 4: `git diff --name-only 4d78ad6..HEAD` outside .checkpoints/ = Cargo.toml, four crates' lib.rs,
  REPO_MAP, ROADMAP, STATUS, STATUS_ARCHIVE, adr/README.md, the report (11 files); none under .github/ or .claude/, no
  CLAUDE.md / CLAUDE.local.md / AGENTS.md anywhere, no gate file; `git diff --stat 4d78ad6..HEAD -- Cargo.lock` empty.
- [checked] Cargo.toml at HEAD :21-29 = condition 1 exactly: `[profile.release] overflow-checks = true`,
  `[profile.release.package."*"] overflow-checks = false`, no other profile key, no [profile.dev]; 4bd9dd5 touches
  Cargo.toml only. M7: 297d48a removes only the [profile.release] table (3 lines), "*" kept; `git diff 297d48a^ 8d622ac`
  empty (exact revert).
- [read] ADR 0002 part 4 (:247-278): sim+protocol named; "counters and tick numbers use explicit wrapping_* or
  checked_*"; server keeps panic = "unwind", "each connection's work isolated from the room's tick and decoders that
  never panic"; Wasm -Cpanic=abort; Why: "one remote panic must not end a 48-player match".
- [read] R2-ROOM-REC2 (r2-netcode.md:2762-2782): fix pass M9: unwind, per-connection isolation, decoders never panic;
  spike mark 6: a panic in one connection's handler closes that connection only.
- [read] ADR 0001 part 11 (:493-503) + failure table (:768-769) + PM-0001-26/-59 + OQ-0001-18 (answered (a) by the
  Advisor 2026-11-03): connection panic -> that connection closes; tick-thread panic -> match ends, process exits
  non-zero (decided, not inferred). ADR 0001 :776, :1823, PM-0001-58: every 16-bit counter has a wrap test starting
  just below its wrap; sequence numbers "wrap, handled".
- [read] ADR 0004 part 20 (:562-588): "No panic is reachable from network input or content data"; its named sources
  are unwrap, unchecked index, std clamp, "inside the sim" (integer overflow not named); a tick panic is caught by
  catch_unwind at the loop top and treated as fatal. PM-0004-60: 0 panics over 10M fuzzed commands into a running sim;
  room's control-message sequences fuzzed "once room exists ... since a panic there ends the match too (part 20)".
- [read] R5-RUSTLINT-REC4 (r5:1353-1364): its reason says "so the rest of the workspace keeps the release default";
  R5-RUSTLINT-REC3 (:1340-1352): unwrap_used/expect_used/indexing_slicing at deny in protocol decoders and content's
  loader only; R5-RUSTLINT-OQ2 (:1788-1789): `arithmetic_side_effects` "not read ... candidates for REC3 and REC4".

## Working view before the advisor consultation (not yet findings)
- Focus 1: route A opens no DoS path the ADRs leave unguarded. Connection work: an overflow panic closes only that
  connection (ADR 0001 part 11, PM-0001-26) = self-harm only. Tick: part 4's own form already made sim's own code panic
  in release (sim runs in the tick); route A adds room's own arithmetic and tick-side server code; ADR 0004 part 20
  already forbids any panic reachable from network input, and PM-0004-60 fuzzes room's control messages "since a panic
  there ends the match too". Test/dev builds already panic on overflow in room (advisor C4), so route A adds no panic the
  test and fuzz marks cannot already see; it changes only an escaped bug's production outcome: fail-stop (one match,
  reported, crash record) instead of fail-silent (wrong authoritative number: credit underflow, wrapped rate-limit
  counter = limiter bypass, wrapped score written to the match record). Silent wrap is the worse default for an
  authoritative server. Rule needed when room/server get code: part 4's explicit-ops rule and part 20's no-panic rule
  must name overflow and reach room + the server (part 20 names unwrap/index/clamp "inside the sim" only).
- Draft SA2-1 (Medium?): report Risks :195-197 and the OVERFLOW-ALL entry say an overflow in room's or the server's
  code "would likely end that match (inferred)": per ADR 0001 part 11 only a tick-thread panic ends the match (decided,
  OQ-0001-18), a connection-handler panic closes only that connection; and the record omits the rule above.
- Draft SA2-2 (Medium?): no record carries the rule for room/server code (register row OVERFLOW-ALL hands only the
  setting and PM-0002-53 to the owner).
- Draft SA2-3 (Low?): the row's future option "dropping '*'" would also turn release overflow panics on in the
  server's network dependencies; not named.
- Focus 3 so far: CUT-ESCAPE and SETPRIV-HARDEN take D4's and D5's corrections (class of examples, each marked tested
  or not; groups check; root case closed by id -u; "only UP devices"). To check: the report's Reviews line "effective,
  permitted and ambient empty" (inheritable omitted), /proc reads omitted from CUT-ESCAPE.

## Sources read (round 2), one line each
- Rust Reference, "Behavior not considered unsafe", Integer overflow (doc.rust-lang.org, read 2026-11-13): "If a
  program contains arithmetic overflow, the programmer has made an error ... arithmetic overflow and wrapping
  arithmetic. The first is erroneous, while the second is intentional"; with debug assertions "implementations must
  insert dynamic checks that panic on overflow. Other kinds of builds may result in panics or silently wrapped values on
  overflow"; implicit wrap gives "well-defined (even if still considered erroneous) results". Official. Sure: high.
- Cargo Book, Profiles (doc.rust-lang.org/cargo, read 2026-11-13): overflow-checks "When overflow-checks are enabled, a
  panic will occur on overflow", dev true / release false; "To override the settings for all dependencies (but not any
  workspace member), use the "*" package name"; precedence, first match wins: named package, then "*" ("For any
  non-workspace member"), build-override, [profile.*], defaults; "Overrides and generics": "the code may be generated in
  the crate where it is instantiated, and thus may use the optimization settings of that crate". Official. Sure: high.
- clippy source, clippy_lints/src/operators/mod.rs at master (raw.githubusercontent.com, read 2026-11-13, as data):
  ARITHMETIC_SIDE_EFFECTS, category restriction, since 1.64.0, "any arithmetic expression that can cause side effects
  like overflows or panics"; "Checks any kind of arithmetic operation of any type"; ignores Wrapping/Saturating, floats,
  const contexts and operations that "won't overflow"; why: "it is desirable to explicitly call checked, wrapping or
  saturating arithmetic methods". Official source (master, not the 1.99.1 tag). Sure: high for existence and intent;
  its noise level on our code not measured.
- ADR 0001 :3 and ADR 0004 :3: "Status: Proposed" (OQ-0001-18 is answered by the Advisor inside a draft still
  Proposed); ADR 0002 :3 Proposed, part 1 accepted only.

## My own scratch run (round 2), room's OWN arithmetic
- [checked] /home/user/Shamble-Frontlines/.gate/security-auditor-r2/head (git archive of HEAD's Cargo.toml, Cargo.lock,
  rust-toolchain.toml, crates/; lock cmp-identical), rustc/cargo 1.99.1; added only crates/room/tests/overflow_r2.rs
  (room-style `credits - price` u32 and `count + 1` u16, #[inline(never)], black_box, catch_unwind; one spawned thread as
  a connection stand-in). `cargo test --locked -q [--release] -p room --test overflow_r2 -- --nocapture`:
  route A (HEAD): test profile and release: purchase 100-150 panicked, counter u16::MAX+1 panicked, the thread's panic
  stayed in its thread (join Err), main thread ran on.
  part 4's blocks (4d78ad6's Cargo.toml): test profile: the same panics; release: purchase = Ok(4294967246), counter =
  Ok(0), thread join Ok (no panic).
  So: (i) route A makes release do what test builds already do for room's own arithmetic (agrees with the ruling's C4,
  which used a sim call too); (ii) part 4's form, in release, turns a credit underflow into ~4.29 billion credits and a
  counter into 0 (a limiter that resets), silently. Scratch Cargo.toml restored to route A after the run.
