# R5 draft, question 2: strict TypeScript and lint rules (R5-TSLINT)

Brief question (quoted): "strict TypeScript and lint rules". Task scope: tsconfig strictness flags, ESLint with
typescript-eslint (which shared configs), formatting, and what each catches. All sources read 2026-10-28.

## Findings

- **R5-TSLINT-F1. TypeScript 7.0.2 (the native port) is the latest release; 6.0.3 is the latest 6.x.** Source:
  https://github.com/microsoft/TypeScript/releases · official · read 2026-10-28 · confirmed in the source: "TypeScript
  7.0.2" (20 Aug, marked "Latest"), "TypeScript 7.1 Beta" (20 Oct), "TypeScript 6.0.3" (16 Apr), "TypeScript 6.0"
  (23 Mar). The headers carry no year; the asset timestamps say 2026 (inferred). The 7.0.2 entry links the
  typescript-go repository, where the tag "was originally released at".
- **R5-TSLINT-F2. TypeScript 6.0 turned `strict` on by default and changed other defaults.** Source:
  https://www.typescriptlang.org/docs/handbook/release-notes/typescript-6-0.html ("Last updated: Oct 27, 2026") ·
  official · read 2026-10-28 · confirmed in the source: "**`strict` is now `true` by default**"; "**`module` defaults to
  `esnext`**"; the target is "the most recent supported ECMAScript spec version (effectively a floating target)";
  "**`noUncheckedSideEffectImports` is now `true` by default**"; "the default `types` value will be `[]` (an empty
  array)."
- **R5-TSLINT-F3. TypeScript 6.0 is a transition release: options it deprecates are gone in 7.0.** Source: same page ·
  official · read 2026-10-28 · confirmed in the source: "TypeScript 6.0 arrives as a significant transition release,
  designed to prepare developers for TypeScript 7.0, the upcoming native port"; "TypeScript 7.0 _will not_ support any of
  these deprecated options."; `--stableTypeOrdering` "is only intended to help diagnose differences between 6.0 and 7.0".
- **R5-TSLINT-F4. typescript-eslint 8.72.1 supports TypeScript below 6.1 and ESLint 8.57, 9 and 10.** Source:
  https://typescript-eslint.io/users/dependency-versions (site header v8.72.1) · official · read 2026-10-28 · confirmed
  in the source: "The version range of TypeScript currently supported is `>=4.8.4 <6.1.0`."; "The version range of
  ESLint currently supported is `^8.57.0 || ^9.0.0 || ^10.0.0`."
- **R5-TSLINT-F5. typescript-eslint does not work with TypeScript 7 yet, because 7.0 has no compiler API.** Source:
  https://github.com/typescript-eslint/typescript-eslint/issues/12518 ("TypeScript 7.0.2 Support", opened and closed as
  not planned on Jul 8, 2026; marked blocked by #10940 on Aug 10, 2026) · official (the project's tracker) · read
  2026-10-28 · confirmed in the source (maintainer kirkwaiblinger): "typescript-eslint isn't compatible with TS 7 at this
  time, because there is no TS 7 API at this time." and "There is nothing we can do about this until TS 7 provides an
  API."
- **R5-TSLINT-F6. `strict` is a family of checks.** Source: https://www.typescriptlang.org/tsconfig/ (`strict`; no
  date) · official · read 2026-10-28 · confirmed in the source: "The `strict` flag enables a wide range of type checking
  behavior that results in stronger guarantees of program correctness." The related list names alwaysStrict,
  strictNullChecks, strictBindCallApply, strictBuiltinIteratorReturn, strictFunctionTypes, strictPropertyInitialization,
  noImplicitAny, noImplicitThis and useUnknownInCatchVariables (list from the fetch summary, inferred).
- **R5-TSLINT-F7. `noUncheckedIndexedAccess` makes every index read possibly `undefined`.** Source: same page ·
  official · read 2026-10-28 · confirmed in the source: "Turning on `noUncheckedIndexedAccess` will add `undefined` to
  any un-declared field in the type." What it catches for us (inferred): reading slot `n` of a typed array view or of
  a per-team table without a bounds check, the shape of the legacy `SOLDIERS[r.id]` reads.
- **R5-TSLINT-F8. `exactOptionalPropertyTypes` separates "missing" from "set to undefined".** Source: same page ·
  official · read 2026-10-28 · confirmed in the source: "With exactOptionalPropertyTypes enabled, TypeScript applies
  stricter rules around how it handles properties on `type` or `interfaces` which have a `?` prefix."
- **R5-TSLINT-F9. `noImplicitOverride` requires the `override` keyword.** Source: same page · official · read 2026-10-28
  · confirmed in the source: "ensuring that functions which override include the keyword `override`."
- **R5-TSLINT-F10. `noFallthroughCasesInSwitch` reports fall-through cases.** Source: same page · official · read
  2026-10-28 · confirmed in the source: "Report errors for fallthrough cases in switch statements."
- **R5-TSLINT-F11. `noImplicitReturns` checks that every path returns.** Source: same page · official · read
  2026-10-28 · confirmed in the source: "When enabled, TypeScript will check all code paths in a function to ensure they
  return a value."
- **R5-TSLINT-F12. `erasableSyntaxOnly` rejects TypeScript-only constructs that need code generation.** Source: same
  page (read at offset 100000) · official · read 2026-10-28 · confirmed in the source: "The `--erasableSyntaxOnly` flag
  will cause TypeScript to error on most TypeScript-specific constructs". Which constructs (enums, namespaces, parameter
  properties) is from general knowledge, not the read text (unverified).
- **R5-TSLINT-F13. typescript-eslint's shared configs, and which are stable.** Source:
  https://typescript-eslint.io/users/configs · official · read 2026-10-28 · confirmed in the source: `recommended`
  "Recommended rules for code correctness that you can drop in without additional configuration."; `strict` "Contains all
  of `recommended`, as well as additional strict rules that can also catch bugs."; `stylistic` "Rules considered to be
  best practice for modern TypeScript codebases, but that do not impact program logic."; "With the exception of `all`,
  `strict`, and `strict-type-checked`, all configurations are considered "stable"."; `strict` is "not considered
  "stable" under Semantic Versioning (semver)."; "We do not recommend TypeScript projects extend from
  `plugin:@typescript-eslint/all`." The page's advice that `strict` suits teams where many developers are highly
  proficient in TypeScript comes from the fetch summary (inferred).
- **R5-TSLINT-F14. ESLint's `no-restricted-imports` forbids imports by path pattern, with a message.** Source:
  https://eslint.org/docs/latest/rules/no-restricted-imports (latest is v10.13.0 in the version switcher) · official ·
  read 2026-10-28 · confirmed in the source: "Disallow specified modules when loaded by `import`"; `patterns` "allows
  you to specify multiple modules to restrict using `gitignore`-style patterns or regular expressions."; `message` "used
  to specify a custom message."
- **R5-TSLINT-F15. Typed linting asks TypeScript's service for types and costs a build of the project.** Source:
  https://typescript-eslint.io/getting-started/typed-linting · official · read 2026-10-28 · confirmed in the source:
  `projectService: true` "indicates to ask TypeScript's type checking service for each source file's type
  information."; the cost: "incur the performance penalty of asking TypeScript to do a build of your project before
  ESLint can do its linting."; "For small projects this takes a negligible amount of time (a few seconds or less)".
- **R5-TSLINT-F16. `no-floating-promises` reports promises nobody handles, and `recommended-type-checked` turns it on.**
  Source: https://typescript-eslint.io/rules/no-floating-promises/ · official · read 2026-10-28 · confirmed in the
  source: "Require Promise-like statements to be handled appropriately."; it also "reports when an Array containing
  Promises is created and not properly handled." What it catches for us (inferred): an unawaited WebTransport write, a
  Worker start-up promise or a `KVStore`-style async save whose error vanishes.
- **R5-TSLINT-F17. `switch-exhaustiveness-check` requires every member of a union to have a case.** Source:
  https://typescript-eslint.io/rules/switch-exhaustiveness-check/ · official · read 2026-10-28 · confirmed in the source:
  "Require switch-case statements to be exhaustive."; with `considerDefaultExhaustiveForUnions` (default `false`) off, a
  `default` does not count: "If set to true, a `switch` statement over a union type that includes a `default` case is
  considered exhaustive." Whether a shared config includes it: not stated on the page (confirmed by absence). What it
  catches for us (inferred): a new message kind or entity kind from the generated types that the client does not handle.
- **R5-TSLINT-F18. typescript-eslint advises against ESLint's `no-undef` in TypeScript.** Source:
  https://typescript-eslint.io/troubleshooting/faqs/eslint/ · official · read 2026-10-28 · confirmed in the source: "We
  strongly recommend that you do not use the `no-undef` lint rule on TypeScript projects."; "The checks it provides are
  already provided by TypeScript without the need for configuration."
- **R5-TSLINT-F19. ESLint's `no-restricted-globals` forbids chosen global names.** Source:
  https://eslint.org/docs/latest/rules/no-restricted-globals · official · read 2026-10-28 · confirmed in the source:
  "Disallow specified global variables."; "early Internet Explorer versions exposed the current DOM event as a global
  variable `event`, but using this variable has been considered as a bad practice for a long time."
- **R5-TSLINT-F20. ESLint's flat config is one `eslint.config.*` file with `files` globs per block.** Source:
  https://eslint.org/docs/latest/use/configure/configuration-files (latest v10.13.0) · official · read 2026-10-28 ·
  confirmed in the source: "The ESLint configuration file may be named any of the following:" (`eslint.config.js`,
  `.mjs`, `.cjs`, `.ts`, `.mts`, `.cts`); `files` is "An array of glob patterns indicating the files that the
  configuration object should apply to."
- **R5-TSLINT-F21. ESLint recommends a formatter instead of its own formatting rules, which it deprecated.** Source:
  https://eslint.org/blog/2023/10/deprecating-formatting-rules/ (26 October 2023) · official · read 2026-10-28 ·
  confirmed in the source: "We recommend using a source code formatter instead of ESLint for formatting your code."; the
  rules "will not be removed until at least ESLint v10.0.0 (if not later)." The post names Prettier and dprint (fetch
  summary, inferred).
- **R5-TSLINT-F22. Prettier formats TypeScript, JSON, Markdown and YAML.** Source: https://github.com/prettier/prettier
  (README; no version shown) · official (the project's own page) · read 2026-10-28 · confirmed in the source: "Prettier is
  an opinionated code formatter."; the language list includes TypeScript, JSON, Markdown and YAML.
- **R5-TSLINT-F23. Biome is a formatter and linter that claims 97 % Prettier compatibility.** Source:
  https://github.com/biomejs/biome (README; no version shown) · official (the project's own page) · read 2026-10-28 ·
  confirmed in the source: "Biome offers formatter and linter, usable via CLI and LSP."; its formatter "scores **97%
  compatibility with _Prettier_**"; "more than 500 rules". Type-aware rules: not stated (confirmed by absence).
- **R5-TSLINT-F24. `isolatedModules` warns about code a single-file transpiler cannot handle, and Vite requires it.**
  Sources: https://www.typescriptlang.org/tsconfig/ (offset 100000) and R5-LAYOUT-F10 · official · read 2026-10-28 ·
  confirmed in the source: "Setting the `isolatedModules` flag tells TypeScript to warn you if you write certain code".
- **R5-TSLINT-F25. `verbatimModuleSyntax` (TypeScript 5.0) is the option for how imports are kept or dropped.** Source:
  https://www.typescriptlang.org/tsconfig/ (offset 100000) · official · read 2026-10-28 · confirmed in the source only as
  "TypeScript 5.0 introduces a new option called `--verbatimModuleSyntax` to simplify the situation."; that it makes
  type-only imports explicit (`import type`) is inferred.

### Legacy facts used
- **TL6** (drop): check 5b exists because classic scripts resolve names at run time; in v2 the type checker does it.
  The lesson carries: a reviewed allowlist of browser globals, so a typo that happens to be a window property (`name`,
  `status`, `event`) fails. Pin the tool versions (CI pins Node 22).
- **TL10 / KT9** (redesign): keep instant per-edit feedback through exit code 2; in v2 a fast `tsc` on the touched
  project, not `node --check`.
- **TL4, TL7** (redesign): the player build never loads editor code; the client reaches the sim only through the Wasm
  boundary (R5-LAYOUT-REC2).
- CLAUDE.md code rule: dynamic text through `textContent`, never `innerHTML` (security, carried to v2 as a lint target;
  the rule that enforces it was not read: OQ5).

## Options

### Which TypeScript
| Option | Type check | Typed lint | Risk | Effort |
|---|---|---|---|---|
| A. TypeScript 6.0.x everywhere, exact pin | `tsc -b` (6.0) | typescript-eslint works (F4) | slower checker than 7; 6.0 is a transition release (F3) | low |
| B. TypeScript 7 for checking, 6.0 kept for typed lint | `tsgo` (7.0) | typescript-eslint on 6.0 | two compilers can disagree (F3's `--stableTypeOrdering` exists for 6-vs-7 differences); two pins | medium |
| C. TypeScript 7 plus another type-aware linter | 7.0 | a linter built for 7 (not read: outside the allow list) | unread; replaces typescript-eslint's rules | unknown |

### Which lint configs
| Option | Catches | Cost |
|---|---|---|
| `recommended-type-checked` + `stylistic-type-checked` | correctness rules with types, `no-floating-promises` (F16) | stable under semver (F13) |
| `strict-type-checked` + `stylistic-type-checked` | more bug-catching rules (F13) | not semver-stable: a minor upgrade can add errors, so pin exactly and upgrade in its own change-set |
| `all` | everything | advised against (F13) |

### Formatter
| Option | Fit | Risk |
|---|---|---|
| Prettier (F22) | the formatter ESLint's post names (F21); formats TS, JSON, Markdown, YAML | one more dev dependency |
| Biome (F23) | formatter and linter in one, 97 % Prettier-compatible | does not replace typed lint (not stated); a second linter would overlap ESLint |
| No formatter, lint rules only | none | ESLint deprecated its formatting rules (F21) |

## Recommendations

- **R5-TSLINT-REC1. TypeScript 6.0.x, pinned exactly, with one shared strict base tsconfig.** Base options: `strict:
  true` (written out even though 6.0 defaults to it, so the intent survives a default change), `noUncheckedIndexedAccess`,
  `exactOptionalPropertyTypes`, `noImplicitOverride`, `noFallthroughCasesInSwitch`, `noImplicitReturns`,
  `isolatedModules`, `verbatimModuleSyntax`, `erasableSyntaxOnly`, and the `lib` and `types` each package needs set
  explicitly (6.0's `types` default is empty, F2). Every package extends the base and is a project reference built with
  `tsc -b`. Move to TypeScript 7 in its own change-set once typescript-eslint supports it (F5), so one compiler does both.
  Reason: typed lint needs TypeScript below 6.1 (F4, F5), and one compiler avoids two checkers disagreeing (F3); each
  flag closes a gap named in F6–F12 and F24–F25; `erasableSyntaxOnly` and `isolatedModules` keep files safe for Vite's
  per-file transpile (R5-LAYOUT-F10).
  Rests on: R5-TSLINT-F1, F2, F3, F4, F5, F6, F7, F8, F9, F10, F11, F12, F24, F25; R5-LAYOUT-F10, F13.
  Serves: none directly (toolchain); it protects the client side of §6.1–§6.9.
  A spike must prove (proposed): one planted violation per flag (an unchecked index read, `x = undefined` into an
  optional field, a missing `override`, a fall-through `case`, a path without `return`, an `enum`, a value import used
  only as a type) each fails `tsc -b`; the clean tree passes; `tsc -b` on the whole client takes under 30 s on the CI
  runner, warm.
- **R5-TSLINT-REC2. ESLint 10 flat config with typescript-eslint `strict-type-checked` plus `stylistic-type-checked`,
  exact pins, and four project rules.** Typed linting through `projectService: true`; `--max-warnings 0` in the gate.
  Rules added on top: `@typescript-eslint/switch-exhaustiveness-check` with `considerDefaultExhaustiveForUnions: false`;
  `no-restricted-imports` patterns for the layering rules (`client` ↛ `editor`; `sim-worker` and `net` ↛ rendering and
  DOM packages; nothing ↛ `legacy/`); `no-restricted-globals` with a reviewed list (`event`, `name`, `status` and the
  rest of TL6's lesson, each with a message); `no-undef` off. `no-floating-promises` comes with the config (F16).
  Reason: strict catches more bugs (F13) and the code is written by agents that work in TypeScript every day, so the
  proficiency caveat matters less; strict is not semver-stable, so the pin is exact and upgrades are their own
  change-set (F13); exhaustive switches catch an unhandled generated message kind (F17); the import rule enforces the
  layering before any build (F14); restricted globals keep the TL6 lesson (F19); TypeScript already does `no-undef`'s
  job (F18).
  Rests on: R5-TSLINT-F4, F13, F14, F15, F16, F17, F18, F19, F20; legacy TL4, TL6, TL7.
  Serves: §6.1 (a missed message kind or an unhandled transport promise is a netcode bug on the client); otherwise
  toolchain.
  A spike must prove (proposed): planted violations fail the lint, one per rule (an unawaited promise, a switch missing
  one generated kind while having a `default`, an import of `editor` from `client`, a bare `event`); the clean tree has 0
  errors and 0 warnings; typed lint of the whole TypeScript tree takes under 60 s on the CI runner, warm (F15 says small
  projects take seconds).
- **R5-TSLINT-REC3. Prettier formats the TypeScript packages and their JSON; the gate runs it in check mode.** Scope:
  `packages/**` and the root config files, not `docs/` (reflowing reviewed prose would bury content changes) and not
  `legacy/` (frozen). Formatting stays out of ESLint.
  Reason: ESLint's own advice (F21); Prettier is the formatter it names and covers our file types (F22); Biome would add
  a second linter beside the typed one without type-aware rules on record (F23).
  Rests on: R5-TSLINT-F21, F22, F23.
  Serves: none (toolchain).
  A spike must prove (proposed): `prettier --check` fails on a planted misformat; after `prettier --write` on the tree,
  ESLint reports 0 problems (no rule fights the formatter); line endings are LF (KT19).
- **R5-TSLINT-REC4. The per-edit hook checks only what changed, fast.** On a `.ts` edit: `tsc -b` on the touched
  project (incremental) and ESLint on the file; on an `.rs` edit: `cargo check -p` on the crate (R5-RUSTLINT); exit 2 on
  failure, as today's syntax hook does. The full gate stays the only judge.
  Reason: TL10 and KT9 keep instant feedback; Vite does not type-check (R5-LAYOUT-F10); typed lint costs a project build
  (F15), so the hook must stay per project.
  Rests on: R5-TSLINT-F15; R5-LAYOUT-F10; legacy TL10, KT9.
  Serves: none (toolchain).
  A spike must prove (proposed): a planted type error in one file is reported by the hook within 10 s, p95 over 20
  edits, on Mustafa's Windows machine and on Linux; the hook path works from any working folder (KT9's fail-open
  relative path is fixed).

## Open questions
- **R5-TSLINT-OQ1. When does typescript-eslint support TypeScript 7?** Tracked in its issue #10940 (not read); F5 says it
  waits on a TypeScript 7 API. Decides it: re-reading #10940 and the dependency-versions page at the phase 1 start.
- **R5-TSLINT-OQ2. Is a TypeScript-7-native type-aware linter worth it (option C)?** Search results point to one; its
  pages are outside the allow list and were not read. Decides it: reading its docs if Mustafa allows the domain.
- **R5-TSLINT-OQ3. Do the generated types pass `erasableSyntaxOnly`?** If ts-rs or wasm-bindgen emit `enum` or
  `namespace`, the flag rejects them (F12). Not read. Decides it: R5-LAYOUT-REC3's spike runs `tsc -b` on the generated
  package.
- **R5-TSLINT-OQ4. `exactOptionalPropertyTypes` and `noUncheckedIndexedAccess` against three.js's own types.** The cost
  of these flags in calls into three.js was not measured. Decides it: the spike counts the errors and the casts the
  client needs.
- **R5-TSLINT-OQ5. Which rule enforces "`textContent`, never `innerHTML`" for dynamic text?** No rule for it was read.
  Decides it: security-auditor with R6's security baseline; ESLint's `no-restricted-properties` or `no-restricted-syntax`
  are candidates (not read).
- **R5-TSLINT-OQ6. `skipLibCheck`.** Off checks every `.d.ts`, including the generated ones (tsconfig reference, S17 in the notes: "Skip type
  checking of declaration files."), at a cost not measured. Decides it: the spike's `tsc -b` time with it on and off.

| Recommendation that hinges on a source not read | Source | Reason not read |
|---|---|---|
| REC1 (when to move to 7) | https://github.com/typescript-eslint/typescript-eslint/issues/10940 | not fetched in this run (found linked from #12518); OQ1 |
| REC3 (Prettier against dprint) | https://prettier.io/docs/ and https://dprint.dev/ | outside the allow list |
| REC2 (option C ruled out for now) | the TypeScript-7-native linters' docs (oxc.rs) | outside the allow list |

## Sources read (R5-TSLINT)
S9, S10, S11, S12, S13, S14, S15, S16, S17, S18, S19, S20, S21, S22, S23, S24, S25 (notes file). Not read: prettier.io,
dprint.dev, oxc.rs, biomejs.dev (outside the allow list); typescript-eslint issue #10940 (not fetched).
Search leads only (not sources): infoq.com, dev.to, ecorpit.com, mergify.com, coddy.tech, devblogs.microsoft.com.

## Cross-topic notes
- security-auditor: OQ5 (`innerHTML`), and the dependency pins of the lint stack (R6).
- graphics-engineer: OQ4 (three.js types under the strict flags).
- infra-engineer: the lint and type-check times (REC1, REC2) are CI job budgets.
