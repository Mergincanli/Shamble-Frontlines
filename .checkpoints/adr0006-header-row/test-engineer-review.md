# test-engineer review, round 1: ADR 0006 header-row revision (09faefa..1c45ed7)

Reviewer: test-engineer (owner of ADR 0006's pass marks). Started 2026-11-25. Notes only; no docs edited, no commit.
Memory: no local agent-memory folder for test-engineer in this cloud session (checked .claude/ and ~/.claude/).

## Plan
1. Read the ADR diff (E1, E2, E3, E4, E6, E7, E9, E10) against plan.md revision 2, section 3. DONE.
2. Read Mustafa's ONE-OFF-HEADER (a) and STEP4-COUNT (b) (DECISIONS :169-193) and the options as put (report :278-301). DONE.
3. E1/E2: enumerate what fails PM-0006-2 on each URL form after the revision; re-derive the bite and the "met by
   readings already taken" claim from the raw verify artifacts in `.gate/preview-hosting-check/day1/`
   (round-a, bite-production, restore-production) with jq, not only from Appendix A's prose.
4. E4/E9/E10: can PM-0006-7 fail, can it record something wrong; consistency with PM-0006-9, the "When a mark fails"
   paragraph (ADR :1420-1434) and the hosting checklist intro (:1462-1467).
5. The advisor's note: the preview `noindex` gap; word a follow-up.
6. Looser-than-his-words check on every revised cell.
7. Verdict and ranked findings, file:line, exact fix.

## Findings (appended as found)

### Evidence re-derived from the raw verify files (jq on `.gate/preview-hosting-check/day1/*/headers.json`)
- round-a: 6 forms; 5 pass; production one-off 0354374e fails on 7 paths, distinct failures exactly
  ["production carries X-Robots-Tag noindex"]; nothing else on any form.
- bite-production (e9a37459): stable https://shamble-preview-check.pages.dev/ fail, 7 paths, only the noindex failure;
  one-off fail, 7 paths, same and only. => the bite bites on production's own address under the revised row.
- restore-production (de1c3e5e): stable pass (control: the host adds nothing there); one-off fail, 7 paths, only the
  noindex failure. => under the revised row this form passes (X-Robots-Tag recorded).
- uploads: 20 of 20 preview forms pass, x-robots-tag "noindex" on every response. bites: only the planted failures.
- So E7's last sentence ("the revised header row is met by the readings already taken on Pages") holds for PM-0006-2.
- Build rules (`.gate/preview-hosting-check/builds/*/_headers`): production has no X-Robots-Tag rule; preview and
  preview-isolated both ship `/* X-Robots-Tag: noindex`; responses carry ONE value "noindex".

### E1: what fails PM-0006-2 now, per URL form
- Preview alias and preview hash: wrong `.wasm` type; COOP/COEP not per the flag; X-Robots-Tag not noindex (but see F-gap).
- Production own address: any X-Robots-Tag noindex; any COOP/COEP; wrong `.wasm` type. Proven to fail (bite) and to pass
  (round-a, restore).
- Production hash: wrong `.wasm` type; any COOP/COEP. X-Robots-Tag never fails (recorded). Stricter than option (a)'s
  literal "judge production's own address only" (other headers still judged there). Not looser.

### Findings
- F1 (Medium, pre-existing, follow-up; the advisor's note): the preview noindex expectation cannot fail on pages.dev.
  Cloudflare adds noindex to every preview deployment (r5 :839-841, Pages docs; consistent with production's one-off
  links measured), and the response shows one "noindex", so a preview build missing its `/* X-Robots-Tag: noindex` rule
  would pass. Follow-up wording in the reply.
- F2 (Low, trivial wording): PM-0006-2's claim cell (ADR :1386) still says "serves each header set as generated, on both
  URL forms"; on production's one-off form the host adds a header outside the set. Append the exemption to the claim.
- F3 (Low): E9 "the count the page shows" (:1391) names no page, so a "no count shown" reading cannot be checked or
  redone. The second reading read only the Deployments page (report :750-757), the first read three places (:732).
  Fix: "... or as "no count shown", with the dashboard pages read named". Same in E10 (:1468).
- F4 (Low, trivial wording): the checklist intro (:1459) still says "or the numbers it asks for"; step 4 now asks for
  readings. -> "or the readings it asks for".
- F5 (Low): E4 (:1391) dates the docs read "2026-11-24" only; part 1 (:143) says "read 2026-11-24 and 2026-11-25".
  Align E4 to both dates.
- Not a finding: E9/E10 loosen the draft's old pass words ("two numbers") but are tagged as the advisor agent's reading,
  and plan §4/§8 ask him separately whether his go covers them. That keeps them within his words; dropping that
  separate question from the report would make it High (a rule loosened beyond his words).
- Consistency: E9 matches "When a mark fails" (:1421, a reported mark passes when its record is complete); PM-0006-9
  unchanged and consistent (step 4's expected result is met by the 2026-11-18 readings only if he says "achieved").

## Revision after the advisor tool's read (supersedes F1-F5 above where they differ)
Artifact ids (`.gate/` is gitignored): round-a headers-44347351522, artifact 14448883620; bite-production
headers-44366750977, artifact 14458320188; restore-production headers-44367096948, artifact 14458520412; uploads
artifact 14453018448; bites artifact 14456636722 (report Appendix A :667-672, :776-781, :805-818).

E9 (check 1): PM-0006-7 fails only on a missing reading (a reported mark, "When a mark fails" :1421). A wrong
"no count shown" is harmless: the cautious rule (newest push only, deploys counted) holds anyway. The wrong record that
matters: a figure that is not the quota's count (for example one that counts Git builds only) reading "rose by 0" would
lift the rule. Older than this change: follow-up F7.

E6 (spike 1's list, :1229-1230): "not answered by the check, which saw no count (part 1)": clear, consistent with E3,
E4 and E10. No finding.

Custom domain (check 5): ADR :315 says production uses shamblefrontlines.com (the advisor's reading, 2026-11-03), and
PM-0006-8 already reads "a custom domain if CF has one". E1 pins "own address" to the `pages.dev` host, so a production
custom domain would be judged on neither side. No effect on readings taken (the test project had none). F6, Low.

### Final findings (exact before -> after; fixes outside plan §3 go into plan §3 and the report's table, or follow-up)
- F1 Medium, pre-existing, FOLLOW-UP (the advisor agent's note). Wording:
  "PM-0006-2 cannot catch a preview build that lacks its `noindex` rule. Every preview build of the check shipped
  `/* X-Robots-Tag: noindex`, and every preview response carried one `noindex`; the host adds the same header by itself
  (measured on production's one-off addresses 0354374e and de1c3e5e, which have no rule; on preview branch aliases only
  by Cloudflare's docs, r5 :839-841). So a missing rule would pass. (1) Step 2.6 gets a build-output test: each preview
  mode's generated `_headers` carries `X-Robots-Tag: noindex` on `/*` and production's carries none, with a bite (the
  rule removed fails the test). (2) Before any preview is served where the host adds no `noindex` (a custom domain on a
  preview; Workers' custom-domain Previews, ADR 0006 part 1 :157-158), PM-0006-2's preview expectation is read there,
  with a bite build that lacks the rule."
  Sure: high on hash addresses (measured); medium-high on branch aliases (docs only; not separable on Pages).
- F2 Low, trivial, outside §3. PM-0006-2 "what it proves" cell (:1386):
  "The host serves each header set as generated, on both URL forms (part 1; decides R5-PREV-OQ6)" ->
  "The host serves each header set as generated, on both URL forms (production's one-off addresses: the host's own
  `X-Robots-Tag` recorded, not judged) (part 1; decides R5-PREV-OQ6)".
- F6 Low, trivial, inside E1. PM-0006-2 "number and statistic" cell (:1386):
  "production none on its own address (the project's `pages.dev` host)" -> "production none on its own addresses (the
  project's `pages.dev` host, and a custom domain if CF has one, as in PM-0006-8)".
- F5 Low, trivial, inside E4 (not tagged words). PM-0006-7 "number and statistic" cell (:1391):
  "and Cloudflare's docs do not say (2026-11-24)" -> "and Cloudflare's docs do not say (read 2026-11-24 and 2026-11-25)".
- F4 Low, trivial, outside §3. Checklist intro (:1459): "or the numbers it asks for." -> "or the readings it asks for."
- F3 Low, FOLLOW-UP (E9/E10 are the advisor agent's tagged exact words; do not edit them). PM-0006-7 "method and tool"
  cell (:1391), for a later revision: append "; each reading names the dashboard pages read" (the second reading read
  the Deployments page only, report :750-757; the first read three places, :732).
- F7 Low, pre-existing, FOLLOW-UP. PM-0006-7 "number and statistic" cell: "does not if by 0" -> "does not if by 0 on a
  figure the page names as the Free plan's build count".
- C1, carried to the final-head review (the report is not in this diff): the report's "For Mustafa" asks separately
  whether his go covers E9 and E10 (plan §4, §8). Without that question, E9/E10 loosen the draft's old pass words
  ("two numbers") beyond his words: High.

## VERDICT: APPROVE (no Critical or High; C1 carried to the final head; F1, F3, F7 follow-ups; F2, F4, F5, F6 optional
trivial wording fixes).
