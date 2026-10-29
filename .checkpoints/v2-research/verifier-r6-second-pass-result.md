# R6 wave 2 second pass: the verifier's report, verbatim

Workflow run wf_d5608bdb-da9 (script `v2-research-r6-second-pass-run1.js`, prompt `prompts/verifier-r6-second-pass.txt`), returned 2026-10-29 (about 21:26 UTC). Notes: `verifier-r6-second-pass.md`.

VERDICT: PASS WITH NOTES (R6 second pass, `/home/user/Shamble-Frontlines/docs/research/r6-infrastructure.md`)

The fix closes all four medium findings. What remains is only nits, and one of them is a wrong statement in the CM4/TL15 note. I checked the working tree against 9d5e78a, because the fix is only partly committed (11b2089 plus uncommitted edits). The gate and symbol checks were skipped, as the task says.

**Item 1: the spot-check's 13 findings**
- 1 (medium), certificate departs from R2: **closed.** REC4 at l.661–665 now quotes R2 in full, which matches r2-netcode.md:2667. Option C2 says so at l.416–419. The shared key is in SEC-OQ1 at l.904–906.
- 2 (medium), K4 comparison: **closed, one nit left.** K4 is reworded at l.500–504. But "costs more per host" (369 USD against 86.49 EUR) is still a comparison across currencies with no rate read. It only holds below about 4.27 USD per EUR. Either say "(inferred)" or drop the comparison.
- 3 (medium), missing "Rests on:" entries: **closed.** See l.614 and l.638–641. R2's marks are at r2:2733–2737 ("8 such rooms … within 10 %", "under 2 s").
- 4 (medium), F20 left out of REC2: **closed** at l.630–633 and l.638. Nit: the TL;DR (l.44) still names Hetzner as the pick with no condition.
- 5, fsn1: **closed** at l.138–140, l.424–426 and l.823–824. I re-read the product pages and they confirm it.
- 6, two missing options: **not applied. The owner's reason holds:** REC2 and REC4 stand without those sources. The options are recorded at l.829–831 and l.837–839, and in sources-not-read rows 18–19. One problem is left, covered under item 4 below.
- 7, the 25 MiB mark: **closed** at l.620–622, l.915 and sources-not-read row 20.
- 8, Fly.io hourly price: **closed** at l.451 and l.183–188.
- 9, F15 had no quote: **closed** at l.154–156.
- 10, COOP/COEP stated as fact: **closed** at l.43, l.399–401, l.611–612 and l.934–935.
- 11, logging loose ends: **closed** at l.738–741, l.745–746, l.705–706 and l.849.
- 12, TEL-REC2 against TEL-REC1: **closed** at l.783–788 and l.792–793.
- 13, CM4 against TL15: **still open (low).** l.208–211 says "the measurement is TL15's, so the id slip is R2's", and part 7 repeats it at l.324. That is wrong. The CM4 row (LEGACY_INVENTORY l.152) carries "perfbots at 48 soldiers, combat, mean 0.80-1.00 ms per update step, p95 0.97-1.54, 3 reps (:48)". GC2 (l.241) and TL15 (l.400) carry the same figure. So R2's citation is valid and there is no slip. The note should say all three rows carry the figure, which was measured on DM_Tri.
- Arithmetic nit (3.156 GB): **closed** at l.454–455.
- Label upgrades: F3, COST-F1 and COST-F5 are closed.
  - OBS-F9 (l.317–319) calls six items "the list", but the page lists eleven. Say "includes".
  - OBS-F7 (l.299–300): the link text "WASM DWARF debug information" now comes back in the fetch. "DWARF" can move up to confirmed.

**Item 2: the rewritten recommendations**
- HOST-REC1, HOST-REC2, HOST-REC4, OBS-REC4 and TEL-REC2 now follow from their findings. Their "Rests on:" lists are complete, and no two pass marks contradict each other.
- Nit: the R2 pointer (l.936–938) does not mention REC4's departure from R2-TRN-REC5.

**Item 3: numbers the fix added or changed**
- All are sourced, and I confirmed them at the source.
- All arithmetic checks out when redone: 0.0917/4 = 0.0229; 8×3.156×730 = 18,431 GB, which gives 368.62, so 369 USD; 6×3.156×730 = 13,823 GB, which gives 276.47, so 276 USD; 146.1 kbps × 48 × 0.45 MB = 3.156 GB.

**Item 4: open questions the fix added**
- HOST-OQ2, COST-OQ2 and SEC-OQ1 add no new fact.
- HOST-OQ6 (l.837–839) states a new fact, "needing only TCP 443", from a source it calls not read. The fact is true: RFC 8737 says "This connection MUST use TCP port 443." The file should cite it. "Which the WebSocket fallback may open anyway" is unmarked inference: R2 names no port (grep finds no 443), and REC2 at l.627 opens only "one TCP range".
- HOST-OQ4 calls root servers "a further candidate at goal scale" with no source. Acceptable as a candidate.

**Item 5: header, totals and part 7**
- Totals at l.27–28 match the file: 43 findings, 13 recommendations, 23 open questions, 42 sources read, 20 not read.
- The status at l.8 ("19 of 40") and part 7 at l.967 are out of date. The fix added F8 (l.614) and F20 (l.638) to "Rests on:" lists, so there are now 42 key claims.
- Part 7's list of 21 claims not checked should also name F8 and F20, unless this pass's checks of them count.
- Part 7's item 13 at l.324 is wrong, as above.

**Claims and numbers checked**

| id | claim (short) | source / arithmetic | verdict | location and exact words |
|---|---|---|---|---|
| HOST-F3 | 100,000 files on paid plans; builds 500/5,000/20,000 | developers.cloudflare.com/pages/platform/limits (Sep 5, 2026) | confirmed | "Paid plans (such as Pro, Business, and Enterprise plans) can have up to 100,000 files per site."; row "Builds per month \| 500 \| 5,000 \| 20,000" |
| HOST-F13 | product pages list "NBG1 HEL1" | hetzner.com/cloud/cost-optimized, /general-purpose | confirmed | CX23, CX33, CX43, CAX21, CCX13, CCX23 and CCX33 all show "eu-central NBG1 HEL1" (the CCX23 Create link uses fsn1, so the "unverified" hedge is right) |
| HOST-F15 | new cells CCX13, CX23, CX43 and the rest | same pages | confirmed | "€ 43.49 per month, excl. VAT"; "€ 5.99 per month, excl. VAT"; "€ 16.49 per month, excl. VAT"; CCX23 "€ 86.49 per month, excl. VAT" "€ 0.1386 per hour, excl. VAT"; heading "Price incl. IPv4" |
| HOST-F20 | "currently unavailable" | same pages (still shown 2026-10-29) | confirmed | "This product is currently unavailable. Please check back later." |
| HOST-F22 | hourly rates | fly.io/pricing | confirmed | "\| performance-1x · 2GB \| $0.0458 \| $33.00 \|"; "\| performance-2x · 4GB \| $0.0917 \| $66.00 \|"; "\| performance-4x · 8GB \| $0.1833 \| $132.01 \|" |
| HOST-F8 | vercel.json headers | vercel.com/docs/project-configuration/vercel-json (2026-08-14) | confirmed | "A pattern that matches each incoming pathname (excluding querystring)."; "A non-empty array of key/value pairs representing each response header." |
| COST-F1 | 14 days retention | grafana.com/pricing | confirmed | "14 days retention" (metrics); "14 day retention" (logs, traces) |
| COST-F5 | 1 TB included; locations | hetzner.com/storage/object-storage | confirmed | "This base price includes 1 TB of storage (up to 744 TB-hours) and 1 TB of egress traffic."; "Falkenstein, DE (FSN1), Helsinki, FI (HEL1)," "and Nuremberg, DE (NBG1)." |
| OBS-F7 | DWARF | docs.sentry.io/platforms/native/guides/wasm | confirmed (can be upgraded) | "if your compiler can output" [WASM DWARF debug information] "alongside the binary code." |
| OBS-F9 | data kept in the US | docs.sentry.io/organization/data-storage-location | confirmed, but the list is partial (6 of 11) | "Access tokens for users and organizations"; "DSN keys"; "Cron check-ins"; the file leaves out "Organization audit logs", "Project metadata" and 3 more |
| HOST-OQ6 | tls-alpn-01 needs only TCP 443 | rfc-editor.org RFC 8737 (Feb 2020) | confirmed at source, unsourced in the file | "This connection MUST use TCP port 443." |
| HOST-OQ6 | the WebSocket fallback may open 443 | r2-netcode.md (no "443"); r6 l.627 | unsupported (unmarked inference) | "one TCP range for the WebSocket fallback" |
| REC4 quote | R2 "per game server hostname" | r2-netcode.md:2667 | confirmed | "a publicly trusted certificate per game server hostname." |
| REC2 marks | 8 rooms, 10 %, 2 s | r2-netcode.md:2733–2737 | confirmed | "8 such rooms … within 10 % of the single-room figure"; "under 2 s" |
| OBS-REC1 Serves | §4 quote | VISION_V2.md:54 | confirmed | "Matches feel fair up to about 100 ms of ping." |
| TL15 note | "the id slip is R2's" | LEGACY_INVENTORY l.152, 241, 400 | wrong | CM4: "perfbots at 48 soldiers, combat, mean 0.80-1.00 ms per update step" |
| room-hour | 0.0229 USD | 0.0917 ÷ 4 = 0.022925 | confirmed | l.451 |
| full host | 369 USD | 8 × 3.156 × 730 = 18,431 GB; × 0.02 = 368.62 | confirmed | l.454–457 |
| goal | 276 USD | 6 × 3.156 × 730 = 13,823 GB; × 0.02 = 276.47 | confirmed | l.488 |
| K4 | costs more per host than Hetzner | 369 USD against 86.49 EUR, no rate read | unsupported (needs a rate; holds below 4.27 USD/EUR) | l.500–502 |
| totals | 43 / 13 / 23 / 42 / 20 | id count by grep; table rows counted | confirmed | l.27–28 |
| status | 19 of 40 | "Rests on:" lists now name F8 and F20 | wrong (now 42) | l.8, l.967 |

**Counts:** 19 confirmed, 2 wrong, 2 unsupported (23 checked).

**Only nits left?** Yes. All of them can be fixed in place, without new research:
- The CM4/TL15 wording at l.208–211 and l.324.
- The status and part 7 count (40 should be 42).
- OQ6: cite RFC 8737 and mark the 443 point as inferred.
- K4: mark "(inferred)" or drop the comparison.
- The TL;DR should say the Hetzner pick is conditional.
- OBS-F9: say "includes".
- OBS-F7: "DWARF" can move up to confirmed.
- The R2 pointer should mention REC4's departure.

**Header label:** "spot-checked (21 of 42 key claims)". This counts F8 and F20, which this pass checked at their source. Counting only the wave 2 spot-check, it would be "spot-checked (19 of 42 key claims)".

Notes, with every quote and ending in DONE: /home/user/Shamble-Frontlines/.checkpoints/v2-research/verifier-r6-second-pass.md
