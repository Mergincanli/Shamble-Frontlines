# Advisor agent: MUL-ADD (cs3a round 1, code head 5c923119, main 1ec6f6c), 2026-11-24
**MUL-ADD: (a), on seven conditions.** The rule stays Mustafa's: 3a builds the interim ban now and merges only after
his own answer to MUL-ADD (any option); an answer that changes 3a's code is a new head (my review, then his go). No
word of the drafts or the research changes in 3a: this is a new departure, not on his DEPART-3 list; after his answer
DECISIONS records it and part 7 gets a dated pointer at its next revision.
1. Scan: part 7's list stays word for word (`ALLOWED`, 14; `the_lists_are_the_draft_s` unchanged); the ban is its own
   one-name list with its reason (MUL-ADD), so his answer removes one line; the control at `rules_scan.rs:357` drops
   `mul_add`; bites `x.mul_add(y, z)` and `f32::mul_add(a, b, c)` join, each found once by name.
2. Inputs: test-engineer's constructed cases and their sign and scale mirrors are added (CASES grows past 10,000),
   never swapped for seeded ones; the generator asserts each is a true double-rounding case; `--check`; the new sha256.
3. WebAssembly (Node, Chromium; release, dev): `mul_add` stays checked on every input, the reference except at the
   pinned cases, each checked by case at its measured value; any change fails loudly (a toolchain fix reopens the ban;
   lifting it is his word). The pin is labelled a known toolchain defect, never evidence.
4. Natively `mul_add` stays in the must-equal list; the test prints whether the CPU has FMA (on failure too): without
   FMA it may fail, for a true reason (two servers would differ); that path ran nowhere, as the report says. The plant
   "mul_add through f64" is caught natively and in Wasm.
5. Round 2, a line each: rust-reviewer, whether another allowed operation's native code picks its path by CPU (if so,
   to me before the push); test-engineer, the hard-case classes present for `floor`, `ceil`, `trunc` and `round`.
   F-MULADD (High) counts as fixed once round 2 confirms 1 to 4: MUL-ADD is then a decision, not an open High.
6. The question goes in the chat's next "Decisions needed" (the build does not wait) and in the report, holding: the
   question in plain words; why (allowed operations must give the same bits on server and browser; without FMA two
   servers may differ, untested); the values, where measured, that nothing uses it, and that Rust promises one
   rounding (a toolchain defect); what 3a did meanwhile; options (a) keep the ban, the sim writes `a * b + c`, part 7
   loses it by a dated pointer, (b) our own exact fused multiply-add, the same code everywhere, only when a formula
   needs it, (c) allow it, FMA required on servers, browsers differ (an exception to part 7's identity target and to
   PM-0004-19); recommended (a), free today; the limit (our code, not a library's); the merge waits for his answer.
7. Follow-ups (report, STATUS): step 2.3's lint ("this list's complement") carries his answer, so `mul_add` never comes
   back silently; when an outside crate enters `sim` (part 10's physics), a check that the players' Wasm has no `fmaf`.
**My build-plan condition** ("checks exactly ADR 0004 part 7's allow-list", lines 950-953, in the plan he approved as
written on 2026-11-17) now reads "part 7's list exactly, less `mul_add` until his MUL-ADD answer"; its purpose (nothing
through that the lint would refuse, no claim the draft does not make) holds under 1 and 6. The report quotes it.
**PM-0004-22** (section 9's row, the report): "in part: `sqrt` bit for bit and equal to a correctly rounded reference |
3a step 5 | natively, Node and Chromium, on 10,000 inputs plus the constructed cases; beyond the mark, part 7's other
operations too, not `mul_add` (exact natively with FMA, not in Wasm: pinned, banned, MUL-ADD); not yet: the `libm`
clause and reports, the three helpers, Firefox, WebKit, Safari, 20 runs on one commit unless shown." The card's "all
38 float operations passed" goes; PM-0004-23's row adds "the scan also bans `mul_add` (MUL-ADD)".
**Why (a).** (b) is not ours: it keeps an operation the tests disprove and drops the case (his words of 2026-11-19,
13:36:37 UTC: "I do not take tests out to fit a number") or hides a failure. (c) is not needed: his 11:03:53 UTC words
stop the build only at the resume check; (a) changes no word of his and leaves main to him. Part 7 allows `mul_add` as
"exactly specified on every target" (line 300); its required same-build level (line 257) is at stake too, since the
native `fmaf` picks its path by CPU at run time.
**Not verified by me** (I ran nothing): the runs (read in the notes and the probe's source); the arithmetic by hand
only (a·b + c = 2^-126 − 2^-150 − 2^-196, under the midpoint: 0x007FFFFF; through f64 the midpoint, ties to even:
0x00800000); `fmaf`'s source and dispatch; a CPU without FMA; 20 runs of the vectors.
