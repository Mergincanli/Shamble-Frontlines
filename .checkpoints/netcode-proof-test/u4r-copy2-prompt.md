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

**Resume: copy 2 (the main session, 2026-11-13 about 20:50 UTC).** You are that fresh copy. The first copy (agent a11e2354169f39acf, 19:31-20:39 UTC) ended at its context limit, about half built; nothing is committed. Where this part and the text above differ, this part wins for you (the brief still wins over both).
- **Your worktree already exists:** /home/user/Shamble-Frontlines/.claude/worktrees/agent-a11e2354169f39acf (branch worktree-agent-a11e2354169f39acf, HEAD 2fbec7780cd04feafd8a3c67e1ebdb8889ad45ef, the first copy's work uncommitted in it). The main session's snapshot of that work: /home/user/Shamble-Frontlines/.checkpoints/netcode-proof-test/wip/u4r.patch (11 files, sha256 1e2fa56a09b6dc2e921e54a84a26bc81575b77d621d067413b24e177c4726380). Your session's folder is the main folder, not the worktree: begin every Bash command with `cd /home/user/Shamble-Frontlines/.claude/worktrees/agent-a11e2354169f39acf && `, and give the Read, Edit and Write tools absolute paths inside the worktree. The runner and the logs folder /home/user/Shamble-Frontlines/.gate/u4r/ stay where they are. Never edit a file outside the worktree, and never edit anything in the main folder.
- **The start checks above were the first copy's** (its notes, Block 0). In their place, first: check that HEAD is 2fbec77…, the brief's sha256 59d570b8…, the runner's 349b51ad…, and run `df -h /home/user`; write them under a new heading "## Copy 2" at the end of your notes (the worktree's .checkpoints/netcode-proof-test/u4r-build.md). If any does not hold, stop and report.
- **Before you change anything** (Mustafa's rule: a fresh copy reads its notes and the diff in its worktree before it changes anything): read your notes whole (282 lines), then the diff: `git status --short`, `git diff`, and each untracked file under spikes/. The first copy read the brief whole; read again the sections your next blocks need (at least 1, 2, 4, 5, 6 and 7) and the merged code they name.
- **Continue** from the notes' last section, "State at this copy's end, and what is next": items 1 to 4 in order (fmt; B9 `replay` and r3; B10 the -30 pre-check with r6 and r7; r2 and the real judge by hand).
- **Held for the advisor** (the main folder's .checkpoints/netcode-proof-test/u4r-build1-questions.md; the rulings come to you by message): the determinism route (the runner entries in recordings.json, hash.rs `legs`, where carry 84's bound lives), F1 (mirror.rs's radius), F3 (the -57 series' size), F4 (carry 62 (h)'s bite) and F5 (the `span_ms` copy). Change none of them until a ruling reaches you. Items 5 to 7 (r10's check.mjs half, B12, r5 and the end checks) wait for the rulings; if you reach item 5 with none, write your state and report.
- **Your context is finite:** the first copy used all of its own in about 70 minutes. Save each finished block to the notes at once. When most of it is used, stop at a clean point (no plant in place, no half edit, no run going), write the state and next steps as the first copy did, and report.
- **Your report:** if you finish the build, section 6's return list, in its order, covering both copies' work; if you stop partway, what this copy did and where the notes' state section is, as the first copy did.
