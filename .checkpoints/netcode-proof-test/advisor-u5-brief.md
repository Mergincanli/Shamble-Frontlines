# Advisor's check of U5's brief, rev 1 (2026-11-21)

Verdict: REVISE (2 High, 5 Medium, 7 Low). Rev 2 comes back to me for sections 1.4 (Blocks 0, 3, 4, 7), 2, 3, 4, 5,
6 and 8 only. Rulings: Q1, Q2, Q3, Q5, Q6, Q7 mine (below, with terms); **Q4 is his** (U5-ALLOC, text below).

Seat: a day session's advisor (general agent acting on main's `.claude/agents/advisor.md`, read whole with `git show
origin/main:`), branch claude/netcode-proof-test-plan-hcky5g. Read only; this file is my one write; no cargo, check.mjs,
npt-lab or judge run; no commit.

## Plan
1. Read main's advisor.md (done), the brief and its notes (done), TEST-FIRST, SIZE-CHECK and KEEP-PROGRESSING (done).
2. Read the model: u4h-brief.md's form and advisor-u4h-brief.md (done).
3. Completeness against plan.md :1252, :588-596, :635-653; u4-build-plan.md:961-962; u4s-carries.md:759-764; the
   catalogue :245-248, :255; marks.json -34 and -35; SERIES.md's rows; a search of the rulings and reports (done).
4. U5-Q2's premise (done). 5. U5-Q3 against ADR 0002:269, Cargo.toml:77-101, check.mjs (done). 6. Block 0 against
   DEP-HOLD (done). 7. U5-Q4 against SERIES.md:162 and the decoders' code (done). 8. C-U6-19's line (done).
9. Section 6's data and U5-SIZE (done). 10. Anything his unsaid (done). 11. Verdict, rulings, texts, conditions (done).
STATUS: all blocks done (2026-11-21).

## Notes (one line per check, as I went)
- Every code anchor in 1.2 holds at the head: msg.rs:303 and :396; control.rs:601; ws_frame.rs:101 (`pub(crate)
  struct Scanner`), :116 `client`, :132 `server`, :170 `scan`, :607 the seeded test; room.rs:890; history.rs:59;
  crates/net/Cargo.toml:12-15; rust-toolchain.toml 1.99.1; deny.toml as described; no package.json under the spike.
- U5-Q2's premise holds: `wt-wtransport` and `wt-wtq` appear only in crates/net (its Cargo.toml:14-15 and its own cfgs
  and tests); the root's `npt-net = { path = "crates/net" }` (Cargo.toml:40) and the two users (bots/Cargo.toml:25,
  server/Cargo.toml:29, `workspace = true`) name no feature; npt-net has no `default` feature.
- ADR 0002's sentence is at :269 (docs/adr/0002-code-structure-build-toolchain.md); Cargo.toml:79-80 cites :264-265.
  The brief's drift line is right.
- npt-tcpinfo has four unsafe blocks (linux.rs:62, :91, :145, :171): plan :652's "four" holds.
- check.mjs's unsafe scan reads only `crates/` (crateFiles, check.mjs:736-745; unsafeProblems skips any path not
  `crates/<x>/`, :665-666): fuzz/ is unscanned until check.mjs changes (the brief plans it).
- S's letter: `git show --shortstat 411eeee -- spikes/netcode-proof` = 4 files, 701 insertions (U4b); u4f-f1-report.md:104
  = 239 added (205 + 34; its plan "about 60-150, size S, stop past 300", :103). U4s 1,979 (f080551) and U4d2 5,219
  (cc8595d against its first parent) also hold.
- Plan §12.1's own S rows are U0, U1, U3, U5, U8. U3 came to about 24,400 added lines of code (`git diff --shortstat
  04e40bc 78b0c07` over crates/, check.mjs, check.test.mjs, Cargo.toml: 24,404 insertions). His M figure came from
  pieces (U4c, U4j: history.md:2069-2070), not from U4 whole, so using pieces for S is the precedent; but "the two
  small units built so far" in the text is not true as written.
- **Input's decode reserves `Vec::with_capacity(count)`, count up to 8** (proto/src/input.rs:208-212); `Command` holds
  32 B (seam.rs:23-44: u8, u32, u16, u16, [u8; 4], [i8; 2], [i32; 3], u32, u8), so 6 commands ask for 192 B and 8 for
  256 B against Input's 166 B limit.
- **Snapshot's decode grows `Vec::new()` by push** (snapshot.rs:589-612). `EntityDelta` holds an `EntityRecord`
  (seam.rs:348-366: kind, three u16, two [_; 12] arrays of 4 B) of about 104 B, so a snapshot of 9 or more entity
  changes grows the vector to 16 slots, about 1,660 B, against the 1,150 B receive limit. FullState's
  `Vec<EntityRecord>` passes 65,536 B once it holds more than 512 records (1,024 slots, about 106 KB).
- C-U6-19's gate (advisor-u4f-f1-r1.md:120-122) is about kind 5's first-command gap: room/src/input.rs:287 checks
  edges only `if let Some(prev) = &self.accepted`, so a session's first command carries any edges.
- DEP-HOLD (main's CHECKPOINTS.md:60-69): any change to a Cargo.toml or Cargo.lock in any folder is held until
  dependency-check.md names that exact content; "Until then build nothing that holds the change".
- Mustafa's shared-target words (history.md:411-412): "from U3d on, point all copies at one shared Cargo target folder
  (CARGO_TARGET_DIR), so each unit doesn't leave a 4-6 GB cache behind"; u4-build-plan.md §7.1 items 1 and 6.
- A19 (u4-build-plan.md:102, §7.2 :999-1000): "each report lists the pub items it adds ...; engine-architect checks
  them in review".
- Items addressed to U5 outside the register: T2-L2 (u4d-round2-test.md:173-176, "one line in U5's carry 48 ...:
  decoding a 60 B header that claims 2^20 ticks allocates at most room x size_of::<TickInput>()"; u4d-report.md:63 lists
  it as going "with U5's carry 48"; the cap is sim/src/replay.rs:323-326); S2's differential (u3c-review-result.txt:396:
  "Make U5's parser fuzz target (-34) a differential test: DelayStream's head length must equal tungstenite's for every
  input"; U3c built a seeded differential in ws_frame.rs's tests, :517-572, so the fuzz half is open). Everything else I
  found (futures-util, tokio-rustls, npt-tcpinfo's blocks, libm and every U4 dependency, carry 48, C-U5-1, C-U6-19's
  line, catalogue items 3 and 6, SessionStatus's and RoomControl's targets, the scanner) is in the brief.

## Findings (CLAUDE.md's scale)
- **ADV-U5B-1 (High): U5-Q4's lean fails a correct build, and every workable bar is his.** Read as (a) reads it, "one
  allocation larger than the type's size limit", the decoders as built count over the limit on ordinary input: Input
  reserves up to 8 commands of 32 B (input.rs:212: 256 B against 166 B); Snapshot's entity vector reaches about
  1,660 B at 9 changes (snapshot.rs:589-612, against 1,150 B); FullState passes 65,536 B past 512 records. The seed
  corpus itself (valid encodings) would trip it, so -34's Input and Snapshot parts would FAIL on a correct decoder. The
  Snapshot and FullState targets through npt-client's receive path also allocate copies of the held world, sized by
  the scene, not the message. Any bar that lets a correct decoder pass reads "allocations above the size limits"
  (marks.json:7198, SERIES.md:162, ADR 0001:1626) as something other than the wire limit, and that reading decides the
  verdict: TEST-FIRST rule 2, "any change to a pass mark", as I ruled U4H-Q4. **Fix:** U5-Q4 becomes his (U5-ALLOC,
  text below, lean (a)); Block 3's check records per decode call the largest single allocation and the total, and
  counts over-limit allocations by the bar his answer picks; rev 2 computes option (a)'s bound per type from the code
  (each vector's largest reach for an input within the limit times its element's size, at Vec's growth; for the two
  client targets plus the fixed scene's held state; RoomControl at 1,024 B; the scanner 0, its state is fixed,
  ws_frame.rs:23), with the test-engineer's read, and puts the figures in the text; section 8's "Nothing else is his"
  is struck. Only -34's counted runs wait for his answer; the build does not.
- **ADV-U5B-2 (High): the fuzz builds leave his shared-target rule.** Section 5 keeps "the shared target and its lock
  for every stable build" and section 2 puts the fuzz builds in a folder "never the shared target". His words cover
  every copy (history.md:411-412), and cargo-fuzz's default folder is `fuzz/target` inside the builder's worktree: the
  cache he asked never to leave behind. As worded the brief narrows his rule to stable builds. The stated reason does
  not force it: cargo-fuzz builds with an explicit target triple, so its artifacts sit under
  `<target>/x86_64-unknown-linux-gnu/`, apart from the host builds, and the nightly's rustc gives its host build scripts
  their own hashes. **Fix (one line in sections 2 and 5):** every cargo call in a copy, the nightly fuzz builds
  included, runs with `CARGO_TARGET_DIR=.gate/shared-target` (or cargo-fuzz's `--target-dir` naming it), under the lane
  lock; the main session's counted builds go to the main folder's cache (§7.1 item 6) or to one named .gate folder
  emptied with `cargo clean --target-dir` at U5's close. If a build shows the shared folder cannot hold them, that comes
  to him, never decided in the brief.
- **ADV-U5B-3 (Medium): the fuzz crate's lockfile would resolve its own dependency versions, unaudited.**
  `cargo generate-lockfile` in fuzz/ resolves every transitive dependency of npt-net, npt-client and npt-proto afresh
  (quinn-proto, rustls, ring, tokio's own and the rest), newest compatible, so the fuzzed build can carry other
  versions than the workspace's Cargo.lock, and Block 8's `cargo deny` and `cargo audit` read the workspace lock only.
  The root lock's update for the new member is not stated either (a root `generate-lockfile` or `cargo update` would
  re-resolve everything). **Fix:** fuzz/Cargo.lock starts as a copy of the workspace lock and gains only libfuzzer-sys
  and its own dependencies by a minimal resolve; Block 0 and check.mjs check that every package both locks hold has the
  same version and checksum; the audits run on both locks; the root lock changes only by an offline resolve that adds
  the new path package and its edges.
- **ADV-U5B-4 (Medium): Block 0 does not hold every manifest change U5 makes.** Later blocks change manifests too:
  npt-net's `fuzz` feature (1.3; crates/net/Cargo.toml), npt-client's entry if it takes a feature, npt-alloc as a dev
  dependency in crates/sim, crates/room and crates/proto (section 3's per-type case), the root lock. Each is a DEP-HOLD
  change, and one built before the read breaks "build nothing that holds the change". **Fix:** Block 0 lists every
  manifest and lock U5 changes, by path, with its content; a manifest change found needed later stops the builder for
  a second read before anything that holds it is built.
- **ADV-U5B-5 (Medium): -35's swap check has no stated rule, and U6's likely swap would fail it.** At U6 the swap is
  likely a feature choice in a crate outside crates/net (npt-server's manifest), which a count of "files outside
  crates/net" reads as 1 (FAIL); ADR 0001:1627's "changes no code outside the transport module" then needs a reading,
  and a reading that changes what counts is his. **Fix (U5-Q2's terms below):** the tool counts every changed path
  outside crates/net, manifests and Cargo.lock included, with a fixture "only crates/server/Cargo.toml changed: FAIL";
  a carry to U6: U6 makes the swap a change inside crates/net (npt-net's own selection), and a swap that needs a line
  outside it comes to the advisor before -35's run (to him if it changes what counts).
- **ADV-U5B-6 (Medium): the C-U6-19 line is not true for C-U5-1 as worded.** C-U5-1's seeded test does hold a hostile
  client: its fuzzed `Input`s carry arbitrary press counters, so its commands press throw and fire, and its first
  command's edges go unchecked (room/src/input.rs:287). What makes it safe is that it is a unit test whose assertions
  read no presses. And "-34's runs ... with no client at all" misses Block 4, which drives an honest `ClientEndpoint`.
  **Fix:** the line as below.
- **ADV-U5B-7 (Medium): U5-SIZE's text is not accurate as written.** "the two small units built so far came to 239 and
  701 lines" leaves out U0, U1 and U3, which the plan's table also sizes S (U3 came to about 24,400 added lines of
  code). The text does not say that at U4f2's growth (about 1.35) the top of the range, about 4,200, passes option (a)'s
  4,000 stop, nor that option (b) moves the swap checker's build to U6's time (U5-Q2 then reads as (c)). "Fuzz
  targets", "allocator crate" and "record writers" are not plain words for him. **Fix:** the text below, figures from
  rev 2's recomputed section 6.
- **ADV-U5B-8 (Low): two items addressed to U5 are missing.** T2-L2 (a test with npt-alloc that `Recording::decode` of
  a short file claiming 2^20 ticks reserves at most room x `size_of::<TickInput>()`, replay.rs:323-326) and S2's
  differential (the scanner target asserts DelayStream's head length equals tungstenite's on every input; tungstenite
  is already in npt-net's tree). **Fix:** fold both into Blocks 2 and 4 (each about 20-50 lines), or state why not;
  register lines either way.
- **ADV-U5B-9 (Low): the runner's crash classes and the abort's place.** libFuzzer also ends a run on a timeout, an
  out-of-memory, a leak (LeakSanitizer is on under ASan) or a slow unit, each with its own file; Block 5 counts only
  panics and the over-limit message. **Fix:** classify every artifact kind; a run ended by anything else is reported by
  name and comes to me, never re-run into a pass; the leak setting is in the header; `inputs_count` is libFuzzer's final
  executed-units figure. The allocator only counts: the over-limit decision, its message and the abort happen in the
  harness after the decode returns, never inside `alloc`.
- **ADV-U5B-10 (Low): -35's tool details.** `cargo tree -p npt-sim` with all features and all targets, so a gated
  transport dependency shows; the transport list is check.mjs's `TRANSPORT` (check.mjs:41-53), one home, not a second
  fixed list; with two versions of one crate in the lock the header takes the lower (a fixture); wtransport's features
  read from an all-features resolve (as deny.toml's `[graph] all-features = true`), since at U5 no member enables it.
- **ADV-U5B-11 (Low): the -34 bite's form and the build rule.** "Unchecked index" here means a plain slice index with no
  length check before it (it panics); npt-proto stays under the forbid, no `get_unchecked`. The bite run's header names
  the bite, its log's `build.features` lists `bites` (README §4's build rule, marks/README.md:257-262).
- **ADV-U5B-12 (Low): deny.toml has never run.** It denies wildcards (deny.toml `[bans] wildcards = "deny"`) while every
  internal crate is a path dependency with no version; as I recall cargo-deny counts those as wildcards unless
  `allow-wildcard-paths = true` (every member has `publish = false`). Unverified (cargo-deny is not installed). **Fix:**
  Block 8 expects it; a deny.toml change is an audit-config change read by security-auditor in the round, named in the
  report.
- **ADV-U5B-13 (Low): the judge's read of a fuzz log is untested before the counted runs, and the panel lacks A19's
  seat.** **Fix:** section 3 gains a canned fuzz log set judged by npt-judge with marks.json's -34 (PASS on clean, FAIL
  on a panic and on an over-limit count, the bite shown), with README §1's header fields as the judge reads them; if the
  judge cannot read a fuzz log as written, that is a judge change and stops for him (section 5's line). Section 7 adds
  engine-architect for the public items (npt-alloc, npt-net's and npt-client's entries: A19), in the second batch.
- **ADV-U5B-14 (Low): section 6 moves with the findings.** check.mjs's fourth exception plus fuzz/ in the scan, its
  lints and the lock check look more like 150-400 than 100-250 (npt-hash-wasm's rules alone are about 100 lines of
  check.mjs); the audit runner, manifests, SERIES rows and README more like 100-250; the findings add about 160-370
  (the per-type bounds, the lock check, T2-L2, the differential, the crash classes, the strict-swap fixture, the judge
  set). My rounded whole: about 1,800 to 3,450 (up to about 4,650 at 1.35). These are judgements on the code's shape,
  not counts. **Fix:** section 6 recomputed item by item; the texts take its figures.

## Rulings on U5-Q1 to U5-Q7 (TEST-FIRST rule 2 for all but Q4; the main session records each with its reason and names
## it in the day's message that carries U5-SIZE)
- **U5-Q1, the three other targets: (a), with three terms.** Plan §4.7 (:590-595) and §4.8 (:653) make SessionStatus,
  RoomControl and the scanner fuzz targets; marks.json's -34 has no part for them. (1) Same settings and numbers as -34's
  parts (10,000,000 inputs, 0 panics, 0 over-limit by the bar his U5-ALLOC answer picks; the scanner's bar 0); a panic
  or an over-limit allocation stops U5's close like a failed test and comes to me. (2) Their logs name no mark
  (`marks: []`), so npt-judge never reads them into -34's checks; the runner prints their verdict beside -34's. (3)
  "After the close" gains, as his marks.json item: -34's notes name "plan §4.7's npt-proto messages", which include
  SessionStatus, while its parts do not. Why: inside the plan, no mark text or number changes.
- **U5-Q2, -35's counted run: (a), with three terms.** The premise holds (Notes). (1) The tool counts every changed path
  outside crates/net, manifests and Cargo.lock included, with ADV-U5B-5's fixture and ADV-U5B-10's details. (2) The one
  counted run follows U6's merge, "once, after the swap" (marks.json:7321; ADR 0001:1627), on the run's checks
  unchanged; plan §12.1's U5 and U6 rows gain one line each. (3) U6's register section gains the swap-form carry
  (ADV-U5B-5). If he picks U5-SIZE (b), this reads as (c): tool and run both at U6's time.
- **U5-Q3, the allocator's home: (a), with these terms.** A member crate npt-alloc, the fourth exception at deny
  (ADR 0002:269: a written invariant, a test, an `#[allow]` with its reason). (1) Its only unsafe is the `unsafe impl
  GlobalAlloc` over `System`, one `#[allow(unsafe_code, reason = "...")]` on it, each inner unsafe block one operation
  with a SAFETY comment under the two clippy denies npt-tcpinfo's lib.rs holds (:37-38); the invariant: it returns
  exactly System's pointers and forwards every dealloc and realloc with the same layout, and counting never allocates
  (const-initialised thread-locals without Drop). (2) No build script, no `include` (DENY_EXCEPTION_WHY's rules), std
  only. (3) check.mjs: `crates/alloc/` in UNSAFE_EXCEPTIONS and EXCEPTION_LEVELS (deny), a placed-allow list like
  TCPINFO_UNSAFE_FNS, the table now (allow, deny, deny, deny), Cargo.toml's comment updated with the :269 cite;
  check.test.mjs plants (an unsafe block outside the impl, a second allow, an allow with an empty reason) each failing
  by name. (4) At U5 only dev dependencies and the fuzz crate depend on it; a normal dependency (U6's PM-0006-20) is
  U6's change with its own read. (5) Each `#[global_allocator]` static (the sim and room test files, the fuzz targets)
  compiles under its crate's lints; if one does not, it comes to me. Why: (b) duplicates the unsafe code, (c) makes a
  large crate an exception.
- **U5-Q4, what counts as an over-limit allocation: his** (ADV-U5B-1; text below). The build makes the figures every
  option needs (per call the largest single allocation and the total; the count by the bar), so his answer picks only
  the bar and the SERIES.md:162 meaning cell that states it.
- **U5-Q5, the sanitizer: (a) if it fits, else (b), with two terms.** (1) The rate is measured at Block 3 on Input and
  on FullState through the client (the slowest), with and without ASan; the choice and both rates go in the fill form
  before the first counted run. (2) A target's bite run uses its counted runs' sanitizer setting; the header records
  it. Why: the decoders are safe code, so a bounds slip is already a panic; ASan's gain is the allocator's unsafe and
  libFuzzer's own C++.
- **U5-Q6, the bite's choice: yes, with three terms.** (1) `NPT_BITE` is read once at the target's start, only in the
  bites build (the fuzz crate's `bites` feature turning on npt-proto's); a measured build that sees it set refuses to
  start. (2) The runner sets it, or clears it, explicitly for every run. (3) The header records the bite id (`bite`),
  `build.features` with `bites`, and the binary's SHA-256 (plan §4.7 :631-632). The plan's `--bite` is the mechanism;
  the rule it serves (compiled only into the bites build, chosen at run time, recorded) holds.
- **U5-Q7, the fuzz crate under the checks: as written, with three terms.** (1) check.mjs reads fuzz/'s .rs files and
  Cargo.toml as it reads crates/ (the unsafe scan, #[path] and include!, no build script, no cargo-features or rustflags
  key), and checks that fuzz/'s `[lints]` equal the workspace's key for key (unsafe_code forbid, the three clippy
  denies). (2) fmt and clippy of the fuzz crate on stable 1.99.1, `--locked`, in the shared target (a cargo check of
  `fuzz_target!` needs no nightly), or the brief says why not. (3) The read of libfuzzer-sys's expansion at Block 0 is
  the check: rustc does not report most lints inside another crate's macro, so a forbid there can stay silent; any
  unsafe or `no_mangle` it holds is named in the read, and a fifth exception comes to me before Block 3 goes on.

## Item 1: completeness
- Against the plan and the register: every source the brief names is met in the blocks (plan :1252, :588-596,
  :635-653; u4-build-plan.md:961-962; u4s-carries.md:759-764; the catalogue's items 3 and 6; marks.json -34's 14 parts,
  three checks, report and bite; -35's checks, header params and two bites; SERIES.md :143, :160-163, :229, :244-256).
- Missing: T2-L2 and S2's differential (ADV-U5B-8); A19's seat (ADV-U5B-13).
- On the register, before U5's launch (the main session, u4s-carries.md's U5 section, the next free numbers, each
  quoting its source): the U5-Q1 to Q7 rulings with their terms; T2-L2 and S2 (if taken); in U6's section, -35's counted
  run after U6's merge and the swap-form carry (ADV-U5B-5). u4s-carries.md:764's cite (ADR 0002:264-265) becomes :269.

## Item 4: Block 0 against DEP-HOLD
- The order is right in shape (manifests, then the read, then build), and the tools' versions in the read go beyond
  DEP-HOLD's letter, which is good: `cargo install --locked` runs the tools' build scripts on this machine. Not enough
  as written: ADV-U5B-3 (the fuzz lock's versions, the root lock's update) and ADV-U5B-4 (every manifest in Block 0).
  `cargo generate-lockfile` runs no build script: true; it is the versions it picks that are the problem.

## Item 6: the C-U6-19 line, corrected (section 4; u4s-carries.md:760-761)
```text
C-U6-19 (u4s-carries.md:760-761): no U5 run holds a throwing or rejoining hostile client or a client restarted
mid-run. -34's runs feed bytes to decoders (Snapshot's and FullState's through an honest ClientEndpoint from a fixed
joined state, whose output no room receives); -35's tool reads manifests. C-U5-1's seeded test does hold a hostile
client: its fuzzed Inputs carry any press counters, so its commands press throw and fire, and its first command's
edges go unchecked (kind 5's first-command gap, room/src/input.rs:287). It is a unit test, not a run: one joined slot,
no rejoin, no restart; it asserts no panic, one counter per offered command and the bounds, none of which reads a
press count or what a throw spawns, so the gap cannot change its result.
```

## Item 7: section 6 and U5-SIZE
- The letter data hold (Notes). The pieces-not-units reading follows his own M figure's sources; the text must say it.
- The estimate is plausible item by item and its sums hold (1,630-3,085, "about 1,650 to 3,100"; 3,100 x 1.35 = 4,185);
  with the findings, about 1,800 to 3,450 (ADV-U5B-14).
- Rightly his: SIZE-CHECK (history.md:2048-2051, :2067-2075) makes a unit past its line stop and come to him, and the
  plan's units and stops are his (TEST-FIRST rule 2). Not a stop now: U5 builds after U4's close.

## Item 8: anything his, unsaid
- U5-Q4 (ADV-U5B-1): yes, now his. Nothing else: no money, nothing on main, no run day (every run in this container);
  U5-Q1 (a) adds runs that no mark reads; U5-Q2 (a) follows marks.json's "after the swap"; new SERIES.md rows only for
  the fuzz header params, and :162's meaning cell only by his U5-ALLOC answer.

## For Mustafa, paste-ready (figures in brackets from rev 2's computation; nothing else changes)
```text
U5-ALLOC: what counts as "an allocation above the size limits" in the decoder test PM-0001-34.
The test feeds each message type 10 million random and broken inputs and passes with 0 crashes and "0 allocations
above the size limits". The size limits are counted in bytes on the wire. Once decoded, a message takes more memory
than its wire bytes, so read word for word (any one memory request larger than the wire limit) a correct decoder
fails: an Input of 6 to 8 commands asks for 192 to 256 bytes against its 166-byte limit, and a Snapshot carrying 9 or
more changes asks for about 1,660 bytes against its 1,150. What the test is for is a decoder that believes a forged
length and asks for far more memory than any real message could need.
(a) Each type's bar is the most memory a message within its limit can make that code ask for at once, worked out from
    the code and the fixed test scene, written in the series file and printed in every run (Input [256] bytes;
    Snapshot [about 53,000], since one 1,150-byte snapshot can carry over 400 small changes; FullState [...]). A
    message over its limit must also be refused before any memory is asked for. No number chosen by us.
(b) A fixed multiple of each limit (for example 64 times): simpler, but a new number.
(c) Keep the words as they stand, and rebuild the decoded message types so that no single request passes the wire
    limit: a large change across the protocol, room and client code before the runs.
No other pass mark, number or run changes. Only PM-0001-34's counted runs wait for this; the build does not.
My lean, and the advisor's: (a).
```
```text
U5-SIZE: how big unit U5 may grow before its build stops.
U5 tests the message decoders against random and broken input, checks the dependencies and the transport swap, and
runs the security audits. The plan calls it small (S). Your size rule takes what earlier work of a letter came to: the
two pieces planned as small on their own came to 701 lines (the server's loop policy, U4b) and 239 (the press-edge
fix, U4f-F1), so a small unit is about 700 lines and is past its line at about 1,400. (U1 and U3 also carry S in the
plan's table, but each was built as several pieces with no letter of their own: U3 came to about 24,400 lines. Your M
figure of about 3,000 came from pieces too.) Built as the plan words it, U5 is about [1,800 to 3,450] added lines, past
that line before it starts: 17 fuzz programs (each feeds one message type millions of random and broken inputs), a
small library that counts memory requests, the record writers for those runs, a checker for the transport swap, the
changes to the build checks, a few tests and three planted bugs.
(a) Build U5 whole: its size line about [1,800 to 3,450], the build stopping past [4,000] at a block end. Units have
    grown about 1.35 times their estimate (U4f2), [about 4,650] at the top, so the stop could be reached; if it is,
    you get three lines then.
(b) Split it: U5 (the fuzzing, the memory counter, the tests and the audits: about [...]; stop past [...]), then U5b
    (the transport-swap checker and its two planted bugs: about [350 to 670]; stop past [1,300]), built when unit U6
    swaps the transport libraries.
No pass mark, judge rule, marks.json text or run changes under either.
My lean, and the advisor's: (a). One review panel and one merge; the swap checker is small, its run waits for U6
either way, and it belongs with the dependency checks.
```
- **Whose:** both his (U5-ALLOC a pass mark's meaning; U5-SIZE the plan's unit size and stop). **Which day:** the first
  day's message after rev 2 is checked; with the U4H texts if rev 2 is checked by then (TEST-FIRST: one message a day,
  any number of decisions). Neither is a stop: U5 builds after U4's close.
- The stops in the brackets are the main session's to propose in rev 2, by one stated rule (U4f2's ratio, about 1.3
  times the top, or SIZE-CHECK's twice the middle); the text says which.

## What rev 2 must change (REVISE: the exact changes)
1. ADV-U5B-1: U5-Q4 his (U5-ALLOC); Block 3's figures per call; option (a)'s bounds per type computed with the
   test-engineer's read; section 8's text and "Nothing else is his" corrected.
2. ADV-U5B-2: sections 2 and 5: every copy's cargo call, nightly included, in .gate/shared-target; the counted builds'
   folder named and emptied at U5's close.
3. ADV-U5B-3 and -4: Block 0 lists every manifest and lock by path; fuzz/Cargo.lock seeded from the workspace lock, the
   equal-versions check, the audits on both locks, the root lock by an offline minimal resolve; a later manifest change
   stops for a second read.
4. ADV-U5B-5: Block 7 and section 3: the strict count and its fixture; the U6 carry.
5. ADV-U5B-6: section 4's line as above.
6. ADV-U5B-7 and -14: section 6 recomputed; section 8's texts as above, figures filled.
7. ADV-U5B-8 to -13 in the same pass (T2-L2 and S2 folded or declined with a reason; crash classes and the abort's
   place; -35's details; the bite's form and build rule; deny.toml expected; the judge set; engine-architect in 7).
8. Rulings Q1-Q3 and Q5-Q7 folded with their terms, one "Ruled" line each pointing here; Q4 "his, text:
   advisor-u5-brief.md".
9. The register lines (item 1) are the main session's, before U5's launch.
10. Rev 2 comes back to me for sections 1.4 (Blocks 0, 3, 4, 7), 2, 3, 4, 5, 6 and 8 only.

## What I did not check
- I ran nothing. The allocation sizes are arithmetic on the types' fields (32 B for `Command`, about 104 B for
  `EntityDelta`), not `size_of` printed; Vec's growth (4, 8, 16, ...) is std's present behaviour, not a guarantee.
  cargo-deny's wildcard rule for path dependencies and rustc's external-macro lint rule are from memory. The disk cost
  of the fuzz builds in the shared target is unmeasured. U3's 24,404 lines count crates/, check.mjs, check.test.mjs
  and Cargo.toml between 04e40bc and 78b0c07, not U3's own merges. u4f-brief.md's sections beyond their cites.

## Decisions needed
- **U5-ALLOC** (his): what -34 counts as an over-limit allocation. Lean (a), each type's bar the most a message within
  its limit can make the code ask for. Text above.
- **U5-SIZE** (his): build U5 whole or split it. Lean (a). Text above.
- Both in the first day's message after rev 2's check; neither is a stop.

## For Mustafa
Nothing now from this check: the two questions ride a later day's message, after rev 2.
DONE

## Rev 2 check
### Plan
1. Read u5-brief.md rev 2 whole (385 lines) and u5-alloc-bounds.md (254 lines; summary :224-245, open points :247-254).
2. Check sections 1.4 (Blocks 0, 1, 3, 4, 7), 2, 3, 4, 5, 6 and 8 against my list (:289-305) and my rulings' terms
   (:157-203).
3. Check the coordinator's additions: type byte pinned, Ping/Pong both ways, realloc counted as one request, the window
   on the decode call only, item 3 on the decoder, the nightly re-measure, section 6's stop rule.
4. Spot-check the bounds' arithmetic against the code (Snapshot through the client 304 x L_e; FullState 14,561 doors).
5. Section 8's two texts: accuracy and plain words.
6. Verdict, findings ADV-U5B-15 on, conditions.

### Notes (as I go)
