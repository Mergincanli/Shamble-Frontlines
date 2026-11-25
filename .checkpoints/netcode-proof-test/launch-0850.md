# The 08:50-09:20 UTC window, 2026-11-25 (the main session; at most 3 agents at a time; new agents only in the window)
Order: 1 (security) first; 2 (U8's C9) beside it; 3 (U7's R-half) once 0b is committed (security's Q2 (a)), or after
step 7 and the commit (Q2 (b)); if 3 cannot start by 09:20 it takes the next window (R-half planned Sun 29: slack).
Each brief carries the standing sentence. `<base>`: the 0b commit's SHA, filled at sending.

## 1. security-auditor (new agent; read only)
Answer the three questions in /home/user/Shamble-Frontlines/.checkpoints/netcode-proof-test/u7-0b-security-request.md
(U7's Block 0b; your read is security-u7-deps.md in the same folder). The evidence: u7-block0b.md there and the logs it
names under /home/user/Shamble-Frontlines/.gate/u7-0b/. Write your answer to
.checkpoints/netcode-proof-test/security-u7-0b-answer.md: each question by number with your verdict and any condition,
findings on CLAUDE.md's severity scale, ending with DONE. Run nothing that installs, fetches or writes outside that
file (no cargo install, update or fetch; read the cargo book or docs on the web if you need them). Write files with
the Write and Edit tools only (no sed, no shell redirect, no python writes; never `cargo fmt` writing files: apply
rustfmt's diff by Edit); an agent with no Edit tool rewrites its notes file whole with Write. Return one line per
question: its letter and any condition.

## 2. U8's builder (SendMessage; its notes u8-build.md, "## Pass 3")
C9 is yours now (advisor-u6c-r1.md:86-92): U6c merged into the netcode branch at 17964a3 (pushed d85cb43, CI green).
1. In your worktree: `git merge --ff-only d85cb43` (U8's work stays uncommitted; if git refuses because a local change
   clashes, stop and report the files); then `git log -1 --format=%H` into your notes.
2. NC-U6C-12 (Low): probe_presence (wip/u8-build.patch:2446, :2479-2485 in your tree's terms) reads P1's header
   `probe_start_timer_slack_ns`, `probe_start_sched_policy`, `l2_timer_slack_ns` and `l2_sched_policy`, and takes the
   rate from FP-1's `probe_up_window_ns` and `probe_down_window_ns`, with its fixture test. The field names and where
   each is written: marks/SERIES.md:921-952 at d85cb43 (header.params for the start pair; the header for the l2 pair,
   read at the leg's end edge; event.l2_summary for the two window fields).
3. SEC-U6C-5 (Low): one line in R1.md's certificate block naming Let's Encrypt's Subscriber Agreement and its URL as
   the ACME directory's `meta.termsOfService` gives it (he reads it at Stop B).
4. Size: 89 lines left under his 6,200 stop (6,111 at pass 3); past it, an unbuilt Low is carried and named
   (history.md:7703-7722). Tests first where a test can show it.
5. End checks as pass 3's: `node --test 'ops/**/*.test.mjs'` (100 pass before), `bash ops/tests/wrappers.test.sh` (315
   ok), `check.mjs --only redact` (0 findings) through your runner, package files unchanged, no plant left. Then report:
   the diff's files and lines, the counts, the total against 6,200, anything carried. I take the snapshot.
The cargo cache is cold (emptied 06:1x UTC); free 13.3 GiB then. Write files with the Write and Edit tools only (no sed,
no shell redirect, no python writes; never `cargo fmt` writing files: apply rustfmt's diff by Edit); an agent with no
Edit tool rewrites its notes file whole with Write.

## 3. U7's T-builder takes the R-half (SendMessage; its notes u7-t-build.md; new notes u7-build-rs.md)
The R-half now: Blocks R1-R4, then T5 (ii), by u7-brief.md rev 1.1 (sections 1, 3-6, 8) and the advisor's conditions
C1-C14 (advisor-u7-brief.md:110-247, binding by pointer).
1. `<base>` = the netcode branch with U6c merged and U7's Block 0b committed (wasm-bindgen = "=0.2.130" on
   npt-client-wasm, its three dev-dependencies npt-room, npt-server, npt-sim; the lock +4 lines, no new package).
   Counts at `<base>`: check.mjs test measured 861, bites 958 (u7-block0b.md step 5). In your worktree: `git merge
   --ff-only <base>` (your T-half files under web/ stay uncommitted; nothing under web/ changed since 577816c), then
   `git log -1 --format=%H` into u7-build-rs.md, with your plan for R1-R4 before any edit.
2. Every cargo call through `bash /home/user/Shamble-Frontlines/.checkpoints/netcode-proof-test/u7-run.sh [-x] [-k]
   <your worktree> <log under /home/user/Shamble-Frontlines/.gate/u7/> <command...> [:: <command...>]` (u6c-run.sh
   copied, names only: the shared target, the lane lock, CARGO_NET_OFFLINE=true, CARGO_INCREMENTAL=0, the 4 GB floor).
   On exit 3 (under 4 GB) stop and tell me; never clean anything yourself.
3. The cache is cold (emptied 06:1x UTC): your first build takes about 20-40 minutes. Free 13.3 GiB then.
4. The wasm-bindgen CLI is not installed yet (security's answer on the one download). R1-R4 do not need it. Before
   T5 (ii) wait for my message with its path (`<root>/bin/wasm-bindgen`, 0.2.130), or stop after R4 and report.
5. Size: about 660 Rust lines on the lane plus C2-C9's share (u7-brief.md section 8); SIZE-CHECK: the whole unit past
   6,000 added stops and reports; valve C11 if the T-half's count passes 3,800.
6. Your notes: each finished block at once, a notes line per finding, added lines after each block. At the end: the
   blocks, tests, plants, added lines, open findings, in a short summary; I take the snapshot.
Write files with the Write and Edit tools only (no sed, no shell redirect, no python writes; never `cargo fmt` writing
files: apply rustfmt's diff by Edit); an agent with no Edit tool rewrites its notes file whole with Write. Never switch
the main folder; never `cd` into another worktree; no shell loops.
