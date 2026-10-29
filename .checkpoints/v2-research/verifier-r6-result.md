# R6 verifier spot-check: the report, verbatim

Workflow run wf_12f840d1-878 (script `v2-research-r6-check-fix-run1.js`, prompt `prompts/verifier-r6.txt`), started 2026-10-28, cut off by the spend limit, resumed and returned 2026-10-29 (about 21:17 UTC; copied from the workflow's own result by the session). Notes: `verifier-r6.md`.

VERDICT: PASS WITH NOTES (research spot-check of R6, `/home/user/Shamble-Frontlines/docs/research/r6-infrastructure.md`)

Gate: skipped, as the task says (research check; the main session runs the gate). Symbol diff: not applicable.

**Spot-check table** (every source opened with WebFetch on 2026-10-28/29; exact quotes are in `/home/user/Shamble-Frontlines/.checkpoints/v2-research/verifier-r6.md`)

| id | claim | source | verdict | location and exact words |
|---|---|---|---|---|
| R6-HOST-F15 | Hetzner CX/CAX/CCX prices in EUR, 20 TB included | hetzner.com/cloud/cost-optimized, /general-purpose | confirmed | table cells: CX33 "€ 8.99 per month, excl. VAT" "€ 0.0144 per hour"; CCX23 "€ 86.49 per month, excl. VAT" "€ 0.1386 per hour"; "20 TB"; CCX33 "30 TB" "€ 138.99"; CAX21 "€ 10.99" |
| R6-HOST-F16 | extra traffic 1.00 EUR per TB | hetzner.com/cloud/general-purpose | confirmed (the words; the per-TB reading stays inferred, as the file says) | "For each additional TB at EU-Central locations we charge € 1.00 per hour, excl. VAT." |
| R6-HOST-F17 | billed by the hour, rounded up; only outgoing traffic | docs.hetzner.com/cloud/billing/faq (2020-07-07) | confirmed | "Servers have both a monthly price cap and a price per hour."; "We always round up the hourly usage of a server."; "We only bill for outgoing traffic." |
| R6-HOST-F22 | Fly.io prices | fly.io/pricing | confirmed | "performance-2x · 4GB \| $0.0917 \| $66.00"; "Egress, North America and Europe \| $0.02 / GB"; "Dedicated IPv4 \| $2.00 / mo" |
| R6-HOST-F3 | Pages: 20,000 files, 25 MiB per file, unlimited previews | developers.cloudflare.com/pages/platform/limits (Sep 5, 2026) | confirmed | "up to 20,000 files on the Free plan."; "25 MiB."; "unlimited number of preview deployments"; builds 500 / 5,000 / 20,000 (table cells) |
| R6-HOST-F5 | static requests free, no egress charge, 5 USD minimum | developers.cloudflare.com/workers/platform/pricing (Oct 2, 2026) | confirmed | "Requests to static assets are free and unlimited."; "There are no additional charges for data transfer (egress)"; "a minimum charge of $5 USD per month for an account" |
| R6-HOST-F1 | `_headers` file, 100 rules, 2,000 characters | developers.cloudflare.com/pages/configuration/headers (Aug 25, 2026) | confirmed | "You may define up to 100 header rules."; "Each line in the `_headers` file has a 2,000 character limit." |
| R6-HOST-F12 | GitHub Pages: COOP/COEP request open, "No ETA" | github.com/orgs/community/discussions/13309 | confirmed (community thread only) | "No ETA at the moment as to when this feature will be available." (Jul 10, 2023) |
| R6-HOST-F13 | EU locations and countries | hetzner.com/cloud | confirmed (which plan runs where stays inferred, as the file says) | "available to you in Germany, Finland, Singapore, and the USA."; "Falkenstein, Nuremberg, and Helsinki" |
| R6-HOST-F14 | shared vCPU for variable load; dedicated for gaming | hetzner.com/cloud | confirmed | "They are designed for variable usage patterns rather than sustained high workloads."; "highly predictable resources."; "gaming servers" |
| R6-HOST-F24 | ACME wildcard orders, dns-01 TXT record, http-01 on port 80 | RFC 8555 §7.1.3, §8.3, §8.4 | confirmed | "MAY have a wildcard domain name as its value."; "prepending the label "_acme-challenge""; "MUST be sent to TCP port 80" |
| R6-COST-F1 | Grafana Free: 10k series, 50 GB, 14 days, 3 users | grafana.com/pricing | confirmed (the 14 days too) | "Limited to 10k active series per month"; "Limited to 50 GB ingested per month"; "14 days retention"; "Limited to 3 active users per month" |
| R6-COST-F2 | Grafana Pro: 19 USD plus usage | grafana.com/pricing | confirmed | "Platform fee of $19 per month includes:"; "$6.50/ 1k series"; "$0.050/GB Process" "$0.400/GBWrite" "$0.100/GBRetain" |
| R6-COST-F5 | Hetzner Object Storage 6.49 EUR with 1 TB, EU locations | hetzner.com/storage/object-storage | confirmed (the 1 TB and the locations too) | "base price of € 6.49 per month, excl. VAT"; "includes 1 TB of storage (up to 744 TB-hours) and 1 TB of egress traffic."; "(FSN1) … (HEL1), and … (NBG1)" |
| R6-OBS-F5 | Prometheus: 15 days default, 1–2 bytes per sample, not replicated | prometheus.io/docs/prometheus/latest/storage | confirmed | "the retention time defaults to `15d`."; "an average of only 1-2 bytes per sample."; "not clustered or replicated." |
| R6-OBS-F1 | `tracing` 0.1.44: spans and events; no allocation by default | docs.rs/tracing/latest | confirmed ("not built" stays inferred, as the file says) | "a span represents a _period of time_ with a beginning and an end."; "does not allocate by default." |
| R6-OBS-F7 | Sentry Wasm: build_id, wasm-split, wasmIntegration, "still developing" | docs.sentry.io/platforms/native/guides/wasm | confirmed (the word "DWARF" is paraphrase only: the fetch dropped the link text) | "augmented by a build_id custom section."; "We provide a tool called wasm-split"; "apply its wasmIntegration"; "This feature is still developing, so may be rough around the edges." |
| R6-OBS-F9 | Sentry: US or EU (Frankfurt), cannot be changed, some data stays in the US | docs.sentry.io/organization/data-storage-location | confirmed (the US list too) | "Sentry supports data storage in both the US and the EU."; "Frankfurt, Germany."; "once selected, your data storage location can’t be changed." |
| R6-TEL-F2 | LOOP_V2 points race, steals, counters | docs/LOOP_V2.md §4 l.44, §8 l.98–100, §11 l.153–154 | confirmed | "The countdown keeps running, now for the new owner."; "Destroying an HQ is a big points swing"; "(cashouts, steals, deliveries, harvester kills)" |

**Counts:** 19 confirmed, 0 wrong, 0 unsupported. Two sub-points have only a paraphrase: OBS-F1 "not built" (the file already marks it inferred) and OBS-F7 "DWARF".

**Wrong or unsupported basis claims, per recommendation:** none for any of the 13. Each recommendation has at least one basis claim checked: HOST-REC1 (F1, F3, F5, F12), HOST-REC2 (F13–F17, F22), HOST-REC3 (F15, F17), HOST-REC4 (F24), COST-REC1 (COST-F1, F2, F5; HOST-F15), COST-REC2 (HOST-F15, F17), OBS-REC1 (F1), OBS-REC2 (F5, COST-F1), OBS-REC3 (F7, F9), OBS-REC4 (F5, F9, COST-F1), TEL-REC1 (F2), TEL-REC2 (COST-F5), TEL-REC3 (F2).

**Labels the owner can upgrade to "confirmed":** COST-F1's 14 days, COST-F5's 1 TB and locations, OBS-F9's list of data kept in the US, and F3's build counts (table cells).

**Critical read, ranked**
1. (medium) **The certificate plan quietly departs from R2.** R2-TRN-REC5 asks for "a publicly trusted certificate per game server hostname" (r2-netcode.md:2667). R6-HOST-REC4 instead copies one wildcard certificate to every VM, and quotes R2 cut short as "publicly trusted certificate". The departure should be stated. The shared private key on every VM should also go on R6-SEC-OQ1's list of items met on the way (today only the DNS key is there).
2. (medium) **Option K4's comparison does not follow from the file's own numbers.** K4 says per-GB pricing "costs more than the whole Hetzner fleet (above)". At the goal, Fly egress is about 276 USD against 345.96 EUR (fixed fleet) or 222.88 EUR (K2). The currencies differ and no exchange rate was read. The claim holds only for one fully loaded host: 369 USD against 86.49 EUR.
3. (medium) **"Rests on:" lists miss claims the recommendations use.**
   - HOST-REC2's reason uses R2-BUD-REC6, and its pass marks are R2-ROOM-REC2's (8 rooms, within 10 %, under 2 s; r2-netcode.md:2733–2738). It also relies on R2-TRN-REC3, R1-DET-OQ7 and R1-DET-REC6. None of these are listed.
   - HOST-REC1 uses F8 (Vercel headers) for its Vercel fallback and R1-BND-REC5 (option G3), and lists neither.
4. (medium) **HOST-REC2 picks Hetzner without naming F20.** F20 records that every EU plan showed "currently unavailable" on 2026-10-28. It appears only in the options risk column and in R6-HOST-OQ1, not in the recommendation's text or its "Rests on:" list.
5. (low–medium) **fsn1 may not offer the recommended plans.** As fetched, the product pages list "NBG1 HEL1" (no FSN1) for CX23, CX33, CX43, CAX21, CCX13, CCX23 and CCX33. Q4 and R6-HOST-OQ2 still offer fsn1 as a candidate without qualification.
6. (low–medium) **Two options a reader would expect, both readable on allowed domains:**
   - Hetzner dedicated root servers (www.hetzner.com) for goal scale.
   - ACME tls-alpn-01 (RFC 8737, rfc-editor.org), which needs only TCP 443; the WebSocket fallback may open that port anyway.
7. (low) **The 25 MiB pass mark is a Pages limit.** HOST-REC1's mark "largest build file under 25 MiB (F3)" applies the Pages limit to the preferred Workers static assets. The Workers static-asset limits page was not read and is not in "Sources not read".
8. (low) **The Fly.io hourly price was on the page.** The page gives $0.0917/h for performance-2x, so the room-hour table's "not computed per hour (monthly price only)" is wrong. At A-3 it is about 0.0229 USD per room-hour.
9. (low) **F15 is marked "confirmed" without a quote** (only "table values as extracted"). The verifier now has the cell values (table above).
10. (low) **"Sends COOP and COEP" is stated as fact.** The TL;DR and the R1 pointer say Cloudflare sends them, but F1 marks that inferred. The options table also gives Netlify and Vercel a plain "Yes" without the same "inferred".
11. (low) **Two loose ends in the logging recommendations.**
    - OBS-REC4's "crash reports at Sentry's shortest setting" has no source showing Sentry has a retention setting.
    - The "Serves" lines of OBS-REC1 and OBS-REC4 (§6.1, §6.6) are a loose trace; §4's fair tick is the better one.
12. (low) **TEL-REC2 conflicts with TEL-REC1.** TEL-REC2 recovers crashed rooms from "a file the room appends to". TEL-REC1 and option R-a use a bounded in-memory buffer written at Ended.
13. (low) **A legacy id differs between R6 and R2.** R6 says R2-BUD-REC6 rests on TL15 "too", but R2-BUD-REC6 cites "legacy CM4", which is the DM_Tri map. The 0.80–1.00 ms figure is TL15's. The wrong id is in R2; R6 should say so.

**Scope checks**
- Security baseline: it appears only as R6-SEC-OQ1, with no findings, options or recommendations.
- Advisor condition A4 (COOP/COEP for every static host considered) is met: Cloudflare A yes (inferred), Netlify B yes, Vercel C yes, GitHub Pages D no, own server E yes.
- Every price carries its currency and a page date (or "no date") plus the read date.
- No look choices.
- Arithmetic re-done:
  - totals: 361.44 / 238.36 / 51.44 EUR (goal) and 17.98 / 101.97 EUR (start)
  - K2 savings: 35.6 % on game VMs, 34.1 % on the total
  - assumptions: A-8 0.62 GB, A-10 12,000 events × 80 B, A-11 13,140 matches
  - nit: 18,431 GB and 13,823 GB use 3.156 GB per hour, not 3.16.

**Label:** M = 40 key claims (HOST 21: F1–F7, F9–F19, F22–F24; COST 5; OBS 12; TEL 2). N = 19. The status label becomes **"spot-checked (19 of 40 key claims)"**.

**Nits only?** No. No source claim is wrong, but items 1–4 are four medium findings (a silent departure from R2, an unsupported comparison, incomplete "Rests on:" lists, a known risk left out of a recommendation). The owner can fix all of them in place without new sources. Suggestion: the owner fixes them, then the changed lines get a short check rather than a full second verifier pass. The caller decides.

**Not verified**
- Findings in M not checked at their source (21): HOST-F2 (Workers half), F4, F6, F7, F9, F10, F11, F18, F19, F23; COST-F3, F4; OBS-F2, F3, F4, F6, F8, F10, F11, F12; TEL-F1.
- Pages the file lists as not read (netlify.com, sentry.io/pricing, letsencrypt.org, GDPR) are outside the allow list and were not attempted.

Notes (ending with the full table and DONE): `/home/user/Shamble-Frontlines/.checkpoints/v2-research/verifier-r6.md`
