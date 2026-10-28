Task: verify R1 recommendations R1-ECS-* in docs/research/r1-simulation-core.md (v2-research wave 1 step 3, verifier part r1-ecs)
Scope: REC1-REC4. Basis set: F1,F4,F5,F8,F9,F10,F12,F13,F14,F17,F18,F21,F23,F24,F25,F26,F27,F28,F29,F30,F31,F32,F33,F34 + legacy GC2 (0.80-1.00 ms), CS7, GP14, GC15 + VISION_V2 §4 100 ms, §6 items.
Next: read VISION_V2 §4/§6 and LEGACY_INVENTORY rows, then fetch sources one by one.
- VISION_V2 §4 "[DEFAULT] Matches feel fair up to about 100 ms of ping." (docs/VISION_V2.md:54): confirmed. §6 items 1-8 exist; REC1-4 serves-lists all trace to real §6 items.
- GC2 0.80-1.00 ms mean at 48: confirmed, docs/reports/2026-10-16-three-teams-core.md:48 "48 soldiers (head): combat 0.80–1.00, p95 0.97–1.54"; LEGACY_INVENTORY.md:241 GC2 row.
- CS7 (static id -1): confirmed, LEGACY_INVENTORY.md:175 "today every block is id -1, so a hit cannot name which crate, wall or prop it struck".
- GP14 (order decides outcomes; shared Math.random with sound/FX): confirmed, LEGACY_INVENTORY.md:324 "change the legacy order only on purpose, because the order decides outcomes" / "the one shared Math.random, which sound and FX draw from inside the step".
- GC15 (bolt follows victim): confirmed, LEGACY_INVENTORY.md:254 "a stuck bolt follows the victim's mesh position (2873, 2881)". Note: "by reference" is a light gloss (inventory says mesh position).
Next: fetch web sources, starting F1.
- F1: CONFIRMED. github.com/bevyengine/bevy-website/tree/main/content/news folder list: "2025-04-24-bevy-0.16","2025-09-30-bevy-0.17","2026-01-13-bevy-0.18","2026-06-19-bevy-0.19","2026-10-08-bevy-0.20" (17.5 months). docs.rs/bevy_ecs header "bevy_ecs 0.20.0". API-churn part labelled inferred in doc (not a basis claim with a quote).
- F2 (not in rests-on): confirmed "it can easily be used as a standalone crate in other projects."
- F4: CONFIRMED. docs.rs Query (bevy_ecs-0.20.0): iter "Iteration order is not guaranteed."; par_iter "Iteration order and thread assignment is not guaranteed."; iter_many "Items are returned in the order of the list of entities, and may not be unique..."
Next: F5 QueryIter sort.
- F5: CONFIRMED. docs.rs QueryIter 0.20.0, method sort: "Sorts all query items into a new iterator, using the query lens as a key." and "The sort is not cached across system runs." (in every sort method).
- F8: CONFIRMED. docs.rs World 0.20.0, Trait Implementations (all three page parts read, offsets 0/100000/200000): "impl Debug for World", "impl Default for World", "impl Send for World", "impl Sync for World" (Send/Sync are listed as auto traits), no "impl Clone for World" anywhere. Minor: doc calls Send/Sync part of "its trait list"; they are auto traits. Not material.
- F9: CONFIRMED. docs.rs Entity 0.20.0: "For all intents and purposes, `Entity` should be treated as an opaque identifier."; "...one should not try to synchronize the ECS by attempting to ensure that `Entity` lines up between instances..."; to_bits "Do not use for serialization between runs."; "...make zero guarantee of long term wire format compatibility."
- F10 part 1: README confirmed: "Handles advancing game state and rollbacks via a dedicated schedule, snapshotting only the components and resources you register." compat rows "0.20 main main", "0.19 0.22 0.13.0". Next: architecture.md.
- F10: CONFIRMED. architecture.md: "a `HashMap<RollbackId, As>` mapping each rollback entity to its stored value at that frame"; "spawns a fresh entity with the same `Rollback` + `RollbackId`"; "despawns it (it didn't exist at the rollback target frame)"; "Each snapshotted type gets its own `GgrsSnapshots<For, As>` resource" + README "snapshotting only the components and resources you register." ("does not clone the World" is a fair reading of per-type snapshots; the doc never says it in those words.)
- F12: CONFIRMED. pitfalls.md: "Bevy does not guarantee a stable iteration order for queries."; advice "Always sort queries that mutate state:" / "If your entities don't have a natural sort key, use `RollbackOrdered`:" (advice now verbatim too).
- F13: CONFIRMED. pitfalls.md: "Any state stored elsewhere will not be restored on rollback, causing silent desyncs."; "`Local<T>` is per-system state that is **not snapshotted**."
- F14: CONFIRMED. pitfalls.md "Change Detection": "Every snapshot restore triggers change detection on all restored components."; "Systems that react to `Changed<T>` will fire after every rollback, which can cause performance issues"
Next: hecs F17, F18, F21.
- F17: CONFIRMED. docs.rs hecs 0.11.2 World: "An unordered collection of entities, each having any number of distinctly typed components"; "...are stored in contiguous runs, allowing for extremely fast, cache-friendly iteration."; query "Entities are yielded in arbitrary order."; "When old entities are despawned, their IDs will be reused on a future entity,"
- F18: CONFIRMED. same page, Trait Implementations: "impl Default for World", "impl<A: DynamicBundle> Extend<A> for World", "...FromIterator<A> for World", "impl<'a> IntoIterator for &'a World", "impl Send for World", "impl Sync for World"; no Clone.
- F21: CONFIRMED. github.com/Ralith/hecs README "Why Not ECS?": "If your game will have few types of entities, consider a simpler architecture such as storing each type of entity" + "...ch as storing each type of entity in a separate plain `Vec`."; "Similarly, ECS may be overkill for games that don't call for batch processing of entities."
- F23: CONFIRMED. doc.rust-lang.org HashMap (1.99.1): "As explained above, `HashMap` is randomly seeded: each `HashMap` instance uses a different seed, ..."; iter "An iterator visiting all key-value pairs in arbitrary order."
- F24: CONFIRMED. BTreeMap: "Iterators obtained from functions such as BTreeMap::iter, ... produce their items in key order, and take worst-case logarithmic and amortized constant time per item returned."
Next: F25 slice, F26 Vec clone_from, F27 Clone.
- F27: CONFIRMED. doc.rust-lang.org Clone: "Differs from `Copy` in that `Copy` is implicit and an inexpensive bit-wise copy, while `Clone` is always explicit and may or may not be expensive."; clone_from "`a.clone_from(&b)` is equivalent to `a = b.clone()` in functionality, but can be overridden to reuse the resources of `a` to avoid unnecessary allocations."
- F25, F26: first 100000 chars did not reach the passages; paging with offsets next.
- F25: CONFIRMED. doc.rust-lang.org primitive.slice (offset 100000-200000), method iter: "Returns an iterator over the slice." / "The iterator yields all items from start to end."
- F26: CONFIRMED. doc.rust-lang.org Vec (offset 400000-500000), impl Clone for Vec, clone_from: "This method is preferred over simply assigning `source.clone()` to `self`, as it avoids reallocation if possible."; "Additionally, if the element type `T` overrides `clone_from()`, this will reuse the resources of `self`'s elements as well."; example "// And no reallocation occurred" "assert_eq!(yp, y.as_ptr());"
Next: F28 slotmap, F29 RigidBodySet, F30/F31 rapier serialization.
- F28: CONFIRMED. docs.rs slotmap 1.1.1: "once a key is removed, it stays removed" (truncated by tool); dense "iteration is significantly faster, as fast as a normal Vec"; "It uses two indirections per random access; ..."; "This crate also supports no_std environments, but does require the alloc crate to be available." (The indirection part, labelled paraphrase in the doc, is now verbatim.)
- F29: CONFIRMED. docs.rs rapier3d-0.36.1 RigidBodySet: "The collection that stores all rigid bodies in your physics world."; "This handle is lightweight (just an index + generation) and remains valid even if other bodies are removed,..."; "impl Clone for RigidBodySet", "impl Serialize for RigidBodySet", "impl<'de> Deserialize<'de> for RigidBodySet". Note: "arena-backed" not in the quotes returned (descriptive gloss, not material).
- F30: CONFIRMED with note. rapier.rs serialization (nav "Rust (0.36)"): "When the `serde-serialize` feature of Rapier is enabled, most data structures of Rapier become serializable using serde."; "The complete state of the simulation is serialized by serializing its physics world as a whole"; "except for the `PhysicsPipeline` and the `CollisionPipeline` which don't hold any useful state". Note: needs the serde-serialize feature (doc omits this condition).
- F31: CONFIRMED with note. same page: "If the `enhanced-determinism` feature of Rapier is enabled" ... "running the exact same simulation on two different machine will result in the exact same byte vectors"; also conditioned "if your platform fulfills the required determinism requirements" (doc omits that condition).
Next: F32 issue 910, F33 rustwasm code-size, F34 (absence).
- F32: CONFIRMED. github.com/dimforge/rapier/issues/910 "Non-determinism in BroadPhase when restoring from serialized snapshot", opened 2026-01-16 by raysplaceinspace: "Rapier version: 0.30.1"; "it appears to be because BvhWorkspace is not serialized"; "the BvhWorkspace::rebuild_frame_index value is actually important"; "It appears this would cause further divergences as the game continues"; reporter comment 2026-06-09 "This was fixed in Parry 0.26.1 under dimforge/parry#402, which has now been included in Rapier 0.33"; closed by reporter 2026-06-09; no maintainer comment. (Doc's quote joins two fragments with ", but"; wording matches.)
- F33: CONFIRMED. rustwasm.github.io/book/reference/code-size.html: "Then `rustc` and LLVM will create a new copy of the function for each `T` type that the function is used with."; "these copies add up quickly in terms of code size."; "Always measure!"; "can often get another 15-20% savings on code size."; "unpublished documentation of working with Rust and WebAssembly". Checking tier (rustwasm WG status) next.
- F34: absence claim, not quotable. The sources I re-read (bevy_ecs, hecs, rustwasm code-size) give no size/compile-time figure for these options. Counted as n/a (absence), not as confirmed.
- F33 tier note: github.com/rustwasm/book "This repository was archived by the owner on Aug 25, 2025." -> "official" tier is dated (book unmaintained). Claim content still confirmed.

## Other findings (ranked)
1. (none rests on nothing) All four RECs have Rests-on lists.
2. Rests-on misses: REC1 cites Legacy facts (dozen kinds), CS7, GC2 (mark 1 number), VISION_V2 §4 (mark 3), and F7 (mark 5 "default features off") but lists none; REC2 cites GP14 (RNG and order) unlisted; REC3 cites CS7, GC15 unlisted. All cited items check out.
3. REC1 reason overreaches F26/F27: "A plain struct is cloned into a preallocated copy that reuses its buffers" - F27 says clone_from by default "is equivalent to a = b.clone()" and only "can be overridden"; derive behaviour is open (OQ2). Mark 2 also requires 0 allocations "Rapier included": no finding shows Rapier's Clone/clone_from reuses buffers (F29 shows only impl Clone).
4. REC1 "avoids a fast-moving 0.x dependency (F1)": the churn reading is inferred (F1 says so), and REC4 adopts Rapier, itself 0.x (0.30.1 in Jan 2026 per F32, 0.36.1 now per F29). Argument applied unevenly.
5. REC1 bevy-route traps (F10, F13, F14) are bevy_ggrs specifics, not bevy_ecs; and hecs counter-evidence F19 (deterministic ids via freelist) and F20 (column serialization) is not weighed. Conclusion still follows from F8/F18/F21.
6. Source conditions dropped: F30 needs the serde-serialize feature; F31 also needs "if your platform fulfills the required determinism requirements".
7. No unmarked numbers: every pass mark is under "(proposed)"/"all marks proposed". VISION_V2 §4 100 ms is [DEFAULT], not [DECIDED].
8. Every Serves item traces to a real VISION_V2 §6 item (or §4). None untraced.

## Table
F1 bevy 0.20.0 current; 0.16-0.20 dates | bevy-website news folders + docs.rs | confirmed (churn inferred)
F4 query order not guaranteed | docs.rs Query iter/par_iter/iter_many | confirmed
F5 sort not cached | docs.rs QueryIter sort | confirmed
F8 World no Clone | docs.rs World trait impls | confirmed
F9 Entity opaque, not cross-instance | docs.rs Entity | confirmed
F10 bevy_ggrs per-type snapshots, RollbackId | bevy_ggrs README + architecture.md | confirmed
F12 bevy needs explicit sort | pitfalls.md | confirmed
F13 unregistered state silently lost | pitfalls.md | confirmed
F14 restore triggers Changed<T> | pitfalls.md Change Detection | confirmed
F17 hecs arbitrary order, ids reused | docs.rs hecs World | confirmed
F18 hecs World no Clone | docs.rs hecs World | confirmed
F21 hecs: few kinds -> plain Vec | hecs README Why Not ECS? | confirmed
F23 HashMap random seed, arbitrary order | std HashMap | confirmed
F24 BTreeMap key order | std BTreeMap | confirmed
F25 slice iter start to end | std slice iter | confirmed
F26 Vec clone_from no realloc | std Vec impl Clone | confirmed
F27 Copy vs Clone, clone_from reuse | std Clone | confirmed
F28 slotmap removed key stays removed | docs.rs slotmap 1.1.1 | confirmed
F29 RigidBodySet Clone/Serde, index+generation | docs.rs rapier3d 0.36.1 | confirmed
F30 full state serializable, pipelines stateless | rapier.rs serialization | confirmed (note: serde-serialize feature)
F31 byte-identical with enhanced-determinism | rapier.rs serialization | confirmed (note: platform condition)
F32 0.30.1 BvhWorkspace gap, fixed 0.33 | rapier issue 910 | confirmed
F33 generics grow Wasm, measure, wasm-opt 15-20% | rustwasm book code-size | confirmed (tier dated: archived 2025-08-25)
F34 no size/compile figures | absence | n/a
GC2 0.80-1.00 ms at 48 | reports/2026-10-16-three-teams-core.md:48 | confirmed
CS7 static id -1 | LEGACY_INVENTORY.md:175 | confirmed
GP14 order decides; shared Math.random | LEGACY_INVENTORY.md:324 | confirmed
GC15 bolt follows victim | LEGACY_INVENTORY.md:254 | confirmed
V§4 100 ms ping | VISION_V2.md:54 | confirmed ([DEFAULT])
Counts: 28 confirmed, 0 wrong, 0 unsupported, 1 n/a.
DONE
