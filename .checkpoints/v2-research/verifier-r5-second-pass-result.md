# R5 wave 2 second pass: the verifier's report, verbatim

Workflow run wf_22f1f23c-b99 (script `v2-research-r5-second-pass-run1.js`, prompt `prompts/verifier-r5-second-pass.txt`), returned 2026-10-29 (about 21:49 UTC). Notes: `verifier-r5-second-pass.md`.

VERDICT: PASS WITH NOTES. All ten points of the spot-check's critical read are now fixed, cut or turned into an open question. Every recommendation the fix rewrote follows from its findings. What is left is nits, plus one wording gap the fix introduced in R5-CI-REC5. I edited nothing; my notes are in `/home/user/Shamble-Frontlines/.checkpoints/v2-research/verifier-r5-second-pass.md` (ends with DONE).

Gate: skipped as instructed (this is a research check). Symbol diff: not applicable.

Evidence: I read the diff `9a623a0..HEAD` of `/home/user/Shamble-Frontlines/docs/research/r5-toolchain-quality.md` (294 lines added, 134 removed). I checked the cross-references against r1, r2, r3 and `docs/LEGACY_INVENTORY.md`, and re-checked the legacy lines in `tools/gate.mjs` at HEAD. I opened 9 pages myself with WebFetch, all on the allow list. A script in the scratchpad (`/tmp/claude-0/-home-user-Shamble-Frontlines/ed3ff506-9a45-5a14-adc3-75c262311890/scratchpad/restson.py`) recounted which findings the "Rests on:" lists name.

**Item 1, the critical read: closed. Nits listed under 1 apply to every point.**
1. **Real Safari (fixed).**
   - The finding F8 is at :450-460. R5-TEST-REC1's Safari row is at :1389 with its reason at :1396-1397; R5-TEST-REC2 is at :1404-1422; the options row is at :1066; R5-XQ-OQ1 is at :1859-1868.
   - Nit: the options row (:1066) and REC2's reason (:1414-1415) credit F6 with "safaridriver" and "macOS". F6's page only says "`webdriverio` supports these browsers:" and lists Safari among them. Only F5 names safaridriver.
2. **The required check could be skipped (fixed)** at :1545-1567; the new pass mark (5) is at :1566-1567.
   - Still open, minor wording: the headline still says "Jobs skip by condition" (:1547), and the reason still says conditional skips count as success (:1556). The body now fails the final job on any skipped dependency (:1559-1560).
   - The recommendation also never says which jobs the final job waits on. The preview deploy job is skipped on fork PRs (R5-PREV-REC1 mark 2, :1586), and the Safari run is conditional (R5-TEST-REC1, :1389).
   - If either is among the jobs the final job waits on, PRs get blocked. That fails safe, so no failing PR can merge. One sentence fixes it: name the jobs the final job waits on and say skipping is off until R5-GATE-OQ1 decides.
3. **"At warn" (fixed)** at :1337-1340 (R5-RUSTLINT-REC3) and :1306-1307 (R5-RUSTLINT-REC1). Nit: REC3's body now cites F1, but its Rests on (:1344) does not list it.
4. **The case against option B (fixed)** at :1671-1677, in the options rows at :1134-1135 and in R5-GATE-OQ1 at :1840-1843. G0 checks the Rust toolchain and `--quick` (G0–G5) includes cargo fmt and clippy, which matches the table at :1637-1639.
5. **§6.2 traces (fixed)** at :1210, :1329, :1355, :1417 and :1658. Each now says §4, §6.6 and §6.8, which matches R1-ECS-REC2 (r1-simulation-core.md:1937-1938).
   - The owner kept §6.2 in R5-LAYOUT-REC5 (:1223), and that reason is sound. R1's option A2 at r1-simulation-core.md:1725 reads "Serves §6.2, §6.9, §4".
6. **Previews would not match production: now an open question,** R5-XQ-OQ3 (:1873-1882), with a pointer from R5-PREV-REC4 (:1614-1615). The owner's reason is sound.
7. **Missing options: now an open question,** R5-TEST-OQ8 (:1791-1796), plus a benchmark row (:1394), a not-read row (:2226) and a row in the hinge table (:1899).
   - Nit: the benchmark row rests on R1-TICK-REC2's bench (r1-simulation-core.md:2948), but REC1's Rests on (:1398) does not name R1-TICK-REC2.
8. **Missing "Rests on" entries (fixed)** at :1162, :1183-1184, :1398, :1438-1439, :1459, :1595-1597 and :1678-1679. CD4, CD6 and CD10–CD14 exist in `docs/LEGACY_INVENTORY.md` (:85, :87, :91-95).
9. **Drift from R2 (fixed)** at :1462-1464. It now equals r2-netcode.md:2657: "App-level RTT agrees with quinn's server-side `rtt()` within ±5 ms at 100 ms injected RTT."
10. **Nits from the first pass, all applied:** :271-272, :274-277, :1482-1483, :1590-1592, :1642, :1644, :1696, :756, :911, :1901. The two "research only" points are fixed at :1662 and :1229-1230.

**Item 2, rewritten recommendations: closed except two nits.** Each now follows from its findings, marks its inferences, and names its basis (apart from the gaps under 1, 3, 7 and the CI-REC5 wording).
- Pass marks: two of R5-TEST-REC1's own parts disagree. Its mark (:1401) says "one planted defect per layer fails that layer", but the new Benchmarks row says "never a gate pass or fail". The mark should exempt the benchmark layer.
- No other two marks contradict. CI-REC5's mark 1 and its new mark 5 both hold while every check runs (R5-GATE-OQ1).

**Item 3, numbers: closed.**
- ±5 ms at 100 ms comes from R2 (r2-netcode.md:2657).
- The 200 KB figure is gone (:1696).
- I redid the counts:
  - Open questions: 45 = 29 + 13 + 3.
  - Recommendations: 36 = 26 + 10.
  - Sources not read: 39 rows = 25 + 10 + 4 (24 before).
  - Key claims: 115 = 117 − 2, which the script confirms at HEAD (R5-CI-F8 and R5-PREV-F16 are the two no list names).
  - The part 7 table has 48 rows holding 59 distinct ids, plus R5-CI-F10 = 60.
  - Fix results: 6 fixed + 54 unchanged = 60.

**Item 4, new open questions: open (nit).**
- R5-TEST-OQ8 (:1791-1792) says cargo-nextest is "said to run each test in its own process". That is a new fact with no source, even though it is marked unverified. Neither the nextest GitHub page nor its docs.rs page says it. Reword it to "how nextest isolates tests is unread", or cut it.
- R5-XQ-OQ3 (:1880) states an answer ("should carry the header set production will ship") while naming engine-architect as the one who decides. Make it one option among others.
- R5-XQ-OQ1 and R5-GATE-OQ1 add no new facts.

**Item 5, header, totals and part 7: they match the file.** One nit: the date line (:10) and the counts line (:2274) say every source was read 2026-10-28, but the re-read for F22 was on 2026-10-29.

**Claims and numbers checked (id · claim · source or arithmetic · verdict · exact words)**
| id | claim | source | verdict | exact words |
|---|---|---|---|---|
| S1 | F8: Playwright points to WebKit on mac, not to real Safari | playwright.dev/docs/browsers | confirmed | "for the closest-to-Safari experience you should run WebKit on mac, for example if you do video playback."; "Playwright doesn't work with the branded version of Safari since it relies on patches."; "into Apple Safari and other WebKit-based browsers." |
| S2 | R5-TSLINT-F13: when to use `strict` | typescript-eslint.io/users/configs | confirmed | "We recommend a TypeScript project extend from `plugin:@typescript-eslint/strict` only if a nontrivial percentage of its developers are highly proficient in TypeScript." |
| S3 | R5-TSLINT-F16: which config turns the rule on | the no-floating-promises rule page | confirmed | "Extending "plugin:@typescript-eslint/recommended-type-checked" in an ESLint configuration enables this rule." |
| S4 | R5-TSLINT-F22: Prettier's language list | github.com/prettier/prettier | confirmed | "JavaScript · TypeScript · Flow · JSX · JSON" / "CSS · SCSS · Less" / "HTML · Vue · Angular" / "GraphQL · Markdown · YAML"; "Prettier is an opinionated code formatter." |
| S5 | R5-TEST-F2: no seed means a random seed | docs.rs proptest 1.12.0 Config | confirmed | "If the environment variable is undefined, a random seed is generated (this is the default option)." |
| S6 | R5-CI-F2: the cache key includes the manifests | github.com/Swatinem/rust-cache | confirmed | "a hash of the workspace's Cargo.lock and of the Cargo.toml of the workspace root and every workspace member (if present)." |
| S7 | F6 says WebdriverIO reaches Safari through safaridriver on macOS | vitest.dev/guide/browser (v5.0.5) | unsupported | only "`webdriverio` supports these browsers:" with Safari in the list; no sentence mentions safaridriver or macOS |
| S8 | R5-TEST-OQ8: nextest runs each test in its own process | github.com/nextest-rs/nextest, docs.rs/crate/cargo-nextest | unsupported | only "A next-generation test runner for Rust." and "a new, faster Cargo test runner." |
| S9 | ±5 ms at 100 ms (R5-TEST-REC4) | r2-netcode.md:2657 | confirmed | "within ±5 ms at 100 ms injected RTT. Proposed." |
| S10 | the A2 Worker serves §6.2, §6.9, §4 | r1-simulation-core.md:1725 | confirmed | "Serves §6.2, §6.9, §4" |
| S11 | R1-ECS-REC2 traces determinism to §4, §6.6, §6.8 | r1-simulation-core.md:1937-1938 | confirmed | "Serves VISION_V2 §4 … and §6.6 and §6.8" |
| S12 | CI gates counts, not timings | r3-rendering.md:1197-1200 | confirmed | "CI (W-A) gates only hardware-independent numbers … frame times and GPU times come only from the reference machines" |
| S13 | ESLint 10 is current | the file's own F14 (:233) and F20 (:263) | confirmed | "latest is v10.13.0" |
| S14 | the legacy gate lines in R5-GATE-REC2 | tools/gate.mjs at HEAD | confirmed | :37 exits with 2; :194 `const ORDER`; :213 exit codes 0, 1 and 2; :372 "[5a] smoke (slowest, so last)" |
| A1–A6 | 45, 36, 39, 115, 60, 54 + 6 | redone arithmetic (item 3) | correct | — |

**Counts:** 13 confirmed, 0 wrong, 2 unsupported (S7 and S8, both in wording the fix added and already hedged), plus 6 numbers redone and correct.

**Only nits?** Yes. Nothing left changes what a recommendation recommends. The largest item is the R5-CI-REC5 wording (its headline and the jobs the final job waits on), a one-sentence fix. The rest:
- F6 credited with safaridriver
- F1 missing from R5-RUSTLINT-REC3's Rests on
- R1-TICK-REC2 missing from R5-TEST-REC1's Rests on
- REC1's mark versus its Benchmarks row
- the unsourced nextest sentence in R5-TEST-OQ8
- the stated answer in R5-XQ-OQ3
- the 2026-10-29 re-read date

**Not verified:** whether ESLint 10.x removed the formatting rules (the file marks it unverified). The `if: always()` and `needs.<job>.result` syntax in R5-CI-REC5 is marked inferred; I did not open GitHub's workflow syntax page for it.

**Header label the file should carry:** "spot-checked (61 of 115 key claims)". R5-TSLINT-F22 is a key claim (it is in R5-TSLINT-REC3's Rests on), and this pass confirmed it at its source. That follows R6's second pass, which added the claims it checked itself. Counting only the first spot-check, it stays "spot-checked (60 of 115 key claims)". Either way, add that the second pass ran on 2026-10-29 and closed with nits only.
