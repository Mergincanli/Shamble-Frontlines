# Advisor: NB2-MERGE step 9, the pushed head by SHA (2026-11-26, 20:0x UTC)
Merge M = 48efc9c59b06347c79d0e9ecbdcc78c6992eebde (parents H0 22cd65d7d32f654742753867712883910429722f and
T 0e621dae6e5b009a4cf00b6401b46d771b2215a3). Pushed save S = 23e9e92d11a8bc78f7aa9d0a2862790f1e7d0736.
Branch head at launch 1fa5410e13651dec4db480610a59ac138083155c; PR #40's head at my read f8c50434a53d (two notes saves
later). Form: MAIN-65 step 7. Read only: commits by SHA; from disk only .gate/nb2-merge/, .gate/u8-merge/ and the notes;
CI through `gh api` (read only). No fetch, cargo, tests or gate.

## 1. Verdict: APPROVE WITH FINDINGS (all Low) for M 48efc9c and S 23e9e92
The merge is exactly his word, and nothing else:
- M's tree is the tree a fresh merge of its two parents gives (5565f253 both ways).
- H0 to M is the same 46 paths and +6,951/-208 as e2c0e896 to T. Every spike file equals T's. SERIES.md differs from
  T's by U6c's 51 added lines alone. marks.json is T's (blob bd93d3d, sha256 c3396db2…).
- The guard's full pathspec, plus tools and every manifest and lockfile, is empty over H0..M. Every file mode is 100644.
- S and every commit up to PR #40's current head touch only .checkpoints/.

His two checks passed in his order, each on its bar fixed before the run:
- **The whole check:** 11 of 11 steps; 908 and 1005 tests, 0 failed, 0 skipped; refs 2037 = N0 2005 + 32; redact 431 =
  421 + 10 items, 0 findings; marks 81 unchanged; self 78 of 78; wasm as U8's run.
- **The -47 re-judge:** MOVED on both calls, exit 0, with the exact lines the plan check predicted (3 runs, 2 runs).
  Both out files read "verdict": "MOVED".

CI on S is green: gate, smoke, and toolchain on its one re-run (I read the check runs and the failed attempt's log
myself). Risk tier 1. Nothing for Mustafa to check in Brave.

## 2. The four departures: each ruled
1. **Step 4's bars re-based on U8's merge run: accepted.**
   - The last code commit before H0 is U8's merge 05c63a8: `git diff --name-only 05c63a8 H0` outside .checkpoints/ is
     empty.
   - U8's re-run (.gate/u8-merge/m5-check-r.log:952, :958) read 861 and 958 tests and 421 redact items. The base light
     run at H0 read redact 421 too.
   - NB2-C1 tied the +47 to 0b's counts because 0b was then expected right before NB2. U8's merge came between, so U8's
     run is the right base.
   - The bars were written down (step1-record.txt:15-17, 17:46) before the run started (18:05). The results met them
     exactly.
2. **Step 8's two bars, re-read: accepted.**
   - (a) "PR #40 mergeable" read as "no conflict path beyond docs/STATUS.md and docs/STATUS_ARCHIVE.md". H0 already had
     exactly those two conflicts with main b9aad01 (my merge-tree on H0). S and 1fa5410 show the same two, and nothing
     since H0 touched docs/. So NB2 added no conflict. "Mergeable" was out of NB2's reach: PR #40 reads mergeable false
     and dirty now (gh api).
   - (b) Push runs only: pull-request runs do not start while #40 conflicts (main-66-plan.md:44-45).
   - Both were named to him with step 2 (step1-record.txt:19-20).
3. **`-F` in place of three `-m`: accepted.**
   - His go of 18:01 (history.md:8123-8131) was for the command as named, which was the -F form
     (step1-record.txt:21-23).
   - It is still "one git merge --no-ff of 0e621dae… and nothing else" (his NB2-PROMPT (a), history.md:7965).
   - M's message equals merge-msg.txt byte for byte, both attribution lines included.
4. **Toolchain's one re-run: accepted.**
   - Attempt 1 of run 45876736642 (push, S; job 140105605787) failed in the archive step at `curl: (22) The requested
     URL returned error: 504` (18:34:47). That is the download itself, before the size or sha256 check ran: GitHub's
     server, not the code and not a hash mismatch.
   - Attempt 2 (job 140106992032) succeeded at 18:40:52. One re-run is PR #40's rule.

## 3. NB2-C1 to NB2-C12 on the pushed head
| Condition | Status | Evidence |
|---|---|---|
| C1: 0b first; clean tree; the +47 base | Met | 0b 2701220 (11-25 11:37) and its amendment 56b3e1b, then MAIN-66/67 and U8 05c63a8, all before H0 22cd65d (11-26 17:40). The main-run.sh headers of 1i, step 4 and step 5 read the same empty diff and untracked hashes (e3b0c442…); 1h "= origin, clean". The base: departure 1. |
| C2: the gap re-projected | Met | 1f read 13,918 MiB (≥ 8,192), so no clean and a warm run. Step 4 took 23.4 min (18:05:36-18:28:58) against the 23.5 projected. Every lane-lock take was "idle after 0 s". |
| C3: named, his reply, then send | Met (board not readable) | Named 17:4x (step1-record.txt:21-23); his "go" 18:01:32; sent 18:02 after the tip re-read; M at 18:05:11. The needs-musti board thread and the prompt itself were not readable by me. |
| C4: the stop rules | Not triggered | No step went off its bar. |
| C5: the part-clean form | Not triggered | Free: 13,918 at 1f, 13,733 at step 4, 7,711 at step 5 (≥ 4,096). |
| C6: step 5's exact bar | Met | Both logs: exit 0, `env CARGO_INCREMENTAL=0`, the exact lines with 3 and 2 runs. Both JSONs: "verdict": "MOVED" and moved.to "Slice 1 change-set 7 (Lag)". |
| C7: tree anchor, kit pathspec, bars | Met | (a) step1-record.txt:12 tree 5565f253 = M^{tree} (my read). (b) Full pathspec over H0..M: empty (my read). (c) redact 431, refs 2037, self 78. |
| C8: a moved tip | Not triggered | T unchanged at 1a and before sending. His NB2-PROMPT (a) overrode C8's route anyway (plan :14-18). |
| C9: the risk line | Met by reference | The plan's "As approved" block (:3-18) says the check's words govern. Step 4 ran the cross-reads and they passed. |
| C10: a reviewer's read of 67892d39..72651a3c | Open | The test-engineer's read (review-nb2-c10.md) is planned; it binds NB2's report, not this head. |
| C11: -57 named as not verified | Open | Binds NB2's report (none written yet). |
| C12: text fixes | Met by reference | The As-approved header. Plan lines 57, 102 and 138 still read the old words (ADV-NB2H-2). |

## 4. Findings (CLAUDE.md's scale)
| Id | Severity | What | Fix |
|---|---|---|---|
| ADV-NB2H-1 | Low | Step 6's gate output is not kept in .gate/nb2-merge/: only "GATE PASS" in S's message and on the card. CI's gate and smoke jobs on S (success) stand for it, since NB2 changed no .github or gate file. | NB2's report says so; no re-run |
| ADV-NB2H-2 | Low | The plan's old lines (57 "33 table rows", 102 "may ask", 138 the `--abort` reason) stand under the As-approved header instead of being corrected in place. | NB2's report uses the corrected words |
| ADV-NB2H-3 | Low (note) | No pull-request CI has run on NB2's content yet: PR #40 is dirty until MAIN-68. | MAIN-68's step 7 reads the pull-request runs once the conflict clears |
| ADV-NB2H-4 | Low (open) | NB2-C10's reviewer read and NB2-C11's not-verified line are still to come. | NB2's report waits for review-nb2-c10.md |
| ADV-NB2H-5 | Low (note) | DISK-PACK (b) kept u4f-47-b57221ed139d "until the -47 re-judge after NB2's merge" (card's disk line). That re-judge is now done. | The card's disk line: its pack may follow, by his DISK-PACK route |

## 5. What I did not check
- I ran no gate, cargo, test or light run, and did not re-run the re-judge: the numbers are read from the logs in
  .gate/nb2-merge/.
- Whether the guard's prompt appeared and he approved it there, and the needs-musti board thread: not readable here.
  His "go" in the chat is recorded word for word.
- The toolchain job's success log (attempt 2): only its conclusion and job id.
- MAIN-68's work in the main folder: not read (no working-tree read of a tracked file).

## 6. Notes (one line per check, kept as written)
- M: parents 22cd65d7 + 0e621dae; subject "Merge claude/practical-ride-igo41n (Netcode Builder 2's work) at 0e621dae";
  author Claude, 18:05:11; message = merge-msg.txt byte for byte, with both attribution lines.
- Chain M -> S 23e9e92 (no [skip ci]) -> fc6a095 -> ed02073 -> 590246f -> 0498241 -> e25188e -> 5e8519b -> 591e2cb ->
  07cb835 -> 1fa5410 -> 9ceecb4 -> f8c5043 (PR #40's head at my read). Every diff after M: .checkpoints/ only
  (M..S: the card, history.md; S..1fa5410: 8 notes; 1fa5410..f8c5043: 4 notes, my plan file among them).
- `git merge-tree --write-tree H0 T` = 5565f253…; `git rev-parse M^{tree}` = 5565f253…: equal.
- H0..M: `--shortstat` 46, +6,951/-208; `--name-status` the 46 paths of the plan check, 32 A and 14 M; `--raw` spike
  blobs equal those seen at e2c0e896..T; SERIES.md eddad09 -> f28f2ae; marks.json f9aab0d -> bd93d3d (= T's).
- T..M on the judge, marks.json, marks/README.md and netcode-judge*: empty. SERIES.md numstat 51 0.
- Guard pathspec plus tools, Cargo.toml, Cargo.lock and package*.json over H0..M: empty.
- H0's history: U6c 17964a3 (11-25 04:54), 0b 2701220 (11-25 11:37), main 6a4de15, 0b amendment 56b3e1b, main
  07cc9b7, U8 05c63a8 (11-26 05:43), then notes to H0 (17:40). 05c63a8..H0 outside .checkpoints/: empty.
- step1-record.txt: 1a-1i as planned; 1i N0 2005, redact 421, header hashes e3b0c442 (empty); bars written at 17:46;
  step 2's command `git merge --no-ff -F …/merge-msg.txt 0e621dae…`.
- light-base.log: HEAD 22cd65d, free 13,917; refs 2005, marks 81 unchanged, self 78/78, redact 421 items, 0 findings.
- check.log: HEAD 48efc9c, free 13,733, hashes e3b0c442; 11/11; test 908/1005, 0 failed, 0 skips; refs 2037; marks 81;
  self 78/78; redact 431, 0 findings; wasm 27000/27000 equal, hash-map caught at tick 1234, std-sin at tick 2, the Wasm
  digest f5e8d0e8… as in U8's run (the native hash-map digests vary per run by design); deps line identical to U8's.
- u8-merge/m5-check-r.log: 861/958, refs 1982, redact 421 (the base of departure 1).
- rejudge logs: HEAD 48efc9c, free 7,711, hashes e3b0c442, exit 0, the exact lines (3 and 2 runs); JSONs: MOVED, the
  moved block.
- history.md:8123-8131 his "go" 18:01:32 (2 characters); :7965 NB2-PROMPT (a)'s terms.
- gh api check-runs on S: toolchain 140106992032 success 18:40:52; gate 140105605763 success; smoke 140105605489
  success. Run 45876736547: push, S, attempt 1, success. Run 45876736642: attempt 1 push S failure (job 140105605787,
  step "The wasm-bindgen tool's archive …"; log: curl (22) 504 at 18:34:47), attempt 2 success.
- merge-tree vs main b9aad01: H0, S and 1fa5410 each conflict in docs/STATUS.md and docs/STATUS_ARCHIVE.md only;
  H0..1fa5410 -- docs: empty. PR #40: mergeable false, dirty, draft, head f8c50434.
- Plan on disk: As-approved block :3-18 carries NB2-C1 to -C12 by reference and records NB2-PROMPT (a). No NB2 report
  and no review-nb2-c10.md yet.

DONE
