# Skeptic r1, D15 (= EA-4, Low): does the finding hold?

Plan: re-read what EA-4 rests on at the reviewed head 4d78ad6 (worktree HEAD 1f7fc44 differs only under .checkpoints/):
ADR 0002 OQ-0002-8 and the Advisor's answer (a) (0002:1100-1102), why room / client-endpoint / navbake exist
(0002:221-237), docs/adr/README.md:74, the report draft (docs/reports/2026-11-12-foundation-2-2.md, line 185 and a
grep of the whole report for OQ-0002-8, "not researched", room, client-endpoint, navbake), the approved plan (did it
promise anything on OQ-0002-8?), the migration plan's steps 2.2-2.6 (which step first gives room and client-endpoint
code: the fix names 2.5), the workspace members at 4d78ad6 (is navbake built?), and CI job 131510223737's wasm32 step.
Then judge: holds or not, severity on CLAUDE.md's scale, and whether the one-sentence fix is right and complete.

## Checks
- c0: worktree HEAD 1f7fc44; `git diff --stat 4d78ad6 HEAD -- . ':(exclude).checkpoints'` is empty, so every
  non-checkpoint file reads the same at the reviewed head.
- c1 (the ruling, as cited): ADR 0002:1095-1102 holds OQ-0002-8, options (a)/(b), and "Answered by the Advisor
  (2026-11-03): "(a)." ... the `room`, `client-endpoint` and `navbake` crates stay proposed, labelled "not
  researched", and the first foundation milestone that builds them is their test." The ruling is the Advisor's, not
  Mustafa's. Restated at 0002:908-913 (trade-offs: "No research compared other placements, so the first foundation
  milestone that builds them is also their test") and in the register, docs/adr/README.md:74 (row "Structure without a
  research option (OQ-0002-8)"). 0002:221-237 gives each crate's reason: room = the local server in the Worker with no
  async runtime or transport (SLICE-ORDER condition 1); client-endpoint = the endpoint's Rust run natively and headless
  (ADR 0001's network marks, soaks); navbake = keep the bake out of the player's Wasm. All cites match EA-4's.
- c2 (the report): docs/reports/2026-11-12-foundation-2-2.md (386 lines, byte-identical at 4d78ad6 by `git show | cmp`).
  `grep -n -i -E "OQ-0002-8|not researched|unresearched|without research|proposed|navbake|their test|its test"` hits
  only line 153 ("## Proposed next step"). room / client-endpoint appear only at :8 (the goal quote), :23 ("six empty
  crates"), :172 (CRATE-NAMES). The EMPTY-SLOTS bullet (:185-186) lists OQ-0002-7, -3, -4, -1 and the spike's items 4
  and 8, not OQ-0002-8. So "the report claims nothing either way" holds as a fact.
- c3 (what "foundation milestone" means, and which step gives the crates code): ADR 0002:1194-1195 "The foundation
  ... its acceptance checks are the milestones of the Migration plan's phase 2". Migration plan 0002:1450-1456: step
  2.2's done-when checks build every crate (room, client-endpoint among them) but none that exercises a crate's reason.
  0002:1467-1472: step 2.5's done-when is "the Worker's client endpoint and local server exchange Join, JoinAccepted,
  Input and Snapshot over the loopback backend with L1 behind it in CI (ADR 0001's milestone)": the first milestone
  that runs room's and client-endpoint's code. 0002:1486: navbake "enters" at slice 2 (migration step 4), not in the
  foundation at all. So read literally, 2.2 is "the first foundation milestone that builds" room and client-endpoint,
  while it exercises neither reason: the ambiguity EA-4 names is real. navbake's "first foundation milestone" does not
  exist (it is a slice-2 milestone): a second, smaller wrinkle in the ruling's words, outside 2.2.
- c4 (who else records it): the plan at 4d78ad6 never names OQ-0002-8 or the crates' test (its 2 grep hits are :201,
  navbake in the "enters with its slice" list, and :383, "Not researched" about user namespaces); the advisor agent's
  verdicts on rev 1 and rev 3 have 0 hits. The PR's own docs add no mention: DECISIONS.md:748 ("OQ-0002-8, the
  Advisor: "(a)".") and adr/README.md:74 are already on origin/main (main's DECISIONS.md:572, README.md:74) and the PR
  diff of both files has no OQ-0002-8 line. The crate docs at 4d78ad6 (crates/room/src/lib.rs:1-2,
  crates/client-endpoint/src/lib.rs:1-2) say what each crate will hold and "Empty until its first slice" (D13's
  subject), nothing on "not researched". So no file the PR touches says whether 2.2 was the test; the silence started
  in the plan, which the advisor agent approved without raising it.
- c5 (CI, job 131510223737): fetched with curl from the public API (200, 35788 bytes) into .gate/skeptic-D15/ci/. It
  checked out 4d78ad61e652f2e35ddf51c49557e725420248c8 (log :109, :121); `cargo build --locked --target
  wasm32-unknown-unknown -p sim -p protocol -p room -p client-endpoint -p sim-wasm` (:227) compiled room and
  client-endpoint, `dev` profile in 0.06 s (:363-367); natively each ran its one `tests::stub_builds ... ok` (:255-281).
  EA-4's CI evidence holds: both are built, and built empty.
- c6 (what the test would be): ADR 0001:187-189 runs the client endpoint in "ADR 0002's `client-endpoint` crate (no
  bindings, transport or clock), which the native headless test clients run too"; 0001:1828 (load and soak: "bots as
  headless protocol clients on ADR 0002's `client-endpoint` crate"); 0001:1500 ("L1 runs the headless clients").
  Step 2.5's done-when has the loopback exchange "with L1 behind it in CI", so 2.5 is the first milestone that runs
  room's and client-endpoint's code; the native headless use is exercised by slice 2's nightly soaks over L1 at the
  latest; navbake's first build is slice 2. The fix's "2.5" is right as "the first step that gives them code".
- c7 (where follow-ups live): the report's "Follow-ups for later steps" (:181-191) are mirrored in
  STATUS_ARCHIVE.md:626-641 ("For later steps": OVERFLOW-PER-CRATE, PACKAGE-ENTRIES, EMPTY-SLOTS, BRIEF-STALE) and
  named in STATUS.md:96-99 ("later: ..."). CLAUDE.md sends every session to STATUS first, so a sentence only in the
  report would not reach the session that plans 2.5. The EMPTY-SLOTS bullet ends "each waits for the boundary spike"
  (report :186, archive :637): OQ-0002-8 is answered, not an empty slot, so the sentence belongs in its own follow-up
  bullet, not inside EMPTY-SLOTS.
- c8 (replication and partial cover): `git grep -n -E "OQ-0002-8|not researched" 4d78ad6 -- docs/reports/...` finds
  nothing (exit 1), as EA-4 says. The report's only related line is :23 ("six empty crates and five empty packages,
  one trivial test each"): an attentive reader can infer that nothing about the crates' purposes was tested, but the
  report never ties that to the ruling. No "not proven" list exists yet (that is D17 / TE-3).
- c9 (what the fix would break): report-appendix.mjs `check()` only matches `"<words>" (A<n>)` in the body
  (report-appendix.mjs, the matchAll regex); the proposed sentence has no such quote, so the words check stays green.
  No gate file, CI file or code is touched; under the review budget a Low rides another push (D1's), as D13 does.

- c10 (after the advisor consult: the pass-mark table, which ADR 0002:1194-1199 makes the foundation's milestones,
  PM-0002-35 to -69): grep of `room|client-endpoint|navbake|client endpoint|local server` over 0002:1197-1345.
  Phase tags: PM-0002-35 phase 2.1, -37 phase 2.2, -63 phase 2.4, -64 phase 2.5, -65 phase 2.6; the gate rows -40 to
  -62 carry none, but their "How" is `--prove`, which with G0-G12 is step 2.3's (0002:1457-1458). The hit that matters:
  PM-0002-44 "G5 bites (part 3: who may depend on whom)": "each fails G5 only: ... `tokio` added to `room`; `navbake`
  or `server` reachable from `sim-wasm` (each when that crate enters); wasm-bindgen or a transport crate added to
  `client-endpoint`; ..." (CI, `--prove`, every `--prove` run). Part 3's rule behind it (0002:202-205): "`room` and
  `client-endpoint` build for `wasm32-unknown-unknown` and natively, with no async runtime, no transport crate and no
  wasm-bindgen ..., and read no clock", each a gate check (G5, G8). So a 2.3 milestone plants a violation of exactly
  the constraint each crate exists for (room: no async runtime; client-endpoint: no bindings, no transport), still on
  empty crates. PM-0002-64 (phase 2.5) requires "ADR 0001's loopback milestone" to pass: the first that runs their
  code. PM-0002-72 (slice 2): soaks over L1, where ADR 0001:1500 puts the headless clients; navbake enters there.
  SPIKE_PLAN.md:428-447 maps the foundation only as "PM-0002-35 to PM-0002-69", no per-crate step.
- c11 (does 2.4 give client-endpoint code first?): part 3 (0002:190) puts "the block writer and the block's layout
  table (part 5)" in client-endpoint; part 5 (0002:303-305, 342-344): "one layout table in Rust drives the writer,
  the generated offsets and the layout hash" and the generator writes the layout "from the layout table the writer
  uses". PM-0002-63 (phase 2.4) needs PM-0002-15 ("regenerating ... the block's layout and its hash") and -16 ("the
  test builds `sim-wasm` with one field added to a section"). So client-endpoint plausibly gets its first code (the
  layout table) at 2.4, not 2.5; the ADR names no step for it, so this is a "may", not a fact. room's first code is
  2.5 (nothing at 2.4 needs it; ts-rs types come from `protocol`).
- Effect on the fix: "(2.5, under 2.3's G5 rules)" treats 2.3 as background, but PM-0002-44 at 2.3 is the first
  foundation milestone that checks both crates' defining rules, and 2.4 may give client-endpoint code before 2.5. So
  the fix's single step is incomplete, and which mark is "their test" is itself the reading the Advisor's ruling
  leaves open: one more reason to label the sentence as a reading.

## Verdict (draft, before the advisor consult)
- Holds: yes, as stated. Every cited fact re-checks (c1, c2, c5, c8); the ambiguity is real because ADR
  0002:1194-1195 defines foundation milestones as phase 2's acceptance checks and step 2.2's checks do build room and
  client-endpoint (c3), while nothing in 2.2 runs either crate's reason (c5, c6).
- Severity: Low. An omission, no wrong claim; the practical effect is small, because step 2.5's own done-when runs room
  and client-endpoint in the Worker over the loopback, so the placement gets exercised there whatever the report says;
  the cost of silence is a ruling whose status a reader must infer (CLAUDE.md: reports "honest about what was NOT
  verified"; plans and reports judged "from evidence alone"). Not None: the literal reading (2.2 = the test, passed by
  an empty build) is available to the Advisor and to a later session, and the plan and the advisor agent's verdicts
  never addressed it (c4).
- Fix: right in substance (2.2 should not count; 2.5 is the first step that gives them code: c3, c6; G5 is 2.3's:
  0002:1457), but not complete as written:
  (i) not inside EMPTY-SLOTS, whose bullet says "each waits for the boundary spike" (c7): a named bullet of its own
      under "Follow-ups for later steps";
  (ii) also in STATUS_ARCHIVE's "For later steps" and STATUS.md:99's list (docs-keeper), or the session that plans
       2.5 (which reads STATUS first) never sees it (c7);
  (iii) worded as a reading of the Advisor's ruling, not as the ruling: its words name "the first foundation milestone
        that builds them", which literally is 2.2; EA-4 itself calls its view "my reading". Label it (the session's or
        the advisor agent's reading, dated) so the Advisor can confirm or correct it;
  (iv) optional: name navbake (enters at slice 2, migration step 4, so its test is not a foundation milestone).
  Breaks nothing (c9).

## Verdict (final, after the advisor consult and c10-c11)
- Holds: yes, as stated. Severity: Low (unchanged by c10-c11).
- Fix: right in direction (say that 2.2 builds room and client-endpoint empty and does not count as OQ-0002-8 (a)'s
  test), not complete as written. Completeness points, all about the fix, none a new finding:
  (a) the step: "2.5, under 2.3's G5 rules" skips PM-0002-44 at 2.3, which plants `tokio` in room and wasm-bindgen or a
      transport crate in client-endpoint (c10), and 2.4 may give client-endpoint its layout table first (c11). Name
      the marks instead of one step: PM-0002-44 (2.3, the crates' rules bite, on empty crates) and PM-0002-64 with ADR
      0001's loopback milestone (2.5, the first that runs room's and client-endpoint's code); navbake at slice 2.
      Keep client-endpoint's claim at "first step with code": nothing read pins its native headless use to 2.5.
  (b) wording: a labelled reading of the Advisor's ruling (dated, the session's or the advisor agent's), not the
      ruling; not a "Decisions needed" entry, no push of its own (it rides the D1 push).
  (c) placement: its own bullet under "Follow-ups for later steps", not inside EMPTY-SLOTS ("each waits for the
      boundary spike").
  (d) reach: the same named item in STATUS_ARCHIVE "For later steps" (:631-641) and STATUS.md:99's list
      (docs-keeper); then the archive's [verified]/[assumed] list at :556-560 takes the new name too.
- Breaks: nothing in the report (the quote check matches only `"..." (A<n>)`); the docs-keeper edits touch no gate
  file.
- Suggested sentence (for the session to weigh): "2.2 builds `room` and `client-endpoint` empty (one stub test each;
  CI's wasm32 step compiles both), which exercises none of the reasons OQ-0002-8 (a)'s crates exist for, so this
  report does not count 2.2 as their test. The session's reading, for the Advisor to confirm: their test is the first
  marks that exercise those reasons, PM-0002-44 at 2.3 (G5 fails `tokio` in `room`, wasm-bindgen or a transport crate
  in `client-endpoint`) and step 2.5's skeleton (PM-0002-64, with ADR 0001's loopback milestone: the local server and
  the client endpoint in the Worker); `navbake`'s comes at slice 2."
