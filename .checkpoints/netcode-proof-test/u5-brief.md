# U5's brief, rev 2 (2026-11-21; rev 1 revised to the advisor's check, for its check of the changed parts)
**The unit:** plan.md §12.1's U5 row (:1252; size S): fuzzing (-34), the dependency and swap checks (-35), the audits
(§4.8), with the counting allocator and the allocation half of catalogue item 3 (u4-build-plan.md:961-962), C-U5-1
(plan.md:1252) and carry 48 (u4s-carries.md:762-764). It builds after U4's close (U4h's merge and sets, then U4's
report), in plan §12.1's order (section 1.1 says why not sooner). Short by TEST-FIRST rule 3 (history.md:2264-2267): what
U5 builds, how it is tested, what is open; the rest is pointed to. U4f's brief stays the reference for the start and
stops (u4f-brief.md section 1, :288-355), the build rules (section 4, :831-958), the end checks (section 6,
:1163-1183) and the return list (section 7, :1187-1225). **Two questions are his, neither a stop now** (section 8):
U5-ALLOC (what -34 counts as an allocation above the size limits) and U5-SIZE (U5 as worded is past what earlier S
units came to). The advisor ruled U5-Q1 to Q3 and Q5 to Q7 (section 8).

**Rev 1 to rev 2** (the advisor's check, REVISE, 2 High, 5 Medium, 7 Low: advisor-u5-brief.md, 322 lines; its list
:289-305):
- ADV-U5B-1 (High): U5-Q4 is his, U5-ALLOC; Block 3 records each decode's largest allocation and total and counts by
  the bar his answer picks; option (a)'s bar per type worked out from the code by the test-engineer, its sizes and
  capacities measured at rustc 1.99.1 (u5-alloc-bounds.md, 254 lines; summary :224-245): Blocks 1, 3 and 4, section 8. ADV-U5B-2 (High): every copy's cargo call, the nightly fuzz builds included, in the shared
  target: sections 2 and 5.
- ADV-U5B-3 and -4: Block 0 lists every manifest and lock by path; the fuzz lock seeded from the workspace lock, kept
  equal by a check, both locks audited; the root lock by an offline minimal resolve; a later manifest change stops for
  a second read. ADV-U5B-5: the swap count is strict, with its fixture and a carry to U6 (Block 7, sections 3, 4, 9).
  ADV-U5B-6: section 4's C-U6-19 line, the advisor's words. ADV-U5B-7 and -14: section 6 recomputed, section 8's texts.
- Lows -8 to -13: T2-L2 (Block 2) and S2's differential (Block 4) folded; every libFuzzer artifact class and the
  abort's place (Blocks 3 and 5); -35's tool details (Block 7); the bite's form and build rule (Block 6); deny.toml
  expected to need a change (Block 8); the judge set (section 3); engine-architect for A19 (section 7).

## 0. Where things are
- **`<base>`:** the spike branch's head at launch, after U4's close, filled by the main session in U4f's fill form
  (u4f-brief.md section 12, :1724); every count here is a delta on it. Every "there now" below was read at 93c90c2
  (the advisor re-read its anchors there: advisor-u5-brief.md:21-23). U4f2, U4g and U4h open crates/room, client, bots,
  lab and sim before U5, so those lines move and are re-read.
- **Read, never retyped** (stamps refilled at `<base>`): plan.md (§4.7 :588-596, §4.8 :635-653, §12.1's U5 row :1252),
  u4-build-plan.md:961-962, u4s-carries.md:759-764, marks/marks.json (PM-0001-34 :7054-7224, PM-0001-35 :7225-7322),
  marks/SERIES.md (:143, :160-163, :229, :244, :256), marks/README.md:257-262 (the bites build rule),
  docs/design/netcode-proof-messages.md (§2's size limits, §5's shared rules, §6 at U4 items 3 and 6, :245-248 and
  :255), spikes/netcode-proof/Cargo.toml:77-101 (the unsafe rule and its three exceptions), deny.toml, check.mjs:41-53
  (`TRANSPORT`), u0-probes.md:9 (the nightly), main's .claude/CHECKPOINTS.md:60-69 (DEP-HOLD), advisor-u5-brief.md (the
  check and its rulings), u5-alloc-bounds.md (the per-type bars).
- **One cite drift (Low), fixed in Block 0's manifest edit:** Cargo.toml:79-80 and u4s-carries.md:764 cite ADR
  0002:264-265 for "an exception needs a written invariant, a test and an `#[allow]` with its reason"; the sentence is
  now at ADR 0002:269.
- My working notes, with every source: u5-brief-notes.md.

## 1. What U5 builds
### 1.1 Its place and lane
- **After U4's close, one lane:** one builder (general-purpose, Opus 5.5, xhigh) in an isolated worktree. Not beside
  U4g or U4h: both open crates/lab, where U5's two writers go, and check.mjs may move with them; the nightly fuzz builds
  (proto, client, room, sim and net, instrumented) add to the shared target, which the disk floor of 4 GiB cannot carry
  beside a U4 lane's sets (U4f2's R2-pre and R3-pre wanted 6.5 GB free).
- **Its files:** a new `fuzz/` (cargo-fuzz's crate, outside the workspace); a new `crates/alloc` (U5-Q3); crates/lab
  (two subcommands); ops/runs (the fuzz, deps and audit runners); check.mjs and check.test.mjs; crates/proto (the -34
  bite); crates/net (the scanner's entry); crates/client (an entry, only if its receive path is not reachable); new
  test files in crates/proto/tests, crates/room/tests and crates/sim/tests; the manifests and locks of Block 0;
  SERIES.md (new header-param rows only); README's status line; results/u5-audits/.

### 1.2 What is there now (read at 93c90c2)
- **The decoders:** npt-proto `decode(way, channel, bytes)` (msg.rs:303), through `Quant::decode` (:396), which carries
  a bites build's planted defect; `RoomControl::decode(bytes)` (control.rs:601); `Limits` (limits.rs:12-51), one const
  per type (INPUT 166 to ROOM_CONTROL 1,024; NOTE 480; PEERS 16). The client's own decode of `Snapshot` and `FullState`
  groups against its baselines sits behind `ClientEndpoint` (client/src/lib.rs:295-682); its entry is the builder's to
  name at Block 4. DelayStream's request and frame scanner: `Scanner` in crates/net/src/l2/ws_frame.rs (pub(crate),
  :101; `client(max_payload, max_head)` :116, `server()` :132, `scan(&mut self, input)` :170), already with a seeded
  test (:607) and a seeded differential against tungstenite (:517-572).
- **What the decoders allocate** (u5-alloc-bounds.md, measured at rustc 1.99.1): Input reserves 8 x 32 B before
  reading a command (input.rs:212); SessionControl's note up to 480 B (control.rs:285); RoomControl's peer list 16 x
  17 B (control.rs:436); Snapshot's entity vector grows by push to 512 x 104 B (snapshot.rs:599-608) and the client
  copies its held frame twice and regrows one copy (client lib.rs:1065, :1074, :135/:145); FullState's grows to
  16,384 x 104 B (snapshot.rs:705-708) and the client collects it at 152 B a record (lib.rs:1125); the other types and
  the scanner allocate nothing. Every decoder refuses an over-limit input before any allocation (msg.rs:403, :411;
  control.rs:602-604). A decoded message takes more memory than its wire bytes, which is why U5-ALLOC is his.
- **The room and the sim:** `Room::receive(&mut self, slot: u8, msg: &Message)` (room.rs:890);
  `HitHistory::push(&mut self, volumes: &HitVolumes)` (history.rs:59); `Recording::decode` caps its reserve at what the
  bytes can hold (replay.rs:323-326).
- **Unsafe:** `unsafe_code = "forbid"` for the workspace (Cargo.toml:101); three exceptions with their own `[lints]`
  tables (npt-client-wasm allow; npt-tcpinfo and npt-hash-wasm deny, with `#[allow]`s on two and seven functions);
  check.mjs's deps step checks the table "(allow, deny, deny)"; its unsafe scan reads only `crates/` (check.mjs:665-666,
  :736-745), so fuzz/ is unscanned until Block 1.
- **The transport features:** npt-net's `wt-wtransport` and `wt-wtq` (crates/net/Cargo.toml:12-15), "swappable behind
  npt-link (PM-0001-35)". **No crate outside npt-net enables either** (the advisor re-read it: advisor-u5-brief.md:24-26):
  no binary picks a WebTransport crate before U6's WtServer (plan.md:1253).
- **The audits' inputs:** deny.toml (U1, never yet run: every advisory an error unless listed with a reason and a date;
  crates.io only; wildcards denied); no package.json under spikes/netcode-proof (web/ is U7's).
- **The tools:** toolchains stable and 1.99.1 only (rust-toolchain.toml pins 1.99.1); no cargo-fuzz, cargo-deny or
  cargo-audit installed; `nightly-2026-11-01` exists (u0-probes.md:9; "installed at U5 when fuzzing starts"); C and C++
  compilers present for libfuzzer-sys's build.

### 1.3 Public items, by crate (A19: each report lists them; u4-build-plan.md:102)
- **npt-alloc** (U5-Q3): the counting allocator type and its read-out (a per-thread window: start, then stop with the
  window's count, bytes and largest single allocation); names the builder's.
- **npt-net:** the scanner's entry for the fuzz target, under a `fuzz` feature, `#[doc(hidden)]`; none if the builder
  shows another way that leaves `Scanner` pub(crate).
- **npt-client:** an entry to its datagram and stream receive path, only if none is public.
- npt-lab's two subcommands are not pub items.

### 1.4 Blocks (each compiling with its tests passing before the next; at most about 400 added lines, a larger one split)
0. **Every manifest and lock, by path, then the DEP-HOLD stop.** The build notes give each file's whole new content
   (or its diff) and its sha256:
   - spikes/netcode-proof/Cargo.toml: `crates/alloc` in `members`, `npt-alloc` among the internal workspace
     dependencies, `exclude = ["fuzz"]`, the comment's cite at ADR 0002:269;
   - spikes/netcode-proof/Cargo.lock: changed only by an offline resolve that adds the new path package and its edges
     (no `cargo update`, no `generate-lockfile`); its diff read whole;
   - crates/alloc/Cargo.toml (std only; no build script, no `include`);
   - the dev-dependencies on npt-alloc: crates/sim/Cargo.toml (carry 48, T2-L2), crates/room/Cargo.toml (if C-U5-1
     needs it) and crates/proto/Cargo.toml (section 3's per-type case);
   - crates/net/Cargo.toml's `fuzz` feature (Block 4's entry) and crates/client/Cargo.toml (only if its entry takes a
     feature);
   - fuzz/Cargo.toml (its own `[workspace]`, its `[lints]` equal to the workspace's key for key, a `bites` feature
     turning on npt-proto's) and fuzz/Cargo.lock: **a copy of the workspace lock**, then a minimal resolve that adds only
     libfuzzer-sys and its own dependencies (never `generate-lockfile`), with the check that every package both locks
     hold has the same version and checksum;
   - the exact versions of cargo-fuzz, cargo-deny and cargo-audit, and the nightly's date.
   **Then the builder stops:** security-auditor reads exactly that content, libfuzzer-sys's `fuzz_target!` expansion
   included (U5-Q7 term 3), the main session writes dependency-check.md (CHECKPOINTS.md:64-67), and only then is
   anything built or installed ("build nothing that holds the change"). A manifest or lock change found needed later
   stops the builder for a second read before anything that holds it is built.
1. **npt-alloc and its checks** (U5-Q3's terms, advisor-u5-brief.md:171-183): its only unsafe is the `unsafe impl
   GlobalAlloc` over `System`, with one `#[allow(unsafe_code, reason = "...")]` on it; each inner unsafe block is one
   operation with a SAFETY comment, under the two clippy denies npt-tcpinfo's lib.rs holds (:37-38). The invariant: it
   returns exactly System's pointers and forwards every dealloc and realloc with the same layout, and counting never
   allocates (const-initialised thread-locals without Drop). The allocator only counts, and counts a `realloc` as one
   request of its new size (every push-grown vector reaches its largest size by realloc: u5-alloc-bounds.md:105-107).
   check.mjs: `crates/alloc/` in
   UNSAFE_EXCEPTIONS and EXCEPTION_LEVELS (deny), a placed-allow list like TCPINFO_UNSAFE_FNS, the table (allow, deny,
   deny, deny); fuzz/'s .rs files and Cargo.toml read as crates/ are, and its `[lints]` compared with the workspace's;
   the two locks' equal-versions check. check.test.mjs plants (an unsafe block outside the impl, a second allow, an
   allow with an empty reason, a fuzz/ lint that differs, a lock version that differs) each fail by name. Each
   `#[global_allocator]` static compiles under its crate's lints; if one does not, it comes to the advisor.
2. **Carry 48, T2-L2 and C-U5-1.** crates/sim/tests: a push into a full `HitHistory` allocates nothing (carry 48);
   `Recording::decode` of a short file whose header claims 2^20 ticks reserves at most room x
   `size_of::<TickInput>()` (T2-L2, u4d-round2-test.md:173-176; replay.rs:323-326). crates/room/tests: C-U5-1 word for
   word (plan.md:1252): seeded `Input` messages, valid and mutated, through `Room::receive` under options A and B, in the
   debug test build; no panic, exactly one counter per offered command, held at most 8, refusals at most 4,096 a tick,
   the kept window at most 16.
3. **The fuzz crate's core and 14 targets:** the shared check records, per decode call, the largest single allocation
   and the total bytes, and counts allocations above the bar his U5-ALLOC answer picks; an input longer than its limit
   must allocate nothing before its refusal (catalogue item 3's half). Option (a)'s bars (u5-alloc-bounds.md:224-245),
   one per type in a const table printed in each run's header: Input 256 B; SessionControl 480; RoomControl 272;
   Snapshot and FullState as Block 4; the other twelve targets 0. **Each target pins its message's type byte** (the
   harness writes the framing and the type byte, the fuzzer the rest), so it meets its own type's path only: with the
   byte free, a Join input would reach a 104 B note and a Ping 256 B or 832 B (u5-alloc-bounds.md:51-54, :121-136).
   Ping and Pong run on both receivers' ways, the room's and the client's. The counting window holds the decode call
   only: the harness's own copy of the input sits outside it. The over-limit decision, its message and the abort happen
   in the harness after the decode returns, never inside `alloc`. The bars were measured at rustc 1.99.1 (std's sort
   scratch computed); the counted builds use the nightly, so Block 3 re-measures them there from the harness's own
   records, and a bar that differs comes to the advisor before the first counted run. A seed generator writes valid
   encodings of each type as the starting corpus (corpora stay under .gate/, never committed). Targets for `Input`,
   `Resync`, `Ping`, `Pong`, `Join`, `JoinAccepted`, `JoinRejected`, `ClientSettings`, `RoomEnded`, `Leave`, `Kick`,
   `SessionControl`, `SessionStatus` and `RoomControl`, each through the decode its receiver runs, on its own direction
   and channel. The builder measures inputs a second on Input and on FullState through the client (the slowest), with
   and without ASan (U5-Q5).
4. **The three targets with state:** `Snapshot` and `FullState` through npt-client's receive path
   (`ClientEndpoint::input`, lib.rs:532-558, :783-831) from a fixed joined state: the stress scene's slot 0, its
   `JoinAccepted` and the room's `FullState` for it at the join tick (room.rs:1380-1398), built once at the harness's
   start, the largest held state any proof run gives a client (u5-alloc-bounds.md:211-222). Per input, a clone of that
   endpoint is made outside the counting window, then one `input` call inside it, so the group decode against baselines
   and the client's copies of its held world are fuzzed, not only the framing. **Their bars:** Snapshot through the
   client 304 x L_e bytes for a held frame of L_e >= 238 entity records (the first insert regrows a copied list to
   twice its length at 152 B a record), 155,952 at the stress scene's upper bound of 513 (lib.rs:135/:145); the harness
   prints L_e and computes the bar by u5-alloc-bounds.md's Block 15 rule (:184-187); the decoder alone 53,248.
   FullState through the client 2,213,272 (14,561 door records fit in 65,536 B; lib.rs:1125, its sort scratch at :1128
   the same size, computed); the decoder alone 1,703,936. After a decoder's refusal the client still allocates at most
   256 B for its bookkeeping (lib.rs:806-829), so catalogue item 3's half is checked on the decoder, not on the whole
   `input` call. DelayStream's scanner, client and server directions (bar 0: it never allocates), as a differential
   (S2, u3c-review-result.txt:396): on every input its head length equals tungstenite's (tungstenite is already in
   npt-net's tree).
5. **The record and the runner:** an npt-lab subcommand writing one log per run through npt-log (process `fuzz`, as
   SERIES.md:160-163 names the writer): the header (suite `fuzz`, the message, the nightly's date and rustc commit, the
   target binary's SHA-256, the seed, the sanitizer and the leak setting, the run count, the type's bar) and one
   `fuzz_end` event with the four counts, `inputs_count` being libFuzzer's final executed-units figure.
   ops/runs/c-u5-fuzz.mjs builds each target once, runs it with `-runs=10000000` and a recorded seed, reads libFuzzer's
   final stats, sorts every artifact by kind (a panic, the over-limit message, a timeout, an out-of-memory, a leak, a
   slow unit), counts the corpus and calls the writer. A run ended by anything but a panic or the over-limit abort is
   reported by name and comes to the advisor, never re-run into a pass. The three targets -34 does not judge write logs
   that name no mark (`marks: []`); the runner prints their verdict beside -34's (U5-Q1). Its own test on canned
   libFuzzer output.
6. **The -34 bite:** PM-0001-34/unchecked-index in npt-proto's bites build: one decoder's checked read replaced by a
   plain slice index with no length check before it (a panic), through `Quant`'s planted-defect path; npt-proto stays
   under the forbid, no `get_unchecked`. Chosen by U5-Q6's terms: `NPT_BITE`, read once at the target's start, only in
   the bites build (the fuzz crate's `bites` feature turning on npt-proto's); a measured build that sees it set refuses
   to start; the runner sets or clears it for every run; the header records `bite`, `build.features` with `bites` and
   the binary's SHA-256 (marks/README.md:257-262; plan §4.7 :631-632). Its run must end with panics at least 1, "found
   within the run", under the counted runs' sanitizer setting.
7. **-35's tool (U5-Q2's terms):** an npt-lab subcommand that counts **every changed path outside crates/net**,
   manifests and Cargo.lock included, from two refs; npt-sim's transport crates by `cargo tree -p npt-sim` with all
   features and all targets, against check.mjs's `TRANSPORT` (check.mjs:41-53, one home, never a second list); the four
   versions from `cargo metadata --locked --offline` (with two versions of one crate in the lock, the lower), and
   wtransport's features from an all-features resolve (as deny.toml's `[graph] all-features = true`); written as
   SERIES.md's rows (:143, :229, :244-256). Fixtures for each check, among them "only crates/server/Cargo.toml changed:
   FAIL" and two versions of one crate. The two bites (PM-0001-35/quinn-in-sim, PM-0001-35/quinn-proto-0.11.17) planted
   by script in a scratch copy, never committed.
8. **The audits and the end:** ops/runs/c-u5-audits on both locks (the workspace's and fuzz/'s): `cargo deny --locked
   check advisories bans sources`; `cargo audit` with its database date; `npm audit`: no lockfile at U5, stated in the
   record, U7's. deny.toml has never run: its `wildcards = "deny"` may refuse the path dependencies (unverified until
   cargo-deny runs); a deny.toml change is an audit-config change, read by security-auditor in the round and named in
   the report. Outputs under results/u5-audits/<date>/; then U4f's end checks.

## 2. Marks moved and the runs (the main session's, after the merge, in the main folder)
- **-34:** 14 counted runs, one per part, each `-runs=10000000` with one recorded seed, counted by the bar his
  U5-ALLOC answer picks; the three other targets (`SessionStatus`, `RoomControl`, the scanner, whose bar is 0) run the
  same way and are reported beside it (U5-Q1); the bite run must FAIL; npt-judge on the logs; raw logs under .gate/,
  verdicts and runs.txt under results/u5/<build>/pm-0001-34/. **Only these counted runs wait for his U5-ALLOC answer;
  the build does not.**
- **Time:** 17 runs of 10,000,000 inputs at Block 3's measured rates, one at a time; the sanitizer choice and both rates
  in the fill form before the first counted run (U5-Q5).
- **-35:** proven on fixtures and its bites at U5; its one counted run after U6's merge, once a binary picks a
  WebTransport crate (U5-Q2: marks.json:7321, "once, after the swap"; the run's checks unchanged).
- **The audits:** at U5's close (and at stop B and before every run: plan.md:644), on both locks; an advisory on a
  network-facing crate (plan.md:648-649, with tokio-rustls since U3c: :651) blocks the next run and goes to the
  advisor; a version bump is a dependency change, so DEP-HOLD's read comes first.
- **The shared target (his rule, history.md:411-412; u4-build-plan.md §7.1):** every cargo call in a copy, the nightly
  fuzz builds included, runs with `CARGO_TARGET_DIR=/home/user/Shamble-Frontlines/.gate/shared-target` (or cargo-fuzz's
  own target-dir option naming it), under the lane lock; the fuzz builds' explicit target triple keeps their artifacts
  apart from the host builds. The main session's counted fuzz builds go to the main folder's cache (§7.1 item 6), named
  explicitly since cargo-fuzz's default folder sits inside the crate, or to one named .gate folder emptied with `cargo
  clean --target-dir` at U5's close. If a build shows the shared folder cannot hold them, that comes to him, never
  decided in the brief. The runner refuses under 4 GiB free, as u4f-run.sh does.

## 3. How it is tested
- **npt-alloc and check.mjs:** a counted allocation, the largest single allocation, a realloc counted at its new size,
  zero inside allocation-free code, two threads' windows apart; plants (dealloc counted as an allocation; a realloc
  counted at its old size or not at all; the largest one off by one) each fail by name; check.test.mjs's plants as
  Block 1.
- **Carry 48, T2-L2 and C-U5-1:** as Block 2; plants (a push that grows its storage; a reserve that trusts the header's
  tick count; a counter skipped; each bound off by one) each fail by name.
- **The fuzz harness:** the over-limit count shown on a planted decoder that allocates by an unchecked length;
  catalogue item 3's half per type (a buffer one byte over the limit allocates nothing before its refusal); the
  scanner's differential on a planted head-length slip; the runner's artifact classes on canned output; the bite run
  FAILs.
- **The judge set:** canned fuzz logs (README §1's header fields as the judge reads them) judged by npt-judge with
  marks.json's -34: PASS on clean logs, FAIL on a panic, FAIL on an over-limit count, the bite run shown. If the judge
  cannot read a fuzz log as written, that is a judge change and stops for him (section 5).
- **-35's tool:** fixtures: a metadata file with quinn-proto 0.11.17 (FAIL), two versions of one crate (the lower
  taken), npt-sim's tree with quinn (FAIL), a swap diff touching crates/room (FAIL), only crates/server/Cargo.toml
  changed (FAIL), wtransport's dangerous-configuration feature on (FAIL), the clean case (PASS).
- **The two locks:** the equal-versions check fails by name on a planted version or checksum that differs.
- The evidence kinds of CLAUDE.md step 4: logic (tests on the real code), and every random draw seeded and recorded.

## 4. The carries, by number (met by number in U5's report)
- **U5's section of u4s-carries.md (:759-764):** C-U6-19's line (:760-761), the advisor's words
  (advisor-u5-brief.md:221-228):
  ```text
  C-U6-19 (u4s-carries.md:760-761): no U5 run holds a throwing or rejoining hostile client or a client restarted
  mid-run. -34's runs feed bytes to decoders (Snapshot's and FullState's through an honest ClientEndpoint from a fixed
  joined state, whose output no room receives); -35's tool reads manifests. C-U5-1's seeded test does hold a hostile
  client: its fuzzed Inputs carry any press counters, so its commands press throw and fire, and its first command's
  edges go unchecked (kind 5's first-command gap, room/src/input.rs:287). It is a unit test, not a run: one joined slot,
  no rejoin, no restart; it asserts no panic, one counter per offered command and the bounds, none of which reads a
  press count or what a throw spawns, so the gap cannot change its result.
  ```
  Carry 48 (:762-764): Block 2.
- **C-U5-1** (plan.md:1252; RR-U4F-4, advisor-u4f-r1.md:121): Block 2.
- **T2-L2** (u4d-round2-test.md:173-176; u4d-report.md:63, "with U5's carry 48") and **S2's differential**
  (u3c-review-result.txt:396): Blocks 2 and 4; register lines before launch (section 9).
- **Catalogue §6 at U4, item 3's allocation half** (the advisor's Q8 with nit (e), advisor-u4.md:166, :206): Block 3's
  check and section 3's per-type case. **Item 6** (the control-socket messages decode within 1,024 B and never panic;
  the advisor's Q7, :204-205): the `RoomControl` target.
- **§4.8's audit items:** tokio-rustls on the network-facing list (:651); npt-tcpinfo's four unsafe blocks
  (linux.rs:62, :91, :145, :171) with their invariants and tests (:652; R20, "U5 audits it", advisor-ws-docs-r2.md:410);
  futures-util (advisor-u3c-plan.md:87); "the audit of libm and every other U4 dependency" (u4-build-plan.md:962): one
  line per crate added since U3 in the audit record, read by security-auditor's seat.
- **For U6** (register lines before launch): -35's counted run after U6's merge; the swap's form: U6 makes the swap a
  change inside crates/net (npt-net's own selection), and a swap that needs a line outside it comes to the advisor
  before -35's run, to him if it changes what counts (ADV-U5B-5).

## 5. The rules that stand
- Start and stops: u4f-brief.md section 1 (:288-355), read for U5: the block-end size stop is the one his U5-SIZE
  answer sets; counts are deltas on `<base>`'s; the test step's stop is 1.5 times `<base>`'s measured test step
  (TE31-10, u4f-brief.md:326), written in the fill form.
- U4f's build rules (section 4, :831-958) with 1.1's lane; **his shared-target rule for every cargo call in a copy, the
  nightly fuzz builds included** (section 2); one builder in an isolated worktree; at most three agents at a time;
  never `cd` into its worktree.
- **No pass mark, judge rule, marks.json text or existing SERIES.md row changes, and no run is dropped.** New SERIES.md
  rows only for the fuzz runs' header params; SERIES.md:162's meaning cell only by his U5-ALLOC answer; a build that
  needs any other such change stops, and the main session asks him (history.md:2262).
- **DEP-HOLD:** Block 0's stop and its second-read rule; nothing pushed by hand; the tools installed only at the read's
  versions, `--locked`; the nightly by its date through `cargo +nightly-2026-11-01`, rust-toolchain.toml untouched; the
  fuzz crate's fmt and clippy on stable 1.99.1, `--locked`, in the shared target (U5-Q7 term 2).
- **Unsafe** only in npt-alloc's impl (U5-Q3) and the fuzz crate's dependency code as Block 0's read names it (U5-Q7).
- Every agent brief carries, word for word: "Write files with the Write and Edit tools only (no sed, no shell
  redirect, no python writes; never `cargo fmt` writing files: apply rustfmt's diff by Edit); an agent with no Edit
  tool rewrites its notes file whole with Write."

## 6. Size (SIZE-CHECK, history.md:2048-2051 and its reading :2067-2075)
- **What earlier work of the letter came to** (written on the card before U5's first snapshot): the pieces planned S
  on their own: U4b 701 added lines (411eeee, 4 files) and U4f-F1 239 (u4f-f1-report.md:104), so S's plan is about 700
  and "well past" is past about 1,400. U0, U1 and U3 also carry S in plan §12.1's table but were built as several pieces
  with no letter of their own (U3 came to about 24,400 added lines of code: advisor-u5-brief.md:35-38); his M figure of
  about 3,000 came from pieces too (U4c, U4j: history.md:2069-2070). Beside them, S-M: U4s 1,979 (f080551's merge) and
  U4d2 5,219 with its data files (cc8595d's merge).

| Item (added lines in spikes/) | Range |
|---|---|
| npt-alloc: the allocator, its invariants, its tests | 180-320 |
| fuzz/: Cargo.toml with 17 targets and its lints, the shared check with the bars' table, the seed generator, 14 plain targets, 3 with state | 520-860 |
| the fuzz record (npt-lab) and c-u5-fuzz.mjs with every artifact class, with their tests | 250-520 |
| -35's tool (npt-lab), its fixtures (the strict count, two versions), the bites' script | 320-600 |
| check.mjs and check.test.mjs: the fourth exception, fuzz/ in the scan and its lints, the two locks' check | 190-500 |
| C-U5-1's seeded test; carry 48's and T2-L2's tests | 210-430 |
| the -34 bite and its run; the scanner's entry with S2's differential; the client's entry | 100-250 |
| the judge set: canned fuzz logs judged with -34 | 40-100 |
| SERIES.md rows, the audit runner on both locks, manifests, README | 100-250 |
| **Whole** | **about 1,900 to 3,850** |

- Beside it, not code: the fuzz crate's lock (counted apart at each snapshot) and the audit records.
- Midpoint about 2,900; U4f2 grew about 1.35 (history.md:7350-7351): up to about 5,200. **Past S's line before it
  starts**, so the size line is his (U5-SIZE), as U4F2-SIZE was. The stops below follow one stated rule, SIZE-CHECK's
  own "well past" (more than twice the plan), with the plan taken as the estimate's middle: whole, past about 5,750;
  split, U5 past about 4,800 (middle about 2,400) and U5b past about 950 (middle about 475). These are judgements on the
  code's shape, not counts.

## 7. Reviewers and rounds
security-auditor (the fuzzing, the dependencies, both locks and the tools, the audits, the unsafe exception),
test-engineer (the bites, the counts, the record, the judge set) and rust-reviewer (the allocator, the nightly and stable
split, Wasm untouched) in the first batch; netcode-architect (each decoder's coverage, the client's paths, C-U5-1's room
bounds) and engine-architect (the public items: npt-alloc, npt-net's and npt-client's entries, A19) in the second; each
at xhigh, at most three agents at a time; at most two rounds, a third only for a Critical or High.

## 8. Questions
**Ruled by the advisor** (TEST-FIRST rule 2; each recorded with its reason under "Choices taken without him" and named
in the day's message that carries U5-SIZE; the terms are folded above):
1. **U5-Q1 (-34's three other targets): Ruled (a), with three terms** (advisor-u5-brief.md:159-165): -34's settings and
   numbers, the scanner's bar 0, a panic or over-limit stopping U5's close; their logs name no mark; "After the close"
   gains his marks.json item (-34's notes name plan §4.7's messages, which include SessionStatus; its parts do not).
2. **U5-Q2 (-35's counted run): Ruled (a), with three terms** (:166-170): the strict count; the run after U6's merge,
   plan §12.1's U5 and U6 rows gaining one line each; U6's swap-form carry. Under U5-SIZE (b) it reads as (c).
3. **U5-Q3 (the allocator's home): Ruled (a), with five terms** (:171-183): Block 1.
4. **U5-Q4 (what counts as over the limit): his** (ADV-U5B-1, :184-186): U5-ALLOC below.
5. **U5-Q5 (the sanitizer): Ruled (a) if it fits, else (b), with two terms** (:187-191): section 2, Block 6.
6. **U5-Q6 (the bite's choice): Ruled yes, with three terms** (:192-196): Block 6.
7. **U5-Q7 (the fuzz crate under the checks): Ruled as written, with three terms** (:197-203): Blocks 0 and 1,
   section 5.

**His (two; neither a stop; for the first day's message after this rev is checked):**
```text
U5-ALLOC: what counts as "an allocation above the size limits" in the decoder test PM-0001-34.
The test feeds each message type 10 million random and broken inputs and passes with 0 crashes and "0 allocations
above the size limits". The size limits are counted in bytes on the wire. Once decoded, a message takes more memory
than its wire bytes, so read word for word (any one memory request larger than the wire limit) a correct decoder
fails: an Input of 6 to 8 commands asks for 192 to 256 bytes against its 166-byte limit, and a Snapshot carrying 9 or
more changes asks for about 1,660 bytes against its 1,150. What the test is for is a decoder that believes a forged
length and asks for far more memory than any real message could need.
(a) Each type's bar is the most memory a message within its limit can make that code ask for at once, worked out from
    the code and the test scene (the stress scene) and measured where it could be, written in the series file and
    printed in every run: Input 256 bytes; SessionControl 480; RoomControl 272; Snapshot 155,952 (the client copies
    the world it holds, so the scene sets this; the decoder alone 53,248); FullState 2,213,272 (one 65,536-byte full
    state can carry 14,561 small records; the decoder alone 1,703,936); every other type 0. A message over its limit
    must also be refused before any memory is asked for, which every decoder already does. No number chosen by us.
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
figure of about 3,000 came from pieces too.) Built as the plan words it, U5 is about 1,900 to 3,850 added lines, past
that line before it starts: 17 fuzz programs (each feeds one message type millions of random and broken inputs), a
small library that counts memory requests, the record writers for those runs, a checker for the transport swap, the
changes to the build checks, a few tests and three planted bugs.
(a) Build U5 whole: its size line about 1,900 to 3,850, the build stopping past about 5,750 at a block end (your
    rule's "more than twice the plan", the plan taken as the middle, about 2,900). Units have grown about 1.35 times
    their estimate (U4f2), about 5,200 at the top, so the stop is near but under that.
(b) Split it: U5 (the fuzzing, the memory counter, the tests and the audits: about 1,600 to 3,250; stop past about
    4,800), then U5b (the transport-swap checker and its two planted bugs: about 330 to 620; stop past about 950),
    built when unit U6 swaps the transport libraries.
No pass mark, judge rule, marks.json text or run changes under either.
My lean, and the advisor's: (a). One review panel and one merge; the swap checker is small, its run waits for U6
either way, and it belongs with the dependency checks.
```
- Nothing else is his: no money, nothing on main, no run day (every run is in this container).

## 9. For the main session before launch
- Fill `<base>` and the counts; re-read every "there now" line in crates/room, client, lab and sim, and the per-type
  bars (u5-alloc-bounds.md) if U4g or U4h changed a decoder.
- The register (u4s-carries.md, the next free numbers, each quoting its source): U5's section gains U5-Q1 to Q3 and Q5
  to Q7 with their terms, T2-L2 and S2; U6's section gains -35's counted run after U6's merge and the swap-form carry;
  :764's cite becomes ADR 0002:269.
- plan.md: §12.1's U5 and U6 rows under U5-Q2 (a).
- The card: S's letter record (section 6) before U5's first snapshot; U5-ALLOC's and U5-SIZE's answers; "After the
  close" gains U5-Q1 term 3's marks.json item (his).
- The disk: free space before Block 3 and before the counted runs (the part-clean between waves, P27, if needed).
