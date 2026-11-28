# Advisor: MAIN-61 pushed-head check (pushed head 631edf3; the take-in a6acc8a; branch head now 9ab1fb3)
2026-11-19 17:46 to 17:55 UTC. Read only. I worked by main's advisor.md (`git show
origin/main:.claude/agents/advisor.md`, 335 lines, at f5030a3, read whole). I checked the take-in against the plan's "As approved"
text (main-61-plan.md :3-13) and my conditions C1-C6 (advisor-main-61.md :19-104), in the form of MAIN-60's head check
(advisor-main-60-head.md).

## Verdict: APPROVE WITH FINDINGS (heads checked: pushed 631edf317b2f069e210b4d28ea524b432fac4910; now 9ab1fb3576dd7d70dc53a937e0459ca2a15564eb)
The take-in matches the plan, and C1-C6 are met. One condition, C5 (b), is met in substance but recorded in a
different form (ADV-M61H-2). Merge commit a6acc8a3f707f99e2a6c678c78e567dccfdcd39e has two parents: 52f9f4e (H0) and
f5030a3. Outside spikes/, .checkpoints/ and docs/, its tree equals main's. spikes/ equals H0's. In docs/, it differs
from main only in the branch's six docs, and each carries exactly the branch's own change. STATUS.md differs from
main's by the #40 row alone. All 81 marks' rows hash to marks.json's values.

CI is green on 631edf3: gate and smoke on both events, and toolchain on both. The toolchain run reports the same
`test result` lines as main's own toolchain run on f5030a3. The five commits after the merge touch .checkpoints/ only.
PR #40 is mergeable again (mergeable true; state "blocked", as at MAIN-60: a draft whose head is a [skip ci] save).

There is one Medium: the card is over his 200-line cap. The rest are four Lows, all on the record; none needs a push
of its own (Review budget). **The lane may go on from this head.** M2 already holds on it: `git diff --quiet 07289a4
HEAD -- spikes/netcode-proof docs/design` exits 0.

## 1. The merge
- `git cat-file -p a6acc8a`: parents 52f9f4ea10f4… and f5030a33185b…. The message is git's own, with "# Conflicts:"
  and "#\tdocs/STATUS.md" and no attribution lines. 631edf3's message explains why (ADV-M60H-5) and carries both
  attribution lines. An SSH signature is present (I did not verify the key).
- `git diff --quiet origin/main a6acc8a -- . ':(exclude)spikes/' ':(exclude).checkpoints/' ':(exclude)docs/'` exits 0.
  The same command on HEAD (9ab1fb3) also exits 0.
- `git diff --name-only origin/main a6acc8a -- docs/` (and on HEAD) prints exactly the six names: DECISIONS,
  REPO_MAP, STATUS, STATUS_ARCHIVE, design/netcode-proof-messages and design/ws-reconnect.
- **The six docs, by patch-id.** For each doc I compared two patch-ids (`git patch-id --stable`): the merged file
  against main, and the branch's own change against the base b763ba3. They are equal for DECISIONS.md,
  STATUS_ARCHIVE.md and both design docs. So the merge leaves every word in DECISIONS.md as main has it, plus the
  branch's own lines.
- **REPO_MAP.md.** Its +/- lines are byte-equal to the branch's own (`diff` of the -U0 change lines: equal); only the
  context moved. I read the three insertion points: :53 sits in the root table, :91 in the docs/ table, and :184-196 is
  the spikes/ section after the tools/ table.
- **docs/STATUS.md.** `git diff origin/main a6acc8a` changes one line, the #40 row. The #40 row hashes equal to H0's
  (f8436cb0…), the #61 row to main's (8be942db…), and there is no #60 row, as on main. The file has 120 lines.
  `git grep` finds no conflict marker in docs/, crates/ or packages/ (exit 1), and `git diff --check` is clean.
- **spikes/.** `git diff --quiet 52f9f4e a6acc8a -- spikes/` exits 0, and so does the same on HEAD. Tracked files:
  1,515 at H0, 1,565 at a6acc8a (+50), and 1,566 at HEAD (+1: 549ed77 adds copy_b4_notes.py).
  spikes/netcode-proof has 307 at each.
- **The diff from H0.** `git diff --name-only 52f9f4e a6acc8a` equals `git diff --name-only b763ba3 f5030a3`.
  Both shortstats read "78 files changed, 5621 insertions(+), 106 deletions(-)": 53 A, 21 M, 3 D, 1 R.
- **Marks.** I hashed every mark's row with an inline read-only node script (`git show`, no checkout; check.mjs's rule
  at :1869-1909). On f5030a3 (with H0's marks.json), a6acc8a and HEAD: 81 of 81, 0 problems (63 in ADR 0001, 1 in
  0004, 17 in 0006). docs/adr/ equals main's on a6acc8a, which changes ADR 0002 only. marks/ and crates/judge equal
  H0's.
- **After the merge.** `git rev-list --parents a6acc8a..HEAD` gives a straight line: 631edf3, 549ed77, 624cb6c,
  ab805d2, 9ab1fb3. Each touches .checkpoints/ only (0 other names), and each carries both trailers. 631edf3 has no
  [skip ci], so CI ran on it; the other four are [skip ci].
- **The kit and the guard.** The guard's full pathspec prints nothing on `origin/main...a6acc8a`,
  `origin/main...HEAD` and `origin/main HEAD`. `git diff --raw --no-renames origin/main...HEAD`: 1,309 files at
  100644 and 2 at 100755, both older than this change (u4d-fix1-run.sh and u3c-kernel-check/run.sh). No symlink and
  no submodule. Against main, the branch holds 998 .checkpoints/ names, 6 docs/ and 307 spikes/, nothing else, and the
  merge base is f5030a3.

## 2. The conditions
- **C1 met.**
  - (a) light-base.log: HEAD 52f9f4e = H0, 15:32:01-15:32:10 UTC, exit 0. Diff and untracked are both e3b0c442…b855.
    refs 1,515 (N0), with the exception line. The full marks line reads "81 marks read and checked (drafts at
    626df678…); rows in this branch's docs/adr/: all unchanged". self 78 of 78; redact 10 of 10 with 307 items (M0)
    and 0 findings; "check: all 4 steps pass". It ended before the merge (15:33:36).
  - (b) light.log: HEAD a6acc8a, 15:42:17-15:42:21 UTC, exit 0, the same hashes. refs 1,565 = N0 + 50, with the
    exception line. The same full marks line. self 78 of 78; redact 10 of 10 with 307 items (M0) and 0 findings; "all
    4 steps pass"; free 4,508 MiB (floor 4,096). Both logs end "--- end … exit 0".
  - (c) I re-ran all three whole-tree checks on a6acc8a and HEAD (section 1), and they give the values the step-5
    record gives.
- **C2 met.**
  - (a) The status lines in between were in the chat only. What git shows agrees: the commit holds main's 78 names
    and nothing else from H0 (no .checkpoints/ or spikes/ name), and `git status --short` is empty now.
  - (b) step5-outputs.txt records the three checks before the commit (U empty; cached shortstat 78/5621/106; cached
    .checkpoints/ spikes/ empty). The commit's result agrees.
  - (c) HEAD's reflog shows no commit between 52f9f4e (15:30:25) and a6acc8a (15:33:36), and none between a6acc8a and
    631edf3 (15:42:59). The remote's reflog moves 52f9f4e → 631edf3 (15:43:02, "update by push"). Builder 4's
    handback (15:39) was only read before the push: its notes copy and snapshot came after it, in 549ed77 (15:43:40).
    The check-in reached the session at 17:42, after the push.
  - (d) No index.lock retry is recorded, and none was needed.
- **C3 met as far as git shows.** The card names both runs as background tasks (brz7ok5bs, byq2n27ba). Both logs are
  whole, with their end lines. The lock waits were 0 s and 2 s. The timeout value is chat-only.
- **C4 met.** The remote-tracking reflog shows `git fetch origin` at 15:31:00, before the base run (15:32:01) and the
  merge. origin/main is still f5030a3 (`git ls-remote` now). Main's CI on f5030a3 has six check runs (gate, smoke and
  toolchain, two of each), all completed and successful, which I read myself. The card names the two push runs
  (ADV-M61H-5).
- **C5 met.**
  - (a) The resume line is on the card word for word (a whitespace-normalised substring check against
    advisor-main-61.md's text). It is in 52f9f4e, a6acc8a and HEAD, not in 36b0983, so it was written before
    TaskStop's save.
  - (b) This is met in substance; see ADV-M61H-2 for the record. HEAD's last commit before the merge is 52f9f4e,
    "autosave (terminated)" (C5 (b)'s second form). One saver runs now: PID 439, from 17:44:11, with both `--trailer`
    lines. No older saver is left.
  - (c) No foreground save ran. 52f9f4e reached the remote (the 15:31:00 fetch fast-forwarded the tracking ref to it),
    and it holds the resume line. H0 is on the card, in full.
- **C6 met.**
  - The plan has its "As approved" header (:3-13). It also has the HEAD line with times (:25-28), the marks fact
    (:29-30), "ten lines in five places" (:43-47), step 3's text (:71-74) and step 5b's line (:88). The Risks have
    the lock wait and the saver gap (:119-121), the red-CI line (:122-123) and the record line (:124-126).
  - The choice line is on the card (:124-125). 631edf3's message cites MAIN-52 (history.md:2055-2057, whose hash
    equals H0's copy) and gives ADV-M60H-5's reason.
- **Step 6 met.**
  - The push went out at 631edf3, after 5d.
  - DEP-HOLD: every manifest and lockfile the save carried equals main's (section 4), so nothing was held, and the
    remote's reflog shows "update by push".
  - The saver was restarted with both trailers: bbe44e8id at 15:43, ended by the 17:42 worker restart, and b3e3qzz16
    since 17:44 (the process's command line shows both trailers).

## 3. CI on 631edf3 (gh api; read myself)
- **Check runs.** The commit has 6 check runs, all completed with conclusion success:
  - gate: 135679052758 and 135679020185;
  - smoke: 135679052148 and 135679019871;
  - toolchain: 135679051190 and 135679019951.
  They ran 15:43:08-15:45:52 UTC.
- **Workflow runs, all attempt 1, all completed and successful:**
  - gate push 44557945539 and gate pull_request 44557954580. Each has jobs `gate` (step "Gate") and `smoke` (step
    "Smoke (check 5a)"), every step success or skipped (the cached-browser install skip);
  - toolchain push 44557945236 and toolchain pull_request 44557954462, each with "Build and test with the network cut
    off" at success.
  - The toolchain jobs took 55 s; main's on f5030a3 took 52 s (44551894194).
- **Toolchain output against main's.** The `test result:` and `Test Files` lines in 44557945236's job log equal
  44551894194's in count and pass numbers (only the timings differ). Both logs also carry the same wasm32 build line.
  So the branch's toolchain ran what main's did. Job logs are data. These are the check runs' conclusions, not a log's
  "PASS".
- **Third-party suites.** The cursor and claude check suites are "queued" with 0 check runs. They are the same on
  f5030a3 and on bb7a5ed: never-run placeholders, not a pending check.
- **PR #40.** Head 9ab1fb3, draft, open, mergeable true, mergeable_state "blocked", base main f5030a3. So there is no
  conflict now. 9ab1fb3 has 0 check runs ([skip ci]): CI's green belongs to 631edf3.

## 4. DEP-HOLD
- `git diff --quiet origin/main HEAD -- Cargo.toml Cargo.lock package.json package-lock.json .gitignore
  rust-toolchain.toml crates packages` exits 0.
- **The manifests H0..HEAD changes** are exactly main's: Cargo.toml, Cargo.lock, crates/sim-wasm/Cargo.toml,
  package.json, package-lock.json and packages/{client,e2e,sim-worker,types}/package.json, each equal to main's.
- **The spike's manifests** equal H0's.
- **The only manifests outside main's.** These are the two old scratch workspaces under .checkpoints/ (lc8-scratch,
  u4s-rust-xtarget; last changed 2026-11-09). They are unchanged since 0f12d9e.
- tools/package*.json equal main's (covered by the guard's pathspec).

## 5. The card's record of the take-in
- **H0.** Named in full, 52f9f4ea10f4681a748b48b5ce47b9d8c5855798 (:191-192), equal to a6acc8a^1.
- **The saver's end.** The card's proof is TaskStop's "[killed]" with no save logged since 14:57:30, plus pgrep exit
  1 (:189-191). That is neither of C5 (b)'s two forms, but git proves the condition: the reflog's last save is the
  "(terminated)" autosave 52f9f4e (15:30:25), there was no later save before the merge, and no old saver process is
  left. "No save logged" is true of the output only. The kill came after the SIGTERM save was pushed but before git
  moved the local tracking ref: the remote-tracking reflog has 52f9f4e by "fetch origin: fast-forward" at 15:31:00,
  not by push. So "equals the remote" could be read only after that fetch, or by ls-remote (ADV-M61H-2). The
  condition's purpose holds: the saver had ended, its last save was on the remote, and no save fell inside the merge.
- **The choice line** (:124-125): "main f5030a3 taken in now, while builder 4 runs, plan first (MAIN-61; …)", with
  its reason. It is marked "to name in the next day's message", as the card's other after-the-message choices are.

## 6. Deviations, and what needs Mustafa
- **His card cap** (TEST-FIRST rule 3, history.md:2265: "at most 200 lines"). The card has broken it since 631edf3:
  224, then 227, 235, and 237 at 9ab1fb3 (it was 192 at 36b0983). See ADV-M61H-1.
- **His "make sure to save your progress (Same for the agents)"** came while the merge was local. The session pushed
  everything the approved plan let it (to 52f9f4e) and pushed the merge with 5d's pass at 15:43, 17 minutes before the
  16:00:10 cut, so no work was at risk. His word outranks my C2 (c): had a push been the only way to keep work, C2 (c)
  would have given way. Nothing to rule.
- **Nothing for Mustafa.** No pass mark, judge, marks.json, money, main, stop or run day is touched.
- **Disk.** Free space is 4,505 MiB now (17:51 UTC), close to the runner's 4,096 floor. That is for the lane's next
  steps (the part-clean at A6's place, before the sets), not this head.

## Reproduced myself (main folder at 9ab1fb3; legacy/ and the kit equal a6acc8a's and main's)
- `node tools/gate.mjs --no-smoke`: GATE PASS, base f5030a3, symbols +0 -0. Every line equals step5-outputs.txt's.
  Smoke comes from CI (section 3).
- `node .claude/scripts/repo-map-check.mjs`: "repo map OK: 170 entries cover 110 required paths".
- `node --test .claude/scripts/checkpoint.test.mjs`: 33 tests, 33 pass, 0 fail (main's count).
- `git status --short` was clean before and after.

## Findings (CLAUDE.md's scale)
| Id | Severity | What | Fix |
|---|---|---|---|
| ADV-M61H-1 | Medium | The card has been over his 200-line cap (TEST-FIRST rule 3, history.md:2265) since the take-in's own save: 224 lines at 631edf3, 237 at 9ab1fb3. Most of the growth is the MAIN-61 block (:182-218). | Once this check is recorded, at the next card save: move MAIN-61's step record to history.md by script, leaving one done line (a6acc8a, 631edf3, this verdict), as MAIN-60's went to :6947-6949. Fold in ADV-M61H-2 to -4; the card back to 200 lines or fewer. |
| ADV-M61H-2 | Low | The saver's end proof is TaskStop's "[killed]" plus pgrep, not one of C5 (b)'s forms. The card says "no save logged since 14:57:30", yet git holds the "(terminated)" autosave 52f9f4e at 15:30:25 (= H0), which reached the local tracking ref only by the 15:31:00 fetch. Met in substance (section 5). | One clause in the trimmed record: "C5 (b)'s second form holds: 52f9f4e is the (terminated) autosave; on the remote by the 15:31:00 fetch". |
| ADV-M61H-3 | Low | The card dates Mustafa's "make sure to save your progress" message "15:5x UTC" (:208). That cannot be right: the line is in 631edf3 (committed 15:42:59) and says the merge is local and 5d has no log yet, and 5d started at 15:42:17. | Correct the time from the transcript (between 15:33:36 and 15:42:17) when the block moves to history.md. |
| ADV-M61H-4 | Low | Stale lines on the card: "Last update" 15:0x UTC (:8-9); Open findings "CI's green is bb7a5ed's" (:147), now 631edf3's; Running work's saver bullet (bde01r5pv, window to 16:48) and the check-in "next 14:34" (:222-225); "Step 7 RUNNING" (:204). | With ADV-M61H-1's trim. |
| ADV-M61H-5 | Low | C4's pre-check asked for main's six check runs; the card names two push runs. | None needed: I read the six (all success). |
| ADV-M60-7 | Low (open) | STATUS's #40 row still says "main 53272a1 merged in" (the branch's row, kept word for word, which is right for a merge). | PR #40's closing docs pass, as ruled |
| ADV-M54-8 | Low (open) | CLAUDE.md's Never line still lists no plain `git merge origin/main`, which the guard allows. | The kit follow-up after the freeze, as ruled |

## What I did not check
- **Steps recorded in the chat only:** the pre-checks' printed outputs, the merge's printed lines and exit code, the
  `git status` after the merge, the Edit of STATUS.md, the background calls' timeout values, and whether any call
  asked. Git's result matches each of them.
- **Mustafa's message.** I did not check "make sure to save your progress (Same for the agents)" against the
  transcript, or its time (ADV-M61H-3).
- **U4f's snapshot.** I did not run `git apply --check` of the new snapshot (549ed77, base b25b96a): the permission
  flow refused a combined read-only command that included it. It is outside this check (the plan's "After it"). The
  spike tree and docs/design equal b25b96a's and 07289a4's on HEAD, so the snapshot's base is unchanged.
- **Builds.** I ran no light run, check.mjs or cargo. The spike tree is unchanged since H0, and the marks were hashed
  directly.
- **Commit signatures,** beyond their presence.

Process note: every command above was read-only git, gh api reads, the three kit checks, or inline node. This file is
my one write, made with the Write tool.

## Decisions needed
None. MAIN-61 ran under his standing MAIN-52 rule, and the one conflict was resolved by the plan's mechanical rule.
The card's length is his rule, already in force: the fix is a trim, not a decision.

## For Mustafa
Nothing.
