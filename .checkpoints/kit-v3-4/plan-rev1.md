# kit-v3-4 (A4) plan, revision 1 (2026-11-07): the review policy, plus his answers of 2026-11-07

Branch `claude/sleepy-tesla-ofsxsr` stands in for `cs/kit-v3-4` (draft PR #44); the worktree is from main 69ea88e.

The change-set:
- **Kind:** a kit change-set (rules and agent files), plus a small docs part. No game code, so symbols +0 −0.
- **Freeze:** the kit freeze has lifted, by his UNDER-WAY (a) as the netcode session records it.
- **Mustafa's go:** needed, because the kit changes.
- **Sources:** his words are in `facts.md` §0 (STATUS:544-570); the facts are in `facts.md` §1-§8, with file:line.

## 1. Goal
Write his review policy (A4, 2026-11-04) into the rules, mapped onto today's process, without loosening any rule he has
not asked to loosen:
1. risk tiers;
2. a review budget;
3. a rollback rule;
4. and 6. foundation notes;
5. per-tier checklists.

Also record his answers of 2026-11-07, XP-TERM (a) and LOOK (a), and the advisor's four Low notes on #43.

## 2. The parts
**A. Risk tiers (his item 1).** "Tier" already names game tech tiers and render quality tiers (facts §6), so the rules
say "risk tier 0-3".
- In CLAUDE.md's change-set protocol, one short table, with a longer version in advisor.md:

  | Risk tier | What it covers (his words, then the mapping) | Reviews | Who merges |
  |---|---|---|---|
  | 0 | "docs only": see TIER0-SCOPE | CI + the PR Advisor | the PR Advisor's own pinned merge (T0-MERGE) |
  | 1 | "game code and tools": the game files, `tools/`, spikes | CI + the advisor; the change type's reviewers | the PR Advisor, or his Brave check first when players see, hear or feel it |
  | 2 | "rules, CI, guard, secrets, infrastructure": CLAUDE.md, `.claude/`, `.github/`, the gate files, hosting | the advisor + security-auditor, always | a session, on his go only |
  | 3 | "irreversible or money" | agents only propose | Mustafa decides |

- **Today's classes mapped:**
  - "ready" with no human check becomes tier 0 or 1;
  - `needs-mustafa` becomes tier 1 with a Brave check, tier 2, tier 3, or any PR that records his words or decisions
    (TIER0-SCOPE).
- **Today's four escalation categories (advisor.md:36-43) stay:**
  - category 1, player-facing: tier 1 with the Brave check;
  - category 2, design decisions: his, at any tier;
  - category 3, ADRs and hard constraints: his, at any tier;
  - category 4, deleting data or rewriting history: tier 3.
  - Money joins category 4, as tier 3 (facts §8.5).
- **"Sees, hears or feels" stays.** It is the standing rule, wider than his A4 "see or hear" (facts §8.3).
- **security-auditor joins every tier-2 change** (facts §8.2), on top of today's subject list.

**B. Review budget (his item 2).** In CLAUDE.md step 5, in advisor.md, and in the changeset skill, whose "fix and re-run
until PASS" gains "within the budget":
- **A round** is one review of one head by all of the change-set's reviewers. After each round comes one fix pass.
- **At most two rounds per change-set.** The second round reviews only what changed. A third round happens only for a
  Critical or High finding. The advisor's own plan review and final-head review are not rounds.
- **One severity scale for every reviewer**, defined once in CLAUDE.md:
  - **Critical:** breaks security, data, the guard, or main.
  - **High:** a wrong result or rule that a player, Mustafa or a session would hit.
  - **Medium:** a real defect with a workaround or a small reach.
  - **Low:** polish, wording or a nit.
- **What happens below the cut:** findings below the third-round bar become STATUS follow-ups. A Low finding never
  triggers a push on its own; it rides the next push that is needed anyway, or becomes a follow-up. This is his "Nits
  never trigger a push".
- security-auditor.md's four labels point to this scale (facts §2).

**C. Rollback (his item 3).** In CLAUDE.md, with the steps in advisor.md:
- **"Main breaks" means one of:** `gate` or `smoke` red on main; Mustafa reports a merged change that breaks the game;
  or a merged kit change that stops sessions working.
- **"Revert first":**
  - a session opens a revert PR (`git revert -m 1 <merge>`) on its own branch, with no plan step: a pure revert is the
    one exception to "plan first";
  - the advisor checks that its diff is the exact inverse of the merge, and that CI is green;
  - the merge follows the reverted change's risk tier: the PR Advisor for tiers 0-1, and his go for tiers 2-3 (a kit
    revert touches kit files: guard :1460-1464, facts §3).
- **"Then fix in a new PR"**, through the normal protocol.
- **No guard change:** reverting stays a pull request, since main is protected.

**D. Foundation notes (his items 4 and 6), "as notes, not designs":**
- **`docs/SPIKE_PLAN.md`'s foundation entry (:420-438)**, one note in his words: fast checks on save (format, and type
  checks for Rust and TypeScript), the full gate in CI. ADR 0002 part 7's per-edit hook (0002:472-474) has no format
  check, so that is for the foundation's plan to take up.
- **ADR 0006**, one dated note per place, quoting him, with no design:
  - part 10 (browser error reports; PM-0006-30): "from the first playtests on";
  - part 9 (the server metrics already planned);
  - OQ-0006-14 (server-version rollback): "one-click rollback ... of server versions";
  - the client-deploy rollback, which ADR 0006 has not read (:1011, :1554).
- **Register "bears on it" clauses:** OQ-0006-14, and ADR 0003's client field data, which is planned for slice 6, later
  than his "from the first playtests on" (0003:1245).

**E. Per-tier checklists (his item 5)** in advisor.md, beside "PR review". For each tier: what to check, and what to
ignore. None may contradict the "Never" list or step 0 (the session's note). Drafts:
- **Tier 0:**
  - check: every fact and line number is true; no decision of his is recorded or changed (or, if one is, TIER0-SCOPE);
    CI is green;
  - ignore: wording style, line wraps, Low findings.
- **Tier 1:**
  - check: the gate and the evidence for the change type (CLAUDE.md step 4); symbols; layering; whether a Brave check
    is needed;
  - ignore: Low findings outside the change.
- **Tier 2:**
  - check: security-auditor's review; no rule loosened; the guard's tests; his go;
  - ignore: nothing marked security.
- **Tier 3:**
  - check: nothing merges; the proposal names its cost, its risk and his options;
  - ignore: nothing.

**F. His answers of 2026-11-07 and the four Low notes on #43** (a docs part; DOCS-RIDE):
- **VISION_V2's XP block (:71-75):** a pointer in brackets with his XP-TERM (a) words, word for word.
- **The register's R2-ROOM-OQ7 row:** "Which progression 'XP' means ... stays open" becomes his answer, labelled
  "Mustafa (2026-11-07) answers: ...". Telemetry stays open.
- **STATUS:** the XP-TERM and LOOK follow-ups become answered, in his words.
- **The vfx report:** a dated correction note with the four Low notes (facts §7).

## 3. Files
- **Kit (each Edit or Write asks him in a day session; an agent's edit is refused):** CLAUDE.md,
  `.claude/agents/advisor.md`, `.claude/skills/changeset/SKILL.md` and `.claude/agents/security-auditor.md`.
- **Docs (no ask):** SPIKE_PLAN, ADR 0006, `docs/adr/README.md`, VISION_V2, STATUS, ROADMAP, the vfx report, and this
  pass's report.
- **No guard or settings change, and no `.github/` change.**
- **The PR Advisor routine's stored prompt** (STATUS:774-777) reads advisor.md as its procedure. If its text has to
  change, that is an `update_trigger` on his go, which this plan does not do.

## 4. Build order
1. The docs parts first (F, then D), since they need no ask.
2. Then the kit parts (A, B, C, E) in one sitting while he can answer the asks: one Write per kit file, so four asks.

## 5. Evidence
- **E1:** his words byte for byte: every A4 quote placed, and his XP-TERM and LOOK answers.
- **E2:** a rule diff table. Every changed rule sentence, before and after, with "looser / stricter / same" and why. Only
  his own words loosen anything (for example tier 0's PR-Advisor merge).
- **E3:** the kit's tests are unchanged and pass: `node --test .claude/scripts/*.test.mjs`, with the same counts as on
  main. No guard change.
- **E4:** `node tools/gate.mjs` (CI runs smoke); `repo-map-check.mjs`.
- **E5:** STATUS and ROADMAP are true; #43's merge facts come from the API.
- **E6:** scope: only the files in §3.

## 6. Reviews (one round, at most 3 agents at a time, Opus 5.5 at high)
- **security-auditor** (tier 2: the rules): nothing is loosened beyond his words, and the rollback path keeps main's
  protection.
- **the verifier:** E1-E6.
- **infra-engineer:** parts C and D (reverts through CI; the rollback and telemetry notes against ADR 0006).
- **game-designer,** only if the advisor asks: part F's XP pointer.

## 7. Decisions
- **TIER0-SCOPE (for Mustafa).** Does "Tier 0: docs only" include docs that record his words or decisions? Today
  those carry `needs-mustafa`, as with B.1, B.2 and vfx.
  - (a) No: tier 0 is docs that record no decision of his. Docs with his words or decisions, ADRs and hard
    constraints still need his go. Recommended: his confirmation of placement caught real choices (pillar 5).
  - (b) Yes: every docs-only PR is tier 0, and the PR Advisor merges it; he reads after.
- **T0-MERGE.** His "auto-merge" means:
  - (a) the PR Advisor's own merge, pinned to the approved head. This needs no guard change. Recommended: GitHub
    auto-merge skips the expected-head check (guard :1393).
  - (b) GitHub auto-merge itself. That is a guard change, a tier-2 change of its own; not in A4.
- **DOCS-RIDE.**
  - (a) Part F rides A4: one PR, one go, and the docs build first. Recommended.
  - (b) A small docs change-set before A4.
- **SCALE-HOME.** The severity scale is defined in CLAUDE.md, and the agent files point to it. Recommended over
  repeating it in each file.
- **REVERT-EXCEPTION.** A pure revert needs no plan, and goes into CLAUDE.md's protocol as the one exception.
  Recommended. This is his to confirm at his go: it changes a "Never" line.

## 8. Mustafa's check
His go to merge (tier 2), the four asks during the build, and his answer to TIER0-SCOPE. No Brave check.

## 9. Risks
1. Rewording a rule loosens it by accident. E2's table, security-auditor and the advisor guard against this.
2. The tiers clash with today's classes or "Never" lines. The mapping keeps every category, and E2 lists each line.
3. The kit asks stall the build if he is away. The docs parts go first; the kit sitting waits for him.
4. PR #40 still edits STATUS. B.2 and vfx kept their edits to their own lines; this pass does the same.
