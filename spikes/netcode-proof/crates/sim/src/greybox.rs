//! The greybox: the static world every scene stands in (plan E1: "a DM_Tri-sized greybox with enterable buildings,
//! areas, portals, doors at 1/s, and a destructible-wall stand-in (R-10)"; ADR 0001:1470-1474). All of it is fixed at
//! construction; what changes (the doors, the wall) is `World`'s state.

/// A box with its faces on the axes: in metres for collision and rays, and in the position grid's centimetres it was
/// given in.
#[derive(Clone, Copy, Debug, PartialEq)]
pub(crate) struct Aabb {
    pub(crate) min: [f32; 3],
    pub(crate) max: [f32; 3],
    pub(crate) min_cm: [i32; 3],
    pub(crate) max_cm: [i32; 3],
}

impl Aabb {
    /// The box from `min` to `max`, both in the position grid's centimetres, each corner on the wire grid.
    pub(crate) fn cm(min: [i32; 3], max: [i32; 3]) -> Aabb {
        let m = |c: i32| crate::grid::Config::default().from_grid(crate::grid::POSITION_XY, c);
        Aabb {
            min: min.map(m),
            max: max.map(m),
            min_cm: min,
            max_cm: max,
        }
    }
}

/// The areas of the static graph (plan.md:567; u4-build-plan.md §4.2), as plain integer data the room's interest reads:
/// area 0 is the open map, area `i + 1` the inside of building `i`.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Areas {
    /// Each area's footprint, `[x_min, y_min, x_max, y_max]` in the position grid's centimetres; a point inside no
    /// building's footprint is in area 0.
    pub rects: Vec<[i32; 4]>,
}

/// The portals of the static graph: portal `i` is door `i`, open or closed in [`crate::PortalStates`].
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Portals {
    /// The two areas each portal joins.
    pub joins: Vec<[u16; 2]>,
}

/// The DM_Tri-sized greybox (docs/LEGACY_INVENTORY.md CM4: bases 92 m out from the centre field; here a 256 m square
/// centred on the origin): six enterable buildings round the centre field, each with one door, a few crates, and the
/// destructible-wall stand-in in the field.
#[derive(Clone, Debug)]
pub struct Greybox {
    areas: Areas,
    portals: Portals,
    /// Everything that never moves: the map's edge, the buildings' walls, the crates.
    pub(crate) solids: Vec<Aabb>,
    /// Each door's box when closed; door `i` is portal `i` and fills a gap in building `i`'s wall.
    pub(crate) doors: Vec<Aabb>,
    /// The wall stand-in's box while it stands.
    pub(crate) wall: Aabb,
    /// The building footprints in centimetres, building `i` being area `i + 1`.
    footprints: Vec<[i32; 4]>,
}

/// The map's half width, cm: 128 m.
const HALF_MAP: i32 = 12_800;
/// A building's inside half width, cm.
const ROOM: i32 = 400;
/// A wall's thickness, cm.
const THICK: i32 = 30;
/// A wall's height, cm.
const WALL_HEIGHT: i32 = 300;
/// A door gap's half width, cm.
const DOOR_HALF: i32 = 100;

/// Which side of its building a door is on.
#[derive(Clone, Copy, PartialEq, Eq)]
enum Side {
    West,
    East,
    South,
    North,
}

impl Greybox {
    /// The one greybox every scene uses.
    pub fn dm_tri() -> Greybox {
        let mut solids = Vec::new();
        // The map's edge: four walls 1 m thick and 5 m high.
        let (h, t) = (HALF_MAP, 100);
        solids.push(Aabb::cm([-h - t, -h - t, 0], [h + t, -h, 500]));
        solids.push(Aabb::cm([-h - t, h, 0], [h + t, h + t, 500]));
        solids.push(Aabb::cm([-h - t, -h, 0], [-h, h, 500]));
        solids.push(Aabb::cm([h, -h, 0], [h + t, h, 500]));
        // Six buildings round the centre field, 22 m out, each door facing the centre.
        let buildings: [([i32; 2], Side); 6] = [
            ([2_200, 0], Side::West),
            ([1_100, 1_905], Side::South),
            ([-1_100, 1_905], Side::South),
            ([-2_200, 0], Side::East),
            ([-1_100, -1_905], Side::North),
            ([1_100, -1_905], Side::North),
        ];
        let mut doors = Vec::new();
        let mut footprints = Vec::new();
        for ([cx, cy], door_side) in buildings {
            let (x0, y0, x1, y1) = (cx - ROOM, cy - ROOM, cx + ROOM, cy + ROOM);
            footprints.push([x0, y0, x1, y1]);
            let sides = [Side::West, Side::East, Side::South, Side::North];
            for side in sides {
                // The wall's box, and the gap's span along it when the door is on this side.
                let (min, max) = match side {
                    Side::West => ([x0 - THICK, y0, 0], [x0, y1, WALL_HEIGHT]),
                    Side::East => ([x1, y0, 0], [x1 + THICK, y1, WALL_HEIGHT]),
                    Side::South => ([x0 - THICK, y0 - THICK, 0], [x1 + THICK, y0, WALL_HEIGHT]),
                    Side::North => ([x0 - THICK, y1, 0], [x1 + THICK, y1 + THICK, WALL_HEIGHT]),
                };
                if side != door_side {
                    solids.push(Aabb::cm(min, max));
                    continue;
                }
                match side {
                    Side::West | Side::East => {
                        solids.push(Aabb::cm(min, [max[0], cy - DOOR_HALF, max[2]]));
                        solids.push(Aabb::cm([min[0], cy + DOOR_HALF, min[2]], max));
                        doors.push(Aabb::cm(
                            [min[0], cy - DOOR_HALF, 0],
                            [max[0], cy + DOOR_HALF, WALL_HEIGHT],
                        ));
                    }
                    Side::South | Side::North => {
                        solids.push(Aabb::cm(min, [cx - DOOR_HALF, max[1], max[2]]));
                        solids.push(Aabb::cm([cx + DOOR_HALF, min[1], min[2]], max));
                        doors.push(Aabb::cm(
                            [cx - DOOR_HALF, min[1], 0],
                            [cx + DOOR_HALF, max[1], WALL_HEIGHT],
                        ));
                    }
                }
            }
        }
        // Eight crates, 1 m cubes, in the centre field.
        for [x, y] in [
            [600, 600],
            [-600, 600],
            [600, -600],
            [-600, -600],
            [0, 900],
            [0, -900],
            [900, 0],
            [-900, 0],
        ] {
            solids.push(Aabb::cm([x - 50, y - 50, 0], [x + 50, y + 50, 100]));
        }
        let mut rects = vec![[-HALF_MAP, -HALF_MAP, HALF_MAP, HALF_MAP]];
        rects.extend_from_slice(&footprints);
        let joins = (1..=footprints.len() as u16).map(|a| [0, a]).collect();
        Greybox {
            areas: Areas { rects },
            portals: Portals { joins },
            solids,
            doors,
            // The wall stand-in: 6 m long, 40 cm thick, 3 m high, 12 m north of the centre.
            wall: Aabb::cm([-300, 1_180, 0], [300, 1_220, WALL_HEIGHT]),
            footprints,
        }
    }

    /// The areas (plain integer data, for the room's interest).
    pub fn areas(&self) -> &Areas {
        &self.areas
    }

    /// The portals (plain integer data, for the room's AreaFlood): portal `i` is door `i`.
    pub fn portals(&self) -> &Portals {
        &self.portals
    }

    /// The area a point is in, the point in the position grid's centimetres (x, y).
    pub(crate) fn area_of(&self, x: i32, y: i32) -> u16 {
        for (i, [x0, y0, x1, y1]) in self.footprints.iter().enumerate() {
            if x >= *x0 && x < *x1 && y >= *y0 && y < *y1 {
                return i as u16 + 1;
            }
        }
        0
    }
}
