# Approach sketch for the step-2.3 plan (before the advisor tool's moment-1 consultation)

## The cut (first part = built before the proof tests report; each a separate PR from this branch, in order)
- CS-A docs (tier 1, decision docs): STATUS/ROADMAP true for main 4460ef5 (not in on main: STATUS:29 #52 in flight,
  ROADMAP:117 "PR #52: in review"); his words of 2026-11-14 (the #52 go with rules 2 and 3, and today's start message)
  into DECISIONS.md by script; his go on this plan and his answers to its decisions. Skipped in part if a docs chat on
  main carries them first.
- CS-B kit rules text (tier 2; CLAUDE.md, agent files, .claude/CHECKPOINTS.md; no code): MODELS-2026-11-12;
  ADVISOR-TOOL-WORDS (+ moment-5 ordering line, read the model from the records); the three 2.1/2.2 exception lines name
  his 2026-11-13 decision; MANUAL-MODE; GIT-C (CHECKPOINTS:221); L-4 (setup line uses the allowed playwright cli path);
  SD-N1, SD-N2; status-diet's STATUS citations (advisor.md :17 :286, netcode-architect.md :24); a guard-readable-shell
  line in the agent paste block (PROMPTS-2.3's route). NOT the required-check lines (they move with the switch, CS-F).
- CS-C1 legacy gate findings (tier 2; tools/gate.mjs, tools/smoke.mjs): IE-1, TE-2, TE-7, R2-1, F4 (stray pages: root
  only, so spikes/ and packages/ html stay legal), TOOLKIT-RESOLVES-UP; A-1, N2, the v2-root-page limit recorded as
  known limits in tools/README.md.
- CS-D guard (tier 2; routine-guard.mjs + tests): G-1 PIN-GUARD (root-anchored pins and gate configs), G-2 the gate v2
  folder protected (protectedRel + mergeAllowed), G-3 AUTO-ASK (deny kit/gate edits in auto/bypass modes, only if the
  hook input carries permission_mode; missing field -> today's ask), plus the settings allow rule for the gate v2
  command (looser; asked; three lines). Lands BEFORE the gate v2's code, so its files are protected from their first
  commit.
- CS-C gate v2, first part (tier 2; tools/gate-v2/, configs, root devDependencies): conventions (base ref, exits 0/1/2,
  --json, --allow-removed, --quick, --prove, --only, --list); G0, G1, G2 (minus the float ban), G3 (minus the
  generated-union mutation), G5 (minus the same-feature check), G6 (native release + wasm32 library build; no Vite),
  G9 (cargo test + Vitest); --prove with a mutation and a control per built check; TSCONFIG, CRATE-NAMES,
  CI-ENFORCED-LATER (lint-table opt-in, exact npm versions), LINT-TABLE-PEDANTIC (route via the gate's command line),
  ROOT-WORKSPACE (exclude .gate, .checkpoints). Tools run by path, never package scripts (the deny list).
- CS-E CI (tier 2; .github/workflows): one workflow replacing gate.yml and toolchain.yml: legacy `gate` and `smoke`
  kept by name, v2 jobs calling `node tools/gate-v2/gate.mjs --only ...` inside the network cut, and one aggregate job
  (proposed name `required`, needs all, if: always(), compares ran check ids with --list). CUT-CARRY (keep the cut),
  SETPRIV-HARDEN, CUT-ESCAPE (boundary design), SETUP-NODE-CACHE, CANCEL-ON-MAIN, IE-5, rust-cache from main only.
- [his ruleset: add `required` (source GitHub Actions) after CS-E's first green run on main; keep gate+smoke until CS-F]
- CS-F the switch (tier 2; guard mergeCheck + tests, CLAUDE.md :119 :130, advisor.md :157 :163 :224 :273 :294 :303
  :323): `required` named as the required check; then he drops gate+smoke from the ruleset. KIT-FREEZE-2 leaves
  "Queued" here (if FREEZE-END (a)).
- Later parts of 2.3 (each joins when its result is in, marks unchanged): G11 + PM-0002-49/-50 after the netcode proof
  test; G10 + PM-0002-48, G2's float ban, G5's same-feature check (+FEATURE-UNIFY) after the sim core spike (and 2.5's
  first recording for G10); G4, G9's boundary layer + Wasm tests, G7 + budget file, PM-0002-60, WASM-ARTIFACT with 2.4
  (boundary spike); G6's Vite, G8, G12, PM-0002-39's missing-browser mutation, TWO-PLAYWRIGHTS with 2.5; G13 at slice
  2. PM-0002-62 is phase 1's kit change-set (not the foundation). PM-0002-58 / D5 (fork PR) moves unchanged to 2.6,
  where the first secret enters.

## Items marked later or dropped (lean)
- Later, the kit queue after 2.3's first part (KIT-FREEZE-2 over): FU-P4a and FU-P4c first (Medium), then FU-P3a,
  FU-P4b (his call), FU-P4d, L12, L14, L9/SA-9, L13, L1's runners, A-4, A3's cuts and plan readers; PX-1 + its two
  test gaps (his PX1-ROUTE (a): its own CS after the gate step), W1-W3 and the Windows-name limits with it; F2 (its own
  CS); F5 (needs his words); F-1, CHECKPOINTS §7 cap line, E9's cases (a CHECKPOINTS pass); KIT-ALIGN; the kit-v3-1
  Windows test; SD-N6 (his call, with PX-1's CS); NODE-26 (with the Node>=24.16/Playwright<1.60 hang constraint);
  CARGO-NET-OFFLINE and OVERFLOW-PER-CRATE (first third-party crate); CLIENT-NET-RULE (drafts' next revision);
  CRATE-LICENCE (with a licence check, if ADR 0006 adopts one); EMPTY-SLOTS, PACKAGE-ENTRIES, OQ-0002-8-TEST,
  OVERFLOW-PANIC-RULE (their own triggers).
- Dropped (why): REFUSED-PUSH (the classifier, not a kit rule; plain push is the route); SAVER-SKIP-CI (fails closed:
  a [skip ci] head has no required check; CHECKPOINTS §9 already makes the last commit by hand); SD-N3 (unenforceable:
  one GitHub user; expected head still fails a stale go); SD-L3 (nothing lost); CI-ENFORCED-LATER's "has a test" (no
  ADR mark asks it).
- Not a session's edit: BRIEF-STALE and the five "before the foundation" lines (his and the Advisor's brief; the
  drafts' owners at their next revision).

## Design calls to put to the advisor agent (technical) and to him (his rules)
- His: GATE-PATH (item 7: (a) tools/gate-v2/ folder, rec; (b) take tools/gate.mjs, legacy gate moves; (c) root gate/,
  touches accepted part 1's layout); FREEZE-END ((a) when 2.3's first part merges, rec; (b) when the last check joins);
  MODELS-HOW (effort max in the 12 reviewing agents' frontmatter vs callers passing max; the netcode chat "keeps its
  models and efforts as they are" would change once #40 merges main).
- The advisor agent's: OVERFLOW-XCRATE-CI's anchor (test-only dependency now vs move the anchor); which G2 bans need
  the sim core spike's result; CUT-ESCAPE's boundary; the aggregate's name; G3's generated-union mutation timing.

## PR #40 (item 8)
Merge main before its own merge after each CS; conflicts in STATUS/ROADMAP/DECISIONS; nothing in spikes/ changes; the
gate v2 must pass on a local merge with its head (coexistence evidence in CS-C); CS-C1's stray-page check must not hit
spikes/ html; after CS-D, kit edits in auto mode are denied for it too (its W-PR); after CS-E it must merge main so its
head carries `required` before his ruleset switch matters for its merge.
