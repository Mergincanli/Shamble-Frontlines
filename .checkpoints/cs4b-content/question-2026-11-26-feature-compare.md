# Two questions for Mustafa from 4b's step 0 (2026-11-26; asked in the chat, saved here)

Step 0 is the manifest batch: the collision library `parry3d` =0.31.1 enters the simulation's tree. The batch is in
the working tree, uncommitted and unbuilt (his DEP-HOLD); its text is in `deps-manifests.diff` and
`deps-lockfile.diff`, its digests in `deps-batch.sha256`. The lockfile gains 33 packages from crates.io, the
versions rust-reviewer's paper reading named, plus our own new `content` crate.

## FEATURE-COMPARE: may the build's check on the simulation's dependencies take two named exceptions?
**What the check is.** Since change-set 3a every build first asks cargo what it resolved, and stops unless
everything the simulation depends on has the same packages with the same features in the three builds that run the
simulation: the players' file, the test-only module, the native tests (ADR 0002 part 3; mark PM-0004-74;
`simFeatureList` and `simFeatureDifferences` in `packages/client/scripts/build-checks.mjs`). Until today the
simulation depended on nothing, so the check had nothing to refuse.

**What it says with the collision library** (`deps-feature-compare.txt`; nothing is compiled for this): the players'
file and the test module agree (71 entries each); the native tests differ in five entries:
1. `safe_arch` (three entries: the package and two of its features). It is the x86 backend of `wide`, a vector
   helper the collision library always links. It exists on x86 processors only: in the native tests, never in
   WebAssembly. No setting of the library removes it.
2. `thiserror`'s features `default` and `std`. The test-only type writer (ts-rs, in the workspace since change-set
   1) turns them on in the native tests. `thiserror` is now also in the simulation's tree, through the collision
   library. With the library's own `std` feature on (the plan's fallback) `std` agrees but `default` still differs,
   and the lockfile takes six more packages, one of them a second version of a package already in it.

**Why it matters.** The check exists so that no build computes with other code than the players' file does. Neither
difference changes a number: `safe_arch` backs a helper the library's query paths do not use (rust-reviewer's
reading of the library's source, `rust-reviewer-cs4.md`, B2-3), and the plan already demands the proof on the real
thing (the collision tree's bytes and 10,000 rays the same natively and in WebAssembly, PM-0004-32); `thiserror`'s
two features only change how error types print. But the check cannot know that, and the advisor agent's condition
is plain: the compare is never loosened without his word. The check's own comment foresaw the second kind: "the
step that first nests generated types decides which of the two gives way."

**Options.**
- (a) Two named exceptions inside the check, each with its reason in the code and a test that it is no wider than
  its name: a package that exists for one target only must be on a named list (today exactly `safe_arch`); the
  features of `thiserror` that only test builds turn on are named (`default`, `std`). Any other difference stops
  every build as today. Recorded as a dated pointer for ADR 0002's next revision, like the departures he accepted
  before. The library's `std` stays off, as rust-reviewer recommends (one maths implementation on both targets, six
  fewer packages). rust-reviewer and engine-architect read the exception's wording before it is written.
- (b) The exception for `safe_arch` only (nothing else removes it), and the type-writing tests move out of
  `cargo test --workspace` into a cargo command of their own, so that no test build turns `thiserror`'s features
  on. That is a CI edit (the CI Builder's) and a change to how the generated types are checked; 4b waits for it.
- (c) No exception: the collision library does not enter until the check or the drafts are revised. Walk stops.
- Recommended: **(a)**.

## SIZE-4B: 4b will pass its stop line; how should it be cut?
**The numbers, each named** (his rule of 2026-11-25). The designs' own count for 4b was 5,410 lines. The two
readers' points and the advisor agent's conditions of today add about 950 by the same way of counting (caps, the
bundle's path rules, expected error records, the fuzz's histogram, the page parsing only after the loader's yes,
the tools' file rules, the proof that loading content changed no state): about **6,360**. Planned at 1.8: about
11,450. At 4a's measured rate (2.2 times before review fixes, 2.4 after): about **14,000 to 15,300 written
lines**. The stop line is **12,200** and stays as it is (his words). The plan's two ladder rungs (the schema writer
to 4d; the page's scene to the start of 4c) take about 1,650 to 1,800 written lines out: 4b still ends near 12,400
to 13,500.

**Why it matters.** At 4a I reached the stop line with the code already written. Here the count says so before
step 1, so he can choose the cut instead of meeting it.

**Options.**
- (a) Nothing changes: I build in the plan's order, fire each ladder rung at its line, and stop and ask at 12,200.
- (b) 4b becomes two pull requests at its natural seam. **4b-1** (steps 0 to 4 and the links check): the manifest
  batch, the built-file check, the content reader with its proofs and its fuzz run, the time unit. Nothing a player
  sees; no browser check; it does not wait for his Export. **4b-2** (steps 5 to 9): the tools, his values by
  script, content into the simulation, the collision world, the page's scene; his short browser check. Each keeps
  the rule "a proof merges with its code". Each gets a stop line of its planned size times 1.25, summing to what
  4b's would be for 6,360 lines (about 14,300); the split of that sum is shown to him before step 1.
- (c) One pull request, and the stop line moves to cover the readers' additions (6,360 times 1.8 times 1.25: about
  14,300).
- Recommended: **(b)**: two reviews of about 7,000 lines each find more than one of 14,000, and the first half can
  merge while he plays the sandbox for his Export.
