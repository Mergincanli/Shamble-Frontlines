# infra-engineer, follow-up batch b (ADR 0006), from engine-architect's ADR 0002 notes (the session, about 11:40 Amsterdam)

Every BEFORE was copied by script (whole lines, by line range) from docs/adr/0006-infrastructure-operations.md at
8fa7173 (identical to the branch head for this file) and occurs exactly once there; no two blocks share a line; none
touches "### Pass marks". OQ-0006-13 is narrowed so that one decision has one home: the guard on a rewritten deploy job
is ADR 0002's OQ-0002-9 (Mustafa, on the Advisor's advice); OQ-0006-13 keeps only the unread facts it rests on and the
security review's question 1 (the Advisor, with security-auditor).

### FIX IE-19 (part 3: the retention of CI failure artefacts; lines 170-171 at 8fa7173)
FILE: docs/adr/0006-infrastructure-operations.md
BEFORE:
   requests older than 7 days; build artifacts are kept 7 days, not the default 90 (r5-toolchain-quality.md,
   Recommendations, Question 6, R5-PREV-REC3, lines 1616–1624). The policy is this record's; the close and sweep jobs
AFTER:
   requests older than 7 days; build artifacts are kept 7 days, not the default 90 (r5-toolchain-quality.md,
   Recommendations, Question 6, R5-PREV-REC3, lines 1616–1624). CI failure artefacts (each failure's seed, recording
   and trace, ADR 0002 part 8) are kept 30 days, and each failure's issue carries its seed, so the run can be repeated
   after the artefacts expire (proposed: a nightly failure triaged a week later must still have its trace, and 30 days
   stays far under the default 90 that R5-PREV-REC3 cuts so a public repository keeps little lying around; R5 hands
   this retention here, r5-toolchain-quality.md, Pointers, line 2083). The policy is this record's; the close and sweep
   jobs
END

### FIX IE-14, TE-7 (part 8: the 3-hour soak before each playtest beside the tick bench; lines 343-346 at 8fa7173)
FILE: docs/adr/0006-infrastructure-operations.md
BEFORE:
   - *The tick bench (proposed):* ADR 0002's tick bench runs on one vCPU of the reference server from slice 1, when no
     game server is budgeted. A CCX23 is created per bench session and deleted after it, billed by the hour (R6-HOST-F17,
     r6 lines 164–168) at 0.1386 EUR (R6-HOST-F15), so 20 bench hours a month, a placeholder, cost 20 × 0.1386 =
     2.77 EUR; who creates and deletes it is OQ-0006-17.
AFTER:
   - *The tick bench and the playtest soak (proposed):* ADR 0002's tick bench runs on one vCPU of the reference server
     from slice 1, when no game server is budgeted. A CCX23 is created per bench session and deleted after it, billed
     by the hour (R6-HOST-F17, r6 lines 164–168) at 0.1386 EUR (R6-HOST-F15), so 20 bench hours a month, a placeholder,
     cost 20 × 0.1386 = 2.77 EUR. ADR 0002's 3-hour soak before each playtest takes the same path, with its bot clients
     on a CX33 beside the reference server, never on it (as spike 2's load generators): 3 × (0.1386 + 0.0144) =
     0.46 EUR a playtest. Who creates and deletes these VMs is OQ-0006-17.
END

### FIX SA-1, IE-14 (part 13, B9: the guard is OQ-0002-9, the `production` environment, the soak's VM; lines 487-487 at 8fa7173)
FILE: docs/adr/0006-infrastructure-operations.md
BEFORE:
    | B9 | The tokens CI holds (the client host's deploy tokens, for previews and for production; the Sentry upload token, "a token: Mustafa's secret", r6 line 1082), a multiplayer preview's binary delivery (R6-XT-OQ4), and the Hetzner API token the spikes and the tick bench need (OQ-0006-17) | their scope and handling | each held only by the job that needs it, in a GitHub environment, as R5-PREV-REC1 does for the deploy token (proposed for the others); production deployed only from `main`, in its own environment with its own token; a required reviewer on `preview`; the preview hosting check's token revoked when the check ends; the Hetzner token in a project of its own; all added by Mustafa (below; OQ-0006-13) | 0002 (the jobs); 0006 (the scope) |
AFTER:
    | B9 | The tokens CI holds (the client host's deploy tokens, for previews and for production; the Sentry upload token, "a token: Mustafa's secret", r6 line 1082), a multiplayer preview's binary delivery (R6-XT-OQ4), and the Hetzner API token the spikes, the tick bench and the 3-hour soak before each playtest need (OQ-0006-17) | their scope and handling | each held only by the job that needs it, in a GitHub environment, as R5-PREV-REC1 does for the deploy token (proposed for the others); production deployed only from `main`, in its own environment `production` with a `main`-only deployment-branch rule and its own token (ADR 0002 part 10); a required reviewer on `preview`, the proposed interim of ADR 0002's OQ-0002-9 (what guards a rewritten deploy job; Mustafa, on the Advisor's advice, before the first client-host token); the preview hosting check's token revoked when the check ends; the Hetzner token in a project of its own; all added by Mustafa (below; the scope read is OQ-0006-13) | 0002 (the jobs; OQ-0002-9); 0006 (the scope) |
END

### FIX SA-1, IE-14 (part 13, Secrets: `preview` and `production`; lines 497-498 at 8fa7173)
FILE: docs/adr/0006-infrastructure-operations.md
BEFORE:
    project was not read), held only by the deploy job's environment `preview` (R5-PREV-REC1, r5 lines 1588–1604; ADR
    0002's job). That environment alone does not keep the token from a branch: a `pull_request` run takes its workflow
AFTER:
    project was not read), held only by the deploy jobs' environments, `preview` and `production` (R5-PREV-REC1, r5
    lines 1588–1604; ADR 0002's jobs). The `preview` environment alone does not keep its token from a branch: a
    `pull_request` run takes its workflow
END

### FIX SA-1, IE-14 (part 13, Secrets: (b) and (d); lines 507-513 at 8fa7173)
FILE: docs/adr/0006-infrastructure-operations.md
BEFORE:
    required reviewers approves it", the same finding), so each preview deploy waits for Mustafa's approval, unless
    OQ-0006-13 puts production where the preview token cannot reach it; (c) the preview hosting check's token is revoked
    when the check ends (part 14); (d) the Hetzner API token for the spikes and the tick bench belongs to a Hetzner
    project of its own, never the production project (a project's token presumably reaches all its VMs: not read;
    OQ-0006-17). Whether a session, which posts on GitHub as Mustafa's account (CLAUDE.md), could approve a pending
    deploy, and how a deployment-branch rule treats a pull request's merge ref, were not read: the preview hosting
    check tests the second (What a spike must prove, item 1) and OQ-0006-13 carries the first. *Ids:* BASELINE-WHEN,
AFTER:
    required reviewers approves it", the same finding), so each preview deploy waits for Mustafa's approval: the
    proposed interim of ADR 0002's OQ-0002-9, which decides the guard (Mustafa, on the Advisor's advice, before the
    first client-host token is added) and may instead put production where the preview token cannot reach it; (c) the
    preview hosting check's token is revoked when the check ends (part 14); (d) the Hetzner API token for the spikes,
    the tick bench and the 3-hour soak before each playtest belongs to a Hetzner project of its own, never the
    production project (a project's token presumably reaches all its VMs: not read; OQ-0006-17). Whether a session,
    which posts on GitHub as Mustafa's account (CLAUDE.md), could approve a pending deploy, and how a deployment-branch
    rule treats a pull request's merge ref, were not read: the preview hosting check tests the second (What a spike
    must prove, item 1), and OQ-0002-9 and OQ-0006-13 carry the first. *Ids:* BASELINE-WHEN,
END

### FIX SA-1 (Trade-off: the approvals last until OQ-0002-9 is answered; lines 701-701 at 8fa7173)
FILE: docs/adr/0006-infrastructure-operations.md
BEFORE:
  and until OQ-0006-13 is answered every preview deploy waits for Mustafa's approval (B9).
AFTER:
  and until OQ-0002-9 is answered every preview deploy waits for Mustafa's approval (B9).
END

### FIX SA-1 (Consequences: the approvals last until OQ-0002-9 is answered; lines 718-719 at 8fa7173)
FILE: docs/adr/0006-infrastructure-operations.md
BEFORE:
  lobby still has no place in the budget (OQ-0006-3); until OQ-0006-13 is answered, every preview deploy waits for his
  approval (B9); releases wait on OQ-0006-14.
AFTER:
  lobby still has no place in the budget (OQ-0006-3); until ADR 0002's OQ-0002-9 is answered, every preview deploy
  waits for his approval (B9); releases wait on OQ-0006-14.
END

### FIX IE-4 (OQ-0006-5: ADR 0002 part 10's two shapes for the staging game server; lines 812-815 at 8fa7173)
FILE: docs/adr/0006-infrastructure-operations.md
BEFORE:
    name, so the security baseline rules on its handling. For the staging game server, also: whether it presents that
    trusted certificate on WebTransport too, so multiplayer previews carry no hash and no rotation (part 4); that would
    depart from R2-TRN-REC5's development and preview certificates and from GLOSSARY §5's definition of the staging
    game server, which would follow.
AFTER:
    name, so the security baseline rules on its handling. For the staging game server, ADR 0002 part 10's two shapes:
    (A) it presents that trusted certificate on both transports, so multiplayer previews pin no hash and the delivery
    job hands over only the binary; this departs from R2-TRN-REC5's development and preview certificates and from
    GLOSSARY §5's definition of the staging game server, which would follow; (B) it keeps pinning: the supervisor keeps
    one certificate per VM across restarts and makes the next one ahead, and the preview's file lists both hashes (part
    4's interim position). Part 4 states which one the staging game server presents once this is answered.
END

### FIX SA-1, SA-Q1 (OQ-0006-13: narrowed to the scope read and SA Q1; the guard is OQ-0002-9's; lines 885-900 at 8fa7173)
FILE: docs/adr/0006-infrastructure-operations.md
BEFORE:
- **OQ-0006-13. How is the production player build kept out of the preview deploy token's reach?**
  - *Why it matters:* a `pull_request` run takes its workflow from the pull request's merge commit (R5-PREV-F2, r5
    lines 770–774), so any branch of this repository can rewrite the deploy job and use its token, and the Pages token
    is account-scoped as far as was read (R5-PREV-F10, r5 lines 823–830), so it may deploy production too. The first
    such token arrives with the preview hosting check, months before the security baseline (BASELINE-WHEN (b)).
  - *Options:* (a) a required reviewer on `preview` (part 13's interim position): Mustafa approves each preview
    deploy (R5-PREV-F6, r5 lines 795–803); whether a session, which posts as his account (CLAUDE.md), could approve
    one was not read; (b) production in a second Cloudflare account with its own token, so the preview token cannot
    reach it, and `preview` without a reviewer (not researched; 0 USD if it stays on the Free plan, inferred as in part
    1); (c) a token limited to the preview project, if Cloudflare offers one (not read). Beside them, the security
    review's question 1: whether B9's part of the security baseline is read before the foundation's preview step; that
    review recommends not, since the interim position needs no new research.
  - *Decider:* the Advisor, with security-auditor; Mustafa for a second account and for the approvals (a) asks of him.
  - *What decides it:* a read of GitHub's rules for reviewing pending deployments and of Cloudflare's token scopes,
    before the foundation's preview step; the preview hosting check's B9 marks (What a spike must prove, item 1).
  - *Carries:* no research id (the security review's finding 1 and its question 1).
AFTER:
- **OQ-0006-13. Is B9's part of the security baseline read before the foundation's preview step: who may approve a
  pending deploy, and can a client-host token be narrowed to one project?**
  - *Why it matters:* a `pull_request` run takes its workflow from the pull request's merge commit (R5-PREV-F2, r5
    lines 770–774), so any branch of this repository can rewrite the deploy job and use its token, and the Pages token
    is account-scoped as far as was read (R5-PREV-F10, r5 lines 823–830), so it may deploy production too. What guards
    that path until the baseline (a required reviewer on `preview`, production in a second Cloudflare account, or
    both) is ADR 0002's OQ-0002-9 (Mustafa, on the Advisor's advice). Two facts it rests on were not read: whether a
    session, which posts as Mustafa's account (CLAUDE.md), could approve a pending deploy, and whether Cloudflare
    offers a token limited to one project (which would let the preview token miss production with no second account).
  - *Options:* (a) keep BASELINE-WHEN (b): B9's interim position stands until the baseline, and the preview hosting
    check's B9 marks (What a spike must prove, item 1) are the only evidence before then (the security review's
    question 1 recommends this: it needs no new research); (b) a short read of GitHub's rules for reviewing pending
    deployments and of Cloudflare's token scopes before the foundation's preview step, the rest of the baseline
    unchanged.
  - *Decider:* the Advisor, with security-auditor; its answer feeds OQ-0002-9.
  - *What decides it:* whether the guard OQ-0002-9 picks depends on either unread fact; before the first client-host
    token is added to this repository.
  - *Carries:* no research id (the security review's finding 1 and its question 1).
END

### FIX IE-14, TE-7 (OQ-0006-17: the 3-hour soak's VM too; lines 948-949 at 8fa7173)
FILE: docs/adr/0006-infrastructure-operations.md
BEFORE:
- **OQ-0006-17. How do sessions get VMs for the spikes and the tick bench without holding a secret?**
  - *Why it matters:* the netcode proof test and the tick bench (part 8) create VMs and delete them after each
AFTER:
- **OQ-0006-17. How do sessions get VMs for the spikes, the tick bench and the 3-hour soak before each playtest
  without holding a secret?**
  - *Why it matters:* the netcode proof test, the tick bench and the playtest soak (part 8) create VMs and delete them
    after each
END

### FIX IE-14, TE-7 (Migration step 0: the Hetzner project serves the soaks too; lines 1235-1235 at 8fa7173)
FILE: docs/adr/0006-infrastructure-operations.md
BEFORE:
   of OQ-0006-1 and a Hetzner project of its own for the spikes and the tick bench (OQ-0006-17); a domain, only if
AFTER:
   of OQ-0006-1 and a Hetzner project of its own for the spikes, the tick bench and the playtest soaks (OQ-0006-17); a
   domain, only if
END

### NOTE IE-3 (routed from engine-architect's NOTE IE-3)
already done: ADR 0006 at 8fa7173 splits them. Options Q13 has a row "Standard macOS runners for real Safari" (0 on a
public repository) and a row "Larger paid runners (GPU)" (price not read). Part 8's CI-runner bullet says real Safari
costs runner time, not money, while the repository is public. OQ-0006-7 now asks only about GPU, self-hosted machines
and the cache. No further change.
END

### NOTE IE-4, SA-1 (consistency with ADR 0002's batch)
routed: once these blocks land, ADR 0002 part 10 and OQ-0002-9 read the same as ADR 0006 B9: the guard is
OQ-0002-9's, the scope read is OQ-0006-13's. OQ-0002-9's "Decider" line may add "with OQ-0006-13's read of the two
unread facts". The README register lists both, OQ-0006-13 with its narrowed question. For staging, part 4's interim
is shape (B); OQ-0006-5 now names both of ADR 0002 part 10's shapes, so part 4 needs no change now.
END
