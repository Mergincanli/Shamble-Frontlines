# Advisor: U7's C6 re-rule, then MAIN-66's pushed-head check (step 7)
2026-11-25/26, 23:5x UTC window. Advisor agent, day session, effort xhigh. Acting by main's advisor.md (read whole at
18:5x in this run). Read only, except this file; re-runs as advisor-main-66.md's Q1 (d). HEAD at start: 5da5f7d (= origin).

## Plan of this check
1. C6 first (due before 00:12): advisor-u7-brief.md:167-173; the request and security's answer; the prepared copies in
   .gate/u7-amend against the branch's files; check.mjs's shipped-edge rule; npt-link and npt-proto as workspace path
   members. Write the ruling at once.
2. Step 7: H0 8759e127df07, merge 6a4de15, pushed 69d9b5f; rev-list and diff --name-only to 5da5f7d; section 3 numbers
   by SHA; CI by run id; toolchain lines vs main's job 139375347198; re-runs (gate, repo map, checkpoint tests, guard
   tests) in the main folder if the precondition holds.
3. Verdict lines, notes, DONE.

## 1. C6 re-rule. Verdict: APPROVE. The amendment may land under C6 reworded as below, on security's Q1 conditions 1-7.
The amendment adds two featureless dev edges, npt-link and npt-proto, to npt-client-wasm.
- Both are path members of this workspace (spikes/netcode-proof/Cargo.toml:34 and :37).
- npt-client-wasm already ships both through npt-client, so they add edges, not crates. The package count stays 252.
- The lock gains two array lines and nothing else.
- check.mjs keeps only normal and build edges (`shipped`, :59, and `isShipped`, :78), so nothing new ships.
- The need is real. R3's room test has to build a `Loopback` and `Message`s itself, as npt-server's harness does
  (crates/server/tests/common/mod.rs:13-14), and a Rust crate names only its direct dependencies.
- Security found nothing (security-u7-0b-amend-answer.md).

So C6's purpose (the real `Decoder`, never a hand-written pump, nothing new shipped) is unchanged. Only its two lists
grow, and "and nothing else" stays. Replace C6's quoted text (advisor-u7-brief.md:167-173) with this, word for word:

> "Block 0b's `[dev-dependencies]` in crates/client-wasm, with its amendment of 2026-11-25
> (security-u7-0b-amend-answer.md), are `npt-link.workspace = true`, `npt-proto.workspace = true`,
> `npt-room.workspace = true`, `npt-server.workspace = true` and `npt-sim.workspace = true` (no other key), so R3's
> in-process test hosts the `Room` behind npt-server's real `Decoder`, on an `npt_link::Loopback` with `npt_proto`
> messages, as npt-server's tests do (crates/server/tests/common/mod.rs:12-18), never a hand-written pump. Expected
> lock diff against U6c's merged lock: npt-client-wasm's dependency array gains ` "npt-link",`, ` "npt-proto",`,
> ` "npt-room",`, ` "npt-server",`, ` "npt-sim",` and ` "wasm-bindgen",` (six names) and nothing else (no package,
> version, checksum or source line); against Block 0b's lock (55604a5f…9f5d), exactly ` "npt-link",` and
> ` "npt-proto",`. The dev edges ship nothing: check.mjs reads shipped edges only (check.mjs:56-78), so R4's deps rule
> (npt-client and wasm-bindgen only, written over npt-client-wasm's direct edges filtered by `isShipped`, as
> security-u7-0b-amend-answer.md Q2 asks) holds; npt-bots already has the same dev edge to npt-server."

Terms:
- (a) It lands by security's conditions 1-7, after MAIN-66's push, in its own step. The two Edits go in only while the
  base hashes still read d8af72f7…000e (client-wasm/Cargo.toml) and 55604a5f…9f5d (Cargo.lock), which they do now. The
  after-hashes must be f949f2bf…dc25a and 6e6529a8…00bc, and nothing else may change under spikes/.
- (b) It lands before U7's builder resumes on its new base. The builder builds nothing that holds the two edges until
  dependency-check.md has its new section (CHECKPOINTS.md section 3).
- (c) The reworded C6 is recorded with this file named as its source, as a departure U7's report lists. It needs no
  word of his: it is my reading of my own condition, and it changes no pass mark, judge or marks.json.
- (d) If MAIN-67 lands first and changes either file's base hash, the amendment is void as prepared (security's
  condition 1).

## Notes (one line after each check)
- C6: advisor-u7-brief.md:167-173 read. Request :19-31 and security's answer read whole (Q1 YES with 1-7, Q2 NO, Q3
  NO; no findings).
- `sha256sum` 23:5x: .gate/u7-amend client-wasm/Cargo.toml f949f2bf…dc25a and Cargo.lock 6e6529a8…00bc (as stated).
  The branch's copies: d8af72f7…000e and 55604a5f…9f5d; spikes/netcode-proof/Cargo.toml bd1e2335…f59a (the bases
  security named).
- `git diff --no-index` branch -> amend: the manifest gains +2 (npt-link, npt-proto at the top of [dev-dependencies]);
  the lock gains +2 in npt-client-wasm's array. Nothing else.
- check.mjs:55-78: `closures` and `isShipped` keep only edges whose dep_kind is null or "build".
- spikes/netcode-proof/Cargo.toml:34 and :37 are path entries. common/mod.rs:12-18 imports npt_link::{…Loopback…} and
  npt_proto::{Join, Limits, Message, Quant}.
- C6 written 23:5x UTC (before the 00:12 target).
- Step 7: H0 8759e127df07…, merge 6a4de150a57e… (parents H0 and c1da1bd), pushed 69d9b5f36a60… (parent 6a4de15). The
  first-parent line H0..69d9b5f holds only those two. 69d9b5f..HEAD (5da5f7d): 16 commits in a straight line, one parent
  each, touching .checkpoints/ only (9 names: the card, answer-1914, history, msg-2026-11-26, u7-build-rs.md,
  u7-t-build.md, u8-c1.mjs, u8-merge-plan.md, wip/u7-build-rs.patch).
- Heads-up for the 00:20 resume (not step 7's): 6cc6712 (20:51) added u7-build-rs.md and u7-t-build.md to the branch.
  The builder's worktree holds both, untracked and byte-identical (cee61bba…, d073228f…). This is launch-0850.md:6-18's
  collision: ffcheck will show 2 ADDED present, and `git merge --ff-only` refuses over untracked files.
- By SHA:
  - `git diff --name-status H0 6a4de15` is main's 92 (31 A, 61 M), the same list as 28254a7..c1da1bd.
  - shortstat "92 files changed, 11561 insertions(+), 631 deletions(-)"; without STATUS.md, the 91-file line
    (+11,512/-582).
  - `git diff --quiet 2935f0a H0 -- . ':(exclude).checkpoints/'` exits 0, so f3a420bc is the anchor. The anchor
    `git diff --quiet f3a420bc 6a4de15 -- . ':(exclude).checkpoints/' ':(exclude)docs/STATUS.md'` exits 0.
  - STATUS.md: `git diff c1da1bd 6a4de15` takes one line out (main's #40 row, the 2026-11-14 text) and puts one in
    (the branch's #40 row, the 2026-11-15 text). It has 120 lines.
  - Trees: H0 1,890 files, merge 1,921 (+31).
- The whole tree: `git diff --quiet c1da1bd 69d9b5f -- . ':(exclude)spikes/' ':(exclude).checkpoints/'
  ':(exclude)docs/'` exits 0. docs/ differs in the six names. spikes/ and docs/design/ equal H0's (exit 0). The kit
  pathspec c1da1bd..69d9b5f prints nothing.
- `--raw` outside .checkpoints/ and spikes/: six lines, all 100644 (no link, no submodule). No conflict marker (`git
  grep` at 69d9b5f, outside .checkpoints/: none).
- The merge 6a4de15: git's own message, "# Conflicts: docs/STATUS.md", no attribution lines, an SSH signature.
  69d9b5f (19:35:18): the card only (+8/-8), both trailers.
- The remote-tracking reflog: e32bd06 (19:24:29), then 8759e12 = H0 (19:28:32), then 69d9b5f (19:35:20). 6a4de15 was
  never a pushed tip, so no save ran between step 2 and step 6 (M65-1).
- CI on 69d9b5f (by run id, 6 jobs, all attempt 1, completed success, 19:35:28-19:40:46): gate run 45686693179 (gate
  139461044262, smoke 139461044639); toolchain 45686692982 (139461043587); gate 45686704323 (gate 139461079183, smoke
  139461079628); toolchain 45686704452 (139461080011).
- Toolchain job logs, whole (each starts at "Current runner version"), read as data. Main's job 139375347198 and both
  of #40's (139461043587, 139461080011) hold the same lines in the same order:
  - 49 `test result: ok.` lines, all "0 failed; 0 ignored; 0 measured; 0 filtered out";
  - 5 vitest `Test Files` lines (10, 1, 7, 2, 2 passed);
  - Playwright's "1 passed" and "16 passed".
  - No `##[error]` and no FAILED in any of the three.
