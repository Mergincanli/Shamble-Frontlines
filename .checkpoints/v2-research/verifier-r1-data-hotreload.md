Task: verify R1 data-hotreload (R1-DATA-REC1..REC8) of docs/research/r1-simulation-core.md, wave 1 step 3 of v2-research plan
Scope: REC1-REC8 rest on F1-F29 (all), legacy facts, LOOP_V2/EDITOR_V2/VISION_V2 refs, VISION_V2 §4 100 ms number.
- F1 confirmed · serde container-attrs.md, deny_unknown_fields: "Always error during deserialization when encountering unknown fields." / "When this attribute is not present, by default unknown fields are ignored for self-describing formats like JSON."
- F2 confirmed · same, deny_unknown_fields: "this attribute is not supported in combination with flatten, neither on the outer struct nor on the flattened field."
- F3 confirmed · same, try_from: "Deserialize this type by deserializing into `FromType`, then converting fallibly." "This type must implement `TryFrom<FromType>` with an error type that implements `Display`,"
- F5 confirmed · same, untagged: "When no variant matches, `untagged` does not produce an informative error" (note: source continues "but this can be improved by adding an expecting message."; draft omits that). tag: "Use the internally tagged enum representation, with the given tag."
- F4 confirmed (quotes) · field-attrs.md default: "If the value is not present when deserializing, use the `Default::default()`."; alias: "Deserialize this field from the given name *or* from its Rust name." "May be repeated..." ("without a migration step" is the draft's inference)
- F6 confirmed · serde_json Cargo.toml 1.0.154 float_roundtrip comment, quotes match exactly.
- F11 confirmed · Cargo.toml preserve_order comment matches exactly.
- F9 part 1 confirmed · Cargo.toml unbounded_depth: "Provide a method disable_recursion_limit to parse arbitrarily deep JSON structures without any consideration for overflowing the stack." (Deserializer page pending)
- F7 confirmed · docs.rs serde_json 1.0.154 Error: line() "One-based line number at which the error was detected."; classify(): "Category::Syntax - input that is not syntactically valid JSON"; "Category::Data - input data that is semantically incorrect"
Next: F8, F9 docs page, F10, F12-F14.
- F8 confirmed · docs.rs serde_path_to_error 0.1.20: "a wrapper that works with any existing Serde `Deserializer`" ... "exposes the chain of field names leading to the error."; example "dependencies.serde.version"
- F9 confirmed (feature-gate part) · docs.rs serde_json Deserializer::disable_recursion_limit: "This method is only available if serde_json is built with the "unbounded_depth" feature." Default limit value not on page (draft already says so; REC2 "default depth limit" rests on existence only: OK).
- F10 confirmed (quotes) · docs.rs serde_json value index: "The Value enum, a loosely typed way of representing any valid JSON value."; from_value "Interpret a `serde_json::Value` as an instance of type `T`." (migration-on-tree use is inference, labeled)
- F12 confirmed · github.com/ron-rs/ron README: "RON is not designed to be a fully self-describing format (unlike JSON)"; "While RON offers a best-effort implementation for `#[serde(flatten)]`, it may be unsupported in further cases"; "RON does not yet make any guarantees about roundtrips through `ron::Value`."; features now verbatim: "trailing commas allowed", "single- and multi-line comments", "enums are supported (and less verbose than their JSON representation)"; JS impl "ron-js", no TS listed. docs.rs/ron "limited support" bullets: "`#[serde(tag = "tag")]`, i.e. internally tagged enums", "`#[serde(untagged)]`...", "`#[serde(flatten)]`...". NOTE: docs.rs now shows ron 0.12.3, draft says 0.12.2 (version drift, minor).
- F13 confirmed · toml.md: "TOML aims to be a minimal configuration file format that's easy to read due to obvious semantics."; "A hash symbol marks the rest of the line as a comment, except when inside a string."; "Unspecified values are invalid."; "Values must have one of the following types." list (String, Integer, Float, Boolean, Offset Date-Time, Local Date-Time, Local Date, Local Time, Array, Inline Table: no null). docs.rs toml "toml 1.1.8+spec-1.1.0"; Value variants String/Integer/Float/Boolean/Datetime/Array/Table. Main-branch = spec 1.1.0 still unverified (no version on page), not load-bearing.
- F14 confirmed · github.com/dtolnay/serde-yaml: "This repository was archived by the owner on Mar 25, 2024. It is now read-only." "This project is no longer maintained." "serde_yaml = "0.9""
Next: F15-F19.
- F15 confirmed · datatracker draft-bhutton-json-schema-validation-01: §6.2.4 "The value of "minimum" MUST be a number, representing an inclusive lower limit for a numeric instance."; §6.1.2 enum / §6.5.3 required exist ("The value of this keyword MUST be an array."); §9 "commonly used information for documentation and user interface display purposes."; §9.1 "Both of these keywords can be used to decorate a user interface with information about the data"; §9.3 deprecated "applications SHOULD refrain from usage of the declared property." (UI purpose now verbatim: draft's "inferred" upgrades to confirmed). Status: expired individual I-D, 10 June 2022, Informational, "no formal standing". Minor: draft cites deprecated as §9, it is §9.3. default/examples not individually quoted (part of §9, not load-bearing beyond deprecated/default display).
- F16 confirmed · docs.rs schemars 1.2.3: "The default settings currently conform to JSON Schema 2020-12," ... "liable to change..."; "Any generated schema should match how serde_json would serialize/deserialize to/from JSON."; "Schemars will check for any #[serde(...)] attributes on types that derive JsonSchema"
- F17 confirmed · schemars docs/_includes/attributes.md: "many of which will be used by Schemars to generate more accurate schemas."; range forms incl `#[garde(range(min = 1, max = 10))]`; "Sets the `minimum`/`maximum` properties for number schemas."; length "Sets the `minLength`/`maxLength` properties for string schemas, ..."; pattern "Sets the `pattern` property for string schemas."; doc comments "then these will be used as the generated schema's `description`."
- F18 confirmed · docs.rs garde 0.23.0: "garde will generate an implementation of the Validate trait for you, allowing you to call the validate method."; "The context may be any type without generic parameters. By default, the context is ()."; "username[0]: length is lower than 3"; rule table verbatim incl "length","range","pattern","dive","custom". No wasm/no_std on page (consistent with draft's "unverified", OQ5).
Next: F19-F22.
- F19 confirmed · github.com/ajv-validator/ajv README Features: "Ajv implements JSON Schema draft-06/07/2019-09/2020-12 standards (draft-04 is supported in v6):"; "To install version 8"
- F20 confirmed · docs.rs notify 8.2.1 Known Problems: "...the actual events can differ a lot between file editors."; "Network mounted filesystems like NFS may not emit any events for notify to listen to."; "When watching a very large amount of files, notify may fail to receive all events."; "If you want debounced events (or don't need them in-order), see notify-debouncer-mini or notify-debouncer-full."
- F21 confirmed (quote) · docs.rs bevy_asset 0.20.0: "To enable asset hot reloading on desktop platforms, enable `bevy`'s `file_watcher` cargo feature." No web/wasm sentence. NOTE: REC5 writes "(browsers cannot watch files: F21)" as sourced; F21 does not say that (F21 itself labels it inferred).
- F22 confirmed (api-plugin part) · vite.dev/guide/api-plugin "Client-server Communication" (v8.3.4): "On the plugin side, we could use `server.ws.send` to broadcast events to the client:"; "server.ws.send('my:greetings', { msg: 'hello' })"; "On the client side, use `hot.on` to listen to the events:"; "To send events from the client to the server, we can use `hot.send`". api-hmr page pending.
Next: vite api-hmr, F23-F26.
- F22 confirmed (api-hmr part) · vite.dev/guide/api-hmr: hot.send "Send custom events back to Vite's dev server."; no mention of Web Workers (consistent with draft's "unverified", OQ8).
- F23 confirmed · MDN Object_initializer "Object literal syntax vs. JSON": "In JSON, `"__proto__"` is a normal property key. In an object literal, it sets the object's prototype."; MDN Object/proto: "The `__proto__` property is just an accessor property on `Object.prototype` consisting of a getter and setter function."; MDN Object/assign Description: "It uses `[[Get]]` on the source and `[[Set]]` on the target, so it will invoke getters and setters."
- Legacy facts checked in repo: shamble-game.js:41 rulesMerge uses Object.assign(d[k],src[k]) (confirmed); shamble-core.js:180 envMerge same (confirmed); shamble-game.js:661 version:1 and no `.version` read anywhere in shamble-*.js (grep, confirmed); shamble-editor.js:770 RULES_SCHEMA, :796 slider calls applyRules(_rules) (confirmed); legacy-tuning.json: 60 tables, groups gameplay 24/look 19/progression 11/simulation 4/state 2 (state = NAV, AI_STATE), 36 skipped, boot_changes present; FIXED_DT 0.016666666666666666, camYawBase 0.7853981633974483, T.sprintMult present (confirmed). ED13/GW12 inventory rows both mention load-time dedupe (REC4's "duplicate-tree dedupe (ED13)" OK; GW12 row "dedupeMapDef"), but the unit's legacy-fact bullet for ED13 describes storage fallback, not the dedupe (minor).
Next: F24-F29, design-doc refs.
- F24 confirmed · OWASP Prototype Pollution Prevention CS: "Developers should use `new Set()` or `new Map()` instead of using object literals:"; also "If objects have to be used then they should be created using the `Object.create(null)` API"
- F25 confirmed · OWASP Input Validation CS: Input Validation Strategies "Validate both syntax (the expected type and format) and semantics (whether the value makes sense for the operation)."; Common Pitfalls "Validate on the server even when the browser checks the same fields." "Client-side validation is bypassable; use it for immediate feedback."; Validate Structured Data "For JSON, explicitly configure required and additional properties"
- F26 confirmed · MDN JSON.parse, Description > The reviver parameter: "...numbers in JSON text will have already been converted to JavaScript numbers, and may lose precision in the process."
- F27 confirmed · MDN Cache-Control, Response Directives > immutable: "The `immutable` response directive indicates that the response will not be updated while it's fresh."; "A modern best practice for static resources is to include version/hashes in their URLs, while never modifying the resources..."
Next: F28, F29, design-doc refs.
- F28 confirmed · doc.rust-lang.org include_bytes (1.99.1, 2026-10-13, since 1.38.0): "Includes a file as a reference to a byte array." "The provided path is interpreted in a platform-specific way at compile time."
- F29 confirmed · MDN SubtleCrypto.digest: "Note: This feature is available in Web Workers."; "Secure context: This feature is available only in secure contexts (HTTPS), in some or all supporting browsers."; supported values include `"SHA-256"`.
All 29 findings checked. Next: VISION_V2 §4 100 ms, §6 trace, LOOP_V2/EDITOR_V2 refs.
- Design refs confirmed: VISION_V2.md:54 §4 "`[DEFAULT]` Matches feel fair up to about 100 ms of ping." (REC5 pass mark 2 number sourced); §6 items 1,3,5,6,7 exist with quoted words ("tuning needs no code changes" §6.3; "All server-side and tunable." §6.5); LOOP_V2.md:8 "Keep them all in one data table so they can be tuned without code changes."; LOOP_V2.md:90-91 §7 `[DECIDED]` feel kit "tuned live in a small **Feel Lab**"; EDITOR_V2.md:16 rule 1 "A map is plain, versioned JSON. Every entity has a stable ID."; :47-48 §4 1,000 commands byte-identical, "Every v1 map loads through a migration; saving and loading round-trips exactly."; §3 validation panel "bot reachability..., the 6 m gap rule, a symmetry check, and budgets" (REC7's "6 m gaps" sourced); :51 §5 double-click. legacy-tuning.json about: "Starting values, not decisions." STATUS.md:96 SEC-MDL-2 exists. tracer 16773312 at legacy-tuning.json:198.
- Pass marks: every number in REC1-REC8 pass marks is under "(proposed pass marks)"/"(proposed)" except 100 ms (sourced, confirmed) and 6 m (EDITOR_V2 §3, confirmed). None unmarked-unsourced.

FLAGS (ranked):
1. REC2 (and REC3/REC5/REC7 via REC2) "same code everywhere ... client's Wasm worker, the editor (through Wasm)": depends on garde (and the crate set) building for wasm32, which F18 says is undocumented (OQ5), and on the editor double-click rule vs Wasm (OQ11); neither is in "Rests on". Spike pass mark covers Wasm, so tested, but the basis gap is unlisted.
2. REC5 "(browsers cannot watch files: F21)": F21's source only says hot reload is desktop-only via file_watcher; the browser claim is the draft's own inference (F21 labels it inferred), cited in REC5 as if sourced.
3. REC5 server-to-client tick-synchronised mid-match reload (pass marks 2-3): trace to §6 is weak; §6.3 asks for tuning without code changes, not live reload; LOOP_V2 §7 "tuned live" is client feel data only. No §6 item plainly needs synchronised live sim reload into running matches (dev only, so cost bounded).
4. REC6 SHA-256 in the worker (F29): source says secure contexts only; the caveat is not carried into REC6 (file:// / LAN dev, cf. EDITOR_V2 §5 double-click, OQ11).
5. REC1 missing basis: "the editor already writes JSON (EDITOR_V2 rule 1)" and "fixed key order" (F11) used in Reason/body but not in "Rests on"; "shortest round-trip number text" from both serde_json and JS writer has no finding (spike pass mark 1 tests it).
6. REC7 missing basis: EDITOR_V2 §3 validation panel (reachability, 6 m gap, symmetry, budgets) used but not in "Rests on".
7. Minor: ron version 0.12.2 in F12, docs.rs now 0.12.3; F15 deprecated is §9.3 not §9; F5 omits the source's "but this can be improved by adding an expecting message"; ED13 legacy-fact bullet describes storage fallback, while REC4 cites ED13 for dedupe (inventory row ED13 does mention dedupe, so OK).

TABLE
F1 deny_unknown_fields errors; unknown ignored by default · serde container-attrs.md · confirmed · deny_unknown_fields
F2 not with flatten · same · confirmed · deny_unknown_fields note
F3 try_from fallible conversion · same · confirmed · try_from
F4 default + alias · serde field-attrs.md · confirmed · default, alias
F5 untagged poor errors; tag exists · container-attrs.md · confirmed · untagged, tag
F6 float_roundtrip exact, ~2x cost · serde_json Cargo.toml 1.0.154 · confirmed · [features] float_roundtrip
F7 line/column, Syntax vs Data · docs.rs serde_json Error · confirmed · line(), classify()
F8 path_to_error field path · docs.rs serde_path_to_error 0.1.20 · confirmed · crate root
F9 recursion limit default, unbounded_depth · Cargo.toml + docs.rs Deserializer · confirmed · unbounded_depth, disable_recursion_limit
F10 Value tree + from_value · docs.rs serde_json::value · confirmed · module intro, from_value
F11 preserve_order · Cargo.toml · confirmed · preserve_order
F12 RON limits + features · ron README, docs.rs ron · confirmed · README; "limited support" list
F13 TOML comments, no null, toml spec 1.1.0 · toml.md, docs.rs toml · confirmed · Objectives, Comment, value types list; crate version
F14 serde_yaml archived · github dtolnay/serde-yaml · confirmed · archive banner, README
F15 JSON Schema bounds/enum/required/annotations · draft-bhutton-json-schema-validation-01 · confirmed · §6.2.4, §6.1.2, §6.5.3, §9, §9.1, §9.3
F16 schemars 2020-12, follows serde · docs.rs schemars 1.2.3 · confirmed · crate root
F17 schemars reads garde range/length/pattern, doc comments · schemars attributes.md · confirmed · range, length, pattern, doc comments
F18 garde Validate, context, error paths · docs.rs garde 0.23.0 · confirmed · crate root
F19 Ajv 2020-12 · github ajv README · confirmed · Features
F20 notify gaps + debouncers · docs.rs notify 8.2.1 · confirmed · Known Problems
F21 Bevy hot reload desktop file_watcher · docs.rs bevy_asset 0.20.0 · confirmed · crate root (browser inference not in source)
F22 Vite server.ws.send / hot.on / hot.send · vite.dev api-plugin, api-hmr · confirmed · Client-server Communication; hot.send
F23 __proto__ in JSON / accessor / assign [[Set]] · MDN x3 · confirmed · Object literal syntax vs. JSON; proto; assign Description
F24 Map/Set over literals · OWASP Prototype Pollution CS · confirmed · Set/Map recommendation
F25 syntax+semantics, server-side, required/additional · OWASP Input Validation CS · confirmed · Input Validation Strategies; Common Pitfalls; Validate Structured Data
F26 JSON numbers lose precision · MDN JSON.parse · confirmed · Description > The reviver parameter
F27 immutable + hashed URLs · MDN Cache-Control · confirmed · Response Directives > immutable
F28 include_bytes · doc.rust-lang.org · confirmed · macro intro
F29 SHA-256 in workers, secure contexts · MDN SubtleCrypto.digest · confirmed · notes, algorithm
L-legacy facts (rulesMerge :41, envMerge :180, version:1 :661 unread, RULES_SCHEMA :770, applyRules :796, legacy-tuning shape/groups/floats/dead key sprintMult) · repo · confirmed
V4 100 ms · VISION_V2.md:54 §4 · confirmed
D-refs LOOP_V2 :8, §7; EDITOR_V2 §2 r1, §3, §4, §5 · repo · confirmed
Counts: 33 confirmed / 0 wrong / 0 unsupported.
DONE
