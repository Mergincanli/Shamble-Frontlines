# Advisor agent: C4's change to the maths allow-list guard (2026-11-28)

## Plan
1. Read the two safe files as they stand (packages/client/scripts/maths-allowed.mjs, build-checks.test.mjs) and their
   diff against HEAD a95ea332; build-checks.mjs from HEAD (`git diff --quiet HEAD` on it: unchanged), copied to my
   scratchpad; my delta verdict's ruling 8; maths-refusals-c4.txt; Mustafa's words on cutting tests (DECISIONS.md at
   HEAD); face_cm from the saved c4-wip.patch (the live collision.rs is being planted: not read).
2. Probe, outside the worktree only: node on the scratch copies of HEAD's build-checks.mjs and C4's maths-allowed.mjs
   (both import node built-ins only). No cargo, node or build in the worktree.
3. Rule on (1) thinning or not, (2) verdict, (3) what security-auditor must be told; notes for the report; hand back.

## What I checked
- The rules (build-checks.mjs at HEAD): `bannedMathsProblems` (:735-737) tests each WHOLE name against BANNED_MATHS
  (:728-732) and takes no list; twinProblems calls it on the twin's whole function names (:851), so a mangled libm
  name never meets it. libmProblems (:776-789) is the only rule the list lifts (:778, the segment rule: the entry
  itself or `entry::` below it); it never writes `C::<banned>` for a banned plain name (:785). So no list can lift
  the banned rule; the list lifts libm's rule only.
- The old guard (HEAD test :769) asserted `bannedMathsProblems(entry.routine.split("::"))` is `[]` for every entry: a
  proxy that reads an entry's segments as if they were C names. The new form (test :770-777): the same call for every
  entry, with an exact expected value: `[]` for 21, and for exactly the strings `libm::math::exp::exp` and
  `libm::math::sincos::sincos` two lines of their own name and nothing else; :780 both are on the list; :783-785 the
  planted twin; :788-798 the 23-routine pin (replacing HEAD :771's two-routine pin); :773, :775 unchanged; :778 new
  (five bare prefixes refused); :786 new (no duplicate).
- Set difference, old vs new, over every possible entry: the only entries that fail the old assertion and pass the new
  are those two exact strings. Any other entry with a banned segment (`libm::math::sin::sin`, `libm::math::exp`,
  `libm::math::exp::exp::x`, `C::exp`) still fails, and also fails the pin (:788).
- Probe 1 (scratch, HEAD's check, C4's list): mangled `libm::math::expf::expf`, `exp2::exp2`, `expm1::expm1`,
  `sin::sin`, `cos::cos`, `sincosf::sincosf`, `k_sinf::k_sinf`, `rem_pio2f::rem_pio2f` and the synthetic in-module
  siblings `exp::expf`, `sincos::sin`: 10 of 10 refused by libm's rule under the whole list. The two real mangled
  names: libm rule `[]`, banned rule `[]`. Plain `exp, sincos, sin, cos, expf`: libm rule `[]` under the list, banned
  rule fires (2 of 2 for exp, sincos with an empty list too).
- Probe 2: a trait impl name (`<f64 as ...float_traits::Float>::to_bits` shape, `NtB7_5Float7to_bits` after a type
  `d`) is read by pathsInside (:740-760) as `libm::math::support::float_traits`: the path stops at the module. So the
  three module entries (maths-allowed.mjs:102 support::env, :106 support::float_traits, :126 support::int_traits) are
  the only form that admits what the build printed (refusals file: 2, 3 and 6 names). Parents among the entries:
  rem_pio2::rem_pio2 -> ::medium; rem_pio2_large::rem_pio2_large -> ::floor; support::float_traits -> the four
  Float::* entries.
- Mustafa's words: DECISIONS.md (HEAD) :226, CHROME-TIME, 2026-11-28: "Until then never cut or thin a test to fit the
  5 minutes." That is about the Chromium time limit; this change is not made to fit time and adds assertions. The
  general rule is CLAUDE.md's "Explicitly not allowed: skipping verification, shrinking evidence". Searched:
  `git grep -i "thin a test|thinned|thin the|cut or thin"` at HEAD over docs/ and CLAUDE.md: the only other is
  DECISIONS.md:480 (CI-TIME, "Never cut or thin a test to fit the limit"), also about a time limit. The same block,
  read whole (:201-228): DEPART-4C (:218-221) is about SWEEP-ROUTE, PM-0004-60 and PM-0004-20, not this; :224
  "The advisor agent decides what it may decide." This is a technical choice inside ruling 8 that loosens no rule
  of his, so it is mine to decide; nothing here goes to him.
- face_cm (c4-wip.patch:6133-6147, the saved patch, not the live file): on_grid first (:6134), and off the grid its
  own `(value * per_metre).round()` (:6138). So C::round's why is right that a second caller exists; its "once more"
  means face_cm's own rounding, which the why does not name.
- libm pin: Cargo.toml:28 `libm = "=0.2.16"`, Cargo.lock:170-171 version 0.2.16.

Not reproduced by me: the 59-of-59 run, the production build, maths-refusals-c4.txt's counts (the plant run is live;
I ran nothing in the worktree). My probes ran HEAD's committed check, not the worktree's test file.

## Rulings
(1) Not a thinning. What the guard exists to stop is an entry that names or lifts a banned function. The new form
keeps the same call on every entry and makes its expected value exact; the only entries it now admits are the two
exact strings ruling 8 accepted (each pinned to exactly its own two lines), and the pin admits nothing else. The banned
rule itself never read the list (:735-736, :851, :785), so it lifts nothing for plain C names before or after. The old
assertion could not stand with the list ruling 8 approved; keeping it would have meant not listing exp and sincos.
One honest correction: the planted-twin check (:783-785) passes for ANY list, since the banned rule takes none; it pins
the rules' structure (useful), it is not evidence about this list. The list-sensitive half (that exp::exp and
sincos::sincos lift those two and no sibling) is not in the file; probe 1 shows it holds, and condition 1 puts it in.
With condition 1, the guard is stricter than HEAD's: HEAD proved a string property of the entries; C4 proves that and
the real rules' behaviour under the real list.

(2) APPROVE, on these conditions, all in C4's commit (not yet made: no extra push):
1. Add, in the same test after :785, this block (I ran its assertions against HEAD's check and C4's list in my
   scratchpad with node:assert: all hold; not run inside the vitest file). It fixes the assertions, not the layout:
   split the `siblings` line as the file's formatter wants.
   ```js
   // With today's whole list, libm's other routines of the banned names, and a sibling inside exp's and sincos's own
   // modules, are each refused by libm's rule: the two entries lift exp and sincos alone. The two themselves, by the
   // names the build prints, meet neither rule, and without their two entries libm's rule refuses them.
   const LIBM = "_RNvNtNtCs8f6xQ2mK1pZ_4libm4math";
   const siblings = { "4expf4expf": "expf::expf", "4exp24exp2": "exp2::exp2", "5expm15expm1": "expm1::expm1", "3sin3sin": "sin::sin", "3cos3cos": "cos::cos", "7sincosf7sincosf": "sincosf::sincosf", "3exp4expf": "exp::expf", "6sincos3sin": "sincos::sin" };
   const outside = (name, routine) => `${name}: ${routine} is a maths routine outside the allow-list (packages/client/scripts/maths-allowed.mjs)`;
   const names = Object.keys(siblings).map((tail) => LIBM + tail);
   expect(libmProblems(names, MATHS_ALLOWED)).toEqual(Object.values(siblings).map((path, k) => outside(names[k], `libm::math::${path}`)));
   const theTwo = [`${LIBM}3exp3exp`, `${LIBM}6sincos6sincos`];
   expect(libmProblems(theTwo, MATHS_ALLOWED)).toEqual([]);
   expect(libmProblems(theTwo, MATHS_ALLOWED.filter(({ routine }) => !own.has(routine)))).toHaveLength(2);
   expect(bannedMathsProblems(theTwo)).toEqual([]);
   ```
2. Make the comment at :781-782 say what it proves: "The banned rule reads no list: with today's whole list, a twin
   that holds the C functions exp and sincos (and sin, cos, expf) is refused for each of them."
3. Correct two false sentences (Medium: they misstate what a security list covers, and :13 credits it to my condition):
   - maths-allowed.mjs:13-14 "each its own entry, none a prefix that covers another routine (the advisor agent's
     condition of 2026-11-28)" -> "each its own entry, none a covering prefix beyond what the build printed (the
     advisor agent's condition of 2026-11-28): three are module paths (support::env, support::float_traits,
     support::int_traits), because the check reads a trait's method by its module alone, and so cover every function
     of those modules; six entries sit below another and are listed because the build printed them."
   - test:765 "No libm entry is a bare module (`libm::math` and the like): each names a routine's own path." -> "No
     libm entry is a bare prefix (`libm::math` and the like): each is a path the build printed, a routine's or, for a
     trait's methods, its module's."
4. After the plant run ends: run build-checks.test.mjs and report its count (59 tests, same as now if the block joins
   the existing test) and the gate on C4's head; the record of the refusals stays in the report with its method.
5. Low, rides the same commit: C::round's why (maths-allowed.mjs:36) "and once more for a face off the grid" ->
   "and, for a face off the grid, face_cm's own rounding once more".

Ruling 8(a) is met in substance: every entry is a path the real build printed (refusals file: 21 listed, 0 listed but
not refused, 0 refused but not listed), none is `libm::math` or another bare prefix, and the three module entries are
forced by how the check reads trait methods (probe 2). (b) holds: outside libm::math: [] (refusals file), and every
one of the 21 is in B17-5's positive-control list (rust-reviewer-read.md:1198: release 4, dev 29, the same counts
as the refusals file; the four float_traits::Float::* paths sit under its `support::float_traits`). (c): the
rem_pio2_large why (maths-allowed.mjs:77-78) says linked, unreachable on [0, 360), and the one target-dependent code.

(3) For security-auditor, when it reads C4's commit under ruling 8(d):
- The count: 21 libm entries, not F2's four (security-auditor-deps-4c-v2.md:85-87 expected libm::math::exp,
  ::sincos, ::rem_pio2, ::rem_pio2_large): 4 in the release twin (which the dev twin prints too) and 17 in the dev twin
  only, against B17-6's "about 14" (rust-reviewer-read.md:1199), which counted module prefixes (`libm::math::exp`);
  C4 lists one entry per printed path, so exp and sincos are routine paths (`exp::exp`, `sincos::sincos`), narrower
  than F2's expected module paths.
- The per-entry banned assertion now carries an exact two-string carve-out (test :770-777, was HEAD :769); check that
  the carve-out is the exact pair and the expected value exact, and that condition 1's block is in.
- The banned rule takes no list (build-checks.mjs:735-736, :851) and :785 keeps banned plain names out of libm's rule:
  confirm no list can lift a plain C banned function.
- The three module entries (maths-allowed.mjs:102, :106, :126) lift every function of those modules in libm 0.2.16,
  printed or not; read what else lives there (in particular any method of the Float or Int traits that is not exact:
  a fused multiply-add is refused by its own rule whatever the list says, :715-720, :850). Only the version pin
  (Cargo.toml:28, Cargo.lock:170-171) bounds this: a libm bump must bring the list back to review.
- rem_pio2_large is listed although it is not the same code on every target; it rests on unreachability (content's
  heading range) plus the sincos vectors; weigh it against F2.
- The list is pinned at :788-798; the six child entries are redundant under the segment rule (harmless, listed as
  printed).

## Notes for the report (no ruling)
- Six entries below a parent (rem_pio2::rem_pio2::medium, rem_pio2_large::rem_pio2_large::floor, the four
  float_traits::Float::*): I agree; list them as printed, say they are redundant under the segment rule.
- C::round's second caller (face_cm, at match setup): confirmed in c4-wip.patch:6133-6147 (saved patch); condition 5
  sharpens the wording.
- The `own` presence check (:780) is also implied by the pin (:788); harmless.

## Severity of my findings
- Medium: the two false sentences (condition 3).
- Medium: the list-sensitive control missing (condition 1); probe 1 shows the behaviour holds today.
- Low: :781-782's comment overstates what the check proves (condition 2); C::round's why wording (condition 5).
