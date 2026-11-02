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

### FIX FC-3 (SPIKE_PLAN item 3: R1-RR-OQ5 is measured here; the Advisor sets the limit)
FILE: docs/SPIKE_PLAN.md
BEFORE:
  (optional). **It decides:** R1-BND-OQ2, OQ3 (the core count recorded), OQ6, OQ9 (only if isolation is adopted);
  R1-XU-OQ3, OQ4; R1-TICK-OQ2; R1-DATA-OQ8; R1-RR-OQ5 (with the sim core spike); R5-LAYOUT-OQ4; R5-TSLINT-OQ3, OQ4,
AFTER:
  (optional). **It decides:** R1-BND-OQ2, OQ3 (the core count recorded), OQ6, OQ9 (only if isolation is adopted);
  R1-XU-OQ3, OQ4; R1-TICK-OQ2; R1-DATA-OQ8; R1-RR-OQ5's measurement (with the sim core spike; the Advisor
  sets the first `sim-wasm` limit after the content and ability spike adds content's share, ADR 0002 part 4); R5-LAYOUT-OQ4; R5-TSLINT-OQ3, OQ4,
END
