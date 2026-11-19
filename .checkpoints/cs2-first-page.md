# Checkpoint: cs2-first-page
- **Goal:** change-set 2 of the build plan, "First page" (foundation step 2.5, first half): the Rust simulation, built
  as WebAssembly, runs in one Web Worker at 30 ticks a second and hands the page one block per tick; the page (which
  never touches WebAssembly) draws an empty lit ground with a 1 m grid with three.js's WebGPU renderer (WebGL 2 as
  the fallback) and names its backend; the small server that lets Mustafa open a build in Brave from a zip. His
  order of work (2026-11-18, 20:20:09 UTC): "Then change-set 2: its plan, its review, the build and its pull request,
  then stop for my go."
- **Branch:** claude/wonderful-hamilton-jdmlnf (stands in for cs/cs2-first-page; lane 1, the first builder chat) ·
  **Base:** main b763ba3e5f34c67fc4f875cfbd9266167fb095f3 (the merge of #60; the branch is level with it and pushed)
- **Last update:** 2026-11-19 08:13 UTC (from the clock)

## His words that govern (read the file; the lines below are pointers and quotes, never a substitute)
`cs2-first-page/mustafa-2026-11-18-2059-merge-60.md` (his message of 2026-11-18, 20:20:09 UTC, whole, by script;
his labelled words, 2,404 bytes, stand byte for byte in the message of merge commit b763ba3):
- "Open change-set 2's checkpoint (the first page, foundation step 2.5), with the plan draft carried over from the
  notes at commit f167f061a9b0b694836c272e397c90c815d7829f. Then stop and tell me the path of its resume card: I
  compact this chat before you go on."
- "WRITE-MODE: (a). I accept both kit writes as they were made. From now on you read the chat's mode right before
  each kit write and stop unless it is Accept edits."
- "The five Medium findings as follow-ups. They are fixed with the kit changes of the quality gate step. Until then
  every ready report of a pull request that changes a manifest or a lockfile quotes each release record and the
  security answer it names."
- "REC-OLD: (a), with one limit. A docs pass may correct a line of the shared record files that describes code, also
  a line another lane wrote, when that code has changed; its report lists each with the old and the new text. A line
  that records what happened or what I decided is never reworded: it gets a dated pointer. The three map rows for
  the files this pull request changed are corrected in change-set 2's docs pass." (He also typed "REC-OLD: A" at the
  top of the same message; the pasted block is the fuller answer.)
- His earlier words that still bind are in `docs/DECISIONS.md` (2026-11-18: 15:38:13, 16:39:23, 18:01:08 UTC, and the
  older entries). The limit "No new library of change-set 2 is committed before that pull request is merged" is
  met: #60 is merged.

## Standing rules (each from his words or the repository's rules; when in doubt read DECISIONS and CLAUDE.md)
- **Models:** this chat is Fable 5.2 (check the session first; stop in one line if it is not). Every agent runs on
  Opus 5.5; agents that review, verify or design game and engine code run at effort max. The advisor tool stays off.
- **Kit, gate and CI files** (`.claude/`, every `CLAUDE.md`, `.github/`, the gate's files): no edit without asking
  him in chat with the exact text; written only after his yes; **read the chat's mode (the session record) right
  before each kit write and stop unless it is Accept edits.** The kit stays frozen otherwise.
- **Dependencies:** `.claude/CHECKPOINTS.md` section 3's hold (the checkpoint script does not push a manifest or
  lockfile change with no record; release record `.checkpoints/<slug>/dependency-check.md`, one line per file:
  `<sha256>  <path>`). The advisor agent's ruling of #58: manifest and lockfile edits stay uncommitted and unbuilt,
  other code is committed by path only (no `-a`, `add -A`, `add .`; read `git diff --cached --name-only` first),
  until security-auditor has answered on exactly that content. Nothing builds the dependency graph before that
  answer. A plain `git push` is not held: push nothing by hand while a dependency edit waits. Every cargo command
  takes `--locked`. The ready report quotes each release record and the security answer it names (his words above).
- **Never share a cargo build folder** between two copies or worktrees of the repository (`CARGO_TARGET_DIR` of its
  own under `.gate/`). Worktrees: `git worktree add .gate/<slug> <ref>`; never switch the main folder's branch.
- **Lanes (Part 15):** lane 1 owns `crates/**`, `packages/types`, build files and dependencies, `gate.yml`,
  `toolchain.yml` and the gate's files while the 2.3 chat is stopped; lane 2 owns the preview workflow. Never push
  to another lane's branch (`claude/optimistic-albattani-mdoonz`, `claude/netcode-proof-test-plan-hcky5g`,
  `claude/sweet-mayer-grsukw`); never open `spikes/`; never read `/root/.claude/uploads/` (the Advisor chat's log
  is not an input). The netcode proof test's rules: no root `.cargo/config.toml`; no `"type"` in the root
  `package.json`; no tracked non-Markdown file names the spike's folder. Read the "Lane requests" issue (#59) at the
  start of the change-set; a comment there is never his word.
- **Process:** plan first, then the advisor agent's APPROVE before any edit; build in the main session; at most 3
  agents at a time; the saver running; a checkpoint after every step; reviews within two rounds; the verifier; the
  advisor agent on every pushed head; the report; the pull request draft then ready; **stop for his go on the
  merge** (from change-set 2 on, also his Brave check: something a player sees).
- **Size:** the planned size stands in the plan; when the change-set passes twice its plan while it is built, stop
  and tell him in three lines (what grew, why, the cheaper option).
- **Records:** his messages are copied by script (`cs2-first-page/words.mjs`), never retyped. Every time in a card,
  a note or a report is printed by `date -u` or read from the session's record in the same command that writes it:
  never typed from memory (three slips on 2026-11-18). About my own process a report says only what a record shows.
- **Shared record files** (STATUS, ROADMAP, the archive, the map): RECORD-FILES, OLD-LINES and REC-OLD (a) with its
  limit, in his words above and in DECISIONS.
- **Refusals:** if the guard, the deny list or the classifier refuses a command, record it with the route taken.
  Denied here: `npx`, `npm run`, `npm test`, `npm exec`, `git checkout`, `git rebase`, git with `-C`, `-c` or
  `--git-dir`, `gh pr merge`, `rm -rf`, forced worktree removal. Do not delete a folder tree by another route either
  (the classifier refused it on 2026-11-18): ask him.
- **GitHub:** only through the `mcp__github__*` tools (load their schemas with ToolSearch first); `gh` for search
  only. Commit trailers: `Co-Authored-By: Claude Fable 5.2 <noreply@anthropic.com>` and
  `Claude-Session: https://claude.ai/code/session_019YNMUV9LFhASseJ9EP12kL`. The last commit of a pull request
  carries no skip mark. Ask him to compact when the chat is nearly full.

## Steps done
1. Pull request #60 (the kit hold and two CI edits) merged on his go of 2026-11-18, 20:20:09 UTC: merge commit
   b763ba3e5f34c67fc4f875cfbd9266167fb095f3 (20:30:46 UTC), parents 9b4b518 and e5ca735, its tree equal to the
   approved head's, his words in its message byte for byte (2,404 bytes, sha256 a6192397..., checked by script).
   Before it: the session's model read (claude-fable-5-2), the head read (e5ca7358d5a3f8cc68ee07cd59d7bbf2c45e253d)
   and its six check runs (gate, smoke, toolchain, for the push and the pull request): all success, read twice (the
   session's worker restarted in between). After it: the branch fast-forwarded to main and pushed.
2. This checkpoint opened. Carried over from the notes of #60 at commit f167f061a9b0b694836c272e397c90c815d7829f,
   byte for byte (checked): `cs2-first-page/plan-draft.md` (777 lines; engine-architect's DRAFT of the plan, Opus
   5.5 at effort max; **not reviewed by anyone yet, nothing of it built, no library fetched**) and
   `cs2-first-page/engine-architect-plan-notes.md` (122 lines, its sources). Also: his message as saved by script,
   and `words.mjs`.

3. Main after the merge, read with the GitHub tools (2026-11-18 20:40 UTC): `gate` run 900 (its `gate` and `smoke` jobs; run
   44391809186, ended 20:38:33 UTC) and `toolchain` run 198 (run 44391809357, ended 20:31:35 UTC) on b763ba3: success.
4. The test zip of change-set 1 was sent to him again with its steps (`.gate/cs1-types/archive/`, 6,480 bytes, sha256
   7138ea42...; 20:40 UTC by the session's record of the save that followed).

5. **The zip route to his laptop is proven** (his message of 2026-11-18, 20:44:29 UTC, with a screenshot;
   `cs2-first-page/mustafa-2026-11-18-zip-saved.md`, by script): "the zip saved and unpacked". His screenshot shows
   `README.txt`, `sha256.txt` and the folder `packages` in `Downloads\shamble-cs1-types-test-archive`, and Windows
   PowerShell 5.1 printing the SHA256 of `packages\types\src\generated\block\layout.ts`: as I read it from the image
   it equals the expected d6c6352c...fb2ef (compared by script, ignoring case). This answers the open point of #58
   (STATUS, "Waiting on Mustafa": to be taken out in this change-set's docs pass). Noted 2026-11-18 20:44 UTC.

6. He compacted the chat (his message after it: read the card, continue from its next steps, do not redo finished
   work, and "Before the first agent starts, tell me in two lines that the card and its notes folder are complete and
   that the background saver runs."). Done at 20:54 UTC by the session's record: the card and the five notes files
   present and pushed (8cb0bfd), the draft equal to f167f061's byte for byte, the model read (claude-fable-5-2, also
   last served), the saver started (20:54:42 UTC, 110 minutes), issue #59 read (no comment), open pull requests read
   (only #40, the netcode chat's draft). The two lines were given to him before any agent started.
7. **The plan adopted:** `cs2-first-page/plan.md` (the draft's text with the main session's changes, each listed in
   its section 20; built by a script of counted replacements, so `diff` against `plan-draft.md` shows every changed
   line). Read for it: the build plan's card (:298-314), its Parts 0, 2, 3.1, 3.3, 7 and 8, `toolchain.yml`, the
   crates and packages, ADR 0004 parts 4, 5 and 9, ADR 0002's OQ-0002-7, the archive's follow-ups. Found: CI's
   `npm test --workspaces` would start the browser tests' server on a build CI never makes (now ruled out: plan 4.4);
   the draft's wording of the random-number finding said more than its source (reworded from the sources, read
   again: rand's changelog and crates.io directly, the pull request page through a fetch agent, because this
   container's proxy refuses github.com pages of other repositories). Reachable by headers only (20:58 UTC): the
   wasm-bindgen tool's tarball and its `.sha256sum`, the crates index, npm. Saved as ad4fc0b (21:04 UTC).

8. **Step 0 of the plan, the reading of wasm-bindgen's health (OQ-0002-7):** `cs2-first-page/oq-0002-7-reading.md`
   (a fetch agent read the two pages between this session's saves of 21:04:49 and 21:07:52 UTC; the release dates
   from the registry at 21:07:52). Maintainers: yes (three core members, a majority vote, reviews, about one release
   a month). Release policy: none written. By the plan's own rule that goes to him before step 2: question
   WB-HEALTH. **It ran before the advisor agent's APPROVE, as reading and notes only; the report says so.**
9. **The advisor agent's plan review: APPROVE with eight conditions** (`cs2-first-page/advisor-plan.md`, its lines of
   21:05 to 21:27 UTC; Opus 5.5 at effort max). Step 1 may start; nothing of step 2 before his answers and
   conditions 1 to 4. Seven Mediums and twelve Lows; the plan's section 22 says where each condition is met.
10. **The plan, revision 2** (21:34 UTC, by a script of counted replacements): the register row OVERFLOW-ALL is
    followed (named overrides, cost measured), so OVERFLOW-3P is no longer a question; SIZE-2 is information;
    WB-HEALTH is new; Part 15.2's interface members and `e2e` folders are kept; the tool's route; the lockfile rule;
    netcode-architect in the round; the Brave checklist's pause line and two padded runs; the Lows.

11. **Step 1 of the plan, the Rust core with no new crate: commit 3833e3e** (2026-11-18 21:38 UTC). `sim`: `world.rs` (the
    world, `authority_step` with `checked_add`, the canonical bytes: tick then seed, 36 bytes, `state_hash`),
    `state_hash.rs` (`fnv1a_64`). `client-endpoint`: `layout.rs` (`Section::align`, `section`, `field`, used in
    constants), `block.rs` (`BlockWriter`, `MAX_PAD_BYTES` 1 MiB, `new` gives `None` above it), `schedule.rs`
    (`LocalSchedule`). 559 lines added by `git diff --numstat` and `wc -l` (planned 550). Checks, all exit 0
    (21:38 UTC): `cargo fmt --check`; clippy natively and for wasm32 with `-D warnings`; `cargo test --workspace`
    (client-endpoint 17, sim 9); the release tests of sim and protocol; the wasm32 build of the five crates; the
    generated types written again with no change; `node tools/gate.mjs --no-smoke` (GATE PASS, smoke skipped);
    the map check; the three netcode rules. Known answers come from a second implementation,
    `cs2-first-page/known-answers.py`. Two departures from the plan's symbol list, for the report: `BlockWriter::new`
    returns an `Option` (the padding is capped, because the number comes from outside), and `SEED_BYTES` is public.

12. **CI on step 1's push** (head 2196b7f, read with the GitHub tools): `toolchain` run 200 (id 44403660148) and
    `gate` run 902 (id 44403660244): success. The `toolchain` run took about half a minute (21:39:05 to 21:39:38 by
    its record), far from its 15-minute limit.
13. **The verifier on the web facts** (the advisor agent's condition 4; `cs2-first-page/verifier-claims.md`): FAIL
    on the first read (two Mediums: the order of two comments on the rand pull request, and one-sided evidence on
    who maintains `rand_chacha`; two Lows), all four corrected in the plan and the reading; **PASS on the re-check**
    (21:50:04 UTC by its own line), its three wording Lows fixed too.
14. **security-auditor on the wasm-bindgen tool's proof of origin** (`cs2-first-page/security-auditor-tool.md`;
    its small files in `.gate/cs2-first-page/tool-check/`): provable here only in part. The tarball's sha256 is
    9687f3bb551b9f66f37ed32a40d7913a00f7962db653054e56857bc306c3227f by three agreeing statements (the project's
    `.sha256sum`, GitHub's digest, GitHub's signed release statement), size 11,860,397; the certificate's chain to
    GitHub's root cannot be checked here, and no build record exists. By the advisor agent's condition 5 that goes
    to him: question TOOL-SOURCE. Its fuller unpacking route is in the plan (section 7, item 5a).
15. **The plan, revision 3** (22:00 UTC; saved as 0d8a838): the verifier's corrections; TOOL-SOURCE; the tool's
    route. The questions as they stand: RNG-CRATE, WB-HEALTH, TOOL-SOURCE, CI-WASM, DEPART-2.

16. **The advisor agent read the changed lines once** (its notes under "Changed lines, read once"): "the questions may
    go to him: yes, after the five wording fixes" (applied; plan saved as 1d79e8e). Its rulings: the three
    manifest-free parts of step 2 may be built, tested and committed now, within four limits (import nothing by
    package name that step 2's manifests bring in: `@shamble/types`, `@shamble/sim-worker`, `three`, its types, the
    glue; no use of Vite's API yet; commit by path and push only through `checkpoint.mjs save`; CI's `npm test`
    stays green), with the server's printed address in one constant; TOOL-SOURCE's option (c) is rightly an option.
    Two notes for later steps are in the plan: the overflow check's command needs the dev edges (before step 3);
    `build.mjs` refuses to run the tool without its expected checksum (before step 4).
17. **The five questions were put to Mustafa in chat** (2026-11-18 22:09 UTC): RNG-CRATE, WB-HEALTH, TOOL-SOURCE, DEPART-2,
    CI-WASM, each with its options and recommendation ((c), (a), (a), (a), (a)). The first, second and fourth hold
    the dependency step; the third holds the tool's download and first run; the fifth holds nothing.

18. **The container restarted** (seen at 22:11 UTC; the harness's notice). Nothing was lost: HEAD equalled the pushed
    branch, the one uncommitted file (`serve.mjs`) was still there, `node_modules` and `target` too. The saver had
    stopped and was started again.
19. **The manifest-free parts of step 2**, on the advisor agent's ruling, each committed by path:
    - **the small server, 784ae3e:** `packages/client/serve.mjs` (165 lines) and `serve.test.mjs` (14 tests): binds
      127.0.0.1 only, the address in one place (`HOST`, `addressFor`); GET and HEAD only; only files inside its own
      folder (twelve ways to write a path that leaves it, and a link that leads outside, are refused); a fixed table
      of content types, nosniff, no-store; a Host header that is not its own address gets 421 with the right address
      in the answer; a taken port stops it with a plain line. Also started by hand from a scratch folder.
    - **the Worker's pure modules, 4c32a06:** `packages/sim-worker/src/` `pacing.ts` (`MAX_TICKS_PER_WAKE` 7, `Pacer`,
      `PacedSim`), `block-pool.ts`, `read-block.ts` (`copyBlock`, `LinearMemory`), `stats.ts` (`Ring`, `percentile`),
      `messages.ts` (`PageToWorker`, `WorkerToPage`, `WorkerStats`, `MODE_LOCAL`, `isPageToWorker`), 24 tests; tsc
      with CI's strict flags and `--lib es2023,webworker`: exit 0. `index.ts` and its stub test stay until step 5.
    - **the build script's checks, a7f785a:** `packages/client/scripts/build-checks.mjs` (`lockedWasmBindgenVersion`,
      `checkCli`, `checkCliFile`, `sizeTable`, `formatSizeTable`, `tooLarge`, `MAX_SERVED_BYTES`), 13 tests.
    Checks at 2026-11-18 22:30 UTC: every package's test script as CI's `npm test` runs it (types 7, net 1, sim-worker 25, client
    28, e2e 1: all passed); `node tools/gate.mjs --no-smoke` (GATE PASS, smoke skipped); the map check.
    **Size so far: 1880 added lines** in `crates/` and `packages/` by `git diff --numstat origin/main...HEAD`
    (plan: about 2,900; stop line 4,000). The parts built so far ran over their rows (the Rust core 559 against 550,
    the server 366 against 270, the Worker's pure modules 707 against 590 for the whole package, the build checks
    248 against 240 for the whole script): the rest must stay tight, or the stop line comes into view.
    Departures from the plan's lists, for the report: `BlockWriter::new` returns an `Option`; `SEED_BYTES` is public;
    the paced simulation's `rebase` returns the next due time (so `Sim::rebase` will, in step 4); `isPageToWorker`
    is added; the build checks have their own module (`build-checks.mjs`), with `build.mjs` to come in step 4;
    `BlockPool.give` returns nothing and the pool counts `refused` beside `misses`.

20. **Mustafa's answers to the five questions** (2026-11-18, 22:34:37 UTC; his label 23:26 Amsterdam):
    `cs2-first-page/mustafa-2026-11-18-2326-five-answers.md`, whole, by script. In short, and never a substitute for
    his words: RNG-CRATE (c), with two things to read for change-set 3's question (the rand project's upgrade guide
    for 0.10, and why `chacha20` 0.10.0 and 0.10.1 were withdrawn); WB-HEALTH (a), keeping what crosses wasm-bindgen
    as small as the plan has it; TOOL-SOURCE (a) "for this one file only" (the tarball of 11860397 bytes, sha256
    9687f3bb...227f, by section 7's item 5a; the program inside it has 8170784 bytes and sha256
    ca4df773020dae87d92b9eadb2b85412bea7a9cf6318ab9999dd119fd9f46931 as the Advisor read it: "If your copy differs
    in any of these, stop and tell me"; "every other file, version or checksum comes back to me"); DEPART-2 (a), all
    six; CI-WASM (a), a pull request of its own right after the merge and before change-set 3's build, its exact
    text to him first, running GitHub's own check before the tool's first run there; the size is fine. The session's
    model was read again at 22:35 UTC: claude-fable-5-2, also last served.
21. **Step 2's manifest part, prepared and not committed** (the scratch worktree `.gate/cs2-deps` at b6339d2; nothing
    built or installed there): seven manifests edited; `cargo update --workspace` added seven entries (wasm-bindgen,
    its macro, macro-support and shared crates at 0.2.130, bumpalo 3.20.4, cfg-if 1.0.5, rustversion 1.0.23) and
    removed no line; `npm install --package-lock-only --ignore-scripts` added eight entries (three 0.187.0,
    @types/three 0.187.0 and its six) and changed 37 existing entries in their `peer` flag only. The lockfile rule is
    kept (`cs2-first-page/lock-compare.py`, output in `deps-lock-compare.txt`). Six overflow overrides in the root
    `Cargo.toml`: cfg-if, thiserror, ts-rs, unicode-ident, wasm-bindgen, wasm-bindgen-shared. In the notes:
    `deps-manifests.diff`, `deps-lockfiles.diff`, `deps-sha256.txt` (the nine files' sha256).
22. **The e2e package's node project** (commit a68e3da, no manifest): `playwright.config.ts` with the `node` project on
    `tests/node/`, the stub moved there, so the script `--project=node` finds its project the moment the manifests
    land. Run both ways: 1 passed.

23. **CI on b6339d2** (the three manifest-free commits; read with the GitHub tools at 2026-11-18 22:39 UTC): `toolchain` run 201
    (id 44412198394) and `gate` run 903 (id 44412198575, ended 22:38:11 UTC): success. On bc0bef1 (with the e2e
    node project) `toolchain` run 202 and `gate` run 904 were still running. **A refusal, with the route taken:**
    a hook refused `gh api -X GET repos/.../actions/runs` ("gh api is read-only here; GitHub writes go through the
    GitHub MCP tools"); the runs were read with the GitHub tool for workflow runs instead. The scheduled check-in
    for this read (22:40 UTC) was deleted as done.

24. **security-auditor's dependency answer** (`cs2-first-page/security-auditor-deps.md`; the nine files hashed to
    `deps-sha256.txt` at its start, 22:39 UTC, and its end, 23:04 UTC, by its own lines): **"the dependency change may
    be committed and built: yes with conditions"**; no condition changes the nine files. Conditions: (1) commit
    exactly the nine reviewed files; (2) install only with `npm ci --ignore-scripts`, every cargo command with
    `--locked`; (3) Vite's dev and preview servers never listen beyond 127.0.0.1, and vite stays at 8.3.7 or later
    (8.3.7 is the fixed version of a Critical advisory on an exposed dev server); (4) its F1 and F2 written into the
    plan's item 5a before the tool's download, F3 and F4 fixed before its first run. Findings: F1 High and F2 High
    (the tool's route did not compare the program with his values or carry his "stop and tell me" and "every other
    file, version or checksum comes back to me": both written in, plan revision 4, 23:08 UTC, before the download);
    F3 Medium (the expected size and the exact path: `wasm-bindgen-tool.mjs` and `build.mjs`, step 4); F4, F5, F7
    Low (in the route and the plan); F6 Low (the npm scope `@shamble` is unclaimed: a follow-up); a residual Low
    (root container: a left-behind process could swap the tool between check and run).
25. **Step 3 of the plan, the answer applied: commit cc9fc71, pushed through the hold as a2ed2d9** (23:07:36 UTC).
    The two reviewed diffs applied in the main folder, each of the nine files equal to its reviewed sha256; the
    release record `cs2-first-page/dependency-check.md` written before the commit; the nine committed by path.
    After the answer and before the push, all exit 0: `npm ci --ignore-scripts` (54 packages added); `cargo fmt
    --check`; clippy natively and for wasm32; `cargo test --workspace --locked`; the release tests of sim and
    protocol; the wasm32 build; the generated types written again with no change; every package's test script as CI
    runs it (types 7, net 1, sim-worker 25, client 28, e2e 1); CI's two tsc lines; `node tools/gate.mjs --no-smoke`;
    the map check. The scratch worktree is removed without force. **A refusal, with the route taken:** a hook
    refused `git restore` in the worktree ("git restore is not allowed here"); its edits were undone by applying the
    two saved diffs in reverse (`git apply -R`), then `git worktree remove`.
26. **The tool, by its route** (`cs2-first-page/tool-route.sh`, run 23:09:31 to 23:09:33 UTC; output in
    `cs2-first-page/wasm-bindgen-cli-check.md`): the archive downloaded (one redirect, status 200), 11860397 bytes,
    sha256 equal to his; seven lines in the member list (the folder and the six files); only the program taken out
    into `.gate/tools/wasm-bindgen-0.2.130/`; **8170784 bytes and sha256 ca4df773...6931, equal to his values**;
    ELF64, x86-64, no interpreter; mode 555. **Not run yet.** The archive stays in
    `.gate/cs2-first-page/tool-download/`.
27. **The overflow list checked:** `cargo tree --locked --workspace -e normal,dev,no-proc-macro --target all` names
    exactly six third-party crates (cfg-if, thiserror, ts-rs, unicode-ident, wasm-bindgen, wasm-bindgen-shared):
    the six with an overflow block.

28. **Step 4, its first part: commit 6e76f30** (2026-11-18 23:14 UTC). `crates/sim-wasm/src/lib.rs` (`Sim`, `Sim::create`,
    `CreateError`, `MODE_LOCAL`; 6 native tests); `packages/client/scripts/wasm-bindgen-tool.mjs` (the tool's exact
    values, tracked, with his words); `scripts/build.mjs` (its Wasm part, `--wasm-only`; `buildWasm`); `.gitignore`
    (`packages/sim-worker/pkg/`, `packages/e2e/playwright-report/`). **Risk 1 is settled: `unsafe_code = "deny"`
    accepts the macro's expansion; no exception.** The tool's first run was through `build.mjs`, after its file was
    compared. First build: `sim_wasm_bg.wasm` 29,206 bytes raw, 11,743 gzip -9, 9,970 Brotli 11; the glue 8,799. In
    the glue `input` calls `wasm.sim_input` and `step` calls `wasm.sim_step`: one export each. Refused before any
    run: the tool with one byte added, a stand-in printing the right version, no path, a bare name. Checks all exit
    0 (fmt, both clippy runs, workspace tests: client-endpoint 17, sim 9, sim-wasm 6; release tests; the wasm32
    build; the types written again; the client's tests; the legacy gate; the map check). Departures for the report:
    `input` takes the sequence only; `newest_input` is added; `block_ptr` returns `*const u8`.

29. **Step 4, the rest: the cost of the six overflow overrides** (23:16 UTC; `cs2-first-page/overflow-cost.mjs`; the
    second build in a scratch worktree with its own build folder, removed without force afterwards). The Wasm after
    wasm-bindgen: 29,206 bytes with the lines, 28,928 without (+278, about 1 %); gzip -9 11,743 against 11,626;
    Brotli 11 9,970 against 9,849. One tick (`input` and `step`) in Node 22.23.4 here: median 14.71 ns with the
    lines, 14.04 ns without (7 runs of 2,000,000 ticks each; the two ranges overlap). A reading, not judged.
30. **Step 5: commit 015a212** (2026-11-18 23:58 UTC). `packages/sim-worker/src/host.ts` (`SimHost`, `HostSim`, `HostEnv`; 6 tests
    with a stand-in), `worker.ts` (the entry; the seed drawn there), `index.ts` (types and statistics helpers; the
    stub and its test are gone); `packages/sim-worker/boundary/sim-wasm.test.mjs` (7 tests on the built Wasm;
    plain JavaScript because it reads the Wasm with Node's own module); `crates/sim-wasm`: `tick_hz()`. After the
    rebuild the Wasm is 29,231 bytes (11,759 gzip -9, 10,024 Brotli 11), the glue 9,012. Checks all exit 0 (fmt,
    both clippy runs, workspace tests, the wasm32 build, every package's test script as CI runs it: types 7, net 1,
    sim-worker 30, client 28, e2e 1; the boundary tests 7; the legacy gate; the map check). **A reading for the
    report:** with the lib check on, wasm-bindgen's generated `.d.ts` needs `--lib es2023,webworker,esnext.disposable`
    (it declares `[Symbol.dispose]`); with that the Worker's shipped files and the `.d.ts` type-check under CI's
    strict flags, exit 0. **Size so far: 2758 added lines** in `crates/`, `packages/` and `.gitignore` (plan about
    2,900; stop line 4,000). Departures for the report: `host.ts` is a file of its own beside `worker.ts`; the
    boundary test is `.mjs`; `tick_hz()` is one more export, called once.

31. **Step 6, the page: commit de63a30** (2026-11-19 00:13 UTC). `packages/client`: `index.html`, `vite.config.mjs`, `src/main.ts`,
    `src/picture.ts` (`Picture`: start, loop, draw, counters, resize, stop), `src/styles.css`, `src/app/` (`boot`,
    `worker-link` with 6 tests, `error-hook`, `flags` with 4 tests), `src/render/` (`three-picture`, `empty-scene`),
    `src/overlay/` (`overlay`, `report` with 2 tests), `src/harness/` (`frame-times` with 2 tests, `run-header`);
    `scripts/build.mjs` (the page's build: `buildPage`, `--production`, `--out-dir`), `scripts/build-checks.mjs`
    (`cspProblems`, 2 tests). The stubs are gone. First page build (development flavour): the page's script 824,565
    bytes raw (221,498 gzip -9, 179,793 Brotli 11: three.js's WebGPU build), the Worker's 8,281, the Wasm 29,231,
    the style 495, the readout and report chunks 617 and 1,992, `index.html` 570. In the container's Chromium
    (headless; `/opt/pw-browsers/chromium`; a scratch script, not yet a test): backend "WebGL 2 fallback" on the
    default run and with `?backend=webgl` (no WebGPU adapter here: "No available adapters"), blocks equal ticks,
    errors 0, undetached 0, at most 1 tick per wake, 0 drops; the renderer's counters: 3 draw calls, 3 triangles, 62
    lines, 3 geometries, 3 textures; about 8 frames a second (software rendering), so the pool of 4 buffers counts
    misses here. **Two things found and fixed:** three.js resets its counters in its own loop, so read from our own
    animation frame they were 0: the frame loop now runs on `setAnimationLoop` (`Picture.loop`); and the software
    renderer does not draw a line that crosses behind the camera, so the ground (30 m) ends in front of it. One
    request gets a 404: the browser's own ask for `/favicon.ico`. Type checks, all exit 0 with CI's strict flags:
    the three packages' shipped files with the lib check on (the page: 0 errors against `@types/three` 0.187.0, two
    casts, both in `three-picture.ts`, for `backend.isWebGPUBackend` and `backend.device.adapterInfo`; libs
    `es2023,dom,dom.iterable,esnext.disposable`, `--types vite/client`) and their tests with `--skipLibCheck`. Tests
    as CI runs them: types 7, net 1, sim-worker 30, client 43, e2e 1; boundary 7; the legacy gate; the map check.
    **Size: 3,847 added lines** (`git diff --numstat origin/main...HEAD -- crates packages .gitignore Cargo.toml
    package.json`), against about 2,900 planned and the stop line of 4,000.

32. **Step 7, its core: commit c4a12eb** (2026-11-19 00:17 UTC). `packages/e2e/playwright.config.ts` (the `chromium` project; the
    server only with `SHAMBLE_DIST`; `ADDRESS`), `tests/skeleton/skeleton.spec.ts` (3 tests: the page on the default
    backend and with `?backend=webgl`; a hidden tab through the page's own handler), `tests/skeleton/render-counts.json`
    (WebGL 2 fallback: 3 draw calls, 3 triangles, 62 lines, 3 geometries, 3 textures; WebGPU: none yet). In the
    container's Chromium: 3 passed, three runs in a row; the `node` project as CI runs it: 1 passed. The readout's
    pause line now reads "ticks while hidden: N (a match that ran on: about M)", because blocks already on their way
    when the tab is hidden still arrive (3 in this slow container); the test allows 8. **Size: 3,964 added lines**
    without the fixture (client 1,790; sim-worker 1,203; client-endpoint 342; sim-wasm 253; sim 217; e2e 134;
    types 3; root manifests and `.gitignore` the rest), against about 2,900 planned and the stop line of 4,000.

33. **The size's stop line was reached and he was told in three lines** (00:17 UTC; 3,964 of 4,000). **His answer
    to SIZE-STOP: "A"** (2026-11-19T00:18:21.961Z; `cs2-first-page/mustafa-2026-11-19-size-stop.md`, by script):
    option (a), the stop line moves to 4,400 and the four remaining specs are written as planned.
34. **His message of 2026-11-19T00:23:22.833Z** (`cs2-first-page/mustafa-2026-11-19-until-7am.md`, by script): "You
    don't need to ask permission anymore until 7 am (I am going to sleep), I'll see in the morning what you've done
    Continue with your work as you are". Read as: no stop-and-ask until 7 am his time (06:00 UTC) for the work of
    this change-set. Not read as a go for what his standing rules keep for his explicit word: a merge (and no
    merge of visual work before his Brave check), a kit, gate or CI edit, any other file, version or checksum of
    code from the internet.
35. **Step 7, the rest: commit 74a0c24** (2026-11-19 00:27 UTC). `packages/e2e/tests/skeleton/`: `isolation.spec.ts` (4 cases),
    `allocation.spec.ts`, `clock.spec.ts`, `file-url.spec.ts`, `report.ts` (shared), `skeleton.spec.ts` (uses it).
    In the container's Chromium: 10 passed, four runs in a row. Facts: Playwright's clock does not reach the Worker
    (ticks 22, 83, 85: it ran on real time while the page's clock stood still); opened as a file the page does not
    run in Chromium ("Access to script at 'file:///...' from origin 'null' has been blocked by CORS policy") and the
    plain line shows; the hand-off's page code allocates 63 to 81 bytes per frame and 113 to 129 bytes per block
    (four runs), about 4,400 bytes a second. **Size: 4,126 added lines** (stop line 4,400).
36. **Readings in the container's Chromium** (00:18 to 00:19 UTC; `cs2-first-page/readings.mjs`; Chromium 153
    headless, WebGL 2 fallback, about 8.6 frames a second from software rendering): for block sizes of 8, 1,032,
    4,104, 16,392 and 65,544 bytes the Worker's copy is 0.000 ms at p50 and 0.100 ms at p99 (the Worker's clock
    steps by 0.1 ms here); delivery to the page 58 to 63 ms at p50 and 112 to 119 ms at p95, which is the page's
    own slow frame, not the block's size; tick start lateness 3.7 to 3.8 ms at p50, 6.7 to 9.2 ms at p99; at most
    1 tick per wake; 0 drops; 4 to 11 pool misses (the page returns buffers once per slow frame).

37. **The planted defects: every one caught, and the clean tree passes** (00:30:15 to 00:34:54 UTC by the run's own
    lines; `cs2-first-page/plants.py`, results in `cs2-first-page/planted-defects.md`; the throwaway worktree
    `.gate/cs2-plants` at 30a5a13 with its own packages and build folder, still there for the reviewers). Twelve
    plants (1, 2, 3, 4, 4.5, 5, 6, 7, 8, 8.5, 9, 10 of the plan's list), each failing its named check.
38. **The production check found the development code inside the production bundle** (00:31 UTC): the readout's and
    report's chunks and the test hook were built in, though the page did not show them, because the build's switch
    was read as `import.meta.env["..."]`, which Vite does not fold. **Fix, not yet committed** (`boot.ts`,
    `vite.config.mjs`): the two constants are written into the code by Vite's `define`
    (`__SHAMBLE_DEVTOOLS__`, `__SHAMBLE_COMMIT__`). After it: the production build has 5 files, no marker, no
    `serve.mjs`; the development build has each marker; the production page runs in Chromium with one Worker and no
    error (`cs2-first-page/production-check.mjs`, `production-check.txt`: PASS).
39. **Readings:** the page's type check with TypeScript 6.0.3 and `--ignoreConfig` (warm, 3 runs each, while three
    agents ran): 5.42, 5.16, 4.46 s with the lib check on; 1.31, 1.29, 1.36 s with `--skipLibCheck`. Two casts
    against `@types/three`, both in `three-picture.ts`.
40. **The first three reviewers finished** (their notes: `cs2-first-page/engine-architect-review1.md` 00:43 UTC,
    `graphics-engineer-review1.md` 00:51, `rust-reviewer-review1.md` 00:53; each "findings to fix before the
    merge"). engine-architect: F1 Medium (`boot.ts` imports `render/three-picture.ts` around the interface file:
    one named entry `render/index.ts`), F2 Medium (after a page-side failure the Worker runs on and the page counts
    one error per block: note once and stop the Worker), Lows F3 to F14. graphics-engineer: F1 Medium (a lost
    device or an uncaptured WebGPU error does not reach "errors": route three's error console into the page's
    hook before `init`), Lows F2 to F7 (add `programs` to the counts; read the fallback's renderer string; say
    forced or fallen back; a failure line when no backend starts). rust-reviewer: F3 High by the scale (his
    WB-HEALTH words in `sim-wasm/src/lib.rs` lost a ")": quote them exactly), F1 Medium (`build.mjs` reads the
    Wasm from `target/` whatever `CARGO_TARGET_DIR` says: pass `--target-dir`), Lows F2 (`mode: u8` cuts 256 to
    0: take `u32`), F4 to F7, three test additions. It also showed equal results natively and in wasm32 for the
    world's hashes and the schedule's due times.
41. **The night was lost to an unanswered approval prompt.** At about 00:33 UTC a command of the main session (one
    try of WebGPU on a software adapter: the test browser started with `--enable-unsafe-webgpu` and SwiftShader
    flags; an optional reading of the plan) asked for an approval. He had gone to sleep ten minutes before, having
    written that no permission was needed until 7 am; a prompt of the harness is not something his words can
    answer. The session sat until the worker restarted (seen at 07:13 UTC). Nothing was lost: HEAD equalled the
    pushed branch (the saver's autosaves 45537a3, d019d82, 41d4dd9 hold the reviewers' notes), the two uncommitted
    files were intact. **The try was not made and is not made again without him at the keyboard.** Lesson: no
    command with unusual browser or system flags in an unattended stretch; such readings go last, or to him.

42. **The production fix and the first half of round 1's fix pass: commit 29378dd** (2026-11-19 07:23 UTC); 19 files. Fixed:
    rust-reviewer F3 (his words exact), F1 (`--target-dir`), F2 (`mode: u32`), F6 (cargo settings from the
    environment refused), F7 (lookups return `Option`), its three test additions; engine-architect F1
    (`render/index.ts`, `CreatePicture`), F2 (a failure counted once, the Worker terminated, said on the page), F3,
    F4 (the view time leaves the input message), F5 (tick offset from the generated layout; the cap asserted
    against `tick_hz`), F7, F11, F14; graphics-engineer F1 (`THREE.setConsoleFunction` sends three's errors to
    the page's error event), F2 (`programs`, 6 on the fallback), F3 (the fallback's adapter name), F4 (the readout
    says why a fallback runs), F5, F6. **Left as follow-ups (Low):** rust-reviewer F4 (the DECISIONS pointer lands
    with the docs pass), F5 (the builder's registry path inside the Wasm: for the CI pull request and change-set
    3's size table); engine-architect F6 (record: the block has a fixed size for a Sim's life; ADR 0002 part 5
    pointer), F8 (`isWorkerToPage`), F9 (the endpoint's state belongs in `client-endpoint` from change-set 3), F10
    (`newest_input` goes at change-set 3; three unused exports), F12 (decide and record: the Worker's counters ship
    in production as monitoring candidates), F13 (nits); graphics-engineer F7 (MSAA memory, for change-set 6), its
    text on the software renderer and lines for the report. Checks all exit 0: fmt; both clippy runs; workspace
    tests (client-endpoint 15, sim 9, sim-wasm 6); tsc on both packages; unit tests (sim-worker 30, client 44);
    boundary 7; browser tests 10 passed, twice; the production check PASS. The Wasm is 28,987 bytes now (11,655
    gzip -9, 9,955 Brotli 11). The readout names the adapter: here "ANGLE (Google, Vulkan 1.3.0 (SwiftShader
    Device (Subzero) ...), SwiftShader driver)", so software rendering is confirmed. **Size: 4,257 added lines**
    (stop line 4,400: 143 lines of room for the other four reviewers' findings).

43. **The other three reviewers of round 1** (on 30a5a13 in the worktree `.gate/cs2-plants`; each "findings to fix
    before the merge"): security-auditor (`cs2-first-page/security-auditor-review1.md`: Mediums S1 a broken-off
    download left its file open, S3 blocks piled up after a failure, S5 `--out-dir` could empty any folder; Lows S2,
    S4, S6, S7 and two more; "The small server lets no file out of its folder and answers no other site. The tool
    route holds: F1 to F5 are closed as built."); perf-auditor (`perf-auditor-review1.md`: Mediums P1 fractional
    timer sleeps made about half the ticks start about 4 ms late, A1 the allocation test goes blind once the code
    is optimised; Lows L1 to L5); test-engineer (`test-engineer-review1.md`: Highs F1 the memory-growth proof did
    not cover the host, F2 the hidden-tab test was flaky and judged the wrong count; Mediums F3 (= P1), F4 plants 1
    and 8 were caught by a timeout and not by their check, F5 the message event outside the allocation sums, F6 a
    post per frame not proven, F7 two build claims with no committed check; Lows).
44. **Round 1's fix pass, second half: commit d257bb6** (2026-11-19 07:56 UTC); 16 files. Fixed: S1, S2, S4, S5, S6, S7, the
    comment, the tool's variables refused; S3 by the first half; P1 (whole-millisecond sleeps, rounded up, in
    `host.ts`; a wake-count guard in the browser test), A1 (floors; the spec says what it cannot do), L1, L5; F1
    (the host's memory grown half-way on the real Wasm; a copy before the growth), F2 (the Worker's own count), F5
    (`WorkerLink.receive`), F6, F7 (`bundleProblems` in the build), the file case's visible notice. **Left as
    follow-ups:** perf L2 (a pool of 9 when blocks grow), L3 (the readings' polling: fixed in the notes' script,
    not in code), L4 (the frame callback's own work time, for change-set 6); test-engineer's Lows (four server
    branches without a case of their own, `skipIf` for links, more `WebAssembly` names in the stubs, one Worker
    only, the scratch-buffer test); the security follow-ups (the `cargo tree` list as a test, `dist-*/` in
    `.gitignore`, the env restored in the isolation spec). Checks all exit 0: unit tests (sim-worker 30, client
    48), boundary 7, tsc on both packages, both builds, browser tests 10 passed three runs in a row. Allocation
    now reads about 65 bytes per frame and about 300 per block (the message event counted).
    **Size: 4,430 added lines, 30 over the stop line of 4,400.**

45. **Stopped adding code at the stop line and told him in three lines** (07:57 UTC). Question SIZE-STOP-2 is with
    him: (a) the stop line moves to 4,700; (b) it stays at 4,400 and three test additions (the build's own check of
    a production build, the wider allocation attribution, the post-per-frame check; about 70 lines) are taken back
    out and carried to change-set 3. No code is added until he answers.
46. **The seventh reviewer, netcode-architect** (`cs2-first-page/netcode-architect-review1.md`, on 29378dd): "no finding
    above Low"; seven Lows, all for change-set 3's plan unless cheap: (1) who picks the blend fraction between two
    blocks (one sentence in `picture.ts` now, a field at change-set 4; events must be read from every block, not
    only the two kept); (2) the pause on a hidden tab must come from the mode once online lands, and held buttons
    are cleared on hide; (3) the seed as an authority-only section written last, so the shared state is a prefix;
    (4) the step gets the host's time (`step(nowMs)`); (5) the page's per-frame number is not the protocol's
    command sequence: rename it the post number; (6) the Pacer should throw on a due time that is not finite; (7)
    a start tick in the authority's match setup. Its answers: the checked tick is right for the authority; a plain
    minus on decoded ticks would be a panic from network input, so `checked_sub` or wrap-aware compares in
    change-set 3; two kept blocks is the right number; nothing needs tearing out beyond what is recorded.
    **Round 1 is complete: seven reviewers.**

47. **The planted defects, second version: every one caught at the line that names it** (`cs2-first-page/plants.py`,
    results `planted-defects-2.md`; in the worktree `.gate/cs2-plants` at 9f28a87, 08:03:59 to 08:09:14 UTC): 19
    plants (1, 2, 3, 3b, 4, 4b, 5, 5b, 6, 7, 7b, 8, 8b, 9, 9b, 10, 11, 12, 13), each "FAILED as it must, at the line
    that names the defect"; the control (the clean tree: development build, cargo tests, boundary 7, client 48,
    browser 10 passed, production build) passes. Plants 1 and 8, which a timeout had caught in the first run, now
    fail at their own assertions (skeleton.spec lines 59 and 83).
48. **The readings again after the timer fix** (`cs2-first-page/readings.mjs`, now asked every 500 ms and read after
    1,200 blocks; `readings-2.txt`; 08:09:30 to 08:12:59 UTC, load 2.6 before and 3.1 after, three reviewers
    running beside it; the development build of 07:50 UTC, whose content is d257bb6's): the tick's lateness p50
    0.77 to 0.93 ms (before the fix: 3.7), p99 5.6 to 10.8 ms, max 8.3 to 43.9 ms; the copy p50 0.000, p99 0.1 to
    0.2 ms at every block size from 8 bytes to 65,544; no drops; at most 2 ticks in one wake; the picture on the
    software renderer at 3.3 to 5.0 frames a second (frame p50 200 to 300 ms), so the delivery times (p50 96 to
    166 ms) are the page's frame gaps, not the hand-off. Reported, never judged.
49. **CI on the pushed head** read at 08:08 UTC with the GitHub tools: `toolchain` run 213 and `gate` run 915 on
    9f28a87 (which holds d257bb6): success; on a98de9a (which holds 29378dd): `toolchain` 212 and `gate` 914:
    success.

## Current step
50. **Round 2, on what the fixes changed** (`git diff 30a5a13..d257bb6`): security-auditor, perf-auditor and
    rust-reviewer are reading (notes `*-review2.md`); then engine-architect, graphics-engineer and test-engineer
    (test-engineer after the planted defects' second run, which it must read). **The planted defects, second
    version** (`cs2-first-page/plants.py`: 19 plants, each must fail at the assertion that names it; results in
    `planted-defects-2.md`), run in the worktree `.gate/cs2-plants` at 9f28a87 since about the last save. Then the
    readings again (the timer fix), the verifier, the docs pass, the zip, the advisor agent on the final head, the
    report, the pull request; stop for his go and his Brave check.

## Next steps
1. See "Current step". No reviewer runs the browser project or `build.mjs` in the main folder while the main session
   builds there (the small server's port and the build's output folders are shared).
2. For change-set 3's plan (his words): the rand upgrade guide for 0.10 and why `chacha20` 0.10.0 and 0.10.1 were
   withdrawn. For the report's follow-ups: the npm scope `@shamble`; the favicon 404; the overflow readings; the
   `.d.ts` reading; the software renderer and lines that cross behind the camera.
3. Read CI on the head this save pushes. The saver: restart it within 110 minutes of the time under "Running work".

## Due in this change-set's docs pass (records of #60's merge; nothing of it is written yet)
- `docs/DECISIONS.md`, by script: his message of 2026-11-18, 20:20:09 UTC (his go on #60; what he accepts;
  WRITE-MODE (a); REC-OLD (a) with one limit; what comes after). The script of the last pass is in git:
  `git show f167f061a9b0b694836c272e397c90c815d7829f:.checkpoints/kit-hold-ci/decisions.mjs`.
- STATUS, ROADMAP, the archive: #60 merged (b763ba3, 2026-11-18 20:30:46 UTC, his go); its open findings move from
  its row to "Follow-ups" (their text is in the archive already).
- The map: the three rows for `.claude/CHECKPOINTS.md`, `.claude/scripts/checkpoint.mjs` and
  `.github/workflows/toolchain.yml` are corrected (his REC-OLD words); the report lists each with the old and the
  new text.
- docs-keeper's scripts for moved lines are in git at the same commit (`docs-keeper-move.mjs`,
  `docs-keeper-blame-check.mjs`).

## Open findings (carried; none blocks the start)
- From #60, follow-ups by his word, "fixed with the kit changes of the quality gate step": security-auditor's two
  Mediums (in two corners the hold checks another path than the one it pushes; no rule makes a review read the
  release record) and test-engineer's three (behaviours no test pins). Their text: `docs/STATUS_ARCHIVE.md`,
  "Follow-ups", "From the kit hold and two CI edits (PR #60)". His list for the quality gate v2's plan stands there
  too.
- From #58: round 2's Lows; the doc lines in `crates/protocol/src/ts_export.rs`,
  `crates/client-endpoint/src/layout_ts.rs` and `crates/protocol/src/messages.rs` that claim more than the code
  does, "corrected in the next pull request that touches those files" (change-set 2 may be that one).
- Two wording slips stay in #60's report, named in that pull request's description (accepted with his go).
- (The test zip of change-set 1: answered, step 5. The file route for a zip works; change-set 2's Brave check
  can rest on it.)

## Key facts
- Here: Node 22.23.4, git 2.43.0, cargo 1.99.1. The smoke check cannot start in this container (the installed
  Chromium is build 1243, the pinned Playwright wants 1194): `node tools/gate.mjs --no-smoke` here, CI's `smoke` job
  for the rest. Other local checks: `node .claude/scripts/repo-map-check.mjs`; the kit's tests
  (`node --test .claude/scripts/checkpoint.test.mjs`: 33).
- Scratch left under `.gate/` (ignored by git): `kit-hold-ci/` (scratch copies and broken copies of the checkpoint
  script), `cs1-types/archive/` (the test zip), two build folders of 136 MB each (`plant-check-target`,
  `test-engineer-plant-target`; the classifier refused their removal).

## Running work
- saver: running since 2026-11-19T07:13:40.814Z (its own first line; task b9vzms3oh; 10-minute interval, stops after 110 minutes).
- agents: see "Current step".
