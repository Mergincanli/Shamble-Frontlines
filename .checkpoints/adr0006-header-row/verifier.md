# Verifier notes: ADR 0006 header-row revision (diff 09faefa..1c45ed7)

Started 2026-11-25. Change type: decision-doc revision with research claims. Plan: plan.md revision 2.

## Plan
1. E7 and E3 claims: fetch each Cloudflare page myself; verdict PASS / FAIL / UNVERIFIABLE with the page's words.
2. Numbers and dates in E1, E2, E4, E6, E9, E10 against the merged report's Appendix A
   (round-a :664-678, bite/restore-production :803-826, dashboard :706-757, docs reading :1384-1388).
3. E7's outcome sentence against Appendix A under E1's new words (this row only).
4. DECISIONS: numstat 16/0; quoted block bytes and sha256; equality with transcript message 30f4ab9b-...
5. Gate: node tools/gate.mjs --no-smoke (smoke cannot start locally: Playwright wants chromium_headless_shell-1194,
   container has 1243; CI runs smoke).
Also: symbol diff (changed sentences) vs plan section 3; diff touches only the planned files.

## Results

### Item 5. Gate (done)
`node tools/gate.mjs --no-smoke`: GATE PASS, exit 0. [1] syntax 5/5; [2] concat PASS; [3] symbols +0 -0 (core 138 /
game 714 / editor 353), duplicates 0; [4] player build 51 refs guarded, 0 flagged; [5a] SKIP (--no-smoke); [5b] names 0
problems; [5c] layering 0; [5d] script order PASS. Base 09faefa (origin/main). Smoke not run locally: tools' Playwright
wants chromium_headless_shell-1194, container has 1243 (caller's statement; not re-tested). CI runs smoke on the head.

### Item 2. Numbers and dates vs Appendix A (done)
Report text, and cross-checked against the raw artifacts in .gate/preview-hosting-check/day1/*/headers.json
(script: verifier-scratch/headers_summary.py):
- round-a (:664-678; headers.json checked_at 2026-11-18T17:02:41.858Z, summary 3 rec / 5 pass / 1 fail): production
  one-off https://0354374e...pages.dev/ fail on 7 of 7 paths, only msg "production carries X-Robots-Tag noindex",
  x-robots-tag "noindex" on all 7; production own address pass, x-robots none. PASS.
- bite-production (:803-811; checked_at 18:11:38.137Z, 1 rec / 0 pass / 2 fail): own address and e9a37459 both fail 7/7,
  only the noindex msg. "the bite caught on production's own address" (:811). PASS (E2).
- restore-production (:812-820; checked_at 18:13:45.151Z, 1 rec / 1 pass / 1 fail): own address pass (no noindex);
  de1c3e5e one-off fail 7/7, only the noindex msg. PASS (E1's "0354374e and de1c3e5e ... all 7 paths").
- Dashboard readings (:706-757): first reading 17:19:10 / 17:21:40 / 17:23:36 UTC 2026-11-18, "No count of builds is
  shown" on Deployments, Metrics, overview; second reading 17:33:15 UTC 2026-11-18, "No count of builds is shown". Ten
  uploads 17:28:10-17:28:52 between them. Both on 2026-11-18: PASS (E3, E4, E10). "ten uploads": 10 listed. PASS.
- Docs reading (:1384-1388) under "After the verifier (2026-11-24, before the report)", agent "after 23:09 UTC": docs
  do not say. Date 2026-11-24: PASS (E3, E4).
- "written by no rule of the production build" (E1): report :668 "production's build writes no noindex, so the host
  added it"; own address of the same deployment carries none (artifact). PASS.

### Item 3. E7's outcome sentence, this row only (done)
Under E1's new words, every reading on Pages (artifacts):
- every header on both URL forms: .wasm application/wasm on every link read (round-a 6, uploads 20, restore 2, bites 4);
  every status 200, 7 paths per link. Production one-off links fail on nothing but the noindex msg. PASS.
- production's own address no noindex: round-a and restore-production pass; x-robots none. PASS.
- previews noindex: round-a 4 links, uploads 20 links, bites 4 links: x-robots "noindex" on every path. PASS.
- COOP/COEP: iso-on both links same-origin / require-corp on all 7 paths; iso-off and production none. PASS.
- both bites caught: bite-coep only "/worker.js COEP "" expected require-corp" on both links; bite-production noindex on
  production's own address. PASS.
- One-off X-Robots-Tag on production now "recorded, not judged": recorded noindex on 0354374e and de1c3e5e. PASS.
Verdict: "the revised header row is met by the readings already taken on Pages" holds for PM-0006-2 alone. The whole
check is not judged here (Safari's PM-0006-3 row stays open, report :789-795).

### Item 4. DECISIONS (done)
- `git diff --numstat 09faefa..1c45ed7 -- docs/DECISIONS.md`: 16 0. No removed lines. Placed first under "Decisions,
  newest first", above the 00:22:09 UTC entry.
- Transcript (python, verifier-scratch/dec_check.py): uuid 30f4ab9b-... found once, type user, timestamp
  2026-11-25T11:07:07.843Z; content is one text string: 741 bytes, sha256 ae5eb4f6...176eb.
- DECISIONS quoted block (5 lines, "> " stripped, joined by \n): 741 bytes, sha256 ae5eb4f6...176eb; string-equal to the
  message: True. PASS.
- Records in the entry: gh api pulls/66: merged_at 2026-11-25T11:10:11Z, merge_commit_sha 09faefa7f7cd...4737, head
  d7937de0aad4...b67c; git: 09faefa's second parent d7937de. Match. PASS.
