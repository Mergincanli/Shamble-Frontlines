# engine-architect review: docs-after-kit-v3-1 final diff (head 603f53b on main 868e165)

Scope: PORTAL-EMBED in ADR 0002 and its pointers (plan C1, C2, C5, C6; advisor conditions 6, 7, 8; README counts C4).

## Review plan
1. Read ADR 0002 part 5 isolation bullet in full (head), the failure-mode bullet, OQ-0002-4, OQ-0002-9.
2. Check the four places say one thing; check nothing designs the fallback (no build variants, runtime detection, options).
3. Check ADR 0004 part 5 (W2, W3, PM-0004-68) against the "B3, C2 and W3" reading and the W2/W3 consequence.
4. Check SPIKE_PLAN §3 new bullet and ENGINE_OVERVIEW line 68: cite, not restate or change.
5. Count OQ headings in all six ADRs myself; check README rows, totals, two register rows.
6. Grep "portal" in the diff: "web portal" only; no bare "portal" in new text.
7. Verdict.

## Notes as I go
- Four places agree (head): part 5 0002:364-368 ("flips at once everywhere the client runs isolated" + picking rule,
  OQ-0002-9); OQ-0002-4 (b) 0002:1026-1028 ("non-isolated path kept for a third-party iframe"); failure bullet
  0002:1507-1511 ("wherever the client runs isolated"; "sends the pacing case back to Mustafa"); OQ-0002-9 0002:1099-1103.
  Coherent reading: W3 may be picked where isolated; inside an iframe W2 stays; W2 failing its mark -> Mustafa.
- No fallback design: grep of added lines for detect/variant/runtime/crossOriginIsolated/build-with: none.
- Mustafa's quote in 0002:1094-1098 and 0006:1151-1155 identical to checkpoint line 202 (normalised compare).
- ADR 0004 part 5 (0004:211-213): W2 proposed, W3 measured alt if W2 fails the jitter mark, isolation = ADR 0002's call.
  PM-0004-68 (0004:1195) is that jitter mark; PM-0002-26 (0002:1276) defers to it. Citation right.
- BUT "the only non-isolated fallback is W2" (0002:1102): W1 and W4 (0004:647) need no isolation either (weighed, set
  aside). True only among the options ADR 0004 keeps. -> minor.
- "every design in this record that needs isolation: B3, C2, W3" (0002:1099-1101): B4 (0002:799) and E2 (0002:819) also
  need isolation (rejected). List not exhaustive. -> nit.
- Part 5 0002:367-368 attributes "(B3, C2 or W3)" with "Mustafa 2026-11-04"; widening marked as the advisor's reading
  only in OQ-0002-9 and README:86. -> nit.
- GLOSSARY.md:641-642 (linked from 0002:359) still "then the default flips everywhere at once", and lacks the W3 trigger.
  Contradicts part 5 now. docs-pass-items.md:70-71 asked to touch it if it contradicts; plan C omitted it. -> major.
- "the same client host" 0006:1156-1157 ("his words above") and README:156: his words say "sharing our servers and
  player pool", not the client host. Unsupported, leans to design (where a web portal's copy is served). -> minor.
- SPIKE_PLAN:262-263 cites part 5 + OQ-0002-9, no restatement. ENGINE_OVERVIEW:68-69 cites OQ-0002-9. OK.
- "portal": no bare "portal" in any added line (all files); ADR 0001 13, ADR 0004 6, GLOSSARY 2 hits = enclosure portals.
- Counts (own): headings 18/9/20/12/36/23 = 118; open 12/8/17/10/33/18 = 98. README rows :17, :21, total :23-27 right.
  Register rows README:86 (after -6; -7/-8 have no rows) and :156 (after -22). STATUS:224 old 116/96 is a dated log entry.
- No revision log in the drafts to update. No VISION §6 feature added (the rule keeps today's default; designs nothing).

## Verdict: PASS WITH NOTES (no blocker; fix the major GLOSSARY contradiction in this pass; two minor, two nits). Details in StructuredOutput.
