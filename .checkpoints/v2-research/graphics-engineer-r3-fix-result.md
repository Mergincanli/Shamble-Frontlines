# R3 wave 2 fix step: the owner's report, verbatim

Workflow run wf_1f477dcb-7cb (prompt `prompts/graphics-engineer-r3-fix.txt`), returned 2026-10-28 (about 20:45 UTC). Notes: `graphics-engineer-r3-fix.md`.

I applied the verifier's spot-check to `/home/user/Shamble-Frontlines/docs/research/r3-rendering.md`, editing it in place. Nothing was committed or pushed.

**Changes by type:** 32 in total.
- **nit:** 19.
- **recommendation:** 9. These are GL2-REC1, LOOK-REC2, CHAR-REC1, CHAR-REC2, CHAR-REC4, BUDGET-REC1, BUDGET-REC2, AUDIO-REC1 and AUDIO-REC2.
- **number:** 4.
  - CHAR-REC4 now uses 4 large units, 200 draw calls and 2.0 ms.
  - BUDGET-REC1 adds a proposed laptop CPU floor of 4 cores and 8 threads. No source gives it.
  - BUDGET-REC2's Low draw-call cap is now "≤ 800 without a per-object outline pass; about ≤ 1,100 with one", with the arithmetic shown.
  - WGPU-OQ5 now gives the gaps between releases as 55, 76 and 85 days.
- **source:** 0.

**Recommendations that became open questions:** none.

**Re-sourced claims:** none. The verifier found nothing wrong or unsupported, so I fetched no new pages.

**Verifier findings I did not apply in full:**
- **C7 (missing options):** I added them as five open questions instead of option rows: R3-GL2-OQ5, R3-LOOK-OQ6, R3-CHAR-OQ6, R3-BUDGET-OQ6 and R3-AUDIO-OQ6. The fix rule allows no new claims, and an option row would need sourced findings. This takes the open-question total from 33 to 38.
- **L5:** the verifier marked it note-only, so I made no change.
- **C10:** nothing to do.

**Final header status:** "spot-checked (44 of 77 key claims)".

Files:
- `/home/user/Shamble-Frontlines/.checkpoints/v2-research/verifier-r3-result.md` (the verifier's report, verbatim)
- `/home/user/Shamble-Frontlines/.checkpoints/v2-research/graphics-engineer-r3-fix.md` (the change log, ends with DONE)
