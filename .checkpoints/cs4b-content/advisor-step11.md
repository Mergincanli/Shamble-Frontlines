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
