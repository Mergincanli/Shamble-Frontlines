TASK: critic requirement trace kit-v3-0 plan vs V2_PHASE0_BRIEF
step1: read brief (157 lines) + plan (206 lines). next: read current rules files
step2: read NIGHT_SHIFT, advisor.md, agent frontmatter (verifier/docs-keeper sonnet medium; netcode-architect already has Write), settings.json, changeset+advisor-report skills, docs/adr (template only), .gitignore (.checkpoints not ignored).
step3: trace built. Key gaps: plan's own merge (pre-given go) vs brief "Mustafa merges it himself" + CLAUDE.md go-after-approve; Unit Forge adaptation (user says stop); §1 "every engine feature from VISION_V2 §6" missing; NIGHT_SHIFT rule4 "restart cleanly" not explicitly removed; CLAUDE.md "reviewers read-only" line not amended; advisor/docs-keeper no checkpoint writes; "agents build in isolated copies" vs "never source"; source-preference missing; §6 per-step summary; hard constraints "(the legacy game)" relabel = constraint change w/o ADR; night "never delete data" vs checkpoint delete.
DONE
