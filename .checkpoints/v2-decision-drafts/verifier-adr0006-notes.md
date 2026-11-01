Task: verify ADR 0006 (infrastructure and operations), docs/adr/0006-infrastructure-operations.md, per prompts/verifier-adr0006.txt (verifier, 2026-11-01)

- 04:18 Amsterdam: started fresh (no prior notes). Outside fetch hours: source checks will be marked pending.
- Read plan rev 3, advisor §1, the whole doc (1123 lines). Doc = infra draft lines 1–770 + test-engineer pass-mark subsection (doc 770–927) + draft 772–966 (doc line = draft line + 157 after 771). Ledgers: infra rows 1–355, test-engineer rows TE1…
- Early finding A1: doc line 927 "---8<--- the subsection ends on the line above ---" is a cut marker left in the assembled ADR (assembly artifact).
- Next: read both ledgers in full, then check rows against research by section.
- Read both ledgers in full: infra 355 rows (1–355), test-engineer TE1–TE226. Next: r6 verifier section + findings-checked.tsv, then read r6 in full and check rows section by section.
- r6 1–618 read. Arithmetic re-done OK: 3.156 GB/match-h (146.1×48 kbps), 8×3.156×730=18.4 TB, 17.98, 101.97, 137 GB, 361.44, 238.36 (K2 222.88), 518.94/534.42, 9.2 TB, 10.81/21.62, 36 %, 9.62×, 276/369 USD, A-8 0.62 GB, A-9 0.8 GB, A-10/A-11 1 MB/13 GB, TL13 means 8.67.
- Finding candidate F-Q1: trade-off line 533–534 quotes A-2 as "a density to measure, not one the server tick budget guarantees" — r6:518 has "a density to measure at the tick rate R1-TICK-REC2's rule picks (R2-ROOM-REC2 mark 2), not one the server tick budget guarantees": elision not marked inside quotes (low).
- Candidate: part 10 "`userInfo` off, so the SDK does not send IP addresses" vs F10's caveat (IPs can still arrive in headers/cookies/query unless denied; inferred) — check REC3 wording.
