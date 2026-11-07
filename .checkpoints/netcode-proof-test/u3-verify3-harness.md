# u3-verify3-harness: third skeptic round on harness v3 (fixes.rs) at 1e020a7

Agent: netcode-architect (skeptic). Started 2026-11-07. Writes only this file. No tracked edits, no commits.

## Plan
1. Read prior rounds (u3-verify2-harness.md, u3-build-v3-harness.md), commit message, fixes.rs whole.
2. (1) H6/H7/H8/H19, follow-on window, takeover counts: read code, cite lines.
3. (2) Chromium endpoint lock + WS throttler; Firefox admission queue + fail-delay: compare with sources
   (scratchpad chromium-src, cr2; fetch missing).
4. (3) TLS resumption flights: browser WS reconnect resumption; ClientHello size; 1,300 B resumed server flight
   with ML-KEM; full-handshake option.
5. (4) Multi-burst lineage: accumulation between bursts; why abort-500 Chromium hits 4,994 ms throttle by burst 30.
6. (5) Windows-like client option: client_cfg applied to both ends; server-end fidelity.
7. (6) Censoring at 60 s.
8. (7) Policy recommendation; (8) ADR 0001 / plan conflicts update; L2 changes (H17/H18).
9. (9) Too-good / too-bad hazards. Run tests and tables in a throwaway target dir; compare with u3v3-tables1.txt.
10. Verdicts, bugs, must-correct. End with DONE.

## Findings (a line per finding as I go)
