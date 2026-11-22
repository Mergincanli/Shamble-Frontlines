# netcode-architect: U6c build brief, rev 1 (plan and notes)

Task (main session, 2026-11-22 15:1x UTC): draft U6c's build brief rev 1 into `u6c-brief.md`; the main session sends it
to the advisor agent for its one check (NPT-TRIM). I build nothing and edit no code. (No Edit tool in this run: this
notes file is rewritten whole with Write at each step.)

## Plan
1. Read replan-units.md §1 (U6c row :60, dispositions), §3 (certificate by DNS-01 by hand), §7 (lane); replan.md:61-65,
   :89-94 and :93 (security seat).
2. Read u6-brief.md rev 2.1, u6-facts.md, advisor-u6-brief.md (conditions on U6c's blocks by number).
3. Read plan.md:1254 (U6's row: C-U6-1..8 word for word, the -3 probe's carries).
4. Read u6a-brief.md rev 1.1 (section 0.1: C-U6-4, `StealReader`, npt-super's hold form; section 6 rules) and
   advisor-u6a-brief.md.
5. Read u6b-brief.md rev 1.1 (section 0.1: condition 4, the hash file) and advisor-u6b-brief.md.
6. Read u8-brief.md rev 1.1 section 0.1 (C10, Q-U8-4, Q-U8-5, C4, C-U8-2, C-U8-6) and advisor-u8-brief.md.
7. Read security-r1-deps.md (instant-acme =0.8.5, 16 crates, SEC-R1D-4, DEP-HOLD :249-253) and dependency-check.md.
8. Check the code on the branch for what U6c touches (npt-server, probe, npt-bots, npt-load, page host) to name symbols.
9. Write u6c-brief.md in u6b-brief.md's shape; the task's sections 1-8.
10. Check every word-for-word quote as a substring of its source; hand back a 15-line summary with contradictions cited.

## Notes (one line per finding)
- Steps 0-8 DONE (details in the brief; the findings below are what shaped it).
- N1 row replan-units.md:60 = 1,950 (2,650-2,950 likely). Cuts touching U6c: :11-14, :26-27, :32. §3 :86-121. §7 :192-216.
- N2 replan.md:68-71 (server checks Cloudflare's name servers; R2/R3/R5 may skip the public order); :93 security seat.
- N3 old-U6d conditions now U6c's: U6-Q2, U6-Q17 (3), U6-Q18, Q11, Q12, conditions 1, 6, 7 (advisor-u6-brief.md).
- N4 plan.md:1254 split on <br>: C-U6-1..8 bullets 24-31; C-U6-8 ends with no period.
- N5 u6a conditions 2, 3, 5; section 6 rules. N6 u6b condition 4; Block 6 WS cert files from npt-cert.
- N7 u8 C10, Q-U8-4, Q-U8-5, C4, Block 2 probe port, npt-cert. N8 STALE u8-brief.md :383, :505-507, :521 (knob to U6c)
  superseded by C10 (:97-101, :172-174).
- N10-N14 security-r1-deps.md (17 = instant-acme + 16; rcgen pem via instant-acme ring :70-72; SEC-R1D-4; binding
  check :249-253 for U6 whole); hyper no server feature; docs.rs instant-acme 0.8.5 API (finalize() behind `rcgen`
  feature; finalize_csr; Account::profiles; builder_with_http; BytesResponse From<Response<B>>).
- N15 SERIES.md:497-501 fixes the probe's home (npt-server probe mode). N16 ADR 0001:1590 base RTT. N17 no bite.
- N18 E machinery is test code (socket_tests.rs:676-810, :1384-1406; stats.rs:426-505). N19 slack via /proc.
- N20 C-U6-8 sites socket.rs:501-503, :724-726. N21 bots clock ban. N22 check.mjs:101-103. N23 deny warn on duplicates.
- N24 npt-load writes inputs_count and cpu_permille (SERIES.md:432-433; README:269-276). N25 plan.md §10.4.
- N26 CONTRADICTION plan.md:286-290 (host-key check over the production cert before the first SSH) vs
  replan-units.md:110-111 / u8-brief.md:416-423 (cert ordered over SSH after bring-up). In the brief as Q-U6C-6.
- N27 GAP no U8 verb starts `npt-host serve` as npt-host (u8-brief.md:205-240). Q-U6C-7.
- N28 one `/npt/cert-hash.json` (plan.md:994-995, u6b-brief.md:64-66) vs a room per mark and browser (u8-brief.md:55-63)
  each with its own cert (u6b-brief.md:212-214). Q-U6C-3.
- Step 9 DONE: u6c-brief.md rev 1 written (sections 0-10; Q-U6C-1..8). Size about 2,800 (2,550-3,050), likely
  3,800-4,200; at least +47 measured and +47 bites; 17 plants; lane before R1 about 11,050-12,550 likely.
- Step 10: quote check next (scratchpad script, read-only).
