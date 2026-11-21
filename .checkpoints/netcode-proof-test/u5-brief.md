# U5's brief, rev 1 (2026-11-21; drafted while F-B-COUNT waits, KEEP-PROGRESSING, for the advisor's check)
**The unit:** plan.md §12.1's U5 row (:1252; size S): fuzzing (-34), the dependency and swap checks (-35), the audits
(§4.8), with the counting allocator and the allocation half of catalogue item 3 (u4-build-plan.md:961-962), C-U5-1
(plan.md:1252) and carry 48 (u4s-carries.md:762-764). It builds after U4's close (U4h's merge and sets, then U4's
report), in plan §12.1's order (section 1.1 says why not sooner). Short by TEST-FIRST rule 3 (history.md:2264-2267): what
U5 builds, how it is tested, what is open; the rest is pointed to. U4f's brief stays the reference for the start and
stops (u4f-brief.md section 1, :288-355), the build rules (section 4, :831-958), the end checks (section 6,
:1163-1183) and the return list (section 7, :1187-1225). **One question is his, not a stop now** (section 8): U5-SIZE,
because U5 as the plan words it is past what earlier S units came to before it starts. Seven are for the advisor
(U5-Q1 to Q7), each with my lean.

## 0. Where things are
- **`<base>`:** the spike branch's head at launch, after U4's close, filled by the main session in U4f's fill form
  (u4f-brief.md section 12, :1724); every count here is a delta on it. Every "there now" below was read at 93c90c2.
  U4f2, U4g and U4h open crates/room, client, bots, lab and sim before U5, so those lines move and are re-read.
- **Read, never retyped** (stamps refilled at `<base>`): plan.md (§4.7 :588-596, §4.8 :635-653, §12.1's U5 row :1252),
  u4-build-plan.md:961-962, u4s-carries.md:759-764, marks/marks.json (PM-0001-34 :7054-7224, PM-0001-35 :7225-7322),
  marks/SERIES.md (:143, :160-163, :229, :244, :256), docs/design/netcode-proof-messages.md (§2's size limits, §5's
  shared rules, §6 at U4 items 3 and 6, :245-248 and :255), spikes/netcode-proof/Cargo.toml:77-101 (the unsafe rule and
  its three exceptions), deny.toml, u0-probes.md:9 (the nightly), main's .claude/CHECKPOINTS.md:60-69 (DEP-HOLD).
- **One cite drift (Low), fixed in Block 0's manifest edit:** Cargo.toml:79-80 and u4s-carries.md:764 cite ADR
  0002:264-265 for "an exception needs a written invariant, a test and an `#[allow]` with its reason"; the sentence is
  now at ADR 0002:269.
- My working notes, with every source: u5-brief-notes.md.

## 1. What U5 builds
### 1.1 Its place and lane
- **After U4's close, one lane:** one builder (general-purpose, Opus 5.5, xhigh) in an isolated worktree. Not beside
  U4g or U4h: both open crates/lab, where U5's two writers go, and check.mjs may move with them; cargo-fuzz builds into a
  target folder of its own (a nightly, instrumented build of proto, client, room, sim and net), which the disk floor of
  4 GiB cannot carry beside a U4 lane's sets (U4f2's R2-pre and R3-pre wanted 6.5 GB free).
- **Its files:** a new `fuzz/` (cargo-fuzz's crate, outside the workspace); a new `crates/alloc` (by U5-Q3 (a));
  crates/lab (two subcommands); ops/runs (the fuzz, deps and audit runners); check.mjs and check.test.mjs; crates/proto
  (the -34 bite); crates/net (the scanner's entry); crates/client (an entry, only if its receive path is not reachable);
  new test files in crates/room/tests and crates/sim/tests; Cargo.toml (members, the exclude, the cite); SERIES.md (new
  header-param rows only); README's status line; results/u5-audits/.

### 1.2 What is there now (read at 93c90c2)
- **The decoders:** npt-proto `decode(way, channel, bytes)` (msg.rs:303), through `Quant::decode` (:396), which carries
  a bites build's planted defect; `RoomControl::decode(bytes)` (control.rs:601); `Limits` (limits.rs:12-51), one const
  per type (INPUT 166 to ROOM_CONTROL 1,024; NOTE 480; PEERS 16). The client's own decode of `Snapshot` and `FullState`
  groups against its baselines sits behind `ClientEndpoint` (client/src/lib.rs:295-682); its entry is the builder's to
  name at Block 4. DelayStream's request and frame scanner: `Scanner` in crates/net/src/l2/ws_frame.rs (pub(crate);
  `client(max_payload, max_head)`, `server()`, `scan(&mut self, input)`), already with a seeded test
  (`random_input_never_panics_and_stays_within_its_bounds`, :607).
- **The room and the sim:** `Room::receive(&mut self, slot: u8, msg: &Message)` (room.rs:890);
  `HitHistory::push(&mut self, volumes: &HitVolumes)` (history.rs:59).
- **Unsafe:** `unsafe_code = "forbid"` for the workspace (Cargo.toml:101); three exceptions with their own `[lints]`
  tables (npt-client-wasm allow; npt-tcpinfo and npt-hash-wasm deny, with `#[allow]`s on two and seven functions);
  check.mjs's deps step checks the table "(allow, deny, deny)" and scans the crates (Cargo.toml:77-100).
- **The transport features:** npt-net's `wt-wtransport` and `wt-wtq` (crates/net/Cargo.toml:12-15), "swappable behind
  npt-link (PM-0001-35)". **No crate outside npt-net enables either** (every Cargo.toml and .rs read): no binary picks a
  WebTransport crate before U6's WtServer (plan.md:1253).
- **The audits' inputs:** deny.toml (U1: every advisory an error unless listed with a reason and a date; crates.io
  only); no package.json under spikes/netcode-proof (web/ is U7's).
- **The tools:** toolchains stable and 1.99.1 only (rust-toolchain.toml pins 1.99.1); no cargo-fuzz, cargo-deny or
  cargo-audit installed; `nightly-2026-11-01` exists (u0-probes.md:9; "installed at U5 when fuzzing starts"); C and C++
  compilers present for libfuzzer-sys's build.

### 1.3 Public items, by crate (A19: each report lists them; u4-build-plan.md:102)
- **npt-alloc** (U5-Q3 (a)): the counting allocator type and its read-out (a per-thread window: start, then stop with
  the window's count, bytes, largest single allocation and the count above a threshold); names the builder's.
- **npt-net:** the scanner's entry for the fuzz target, under a `fuzz` feature, `#[doc(hidden)]`; none if the builder
  shows another way that leaves `Scanner` pub(crate).
- **npt-client:** an entry to its datagram and stream receive path, only if none is public.
- npt-lab's two subcommands are not pub items.

### 1.4 Blocks (each compiling with its tests passing before the next; at most about 400 added lines, a larger one split)
0. **Manifests only, then the DEP-HOLD stop.** The fuzz crate's Cargo.toml (outside the workspace: the root's
   `exclude`) and its Cargo.lock (`cargo generate-lockfile`: resolution only, no build script runs); crates/alloc's
   Cargo.toml and its member line; the dev-dependencies the tests need; the cite fix. The build notes list every new
   package with its exact version, and the exact versions of cargo-fuzz, cargo-deny and cargo-audit and the nightly's
   date. **Then the builder stops:** security-auditor reads exactly that content (the tools included), the main session
   writes dependency-check.md (CHECKPOINTS.md:64-67), and only then is anything built or installed ("build nothing that
   holds the change").
1. **npt-alloc:** the allocator over `System`, its SAFETY invariant written beside each unsafe item, one `#[allow]`
   with its reason on each; counters that never allocate (const-initialised thread-locals, no lazy init inside
   `alloc`); its tests. check.mjs's fourth exception (the table becomes allow, deny, deny, deny) with check.test.mjs
   cases.
2. **Carry 48 and C-U5-1.** crates/sim/tests: a push into a full `HitHistory` allocates nothing (npt-alloc's window).
   crates/room/tests: C-U5-1 word for word (plan.md:1252): seeded `Input` messages, valid and mutated, through
   `Room::receive` under options A and B, in the debug test build; no panic, exactly one counter per offered command,
   held at most 8, refusals at most 4,096 a tick, the kept window at most 16.
3. **The fuzz crate's core and 14 targets:** the shared check (an allocation window around each decode; U5-Q4's rule;
   an over-limit allocation aborts with its own message, so the runner can tell it from a panic); a seed generator that
   writes valid encodings of each type as the starting corpus (corpora stay under .gate/, never committed); targets
   for `Input`, `Resync`, `Ping`, `Pong`, `Join`, `JoinAccepted`, `JoinRejected`, `ClientSettings`, `RoomEnded`,
   `Leave`, `Kick`, `SessionControl`, `SessionStatus` and `RoomControl`, each through the decode its receiver runs, on
   its own direction and channel. The builder measures one target's inputs a second, with and without ASan (U5-Q5).
4. **The three targets with state:** `Snapshot` and `FullState` through npt-client's receive path from a fixed joined
   state (a `JoinAccepted` and a `FullState` of a fixed scene), so the group decode against baselines is fuzzed, not only
   the framing; DelayStream's scanner, client and server directions.
5. **The record and the runner:** an npt-lab subcommand writing one log per run through npt-log (process `fuzz`, as
   SERIES.md:160-163 names the writer): the header (suite `fuzz`, the message, the nightly's date and rustc commit, the
   target binary's SHA-256, the seed, the sanitizer, the run count) and one `fuzz_end` event with the four counts;
   ops/runs/c-u5-fuzz.mjs builds each target once, runs it with `-runs=10000000` and a recorded seed, reads libFuzzer's
   final stats, counts crash files by cause (a panic, or the over-limit message), counts the corpus, and calls the
   writer. Its own test on canned libFuzzer output.
6. **The -34 bite:** PM-0001-34/unchecked-index in npt-proto's bites build (one decoder's checked read replaced by an
   unchecked index, through `Quant`'s planted-defect path), chosen at run time by U5-Q6's form; its run must end with
   panics at least 1, "found within the run".
7. **-35's tool (U5-Q2):** an npt-lab subcommand: the swap diff's files outside crates/net (from two refs), npt-sim's
   transport crates (`cargo tree -p npt-sim`, normal and build edges, against a fixed list), the four versions and
   wtransport's features from `cargo metadata --locked --offline`, written as SERIES.md's rows (:143, :229, :244-256);
   fixtures for each check, and the two bites (PM-0001-35/quinn-in-sim, PM-0001-35/quinn-proto-0.11.17) planted by
   script in a scratch copy, never committed.
8. **The audits and the end:** ops/runs/c-u5-audits (`cargo deny --locked check advisories bans sources`; `cargo audit`
   with its database date; `npm audit`: no lockfile at U5, stated in the record, U7's); outputs under
   results/u5-audits/<date>/; then U4f's end checks.

## 2. Marks moved and the runs (the main session's, after the merge, in the main folder)
- **-34:** 14 counted runs, one per part, each `-runs=10000000` with one recorded seed; the three other targets
  (`SessionStatus`, `RoomControl`, the scanner) run the same way and are reported beside it (U5-Q1); the bite run must
  FAIL; npt-judge on the logs; raw logs under .gate/, verdicts and runs.txt under results/u5/<build>/pm-0001-34/.
- **Time:** 17 runs of 10,000,000 inputs at Block 3's measured rate, one at a time; written in the fill form at launch.
- **-35:** by U5-Q2 (a), proven on fixtures and its bites at U5; its one counted run after U6's merge, once a binary
  picks a WebTransport crate (the run's checks unchanged).
- **The audits:** at U5's close (and at stop B and before every run: plan.md:644); an advisory on a network-facing
  crate (plan.md:648-649, with tokio-rustls since U3c: :651) blocks the next run and goes to the advisor; a version bump
  is a dependency change, so DEP-HOLD's read comes first.
- **Disk:** the fuzz target folder under .gate/ (CARGO_TARGET_DIR, never the shared target: another toolchain and
  other flags), measured at Block 3; the runner refuses under 4 GiB free, as u4f-run.sh does.

## 3. How it is tested
- **npt-alloc:** a counted allocation, the threshold count, zero inside allocation-free code, two threads' windows
  apart; plants (dealloc counted as an allocation; the threshold off by one) each fail by name.
- **check.mjs:** the fourth exception's table, and its scan refusing an unsafe item outside the allowed ones (a plant).
- **Carry 48 and C-U5-1:** as Block 2; plants (a push that grows its storage; a counter skipped; each bound off by one)
  each fail by name.
- **The fuzz harness:** U5-Q4's rule shown on a planted decoder that allocates by an unchecked length (counted as over
  the limit); catalogue item 3's half per type: a buffer one byte over the limit allocates nothing before its refusal;
  the writer's test on canned output; the bite run FAILs.
- **-35's tool:** fixtures: a metadata file with quinn-proto 0.11.17 (FAIL), npt-sim's tree with quinn (FAIL), a swap
  diff touching crates/room (FAIL), wtransport's dangerous-configuration feature on (FAIL), the clean case (PASS).
- The evidence kinds of CLAUDE.md step 4: logic (tests on the real code), and every random draw seeded and recorded.

## 4. The carries, by number (met by number in U5's report)
- **U5's section of u4s-carries.md (:759-764):** C-U6-19's line (:760-761): **U5's runs hold no throwing or rejoining
  hostile client and no client restarted mid-run:** -34's runs drive decoders and -35's reads manifests, with no
  client at all; C-U5-1's seeded test is a unit test whose fuzzed `Input`s reach one joined slot's room (no rejoin, no
  restart), asserting bounds and counters, never press counts. Carry 48 (:762-764): Block 2.
- **C-U5-1** (plan.md:1252; RR-U4F-4, advisor-u4f-r1.md:121): Block 2.
- **Catalogue §6 at U4, item 3's allocation half** (the advisor's Q8 with nit (e), advisor-u4.md:166, :206): Block 3's
  check and section 3's per-type case. **Item 6** (the control-socket messages decode within 1,024 B and never panic;
  the advisor's Q7, :204-205): the `RoomControl` target.
- **§4.8's audit items:** tokio-rustls on the network-facing list (:651); npt-tcpinfo's four unsafe blocks with their
  invariants and tests (:652; R20, "U5 audits it", advisor-ws-docs-r2.md:410); futures-util (advisor-u3c-plan.md:87);
  "the audit of libm and every other U4 dependency" (u4-build-plan.md:962): one line per crate added since U3 in the
  audit record, read by security-auditor's seat.

## 5. The rules that stand
- Start and stops: u4f-brief.md section 1 (:288-355), read for U5: the block-end size stop is the one his U5-SIZE
  answer sets; counts are deltas on `<base>`'s; the test step's stop is 1.5 times `<base>`'s measured test step
  (TE31-10, u4f-brief.md:326), written in the fill form.
- U4f's build rules (section 4, :831-958) with 1.1's lane; the shared target and its lock for every stable build; one
  builder in an isolated worktree; at most three agents at a time; never `cd` into its worktree.
- **No pass mark, judge rule, marks.json text or existing SERIES.md row changes, and no run is dropped.** New SERIES.md
  rows only for the fuzz runs' header params; a build that needs any other such change stops, and the main session asks
  him (history.md:2262).
- **DEP-HOLD:** Block 0's stop; nothing pushed by hand; the tools installed only at the read's versions, `--locked`;
  the nightly by its date through `cargo +nightly-2026-11-01`, rust-toolchain.toml untouched.
- **Unsafe** only in npt-alloc's listed items (U5-Q3) and the fuzz crate's dependency code (U5-Q7).
- Every agent brief carries, word for word: "Write files with the Write and Edit tools only (no sed, no shell
  redirect, no python writes; never `cargo fmt` writing files: apply rustfmt's diff by Edit); an agent with no Edit
  tool rewrites its notes file whole with Write."

## 6. Size (SIZE-CHECK, history.md:2048-2051 and its reading :2067-2075)
- **What earlier units of the letter came to** (written on the card before U5's first snapshot): S: U4b 701 added lines
  (411eeee, 4 files) and U4f-F1 239 (u4f-f1-report.md:104), so S's plan is about 700 and "well past" is past about
  1,400. Beside them, S-M: U4s 1,979 (f080551's merge) and U4d2 5,219 with its data files (cc8595d's merge).

| Item (added lines in spikes/) | Range |
|---|---|
| npt-alloc: the allocator, its invariants, its tests | 180-320 |
| fuzz/: Cargo.toml with 17 targets, the shared check, the seed generator, 14 plain targets, 3 with state | 500-800 |
| the fuzz record (npt-lab) and c-u5-fuzz.mjs, with their tests | 230-460 |
| -35's tool (npt-lab), its fixtures, the bites' script | 300-550 |
| check.mjs and check.test.mjs (the fourth exception, fuzz/ in the scan) | 100-250 |
| C-U5-1's seeded test; carry 48's test | 190-380 |
| the -34 bite and its run; the scanner's and the client's entries | 80-200 |
| SERIES.md rows, the audit runner, manifests, README | 50-125 |
| **Whole** | **about 1,650 to 3,100** |

- Beside it, not code: the fuzz crate's generated Cargo.lock (counted apart at each snapshot) and the audit records.
- Midpoint about 2,350; U4f2 grew about 1.35 (history.md:7350-7351): up to about 4,200. **Past S's line before it
  starts**, so the size line is his (U5-SIZE, section 8), as U4F2-SIZE was. These are judgements on the code's shape,
  not counts.

## 7. Reviewers and rounds
security-auditor (the fuzzing, the dependencies and tools, the audits, the unsafe exception), test-engineer (the bites,
the counts, the record), rust-reviewer (the allocator, the nightly and stable split, Wasm untouched) and
netcode-architect (each decoder's coverage, the client's paths, C-U5-1's room bounds), each at xhigh, in two batches (at
most three agents at a time); at most two rounds, a third only for a Critical or High.

## 8. Questions
**For the advisor** (TEST-FIRST rule 2: inside the plan, undoable, no pass mark, judge, marks.json or money, nothing on
main; each ruling recorded with its reason and named in that day's message):
1. **U5-Q1, -34's three other targets.** plan §4.7 and §4.8 give `SessionStatus`, `RoomControl` and DelayStream's
   scanner fuzz targets; marks.json's -34 has no part for them (its notes: "plan §4.7's npt-proto messages and
   SessionControl"). (a) Run them as -34's parts run (10,000,000 inputs, 0 panics, 0 over-limit allocations), reported
   beside -34 in U5's report, a panic or an over-limit allocation there stopping U5's close like a failed test; (b)
   three new parts in marks.json (his). **My lean: (a)**, marks.json unchanged, the same numbers.
2. **U5-Q2, -35's counted run.** Nothing outside npt-net picks a WebTransport crate until U6 (1.2), so a swap at U5 reads
   0 files outside the transport only because nothing outside uses one: a check that cannot fail. (a) U5 builds the
   tool, its fixtures and its bites; the one counted run follows U6's merge ("Any machine, once, after the swap");
   (b) count it at U5 anyway; (c) tool and run both to U6. **My lean: (a)**; plan §12.1's U5 and U6 rows gain one line
   each.
3. **U5-Q3, the allocator's home.** (a) A new member crate npt-alloc as the fourth exception (deny, with an `#[allow]`
   on each unsafe item), one home for -34's harness, carry 48's test and later PM-0006-20's tick count and C-U6-15's
   bench; (b) inside the fuzz crate, with carry 48's test holding a copy; (c) inside npt-lab (a large crate made an
   exception). **My lean: (a).**
4. **U5-Q4, what "allocations above the size limits" counts** (SERIES.md:162). (a) Per decode call: each single
   allocation larger than that type's size limit, plus, for an input longer than the limit, any allocation before its
   refusal (catalogue item 3's half); each call's total bytes allocated reported (the largest, and how many calls
   passed the limit), never counted; (b) totals counted against a multiple of the limit, a new number, so his.
   **My lean: (a)**, the series' own words and no new number.
5. **U5-Q5, the sanitizer for the counted runs.** The decoders hold no unsafe code, so a bounds overrun is already a
   panic. (a) ASan on every counted run if the 17 runs fit in about 3 hours at Block 3's measured rate; (b) else
   coverage only (`-s none`) with one ASan run of 1,000,000 inputs per target as a reported smoke. **My lean: (a) if it
   fits, else (b)**, decided on the measured rates.
6. **U5-Q6, the bite's choice in a libFuzzer target.** libFuzzer owns the command line, so the bites build reads an
   environment variable `NPT_BITE` once at the target's start, recorded in the run's header as `--bite` is elsewhere.
   **My lean: yes.**
7. **U5-Q7, the fuzz crate under the checks.** check.mjs's scan covers fuzz/ (no unsafe of its own); outside the
   workspace it states the same forbid in its own `[lints]`; what libfuzzer-sys's macro expands to is the dependency's,
   read at Block 0; if that expansion trips the forbid, a fifth exception comes to the advisor before Block 3 goes on.
   **My lean: as written.**

**His (one; not a stop; for the first day's message after this brief's check):**
```text
U5-SIZE: how big U5 may grow before its build stops.
Why it matters: the plan calls U5 small (S), and the two small units built so far came to 239 and 701 lines, so
by your size rule U5 would pass its line before it starts. As the plan words it, U5 is about 1,650 to 3,100 added
lines: 17 fuzz targets, a small allocator crate, two record writers, the transport-swap checker, the checks' changes,
two tests and three planted bugs.
Options:
(a) Build U5 whole: its size line about 1,650 to 3,100 lines, the build stopping past 4,000 at a block end.
(b) Split it: U5 (the fuzzing, the allocator, the two tests and the audits: about 1,350 to 2,550, stop past 3,000)
    and U5b (the transport-swap checker and its planted bugs: about 350 to 650, stop past 1,300), U5b built when U6
    swaps the transport crates.
My lean: (a). One review panel and one merge; the swap checker is small and belongs with the dependency checks.
```
- Nothing else is his: no money, nothing on main, no run day (every run is in this container), and no marks.json
  change under U5-Q1 (a) and U5-Q4 (a).

## 9. For the main session before launch
- Fill `<base>` and the counts; re-read every "there now" line in crates/room, client, lab and sim.
- The register: U5's rulings as carry lines in u4s-carries.md's U5 section (and -35's counted run in U6's, under
  U5-Q2 (a)), word for word from the advisor's check.
- plan.md: §12.1's U5 and U6 rows under U5-Q2 (a).
- The card: S's letter record (section 6) before U5's first snapshot, and U5-SIZE's answer.
- The disk: free space before Block 3 and before the counted runs (the part-clean between waves, P27, if needed).
