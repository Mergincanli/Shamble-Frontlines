# security-auditor: change-set 4b-1 "Content", review round 2 (what the fix pass changed)

Date 2026-11-26. Worktree .gate/lane1-takeover, branch cs/4b-content, code head d36e416 (round 1 read ef2edec),
notes commit 495204a on top. Read-only; notes and scratch only (scratch: .gate/security-auditor-4b1-scratch/,
target: .gate/security-auditor-4b1-target/). Diff read: git diff ef2edec d36e416 -- crates packages.

## Plan
1. Inventory: git diff --stat ef2edec d36e416 -- crates packages; deps: sha256 of Cargo.lock and the two manifests
   vs .checkpoints/cs4b-content/deps-4b1.sha256; no new dependency (manifests, package.json, lockfiles unchanged).
2. SA-1: json.rs string(): read the measure-then-take code; reason about every way a held byte can exceed its
   input byte (strings, keys, the tree's Vec growth, the field paths, the duplicate-key sort buffer). Scratch test
   (copy of my round-1 crate harness, pointed at the tree's content crate, own target): the fifteen-texts bundle
   again, then next-worst shapes: (a) many short strings (per-String overhead 24 B vs 2-3 B input), (b) many small
   objects/arrays (Vec growth by doubling in arrays and objects), (c) texts of escapes (decoded smaller than raw:
   exact sizing must count decoded, not raw), (d) keys, (e) nested arrays. Compare held vs input + 4 MiB.
   Also read content_fuzz.rs's limit-case runner and LIMIT_CASES/limit_case(at).
3. SA-2: the three readers and the verdicts' write: content_proofs.rs (tests), content.test.mjs, content.spec.ts;
   grep the change-set for every other read of a committed file (readFileSync, read_to_string, fs::read, File::open,
   route.fulfill path, fetch of files) outside the helpers.
4. SA-6/RR-4: files.rs file_name, read_plain (one handle, metadata of the handle, take), write_plain; what race or
   path form is left (parent folder links, O_NOFOLLOW absent?, open of a FIFO blocks before the type check?, device
   files, write: temp + rename or direct?); the doc's wording against the code. recordings.rs scan follow-up.
5. SA-3: is_id device names vs Windows' reserved list (CON PRN AUX NUL COM0-9 LPT0-9, CONIN$/CONOUT$ already out
   by grammar; COM superscripts not ASCII); case (ids lower-case only); with extension (con.json is the hazard);
   generator's random_id rule.
6. SA-4: error.rs Echo::of escapes the quote; record() doc.
7. SA-5: rules_scan.rs the_content_crate_keeps_its_rules: what text scanning can and cannot see (index out of
   bounds, slicing, arithmetic overflow in debug, division by zero, .unwrap_or fine, expect in a macro, unwrap via
   a different spelling such as `Option::unwrap(x)`, `[i]` indexing, `as` casts); is its claim honest about clippy.
8. SA-7: content.test.mjs time asserts, content.spec.ts 60 s, map.rs ranges walk.
9. Anything else the diff changed (number.rs tie read-back for panics/loops; generator changes; dependency_rules.rs).
10. Final: <= 20 lines in the brief's shape.

## Findings (one line each, as made)
