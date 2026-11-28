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

    /// Each part times `factor`.
    #[must_use]
    pub fn scale(self, factor: f32) -> Self {
        Self::new(self.x * factor, self.y * factor, self.z * factor)
    }

    /// The dot product, summed in the order x, y, z (a fixed order: the same bits everywhere).
    #[must_use]
    pub fn dot(self, other: Self) -> f32 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }

    /// The length squared, summed as `dot` sums.
    #[must_use]
    pub fn length_sq(self) -> f32 {
        self.dot(self)
    }
}

/// The part-by-part sum.
impl core::ops::Add for V3 {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self::new(self.x + other.x, self.y + other.y, self.z + other.z)
    }
}

/// The part-by-part difference.
impl core::ops::Sub for V3 {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self::new(self.x - other.x, self.y - other.y, self.z - other.z)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_helpers_do_their_arithmetic_part_by_part() {
        let a = V3::new(1.0, -2.0, 0.5);
        let b = V3::new(4.0, 0.25, -3.0);
        assert_eq!(a + b, V3::new(5.0, -1.75, -2.5));
        assert_eq!(a - b, V3::new(-3.0, -2.25, 3.5));
        assert_eq!(a.scale(2.0), V3::new(2.0, -4.0, 1.0));
        // 4 - 0.5 - 1.5: every partial sum is exact.
        assert_eq!(a.dot(b).to_bits(), 2.0_f32.to_bits());
        assert_eq!(a.length_sq().to_bits(), 5.25_f32.to_bits());
    }
}
