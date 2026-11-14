//! The roster (catalogue §2, `Join`'s row and its notes on `bind`; plan §10.3): per slot its test token, the bind value
//! it took at its first accepted `Join`, and whether a session holds it. The caller passes the tokens in (the lab at
//! U4r; minting is U6's).

use npt_proto::JoinRejected;

/// One slot of the roster. No `Debug`: it holds the slot's secrets (catalogue: nothing is echoed back; plan E22).
#[derive(Clone)]
struct Entry {
    token: [u8; 16],
    bind: Option<[u8; 16]>,
    live: bool,
}

/// 1 when the two 16-byte values are equal, else 0, in constant time: every byte is read, the differences are folded
/// into one byte, and that byte becomes the answer by arithmetic, never by a branch on the bytes (catalogue :93: "the
/// test token compared in constant time").
fn same(a: &[u8; 16], b: &[u8; 16]) -> usize {
    let mut d = 0u8;
    for (x, y) in a.iter().zip(b) {
        d |= x ^ y;
    }
    let d = core::hint::black_box(d);
    usize::from((u16::from(d).wrapping_sub(1) >> 8) as u8 & 1)
}

/// The roster: a token per slot, at most `n` sessions at once (no join beyond the roster's N: catalogue :93). Its
/// `Debug` shows how many slots are bound and live, never a token or a bind value.
#[derive(Clone)]
pub struct Roster {
    slots: Vec<Entry>,
    n: usize,
}

impl core::fmt::Debug for Roster {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("Roster")
            .field("slots", &self.slots.len())
            .field(
                "bound",
                &self.slots.iter().filter(|s| s.bind.is_some()).count(),
            )
            .field("live", &self.live())
            .field("n", &self.n)
            .finish()
    }
}

/// What [`Roster`] made of an accepted `Join`: the slot, and whether a live session held it (a replacement: the old
/// session gets `Kick(replaced)`, catalogue :93).
pub(crate) struct Admitted {
    pub(crate) slot: u8,
    pub(crate) replaced: bool,
}

impl Roster {
    /// A roster of one slot per token, slot `i` holding `tokens[i]`, admitting at most `n` sessions at once. At most 256
    /// slots are kept (a slot is a `u8`).
    pub(crate) fn new(mut tokens: Vec<[u8; 16]>, n: usize) -> Roster {
        tokens.truncate(usize::from(u8::MAX) + 1);
        Roster {
            slots: tokens
                .into_iter()
                .map(|token| Entry {
                    token,
                    bind: None,
                    live: false,
                })
                .collect(),
            n,
        }
    }

    /// The sessions live now.
    pub fn live(&self) -> usize {
        self.slots.iter().filter(|s| s.live).count()
    }

    /// Whether a session holds `slot` now.
    pub fn is_live(&self, slot: u8) -> bool {
        self.slots.get(usize::from(slot)).is_some_and(|s| s.live)
    }

    /// Whether `slot`'s token is bound to a client's bind value (at its first accepted `Join`).
    #[cfg_attr(not(test), allow(dead_code))]
    pub(crate) fn is_bound(&self, slot: u8) -> bool {
        self.slots
            .get(usize::from(slot))
            .is_some_and(|s| s.bind.is_some())
    }

    /// The room's checks of a `Join` (catalogue :93): the token against every slot's, each compared in constant time
    /// and the whole roster read whatever matches; the bind value against the one the slot took at its first accepted
    /// `Join`; no join beyond `n` live sessions (a replacement of a live session adds none). The version and the content
    /// hash are npt-server's (checked before the room sees the `Join`).
    pub(crate) fn admit(
        &mut self,
        token: &[u8; 16],
        bind: &[u8; 16],
    ) -> Result<Admitted, JoinRejected> {
        let mut hit = 0usize;
        for (i, s) in self.slots.iter().enumerate() {
            let take = same(&s.token, token) & usize::from(hit == 0);
            hit |= take.wrapping_neg() & (i + 1);
        }
        let Some(i) = hit.checked_sub(1) else {
            return Err(JoinRejected::BadToken);
        };
        let live = self.live();
        let s = &mut self.slots[i];
        if let Some(bound) = &s.bind
            && same(bound, bind) == 0
        {
            return Err(JoinRejected::BindMismatch);
        }
        let replaced = s.live;
        if !replaced && live >= self.n {
            return Err(JoinRejected::RosterFull);
        }
        s.bind = Some(*bind);
        s.live = true;
        Ok(Admitted {
            slot: i as u8,
            replaced,
        })
    }

    /// The slot's session ended (a `Leave`, a closed link, a kick); the slot keeps its token and bind value for a
    /// rejoin (catalogue `Leave`'s row).
    pub(crate) fn leave(&mut self, slot: u8) {
        if let Some(s) = self.slots.get_mut(usize::from(slot)) {
            s.live = false;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn token(i: u8) -> [u8; 16] {
        [i; 16]
    }

    #[test]
    fn same_is_one_for_equal_values_and_zero_for_any_one_bit_differing() {
        let a = [7u8; 16];
        assert_eq!(same(&a, &a), 1);
        for i in 0..16 {
            for bit in 0..8 {
                let mut b = a;
                b[i] ^= 1 << bit;
                assert_eq!(same(&a, &b), 0, "byte {i} bit {bit}");
            }
        }
    }

    /// e9's room-side JoinRejected reasons (the version and content hash are the pump's: server/tests).
    #[test]
    fn e9_the_room_side_join_reasons_bad_token_bind_mismatch_and_roster_full() {
        let mut r = Roster::new(vec![token(1), token(2), token(3)], 2);
        assert_eq!(
            r.admit(&token(9), &[0; 16]).err(),
            Some(JoinRejected::BadToken)
        );
        let a = r.admit(&token(1), &[5; 16]).expect("slot 0");
        assert_eq!((a.slot, a.replaced), (0, false));
        assert!(r.is_bound(0) && r.is_live(0));
        r.leave(0);
        // The slot keeps its bind value: another bind is refused, the same one admitted again.
        assert_eq!(
            r.admit(&token(1), &[6; 16]).err(),
            Some(JoinRejected::BindMismatch)
        );
        assert_eq!(r.admit(&token(1), &[5; 16]).map(|a| a.slot).ok(), Some(0));
        assert_eq!(r.admit(&token(2), &[1; 16]).map(|a| a.slot).ok(), Some(1));
        // Two live of n = 2: a third slot's valid join is refused; a replacement of a live one is not.
        assert_eq!(
            r.admit(&token(3), &[1; 16]).err(),
            Some(JoinRejected::RosterFull)
        );
        let again = r.admit(&token(2), &[1; 16]).expect("a replacement");
        assert_eq!((again.slot, again.replaced), (1, true));
        assert_eq!(r.live(), 2);
        let shown = format!("{r:?}");
        assert_eq!(shown, "Roster { slots: 3, bound: 2, live: 2, n: 2 }");
    }
}
