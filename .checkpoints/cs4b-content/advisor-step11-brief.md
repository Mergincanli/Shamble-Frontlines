# advisor agent: four rulings for 4b-2 step 11 (the schema writer), before any of it is built

Asked by Builder V6 (the main session), 2026-11-27. The worktree is `/home/user/Shamble-Frontlines/.gate/cs4b-2`
(local branch `cs/4b-content`, pushed as `claude/ecstatic-wozniak-5cbdzs`). Step 10 is committed (885d382); the
official count at step 10's end is 6,635 (raw 6,653), under the first rung (6,750), so by the approved ladder step 11
is built in 4b-2. The row: `.checkpoints/cs4b-content/reread-sheet.md`, row 11 ("the schema writer, alone (110 /
130) | `schema.rs`, the field tables' `description` and `default` columns, the export test's schema output into
`packages/types/src/generated/content/schema/` (a link named `schema` and a `.json` link refused) | nothing |
PM-0005-7's schema half; PM-0005-8's schema half"). Its design: engine-architect's block 1.10
(`.checkpoints/cs4-walk/engine-architect-cs4.md:261-267`). The card's rule: "A new threshold or order goes back to
the advisor agent first." Each question gives the approved words, the facts and the builder's proposal; rule
APPROVE, REVISE (with what) or ESCALATE (to Mustafa, with why), each on its own.

## Q1. The `default` column
- **Approved words.** Row 11: "the field tables' `description` and `default` columns". Block 1.10: "The field tables
  drive the reader's ranges, the schema's `minimum`/`maximum` (or exclusive forms), `maxItems`, `description` and
  `default`". ADR 0005 CD-4: "additive via default/alias".
- **Facts.** The reader takes no default: "a content file holds exactly its kind's fields: nothing missing, nothing
  extra, no fallback" (`crates/content/src/read.rs:1-3`); the test list's rules: "No fallback: a refused bundle starts
  no match and draws no map; there is no built-in default map or table" (`test-list.md`); the card's rule: no value
  of his is typed by hand (the movement values are his Export's). No field of version 1 of either kind is optional,
  so no field has a default to give.
- **Proposal.** No `default` column now: the column joins with the first field that is added to a kind with a
  default (CD-4's additive change), and the schema writer writes `default` from it then. The report says the column
  was left out and why.

## Q2. Where the descriptions live
- **Approved words.** Row 11: "the field tables' `description` ... columns". ADR 0005 CD-3 (`docs/adr/0005*.md`,
  :336-349): "with ranges and descriptions from the garde rules and the doc comments ... Neither generator's derives
  reach a shipped build (`server`, `sim-wasm`) ... so the `content` crate the shipped builds compare and measure is the
  shipped one". Block 1.10: "Neither reaches a shipped build (CD-3 :344-348)".
- **Facts.** The field tables are shipped: `movement::FIELDS` (`crates/content/src/movement.rs:18-34`, 15 fields of
  name, unit and range) and `map::FACING`, read at run time by the reader, which the players' file links since cut C.
  MEASURED (to be filled in by the builder before this brief is sent: one description added to one field, the
  players' file built, its bytes searched for the text).
- **Proposal.** The descriptions live in a test-only table beside the schema writer (`schema.rs`, `#![cfg(test)]`, as
  `ts_export.rs` is), keyed by each field's name, with a test that every field of `FIELDS` and every key of the map
  has exactly one description and no other; so no description reaches the players' file, as CD-3 says. The row's
  "columns" then reads as "a description per field of the table, kept where no shipped build has it".

## Q3. What the schema covers, and its booleans
- **Approved words.** Block 1.10's `maxItems` (only the map has lists); `test-list.md:110-111`: "The export test for
  the generated types copies `protocol`'s write rules and refuses a link named `schema` or a `.json` link (S-should 18).
  PM-0005-5's scan includes `regions` and the bundle's `files` (S-should 15)." PM-0005-7 (ADR 0005's marks): "schema
  range = validator range, just-outside refused + flagged by a Vitest schema-driven limit test, at-bound accepted";
  PM-0005-8: "regenerate 0 bytes, tsc strict, rename without regen fails, hand edit fails". The sheet's split: 4b-1's
  row 2, the reader and the writer (`reread-sheet.md:156`), holds "PM-0005-7's Rust half (ranges at the next float up
  and down)", row 11 "PM-0005-7's schema half".
- **Facts.** The content grammar has no booleans: `true` and `false` are refused by name (`crates/content/src/json.rs:5-6`,
  `:185-189`, security-auditor's P15), and the canonical writer writes only objects, arrays, strings and numbers. A
  closed object in JSON Schema 2020-12 is `"additionalProperties": false`; the same meaning without a boolean is
  `"additionalProperties": {"not": {}}` (a schema no value matches). The export test today refuses any folder inside
  `packages/types/src/generated/content` (`crates/content/src/ts_export.rs`, `files_in`: "Anything else in it (a
  folder, a link) fails the test"). FACING is the one range open at its top (`map.rs:64`, `max_inclusive: false`);
  movement has `Range::above` (open at the bottom) and `Range::closed`.
- **Proposal.** Both kinds: `movement.schema.json` from `FIELDS` (each number's `minimum` or `exclusiveMinimum` and
  `maximum` or `exclusiveMaximum` from its `Range`, its description; `format` and `version` as `const`, `id` with the
  id grammar's `pattern`, every key `required`) and `map.schema.json` from map.rs's constants (`maxItems` from
  `MAX_SPAWNS`, `MAX_SHAPES`, `MAX_REGIONS`; every coordinate inside `COORDINATE`; `facing_deg` from `FACING`; the
  fixed words of `FORMS`, `TOWARDS`, `GROUNDS` as `enum`; the grids and the aim's reach said in descriptions, since
  `multipleOf` on decimals is not exact), both written by the canonical writer, so their text is canonical; closed
  objects as `{"not": {}}`, so the shipped grammar is not widened; `$schema` the 2020-12 address. The export test
  checks the `schema` folder as its own plain folder of plain `.json` files (a link named `schema`, a folder inside it
  and a `.json` link refused, each with a test) and writes it with `SHAMBLE_WRITE_TYPES=1` like the types. Tests:
  every range of both schemas equals its reader's constant, all four end forms covered (closed, open below, open
  above as FACING, and the list caps); the loader refuses the next double outside each end and takes the end itself
  (the Rust half already proves the reader's side; here each schema value is held to it); a hand-edited schema byte
  and a renamed field without a new export each fail the export test; a Vitest test in `packages/types` reads the
  committed movement schema and flags each just-outside value and passes each end (PM-0005-7's "flagged by a Vitest
  schema-driven limit test"), with its own small reader of the schema's four range keywords inside the test, and no
  product code (the camera panel, the first product reader, is a later change-set's).

## Q4. The count of the generated schema files
- **Approved words.** The official count (the advisor agent's ruling of 2026-11-27, `advisor-count-rule.md`): every
  file under `packages/types/src/generated/` counts.
- **Facts.** In canonical form (one item a line) the movement schema is about 140 lines and the map schema about 200
  to 250 (estimates, before writing); with `schema.rs` and the tests, step 11 is then about 700 to 800 counted lines
  against the look ahead's 520: the look ahead before step 11 becomes about 6,635 + 800 + 350 = 7,785, under 8,450.
- **Asked.** Whether the generated schema files count in full, as the ruling's words say (the builder applies the
  ruling as written unless you change it).

## What comes back
Per question: APPROVE, REVISE (what to change) or ESCALATE (with why), a paragraph each; anything the proposals get
wrong against the approved text. Read only what you need; edit no file of the worktree but your notes.
