# Change-set 3b, "The loop closes": what a reviewer needs (round 2)

Written by the main session (Builder V3). Read `review-context.md` (round 1) first if you have not: its rules for
every reviewer all stand. This file says what changed since.

## What is reviewed
- **The head of round 2: commit `HEAD2`** on the branch `claude/loving-hopper-6p0mg2`. Round 1's head was
  `bbbf09e4`; the base is main `28254a73`. **What changed since round 1:** `git diff bbbf09e4 HEAD2 -- crates
  packages` (the fix pass, one commit). Round 2 is on what changed: each fix against its finding, and anything the
  fix itself broke. A finding of round 1 that was answered with "follow-up" or "report" is not reopened unless the
  answer is wrong.
- **What happened to round 1:** engine-architect, netcode-architect and rust-reviewer finished it. A restart of
  the container at about 01:32 UTC on 2026-11-25 stopped test-engineer, security-auditor and perf-auditor before
  their answers. Their notes (`<name>-review1.md` in this folder) hold what they had found; all of it went into the
  fix pass. **Those three finish their own plans in this round, on the fixed head** (the plan items their notes do
  not show as done), and check the fixes of what they had found.
- **Every finding of round 1 and what was done about it:** the resume card `.checkpoints/cs3b-loop-closes.md`,
  section "Open findings".

## The fix pass, by finding (the code is the source of truth; these are pointers)
| Finding | What changed |
|---|---|
| engine-architect Medium (the block writer and the newest post in `sim-wasm`) | `client_endpoint::Endpoint` owns the post and the block (`new(hash, first_sequence, pad_bytes) -> Option`, `set_post`, `block`, `block_len`, `write_block(refused_elsewhere)`); `sim-wasm/src/local_match.rs` keeps the room, the endpoint, the link, the schedule and `unsent` |
| engine-architect Lows | `layout::end_of` is the one place a section's end is worked out; the generator prints `BLOCK_BYTES`; one owner of "the newest post" (the host; `Post::newer_than` is gone); `KeyboardInput.key(.., typing)` lets a key-up count in a text box; the host time's check is gone (`step(_now_ms)`); the boundary test pins `Sim`'s own calls |
| netcode-architect Medium (late snapshots counted as refused) | `Readout.late`; `refused` keeps bytes that are no snapshot |
| netcode-architect Lows | `CommandQueue::insert` opens its window with the first command offered; the newest command always goes (`Endpoint::build_input`); `press.rs` states its bound; the `Join`'s version before its token (`codec.rs`) |
| rust-reviewer Lows | one `assert!` with a plain message in `LocalMatch::step` (the Wasm lost 2,934 bytes); all four lints in `post.rs` and `press.rs`; `#[must_use]`; one record per button in `press.rs`; four more cases of the cast from f64 (10,072 cases); **`crates/sim-wasm/tests/any_bytes.rs`**: the room and the endpoint against valid, changed and random messages, their rules asserted at every step |
| test-engineer High (the presses mark's second bite, planted whole, passed) | the page counts ticks that ran an older post than the tick before (`WorkerLink.olderPosts`) and buttons held again with no press (`EchoTally.heldWithoutPress`, shown as `releases.doubled`); `presses.spec.ts` and `loop.spec.ts` assert 0; the shim counts posts sent behind a newer one |
| test-engineer High candidate (releases are worked out, not counted) | as far as the echo allows: see `press-tally.ts`'s doc; the echo has no counter for releases; the report says so against the mark's wording |
| test-engineer High candidate (the allocation limit holds only on a slow page) | the list of returned buffers no longer empties (`WorkerLink.#done`); measured again at six frame rates (the table is in `allocation.spec.ts`'s header); the spec refuses by name a run with fewer than 2.5 blocks per frame, and its per-frame limit follows the blocks per frame |
| test-engineer Lows | a cap per message type in the decoder test's last check; whole-message cases for five acks and nine commands in `codec.rs` |
| security-auditor Medium M1 (no bound on how late a command may run) | **not built**: named in `Room::on_datagram`'s doc with the other checks still to come (its Low L1); the main session's reason: the bound is part of the command-timing choice (ADR 0001's open question 6, option (b): "late commands dropped"), which Mustafa gave to Slice 3 on 2026-11-24, and nothing moves in 3b. Put to the advisor agent with this head. Say if the reason does not hold |
| perf-auditor (the key-to-tick reading began at the handler) | `boot.ts` takes the key event's own `timeStamp` |

## Evidence (the main session's; check what you rely on)
- Locally on the fix pass: the numbers in the resume card's "Current step". Planted defects: `plants-3b.py` (44),
  its last full run in the card.
- GitHub's run on the head: in the card once read.

## Rules that changed for round 2
- **Your own copy at the new head:** `cd /home/user/Shamble-Frontlines/.gate/ci-wasm-wt && git worktree add
  --detach .gate/<your-name>-cs3b2 HEAD2`, then work only there. Your round-1 copy (`.gate/<your-name>-cs3b`)
  stays as it is; do not delete folders.
- **Browser tests and page builds:** one reviewer at a time may run them, in its own copy, and only when its brief
  says so. Check that nothing listens on port 4173 first; never start one while another runs.
- **Notes:** `<your-name>-review2.md` in this folder (the main session's folder, not your copy's): your plan
  first, one line per finding as you find it, `DONE` as the last line. If you are stopped and started again, you
  continue from your notes.
- **Your answer:** at most 30 lines: `VERDICT: PASS` or `VERDICT: CHANGES NEEDED`; per round-1 finding of yours
  one line (`fixed`, `not fixed: why`, or `answer accepted`); then new findings with severity, `file:line` and the
  smallest fix you would accept; then `Not verified:`.
