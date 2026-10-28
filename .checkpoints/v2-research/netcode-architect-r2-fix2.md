Task: R2 netcode second fix round (wave 1 step 4): apply verifier second pass to docs/research/r2-netcode.md

## Progress
- 2026-10-28: read verifier second-pass result + notes, scope (head), first fix log; located every target passage. Backup: scratchpad r2fix2/r2-netcode.before-fix2.md. No re-sourcing needed: ROOM-F12/REC3 correction uses the verifier's W3C TR quote (row 10, already a source) and BCD (row 21); SYNC-F36 correction uses g_active.c (row 115) quotes, bg_pmove.c only as a verifier note (not added as our source).
- Plan: batch A sync (F36, REC2/3/5/8 marks, option 3.5 A/B); B budgets (REC1 lead/Rests on/mark, REC5 wording); C INT-REC6 6.9; D transport (F36 passage, F40 wording, REC1, REC3); E rooms (F12, option 3.2 line, REC3, CD2); F part 7 subsection, header, TL;DR. Edits by python exact-replace (no Edit tool here).

## Change log (type · what · where)
Batch A (sync), applied:
- nit · SYNC-F36 movement clause corrected to g_active.c's words (timer actions get the 200 ms cap; movement via Pmove), verifier note with bg_pmove.c quotes · SYNC-F36 (verifier A: wrong)
- nit · SYNC-REC2 M3 "jump on everyone else's screen" marked as the review's reasoning
- nit · SYNC-REC3 "cancel the spread" marked as the security review's reasoning
- nit · SYNC-REC5 "far more rewind" marked as the review's reasoning
- nit · SYNC-REC5 "WebSocket has no transport RTT" -> "no finding read gives a transport RTT (our reading)" (verifier D: unsupported)
- nit · SYNC-REC8 "aimbot" marked as the security review's reasoning
- nit · option 3.5 A: lead amount marked our reading (F34 unsupported); 3.5 B: victim window now "open (OQ15)" (verifier F)
Next: batch B budgets.
Batch B (budgets), applied:
- recommendation · BUD-REC1 lead sentence: 60 Hz rejected by bandwidth; 30 over 20 Hz only for a 50 ms shorter rewind on the RTT reading, inside ~200 ms only if one-way (XU-OQ2) (verifier D: problem 1)
- recommendation · BUD-REC1 pass mark: "rewind ≤ 200 ms" -> "≤ the cap (XU-OQ1: 200 or 250 ms) for 100 %, clipped share recorded" (verifier D: problem 2)
- nit · BUD-REC1 Rests on + R2-SYNC-F28 (verifier note)
- nit · BUD-REC5 "sending settings"; incomingMaxAge also in all three (verifier note)
Next: batch C (INT-REC6) + D (transport).
Batch C+D (interest, transport), applied:
- nit · INT-REC6 Serves: §6.9 removed as from INT-REC4 (verifier minor note)
- nit · TRN-F36: draft §3.2 "Origin only from browsers / Otherwise OPTIONAL" passage added (verifier quotes; same source) + second-pass confirmed note (verifier B, D)
- nit · TRN-F40: "the newest" -> "three of the six newest (Oct 12) patched only from 0.11.18, covers all ten"; verifier note with 0339/0341/0342/0185 floors (verifier A note)
- nit · TRN-REC1 reason: same F40 wording
- recommendation · TRN-REC1: no-Origin passage cited from F36; "not refused on that ground alone" marked as our policy reading (verifier ranked 5)
- recommendation · TRN-REC3: "documented only for Safari" -> W3C §6.2 defines the HTTP/2 path, only WebKit documents shipping it (verifier D: unsupported)
- nit · TRN-REC3: "no transport RTT" -> marked as our reading (verifier D: unsupported)
Next: batch E (rooms).
Batch E (rooms), applied:
- nit · ROOM-F12 corrected: WebSocket url+protocols only (no auth header, inferred); MDN's WebTransport list incomplete, W3C §6.9 `headers` option exists, shipping unknown (BCD no key); verifier note with W3C quotes (verifier ranked 1)
- nit · option 3.2 token-carriage line corrected the same way
- recommendation · ROOM-REC3 reason reworded: WebSocket constructor + URL leaks + one join path (design choice, our reading); first-fix header claim marked wrong (verifier D: wrong). Design unchanged.
- nit · CD2 legacy row: "guest id and settings; secret in HttpOnly cookie" (verifier F)
- nit · sources rows 10 and 21: Units column notes the second-pass use for ROOM-F12 (no new source; totals unchanged)
Next: batch F (part 7 subsection, header, TL;DR, "exact text search" line).
- nit · TRN-F40/TRN-REC1: "covers all ten" kept only in the verifier note (those 4 advisory pages are not our sources)
Batch F (part 7, header, TL;DR), applied:
- nit · part 7: new subsection "Second verifier pass (2026-10-28)": verifier table (22 rows, 49 claims) with final states; counts 6 fixed · 43 no change · 0 re-sourced · 0 OQ; other-findings table 2V-1..6, 2V-notes, 2V-§6; not applied (2V-§6); nothing re-sourced; ROOM-REC3 re-check note
- nit · part 7: "awaiting the second verifier pass" line -> checked; one reading wrong (F36) (verifier ranked 2); intro sentence
- nit · header status -> "verified (every recommendation claim)" + one line on the second pass; totals unchanged sentence; how-to-read markers; TL;DR "Reviewed and re-checked"
Next: whole-diff review against backup, TL;DR line count, then DONE.
