# C62H-BOUND: the build brief (revised after the advisor's and the test-engineer's check)

Written by the netcode-architect on 2026-11-15 (about 22:00 UTC), read only, from the branch head 0457c85 (its
spikes/ tree equals 442a284's). It carries Mustafa's yes, the frozen texts, the design and every ruling into one set of
instructions for one builder. It decides nothing new, except the three points B-1 to B-3 in section 11, which the
advisor has ruled on; this revision (same day) applies its conditions 1-7 and the test-engineer's TE-CBB-1 to -12.

## 0. What binds, in this order

1. **Mustafa's words** (history.md, verbatim): C62H-BOUND (a) at :2140-2146 (exact limit; wording fixed before the
   judged runs; "the bar is not reworded to fit them: stop and tell me"; (c)'s report lines stay until it is built);
   C62H-BOUND-YES (a) at :2163-2175 (yes to texts v3, paste sha256 cfac7502...; about 650 changed lines, stop past
   1,050; any change after his yes comes back to him before the judged runs; the P0 re-judge; "A changed judge must
   give the same answers on a set it was not changed for"; "The stop rules of your post stand. The check is never
   reworded to fit."); his 13:35 rule at :2113-2129 (a reviewer's read of every marks.json or judge change, named in
   its own report line).
2. **The freeze:** c62h-bound-freeze.txt (sha256 9cb8be172eaa7360...), over c62h-bound-texts-v3.md
   (3f21c9b7e5d5116d...).
3. **The advisor's rulings:** advisor-c62h-bound-r2-full.json (te_rulings, replacements, planned_size, other_changes
   item 6), then advisor-c62h-bound-full.json (C1-C11; Q1 (a), Q2 (a), Q3 (a), Q4 (a), Q5 (b)); on this brief,
   advisor-c62h-bound-brief-full.json (ADV-CBB-1 to -7; conditions 1-9: B-1 yes, B-2 in commit 3, B-3 confirmed).
4. **The test-engineer's read:** c62h-bound-test-full.json (TE-CB-1 to -10), as the r2 ruling took it; on this brief,
   c62h-bound-brief-test-full.json (TE-CBB-1 to -12: -1 to -4 applied; -5 to -12 applied, or deferred in section 11).
5. **The design:** c62h-bound-design-full.json (lab_record, judge, explanations E1/E2, bites, judged_runs, lane).
All files are in /home/user/Shamble-Frontlines/.checkpoints/netcode-proof-test/. Where they differ, the higher wins.

**In plain words:** on L1's lossy profiles every snapshot a test client files as lost in the measured ten minutes needs
a reason from a record the client does not write: the room sent it nothing (E1, `hs.server_hash` null) or npt-l1 lost a
snapshot datagram of it (E2, a new room-log series `tick.s.snap_link_lost_count`). Two judge figures count the rest
(must be 0) and the converse (a tick filed whole that the link lost a datagram of: must be 0). Two planted bugs prove
it.

## 1. Base and lane

- **Risk tier:** 1 (spike code, its tests and marks under spikes/netcode-proof/, on PR #40; ADV-CBB-7). Its reviewers
  are the round's three (section 5); no tier-2 path is touched.
- **Base commit:** 442a28426c3933c1deaeac9466ef3f57dca96c81 (spikes/netcode-proof tree
  6ca9f77fc44486cceacd57ff11e4a29f66066e7f), the merged head the advisor approved (717b802,
  advisor-merged-head-full.json) plus the first pack's list commit b5cb253 and card commits. **LANE_BASE** is the
  branch head when the build starts. Check first, from the main folder: `git diff --quiet 442a284 <LANE_BASE> -- .
  ':(exclude).checkpoints' ':(exclude)spikes/netcode-proof/results'` exits 0, and any change under results/ is a
  kept-logs `raw-logs.sha256` file only. Write LANE_BASE in the notes.
- **The builder: route W, one agent launch per commit step** (ADV-CBB-1; there is no route M). Each launch is one
  agent (general-purpose, opus, effort xhigh), in `.claude/worktrees/c62h-bound`, which the main session makes by hand:
  `git worktree add -b c62h-bound-build .claude/worktrees/c62h-bound <LANE_BASE>`. Each launch is made with NO
  isolation (the Agent tool's and the workflow's isolation base on main: the card's BRAWL-1 lesson) and is told the
  worktree path. Its runner:
  `bash .checkpoints/netcode-proof-test/u4r-run.sh -x .claude/worktrees/c62h-bound <log> <cmd>`.
  - **A launch** does one commit step (section 3), its check.mjs, its plants and its SIZE-CHECK (section 4), and ends
    at a clean point with the step's files staged. It commits nothing and never edits marks/. A fix pass is one more
    launch.
  - **The main session**, between launches: commits that step in the worktree (`cd` there, then `git commit`), checks
    its diff hash and records its SIZE-CHECK (section 4), then starts the next launch.
  - **The insert script** (section 2) is the main session's, run on the worktree: the built check's shown-to-fail run,
    then part 1, before step 1's launch (B-1); part 3 and the built check before step 3's launch, which then does
    B-2's fixture files and check.mjs.
  - The gate runs at the merge step on the cherry-picked head (a worktree has no tools/node_modules:
    tools/gate.mjs:75). At the end the main session cherry-picks the commits onto the branch (section 6).
- **Runners, unchanged, sha256 checked before first use:**
  - u4r-run.sh e9437ae200d234725b392bc5090e2b802d9938bb8ba5b12c23123c3d607e97d7;
  - main-run.sh 8df2dee311a6f00d147b876be787313e54900309625cfbabd12059aac86d0ae6;
  - peak.py dfacdcc0ad51fbde42668293509c541eed009a8d53243d9df3572a59318a31ac;
  - the set runner spikes/netcode-proof/ops/runs/c-u4r-runs.mjs is
    99dd070eb20ffced3e7dcd7a892b2456027817a6c81287f6857831c990ebd09b at LANE_BASE; commit 4 changes it, and the sets
    run on the new one (its sha256 recorded).
- **Logs:** every lane log under /home/user/Shamble-Frontlines/.gate/u4r/c62h-bound/ (u4r-run.sh accepts only
  .gate/u4r/; main-run.sh accepts it too). The sets' logs under .gate/u4r-sets/ as before.
- **Notes:** .checkpoints/netcode-proof-test/c62h-bound-build.md: the plan first, a line after each item, a "## State"
  block at any clean stop. A restart reads them and the diff before it changes anything. The card is updated each step.
- **The files.** The planned size's 11 rows name 10 files (scenario.rs twice, code and tests), all under
  spikes/netcode-proof/: crates/l1/src/lib.rs, crates/l1/src/tests.rs, crates/lab/src/scenario.rs,
  crates/lab/src/logs.rs, crates/judge/src/figures.rs, crates/judge/tests/engine.rs, marks/marks.json, marks/SERIES.md,
  marks/README.md, ops/runs/c-u4r-runs.mjs. **No other file**, except the four judge-fixture files of **B-2** (ruled
  in: section 11).
- **Not in this lane:** JDG-ONLY-1 (C5: its own change-set, plan first; never on the judge crate at the same time as
  this lane), so no change to crates/judge/src/main.rs, eval.rs, marks.rs or tests/cli.rs; no change to npt-sim,
  npt-room, npt-client, npt-proto, npt-server, npt-bots, check.mjs or check.test.mjs; -58's wrap parts (Q4 (a)); U4f;
  U4g.
- **No other lane meanwhile** (C11): nothing else edits l1/src/lib.rs, l1/src/tests.rs, lab/src/scenario.rs,
  lab/src/logs.rs, judge/src/figures.rs, judge/tests/engine.rs, marks/, ops/runs/c-u4r-runs.mjs (or B-2's files) from
  LANE_BASE until the P3 set's results are committed. C62H-BOUND goes before U4f's build and U4g's judged B1 runs.
- **Four commits**, in this order, each ending with the session's attribution lines (Co-Authored-By, Claude-Session):
  1. `C62H-BOUND 1/4: npt-l1's server_datagrams_since and npt-lab's link record (tick.s.snap_link_lost_count)` (the l1
     read and test; the lab join, record, tally, plant and tests; with **B-1**, SERIES.md's series row and header);
  2. `C62H-BOUND 2/4: npt-judge's two C62H-BOUND figures, one shared reader, their fixtures`;
  3. `C62H-BOUND 3/4: PM-0001-17's two checks and one planted bug, SERIES.md and README.md, word for word from texts
     v3` (with **B-2**'s judge fixtures, ruled in);
  4. `C62H-BOUND 4/4: the set runner plans -17's whole-as-lost and link-lost-overstated on P2`.
  A review fix pass, if any, is one more commit: `C62H-BOUND fix pass: <finding ids>`.
- **Disk before the build:** free 5,899 MiB now; shared target 6,043 MiB; the release cache
  .gate/netcode-proof-test/target 776 MiB. If free is under 6,000 MiB when the build is about to start, pack
  u4r-P2-ab10f058a828 first (section 8.1's procedure; about 560 MiB back). Never pack during the build.

## 2. The frozen texts: in by script, checked by script

The texts are never retyped. The main session writes the scripts named in this brief in the notes folder (text, run
with `python3 -I` from the repository root) and runs them; the route-W agent never edits marks/.

**c62h_bound_insert.py `<root>` `--part 1|3`** (`<root>`: the main folder or the worktree). First, every time:
- texts-v3.md's sha256 is 3f21c9b7e5d5116db7cae7c0d8d9a4011d2ab9797c75b907f00554f0996844eb and freeze.txt's is
  9cb8be172eaa73608ecee5b46e2dc5cfd44e8c2b3a26ca7a74bffb5ef4e82950;
- each block, parsed with the freeze script's own regex (c62h_bound_freeze.py), has the sha256 freeze.txt gives it;
- each target file has its LANE_BASE sha256 (marks.json 9fb5be578b27d4c3..., SERIES.md 3411eee7f2b93981...,
  README.md b49f3441f86721ab...) or, for part 3, SERIES.md's part-1 sha256 below.
Every anchor must match exactly once; the script refuses to run twice; it prints each file's sha256 before and after.
- **Part 1 (commit 1, B-1):** SERIES.md: `## Series read only by figures (18)` becomes `(19)`; the block
  `series-row-tick.s.snap_link_lost_count` goes in as the line after the one starting ``| `tick.s.relevant_count` |``.
  Expected SERIES.md after: 89295b57d5c9ceec4e9790d8466b1ea5981215b242865e93f68872bb529774fc (numstat +2 -1).
- **Part 3 (commit 3):**
  - marks.json: in PM-0001-17, the object holding `"name": "L1 P0 loses no group (no client-tick left incomplete)"`
    ends with `        }` just before `      ],` and `      "report": [`; that line becomes `        },`, followed by a
    newline and the block `check-objects`. The whole-as-lost entry ends with `        }` just after `"what": "a client
    writer that files a tick it decoded whole as lost: null and incomplete 1"`; that line becomes `        },`, followed
    by a newline and the block `bite-object`. Then `json.loads` of the result must equal the base's with exactly those
    two checks and that bite added. Expected marks.json after:
    c1935a5b996fb0aaff82b0b3c1c604255e1534f6ca7e8d2e55a9f472cd3b22a9 (git numstat +26 -0: a minimal diff keeps each
    closing brace and inserts the `},` lines; the frozen "+26, 2 changed" is the same text).
  - SERIES.md: `## Figures (36): computations npt-judge carries in code` becomes `(38)`; the blocks
    `series-row-snap_lost_unexplained_count` and `series-row-snap_whole_despite_link_loss_count` go in, in that order,
    after the line starting ``| `session_memory_growth_count` |`` (the next line must start
    ``| `sync_cost_ratio` |``); the line starting ``| `tick.s.snap_incomplete` |`` is replaced by the block
    `series-row-tick.s.snap_incomplete`. Expected SERIES.md after:
    6898e119109a794a7779ad0395301e48a461848b60cfdc16431ec64faad87460 (numstat from LANE_BASE +6 -3).
  - README.md: the block `readme-sentence` goes in after `never the sentinel. ` on the one line (README.md:67) that
    holds `never the sentinel. That covers`; then lines 67 to 77 (to the line before ``  - `replay_hash` ``, the end of
    the `decoded_hash` bullet) are refilled with Python's `textwrap.fill(width=120, initial_indent="    ",
    subsequent_indent="    ", break_long_words=False, break_on_hyphens=False)` over their words joined by single
    spaces. Expected README.md after: f3c3c804c4e4d078ca07461f956eb60bc96e74eb4ed507bf28f652383a5742fe (numstat +16
    -11; the word diff is one insert of the sentence's 73 words; no line over 120).
  These four expected hashes were computed in memory from texts v3 and the LANE_BASE files by this algorithm; the
  script must reproduce them. A different hash stops the build, and the advisor reads why before anything else.

**c62h_bound_built_check.py `<root>`** (C9; TE-CB-3): run by the main session right after part 3 (before step 3's
launch), after step 3's commit, after any fix pass, and on the merged head before the P0 re-judge and before each set;
output kept in .gate/u4r/c62h-bound/built-check-<head12>.log. It:
- re-checks texts v3 and the freeze (as above) and prints the paste's sha256 (cfac7502b4a4ba9e...);
- shows each frozen block once, whole, in its file: `check-objects` and `bite-object` in marks.json; the four row
  blocks in SERIES.md, each a whole line, with the two header lines `(38)` and `(19)`; `readme-sentence` in README.md
  with whitespace collapsed (the reflow may break it across lines);
- shows the three files' sha256 equal the expected values above, and prints them;
- runs `git diff --word-diff=porcelain <LANE_BASE> -- spikes/netcode-proof/marks/README.md` and shows its only added
  words are the sentence's, with nothing removed;
- prints marks.json's sha256 "after", which each set's runs.txt must print (c-u4r-runs.mjs:475) and the report quotes;
- prints PASS or FAIL per check and exits 1 on any FAIL.
**Shown to fail first** (TE-CBB-1): before part 1, the main session runs it once on the worktree as LANE_BASE left it
(log .gate/u4r/c62h-bound/built-check-negative.log, kept): its re-check of texts v3 and the freeze passes; every block,
header line, file hash and the word-diff check prints FAIL; it exits 1. Any of those passing stops for the advisor.

**Any change to a frozen text after his yes** (a check's name, figure, parts or value; the bite's id or text; the four
SERIES rows; the README sentence; or what either figure computes as its row states it), a review finding included,
goes back to Mustafa for a yes before the judged runs. Nobody rewords a check to fit a run.

## 3. The build items

### 3.1 Commit 1: npt-l1's read and npt-lab's link record

1. **npt-l1** (crates/l1/src/lib.rs, beside FateLog at :325-398): `pub struct DatagramCounts { pub sent: u32, pub
   lost: u32, pub burst: u32 }` (derive Debug, Clone, Copy, Default, PartialEq, Eq; each field documented) and `impl
   FateLog { pub fn server_datagrams_since(&self, from: usize) -> DatagramCounts }`: over the entries from index
   `from` to the end (none when `from` is past the end), only those with `carries == Carries::Datagram` and `dir ==
   Dir::Down` (server to client: send_dir, :245-247): `sent` each, `lost` each `Outcome::Lost`, `burst` each
   `Outcome::Burst` (decided at the push, :1097-1104). Read only: no draw, no change to the canonical form (:340-398).
   Its doc says so, and that a datagram sent on a link that is not up, or too large, pushes no entry (:1084-1089).
2. **Its test** (crates/l1/src/tests.rs), `c62hb_server_datagrams_since_counts_down_datagrams_by_fate`: one link on
   B1 (`Profile::b1`, a burst in the window) and a forced loss (`Forced::Datagram { dir: Dir::Down, seq }`); down
   datagrams before the window starts (so `from` matters); in one window, down datagrams delivered, lost and
   burst-lost, plus up datagrams, and down and up stream pieces and acks (TE-CBB-7 (a)). Its expected `sent`, `lost`
   and `burst` come from an independent count over `for_each_canonical`'s entries from index `from` (byte 4 direction
   1, byte 5 flow code 0, byte 6 kind 1 lost or 2 burst: :340-398), never from the function's own output (B1 also
   loses about 2 % at random). It asserts the independent `lost` and `burst` above 0, the function's three equal to the
   independent count for the down datagrams only, and all 0 for a window with no down datagram. Its assert that the
   canonical bytes are equal before and after the call stays as a statement: it cannot fail (`&self`).
3. **The join, one pure function** (crates/lab/src/scenario.rs, near Server; TE-CB-1, C7): `pub(crate) fn
   link_lost(groups: usize, has_session: bool, server_hash: Option<u64>, c: DatagramCounts) -> Result<u32, String>`:
   - no session and `groups > 0`: Err "{groups} snapshot groups for a slot with no session";
   - `c.sent` other than `groups` (no session: other than 0): Err "{groups} snapshot groups, but the send pushed
     {c.sent} server-to-client datagram fates";
   - `groups > 0` and no server hash: Err "{groups} snapshot groups with no server hash";
   - else Ok(`c.lost + c.burst`).
   Five tests in scenario.rs's tests, each asserting its own Err text or value: `c62hb_join_one_datagram_too_many`,
   `c62hb_join_one_datagram_too_few`, `c62hb_join_groups_with_a_null_server_hash`, `c62hb_join_groups_with_no_session`,
   `c62hb_join_ok_counts_lost_and_burst` (Ok(2) from sent 3, lost 1, burst 1).
4. **The call** in `run`'s per-slot loop (:967-1017), for every slot at every room tick, logs or not. Before
   `server.send(slot, ...)` (:989): `let from = net.with_fate_log(|l| l.len());`, `let has_session =
   server.session(slot).is_some();`, `let hash = server.room.client(slot).and_then(|c| c.server_hash());`. After it:
   `let c = net.with_fate_log(|l| l.server_datagrams_since(from));` and `link_lost(datagrams.len(), has_session, hash,
   c)`, its Err wrapped as `fail(format!("slot {slot}, tick {t}: {e} (C62H-BOUND's link record cannot be joined)"))`.
   The run then returns Err and its logs end without footers, which the judge reads INVALID (README §4). The value goes
   into a per-tick `Vec<u32>` of n entries (0 for a slot never sent to).
5. **Measured totals** (Summary, :739-761): `link_sent`, `link_lost`, `link_burst` (u64), summed over room ticks with
   `phase(k) == Phase::Measured`, from the counts `c` (never from the written values).
6. **The plant** (bites builds only: `#[cfg(feature = "bites")]` code, so the measured binary holds none: RR3-L4):
   - `LAB_BITES` (:204) becomes `[&str; 3]` with `"PM-0001-17/link-lost-overstated"` **appended** (index 2:
     precheck.rs:312 reads index 1);
   - `Bites` gains `link_lost_overstated: bool` (documented as whole_as_lost is), set in `Bites::of` by
     `LAB_BITES[2] == id` and added to `any` (:234-252);
   - in the loop, per slot, at the first measured room tick with `c.sent >= 1` and `c.lost + c.burst == 0`, the
     **written** value is 1; the totals keep the counts.
7. **The record** (crates/lab/src/logs.rs): `room_tick` (:495) gains `link_lost: &[u32]` (after `inputs`) and writes
   `rec.s["snap_link_lost_count"]` with n entries (padding with 0) on every room tick record. Keep the written list per
   wide tick beside `logged` (:160-161): inserted in room_tick, removed in write_client_tick where `logged` is removed
   (:420), and dropped at `finish` as `logged` is. The two test call sites (:903, :915) pass `&[]`.
8. **The tally** (TE-CB-2; report only, never judged): in write_client_tick, keep what `self.logged.remove(&w)` and
   the new map's remove return. When `ph == Phase::Measured`, each slot with `snap_incomplete` 1 adds to exactly one of
   Counts' new `lost_room_sent_nothing` (E1: no `logged` flag), `lost_link` (E2: flag set and written value >= 1) and
   `lost_unexplained` (N: flag set and value 0). So E1 + E2 + N = `incomplete_measured`, and N is figure A's count.
9. **The summary phrase** (summary_line, :1612-1690), right after `decided lost at the stop {}`, exactly: `; measured
   snapshot datagrams {S}, lost by the link {L} (in bursts {B}); measured client-ticks filed lost: the room sent nothing
   {E1}, the link lost a datagram {E2}, neither {N}`. S, L and B come from Summary (printed on every run); E1, E2 and N
   go through `logged(...)` (a --logs 0 run prints "not counted (--logs 0)"). `overflow resyncs N` stays where it is.
10. **The short P2 lab test** (scenario.rs tests; 16 clients, 6 s warm-up, 4 s measured: `--seed 1 --profile P2
    --clients 16 --warmup-s 6 --measured-s 4 --after-max-s 10 --marks PM-0001-17`, R1_ARGS' form at :1756-1757).
    `c62hb_p2_short_every_measured_lost_tick_is_explained` reads its room and client logs and asserts:
    - every room tick record holds `snap_link_lost_count` with 16 entries, none below 0;
    - the measured sum of it is above 0;
    - a test-side join written from the two frozen SERIES rows (npt-lab does not depend on the judge) gives 0
      unexplained and 0 whole-despite-loss;
    - Counts gives E1 + E2 + N = `incomplete_measured`, E2 above 0, and N = 0 = the join's unexplained count.
    Bites build only: `c62hb_p2_short_bite_whole_as_lost_gives_16_unexplained` (join A 16, B 0; N 16) and
    `c62hb_p2_short_bite_link_lost_overstated_gives_16_whole_despite_loss` (join B 16, A 0; N 0; and Summary's L equal
    to the clean short run's L, that run made in the same test with the same arguments: L from the counts, never the
    written values: TE-CBB-7 (e)). An unexplained tick in the clean test is a stop rule, never a test to adjust
    (section 10).
11. **Module texts:** logs.rs:1-8 names `tick.s.snap_link_lost_count` among the room log's series; scenario.rs:1-27
    gains one paragraph: the link record (C62H-BOUND), read at each slot's send from npt-l1's fate log, checked one to
    one against the groups, the run failing otherwise.
12. **B-1** (ruled yes): the insert script's part 1 on SERIES.md (section 2), run by the main session on the worktree
    before step 1's launch. Without it the lab's contract test r2 fails with "(b) tick.s.snap_link_lost_count has no
    SERIES.md row" (contract.rs:120-145, :465-513).

### 3.2 Commit 2: the judge's two figures

1. **One shared room-side reader** (crates/judge/src/figures.rs, after `decoded`), used by both figures, with one gap
   list (C2, TE-CB-5; the r2 condition). It takes `clients` from the room logs' headers (one value, else a gap, as
   decoded at :328-343); `hs.server_hash` and `hs.decoded_hash` through `slot_hashes` (:236-279; its own refusals
   stand); `tick.s.snap_link_lost_count` from every room tick record; and the client logs (`source("decoded_hash")`,
   :224-233), **measured tick records only** (`t.ph == Phase::Measured`, as at :28 and :503). It walks the room tick
   records itself for the room-side gaps (slot_hashes skips a record without the key: :249). Gaps, each a `Lack::Gap`
   naming the run, and the tick and slot where there is one, in the frozen row's order; each of the reader's own notes
   holds its gap's phrase quoted here, the frozen row's words, and no other gap's (TE-CBB-2):
   - "no room log gives tick.s.snap_link_lost_count";
   - "a room tick record without tick.s.snap_link_lost_count";
   - "a room tick record without hs.server_hash";
   - "a list that does not hold the run's clients entries", naming the list (`hs.server_hash`, `hs.decoded_hash`,
     `tick.s.snap_link_lost_count`, `tick.s.snap_incomplete`; slot_hashes' refusals keep their own texts);
   - "an entry below 0";
   - "a measured client tick record without tick.s.snap_incomplete";
   - "a snap_incomplete entry other than 0 or 1";
   - "a measured incomplete tick the room log has no record of".
2. **`snap_lost_unexplained_count`:** each measured (slot, tick) with `snap_incomplete` 1 counts when that slot's
   server hash there is non-null and its `snap_link_lost_count` there is 0. It returns the count (`Ratio::int`), never
   Empty.
3. **`snap_whole_despite_link_loss_count`:** each measured (slot, tick) with a `decoded_hash` neither null nor 0 counts
   when the room's record gives that slot `snap_link_lost_count` >= 1; a tick with no room record is skipped. It never
   returns Empty.
4. **Two FIGURES entries** (:632-826, after `decoded_hash_compared_count`), unit "C62H-BOUND", their `what` exactly as
   the design's judge text: "client-ticks filed lost that neither the room's record (no snapshot sent) nor the link's
   (a snapshot datagram lost) explains (-17)" and "client-ticks filed whole at which the link's record lost a snapshot
   datagram: the record overstated (-17)".
5. **Fixtures** (crates/judge/tests/engine.rs, beside hash_mark and hash_log at :1296-1335), on a fixture mark with the
   two figure checks (== 0) on one L1 part:
   - `c62hb_each_explanation_alone_reads_0`: an incomplete measured tick with a null server hash and link loss 0 (E1
     alone); one with a non-null server hash and link loss 1 (E2 alone) (TE-CBB-7 (b));
   - `c62hb_an_unexplained_tick_reads_1`: incomplete, server hash set, link loss 0: A 1 (fail), B 0;
   - `c62hb_warmup_and_after_phase_ticks_are_not_read` (TE-CB-6): an incomplete tick (hash set, loss 0) in the warm-up
     and one in the after phase: A 0; a whole hash at a tick with loss >= 1 in each of those phases: B 0; both asserted
     as the pair (A, B) = (0, 0) in one assert (TE-CBB-7 (c));
   - `c62hb_the_converse_and_the_sentinel`: a measured whole hash with loss 1: B 1; the sentinel with loss 1: B 0; and
     a run whose only such tick is a measured whole hash at a tick with no room record: valid, B 0, gaps empty (the
     frozen row: "not read here"; TE-CBB-8);
   - `c62hb_eight_gaps_read_on_both_figures`: each case makes the run INVALID and **both** checks' outcome
     `Outcome::Gap` (RunOut.checks: eval.rs:1083-1095), each check's `note` holding that case's own phrase from item 1
     (TE-CBB-2): the series missing from every room log; a room tick without it, the other room tick records holding
     it (its note holds the second phrase and not the first: the shadowing pair split); a room tick with it and no
     `hs.server_hash`, with an incomplete measured tick there (C2); a `tick.s.snap_link_lost_count` list one entry
     short (the short-lists fixture already pins hs.server_hash's length gap); a negative entry; a measured client
     record without `snap_incomplete` (TE-CB-5); a measured incomplete tick with no room record; a `snap_incomplete`
     entry of 2 (the frozen row's eighth gap: about +8 lines over the plan's seven);
   - `c62hb_a_bite_with_an_unexplained_tick_is_shown`: valid, A fail 1, B pass 0, gaps empty, status Shown.

### 3.3 Commit 3: the marks, the rows, the sentence

1. The main session runs the insert script's part 3 (section 2) on the worktree, then the built check, both outputs
   kept, before step 3's launch (section 1); that launch does item 2 and check.mjs.
2. **B-2 (ruled in):** crates/judge/tests/fixtures.rs judges crates/judge/fixtures/pm-0001-17/ with the **real**
   marks.json on -17's L1 P2 part. With the two checks there, three cases change: `base` reads INVALID (both new
   figures gap: its room log has no series); `lost-group` reads FAIL (slot 1's tick 2: A 1); `hashed-and-incomplete`
   reads FAIL (A 1) where the test wants INVALID. The proposed edits (about 40 lines):
   - base/p2-room.jsonl: each of its three tick records gains `"snap_link_lost_count"` with 48 zeros in `s`;
   - lost-group/p2-room.jsonl (new): base's room log with slot 1's entry at tick 2 set to 1 (E2 explains the case);
   - tests/fixtures.rs: `hashed-and-incomplete` leaves the INVALID list of
     `pm_0001_17_every_server_hash_is_resolved_on_the_client_side`; a new test asserts L1 P2 FAIL, a why line holding
     "(0 unexplained) = 1, needs == 0", and the run's `gaps` holding "slot 1, tick 2: a client hash and
     tick.s.snap_incomplete 1 at once" (the coverage gap is still found, beside the failure);
   - fixtures/README.md rows :22, :23, :32 say so (base: the room log's series 0 on every slot; lost-group: the link
     lost it, E2; hashed-and-incomplete: FAIL, A 1, the coverage gap beside it, INVALID before).
   `none-compared` also reads A 6 and stays FAIL: no edit. The other -17 cases do not change. No other existing test's
   assertion is edited; any other existing judge test or fixture whose answer changes stops the build for the advisor
   before any edit (ADV-CBB-5).

### 3.4 Commit 4: the set runner (C6)

1. **bitesOf** (ops/runs/c-u4r-runs.mjs:306-318): a bite runs on the profiles its part reads. -17's
   `PM-0001-17/whole-as-lost` runs on P0 and P2; `PM-0001-17/link-lost-overstated` on P2 only, never P0; every other
   -17 and -57 bite on P0 as today; -18's none. Its doc comment says this.
2. **SPLIT_LINE** (:166) names no profile: `(read its L1 P0 parts;` becomes `(read the set's parts;`; nothing else.
3. **The header comment:** at :29, `-17's and -57's on P0,` becomes `-17's and -57's on P0, and -17's whole-as-lost
   and link-lost-overstated on P2 (C62H-BOUND; link-lost-overstated never on P0),`; at :34, `(only P0: bitesOf)`
   becomes `(P0 and P2: bitesOf)`. The lines are refilled within the file's width.
4. **The recorded dry print** (TE-CB-9 in C6): a scratch harness .gate/u4r/c62h-bound/harness/plan-dry.mjs (untracked,
   not counted; its sha256 written into .gate/u4r/c62h-bound/harness/sources.txt before its run and never edited
   after: ADV-F2H-1), run through u4r-run.sh -x, importing the runner by path and reading the built marks.json.
   It prints planJudged, runBytes and judgeRoom for P0, P2, P3 and B1, and asserts (exact values, as the runner prints
   them: ADV-CBB-6, TE-CBB-6):
   - P0: the nine run names in order, l1-p0-s1, l1-p0-s2, l1-p0-s3, bite-tick16, bite-client-rounding,
     bite-whole-as-lost, bite-pos-4cm, bite-yaw-12bit, bite-pitch-8bit (3 measured + 6 bites), no
     link-lost-overstated, 3,615 MiB as 93509dfd7fd1's runs.txt printed;
   - P2: the five run names in order, l1-p2-s1, l1-p2-s2, l1-p2-s3, bite-whole-as-lost, bite-link-lost-overstated
     (C6), 930.7 MB = 887.6 MiB, judge 2129 MiB (measured) and 552 MiB (bites);
   - P3: 3, no bites, 549.9 MB = 524.4 MiB, judge 1530 MiB; B1: 3, no bites, judge 2253 MiB; every call under
     10,240 MiB;
   - SPLIT_LINE holds none of P0, P2, P3, B1;
   - `overflowFlag` on a summary line carrying the new phrase and `overflow resyncs 27`, with marks [PM-0001-17],
     returns the flag; with `overflow resyncs 0` it returns "".

## 4. The checks after each step

- **check.mjs whole**, all 11 steps PASS, 0 failed, run by the step's launch: `bash
  .checkpoints/netcode-proof-test/u4r-run.sh -x .claude/worktrees/c62h-bound .gate/u4r/c62h-bound/step<N>-check.log
  node check.mjs`.
  - The launch stages the step's files first (`git add <files>`), so the header's diff hash covers the new files.
    After the main session's commit, `git diff <step>^ <step> -- . | sha256sum` (from the worktree's
    spikes/netcode-proof) must equal that diff hash; the main session checks it.
  - Record the measured and bites test counts and their change from LANE_BASE's 538 and 616. Expected changes
    (TE-CBB-4): step 1 +7 / +9 (the l1 test, the five join tests and the short P2 test in both builds; the two short
    P2 bite tests in the bites build only); step 2 +6 / +6 (the six engine.rs tests: TE-CBB-2's and TE-CBB-8's cases
    are asserts inside them, no new test function); step 3 +1 / +1 (B-2's new fixtures.rs test); step 4 0 / 0; a fix
    pass 0 / 0 unless it adds a named test. Final 552 / 632. A different count stops for the advisor. The notes list
    each new test's name from the test runner's own output, in each build it runs in.
  - Pass or fail is the exit code and the test runner's own summary, never a grep.
  - `cargo fmt --all --check` runs first in it: apply rustfmt's diff by Edit, never `cargo fmt` writing files.
- **The gate:** `node tools/gate.mjs --no-smoke` in the main folder at the merge step, on the cherry-picked head, GATE
  PASS. The reason for --no-smoke: the lane changes nothing the smoke test loads; CI runs the smoke test on the pushed
  head.
- **SIZE-CHECK** (ADV-CBB-2): `git diff --numstat <LANE_BASE> <step> -- spikes/netcode-proof/
  ':(exclude)spikes/netcode-proof/results'`, added plus deleted, summed, recorded per file, run by the main session in
  the worktree after each commit, the fix pass included. Each launch first runs the same count on its staged work
  (`git diff --cached --numstat <LANE_BASE>` with the same pathspecs) before it ends; the two must be equal. The total
  is the figure at the last code commit (commit 4 or the fix pass), stated as such in the report. Committed verdicts
  and runs.txt under results/ are evidence: named in the report, never counted. Planned by commit, with this
  revision's test additions: 1 about 285-400; 2 about 255-355; 3 about 100 (60, README alone 27 by its reflow, plus
  about 40 for B-2's four files, counted); 4 about 10-20; in all about 650-875 against the plan's 650 (545-755).
  **Past 1,050** the build stops, and Mustafa gets three lines: what grew, why, and the cheaper option (section 10).
- **The planted bugs that must bite.** Each is a temporary Edit, the file's sha256 recorded before and checked equal
  after its restore by Edit; each runs through u4r-run.sh -x as `cargo test` of the named test in the build it
  needs; the result is the test runner's own "1 failed" (or 2) naming that test.
  - CB-P1 (step 1): server_datagrams_since counts `Dir::Up` too. Home:
    c62hb_server_datagrams_since_counts_down_datagrams_by_fate. Expected: fails, `sent` above the independent count.
  - CB-P2 (step 1): link_lost drops the `c.sent` comparison. Home: c62hb_join_one_datagram_too_many and
    c62hb_join_one_datagram_too_few. Expected: both fail, Ok where Err.
  - CB-P3 (step 1): link_lost drops the server-hash check. Home: c62hb_join_groups_with_a_null_server_hash. Expected:
    fails, Ok where Err.
  - CB-P4 (step 1): link_lost drops the no-session check (C7). Home: c62hb_join_groups_with_no_session. Expected:
    fails, Ok where Err.
  - CB-P5 (step 1): the record written as 0 on every slot (understated). Home:
    c62hb_p2_short_every_measured_lost_tick_is_explained. Expected: fails at its assert that the measured sum is above
    0, which reports 0 (TE-CBB-10).
  - CB-P6 (step 1): the record written one room tick late (misaligned). Home: the same. Expected: fails, unexplained
    or whole-despite-loss above 0.
  - CB-P7 (step 1): the tally reads `logged` after its removal. Home: the same. Expected: fails, E2 is 0.
  - CB-P8 (step 2): the shared reader's window "not warm-up" in place of "measured". Home:
    c62hb_warmup_and_after_phase_ticks_are_not_read. Expected: fails, its pair assert reports (A, B) = (1, 1).
  - CB-P9 (step 2): the reader takes a missing `hs.server_hash` as null (no C2 gap). Home:
    c62hb_eight_gaps_read_on_both_figures. Expected: fails, A 0 where a gap is wanted.
  - CB-P10 (step 2): figure B counts the sentinel. Home: c62hb_the_converse_and_the_sentinel. Expected: fails, B 1.
  - CB-P11 (step 4): bitesOf plans link-lost-overstated on P0 too. Home: the dry-print harness. Expected: fails, P0
    has 7 bites.
  - CB-P12 (step 1): server_datagrams_since drops the `carries == Carries::Datagram` filter. Home: the l1 test.
    Expected: fails, `sent` above the independent count (TE-CBB-7 (a)).
  - CB-P13 (step 1): server_datagrams_since ignores `from` (reads from index 0). Home: the l1 test. Expected: fails,
    `sent` above the independent count (TE-CBB-7 (a)).
  - CB-P14 (step 2): figure A drops E1 (a null server hash no longer explains). Home:
    c62hb_each_explanation_alone_reads_0. Expected: fails, A 1 (TE-CBB-7 (b)).
  - CB-P15 (step 2): figure A drops E2 (link loss 1 or more no longer explains). Home: the same. Expected: fails, A 1
    (TE-CBB-7 (b)).
  - CB-P16 (step 2): the reader drops gap 1 ("no room log gives ..."). Home: c62hb_eight_gaps_read_on_both_figures.
    Expected: fails, the first case's note lacks its phrase (TE-CBB-2).
  - CB-P17 (step 2): the reader drops gap 2 ("a room tick record without ..."). Home: the same. Expected: fails, the
    second case's note lacks its phrase (TE-CBB-2).
  - CB-P18 (step 2): the reader drops the length check of `tick.s.snap_link_lost_count`. Home: the same. Expected:
    fails at the one-entry-short case (TE-CBB-2).
  The judged planted bugs (marks.json's) are read in the P2 set (section 8.4): whole-as-lost 48 on A,
  link-lost-overstated 48 on B. The lab tests' bites (16 and 16) run in every check.mjs's bites step (their names in
  the test runner's output: TE-CBB-4).

## 5. The review round (one; C11, TE-CB-10, his 13:35 rule)

- After commit 4 and its checks: **test-engineer, netcode-architect and rust-reviewer**, each at effort xhigh, at most
  3 agents at a time (the builder's last launch has ended first). Each reads the diff `<LANE_BASE>..<commit 4>` (full
  SHAs), texts v3, the freeze, this brief, the design and the rulings, read only, with notes in the notes folder.
  - **test-engineer, the marks.json and judge read:** its notes name the marks.json lines read (the two checks and
    the bite, by line in the built file), the judge lines read (figures.rs and engine.rs ranges; B-2's fixtures.rs and
    fixture files), marks.json's sha256 before (9fb5be578b27d4c3...) and after (c1935a5b996fb0aa...), and
    `git diff --stat <LANE_BASE>..<head> -- spikes/netcode-proof/marks/marks.json spikes/netcode-proof/crates/judge`
    at the close. It reads SERIES.md and README.md too, and checks that every test can fail (section 4's plants) and
    the expected lines. Its notes name B-2's changed fixture answers (section 9) and that no other existing judge
    test's assertion moved (ADV-CBB-5).
  - **netcode-architect:** the join and its checks, E1 and E2 against the frozen rows, the figures against the rows,
    the tally, the runner, the expected lines.
  - **rust-reviewer:** the l1, lab and judge Rust: no HashMap order, no float, no overflow (u32 sums, u64 totals),
    docs on every pub item, clippy in both builds, no bite code in the measured build; the call site's Err handling
    (the join's Err fails the run: no `unwrap_or`, no default value; TE-CBB-7 (d)).
- **The fix pass:** one, within the review budget (a second round only on what changed; a third only for an open
  Critical or High). A fix touching marks.json or the judge gets a read of what changed. A fix to a frozen text, or to
  what a figure computes, goes to Mustafa before the judged runs (section 2). Each finding is fixed or deferred in
  writing.

## 6. The merge step and the head check

1. **The merge:** check that each lane commit touches only spikes/netcode-proof/ (`git show --stat`); from the main
   folder, `git cherry-pick <c1> <c2> <c3> <c4> [<fix>]` in order; then `git diff --quiet c62h-bound-build HEAD --
   spikes/netcode-proof ':(exclude)spikes/netcode-proof/results'` exits 0 (merged bytes equal reviewed bytes). Never a
   `git merge` of the build branch, never a switch of the main folder.
2. **One counted main-run.sh call** on the branch head (log .gate/u4r/c62h-bound/merge.log; build ruling 1: det-u4 at
   every merge that touches the scenario path), its commands in order: `cat /proc/uptime`; `node check.mjs`; `df -B1
   /home/user`; `uptime`; `node ops/runs/det-u4.mjs`; `df -B1 /home/user`; `cargo build --locked --release -p
   npt-lab`; `bash -c "time /home/user/Shamble-Frontlines/.gate/shared-target/release/npt-lab hash --manifest
   crates/lab/precheck-recordings.json --dir /home/user/Shamble-Frontlines/.gate/u4r/c62h-bound/merge/rec --out
   /home/user/Shamble-Frontlines/.gate/u4r/c62h-bound/merge/out"`. Pass:
   - header diff and untracked e3b0...; all 11 steps;
   - det-u4 PASS with no non-cargo sum over 240 s (else the advisor, whole, never retried);
   - "regenerated SS-P0-s4: 16950444 bytes, sha256
     1f00d2d5f70a2c7b6836ede611b5f1c281319a5312060ead91e016da3c0b1b42, as the manifest's" and "9000 ticks compared, 0
     mismatching, equal". The lane changes no simulated behaviour, so every pin holds.
3. The gate --no-smoke; the built check (section 2); push; CI green on that head; **the advisor's check of the pushed
   head** (naming the full SHA). Only then the P0 re-judge and the sets.

## 7. His P0 re-judge (a judge call, no new runs), before the sets

- **The kept P0 set is 93509dfd7fd1** (.gate/netcode-proof-test/runs/u4r-P0-93509dfd7fd1: 1,547 MiB measured/, 1,846
  MiB bites/). Why, from the records:
  - it is the P0 set that counts (advisor-brawl1-sets-p0-full.md: "P0 STANDS AND COUNTS"; BRAWL-1's re-run sets P3,
    P2 and P0 all count);
  - the design's judged_runs keep its verdicts ("93509dfd7fd1's verdicts stand");
  - 051995db9cea is U4r's P0: its -17 L1 P0 part is "recorded, never counted, never re-run"
    (advisor-u4r-sets-r2-full.md:10), and its -2 SS part failed (0.861889).
  **Both folders stay unpacked** through this lane. The report names this reading, so Mustafa can name the other
  folder (its re-judge is one more judge call).
- **The earlier answers:** spikes/netcode-proof/results/u4r/93509dfd7fd1/P0/verdict.json (sha256 e926b812cbe57d81...)
  and verdict-bites.json (b472784f2c7208ae...), byte-equal to the raw folder's copies; judge 65acacfe...; marks
  9fb5be57...
- **The calls,** in one main-run.sh call (log .gate/u4r/c62h-bound/p0-rejudge.log), every path written out whole, with
  R = /home/user/Shamble-Frontlines/.gate/netcode-proof-test/runs/u4r-P0-93509dfd7fd1 and O =
  /home/user/Shamble-Frontlines/spikes/netcode-proof/results/c62h-bound/p0-rejudge-93509dfd7fd1/<head12>, in order:
  - `df -B1 /home/user`; `cargo build --locked --release -p npt-judge`; `mkdir -p <O>`;
  - `sha256sum /home/user/Shamble-Frontlines/.gate/shared-target/release/npt-judge marks/marks.json`;
  - `python3 -I <peak.py> <shared-target>/release/npt-judge --marks marks/marks.json --logs <R>/measured --only
    PM-0001-2,PM-0001-17,PM-0001-57 --out <O>/verdict.json`;
  - the same with `<R>/bites` and `<O>/verdict-bites.json`; `df -B1 /home/user`.
  Projected peaks 3,921 and 4,621 MiB (93509dfd7fd1's runs.txt), under 10,240. These are the kept set's two judge
  calls, measured/ then bites/, as that set was judged (U4r-F2's memory split: c-u4r-runs.mjs:35-41), with no new
  runs; the report says so (ADV-CBB-4).
- **The comparison,** by c62h_bound_rejudge_compare.py (notes folder; it only reads). It first checks the earlier
  files' sha256 (verdict.json e926b812cbe57d81e251bfb5c6685b889a4d2859e9cac25022f6788bf74af095, verdict-bites.json
  b472784f2c7208aeef38af373a8bc257bf0b3b88a18dc5dc376fb6533536942e) and refuses on any other (TE-CBB-1). Each parsed
  new file must equal the earlier one after removing exactly (i) -17's new bite entry `{"id":
  "PM-0001-17/link-lost-overstated", "status": "not_run", "runs": []}` right after whole-as-lost, and (ii) exactly one
  -17 why line `bite PM-0001-17/link-lost-overstated: no run yet` in each file, after whole-as-lost's in verdict.json
  and last in -17's why list in verdict-bites.json (ADV-CBB-4, TE-CBB-5), each removal asserted to match its form. It
  prints each part and bite that holds a run, new beside earlier (setParts' form: PM-0001-2 SS runs PASS; -17 L1 P0
  PASS; -57 L1 P0 PASS; six bites shown), then SAME ANSWERS or every difference. **Any other difference stops the lane
  before the sets** and goes to Mustafa whole (his "A changed judge must give the same answers on a set it was not
  changed for").
- **Shown to fail first** (TE-CBB-1): before the comparison, its `--self-test` (output kept in
  .gate/u4r/c62h-bound/rejudge-compare-negative.log and quoted in rejudge.txt) reads the earlier files and builds in
  memory a copy with the two expected additions, which must give SAME ANSWERS, and three planted copies: one -17 L1
  P0 check value changed (it must print that difference), the new bite entry twice (it must refuse), and
  verdict-bites.json without the new why line (it must print a difference). Any other result stops for the advisor.
- Commit `<O>` (the two verdicts, and rejudge.txt written with the Write tool: the commands, both sha256s, the peaks,
  the comparison's output and its self-test's) after the spike's redact step (`node check.mjs --only redact`) passes
  with 0 findings.

## 8. The judged runs: P2, then P3

### 8.1 Disk and kept logs

- The runner's own start check: free minus the set's projection at least 4,096 MiB, before the builds and after them.
  P2 projects 887.6 MiB (start 4,983.6 MiB), P3 524.4 MiB (4,620.4 MiB). Each release build wants 4 GiB free.
- **Before each set** (never during a build or a set): free at least 5,500 MiB before P2 and 5,100 MiB before P3 (the
  start figure plus room for the release builds). If lower, pack the next folder of this list, one at a time, until it
  holds: u4r-P2-ab10f058a828 (779 MiB), u4r-B1-64f3307d5c7f (821), u4r-P3-0f066338dd8d (519). Each is U4r's, closed by
  advisor-u4r-sets-r2-full.md:45 ("U4r's judged runs close once the report and the card carry this record. There is
  no further run at any U4r set commit."; closed at 6b22cab).
- **Each pack** as the first one (advisor-merged-head-full.json, conditions): cite that ruling; pgrep finds no cargo,
  rustc, npt-lab, npt-judge, check.mjs or runner; free at least the folder plus 1,024 MiB; `node
  .claude/scripts/kept-logs.mjs list <folder> --results spikes/netcode-proof/results/u4r/<c12>/<P>`; commit only its
  raw-logs.sha256, naming the ruling, and push; `node .claude/scripts/kept-logs.mjs pack <folder> --results <same>`;
  its free-before, free-after and read-back lines on the card. Exit 1 or a refusal: stop, tell Mustafa, and pack
  nothing more (§11).
- **Never** pack either P0 folder (both stay), fc411116c1f1's or fd4612050cd6's (this lane's reference sets), or the
  c-u2 folders. The list done and still short: stop, the advisor.

### 8.2 Before each set

One main-run.sh call (log .gate/u4r-sets/pre-<P>-c62h.log): the header's diff and untracked hashes e3b0...; HEAD and
its spike tree; `git diff --quiet <checked head> HEAD -- . ':(exclude)results'` exits 0 from spikes/netcode-proof;
`git diff --name-status <checked head> HEAD` shows only .checkpoints/ and A lines under results/; sha256sum of
c-u4r-runs.mjs (the lane's), main-run.sh, u4r-run.sh and peak.py (section 1); pgrep empty; df. Save the card first:
the runner refuses any uncommitted tracked change ("commit first").

### 8.3 The sets

- **Launch** from the repository root, CARGO_TARGET_DIR unset, in the background: `python3 -I
  .checkpoints/netcode-proof-test/peak.py flock -x .gate/shared-target/.lane.lock node
  spikes/netcode-proof/ops/runs/c-u4r-runs.mjs judged --profile P2 --date <YYYY-MM-DD> >
  .gate/u4r-sets/P2-<c12>.log 2>&1`. About 1,735 s (P2) and 1,020 s (P3).
- **P2:** 3 measured runs (l1-p2-s1 to -s3) and 2 bite runs at seed 1 (bite-whole-as-lost, bite-link-lost-overstated),
  each with 48 clients, SS, 10 s + 600 s, tick_start 63,736; two judge calls (verdict.json, verdict-bites.json).
- Then commit P2's results (results/u4r/<c12>/P2/) with the card, run 8.1 and 8.2 again, and run **P3** (3 measured,
  no bites, one judge call) the same way. Then commit P3's results.
- **After each set,** c62h_bound_set_check.py (notes folder; it only reads; output into .gate/u4r-sets/) checks every
  expected line below, PASS or FAIL each. A FAIL on a stop rule goes to Mustafa (section 10); any other FAIL goes to
  the advisor, whole, before the set counts.
- **Shown to fail first** (TE-CBB-1): before its first real use (after P2's set), it runs once on
  results/u4r/fc411116c1f1/P2 (no new phrase, no new checks: every line naming them must FAIL), and its `--self-test`
  runs on an in-memory copy of P2's new verdict-bites.json whose whole-as-lost run gains one gap (the planted-bug rule
  must FAIL); output kept in .gate/u4r-sets/set-check-negative.log. Any of those passing stops for the advisor.

### 8.4 The expected lines (TE-CB-7 with the advisor's amendments)

- **Both sets:** runs.txt's `marks.json sha256` equals the built check's "after" sha256 (c1935a5b996fb0aa...;
  TE-CB-3). The set's npt-judge sha256 is recorded beside the re-judge's; they come from two build caches (the main
  folder's own and the shared target), so they need not be equal.
- **P2, verdict.json:** `PM-0001-2 SS runs: PASS`, `PM-0001-17 L1 P2: PASS` and `PM-0001-57 L1 P2: PASS`, each
  `(l1-p2-s1 valid, l1-p2-s2 valid, l1-p2-s3 valid)`; on each run, read per check: snap_lost_unexplained_count 0 pass,
  snap_whole_despite_link_loss_count 0 pass, decoded_hash_mismatch_count 0 pass; gaps empty.
- **P2, verdict-bites.json, read per check** (eval.rs:1475-1478: "shown" means any check failed):
  - `PM-0001-17/whole-as-lost: shown (bite-whole-as-lost valid)`: snap_lost_unexplained_count 48 fail,
    snap_whole_despite_link_loss_count 0 pass, mismatches 0 pass, gaps [];
  - `PM-0001-17/link-lost-overstated: shown (bite-link-lost-overstated valid)`: snap_whole_despite_link_loss_count 48
    fail, snap_lost_unexplained_count 0 pass, mismatches 0 pass, gaps [];
  - -57's bites are not run on P2.
- **P2, the summary lines:** each clean line, without the new phrase and the wall time, equals its old line whole
  (results/u4r/fc411116c1f1/P2/runs.txt:24-26; no crate behaviour changed):
  - fate logs 911d96e802b38294 / 6f4d3bf75a875a92 / 819c74ce1c037ef1; measured incomplete 17,260 / 17,089 / 17,532;
    client hashes 861,996 / 862,011 / 861,731; decided lost at the stop 30 / 24 / 23; stale 0; sentinels 0; overflow
    resyncs 0;
  - the new phrase: S, L and B reported (L near 2 % of S is a reported value, not a bar: ADV-CBB-6); the bar is B 0,
    E1 + E2 + N = measured incomplete, and N 0;
  - the bite runs' lines (TE-CBB-3), each compared with this set's l1-p2-s1 line field by field after stripping the
    run name (the line's first field: scenario.rs:1621-1624), the new phrase and the wall time (the P0 precedent:
    93509dfd7fd1/P0/runs.txt:37 against :42 differ in exactly those and the two whole-as-lost fields):
    - bite-whole-as-lost differs in exactly two fields: client hashes s1's minus 48 (861,948) and measured incomplete
      s1's plus 48 (17,308); its new phrase's S, L, B, E1 and E2 equal s1's, and N is 48;
    - bite-link-lost-overstated differs in no field (incomplete 17,260, hashes 861,996); its new phrase's S, L, B, E1,
      E2 and N (0) equal s1's (L from the counts, never the written values);
    - any other differing field goes to the advisor first.
- **P3, verdict.json:** `PM-0001-2 SS runs: PASS`; `PM-0001-17 L1 P3: PASS (l1-p3-s1 valid, l1-p3-s2 valid, l1-p3-s3
  valid)` with both figures 0 pass, mismatches 0 and gaps empty on each run; `PM-0001-57 L1 P3: PASS`.
- **P3, the summary lines** equal fd4612050cd6/P3/runs.txt:24-26 the same way: incomplete 43,295 / 43,210 / 43,454;
  overflow resyncs 27 / 0 / 1; stop 87 / 62 / 59; fate logs b99096c62a5c51dd / 7dbd92ec2098f6da / f167ce6f8dfc3c09;
  client hashes 835,238 / 835,419 / 835,088. s1 and s3 carry the carry-58 flag again: they go to the advisor before
  they count. s1's E1 is above 0 (its 27 overflow resyncs: condition 2 on a real run); if not, the report says
  condition 2 is shown by the fixtures only (not a stop). E1 + E2 + N = incomplete and N 0 on each run.
- **B1:** nothing now: -17's L1 B1 part is INVALID by pre:PM-0001-2 until U4g; U4g's judged B1 runs carry both checks.

## 9. The report

- **Its lines:** the four commits and the fix pass, with SHAs; SIZE-CHECK per step and its total at the last code
  commit (commit 4 or the fix pass), against about 650, the size line naming B-2's four files beyond the paste's table
  and the committed verdicts and runs.txt as evidence, not counted (ADV-CBB-2, condition 5); each step's check.mjs
  counts against section 4's expected changes, with each new test's name (TE-CBB-4); each plant CB-P1 to CB-P18
  bitten; the merge step's log; the advisor's head check; the built check's output (the three "after" hashes, the
  freeze re-checked); the three shown-to-fail runs (built check, compare, set check: TE-CBB-1); the P0 re-judge's
  verdicts beside the earlier ones and its SAME ANSWERS line, named as the kept set's two judge calls, as that set was
  judged, with no new runs (ADV-CBB-4); both sets' expected lines, each PASS or FAIL; P2's and P3's S / L / B and
  E1 / E2 / N per run; the kept logs packed (free before and after); the runners' and peak.py's sha256s.
- **Its own line for the marks.json and judge read** (his 13:35 rule): who read it (the test-engineer, its notes file),
  marks.json 9fb5be578b27d4c3... before and c1935a5b996fb0aa... after, the lines read, and the `git diff --stat` at the
  close; the same for any fix-pass read. Beside the P0 SAME ANSWERS line it names B-2's changed fixture answers, each
  with its why (ADV-CBB-5, TE-CBB-12): hashed-and-incomplete INVALID to FAIL, A 1, its coverage gap kept beside the
  failure (on -17's L1 P2 part, which the checks were made for); base and lost-group, their room logs gaining the
  series, answers unchanged; none-compared still FAIL, now A 6; and no other existing judge test's assertion moved.
- **(c)'s report lines** (u4r-report.md's two sub-lines "Against the profile's loss (C62H-REVIEW; ...)", under -17 L1
  P3 at :996 and under -17 L1 P2 at :1007) give way, by a script asserting each matches once, **only after both sets
  PASS** and the advisor has ruled on P3's flagged runs. Each becomes: "Bounded by C62H-BOUND (his words of
  2026-11-15, 16:35 and 20:50): snap_lost_unexplained_count 0 / 0 / 0 and snap_whole_despite_link_loss_count 0 / 0 /
  0 (results/u4r/<c12>/<P>/verdict.json); the lab's tally E1 / E2 / N per run; the planted bugs 48 and 48 (P2's
  verdict-bites.json)". **B1's line stays** ("-17 L1 B1 stays with U4g's judged B1 runs", :1034) until U4g.
- **Still unbounded,** in the report: -58's two wrap parts (L1 P0 and L1 P3), offered to Mustafa as a later decision
  now that both sets pass (C62HB-Q4 (b)); -17's L2 parts (no network record to explain a loss); -17's L1 B1 part until
  U4g; outside each run's measured ten minutes; a double fault (a wrong record at exactly the tick a client bug files
  lost), narrow but not zero.
- **The kept P0 set:** one line naming 93509dfd7fd1 and why (section 7), and that 051995db9cea is still on disk.
- The report ends with "Decisions needed" (the -58 offer) and "For Mustafa".

## 10. The stop rules

**His, word for word** (ADV-CBB-3): the post's stop rules (c62h-bound-paste.md:120-124), which he adopted ("The stop
rules of your post stand. The check is never reworded to fit.", history.md:2175), and its size line (:89), under his
yes to "the build stops and tells me past 1,050" (history.md:2172).

> - The work stops and comes to you if:
>   - any clean run or test (the lab's short P2 test included) shows a lost tick that nothing explains;
>   - the lab's own count of unexplained ticks differs from the judge's;
>   - a planted-bug run reads anything other than 48 on its own check, with every other check passing and no gap.
> - The check is never reworded to fit.

> SIZE-CHECK: if `git diff --numstat <lane base>..HEAD -- spikes/netcode-proof/` passes 1,050 changed lines (added
> plus deleted) of C62H-BOUND alone, the build stops and goes to Mustafa in three lines: what grew, why, and the
> cheaper option. The line stays at 1,050: about 1.6 times the plan and 1.4 times the top of its range.

**Also to Mustafa, from his other words:** a frozen text, or what a figure computes, would change after his yes
(section 2; history.md:2173); the P0 re-judge gives any answer other than the earlier one (section 7; history.md:2174).

**The advisor's reading of them** (advisor-c62h-bound-brief-full.json, condition 3; TE-CBB-9), which the builder and
the set check apply; any other reading on a planted-bug run goes to Mustafa:
- "a lost tick" is a measured one: outside each run's measured ten minutes "no lost tick is checked, as the wording
  says" (paste :105). So figure A above 0, or the lab's N above 0, on any clean run or test stops for him.
- "the lab's own count" is the summary's N ("neither"), and "the judge's" is figure A, on every run, planted-bug runs
  included.
- A planted-bug run stops for him unless all of these hold (r2 TE-cond-6, a conjunction, never the trigger's
  condition): the run is valid; its own figure reads exactly 48 (fail); the other new figure and
  decoded_hash_mismatch_count read 0 (pass); its gaps list is empty; the lab's N equals the judge's figure A.
- The size line counts without spikes/netcode-proof/results (evidence), after each code commit, the fix pass
  included; its total is the figure at the last code commit (ADV-CBB-2; section 4).

**The advisor's. Stop and report whole, never retried or cleaned around, if:** a built hash differs from section 2's;
a summary field differs from section 8.4's; a bite run's line differs from s1's in a field section 8.4 does not allow;
any run meets a join Err; a floor stops a runner; det-u4 shows over 240 s of non-cargo time; a check.mjs step fails
that no edit of this lane explains; a test count differs from section 4's expected change; a shown-to-fail run passes
(sections 2, 7, 8.3); an existing judge test or fixture answer changes beyond B-2's named ones (section 3.3); the
context runs low or a cut comes (stop at a clean point: no plant in place, no half edit, no run in flight; write
"## State" in the notes).

## 11. Rulings on this brief (the advisor: advisor-c62h-bound-brief-full.json)

- **B-1 (commit grouping; no text changes):** the lab's contract test r2 (contract.rs:465-513) fails on any name the
  logs write without a SERIES.md row. So commit 1, which writes `tick.s.snap_link_lost_count`, must carry that frozen
  row and the `(18)` to `(19)` header (the insert script's part 1); commit 3 carries the rest of SERIES.md.
  Recommended: yes. Ruled: yes, as written (condition 4); SERIES.md after part 1 is 89295b57..., which the advisor
  reproduced.
- **B-2 (four files beyond the 11 rows, about 40 lines):** section 3.3 item 2. The real-marks fixtures of -17's L1 P2
  part cannot pass once marks.json gains the checks; `hashed-and-incomplete` moves from INVALID to FAIL (A 1), with its
  coverage gap still found beside the failure. This is no frozen text and no change to what a figure computes: it is
  the approved checks' reading of an existing fixture, so it falls under the marks.json and judge read. Recommended:
  in this lane, in commit 3. The other way, leaving those room logs without the series so the cases stay INVALID by a
  gap, would hide the new checks from the fixtures; not recommended. Ruled: in, in commit 3 (condition 5): its lines
  count in SIZE-CHECK and the report's size line names the four files; the test-engineer's 13:35 read covers
  fixtures.rs and the fixture files; the report names hashed-and-incomplete's move as the only changed fixture answer;
  no other existing test's assertion is edited, and any other changed answer stops for the advisor.
- **B-3 (the kept P0 set, MH-L1):** this brief reads it as 93509dfd7fd1 (section 7) and keeps both folders until
  Mustafa has the report. Recommended: confirm. Ruled: confirmed (condition 6); both P0 folders stay unpacked until
  the report has reached Mustafa, and the report names this reading.
- **The order** stands as written (condition 8): the P0 re-judge after the pushed head's check and before P2.
- **Deferred in writing** (TE-CBB-11): (i) re-judging both P0 folders: not done, since the advisor confirmed B-3; the
  other folder's re-judge stays one more judge call Mustafa can name (section 7). (ii) The design's "old P2 and P3
  logs re-judged under the new marks.json read INVALID" (design bites[3]) gets no judge call on old logs: the gaps
  fixture's first case (section 3.2) shows the same reading (room logs without the series: INVALID on both figures,
  with gap 1's phrase); the report says the claim is shown by that fixture, not by old logs.
- **Noted, no ruling:** README's numstat is 27 (the reflow rewrites 11 lines), not "about +4 to +8"; marks.json's is
  +26 -0, not "2 changed"; the eighth gap fixture adds about 8 lines. The estimate, about 650-875 (section 4), stays
  under the 1,050 stop; its top passes the plan's range (545-755), and the report states the final total against both.
