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
- [x] L2 (Low, new, RUN): marking.rs's own tokenizer mis-reads the escaped quote `'\''`. marking.rs:86-91: when
  escaped, `end = open + 2` is the escaped character itself, so for `'\''` the loop stops there and the literal's real
  closing quote is read as a new quote (:83); if the character after it is followed by `'`, that pair is eaten as a
  char and the next literal's content is exposed. Probe (marking.rs@d7e260f7 copied to the scratchpad, rustc 1.99.1,
  binary .gate/sa-c3-probe): `#[cfg(test)] mod tests { const Q: [char; 2] = ['\'','{']; } use content::map::MapDocument;`
  -> every token marked test-only, `content` read as test (false pass for layering.rs AND content's panic scan);
  controls in the same run: `['{', 'x']` and `'\''` alone mark correctly; `[b'\'', b'{']` (spaced) correct.
  rules_scan.rs's own tokenizer is right here (:201-206 skips three, then seeks the quote). Reach: needs the unspaced
  form (rustfmt adds the space; survives under #[rustfmt::skip] or in a macro's input); backstops: Cargo keeps the
  dev-dependency out of non-test builds (layering), clippy's guard in content where it runs. Fix: escaped -> start the
  search at open + 3; plants: the probe's line red in layering.rs, the same with `x.unwrap()` red in content_breaches.
- [x] (a) RUN: `#[cfg(test)] pub(crate) mod tests {..}` and `pub(in crate::a) mod` both mark the module only; code
  after stays shipped (probe). marking.rs:194-200 takes `pub`, then a balanced `(..)`, then requires plain `mod`,
  a name and `{` (:208). Visibility does not change what cfg(test) compiles out: within the ruling, not a widening.
- [x] 4 F2 MET (plants and controls as ruled), with L2 above and L3/L4 below. layering.rs@d7e260f7: manifest half
  :57-81 (any table header or line naming content, except DEV_LINE in [dev-dependencies]); plants :297-303 (normal,
  renamed via package=, [dependencies.content], per-target, dotted key), control :307-312. Src half :85-117 on the
  shared marking (:15-16), any non-test token `content` (raw too: marking keeps raw text without r#), plus include!,
  include_bytes!, include_str!, #[path]; walk recursive, lib.rs asserted :153-158; T-G in a room AND an endpoint copy
  :239-248; 19 bites :250-275 = my third answer's list + amendment (cfg any/not/cfg_attr, extern crate as, use ::content,
  r#content, F4's three + the #[allow] variant, strip!, two top-level non-module cfg(test) items, literals/comments,
  mod in mod, #[path], include!, include_bytes!); controls :281-287 (tests mod, pub(crate) form, #![cfg(test)] file,
  content_hash, comment/string). Sim half :120-141, HANDED_ON = time.rs's one :29; plant MapDocument :316-317;
  pub(crate) use not counted (does not leave sim) :318-325. Not planted: include_str! (rule there, :104). Nit.
- [x] L3 (Low, new, by reading): the manifest half and dependency_rules.rs's names() match raw TOML text, and a TOML
  basic string may spell a name with escapes: `c = { package = "content", path = "../content" }` in room's
  [dependencies] names content with no `content` substring; src then says `c::..` (no token): both halves pass, and as a
  normal dependency the compiler does not stop it either. Deliberate evasion only; reviewers read manifest diffs. No
  manifest of ours holds a backslash today (git grep '\\' over crates/*/Cargo.toml and Cargo.toml: none). Fix: refuse
  any backslash in a non-comment line of room's, client-endpoint's and sim's manifests (one assert each, one plant).
- [x] L4 (Low, forward): fixed lists. NEVER_CALLING :26 names room, client-endpoint, sim-wasm; today these plus devtools
  are sim's only dependents (git grep of crates/*/Cargo.toml), so it holds now, but a new crate depending on sim is not
  walked. And the sim half sees `pub use`/`pub extern crate` only: `pub type Doc = content::map::MapDocument;` hands
  content's type (and its inherent functions) to room as `sim::Doc` unseen; none exists today (git grep sim/src).
  Fix: walk every crates/* but sim and devtools for the raw names; count `pub type` naming content in handed_on, plant.
- [x] 5 F5 MET. rules_scan.rs@d7e260f7 content_breaches :847-865 uses marking::tokens/test_only (:849-850); `in_tests`
  and TEST_ITEMS gone (grep: none at d7e260f7; parent :691, :699, :767). Former controls now bites :930 (top-level
  cfg(test) const) and :931 (fn); r#fn bite :932; the control rewritten as a `mod helpers {..}` :960. Real content tree
  scanned :868-896 (needs (a)'s pub form for map.rs). L2 also reaches this scan (probe line with x.unwrap()).
- [x] (b) libm conditions kept. libm named only in fmath.rs :43, :50, full paths, (exp_of, exp) and (sincos_of, sincos);
  every other path refused (item 3). New reach: exp_of/sincos_of are `pub` (fmath.rs:42, :49; sim's lib.rs:20 pub mod
  fmath), so libm's raw f64 result is callable from other crates. Bounded: RAW in sim, test code included
  (rules_scan.rs:118, :558-565; bites :717-725 incl. fmath's own tests and a glob import); layering refuses both names in
  room, client-endpoint, sim-wasm src, tests included (:172-220, plants room + sim-wasm, control longer names); devtools
  (the vectors, not shipped) is the one outside caller. Within my ruling: one file names libm, two allowances; the raw
  names are a proxy with their own one-file, one-caller rule. Players' file: no libm entry yet (maths-allowed.mjs
  header: entries come with C4, where the constructor first calls them) = my F2 condition (entries with the first link).
- [x] (c) No security concern of weight. No libm copied (generator: stdlib only, writes inputs, never outputs;
  make_libm_vectors.py docstring). Inputs: test-only, read through read_plain (plain file in a plain folder, capped,
  files.rs:157-166), length checked and NaN refused (libm_ops.rs:32-48), never in a players' build. Self-recording only
  on exactly "1" (libm_vectors.rs:30); CI sets only SHAMBLE_WRITE_TYPES (toolchain.yml:159), so CI checks, never
  records; the header binds the inputs' length and sha256 and libm's version (tied to the root line, :37-45); a
  re-record shows in the diff. Release-profile run with overflow checks on: toolchain.yml:162 runs `cargo test --release`
  for devtools, which runs the vectors: my item-3 no-panic condition is met in CI. The recorded file proves sameness
  across targets, not accuracy (by design, block 18; accuracy reported): not a security matter.
- [x] L5 (Low): CI does not run `make_libm_vectors.py --check` (toolchain.yml:165-166 run the chacha8 and f32
  generators only); local-checks.sh:26 does, and the checkpoint records CI's line as a request to the CI Builder
  (.checkpoints/cs4c-soldier.md:258). Until it lands, the 1 MB binary and its header could change together and CI would
  stay green; the binary is bound to the readable generator by local runs only. Fix: the CI line (a .github edit: main
  session, Mustafa asked first), or a STATUS follow-up naming it.
- [x] L2 widened and raised to Medium (now M1). Probe, closing direction (RUN):
  `strip! { const Q: [char; 2] = ['\'','}']; #[cfg(test)] mod leak { use content::map::MapDocument; } }` -> the exposed
  `}` closes strip!'s brace, depth returns to 0 and the leak module is marked test-only: `content` read as test. So
  M1 defeats my amendment's strip! plant too (both directions: a spurious `{` runs a mark past its end, a spurious `}`
  lets a mark stand inside a macro). rustfmt (RUN, rustfmt --edition 2024 --emit stdout on a scratch file): it writes
  `['\'', '{']` in plain code (harmless form), but keeps `['\'','{']` under #[rustfmt::skip] and inside a brace-
  delimited macro's input, exactly the strip! route. A regression for content's panic scan: the old tokenizer reads
  `'\''` right (rules_scan.rs:201-206). Medium: a real false pass in the marking items 4 and 5 rest on, small reach,
  backstops (Cargo for the dev-dependency; content's clippy guard only where clippy runs). Fix: escaped -> seek the
  closing quote from open + 3 (marking.rs:88); plants red in layering.rs: the `['\'','{']` tests-module line and the
  strip! `['\'','}']` line; in content_breaches: the first with `x.unwrap()` after the module.
- [x] (c) evidence completed. libm-inputs.bin at d7e260f7: 1,077,304 bytes, sha256 0ee2be59...d052 = the expected
  file's header (git show | sha256sum, and the worktree copy). `python3 -I -B make_libm_vectors.py --check`: exit 0,
  "as this script makes it" (writes nothing). Generator 41-129: stdlib only, its one write is `target.write_bytes` in
  no-argument mode, refused through a link (:120-123); --check reads only a plain file. JS diffs read: rules.test.mjs
  (+10) and wasm-rules.spec.ts (+8/-3) load the two files from crates/devtools/vectors, call the test-only module's
  libm_vectors, bite on one changed input byte (header named); the spec's made-up origin (rules.invalid) is unchanged.
  The Wasm bites only the header; a changed output digest is bitten natively (libm_vectors.rs:61-77) and PV1 (plants-
  4c2.txt:28) flipped a wasm32-only bit and turned Node red: session's run, not re-run by me.
- [x] Item 3's "against the correctly rounded reference" for the sincos vectors: superseded by block 18, accepted.
  Why: the property authority needs is the same bits on server and every client, which the f64 vectors hold
  (natively, Node, each browser); libm is not correctly rounded, so a reference compare of its f64 bits would fail by
  design. Accuracy rests on fmath.rs:138-151 (four facings, the route's correctly rounded f32, by bits) and on the
  reported figures (rust-reviewer-read.md:1323: exp <= 0.8148 ulp, sin 0.7539, cos 0.7178, narrowed 0 of 202,034; not
  re-run by me). Not a security matter.
- [x] Domain kept: facing_of (fmath.rs:67-86) takes whole quarter turns out by comparisons; for [0, 360) the rest is
  in [0, 90), radians in [0, pi/2), so rem_pio2_large stays unreachable while content refuses other facings
  (map.rs:66, :185, my earlier read). NaN falls to quarter 3, then sincos(NaN) returns early (rem_pio2.rs:162): no panic.
- [x] Item 2 correction: dependency_rules.rs:335 is `assert_ne!` of MATHS against its own edit: it exercises no check,
  so it is not the root-line plant. The root-line plant is P10 (plants-4c2.txt:27: `=0.2.16` loosened to `0.2.16`,
  exit 101, caught): the session's run, not re-run by me. Item 2 stays MET.
- [x] Item 3 stated plainly: the rule MET; three of my nine named plants NOT MET (L1).

## Not checked
- Nothing built or run except: the five digests, the inputs' sha256, the generator's --check, the marking probe
  (marking.rs@d7e260f7 compiled with rustc 1.99.1 into .gate/sa-c3-probe) and one rustfmt --emit stdout. No cargo test,
  clippy, gate, Node or browser run; the session's plant runs (plants-4c2.txt) taken as recorded.
- L1's three bites and the nested-fn trick: hand traces. L3 (TOML escape): by reading the code and TOML's escapes,
  not run through cargo. L4: by reading and git grep.
- fmath.rs's real call sites pass the scan: as claimed by the session's run, not re-run.
- My agent memory was neither read nor updated (the brief limits my writes to this file).

## Verdict: REVISE (one Medium; the batch's approval stands)
The five batch files equal the approved digests (item 1): the dependency approval stands whatever the findings.
Items: 1 MET; 2 MET; 3 rule MET, three named plants NOT MET (L1); 4 MET on the listed plants and controls, except M1;
5 MET, except M1. (a) within the ruling. (b) keeps the libm conditions. (c) no concern beyond L4 below.
- Medium M1: marking.rs's tokenizer misreads `'\''` (marking.rs:86-91), a run-confirmed false pass in layering.rs and
  content's panic scan, both directions (a spurious `{` or `}`), the strip! plant defeated. Fix: seek from open + 3;
  three plants (above).
- Low L1: rules_scan.rs LATER_BITES lacks `libm::expf(`, `::libm::exp(` outside the site, `extern crate libm as m;`
  (rule closes them by trace). Add three rows.
- Low L3: the manifest half (layering.rs:57-81) and names() (dependency_rules.rs:119-123) read raw TOML: an escaped
  name (`"content"`) passes both. Fix: refuse a backslash in a non-comment line of room's, client-endpoint's and
  sim's manifests; one plant each.
- Low L4: fixed lists: NEVER_CALLING (layering.rs:26) misses a future dependent of sim; handed_on misses `pub type`
  naming content. Fix: walk every crates/* but sim and devtools; count `pub type`; plants.
- Low L5: CI runs no `make_libm_vectors.py --check` (toolchain.yml:165-166 has the other two). Fix: the CI line, a
  .github edit for the main session, Mustafa asked first; or a STATUS follow-up naming it.
Any of these left open keeps the PR needs-mustafa until fixed or explicitly deferred.

DONE
