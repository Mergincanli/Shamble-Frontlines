# Skeptic, review round 1, D7 (= IE-3, Low): head 4d78ad6

Plan: (1) re-read toolchain.yml at 4d78ad6 (line 3's "2.3 replaces this file", the cut at :60-88) and what the cut
actually wraps; (2) re-read ADR 0002 part 9 at the head (:569-593, G9, G12, option G1) and PM-0002-37 (:1303) to see
whether either already requires the cut beyond 2.2; (3) find the report the finding cites (:156-179, "Findings for
2.3") and docs/STATUS_ARCHIVE.md:551-640, and grep the whole tree at the head (docs, plan, checkpoint, STATUS, ROADMAP,
DECISIONS, kit) for anything that does carry the cut, the offline proof or PM-0002-37 into 2.3 (CARGO-NET-OFFLINE
included); (4) check the CI log of the head's toolchain job for what the cut proves; (5) check the fix's premise (his
A10: any other change to the CI job asks him again) in his recorded words; (6) judge holds, severity, and the fix.
Nothing in the worktree changes but this file; scratch work goes in .gate/skeptic-D7.

## Checks
- C0 (setup): the worktree head is 5ad4e55; 4d78ad6 is its ancestor and the diff 4d78ad6..5ad4e55 touches only
  `.checkpoints/` (18 files), so every file the finding cites reads the same at the reviewed head. All reads below are
  `git show 4d78ad6:<path>` (copies in .gate/skeptic-D7/).
- C1 toolchain.yml at 4d78ad6: :3 reads "Step 2.3 replaces this file with the gate v2's workflow." [holds]; :60-88 is
  the cut step (control curl before, `sudo -n unshare --net`, setpriv, env -i, probe, then cargo test, cargo test
  --release, the wasm32 build, npm ci --offline, npm test, tsc --version) [holds].
- C2 ADR 0002 at 4d78ad6: part 9's table is :569-582; G9 :578 is "`cargo test --workspace` (...), Wasm tests in Node,
  Vitest" with no network clause; G12 :581 ends "every outbound request fails the run except to the local test server"
  (the end-to-end run only); option G1 :593 lists the jobs (Rust native, Rust wasm32, TypeScript, Playwright,
  determinism) and one aggregate check, no cut [holds]. PM-0002-37 :1303 is under "The foundation's acceptance checks",
  "(part 2; phase 2.2)", frequency "1 run"; its method names the cut ("the tests with the network cut off") [holds].
  A grep of the whole ADR for offline/cut/namespace/unshare/no network finds the cut only in PM-0002-37 (:1303); part 2
  (:157-181) has none; the migration plan's 2.2 entry (:1450-1456) has the offline proof in its "Done when" and 2.3's
  (:1457-1464) has none. Compare PM-0002-38 :1304, "every run; the lint pair once": the ADR does mark standing checks
  as such, and PM-0002-37 is not one.
- C3 the research it traces to: R5-LAYOUT-REC1's mark (r5 :1166-1169, "A spike must prove (proposed)") and R5-CI-REC2's
  marks (r5 :1532-1535, "Spike must prove (proposed)") are one-time proofs, not standing CI rules [consistent with
  "1 run"].
- C4 the report docs/reports/2026-11-12-foundation-2-2.md at 4d78ad6: "Findings for 2.3" is :156-179 (21 rows); none
  keeps the cut or names PM-0002-37's fate when toolchain.yml goes [holds]. Two rows presuppose the cut: CARGO-NET-OFFLINE
  :177 ("`CARGO_NET_OFFLINE=true` inside the cut, once third-party crates exist") and TWO-PLAYWRIGHTS :175 ("no real
  browser ran inside the cut yet"); REQUIRED-CHECK :169 says 2.3's aggregate check replaces `toolchain`. Risks :145-146
  speak of the cut in the present only.
- C5 docs/STATUS_ARCHIVE.md at 4d78ad6, "From foundation 2.2" :551-640: no item keeps the cut [holds]; CARGO-NET-OFFLINE
  :627-628 and TWO-PLAYWRIGHTS :620-621 presuppose it; REQUIRED-CHECK :597-599 repeats "step 2.3 replaces
  `toolchain.yml` with the gate v2's workflow (the file's header says so)".
- C6 a tree-wide `git grep` at 4d78ad6 (outside legacy/ and .checkpoints/) for "network cut|the cut|cut off|unshare|
  offline proof|PM-0002-37|no network access|network namespace": the cut appears only in toolchain.yml, the report,
  STATUS_ARCHIVE (the two presupposing rows), REPO_MAP :20 and :146 and ROADMAP :119 (both describe 2.2's job as it is),
  ADR 0002 :1303 and r5 :1167 (the one-run mark), and DECISIONS.md :134, :468, :479 (his 2.2 words). Nothing states that
  the cut, or PM-0002-37's proof, continues after toolchain.yml is replaced [holds].
- C7 his words: DECISIONS.md :479 (A2, 2026-11-10 19:43:00 UTC) "One rule for the network cut: if the runner refuses it
  and one fix round does not solve it, do not engineer further. Fall back to cargo's and npm's own offline flags, record
  PM-0002-37 as partly proven, and tell me." It is scoped to proving PM-0002-37 in 2.2; it neither keeps nor drops the
  cut later. No decision of his about the cut after 2.2 exists, so the finding misrecords nothing of his.
- C8 the fix's premise: the report's A10 item 2 (:344, his words of 2026-11-12 16:28:28 UTC) ends "Any other change to
  the CI job asks me again." [holds]: the fix's "No edit to toolchain.yml now" is right.
- C9 CI at 4d78ad6 (GitHub REST, no token): 4 runs, all success: toolchain push 43312017234 (job 131510223737) and
  pull_request 43312027194 (job 131510257155), gate 43312016939 and 43312026745. Both toolchain logs (copies in
  .gate/skeptic-D7/logs/): "control: connected to 20.169.109.160:443 before the cut" / "140.82.114.4:443";
  "uid=1001(runner) gid=65534(nogroup) groups=65534(nogroup)"; only `1: lo` UP; "the network is cut: no TCP connection
  to ...:443 (curl exit 7)"; then cargo test (debug and release), the wasm32 build, `npm ci --offline` ("added 46
  packages"), Vitest, Playwright's 1 test, `tsc` "Version 6.0.3". So PM-0002-37's proof holds on this head; the finding
  is about what happens to it after 2.2, not about 2.2's proof.
- C10 PR #48 (REST): draft, head now 20778e3, 18 commits ahead of 4d78ad6, all 20 files under `.checkpoints/` (compare
  API): the cited evidence is unchanged at the PR head. Its body mentions the cut only as 2.2 evidence (body lines
  19-21, 26-27); nothing about 2.3.
- C11 the plan at 4d78ad6 (`.checkpoints/foundation-toolchain/plan.md`, which the report keeps readable at the last
  checkpoint commit): :268 repeats "2.3 replaces this file with the gate v2's workflow (part 9, option G1)"; its other
  PM-0002-37 lines are 2.2's proof and fallback. findings-2-3.md / findings-2-3-c4.md and the checkpoint: nothing keeps
  the cut after 2.2. ADR 0002's status (:3, :7): "Proposed", accepted only for part 1; so PM-0002-37's "1 run" and G9
  without a cut are draft text, not a decision of his that already settles the cut's fate.
- C12 the fix's reach: STATUS.md :96-99 lists every 2.2 finding for 2.3 by name, and the report's table heading (:156)
  says their full text lives in STATUS_ARCHIVE "From foundation 2.2" (whose header :551-560 also tags each item
  [verified]/[assumed]). The fix names only "Findings for 2.3" (the report). No script reads these lists (grep of the
  notes' .mjs/.sh/.js, tools/ and .claude/scripts/ for the table heading and the row names: no hit), so a one-place edit
  breaks nothing mechanical, but the three lists would drift.
- C13 the fix's scope (corrected after the advisor tool's check): in the fix's "(the control, the probe, curl exit 7 or
  28 only)", "only" binds to the exit codes, as in toolchain.yml:6 ("only curl exit 7 or 28 counts as cut"); it does
  not say "keep only these three parts", so I drop that objection. What PM-0002-37 proved is today's commands inside
  the cut (toolchain.yml:82-87: cargo test, cargo test --release -p sim -p protocol, the wasm32 build of five crates,
  npm ci --offline, npm test --workspaces, tsc --version), not all of G6 (G6 also names native release builds and Vite
  builds that never ran in the cut). "Around its build and test jobs" in option G1 (ADR :593) would also cover the
  Playwright end-to-end job and the determinism job's browser runs (G10 :579), where a browser inside the cut's
  privilege drop is unproven (TWO-PLAYWRIGHTS, report :175, archive :620-621) and G12 (:581) has its own outbound rule:
  a scope 2.3 must settle, not a flaw in the fix. The fix's either/or (keep, or drop by decision citing the one-run
  mark) is right. D4 and D5's skeptics both return holds, Low (skeptic-r1-D4.md :103-104, skeptic-r1-D5.md :94-95), both
  as 2.3 findings; if they are recorded, a kept cut takes D5's hardening and D4's stated limit with it. "No edit to
  toolchain.yml now" matches A10 item 2 and the review budget (a Low rides another push).
- C14 (the advisor tool asked for it: phrase greps miss hard-wrapped lines, e.g. ADR :1453-1454) a word-level pass at
  4d78ad6 over docs/, .claude/ (agent files included) and .github/ for -w offline|hermetic|cut|netns, every hit
  scanned, including the 136 hits in r1-r4, ADRs 0001/0003-0006, design/, the October reports, LEGACY_INVENTORY and
  reference/: the hits are offline play (design), session cut-offs, the 2.2 job's own description, his 2.2 words, the
  two presupposing rows, and ADR 0006 :1492 ("CI stays on free standard runners"). No line keeps or drops the cut for
  2.3 or the gate v2 [the negative holds].
- C15 the likely rebuttal, checked: option G1's aggregate job "compares the set of check ids that ran with the gate's own
  list, so a check cannot drop out of CI silently" (ADR :597-598). The cut is no check id (G0-G13 have none for it), so
  this guard would not see the cut go; it strengthens "holds".

## Verdict
Holds, Low. Verified facts: every cited line checks out at 4d78ad6, unchanged at the PR head 20778e3 (only
`.checkpoints/` differs): the header's "2.3 replaces this file" (:3); no cut in ADR part 9's G9 (:578) or option G1
(:593); G12's outbound rule covers the end-to-end run only (:581); PM-0002-37 "1 run" (:1303); no row in the report's
"Findings for 2.3" (:156-179) or the archive's list (:551-640) keeps the cut, while CARGO-NET-OFFLINE (:177) and
TWO-PLAYWRIGHTS (:175) presuppose it. Nothing else in the tree, the plan, the checkpoint notes or PR #48's body carries
it (C6, C10, C11, C14), and the ADR's one guard against silent drop-out counts G-ids only (C15). Both head runs prove the
cut (curl exit 7). The inference ("2.3 could drop it without anyone deciding to") is plausible but not certain: the
counterweights are that "1 run" sits in a Proposed ADR (so a drop would follow the draft) and that 2.3's CI edit asks him
anyway (the deletion would be visible, if not named as a choice). Low, not Medium: 2.2's proof holds; nothing built or
recorded is wrong; no word of his is touched (his network-cut rule, DECISIONS.md :479, is 2.2-only); the reach is one
handover item for 2.3.
Fix: right in substance and direction, and breaks nothing. Needed for completeness: CUT-CARRY in all three places, the
report's table, STATUS_ARCHIVE's "From foundation 2.2" full text (with its [verified]/[assumed] tag) and STATUS.md
:96-99's names. Optional: tie CARGO-NET-OFFLINE and TWO-PLAYWRIGHTS' cut clause to it; state the scope by today's
commands (toolchain.yml:82-87), with browser jobs unproven (TWO-PLAYWRIGHTS) and G12's own rule. Docs only, Appendix A
untouched, no script reads the lists; it rides another push (D1's), never a push of its own.
