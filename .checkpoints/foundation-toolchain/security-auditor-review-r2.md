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

## Focus 1, the answer (after the advisor consultation)
- Does route A open a DoS path that part 4's form avoided? Yes, narrowly. Room's own arithmetic and any server code on
  the tick thread now panic in release, where part 4's two blocks left them wrapping (my scratch run; the ruling's C4).
  Part 4's isolation separates each connection's work from the tick; it does not protect the tick from itself (ADR 0001
  :495-503, ADR 0004 :566-572: a tick panic is caught at the loop top and is fatal to the match). Connection-side server
  code: an overflow panic closes only that connection (ADR 0001 :768, PM-0001-26) = self-harm, with one caveat: state
  shared between connections (per-address limits, ADR 0001 part 11's I/O side) behind a std Mutex that handlers
  unwrap would spread a poison to every handler; ADR 0001's lock rule (:495-496) and PM-0001-26's bites name only a
  lock shared with the tick (reasoning from ADR 0001's own Mutex note; not run).
- What guards the tick: ADR 0004 part 20's first sentence ("No panic is reachable from network input or content
  data"), PM-0004-60 (fuzzed commands into the sim; room's control messages "once room exists"), PM-0001-34
  (decoders), PM-0001-58 (every 16-bit counter crosses its wrap in an L1 test), all in drafts still Proposed: rules
  and proposed marks, not code or CI.
- Part 4's form was never panic-free on the tick: sim's own (non-inlined) code already panicked in release under the
  two blocks, and sim runs on the tick. Test builds already panic in room under both forms (scratch run), so route A
  adds no panic a test-profile run cannot already see; what changes is an escaped bug's outcome in production.
- Is a silent wrap worse? As the default for an authoritative server, yes. A wrap is a silent integrity failure: under
  the old blocks a release build turned 100 - 150 credits into 4,294,967,246 and a u16 counter into 0 (a limiter
  that resets = bypass, fail-open); the wrong number is broadcast as authoritative and written into the match record at
  Ended. A panic is fail-stop: one match (one process per room), reported (ADR 0006), located (panic message), and the
  Wasm client and server no longer disagree. Counter-case, stated plainly: a benign wrap (a statistics counter, a
  cosmetic id) that a client can drive becomes a match-ender that client can repeat on every match it joins. The fix
  for both cases is the same (no client-reachable overflow), so fail-stop is the better default, provided the rule
  below lands with room's first code.
- The rule for when room (or a server crate) gets code: see SA2-2's fix.

## Findings (round 2)
- SA2-1 Medium, docs/reports/2026-11-12-foundation-2-2.md:213-215 (Risks) and :299-302 (OVERFLOW-ALL, "Why it
  matters"): both say an overflow in room's or the server's code "would likely end that match (inferred ...)" from part
  4's panic rules; by ADR 0001 part 11 (:493-503, :768-769, PM-0001-26, PM-0001-59) and ADR 0004 part 20 (:566-572),
  only a panic on the tick thread ends the match (answered by the Advisor, OQ-0001-18, in a draft still Proposed: not
  an inference), a panic in one connection's work closes only that connection, and a browser trap ends that player's
  sim instance (online, a fresh one resyncs). The Risks bullet also ends with an IOU, "Round 2's security review reads
  it against part 4 and R2-ROOM-REC2", which points at notes deleted before ready (round 1's D4 defect again).
  Fix: replace the IOU and the "likely ... inferred" clause in both places with the three outcomes above, the silent-
  wrap example (credits 100 - 150 = 4,294,967,246 in release under the old blocks; panic under route A), and the guard
  (ADR 0004 part 20's rule and its fuzz marks, written but not built), pointing to SA2-2's follow-up.
- SA2-2 Medium, docs/adr/README.md:234 (row OVERFLOW-ALL), STATUS.md "From foundation 2.2" list, STATUS_ARCHIVE
  :617-631 and :779-784: no record hands the panic side of route A to the owners or anchors it to room's first code.
  The row hands engine-architect only part 4's setting, its R1-RR-OQ4 answer and PM-0002-53 (the determinism side).
  The rules that must now cover room and the server are worded narrower: part 4's explicit-operations rule names
  "counters and tick numbers" for sim and protocol (ADR 0002 :247-249); ADR 0004 part 20 covers overflow only through
  its first sentence's plain words, while its named sources are unwrap, an unchecked index and std clamp "inside the
  sim" (:562-566); R5-RUSTLINT-REC3's no-implicit-panic lints reach protocol's decoders and content's loader only, and
  `arithmetic_side_effects` was never read (R5-RUSTLINT-OQ2, r5:1788-1789); R5-RUSTLINT-REC4's reason assumed "the
  rest of the workspace keeps the release default" (r5:1357-1358), which route A reverses. Exploit sketch (once room
  has code): a client sends valid commands that drive a room-side sum or counter written with bare `+`/`-` past its
  type (a u16 per-room id a client can advance; credits - price without a check): route A ends the match for 48
  players, repeatably; the old blocks wrap silently instead. Also the row's later option "dropping '*'" turns release
  overflow checks on in every dependency, not only sim's maths crate (without "*" a non-member gets
  `-C overflow-checks=on`: the ruling's S3, tested), and the row names only its cost.
  Fix: a named follow-up (for example OVERFLOW-PANIC-RULE) in STATUS and STATUS_ARCHIVE, and a clause in the row for
  the owners, anchored to the first change that gives room or a server crate code, or ADR 0002's next revision,
  whichever comes first: (1) part 4's explicit-operations rule reaches every workspace crate: `wrapping_*` only where
  wrap is the defined behaviour (tick offsets, sequence numbers: ADR 0001 :776), `checked_*`/`saturating_*` with a
  defined result (reject the command, clamp) for values a client or content can set or accumulate (credits, scores,
  counts, durations, ids, lengths), widths a match cannot exhaust; (2) ADR 0004 part 20 names integer overflow among
  its sources and room's and the server's tick-side code beside the sim; (3) state shared between connections never
  sits behind a lock that handlers unwrap after a poison; (4) PM-0004-60 and PM-0001-34 build with overflow checks on
  (route A gives it in every profile; pin it in the marks); (5) weigh clippy's `arithmetic_side_effects` (restriction)
  for room's command handling and the server's limiter code (R5-RUSTLINT-OQ2's candidate); (6) "dropping '*'" names
  its reach (every dependency, the server's network stack included) and gets the security review's read then.
- SA2-3 Low, docs/reports/2026-11-12-foundation-2-2.md:148 and :162-164: the Reviews table's D5 row still says "the
  cut asserts none of its properties", which the D5 skeptic corrected (the TCP probe asserts the network property;
  print-only was the approved design); STATUS_ARCHIVE :719 and the report's SETPRIV-HARDEN row carry the corrected
  wording. And the Reviews sentence says security-auditor "confirmed" the capability sets after setpriv; round 1
  confirmed them by a local run as root (util-linux 2.39.3) and, for the runner, by capabilities(7) only (CI's log
  prints no capability lines). Fix: D5 row "setpriv's flags; the cut asserts none of its privilege properties"; the
  sentence "... confirmed locally (a run as root, util-linux 2.39.3) and, for the runner, by capabilities(7) ...".

## Checked and found right (one line each)
- Focus 4: nothing under .github/ or .claude/, no CLAUDE.md, CLAUDE.local.md, AGENTS.md or gate file changed since
  4d78ad6; Cargo.lock unchanged; the head moved only in .checkpoints/ (b79424c).
- Cargo.toml :21-29 = the ruling's condition 1; 4bd9dd5 touches Cargo.toml only; no dependency added; no `panic` key in
  any profile, so the server keeps unwind and Wasm keeps the target's abort (part 4's panic strategy untouched).
- M7: 297d48a removes only the [profile.release] table; 8d622ac is its exact revert (empty diff).
- The row's mechanics match the Cargo Book: "*" = non-members only; a named override beats "*"; generic code is built
  where it is instantiated.
- Row OVERFLOW-ALL: "the advisor agent's ruling (D1, 2026-11-13), never his decision; his go for this row: PR #48's
  merge" (not worded as given); ADR 0002's own text untouched.
- Decision entry OVERFLOW-ALL: (a)/(b) as condition 4 asks; (b) = back to part 4's two blocks, comments narrowed, row
  out, an open High deferred on his word, a new head with the advisor's review and his go again; the question text
  names the silent wrong number; recommendation (a) is right from the security side.
- STATUS_ARCHIVE OVERFLOW-XCRATE-CI (:617-631) and OVERFLOW-PER-CRATE (:779-784): true to the ruling and E12;
  OVERFLOW-PER-CRATE rewritten for third-party crates, not dropped.
- REPO_MAP :16, ROADMAP's 2.2 entry, STATUS: overflow wording matches route A; his go on OVERFLOW-ALL recorded as
  waiting, not given.
- CUT-ESCAPE (report :197-207 and :244; STATUS_ARCHIVE :702-715) takes all four of D4's fixes: a class with examples,
  each route marked shown or not, a real boundary named, the full text in STATUS_ARCHIVE with a report row;
  lightningcss corrected ("loads only lazily"); "hermetic" appears nowhere in the PR's files.
- SETPRIV-HARDEN (report :245; STATUS_ARCHIVE :716-724) takes D5's corrections: no "relies on" claim; "asserts none of
  the privilege properties (only the TCP probe judges the network)"; the id -u check named as what closes the root
  case; the groups 65534 check added; "no device but lo is up"; at 2.3, asking him; it travels with the cut (CUT-CARRY).
- My round-1 SA-4 is in: For Mustafa item 3 names the two toolchain runs.
- The four crates' doc comments (1de98a9) make no security claim.

## Not verified
- M7's CI runs and their revert runs (43415093334, 43415101662, 43415458569, 43415465726; gate 43415458525,
  43415465755): not read by me.
- E12 and the ruling's C1-C8: not re-run; my scratch run covers room's own arithmetic only, native, cargo 1.99.1 here;
  no wasm32 and no sim call.
- Which profile and flags PM-0004-60's and PM-0001-34's fuzz builds use (cargo-fuzz defaults not read): "the fuzz sees an
  overflow as a panic" holds only for a build with overflow checks on.
- The poison spread through a lock shared between connections: reasoning from ADR 0001's own Mutex note, not run.
- `arithmetic_side_effects`: read at clippy master, not the 1.99.1 tag; its noise on our code not measured.
- CUT-ESCAPE states the runner's docker group outside the cut without a label; no log shows it (the D4 skeptic's
  note); immaterial, since GitHub documents passwordless sudo. The inheritable set and /proc reads are omitted from
  the wording: omissions, not errors. Cache reach sits under SETUP-NODE-CACHE, not CUT-ESCAPE: acceptable.

## Durable decisions (no security-auditor memory folder exists, and this review writes notes only)
- v2 Rust threat-model position: overflow checks are a fail-stop backstop, never the validator. Arithmetic on any value a
  client or content can set or accumulate uses checked_/saturating_ with a defined result, or wrapping_ where wrap is
  the defined behaviour. A silent wrap in authoritative state counts as an integrity (cheat) defect; a client-reachable
  panic on the tick counts as a DoS defect. Both block.
- Accepted risk until ADR 0004 part 20's fuzz marks are built: a client-reachable overflow on the tick ends one match
  (one process per room), and the crash is reported.

## Verdict
0 Critical, 0 High, 2 Medium (SA2-1, SA2-2), 1 Low (SA2-3). Nothing here calls for a round 3. SA2-1 and SA2-3 are docs
wording that the fix pass can take if it pushes anyway; SA2-2 becomes a named STATUS follow-up plus a clause in the row.
Each stays open until it is fixed or explicitly deferred.
Scratch leftovers (not deleted): /home/user/Shamble-Frontlines/.gate/security-auditor-r2/ (head/ with
crates/room/tests/overflow_r2.rs, target-a/, target-p4/, Cargo.toml.4d78ad6, Cargo.toml.routeA).

## Corrections after the second advisor consultation (these supersede the lines named)
- The "Working view before the advisor consultation" block is superseded: its headline ("opens no DoS path the ADRs
  leave unguarded") is replaced by focus 1's answer ("yes, narrowly"); its draft SA2-3 (dropping "*") is folded into
  SA2-2 as fix (6); its "(decided, not inferred)" in the ADR 0001 line now reads "answered by the Advisor (OQ-0001-18,
  2026-11-03) in a draft still Proposed" (ADR 0001 :3, ADR 0004 :3: Status Proposed).
- SA2-2 fix (4) now reads: PM-0004-60 and PM-0001-34 pin a build with overflow checks on. Route A reaches a fuzz
  build only if that build runs inside this workspace ("Cargo only looks at the profile settings in the Cargo.toml
  manifest at the root of the workspace. Profile settings defined in dependencies will be ignored.": Cargo Book,
  Profiles, re-read 2026-11-13). A fuzz crate with its own [workspace] takes its own profile.
- Attributions in focus 1's answer: sim's own code panicking in release under the old blocks is the ruling's C1 (the
  #[inline(never)] call); the release Wasm client and the server agreeing under route A is the report's E12 and the
  ruling's C7, not my runs. The match-record effect "would be", since no room code exists.
- Focus 4 re-run at HEAD 4f1a799303b54b48052bdf054bcdb96ea837be5c: 4bccf55..4f1a799 touches only .checkpoints/
  (b79424c, 64738ad, 4f1a799); since 4d78ad6 no path under .github/ or .claude/, no CLAUDE.md, CLAUDE.local.md,
  AGENTS.md or gate file, and Cargo.lock unchanged. `git status --short` shows only three notes files modified (mine and
  rust-reviewer's and test-engineer's round-2 notes); no tracked file outside .checkpoints/ changed.
- STATUS.md's "From foundation 2.2" follow-up list: :95-99.

## Final answer (as handed back)
Reviewed: 4d78ad6..4bccf55 outside .checkpoints/; the head is now 4f1a799, with only checkpoint commits since 4bccf55.
Worktree root for every relative path: /home/user/Shamble-Frontlines/.gate/foundation-toolchain-wt.

1. Verdict: 0 Critical, 0 High, 2 Medium (SA2-1, SA2-2), 1 Low (SA2-3). None calls for a round 3. All three stay open
   until fixed or explicitly deferred. An open security finding keeps the PR needs-mustafa at any severity (it is
   needs-mustafa anyway: tier 2, decision docs). Focus 1: route A does open a DoS path that part 4's form avoided, but
   only a narrow one, and a silent wrap is the worse default (the panic-versus-wrap answer, in the notes above).
2. Findings: SA2-1 (Medium), SA2-2 (Medium), SA2-3 (Low), as written under "Findings (round 2)", with SA2-2's fix (4)
   as corrected above.
3. Checked and found right: "Checked and found right" above. Anti-cheat coverage table: not applicable (no game,
   server or protocol code; room is an empty stub).
4. Not verified: "Not verified" above. Memory not written (no folder; notes only). Scratch leftovers named in the
   Verdict section.
