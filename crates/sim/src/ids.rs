//! The ids of the match's entities (ADR 0004 part 2; PM-0004-15). An id names a slot and the slot's generation when
//! the id was given out, so an id kept past its entity's despawn names a generation that is gone and resolves to
//! nothing. So far only soldiers have ids.

/// A soldier's id: his slot in the soldier table and the slot's generation when he was spawned.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SoldierId {
    pub slot: u16,
    pub generation: u16,
}

impl SoldierId {
    /// An id that names no soldier: its slot lies past every table's last (the table holds at most `u16::MAX` slots).
    pub const NONE: Self = Self {
        slot: u16::MAX,
        generation: u16::MAX,
    };

    /// The id as one number: the slot in the high 16 bits, the generation in the low 16.
    #[must_use]
    pub const fn to_bits(self) -> u32 {
        let [slot_high, slot_low] = u16::to_be_bytes(self.slot);
        let [generation_high, generation_low] = u16::to_be_bytes(self.generation);
        u32::from_be_bytes([slot_high, slot_low, generation_high, generation_low])
    }

    /// The id a number names, as `to_bits` writes it: every number names one id.
    #[must_use]
    pub const fn from_bits(bits: u32) -> Self {
        let [slot_high, slot_low, generation_high, generation_low] = u32::to_be_bytes(bits);
        Self {
            slot: u16::from_be_bytes([slot_high, slot_low]),
            generation: u16::from_be_bytes([generation_high, generation_low]),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn an_id_and_its_bits_name_each_other() {
        let id = SoldierId {
            slot: 0x0102,
            generation: 0xA0B0,
        };
        assert_eq!(id.to_bits(), 0x0102_A0B0);
        assert_eq!(SoldierId::from_bits(0x0102_A0B0), id);
        for bits in [0, 1, 0xFFFF, 0x1_0000, 0x8000_0001, u32::MAX] {
            assert_eq!(SoldierId::from_bits(bits).to_bits(), bits);
        }
        assert_eq!(SoldierId::NONE.to_bits(), u32::MAX);
    }
}
