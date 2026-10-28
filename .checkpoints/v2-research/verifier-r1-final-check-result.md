# R1: the final check, the agent's report verbatim

Workflow run wf_0ad9f0c0-59d (script `v2-research-fix2-run1.js`, prompt `prompts/verifier-r1-final-check.txt`), returned 2026-10-28 (about 20:24 UTC). Notes: `verifier-r1-final-check.md`.

VERDICT: PASS WITH NOTES. Everything the second fix round changed holds up against the sources. Only nits remain.

**Gate:** skipped, as task rule 1 says (research check; the main session runs the gate). There is no symbol diff because no code changed.

**1. Items the second pass found wrong or unsupported, and its recommendation problems: all closed**
- **R1-DET-F34b** (was wrong): corrected to the source's words. F34 now quotes "Note that two generators with different stream parameters may be closely correlated." (l.812-820). R1-DET-REC4 no longer counts the stream number as a merit or defers the question "for the spike" (l.2142-2145). The options Q4 row is fixed (l.1706). The rest is now an open question, R1-DET-OQ11 (l.2628-2632). Closed.
- **R1-DATA-REC6 file:// clause** (was wrong): cut, and plain-HTTP LAN kept (l.2491-2493). R1-DATA-OQ11 corrected and attributed (l.2733-2737). Closed.
- **R1-ECS-F34** (was unsupported, an absence): its "Sure" label now says no quote can confirm it (l.263-267). It is out of R1-ECS-REC1's "Rests on" (l.1900), and mark 5 says it found no figure (l.1924). Closed.
- **R1-ECS-REC1 minor point:** "does not reuse buffers" is now marked inferred (l.1886-1890, options table l.1621). F35's head reads "clones field by field" (l.270), and "derive leaves clone_from at the default" is marked our reading (l.277-280). Closed.
- **R1-DET-REC5 and REC6 minor point:** both now say "can catch" / "can expose", marked inferred (l.2163-2166, l.2175-2179). Closed.
- **R1-TICK-REC2 Serves line:** now cites §3, not §6.1 (l.2009-2011). VISION_V2.md:44 says "The goal: 12v12v12 or 24v24.", :48 says "Bots fill empty slots.", and §6.1 is "Modes as data". Closed.
- **R1-BND-REC5:** "or CORS" cut, and Serves now says no §6 item names isolation (l.2255-2258). Closed, with nit (a) below.
- **Part 7 rows, the draft date, and the F10 process note:** 8 rows added. The unit tables now have 251 data rows (243 at HEAD, plus 8). The F3 and F10 rows and F35 carry the notes (l.3612, 3619, 1128-1133). Closed.

**2. Claims the round re-sourced or added, checked at the source**

| id | claim, short | source | verdict | location and exact words |
|---|---|---|---|---|
| R1-DET-F34 | Pcg32 streams may be closely correlated | docs.rs rand_pcg 0.10.2, Lcg64Xsh32 | confirmed | `new` docs: "Note that two generators with different stream parameters may be closely correlated." The struct docs repeat it ("…stream parameter may be…", singular on the page). |
| R1-DET-F34 | constructor, jump-ahead (quotes re-laid) | same page | confirmed | `pub fn new(state: u64, stream: u64) -> Self` "Construct an instance compatible with PCG seed and stream."; `advance`: "Multi-step advance functions (jump-ahead, jump-back)" |
| R1-DATA-OQ11 | a file:// page is a secure context | MDN Secure contexts | confirmed | "This page was last modified on Sep 14, 2026"; table row "`file:///path/to/resource.html`" "✅ Secure (`file` URL)" |
| R1-DATA-REC6 | plain-HTTP LAN is not a secure context | MDN Secure contexts (with F29) | confirmed | "Documents are secure contexts when their resources are delivered over an [HTTPS] connection, or are delivered from a loopback (local) address."; "`http://example.com`" "❌ Not secure (`http` URL)" |
| R1-BND-F35 | the draft is dated 28 October and step 5 is unchanged | WebAssembly JS API | confirmed | "Editor's Draft, 28 October 2026"; "5. Set buffer.[[ArrayBufferDetachKey]] to "WebAssembly.Memory"." |
| R1-BND-REC5 | a `no-cors` cross-origin resource needs CORP | MDN COEP (last modified Sep 4, 2026) | confirmed | `require-corp`: "A document can only load resources requested in `no-cors` mode from the same origin, or resources that have explicitly set the Cross-Origin-Resource-Policy header to a value that allows it to be embedded." |
| R1-ECS-REC1 / F35 | default clone_from is "equivalent to `a = b.clone()`"; the derived Clone calls clone on each field | std Clone 1.99.1 | confirmed | "`a.clone_from(&b)` is equivalent to `a = b.clone()` in functionality, but can be overridden to reuse the resources of `a`…"; "The `derive`d implementation of Clone calls clone on each field." The page has no sentence on whether derive writes clone_from, so "our reading" is the right label. |

Also re-read for item 3: std HashMap 1.99.1, "each `HashMap` instance uses a different seed" and "An iterator visiting all key-value pairs in arbitrary order." These support REC5 and REC6's "can … (inferred)".

**Counts:** 7 confirmed, 0 wrong, 0 unsupported.

**3. Rewritten reasons, pass marks and numbers: closed**
- REC4, REC5, REC6 and ECS-REC1 now claim only what F34, F33 and F35 say, and the rest is marked inferred. BND-REC5 claims only F21's CORP sentence.
- No new number was added to a pass mark.
- The counts are redone and correct:
  - 77 open questions: 76 at HEAD plus R1-DET-OQ11; the definitions count ABL 10, BND 9, DATA 12, DET 11, ECS 7, TICK 9, RR 7, XU 12.
  - 37 recommendations.
  - Second-pass tally: 42 + 2 + 1 + 11 = 56.
  - The new subsection has 33 claim rows and 7 recommendation rows.

**4. Header and part 7's "Second verifier pass" subsection: closed**
- The header (l.10-13) matches the file: 2 wrong and 1 unsupported claim fixed, none re-sourced.
- The subsection (l.3333-3417) matches the second pass's report and the fixes.
- The sentence "No recommendation rests on a claim whose final verdict is wrong or unsupported" now holds.

**What remains: nits only**
- **(a) CORS is less of an inference than the file says.** F21's own source, MDN COEP, says "requests made in `cors` mode won't be blocked by COEP or trigger COEP violations, but must still be permitted by CORS." So the labels "CORS as another route is our reading" in options G2 (l.1766-1767) and R1-BND-OQ5 (l.2650-2651) understate the evidence. They are not wrong, and the CORP-only REC5 is stricter, not false. Optional fix: add that quote to F21.
- **(b) MDN Secure contexts is cited by title and date only** (l.3622-3625). The URL is https://developer.mozilla.org/en-US/docs/Web/Security/Secure_Contexts.

**Not verified:** "a transfer of the Wasm memory's buffer throws". The rule is in ECMA-262 and the HTML standard, both outside the allow list, and the file already marks it inferred.

**Header label the file should carry:** "verified (every recommendation claim)"

Notes: /home/user/Shamble-Frontlines/.checkpoints/v2-research/verifier-r1-final-check.md (ends with DONE). File checked: /home/user/Shamble-Frontlines/docs/research/r1-simulation-core.md
