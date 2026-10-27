# R1 unit data-hotreload: data formats and hot reload of tuning

Brief question (quoted): "data formats and hot reload of tuning" (docs/V2_PHASE0_BRIEF.md §4, row R1, last item).
Unit scope (from the task): the file format for content and tuning and its tooling; typed loading with serde, validation
at load and a schema the editor and other tools can use; versioning and migration; hot reload in development on the
server and in the Wasm client, and how a running match takes a changed number; how content reaches the browser;
`docs/legacy-tuning.json` as v2's starting values; the consumers (map editor, faction pipeline §6.7, modes as data §6.1,
balance telemetry §6.6); server-side validation of anything a client or editor sends.

Author: engine-architect (R1 unit). Date: 2026-10-27. Status: draft, not verified. Every source read 2026-10-27.
"Verbatim" marks a passage WebFetch returned as an exact quote; a claim seen only in WebFetch's paraphrase is marked
"inferred".

## 1. Findings

### serde (typed loading)
- **R1-DATA-F1.** With `#[serde(deny_unknown_fields)]` deserialization fails on any unknown field; without it, unknown
  fields are silently ignored for JSON. Verbatim: "Always error during deserialization when encountering unknown
  fields." / "When this attribute is not present, by default unknown fields are ignored for self-describing formats like
  JSON." · Source: https://raw.githubusercontent.com/serde-rs/serde-rs.github.io/master/_src/container-attrs.md
  (source of serde.rs "Container attributes"; serde 1.x) · official · read 2026-10-27 · confirmed in the source.
- **R1-DATA-F2.** `deny_unknown_fields` cannot be combined with `flatten`. Verbatim: "this attribute is not supported
  in combination with `flatten`, neither on the outer struct nor on the flattened field." · Source: as F1 · official ·
  read 2026-10-27 · confirmed in the source.
- **R1-DATA-F3.** serde can load a type through a fallible conversion, so a value can be range- or format-checked as
  it is loaded. Verbatim: `try_from`: "Deserialize this type by deserializing into `FromType`, then converting
  fallibly." "This type must implement `TryFrom<FromType>` with an error type that implements `Display`" · Source: as F1
  · official · read 2026-10-27 · confirmed in the source.
- **R1-DATA-F4.** Missing fields can take a default and renamed fields can keep their old name, which allows additive
  format changes without a migration step. Verbatim: `default`: "If the value is not present when deserializing, use
  the `Default::default()`." `alias`: "Deserialize this field from the given name *or* from its Rust name. May be
  repeated to specify multiple possible names for the same field." · Source:
  https://raw.githubusercontent.com/serde-rs/serde-rs.github.io/master/_src/field-attrs.md (serde.rs "Field
  attributes") · official · read 2026-10-27 · confirmed in the source.
- **R1-DATA-F5.** Untagged enums give poor errors. Verbatim: "When no variant matches, `untagged` does not produce an
  informative error" · Source: as F1 · official · read 2026-10-27 · confirmed in the source. Internally tagged enums
  (`#[serde(tag = "type")]`) exist as the alternative (same page, confirmed).

### serde_json (the JSON loader)
- **R1-DATA-F6.** serde_json's default float parsing is "best-effort precision"; the `float_roundtrip` feature makes
  f64 → JSON → f64 exact at about twice the float-parsing cost. Verbatim: "Use sufficient precision when parsing fixed
  precision floats from JSON to ensure that they maintain accuracy when round-tripped through JSON. This comes at an
  approximately 2x performance cost for parsing floats compared to the default best-effort precision." "Unlike
  arbitrary_precision, this feature makes f64 -> JSON -> f64 produce output identical to the input." · Source:
  https://raw.githubusercontent.com/serde-rs/json/master/Cargo.toml (serde_json 1.0.154, `[features]` comments) ·
  official · read 2026-10-27 · confirmed in the source.
- **R1-DATA-F7.** serde_json errors carry a one-based line and column and a category that separates bad syntax from
  well-formed but wrong data. Verbatim: `line()`: "One-based line number at which the error was detected."
  `Category::Syntax`: "input that is not syntactically valid JSON"; `Category::Data`: "input data that is semantically
  incorrect" · Source: https://docs.rs/serde_json/latest/serde_json/struct.Error.html (1.0.154, page 2026-10-11) ·
  official · read 2026-10-27 · confirmed in the source.
- **R1-DATA-F8.** serde_path_to_error wraps any serde deserializer and reports the field path of an error. Verbatim:
  "provides a wrapper that works with any existing Serde `Deserializer` and exposes the chain of field names leading to
  the error." Example path "dependencies.serde.version". · Source: https://docs.rs/serde_path_to_error/latest/serde_path_to_error/
  (0.1.20) · official (crate docs) · read 2026-10-27 · confirmed in the source.
- **R1-DATA-F9.** serde_json has a recursion limit by default: unlimited depth needs the `unbounded_depth` feature
  and an explicit call. Verbatim (Cargo.toml): "Provide a method disable_recursion_limit to parse arbitrarily deep JSON
  structures without any consideration for overflowing the stack." Docs: "This method is only available if serde_json
  is built with the "unbounded_depth" feature." · Sources: as F6; https://docs.rs/serde_json/latest/serde_json/struct.Deserializer.html
  · official · read 2026-10-27 · confirmed in the source (the default limit's value is not stated on the pages read:
  unverified).
- **R1-DATA-F10.** serde_json can hold any JSON as an untyped tree and convert it to a typed value afterwards, so a
  migration can rewrite the tree before the typed load. Verbatim: "The Value enum, a loosely typed way of representing
  any valid JSON value." `from_value`: "Interpret a `serde_json::Value` as an instance of type `T`." · Source:
  https://docs.rs/serde_json/latest/serde_json/value/index.html (1.0.154) · official · read 2026-10-27 · confirmed in
  the source (the migration use is inferred).
- **R1-DATA-F11.** `preserve_order` keeps map key order through a `Value` round trip. Verbatim: "Make serde_json::Map
  use a representation which maintains insertion order. This allows data to be read into a Value and written back to a
  JSON string while preserving the order of map keys in the input." · Source: as F6 · official · read 2026-10-27 ·
  confirmed in the source.

### Other text formats
- **R1-DATA-F12.** RON (ron 0.12.2) has comments, trailing commas and native enums, but is not fully self-describing,
  supports `tag`, `untagged` and `flatten` only in part, and gives no round-trip guarantee through its `Value`.
  Verbatim: "RON is not designed to be a fully self-describing format (unlike JSON)"; "While RON offers a best-effort
  implementation for `#[serde(flatten)]`, it may be unsupported in further cases"; "RON does not yet make any
  guarantees about roundtrips through `ron::Value`." · Sources: https://github.com/ron-rs/ron (README);
  https://docs.rs/ron/latest/ron/ (0.12.2, page 2026-09-13) · official (project) · read 2026-10-27 · the three quoted
  limits confirmed in the source; the feature list (comments, trailing commas, enums) and the README's one JavaScript
  implementation ("ron-js", no TypeScript one) come from WebFetch's summary: inferred; ron-js's maturity: unverified.
- **R1-DATA-F13.** TOML is a configuration format with comments and no null value; the toml crate targets spec 1.1.0.
  Verbatim (spec text): "TOML aims to be a minimal configuration file format that's easy to read due to obvious
  semantics." "A hash symbol marks the rest of the line as a comment, except when inside a string." · Sources:
  https://raw.githubusercontent.com/toml-lang/toml/main/toml.md (main branch, no version on the page);
  https://docs.rs/toml/latest/toml/ (toml 1.1.8+spec-1.1.0, page 2026-10-09: its `Value` has String, Integer, Float,
  Boolean, Datetime, Array, Table) · official · read 2026-10-27 · the two quotes confirmed in the source; "no null"
  comes from the `Value` variants as WebFetch listed them and the spec's value list (verbatim: "Unspecified values are
  invalid."): inferred; that the main-branch text is exactly spec 1.1.0: unverified.
- **R1-DATA-F14.** The serde YAML crate is archived and unmaintained. Verbatim: "This repository was archived by the
  owner on Mar 25, 2024. It is now read-only." "(This project is no longer maintained.)" · Source:
  https://github.com/dtolnay/serde-yaml (last version line `serde_yaml = "0.9"`) · official (author) · read 2026-10-27
  · confirmed in the source.

### Schemas and validation
- **R1-DATA-F15.** JSON Schema 2020-12 has numeric bounds, enums, required properties, and annotations meant for UI
  and documentation (title, description, default, deprecated, examples). Verbatim: "The value of "minimum" MUST be a
  number, representing an inclusive lower limit for a numeric instance."; deprecated: "applications SHOULD refrain from
  usage of the declared property" · Source: https://datatracker.ietf.org/doc/html/draft-bhutton-json-schema-validation-01
  §6.1, §6.2, §6.5.3, §9 (Internet-Draft, 2020-12 dialect, published 2022-06-10, Informational, expired) · official
  (specification) · read 2026-10-27 · the quoted keyword definitions confirmed in the source; "meant for UI and
  documentation" is WebFetch's summary of §9.1: inferred.
- **R1-DATA-F16.** schemars 1.2.3 derives JSON Schema 2020-12 from Rust types and follows their serde attributes.
  Verbatim: default settings "currently conform to JSON Schema 2020-12"; "Any generated schema _should_ match how
  serde_json would serialize/deserialize to/from JSON." · Source: https://docs.rs/schemars/latest/schemars/ (1.2.3, page
  2026-10-12) · official · read 2026-10-27 · confirmed in the source (the page says the default draft "is liable to
  change").
- **R1-DATA-F17.** schemars reads garde and validator field rules into the schema (range → minimum/maximum, length,
  pattern) and turns doc comments into descriptions. Verbatim: "Validator and Garde allow setting
  `#[validate(...)]`/`#[garde(...)]` attributes to restrict valid values of particular fields, many of which will be
  used by Schemars to generate more accurate schemas."; forms listed: "`#[validate(range(min = 1, max = 10))]` /
  `#[garde(range(min = 1, max = 10))]` / `#[schemars(range(min = 1, max = 10))]`"; range: "Sets the
  `minimum`/`maximum` properties for number schemas."; length: "Sets the `minLength`/`maxLength` properties for string
  schemas"; doc comments "will be used as the generated schema's `description`." · Source: https://raw.githubusercontent.com/GREsau/schemars/master/docs/_includes/attributes.md
  (master, no date) · official · read 2026-10-27 · confirmed in the source.
- **R1-DATA-F18.** garde 0.23.0 validates derived Rust types with rules (range, length, pattern, dive into nested
  values, custom), can take a context value, and reports errors with field paths. Verbatim: "garde will generate an
  implementation of the `Validate` trait for you, allowing you to call the `validate` method."; "The context may be any
  type without generic parameters. By default, the context is `()`."; error example "username[0]: length is lower than
  3" · Source: https://docs.rs/garde/latest/garde/ (0.23.0, page 2026-10-03) · official · read 2026-10-27 · the quotes
  confirmed in the source; the full rule list is WebFetch's summary (inferred), but the range rule's form
  `#[garde(range(min = 1, max = 10))]` is quoted verbatim in F17's source. Using the context for cross-reference checks (a table of known ids) is inferred. Wasm support is not
  documented on the page (unverified).
- **R1-DATA-F19.** Ajv validates JSON Schema 2020-12 in JavaScript. Verbatim: "Ajv implements JSON Schema
  draft-06/07/2019-09/2020-12 standards (draft-04 is supported in v6):" · Source: https://github.com/ajv-validator/ajv
  (README; version 8 named, no release date shown) · official (project) · read 2026-10-27 · confirmed in the source.

### Hot reload
- **R1-DATA-F20.** notify 8.2.1 watches files natively, with known gaps: events differ between editors, network
  filesystems may send none, very large trees can drop events, and debouncing lives in companion crates. Verbatim: "the
  actual events can differ a lot between file editors."; "Network mounted filesystems like NFS may not emit any events
  for notify to listen to."; "If you want debounced events (or don't need them in-order), see notify-debouncer-mini or
  notify-debouncer-full." · Source: https://docs.rs/notify/latest/notify/ (8.2.1, page 2026-10-25), "Known Problems" ·
  official · read 2026-10-27 · confirmed in the source.
- **R1-DATA-F21.** Bevy documents its asset hot reload as a desktop feature behind a file-watcher flag. Verbatim: "To
  enable asset hot reloading on desktop platforms, enable `bevy`'s `file_watcher` cargo feature." · Source:
  https://docs.rs/bevy_asset/latest/bevy_asset/ (0.20.0, 2026-10-08) · official (engine docs; Bevy is an engine, not a
  shipped game) · read 2026-10-27 · confirmed in the source. That a browser client must therefore receive changes
  pushed to it, rather than watch files, is inferred.
- **R1-DATA-F22.** The Vite dev server can push custom events to client modules and receive events back. Verbatim:
  "On the plugin side, we could use `server.ws.send` to broadcast events to the client:" `server.ws.send('my:greetings',
  { msg: 'hello' })`; "On the client side, use `hot.on` to listen to the events:"; `hot.send`: "Send custom events back
  to Vite's dev server." · Sources: https://vite.dev/guide/api-plugin ("Client-server Communication");
  https://vite.dev/guide/api-hmr (both show Vite v8.3.4) · official · read 2026-10-27 · confirmed in the source. HMR
  inside a Web Worker is not covered on these pages (unverified).

### Prototype pollution and input validation (the legacy SEC-MDL-2 risk)
- **R1-DATA-F23.** `JSON.parse` makes `"__proto__"` an ordinary own property; `__proto__` is an accessor on
  `Object.prototype`; `Object.assign` writes with `[[Set]]` and so calls setters. Verbatim: "In JSON, `"__proto__"` is a
  normal property key. In an object literal, it sets the object's prototype."; "The `__proto__` property is just an
  accessor property on `Object.prototype` consisting of a getter and setter function."; "It uses `[[Get]]` on the source
  and `[[Set]]` on the target, so it will invoke getters and setters." · Sources:
  https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Operators/Object_initializer ("Object literal syntax
  vs. JSON"); https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Object/proto;
  https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Object/assign ("Description") ·
  official · read 2026-10-27 · each sentence confirmed in the source; that the three together are the legacy
  `rulesMerge` / `envMerge` mechanism is inferred (and matches shamble-game.js:41 and shamble-core.js:180, read here).
- **R1-DATA-F24.** OWASP recommends `Map`/`Set` or prototype-less objects in place of object literals for untrusted
  keys. Verbatim: "Developers should use `new Set()` or `new Map()` instead of using object literals:" · Source:
  https://cheatsheetseries.owasp.org/cheatsheets/Prototype_Pollution_Prevention_Cheat_Sheet.html (no page date) ·
  official (security guidance) · read 2026-10-27 · confirmed in the source.
- **R1-DATA-F25.** OWASP: validate syntax and meaning, on the server even when the browser checks the same thing, and
  for JSON set required and additional properties explicitly. Verbatim (as returned): "Validate both syntax (the
  expected type and format) and semantics (whether the value makes sense for the operation)."; "Validate on the server
  even when the browser checks the same fields."; "Client-side validation is bypassable; use it for immediate
  feedback."; "For JSON, explicitly configure required and additional properties" · Source:
  https://cheatsheetseries.owasp.org/cheatsheets/Input_Validation_Cheat_Sheet.html (no page date) · official ·
  read 2026-10-27 · confirmed in the source (quoted as WebFetch returned it).
- **R1-DATA-F26.** JavaScript turns JSON numbers into doubles at parse time, so integers beyond the double range lose
  precision. Verbatim: "numbers in JSON text will have already been converted to JavaScript numbers, and may lose
  precision in the process." · Source: https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/JSON/parse
  ("reviver parameter") · official · read 2026-10-27 · confirmed in the source.

### Delivery to the browser
- **R1-DATA-F27.** HTTP's `immutable` directive plus a hash in the URL is the documented pattern for static files that
  never change in place. Verbatim: "The `immutable` response directive indicates that the response will not be updated
  while it's fresh."; "A modern best practice for static resources is to include version/hashes in their URLs, while
  never modifying the resources" · Source: https://developer.mozilla.org/en-US/docs/Web/HTTP/Reference/Headers/Cache-Control
  · official · read 2026-10-27 · confirmed in the source.
- **R1-DATA-F28.** Rust can embed a file into the binary at compile time. Verbatim: "Includes a file as a reference to
  a byte array." "The provided path is interpreted in a platform-specific way at compile time." · Source:
  https://doc.rust-lang.org/std/macro.include_bytes.html (Rust docs 1.99.1, 2026-10-13; stable since 1.38.0) · official
  · read 2026-10-27 · confirmed in the source. (A tuning change then needs a rebuild of the Wasm and native binaries:
  inferred.)
- **R1-DATA-F29.** Browsers compute SHA-256 natively, also inside Web Workers, in secure contexts. Verbatim: "This
  feature is available in Web Workers."; "This feature is available only in secure contexts (HTTPS), in some or all
  supporting browsers."; supported values include "SHA-256" · Source:
  https://developer.mozilla.org/en-US/docs/Web/API/SubtleCrypto/digest · official · read 2026-10-27 · confirmed in the
  source.

No shipped-game source was read for this unit: the Valve Developer Wiki pages on console variables answered HTTP 403
(see Sources not read), and the one GDC lead found is a video talk on another unit's topic. Every recommendation below
rests on official documentation and specifications, plus the legacy facts in §2.

## 2. Legacy facts used
- **legacy-tuning.json (R7):** 60 tables, each `{where, group, look_brief_pending, values}`; `group` is gameplay,
  simulation, progression, look or state; plus `boot_changes` per map and 36 `skipped` functions (builders). *Tells v2:*
  the starting values already exist as JSON with provenance (file:line); the groups split cleanly into simulation
  content, client content, look data (pending the look brief) and runtime state (not content); the builders stay code.
- **legacy-tuning.json, WEAPONS:** the Beam Katana entry has no `reload` and no `tracer` key, while guns do; damage
  9999 and `T.meleeDamage` 999 act as "kills outright" values; colours are stored as decimal integers (tracer
  16773312). *Tells v2:* weapon kinds differ in shape (a tagged kind, F5) rather than optional keys; "kills outright"
  should be an explicit rule, not a sentinel number; colours need a type of their own.
- **legacy-tuning.json, floats:** `FIXED_DT` 0.016666666666666666 and `T.camYawBase` 0.7853981633974483 are written
  as shortest round-trip decimals. *Tells v2:* the content loader must reproduce these doubles bit for bit on native,
  Wasm and in the editor's JavaScript (F6, F26).
- **CD4, CD6, CD7, CD10, CD12, CD14 (dead keys):** `T.sprintMult`, `incomeBase`, `incomeRefinery`, `enemyBurst`,
  `enemyDamage`, `enemyPreferRange`, `TEAMS.short`, `WEAPONS.short`, and the Juggernaut's `seats` and `desc` are never
  read. *Tells v2:* a schema that rejects unknown keys (F1) plus an import that accounts for every legacy key stops dead
  tuning from looking live.
- **CD10 (TEAMS):** the order of the table silently decides who the first enemy team is. *Tells v2:* modes state team
  order and roles explicitly (§6.1).
- **CD12 (WEAPONS):** the note asks to split simulation fields (rate, spread, damage, range, falloff) from client feel
  fields (recoil, tracer colour). *Tells v2:* separate files, or separate sections, for what the simulation decides and
  what the client only shows (LOOP_V2 §7 feel kit).
- **GW2 / GE1 (RULES_DEFAULT, rulesMerge, applyRules):** shamble-game.js:41 merges a map's rules with
  `Object.assign(d[k], src[k])`, so any key, including `__proto__`, enters a rules sub-object (SEC-MDL-2; `envMerge` at
  shamble-core.js:180 is the same). The editor's `RULES_SCHEMA` (shamble-editor.js:770) holds min, max, step, unit and a
  tooltip per rule, but only the editor's sliders use it: loading a map enforces none of it. *Tells v2:* one schema both
  drives the editor's inputs and is enforced at load, on the server (F17, F25).
- **GE1 and the editor Rules panel:** each slider change calls `applyRules(_rules)` at once (shamble-editor.js:796).
  *Tells v2:* live re-application of tuning is already the legacy editor's habit; v2 keeps it, through a validated path.
- **GW12 (serializeMap):** writes `version:1` (shamble-game.js:661) that no code reads; rounds most positions to 0.1 m;
  lacks ids, arena size, some heights and tree rotation. *Tells v2:* the v1 save is migration input; the version must be
  read and acted on; no lossy rounding in the canonical writer (EDITOR_V2 §4).
- **GE11 (placeObjectives):** checks only the objective keys (finite, clamped) and falls back to DM_Cabana's values.
  **CM1:** DM_Cabana is the hidden default map. *Tells v2:* validation covers the whole document, and a missing
  objective is an error, never a silent fallback (STATUS "map data needs a full schema check server-side").
- **ED13 (map storage):** a storage quota failure falls back to memory without telling the user; there is no Import
  JSON. **CD2 (KVStore):** marked "port to TypeScript". *Tells v2:* saves report failure; the editor imports and exports
  the same canonical document the server validates.
- **CD19 (ENV_DEFAULT, envMerge):** the environment values are look data. *Tells v2:* they load through the same
  pipeline, but their values wait for the look brief.
- **Design inputs:** LOOP_V2 header ("Keep them all in one data table so they can be tuned without code changes");
  LOOP_V2 §7 `[DECIDED]` feel kit "tuned live in a small Feel Lab"; EDITOR_V2 §2 rule 1 (plain, versioned JSON, stable
  ids), rule 5 (document layer testable in Node), §4 (1,000 random commands undone and redone give a byte-identical
  document; every v1 map migrates; exact round trip).

## 3. Options

### Q1. The file format
| Option | Complexity and effort | Performance | Risk | Fit with VISION_V2 §6 and the editor |
|---|---|---|---|---|
| A. JSON for everything (content, tuning, maps, modes), one canonical writer | Lowest: serde_json on the Rust side, `JSON.parse` in the browser and editor, no second parser | Parse cost measured in the spike; exact floats need `float_roundtrip` at about 2x float-parse cost (F6) | No comments in files (documentation goes into the schema's descriptions, F15, F17); big integers lose precision in JS (F26) | EDITOR_V2 rule 1 already chose JSON for maps; legacy-tuning.json is JSON; JSON Schema tooling on both sides (F16, F19) |
| B. RON for hand-written content, JSON for editor maps | Two formats, two writers; the TypeScript editor has no first-class RON parser (F12: one JS implementation listed, maturity unread), so it would parse through the Wasm loader | Unknown, not measured | Partial `tag`/`flatten` support and no `Value` round-trip guarantee (F12) clash with migrations on a tree (F10) | Nice for Rust authors; weaker for the editor and for tools |
| C. TOML for flat tuning tables, JSON for the rest | Two formats; TOML's arrays of tables are awkward for nested definitions | Not measured | No null (F13); config-oriented (F13) | Good for a few flat tables, poor for units, abilities and maps |
| D. Text for authoring, compiled to a binary runtime format | A build step and a second schema; formats not researched in this run | Possibly faster to load; not measured | Unread sources: no claim here (open question R1-DATA-OQ2) | Only worth it if the spike shows JSON loading too slow |
YAML is not an option: the serde YAML crate is archived (F14).

### Q2. Typed loading and validation
- **A. One Rust content crate, typed all the way:** serde with `deny_unknown_fields` (F1), checked newtypes via
  `try_from` (F3), tagged enums for kinds (F5), garde rules for ranges and nested checks (F18), then a whole-set pass for
  cross-references (every id a mode, faction or map names exists). The same crate runs on the server (native), in the
  client's Wasm worker and in the editor (Wasm). Cost: more Rust types; flatten unavailable where unknown keys must fail
  (F2). Gain: one verdict everywhere.
- **B. Hand-written JSON Schema as the source of truth, validated by Ajv in TypeScript and a schema validator in
  Rust:** two validators of one schema can disagree; the Rust types still have to match the schema by hand. Not
  recommended.
- **C. Validate in the editor only:** ruled out: client-side checks are bypassable (F25), and STATUS already lists the
  server-side map check as debt.

### Q3. Versioning and migration
- **A. A version per file, migrations on the untyped tree:** read `version` first, run step functions N → N+1 on a
  `serde_json::Value` (F10), then the typed load. Additive changes use `default` and `alias` without a version bump
  (F4). Clear and testable with fixtures.
- **B. Versioned Rust types side by side (V1, V2 … structs, converted with `From`):** type-safe but multiplies types
  with each change; heavy for many small content files.
- **C. No versions, best effort:** the legacy path (GW12: `version:1` written, never read). Ruled out by EDITOR_V2 §4.

### Q4. Hot reload, and how a running match takes a changed number
- **A. Development only, swapped at a tick boundary, server first:** the server watches the content folder (notify
  with a debouncer, F20), re-validates the whole set, and on success swaps an immutable content set between two ticks;
  on failure keeps the old set and reports the error with file, line, column and field path (F7, F8). Clients in the
  match get the new set from the server with "applies from tick T". Offline play in the browser gets it from the Vite
  dev server's push (F22) and swaps at its own next tick. Production pins one content set per match.
- **B. Live tuning in production too:** the same mechanism switched on for live servers. Risk: matches whose rules
  change mid-way make telemetry (§6.6) and fairness harder; shipped-game practice was not read (OQ3).
- **C. No hot reload; restart the match on any change:** simplest, but works against LOOP_V2's "tuned without code
  changes" iteration and the decided Feel Lab.

### Q5. How content reaches the browser
- **A. Embedded in the Wasm binary (F28):** always consistent with the code; every tuning change means a rebuild and a
  bigger download.
- **B. A content bundle with its hash in the URL, cached as immutable (F27), its hash checked against the one the
  server names at join (F29):** cacheable, independent of code releases, verifiable.
- **C. Sent by the server over the game connection at join:** always matches the server; costs bandwidth at every join
  and caches nothing. Suits small things: private-match overrides, dev reloads.

## 4. Recommendations

- **R1-DATA-REC1. JSON is the one content format.** Every content and tuning file (maps, modes, factions, units,
  vehicles, weapons, ability data, rule tables) is JSON, one definition per file, written by one canonical writer
  (fixed key order, shortest round-trip number text, no lossy rounding). serde_json is built with `float_roundtrip`.
  Content integers stay within the range a double holds exactly; ids are strings.
  *Reason:* the editor already writes JSON (EDITOR_V2 rule 1), the browser parses it natively, the legacy values are
  JSON, and JSON Schema works on both sides; RON and TOML add a second format with gaps (F12, F13); YAML's crate is
  archived (F14); exact floats are needed for identical results on every side (legacy floats).
  *Rests on:* F6, F12, F13, F14, F15, F16, F19, F26; legacy facts (floats, GW12).
  *Serves:* §6.1, §6.3 ("tuning needs no code changes"), §6.5, §6.7; EDITOR_V2 §2 and §4.
  *A spike must prove (proposed pass marks):* (1) every double in legacy-tuning.json, plus a generated corpus of
  10,000 random doubles, parses to the same 64-bit pattern in native Rust, in Rust on Wasm and in `JSON.parse`, and is
  written back as the same text: 100 % equal; (2) loading a content set the size of the imported legacy tables plus
  two maps and two modes, on the native server, takes a time the spike reports, against a budget the ADR sets (no
  number is sourced yet).

- **R1-DATA-REC2. One Rust content crate loads and validates, the same code everywhere.** Stages: byte-size cap →
  JSON to tree with the default depth limit (F9) → read format and version, migrate (REC4) → typed load with
  `deny_unknown_fields` (F1), checked newtypes (F3) and tagged kinds (F5; never `untagged`, never `flatten` where unknown
  keys must fail, F2) → garde rules (F18) → whole-set checks (cross-references, every objective a mode needs is in the
  map, no fallback map) → a frozen content set with its hash. Errors name file, line and column for syntax (F7) and the
  field path for data (F8). The crate depends on nothing above it; the simulation, the server, the client's Wasm worker,
  the editor (through Wasm) and the tools use it.
  *Reason:* one verdict on every side; the legacy failures (unknown keys, no ranges at load, silent fallbacks, GW2,
  GE11, CM1) are each a stage here.
  *Rests on:* F1, F2, F3, F5, F7, F8, F9, F18, F25; legacy facts GW2/GE1, GE11, CM1, dead keys.
  *Serves:* §6.1, §6.3, §6.5, §6.7.
  *A spike must prove (proposed):* (1) a fuzz corpus of 10,000 mutated content files (unknown keys, `__proto__` keys,
  wrong types, out-of-range numbers, 10,000-deep nesting, 50 MB files, duplicate ids, dangling references) gives 0
  crashes and 0 accepted invalid files on native and on Wasm, and the same verdict and error path on both for 100 % of
  files; (2) every error message names the file and either line and column or a field path.

- **R1-DATA-REC3. The schema is generated from the Rust types and drives the editor's inputs.** schemars writes JSON
  Schema 2020-12 for every content type at build time (F16), with ranges and descriptions taken from the garde rules and
  doc comments (F17). The editor's inspector builds its fields from it (minimum and maximum become slider limits,
  description becomes the tooltip, default and deprecated are shown), as the legacy `RULES_SCHEMA` did, but the same
  limits are now enforced at load. The authoritative verdict always comes from the Rust validator (REC2); Ajv (F19) is
  optional, for instant per-field hints only. CI fails when the committed schema differs from the generated one.
  *Reason:* one source of truth for types, ranges and help text; client-side checks are only feedback (F25).
  *Rests on:* F15, F16, F17, F19, F25; legacy fact GW2/GE1 (`RULES_SCHEMA`).
  *Serves:* EDITOR_V2 §3 (inspector, validation panel), §6.1, §6.7.
  *A spike must prove (proposed):* for 100 % of fields with a garde range, the generated schema carries the same
  minimum and maximum; a value just outside a range is refused by the Rust validator and flagged by the schema-driven
  editor field; the schema check in CI fails on a deliberately changed type.

- **R1-DATA-REC4. Every file carries a format name and a version; migrations run on the tree.** Additive changes use
  `default` and `alias` (F4) without a bump; breaking changes bump the version and add one migration step N → N+1 on the
  untyped tree (F10); a file newer than the loader knows is refused, never guessed. The legacy v1 map (GW12) is the first
  migration's input; the duplicate-tree dedupe (ED13) is one of its steps. Each migration has golden fixtures.
  *Reason:* EDITOR_V2 §4 asks for migration of every v1 map and exact round trips; the legacy `version:1` was never read.
  *Rests on:* F4, F10, F11; legacy facts GW12, ED13.
  *Serves:* EDITOR_V2 §4; §6.1 and §6.7 (content keeps loading as formats change).
  *A spike must prove (proposed):* Cabana, DM_Tri and every saved map Mustafa supplies migrate and validate (100 %);
  migrate → save → load → save is byte-identical; a file with a future version is refused with a clear message.

- **R1-DATA-REC5. Hot reload in development, swapped between ticks; production pins content per match.** Server:
  notify with a debouncer watches the content folder (F20); a change re-validates the whole set (REC2); a valid set is
  swapped in between two ticks and sent to the match's clients with the tick it applies from; an invalid set leaves the
  old one running and reports errors to the log and to connected dev tools. Browser, offline or local: a small Vite
  plugin pushes changed files (F22) to the page, which hands them to the Wasm worker; the worker validates and swaps at
  its next tick (browsers cannot watch files: F21). Entities hold definition ids, not copies, so a changed number reaches
  live units on the next tick, while live state (current health, ammo) is left as it is. Client-only feel data (the Feel
  Lab, LOOP_V2 §7) reloads freely, since presentation decides nothing. Production: one content set per match; changes
  take effect at the next match.
  *Reason:* the iteration loop LOOP_V2 and §6.3 ask for, without letting a half-written file or a mid-match change
  break a match, and with the client predicting on the same numbers as the server.
  *Rests on:* F7, F8, F20, F21, F22; legacy fact GE1 (live `applyRules`); LOOP_V2 §7.
  *Serves:* §6.3, §6.5 ("tunable"), §6.6 (tune on evidence).
  *A spike must prove (proposed):* (1) a saved change to a weapon's damage applies on the dev server within 1 s of the
  save; (2) the client's Wasm simulation applies it at the same tick as the server, with 0 extra mispredictions beyond
  the change itself, under 100 ms injected latency with jitter (VISION_V2 §4: matches feel fair up to about 100 ms of ping); (3) 50 random valid tuning edits in a 5-minute bot match
  give 0 crashes and 0 desyncs; (4) a malformed save leaves the match running on the old numbers and shows the error
  within 1 s.

- **R1-DATA-REC6. Content reaches the browser as a hashed, cached bundle, and every match records its content hash.**
  The build writes the validated content set as one bundle whose file name holds its hash, served as immutable (F27);
  at join the server names the hash the match uses; the worker checks the bundle against it (SHA-256 is available in
  workers, F29) before the simulation starts. Small or per-match data (private-match overrides, dev reloads) comes over
  the game connection. Content is not embedded in the Wasm binary (F28), so tuning ships without a code release. Every
  match record (§6.6 telemetry) stores the content hash and the mode id, so pick and win rates are grouped by tuning
  version.
  *Reason:* client and server must run the same numbers for prediction; telemetry must know which numbers a match used.
  *Rests on:* F27, F28, F29.
  *Serves:* §6.1, §6.6, §6.7.
  *A spike must prove (proposed):* a client with a stale or altered bundle is refused or re-fetched before the match
  starts (100 % of 3 tampering cases: stale file, one changed byte, missing file); a repeat visit loads the bundle from
  the browser cache (0 bytes over the network for it); 100 % of spike match records carry a hash that resolves to an
  archived content set.

- **R1-DATA-REC7. The server validates everything a client or editor sends, through the same pipeline.** Private-match
  settings expose a named list of tunables with ranges from the schema (an allow list), never free paths into the
  content; a map uploaded for a private match goes through REC2 plus the editor's map checks (reachability, 6 m gaps,
  symmetry, budgets) on the server. On the TypeScript side, parsed JSON is never merged into plain objects with
  `Object.assign` or spread; untrusted keys go into `Map` (F24) or straight into the Wasm validator.
  *Reason:* client checks are bypassable (F25); the legacy merge is how `__proto__` got in (F23, GW2).
  *Rests on:* F1, F9, F18, F23, F24, F25; legacy facts GW2/GE1, CD19, GE11.
  *Serves:* §6.1 (rooms and private matches), §6.5 ("all server-side"); STATUS SEC-MDL-2 and the map-check debt.
  *A spike must prove (proposed):* the REC2 fuzz corpus sent through the private-match and map-upload paths gives 0
  accepted invalid payloads and 0 server crashes; a payload with a `__proto__` key is refused; the editor's TypeScript
  document layer passes a test that `Object.prototype` and every content object's prototype are unchanged after loading
  such a payload.

- **R1-DATA-REC8. Import legacy-tuning.json once, with every key accounted for.** A one-off import (phase 1) maps the
  gameplay and simulation groups to simulation content, the progression group to client content, leaves the look group
  pending the look brief, drops the state group (NAV, AI_STATE), and turns `boot_changes` into map data. Each imported
  value keeps its legacy `where` (file:line) as provenance; each dead key (CD4, CD6, CD7, CD10, CD12, CD14) is either
  dropped with a reason or kept by decision. These are starting values, not decisions (LEGACY_TUNING header).
  *Reason:* LOOP_V2's one data table, filled from the measured legacy values without carrying dead tuning forward.
  *Rests on:* legacy facts (legacy-tuning.json shape, dead keys, CD12 split); F1 (unknown keys fail, so nothing slips
  in unmapped).
  *Serves:* §6.3, §6.5, §6.7.
  *A spike must prove (proposed):* 100 % of the non-look, non-state values in legacy-tuning.json appear in the import
  report as mapped (with their v2 path) or dropped (with a reason); the imported set passes REC2.

## 5. Open questions
- **R1-DATA-OQ1. Comments in content files.** JSON has none. Are schema descriptions (F15, F17) and the editor's
  tooltips enough, or do hand-edited tuning files need comments (JSONC or JSON5, not researched here)? Decides it: how
  much tuning Mustafa edits as raw files rather than in the editor and Feel Lab; a short trial in the spike.
- **R1-DATA-OQ2. A binary runtime format.** Is JSON parsing in the Wasm worker fast and small enough, or should the
  build compile content to a binary format? Binary serde formats (postcard, bincode) and their maintenance status were
  not read in this run. Decides it: the spike's measured load time and the Wasm size serde_json adds, against budgets the
  ADR sets; then a read of those crates' docs and RustSec entries.
- **R1-DATA-OQ3. Shipped-game practice for mid-match tuning changes.** Valve's documentation of server-replicated
  console variables was refused (HTTP 403), so no shipped-game source backs REC5's "development only". Decides it: a
  readable copy of that documentation or another shipped-game source (R2 may hold one), then Mustafa's call on whether
  production may change numbers mid-match.
- **R1-DATA-OQ4. serde_json's default depth limit.** The pages read say a limit exists but not its value (F9). Decides
  it: the serde_json source or the spike's fuzz test (REC2).
- **R1-DATA-OQ5. garde on wasm32.** Its docs do not cover Wasm (F18). Decides it: the spike builds the content crate
  for wasm32 and runs the same validation tests there; if it fails, checked newtypes (F3) and hand-written checks
  replace garde, and schemars attributes carry the ranges.
- **R1-DATA-OQ6. Faction variants: copies or overrides.** Should a faction variant restate a whole unit or override a
  base one (`extends`)? Overrides bring back merge rules (the legacy SEC-MDL-2 lesson). Decides it: the faction design
  session (VISION_V2 §7 faction asymmetry) and the ability unit's definition shape.
- **R1-DATA-OQ7. What a reload may change mid-match.** Numbers in existing definitions, yes (REC5). New or removed
  definitions, map geometry and navigation data: probably a match restart. And when a maximum changes (max health),
  is current health clamped or scaled? Decides it: the ECS unit (whether entities hold ids or copies), R4 (navigation
  rebuild cost) and the game-designer.
- **R1-DATA-OQ8. Hot reload into a Web Worker under Vite.** The Vite pages read do not cover HMR inside workers (F22);
  REC5 relays through the page. Decides it: the spike, with the boundary unit's worker design.
- **R1-DATA-OQ9. The hash on the Rust side.** SHA-256 is native in browsers (F29); the Rust crate for it was not read.
  Decides it: rust-reviewer's crate pick, checked equal to WebCrypto on the spike's bundles.
- **R1-DATA-OQ10. Which tunables private matches expose.** REC7 asks for an allow list; its contents are design
  (VISION_V2 §3 private matches, LOOP_V2). Decides it: the game-designer and Mustafa.
- **R1-DATA-OQ11. The editor's double-click rule and Wasm.** EDITOR_V2 §5 keeps editor.html runnable by double-click;
  whether the Wasm validator loads from `file://` was not researched. Decides it: ADR-0002 (code structure and build)
  and R5's build research.

## 6. Sources read (all read 2026-10-27)
1. https://raw.githubusercontent.com/serde-rs/serde-rs.github.io/master/_src/container-attrs.md: serde.rs "Container attributes" source, serde 1.x, no page date.
2. https://raw.githubusercontent.com/serde-rs/serde-rs.github.io/master/_src/field-attrs.md: serde.rs "Field attributes" source, no page date.
3. https://docs.rs/serde_json/latest/serde_json/: serde_json 1.0.154, page 2026-10-11.
4. https://docs.rs/crate/serde_json/latest/features: serde_json 1.0.154 feature list (no descriptions; used only for the default feature).
5. https://raw.githubusercontent.com/serde-rs/json/master/Cargo.toml: serde_json 1.0.154 Cargo.toml feature comments.
6. https://docs.rs/serde_json/latest/serde_json/struct.Deserializer.html: serde_json 1.0.154.
7. https://docs.rs/serde_json/latest/serde_json/struct.Error.html: serde_json 1.0.154.
8. https://docs.rs/serde_json/latest/serde_json/value/index.html: serde_json 1.0.154.
9. https://docs.rs/serde_path_to_error/latest/serde_path_to_error/: serde_path_to_error 0.1.20.
10. https://github.com/ron-rs/ron: RON README, no release listed.
11. https://docs.rs/ron/latest/ron/: ron 0.12.2, page 2026-09-13.
12. https://docs.rs/toml/latest/toml/: toml 1.1.8+spec-1.1.0, page 2026-10-09.
13. https://raw.githubusercontent.com/toml-lang/toml/main/toml.md: TOML spec text, main branch, no version shown.
14. https://github.com/dtolnay/serde-yaml: archived 2024-03-25, serde_yaml 0.9.
15. https://datatracker.ietf.org/doc/html/draft-bhutton-json-schema-validation-01: JSON Schema Validation (2020-12), I-D published 2022-06-10.
16. https://docs.rs/schemars/latest/schemars/: schemars 1.2.3, page 2026-10-12.
17. https://github.com/GREsau/schemars: README (master).
18. https://github.com/GREsau/schemars/tree/master/docs: docs folder listing.
19. https://raw.githubusercontent.com/GREsau/schemars/master/docs/1.1-attributes.md: a Jekyll include stub only.
20. https://raw.githubusercontent.com/GREsau/schemars/master/docs/_includes/attributes.md: schemars attribute docs, master.
21. https://docs.rs/garde/latest/garde/: garde 0.23.0, page 2026-10-03.
22. https://github.com/ajv-validator/ajv: Ajv README (version 8).
23. https://docs.rs/notify/latest/notify/: notify 8.2.1, page 2026-10-25.
24. https://docs.rs/bevy_asset/latest/bevy_asset/: bevy_asset 0.20.0, 2026-10-08.
25. https://vite.dev/guide/api-hmr: Vite HMR API, v8.3.4 shown.
26. https://vite.dev/guide/api-plugin: Vite Plugin API, "Client-server Communication", v8.3.4 shown.
27. https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Object/proto: MDN, `Object.prototype.__proto__`.
28. https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/JSON/parse: MDN, `JSON.parse()`.
29. https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Operators/Object_initializer: MDN, "Object literal syntax vs. JSON".
30. https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Object/assign: MDN, `Object.assign()`.
31. https://cheatsheetseries.owasp.org/cheatsheets/Prototype_Pollution_Prevention_Cheat_Sheet.html: OWASP, no page date.
32. https://cheatsheetseries.owasp.org/cheatsheets/Input_Validation_Cheat_Sheet.html: OWASP, no page date.
33. https://developer.mozilla.org/en-US/docs/Web/HTTP/Reference/Headers/Cache-Control: MDN, `Cache-Control`.
34. https://doc.rust-lang.org/std/macro.include_bytes.html: Rust std docs 1.99.1 (2026-10-13).
35. https://developer.mozilla.org/en-US/docs/Web/API/SubtleCrypto/digest: MDN, `SubtleCrypto.digest()`.

Tier count: 35 official (specifications, official docs, project READMEs), 0 shipped game, 0 other.

### Sources not read
- https://developer.valvesoftware.com/wiki/Developer_Console_Control: HTTP 403 Forbidden (allow-listed domain).
- https://developer.valvesoftware.com/wiki/ConVar: HTTP 403 Forbidden (allow-listed domain).
- https://bevy.org/examples/asset/hot-asset-reloading/: outside the allow list (bevyengine.org redirects there).
- https://gdcvault.com/play/1024050/Modify-Everything-Data-Driven-Dynamic (GDC 2017, For Honor's data-driven
  modifiers): a search lead only; not read: video only (the search result reports a login wall).
- The "ron-js" JavaScript implementation listed in the RON README: not fetched; its URL was not shown.

## 7. Cross-unit notes
- **Ability and effect unit:** the shape of ability, effect and modifier definitions is theirs; REC1–REC4 give the
  format, loading, validation, schema and versioning those definitions use. For Honor's modifiers talk (GDC 2017) is a
  lead for them (not read).
- **ECS unit:** REC5 assumes entities refer to definitions by id; R1-DATA-OQ7 needs their answer.
- **Determinism unit:** REC1's float round-trip pass mark (native, Wasm and `JSON.parse` give the same bits) is input
  for identical results; the content hash (REC6) can be part of their desync checks.
- **Boundary and Web Worker unit:** how content bytes enter the worker and the Wasm memory (REC5, REC6), and whether
  TypeScript types for content come from the same schema (REC3) or from their type generator.
- **Fixed tick unit:** "applies from tick T" in REC5 depends on their tick numbering.
- **R2 netcode:** the join handshake carries the content hash (REC6); live reload messages and private-match overrides
  travel on the game connection; R2 may hold a shipped-game source for OQ3.
- **R4 physics and navigation:** whether map geometry or navigation data may reload mid-match (OQ7).
- **R5 toolchain:** the CI check that the committed schema matches the generated one (REC3); the Vite plugin (REC5);
  golden migration fixtures (REC4); OQ11.
- **R6 infrastructure:** match records store the content hash and mode id (REC6) for balance telemetry; immutable
  hosting of content bundles.
- **Security (R2's protocol review, and the deferred R6 baseline):** REC7's allow list and fuzz corpus.
