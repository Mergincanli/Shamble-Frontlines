# Skeptic r1, D13 (EA-2, Low): crate docs give the wrong time for code arriving; client-endpoint's constraints left out

Plan: confirm worktree = 4d78ad6 outside .checkpoints/; read the doc lines of all six crates and five packages at 4d78ad6;
read ADR 0002 part 3 (client-endpoint row, "each crate enters with the slice that needs it"), part 5 (layout table), the
migration plan steps 2.1-2.6 (esp. 2.5) and the meaning of "slice" vs "foundation"/"skeleton step"; ADR 0004:1252; REPO_MAP:18;
the approved plan's text for these doc lines (was the wording planned/approved?); report :116-119; then judge each half of
the finding, its severity, and the proposed fix (does it break anything: gate, repo-map check, approved plan, report).

Checks:
- C0: `git diff --name-only 4d78ad6 HEAD` outside .checkpoints/ = 0 files; 4d78ad6 is an ancestor of HEAD (ed393f0). Reading HEAD's files = reading 4d78ad6's.
- C1 (doc lines, 4d78ad6 = HEAD for these files, `git diff --quiet`): sim/src/lib.rs:2, room/src/lib.rs:2 and
  client-endpoint/src/lib.rs:2 each end "Empty until its first slice." (all from 566363f; 70f1c31 touched only the
  overflow tests). protocol: "Empty until the protocol's first messages."; devtools: "Empty until its first tool.";
  sim-wasm: "Empty, with no bindings, until the boundary spike reports". Packages: client and sim-worker "Empty until the
  skeleton step.", types "...the generated-types step.", e2e "...the gate step.", net "...slice 3.". Evidence quoted in
  EA-2: exact.
- C2 (what "slice" means): ADR 0002 migration plan 0002:1431-1500 numbers phase 2 = the foundation (2.1-2.6), then
  phase 3 = "Slice 1" ... phase 8 = "Slice 6"; GLOSSARY §8 (research/GLOSSARY.md:750) defines the foundation as "Phase 1's
  work before slice 1's gameplay". The PR's own report uses "the first slice" for slice 1 (report:183, OVERFLOW-PER-CRATE:
  "content ... (the first slice)"). Only part 3's lead (0002:182, echoed ENGINE_OVERVIEW:83) uses "the first slice that
  needs it" loosely, with "the foundation" as an Enters value for sim/protocol/room/client-endpoint/sim-wasm/devtools
  (0002:186-194). Read either way, "Empty until its first slice" does not say when code arrives: as slice 1+ it is wrong;
  as part 3's Enters stage it names the foundation, which the crates already entered at 2.2.
- C3 (when code arrives): 0002:1467-1472 step 2.5 "the Worker's client endpoint and local server exchange Join,
  JoinAccepted, Input and Snapshot over the loopback ... the page receives one block per tick ... the first recording's
  per-tick hashes match natively twice and in Wasm"; part 3's sim-wasm row (0002:191): the local server = `room` + `sim`;
  client-endpoint holds the block writer (0002:190). ADR 0001:1778-1782 (foundation: "the Worker's client side and local
  server exchange Join ..."); ADR 0004:1252 (phase 2, the foundation: "the sim crate ... a replay runner, and the first
  recording in CI"; milestone "its per-tick hashes match natively twice and in Wasm"). So sim, room and client-endpoint get
  code in the foundation, before slice 1. First half of EA-2 holds.
- C4 (nuance on the fix's step number): step 2.4 (0002:1465-1466) is done when "the marks of the spike's item 4 hold in
  CI"; PM-0002-63 (0002:1329) puts PM-0002-15..19 in phase 2.4; PM-0002-16 (0002:1280): "the test builds sim-wasm with one
  field added to a section and runs the page's startup check"; part 5 (0002:342-345): the layout generator reads "the
  layout table the writer uses", which part 3 (0002:190) puts in client-endpoint; PM-0002-15 needs "an exported Rust type"
  (ts-rs: protocol). So client-endpoint's layout table (and protocol types, sim-wasm bindings) most likely arrive at 2.4,
  one step before the fix's "skeleton step (foundation 2.5)". For sim and room, 2.5 is the first step that needs code.
  (Inference from the PM table; the ADR never says which step first writes client-endpoint code.)
- C5 (second half): client-endpoint/src/lib.rs:1-2 names prediction and reconciliation, interpolation buffer, snapshot
  decoding, aim cast, block writer; it omits part 3's "with no bindings, no transport, no async runtime and no clock",
  "the block's layout table" and "natively for the headless clients of part 8" (0002:190). Its siblings carry their
  constraints (room:1 "no I/O, no async runtime and no clock"; sim:2 "content only, ... no clock"), so client-endpoint is
  the odd one out. REPO_MAP:18 client-endpoint = "(prediction and reconciliation, interpolation, snapshot decoding)": no
  aim cast, no block writer, no layout table, no constraints. Part 5 0002:304-305: "one layout table in Rust drives the
  writer, the generated offsets and the layout hash". Holds as stated. Mitigation: the doc line cites "ADR 0002 parts 3
  and 5"; REPO_MAP rows are summaries (sim's row omits its constraints too), and G5 enforces part 3's rules from 2.3.
- C6 (plan): plan.md:228-229 asks only "one doc line naming its part-3 role"; no "Empty until" wording was planned or
  approved, so the fix contradicts no approved text.
- C7 (what the fix touches): tools/gate.mjs has no reference to crates/, .rs or Cargo (grep: none); repo-map-check.mjs
  reads only each row's first backticked path (repo-map-check.mjs:3, :19), so REPO_MAP:18's text can change freely; no
  notes script depends on the doc text (rust-reviewer-overflow-xcrate.sh overwrites lib.rs in its own copy). Report
  :116-119 ("crates ... unchanged since step 2 (566363f), but for ... (70f1c31)") would turn false unless it names the
  doc-comment commit, as the fix says.
- C8 (fix applied in a copy: git archive 4d78ad6 -> .gate/skeptic-D13/fixed; sim/room "Empty until the skeleton step
  (foundation 2.5)."; client-endpoint takes part 3's cell, 4 doc lines): CI's commands, offline and locked: `cargo test
  --workspace` 6 passed; `cargo test --release -p sim -p protocol` 2 passed; wasm32 build of the five crates: exit 0.
  `cargo clippy --workspace --all-targets -- -W clippy::pedantic -W clippy::nursery -D warnings` (after cleaning the three
  crates): exit 0; `RUSTDOCFLAGS=-D warnings cargo doc --workspace --no-deps`: exit 0; `cargo fmt --all --check`: exit 0.
  Same clippy and doc on the untouched head copy: exit 0. The comment-only fix breaks nothing now or under 2.3's lints.
- C3b (contrary evidence, weighed): ADR 0004:1253 (phase 3, Slice 1) lists "the tick and its loops" and "the local server
  in the Worker" under slice 1, which may be where "its first slice" came from. It does not flip C3: ADR 0002 carries the
  foundation and its step 2.5 done-when is explicit (0002:1467-1472), ADR 0001:1778-1782 says the same, and ADR 0004's own
  foundation row (0004:1252) needs per-tick hashes of the first recording natively and in Wasm, i.e. sim code before slice
  1. Slice 1 grows these crates; it does not bring their first code.
- Verdict: HOLDS, Low. Timing half: true (every numbered-slice usage puts "slice" after the foundation; the foundation's
  steps bring the first code of sim, room and client-endpoint). Omission half: true as quoted, but polish (the doc cites
  parts 3 and 5; REPO_MAP rows are summaries; G5 enforces part 3 from 2.3). No player, Mustafa or session result changes:
  2.3-2.5 are planned from the ADRs, and the stubs' doc lines get rewritten when code arrives.
  Fix: right in kind and safe (C7, C8). Caveats: (1) "(foundation 2.5)" fits sim and room; for client-endpoint it may be
  one step late (C4: 2.4's layout generator needs the writer's layout table): wording true either way, e.g. "Empty until
  the foundation's generated-types and skeleton steps (2.4-2.5)". (2) The report :116-119 sentence must change in the
  same push as the comments, or it becomes evidence that does not hold. (3) REPO_MAP:18: adding "the aim cast, the block
  writer and its layout table" is enough; the full cell would make one crate's entry unlike its neighbours'. (4) A Low
  never pushes alone: ride the D1 push if it touches crates/, otherwise a STATUS follow-up.
  Not verified: the C4 inference (the ADR never names the step that first writes client-endpoint code); the finding's
  "which a builder reads first" framing (CLAUDE.md sends readers to REPO_MAP first; not load-bearing). No file changed in
  the worktree except this notes file; scratch work in .gate/skeptic-D13/.
