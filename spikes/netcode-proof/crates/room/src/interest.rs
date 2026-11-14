//! Interest (catalogue §3: "the spatial groups are packed by the priority accumulator under the per-client cap"; part 9):
//! [`InterestGrid`] finds what lies near a point, in integer centimetres; [`AreaFlood`] says which areas a client's
//! area connects to through the open portals; [`Priority`] orders what a snapshot carries. Float-free
//! (u4-build-plan.md §4.11).

use std::collections::BTreeMap;

use npt_sim::{Areas, EntityKind, Portals};

/// A uniform grid over the map's extent in the position grid's centimetres: each cell lists the records whose point
/// lies in it, rebuilt each tick.
#[derive(Clone, Debug)]
pub struct InterestGrid {
    min: [i32; 2],
    cell: i32,
    n: [usize; 2],
    cells: Vec<Vec<u32>>,
}

impl InterestGrid {
    /// A grid over `extent` (`[x_min, y_min, x_max, y_max]`, cm) of square cells `cell_cm` wide.
    pub(crate) fn new(extent: [i32; 4], cell_cm: i32) -> InterestGrid {
        let cell = cell_cm.max(1);
        let span =
            |lo: i32, hi: i32| ((i64::from(hi) - i64::from(lo)) / i64::from(cell) + 1) as usize;
        let n = [span(extent[0], extent[2]), span(extent[1], extent[3])];
        InterestGrid {
            min: [extent[0], extent[1]],
            cell,
            n,
            cells: vec![Vec::new(); n[0] * n[1]],
        }
    }

    /// The cell a point falls in, a point outside the extent taken into its nearest edge cell.
    fn cell_of(&self, at: [i32; 2]) -> [usize; 2] {
        let c = |v: i32, min: i32, n: usize| {
            let k = (i64::from(v) - i64::from(min)).div_euclid(i64::from(self.cell));
            k.clamp(0, n as i64 - 1) as usize
        };
        [
            c(at[0], self.min[0], self.n[0]),
            c(at[1], self.min[1], self.n[1]),
        ]
    }

    /// Empties every cell, keeping their storage.
    pub(crate) fn clear(&mut self) {
        for c in &mut self.cells {
            c.clear();
        }
    }

    /// Files record `id` at `at`.
    pub(crate) fn insert(&mut self, at: [i32; 2], id: u32) {
        let [x, y] = self.cell_of(at);
        self.cells[y * self.n[0] + x].push(id);
    }

    /// Every record filed in a cell that the square of half-width `radius` round `at` touches (the caller checks the
    /// exact distance).
    pub(crate) fn near(&self, at: [i32; 2], radius: i32, out: &mut Vec<u32>) {
        out.clear();
        let lo = self.cell_of([at[0].saturating_sub(radius), at[1].saturating_sub(radius)]);
        let hi = self.cell_of([at[0].saturating_add(radius), at[1].saturating_add(radius)]);
        for y in lo[1]..=hi[1] {
            for x in lo[0]..=hi[0] {
                out.extend_from_slice(&self.cells[y * self.n[0] + x]);
            }
        }
    }
}

/// Which areas each area connects to through the portals open now (u4-build-plan.md §4.2, A2): re-flooded when the
/// portal states change, from npt-sim's plain-data static graph.
#[derive(Clone, Debug)]
pub struct AreaFlood {
    joins: Vec<[u16; 2]>,
    areas: usize,
    open: Vec<bool>,
    /// `reach[a * areas + b]`: area b connects to area a.
    reach: Vec<bool>,
}

impl AreaFlood {
    pub(crate) fn new(areas: &Areas, portals: &Portals) -> AreaFlood {
        let n = areas.rects.len().max(1);
        let mut f = AreaFlood {
            joins: portals.joins.clone(),
            areas: n,
            open: Vec::new(),
            reach: vec![false; n * n],
        };
        f.flood(&vec![false; portals.joins.len()]);
        f
    }

    /// Re-floods when `open` differs from the states it last flooded with; returns whether it did.
    pub(crate) fn update(&mut self, open: &[bool]) -> bool {
        if open == self.open.as_slice() {
            return false;
        }
        self.flood(open);
        true
    }

    fn flood(&mut self, open: &[bool]) {
        self.open.clear();
        self.open.extend_from_slice(open);
        let n = self.areas;
        self.reach.iter_mut().for_each(|r| *r = false);
        let mut stack = Vec::new();
        for start in 0..n {
            self.reach[start * n + start] = true;
            stack.clear();
            stack.push(start);
            while let Some(a) = stack.pop() {
                for (p, [x, y]) in self.joins.iter().enumerate() {
                    if !open.get(p).copied().unwrap_or(false) {
                        continue;
                    }
                    let (x, y) = (usize::from(*x), usize::from(*y));
                    let other = if x == a {
                        y
                    } else if y == a {
                        x
                    } else {
                        continue;
                    };
                    if other < n && !self.reach[start * n + other] {
                        self.reach[start * n + other] = true;
                        stack.push(other);
                    }
                }
            }
        }
    }

    /// Whether area `to` connects to area `from` now (an area outside the graph connects to nothing).
    pub(crate) fn reaches(&self, from: u16, to: u16) -> bool {
        let (a, b) = (usize::from(from), usize::from(to));
        a < self.areas && b < self.areas && self.reach[a * self.areas + b]
    }

    /// Whether portal `p` lies in an area `from` connects to: a portal counts as inside both areas it joins
    /// (catalogue §3's world-pieces row).
    pub(crate) fn sees_portal(&self, from: u16, p: usize) -> bool {
        self.joins
            .get(p)
            .is_some_and(|[x, y]| self.reaches(from, *x) || self.reaches(from, *y))
    }
}

/// The priority accumulator (catalogue §3, part 9 (d)): own state always (outside it), the nearest 8 enemies in every
/// snapshot, and every relevant entity with a change at least every 4 snapshots; this holds, per entity, how many
/// snapshots in a row skipped its change.
#[derive(Clone, Debug, Default)]
pub struct Priority {
    skipped: BTreeMap<(EntityKind, u16), u32>,
}

impl Priority {
    /// The nearest enemies every snapshot carries.
    pub(crate) const NEAREST_ENEMIES: usize = 8;
    /// Every relevant changed entity goes out at least once in this many snapshots, when the budget allows.
    pub(crate) const EVERY: u32 = 4;

    /// How many snapshots in a row skipped `id`'s change.
    pub(crate) fn skipped(&self, id: (EntityKind, u16)) -> u32 {
        self.skipped.get(&id).copied().unwrap_or(0)
    }

    /// `id`'s change was sent, or it had none.
    pub(crate) fn sent(&mut self, id: (EntityKind, u16)) {
        self.skipped.remove(&id);
    }

    /// `id`'s change was skipped this snapshot.
    pub(crate) fn skip(&mut self, id: (EntityKind, u16)) {
        *self.skipped.entry(id).or_insert(0) += 1;
    }

    /// Forget every entity no longer relevant (`keep` says which are).
    pub(crate) fn retain(&mut self, mut keep: impl FnMut((EntityKind, u16)) -> bool) {
        self.skipped.retain(|k, _| keep(*k));
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_grid_finds_what_lies_in_the_touched_cells_and_clamps_points_outside() {
        let mut g = InterestGrid::new([-1_000, -1_000, 1_000, 1_000], 500);
        g.insert([0, 0], 1);
        g.insert([900, 900], 2);
        g.insert([50_000, 0], 3);
        let mut out = Vec::new();
        g.near([10, 10], 100, &mut out);
        assert_eq!(out, [1]);
        g.near([1_000, 0], 100, &mut out);
        assert!(out.contains(&3) && !out.contains(&1));
        g.clear();
        g.near([10, 10], 100, &mut out);
        assert!(out.is_empty());
    }

    #[test]
    fn the_flood_follows_open_portals_and_refloods_only_on_a_change() {
        let areas = Areas {
            rects: vec![[0; 4]; 4],
        };
        let portals = Portals {
            joins: vec![[0, 1], [0, 2], [2, 3]],
        };
        let mut f = AreaFlood::new(&areas, &portals);
        assert!(f.reaches(1, 1) && !f.reaches(1, 0));
        assert!(f.update(&[true, false, true]));
        assert!(!f.update(&[true, false, true]));
        assert!(f.reaches(1, 0) && !f.reaches(1, 2) && f.reaches(2, 3));
        assert!(
            f.sees_portal(1, 1),
            "portal 1 joins area 0, which area 1 reaches"
        );
        assert!(!f.sees_portal(3, 0));
        assert!(f.update(&[true, true, true]));
        assert!(f.reaches(1, 3) && f.reaches(3, 1));
        assert!(!f.reaches(9, 0));
    }

    #[test]
    fn priority_counts_skips_in_a_row_and_forgets_on_a_send() {
        let mut p = Priority::default();
        let id = (EntityKind::Projectile, 4);
        p.skip(id);
        p.skip(id);
        assert_eq!(p.skipped(id), 2);
        p.sent(id);
        assert_eq!(p.skipped(id), 0);
        p.skip(id);
        p.retain(|k| k != id);
        assert_eq!(p.skipped(id), 0);
    }
}
