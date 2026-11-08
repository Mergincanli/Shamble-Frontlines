# infra-engineer review of kit-v3-4 (A4), head ad1f9bd, 2026-11-08

Role read from origin/main (`.claude/agents/infra-engineer.md`, CLAUDE.md). Read-only; this file is my only write.

## Plan
1. Rollback rule (CLAUDE.md Rollback bullet; advisor.md "Revert"): against `.github/workflows/gate.yml`, main's
   protection, the guard (worktree/revert/push), and git semantics of `git revert -m 1` and the exact-undo diff
   (test in a scratch repo: merge commit, squash merge, later overlapping merge).
2. Foundation notes (SPIKE_PLAN:438-443, ADR 0006 parts 9/10, OQ-0006-14, "A bad release", register OQ-0006-14, I-18):
   notes not designs; consistent with ADR 0006, 0002 part 7, 0003; no slice mapping for "first playtests".
3. CHECKPOINTS.md §10: cargo clean --target-dir, the netcode spike's target dir, worktree remove + untracked target/,
   shared CARGO_TARGET_DIR and Cargo's lock, who may run it (guard per seat).
4. Infra: cost, exposure, CI/hosting change, how this change is itself rolled back.

## Findings (added as confirmed)

### Evidence runs (scratch only: scratchpad/infra-rv, scratchpad/infra-cargo; git 2.43.0, cargo 1.97.0)
- Guard per seat (`.claude/hooks/guard.mjs`, PreToolUse JSON): day: `git worktree remove`, `cargo clean`, `git revert`,
  `git worktree add -b cs/revert-1 .gate/revert-1 origin/main` get no hook decision (settings decide: worktree allowed,
  cargo and revert prompt); night-shift: worktree remove and add pass, cargo and `git revert` refused; pr-advisor: all
  four refused. Matches CLAUDE.md's Rollback "who" and CHECKPOINTS §10's "Who runs it".
- Revert + CI: synthetic merge on main adding `function infraProbeHelper` to shamble-game.js, then `git revert -m 1`:
  `node tools/gate.mjs <main> --no-smoke` -> `[3] symbols FAIL ... - DROPPED: infraProbeHelper`, GATE FAIL. gate.yml
  passes no `--allow-drop` (gate.yml "Gate" step), so CI's required `gate` can never go green on that revert.
- Exact-undo: `git diff M M^1` vs `git diff main...revert`: byte-equal when nothing later touched the files (also when
  main moved on other files); after a later merge edited the same file elsewhere, the revert applied cleanly but the
  byte diff differs (index line, hunk offset); `git patch-id --stable` equal for both.
- `git revert -m 1` on a non-merge (squash) commit: accepted by git 2.43 (main has one squash merge, #33 e4d8b22).
- `cargo clean --target-dir <dir>` in a folder with no Cargo.toml: exit 101 "could not find `Cargo.toml`". From the
  workspace folder, plain `cargo clean` uses `.cargo/config.toml`'s target-dir. `cargo clean --target-dir` on a folder
  that is not a target: aborted ("to prevent accidental deletion of unrelated files"), nothing deleted.
- `git worktree remove` (no --force): refused with an untracked `target/`; after moving it away, succeeded and deleted
  the ignored `.gate/x/evidence.txt` without a word; succeeded and dropped a commit made on a detached HEAD (now on no
  branch); succeeded on a copy holding a nested copy under its `.gate/`, deleting the nested copy's files and leaving
  a "prunable" entry.
- Shared Cargo target: two copies of one workspace (same crate, different source text) with one target-dir: concurrent
  builds serialise ("Blocking waiting for file lock on build directory"); both copies use ONE artifact slot
  (`deps/a-d1103dd5ee8a232e`); sequentially, copy two's `cargo build` says Finished in 0.00s and `cargo run` prints
  copy one's text. A copy silently runs the other copy's build.

## Findings (CLAUDE.md's new scale)

### IE-1 (High) A pure revert of a merge that added game symbols can never get CI's `gate` green
- Where: `.claude/agents/advisor.md:270-272` ("CI green on its head"), CLAUDE.md:112-115; `.github/workflows/gate.yml:50`
  (`node tools/gate.mjs "$BASE" --no-smoke`, no `--allow-drop`); `tools/gate.mjs:259,273` (any dropped symbol FAILS).
- Evidence: the synthetic revert above: `[3] symbols FAIL ... DROPPED: infraProbeHelper`, GATE FAIL. Main's ruleset
  requires `gate` (STATUS:981), so the revert PR cannot merge; the path dead-ends for exactly the merge most likely to
  "break the game" (a legacy critical fix that adds a helper). Bypass actors on the ruleset: not read.
- Fix (text, this change-set): in "Revert", after "CI green on its head": "If the revert drops game symbols (they are the
  reverted merge's '+ added' list), CI's `gate` fails by design, since CI passes no `--allow-drop`: the day session runs
  the gate with `--allow-drop` naming exactly those symbols, the PR is needs-mustafa, and it goes to Mustafa under
  Decisions needed (a fix forward, or his own way past the required check)." Plus a STATUS follow-up (risk tier 2, its
  own change-set): a way for CI to accept a pure revert's drops (for example, the gate computes the allowed drops from
  the reverted merge named in the revert commit's "This reverts commit <sha>" line).

### IE-2 (Medium) The exact-undo check is byte-strict and the later-merge test fires on almost every revert
- Where: `.claude/agents/advisor.md:270-272`.
- Evidence: byte-equal holds for a merge commit and for a squash commit (`M^1` is the parent either way; git 2.43 takes
  `-m 1` on a non-merge) and when main moved on other files. After a later merge edited the same file elsewhere, git
  applied the revert cleanly, the byte diff differed only in the `index` line and hunk offsets, and
  `git patch-id --stable` was equal. Every change-set edits STATUS and ROADMAP (protocol step 6), so "no later merge on
  main touched the same files" is false whenever any merge followed the broken one: the no-plan path works only while
  the broken merge is main's latest.
- Fix: compare with `git patch-id --stable` (`git diff M M^1 | git patch-id --stable` against
  `git diff origin/main...<head> | git patch-id --stable`), which is exact on content and ignores offsets; and either
  keep the later-merge rule but say plainly that it usually means "plan first", or narrow it to later merges touching
  the same code or rules files (STATUS and ROADMAP overlaps that git applied cleanly do not count). The second is a
  loosening of the drafted text: the advisor's or Mustafa's call.

### IE-3 (Medium) A revert PR cannot meet the every-tier "STATUS and ROADMAP updated" check
- Where: `.claude/agents/advisor.md:213` against :270 (exact inverse) and :266-267 (anything more is not a pure revert).
- Fix: in "Every tier, check", "STATUS and ROADMAP updated (a pure revert aside: the fix PR records it)"; or say in
  "Revert" step 4 that STATUS and ROADMAP are updated in the follow-up PR.

### IE-4 (Medium) Other PRs while a revert is open, or right after it lands
- Where: `.claude/agents/advisor.md:255-276`; CLAUDE.md:112-115.
- Facts: main's ruleset is not strict (a branch behind main may merge, STATUS:981); a PR's `pull_request` run tests
  the merge ref made when it ran, and a re-run reuses that commit (GitHub's re-run docs, from memory: not re-read in
  this review). So a PR approved and green before the revert was tested with the reverted change in it; the PR Advisor
  can merge it after the revert, and if it uses what the revert removed, main breaks again (gate 5b would catch an
  undefined name only on main's own push run, after the merge). Also `gate.yml:15-17`
  (`cancel-in-progress` per ref): a merge right after the revert cancels the revert's own main run, so "main green
  again" is never shown for the revert commit.
- Fix (two lines in "Revert"): "Until the revert merges, no other PR merges (the session says so in For Mustafa; the
  PR Advisor holds a PR while main is red). After it merges, a PR whose last CI run started before the revert merges
  only after its branch is brought up to date with main (a new head, so a new review)". And: "check that CI ran to the
  end on the revert's main commit; re-run it if it was cancelled".

### IE-5 (Medium) No path when the broken kit change stops the day session itself
- Where: CLAUDE.md:112-115 ("a merged kit change stops sessions working"), `.claude/agents/advisor.md:260-266` (day
  session only, `git revert` in a worktree).
- Why: if the merged kit change makes the guard or settings refuse the session's git or Bash calls, no session can
  build the revert, and the routines may not (guard per seat, above). Mustafa's own route exists: the "Revert" button
  on the merged PR's GitHub page opens a revert branch and PR, which runs CI like any PR.
- Fix: one line: "If no session can work, Mustafa opens the revert from the merged PR's page on GitHub (its Revert
  button); the advisor checks it and it merges on his go, as above."

### IE-6 (Low) "One re-run, pinned to one merge" needs an operational test and an owner
- Where: CLAUDE.md:112-113; `.claude/agents/advisor.md:257-259`.
- Facts: the push gate on main compares with the previous tip (`gate.yml:41-47`, BASE = `github.event.before`), so its
  checks 1-4 are per merge, but smoke tests the whole tree, and a non-strict ruleset lets two PRs that were each green
  combine into a red main; `cancel-in-progress` can leave an earlier merge with no finished run, and re-running that run
  cancels the run in progress on the newer main commit (same concurrency group). `gh run rerun` is not in the allow
  list (`.claude/settings.json`), so a day session's re-run asks.
- Fix: "Pinned = the red main commit M has a green first parent M^1 (its CI, or the full gate run on M^1 in a worktree);
  the re-run is the same commit's run re-run once (a day session, with an ask, or Mustafa)."

### IE-7 (Low) Revert step 1 details
- Where: `.claude/agents/advisor.md:263-264`.
- `<n>` is undefined (the reverted PR's number?); a cloud session limited to a `claude/…` branch uses its stand-in
  (CLAUDE.md, Standing permission), not `cs/revert-<n>`; fetch first (`git fetch origin`) so `origin/main` is current.
  The revert copy is then removed under CHECKPOINTS §10.

### IE-8 (Medium) §10: `git worktree remove` without `--force` does not protect all unsaved work
- Where: `.claude/CHECKPOINTS.md:217-218` ("Git refuses a copy that still has unsaved work").
- Evidence (scratch, git 2.43): it refuses on tracked changes and untracked files (an unignored `target/` blocks it,
  as the advisor noted: root `.gitignore` has no `target/`; ADR 0002:148 plans to add it). It does NOT refuse, and
  deletes or drops: ignored files (a copy's `.gate/` scratch and evidence, the spike's per-copy builds under
  `.gate/netcode-proof-test/target`, `node_modules/`, `*.log`, `.env`); commits made on a detached HEAD (most review
  copies are detached: `git worktree list` shows `advisor-a1/wt` and `a3/.gate/kit-v3-3/vr-main-newtests`); and a
  nested copy under the copy's `.gate/` (files deleted, a "prunable" entry left).
- Fix: replace the sentence with: "Git refuses a copy with changed or untracked files, but deletes ignored files
  (`.gate/`, builds, logs) and drops commits on a detached HEAD. Before removing: nothing to keep in
  `git -C <path> status --short --ignored`; its HEAD is on a pushed branch or inside main
  (`git merge-base --is-ancestor <HEAD> origin/main`); copies nested under it removed first; then `git worktree prune`."
  The night shift's removals need the same checks (it may run them, `Who runs it`).

### IE-9 (Medium) §10: one shared Cargo target folder can make a copy run another copy's build
- Where: `.claude/CHECKPOINTS.md:219`.
- Evidence (scratch, cargo 1.97): two copies of one workspace sharing a target folder use one artifact slot for their
  own crates (`deps/a-d1103dd5ee8a232e` for both); copy two's `cargo build` said Finished in 0.00s and `cargo run`
  printed copy one's text. Concurrent builds serialise on the folder's lock ("Blocking waiting for file lock on build
  directory"), so parallel copies lose their parallelism. And a shared folder conflicts with bullet 2: `cargo clean`
  on it removes every copy's builds, not one unit's. Base-vs-head measurements in two copies would silently compare
  the same binary (evidence that does not hold: High if adopted).
- Fix: keep the line as "to consider" (his word) but add: "Not for two copies of the same workspace: Cargo gives their
  crates one slot, so a copy can run the other's build, and the folder's lock lets one build run at a time. Today each
  copy keeps its own (the spike's `.cargo/config.toml` puts it under the copy's `.gate/netcode-proof-test/target`)."

### IE-10 (Low) §10: the cargo command as written fails outside the workspace
- Where: `.claude/CHECKPOINTS.md:216`.
- Evidence: `cargo clean --target-dir <dir>` from a folder with no `Cargo.toml` (the repository root) exits 101. The
  spike's `spikes/netcode-proof/.cargo/config.toml` sets `target-dir = "../../.gate/netcode-proof-test/target"`,
  read only from that folder. Good news: cargo 1.97 refuses to clean a folder that is not a target folder.
- Fix: "from the workspace folder (`spikes/netcode-proof/`, whose config names the folder) `cargo clean`; from the
  root, `cargo clean --manifest-path spikes/netcode-proof/Cargo.toml --target-dir .gate/netcode-proof-test/target`".
  His quote stays word for word. A day session's cargo asks (no allow entry), the night shift's and pr-advisor's are
  refused (guard, verified per seat).

## Part D, the foundation notes: no finding
- Notes, not designs: each quotes him exactly (SPIKE_PLAN:438-439 = item 4 minus the period; ADR 0006:461-462 = item 6's
  substring; OQ-0006-14 :1026-1027 and Rollback :1564-1565 quote "one-click rollback of client deploys and of server
  versions"), adds no metric, mechanism or option; part 9's note is a pointer ("No metric is added").
- Consistent: part 9 is "Logs and metrics" (:398); part 10 stores ADR 0003's field data (:453-456) and PM-0006-30 is
  its tab-crash mark (:1398); OQ-0006-14's options (a)/(b) both keep the previous room binary (:1018-1019); the client
  rollback "not read" matches :1019 and the Rollback section; ADR 0002:472-474 (per-edit hook: `tsc -b`, ESLint,
  `cargo check -p`) and :570 (G1 format) are cited correctly; ADR 0003:1245 says slice 6.
- No slice mapping: ADR 0006:463-465 and I-18 say "which milestone his 'first playtests' are is not set here".
- Optional (no label): the client-deploy rollback will also touch ADR 0002 part 13's production deploy job (part 15
  says the job is ADR 0002's); the OQ-0006-14 note could say so when that question is designed.

## Infra points
- Cost: none (rules and docs; no account, service or runner change).
- Exposure: none. No `.github/`, `.claude/hooks/`, `.claude/settings.json` or gate file in the diff (`git diff --stat
  origin/main...HEAD`); CI unchanged.
- How A4 itself is rolled back: by its own rule: a revert PR of a kit merge, built in a day session after asking Mustafa
  (kit files), merged on his go. It drops no game symbol, so IE-1 does not bite; IE-2's later-merge test will, since
  later PRs edit STATUS. Nothing enforced changes (no guard or settings edit), so a revert restores the old text only.
- Provider-down: not applicable.

## Verdict: PASS WITH NOTES
The rollback rule is sound in shape and matches the guard per seat (day only; `git revert` refused in both routines),
main's protection (a PR, no push) and today's merge classes. Fix IE-1 (High) in this change-set's fix pass: it is one
paragraph plus a follow-up. IE-2 to IE-5, IE-8 and IE-9 are Medium (fix now if the fix pass runs anyway, else
follow-ups); IE-6, IE-7 and IE-10 are Low. Part D: clean.
