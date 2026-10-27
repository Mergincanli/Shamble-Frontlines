# R1 unit ecs: ECS or plain data (bevy_ecs, hecs, custom)

Brief question (quoted): "ECS or plain data (bevy_ecs, hecs, custom)" (docs/V2_PHASE0_BRIEF.md §4, row R1).

Author: engine-architect (R1 unit "ecs", wave 1). Date: 2026-10-27. Status: draft, not yet verified. Scope: how the
Rust simulation core holds its state (native on the server, Wasm in a Web Worker in the browser): entity kinds and
counts, deterministic iteration order, whole-state snapshot and restore, serialization for snapshots, Wasm size and
compile time, and how each option would hold the data-driven ability system and physics. Notes:
`.checkpoints/v2-research/engine-architect-r1-ecs.md` (every source summary, with the verbatim quotes).

Sureness labels: "confirmed in the source" = the quoted sentence was returned verbatim from the page; "inferred" = my
reasoning from the source, or a point seen only in a fetch summary; "unverified" = not checked.

## Findings

### bevy_ecs (0.20.0)
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
  (2025-04-24). Tier: official. Read 2026-10-27. Sure: inferred (the fetch returned only summary-level text; the
  relationships section was not read in detail).

### hecs (0.11.2)
- **R1-ECS-F16.** hecs is a library with no scheduler: gameplay code queries the world directly. Quotes: "It is a
  library, not a framework."; "In place of an explicit "System" abstraction, a World's entities are easily queried
  from regular code." The crate's stated goals, in order: fast traversals, a simple interface, a small dependency
  closure, and leaving out what users can build themselves (paraphrase). Source: https://docs.rs/hecs/latest/hecs/
  (crate docs), 0.11.2. Tier: official (the crate's own docs). Read 2026-10-27. Sure: confirmed in the source (the
  quotes); the order of the goals is inferred.
- **R1-ECS-F17.** A hecs world is archetypal and yields entities in arbitrary order. Quotes: "An unordered collection
  of entities, each having any number of distinctly typed components"; "The components of entities who have the same
  set of component types are stored in contiguous runs"; query: "Entities are yielded in arbitrary order."; "When old
  entities are despawned, their IDs will be reused on a future entity". Source:
  https://docs.rs/hecs/latest/hecs/struct.World.html, 0.11.2. Tier: official. Read 2026-10-27. Sure: confirmed in the
  source.
- **R1-ECS-F18.** A hecs `World` does not implement `Clone`. Its trait list has Default, Extend, FromIterator,
  IntoIterator, Send and Sync. Source: https://docs.rs/hecs/latest/hecs/struct.World.html (Trait Implementations),
  0.11.2. Tier: official. Read 2026-10-27. Sure: confirmed in the source (the impl list).
- **R1-ECS-F19.** hecs can make its entity ids come out the same in two worlds, if both hold the same live entities
  and the same freelist. Quote: freelist: "Entity handles will be allocated deterministically between different
  worlds (e.g. across serialization)". set_freelist may be called only when the live ids match exactly (paraphrase).
  Source: https://docs.rs/hecs/latest/hecs/struct.World.html (freelist, set_freelist), 0.11.2. Tier: official. Read
  2026-10-27. Sure: confirmed in the source (the quote); the condition is inferred.
- **R1-ECS-F20.** hecs can't serialize a world on its own: the user writes a context that serializes each component
  type, and its compact format is column-major. Quotes: column format "Efficient, compact, and highly compressible,
  but difficult to read or edit by hand"; "Preferred for data that will be read/written programmatically"; "For some
  applications, a custom approach may be preferable." Sources: https://docs.rs/hecs/latest/hecs/serialize/index.html
  and https://docs.rs/hecs/latest/hecs/serialize/column/index.html, 0.11.2. Tier: official. Read 2026-10-27. Sure:
  confirmed in the source (the quotes); that the World can't implement Serialize because components aren't always
  serializable is paraphrase (inferred).
- **R1-ECS-F21.** The hecs authors advise plain `Vec`s per entity type for games with few entity types. Quotes: "If
  your game will have few types of entities, consider a simpler architecture such as storing each type of entity in a
  separate plain Vec."; "ECS may be overkill for games that don't call for batch processing of entities." Source:
  https://github.com/Ralith/hecs (README, "Why Not ECS?"; no version shown). Tier: official (the crate's own
  repository). Read 2026-10-27. Sure: confirmed in the source.
- **R1-ECS-F22.** Even with an ECS, much of a game's state lives in other structures, such as a spatial index. Quotes:
  "Most games will store significant amounts of state in other structures."; "many games maintain a spatial index
  structure (e.g. a tile map or bounding volume hierarchy)". Source: https://github.com/Ralith/hecs (README, "Why Not
  ECS?"). Tier: official. Read 2026-10-27. Sure: confirmed in the source.

### Plain data in Rust (std 1.99.1, slotmap 1.1.1)
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
- **R1-ECS-F28.** Versioned (generational) keys keep a removed key invalid even when its slot is reused. Quote: "once
  a key is removed, it stays removed, even if the physical storage inside the slotmap is reused for new elements."
  The dense variant iterates "as fast as a normal Vec", at the cost of two indirections per random access
  (paraphrase). The crate supports no_std: "This crate also supports no_std environments, but does require the alloc
  crate to be available." Source: https://docs.rs/slotmap/latest/slotmap/, slotmap 1.1.1. Tier: official (the crate's
  own docs). Read 2026-10-27. Sure: confirmed in the source (the quotes); the indirection cost and serde support are
  inferred.

### Physics state (Rapier 0.36)
- **R1-ECS-F29.** Rapier keeps bodies in its own arena-backed set, which implements Clone, Serialize and Deserialize
  and is addressed by index-plus-generation handles. Quote: "The collection that stores all rigid bodies in your
  physics world."; handle: "This handle is lightweight (just an index + generation) and remains valid even if other
  bodies are removed." The trait list includes Clone, Serialize and Deserialize. Source:
  https://docs.rs/rapier3d/latest/rapier3d/dynamics/struct.RigidBodySet.html, rapier3d 0.36.1. Tier: official. Read
  2026-10-27. Sure: confirmed in the source.
- **R1-ECS-F30.** Rapier's whole simulation state can be serialized, and its pipelines hold no state. Quotes: "most
  data structures of Rapier become serializable using serde"; "The complete state of the simulation is serialized by
  serializing its physics world as a whole."; "except for the PhysicsPipeline and the CollisionPipeline which don't
  hold any useful state." Source: https://rapier.rs/docs/user_guides/rust/serialization (guide navigation shows "Rust
  (0.36)"; no date). Tier: official. Read 2026-10-27. Sure: confirmed in the source.
- **R1-ECS-F31.** With enhanced determinism, Rapier promises byte-identical serialized states across machines. Quote:
  "running the exact same simulation on two different machine will result in the exact same byte vectors." Source:
  https://rapier.rs/docs/user_guides/rust/serialization. Tier: official. Read 2026-10-27. Sure: confirmed in the
  source. (Whether this holds between Wasm and native belongs to the determinism unit.)
- **R1-ECS-F32.** A user reported that Rapier 0.30.1 skipped part of its broad-phase state when serializing, so worlds
  restored from snapshots diverged; the reporter says it was fixed in Rapier 0.33. Quote (reporter): "BvhWorkspace is
  not serialized, but the BvhWorkspace::rebuild_frame_index value is actually important." Source:
  https://github.com/dimforge/rapier/issues/910 (opened 2026-01-16; closed by the reporter 2026-06-09; no maintainer
  reply). Tier: other (a user's issue report). Read 2026-10-27. Sure: confirmed in the source (the quote); the
  divergence and the fix version are the reporter's account, seen in the fetch summary (inferred).

### Wasm size
- **R1-ECS-F33.** Generic code grows Wasm size, because the compiler makes one copy per type it is used with, and size
  has to be measured. Quotes: "rustc and LLVM will create a new copy of the function for each T type that the function
  is used with."; "these copies add up quickly in terms of code size."; on opt-level "s" against "z": "Always
  measure!"; wasm-opt "can often get another 15-20% savings on code size." Source:
  https://rustwasm.github.io/book/reference/code-size.html (the Rust and WebAssembly book; the page calls itself
  "unpublished documentation"; no date). Tier: official (the Rust and WebAssembly working group's book). Read
  2026-10-27. Sure: confirmed in the source (short quotes).
- **R1-ECS-F34.** No source read in this run gives a Wasm size or a compile time for bevy_ecs, hecs or a plain-data
  core. Sure: confirmed (an absence across every source listed below), so all size and compile-time marks below are
  proposed and must be measured.

## Legacy facts used
- **CS8** (the SIM world container): legacy keeps the authoritative state on three.js objects and mirrors it into flat
  records once per step. v2 inverts this: the Rust sim owns the state. The flat, pooled record schema (players, plus
  kinded static boxes with an alive flag and an optional Y rotation) is the starting data layout.
- **GW16** (scene-to-SIM mirror): reusing slots with no allocation was a win worth keeping in Rust's own layout; the
  mirror itself goes.
- **GC2** (soldier slot table): 12 fixed slots, where slot id = sim id, plus a per-team index with no per-frame
  searches. Measured: perfbots at 48 soldiers, combat, mean 0.80–1.00 ms per update step, p95 0.97–1.54
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

Entity kinds the layout must hold (LOOP_V2 §2–§6 and VISION_V2 §3, §6): soldiers (humans and bots in slots; up to 48 at
the goal, "24v24" or "12v12v12", and LOOP_V2's "12v12v12 to 16v16v16, possibly more"), vehicles (the Juggernaut and one
signature tank per faction, multi-part, with seats `[OPEN]`), projectiles (grenades, Molotovs, bolts, Javelin rockets),
harvesters, buildings (refinery, power plant, barracks), the carried tech cache and the uplinks (two at full scale),
per-team economy and tech, active ability effects, and static colliders. There are about a dozen kinds, known in
advance. The counts beyond 48 soldiers are not given in any design doc.

## Options

| | A. bevy_ecs 0.20 (standalone) | B. hecs 0.11.2 | C. Custom plain data (typed tables in one state struct) |
|---|---|---|---|
| **Shape** | A World of archetype tables (F3), systems in schedules, single- or multi-threaded executor (F7). | An archetype World with no scheduler; plain functions query it (F16, F17). | One sim-state struct: a fixed-slot soldier table sized by the mode, generational pools for vehicles, projectiles and effects (F28 shows the key scheme), per-team tables, static colliders with ids, the RNG state, the tick counter, the tick's event list; phases are plain functions called in a fixed order. |
| **Deterministic order** | Not guaranteed (F4, F12); needs a sort in every outcome-deciding system, redone each run (F5); order ambiguities ignored by default (F6). Achievable, but every system has to opt in. | Arbitrary (F17); needs our own ordering (an index or a sort) on every outcome loop. | Storage order is iteration order (F25), and we control it. Still needs a rule against default-hasher HashMap iteration (F23); BTreeMap where a map is needed (F24). |
| **Whole-state snapshot / restore** | The World is not Clone (F8). The proven route (bevy_ggrs) registers each type, keeps per-frame maps keyed by a rollback id and reconciles entities on load (F10). Unregistered state silently desyncs (F13), and change detection fires after every restore (F14). | The World is not Clone (F18). We would write a snapshot by hand, per component type, or serialize (F20); the freelist makes re-created ids match (F19). | Derive or write Clone over the whole struct; clone_from into a preallocated copy reuses buffers (F26, F27). Nothing lives outside the struct, so nothing can be "unregistered" (inferred: it holds only if the sim keeps no globals or statics, which is a code rule). |
| **Ids across server and client** | Entity must not be lined up between instances (F9): a separate network id plus MapEntities mapping, as bevy_ggrs does (F10). | Ids can be aligned through the freelist (F19), but only while the live sets match exactly. | The sim defines the id (kind, index, generation, F28), and the same id is used on the wire, in events and in the TS client. |
| **Serialization for snapshots** | An optional serialize feature (F7); Entity's serialized form has no format guarantee (F9). | Context-based serialization with a compact column format (F20). | serde on our own types, the route Rapier (F30) and slotmap (F28, inferred) use; the wire format and deltas are R2's. |
| **Physics (if R4 picks Rapier)** | Rapier keeps its own sets either way (F29); the ECS holds handles and syncs transforms. | Same. | The Rapier sets are fields of the state struct, Clone and Serialize (F29, F30), so they join the same snapshot. |
| **Data-driven abilities** | Effects as entities linked to their target (relationships, F15) or as components; adding and removing components on table storage is the slow path (F3), sparse sets are the escape. | Effects as entities with a target component; no hooks or scheduler. | Effects as rows in an ordered effect pool (owner id, spec id from data, timers, stacks), plus attribute arrays per actor keyed by ids from data. No runtime-defined Rust types needed (inferred). The ability unit designs the rows. |
| **Wasm size, compile time** | Generic-heavy (systems and queries per type, F33); default features pull in reflection and std (F7). Size unmeasured (F34). | Small dependency closure is a stated goal (F16); size unmeasured (F34). | The fewest dependencies, but our own generics still monomorphize (F33); size unmeasured (F34). |
| **Upkeep and risk** | Five 0.x releases in 18 months (F1): a migration roughly each release (inferred). The largest feature set (scheduler, change detection, observers, relationships), most of which no §6 item needs (inferred against VISION_V2 §6). | A smaller API; we write ordering, snapshots and ids ourselves. | The most code we write (pools, ids, ordering, snapshot), but about a dozen kinds known in advance (Legacy facts) keeps it small, and the hecs authors point games with few types this way (F21). We lose ready-made change detection: if R2's deltas need dirty flags, we write them. |
| **Fit with VISION_V2 §6** | Meets §6.1–§6.8 with extra machinery. | Meets them with less machinery, but snapshots and order are on us. | Meets them directly: modes as data size the slot tables (§6.1), Rapier inside the state (§6.2), vehicles and effects as pools (§6.3, §6.4), economy tables (§6.5), the event list feeds telemetry and moments (§6.6, §6.8). |

Effort (inferred, not measured): A has the lowest effort to a first tick and the highest ongoing cost for determinism
and snapshots; B sits in between; C costs the most up front for pools and ids, and the least for snapshots and order.

## Recommendations

- **R1-ECS-REC1. Custom plain data (option C): one sim-state struct of typed tables, phases as plain functions called
  in a fixed order, no ECS crate.**
  The reason:
  - The entity kinds are few and known (about a dozen; Legacy facts), which is the case the hecs authors say suits
    plain Vecs (F21).
  - Both ECS crates leave iteration order unspecified (F4, F17), so determinism would need sorting or our own ordering
    in every outcome loop anyway (F5, F12). Plain tables iterate in the order we store (F25).
  - The client's replay needs a cheap whole-state snapshot and restore. Neither World is Clone (F8, F18). The bevy
    route is per-type registration with silent-desync and change-detection traps (F10, F13, F14). A plain struct is
    cloned into a preallocated copy that reuses its buffers (F26, F27).
  - Server and client need one shared id, which bevy's Entity is documented not to be (F9). The sim owning generational
    ids (F28) also gives static colliders the stable ids legacy lacked (CS7).
  - Rapier's sets are Clone and Serialize and slot straight into the struct (F29, F30).
  - It avoids a fast-moving 0.x dependency (F1) and features no §6 item needs (scheduler, observers, reflection).

  Rests on: F1, F4, F5, F8, F9, F10, F12, F13, F14, F17, F18, F21, F25, F26, F27, F28, F29, F30 (plus F33 and F34 for
  the size marks).
  Serves VISION_V2 §6.1, §6.2, §6.3, §6.4, §6.5, §6.6 and §6.8, and §4 ("matches feel fair up to about 100 ms of
  ping", which needs reproducible replay).

  A spike must prove (all marks proposed; reference hardware proposed: one server core of the R6 host type, and the
  VISION_V2 §4 minimum laptop for Wasm):
  1. **Tick cost.** Stress load (proposed, not a design target; the game-designer may change it): 48 soldiers (bots),
     8 vehicles, 300 live projectiles and 200 active effects, on a 3D greybox. The pass mark: a mean tick ≤ 2 ms and
     a p99 tick ≤ 4 ms native; Wasm in a Worker ≤ 2× native. For reference only, the legacy JS step at 48 soldiers
     measured 0.80–1.00 ms mean (GC2), without physics or a 3D world.
  2. **Snapshot.** clone_from of the whole state, Rapier included, into a preallocated copy: 0 heap allocations after
     warm-up (counted by a counting global allocator), ≤ 0.2 ms native and ≤ 0.5 ms Wasm at the stress load.
  3. **Restore and replay.** Restore, plus re-simulating k = ceil(0.15 s × tick rate) ticks (VISION_V2 §4's 100 ms,
     plus a proposed 50 ms margin; the tick rate is the tick unit's), fits in ≤ 8 ms in Wasm on the minimum laptop. If
     it doesn't, whole-world replay is ruled out and R2's partial prediction applies (R1-ECS-OQ3).
  4. **Snapshot fidelity.** A run that is snapshotted, restored and continued is byte-identical (serialized state
     hash each tick) to an uninterrupted run over a 10-minute bot match, and serialize → deserialize → serialize is
     byte-identical.
  5. **Size and build.** The sim .wasm, with and without Rapier, after wasm-opt and gzip, is recorded and is smaller
     than a stub of the same tick on bevy_ecs 0.20 with default features off (one build each, F33, F34). An
     incremental native rebuild after a one-line change to a phase takes ≤ 10 s on the dev machine.

- **R1-ECS-REC2. Determinism rules for the layout, whatever option the ADR picks.**
  Every outcome-deciding loop iterates in a defined order: slot or pool index order, or a sort by a stable key. The
  sim never iterates a default-hasher HashMap or HashSet (BTreeMap or an index-ordered Vec instead). The sim's RNG
  state and tick counter live inside the snapshotted state. Tick phases are an explicit, fixed sequence.

  The reason: the default HashMap is seeded per instance and iterates arbitrarily (F23); BTreeMap and slices have
  defined orders (F24, F25); ECS queries don't (F4, F17); and the legacy step order decided outcomes while it shared
  its RNG with FX and audio (GP14).

  Rests on: F4, F12, F17, F23, F24, F25.
  Serves VISION_V2 §4 (fair to about 100 ms of ping: prediction and replay need reproducible ticks) and §6.6 and
  §6.8 (server-side telemetry and moments are computed from reproducible state).

  A spike must prove (proposed):
  1. Two processes (different HashMap seeds) fed the same seed and inputs give identical per-tick state hashes over a
     10-minute bot match.
  2. A CI check finds zero uses of std HashMap or HashSet iteration in the sim crate's outcome code (the tooling is
     R5's).
  3. Under REC1's fidelity test, the RNG continues identically after a restore.

- **R1-ECS-REC3. The sim defines entity identity: one id (kind, index, generation) for every entity, static colliders
  included, used on the server, by the client predictor, in snapshots, in events and in the TypeScript client; never
  an ECS- or library-internal id on the wire.**
  The reason: bevy documents that its Entity must not be lined up across instances (F9), and rollback on bevy needs
  its own stable id (F10). Generational keys reject stale handles after a slot is reused (F28), and Rapier's handles
  work the same way (F29). Legacy can't name the static a shot hit (CS7), and its bolts follow a victim by reference
  (GC15).

  Rests on: F9, F10, F28, F29.
  Serves VISION_V2 §6.3 (effects name their source and target), §6.4 (vehicle parts and seats), §6.6 (telemetry names
  units) and §6.8 (moments name who did what).

  A spike must prove (proposed):
  1. Ids survive snapshot and restore unchanged.
  2. A despawned id never resolves after its slot is reused (a property test over random spawn and despawn sequences,
     10,000 cases).
  3. A server snapshot applied on the client maps every id with no lookup table beyond the pools themselves.

- **R1-ECS-REC4. If R4 picks Rapier, its sets are fields of the sim state and go into every snapshot and serialized
  state; the snapshot test covers the physics state byte for byte.**
  The reason: Rapier's sets are Clone and Serialize (F29); its full state is serializable while its pipelines hold no
  state (F30); and a reported case showed a library skipping internal state on serialize, which broke restored
  worlds (F32).

  Rests on: F29, F30, F32 (F31 for the cross-machine part, which is the determinism unit's).
  Serves VISION_V2 §6.2 (a true 3D simulation) and §6.4 (big vehicles as physical bodies).

  A spike must prove (proposed): REC1 mark 4 with Rapier active, including a cloned (not serialized) restore, on the
  Rapier version the ADR pins; any divergence is a blocker for that version.

## Open questions
- **R1-ECS-OQ1. What do the options really cost in Wasm size and compile time?** No source read gives figures (F34).
  Decided by: REC1 mark 5's measurement; the budget itself is set in the ADR from the number, not before.
- **R1-ECS-OQ2. Does `#[derive(Clone)]` give a field-wise `clone_from` that reuses each Vec's buffer?** The sources
  read cover Vec's clone_from (F26) and the trait's default (F27), not what the derive generates. Decided by: reading
  the derive's documentation, or REC1 mark 2's allocation count; if it doesn't, clone_from is written once for the
  state struct.
- **R1-ECS-OQ3. Does the client re-simulate the whole world, or only what it predicts (its own soldier, its vehicle,
  its projectiles)?** This sets how much state a snapshot and replay must cover, and so whether REC1 mark 3 can pass.
  Decided by: R2's prediction recommendation, checked against REC1 marks 1–3.
- **R1-ECS-OQ4. Does the server need parallelism inside one match at 48+ players, or is one match per core enough?**
  bevy's multi-threaded executor (F7) would be the one thing option A offers that C lacks. Decided by: REC1 mark 1's
  native tick time against the tick period the tick unit picks, and R6's hosting cost per match.
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

## Sources read
| URL | Title | Version or page date | Read |
|---|---|---|---|
| https://docs.rs/bevy_ecs/latest/bevy_ecs/ | bevy_ecs crate docs | 0.20.0 | 2026-10-27 |
| https://docs.rs/crate/bevy_ecs/latest/features | bevy_ecs feature flags | 0.20.0 | 2026-10-27 |
| https://docs.rs/bevy_ecs/latest/bevy_ecs/system/struct.Query.html | Query | 0.20.0 | 2026-10-27 |
| https://docs.rs/bevy_ecs/latest/bevy_ecs/query/struct.QueryIter.html | QueryIter (sort methods) | 0.20.0 | 2026-10-27 |
| https://docs.rs/bevy_ecs/latest/bevy_ecs/schedule/index.html | schedule module | 0.20.0 | 2026-10-27 |
| https://docs.rs/bevy_ecs/latest/bevy_ecs/schedule/struct.ScheduleBuildSettings.html | ScheduleBuildSettings | 0.20.0 | 2026-10-27 |
| https://docs.rs/bevy_ecs/latest/bevy_ecs/schedule/struct.SingleThreadedExecutor.html | SingleThreadedExecutor | 0.20.0 | 2026-10-27 |
| https://docs.rs/bevy_ecs/latest/bevy_ecs/entity/struct.EntityCloner.html | EntityCloner | 0.20.0 | 2026-10-27 |
| https://docs.rs/bevy_ecs/latest/bevy_ecs/entity/struct.Entity.html | Entity | 0.20.0, page dated 2026-10-08 | 2026-10-27 |
| https://docs.rs/bevy_ecs/latest/bevy_ecs/world/struct.World.html | World (trait implementations) | 0.20.0 | 2026-10-27 |
| https://github.com/bevyengine/bevy-website/tree/main/content/news | Bevy news posts (folder list) | to 2026-10-08 | 2026-10-27 |
| https://raw.githubusercontent.com/bevyengine/bevy-website/main/content/news/2025-04-24-bevy-0.16/index.md | Bevy 0.16 release post | 2025-04-24 | 2026-10-27 |
| https://github.com/gschup/bevy_ggrs | bevy_ggrs README | bevy 0.19 / bevy_ggrs 0.22; 0.20 on main | 2026-10-27 |
| https://raw.githubusercontent.com/gschup/bevy_ggrs/main/docs/architecture.md | bevy_ggrs architecture | main, no date | 2026-10-27 |
| https://raw.githubusercontent.com/gschup/bevy_ggrs/main/docs/pitfalls.md | bevy_ggrs pitfalls | main, no date | 2026-10-27 |
| https://docs.rs/hecs/latest/hecs/ | hecs crate docs | 0.11.2 | 2026-10-27 |
| https://docs.rs/hecs/latest/hecs/struct.World.html | hecs World | 0.11.2 | 2026-10-27 |
| https://docs.rs/hecs/latest/hecs/serialize/index.html | hecs serialize | 0.11.2 | 2026-10-27 |
| https://docs.rs/hecs/latest/hecs/serialize/column/index.html | hecs serialize::column | 0.11.2 | 2026-10-27 |
| https://github.com/Ralith/hecs | hecs README | no version shown | 2026-10-27 |
| https://doc.rust-lang.org/std/collections/struct.HashMap.html | HashMap | std 1.99.1 (2026-10-13) | 2026-10-27 |
| https://doc.rust-lang.org/std/collections/struct.BTreeMap.html | BTreeMap | std 1.99.1 | 2026-10-27 |
| https://doc.rust-lang.org/std/primitive.slice.html | slice (iter) | std 1.99.1 | 2026-10-27 |
| https://doc.rust-lang.org/std/clone/trait.Clone.html | Clone | std 1.99.1 | 2026-10-27 |
| https://doc.rust-lang.org/std/vec/struct.Vec.html | Vec (clone_from) | std 1.99.1 | 2026-10-27 |
| https://docs.rs/slotmap/latest/slotmap/ | slotmap crate docs | 1.1.1 | 2026-10-27 |
| https://docs.rs/rapier3d/latest/rapier3d/dynamics/struct.RigidBodySet.html | RigidBodySet | rapier3d 0.36.1 | 2026-10-27 |
| https://rapier.rs/docs/user_guides/rust/serialization | Rapier serialization guide | Rust guide 0.36, no date | 2026-10-27 |
| https://github.com/dimforge/rapier/issues/910 | Non-determinism in BroadPhase when restoring from serialized snapshot | opened 2026-01-16, closed 2026-06-09 | 2026-10-27 |
| https://rustwasm.github.io/book/reference/code-size.html | Shrinking .wasm code size | "unpublished documentation", no date | 2026-10-27 |
| https://github.com/SanderMertens/ecs-faq | ECS FAQ (other tier; read, not cited) | no date | 2026-10-27 |

Counts: 31 read (official 26, other 5; shipped game 0). 10 not read.

## Sources not read
| URL | Reason |
|---|---|
| https://bevyengine.org/news/bevy-0-16/ | Redirects (301) to bevy.org, outside the allow list; read from the site's GitHub repository instead |
| https://www.gdcvault.com/play/1024001/-Overwatch-Gameplay-Architecture-and | Redirects (302) to gdconf.com (a maintenance page), outside the allow list; the talk is video (likely video only) |
| https://gdcvault.com/play/1024001/-Overwatch-Gameplay-Architecture-and | Same redirect, outside the allow list |
| https://developer.valvesoftware.com/wiki/Networking_Entities | HTTP 403 Forbidden (on the allow list, but refused) |
| https://edgegap.com/blog/game-backend-deep-dive-overwatch-2016-netcode-architecture-rollback | Search result; outside the allow list |
| https://www.sohu.com/a/148848770_466876 | Search result (Overwatch talk write-up); outside the allow list |
| https://juejin.cn/post/6844903635864780813 | Search result; outside the allow list |
| https://www.gcores.com/articles/190857 | Search result; outside the allow list |
| https://www.gamedev.net/forums/topic/696756-command-frames-and-tick-synchronization/5386794/?page=3 | Search result; outside the allow list |
| https://godot.rapier.rs/docs/reference/state-manager | Search result; godot.rapier.rs is not listed (only rapier.rs), and it is not needed for a Rust core |

## Cross-unit notes
- **Fixed tick unit:** REC1's marks 1 and 3 take the tick rate as an input; the state struct holds the tick counter
  (REC2).
- **Wasm-vs-native unit:** REC1 mark 4 and REC2 mark 1 reuse the same per-tick state hash; Rapier's
  enhanced-determinism claim (F31) is that unit's to test between Wasm and native.
- **Boundary and Worker unit:** check how the state struct's fixed-slot tables map onto the render-snapshot buffer in
  shared memory; whether bevy's multi-threaded executor could run inside a Worker is that unit's question, and REC1
  does not need it.
- **Ability and effect unit:** REC1 assumes effects are rows in an ordered pool with ids from REC3, and attributes are
  arrays keyed by ids from data; that unit designs the rows, stacking and ordering.
- **Data formats and hot reload unit:** content specs are referenced from the state by id, and hot reload must not
  invalidate ids held in the live state.
- **R2 (netcode):** whole-world versus partial replay (OQ3); the wire format, delta compression and dirty tracking
  (REC1 drops bevy's change detection); entity ids on the wire (REC3).
- **R4 (physics and navigation):** REC4 depends on R4's physics choice; the spatial index for shots and line of sight
  (CS6, F22) lives beside the tables, inside the snapshot or rebuilt after a restore (R4 decides which). If bevy_ecs is
  reconsidered, check its physics integrations (not read here).
- **R5 (toolchain):** the CI check against HashMap and HashSet iteration in the sim crate (REC2 mark 2); the
  counting-allocator test (REC1 mark 2).
- **R6 (infrastructure):** cost per match, if one match per core (OQ4).
