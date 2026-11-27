# Waiting on Mustafa: the one-way delay change (NPT-L2-ONEWAY) is ready for his main Advisor's check and his word for the merge
2026-11-26, Netcode Builder 2. Saved as his rule of 2026-11-25 asks (the question as a file, pushed, named in the post).

## What I ask
1. That his main Advisor checks commit **c56afd07002f6c13af418c8f016d6087fe806813** on `claude/practical-ride-igo41n`.
2. After that check, his word in the netcode chat to merge that branch again (it sits on the netcode branch's merged
   head 591e2cb0, so the merge takes two code commits: 1a637408 and c56afd07, plus notes).
Due: before R1's report (his message of 2026-11-25, item 4).

## What the commit is
- **The rule (his decision of 2026-11-25, answer (a)):** on the real network (L2) the judge reads PM-0001-3's 80
  one-way delay checks against the written bound minus half the leg's own measured base round trip
  (`header.params.base_rtt_ns` of the probe's log), and prints the bound it used. No tolerance changes; the
  round-trip checks, the packet counts, the reports and every run on the model (L1) are untouched.
- **Files** (`git diff 591e2cb0 c56afd07 -- spikes/netcode-proof`: 6 files, 379 added, 6 deleted; my stop was 400):
  `crates/judge/src/eval.rs`, `crates/judge/src/marks.rs`, `crates/judge/tests/engine.rs`, `marks/README.md`
  (one bullet in 2.3), `marks/SERIES.md` (three rows: the two one-way series and `header.params.base_rtt_ns`),
  `marks/marks.json` (one field). No manifest or lockfile; nothing outside the fence.
- **marks.json:** only PM-0001-3's `notes` changed: it gains his two sentences, copied by script
  (`oneway-note.mjs`). sha256 before c3396db225b394695b6cf41460723e1d98b96f42e1d42d412976f4ca702f1b7a, after
  a41ee58398274840c20eac3f09ab144fe8bc84dda5f7c7233fd5fd7589ea24f2. No pass mark, number or tolerance changes.

## What was checked, and by whom
- **Plan:** the advisor agent, APPROVE with 4 conditions (plan-oneway.md); all four held at its read of the head.
- **One reviewer's read:** netcode-architect (netcode-architect-oneway.md): APPROVE WITH FINDINGS, five Low. The sign
  and the half are right against the network code (the probe stamps only the delay the model added, on the server's
  own clock); the round trip is measured end to end and rightly stays. Four Lows taken in c56afd07; the fifth is a
  follow-up (below).
- **The advisor agent's read of the head:** APPROVE for c56afd07.
- **In my container, on c56afd07, from `spikes/netcode-proof/`:** `cargo test -p npt-judge --locked --no-fail-fast`:
  136 passed (19 + 4 + 87 + 26), 0 failed (132 before this change); `cargo clippy -p npt-judge --all-targets --locked
  -- -D warnings`: exit 0; `cargo fmt --check`: exit 0; `cargo run -q --locked -p npt-judge -- --marks
  marks/marks.json --check-marks`: 81 marks read and checked. From the repository root:
  `node spikes/netcode-proof/check.mjs`: all 11 steps pass (test: 912 on the measured build, 1009 on the bites build,
  0 failed, 0 skipped).
- **GitHub on c56afd07:** the gate workflow (jobs gate and smoke) success, run 45895055329; the toolchain workflow
  success, run 45895055296.
- **Planted defects:** thirteen (eleven of mine, two of the reviewer's that had survived), each failing a test and
  taken back.
- **Not verified:** no real L2 probe log exists in the repository yet; R1's run gives the first.

## One follow-up for the netcode chat (Low, not for this merge)
The part named "L2 P0 (the real path, recorded)" in PM-0001-3: on P0 the probe's one-way stamps hold only the
pass-through socket's own hold time, never the real path, so the name misleads. It is a part name in marks.json
(his), for the readings change before R5.
