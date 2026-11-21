# U4e report: the room and the codec path (npt-room, npt-server's library, npt-client)

Merged 2026-11-12 into the spike branch by the main session (the commit "U4e: the room and the codec path (npt-room,
npt-server's library, npt-client), merged after review round 2"). The approved brief: u4e-brief.md rev 3.3 (the
advisor's check, advisor-u4e-brief.md: APPROVE on L1 to L4), on `<base>` 28a85e7 (U4e's L3: carry 51's
`Scene::effect_rows`). For U4's report, which folds this one. Mustafa's words that bind it: history.md:340
(2026-11-11: the Low about the effect row's owner deferred to U4e in writing; a Low never triggers a push).

## Who built it, and how
- **The build:** one builder (general-purpose, opus, xhigh) in `.claude/worktrees/wf_0fa6595a-3e2-1` (branch
  worktree-wf_0fa6595a-3e2-1 at 97bb9d3, whose spikes/ is `<base>`'s), every cargo run through u4e-run.sh under
  W1-TARGET's lock. Its first run (wf_0fa6595a-3e2) was cut by the spend limit at 02:03 UTC after 49 s with no code;
  the relaunch (wf_1e3b5232-26a) built npt-room, npt-server's library (the pump) and npt-client's endpoint, and its
  two counted check.mjs runs passed all 10 steps: measured 458, bites 524, self 52 of 52. Record: u4e-build.md.
- **Review round 1** (wf_ed170231-913; the packet u4e-packet.md, cut by review_packet.py): netcode-architect REVISE,
  engine-architect REVISE, test-engineer REVISE, security-auditor and rust-reviewer APPROVE WITH FINDINGS. The test
  seat was cut by the spend limit at about 06:42 UTC with its plant P-D in place; the main session restored the file by
  Edit (its sha256 equal to the start), and the seat resumed on the same bytes. Notes:
  u4e-review-{netcode,security,rust,arch,test}.md.
- **The advisor's round-1 ruling** (advisor-u4e-r1.md: REVISE; R1 to R9, texts T1 to T6): the fix pass FX-1 to FX-13.
- **The fix pass** (wf_4c29cbe6-2fd): one builder in the same worktree, each FX with its test and a plant that fails it
  by name; its two counted check.mjs runs passed all 10 steps at 473 / 541 / 52 of 52. The spend limit cut it after
  those runs; the main session recorded them (a labelled block in the record) and wrote the end hashes.
- **Review round 2** (wf_e623d489-17e; the packet u4e-r2-packet.md): the same five seats in three batches of at most
  two (the advisor's U4d2 brief check, L5), all APPROVE WITH FINDINGS: one Medium (NC2-M1) and twelve Lows, none before
  the merge. Notes: u4e-review-r2-{netcode,security,arch,rust,test}.md.
- **The advisor's round-2 ruling** (advisor-u4e-r2.md: APPROVE on Q1 to Q4): NC2-M1 a carry (C-U6-16 (a)); the merge
  on M1 to M9, with M5, M8 and M9 amended; T1, T2's block (3), T3 and T5 changed and T7 new; nothing for Mustafa.

## What U4e's review settled (the advisor's U4e rulings: round 1, advisor-u4e-r1.md; round 2, advisor-u4e-r2.md)
- **Round 1** (five seats on one head, 97bb9d3 plus the snapshot of db49108; the test seat was cut by the spend limit
  at about 06:42 UTC and resumed at about 07:36 on the same bytes, so one round): netcode-architect and
  engine-architect REVISE, test-engineer REVISE, security-auditor and rust-reviewer APPROVE WITH FINDINGS; two Highs
  (NC-H1 and TE-1, one defect). One fix pass (FX-1 to FX-13), then round 2 on what changed (the snapshot of 4300944;
  the same five seats in three batches of at most two): all five APPROVE WITH FINDINGS, one Medium (NC2-M1, carried)
  and twelve Lows, none before the merge; every fix's test failed by name under its plant when the test seat re-ran
  it, and its counted check.mjs passed all 10 steps at 473 / 541 / 52 of 52. No third round.
- **tick16's proof (NC-H1, TE-1; High; fixed).** Under PM-0001-17/tick16 and PM-0001-58/tick16 the client dropped every
  post-wrap group as too old and filed nothing, so the twin passed on a silent drop and a judged run would have read 0
  mismatches. Since the fix the client files the sentinel when a header names a tick before its first snapshot and at
  least `npt_sim::ACK_RING_TICKS` ticks behind its newest (a bad header: marks/README.md §1), and the twin asserts what
  the judge counts: under each bite 14 client hashes at ticks the room never logged for that slot and none at or past
  65,536; in the clean build 0 bad of 58 compared, the hash at 65,537 equal to the mirror's. U4r's judged half: carry
  73. The rest of the two cases' form (a bad header never acknowledged and kept out of the frame ring; a tick already
  resolved dropped, never the sentinel) gets its own endpoint test at U4f (carry 80 (d)).
- **The client's late groups (carry 61 (d); B2.1): "decode".** A group of a tick still in the client's frame ring is
  decoded and hashed under its tick. A group of a tick from the client's first snapshot on that has left the ring
  (already resolved) is dropped, never acknowledged, and counted in `ClientEndpoint::stale()`, which is not carry 61
  (d)'s drop and has no SERIES.md row (U4r reports its sum: carry 77). The readings list gains no entry, and carry 62
  (g)'s drop count does not apply.
- **-18's condition 6 (§5 U4e; NC-L1): not triggered.** Under no-fallback the room codes F = 363 against B = 329 at age
  34 (AfterSend) and F = 362 at age 33 (BeforeSend). B's bytes are still in the client's ring slot 9 (F uses slot 11 or
  10), but the client refuses a baseline age of `ACK_RING_TICKS` or more before it looks at the frame: §4.5's "the
  client keeps its complete frames for the same ages". B is not held, so the bite is a defect on L1, and -18's L1 part
  counts at U4r.
- **The own-state section (carry 56, with carry 68).** SS: 18.94 B mean, 30 B p99, 121 B max per snapshot, the OwnState
  record 9.03 B, changed in 0.9961 of snapshots; ES: 15.55 B, 21 B, 25 B, OwnState 8.27 B, changed in 1.0000. The SS
  mean is above R2's 16 B a snapshot for the local player's own state (r2-netcode.md:1756): reported beside it, not
  judged (PM-0001-12 judges the downstream total). ES's figure comes from `Recording::synthetic`'s ES recording, whose
  alive soldiers change yaw on about 48 % of soldier-ticks and pitch on about 18 %, against R2's moving-delta assumption
  of yaw in every moving update and pitch half the time (docs/research/r2-netcode.md:1751); the own Soldier record
  carries both, so ES's figure reads lighter than R2's estimate and is no measure of ES's load (carry 68).
- **e7's clauses (NC-L3).** On zero-delay Loopback no tick splits and no event is resent (e7: 48432 pairs compared, 0
  differ or are the sentinel; per client at least 1009; split ticks 0, skipped 17184, events carried 562794, resent 0,
  non-default baselines 48384; ClientCounters { snapshots: 48384, groups: 48384, split_ticks: 0, full_states: 0,
  defaults_coded: 0, stale_coded: 0, skipped: 17184, budget_short: 0, refused_out_of_range: 0, refused_too_many: 0,
  refused_too_large: 0, refused_not_canonical: 0, events_queued: 562794, events_sent: 562794, events_resent: 0,
  events_expired: 0, cosmetic_sent: 278219, cosmetic_skipped: 805371, overflow_resyncs: 0, resyncs: 0, resyncs_dropped:
  0, resyncs_limited: 0, kicked: 0, commands_buffered: 47623, commands_repeat: 0, press_refused: 0, late: 0,
  buffer_full: 0, ack_refused: 0, ack_refused_commands: 0, acks_old: 0, commands_run: 47623, commands_rerun: 761,
  rewinds_refused: 63 }). The split clause is carried by e2 (300 B links: 3,024 split ticks, 3,023 compared equal) and
  the resend clause by gap.rs's clean run (236 pairs, 0 differ, 462 and 439 events resent).
- **Carry 26 (A7).** The room copies each step's events once a tick into its stamp list (carry 23's tick and 32-bit
  number), a reused vector of `Copy` events; raised with the advisor, who keeps the seam as it stands.
- **Carry 54's series (A14):** `event.snap_refused.tick_count`, its row added to SERIES.md in the merge commit.
- **§4.2's rewind assert (NC-L5, RR-1):** met by construction: `rewind_at` returns `None` before it builds a Rewind
  that names an unheld tick, so its debug assert cannot fire.
- **Values the plan leaves open (NC-L11, A11):** the interest radius (64 m) and its cell, on which the plan is silent,
  and the hit history's 2 s (provisional until U4h: carry 28) are module constants, U4e's choices; a setter or getter
  comes with the unit that first needs one.
- **The RTT (A2).** The pump takes each session's RTT from its `RttSource` (§4.9). The room's per-tick report from the
  pump (`pump_report`) carries the bytes sent and the link's largest datagram, not the RTT, which U4h's tolerance will
  read; the builder's record (u4e-build.md, design decisions) says otherwise.
- **A known deviation (SEC-U4E-6, A13).** A Hit or Kill reaches a client that sees only the victim with the shooter's
  slot, against ADR 0001 part 9 (f) ("no shooter id when the shooter is not relevant to the viewer"); the wire has no
  form without it, and part 9 is proven at slice 3 (ADR 0001:1687-1688).
- **The 1,150 B case (TE-10).** The endpoint's test datagram is refused by its trailing bytes too; the size check's own
  proof is npt-proto's a3 test (1,151 B refused by its size).
- **Public items beyond §4.1's rows** (A5, RR-8): re-derived from the merged tree for T6 (u4e_pub_items.py;
  .gate/u4e-merge/pub-items.log): every `pub` item U4e adds is one of plan.md §4.7's own (plan.md:596-600) or in T6's
  §4.1 rows, each with its reason in its row's last column: npt-room 100 names (34 of them `ClientCounters`' fields,
  named as a group), npt-server 41 (23 `Counters`' fields), npt-client 22 (6 `StepOut`'s fields). Made pub(crate) in the
  fix pass: `Roster::new`, `Roster::is_bound`, `Decoder::limits`, `Decoder::is_joined`, `ConnLimits::join_deadline_ns`,
  `ClientEndpoint::closed`, `ClientEndpoint::unknown_pongs`, `ClientSlot::skipped`. `ClientEndpoint::with_lead` stays
  public for U4r (carry 74). The six types plan §4.7 lists whose methods are all pub(crate) (`AckRing`, `Encoder`,
  `EventQueue`, `InterestGrid`, `AreaFlood`, `Priority`) are public in name only. The fix pass also added two public
  fields to public counter types, the counters' own names the review asked for: `ClientCounters::resyncs_limited` and
  `Counters::joining_dropped`.
- **The Resync limit (SEC-U4E-4's room half; NC2-M1, Medium; carried).** A client's `Resync` is answered at most once
  per `npt_sim::FULL_STATE_ACK_TICKS` ticks after the last full state the room sent it; a sooner one is dropped with
  no reply and counted (`resyncs_limited`), and the catalogue's `Resync` row says so since the merge commit. For ADR
  0001's message table, which proposes "one in flight" as `Resync`'s rate limit (ADR 0001:688): round 1's probe Q-6
  showed a client that acknowledges each full state drawing 29 in 30 ticks under that rule, about 8 times its
  downstream window, so the proof test adds the time limit. As built, the limit leaves an honest `Resync` soon after a
  full state unanswered and starts afresh on a re-join (SEC-U4E-R2-1); its form is revisited at U6, before -19's runs
  and the boundary spike's PM-0004-61 (C-U6-16 (a)). No U4 mark meets it.
- **Round 2's Lows** (each routed; none pushed on its own). The client's two too-old cases pinned only by what the
  judge counts (NC2-L1, TE2-L1), and carry 61 (b)'s "two different hashes for one tick give the sentinel" failing in
  one order U4e's room never sends (a snapshot, then a full state of another hash for that tick: RR2-2): carry 80 (d)
  and (e), U4f; carry 61 (b) is met for every order U4e's room sends. Case 3's "once" holds while the client remembers
  the last 32 bad-header ticks (RR2-3): marks/README.md §1's sentence says so. The limit's 30 Hz tick count
  (SEC-U4E-R2-3), `take_control`'s bytes carrying no channel (SEC-U4E-R2-4, A-R2-1) and SEC-U4E-1's untested
  wrong-direction case (TE2-L2): C-U6-16 (b) to (d). The catalogue's `Resync` row (SEC-U4E-R2-2): closed in the merge
  commit. `send_control` dropping an encode error unseen, unreachable today (A-R2-2), and carry 50's test's "none at
  256" assertion that cannot fail alone (RR2-1): follow-ups on the card.
- **Open security findings, for PR #40's report** (each left open keeps it needs-mustafa): SEC-U4E-4's transport half
  (Medium; C-U6-12), SEC-U4E-2 (Low; C-U6-12), SEC-U4E-5 (Low; carry 76 and C-U6-14), SEC-U4E-6 (Low; slice 3),
  SEC-U4E-8 (Low; carry 81), SEC-U4E-9 (Low; carry 80), SEC-U4E-R2-1, SEC-U4E-R2-3 and SEC-U4E-R2-4 (Low; C-U6-16).
  Fixed in U4e: SEC-U4E-4's room half, SEC-U4E-1 (its wrong-direction case's test: C-U6-16 (d)), SEC-U4E-3 and
  SEC-U4E-7; SEC-U4E-R2-2 by the merge commit's catalogue line.

## Checks on the merged tree
- **M1:** the copy's HEAD 97bb9d32aa7b…; `git status --untracked-files=all` lists only the 23 files under crates/room,
  crates/server and crates/client and the record; `sha256sum -c .gate/u4e/r2-hashes-start.sha256` 23 of 23 OK (the
  same list as fx-hashes-end.sha256).
- **M2:** `git diff --quiet 97bb9d3 HEAD -- ` the six crates (room, server, client, sim, proto, link): true at
  2521d67; main's merge bfda3cd moved nothing under spikes/, and U4d2 has not merged.
- **M3:** worktree_patch.py's snapshot of the copy (.gate/u4e-merge/snapshot.patch) equals 4300944's wip/u4e.patch
  (cmp; sha256 1c31eeee…; 23 files); `git apply --check` clean; applied in the main folder.
- **M4:** `sha256sum -c .gate/u4e/r2-hashes-start.sha256` from the main folder's root: 23 of 23 OK.
- **M5:** T1, T4 and T7 applied first by u4e_merge_text.py (phase A); then check.mjs counted through main-run.sh
  (.gate/u4e-merge/m5-counted.log): the two `--no-run` builds exit 0, their Compiling lines naming the main folder's
  crates; "check: all 10 steps pass": the test step measured 473 and bites 541 passed, 0 failed (481.8 s), self 52 of
  52, marks 81 read and checked with the marked ADR rows unchanged, redact 255 items with 0 findings; the three by-name
  runs exit 0, e7's print line equal to the fix pass's (.gate/u4e/fx-t5print.log); the by-name lines are in the
  rulings section above, each fill's log line named by u4e_t5_fill.py (.gate/u4e-merge/t5-fill.log).
- **M6:** no failure in any run; nothing recorded under TE2-5.
- **M7:** `node tools/gate.mjs` (main's gate since bfda3cd): every check PASS but smoke (5a), which could not start
  this container's Chromium (tools/'s Playwright expects a headless shell that is not installed here:
  .gate/u4e-merge/gate-full.log); with `--no-smoke`: GATE PASS (.gate/u4e-merge/gate-nosmoke.log). CI's gate and smoke
  run on the pushed head.

## Carries
- **Met** (the builder's return list, item 6, with the rulings): 21, 22, 23, 24 (under the brief's note: -45 and the
  cues from the replay are U4g's), 25, 26 (one copy a tick; the seam as it stands: the rulings section), 27, 39, 50,
  51, 53, 54, 55, 56 (measured: the rulings section), 57, 61 (for (b), every order U4e's room sends: carry 80 (e)),
  and 68 (in the fix pass: NC-M2).
- **Added by the merge commit:** 73 to 77 (§U4r), 78 and 79 (§U4g), 80 (§U4f) and 81 (§U4h) in u4s-carries.md; entry
  62 (h)'s setup check reads the measured window (the advisor's round-1 ruling, R5, NC-L10); C-U6-12 to C-U6-16 in
  plan.md's U6 row.

## Follow-ups (Lows; none triggers a push)
- A-R2-2: `ClientSlot::send_control` drops an encode error unseen (room.rs:327-332), unreachable today; RR2-1: carry
  50's test's "none at 256" assertion cannot fail on its own (room/tests/room.rs:627). Both on the card (the advisor's
  round-2 ruling, Q2), with RR-6's follow-up gaining the limit's 30 Hz site (room.rs:728, SEC-U4E-R2-3) and A9's
  gaining SEC-U4E-R2-4 and A-R2-1 (server/tests/common/mod.rs:47).
- SERIES.md's heading says "## Series (235)" while its table holds 236 rows since T4 added
  `event.snap_refused.tick_count`. No check reads the number, and M8 lets the commit's spikes/ part hold T1 and T4 and
  nothing else, so the heading is corrected at the next change to marks/, with the advisor's word.
- Round 1's Lows routed as follow-ups: as the advisor's round-1 ruling lists them (advisor-u4e-r1.md, R5), on the card.
- An automated security review (Claude Code's security-guidance plugin, during the merge, 2026-11-12) flagged twice
  that `rewind_at` (room/src/rewind.rs) trusts the client's claimed view time within the history's span (Medium). It is
  SEC-U4E-8's point: carried to U4h's tolerance (carry 81, with carry 28's provisional 2 s span); no change in the merge
  commit, whose spikes/ part is the reviewed bytes (M8).

## Decisions needed
None.

## For Mustafa
Nothing. PR #40 stays needs-mustafa for its existing reasons; U4e's open security findings are listed in the rulings
section above, each Low but SEC-U4E-4's transport half (Medium), each with its carry.
