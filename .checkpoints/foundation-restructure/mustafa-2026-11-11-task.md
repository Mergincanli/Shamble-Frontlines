# His task message for foundation step 2.1b, 2026-11-11

Source: his chat message in session_019YNMUWdGJZoQFUuuvvBP1Y (his personal account, foundation step 2.1b), event e018683d-30d9-49a0-b19d-ab279e7d23a1, at
2026-11-11T20:21:02.544284Z, extracted by `capture-message.mjs` (run from `.gate/foundation-restructure/`; a copy beside this file) from
the session's own events (`list_events`), never retyped.

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
