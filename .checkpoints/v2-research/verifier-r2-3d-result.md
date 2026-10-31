# Step 3d, R2: the verifier's report (verbatim)

Workflow run wf_efa16a80-44a (prompt `prompts/verifier-r2-3d.txt`), launched 2026-10-30 20:39 UTC, returned about 21:04 UTC; the skeptic after it was cut off by the org spend limit at once ("resets 1:10am (UTC)"). Notes: `verifier-r2-3d.md`; hit judgments `verifier-r2-3d-hits.tsv`.

VERDICT: PASS WITH NOTES. R2 keeps the label "verified (every recommendation claim)". What remains is not only nits: 16 uses of a bare "48" and one wrongly attributed quote in part 7 go back to the netcode-architect. All 17 are wording fixes. None changes a number under MODES-3 reading (a).

**Gate:** not run. This is a research check; the main session runs the gate.
- Glossary search, re-run by me on f67245e: R2 has 0 "Retire (exact)" hits and 198 "in context" hits. The header links GLOSSARY.md. My hit rows are identical to `glossary-hits-3c-r2.tsv`.
- All 48 GLOSSARY.md#anchor links in R2 resolve (26 distinct, 0 broken).
- The 19 inserted questions match the approved texts word for word (links ignored): ABL-PREDICT, SEC-SPREAD, MODES-3, BASELINE-WHEN and R2-XT-OQ1 to OQ15.

**Symbol diff:** not applicable (research step). Every approved change for R2 is present (C1.7–C1.14 with C1.10b, C5.8–C5.10, C6.5–C6.7, C9.9–C9.11, C10.4–C10.11, C11.8–C11.12, C17.3–C17.4, C23.1–C23.2). Matches the plan: yes. C17.b2 and the C23.b changes are correctly left out until BASELINE-WHEN is answered. The one OBJECT (r2:870, "one core") was upheld by the advisor and applied as ruled; its reason holds.

## Check 1: every change in the diff
- Of the 7 changes the apply check marked "NOT found verbatim", 6 (C1.9, C1.10b, C11.9, C11.10, C11.12, C17.3) differ from the approved text only by glossary renames, links or "at p99". The seventh, C23.2, differs by "at commit 8dee746", the OBJECT-1 nit.
  - C17.3: "audit gate" became "An advisories check … that gates pull requests". The meaning is kept: the gating form is still the security baseline's call.
- Two additions go beyond the approved text. Both are accurate nits:
  - budgets table E's evidence cell (r2:1725): "weighed and set aside on 2026-10-30 for the server tick budget's 50 % (R2-BUD-REC6)", a record of resolution 1;
  - the pointers to R6 (r2:3500-3502): "(R5-GATE-REC1 schedules the check)", which mirrors C17.3 and matches R5.
- New claim, wrong (r2:3896, part 7): the line "(the design point, VISION_V2 §3's "design for 48+"…)" misattributes a quote.
  - VISION_V2.md contains no "48" at all; the words are CLAUDE.md:8, "Design for 48+ players".
  - Fix for the owner: attribute it to CLAUDE.md, or drop the quotation marks.
  - The older Serves line r2:2289 has the same slip, but it predates step 3c.
- Nits:
  - Part 7 r2:3913 still says "94 nits"; after OBJECT-1 the owner's notes give 97.
  - R2-XU-OQ9 (r2:3195-3197) still says in the present tense that the recommendations "propose" wordings they no longer contain. Unlike XU-OQ8, it has no "before 2026-10-30" framing.
  - A glossary link was added inside the quoted brief-question cell at r2:21 (the words are unchanged).

## Check 2: the recommendation changes
The owner logged 20, a superset of condition 4's R2 items. Each now claims only what its basis supports:
- Every changed number is either resolution-1 arithmetic (8.3, 16.7, 0.8, 1.7 ms, 5 %, 50 %) or an unchanged proposed mark.
- No "Rests on" list changed. No new finding or source.
- The four marks that now say "at p99" (BUD-REC6 encoding, INT-REC1 (c), INT-REC2 (b), INT-REC4) keep their proposed numbers and still hold.
- TRN-REC6 mark 2's change to "Server tick time" holds: the recommendation's own body says "Never block the simulation tick on a send" (r2:2724), and only the server tick time contains the sends.
- Five other time marks with no statistic now read as p99 by definition, with no text change: r2:2130, 2358, 2378, 2778 and 2820. All are proposed and still hold.
  - Nit: ROOM-REC2 mark 5 ("start-up under 2 s") probably meant every start-up. Read as p99 it lets 1 % be slower; say "every" if that was meant.

## Check 3: the tick-budget table (GLOSSARY §1), recomputed from the R1, R2 and R4 files
Every row's value, id and statistic matches its source:
- R4-PHYS-REC1 mark 2: 1 ms mean. R4-NAV-REC3: 0.2 ms p99. R1-ABL-REC6: 1 ms at p99.
- R4-NAV-REC2: 5 % of the period at p99. R2-INT-REC3: 5 % of the period (no statistic, so p99).
- R2-BUD-REC6: 50 µs at p99. R2-SYNC-REC5 mark 4: 20 µs at the 99th percentile, at 700 shots/s.
- R2-INT-REC1 (c): 0.2 ms p99. R2-INT-REC4: 0.5 ms p99. R2-INT-REC2 (b): 0.1 µs per enemy pair p99, re-flood 1 ms p99.
- 768 pairs (R2-INT-F20). Snapshots at 30 Hz (R2-BUD-REC1).

Results:
- **N = 48:** every sum and every "left" figure matches (exact values 3.8667, 3.4101, 7.2768, 9.1768; left 0.300 / 0.757 / −0.243).
- **N = 54 at 60 Hz:** all match. The "0.03 ms left" is exactly 0.025.
- **N = 54 at 30 Hz:** four figures are 0.01 high, because rounded parts were summed. The exact values are 4.11, 5.11, 9.92 and 10.92 against the stated 4.12, 5.12, 9.93 and 10.93. Nit; no conclusion changes.
- **The 25 % option:** 6.44, 5.24, 7.51 and 8.51 all match. "Any snapshot tick" is right, since a 60 Hz tick without a snapshot is 4.04 ms and fits.
- **"Why 50 %" point 1:** every clause holds against the rows.
- Nit, outside R2: GLOSSARY:74-75 and :168 themselves use "whole tick", which the glossary retires exactly. The search script does not read GLOSSARY.md.

## Claims checked

| Id | Claim (short) | Source | Verdict | Location and exact words |
|---|---|---|---|---|
| R6-HOST-F15/REC2 (behind R2-ROOM-REC2, BUD-REC6, TRN-REC1 mark 2) | the CCX23 is 4 dedicated vCPU | hetzner.com general-purpose page | confirmed | "you get dedicated vCPUs"; table header "VCPU \| RAM \| Drives"; row "CCX23 \| 4 \| AMD \| 16 GB \| 160 GB". R6's own verifier row (r6:1094) quoted only prices. |
| R6-HOST-F23 (resolution 11, C11.9) | a vCPU is not always a whole core | aws.amazon.com GameLift pricing | confirmed | "c6a.large (1 core, 2 vCPU, 4 GB) @ $.099 OD /$.03 Spot". R6's verifier left this unchecked (r6:1150). |
| R1-DET-REC4 (C6.5) | the Worker holds the stream offline | r1:2185-2188 | confirmed | "held by the server online and by the Worker in an offline match" |
| R5-GATE-REC1 (C17.3) | it already schedules the advisories check | r5:1671-1674 | confirmed | "Scheduled … cargo-deny advisories … a PR that changes `Cargo.lock` runs the advisories check too" |
| R6-HOST-REC4 (C23.1) | one wildcard key on every game VM | r6:683-688 | confirmed | "one private key sits on every game VM" |
| C23.2 | the game VMs are not ephemeral | r6:424-425 at 8dee746 | confirmed | "G1/G2 keep VMs warm (F17), so they are not ephemeral" |
| C1.13 | the transport runs beside the tick | R2-ROOM-REC2 heading | confirmed | "the sim on its own thread and network I/O on an async runtime" |
| TRN-REC6 mark 2 | sends are part of the tick | r2:2724 | confirmed | "Never block the simulation tick on a send" |
| C1.10 | the "48 clients" quote | r2:2292 | confirmed | "One match server with 48 clients" |
| C1.9 reason and GLOSSARY §1 | the arithmetic | recomputed | confirmed | 0.01 rounding nit at N = 54, 30 Hz |
| r2:3896 | VISION_V2 §3 says "design for 48+" | VISION_V2.md | wrong | the file has no "48"; the words are CLAUDE.md:8 |

Counts: 10 confirmed, 1 wrong, 0 unsupported.

## Check 4: in-context hits
All 198 are judged in `verifier-r2-3d-hits.tsv`: 182 right, 16 wrong.
- **Rule I applied** (the same one the R1+R4 verifier is using): a bare 48 is wrong when it is the single whole-match count in a pass mark or spike, with no marking words and no 54 value, or when GLOSSARY §1 scales that mark at N = 54.
- **The 16 wrong ones, all bare "48" (owner: netcode-architect):**
  - r2:2017 and 2022 (SYNC-REC1 marks 1 and 4)
  - r2:2103 (SYNC-REC3 mark 2)
  - r2:2186 (SYNC-REC5 mark 4)
  - r2:2292 (SYNC-REC9 mark 1)
  - r2:2319, 2338 and 2358 (the BUD-REC1, BUD-REC2 and BUD-REC3 spikes)
  - r2:2439 (INT-REC1 (c))
  - r2:2499 (INT-REC4: "48 clients" inside a mark now set in the stress scene)
  - r2:2588 (TRN-REC1 mark 2)
  - r2:2718 (TRN-REC5 mark 2)
  - r2:2773 and 2777 (ROOM-REC2 marks 1 and 3)
  - r2:2952 (SYNC-OQ13)
  - r2:2993 (BUD-OQ9)
- **Fix for each:** write "the largest mode's N (48 or 54, MODES-3)", or label it "the design point, N = 48". Part 7's claim that every bare "48" means the design point does not hold for these.
- **First uses:** the 25 terms are linked. The 7 the script flags as unlinked are all other meanings, or have the link on the adjacent line.

## Check 5: the label
R2 keeps "verified (every recommendation claim)".
- From part 7: all 135 findings named in "Rests on" lists were checked, then the second pass and the final check ran.
- From the diff: no "Rests on" list changed, there is no new finding or source, and every changed basis claim is now confirmed (table above).

## Host audit
| Tool | Fetches | Requested | HTTP code and final URL |
|---|---|---|---|
| WebFetch | 0 | – | – |
| curl | 1 | https://www.hetzner.com/cloud/general-purpose | 301, same host, Location https://www.hetzner.com/cloud/general-purpose/ |
| curl | 1 | https://www.hetzner.com/cloud/general-purpose/ | 200, same URL |
| curl | 1 | https://aws.amazon.com/gamelift/servers/pricing/ | 200, same URL |

Final hosts: www.hetzner.com and aws.amazon.com, both on the allow list. Downloads were never run; Python ran on them only with `-I`.

**Not verified:** no other source was fetched. Every other changed basis claim rests on files I read or arithmetic I redid.

Files:
- Notes (end with DONE): /home/user/Shamble-Frontlines/.checkpoints/v2-research/verifier-r2-3d.md
- Hit judgments (198 rows): /home/user/Shamble-Frontlines/.checkpoints/v2-research/verifier-r2-3d-hits.tsv
- My re-run of the search script: /tmp/claude-0/-home-user-Shamble-Frontlines/ed3ff506-9a45-5a14-adc3-75c262311890/scratchpad/3d-r2/run/hits.tsv

Nothing was committed.

**Decisions needed:** none.

**For Mustafa:** nothing.
