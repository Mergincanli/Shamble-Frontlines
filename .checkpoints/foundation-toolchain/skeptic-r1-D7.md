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
  (:157-181) has none; the migration plan's 2.2 "Done when" (:1453-1458) has the offline proof and 2.3's (:1459-1465)
  has none. Compare PM-0002-38 :1304, "every run": the ADR does mark standing checks as such, and PM-0002-37 is not one.
- C3 the research it traces to: R5-LAYOUT-REC1's mark (r5 :1166-1169) and R5-CI-REC2's marks (r5 :1532-1535) are both
  "(A) spike must prove (proposed)", one-time proofs, not standing CI rules [consistent with "1 run"].
- C4 the report docs/reports/2026-11-12-foundation-2-2.md at 4d78ad6: "Findings for 2.3" is :156-179 (19 rows); none
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
