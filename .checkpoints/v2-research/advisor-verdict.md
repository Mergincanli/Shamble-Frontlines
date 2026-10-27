# Advisor verdict on the R1–R6 plan (verbatim)

The advisor agent's plan review in session_017agAXp7koecrVYw8LUufy5, 2026-10-27 11:36 UTC: APPROVE with 14 conditions.
Recovered from that session's transcript (see `provenance.md`); the request that started the review and the
decisions it was judged with (BRANCH-2 (a), SPEND-1 (b)) are in `request-and-decisions.md`.

## 3. The verdict: APPROVE with 14 conditions

The advisor subagent's SubagentHandback call, created_at 2026-10-27T11:36:28.486436Z, event uuid
86416dc2-d870-4529-a0d3-2efca2043802 (it reached the main session as a hand-back at 11:36:41.247765Z, event uuid
ce92208b-f31f-4036-a90c-818a92a03690, with each line indented and a harness note added; the text is otherwise the same).
Verbatim:

````markdown
**Verdict: APPROVE, with the 14 conditions below.** BRANCH-2 (a) and SPEND-1 (b) are in force. Apply the conditions to the plan file before the first launch, so the checkpoint and any later Night queue item point to a plan that matches what runs.

**What I checked myself:**
- advisor.md on main; the plan.
- The brief: §4, §5 and §7.3.
- `docs/research/README.md` (on main its table has no status column).
- VISION_V2 §4 and §6; `.claude/CHECKPOINTS.md`; `.claude/NIGHT_SHIFT.md`.
- `.claude/settings.json`: 62 WebFetch domains, confirmed.
- The agent files' frontmatter: engine-architect and netcode-architect are xhigh, the others high. The verifier may not edit docs. security-auditor's findings go into infra-engineer's R6 file.
- Issue #16 comment 6396928581: DOMAINS-ACCEPT means daytime only until WRITE-GUARD merges, and no research item goes in the queue before that.
- Git state:
  - R7's three files exist only on `claude/bold-faraday-i49gug`, not on main.
  - kit-v3-0 is merged (f2ccc13).
  - `cs/v2-research` and `cs/kit-v3-1` don't exist yet.
- STATUS on main was last updated 2026-10-17 and still says the next work is N1. The v2 pivot, kit-v3-0 and R7 are not in it.

**Fit:** It fits the queue (the Advisor's order: R7 merges, then R1–R6 by day) and the brief. It is docs only: no engine code, no spike, no look decision, symbols +0 −0. Its day/night rule matches DOMAINS-ACCEPT.

**Your reading of SPEND-1: confirmed, with these corrections.**
- **R1 and R2 per-question units: yes, but merge two coupled pairs.**
  - R1: merge "the Rust ↔ TypeScript boundary" with "the simulation in a Web Worker". Shared memory between the worker and the page is the boundary question, and its cross-origin-isolation headers also touch R6. That gives R1 6 units.
  - R2: fold "lessons from shipped games" into the snapshots / prediction / interpolation / lag compensation unit. The brief names exactly those sources for that question. That gives R2 5 units.
  - This saves two xhigh agents and two places where drafts must be stitched together.
- **Verifier on every claim a recommendation rests on, for R1 and R2:** yes.
- **rust-reviewer on R1:** yes. The same call also covers R2's server-stack section (the Rust WebTransport and WebSocket crates are a top risk). R4 losing rust-reviewer is acceptable if its crate claims are among its spot-checks.
- **security-auditor on R2's protocol:** yes.
- **Second verifier pass:** yes, but only when a fix changed a recommendation, a number or a source. Nit fixes don't trigger one.
- **R3–R6, one owner agent per topic:** yes.
  - infra-engineer writes R5's CI and preview-link part; security-auditor writes R6's security baseline.
  - Each helper writes a draft in its notes and the owner assembles the topic file, because two agents never edit one file.
  - These run as ordinary subagents, not ultracode. A cut-off agent is restarted with the same prompt and carries on from its notes (there is no `resumeFromRunId` for plain agents).
- **R3–R6 verification, correction:** SPEND-1 asks for "one skeptic pass plus spot-checks". So the single verifier pass also reads the whole file critically: recommendations that don't follow from the findings, missing options, features not traced to a §6 need, and look choices in R3. It spot-checks at least one basis claim per recommendation and at least 10 claims in all, numbers first. The prompt states this scope, because it is narrower than the verifier file's default ("every key claim").
- **Consistency critic:** yes, once, after wave 2's files exist. It resolves nothing. Its list goes into the report and into the affected topics' open questions, which adds no claims and needs no re-check.
- **Two waves:** yes. Wave 1 is R1 and R2 (ultracode); wave 2 is R3–R6 (single agents). Check wave 1's tally before launching wave 2: did the files follow the shape, what share of claims was unsupported, what went unread. Wave 2's shape is exactly the night shift's research item type, so it is the natural part to queue after kit-v3-1.
- **Full verification moves to the synthesis:** yes, provided the labels are honest (condition 9).
- **Size:** at most about 37 agents, against about 65 in the plan as written.

**Conditions**
1. **Branches (BRANCH-2):**
   - Create `cs/v2-research` from origin/main only after PR #27 (R7) has merged, and check that `docs/LEGACY_INVENTORY.md` is on it before any unit starts.
   - Work kit-v3-1 in its own `git worktree` on `cs/kit-v3-1`. The research checkout must never switch branch: the saver stops on a branch change, and agents write notes to that checkout by absolute path.
   - If the two collide, kit-v3-1 lands first and research then merges main in. No rebase, no force-push.
   - If the proxy refuses a `cs/` push, stop and list it under Decisions needed. Never fall back to the `claude/` branch.
   - Replace the plan's "Branch" section with this.
2. **Day/night (DOMAINS-ACCEPT):**
   - Until kit-v3-1 is merged, launch or resume research only between 08:30 and 00:10 Amsterdam time.
   - A run still going at 00:10 is stopped; its finished agents stay cached and it resumes after 08:30.
   - No `send_later` or routine wakes the session for research inside the night window, and nothing goes into issue #16.
   - The day kit-v3-1 merges, add the remaining research to #16. Link the committed plan and this verdict, and name `cs/v2-research` and its checkpoint path.
3. **Write-guard stand-in until WRITE-GUARD lands:**
   - After each wave and before each commit, `git status --porcelain` may show changes only under `docs/research/`, `docs/reports/` and `.checkpoints/v2-research/`. Anything else stops the run, uncommitted, and is reported.
   - `git log` shows commits only from the session and the saver.
4. **Model and effort, stated on every agent call:** model opus for all.

   | Effort | Agents |
   |---|---|
   | xhigh | R1 units, assembler and fixer (engine-architect) |
   | xhigh | R2 units, assembler and fixer (netcode-architect) |
   | xhigh | R4 (engine-architect, as its agent file says) |
   | xhigh | the verifier on R2 (netcode) |
   | xhigh | security-auditor on R2's protocol |
   | xhigh | rust-reviewer on R1 plus R2's server stack |
   | xhigh | the critic |
   | high | R3, R5, R6 and their helpers |
   | high | the verifier on R1, R3, R4, R5 and R6 |
   | high | docs-keeper |

5. **Sources:**
   - A recommendation never rests on a source that was not read in this run, or on memory.
   - The report lists, per topic, every source not read, with its URL and the reason: "outside the allow list" or "video only". The Overwatch GDC talks the brief names are likely video only, so say so.
   - Where an unread source matters, the recommendation becomes an open question.
6. **No spikes:** agents write or run no code to test a claim. No cargo, no npm installs, no benchmarks. Put it in every prompt; the agents have Bash.
7. **Pass marks and look:**
   - Pass marks and reference hardware are marked "proposed". Targets come only from VISION_V2 §4 and §6.
   - R3 gives each technique's capability and cost only, with "look brief pending". It cites ART_DIRECTION only as the legacy look and picks no set of techniques.
   - R4's mid-match obstacles trace to §6.4 (big vehicles as moving obstacles). Player building is "don't design it out" only.
8. **Fix step:**
   - Editors are the topic owners; the verifier may not edit docs.
   - No recommendation may rest on a claim whose final verdict is "wrong" or "unsupported". Either re-source it (and re-check it) or turn the recommendation into an open question.
   - "Unverified" is allowed only for claims no recommendation rests on.
9. **Honest verification labels:**
   - Header and README status column: R1 and R2 read "verified (every recommendation claim)"; R3–R6 read "spot-checked (N of M key claims)".
   - The report says R3–R6 depart from brief §4's "every key claim" by SPEND-1.
   - The report's next steps and the checkpoint carry: "the synthesis plan verifies every ADR claim and number, including those from R3–R6".
10. **Checkpoint:**
    - `.checkpoints/v2-research.md` on `cs/v2-research`, with a per-topic table: units, drafts, verification level, run ids.
    - Save the plan, with these conditions applied, and this verdict into `.checkpoints/v2-research/`.
    - Note names follow `<agent>-<part>`: `<agent>-<topic>-<unit>.md`, with drafts as `<agent>-<topic>-<unit>-draft.md` (not `draft-…`).
    - Run the saver (Bash timeout 7200000, restart after 110 minutes), keep a launched copy of each script, and count what came back at each stage (CHECKPOINTS §5 and §7).
11. **STATUS and ROADMAP are not deferred.** docs-keeper updates them in this PR, as protocol step 6 requires; the advisor's PR review checks it. Add the missing v2 pivot, kit-v3-0 and R7 entries, unless kit-v3-1, which lands first, has already added them.
12. **Report:** a coverage table of every brief §4 question, with its section, its recommendation or open question, and its verification level. Plus per-topic source counts by tier and the not-read list.
13. **Risks to add:**
    - Unreadable talks and sources outside the allow list (R2).
    - Two branches worked from one session (handled by worktrees).
    - R3–R6's lighter checking, carried forward to the synthesis.
    Update risk 1's wave description.
14. **Reviewers line:** the verifier (6 topics) · rust-reviewer (R1 plus R2's server stack) · security-auditor (R2's protocol) · the consistency critic · advisor (the PR). Review rounds stop once findings are only nits.

Notes: `/tmp/claude-0/-home-user-Shamble-Frontlines/dbb5e494-b49f-5970-85a2-528a1e15b24b/scratchpad/v2-research/advisor-plan-review.md` (ends with DONE). I posted nothing and edited nothing else.

**What I could not check:**
- Whether the proxy lets this session push `cs/` branches (condition 1 covers a refusal).
- How auto mode treats ask-first rules (condition 3 is the cheap stand-in).

**Decisions needed:** none. BRANCH-2 and SPEND-1 settle the open questions.

**For Mustafa:** nothing. There is nothing for him to check in Brave.
````
