# Advisor report: ci-wasm (2026-11-23, branch `claude/loving-hopper-6p0mg2` standing in for `cs/ci-wasm`, base main f5030a3)

Pull request #64 (it replaces #63 and #62). Risk tier 2: it changes `.github/workflows/toolchain.yml`. Written by the
builder chat Mustafa calls Builder V3 (session_01WLuqnvpAh9XTitx8ehKJcQ), which took the pull request over on his
words of 2026-11-23, 16:30:46 UTC. The plan and the first code are the first builder chat's (2026-11-19); a thread of
his project carried the records in between and wrote no CI file.

## Goal
After this pull request, a push or a pull request cannot go green unless the Wasm builds with the checked tool, every
boundary test that runs passes on that Wasm, the Worker, the page and the specs type-check, and every browser test
that runs passes, all inside the network cut. CI does not pin how many tests run: a skipped or removed test keeps it
green (only zero tests turns it red). Today no spec skips itself, and each clean run printed "Running 12 tests using
1 worker", "12 passed" and the boundary block's "Tests  7 passed (7)"; pinning the counts is follow-up TE-COUNT. His
words (CI-WASM (a), 2026-11-18): "It runs GitHub's own check on the tool's file before the tool's first run there, and
its report quotes that check's output." Nothing a player sees changes.

## Outcome
DONE, waiting for his go. The workflow text is in, in two writes: the text he approved first stopped at its install
check on GitHub's runners (the stop rule fired; nothing was fixed before he answered), and the corrected text runs
clean. On the code head 6d0b5b1 `gate`, `smoke` and `toolchain` are green for the push and for the pull request.
Three planted faults each failed the job at their own step and were taken back.

## Plan and approval
- **The plan** (`.checkpoints/ci-wasm/plan.md`, revisions 1 to 3 by the first builder chat on 2026-11-19, revision 4
  here): one job; before the network cut the one tool archive Mustafa named is fetched, checked by size and sha256
  and by GitHub's own check, and its one program installed root-owned; inside the cut the page is built twice, then
  the boundary tests, five type checks and the 12 browser tests in the runner image's Chrome. With it four small
  follow-ups from the archive and the docs pass for #61's merge.
- **The advisor agent, plan review (2026-11-19):** APPROVE with eight conditions, no Critical or High; all worked in
  (revision 2). security-auditor's reading of the CI text before it went to him: one High (F-QUOTE, two apostrophes
  inside the cut's quoted script; the first builder chat's own slip; it would have failed closed) and two Lows, all
  three taken (revision 3). His answers: ANSWERS-7 (2026-11-23, 14:31:06 UTC): CHROME-SOURCE (a), CI-TEXT (a),
  FIREFOX-TESTS (c), since changed to (b) (21:24:59 UTC).
- **The takeover (2026-11-23):** the advisor agent read the thread's records (ESCALATE with eight record fixes, all
  made by the thread) and then this chat's start (APPROVE for the first CI write on two conditions: his yes on the
  exact text in this chat, and the mode read on that reply). Three wording points on this chat's first DECISIONS
  entries were fixed by script.
- **After the stop (revision 4):** his OPT-PATH (a); one short pass as he asked: infra-engineer proposed,
  security-auditor read the changed lines (accept; two Lows, both taken), the advisor agent ruled (APPROVE as it is,
  no Critical or High, no second round; its record conditions made).

## What changed
| File | Change |
|---|---|
| `.github/workflows/toolchain.yml` | +97, -0 against main (the section below) |
| `packages/client/scripts/build-checks.mjs` | the refusal pattern moved in from `build.mjs`, new `refusedBuildSettings(env)`; in review round 1 the pattern also refuses `CARGO_RESOLVER_*`, `CARGO_INCREMENTAL`, `RUSTC_BOOTSTRAP` |
| `packages/client/scripts/build.mjs` | the pattern out; one call changed |
| `packages/client/scripts/build-checks.test.mjs` | one test: 18 refused names, 15 allowed |
| `packages/e2e/playwright.config.ts` | chromium project: `workers: 1`, `timeout: 90_000` (the plan said 60_000; built at 90 s so the test's own limit lies above `report.ts`'s 60 s wait) |
| `packages/e2e/tests/skeleton/skeleton.spec.ts`, `clock.spec.ts`, `allocation.spec.ts`, `report.ts` | five timing points made robust for a slower machine without lowering a bound; a readings line per test; `report.ts` +9 (`drawn(page, frames)`, not in the plan's file table) |
| `packages/e2e/node-names.d.ts` | new, 16 lines: the four Node names the specs use, so the specs type-check with no new dependency |
| `.gitignore` | `dist-*/` |
| `docs/` | the docs pass (its own section below) and this report |

No manifest, lockfile or dependency changes. `gate.yml`, `tools/`, `.claude/` and every `CLAUDE.md` equal main's.
**Size:** outside the docs 191 added lines (planned about 155, stop line 310): the workflow 97, the build checks and
their test 38, the config and the specs 39, the declaration file 16, `.gitignore` 1.

## The workflow text (kept apart from the docs, as his words of 2026-11-19, 16:08:04 UTC ask)
**The texts he said yes to, and what was written:**
| Write | His yes | The text | Commit | The file after it (sha256) |
|---|---|---|---|---|
| 1 | CI-TEXT (a), 14:31:06 UTC (in the thread); CI-WRITE-1, 17:15:07 and again 17:22:42 UTC | `ci-edit.diff`, sha256 240c7db726dee600b54d10b0252b33e527482ecf88ba41af849660d55e8d2673 (93 added, none removed, against main) | cb476b141212c018e188061d65ab6e6b203c2c47 | d1dbcb533460936e9f4f3ca2e97ed28c6d586638e52c00fa034e875f7a4e975c |
| the correction | OPT-PATH (a), 18:40:57 UTC; FIX-WRITE (a), 19:58:23 UTC | `ci-edit-2.diff`, sha256 b4081e48b4c07e790fa489d31de940e0f95c162ae476b0f98b0160b7358203ec (10 added, 6 removed, against the file at cb476b1) | 2a654f37e9ca185e9f1828ec1c761f84e83d2578 | 56d37fc64ccfce2a21765613401a746633c6a9c661f3af52a9054bf650df9811 |
| the planted lines | PLANT-WRITE (a), 20:47:59 UTC | `ci-plant-p-gh.diff`, sha256 e4f21ab359bc9fd911c663a4fcbc892133195fefe44e88ad7ec51b50e3d0b1ac (2 added) | 47d7024c1dcc84db26dec73f7eab1d1af66385f0 | 84b6b3fd61dc1e798375374eef48f16ca3e9098887bd63cd944c4191bc631ff8 |
| their take-back | in advance, 20:47:59 UTC, for a planted run that fails the expected way (it did) | the removal of those two lines | ce5a7e78a7048f966460fdd346adf19250e009a3 | 56d37fc64ccfce2a21765613401a746633c6a9c661f3af52a9054bf650df9811 |

- **Byte-equal:** main's file with `ci-edit.diff` and then `ci-edit-2.diff` applied is the head's file byte for byte
  ({{VERIFIER_CLAIM_2}}). After each write the file's sha256 was compared with the expected one and shown to him, and
  `diff -u` of the file before against the file after equalled the approved diff's body. Each of the four commits
  holds that one file only; no other commit touches `.github/`.
- **The mode at each write, as his rule of 17:15:07 UTC asks it named:** the record read `default` on his messages
  of 17:22:42, 19:58:23 and 20:47:59 UTC (the session record and the mode stored on the message); his screen showed
  Accept edits, and his rule says to write on `default` too. On his first yes (17:15:07 UTC) the record read `auto`:
  nothing was written on it, and he was told the value.
- **The guard** asked before each of the four copies ("kit and gate files ask first in a day session:
  .github/workflows/toolchain.yml (cp)"), and each ask was answered by a one-time allow given in the app (the
  transcript: 17:23:43, 20:00:41, 20:48:40 and 21:01:33 UTC).
- **The model:** this chat runs on Fable 5.2 (the session record: configured and served). His block's point 6 set his
  words of 2026-11-12 on CI work aside for this chat's CI edits of this pull request, each asked of him with its exact
  text (CI-MODEL (a)). Every agent ran on Opus 5.5 at effort max.

**The merged text, whole** (`git diff origin/main HEAD -- .github/workflows/toolchain.yml`, inserted by script):
```diff
{{WORKFLOW_DIFF}}
```

**Every rule sentence, before and after** (infra-engineer's table from review round 1, inserted by script; the
verifier spot-checked ten rows: {{VERIFIER_CLAIM_9}}). One wording point on it (security-auditor F-TOKEN-WORDS,
infra-engineer F3, Low): the comment "The only step with the token" is true of run steps; checkout and setup-node,
both pinned by SHA, receive the job's token as their default input, as on main. The comment is his approved text and
changes with the next CI edit (follow-up TOKEN-WORDS).

{{RULE_TABLE}}

## Evidence
**CI on the code head 6d0b5b12945f3ca6b66bdb23036b6c7f90fc9299** (the commits after it change only docs and the
notes): `gate` and `smoke` success (push run 45272654640, pull request run 45272669452); `toolchain` success (push
run 45272654783, job 138036214520; pull request run 45272669543, job 138036262546).

**The stop rule** (plan section 9, revision 4; his yes to its new wording at 19:58:23 UTC) was read on every run of
the new lines. A script checks each clean `toolchain` log against it, 18 points (the reviewers tightened two of them
in round 1: the boundary tests' "Tests  7 passed (7)" is read inside its own block, and the job's time is checked):
18 of 18 on all eight clean jobs (2a654f3, d6e4fb6, ce5a7e7 and 6d0b5b1, push and pull request each; about 135 to
147 s per job against the rule's 10 minutes).

**From the first clean run** (push run 45254907594, job 137974047720, image ubuntu-24.04 20261116.133.1; the lines
are copied from the job log by script):
```text
{{CLEAN_RUN_QUOTES}}
```
- GitHub's check passed with the job's token at "Contents: read, Metadata: read" (the advisor agent's ruling PERM-ATT
  (a) holds). It proves the file is an asset GitHub recorded for that release; it does not prove how the program was
  built.
- The Wasm: 28,991 bytes, sha256 22dad2ebf4be21ce0f123753bb77cfa25ca06886eddace43651601831ee8018b, on every runner
  build; 28,987 bytes in the containers (2026-11-19, and the thread's rehearsal of 2026-11-23: sha256 c4935aca...2c31). The builder's path is 7
  characters longer on a runner and the file 4 bytes larger: consistent with RR-F5 (the builder's path inside the
  Wasm), not shown, since no build with the runner's path exists. The job prints both and judges neither.

**The first text's stop (cb476b1).** Both `toolchain` runs (push 45227423151, job 137877549845; pull request
45227437170, job 137877597017) passed the download, its size and sha256, GitHub's check and the member list, then
refused at the install check: "mode 777, owner root:root, 4096 bytes, /opt" and "/opt/wasm-bindgen-0.2.130 or /opt
is not owned by root, or others than root may write in it". The runner image opens `/opt` to everyone on purpose
(`chmod -R 777 /opt` in its setup script, read by infra-engineer at the image's tag). The plan's review had named
`/opt`'s mode on a runner as the open unknown. The chat stopped and told him at 17:28:43 UTC; nothing was pushed
between the failure (17:25 UTC) and his answer (18:40:57 UTC).

**Planted faults, each failing only at its own step and taken back in the next code commit:**
| Plant | Commits | What the runs showed (push and pull request alike) |
|---|---|---|
| P-BROWSER: one committed render count 3 to 4 | 45a6f0e, back d6e4fb6 | runs 45255955923 and 45255972032: only the browser tests failed, tests 8 and 9, "Error: expect(received).toEqual(expected)", `"drawCalls": 4` expected, `3` received; "2 failed", "10 passed (1.1m)"; no retry line. Two tests, because the runner's default backend is the WebGL 2 fallback |
| P-GH: GitHub's check given a file that is not the release's asset (CI text, his yes) | 47d7024, back ce5a7e7 | runs 45263235401 and 45263244392: only the step "GitHub's own check of the archive" failed, with gh's own line "attestation for 0.2.130 does not contain subject sha256:27c24e42e238fa86ca5feb42b3eeb1a4535040ed5dd0eb183da810f85cff6e10" (the planted file's sha256); the three later steps were skipped; no permission line, no "no attestations found" |
| P-BOUNDARY: one boundary test expects a tick rate of 31 (added in review round 1, not in the plan) | 8d2ce32, back 6d0b5b1 | runs 45270802317 and 45270822315: the job failed in the boundary block, "AssertionError: expected 30 to be 31", "Tests  1 failed \| 6 passed (7)"; the 49 unit tests and both builds before it passed; no type check, Chrome line or browser test ran after it |

After the last take-back, outside `.checkpoints/` and `docs/`, the tree equals the fix commit 280f31c's.
What the plants do not show: that the other ten browser tests bite on a runner; anything about WebGPU; that gh's
signature check itself would refuse (gh compares the file with the release's list first).

**Tests.** The client's unit tests run inside the cut: "scripts/build-checks.test.mjs (18 tests)", "Tests  49 passed
(49)". test-engineer replayed the refusal test against 14 broken copies of the pattern: 14 caught. The specs'
type-check line caught seven of seven planted type errors in the final specs, in the container only and with
near-matching versions (TypeScript 6.0.2, Playwright 1.63.0 types). In the first builder chat's container
(2026-11-19): the browser tests passed three times with one worker, and the build exits 1 with `RUSTFLAGS` set.
**Readings on the runner** (test-engineer, six clean jobs): nothing is thin; for example the allocation test drew 65
to 76 frames against a floor of more than 20, and the hidden-tab bound had 9 to 15 ticks to spare.

**Risk tier 2's own evidence.** The kit's three test files pass with the same counts on main and on the head:
{{VERIFIER_CLAIM_7}}. `.claude/`, every `CLAUDE.md` and `tools/` equal main's. The legacy gate: {{VERIFIER_CLAIM_1}}.
The rule-sentence table is above: +97, -0, no rule of main's changed or removed.

## Reviews
- **Round 1, on the final diff (code and CI; the docs are read by the verifier and the advisor agent):**
  - infra-engineer: APPROVE, the diff needs no change. One Medium on the chat's stop-rule script (two points too
    loose): fixed, and all clean logs read again. Three Lows on wording: taken into this report.
  - test-engineer: proven, with follow-ups; no code or CI line to change. Three Mediums: the goal's wording (CI does
    not pin test counts): this report's Goal uses its sentence; the script's boundary check: fixed; two failures the
    plan called "shown in the container" were not on record: a failing boundary test is now shown on a runner
    (P-BOUNDARY); a planted type error in the Worker's or the page's files stays unshown (follow-up TE-TYPE-PLANTS).
    Four Lows: deferred as follow-ups below.
  - security-auditor: the workflow file passes with no finding above Low. One Medium in the code (F-LOCKPATH: the
    refusal pattern missed `CARGO_RESOLVER_LOCKFILE_PATH`, with which a `--locked` build reads another lockfile;
    older than this pull request, first tested here): fixed in 280f31c with its text, together with its Low
    F-NIGHTLY-INCR. Its Lows on wording: taken into this report.
- **Round 2, on what changed:** {{ROUND_2}}
- **The verifier:** {{VERIFIER}}
- **The advisor agent** read every head that changed more than notes: c37fc11 (the takeover: APPROVE for the first
  write), e195bc8 (the corrected text: APPROVE), and the final head: {{ADVISOR_FINAL}}

## The docs pass (his words of 2026-11-19, 16:08:04 UTC: it "stays with the CI pull request")
- `docs/DECISIONS.md`, by script (`decisions-3.mjs`, every check "yes", his words byte for byte from the transcripts):
  his go on #61 with BROWSER-SET (a); VISION-NOTE (a); a pointer to his browser check; his two notes; the pause of the
  first builder chat; from the thread: the takeover, ANSWERS-7, LONG-RUNS, WRITE-THREAD (replaced), WRITE-WHERE,
  BUILD-SEAT, ACCT-3; from this chat: his block (CI-MODEL (a)), CI-WRITE-1 with his mode rule and GH-TOOLS (a), his
  yes again, OPT-PATH (a), FIX-WRITE (a), PLANT-WRITE (a) with the take-back in advance, and FIREFOX-TESTS (b) with
  the start of change-set 3a. One dated pointer under the entry of 2026-11-19, 13:36:37 UTC.
- `docs/reports/2026-11-19-cs2-browser-check.md`: his eight browser reports whole, with their reading.
- `docs/VISION_V2.md`: the dated note he approved (VISION-NOTE (a)).
- docs-keeper: {{DOCS_KEEPER}}

## Decisions and deviations
- **Two CI writes instead of one, and four in all.** The first text could not pass on a runner; the correction is
  his OPT-PATH (a) and FIX-WRITE (a). His block's point 7 counted three writes with the first text's checksums; his
  yes of 19:58:23 UTC set the new ones.
- **A third plant** (P-BOUNDARY) beyond the plan's two: this chat's call on test-engineer's finding, a test file,
  no CI text.
- **This chat cannot write to GitHub** (GH-TOOLS): cloud chats have no GitHub tool now, the platform refuses
  `gh pr create`, and the guard keeps `gh api` read-only. He opened #64 himself (17:21:24 UTC; made a draft by
  him before his message of 18:40:57 UTC; title and text set by his Advisor from the notes) and presses Ready and Merge himself, by the steps of
  his answer GH-TOOLS (a), which stand in for the "expected head" part of his block's point 9.
- **No saver script in this chat** (a departure from `.claude/CHECKPOINTS.md` section 5; the advisor agent ruled it
  acceptable): the worktree's local branch is named `cs/ci-wasm`, because the main folder holds the `claude/` name
  and is never switched, and the saver pushes to the local name. The card and notes were committed by path and pushed
  by hand after every step.
- **A push refused once** by a settings rule against forced pushes: it matched a `+` in a clock command chained after
  the push. The push was repeated alone; nothing was forced.
- **The session's worker process restarted once** (between 20:55 and 21:01 UTC), while planted runs were being read;
  nothing was lost.
- **What went wrong before this chat, from the records:** a compiled Python file
  (`.checkpoints/ci-wasm/__pycache__/rr-chacha-ref.cpython-313.pyc`, 7,356 bytes) was committed by the first builder
  chat's saver (autosave b7325df, 2026-11-19, 16:39:03 UTC), found at 17:33:58 UTC and removed (395a370, 17:39:12
  UTC). It stays in this branch's history and enters main's with the merge. His words put the `.gitignore` line
  "With change-set 3". The project thread's first answer on where the CI file could be written was incomplete and
  was corrected by the thread (WRITE-THREAD, WRITE-WHERE in DECISIONS).
- **Moved to change-set 3, as the plan said:** SA-TREE (a test that every runtime crate has its overflow override)
  and the fix for RR-F5.

## Not verified, not covered
- Firefox, WebKit, Brave, WebGPU (on the runner the "default backend" test is a second WebGL 2 test), a real GPU, any
  timed mark on a named machine, how the tool's program was built.
- CI does not pin the test counts, and `toolchain` is not a required check: a red `toolchain` does not block a merge
  by itself; it is read by hand before every merge.
- Never shown failing: a planted type error in the Worker's or the page's files; the build's refusal of a cargo
  setting on a runner (the cut's `env -i` drops such names); the hidden-tab bound at `skeleton.spec.ts:104` being hit
  (it holds by arithmetic; the plant that would reach it is caught two lines earlier).
- Inside the cut a hostile dependency can still fake every result (accepted in #60); the cut's PATH starts with
  folders the cut's user can write on this image, and checkout's post step runs after the cut (CI-CUT, known). The
  root-owned tool protects its own file only.
- The runner's CPU model; pull requests from forks (every run came from this repository); the PATH inside the cut
  (no run prints it).
- {{NOT_VERIFIED_VERIFIER}}

## Risks
1. The image's Chrome moves with the image (157.0.8103.50 in the plan, 157.0.8103.78 in every run five days later).
   A later Chrome can outrun Playwright 1.65.0 or move the allocation test's byte windows: a visible red, to be read
   and not re-run into green. His CHROME-SOURCE (a) accepts a moving browser.
2. `toolchain` is not a required check (REQUIRED-CHECK, his setting).
3. A browser test that fails once is a defect to find; no bound is lowered blind.
4. A second merge to main within about 2.5 minutes cancels main's first `toolchain` run (CANCEL-ON-MAIN, known).

## Follow-ups (for STATUS)
TE-COUNT (the gate v2 fails a run with a skipped or missing test) · TE-TYPE-PLANTS (a planted type error in a Worker
file and a page file) · TE-TSC-LINES (each type check prints its own result line; CI text, his yes) · TE-104 ·
TE-MONO (`performance.now()` at `skeleton.spec.ts:92` and `:98`) · TE-WATCH (on an image or Chrome change read the
byte windows and the default backend first) · TE-CPU · TE-HIDDEN-FRAMES · IMAGE-CHROME · CUT-PATH · TOKEN-WORDS ·
REQUIRED-CHECK, CANCEL-ON-MAIN, CI-CUT, NODE-26 (known) · RR-F5, SA-TREE and the `__pycache__/` line (change-set 3)
· Firefox in CI with the CI pull request after 3a (his FIREFOX-TESTS (b)) · the line for CLAUDE.md on the third
account (ACCT-3, his) · GH-TOOLS for the next pull requests · the saver and a worktree's local branch name.

## Proposed next step
Change-set 3a, "Rules and boundaries", by this chat, from the approved plan (`cs3-plan.md`, draft 3, with his
ANSWERS-7). His words of 21:24:59 UTC: it starts when #64 is merged, with no second ask; above 40 % of context the
chat tells him first, and he compacts.

## Decisions needed
- **GH-NEXT: how are pull requests opened, marked ready and merged for change-set 3a and after?** Why: his answer
  GH-TOOLS (a) was "for this pull request", and the same wall stands for the next one. Options: (a) as now: he opens
  the draft, presses Ready and Merge, by the same steps; (b) he gives this account a GitHub connector (it may need a
  new chat); (c) a small kit pull request that lets a day session write pull requests through `gh api` with the
  guard's merge check moved to it (risk tier 2, Opus 5.5 work by his words of 2026-11-12). Recommendation: (a) for
  3a, and (c) planned with the gate step, where the guard changes anyway.

## For Mustafa
{{FOR_MUSTAFA}}
