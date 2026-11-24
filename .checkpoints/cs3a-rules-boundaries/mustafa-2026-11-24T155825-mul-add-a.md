# Mustafa, 2026-11-24, 15:58:25 UTC: MUL-ADD (a), the fused multiply-add stays banned in the simulation; two conditions from his Advisor; the stop line stays; four decisions of his to record; plain words with him from now on

_Copied by script from the chat's transcript (session_01WLuqnvpAh9XTitx8ehKJcQ), never retyped: the user message of 2026-11-24T15:58:25.313Z (uuid ae21a771-6b61-41e8-8a91-81d47c5f0946), whole (1894 bytes, sha256 78e74b7a9614ca8e186f68a7c94b4d9cf4d81c85c4ec880f68bddf7fbb94e528). What it answers: the chat's question MUL-ADD of 15:32:07 UTC (repeated at 15:51:16 and 15:55:06 UTC): does the fused multiply-add (`mul_add`) stay banned in the simulation, with the options (a) keep the ban, (b) our own exact version when a formula needs it, (c) allow it and require the instruction on servers; the chat recommended (a). Points 2 to 5 answer nothing the chat asked_

---
For Builder V3, from me, written with my main Advisor (24 November). From now on use plain words with me: no tag or code without its explanation in the same sentence.

1. MUL-ADD: (a). The fused multiply-add stays banned in the simulation; it writes a * b + c. What 3a built for it stays as it is (the one-name ban, the 48 hard cases, exact natively, the 16 WebAssembly values pinned as a known toolchain defect). No word of ADR 0004 changes in 3a; part 7 gets its dated pointer at its next revision, with its exact sentence shown to me first.
2. Two conditions from my Advisor, both for the report and STATUS's follow-ups:
   - The check on the built file (no fused multiply-add inside the players' WebAssembly) is built in the same change-set that brings the first outside crate into the simulation (the collision library, planned for change-set 4), not later. That change-set's plan names it.
   - "Report the defect to the Rust project" is a follow-up. Nothing is posted anywhere without my go.
3. The stop line of 4,100 lines stays. If round 2's fixes would pass it, cut by the plan's ladder and tell me in three lines.
4. Also record, by script with 3a's docs pass, these four decisions of mine of 24 November. They change nothing in 3a:
   - "No separate animation editor. Rigging and animation happen in Blender, with the Blender MCP."
   - "Testlines stays for feel and metrics. The real level editor goes in the main game (Editor V2)."
   - "The level editor can start against the offline core, but 3b stays first in line for the main builder."
   - "VFX repo stays separate for now; we merge later. Both sides should agree on one effect file format, so the merge is easy."
5. Carry on: round 2, the verifier, the advisor agent on the head, the docs, the report and my zip. When the pull request's title and text are in your notes folder, tell me, and my main Advisor opens the draft.
