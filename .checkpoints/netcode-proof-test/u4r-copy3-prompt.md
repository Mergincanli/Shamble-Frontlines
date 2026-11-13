You are U4r's builder for the netcode proof test (PR #40, the spike branch claude/netcode-proof-test-plan-hcky5g): one builder, alone, in your own isolated worktree. Your instructions are the brief. Read it whole before you build: `.checkpoints/netcode-proof-test/u4r-brief.md` in your worktree (rev 3.2 as filled at launch: 1,909 lines, sha256 59d570b8a73fa6d9ab6b9a02e9f6d6cf6ec37a57d36d2c3ef3285bc8c5196046). Check its sha256 first and stop if it differs. Follow its section 1 (Start) in order, then sections 2 to 6. What follows is only what the brief leaves to this prompt; where they seem to differ, the brief wins and you say so in your notes.

**Your worktree's start (the brief's section 1 item 1).** `<base>` is 614823c3e69f93c5e2c3d9d57f842c15368710b6 (P6). Your worktree starts at the spike branch's head at launch, 2fbec7780cd04feafd8a3c67e1ebdb8889ad45ef (or a later checkpoint commit of that branch). That head is later than `<base>` by .checkpoints/ commits only: 2fbec77 is the brief's fill. So item 1's `git merge --ff-only <base>` changes nothing, and `git log -1 --format=%H` shows that head, not `<base>`. That is expected, as U4d2's builder recorded (u4d2-build.md, Block 0). Before anything else, record in your notes:
- the worktree's path and branch;
- `git log -1 --format=%H`;
- that `git diff --quiet 614823c HEAD -- spikes/` exits 0;
- that `git merge-base --is-ancestor 614823c HEAD` and `git merge-base --is-ancestor 4e91830 HEAD` both exit 0;
- `df -h /home/user`.
If any of these does not hold, stop and report.

**The runner** (the brief's section 1 item 4, P7): /home/user/Shamble-Frontlines/.checkpoints/netcode-proof-test/u4r-run.sh, sha256 349b51adbed43a889ba9b88eeb89d573aadc3b4f035e186cc8b56de18f4e0831. Check it before its first use. Every cargo, rustc, rustdoc and check.mjs call goes through it, under section 4's shared-target rules.

**Order:** D13's five short runs (section 11) come before any `scenario --record`, as section 7.1 says (:1175).

**Your limits.**
- T4 item 1, word for word: "Write files with the Write and Edit tools only (no sed, no shell redirect, no python writes; never `cargo fmt` writing files: apply rustfmt's diff by Edit); an agent with no Edit tool rewrites its notes file whole with Write."
- A syntax check that writes nothing: `python3 -I -c "import ast,sys; ast.parse(open(sys.argv[1]).read())" <file>`. Never py_compile: it writes a .pyc.
- Git runs only in your worktree, as `cd <worktree> && git ...`. Never use `git -C`, `git -c`, `git checkout` or `git restore`; the guard refuses them. Commit nothing and push nothing; the main session merges.
- Start no agents and no workflows. The session runs at most three agents, and you are counted.
- Never read .env or secrets. Never edit .claude/, any CLAUDE.md, .github/ or tools/.

**Cuts.** The session's spend limit may cut you mid-run. Save as you go (section 1 item 3). The main session then resumes a fresh copy of you. That copy reads your notes and `git diff` in the worktree first, then continues after the last block marked done.

**Your report** is section 6's return list, in its order. Your final message is that report; it goes to the main session.

**Resume: copy 3 (the main session, 2026-11-13 about 22:00 UTC).** You are a fresh copy. Copy 1 (19:31-20:39 UTC) and copy 2 (20:55-21:51 UTC) each ended at its context limit; nothing is committed. Where this part and the text above differ, this part wins for you (the brief still wins over both).
- **Your worktree already exists:** /home/user/Shamble-Frontlines/.claude/worktrees/agent-a11e2354169f39acf (branch worktree-agent-a11e2354169f39acf, HEAD 2fbec7780cd04feafd8a3c67e1ebdb8889ad45ef, both copies' work uncommitted in it). The main session's snapshot of that work: /home/user/Shamble-Frontlines/.checkpoints/netcode-proof-test/wip/u4r.patch (17 files, sha256 48fa2959799d7f0a6f65f6b2524e3d283a8b836474b6d27e96198c0ae69c7b53). Your session's folder is the main folder, not the worktree: begin every Bash command with `cd /home/user/Shamble-Frontlines/.claude/worktrees/agent-a11e2354169f39acf && `, and give the Read, Edit and Write tools absolute paths inside the worktree. The runner and the logs folder /home/user/Shamble-Frontlines/.gate/u4r/ stay where they are; every runner call names the worktree (copy 2 once left it out). Never edit a file outside the worktree, and never edit anything in the main folder.
- **The start checks above were the first copy's** (its notes, Block 0). In their place, first: check that HEAD is 2fbec77…, the brief's sha256 59d570b8…, the runner's 349b51ad…, and run `df -h /home/user`; write them under a new heading "## Copy 3" at the end of your notes (the worktree's .checkpoints/netcode-proof-test/u4r-build.md). If any does not hold, stop and report.
- **Before you change anything** (Mustafa's rule: a fresh copy reads its notes and the diff in its worktree before it changes anything): read your notes whole (569 lines), then the diff: `git status --short`, `git diff`, and each untracked file under spikes/. Read again the brief's sections your next blocks need (at least 2.3, 2.4's r2 and r10, 4, 5, 6 and 7.1) and the merged code they name.
- **The advisor's rulings** (the main folder's .checkpoints/netcode-proof-test/advisor-u4r-build1-full.md, sha256 476121ab65e65e725001b149dd37a41aa5f189bb8c3c88cdfe8dce7bc6e30752): each point's "**Builder:**" line binds you, word for word; your notes' block "Rulings (advisor-u4r-build1)" gives each point's state.
- **Continue** from the notes' last section, "Copy 2: state and what is next": its items (4) to (7) in order: r2 with the real judge by hand, and point 3's per-profile bytes; point 1 (4) to (7) with r10's check.mjs half; B12's run scripts and SERIES.md rows; r5 and section 5's end checks twice.
- **Held for the advisor** (its ruling comes to you by message): copy 2's two pre-check findings, the header's `buffer_depth_ticks_count` (L) against the room's measured L - 1, and that the pre-check writes no per-leg log. Change neither precheck.rs's outputs nor its depth reading until a ruling reaches you. If you reach B12's pre-check script with no ruling, build the other scripts first.
- **Your context is finite:** each earlier copy used all of its own in about an hour. Save each finished block to the notes at once. When most of it is used, stop at a clean point (no plant in place, no half edit, no run going), write the state and next steps as the earlier copies did, and report.
- **Your report:** if you finish the build, section 6's return list, in its order, covering all copies' work; if you stop partway, what this copy did and where the notes' state section is.
