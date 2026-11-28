# security-auditor: C3 read (d7e260f7, parent 0bcdddf0) against my conditions

Read only. Conditions from security-auditor-deps-4c-v2.md (APPROVE, F1, F2, item 3, third answer + amendment),
dependency-check.md (five digests), step1-build-order.md C3.

## Plan
1. Five digests at d7e260f7 vs approved; `git diff --name-only` of the commit for any other manifest/lock.
2. F1: dependency_rules.rs at d7e260f7 (sim's three lines, root libm byte line, one locked libm, only sim names libm, plants).
3. Item 3: rules_scan.rs libm rule (MATHS, MATHS_FILE, MATHS_CALLS, maths_breach, enclosing_fn) + bites/controls vs my list.
4. F2: layering.rs + support/marking.rs: manifest half, src half (plants/controls), sim half.
5. F5: content_breaches on the shared marking; in_tests gone; two controls now bites; r#fn bite.
6. (a) pub before mod; (b) RAW rule for exp_of/sincos_of; (c) vectors form.
7. Verdict, DONE.

## Notes
- [x] 1 MET. sha256 of git show d7e260f7:<path>: Cargo.toml fc436ec3, Cargo.lock b7c6ee5d, sim c7432b03, room 79b829a9, client-endpoint 22e2c5bf = dependency-check.md's five exactly. Commit's 24 files (stat) hold no other Cargo.toml, lock, package.json, deny.toml or toolchain file.
- [x] 2 F1 MET. dependency_rules.rs@d7e260f7: sim tables [package, dependencies, lints] :172-176; three exact lines :179-187 (rename m={package=libm} is another line: plant :315-323); root libm line to the byte MATHS :155 checked :205-213 (plant :335 drops '='); one locked version :222-226 (plant two versions :336-342); only sim names libm askers :237-241 (plants plain, renamed via package=, dotted table :324-330; comment control :331-334). names() is substring-on-non-comment-line: errs toward red. No [patch] table in root (Cargo.toml tables :2-155); root digest pinned by item 1.
- [x] 3 item 3 (RR1, RR4): rule MET, plants short by three (L1). rules_scan.rs@d7e260f7: MATHS/MATHS_FILE/MATHS_CALLS
  :108-110 (exp_of->exp, sincos_of->sincos); maths_breach :435-461: outside fmath.rs refused :441; anything but
  `libm :: name (` refused :446 (import, glob, `as`, `pub use`, `libm::math::..`, fn address, extern crate, macro `$r`);
  routine must equal the enclosing fn's one :449-452 (enclosing_fn :388-430, innermost named fn); fused names left to
  FUSED :438. Float rule skips `libm::` paths :495 so maths rule alone judges them. Bites :717-737 (21) hold expm1,
  sincos in `other`, sincosf, sin, a const outside any fn, `use libm::{exp}`+bare, `use libm::*`, `pub use`,
  `libm::math::sincos`, fn address, libm in movement.rs; controls :762-764. Of my named list three are not planted:
  `libm::expf(`, `::libm::exp(` outside the site, `extern crate libm as m;`. Traced by hand: expf -> :450 refused;
  `::libm::exp(` in `fn other` -> :450 refused; `extern crate libm as m;` -> :446 (`as`) refused. So the rule closes
  them; the plants are missing (L1, Low: add three LATER_BITES rows; no push on its own).
  Nested-fn trick (`fn other(){ fn exp_of(x:f64)->f64{libm::exp(x)} exp_of(1.0) }`): libm passes in the inner exp_of,
  but the call `exp_of(` in `other` is refused by RAW :558-565. Closed.
