# Mustafa's task message, byte for byte (copied by script from this session's transcript)
- Session a7e7f150-3195-52bd-a549-73b27426198b (claude.ai session_019YNMUSYpJGi1xxCjKgr4e5), user message 7dceb92c-e718-42e9-bfe7-7cec24c10671, stamped 2026-11-15T15:33:37.428Z; 5830 bytes; sha256 e86c6b53e9f6769a47a7024a01a127042681fd6b0b101d98cb55ab53a86b4009.
- Everything between the two marker lines is his text, unchanged.

-----BEGIN HIS TEXT-----
My words of 2026-11-14, for a fresh chat on my personal account: the second builder chat for the game.

Where things stand. Main is 4460ef5b8d6dc8566b29c15ded75c6f0c8be386b, with gate, smoke and toolchain green on it. It holds the empty skeleton of the new game and no game code. The first builder chat (branch claude/wonderful-hamilton-jdmlnf, main model Fable 5.2) has my go on its plan for foundation steps 2.4 and 2.5 and Slice 1, move and shoot: .checkpoints/build-plan-slice-1/plan.md on that branch, 14 change-sets. Two more chats work on this repo: the netcode proof test (PR #40, branch claude/netcode-proof-test-plan-hcky5g, my university account) and step 2.3, the gate v2 and CI (branch claude/sweet-mayer-grsukw; its plan waits for my go). Never push to another chat's branch and never touch spikes/.

Model. Your main model is Opus 5.5. Check this session's model first and stop in one line if it is not Opus 5.5. Fable 5.2 advises through the advisor tool. My words of 2026-11-11 on it apply: consult it at the five moments, always before you save a step as done and never after, and record each consultation in the checkpoint in one line with its time. Tell me in one line which model answers the advisor tool, read from the session's records. Agents run on Opus 5.5; the agents that review, verify or design game and engine code run at effort max.

My decision of 2026-11-14 on two builder chats. The first builder chat's change-set 0 records it in docs/DECISIONS.md; open no docs pull request for it:
"Two chats build the game on my personal account. The first builder chat keeps Fable 5.2 as its main model. A second builder chat has Opus 5.5 as its main model, with Fable 5.2 answering its advisor tool, and it also writes game and engine code. This changes my words of 2026-11-12 ("On my personal account, Fable 5.2 is the main model of every chat that writes game or engine code, and of every builder agent that chat starts to write that code.") for the second builder chat only. Every file has one owner: the first builder chat's plan says which change-sets and folders belong to the second chat, and neither chat edits the other's files."

Your lane, in this order:
1. Now: the preview hosting check (docs/SPIKE_PLAN.md, entry 6; docs/adr/0006-infrastructure-operations.md, parts 1 to 3 and part 13). It is the small test of the hosting service, Cloudflare Pages, that must pass before every build can get its own web link. My answer of today to the first builder's question PREVIEW-START: it starts now, on the infra draft as proposed.
2. After it: foundation step 2.6, the previews, so that every pull request of both builder chats gets a link I can open in Brave and on my phone. Part 8 of the first builder chat's plan names what step 2.6 waits for.
3. The carry of my look decisions of 13 and 14 November into docs/LOOK_V2.md: a docs change-set. Its own block comes from me later, with a picture file; start nothing of it now.
4. Then the units that Part 15 of the first builder chat's plan, "The second lane", gives you. It is being written now. My proposal there: the comparisons that stay (its Part 3.2) and, from change-set 6 on, the picture track: 6 Look, 10 Body, 11 Feel, and M, the first Blender model. Start none of them before I tell you which are yours.

The task now: the plan for the preview hosting check. No build, no secret, and no edit outside your checkpoint. The plan gives me:
1. What the check proves and which pass marks it meets (PM-0006-1 to PM-0006-10 and PM-0006-32), in plain words.
2. What you build: the test page, the throwaway workflows, the scripts.
3. What I do myself, in steps one at a time: in my Cloudflare account (shamblefrontlines.com is registered there), on GitHub and in Brave. For each step: where I click, what I type, what I paste where. Which token I make, with exactly which permissions, where it is stored, who can read it, and when I revoke it.
4. The security side in plain words: what a pull request can and cannot reach, and what could go wrong with the token.
5. The calendar: what waits 24 hours, and the earliest day the check can end.
6. Which rows need the minimum laptop or a Mac, and whether they can wait for those machines without holding the rest. My machines: the recommended desktop is an RTX 2070 with an i7-9700K; the minimum laptop is of the Intel Iris Xe class or the AMD equivalent and is not named yet; no Mac yet.
7. Every CI or workflow file you need, with its exact text. No kit, gate or CI file is written before my yes to that text.
8. The decisions you need from me, each with its options and your recommendation, in plain words.

How to work. CLAUDE.md's change-set protocol. Your branch stands in for the cs/ branch; one pull request at a time. At most 3 agents, the saver running, a checkpoint after every step. Have the advisor agent review the plan. The plan gets no review rounds and no verifier; every change-set gets the full review when it is built. In a report, write about your own process and about what you told me only what a record shows, with its time. Write each unit's planned size in the plan; when one passes twice its plan while it is built, stop and tell me in three lines: what grew, why, and the cheaper option. Ask me to compact when the chat is nearly full, and do not stand still waiting for it. If the guard, the deny list or the classifier refuses a command, record it with the route you took. If a rule in the repo forbids something here and my words above do not cover it, the repo wins: stop and tell me in three lines.

What this block covers: the plan for the preview hosting check. It covers no build, no secret and no merge.

End with: the plan's path, a one-screen summary in plain words, my steps in order, the decisions you need from me, and how full this chat is. Then stop for my go.
-----END HIS TEXT-----
