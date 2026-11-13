# U4r builder brief: the L1 runner and its judged runs (rev 3, drafted by test-engineer, 2026-11-13)

Rev 2 was the night shift's (2026-11-11; `git show ed973c9:.checkpoints/npt-u4r-brief/u4r-brief-draft.md`, every cite
at df8975b), after netcode-architect's check of rev 1 (`git show
ed973c9:.checkpoints/npt-u4r-brief/netcode-architect-u4r-check.md`: REVISE, 8 Medium, 8 Low, all accepted in rev 2;
its change list stays in ed973c9 as the record). Rev 3 is this workflow's (draft step, test-engineer acting at xhigh):
rev 2 brought to the merged state of wave 2 (U4c, U4d with W1-S1, U4j, U4d-F, U4e, A13's server bans, U4d2), every
cite re-pointed to the head, the rulings since rev 2 applied, the recheck table walked and the day items filled where
a read settles them. The drafter ran no cargo, check.mjs or node script; the read-only commands it ran (git reads and
greps, a print-only `python3 -I` line map and two print-only reads of marks.json) and one own slip are in
u4r-brief-r3-notes.md. Next: netcode-architect's check of rev 3, one revision, then the advisor's check with INF2-4's
two diffs and proof logs (advisor-u4d2-m9.md Block 2, :223-227), then section 12's launch steps.

## Changes in rev 3 (rev 2 to rev 3; each with its source)

- **C1. Section 0 re-stamped** at the head fa8e335 with the sha256 of the plan, plan.md and the carries (the task's
  item 2; u4d2-brief.md section 0's form). Every df8975b cite of rev 2 is re-pointed to the line that holds its text
  now (rev 2 to head by a print-only line map of the two versions, each re-pointed line then read) or marked gone;
  section 0's table lists the cites whose text changed or went.
- **C2. -58 leaves U4r's judged runs** (the advisor's W1-N2 ruling, wave1 :331-334, with U4E-Q2 widened, wave1
  :446-452: "all of -58 is judged after U4g"; plan :769-771 lost its -58 line and §5 U4g :828-831 gained it). 7.1's
  two -58 rows, 7.3's -58 row and 2.5's -58 bullet are gone; U4R-Q5 is answered (section 10). U4r still builds the
  header param `ticks_below_wrap` (plan :752) and the counters' injectable starts in the runner, which its contract
  test uses (carries 73, 75 and 77 (a)).
- **C3. U4R-Q3 answered.** npt-lab's clippy.toml is U4d2's, merged with both type bans and the clock ban
  (U4D2-Q10; plan :674-675; lab/clippy.toml:1-16); npt-bots' clippy.toml with both type bans and the clock ban is
  U4r's (the advisor's U4d2 brief check, A13, d2-ruling :588-589 and :725; plan §4.11 :393-396). Section 4's lane gains
  crates/bots/clippy.toml.
- **C4. U4R-Q2 answered in part by carries 82 to 85** (d2-r1 Blocks 4 and 5; d2-r2 Blocks 1 and 2): the recordings
  manifest's schema and its reader (crates/lab/recordings.json; hash.rs's `manifest` :213 and `regenerate_all` :292),
  check.mjs's `wasm` and `deps` steps, check.test.mjs, TickHash's struct doc (lab/src/lib.rs:8) and loader.mjs's
  header sentence (crates/hash-wasm/loader.mjs:13-14) are U4r's, so the lane (section 4) and the seam rule (2.6) name
  them; the determinism step's route is decided by U4r's measured times under plan :800-802 and m9 Block 1 (:144-145).
  Still open in U4R-Q2: the SERIES.md rows and carry 62 (h)'s one check in marks.json.
- **C5. The carries, whole.** Rev 2 quoted carry 40 alone; section 3.1 now holds u4s-carries.md's §U4r whole as
  placeholder P1 (entries 40, 49, 58, 59, 60, 62, 63, 69, 73, 74, 75, 76, 77, 82, 83, 84 and 85), and 3.2 says, entry
  by entry, where this brief meets it (the task's item 4).
- **C6. Carry 40's new words** (the advisor's U4d close ruling, F3, close :321-327; U4E-Q5's condition, wave1
  :467-472): the room's count of commands that `rewind_at` gives no Rewind (`ClientCounters::rewinds_refused`,
  room.rs:133-135) counts as a replacement in the -30 pre-check's setup condition, so a leg with any is VOID. Rev 2's
  "an addition beyond the plan (a log, not a rule)" (its N16) and 8.2 item 1's "cannot fail at U4r" are withdrawn: 7.2,
  8.1, 8.2 and 8.4 now say the setup condition holds by construction at U4r for the tolerance and the cap only.
- **C7. -18's condition 6 settled** (the advisor's U4e round-1 ruling, R4, NC-L1, e-r1 :224-232: "-18's L1 part counts
  at U4r as planned, with no further check of mine"): rev 2's 7.1 branch "if U4e reported the bite's baseline as one
  the client still holds" is removed. Carry 77 (b)'s bursts at age exactly 32 are counted apart, and carry 58's rule
  (an overflow resync in a judged -17 or -18 run goes to the advisor before it counts) replaces rev 2's own proposal for
  a bite run healed through a `FullState`.
- **C8. R34 at the head:** npt-client never sends `Resync` (no hit for it in crates/client/src), so after the join a
  `FullState` reaches a client only from the room's overflow resync; the event queue's cap is 1,024, provisional
  (events.rs:40; U4E-Q6 (a)). 2.1's -18 record reads it so.
- **C9. R2 at the head:** an L1 link's `rtt_ns` is `None` until its first sample (l1/src/lib.rs:64, :1748-1750); 333 ms
  is only the starting state of the probe timeout (:118, :633-635). 7.2's RTT rule, 8.2 item 2 and 8.3 say so (rev 2's
  N12 (d) waited on this). New row R38.
- **C10. R15 and R33 at the head:** the pump reads no RTT; `Decoder::rtt_ns` is a getter whose source, `RttSource`
  (`Link` or `Pings`), the caller sets at `Decoder::new` (pump.rs:183-190, :224-247, :295-303); D10's greps find no
  other reader of an RTT and no replace, clip or tolerance path in npt-room. 7.2's leg glue gives both legs
  `RttSource::Link`.
- **C11. R17, a new finding: the late-command bite as rev 2 worded it would not count late.** The room's forward
  compare runs before its lateness check (room/src/input.rs:84-89 before :111-117): a delayed command whose sequence is
  not after the newest received is a repeat, never late. Rev 2's bite (hold one command, keep sending the later ones)
  would give `late_count` 0 and a mismatch. 2.5 and 7.2 now have the bite hold the command and every later one until its
  tick has run, then send it first. Option B's depth is measured, not set (input.rs:28-29, :142-145): 7.2 says so.
- **C12. R16, a new finding: `ClientEndpoint::input` stamps each command's tick (newest + lead) and sequence itself**
  (client/src/lib.rs:390-400), so the pre-check's replay clients cannot send the recording's ticks and sequences
  through it, and no endpoint accessor gives decoded records, so -57's pairing cannot read the client's frame. 2.1 and
  7.2 state a route for each inside npt-lab (plan silent), and a needed npt-client change stops (2.6).
- **C13. R30 answered:** `Recording` carries `first_tick` and `span_ms` (sim/src/replay.rs:41-54), so the pre-check's
  tick base is the recording's `first_tick`; the room's per-tick World hash is `SimCore::state_hash`
  (sim/src/seam.rs:551), written in the room log's tick records as `h` (marks/README.md:15). New row R35.
- **C14. The U4e and U4j carries in the build and the tests:** carry 74's lead from the profile (2.1), 75's starts (2.1;
  test r2), 76's session ending (new tests r8 and r9 in 2.4), 77 (2.1, r2), 49, 58, 59, 60 and 62 (a) to (h) (2.1, r2),
  69 (2.1), 73 (r2); each in section 3.2's table.
- **C15. New question U4R-Q13** (carry 63 leaves the choice "U4r's, if its brief takes crates/log; else the unit that
  next edits it").
- **C16. Section 4 brought to the rules in force** (u4d2-brief.md section 4's form): the lane runner u4r-run.sh (sha256
  349b51ad…; u4d2-run.sh copied, only the lane's names changed: u4r-run.sh.diff) with `-x` for every counted run,
  check.mjs run, `--list` or clippy confirmation, bite run and plant, logs under .gate/u4r/, its CACHEDIR.TAG check
  and 4 GB free; W1-TARGET item 3 as amended (placeholder P9), m9 Block 2's two lessons (P10) and d2-r2 Block 6's
  lesson (P11); T4 items 1 and 2 (P3, P4); TE2-5's rule (P5) with its reading after U4d2's merge (P12); B6's plant line
  (P8); Mustafa's builder lines of USAGE-SAVINGS (a) (P2); RM (T4 item 3, carry 67) only if U4r moves ES's synthetic
  recording or the World's outcome on it; carries 6, 14 and 20 by P13 to P15 (3.3); TE-M3's npt-net flake recorded
  under TE2-5, never a stop of U4r's own (d2-r1 :287-310). Rev 2's `<floor>` and its "the day session aligns their
  wording" note are gone (rule 6 states the floors).
- **C17. Section 5 at the head:** 11 steps (check.mjs:2468-2503), the base counts measured 491, bites 561, self 71 of
  71, marks 81, redact 0 (D1), the `wasm` step's 33.1 s warm and 61.6 s cold (D3), and the provenance builds inside
  each counted call (u4d2-brief.md section 4 rule 3's form).
- **C18. Section 11 filled where a read settles it** (D1, D2, D3, D5, D6, D8, D10, D11 and D12); D4, D7 and D9 stay for
  the main session with their commands.
- **C19. New section 12, "Left for the main session at launch"** (u4d2-brief.md 7.2's form): `<base>` and its counts,
  the runner, the placeholders by script, the rulings on the open questions folded, the review panel, re-stamp and
  re-check.
- **C20. Section 9 walked at the head:** 18 of R1 to R34 hold, 16 changed (each with what the code has and what the
  brief now says), and rows R35 to R44 added for what merged since df8975b.
- **C21. 7.3 recomputed** without -58: 15,600 s simulated, 468,000 ticks (13,800 s if -18 shares -17's B1 runs).
- **C22. The review panel** (section 12 item 7): plan :803's four (netcode-architect, test-engineer, engine-architect,
  rust-reviewer) plus security-auditor (carry 76's session boundary, carry 85's refusals; carry 63 if U4R-Q13 gives it
  to U4r) and infra-engineer (carry 84's time bound; the step's time against its budget), at most three at a time
  (plan :885-886), for the advisor to confirm.
- **C23. U4R-Q10's facts settled at the head** (D12): at P0 the server end's first sample, from `JoinAccepted`'s
  packet, is applied by the first net call at or after its send (l1/src/lib.rs:947-959, :1418-1465), before any later
  read at that instant; so the RTT rule's VOID cannot fire on a correct build at P0, and (a') could not show it.
  The recommendation (b') stands.
- **C24. The run scripts' target folder:** c-u2 builds into the main folder's own cache and reads its binaries from
  .gate/netcode-proof-test/target (c-u2:55-61; .cargo/config.toml:3-4), as plan §7.1 item 6 (:917-918) has the
  judged runs; 2.3 asks the scripts to read each artifact's path from cargo's JSON messages (check.mjs:2120-2140's form)
  so that they hold under either target, and section 12 item 8 leaves the choice to the main session.
- **C25. U4R-Q4 updated** for -58's move and the contract test's new suites (ES: carry 59; tick16 past the wrap: 73;
  a far event start: 75; L1 P3 with the reorder step: 62 (g)).
- **C26. U4R-Q1 gains one fact:** npt-bots' manifest names npt-net (bots/Cargo.toml:24), so an npt-lab dependency on
  npt-bots brings npt-net's tree into npt-lab's build.

Every place where the brief must hold someone's words verbatim (the carries, Mustafa's words, the advisor's sentences
the card makes binding) is a placeholder `<<P...>>` naming its file and its anchor; the main session copies each by
script at launch (section 12 item 3). Every `<base>` and count is filled then too.

---

## 0. How to read the citations

- **The head read:** fa8e335f1b9ff83cb20f7173bd4e98605c6a5bcc (`git rev-parse HEAD` in the main folder, 2026-11-13,
  the main session's checkpoint commit after U4d2's M9). Its spikes/ is cc8595d's (`git diff --quiet cc8595d HEAD --
  spikes/netcode-proof` exits 0), U4d2's merge, which holds U4c (f3f7c2f), U4d with W1-S1's npt-proto half (39f4290),
  U4j (4d2b0ef, d50396a), U4d-F (adb1bab), U4e (c88eb31), A13's server bans (1b4a3c8) and main's PRs #50 and #51
  (bfda3cd). The checkpoint saver may add commits after it; they touch .checkpoints/ only, and the stamps below are what
  the re-check at launch compares (section 12 item 8).
- **Stamps at that head** (sha256, line count): `plan` is .checkpoints/netcode-proof-test/u4-build-plan.md (1,092
  lines, 01ede63d…); `plan.md` is the proof test's plan (rev 3.3, 1,518 lines, 3ca4e12a…); `carries` is
  u4s-carries.md (644 lines, 8b7f165e…); `history` is history.md (2,011 lines, 88bc195f…). The advisor's rulings, each
  a closed record: `advisor` advisor-u4.md (7b3e76ca…), `wave1` advisor-u4-wave1.md (a45575ae…), `close`
  advisor-u4d-close.md (cbc7e1c9…), `j-ruling` advisor-u4j.md (a98dfdb4…), `j-r2` advisor-u4j-r2.md (7efa9410…),
  `df-ruling` advisor-u4df.md (842ec092…), `e-ruling` advisor-u4e-brief.md (3485a6c6…), `e-r1` advisor-u4e-r1.md
  (7c129309…), `e-r2` advisor-u4e-r2.md (14b0b180…), `d2-ruling` advisor-u4d2-brief.md (155f3376…), `d2-r1`
  advisor-u4d2-r1.md (9978b43b…), `d2-r2` advisor-u4d2-r2.md (e776f0ba…), `m9` advisor-u4d2-m9.md (f31f7876…). All of
  these are in .checkpoints/netcode-proof-test/. `card` is .checkpoints/netcode-proof-test.md, cited by section and
  item, never by line.
- **Living and closed files.** The card, carries, history and the plan are living (history's "Decisions taken" is
  newest first; carries gain entries at merges): every placeholder in a living file is copied by a text anchor, and
  every plan or carries cite is re-checked at launch if its sha256 moved (section 12 items 3 and 8). The rulings are
  closed records: their placeholders are copied by line range with the file's sha256 asserted.
- `catalogue` is docs/design/netcode-proof-messages.md; `marks.json`, `SERIES.md` and `README` are
  spikes/netcode-proof/marks/'s; `c-u2` is spikes/netcode-proof/ops/runs/c-u2-pm-0001-3.mjs; `check.mjs`,
  `check.test.mjs`, `Cargo.toml` and `Cargo.lock` are spikes/netcode-proof/'s; a crate path (`lab/src/main.rs`) is
  under spikes/netcode-proof/crates/; `ADR 0001` is docs/adr/0001-multiplayer-netcode.md (unchanged since df8975b).
- **How rev 2's df8975b cites were re-pointed:** files unchanged since df8975b keep their lines (advisor-u3q.md,
  advisor-u2.md, u4-conditions-check.md, c-u2, crates/link/src/lib.rs, crates/lab/Cargo.toml, crates/bots/Cargo.toml,
  ADR 0001, .claude/rules/netcode.md); `advisor` gained only four lines at its end (:476-479), so every rev 2 cite of it
  holds; the plan, plan.md, carries, the catalogue, marks.json, SERIES.md, README, check.mjs, lab/src/main.rs,
  l1/src/lib.rs and judge/src/eval.rs moved, and each cite into them below is the head's line. The cites whose text
  changed or went:

| Rev 2's cite (df8975b) | What it named | At the head |
|---|---|---|
| plan :737 | "-58 for 10 s at P0 and P3 ..." in §5 U4r | gone (W1-N2); the line is now §5 U4g's, plan :828-831 |
| plan :454 | the -58/lt-tick-offset row ("offsets crossing the wrap") | rewritten by W1-N2: plan :470 (command sequences crossing the wrap; U4g's Predictor) |
| plan :445 | -18/no-fallback's run setting ("a 1 s gap evicts B") | plan :461 (on Loopback `npt_sim::ACK_RING_TICKS` tick periods: U4E-Q14) |
| plan :380-383 | §4.11's new clippy.toml files (clock ban) | plan :393-396 (with the HashMap and HashSet type bans: A13) |
| plan :193-197 | §4.1's npt-room, npt-server, npt-bots and npt-lab rows | plan :194, :195, :197, :198 (rows widened by W1-PUB, e-r1 T6 and d2-r1 T1) |
| plan :478-480 | §4.15's sources (the generator in test support) | plan :494-497 (`Recording::synthetic` in npt-sim's library) |
| plan :651-657 | §5 U4d2's `begin`, `hash` and `wasm` step lines | plan :668-683 (`begin(bite, arm_tick)`; npt-lab's clippy.toml by U4D2-Q10) |
| plan :832-834 | the wave-2 Lanes line | plan :873-880 (U4d2's lane widened) |
| carries :176-182 | carry 40 | carries :256-264 (its words changed: F3 and U4E-Q5; C6) |
| carries :146-147 | carry 21 (the canonical order) | carries :148-153 |
| carries :184-189 | §U4h | carries :548-560 |
| marks.json :3845-4308, :6325-6390 | -17, -57, -58, -18, -30 | -17 :3845, -57 :4008, -58 :4107, -18 :4262, -30 :6512 (U4j's split of -18 and its edits) |
| lab/src/main.rs:132-135, :151, :10-11 | U2's probe header: `burst_starts_ns`, `tick_hz` 30, exit 2 | :142, :159, :12-13 |
| l1/src/lib.rs:349-353, :515, :527, :858 | `rtt_ns` before a sample; P0's ack timing; `with_bite` | :64 and :1748-1750 (None); :947-959 and :1418-1465; :1828-1840 |
| judge eval.rs:30, :310-323, :1048-1051, :1283 | select by marks; the gap rule; gaps make a run invalid; expect-invalid | eval.rs:32, :314-327, :1165-1168, :1412 |
| check.mjs:678, :712-747, :1321, :1380-1385, :1474-1507, :1532 | bite ids; one home; counts; the test step; the steps; the last line | :879, :914-924, :1864, :1973-1982, :2468-2503, :2548 |
| catalogue :249-252, :282 | §6 item 5; Q8 | :251-254; :284 |
| plan.md:616-627 | npt-lab's §4.7 entry with RQ8's order | plan.md:617-628 (order :618-624) |

---

## 1. Start

1. **Base:** `<base>` (P6, filled by the main session at launch: the spike branch's head then; U4r depends on U4c,
   U4d2, U4e and U4j, plan :750, all merged by cc8595d). In your worktree: `git merge --ff-only <base>`; if that cannot
   fast-forward, `git switch -c u4r-build <base>`; then `git log -1 --format=%H` shows `<base>` and `git merge-base
   --is-ancestor cc8595d HEAD` exits 0. Record both in your notes before anything else.
2. **Who and where:** one builder, general-purpose, opus, effort xhigh, in an isolated worktree under
   `/home/user/Shamble-Frontlines/.claude/worktrees/` (`isolation: worktree`), under plan §7.1's shared-target rule and
   W1-TARGET's lock (section 4; plan :909-921; wave1 :558-586). U4r runs alone after wave 2; U4f, U4g and U4h follow it
   one at a time (Lanes, plan :881-882). The judged runs and the -30 pre-check are not yours: the main session runs them
   after the merge, in the main folder (plan :766; §7.2 :927-928; section 7).
3. **Your notes:** `.checkpoints/netcode-proof-test/u4r-build.md` in your worktree. Its first line names this task;
   then your plan as blocks (one block: one type with its tests, one test, one bite and its proof, one run script, one
   check.mjs change with its self-tests); each finished block written at once, a notes line per finding, one line after
   each block (done, what next). On a restart, read your notes and `git diff` in the worktree first, then continue after
   the last block marked done (section 4's builders' limits).
4. **The runner:** `/home/user/Shamble-Frontlines/.checkpoints/netcode-proof-test/u4r-run.sh` (P7: sha256 349b51ad…,
   made by the main session from u4d2-run.sh, only the lane's names changed: u4r-run.sh.diff). `bash <runner> [-x] [-k]
   <worktree> <log> <command...> [:: <command...>]` takes the lane lock for that one call (`-x` exclusive, else
   shared), waits until no other cargo or rustc runs, checks 4 GB free, touches every file under your copy's crates/
   first and every 2 s while the commands run, runs each command from your copy's spikes/netcode-proof with the shared
   target and `CARGO_NET_OFFLINE=true`, and appends each exit code and the provenance lines (u4r-run.sh:2-16). It stops
   with exit 5 before taking the lock if the shared target has no CACHEDIR.TAG, and before any command if the lock's
   file cannot be opened or the lock taken (u4r-run.sh:12-15; m9 Block 2). It stops at the first command that fails;
   with `-k` it runs them all and exits with the first non-zero code. A run's result is its exit code and the test
   runner's own summary lines, never a grep (section 4).
5. **What stops you** (each written in your notes and report, then build the rest): a change to a file outside your
   lane or to an item another unit merged (2.6); a failure under the stop clause (section 4); a manifest change before
   U4R-Q1's ruling; a new external dependency; disk below section 4 rule 6's floors; a part a ruling leaves open (every
   U4R-Q of section 10 is ruled before launch: section 12 item 4; where this brief says "by U4R-Qn" it follows the
   recommendation, and the main session's fold puts the ruling's words in its place).
6. **Read before you build** (and nothing you do not need):
   - plan §5 U4r (:750-803) and what it builds to: §2's rows C2, C4, -18 split and "Sizes for fixes.rs" (:145-150);
     §3's U4-SEAM and U4-SERVER (:162-170); §4.1 (:178-203; npt-bots :197, npt-lab :198); §4.4's starts (:285-286);
     §4.5 (:292-304); §4.6 (:306-328; -57's pairing :327-328); §4.7 (:330-336); §4.9 (:343-358); §4.11 to §4.15
     (:392-504); §5 U4e's Builds and -18 lines (:715-741); U4g's -58 line (:828-831); U4h's Builds (:835-839); Lanes
     (:850-888); §6 (:890-906); §7 (:908-931);
   - plan.md §4.2 (:421-454; "Nothing depends on npt-lab", :431-432) and §4.7's npt-lab entry with RQ8's order
     (:617-628), its binaries (:611-612) and bites (:630-631); context only, U7's work: plan.md's U7 row, C-U7-2
     (:1247: U7 splits loader.mjs; carry 85 (a) edits its header sentence first);
   - advisor: the U4 verdict's conditions 2, 3, 5, 6 and 9 (:90-111, :119-129, :140-142), 10 (:143-149) and 12
     (:154-158); O1 (:343-347); the F2 ruling (:436-465); and every ruling of section 10's table;
   - wave1: W1-N2 (:305-351), W1-U4E (:437-497: U4E-Q2, Q5, Q6), W1-TARGET (:558-586); close B1.1 to B1.5 and F3
     (:21-228, :321-327); j-ruling's "Carries", U4r (:402-406) and R2.4 (:438-449); j-r2 B2.1, B2.3 and B6
     (:88-130, :146-171, :283-298); df-ruling R3, R4.1, R6 and T4 (:182-244, :301-322, :459-470); e-ruling Block 5's
     U4E-Q13 (:246-264) and "For later briefs" (:338-344); e-r1 R1, R2's A5, R4 and R5 (:57-122, :157-166, :221-250,
     :282-285); e-r2's merge note (:190-196); d2-ruling A13 (:568-592); d2-r1 Block 5 (:287-310) and its routes
     (:205-216); d2-r2 Blocks 1, 2 and 6 (:59-170, :294-311); m9 Blocks 1 and 2 (:82-266);
   - carries §U4r whole (section 3.1) and entries 6, 14 and 20 (3.3); the recordings section (carries :461-489), for
     2.6's RM line;
   - marks.json as merged at `<base>`: -17 (:3845 on), -57 (:4008 on), -18 (:4262 on), -30 (:6512 on), PM-0001-2
     (:152 on), -20's and -21's parts (for U4R-Q4); SERIES.md's rows of section 9's R26; README §1 (:8-86) and §4
     (:247-316);
   - catalogue §3 Field groups (:125-158) and §6 at U4, item 5 (:251-254); its `Resync` row (:90);
   - advisor-u3q.md:180-186 (-18's fresh-delta reading); advisor-u2.md:72-83 (C1, C2) and :97-103 (records; C4);
   - c-u2 (the run-script model, 114 lines);
   - the code as merged at `<base>`: crates/lab (all src files, Cargo.toml, clippy.toml, recordings.json), crates/bots
     (a library stub and two binary stubs), crates/l1 (lib.rs's model doc :1-70 and `L1Net` :1790-1898), crates/room
     (room.rs, input.rs, rewind.rs, events.rs, lib.rs), crates/server (lib.rs, pump.rs, schedule.rs's `new` and
     `with_bite`), crates/client, crates/log, crates/judge (eval.rs's gap and expect-invalid rules; figures.rs's
     coverage), crates/sim (replay.rs's `Recording` :1-60; seam.rs's `SimCore` :545-565; world.rs's `new` and
     `with_bite`), crates/proto (lib.rs, quant.rs's `BITES` and `with_bite`, input.rs's `Input`, snapshot.rs's
     `EntityDelta`), crates/hash-wasm/loader.mjs's header (:1-30); check.mjs's steps (:2468-2503), test step
     (:1973-1982), deps step (:2062 on) and wasm step (:2099 on), and check.test.mjs's tests of those two steps;
   - .claude/rules/netcode.md: binding on proto, room, client and server code (plan §7.1 item 8, :920-921); U4r edits
     none of those crates, but its runner, replay clients and bots feed them, and its determinism lines (seeded RNG, no
     clock in shared code, deterministic iteration order) are what the twin test checks.

---

## 2. What to build

U4r builds plan §5 U4r (:750-803) exactly: its Builds, its Tests, and the run scripts for the judged runs (section 7),
which the main session runs after the merge. U4r moves the marks -17, -57 and -18 by those runs (plan :766-771),
records -30's pre-check without moving -30 (:772-796) and logs the sizes for fixes.rs (:797-799). Out of scope, by the
plan: `InputBuffer` option A, `ClockSync` and the cheats' `Behaviour`s (U4f, :806-809); `Predictor`, `Interp`,
`CueLog` (U4g, :819-823); -58's judged runs (U4g, :828-831: W1-N2); the view-time tolerance, the cap, the view-time
estimate with the pump's RTT, the TC shooter's `Behaviour`, the tolerance-edge stamper and the 150 ms pong delayer (U4h,
:835-839); npt-bots' npt-net glue and npt-load (U6, :896-897); -30's marked C part (U6, :894-895); fixes.rs's
run-structured bound (its own small unit after U4r, :903-904: U4r only logs the sizes). Where this brief finds the
plan silent it says so; it adds nothing it does not mark as its own (2.4 r6 and r7, 7.2's added records, 8.4's switch),
each there for the advisor to keep or drop. The carries' own tests and reports (section 3.2) are the carries', not
additions.

### 2.1 npt-lab (crates/lab/src/, with the lane section 4 gives)
Each item from plan :751-757, built to the section named.
- **`SimClock`** (integer ns; plan.md:617).
- **`L1Scenario`, in RQ8's order** (plan.md:618-624), at each simulated instant: (1) packets due, by arrival time and
  then fate-log index, never delivered in the instant they were sent, P0 included (l1/src/lib.rs:65-70; `poll`
  :1667-1681); (2) the server's I/O step: per session `Decoder::pump(link, now, tick, out)` (server/src/pump.rs:331),
  then the room's `join`, `receive` and `pump_report` for what it decoded (room.rs:637, :713, :700); (3) the server
  tick if due (tick k due at base + ⌊k × 10⁹ / TICK_HZ⌋ ns, through U4b's `Schedule` built as
  `Schedule::new(npt_sim::TICK_HZ, npt_sim::LOOP_CATCHUP)`: schedule.rs:77, carry 6), `Room::tick` (room.rs:805), then
  each session's output (`take_datagrams` :595, `take_control` :607) onto its link; (4) the client frames that are due
  (60 Hz, seeded phase); (5) each client whose `next_due_ns` is due (`StepOut::next_due_ns`, client/src/lib.rs:193-195),
  in slot order. Each client's host clock is offset + rate × SimClock, both seeded (plan.md:625; the 1.5 rate is -21's
  cheat, U4f's). The link is U4c's `L1Net`, event-driven through `next_event_ns` (l1/src/lib.rs:1876-1882; L-c5); L1's
  RTT is its RFC 9002 estimate, never the simulator's true value (plan.md:626). npt-lab names `npt_sim::World` once
  (U4-SEAM, plan :162-163). The plan gives `L1Scenario` no test of the order (plan silent, not added): say in your
  report how the order is shown.
- **The header params** (plan :751-752): `scene`, `clients`, `profile`, `seed`, `cmd_timing`, `tick_start`,
  `ticks_below_wrap`, `suite`, `stand_ins`. `cmd_timing` is `B` in every U4r run (option A is U4f's, :806);
  `stand_ins` records S1's 1 ms burn and 2.80 ms top-up as off on L1 (§4.13 :448-449). Each header also records the
  bite id, the enabled features and the binary's SHA-256 (plan.md:630-631), and the params the runs and the carries
  read: `burst_starts_ns` (as U2's probe writes it: lab/src/main.rs:142); `buffer_depth_ticks_count` for -30's report
  (marks.json:6557; 7.2); `twin` for the twin test's runs; the lead beside `cmd_timing` (carry 74); the scene's weapon
  (interval, damage, reach, magazine, reload, spread) and its soldiers' health from `Scene::weapon()` and
  `Scene::health()` (carry 60); the event cap and whether it is provisional (`Room::event_cap`, room.rs:558: carry 57's
  "for the run's header"). The header's `tick_hz` comes from `npt_sim::TICK_HZ` by path, not a literal as U2's probe
  header has it (lab/src/main.rs:159; carry 6's pattern, the F2 ruling's point 2, advisor :445-449).
- **The `--bite` dispatch to each instance** (plan :752; §4.11 :399-401): one `--bite <id>`, handed to each instance
  whose crate's list holds it, not a static: `World::with_bite(scene, seed, first_tick, bite, arm_tick)` (world.rs:944;
  `World::BITES` :937-938), `Room::with_bite` (room.rs:510-518; `npt_room::BITES`, room/src/lib.rs:37-46),
  `ClientEndpoint::with_bite` (client/src/lib.rs:308-320; `npt_client::BITES` :28-32), `L1Net::with_bite`
  (l1/src/lib.rs:1828-1840; `L1Net::BITES` :1823-1824), `Schedule::with_bite` (schedule.rs:257; `npt_server::BITES`
  :251). An npt-proto id (`npt_proto::BITES`, quant.rs:34-42) goes to every instance that codes the wire: the room's
  encoder, each `Decoder` (its `Quant`, pump.rs:224-231) and each `ClientEndpoint`, each taking its `Quant` from
  `Quant::with_bite` (quant.rs:102-112; U4E-Q4, wave1 :458-460). `catalogue-5/owner-to-all` is selected by that string
  (U4E-Q13 (a), e-ruling :262-264). An id no instance knows is a usage error (exit 2, as U2's probe:
  lab/src/main.rs:12-13). npt-lab's sources never spell another crate's id (section 4). The measured build takes no
  bite (plan.md:630-631), and C4 makes a measured run whose build lists `bites` or names a bite, or a bite run whose
  build lacks it, INVALID (judge/src/eval.rs:539-570).
- **The per-process logs with `admit` events and footers** (§4.12 :410-415; plan :753): on L1 one npt-lab process
  writes one log per process value the checks select on (`room`, `client`, `lab`), each with its own header; one
  `admit` event per client (README :80: `session`, `in_roster`; the roster rule :258); a footer per log (the complete
  rule :250); `tick.v.late_count` wherever `late-zero` applies (README :275; SERIES.md:365), the per-tick rise of the
  room's `ClientCounters::late` (room.rs:119-120, counted at :797 and :846). In the room log, each tick record's `h` is
  npt-sim's canonical hash of the room's `World` after that tick (README :15; `room.sim().state_hash()`: room.rs:563,
  seam.rs:551; npt-log's `TickRecord::set_hash`, log/src/lib.rs:144), which r1 and 7.2's comparer read; never to be
  confused with the encoder's mirror per (client, tick), `hs.server_hash` (`ClientSlot::server_hash`, room.rs:206;
  SERIES.md:317). Each client's hashes go in as `hs.decoded_hash` from `StepOut::hashes` (client/src/lib.rs:196-197)
  and its lost ticks as `tick.s.snap_incomplete` from `StepOut::lost` (:198-199; SERIES.md:320), under carry 62's
  rules (a) to (f) and carry 73's placement (section 3.2). U4e's exposed values (U4E-Q7: U4r's to write):
  `ClientSlot::burst_eviction` (room.rs:212) as `event.burst_eviction.evicted` (SERIES.md:321), `baseline_age` (:217),
  `overflow_resync` (:253), `wraps` (:258) and `StepOut::wraps` as `event.counter_wrap` (one record per counter name
  that wrapped, never one per client: carry 77 (a); SERIES.md:150), `rewinds_refused` (:264), `queue` (:278),
  `headroom` (:285), `buffer_depth_ticks` (:290), `kicked` (:295). Every tick and second record carries all its
  counters, 0 included: an empty tick or second series is a gap, and a gap makes the run INVALID (eval.rs:314-327,
  :1165-1168; README :147).
- **`scenario`** (with `--record`) and **`replay`** (§4.1 :198; plan :753-755): `replay` is the x86-64 re-decode (R-8,
  the advisor's condition 9, :140-142): it re-decodes a run's logged client bytes natively and writes the clients'
  hashes as `hs.replay_hash`, only in a `replay` log on x86_64 (carry 62 (a); SERIES.md:319). The clients' received
  bytes (datagrams and stream bytes) are logged under .gate/ (plan :762); never committed (plan §7.2 :927-928). U4d2's
  `hash` subcommand, `HashCompare` and `TickHash` stay as merged but for carries 82 and 83 (2.6).
- **The four runner recordings** (§4.15 :494-501; plan :755-756; the advisor's condition 3): `scenario --record` at SS
  P2 seed 1, ES P3 seed 2, TC B1 seed 3 (U4R-Q9), and SS P0 seed 4 (the -30 pre-check's source only), each written
  through npt-sim's `Recording` (`Recording::encode`, replay.rs:294), its ticks the room's `tick_input()` after each
  tick (room.rs:579: "what a recording holds"), never a second writer: `generator` 0, a runner's (replay.rs:51-52),
  `first_tick` the run's, `span_ms` the ring the room used (HISTORY_MS 2,000, provisional until U4h: room.rs:33, :446;
  carry 14's header half). They join crates/lab/recordings.json under carry 82 (2.6): its schema gains a kind or script
  key and the dispatch `manifest` and `regenerate_all` need (hash.rs:213, :292), the reader checks each entry's name
  against `[A-Za-z0-9-]+` and bounds `ticks` by `bytes / 12` before it generates anything. The determinism step's route
  (plan :800-802): 2.6 and U4R-Q2. No ES byte or bandwidth figure is taken from `Recording::synthetic`'s ES recording
  as ES's load (carry 69).
- **-57's pairing** (§4.6 :327-328): npt-lab pairs (client, tick, entity) only for the fields that tick's snapshot
  carried and the client decoded, against the server's pre-grid value at that tick (`EntityRecord::raw`, carry 9: the
  room's `sim().entity_records()`, seam.rs:554), and writes `tick.s.pos_err_um`, `tick.s.yaw_err_microdeg` and
  `tick.s.pitch_err_microdeg` (marks.json -57's checks; SERIES.md:66-73). **Plan silent on where "carried and decoded"
  is read, and npt-client exposes no decoded record** (`ClientEndpoint`'s pub items, client/src/lib.rs:253-365; R16).
  This brief's reading: decode each snapshot datagram the room handed out for that client (`take_datagrams`,
  room.rs:595) with npt-proto's decoder and the `Quant` the client decodes with, take each `EntityDelta`'s carried
  fields (an `Add`'s whole record; an `Update`'s `changed` bits, with its absolute grid integer: snapshot.rs:62-77),
  and count a field decoded only at a tick whose hash that client filed and which is not the sentinel
  (`StepOut::hashes`). The error is the value the grid integer stands for against `raw`. State the route you took; one
  that needs an npt-client or npt-proto change stops (2.6).
- **-18's heal record, by the fresh-delta reading** (plan :770; advisor-u3q.md:184-185; now in -18's own notes,
  marks.json:4442: "from the link's return to the decode of the first snapshot the room writes at or after the return
  instant (a fresh delta; deltas queued during the burst do not count)"): per client and burst, one
  `event.burst_heal` with `after_link_ns`, the SimClock ns from the link's return to the first decode of a `Snapshot`
  delta encoded at or after that return (a snapshot of a tick whose run instant is at or after it), and `burst`, its
  index (SERIES.md:134, :323). A `FullState` never counts (catalogue :150: no baseline). A client that never heals
  writes none, which the ≥ 480 count catches (marks.json -18's second check). Per client and burst, also: whether a
  `FullState` reached that client between the burst's start and its heal (the lab sees it on the FullState channel;
  npt-client never sends `Resync`, R34, so after the join only the room's overflow resync sends one:
  `ClientSlot::overflow_resync`, room.rs:253); `event.burst_eviction` (fields `slot`, `burst`: SERIES.md:321); the
  baseline's age at the first tick after the burst (`baseline_age`, room.rs:217), so that the bitten runs' bursts at age
  exactly 32 are counted apart (carry 77 (b)); and each client's relevant gameplay-event rate and its event queue's
  high-water mark per burst (`queue`, room.rs:278: carry 58).
- **The counters' starts** (§4.4 :285-286): `tick_start` and `ticks_below_wrap` are header values; the room's event
  counter starts by `Room::with_event_start` (room.rs:488) and each endpoint's command counter and event rebuild by
  `with_starts(start, event_start)` (client/src/lib.rs:298-302), the endpoint given the event start the room was given
  (carry 75); every client is admitted before the room has stamped 32,768 events past that start (about 30 s of SS's
  events), a later admission coming to the advisor first (carry 75). -58's judged runs are U4g's (C2); U4r's contract
  test runs these settings (r2).
- **The lead** (carry 74): each endpoint's `with_lead` (client/src/lib.rs:291-295; 1 by default, :262) from the run's
  profile, its largest one-way delay (half the round trip plus half the jitter: `Profile::one_way_ns` and
  `half_jitter_ns`, l1/src/lib.rs:215-221) in ticks rounded up, plus 1, recorded beside `cmd_timing` in the header.
- **Session handling** (carry 76; tests r8 and r9): when `Room::join` reports a replacement (`Ok((slot, true))`,
  room.rs:632-637), the lab sends the old session `Kick(replaced)` and drops its `Decoder` and link before it routes
  anything else; when the room kicks a client (`ClientSlot::kicked`) or takes its `Leave`, the lab drops that session.
- **Scripted spawns and teleports** (carry 49): the runner refuses one above World's stated clamp bound (TUNE-2) as a
  scenario error.
- **The -30 pre-check's replay clients** (their encoder, their pacing, the tick base, and the per-client records the
  RTT rule reads) and the late-command bite: section 7.2.
- **The sizes for fixes.rs** (§2 :150; plan :797-799): on L1 B1 runs of the stress scene, each client's per-tick
  snapshot payload and datagram count, each input's payload, and the Join, JoinAccepted and FullState sizes, as series;
  SERIES.md already has `tick.s.snap_datagram_bytes` and `tick.s.snap_wire_bytes` with npt-server as their writer
  (:70, :72): say whether you write those names from npt-lab or new ones, which need rows (U4R-Q2 (a)); state the
  WebSocket framing they assume, as plan :797-799 gives it. U4j's FU-6 waits on these per-message sizes (j-ruling
  :384-391).
- **Tokens:** the lab passes each slot's token into `Room::new` (`Vec<[u8; 16]>`, room.rs:431-438; minting is U6's,
  plan :716), seeded, never logged in clear.

### 2.2 npt-bots' library and its clippy.toml (crates/bots: src/ but src/bin/, clippy.toml, tests/)
- **`Behaviour`** (§4.1 :197), clock-injected and transport-free (U4-SERVER, plan :169-170), with the two kinds U4r
  builds (plan :756): the brawl (the stress scene's 48 soldiers fighting in 3 × 16, holding fire so the scene's shot
  rate can meet PM-0001-2's band: §4.13 :428-447; and a short ES suite: carry 59) and the open-loop scripts (the
  contract test's suites, U4R-Q4). The TC shooter is U4h's (:837); the cheats and the never-acker are U4f's (:808-809).
- The button bits and press-counter indices come from npt-sim's constants (carry 17; `Command::FIRE` and
  `Command::PRESS_FIRE`, as the room reads them: room.rs:840, :855). At the head npt-bots depends on npt-link,
  npt-proto, npt-client, npt-net and npt-log, not on npt-sim (bots/Cargo.toml:20-25), npt-proto re-exports no npt-sim
  item (proto/src/lib.rs:31-44), and npt-lab does not depend on npt-bots (lab/Cargo.toml:23-34): U4R-Q1.
- **crates/bots/clippy.toml** (new: A13, d2-ruling :588-589; plan §4.11 :393-396): the HashMap and HashSet type bans
  and the clock ban (`std::time::Instant`, `std::time::SystemTime`, `tokio::time::Instant` and their `now`),
  crate-wide, in lab/clippy.toml's form (:1-16); U6's runtime half adds its reviewed allows (§4.11 :395-396). Its proof,
  in A13's form for the server's file (d2-ruling :584-588): a scratch `std::collections::HashMap` and a scratch
  `std::time::Instant::now()` in crates/bots/src, each refused by clippy by name under the runner (`-x`), each restored
  by Edit and its sha256 checked.
- crates/bots/src/bin/ (npt-bots, npt-load) stay as merged: their U4 halves are not in the plan.

### 2.3 Run scripts (ops/runs/c-u4r-*.mjs)
- As c-u2 does (plan :756-757): `--date YYYY-MM-DD` (c-u2:40-42); refuse uncommitted changes to tracked files
  (c-u2:43-45); raw logs under .gate/netcode-proof-test/runs/<set>-<short>/ and results under results/<set>/<short>/,
  never overwritten (c-u2:46-50); both builds of npt-lab built `--release --locked` and copied out before the next build
  replaces them, then npt-judge (c-u2:52-61); `npt-judge --marks marks/marks.json --logs <logs> --only <ids> --out
  verdict.json` (c-u2:86-92); runs.txt with the commit, each binary's SHA-256, the raw-log path, the judge's line and
  one line per run (c-u2:93-104).
- **Where the binaries are read from:** c-u2 reads them from the main folder's own cache (`join(GATE, "target",
  "release", ...)`, c-u2:55 and :61; .cargo/config.toml:3-4), as plan §7.1 item 6 has judged runs (`CARGO_TARGET_DIR`
  unset: :917-918). Read each artifact's path from cargo's `--message-format=json` messages instead (check.mjs's form
  for its release builds, WASM_BUILDS, :2120-2140), so that a script is right under the main folder's cache and under
  the shared target alike, and say so; the main session chooses the target at the judged runs (section 12 item 8).
- Each script also times each npt-lab process it starts (wall time around the call) and writes it per run, for the
  ticks-per-second figure (2.4 r5).
- The sets and their files: section 7. One script per set or one for all is your choice; say which.
- You cannot run them as written: they refuse uncommitted changes (c-u2:43-45), and you commit nothing. Run each
  npt-lab command line a script issues by hand on a short setting instead, and list those lines in your report.

### 2.4 Tests (plan :758-765), each named in your report with its file
- **r1, the runner's twin test** (§4.11 :406-407): two processes, same seed and scenario: equal fate-log hashes, equal
  per-tick server and client hashes over the run (carry 62 (f): a run and its twin log `h` over the same ticks). Bite: a
  HashMap-ordered client order (marks F29), bites build only, planted in npt-lab's client loop: the two processes'
  per-tick hashes must differ. Each process logs its order digest; equal digests mean "not exercised", never caught or
  missed (the rule §4.14 :476 gives the sim's hash-map bite). Say how the second process is started (U4d's D-2 re-runs
  itself through `std::env::current_exe()`: plan :612).
- **r2, the contract test** (§4.12 :412-415; plan :758-761; the advisor's condition 5, :119-122): a short scenario per
  suite shows (a) every check of every U4 L1 part U4r moves (-17's four, -57's four, -18's B1) resolves with no gap,
  and those of the parts U4R-Q4 brings into scope; (b) every written name has its SERIES.md row; (c) the validity
  inputs exist: one `admit` event per client, a footer per log, and `tick.v.late_count` wherever `late-zero` applies
  (T18); (d) PM-0001-2's checks resolve on every SS suite (T1) and **pass** on the short runs of each precondition suite
  in U4R-Q4's scope (-17's four profiles, -18's B1, -20's P3, -21's suites); (e) the shot band per suite (§4.13
  :446-447). A suite that fails comes to the advisor before its marks count (condition 5, :122): report it, never re-run
  it into a pass. The carries add these, each in the report by its number (section 3.2 gives the words' home):
  - carry 49: per SS suite, the share of snapshots whose client set holds all 4 very large units, and the mean number of
    projectiles per client set (reported, not judged);
  - carry 58: each client's relevant gameplay-event rate and its event queue's high-water mark per burst;
  - carry 59: a short ES suite of the brawl (48 clients), PM-0001-2's ES checks resolved on it (shots, gameplay events,
    live projectiles and spawns in their bands, and the share of snapshots whose client set holds all 47 other
    soldiers and all 15 vehicles), with U4d-F's 7.250 figure beside its event rate;
  - carry 60: the weapon and health in every header;
  - carry 62 (g): `decoded_hash_mismatch_count` resolving with no gap on every -17 suite, a short L1 P3 suite with the
    reorder step on among them (no `crash` footer, 0 mismatches, no gap); (h): `tick.s.snap_incomplete`'s sum over the
    measured phase on each short L1 P0 run, and what follows from it (3.2);
  - carry 73: one short tick16 bite run past the wrap (`ticks_below_wrap` = 30), the judge's outcome the mismatch figure
    FAIL with at least one mismatch, never INVALID;
  - carry 74: on each short run the commands that arrived late (the room's `late` count) beside the commands sent;
  - carry 75: `decoded_hash_mismatch_count` 0 on a run whose event start is far from 0 (-58/lt-event-seq's setting, 3
    below the wrap, among them);
  - carry 77 (c): each client's `ClientEndpoint::stale()` sum (client/src/lib.rs:359) on each short L1 run.
  The short runs' length is not in the plan: say which you used. Negative cases for (b) and (c): U4R-Q11.
- **r3, `replay`'s U4 test** (plan :761-764; the advisor's condition 9): one short L1 run (SS P2) logs each client's
  received bytes under .gate/; `replay`, re-decoding them, gives each client's logged hash at every (client, tick)
  that has one, with the number compared stated and above 0 (a run of 0 compared fails, as the decoded-hash figure does:
  §4.6 :325). Bite: the re-decoder's grid config floored (`grid::Config` passed by value: carry 20) must give at least
  one mismatch, and the test fails by name in the bites build. The plan names no id for this bite: either the
  re-decoder's client instance takes PM-0001-17/client-rounding through `npt_client::grid_with_bite`
  (client/src/lib.rs:37-40; no new id), or npt-lab gets its own id (check.mjs's one-home rule, :914-924); say which.
- **r4, the class audit** (catalogue §6 at U4, item 5, :251-254; plan :764): SS, 48 clients in 3 × 16, P2, 3 seeds:
  every group the encoder writes into each client's snapshots and full states is one its class lets that client see
  (`owner` only to its own slot, `team` only to that team, `spatial` only inside that client's interest set or event
  test, `public` to anyone), each client's join FullState included. Bite: owner-to-all, selected by the string
  `catalogue-5/owner-to-all` (U4E-Q13 (a); e-ruling :262-264, :339), its runs' headers listing no mark, so the judge
  never reads them; it must fail the audit by name. The `team` and `public` sections are empty in the spike
  (catalogue :144-145, :153-154), so their two rules cannot fire: report them as not exercised, never as passed. The run
  length is not in the plan: say which you used.
- **r5, the runner's measured ticks per second, and the total wall time of the run set stated before it starts**
  (plan :764-765): section 7.3.
- **r6, the pre-check's pacing test** (this brief's, for 7.2's pacing rule, which the plan leaves silent): a short clean
  replay of the SS P0 recording (its first ticks, enough for every replay client's first commands; say how many) over
  both legs gives `tick.v.late_count` 0 on both legs, every command run at its stamped tick, and equal per-tick World
  hashes. Its negative case is the late-command bite (2.5), which must give exactly that leg's one late command.
- **r7, the RTT rule's unit test** (only if U4R-Q10 rules (b')): the rule's code on doctored per-client records: a
  first-sample instant after the first command's run instant gives VOID; equal instants give not VOID; no first-sample
  instant on the L1 leg before the first command gives VOID; the Loopback leg's "Some(0) from the start" never gives
  VOID; each case failing by name if the rule's code is changed to miss it.
- **r8 and r9, the session ending** (carry 76's two tests): r8, a second endpoint joins with slot 0's token and bind
  while the first is live: after it, the first's Inputs and `Leave` reach no room call and the second keeps the slot;
  r9, a kicked session's later datagrams reach no room call. Each fails by name under a plant of the lab's glue.
- **r10, the check.mjs and manifest changes** (carries 82 to 85), each with its own test that fails under a plant: the
  manifest reader refuses a name outside `[A-Za-z0-9-]+` and a `ticks` above `bytes / 12` before it generates anything
  (82); one fixture case each for the `wasm` step's clean half requiring exit 0 from `npt-lab hash` and `npt-lab hash
  compare` (83 (b)); the std-sin verdict's re-show, with its failing case (83 (c)); the npt-lab bin test of `hash
  compare` with a bite and no `--on` over two recordings, only the first listed armed (83 (d)); the two self-tests of
  84 (c); and one self-test case per refusal of 85, with the container's environment and the spike's own config still
  passing.
- Where they live: npt-lab's and npt-bots' own tests, and check.test.mjs for check.mjs's changes; nothing in the crates
  other lanes built (2.6).

### 2.5 Bites U4r plants (unit-shown; sites in U4r's files unless marked)
Each chosen per instance (§4.11 :399-401), shown by a test that fails by name in the bites build (§7.1 item 3, :913-914)
and passes in the clean build. The bites U4r runs judged but other units planted are in section 7.

| Bite | Site | Proof |
|---|---|---|
| the twin test's HashMap-ordered client order (marks F29) | npt-lab's client loop, bites only | r1: the per-tick hashes of the two processes differ; equal order digests mean "not exercised" (§4.14 :476) |
| the re-decoder's floored grid (plan :763-764) | `ReplayX86`'s decoder (id: 2.4 r3) | r3: at least one (client, tick) mismatch |
| PM-0001-30/late-command (plan :791-792; marks.json:6567) | the pre-check's replay client, one leg: the command stamped T and every later one held out of every `Input` until tick T has run on that leg, then T sent first (delayed, never dropped), the later ones after it (7.2) | 7.2: the pair VOID by `late_count` alone (that leg's `late_count` expected 1), never a pass |
| PM-0001-30/wallclock (plan :791; marks.json:6563) | "a wall-clock read in the pump for one leg's sessions" (npt-server, U4e's crate): U4R-Q6; the read changes a sim input on that leg on every run (7.2) | 7.2: the pair valid and the comparer naming a tick; a PASS is a missed bite, reported |
| carry 62 (h)'s bite, only if its check goes into -17 | "a client that writes null and incomplete 1 for a refused group": npt-lab's client writer (its id then lives in npt-lab's sources) | -17's L1 P0 setup check fails on that bite's run (carry 62 (h); U4R-Q2 (b)) |

- Bite ids are `PM-<ADR>-<row>/<name>` (check.mjs BITE_ID, :879); each id lives in one crate's sources
  (`biteHomeProblems`, :914-924). No PM-0001-30 id is planted anywhere at the head (D6).

### 2.6 The seam rule, and what stops the build
- **No existing item changes in what the earlier units merged:** npt-sim's seam, `World`, `Scene`, `Recording`,
  `replay::Replayer` and the tick constants (U4s, U4d, U4d-F); npt-proto (U4a); `Schedule`, `Due`, `due_ticks` (U4b);
  npt-l1 (U4c); npt-room, npt-server and npt-client (U4e); npt-log, npt-judge and marks/ (U4j); crates/hash-wasm,
  npt-lab's `hash`, `HashCompare`, `TickHash` and check.mjs (U4d2). **The carries' named edits are U4r's** and are the
  only exceptions: carry 82 (crates/lab/recordings.json's schema; hash.rs's `manifest` and `regenerate_all`, with the
  name and `ticks` checks); carry 83 (a) (TickHash's struct doc, lab/src/lib.rs:8, to say what its field doc says, "the
  recording both legs replay") and (b) to (d) (the `wasm` step's clean half and the std-sin re-show in check.mjs, the
  npt-lab bin test); carry 84 (the `wasm` step's one shared deadline in place of WASM_CALL_LIMIT_MS per call,
  check.mjs:2212 and :2258-2266, and a stopped call taking every process it started; or in ops/runs/det-u4.mjs if the
  Evidence decision moves the step's runs there); carry 85 (the deps step's refusals, DEPS_CHECKED's limits
  (check.mjs:2020), the text beside `buildInputProblems` (:1560), and crates/hash-wasm/loader.mjs's header sentence,
  :13-14, "the one edit U4r makes in crates/hash-wasm"); A13 (crates/bots/clippy.toml, new); carry 63 only if U4R-Q13
  gives U4r crates/log; carry 62 (h)'s marks.json check only if U4R-Q2 (b) gives it.
- **A needed change stops that part:** write it in your notes and report (the item, the change, why, the site), build
  the rest, and leave the change to the main session, which takes it to the advisor. The npt-client and npt-proto routes
  of 2.1 (-57's pairing) and 7.2 (the replay clients) are such stops if they cannot be met inside npt-lab.
- **A tick count or a named constant derived from the rate:** no local form; the need goes in your report (carry 39's
  rule, binding every crate outside npt-sim; the F2 ruling's route (b), advisor :449-458: the main session adds it by
  carry 18's route). Run settings that marks.json and SERIES.md fix as data (`tick_start` = 63,736: -17's setup check;
  `ticks_below_wrap` = 30: SERIES.md:253) are header values, never `*_TICKS` constants; durations are in ns or s, turned
  into ticks through npt-sim's functions.
- **RM (T4 item 3; carry 67, carries :462-479)** binds only a change that moves ES's synthetic recording (D-7's ES
  digest) or the World's outcome on it (t_m1's line); carries 71 and 72 likewise. U4r's plan is neither: it edits
  no crates/sim file and regenerates the three synthetic recordings with `Recording::synthetic` as merged. Your report
  says "not applicable, no change to the synthetic recordings or the World" with `git diff --stat <base> --
  spikes/netcode-proof/crates/sim` (empty). If your manifest change alters how the synthetic three are generated, that
  is such a change: stop.

### 2.7 Public items (A19; plan §7.2 :929-930)
- Allowed: npt-lab's `SimClock`, `L1Scenario`, `ReplayX86` (plan.md:617; `HashCompare` and `TickHash` are U4d2's) and
  its subcommands `scenario` (with `--record`) and `replay` beside U4d2's `hash` (§4.1 :198); npt-bots' `Behaviour`
  (§4.1 :197). Everything else `pub(crate)` (plan.md:454); `probe`, `machine` and `hash` stay modules of the binary
  (lab/src/main.rs:16-18).
- Your report lists every pub item U4r adds (types, functions, methods, fields, constants), by crate, against §4.1 and
  plan.md §4.7, and the names check.mjs newly exports for its self-tests, counted as d2-r2 counted them (EA2-1,
  :150-154); engine-architect checks them (check.mjs does not: plan :930).

---

## 3. The carries

### 3.1 Word for word
Quoted from carries' section "U4r (the L1 runner and its judged runs)" as it stands at launch (the advisor's U4s
ruling, condition 4: advisor :334-335; carries' rule, :9-10: "A brief may reword the plain parts, never the rule"). The
main session copies the block by its heading, by script, at launch (section 12 item 3). Your report says of each, by
number, "met" (file:line and the test or record that shows it), "not met" (why) or "not applicable" (why).

```
<<P1>>
```

Seventeen entries: 40, 49, 58, 59, 60, 62, 63, 69, 73, 74, 75, 76, 77, 82, 83, 84 and 85.

### 3.2 Where this brief meets each entry (notes for the builder, which change no word)

| Entry | Its home in this brief | Note |
|---|---|---|
| 40 (O1; U4E-Q5's counter) | section 8 (the statement), 7.2 (the setup counts and the RTT records), 8.4 (the switch), U4R-Q12 (the re-run) | met by the brief's text and 7.2's logs; the report states the tolerance and cap counts as "not built: 0 by construction at U4r", and `rewinds_refused` as counted (C6) |
| 49 (N-M4, TUNE-2) | 2.1 (the spawn and teleport refusal), 2.4 r2 | the two figures are reported, not judged |
| 58 (U4E-Q6's conditions) | 2.1 (-18's record), 2.4 r2, 7.1 (-17 and -18 rows), section 6 item 5 | an overflow resync in a judged -17 or -18 run comes to the advisor before it counts |
| 59 (ES under the bots) | 2.2, 2.4 r2 | a missed ES band or share goes to the advisor before any ES run of PM-0001-12 counts; no U4r judged part reads ES |
| 60 (TUNE-1's header) | 2.1 (the header params), 2.4 r2 | |
| 62 (U4j's log rules) | (a) to (c): 2.1's logs; (d): 2.1's starts and 7.2's admits; (e): 2.1's logs, the tail; (f): r1; (g) and (h): r2, with (h)'s check by U4R-Q2 (b) | (g)'s drop count does not apply: the client decodes such a group (e-r1 R4, NC-L4, :238-241) |
| 63 (FU-3, FU-7's log half) | U4R-Q13 | crates/log is not in U4r's lane unless the ruling gives it |
| 69 (NF-L1's U4r half) | 2.1 (the runner recordings) | |
| 73 (tick16's judged half) | 2.1's logs (the client writer's placement), 2.4 r2 | the -17/tick16 judged bite run (7.1) depends on it |
| 74 (the endpoint's lead) | 2.1 (the lead), 2.4 r2 | a short run with more than 1 % of its commands late comes to the advisor before its marks count |
| 75 (the event-number base) | 2.1 (the starts), 2.4 r2 | |
| 76 (ending a session) | 2.1 (session handling), 2.4 r8 and r9 | a security boundary: security-auditor's seat (section 12 item 7) |
| 77 (U4r's readings of the room and client) | (a): 2.1's logs; (b): 2.1's -18 record, 7.1's -18 row; (c): 2.4 r2 | |
| 82 (the recordings manifest) | 2.1 (the runner recordings), 2.6, 2.4 r10 | the schema change is U4r's, and this brief says so (the carry's "its brief says so") |
| 83 (the comparer; the wasm step's verdicts) | (a): 7.2 (the comparer), 2.6; (b) to (d): 2.6, 2.4 r10 | |
| 84 (the wasm step's time bound) | 2.6, 2.4 r10, section 5 item 4 | in check.mjs, or in ops/runs/det-u4.mjs if the Evidence decision moves the runs (U4R-Q2's answered part) |
| 85 (the deps step's reads) | 2.6, 2.4 r10, section 4 rule 7 (85 (d)'s test and P11) | loader.mjs's header sentence is the one edit in crates/hash-wasm |

Carry 66 (F2-ROUTE, e-ruling Block 6, :278-279: "U4r's brief takes it if U4d2's is final first") is not U4r's:
U4d2's brief carried it and U4d2 met it (lab/src/main.rs:120-123, :190-191; probe.rs:47-49), so this brief carries no
word of it.

### 3.3 Three entries routed to other units whose words reach U4r's code
Quoted word for word by placeholder (each copied by script at launch); your report says of each, by number, whether
U4r's code meets it (met, with file:line; or not met, with why). The routing stays as the file has it; whether U4r
reports them is the advisor's to confirm in reviewing this brief.

```
<<P13>>
<<P14>>
<<P15>>
```

Notes for the builder, which change no word: carry 6's npt-lab half is `L1Scenario`'s `Schedule::new` call (2.1);
carry 14's header and replayer halves are U4d's, merged (the header's span word, replay.rs:7-15; `span_ms`, :49-50),
and its U4r half is that the runner recordings record the ring the room used (HISTORY_MS, room.rs:33; U4h sizes the
ring later: §4.9 :345-347; carry 28); carry 20 binds `replay`'s floored-grid bite (2.4 r3) and every recording and
header U4r writes.

---

## 4. Build rules

**The shared target** (Mustafa's rule, 2026-11-09: plan §7.1 :909-921; the advisor's W1-TARGET, wave1 :558-586),
binding on every cargo, rustc and rustdoc call in your worktree, check.mjs's and the `wasm` step's included:
1. `CARGO_TARGET_DIR=/home/user/Shamble-Frontlines/.gate/shared-target` and `CARGO_NET_OFFLINE=true` on every call (the
   runner sets both: u4r-run.sh:18-22). Checked once after the first build: the artifacts are in that folder, and no
   `target/` appears in your worktree (advisor nit (g), :169-171).
2. **The lock** (wave1 :570-573): every such call goes through the runner, which takes `flock` on
   `/home/user/Shamble-Frontlines/.gate/shared-target/.lane.lock` for that one call and stops with exit 5 if it cannot
   (m9 Block 2, :187-199): shared for an ordinary build; exclusive (`-x`) for a counted run (check.mjs, the `--list`
   and clippy confirmations), every run of the `wasm` step or of `npt-lab hash`, each bite run, each plant (B6) and
   scratch switch, every `scenario --record` or r5 timing run whose output your report gives as a result, and every run
   with a scratch plant in place; the runner's touch is among the exclusive acts (wave1 :570-573). The lock is per
   call: a plant's commands go in one call (`-x -k`), and the plant is restored and its sha256 checked before your next
   call; never leave a plant in place between calls.
3. **Provenance** (wave1 :574-575): check.mjs prints each step's last line only (`main`, :2520-2548) and keeps cargo's
   Compiling lines out of its output (e-r1 R6), so a counted run's provenance comes from builds run before check.mjs
   inside the same exclusive runner call (u4d2-brief.md section 4 rule 3's form): `cargo test --locked --workspace
   --no-run` and the same with `--all-features` (the debug test builds), and the `wasm` step's four release builds as
   WASM_BUILDS gives them (check.mjs:2120-2140). Their "Compiling npt-lab v… (<your
   worktree>/spikes/netcode-proof/crates/lab)" and npt-bots lines the runner appends are the provenance. Match on
   the path, not the version. A missing line means "not counted", whatever the count says.
4. Touch before each build: `find <copy>/spikes/netcode-proof/crates -type f -exec touch {} +` (plan :912; the runner
   does it).
5. **Counts are deltas** (plan :913-914, :883-884), on check.mjs's measure (`testBothBuilds`, :1973-1982;
   `cargoCounts`, :1864). The base: measured 491, bites 561, self 71 of 71, 11 steps, marks 81, redact 0 (P6: the
   counts at `<base>`; at cc8595d, U4d2's merge, the same: D1). Each run's count is the base plus the
   tests you added; the step count stays 11 unless your change adds a step; the self count is the base's plus your new
   check.test.mjs tests. A mismatch means another copy's artifact ran: touch, rebuild, never report that run. `--list`
   (`cargo test --locked --workspace --all-features -- --list`) only names the new tests; it is not a count. An
   `#[ignore]` test is not in check.mjs's count (`cargoCounts` reads libtest's "passed" and "failed" only).
6. **Disk** (wave1 :576-580; the runner checks the first floor itself): `df -h /home/user` before each build, the
   reading in your notes. At least 4 GB free before any wasm32 or release build, which every check.mjs run is
   (clippy-wasm32, doc, the `wasm` step) and every `scenario --record` or r5 timing build; at least 2 GB before every
   other build; below that, stop and report; never clean anything (plan :919).
7. **Cleans and cargo probes are not yours.** No copy cleans the shared folder (plan :919). The main session's cleans
   follow W1-TARGET item 3 as amended: "<<P9>>" (m9 Block 2, :252-256). Every brief that cleans or dry-runs cargo keeps
   m9 Block 2's two lessons: "<<P10>>" (:259-264). Every brief that probes cargo keeps d2-r2 Block 6's lesson:
   "<<P11>>" (:304-308); it binds carry 85 (d)'s self-test, which reads an empty or relative CARGO_HOME through
   check.mjs's own function, never by running cargo with it.
8. After the last bite or plant, touch and rebuild before any further claim (plan :915).
9. Cargo's own lock still serializes builds: expected (plan :916). The main folder's cache is not yours (plan
   :917-918).

**Your files, and only these** (Lanes :881; carries 82 to 85 and A13 by their words; anything more only by a ruling of
section 10): crates/lab/ (src/ and tests/; recordings.json's schema and entries, carry 82; U2's `probe` and `machine`
kept as merged; U4d2's `hash`, `HashCompare` and `TickHash` kept as merged but for carries 82 and 83); crates/bots/src/
but src/bin/, crates/bots/tests/ and crates/bots/clippy.toml (new, A13); ops/runs/c-u4r-*.mjs; check.mjs and
check.test.mjs for carries 83 (b) to (d), 84 and 85 only (or ops/runs/det-u4.mjs in place of the step's runs, if the
Evidence decision moves them: carry 84); crates/hash-wasm/loader.mjs's header sentence (:13-14) only (carry 85 (a)).
- **Waiting on a ruling, so not yours until it comes:** crates/lab/Cargo.toml, crates/bots/Cargo.toml and Cargo.lock
  (U4R-Q1); marks/SERIES.md and marks.json's one check (U4R-Q2 (a) and (b)); crates/log (U4R-Q13); the wallclock bite's
  site in crates/server/src (U4R-Q6). A part that needs one stops, goes in your report, and the rest is built.
- **Never:** crates/sim, crates/proto, crates/l1, crates/link, crates/room, crates/server (but U4R-Q6's site, if ruled),
  crates/client, crates/log (but U4R-Q13's), crates/judge, crates/hash-wasm (but loader.mjs's sentence), crates/net,
  crates/tcpinfo; marks/ (but U4R-Q2's); docs/; results/ (the main session's runs write it: §7.2 :927-928).
- **Files:** "<<P3>>" (the advisor's U4d-F ruling, T4 item 1's sentence, word for word). **Plants:** "<<P8>>" (the
  advisor's U4j round-2 ruling, B6, word for word), then restored and the sha256 checked against the clean file. A
  `cp`, `mv` or `tee`, and a `>` or `2>` into a file, are shell writes too, in any folder, the session scratchpad
  included (u4d2-brief.md section 4; this brief's drafter slipped so once: u4r-brief-r3-notes.md): read an older
  revision with `git show <sha>:<path>`, and let the runner's log hold a command's output.
- **Git** only in your worktree (`cd <worktree> && git ...`), never `git -C` or `git -c`; commit nothing and push
  nothing; the main session merges (plan :924-926).
- npt-lab's sources never spell another crate's bite id as a string literal: check.mjs reads every exact-id literal
  in crates/<crate>/src/ outside test modules and fails an id found in two crates (`biteHomeProblems`, :914-924). The
  `--bite` dispatch finds each id's owner through that crate's own list at run time; tests and the run scripts may name
  ids.
- **The stop clause:** "<<P4>>" (the advisor's U4d-F ruling, T4 item 2's clause, with `<crate>` replaced by the script);
  and TE2-5's rule, "<<P5>>" (the advisor's U4j round-2 ruling, B2.3's rule), which after U4d2's merge reads: "<<P12>>"
  (the advisor's U4d2 round-1 ruling, Block 5). **Expected failures, named in advance:** a check.test.mjs pin that a
  carry 84 or 85 change re-pins (plan the edit and its re-pin as one step, or record each under TE2-5 with the step that
  re-pins it); and TE-M3's npt-net test, `l2::delay::tests::a_shutdown_s_never_completes_ends_at_its_deadline_without_
  spinning` (delay_tests.rs:2551-2603), which failed once under a counted run's load (d2-r1 :287-293): it is in no crate
  U4r changes and reads no U4r file, so a failure of it is recorded under TE2-5 with its name and panic (FX-10's file of
  the failed step kept beside the log under .gate/u4r/), and it does not stop your step group; its cause is the main
  session's planned change before U4r's merge (TE2-5 (c), d2-r1 :303-304). A scratch plant's own run fails by design and
  is reported as that plant's proof, never as a TE2-5 failure.
- **No retry for the `wasm` step** (u4d2-brief.md section 4's rule, applied to the step that judges determinism): a
  `wasm` step run that mismatches, traps, fails a SHA-256 or reports a call stopped by the limit is never re-run into
  green. Its first tick, field diff and digests go into your notes, tagged TE2-5, with the run's whole log kept under
  .gate/u4r/, and it stops your step group until it is explained. For a stopped call, until carry 84 is in: list and end
  any leftover process (`pgrep -a npt-lab`; `pgrep -af loader.mjs`) and record them (d2-r2 Block 5's interim rule,
  :255-257).
- **The builders' limits** (Mustafa, 2026-11-11): "<<P2>>" (history, word for word).
- .claude/rules/netcode.md binds proto, room, client and server code (plan §7.1 item 8, :920-921): none of them is
  yours (U4R-Q6's site aside, if ruled); your runner, replay clients and bots feed them only through their public items.

---

## 5. End checks

Run from spikes/netcode-proof/ in your worktree, each as a counted run under section 4's rules (`-x`; `df` before each):
1. check.mjs twice, each with every step passing: 11 steps at the head (fmt, clippy, clippy-wasm32, doc, test, deps,
   wasm, refs, marks, self, redact: :2468-2503), so "check: all 11 steps pass" (:2548). Each run's runner call holds
   section 4 rule 3's builds before check.mjs. Report both runs' last lines, the test step's two counts, the self count,
   the `deps` and `wasm` lines, the `wasm` step's seconds labelled as d2-r2 Block 5 labels them (warm for dependencies;
   the machine's uptime in the notes: :252-254), and each run's provenance lines.
2. clippy on the host and on wasm32, in both builds, warnings as errors: check.mjs's clippy and clippy-wasm32 steps
   (`lintBothBuilds`, :1935); the wasm32 step covers WASM_CRATES (LEAF plus npt-tcpinfo: :39, :1891), which holds
   neither npt-lab nor npt-bots, so U4r's crates are linted on the host only (crates/bots/clippy.toml read there). State
   each step's line.
3. fmt: check.mjs's fmt step (`cargo fmt --all --check`, :2469); a diff is applied by Edit (section 4's write rule).
4. **The `wasm` step's time with U4r's recordings** in the manifest, if they join it (carry 82), each run labelled:
   against the 5-minute budget, beside the base's 33.1 s warm and 61.6 s cold (section 11's D3). Over 300 s, it comes to
   the advisor as U4r's det-u4.mjs question (m9 Block 1, :117-119, :144-145); the step's runs move to
   ops/runs/det-u4.mjs only on the advisor's ok (plan :800-802).
5. The run scripts: each npt-lab command line they issue, run by hand on a short setting (2.3), with its output line.
6. `node tools/gate.mjs --no-smoke` is the merge step's, in the main folder (§5's preamble :510): not yours.

---

## 6. Return list (your report, in this order)

1. `<base>`, the `git merge-base --is-ancestor cc8595d HEAD` result, and your final diff's file list (`git status
   --porcelain` and `git diff --stat <base>`), each file inside section 4's lane (or inside a lane a ruling opened,
   named with the ruling); `git diff --stat <base> -- spikes/netcode-proof/crates/sim` (empty).
2. Each Build item of 2.1-2.3: built (file:line) or not (why); 2.1's -57 route; and 7.2's statements (the room's tick
   base on both legs, the replay clients' encoder and pacing rule and why it lands each command before its tick,
   whether an admit changes the World's hashed state), each with the file:line that does it.
3. The tests r1-r10 (r7 only if U4R-Q10 rules (b')) and any you added: name, file, what each shows; the counts in both
   builds as deltas from `<base>`'s, on the measure of section 4 rule 5, with the `--list` lines that name the new
   tests; the self count's delta.
4. Each bite of 2.5: its id (or "no id: 2.4 r3's choice"), its site (file:line), the test that fails by name in the
   bites build (the output line), and that test passing in the clean build; for r1, both processes' order digests; the
   two plants that show crates/bots/clippy.toml's bans, with each restore's sha256.
5. The contract test (r2): per suite, each check that resolved, each PM-0001-2 check's value against its band, the shot
   band, and each carry's figure (49, 58, 59, 60, 62 (g) and (h), 73, 74, 75, 77 (c)); every suite that failed and every
   figure that goes to the advisor under its carry; the short runs' length.
6. The class audit (r4): per seed, the groups audited by class, the run length, the bite's failing line; `team` and
   `public` reported as not exercised.
7. `replay` (r3): the (client, tick) pairs compared, 0 mismatches clean, the bite's mismatch count.
8. The ticks per second (r5): per scene and profile, with the method (section 7.3), the exact command used, and the
   set's estimate redone with the measured value.
9. The four recordings (or three, by U4R-Q9): each file's SHA-256, size and `scenario --record` wall time, its
   `span_ms`; the manifest's new schema and entries (carry 82); the `wasm` step's time with them (section 5 item 4) and
   the route that gives (U4R-Q2's answered part).
10. Every carry of section 3 by number (3.2's table; carries 6, 14 and 20): met (file:line, test), not met (why) or not
    applicable (why); 2.6's RM finding.
11. Every pub item U4r adds, by crate, against §4.1 and plan.md §4.7 (2.7); any beyond them named as such.
12. What stopped and goes to the main session: seam changes (the npt-client and npt-proto routes of 2.1 and 7.2), tick
    counts or rate-derived constants (carry 39's rule: name, definition, values at 30 and 60 Hz, site), and the
    out-of-lane needs (U4R-Q1, Q2, Q6, Q13).
13. The end checks (section 5): check.mjs's two runs (each step's line), clippy's lines, fmt; `CARGO_TARGET_DIR`
    checked after the first build; each `df` reading; each counted run's provenance lines and its `flock -x` use.
14. Each run script: its file, the sets it runs (section 7), the npt-lab lines run by hand (2.3), and how it reads its
    binaries' paths.
15. How each of section 10's rulings was applied.
16. Every TE2-5 entry in your notes (expected or not), and any `wasm` step run kept under the no-retry rule.
17. What you did not verify, and what is left.
18. Who built it (the advisor's U4S-BUILDER terms, advisor :365-371).

---

## 7. The judged runs and the -30 pre-check (the main session's, after the merge)

Run in the main folder, on release builds, one set at a time, in the background, raw logs under .gate/, verdicts and
runs.txt committed under results/ (plan :766; §7.2 :927-928), by the scripts of 2.3, judged by `npt-judge --only`
(plan :766). You build the scripts and show their npt-lab lines work (2.3); you run none of the sets.

### 7.1 One row per part
3 seeds per part (plan :766): seeds 1, 2 and 3, as c-u2 runs them (c-u2:23) and as U4h's judged runs are numbered
(plan :841-842). Every part's select at `<base>` holds scene SS and clients 48 where §4.12's table gives them, and the
part's validity `pre:PM-0001-2` (plan :418-424; marks.json -17's and -18's L1 parts). Bite runs: the plan fixes neither
their number nor their profile; the scripts follow c-u2's model, one run per bite at seed 1 (c-u2:80-84), with the
part's own settings (length, scene, clients), so a bite is shown on a run its part would accept; each bite run is in the
bites build and each measured run in the measured build (U4j's C4: eval.rs:539-570). -58's judged runs are U4g's, not
here (C2).

| Part (marks.json) | Layer, profile | Scene, clients | Length | Seeds | Bites (planted by) and their run settings | What the part checks |
|---|---|---|---|---|---|---|
| -17 L1 P0 (:3845 on) | L1, P0 | SS, 48 | 10 min (plan :767) | 1-3 | tick16 at `tick_start` = 63,736, so the bite run crosses the 16-bit wrap 1,800 ticks (60 s at 30 Hz) in (plan :767-768); client-rounding, any -17 profile (§4.14 :460). tick16 is npt-proto's (`npt_proto::BITES`, quant.rs:38: U4E-Q4), client-rounding npt-client's (client/src/lib.rs:29); the clean runs' `tick_start`: U4R-Q7; tick16's judged half rests on carry 73's client writer | `decoded_hash_mismatch_count` == 0; compared count and the incomplete sum reported (-17's report) |
| -17 L1 P2 | L1, P2 | SS, 48 | 10 min | 1-3 | as above | as above |
| -17 L1 P3 | L1, P3 | SS, 48 | 10 min | 1-3 | as above | as above |
| -17 L1 B1 | L1, B1 | SS, 48 | 10 min | 1-3 | as above; its bursts: U4R-Q8 | as above; also logs the sizes for fixes.rs (§2 :150) |
| -57 L1 P0 (:4008 on) | on -17's P0 runs (plan :769; -57's parts select layer and profile only) | SS, 48 | -17's | -17's | pos-4cm, yaw-12bit, pitch-8bit, in npt-proto's `Quant` (quant.rs:35-37; carry 2), the room's encoder coding with it (room.rs:510-518), each its own bite run | position error max ≤ 10,000 µm, yaw ≤ 10,000 µdeg, pitch ≤ 180,000 µdeg (-57's checks), from 2.1's pairing |
| -57 L1 P2 | on -17's P2 runs | SS, 48 | -17's | -17's | as above | as above |
| -57 L1 P3 | on -17's P3 runs | SS, 48 | -17's | -17's | as above | as above |
| -57 L1 B1 | on -17's B1 runs | SS, 48 | -17's | -17's | as above | as above |
| -18 L1 B1 (:4262 on) | L1, B1 | SS, 48 (§4.12 :421) | 10 min with 10 bursts (§4.13 :439-441's 10 bursts in 600 s; -18's notes, :4442), each burst's start offset 0 to 1 tick (plan :770; read here as from a tick boundary), the offsets in the header's `burst_starts_ns`. The plan fixes the offsets' range only: state how the 10 offsets are drawn and how the bursts are spaced in 600 s (for example a seeded draw in [0, 1 tick) ns per burst, and bursts every 60 s from 30 s) | 1-3 | no-fallback (npt-room's encoder: room/src/lib.rs:39; §4.14 :461), with `burst_eviction` logged per client and burst (plan :770-771; §4.5 :299-300); condition 6 is settled, not triggered: "-18's L1 part counts at U4r as planned" (e-r1 :224-232); the bitten runs' bursts at age exactly 32 counted apart (carry 77 (b)); an overflow resync in any of these runs is reported and the runs come to the advisor before they count (carry 58) | every client decodes a fresh delta within 500 ms of the link's return (`event.burst_heal.after_link_ns` ≤ 500,000,000 on L1 B1), and 48 × 10 heal records (≥ 480), by the fresh-delta reading (2.1), a `FullState` never counting as a heal |
| -30 pre-check, suite `precheck-30`: not a part, no mark (7.2) | leg 1 `loopback`; leg 2 L1, P0 | SS (the recording), 48 replay clients | the SS P0 recording's length (seed 4; about 5 min if it holds 9,000 ticks: §4.15 :483-484) | the recording's seed 4 only (one recording; "3 seeds per part" is for parts) | wallclock (U4R-Q6) and late-command (U4r's), each run as a pair | 7.2 |

- **The order inside the set:** the four recordings first (the pre-check needs SS P0 seed 4), then the pre-check, then
  the judged sets; the ticks-per-second measure (7.3) before any of them.
- **Before -17's judged runs:** carry 62 (h)'s outcome (r2's measured-phase sum of `tick.s.snap_incomplete` on each
  short L1 P0 run): 0 on every one puts its setup check, with its bite, into -17's L1 P0 part first (U4R-Q2 (b));
  not 0 sends the cause to the advisor first.

### 7.2 The -30 pre-check (plan :772-796; the advisor's conditions 2 and 3, :90-111; carry 40)
A recorded npt-lab test, not a judged id (Q10, advisor :208). Its parts as the plan gives them, and, where the plan is
silent, what you state:
- **Suite and legs:** suite `precheck-30`; one leg over Loopback (layer `loopback`: README :36), one over L1 P0
  (layer `L1`). Replay clients send the SS P0 recording's commands (§4.15) to the room on both legs, re-encoded as
  `Input` (stamped ticks, view times, sequences and press counters from the recording; the ack from what that leg's
  replay client decodes live), and the room computes its own Rewinds (§4.15 :487-491). Option B (`cmd_timing` B).
- **The depth (changed at the head: R17).** Option B has no depth setting: the room measures how far ahead of the tick
  last run the newest buffered command was (`ClientSlot::buffer_depth_ticks`, room.rs:289-292; input.rs:28-29,
  :142-145), and holds at most `CMD_CATCHUP` = 8 commands per client (input.rs:56-58, :118-120; limits.rs:17). So "the
  input buffer deep enough for P0" (plan :774-775) is set by how far ahead the replay clients send (the pacing rule
  below). This brief's reading (plan silent at the head): write the pacing lead in ticks as the header's
  `buffer_depth_ticks_count` on both legs (marks.json:6557, read as "max"; SERIES.md:240: "-30's input buffer depth set
  for P0"; condition 3, advisor :111), log the depth the room measured per client per tick beside it, and report any
  tick after each client's first commands where the two differ. Both legs run one room configuration (the same lead, the
  same HitHistory span, the same seed, the same tick base), recorded in both headers; only the leg's link and the
  wallclock bite's leg differ (this line is this brief's: the comparison implies it).
- **The replay clients' encoder (changed at the head: R16; plan silent).** `ClientEndpoint::input` stamps each command's
  tick as its newest tick plus its lead and its sequence from its own counter (client/src/lib.rs:390-400), so it cannot
  send the recording's ticks and sequences. This brief's reading: each replay client encodes its own `Input` with
  npt-proto (`npt_proto::encode` of `Message::Input`: proto/src/lib.rs:36-39; `Input`'s commands newest first, at most
  `Limits::REDUNDANCY_CAP`: proto/src/input.rs:42-50) carrying the recorded commands and the acknowledgement of the
  newest snapshot it decoded; for decoding it may run a `ClientEndpoint` fed by `input(None, received, now)` (whose own
  `Input` then carries the acknowledgement only: client/src/lib.rs:415-430) or decode through npt-proto itself. Either
  way its acknowledgement names only snapshots the room sent it (one that names a snapshot never sent refuses that
  datagram's commands: room.rs:783-786), and it stays inside the Input bucket (8 datagrams, refilled at 2 × TICK_HZ a
  second: pump.rs:67-68, :409-414). State the route; one that needs an npt-client change stops (2.6).
- **Pacing and the tick base (plan silent: state them, and show them by test r6).** The recording holds, per tick, the
  commands the room ran (`tick_input()`, room.rs:579; on a tick where option B repeated a client's last command, that
  command again, with its sequence) and no send instants (§4.15 :479-482; replay.rs:7-15); option B runs exactly one
  command per client per tick, at its stamped tick, and drops late ones (ADR 0001:1160-1162); a pair counts only if
  every command ran at the tick stamped on it with the late counter at 0 (ADR 0001:1312-1315). The replay clients have
  no `ClockSync` and no tick-lead controller (both U4f's: plan :806). State:
  (a) the room's tick base on both legs: the recording's `first_tick` (replay.rs:47-48), the `World` built with it
  (`World::new(scene, seed, first_tick)`: world.rs:701; the room's first tick is the one after the World's hit volumes'
  tick: room.rs:424-426, :442), recorded in both legs' headers beside the lead;
  (b) the pacing rule: when a replay client sends the command stamped T (for example at the SimClock instant at which
  tick T − L is due, L the lead, 1 ≤ L ≤ 8, the same instant on both legs), why it reaches the room before tick T runs
  on both legs at P0 and is still kept by the buffer, and how the `Input`'s redundancy covers it; a command the
  recording repeats (its sequence again) is sent once, since the room takes a second copy as a repeat (input.rs:84-89)
  and option B reruns the last command on a tick with none (input.rs:136-141), as in the recording;
  (c) whether admitting a client changes the World's hashed state (a join makes a full state due, room.rs:656; spawns
  are `TickInput`'s, §4.15 :481). Loopback delivers in the instant of the send (link/src/lib.rs:154-163, :185-190), L1
  only after it (l1/src/lib.rs:65-70, :1667-1681), so the two legs can admit a client at different instants; if an
  admit changes the hashed state, both legs must admit each client at the same tick, and the records' admit ticks
  (below) show it.
- **Each replay client sends its first `Input` only after it decodes `JoinAccepted`,** whose packet L1 acknowledges at
  once (U4c's immediate acks: l1/src/lib.rs:27-30; nothing the replay client sends; Loopback has none) (plan :785-787).
- **The comparer:** npt-lab's `HashCompare::of` (lab/src/lib.rs:43-61; U4d2's, which "takes two per-tick hash lists":
  plan :672-673) over each leg's list of `TickHash { recording, tick, hash }` (:8-17): the same recording name on both
  legs (the SS P0 recording's manifest name), the lists lined up tick for tick from the recording's first tick (it
  compares by position, names included: carry 83 (a)), each hash the room log's `h` for that tick, npt-sim's canonical
  hash of the room's `World` (2.1; seam.rs:551), the hash D-2 compares. Never the mirror hash per (client, tick) (§4.6
  :310-312): it follows each leg's baselines, acks and priority skips, so a correct pair could read FAIL on it.
  `Mismatch` names the first differing index, a list that ended early showing `None` there (:29-38); `Empty` is never
  "equal". Say where the printed comparison line comes from (`compare_text` is private to npt-lab's binary: hash.rs:597;
  carry 83 (a)), and change TickHash's struct doc (lib.rs:8) to its field doc's words (carry 83 (a)).
  `tick.v.late_count` must sum to 0 on both legs (condition 2, advisor :93-94).
- **The setup condition** (condition 3, advisor :107-108; carry 40): 0 view-time replacements and 0 clipped shots on
  both legs, each replacement and clip logged per shot; a leg with any, or with `late_count` above 0, makes the pair
  VOID, reported, never a pass or a fail (plan :778-780). **At U4r the room has no tolerance and no cap**
  (rewind.rs:1-2: "The tolerance and the cap are U4h's"; R33, D10): write the tolerance's replacements and the cap's
  clips per leg as "not built: 0 by construction" (8.4), and never cite them as the setup holding. **But the room's
  count of commands that `rewind_at` gives no Rewind**, because the ring does not hold their claimed view time's tick
  (`ClientCounters::rewinds_refused`, room.rs:133-135, firing commands only; per tick `ClientSlot::rewinds_refused`, the
  commands' sequences, :264, :860-864), **counts as a replacement in that setup condition: a leg with any is VOID**
  (carry 40's last sentence; U4E-Q5's condition, wave1 :470-471). Log it per leg and per command. (Rev 2 logged it as a
  report beyond the plan; carry 40 now makes it a VOID cause.) The ring holds 2 s at U4r (HISTORY_MS, room.rs:33).
- **The RTT rule** (plan :781-790). The runner logs per client, on each leg:
  (1) the first-sample instant: the SimClock instant of the first pump call (RQ8's step 2) after which that session's
  `Decoder::rtt_ns(link)` (pump.rs:295-303) is `Some`: on L1 the link's estimate is `None` until its first sample
  (l1/src/lib.rs:64, :1748-1750; link/src/lib.rs:79-80: "as of the last call that took `now_ns`; `None` before its
  first sample"); never the client end's (each L1 link keeps one estimate per end: `rtt[send_dir(self.end)]`, :1749;
  the client end samples from its `Join`'s ack, which always comes before its first command, so the rule would pass
  vacuously);
  (2) the first command's receipt instant: the pump call that decoded the `Input` carrying it (this brief's addition,
  one field; it lets the re-run after U4h compare whichever instant U4h's estimate reads the RTT at: 8.4);
  (3) the tick of its first command run, and that tick's due instant (base + ⌊k × 10⁹ / TICK_HZ⌋: 2.1).
  A client whose first command's run instant is strictly earlier than its first-sample instant, or that has no
  first-sample instant on the L1 leg when its first command runs, makes that leg VOID. Equal instants are not VOID: at
  one instant the pump (step 2) comes before the tick (step 3). On the Loopback leg `rtt_ns` is always Some(0)
  (link/src/lib.rs:203-205): its column reads "Some(0) from the start", never an empty gap, and the rule binds the L1
  leg. **The leg glue** builds every session's `Decoder` with `RttSource::Link` on both legs (pump.rs:183-190,
  :224-231), so the Loopback leg's sessions take the link's Some(0), never the room's ping samples (§4.9 :352-355 gives
  those to `ws` sessions; the pump sends pings on every joined session, pump.rs:346-363, but only a `Pings` session
  reads them). **At P0 the VOID cannot fire on a correct build** (D12): the server end's first sample comes from
  `JoinAccepted`'s packet (the pump sends nothing to a session before it is joined: pump.rs:346), it is 0 (arrival
  minus send, no delay at P0: l1/src/lib.rs:1433-1435), and it is applied by the first net call at or after its send
  instant (`advance`, :947-959), so it is held before any later read at that instant (U4R-Q10).
- **The result:** VOID if either leg breaks the setup condition or the RTT rule; else PASS if the comparer names no
  differing tick, FAIL if it names one. Each written to runs.txt with its reason. Beside it, runs.txt lists per leg the
  clients admitted (48), each client's admit tick, and any client dropped or kicked (Kick(overflow): §4.7 :331-334;
  `ClientSlot::kicked`), so a FAIL caused by a client dropped on one leg is read as that.
- **The bites** (plan :791-792; condition 2, advisor :95; marks.json:6563-6567; -30's wallclock: "a wall-clock read on
  one transport's command path"):
  - **wallclock** (a wall-clock read in the pump for one leg's sessions, bites build only: its site is U4R-Q6; no
    PM-0001-30 id is planted anywhere at the head: D6). The read must change a sim input on that leg on every run,
    whatever R is (for example one field of each command the pump passes on for that leg, set from the wall-clock
    value), never through the RTT (nothing in the room reads it at U4r: 8.2 item 2), through lateness (that makes the
    pair VOID, which shows nothing) or through a rate bucket's refill (whether a bucket starves depends on the machine's
    ticks per second, 7.3). It is shown only on a pair that is not VOID, the comparer naming a first differing tick;
    a wallclock pair that comes out VOID shows nothing, and one that comes out PASS is a missed bite, reported as such,
    never re-planted into a fail without saying so.
  - **late-command** (changed at the head: R17). One replay client on one leg holds the command stamped T **and every
    later command** out of every `Input` until tick T has run on that leg (option B then reruns its last command at T:
    input.rs:136-141), then sends T first, with the later ones after it in the same `Input` (the room offers an
    `Input`'s commands oldest first: room.rs:791-792 over the newest-first list, proto/src/input.rs:47-49). So T arrives
    as the newest sequence received, passes the forward compare and is counted late once (input.rs:84-89, then
    :111-117; room.rs:797), and the later commands reach the room in time for their own ticks. Delayed, never dropped (a
    command never sent is a miss, which the late counter may not count). A later command sent before T would make T a
    repeat, never late (input.rs:84-89), and the pair a FAIL that says nothing of the late rule: a mis-built bite,
    reported as such. It is shown only when the pair comes out VOID by `late_count` alone, with the setup counts and the
    RTT rule holding (the judge's rule for an expect-invalid bite, eval.rs:1412: "shown only when its run is INVALID by
    that one rule and holds every other"). The bite run's records state the delayed command's tick and that leg's
    `late_count` (expected exactly 1; any other value reported with its reason).
- **The runs at U4r:** three pairs (clean, wallclock, late-command), six runs, all on the one recording (seed 4). The
  plan does not fix the count. The advisor's "6 runs" is said of the re-run after U4h (advisor :346) and has two
  readings: U4R-Q12. U4R-Q10 (a') cannot fire at P0 on this code (D12), so it adds no pair at U4r.
- **Records** (plan :793-795; condition 2, advisor :96-98): the headers list no mark, so the judge never reads these
  runs; results/u4r-precheck-30/<commit>/runs.txt and the comparer's output are committed; the advisor reads them in
  U4r's review, as for U3d's self-test (advisor-u3d.md, U3D-JUDGE); nothing goes into marks.json.
- **-30 stays NOT-RUN until U6** (plan :796): never reported as -30's part, never in U6's folder.

### 7.3 The run set's wall time: what U4r measures first, and the estimate
**Measured first** (plan :764-765), on the release build, before any set starts: the runner's ticks per second R,
per scene and profile the set uses, each a short timed run (for example 1,800 ticks, 60 s simulated, 48 clients, seeds
1-3), wall time read by the run script around the npt-lab process (2.3; npt-lab itself reads no clock:
lab/clippy.toml:6-16), R = ticks run ÷ wall seconds, the slowest of the three seeds kept. The builder measures it in its
worktree (return list item 8); the main session measures it again in the main folder before the set starts (D4), and
states the set's total before starting it.

**Simulated time of the set** (30 Hz), from section 7.1 and its assumptions:

| Item | Runs × length | Simulated s |
|---|---|---|
| -17 measured, also -57's (P0, P2, P3, B1 × seeds 1-3) | 12 × 600 s | 7,200 |
| -17's bites (tick16, client-rounding) | 2 × 600 s | 1,200 |
| -57's bites (pos-4cm, yaw-12bit, pitch-8bit) | 3 × 600 s | 1,800 |
| -18 measured (B1 × seeds 1-3), if not shared with -17's B1 runs (U4R-Q8) | 3 × 600 s | 1,800 |
| -18's bite (no-fallback) | 1 × 600 s | 600 |
| -30 pre-check (three pairs) | 6 × 300 s | 1,800 |
| the four runner recordings | 4 × 300 s | 1,200 |
| **Total** | | **15,600 s = 4.33 h = 468,000 ticks** (13,800 s = 414,000 ticks if -18 shares -17's B1 runs) |

**Wall time** = 468,000 ticks ÷ R, plus the builds (two release builds of npt-lab and one of npt-judge, as c-u2:52-61:
not measured here) and the judge's passes:

| R (ticks per second) | As a multiple of real time | The set's runs |
|---|---|---|
| 300 | 10× | 1,560 s ≈ 26 min |
| 90 | 3× | 5,200 s ≈ 1.4 h |
| 30 | 1× | 15,600 s ≈ 4.3 h |
| 10 | 0.33× | 46,800 s ≈ 13.0 h |

Assumptions, each to be replaced by the measured value: runs one at a time (c-u2 starts one process at a time:
c-u2:65-84; §7.2 :927's "one set at a time"); every bite run at its part's full length; each runner recording and the
pre-check's legs 9,000 ticks (5 min: §4.15 :483-484); every run costing what SS with 48 clients costs (the ES and TC
recordings likely less); no run repeated; -58's runs gone to U4g (C2; rev 2 counted 144 s for them). The tests' own runs
(the contract test's short runs, the class audit, `replay`'s test, the twin test) run in the build's and the merge's
test steps, not in this set. Which R applies cannot be known before a run: the estimate is the arithmetic, never the
answer.

### 7.4 The evidence files
- **results/u4r/<commit>/verdict.json and runs.txt** (plan :800): the commit, the binaries' SHA-256s (both npt-lab
  builds and npt-judge, as c-u2:93-99), the raw logs' path, the judge's line, and one line per run; each run's line
  also gives its ticks and wall time (7.3). `<commit>` is the folder name c-u2 uses: the commit's first 12 hex digits
  (c-u2:46-49).
- **results/u4r-precheck-30/<commit>/runs.txt and the comparer's output** (plan :793-795): per pair, both legs' setup
  counts (the tolerance's and the cap's as "not built: 0 by construction", `rewinds_refused` as counted), `late_count`
  sums, leads and measured depths, tick bases, the RTT rule's per-client instants (first sample, first receipt, first
  run), the clients admitted with each one's admit tick and any drop or kick, and the result with its reason; for the
  late-command pair, the delayed command's tick and that leg's `late_count`; no verdict.json (the judge never reads
  these runs).
- **The recordings' SHA-256s** (plan :801) in runs.txt and in the committed recordings manifest (carry 82; §4.15
  :502-504), and U4d2's determinism step green on them (in check.mjs if the step stays within 5 minutes, else through
  ops/runs/det-u4.mjs at every merge that touches sim, proto, room or hash-wasm, on the advisor's ok: plan :800-802;
  section 5 item 4).
- **Raw logs** under .gate/netcode-proof-test/runs/<set>-<short>/ (c-u2:9-10, :48), never committed (plan §7.2 :928).
- **The sizes for fixes.rs** (§2 :150) as series in -17's (and -18's) B1 logs, with SERIES.md rows (U4R-Q2 (a)).

---

## 8. O1 (carry 40): what the -30 pre-check can and cannot show at U4r, and the re-run after U4h

### 8.1 The statement carry 40 asks for
At U4r the room has neither the view-time tolerance nor the cap: both land in U4h. The plan's own lines, at the head:
U4e builds "`rewind_at` (the integer half; the tolerance and cap rules land in U4h)" (plan :719-720); U4h builds
"`HitHistory`'s sizing and `rewind_at`'s rules (§4.9); the view-time estimate with the pump's RTT; the switchable cap;
the per-shot log of its parts" (plan :835-836). So the -30 pre-check's setup condition (0 view-time replacements and 0
clipped shots on both legs: the advisor's condition 3, advisor :107-108) holds by construction at U4r for the tolerance
and the cap, and its RTT argument (plan :781-790) cannot be tested there. The one way a replacement can still occur at
U4r is the room's count of commands that `rewind_at` gives no Rewind, which carry 40's last sentence makes a
replacement in that setup condition (the advisor's U4d close ruling, F3, close :321-324: "The VOID sentence then reads
as the one way a replacement can still occur at U4r, with no contradiction").

**Checked in the merged code** (R33, D10): rewind.rs:1-2 and :13-33 (`rewind_at` returns `None` for a view time the ring
does not hold, and nothing else); room.rs:133-135 and :860-864 (the count); no reader of an RTT in crates/room/src or
crates/client/src; in crates/server/src only the pump's `RttSource` and the getter `Decoder::rtt_ns`
(pump.rs:183-190, :295-303), which the pump itself never calls. So the statement holds against the code, not only the
plan's split of work.

**The cites, checked.** O1 cites "u4-build-plan.md:678-679, :787" (advisor :344), and the conditions check it comes from
cites "U4e :678-679" and "U4h :787" (u4-conditions-check.md:42-45). At the head those lines hold other text (:678-679
are §5 U4d2's `wasm` step flags; :787 is §5 U4r's "internal events come before application sends: L-c6" line), as at
df8975b they did too (rev 2's 8.1). The words O1 relies on stand at :719-720 and :835-836, as quoted above. The claim
holds; only the line numbers are those of an earlier text of the plan. Carry 40 itself cites by section ("§5 U4e's
`rewind_at` and §5 U4h"), so its words need no change. Also: advisor :343 names its source as
"u4-conditions-check.md:39-42"; O1's text there is :42-45, the lines carry 40 cites.

### 8.2 What follows at U4r (this brief's reading of plan :772-796)
1. **The setup condition can fail at U4r only through the no-Rewind count.** No code in the room replaces a view time
   by an estimate or clips a shot before U4h, so those two counts are 0 whatever the build does, and a check that
   cannot fail is evidence of nothing (the advisor's own words in the NF-D4 ruling, advisor :353). The records say "not
   built: 0 by construction at U4r" for them (7.2, 8.4), never "the setup held". `rewinds_refused` is built and counted:
   with `late_count` 0 both legs run each command at its stamped tick against rings built from equal states, so in a
   correct pair it is equal on both legs; any value above 0 makes the leg VOID (carry 40), whatever the other leg shows.
2. **The RTT argument holds trivially and is untested at U4r.** The RTT reaches the room only through the tolerance's
   accept-or-replace and the cap's clip (plan :781-785; §4.9 :351-356), and the view-time estimate that reads the pump's
   RTT is U4h's too (plan :835). With none of them built, nothing in the room reads the RTT (R33), so the legs cannot
   decide differently through it, whatever the L1 leg's RTT is: `None` before its first sample at the head
   (l1/src/lib.rs:64, :1748-1750; the plan's "L1 starts at U4c's 333 ms", :782, is the probe timeout's starting state,
   :118 and :633-635, not what `rtt_ns` returns), Some(0) on Loopback (link/src/lib.rs:203-205). No defect on the RTT's
   path can show.
3. **What the pre-check does test at U4r:** that the two legs give equal per-tick World hashes (7.2's comparer) when the
   same commands run at the same ticks: carry 21's canonical order, which "-30's comparer relies on" (carries :148-153);
   option B's buffer; and the sim's inputs (which command runs at each tick, and its Rewind) being independent of the
   link once `late_count` is 0 and no Rewind is refused. The encoder's baseline, the window's remainder, the priority
   skips, event resends and resyncs do follow each leg's acks (§4.5 :293-303; §4.7 :331-334), but they stay out of the
   World hash, and a client dropped or kicked on one leg is listed beside the result (7.2). It also tests `late_count`
   (condition 2); the wallclock bite, a mismatch on a valid pair, which needs no tolerance and no cap; the late-command
   bite, a VOID by `late_count`, which needs neither; and the no-Rewind count, a VOID cause. And the RTT rule's premise:
   each client's first-sample instant, first-command receipt instant and first-command tick are logged at U4r. The
   "first command before first sample" VOID cannot fire on a correct build at P0 (7.2; D12). A breach changes no hash
   at U4r. How the rule's code is shown to fire: U4R-Q10.
4. **So at U4r the pre-check is evidence for the comparer and the command path, not for the RTT argument nor for the
   tolerance-and-cap half of the setup condition.** U4r's report and the advisor's reading of results/u4r-precheck-30/
   say so.

### 8.3 The proposal: re-run the pre-check pair after U4h merges (the advisor's lean, advisor :346-347)
- **Why:** U4h builds exactly what the argument is about: the estimate with the pump's RTT, the tolerance, the
  switchable cap and the per-shot log of their parts (plan :835-836). Only then can a replacement by the estimate or a
  clip happen, so that half of the setup condition can fail, and only then can the L1 leg's RTT before its first sample
  reach the room, so the argument means something: in a correct build each L1 client's first sample comes before its
  first command (the ordering log shows it), its RTT is about 0 at P0, both legs accept the same view times, and the
  pair is PASS with 0 replacements. What U4h's estimate does with an L1 session whose `rtt_ns` is still `None` is U4h's
  to state (its brief: U4R-Q12's carry premises).
- **What it is (this brief's reading of the advisor's "6 runs"; the other reading and the choice: U4R-Q12):** the same
  three pairs as at U4r (7.2: the clean pair, the wallclock pair and the late-command pair), 6 runs, on the merged U4h
  build, by the same script, its results in results/u4h-precheck-30/<commit>/ (a folder name this brief proposes:
  U4R-Q12), read by the advisor in U4h's review as condition 2 asks for U4r's; -30 stays NOT-RUN until U6.
- **The recording:** first replay U4r's SS P0 seed 4 recording on U4h's room and count the replacements; this replay
  is the re-run's clean pair. U4r's clients have no `Interp` (U4g, plan :822) and no `ClockSync` (U4f, :806), so the
  view times they stamp may not be what U4h's estimate expects, and §4.15 :498-500's reason for a P0 recording ("a
  recording made at P0 keeps them inside the tolerance") was written for clients that interpolate. If any view time is
  replaced on either leg, that clean pair is VOID: re-record SS P0 seed 4 with `scenario --record` on the U4h build
  (one more run, its SHA-256 beside the old one's), then run the six. Needs a run (day), after U4h merges: D7.
- **Its cost, both branches:** 6 runs of the recording's length (about 6 × 300 s = 1,800 s simulated) if the
  replay-first clean pair shows no replacement; 9 runs otherwise (the VOID pair, the re-recording, then the six: about
  2,700 s). At 7.3's R values: 1,800 s takes about 3 min at R = 300 and 30 min at R = 30; 2,700 s about 4.5 min and 45
  min. U4R-Q12's reading (ii) adds to these (U4R-Q12 gives its totals).
- **Why not the alternatives:** (b) no re-run leaves the setup condition's tolerance-and-cap half and the RTT argument
  untested until U6's -30 C part on the real transports (plan :894-895), where a VOID or a mismatch would mix the
  transports' effects with the room's, which is what the pre-check exists to separate first (conditions 2 and 3); (c)
  moving U4r's pre-check to after U4h drops what it already tests at U4r (8.2 item 3), and condition 2 records it as
  U4r's (advisor :97).

### 8.4 What U4r builds now so the re-run reads real counts (this brief's proposal)
- The pre-check script reads the tolerance's replacement count and the cap's clip count by name from the room's logs.
  At U4r no such series exists (U4h's per-shot log is not built: plan :836), so the script writes "not built: 0 by
  construction" for both, never a bare 0; after U4h it reads U4h's series, and it gives no result (VOID, with the
  reason) if a room log lacks them, the same logic as §4.12's validity inputs (:413-414) and the judge's gap rule
  (eval.rs:314-327). `rewinds_refused` is read by name from U4r on (7.2). Your report names the switch and its
  test.
- The per-client first-sample instant, the first command's receipt instant and its run tick (7.2) are written at U4r,
  so the re-run needs no new log whichever instant U4h's estimate reads the RTT at (the receipt, if the estimate is
  taken in the pump; the run, if `rewind_at` reads it at the command's tick).
- U4h's brief then carries the re-run and its premises (a line the main session adds to u4s-carries.md's §U4h,
  carries :548-560, or to U4h's brief: U4R-Q12).

---

## 9. Recheck after wave 1 and wave 2: the table, walked at the head

Every place rev 2 named an item U4c, U4d2, U4e or U4j builds (rows R1-R29, R33 and R34), plus three items of U4d, U4b
and U4a it leans on (R30-R32), read at the head fa8e335 (spikes/ = cc8595d), and rows R35-R44 for what merged since
df8975b that this brief relies on and rev 2's table missed. "Holds" means the name, the shape and the behaviour rev 2
assumed are in the code; "changed" says what the code has and what this brief now says. Re-check at launch only if
spikes/ at `<base>` differs from cc8595d (section 12 item 8).

| Row | Item | Built by | Its use in U4r | At the head |
|---|---|---|---|---|
| R1 | `L1Net::next_event_ns`, C1' event-driven (plan :570-571) | U4c | how `L1Scenario` reaches each instant (2.1) | **holds:** `next_event_ns(&self) -> Option<u64>` (l1/src/lib.rs:1876-1882); L-c5's test `lc5_polling_every_millisecond_or_only_at_next_event_ns_gives_the_same_log_and_arrivals` (l1/src/tests.rs:1043) |
| R2 | the RFC 9002 starting state: 333 ms before the first sample (plan :572-574) | U4c | the RTT rule (7.2); O1 (8.2) | **changed:** 333 ms is kInitialRtt for the probe timeout only (:42-44, :118, `smoothed_ns` :633-635); `rtt_ns` reads `smoothed`, `None` until the first sample (:64, :1748-1750). 7.2 (1), 8.2 item 2 and 8.3 say so (C9); D2 |
| R3 | immediate acks (plan :575); L-c6 (:588) | U4c | the RTT argument's premise (7.2; plan :785-787) | **holds:** each packet acknowledged at its arrival (:27-30; `send_ack` :1418-1465); L-c6 `lc6_at_one_instant_internal_events_come_before_application_sends` (tests.rs:1432); L-c7 (:1561) |
| R4 | C2's sampling rules (L-c7, plan :589-590; §2 :145) | U4c | the per-client first-sample instant (7.2); U4R-Q10 | **changed (answered):** no accessor gives a first-sample instant; the lab reads `Decoder::rtt_ns(link)` (pump.rs:295-303) after each pump call, `Some` meaning a sample is held. At P0 `JoinAccepted`'s ack is an event at its send instant (delay 0: :1433-1435), applied by the first net call at or after it (`advance`, :947-959). 7.2 (1) and U4R-Q10's facts (C23); D12 |
| R5 | the fate log v2, tag `npt-l1 fate log v2` (plan :579-580) | U4c | r1's fate-log hashes (2.4) | **holds:** the tag (:330; `for_each_canonical` :351); U2's probe writes `fate_log_sha256` (probe.rs:47; main.rs:120) and, since U4d2, the v1 datagram hash (:121-123) |
| R6 | `L1Net::with_bite` and U4c's ids (plan :595-598) | U4c | the `--bite` dispatch (2.1) | **holds:** `with_bite(seed, bite) -> Option<L1Net>` (:1828-1840); `L1Net::BITES`, 4 ids (:1823-1824; BITE_IDS :752-758); the model's own bites are unit-test only (:764-773) |
| R7 | `Profile` P0, P2, P3 and `Profile::b1(burst_starts_ns)` | U2, kept by U4c | every run's profile; -18's bursts (7.1) | **holds,** lines moved: p0 :178-180, p2 :186-188, p3 :190-192, b1 :198-202 (1 s of total loss both ways: :23); every profile has the reorder step (:166-175), which finds nothing to swap at P0 (:20-22) |
| R8 | `HashCompare`, two per-tick hash lists (plan :672-673) | U4d2 | the pre-check's comparer (7.2) | **changed:** `HashCompare::of(&[TickHash], &[TickHash])`, `TickHash { recording, tick, hash }` (lab/src/lib.rs:8-61): it compares by position, names included; a shorter list gives `Mismatch` with `None` at the first missing index; two empty lists give `Empty`. 7.2's comparer, carry 83 (a) |
| R9 | npt-lab's `hash` subcommand (plan :670-673) | U4d2 | `scenario` and `replay` beside it (2.1, 2.7) | **changed:** main.rs dispatches `hash` and `probe` (:195-235); `hash`'s modes: the two native legs, `compare`, `vectors`, `bites` (hash.rs:111-139); no clock read (D8; lab/clippy.toml bans it). No name clash. U4R-Q3 answered (C3) |
| R10 | check.mjs's `wasm` step, its 5-minute budget and printed wall time (plan :677-680) | U4d2 | the recordings' determinism route (7.4; U4R-Q2) | **changed:** the step exists (`wasmStep`, check.mjs:2254 on), each call but its builds limited to 300 s on its own (WASM_CALL_LIMIT_MS :2212, :2258-2266; carry 84 makes it one deadline); 33.1 s warm and 61.6 s cold on the three synthetic recordings (D3). Section 5 item 4 |
| R11 | crates/lab/Cargo.toml as U4d2 left it (plan :873-880) | U4d2 | U4R-Q1 | **holds:** lab's and bots' manifests unchanged since df8975b (D5): npt-lab names no npt-bots (lab/Cargo.toml:23-34), npt-bots no npt-sim (bots/Cargo.toml:20-25). New fact for U4R-Q1: npt-bots names npt-net (:24) (C26) |
| R12 | check.mjs's steps (10 at df8975b) | U4d2 | section 5's "every step passing" | **changed:** 11 steps, `wasm` after `deps` (check.mjs:2468-2503); the last line :2548 (C17) |
| R13 | `Room<S: SimCore>`, `Room::with_bite` (§4.1 :194; §4.11 :400) | U4e | `L1Scenario`; the dispatch (2.1) | **holds:** `Room::new(sim, scene, greybox, tokens, n, tick_hz)` (room.rs:431-478); `with_bite(&str) -> Option<Room>`, its own ids or npt-proto's (:510-518); `npt_room::BITES`, 7 ids (room/src/lib.rs:37-46) |
| R14 | `Roster`, tokens passed in by the caller (plan :716) | U4e | the lab's tokens (2.1) | **holds:** `Vec<[u8; 16]>` into `Room::new` (room.rs:431-438, :447) |
| R15 | `Decoder::pump` with the per-client RTT (plan :722-723; §4.1 :195) | U4e | step 2 of the order; the RTT rule; the wallclock bite's site (U4R-Q6) | **changed:** `pump(&mut self, link: &mut dyn Link, now_ns, tick: u32, out: &mut Vec<(Message, u64)>)` (pump.rs:331-447) reads no RTT; the getter `rtt_ns(link)` (:295-303) by `RttSource` (`Link` or `Pings`, :183-190), which the caller sets at `Decoder::new` (:224-247). 7.2's leg glue: `RttSource::Link` on both legs (C10) |
| R16 | `ClientEndpoint` and `StepOut` (plan :726; plan.md:609) | U4e | step 5; the client logs; -57's pairing; -18's record | **changed:** `new(join, tick_hz)` (client/src/lib.rs:253), `with_lead`, `with_starts`, `with_bite` (:291-320), `input(command, received, now)` stamping the tick (newest + lead) and the sequence itself (:379-409, :390-400), `step() -> StepOut` (:415-456; `StepOut` :189-202: hashes, lost, wraps); no decoded record or held float is exposed. 2.1's -57 route, 7.2's replay-client encoder (C12) |
| R17 | `InputBuffer` option B with `buffer_depth_ticks` (plan :719) | U4e | `cmd_timing` B; the pre-check's depth, pacing and late-command bite (7.2) | **changed:** no depth setting; `buffer_depth_ticks` is measured (input.rs:28-29, :60-63, :142-145; room.rs:289-292); at most 8 buffered (:56-58, :118-120); late when stamped for a tick already run or already buffered (:111-117) or dropped at the take (:131-134), the forward compare first (:84-89), so a delayed command that is not the newest sequence is a repeat. 7.2's depth and late-command bite (C11) |
| R18 | the encoder's exposed values (§4.5 :299-300; §4.6 :307-328; U4E-Q7) | U4e; `EntityRecord::raw` U4d's (carry 9) | 2.1's logs; -57's pairing; -18's record | **changed:** `ClientSlot`'s getters (room.rs:195-297: `server_hash` :206, `burst_eviction` :212, `baseline_age` :217, `groups` :222, `bytes` :227, `budget` :232, `last_acked` :238, `own_section` :244, `overflow_resync` :253, `wraps` :258, `rewinds_refused` :264, `queue` :278, `headroom` :285, `buffer_depth_ticks` :290, `kicked` :295); the fields a snapshot carried are not exposed: the lab decodes the bytes (2.1); `raw` through `room.sim().entity_records()` (seam.rs:554) |
| R19 | `rewind_at`'s answer for a view time the ring does not hold, and its counter (U4E-Q5) | U4e | the pre-check's setup condition (7.2) | **changed:** `rewind_at -> Option<Rewind>` (rewind.rs:18-33), counted as `ClientCounters::rewinds_refused` (room.rs:133-135, :860-864), firing commands only; by carry 40 a replacement in the setup condition, so a VOID cause, no longer only a report (C6) |
| R20 | the bites U4e planted, and its -18 end-to-end finding (condition 6) | U4e (tick16 with U4a) | the judged runs' bites (7.1); r4 | **holds, its open part settled:** npt-room's 7 ids and npt-client's 3 (D6); tick16 stays in `npt_proto::BITES` (U4E-Q4); no-trailing and no-channel are npt-proto's, through the pump; condition 6 not triggered: -18's L1 part counts at U4r as planned (e-r1 R4, NC-L1, :224-232) (C7) |
| R21 | marks.json -18's split, `burst_heal` and `burst_return` (plan :651; §2 :149) | U4j | -18's record (2.1, 7.1) | **changed:** parts L1 B1 (SS, 48), L2 B1 wt, L2 B1 ws (marks.json:4262 on); series `event.burst_heal.after_link_ns` and `.burst` (SERIES.md:134, :323); `burst_return`'s series serve the L2 WebSocket part (marks.json:4402-4418); the fresh-delta reading is in -18's notes (:4442), timed "from the link's return". 2.1's -18 record |
| R22 | §4.12's table in marks.json (plan :418-424, :651) | U4j | each header's scene and clients; the precondition (7.1) | **holds:** -17's four L1 parts and -18's L1 B1 select scene SS and clients 48, each with the part validity `pre:PM-0001-2`; -20's two L1 P3 parts and -21's parts carry it; -22 does not (its notes only) |
| R23 | npt-log's `hs`, `admit` events, footers, `tick.v.late_count`, one header per process value (plan :646, :652-654; §4.12 :410-415) | U4j, on U2's npt-log | the per-process logs (2.1) | **holds:** README §1 (:8-86: `h` :15, footer :18, `hs` :44-79, `admit` :80); `TickRecord::set_hash` and `set_hashes` (log/src/lib.rs:144, :158), `Writer` (:338-412); `tick.v.late_count` (SERIES.md:365) |
| R24 | the figures `decoded_hash_mismatch_count`, `decoded_hash_compared_count`, `twin_hash_mismatch_count` (plan :646-648) | U4j | -17's checks (7.1); r1 | **holds:** figures.rs:542; SERIES.md:266-267, :295; `cross_arch_mismatch_count` (:265) in -17's report |
| R25 | README §1's layer value `loopback` (plan :652-654) | U4j | the pre-check's Loopback leg (7.2) | **holds:** README :36 |
| R26 | SERIES.md's rows for the names U4r writes (plan :658-659) | U4j | r2 (b) | **changed:** rows exist for -57's error series (:66-73), `counter_wrap` (:150), `burst_heal` (:134, :323), `burst_eviction` (:321), `snap_incomplete` (:320), `late_count` (:365), the `hs` rows (:317-319), `snap_refused` (:218), `buffer_depth_ticks_count` (:240), `tick_start` (:252), `ticks_below_wrap` (:253), `suite` (:376), `stand_ins` (:374), `cmd_timing` (:375), `burst_starts_ns` (:403), `snap_datagram_bytes` and `snap_wire_bytes` (:70, :72; npt-server's); none for the input payload and the Join, JoinAccepted and FullState sizes, carry 58's rate and high-water mark, the lead, the weapon and health params, the RTT rule's three instants, each client's admit tick, the FullState flag. U4R-Q2 (a) |
| R27 | -30's report series `header.params.buffer_depth_ticks_count` | marks.json (U4j edited it) | the pre-check's depth param (2.1, 7.2) | **holds:** marks.json:6557 (stat max); SERIES.md:240 |
| R28 | C4: a measured run from a bites build, or a bite run from a measured build, is INVALID (plan §2 :147) | U4j | the scripts' build per run (2.3, 7.1) | **holds:** eval.rs:539-570, its fixtures in check.mjs's counted test step |
| R29 | marks.json's bite ids for -17, -57, -58, -18 and -30 | marks.json | the ids the scripts pass to `--bite` (7.1) | **changed:** -17 :3991, :3995; -57 :4089-4097; -18 :4427 (and ws-budget-x2 :4431, the L2 ws part's); -30 :6563, :6567; -58's are U4g's (C2). Neither PM-0001-30 id is planted anywhere (D6) |
| R30 | `World`, `Scene`, `Recording` (its header; carry 14), `replay::Replayer` (plan :603-608) | U4d | the runner and its recordings (2.1); the room log's World hash and the pre-check's tick base (7.2) | **changed (answered):** `Recording { scene, seed, first_tick, span_ms, generator, ticks }` (replay.rs:41-54; the 15-word header :7-15), `Recording::encode` (:294); `World::new(scene, seed, first_tick)` (world.rs:701); the canonical hash is `SimCore::state_hash` (seam.rs:551); TC's scene `Scene::course` (§4.1 :186). 7.2's tick base, 2.1 (C13) |
| R31 | `Schedule::new(rate, catch-up)` (plan :560; carry 6) | U4b | step 3 of the order (2.1) | **holds:** `Schedule::new(tick_hz, catchup)` (schedule.rs:77), `with_bite` (:257) |
| R32 | `Quant`'s own bite choice for -57's three bites (carry 2) | U4a | -57's bite runs (7.1) | **holds:** `Quant::with_bite` (quant.rs:102-112), the ids in `npt_proto::BITES` (:35-37); the room's encoder codes with it (room.rs:510-518) |
| R33 | every reader of the per-client RTT, and every replace, clip or substitute path | U4e | carry 40's statement (8.1); the setup counts (7.2, 8.4) | **holds:** D10's greps find no RTT reader in crates/room/src or crates/client/src, and in crates/server/src only pump.rs's `RttSource` and getter; no tolerance, clip or estimate path in crates/room/src (rewind.rs:1-2, :13-17) |
| R34 | npt-client's `Resync` sender, and the event queue's cap (§4.7 :331-336; catalogue :90) | U4e | -18's record and its FullState flag (2.1, 7.1) | **changed:** npt-client sends no `Resync` (D11); the room's `Resync` handling and its limit (room.rs:713-741; `resyncs_limited` :107-110: NC2-M1, C-U6-16, U6's); the cap is 1,024, provisional (events.rs:40; `Room::event_cap`, room.rs:558), above §4.7's "about 400" (:335-336) (C8) |
| R35 (new) | the room log's per-tick World hash | U4d (the hash), U4e (the room) | r1; 7.2's comparer | `room.sim().state_hash()` after `Room::tick` (room.rs:563, :805-826; seam.rs:551), written as the tick record's `h` (README :15; log/src/lib.rs:144) |
| R36 (new) | the room's per-client counters and its tick input | U4e | 2.1's logs; `scenario --record` | `ClientCounters` (room.rs:65-136: `late`, `commands_run`, `commands_rerun`, `resyncs`, `overflow_resyncs`, `kicked`, `rewinds_refused`, `ack_refused` among them); `Room::tick_input` (:579) |
| R37 (new) | the pump's RTT source, its pings and the Input bucket | U4e | 7.2's leg glue and replay clients | `RttSource` (pump.rs:183-190), `Decoder::rtt_ns` (:295-303), pings on every joined session (:346-363), the bucket of 8 refilled at 2 × TICK_HZ a second (:67-68, :409-414) |
| R38 (new) | U4c's `rtt_ns` before the first sample | U4c | 7.2's RTT rule (1); 8.2 item 2 | `None` (l1/src/lib.rs:64, :1748-1750), as the trait's doc says (link/src/lib.rs:79-80); Loopback's Some(0) (:203-205) |
| R39 (new) | the client's counters | U4e | r2 (carry 77 (c)); 2.1's logs | `stale()` (client/src/lib.rs:359), `duplicates()` (:364), `refused()` (:353), `wrong_channel()` (:347), `newest_tick()` (:336), `last_run_command()` (:342) |
| R40 (new) | U4d2's `hash`, `HashCompare`, `TickHash` and the recordings manifest | U4d2 | 7.2's comparer; 2.1's recordings; carries 82 and 83 | lab/src/lib.rs:8-61; hash.rs:111-139, `compare_text` :597 (private), `manifest` :213, `regenerate_all` :292; crates/lab/recordings.json (three synthetic entries, generator 2); check.mjs's RECORDINGS_MANIFEST (:2099) |
| R41 (new) | U4j's log and judge rules | U4j | 2.1's logs; 7.2's bites; r2 | README §1's `hs` rules (:44-79), §4's validity rules (:247-316: complete :250, roster :258, late-zero :275); the gap rule (eval.rs:314-327, :1165-1168); the expect-invalid rule (:1412); the coverage figure (figures.rs:542) |
| R42 (new) | check.mjs's `wasm` and `deps` steps as U4r changes them, and loader.mjs's header | U4d2 | carries 83 to 85 (2.6, r10) | WASM_CALL_LIMIT_MS (:2212), `wasmStep` (:2254 on), WASM_BUILDS (:2120-2140), `depsStep` (:2062), DEPS_CHECKED (:2020), `buildInputProblems` (:1560); crates/hash-wasm/loader.mjs:13-14 |
| R43 (new) | the client's lead and starts, and the room's event start | U4e | carries 74 and 75 (2.1) | `with_lead` (client/src/lib.rs:291-295; default 1, :262), `with_starts` (:298-302), `Room::with_event_start` (room.rs:488) |
| R44 (new) | the session-ending surface | U4e | carry 76 (2.1; r8, r9) | `Room::join` returns `(slot, replaced)` (room.rs:632-637); no `Decoder::close` (pump.rs; e-r1 :206-209); `ClientSlot::kicked` (:295) |

Of R1-R34: 18 hold (R1, R3, R5, R6, R7, R11, R13, R14, R20, R22, R23, R24, R25, R27, R28, R31, R32, R33) and 16
changed (R2, R4, R8, R9, R10, R12, R15, R16, R17, R18, R19, R21, R26, R29, R30, R34); ten rows added (R35-R44).

---

## 10. Questions for the advisor

**Answered by rulings since rev 2** (the questions as rev 2 asked them stay in ed973c9, for the record):

| Question | Ruling | Where the brief now stands |
|---|---|---|
| U4R-Q3 (the clock bans for npt-lab and npt-bots) | (a): npt-lab's clippy.toml is U4d2's, merged with both type bans and the clock ban (U4D2-Q10; plan :674-675); npt-bots' with both bans is U4r's (A13, d2-ruling :588-589, :725). The run scripts time each npt-lab process; npt-lab and U4d2's `hash` read no clock (D8) | 2.2, 2.3, 7.3; section 4's lane |
| U4R-Q5 (-58 at U4r) | all of -58 is judged after U4g (W1-N2, wave1 :331-334; U4E-Q2 widened, :446-452; plan :828-831) | C2; 7.1 without -58; 2.1's starts used by r2 |
| U4R-Q2 (ii) and (iii) (the recordings manifest; the determinism step) | U4r's, by carries 82 to 85 (d2-r1 Blocks 4 and 5; d2-r2 Blocks 1 and 2); the step's route by measurement: in check.mjs while it stays within 5 minutes, else ops/runs/det-u4.mjs on the advisor's ok (plan :800-802), an over-300 s figure coming as U4r's det-u4.mjs question (m9 :117-119, :144-145) | 2.1, 2.6, 2.4 r10, section 5 item 4; U4R-Q2 below keeps (a) and (b) |

**Open** (each self-contained; only where the plan or a ruling leaves a choice):

**U4R-Q1 (the npt-lab to npt-bots dependency).** May npt-lab gain a dependency on npt-bots, and npt-bots one on
npt-sim, and who makes the edit?
- Why it matters: `L1Scenario`'s 48 clients run npt-bots' `Behaviour` (plan :756; §4.1 :197), and the bots read the
  button bits from npt-sim (carry 17); at the head neither edge exists (lab/Cargo.toml:23-34; bots/Cargo.toml:20-25;
  D5), and npt-proto re-exports no npt-sim item (proto/src/lib.rs:31-44). npt-bots' manifest names npt-net
  (bots/Cargo.toml:24), so npt-lab depending on npt-bots brings npt-net's tree (U3's L2 net, with its tokio and quinn
  dependencies) into npt-lab's build; npt-lab may depend on anything (lab/Cargo.toml:24's comment; plan.md:431-432:
  only nothing may depend on it). The Lanes give manifests to U4s and, in wave 2, to U4d2 (plan :850-855, :873-880);
  none to U4r. All are workspace crates already in the lock, so no package is fetched.
- Options: (a) U4r adds both edges and names them in its report as the only Cargo.lock change (§5's preamble, :509);
  (b) the main session adds them in its own commit on committed state before U4r's build starts (condition 10 (d),
  plan :887-888), and U4r's lane stays free of manifests; (c) `Behaviour` lives in npt-lab (against §4.1 :197).
- Recommendation: (b): every manifest change so far came from U4s or U4d2 under a ruling, a separate commit keeps the
  builder's diff to sources, and its own counted run shows what npt-net's tree costs npt-lab's build; (a) if the advisor
  prefers one step less.

**U4R-Q2 (the files outside U4r's paths: what is left).** Who writes (a) the SERIES.md rows for the names U4r writes
that have none (R26's list), and (b) carry 62 (h)'s one check in marks.json, if r2's evidence calls for it?
- Why it matters: r2 (b) fails on a written name with no row (§4.12 :413), and check.mjs's marks step reads
  marks.json and SERIES.md together; carry 62 (h) says: "marks/ is not U4r's lane unless its brief gives it this one
  check; else the main session adds it in its own commit"; marks/ is U4j's (Lanes :857-858) and no lane's during U4r.
  The check's bite ("a client that writes null and incomplete 1 for a refused group") is npt-lab's client writer,
  U4r's code either way.
- Options: (a1) U4r's lane gains marks/SERIES.md (new rows only), and (b1) marks.json's one check with its bite's
  entry; (a2) and (b2) the main session writes both in its own commit after U4r's build, before U4r's merge's counted
  run; (c) the rows in U4r (they follow the code that writes those names), and the check by the main session, after
  r2's evidence (the carry's "else" branch), with the bite's site built by U4r.
- Recommendation: (c): r2 (b) cannot pass without the rows, while carry 62 (h) itself puts the evidence before the
  check (j-ruling R2.4, :438-449: "evidence first"), as A14's row went into U4e's merge commit (e-r1 :245-247).

**U4R-Q4 (the contract test's scope at U4r).** Which suites and parts does r2 cover at U4r?
- Why it matters: condition 5 (advisor :120-121) and plan :759-760 ask PM-0001-2 to pass on "each of -21's eight
  suites" and on -20's P3, and §4.12 :412 asks that "every check of every U4 L1 part resolves"; but -21's option-A parts
  and -20's "A and B" (§4.12 :422) need option A and the cheats' `Behaviour`s, which are U4f's (plan :806-809), and the
  series of the parts U4f, U4g and U4h move come from their code, so at U4r they would be gaps, which make runs INVALID
  (eval.rs:314-327). -58's part moved to U4g (C2), while carries 73 and 75 run -58's settings in the contract test
  (tick16 past the wrap, an event start 3 below the wrap), carry 59 adds an ES suite and carry 62 (g) a short L1 P3
  suite with the reorder step.
- Options: (a) at U4r, r2 covers -17's four profiles, -18's B1, -20's P3 option B and -21's option-B suites, the
  open-loop scripts standing in for the cheats' command patterns (PM-0001-2 and the shot band only; those suites' own
  checks not resolved), plus the carries' suites (ES, tick16, the far event start, P3 with the reorder step, run with
  -58's settings but judging no -58 check), and each later unit extends r2 to its parts before its judged runs, option
  A's suites joining at U4f; (b) as (a), but -20's and -21's suites wait whole for U4f; (c) U4r writes every later
  part's series as placeholders (a placeholder is a check that cannot fail).
- Recommendation: (a): it keeps condition 5's order (the load passes before the marks count; -20 and -21 move in U4f:
  plan :810-812) for every suite that can run now.

**U4R-Q6 (the wallclock bite's site).** Who plants PM-0001-30/wallclock, "a wall-clock read in the pump for one leg's
sessions, bites build only" (plan :791)?
- Why it matters: the pump is npt-server's (U4e, plan :722-723); no PM-0001-30 id is planted anywhere at the head (D6);
  U4r's lane holds no npt-server file; server/clippy.toml bans clocks and hash maps (server/clippy.toml:1-17; A13), so
  the site needs a bites-only allow, as the sim's and l1's bite sites carry (l1/src/lib.rs:768-770); check.mjs's
  one-home rule then puts its id in npt-server's sources, whose list `npt_server::BITES` is schedule.rs:251's (U4b's
  file; U4E-Q3 had U4e add its pump's ids to that list). Wherever it sits, the read must change a sim input on that leg
  on every run, whatever the machine's ticks per second (7.2; marks.json:6563: "a wall-clock read on one transport's
  command path"): on the commands the pump passes on, never in its RTT or ping code, which nothing in the room reads at
  U4r, nor in a rate bucket, whose starving depends on the machine.
- Options: (a) U4r's lane gains that one bites-only site with its allow in crates/server/src/pump.rs, and the id's line
  in `npt_server::BITES` (schedule.rs:251, or a list in lib.rs beside it), nothing else; (b) the main session plants it
  in its own commit before U4r's build; (c) the site moves into npt-lab's leg glue (not "in the pump", and npt-lab's own
  clock ban, lab/clippy.toml, would need an allow too).
- Recommendation: (a): the plan plants each bite where its code lives (§4.14 :454), and U4r is the unit that shows it.

**U4R-Q7 (-17's clean runs' `tick_start`).** Do -17's clean L1 runs start at `tick_start` = 63,736 like its tick16 bite
runs?
- Why it matters: the plan sets 63,736 on the bite runs (plan :767-768); marks.json's `tick_start` setup check covers
  only -17's "every S30" part. If the clean runs start at 0, a bite run differs from its clean run in two things, and no
  clean L1 run shows the codec surviving the 16-bit wrap under the stress scene's load (-58's runs are U4g's now, and
  last 10 s).
- Options: (a) every -17 L1 run, clean and bitten, starts at 63,736; (b) the clean runs start at 0.
- Recommendation: (a): same run count and length, one difference per bite run, and the wrap covered under load.

**U4R-Q8 (-18's runs).** Are -18's L1 B1 runs -17's B1 runs (10 bursts each, headers listing -17, -57 and -18), or a set
of their own?
- Why it matters: both parts select L1 B1 with SS and 48 clients (marks.json -17's and -18's L1 B1 parts; §4.12
  :420-421), B1 is the burst profile, and the judge selects runs by the marks their headers list (eval.rs:32); separate
  runs add 1,800 s simulated (7.3).
- Options: (a) shared, with -18's burst settings (10 bursts, offsets 0 to 1 tick) on -17's B1 runs; (b) separate.
- Recommendation: (a).

**U4R-Q9 (the TC B1 recording at U4r).** Which clients make the TC B1 seed 3 runner recording (§4.15 :495-497), when
the TC shooter `Behaviour` is U4h's (:837) and U4r builds only the brawl and the open-loop scripts (:756)?
- Why it matters: the runner recordings are meant to hold "the room's real Rewinds" (§4.15 :497); a TC recording made
  without TC's shooter holds Rewinds that shooter would not make; and each runner recording that joins the manifest
  adds to the `wasm` step's time (carry 82; section 5 item 4).
- Options: (a) U4r records it with the open-loop scripts aimed at TC's targets, and U4h re-records it with its shooter
  (the manifest's entry changes); (b) U4r records SS P2 seed 1, ES P3 seed 2 and SS P0 seed 4, and TC B1 seed 3 waits
  for U4h, the determinism step keeping U4d's three synthetic recordings meanwhile (§4.15 :494-501); (c) the brawl on
  TC.
- Recommendation: (b): U4h builds both the shooter and the Rewind rules (:835-837).

**U4R-Q10 (showing the RTT rule can fire).** How is it shown that the pre-check's RTT rule (a client whose first command
ran before its first sample makes the leg VOID: plan :788-790) can fire?
- Why it matters: the rule is the pre-check's only run-level guard on the RTT argument once U4h's estimate reads the
  RTT, and the plan's two bites (:791-792) never exercise it; a guard never shown to fire is not shown to work (the
  "bites first" rule: catalogue :237-238; §5's preamble :511-512). At the head a run cannot show it at P0 (D12): the
  server end's first sample comes from `JoinAccepted`'s packet, at its send instant, and is applied by the first net
  call at or after it (l1/src/lib.rs:947-959, :1433-1435), before any later read at that instant; and the pump returns
  after a `Join`, so a session's next message is read only once the room has answered it (pump.rs:328-330, :394-399).
  So rev 2's (a'), a run-level bite with the one timing that might fire, is not exercised at U4r on this code.
- Options: (b') test r7 (2.4): the rule's code on doctored per-client records (a first-sample instant after the first
  command's run instant: VOID; equal instants: not VOID; no first sample on the L1 leg: VOID; Loopback's "Some(0) from
  the start": never VOID), U4R-Q11's pattern, with no runs; (a') a run-level bite, only in the re-run after U4h and only
  if U4h's code gives a timing in which it can fire (U4h's brief states it); (c) neither.
- Recommendation: (b') at U4r; (a') only as its option says.

**U4R-Q11 (negative cases for the contract test).** Does r2 get negative cases: a short run's logs doctored to lose one
`admit` event, one footer, one written name's SERIES.md row, or one `late_count` series where `late-zero` applies, each
failing r2 by name?
- Why it matters: the plan gives r2 no bite (plan :758-761), so nothing shows it can fail, and it is the test that
  stands between a log missing a validity input and a judged run.
- Options: (a) yes, as unit tests on doctored copies, the "bites first" pattern (catalogue :237-238); (b) no.
- Recommendation: (a).

**U4R-Q12 (the re-run after U4h: its shape and its home).** Which re-run does the advisor's "re-run the pre-check pair
after U4h merges (6 runs)" (advisor :346; carry 40) mean, and where does it live: 8.3's replay-first rule for the
recording, results in results/u4h-precheck-30/<commit>/ read in U4h's review, and a carry for it in carries' §U4h
(:548-560)?
- Why it matters: carry 40 asks this brief to propose the re-run or say why not, and the words have two readings, so the
  brief must not settle them alone; U4h's §5 (:834-848) has no line for the re-run, so without a carry and a folder it
  has no owner.
- The two readings, with their cost (each run the recording's length, about 300 s simulated; 7.3's R values turn them
  into wall time):
  - (i) this brief's (8.3): one recording (SS P0 seed 4), three pairs (clean, wallclock, late-command): 6 runs, about
    1,800 s simulated; 9 runs, about 2,700 s, if the replay-first clean pair shows replacements (the VOID pair, one
    re-recording, then the six).
  - (ii) -30's own shape (three recordings of 5 minutes, both transports, so all 6 runs are measured: marks.json -30's
    parts, `min` 6): three P0 recordings (two more recorded on U4h's build), three clean pairs plus the two bite pairs:
    10 runs and 2 recordings, about 3,600 s; 12 runs and 3 recordings, about 4,500 s, if seed 4 must be re-recorded.
  - Not offered: three clean pairs on the one recording would repeat one deterministic run (the same commands, and P0
    draws no delay and no loss: l1/src/lib.rs:178-180), so they would show nothing one pair does not.
- The carry's premises, whichever reading: the re-run compares each client's first-sample instant with the instant the
  room first reads that client's RTT for a command (its receipt, if U4h's estimate is taken in the pump; its run, if
  `rewind_at` reads it at the command's tick), both logged at U4r (7.2); the Loopback leg's sessions take the link's
  `rtt_ns` (Some(0)), not the `ws` ping samples (§4.9 :352-355), as U4r's leg glue sets them (`RttSource::Link`); and
  U4h's brief states what its estimate does with an L1 session whose `rtt_ns` is still `None` (R38).
- Options: (a) reading (i), as 8.3 proposes, with the carry; (b) reading (ii), with the carry; (c) either reading,
  written straight into U4h's brief from this ruling, with no carry; (d) no re-run (8.3's alternative (b)).
- Recommendation: (a): the pre-check separates the room's command path from the link, which one P0 recording tests;
  the three-recording shape is -30's C part at U6 (plan :894-895).

**U4R-Q13 (carry 63: does U4r take crates/log?)** Carry 63 gives "the next change to npt-log (U4r's, if its brief takes
crates/log; else the unit that next edits it)" two refusals: `read_log` refusing a JSON object with a duplicate key
(FU-3), and `check_header` refusing any header text field holding a control character (FU-7; log/src/lib.rs:305-321).
- Why it matters: both stay open security findings in PR #40's report until made (carry 63's last sentence); U4r's
  runner is the first writer since U4j of the logs npt-log reads, and the one that must never emit such a key or
  character, so its tests are where the refusals meet real logs; but npt-log is U4j's crate, U4r's lane holds none of
  it, and no later unit is named for it.
- Options: (a) U4r's lane gains crates/log/src for these two refusals and their tests only (each failing before its
  refusal, as the carry asks), with security-auditor in the review panel; (b) not U4r's: the next unit that edits
  npt-log makes it, and both findings stay open meanwhile; (c) the main session makes it in its own commit, before U4r's
  launch or after its merge.
- Recommendation: (a): two small refusals with their tests, next to the first large set of logs they guard, reviewed by
  the seat that raised them; (c) if the advisor prefers U4r's lane to stay inside crates/lab and crates/bots.

---

## 11. Needs a run (day)

What only a build, a test or a run can settle, with its command; the items a read settles are filled. `<base>` is the
SHA of section 1.
- **D1, the base counts (filled):** at cc8595d, U4d2's merge, the main session's counted run (M5): check.mjs's 11
  steps all PASS, measured 491, bites 561, self 71 of 71, marks 81 read and checked, redact 0 findings (d2-r2 Block 5,
  :245-248: 473 + 17 + 1, 541 + 19 + 1, 52 + 11 + 8). They are `<base>`'s counts if `cd /home/user/Shamble-Frontlines
  && git diff --quiet cc8595d <base> -- spikes/netcode-proof` exits 0 (it exits 0 at fa8e335); else a counted run in the
  main folder through main-run.sh gives them, `<M>` being /home/user/Shamble-Frontlines:
  `bash <M>/.checkpoints/netcode-proof-test/main-run.sh <M>/.gate/u4r-launch/base-counted.log node check.mjs`
  (the shared target, the lane lock held exclusively, offline: main-run.sh:1-11), reading the test step's two counts,
  self's "K of K" and the step count.
- **D2, the starting RTT (filled):** `git grep -n 333 <head> -- spikes/netcode-proof/crates/l1/src` gives
  INITIAL_RTT_NS (l1/src/lib.rs:118) and the doc (:40-44); `rtt_ns` returns `None` before the first sample (:64,
  :1748-1750). R2, R38.
- **D3, check.mjs's step count and the `wasm` step's time (filled):** 11 steps (check.mjs:2468-2503); the `wasm` step
  33.1 s warm, in M5's counted run at 52 minutes of uptime, and 61.6 s cold, INF-1's figure (m9 Block 1;
  .gate/u4d2-merge/cold-run2.log: the four release builds 41.8 s), against the step's 5-minute budget, on the three
  synthetic recordings.
- **D4, the runner's ticks per second (stays):** in the main folder after the merge, before any set starts: the timing
  command the builder reports (return list item 8). Its exact form cannot be written before the build.
- **D5, the manifests (filled):** `git diff df8975b <head> -- spikes/netcode-proof/crates/lab/Cargo.toml
  spikes/netcode-proof/crates/bots/Cargo.toml` is empty; Cargo.lock changed since (U4d2's npt-hash-wasm), with no
  npt-lab to npt-bots edge and no npt-bots to npt-sim edge. R11; U4R-Q1.
- **D6, the bite ids at their sites (filled):**
  `git grep -n "\"PM-0001-17/" <head> -- 'spikes/netcode-proof/crates/*/src/*.rs'` and the same for -18, -57, -58,
  -30 and `catalogue-5/`: -17/tick16 proto/src/quant.rs:38, -17/client-rounding client/src/lib.rs:29; -18/no-fallback
  room/src/lib.rs:39; -57's three quant.rs:35-37; -58/tick16 quant.rs:39, lt-ack, lt-cmd-seq and lt-press
  room/src/lib.rs:40-42, lt-tick-offset and lt-event-seq client/src/lib.rs:30-31;
  catalogue-5/owner-to-all room/src/lib.rs:45; PM-0001-30/wallclock and /late-command: no site. Each equals
  marks.json's spelling (R29).
- **D7, after U4h merges (stays):** the pre-check's clean pair on U4r's SS P0 seed 4 recording, on the U4h build, by
  the pre-check script, reading both legs' replacement counts (re-record if either is above 0); with both at 0, this
  pair is the re-run's clean pair (8.3).
- **D8, a clock in npt-lab or npt-bots (filled):** `git grep -n -e Instant -e SystemTime <head> --
  spikes/netcode-proof/crates/lab spikes/netcode-proof/crates/bots` finds only lab/clippy.toml's bans (:8-10,
  :13-15): no source reads a clock.
- **D9, the recordings' time (stays):** the `scenario --record` runs' wall times as the builder reports them (return
  list item 9), and the `wasm` step's time with them in the manifest (section 5 item 4), against the 5-minute budget
  beside D3's figures.
- **D10, the RTT's readers and the replace, clip or substitute paths (filled):** `git grep -n -i -e rtt -e round_trip
  -e srtt <head> -- spikes/netcode-proof/crates/room/src spikes/netcode-proof/crates/server/src
  spikes/netcode-proof/crates/client/src` finds only pump.rs's `RttSource`, its getter and the doc lines (pump.rs:5,
  :25, :95, :180-190, :210-238, :295-301, :454) and lib.rs's re-export (:14); `git grep -n -i -e replace -e clip -e
  tolerance -e estimate -e rewind_at <head> -- spikes/netcode-proof/crates/room/src` finds `rewind_at` (rewind.rs; its
  call room.rs:860), the counter's doc (room.rs:134), and the roster's replacement of a live session (roster.rs,
  room.rs:632-686), which is not a view time. R33 holds; O1's statement stands (8.1).
- **D11, the client's `Resync` and the event cap (filled):** `git grep -n -e Resync <head> --
  spikes/netcode-proof/crates/client/src` finds nothing: npt-client never sends one; `git grep -n -e EventQueue <head>
  -- spikes/netcode-proof/crates/room/src` gives the cap, `PROVISIONAL_CAP` 1,024 (room/src/events.rs:40; U4E-Q6 (a)),
  above §4.7's "about 400" (:335-336). R34.
- **D12, when the server end first holds a sample at P0 (filled):** `git grep -n -e rtt_ns -e advance -e sample <head>
  -- spikes/netcode-proof/crates/l1/src`, then the reads: an ack is sent at its packet's arrival with a delay drawn the
  other way (`send_ack`, l1/src/lib.rs:1418-1465; 0 at P0), so `JoinAccepted`'s ack is an event at its send instant,
  applied by the first call that takes a `now` at or after it (`advance`, :947-959; only the sends-before-events bite
  applies it inside the send, :941-945), and the sample is that arrival minus the send, 0. The pump reads no RTT
  (D10); the lab reads `Decoder::rtt_ns` after each pump call (7.2). R4; U4R-Q10.

---

## 12. Left for the main session at launch

1. **`<base>` and its counts (P6).** The spike branch's head at launch. If `cd /home/user/Shamble-Frontlines && git diff
   --quiet cc8595d <base> -- spikes/netcode-proof` exits 0, the counts are D1's (measured 491, bites 561, self 71 of 71,
   11 steps, marks 81, redact 0); else D1's counted run through main-run.sh gives them. Fill P6 in section 1 item 1 and
   section 4 rule 5.
2. **The runner (P7: u4r-run.sh, sha256 349b51ad…).** Made from u4d2-run.sh by u4r_runner.py, its diff u4r-run.sh.diff
   holding only the lane's names (the header, the log folder .gate/u4r/, the usage line), the CACHEDIR.TAG check and the
   two lock lines u4d2-run.sh's (m9 Block 2, :202-206). Re-check its sha256 and `bash -n` at launch. If
   `.claude/worktrees/u4d2-build` is gone, INF2-4's P3 second call is instead the first call of U4r's lane runner, with
   the same check, shown at U4r's brief (m9 :211-216).
3. **The placeholders, by script, each checked after the copy** (u4d2-brief.md 7.2 item 3's rules: the anchor found
   once, the copy's first and last words as named, the placeholder gone from its one home outside this list, no other
   byte of the brief changed; a source span that wraps is copied with each line break and the next line's indent read
   as one space and rewrapped to 120, and the assert compares copy and source with every run of white space read as one
   space; a closed file's sha256 is asserted equal to section 0's stamp first):
   - `<<P1>>` carries: from the line beginning "## U4r (the L1 runner and its judged runs)" (found once) to the line
     before the next line beginning "## "; the script asserts the copy's entry numbers at line start are exactly 40,
     49, 58, 59, 60, 62, 63, 69, 73, 74, 75, 76, 77, 82, 83, 84 and 85.
   - `<<P2>>` history: the line beginning "  - Builders: pass or fail comes from the exit code" (found once) and the one
     continuation line after it (ending "before it changes anything."); the script asserts the line before it is "
     USAGE-SAVINGS: (a), all three, with these limits.".
   - `<<P3>>` df-ruling :462-464: the sentence inside T4 item 1's quotation marks, from "Write files with the" to "whole
     with Write.".
   - `<<P4>>` df-ruling :465-468: the clause inside T4 item 2's quotation marks, from "a failure in <crate>" to "and go
     on.", with the first `<crate>` replaced by "npt-lab or npt-bots, in check.mjs's deps or wasm step, or in
     check.test.mjs" (and ", or in npt-log" after "npt-bots" if U4R-Q13 gives U4r crates/log) and "<crate>'s files" by
     "their files"; the script asserts the replacements and no `<crate>` left.
   - `<<P5>>` j-r2 :168-171: from "a FAIL in check.mjs's test step" to "until it is explained.".
   - `<<P8>>` j-r2 :298: the words after "From now on: " to the line's end.
   - `<<P9>>` m9 :252-256: the text inside W1-TARGET item 3's quotation marks, from "Between waves, with no lane
     running" to "never `--target` without `-p`.".
   - `<<P10>>` m9 :259-264: from "(1) never pass `--target`" to "no session removes it by hand.".
   - `<<P11>>` d2-r2 :304-308: from "never run cargo" to "may use it as one.".
   - `<<P12>>` d2-r1 :308-310: from "its \"until carry 65 is met\"" to "from that failure itself.".
   - `<<P13>>`, `<<P14>>`, `<<P15>>`: each from the line beginning "6. **arch F6 / rust 1", "14. **rust 3 (the 1 MB
     half)" and "20. **arch N2:**" (each found once) to the line before the next line that begins with an entry number
     or "## ".
   - `<<P6>>` (item 1); `<<P7>>` (item 2): values, filled wherever they stand.
4. **The advisor's rulings on U4R-Q1, Q2 (a) and (b), Q4, Q6 to Q13** (section 10), folded into the places they name
   (each "by U4R-Qn" read as the ruling's words), with the lane of section 4 widened or narrowed to match; the plan text
   a ruling changes is in the main session's own commit, as the earlier briefs' rulings were.
5. **TE2-5 (c), TE-M3's planned change** (d2-r1 :303-304: before U4r's merge): name its state here (planned, made at
   <sha>, or not yet), so the builder's TE2-5 entries read it.
6. **This brief's checks:** netcode-architect's check of rev 3 and test-engineer's revision (this workflow), then the
   advisor's check, which also checks INF2-4's two diffs (main-run.sh.inf2-4.diff, u4d2-run.sh.inf2-4.diff) and the
   proof logs P1 to P4, with u4r-run.sh and its diff (m9 Block 2, :223-227).
7. **The review panel** (plan :803 and the rulings): netcode-architect, test-engineer, engine-architect (public items:
   plan :929-930), rust-reviewer; plus security-auditor (carry 76's session boundary, carry 85's refusals, carry 63 if
   U4R-Q13 gives it to U4r) and infra-engineer (carry 84's time bound and the `wasm` step's time against its budget: no
   CI job runs check.mjs); at effort xhigh, at most three agents at a time, reviewers included (plan :885-886), so in
   two batches; the advisor confirms the panel.
8. **The judged runs' target.** The scripts read their binaries' paths from cargo's JSON messages (2.3); at the judged
   runs the main session chooses the main folder's own cache (plan §7.1 item 6, `CARGO_TARGET_DIR` unset: :917-918) or
   the shared target under main-run.sh's lock, and measures D4 there first.
9. **Re-stamp and re-check at launch.** Re-stamp section 0's sha256 of the plan, plan.md, carries and history; if one
   moved, re-check every cite of this brief into it (the carries gain entries at merges; history's "Decisions taken" is
   newest first) and fold the new lines in the main session's commit; if spikes/ at `<base>` differs from cc8595d,
   re-walk section 9's rows for what changed.

