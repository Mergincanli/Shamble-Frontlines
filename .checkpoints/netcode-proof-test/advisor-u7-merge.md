# Advisor: U7's merge plan check (u7-merge-plan.md, sha256 9454f9792169bc83f67ca3299d90bc3839bce4e213b8446adfecb05e2495af08)
2026-11-26, 19:5x UTC window. Advisor agent, day session, effort xhigh, read only (no cargo). Acting by main's
advisor.md. Measured against my U7-M-1 to U7-M-8 (advisor-u7-snapshot.md section 4). HEAD 591e2cb (= origin); main
b9aad01 (ls-remote).

## Verdict: APPROVE WITH CONDITIONS UM-1 to UM-5 (all Low)
- The plan meets U7-M-1 to U7-M-8.
- The `ops` step after the merge is not a departure. U7-M-6 reads "at U7's merge (M4b) or in the first change after
  it, before Stop B's fill", and the plan takes the second form, for the reason it gives: M6's undo, and a commit equal
  to the snapshot.
- The facts hold when I re-run them:
  - the snapshot's sha256 6535c181…2728;
  - `git apply --check` exits 0 on today's tree, after Builder 2's merge 48efc9c;
  - U7's paths are unchanged on the lane since 80b6748 (`git diff --stat` empty over check.mjs, check.test.mjs,
    crates/bots, client, client-wasm, lab, server/tests/carry.rs and web);
  - main's #73 to #77 touch nothing under spikes/;
  - the builder's worktree branch is at 80b6748;
  - its t5-dist Wasm hashes to 901a6492…, run 4's summary.json `dist` entry.
- The conditions bind one moved fact, one count, the C1 script's tie and two forms. Applied word for word, they need
  no new check.

## Conditions (apply word for word)
- **UM-1 (Low): the order line.** In Facts, replace "MAIN-68 first (main-68-plan.md rev 3: #70, #73, #74, #76; rev 2's
  three until main moved at 18:00 UTC)" with "MAIN-68 first, at main's tip when it runs (b9aad01 at the advisor's check:
  #70, #73, #74, #76, #77 and #75; #75 adds a root-workspace crate, nothing under spikes/)". M0's `git fetch origin
  main` must find the tip MAIN-68 took in, as M0 already says.
- **UM-2 (Low): M5's test bar.**
  - Add to M5's counts: "test: 931 measured and 1,028 bites passed, 0 failed, 0 skipped." That is Builder 2's 908/1005 at
    48efc9c (its step 4) plus U7's 23 each (end-2 884/981 against the base's 861/958).
  - "If the totals differ: name the difference. If `git diff --stat 48efc9c H0 -- spikes/netcode-proof/crates` explains
    it, record it and go on; otherwise M6."
  - The other M5 bars stand: web 70, self 80, marks 81 with rows unchanged, redact 0 findings, refs N0 + 33, wasm as
    end-2.
- **UM-3 (Low): C1's script and stamp.**
  - Before M0: `git diff --no-index` of u8-c1.mjs and u7-c1.mjs (its own call) shows only the four changes Facts names:
    the header (:1-4), :12, :14 and :47.
  - At C1: the line "wasm sha256" it prints equals
    901a649231dc9c217204b121a2800641c3c6aa9cd3cb9d42eb5657452b0a0e77, run 4's npt_client_wasm_bg.wasm. Otherwise M6.
- **UM-4 (Low): the `ops` step's own change** (Q1). It is its own change by the main session: a plan first and my check
  in a window, after this merge's M9 and before Stop B's fill, with these parts:
  - (a) **Test first.** check.test.mjs:2785's list gains "ops" after "web", and `node --test check.test.mjs` fails on
    that assert. Then check.mjs's STEPS gains `ops` after `web`: `node --test 'ops/**/*.test.mjs'`, then `bash
    ops/tests/wrappers.test.sh`, both in HERE, failing if either fails, summarised with nodeCounts and the wrappers'
    last line. The self tests pass.
  - (b) **U8's fix 2, the real install.** First, package.json 8370495d… and package-lock.json a0efce8e… by sha256. Then
    `npm_config_ignore_scripts=true npm ci --ignore-scripts --prefix spikes/netcode-proof/ops/browser`, its own call.
    Then both sha256s unchanged and `git status --short` unchanged (the folder is a real one, so the root .gitignore's
    `node_modules/` covers it). npm's "added N packages" and its audit line are recorded; "4 moderate" is security's
    expected chain (security-u8-deps.md C3 (c)).
  - (c) **The dependency record.** One note section in dependency-check.md (the install's form, both sha256s before and
    after, npm's two lines), with no bare sha256 line: no manifest or lockfile changes, so DEP-HOLD holds nothing, and
    no new security read is needed (the form and the lock were cleared at U8).
  - (d) **A missing folder.** When ops/browser/node_modules is absent, the step fails with one line naming that install.
  - (e) **Whole runs before it lands** keep choice 15 (i): ops tests and wrappers beside `node check.mjs`.
- **UM-5 (Low): M9's cleanups.**
  - Their forms are named at my read of the pushed head, not here.
  - Never `git worktree remove --force` and never `rm -rf`: the guard refuses both (launch-0850.md:103-104).
  - The builder's worktree holds U7's uncommitted files and its notes, so it comes off only by a form that keeps every
    file a copy is not yet committed for.

## The plan's questions
- **Q1: the `ops` step's form and record.** UM-4. It need not wait for Stop B's fill, only come before it.
- **Q2: C1 through u7-c1.mjs is enough.** It is a one-off copy of a checked script, with four named lines (UM-3 ties
  both the script and the Wasm it stamps). An argument form is not needed.
- **Q3: M2b's form is right for U7-M-3.**
  - The line is `bash <u7-run.sh> <worktree> <log under .gate/u7/> true`: no git, gh or xargs word, so #73's rule U has
    nothing to read.
  - u7-run.sh's own git and xargs calls (the header's hashes, :68-69) run inside the script (the INPUT-RUN limit, as
    the plan says).
  - The header is written before the touch loop starts (:66-75), so the hashes are the tree's as it stands.
  - It takes the lane lock shared and builds nothing.

## Findings (CLAUDE.md's scale)
| Id | Sev | What | Fix |
|---|---|---|---|
| ADV-U7MP-1 | Low | The Facts' order line lists MAIN-68 rev 3's merges; main has since gained #77 and #75 | UM-1 |
| ADV-U7MP-2 | Low | M5's test counts are "recorded as found" with no expected total, though the total is knowable (931/1028) | UM-2 |
| ADV-U7MP-3 | Low | u7-c1.mjs is not written yet; nothing ties it to u8-c1.mjs or ties its Wasm to run 4 | UM-3 |
| ADV-U7MP-4 | Low | The `ops` step's change and U8's fix 2 have no named form or record | UM-4 |

## What I did not check
- I ran nothing beyond the read-only commands listed in the verdict: no apply with `--index`, no u7-run.sh call, no
  cargo.
- MAIN-68's own plan is not part of this check.
- U8-F1's state was not read. If it merges first, M5's ops count is its count, as the plan's Risks says.

## Decisions needed
None.

## For Mustafa
Nothing.

DONE
