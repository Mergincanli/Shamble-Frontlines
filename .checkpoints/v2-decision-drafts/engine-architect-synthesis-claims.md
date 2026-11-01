# Claims ledger: the synthesis (ENGINE_OVERVIEW, SPIKE_PLAN, adr/README), engine-architect, 2026-11-01

Plan rule 9. Tiers: **settled** (a STATUS decision code or GLOSSARY §8) · **design doc** (VISION_V2, LOOP_V2,
EDITOR_V2, LOOK_V2) · **draft** (restates an assembled draft at `docs/adr/000N`; that draft's own ledger carries the
research source and tier) · **pass-mark file** (test-engineer's spike entries, plan step 4) · **map** (the decision
map, a checkpoint file) · **counted** (a script count on the files named) · **glossary** (GLOSSARY §1–§7 at 9e7db28).
The synthesis adds no research claim and no number of its own: every number below is a settled input, a glossary
value, a draft's, or a count.

## Numbers
| # | Claim | Where used | Source | Tier |
|---|---|---|---|---|
| 1 | Server tick 30 Hz baseline; 60 Hz only with a 60 Hz margin of at least 25 % | all three | TICK-30 (STATUS:118–120); GLOSSARY §8 (J1 text) | settled |
| 2 | 48 players, the largest mode 24v24 | all three | MODES-3 (a) (STATUS:104–105) | settled |
| 3 | 60 Hz margin: p99 ≤ 6.25 ms against 8.33 ms; one vCPU of RS; stress scene at N = 48 with a snapshot-receiving client per soldier; 30-minute soak; re-flood ticks; R2-BUD-REC6's two shares and R1-TICK-REC2 P1–P3 | SPIKE_PLAN item 2; README TICK-60-HEADROOM row | TICK-60-HEADROOM (advisor-verdicts.md §1 condition 2); GLOSSARY §8; ADR 0001 part 2, OQ-0001-1 (0001:840) | settled (reading to be confirmed at the stop) |
| 4 | The stress scene: 16v16v16 bots in one fight, 15 big vehicles of which 4 very large units, 300 live projectiles, up to 8 active effects per soldier (384 at N = 48), DM_Tri-sized greybox | SPIKE_PLAN fixtures, items 4, 7; overview §6.9 row | GLOSSARY §3 (GLOSSARY:397–437) | glossary |
| 5 | The test-mode list: 24v24, 12v12v12; 16v16v16; 1v1, 4v4, 2v2v2, 6v6v6; N = 64 headroom | SPIKE_PLAN fixtures | GLOSSARY §2 (GLOSSARY:375–394) | glossary |
| 6 | Sizes: S about a day, M 2 to 4 days, L a week or more of session work | SPIKE_PLAN | every test-engineer-passmarks-000N.md, "Spike entries" first line | pass-mark file |
| 7 | Each spike's size (netcode proof test L + M; boundary L + M + S; sim core L; look L + S/M; preview hosting S, ≥ 2 days; content and ability L + S; bench L; asset M; Feel Lab M; foundation L, six change-sets) | SPIKE_PLAN table and items | passmarks-0001:262–302; -0002:246–295; -0003:229–270; -0004:224–297; -0005:239–318; -0006:169–221 | pass-mark file |
| 8 | Pass-mark id ranges per spike (PM-0001-1 to -56; PM-0002-1 to -34, -35 to -69, -70 to -76; PM-0003-1 to -17, -18 to -42, -43; PM-0004-1 to -61, -62 to -66, -67, -68 to -70, -71, -72; PM-0005-1 to -32, -33 to -37, -38 to -44, -45, -46; PM-0006-1 to -10, -11 to -27, -28 to -31) | SPIKE_PLAN | the spike entries above; rows counted in the drafts: 56, 76, 43, 72, 46, 31 (`grep -c "^| PM-000N-"`, max id equal) | counted |
| 9 | The 60 Hz margin run's marks: PM-0001-39, -40; PM-0004-71, -72 | SPIKE_PLAN item 2 | 0001:1134–1135; 0004:1014–1015 | draft |
| 10 | Checklist homes: PM-0001-53 (netcode), PM-0002-34 (spike), foundation checklist (0002:1159), PM-0003-17 (look), bench checklist (0003:1092), PM-0005-17 (content), PM-0005-37 (families), LOD checklist (0005:2123), PM-0005-46 (Feel Lab), PM-0006-9 (hosting), PM-0006-12 and -18 (operations) | SPIKE_PLAN "Who judges what" | grep of the drafts' "####" checklist headings and rows | draft |
| 11 | N = 12, 24, 36, 48 for the page's frame-time comparison | SPIKE_PLAN item 3 | ADR 0002 "What a spike must prove" item 1 (0002:883–890) | draft |
| 12 | 60, 120 and 144 Hz displays | SPIKE_PLAN item 3 | PM-0004-69 (0004:1011); passmarks-0002 dependencies | draft |
| 13 | 8 rooms per VM, two machine types (CCX23, CX33) | SPIKE_PLAN item 1 | ADR 0006 "What a spike must prove" item 2 (0006:738–742); R6-HOST-REC2 via ADR 0001's key (0001:1010) | draft |
| 14 | 24 h after a pull request closes (PREV-DELETE) | SPIKE_PLAN item 6 | ADR 0006 part 3; passmarks-0006 | draft |
| 15 | 25 MiB largest file on the client host | SPIKE_PLAN items 6, 9 | ADR 0006 part 1 (N37: R6-HOST-REC1's mark) | draft |
| 16 | 384 active effects for the ability step | SPIKE_PLAN item 7 | GLOSSARY §3 (N × 8); passmarks-0004 content entry | glossary |
| 17 | "About 60 marks", "seven targets", "three machines" (sim core spike) | SPIKE_PLAN item 4 | passmarks-0004:250–253 | pass-mark file |
| 18 | Three T1 abilities; the Molotov and the Shield | SPIKE_PLAN items 7, 10 | LOOP_V2 §5 (via passmarks-0005); STATUS ABL-PREDICT entry | design doc / settled |
| 19 | 88 open questions (13, 7, 15, 7, 34, 12) | README "The drafts" | script over `- **OQ-000N-…` lines in each draft (scratchpad `oqs.tsv`) | counted |
| 20 | 38 DESIGN ids | README | `engine-architect-decision-map-ids.tsv`, owner column (`awk '$2=="DESIGN"'`) | counted |
| 21 | 28 critic ranks, 72 XT ids; 30 XU ids | README register | id table grouped by "rank N" in the notes column (28 groups); map part f2 (XT 72, XU 30); plan rule 8 (XT 72, XU 30) | counted / map |
| 22 | Line numbers in README rows (OQ lines, "line 161", "line 76") | README | the assembled drafts on 2026-11-01 (grep) | draft (may move in the fix round) |
| 23 | P2: 100 ms round trip with P2's jitter and loss | README "100 ms" row | ADR 0001 OQ-0001-3 (0001:856), part 13 | draft |
| 24 | 118 inventory rows "look brief pending" | README LE-H | map part f3 (LE-H) | map |
| 25 | GLOSSARY.md at 9e7db28: 742 newlines, 64,088 bytes, unterminated line 743 | glossary8 correction 1 | `git show 9e7db28:docs/research/GLOSSARY.md | wc -lc`; `tail -c` | counted |

## Claims (structure, ownership, placement)
| # | Claim | Where used | Source | Tier |
|---|---|---|---|---|
| 31 | One Rust sim, native in each room process and Wasm in one Web Worker; the Worker is the client endpoint online and the local server in local play from slice 1 | overview | brief §1 stack; SLICE-ORDER condition 1; ADR 0002 parts 5–6; ADR 0004 summary (0004:1–12) | settled / draft |
| 32 | The page makes no Wasm call, touches no network; one command per tick up, one transferred block per tick down (B2; B3 measured) | overview | ADR 0002 part 5, spike items 1–3 (0002:883–903) | draft |
| 33 | Crates and dependency rules (`sim` → `content` only; `room` builds for Wasm; `sim-wasm` never `navbake` or `server`; `client` never `editor`) | overview | ADR 0002 part 3 crate table (0002:176–198) | draft |
| 34 | Per-tick flow (inputs, phases, snapshots and events with visibility classes, prediction, interpolation, lag compensation) | overview | ADR 0001 parts 4–9, 11; ADR 0004 parts 4–19 | draft |
| 35 | Per-frame flow on the page | overview | ADR 0003 part 10 (0003:377) | draft |
| 36 | State ownership (authority, Worker, page); production pins one content set per match | overview | ADR 0001 "State ownership" (0001:444); ADR 0005 CD-7 (0005:358) | draft |
| 37 | The §6 table: each requirement's draft and part | overview | map part e1, re-pointed to the drafts' own part numbers (Decision part lists grepped); ADR 0005 AB-6 for §6.4's two primitives (0005:875, :895) | draft / map |
| 38 | The netcode proof test runs before the foundation | SPIKE_PLAN, README | SLICE-ORDER condition 3 (STATUS:101) | settled |
| 39 | The rest of the order is a proposal | SPIKE_PLAN, README | map J6 (5); map part g's proposed order | map |
| 40 | Spikes are throwaway in `spikes/`, exempt from legacy constraints; none before approval | SPIKE_PLAN | SPIKE-SCOPE (a) (STATUS:187); brief §4 | settled |
| 41 | Each spike's "what it proves", dependencies, research subsumed and "decides" lists | SPIKE_PLAN items 1–10 | the six spike entries (passmarks files, "Spike entries"), cross-read with each draft's "What a spike must prove" | pass-mark file / draft |
| 42 | ADR 0004 parts per spike (sim core 2–4, 6–15, 20, 21; content and ability 16, 18; proof test 5; boundary 5, 6, 20) | SPIKE_PLAN | ADR 0004 spike table, claim → part → spike columns (0004:822–843) | draft |
| 43 | ADR 0001 parts per item (1, 10; 2, 3, 6, 11, 14; 2; 4, 7, 8) | SPIKE_PLAN item 1 | 0001:930–994 (items 1–5) | draft |
| 44 | ADR 0003 look proof test parts 1, 3, 4, 5, 11; bench parts 6–9 | SPIKE_PLAN items 5, 8 | 0003:806, 0003:838 | draft |
| 45 | ADR 0006 preview hosting check parts 1–3; operations marks parts 5, 7, 8, 9; optional Sentry marks | SPIKE_PLAN items 1, 3, 6 | 0006:717–769 | draft |
| 46 | R2-SYNC-REC6 mark 1 and R5-LAYOUT-REC4 placed in the sim core spike | SPIKE_PLAN item 4 | map part g (sim core row); passmarks-0004 "Not placed by any draft" | map / pass-mark file |
| 47 | R1-TICK-REC4 in the netcode proof test through PM-0004-67 | SPIKE_PLAN item 1 | map part g; PM-0004-67 (0004:1008) | map / draft |
| 48 | The asset-pipeline research: graphics-engineer, verified like R3, own daytime session, before the asset spike; timing for Mustafa and the Advisor | SPIKE_PLAN, README | ASSET-RESEARCH (a) (advisor-verdicts.md §1 condition 4); map J4 | settled |
| 49 | The Feel Lab test after abilities are playable (slice 4) | SPIKE_PLAN item 10 | ABL-PREDICT; map LE-N, J6 (8); passmarks-0005 Feel Lab entry | settled / map |
| 50 | Decisions table rows (decider per OQ) | README | each draft's OQ "Decider" text (script, `oqs.tsv`); rows with Mustafa or the Advisor kept | draft |
| 51 | WEAPON-UNLOCKS, carry slots, looted ammo, levelling as OQ-0005-AB5, AB1, AB2, AB3 | README | 0005:1095–1148; UPGRADE-SCOPE (inputs file) | draft / settled |
| 52 | The 38 design rows: each id's question and why it is design | README | the id table's notes; each id's question text in r1/r2/r3/r4/r6 (grep of the bold id lines) | map / research text |
| 53 | Loose-ends rows (CF-1..5, the answered codes, XT ranks, OLD-ADR, ADR-001..003, SEC-MDL-2, CDN-SRI, WALL-CLOCK, LE-A..P) and where each is resolved | README | map parts f1–f3 and J2; each owner draft's Appendix A (grep per rank); 0001:161; 0003:140, :1182; 0005:282; 0002:120, :339–340, :773; 0006:215–229 | map / draft |
| 54 | Needs outside VISION_V2 §6, per draft | README | the drafts' "Need:" lines (grep); map J3 | draft / map |
| 55 | Codes the drafts cite (I-1..21, S1/S3/S4/S6/S12, G1–G5/G7–G9/G11/G15, N1/N2/N6/N12/N30/N32/N34/N37/N39–N41/N48, J1–J6, LE-B) | README | map parts a (seams), c (numbers), d (interfaces), h (gaps), j; which codes are cited: grep of the drafts | map / counted |
| 56 | The drafts replace ADR-001..003 | README, overview | STATUS:198; CLAUDE.md "Decisions pending" | settled |
| 57 | The GLOSSARY §8 quotes equal brief lines 52–59 and 60–66 | glossary8 | `diff` of the `>` lines (prefix removed) against `sed -n '52,59p;60,66p'` of the brief: no difference | counted |

## Not verified here
- The drafts' own claims and numbers: their ledgers and the verifier (plan step 7).
- Whether each "Decider" text was parsed whole: the script cut at the first " · " or "What decides it"; rows were read
  by eye, not against every draft's full OQ text.
- The XT rank rows for ranks 2, 4 and 35: the owner's Appendix A grep matched other rows; the part named comes from the
  draft's options sections (0001 Q4, Q7; 0003 part 9), not from the rank's own Appendix A row.
