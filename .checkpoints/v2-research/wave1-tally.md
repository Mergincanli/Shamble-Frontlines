Task: wave 1 step 5 (tally: shape, share unsupported, what went unread) and the fetch-method audit, main session.

## Shape (docs/research/README.md, 8 parts)
- R1 and R2 both have the 8 parts in order: header (topic, author, date, quoted brief questions, status), TL;DR,
  findings (source, section, date read, sureness), options, recommendations (reason, "Rests on:", "Serves", proposed
  pass marks), open questions, verifier spot-check (merged table), sources (read and not read).
- One nit: R1's TL;DR is 6 lines (the README allows at most 5). R2's is 5.
- Extra parts both files added (kept): a per-question index table in the header, cross-unit open questions (R1-XU,
  R2-XU), the "recommendations that hinge on a source not read" table, R1's rust-reviewer open questions (R1-RR).

## Share of claims not confirmed (first pass, 2026-10-28)
| Check | Claims | Confirmed | Wrong | Unsupported | Partly | Not checkable |
|---|---|---|---|---|---|---|
| verifier, R1 (six parts) | 226 | 219 (96.9 %) | 1 (0.4 %) | 4 (1.8 %) | 1 | 1 |
| verifier, R2 | 135 | 133 (98.5 %; 1 with a wrong section number) | 0 | 2 (1.5 %) | 0 | 0 |
| rust-reviewer, R1 + R2 server stack | 112 | 110 | 1 (same claim as the verifier's R1-DET-F25b) | 0 | 1 (same as R1-BND-F2b) | 0 |
| security-auditor, R2 protocol | 28 claims + 1 recommendation | 24 | 0 | 0 | 4 (part or caveat) | 0 |
- The security-auditor also found one recommendation contradicted by its own source class (R2-ROOM-REC8: guest secret
  in client storage, against OWASP), 4 High and 10 Medium protocol risks; the rust-reviewer found Rust risks missing
  from the recommendations (R1: 7 open questions R1-RR-OQ1–OQ7; R2: version floors, panic policy, bounded queues).
- Every wrong or unsupported claim a recommendation rested on was re-sourced or turned into an open question
  (R1: 2 re-sourced, 1 open question, 3 fixed in place; R2: 2 re-sourced, 1 open question).
- Second pass: see `verifier-r1-second-pass.md` and `verifier-r2-second-pass.md` (counts added when they return).

## What went unread
- R1: 48 sources not read (of the 53 entries the units listed); R2: 26.
- By cause (both topics): Valve's developer wiki answers HTTP 403 (R1 9 pages, R2 several; the domain is on the allow
  list); GDC Vault redirects to gdconf.com's maintenance page, outside the allow list, and the Overwatch talks are video
  only; Epic's Unreal documentation (dev.epicgames.com) is outside the allow list and the engine's GitHub source needs
  an Epic-linked account (404); the WHATWG HTML standard, ECMA-262 (tc39.es), rand's book (rust-random.github.io),
  bevy.org (bevyengine.org now redirects there), wasm-bindgen's new guide host (wasm-bindgen.github.io), Riot's
  technology blog: outside the allow list; search leads on blogs and forums: outside the allow list.
- Recommendations that hinge on an unread source (the Advisor's SRC-ACCESS (a) list): R1 14 rows (part 6 of the file,
  "Recommendations that hinge on a source not read"), R2 7 rows (same heading). Domains that would close most rows, for
  the next kit PR (the Advisor named Epic's): dev.epicgames.com, html.spec.whatwg.org, tc39.es, rust-random.github.io,
  bevy.org, wasm-bindgen.github.io, technology.riotgames.com. Valve's wiki and GDC Vault are on the list but unreadable
  to the fetcher; another copy is needed (the Advisor's search).

## Fetch-method audit (all agent transcripts of this session, by tool call)
- WebFetch: every wave-1 unit (19–75 calls each), four of R1's six verifiers (ecs, determinism, boundary-worker,
  data-hotreload: 31–38 each), the security-auditor (38), the rust-reviewer (168) and the R1 fixer (9).
- curl through Bash (outside the WebFetch permission layer): verifier-r1-tick (3 calls), verifier-r1-abilities (2
  calls), verifier-r2 (a scripted loop over 111 URLs in `scratchpad/r2v/bin/urls.txt`, plus single calls), the R2
  fixer (4 calls: owasp, docs.rs, raw.githubusercontent.com, rustsec.org, rfc-editor.org), and the advisor's PR #29
  review (3 calls to api.github.com, on the session's instruction). Proxy status checks (`$HTTPS_PROXY/__agentproxy/
  status`) by three units are not web fetches.
- Hosts: every host fetched by curl is on the allow list except api.github.com (the advisor's #29 review).
- Correction: the checkpoint said "the verifiers (R1's six and R2's)" used curl; by the transcripts it was two of R1's
  six (tick, abilities), R2's verifier, and also the R2 fixer.
- The second-pass prompts say WebFetch only, no curl, until the Advisor decides VERIFY-FETCH.

## Lessons for the wave-2 prompts
1. "Rests on:" lists missed claims the reasons use (the verifiers flagged several; the fixers added 13 entries in R1
   and 9 in R2, some never checked by the first pass): the owner prompts say every finding a reason uses goes into
   "Rests on:"; the wave-2 verifier checks the lists.
2. Known-unreadable sources (Valve's wiki 403, GDC Vault redirect, off-list domains above): note them as not read
   without retrying, to save tokens.
3. WebFetch only, never curl or wget (until VERIFY-FETCH is decided); WebFetch returns at most about 125 characters
   verbatim, so long passages are quoted as fragments.
4. The spend limit cut runs off twice: run at most 4–5 agents at once (the Advisor, 2026-10-28).
