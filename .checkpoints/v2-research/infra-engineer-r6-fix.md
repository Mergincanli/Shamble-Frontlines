Task: R6 wave 2 fix step (apply verifier spot-check to docs/research/r6-infrastructure.md), started 2026-10-29.

- Saved verifier report verbatim to .checkpoints/v2-research/verifier-r6-result.md. Next: read topic file and verifier notes.
- Read topic file (1010 lines), verifier notes (all quotes), R2 r2-netcode.md:2662-2668 (REC5 "per game server hostname"), R2:2382 (BUD-REC6 rests on "legacy CM4"; inventory CM4 = DM_Tri map row, TL15 = perfbots). VISION_V2 §4 l.54 "Matches feel fair up to about 100 ms of ping".
- Plan (verifier items -> action): 1 REC4 departure stated + shared key to SEC-OQ1; 2 K4 reworded; 3 Rests-on adds (REC2, REC1); 4 F20 into REC2 text+Rests-on; 5 fsn1 qualified in F13/Q4/OQ2; 6 NOT applied as options (no new sources: fix rule 1/5) -> named as unread candidates in OQ4/OQ6 + Sources not read; 7 25 MiB mark labelled Pages limit + Workers limits page in not-read lists; 8 Fly hourly cell; 9 F15 quote + label upgrades (COST-F1, COST-F5, OBS-F9, HOST-F3); 10 COOP/COEP "inferred" in TL;DR, options, R1 pointer; 11 OBS-REC4 Sentry clause dropped -> COST-OQ2, Serves lines fixed; 12 TEL-REC2 stub record instead of append file; 13 CM4/TL15 note; nit 3.156.
## Change log
- batch 1 applied (items 1, 3, 4, 7):
  - recommendation: HOST-REC4 text states departure from R2-TRN-REC5 "per game server hostname" (r2:2667), shared key -> SEC-OQ1, per-hostname alternative named
  - nit: HOST-REC4 Rests-on notes the departure
  - nit: SEC-OQ1 items met on the way + wildcard private key
  - nit: Options Q3 C2 names key copy and departure
  - nit: HOST-REC1 Vercel fallback cites F8, F9; reason marks COOP/COEP by name inferred
  - nit: HOST-REC1 Rests-on + F8, R1-BND-REC5
  - recommendation: HOST-REC1 pass mark 25 MiB labelled Pages limit, recheck vs Workers limits page
  - recommendation: HOST-REC2 text conditional on OQ1 with F20 quote
  - nit: HOST-REC2 Rests-on + F20, R2-BUD-REC6, R2-ROOM-REC2, R2-TRN-REC3, R1-DET-OQ7, R1-DET-REC6
  - nit: hinge table + Workers static-assets limits row
  Next: items 2, 5, 6, 8, 9, 10.
- batch 2 applied (items 2, 5, 6, 8, 9, 10, 13, arithmetic nit):
  - nit: K4 comparison reworded (per full host 369 USD vs 86.49 EUR; goal 276 USD same order as 222.88-345.96 EUR; no rate)
  - nit: HOST-F13 + verifier's "NBG1 HEL1" location column note
  - nit: Q4 and HOST-OQ2 qualify fsn1
  - nit: HOST-OQ4 + Hetzner dedicated root servers (unread candidate); HOST-OQ6 + tls-alpn-01 RFC 8737 (unread)
  - nit: Sources not read + rows 18-20 (RFC 8737, Hetzner root servers, Workers static-assets limits) -> 20 not read
  - number: HOST-F22 hourly rates 0.0458/0.0917/0.1833 USD added (verifier quotes)
  - number: room-hour table Fly compute 0.0229 USD (was "not computed")
  - nit: HOST-F15 quotes (table cells); heading note scoped to general-purpose
  - nit: label upgrades HOST-F3 builds, COST-F1 14 days, COST-F5 1 TB+locations, OBS-F9 list; OBS-F7 DWARF marked paraphrase
  - nit: TL;DR "can send ... (inferred)"; Options B/C "inferred"; R1 pointer inferred note
  - nit: TL15 legacy line: CM4 id slip is R2's
  - nit: 3.156 GB note in the full-host egress sentence
  Disagreement (item 6): not added as Options rows; reason: fix rule 1 and 5 (no new sources unless a rec cannot stand without; REC4 and REC2 stand); recorded as unread candidates in OQ4/OQ6.
  Next: items 11 (OBS-REC4, Serves lines), 12 (TEL-REC2), then part 7, header, TL;DR counts.
- batch 3 applied (items 11, 12):
  - recommendation: OBS-REC4 Sentry retention clause dropped -> open, decided by R6-COST-OQ2's read
  - nit: OBS-REC4 Serves line: says no §6 item asks for it (a condition for §6.1/§6.6 stores)
  - nit: OBS-REC1 Serves line + §4 fair tick
  - nit: COST-OQ2 names Sentry retention
  - recommendation: TEL-REC2 append-file recovery replaced by a supervisor stub record marked incomplete (consistent with TEL-REC1)
  - nit: TEL-REC2 Rests-on + R6-TEL-REC1
  - recommendation: TEL-REC2 pass mark: SIGKILL leaves a stub record marked incomplete with exit reason
  Next: part 7 table, header status, counts.
- batch 4 applied: part 7 (verifier table + final state column, 13 critical-read items with actions, counts); header status "spot-checked (19 of 40 key claims)"; header totals: sources not read 17 -> 20 (nit). Findings 43, recs 13, OQs 23, sources read 42 unchanged.
  Next: sanity checks (TL;DR <= 5 lines, grep leftovers), then DONE.
- Checks: TL;DR 5 lines; no stale "sends COOP", "shortest setting", "appends to", "not computed per hour", "rests on it too", "spot-check pending". Remaining "inferred (summary)" lines are in unchecked findings (HOST-F4, COST-F3, OBS-F10): untouched.
- Totals by type: recommendation 6 (HOST-REC1 mark, HOST-REC2 text, HOST-REC4 text, OBS-REC4 text, TEL-REC2 text, TEL-REC2 mark) · number 2 (F22 hourly rates, Fly room-hour cell) · nit 23 · source 0.
- No rec became an open question; nothing re-sourced; not applied: item 6 as options (recorded as unread candidates in HOST-OQ4/OQ6). Final status: "spot-checked (19 of 40 key claims)".
DONE
