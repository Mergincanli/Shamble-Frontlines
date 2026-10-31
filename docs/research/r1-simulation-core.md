# R1: Simulation core

- **Topic:** R1, the simulation core (`docs/V2_PHASE0_BRIEF.md` §4).
- **Author agent:** engine-architect. Units (wave 1 of `.checkpoints/v2-research/plan.md`, one engine-architect agent
  each): `ecs`, `tick`, `determinism`, `boundary-worker`, `abilities`, `data-hotreload`. Assembly: engine-architect
  (wave 1, step 2 "Assemble": nothing new is claimed; where units disagree, both findings stay and the conflict is an
  open question).
- **Date:** 2026-10-27 (every source the units cite was read that day); fix step 2026-10-28 (the findings added or
  re-read then carry that date); second verifier pass and second fix round the same day.
- **Status:** verified (every recommendation claim). Fix step 2026-10-28 (engine-architect, wave 1 step 4): the
  verifier's six reports and rust-reviewer's R1 review are applied; part 7 holds their merged table. Second verifier
  pass and second fix round, 2026-10-28: its 2 wrong claims and 1 unsupported claim are fixed, none re-sourced (part 7,
  "Second verifier pass (2026-10-28)"); the final check of that round found nits only (part 7, "Final check").
- Glossary and cross-topic resolutions applied 2026-10-30 (see [GLOSSARY.md](GLOSSARY.md) and part 7).
- **The brief's questions for this topic (quoted, `docs/V2_PHASE0_BRIEF.md` §4, row R1):** "ECS or plain data
  (bevy_ecs, hecs, custom); a fixed tick rate; **identical results between Wasm and native** (floats, Rapier's
  determinism options); the Rust ↔ TypeScript boundary (shared memory, wasm-bindgen, generated TypeScript types);
  running the simulation in a Web Worker; a **data-driven ability and effect system** in the spirit of UE's Gameplay
  Ability System; data formats and hot reload of tuning"

**How this file is laid out.** Each part of the shape (`docs/research/README.md`) has one section per brief question,
in the brief's order. The boundary and the Web Worker were researched by one unit, so they share section 4.

| # | Brief question | Unit | Ids |
|---|---|---|---|
| 1 | ECS or plain data (bevy_ecs, hecs, custom) | `ecs` | R1-ECS-F1–F35, REC1–REC4, OQ1–OQ7 |
| 2 | a fixed tick rate | `tick` | R1-TICK-F1–F26, REC1–REC6, OQ1–OQ9 |
| 3 | identical results between Wasm and native | `determinism` | R1-DET-F1–F34, REC1–REC7, OQ1–OQ11 |
| 4 | the Rust ↔ TypeScript boundary; running the simulation in a Web Worker | `boundary-worker` | R1-BND-F1–F35, REC1–REC6, OQ1–OQ9 |
| 5 | a data-driven ability and effect system | `abilities` | R1-ABL-F1–F26, REC1–REC6, OQ1–OQ10 |
| 6 | data formats and hot reload of tuning | `data-hotreload` | R1-DATA-F1–F29, REC1–REC8, OQ1–OQ12 |
| – | conflicts and gaps between the units (added at assembly) | – | R1-XU-OQ1–OQ12 |
| – | Rust risks the rust-reviewer found (added at the fix step) | – | R1-RR-OQ1–OQ7 |
| – | shared decision questions (added 2026-10-30) | – | ABL-PREDICT, SEC-SPREAD, MODES-3, BASELINE-WHEN |
| – | cross-topic open questions (consistency critic, added 2026-10-30) | – | R1-XT-OQ1–OQ18 |

Totals: 185 findings (5 added at the fix step: R1-ECS-F35, R1-DET-F33, F34, R1-BND-F34, F35), 37 recommendations, 99
open questions (55 from the units, 12 added at assembly, 9 at the fix step: R1-BND-OQ9, R1-DATA-OQ12, R1-RR-OQ1–OQ7, 1
at the second fix round: R1-DET-OQ11, 22 at the consistency step of 2026-10-30: 4 shared decision questions and
R1-XT-OQ1–OQ18; 3 settled at the fix step: R1-ECS-OQ2, R1-XU-OQ5, R1-XU-OQ6, 1 at the consistency step: R1-XU-OQ1),
161 sources read by the units (170 reads across the units; 9 were the same page read by a second or third unit) plus
10 reads at the fix step (3 new pages), 48 sources not read.

**Reading rules (taken from the unit drafts).**
- Sureness: "confirmed in the source" means the quoted words came back verbatim from the page (the fetch tool caps a
  verbatim quote at about 125 characters, so long passages are quoted as consecutive fragments); "inferred" means the
  unit's own reasoning or arithmetic on a source, or a point seen only in the fetch tool's summary; "unverified" means
  not checked. Each unit states its own wording of these labels in its section of the findings.
- Tiers, as each unit labelled its sources: official (official docs and specifications), shipped game, other. One
  source carried two labels; the fix step settled it, and one more Fiedler article, as other (R1-XU-OQ6).
- Ids: F = finding, REC = recommendation, OQ = open question, each prefixed by its unit. A short id inside a unit's
  text ("F12", "REC2", "OQ3") is that unit's. A section reference inside a unit's text ("§3", "section 7") points into
  that unit's draft; the same material sits in this file in that unit's section of the matching part.
- Every pass mark and every reference machine is **proposed**; targets come only from VISION_V2 §4 and §6.
- Legacy facts cite `docs/LEGACY_INVENTORY.md` by row id and `docs/legacy-tuning.json` by table. The inventory's marks
  are working recommendations (the Advisor's R7-MARKS (a)); rows marked "look brief pending" stay pending. Nothing here
  decides the look; no engine code and no spike was written.
- Each unit's working notes (every source summary, with its verbatim quotes) and its draft are in
  `.checkpoints/v2-research/engine-architect-r1-<unit>.md` and `…-<unit>-draft.md` while this change-set is open.

## TL;DR
- Found: neither ECS crate orders its queries or clones its world; std trig, NaN bits and relaxed SIMD differ by
  platform and `HashMap` order by instance; Rapier matches across machines only with `enhanced-determinism`.
- Recommended (marks proposed): one plain-data sim struct, sim-owned ids; one integer tick, 60 Hz spiked beside 30 Hz;
  `libm`, ChaCha8, per-tick state hashes, no lockstep; one Worker handing the page one transferred block per tick; a
  reduced GAS subset as data; JSON through one Rust content crate, hot reload in development only; R1-XU-OQ1–OQ12 open.

## Findings

### 1. ECS or plain data (bevy_ecs, hecs, custom) (unit `ecs`)

Brief question (quoted): "ECS or plain data (bevy_ecs, hecs, custom)" (docs/V2_PHASE0_BRIEF.md §4, row R1).

Scope: how the
Rust simulation core holds its state (native on the server, Wasm in a Web Worker in the browser): entity kinds and
counts, deterministic iteration order, whole-state snapshot and restore, serialization for snapshots, Wasm size and
compile time, and how each option would hold the data-driven ability system and physics. Notes:
`.checkpoints/v2-research/engine-architect-r1-ecs.md` (every source summary, with the verbatim quotes).

Sureness labels: "confirmed in the source" = the quoted sentence was returned verbatim from the page; "inferred" = my
reasoning from the source, or a point seen only in a fetch summary; "unverified" = not checked.

#### bevy_ecs (0.20.0)
- **R1-ECS-F1.** bevy_ecs 0.20.0 is the current release, and Bevy has shipped five releases from 0.16 to 0.20 in
  under eighteen months: 0.16 on 2025-04-24, 0.17 on 2025-09-30, 0.18 on 2026-01-13, 0.19 on 2026-06-19, 0.20 on
  2026-10-08 (the news-post folder names). That every 0.x release breaks the API is inferred, not read.
  Source: https://github.com/bevyengine/bevy-website/tree/main/content/news (folder list); version from
  https://docs.rs/bevy_ecs/latest/bevy_ecs/ (header "0.20.0"). Tier: official (the project's own site repository and
  docs). Read 2026-10-27. Sure: confirmed in the source (dates and version); the API-churn reading is inferred.
- **R1-ECS-F2.** bevy_ecs can be used without the rest of Bevy. Quote: "can easily be used as a standalone crate in
  other projects". Source: https://docs.rs/bevy_ecs/latest/bevy_ecs/ (crate docs), 0.20.0. Tier: official. Read
  2026-10-27. Sure: confirmed in the source.
- **R1-ECS-F3.** bevy_ecs stores components in tables by default and offers sparse sets as an opt-in, trading
  iteration speed against add and remove cost. Quotes: tables "Fast and cache friendly iteration, but slower adding
  and removing of components"; sparse sets "Fast adding and removing of components, but slower iteration". Source:
  https://docs.rs/bevy_ecs/latest/bevy_ecs/ (component storage), 0.20.0. Tier: official. Read 2026-10-27. Sure:
  confirmed in the source.
- **R1-ECS-F4.** A bevy_ecs query does not guarantee its iteration order. Quotes: iter: "Iteration order is not
  guaranteed."; par_iter: "Iteration order and thread assignment is not guaranteed."; iter_many: "Items are returned
  in the order of the list of entities." Source: https://docs.rs/bevy_ecs/latest/bevy_ecs/system/struct.Query.html
  (iter, par_iter, iter_many), 0.20.0. Tier: official. Read 2026-10-27. Sure: confirmed in the source.
- **R1-ECS-F5.** bevy_ecs can sort query results, but the sort is redone on every run. Quotes: sort: "Sorts all query
  items into a new iterator, using the query lens as a key."; "The sort is not cached across system runs." Source:
  https://docs.rs/bevy_ecs/latest/bevy_ecs/query/struct.QueryIter.html (sort, sort_by, sort_by_key), 0.20.0. Tier:
  official. Read 2026-10-27. Sure: confirmed in the source. The sort's allocation cost is not stated (unverified).
- **R1-ECS-F6.** A bevy_ecs schedule ignores system-order ambiguities by default. Quote: ambiguity_detection
  "Determines whether the presence of ambiguities (systems with conflicting access but indeterminate order) is only
  logged", "Defaults to LogLevel::Ignore."; a debug-only shuffle_seed exists: "If Some, systems will be shuffled
  according to the given seed." Source:
  https://docs.rs/bevy_ecs/latest/bevy_ecs/schedule/struct.ScheduleBuildSettings.html, 0.20.0. Tier: official. Read
  2026-10-27. Sure: confirmed in the source.
- **R1-ECS-F7.** bevy_ecs has a single-threaded executor, and its multi-threaded feature is off by default. Quote:
  SingleThreadedExecutor "Runs the schedule using a single thread." The feature list shows the defaults
  async_executor, backtrace, bevy_reflect and std, with multi_threaded and serialize off by default. Sources:
  https://docs.rs/bevy_ecs/latest/bevy_ecs/schedule/struct.SingleThreadedExecutor.html and
  https://docs.rs/crate/bevy_ecs/latest/features, 0.20.0. Tier: official. Read 2026-10-27. Sure: confirmed in the
  source (the feature list is a docs.rs table, read as data).
- **R1-ECS-F8.** A bevy_ecs `World` does not implement `Clone`: its trait list has Debug, Default, Send, Sync and the
  SystemParam impls, and no Clone. Source: https://docs.rs/bevy_ecs/latest/bevy_ecs/world/struct.World.html (Trait
  Implementations), 0.20.0. Tier: official. Read 2026-10-27. Sure: confirmed in the source (the impl list).
- **R1-ECS-F9.** A bevy `Entity` id is opaque and local to one World, so it can't be the shared id between server and
  client. Quotes: "For all intents and purposes, Entity should be treated as an opaque identifier."; "one should not
  try to synchronize the ECS by attempting to ensure that Entity lines up between instances"; to_bits: "Do not use for
  serialization between runs."; "directly serializing with Serialize and Deserialize make zero guarantee of long term
  wire format compatibility." Source: https://docs.rs/bevy_ecs/latest/bevy_ecs/entity/struct.Entity.html
  (struct docs, from_index, to_bits), 0.20.0, page dated 2026-10-08. Tier: official. Read 2026-10-27. Sure: confirmed
  in the source.
- **R1-ECS-F10.** Bevy's rollback library, bevy_ggrs, does not clone the World. It snapshots only the types it is told
  about: per frame, a map from a stable rollback id to each value, and on load it spawns or despawns entities to
  match. Quotes: it snapshots "only the components and resources you register" (README); a component snapshot is "a
  HashMap<RollbackId, As> mapping each rollback entity to its stored value at that frame"; on load, an entity only in
  the snapshot: "spawns a fresh entity with the same Rollback + RollbackId"; an entity only in the world: "despawns it
  (it didn't exist at the rollback target frame)." Sources: https://github.com/gschup/bevy_ggrs (README; compatibility
  table: bevy 0.19 with bevy_ggrs 0.22, bevy 0.20 on main only) and
  https://raw.githubusercontent.com/gschup/bevy_ggrs/main/docs/architecture.md (main branch, no date). Tier: other (a
  third-party library, not a shipped game). Read 2026-10-27. Sure: confirmed in the source.
- **R1-ECS-F11.** bevy_ggrs ranks its own snapshot strategies by cost but gives no benchmarks. Quotes (table notes):
  Copy "Cheapest — bitwise copy"; Clone "Heap-allocating clone each frame"; Reflect "Uses dynamic reflection;
  slowest". Source: https://raw.githubusercontent.com/gschup/bevy_ggrs/main/docs/architecture.md. Tier: other. Read
  2026-10-27. Sure: confirmed in the source. That there are no benchmarks was seen in the fetch summary (inferred).
- **R1-ECS-F12.** In bevy, deterministic re-simulation needs an explicit sort, because query order isn't stable.
  Quote: "Bevy does not guarantee a stable iteration order for queries." The page advises sorting by a stable key or
  by RollbackOrdered (paraphrase). Source: https://raw.githubusercontent.com/gschup/bevy_ggrs/main/docs/pitfalls.md.
  Tier: other. Read 2026-10-27. Sure: confirmed in the source (the quote); the advice is inferred.
- **R1-ECS-F13.** In bevy, state that isn't registered for snapshots is silently not restored. Quotes: "Any state
  stored elsewhere will not be restored on rollback, causing silent desyncs."; "Local<T> is per-system state that is
  not snapshotted". Source: https://raw.githubusercontent.com/gschup/bevy_ggrs/main/docs/pitfalls.md. Tier: other.
  Read 2026-10-27. Sure: confirmed in the source.
- **R1-ECS-F14.** In bevy, a restore marks the restored components as changed, so change-detection systems fire after
  every rollback. Quote: "Systems that react to Changed<T> will fire after every rollback." Source:
  https://raw.githubusercontent.com/gschup/bevy_ggrs/main/docs/pitfalls.md. Tier: other. Read 2026-10-27. Sure:
  confirmed in the source.
- **R1-ECS-F15.** Bevy 0.16 added ECS relationships ("entity-entity connections") and made bevy and many of its
  subcrates work without the standard library. Quotes: "One of the hottest ECS features is finally here"; "bevy itself
  and a ton of our subcrates no longer rely on Rust's standard library". Source:
  https://raw.githubusercontent.com/bevyengine/bevy-website/main/content/news/2025-04-24-bevy-0.16/index.md
  (2025-04-24). Tier: official. Read 2026-10-27. Sure: confirmed in the source (upgraded from inferred at the fix step:
  rust-reviewer got both quotes verbatim, 2026-10-28).

#### hecs (0.11.2)
- **R1-ECS-F16.** hecs is a library with no scheduler: gameplay code queries the world directly. Quotes: "It is a
  library, not a framework."; "In place of an explicit "System" abstraction, a World's entities are easily queried from
  regular code." The crate's stated goals, in order: fast traversals, a simple interface, a small dependency closure,
  and leaving out what users can build themselves ("fast traversals", "a simple interface", "a small dependency
  closure", "exclusion of externally-implementable functionality"). Source: https://docs.rs/hecs/latest/hecs/ (crate
  docs), 0.11.2. Tier: official (the crate's own docs). Read 2026-10-27. Sure: confirmed in the source (the order of the
  goals confirmed by rust-reviewer, 2026-10-28).
- **R1-ECS-F17.** A hecs world is archetypal and yields entities in arbitrary order. Quotes: "An unordered collection
  of entities, each having any number of distinctly typed components"; "The components of entities who have the same
  set of component types are stored in contiguous runs"; query: "Entities are yielded in arbitrary order."; "When old
  entities are despawned, their IDs will be reused on a future entity". Source:
  https://docs.rs/hecs/latest/hecs/struct.World.html, 0.11.2. Tier: official. Read 2026-10-27. Sure: confirmed in the
  source.
- **R1-ECS-F18.** A hecs `World` does not implement `Clone`. Its trait list has Default, Extend, FromIterator,
  IntoIterator, Send and Sync. Source: https://docs.rs/hecs/latest/hecs/struct.World.html (Trait Implementations),
  0.11.2. Tier: official. Read 2026-10-27. Sure: confirmed in the source (the impl list).
- **R1-ECS-F19.** hecs can make its entity ids come out the same in two worlds, if both hold the same live entities and
  the same freelist. Quote: freelist: "Entity handles will be allocated deterministically between different worlds (e.g.
  across serialization)". set_freelist: "May only be called when the set of live entity IDs exactly matches". Source:
  https://docs.rs/hecs/latest/hecs/struct.World.html (freelist, set_freelist), 0.11.2. Tier: official. Read 2026-10-27.
  Sure: confirmed in the source (the condition confirmed by rust-reviewer, 2026-10-28).
- **R1-ECS-F20.** hecs can't serialize a world on its own: the user writes a context that serializes each component
  type, and its compact format is column-major. Quotes: column format "Efficient, compact, and highly compressible,
  but difficult to read or edit by hand"; "Preferred for data that will be read/written programmatically"; "For some
  applications, a custom approach may be preferable." Sources: https://docs.rs/hecs/latest/hecs/serialize/index.html
  and https://docs.rs/hecs/latest/hecs/serialize/column/index.html, 0.11.2. Tier: official. Read 2026-10-27. Sure:
  confirmed in the source, including why the World has no Serialize: "Components are not necessarily serializable, so
  we cannot directly implement serde::Serialize for World." (rust-reviewer, 2026-10-28; the column format is "Available
  on crate feature `column-serialize` only").
- **R1-ECS-F21.** The hecs authors advise plain `Vec`s per entity type for games with few entity types. Quotes: "If
  your game will have few types of entities, consider a simpler architecture such as storing each type of entity in a
  separate plain Vec."; "ECS may be overkill for games that don't call for batch processing of entities." Source:
  https://github.com/Ralith/hecs (README, "Why Not ECS?"; no version shown). Tier: official (the crate's own
  repository). Read 2026-10-27. Sure: confirmed in the source.
- **R1-ECS-F22.** Even with an ECS, much of a game's state lives in other structures, such as a spatial index. Quotes:
  "Most games will store significant amounts of state in other structures."; "many games maintain a spatial index
  structure (e.g. a tile map or bounding volume hierarchy)". Source: https://github.com/Ralith/hecs (README, "Why Not
  ECS?"). Tier: official. Read 2026-10-27. Sure: confirmed in the source.

#### Plain data in Rust (std 1.99.1, slotmap 1.1.1)
- **R1-ECS-F23.** The standard `HashMap` is seeded at random per instance and iterates in arbitrary order. Quotes:
  "The algorithm is randomly seeded"; "each HashMap instance uses a different seed"; iter: "An iterator visiting all
  key-value pairs in arbitrary order." Source: https://doc.rust-lang.org/std/collections/struct.HashMap.html (struct
  docs, iter), std 1.99.1 (2026-10-13). Tier: official. Read 2026-10-27. Sure: confirmed in the source.
- **R1-ECS-F24.** `BTreeMap` iterates in key order. Quote: iterators "produce their items in key order, and take
  worst-case logarithmic and amortized constant time per item returned." Source:
  https://doc.rust-lang.org/std/collections/struct.BTreeMap.html, std 1.99.1. Tier: official. Read 2026-10-27. Sure:
  confirmed in the source.
- **R1-ECS-F25.** A slice (and so a `Vec`) iterates from start to end. Quote: "The iterator yields all items from
  start to end." Source: https://doc.rust-lang.org/std/primitive.slice.html (method iter), std 1.99.1. Tier:
  official. Read 2026-10-27. Sure: confirmed in the source.
- **R1-ECS-F26.** `Vec::clone_from` copies into the existing buffer without reallocating when it can. Quote: "This
  method is preferred over simply assigning source.clone() to self, as it avoids reallocation if possible.
  Additionally, if the element type T overrides clone_from(), this will reuse the resources of self's elements as
  well." The doc's example asserts that the buffer pointer is unchanged. Source:
  https://doc.rust-lang.org/std/vec/struct.Vec.html (impl Clone for Vec, clone_from), std 1.99.1. Tier: official.
  Read 2026-10-27. Sure: confirmed in the source.
- **R1-ECS-F27.** `Copy` is a cheap bitwise copy, while `Clone` may run arbitrary code, and `clone_from` exists so that
  types can reuse their allocations. Quotes: "Copy is implicit and an inexpensive bit-wise copy, while Clone is always
  explicit and may or may not be expensive."; clone_from "can be overridden to reuse the resources of a to avoid
  unnecessary allocations." Source: https://doc.rust-lang.org/std/clone/trait.Clone.html, std 1.99.1. Tier:
  official. Read 2026-10-27. Sure: confirmed in the source.
- **R1-ECS-F28.** Versioned (generational) keys keep a removed key invalid even when its slot is reused. Quote: "once a
  key is removed, it stays removed, even if the physical storage inside the slotmap is reused for new elements." The
  dense variant iterates "as fast as a normal Vec", and "It uses two indirections per random access". Keys and maps
  serialize through serde behind its `serde` feature: "Both keys and the slot maps have full (de)seralization support
  through the serde library." The crate supports no_std: "This crate also supports no_std environments, but does require
  the alloc crate to be available." Source: https://docs.rs/slotmap/latest/slotmap/, slotmap 1.1.1. Tier: official (the
  crate's own docs). Read 2026-10-27. Sure: confirmed in the source (the indirection cost and serde support confirmed by
  rust-reviewer, 2026-10-28).

#### Physics state (Rapier 0.36)
- **R1-ECS-F29.** Rapier keeps bodies in its own arena-backed set, which implements Clone, Serialize and Deserialize and
  is addressed by index-plus-generation handles. Quote: "The collection that stores all rigid bodies in your physics
  world."; handle: "This handle is lightweight (just an index + generation) and remains valid even if other bodies are
  removed." The trait list includes Clone, Serialize and Deserialize; its `clone_from` is not overridden (R1-ECS-F35).
  Source: https://docs.rs/rapier3d/latest/rapier3d/dynamics/struct.RigidBodySet.html, rapier3d 0.36.1. Tier: official.
  Read 2026-10-27. Sure: confirmed in the source.
- **R1-ECS-F30.** Rapier's whole simulation state can be serialized, and its pipelines hold no state. Quotes: "When the
  `serde-serialize` feature of Rapier is enabled, most data structures of Rapier become serializable using serde" (so
  the feature is required; the unit had left it out, both reviewers, 2026-10-28); "The complete state of the simulation
  is serialized by serializing its physics world as a whole."; "except for the PhysicsPipeline and the CollisionPipeline
  which don't hold any useful state." Source: https://rapier.rs/docs/user_guides/rust/serialization (guide navigation
  shows "Rust (0.36)"; no date). Tier: official. Read 2026-10-27. Sure: confirmed in the source.
- **R1-ECS-F31.** With enhanced determinism, Rapier promises byte-identical serialized states across machines. Quotes:
  "If the `enhanced-determinism` feature of Rapier is enabled" … "running the exact same simulation on two different
  machine will result in the exact same byte vectors."; the passage also sets the condition "if your platform fulfills
  the required determinism requirements" (verifier, 2026-10-28). Source:
  https://rapier.rs/docs/user_guides/rust/serialization. Tier: official. Read 2026-10-27. Sure: confirmed in the
  source. (Whether this holds between Wasm and native belongs to the determinism unit.)
- **R1-ECS-F32.** A user reported that Rapier 0.30.1 skipped part of its broad-phase state when serializing, so worlds
  restored from snapshots diverged; the reporter says it was fixed in Rapier 0.33. Quote (reporter): "BvhWorkspace is
  not serialized, but the BvhWorkspace::rebuild_frame_index value is actually important." Source:
  https://github.com/dimforge/rapier/issues/910 (opened 2026-01-16; closed by the reporter 2026-06-09; no maintainer
  reply). Tier: other (a user's issue report). Read 2026-10-27. Sure: confirmed in the source (the quote); the
  divergence and the fix version are the reporter's account, seen in the fetch summary (inferred).

#### Wasm size
- **R1-ECS-F33.** Generic code grows Wasm size, because the compiler makes one copy per type it is used with, and size
  has to be measured. Quotes: "rustc and LLVM will create a new copy of the function for each T type that the function
  is used with."; "these copies add up quickly in terms of code size."; on opt-level "s" against "z": "Always measure!";
  wasm-opt "can often get another 15-20% savings on code size." Source:
  https://rustwasm.github.io/book/reference/code-size.html (the Rust and WebAssembly book; the page calls itself
  "unpublished documentation"; no date). Tier: official, dated (the Rust and WebAssembly working group's book; its
  repository says "This repository was archived by the owner on Aug 25, 2025.", verifier 2026-10-28). Read 2026-10-27.
  Sure: confirmed in the source (short quotes).
- **R1-ECS-F34.** No source read in this run gives a Wasm size or a compile time for bevy_ecs, hecs or a plain-data
  core, so all size and compile-time marks below are proposed and must be measured. Sure: an absence across every source
  listed below, which no quote can confirm (first verifier pass: not checkable; second pass, 2026-10-28: unsupported,
  and harmless); so no recommendation rests on it (R1-ECS-REC1 cites it only to say why mark 5 measures; the label read
  "confirmed" until the second fix round).

#### Clone and clone_from (added at the fix step, 2026-10-28)
- **R1-ECS-F35.** `#[derive(Clone)]` clones field by field, the default `clone_from` is a plain clone, and Rapier's
  `RigidBodySet` keeps that default. Quotes (std): "The `derive`d implementation of `Clone` calls `clone` on each
  field."; "`a.clone_from(&b)` is equivalent to `a = b.clone()` in functionality, but can be overridden to reuse the
  resources of `a` to avoid unnecessary allocations." RigidBodySet's `clone_from`: "Performs copy-assignment from
  `source`.", its source link pointing to core's `clone.rs`, not to Rapier's file. Sources:
  https://doc.rust-lang.org/std/clone/trait.Clone.html (std 1.99.1) and
  https://docs.rs/rapier3d/latest/rapier3d/dynamics/struct.RigidBodySet.html (rapier3d 0.36.1, Trait
  Implementations). Tier: official. Read 2026-10-28 (fix step: rust-reviewer's risk 5 and the verifier's ECS finding 3
  named the gap; settles R1-ECS-OQ2). Sure: confirmed in the source (the quotes and the source link); that derive
  leaves `clone_from` at the default is our reading (the page says only that the derived Clone "calls clone on each
  field"; the fix step's "writes only `clone`" was corrected at the second fix round), and that a derived or default
  `clone_from` therefore builds new buffers, so a Rapier set clone allocates, is inferred.

#### Legacy facts used (unit `ecs`)

- **CS8** (the SIM world container): legacy keeps the authoritative ([GLOSSARY](GLOSSARY.md#the-authority)) state on
  three.js objects and mirrors it into flat
  records once per step. v2 inverts this: the Rust sim owns the state. The flat, pooled record schema (players, plus
  kinded static boxes with an alive flag and an optional Y rotation) is the starting data layout.
- **GW16** (scene-to-SIM mirror): reusing slots with no allocation was a win worth keeping in Rust's own layout; the
  mirror itself goes.
- **GC2** (soldier slot table): 12 fixed slots, where slot id = sim id, plus a per-team index with no per-frame
  searches. Measured: perfbots at 48 soldiers, combat, mean ([GLOSSARY](GLOSSARY.md#time-statistics)) 0.80–1.00 ms per
  update step, p95 0.97–1.54
  (docs/reports/2026-10-16-three-teams-core.md:48, via inventory CM4/GC2). This is a JS reference point for the scale
  test, not a v2 target. For v2, fixed slot tables sized by the mode fit plain arrays (VISION_V2 §6.1).
- **GN2** (bot roster record): one record mixes sim state with view state. v2 splits them, so the sim layout holds
  sim state only.
- **CS7** (resolveShot): every static has id −1, so a hit can't name which crate or wall it struck. v2 needs stable
  ids for static colliders too (destructibles, hit events).
- **GP14** (update order): the order of the step decides outcomes (the player acts first), and the sim shares
  Math.random with sound and FX. v2 needs explicit tick phases and a sim-only seeded stream held in the state.
- **GE2** (economy state): per-team state indexed by team id. These are plain per-team tables, not entities.
- **GC15** (Torque Bow bolts): a fly–stick–fuse projectile that follows the soldier it sticks to. A projectile needs a
  reference to another entity, and that reference must survive a snapshot and restore.
- **GC17 and VEHICLE_TYPES** (Juggernaut): the vehicle is not in the SIM at all, and its cannon fires from an animated
  node. v2 vehicles are sim entities with fixed mount points and seats as data (legacy: seats 1, maxHealth 1500).
- **UNIT_TYPES** (legacy-tuning.json): ability fields are ad-hoc columns on unit rows (banzaiCd, bayonetCd), which
  won't scale to the data-driven ability system (VISION_V2 §6.3).
- **CS1**: a fixed step (FIXED_DT) and an input-command record were declared but never wired. That is a lesson for the
  tick unit, noted here only because the state layout must be steppable at a fixed tick.

Entity kinds the layout must hold (LOOP_V2 §2–§6 and VISION_V2 §3, §6): soldiers (humans and bots in slots
([GLOSSARY](GLOSSARY.md#team-size-and-how-modes-are-written)); up to 48 at the goal
([GLOSSARY](GLOSSARY.md#the-start-modes-and-the-goal-modes)), "24v24" or "12v12v12", and LOOP_V2's
"12v12v12 to 16v16v16, possibly more"), vehicles (the Juggernaut and one signature tank per faction, multi-part, with
seats `[OPEN]`), projectiles (grenades, Molotovs, bolts, Javelin rockets),
harvesters, buildings (refinery, power plant, barracks), the carried tech cache and the uplinks (two at full scale),
per-team economy and tech, active ability effects, and static colliders. There are about a dozen kinds, known in
advance. The counts beyond 48 soldiers are not given in any design doc.

### 2. A fixed tick rate (unit `tick`)

Brief question (quoted): "a fixed tick rate" (docs/V2_PHASE0_BRIEF.md §4, row R1). Unit scope from the approved plan
(.checkpoints/v2-research/plan.md, Units, R1): "fixed tick rate"; as briefed: the fixed step, the fixed-timestep loop
on server and client (accumulator, interpolation for rendering), how the tick relates to the send and snapshot rates,
the cost per tick for 48+ ([GLOSSARY](GLOSSARY.md#the-design-point-and-headroom)) players plus bots on the server and
in a Web Worker on the minimum hardware, 30 Hz vs 60 Hz
(and others) for a top-down shooter with hitscan, projectiles and
[big vehicles](GLOSSARY.md#big-vehicle-and-very-large-unit), and what shipped games use
and why.

Every source was read 2026-10-27.
WebFetch caps a verbatim quote at about 125 characters, so long passages are quoted as consecutive fragments.
"Confirmed in the source" means the quoted words were returned verbatim; "inferred" means my reasoning or arithmetic
on top of a source, or a claim seen only in WebFetch's own summary.

**R1-TICK-F1.** A simulation stepped with a variable delta time behaves differently at different frame rates. Quote:
"The problem is that the behavior of your physics simulation depends on the delta time you pass in." Source:
https://gafferongames.com/post/fix_your_timestep/ (Glenn Fiedler, "Fix Your Timestep!", page dated 2004-06-10), section
"Variable delta time" (location corrected at the fix step, verifier 2026-10-28). Tier: other (practitioner article; the
brief names Fiedler's articles as a reference). Read 2026-10-27. Sure: confirmed in the source.

**R1-TICK-F2.** The standard fixed-step loop: the renderer produces real time, the simulation consumes it in fixed
`dt` steps, and the leftover is carried in an accumulator. Quote (code): `accumulator += frameTime;` /
`while ( accumulator >= dt )` / `integrate( state, t, dt );` / `accumulator -= dt;`. Source: as F1, section "Free the
physics". Tier: other. Read 2026-10-27. Sure: confirmed in the source.

**R1-TICK-F3.** A fixed-step loop that cannot keep up falls into a "spiral of death"; the simulation must cost much
less real time than the time it simulates, and the sample loop caps the time fed in per frame at 0.25 s. Quotes:
"It’s what happens when your physics simulation can’t keep up with the steps it’s asked to take." / "You really need
to ensure that it takes significantly less than X seconds of real time" "to update X seconds worth of physics
simulation." / code `if ( frameTime > 0.25 ) frameTime = 0.25;`. Source: as F1, section "Semi-fixed timestep" for the
spiral and headroom quotes and "The final touch" for the clamp (locations corrected at the fix step, verifier
2026-10-28). Tier: other. Read 2026-10-27. Sure: confirmed in the source.

**R1-TICK-F4.** Rendering between fixed steps interpolates the previous and current state by the accumulator's
remainder. Quotes: "One solution to this problem is to interpolate between the previous and current physics state"
"based on how much time is left in the accumulator:" / `const double alpha = accumulator / dt;` /
`State state = currentState * alpha + previousState * ( 1.0 - alpha );`. Source: as F1, section "The final touch".
Tier: other. Read 2026-10-27. Sure: confirmed in the source. That the rendered state therefore trails the newest step
by up to one `dt` is inferred (the page does not quantify it).

**R1-TICK-F5.** A fixed step is the precondition for exact reproducibility given the same inputs. Quotes: "What if
you want exact reproducibility from one run to the next given the same inputs?" / "This comes in handy when trying to
network your physics simulation using deterministic lockstep". Source: as F1. Tier: other. Read 2026-10-27. Sure:
confirmed in the source.

**R1-TICK-F6.** Bevy's fixed timestep defaults to 64 Hz rather than 60 Hz, for two stated reasons: a 60 Hz step can
beat against a 60 Hz monitor, and 1/64 s is exact in binary floats. Quotes: "The default timestep() is 64 hertz, or
15625 microseconds." / "using 60 hertz has the potential for a pathological interaction with the monitor refresh rate"
/ "the game alternates between running two fixed timesteps and zero fixed timesteps per frame" / "the value is a power
of two which losslessly converts into f32 and f64." Source: https://docs.rs/bevy/latest/bevy/time/struct.Fixed.html
(bevy 0.20.0), type-level docs. Tier: official (engine documentation). Read 2026-10-27. Sure: confirmed in the source
(fragments).

**R1-TICK-F7.** Bevy's fixed schedule does not run while its virtual clock is paused, and the virtual clock caps the
time added at once so a freeze or a suspended laptop does not trigger a catch-up "death spiral". Quotes: "If the
virtual clock is paused, the FixedUpdate schedule will not run." (Fixed page) / "Usually if a laptop is suspended for
an hour, it doesn’t make sense to try to simulate the game logic" / "By limiting the maximum time that can be added at
once, we also limit the amount of virtual time" (Virtual page). Sources:
https://docs.rs/bevy/latest/bevy/time/struct.Fixed.html and https://docs.rs/bevy/latest/bevy/time/struct.Virtual.html
(bevy 0.20.0). Tier: official. Read 2026-10-27. Sure: confirmed in the source (fragments). The default cap value was
not on the part of the page read; no number is claimed.

**R1-TICK-F8.** Rapier's physics step defaults to 1/60 s, is meant to match the target rate, and a smaller step costs
more CPU per simulated second. Quotes: "The timestep length - how much simulated time passes per physics step
(default: `1.0 / 60.0`)." / "Set this to `1.0 / your_target_fps`." / "Smaller timesteps are more accurate but require
more CPU time per second of simulated time." / `max_ccd_substeps`: "Maximum number of CCD substeps performed by the
solver (default: `1`)." Source: https://docs.rs/rapier3d/latest/rapier3d/dynamics/struct.IntegrationParameters.html
(rapier3d 0.36.1), fields `dt` and `max_ccd_substeps`. Tier: official. Read 2026-10-27. Sure: confirmed in the source.

**R1-TICK-F9.** Quake III Arena's server runs its world at 20 frames per second by default. Quote:
`sv_fps = Cvar_Get ("sv_fps", "20", CVAR_TEMP );`. Source:
https://raw.githubusercontent.com/id-Software/Quake-III-Arena/master/code/server/sv_init.c (GPL source, "Copyright (C)
1999-2005 Id Software, Inc."). Tier: shipped game (source code). Read 2026-10-27. Sure: confirmed in the source.

**R1-TICK-F10.** Quake III's server frame is an integer-millisecond accumulator loop, and it sends client messages
after the frame loop. Quotes: `frameMsec = 1000 / sv_fps->integer ;` / `sv.timeResidual += msec;` /
`while ( sv.timeResidual >= frameMsec ) {` / `VM_Call( gvm, GAME_RUN_FRAME, svs.time );` / then
`SV_SendClientMessages();`. Source: https://raw.githubusercontent.com/id-Software/Quake-III-Arena/master/code/server/sv_main.c,
function `SV_Frame`. Tier: shipped game. Read 2026-10-27. Sure: confirmed in the source. That integer division turns a
60 Hz setting into 16 ms frames (62.5 Hz) is inferred arithmetic.

**R1-TICK-F11.** Quake III sets each client's snapshot rate separately from the server frame rate, capped at 30 per
second, 20 by default. Quotes: `} else if ( i > 30 ) {` / `i = 30;` / `cl->snapshotMsec = 1000/i;` /
`cl->snapshotMsec = 50;`. Source:
https://raw.githubusercontent.com/id-Software/Quake-III-Arena/master/code/server/sv_client.c, `SV_UserinfoChanged`
("snaps"). Tier: shipped game. Read 2026-10-27. Sure: confirmed in the source.

**R1-TICK-F12.** Quake III integrates player movement per user command over that command's own duration, chops long
moves "to prevent framerate dependent behavior", and offers an optional fixed movement step (off by default, 8 ms).
Quotes: `msec = finalTime - pmove->ps->commandTime;` / "// chop the move up if it is too long, to prevent framerate"
"// dependent behavior" / `if ( pmove->pmove_fixed ) {` / `if ( msec > 66 ) {` (bg_pmove.c) /
`{ &pmove_fixed, "pmove_fixed", "0", CVAR_SYSTEMINFO, 0, qfalse},` /
`{ &pmove_msec, "pmove_msec", "8", CVAR_SYSTEMINFO, 0, qfalse},` (g_main.c). Sources:
https://raw.githubusercontent.com/id-Software/Quake-III-Arena/master/code/game/bg_pmove.c (`Pmove`) and
https://raw.githubusercontent.com/id-Software/Quake-III-Arena/master/code/game/g_main.c (cvar table). Tier: shipped
game. Read 2026-10-27. Sure: confirmed in the source. That movement runs once per received command, outside the world
frame, is confirmed by the verifier's extra read (sv_client.c:1424 `SV_ClientThink (cl, &cmds[ i ]);`, 2026-10-28); that
the command rate was finer than the 20 Hz world frame stays inferred (the client input code was not read).

**R1-TICK-F13.** In the Source engine the tick interval is a per-game constant: Counter-Strike's default is 66 ticks
per second, and the `-tickrate` override is compiled out for TF2, Counter-Strike and Day of Defeat. Quotes:
"[Forrest] For Counter-Strike, set default tick rate of 66 and removed -tickrate command line parameter." /
`#if !defined( TF_DLL ) && !defined( CSTRIKE_DLL ) && !defined( DOD_DLL )` / "Ignoring this for now, server ops are
abusing it" / "See shareddefs.h for redefining this.  Don't even think about it, though, for HL2.  Or you will pay."
Source: https://raw.githubusercontent.com/ValveSoftware/source-sdk-2013/master/src/game/server/gameinterface.cpp,
`CServerGameDLL::GetTickInterval` (Source SDK 2013, master). Tier: shipped game (source code). Read 2026-10-27. Sure:
confirmed in the source. The value of `DEFAULT_TICK_INTERVAL` itself was not found in the files read.

**R1-TICK-F14.** Source expresses game time in whole ticks, rounding a duration to the nearest tick, and notes the
quantization error. Quotes: `#define TIME_TO_TICKS( dt ) ( (int)( 0.5f + (float)(dt) / TICK_INTERVAL ) )` /
`#define TICKS_TO_TIME( t ) ( TICK_INTERVAL *( t ) )` / "// NOTE: The discrete ticks can have quantization error, so
these numbers are biased a little to". Source:
https://raw.githubusercontent.com/ValveSoftware/source-sdk-2013/master/src/game/shared/shareddefs.h. Tier: shipped
game. Read 2026-10-27. Sure: confirmed in the source.

**R1-TICK-F15.** MDN describes the browser game loop as a constant-frequency update with rendering as often as
possible, and lists updating in a Web Worker on a timer as one way to do it. Quotes: "One common technique is to update
the simulation at a constant frequency and then draw as much (or as little) of the actual frames as possible." / "Draw
on `requestAnimationFrame()` and update on a `setInterval()` or `setTimeout()` in a Web Worker." / "This is the same as
above, except update does not hog the main thread (nor does the main thread hog it). This is a more complex solution,
and might be too much overhead for simple updates." Source: https://developer.mozilla.org/en-US/docs/Games/Anatomy
("Anatomy of a video game", last modified 2026-08-21). Tier: official (MDN). Read 2026-10-27. Sure: confirmed in the
source. MDN marks its tick example "in need of technical review".

**R1-TICK-F16.** MDN names three ways to handle a long gap (a hidden tab or a slow machine): treat it as a pause,
simulate it, or recover state from the server; and it advises watching the catch-up tick count. Quotes: "Consider the
gap 'a pause' and skip the time." / "You can simulate the gap to catch up." / "This can be a problem for long drops
and/or complex updates." / "You can recover the game state from a peer or the server." / "you should keep track of how
large numTicks is. If it is large, then either your game was asleep, or the machine cannot keep up." Source: as F15.
Tier: official. Read 2026-10-27. Sure: confirmed in the source. (MDN calls the pause option "problematic for most
multiplayer games": verbatim, confirmed by the verifier, 2026-10-28.)

**R1-TICK-F17.** `requestAnimationFrame`, on the main thread or in a dedicated worker, runs at the display's refresh
rate and is paused in background tabs in most browsers. Quotes: "The frequency of calls to the callback function will
generally match the display refresh rate. The most common refresh rate is 60 Hz, (60 cycles/frames per second), though
75 Hz, 120 Hz, and 144 Hz are also widely used." / "`requestAnimationFrame()` calls are paused in most browsers when
running in background tabs or hidden `<iframe>`s, to improve performance and battery life." Source:
https://developer.mozilla.org/en-US/docs/Web/API/DedicatedWorkerGlobalScope/requestAnimationFrame (last modified
2025-09-25; Baseline "available across browsers since March 2023"). Tier: official. Read 2026-10-27. Sure: confirmed in
the source.

**R1-TICK-F18.** Browsers clamp and throttle timers: a 4 ms minimum after five nested `setTimeout` calls; minimum delays
in inactive tabs (Firefox desktop: 1 s); Chrome's intensive throttling (from Chrome 88) checks timers once per minute
when a page has been hidden for more than 5 minutes and three more conditions hold (a timer chain of 5 or more, 30 s of
silence, no WebRTC in use: R1-BND-F31, R1-TICK-F19; the unit had shortened this to "hidden for 5 minutes", verifier
2026-10-28); and timers fire late when the thread is busy. Quotes: "As specified in the HTML standard, browsers will
enforce a minimum timeout of 4 milliseconds once a nested call to setTimeout has been scheduled 5 times." / "To reduce
the load (and associated battery usage) from background tabs, browsers will enforce a minimum timeout delay in inactive
tabs." / "Firefox Desktop has a minimum timeout of 1 second for inactive tabs." / "Timers in this state are checked once
per minute" / "The timeout can also fire later than expected if the page (or the OS/browser) is busy with other tasks."
Source: https://developer.mozilla.org/en-US/docs/Web/API/Window/setTimeout (last modified 2026-10-26), section "Reasons
for longer delays than specified" (heading corrected at the fix step). Tier: official. Read 2026-10-27. Sure: confirmed
in the source.

**R1-TICK-F19.** Neither MDN's `setTimeout` page nor Chrome's throttling post says whether timers inside a dedicated
worker are throttled when the tab is hidden. Sources: as F18, and https://developer.chrome.com/blog/timer-throttling-in-chrome-88
("Last updated 2021-01-18"; Chrome 88), which says hidden-page timers are checked "once per minute" and exempts pages
where "WebRTC is in use". Tier: official. Read 2026-10-27. Sure: inferred (an absence reported by WebFetch on both
pages).

**R1-TICK-F20.** `performance.now()` works in workers, is coarsened to 100 µs (5 µs when cross-origin isolated), and
in a worker counts from when the worker started. Quotes: "Resolution in isolated contexts: 5 microseconds" /
"Resolution in non-isolated contexts: 100 microseconds" / "Note: This feature is available in Web Workers." / "the time
when the worker is run in Worker and ServiceWorker contexts". Source:
https://developer.mozilla.org/en-US/docs/Web/API/Performance/now (last modified 2025-08-25), "Security requirements"
and description. Tier: official. Read 2026-10-27. Sure: confirmed in the source.

**R1-TICK-F21.** `performance.now()` stops counting during OS sleep on every OS except Windows. Quote: "The
specification (Level 2) requires that performance.now() should tick when the operating system sleeps or the browser
process freezes in any other way. It appears that only browsers on Windows keep ticking during sleep." Source: as F20,
section "Ticking during sleep". Tier: official. Read 2026-10-27. Sure: confirmed in the source.

**R1-TICK-F22.** `Atomics.wait` can block a worker thread until a notify or a millisecond timeout, needs a typed array
on a `SharedArrayBuffer`, and is not allowed on the main thread. Quotes: "The Atomics.wait() static method verifies
that a shared memory location contains a given value and if so sleeps, awaiting a wake-up notification or a timeout." /
"It is blocking and cannot be used in the main thread." / "This method only works with an Int32Array or BigInt64Array
that views a SharedArrayBuffer." / timeout: "Time to wait in milliseconds." Source:
https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Atomics/wait (last modified
2026-08-31). Tier: official. Read 2026-10-27. Sure: confirmed in the source.

**R1-TICK-F23.** Tokio's timers have millisecond granularity (coarser on Windows), and its `Interval` catches up on
missed ticks by bursting unless told otherwise. Quotes: "`Sleep` operates at millisecond granularity and should not be
used for tasks that require high-resolution timers." / "some platforms (specifically Windows) will provide timers with
a larger resolution than 1 ms." (fn.sleep) / Burst: "Ticks as fast as possible until caught up." / Skip: "Skips missed
ticks and tick on the next multiple of `period` from `start`." / "For these reasons, MissedTickBehavior::Burst is the
default for MissedTickBehavior." / "these strategies will only apply when the delay is greater than 5 milliseconds."
(enum.MissedTickBehavior). Sources: https://docs.rs/tokio/latest/tokio/time/fn.sleep.html and
https://docs.rs/tokio/latest/tokio/time/enum.MissedTickBehavior.html (tokio 1.54.1). Tier: official. Read 2026-10-27.
Sure: confirmed in the source.

**R1-TICK-F24.** Client-side prediction keeps a ring buffer of the local player's inputs and states and, on a server
correction, replays from the corrected state to the present predicted time; with 100 ms each way the correction is
200 ms old. Quotes: "The solution is to keep a circular buffer of past character state and input for the local player
on the client" / the client "first discards any buffered state older than the corrected state from the server" "and
replays the state starting from the corrected state back to the present “predicted” time on the client" / "if it takes
100ms from client to server and 100ms back". Source:
https://gafferongames.com/post/what_every_programmer_needs_to_know_about_game_networking/ (Glenn Fiedler, 2010-02-24).
Tier: other. Read 2026-10-27. Sure: confirmed in the source (fragments). That the number of replayed steps is about the
round-trip time divided by the tick length, and so doubles from 30 Hz to 60 Hz, is inferred arithmetic.

**R1-TICK-F25.** The interpolation delay for remote entities follows the snapshot send rate, not the simulation rate:
Fiedler gives 150 ms at 30 snapshots per second and 85 ms at 60. Quotes: "a delay of 150ms" (30 pps) / "60 packets
per-second needs only 85ms". Source: https://gafferongames.com/post/snapshot_interpolation/ (Glenn Fiedler, 2014-11-30),
conclusion. Tier: other. Read 2026-10-27. Sure: confirmed in the source for the two numbers; his "about 3x the packet
interval for 2-5% loss" rule was seen only in WebFetch's summary (inferred). R2 owns these budgets.

**R1-TICK-F26.** Explicit Euler is exact only when the rate of change is constant over a step, so integration error
depends on the step size; Fiedler recommends semi-implicit Euler, which most commercial physics engines use. Quotes:
"It is only 100% accurate when the rate of change is constant over the timestep." / "After 10 seconds, the object
should have moved 500 meters, but explicit euler gives a result of 450 meters." / "Most commercial game physics engines
use this integrator." / "My recommendation is semi-implicit euler." Source:
https://gafferongames.com/post/integration_basics/ (Glenn Fiedler, 2004-06-01). Tier: other. Read 2026-10-27. Sure:
confirmed in the source (fragments).

#### Legacy facts used (unit `tick`)

- **CS1** (docs/LEGACY_INVENTORY.md:169): `FIXED_DT` = 1/60 s is declared (shamble-core.js:1479) but never read; the
  live loop uses `clock.getDelta()` clamped to 0.05 s (shamble-game.js:3053). Mark: drop. Tells v2: a declared tick
  is not a tick; the v2 tick must drive the loop and be proven by tests.
- **GP13** (:323): one `requestAnimationFrame` loop runs `update(dt)` and freezes the whole world for pause, shop,
  tutorial and hit-stop. Mark: redesign (a fixed-tick Rust sim in a Worker or on the server, a render loop that
  interpolates). Tells v2: a shared authoritative sim never pauses for one player's UI.
- **GP14** (:324): the step's order decides outcomes; the player acts before every bot; wall-clock reads and a shared
  `Math.random` leak into the step. AUDIT_BUGS_PERF.md:10: `update()` 0.49 ms per frame (scripted 60 s combat).
  Tells v2: the tick is explicit phases in a fixed order, with no clock read inside it.
- **GW14** (:202): the jump apex depends on the frame rate, measured 2.25 / 2.43 / 2.48 m at 20 / 60 / 144 Hz against
  2.52 m from the formula, so the reachable ledge is about 2.98 m at 60 Hz while `EJUMP_REACH` assumes 3.07 m.
  Tells v2: reach follows from the tick and the integrator; map validation must use the integrated value.
- **GN4** (:269): bot sight checks run on a budget per game second (`TUNE.ai.losPerS` 480, banked up to 0.1 s), bots
  retarget every 0.25 s and re-check sight every 0.1 s, staggered by slot; fairness was measured at 60 and 144 fps.
  Tells v2: slow systems already run on their own cadence, independent of the step rate.
- **GN10** (:275, added at the fix step: R1-TICK-REC6 cites it): bots climb and jump against `EJUMP_REACH`, 3.07 m by
  the formula, while the jump actually integrated reaches about 2.98 m at 60 Hz (GW14). Tells v2: bots and map
  validation take reach from the same integrated value.
- **GC15** (:254) and **GC14** (:253): bolts are swept from the previous to the new position each step, so they never
  tunnel, but flight integrates the frame `dt`, so the landing point moves with the frame rate. Tells v2: keep swept
  tests at any tick; fix the step.
- **GC17** (:256): the Juggernaut ripples shells 80 ms apart, and a wall-clock idle bob moves its muzzles. Tells v2:
  vehicle fire timing in ticks, from fixed mount points.
- **GP9** (:319): hit-stop freezes `update()` for everyone for 0.12 s. Tells v2: hit-stop is client-only presentation
  time.
- **TL13** (:398) and **TL15** (:400): `tools/sim.js` and `tools/perfbots.js` step `update(1/60)`
  (tools/sim.js:19, tools/perfbots.js:24). At 48 soldiers in combat, one 1/60 s step costs 0.80–1.00 ms mean, p95
  0.97–1.54 ms (docs/reports/2026-10-16-three-teams-core.md:48; headless Node on the dev container, comparable only on
  the same machine). Tells v2: the legacy behaviour we have measured is at 60 steps per second; the legacy cost is no
  prediction of v2's (true 3D, physics, abilities), only an order of magnitude for this JS step.
- **KT13** (:422): the netcode rule already asks for a fixed tick, no wall clock and seeded randomness in the shared
  sim.
- **Tuning** (docs/LEGACY_TUNING.md, from docs/legacy-tuning.json): Simulation `FIXED_DT` 0.016666…; `WEAPONS`
  `fireInterval` 0.072 s (shabzon), 0.2 s (amk), 0.42 s (lancer) and `burstInterval` 0.06 s (lancer); `JUG.rippleMs`
  80; `T.moveSpeed` 6.3 m/s, `T.sprintMult` 1.4286, `T.gravity` 24, `T.jumpV` 11, `T.fragSpeed` 64; `T_CHARGE.bow.sMax`
  94 m/s. Arithmetic on these (inferred): the shortest legacy interval (0.06 s) is 1.8 ticks at 30 Hz and 3.6 ticks at
  60 Hz, so no legacy weapon needs two shots in one tick at either rate, but none of 0.072, 0.06 or 0.08 s is a whole
  number of ticks at 30, 60 or 64 Hz; a sprinting soldier (9.0 m/s) moves 0.30 m per tick at 30 Hz and 0.15 m at 60 Hz;
  the fastest bolt (94 m/s) 3.13 m and 1.57 m.

### 3. Identical results between Wasm and native (unit `determinism`)

Brief question (quoted): "**identical results between Wasm and native** (floats, Rapier's determinism options)"
(`docs/V2_PHASE0_BRIEF.md` §4, R1 row). Unit scope from the task: IEEE 754 and Wasm float semantics, relaxed SIMD,
Rust's float guarantees, fused multiply-add and auto-vectorization, transcendental functions, Rapier's determinism
features, fixed point, hash-map order, seeded RNG, the legacy's determinism debts, how a test proves identical
results, and what the netcode actually requires.

All sources read
2026-10-27. "Confirmed in the source" means the finding quotes the source's own words (returned verbatim by the
fetch tool, which caps a quote at 125 characters); "inferred" means a summary, a reading of code or data, or a
conclusion drawn from confirmed parts.

#### WebAssembly floats

- **R1-DET-F1.** Wasm floating-point follows IEEE 754 and every operator rounds to nearest, ties to even: "All
  operators use round-to-nearest ties-to-even, except where otherwise specified." · Source:
  https://webassembly.github.io/spec/core/exec/numerics.html, "Floating-Point Operations" (WebAssembly 3.0, header
  dated 2026-10-27; it showed 2026-10-23 at the first read the same day) · official · read 2026-10-27 · confirmed in
  the source.
- **R1-DET-F2.** When a Wasm float operator returns NaN, its sign and (unless every input NaN is canonical) its payload
  are non-deterministic: "then its sign is non-deterministic and the payload is computed as follows:" · "If the
  payload of all NaN inputs to the operator is canonical" (then canonical) · "Otherwise the payload is picked
  non-deterministically among all arithmetic NaNs" · Source: same page, "NaN Propagation" · official · read
  2026-10-27 · confirmed in the source.
- **R1-DET-F3.** Wasm 3.0 defines a deterministic profile, but a host chooses whether to run only it, and tools target
  the full profile by default: "All NaN values generated by floating-point instructions are canonical and positive." ·
  "All relaxed vector instructions have a fixed behaviour that does not depend on the implementation." · "Even under
  this profile, the memory.grow and table.grow instructions technically remain non-deterministic" · "A host platform
  can thereby decide to support the language only under a restricted profile" · "Tools are generally expected to
  handle and produce code for the full profile by default." · Source:
  https://webassembly.github.io/spec/core/appendix/profiles.html (WebAssembly 3.0, 2026-10-27) · official · read
  2026-10-27 · confirmed in the source. No source read says which profile browsers run; that we cannot count on the
  deterministic profile in Chrome, Edge, Firefox or Safari is inferred.
- **R1-DET-F4.** Relaxed SIMD operators may give hardware-dependent results, fixed for one run of a program;
  relaxed multiply-add may be fused or not: "set of possible results may depend on properties of the host
  environment, such as its hardware." · "These choices are fixed, that is, parameters are constant during the
  execution of any given program." · "Relaxed multiply-add allows for fused or unfused results, which leads to
  implementation-dependent rounding behaviour." · In the deterministic profile "every parameter is prescribed to be
  0" (the unfused, regular alternatives) · Source: numerics page, "Relaxed Operations" · official · read 2026-10-27 ·
  confirmed in the source. So the same Wasm module can give different results in two browsers on two machines.
- **R1-DET-F5.** Relaxed SIMD ships in Chrome 114 and Firefox 145, sits behind a flag in Safari
  (`useWebAssemblyRelaxedSIMD`), and is in Node.js 21.0 and Wasmtime 15; fixed-width SIMD is in Chrome 91, Firefox 89,
  Safari 16.4 · Source: https://raw.githubusercontent.com/WebAssembly/website/main/features.json (the data behind
  webassembly.org/features; main branch, no date) · official (WebAssembly CG website) · read 2026-10-27 · confirmed in
  the source (upgraded at the fix step: the verifier matched the values verbatim, 2026-10-28: `"relaxedSimd": "114"` /
  `"145"` / `["flag", "Requires JavaScriptCore flag useWebAssemblyRelaxedSIMD"]` / `"21.0"` / `"15"`).
- **R1-DET-F6.** Rust's `wasm32-unknown-unknown` target enables only some proposals by default: "As of the time of this
  writing the proposals that are enabled by default (the `generic` CPU in LLVM terminology) are:" multivalue,
  mutable-globals, reference-types, sign-ext, nontrapping-fptoint, bulk-memory; others are opted into with
  "`-Ctarget-feature=+foo`" · Source: https://doc.rust-lang.org/rustc/platform-support/wasm32-unknown-unknown.html
  (rustc book; no Rust version on the page; it names Rust 1.87.0+ and "as of 2025-10-03") · official · read
  2026-10-27 · confirmed in the source for the list; that `simd128` and `relaxed-simd` are therefore off unless we turn
  them on is inferred (they are absent from the list).

#### Rust floats

- **R1-DET-F7.** Rust's basic float operations are IEEE-rounded: "When a primitive operation (addition, subtraction,
  multiplication, or division) is performed on this type, the result is rounded according to the roundTiesToEven
  direction defined in IEEE 754-2008." · Source: https://doc.rust-lang.org/std/primitive.f32.html, type docs (std
  1.99.1, 58cf8f9c3 2026-10-13) · official · read 2026-10-27 · confirmed in the source.
- **R1-DET-F8.** Rust does not make NaN bits portable: "Rust does not currently guarantee that the bit patterns of NaN
  are preserved over arithmetic operations, and they are not guaranteed to be portable or even fully deterministic!" ·
  Source: same page, "NaN bit patterns" (rust-reviewer places the sentence in the type-level NaN bullet) · official ·
  read 2026-10-27 · confirmed in the source.
- **R1-DET-F9.** Rust std's transcendental functions have no fixed precision: on `sin`, "The precision of this function
  is non-deterministic. This means it varies by platform, Rust version, and can even differ within the same execution
  from one invocation to the next." · Source: same page, `sin`, "Unspecified precision" · official · read 2026-10-27 ·
  confirmed in the source for `sin`, and by the verifier (2026-10-28) for `cos`, `tan`, `atan2`, `powf`, `exp` and `ln`;
  that the same note sits on `asin`, `acos`, `atan`, `exp2`, `log*`, `powi`, `hypot`, `cbrt` and the hyperbolic
  functions is inferred (the fetch tool's list, not quoted per method). rust-reviewer (2026-10-28) also found on `tan`:
  "This function currently corresponds to the `tanf` from libc on Unix and Windows."
- **R1-DET-F10.** `sqrt` and `mul_add` are exactly specified and "guaranteed not to change"; rounding functions are
  exact: `sqrt`: "The result of this operation is guaranteed to be the rounded infinite-precision result. It is
  specified by IEEE 754 as `squareRoot` and guaranteed not to change." · `mul_add`: "with only one rounding error" and
  the same guarantee as `fusedMultiplyAdd` · `floor`, `ceil`, `round`, `trunc`, `fract`: "This function always returns
  the precise result." · Source: same page, the methods' "Precision" sections · official · read 2026-10-27 · confirmed
  in the source.
- **R1-DET-F11.** Rust never fuses `a*b + c` or applies fast-math on its own; opting in is explicit: "Providing strict
  IEEE 754-2008 guarantees precludes many transformations, such as turning `a*b + c` into FMA operations" · "it seems
  preferable to have explicit opt-in for such semantics-alternating transformations" · Source:
  https://rust-lang.github.io/rfcs/3514-float-semantics.html (RFC 3514, start date 2023-10-14, in the RFC book) ·
  official · read 2026-10-27 · confirmed in the source. The opt-in exists: the `algebraic_*` methods mean "the results
  of elementary operations may have undefined precision" (f32 page, "algebraic operators"; confirmed); that they are
  stable since 1.98.0 is confirmed by rust-reviewer (`algebraic_add` "1.98.0 (const: 1.98.0)", 2026-10-28). That LLVM
  auto-vectorization therefore cannot reorder float sums in our code is inferred from the strict-IEEE rule, not stated
  for vectorization in any source read.
- **R1-DET-F12.** The main 64-bit targets are fully IEEE-compliant in Rust; the exceptions are 32-bit x86 without SSE2
  and 32-bit ARM NEON: "The most widely used targets (64bit x86 and 64bit ARM, and also the increasingly popular RISC-V)
  are fully compliant." · "On 32bit x86 without SSE2 (i586 targets), x87 registers are used even more pervasively" ·
  semantics are "without abruptUnderflow/flush-to-zero" · Source: RFC 3514 · official · read 2026-10-27 · confirmed in
  the source. wasm32 appears in neither list, but the RFC names wasm for NaN: "Also note that in some cases even the
  underlying target behavior is non-deterministic, namely on wasm." (rust-reviewer, 2026-10-28).
- **R1-DET-F13.** Compile-time float evaluation may choose different NaN bits than run time: "there is no guarantee that
  the choice made in const evaluation is consistent with the choice made at runtime." · Source: RFC 3514 · official ·
  read 2026-10-27 · confirmed in the source.
- **R1-DET-F14.** `min` and `max` may return either zero for `+0.0` and `-0.0`: "If the inputs compare equal (such as
  for the case of `+0.0` and `-0.0`), either input may be returned non-deterministically." `clamp` likewise: "either
  `0.0` or `-0.0` is returned non-deterministically." · Source: f32 page, `min`, `max`, `clamp` · official · read
  2026-10-27 · confirmed in the source. rust-reviewer (2026-10-28) adds from the same page: `clamp` "Panics if `min >
  max`, `min` is NaN, or `max` is NaN."
- **R1-DET-F15.** Float-to-integer casts are fully defined on every target, but integer overflow depends on build flags:
  "Casting from a float to an integer will round the float towards zero" · "`NaN` will return `0`" · out-of-range
  values "will saturate" · "Integer operators will panic when they overflow when compiled in debug mode." · "The `-C
  debug-assertions` and `-C overflow-checks` compiler flags can be used to control this more directly." · Source:
  https://doc.rust-lang.org/reference/expressions/operator-expr.html, "Numeric cast" and "Overflow" (The Rust
  Reference; no version on the page) · official · read 2026-10-27 · confirmed in the source.

#### Transcendental functions

- **R1-DET-F16.** On `wasm32-unknown-unknown`, Rust's runtime support crate supplies the pure-Rust libm's math symbols;
  on Linux it does not, so the system libm is used: "/// This group of functions has more performance or precision
  issues than system versions, or" · "/// - All unix targets (linux, macos, freebsd, android, etc)" · "/// - wasm
  with known target_os" (the cfg excludes `unix` and `all(target_family = "wasm", not(target_os = "unknown"))`; the
  group declares `acos`, `acosf`, `asin`, …) · Source:
  https://raw.githubusercontent.com/rust-lang/compiler-builtins/master/compiler-builtins/src/math/mod.rs (master
  branch) · official (rust-lang repository) · read 2026-10-27 · confirmed in the source. That std's `f32::sin` binds to
  these symbols, so a browser client and a Linux server compute `sin` with different code, is inferred.
- **R1-DET-F17.** The `libm` crate (0.2.16) is "libm in pure Rust" with f32 and f64 `sin`, `cos`, `tan`, `atan2`,
  `exp`, `log`, `pow`, `sqrt`, `hypot` and more; its default `arch` feature swaps in hardware code only for exact
  operations: wasm32 `pub use wasm32::{ ceil, ceilf, fabs, fabsf, floor, floorf, rint, rintf, sqrt, sqrtf, trunc,
  truncf, };` · x86 with SSE2 `pub use x86::{sqrt, sqrtf, fma, fmaf};` · AArch64 with NEON `fma`, `fmaf`, `rint`,
  `rintf`, `sqrt`, `sqrtf` · Sources: https://docs.rs/libm/latest/libm/ (0.2.16, 2026-09-13);
  https://raw.githubusercontent.com/rust-lang/compiler-builtins/master/libm/src/math/arch/mod.rs (master) · official ·
  read 2026-10-27 · confirmed in the source for the function lists; that `arch` therefore cannot change results (those
  operations are exactly specified, F10) is inferred.
- **R1-DET-F18.** The libm docs and README read make no claim of identical results across platforms and give no
  accuracy bound · Sources: docs.rs page above; https://raw.githubusercontent.com/rust-lang/compiler-builtins/master/libm/README.md
  · official · read 2026-10-27 · inferred (an absence, as the fetch tool reported it).

#### Rapier

- **R1-DET-F19.** Rapier by default is deterministic only on one machine: "By default, Rapier is locally deterministic,
  meaning that running the exact same simulation" (on the same machine with the same versions) · "However, doing this
  on two different computers may result in completely different results." · Source:
  https://rapier.rs/docs/user_guides/rust/determinism (Rapier user guide, Rust, docs version 0.36; no page date) ·
  official · read 2026-10-27 · confirmed in the source.
- **R1-DET-F20.** Rapier's cross-platform determinism needs the `enhanced-determinism` feature, IEEE 754-2008 targets
  (Wasm included), identical construction and insertion order, and nalgebra's trait functions for trig: "The
  `enhanced-determinism` feature of Rapier is enabled." · "The target platforms must strictly comply to the IEEE
  754-2008 floating-points standard." · "This include most modern mainstream processors as well as WASM targets." ·
  "and they are added/removed to sets (rigid-body sets, etc.) in the exact same order." · "you need to make sure the
  functions being used originate from the `ComplexField` or `RealField` traits from `nalgebra`" · Source: same page ·
  official · read 2026-10-27 · confirmed in the source.
- **R1-DET-F21.** `enhanced-determinism` excludes `simd8` but combines with `parallel`: "Note that the
  `enhanced-determinism` feature cannot be enabled at the same time as the `simd8` feature," which goes on "which
  changes the SIMD lane width and is therefore its own determinism domain." (rust-reviewer, 2026-10-28) · "The
  `parallel` feature, on the other hand, can be combined with it: the results of the parallel solver are identical" ·
  Source: same page · official · read 2026-10-27 · confirmed in the source.
- **R1-DET-F22.** In rapier3d 0.36.1, `enhanced-determinism` forces the pure-Rust libm in simba and passes the flag to
  parry; the default solver runs a 4-lane SIMD path, which the flag does not switch off:
  `enhanced-determinism = ["simba/libm_force", "parry3d/enhanced-determinism"]` · `# Widens the solver's SIMD from 4 to
  8 lanes (f32 only), replacing the default` `# 4-lane path.` · `simba = { workspace = true, features = ["wide"] }` ·
  Source: https://raw.githubusercontent.com/dimforge/rapier/master/crates/rapier3d/Cargo.toml (master; workspace
  version 0.36.1 per https://raw.githubusercontent.com/dimforge/rapier/master/Cargo.toml) · official (Dimforge
  repository) · read 2026-10-27 · confirmed in the source (lines quoted). Whether the 4-lane path gives bit-identical
  results on wasm32 and x86-64 / AArch64 is stated in no source read.
- **R1-DET-F23.** In parry3d, `enhanced-determinism` also forces libm and scalar maths in glamx and pulls in indexmap:
  `enhanced-determinism = ["simba/libm_force", "indexmap", "glamx/libm", "glamx/scalar-math"]` · Source:
  https://raw.githubusercontent.com/dimforge/parry/master/crates/parry3d/Cargo.toml (master; Rapier's workspace pins
  parry3d 0.31.1, nalgebra 0.35, simba 0.10.2, glamx 0.3.1: confirmed verbatim by the verifier and rust-reviewer,
  2026-10-28) · official · read 2026-10-27 · confirmed in the source for the line; that indexmap is there for a stable
  iteration order is inferred. The workspace manifest also notes why glam's fast-math stays off: it "re-enables FMA
  contraction in" … "glam's sse2 backend (a cross-platform determinism hazard)." (confirmed by rust-reviewer,
  2026-10-28).
- **R1-DET-F24.** Rapier's determinism page gives no performance cost for `enhanced-determinism` and no method to check
  determinism · Source: the determinism page · official · read 2026-10-27 · inferred (an absence, as the fetch tool
  reported it).

#### Collections, RNG, fixed point, JavaScript

- **R1-DET-F25.** std `HashMap` is randomly seeded and iterates in arbitrary order; a fixed hasher is possible but gives
  up HashDoS resistance (the seed is per instance: R1-DET-F33): "The algorithm is randomly seeded, and a reasonable
  best-effort is made to generate this seed from a high quality, secure source of randomness provided by the host
  without blocking the program." · `iter`: "An iterator visiting all key-value pairs in arbitrary order." · "The hashing
  algorithm can be replaced on a per-`HashMap` basis using the `default`, `with_hasher`, and `with_capacity_and_hasher`
  methods." · "Setting it manually using this function can expose a DoS attack vector." · Source:
  https://doc.rust-lang.org/std/collections/struct.HashMap.html (std 1.99.1) · official · read 2026-10-27 · confirmed in
  the source. Corrected at the fix step: the unit had written "seeded per process", which the source contradicts (wrong,
  verifier and rust-reviewer, 2026-10-28); re-sourced as R1-DET-F33.
- **R1-DET-F26.** std `BTreeMap` iterates in key order: "Given a key type with a total order, an ordered map stores its
  entries in key order." · `iter`: "Gets an iterator over the entries of the map, sorted by key." · Source:
  https://doc.rust-lang.org/std/collections/struct.BTreeMap.html (std 1.99.1) · official · read 2026-10-27 · confirmed
  in the source.
- **R1-DET-F27.** rand's `StdRng` is not portable and may change algorithm: "Non-portable: any future library version
  may replace the algorithm and results may be platform-dependent." · "even with a fixed seed, output is not
  portable." · Source: https://docs.rs/rand/latest/rand/rngs/struct.StdRng.html (rand 0.10.3, 2026-09-20) · official ·
  read 2026-10-27 · confirmed in the source.
- **R1-DET-F28.** rand_chacha and rand_pcg generators are portable and seedable; ChaCha has independent streams and a
  settable position: "These generators are all deterministic and portable (see Reproducibility in the book)," "with
  testing against reference vectors." (rand_chacha 0.10.0, 2026-09-13; the same words on rand_pcg 0.10.2,
  2026-09-22) · ChaCha8Rng `set_stream`: "This is initialized to zero; 2^64 unique streams of output are available per
  seed/key." (the superscript was lost in the page text) · `set_word_pos`: "Set the offset from the start of the
  stream, in 32-bit words." · rand_pcg `Pcg64Mcg` "has poor performance on 32-bit CPUs" · Sources:
  https://docs.rs/rand_chacha/latest/rand_chacha/, https://docs.rs/rand_chacha/latest/rand_chacha/struct.ChaCha8Rng.html,
  https://docs.rs/rand_pcg/latest/rand_pcg/ · official · read 2026-10-27 · confirmed in the source.
- **R1-DET-F29.** The `fixed` crate (1.31.0, 2026-09-20) gives binary fixed-point numbers but no trigonometry: "Representation
  of binary fixed-point numbers up to 128 bits wide." · "does _not_ provide general analytic functions." · "No
  trigonometric functions are provided, for example no `sin` or `cos`." · Source: https://docs.rs/fixed/latest/fixed/ ·
  official · read 2026-10-27 · confirmed in the source. Rapier and parry ship f32 and f64 builds only (the `f32`
  feature and the parry3d / parry3d-f64 crates in the manifests of F22): inferred; so a fixed-point sim cannot use
  Rapier.
- **R1-DET-F30.** JavaScript's `Math` functions differ between browsers and machines, and `Math.random` takes no seed:
  "Many `Math` functions have a precision that's _implementation-dependent_." · "This means that different browsers can
  give a different result. Even the same JavaScript engine on a different OS or architecture can give different
  results!" · Source: https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Math (MDN, last
  modified 2025-07-10) · official (browser vendor docs) · read 2026-10-27 · confirmed in the source; "no seed" is
  confirmed by the verifier on MDN `Math.random()`
  (https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Math/random, 2026-10-28): "it cannot
  be chosen or reset by the user."

#### Shipped games and what the netcode needs

- **R1-DET-F31.** Lockstep games need bit-identical results; shipped PC games reached them with IEEE floats and strict
  control, and found transcendental functions differ between CPU vendors: Battlezone 2 required "absolutely identical
  results on every client, down to the least-significant bit of the mantissa" · Gas Powered Games (Supreme Commander):
  "Also, every tick we assert that these fpu settings are still set:" · "We have never had a problem with the IEEE
  standard across any PC cpu AMD and Intel with this approach." · Pandemic: "we discovered that AMD and Intel processors
  produced slightly different results for trancendental functions" · Source:
  https://gafferongames.com/post/floating_point_determinism/ (Glenn Fiedler, 2010-02-24; quotes shipped-game developers;
  C and C++ on x87 and SSE) · other (an article that quotes shipped-game developers; relabelled from "shipped game" at
  the fix step, per the three tiers of docs/research/README.md: R1-XU-OQ6) · read 2026-10-27 · confirmed in the source
  (as quoted by the article).
- **R1-DET-F32.** With server authority and client prediction, a client that disagrees with the server is corrected, not
  desynchronized; lockstep cannot absorb a difference: prediction means "running a subset of the game code for your
  player character on the client machine." · on correction the client "invisibly 'rewinds and replays' the last n frames
  of local player character movement while holding the rest of the world fixed." · "the client's position is seldom
  corrected." · lockstep: "one tiny difference results in complete desynchronization over time." · Source:
  https://gafferongames.com/post/what_every_programmer_needs_to_know_about_game_networking/ (Glenn Fiedler, 2010-02-24)
  · other (practitioner article; relabelled at the fix step: R1-XU-OQ6) · read 2026-10-27 · confirmed in the source.

#### Added at the fix step (2026-10-28)

- **R1-DET-F33.** std `HashMap` takes its own random seed for each instance, not one per process. Quote: "As
  explained above, `HashMap` is randomly seeded: each `HashMap` instance uses a different seed, which means that
  `HashMap::new` normally cannot be used in a `const` or `static` initializer." · Source:
  https://doc.rust-lang.org/std/collections/struct.HashMap.html, section "Usage in `const` and `static`" (std 1.99.1,
  58cf8f9c3 2026-10-13) · official · read 2026-10-28 (fix step; replaces the "per process" part of F25, which both
  reviewers found wrong; agrees with R1-ECS-F23) · confirmed in the source. That two maps built in one process can
  therefore iterate the same keys in different orders is inferred (arbitrary order, F25, plus a seed per instance).
- **R1-DET-F34.** rand_pcg's Pcg32 suits 32-bit and 64-bit CPUs and can jump ahead; it takes a stream number in its
  constructor, but its docs warn that generators on different streams may be closely correlated. Quotes: "Pcg32 aka
  Lcg64Xsh32, officially known as pcg32, a general purpose RNG."; "This is a good choice on both 32-bit and 64-bit CPUs
  (for 32-bit output)." (crate root); `pub fn new(state: u64, stream: u64) -> Self`: "Construct an instance compatible
  with PCG seed and stream."; "Note that two generators with different stream parameters may be closely correlated."
  (`new`; the struct docs say it again); `advance`: "Multi-step advance functions (jump-ahead, jump-back)"
  (Lcg64Xsh32) · Sources: https://docs.rs/rand_pcg/latest/rand_pcg/ and
  https://docs.rs/rand_pcg/latest/rand_pcg/struct.Lcg64Xsh32.html (rand_pcg 0.10.2) · official · read 2026-10-28 (fix
  step; the verifier found REC4's Pcg32 fallback unbacked, since F28 quotes only Pcg64Mcg) · confirmed in the source
  (the correlation warning by the second verifier pass, 2026-10-28). Corrected at the second fix round: the fix step
  had closed with "the pages say nothing on how streams relate", which that pass found wrong.

#### Legacy facts used (unit `determinism`)

- **CS1** and `docs/LEGACY_TUNING.md` "Simulation: tick, input bits, hit bodies" (`FIXED_DT` 0.016666666666666666,
  shamble-core.js:1479): a fixed step and an input-command record are declared, nothing reads them, and the loop runs a
  variable dt clamped to 0.05 s (shamble-game.js:3053). Tells v2: the legacy proves nothing about fixed-step
  determinism, and its docs called it deterministic when it was not; v2 must prove it by test.
- **STATUS.md:70** (ADR-001 inputs): 11 wall-clock reads make same-seed matches differ; boot-time `Math.random` draws
  per slot; three.js UUIDs draw from the shared `Math.random`; the bot-shot FX cull makes the shared sequence
  camera-dependent; "cosmetic effects need their own RNG stream". Tells v2: one sim RNG, never shared with
  presentation, and no clock in the sim.
- **GP14** (update order): the 11 reads are 10 calls plus one comment; two reach sim outcomes through animation (the
  piloted Juggernaut's muzzle bob from `tickPilot`'s clock; `_animClock` breathing moves the muzzle grenades launch
  from); `sfxShot` fills a noise buffer from `Math.random` before the spread draws, only when audio is unlocked and not
  muted, so spread depends on mute and the device's sample rate; camera shake draws inside `update()`. A parity harness
  had to seed `Math.random` and stub audio and FX (three-teams-core.md:39). Tells v2: the sim's inputs are only the
  tick, the inputs and the seed; animation and sound never feed it.
- **GN6** (bot gunfire) and **GC5** (player firing): spread comes from the shared `Math.random`; v2 needs a seeded sim
  RNG. GC5 also measured that cooldowns reset instead of carrying the remainder, so a gun fires 10.0 / 12.0 / 13.1 shots
  per second at 30 / 60 / 144 Hz against 13.9 from its table value. Tells v2: timers live in the sim's ticks (or carry
  their remainder), identical on server and client.
- **GN8** (bot-shot FX cull): with `Math.random` seeded and effects stubbed, cull on and off gave identical damage logs
  (60 s, 69 events; 180 s, 207 events). Tells v2: seeding plus isolating presentation gives identical outcomes, and
  per-tick logs are a usable proof method.
- **CS4** (ray vs sphere and capsule): its v2 note says "pick f32 or f64 once and test against the same fixtures on
  both". **CS6** and **CS10**: NaN handling in `rayStatics` and `hasLOS` is accidental (a NaN range counts as blocked,
  a NaN direction silently misses); "Define NaN handling explicitly." Tells v2: NaN must never enter sim state (F2, F8).
- **CM9** (building kit roll): `_mulberry32` is the legacy's only seeded PRNG, and the editor stores the resolved spec,
  not the seed alone. Tells v2: content stores results, so content never depends on an RNG's version.
- **TL13** (sim.js): unseeded random AI needs many runs; LOOP_V2 §11 asks for at least 3 runs per build and the scale
  test at N ([GLOSSARY](GLOSSARY.md#player-count-n)) = 12 / 24 / 36 / 48; the row's note says v2 should run seeded,
  deterministic bot matches on the native
  server. Tells v2: same-build reproducibility pays off in testing and balance work at once.

### 4. The Rust ↔ TypeScript boundary, and the simulation in a Web Worker (unit `boundary-worker`)

Brief question (quoted, `docs/V2_PHASE0_BRIEF.md` §4, R1 row): "the Rust ↔ TypeScript boundary (shared memory,
wasm-bindgen, generated TypeScript types); running the simulation in a Web Worker".

Unit scope (from the task): how inputs go in and snapshots and events come out with no call per entity per frame;
wasm-bindgen and type generation; SharedArrayBuffer and Atomics against postMessage with transferables; reading state
straight out of Wasm memory; Wasm threads; cross-origin isolation (COOP and COEP) and what it costs; the worker on the
minimum hardware; browser support in Chrome, Edge, Firefox and Safari; offline play through the same simulation in
the worker. All pass marks and the reference hardware
below are **proposed**.

Tiers: every source in this unit is official (vendor docs, specifications, the Rust project, the spec repos and the
browser-compat data that MDN publishes). No shipped-game source was found (see section 7, "Sources not read"). "Confirmed in the source"
means the claim rests on the sentence quoted; "inferred" means it rests on a summary or on reasoning across sources.

**Tooling: wasm-bindgen and type generation**

- **R1-BND-F1.** The Rust and WebAssembly working group's GitHub org was wound down: wasm-bindgen moved to a new
  `wasm-bindgen` org with new maintainers, and every other rustwasm repository (wasm-pack, gloo, twiggy, walrus and the
  rest) was to be archived in place or handed to its existing maintainers. Quotes: "All other repositories in the
  rustwasm organization are going to be archived in place", which goes on "or transferred to their existing maintainers
  if they elect to do so," (rust-reviewer, 2026-10-28; the unit had left the second half out) · "a critical mass of new
  maintainers has already been reached" · "Transferring wasm-bindgen will occur in the weeks following this post." ·
  Source: https://blog.rust-lang.org/inside-rust/2025/07/21/sunsetting-the-rustwasm-github-org/ (Inside Rust blog,
  2025-07-21) · official · read 2026-10-27 · confirmed in the source.
- **R1-BND-F2.** wasm-bindgen ships often and with breaking changes in 2026; since 0.2.128 the CLI and the library must
  be the same version. Quote (0.2.128, "05 Sep"): "Requires matching `wasm-bindgen` and CLI versions (schema bump)."
  0.2.123 was "a breaking change for callers that relied on widening `&mut` references." (verifier, 2026-10-28). The
  cadence runs from 0.2.118 on 10 Apr to 0.2.129 on 25 Sep. · Sources:
  https://github.com/wasm-bindgen/wasm-bindgen/releases/tag/0.2.128 and
  https://github.com/wasm-bindgen/wasm-bindgen/releases · official · read 2026-10-27 · version-match rule, the dates and
  0.2.123's break confirmed in the source; the year 2026 comes from the release assets' timestamps ("2026-09-25T…",
  rust-reviewer, 2026-10-28), not from the page text. Corrected at the fix step: the unit had also listed 0.2.122 as
  breaking, which its release notes do not say (unsupported, both reviewers).
- **R1-BND-F3.** wasm-bindgen writes a TypeScript declaration file by default. Quote: "Output a TypeScript
  declaration file for the generated JavaScript bindings. This is on by default." · Source:
  https://raw.githubusercontent.com/wasm-bindgen/wasm-bindgen/main/guide/src/reference/cli.md (guide source, main
  branch) · official · read 2026-10-27 · confirmed in the source.
- **R1-BND-F4.** `typescript_custom_section` appends text to that generated `.d.ts` and is meant for code generation.
  Quotes: "it will append the contents of the string to the `.d.ts` file exported by `wasm-bindgen-cli`" · "The
  primary target for this feature is for code generation." · Source:
  https://raw.githubusercontent.com/wasm-bindgen/wasm-bindgen/main/guide/src/reference/attributes/on-rust-exports/typescript_custom_section.md
  · official · read 2026-10-27 · confirmed in the source.
- **R1-BND-F5.** Passing structured data through serde costs many boundary calls. Quotes: serde-wasm-bindgen "works
  by directly manipulating JavaScript values" and "requires a lot of calls back and forth between Rust and
  JavaScript, which can sometimes be slow"; the JSON route "can be anywhere from 2x to 0.2x the speed of
  `serde-wasm-bindgen`". · Source:
  https://raw.githubusercontent.com/wasm-bindgen/wasm-bindgen/main/guide/src/reference/arbitrary-data-with-serde.md
  · official · read 2026-10-27 · confirmed in the source.
- **R1-BND-F6.** Slices passed through wasm-bindgen are copied, not shared: a `&mut [T]` parameter "only accepts a
  `TypedArray`, since the results are copied back into its buffer"; plain JS arrays are "copied into the Wasm memory
  the same way as a `TypedArray`". · Source:
  https://raw.githubusercontent.com/wasm-bindgen/wasm-bindgen/main/guide/src/reference/types/number-slices.md ·
  official · read 2026-10-27 · confirmed in the source for `&mut` and plain arrays; for `&[T]` from a TypedArray,
  inferred.
- **R1-BND-F7.** The Rust and WebAssembly book's interface rule: keep long-lived data inside Wasm linear memory and
  let JS read it through a typed-array view, so nothing is copied per tick. Quotes: "Minimizing copying into and out
  of the WebAssembly linear memory. Unnecessary copies impose unnecessary overhead." · "large, long-lived data
  structures are implemented as Rust types that live in the WebAssembly linear memory" · "we get a pointer to the
  universe's cells, construct a `Uint8Array` overlaying the cells buffer" · "By working with pointers and overlays,
  we avoid copying the cells across the boundary on every tick." · Source:
  https://rustwasm.github.io/docs/book/game-of-life/implementing.html (the page says the project is "no longer
  maintained") · official (archived) · read 2026-10-27 · confirmed in the source.
- **R1-BND-F8.** tsify generates TypeScript definitions for Rust types used with wasm-bindgen. Quote: "Tsify is a
  library for generating TypeScript definitions from rust code." Version 0.5.8, repository madonoharu/tsify. Its
  values cross through serde-wasm-bindgen: "`js` enables serialization through serde-wasm-bindgen and generates the
  appropriate types for it."; its `into_wasm_abi`/`from_wasm_abi` attributes are "Deprecated in favour of using
  `Ts<T>`" (both confirmed by rust-reviewer, 2026-10-28). A second crate, tsify-next 0.5.6 (repository siefkenj/tsify,
  page date 2026-10-05), exists; neither page says which line is maintained. · Sources:
  https://docs.rs/tsify/latest/tsify/ and https://docs.rs/tsify-next/latest/tsify_next/ · official (crate docs) ·
  read 2026-10-27 · confirmed in the source (the serde route and the deprecation upgraded by rust-reviewer, 2026-10-28;
  that neither page says which line is maintained is an absence both reviewers confirmed).
- **R1-BND-F9.** ts-rs generates TypeScript declarations from Rust types at test time, with no runtime part. Quotes:
  "Generate typescript type declarations from rust types." · bindings are written "when running `cargo test` or `cargo
  test export_bindings`". Version 12.0.1 (MIT, repository Aleph-Alpha/ts-rs); output to `./bindings` or
  `TS_RS_EXPORT_DIR`; the default `serde-compat` feature means "serde attributes are parsed for enums and structs".
  MSRV: "The Minimum Supported Rust Version for this crate is 1.88.0" (confirmed by rust-reviewer, 2026-10-28). ·
  Source: https://docs.rs/ts-rs/latest/ts_rs/ · official (crate docs) · read 2026-10-27 · confirmed in the source.
- **R1-BND-F10.** Only `#[repr(C)]` (or another explicit representation) gives a Rust struct a layout another
  language can read by offset. Quotes: default representation: "The only data layout guarantees made by this
  representation are those required for soundness" and "The ordering does not have to be the same as the order in
  which the fields are specified"; C representation: "The second purpose is to create types that you can soundly
  perform operations on that rely on data layout"; layout rule: "For each field in declaration order in the struct
  ... add padding bytes to the current offset until it is a multiple of the field's alignment." · Source:
  https://doc.rust-lang.org/reference/type-layout.html (Rust Reference, current) · official · read 2026-10-27 ·
  confirmed in the source.
- **R1-BND-F11.** A typed array built on a buffer is a view, not a copy, and its offset must be a multiple of its
  element size. Quotes: "When called with an `ArrayBuffer` or `SharedArrayBuffer` instance, and optionally a
  `byteOffset` and a `length` argument, a new typed array view is created that views the specified buffer." ·
  RangeError when "`byteOffset` is not an integral multiple of the new typed array's element size." · Source:
  https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/TypedArray (last modified
  2026-10-26) · official · read 2026-10-27 · confirmed in the source.

**Shared memory, Atomics, transfer, Wasm memory**

- **R1-BND-F12.** Sharing memory between the page and a worker requires a secure, cross-origin-isolated page; without
  it `postMessage` throws for a SharedArrayBuffer. Quotes: "To use shared memory your document must be in a secure
  context and cross-origin isolated." · "Unless the two headers mentioned above are set, the various `postMessage()`
  APIs will throw for `SharedArrayBuffer` objects. If they are set, `postMessage()` on `Window` objects and dedicated
  workers will function and allow for memory sharing." · Source:
  https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/SharedArrayBuffer (last
  modified 2026-02-10) · official · read 2026-10-27 · confirmed in the source.
- **R1-BND-F13.** Writes to shared memory reach the other thread only "eventually"; ordering needs Atomics. Quotes: "a
  side effect to the block in one agent will eventually become visible in the other agent" · "To synchronize, atomic
  operations are needed." (SharedArrayBuffer page) · "Atomic operations make sure that predictable values are written
  and read, that operations are finished before the next operation starts and that operations are not interrupted."
  · Sources: the SharedArrayBuffer page above and
  https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Atomics (last modified
  2026-02-10) · official · read 2026-10-27 · confirmed in the source.
- **R1-BND-F14.** The page cannot block on shared memory; it can only wait asynchronously. Quotes: `Atomics.wait`:
  "(Most browsers will not allow `wait()` on the browser's main thread.)" · `Atomics.waitAsync` "is non-blocking and,
  unlike `Atomics.wait()`, can be used on the main thread." · Sources: the Atomics page above and
  https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Atomics/waitAsync (last modified
  2026-02-10) · official · read 2026-10-27 · confirmed in the source.
- **R1-BND-F15.** Transferring an ArrayBuffer moves it without copying, and the sender loses it (that data outside the
  transfer list is copied is R1-BND-F34). Quotes: "the memory resource that it points to is _literally_ moved between
  contexts in a fast and efficient zero-copy operation" · "the original object is no longer usable ... and any attempt
  to read or write the object will throw an exception." Typed arrays are not transferable, their buffers are:
  "…serializable, but not transferable. However their underlying buffer is an ArrayBuffer, which is a transferable
  object."; the list includes `WebTransportReceiveStream` and `WebTransportSendStream` (both confirmed by the verifier,
  2026-10-28). · Source: https://developer.mozilla.org/en-US/docs/Web/API/Web_Workers_API/Transferable_objects (last
  modified 2025-09-18) · official · read 2026-10-27 · confirmed in the source. Corrected at the fix step: the unit's
  "without a transfer list the data is copied" is not on this page (unsupported, verifier 2026-10-28); re-sourced as
  R1-BND-F34.
- **R1-BND-F16.** Growing a non-shared Wasm memory detaches every existing view of it; for shared memory the old
  buffer keeps its old length. Quotes: "Every call to `grow` will detach any references to the old `buffer`, even for
  `grow(0)`!" · "For a shared `Memory` instance, the initial `buffer` ... will not become detached, but rather its
  length will not be updated." · Source:
  https://developer.mozilla.org/en-US/docs/WebAssembly/Reference/JavaScript_interface/Memory/grow (last modified
  2026-05-14) · official · read 2026-10-27 · confirmed in the source.
- **R1-BND-F17.** A Wasm memory is a SharedArrayBuffer only when created shared, a module must declare a shared memory
  to import one, and a shared memory must have a fixed maximum. Quotes: "When this flag is set to `true`, the
  constructed `Memory` object can be shared between workers via `postMessage()`, just like `SharedArrayBuffer`, and the
  backing `buffer` of the `Memory` object is a `SharedArrayBuffer`." (SharedArrayBuffer page) · "It is a validation
  error to attempt to import shared linear memory if the module's memory import" "doesn't specify that it allows shared
  memory." · "Note that shared linear memory without an explicit maximum size is not permitted." · Sources: the
  SharedArrayBuffer page above and
  https://raw.githubusercontent.com/WebAssembly/threads/main/proposals/threads/Overview.md (threads proposal overview,
  main) · official · read 2026-10-27 · confirmed in the source. Corrected at the fix step: the unit had quoted the
  source's other rule ("Similarly, it is a validation error to import non-shared memory if the module's memory import
  specifies it as being shared."); the rule the claim needs is quoted above (verifier, re-read 2026-10-28).
- **R1-BND-F18.** Building Rust for Wasm threads needs nightly Rust, the atomics target feature and a rebuilt
  standard library, works only with the `web` or `no-modules` output, and keeps `std::thread` unusable. Quotes: "Note
  that this requires a nightly Rust toolchain." · `RUSTFLAGS='-C target-feature=+atomics'` with `-Z
  build-std=panic_abort,std` · "Currently it's required to use the `--target no-modules` or `--target web` flag" ·
  "The main thread in a browser cannot block." · "the standard library has no viable route to implement the
  `std::thread` module" · "This demo should work in the latest Firefox and Chrome versions at this time." · Source:
  https://raw.githubusercontent.com/wasm-bindgen/wasm-bindgen/main/guide/src/examples/raytrace.md (guide source,
  main, undated) · official · read 2026-10-27 · confirmed in the source. That Rust emits a shared memory only in this
  atomics build is inferred (the page says threads need it; it does not say so of shared memory in those words).
- **R1-BND-F19.** On the default Rust Wasm target, `std::thread::spawn` panics, and the default Wasm features include
  bulk-memory and nontrapping-fptoint from Rust 1.87.0. Quote: "`std::thread::spawn` will panic. There is no means by
  which this can be overridden." The same page: "By default the `wasm32-unknown-unknown` target is compiled with
  `-Cpanic=abort`." (rust-reviewer, 2026-10-28; so a panic ends the sim's instance: R1-RR-OQ3). Default features listed:
  multivalue, mutable-globals, reference-types, sign-ext, nontrapping-fptoint (Rust 1.87.0+, LLVM 20+), bulk-memory
  (Rust 1.87.0+, LLVM 20+). Target tier 2. · Source:
  https://doc.rust-lang.org/rustc/platform-support/wasm32-unknown-unknown.html (rustc book, current) · official · read
  2026-10-27 · confirmed in the source.

**Cross-origin isolation and its costs**

- **R1-BND-F20.** Cross-origin isolation is COOP `same-origin` plus COEP `require-corp` or `credentialless`. Quote:
  "To use these features in a document, you will need to set the COOP header to `same-origin` and the
  `Cross-Origin-Embedder-Policy` header to `require-corp` (or `credentialless`)." It also unlocks finer timers and a
  memory API: isolated documents get "`Performance.now()` offers better precision" and
  "`Performance.measureUserAgentSpecificMemory()` can be called". · Sources:
  https://developer.mozilla.org/en-US/docs/Web/HTTP/Reference/Headers/Cross-Origin-Opener-Policy (last modified
  2026-07-12) and https://developer.mozilla.org/en-US/docs/Web/API/Window/crossOriginIsolated (last modified
  2026-08-21) · official · read 2026-10-27 · confirmed in the source.
- **R1-BND-F21.** Under `require-corp`, every cross-origin `no-cors` resource needs a CORP header, and frames are
  covered too. Quotes: "A document can only load resources requested in `no-cors` mode from the same origin, or
  resources that have explicitly set the `Cross-Origin-Resource-Policy` header to a value that allows it to be
  embedded." · `credentialless`: requests "are sent without credentials: cookies are omitted in the request, and
  ignored in the response." · "The entire chain of the documents, including parent frames and child frames, must be
  cross-origin isolated, too." · Sources:
  https://developer.mozilla.org/en-US/docs/Web/HTTP/Reference/Headers/Cross-Origin-Embedder-Policy (last modified
  2026-09-04) and https://web.dev/articles/coop-coep (last updated 2022-06-21) · official · read 2026-10-27 ·
  confirmed in the source.
- **R1-BND-F22.** Safari has no `credentialless`, so an isolated page on Safari must use `require-corp`. Browser
  compatibility data: COEP header Chrome "83", Edge "mirror", Firefox "79", Safari "15.2"; `credentialless` Chrome
  "96", Edge "mirror", Firefox "119", Safari `false`. web.dev's line that credentialless "is not yet supported by other
  browsers" than Chrome and Edge is out of date. · Source:
  https://raw.githubusercontent.com/mdn/browser-compat-data/main/http/headers/Cross-Origin-Embedder-Policy.json (main
  branch) · official · read 2026-10-27 · confirmed in the source (data values).
- **R1-BND-F23.** COOP `same-origin` cuts the link to popups that do not opt in; MDN names OAuth and payment
  services as the case for the relaxed value `same-origin-allow-popups`. Quotes: "A document with
  `same-origin` will always open a document with `unsafe-none` in a new BCG." · "When opened in a new BCG, any
  references between the new document and its opener are severed." · `same-origin-allow-popups` is for "when using a
  cross-origin service for OAuth or payments". · Source: the COOP page above · official · read 2026-10-27 ·
  confirmed in the source; the consequence for a later login (VISION_V2 §5) inferred. Corrected at the fix step: the
  unit had also said that `same-origin-allow-popups` "does not give isolation"; no source read says so (unsupported,
  verifier 2026-10-28), and MDN's COOP page, re-read 2026-10-28, reads both ways: the value "is used to relax the
  `same-origin` restriction for integrations where a document needs the benefits of cross-origin isolation but also
  needs to open and retain a reference to trusted cross-origin documents", while its isolation section says "you will
  need to set the COOP header to `same-origin`" (F20). Now an open question: R1-BND-OQ9.
- **R1-BND-F24.** Rollout is testable before it is enforced, and the headers must cover worker scripts as well.
  Quote: with `Cross-Origin-Embedder-Policy-Report-Only` you "will receive reports without blocking embedded
  content." The headers must also reach worker scripts: "Recursively apply this to all documents including the top-level
  document, iframes and worker scripts." (confirmed by the verifier, 2026-10-28). · Source:
  https://web.dev/articles/coop-coep (published 2020-04-13, last updated 2022-06-21) · official, dated · read
  2026-10-27 · confirmed in the source (both).

**Browser support (desktop Chrome, Edge, Firefox, Safari)**

- **R1-BND-F25.** Module workers work in all four target browsers: `Worker` `options.type` "module": Chrome "80",
  Edge "mirror", Firefox "114", Safari "15". The wasm-bindgen guide's line that "only Chrome supports modules in web
  workers" is out of date. · Sources: https://raw.githubusercontent.com/mdn/browser-compat-data/main/api/Worker.json
  (main) and https://raw.githubusercontent.com/wasm-bindgen/wasm-bindgen/main/guide/src/examples/wasm-in-web-worker.md
  · official · read 2026-10-27 · confirmed in the source (data values).
- **R1-BND-F26.** SharedArrayBuffer and Atomics are in all four: SharedArrayBuffer Chrome "68", Edge "mirror",
  Firefox "79", Safari "15.2"; growable SharedArrayBuffer Chrome "111", Firefox "128", Safari "16.4"; `Atomics.wait`
  and `notify` Chrome "68", Firefox "78", Safari "15.2"; `Atomics.waitAsync` Chrome "90", Firefox "145", Safari
  "16.4". MDN marks waitAsync "Baseline 2025" ("since November 2025"). · Sources:
  https://raw.githubusercontent.com/mdn/browser-compat-data/main/javascript/builtins/SharedArrayBuffer.json,
  https://raw.githubusercontent.com/mdn/browser-compat-data/main/javascript/builtins/Atomics.json, the waitAsync page
  above · official · read 2026-10-27 · confirmed in the source (data values).
- **R1-BND-F27.** The Wasm features a default Rust build uses, and Wasm threads, are in all three engines: threads
  Chrome "74", Firefox "79", Safari "14.1"; bulk memory "75"/"79"/"15"; reference types "96"/"79"/"15"; non-trapping
  float-to-int "75"/"64"/"15"; sign extension "74"/"62"/"14.1"; multi-value "85"/"78"/"13.1". Edge is not listed (it is
  Chromium). `mutable-globals`, in Rust's default list (F19), is not among these figures (verifier, 2026-10-28). ·
  Source: https://raw.githubusercontent.com/WebAssembly/website/main/features.json (the webassembly.org feature table,
  main) · official · read 2026-10-27 · confirmed in the source (data values); "Edge follows Chrome" inferred.
- **R1-BND-F28.** wasm-bindgen supports all four browsers. Quote: "Firefox, Chrome, Safari, and Edge browsers are
  all supported by `wasm-bindgen`." · Source:
  https://raw.githubusercontent.com/wasm-bindgen/wasm-bindgen/main/guide/src/reference/browser-support.md ·
  official · read 2026-10-27 · confirmed in the source.

**Networking from a worker, timing and the hardware**

- **R1-BND-F29.** Both transports the netcode plans can be opened inside a worker. WebTransport: "Note: This feature
  is available in Web Workers." and "Since March 2026, this feature works across the latest devices and browser
  versions." WebSocket: "This feature is available in Web Workers." · Sources:
  https://developer.mozilla.org/en-US/docs/Web/API/WebTransport (last modified 2026-10-12) and
  https://developer.mozilla.org/en-US/docs/Web/API/WebSocket (last modified 2024-09-25) · official · read 2026-10-27
  · confirmed in the source.
- **R1-BND-F30.** Timer resolution is coarser without isolation, and a worker has its own time origin. Quotes:
  "Resolution in isolated contexts: 5 microseconds" · "Resolution in non-isolated contexts: 100 microseconds" ·
  `performance.now()` is available in workers, and its origin there is "the time when the worker is run". · Source:
  https://developer.mozilla.org/en-US/docs/Web/API/Performance/now (last modified 2025-08-25) · official · read
  2026-10-27 · confirmed in the source.
- **R1-BND-F31.** Chrome throttles timers in hidden pages, to once per minute after five minutes; the article does not
  say whether workers are affected. Quotes: "The page has been hidden for more than 5 minutes." · "The chain count is
  5 or greater." · "The page has been silent for at least 30 seconds." · "the browser will check timers in this group
  once per minute". · Source: https://developer.chrome.com/blog/timer-throttling-in-chrome-88 (last updated
  2021-01-18; Chrome 88) · official · read 2026-10-27 · conditions confirmed in the source; the article's silence on
  workers inferred from its summary.
- **R1-BND-F32.** The core count a page sees is not a hardware fact. Quotes: `hardwareConcurrency` "returns the number
  of logical processors available to run threads on the user's computer" · "The browser may, however, choose to
  report a lower number of logical cores in order to represent more accurately the number of `Worker`s that can run
  at once, so don't treat this as an absolute measurement". · Source:
  https://developer.mozilla.org/en-US/docs/Web/API/Navigator/hardwareConcurrency (last modified 2024-10-16) ·
  official · read 2026-10-27 · confirmed in the source.
- **R1-BND-F33.** Vite (docs v8.3.4) bundles module workers and Wasm and can set dev-server headers. Quotes: "A web
  worker script can be imported using `new Worker()` and `new SharedWorker()`." · "The worker detection will only work
  if the `new URL()` constructor is used directly inside the `new Worker()` declaration." · Wasm: "When you need
  control over when and how the module is instantiated, import it with `?init`." · `server.headers`: "Specify server
  response headers." · Sources: https://vite.dev/guide/features and https://vite.dev/config/server-options ·
  official · read 2026-10-27 · confirmed in the source.

**Added at the fix step (2026-10-28)**

- **R1-BND-F34.** `postMessage` hands data to a worker through the structured clone algorithm, which copies it; a
  transferred buffer (F15) is the exception. Quote: "The structured clone algorithm copies complex JavaScript objects.
  It is used internally when invoking structuredClone(), to transfer data between Workers via postMessage(), storing
  objects with IndexedDB, or copying objects for other APIs." · Source:
  https://developer.mozilla.org/en-US/docs/Web/API/Web_Workers_API/Structured_clone_algorithm (last modified
  2025-06-29) · official · read 2026-10-28 (fix step; re-sources F15's copy-by-default clause, which the verifier found
  unsupported on F15's page) · confirmed in the source. The page does not contrast copying with transferring; that a
  buffer named in the transfer list is moved instead is F15.
- **R1-BND-F35.** A Wasm memory's buffer is created with a detach key that names the Wasm memory. Quote: "Set
  buffer.[[ArrayBufferDetachKey]] to "WebAssembly.Memory"." (the algorithm "create a fixed length memory buffer", step
  5) · Source: https://webassembly.github.io/spec/js-api/ §5.3 "Memories" (WebAssembly JavaScript Interface, Editor's
  Draft dated 27 October 2026 when the fix step read it; the second verifier pass, the same day, found it dated 28
  October 2026, the step unchanged) · official · read 2026-10-28 (fix step; the verifier's boundary finding 1: REC2's
  one copy per tick depends on the sim's memory not being handed over) · confirmed in the source for the step. That a
  transfer of this buffer therefore throws is inferred: the rule that a keyed buffer cannot be detached by a transfer
  is in ECMA-262 and the HTML standard, both outside the allow list. Either way, transferring the sim's memory would
  take it from the worker (F15).

#### Legacy facts used (unit `boundary-worker`)

- **CS1** (fixed tick and input-command groundwork, mark: drop). `FIXED_DT` 1/60 s, the `BTN` bitmask (LEGACY_TUNING
  "Simulation: tick, input bits, hit bodies": FIRE 1, ADS 2, RELOAD 4, MELEE 8, GRENADE 16, SHIELD 32, SPRINT 64) and
  `makeInputCommand(seq, tick)` were declared and never wired. For v2: the idea of one input command per tick (seq,
  tick, move, aim, buttons) crosses the boundary as one small record; its exact format is R2's (ADR-0001). The lesson
  is to wire it from the first spike, not to declare it.
- **GP2** (input mapping and listeners, mark: port to TypeScript). Legacy runs reload, ability, grenade, bow release,
  E, weapon switch and ADS at event time, outside `update()`. For v2: listeners stay on the page and only emit the
  command; one-shot actions become press and release edges carried so a press is neither lost nor doubled (the row's
  own v2 note).
- **GP13** (frame loop, mark: redesign). One rAF loop runs a variable-dt `update(dt)` (dt clamped to 0.05 s) and
  freezes the world for pause, shop, tutorial and hit-stop. For v2: the fixed-tick sim runs apart from the render
  loop, which interpolates; nothing on the page can pause the sim.
- **GP14** (the step order, mark: rewrite in Rust). HUD writes are interleaved through the step; `update()` costs
  0.49 ms per frame (AUDIT_BUGS_PERF §1). For v2: the sim touches no DOM, and the HUD reads snapshots and events on the
  page.
- **GP10** (HUD writers, mark: port to TypeScript, look brief pending). About 30 DOM writes per frame (AUDIT P3) fell
  to "0.01 real writes per frame" with change-only setters (CS3). For v2: the HUD keeps change-only writes and is fed
  from snapshot fields and events, never from calls into the sim.
- **GC2** (soldier slots, mark: rewrite in Rust). 12 fixed slots, slot id equal to sim id; perfbots at 48 soldiers:
  combat 0.80–1.00 ms per update step (legacy JavaScript, not Rust). For v2: a stable slot id per entity is what lets
  the page index a flat snapshot with no lookup; the legacy step cost gives a scale for what a worker would take off
  the main thread, not a v2 figure.
- **GA15** (per-frame animation driver). Every soldier fills one state struct per frame (aim yaw, reload progress,
  throw charge, ADS, grounded and more), while aim pitch travels in a side channel through the gun's rotation. For
  v2: the fields presentation needs per soldier must be named fields of the snapshot layout (aim pitch included, for
  VISION_V2 §6.2 "aiming at height"); purely cosmetic state (kick, breathing clock) stays on the client.
- **CS11** (headless SIM fence, mark: redesign). The comment fence leaked: shape data came from the visual recipe and
  `hasLOS` read a global. For v2: the crate boundary is enforced by the compiler, and the boundary module only
  reads what the sim publishes.

### 5. A data-driven ability and effect system (unit `abilities`)

Brief question (quoted): "a **data-driven ability and effect system** in the spirit of UE's Gameplay Ability System"
(`docs/V2_PHASE0_BRIEF.md` §4, row R1).

Method note: quotes marked "verbatim" were returned by WebFetch when it was asked to copy the sentence character for
character; the verifier should re-open them at the source. Claims seen only in a WebFetch summary are marked "inferred".
The official Unreal Engine documentation was not read (outside the allow list), nor was the engine source (private
repository, HTTP 404), so every GAS fact below comes from one community write-up (tranek's GASDocumentation, tier
"other"), which describes GAS as of UE 5.3.

VISION_V2 needs this unit serves: §6.3 (abilities and effects from data, combos and counters emerge, tuning without
code changes), §6.4 (vehicles with their own weapons and abilities: leech, marking), §6.5 (economy and objectives
server-side and tunable: tech tiers gate abilities), §6.6 (telemetry events), §6.7 (faction content as data).

#### GAS concepts (from the community write-up; tier other)
Source for F1–F20: tranek, "GASDocumentation", README.md, https://raw.githubusercontent.com/tranek/GASDocumentation/master/README.md
(the README says it is current with UE 5.3; no page date; read 2026-10-27).

- **R1-ABL-F1.** A GAS attribute holds two values: a permanent BaseValue and a CurrentValue that is the BaseValue plus the
  temporary changes from [active effects](GLOSSARY.md#active-effect-and-visual-effect). · §4.3.2 · other · read
  2026-10-27 · confirmed in the source (verbatim:
  "An `Attribute` is composed of two values - a `BaseValue` and a `CurrentValue`." … "The `BaseValue` is the permanent
  value of the `Attribute` whereas the `CurrentValue` is the `BaseValue` plus temporary modifications from
  `GameplayEffects`.").
- **R1-ABL-F2.** Gameplay effects have three duration types: Instant (a permanent BaseValue change that applies no
  tags), Duration (temporary CurrentValue changes and tags, removed on expiry) and Infinite (never expires by itself). ·
  §4.5.1 (duration table; the unit wrote §4.5) · other · read 2026-10-27 · confirmed in the source (verbatim: Instant
  "For immediate permanent changes to `Attribute's` `BaseValue`. `GameplayTags` will not be applied, not even for a
  frame."; Duration "For temporary changes to `Attribute's` `CurrentValue` and to apply `GameplayTags`"; Infinite "These
  will never expire on their own and must be manually removed by an ability or the `ASC`.").
- **R1-ABL-F3.** Duration and Infinite effects can be periodic: they re-apply their modifiers and executions every
  Period seconds, and periodic effects are not predicted. · §4.5.1, §4.10 · other · read 2026-10-27 · confirmed in the
  source (verbatim: "`Periodic Effects` that apply its `Modifiers` and `Executions` every `X` seconds as defined by its
  `Period`."; "`Periodic Effects` cannot be [predicted](#concepts-p)."; not-predicted list: "GameplayEffect periodic
  effects (dots ticking)").
- **R1-ABL-F4.** Modifiers aggregate as ((base + sum of adds) × multiply) / divide, and by default all Multiply (and all
  Divide) modifiers are summed before they are applied; an Override replaces the result. · §4.5.4, §4.5.4.1 (the
  summing) · other · read 2026-10-27 · confirmed in the source for the formula and the summing (verbatim: "The formula
  for how `Modifiers` are aggregated is defined as follows" `((InlineBaseValue + Additive) * Multiplicitive) /
  Division`; "By default, all `Multiply` and `Divide` `Modifiers` are added together before multiplying or dividing");
  the Override rule is confirmed too (verbatim, verifier 2026-10-28): "Any `Override` `Modifiers` will override the
  final value with the last applied `Modifier` taking precedence."
- **R1-ABL-F5.** Stacking applies only to Duration and Infinite effects, aggregated either per source or per target,
  with policies for expiration, duration refresh and period reset, for which the write-up points to the editor's hover
  tooltips (the unit's "explained only by editor tooltips" overstated the source: unsupported, verifier 2026-10-28). ·
  §4.5.5 · other · read 2026-10-27 · confirmed in the source (verbatim: "Stacking only works for `Duration` and
  `Infinite` `GameplayEffects`."; "There is a separate instance of stacks per Source `ASC` on the Target. Each Source
  can apply X amount of stacks."; "There is only one instance of stacks on the Target regardless of Source."; "Stacks
  also have policies for expiration, duration refresh, and period reset."; they "have helpful hover tooltips in the
  `GameplayEffect` Blueprint.").
- **R1-ABL-F6.** Tags are hierarchical names (Parent.Child.Grandchild), and the ability system counts how many times each
  tag is held. · §4.2 · other · read 2026-10-27 · confirmed in the source (verbatim: "`FGameplayTags` are hierarchical
  names in the form of `Parent.Child.Grandchild...`"; "`GameplayTags` stored in `FGameplayTagCountContainer` have a
  `TagMap` that stores the number of instances").
- **R1-ABL-F7.** An effect can grant tags to its target for as long as it is active, and can carry ongoing tag
  requirements that switch it on or off while it is applied. · §4.5.7 · other · read 2026-10-27 · confirmed in the source
  (verbatim: Granted Tags "Tags that live on the `GameplayEffect` but are also given to the `ASC` that the
  `GameplayEffect` is applied to."; Ongoing Tag Requirements "Once applied, these tags determine whether the
  `GameplayEffect` is on or off.").
- **R1-ABL-F8.** Duration and Infinite effects can grant abilities, with a removal policy when the effect ends. · §4.5.6 ·
  other · read 2026-10-27 · confirmed in the source (verbatim: "`GameplayEffects` can grant new
  [`GameplayAbilities`](#concepts-ga) to `ASCs`."; "Only `Duration` and `Infinite` `GameplayEffects` can grant
  abilities."; removal policies "Cancel Ability Immediately", "Remove Ability on End", "Do Nothing").
- **R1-ABL-F9.** Execution calculations (custom code that computes an effect's outcome) are not predicted and must be
  written in C++; only attribute modifiers predict. · §4.5.12, §4.10 · other · read 2026-10-27 · confirmed in the source
  (verbatim: "The downside to this power and flexibility is that they can not be predicted and they must be implemented
  in C++."; "Attribute modification (EXCEPTIONS: Executions do not currently predict, only attribute modifiers)").
- **R1-ABL-F10.** A cost is an Instant effect, and a cooldown is a Duration effect with no modifiers whose granted tag the
  ability checks. · §4.5.14, §4.5.15 · other · read 2026-10-27 · confirmed in the source (verbatim: "This `Cost GE` should
  be an `Instant` `GameplayEffect`"; "This `Cooldown GE` should be a `Duration` `GameplayEffect` with no `Modifiers`";
  "The `GA` actually checks for the presence of the `Cooldown Tag` instead of the presence of the `Cooldown GE`.").
- **R1-ABL-F11.** Abilities gate and cancel each other through tag sets: required tags (all), blocked tags (any) and
  cancel-with-tag; a stun is built by putting the stun tag in every ability's blocked tags. · §4.6.9, §5.1 · other · read
  2026-10-27 · confirmed in the source (verbatim: "This `GameplayAbility` can only be activated if the owner has **all**
  of these `GameplayTags`."; "This `GameplayAbility` cannot be activated if the owner has **any** of these
  `GameplayTags`."; "Other `GameplayAbilities` that have these `GameplayTags` in their `Ability Tags` will be canceled";
  "the GameplayAbilities are given the stun GameplayTag in their Activation Blocked Tags GameplayTagContainer").
- **R1-ABL-F12.** A "Local Predicted" ability activates on the owning client first, then on the server. · §4.6.8 · other ·
  read 2026-10-27 · confirmed in the source (verbatim: "`Local Predicted` `GameplayAbilities` activate first on the
  owning client and then on the server."). That the server's run corrects the client is inferred (summary only).
- **R1-ABL-F13.** GAS predicts ability activation, triggered events, effect application (attribute modifiers and tag
  changes), cue events, montages and movement; it does not predict effect removal or periodic ticks. · §4.10 · other ·
  read 2026-10-27 · confirmed in the source (verbatim lists: "Ability activation", "Triggered Events", "GameplayEffect
  application:", "Attribute modification (EXCEPTIONS: Executions do not currently predict, only attribute modifiers)",
  "GameplayTag modification", "Gameplay Cue events (both from within predictive gameplay effect and on their own)",
  "Montages", "Movement (built into UE's UCharacterMovement)"; not predicted: "GameplayEffect removal",
  "GameplayEffect periodic effects (dots ticking)").
- **R1-ABL-F14.** GAS ties predicted effects to server confirmation with a prediction key: the client sends the key with
  its activation request, and removes the effects it created under a key once that key comes back from the server;
  predicted effects with no matching server version were mispredicted. · §4.10.1 · other · read 2026-10-27 · confirmed
  in the source (verbatim: "Client sends this prediction key to the server with CallServerTryActivateAbility().";
  "Client removes all GameplayEffects that it created with the now stale replicated prediction key."; "Any
  GameplayEffects that the client added and didn't receive a matching replicated version from the server were
  mispredicted.").
- **R1-ABL-F15.** Paragon and Fortnite (both Epic's shipped games) do not predict damage, and Epic's stated guidance is
  to keep prediction to a minimum. · §4.10 · other (second-hand report of shipped-game practice) · read 2026-10-27 ·
  confirmed in the source (verbatim: "For example, Paragon and Fortnite do not predict damage."; "We still feel player
  prediction is best kept to a minimum"; "predict the minimum amount of stuff you can get away with"). The source
  attributes the last two to "Dave Ratti from Epic's comment from the new Network Prediction Plugin" (verbatim, verifier
  2026-10-28).
- **R1-ABL-F16.** Gameplay cues are for non-gameplay output (sound, particles, camera shake) and are unreliable, so
  unfit for anything that affects gameplay. · §4.8.1, §4.8.9 (the reliability sentence) · other · read 2026-10-27 ·
  confirmed in the source (verbatim: "`GameplayCues` (`GC`) execute non-gameplay related things like sound effects,
  particle effects, camera shakes, etc."; "GameplayCues in general should be considered unreliable and thus unsuited for
  anything that directly affects gameplay.").
- **R1-ABL-F17.** Fortnite tracks weapon fire rates with its own bookkeeping instead of cooldown effects; the answer
  comes in reply to a question about the server overwriting the cooldown of locally predicted abilities, which, the
  answer says, gave Paragon's high-latency players a lower rate of fire. · §11.1.1 "Community Questions 1", question 7 ·
  other · read 2026-10-27 · confirmed in the source (verbatim question start: "Is there a way to stop the Server from
  overwriting the cooldown duration of locally predicted abilities"; answer: "Fortnite does its own bookkeeping for
  weapon firing rates: it does not use GEs for cooldowns on weapons."). The latency context is verbatim too (verifier
  2026-10-28; re-read at the fix step in the verifier's copy of the README): the question, "In scenarios of high
  latency, this would let the Owning Client "try" to activate the ability again when its local cooldown expires but it
  is still on cooldown on the Server."; the answer, "The short answer there is not a way to prevent this and Paragon
  definitely had the problem. Higher latency connections would have a lower ROF with basic attacks."
- **R1-ABL-F18.** The ability's activation prediction key is a 16-bit integer known to both client and server, which the
  write-up offers as a shared random seed. · §5.5 · other · read 2026-10-27 · confirmed in the source for the key
  (verbatim: "is an int16 guaranteed to be synchronized and available in both the client and server"); that it "always
  starts at zero each time the game starts" and that with a seed sent in an event payload "the client could easily hack
  their game to only send the same seed value every time" are confirmed too (verifier 2026-10-28).
- **R1-ABL-F19.** Paragon's slows did not stack: only the strongest slow applied. · §5.7 · other · read 2026-10-27 ·
  confirmed in the source (verbatim: "Slow effects in Paragon did not stack."; "but only the greatest magnitude slow
  effect actually affected the Character").
- **R1-ABL-F20.** Background, no recommendation rests on these (summary only, inferred): GAS replicates effects in three
  modes (Full to all clients; Mixed, owner only, for players; Minimal, none, for AI), with cues still multicast to all
  (§7.3); Fortnite replaces attribute replication on simulated proxies by a proxy struct on the pawn (§7.4); lifesteal
  and critical hits are written inside the damage execution, switched on by tags such as `Effect.CanLifesteal` (§5.4,
  §5.6); abilities run in one frame and use latent "ability tasks" for anything over time (§4.7.1); ability batching
  merges activate, target data and end messages into one, mainly for hitscan guns (§4.6.15); predicted gameplay
  projectiles need extra work (Unreal Tournament's dummy projectile, §4.10.3).

#### A shipped game's weapon code (Quake III Arena)
Source for F21–F22: id Software, Quake III Arena source (GPL release), `code/game/bg_pmove.c`,
https://raw.githubusercontent.com/id-Software/Quake-III-Arena/master/code/game/bg_pmove.c (master; no page date; read
2026-10-27).

- **R1-ABL-F21.** Quake III runs the same player code in both client and server ("both games"): it maps a player state and
  a user command to a new player state, and weapon firing is part of it. · file header and `PM_Weapon` · shipped game ·
  read 2026-10-27 · confirmed in the source (verbatim: "// bg_pmove.c -- both games player movement code"; "// takes a
  playerstate and a usercmd as input and returns a modifed playerstate"; inside PM_Weapon: "PM_AddEvent( EV_FIRE_WEAPON
  );").
- **R1-ABL-F22.** Quake III's weapon cooldown is an integer millisecond counter: it is decremented by the frame's
  milliseconds, firing adds the weapon's interval to it (so an overshoot carries into the next shot while the trigger is
  held), and releasing the trigger resets it to 0; ammo is decremented in the same function; the per-weapon intervals are
  code constants in a switch, not data. · `PM_Weapon` · shipped game · read 2026-10-27 · confirmed in the source
  (verbatim: "if ( pm->ps->weaponTime > 0 ) {" "pm->ps->weaponTime -= pml.msec;" … "if ( pm->ps->weaponTime > 0 ) {"
  "return;" … "// check for fire" "if ( ! (pm->cmd.buttons & BUTTON_ATTACK) ) {" "pm->ps->weaponTime = 0;" … "case
  WP_MACHINEGUN:" "addTime = 100;" … "pm->ps->weaponTime += addTime;"; ammo: "pm->ps->ammo[ pm->ps->weapon ]--;"). The
  carry-over reading of `+=` after the decrement is inferred from the code.

#### Rust ability crates (tier other)
- **R1-ABL-F23.** leafwing_abilities 0.13.0 (released 2026-09-27 per its page) is built for Bevy and covers cooldowns,
  charges and resource pools; damage, cast times, range checking and resource management are listed as planned; its
  cooldown ticks by a `Duration` and is built from `f32` seconds. ·
  https://docs.rs/leafwing_abilities/latest/leafwing_abilities/ (crate root) and …/cooldown/struct.Cooldown.html · other
  · read 2026-10-27 · confirmed in the source (verbatim: "A convenient, well-tested ability management suite. Built for
  the Bevy game engine."; "track and automatically tick cooldowns"; "store multiple charges of abilities"; Planned:
  "resource management (health, mana, energy etc)", "damage", "cast times", "range checking"; `pub fn tick(&mut self,
  delta_time: Duration, charges: Option<&mut Charges>)`; `pub fn from_secs(max_time: f32) -> Cooldown`). Its dependency
  on Bevy "^0.19 (normal)" is confirmed by rust-reviewer (2026-10-28); the page's silence on attributes, effects, tags,
  fixed ticks and networking is inferred (summary only).
- **R1-ABL-F24.** bevy_gauge 0.5.1 (released 2026-07-06 per its page) is an attribute and stat system for Bevy:
  attributes are expressions over other attributes, with tagged modifiers that can be applied and removed as sets. ·
  https://docs.rs/crate/bevy_gauge/latest · other · read 2026-10-27 · confirmed in the source (upgraded by
  rust-reviewer, 2026-10-28: "A flexible attribute and stat system for Bevy"; "Define attributes as expressions, not
  update systems."; "Equipment and buffs are modifier sets that apply and remove cleanly:"). The page's silence on
  duration, stacking, determinism and fixed ticks is inferred.
- **R1-ABL-F25.** bevy_alchemy 0.6.0 models each status effect as its own entity linked to its target, with merge modes
  Stack, Insert and Merge, a Lifetime timer and a repeating Delay timer for periodic effects. ·
  https://docs.rs/crate/bevy_alchemy/latest/source/README.md · other · read 2026-10-27 · confirmed in the source
  (upgraded by rust-reviewer, 2026-10-28: "An experimental, status effects-as-entities system for Bevy."; Stack
  "Multiple of the same effect can exist at once."; Insert "New applications will overwrite the existing one."; Merge
  "New applications are merged with the existing one, using a configurable merge function."; Lifetime "A timer that
  despawns the effect when the timer finishes."); the repeating Delay timer stays inferred.

#### Rust standard library (official)
- **R1-ABL-F26.** Rust's standard `HashMap` is randomly seeded by default and iterates in arbitrary order. ·
  https://doc.rust-lang.org/std/collections/struct.HashMap.html (page shows Rust 1.99.1, 2026-10-13), overview and
  `iter` · official · read 2026-10-27 · confirmed in the source (verbatim: "The algorithm is randomly seeded, and a
  reasonable best-effort is made to generate this seed from a high quality, secure source of randomness provided by the
  host without blocking the program."; "An iterator visiting all key-value pairs in arbitrary order.").

#### Legacy facts used (unit `abilities`)

From `docs/LEGACY_INVENTORY.md` (by row id) and `docs/LEGACY_TUNING.md` / `docs/legacy-tuning.json` (by table). Marks
are working recommendations (R7-MARKS (a)); rows marked "look brief pending" stay pending.

| Fact | Where | What it tells v2 |
|---|---|---|
| Seven weapons as one data table: fire rate, magazine, reserve, pellets, five spread fields, damage, range, recoil, reload, tracer; special fields for falloff (Shabzon 15–35 m, min 0.2), burst (Lancer 3 at 0.06 s), melee arc (Katana 120°, impact 0.13 s, swing 0.4 s) and charge (Torque Bow) | CD12; `WEAPONS` table | The test arsenal. Sim fields and client feel fields (recoil, tracer) are mixed today and must split. Special-case fields per weapon are the pressure that a generic shape must absorb. |
| Four classes as data: weapon, ability, cooldowns, HP 100/150/120/110, speed and sprint multipliers, bot-only fields, a colour | CD13; `UNIT_TYPES` table | The unit definition's test case; bot fields and the colour do not belong in sim unit data. CD13's computed matchups (Warrior kills Conscript in 0.86 s, 10.5 s the other way; Peacekeeper kills Warrior in 0.85 s only if 11 of 16 pellets land) are golden numbers for the spike. Bots fire a separate, weaker model today: v2 needs one weapon model for both. |
| Juggernaut as data (tier 2, cost 2500, 1500 HP, one seat) and its cannon (three 130-damage shells 80 ms apart, 3.4 s cooldown, 130 m) | CD14, GC17; `VEHICLE_TYPES`, `JUG`, `JUG_PILOT`, `JUG_CANNON` | The vehicle-weapon test case. Its HP is never reduced and its shells start at an animated node: in v2 the mount points are sim data. |
| Class abilities: Molotov (a grenade, building ×1.8, fuse 1.4 s), Riot Shield (70 % frontal block within arc cos 0.30, move ×0.72), Banzai (Katana for 10 s, 45 s cooldown, 1.3× speed), Bayonet (1.3 s charge, kills within 2.4 m, 10 s cooldown) | GC11, CD8; `UNIT_TYPES` (banzaiCd/Dur, bayonetCd/Dur), `T` (shield fields) | The ability test cases: a timed equip with a speed buff (Banzai), a toggled stance with a conditional damage reduction (Shield), a throwable (Molotov), a dash with contact kills (Bayonet). LOOP_V2 §4 locks the three T1 abilities until Tech 1; the Bayonet is not in LOOP_V2. Legacy abilities check no tech tier. |
| Damage path: locational multipliers head 2.6 / body 1.0 / legs 0.65 (Bagpipe head 1.2), the shield block (not for blasts), team filter, kill credit | GC7, CS3, CD8; `T`, `PART` | One damage pipeline with fixed stages; the multipliers live twice today (PART unread, T live): one source per number in v2. |
| Frag grenade 140 damage in 6.5 m with linear falloff, owner ×0.5, fuse 1.5 s, buildings 240, max 3; bolt and Molotov share the blast | GC14, CD8; `T`, `T_CHARGE` | Area damage with falloff and owner/team rules as data; blast reach by line of sight is still open (NC-7). |
| Torque Bow: charge-scaled speed, gravity flight with a swept ray, sticks, fuse 0.55 s, building ×1.5 | GC15, GC13; `T_CHARGE.bow` | A fly-stick-fuse projectile primitive (the Javelin needs projectile flight too). Flight integrates with frame dt today, so landing shifts with frame rate. |
| Weapon state: ammo, reserve, switch and fire cooldowns, timed reloads, spread that grows and decays; normal guns have unlimited reserve by a quirk | GC4 | Weapon bookkeeping the server decides; LOOP_V2 §7 adds active reload (faster reload plus a short damage boost). |
| Fire cooldowns are reset, not carried: the Shabzon fires 10.0 / 12.0 / 13.1 shots per second at 30 / 60 / 144 Hz against 13.9 from its 0.072 s table value | GC5 | On a fixed tick the table values hold only if cooldowns carry their remainder (compare F22). |
| Melee hits through walls and ignores height; the arc uses the render object's facing | GC12 | Impact timing and facing must come from sim state; the line-of-sight rule is R4's. |
| Health regen 3.5 s delay, 16 HP/s, player only | GC19 | A candidate periodic effect, but whether v2 keeps regen is Mustafa's decision (LOOP_V2 is silent). |
| Gun sockets and mass are cosmetic; a missing gun key silently falls back to the AMK | CC9 | The feel kit stays client-side; definitions must be validated at load, never silently defaulted. |
| Shield and blade props only show the ability; the shield block flash is broken | CC12 | Presentation follows sim events (cues); nothing animated decides a hit. |
| Input command groundwork (seq, tick, button bitmask) declared but never wired | CS1 | The sequence number is the natural prediction key (see REC4). |
| Hitscan resolution is a pure query returning a small typed hit record | CS7 | Weapons call one hit query and feed its result into the damage pipeline. |
| Era gating and purchase status enum (owned / era / level / poor / buy) | GE4, GE13 | Tech-tier gates become activation and purchase requirements read from team state. |
| AMK reload tiers (5 / 3.5 / 2.5 s) | `RIFLE_TIERS` | Variants of one weapon as data; LOOP_V2 §8 wants sidegrade variants, not raw power. |

### 6. Data formats and hot reload of tuning (unit `data-hotreload`)

Brief question (quoted): "data formats and hot reload of tuning" (docs/V2_PHASE0_BRIEF.md §4, row R1, last item).
Unit scope (from the task): the file format for content and tuning and its tooling; typed loading with serde, validation
at load and a schema the editor and other tools can use; versioning and migration; hot reload in development on the
server and in the Wasm client, and how a running match takes a changed number; how content reaches the browser;
`docs/legacy-tuning.json` as v2's starting values; the consumers (map editor, faction pipeline §6.7, modes as data §6.1,
balance telemetry §6.6); server-side validation of anything a client or editor sends.

Every source read 2026-10-27.
"Verbatim" marks a passage WebFetch returned as an exact quote; a claim seen only in WebFetch's paraphrase is marked
"inferred".

#### serde (typed loading)
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
  informative error", which goes on "but this can be improved by adding an expecting message." (verifier, 2026-10-28)
  · Source: as F1 · official · read 2026-10-27 · confirmed in the source. Internally tagged enums
  (`#[serde(tag = "type")]`) exist as the alternative (same page, confirmed).

#### serde_json (the JSON loader)
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

#### Other text formats
- **R1-DATA-F12.** RON (ron 0.12.2 at the read; docs.rs showed 0.12.3, dated 2026-10-28, a day later) has comments,
  trailing commas and native enums, but is not fully self-describing, supports `tag`, `untagged` and `flatten` only in
  part, and gives no round-trip guarantee through its `Value`. Verbatim: "RON is not designed to be a fully
  self-describing format (unlike JSON)"; "While RON offers a best-effort implementation for `#[serde(flatten)]`, it may
  be unsupported in further cases"; "RON does not yet make any guarantees about roundtrips through `ron::Value`." ·
  Sources: https://github.com/ron-rs/ron (README); https://docs.rs/ron/latest/ron/ (0.12.2, page 2026-09-13) · official
  (project) · read 2026-10-27 · the three quoted limits confirmed in the source; the feature list confirmed by the
  verifier ("trailing commas allowed", "single- and multi-line comments", 2026-10-28); the README's one JavaScript
  implementation ("ron-js", no TypeScript one) comes from WebFetch's summary: inferred; ron-js's maturity: unverified.
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

#### Schemas and validation
- **R1-DATA-F15.** JSON Schema 2020-12 has numeric bounds, enums, required properties, and annotations meant for UI and
  documentation (title, description, default, deprecated, examples). Verbatim: "The value of "minimum" MUST be a number,
  representing an inclusive lower limit for a numeric instance."; deprecated: "applications SHOULD refrain from usage of
  the declared property" · Source: https://datatracker.ietf.org/doc/html/draft-bhutton-json-schema-validation-01 §6.1,
  §6.2, §6.5.3, §9 (deprecated: §9.3) (Internet-Draft, 2020-12 dialect, published 2022-06-10, Informational, expired) ·
  official (specification) · read 2026-10-27 · the quoted keyword definitions confirmed in the source; "meant for UI and
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

#### Hot reload
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

#### Prototype pollution and input validation (the legacy SEC-MDL-2 risk)
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

#### Delivery to the browser
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

#### Legacy facts used (unit `data-hotreload`)

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

## Options

### 1. ECS or plain data (bevy_ecs, hecs, custom) (unit `ecs`)

| | A. bevy_ecs 0.20 (standalone) | B. hecs 0.11.2 | C. Custom plain data (typed tables in one state struct) |
|---|---|---|---|
| **Shape** | A World of archetype tables (F3), systems in schedules, single- or multi-threaded executor (F7). | An archetype World with no scheduler; plain functions query it (F16, F17). | One sim-state struct: a fixed-slot soldier table sized by the mode, generational pools for vehicles, projectiles and effects (F28 shows the key scheme), per-team tables, static colliders with ids, the RNG state, the tick counter, the tick's event list; phases are plain functions called in a fixed order. |
| **Deterministic order** | Not guaranteed (F4, F12); needs a sort in every outcome-deciding system, redone each run (F5); order ambiguities ignored by default (F6). Achievable, but every system has to opt in. | Arbitrary (F17); needs our own ordering (an index or a sort) on every outcome loop. | Storage order is iteration order (F25), and we control it. Still needs a rule against default-hasher HashMap iteration (F23; now the hash-map ban, [GLOSSARY](GLOSSARY.md#the-hash-map-ban-in-the-sim-crate)); BTreeMap where a map is needed (F24). |
| **Whole-state snapshot / restore** | The World is not Clone (F8). The proven route (bevy_ggrs) registers each type, keeps per-frame maps keyed by a rollback id and reconciles entities on load (F10). Unregistered state silently desyncs (F13), and change detection fires after every restore (F14). | The World is not Clone (F18). We would write a snapshot by hand, per component type, or serialize (F20); the freelist makes re-created ids match (F19). | Derive or write Clone over the whole struct; a hand-written clone_from into a preallocated copy reuses buffers (F26, F27; that a derived or default clone_from does not is inferred from F35). Nothing lives outside the struct, so nothing can be "unregistered" (inferred: it holds only if the sim keeps no globals or statics, which is a code rule). |
| **Ids across server and client** | Entity must not be lined up between instances (F9): a separate network id plus MapEntities mapping, as bevy_ggrs does (F10). | Ids can be aligned through the freelist (F19), but only while the live sets match exactly. | The sim defines the id (kind, index, generation, F28), and the same id is used on the wire, in events and in the TS client. |
| **Serialization for snapshots** | An optional serialize feature (F7); Entity's serialized form has no format guarantee (F9). | Context-based serialization with a compact column format (F20). | serde on our own types, the route Rapier (F30) and slotmap (F28, inferred) use; the wire format and deltas are R2's. |
| **Physics (if R4 picks Rapier)** | Rapier keeps its own sets either way (F29); the ECS holds handles and syncs transforms. | Same. | The Rapier sets are fields of the state struct, Clone and Serialize (F29, F30), so they join the same snapshot. |
| **Data-driven abilities** | Effects as entities linked to their target (relationships, F15) or as components; adding and removing components on table storage is the slow path (F3), sparse sets are the escape. | Effects as entities with a target component; no hooks or scheduler. | Effects as rows in an ordered effect pool (owner id, spec id from data, timers, stacks), plus attribute arrays per actor keyed by ids from data. No runtime-defined Rust types needed (inferred). The ability unit designs the rows. |
| **Wasm size, compile time** | Generic-heavy (systems and queries per type, F33); default features pull in reflection and std (F7). Size unmeasured (F34). | Small dependency closure is a stated goal (F16); size unmeasured (F34). | The fewest dependencies, but our own generics still monomorphize (F33); size unmeasured (F34). |
| **Upkeep and risk** | Five 0.x releases in 18 months (F1): a migration roughly each release (inferred). The largest feature set (scheduler, change detection, observers, relationships), most of which no §6 item needs (inferred against VISION_V2 §6). | A smaller API; we write ordering, snapshots and ids ourselves. | The most code we write (pools, ids, ordering, snapshot), but about a dozen kinds known in advance (Legacy facts) keeps it small, and the hecs authors point games with few types this way (F21). We lose ready-made change detection: if R2's deltas need dirty flags, we write them. |
| **Fit with VISION_V2 §6** | Meets §6.1–§6.8 with extra machinery. | Meets them with less machinery, but snapshots and order are on us. | Meets them directly: modes as data size the slot tables (§6.1), Rapier inside the state (§6.2), vehicles and effects as pools (§6.3, §6.4), economy tables (§6.5), the event list feeds telemetry and moments (§6.6, §6.8). |

Effort (inferred, not measured): A has the lowest effort to a first tick and the highest ongoing cost for determinism
and snapshots; B sits in between; C costs the most up front for pools and ids, and the least for snapshots and order.

### 2. A fixed tick rate (unit `tick`)

#### 4a. The tick rate
Arithmetic in this table (periods, per-tick distances, replay counts) is inferred from the findings and tuning above.

| Option | Shipped or official precedent (read) | Fit for SHAMBLE (VISION_V2 §4, §6) | Cost and risk |
|---|---|---|---|
| 20 Hz (50 ms) | Quake III world frame (F9); its player movement was integrated per user command, not per world frame (F12; the command rate is inferred) | Shots and movement quantised to 50 ms; legacy's jump at 20 Hz lost 11 % of its apex (GW14) | Cheapest; coarse input timing; would need movement or shots off the tick (Q3 style), which is the frame-rate dependence legacy and Q3 both fought (F12, GW14) |
| 30 Hz (33.3 ms) | None read for a shooter | Every legacy weapon fits (at most one shot per tick: tuning arithmetic in §3); up to 33 ms extra input wait and up to one tick of render-interpolation lag | About half the CPU of 60 Hz per simulated second (assumes a constant cost per tick); half the replay steps per correction (F24); legacy parity harness would run at a rate legacy never measured |
| 60 Hz (16.7 ms) | Rapier's default `dt` (F8); legacy `FIXED_DT` (CS1) and the legacy harness measurements (TL13, TL15); GW14 was measured at 20, 60 and 144 Hz | Finest of the practical options; one new tick per frame on the most common display rate, 60 Hz (F17; inferred); Rapier step equals the tick | About twice the CPU of 30 Hz (same assumption); may beat against a 60 Hz display when steps are paced by frames (F6); 1/60 is not exact in binary floats (F6) |
| 64 Hz (15.625 ms) | Bevy's default fixed step, chosen against the 60 Hz beat and for an exact float `dt` (F6) | As 60 Hz; every legacy number shifts slightly when converted to ticks | As 60 Hz plus about 7 % more steps; no legacy measurement at this rate |
| 66 Hz (15 ms) | Counter-Strike on Source (F13) | Closest precedent for a competitive shooter | As 64 Hz; no Rust or web precedent read |
| Variable-duration commands (movement per input, world on a slower frame) | Quake III (F12) | Lets a slow world tick keep fine movement | Frame-rate dependent unless chopped (F12); exactly the legacy defect (GW14, GC15); no clean tick to replay or rewind |

#### 4b. How slow systems relate to the tick
| Option | Trade-off |
|---|---|
| Everything every tick | Simplest; cost of AI, sight and route searches scales with the tick rate (F8's "more CPU time per second of simulated time" applies to the whole step) |
| One tick, with per-system cadences in whole ticks (bot thinking, sight budget per game second, route replans, economy, telemetry sampling), staggered by slot | What legacy already does in seconds (GN4); keeps 60 Hz affordable; cadence must be stated in game time so it holds at any tick |

#### 4c. The server loop
| Option | Trade-off |
|---|---|
| Timer period as the clock (`tokio::time::interval`, default `Burst`) | Simple; timers are millisecond-granular (coarser on Windows, F23) so 16.67 ms cannot be hit exactly; `Burst` catches up without a bound (F23) |
| Monotonic clock accumulator: wake on a timer, compute due ticks from elapsed time (Q3's `timeResidual`, F10; Fiedler's accumulator, F2), bounded catch-up | Average rate exact regardless of timer granularity; needs an explicit overrun policy (F3, F7) |
| Integer-millisecond period (Q3's `1000 / sv_fps`, F10) | Simple integers, but 60 Hz becomes 62.5 Hz (inferred); avoid |

#### 4d. The client loop (Worker sim, main-thread render)
| Option | Trade-off |
|---|---|
| W1. Main-thread `requestAnimationFrame` posts a frame message; the Worker steps its accumulator | Display-synced; rate varies 60–144 Hz and stops in background tabs (F17); a main-thread hitch stalls the sim |
| W2. Worker self-paced by `setTimeout` with a `performance.now()` accumulator | Independent of the main thread (F15); 100 µs clock resolution is ample (F20); timers fire late under load and their hidden-tab behaviour in workers is not stated in the sources read (F18, F19) |
| W3. Worker blocking loop on `Atomics.wait` with a timeout, woken early by `Atomics.notify` | Precise wake, lowest input latency; needs `SharedArrayBuffer` (cross-origin isolation, boundary unit) (F22); while blocked the Worker's own message queue is not served (inferred from "It is blocking"), so inputs must arrive through shared memory |
| W4. Worker `requestAnimationFrame` | Display-synced inside the Worker (F17); same rate variation and background pause as W1 |
Rendering stays on the main thread's `requestAnimationFrame` in every option and interpolates between the two newest
ticks (F4).

### 3. Identical results between Wasm and native (unit `determinism`)

#### Q1. How much determinism, and of what

| Option | What it is | Complexity and effort | Performance | Risk | Fit with VISION_V2 §6 and the netcode |
|---|---|---|---|---|---|
| A. None | std floats and math, `HashMap` iteration, `StdRng` | lowest | best | tests irreproducible, the legacy TL13 problem again; more prediction corrections | server authority still works (F32), but balance runs (§6.6) and golden tests cannot be replayed |
| B. Same build only | seeded portable RNG, fixed tick, stable order; any float maths | low | near best | the browser's predicted results drift from the server whenever std maths differ (F9, F16) | reproducible server runs for §6.6 and tests; prediction corrected often where transcendental maths feed movement or aim |
| C. Cross-target by discipline | B plus: transcendental maths only from the pure-Rust `libm`, no relaxed SIMD or fast-math, NaN-free state, Rapier `enhanced-determinism`, proven by per-tick state hashes | medium (rules plus a test suite) | cost of `libm` and Rapier's flag unknown (F24) | an unknown divergence (F22's 4-lane path) is caught by the test, and the netcode absorbs it (F32) | serves prediction, offline play and tests with one code path; no lockstep needed |
| D. Fixed point | integers for all sim maths | high: own physics, own trig (F29) | unknown | Rapier unusable (F29); large rewrite of every query | bit-exact by construction, but VISION needs a true 3D world with vehicles (§6.2, §6.4), which Rapier could provide (R4 decides) |
| E. Lockstep-grade | C or D, and the netcode depends on bit-identity | highest | n/a | one divergence desyncs the match (F32) | not the brief's model (server authority, `docs/V2_PHASE0_BRIEF.md` §0) |

#### Q2. Transcendental functions (sin, cos, atan2, exp, pow)

| Option | Trade-offs |
|---|---|
| std methods | fastest to write; precision "varies by platform, Rust version" (F9); on Wasm they come from the bundled libm, on Linux from the system libm (F16): different code on client and server |
| `libm` crate, called explicitly | the same pure-Rust code on both targets (F17); no documented accuracy bound (F18); speed vs std not measured |
| nalgebra `ComplexField` / `RealField` with simba `libm_force` | what Rapier asks for (F20, F22); consistent with Rapier's own maths; ties the sim's maths to nalgebra's traits |
| Own approximations or tables | full control and identical by construction if written in exact ops (F10); effort and accuracy are ours to prove |

#### Q3. Rapier

| Option | Trade-offs |
|---|---|
| Default features | locally deterministic only (F19); two machines "may result in completely different results" |
| `enhanced-determinism` | cross-platform on IEEE targets including Wasm (F20); forces libm and scalar glamx maths (F22, F23); cost not documented (F24); not with `simd8` (F21) |
| `enhanced-determinism`, Wasm built without `simd128` | removes one unknown (F22's 4-lane path on Wasm SIMD) until a test clears it; possible speed loss, not measured |
| Custom physics | full control, no Rapier; R4's topic |

#### Q4. Random numbers

| Option | Trade-offs |
|---|---|
| `StdRng` / thread RNG | "Non-portable", may change algorithm (F27): excluded |
| `rand_chacha` ChaCha8Rng | portable, reference-tested (F28); 2^64 streams per seed and a settable position, so the RNG state is small and can be saved in a snapshot (inferred); speed not measured |
| `rand_pcg` Pcg32 | portable, reference-tested (F28); a jump-ahead `advance` and "a good choice on both 32-bit and 64-bit CPUs" (F34); a stream number in its constructor, but "two generators with different stream parameters may be closely correlated" (F34), so stream numbers alone may not keep purposes apart (R1-DET-OQ11); `Pcg64Mcg` slow on 32-bit CPUs (F28; whether wasm32 counts is not stated) |
| Port the legacy `_mulberry32` | tiny, but no source read on its quality; legacy uses it only for the editor (CM9) |

#### Q5. Iteration order

| Option | Trade-offs |
|---|---|
| std `HashMap` iterated | order arbitrary (F25), with a seed per instance (F33): excluded for anything that changes outcomes |
| `HashMap` with a fixed hasher | order repeatable for one build, but no source read says it is the same on 32-bit Wasm and 64-bit native; loses HashDoS resistance (F25), which matters for keys a client controls (inferred) |
| `BTreeMap` | sorted order (F26); slower lookups than a hash map (not measured here) |
| Dense arrays or slot maps by stable entity id | order is the id order; fits the ECS-or-plain-data unit's choice |

### 4. The Rust ↔ TypeScript boundary, and the simulation in a Web Worker (unit `boundary-worker`)

#### Q1. Where the client simulation runs

| Option | What it is | Complexity and effort | Performance and risk | Fit with VISION_V2 |
|---|---|---|---|---|
| A1 Main thread | Wasm sim instantiated on the page, stepped inside the render loop | Lowest: no worker, direct calls | The tick competes with rendering and DOM; a long tick (prediction replay, many bots) drops frames; legacy is this shape (GP13, GP14) | Weak for §6.9 spectacle budget and §4's 30+ fps minimum |
| A2 One dedicated module worker for the sim | Worker owns the Wasm module; page keeps input, HUD and rendering | One boundary; module workers in all four browsers (F25); Vite bundles it (F33) | Sim step time ([GLOSSARY](GLOSSARY.md#sim-step-time)) leaves the render thread; costs one thread and a per-tick hand-off | Serves §6.2, §6.9, §4 |
| A3 A2 plus rendering in another worker | The renderer also leaves the page (OffscreenCanvas) | Highest; R3's question | Page keeps only DOM; not assessed here | R3 decides; not needed to answer this unit |

#### Q2. How snapshots and events leave the sim

| Option | Copies per tick | Isolation needed | Toolchain | Main risks |
|---|---|---|---|---|
| B1 Structured clone of JS objects | Every field, plus per-entity object building in the worker (F5's per-field cost) | No | Stable | Allocation and per-entity work every tick: breaks the "no call per entity" rule |
| B2 One flat buffer, transferred from a pool | One block copy in the worker from linear memory into a pooled ArrayBuffer; the transfer itself is zero-copy (F15) | No | Stable Rust, no Wasm threads | Delivery waits for the page's message queue (inferred, to be measured); the pool must recycle buffers (the sender loses each one, F15) |
| B3 One flat buffer in a SharedArrayBuffer ring | One block copy into a ring slot; the page reads the newest complete slot with Atomics (F13) | Yes (F12, F20); Safari only with `require-corp` (F22) | Stable Rust | Torn reads unless slots are published with Atomics; events need their own lossless ring; isolation costs (F21, F23) |
| B4 Page reads the sim's own shared Wasm memory | None | Yes | Nightly Rust, atomics, rebuilt std, `web`/`no-modules` output only (F18); fixed maximum memory (F17) | Most brittle (F18's caveats); views go stale on growth (F16); the sim must double-buffer its output to avoid torn reads |

Inside the buffer, two layouts are possible: records per entity (`#[repr(C)]` structs, F10) or sections per field
(all positions in one `f32` run, and so on). Either way each section starts at an offset aligned to its element size
(F11), and a section header says how many live entries each kind has.

#### Q3. How inputs go in
- **C1 A small message per tick** carrying the latest command (seq, tick, move, aim, held buttons, edge counters).
  Works without isolation; one message per tick.
- **C2 An input slot in a SharedArrayBuffer**, written by the page and read by the worker with Atomics. Needs
  isolation; saves a message.
Either way, one-shot actions travel as edge counters (a count of presses and releases), so a duplicated or late
command neither loses nor doubles a press (GP2).

#### Q4. Generated TypeScript types

| Option | Covers | Runtime cost | Maintenance | Fit |
|---|---|---|---|---|
| D1 wasm-bindgen's own `.d.ts` | The exported control functions and classes (create, load content, step, pointer and length of the output block) | Calls only, a constant number per tick | Default output (F3); CLI must match the library (F2) | Needed whatever else is chosen |
| D2 tsify | Serde types as TS types, values passed by serde | Per-field calls (F5) | Two forks, status unclear (F8) | Fine for rare messages, wrong for per-tick state |
| D3 ts-rs | Any Rust type as a TS declaration, written at `cargo test` | None (types only) | One crate, 12.0.1 (F9) | Message, protocol and content-schema types shared by sim, server, client and tools |
| D4 Own layout generator | The flat snapshot and event layout: offsets, strides, element types and a layout hash, emitted as a TS module or through `typescript_custom_section` (F4) | None (types and constants) | Ours to keep; small | The only option that covers the bulk buffer |

#### Q5. Wasm threads in the client sim
- **E1 None:** one single-threaded sim in one worker, stable Rust, the default target (F19).
- **E2 Threads** (a thread pool across workers): needs nightly, atomics, a rebuilt std and isolation (F18, F12);
  `std::thread` stays unusable (F18, F19). No VISION_V2 §6 item asks the client to split one tick across cores.

#### Q6. Cross-origin isolation
- **G1 Not isolated:** B2 and C1 work everywhere; no header or embed constraints; timers at 100 µs (F30).
- **G2 Isolated in production:** enables B3/C2; COOP `same-origin` + COEP `require-corp` (Safari, F22); every
  cross-origin resource loaded in `no-cors` mode needs CORP and frames must be isolated too (F21; CORS as another route
  is our reading, not in F21's quotes: second fix round); cross-origin popups lose their opener (F23).
- **G3 Isolated in development and previews only,** for 5 µs timers and the memory API in profiling (F20, F30),
  while production runs G1. The boundary code is the same either way if B2/C1 is chosen.

#### Q7. Who owns the network connection (shared with R2)
- **H1 The page** receives packets and forwards them to the worker: two hops per packet, and a busy frame delays
  server snapshots and prediction.
- **H2 The worker** opens WebTransport or WebSocket itself (F29): server snapshots, prediction and reconciliation
  never touch the page; the page only sees the presentation snapshot.

### 5. A data-driven ability and effect system (unit `abilities`)

#### Question A: how abilities and effects are expressed

| Option | What it is | Complexity and effort | Performance | Risk | Fit with VISION_V2 §6 |
|---|---|---|---|---|---|
| A1. Full GAS model | Attributes, all effect types, stacking policies, execution hooks, ability tasks, instancing policies, replication modes, prediction keys per effect | Highest: GAS's own write-up needs 11 concept sections (F1–F20) and several concepts exist for Unreal's object and replication model (instancing policy, replication modes, F20) | Unknown here | Builds generic-engine features "for later" (CLAUDE.md: REVISE); executions are unpredictable code anyway (F9) | Over-serves §6.3 |
| A2. Reduced GAS subset as data, with a closed set of operations written in Rust | Attributes (base and current, F1), effects (instant, duration, infinite, periodic; F2–F3), counted hierarchical tags (F6–F7), abilities with tag gates, costs and cooldowns (F10–F11), effect-granted abilities (F8), cues as events (F16). Content picks and parameterizes operations; it cannot add new code paths | Medium: one generic interpreter over plain data; every new operation is a reviewed Rust change | Bounded by data size; no script VM; testable for zero allocation | A needed mechanic may lack an operation (then it is a Rust change, which is visible and reviewed) | Serves §6.3, §6.4, §6.5, §6.7 directly; each operation traces to a legacy or LOOP_V2 test case |
| A3. Embedded scripting language for abilities | Abilities as scripts (Overwatch's Statescript is the shipped example, but the talk was not read: OQ2) | High: a second language and runtime in both the Wasm and native builds | Not researched | Determinism and Wasm support of a script runtime not researched (OQ10) | Serves §6.3 but adds a runtime no §6 item asks for |
| A4. Hard-coded Rust per ability, numbers in tables | The legacy shape (CD12, CD13, GC11): each ability is code, only tuning is data | Low at first, grows per ability | Best | Combos and counters need code per pairing; content work blocks on programmers | Meets "tuning without code changes" but not "built from data" (§6.3) |
| A5. Adopt Bevy crates (leafwing_abilities, bevy_gauge, bevy_alchemy) | Assemble from existing pieces | Medium, plus glue | Not measured | Each is Bevy-bound (F23–F25), covers one slice (cooldowns, stats or status effects), and leafwing's cooldowns tick by `Duration` from `f32` seconds (F23); none documents fixed ticks, determinism or prediction (inferred) | Partial; ties this unit to the ECS unit's choice |

#### Question B: weapons on the effect system or beside it
- **B1. Weapons keep their own bookkeeping** (fire timer, ammo, burst, charge, reload, spread) as a small state machine
  driven by the weapon definition, and send hits into the shared damage pipeline as instant effects. Fits F17 (Fortnite)
  and F22 (Quake III); keeps per-shot cost low; one more mechanism to learn.
- **B2. Weapons as abilities with cooldown effects per shot** (F10's cooldown pattern for every shot). One mechanism,
  but F17's question points at the predicted-cooldown problem this creates, and per-shot effect instances would add
  bookkeeping at up to 13.9 shots per second per soldier (CD12 Shabzon; the cost is inferred, not measured).

#### Question C: the time base for durations, periods and cooldowns
- **C1. Integer time on the fixed tick**: durations converted from authored seconds at load into an integer unit in which
  the tick step is exact, decremented per tick, with the remainder carried (F22's `+=`). Same result on every machine
  and frame rate; needs a load-time conversion and a rounding report.
- **C2. Floating-point seconds advanced by a delta** (leafwing's `Duration` and `f32`, F23; legacy's frame dt, GC5,
  GC15). Simple, but legacy shows the rate drifting with the frame rate (GC5) and bit-identical results on Wasm and
  native are the determinism unit's question.

#### Question D: what the client predicts
- **D1. Nothing** (server only): simplest; every ability press waits a round trip, against LOOP_V2 §7's "the shot feels
  instant".
- **D2. The local player's own activation, cost, cooldown, tags and movement changes, plus cues; never damage, kills or
  effects on other players.** Matches Epic's practice (F13, F15) and LOOP_V2 §7's split feedback (instant shot, hit
  feedback on confirmation).
- **D3. Everything, damage included.** F15: Epic's own games avoid it, and the write-up warns of health jumping back.

#### Question E: the shape of the definitions
- **E1. One table per kind** (unit, vehicle, weapon, ability, effect, tag registry, feel kit), linked by string ids that
  are resolved to dense integer handles at load and validated (an unknown id is a load error). Matches the legacy
  tables (CD12–CD14) and fixes CC9's silent fallback.
- **E2. Nested, self-contained definitions** (each unit embeds its weapons and abilities). Simpler to read; duplicates
  shared content (the three T1 classes are shared by every faction, LOOP_V2 §5) and blocks reuse across factions
  (§6.7).

### 6. Data formats and hot reload of tuning (unit `data-hotreload`)

#### Q1. The file format
| Option | Complexity and effort | Performance | Risk | Fit with VISION_V2 §6 and the editor |
|---|---|---|---|---|
| A. JSON for everything (content, tuning, maps, modes), one canonical writer | Lowest: serde_json on the Rust side, `JSON.parse` in the browser and editor, no second parser | Parse cost measured in the spike; exact floats need `float_roundtrip` at about 2x float-parse cost (F6) | No comments in files (documentation goes into the schema's descriptions, F15, F17); big integers lose precision in JS (F26) | EDITOR_V2 rule 1 already chose JSON for maps; legacy-tuning.json is JSON; JSON Schema tooling on both sides (F16, F19) |
| B. RON for hand-written content, JSON for editor maps | Two formats, two writers; the TypeScript editor has no first-class RON parser (F12: one JS implementation listed, maturity unread), so it would parse through the Wasm loader | Unknown, not measured | Partial `tag`/`flatten` support and no `Value` round-trip guarantee (F12) clash with migrations on a tree (F10) | Nice for Rust authors; weaker for the editor and for tools |
| C. TOML for flat tuning tables, JSON for the rest | Two formats; TOML's arrays of tables are awkward for nested definitions | Not measured | No null (F13); config-oriented (F13) | Good for a few flat tables, poor for units, abilities and maps |
| D. Text for authoring, compiled to a binary runtime format | A build step and a second schema; formats not researched in this run | Possibly faster to load; not measured | Unread sources: no claim here (open question R1-DATA-OQ2) | Only worth it if the spike shows JSON loading too slow |
YAML is not an option: the serde YAML crate is archived (F14).

#### Q2. Typed loading and validation
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

#### Q3. Versioning and migration
- **A. A version per file, migrations on the untyped tree:** read `version` first, run step functions N → N+1 on a
  `serde_json::Value` (F10), then the typed load. Additive changes use `default` and `alias` without a version bump
  (F4). Clear and testable with fixtures.
- **B. Versioned Rust types side by side (V1, V2 … structs, converted with `From`):** type-safe but multiplies types
  with each change; heavy for many small content files.
- **C. No versions, best effort:** the legacy path (GW12: `version:1` written, never read). Ruled out by EDITOR_V2 §4.

#### Q4. Hot reload, and how a running match takes a changed number
- **A. Development only, swapped at a tick boundary, server first:** the server watches the content folder (notify
  with a debouncer, F20), re-validates the whole set, and on success swaps an immutable content set between two ticks;
  on failure keeps the old set and reports the error with file, line, column and field path (F7, F8). Clients in the
  match get the new set from the server with "applies from tick T". Offline play in the browser gets it from the Vite
  dev server's push (F22) and swaps at its own next tick. Production pins one content set per match.
- **B. Live tuning in production too:** the same mechanism switched on for live servers. Risk: matches whose rules
  change mid-way make telemetry (§6.6) and fairness harder; shipped-game practice was not read (OQ3).
- **C. No hot reload; restart the match on any change:** simplest, but works against LOOP_V2's "tuned without code
  changes" iteration and the decided Feel Lab.

#### Q5. How content reaches the browser
- **A. Embedded in the Wasm binary (F28):** always consistent with the code; every tuning change means a rebuild and a
  bigger download.
- **B. A content bundle with its hash in the URL, cached as immutable (F27), its hash checked against the one the
  server names at join (F29):** cacheable, independent of code releases, verifiable.
- **C. Sent by the server over the game connection at join:** always matches the server; costs bandwidth at every join
  and caches nothing. Suits small things: private-match overrides, dev reloads.

## Recommendations

Every pass mark and every reference machine below is proposed. Each recommendation keeps its unit's reason,
its "Rests on:" list and the VISION_V2 items it serves.

### 1. ECS or plain data (bevy_ecs, hecs, custom) (unit `ecs`)

- **R1-ECS-REC1. Custom plain data (option C): one sim-state struct of typed tables, phases as plain functions called
  in a fixed order, no ECS crate.**
  The reason:
  - The entity kinds are few and known (about a dozen; Legacy facts), which is the case the hecs authors say suits
    plain Vecs (F21).
  - Both ECS crates leave iteration order unspecified (F4, F17), so determinism would need sorting or our own ordering
    in every outcome loop anyway (F5, F12). Plain tables iterate in the order we store (F25).
  - The client's replay needs a cheap whole-state snapshot and restore. Neither World is Clone (F8, F18). The bevy route
    (bevy_ggrs, a third-party rollback library, not bevy_ecs itself) is per-type registration with silent-desync and
    change-detection traps (F10, F13, F14). A plain struct can be cloned into a preallocated copy that reuses its
    buffers if its clone_from is written by hand: Vec's clone_from reuses its buffer (F26), while the default clone_from
    is "equivalent to `a = b.clone()`" (F27, F35), so a derived or default one does not reuse buffers (inferred from
    F35; stated flatly until the second fix round). hecs narrows the gap (its ids can be made to match, F19, and it has
    a column serializer, F20), but its World is not Clone either (F18).
  - Server and client need one shared id, which bevy's Entity is documented not to be (F9). The sim owning generational
    ids (F28) also gives static colliders the stable ids legacy lacked (CS7).
  - Rapier's sets are Clone and Serialize and slot straight into the struct (F29, F30).
  - It avoids features no §6 item needs (scheduler, observers, reflection) and a second fast-moving 0.x dependency
    beside Rapier, which is 0.x too (0.30.1 in January 2026, F32; 0.36.1 now, F29); that each bevy release breaks the
    API is inferred (F1).

  Rests on: F1, F4, F5, F8, F9, F10, F12, F13, F14, F17, F18, F19, F20, F21, F25, F26, F27, F28, F29, F30, F32, F35
  (plus F7 and F33 for the size marks; F34, an absence no quote can confirm, only says why mark 5 measures); the legacy
  facts (about a dozen kinds; CS7; GC2 for mark 1); VISION_V2 §4 for mark 3; R2's worked bandwidth estimate for mark
  1's 15 big vehicles (an R2 assumption, through the stress scene).
  Serves VISION_V2 §6.1, §6.2, §6.3, §6.4, §6.5, §6.6 and §6.8, and §4 ("matches feel fair up to about 100 ms of
  ping", which needs reproducible replay).

  A spike must prove (all marks proposed; reference hardware proposed:
  [one vCPU](GLOSSARY.md#the-measuring-unit-one-vcpu) of the reference server
  ([GLOSSARY](GLOSSARY.md#the-reference-server): the CCX23 of R6-HOST-REC2), and the VISION_V2 §4 minimum laptop for
  Wasm):
  1. **Sim step time.** The stress scene ([GLOSSARY](GLOSSARY.md#the-stress-scene); proposed, not a design target; the
     game-designer's caps replace its counts, R2-BUD-OQ5):
     [the largest mode's N](GLOSSARY.md#the-largest-mode-and-its-n) soldiers (bots), 15 big vehicles, 300 [live projectiles](GLOSSARY.md#live-projectile) and up to 8 active effects
     per soldier, on a 3D greybox. The pass mark: a p99 [sim step time](GLOSSARY.md#sim-step-time) within the sim step
     share ([GLOSSARY](GLOSSARY.md#the-sim-step-share-and-the-server-only-share): 25 % of the
     [tick period](GLOSSARY.md#tick-period), 4.2 ms at 60 Hz and 8.3 ms at 30 Hz, the mark of R1-TICK-REC2 P1), the
     mean reported, native; Wasm in a Worker ≤ 2× native.
     For reference only, the legacy JS step at 48 soldiers measured 0.80–1.00 ms mean (GC2), without physics or a 3D
     world.
  2. **Snapshot.** clone_from of the whole state into a preallocated copy, with a hand-written clone_from for our own
     tables (F35): 0 heap allocations after warm-up in our tables (counted by a counting global allocator). Rapier's
     sets keep the default clone_from (F35), so their allocations per snapshot are counted and reported separately,
     and a workaround is measured if they matter. The whole snapshot takes ≤ 0.2 ms native and ≤ 0.5 ms Wasm at p99
     ([GLOSSARY](GLOSSARY.md#time-statistics)) in the stress scene. (The unit's "0 heap allocations, Rapier included"
     was not reachable as written: rust-reviewer risk 5, verifier ECS finding 3; changed at the fix step.)
  3. **Restore and replay.** Restore, plus re-simulating k = ceil(0.15 s × tick rate) ticks (VISION_V2 §4's 100 ms,
     plus a proposed 50 ms margin; the tick rate is the tick unit's), fits in ≤ 8 ms at p99 in Wasm on the minimum
     laptop. If it doesn't, whole-world replay is ruled out and R2's prediction scope
     ([GLOSSARY](GLOSSARY.md#the-prediction-scope)) applies (R1-ECS-OQ3).
  4. **Snapshot fidelity.** A run that is snapshotted, restored and continued is byte-identical (serialized
     [state hash](GLOSSARY.md#state-hash) each tick) to an uninterrupted run over a 10-minute bot match, and serialize
     → deserialize → serialize is
     byte-identical.
  5. **Size and build.** The sim .wasm, with and without Rapier, after wasm-opt and gzip, is recorded and is smaller
     than a stub of the same tick on bevy_ecs 0.20 with default features off (one build each, F33; F34 found no figure
     to cite). An incremental native rebuild after a one-line change to a phase takes ≤ 10 s on the dev machine.

  Cross-topic open questions: R1-XT-OQ2, R1-XT-OQ7, R1-XT-OQ18.

- **R1-ECS-REC2. Determinism rules for the layout, whatever option the ADR picks.**
  Every outcome-deciding loop iterates in a defined order: slot or pool index order, or a sort by a stable key. The
  sim uses no std HashMap or HashSet (R5-RUSTLINT-REC2 bans the types) (BTreeMap or an index-ordered Vec instead;
  [GLOSSARY](GLOSSARY.md#the-hash-map-ban-in-the-sim-crate): the hash-map ban). The sim's RNG
  state and tick counter live inside the snapshotted state. Tick phases are an explicit, fixed sequence.

  The reason: the default HashMap is seeded per instance and iterates arbitrarily (F23); BTreeMap and slices have
  defined orders (F24, F25); ECS queries don't (F4, F17); and the legacy step order decided outcomes while it shared
  its RNG with FX and audio (GP14).

  Rests on: F4, F12, F17, F23, F24, F25; legacy GP14.
  Serves VISION_V2 §4 (fair to about 100 ms of ping: prediction and replay need reproducible ticks) and §6.6 and
  §6.8 (server-side telemetry and moments are computed from reproducible state).

  A spike must prove (proposed):
  1. Two processes (different HashMap seeds) fed the same seed and inputs give identical per-tick state hashes over a
     10-minute bot match.
  2. R5-RUSTLINT-REC2's type ban finds zero std HashMap or HashSet in the sim crate outside reviewed `#[allow]`s
     that give their reason (the tooling is R5's).
  3. Under REC1's fidelity test, the RNG continues identically after a restore.

- **R1-ECS-REC3. The sim defines entity identity: one id (kind, index, generation) for every entity, static colliders
  included, used on the server, by the client predictor, in snapshots, in events and in the TypeScript client; never
  an ECS- or library-internal id on the wire.**
  The reason: bevy documents that its Entity must not be lined up across instances (F9), and rollback on bevy needs
  its own stable id (F10). Generational keys reject stale handles after a slot is reused (F28), and Rapier's handles
  work the same way (F29). Legacy can't name the static a shot hit (CS7), and its bolts follow a victim's mesh position
  (GC15).

  Rests on: F9, F10, F28, F29; legacy CS7, GC15.
  Serves VISION_V2 §6.3 (effects name their source and target), §6.4 (vehicle parts and seats), §6.6 (telemetry names
  units) and §6.8 (moments name who did what).

  A spike must prove (proposed):
  1. Ids survive snapshot and restore unchanged.
  2. A despawned id never resolves after its slot is reused (a property test over random spawn and despawn sequences,
     10,000 cases).
  3. A server snapshot applied on the client maps every id with no lookup table beyond the pools themselves.

  Cross-topic open questions: R1-XT-OQ11.

- **R1-ECS-REC4. If R4 picks Rapier, its sets are fields of the sim state and go into every snapshot and serialized
  state; the snapshot test covers the physics state byte for byte.**
  The reason: Rapier's sets are Clone and Serialize (F29); its full state is serializable, with the `serde-serialize`
  feature, while its pipelines hold no state (F30); and a reported case showed a library skipping internal state on
  serialize, which broke restored worlds (F32).

  Rests on: F29, F30, F32 (F31 for the cross-machine part, which is the determinism unit's).
  Serves VISION_V2 §6.2 (a true 3D simulation) and §6.4 (big vehicles as physical bodies).

  A spike must prove (proposed): REC1 mark 4 with Rapier active, including a cloned (not serialized) restore, on the
  Rapier version the ADR pins; any divergence is a blocker for that version.

  Cross-topic open questions: R1-XT-OQ2.

### 2. A fixed tick rate (unit `tick`)

**R1-TICK-REC1. One fixed tick, counted in integers, the same constant on server and client; durations are converted
to ticks at load, with remainders carried.**
The simulation advances only in whole ticks of one build-time constant `dt`; a `u32` tick counter is the sim's only
clock (sim time = tick × `dt`); the step takes no time argument and reads no wall clock. Content data stays authored in
seconds (LOOP_V2: numbers in one table) and is converted to ticks when loaded (as Source does, F14). Cooldowns and
fire intervals keep their fractional remainder (the next allowed shot is scheduled from the previous scheduled time,
not from the tick it landed on), so a 0.072 s weapon fires at the same average rate at any tick rate. The tick rate is
not a server option (Valve removed `-tickrate` for its shooters, F13).
Reason: variable steps change outcomes (F1, F26), which legacy measured (GW14 jump reach, GC15 bolt landing), and a
fixed step is the precondition for reproducible runs (F5) and for netcode, replays and parity tests.
Rests on: F1, F2, F5, F13, F14, F26; legacy CS1, GP14, GW14, GC15, KT13, `WEAPONS`.
Serves: VISION_V2 §6.2 (jumps and ledges at height), §6.3 (abilities and effects built from data with timed
durations), §6.5 (economy and objectives server-side, tunable), §6.8 (moments; [LATER] replays not designed out).
A spike must prove (proposed pass marks):
- With held fire for 60 s of game time, every legacy `fireInterval` and `burstInterval` from docs/legacy-tuning.json
  gives the same shot count at 30 Hz and at 60 Hz, within 1 shot, and within 1 shot of 60 / interval.
- The same seed and the same input log, run twice on the same build, give the same state hash at every tick for
  36,000 ticks (identity across Wasm and native is the determinism unit's test).
- A CI check shows the sim crate's step reads no clock (no `std::time`, no host time import in the Wasm module).

**R1-TICK-REC2. Start the spike at 60 Hz, measure 30 Hz beside it on the same scene, and choose by the pass marks.**
60 Hz is the starting candidate: it is Rapier's default step (F8), legacy's declared step (CS1) and the rate of the
legacy harness measurements (TL13, TL15; GW14 was measured at 20, 60 and 144 Hz and GN4 at 60 and 144 fps), and it keeps
player movement and shots on a fine fixed step. Quake III did not integrate movement on its 20 Hz world frame but per
user command, and still had to chop long moves "to prevent framerate dependent behavior" and add an optional fixed 8 ms
movement step (F12); a single fine tick avoids both problems. Shipped shooters read span 20 Hz (F9) to 66 Hz (F13). 30
Hz roughly halves the CPU per simulated second (inferred: F8 gives only the direction, and halving assumes a constant
cost per tick, which REC3's cadences change) and the replay work per correction (F24), and every legacy weapon still
fits one shot per tick (tuning arithmetic), so it is the measured fallback. 64 Hz stays an open question (OQ1).
Rests on: F3, F6, F8, F9, F12, F13, F24; legacy CS1, GN4, GW14, TL13, TL15, `WEAPONS`, `T`.
Serves: VISION_V2 §4 (fair up to about 100 ms ping; 30+ fps minimum, 60 fps recommended), §3 (up to 24v24, with bots
filling empty slots; the unit had glossed §6.1 this way, corrected at the second fix round: §6.1 is modes as data), §6.2
(aiming and jumping at height), §6.4 (big vehicles: physics steps with the tick).
A spike must prove (proposed pass marks; reference hardware proposed: the reference server
([GLOSSARY](GLOSSARY.md#the-reference-server): R6-HOST-REC2's CCX23) and R3-BUDGET-REC1's minimum laptop):
- P1 server, native: the stress scene ([GLOSSARY](GLOSSARY.md#the-stress-scene)) at N = 12 / 24 / 36 / 48 / 64
  soldiers and at the largest mode's N where it is not among them (MODES-3) (bots with full AI, worst case: all in one
  combat cluster), with its 15 big vehicles, 300 live projectiles and up to 8 active effects per soldier, on a
  DM_Tri-sized greybox, 36,000 ticks per run, 3 runs. At 60 Hz: p99 sim step time at the largest mode's N at most
  4.2 ms (the sim step share, [GLOSSARY](GLOSSARY.md#the-sim-step-share-and-the-server-only-share): 25 % of 16.7 ms)
  on one vCPU of the reference server, max at most 16.7 ms, zero overruns. At 30 Hz: p99 at most 8.3 ms (25 % of
  33.3 ms). Report mean, p50, p95, p99 and max per N. The other 25 % of the tick period, inside the server tick budget
  ([GLOSSARY](GLOSSARY.md#the-server-tick-budget): 50 % of the tick period), is the server-only share, for snapshot
  encoding, interest management, lag compensation and commands (R2-SYNC-REC9); how many matches share a vCPU is
  measured, not budgeted (R2-ROOM-REC2 mark 2); headroom itself is Fiedler's advice (F3).
- P2 client, Wasm in a dedicated Worker on the proposed minimum laptop, offline at the largest mode (OQ7 decides N): p99
  sim step time within the client's sim share ([GLOSSARY](GLOSSARY.md#the-clients-sim-share): at most 40 % of the
  tick period, 6.7 ms at 60 Hz, 13.3 ms at 30 Hz) while the main thread holds 30+ fps on Low
  for the whole run (VISION_V2 §4; R3 owns the render budget).
- P3 prediction replay on the same laptop: re-simulating the local soldier, or the vehicle it drives, for 200 ms of game
  time (12 ticks at 60 Hz, 6 at 30 Hz; proposed, to cover VISION_V2 §4's 100 ms ping plus buffers; F24's own example is
  a 200 ms round trip, 100 ms each way) takes at most 1 ms p99.
- Decision rule: 60 Hz if P1–P3 pass at 60 Hz; otherwise 30 Hz if they pass at 30 Hz, with R2 weighing sub-tick input
  timestamps (OQ4); otherwise back to the drafts.

Cross-topic open questions: R1-XT-OQ2.

**R1-TICK-REC3. Slow systems run on cadences in whole ticks, stated in game time and staggered by slot.**
Bot target scans, sight-check budgets per game second, route replans, the economy trickle and telemetry sampling run
every k ticks (k from a cadence in seconds), with the work spread across ticks by slot, as legacy does in seconds
(GN4). Only movement, shots, projectiles, collisions and damage need every tick.
Reason: the per-tick cost then grows with what must be fine-grained, not with all of AI; this is what keeps REC2's
60 Hz candidate affordable (F8: cost grows per simulated second).
Rests on: F8; legacy GN4, GP14, TL15.
Serves: VISION_V2 §3 (bots fill empty slots; the unit had written §6.1), §6.5 (economy), §6.6 (balance telemetry).
A spike must prove (proposed): at the largest mode's N in the stress scene ([GLOSSARY](GLOSSARY.md#the-stress-scene)),
p99 sim step time at most 2 × the mean sim step time (no tick where all bots think at once); sight checks per game
second per bot equal within 1 % at 30 Hz and 60 Hz.

**R1-TICK-REC4. The server counts due ticks from a monotonic clock, catches up within a bound, and sends snapshots
after the tick at R2's rate.**
Each match loop wakes on a timer but takes the number of due ticks from elapsed monotonic time (Q3's
`timeResidual`, F10; Fiedler's accumulator, F2), not from the timer period, because Tokio's timers are
millisecond-granular and coarser on Windows (F23). Catch-up per wake is bounded (proposed: at most 0.25 s of game time,
the clamp in Fiedler's loop, F3; 15 ticks at 60 Hz); a longer stall drops time and records it. Every overrun and every
dropped interval is a metric (R6 monitoring). Snapshots go out after the tick (as Q3 does, F10) at a send rate that is
separate from the tick (Q3: per client, up to 30 per second, F11); R2 sets that rate.
Rests on: F2, F3, F7, F10, F11, F23.
Serves: VISION_V2 §6.5 (authoritative economy and objectives), §4 (one EU region, fair up to about 100 ms). The overrun
and dropped-time counters are operations monitoring (R6's scope), which no §6 item names (the unit had filed them under
§6.6, balance telemetry); they stay because this recommendation's own pass marks read them.
A spike must prove (proposed): on the reference server (Linux), tick start jitter (actual minus scheduled) p99 at most
2 ms over 36,000 ticks; after an injected 200 ms stall the loop is back on schedule within 1 s, never running more than
15 ticks in one wake; overrun and dropped-time counters appear in the metrics output.

**R1-TICK-REC5. In the browser, the Worker owns the tick; the main thread renders every frame and interpolates; a gap is
a pause offline and a resync online.**
The Worker paces the sim with a `performance.now()` accumulator (F2, F20) woken by `setTimeout` (W2, MDN's Worker
pattern, F15); `Atomics.wait` (W3, F22) is the measured alternative if W2 fails the jitter mark. `requestAnimationFrame`
is the render clock only, never the tick clock, because its rate follows the display and it stops in background tabs
(F17). The main thread draws every frame from the two newest ticks with `alpha` (F4), read from shared memory (boundary
unit; R1-BND-REC2 recommends a transferred block instead: R1-XU-OQ3). After a long gap (hidden tab, sleep: F18, F21) the
client does not simulate the gap: offline it pauses, as Bevy's paused clock stops its fixed schedule (F7, F16 "a
pause"); online it resyncs from the server's state (F16: "You can recover the game state from a peer or the server."),
with catch-up per frame bounded as in REC4. Pause, shop, tutorial and hit-stop never stop the shared tick in
multiplayer; hit-stop is presentation-time only (GP9, GP13).
Rests on: F2, F4, F7, F15, F16, F17, F18, F20, F21, F22; legacy GP9, GP13.
Serves: VISION_V2 §4 (30+ fps on the minimum laptop, 60 fps recommended, on Chrome, Edge, Firefox and Safari), §6.9
(spectacle budget: the render loop is never blocked by the sim), and the brief's principle that the client runs the same
sim in a Worker for prediction and offline play.
A spike must prove (proposed), on Chrome, Edge, Firefox and Safari:
- Foreground Worker tick start jitter p99 at most 4 ms over 10 minutes.
- A unit moving at constant speed shows a per-frame on-screen displacement whose standard deviation is at most 5 % of
  its mean, on 60, 120 and 144 Hz displays.
- Hide the tab for 60 s, then return: offline, no tick ran while hidden and play resumes where it stopped; online, the
  client is back in step with the server within 0.5 s and never runs more than 15 ticks in one frame.

Cross-topic open questions: R1-XT-OQ3.

**R1-TICK-REC6. Map validation computes reach with the sim's own integrator at the shipped tick.**
The editor's reachability check (EDITOR_V2, LOOP_V2 §9) runs the sim's movement code at the shipped `dt`, not the
closed-form formula, because integration error depends on the step (F26) and legacy's formula overstates the real
reach (GW14).
Rests on: F26; legacy GW14, GN10.
Serves: VISION_V2 §6.2 (ledges, floors, bots that move between levels).
A spike must prove (proposed): the editor's reach value equals the measured jump apex plus step-up in the sim within
1 mm at the shipped tick, for the legacy values `T.jumpV` 11, `T.gravity` 24, `T.stepUp` 0.55.

### 3. Identical results between Wasm and native (unit `determinism`)

- **R1-DET-REC1. Two levels, and what the netcode really needs.** Same-build reproducibility is required: the same seed,
  data and input stream give the same state hash at every tick, on every run of the same binary. Identity between Wasm
  and native is a target, tested on every change, but the netcode must not depend on it: the server is the authority,
  and the client's prediction is corrected when it differs. Reject lockstep (Q1-E).
  Reason: with server authority a mismatch costs a correction (F32); lockstep turns "one tiny difference" into a desync
  (F32, F31). Same-build reproducibility fixes the legacy's testing debt (TL13, STATUS.md:70) at low cost. Cross-target
  identity keeps corrections rare when the client predicts with the same Rust code, and it is reachable on IEEE targets
  (F1, F7, F12, F20) if the rules of REC2 hold.
  Rests on: R1-DET-F1, F7, F12, F19, F20, F31, F32; legacy TL13, STATUS.md:70. Serves: VISION_V2 §6.5 (server-side
  economy and objectives), §6.6 (balance telemetry from reproducible runs), §3 (48 players and bots).
  A spike must prove (proposed): (a) a bots-only 24v24 match (48 players) and a 12v12v12 match (36 players), the two
  goals of VISION_V2 §3, plus the largest mode where it is neither
  ([GLOSSARY](GLOSSARY.md#the-largest-mode-and-its-n); MODES-3: 18v18v18 under reading (b)), each run 10 minutes at
  the tick rate the tick-rate unit proposes, 3 seeds each: 0 mismatching per-tick state hashes between two native
  runs in separate processes; (b) the same recorded matches compiled to Wasm: 0 mismatches against native (REC6 says
  where they run).
  Cross-topic open questions: R1-XT-OQ4.
- **R1-DET-REC2. Float rules for the sim crate.** One float type for the whole sim (f32 or f64, see OQ3). Inside the
  sim: only `+ - * /`, `sqrt`, `floor`, `ceil`, `round`, `trunc`, explicit `mul_add` and `as` casts from core;
  transcendental maths only through the pure-Rust `libm` crate, pinned, the same on both targets (or nalgebra's
  `ComplexField` with simba's `libm_force`, the path Rapier uses); no std `sin`, `cos`, `atan2`, `powf`, `exp`; no
  `algebraic_*` methods, no fast-math, no `relaxed-simd` target feature; Wasm `simd128` off until OQ2 is settled. NaN
  never enters sim state: every input and every data value is checked finite where it enters the sim, and debug and test
  builds assert finite state each tick. No sim value is computed in TypeScript. Tests build both targets with the same
  overflow setting, and tick counters and timers use explicit wrapping or checked integer operations.
  Reason: basic operations, `sqrt` and `mul_add` are exactly specified everywhere (F1, F7, F10) and Rust never fuses on
  its own (F11); std's transcendental functions vary by platform and Rust version (F9) and come from different code on
  Wasm and Linux (F16), while `libm` is one pure-Rust implementation whose `arch` feature only touches exact operations
  (F17); NaN bits are non-deterministic in Wasm and Rust (F2, F8, F13); relaxed SIMD is hardware-dependent and ships in
  Chrome and Firefox (F4, F5); JS `Math` differs per browser (F30); overflow differs by build flag (F15). Signed zeros
  from `min`, `max`, `clamp` (F14) are caught by the hash test (REC6), which compares `to_bits`.
  Rests on: R1-DET-F1, F2, F3, F4, F6, F7, F8, F9, F10, F11, F13, F14, F15, F16, F17, F18, F20, F22, F30 (F5 is context
  only: the ban on relaxed SIMD rests on F4; F3 is why the browsers' profile cannot be assumed; F18 leaves libm's
  cross-target identity to the pass mark; F20 and F22 carry the nalgebra route and the `simd128` rule). Serves: §6.2 (a
  true 3D sim: aiming at height and line of sight are trig and ray maths), §6.4 (vehicles), §6.3 (abilities as data,
  whose numbers enter this maths).
  A spike must prove (proposed): a table of at least 10,000 fixed inputs per `libm` function the sim uses gives
  bit-identical outputs on native x86-64, native AArch64 if R6 picks it, and Wasm in each browser engine of REC6; a lint
  or build check (tool chosen by R5) fails the build on a std transcendental call, `algebraic_*`, or the `relaxed-simd`
  feature in the sim crate; the cost of `libm` against std on native and Wasm is reported, with method.
  Cross-topic open questions: R1-XT-OQ1.
- **R1-DET-REC3. Rapier with `enhanced-determinism` on both targets.** Turn the feature on for server and client builds,
  never `simd8`; build bodies, colliders and joints from map data in a stable order of entity ids, and remove them in
  that order too; compute any trig that feeds Rapier through `ComplexField` / `libm`.
  Reason: without it Rapier promises identity only on one machine (F19); with it, on IEEE targets including Wasm, under
  stated conditions (F20, F21); it forces libm and scalar maths in its stack (F22, F23). Its cost is undocumented (F24)
  and the default 4-lane path stays on (F22), so both are spike items, not assumptions.
  Rests on: R1-DET-F19, F20, F21, F22, F23, F24. Serves: §6.2 (floors, ledges, line of sight across heights), §6.4 (big
  vehicles as moving obstacles), if R4 keeps Rapier.
  A spike must prove (proposed): with the flag on, the stress scene ([GLOSSARY](GLOSSARY.md#the-stress-scene): the
  largest mode's N soldiers and 15 big vehicles, the count R4-PHYS-REC4 takes up) gives 0 hash mismatches between
  native and Wasm over 10 minutes of ticks; the per-tick Rapier step time with the flag on and off is reported on
  native and in a browser, with method, and with the flag on it fits inside the sim step share
  ([GLOSSARY](GLOSSARY.md#the-sim-step-share-and-the-server-only-share)).
  Cross-topic open questions: R1-XT-OQ2.
- **R1-DET-REC4. One seeded sim RNG with streams, and no clock.** The sim draws only from `rand_chacha`'s `ChaCha8Rng`
  (pinned), seeded by the server per match, one stream per purpose (for example match setup, weapon spread, bot
  decisions, ability procs); the RNG state (seed, stream, position) is part of the authority's sim state and its state
  hash ([GLOSSARY](GLOSSARY.md#the-authority): the authority; [state hash](GLOSSARY.md#state-hash)). Streams are
  shared or authority-only ([GLOSSARY](GLOSSARY.md#shared-stream-and-authority-only-stream)): an authority-only
  stream is held by the server online and by the Worker in an offline match, and stays out of the client's predicting
  state and out of any hash compared with a client. Weapon spread is authority-only (R2-SYNC-REC3, the security
  review's finding H2: a seed the client knows would let a cheat cancel the spread through its aim point); whether it
  becomes shared so tracers match is SEC-SPREAD (below), Mustafa's call. Presentation
  (effects, sound, camera shake, animation variety) uses its own client-side generator and never the sim's. The sim
  reads no clock: time is the tick counter times the fixed step; animation never moves a launch point or a hitbox. Pcg32
  is a candidate fallback if ChaCha8's cost shows up in the profile (portable, "a good choice on both 32-bit and 64-bit
  CPUs": F34), but not with one stream number per purpose: its docs warn that "two generators with different stream
  parameters may be closely correlated" (F34). How a Pcg32 fallback would keep purposes apart is open (R1-DET-OQ11;
  the second fix round removed the stream number as a merit and the "for the spike" deferral).
  Reason: `StdRng` is non-portable (F27); ChaCha8Rng is portable, reference-tested, with streams and a settable position
  (F28), so a snapshot can carry the RNG exactly (inferred); the legacy's shared `Math.random` and wall-clock reads are
  exactly what made same-seed matches differ (STATUS.md:70, GP14, GN6, GN8, GC5), and JS `Math.random` has no seed
  (F30).
  Rests on: R1-DET-F27, F28, F30, F34 (and the legacy rows above). Serves: §3 (bots fill empty slots), §6.3 (ability
  procs), §6.5 (server-side outcomes), §6.6 (reproducible runs).
  A spike must prove (proposed): the first 10,000 outputs of each stream are identical on native and Wasm; a stream's
  state saved mid-match and restored continues with identical outputs; muting sound, moving the camera or changing the
  display frame rate changes no state hash in a recorded match (the legacy's failure modes, GP14).
- **R1-DET-REC5. Stable iteration order.** Sim state lives in dense arrays or slot maps keyed by stable entity ids and
  is iterated in id order; systems run in a fixed order each tick (the legacy order in GP14 is the starting point);
  the sim crate bans std `HashMap` and `HashSet` by type (R5-RUSTLINT-REC2;
  [GLOSSARY](GLOSSARY.md#the-hash-map-ban-in-the-sim-crate): the hash-map ban); a reviewed `#[allow]` with a written
  reason may admit a map that is only looked up and lies outside the serialized state, since REC6's canonical
  serialization walks the whole state; where an ordered map is needed, `BTreeMap`.
  Reason: `HashMap` order is arbitrary (F25) and every `HashMap` instance takes its own seed (F33; the unit had written
  "per process", corrected at the fix step); `BTreeMap` order is the key order (F26); Rapier
  requires the same insertion order (F20) and parry's flag pulls an ordered map for the same reason (F23, inferred).
  Rests on: R1-DET-F20, F23, F25, F26, F33; legacy GP14. Serves: §3 (fair processing of 48+ players and bots), §6.5.
  A spike must prove (proposed): REC1 (a) passes; because every std `HashMap` instance takes its own seed (F33), two
  runs, in one process or in two, can iterate a hash map in different orders and so can catch an order that leaks from
  one (inferred, not certain: F33 gives each map its own seed, not a different order every time; REC6's injected
  `HashMap` iteration tests the catch itself; the second fix round softened "any two runs … also catch any order").
- **R1-DET-REC6. Prove it with recorded matches and per-tick state hashes.** A determinism suite that every sim change
  runs: (1) a recorded match is the seed, the map and mode data, the content data's hash and the per-tick input stream;
  (2) after each tick the sim writes a canonical serialization of its whole state (fixed field order, floats by
  `to_bits`, RNG state, tick counter) and hashes it with a fixed, unseeded hash function; (3) the same recording runs
  natively twice in separate processes, as Wasm in Node.js, and as Wasm in the browser engines the test runner offers
  for Chrome and Edge, Firefox and Safari (R5 picks the tool); (4) the first mismatching tick is reported with a
  field-level diff of the two states; (5) unit vectors for `libm` and the RNG streams (REC2, REC4) run on the same
  targets.
  Reason: no source read gives a verification method (F24), so we define one; every `HashMap` instance takes its own
  seed (F33), so the two native runs can expose hash-map order, as two runs in one process could (inferred, not certain:
  a seed per map does not promise a different order every run; the unit had tied this to separate processes, corrected
  at the fix step, and the second fix round softened "expose" to "can expose"); NaN and relaxed-SIMD effects differ by
  engine and hardware (F2, F4), so the browsers themselves must be in the matrix; the legacy already showed that event
  logs on seeded runs expose divergence (GN8).
  Rests on: R1-DET-F2, F3, F4, F24, F25, F33; legacy GN8. Serves: §6.6 (balance telemetry: the numbers it records come
  from runs that can be re-run), §3 (the scale test at 48).
  A spike must prove (proposed): the suite runs REC1's matches and reports 0 mismatches on all targets; a deliberately
  injected divergence (one std `sin` call, one `HashMap` iteration, one presentation RNG draw inside the sim) is caught
  at the first tick it changes the state; hashing cost per tick is reported, with method.
  Cross-topic open questions: R1-XT-OQ2.
- **R1-DET-REC7. No fixed point.** Keep IEEE floats under REC2's rules.
  Reason: shipped PC games held float identity with strict control (F31); the targets we need are IEEE-compliant (F1,
  F7, F12); the netcode does not need bit-identity for correctness (F32); fixed point would exclude Rapier and bring no
  trig (F29). Revisit only if REC6 finds a divergence that cannot be removed and R2 shows its corrections break the
  VISION_V2 §4 fairness target (fair up to about 100 ms of ping).
  Rests on: R1-DET-F1, F7, F12, F29, F31, F32. Serves: §6.2, §6.4 (through Rapier, if R4 keeps it).
  A spike must prove (proposed): nothing beyond REC1 to REC6; this recommendation stands or falls with their results.

### 4. The Rust ↔ TypeScript boundary, and the simulation in a Web Worker (unit `boundary-worker`)

- **R1-BND-REC1. Run the client simulation in one dedicated module Web Worker, single-threaded, on stable Rust
  (A2 + E1).** The page keeps input listeners, the HUD and rendering and never instantiates the sim. Reason: the
  tick leaves the render thread, the worker form works in all four target browsers, and Wasm threads would bring
  nightly Rust, a rebuilt std and isolation for no §6 need. Rests on: F12, F18, F19, F25, F27, F28, F32, F33 (and legacy
  GP13, GP14, GC2). Serves: VISION_V2 §6.2 (a true 3D sim with line of sight across heights; keeping it off the render
  thread is this recommendation's reason, not §6.2's words),
  §6.9 (the render thread keeps its frame budget), §4 (four browsers; minimum 30+ fps). Spike must prove (proposed):
  (1) on the proposed minimum reference machine, a laptop with integrated graphics per VISION_V2 §4 (model named in
  the ADR, its `hardwareConcurrency` recorded), at the LOOP_V2 §11 scale points N = 12, 24, 36, 48 soldiers, the
  page's frame time p95 with the sim in the worker is no worse than with no sim at all by more than 1.0 ms; (2) the
  page makes zero calls into Wasm, and the worker makes the same number of Wasm calls per tick at N = 12 and N = 48
  (at most 4); (3) the same build runs in the latest desktop Chrome, Edge, Firefox and Safari.
  Cross-topic open questions: R1-XT-OQ1, R1-XT-OQ14.
- **R1-BND-REC2. One flat, versioned output block per tick, written by Rust into preallocated linear memory and handed
  to the page as one transferred ArrayBuffer from a small pool (B2); do not share the sim's Wasm memory (not B4).** The
  block holds a header (tick, layout hash, counts per entity kind), aligned sections per entity kind (soldiers, vehicle
  parts, projectiles, structures) and an event section for the tick (hits, deaths, purchases, captures, moments). The
  worker copies the block out once per tick; the page binds typed-array views (no copy) and keeps the latest two blocks
  for interpolation, returning older ones to the pool. B3 (SharedArrayBuffer ring) is the spike's measured alternative.
  Reason: this is the book's "pointers and overlays" rule with one bounded copy, without isolation or nightly Rust (the
  copy is needed because the sim's own memory cannot be handed over: transferring it would take it from the worker, F15,
  and its buffer carries the Wasm detach key, F35); B4's zero copies cost the threads toolchain, isolation, a fixed
  memory maximum and re-binding on growth. Rests on: F7, F10, F11, F12, F15, F16, F17, F18, F30, F35 (and legacy GA15,
  GP10, GC2). Serves: §6.2, §6.4 (multi-part vehicles as records), §6.8 (moments arrive as events), §6.9. Spike must
  prove (proposed): (1) per tick: at most one block copy in the worker, zero copies and zero per-entity calls on the
  page; (2) on the page, no buffer or typed-array allocation per frame after warm-up, and boundary-caused heap growth
  under 0.1 MB/s over a 10-minute match (allocation sampling); (3) the time from the worker finishing tick T to T's
  block being readable on the page is at most one display frame (16.7 ms at 60 Hz) at p95, measured on a shared clock
  (`performance.timeOrigin + performance.now()` on both sides, since a worker has its own time origin, F30); (4) zero
  lost or reordered events over a 10-minute match with the largest mode's N soldiers (48, or 54 under MODES-3 (b));
  (5) B2 against B3 on the four browsers: if B3 saves
  at least 0.5 ms of page time per frame or one frame of latency at p95 on the minimum machine, the ADR weighs isolation
  (REC5); otherwise B2 stands.
  Cross-topic open questions: R1-XT-OQ3, R1-XT-OQ9, R1-XT-OQ16, R1-XT-OQ17.
- **R1-BND-REC3. Inputs enter as one command per tick, posted from the page to the worker (C1).** The command carries
  seq, tick, move, aim (yaw and pitch, for aiming at height), the held-button bitmask and press and release edge
  counters; the worker stamps the tick it applies it to. The exact fields are R2's. Reason: one small message per tick
  is constant work and needs no isolation; edge counters fix the legacy event-time actions (GP2) and the never-wired
  command (CS1). Rests on: F34 (posted messages are copied by the structured clone algorithm; re-sourced at the fix
  step), F15 (a transfer is the zero-copy exception), F12 (C2 would need isolation) and legacy CS1, GP2. Serves: §6.2
  (aiming at height), §4 (fair up to about 100 ms of ping: commands carry ticks for R2's lag compensation). Spike must
  prove (proposed): (1) in a test with injected delay, duplication and reordering of commands, zero lost and zero
  doubled presses over 10,000 scripted presses; (2) offline, from the key event's timestamp to the tick that applies it:
  at most one tick plus one frame at p95.
  Cross-topic open questions: R1-XT-OQ1.
- **R1-BND-REC4. Generate every type that crosses the boundary from Rust, from three sources: wasm-bindgen's default
  `.d.ts` for the small control API (D1); our own small generator for the output block's layout, with a layout hash
  checked at startup (D4); ts-rs for low-rate messages, protocol and content-schema types (D3). Not tsify for now.**
  Pin the wasm-bindgen CLI to the library version. Reason: D1 is free and needed anyway; only D4 covers the bulk
  block; ts-rs has no runtime part and serves server, client and tools alike, while tsify's values cross by serde
  (per-field calls) and its maintenance line is unclear. Rests on: F2, F3, F4, F5, F8, F9, F10, F11. Serves: §6.3
  and §6.7 (units, abilities and faction content as data typed from one definition), and the starting principle of
  a narrow typed boundary. Spike must prove (proposed): (1) CI regenerates all types and finds zero diff against the
  committed output; (2) a deliberate Rust layout change without regenerating fails at build or startup (layout hash
  mismatch), proven by a test; (3) `tsc --strict` passes on the generated output; (4) CI fails when the wasm-bindgen
  CLI and library versions differ.
  Cross-topic open questions: R1-XT-OQ13.
- **R1-BND-REC5. Do not require cross-origin isolation for v2's start (G1); allow it in development and previews for
  profiling (G3; [GLOSSARY](GLOSSARY.md#preview-isolation-and-the-isolation-flag): the isolation flag). Adopt it in
  production only if the spike's B2-against-B3 test passes REC2's threshold.** If adopted:
  COOP `same-origin` with COEP `require-corp` (Safari has no `credentialless`), every asset same-origin or served with
  CORP, rollout through the report-only header first, and the later login (VISION_V2 §5) on a page or redirect flow
  outside the isolated game page (isolation needs COOP `same-origin`, F20, which severs cross-origin popups, F23;
  whether `same-origin-allow-popups` could keep a login popup on an isolated page is R1-BND-OQ9). Reason: isolation buys
  B3/C2, finer timers and the memory-measurement API (F20, F30), and it costs, on every browser: the page and its worker
  scripts must carry the headers (F20, F24), every cross-origin resource loaded in `no-cors` mode needs CORP and frames
  must be isolated too (F21; the unit's "or CORS" is not in F21's quotes and was cut at the second fix round), and
  cross-origin popups lose their opener (F23). Rests on: F12, F20, F21, F22, F23, F24, F26, F30. Serves: §4 (four
  browsers including Safari), §5 (guest play first, login later); no VISION_V2 §6 item names isolation, which matters
  only through REC2's B2-against-B3 test (stated at the second fix round). Spike must prove (proposed): if isolation is
  adopted, `crossOriginIsolated` is true in all four browsers on a [preview link](GLOSSARY.md#preview), and the
  report-only run of a full match
  shows zero COEP violations.
  Cross-topic open questions: R1-XT-OQ17.
- **R1-BND-REC6. The worker owns the network connection in multiplayer and runs the same sim locally when offline
  (H2).** Online, server snapshots, prediction and reconciliation run in the worker, which then publishes the same
  output block to the page; offline, the same worker build runs the match itself, with bots filling the slots. The
  page code is the same in both. Reason: both planned transports exist in workers, so the page never handles a
  network packet, and one boundary serves prediction and offline play. Rests on: F29, F25 (and legacy GP13). Serves:
  VISION_V2 §3 ("Bots fill empty slots"), §4 (fair up to about 100 ms: prediction is not delayed by a busy frame) and
  the brief's starting principle that the client runs the same sim for prediction and offline play; offline play
  itself traces to no §6 item (OQ1), and the unit's §6.1 link was dropped at the fix step (§6.1 is modes, rooms and
  private matches, not who owns the connection). Spike must prove (proposed): (1) the page handles zero network
  messages (a counter); (2) a 100 ms busy loop injected on the page leaves no gap in the worker's processing of
  server snapshots (worker timestamps); (3) the offline match and the online client run the same Wasm file and the
  same page code, differing only in a mode flag. Offline play's status as a v2 need is OQ1.
  Cross-topic open questions: R1-XT-OQ3.

### 5. A data-driven ability and effect system (unit `abilities`)

**R1-ABL-REC1. A reduced GAS subset, as data, over a closed set of Rust operations (option A2).**
- The model: attributes with a base and a current value; effects that are instant, timed or permanent, optionally
  periodic; counted hierarchical tags; abilities gated by required and blocked tags (team tech tiers included, LOOP_V2
  §4), with a cost and a cooldown; effects that grant tags or abilities for their lifetime; cues as presentation events
  only. Content chooses and parameterizes operations (for example "apply effect E to targets in radius R with falloff
  F", "grant ability A while active", "on kill by source, run effect E"); it cannot run arbitrary code. Leave out what
  exists for Unreal's object and replication model (instancing policies, replication modes, ability tasks as a general
  framework) and anything no legacy or LOOP_V2 test case needs.
- Damage and similar outcome logic is a fixed pipeline of Rust stages parameterized by data (locational multiplier,
  range falloff, frontal-arc block, target-tag multiplier for counters such as "strong against vehicles", owner and team
  rules), the role GAS gives to execution code (F9), which GAS does not predict either.
- Aggregation order is fixed and documented per attribute (F4 shows that the default GAS rule sums multipliers, which is
  a rule the designers must know). Stacking is an explicit per-effect policy chosen from a short list (no stacking,
  refresh duration, add a stack up to a limit, strongest only as in F19), never an implicit default (GAS has policies
  for expiration, refresh and reset, for which the write-up points to the editor's tooltips, F5; Paragon kept only the
  strongest slow, F19). The unit had said the policies are "only documented in tooltips"; the source has no "only"
  (unsupported, verifier 2026-10-28), so the fix step rewrote the reason to the source's words.
- Reason: §6.3 asks for GAS-like data; the subset covers every legacy ability and weapon (Legacy facts) and the
  primitives §6.4 names (an effect bound to its source's life for leech; for marking, an effect that grants a tag, which
  is our reading, since §6.4 names marking without defining it), without generic-engine features.
- Rests on: F1, F2, F3, F4, F5, F6, F7, F8, F9, F10, F11, F16, F19.
- Serves: VISION_V2 §6.3, §6.4, §6.5 (tech gates), §6.7.
- A spike must prove (proposed pass marks):
  1. The test content runs with no ability-specific Rust code: AMK, Bagpipe, Shabzon, Retro Lancer (burst), Beam Katana
     (melee arc), Torque Bow (charge, stick, 0.55 s fuse), frag grenade, Molotov, Riot Shield, Banzai, Bayonet and the
     Juggernaut cannon, all as data. Measure: zero matches for those content ids in the simulation crate's non-test
     code.
  2. A new combination needs data only: a test-only definition (not a game rule) that puts the Riot Shield's block on
     a vehicle and the Banzai speed buff on a different class loads and behaves as its parts do, with zero Rust
     changes.
  3. The two §6.4 primitives work in a test fixture: an effect removed when its source dies (LOOP_V2 §6: "Stolen items
     only last until the Hammer dies"), and a trigger on the last hit or tick that moves the victim's item to the source
     (LOOP_V2 §6: the Hammer's leech "steals an enemy tank's item when it lands the last hit or tick"; modelling the
     item as a granted ability is our inference; the unit had written "an on-kill trigger that moves a granted
     ability"). The item and marking rules themselves stay undesigned ([LATER], Mustafa's).
  4. Changing one number in data (for example Shabzon damage 8 to 9) changes the next match with zero Rust changes
     (the hot-reload path is the data-formats unit's).

**R1-ABL-REC2. Weapons keep their own bookkeeping and share the damage pipeline (option B1).**
- A weapon definition holds the sim fields only (fire interval, fire mode auto, semi or burst, magazine, reserve,
  pellets, spread hip, ADS, per shot, max and decay, range, falloff, damage, reload, charge curve, projectile kind);
  the feel kit (recoil, tracer, flash, sound, kick) is a separate client definition keyed by the same id (LOOP_V2 §7,
  CC9). The weapon's state machine (timer, ammo, burst count, charge, reload, active-reload window) is per-soldier sim
  state; each hit becomes an instant damage effect through REC1's pipeline. Abilities may grant or swap weapons (Banzai
  equips the Katana; Molotov is a thrown weapon) and apply effects to weapon attributes (active reload's short damage
  boost, LOOP_V2 §7).
- Reason: F17 says Fortnite keeps weapon fire rates out of the effect system, and its source gives the reason as
  latency: with cooldowns as effects that the server overwrites, "Higher latency connections would have a lower ROF
  with basic attacks" (Paragon). F22 shows a shipped shooter's per-player integer weapon timer, inside the player code
  both sides run (F21). That this also keeps the hottest path (13.9 shots per second for the Shabzon, CD12) out of
  effect bookkeeping is our inference: neither source gives a performance reason (the unit had stated it as the
  sources' reason: unsupported, verifier 2026-10-28; rewritten at the fix step).
- Rests on: F17, F21, F22, F9.
- Serves: VISION_V2 §6.3 (weapons built from data), §6.4 (vehicle weapons on mounts).
- A spike must prove (proposed): for each of the seven legacy weapons and the Juggernaut cannon, per-shot results equal
  the legacy tables: damage per body part (head 2.6, body 1.0, legs 0.65; Bagpipe head 1.2), Shabzon falloff at 10, 15,
  25, 35 and 50 m against the legacy `falloffMult` formula, pellet count, magazine, reload and burst timing; integer
  results exactly, floating-point results within 1e-6 relative (the bound is proposed); and one weapon model for bots
  and humans (CD13).

**R1-ABL-REC3. Integer time on the fixed tick, with carried remainders and a stable order (option C1).**
- Authors write seconds; the loader converts every duration, period, cooldown, fuse and fire interval into an integer
  time unit in which the tick step is exact, and reports the rounding of each value. Timers carry their remainder
  (F22's `+=`), so table rates hold on any tick rate. Effects start, tick and expire only on tick boundaries; the order
  of application within a tick is fixed (for example by entity index, then effect start tick, then a per-match sequence
  number), and no per-tick logic iterates a randomly seeded `HashMap` (F26; the hash-map ban,
  [GLOSSARY](GLOSSARY.md#the-hash-map-ban-in-the-sim-crate), bans the types). Random rolls come from the
  simulation's seeded RNG streams (spread from an authority-only stream, R1-DET-REC4; SEC-SPREAD), never from the
  activation key alone (F18 is a 16-bit key that both sides know).
- Reason: GC5 measured the legacy reset-not-carry rule losing up to 28 % of the Shabzon's rate at 30 Hz
  (10.0 vs 13.9 shots per second); F22 shows the carry; F26 shows that `HashMap` order is not stable across runs.
- Rests on: F22, F26, F3, F18; legacy GC5.
- Serves: VISION_V2 §6.3 (tuning that means the same thing everywhere), §6.6 (telemetry numbers comparable across
  matches).
- A spike must prove (proposed): at the tick rate the fixed-tick unit recommends, the Shabzon's sustained rate is within
  1 % of 1 / 0.072 s = 13.9 shots per second, and CD13's Warrior-kills-Conscript time (0.86 s from the table values) is
  reproduced within one tick; a 120 s scripted scenario that uses every test ability, run with 3 seeds, gives
  byte-identical logs of damage, effect start and end, and cooldown start and end on native and Wasm.

**R1-ABL-REC4. Predict only the local player's own weapons, and its own abilities if ABL-PREDICT says so; never
  damage (option D2).**
- The client predicts its own weapon timer and ammo (R2-SYNC-REC3's own weapon state) and, if ABL-PREDICT (below)
  says so, its own activation, cost, cooldown, granted tags and movement changes (Banzai speed, Shield slow), and the
  cues of those; damage, kills and effects on other players wait for the server. The
  prediction key is the input command's sequence number (the field CS1 already declared), echoed by the server with
  accept or reject; a rejected activation is rolled back. Cues are events keyed by (entity, ability or weapon,
  command sequence), so a re-simulated or confirmed activation does not play twice. Effect removal and periodic ticks
  are not predicted (as in GAS, F13), so a predicted buff such as Banzai's speed ends on the client only when the
  server's removal arrives (inferred). The GAS route's known failure (F17): when the server overwrites predicted
  cooldowns, high-latency players fire and re-activate more slowly (Paragon); REC2 keeps weapon timers out of effects
  for that reason, and whether ability cooldowns avoid it depends on R2's prediction model (OQ4). Both points were
  added at the fix step (verifier, 2026-10-28).
- Reason: F13 and F15 (Epic's shipped practice, second-hand), F14's key-and-confirm model, F16 (cues never decide
  gameplay), LOOP_V2 §7 (split feedback). Because the client runs the same Rust code in Wasm, prediction of the local
  player can be the netcode's re-simulation from the last confirmed state instead of per-effect keys: that is an
  inference, and R2 decides the prediction model (OQ4): R2-SYNC-REC3 replays the local player from the authoritative
  state, and leaves abilities to ABL-PREDICT.
- Rests on: F12, F13, F14, F15, F16, F17, F21.
- Serves: VISION_V2 §6.3, §4 ("matches feel fair up to about 100 ms of ping").
- A spike must prove (proposed): with 100 ms injected round trip (plus jitter and loss as R2 sets), if ABL-PREDICT says
  yes, the local player's ability press changes its own cooldown, cost and speed in the same client tick as the input,
  and a rejected press (for example before Tech 1) is undone within one round trip plus one tick; every accepted
  activation plays its cue exactly once (count over 1,000 scripted presses); no damage number or kill on another
  soldier is shown before the server's
  event.

**R1-ABL-REC5. One table per kind, linked by validated ids (option E1).**
- Kinds: tag registry; attribute set per unit kind; effect; ability; weapon (sim); feel kit (client only); unit (class:
  attributes, weapons, abilities, tier, cost, tags); vehicle (parts, each with HP and tags; weapon mounts as fixed
  points; seats, with the count left open by VISION_V2 §6.4 [OPEN]); faction (which units and vehicles, uniforms by
  reference, §6.7). String ids resolve to dense handles at load; every reference, tag and number range is checked, and
  a failure names the file and field; no silent defaults (CC9). The file format, schema tooling and hot reload are the
  data-formats unit's.
- Reason: the legacy tables already have this shape (CD12–CD14) and their faults are mixing (bot and colour fields in
  unit data, CD13), duplication (multipliers in two places, CS3) and silent fallbacks (CC9).
- Rests on: legacy CD12–CD14, CS3, CC9 (the case for one table per kind); F6 (tags as the shared vocabulary between
  kinds), F8 (abilities granted by effects need referenceable ids), which show only that kinds share tags and ids.
- Serves: VISION_V2 §6.3, §6.4, §6.7, §6.6 (telemetry keys pick and win rates by these ids).
- A spike must prove (proposed): the legacy `WEAPONS`, `UNIT_TYPES`, `VEHICLE_TYPES` and the ability fields convert
  into these tables with every legacy number present once (a script lists any number with no home); a definition with
  an unknown id, tag or out-of-range value fails at load with file and field named; 10,000 fuzzed definitions never
  panic the simulation.

**R1-ABL-REC6. Cost budget for the ability and effect step (proposed).**
- The ability, effect and weapon step in the stress scene ([GLOSSARY](GLOSSARY.md#the-stress-scene): the largest
  mode's N soldiers, VISION_V2 §3, each with up to 8 active effects, a proposed load), costs at most 1 ms per tick at
  p99, inside the sim step share on the server and the client's sim share on the laptop
  ([GLOSSARY](GLOSSARY.md#the-clients-sim-share)), on one vCPU of the reference server
  ([GLOSSARY](GLOSSARY.md#the-reference-server)) and on the VISION_V2 §4 minimum laptop running Wasm in a Worker
  (both budgets proposed), and allocates nothing on the heap in steady state (counted
  with a counting allocator).
- Reason: the brief's own example pass mark ("48 simulated players at a 30 Hz server tick within X ms"); this unit's
  share of X must be measured, not assumed.
- Rests on: the brief's example pass mark ("48 simulated players at a 30 Hz server tick within X ms",
  docs/V2_PHASE0_BRIEF.md §4) and VISION_V2 §3 (24v24, 48 players), both confirmed by the verifier (2026-10-28); no
  external finding; every number in it is proposed.
- Serves: VISION_V2 §3 (scale), §4 (minimum hardware); no §6 item, since it is a budget, not a feature.
- A spike must prove: the numbers above, measured with the method R5 sets.
- Cross-topic open questions: R1-XT-OQ15.

### 6. Data formats and hot reload of tuning (unit `data-hotreload`)

- **R1-DATA-REC1. JSON is the one content format.** Every content and tuning file (maps, modes, factions, units,
  vehicles, weapons, ability data, rule tables) is JSON, one definition per file, written by one canonical writer (fixed
  key order, shortest round-trip number text, no lossy rounding). serde_json is built with `float_roundtrip`. Content
  integers stay within the range a double holds exactly; ids are strings.
  *Reason:* the editor already writes JSON (EDITOR_V2 rule 1), the browser parses it natively, the legacy values are
  JSON, and JSON Schema works on both sides; RON and TOML add a second format with gaps (F12, F13); YAML's crate is
  archived (F14); exact floats are needed for identical results on every side (legacy floats).
  *Rests on:* F6, F11 (fixed key order), F12, F13, F14, F15, F16, F19, F26; EDITOR_V2 §2 rule 1; legacy facts (floats,
  GW12). The "shortest round-trip number text" from both writers has no finding: mark (1) tests it.
  *Serves:* §6.1, §6.3 ("tuning needs no code changes"), §6.5, §6.7; EDITOR_V2 §2 and §4.
  *A spike must prove (proposed pass marks):* (1) every double in legacy-tuning.json, plus a generated corpus of 10,000
  random doubles, parses to the same 64-bit pattern in native Rust, in Rust on Wasm and in `JSON.parse`, and is written
  back as the same text: 100 % equal; (2) loading a content set the size of the imported legacy tables plus two maps and
  two modes, on the native server, takes a time the spike reports, against a budget the ADR sets (no number is sourced
  yet).

- **R1-DATA-REC2. One Rust content crate loads and validates, the same code everywhere.** Stages: byte-size cap → JSON
  to tree with the default depth limit (F9) → read format and version, migrate (REC4) → typed load with
  `deny_unknown_fields` (F1), checked newtypes (F3) and tagged kinds (F5; never `untagged`, never `flatten` where
  unknown keys must fail, F2) → garde rules (F18) → whole-set checks (cross-references, every objective a mode needs is
  in the map, no fallback map) → a frozen content set with its hash. Errors name file, line and column for syntax (F7)
  and the field path for data (F8). The crate depends on nothing above it; the simulation, the server, the client's Wasm
  worker, the editor (through Wasm) and the tools use it.
  *Reason:* one verdict on every side; the legacy failures (unknown keys, no ranges at load, silent fallbacks, GW2,
  GE11, CM1) are each a stage here.
  *Rests on:* F1, F2, F3, F5, F7, F8, F9, F18, F25; legacy facts GW2/GE1, GE11, CM1, dead keys. It also depends on two
  open points no source settles: garde building for wasm32 (OQ5) and the editor's double-click rule working with Wasm
  (OQ11); mark (1) tests native and Wasm.
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
  *Cross-topic open questions:* R1-XT-OQ13.

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
  its next tick (that a browser cannot watch the files itself is our inference: F21 says only that Bevy's file watcher
  is a desktop feature). Entities hold definition ids, not copies, so a changed number reaches
  live units on the next tick, while live state (current health, ammo) is left as it is. Client-only feel data (the Feel
  Lab, LOOP_V2 §7) reloads freely, since presentation decides nothing. Production: one content set per match; changes
  take effect at the next match.
  *Reason:* the iteration loop LOOP_V2 and §6.3 ask for, without letting a half-written file or a mid-match change
  break a match, and with the client predicting on the same numbers as the server.
  *Rests on:* F7, F8, F20, F21, F22; legacy fact GE1 (live `applyRules`); LOOP_V2 §7.
  *Serves:* §6.3, §6.5 ("tunable"), §6.6 (tune on evidence). The trace is partial (verifier, 2026-10-28): §6.3 asks for
  tuning without code changes, not for live reload, and LOOP_V2 §7's live tuning covers the client's feel data;
  swapping sim numbers inside a running match (marks 2 and 3) is a development convenience no §6 item names, so
  whether to keep it is OQ12.
  *A spike must prove (proposed):* (1) a saved change to a weapon's damage applies on the dev server within 1 s of the
  save; (2) the client's Wasm simulation applies it at the same tick as the server, with 0 extra mispredictions beyond
  the change itself, under 100 ms injected latency with jitter (VISION_V2 §4: matches feel fair up to about 100 ms of ping); (3) 50 random valid tuning edits in a 5-minute bot match
  give 0 crashes and 0 desyncs; (4) a malformed save leaves the match running on the old numbers and shows the error
  within 1 s.

- **R1-DATA-REC6. Content reaches the browser as a hashed, cached bundle, and every match records its content hash.**
  The build writes the validated content set as one bundle whose file name holds its hash, served as immutable (F27);
  at join the server names the hash the match uses; the worker checks the bundle against it (SHA-256 is available in
  workers, but only in secure contexts, F29, so plain-HTTP LAN development needs another path; the second fix round
  cut "file://" from this clause, see OQ11) before the simulation starts. Small or per-match data (private-match
  overrides, dev reloads) comes over the game connection. Content is not embedded in the Wasm binary (F28), so tuning
  ships without a code release. Every match record (§6.6 telemetry) stores the content hash and the mode id, so pick
  and win rates are grouped by tuning version.
  *Reason:* client and server must run the same numbers for prediction; telemetry must know which numbers a match used.
  *Rests on:* F27, F28, F29.
  *Serves:* §6.1, §6.6, §6.7.
  *A spike must prove (proposed):* a client with a stale or altered bundle is refused or re-fetched before the match
  starts (100 % of 3 tampering cases: stale file, one changed byte, missing file); a repeat visit loads the bundle from
  the browser cache (0 bytes over the network for it); 100 % of spike match records carry a hash that resolves to an
  archived content set.
  *Cross-topic open questions:* R1-XT-OQ6.

- **R1-DATA-REC7. The server validates everything a client or editor sends, through the same pipeline.** Private-match
  settings expose a named list of tunables with ranges from the schema (an allow list), never free paths into the
  content; a map uploaded for a private match goes through REC2 plus the editor's map checks (reachability, 6 m gaps,
  symmetry, budgets) on the server. On the TypeScript side, parsed JSON is never merged into plain objects with
  `Object.assign` or spread; untrusted keys go into `Map` (F24) or straight into the Wasm validator.
  *Reason:* client checks are bypassable (F25); the legacy merge is how `__proto__` got in (F23, GW2).
  *Rests on:* F1, F9, F18, F23, F24, F25; legacy facts GW2/GE1, CD19, GE11; EDITOR_V2 §3 (the validation panel:
  reachability, the 6 m gap rule, symmetry, budgets).
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

## Open questions

### 1. ECS or plain data (bevy_ecs, hecs, custom) (unit `ecs`)

- **R1-ECS-OQ1. What do the options really cost in Wasm size and compile time?** No source read gives figures (F34).
  Decided by: REC1 mark 5's measurement; the budget itself is set in the ADR from the number, not before.
- **R1-ECS-OQ2. Does `#[derive(Clone)]` give a field-wise `clone_from` that reuses each Vec's buffer?** The sources read
  cover Vec's clone_from (F26) and the trait's default (F27), not what the derive generates. Decided by: reading the
  derive's documentation, or REC1 mark 2's allocation count; if it doesn't, clone_from is written once for the state
  struct. Settled at the fix step: it does not. The derive calls `clone` on each field, and the default `clone_from` is
  equivalent to `a = b.clone()` (F35; rust-reviewer risk 5), so clone_from is written by hand (REC1 mark 2).
- **R1-ECS-OQ3. Does the client re-simulate the whole world, or only what it predicts (its own soldier, its vehicle,
  its projectiles)?** This sets how much state a snapshot and replay must cover, and so whether REC1 mark 3 can pass.
  Decided by: R2's prediction recommendation, checked against REC1 marks 1–3.
- **R1-ECS-OQ4. Does the server need parallelism inside one match at 48+ players, or is one match per vCPU of the
  reference server enough?**
  bevy's multi-threaded executor (F7) would be the one thing option A offers that C lacks. Decided by: REC1 mark 1's
  native sim step time against the sim step share ([GLOSSARY](GLOSSARY.md#the-sim-step-share-and-the-server-only-share))
  at the tick rate the tick unit picks, and R6's hosting cost per match.
- **R1-ECS-OQ5. Is the pinned Rapier version's serialized and cloned state complete?** F32 is a user report of a gap,
  fixed per the reporter. Decided by: REC4's test on the version the ADR pins.
- **R1-ECS-OQ6. What did Overwatch's ECS experience teach about gameplay architecture at scale?** The GDC 2017 talk was
  not read (the vault redirected to a maintenance page outside the allow list). Decided by: reading the talk or its
  slides when the vault is back, and checking whether its lessons change the trade-off as the roster grows
  (VISION_V2 §6.7).
- **R1-ECS-OQ7. Will the entity kinds stay few?** REC1 rests on about a dozen known kinds (F21). If factions and
  abilities add many kinds that each need their own table (deployables, zones, mines), the case for an ECS grows.
  Decided by: the game-designer's roster and ability lists; the proposed trigger is to revisit when a new kind can't
  be expressed as a row in an existing pool.

### 2. A fixed tick rate (unit `tick`)

**R1-TICK-OQ1. 60 Hz or 64 Hz?** Bevy picks 64 Hz for an exact float `dt` and against a 60 Hz display beat (F6); legacy
parity and Rapier's default favour 60 (F8, CS1). Decided by: the determinism unit (does an exactly representable `dt`
matter for identical Wasm and native results?) and REC5's smoothness test (does a Worker-paced 60 Hz tick judder on a
60 Hz display?).

**R1-TICK-OQ2. Are worker timers throttled in hidden tabs?** The sources read do not say (F19); the WHATWG HTML timer
text is outside the allow list. Decided by: REC5's hidden-tab test in all four browsers, or a reading of the HTML
standard if the Advisor allows that domain.

**R1-TICK-OQ3. The tick rates of the brief's named references.** Overwatch's talk could not be read (GDC Vault
redirected to a maintenance page outside the allow list; video only), and Valve's networking pages returned HTTP 403.
Leads outside the allow list claim figures that are not cited here. Decided by: R2's reading of those sources, and the
consistency critic aligning both topics.

**R1-TICK-OQ4. Sub-tick input timestamps.** Whether shots carry their moment inside a tick, so a 30 Hz tick resolves
them as finely as 60 Hz. No source was read on this. Decided by: R2's lag-compensation unit; it matters only if REC2
falls back to 30 Hz.

**R1-TICK-OQ5. The real cost per tick of v2 at 48+.** No source read gives it; legacy's 0.80–1.00 ms per JS step
(TL15) is no prediction for a true-3D Rust sim with physics and abilities, and no Wasm-versus-native speed figure was
read. Decided by: REC2's P1–P3.

**R1-TICK-OQ6. The reference hardware.** VISION_V2 §4 names "a laptop with integrated graphics" and "a mid-range
graphics card", no CPU. P2 and P3 need a named minimum laptop CPU and P1 a named server. R6 proposes the server, one
vCPU of a CCX23 (R6-HOST-REC2; [GLOSSARY](GLOSSARY.md#the-reference-server): the reference server), and R3 the laptop
class (R3-BUDGET-REC1). Decided by: those proposals, approved by Mustafa.

**R1-TICK-OQ7. Offline at the largest mode?** The brief's principle says the client runs the sim for offline play; VISION
does not say whether the largest mode ([GLOSSARY](GLOSSARY.md#the-largest-mode-and-its-n); 48 soldiers under MODES-3
(a), 54 under (b), all but the player bots) must run offline on the minimum laptop. Decided by:
Mustafa and the Advisor; it sets N in P2.

**R1-TICK-OQ8. Client tick speed-up and slow-down.** Whether the client nudges its tick rate to keep its inputs arriving
just in time at the server. Not settled by any source read. Decided by: R2's time-sync design; REC5's loop must allow it.

**R1-TICK-OQ9. Big vehicles at 30 Hz.** If REC2 falls back to 30 Hz, do multi-part vehicles need physics substeps (two
Rapier steps per tick)? Rapier says smaller steps are more accurate and cost more (F8). Decided by: R4.

### 3. Identical results between Wasm and native (unit `determinism`)

- **R1-DET-OQ1.** What does `enhanced-determinism` cost, on native and in the browser? No source states it (F24).
  Decided by REC3's spike: step time with and without the flag at the largest mode.
- **R1-DET-OQ2.** Is Rapier's default 4-lane SIMD path (simba `wide`, F22) bit-identical between wasm32 (with and
  without `simd128`) and x86-64 or AArch64? No source read says so. Decided by REC6's suite with `simd128` off and then
  on; until then `simd128` stays off for the sim. rust-reviewer (2026-10-28) names a concrete hazard on this path:
  R1-RR-OQ1.
- **R1-DET-OQ3.** f32 or f64 for the sim (rapier3d or the f64 crates, F22)? It depends on the map size, which LOOP_V2 §9
  leaves [OPEN] (TL13, CS6 notes), and on precision at the far edge of the largest map. Decided by R4 with the map-size
  decision; the inventory's CS4 note asks for one choice, made once.
- **R1-DET-OQ4.** Which Wasm profile do browsers run? F3 leaves it to the host and no source read names the browsers'
  choice. It matters only if NaN can enter state, which REC2 forbids; REC6's browser runs settle it in practice.
- **R1-DET-OQ5.** Are rand's distribution outputs (for example uniform ranges) stable across rand versions? The
  docs.rs pages read do not say, and rand's book is outside the allow list. Decided by reading the book (when allowed)
  or by writing the sim's few sampling functions on raw generator output; either way the version is pinned and REC6's
  recorded matches catch a change. rust-reviewer (2026-10-28, risk 9): rand's uniform floats "may equal `high`", and
  the docs make no statement on value stability across versions; it advises the second route (map raw ChaCha8 output
  in the sim) and pinning rand_chacha with test vectors.
- **R1-DET-OQ6.** How much does the client predict: only the local player and the vehicle they drive, or more? R2
  decides. It sets how much cross-target identity buys (F32: corrections come only from what the client predicts).
  R2-SYNC-REC3's answer: own movement, own weapon state, own vehicle and cosmetic shot feedback; abilities are
  ABL-PREDICT (below).
- **R1-DET-OQ7.** Which CPU architecture do the servers use (x86-64 or AArch64)? R6 decides. Both are "fully
  compliant" (F12), but the chosen one must be in REC6's matrix; R1-RR-OQ1 gives a reason to test native AArch64
  before R6 picks it.
- **R1-DET-OQ8.** Is `libm`'s accuracy enough for aiming at height and line of sight (§6.2)? No accuracy bound is
  documented (F18). Decided by unit tests against reference values for the sim's functions.
- **R1-DET-OQ9.** Which hash function, and should production servers and clients compare hashes now and then to measure
  prediction divergence? R2 decides (it is protocol); this unit only needs a fixed, unseeded hash in tests. A hash
  compared with a client covers only the state the client holds, so it leaves authority-only streams out (R1-DET-REC4;
  [GLOSSARY](GLOSSARY.md#state-hash): state hash).
- **R1-DET-OQ10.** Does Wasm's `sqrt` instruction (what Rust's `sqrt` compiles to on wasm32, F17's `arch` list) match
  IEEE `squareRoot` exactly? F1 says Wasm follows IEEE 754, but the `sqrt` definition was not quoted in this run.
  Decided by REC2's unit vectors (and a re-read of the spec's `fsqrt` definition).
- **R1-DET-OQ11.** (added at the second fix round) If ChaCha8's cost shows up in REC4's profile, how would a Pcg32
  fallback keep the RNG's purposes apart? Its docs warn that "two generators with different stream parameters may be
  closely correlated" (F34), so REC4's one stream per purpose does not carry over as is, and no source read in this
  run says how to keep Pcg32 generators independent. Decided only if the profile asks for a fallback: by a source on
  PCG stream and seed independence, read then; until then ChaCha8 alone (REC4).

### 4. The Rust ↔ TypeScript boundary, and the simulation in a Web Worker (unit `boundary-worker`)

- **R1-BND-OQ1. Is offline (solo against bots) play a v2 need?** VISION_V2 §6 does not list it; §3 says bots fill
  empty slots, and the legacy game is PvE against bots. REC6 costs no extra boundary work if yes. Decides: Mustafa
  (VISION_V2).
- **R1-BND-OQ2. Does a worker's tick timer keep its rate in a hidden tab, in each of the four browsers?** Chrome's
  throttling article does not mention workers (F31). It matters for offline matches and for prediction while the
  player has tabbed away. Decides: a spike measurement (tick rate with the tab hidden for more than 5 minutes, per
  browser).
- **R1-BND-OQ3. What is the minimum reference machine, and how many cores does it give the page?**
  `hardwareConcurrency` may under-report (F32), and no source read here names the integrated-graphics laptop of
  VISION_V2 §4. Decides: the ADR names a model; the spike records its core count and measures page and worker
  contention.
- **R1-BND-OQ4. Which tsify line, if any, is maintained?** Two forks exist (F8). Only matters if ts-rs proves short.
  Decides: reading both repositories' activity (github.com, allowed) in the synthesis.
- **R1-BND-OQ5. Will the game page ever need third-party embeds** (video, ads, analytics, social or payment widgets)
  **or cross-origin popups?** Each would need CORP under isolation (CORS as another route is our reading, not in
  F21's quotes), or a separate page (F21, F23). Decides: Mustafa (VISION_V2 §5) with R6.
- **R1-BND-OQ6. How big is one output block for the largest mode, and is the layout per entity or per field?** It
  depends on the entity caps (soldiers, multi-part vehicles, projectiles, events), on R2's snapshot contents and on
  R3's instancing needs. It decides whether the one copy per tick matters. Decides: the spike measures bytes per
  tick and copy time at the largest mode's N (48, or 54 under MODES-3 (b)).
- **R1-BND-OQ7. Is wasm-bindgen's new home healthy for a multi-year project?** It moved in 2025 and broke
  compatibility at least twice in 2026, in 0.2.123 and 0.2.128 (F1, F2; the unit's "three times" counted 0.2.122,
  which the fix step dropped). Decides: reading the maintenance issue (#4533) and the guide's
  governance page in the synthesis; pinning versions limits the risk meanwhile.
- **R1-BND-OQ8. Does the server's snapshot reach the worker in the same layout as the page's block?** Reusing it
  would save a translation but couples the wire format to presentation. Decides: R2 (snapshot format) with ADR-0001.
- **R1-BND-OQ9. Can `same-origin-allow-popups` give cross-origin isolation?** (added at the fix step) MDN's COOP
  page says the value serves "integrations where a document needs the benefits of cross-origin isolation but also
  needs to open and retain a reference to trusted cross-origin documents", while its isolation section asks for
  `same-origin` (F20, F23). If it can, a login popup could stay on the isolated game page (REC5). Decides: the HTML
  standard's isolation rules (html.spec.whatwg.org, outside the allow list) or a test in each of the four browsers
  (`crossOriginIsolated` under that value).

### 5. A data-driven ability and effect system (unit `abilities`)

- **R1-ABL-OQ1.** The official GAS documentation (dev.epicgames.com, outside the allow list) and the engine source
  (private, HTTP 404) were not read: every GAS fact here is one community write-up's. What decides it: allowing the
  Epic documentation domain for one verifier pass, or accepting the write-up as the reference.
- **R1-ABL-OQ2.** Overwatch's "Networking Scripted Weapons and Abilities" (GDC 2017, per a search result) is the closest
  shipped team-shooter precedent for scripted, networked abilities, and was not read (GDC Vault redirected to a
  maintenance page outside the allow list; the talk is video). It would show whether a scripting layer (A3) paid off in
  a shipped game. What decides it: reading the talk or its slides.
- **R1-ABL-OQ3.** Valve's developer wiki pages on prediction and on Dota 2's data-driven abilities returned HTTP 403:
  how Source suppresses repeated effects during re-prediction, and how Dota 2 shapes modifiers as data, are unread. What
  decides it: another fetch route to those pages, or leave REC4's cue rule as an inference to be tested in the spike.
- **R1-ABL-OQ4.** Per-effect prediction keys (GAS, F14) or re-simulation of the local player from the last confirmed
  state: which one depends on R2's prediction model and on the Worker boundary unit. What decides it: R2's
  recommendation. R2-SYNC-REC3's model is the replay from the authoritative state (Quake III's method); for abilities
  the question rides with ABL-PREDICT (below).
- **R1-ABL-OQ5.** Attribute math in floats or fixed-point, and the exact aggregation order (F4): the determinism unit
  decides whether floats are bit-identical on Wasm and native.
- **R1-ABL-OQ6.** Rules the system must support but not decide (Mustafa's): stacking of slows and marks; whether health
  regen stays (GC19); whether the Bayonet and the Torque Bow stay (GC11, GC15); active reload numbers (LOOP_V2 §7);
  the Hammer's "item" and leech details ([LATER]); what marking does (§6.4); seats per vehicle ([OPEN]); one weapon
  model for bots and humans (CD13). The spike uses legacy behaviour as its test case only.
- **R1-ABL-OQ7.** Fire intervals shorter than one tick: fire several shots in one tick or cap at one per tick? Legacy's
  fastest intervals (Shabzon 0.072 s, Lancer burst 0.06 s) are longer than a 30 Hz tick (0.033 s) but not a 10 Hz one.
  What decides it: the tick-rate choice.
- **R1-ABL-OQ8.** Who sees which tags: marking and the Spy's disguise (LOOP_V2 §5, [LATER]) need tags visible to some
  teams only. What decides it: R2's interest management.
- **R1-ABL-OQ9.** Whether a Rust-embeddable script runtime compiles to Wasm and runs deterministically was not
  researched; it matters only if A3 comes back after OQ2.
- **R1-ABL-OQ10.** Effects as their own entities (bevy_alchemy's approach, F25) or as a list on the target: the ECS unit's
  choice decides it.

### 6. Data formats and hot reload of tuning (unit `data-hotreload`)

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
  and R5's build research. REC6's hash check needs a secure context (F29); whether a `file://` page is one was not in
  this unit's reading. The second verifier pass (2026-10-28) reports that MDN's Secure contexts page (last modified
  2026-09-14) lists "`file:///path/to/resource.html` ✅ Secure (`file` URL)", so the fix step's link from the
  secure-context rule to file:// was withdrawn at the second fix round; whether the worker and the Wasm load from
  `file://` stays this question.
- **R1-DATA-OQ12. Mid-match reload of simulation numbers in development.** (added at the fix step) REC5 swaps sim
  numbers inside a running match, on the server and the client at one tick; no VISION_V2 §6 item names live reload
  (§6.3 asks for tuning without code changes; LOOP_V2 §7's live Feel Lab covers client feel data). Is a swap at the
  next match enough for development? Decides it: Mustafa and the Advisor, weighing iteration speed against the cost
  of REC5's marks 2 and 3.

### Conflicts and gaps between the units (added at assembly)
Each entry names where two or more unit drafts disagree, or where one unit left a question to another unit that the
other draft does not take up. Both sides stay as written above; none is resolved here. The consistency critic and the
synthesis (`docs/ENGINE_OVERVIEW.md`, `docs/adr/0001`–`0006`) take them up.

- **R1-XU-OQ1. The stress scene and the server tick budget differ between units.**
  - R1-ECS-REC1 mark 1: 48 soldiers (bots), 8 vehicles, 300 live projectiles and 200 active effects; mean sim step
    time ≤ 2 ms and p99 ≤ 4 ms native; Wasm in a Worker ≤ 2× native.
  - R1-TICK-REC2 P1: N = 12 / 24 / 36 / 48 / 64 soldiers plus 4 big vehicles and 200 live projectiles; at 60 Hz, p99 at
    N = 48 at most 4.2 ms, max at most 16.7 ms, zero overruns (at 30 Hz, p99 at most 8.3 ms). P2: Wasm in a Worker, p99
    at most 40 % of the tick period.
  - R1-DET-REC3: "48 characters plus the vehicles VISION_V2 §6.4 implies, count set by R4".
  - R1-ABL-REC6: 48 soldiers, each with up to 8 active effects; the ability, effect and weapon step at most 1 ms per
    tick.
  - R1-BND-REC1: the LOOP_V2 §11 scale points N = 12, 24, 36, 48.
  What decides it: the synthesis picks one stress scene and one split of the server tick budget; until then every
  figure stays as each unit proposed it. Settled by the shared glossary (2026-10-30): one stress scene and one server
  tick budget with its shares ([GLOSSARY](GLOSSARY.md#1-tick-budgets)), which the marks above now use.
- **R1-XU-OQ2. Whole-world or local replay, and its budget.** R1-ECS-REC1 mark 3 restores and re-simulates the whole
  world for k = ceil(0.15 s × tick rate) ticks within 8 ms in Wasm on the minimum laptop. R1-TICK-REC2 P3 re-simulates
  "the local soldier, or the vehicle it drives, for 200 ms of game time" within 1 ms p99. R1-ABL-REC4 predicts the local
  player's own weapons, and its own abilities if ABL-PREDICT says so (option D2). Three units ask the same question:
  R1-ECS-OQ3, R1-DET-OQ6,
  R1-ABL-OQ4. What decides it: R2's prediction recommendation, as all three say.
- **R1-XU-OQ3. Shared memory or a transferred buffer for what the page draws.** R1-TICK-REC5 has the main thread draw
  "every frame from the two newest ticks with `alpha` (F4), read from shared memory (boundary unit)", and the tick and
  ecs cross-unit notes speak of the tick states and the render-snapshot buffer "in shared memory". R1-BND-REC2
  recommends one flat block per tick "handed to the page as one transferred ArrayBuffer from a small pool (B2)", with
  the SharedArrayBuffer ring (B3) only as the measured alternative, and R1-BND-REC5 does not require cross-origin
  isolation at the start (shared memory needs it, R1-BND-F12). What decides it: R1-BND-REC2 mark 5 (B2 against B3)
  and the synthesis.
- **R1-XU-OQ4. The Worker's pacing fallback and how inputs arrive.** R1-TICK-REC5 keeps W3 (`Atomics.wait`) as "the
  measured alternative if W2 fails the jitter mark"; the tick unit's option W3 needs `SharedArrayBuffer` and says
  "inputs must arrive through shared memory" (tick options 4d, from R1-TICK-F22). R1-BND-REC3 posts inputs as one
  message per tick (C1), and R1-BND-REC5 adopts isolation only on the B2-against-B3 result, not on a pacing result.
  What decides it: R1-TICK-REC5's jitter mark and R1-BND-REC2 mark 5, read together in the synthesis.
- **R1-XU-OQ5. Is the std `HashMap` seed per instance or per process?** R1-ECS-F23 quotes "each HashMap instance uses
  a different seed"; R1-DET-F25's summary says "randomly seeded per process", and R1-DET-REC5's pass mark relies on
  "std's `HashMap` seed differs per process". Both cite the same page
  (https://doc.rust-lang.org/std/collections/struct.HashMap.html). What decides it: the verifier, at the source.
  Settled at the fix step: per instance (R1-DET-F33; verifier and rust-reviewer, 2026-10-28). R1-DET-F25, REC5 and
  REC6 were corrected.
- **R1-XU-OQ6. One source, two tiers.** Glenn Fiedler's "What Every Programmer Needs To Know About Game Networking"
  (https://gafferongames.com/post/what_every_programmer_needs_to_know_about_game_networking/) is "other" in
  R1-TICK-F24 and "shipped game (practitioner article)" in R1-DET-F32. The tick unit labels all four of its Fiedler
  articles "other"; the determinism unit labels "Floating Point Determinism" "shipped game" because it quotes
  shipped-game developers (R1-DET-F31). What decides it: the verifier, against the tiers in `docs/research/README.md`.
  Settled at the fix step: both are tier "other". docs/research/README.md:22–23 has three tiers (official docs, talks
  and papers from shipped games, everything else), and blog articles are the third (verifier, 2026-10-28); R1-DET-F31
  and F32 were relabelled, and F31 notes that it quotes shipped-game developers.
- **R1-XU-OQ7. Fire rates: two time representations, two pass marks.** R1-TICK-REC1 converts durations "to ticks at
  load, with remainders carried" (fire intervals "keep their fractional remainder"); its mark: every legacy
  `fireInterval` and `burstInterval` gives the same shot count at 30 Hz and 60 Hz within 1 shot over 60 s, and within
  1 shot of 60 / interval. R1-ABL-REC3 converts them "into an integer time unit in which the tick step is exact"; its
  mark: the Shabzon's sustained rate within 1 % of 13.9 shots per second at the recommended tick. The drafts do not
  say whether the two representations are the same. What decides it: the synthesis (one representation, one mark).
- **R1-XU-OQ8. Where content's TypeScript types come from.** R1-BND-REC4 generates types for "low-rate messages,
  protocol and content-schema types" with ts-rs (D3). R1-DATA-REC3 generates JSON Schema 2020-12 with schemars and builds the
  editor's inspector from it, and the data unit's cross-unit note asks "whether TypeScript types for content come from
  the same schema (REC3) or from their type generator". What decides it: the content-data decision draft
  (`docs/adr/0005`), with R5.
- **R1-XU-OQ9. How definitions are laid out in files.** R1-ABL-REC5 (option E1) has "One table per kind, linked by
  validated ids" and leaves "the file format, schema tooling and hot reload" to the data unit; R1-DATA-REC1 has "one
  definition per file". The drafts do not say whether a kind's table is one file or a set of per-definition files.
  What decides it: `docs/adr/0005`.
- **R1-XU-OQ10. Gap: may the tick constant change on hot reload?** The tick unit's cross-unit note says "the tick
  constant underlies every converted duration; whether it may change on hot reload is that unit's call" (the data
  unit's). The data draft does not take it up: R1-DATA-REC5 and R1-DATA-OQ7 list what a reload may change without
  the tick. What decides it: still open; the synthesis.
- **R1-XU-OQ11. Gap: does an exactly representable `dt` matter for identical results?** R1-TICK-OQ1 (60 Hz or 64 Hz)
  names "the determinism unit (does an exactly representable `dt` matter for identical Wasm and native results?)" as
  one of its deciders; the determinism draft does not take it up. What decides it: still open; R1-TICK-OQ1's other
  decider (R1-TICK-REC5's smoothness test) and the synthesis.
- **R1-XU-OQ12. The same open question in two units, with different test conditions.**
  - Hidden tabs: R1-TICK-OQ2 and R1-BND-OQ2 ask whether worker timers keep their rate in a hidden tab. R1-TICK-REC5's
    mark hides the tab for 60 s; R1-BND-OQ2 asks for "the tab hidden for more than 5 minutes", the point after which
    Chrome's intensive throttling applies (R1-TICK-F18, R1-BND-F31).
  - Reference hardware: R1-TICK-OQ6 and R1-BND-OQ3 (and the proposed machines in R1-ECS-REC1, R1-TICK-REC2 and
    R1-ABL-REC6).
  - Offline play: R1-TICK-OQ7 asks whether the largest mode must run offline on the minimum laptop; R1-BND-OQ1 asks
    whether offline play is a v2 need at all (VISION_V2 §6 does not list it). Both name Mustafa (with the Advisor in
    R1-TICK-OQ7) as the decider.
    What decides it: as each entry says; the synthesis states each question once.

### Rust risks the rust-reviewer found (added at the fix step)
rust-reviewer (2026-10-28; verdict REVISE for R1: the facts hold, Rust risks are missing) ranked risks the
recommendations do not cover. The fix step adds no recommendation and no claim of its own (fix rule 6), so each risk
is an open question here, with the evidence as the reviewer reported it (its notes:
`.checkpoints/v2-research/rust-reviewer-r1-r2.md`; not re-checked by the verifier); the synthesis decides whether
each joins a recommendation. Risk 5 (snapshot allocation) was a pass mark that could not pass and is fixed in place
(R1-ECS-REC1 mark 2, R1-ECS-F35); risk 9 joined R1-DET-OQ5; risks 1 and 10 are R2's, apart from the tokio entry in
R1-RR-OQ7.

- **R1-RR-OQ1. FMA that differs by target in Rapier's default 4-lane SIMD path (risk 2).** The reviewer reports that
  wide 1.7.1's `mul_add` fuses where the target has FMA (x86 built with `+fma`, AArch64 with NEON) and rounds twice
  elsewhere, with no Wasm branch ("Platform-specific behavior (may change in the future)"); that simba 0.10.2's
  `simd_mul_add` calls it and `libm_force` switches only sin and cos; that Rapier's manifest suggests
  `-C target-cpu=native`; and that wide's `max` and `min` also differ at ±0 between SSE, NEON and the fallback. Not
  established: that the solver reaches this code (the two solver files it checked do not call `mul_add`). Bears on
  R1-DET-REC3, R1-DET-REC6's matrix, R1-DET-OQ2 and R1-DET-OQ7. The reviewer's proposal: build the server for
  baseline x86-64 (no `+fma`, no `target-cpu=native`), add native AArch64 to the native-against-Wasm hash test
  before R6 may pick ARM hosts, and use glam's `scalar-math` if the sim uses glam itself. Decides it: the synthesis
  (ADR-0004), then R1-DET-REC6's suite.
- **R1-RR-OQ2. Allocations a client or a content file controls (risk 3).** R1's content pipeline caps file bytes but
  not array lengths (R1-DATA-REC2, REC7). The reviewer's proposal: length rules on collections (and, on R2's side,
  bounded per-client queues and message caps). Decides it: the synthesis (ADR-0005) with the security review.
- **R1-RR-OQ3. Panics reachable from content and input (risk 4).** wasm32 builds with `-Cpanic=abort` (R1-BND-F19),
  so one panic ends the client sim's instance, and R1 plans no recovery; `f32::clamp` panics on bad bounds
  (R1-DET-F14). The reviewer's proposal: the content loader and decoders return errors and are fuzzed with zero
  panics as the pass mark, and the Worker restarts from the server's snapshot after a trap. Bears on R1-BND-REC1,
  REC6, R1-DATA-REC2 and R1-DET-REC2. Decides it: the synthesis (ADR-0002, ADR-0004).
- **R1-RR-OQ4. Overflow behaviour differs between build profiles (risk 6).** The reviewer reports Cargo's defaults as
  `[profile.dev] overflow-checks = true` and `[profile.release] overflow-checks = false`, so a debug client and a
  release server can diverge, or one of them panics. R1-DET-REC2 already asks for one overflow setting in tests and
  explicit wrapping or checked arithmetic for counters; the reviewer adds overflow checks for the sim package only
  and a lint. Decides it: ADR-0002 (build profiles) with R5.
- **R1-RR-OQ5. No budget for Wasm size and speed (risk 7).** R1 records the size but sets no limit (R1-ECS-OQ1); the
  content crate in the Worker brings serde_json, garde and regex; std `mul_add` has no hardware instruction on wasm32
  and runs in software (the reviewer's inference), which matters because R1-DET-REC2 allows explicit `mul_add`. The
  reviewer's proposal: measure wasm-opt, `opt-level` "s" against "z" and the panic strategy, before and after.
  Decides it: the spike's measurements, then the ADR's budget.
- **R1-RR-OQ6. Hashing state and writing the output block (risk 8).** `repr(C)` padding bytes are uninitialised, so a
  state hash must read fields, never raw struct bytes (R1-DET-REC6 already serializes field by field), and writing
  R1-BND-REC2's block safely needs a derive that rejects padding, or `unsafe` code with a written invariant and a
  test. Decides it: rust-reviewer's crate pick in the synthesis.
- **R1-RR-OQ7. Dependency floors and advisories (risk 1, the R1 part).** The reviewer reports RUSTSEC-2026-0384, a
  race in `tokio::time::Interval` on Tokio 1.54.0, patched in 1.54.1 (the version R1-TICK-F23 read and R1-TICK-REC4's
  loop would use); its wider list (quinn-proto, rustls, ring) is R2's. The reviewer's proposal: lockfile floors, and
  cargo-audit and cargo-deny as CI gates. Decides it: R5's CI research and the synthesis. R5-GATE-REC1 schedules a
  cargo-deny advisories check and runs it on every PR that changes `Cargo.lock`
  ([GLOSSARY](GLOSSARY.md#the-advisories-check): the advisories check); whether an advisory blocks a PR is
  R5-GATE-OQ4, left to [the security baseline](GLOSSARY.md#the-security-baseline). Shared as BASELINE-WHEN (below).

### Shared open questions (the same words in every topic)
Added 2026-10-30 (step 3c): the decision questions the consistency resolutions share between topics, worded as
approved (`.checkpoints/v2-research/resolutions.md`, advisor verdict of 2026-10-30), each headed by its decision code,
so a search for the code finds every copy.
- **ABL-PREDICT (Mustafa, after a Feel Lab test). Does the client predict the local player's own abilities?** The
  prediction scope ([GLOSSARY](GLOSSARY.md#the-prediction-scope)) covers the player's own movement, own weapon state,
  own vehicle and cosmetic shot feedback (R2-SYNC-REC3, option 3.2 C), and never damage. R1-ABL-REC4 (option D2) would
  add the player's own ability activation, cost, cooldown, granted tags and movement changes (Banzai speed, Shield
  slow). No source on predicting abilities and rolling them back was read (R2-SYNC-OQ10). If yes, the ability system
  uses R1-ABL-REC4's prediction key (the command's sequence number), undoes a rejected press and keys cues so that a
  re-simulated activation does not play twice; whether that runs through per-effect keys or through R2-SYNC-REC3's
  replay from the authoritative state is R1-ABL-OQ4. If no, ability presses wait for the server's confirmation and
  R1-ABL-REC4's ability marks drop out. Decided by: R2-SYNC-OQ10's Feel Lab comparison of predicted against
  server-confirmed Molotov and Shield (or a readable source), then Mustafa (game feel). Also asked as R2-SYNC-OQ10 and
  R1-ABL-OQ4.
- **SEC-SPREAD (Mustafa). Matching tracers, or cheat resistance?** A spread seed the client knows makes its predicted
  tracers match the server's spread, but lets a cheat cancel the spread through the aim point it sends (R2-SYNC-OQ11,
  the security review's finding H2). The design keeps weapon spread in an authority-only stream
  ([GLOSSARY](GLOSSARY.md#shared-stream-and-authority-only-stream); R1-DET-REC4, R2-SYNC-REC3): the client draws its
  tracers from a cosmetic spread and the hit event corrects them. If Mustafa picks matching tracers, spread moves to a
  shared stream, inside the client's state and its compared hash; nothing else in the stream design changes. Also
  asked as R2-SYNC-OQ11. Decided by: Mustafa.
- **MODES-3 (Mustafa). How are VISION_V2 §3's starting sizes read?** VISION_V2 §3 says "two teams with 2–8 players,
  and three teams with 6–18 players" to start, and "The goal: 12v12v12 or 24v24". (a) Totals: 1v1 to 4v4, and 2v2v2 to
  6v6v6; the largest mode is then the goal 24v24, N = 48. (b) Per team: 2v2 to 8v8, and 6v6v6 to 18v18v18; the largest
  mode is then 18v18v18, N = 54. The answer sets the largest mode's N
  ([GLOSSARY](GLOSSARY.md#the-largest-mode-and-its-n)), which the stress scene, the offline scale test (R1-TICK-OQ7),
  the soak runs (R5-TEST-REC5) and the cost rows (R6-COST) use, and items 3 and 4 of the test-mode list
  ([GLOSSARY](GLOSSARY.md#the-test-mode-list)). Until it is answered, a mark that needs the largest mode's N gives
  both values. Also asked as R2-BUD-OQ6, R2-XU-OQ13 and R3-BUDGET-OQ4. Decided by: Mustafa.
- **BASELINE-WHEN (the Advisor). Research the deferred security baseline before ADR 0006, or list it open there?** The
  security baseline ([GLOSSARY](GLOSSARY.md#the-security-baseline): content security policy, dependency audits,
  anti-cheat basics) was deferred by the Advisor on 2026-10-27, and "ADR 0006 lists it as open, or it is researched
  before 0006 is accepted" (R6-SEC-OQ1). Two cross-topic disagreements wait on it: who owns the advisories check
  ([GLOSSARY](GLOSSARY.md#the-advisories-check)) and whether an advisory blocks a pull request (R1-RR-OQ7, R2-TRN-REC1
  mark 5, R5-GATE-REC1, R5-GATE-OQ4, R6-SEC-OQ1), and one wildcard certificate whose private key sits on every game VM
  (R6-HOST-REC4) against a certificate per game-server hostname (R2-TRN-REC5). (a) Research it now, as one
  security-auditor topic before the drafts. (b) List it open in ADR 0006 with interim positions: R5-GATE-REC1's
  scheduled advisories check stays the mechanism and the baseline sets the policy later; R2-TRN-REC5's certificate per
  hostname stays, and R6's wildcard becomes an option the baseline weighs. (c) Leave both as one shared open question.
  Decided by: the Advisor.

### Cross-topic open questions (consistency critic, 2026-10-29)
Added 2026-10-30 (step 3c): the consistency critic's hidden dependencies and gaps that touch this topic, in the
critic's words (`.checkpoints/v2-research/cross-topic-oqs.md`, approved by the advisor on 2026-10-30), each with its
id and both sides' ids; the recommendations they name point here.
- **R1-XT-OQ1. Who casts the cursor ray for the aim point, and how does the point cross the boundary?** The aim point
  must come from a Rust ray query against what the page draws (interpolated positions live on the page). But R1 allows
  the page no Wasm calls and no sim maths in TypeScript, and R1's command carries yaw and pitch, not a point. No topic
  says which side casts the cursor ray, against which interpolated state, or how the result crosses the boundary. Side
  A: R1-BND-REC1; R1-BND-REC3; R1-DET-REC2. Side B: R2-SYNC-REC8; R4-LOS-REC3. Existing questions: none. Decided by:
  ADR-0001 (the command format) and ADR-0004 (the boundary), with R2-SYNC-REC8 mark 1 and R1-BND-REC1 mark 2 as the
  spike marks. (Consistency critic, 2026-10-29, rank 2.)
- **R1-XT-OQ2. Which R1 items still assume Rapier, now that R4 recommends parry3d?** R4 states openly that some of
  R1's Rapier items change. It does not list R1-TICK-REC2's reason for starting at 60 Hz (Rapier's default step),
  R1-ECS-REC1's reason and mark 5, R1's TL;DR, or whether R1-RR-OQ1's SIMD/FMA hazard applies to parry3d's "3D f32
  SIMD" BVH (R4-PHYS-OQ3 asks only by analogy). ADR-0004's determinism and snapshot sections rest on whichever crate
  wins. Side A: R1-DET-REC3; R1-ECS-REC4; R1-ECS-REC1; R1-TICK-REC2; R1-RR-OQ1. Side B: R4-PHYS-REC1. Also named:
  R4-PHYS-OQ3. Existing questions: R4 'Wave-1 items R4 changes or answers' (r4:1110-1129 at commit 8dee746) and
  R4-PHYS-OQ3, partly. Decided by: ADR-0004 (sim core: physics), with R4-PHYS-REC1 marks 2-5 and R1-DET-REC6's suite
  as the evidence. (Consistency critic, 2026-10-29, rank 3.)
- **R1-XT-OQ3. Where does the remote-entity interpolation buffer live: in the worker or on the page?** A two-block
  buffer on the page cannot hold R2's 100 to 167 ms delay. So either the worker (Rust) runs R2's adaptive snapshot
  buffer and publishes already-delayed remote poses, or the page (TypeScript) keeps a snapshot buffer, as R5's test
  layer assumes. No topic places R2's interpolation and clock estimate on one side of R1's boundary. Side A:
  R1-TICK-REC5; R1-BND-REC2; R1-BND-REC6. Side B: R2-SYNC-REC4; R5-TEST-REC1. Existing questions: none. Decided by:
  ADR-0001 (interpolation design) with ADR-0004 (boundary: block contents and the number of blocks kept). (Consistency
  critic, 2026-10-29, rank 4.)
- **R1-XT-OQ4. Does the E3 projectile encoding rest on the cross-target identity R1 says the netcode must not need?**
  R2's chosen encoding and its bandwidth figure (146 rather than 188 kbps) rest on client-side flight that matches the
  server across targets. R1 says the netcode must not depend on that identity, and R1 never answers R2-BUD-OQ8, the
  question R2 put to it. Side A: R1-DET-REC1. Side B: R2-BUD-REC2; R2-SYNC-REC6; R2-BUD-OQ8. Existing questions:
  R2-BUD-OQ8 (unanswered by R1). Decided by: ADR-0001 (E3 or E3b) with ADR-0004 (whether cross-target identity is a
  guarantee for the trajectory function); R2-SYNC-REC6 mark 1 and R2-BUD-REC2's desync count in the spike.
  (Consistency critic, 2026-10-29, rank 7.)
- **R1-XT-OQ5. Who answers R1's timing questions (sub-tick input timestamps, client tick nudging, shipped tick
  rates)?** R1 leaves three timing questions to R2's design, and R2 does not take them up: which tick a client stamps
  and how it stays just ahead of the server is part of the command path both ADRs rely on. Neither topic could read
  the shipped references, so R1-TICK-OQ3 cannot be aligned. Side A: R1-TICK-OQ3; R1-TICK-OQ4; R1-TICK-OQ8. Side B:
  missing answer (R2-BUD-OQ1, R2-BUD-REC4 named). Existing questions: R1-TICK-OQ3, R1-TICK-OQ4, R1-TICK-OQ8;
  R2-BUD-OQ1. Decided by: ADR-0001 (time sync and command stamping); the R2-SYNC-OQ3 spike; R1-TICK-OQ4 only if the
  tick falls back to 30 Hz. (Consistency critic, 2026-10-29, rank 12.)
- **R1-XT-OQ6. Who answers R1's protocol questions (hash comparison, snapshot layout, mid-match tuning, the content
  hash at join)?** R1 hands these protocol choices to R2 by id, and R2 answers none of them. The join message R2 does
  propose does not carry the content hash R1's bundle check and R6's match record (r6:757 at commit 8dee746) both rely
  on. Side A: R1-DET-OQ9; R1-BND-OQ8; R1-DATA-OQ3; R1-DATA-REC6. Side B: missing answer. Also named: R2 room control
  messages. Existing questions: R1-DET-OQ9, R1-BND-OQ8, R1-DATA-OQ3. Decided by: ADR-0001 (join handshake, snapshot
  format, divergence telemetry) with ADR-0005 (content hash). (Consistency critic, 2026-10-29, rank 13.)
- **R1-XT-OQ7. Is whole-world replay still a spike mark?** R2 answers R1's question by subject without citing it, and
  R4 already builds on the local-only answer (no navigation in prediction). R1's whole-world replay mark (and the
  snapshot cost it sizes) is still a proposed spike mark for a model no topic recommends. Side A: R1-ECS-REC1;
  R1-XU-OQ2. Side B: R2-SYNC-REC3; R2-SYNC-REC9; R4-NAV-REC4. Also named: R1-ECS-OQ3. Existing questions: R1-XU-OQ2
  (with R1-ECS-OQ3, R1-DET-OQ6, R1-ABL-OQ4). Decided by: ADR-0001 (prediction scope), then ADR-0004 keeps or drops
  R1-ECS-REC1 mark 3. (Consistency critic, 2026-10-29, rank 14.)
- **R1-XT-OQ8. Do browser navigation and server-less previews rest on an undecided offline-play need?** R4 builds
  navigation for the browser, R6 serves previews without a game server, and R5 plans the first previews, all on
  offline play, which R1 leaves as an open VISION question (and, at the largest mode, as R1-TICK-OQ7). Side A:
  R1-BND-OQ1. Side B: R4-NAV-REC4; R6-HOST-REC3; R5-PREV-OQ3. Also named: R1-TICK-OQ7, R1-XU-OQ12. Existing questions:
  R1-BND-OQ1, R1-TICK-OQ7, R1-XU-OQ12. Decided by: Mustafa and the Advisor (VISION_V2), then ADR-0004 and ADR-0006.
  (Consistency critic, 2026-10-29, rank 15.)
- **R1-XT-OQ9. Do the snapshot and the output block carry remote soldiers' animation state?** Animation of remote
  soldiers online needs those states in R2's snapshot, then in R1's block. R2's bit budget names none of them and R1
  leaves the layout open, so R3's character path and R2's bandwidth figure may not fit together. Side A: R1-BND-REC2.
  Side B: none by id. Also named: R3-CHAR-REC3, R3-CHAR-REC4, R1-BND-OQ6, R2 worked estimate. Existing questions:
  R1-BND-OQ6 (block size and layout), partly. Decided by: ADR-0001 (snapshot fields) with ADR-0004 (block layout) and
  ADR-0003 (what animation reads). (Consistency critic, 2026-10-29, rank 16.)
- **R1-XT-OQ10. Does the editor still open by double-click with a Wasm validator?** R1 hands this question to R5, and
  R5's editor build does not address the double-click rule EDITOR_V2 keeps. Side A: R1-DATA-OQ11. Side B: missing
  answer (R5-LAYOUT-REC5, R3-WGPU-REC4 named). Existing questions: R1-DATA-OQ11. Decided by: ADR-0002 (editor build),
  or Mustafa if EDITOR_V2 §5's double-click rule must change. (Consistency critic, 2026-10-29, rank 25.)
- **R1-XT-OQ11. Do the wire's entity ids and tick counter fit R1's generational ids and a 20-minute match?** R2's byte
  budget has no room for R1's generational ids, and with a 16-bit tick a 20-minute match at 60 Hz wraps. R2 does not
  answer either pointer. Side A: R1-ECS-REC3. Side B: R2-ROOM-REC1. Also named: R1 pointer to R2 (tick wrap), R2
  worked estimate. Existing questions: none. Decided by: ADR-0001 (wire id and tick widths), recomputing R2-BUD-REC2's
  estimate. (Consistency critic, 2026-10-29, rank 27.)
- **R1-XT-OQ12. Who owns recovery from a trap in the client's Wasm sim?** R2 says R1 owns client-trap recovery; R1
  leaves it to the synthesis. No topic recommends it. Side A: none by id. Side B: R1-RR-OQ3; R6-OBS-REC3. Also named:
  R2 pointer to R1 (trap recovery). Existing questions: R1-RR-OQ3. Decided by: ADR-0004 (worker restart) with ADR-0001
  (resync path, R2-XU-OQ6) and ADR-0002 (panic strategy). (Consistency critic, 2026-10-29, rank 28.)
- **R1-XT-OQ13. Where do content's TypeScript types come from?** R5 gives the outputs one home but does not choose
  between the two sources for content types. Side A: R1-XU-OQ8; R1-BND-REC4; R1-DATA-REC3. Side B: R5-LAYOUT-REC3.
  Existing questions: R1-XU-OQ8. Decided by: ADR-0005 (content data), with ADR-0002. (Consistency critic, 2026-10-29,
  rank 31.)
- **R1-XT-OQ14. Who owns audio occlusion rays, if option O-B is chosen?** Page-side audio rays would need the sim's
  query across a boundary that allows no page calls. This only matters if option O-B is adopted. Side A:
  R3-AUDIO-REC3; R3-AUDIO-OQ5. Side B: missing answer (R4-LOS-REC1 named). Also named: R1-BND-REC1. Existing
  questions: R3-AUDIO-OQ5. Decided by: ADR-0003 with ADR-0004, only if R3-AUDIO-REC3's spike asks for O-B.
  (Consistency critic, 2026-10-29, rank 35.)
- **R1-XT-OQ15. Which bench method measures R1's budget marks?** R1's budget marks wait on a measuring method R5 has
  not chosen. Side A: R1-ABL-REC6. Side B: R5-TEST-REC1; R5-TEST-OQ8. Also named: R1 pointer to R5 (tick benches).
  Existing questions: R5-TEST-OQ8. Decided by: ADR-0002 (bench tool and method). (Consistency critic, 2026-10-29, rank
  36.)
- **R1-XT-OQ16. Does R3 need OffscreenCanvas, or a per-entity or per-field block layout?** R1 asks R3 for two inputs
  to the block layout, and R3 gives neither. Side A: R1-BND-OQ6. Side B: missing answer. Also named: R1 pointer to R3.
  Existing questions: R1-BND-OQ6. Decided by: ADR-0003 with ADR-0004 (block layout), measured in the R1-BND-REC2
  spike. (Consistency critic, 2026-10-29, rank 37.)
- **R1-XT-OQ17. Who answers R1-BND-OQ5 (embeds, cross-origin popups, CORP and the login flow under isolation)?** These
  are part of R1-BND-REC5's isolation cost, which R6 was asked to report. Side A: R1-BND-OQ5. Side B: none by id. Also
  named: R1-BND-REC5, R1 pointer to R6. Existing questions: R1-BND-OQ5. Decided by: Mustafa with ADR-0006, only if
  production isolation is adopted (R1-BND-REC2 mark 5). (Consistency critic, 2026-10-29, rank 38.)
- **R1-XT-OQ18. Does R5's cache question size a workspace no topic proposes?** R5's cache question assumes crates R1
  and R4 recommend against, so it sizes a workspace no topic proposes. Side A: R5-CI-OQ1. Side B: R1-ECS-REC1;
  R4-PHYS-REC1. Existing questions: none. Decided by: ADR-0002 (crate list), then the cache spike (R5-CI-REC1 mark 2).
  (Consistency critic, 2026-10-29, rank 40.)

### Pointers between R1's units
From the units' cross-unit notes; where a note shows a disagreement or a gap, it is an open question above (R1-XU-…).
- **ecs and tick:** R1-ECS-REC1 marks 1 and 3 take the tick rate as an input; the state struct holds the tick counter
  (R1-ECS-REC2).
- **ecs and determinism:** R1-ECS-REC1 mark 4 and R1-ECS-REC2 mark 1 reuse the per-tick state hash of R1-DET-REC6;
  Rapier's enhanced-determinism claim (R1-ECS-F31) is the determinism unit's to test (R1-DET-REC3). The determinism
  unit asks, if bevy_ecs or hecs were chosen, whether its query order is deterministic: R1-ECS-F4 and R1-ECS-F17 say
  the order is not guaranteed.
- **ecs and boundary-worker:** how the fixed-slot tables map onto the output block (R1-XU-OQ3); bevy's multi-threaded
  executor inside a Worker is not needed by R1-ECS-REC1, and R1-BND-REC1 runs one single-threaded sim.
- **ecs and abilities:** R1-ECS-REC1 assumes effects are rows in an ordered pool with R1-ECS-REC3 ids, and attributes
  are arrays keyed by ids from data; the ability unit designs the rows, stacking and ordering. R1-ABL-OQ10 (effects as
  entities or as a list on the target) names the ecs unit's choice as its decider.
- **ecs and data-hotreload:** content specs are referenced from the state by id, and hot reload must not invalidate ids
  held in the live state; R1-DATA-REC5 assumes entities hold definition ids, and R1-DATA-OQ7 needs the ecs unit's
  answer.
- **tick and determinism:** the float exactness of `dt` (R1-TICK-OQ1, R1-XU-OQ11); the tick counter as the only clock;
  seeded randomness drawn per tick (R1-DET-REC4); timers count ticks or carry their remainder (legacy GC5).
- **tick and boundary-worker:** the two newest tick states for interpolation (R1-XU-OQ3); W3 needs `SharedArrayBuffer`
  and cross-origin isolation (R1-XU-OQ4); a Worker's `performance.now()` counts from the Worker's start (R1-TICK-F20,
  R1-BND-F30), which R1-BND-REC2 mark 3 handles with `performance.timeOrigin + performance.now()`; hidden-tab timers
  (R1-XU-OQ12).
- **tick and abilities:** durations, periods and cooldowns are authored in seconds and converted at load with
  remainders (R1-TICK-REC1, R1-ABL-REC3, R1-XU-OQ7); fire intervals shorter than a tick (R1-ABL-OQ7).
- **tick and data-hotreload:** the tick constant and hot reload (R1-XU-OQ10); "applies from tick T" in R1-DATA-REC5
  depends on the tick numbering.
- **determinism and boundary-worker:** inputs are quantized and checked finite before they enter the sim, identically
  on server and client; no sim value is computed in TypeScript (R1-DET-F30, R1-DET-REC2); the boundary feeds the sim
  only commands with tick numbers and content data, so no wall clock or JS randomness enters through it (legacy GP14);
  the canonical state serialization of R1-DET-REC6 may share a format with snapshots.
- **determinism and abilities:** proc chances and random effects draw from a named sim RNG stream (R1-DET-REC4);
  ability data is checked finite at load; floats or fixed point for attribute maths (R1-ABL-OQ5) is the determinism
  unit's, and R1-DET-REC7 recommends no fixed point; no `HashMap` iteration in per-tick logic (R1-ABL-F26; the
  hash-map ban, R1-ECS-REC2, bans the types).
- **determinism and data-hotreload:** the content data's hash is part of a recorded match (R1-DET-REC6) and of every
  match record (R1-DATA-REC6); whether text-to-float parsing gives the same bits on both targets was not researched by
  the determinism unit, and R1-DATA-REC1 mark 1 tests it.
- **boundary-worker and abilities:** cues and effects reach presentation as typed events in the output block's event
  section (R1-BND-REC2), not as callbacks or a call per event; definition ids and feel-kit keys need generated
  TypeScript types; the feel kit stays client-side.
- **boundary-worker and data-hotreload:** content and tuning enter the worker as low-rate messages, and hot reload is a
  message to the worker (R1-DATA-REC5 relays through the page); where content's types come from (R1-XU-OQ8); hot
  reload into a Worker under Vite (R1-DATA-OQ8).
- **abilities and data-hotreload:** R1-ABL-REC5 gives only the shape of the definitions; the file format, schema,
  validation tooling and hot reload are the data unit's (R1-XU-OQ9). For Honor's data-driven modifiers talk (GDC 2017)
  is a lead for the ability unit, not read.

### Pointers to other topics
No new claims: each line names what a unit hands to another topic.
- **R2 (netcode):**
  - ecs: whole-world or partial replay (R1-ECS-OQ3, R1-XU-OQ2); the wire format, delta compression and dirty tracking
    (R1-ECS-REC1 drops bevy's change detection); entity ids on the wire (R1-ECS-REC3).
  - tick: the snapshot and send rate is separate from the tick (R1-TICK-F10, F11); remote-entity interpolation delay
    follows the send rate (R1-TICK-F25); lag-compensation history has one entry per tick; a 16-bit tick on the wire
    wraps every 18.2 minutes at 60 Hz (inferred arithmetic); R1-TICK-OQ3, OQ4, OQ8.
  - determinism: lockstep rejected (R1-DET-REC1); what the client predicts (R1-DET-OQ6); hash comparison as divergence
    telemetry (R1-DET-OQ9); input quantization.
  - boundary-worker: the input command's fields and edge semantics (R1-BND-REC3); whether the worker owns the
    connection (R1-BND-REC6, F29); the server snapshot format against the page block (R1-BND-OQ8); prediction replay
    cost inside the worker.
  - abilities: the prediction model (R1-ABL-OQ4); the prediction key as the command sequence (legacy CS1); which tags
    other teams see (R1-ABL-OQ8); how much effect state is replicated (R1-ABL-F20 for GAS's own modes).
  - data-hotreload: the join handshake carries the content hash (R1-DATA-REC6); live reload messages and private-match
    overrides travel on the game connection; R2 may hold a shipped-game source for R1-DATA-OQ3.
- **R3 (rendering and presentation):** render interpolation and frame pacing on 60, 120 and 144 Hz displays, and
  hit-stop as presentation time (R1-TICK-REC5); presentation's own RNG and animation never feed the sim (R1-DET-REC4;
  legacy GP14, GN8); rendering in a worker with OffscreenCanvas (boundary-worker option A3) and a per-entity or
  per-field layout for instancing (R1-BND-OQ6); cues drive effects and the spectacle budget (VISION_V2 §6.9), and the
  look is not decided.
- **R4 (physics and navigation):** R1-ECS-REC4 depends on R4's physics choice; the spatial index for shots and line of
  sight (legacy CS6, R1-ECS-F22) lives beside the tables, inside the snapshot or rebuilt after a restore (R4 decides);
  if bevy_ecs is reconsidered, its physics integrations (not read); Rapier's `dt` equal to the tick or a whole fraction
  of it (R1-TICK-OQ9); swept tests for fast projectiles (legacy GC15) or Rapier's CCD (R1-TICK-F8); Rapier's flag, cost
  and insertion order (R1-DET-REC3, OQ1, OQ2); f32 or f64 (R1-DET-OQ3); whether navigation runs only on the server;
  line of sight for blasts (NC-7, legacy GC14) and melee (legacy GC12), and the shield's frontal arc from sim facing;
  whether map geometry or navigation data may reload mid-match (R1-DATA-OQ7).
- **R5 (toolchain and quality):** the CI check against `HashMap` and `HashSet` iteration (R1-ECS-REC2 mark 2) and the
  counting-allocator test (R1-ECS-REC1 mark 2); a Rust tick bench and a Worker tick bench in CI (R1-TICK-REC2 P1–P3),
  and the legacy-parity harness steps at 1/60 (legacy TL13); R1-DET-REC6's suite on native, Node.js and the browser
  engines, a lint for R1-DET-REC2's bans and test builds with matching overflow settings; Vite's worker and Wasm setup
  (R1-BND-F33), `server.headers` for COOP and COEP in development (boundary-worker option G3), CI checks for
  generated-type diffs and the wasm-bindgen version pin (R1-BND-REC4), Playwright on all four browsers; the CI check
  that the committed schema matches the generated one (R1-DATA-REC3), the Vite plugin (R1-DATA-REC5), golden migration
  fixtures (R1-DATA-REC4) and R1-DATA-OQ11.
- **R6 (infrastructure):** cost per match if one match runs per vCPU (R1-ECS-OQ4); tick overrun and dropped-time
  metrics (R1-TICK-REC4) and matches per vCPU at 30 Hz against 60 Hz; the server CPU architecture joins R1-DET-REC6's
  matrix (R1-DET-OQ7); whether preview and production hosting can set COOP, COEP and CORP headers, and third-party
  embeds and the later login flow under isolation (R1-BND-REC5, OQ5); damage and effect events carry the source ability
  or weapon id, which pick and win rates need (VISION_V2 §6.6); match records store the content hash and mode id
  (R1-DATA-REC6); immutable hosting of content bundles.
- **Security (R2's protocol review, and the security baseline (R6-SEC-OQ1), deferred by the Advisor on 2026-10-27):**
  R1-DATA-REC7's allow list and fuzz corpus; COOP also guards against XS-Leaks (the COOP page, per the boundary-worker
  unit).
- **EDITOR_V2:** reachability runs the sim's own integrator (R1-TICK-REC6).

### Recommendations that hinge on a source not read
For the Advisor, to look for another copy (SRC-ACCESS (a), 2026-10-27). Each row is a recommendation whose unit draft
ties it, through the named open question, to a source the unit could not read. Nothing is claimed here about what
those sources say.

| Recommendation | Source not read | Reason | Tied by |
|---|---|---|---|
| R1-ECS-REC1 | https://www.gdcvault.com/play/1024001/-Overwatch-Gameplay-Architecture-and (also without `www.`): Overwatch's gameplay architecture talk, GDC 2017 | redirects to a gdconf.com maintenance page, outside the allow list; the talk is video ("likely video only" per the ecs unit, "video only" per the tick and abilities units) | R1-ECS-OQ6 |
| R1-TICK-REC2 | the same Overwatch talk (above) | as above | R1-TICK-OQ3 |
| R1-TICK-REC2 | https://developer.valvesoftware.com/wiki/Source_Multiplayer_Networking and https://developer.valvesoftware.com/wiki/Latency_Compensating_Methods_in_Client/Server_In-game_Protocol_Design_and_Optimization | HTTP 403 | R1-TICK-OQ3 |
| R1-TICK-REC5 | https://html.spec.whatwg.org/multipage/timers-and-user-prompts.html (the timer rules MDN cites) | outside the allow list | R1-TICK-OQ2 (R1-BND-OQ2 asks the same) |
| R1-DET-REC3 | https://docs.rs/crate/rapier3d/latest/features | the fetch was stopped by the organisation's spend limit and not retried; the draft says the repository manifests it read cover it | Sources not read (determinism) |
| R1-DET-REC4 | rand's book, "Reproducibility" chapter (rust-random.github.io; the draft records no full URL) | outside the allow list | R1-DET-OQ5 |
| R1-BND-REC4 | https://github.com/wasm-bindgen/wasm-bindgen/issues/4533 | on the allow list; not read in this run | R1-BND-OQ7 |
| R1-ABL-REC1, REC2, REC3, REC4, REC5 (each rests on findings among F1–F20, which all come from one community write-up) | https://dev.epicgames.com/documentation (the Unreal Engine Gameplay Ability System pages) and https://github.com/EpicGames/UnrealEngine/blob/release/Engine/Plugins/Runtime/GameplayAbilities/Source/GameplayAbilities/Public/GameplayPrediction.h | Epic's documentation: outside the allow list; the engine source: HTTP 404 (private repository: needs an Epic-linked GitHub account) | R1-ABL-OQ1 ("every GAS fact here is one community write-up's") |
| R1-ABL-REC1 | https://gdcvault.com/play/1024041/Networking-Scripted-Weapons-and-Abilities (also with `www.`): Overwatch's "Networking Scripted Weapons and Abilities" (GDC 2017, per a search result) | redirected to gdconf.com/gdc-vault-maintenance/, outside the allow list; video only | R1-ABL-OQ2 (whether a scripting layer, option A3, paid off) |
| R1-ABL-REC4 | https://developer.valvesoftware.com/wiki/Prediction and https://developer.valvesoftware.com/wiki/Dota_2_Workshop_Tools/Scripting/Abilities_Data_Driven | HTTP 403 | R1-ABL-OQ3 (REC4's cue rule) |
| R1-DATA-REC5 | https://developer.valvesoftware.com/wiki/Developer_Console_Control and https://developer.valvesoftware.com/wiki/ConVar | HTTP 403 | R1-DATA-OQ3 ("no shipped-game source backs REC5's 'development only'") |
| R1-DATA-REC1 | the docs of binary serde formats (postcard, bincode) and their RustSec entries (the draft records no URL) | not read in this run | R1-DATA-OQ2 (option D, a binary runtime format) |
| R1-BND-REC5 (added at the fix step) | the HTML standard's COOP and cross-origin isolation rules (https://html.spec.whatwg.org) | outside the allow list | R1-BND-OQ9 (whether `same-origin-allow-popups` can give isolation) |
| R1-BND-REC2 (added at the fix step) | ECMA-262's rule for detaching a buffer that has a detach key (tc39.es) | outside the allow list | R1-BND-F35 (that a transfer of the Wasm memory's buffer throws is inferred; the recommendation holds on F15 alone) |

## Verifier spot-check

The verifier checked every claim the R1 recommendations rest on, one report per unit; rust-reviewer checked R1's Rust
crate and determinism claims. Both ran on 2026-10-28 (reports: `.checkpoints/v2-research/verifier-r1-results.json`,
`verifier-r1-<unit>.md` and `rust-reviewer-r1-r2-result.md`, with every quote in full in their notes). The tables below
merge them: one row per claim, each verdict as the reviewer gave it, then the state after the fix step. Locations are
shortened; "—" means the reviewer had no row for that claim.

**Counts.**
- Verifier: 226 claims (its own counts per unit: ecs 29, tick 37, determinism 41, boundary-worker 46, abilities 40,
  data-hotreload 33; the tables below show 36 and 32 rows for tick and data-hotreload, as it grouped them): 219
  confirmed, 1 wrong (R1-DET-F25b), 4 unsupported (R1-BND-F15b, R1-BND-F23c, R1-ABL-C1, R1-ABL-C2), 1 partly
  confirmed (R1-BND-F2b, its 0.2.122 part unsupported), 1 not checkable (R1-ECS-F34, an absence). Verdicts: PASS WITH
  NOTES for all six units.
- rust-reviewer, R1 part: 101 claims: 99 confirmed, 1 wrong (R1-DET-F25), 1 partly confirmed (R1-BND-F2). Verdict:
  REVISE (the facts hold; Rust risks are missing from the recommendations).
- After the fix step, for the 6 claims found wrong, unsupported or partly confirmed (rust-reviewer's two are the
  same claims as the verifier's R1-DET-F25b and R1-BND-F2b): 2 re-sourced (R1-DET-F25b → R1-DET-F33, R1-BND-F15b →
  R1-BND-F34), 1 now an open question (R1-BND-F23c → R1-BND-OQ9), 3 fixed in place (R1-BND-F2b, whose wrong part no
  recommendation rested on; R1-ABL-C1 and C2, recommendation reasons rewritten to the source's words). Three more
  claims were re-sourced to close gaps the reviewers named (R1-ECS-F35, R1-DET-F34, R1-BND-F35). The five re-sourced
  claims, and the recommendations whose text, basis or pass marks changed, then went to a second verifier pass
  ("Second verifier pass (2026-10-28)", below): it found 2 more claims wrong and 1 unsupported, all fixed at the second
  fix round. No recommendation rests on a claim whose final verdict is wrong or unsupported.
- Rows: the unit tables' 243 rows from the fix step, plus 8 added at the second fix round (R1-ECS-F35, R1-DET-F33,
  F34a, F34b, R1-BND-F32, F34, F35, and R1-DATA-REC6's file:// clause), each giving the second pass's verdict.

### 1. ECS or plain data (unit `ecs`)

| Claim id | Claim | Source | Verdict (verifier; rust-reviewer) | Location | Final state |
|---|---|---|---|---|---|
| R1-ECS-F1 | bevy_ecs 0.20.0 is current; 0.16 to 0.20 shipped in under 18 months | github.com/bevyengine/bevy-website/tree/main/content/news and docs.rs/bevy_ecs | verifier: confirmed (that every 0.x release breaks the API is inferred, and the doc says so); rust-reviewer: confirmed | folders "2025-04-24-bevy-0.16" through "2026-10-08-bevy-0.20"; crate header "bevy_ecs 0.20.0" | confirmed; REC1 now marks the API churn inferred and weighs Rapier's own 0.x pace (fixed: REC1 reason) |
| R1-ECS-F2 | standalone | docs.rs bevy_ecs | verifier: —; rust-reviewer: confirmed | "can easily be used as a standalone crate in other projects" | confirmed; no change |
| R1-ECS-F3 | tables default, sparse opt-in | docs.rs bevy_ecs | verifier: —; rust-reviewer: confirmed | "This is the default storage type." | confirmed; no change |
| R1-ECS-F4 | the order of query results is not guaranteed | docs.rs bevy_ecs Query | verifier: confirmed; rust-reviewer: confirmed | iter: "Iteration order is not guaranteed."; par_iter: "Iteration order and thread assignment is not guaranteed."; iter_many: "Items are returned in th… | confirmed; no change |
| R1-ECS-F5 | the sort is not cached between runs | docs.rs QueryIter | verifier: confirmed; rust-reviewer: confirmed | sort: "Sorts all query items into a new iterator, using the query lens as a key." / "The sort is not cached across system runs." | confirmed; no change |
| R1-ECS-F6 | ambiguity Ignore; shuffle_seed debug | ScheduleBuildSettings | verifier: —; rust-reviewer: confirmed | "Defaults to `LogLevel::Ignore`." / "crate feature `debug`" | confirmed; no change |
| R1-ECS-F7 | single-thread exec; multi_threaded off | SingleThreadedExecutor; features | verifier: —; rust-reviewer: confirmed | "Runs the schedule using a single thread." | confirmed; no change |
| R1-ECS-F8 | World does not implement Clone | docs.rs World, Trait Implementations (all three parts of the page read) | verifier: confirmed; rust-reviewer: confirmed | Debug, Default, Send, Sync are listed; no "impl Clone for World" anywhere | confirmed; no change |
| R1-ECS-F9 | Entity is opaque and must not be lined up between worlds | docs.rs Entity | verifier: confirmed; rust-reviewer: confirmed | "should be treated as an opaque identifier."; "…ensure that `Entity` lines up between instances…"; to_bits: "Do not use for serialization between runs… | confirmed; no change |
| R1-ECS-F10 | bevy_ggrs snapshots each registered type, keyed by a rollback id | gschup/bevy_ggrs README and docs/architecture.md | verifier: confirmed; rust-reviewer: confirmed | "snapshotting only the components and resources you register."; "a `HashMap<RollbackId, As>` mapping each rollback entity…"; "spawns a fresh entity wi… | confirmed; REC1 now says these traps are bevy_ggrs's, a third-party library (fixed: REC1 reason) |
| R1-ECS-F11 | strategy cost notes, no benches | architecture.md | verifier: —; rust-reviewer: confirmed | "Heap-allocating clone each frame" | confirmed; no change |
| R1-ECS-F12 | deterministic replay in bevy needs an explicit sort | bevy_ggrs docs/pitfalls.md | verifier: confirmed; rust-reviewer: confirmed | "Bevy does not guarantee a stable iteration order for queries."; "Always sort queries that mutate state:" | confirmed; no change |
| R1-ECS-F13 | state that is not registered is silently not restored | pitfalls.md | verifier: confirmed; rust-reviewer: confirmed | "Any state stored elsewhere will not be restored on rollback, causing silent desyncs."; "`Local<T>` is per-system state that is not snapshotted." | confirmed; REC1 wording fixed as for F10 |
| R1-ECS-F14 | a restore triggers change detection | pitfalls.md, "Change Detection" | verifier: confirmed; rust-reviewer: confirmed | "Every snapshot restore triggers change detection on all restored components."; "Systems that react to `Changed<T>` will fire after every rollback…" | confirmed; REC1 wording fixed as for F10 |
| R1-ECS-F15 | 0.16 relationships, no_std | 0.16 post | verifier: —; rust-reviewer: confirmed (R1: inferred) | "One of the hottest ECS features is finally here" | fixed: label upgraded to confirmed |
| R1-ECS-F16 | hecs library, goals | docs.rs hecs | verifier: —; rust-reviewer: confirmed (order now C) | "It is a library, not a framework." | fixed: the goals' order quoted, confirmed |
| R1-ECS-F17 | a hecs world yields entities in arbitrary order and reuses ids | docs.rs hecs 0.11.2 World | verifier: confirmed; rust-reviewer: confirmed | "An unordered collection of entities…"; "…stored in contiguous runs…"; "Entities are yielded in arbitrary order."; "…their IDs will be reused on a fut… | confirmed; no change |
| R1-ECS-F18 | hecs World does not implement Clone | same page, Trait Implementations | verifier: confirmed; rust-reviewer: confirmed | Default, Extend, FromIterator, IntoIterator (&World), Send, Sync; no Clone | confirmed; REC1 now weighs hecs's F19 and F20 against it |
| R1-ECS-F19 | freelist deterministic ids | hecs World | verifier: —; rust-reviewer: confirmed (condition now C) | "May only be called when the set of live entity IDs exactly matches" | fixed: condition quoted; now in REC1's Rests on |
| R1-ECS-F20 | user context; column format | hecs serialize/column | verifier: —; rust-reviewer: confirmed | "For some applications, a custom approach may be preferable." | fixed: the no-Serialize reason quoted; now in REC1's Rests on |
| R1-ECS-F21 | for few entity kinds, use plain Vecs | github.com/Ralith/hecs README, "Why Not ECS?" | verifier: confirmed; rust-reviewer: confirmed | "If your game will have few types of entities, consider a simpler architecture such as storing each type of entity" + "…in a separate plain `Vec`."; "… | confirmed; no change |
| R1-ECS-F22 | state in other structures | hecs README | verifier: —; rust-reviewer: confirmed | "Most games will store significant amounts of state in other structures." | confirmed; no change |
| R1-ECS-F23 | HashMap is seeded per instance and iterates in arbitrary order | doc.rust-lang.org HashMap (1.99.1) | verifier: confirmed; rust-reviewer: confirmed | "`HashMap` is randomly seeded: each `HashMap` instance uses a different seed…"; iter: "An iterator visiting all key-value pairs in arbitrary order." | confirmed; no change |
| R1-ECS-F24 | BTreeMap iterates in key order | std BTreeMap, struct docs | verifier: confirmed; rust-reviewer: confirmed | "…produce their items in key order, and take worst-case logarithmic and amortized constant time per item returned." | confirmed; no change |
| R1-ECS-F25 | a slice iterates from start to end | std slice, method iter | verifier: confirmed; rust-reviewer: confirmed | "The iterator yields all items from start to end." | confirmed; no change |
| R1-ECS-F26 | Vec::clone_from does not reallocate when it can avoid it | std Vec, impl Clone, clone_from | verifier: confirmed; rust-reviewer: confirmed | "…as it avoids reallocation if possible."; "…this will reuse the resources of `self`'s elements as well."; "assert_eq!(yp, y.as_ptr());" | confirmed; REC1 no longer reads more into it (fixed: REC1 reason and mark 2, with R1-ECS-F35) |
| R1-ECS-F27 | Copy versus Clone; clone_from can reuse allocations | std Clone | verifier: confirmed; rust-reviewer: confirmed | "`Copy` is implicit and an inexpensive bit-wise copy, while `Clone` is always explicit and may or may not be expensive."; "…can be overridden to reuse… | confirmed; REC1 reason and mark 2 fixed; the derive and default clone_from re-sourced: R1-ECS-F35 |
| R1-ECS-F28 | a removed slotmap key stays removed | docs.rs slotmap 1.1.1 | verifier: confirmed; rust-reviewer: confirmed (indirection, serde now C) | "once a key is removed, it stays removed"; "as fast as a normal Vec"; "It uses two indirections per random access"; "…supports no_std environments, bu… | fixed: indirection and serde quoted, confirmed |
| R1-ECS-F29 | RigidBodySet implements Clone and Serialize; its handles are index plus generation | docs.rs rapier3d 0.36.1 RigidBodySet | verifier: confirmed; rust-reviewer: confirmed | "The collection that stores all rigid bodies in your physics world."; "This handle is lightweight (just an index + generation)…"; "impl Clone / Serial… | confirmed; its default clone_from now stated (R1-ECS-F35) |
| R1-ECS-F30 | the whole physics state can be serialized; the pipelines hold no state | rapier.rs/docs/user_guides/rust/serialization | verifier: confirmed with a note; rust-reviewer: confirmed (needs `serde-serialize` feature) | "When the `serde-serialize` feature of Rapier is enabled, most data structures of Rapier become serializable using serde."; "The complete state of the… | fixed: the `serde-serialize` feature named (F30, REC4) |
| R1-ECS-F31 | with enhanced determinism, the bytes are identical across machines | same page | verifier: confirmed with a note; rust-reviewer: confirmed | "If the `enhanced-determinism` feature of Rapier is enabled"…"the exact same byte vectors". The doc leaves out the condition "if your platform fulfill… | fixed: the platform condition added |
| R1-ECS-F32 | Rapier 0.30.1 skipped part of its broad-phase state when serializing; fixed in 0.33 | github.com/dimforge/rapier/issues/910 | verifier: confirmed; rust-reviewer: confirmed | "Rapier version: 0.30.1"; "BvhWorkspace is not serialized"; "the BvhWorkspace::rebuild_frame_index value is actually important"; "It appears this woul… | confirmed; REC1 now cites it for Rapier's 0.x pace |
| R1-ECS-F33 | generic code grows Wasm size; always measure; wasm-opt saves 15–20 % | rustwasm.github.io/book/reference/code-size.html | verifier: confirmed, but the "official" label is dated; rust-reviewer: confirmed | "…create a new copy of the function for each `T` type…"; "these copies add up quickly in terms of code size."; "Always measure!"; "can often get anoth… | fixed: tier marked archived (2025-08-25) |
| R1-ECS-F34 | no source gives a Wasm size or compile time | a claim of absence | verifier: not checkable against a quote; none of the sources I re-read gives such a figure; rust-reviewer: — | — | not checkable (an absence); REC1 mark 5 measures it. Second pass: unsupported (harmless). Fixed at the second fix round: its "Sure" now says it is an absence no quote can confirm, and REC1 no longer lists it in "Rests on" |
| R1-ECS-F35 (added at the fix step) | derive and default clone_from; RigidBodySet keeps the default | std Clone 1.99.1; docs.rs rapier3d 0.36.1 RigidBodySet | verifier: — (no first-pass row); second pass: confirmed; rust-reviewer: — (its risk 5 named the gap) | "The `derive`d implementation of Clone calls clone on each field."; RigidBodySet clone_from: "Performs copy-assignment from `source`.", source link to core's clone.rs | confirmed. Fixed at the second fix round: the head says "clones field by field" (the fix step's "writes only `clone`" was a reading), and REC1's "does not reuse buffers" is marked inferred |
| GC2 | the legacy step at 48 soldiers took 0.80–1.00 ms mean | docs/reports/2026-10-16-three-teams-core.md:48 | verifier: confirmed; rust-reviewer: — | "48 soldiers (head): combat 0.80–1.00, p95 0.97–1.54" | confirmed; added to REC1's Rests on (mark 1) |
| CS7 | every static collider has id −1 | docs/LEGACY_INVENTORY.md:175 | verifier: confirmed; rust-reviewer: — | "today every block is id -1" | confirmed; added to REC1's and REC3's Rests on |
| GP14 | the step order decides outcomes; the sim shares Math.random with sound and FX | docs/LEGACY_INVENTORY.md:324 | verifier: confirmed; rust-reviewer: — | "because the order decides outcomes"; "the one shared Math.random, which sound and FX draw from" | confirmed; added to REC2's Rests on |
| GC15 | a stuck bolt follows its victim | docs/LEGACY_INVENTORY.md:254 | verifier: confirmed; rust-reviewer: — | "a stuck bolt follows the victim's mesh position". The doc's "by reference" is a gloss. | confirmed; REC3's "by reference" gloss replaced and GC15 added to its Rests on |
| VISION_V2 §4 | matches feel fair up to about 100 ms of ping | docs/VISION_V2.md:54 | verifier: confirmed; rust-reviewer: — | "`[DEFAULT]` Matches feel fair up to about 100 ms of ping." This is a default, not a decided target. | confirmed; added to REC1's Rests on (mark 3) |
| REC1 mark 2 | "0 heap allocations after warm-up, Rapier included" | std Clone; RigidBodySet | verifier: goes further than F26 and F27 allow (finding 3); rust-reviewer: not reachable as written (risk 5) | std Clone: the default clone_from "is equivalent to `a = b.clone()`" | fixed: mark 2 rewritten; re-sourced: R1-ECS-F35 |

### 2. A fixed tick rate (unit `tick`)

| Claim id | Claim | Source | Verdict (verifier; rust-reviewer) | Location | Final state |
|---|---|---|---|---|---|
| R1-TICK-F1 | a variable step changes behaviour | gafferongames fix_your_timestep | verifier: confirmed; rust-reviewer: — | section "Variable delta time", not the "opening section" the doc names: "The problem is that the behavior of your physics simulation depends on the de… | fixed: location corrected |
| R1-TICK-F2 | the accumulator loop | same page | verifier: confirmed; rust-reviewer: — | "Free the physics" code: `accumulator += frameTime;` … `accumulator -= dt;` | confirmed; no change |
| R1-TICK-F3 | spiral of death, headroom, 0.25 s clamp | same page | verifier: confirmed; rust-reviewer: — | the spiral and "significantly less than X seconds" quotes are in "Semi-fixed timestep", not "Free the physics"; the clamp `if ( frameTime > 0.25 )` is… | fixed: locations corrected; added to REC2's Rests on |
| R1-TICK-F4 | interpolation by `alpha` | same page | verifier: confirmed; rust-reviewer: — | "The final touch" | confirmed; no change |
| R1-TICK-F5 | a fixed step is needed for reproducibility | same page | verifier: confirmed; rust-reviewer: — | "Free the physics", paragraphs 1 and 2 ("why is it necessary to have fully fixed delta time") | confirmed; no change |
| R1-TICK-F6 | Bevy defaults to 64 Hz, and why | docs.rs bevy 0.20.0, Fixed | verifier: confirmed; rust-reviewer: confirmed | type docs: "The default timestep() is 64 hertz, or 15625 microseconds." | confirmed; no change |
| R1-TICK-F7 | paused clock, capped catch-up | docs.rs bevy Fixed and Virtual | verifier: confirmed; rust-reviewer: confirmed | "If the virtual clock is paused, the FixedUpdate schedule will not run."; Virtual, paragraph on `set_max_delta` | confirmed; no change |
| R1-TICK-F8 | Rapier step 1/60 s; a smaller step costs more CPU | docs.rs rapier3d 0.36.1, IntegrationParameters | verifier: confirmed; rust-reviewer: confirmed | fields `dt` and `max_ccd_substeps` | confirmed; REC2's "halves" now marked inferred |
| R1-TICK-F9 | Quake III world runs at 20 fps | sv_init.c:596 | verifier: confirmed; rust-reviewer: — | `sv_fps = Cvar_Get ("sv_fps", "20", CVAR_TEMP );` | confirmed; no change |
| R1-TICK-F10 | Quake III millisecond accumulator, messages sent after the loop | sv_main.c `SV_Frame` :775, :777, :832, :837, :848 | verifier: confirmed; rust-reviewer: — | — | confirmed; no change |
| R1-TICK-F11 | Quake III snapshot rate capped at 30, default 50 ms | sv_client.c `SV_UserinfoChanged` :1151–1161 | verifier: confirmed; rust-reviewer: — | — | confirmed; no change |
| R1-TICK-F12 | Quake III chops long moves; fixed movement step off by default, 8 ms | bg_pmove.c :2041–2054, g_main.c :178–179 | verifier: confirmed. An extra read also confirms the part the doc marks inferred: movement runs once per received command, outside the world frame (sv… | — | fixed: the per-command part upgraded to confirmed |
| R1-TICK-F13 | Counter-Strike at 66 ticks; `-tickrate` compiled out | gameinterface.cpp :825–838 | verifier: confirmed. The 66 comes from a code comment only: "[Forrest] For Counter-Strike, set default tick rate of 66 …"; rust-reviewer: — | — | confirmed; no change |
| R1-TICK-F14 | Source rounds durations to whole ticks | shareddefs.h :17–18 and :407 | verifier: confirmed; rust-reviewer: — | — | confirmed; no change |
| R1-TICK-F15 | MDN loop; a timer in a Web Worker | MDN Games/Anatomy | verifier: confirmed; rust-reviewer: — | all quotes, plus "in need of technical review" | confirmed; no change |
| R1-TICK-F16 | pause, simulate or recover; watch the tick count | same page, Summary list and a code comment | verifier: confirmed. The words "problematic for most multiplayer games" are verbatim, although the doc labels them inferred.; rust-reviewer: — | — | fixed: label upgraded to confirmed |
| R1-TICK-F17 | `requestAnimationFrame` follows the refresh rate and pauses in background tabs | MDN DedicatedWorkerGlobalScope/requestAnimationFrame | verifier: confirmed. The main-thread half is also confirmed on MDN Window/requestAnimationFrame (a page the doc does not cite).; rust-reviewer: — | — | confirmed; no change |
| R1-TICK-F18 | timer clamping and throttling | MDN setTimeout | verifier: confirmed; rust-reviewer: — | the heading is "Reasons for longer delays than specified", not the doc's wording. Chrome's intensive throttling needs four conditions; the doc shorten… | fixed: heading and Chrome's four conditions corrected |
| R1-TICK-F20 | `performance.now()` in workers, 100 µs / 5 µs | MDN Performance/now | verifier: confirmed; rust-reviewer: — | — | confirmed; no change |
| R1-TICK-F21 | `performance.now()` keeps ticking during sleep only on Windows | same page, "Ticking during sleep" | verifier: confirmed; rust-reviewer: — | — | confirmed; no change |
| R1-TICK-F22 | `Atomics.wait` rules | MDN Atomics/wait | verifier: confirmed; rust-reviewer: — | four quotes | confirmed; no change |
| R1-TICK-F23 | Tokio timers are millisecond-granular; Burst is the default | docs.rs tokio 1.54.1, fn.sleep and enum.MissedTickBehavior | verifier: confirmed; rust-reviewer: confirmed | — | confirmed; tokio 1.54.0's advisory noted in R1-RR-OQ7 |
| R1-TICK-F24 | prediction ring buffer and replay; the 200 ms example | gafferongames what_every_programmer… | verifier: confirmed; rust-reviewer: — | "if it takes 100ms from client to server and 100ms back … 200ms in the past" | confirmed; REC2 P3's explanation corrected (F24's 200 ms is a round trip) |
| R1-TICK-F26 | explicit Euler error; semi-implicit Euler | gafferongames integration_basics | verifier: confirmed; rust-reviewer: — | "Most commercial game physics engines use this integrator." sits under the heading "Semi-implicit Euler" | confirmed; no change |
| CS1 | legacy row CS1 | inventory :169, shamble-core.js:1479, shamble-game.js:3053 | verifier: confirmed; rust-reviewer: — | — | confirmed; no change |
| GP9 | legacy row GP9 | inventory :319 | verifier: confirmed; rust-reviewer: — | — | confirmed; no change |
| GP13 | legacy row GP13 | inventory :323 | verifier: confirmed; rust-reviewer: — | — | confirmed; no change |
| GP14 | legacy row GP14 | inventory :324, AUDIT_BUGS_PERF.md:10 (0.49 ms) | verifier: confirmed; rust-reviewer: — | — | confirmed; no change |
| GW14 | legacy row GW14 | inventory :202, shamble-core.js:82, shamble-game.js:1575; 11²/48 = 2.52 m, plus 0.55 = 3.0… | verifier: confirmed; rust-reviewer: — | — | confirmed; REC2's "every legacy measurement" corrected (GW14 ran at 20, 60 and 144 Hz) |
| GN10 | legacy row GN10 | inventory :275 | verifier: confirmed; rust-reviewer: — | — | confirmed; added to the unit's legacy facts |
| GC15 | legacy row GC15 | inventory :254 | verifier: confirmed; rust-reviewer: — | — | confirmed; no change |
| GN4 | legacy row GN4 | inventory :269 | verifier: confirmed; rust-reviewer: — | — | confirmed; added to REC2's Rests on (measured at 60 and 144 fps) |
| TL13 | legacy row TL13 | inventory :398, tools/sim.js:19 | verifier: confirmed; rust-reviewer: — | — | confirmed; no change |
| TL15 | legacy row TL15 | inventory :400, tools/perfbots.js:24, docs/reports/2026-10-16-three-teams-core.md:48 (0.80… | verifier: confirmed; rust-reviewer: — | — | confirmed; no change |
| KT13 | legacy row KT13 | inventory :422 | verifier: confirmed; rust-reviewer: — | — | confirmed; no change |
| `WEAPONS` and `T` | legacy row `WEAPONS` and `T` | docs/legacy-tuning.json (0.072, 0.2, 0.42, 0.06; `jumpV` 11, `gravity` 24, `stepUp` 0.55;… | verifier: confirmed. The smallest interval in all tables is 0.06 s, so "one shot per tick at 30 Hz" holds.; rust-reviewer: — | — | confirmed; no change |
| REC2 "halves" | "30 Hz halves the CPU per simulated second (F8)" | IntegrationParameters (F8) | verifier: a number not in the source (finding 2) | F8 gives only the direction | fixed: marked inferred, with its assumption |

### 3. Identical results between Wasm and native (unit `determinism`)

| Claim id | Claim | Source | Verdict (verifier; rust-reviewer) | Location | Final state |
|---|---|---|---|---|---|
| R1-DET-F1 | Wasm floats follow IEEE 754, round to nearest, ties to even | webassembly.github.io/spec/core/exec/numerics.html (3.0, 2026-10-27) | verifier: confirmed; rust-reviewer: confirmed | "Floating-Point Operations": "All operators use round-to-nearest ties-to-even, except where otherwise specified." | confirmed; no change |
| R1-DET-F2 | NaN sign and payload are non-deterministic | numerics.html | verifier: confirmed; rust-reviewer: confirmed | "NaN Propagation": "then its sign is non-deterministic…"; "Otherwise the payload is picked non-deterministically among all arithmetic NaNs" | confirmed; no change |
| R1-DET-F3 | The host chooses the deterministic profile; tools target the full profile (in no Rests on list) | spec/core/appendix/profiles.html | verifier: confirmed; rust-reviewer: confirmed | "A host platform can thereby decide to support the language only under a restricted profile"; "Tools are generally expected to handle and produce code… | confirmed; added to REC2's and REC6's Rests on |
| R1-DET-F4 | Relaxed SIMD depends on hardware, is fixed for one run, and its multiply-add may be fused or not | numerics.html | verifier: confirmed; rust-reviewer: confirmed | "Relaxed Operations": "…such as its hardware"; "These choices are fixed…"; "Relaxed multiply-add allows for fused or unfused results…"; "every paramet… | confirmed; no change |
| R1-DET-F5 | Relaxed SIMD: Chrome 114, Firefox 145, Safari behind a flag, Node 21.0, Wasmtime 15 (context only) | raw.githubusercontent.com/WebAssembly/website/main/features.json | verifier: confirmed, verbatim (the doc says "inferred"); rust-reviewer: — | `"relaxedSimd": "114"` / `"145"` / `["flag", "Requires JavaScriptCore flag useWebAssemblyRelaxedSIMD"]` / `"21.0"` / `"15"` | fixed: label upgraded to confirmed |
| R1-DET-F6 | Default proposals on wasm32-unknown-unknown | doc.rust-lang.org/rustc/platform-support/wasm32-unknown-unknown.html | verifier: confirmed; rust-reviewer: confirmed | "As of the time of this writing the proposals that are enabled by default…" followed by the 6-item list; simd128 is not in it | confirmed; no change |
| R1-DET-F7 | Rust `+ - * /` round ties to even | std f32 (1.99.1) | verifier: confirmed; rust-reviewer: confirmed | type docs: "…rounded according to the roundTiesToEven direction defined in IEEE 754-2008." | confirmed; no change |
| R1-DET-F8 | NaN bits are not portable | std f32 | verifier: confirmed; rust-reviewer: confirmed (location: type-level bullet) | "NaN bit patterns": "…not guaranteed to be portable or even fully deterministic!" | confirmed; location note added |
| R1-DET-F9 | std transcendental functions have no fixed precision | std f32 | verifier: confirmed for sin, cos, tan, atan2, powf, exp and ln; the other functions stay inferred; rust-reviewer: confirmed for sin; others listed, no… | "The precision of this function is non-deterministic. This means it varies by platform, Rust version…" | fixed: confirmed for cos, tan, atan2, powf, exp, ln too; tan's libc binding quoted |
| R1-DET-F10 | `sqrt` and `mul_add` exact; floor, ceil, round, trunc and fract precise | std f32 | verifier: confirmed; rust-reviewer: confirmed | sqrt: "…specified by IEEE 754 as `squareRoot` and guaranteed not to change."; mul_add: "…as `fusedMultiplyAdd` and guaranteed not to change."; the rou… | confirmed; no change |
| R1-DET-F11 | No implicit fused multiply-add; opting in is explicit; `algebraic_*` has undefined precision | RFC 3514; std f32 | verifier: confirmed; rust-reviewer: confirmed (1.98.0 now C) | "…such as turning `a*b + c` into FMA operations"; "…explicit opt-in for such semantics-alternating transformations"; "…results of elementary operation… | fixed: "stable since 1.98.0" upgraded to confirmed |
| R1-DET-F12 | 64-bit x86, 64-bit ARM and RISC-V fully compliant; i586 and 32-bit ARM NEON are the exceptions | RFC 3514 | verifier: confirmed; rust-reviewer: confirmed (RFC also names wasm for NaN) | "The most widely used targets (64bit x86 and 64bit ARM, and also the increasingly popular RISC-V) are fully compliant."; "On 32bit ARM, NEON SIMD oper… | fixed: the RFC's wasm NaN sentence added |
| R1-DET-F13 | Compile-time NaN choice may differ from run time | RFC 3514 | verifier: confirmed; rust-reviewer: confirmed | "there is no guarantee that the choice made in const evaluation is consistent with the choice made at runtime." | confirmed; no change |
| R1-DET-F14 | min, max and clamp: which zero comes back is non-deterministic | std f32 | verifier: confirmed; rust-reviewer: confirmed (+ clamp "Panics if `min > max`") | min and max: "…either input may be returned non-deterministically."; clamp: "…either `0.0` or `-0.0` is returned non-deterministically." | fixed: clamp's panic added (R1-RR-OQ3) |
| R1-DET-F15 | Float-to-int casts saturate, NaN gives 0; integer overflow depends on build flags | Rust Reference, operator-expr | verifier: confirmed; rust-reviewer: confirmed | "Numeric cast": "…round the float towards zero", "`NaN` will return `0`", "…will saturate…"; "Overflow": "…`-C debug-assertions` and `-C overflow-chec… | confirmed; no change |
| R1-DET-F16 | compiler-builtins supplies the pure-Rust math on wasm32-unknown-unknown but not on Unix | compiler-builtins/src/math/mod.rs (master) | verifier: confirmed; rust-reviewer: confirmed | group `partial_availability`: "- All unix targets (linux, macos, freebsd, android, etc)" / "- wasm with known target_os", plus its cfg. The doc's cfg… | confirmed; no change |
| R1-DET-F17 | libm 0.2.16 is pure Rust; the default `arch` feature swaps in only exact operations | docs.rs/libm; libm arch/mod.rs; libm Cargo.toml | verifier: confirmed; rust-reviewer: confirmed | "libm in pure Rust"; `pub use wasm32::{ ceil, … sqrt, sqrtf, trunc, truncf, };`; `pub use x86::{sqrt, sqrtf, fma, fmaf};`; `default = ["arch"]` | confirmed; no change |
| R1-DET-F18 | libm claims no accuracy or cross-platform identity (in no Rests on list) | docs.rs/libm; libm README | verifier: confirmed (an absence); rust-reviewer: confirmed (absence) | the fetch found no such sentence on either page | confirmed (an absence); added to REC2's Rests on |
| R1-DET-F19 | Rapier by default is deterministic only on one machine | rapier.rs determinism page (0.36) | verifier: confirmed; rust-reviewer: confirmed | "By default, Rapier is locally deterministic…"; "However, doing this on two different computers may result in completely different results." | confirmed; no change |
| R1-DET-F20 | Conditions for cross-platform identity | rapier.rs determinism page | verifier: confirmed; rust-reviewer: confirmed | "The `enhanced-determinism` feature of Rapier is enabled."; "…as well as WASM targets."; "…in the exact same order."; "…`ComplexField` or `RealField`… | confirmed; added to REC2's Rests on |
| R1-DET-F21 | The flag excludes `simd8` but combines with `parallel` | rapier.rs determinism page | verifier: confirmed; rust-reviewer: confirmed (+ missed "is therefore its own determinism domain.") | "…cannot be enabled at the same time as the `simd8` feature,"; "The `parallel` feature, on the other hand, can be combined with it:" | fixed: the "determinism domain" sentence added |
| R1-DET-F22 | The rapier3d flag forces libm and passes the flag to parry; the 4-lane path stays on; version 0.36.1 | rapier3d Cargo.toml; workspace Cargo.toml | verifier: confirmed; rust-reviewer: confirmed | `enhanced-determinism = ["simba/libm_force", "parry3d/enhanced-determinism"]`; `simba = { workspace = true, features = ["wide"] }` (no condition on it… | confirmed; added to REC2's Rests on |
| R1-DET-F23 | The parry3d flag forces libm and scalar maths in glamx and adds indexmap | parry3d Cargo.toml; rapier workspace Cargo.toml | verifier: confirmed (why indexmap is there stays inferred); rust-reviewer: confirmed (pins + fast-math note now C) | `enhanced-determinism = ["simba/libm_force", "indexmap", "glamx/libm", "glamx/scalar-math"]`. The workspace pins and the no-fast-math comment are also… | fixed: workspace pins and the fast-math note upgraded to confirmed |
| R1-DET-F24 | Rapier's page gives no cost and no way to check determinism | rapier.rs determinism page | verifier: confirmed (an absence); rust-reviewer: confirmed (absence) | whole page | confirmed; no change |
| R1-DET-F25a | HashMap is randomly seeded, iterates in arbitrary order, and a fixed hasher brings a DoS warning | std HashMap | verifier: confirmed; rust-reviewer: — | "The algorithm is randomly seeded…"; iter: "An iterator visiting all key-value pairs in arbitrary order."; with_hasher: "…can expose a DoS attack vect… | confirmed; no change |
| R1-DET-F25b | HashMap is "randomly seeded per process" | std HashMap | verifier: wrong; rust-reviewer: wrong (per instance) | const/static section: "each `HashMap` instance uses a different seed" | re-sourced: R1-DET-F33 (std HashMap: "each `HashMap` instance uses a different seed"); F25, REC5, REC6 and options Q5 corrected |
| R1-DET-F26 | BTreeMap iterates in key order | std BTreeMap | verifier: confirmed; rust-reviewer: confirmed | "…stores its entries in key order."; iter: "…sorted by key." | confirmed; no change |
| R1-DET-F27 | StdRng is not portable | docs.rs rand 0.10.3, StdRng | verifier: confirmed; rust-reviewer: confirmed | "Non-portable: any future library version may replace the algorithm…"; "…even with a fixed seed, output is not portable." | confirmed; no change |
| R1-DET-F28 | rand_chacha and rand_pcg are portable; ChaCha has 2^64 streams and a settable position; Pcg64Mcg is slow on 32… | docs.rs rand_chacha 0.10.0, ChaCha8Rng, rand_pcg 0.10.2 | verifier: confirmed; rust-reviewer: confirmed | "These generators are all deterministic and portable … with testing against reference vectors."; set_stream: "…2^64 unique streams…" (the page shows "… | confirmed; REC4's Pcg32 fallback re-sourced: R1-DET-F34 (its stream note corrected at the second fix round: R1-DET-F34b, R1-DET-OQ11) |
| R1-DET-F29 | The fixed crate has no trigonometry | docs.rs fixed 1.31.0 | verifier: confirmed; rust-reviewer: confirmed | "This crate does _not_ provide general analytic functions."; "No trigonometric functions are provided…" | confirmed; no change |
| R1-DET-F30 | JavaScript Math differs between browsers and machines; Math.random takes no seed | MDN Math; MDN Math/random (not cited by the doc) | verifier: confirmed; rust-reviewer: — | "Many `Math` functions have a precision that's implementation-dependent."; Math/random: "…it cannot be chosen or reset by the user." | fixed: "no seed" confirmed on MDN Math.random (verifier's read) |
| R1-DET-F31 | Lockstep needs bit-identical results; shipped PC games held them with IEEE floats; transcendental results diff… | gafferongames "Floating Point Determinism" (2010-02-24) | verifier: confirmed; rust-reviewer: — | Ken Miller (Pandemic): "absolutely identical results … least-significant bit of the mantissa", "…slightly different results for trancendental function… | fixed: tier relabelled "other" (R1-XU-OQ6 settled) |
| R1-DET-F32 | Prediction gets corrected; lockstep desyncs | gafferongames "What Every Programmer Needs To Know About Game Networking" | verifier: confirmed; rust-reviewer: — | "running a subset of the game code for your player character…"; "invisibly 'rewinds and replays'…"; "…seldom corrected."; "one tiny difference results… | fixed: tier relabelled "other" (R1-XU-OQ6 settled) |
| R1-DET-F33 (added at the fix step) | the HashMap seed is per instance | std HashMap 1.99.1, "Usage in `const` and `static`" | verifier: — (no first-pass row); second pass: confirmed; rust-reviewer: — | "each `HashMap` instance uses a different seed"; no sentence ties the seed to "process" | confirmed. REC5's mark and REC6's reason now say two runs "can" expose an order, marked inferred (second fix round) |
| R1-DET-F34a (added at the fix step) | Pcg32 suits 32- and 64-bit CPUs, takes a stream in its constructor, can jump ahead | docs.rs rand_pcg 0.10.2, crate root and Lcg64Xsh32 | verifier: — (no first-pass row); second pass: confirmed; rust-reviewer: — | "This is a good choice on both 32-bit and 64-bit CPUs (for 32-bit output)."; `pub fn new(state: u64, stream: u64) -> Self`; "Multi-step advance functions (jump-ahead, jump-back)" | confirmed; no change |
| R1-DET-F34b (added at the fix step) | "the pages say nothing on how streams relate" | Lcg64Xsh32 page | verifier: — (no first-pass row); second pass: wrong; rust-reviewer: — | `new`: "Note that two generators with different stream parameters may be closely correlated." (the struct docs say it again) | fixed: F34 quotes the warning; REC4's fallback and options Q4 no longer count the stream number as a merit; now an open question: R1-DET-OQ11 |
| R1-DET-V3 | The goal is 12v12v12 or 24v24 (36 or 48 players) | docs/VISION_V2.md §3 | verifier: confirmed; rust-reviewer: — | "The goal: 12v12v12 or 24v24." | confirmed; no change |
| R1-DET-V4 | Fair up to about 100 ms of ping | docs/VISION_V2.md §4 | verifier: confirmed; rust-reviewer: — | "[DEFAULT] Matches feel fair up to about 100 ms of ping." | confirmed; no change |
| R1-DET-L1 | 11 wall-clock reads; shared Math.random; effects need their own RNG stream | docs/STATUS.md:70 | verifier: confirmed; rust-reviewer: — | ADR-001 inputs | confirmed; added to REC1's Rests on |
| R1-DET-L2 | GP14: 10 calls plus 1 comment, animation clocks, sound noise drawn before spread, camera shake | docs/LEGACY_INVENTORY.md:324 | verifier: confirmed; rust-reviewer: — | GP14 row | confirmed; added to REC5's Rests on |
| R1-DET-L3 | GN6: bot spread comes from the shared Math.random | docs/LEGACY_INVENTORY.md:271 | verifier: confirmed; rust-reviewer: — | GN6 row | confirmed; no change |
| R1-DET-L4 | GN8: cull on and off give identical logs (69 and 207 events) | docs/LEGACY_INVENTORY.md:273 | verifier: confirmed; rust-reviewer: — | GN8 row | confirmed; added to REC6's Rests on |
| R1-DET-L5 | GC5: 10.0, 12.0 and 13.1 shots per second against 13.9 | docs/LEGACY_INVENTORY.md:244 | verifier: confirmed; rust-reviewer: — | GC5 row | confirmed; no change |
| R1-DET-L6 | TL13: at least 3 runs, N = 12/24/36/48, seeded bot matches on the native server | docs/LEGACY_INVENTORY.md:398 | verifier: confirmed; rust-reviewer: — | TL13 row | confirmed; added to REC1's Rests on |
| REC4 fallback | Pcg32 as the fallback RNG | rand_pcg | verifier: not backed by F28, which quotes only Pcg64Mcg (finding 6) | crate root: Pcg32 "is a good choice on both 32-bit and 64-bit CPUs" | re-sourced: R1-DET-F34. Second pass: F34's closing note on streams wrong; fixed, and how a Pcg32 fallback keeps purposes apart is now an open question: R1-DET-OQ11 |

### 4. The boundary and the Web Worker (unit `boundary-worker`)

| Claim id | Claim | Source | Verdict (verifier; rust-reviewer) | Location | Final state |
|---|---|---|---|---|---|
| R1-BND-F1 | rustwasm org wound down | Inside Rust 2025-07-21 | verifier: —; rust-reviewer: confirmed (omits "or transferred to their existing maintainers") | "archived in place" | fixed: the "or transferred" half added |
| R1-BND-F2a | the CLI and the library must be the same version since 0.2.128 | GitHub release 0.2.128 | verifier: confirmed; rust-reviewer: — | release notes, "Requires matching `wasm-bindgen` and CLI versions (schema bump)." | confirmed; unchanged (REC4 rests on it) |
| R1-BND-F2b | release cadence; breaking changes in 0.2.122, 0.2.123 and 0.2.128; the year 2026 | GitHub releases list | verifier: partly confirmed; rust-reviewer: partly confirmed (the version match and .123 confirmed; .122 breaking unsupported) | dates from 0.2.118 "10 Apr" to 0.2.129 "25 Sep" confirmed; 0.2.123 "a breaking change for callers that relied on widening `&mut` references." confirme… | fixed: 0.2.122 dropped, the year attributed to the asset timestamps, OQ7's count corrected (no recommendation rests on it) |
| R1-BND-F3 | a .d.ts file is written by default | guide cli.md | verifier: confirmed; rust-reviewer: confirmed | `--typescript`: "Output a TypeScript declaration file for the generated JavaScript bindings. This is on by default." | confirmed; no change |
| R1-BND-F4 | `typescript_custom_section` appends to the .d.ts | guide attribute page | verifier: confirmed; rust-reviewer: confirmed | "it will append the contents of the string to the `.d.ts` file exported by `wasm-bindgen-cli`" / "The primary target for this feature is for code gene… | confirmed; no change |
| R1-BND-F5 | serde-wasm-bindgen makes many boundary calls; JSON runs at 2x to 0.2x its speed | guide serde page | verifier: confirmed; rust-reviewer: confirmed | heading "An alternative approach - using JSON", "This requires a lot of calls back and forth between Rust and JavaScript, which can sometimes be slow.… | confirmed; no change |
| R1-BND-F6 | slices copied | number-slices.md | verifier: —; rust-reviewer: confirmed (&[T] inferred) | "since the results are copied back into its buffer." | confirmed; no change |
| R1-BND-F7 | keep data in linear memory and read it through overlay views | Rust and WebAssembly book, game-of-life/implementing | verifier: confirmed; rust-reviewer: confirmed | "By working with pointers and overlays, we avoid copying the cells across the boundary on every tick."; banner "This project and website is no longer… | confirmed; no change |
| R1-BND-F8 | tsify's purpose, its serde route and deprecation; tsify-next exists; neither page says which line is maintaine… | docs.rs tsify 0.5.8 and tsify-next 0.5.6 | verifier: confirmed; rust-reviewer: confirmed (js feature, deprecation now C) | `js` feature "enables serialization through serde-wasm-bindgen"; "Deprecated in favour of using Ts<T> as on function parameters and return type"; neit… | fixed: serde route and deprecation upgraded to confirmed |
| R1-BND-F9 | ts-rs generates types at test time | docs.rs ts-rs 12.0.1 | verifier: confirmed; rust-reviewer: confirmed (MSRV now C) | "When running `cargo test` or `cargo test export_bindings`, …"; "no runtime part" is inferred from this; the minimum Rust version was not checked | fixed: MSRV upgraded to confirmed (rust-reviewer) |
| R1-BND-F10 | only `repr(C)` gives a layout another language can read by offset | Rust Reference, type-layout | verifier: confirmed; rust-reviewer: confirmed | sections "The `Rust` representation", "The `C` representation", "`#[repr(C)]` Structs" | confirmed; no change |
| R1-BND-F11 | a typed array is a view, and its offset must be a multiple of the element size | MDN TypedArray | verifier: confirmed; rust-reviewer: — | constructor section; Exceptions, RangeError | confirmed; no change |
| R1-BND-F12 | shared memory needs a secure, isolated page, otherwise `postMessage` throws | MDN SharedArrayBuffer | verifier: confirmed; rust-reviewer: — | "To use shared memory your document must be in a secure context and cross-origin isolated." | confirmed; no change |
| R1-BND-F15a | a transfer moves the buffer without copying and the sender loses it | MDN Transferable objects | verifier: confirmed; rust-reviewer: — | "literally moved … zero-copy operation" / "any attempt to read or write the object will throw an exception." | confirmed; no change |
| R1-BND-F15b | without a transfer list the data is copied | MDN Transferable objects | verifier: unsupported; rust-reviewer: — | no sentence on the page says this, only a `structuredClone` code example | re-sourced: R1-BND-F34 (MDN structured clone algorithm: "copies complex JavaScript objects", used by postMessage); REC3 rests on it |
| R1-BND-F15c | typed arrays are not transferable but their buffers are; WebTransport streams are on the list | MDN Transferable objects | verifier: confirmed; rust-reviewer: — | "…serializable, but not transferable. However their underlying buffer is an ArrayBuffer, which is a transferable object." | confirmed; its label upgraded |
| R1-BND-F16 | growing a non-shared memory detaches its views; a shared one keeps its old length | MDN Memory.grow | verifier: confirmed; rust-reviewer: — | Examples, "Detachment upon growing" | confirmed; no change |
| R1-BND-F17a | a shared Wasm memory's buffer is a SharedArrayBuffer | MDN SharedArrayBuffer | verifier: confirmed; rust-reviewer: — | section "WebAssembly shared memory" | confirmed; no change |
| R1-BND-F17b | a module must declare a shared memory to import one | threads proposal Overview.md | verifier: confirmed; rust-reviewer: — | section "Shared Linear Memory", "It is a validation error to attempt to import shared linear memory if the module's memory import doesn't specify that… | fixed: the quote replaced by the rule the claim needs (re-read 2026-10-28) |
| R1-BND-F17c | a shared memory must have a maximum size | threads proposal Overview.md | verifier: confirmed; rust-reviewer: — | section "Spec Changes", "Note that shared linear memory without an explicit maximum size is not permitted." | confirmed; no change |
| R1-BND-F18 | Wasm threads need nightly Rust, `+atomics`, a rebuilt std and the `web` or `no-modules` output | guide raytrace.md | verifier: confirmed; rust-reviewer: confirmed | all six quotes found word for word | confirmed; no change |
| R1-BND-F19 | `std::thread::spawn` panics; the default Wasm features; target tier 2 | rustc book, wasm32-unknown-unknown | verifier: confirmed; rust-reviewer: confirmed | page body | fixed: the `-Cpanic=abort` sentence added (R1-RR-OQ3) |
| R1-BND-F20 | isolation is COOP `same-origin` plus COEP; it unlocks finer timers and the memory API | MDN COOP and crossOriginIsolated | verifier: confirmed; rust-reviewer: — | COOP intro; the crossOriginIsolated feature list | confirmed; REC5 now names the memory API it unlocks |
| R1-BND-F21 | `require-corp` needs CORP on cross-origin resources; what `credentialless` does; frames are covered | MDN COEP and web.dev coop-coep | verifier: confirmed; rust-reviewer: — | directive descriptions; "The entire chain of the documents, including parent frames and child frames, must be cross-origin isolated, too." | confirmed; no change |
| R1-BND-F22 | Safari has no `credentialless`; the version numbers | MDN browser-compat data, COEP json | verifier: confirmed; rust-reviewer: — | header 83 / mirror / 79 / 15.2; `credentialless` 96 / mirror / 119 / false | confirmed; no change |
| R1-BND-F23a | COOP `same-origin` cuts the link to popups | MDN COOP | verifier: confirmed; rust-reviewer: — | "A document with `same-origin` will always open a document with `unsafe-none` in a new BCG." | confirmed; REC5's login reasoning now rests on F20 and F23a |
| R1-BND-F23b | `same-origin-allow-popups` is meant for OAuth and payments | MDN COOP | verifier: confirmed; rust-reviewer: — | "…when using a cross-origin service for OAuth or payments." | confirmed; no change |
| R1-BND-F23c | `same-origin-allow-popups` does not give isolation | MDN COOP and web.dev | verifier: unsupported; rust-reviewer: — | neither source says it, and both read the other way: MDN, "integrations where a document needs the benefits of cross-origin isolation"; web.dev change… | now an open question: R1-BND-OQ9 (clause dropped from F23; REC5 rests on F20 and F23a) |
| R1-BND-F24a | the report-only header lets you test before enforcing | web.dev coop-coep | verifier: confirmed; rust-reviewer: — | "You will receive reports without blocking embedded content." | confirmed; no change |
| R1-BND-F24b | the headers must also cover worker scripts | web.dev coop-coep | verifier: confirmed (the doc says "inferred"; it can be upgraded); rust-reviewer: — | "Recursively apply this to all documents including the top-level document, iframes and worker scripts." | fixed: label upgraded to confirmed |
| R1-BND-F25 | module workers work in all four browsers | browser-compat data Worker.json; guide wasm-in-web-worker.md | verifier: confirmed; rust-reviewer: confirmed | `options_type_parameter` 80 / mirror / 114 / 15; the guide's "only Chrome supports modules in web workers" is out of date, as the doc says | confirmed; no change |
| R1-BND-F26 | SharedArrayBuffer and Atomics are in all four; `waitAsync` is "Baseline 2025" | browser-compat data SharedArrayBuffer and Atomics; MDN waitAsync | verifier: confirmed; rust-reviewer: — | all values match; "Since November 2025, this feature works across the latest devices and browser versions." | confirmed; no change |
| R1-BND-F27 | Wasm threads and the default Rust features are in all three engines | webassembly.org features.json | verifier: confirmed; rust-reviewer: — | all values match; "Edge follows Chrome" is inferred (no Edge key); `mutable-globals` is missing from the doc's list | fixed: the mutable-globals gap noted |
| R1-BND-F28 | wasm-bindgen supports all four browsers | guide browser-support.md | verifier: confirmed; rust-reviewer: confirmed | "Firefox, Chrome, Safari, and Edge browsers are all supported by `wasm-bindgen`." | confirmed; no change |
| R1-BND-F29 | WebTransport and WebSocket both work in workers | MDN WebTransport and WebSocket | verifier: confirmed; rust-reviewer: — | "This feature is available in Web Workers."; "Since March 2026, this feature works across the latest devices and browser versions." | confirmed; no change |
| R1-BND-F30 | timers are 5 µs isolated and 100 µs not; a worker has its own time origin | MDN Performance.now | verifier: confirmed; rust-reviewer: — | "Security requirements"; intro, "the time when the worker is run in Worker and ServiceWorker contexts" | confirmed; added to REC2's Rests on |
| R1-BND-F32 | `hardwareConcurrency` is not a hardware fact | MDN Navigator.hardwareConcurrency (last modified 2024-10-16) | verifier: — (no first-pass row); second pass: confirmed; rust-reviewer: — | "The browser may, however, choose to report a lower number of logical cores … so don't treat this as an absolute measurement of the number of cores in the user's system." | confirmed; no change (row added at the second fix round: R1-BND-REC1 rests on it) |
| R1-BND-F33 | Vite bundles workers and Wasm and can set dev-server headers | vite.dev features and server-options (v8.3.4) | verifier: confirmed; rust-reviewer: — | Web Workers, "Import with Constructors"; WebAssembly, "Manual Initialization"; `server.headers` | confirmed; no change |
| R1-BND-F34 (added at the fix step) | postMessage copies through structured clone | MDN Structured clone algorithm (2025-06-29) | verifier: — (no first-pass row); second pass: confirmed; rust-reviewer: — | first paragraph: "…to transfer data between Workers via postMessage()…"; no sentence on transferables, as the finding says | confirmed; no change |
| R1-BND-F35 (added at the fix step) | a Wasm memory's buffer has a detach key | WebAssembly JS API §5.3 | verifier: — (no first-pass row); second pass: confirmed for the step; rust-reviewer: — | "create a fixed length memory buffer", step 5: "Set buffer.[[ArrayBufferDetachKey]] to "WebAssembly.Memory"." (the draft now dated 28 October 2026) | confirmed; the draft's date noted in F35 and in the fix-step sources table (second fix round); throw-on-transfer stays inferred |
| CS1 | fixed step, button bitmask and input command declared but never wired | shamble-core.js:1479-1481 and the legacy inventory | verifier: confirmed; rust-reviewer: — | code | confirmed; no change |
| GP2 | actions run at event time; press and release edges in v2 | legacy inventory, row GP2 | verifier: confirmed; rust-reviewer: — | row | confirmed; no change |
| GP13 | rAF loop with variable dt clamped to 0.05 s; the world freezes for pause, shop, tutorial and hit-stop | shamble-game.js:3053 | verifier: confirmed; rust-reviewer: — | `loop()` | confirmed; no change |
| GP14 | HUD writes interleaved through the step; `update()` costs 0.49 ms | inventory row GP14; AUDIT_BUGS_PERF.md:10 | verifier: confirmed; rust-reviewer: — | row; runtime profile | confirmed; no change |
| GP10 | about 30 DOM writes per frame fell to 0.01 real writes | AUDIT_BUGS_PERF.md:27 and :85 | verifier: confirmed; rust-reviewer: — | P3; CS3 | confirmed; no change |
| GC2 | 12 fixed slots; 0.80–1.00 ms per step at 48 soldiers | inventory row GC2; docs/reports/2026-10-16-three-teams-core.md:48 | verifier: confirmed; rust-reviewer: — | row; table | confirmed; no change |
| GA15 | one state struct per soldier; aim pitch travels in a side channel | inventory row GA15 | verifier: confirmed; rust-reviewer: — | row | confirmed; no change |
| VISION_V2 §3 | "Bots fill empty slots." | VISION_V2 §3 | verifier: confirmed; rust-reviewer: — | the [DEFAULT] line | confirmed; no change |
| VISION_V2 §4 | four browsers; integrated-graphics laptop at 30+ fps; fair up to about 100 ms of ping | VISION_V2 §4 | verifier: confirmed; rust-reviewer: — | lines 1, 2 and 4 | confirmed; no change |
| VISION_V2 §5 | guest play first, login later | VISION_V2 §5 | verifier: confirmed; rust-reviewer: — | line 1 | confirmed; no change |
| LOOP_V2 §11 | scale points N = 12, 24, 36, 48 | LOOP_V2 §11 | verifier: confirmed; rust-reviewer: — | "Then run the scale test on this map (N = 12, 24, 36, 48)." | confirmed; no change |
| REC2 basis | the sim's own Wasm memory cannot be handed to the page by transfer (needed for "one block copy per tick"; no finding) | WebAssembly JS API §5.3 | verifier: missing claim (finding 1) | §5.3 "Memories": "Set buffer.[[ArrayBufferDetachKey]] to "WebAssembly.Memory"." | re-sourced: R1-BND-F35 |

### 5. Abilities and effects (unit `abilities`)

| Claim id | Claim | Source | Verdict (verifier; rust-reviewer) | Location | Final state |
|---|---|---|---|---|---|
| R1-ABL-F1 | An attribute has a base and a current value | tranek README | verifier: confirmed; rust-reviewer: — | §4.3.2, l.510 | confirmed; no change |
| R1-ABL-F2 | Three duration types | tranek | verifier: confirmed; rust-reviewer: — | §4.5.1 table, l.839-841 (the doc says §4.5) | fixed: section corrected to §4.5.1 |
| R1-ABL-F3 | Periodic effects; they are not predicted | tranek | verifier: confirmed; rust-reviewer: — | §4.5.1 l.843 "**Note:** `Periodic Effects` cannot be [predicted]"; §4.10 l.2569 | fixed: section corrected to §4.5.1 |
| R1-ABL-F4 | Aggregation formula; Multiply modifiers summed; Override | tranek | verifier: confirmed; rust-reviewer: — | §4.5.4 l.909-914; summing quote at §4.5.4.1 l.933 | fixed: Override rule quoted; summing location added |
| R1-ABL-F5 | Stacking: Duration/Infinite only, by source or target, three policies | tranek | verifier: confirmed (its "only by tooltips" is overstated); rust-reviewer: — | §4.5.5 l.1041-1050 | fixed: "only" removed (see C1) |
| R1-ABL-F6 | Tags are hierarchical and counted | tranek | verifier: confirmed; rust-reviewer: — | §4.2 l.419, l.427 | confirmed; no change |
| R1-ABL-F7 | Granted tags; ongoing tag requirements | tranek | verifier: confirmed; rust-reviewer: — | §4.5.7 l.1081-1082 | confirmed; no change |
| R1-ABL-F8 | Effects grant abilities, with a removal policy | tranek | verifier: confirmed; rust-reviewer: — | §4.5.6 l.1060-1070 | confirmed; no change |
| R1-ABL-F9 | Executions are not predicted and must be C++ | tranek | verifier: confirmed; rust-reviewer: — | §4.5.12 l.1251; §4.10 l.2561 | confirmed; no change |
| R1-ABL-F10 | Cost is an Instant effect; cooldown is a Duration effect plus a tag | tranek | verifier: confirmed; rust-reviewer: — | §4.5.14 l.1363; §4.5.15 l.1399 | confirmed; no change |
| R1-ABL-F11 | Required, blocked and cancel tags; stun | tranek | verifier: confirmed; rust-reviewer: — | §4.6.9 l.2089-2093; §5.1 l.2885 | confirmed; no change |
| R1-ABL-F12 | Local Predicted runs on the client first | tranek | verifier: confirmed; rust-reviewer: — | §4.6.8 l.2076 | confirmed; no change |
| R1-ABL-F13 | Lists of what is and is not predicted | tranek | verifier: confirmed; rust-reviewer: — | §4.10 l.2557-2569 | confirmed; no change |
| R1-ABL-F14 | The prediction key's round trip | tranek | verifier: confirmed; rust-reviewer: — | §4.10.1 l.2598, l.2610 (the doc says §4.10) | fixed: section corrected to §4.10.1 |
| R1-ABL-F15 | Paragon and Fortnite do not predict damage; predict the minimum | tranek | verifier: confirmed; rust-reviewer: — | §4.10 l.2551-2555 | fixed: the attribution upgraded to confirmed |
| R1-ABL-F16 | Cues are non-gameplay and unreliable | tranek | verifier: confirmed; rust-reviewer: — | §4.8.1 l.2303; "considered unreliable" is at §4.8.9 l.2507 (the doc says §4.8.1) | fixed: §4.8.9 added for the reliability sentence |
| R1-ABL-F17 | Fortnite keeps its own bookkeeping for weapon fire rates | tranek | verifier: confirmed; rust-reviewer: — | §11.1.1, Q7, l.3352-3358 | fixed: latency context upgraded to confirmed; REC2's reason and REC4 now rest on it |
| R1-ABL-F18 | The int16 activation key used as a shared seed | tranek | verifier: confirmed; rust-reviewer: — | §5.5 l.2942 | fixed: both inferred points upgraded; added to REC3's Rests on |
| R1-ABL-F19 | Paragon slows: only the strongest applies | tranek | verifier: confirmed; rust-reviewer: — | §5.7 l.2959 | confirmed; no change |
| R1-ABL-F21 | Quake III runs the same player code on both sides and adds a fire event | bg_pmove.c | verifier: confirmed; rust-reviewer: — | l.23-24; `PM_Weapon` l.1644 | confirmed; added to REC2's Rests on |
| R1-ABL-F22 | Integer millisecond timer; `+=` carries the overshoot; releasing the trigger resets it; ammo is decremented; i… | bg_pmove.c, plus q_shared.h l.1157 `int weaponTime;` | verifier: confirmed; rust-reviewer: — | `PM_Weapon` l.1576-1705 | confirmed; no change |
| R1-ABL-F23 | leafwing_abilities 0.13.0 | docs.rs | verifier: —; rust-reviewer: confirmed (bevy ^0.19 now C) | "Built for the Bevy game engine." | fixed: Bevy ^0.19 upgraded to confirmed |
| R1-ABL-F24 | bevy_gauge 0.5.1 | docs.rs | verifier: —; rust-reviewer: confirmed (R1: inferred) | "Define attributes as expressions, not update systems." | fixed: label upgraded to confirmed |
| R1-ABL-F25 | bevy_alchemy 0.6.0 | docs.rs README | verifier: —; rust-reviewer: confirmed (R1: inferred) | "New applications will overwrite the existing one." | fixed: label upgraded to confirmed |
| R1-ABL-F26 | `HashMap` is randomly seeded and iterates in arbitrary order | doc.rust-lang.org | verifier: confirmed; rust-reviewer: — | overview; `iter` | confirmed; no change |
| R1-ABL-N1 | 48 soldiers = 24v24 | VISION_V2 §3 | verifier: confirmed; rust-reviewer: — | l.44 | confirmed; REC6 now rests on it |
| R1-ABL-N2 | "48 simulated players at a 30 Hz server tick within X ms" | V2_PHASE0_BRIEF | verifier: confirmed; rust-reviewer: — | l.138-139 | confirmed; REC6 now rests on it |
| R1-ABL-N3 | "about 100 ms of ping" | VISION_V2 §4 | verifier: confirmed; rust-reviewer: — | l.54 | confirmed; no change |
| R1-ABL-N4 | Shabzon 0.072 s, so 13.9 shots/s | shamble-core.js | verifier: confirmed; rust-reviewer: — | :114 (the 13.9 is in the GC5 row; REC2 cites CD12) | confirmed; no change |
| R1-ABL-N5 | 10.0 vs 13.9 shots/s at 30 Hz; 28 % loss | GC5 | verifier: confirmed; rust-reviewer: — | LEGACY_INVENTORY l.244; 28.1 % by arithmetic; not re-measured | confirmed; no change |
| R1-ABL-N6 | Warrior kills Conscript in 0.86 s | CD13 | verifier: confirmed; rust-reviewer: — | l.94; recomputed: 13 shots of 8 against 100 HP, 12 × 0.072 = 0.864 s | confirmed; no change |
| R1-ABL-N7 | Head 2.6, body 1.0, legs 0.65; Bagpipe head 1.2 | shamble-core.js | verifier: confirmed; rust-reviewer: — | :88, :112 | confirmed; no change |
| R1-ABL-N8 | Shabzon falloff 15 / 35 m, minimum 0.2; `falloffMult` | code | verifier: confirmed; rust-reviewer: — | core:114, game:1484 | confirmed; no change |
| R1-ABL-N9 | Torque Bow fuse 0.55 s | shamble-game.js | verifier: confirmed; rust-reviewer: — | :1787 | confirmed; no change |
| R1-ABL-N10 | Shabzon damage 8 | shamble-core.js | verifier: confirmed; rust-reviewer: — | :114 | confirmed; no change |
| R1-ABL-N11 | Banzai speed ×1.3; Shield slow ×0.72 | code | verifier: confirmed; rust-reviewer: — | game:2774; core:85 | confirmed; no change |
| R1-ABL-N12 | "Stolen items only last until the Hammer dies." | LOOP_V2 §6 | verifier: confirmed; rust-reviewer: — | l.86 | confirmed; REC1 mark 3 now follows its "item" and "last hit or tick" wording |
| R1-ABL-N13 | Abilities locked until Tech 1 | LOOP_V2 §4 | verifier: confirmed; rust-reviewer: — | l.50, l.58 | confirmed; no change |
| R1-ABL-N14 | Feel kit, split feedback, active reload with a short damage boost | LOOP_V2 §7 | verifier: confirmed; rust-reviewer: — | l.90-94 | confirmed; no change |
| R1-ABL-N15 | Vehicles have "(leech, marking)" | VISION_V2 §6.4 | verifier: confirmed for the names only; reading "marking" as "an effect that grants a tag" is inference; rust-reviewer: — | l.71 | confirmed for the names; REC1 now marks "marking as a tag-granting effect" as our reading |
| R1-ABL-N16 | The input command has a sequence field | CS1 | verifier: confirmed; rust-reviewer: — | LEGACY_INVENTORY l.169 | confirmed; no change |
| R1-ABL-C1 | REC1: "F5 says GAS's own policies are only documented in tooltips" | tranek §4.5.5 | verifier: unsupported; rust-reviewer: — | l.1050 says only "They have helpful hover tooltips in the `GameplayEffect` Blueprint." There is no "only". | fixed: REC1's reason rewritten to the source's words (no "only") |
| R1-ABL-C2 | REC2: F17 shows Fortnite keeps "the hottest path ... out of effect bookkeeping" | tranek §11.1.1 Q7 | verifier: unsupported; rust-reviewer: — | The source's reason is latency: "Higher latency connections would have a lower ROF with basic attacks." | fixed: REC2's reason re-based on the source's latency answer (R1-ABL-F17, confirmed); the hot-path point marked as our inference |

### 6. Data formats and hot reload (unit `data-hotreload`)

| Claim id | Claim | Source | Verdict (verifier; rust-reviewer) | Location | Final state |
|---|---|---|---|---|---|
| R1-DATA-F1 | deny_unknown_fields errors on unknown keys; by default JSON ignores them | serde container-attrs.md | verifier: confirmed; rust-reviewer: confirmed | deny_unknown_fields: "Always error during deserialization when encountering unknown fields." / "When this attribute is not present, by default unknown… | confirmed; no change |
| R1-DATA-F2 | cannot be combined with flatten | same | verifier: confirmed; rust-reviewer: confirmed | deny_unknown_fields note: "this attribute is not supported in combination with flatten, neither on the outer struct nor on the flattened field." | confirmed; no change |
| R1-DATA-F3 | try_from loads through a fallible conversion | same | verifier: confirmed; rust-reviewer: confirmed | try_from: "Deserialize this type by deserializing into `FromType`, then converting fallibly." | confirmed; no change |
| R1-DATA-F4 | default and alias | serde field-attrs.md | verifier: confirmed; rust-reviewer: confirmed | default: "If the value is not present when deserializing, use the `Default::default()`."; alias: "Deserialize this field from the given name *or* from… | confirmed; no change |
| R1-DATA-F5 | untagged gives poor errors; internal tag exists | container-attrs.md | verifier: confirmed; rust-reviewer: confirmed | untagged: "When no variant matches, `untagged` does not produce an informative error"; tag: "Use the internally tagged enum representation, with the g… | fixed: the rest of the sentence added |
| R1-DATA-F6 | float_roundtrip is exact, about 2x float-parse cost | serde_json Cargo.toml 1.0.154 | verifier: confirmed; rust-reviewer: confirmed | float_roundtrip comment, matches the draft word for word | confirmed; no change |
| R1-DATA-F7 | one-based line and column; Syntax vs Data | docs.rs serde_json Error | verifier: confirmed; rust-reviewer: confirmed | line(): "One-based line number at which the error was detected."; classify(): "Category::Data - input data that is semantically incorrect" | confirmed; no change |
| R1-DATA-F8 | serde_path_to_error gives the field path | docs.rs 0.1.20 | verifier: confirmed; rust-reviewer: confirmed | crate root: "exposes the chain of field names leading to the error."; example "dependencies.serde.version" | confirmed; no change |
| R1-DATA-F9 | recursion limit on by default; unbounded_depth is opt-in | Cargo.toml + docs.rs Deserializer | verifier: confirmed; rust-reviewer: confirmed (value unstated) | disable_recursion_limit: "This method is only available if serde_json is built with the "unbounded_depth" feature." (the default limit's value is not… | confirmed; no change |
| R1-DATA-F10 | untyped Value tree, then from_value | docs.rs serde_json::value | verifier: confirmed; rust-reviewer: confirmed | "The Value enum, a loosely typed way of representing any valid JSON value."; from_value: "Interpret a `serde_json::Value` as an instance of type `T`." | confirmed; no change |
| R1-DATA-F11 | preserve_order | Cargo.toml | verifier: confirmed; rust-reviewer: confirmed | preserve_order comment, verbatim | confirmed; added to REC1's Rests on |
| R1-DATA-F12 | RON's limits and features | github ron-rs/ron README; docs.rs ron | verifier: confirmed; rust-reviewer: confirmed (now 0.12.3, 2026-10-28) | "RON is not designed to be a fully self-describing format (unlike JSON)"; "limited support" list covers tag, untagged and flatten; the feature list is… | fixed: version note (0.12.3) and the feature list upgraded |
| R1-DATA-F13 | TOML has comments, no null; the crate targets spec 1.1.0 | toml.md; docs.rs toml | verifier: confirmed; rust-reviewer: confirmed | "A hash symbol marks the rest of the line as a comment, except when inside a string."; the "Values must have one of the following types." list has no… | confirmed; no change |
| R1-DATA-F14 | serde_yaml archived | github dtolnay/serde-yaml | verifier: confirmed; rust-reviewer: confirmed | banner: "This repository was archived by the owner on Mar 25, 2024." and "This project is no longer maintained." | confirmed; no change |
| R1-DATA-F15 | JSON Schema bounds, enum, required, UI annotations | draft-bhutton-json-schema-validation-01 | verifier: confirmed; rust-reviewer: — | §6.2.4 "minimum" definition; §9 "documentation and user interface display purposes"; §9.3 "applications SHOULD refrain from usage of the declared prop… | fixed: deprecated located at §9.3 |
| R1-DATA-F16 | schemars writes 2020-12 and follows serde | docs.rs schemars 1.2.3 | verifier: confirmed; rust-reviewer: confirmed | "The default settings currently conform to JSON Schema 2020-12,"; "Any generated schema should match how serde_json would serialize/deserialize to/fro… | confirmed; no change |
| R1-DATA-F17 | schemars reads garde range, length and pattern, and doc comments | schemars attributes.md | verifier: confirmed; rust-reviewer: confirmed | "many of which will be used by Schemars to generate more accurate schemas."; "Sets the `minimum`/`maximum` properties for number schemas." | confirmed; no change |
| R1-DATA-F18 | garde: Validate, a context value, error paths | docs.rs garde 0.23.0 | verifier: confirmed; rust-reviewer: confirmed | "The context may be any type without generic parameters."; "username[0]: length is lower than 3"; no mention of Wasm (matches the draft and OQ5) | confirmed; REC2 now names its open Wasm dependency (OQ5) |
| R1-DATA-F19 | Ajv validates 2020-12 | github ajv README | verifier: confirmed; rust-reviewer: — | Features: "Ajv implements JSON Schema draft-06/07/2019-09/2020-12 standards" | confirmed; no change |
| R1-DATA-F20 | notify's known gaps; debouncers | docs.rs notify 8.2.1 | verifier: confirmed; rust-reviewer: confirmed | Known Problems: "Network mounted filesystems like NFS may not emit any events for notify to listen to." | confirmed; no change |
| R1-DATA-F21 | Bevy hot reload is desktop-only, behind file_watcher | docs.rs bevy_asset 0.20.0 | verifier: confirmed; rust-reviewer: confirmed | "To enable asset hot reloading on desktop platforms, enable `bevy`'s `file_watcher` cargo feature." | confirmed; REC5 now marks the browser part as inferred |
| R1-DATA-F22 | Vite server.ws.send / hot.on / hot.send | vite.dev api-plugin and api-hmr (v8.3.4) | verifier: confirmed; rust-reviewer: — | Client-server Communication; hot.send: "Send custom events back to Vite's dev server."; no mention of workers | confirmed; no change |
| R1-DATA-F23 | "__proto__" in JSON; accessor; Object.assign uses [[Set]] | MDN (3 pages) | verifier: confirmed; rust-reviewer: — | "In JSON, `"__proto__"` is a normal property key."; proto page: accessor sentence; assign Description: "It uses `[[Get]]` on the source and `[[Set]]`… | confirmed; no change |
| R1-DATA-F24 | use Map/Set instead of object literals | OWASP Prototype Pollution cheat sheet | verifier: confirmed; rust-reviewer: — | "Developers should use `new Set()` or `new Map()` instead of using object literals:" | confirmed; no change |
| R1-DATA-F25 | validate syntax and meaning, on the server; set required and additional properties | OWASP Input Validation cheat sheet | verifier: confirmed; rust-reviewer: — | Common Pitfalls: "Validate on the server even when the browser checks the same fields." "Client-side validation is bypassable; use it for immediate fe… | confirmed; no change |
| R1-DATA-F26 | JSON.parse numbers lose precision | MDN JSON.parse | verifier: confirmed; rust-reviewer: — | Description > The reviver parameter: "may lose precision in the process." | confirmed; no change |
| R1-DATA-F27 | immutable plus hashed URLs | MDN Cache-Control | verifier: confirmed; rust-reviewer: — | Response Directives > immutable: both sentences verbatim | confirmed; no change |
| R1-DATA-F28 | include_bytes | doc.rust-lang.org (1.99.1) | verifier: confirmed; rust-reviewer: — | "Includes a file as a reference to a byte array." | confirmed; no change |
| R1-DATA-F29 | SHA-256 in Web Workers, secure contexts only | MDN SubtleCrypto.digest | verifier: confirmed; rust-reviewer: — | "This feature is available in Web Workers."; "available only in secure contexts (HTTPS)" | confirmed; REC6 now carries its secure-context caveat (its "file://" part, wrong per the second pass, cut at the second fix round) |
| Legacy facts | legacy row Legacy facts | repo | verifier: confirmed; rust-reviewer: — | shamble-game.js:41 rulesMerge Object.assign; shamble-core.js:180 envMerge; shamble-game.js:661 version:1, read nowhere in shamble-*.js; shamble-editor… | confirmed; no change |
| VISION_V2 §4, 100 ms | docs/VISION_V2.md:54 | VISION_V2 §4, 100 ms | verifier: confirmed; rust-reviewer: — | "`[DEFAULT]` Matches feel fair up to about 100 ms of ping." | confirmed; no change |
| Design references | legacy row Design references | repo | verifier: confirmed; rust-reviewer: — | LOOP_V2.md:8 "one data table"; LOOP_V2 §7 Feel Lab `[DECIDED]`; EDITOR_V2 §2 rule 1, §3 validation panel ("the 6 m gap rule"), §4, §5; STATUS.md:96 SE… | confirmed; EDITOR_V2 §2 rule 1 and §3 added to REC1's and REC7's Rests on |
| REC6 file:// (added at the fix step) | "file:// and plain-HTTP LAN development need another path" (a secure context is needed) | MDN Secure contexts (last modified 2026-09-14) | verifier: — (no first-pass row); second pass: wrong for file://; rust-reviewer: — | its table of top-level documents: "`file:///path/to/resource.html` ✅ Secure (`file` URL)"; the plain-HTTP LAN part follows from F29 | fixed: "file://" cut from REC6 (plain-HTTP LAN kept, F29); R1-DATA-OQ11's matching sentence corrected |

### Other reviewer findings and what the fix step did
- **Rests-on lists completed** (verifier, every unit): R1-ECS-REC1 (F7, F19, F20, F32, F35, the legacy kinds, CS7, GC2,
  VISION_V2 §4), REC2 (GP14), REC3 (CS7, GC15); R1-TICK-REC2 (F3, GN4); R1-DET-REC1 (TL13, STATUS.md:70), REC2 (F3,
  F18, F20, F22), REC5 (F33, GP14), REC6 (F3, F33, GN8); R1-BND-REC1 (F32), REC2 (F30, F35), REC3 (F34); R1-ABL-REC2
  (F21), REC3 (F18, GC5), REC4 (F17), REC5 (CD12–CD14, CS3, CC9), REC6 (the brief's example and VISION_V2 §3);
  R1-DATA-REC1 (F11, EDITOR_V2 §2 rule 1), REC2 (the open OQ5 and OQ11), REC7 (EDITOR_V2 §3).
- **Recommendation text, reasons and marks changed:** R1-ECS-REC1 (reason: the clone_from overreach, bevy_ggrs named as
  third-party, hecs's F19 and F20 weighed, the uneven 0.x argument; mark 2 rewritten); R1-TICK-REC2 ("halves" marked
  inferred, "every legacy measurement" corrected, P3's explanation); R1-DET-REC4 (Pcg32 fallback sourced), REC5 and REC6
  ("per process" corrected); R1-BND-REC2 (why the one copy), REC3 (basis F34), REC5 (the costs now each name a finding;
  the login reasoning on F20 and F23a, with OQ9); R1-ABL-REC1 (C1; marking marked as our reading; mark 3 follows
  LOOP_V2 §6), REC2 (C2), REC4 (the effect-removal gap and F17's known failure stated); R1-DATA-REC5 (F21's inference
  labelled; the partial §6 trace stated, OQ12), REC6 (secure-context caveat). No pass-mark number changed.
- **VISION_V2 links corrected:** R1-TICK-REC3 (§3, not §6.1), REC4 (the counters trace to no §6 item); R1-DET-REC6
  (§6.6 wording); R1-BND-REC1 (§6.2 wording), REC6 (§6.1 dropped; offline play traces to no §6 item, OQ1);
  R1-ABL-REC6 (no §6 item: a budget).
- **Labels, tiers, versions and locations:** the upgrades and corrections in the "Final state" column; tiers of the two
  Fiedler articles (R1-XU-OQ6 settled); the HashMap seed (R1-XU-OQ5 settled); ron 0.12.3; the rustwasm book archived.
- **Rust risks** (rust-reviewer's ranked list): risk 5 fixed in place (R1-ECS-REC1 mark 2, R1-ECS-F35, R1-ECS-OQ2
  settled); risk 9 joined R1-DET-OQ5; risks 2, 3, 4, 6, 7 and 8 and the tokio part of risk 1 became R1-RR-OQ1–OQ7.

### Findings not applied as written, and why
- **rust-reviewer's missed risks (2–4, 6–8) are open questions, not recommendation changes.** The reviewer asks for them
  to go into the recommendations; the fix step may add no recommendation and no claim of its own (fix rule 6), and
  their evidence (wide, simba, Cargo profiles, RustSec) was read by the reviewer, not by a unit or the verifier. The
  synthesis decides (R1-RR-OQ1–OQ7). Risks 1 (beyond tokio) and 10 are R2's.
- **R1-TICK-REC5's "read from shared memory" is kept** (verifier tick finding 7): it is a disagreement between two
  units, which the assembly keeps on both sides by rule; R1-XU-OQ3 decides it. The fix step only added a pointer.
- **R1-DATA-REC5's mid-match reload is kept, not cut back** (verifier data finding 3 offered "argue or cut back"): the
  trace is now stated as partial and the choice is R1-DATA-OQ12, since dropping a development feature is Mustafa's and
  the Advisor's call.
- **R1-TICK-REC4's overrun counters are kept** (verifier tick finding 6): the recommendation's own pass marks read them;
  their trace is now stated honestly (operations monitoring, R6's scope, no §6 item).
- **R1-DET-REC5's link to §6.5 is kept** (verifier determinism finding 7, "loose"): server-side outcomes such as steals
  depend on a fixed processing order (our inference).
- **The GAS facts still come from one community write-up** (verifier abilities finding 9): Epic's documentation is
  outside the allow list; R1-ABL-OQ1 stands.

### Second verifier pass (2026-10-28)
The verifier re-checked everything the fix step changed: the five findings it added (A), the claims recommendations
rest on that had no first-pass "confirmed" (B), the findings whose text changed (C), one new claim inside a
recommendation (D), the change log's number lines (E) and this part's counts (F). Scope list:
`.checkpoints/v2-research/second-pass-scope-r1.md`; report: `verifier-r1-second-pass-result.md`, every quote in full in
`verifier-r1-second-pass.md`. Verdict: PASS WITH NOTES. Each verdict below is as the verifier gave it; "Final state" is
after the second fix round (engine-architect, 2026-10-28).

| Scope | Claim id | Claim | Source | Verdict (second pass) | Location or exact words | Final state |
|---|---|---|---|---|---|---|
| A | R1-DET-F33 | the HashMap seed is per instance | std HashMap 1.99.1 | confirmed | "Usage in `const` and `static`": "As explained above, `HashMap` is randomly seeded: each `HashMap` instance uses a different seed, which means that `HashMap::new` normally cannot be used in a `const` or `static` initializer."; no sentence ties the seed to "process" | confirmed; no change |
| A | R1-BND-F34 | postMessage copies through structured clone | MDN Structured clone algorithm (last modified 2025-06-29) | confirmed | first paragraph: "The structured clone algorithm copies complex JavaScript objects. It is used internally when invoking structuredClone(), to transfer data between Workers via postMessage(), …"; the page has no sentence on transferables, as the finding says | confirmed; no change |
| A | R1-BND-F35 | a Wasm memory's buffer has a detach key | WebAssembly JS API §5.3 | confirmed for the step | "create a fixed length memory buffer", step 5: "Set buffer.[[ArrayBufferDetachKey]] to "WebAssembly.Memory"."; the draft now reads "Editor's Draft, 28 October 2026" (the file said 27 October; date only) | fixed: the date noted in F35 and in the fix-step sources table, row F3 |
| A | R1-ECS-F35 | derive and default clone_from; RigidBodySet keeps the default | std Clone 1.99.1; docs.rs rapier3d 0.36.1 | confirmed | "The `derive`d implementation of Clone calls clone on each field."; "`a.clone_from(&b)` is equivalent to `a = b.clone()` in functionality, but can be overridden to reuse the resources of `a` to avoid unnecessary allocations."; RigidBodySet clone_from: "Performs copy-assignment from `source`.", source link `https://doc.rust-lang.org/nightly/src/core/clone.rs.html#245-247` | confirmed; fixed (minor point on REC1): the head says "clones field by field" and "derive leaves clone_from at the default" is marked our reading |
| A | R1-DET-F34a | Pcg32 suits 32- and 64-bit CPUs, takes a stream in its constructor, can jump ahead | docs.rs rand_pcg 0.10.2 (crate root and Lcg64Xsh32) | confirmed | "This is a good choice on both 32-bit and 64-bit CPUs (for 32-bit output)."; `pub fn new(state: u64, stream: u64) -> Self` "Construct an instance compatible with PCG seed and stream."; "Multi-step advance functions (jump-ahead, jump-back)" | confirmed; no change |
| A | R1-DET-F34b | "the pages say nothing on how streams relate" | Lcg64Xsh32 page | **wrong** | `new` docs: "Note that two generators with different stream parameters may be closely correlated." (the struct docs say it again) | fixed: F34 quotes the warning; R1-DET-REC4's fallback and options Q4 no longer count the stream number as a merit; now an open question: R1-DET-OQ11 |
| B | R1-BND-F15 | a transfer moves the buffer and the sender loses it; typed arrays; WebTransport streams | MDN Transferable objects | confirmed (first-pass quotes, text unchanged) | the unsupported F15b clause is gone; R1-BND-REC3 now rests on F34 | confirmed; no change |
| B | R1-BND-F2 | version match since 0.2.128; 0.2.123 breaking; dates; year from the asset timestamps | GitHub releases list | confirmed | "This rejects the invalid widening but is a breaking change for callers that relied on widening `&mut` references."; "10 Apr" … "25 Sep"; asset timestamp "2026-09-25T07:11:34Z"; the 0.2.122 notes have no "breaking" | confirmed; no change |
| B | R1-BND-F23 | `same-origin` severs popups; `same-origin-allow-popups` is for OAuth and payments | MDN COOP (last modified 2026-07-12) | confirmed | "A document with `same-origin` will always open a document with `unsafe-none` in a new BCG."; "…For example, when using a cross-origin service for OAuth or payments."; F23c survives only as the correction note and R1-BND-OQ9, and R1-BND-REC5 does not use it | confirmed; no change |
| B | R1-BND-F32 | hardwareConcurrency is not a hardware fact | MDN (last modified 2024-10-16) | confirmed | "The browser may, however, choose to report a lower number of logical cores … so don't treat this as an absolute measurement of the number of cores in the user's system." | confirmed; row added to the boundary table |
| B | R1-DET-F25 | random seed, arbitrary order, hasher per map | std HashMap | confirmed | "The algorithm is randomly seeded, and a reasonable best-effort is made…"; "per-`HashMap` basis"; "per process" is gone from F25, R1-DET-REC5 and R1-DET-REC6 | confirmed; no change |
| B | R1-ECS-F19 | hecs freelist and its condition | docs.rs hecs 0.11.2 World | confirmed | "Entity handles will be allocated deterministically between different worlds (e.g. across serialization)"; "May only be called when the set of live entity IDs exactly matches those that were live" | confirmed; no change |
| B | R1-ECS-F20 | World has no Serialize; column format | hecs serialize and column pages | confirmed | "Components are not necessarily serializable, so we cannot directly implement serde::Serialize for World."; "Available on crate feature `column-serialize` only." | confirmed; no change |
| B | R1-ECS-F34 | no source gives a Wasm size or compile time | a claim of absence | unsupported (an absence cannot be quoted; harmless, since it only makes the size marks proposed) | — | fixed: its "Sure" says it is an absence no quote can confirm; R1-ECS-REC1 no longer lists it in "Rests on" (a note says why mark 5 measures) |
| B | R1-ECS-F7 | single-threaded executor; multi_threaded off by default | docs.rs bevy_ecs 0.20.0 | confirmed | "Runs the schedule using a single thread."; the four defaults are async_executor, backtrace, bevy_reflect and std | confirmed; no change |
| C | R1-ABL-F2, F3, F14, F16; R1-BND-F33; R1-DATA-F15; R1-DET-F8, F31, F32; R1-TICK-F1, F3 (11) | a label, location, date or tier changed | — | label only | — | no change |
| C | R1-ABL-F4, F5, F15, F18; R1-BND-F24, F27; R1-DATA-F5; R1-DET-F5, F30; R1-ECS-F30, F31, F33; R1-TICK-F12, F16 (14) | changed words | the first-pass notes | confirmed from the first-pass notes, no fetch | abilities notes l.8, l.50, l.19, l.22; boundary l.35, l.43; data l.6; determinism l.12, l.51; ecs l.36, l.37, l.42; tick l.33, l.22 | confirmed; no change |
| C | R1-ABL-F17 | high-latency players re-activate on a cooldown the server still holds | GAS README §11.1.1, question 7 | confirmed at the source | question: "In scenarios of high latency, this would let the Owning Client "try" to activate the ability again when its local cooldown expires" + "but it is still on cooldown on the Server."; answer: "The short answer there is not a way to prevent this and Paragon definitely had the problem." / "Higher latency connections would have a lower ROF with basic attacks." | confirmed; no change |
| C | R1-BND-F8 | tsify's `js` feature | docs.rs tsify | confirmed at the source | "`js` enables serialization through serde-wasm-bindgen and generates the appropriate types for it." | confirmed; no change |
| C | R1-BND-F9 | ts-rs's minimum Rust version | docs.rs ts-rs | confirmed at the source | "The Minimum Supported Rust Version for this crate is 1.88.0" | confirmed; no change |
| C | R1-BND-F17 | a shared memory import must be declared shared | threads proposal Overview.md | confirmed at the source | "It is a validation error to attempt to import shared linear memory" "if the module's memory import doesn't specify that it allows shared memory." | confirmed; no change |
| C | R1-BND-F19 | wasm32-unknown-unknown builds with panic=abort | rustc book | confirmed at the source | "By default the `wasm32-unknown-unknown` target is compiled with `-Cpanic=abort`." | confirmed; no change |
| C | R1-DATA-F12 | ron's version | docs.rs ron | confirmed at the source | "ron 0.12.3", dated "28 October 2026" | confirmed; no change |
| C | R1-DET-F9 | std `tan` is libc's `tanf` | std f32 | confirmed at the source | "This function currently corresponds to the `tanf` from libc on Unix and Windows." | confirmed; no change |
| C | R1-DET-F11 | `algebraic_add`'s release | std f32 | confirmed at the source | marked "1.98.0 (const: 1.98.0)" | confirmed; no change |
| C | R1-DET-F12 | NaN behaviour is non-deterministic on Wasm | RFC 3514 | confirmed at the source | "Also note that in some cases even the underlying target behavior is non-deterministic, namely on wasm." | confirmed; no change |
| C | R1-DET-F14 | `clamp` panics | std f32 | confirmed at the source | "Panics if `min > max`, `min` is NaN, or `max` is NaN." | confirmed; no change |
| C | R1-DET-F21 | `simd8` is its own determinism domain | Rapier determinism page | confirmed at the source | "…which changes the SIMD lane width and is therefore its own determinism domain." | confirmed; no change |
| C | R1-DET-F23 | Rapier's workspace pins and its fast-math comment | Rapier workspace Cargo.toml | confirmed at the source | pins 0.36.1, parry3d 0.31.1, nalgebra 0.35, simba 0.10.2 and glamx 0.3.1; fast-math "re-enables FMA contraction in glam's sse2 backend (a cross-platform determinism hazard)." | confirmed; no change |
| C | R1-ECS-F28 | slotmap's indirections and serde feature | docs.rs slotmap 1.1.1 | confirmed at the source | "It uses two indirections per random access…"; "If you wish to use these features you must enable the `serde` feature flag…" | confirmed; no change |
| C | R1-ECS-F29 | RigidBodySet does not override clone_from | docs.rs rapier3d 0.36.1 | confirmed at the source | the same check as R1-ECS-F35 | confirmed; no change |
| C | R1-TICK-F18 | Chrome 88's conditions for intensive throttling | Chrome 88 post | confirmed at the source | "The page has been hidden for more than 5 minutes." / "The chain count is 5 or greater." / "The page has been silent for at least 30 seconds." / "WebRTC is not in use." | confirmed; no change |
| D | R1-DATA-REC6 | "file:// and plain-HTTP LAN development need another path" (a secure context is needed) | MDN Secure contexts (last modified 2026-09-14) | **wrong for file://** | its table of top-level documents: "`file:///path/to/resource.html` ✅ Secure (`file` URL)"; the plain-HTTP LAN part follows from R1-DATA-F29 | fixed: "file://" cut from REC6 (plain-HTTP LAN kept, F29); R1-DATA-OQ11's matching sentence corrected |

**Recommendations the pass read** (its "Changed recommendations"), and what the second fix round did:

| Recommendation | Second pass | Final state |
|---|---|---|
| R1-DET-REC4 (and options Q4) | problem: the Pcg32 fallback lists "a stream number per generator" as a strength and leaves stream independence "for the spike"; the source warns that streams "may be closely correlated" | fixed: Pcg32 is a candidate fallback without the stream number as a merit; how it would keep purposes apart is now an open question: R1-DET-OQ11; options Q4 corrected |
| R1-DATA-REC6 | problem: the file:// clause is wrong (row D) | fixed: the clause cut; plain-HTTP LAN kept (F29); R1-DATA-OQ11 corrected |
| R1-ECS-REC1 | OK, one minor point: it states flatly that derived and default clone_from do not reuse buffers, which R1-ECS-F35 marks as inferred; F35's "derive writes only `clone`" is also a reading; R1-ECS-F34 is the absence row (harmless) | fixed: marked inferred in REC1 and in the options table; F35's head and "Sure" reworded; F34 out of "Rests on" |
| R1-DET-REC5, R1-DET-REC6 | OK, one minor point: "any two runs … also catch any order that leaks" claims more than F33 allows; F33's own inference says the maps "can" iterate differently | fixed: REC5's mark says two runs "can catch" and REC6's reason "can expose", both marked inferred |
| R1-TICK-REC2 | OK on numbers (TL13 and TL15 at 1/60, GW14 at 20, 60 and 144 Hz, GN4 at 60 and 144 fps; 0.2 s × 60 = 12 ticks and 0.2 s × 30 = 6); Serves glosses §6.1 as "(modes up to 48+ with bots)", which reads §3 into §6.1 | fixed: Serves names §3 (up to 24v24, bots fill empty slots) |
| R1-BND-REC5 | OK, minor points: "CORP or CORS", while R1-BND-F21 quotes only CORP; its Serves line names §4 and §5 and does not say it serves no §6 item | fixed: "or CORS" cut; Serves says no §6 item names isolation |
| R1-ECS-REC2, REC3, REC4; R1-TICK-REC3, REC4, REC5; R1-DET-REC1, REC2; R1-BND-REC1, REC2, REC3, REC6; R1-ABL-REC1–REC6; R1-DATA-REC1, REC2, REC5, REC7 | OK (checked: GC15; 0.25 s × 60 = 15 ticks; STATUS.md:70; LOOP_V2.md:85-86; 1 − 10.0/13.9 = 28.1 %; V2_PHASE0_BRIEF.md:138-139; EDITOR_V2.md:39) | no change |

**The change log and this part (E, F).** The fix step's four number lines check out (0.2.122 dropped from R1-BND-F2;
R1-BND-OQ7's "at least twice"; the header totals of 185 findings, 37 recommendations, 76 open questions and 10
fix-step reads, 3 of them new; source tiers 138 + 16 + 7 = 161). Part 7's 243 rows and its totals add up. Missing were
rows for R1-BND-F32 and the five new findings (added at the second fix round), and the sentence "No recommendation
rests on a claim whose final verdict is wrong or unsupported" did not hold until R1-DET-F34/REC4 and R1-DATA-REC6 were
fixed (now fixed).

**Other findings, ranked, and what was done.**
1. R1-DET-F34b, R1-DET-REC4 and options Q4: fixed (above).
2. R1-DATA-REC6's file:// clause: fixed (above).
3. Part 7's missing rows and this pass's verdicts: 8 rows added to the unit tables; this subsection.
4. The minor points in R1-ECS-REC1, R1-DET-REC5 and REC6, R1-TICK-REC2's Serves line and R1-BND-REC5: fixed (above).
5. Process: fix-step read F10 used the verifier's downloaded copy of the GAS README, not the page; the verifier opened
   the page and the quotes match. Noted in the fix-step sources table, row F10.
6. R1-BND-F35 and fix-step row F3 gave the draft's date as 27 October; the page now says 28 October: noted in both.

Not verified by the pass: that a transfer of the Wasm memory's buffer throws (ECMA-262 and the HTML standard are outside
the allow list; R1-BND-F35 keeps it inferred); the R1-ECS-F34 absence, which no quote can prove.

**Counts.**
- Second pass: 56 claims: 42 confirmed, 2 wrong (R1-DET-F34b; R1-DATA-REC6's file:// clause), 1 unsupported
  (R1-ECS-F34), 11 label only. The five claims re-sourced at the fix step: 4 confirmed whole, R1-DET-F34 confirmed in
  its quotes and wrong in its closing note.
- After the second fix round: the 2 wrong claims fixed (R1-DET-F34b in place, its recommendation clause now an open
  question: R1-DET-OQ11; R1-DATA-REC6's clause cut); the 1 unsupported claim fixed (R1-ECS-F34 out of "Rests on");
  0 re-sourced; 2 confirmed claims reworded (R1-BND-F35's date; R1-ECS-F35's reading marked); 7 recommendations
  changed (R1-DET-REC4, R1-DATA-REC6, R1-ECS-REC1, R1-DET-REC5, REC6, R1-TICK-REC2, R1-BND-REC5), no pass-mark number
  changed. Every verifier finding was applied. No recommendation rests on a claim whose final verdict is wrong or
  unsupported.

### Final check (2026-10-28)

The verifier checked only what the second fix round changed (report:
`.checkpoints/v2-research/verifier-r1-final-check-result.md`, notes `verifier-r1-final-check.md`). Verdict: PASS WITH
NOTES, nits only.
- Every item the second pass found wrong or unsupported, and every problem it named in a recommendation: closed.
- Claims the round changed, checked at the source: 7 confirmed, 0 wrong, 0 unsupported (R1-DET-F34's correlation
  warning and its re-laid quotes; MDN Secure contexts for R1-DATA-OQ11 and R1-DATA-REC6; R1-BND-F35's draft date;
  MDN COEP for R1-BND-REC5; std Clone for R1-ECS-REC1 and F35).
- Nits left as they are: options G2 and R1-BND-OQ5 call CORS "our reading", where F21's own source (MDN COEP) also says
  `cors`-mode requests "must still be permitted by CORS"; the labels understate the evidence and REC5's CORP-only rule
  is stricter, not wrong. Applied by the main session: the URL of MDN's Secure contexts page under "Sources read at the
  fix step".

### Consistency resolutions (2026-10-30)
Applied by engine-architect in step 3c of `.checkpoints/v2-research/plan-amendments-2.md`: the resolutions of the
consistency critic's 12 contradictions as the advisor approved them on 2026-10-30
(`.checkpoints/v2-research/resolutions.md`, verdict in `advisor-resolutions-verdict.md`), the terms of
[GLOSSARY.md](GLOSSARY.md) and the shared cross-topic open questions (`cross-topic-oqs.md`). Nothing was fetched; no
claim, number or source was added. Every "before" quote matched the file exactly once.
- **Resolution 1 (the server tick budget; REVISE, applied as folded):** C1.1 (R1-ECS-REC1 mark 1), C1.2 and C1.3
  (R1-TICK-REC2 P1), C1.4 (R1-DET-REC3), C1.5 (R1-ABL-REC6), C1.6 (R1-ECS-OQ4). R1-XU-OQ1 is settled through C9.8.
- **Resolution 5 (ABL-PREDICT, aligned):** C5.1 to C5.4 (R1-ABL-REC4), C5.5 (R1-ABL-OQ4), C5.6 (R1-DET-OQ6), C5.7 (the
  shared question inserted).
- **Resolution 6 (the spread stream):** C6.1 (R1-DET-REC4), C6.2 (R1-ABL-REC3), C6.3 (R1-DET-OQ9), C6.4 (SEC-SPREAD
  inserted).
- **Resolution 9 (the stress scene; REVISE, wording outside R1's texts):** C9.1 and C9.2 (R1-ECS-REC1 marks 1 and 2),
  C9.3 and C9.4 (R1-TICK-REC2 P1), C9.5 (R1-TICK-REC3), C9.6 (R1-DET-REC3), C9.7 (R1-ABL-REC6), C9.8 (R1-XU-OQ1).
- **Resolution 10 (MODES-3, escalated; the interim wording):** C10.1 (R1-DET-REC1), C10.2 (R1-TICK-OQ7), C10.3
  (MODES-3 inserted).
- **Resolution 11 (one vCPU of the reference server):** C11.1 (R1-ECS-REC1), C11.2 and C11.3 (R1-TICK-REC2), C11.4
  (R1-ABL-REC6), C11.5 (R1-ECS-OQ4), C11.6 (R1-TICK-OQ6), C11.7 (the pointers to R6, both phrases).
- **Resolution 17 (BASELINE-WHEN, escalated; the "now" changes):** C17.1 (R1-RR-OQ7), C17.2 (BASELINE-WHEN inserted).
  Not applied: C17.b1, which waits for the Advisor's answer to BASELINE-WHEN.
- **Resolution 22 (the isolation flag):** no R1 text change; the glossary link sits on R1-BND-REC5's first G3.
- **Resolution 29 (the hash-map ban; REVISE, C29.2 as folded):** C29.1 (R1-DET-REC5), C29.2 (R1-ECS-REC2), C29.3
  (R1-ECS-REC2 mark 2).
- **Resolutions 19, 21 and 23:** no R1 change.
- **Recommendation changes, for the verifier (advisor condition 4):** C1.1, C1.5, C5.1 to C5.4, C6.1, C6.2, C9.1,
  C9.3, C9.7, C29.1 to C29.3; and, under the rule "a time mark that names no statistic is a p99" (GLOSSARY: time
  statistics), R1-ABL-REC6 (through C1.5) and R1-ECS-REC1 marks 2 and 3, which now say "at p99".
- **Glossary terms:** the header and the first use of each term link GLOSSARY.md; every "Retire (exact)" phrase is
  gone outside quotes and dated records (the session's check script: 13 hits before, 0 after). Renamed without a value
  change: "tick time", "mean tick", "p99 tick" and "Tick cost" where only the sim is meant became sim step time
  (R1-ECS-REC1 mark 1, R1-TICK-REC2 P1 and P2, R1-TICK-REC3, R1-ECS-OQ4, R1-XU-OQ1, the boundary options row A2);
  R1-TICK-REC2 P2's 40 % is now named the client's sim share; "the tick budget" became the server tick budget
  (R1-XU-OQ1); "the period" the tick period (R1-XU-OQ1); "R2's partial prediction" R2's prediction scope (R1-ECS-REC1
  mark 3); R1-XU-OQ2's description of R1-ABL-REC4 follows C5.1; "R6's security baseline" became the security baseline
  (the pointers to other topics). R1-XU-OQ1's list keeps the figures each unit first proposed, as a record. In-context
  words that mean something else (effects in the ability model, vehicles in general, 48 in legacy measurements and the
  LOOP_V2 scale steps, text characters, Source's own tick interval) stay; the step-3d verifier judges them.
- **Shared questions added** (part 6, "Shared open questions (the same words in every topic)"): ABL-PREDICT,
  SEC-SPREAD, MODES-3, BASELINE-WHEN. **Cross-topic questions added** (part 6, "Cross-topic open questions
  (consistency critic, 2026-10-29)"): R1-XT-OQ1 to R1-XT-OQ18 in the critic's words, with a pointer line in each of
  the 17 recommendations they name.
- **OBJECT:** none.

Step 3d fixes (2026-10-30):
- R1-TICK-REC2 P1: "The other 25 % of the server tick budget" now reads "The other 25 % of the tick period, inside the
  server tick budget", the whole GLOSSARY gives the server-only share (4.2 ms at 60 Hz, as intended) — nit.
- R1-BND-REC2 mark 4: "48 soldiers" is now the largest mode's N soldiers (48, or 54 under MODES-3 (b)) —
  recommendation.
- R1-BND-OQ6: "copy time at N = 48" is now at the largest mode's N (48, or 54 under MODES-3 (b)) — recommendation (its
  measuring point changes under (b)).
- Pointer lines: R1-BND-REC2 gains R1-XT-OQ16 and R1-XT-OQ17; R1-DET-REC6 gains R1-XT-OQ2 — nit.
- R1-ECS-REC1's "Rests on" names R2's worked bandwidth estimate for mark 1's 15 big vehicles — nit.
- The hash-map ban named beside the iteration-only wording: the ECS options table (deterministic order), R1-ABL-REC3
  and the pointers between R1's units — nit.

## Sources

### Sources read (161)

Every source read by any unit, merged and de-duplicated by URL (170 reads across the six units; a page read by more than
one unit appears once, with every unit that read it). All were read on 2026-10-27. By tier, after the fix step's
relabelling (R1-XU-OQ6): official 138, shipped game 7, other 16.

| # | URL | Title | Version or page date | Tier | Read by unit | Read |
|---|---|---|---|---|---|---|
| 1 | https://docs.rs/bevy_ecs/latest/bevy_ecs/ | bevy_ecs crate docs | 0.20.0 | official | ecs | 2026-10-27 |
| 2 | https://docs.rs/crate/bevy_ecs/latest/features | bevy_ecs feature flags | 0.20.0 | official | ecs | 2026-10-27 |
| 3 | https://docs.rs/bevy_ecs/latest/bevy_ecs/system/struct.Query.html | Query | 0.20.0 | official | ecs | 2026-10-27 |
| 4 | https://docs.rs/bevy_ecs/latest/bevy_ecs/query/struct.QueryIter.html | QueryIter (sort methods) | 0.20.0 | official | ecs | 2026-10-27 |
| 5 | https://docs.rs/bevy_ecs/latest/bevy_ecs/schedule/index.html | schedule module | 0.20.0 | official | ecs | 2026-10-27 |
| 6 | https://docs.rs/bevy_ecs/latest/bevy_ecs/schedule/struct.ScheduleBuildSettings.html | ScheduleBuildSettings | 0.20.0 | official | ecs | 2026-10-27 |
| 7 | https://docs.rs/bevy_ecs/latest/bevy_ecs/schedule/struct.SingleThreadedExecutor.html | SingleThreadedExecutor | 0.20.0 | official | ecs | 2026-10-27 |
| 8 | https://docs.rs/bevy_ecs/latest/bevy_ecs/entity/struct.EntityCloner.html | EntityCloner | 0.20.0 | official | ecs | 2026-10-27 |
| 9 | https://docs.rs/bevy_ecs/latest/bevy_ecs/entity/struct.Entity.html | Entity | 0.20.0, page dated 2026-10-08 | official | ecs | 2026-10-27 |
| 10 | https://docs.rs/bevy_ecs/latest/bevy_ecs/world/struct.World.html | World (trait implementations) | 0.20.0 | official | ecs | 2026-10-27 |
| 11 | https://github.com/bevyengine/bevy-website/tree/main/content/news | Bevy news posts (folder list) | to 2026-10-08 | official | ecs | 2026-10-27 |
| 12 | https://raw.githubusercontent.com/bevyengine/bevy-website/main/content/news/2025-04-24-bevy-0.16/index.md | Bevy 0.16 release post | 2025-04-24 | official | ecs | 2026-10-27 |
| 13 | https://github.com/gschup/bevy_ggrs | bevy_ggrs README | bevy 0.19 / bevy_ggrs 0.22; 0.20 on main | other | ecs | 2026-10-27 |
| 14 | https://raw.githubusercontent.com/gschup/bevy_ggrs/main/docs/architecture.md | bevy_ggrs architecture | main, no date | other | ecs | 2026-10-27 |
| 15 | https://raw.githubusercontent.com/gschup/bevy_ggrs/main/docs/pitfalls.md | bevy_ggrs pitfalls | main, no date | other | ecs | 2026-10-27 |
| 16 | https://docs.rs/hecs/latest/hecs/ | hecs crate docs | 0.11.2 | official | ecs | 2026-10-27 |
| 17 | https://docs.rs/hecs/latest/hecs/struct.World.html | hecs World | 0.11.2 | official | ecs | 2026-10-27 |
| 18 | https://docs.rs/hecs/latest/hecs/serialize/index.html | hecs serialize | 0.11.2 | official | ecs | 2026-10-27 |
| 19 | https://docs.rs/hecs/latest/hecs/serialize/column/index.html | hecs serialize::column | 0.11.2 | official | ecs | 2026-10-27 |
| 20 | https://github.com/Ralith/hecs | hecs README | no version shown | official | ecs | 2026-10-27 |
| 21 | https://doc.rust-lang.org/std/collections/struct.HashMap.html | Rust std `HashMap` | std 1.99.1 (2026-10-13) | official | ecs, determinism, abilities | 2026-10-27 |
| 22 | https://doc.rust-lang.org/std/collections/struct.BTreeMap.html | Rust std `BTreeMap` | std 1.99.1 | official | ecs, determinism | 2026-10-27 |
| 23 | https://doc.rust-lang.org/std/primitive.slice.html | slice (iter) | std 1.99.1 | official | ecs | 2026-10-27 |
| 24 | https://doc.rust-lang.org/std/clone/trait.Clone.html | Clone | std 1.99.1 | official | ecs | 2026-10-27 |
| 25 | https://doc.rust-lang.org/std/vec/struct.Vec.html | Vec (clone_from) | std 1.99.1 | official | ecs | 2026-10-27 |
| 26 | https://docs.rs/slotmap/latest/slotmap/ | slotmap crate docs | 1.1.1 | official | ecs | 2026-10-27 |
| 27 | https://docs.rs/rapier3d/latest/rapier3d/dynamics/struct.RigidBodySet.html | RigidBodySet | rapier3d 0.36.1 | official | ecs | 2026-10-27 |
| 28 | https://rapier.rs/docs/user_guides/rust/serialization | Rapier serialization guide | Rust guide 0.36, no date | official | ecs | 2026-10-27 |
| 29 | https://github.com/dimforge/rapier/issues/910 | Non-determinism in BroadPhase when restoring from serialized snapshot | opened 2026-01-16, closed 2026-06-09 | other | ecs | 2026-10-27 |
| 30 | https://rustwasm.github.io/book/reference/code-size.html | Shrinking .wasm code size | "unpublished documentation", no date | official (archived: the book's repository was archived on 2025-08-25, verifier 2026-10-28) | ecs | 2026-10-27 |
| 31 | https://github.com/SanderMertens/ecs-faq | ECS FAQ (other tier; read, not cited) | no date | other | ecs | 2026-10-27 |
| 32 | https://gafferongames.com/post/fix_your_timestep/ | Glenn Fiedler, "Fix Your Timestep!" | page dated 2004-06-10 (read twice for quotes) | other | tick | 2026-10-27 |
| 33 | https://gafferongames.com/post/snapshot_interpolation/ | Glenn Fiedler, "Snapshot Interpolation" | 2014-11-30 | other | tick | 2026-10-27 |
| 34 | https://gafferongames.com/post/what_every_programmer_needs_to_know_about_game_networking/ | Glenn Fiedler, "What Every Programmer Needs To Know About Game Networking" | 2010-02-24 | other (the determinism unit had "shipped game, practitioner article"; settled at the fix step: R1-XU-OQ6) | tick, determinism | 2026-10-27 |
| 35 | https://gafferongames.com/post/integration_basics/ | Glenn Fiedler, "Integration Basics" | 2004-06-01 | other | tick | 2026-10-27 |
| 36 | https://raw.githubusercontent.com/id-Software/Quake-III-Arena/master/code/server/sv_init.c | Quake III Arena GPL source, `code/server/sv_init.c` | master, (C) 1999-2005 | shipped game | tick | 2026-10-27 |
| 37 | https://raw.githubusercontent.com/id-Software/Quake-III-Arena/master/code/server/sv_main.c | Quake III Arena GPL source, `code/server/sv_main.c` | master (same repository) | shipped game | tick | 2026-10-27 |
| 38 | https://raw.githubusercontent.com/id-Software/Quake-III-Arena/master/code/server/sv_client.c | Quake III Arena GPL source, `code/server/sv_client.c` | master (same repository) | shipped game | tick | 2026-10-27 |
| 39 | https://raw.githubusercontent.com/id-Software/Quake-III-Arena/master/code/game/g_main.c | Quake III Arena GPL source, `code/game/g_main.c` | master (same repository) | shipped game | tick | 2026-10-27 |
| 40 | https://raw.githubusercontent.com/id-Software/Quake-III-Arena/master/code/game/bg_pmove.c | Quake III Arena GPL source, `code/game/bg_pmove.c` | master; no page date | shipped game | tick, abilities | 2026-10-27 |
| 41 | https://raw.githubusercontent.com/ValveSoftware/source-sdk-2013/master/src/game/shared/shareddefs.h | Source SDK 2013, `src/game/shared/shareddefs.h` | master, no date | shipped game | tick | 2026-10-27 |
| 42 | https://raw.githubusercontent.com/ValveSoftware/source-sdk-2013/master/src/game/server/gameinterface.cpp | Source SDK 2013, `src/game/server/gameinterface.cpp` | master, no date (first 100,000 characters) | shipped game | tick | 2026-10-27 |
| 43 | https://developer.mozilla.org/en-US/docs/Games/Anatomy | MDN, "Anatomy of a video game" | last modified 2026-08-21 | official | tick | 2026-10-27 |
| 44 | https://developer.mozilla.org/en-US/docs/Web/API/Window/setTimeout | MDN, `setTimeout()` | last modified 2026-10-26 | official | tick | 2026-10-27 |
| 45 | https://developer.mozilla.org/en-US/docs/Web/API/Performance/now | MDN, `performance.now()` | last modified 2025-08-25 | official | tick, boundary-worker | 2026-10-27 |
| 46 | https://developer.mozilla.org/en-US/docs/Web/API/DedicatedWorkerGlobalScope/requestAnimationFrame | MDN, `DedicatedWorkerGlobalScope.requestAnimationFrame()` | last modified 2025-09-25 | official | tick | 2026-10-27 |
| 47 | https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Atomics/wait | MDN, `Atomics.wait()` | last modified 2026-08-31 | official | tick | 2026-10-27 |
| 48 | https://developer.chrome.com/blog/timer-throttling-in-chrome-88 | Chrome for Developers, "Heavy throttling of chained JS timers beginning in Chrome 88" | last updated 2021-01-18 | official | tick, boundary-worker | 2026-10-27 |
| 49 | https://docs.rs/bevy/latest/bevy/time/struct.Fixed.html | bevy `time::Fixed` | bevy 0.20.0 (first 100,000 characters) | official | tick | 2026-10-27 |
| 50 | https://docs.rs/bevy/latest/bevy/time/struct.Virtual.html | bevy `time::Virtual` | bevy 0.20.0 (first 100,000 characters) | official | tick | 2026-10-27 |
| 51 | https://docs.rs/rapier3d/latest/rapier3d/dynamics/struct.IntegrationParameters.html | rapier3d `IntegrationParameters` | rapier3d 0.36.1 | official | tick | 2026-10-27 |
| 52 | https://docs.rs/tokio/latest/tokio/time/enum.MissedTickBehavior.html | tokio `MissedTickBehavior` | tokio 1.54.1 | official | tick | 2026-10-27 |
| 53 | https://docs.rs/tokio/latest/tokio/time/fn.sleep.html | tokio `time::sleep` | tokio 1.54.1 | official | tick | 2026-10-27 |
| 54 | https://webassembly.github.io/spec/core/exec/numerics.html | WebAssembly 3.0 core spec, Numerics | header 2026-10-23 at the first read, 2026-10-27 at the second | official | determinism | 2026-10-27 |
| 55 | https://webassembly.github.io/spec/core/appendix/profiles.html | WebAssembly 3.0, Profiles | 2026-10-27 | official | determinism | 2026-10-27 |
| 56 | https://webassembly.org/features/ | WebAssembly feature status page | the table did not render; no data used | official | determinism | 2026-10-27 |
| 57 | https://raw.githubusercontent.com/WebAssembly/website/main/features.json | WebAssembly feature data (behind webassembly.org/features) | main branch, no date | official | determinism, boundary-worker | 2026-10-27 |
| 58 | https://doc.rust-lang.org/std/primitive.f32.html | Rust std `f32` | std 1.99.1, 2026-10-13 | official | determinism | 2026-10-27 |
| 59 | https://rust-lang.github.io/rfcs/3514-float-semantics.html | RFC 3514, float semantics | start date 2023-10-14 | official | determinism | 2026-10-27 |
| 60 | https://doc.rust-lang.org/rustc/platform-support/wasm32-unknown-unknown.html | rustc book, wasm32-unknown-unknown | no version shown; names Rust 1.87.0+ and "as of 2025-10-03" | official | determinism, boundary-worker | 2026-10-27 |
| 61 | https://doc.rust-lang.org/reference/expressions/operator-expr.html | The Rust Reference, operator expressions | no version shown | official | determinism | 2026-10-27 |
| 62 | https://docs.rs/libm/latest/libm/ | libm crate docs | 0.2.16, 2026-09-13 | official | determinism | 2026-10-27 |
| 63 | https://raw.githubusercontent.com/rust-lang/compiler-builtins/master/libm/README.md | libm README | master | official | determinism | 2026-10-27 |
| 64 | https://raw.githubusercontent.com/rust-lang/compiler-builtins/master/libm/Cargo.toml | libm 0.2.16 manifest | master | official | determinism | 2026-10-27 |
| 65 | https://raw.githubusercontent.com/rust-lang/compiler-builtins/master/libm/src/math/arch/mod.rs | libm arch module | master | official | determinism | 2026-10-27 |
| 66 | https://github.com/rust-lang/compiler-builtins/tree/master/libm/src/math/arch | libm `src/math/arch` directory listing | master | official | determinism | 2026-10-27 |
| 67 | https://raw.githubusercontent.com/rust-lang/compiler-builtins/master/README.md | compiler-builtins repository README | master; nothing on math symbols | official | determinism | 2026-10-27 |
| 68 | https://raw.githubusercontent.com/rust-lang/compiler-builtins/master/compiler-builtins/README.md | compiler-builtins crate README | master; nothing on math symbols | official | determinism | 2026-10-27 |
| 69 | https://raw.githubusercontent.com/rust-lang/compiler-builtins/master/compiler-builtins/src/math/mod.rs | compiler-builtins math module | master | official | determinism | 2026-10-27 |
| 70 | https://rapier.rs/docs/user_guides/rust/determinism | Rapier user guide, Determinism | docs 0.36, no page date | official | determinism | 2026-10-27 |
| 71 | https://raw.githubusercontent.com/dimforge/rapier/master/crates/rapier3d/Cargo.toml | rapier3d manifest | master | official | determinism | 2026-10-27 |
| 72 | https://raw.githubusercontent.com/dimforge/rapier/master/Cargo.toml | Rapier workspace manifest | 0.36.1, master | official | determinism | 2026-10-27 |
| 73 | https://raw.githubusercontent.com/dimforge/parry/master/crates/parry3d/Cargo.toml | parry3d manifest | master | official | determinism | 2026-10-27 |
| 74 | https://raw.githubusercontent.com/dimforge/rapier.js/master/README.md | rapier.js README | says only that the repository merged into dimforge/rapier | official | determinism | 2026-10-27 |
| 75 | https://docs.rs/rand/latest/rand/ | rand crate docs | 0.10.3, 2026-09-20 | official | determinism | 2026-10-27 |
| 76 | https://docs.rs/rand/latest/rand/rngs/struct.StdRng.html | rand `StdRng` | 0.10.3 | official | determinism | 2026-10-27 |
| 77 | https://docs.rs/rand_chacha/latest/rand_chacha/ | rand_chacha crate docs | 0.10.0, 2026-09-13 | official | determinism | 2026-10-27 |
| 78 | https://docs.rs/rand_chacha/latest/rand_chacha/struct.ChaCha8Rng.html | rand_chacha `ChaCha8Rng` | 0.10.0 | official | determinism | 2026-10-27 |
| 79 | https://docs.rs/rand_pcg/latest/rand_pcg/ | rand_pcg crate docs | 0.10.2, 2026-09-22 | official | determinism | 2026-10-27 |
| 80 | https://docs.rs/fixed/latest/fixed/ | fixed crate docs | 1.31.0, 2026-09-20 | official | determinism | 2026-10-27 |
| 81 | https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Math | MDN, `Math` | last modified 2025-07-10 | official | determinism | 2026-10-27 |
| 82 | https://gafferongames.com/post/floating_point_determinism/ | Glenn Fiedler, "Floating Point Determinism" | 2010-02-24 | other (quotes shipped-game developers; the unit had "shipped game"; relabelled at the fix step: R1-XU-OQ6) | determinism | 2026-10-27 |
| 83 | https://blog.rust-lang.org/inside-rust/2025/07/21/sunsetting-the-rustwasm-github-org/ | "Sunsetting the rustwasm GitHub org" | 2025-07-21 | official | boundary-worker | 2026-10-27 |
| 84 | https://github.com/wasm-bindgen/wasm-bindgen/releases | wasm-bindgen releases | 0.2.118 to 0.2.129 (2026) | official | boundary-worker | 2026-10-27 |
| 85 | https://github.com/wasm-bindgen/wasm-bindgen/releases/tag/0.2.128 | release 0.2.128 | "05 Sep" (2026) | official | boundary-worker | 2026-10-27 |
| 86 | https://rustwasm.github.io/docs/wasm-bindgen/examples/raytrace.html | old wasm-bindgen guide page (raytrace) | marked "no longer maintained at this domain"; read only for that notice | official | boundary-worker | 2026-10-27 |
| 87 | https://raw.githubusercontent.com/wasm-bindgen/wasm-bindgen/main/guide/src/examples/raytrace.md | Parallel Raytracing | main, undated | official | boundary-worker | 2026-10-27 |
| 88 | https://raw.githubusercontent.com/wasm-bindgen/wasm-bindgen/main/guide/src/SUMMARY.md | guide contents | main | official | boundary-worker | 2026-10-27 |
| 89 | https://raw.githubusercontent.com/wasm-bindgen/wasm-bindgen/main/guide/src/reference/types/number-slices.md | Number Slices | main | official | boundary-worker | 2026-10-27 |
| 90 | https://raw.githubusercontent.com/wasm-bindgen/wasm-bindgen/main/guide/src/reference/arbitrary-data-with-serde.md | Arbitrary Data with Serde | main | official | boundary-worker | 2026-10-27 |
| 91 | https://raw.githubusercontent.com/wasm-bindgen/wasm-bindgen/main/guide/src/examples/wasm-in-web-worker.md | Wasm in Web Worker | main | official | boundary-worker | 2026-10-27 |
| 92 | https://raw.githubusercontent.com/wasm-bindgen/wasm-bindgen/main/guide/src/reference/browser-support.md | Supported Browsers | main | official | boundary-worker | 2026-10-27 |
| 93 | https://raw.githubusercontent.com/wasm-bindgen/wasm-bindgen/main/guide/src/reference/cli.md | Command Line Interface | main | official | boundary-worker | 2026-10-27 |
| 94 | https://raw.githubusercontent.com/wasm-bindgen/wasm-bindgen/main/guide/src/reference/attributes/on-rust-exports/typescript_custom_section.md | typescript_custom_section | main | official | boundary-worker | 2026-10-27 |
| 95 | https://doc.rust-lang.org/reference/type-layout.html | Rust Reference, Type layout | current | official | boundary-worker | 2026-10-27 |
| 96 | https://rustwasm.github.io/docs/book/game-of-life/implementing.html | Rust and WebAssembly book, Implementing Life | archived ("no longer maintained") | official | boundary-worker | 2026-10-27 |
| 97 | https://docs.rs/tsify/latest/tsify/ | tsify crate docs | 0.5.8 | official | boundary-worker | 2026-10-27 |
| 98 | https://docs.rs/tsify-next/latest/tsify_next/ | tsify-next crate docs | 0.5.6, 2026-10-05 | official | boundary-worker | 2026-10-27 |
| 99 | https://docs.rs/ts-rs/latest/ts_rs/ | ts-rs crate docs | 12.0.1 | official | boundary-worker | 2026-10-27 |
| 100 | https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/SharedArrayBuffer | MDN, `SharedArrayBuffer` | last modified 2026-02-10 | official | boundary-worker | 2026-10-27 |
| 101 | https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Atomics | MDN, `Atomics` | last modified 2026-02-10 | official | boundary-worker | 2026-10-27 |
| 102 | https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Atomics/waitAsync | MDN, `Atomics.waitAsync()` | last modified 2026-02-10 | official | boundary-worker | 2026-10-27 |
| 103 | https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/TypedArray | MDN, `TypedArray` | last modified 2026-10-26 | official | boundary-worker | 2026-10-27 |
| 104 | https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Float32Array/Float32Array | MDN, `Float32Array()` constructor | last modified 2026-10-26 (defers to TypedArray) | official | boundary-worker | 2026-10-27 |
| 105 | https://developer.mozilla.org/en-US/docs/WebAssembly/Reference/JavaScript_interface/Memory/grow | MDN, `WebAssembly.Memory.prototype.grow()` | last modified 2026-05-14 | official | boundary-worker | 2026-10-27 |
| 106 | https://developer.mozilla.org/en-US/docs/Web/API/Web_Workers_API/Transferable_objects | MDN, Transferable objects | last modified 2025-09-18 | official | boundary-worker | 2026-10-27 |
| 107 | https://developer.mozilla.org/en-US/docs/Web/HTTP/Reference/Headers/Cross-Origin-Embedder-Policy | MDN, `Cross-Origin-Embedder-Policy` | last modified 2026-09-04 | official | boundary-worker | 2026-10-27 |
| 108 | https://developer.mozilla.org/en-US/docs/Web/HTTP/Reference/Headers/Cross-Origin-Opener-Policy | MDN, `Cross-Origin-Opener-Policy` | last modified 2026-07-12 | official | boundary-worker | 2026-10-27 |
| 109 | https://developer.mozilla.org/en-US/docs/Web/API/Window/crossOriginIsolated | MDN, `Window.crossOriginIsolated` | last modified 2026-08-21 | official | boundary-worker | 2026-10-27 |
| 110 | https://developer.mozilla.org/en-US/docs/Web/API/Navigator/hardwareConcurrency | MDN, `Navigator.hardwareConcurrency` | last modified 2024-10-16 | official | boundary-worker | 2026-10-27 |
| 111 | https://developer.mozilla.org/en-US/docs/Web/API/WebTransport | MDN, `WebTransport` | last modified 2026-10-12 | official | boundary-worker | 2026-10-27 |
| 112 | https://developer.mozilla.org/en-US/docs/Web/API/WebSocket | MDN, `WebSocket` | last modified 2024-09-25 | official | boundary-worker | 2026-10-27 |
| 113 | https://raw.githubusercontent.com/mdn/browser-compat-data/main/http/headers/Cross-Origin-Embedder-Policy.json | MDN browser-compat-data, `Cross-Origin-Embedder-Policy.json` | main | official | boundary-worker | 2026-10-27 |
| 114 | https://raw.githubusercontent.com/mdn/browser-compat-data/main/javascript/builtins/Atomics.json | MDN browser-compat-data, `Atomics.json` | main | official | boundary-worker | 2026-10-27 |
| 115 | https://raw.githubusercontent.com/mdn/browser-compat-data/main/javascript/builtins/SharedArrayBuffer.json | MDN browser-compat-data, `SharedArrayBuffer.json` | main | official | boundary-worker | 2026-10-27 |
| 116 | https://raw.githubusercontent.com/mdn/browser-compat-data/main/api/Worker.json | MDN browser-compat-data, `Worker.json` | main | official | boundary-worker | 2026-10-27 |
| 117 | https://raw.githubusercontent.com/WebAssembly/threads/main/proposals/threads/Overview.md | Wasm threads proposal overview | main | official | boundary-worker | 2026-10-27 |
| 118 | https://web.dev/articles/coop-coep | "Making your website cross-origin isolated using COOP and COEP" | last updated 2022-06-21 | official | boundary-worker | 2026-10-27 |
| 119 | https://vite.dev/guide/features | Vite features | docs v8.3.4 | official | boundary-worker | 2026-10-27 |
| 120 | https://vite.dev/config/server-options | Vite server options | docs v8.3.4 | official | boundary-worker | 2026-10-27 |
| 121 | https://github.com/tranek/GASDocumentation | tranek, GASDocumentation (repository page) | no date shown | other | abilities | 2026-10-27 |
| 122 | https://raw.githubusercontent.com/tranek/GASDocumentation/master/README.md | GASDocumentation README (community; tier other) | says current with UE 5.3; no page date; read in windows 0–331,640 (the last 6,640 characters, around the changelog, not read) | other | abilities | 2026-10-27 |
| 123 | https://docs.rs/leafwing_abilities/latest/leafwing_abilities/ | leafwing_abilities crate docs | 0.13.0, released 2026-09-27 per page | other | abilities | 2026-10-27 |
| 124 | https://docs.rs/leafwing_abilities/latest/leafwing_abilities/cooldown/struct.Cooldown.html | leafwing_abilities `Cooldown` | 0.13.0 (first 100,000 characters read) | other | abilities | 2026-10-27 |
| 125 | https://docs.rs/crate/bevy_gauge/latest | bevy_gauge crate page | 0.5.1, released 2026-07-06 per page | other | abilities | 2026-10-27 |
| 126 | https://docs.rs/crate/bevy_alchemy/latest/source/README.md | bevy_alchemy README | 0.6.0; no date | other | abilities | 2026-10-27 |
| 127 | https://raw.githubusercontent.com/serde-rs/serde-rs.github.io/master/_src/container-attrs.md | serde.rs "Container attributes" source | serde 1.x, no page date | official | data-hotreload | 2026-10-27 |
| 128 | https://raw.githubusercontent.com/serde-rs/serde-rs.github.io/master/_src/field-attrs.md | serde.rs "Field attributes" source | no page date | official | data-hotreload | 2026-10-27 |
| 129 | https://docs.rs/serde_json/latest/serde_json/ | serde_json crate docs | 1.0.154, page 2026-10-11 | official | data-hotreload | 2026-10-27 |
| 130 | https://docs.rs/crate/serde_json/latest/features | serde_json feature list | 1.0.154 (no descriptions; used only for the default feature) | official | data-hotreload | 2026-10-27 |
| 131 | https://raw.githubusercontent.com/serde-rs/json/master/Cargo.toml | serde_json Cargo.toml (feature comments) | 1.0.154 | official | data-hotreload | 2026-10-27 |
| 132 | https://docs.rs/serde_json/latest/serde_json/struct.Deserializer.html | serde_json `Deserializer` | 1.0.154 | official | data-hotreload | 2026-10-27 |
| 133 | https://docs.rs/serde_json/latest/serde_json/struct.Error.html | serde_json `Error` | 1.0.154 | official | data-hotreload | 2026-10-27 |
| 134 | https://docs.rs/serde_json/latest/serde_json/value/index.html | serde_json `value` module | 1.0.154 | official | data-hotreload | 2026-10-27 |
| 135 | https://docs.rs/serde_path_to_error/latest/serde_path_to_error/ | serde_path_to_error crate docs | 0.1.20 | official | data-hotreload | 2026-10-27 |
| 136 | https://github.com/ron-rs/ron | RON README | no release listed | official | data-hotreload | 2026-10-27 |
| 137 | https://docs.rs/ron/latest/ron/ | ron crate docs | 0.12.2, page 2026-09-13 | official | data-hotreload | 2026-10-27 |
| 138 | https://docs.rs/toml/latest/toml/ | toml crate docs | 1.1.8+spec-1.1.0, page 2026-10-09 | official | data-hotreload | 2026-10-27 |
| 139 | https://raw.githubusercontent.com/toml-lang/toml/main/toml.md | TOML spec text | main branch, no version shown | official | data-hotreload | 2026-10-27 |
| 140 | https://github.com/dtolnay/serde-yaml | serde-yaml repository | archived 2024-03-25; serde_yaml 0.9 | official | data-hotreload | 2026-10-27 |
| 141 | https://datatracker.ietf.org/doc/html/draft-bhutton-json-schema-validation-01 | JSON Schema Validation (2020-12) | I-D published 2022-06-10 | official | data-hotreload | 2026-10-27 |
| 142 | https://docs.rs/schemars/latest/schemars/ | schemars crate docs | 1.2.3, page 2026-10-12 | official | data-hotreload | 2026-10-27 |
| 143 | https://github.com/GREsau/schemars | schemars README | master | official | data-hotreload | 2026-10-27 |
| 144 | https://github.com/GREsau/schemars/tree/master/docs | schemars docs folder listing | master | official | data-hotreload | 2026-10-27 |
| 145 | https://raw.githubusercontent.com/GREsau/schemars/master/docs/1.1-attributes.md | schemars `docs/1.1-attributes.md` (a Jekyll include stub only) | master | official | data-hotreload | 2026-10-27 |
| 146 | https://raw.githubusercontent.com/GREsau/schemars/master/docs/_includes/attributes.md | schemars attribute docs | master | official | data-hotreload | 2026-10-27 |
| 147 | https://docs.rs/garde/latest/garde/ | garde crate docs | 0.23.0, page 2026-10-03 | official | data-hotreload | 2026-10-27 |
| 148 | https://github.com/ajv-validator/ajv | Ajv README | version 8; no release date shown | official | data-hotreload | 2026-10-27 |
| 149 | https://docs.rs/notify/latest/notify/ | notify crate docs | 8.2.1, page 2026-10-25 | official | data-hotreload | 2026-10-27 |
| 150 | https://docs.rs/bevy_asset/latest/bevy_asset/ | bevy_asset crate docs | 0.20.0, 2026-10-08 | official | data-hotreload | 2026-10-27 |
| 151 | https://vite.dev/guide/api-hmr | Vite HMR API | v8.3.4 shown | official | data-hotreload | 2026-10-27 |
| 152 | https://vite.dev/guide/api-plugin | Vite Plugin API | "Client-server Communication", v8.3.4 shown | official | data-hotreload | 2026-10-27 |
| 153 | https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Object/proto | MDN, `Object.prototype.__proto__` | no date recorded | official | data-hotreload | 2026-10-27 |
| 154 | https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/JSON/parse | MDN, `JSON.parse()` | no date recorded | official | data-hotreload | 2026-10-27 |
| 155 | https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Operators/Object_initializer | MDN, Object initializer ("Object literal syntax vs. JSON") | no date recorded | official | data-hotreload | 2026-10-27 |
| 156 | https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Object/assign | MDN, `Object.assign()` | no date recorded | official | data-hotreload | 2026-10-27 |
| 157 | https://cheatsheetseries.owasp.org/cheatsheets/Prototype_Pollution_Prevention_Cheat_Sheet.html | OWASP Prototype Pollution Prevention Cheat Sheet | no page date | official | data-hotreload | 2026-10-27 |
| 158 | https://cheatsheetseries.owasp.org/cheatsheets/Input_Validation_Cheat_Sheet.html | OWASP Input Validation Cheat Sheet | no page date | official | data-hotreload | 2026-10-27 |
| 159 | https://developer.mozilla.org/en-US/docs/Web/HTTP/Reference/Headers/Cache-Control | MDN, `Cache-Control` | no date recorded | official | data-hotreload | 2026-10-27 |
| 160 | https://doc.rust-lang.org/std/macro.include_bytes.html | Rust std `include_bytes!` | 1.99.1 (2026-10-13) | official | data-hotreload | 2026-10-27 |
| 161 | https://developer.mozilla.org/en-US/docs/Web/API/SubtleCrypto/digest | MDN, `SubtleCrypto.digest()` | no date recorded | official | data-hotreload | 2026-10-27 |

Per unit, as each unit labelled its sources (the not-read count is the unit's own list):

| Unit | Read | Official | Shipped game | Other | Not read |
|---|---|---|---|---|---|
| ecs | 31 | 26 | 0 | 5 | 10 |
| tick | 22 | 11 | 7 | 4 | 12 |
| determinism | 32 | 30 | 2 | 0 | 11 |
| boundary-worker | 42 | 42 | 0 | 0 | 6 |
| abilities | 8 | 1 | 1 | 6 | 9 |
| data-hotreload | 35 | 35 | 0 | 0 | 5 |

After the fix step's relabelling (R1-XU-OQ6), the determinism unit's two Fiedler articles count as other (its row
would read 30 / 0 / 2).

### Sources read at the fix step (2026-10-28)
Read by engine-architect to re-source or correct the claims the reviewers found wrong, unsupported or incomplete. Three
pages are new to this file; the rest are re-reads of pages listed above.

| # | URL | Title | Version or page date | Tier | Used for | Read |
|---|---|---|---|---|---|---|
| F1 | https://doc.rust-lang.org/std/collections/struct.HashMap.html | Rust std `HashMap` (re-read; #21 above) | std 1.99.1 (2026-10-13) | official | R1-DET-F33 | 2026-10-28 |
| F2 | https://developer.mozilla.org/en-US/docs/Web/API/Web_Workers_API/Structured_clone_algorithm | MDN, The structured clone algorithm (new) | last modified 2025-06-29 | official | R1-BND-F34 | 2026-10-28 |
| F3 | https://webassembly.github.io/spec/js-api/ | WebAssembly JavaScript Interface, §5.3 Memories (new) | Editor's Draft, 27 October 2026 (dated 28 October 2026 when the second verifier pass read it; the step unchanged) | official | R1-BND-F35 | 2026-10-28 |
| F4 | https://developer.mozilla.org/en-US/docs/Web/HTTP/Reference/Headers/Cross-Origin-Opener-Policy | MDN, `Cross-Origin-Opener-Policy` (re-read; #108) | last modified 2026-07-12 | official | R1-BND-F23, R1-BND-OQ9 | 2026-10-28 |
| F5 | https://doc.rust-lang.org/std/clone/trait.Clone.html | Rust std `Clone` (re-read; #24) | std 1.99.1 | official | R1-ECS-F35 | 2026-10-28 |
| F6 | https://docs.rs/rapier3d/latest/rapier3d/dynamics/struct.RigidBodySet.html | rapier3d `RigidBodySet`, Trait Implementations (re-read; #27) | rapier3d 0.36.1 | official | R1-ECS-F35 | 2026-10-28 |
| F7 | https://docs.rs/rand_pcg/latest/rand_pcg/ | rand_pcg crate docs (re-read; #79) | 0.10.2 | official | R1-DET-F34 | 2026-10-28 |
| F8 | https://docs.rs/rand_pcg/latest/rand_pcg/struct.Lcg64Xsh32.html | rand_pcg `Lcg64Xsh32` (new) | 0.10.2 | official | R1-DET-F34 | 2026-10-28 |
| F9 | https://raw.githubusercontent.com/WebAssembly/threads/main/proposals/threads/Overview.md | Wasm threads proposal overview (re-read; #117) | main | official | R1-BND-F17 (quote corrected) | 2026-10-28 |
| F10 | https://raw.githubusercontent.com/tranek/GASDocumentation/master/README.md | GASDocumentation README (re-read in the verifier's downloaded copy; #122; the second verifier pass matched the quotes on the page itself) | UE 5.3 | other | R1-ABL-F4, F5, F15, F17, F18 (quotes checked) | 2026-10-28 |

Pages the reviewers read that this file now cites (read by the verifier on 2026-10-28, not by the fix step):
https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Math/random (R1-DET-F30) and
https://github.com/rustwasm/book (the archive notice, R1-ECS-F33); and, read by the second verifier pass on the same
day, MDN's Secure contexts page, https://developer.mozilla.org/en-US/docs/Web/Security/Secure_Contexts (last modified
2026-09-14; R1-DATA-REC6's correction and R1-DATA-OQ11; the URL from the final check). rust-reviewer's own sources (RustSec, wide, simba,
glam, Cargo profiles) are listed in its notes and feed only the open questions R1-RR-OQ1–OQ7.

### Sources not read (48)

Every source a unit tried or found and did not read, merged by URL (53 entries across the six units).
Some rows group more than one URL, as the unit listed them.

| # | Source | Reason (each unit's words) | Unit |
|---|---|---|---|
| 1 | https://bevyengine.org/news/bevy-0-16/ | Redirects (301) to bevy.org, outside the allow list; read from the site's GitHub repository instead | ecs |
| 2 | https://gdcvault.com/play/1024001/-Overwatch-Gameplay-Architecture-and (and the `www.` host) | ecs, `www.` host: Redirects (302) to gdconf.com (a maintenance page), outside the allow list; the talk is video (likely video only); ecs: Same redirect, outside the allow list; tick: redirected to gdconf.com/gdc-vault-maintenance/, outside the allow list; the talk is video only; abilities: not tried: same host in maintenance; video only | ecs, tick, abilities |
| 3 | https://developer.valvesoftware.com/wiki/Networking_Entities | HTTP 403 Forbidden (on the allow list, but refused) | ecs |
| 4 | https://edgegap.com/blog/game-backend-deep-dive-overwatch-2016-netcode-architecture-rollback | ecs: Search result; outside the allow list; tick: outside the allow list (search lead) | ecs, tick |
| 5 | https://www.sohu.com/a/148848770_466876 | Search result (Overwatch talk write-up); outside the allow list | ecs |
| 6 | https://juejin.cn/post/6844903635864780813 | Search result; outside the allow list | ecs |
| 7 | https://www.gcores.com/articles/190857 | Search result; outside the allow list | ecs |
| 8 | https://www.gamedev.net/forums/topic/696756-command-frames-and-tick-synchronization/5386794/?page=3 | Search result; outside the allow list | ecs |
| 9 | https://godot.rapier.rs/docs/reference/state-manager | Search result; godot.rapier.rs is not listed (only rapier.rs), and it is not needed for a Rust core | ecs |
| 10 | https://developer.valvesoftware.com/wiki/Source_Multiplayer_Networking | tick: HTTP 403 from the site (not retried); determinism: the server returned 403 Forbidden | tick, determinism |
| 11 | https://developer.valvesoftware.com/wiki/Latency_Compensating_Methods_in_Client/Server_In-game_Protocol_Design_and_Optimization | HTTP 403 from the site (not retried) | tick |
| 12 | https://gdconf.com/gdc-vault-maintenance/ | outside the allow list | tick |
| 13 | https://raw.githubusercontent.com/ValveSoftware/source-sdk-2013/master/mp/src/game/shared/shareddefs.h | HTTP 404 (the repository was restructured) | tick |
| 14 | https://www.pcgamesn.com/overwatch/overwatch-netcode | outside the allow list (search lead) | tick |
| 15 | https://gamesbeat.com/overwatch-netcode-tick-rate-60hz/ | outside the allow list (search lead) | tick |
| 16 | https://gamedev.net/forums/topic/696756-command-frames-and-tick-synchronization/5378869/ | outside the allow list (search lead) | tick |
| 17 | https://en.wikipedia.org/wiki/Source_(game_engine) | outside the allow list (search lead) | tick |
| 18 | https://wiki.facepunch.com/gmod/engine.TickInterval | outside the allow list (search lead) | tick |
| 19 | https://html.spec.whatwg.org/multipage/timers-and-user-prompts.html | outside the allow list (the timer rules MDN cites) | tick |
| 20 | https://crates.io/crates/rapier3d | the page rendered no content (script-built page); versions taken from the repository manifests instead | determinism |
| 21 | https://docs.rs/crate/rapier3d/latest/features | the fetch was stopped by the organisation's spend limit and not retried (the manifests in sources 20–22 cover it) [the determinism unit's list: the rapier3d, Rapier workspace and parry3d manifests] | determinism |
| 22 | https://raw.githubusercontent.com/dimforge/rapier/master/typescript/README.md and https://github.com/dimforge/rapier/tree/master/typescript | 404 Not Found | determinism |
| 23 | https://cdn.jsdelivr.net/npm/@dimforge/rapier3d-deterministic@0.19.3/README.md | outside the allow list | determinism |
| 24 | https://npmjs.com/package/@dimforge/rapier3d | outside the allow list | determinism |
| 25 | https://unpkg.com/@dimforge/rapier3d@0.17.3/README.md (and the other unpkg.com results) | outside the allow list | determinism |
| 26 | https://godotengine.org/asset-library/asset/edit/19573 (and the other godotengine.org results) | outside the allow list | determinism |
| 27 | https://dev.to/jerzakm/this-little-known-javascript-physics-library-blew-my-mind-57oo | outside the allow list | determinism |
| 28 | rand's book, "Reproducibility" chapter (rust-random.github.io, linked from rand_chacha and rand_pcg) | outside the allow list | determinism |
| 29 | The `cordic` crate (named by `fixed` for trig) | not read: fixed point is not recommended, so it did not matter | determinism |
| 30 | https://wasm-bindgen.github.io/wasm-bindgen/ (the guide's new home) | outside the allow list (the same guide was read from its source on raw.githubusercontent.com) | boundary-worker |
| 31 | https://gh.nn.ci/emscripten-core/emscripten/discussions/24472 (search lead) | outside the allow list | boundary-worker |
| 32 | https://wiki.mozilla.org/Platform/Games/StatusUpdate (search lead) | outside the allow list | boundary-worker |
| 33 | https://hacks.mozilla.org/2017/07/the-next-generation-of-web-gaming/ (allowed) | not read: 2017 background, superseded by the compatibility data | boundary-worker |
| 34 | https://www.w3.org/2019/09/03-me-irc (allowed) | not read: a meeting log found by search, other tier | boundary-worker |
| 35 | https://github.com/wasm-bindgen/wasm-bindgen/issues/4533 (allowed) | not read in this run; see OQ7 | boundary-worker |
| 36 | https://dev.epicgames.com/documentation (Unreal Engine Gameplay Ability System pages) | outside the allow list | abilities |
| 37 | https://github.com/EpicGames/UnrealEngine/blob/release/Engine/Plugins/Runtime/GameplayAbilities/Source/GameplayAbilities/Public/GameplayPrediction.h | HTTP 404 (private repository: needs an Epic-linked GitHub account) | abilities |
| 38 | https://developer.valvesoftware.com/wiki/Dota_2_Workshop_Tools/Scripting/Abilities_Data_Driven | HTTP 403 | abilities |
| 39 | https://developer.valvesoftware.com/wiki/Prediction | HTTP 403 | abilities |
| 40 | https://gdcvault.com/play/1024041/Networking-Scripted-Weapons-and-Abilities (and the www. form) | redirected to gdconf.com/gdc-vault-maintenance/, outside the allow list; the talk is video only | abilities |
| 41 | https://ikrima.dev/ue4guide/gameplay-programming/gameplay-ability-system/epic-technical-brief/ | outside the allow list (search lead only) | abilities |
| 42 | https://www.gamedeveloper.com/design/attend-gdc-and-see-how-blizzard-scripts-i-overwatch-i-s-weapons-and-powers | outside the allow list (search lead only) | abilities |
| 43 | https://crates-io.ganymede-static.progval.net/crates/leafwing_abilities/0.1.1 and https://rust-digger.code-maven.com/crates/bevy_gauge | outside the allow list (search leads only; docs.rs read instead) | abilities |
| 44 | https://developer.valvesoftware.com/wiki/Developer_Console_Control | HTTP 403 Forbidden (allow-listed domain) | data-hotreload |
| 45 | https://developer.valvesoftware.com/wiki/ConVar | HTTP 403 Forbidden (allow-listed domain) | data-hotreload |
| 46 | https://bevy.org/examples/asset/hot-asset-reloading/ | outside the allow list (bevyengine.org redirects there) | data-hotreload |
| 47 | https://gdcvault.com/play/1024050/Modify-Everything-Data-Driven-Dynamic (GDC 2017, For Honor's data-driven modifiers) | a search lead only; not read: video only (the search result reports a login wall) | data-hotreload |
| 48 | The "ron-js" JavaScript implementation listed in the RON README | not fetched; its URL was not shown | data-hotreload |

The boundary-worker unit adds: "Shipped-game talks or papers on a browser game's sim in a worker: none found by search (no source to list)."
