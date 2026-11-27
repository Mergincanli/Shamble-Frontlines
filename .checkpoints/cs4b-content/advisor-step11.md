Task: advisor rulings for 4b-2 step 11 (schema writer): Q1 default column, Q2 descriptions, Q3 coverage+booleans, Q4 schema file count; plus STEP11-ROOM dependency.
Plan (blocks):
B1 read sources: block 1.10, ADR 0005 CD-3/CD-4, count rule, read.rs/json.rs/movement.rs/map.rs/ts_export.rs heads, test-list :110
B2 rule Q1
B3 rule Q2
B4 rule Q3
B5 rule Q4
B6 STEP11-ROOM dependency
Notes:
B1 DONE (sources read). Findings:
- F1 read.rs:1-3 no fallback, confirmed. CD-4 (adr 0005:358-361) "additive changes use default and alias with no version bump" -> when it first applies, read.rs's "nothing missing" rule changes: that change comes to the advisor then.
- F2 CD-3 :336-349 says descriptions "from the garde rules and the doc comments" via schemars; the built design is hand tables (block 1.10). Doc comments don't ship but a hand writer can't read them; test-only table meets CD-3's aim.
- F3 movement.rs:18-34 FIELDS shipped; unit column unread at run time (brief's measure) -> Low follow-up: unit ships unread.
- F4 json.rs:64-70 key grammar lower-case only: schema keys ($schema, additionalProperties, exclusiveMinimum, maxItems...) are NOT content grammar -> schema files can't be read back by json.rs; fine, they are not content. write.rs:85-100 escapes only " \n \\ : descriptions/patterns must be printable ASCII (else text != JSON.stringify of non-ASCII? JSON.stringify keeps non-ASCII raw, but control chars differ). Condition: printable ASCII test.
- F5 is_id json.rs:77-92: pattern alone misses reserved names con/prn/aux/nul/com0-9/lpt0-9 and the 64-byte cap -> schema id needs pattern + maxLength 64 + not{pattern reserved}; test both agree.
- F6 map.rs:316-330 aim reach is a one-sided per-value check on bounds (min side x,z >= -327.67, y >= 0; max side x,z <= 327.67, y <= 655.35). Proposal puts it in descriptions -> schema would offer bounds to 10,000 m the loader refuses: REVISE, carry as prefixItems per axis.
- F7 FACING is the only exclusiveMaximum; proposal's Vitest reads only movement schema -> exclusiveMaximum never exercised in Vitest: REVISE, Vitest covers both schemas.
- F8 ts_export.rs:63-79 files_in refuses any folder; must allow exactly `schema` real folder; write mode deletes only .ts; schema write mode only .json in schema/.
- F9 count rule (advisor-count-rule.md B4(2)): generated stays counted to end of 4b-2 because dropping it moves a trigger. Same reasoning applies to schema files.
Next: B2-B6 rulings.
B1b DONE (after advisor tool): card l.533-572 read: rung 1 at 6,750 (count at step 10's end 6,635: not fired, step 11 built by the ladder); look ahead before 12 = count + ~1,060 + 350; reaching 8,450 -> stop and ask before the step. Row 12 grows by the map reading (advisor-cutC Q2). PM-0005-5 scan is reread-sheet row 2's (4b-1, "a scan of the field tables: every array has a bound"); bundle files bound: bundle.rs:24 MAX_FILES=16, :182, test :440 -> not step 11's; no bundle schema wanted.
- F10 the tree type (json.rs:35-41 Value: Object, Array, Text, Number) has no boolean or null: a schema built as a Node tree and written by write.rs cannot hold a boolean, by type. Good; say so.
- F11 map keys repeat (min/max in bounds, shapes, regions; id in 4 places): a description table "keyed by each field's name" is ambiguous -> key by object and key.
- F12 Rust has no regex crate: id pattern agreement can be tested only in Vitest; adding a regex/jsonschema/Ajv/schemars dep is a dependency change (security-auditor, his DEP rules) -> forbid for step 11.
- F13 "takes the end itself" wrong at exclusive ends (Range::above min, FACING max): there the end is refused and the next double inside accepted.
- F14 aim reach ends: next double past +-327.67 is refused by the bounds grid rule first (check_box) -> equality of schema value with constant only; loader side stays 4b-1's 1 cm plants.
B2 Q1 RULED: APPROVE. No `default` (nor `deprecated`) keyword or column now; a column of empty values is a "for later" feature. Conditions: report says left out and why (read.rs:1-3; no optional field in v1); sheet rows 2 and 11 get a pointer to this ruling; the first additive field (CD-4) brings the column AND the change to read.rs's "nothing missing" rule to the advisor agent first.
B3 Q2 RULED: APPROVE with conditions: (a) test-only table in schema.rs (#![cfg(test)]); (b) keyed by object and key (F11), test: every property of both schemas has exactly one description and the table no other; (c) descriptions printable ASCII, state only name, unit, range and what the code does with the value (file:line in the code comment, not in the text), no tuning advice; (d) evidence after build: 3 distinctive description strings searched in the built players' file bytes: 0 hits (same method as the unit measure); (e) report records the reading of "columns" and that CD-3's schemars/doc-comment route is not used (hand writer, block 1.10). Low follow-up L1: `unit` column ships unread (brief's measure): STATUS follow-up, not this step.
B4 Q3 RULED: REVISE; fold in and build, no return unless a condition cannot be met. Changes:
 C1 (High, wrong claim) open ends: closed end -> end accepted, next double outside refused/flagged; open end (Range::above's min, FACING's max) -> end refused/flagged, next double inside accepted; both in Rust (schema value held to the reader) and Vitest. ~+10 lines.
 C2 (Medium) aim reach on bounds: bounds.min/max carry per axis the end the check holds (min: x,z >= -327.67, y >= 0; max: x,z <= 327.67, y <= 655.35; other end COORDINATE) via prefixItems; Rust test: schema values equal AIM_REACH_CM/100, 0, AIM_HEIGHT_MAX_CM/100; no next-double test there (F14). Grids, thickness, inside-bounds, min<max, spawn count stay descriptions. ~+35.
 C3 (Medium) Vitest limit test walks every number range of BOTH schemas (FACING's exclusiveMaximum is the only one: movement alone never exercises it), its small reader handles prefixItems. ~+15.
 C4 (Low, cheap) id: one $defs entry ($ref from the 5 id sites): pattern ^[a-z0-9][a-z0-9_-]*$, maxLength 64 (MAX_NAME_BYTES), not{pattern of con|prn|aux|nul|com[0-9]|lpt[0-9]}; plants in Vitest only (F12). If dropped, the description says the loader also refuses those. ~+15.
 C5 no new dependency of any kind (F12).
 C6 export test: generated/content allows exactly one folder, `schema`, real (DirEntry type, not followed); any other folder still fails; schema write mode deletes only .json directly in schema/, same link rules; tests: link named schema, folder inside schema, .json link, a non-.json file in schema.
 C7 booleans impossible by type (F10): build the schema as a Node tree; fixed-length arrays by minItems=maxItems; closed objects {"not":{}}; flattened shape union as oneOf, each branch closed and listing all its keys (additionalProperties does not see across allOf).
 Not step 11's: PM-0005-5 bundle files and the scan (row 2 / bundle.rs tests); no bundle schema.
 Estimate: conditions add ~75-90 counted lines -> step 11 ~800-900; look ahead before 12 moves ~+80.
B5 Q4 RULED: APPROVE counting in full (count rule as written; same reason as count-rule B4(2): no re-count near a trigger of his). Arithmetic: schema files ~340-390; 9,050 - 365 ~ 8,685 > 8,450, so the count rule does not decide STEP11-ROOM either way.
B6 STEP11-ROOM: Q1-Q3 hold wherever step 11 is built (4b-2 or 4d). Q4 binds 4b-2 only; in 4d its card sets its count at its start. If unanswered, build per ladder (rung 1 did not fire: 6,635 < 6,750); builder restates to Mustafa the look ahead with these conditions (~9,130 with step 11, ~8,300 without) before step 12 — the stop-and-ask stands; never shrink the count or evidence to fit.
Not checked: no build or test run; brief's unit-string measure not reproduced; schema line estimates are the builder's; protocol's ts_export not read.
DONE
