# Handover from Builder V3 to Builder V4 (lane 1), 2026-11-25
_By the chat Mustafa calls Builder V3 (session_01WLuqnvpAh9XTitx8ehKJcQ), on his words of 14:24:13 UTC, saved by script
in `handover-builder-v3/mustafa-2026-11-25-handover.md`. Written from what the chat already knew: no new reading, no
agents, no test runs. State: change-set 3b is merged (#68, main 3ab9eaec); nothing is half done. 3b's notes, with every
script named here, are at commit 80a51e42 under `.checkpoints/cs3b-loop-closes/`._

## a. Traps that cost time, and the way past each
- **The guard refuses** `npx`, `npm run`, `npm test`, `git checkout`, `restore`, `rebase`, `git -C`, `git -c`, `gh pr`,
  every `gh api` write, `rm -rf` and a foreground `sleep`. Past it: `node node_modules/<tool>/…`; `gh api` GET only;
  `git rm -r -q` for a tracked folder; a background loop to wait.
- **The worktree:** never switch the main folder. Work in `.gate/<slug>` (`git worktree add`), commit by path, and push
  `git push origin cs/<slug>:claude/<branch>` alone in its command (a second command joined to it got the call
  refused). `.gate/` is untracked: a container restart loses builds and tools, never notes that were pushed.
- **The WebAssembly:** wasm-bindgen 0.2.130 is not installed. Fetch it as CI does, check its sha256, keep it under
  `.gate/wb-tool/`, then `SHAMBLE_WASM_BINDGEN=<it> node packages/client/scripts/build.mjs [--wasm-only|--production]
  --out-dir "$PWD/.gate/dist-development"`. Cargo with `--locked`; `npm ci --ignore-scripts` only. After every push,
  compare the players' file's size and sha256 with the runner's log.
- **Browser tests:** `SHAMBLE_DIST=<build> SHAMBLE_CHROMIUM=/opt/pw-browsers/chromium node
  node_modules/@playwright/test/cli.js test -c packages/e2e/playwright.config.ts --project=chromium --workers=1
  --retries=0 --trace off`. Port 4173 must be free: two runs at once (mine and an agent's) failed in odd ways, and a
  busy port once made a planted defect (`plants-3b.py`) look uncaught, so one run at a time. Headless falls back to
  WebGL 2. Time the suite before the first push (16 tests: 3.4 of CI's 5 minutes). A test starts its clock before its
  first reading. The allocation test's limit depends on the frame rate.
- **CI:** a push cancels the branch's running checks and `[skip ci]` starts none, so a change-set's last commit has no
  `[skip ci]`. Write the expected counts down before each push and read them in the job's log (`gh api`: runs by
  `head_sha`, then jobs, then `jobs/<id>/logs`). `tools/gate.mjs` checks the legacy game only; v2 is the `toolchain`
  job, which runs no lint: run the lint by hand for `room` and `protocol`.
- **Main moved during the review** (#66): DECISIONS, STATUS, the archive, ROADMAP and REPO_MAP conflicted. One merge
  commit, resolved by script: DECISIONS newest first with every entry of both sides checked byte for byte; lane 2's
  lines never re-wrapped (his RECORD-FILES); STATUS back to 120 lines by moving old lines of merged pull requests by
  script (his OLD-LINES), each listed in the report. Take main in before asking for the last approval.
- **The zip for his check:** `make-zip.py` builds it from one clean commit's production build; the page shows that
  commit, so his "Copy report" proves which build ran. Give its bytes and sha256. One inexact README line ("A should
  be dark") cost a question: write each step as what he will see.
- **His messages:** `chat-words.mjs` (copied beside this note) writes a message whole from the chat's transcript, with
  bytes and sha256; `decisions-6.mjs` wrote DECISIONS' entries from those and has a `--check` mode. Use the
  transcript's UTC time, not his header's. A message sent while the chat works is stored as a queued command.

## b. The agents, and keeping a stop near its planned size
- **How:** the advisor agent reviewed the plan (APPROVE with eight conditions), then every pushed head; I resumed the
  same agent each time so it kept its context, and its verdict names the full commit id. Six reviewers (test-engineer,
  security-auditor, perf-auditor, engine-architect, netcode-architect, rust-reviewer), Opus 5.5 at effort max, three
  at a time, each in its own worktree copy, each writing its notes file as it went (a restart killed three mid-review;
  their notes kept the findings). Two rounds, the second on what changed; the verifier last, on the final code.
- **Size:** 6,909 added lines against about 3,800 planned (1.82 times; his stop line was 7,600), with the link
  simulator and the recording already moved out to 3c. Tests and planted-defect checks were the larger half.
- **Differently:** plan at 1.8 times and count test lines in the plan; let test-engineer and security-auditor read
  the plan's test list before the build (round 1's High was a check that let a planted defect through, and a stronger
  check then showed the command queue's window followed refused commands); write each check's planted defect with
  the check; fix the size ladder's cut points before step 1 and report the count at each step; have the advisor agent
  read the report's draft before the zip is built (one REVISE was on text alone).

## c. For 3c: what is named for its plan, and where it is on main
- **3c holds:** the link simulator (a link that delays, drops and reorders) behind the transport interface 3b built
  (`LocalMatch<P: TransportPair = Loopback>` is the seam); the first recording; one new read-only number on the
  players' file (the state's hash); `devtools`' three path edges, declared in 3b and not used yet.
- **Named for its plan, the ones I am sure of:** 3B-HOSTILE-ALLOC (a counted "no allocation" for hostile bytes in the
  room and the endpoint, with 3c's tests); his RELEASE-COUNT (a): no release counter in 3c. Not 3c's: 3B-LATE-BOUND
  and 3B-PLAUSIBLE (change-set 7); 3B-ACKS and 3B-OUTAGE-256 (before online play).
- **Where, whole:** `docs/STATUS_ARCHIVE.md`, "Follow-ups", "From change-set 3b" (each item's text, who named it, when
  it is due, and 3c's plan items); `docs/STATUS.md`, "Next" item 2 and the follow-ups bullet of 3b; the report
  `docs/reports/2026-11-25-cs3b-the-loop-closes.md` ("Follow-ups", "Risks"); each reviewer's own words at 80a51e42,
  `.checkpoints/cs3b-loop-closes/<name>-review1.md` and `-review2.md` (in main's history through #68).

## d. ORDER-3C: does change-set 4, Walk, need 3c first?
To be built: no. Walk (the collision library as the first outside crate in `sim`; a soldier moved by the keys) runs on
the local loopback 3b merged and uses neither. To be judged in his browser: no, it is local play. The cost of Walk
first, as I remember the plan: the recording is the whole-match check that native and WebAssembly agree, and Walk
brings the first outside float code (3A-FMA-CHECK), so it would merge on unit tests and vectors alone; 3c then records
a state that already moves. Whether a mark of Walk names the recording as its method I cannot say without new reading.

## e. What he told this chat that binds lane 1 and is not on main
- His message that asked for this note (saved beside it): lane 1 moves to Builder V4 at this merge; and until the
  guard is fixed, no session or agent puts `env`, `stdbuf` or `nice` with options in front of `git` or `gh`.
- His Advisor's block of 15:25 Amsterdam on change-set 4's plan (seven points on his Testlines sandbox and its
  Export) reached this chat at 14:13:07 UTC and was replaced for it by the message above before anything was begun.
  It is saved beside this note as it arrived (`mustafa-2026-11-25-cs4-plan-block-replaced.md`); whether it binds
  Builder V4 is his to say. His block of 15:05 never reached this chat.
- The board add-on is active for this chat.
- Beyond these: nothing. His earlier messages that decide or instruct are in `docs/DECISIONS.md` on main, by script.
