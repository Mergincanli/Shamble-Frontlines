# R6 wave 2 fix step: the owner's report, verbatim

Workflow run wf_12f840d1-878 (prompt `prompts/infra-engineer-r6-fix.txt`), returned 2026-10-29 (about 21:17 UTC). Notes: `infra-engineer-r6-fix.md`.

I applied the R6 spot-check in place in `/home/user/Shamble-Frontlines/docs/research/r6-infrastructure.md` and saved the verifier's report verbatim first.

**Changes by type:**
- recommendation: 6
  - R6-HOST-REC1: the 25 MiB pass mark is now labelled as the Pages limit, to be rechecked against Workers' own limits page.
  - R6-HOST-REC2: the Hetzner pick now quotes the "currently unavailable" message (F20) and depends on R6-HOST-OQ1.
  - R6-HOST-REC4: it now says plainly that one wildcard certificate departs from R2's "per game server hostname", and the shared private key is listed in R6-SEC-OQ1.
  - R6-OBS-REC4: the unsourced Sentry retention cap is dropped and left to R6-COST-OQ2.
  - R6-TEL-REC2 text and pass mark: a crashed room now leaves a stub record marked incomplete, which matches TEL-REC1.
- number: 2 (Fly.io hourly rates added to HOST-F22; the Fly room-hour cell is now 0.0229 USD)
- nit: 23 (missing "Rests on:" entries, the reworded K4 comparison, fsn1 qualified, COOP/COEP marked inferred, label upgrades, the CM4/TL15 note, the 3.156 GB note, part 7, header totals with 20 sources not read)
- source: 0

**Open questions:** none of the recommendations became one.

**Re-sourced claims:** none.

**Not applied:** critical-read item 6 (Hetzner dedicated root servers, ACME tls-alpn-01 in RFC 8737). I did not add them as options because the fix rule allows a new source only where a recommendation cannot stand without one, and REC2 and REC4 stand. Both are named as unread candidates in R6-HOST-OQ4 and OQ6 and listed under "Sources not read".

**Final header status:** "spot-checked (19 of 40 key claims)".

The change log is in `/home/user/Shamble-Frontlines/.checkpoints/v2-research/infra-engineer-r6-fix.md`, and the report is in `/home/user/Shamble-Frontlines/.checkpoints/v2-research/verifier-r6-result.md`.
