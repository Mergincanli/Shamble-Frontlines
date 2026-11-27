# NB2-ONEWAY: take Builder 2's one-way delay work into the netcode branch (plan, 2026-11-27 12:4x UTC)
Status: DRAFT, for the advisor agent's check in the 15:50 window (a10848a10f73ed39e, the seat that wrote NB2-C1 to
-C12; launch-1550-msgs.md, section NB). Nothing is run before that check. A delta on nb2-merge-plan.md: its steps,
numbers, stop rules and risks hold except where this file differs. The order (the advisor tool, 12:3x): UM-4's build
first, after the 15:50 seats finish; this merge after UM-4's push, in the next gap that fits, before R1's report.

## His word (history.md:8133-8151, word for word there)
- Item 3: "merge that branch at exactly that tip, by the same steps and bars as the last merge (the command named
  first and my reply to it, afterwards the whole check). Take the next gap that fits, before R1's report. If the tip
  has moved, send no command and two lines to me."
- So T is 29a901994f5b65c23a17162422770aaacc0ca44e exactly. Any other tip at step 1 or just before step 2's send: no
  command, two lines to him (as NB2-PROMPT (a): no other route).
- Item 2's facts, each re-read below; his main Advisor checked the code commit c56afd07; "Not run by my Advisor: its
  136 judge tests" (step 4 runs them).

## Facts checked (12:3x-12:4x UTC, read only, printing forms)
- `git fetch origin claude/practical-ride-igo41n`, then `git rev-parse FETCH_HEAD`: 29a901994f5b65c23a17162422770aaacc0ca44e.
- `git merge-base HEAD 29a90199…` (HEAD 7b50a66): 591e2cb0ab07d8483290c98d8f6ab8595a2621e3.
- After the code commit: `git rev-list --count c56afd07..29a90199` printed 3; `git diff --name-only c56afd07 29a90199`
  printed only .checkpoints/netcode-judge.md and .checkpoints/netcode-judge/q-oneway-merge.md ("only its notes").
- `git diff --name-status 591e2cb 29a90199`: 12 paths, the whole list. Six notes: .checkpoints/netcode-judge.md (M);
  na-plant.sh, netcode-architect-oneway.md, oneway-note.mjs, plan-oneway.md, q-oneway-merge.md under
  .checkpoints/netcode-judge/ (A). Six spike files, all M (`--numstat`): crates/judge/src/eval.rs 70/2,
  crates/judge/src/marks.rs 28/0, crates/judge/tests/engine.rs 263/0, marks/README.md 14/0, marks/SERIES.md 3/3,
  marks/marks.json 1/1 (+379/-6). No kit, CI, gate, manifest or lockfile path is in that whole list. (A pathspec form
  over the kit paths was refused at its permission prompt, 12:4x, and not retried; the whole list carries the answer.)
- marks.json: `git show 29a90199:spikes/netcode-proof/marks/marks.json | sha256sum` printed
  a41ee58398274840c20eac3f09ab144fe8bc84dda5f7c7233fd5fd7589ea24f2 (his). `git diff 591e2cb 29a90199 -- …/marks.json`:
  one hunk (:2365-2371), one line, PM-0001-3's "notes", which gains the NPT-L2-ONEWAY text (his 2026-11-25 words to
  Builder 2 and his answer (a)) and the pointer "marks/README.md 2.3"; the next object is PM-0001-4. No other field.
- This branch since 591e2cb in those folders: `git diff --name-only 591e2cb HEAD -- spikes/netcode-proof/crates/judge
  spikes/netcode-proof/marks .checkpoints/netcode-judge.md .checkpoints/netcode-judge` printed nothing; control, the
  same form over ops/operator: PROMPT-R1.md and rules.md.
- `git merge-tree --write-tree --name-only HEAD 29a90199`: one line, tree 147dec9688e561e4e112845096584938907dc1c3, no
  conflict.
- Builder 2's card at T (.checkpoints/netcode-judge.md:241-249): its whole check on its head (based on 591e2cb): 11 of
  11, test 912 and 1009, 0 failed, 0 skipped; the judge's own 136 tests (19 + 4 + 87 + 26), clippy and fmt exit 0.
  NB2's first merge read 908 and 1005 (its step 4), so this merge adds 4 and 4.
- The -47 set: `ls -d` lists …/runs/u4f-47-b57221ed139d/measured and …/bites (still unpacked).
- Free disk: 8,008 MiB (12:4x), under the 8,192 bar.

## Steps (nb2-merge-plan.md's numbers; only what differs; each command its own call, printing forms only)
1. a. T as above, else no command and two lines to him.
   b. `git merge-base c56afd07 T` prints c56afd07… (replaces `--is-ancestor` exit 0); `git rev-list --count
      c56afd07..T` = 3; `git diff --name-only c56afd07 T` only Builder 2's notes. Anything else: no merge, two lines
      to him.
   c. `git diff --name-status 591e2cb T`: exactly the 12 paths above; more: stop, the advisor rules.
   d. `git merge-base HEAD T` = 591e2cb0….
   e. merge-tree: one tree line, no conflict.
   e2. marks.json at T: the pipe hash a41ee583…, and its diff against 591e2cb the one hunk above; else stop, to him.
   f. Free disk at least 8,192 MiB: the approved dev part-clean first (UM-4's run leaves it under again), measured.
   g. The saver stopped by TaskStop, then `pgrep -c -f checkpoint.mjs` printing 0 (replaces `pgrep -f` exit 1).
   h, i. As before: the card's resume line, H0, then .gate/nb2b-merge/light-base.log (refs, marks, self, redact).
2. The command named to him first, the board line set, the turn ended, his reply awaited (NB2-PROMPT (a)); T read
   again just before sending: `git merge --no-ff -F /home/user/Shamble-Frontlines/.gate/nb2b-merge/merge-msg.txt
   29a901994f5b65c23a17162422770aaacc0ca44e`, the message file naming his word (history.md:8133-8151), T and c56afd07.
3. Content checks: `git rev-parse HEAD^1 HEAD^2` = H0 and T; `git diff --stat H0 HEAD` lists the same 12 paths and
   counts as `git diff --stat 591e2cb T`; `git diff --name-only T HEAD -- spikes/netcode-proof/crates/judge
   spikes/netcode-proof/marks .checkpoints/netcode-judge.md .checkpoints/netcode-judge` prints nothing, with the
   control `git diff --name-only 591e2cb T -- spikes/netcode-proof/marks` listing the three marks files;
   `sha256sum spikes/netcode-proof/marks/marks.json` = a41ee583….
4. The whole check, the part-clean first if under 8,192, counted as cold: `bash .checkpoints/netcode-proof-test/main-run.sh
   /home/user/Shamble-Frontlines/.gate/nb2b-merge/check.log env CARGO_INCREMENTAL=0 node check.mjs`. Bars, on UM-4's
   counted run (its push lands first): every step passes (13 with UM-4's `ops`); test = UM-4's + 4 each (931 + 4 =
   935 and 1,028 + 4 = 1,032 if UM-4 reads U8-F1's M5 numbers); 0 failed, 0 skipped; the judge's 136 (19 + 4 + 87 +
   26) read from the log's npt-judge test lines, or, if the log does not show them, one `cargo test --locked -p
   npt-judge` through main-run.sh; marks 81 read, rows unchanged; self as UM-4's; ops and wrappers as UM-4's; redact 0
   findings, its items step 1i's + 5 (the five added notes); refs step 1i's N0 + 5; wasm as UM-4's.
5. The -47 re-judge, kept ("the same steps"), as before: both calls MOVED.
6-9. As before: gate PASS; nothing edited by hand; the save pushes the merge, CI on the pushed head (gate, smoke and
   toolchain green, PR #40 with no conflict); the saver restarts; Builder 2 is told the merged SHA; the advisor's read
   of the pushed head in the next window; and a reviewer's read of `git diff 591e2cb T -- spikes/netcode-proof/crates/judge
   spikes/netcode-proof/marks` before the merge's report (his rule of 2026-11-15, history.md:2127; NB2-C10's form).

## Change type, tier
A merge of another lane's branch, its code commit checked by his main Advisor, nothing edited by hand: 12 files, six
of them spike files in the judge's code and marks/ (+379/-6). Risk tier 1 (spike code and notes). No Brave check.

## Questions for the checker
- Q1: each of NB2-C1 to -C12 (advisor-nb2-merge.md): carried here, or moot (and why)?
- Q2: the reviewer's read of the judge and marks diff: who, and when (before the report, or before the push)?
- Q3: step 4's bars on UM-4's counted run, as the order puts UM-4 first: right? If UM-4 has not landed when a gap
  comes, may this merge go first on U8-F1's M5 numbers (then UM-4's bars take this merge's run)?
