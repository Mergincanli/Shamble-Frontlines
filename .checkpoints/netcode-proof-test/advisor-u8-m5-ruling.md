# Advisor: U8's merge stopped at M5 (test 71, item 8) — ruling and fix brief
2026-11-26, 04:5x-05:1x UTC window. Advisor agent, day session, effort xhigh, read only (no cargo). Acting by main's
advisor.md. Inputs: u8-merge-plan.md ("Changes before M0" and its STOPPED line, :56-99), .gate/u8-merge/m5-check.log
(whole: 657 lines), U8's worktree .claude/worktrees/agent-ac59cce3e5d002441 (read only: ops/tests/r1sh.test.mjs:213-259
and :321-335, ops/runs/r1.sh:206-280 and :326-351, ops/tests/wrappers.test.sh:174-176), the lane's merged
spikes/netcode-proof/crates/server/src/main.rs (:77-147, :316-365, :420-440, :1002), advisor-u8-stopb.md section 6,
marks.json PM-0001-24.

## 1. Verdict: FIX, then the merge runs again from M0. Two parts, both U8's; one builder message.
Test 71 is a true alignment catch, and reading the merged struct behind it shows a real defect as well:
- **The pin moved (expected).** Item 8 counts every field of the merged `Settings` as "U6a's". The struct now holds 40
  fields: U6a's 26, `bite`, U6b's 7 (the test's own U6B list), U6b's `cc` (E6) and U6c's 5 (the U6C list). So
  `40 !== 27` is the merged-state run doing its job (advisor-u8-stopb.md section 6: "test 7 reads the merged struct").
- **A real defect behind it.** room_settings writes `ws_cert_chain` and `ws_cert_key` as JSON `null` in every
  non-browser room (r1.sh:230 `chain=null key=null`, :269 `j:ws_cert_chain="$chain" j:ws_cert_key="$key"`).
  - U6b's merged struct takes them as `String` (main.rs:121-122: "both empty: the room's own certificate"). It checks
    them by emptiness (:431-432, "both or neither"), and its own fixture writes `""` (:1002).
  - serde refuses `null` for a `String`. So every R1 room but the browser rooms (alone, headers, probe, calibrate,
    starts, ds, bites) would stop at start with exit 2 (EXIT_SETTINGS, :54).
  - This is the card's open item "U6b's null certificate pair", now decided by the merged code.
  - No test caught it: item 8 checks names only, and 2.2 (r1sh.test.mjs:328-329) pins `[null, null]`.
- **Everything else room_settings writes fits the merged struct.**
  - Names: U6b's binds, `transport`, `hash_path` and `ws_split_mark_s`; U6c's `mode`, with the probe's four in the
    probe room only.
  - Types: strings, integers, `null` only for the `Option` fields `tick_vcpu` and `ws_split_mark_s`, arrays for
    `marks`, `io_vcpus` and `probe_legs`.
  - U6c's probe rules: legs P0 P1 P2 P3 P4 B1 in order (r1.sh:43 against main.rs:338-355); packets and warm-up in
    range (:356-361).
  - It leaves `cc` out, which is right for R1. `cc` is optional (`#[serde(default)]`, main.rs:127-129; absent means
    cubic). The one mark that selects `params.cc` is PM-0001-24, whose `where` is R3 (marks.json:5322-5353).
  - load_settings reads the pair as `r.get("ws_cert_chain")` (r1.sh:334). `""` is falsy as `null` was, so its pin
    choice does not change.

## 2. The rulings asked
- **What the fix is: both.** r1.sh changes one line: the null pair becomes the empty pair. Item 8 moves from a
  unit-count pin to the merged struct, with a type check that would have caught the pair. Test 2.2's pin follows. Two
  comments in r1.sh are re-pointed. The exact edits are in the brief, section 4.
- **Who builds it:** U8's builder ac59cce3e5d002441, resumed by SendMessage in a reset window (LEAN-MODE item 3). This
  is the 04:50 window if the message goes before 05:20 UTC, else 09:50. The brief is section 4, paste-ready.
- **Test first, with plants:**
  1. The builder fast-forwards to the lane head and reproduces M5's failure.
  2. The new item 8 fails on the current r1.sh by the type check, naming the pair.
  3. The one-line r1.sh fix makes it pass.
  4. Three plants each fail by name, and each is restored with its sha256 equal.
- **Must anything else in U8 change before M0 again?** No code beyond this fix. These procedural points bind:
  - (a) **wrappers.test.sh never ran on the merged state** (main-run.sh stopped at the ops failure). It runs in the
    builder's end checks on the lane-head tree, and at M5.
  - (b) **The new snapshot replaces wip/u8-build.patch.** M1 binds its new sha256. M2 compares the 43 paths between
    the head the builder fast-forwarded to and H0. M3 compares cmp and mode with the worktree again.
  - (c) **C1 stands** (u8-merge-plan.md:81-87). The fix touches neither ops/browser/lib.mjs nor config.mjs, which C1
    imports. If either changes, C1 runs again.
  - (d) **Size.** U8 stands at 6,170 of his 6,200 stop (the card's U8 item; history.md:7703-7722). The fix's net added
    lines must keep the whole at or under 6,200, which leaves 30. Past it, the builder stops and reports, and he gets
    three lines (what grew, why, the cheaper option). No Low may ride this fix.
  - (e) **M5's order (my lean, not a bar).** Put the cheap runs first in the one main-run.sh call: `node --test
    'ops/**/*.test.mjs' :: bash ops/tests/wrappers.test.sh :: env CARGO_INCREMENTAL=0 node check.mjs`. Then an ops
    failure stops in seconds, not after check.mjs's 17 minutes. The bars are unchanged: check.mjs 11 of 11, ops all
    pass (100 expected, the count derived), wrappers' last line with 0 failed.
  - (f) **The "open at its merge" list stays in u8-report.md.** It is F28's live proof, ds-selftest's place in C1's run
    list, Stop B's laptop reads, `sudo -l` on RS, SEC-SB-9's rest and NC-R3-1. "U6b's null certificate pair" moves
    from open to fixed (this ruling). "U6c's names" can be recorded as checked for the probe's settings (section 1).
- **Does choice 15 (the `ops` step after U7's merge) stand? Yes,** on these terms:
  - (i) **Until the `ops` step lands,** every whole run runs the ops tests and wrappers.test.sh beside `node
    check.mjs`, in the same main-run.sh call, with the ops/browser/node_modules link or install the plan names. That
    covers U8's M5, U7's merge's counted run and NB2's check.mjs whole.
  - (ii) **It lands at U7's merge or in the first change after it,** and before Stop B's fill. Both edits (check.mjs's
    STEPS and the pin at check.test.mjs:2785) go through U7's apply check, with U7's "web" and U8's "ops" in one list.
  - (iii) **It is named in the day message** as choice 15.
  - The brief's own order is "merge, then the `ops` step" (u8-brief.md:524). Moving it later keeps U7's patch, which
    edits the same pin, applying clean.

## 3. Findings (CLAUDE.md's scale)
| Id | Severity | What | Fix |
|---|---|---|---|
| ADV-U8M5-1 | High | room_settings writes `ws_cert_chain`/`ws_cert_key` as `null` in every non-browser room. U6b's merged struct takes `String` (main.rs:121-122, :431-432), so those rooms would refuse their settings at R1's start (exit 2). Caught by reading, not by a test. | Brief step 3 (r1.sh:230) and step 5 (2.2's pin) |
| ADV-U8M5-2 | Medium | Item 8 pins U8-time unit counts and checks names only, so it can neither follow a merged struct nor see a value of the wrong type. | Brief step 2 |
| ADV-U8M5-3 | Low | r1.sh's comments cite the pre-U6b struct lines (:220 "main.rs:62-103") and call the null pair U6b's form (:241-242). | Brief step 4 |
| ADV-U8M5-4 | Low (process) | wrappers.test.sh did not run on the merged state at M5. | Section 2 (a), (e) |

The High is fixed by the brief, before any merge. Nothing is carried as a follow-up.

## 4. The fix brief for U8's builder (paste-ready; the main session fills `<head>` and runs ffcheck first)
> **U8: the M5 fix (advisor-u8-m5-ruling.md).** U8's merge into the lane stopped at M5: test 71 (r1sh.test.mjs:226-227,
> item 8) failed on the merged tree, `40 !== 27`, because U6b and U6c added fields to npt-server's `Settings`. Reading
> the merged struct also shows one real defect: room_settings writes the WebSocket certificate pair as JSON `null`,
> and U6b's merged struct takes two strings, empty for the room's own certificate. Fix both, test first. Your files
> stay under spikes/netcode-proof/ops/; your other 41 files are not touched. No cargo.
>
> 1. **Base.** `<head>` is the lane head the main session names. In your worktree, first stage your own notes file,
>    whose content already equals `<head>`'s copy byte for byte: `git add .checkpoints/netcode-proof-test/u8-build.md`.
>    Then `git merge --ff-only <head>`, then `git log -1 --format=%H` into a NEW notes file,
>    `.checkpoints/netcode-proof-test/u8-fix-m5.md` (untracked; write no more into u8-build.md, so the next
>    fast-forward cannot collide with it). If git refuses, stop and report its message. Remove, restore or move nothing.
>    Then run `node --test 'ops/**/*.test.mjs'` from spikes/netcode-proof. Expected: item 8 fails with `40 !== 27` (M5's
>    failure, reproduced on the merged crates). Record it.
> 2. **Item 8, rewritten against the merged struct** (r1sh.test.mjs:213-235; keep its other asserts as they are).
>    - Read each field of `struct Settings` in crates/server/src/main.rs with its Rust type, and mark it optional when
>      its type is `Option<…>` or a `#[serde(default)]` line stands above it. `bite` is the bites build's.
>    - Pin the struct: 40 fields, U6a's 26 + `bite` + U6b's 7 (the U6B list) + `cc` (U6b's E6) + U6c's 5 (the U6C
>      list). Assert the U6B and U6C lists and `cc` are among them, with a message naming that sum, so a later struct
>      change fails here by name.
>    - For every kind:
>      - every key room_settings writes is a struct field;
>      - every required field (not optional, not `bite`) is written;
>      - each value's JSON type fits its Rust type: `String` a string; `u32`/`u64` a non-negative integer;
>        `Vec<String>` an array of strings; `Vec<u32>` an array of non-negative integers; `Option<T>` `null` or T;
>      - `cc` is absent: R1 runs quinn's default, cubic (main.rs:127-129); PM-0001-24, the one mark that selects
>        `params.cc`, is R3's.
>    - Update the title and the comment above it to match ("every required field of the merged Settings; values of
>      the types it takes").
>    - Run it: it must FAIL now on `ws_cert_chain` (and `ws_cert_key`): `null` is not a string. Record the message.
> 3. **The fix, r1.sh:230.** `chain=null key=null` becomes `chain='""' key='""'`: the empty pair, U6b's merged form for
>    the room's own certificate (main.rs:119-122, :431-432; its fixture :1002). The browser line (:244) is unchanged,
>    so wrappers.test.sh:175 still holds. load_settings (:334) reads the pair as `r.get(...)`, and `""` is falsy as
>    `null` was. Item 8 now passes.
> 4. **Comments.** r1.sh:219-221: the struct is crates/server/src/main.rs:77-147, merged with U6b's and U6c's fields,
>    checked by item 8 against the merged struct. r1.sh:241-242: "null pair: u6b-brief.md:209-212's form …" becomes
>    "the empty pair, U6b's merged form (main.rs:119-122)". Nothing else.
> 5. **Test 2.2's pin** (r1sh.test.mjs:329): `[null, null]` becomes `["", ""]` for a non-browser room.
> 6. **Plants**, each by Edit, the named test run, then restored by Edit with the sha256 equal to the pre-plant value:
>    - P8: drop `s:hash_path=…` from room_settings's args (r1.sh:270). Item 8 must fail, "every required field".
>    - P9: `j:io_threads=null` in room_settings. Item 8 must fail on the type (`u32` is not null).
>    - P10: add the line `    planted_field: u32,` inside crates/server/src/main.rs's `struct Settings`. This is a
>      scratch plant, never built. The 40 pin and "every required field" must fail. Restore it before anything else.
> 7. **End checks**, from spikes/netcode-proof on the lane-head tree:
>    - `node --test 'ops/**/*.test.mjs'`: all pass, 0 fail, the count recorded (100 expected);
>    - `bash ops/tests/wrappers.test.sh`: 0 failed, its last line recorded (it never ran on the merged tree at M5);
>    - `node check.mjs --only redact`: 0 findings;
>    - the package files unchanged; no plant left;
>    - `git diff --stat <head>` (no tracked change).
> 8. **Size.** U8 stands at 6,170 of the 6,200 stop. Count your net added lines. If the whole would pass 6,200, stop
>    before writing past it and report: what grew, why, the cheaper option. Build no Low beside this fix.
> 9. **Report:** the commit `<head>` you reached; the edits (file:line); the three reds and the green with their test
>    lines; the plants; the end-check counts; the net added lines and the new total. I take the snapshot.
>
> Write files with the Write and Edit tools only (no sed, no shell redirect, no python writes; never `cargo fmt`
> writing files: apply rustfmt's diff by Edit); an agent with no Edit tool rewrites its notes file whole with Write.
> Never switch the main folder; never `cd` into another worktree; no shell loops; no cargo. git and gh always start
> their own command, with no other program in front of them.

## 5. For the main session, before sending
- **ffcheck** from the main folder against the head you name (`node .checkpoints/netcode-proof-test/ffcheck.mjs <U8
  worktree> 096808b <head>`). Expect exactly one differing path, u8-build.md, whose worktree copy equals the branch's
  (both 4584b55f…382b at this ruling). The brief's step-1 staging covers that path: with its index entry equal to
  `<head>`'s, git keeps it in a fast-forward. Any other ADDED-present or differing path: stop, and bring it here.
- **After the report:**
  - take the snapshot (worktree_patch.py, prefix spikes/netcode-proof/ops; 43 files);
  - restart the merge at M0 under u8-merge-plan.md with section 2's (a)-(f);
  - M0's checks stand: main still 07d673b, else MAIN-52 decides first.
- **Not a new agent's work for me** unless M5 fails again: the pushed head's read rides the next window's agent, as
  the plan's M9 says.

## 6. What I did not check
- I ran nothing (no node test, no cargo). That serde refuses `null` for a `String` is serde_json's documented behaviour
  ("invalid type: null, expected a string"), not run here. The merged code's own fixture (:1002) and its emptiness
  rule (:431) support it.
- That git keeps a staged entry equal to the target's in a fast-forward is git's two-way merge rule (an index entry
  equal to the new tree's is kept). If git refuses anyway, the builder stops: step 1.
- U8's other 41 files were not re-read against the merged code. The 99 passing tests and check.mjs's 11 steps cover
  what they test, and wrappers.test.sh runs in the end checks.
- The 6,170 count is the card's (C9's end), not recounted.

## Decisions needed
None. A test and a one-line fix inside U8's approved brief: no pass mark, judge, marks.json, money or main. The size
stop is his: it reaches him only if the fix would pass 6,200.

## For Mustafa
Nothing.

DONE
