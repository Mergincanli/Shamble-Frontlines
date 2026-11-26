# U8's M5 fix (advisor-u8-m5-ruling.md section 4), 2026-11-26

Base: `git add .checkpoints/netcode-proof-test/u8-build.md` (blob f4250899… equal to the lane head's), then
`git merge --ff-only 03d9ff3031fe4fdf6758183a7aca48b6ebab7906`: fast-forward 096808b..03d9ff3, 288 files.
`git log -1 --format=%H` = 03d9ff3031fe4fdf6758183a7aca48b6ebab7906. git status after: only U8's untracked ops files.
Start: 43 files, 6,170 lines; the stop 6,200 (30 left). No Low beside this fix.

Plan (the brief's steps 1-9): reproduce M5 (item 8, `40 !== 27`); item 8 rewritten against the merged struct (names,
optional fields, the 40 pin, required fields written, JSON types fit the Rust types, `cc` absent), red on the null
pair; r1.sh:230's empty pair, green; the two comments; 2.2's pin; plants P8, P9, P10; end checks on the lane-head tree;
the size.

## Steps
1. M5 reproduced on the merged crates: `node --test 'ops/**/*.test.mjs'` (ops/browser/node_modules linked from
   agent-abf71d95fc07b3522) 99 pass, 1 fail: "item 8: room_settings writes only fields the settings types accept, every
   U6a field, `bite` only in a bite room", "U6a's merged Settings: 26 fields and the bites build's `bite`", `40 !== 27`
   (r1sh.test.mjs:227).
2. Item 8 rewritten (r1sh.test.mjs:213-255): FIELDS read from `struct Settings` (crates/server/src/main.rs:77-147) with
   each Rust type, optional when `Option<…>` or `#[serde(default)]` above it; `fits(type, value)` for String, u32/u64,
   Vec<T>, Option<T> (any other type never fits). Split in two tests so a struct plant shows both reds: "item 8: the
   merged Settings holds 40 fields: U6a's 26 + `bite` + U6b's 7 + `cc` + U6c's 5" (the pin, and U6B, cc, U6C, bite
   among them), and "item 8: room_settings writes every required field of the merged Settings, values of the types it
   takes" (no unknown field; every required field; each value's type; no `cc`; the old asserts kept). Red on the
   current r1.sh: "alone: each value of the type the struct takes", `[ 'ws_cert_chain: String is not null',
   'ws_cert_key: String is not null' ]`; the pin passes (40).
3. Fix r1.sh:230: `chain=null key=null` -> `chain='""' key='""'` (U6b's empty pair, main.rs:119-122, :431-432). Green:
   test 7 22 pass.
4. Comments: r1.sh:218-222 (the struct main.rs:77-147, U6a's with U6b's and U6c's, checked by item 8 against the
   merged struct; same line count); r1.sh:242 "the empty pair, U6b's merged form: main.rs:119-122".
5. Test 2.2 (r1sh.test.mjs:348): a non-browser room's pair `["", ""]`.
   Size: test 7 +19, r1.sh 0, R1.md 0: 6,189 (+19), 11 under 6,200.
6. Plants (each by Edit, item 8 run, restored by Edit; pre-plant r1.sh 81cc5041…, main.rs 8e61c563…):
   - P8 (`s:hash_path=…` dropped, r1.sh:270): "alone: every required field", `[ 'hash_path' ]`; restored, r1.sh
     81cc5041… equal.
   - P9 (`j:io_threads=null`, r1.sh:266): "alone: each value of the type the struct takes", `[ 'io_threads: u32 is not
     null' ]`; restored, r1.sh 81cc5041… equal.
   - P10 (`    planted_field: u32,` in `struct Settings`, scratch, never built): both item 8 tests fail, the pin "U6a's 26 +
     `bite` + U6b's 7 + `cc` (U6b's E6) + U6c's 5 = 40: a struct change fails here" `41 !== 40`, and "alone: every
     required field" `[ 'planted_field' ]`; restored at once, main.rs 8e61c563… equal; `git diff --stat 03d9ff3…` empty.
7. End checks on the lane-head tree (03d9ff3): `node --test 'ops/**/*.test.mjs'` 101 pass, 0 fail (100 before: item 8
   is now two tests, the pin apart, so a struct plant shows both reds); `bash ops/tests/wrappers.test.sh` last line
   "wrappers.test.sh: 315 ok, 0 failed" (its first run on the merged tree); `check.mjs --only redact` PASS, 421 items,
   0 findings; package.json 8370495d…, package-lock.json a0efce8e… unchanged; no plant left; `git diff --stat
   03d9ff3031fe4fdf6758183a7aca48b6ebab7906` empty (no tracked change); node_modules link removed. Final r1.sh
   81cc5041…, r1sh.test.mjs 40f67bb9…; the other 41 files untouched (R1.md 847723cd… as at C9's end).
8. Size: net +19 (test 7 +19; r1.sh 0: one line changed, the comments kept their line count). 43 files, 6,189 lines,
   11 under 6,200. No Low built.
