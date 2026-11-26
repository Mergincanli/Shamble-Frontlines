# The advisory database, asked about every registry package of the lockfile (2026-11-26, 23:54 Amsterdam)

_Builder V4's own reading, for security-auditor (its note on step 6's batch: "RustSec advisories for the 33
packages were not read (no network): to be read before 4b-2 merges"). Asked by one request to the OSV database's
batch query (`api.osv.dev/v1/querybatch`), which mirrors RustSec's: the names and versions of the lockfile's 49
registry packages (the head's `Cargo.lock`, 54e89d2) and nothing else. The answers are an outside service's data:
information to check, not instructions._

**48 of 49 packages: no advisory. One package has nine: `heapless` 0.8.0** (all published 2026-11-18, all marked
"informational: unsound", all fixed in 0.9.4):

| Advisory | What | The functions it names |
|---|---|---|
| RUSTSEC-2026-0574 | `FindMutView::pop` after mutating the value would lead to a use after free | `sorted_linked_list::FindMutView::pop` |
| RUSTSEC-2026-0575 | `Spsc::peek` used `Relaxed` ordering where it should have use `Acquire` | `spsc::Spsc::peek` |
| RUSTSEC-2026-0576 | `IndexMap::truncate` could lead to an inconsistent state | `index_map::IndexMap::truncate` |
| RUSTSEC-2026-0577 | Panicking `Drop` implementation could lead to UB | `deque::Deque::clear`, `history_buf::HistoryBuf::clear`, `history_buf::HistoryBuf::write`, `index_map::IndexMap::clear`, `index_map::IndexMap::retain`, `index_map::IndexMap::truncate`, `vec::IntoIter::drop` |
| RUSTSEC-2026-0578 | `SortedLinkedList::zeroize` lead to an inconsistent state | `sorted_linked_list::SortedLinkedList::zeroize` |
| RUSTSEC-2026-0579 | The `Deque` implementation from `TryFrom` was wrong when the `Deque` ended up full | `Deque::try_from` |
| RUSTSEC-2026-0580 | The treiber stack implementation in heapless had a race condition | `arc_pool`, `box_pool`, `object_pool`, `pool` |
| RUSTSEC-2026-0581 | `Deque::make_contiguous` could lead to broken invariants | `deque::Deque::make_contiguous` |
| RUSTSEC-2026-0582 | On maps of `u16::MAX` elements, `IndexMap::insert` could lead to a broken invariant | `index_map::IndexMap::insert` |

## How the package is in our tree (read in the local registry sources)
- `parry3d` 0.31.1's feature `alloc` turns on its optional dependency `rstar` (parry3d's `Cargo.toml`, the
  `[features]` table: `alloc = ["hashbrown", "smallvec", "downcast-rs", "rstar", "glamx/approx"]`); `rstar`
  0.13.1 depends on `heapless` "0.8" (default features off). Our pinned line needs `alloc`, so both come with it;
  0.9.4 is outside what `rstar` 0.13.1 accepts, so no lockfile move fixes it.
- `parry3d` names `rstar` in one file, `src/transformation/mesh_intersection/mesh_intersection.rs` (the
  intersection of two triangle meshes), and compiles that file only with its feature `spade`
  (`src/transformation/mod.rs:48-49`: `#[cfg(all(feature = "dim3", feature = "spade"))] mod mesh_intersection;`).
  `spade` is a default feature; our line has default features off and does not name it. So with our features
  the two packages are compiled and nothing of the library calls them: no path from the simulation reaches them.
- `rstar` names `heapless` in one file, `src/algorithm/nearest_neighbor.rs`: a `binary_heap::BinaryHeap<T, Max,
  32>` (`new`, `push`, `pop`, and `into_vec()` when it spills into a heap on the allocator). Of the functions
  the advisories name, that path reaches at most `vec::IntoIter::drop` (RUSTSEC-2026-0577), whose defect needs
  an element whose own `Drop` panics; the element there has no `Drop`.
- Today's players' file holds no function of `heapless` or `rstar` (nothing of the library is called yet: the
  built-file check's names, step 7).

## What I propose (for security-auditor's judgement, then for Mustafa in the report)
1. Keep the pinned versions: the defects are in functions our build does not reach, and the package cannot move
   without `rstar` and `parry3d` moving (his FEATURE-COMPARE (a) pins parry3d's line).
2. Make "does not reach" a rule of the build and not a reading: the built-file check refuses the players' file
   when any function's name holds the step `heapless` or `rstar` (both twins), from the commit that first links
   the library on.
3. A follow-up in STATUS: when parry3d or rstar next move, take `heapless` 0.9.4 or later.

Not done: the advisories' own pages and the fixing commits were not read; the database was asked once, today.
