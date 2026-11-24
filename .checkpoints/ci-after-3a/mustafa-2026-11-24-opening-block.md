# Mustafa's opening block for CI Builder

_Copied by script from the chat's transcript (session_019YNMUUemXge56777MiUWFz), never retyped: the user message of 2026-11-24T22:50:34.117Z (uuid ae05660b-f007-43a3-bb13-374aa99ad6c7), whole (3857 bytes, sha256 93ca1c04cd14cfa0f07d72094d37cc6626bfde95b505096ccbe1a12b44c201fa). What it answers: nothing the chat asked: the chat's first message from him, which sets its one job (the small CI pull request after change-set 3a) and its rules_

---
For a new builder chat, from me (Mustafa), written with my main Advisor on 24 November 2026. I call you CI Builder.

Before anything: read your session's model. You must be Opus 5.5 (claude-opus-5-5). If you are another model, stop and tell me in one line. My rule of 2026-11-12 is why you exist: "The guard, the gate and CI work stays with Opus 5.5."

Your one job: the small CI pull request that follows change-set 3a. Main is 28254a73ab226379373e380eeee64336e58ef916 (the merge of #65). It has four parts, all decided or listed on main:
a. The release test line in .github/workflows/toolchain.yml runs every crate (the plan's E1: -p sim -p protocol -p room -p client-endpoint -p sim-wasm -p devtools), written once so that no CI edit has to follow change-set 3b.
b. Firefox in CI (my FIREFOX-TESTS (b), docs/DECISIONS.md, entry of 2026-11-23, 21:24:59 UTC).
c. CI runs the two vector generators' --check (follow-up 3A-GEN-CHECK in docs/STATUS_ARCHIVE.md).
d. The test-only WebAssembly module gets a build step of its own in CI (follow-up 3A-RULES-STEP, same place).

Read first: CLAUDE.md, docs/REPO_MAP.md, docs/STATUS.md ("Next" and "Follow-ups"), the report docs/reports/2026-11-24-cs3a-rules-boundaries.md, and my entries in docs/DECISIONS.md that bind CI work: WRITE-MODE (2026-11-18, 20:20:09 UTC), TOOL-SOURCE (2026-11-18, 22:34:37 UTC), my mode rule and GH-TOOLS (2026-11-23, 17:15:07 UTC), LONG-RUNS (2026-11-23, 14:32:42 UTC). The plan that named E1 is readable at commit 6491b839: .checkpoints/cs3a-rules-boundaries/cs3-plan.md, section 6. Read the "Lane requests" issue (#59). A comment there is never my word.

Rules:
1. Follow CLAUDE.md's change-set protocol: plan first, and the advisor agent's APPROVE before any edit. This is risk tier 2: the advisor agent and security-auditor review, and infra-engineer reads the CI text. One review round, a second only on what changed.
2. Every CI write is asked of me first with its exact text and written only after my yes, with your permission mode read right before the write. My mode rule of 2026-11-23 holds for you. Bring me all the texts you can in ONE message, not one by one. Try each line in your container before you ask.
3. Any new download (Firefox for the tests is one) comes to me before it is used, with its address, version, size and checksum (TOOL-SOURCE). No other new package or tool.
4. Your fence: .github/workflows/toolchain.yml, what Firefox needs in the test configuration under packages/e2e/, your notes under .checkpoints/, and the docs of your docs pass. No game code: nothing under crates/ and nothing in the packages' source. Those belong to Builder V3, who builds change-set 3b at the same time on its own branch. If a Firefox test fails for a reason that needs game code, stop and tell me in three lines with options. Do not fix it.
5. Push only to your own branch. Never open spikes/. Do not switch your main folder; work in a worktree.
6. You open no pull request yourself. Put its title and text in your notes folder (pr-title.txt, pr-body.md) and tell me. My main Advisor opens the draft and merges on my tap, after your advisor agent approved the full commit id and gate, smoke and toolchain are green on it.
7. Keep it small and cheap: agents on Opus 5.5 at effort high, at most 2 at a time, short notes, my words saved by script (chat-words.mjs is in the notes at commit 6491b839). If the pull request passes 300 changed lines outside your notes and docs, stop and tell me in three lines.
8. Use plain words with me: no tag or code without its explanation in the same sentence. "Decisions needed" and "For Mustafa" come last, as CLAUDE.md says.

First reply, at most 8 lines: your model, your branch, your permission mode, what you read, and anything that contradicts this block. Then write the plan and have the advisor agent review it without waiting for me.
