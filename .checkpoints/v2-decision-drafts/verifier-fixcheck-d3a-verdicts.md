# fixcheck-d3 unit a: verdicts (ADR 0001, 0002, 0006 touch-ups after fixcheck-d1)

Verifier: fresh verifier, fixcheck-d3a. Base of the changed text: 449356c. HEAD at start: f9ec6c8. No web page fetched.

## 0. Preconditions

- `git diff --stat 604a497 449356c` on the three ADRs: empty, so fixcheck-d1's text at 604a497 is the base. Confirmed.
- `git diff 449356c HEAD` on the three ADRs: 3 files, 41 insertions, 40 deletions; commits cd1fc65 (0002), 7858762
  (0001, 0006), 750d46d (0001, 0006); the working tree has no change to them. Saved:
  verifier-fixcheck-d3a-scratch/diff-449356c-HEAD.txt and wdiff.txt.
- Line numbers below are HEAD's (f9ec6c8). ADR 0001 grew by one line at 1359 (FIX F7), so its rows after 1359 sit one
  line lower than in d1 (PM-0001-27 now 1410, PM-0001-44 1431, PM-0001-60 1429). No doc under docs/ cites an ADR 0001
  line after 1359 by number (grep for `0001:1[3-9]xx`, "lines 1[3-9]xx" and `#L1[3-9]xx`: 0 hits), and the OQ lines the
  README points to lie above 1359.

## 1. Claims table (every added or changed claim and number)

| # | doc:line | claim (short) | cited | verdict | where and the words | note |
|---|---|---|---|---|---|---|
| C1 | 0001:188–189 | a pick or effect that changes move speed or a fire interval "causes no correction after the first snapshot that carries it" | part 4 (own-state section); PM-0001-60 | confirmed | 0001:1429 PM-0001-60: "at most one correction, at the first snapshot that carries the change, and 0 corrections larger than 1 mm after it whenever every command arrived in time"; part 4 never predicts picks or own abilities (0001:191–193) | part 4's "no correction" reads with part 4's general mark (1 mm, commands in time); no new problem |
| C2 | 0001:665 | command "about 15–20 B in all (proposed: the perf review's estimate, 2026-11-02 …)" | perf review | confirmed | review-perf-auditor.md:27 "At about 15–20 B per command and a 6 B header"; no field sum there | label now meets rule 2 |
| C3 | 0001:669 | header "about 6 B (proposed: the same estimate)" | perf review | confirmed | the same line: "and a 6 B header" | the arithmetic that follows (25.9, 43.9, 43.9, 53.5 kbps) unchanged and rechecked: 108, 183, 183, 223 B × 240 |
| C4 | 0001:1047 | with S1's stand-ins at 30 Hz the planning check leaves "4.69 ms (16.67 − 9.18 − 2.80)" | GLOSSARY §1; ADR 0004 S1 | confirmed | GLOSSARY:68 server tick sum 9.18 (16.67) at 30 Hz; GLOSSARY:58 sim share 5.53 (8.33), so remainder 2.80; ADR 0004:982 S1 "the 2.80 ms remainder topped up per tick"; 0001:1038–1039 "2.80 ms at 30 Hz" | 16.67 − 9.18 − 2.80 = 4.69; unrounded 16.6667 − 8.3333 − 3.6435 = 4.6899 (3.6435 = 2.40 + 14 ÷ 30 + 0.70 + 0.0768). d1's 4.70 was a sum of rounded parts; 4.69 is the better figure. The re-flood figure (3.69) is not given; 0001:963 still gives 7.49 and 6.49 for the check without stand-ins, which is right |
| C5 | 0001:1359–1361 | V-S: "The first two clauses and the 30-inputs count hold for single-room marks only"; PM-0001-27 runs under PM-0006-13 "(no pinning, its bracket B and LG's 80 % rule, …)" | ADR 0006 PM-0006-13 | confirmed | 0006:1183 "Validity is bracket B's and LG's 80 % rule, not ADR 0001's V-S … ADR 0001's PM-0001-27 is this set of runs, read the same way"; 0001:1410 PM-0001-27 "validity is PM-0006-13's (bracket B and LG's 80 % rule), not V-S"; 0006:1184 PM-0006-14 the same | key, row and PM-0006-13 now say one thing |
| C6 | 0001:1431 | "part 4's formula gives 6.5–8.5 ticks at P3 and 8–10 at P4: (150 + 66.7) ÷ 33.3 to (150 + 133.3) ÷ 33.3 and (200 + 66.7) ÷ 33.3 to (200 + 133.3) ÷ 33.3, arithmetic" | part 4; part 13 | confirmed | 0001:181–183 "the round trip plus the input buffer plus one tick, … with a 1–3 tick buffer … and deeper at P3 and P4"; 0001:538–539 "P3 = 150 ms … P4 = 200 ms" | 66.7 = 1-tick buffer + 1 tick, 133.3 = 3 + 1; 216.7, 283.3, 266.7, 333.3 ÷ 33.33 = 6.50, 8.50, 8.00, 10.00 |
| C7 | 0002:530 | failure artefacts kept for "part 3: 30 days, proposed there; R5 leaves it to infra-engineer, r5 line 2083" | ADR 0006 part 3; r5:2083 | confirmed | 0006:171–176 "CI failure artefacts … are kept 30 days … (proposed: …)"; r5-toolchain-quality.md:2083 "infra-engineer: … artefact retention for failures (REC6)" | |
| C8 | 0006:270–271 | the supervisor answers "with the host and a port from each firewall range (part 5)" | ADR 0006 part 5; ADR 0001 part 10 | confirmed | 0006:224–225 "one UDP range for WebTransport and one TCP range for the WebSocket fallback"; 0001:402–404 "one UDP port … and one TCP port … taken from the VM's two firewall ranges … the host and both ports" | "each firewall range" means the two game ranges part 5 opens (the metrics and SSH rules are rules, not ranges); clear enough |
| C9 | 0006:272–273 | "routing joins by build is ADR 0001 part 11's, the release model OQ-0006-14's" | ADR 0001 part 11 | confirmed | 0001:495–498 "the directory records each room's build, protocol version and content hash, so the lobby can send a client only to rooms that match it (… how releases reach running rooms is OQ-0006-14)" | |
| C10 | 0006:534–535 | "the lobby sends a client only to rooms of its own build and content hash, and a refused client reloads at most once (ADR 0001 part 11 and its failure table)" | ADR 0001 part 11, failure table | confirmed | 0001:495–496 (as C9); 0001:719 "a `JoinRejected` for a mismatch reloads the client once, and only if the lobby names a newer build than its own; … so a refused client never loops" | |
| C11 | 0006:535–536 | "with no release model a new client finds no room of its build until rooms on the new binary run" | inference from C10 | confirmed (inference) | follows from 0001:495–496 with OQ-0006-14's "the client host switches production at once (inferred)" (0006:917) | |
| C12 | 0006:539 | "routing joins by build is ADR 0001's protocol (its part 11)" | ADR 0001 part 11 | confirmed | 0001:495–496 | was "(part 6)": d1's citation-off fixed |
| C13 | 0006:918 | "and the lobby sends it only to rooms of its build (ADR 0001 part 11)" | ADR 0001 part 11 | confirmed | 0001:495–496 | the sentence still opens "every old room refuses a new client with `JoinRejected`", true of a client that reaches one; it reads a little oddly beside routing, not wrong |
| C14 | 0006:925 | option (b)'s routing "(as ADR 0001 part 11's directory does)" | ADR 0001 part 11 | confirmed | 0001:494–496: the directory is ADR 0006's to run ("the room directory and the guest store are ADR 0006's") and part 11 sets its build field | "ADR 0001 part 11's directory" = the directory as part 11 specifies it; acceptable |
| C15 | 0006:953 | "A connection to an address's TCP 443 reaches one listener (no source read)" | none (labelled) | unsupported, labelled | no source in the record or the research; the label is the rule's answer | true in the form now written (with SO_REUSEPORT the kernel still hands each connection to one socket: owner's note, from memory, not read); the "so" holds. The front could route by host name (SNI) as well as by path: unchanged text, not a finding |
| C16 | 0006:1008 | "perhaps 15 to 30 RS-hours, proposed: the infra review's guess, which the run plan replaces with a sum" | infra review; PM-0006-18 | confirmed | review-infra-engineer.md:56 "ADR 0001's 3-run S30 rows add perhaps 15–30 hours, about 2–4 EUR"; 0006:1188 PM-0006-18 "planning check …: the run plan's hours at 0.1386 EUR … the arithmetic shown" | |
| C17 | 0006:1009 | "15 to 30 × 0.1386 = 2.08 to 4.16 EUR" | R6-HOST-F15 | confirmed | r6-infrastructure.md:153, 158 "CCX23 … € 0.1386 per hour, excl. VAT"; 15 × 0.1386 = 2.079, 30 × 0.1386 = 4.158 | "15 to 30 × 0.1386" can be misread as 15 to (30 × 0.1386); the result makes it clear. The conclusion still holds at the low end: 3.19 + 2.08 = 5.27 > 5 EUR before any CX33 hour |
| C18 | 0006:1179 | PM-0006-32 "2 of 2" (the reviewer gate; the `main`-only branch rule) | spike 1's B9 bullet; part 13 | confirmed | 0006:1049–1052 spike 1 names exactly these two legs; 0006:513–516 interim (a) and (b) | the three runs, control and bite stay; "1 run per case" now counts two pass cases and one reported case: consistent |
| C19 | 0006:1179 | third leg "reported, not a pass condition … (part 13 expects the account-scoped token to reach production; …)" | part 13 | confirmed | 0006:509–510 "an account-scoped token can then deploy the production player build too (the security review's inference, finding 1)"; 0006:515–516 "unless OQ-0006-13 puts production where the preview token cannot reach it" | "expects" restates "can … (inference)": fair |

Claims judged: 19; confirmed 18 (C11 as an inference), 1 unsupported but labelled as the rules ask (C15); wrong 0;
citation off 0. Arithmetic recomputed: C3, C4, C6, C17 (all right).

## 2. Blocks table

Rebuild check (verifier-fixcheck-d3a-scratch/rebuild.py): the 14 FIX blocks for ADR 0001, 0002 and 0006 in the four
touch-up files, applied to 449356c, give HEAD's three files exactly; each BEFORE occurs once; 13 blocks keep their line
count, FIX F7 adds one line (5 → 6).

| block | finding | resolved? | new problem |
|---|---|---|---|
| netcode FIX F2 (0001:188–194) | d1 F2 | yes: d1's words, "no correction after the first snapshot that carries it", now match PM-0001-60 | none |
| netcode FIX F1 (0001:1045–1048) | d1 F1 | yes: "4.69 ms (16.67 − 9.18 − 2.80)"; the owner's 4.69 against d1's 4.70 is right (unrounded 4.6899; d1 summed rounded parts) | none; the re-flood figure d1 offered (3.69) is left out, reason given (nothing turns on it); 0001:963's 7.49 and 6.49, the check without stand-ins, stay right |
| netcode FIX F13 (0001:665–672) | d1 F13, ADR 0001 half | yes: 15–20 B and 6 B both "proposed", source named (review-perf-auditor.md:27 gives both) | none |
| infra FIX F10, part 15 (0006:532–540) | d1 F10 | yes: d1's sentence as written; "(part 6)" → "(its part 11)", "its" because this record's own part 11 is data protection: sound | none |
| infra FIX F11 (0006:270–271) | d1 F11 | yes: "a port from each firewall range"; part 5 gives a game VM exactly two ranges (0006:224–225) | none |
| infra FIX F10, part 6 (0006:273) | d1 F10 (a place d1 did not list) | yes: without it part 6 would give routing by build to OQ-0006-14 while part 15 gives it to ADR 0001 part 11 | none |
| infra FIX F10, OQ-0006-14 why (0006:918) | d1 F10 | yes ("sends" for d1's "can route": same claim) | none; the sentence keeps "every old room refuses a new client", true of a client that reaches one (see C13) |
| infra FIX F10, OQ-0006-14 (b) (0006:925) | d1 F10 | yes | none |
| infra FIX F13 (0006:1007–1010) | d1 F13, ADR 0006 half | yes: the hours "proposed: the infra review's guess", the EUR shown as arithmetic (2.08, 4.16: right) | none (C17's notation is readable) |
| infra FIX F14 (0006:953–954) | d1 F14 | yes: weaker claim, "(no source read)" | none |
| engine FIX F9 (0002:530) | d1 F9 | yes: "part 3: 30 days, proposed there"; the stale "until it is set" clause gone | none |
| test-engineer FIX F7 (0001:1358–1363) | d1 F7 | yes, in the key rather than d1's row: the key, PM-0001-27, PM-0006-13, PM-0006-14 and SPIKE_PLAN:100–102 now read one set of runs one way | none: the density runs keep LG's 80 % rule with its calibration run (0006:1111–1112) and PM-0001-2's load counters, 700 shots/s ±5 % among them (0001:1381, "every run of PM-0001-4 to PM-0001-27"), so a starved generator is still caught; the added line moves no line anything cites |
| test-engineer FIX F6 (0001:1431) | d1 F6 | yes: "part 4's formula gives …" with the four quotients shown (right) | none |
| test-engineer FIX F12 (0006:1179) | d1 F12 | yes: "2 of 2", the third leg "reported, not a pass condition"; the row now matches spike 1's B9 bullet (0006:1049–1052), which names only the two legs | none |

Blocks judged: 14; with a problem: 0.

NOTEs and reasons judged (the four files hold reasoning paragraphs, no NOTE blocks):
- netcode, "0001:963's 7.49 ms not changed": sound (that sentence is the check without stand-ins: 16.67 − 9.18).
- netcode, "part 5's 'about twice R2's 8 B input' not changed": sound for this round; it points to the bandwidth
  estimate, which now carries the "proposed" label (unchanged text, not judged further).
- netcode, re-flood figure left out of OQ-0001-2: sound (optional in d1's fix).
- infra, F14 "likely too strong … SO_REUSEPORT (from memory; socket(7) was not read)": the reasoning is sound and the
  new wording does not depend on it; it stays unread.
- infra, F13 "a sum over ADR 0001's rows was not attempted (62 rows; setup hours and shared sessions not given)":
  sound; PM-0006-18's planning check is the sum.
- test-engineer, F7 "fix the key, not the row": sound (checked: 0006:1183, 0006:1184, SPIKE_PLAN:100–102).
- engine-architect, "F8 out of scope, left untouched": sound; F8 is deferred in the appendix §7.3 (see below).

d1 findings no block fixed, against the appendix §7.3 deferred list
(docs/reports/2026-11-01-v2-decision-drafts-appendix.md:591–612):
- d1 F3 (part 8's rewind estimate, the frame wait): deferred, with its reason (lines 606–607: "measured by PM-0001-49;
  changing the 233–383 ms figure now would ripple into the README and other drafts"). Listed.
- d1 F8 (OQ-0002-5's "paid minutes"): deferred (lines 594–595 and 608, "low wording"). Listed.
- d1 F15 (the synthesis): not this unit's text; done at 449356c (ENGINE_OVERVIEW has none of the three old phrases;
  docs/adr/README.md has 16 lines naming OQ-0001-15 to -18 and OQ-0006-13 to -20). Not a deferral.
- d1's optional d0 items (#5, #6, #7, #8b, "not findings"): #6 is listed (lines 596–597); #5, #7 and #8b are not.
  They were marked optional, so no finding.
- Stale in the list: lines 599–600 still defer "Part 4's 'causes no correction' may read 'no correction beyond the
  one at the first snapshot that carries it'", which FIX F2 applied in 7858762 (0001:189). Finding D3A-2.

README's line pointers into ADR 0001, 0002 and 0006 (32 rows): all 32 still land on their OQ heading (script check).

## 3. The four owner-checked verdicts

**fixcheck-d0 row 9 (ADR 0004 part 7, the precision of `rem_euclid`, `div_euclid` and the float `%`).**
- Original verdict: "wrong in part (rem_euclid, div_euclid); abs can now cite std": the draft's reason "no source read
  here states their precision" is overtaken, since std 1.99.1's `f32` and `f64` pages state "§Precision The result of
  this operation is guaranteed to be the rounded infinite-precision result" for both; the Reference states nothing for
  `%`. Proposed fix: quote std, keep both off the list until PM-0004-22 confirms.
- Owner's check (rust-reviewer-fixes-final-d.md:3): read d0's excerpt (X9), noted the `rem_euclid` excerpt is abridged
  (heading elided), "matches the std source as I know it", and applied d0's words (ADR 0004:259–262, 271–277; commit
  604a497). fixcheck-d2 confirmed the applied text against d0's row (d2 rows 59, 61), not against the code.
- The source: verifier-fixcheck-d0-scratch/excerpts.md:118–127 holds the `f32` lines (div_euclid with "§Precision",
  rem_euclid's sentence without its heading, `abs`) and the Reference's `%` line; the `f64` page's lines (d0's "txt
  313–339") were not saved, so the `f64` half rests on d0's word. Fetch audit: audit.tsv rows 4–5, HTTP 200.
- Skeptic test (no fetch: the local toolchain, rustc 1.97.0 of 2026-07-07; source and output in
  verifier-fixcheck-d3a-scratch/div_euclid_check.rs and .out): `a = 3 + 2^-22`, `b = 1 + 2^-23` (both exact in f32);
  `a = 2b + 1` exactly with `0 ≤ 1 < b`, so the Euclidean quotient is exactly 2 (representable). rustc 1.97.0 gives
  `a.div_euclid(b) = 3` and `a.rem_euclid(b) = 1`, so `q·b + r ≠ a`; the f64 analogue (`3 + 2^-51`, `1 + 2^-52`) gives 3
  too. Cause: `self / rhs` rounds up to 3.0 before the truncation. So on 1.97.0 `div_euclid` is not the rounded
  infinite-precision result; whether 1.99.1 changed the code behind its new §Precision line is unverified (no fetch).
  `rem_euclid` matched the exact remainder here (it is one rounding of an exact sum).
- Verdict holds: yes. As a reading of the source it is right: the std page states the guarantee, so "no source read
  here states their precision" was overtaken.
- Fix right: in its decision, yes (both stay off the allow-list until a test says otherwise, and determinism needs
  only that a composition of `/`, `trunc`, `%` and `±` agree across targets). In its wording, not quite: "std 1.99.1
  guarantees `rem_euclid` and `div_euclid` 'the rounded infinite-precision result' … so they may join once PM-0004-22's
  unit vectors confirm it" (ADR 0004:272–274) passes on std's claim for `div_euclid` as fact, while the toolchain one
  release earlier breaks it on a 2-line input; PM-0004-22 compares outputs across targets (`to_bits`) and only `sqrt`
  against a correctly rounded reference, so its unit vectors would not catch this unless they are told to. Finding
  D3A-1 (low, outside this unit's three ADRs; author rust-reviewer, words fixcheck-d0's).

**fixcheck-d1 F1 (ADR 0001 OQ-0001-2, the planning-check figure with S1's stand-ins).**
- Original verdict: 7.49 ms is the check without the 2.80 ms remainder; with it, 8.33 + 3.64 = 11.97, so 4.70 ms.
- Owner's check (touchup-netcode-architect-notes.md, 21:20 and 21:33): holds; read ADR 0004 S1 (0004:982) and
  PM-0004-2 (0004:1107): the remainder is topped up every tick; 4.69, not 4.70 (unrounded 16.6667 − 8.3333 − 3.6435).
- The source: GLOSSARY:53–69 (sim share 5.53 of 8.33; server-only 3.64; sum 9.18 of 16.67 at 30 Hz); ADR 0004:982
  "the 2.80 ms remainder topped up per tick"; ADR 0001:1036–1039.
- Recomputed: 3.6435 = 2.40 + 0.4667 (700 × 20 µs ÷ 30) + 0.70 + 0.0768; with the top-up the sim share is its whole
  8.3333; 16.6667 − 11.9768 = 4.6899. 16.67 − 9.18 − 2.80 = 4.69.
- Verdict holds: yes. Fix right: yes, and 4.69 is the better figure; the expression shown evaluates to it exactly.

**fixcheck-d1 F2 (ADR 0001 part 4, "causes no correction").**
- Original verdict: part 4 never predicts picks or own abilities, so the first snapshot after a server-side change
  corrects; PM-0001-60 marks "at most one correction, at the first snapshot that carries the change".
- Owner's check (touchup-netcode-architect-notes.md, 21:20): holds (0001:191–193; 0001:1428).
- The source: 0001:190–193 at HEAD lists "upgrade offers and picks" and "the player's own abilities until ABL-PREDICT
  is answered" under "Never predicted"; PM-0001-60 (0001:1429) as quoted, and its own source cell: "while picks and own
  abilities are never predicted (part 4), the first snapshot after a change corrects, so the mark is none after it".
- Verdict holds: yes. Fix right: yes (0001:189 "no correction after the first snapshot that carries it"). Stale
  leftover: the appendix still defers this wording (D3A-2).

**fixcheck-d1 F10 (ADR 0006 part 15 and OQ-0006-14 describe ADR 0001's old mismatch row).**
- Original verdict: "every room still on the old binary refuses a new client … the client's reload fetches the same
  new bundle, so it cannot recover" and "(part 6)" no longer match ADR 0001 part 11 (routing by build) and its failure
  table (one reload, only for a newer build); option (b)'s "a field ADR 0001's directory would add" is stale.
- Owner's check (touchup-infra-engineer-fixes.md:7–13, 64–75): holds; read 0001:495–496 and :719 at 449356c;
  "(part 6)" points at ADR 0001's snapshots; found one more stale place, part 6's "routing joins by build is
  OQ-0006-14's" (0006:272–273).
- The source: ADR 0001 at HEAD (unchanged at these lines since 604a497): 0001:494–498 "the directory records each
  room's build, protocol version and content hash, so the lobby can send a client only to rooms that match it (needed
  if releases run live, harmless in a release window: how releases reach running rooms is OQ-0006-14)"; 0001:719
  "a `JoinRejected` for a mismatch reloads the client once, and only if the lobby names a newer build than its own;
  otherwise the client shows the reason and returns to the lobby, so a refused client never loops"; 0001:255 part 6
  is "Snapshots, encoding and events", 0001:445 part 11 "Rooms, the lobby and bots".
- Verdict holds: yes. Fix right: yes, all four blocks (and part 6's fifth place was needed); no stale wording left in
  docs/ (grep "cannot recover", "same new bundle", "directory would add", "reload fetches": the one hit, 0006:205, is
  the staging certificate's reload, another topic).

## 4. The rules, on the changed text only

- Glossary (`python3 .checkpoints/v2-decision-drafts/scripts/glossary_check_drafts.py`, from the repository root, at
  HEAD): exact hits 0 in every draft (0001: in context 138; 0002: 59; 0006: 98, the counts the owners reported before
  and after). On the 41 changed lines one in-context hit: 0001:672 "48" in "at most 48 kbps" (a bandwidth, not the
  largest mode's N; the line was only rewrapped). Output: verifier-fixcheck-d3a-scratch/glossary-head.txt and
  glossary-hits-head.tsv.
- Rule 2, a number with no source: none left. The 15–20 B, the 6 B and the 15–30 RS-hours now carry "proposed" with
  their source (the perf and infra reviews), as d1 F13 asked; 4.69, the four replay depths and 2.08–4.16 EUR are shown
  arithmetic on cited numbers.
- Rule 3, a "not researched" option proposed as the decision: none in the changed words. (Part 15's "Interim position
  (proposed)" is OQ-0006-14's option (a), unchanged words only rewrapped by FIX F10; the question stays open.)
- Rule 6, the look: nothing in the changed text touches it.
- A settled input reopened: none (4.69 ms and the V-S key are TICK-30's 30 Hz figures; nothing reweighs 60 Hz).
- A sentence directly under a table: none (0001:1431 and 0006:1179 are rows inside their tables; the V-S key is a list).
- Cross-draft: no other doc quotes the old 7.49 ms for the stand-in case (0001:963's 7.49 is the check without them);
  docs/reports/2026-10-31-v2-research-appendix.md:192 already gives 4.69 ms (3.69 on a re-flood tick) at 30 Hz, which
  agrees.

## 5. Findings, ranked

1. **D3A-1 (low; ADR 0004, outside this unit's three ADRs; the d0 row 9 fix), `div_euclid`'s "guarantee" is
   contradicted by the toolchain.** Current text (ADR 0004:272–274): "std 1.99.1 guarantees `rem_euclid` and
   `div_euclid` "the rounded infinite-precision result" (its `f32` and `f64` pages, read by the verifier on 2026-11-02),
   so they may join once PM-0004-22's unit vectors confirm it on every target". Source: the quote matches std's page
   (d0 excerpts.md:118–124), but on rustc 1.97.0 (local, 2026-07-07) `(3 + 2^-22).div_euclid(1 + 2^-23)` returns 3,
   while `3 + 2^-22 = 2 × (1 + 2^-23) + 1` exactly, so the Euclidean quotient is 2 and `q·b + r ≠ a`
   (verifier-fixcheck-d3a-scratch/div_euclid_check.rs and .out; f64 the same). Whether 1.99.1's code differs is
   unverified (no fetch). PM-0004-22 checks outputs across targets by `to_bits` and only `sqrt` against a correctly
   rounded reference, so "confirm it" would not catch this by default. The decision (both off the list for now) is
   safe. Smallest fix: "std 1.99.1's pages state that `rem_euclid` and `div_euclid` return "the rounded
   infinite-precision result" (…), but rustc 1.97.0's `div_euclid` returns 3 for (3 + 2^-22) ÷ (1 + 2^-23), whose
   exact quotient is 2 (verifier fixcheck-d3a), so they may join only once PM-0004-22's unit vectors, that case among
   them, match an exact reference on every target". Author: rust-reviewer (the applied words are fixcheck-d0's).
2. **D3A-2 (low; the report's appendix), a deferral that is now done.** Current text
   (docs/reports/2026-11-01-v2-decision-drafts-appendix.md:599–600): "Part 4's "causes no correction" may read "no
   correction beyond the one at the first snapshot that carries it"." Source: ADR 0001:189 at HEAD, "causes no
   correction after the first snapshot that carries it" (FIX F2, commit 7858762). Smallest fix: delete that sentence
   from the bullet (or end it "(done: fixcheck-d1 F2, 7858762)"). Author: the main session (the appendix's writer).

No finding in the changed text of ADR 0001, 0002 or 0006: 0 wrong, 0 citation off, 0 inconsistent, 0 rule breaches.

## 6. Counts and verdict

- Claims judged: 19 (ADR 0001 6, ADR 0002 1, ADR 0006 12); confirmed 18; unsupported but labelled "(no source read)"
  as the rules ask 1 (C15); wrong 0; citation off 0. Arithmetic recomputed: 4 groups, all right.
- Blocks judged: 14 (netcode-architect 3, infra-engineer 7, engine-architect 1, test-engineer 3); every one resolves
  its finding; 0 add a problem; rebuilt from 449356c they give HEAD exactly.
- d1 findings no block fixed: F3 and F8 deferred with reasons in the appendix §7.3; F15 done by the synthesis at 449356c.
- Owner-checked verdicts: 4; all 4 hold; the fixes for d1 F1, F2 and F10 are right; d0 row 9's fix is right in its
  decision, its wording passes on a std claim the toolchain breaks (D3A-1).
- Verdict: **PASS WITH NOTES** (two low findings, neither in the three ADRs' changed text).

## 7. Fetch audit and not verified

- This unit fetched nothing (0 requests). Web facts rest on fixcheck-d0's excerpts (verifier-fixcheck-d0-scratch/).
- Not verified: the `f64` page's §Precision lines (d0's text dump lines 313–339 were not saved; d0's word); whether
  rustc 1.99.1's `div_euclid` differs from 1.97.0's (no fetch, no 1.99.1 toolchain); SO_REUSEPORT's behaviour behind
  C15 (socket(7) not read; the text no longer depends on it).

- HEAD moved to 5c74e7c during the check (cda0577, 5c74e7c: checkpoint autosaves); `git diff f9ec6c8 HEAD -- docs/` is
  empty, so every verdict above holds at 5c74e7c.

DONE
