# engine-architect synthesis fixes, final round (docs at 604a497)


### FIX FC-2 (README: the draft-local code families, as skeptic-5 upholds them)
FILE: docs/adr/README.md
BEFORE:
appendix, `docs/reports/2026-11-01-v2-decision-drafts-appendix.md` §3–§4, quotes them word for word. Codes local to one
draft are not register codes: ADR 0002's gate checks G1–G13, ADR 0004's spike claims S1–S22 and ADR 0006's option
labels (G1–G4, S1–S3, K1–K4, B1–B11); a register code always carries its word ("gap G5", "seam S1").
AFTER:
appendix, `docs/reports/2026-11-01-v2-decision-drafts-appendix.md` §3–§4, quotes them word for word. Codes local to one
draft are not register codes, for example ADR 0002's gate checks G0–G13, ADR 0004's spike claims S1–S22, ADR 0006's
option labels (G1–G4, K1–K4, S1–S5) and its baseline items B1–B12 (part 13); a register code always carries its word
("gap G5", "seam S1").
END

### FIX FC-3 (SPIKE_PLAN item 3: the spike measures for R1-RR-OQ5; the Advisor sets the limit)
FILE: docs/SPIKE_PLAN.md
BEFORE:
  (optional). **It decides:** R1-BND-OQ2, OQ3 (the core count recorded), OQ6, OQ9 (only if isolation is adopted);
  R1-XU-OQ3, OQ4; R1-TICK-OQ2; R1-DATA-OQ8; R1-RR-OQ5 (with the sim core spike); R5-LAYOUT-OQ4; R5-TSLINT-OQ3, OQ4,
  OQ6; R5-TEST-OQ1; R6-OBS-OQ3, OQ4 (optional); critic ranks 4 (the implementation side) and 37; it reports the copy time for
  R1-RR-OQ6 (PM-0002-10; ADR 0002 answers R1-RR-OQ6, proposed, for rust-reviewer to confirm); it gives OQ-0002-1,
AFTER:
  (optional). **It decides:** R1-BND-OQ2, OQ3 (the core count recorded), OQ6, OQ9 (only if isolation is adopted);
  R1-XU-OQ3, OQ4; R1-TICK-OQ2; R1-DATA-OQ8; R5-LAYOUT-OQ4; R5-TSLINT-OQ3, OQ4, OQ6; R5-TEST-OQ1; R6-OBS-OQ3, OQ4
  (optional); critic ranks 4 (the implementation side) and 37. With the sim core spike it measures the `sim-wasm` size
  for R1-RR-OQ5, whose first limit the Advisor sets once the content and ability spike adds content's share (ADR 0002
  part 4); it reports the copy time for R1-RR-OQ6 (PM-0002-10; ADR 0002 answers R1-RR-OQ6, proposed, for rust-reviewer
  to confirm); it gives OQ-0002-1,
END

### FIX FC-4 (SPIKE_PLAN, the foundation: the deploy token is ADR 0006 part 13, item B9; OQ-0006-13 first)
FILE: docs/SPIKE_PLAN.md
BEFORE:
- **Depends on:** the drafts accepted; the boundary spike's and the netcode proof test's results; the preview hosting
  check before the preview step; Mustafa's client-host token (ADR 0006 part 1) and branch-protection setting.
AFTER:
- **Depends on:** the drafts accepted; the boundary spike's and the netcode proof test's results; the preview hosting
  check before the preview step; Mustafa's client-host deploy token in the `preview` environment (ADR 0006 part 13,
  item B9; how production stays out of that token's reach is OQ-0006-13, answered before the first client-host token
  is added) and branch-protection setting.
END

### FIX SA-1 routed by infra-engineer (SPIKE_PLAN item 6: the test project's token is revoked when the check ends)
FILE: docs/SPIKE_PLAN.md
BEFORE:
- **Depends on:** the drafts approved; Mustafa's Cloudflare account with a test project and its deploy token in a
  `preview` environment (secrets are his to add); a static test page with a module worker, a `.wasm` and a hashed
  bundle stand-in; throwaway deploy and close workflows; ML and MAC with the four browsers.
AFTER:
- **Depends on:** the drafts approved; Mustafa's Cloudflare account with a test project and its deploy token in a
  `preview` environment (secrets are his to add; he revokes the token when the check ends: ADR 0006 part 13, item B9);
  a static test page with a module worker, a `.wasm` and a hashed bundle stand-in; throwaway deploy and close
  workflows; ML and MAC with the four browsers.
END
