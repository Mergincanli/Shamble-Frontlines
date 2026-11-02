# engine-architect, ADR 0005's final round (frame, content data, the asset pipeline): fixes on 319d3b6

### FIX FB-11
FILE: docs/adr/0005-content-abilities-assets.md
BEFORE:
**Author:** engine-architect (frame, content data, the asset pipeline) · game-designer (the ability and effect system) · graphics-engineer (the asset pipeline's GPU rules) (the "Pass marks" table: test-engineer, plan step 4) · **Reviewed by:** (filled in at review) · **Signed off by Mustafa:** no
AFTER:
**Author:** engine-architect (frame, content data, the asset pipeline) · game-designer (the ability and effect system) · graphics-engineer (the asset pipeline's GPU rules) · test-engineer (the "Pass marks" table, plan step 4) · **Reviewed by:** (filled in at review) · **Signed off by Mustafa:** no
END

### FIX EA-6, ADR 0004 part 1 (routed: the pools)
FILE: docs/adr/0005-content-abilities-assets.md
BEFORE:
It defines the content every other draft reads (decision map, part d): typed definitions read by id (I-1; ADR 0004
executes them), content's schema and TypeScript types (I-5's content part, in ADR 0002's `packages/types`), the
content bundle and its hash (I-6; ADR 0006 serves it, ADR 0001 names it at join), the mode file (I-7; the room is ADR
AFTER:
It defines the content every other draft reads (decision map, part d): typed definitions read by id (I-1; ADR 0004
executes them and sizes its pools from the whole-set check's report of each pool's worst case, its part 1), content's
schema and TypeScript types (I-5's content part, in ADR 0002's `packages/types`), the content bundle and its hash (I-6;
ADR 0006 serves it and archives every content set a match record names, its part 12; ADR 0001 names it at join), the
mode file (I-7; the room is ADR
END

### FIX ADR 0004 part 1 (routed: the pools), RR-16 (routed: CD-3's half)
FILE: docs/adr/0005-content-abilities-assets.md
BEFORE:
1. **Format, loader and checks:** JSON with one canonical writer (CD-1); one content crate with a whole-set check,
   length bounds on every collection and no panics (CD-2); the schema and content's TypeScript types from the same
   Rust types (CD-3); versions and migrations (CD-4); one definition per file, one folder per kind (CD-5). *Need:*
   VISION_V2 §6.1, §6.3, §6.5, §6.7.
AFTER:
1. **Format, loader and checks:** JSON with one canonical writer (CD-1); one content crate with a whole-set check,
   length bounds on every collection, a report of each sim pool's worst case per mode, and no panics (CD-2); the
   schema and content's TypeScript types from the same Rust types, their generators kept out of the shipped builds
   (CD-3); versions and migrations (CD-4); one definition per file, one folder per kind (CD-5). *Need:* VISION_V2
   §6.1, §6.3, §6.5, §6.7; the pools' report: ADR 0004 parts 1 and 6.
END

### FIX ADR 0004 part 1 (routed: CD-2 reports each pool's worst case)
FILE: docs/adr/0005-content-abilities-assets.md
BEFORE:
    newtypes and hand-written checks replace it and schemars attributes carry the ranges (r1, lines 2801–2803).
  - Research: R1-DATA-REC2 (r1, lines 2500–2516); R1-RR-OQ2 (lines 2933–2935); the loader's side of R1-RR-OQ3
    (lines 2936–2940; the Worker's restart after a trap is ADR 0004's); R5-RUSTLINT-REC3 (r5, Recommendations, lines
    1340–1352, ADR 0002's).
  - Need: VISION_V2 §6.1, §6.3, §6.5, §6.7.
AFTER:
    newtypes and hand-written checks replace it and schemars attributes carry the ranges (r1, lines 2801–2803).
  - The pools' worst case (for ADR 0004 part 1, which gives every sim pool a fixed capacity at match start from the
    mode and the content set, so that the step never grows one and nothing a client sends sizes it): the whole-set
    check reports, for each mode in the set, the worst case of each pool (live projectiles, active effects, weapon
    pickups, big vehicles and their parts), from the content's numbers (fire intervals and projectile lifetimes,
    pellets per shot, effect durations and stack limits, drops per death and pickup lifetimes), the mode's team count
    and team size, and the game-designer's caps where they exist; the report travels with the frozen set. Where the
    content alone sets no bound, the report names the pool and the missing cap instead of a number, and nothing fills
    it with a default: "Caps on vehicles and live projectiles per team are not in VISION_V2 or LOOP_V2", and until the
    game-designer sets them every topic's stress marks use the stress scene's 15 big vehicles and 300 live projectiles
    (R2-BUD-OQ5, r2-netcode.md, Open questions, Brief question 2, lines 2982–2985). No research sizes a pool, so each
    pool's arithmetic is written beside its kind and reviewed like the bounds above (proposed); which row gives way
    when a pool is full is ADR 0004's OQ-0004-11.
  - Research: R1-DATA-REC2 (r1, lines 2500–2516); R1-RR-OQ2 (lines 2933–2935); the loader's side of R1-RR-OQ3
    (lines 2936–2940; the Worker's restart after a trap is ADR 0004's); R5-RUSTLINT-REC3 (r5, Recommendations, lines
    1340–1352, ADR 0002's); for the pools, R1-ABL-REC6's rule that the step "allocates nothing on the heap in steady
    state" (r1, Recommendations §5, lines 2471–2472; ADR 0004 part 6's) and R2-BUD-OQ5.
  - Need: VISION_V2 §6.1, §6.3, §6.5, §6.7; the pools' report: ADR 0004 parts 1 and 6.
END

### FIX RR-16 (routed from ADR 0002's batch: CD-3's half)
FILE: docs/adr/0005-content-abilities-assets.md
BEFORE:
  - What: schemars writes JSON Schema 2020-12 for every content type at build time, with ranges and descriptions from
    the garde rules and the doc comments. The editor's inspector and the Feel Lab (CD-14) build their fields from it:
    minimum and maximum become slider limits, the description becomes the tooltip, default and deprecated are shown.
    The authoritative verdict always comes from the Rust validator (CD-2); Ajv is optional, for instant per-field hints
    only. Content's TypeScript types come from ts-rs on the same Rust types, the generator R1-BND-REC4 already names
    for "content-schema types", not from the JSON Schema. Both outputs are committed in `packages/types`, regenerated
    by the gate and diffed (ADR 0002, R5-LAYOUT-REC3).
AFTER:
  - What: schemars writes JSON Schema 2020-12 for every content type when the gate regenerates the generated files
    (on `cargo test` or by a tool, never from a shipped build), with ranges and descriptions from the garde rules and
    the doc comments. The editor's inspector and the Feel Lab (CD-14) build their fields from it: minimum and maximum
    become slider limits, the description becomes the tooltip, default and deprecated are shown. The authoritative
    verdict always comes from the Rust validator (CD-2); Ajv is optional, for instant per-field hints only. Content's
    TypeScript types come from ts-rs on the same Rust types, the generator R1-BND-REC4 already names for
    "content-schema types", not from the JSON Schema. Both outputs are committed in `packages/types`, regenerated by
    the gate and diffed (ADR 0002, R5-LAYOUT-REC3). Neither generator's derives reach a shipped build (`server`,
    `sim-wasm`): schemars and ts-rs are dev-dependencies, their derives behind `cfg_attr(test, …)` or behind a feature
    that only dev-dependency entries turn on, as ADR 0002 part 5 sets ("Dev-dependencies do not activate features
    unless building a Cargo target that needs them": R5-LAYOUT-F4, r5 lines 83–91), so the `content` crate the shipped
    builds compare and measure is the shipped one (proposed by rust-reviewer at the step-10 review).
END

### FIX EA-6
FILE: docs/adr/0005-content-abilities-assets.md
BEFORE:
    are grouped by tuning version; the hash must resolve to an archived content set (the mark below), and where shipped
    bundles are kept is ADR 0006's storage (I-6). The bundle, like every build file, stays under the client host's
AFTER:
    are grouped by tuning version; the hash must resolve to an archived content set (the mark below): ADR 0006 part 12
    archives every content set a match record names, by its content hash, in the match records' bucket, uploaded by
    the supervisor beside the first record that names it and kept as long as any record does (proposed there; I-6).
    No research finding stores the sets: R6-TEL-REC2 stores the records (r6-infrastructure.md, Recommendations §4,
    lines 812–827), while R1-DATA-REC6's mark asks that every record's hash resolve to an archived set (r1, lines
    2576–2579). The bundle, like every build file, stays under the client host's
END

### FIX ADR 0004 part 1 (routed: the pools' mark)
FILE: docs/adr/0005-content-abilities-assets.md
BEFORE:
   default depth limit (R1-DATA-OQ4) and runs the same tests with garde on wasm32 (R1-DATA-OQ5).
AFTER:
   default depth limit (R1-DATA-OQ4) and runs the same tests with garde on wasm32 (R1-DATA-OQ5). Proposed (reason:
   ADR 0004 part 1 sizes its pools from this report): for each test mode, every pool's reported worst case equals a
   count by hand from the test content's numbers, and every pool the content leaves without a bound is named as such.
END

### FIX AD-6 (routed by graphics-engineer: AP-3's example list)
FILE: docs/adr/0005-content-abilities-assets.md
BEFORE:
  - What: the classes the GPU section's part 2 names (for example environment, unit, faction colour, emissive), and
AFTER:
  - What: the classes the GPU section's part 2 names (for example environment, structure, unit, faction colour,
    emissive), and
END

### FIX AD-6 (routed by graphics-engineer: AP-3's need)
FILE: docs/adr/0005-content-abilities-assets.md
BEFORE:
  - Need: VISION_V2 §6.7; LOOK_V2 §2 (uniform details), §6.1 (the value hierarchy by material class), §6.4 (the glow
    language's emissive slots).
AFTER:
  - Need: VISION_V2 §6.7; LOOK_V2 §2 (uniform details; the ink "especially strong on buildings", which the structure
    class carries), §6.1 (the value hierarchy by material class), §6.4 (the glow language's emissive slots).
END

### FIX PA-11
FILE: docs/adr/0005-content-abilities-assets.md
BEFORE:
    per pass, 200 draw calls ÷ 4 units ÷ 2 passes, r3, lines 1088–1090), the rest once OQ-0005-AS5 sets them. The gate compares each
AFTER:
    per pass, 200 draw calls ÷ 4 units ÷ 2 passes, r3, lines 1088–1090), the rest once OQ-0005-AS5 sets them; the GPU
    section's triangle ceiling (about 5.2k triangles per soldier as drawn at Low, its part 4) is a planning input to
    OQ-0005-AS5 and AS6, not a budget the gate checks. The gate compares each
END

### FIX IE-12, UNRELEASED-ART (routed by infra-engineer: OQ-0005-AP1 and the store's visibility)
FILE: docs/adr/0005-content-abilities-assets.md
BEFORE:
- **OQ-0005-AP1. Where does the automated step run, what does it write, and where do the files live?**
  - Why it matters: the asset list (AP-2) and the budget check (AP-5) need a producer they can trust, and source and
    processed files need a home.
  - Options (not researched): (a) a tool the artist runs when importing a model, which commits the processed file and
    its list entry; (b) a CI job that processes committed exports on every change (ADR 0002 would place it); (c) a
    build step that processes exports at build time and commits nothing processed. With each: whether the gate recounts
    a file or trusts its entry; where Spline source files and raw exports are kept.
  - Decider: the asset-pipeline research, then the asset spike. What decides it: what the research finds Spline's
    export and the tools for step 4 can do. Research ids: none.
AFTER:
- **OQ-0005-AP1. Where does the automated step run, what does it write, and where do the files live?**
  - Why it matters: the asset list (AP-2) and the budget check (AP-5) need a producer they can trust, and source and
    processed files need a home. The repository is public (r5-toolchain-quality.md, Findings, Question 6, legacy
    facts, lines 913–914), so a model file committed to it can be read by anyone once it is pushed (ADR 0006 part 2):
    where the files live also decides who can see unreleased art.
  - Options (not researched): (a) a tool the artist runs when importing a model, which commits the processed file and
    its list entry; (b) a CI job that processes committed exports on every change (ADR 0002 would place it); (c) a
    build step that processes exports at build time and commits nothing processed. With each: whether the gate recounts
    a file or trusts its entry; where Spline source files and raw exports are kept; and whether the files sit in the
    public repository or in a private store the build reads with a secret (ADR 0006's OQ-0006-18 (b), not researched).
  - Decider: the asset-pipeline research, then the asset spike; the files' visibility follows Mustafa's answer to ADR
    0006's OQ-0006-18 (must unreleased art stay unseen?), due before the first authored model is committed, which may
    be one of the look proof test's models (the GPU section's part 8). What decides it: what the research finds
    Spline's export and the tools for step 4 can do, and that answer. Research ids: none.
END

### FIX FB-13
FILE: docs/adr/0005-content-abilities-assets.md
BEFORE:
  Wasm size is recorded, not gated, until the spikes have measured it (ADR 0002 part 4; OQ-0005-CD2). **B.** Pros:
AFTER:
  Wasm size is recorded, not gated, until the Advisor sets the budget's first limit from the spikes' measurements
  (ADR 0002 part 4; OQ-0005-CD2). **B.** Pros:
END

### FIX GE-1 (the frame follows graphics-engineer's applied fix: Consequences, Easier)
FILE: docs/adr/0005-content-abilities-assets.md
BEFORE:
  slot class, so batching survives authored art (the GPU section's parts 1 and 7).
AFTER:
  slot content (its class, its faction for faction slots, and its own colours and textures), never one per mesh, so
  batching survives authored art (the GPU section's parts 1 and 7).
END

### FIX GE-1 (the frame follows: Consequences, Harder), ADR 0004 part 1 (routed: the pools)
FILE: docs/adr/0005-content-abilities-assets.md
BEFORE:
  reviewed length bound for every collection (CD-2); JSON without comments (OQ-0005-CD1); a simulation number tested in
  a new match, not the current one (CD-7); two generators that must agree (CD-3); an asset list kept in step with the
  files (AP-2); every material slot named into a class (OQ-0005-AS4); two drawing paths for buildings and props until
AFTER:
  reviewed length bound for every collection, and each sim pool's worst-case arithmetic beside its kind (CD-2); JSON
  without comments (OQ-0005-CD1); a simulation number tested in a new match, not the current one (CD-7); two
  generators that must agree (CD-3); an asset list kept in step with the files (AP-2); every material slot named into
  a class, and every distinct slot content one more material (OQ-0005-AS4; the GPU section's part 1); two drawing
  paths for buildings and props until
END

### FIX FB-7
FILE: docs/adr/0005-content-abilities-assets.md
BEFORE:
- **Revisit when:** content's load time misses its budget, or the Wasm file the limit ADR 0002 sets (OQ-0005-CD2, option
AFTER:
- **Revisit when:** content's load time misses its budget, or content's share takes the Wasm file past the limit ADR
  0002 sets (OQ-0005-CD2, option
END

### FIX FC-8
FILE: docs/adr/0005-content-abilities-assets.md
BEFORE:
  | GW6, GW8 (194, 196) | redesign (pending) | unchanged: the registries and the placement glue split into the map document (CD-11), the sim's colliders, never read from a model (AP-1), and a per-entity scene on the page (EDITOR_V2 §2 rules 1, 3 and 4); pending becomes "OQ-0005-AP2" for the crates, walls, roads, houses, covers and props, and "the tree style, LOOK_V2 §8 `[OPEN]`" for the trees | LOOK_V2 §8 allows authored models without requiring them and names none of these pieces; the tree style is Mustafa's open item |
AFTER:
  | GW6, GW8 (194, 196) | redesign (pending) | unchanged: the registries and the placement glue split into the map document (CD-11), the sim's colliders, never read from a model (AP-1), and a per-entity scene on the page (EDITOR_V2 §2 rules 1, 3 and 4); pending becomes "OQ-0005-AP2" for the crates, walls, roads, houses, covers and props, and "the tree style, LOOK_V2 §8 `[OPEN]`" for the trees | LOOK_V2 §8 allows authored models without requiring them; §2 keeps the props ("logs and trees with variety, as a base to build on") and the two houses as `[DECIDED]` directions (LOOK_V2 lines 17, 21–22), and §4 asks for buildings you can enter, but no section says whether any of these pieces is authored or procedural; the tree style is Mustafa's open item |
END

### FIX IE-12, UNRELEASED-ART (routed: the frame's list row for OQ-0005-AP1)
FILE: docs/adr/0005-content-abilities-assets.md
BEFORE:
| OQ-0005-AP1 | Where does the automated step run, what does it write, and where do the files live? | the asset-pipeline research, then the asset spike | The asset pipeline |
AFTER:
| OQ-0005-AP1 | Where does the automated step run, what does it write, and where do the files live? | the asset-pipeline research, then the asset spike; the files' visibility follows Mustafa's answer to ADR 0006's OQ-0006-18 | The asset pipeline |
END

### FIX GE-1 (the frame follows: the joined spike's item 2)
FILE: docs/adr/0005-content-abilities-assets.md
BEFORE:
   (slot class, faction) pairs; the two derived caps) and the asset pipeline section's first bullet (every model listed
AFTER:
   the distinct slot contents shown; the two derived caps) and the asset pipeline section's first bullet (every model listed
END

### FIX RR-16 (routed: Appendix A's count)
FILE: docs/adr/0005-content-abilities-assets.md
BEFORE:
Counts: adopted 6 · changed 5 · answered 8 · open 9 · spike 6 · rejected 0 (34 rows).
AFTER:
Counts: adopted 5 · changed 6 · answered 8 · open 9 · spike 6 · rejected 0 (34 rows).
END

### FIX ADR 0004 part 1 (routed: the pools), RR-16 (routed), EA-6 (Appendix A rows)
FILE: docs/adr/0005-content-abilities-assets.md
BEFORE:
| R1-DATA-REC2 | changed: adds a maximum length on every collection (R1-RR-OQ2) and the no-panic loader (R5-RUSTLINT-REC3, R1-RR-OQ3), and refuses a fallback model too; the stages as written | CD-2; spike mark 2 |
| R1-DATA-REC3 | adopted | CD-3 (content's TypeScript types come from ts-rs beside it, answering critic rank 31); spike mark 3 |
AFTER:
| R1-DATA-REC2 | changed: adds a maximum length on every collection (R1-RR-OQ2) and the no-panic loader (R5-RUSTLINT-REC3, R1-RR-OQ3), refuses a fallback model too, and reports each sim pool's worst case per mode for ADR 0004 part 1; the stages as written | CD-2; spike mark 2 |
| R1-DATA-REC3 | changed: the schema is written when the gate regenerates (on `cargo test` or by a tool), not by the shipped build, and the generators' derives stay out of the shipped builds (ADR 0002 part 5; the rust-reviewer's step-10 review); content's TypeScript types come from ts-rs beside it, answering critic rank 31 | CD-3; spike mark 3 |
END

### FIX EA-6 (Appendix A row)
FILE: docs/adr/0005-content-abilities-assets.md
BEFORE:
| R1-DATA-REC6 | adopted | CD-8; spike mark 6 |
AFTER:
| R1-DATA-REC6 | adopted | CD-8 (the archive its mark needs is ADR 0006 part 12's); spike mark 6 |
END

### NOTE EA-6
fixed in CD-8 (it now points to ADR 0006 part 12's archive, as applied in 8fa7173: every content set a match record
names, by its content hash, in the match records' bucket, kept as long as any record does), in "Where this record
sits" (I-6) and in Appendix A's R1-DATA-REC6 row. routed: the synthesis (engine-architect's own later batch):
`docs/adr/README.md`'s interface I-6 row adds "the archive of shipped content sets, by hash (0006 part 12)", as
infra-engineer's NOTE EA-6 also asks.
END

### NOTE PA-11
fixed by graphics-engineer in the GPU section's part 4 (the triangle ceiling, applied in 319d3b6 at 0005:1583–1588:
500,000 ÷ (48 × 2) = 5,208.3, about 5.2k, on R3-BUDGET-REC2's Low triangle row, r3 line 1134). This batch's FIX PA-11
makes AP-5 point to it as a planning input to OQ-0005-AS5 and AS6, so the content side does not read it as a budget
the gate checks. Nothing routed.
END

### NOTE ADR 0004 part 1 (routed by the session from rust-reviewer's batch, b815c85: the pools)
fixed in CD-2 (a new bullet: the whole-set check reports each pool's worst case per mode; a pool the content leaves
unbounded is named with its missing cap, never filled with a default), in the frame's Decision part 1, "Where this
record sits" (I-1), the content spike's item 2, Consequences (Harder) and Appendix A's R1-DATA-REC2 row. routed:
test-engineer's CD-2 rows (PM-0005-4 or PM-0005-5) gain the leg item 2 now names (for each test mode, every pool's
reported worst case equals a count by hand from the test content's numbers, and every pool without a bound is named;
a bite: a projectile kind whose lifetime is doubled while the report stays the same). rust-reviewer (optional, ADR
0004 part 1): a pool the report leaves unbounded (big vehicles, until R2-BUD-OQ5's caps exist) takes the stress
scene's count as a spike fixture, not a rule, as 0004's "the caps on vehicles and projectiles that replace the stress
scene's counts (R2-BUD-OQ5)" already implies.
END

### NOTE RR-16 (routed by the session from engine-architect's ADR 0002 batch: CD-3's half)
fixed in CD-3, the frame's Decision part 1 and Appendix A (R1-DATA-REC3 becomes "changed": the schema is written when
the gate regenerates, not by the shipped build; the counts become adopted 5, changed 6, still 34 rows). The text
matches FIX RR-16 in engine-architect-0002-fixes-final.md (ADR 0002 part 5: "content's schema generator follows the
same rule (ADR 0005, CD-3)"); if that block is not applied, CD-3's "as ADR 0002 part 5 sets" must go. test-engineer:
PM-0005-8 stands; PM-0005-16's Wasm share is then measured on the shipped shape, with no generator derives.
END

### NOTE IE-12, UNRELEASED-ART (routed by infra-engineer)
fixed in OQ-0005-AP1 (why it matters: the repository is public; options: the files in the public repository or in a
private store the build reads with a secret; decider: the files' visibility follows Mustafa's OQ-0006-18, due before
the first authored model is committed) and in the frame's list row. No new open question: the decision is Mustafa's
OQ-0006-18 in ADR 0006, and OQ-0005-AP1 points to it. routed: the synthesis's README row for OQ-0005-AP1 names
OQ-0006-18 beside its decider.
END

### NOTE GE-1 (the frame follows graphics-engineer's applied fix)
graphics-engineer's GE-1 blocks (applied in 319d3b6) made the GPU section's parts 1 and 7 "one material per slot
content", but the frame still read "one material per slot class" (Consequences, Easier) and "(slot class, faction)
pairs" (the joined spike's item 2). The three FIX GE-1 blocks make the frame say the same, and Harder gains the cost
the GPU section's options name (every distinct slot content is one more material). Nothing routed: PM-0005-34 and
PM-0005-35 are already graphics-engineer's route to test-engineer.
END
