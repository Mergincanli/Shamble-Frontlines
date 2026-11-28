//! The aim as the simulation holds it: the world point a player aims at, in whole centimetres, as the command carries
//! it (protocol's `AimPoint`; ADR 0001 part 5). The point's metres are its centimetres over 100.

/// A world point in whole centimetres: x and z across the map, y the height above the map's floor.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct AimCm {
    pub x: i16,
    pub y: u16,
    pub z: i16,
}

impl AimCm {
    /// No aim yet: the held aim of a soldier no command has aimed. Its x is the type's least, a point outside every
    /// map's bounds, which a command's real aim never is, so a command that carries it leaves the held aim as it was.
    pub const NONE: Self = Self {
        x: i16::MIN,
        y: 0,
        z: 0,
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn no_aim_yet_is_the_least_x_and_nothing_else() {
        assert_eq!(AimCm::NONE.x, -32_768);
        assert_eq!((AimCm::NONE.y, AimCm::NONE.z), (0, 0));
        // A point at the origin is a real aim, not "no aim yet".
        assert_ne!(AimCm { x: 0, y: 0, z: 0 }, AimCm::NONE);
    }
}
