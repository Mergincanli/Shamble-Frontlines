# security-auditor: answers to U7's Block 0b questions (u7-0b-security-request.md)

Reader: security-auditor (Opus 5.5), 2026-11-25. Branch claude/netcode-proof-test-plan-hcky5g (main folder, not
switched). Inputs read: u7-0b-security-request.md, u7-block0b.md, dep-check-0b-draft.md, the logs under .gate/u7-0b/,
my read security-u7-deps.md (items (7)-(10), binding check :281-316), security-r1-deps.md B4 (:104-123) and its pins
(:234-237, :276-277), .claude/CHECKPOINTS.md §3 (:60-69), .claude/scripts/checkpoint.mjs (:17-24, :114-140).
Nothing installed, fetched or built; only reads, `git diff`/`git log`, `sha256sum` and `ls`. No Edit tool in this
agent: this file is rewritten whole with Write at each block.

## Q2. May step 8's commit go before step 7? Verdict: (a), with four conditions

Why it is safe:
- The hold binds content, not order. checkpoint.mjs:120-140 releases a held file only when its sha256 in the pushed
  commit equals a `<sha256>  <path>` line of that commit's dependency-check.md (CHECKPOINTS.md §3, :60-69). A later
  change to any of the three files (for example a new wasm-bindgen version, should the CLI fail) gets a new hash and is
  held again until a new answer. So releasing now cannot let a later, different manifest through.
- Today's files equal the draft's three lines (`sha256sum`, read now): Cargo.toml bd1e2335...f59a, Cargo.lock
  55604a5f...9f5d, crates/client-wasm/Cargo.toml d8af72f7...000e; `git diff` shows exactly item (7)'s lines (+3; +6 -1)
  and item (8)'s four lock lines in npt-client-wasm's block, and nothing else under spikes/.
- Step 7 changes no repository file: the CLI is a binary under .gate/ (`--root`), and its download lands in
  ~/.cargo/registry. Nothing the hold guards depends on it. What R1-R4 build is the wasm-bindgen crate 0.2.130, which
  is already locked with its checksum (Cargo.lock:2045 a0181c14...3d1c, equal to the cached .crate's sha256, read now,
  and to the index per security-u7-deps.md's step 5 notes) and was cleared by items (7)-(10).
- Steps 0-6 and the redact step all passed as written (u7-block0b.md; the logs read).
- The order 7-then-8 in my binding check (:313-316) was a sequence, not a dependency: I did not foresee that the CLI's
  .crate would not be cached. Moving 8 before 7 is a difference from (7)-(10) in order only; C10 brings it here, so this
  answer is the ruling.

Conditions:
1. The section says plainly that step 7 is open: in the draft's last header line, "The CLI (step 7): not yet
   installed; under security-u7-0b-answer.md Q1; recorded in an addendum below after its install". It names this file
   with each `<answer>` filled by the verdict letter and its condition in a few words.
2. The addendum after the install is `#` lines only (the CLI's `--version` line, the .crate's sha256 and path, the
   install log's name, the count of crates downloaded). Its sha256 is written in a `#` line, never as a bare
   `<sha256>  <path>` line, so the record's only bare lines stay the repository files the hold reads (the parser
   ignores other lines, checkpoint.mjs:126, but the PR review reads the record).
3. Nothing runs the CLI (build.mjs, T5 (ii), any `wasm-bindgen` call) until that addendum is committed with
   `wasm-bindgen 0.2.130` and the .crate sha256 c37f6904387d19a319525c74af274c73912f3871c35555aa49f3ea64ebb2fc7a.
   If the install fails or either value differs, stop and bring it here before any use; the manifests and lock stay as
   committed unless a new answer says otherwise.
4. The draft's order otherwise stands: re-hash equal to the three lines and `git status --short -- spikes/` those three
   only, the gate PASS, the commit, the section by Edit, then the save, which must say "saved and pushed" (never a
   push by hand).

Findings for Q2: none.

## Q1 and Q3
In progress. This file ends with the line DONE only when all three answers are in.
