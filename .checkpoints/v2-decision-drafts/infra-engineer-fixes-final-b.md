# infra-engineer, follow-up batch b (ADR 0006), from engine-architect's ADR 0002 notes and the coordinator's correction

Every BEFORE was copied by script (whole lines, by line range) from docs/adr/0006-infrastructure-operations.md at
8fa7173 (identical to the branch head for this file) and occurs exactly once there; no two blocks share a line; none
touches "### Pass marks". Per the coordinator's correction there is no OQ-0002-9: ADR 0002 part 10 points to
OQ-0006-13, which keeps the guard question (B9 names it and the `production` environment), and R5-PREV-F6 is cited at
r5 lines 796-804 (9e7db28) in all three places. This batch replaces any earlier draft of batch b.

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

### FIX SA-1, IE-14 (part 13, B9: the `production` environment, OQ-0006-13, the soak's VM; lines 487-487 at 8fa7173)
FILE: docs/adr/0006-infrastructure-operations.md
BEFORE:
    | B9 | The tokens CI holds (the client host's deploy tokens, for previews and for production; the Sentry upload token, "a token: Mustafa's secret", r6 line 1082), a multiplayer preview's binary delivery (R6-XT-OQ4), and the Hetzner API token the spikes and the tick bench need (OQ-0006-17) | their scope and handling | each held only by the job that needs it, in a GitHub environment, as R5-PREV-REC1 does for the deploy token (proposed for the others); production deployed only from `main`, in its own environment with its own token; a required reviewer on `preview`; the preview hosting check's token revoked when the check ends; the Hetzner token in a project of its own; all added by Mustafa (below; OQ-0006-13) | 0002 (the jobs); 0006 (the scope) |
AFTER:
    | B9 | The tokens CI holds (the client host's deploy tokens, for previews and for production; the Sentry upload token, "a token: Mustafa's secret", r6 line 1082), a multiplayer preview's binary delivery (R6-XT-OQ4), and the Hetzner API token the spikes, the tick bench and the 3-hour soak before each playtest need (OQ-0006-17) | their scope and handling | each held only by the job that needs it, in a GitHub environment, as R5-PREV-REC1 does for the deploy token (proposed for the others); production deployed only from `main`, in its own environment `production` with a `main`-only deployment-branch rule and its own token (ADR 0002 part 10); a required reviewer on `preview`, OQ-0006-13's proposed interim (what keeps a rewritten deploy job from the token); the preview hosting check's token revoked when the check ends; the Hetzner token in a project of its own; all added by Mustafa (below; OQ-0006-13) | 0002 (the jobs); 0006 (the scope; OQ-0006-13) |
END

### FIX SA-1 (part 13, Secrets: `preview` and `production`; lines 497-498 at 8fa7173)
FILE: docs/adr/0006-infrastructure-operations.md
BEFORE:
    project was not read), held only by the deploy job's environment `preview` (R5-PREV-REC1, r5 lines 1588–1604; ADR
    0002's job). That environment alone does not keep the token from a branch: a `pull_request` run takes its workflow
AFTER:
    project was not read), held only by the deploy jobs' environments, `preview` and `production` (R5-PREV-REC1, r5
    lines 1588–1604; ADR 0002's jobs). The `preview` environment alone does not keep its token from a branch: a
    `pull_request` run takes its workflow
END

### FIX R5-PREV-F6 citation (part 13, Secrets: R5-PREV-F6 is r5 lines 796–804 at 9e7db28; lines 505-505 at 8fa7173)
FILE: docs/adr/0006-infrastructure-operations.md
BEFORE:
    ("Deployment branches and tags are available for all public repositories", R5-PREV-F6, r5 lines 795–803), with its
AFTER:
    ("Deployment branches and tags are available for all public repositories", R5-PREV-F6, r5 lines 796–804), with its
END

### FIX IE-14, TE-7 (part 13, Secrets (d): the Hetzner token serves the soak too; lines 509-510 at 8fa7173)
FILE: docs/adr/0006-infrastructure-operations.md
BEFORE:
    when the check ends (part 14); (d) the Hetzner API token for the spikes and the tick bench belongs to a Hetzner
    project of its own, never the production project (a project's token presumably reaches all its VMs: not read;
AFTER:
    when the check ends (part 14); (d) the Hetzner API token for the spikes, the tick bench and the 3-hour soak before
    each playtest belongs to a Hetzner project of its own, never the production project (a project's token presumably
    reaches all its VMs: not read;
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

### FIX SA-1, R5-PREV-F6 citation (OQ-0006-13: ADR 0002 part 10 points here; the delivery and Sentry jobs; the citation; lines 890-899 at 8fa7173)
FILE: docs/adr/0006-infrastructure-operations.md
BEFORE:
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
AFTER:
  - *Options:* (a) a required reviewer on `preview`, and on the delivery and Sentry jobs' environments when they run
    for a preview (part 13's interim position): Mustafa approves each preview deploy ("a job cannot access environment
    secrets until one of the required reviewers approves it", R5-PREV-F6, r5 lines 796–804); who approves when
    sessions push and post as his account (CLAUDE.md), and whether GitHub can stop an account approving a run it
    started, were not read; (b) production in a second Cloudflare account with its own token, so the preview token
    cannot reach it, and `preview` without a reviewer (not researched; 0 USD if it stays on the Free plan, inferred as
    in part 1); (c) a token limited to the preview project, if Cloudflare offers one (not read). Beside them, the
    security review's question 1: whether B9's part of the security baseline is read before the foundation's preview
    step; that review recommends not, since the interim position needs no new research. ADR 0002 part 10 points here.
  - *Decider:* the Advisor, with security-auditor; Mustafa for a second account and for the approvals (a) asks of him.
  - *What decides it:* a read of GitHub's rules for reviewing pending deployments and of Cloudflare's token scopes,
    and the approvals' cost per pull request; before the first client-host token is added to this repository; the
    preview hosting check's B9 marks (What a spike must prove, item 1).
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

### FIX R5-PREV-F6 citation (OQ-0006-18: R5-PREV-F6 is r5 lines 796–804; lines 968-968 at 8fa7173)
FILE: docs/adr/0006-infrastructure-operations.md
BEFORE:
    public repositories" on the Free, Pro and Team plans (R5-PREV-F6, r5 lines 795–803), so B9's reviewer would need a
AFTER:
    public repositories" on the Free, Pro and Team plans (R5-PREV-F6, r5 lines 796–804), so B9's reviewer would need a
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

### NOTE SA-1, IE-4 (consistency with ADR 0002)
routed: ADR 0002 part 10 points to OQ-0006-13 for the guard, which now also names the delivery and Sentry jobs'
environments and the unread self-approval question that OQ-0002-9 had; the README register lists OQ-0006-13 once
(decider the Advisor with security-auditor, Mustafa for a second account and the approvals). For staging, part 4's
interim is shape (B); OQ-0006-5 now names both of ADR 0002 part 10's shapes, so part 4 needs no change now. Part 3 now
sets the CI failure artefacts' retention (30 days, proposed), which ADR 0002 part 8 can cite.
END
