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
  (U4D2-Q10; plan :674-675; lab/clippy.toml:1-15); npt-bots' clippy.toml with both type bans and the clock ban is
  U4r's (the advisor's U4d2 brief check, A13, d2-ruling :588-589 and :725; plan §4.11 :393-396). Section 4's lane gains
  crates/bots/clippy.toml.
- **C4. U4R-Q2 answered in part by carries 82 to 85** (d2-r1 Blocks 4 and 5; d2-r2 Blocks 1 and 2): the recordings
  manifest's schema and its reader (crates/lab/recordings.json; hash.rs's `manifest` :213 and `regenerate_all` :292),
  check.mjs's `wasm` and `deps` steps, check.test.mjs, TickHash's struct doc (lab/src/lib.rs:8) and loader.mjs's
  header sentence (crates/hash-wasm/loader.mjs:13-14) are U4r's, so the lane (section 4) and the seam rule (2.6) name
  them; the determinism step's route is decided by U4r's measured times under plan :800-802 and m9 Block 1 (:144-145).
  Still open in U4R-Q2: the SERIES.md rows and carry 62 (h)'s one check in marks.json.
- **C5. The carries, whole.** Rev 2 quoted carry 40 alone; section 3.1 now holds u4s-carries.md's §U4r whole as
  `<<P1>>` (entries 40, 49, 58, 59, 60, 62, 63, 69, 73, 74, 75, 76, 77, 82, 83, 84 and 85), and section 3.2 says, entry
  by entry, where this brief meets it (the task's item 4).
- **C6. Carry 40's new words** (the advisor's U4d close ruling, F3, close :320-327; U4E-Q5's condition, wave1
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
  N12 (d) waited on this). New row R41.
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
  and 4 GB free; W1-TARGET item 3 as amended (`<<P9>>`), m9 Block 2's two lessons (`<<P10>>`) and d2-r2 Block 6's lesson
  (`<<P11>>`); T4 items 1 and 2 (`<<P3>>`, `<<P4>>`); TE2-5's rule (`<<P5>>`) with its reading after U4d2's merge
  (`<<P12>>`); B6's plant line (`<<P8>>`); Mustafa's builder lines of USAGE-SAVINGS (a) (`<<P2>>`); RM (T4 item 3,
  carry 67) only if U4r moves ES's synthetic recording or the World's outcome on it; TE-M3's npt-net flake recorded
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
- **C20. Section 9 walked at the head:** 13 of R1 to R34 hold, 21 changed (each with what the code has and what the
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
     (:20-215, :320-327); j-ruling's "Carries", U4r (:402-406) and R2.4 (:438-449); j-r2 B2.1, B2.3 and B6
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
     `with_bite`), crates/client (lib.rs), crates/log, crates/judge (eval.rs's gap and expect-invalid rules; figures.rs's
     coverage), crates/sim (replay.rs's `Recording` :1-60; seam.rs's `SimCore` :545-565; world.rs's `new` and
     `with_bite`), crates/proto (lib.rs, quant.rs's `BITES` and `with_bite`, input.rs's `Input`, snapshot.rs's
     `EntityDelta`), crates/hash-wasm/loader.mjs's header (:1-30); check.mjs's steps (:2468-2503), test step
     (:1973-1982), deps step (:2062 on) and wasm step (:2099-2300 and on), and check.test.mjs's tests of those two steps;
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
  (60 Hz, seeded phase); (5) each client whose `next_due_ns` has come (`StepOut::next_due_ns`, client/src/lib.rs:193-195),
  in slot order. Each client's host clock is offset + rate × SimClock, both seeded (plan.md:625; the 1.5 rate is -21's
  cheat, U4f's). The link is U4c's `L1Net`, event-driven through `next_event_ns` (l1/src/lib.rs:1876-1882; L-c5); L1's
  RTT is its RFC 9002 estimate, never the simulator's true value (plan.md:626). npt-lab names `npt_sim::World` once
  (U4-SEAM, plan :162-163). The plan gives `L1Scenario` no test of the order itself (plan silent, not added): say in your
  report how the order is shown.
- **The header params** (plan :751-752): `scene`, `clients`, `profile`, `seed`, `cmd_timing`, `tick_start`,
  `ticks_below_wrap`, `suite`, `stand_ins`. `cmd_timing` is `B` in every U4r run (option A is U4f's, :806);
  `stand_ins` records S1's 1 ms burn and 2.80 ms top-up as off on L1 (§4.13 :448-449). Each header also records the
  bite id, the enabled features and the binary's SHA-256 (plan.md:630-631), and the params the runs and the carries
  read: `burst_starts_ns` (as U2's probe writes it: lab/src/main.rs:142); `buffer_depth_ticks_count` for -30's report
  (marks.json:6557; 7.2); `twin` for the twin test's runs; the lead beside `cmd_timing` (carry 74); the scene's weapon
  (interval, damage, reach, magazine, reload, spread) and its soldiers' health from `Scene::weapon()` and
  `Scene::health()` (carry 60); the event cap and whether it is provisional (`Room::event_cap`, room.rs:558: carry 57's
  "for the run's header"). The header's `tick_hz` comes from `npt_sim::TICK_HZ` by path, not from a literal as U2's probe
  header has it (lab/src/main.rs:159; carry 6's pattern, the F2 ruling's point 2, advisor :445-449).
- **The `--bite` dispatch to each instance** (plan :752; §4.11 :399-401): one `--bite <id>`, handed to each instance
  whose crate's list holds it, never a static: `World::with_bite(scene, seed, first_tick, bite, arm_tick)` (world.rs:944;
  `World::BITES` :937-938), `Room::with_bite` (room.rs:510-518; `npt_room::BITES`, room/src/lib.rs:37-46),
  `ClientEndpoint::with_bite` (client/src/lib.rs:308-320; `npt_client::BITES` :28-32), `L1Net::with_bite`
  (l1/src/lib.rs:1828-1840; `L1Net::BITES` :1823-1824), `Schedule::with_bite` (schedule.rs:257; `npt_server::BITES`
  :251). An npt-proto id (`npt_proto::BITES`, quant.rs:34-42) goes to every instance that codes the wire: the room's
  encoder, each `Decoder` (its `Quant`, pump.rs:224-231) and each `ClientEndpoint`, each taking its `Quant` from
  `Quant::with_bite` (quant.rs:102-112; U4E-Q4, wave1 :458-460). `catalogue-5/owner-to-all` is selected by that string
  (U4E-Q13 (a), e-ruling :262-264). An id no instance knows is a usage error (exit 2, as U2's probe:
  lab/src/main.rs:12-13). npt-lab's sources never spell another crate's id (section 4). The measured build takes no
  bite (plan.md:630-631), and U4j's C4 makes a measured run whose build lists `bites` or names a bite, or a bite run whose
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
  fields (an `Add`'s whole record; an `Update`'s `changed` bits, each with its absolute grid integer: snapshot.rs:62-77),
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

### 2.2 npt-bots' library and its clippy.toml (crates/bots/src/ but src/bin/; crates/bots/clippy.toml; crates/bots/tests/)
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
  crate-wide, in lab/clippy.toml's form (:1-15); U6's runtime half adds its reviewed allows (§4.11 :395-396). Its proof,
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
  "release", ...)`, c-u2:55 and :61; .cargo/config.toml:3-4), as plan §7.1 item 6 has the judged runs (`CARGO_TARGET_DIR`
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
  digest) or the World's outcome on it (t_m1's printed line); carries 71 and 72 likewise. U4r's plan is neither: it edits
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
   worktree>/spikes/netcode-proof/crates/lab)" and npt-bots lines, which the runner appends, are the provenance. Match on
   the path, not the version. A missing line means "not counted", whatever the count says.
4. Touch before each build: `find <copy>/spikes/netcode-proof/crates -type f -exec touch {} +` (plan :912; the runner
   does it).
5. **Counts are deltas** (plan :913-914, :883-884), on check.mjs's measure (`testBothBuilds`, :1973-1982;
   `cargoCounts`, :1864). The base: measured 491, bites 561, self 71 of 71, 11 steps, marks 81, redact 0 (P6: the
   counts at `<base>`; at cc8595d, U4d2's merge, they were these: section 11's D1). Each run's count is the base plus the
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

<!-- rev3-continues -->
