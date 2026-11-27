# Inputs B: the full track's reviewer rule, tested on the Highs of stops 3a and 3b

Written by test-engineer (Opus 5.5) for the change-set "review tracks and who merges", 2026-11-26. Read-only work:
every line below is read from the repository; nothing committed or pushed.

## Part 0. His words this answers, and the effort line of 12 November

- CORE-REVIEWERS (a), his brief of 2026-11-25, 17:28:24 UTC, item 3d (docs/DECISIONS.md:143 and :611): "what "full"
  means for the Rust core, and for netcode and server code in the game: the test-engineer always, plus at most two
  more picked by a rule from what the piece touches. The plan writes that rule. At most two rounds, as today. Show
  whether each High of stops 3a and 3b would have been found or missed under it. The effort of these reviewers stays
  as in my words of 12 November."
- His words of 12 November on reviewer effort: the entry "Taken by Mustafa, 2026-11-12, 15:35:17 UTC: his words of
  2026-11-12, 16:25, on models" (docs/DECISIONS.md:3137, the bullet at :3160), item 3, second bullet of "What I mean by it"
  (checked with grep -n). Later entries: the entry's own pointer of 2026-11-17 (:3143-3148) names the two lines of
  item 3 that changed (Fable as main model, "for the second builder chat only"; the CI line set aside for the first
  builder chat's CI edits), not this bullet; his Builder V4 card (:726) says his model words of 2026-11-12 "hold for
  you unchanged"; his card for the ci-after-3a chat (:1352, "agents on Opus 5.5 at effort high") is scoped to that one
  CI change-set, not to reviewers of game and engine code:
  > - Every other agent runs on Opus 5.5 and may run at effort max. The agents that review, verify or design game and engine code always run at max.
  Beside it, the same day (entry "Taken by Mustafa, 2026-11-12, 16:28:28 UTC", docs/DECISIONS.md:3119-3132, item 3,
  about step 7 of foundation 2.2 only): "No objection to the step-7 reviewers and the verifier on Opus 5.5 at effort
  max." The rule line is the 15:35:17 UTC bullet; the 16:28:28 UTC line is a one-step permission.
  So: the test-engineer and the two picked reviewers run at effort max (Opus 5.5). Both 3a's and 3b's reports say
  "Reviews (Opus 5.5, effort max)" (docs/reports/2026-11-24-cs3a-rules-boundaries.md:142,
  docs/reports/2026-11-25-cs3b-the-loop-closes.md:133).

## Part 1. Every High and Critical of stops 3a (PR #65) and 3b (PR #68)

Sources: docs/reports/2026-11-24-cs3a-rules-boundaries.md:142-188 (merge 28254a73), notes at 6491b839
(`.checkpoints/cs3a-rules-boundaries/`); docs/reports/2026-11-25-cs3b-the-loop-closes.md:133-174 (merge 3ab9eaec),
notes at 80a51e42 (`.checkpoints/cs3b-loop-closes/`). Read with `git show <sha>:<path>` only. No Critical was raised in
either stop (every verdict line read: 0 Critical).

Three kinds, because the rule only changes the first:
- (R) raised by a reviewer in a review round on the built code: the only kind the rule can drop;
- (H) raised by the advisor agent in its review of a pushed head: "not rounds, never cut" (CLAUDE.md review budget);
- (P) raised by the advisor agent in the plan review: kept by both tracks (LIGHT-DEF (a) keeps it too).

The cs3 plan was one plan for both stops: its plan-review notes are byte-identical in both notes folders
(`advisor-cs3-plan.md` at 6491b839 and at 80a51e42, compared with diff: same), so its Highs are listed once.

### Stop 3a (PR #65)

| Id | Kind | What it was (one line) | Found by | Files / areas |
|---|---|---|---|---|
| 3A-H1 F-MULADD | R, round 1 | `f32::mul_add` gives 0x007FFFFF natively and 0x00800000 in Wasm for one subnormal halfway case; the 10,000 committed vectors held no double-rounding case, so "bit for bit" did not hold (test-engineer-review1.md:26-27, :39) | test-engineer | crates/sim (ADR 0004 part 7 list, rules scan), crates/devtools (vectors, make_f32_vectors.py, rules/f32_ops.rs) |
| 3A-H2 F-FEATURES | R, round 1 | `simFeatureList` missed PM-0004-74's own planted bite (an extra feature on a shared dependency under `server`); the unit fixture was unlike cargo's real output (test-engineer-review1.md:24, :39) | test-engineer (engine-architect raised the same compare as a Medium, report :147-148) | packages/client/scripts (build-checks.mjs, build-checks.test.mjs) |
| 3A-H3 DEV-SERVER | R, round 2 (round 1: a Low, L-DEVSERVER) | the dev server still served the test-only Wasm module through npm's workspace link in `node_modules`; the claimed 403 did not hold (security-auditor-review2.md:115, :126-128) | security-auditor (engine-architect concurred in round 2 after reading Vite's code, engine-architect-review2.md:30-35) | packages/client (vite.config.mjs, scripts/build.mjs), packages/e2e (isolation.spec.ts), .gitignore |
| 3A-A | H, head c5f6a85a | four places put the register's wording in quotation marks as Mustafa's own (report twice, a comment in crates/sim/src/rng.rs, the PR text) (advisor-head.md:34, :54) | advisor agent | docs/reports, crates/sim/src/rng.rs (comment), PR text |

### Stop 3b (PR #68)

| Id | Kind | What it was (one line) | Found by | Files / areas |
|---|---|---|---|---|
| 3B-H1 BITE-WHOLE | R, round 1 | PM-0002-12's second bite planted whole (stale post counted but taken) passed every line of presses.spec.ts: the mark's quantities could not see it (test-engineer-review1.md:33; report :137-138) | test-engineer | packages/sim-worker (host.ts), crates/sim-wasm (local_match.rs), packages/client (worker-link.ts, press-tally.ts), packages/e2e (presses.spec.ts, loop.spec.ts) |
| 3B-HC2 RELEASES | R, round 1, "High candidate" | releases are worked out from presses and held bits, not counted, so the releases line could not fail on its own; ruled in round 2 "Medium, High if the report calls the release half met" (test-engineer-review1.md:30; review2.md:64) | test-engineer | packages/client (press-tally.ts), packages/e2e (presses.spec.ts, loop.spec.ts) |
| 3B-HC3 ALLOC-FPS | R, round 1, "High candidate" | the allocation limit held only at 3-5 blocks per frame; a clean page at desktop frame rates read 742-797 B per block against 680: a test that depends on timing (test-engineer-review1.md:35; report :139-140; the cause found by perf-auditor) | test-engineer (cause: perf-auditor) | packages/client (worker-link.ts), packages/e2e (allocation.spec.ts) |
| 3B-F12 | H, head 09b0043d | the PR text and a report heading called the presses mark met "by its own method" for releases too (advisor-cs3b-head.md:28, :37) | advisor agent (it applied test-engineer's round-2 ruling) | PR text, docs/reports |

### The shared cs3 plan (plan review, both stops)

`advisor-cs3-plan.md` B12 (both notes folders), verdict REVISE, "High: C1, C3, C4, C5, C6":
| Id | What it was (one line) | Found by |
|---|---|---|
| P-C1 | the random-number question reused his answered code RNG-CRATE and letter (c) for another meaning | advisor agent (plan review) |
| P-C3 | the cut ladders took tests out to fit a number, against his "I do not take tests out to fit a number" | advisor agent (plan review) |
| P-C4 | PM-0002-12 mislabelled (the network press mark PM-0001-38 given its id) and its own page-side run unplanned | advisor agent (plan review) |
| P-C5 | 3a's Wasm rule tests would run in CI without their module: 3a red, unmergeable | advisor agent (plan review) |
| P-C6 | departures from the drafts not asked (DEPART-3 missing) | advisor agent (plan review) |
Not counted as findings: the advisor agent's 3b start review labels its condition C1 "[High]" (the decoder test's bar,
advisor-cs3b-start.md:108): a condition set before the build, not a defect found.

Counts: kind R: 4 Highs (3a: two test-engineer, one security-auditor; 3b: one test-engineer) plus 2 "High
candidates" in 3b (test-engineer; round 2 ruled RELEASES "Medium, High if the report calls the release half met" and
accepted the answer on ALLOC-FPS, so neither stood as a High; the reports count 3 and 1). Kind H: 2 (advisor agent).
Kind P: 5 (advisor agent). Overall: 11 Highs plus 2 candidates. Critical: 0. GitHub: no review, review comment or
issue comment exists on #65 or #68 (REST `pulls/<n>/reviews`, `pulls/<n>/comments`, `issues/<n>/comments`, read
2026-11-26: all empty), so no High hides there.

## Part 2. The rule: which two reviewers join the test-engineer on the full track

Sources for each reviewer's scope: the `description` line of `.claude/agents/<name>.md` (rust-reviewer: "every change
to Rust code, Cargo manifests or the Wasm build"; netcode-architect: "any change to the simulation, server or
protocol"; security-auditor: "networking, the server, message handling, ... dependencies, or build/CI config";
engine-architect: "any change that crosses a module boundary"; perf-auditor: "hot paths, per-frame code"; graphics-
engineer: "rendering, shaders, materials or GPU cost"; infra-engineer: "CI, deployment or hosting"). Crate names
planned but not built yet (`content`, `server`, `supervisor`, `navbake`): ADR 0002 part 3's crate table
(docs/adr/0002-code-structure-build-toolchain.md:184-195).

**R1. The diff.** `git diff --numstat --no-renames <base> <head>`, base = `git merge-base origin/main <head>` for a
live pull request (for a merged one: the merge's first parent). This container's clone is shallow
(`git rev-parse --is-shallow-repository`: true), and `git merge-base` found no base for #65's two parents here: when
there is none the script stops with an error and the session fetches the history it needs (`git fetch --deepen` or
`--unshallow`); it never guesses a base. A file's lines = added + deleted; a binary file
(numstat `-`) counts 1. Tests and generated files count with the area they sit in. Not scored: `docs/`,
`.checkpoints/`, `legacy/`, `tools/` and every other path outside `crates/` and `packages/`; manifests and lockfiles
(`Cargo.toml`, `Cargo.lock`, `package.json`, `package-lock.json`, `npm-shrinkwrap.json`, in any folder), because
DEP-HOLD (.claude/CHECKPOINTS.md:60-69) already brings security-auditor's dependency check on exactly that content,
outside the two picks (both stops have it: `security-auditor-deps.md` in each notes folder).

**R2. One owner per path** (the first prefix in this list that matches):
| Path prefix | Owner |
|---|---|
| `packages/client/scripts/`, `packages/client/vite.config.mjs`, `packages/client/serve.mjs`, `packages/client/serve.test.mjs` | security-auditor (build and serving config) |
| `packages/client/src/render/` | graphics-engineer |
| `packages/client/` (the rest) | perf-auditor (the page's per-frame code) |
| `packages/net/` | netcode-architect |
| `packages/sim-worker/`, `packages/types/` | engine-architect (the Rust ↔ TypeScript boundary) |
| `packages/e2e/` | test-engineer (always in: scores nothing) |
| `packages/` (any other) | engine-architect |
| `crates/protocol/`, `crates/room/`, `crates/client-endpoint/`, `crates/server/` | netcode-architect |
| `crates/supervisor/` | infra-engineer |
| `crates/sim-wasm/` | engine-architect (the bindings: the boundary) |
| `crates/` (the rest: `sim`, `devtools`, `content`, `navbake`, any new crate) | rust-reviewer |

**R3. The security slot.** If any changed file starts with `crates/protocol/`, `crates/room/`, `crates/server/`,
`crates/supervisor/`, `packages/net/`, `packages/client/scripts/`, `packages/client/vite.config.mjs` or
`packages/client/serve.mjs`, security-auditor takes the first slot. Why, from rules that already exist, not from the
results below: CLAUDE.md step 5 names security-auditor for "network, server, storage, dynamic DOM, deps, CI"; his
exceptions name "anything anti-cheat", and the room's command checks are the anti-cheat line (crates/room/src/lib.rs:
the press check, the lead cap); the decoders take bytes from the link (crates/protocol); the build and the dev and
small servers decide what a browser is handed (security-auditor's own scope: "build/CI config").

**R4. The other slot(s).** Score each reviewer by the lines of the paths it owns (R2); fill the free slot(s) by the
highest score; a tie goes by the fixed order netcode-architect, security-auditor, rust-reviewer, engine-architect,
perf-auditor, graphics-engineer, infra-engineer. A slot stays empty when no other reviewer scores above 0 ("at most
two").

**R5. When.** The picks are computed on the head given to round 1, over the whole pull request, and round 2 is read by
the same reviewers. If round 2's head computes another set, the new pick reads the whole pull request in round 2 and
the dropped one still closes its own open findings. A full-track pull request with no path under `crates/` or
`packages/` (a spike, e.g. `spikes/netcode-proof/`) scores no one: its `.rs` lines count for rust-reviewer, and any
other empty slot stays empty and is said in the report (an open point: a `spikes/<name>/` row when one lands). A rule file in the same pull request makes it tier 2 (the fifth
exception): the advisor agent and security-auditor by the risk table, unchanged.

**R6. Effort.** test-engineer and both picks on Opus 5.5 at effort max (his line of 12 November, Part 0).

The script (kept in the session's scratchpad as `pick.mjs`; the block below was cut out of this file and run as it
stands, 2026-11-26: the same output as the table under it):
```js
// pick.mjs <base> <head> [--no-trigger]: the full track's two extra reviewers (draft rule).
import { execFileSync } from 'node:child_process';
const [base, head, flag] = process.argv.slice(2);
const noTrigger = flag === '--no-trigger';
const out = execFileSync('git', ['diff', '--numstat', '--no-renames', base, head], { encoding: 'utf8' });
const OWNER = [
  ['packages/client/scripts/', 'security-auditor'], ['packages/client/vite.config.mjs', 'security-auditor'],
  ['packages/client/serve.mjs', 'security-auditor'], ['packages/client/serve.test.mjs', 'security-auditor'],
  ['packages/client/src/render/', 'graphics-engineer'], ['packages/client/', 'perf-auditor'],
  ['packages/net/', 'netcode-architect'], ['packages/sim-worker/', 'engine-architect'],
  ['packages/types/', 'engine-architect'], ['packages/e2e/', 'test-engineer'], ['packages/', 'engine-architect'],
  ['crates/protocol/', 'netcode-architect'], ['crates/room/', 'netcode-architect'],
  ['crates/client-endpoint/', 'netcode-architect'], ['crates/server/', 'netcode-architect'],
  ['crates/supervisor/', 'infra-engineer'], ['crates/sim-wasm/', 'engine-architect'], ['crates/', 'rust-reviewer'],
];
const TRIGGER = ['crates/protocol/', 'crates/room/', 'crates/server/', 'crates/supervisor/', 'packages/net/',
  'packages/client/scripts/', 'packages/client/vite.config.mjs', 'packages/client/serve.mjs'];
const MANIFEST = /(^|\/)(Cargo\.toml|Cargo\.lock|package\.json|package-lock\.json|npm-shrinkwrap\.json)$/;
const PRIORITY = ['netcode-architect', 'security-auditor', 'rust-reviewer', 'engine-architect', 'perf-auditor',
  'graphics-engineer', 'infra-engineer'];
const score = Object.fromEntries(PRIORITY.map((r) => [r, 0]));
let triggered = [];
for (const line of out.trim().split('\n')) {
  const [a, d, path] = line.split('\t');
  if (MANIFEST.test(path)) continue;
  if (TRIGGER.some((p) => path.startsWith(p))) triggered.push(path);
  const own = OWNER.find(([p]) => path.startsWith(p));
  if (!own) continue;
  const n = (a === '-' ? 1 : Number(a) + Number(d));
  if (own[1] in score) score[own[1]] += n;
}
const ranked = PRIORITY.filter((r) => score[r] > 0)
  .sort((x, y) => score[y] - score[x] || PRIORITY.indexOf(x) - PRIORITY.indexOf(y));
let picks = [];
if (!noTrigger && triggered.length) picks.push('security-auditor');
for (const r of ranked) { if (picks.length === 2) break; if (!picks.includes(r)) picks.push(r); }
console.log(JSON.stringify({ base, head, score, triggered: triggered.length, picks: ['test-engineer', ...picks] }));
```

Its output on the two merges (run 2026-11-26, `node pick.mjs M^1 M` and `--no-trigger`):
| Pull request | Scores (lines owned) | Trigger files | Picks with R3 | Picks without R3 (lines only) |
|---|---|---|---|---|
| #65, 3a (28254a73) | rust-reviewer 1,964 · engine-architect 703 · security-auditor 647 · perf-auditor 384 | 5 (build.mjs, build-checks.mjs and its test, serve.mjs, vite.config.mjs) | test-engineer, security-auditor, rust-reviewer | test-engineer, rust-reviewer, engine-architect |
| #68, 3b (3ab9eaec) | netcode-architect 4,515 · engine-architect 1,238 · perf-auditor 947 · rust-reviewer 411 · security-auditor 8 | 14 (12 in protocol and room; 2 build-checks files in packages/client/scripts) | test-engineer, security-auditor, netcode-architect | test-engineer, netcode-architect, engine-architect |
Who actually reviewed: 3a five (test-engineer, rust-reviewer, security-auditor, engine-architect, perf-auditor);
3b six (those five and netcode-architect).

## Part 3. Each High under the rule

Kind R (the only kind the rule changes):
| Id | Found by then | Picked under the rule (with R3)? | Verdict with R3 | Without R3 |
|---|---|---|---|---|
| 3A-H1 F-MULADD | test-engineer | always | found | found |
| 3A-H2 F-FEATURES | test-engineer | always | found | found |
| 3A-H3 DEV-SERVER | security-auditor (a Low in round 1, raised to High in round 2 by the same reviewer: found only because R5 keeps the same reviewers for round 2) | yes (R3: vite.config.mjs, build.mjs) | found | missed (engine-architect, picked instead, saw nothing of it in round 1 and concurred in round 2 only after security-auditor's measurement, engine-architect-review2.md:30) |
| 3B-H1 BITE-WHOLE | test-engineer | always | found | found |
| 3B-HC2 RELEASES | test-engineer | always | found | found |
| 3B-HC3 ALLOC-FPS | test-engineer (the deeper cause, a Low, by perf-auditor) | always; perf-auditor not picked | found (the timing gap and its fix, refusing runs outside the measured rates, were the test-engineer's; perf-auditor's cause, optimising-compiler bytes charged to the listener's frame, perf-auditor-review2.md:42, would be lost: a Low) | found |

Kind H and P (the advisor agent, kept by both tracks, not a pick): 3A-A, 3B-F12, P-C1, P-C3, P-C4, P-C5, P-C6: found.

**Counts.** With R3: kind R 4 of 4 Highs found and both candidates found, 0 likely found, 0 missed; all 11 Highs
plus 2 candidates found (4 + 2 by the picks, 7 by the advisor agent's plan and head reviews). Without R3: kind R 3
found, 1 missed (3A-H3), both candidates found. No Critical in either stop.

**How much this proves: little.** 3 of the 4 kind-R Highs and both candidates came from the test-engineer, who is
always in, and 7 of the 11 Highs from the advisor agent, which no rule touches. Only 3A-H3 tells two rules apart. A clean score here shows the rule
loses nothing on these two stops; it does not show the picks are good on the next ten. A third and better test: run
the script on #58, #61, #64 and 4a's pull request when it merges, and count their Highs the same way.

## Part 4. What the full track must keep that the rule does not cover

1. **The advisor agent's plan review and its review of every pushed head.** 7 of the 13 Highs came from them (5 in
   the plan, 2 in the head text). LIGHT-DEF (a) keeps the plan read; the full track must not be lighter than light.
2. **The verifier on the final result** (light keeps it). In 3b its own plants found two test gaps no reviewer had
   (the focus-loss listener untested; "allocates nothing" counted for honest ticks only, verifier-cs3b.md:57).
3. **The third round for an open Critical or High.** "At most two rounds, as today": today's rule (CLAUDE.md review
   budget) allows a third round only for an open Critical or High, and 3a used it: round 3, security-auditor alone,
   closed 3A-H3. Write the clause into the full track or it reads as a hard two.
4. **security-auditor's mandate.** CLAUDE.md step 5 names it for network, server, storage, dynamic DOM, deps, CI and
   every tier-2 change. R3 covers network and server code and the build; DEP-HOLD covers deps; the risk table covers
   tier 2. Not covered: dynamic DOM in `packages/client/src` riding in a full-track pull request. A deterministic
   addition if wanted: R3 also fires when the diff adds `innerHTML`, `outerHTML`, `insertAdjacentHTML` or
   `document.write` under `packages/`.
5. **What the picks drop below High, by name.** 3a: engine-architect (one Medium, the same feature compare as 3A-H2;
   five Lows) and perf-auditor (two Lows). 3b: rust-reviewer on a 5,700-line Rust pull request (eight Lows, and its
   1-million-tick check that the block hashes the same natively and in Wasm), engine-architect (Medium EA-1: the
   block writer and the newest post lived in the Wasm wrapper; fixed) and perf-auditor (the allocation cause, a
   Low). Corrected reading of the 3b trade-off (measured): on 3b R3 displaces engine-architect (without R3 the picks
   are netcode-architect and engine-architect); rust-reviewer is out under both variants, because R2 gives the
   netcode crates' lines to netcode-architect alone (rust-reviewer scores 411, fourth). Said plainly: under R2,
   rust-reviewer never reads a protocol, room or client-endpoint pull request. The alternative, every `crates/` line
   also counted for rust-reviewer (the same script with that one line added, run 2026-11-26): 3a unchanged (security-
   auditor, rust-reviewer 2,129); 3b rust-reviewer 5,673 above netcode-architect 4,515, so with R3 the picks are
   security-auditor and rust-reviewer and netcode-architect loses its slot on a protocol pull request; without R3,
   rust-reviewer and netcode-architect, and security-auditor is out. With at most two slots, a protocol pull request
   gets two of netcode-architect, security-auditor and rust-reviewer, never three: an open choice for the plan (my
   recommendation: R2 as written, netcode-architect plus security-auditor, because the room's command checks are
   anti-cheat, his named exception, 3b's security finding was of that class (no bound on how late a command may run,
   Medium, 3B-LATE-BOUND), and the cross-target check rust-reviewer ran is in the test-engineer's own area: the same
   inputs, the same state hash on Wasm and native; Rust-language review then rests on the lint and format checks and
   the verifier).
6. **netcode-architect at xhigh.** CLAUDE.md's "Models and effort" asks xhigh for netcode; his 12 November line asks
   max for every reviewer of game and engine code, which is at least that. Nothing to change; say it once.
7. **"In doubt it is the full track"** (his brief, item 3a): the rule has a default for every path under `crates/` and
   `packages/` (R2's last rows); a path outside both scores nothing and never lowers the track (the spike case in R5).

## Not verified
- The rule on pull requests other than #65 and #68 (#58, #61, #64, 4a's) and on a pull request with `packages/net/`,
  `crates/server/` or `packages/client/src/render/` (none exists yet).
- The reviewers' notes were read for their verdict and High lines (every notes file of both folders, both resume
  cards uncapped); Medium and Low lines were taken from the reports, not each notes file.
- One shell edit of this file was refused by the guard (a sed whose pattern it could not resolve) and redone with
  node and the path spelled out; same file only; `git status` stayed clean (`.gate/` is ignored).
