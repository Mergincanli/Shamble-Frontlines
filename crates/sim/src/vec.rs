//! The simulation's own vector: three `f32`, in metres, y up. The state and the queries speak in this type and never
//! in the collision library's (only `collision.rs` names that library), so what a vector may do is what is written
//! here, in ADR 0004 part 7's operations. It holds what the static world needs so far; movement adds its own.

/// A point or a direction.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct V3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl V3 {
    #[must_use]
    pub const fn new(x: f32, y: f32, z: f32) -> Self {
        Self { x, y, z }
    }

    /// The three numbers by axis: x, y, z.
    #[must_use]
    pub const fn to_array(self) -> [f32; 3] {
        [self.x, self.y, self.z]
    }
}
