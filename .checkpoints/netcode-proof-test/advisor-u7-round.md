# Advisor: U7's round ruling and the fix-pass brief (09:50 UTC window, 2026-11-26)
Advisor agent, day session, effort xhigh, read only (no cargo). Acting by main's advisor.md (read whole in this run).
Plan: read the packet, both reviews, the draft brief, then rule every finding (part 1) and write the brief (part 2)
before 10:15 UTC; parts 3-5 one line each. Notes at the end.

## Verdict: FIX PASS NOW (section 2's brief), then no second round
- The fix pass covers 5 Mediums (NC-1 with TE-2, TE-3 with NC-2, TE-4, T5ii-D = NC-7, TE-1) and the Lows in the same
  files. Each is proven by a test that is red first or by a plant that is caught.
- The second look is my read of the fix pass's snapshot (part 4), not a round.
- Two pass marks of U7's brief cannot be met as written: mark 9 and mark 8 (8b). They are his, by TEST-FIRST's way out:
  see "Decisions needed". The fix pass does not wait on them; calling T5 (ii) passed does.

## 1. Each finding ruled
| Id | Sev | Ruling | Where |
|---|---|---|---|
| U7R-NC-1 + TE-2 (a) | Medium | Fix now (F1) | engine.rs, RoomHost |
| U7R-TE-2 (b) | Medium | Fix now (F2) | worker.test.mjs |
| U7R-TE-3 + NC-2 | Medium + Low | Fix now (F3). The report says mark 9 NOT-RUN and labels the column. The room's per-slot refusal series is a carry to U6d (the room's observability, before R1's run plan). | script.rs, engine.rs |
| U7R-TE-4 | Medium | Fix now (F4) | check.mjs webStep, check.test.mjs |
| T5ii-D = U7R-NC-7 | Medium | Fix now (F5) | worker.ts, main.ts, the driver, probe |
| U7R-TE-1 | Medium | Closed by the one T5 (ii) rerun on the fixed tree (part 3) | the end |
| U7R-NC-3 (T5ii-B) | Low | Report wording: mark 8b "not met as written", with run 3's facts (5: 1.60 s; the bite: 4.06 s at Chromium's QUIC idle timeout). The restated bar is his (Decisions needed). Carry to R8's bite plan and to Stop B's read of `npt-ctl quic off`: the block must DROP, never REJECT; each browser's own timeout recorded at R8. | report; Stop B; R8 |
| U7R-NC-4 | Low | Fix now (rides: t5ii.mjs is touched for TE-10 and the rerun) | t5ii.mjs, t5i.mjs |
| U7R-NC-5 | Low | Fix now (rides: the page is touched for F5) | main.ts + one Node test |
| U7R-NC-6 | Low | Fix now (rides: lib.ts and worker.ts are touched for F5). It closes a possible false pass of -14's browser part. | lib.ts, worker.ts |
| U7R-NC-8 | Low | Report wording only | report |
| U7R-TE-5 (T5i-1) | Low | Carry, not in this pass: one WebSocket leg without ignoreHTTPSErrors before R1, at Stop B's fill by the main session. No mark reads the series. | Stop B |
| U7R-TE-6 | Low | Report wording only | report |
| U7R-TE-7 | Low | Fix now (rides: clock.rs and engine.rs tests) | clock.rs, engine.rs |
| U7R-TE-8 | Low | Fix now (rides: worker.ts), the four Node cases | worker.test.mjs, fakes.mjs |
| U7R-TE-9 | Low | Fix now: build.mjs scans the seven real dist files (rides the rerun's build) | build.mjs |
| U7R-TE-10 | Low | Fix now: `--commit` read by git or required; the dist build log's sha256 in summary.json; C8's pin limit in the report | t5ii.mjs |
| U7R-TE-11 | Low | Report wording (12 steps; 243,393 B; plant logs). The fresh-tree web step is met by U7's merge M5 in the main folder. Carries: client-wasm's clippy.toml in CLIPPY_TWINS, to check.mjs's next owner (beside CHECK-NFF-1); the limits test's copied literal, to the fix pass only if U6b's FULL_STATE_MAX is reachable through client-wasm's dev edges, else to U6d. | report; carries |
| T5ii-C | — | Closed (the flag is in place). The report records the one inadvertent CONNECT that named the real host to the session's proxy, with no token. | report |
| T5i-2 | — | Carry to U6d (Join-deadline closes in `deadline_closed_count`) | U6d |
| T5i-3 | — | Superseded by F5 | — |
| The shell-loop slip | — | Process. The brief repeats the rule, with no evidence affected. | brief |

## 2. The fix-pass brief (send word for word; the main session fills `<base>` and runs ffcheck first)
> **U7's fix pass** (advisor-u7-round.md, section 1's rulings; u7-review-netcode.md and u7-review-test.md for each
> item's detail). Your worktree .claude/worktrees/agent-a9873cdba07a3219e; your notes u7-build-rs.md, new section
> "## Fix pass".
>
> **0. Base.** `<base>` is the lane head at sending. Since 9177fb5 the lane changed only:
> - U8's 43 files under spikes/netcode-proof/ops/;
> - MAIN-67's main paths (.github/workflows/toolchain.yml, packages/e2e/playwright.config.ts, docs);
> - notes.
>
> Nothing changed under crates/ or web/, nor check.mjs, check.test.mjs, Cargo.toml or Cargo.lock. (`git diff --stat
> 9177fb5 80b6748` over those is empty, checked by the advisor at 09:5x.) The main session runs ffcheck first (0 ADDED
> present, 0 differing). In your worktree, touching no file first:
> 1. `git merge --ff-only <base>`. If git refuses, stop and report its message; remove, restore or move nothing.
> 2. `git log -1 --format=%H` into the notes.
> 3. Your new baseline, before any edit: `node check.mjs --only refs,marks,self,redact` through
>    `bash /home/user/Shamble-Frontlines/.checkpoints/netcode-proof-test/u7-run.sh -x <worktree> <log under
>    .gate/u7/>`. Record refs, marks, self and redact: they move with U8's files.
> 4. Your plan for the items below, before any edit.
>
> **1. The items.** For each, put in the notes the test that goes red before the fix or the plant it catches, then the
> green.
> - **F1 (NC-1 with TE-2 (a), Medium).** One RoomHost test with a 40 ms path each way and a 5 ms host send lag:
>   - the client's sends are stamped now + 45 ms, with prev_send_ns = now + 5 ms; the room's at now + 40 ms;
>   - assert Playing, no refusal, and the newest clock sample's RTT at 80 ms (within 1 µs).
>   - Plants: (i) engine.rs:207-209 (`sent(prev_send_ns)`) removed must fail it (85 ms); (ii) `now_ns` passed in place
>     of `prev_send_ns` must fail it. If the test cannot tell (ii) apart, the notes say why.
>   - The report says jitter and loss stay with the endpoint's lab suites.
> - **F2 (TE-2 (b), Medium).** worker.test.mjs's fake writer advances the FakeClock 1 ms per write; assert prevSendNs[i]
>   = nowNs[i-1] + the sends' time. Plant: stamping before the sends must fail it.
> - **F3 (TE-3 with NC-2, Medium).**
>   - Presses are counted from the commands issued: at most one edge per command (`script::command(self.commands,
>     newest)`).
>   - One in-process leg with late wakes: every 10th wake 200 ms late, one 1 s stall and one 3 s gap. Assert no
>     RefusalRecord and the slot's refusal counters at 0.
>   - Plant: the step-count press counter put back must fail it after the 3 s gap.
> - **F4 (TE-4, Medium).** The web step fails when its test count is 0. Add a check.test.mjs recorder case with
>   "# tests 0" that must fail.
> - **F5 (T5ii-D = NC-7, Medium).**
>   - The Worker and the page count `securitypolicyviolation` events into `page_csp_violations`, read by the driver
>     (mark 7 then reads both halves).
>   - Positive control: a page-side refusal in the probe must count 1.
>   - One Node test for the counter.
> - **F6, the Lows in the same files** (each with its test or plant):
>   - **NC-4:** `fail` SIGKILLs every child it started and removes the current tokens file. t5i.mjs's two `e.message`
>     lines go through `clean()`.
>   - **NC-5:** `history.replaceState` after a good parse, before the Worker starts. A Node test on a pure helper.
>   - **NC-6:** the partial second is written at `end()`, and `second` is armed on the Worker clock's grid. A test each.
>   - **TE-7:** the u64::MAX case runs under a step bound. One Core test gives a non-zero `refused` (a malformed datagram
>     in the inbox), read by name.
>   - **TE-8:** JoinAccepted alone writes no attempt.ok; a stuck writer; a 5th uni stream; bufferedAmount over the cap;
>     a fake with createWritable.
>   - **TE-9:** build.mjs scans the seven real dist files for `PM-0001-` and `innerHTML`.
>   - **TE-10:** `--commit` is read by git (or required); the dist build log's sha256 goes in summary.json.
>   - **TE-11's limits test:** pin 65,541 to U6b's FULL_STATE_MAX only if that constant is reachable through
>     client-wasm's dev edges; else leave it and say so.
> - **Not in this pass:**
>   - TE-5 (a carry to Stop B);
>   - every wording item (NC-3, NC-8, TE-6, TE-11's records, T5ii-C's note): those go to the report, which I write.
>
> **2. The end.**
> - `node check.mjs` whole through u7-run.sh -x: 12 steps; the test counts rise by your new Rust tests, against
>   end-1's 881 and 978.
> - `node --test web/test/*.test.mjs`.
> - Then one T5 (ii) rerun on the fixed tree (the shipped web and Wasm sources change, so run 3 no longer stands):
>   - run 3's flags (--no-proxy-server, C8 local, the tokens rule);
>   - every mark of run 3 read again, mark 7 now with both halves;
>   - mark 8 reported as observed (the bite's fallback time), with no restatement;
>   - mark 9 NOT-RUN, with F3's leg named as its in-process stand-in.
> - Record summary.json's commit and dist hashes.
>
> **3. As brief 3** (launch-0850.md section 3, items 2-4b and 6):
> - every cargo call through u7-run.sh, with `--locked`;
> - NPT_WASM_BINDGEN as there; build outputs under .gate/ only; Cargo.lock unchanged;
> - SIZE-CHECK: about 4,731 added now, and this pass's items about 300 more. Past 6,000 the unit stops and you report
>   (what grew, why, the cheaper option);
> - notes after each block.
>
> Waits: run each long call in the background with a long timeout, and read its log when told it ended. No shell loops,
> not even read-only ones. Write files with the Write and Edit tools only (no sed, no shell redirect, no python writes;
> never `cargo fmt` writing files: apply rustfmt's diff by Edit); an agent with no Edit tool rewrites its notes file
> whole with Write. Never switch the main folder; never `cd` into another worktree; no `git -C` and no `git -c`. git and
> gh always start their own command, with no other program in front of them.
>
> **4. At the end:** the items, tests, plants, the rerun's marks, the added lines and the open items, in a short
> summary. I take the snapshot.

## 3. U7R-TE-1
The rerun, not the dist compare: F3 (script.rs, engine.rs), F5 (worker.ts, main.ts), NC-5 and NC-6 change shipped Wasm
and page sources, so the dist differs by design. One T5 (ii) rerun on the fixed tree is budgeted in the brief, end step.

## 4. A second round
None (NPT-TRIM: one round per unit). My read of the fix pass's snapshot (not a round) checks each item's red and green,
its plants and the rerun's marks, with F3's press counting read for the netcode question.

## 5. U8's ops/ files and the baseline
Ruled as proposed. The builder takes the light baseline right after the fast-forward (brief step 0.3). The main session
runs choice 15's two ops commands on the fixed tree at U7's merge M5, as for U8. The builder makes no link. U7's
42 files touch crates/server only in tests/carry.rs, which U8's tests do not read. Builder 2's later merge leaves a
rerun done before it standing.

## Decisions needed (for the day's message; each by TEST-FIRST's way out, "a pass mark that cannot be built or judged")
- **U7-MARK9.** U7's T5 (ii) mark 9 ("the room's log: the page's slot has no kind-10 refusal and nothing counted as
  forged") cannot be read: the room log has no per-slot refusal series.
  - Why it matters: U7's browser check cannot be called passed while a mark stays unread.
  - Options: (a) report it NOT-RUN, with the in-process late-wake leg (F3) as a stand-in, and read it once U6d adds the
    series; (b) U7 adds the series now (outside its brief).
  - My lean: (a).
- **U7-MARK8B.** Mark 8 asks "no fallback event in 10 s" under the no-wt-timeout bite. Chromium's own QUIC timeout falls
  back at 4.06 s, outside the page's control: the bite did remove the page's 1.5 s timer (mark 5: 1.60 s).
  - Why it matters: as written, the mark cannot pass in Chromium.
  - Options: (a) restate it as "a fallback later than 3 s, or none in 10 s"; (b) keep it and report it not met.
  - My lean: (a).
- U7's merge waits for these two answers. The fix pass does not.

## For Mustafa
Nothing now. The two questions above go into today's message.

## Notes
- 09:52-10:0x: the packet, both reviews, the draft brief and u7-brief.md:190-203 read. Step 0 checked: `git diff --stat
  9177fb5 80b6748 -- spikes/netcode-proof/crates web check.mjs check.test.mjs Cargo.toml Cargo.lock` is empty. dirstat
  outside .checkpoints/: ops/ (U8), .github/workflows, packages/e2e and docs (MAIN-67). wip/u7-build-rs.patch has 42
  files; its only server path is crates/server/tests/carry.rs. Main is still 07d673b.
- u7-build-rs.md and u7-t-build.md are untracked on the branch (`git ls-files`: none), so the builder's notes do not
  collide with the fast-forward. U8's files came in at 05c63a8 ("U8: the R1 run package, merged").
- Not checked: the reviewers' line numbers against the worktree (taken as written), the size estimate for this pass
  (about 300 added, my reading), and whether FULL_STATE_MAX is reachable (the builder checks).

DONE
