## Appendix A. Mustafa's words for this change-set, in full

_Each block below is copied byte for byte from its source by `report-appendix.mjs` (in the notes folder, readable at this branch's last checkpoint commit), never retyped: his messages in this session (session_019YNMUWdGJZoQFUuuvvBP1Y, his personal account), each extracted by `capture-message.mjs` from the session's own events. The same script checks every block against its source. Entries in `docs/DECISIONS.md` copy their quotes from here, because the notes folder is deleted before the PR is marked ready._

### A1. His first message in this session, 2026-11-11 20:20:36 UTC (event 2f15e227-dcbb-43a7-b1fd-42179be83871: `mustafa-2026-11-11-hi.md`)

```text
hi, do not do anything yet
```

### A2. His task message: step 2.1b, the move, 2026-11-11 20:21:02 UTC (event e018683d-30d9-49a0-b19d-ab279e7d23a1: `mustafa-2026-11-11-task.md`)

```text
My words of 2026-11-11, for a fresh foundation chat on my personal account. Step 2.1b: the move.

Who you are. You are the foundation lane, step 2.1b. Step 2.1a is merged: main is 76b4f7a (PR #50). The old foundation chat is full and has stopped. Two other chats work on this repo: the netcode proof test (PR #40, branch claude/netcode-proof-test-plan-hcky5g, my university account) and the toolchain (PR #48, branch claude/laughing-pasteur-fwya24, my personal account). Never push to their branches and never touch spikes/.

Model. The main model and every agent stay Opus 5.5. Check this session's model first and stop in one line if it is not Opus 5.5. Fable 5.2 advises through the advisor tool. My words on it of 2026-11-11 are in docs/DECISIONS.md and apply here: consult it at the five moments, record each consultation in the checkpoint in one line, and count them in the report. If the advisor tool is not on, say so in one line and carry on.

Read first: CLAUDE.md, .claude/CHECKPOINTS.md, docs/STATUS.md, my words of 2026-11-10 and 2026-11-11 in docs/DECISIONS.md, and docs/reports/2026-11-11-foundation-2-1a.md. Then the old chat's notes at commit c8f4bb382d80d2248a3aef75f864753428bfac8b, under .checkpoints/foundation-restructure/: plan.md (revision 3) and what else a step needs. Copy plan.md into your own checkpoint folder as your step 0. Never redo finished work.

The task. Build 2.1b now, under plan revision 3, steps 6 to 11.
- The five legacy files move into legacy/ with git mv, bytes unchanged, in one pushed commit that holds nothing else.
- Then the commands in text, the evidence E1 to E6 and E8, and the docs.
- My acceptance of ADR 0002's part 1 goes into the ADR's Status line in your docs pass, as my SCOPE (a) of 2026-11-10 says.
- Take these follow-ups from 2.1a's report into your plan check: IE-2, IE-3, IE-4, IE-7 and SA-L1.
- Check the plan's line numbers against today's main before you use them: CLAUDE.md and the docs changed with PR #49 and PR #50. If the plan and today's main disagree on more than a line number, stop and tell me in three lines.

Kit and gate files.
- The plan names four kit lines: three in CLAUDE.md and one in .claude/agents/verifier.md. Ask me in chat with the exact changes, all four in one message, and write them only after my yes.
- The gate files are not touched in 2.1b. If you find that one must change, stop and tell me.
- A prompt from the guard counts as not seen by me only for an edit to a kit or gate file and for a move of the main folder. For every other question from the guard, my answer on the prompt is my answer.
- Do not edit any settings file, hook or allow-list.

How to work. Plan check first, then the advisor agent's review of it, then build. At most 3 agents at a time, the saver running, a checkpoint after every step. Agents cannot be asked, so they write with the file tools or with one plain command and a literal path. Reviews as the plan's 1.7 says, within the review budget. If the guard, the deny list or the classifier refuses a command, record it and list it in the report with the route you took.

Windows. The first run of the new gate on my laptop passed every check except the smoke, which lacked its browser. I am re-running it and will tell you the result.

Also in your docs pass, queue this note in my words in docs/DECISIONS.md under "Queued", and change no other file for it:
"A note for the render budget bench (ADR 0003), 2026-11-11. The stack stays as decided: Rust decides and computes, TypeScript on three.js draws. The bench reports three times per frame: the page's JavaScript, the Rust side, and the graphics card. If the page's JavaScript is the part over budget, that work moves to the Rust side before the JavaScript is tuned. Candidates I like for more Rust later: crowd and animation math for hundreds of soldiers, offline tools that prepare models, textures and maps, and heavy effects where they do not belong on the graphics card."

End with: the pull request's number and head SHA, what the proof showed, CI's gate and smoke on the final head with run numbers, how full this chat is, and what you need from me. For my part, give me exact steps in plain words: the saves backup with both console lines, the Brave check of legacy/index.html and legacy/editor.html, and the Windows command for your branch. Keep the pull request a draft until its report is in, then mark it ready and stop for my go.
```

### A3. His D1 message: D1 (a), his Windows run of the new gate, the Node 24 finding for 2.3, 2026-11-11 20:44:29 UTC (event ce7f1f4b-08bc-476f-862e-3c1d9362c40b: `mustafa-2026-11-11-d1.md`; sent again at 21:27:35 UTC, event 40ebaa1e-996c-42b2-b313-4198c6274017: `mustafa-2026-11-11-d1-resent.md`, byte-identical, checked by this script)

```text
My words of 2026-11-11, for the foundation chat (step 2.1b, branch claude/clever-fermi-oudsfq).

The advisor tool. Tell me in one line which model answers the advisor tool now. If it is not Fable 5.2, say so and carry on; I will decide.

D1: (a). Record OQ-0002-2's answer as your plan check's section 3 proposes: the register's ADR 0002 row gets the new open-question count and my SCOPE (a) in its status column, PM-0002-36's reading goes into the register's loose-ends tables as one row, and ADR 0002's two lines that say "open: OQ-0002-2" read "answered". No new register row, and nothing wider.

Windows. The new gate passed on my laptop tonight, on main 76b4f7a: Windows PowerShell 5.1, Node v24.17.0, base 44446a5, "files at the root" twice, every check PASS, the smoke 8 of 8 on index.html and 5 of 5 on editor.html, GATE PASS. Record it in the report.

One finding from that run, for 2.3: with Node v24.17.0 the pinned Playwright's installer hung after the browser download reached 100%. It is a known bug with Node 24.16.0 and later and Playwright before 1.60.0. The same install finished under Node 24.15.0. If your own browser install stops after 100%, do not wait on it: record it as this bug and tell me.

Then continue with your next steps: the advisor agent's review of the plan check, then the four kit lines and E8's ask to me in one message.
```

### A4. His message on auto mode, 2026-11-11 21:10:58 UTC (event 7a094d1e-41ed-40b7-91f8-544d91e4b64f: `mustafa-2026-11-11-permission.md`)

```text
I give you permission to go on auto mode and accept yourself for the bash commands and what not
```

### A5. His answers: KIT-2.1B (a), E8-RUN (a), REG-TRUE (b), and the build go for steps 6 and 7, 2026-11-11 21:39:44 UTC (event 45563b1b-718b-4683-9e4c-663daf9cff74: `mustafa-2026-11-11-answers.md`)

```text
My words of 2026-11-11, for the foundation chat (step 2.1b, branch claude/clever-fermi-oudsfq).

The advisor tool. Tell me in one line which model answers the advisor tool now. If it is not Fable 5.2, say so and carry on; I will decide.

KIT-2.1B: (a). Yes to exactly the four lines in your message, in one commit that holds nothing else. My Advisor compared the four "before" lines with main 76b4f7a: identical. My yes covers these four lines only. Any other kit or gate edit gets its own ask in chat.

E8-RUN: (a). Yes to the rollback rehearsal as you describe it: in the throwaway copy .gate/fr-e8 only, by the main session only, nothing pushed and no branch made, the copy removed afterwards. It also covers one re-run the same way on a later head of this pull request, if a fix changes a file the gate reads or a kit line. Nothing else.

REG-TRUE: (b). The register's own rule decides: OQ-0002-2 stays open as "answered in part", because the gate v2's path waits for step 2.3's plan. So 8 open, the Total line stays, and ADR 0002's lines 1596 and 1626 read "answered in part". The answer names all three parts. Add the two acceptance edits for my SCOPE (a). This replaces the "answered" wording and the new count in my D1 (a); the rest of D1 (a) stands.

Then build: step 6, the move, and step 7.
```
